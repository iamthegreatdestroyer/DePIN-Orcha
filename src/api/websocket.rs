/// WebSocket Real-Time Updates
///
/// Handles WebSocket connections for real-time dashboard updates.

use actix_web::{web, HttpRequest, HttpResponse};
use actix_ws::Message;
use futures::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::broadcast;
use serde_json::json;
use chrono::Utc;

use crate::orchestration::AggregatedMetrics;
use super::models::{WsMessage, MetricsSnapshot};
use super::AppState;

/// WebSocket connection handler
pub async fn ws_handler(
    req: HttpRequest,
    stream: web::Payload,
    state: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    let (response, session, msg_stream) = actix_ws::handle(&req, stream)?;

    // Spawn a task to handle the WebSocket session
    actix_web::rt::spawn(handle_ws_session(
        session,
        msg_stream,
        state.into_inner(),
    ));

    Ok(response)
}

/// Handle a WebSocket session
async fn handle_ws_session(
    mut session: actix_ws::Session,
    mut msg_stream: actix_ws::MessageStream,
    state: Arc<AppState>,
) {
    tracing::info!("WebSocket connection established");

    let mut subscriptions = std::collections::HashSet::new();
    subscriptions.insert("metrics".to_string());

    // Send initial connection message
    let init_msg = json!({
        "type": "connected",
        "message": "Connected to DePIN-Orcha Dashboard",
        "timestamp": Utc::now().to_rfc3339(),
    });

    if let Err(e) = session.text(init_msg.to_string()).await {
        tracing::error!("Failed to send init message: {}", e);
        return;
    }

    // Main WebSocket loop
    let mut ticker = tokio::time::interval(std::time::Duration::from_secs(5));

    loop {
        tokio::select! {
            // Handle incoming WebSocket messages
            msg = msg_stream.next() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        if let Ok(ws_msg) = serde_json::from_str::<WsMessage>(&text) {
                            match ws_msg {
                                WsMessage::Subscribe { protocol } => {
                                    if let Some(proto) = protocol {
                                        subscriptions.insert(format!("metrics:{}", proto));
                                        let response = json!({
                                            "type": "subscribed",
                                            "protocol": proto,
                                        });
                                        let _ = session.text(response.to_string()).await;
                                    }
                                }
                                WsMessage::Unsubscribe { protocol } => {
                                    if let Some(proto) = protocol {
                                        subscriptions.remove(&format!("metrics:{}", proto));
                                    }
                                }
                                WsMessage::Ping => {
                                    let _ = session.text(json!({"type": "pong"}).to_string()).await;
                                }
                                _ => {}
                            }
                        }
                    }
                    Some(Ok(Message::Close(_))) => {
                        tracing::info!("WebSocket connection closed");
                        break;
                    }
                    None => {
                        tracing::info!("WebSocket stream ended");
                        break;
                    }
                    _ => {}
                }
            }

            // Send periodic metrics updates
            _ = ticker.tick() => {
                if subscriptions.contains("metrics") {
                    if let Ok(Some(metrics)) = state.coordinator.get_current_metrics().await {
                        let snapshot = MetricsSnapshot {
                            timestamp: metrics.timestamp,
                            total_earnings: metrics.total_earnings_per_hour,
                            earnings_by_protocol: metrics.earnings_by_protocol,
                        };

                        let msg = WsMessage::MetricsUpdate {
                            metrics: snapshot,
                        };

                        if let Ok(json) = serde_json::to_string(&msg) {
                            let _ = session.text(json).await;
                        }
                    }
                }
            }
        }
    }

    tracing::info!("WebSocket session ended");
}

/// Health check for WebSocket endpoints
pub async fn ws_health() -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "ready",
        "endpoint": "/ws",
    })))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ws_message_serialization() {
        let msg = WsMessage::Ping;
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("Ping"));
    }

    #[test]
    fn test_metrics_snapshot_creation() {
        let snapshot = MetricsSnapshot {
            timestamp: Utc::now(),
            total_earnings: 10.5,
            earnings_by_protocol: std::collections::HashMap::new(),
        };
        assert_eq!(snapshot.total_earnings, 10.5);
    }
}
