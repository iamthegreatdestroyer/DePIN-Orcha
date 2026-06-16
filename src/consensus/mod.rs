//! HELIX Consensus Client — Epoch 5
//!
//! DePIN-Orcha calls the HELIX swarm before executing any allocation change.
//! The swarm votes based on HDS resonance; majority approval is required.
//!
//! Integration point: `ReallocationEngine::execute_reallocation()` calls
//! `HelixConsensusClient::vote()` before applying any `AllocationPlan`.

pub mod client;

pub use client::{ConsensusResult, ConsensusStatus, HelixConsensusClient};

/// Default HELIX consensus endpoint.
/// Override with `HELIX_CONSENSUS_URL` environment variable.
pub const DEFAULT_HELIX_CONSENSUS_URL: &str = "http://localhost:8000/consensus/vote";
