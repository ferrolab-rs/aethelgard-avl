use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum SovereignError {
    #[error("Node not found for index: {0}")]
    NodeNotFound(u32),

    #[error("Stale node index (Generation mismatch)")]
    StaleIndex,

    #[error("Duplicate key in AVL tree")]
    DuplicateKey,

    #[error("Sovereign Integrity Violation: Hash mismatch detected at node {0}")]
    IntegrityViolation(u32),

    #[error("Internal storage capacity exceeded")]
    CapacityExceeded,
}

pub type Result<T> = std::result::Result<T, SovereignError>;
