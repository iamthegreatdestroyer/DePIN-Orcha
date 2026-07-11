//! HELIX Consensus HTTP Client
//!
//! Calls HELIX `POST /consensus/vote` before DePIN-Orcha executes any
//! resource reallocation.  Non-blocking: if HELIX is unreachable the
//! call returns `ConsensusStatus::Bypassed` rather than blocking the
//! orchestration pipeline.

use super::DEFAULT_HELIX_CONSENSUS_URL;
use crate::orchestration::AllocationPlan;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::time::Duration;
use tracing::{info, warn};

// ============================================================================
// DATA TYPES
// ============================================================================

/// Status of a consensus call
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConsensusStatus {
    /// Swarm approved the proposal
    Approved,
    /// Swarm rejected the proposal
    Rejected,
    /// HELIX unreachable — caller decides whether to proceed
    Bypassed,
}

/// Result returned by `HelixConsensusClient::vote`
#[derive(Debug, Clone)]
pub struct ConsensusResult {
    pub status: ConsensusStatus,
    pub score: f64,
    pub node_id: String,
    pub reason: String,
    pub eif: f64,
    pub clq: f64,
    pub dfi: f64,
}

impl ConsensusResult {
    /// Approved bypass — used when HELIX is unreachable
    pub fn bypassed(reason: &str) -> Self {
        Self {
            status: ConsensusStatus::Bypassed,
            score: 1.0,
            node_id: "bypassed".to_string(),
            reason: reason.to_string(),
            eif: 0.0,
            clq: 0.0,
            dfi: 0.0,
        }
    }

    pub fn is_approved(&self) -> bool {
        matches!(self.status, ConsensusStatus::Approved | ConsensusStatus::Bypassed)
    }

    /// Denied bypass — used when HELIX is unreachable and bypass_on_error is false.
    /// Fail-safe: an unreachable consensus service must not be treated as approval.
    pub fn blocked(reason: &str) -> Self {
        Self {
            status: ConsensusStatus::Rejected,
            score: 0.0,
            node_id: "unreachable".to_string(),
            reason: reason.to_string(),
            eif: 0.0,
            clq: 0.0,
            dfi: 0.0,
        }
    }
}

// ── Wire format (matches HELIX ConsensusVote.dict()) ─────────────────────────

#[derive(Serialize)]
struct ProposalRequest<'a> {
    proposal_id: &'a str,
    allocation: &'a std::collections::HashMap<String, f64>,
    description: &'a str,
    estimated_improvement: f64,
    requester: &'static str,
}

#[derive(Deserialize)]
struct HelixVoteResponse {
    approved: bool,
    score: f64,
    node_id: String,
    reason: String,
    #[serde(rename = "threshold", default)]
    _threshold: f64,
    #[serde(default)]
    metrics: HelixMetrics,
}

#[derive(Deserialize, Default)]
struct HelixMetrics {
    #[serde(default)]
    eif: f64,
    #[serde(default)]
    clq: f64,
    #[serde(default)]
    dfi: f64,
}

// ============================================================================
// CLIENT
// ============================================================================

/// HTTP client that calls the HELIX swarm consensus endpoint.
pub struct HelixConsensusClient {
    helix_url: String,
    client: Client,
    /// When true, treat HELIX-unreachable as approved (non-blocking mode)
    bypass_on_error: bool,
}

impl HelixConsensusClient {
    /// Create a new client.
    ///
    /// Reads `HELIX_CONSENSUS_URL` from the environment, falls back to
    /// `DEFAULT_HELIX_CONSENSUS_URL`.  Set `HELIX_CONSENSUS_BYPASS=true`
    /// to allow reallocations when HELIX is offline.
    pub fn new() -> Self {
        let url = env::var("HELIX_CONSENSUS_URL")
            .unwrap_or_else(|_| DEFAULT_HELIX_CONSENSUS_URL.to_string());
        let bypass = env::var("HELIX_CONSENSUS_BYPASS")
            .map(|v| v == "true" || v == "1")
            .unwrap_or(false); // default: fail-safe (deny when HELIX is unreachable).
            // Set HELIX_CONSENSUS_BYPASS=true to opt into the old non-blocking behavior.

        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .expect("Failed to build reqwest client");

        Self {
            helix_url: url,
            client,
            bypass_on_error: bypass,
        }
    }

    /// Vote on an `AllocationPlan` with the HELIX swarm.
    ///
    /// Returns `ConsensusResult::Bypassed` (approved) when HELIX is
    /// unreachable and `bypass_on_error` is true.
    pub async fn vote(&self, plan: &AllocationPlan, proposal_id: &str) -> ConsensusResult {
        let description = format!(
            "Reallocation: {} protocols, estimated improvement {:.2}%",
            plan.allocation.len(),
            plan.estimated_improvement * 100.0,
        );

        let payload = ProposalRequest {
            proposal_id,
            allocation: &plan.allocation,
            description: &description,
            estimated_improvement: plan.estimated_improvement,
            requester: "depin-orcha",
        };

        match self.client.post(&self.helix_url).json(&payload).send().await {
            Err(e) => {
                warn!(
                    "HELIX consensus unreachable at {}: {} — {}",
                    self.helix_url,
                    e,
                    if self.bypass_on_error { "bypassing" } else { "blocking" }
                );
                if self.bypass_on_error {
                    ConsensusResult::bypassed(&format!("HELIX unreachable: {e}"))
                } else {
                    ConsensusResult::blocked(&format!("HELIX unreachable: {e}"))
                }
            }
            Ok(resp) => {
                let status_code = resp.status();
                match resp.json::<HelixVoteResponse>().await {
                    Err(e) => {
                        warn!(
                            "HELIX response parse error (HTTP {}): {} — {}",
                            status_code, e,
                            if self.bypass_on_error { "bypassing" } else { "blocking" }
                        );
                        if self.bypass_on_error {
                            ConsensusResult::bypassed(&format!("Parse error: {e}"))
                        } else {
                            ConsensusResult::blocked(&format!("Parse error: {e}"))
                        }
                    }
                    Ok(vote) => {
                        let status = if vote.approved {
                            ConsensusStatus::Approved
                        } else {
                            ConsensusStatus::Rejected
                        };
                        info!(
                            "HELIX consensus: proposal={} approved={} score={:.3} node={}",
                            proposal_id, vote.approved, vote.score, vote.node_id
                        );
                        ConsensusResult {
                            status,
                            score: vote.score,
                            node_id: vote.node_id,
                            reason: vote.reason,
                            eif: vote.metrics.eif,
                            clq: vote.metrics.clq,
                            dfi: vote.metrics.dfi,
                        }
                    }
                }
            }
        }
    }
}

impl Default for HelixConsensusClient {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bypassed_is_approved() {
        let r = ConsensusResult::bypassed("test");
        assert!(r.is_approved());
        assert_eq!(r.status, ConsensusStatus::Bypassed);
    }

    #[test]
    fn test_approved_is_approved() {
        let r = ConsensusResult {
            status: ConsensusStatus::Approved,
            score: 0.8,
            node_id: "helix-node-0".to_string(),
            reason: "ok".to_string(),
            eif: 0.9,
            clq: 0.5,
            dfi: 0.7,
        };
        assert!(r.is_approved());
    }

    #[test]
    fn test_rejected_not_approved() {
        let r = ConsensusResult {
            status: ConsensusStatus::Rejected,
            score: 0.2,
            node_id: "helix-node-0".to_string(),
            reason: "low resonance".to_string(),
            eif: 0.3,
            clq: 0.1,
            dfi: 0.2,
        };
        assert!(!r.is_approved());
    }

    #[test]
    fn test_blocked_not_approved() {
        let r = ConsensusResult::blocked("HELIX unreachable: connection refused");
        assert!(!r.is_approved());
        assert_eq!(r.status, ConsensusStatus::Rejected);
    }

    #[test]
    fn test_client_default_uses_env_fallback() {
        let client = HelixConsensusClient::new();
        assert!(!client.helix_url.is_empty());
    }

    #[test]
    fn test_bypass_defaults_to_false_without_env_override() {
        // Fail-safe: absent HELIX_CONSENSUS_BYPASS, the gate must deny on error, not approve.
        std::env::remove_var("HELIX_CONSENSUS_BYPASS");
        let client = HelixConsensusClient::new();
        assert!(!client.bypass_on_error);
    }
}
