//! Node Registry
//!
//! Thread-safe in-memory registry for DePIN nodes.
//! Manages registration, discovery, and health monitoring.

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;
use uuid::Uuid;

// ============================================================================
// TYPES
// ============================================================================

pub type NodeId = String;

#[derive(Debug, Clone, PartialEq)]
pub enum NodeStatus {
    Online,
    Offline,
    Degraded,
}

#[derive(Debug, Clone)]
pub struct HardwareSpecs {
    pub gpu: bool,
    pub ram_gb: u64,
    pub bandwidth_mbps: u64,
}

#[derive(Debug, Clone)]
pub struct HardwareRequirements {
    pub gpu: Option<bool>,
    pub min_ram_gb: Option<u64>,
    pub min_bandwidth_mbps: Option<u64>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum HealthStatus {
    Healthy,
    Unhealthy,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub id: NodeId,
    pub address: SocketAddr,
    pub hardware_specs: HardwareSpecs,
    pub capacity: f64, // 0.0–1.0 available fraction
    pub status: NodeStatus,
    pub last_seen: Instant,
}

// ============================================================================
// NODE REGISTRY
// ============================================================================

pub struct NodeRegistry {
    nodes: Arc<RwLock<HashMap<NodeId, Node>>>,
    health_timeout_secs: u64,
}

impl NodeRegistry {
    pub fn new(health_timeout_secs: u64) -> Self {
        Self {
            nodes: Arc::new(RwLock::new(HashMap::new())),
            health_timeout_secs,
        }
    }

    pub async fn register(&self, mut node: Node) -> Result<NodeId, String> {
        if node.capacity < 0.0 || node.capacity > 1.0 {
            return Err(format!("capacity {} out of range 0.0–1.0", node.capacity));
        }
        let id = if node.id.is_empty() {
            Uuid::new_v4().to_string()
        } else {
            node.id.clone()
        };
        node.id = id.clone();
        node.last_seen = Instant::now();
        self.nodes.write().await.insert(id.clone(), node);
        Ok(id)
    }

    pub async fn deregister(&self, node_id: &NodeId) -> Result<(), String> {
        let mut nodes = self.nodes.write().await;
        nodes
            .remove(node_id)
            .ok_or_else(|| format!("node {} not found", node_id))?;
        Ok(())
    }

    pub async fn discover(&self, requirements: &HardwareRequirements) -> Vec<Node> {
        self.nodes
            .read()
            .await
            .values()
            .filter(|n| {
                n.status == NodeStatus::Online
                    && requirements.gpu.is_none_or(|g| n.hardware_specs.gpu == g)
                    && requirements
                        .min_ram_gb
                        .is_none_or(|r| n.hardware_specs.ram_gb >= r)
                    && requirements
                        .min_bandwidth_mbps
                        .is_none_or(|b| n.hardware_specs.bandwidth_mbps >= b)
            })
            .cloned()
            .collect()
    }

    pub async fn health_check(&self) -> HashMap<NodeId, HealthStatus> {
        let timeout = self.health_timeout_secs;
        self.nodes
            .read()
            .await
            .iter()
            .map(|(id, node)| {
                let status = if node.last_seen.elapsed().as_secs() > timeout {
                    HealthStatus::Unhealthy
                } else if node.status == NodeStatus::Online {
                    HealthStatus::Healthy
                } else {
                    HealthStatus::Unknown
                };
                (id.clone(), status)
            })
            .collect()
    }

    pub async fn heartbeat(&self, node_id: &NodeId) -> Result<(), String> {
        let mut nodes = self.nodes.write().await;
        let node = nodes
            .get_mut(node_id)
            .ok_or_else(|| format!("node {} not found", node_id))?;
        node.last_seen = Instant::now();
        Ok(())
    }

    pub async fn node_count(&self) -> usize {
        self.nodes.read().await.len()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn make_node(gpu: bool, ram_gb: u64, bandwidth_mbps: u64, capacity: f64) -> Node {
        Node {
            id: String::new(),
            address: "127.0.0.1:9000".parse().unwrap(),
            hardware_specs: HardwareSpecs { gpu, ram_gb, bandwidth_mbps },
            capacity,
            status: NodeStatus::Online,
            last_seen: Instant::now(),
        }
    }

    #[tokio::test]
    async fn test_register_deregister() {
        let registry = NodeRegistry::new(60);
        let node = make_node(true, 16, 1000, 0.8);
        let id = registry.register(node).await.unwrap();
        assert_eq!(registry.node_count().await, 1);
        registry.deregister(&id).await.unwrap();
        assert_eq!(registry.node_count().await, 0);
    }

    #[tokio::test]
    async fn test_discover_by_requirements() {
        let registry = NodeRegistry::new(60);
        registry.register(make_node(true, 32, 2000, 0.9)).await.unwrap();
        registry.register(make_node(false, 8, 500, 0.5)).await.unwrap();

        let gpu_req = HardwareRequirements { gpu: Some(true), min_ram_gb: None, min_bandwidth_mbps: None };
        let results = registry.discover(&gpu_req).await;
        assert_eq!(results.len(), 1);
        assert!(results[0].hardware_specs.gpu);

        let ram_req = HardwareRequirements { gpu: None, min_ram_gb: Some(16), min_bandwidth_mbps: None };
        let results = registry.discover(&ram_req).await;
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].hardware_specs.ram_gb, 32);
    }

    #[tokio::test]
    async fn test_health_check() {
        let registry = NodeRegistry::new(60);
        let node = make_node(false, 8, 100, 0.5);
        let id = registry.register(node).await.unwrap();
        let health = registry.health_check().await;
        assert_eq!(health[&id], HealthStatus::Healthy);
    }
}
