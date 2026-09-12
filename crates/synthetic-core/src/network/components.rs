//! Core ECS components for the stock-and-flow network.
//!
//! Stock reservoirs, stoichiometric converters, flow edges, and sparse-set tags.

use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};
use crate::error::DomainError;
use super::types::ResourceAmount;

/// Passive storage reservoir holding discrete quantities of a resource element.
#[derive(Component, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Storage {
    /// Unique resource or element identifier.
    pub element_id: u32,
    /// Current stored resource quantity in fixed-point micro-units.
    pub current_amount: ResourceAmount,
    /// Maximum storage capacity in fixed-point micro-units.
    pub capacity: ResourceAmount,
}

impl Storage {
    /// Creates a new storage container, validating non-negativity and capacity bounds.
    ///
    /// # Errors
    /// Returns `DomainError` if current amount is negative or exceeds capacity.
    pub fn new(
        element_id: u32,
        current_amount: ResourceAmount,
        capacity: ResourceAmount,
    ) -> Result<Self, DomainError> {
        current_amount.ensure_non_negative()?;
        capacity.ensure_non_negative()?;

        if current_amount > capacity {
            return Err(DomainError::CapacityExceeded {
                current: current_amount.to_raw(),
                capacity: capacity.to_raw(),
            });
        }

        Ok(Self {
            element_id,
            current_amount,
            capacity,
        })
    }

    /// Calculates remaining capacity available for resource intake.
    #[must_use]
    pub fn available_capacity(&self) -> ResourceAmount {
        self.capacity.saturating_sub(self.current_amount)
    }

    /// Deposits resources into this storage reservoir.
    ///
    /// # Errors
    /// Returns `DomainError::CapacityExceeded` if deposit overflows capacity.
    pub fn deposit(&mut self, amount: ResourceAmount) -> Result<(), DomainError> {
        amount.ensure_non_negative()?;
        let updated = self.current_amount.checked_add(amount)?;
        if updated > self.capacity {
            return Err(DomainError::CapacityExceeded {
                current: updated.to_raw(),
                capacity: self.capacity.to_raw(),
            });
        }
        self.current_amount = updated;
        Ok(())
    }

    /// Withdraws resources from this storage reservoir.
    ///
    /// # Errors
    /// Returns `DomainError::InsufficientInventory` if amount exceeds available stock.
    pub fn withdraw(&mut self, amount: ResourceAmount) -> Result<(), DomainError> {
        amount.ensure_non_negative()?;
        if amount > self.current_amount {
            return Err(DomainError::InsufficientInventory {
                available: self.current_amount.to_raw(),
                requested: amount.to_raw(),
            });
        }
        self.current_amount = self.current_amount.checked_sub(amount)?;
        Ok(())
    }
}

/// Active transformer executing stoichiometric conversion recipes.
#[derive(Component, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Converter {
    /// Recipe identifier matching an entry in `RecipeRegistry`.
    pub recipe_id: u32,
    /// Structural health / integrity (0 to 10000 representing 0.0 to 1.0).
    pub health: u16,
    /// Flag indicating whether the converter is enabled.
    pub operational_status: bool,
}

impl Converter {
    /// Maximum structural health representation (10,000 basis units = 100.0%).
    pub const MAX_HEALTH: u16 = 10_000;

    /// Creates a new converter instance.
    #[must_use]
    pub fn new(recipe_id: u32, health: u16, operational_status: bool) -> Self {
        Self {
            recipe_id,
            health: health.min(Self::MAX_HEALTH),
            operational_status,
        }
    }

    /// Returns true if the converter is active and structurally operational.
    #[must_use]
    pub fn is_operational(&self) -> bool {
        self.operational_status && self.health > 0
    }
}

/// Dynamic/Transient Tag indicating an entity is in transit.
/// Stored in Bevy's ``SparseSet`` storage to avoid archetype fragmentation.
#[derive(Component, Debug, Clone, PartialEq, Eq)]
#[component(storage = "SparseSet")]
pub struct Relocating {
    /// Destination entity identifier.
    pub destination_id: Entity,
    /// Simulation tick at which relocation finishes.
    pub arrival_tick: u64,
}

/// Directed flow edge connecting a source storage to a destination storage or converter.
#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub struct FlowEdge {
    /// Source entity (typically a Storage reservoir).
    pub source: Entity,
    /// Destination entity (typically a Storage reservoir).
    pub destination: Entity,
    /// Throughput limit per simulation tick.
    pub max_throughput: ResourceAmount,
    /// Transport latency (ticks required to traverse edge).
    pub latency: u64,
    /// Transmission efficiency in millionths (1,000,000 = 100%).
    pub efficiency_millionths: u32,
}

/// An individual discrete packet traversing a flow edge.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransitPacket {
    /// Element or resource identifier.
    pub element_id: u32,
    /// Quantity in fixed-point micro-units.
    pub amount: ResourceAmount,
    /// Absolute arrival tick (departure tick + latency).
    pub arrival_tick: u64,
    /// Destination storage entity to receive the packet upon arrival.
    pub destination: Entity,
}

/// Queue of in-flight resource packets travelling along a flow edge.
#[derive(Component, Debug, Clone, Default, PartialEq, Eq)]
pub struct FlowQueue {
    /// Ordered list of resource packets in transit.
    pub packets: Vec<TransitPacket>,
}

/// References to input and output storage reservoirs for a converter.
#[derive(Component, Debug, Clone, Default, PartialEq, Eq)]
pub struct ConverterBindings {
    /// Input storages providing recipe ingredients.
    pub inputs: Vec<Entity>,
    /// Output storages receiving conversion products.
    pub outputs: Vec<Entity>,
}

impl ConverterBindings {
    /// Constructs bindings from explicit input and output entity lists.
    #[must_use]
    pub fn new(inputs: Vec<Entity>, outputs: Vec<Entity>) -> Self {
        Self { inputs, outputs }
    }
}
