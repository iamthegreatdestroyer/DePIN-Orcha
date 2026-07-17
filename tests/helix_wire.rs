//! Live end-to-end proof that DePIN-Orcha's real consensus client reaches the
//! HELIX consensus node (which computes the vote via hds_core / HDS resonance).
//!
//! Ignored by default: it requires `helix-consensus.service` to be reachable at
//! the configured `HELIX_CONSENSUS_URL` (default `http://127.0.0.1:8770/consensus/vote`).
//! Run with: `cargo test --test helix_wire -- --ignored --nocapture`
use std::collections::HashMap;

use chrono::Utc;
use depin_orcha::consensus::{ConsensusStatus, HelixConsensusClient};
use depin_orcha::orchestration::AllocationPlan;

#[tokio::test]
#[ignore = "requires a live HELIX consensus node (helix-consensus.service)"]
async fn depin_reaches_live_helix_consensus() {
    let mut allocation = HashMap::new();
    allocation.insert("storj".to_string(), 0.7);
    allocation.insert("grass".to_string(), 0.3);
    let plan = AllocationPlan {
        allocation,
        estimated_improvement: 0.2,
        estimated_cost: 0.01,
        net_benefit: 0.19,
        roi_percent: 19.0,
        confidence: 0.8,
        created_at: Utc::now(),
    };

    // Uses the compiled-in default URL (now the real HELIX node, not the gateway).
    let client = HelixConsensusClient::new();
    let result = client.vote(&plan, "e2e-wire-test").await;

    println!(
        "HELIX consensus: status={:?} score={:.3} node_id={} reason={}",
        result.status, result.score, result.node_id, result.reason
    );

    // A real HELIX response is Approved or Rejected; unreachable HELIX -> Bypassed.
    assert!(
        matches!(result.status, ConsensusStatus::Approved | ConsensusStatus::Rejected),
        "expected a real HELIX vote (Approved/Rejected) — got {:?}; is HELIX reachable?",
        result.status
    );
    // The bypass path self-identifies as "bypassed"; a real node does not.
    assert_ne!(
        result.node_id, "bypassed",
        "vote fell through to the bypass path — DePIN did not actually reach HELIX"
    );
}
