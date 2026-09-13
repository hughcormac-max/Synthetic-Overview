//! Serializable Data Transfer Objects (DTOs) for Tauri IPC and client communication.
//!
//! Enforces plain-text ASCII math notation and pure data layout across the serialization boundary.

use serde::{Deserialize, Serialize};
use crate::orbital::OrbitalState;

/// DTO representing a resource definition in the simulation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceDto {
    /// Unique resource identifier.
    pub id: u32,
    /// Human-readable resource name.
    pub name: String,
}

/// DTO representing a storage node's current amount and capacity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StorageDto {
    /// Entity ID of the storage component.
    pub entity_id: u32,
    /// Unique resource identifier stored.
    pub resource_id: u32,
    /// Current stored resource quantity in fixed-point micro-units.
    pub amount: i64,
    /// Maximum storage capacity in fixed-point micro-units.
    pub capacity: i64,
}

/// DTO representing a converter entity and its active recipe.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConverterDto {
    /// Entity ID of the converter component.
    pub entity_id: u32,
    /// Recipe identifier matching an entry in `RecipeRegistry`.
    pub recipe_id: u32,
}

/// DTO representing a flow edge and its in-transit packet sum.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FlowEdgeDto {
    /// Entity ID of the flow edge.
    pub edge_id: u32,
    /// Entity ID of source storage.
    pub source_id: u32,
    /// Entity ID of destination storage.
    pub destination_id: u32,
    /// Total fixed-point micro-units currently traversing in transit packets.
    pub in_transit: i64,
}

/// DTO representing an astronomical body with physical radius and H3 resolution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AstroNodeDto {
    /// Entity ID of the astronomical body in the ECS.
    pub entity_id: u32,
    /// Unique body identifier (e.g., Earth = 1, Mars = 2).
    pub body_id: u32,
    /// Mean physical radius in kilometers.
    pub radius_km: u32,
    /// Base H3 grid resolution for surface indexing.
    pub h3_resolution: u8,
}

/// DTO representing an entity positioned on a planetary surface via an H3 cell index.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceNodeDto {
    /// Entity ID of the surface node in the ECS.
    pub entity_id: u32,
    /// Body identifier of the parent astronomical body.
    pub parent_body_id: u32,
    /// H3 hexagonal cell index formatted as a 64-bit hexadecimal string.
    pub h3_cell_index: String,
}

/// Serializable DTO conveying simulation state across the Tauri IPC boundary to the webview.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimulationStateDto {
    /// Active simulation tick.
    pub tick: u64,
    /// Total elapsed simulation time in seconds.
    pub timestamp_seconds: f64,
    /// Step interval in seconds for this tick.
    pub delta_time_seconds: f64,
    /// Active celestial bodies with orbital propagation parameters.
    pub entities: Vec<OrbitalState>,
    /// Global resource directory list.
    pub resources: Vec<ResourceDto>,
    /// Active storage reservoirs.
    pub storages: Vec<StorageDto>,
    /// Active converters.
    pub converters: Vec<ConverterDto>,
    /// Active flow edges.
    pub edges: Vec<FlowEdgeDto>,
    /// Active astronomical bodies for surface rendering.
    pub astro_nodes: Vec<AstroNodeDto>,
    /// Active surface facilities attached to H3 cells.
    pub surface_nodes: Vec<SurfaceNodeDto>,
}
