//! Task Router
//!
//! Routes tasks to the best-fit DePIN node using configurable strategies.
//! Tracks active assignments and releases slots on completion or failure.

use crate::nodes::{HardwareRequirements, NodeId, NodeRegistry};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::sync::RwLock;
use uuid::Uuid;

// ============================================================================
// TYPES
// ============================================================================

pub type TaskId = String;

#[derive(Debug, Clone)]
pub enum RoutingStrategy {
    CapacityFirst,
    LowestLatency,
    RoundRobin,
}

#[derive(Debug, Clone)]
pub struct Task {
    pub id: TaskId,
    pub requirements: HardwareRequirements,
    pub priority: u8,
}

impl Task {
    pub fn new(requirements: HardwareRequirements) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            requirements,
            priority: 0,
        }
    }
}

// ============================================================================
// TASK ROUTER
// ============================================================================

pub struct TaskRouter {
    registry: Arc<NodeRegistry>,
    strategy: RoutingStrategy,
    assignments: Arc<RwLock<HashMap<NodeId, Vec<TaskId>>>>,
    round_robin_idx: Arc<AtomicUsize>,
}

impl TaskRouter {
    pub fn new(registry: Arc<NodeRegistry>, strategy: RoutingStrategy) -> Self {
        Self {
            registry,
            strategy,
            assignments: Arc::new(RwLock::new(HashMap::new())),
            round_robin_idx: Arc::new(AtomicUsize::new(0)),
        }
    }

    /// Route a task to the best eligible node according to the configured strategy.
    pub async fn route(&self, task: &Task) -> Result<NodeId, String> {
        let mut candidates = self.registry.discover(&task.requirements).await;
        if candidates.is_empty() {
            return Err("no eligible nodes for task requirements".to_string());
        }

        let node_id = match self.strategy {
            RoutingStrategy::CapacityFirst => {
                // Highest available capacity wins
                candidates.sort_by(|a, b| b.capacity.partial_cmp(&a.capacity).unwrap());
                candidates[0].id.clone()
            }
            RoutingStrategy::LowestLatency => {
                // Proxy: smallest last_seen delta = freshest heartbeat = lowest effective latency
                candidates.sort_by_key(|n| n.last_seen.elapsed().as_millis());
                candidates[0].id.clone()
            }
            RoutingStrategy::RoundRobin => {
                let idx = self.round_robin_idx.fetch_add(1, Ordering::Relaxed) % candidates.len();
                // Sort by id for deterministic ordering before indexing
                candidates.sort_by(|a, b| a.id.cmp(&b.id));
                candidates[idx].id.clone()
            }
        };

        self.assignments
            .write()
            .await
            .entry(node_id.clone())
            .or_default()
            .push(task.id.clone());

        Ok(node_id)
    }

    /// Release a task assignment from a node (call on completion or failure).
    pub async fn release_task(&self, node_id: &NodeId, task_id: &TaskId) {
        let mut assignments = self.assignments.write().await;
        if let Some(tasks) = assignments.get_mut(node_id) {
            tasks.retain(|t| t != task_id);
        }
    }

    pub async fn active_assignments(&self) -> HashMap<NodeId, Vec<TaskId>> {
        self.assignments.read().await.clone()
    }

    pub async fn node_task_count(&self, node_id: &NodeId) -> usize {
        self.assignments
            .read()
            .await
            .get(node_id)
            .map(|v| v.len())
            .unwrap_or(0)
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nodes::{HardwareSpecs, Node, NodeStatus};
    use std::time::Instant;

    fn make_node(id: &str, capacity: f64, ram_gb: u64) -> Node {
        Node {
            id: id.to_string(),
            address: "127.0.0.1:9000".parse().unwrap(),
            hardware_specs: HardwareSpecs { gpu: false, ram_gb, bandwidth_mbps: 1000 },
            capacity,
            status: NodeStatus::Online,
            last_seen: Instant::now(),
        }
    }

    async fn registry_with_nodes(nodes: Vec<Node>) -> Arc<NodeRegistry> {
        let registry = Arc::new(NodeRegistry::new(60));
        for node in nodes {
            registry.register(node).await.unwrap();
        }
        registry
    }

    fn no_requirements() -> HardwareRequirements {
        HardwareRequirements { gpu: None, min_ram_gb: None, min_bandwidth_mbps: None }
    }

    #[tokio::test]
    async fn test_route_capacity_first() {
        let registry = registry_with_nodes(vec![
            make_node("low", 0.2, 8),
            make_node("high", 0.9, 8),
            make_node("mid", 0.5, 8),
        ]).await;
        let router = TaskRouter::new(registry, RoutingStrategy::CapacityFirst);
        let task = Task::new(no_requirements());
        let node_id = router.route(&task).await.unwrap();
        assert_eq!(node_id, "high");
    }

    #[tokio::test]
    async fn test_round_robin_distribution() {
        let registry = registry_with_nodes(vec![
            make_node("a", 0.5, 8),
            make_node("b", 0.5, 8),
        ]).await;
        let router = TaskRouter::new(registry, RoutingStrategy::RoundRobin);

        let t1 = Task::new(no_requirements());
        let t2 = Task::new(no_requirements());
        let id1 = router.route(&t1).await.unwrap();
        let id2 = router.route(&t2).await.unwrap();
        // Both nodes should have received one task each
        assert_ne!(id1, id2);
    }

    #[tokio::test]
    async fn test_no_eligible_node_returns_err() {
        let registry = registry_with_nodes(vec![
            make_node("a", 0.5, 4),
        ]).await;
        let router = TaskRouter::new(registry, RoutingStrategy::CapacityFirst);
        let task = Task {
            id: "t1".to_string(),
            requirements: HardwareRequirements { gpu: None, min_ram_gb: Some(64), min_bandwidth_mbps: None },
            priority: 0,
        };
        let result = router.route(&task).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_release_task() {
        let registry = registry_with_nodes(vec![make_node("n1", 0.8, 8)]).await;
        let router = TaskRouter::new(registry, RoutingStrategy::CapacityFirst);
        let task = Task::new(no_requirements());
        let task_id = task.id.clone();
        let node_id = router.route(&task).await.unwrap();
        assert_eq!(router.node_task_count(&node_id).await, 1);
        router.release_task(&node_id, &task_id).await;
        assert_eq!(router.node_task_count(&node_id).await, 0);
    }
}
