//! Deterministic generator for the Tier-0 multi-node economic simulation scenario.
//!
//! Generates a 40-resource economy across 200 Converter entities (Mines, Factories,
//! and Population Centers) connected via `FlowEdges` with discrete latency queues.

use std::collections::HashMap;
use bevy_ecs::prelude::*;
use crate::network::{
    Converter, ConverterBindings, FlowEdge, FlowQueue, Recipe, RecipeIngredient,
    RecipeRegistry, ResourceAmount, Storage,
};
use crate::simulation::{ResourceDirectory, ResourceDto};

/// Generates the canonical 40-resource directory:
/// - 10 Raw Resources (IDs 1..=10)
/// - 20 Intermediate Goods (IDs 11..=30)
/// - 10 Consumer Goods (IDs 31..=40)
#[must_use]
pub fn create_resource_directory() -> ResourceDirectory {
    let mut resources = Vec::with_capacity(40);
    for i in 1..=10 {
        resources.push(ResourceDto {
            id: i,
            name: format!("Raw Resource {i}"),
        });
    }
    for i in 1..=20 {
        resources.push(ResourceDto {
            id: 10 + i,
            name: format!("Intermediate Good {i}"),
        });
    }
    for i in 1..=10 {
        resources.push(ResourceDto {
            id: 30 + i,
            name: format!("Consumer Good {i}"),
        });
    }
    ResourceDirectory::new(resources)
}

/// Registers standard stoichiometric recipes for mines, factories, and population centers.
pub fn register_recipes(registry: &mut RecipeRegistry) {
    // 10 Mine Recipes: 0 inputs -> 1 raw output (10 units)
    for i in 1..=10 {
        registry.register(Recipe {
            id: i,
            inputs: vec![],
            outputs: vec![RecipeIngredient {
                element_id: i,
                amount: ResourceAmount::from_raw(10_000_000),
            }],
        });
    }

    // 20 Intermediate Factory Recipes: 2 raw inputs -> 1 intermediate output
    for j in 0..20 {
        let recipe_id = 11 + j;
        let in1 = (j % 10) + 1;
        let in2 = ((j + 1) % 10) + 1;
        registry.register(Recipe {
            id: recipe_id,
            inputs: vec![
                RecipeIngredient {
                    element_id: in1,
                    amount: ResourceAmount::from_raw(5_000_000),
                },
                RecipeIngredient {
                    element_id: in2,
                    amount: ResourceAmount::from_raw(5_000_000),
                },
            ],
            outputs: vec![RecipeIngredient {
                element_id: recipe_id,
                amount: ResourceAmount::from_raw(10_000_000),
            }],
        });
    }

    // 10 Consumer Good Factory Recipes: 2 intermediate inputs -> 1 consumer output
    for k in 0..10 {
        let recipe_id = 31 + k;
        let in1 = 11 + (k * 2) % 20;
        let in2 = 11 + (k * 2 + 1) % 20;
        registry.register(Recipe {
            id: recipe_id,
            inputs: vec![
                RecipeIngredient {
                    element_id: in1,
                    amount: ResourceAmount::from_raw(5_000_000),
                },
                RecipeIngredient {
                    element_id: in2,
                    amount: ResourceAmount::from_raw(5_000_000),
                },
            ],
            outputs: vec![RecipeIngredient {
                element_id: recipe_id,
                amount: ResourceAmount::from_raw(10_000_000),
            }],
        });
    }

    // 10 Population Center Recipes: 1 consumer input -> 0 outputs (sink)
    for m in 0..10 {
        let recipe_id = 41 + m;
        let consumer_id = 31 + m;
        registry.register(Recipe {
            id: recipe_id,
            inputs: vec![RecipeIngredient {
                element_id: consumer_id,
                amount: ResourceAmount::from_raw(5_000_000),
            }],
            outputs: vec![],
        });
    }
}

/// Helper spawning a storage entity with guaranteed non-negative capacity.
fn spawn_storage(world: &mut World, element_id: u32, amount: i64, capacity: i64) -> Entity {
    world
        .spawn(
            Storage::new(
                element_id,
                ResourceAmount::from_raw(amount),
                ResourceAmount::from_raw(capacity),
            )
            .expect("valid storage initialization"),
        )
        .id()
}

/// Converter topology grouping outputs and inputs by resource ID.
struct StorageTopology {
    outputs: HashMap<u32, Vec<Entity>>,
    inputs: HashMap<u32, Vec<Entity>>,
}

/// Spawns a single converter with its associated input and output storage reservoirs.
fn spawn_converter_instance(
    world: &mut World,
    recipe: &Recipe,
    topology: &mut StorageTopology,
) {
    let mut in_entities = Vec::with_capacity(recipe.inputs.len());
    for in_req in &recipe.inputs {
        let storage_entity = spawn_storage(world, in_req.element_id, 50_000_000, 1_000_000_000);
        in_entities.push(storage_entity);
        topology
            .inputs
            .entry(in_req.element_id)
            .or_default()
            .push(storage_entity);
    }

    let mut out_entities = Vec::with_capacity(recipe.outputs.len());
    for out_req in &recipe.outputs {
        let storage_entity = spawn_storage(world, out_req.element_id, 20_000_000, 1_000_000_000);
        out_entities.push(storage_entity);
        topology
            .outputs
            .entry(out_req.element_id)
            .or_default()
            .push(storage_entity);
    }

    world.spawn((
        Converter::new(recipe.id, 10_000, true),
        ConverterBindings::new(in_entities, out_entities),
    ));
}

/// Spawns 200 converters across 4 categories:
/// - 50 Mines (5 instances * 10 raw resources)
/// - 60 Intermediate Factories (3 instances * 20 recipes)
/// - 40 Consumer Factories (4 instances * 10 recipes)
/// - 50 Population Centers (5 instances * 10 recipes)
fn spawn_converters_and_storages(
    world: &mut World,
    registry: &RecipeRegistry,
) -> StorageTopology {
    let mut topology = StorageTopology {
        outputs: HashMap::new(),
        inputs: HashMap::new(),
    };

    // 50 Mines: recipes 1..=10, 5 instances each
    for recipe_id in 1..=10 {
        let recipe = registry.get(recipe_id).expect("mine recipe exists");
        for _ in 0..5 {
            spawn_converter_instance(world, recipe, &mut topology);
        }
    }

    // 60 Intermediate Factories: recipes 11..=30, 3 instances each
    for recipe_id in 11..=30 {
        let recipe = registry.get(recipe_id).expect("intermediate recipe exists");
        for _ in 0..3 {
            spawn_converter_instance(world, recipe, &mut topology);
        }
    }

    // 40 Consumer Factories: recipes 31..=40, 4 instances each
    for recipe_id in 31..=40 {
        let recipe = registry.get(recipe_id).expect("consumer recipe exists");
        for _ in 0..4 {
            spawn_converter_instance(world, recipe, &mut topology);
        }
    }

    // 50 Population Centers: recipes 41..=50, 5 instances each
    for recipe_id in 41..=50 {
        let recipe = registry.get(recipe_id).expect("population recipe exists");
        for _ in 0..5 {
            spawn_converter_instance(world, recipe, &mut topology);
        }
    }

    topology
}

/// Spawns flow edges linking output storages to compatible input storages.
fn spawn_flow_edges(world: &mut World, topology: &StorageTopology) {
    let mut edge_counter: u64 = 0;
    for (resource_id, input_list) in &topology.inputs {
        if let Some(output_list) = topology.outputs.get(resource_id) {
            if output_list.is_empty() {
                continue;
            }
            for (idx, &dest) in input_list.iter().enumerate() {
                let src = output_list[idx % output_list.len()];
                let latency = 1 + (edge_counter % 3);
                world.spawn((
                    FlowEdge {
                        source: src,
                        destination: dest,
                        max_throughput: ResourceAmount::from_raw(20_000_000),
                        latency,
                        efficiency_millionths: 1_000_000,
                    },
                    FlowQueue::default(),
                ));
                edge_counter += 1;
            }
        }
    }
}

/// Deterministically generates the complete economic scenario in the given Bevy ECS World.
pub fn generate_economic_scenario(world: &mut World) {
    let directory = create_resource_directory();
    world.insert_resource(directory);

    let mut registry = world
        .get_resource::<RecipeRegistry>()
        .cloned()
        .unwrap_or_default();
    register_recipes(&mut registry);

    let topology = spawn_converters_and_storages(world, &registry);
    spawn_flow_edges(world, &topology);

    world.insert_resource(registry);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::network::initialize_simulation_world;
    use crate::simulation::{SimulationSession, SimulationStateDto};

    #[test]
    fn test_scenario_generation_counts() {
        let mut world = initialize_simulation_world();
        generate_economic_scenario(&mut world);

        let dto = SimulationStateDto::from_world(&mut world);
        assert_eq!(dto.resources.len(), 40);
        assert_eq!(dto.converters.len(), 200);
        assert_eq!(dto.storages.len(), 400);
        assert_eq!(dto.edges.len(), 250);
    }

    #[test]
    fn test_simulation_session_advances_without_errors() {
        let mut session = SimulationSession::new();
        let initial_dto = session.extract_dto();
        assert_eq!(initial_dto.tick, 0);
        assert_eq!(initial_dto.converters.len(), 200);

        for _ in 0..10 {
            let stepped = session.step(86_400.0).expect("tick should succeed");
            assert!(stepped.tick > 0);
        }

        let final_dto = session.extract_dto();
        assert_eq!(final_dto.tick, 10);
    }
}
