use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Domain error representations for simulation calculations and model validation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Error)]
pub enum DomainError {
    #[error("Eccentricity must satisfy 0.0 <= e < 1.0, received: {0}")]
    InvalidEccentricity(String),

    #[error("Orbital period must be strictly greater than zero, received: {0}")]
    InvalidOrbitalPeriod(String),

    #[error("Semi-major axis must be strictly greater than zero, received: {0}")]
    InvalidSemiMajorAxis(String),

    #[error("Kepler equation solver failed to converge within maximum iterations: {0}")]
    ConvergenceFailure(String),

    #[error("Resource amount operation overflowed: {0}")]
    ResourceOverflow(String),

    #[error("Resource amount cannot be negative: received {0}")]
    NegativeResourceAmount(i64),

    #[error("Insufficient inventory: available {available}, requested {requested}")]
    InsufficientInventory { available: i64, requested: i64 },

    #[error("Storage capacity exceeded: current {current}, capacity {capacity}")]
    CapacityExceeded { current: i64, capacity: i64 },

    #[error("Entity not found: {0}")]
    EntityNotFound(String),

    #[error("Simulation tick error: {0}")]
    SimulationTickError(String),
}
