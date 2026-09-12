//! Global simulation resources stored in the Bevy ECS World.

use std::collections::HashMap;
use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};
use super::types::ResourceAmount;

/// Current discrete simulation tick.
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct CurrentTick(pub u64);

/// Simulation timing metadata.
#[derive(Resource, Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct SimulationTime {
    /// Active tick counter.
    pub tick: u64,
    /// Time elapsed per tick in seconds.
    pub delta_time_seconds: f64,
    /// Total simulated elapsed time in seconds.
    pub elapsed_seconds: f64,
}

/// A specific resource ingredient and stoichiometric quantity for a recipe.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecipeIngredient {
    /// Resource element identifier.
    pub element_id: u32,
    /// Exact micro-unit quantity required or produced.
    pub amount: ResourceAmount,
}

/// Stoichiometric conversion recipe definition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Recipe {
    /// Unique recipe identifier.
    pub id: u32,
    /// Input ingredients consumed per conversion cycle.
    pub inputs: Vec<RecipeIngredient>,
    /// Output ingredients yielded per conversion cycle.
    pub outputs: Vec<RecipeIngredient>,
}

/// Global registry of transformation recipes.
#[derive(Resource, Debug, Clone, Default)]
pub struct RecipeRegistry {
    /// Map of recipe ID to recipe definition.
    pub recipes: HashMap<u32, Recipe>,
}

impl RecipeRegistry {
    /// Creates an empty recipe registry.
    #[must_use]
    pub fn new() -> Self {
        Self {
            recipes: HashMap::new(),
        }
    }

    /// Registers a new conversion recipe.
    pub fn register(&mut self, recipe: Recipe) {
        self.recipes.insert(recipe.id, recipe);
    }

    /// Retrieves a recipe by ID.
    #[must_use]
    pub fn get(&self, id: u32) -> Option<&Recipe> {
        self.recipes.get(&id)
    }
}

use crate::error::DomainError;

/// Recorded demand request for a single flow edge.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EdgeDemand {
    /// Entity ID of the ``FlowEdge``.
    pub edge_entity: Entity,
    /// Source stock entity.
    pub source: Entity,
    /// Destination stock entity.
    pub destination: Entity,
    /// Quantity demanded in micro-units.
    pub requested_amount: ResourceAmount,
}

/// Buffer of pending demands generated in Phase 1.
#[derive(Resource, Debug, Clone, Default)]
pub struct PendingDemand {
    /// Ordered list of demands awaiting allocation.
    pub demands: Vec<EdgeDemand>,
}

/// Result of contention resolution for a single edge.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EdgeAllocation {
    /// Entity ID of the ``FlowEdge``.
    pub edge_entity: Entity,
    /// Quantity allocated after pro-rata contention resolution.
    pub allocated_amount: ResourceAmount,
}

/// Buffer of allocated edge transfers determined in Phase 2.
#[derive(Resource, Debug, Clone, Default)]
pub struct FlowAllocations {
    /// Ordered list of finalized allocations.
    pub allocations: Vec<EdgeAllocation>,
}

/// Collector of domain errors during a simulation tick.
#[derive(Resource, Debug, Clone, Default)]
pub struct TickErrorLog {
    /// Domain errors collected across systems during the tick.
    pub errors: Vec<DomainError>,
}

impl TickErrorLog {
    /// Adds an error to the tick log.
    pub fn push(&mut self, err: DomainError) {
        self.errors.push(err);
    }

    /// Returns the first error if any occurred, clearing the log.
    pub fn take_first(&mut self) -> Option<DomainError> {
        if self.errors.is_empty() {
            None
        } else {
            Some(self.errors.remove(0))
        }
    }
}
