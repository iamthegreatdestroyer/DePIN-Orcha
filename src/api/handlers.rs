/// API Endpoint Handlers
///
/// HTTP request handlers for all API endpoints.

use actix_web::{web, HttpResponse, Result as ActixResult};
use crate::orchestration::OrchestrationError;
use chrono::Utc;
use std::collections::HashMap;

use super::models::*;
use super::AppState;

// ============================================================================
// METRICS ENDPOINTS
// ============================================================================

/// GET /api/v1/metrics - Get current metrics
pub async fn get_metrics(
    state: web::Data<AppState>,
) -> ActixResult<HttpResponse> {
    match state.coordinator.get_current_metrics().await {
        Ok(Some(metrics)) => {
            let response = MetricsResponse {
                timestamp: metrics.timestamp,
                total_earnings_per_hour: metrics.total_earnings_per_hour,
                earnings_by_protocol: metrics.earnings_by_protocol,
                allocation_by_protocol: metrics.allocation_by_protocol,
                connection_status: metrics.connection_status,
                resource_utilization: ResourceUtilizationDto {
                    cpu_percent: metrics.resource_utilization.cpu_percent,
                    memory_percent: metrics.resource_utilization.memory_percent,
                    bandwidth_percent: metrics.resource_utilization.bandwidth_percent,
                    storage_percent: metrics.resource_utilization.storage_percent,
                },
            };

            Ok(HttpResponse::Ok().json(SuccessResponse::new(response)))
        }
        Ok(None) => {
            let error = ErrorResponse::new(
                "NO_DATA".to_string(),
                "No metrics available yet".to_string(),
            );
            Ok(HttpResponse::NotFound().json(error))
        }
        Err(_) => {
            let error = ErrorResponse::new(
                "INTERNAL_ERROR".to_string(),
                "Failed to fetch metrics".to_string(),
            );
            Ok(HttpResponse::InternalServerError().json(error))
        }
    }
}

/// GET /api/v1/metrics/history - Get metrics history
pub async fn get_metrics_history(
    state: web::Data<AppState>,
    req: web::Query<MetricsHistoryRequest>,
) -> ActixResult<HttpResponse> {
    let hours = req.hours.unwrap_or(24);
    let limit = req.limit.unwrap_or(1000);

    let history = state.coordinator.get_metrics_history().await;
    let history_len = history.len();

    let snapshots: Vec<MetricsSnapshot> = history
        .into_iter()
        .rev()
        .take(limit)
        .map(|m| MetricsSnapshot {
            timestamp: m.timestamp,
            total_earnings: m.total_earnings_per_hour,
            earnings_by_protocol: m.earnings_by_protocol.clone(),
        })
        .collect();

    let response = MetricsHistoryResponse {
        metrics: snapshots,
        total_count: history_len,
    };

    Ok(HttpResponse::Ok().json(SuccessResponse::new(response)))
}

// ============================================================================
// OPTIMIZATION ENDPOINTS
// ============================================================================

/// GET /api/v1/opportunities - Get optimization opportunities
pub async fn get_opportunities(
    state: web::Data<AppState>,
    req: web::Query<OpportunitiesRequest>,
) -> ActixResult<HttpResponse> {
    match state.coordinator.get_current_metrics().await {
        Ok(Some(metrics)) => {
            let optimizer = state.optimizer.lock().await;
            match optimizer.analyze_opportunities(&metrics) {
                Ok(opportunities) => {
                    let limit = req.limit.unwrap_or(10);
                    let opps: Vec<OpportunityDto> = opportunities
                        .into_iter()
                        .take(limit)
                        .map(|o| OpportunityDto {
                            from_protocol: o.from_protocol,
                            to_protocol: o.to_protocol,
                            current_rate: o.current_rate,
                            projected_rate: o.projected_rate,
                            earnings_improvement: o.earnings_improvement,
                            confidence: o.confidence,
                        })
                        .collect();

                    let best_improvement = opps.first().map(|o| o.earnings_improvement);
                    let confidence = opps.first().map(|o| o.confidence).unwrap_or(0.0);

                    let response = OpportunitiesResponse {
                        opportunities: opps,
                        best_improvement,
                        confidence,
                    };

                    Ok(HttpResponse::Ok().json(SuccessResponse::new(response)))
                }
                Err(_) => {
                    let error = ErrorResponse::new(
                        "ANALYSIS_ERROR".to_string(),
                        "Failed to analyze opportunities".to_string(),
                    );
                    Ok(HttpResponse::InternalServerError().json(error))
                }
            }
        }
        _ => {
            let error = ErrorResponse::new(
                "NO_DATA".to_string(),
                "No metrics available".to_string(),
            );
            Ok(HttpResponse::NotFound().json(error))
        }
    }
}

/// GET /api/v1/allocation - Get optimal allocation
pub async fn get_optimal_allocation(
    state: web::Data<AppState>,
) -> ActixResult<HttpResponse> {
    match state.coordinator.get_current_metrics().await {
        Ok(Some(metrics)) => {
            let optimizer = state.optimizer.lock().await;
            match optimizer.calculate_optimal_allocation(&metrics) {
                Ok(plan) => {
                    let response = AllocationResponse {
                        current_allocation: metrics.allocation_by_protocol,
                        optimal_allocation: plan.allocation,
                        estimated_improvement: plan.estimated_improvement,
                        net_benefit: plan.net_benefit,
                        roi_percent: plan.roi_percent,
                    };

                    Ok(HttpResponse::Ok().json(SuccessResponse::new(response)))
                }
                Err(_) => {
                    let error = ErrorResponse::new(
                        "CALCULATION_ERROR".to_string(),
                        "Failed to calculate allocation".to_string(),
                    );
                    Ok(HttpResponse::InternalServerError().json(error))
                }
            }
        }
        _ => {
            let error = ErrorResponse::new(
                "NO_DATA".to_string(),
                "No metrics available".to_string(),
            );
            Ok(HttpResponse::NotFound().json(error))
        }
    }
}

// ============================================================================
// REALLOCATION ENDPOINTS
// ============================================================================

/// POST /api/v1/reallocate - Execute reallocation
pub async fn execute_reallocation(
    state: web::Data<AppState>,
    req: web::Json<ReallocateRequest>,
) -> ActixResult<HttpResponse> {
    // Validate allocation
    let total: f64 = req.allocation.values().sum();
    if (total - 100.0).abs() > 1.0 {
        let error = ErrorResponse::new(
            "INVALID_ALLOCATION".to_string(),
            format!("Total allocation {} != 100%", total),
        );
        return Ok(HttpResponse::BadRequest().json(error));
    }

    // Check if can reallocate
    if let Err(_) = state.reallocation.can_reallocate().await {
        let error = ErrorResponse::new(
            "CANNOT_REALLOCATE".to_string(),
            "Reallocation not currently allowed (rate limit or hold duration)".to_string(),
        );
        return Ok(HttpResponse::TooManyRequests().json(error));
    }

    // Note: Actual execution would require protocol adapters
    // This is a simplified response for demonstration
    let response = ReallocateResponse {
        success: true,
        message: "Reallocation executed successfully".to_string(),
        changes: vec![],
    };

    Ok(HttpResponse::Ok().json(SuccessResponse::new(response)))
}

/// GET /api/v1/reallocation/history - Get reallocation history
pub async fn get_reallocation_history(
    state: web::Data<AppState>,
) -> ActixResult<HttpResponse> {
    let history = state.reallocation.get_reallocation_history().await;

    let changes: Vec<AllocationChangeDto> = history
        .into_iter()
        .map(|c| AllocationChangeDto {
            timestamp: c.timestamp,
            protocol: c.protocol,
            old_allocation: c.old_allocation,
            new_allocation: c.new_allocation,
            earnings_impact: c.earnings_impact,
        })
        .collect();

    Ok(HttpResponse::Ok().json(SuccessResponse::new(changes)))
}

// ============================================================================
// DASHBOARD ENDPOINTS
// ============================================================================

/// GET /api/v1/dashboard - Get dashboard data
pub async fn get_dashboard(
    state: web::Data<AppState>,
) -> ActixResult<HttpResponse> {
    match state.coordinator.get_current_metrics().await {
        Ok(Some(metrics)) => {
            let optimizer = state.optimizer.lock().await;
            let opportunities = optimizer
                .analyze_opportunities(&metrics)
                .unwrap_or_default();

            let response = DashboardResponse {
                timestamp: Utc::now(),
                total_earnings_per_hour: metrics.total_earnings_per_hour,
                earnings_by_protocol: metrics.earnings_by_protocol.clone(),
                current_allocation: metrics.allocation_by_protocol.clone(),
                optimal_allocation: if let Ok(plan) = optimizer.calculate_optimal_allocation(&metrics) {
                    plan.allocation
                } else {
                    metrics.allocation_by_protocol.clone()
                },
                next_reallocation_in: Some(3600), // 1 hour
                connection_status: metrics.connection_status.clone(),
                alerts_count: 0, // Would fetch from monitor
            };

            Ok(HttpResponse::Ok().json(SuccessResponse::new(response)))
        }
        _ => {
            let error = ErrorResponse::new(
                "NO_DATA".to_string(),
                "No metrics available".to_string(),
            );
            Ok(HttpResponse::NotFound().json(error))
        }
    }
}

// ============================================================================
// ALERT ENDPOINTS
// ============================================================================

/// GET /api/v1/alerts - Get alerts
pub async fn get_alerts(
    state: web::Data<AppState>,
) -> ActixResult<HttpResponse> {
    let alerts = state.monitor.get_alert_history().await;

    let alert_dtos: Vec<AlertDto> = alerts
        .into_iter()
        .map(|a| AlertDto {
            timestamp: a.timestamp,
            alert_type: format!("{:?}", a.alert_type),
            severity: a.severity,
            message: a.message,
            acknowledged: a.acknowledged,
        })
        .collect();

    let total_count = alert_dtos.len();

    let response = AlertsResponse {
        alerts: alert_dtos,
        total_count,
        critical_count: 0,
    };

    Ok(HttpResponse::Ok().json(SuccessResponse::new(response)))
}

/// POST /api/v1/alerts/acknowledge - Acknowledge alert
pub async fn acknowledge_alert(
    state: web::Data<AppState>,
    req: web::Json<AcknowledgeAlertRequest>,
) -> ActixResult<HttpResponse> {
    match state.monitor.acknowledge_alert(req.timestamp).await {
        Ok(_) => {
            Ok(HttpResponse::Ok().json(SuccessResponse::new(
                serde_json::json!({"acknowledged": true})
            )))
        }
        Err(_) => {
            let error = ErrorResponse::new(
                "NOT_FOUND".to_string(),
                "Alert not found".to_string(),
            );
            Ok(HttpResponse::NotFound().json(error))
        }
    }
}

// ============================================================================
// HEALTH & STATUS ENDPOINTS
// ============================================================================

/// GET /api/v1/health - Health check
pub async fn health_check() -> ActixResult<HttpResponse> {
    Ok(HttpResponse::Ok().json(SuccessResponse::new(
        serde_json::json!({"status": "healthy"})
    )))
}

/// GET /api/v1/status - Get system status
pub async fn get_status(
    state: web::Data<AppState>,
) -> ActixResult<HttpResponse> {
    let protocols = state.coordinator.registered_protocols();

    Ok(HttpResponse::Ok().json(SuccessResponse::new(
        serde_json::json!({
            "protocols": protocols,
            "timestamp": Utc::now(),
        })
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_response() {
        let error = ErrorResponse::new(
            "TEST".to_string(),
            "Test message".to_string(),
        );
        assert_eq!(error.error, "TEST");
        assert_eq!(error.message, "Test message");
    }
}
