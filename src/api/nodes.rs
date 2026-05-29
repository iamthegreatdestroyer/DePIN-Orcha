//! Node Registry & Task Router HTTP Handlers
//!
//! POST /nodes/register, DELETE /nodes/:id, GET /nodes/discover, POST /tasks/route

use actix_web::{web, HttpResponse, Result as ActixResult};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

use crate::nodes::{HardwareRequirements, HardwareSpecs, Node, NodeStatus};
use crate::router::Task;

use super::models::{ErrorResponse, SuccessResponse};
use super::AppState;

// ============================================================================
// REQUEST / RESPONSE TYPES
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct RegisterNodeRequest {
    pub id: Option<String>,
    pub address: String,
    pub gpu: bool,
    pub ram_gb: u64,
    pub bandwidth_mbps: u64,
    pub capacity: f64,
}

#[derive(Debug, Serialize)]
pub struct RegisterNodeResponse {
    pub node_id: String,
}

#[derive(Debug, Deserialize)]
pub struct DiscoverQuery {
    pub gpu: Option<bool>,
    pub min_ram_gb: Option<u64>,
    pub min_bandwidth_mbps: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct NodeDto {
    pub id: String,
    pub address: String,
    pub gpu: bool,
    pub ram_gb: u64,
    pub bandwidth_mbps: u64,
    pub capacity: f64,
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct RouteTaskRequest {
    pub gpu: Option<bool>,
    pub min_ram_gb: Option<u64>,
    pub min_bandwidth_mbps: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct RouteTaskResponse {
    pub task_id: String,
    pub node_id: String,
}

// ============================================================================
// HANDLERS
// ============================================================================

/// POST /nodes/register
pub async fn register_node(
    state: web::Data<AppState>,
    req: web::Json<RegisterNodeRequest>,
) -> ActixResult<HttpResponse> {
    let address: SocketAddr = match req.address.parse() {
        Ok(a) => a,
        Err(e) => {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse::new(
                "INVALID_ADDRESS".to_string(),
                format!("invalid socket address: {}", e),
            )));
        }
    };

    if req.capacity < 0.0 || req.capacity > 1.0 {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse::new(
            "INVALID_CAPACITY".to_string(),
            "capacity must be in 0.0–1.0".to_string(),
        )));
    }

    let node = Node {
        id: req.id.clone().unwrap_or_default(),
        address,
        hardware_specs: HardwareSpecs {
            gpu: req.gpu,
            ram_gb: req.ram_gb,
            bandwidth_mbps: req.bandwidth_mbps,
        },
        capacity: req.capacity,
        status: NodeStatus::Online,
        last_seen: std::time::Instant::now(),
    };

    match state.node_registry.register(node).await {
        Ok(node_id) => Ok(HttpResponse::Created()
            .json(SuccessResponse::new(RegisterNodeResponse { node_id }))),
        Err(e) => Ok(HttpResponse::BadRequest().json(ErrorResponse::new(
            "REGISTER_FAILED".to_string(),
            e,
        ))),
    }
}

/// DELETE /nodes/{id}
pub async fn deregister_node(
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    let node_id = path.into_inner();
    match state.node_registry.deregister(&node_id).await {
        Ok(()) => Ok(HttpResponse::Ok().json(SuccessResponse::new(
            serde_json::json!({ "deregistered": node_id }),
        ))),
        Err(e) => Ok(HttpResponse::NotFound().json(ErrorResponse::new(
            "NOT_FOUND".to_string(),
            e,
        ))),
    }
}

/// GET /nodes/discover?gpu=true&min_ram_gb=8&min_bandwidth_mbps=100
pub async fn discover_nodes(
    state: web::Data<AppState>,
    query: web::Query<DiscoverQuery>,
) -> ActixResult<HttpResponse> {
    let requirements = HardwareRequirements {
        gpu: query.gpu,
        min_ram_gb: query.min_ram_gb,
        min_bandwidth_mbps: query.min_bandwidth_mbps,
    };

    let nodes = state.node_registry.discover(&requirements).await;
    let dtos: Vec<NodeDto> = nodes
        .into_iter()
        .map(|n| NodeDto {
            id: n.id,
            address: n.address.to_string(),
            gpu: n.hardware_specs.gpu,
            ram_gb: n.hardware_specs.ram_gb,
            bandwidth_mbps: n.hardware_specs.bandwidth_mbps,
            capacity: n.capacity,
            status: format!("{:?}", n.status),
        })
        .collect();

    Ok(HttpResponse::Ok().json(SuccessResponse::new(dtos)))
}

/// POST /tasks/route
pub async fn route_task(
    state: web::Data<AppState>,
    req: web::Json<RouteTaskRequest>,
) -> ActixResult<HttpResponse> {
    let task = Task::new(HardwareRequirements {
        gpu: req.gpu,
        min_ram_gb: req.min_ram_gb,
        min_bandwidth_mbps: req.min_bandwidth_mbps,
    });
    let task_id = task.id.clone();

    match state.task_router.route(&task).await {
        Ok(node_id) => Ok(HttpResponse::Ok().json(SuccessResponse::new(
            RouteTaskResponse { task_id, node_id },
        ))),
        Err(e) => Ok(HttpResponse::ServiceUnavailable().json(ErrorResponse::new(
            "NO_NODE".to_string(),
            e,
        ))),
    }
}
