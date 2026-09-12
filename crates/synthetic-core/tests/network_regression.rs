//! Hermetic regression verification for stock-and-flow ECS network.
//!
//! Tests absolute mass conservation, latency queues, contention resolution,
//! and strict Two-Pass Cycle execution order matching SSOT-001.

use bevy_ecs::prelude::*;
use synthetic_core::network::{
    initialize_simulation_world, tick_simulation_world, Converter, ConverterBindings, CurrentTick,
    FlowEdge, FlowQueue, Recipe, RecipeIngredient, RecipeRegistry, ResourceAmount, Storage,
};

/// Helper summing all mass across all storages and in-flight edge queues in the world.
fn total_world_inventory(world: &mut World) -> i64 {
    let mut total: i64 = 0;

    let mut storage_query = world.query::<&Storage>();
    for storage in storage_query.iter(world) {
        total += storage.current_amount.to_raw();
    }

    let mut edge_query = world.query::<&FlowQueue>();
    for queue in edge_query.iter(world) {
        for packet in &queue.packets {
            total += packet.amount.to_raw();
        }
    }

    total
}

#[test]
fn test_absolute_zero_sum_conservation_across_100_ticks() {
    let mut world = initialize_simulation_world();

    // Register 1:1 conversion recipe (10 units Element 1 -> 10 units Element 2)
    let mut registry = RecipeRegistry::new();
    registry.register(Recipe {
        id: 101,
        inputs: vec![RecipeIngredient {
            element_id: 1,
            amount: ResourceAmount::from_units(10).unwrap(),
        }],
        outputs: vec![RecipeIngredient {
            element_id: 2,
            amount: ResourceAmount::from_units(10).unwrap(),
        }],
    });
    world.insert_resource(registry);

    // Setup topology:
    // Stock 1 (1000 units Element 1) -> Edge 1 (tau=3, cap=50) -> Stock 2 (input buffer)
    // -> Converter 1 -> Stock 3 (output buffer) -> Edge 2 (tau=5, cap=30) -> Stock 4 (final)
    let s1 = world
        .spawn(Storage::new(
            1,
            ResourceAmount::from_units(1_000).unwrap(),
            ResourceAmount::from_units(1_000).unwrap(),
        ).unwrap())
        .id();
    let s2 = world
        .spawn(Storage::new(
            1,
            ResourceAmount::ZERO,
            ResourceAmount::from_units(500).unwrap(),
        ).unwrap())
        .id();
    let s3 = world
        .spawn(Storage::new(
            2,
            ResourceAmount::ZERO,
            ResourceAmount::from_units(500).unwrap(),
        ).unwrap())
        .id();
    let s4 = world
        .spawn(Storage::new(
            2,
            ResourceAmount::ZERO,
            ResourceAmount::from_units(1_000).unwrap(),
        ).unwrap())
        .id();

    world.spawn((
        FlowEdge {
            source: s1,
            destination: s2,
            max_throughput: ResourceAmount::from_units(50).unwrap(),
            latency: 3,
            efficiency_millionths: 1_000_000,
        },
        FlowQueue::default(),
    ));

    world.spawn((
        Converter::new(101, 10_000, true),
        ConverterBindings::new(vec![s2], vec![s3]),
    ));

    world.spawn((
        FlowEdge {
            source: s3,
            destination: s4,
            max_throughput: ResourceAmount::from_units(30).unwrap(),
            latency: 5,
            efficiency_millionths: 1_000_000,
        },
        FlowQueue::default(),
    ));

    let initial_mass = total_world_inventory(&mut world);
    assert_eq!(initial_mass, 1_000_000_000); // 1,000 whole units in micro-units

    for tick in 1..=100 {
        tick_simulation_world(&mut world).expect("tick should succeed");
        let current_mass = total_world_inventory(&mut world);
        assert_eq!(
            current_mass, initial_mass,
            "Mass violation at tick {tick}: expected {initial_mass}, found {current_mass}"
        );
    }
}

#[test]
fn test_exact_latency_queue_delay_tau_ticks() {
    let mut world = initialize_simulation_world();

    let s1 = world
        .spawn(Storage::new(
            1,
            ResourceAmount::from_raw(100),
            ResourceAmount::from_raw(100),
        ).unwrap())
        .id();
    let s2 = world
        .spawn(Storage::new(
            1,
            ResourceAmount::ZERO,
            ResourceAmount::from_raw(100),
        ).unwrap())
        .id();

    // Edge with latency tau = 4
    let edge = world
        .spawn((
            FlowEdge {
                source: s1,
                destination: s2,
                max_throughput: ResourceAmount::from_raw(100),
                latency: 4,
                efficiency_millionths: 1_000_000,
            },
            FlowQueue::default(),
        ))
        .id();

    // Tick 1: Depart with arrival_tick = 1 + 4 = 5
    tick_simulation_world(&mut world).expect("tick 1");
    assert_eq!(world.resource::<CurrentTick>().0, 1);
    assert_eq!(world.get::<Storage>(s1).unwrap().current_amount.to_raw(), 0);
    assert_eq!(world.get::<Storage>(s2).unwrap().current_amount.to_raw(), 0);
    assert_eq!(world.get::<FlowQueue>(edge).unwrap().packets[0].arrival_tick, 5);

    // Ticks 2 through 4: In-flight, destination remains 0
    for expected_tick in 2..=4 {
        tick_simulation_world(&mut world).expect("tick in flight");
        assert_eq!(world.resource::<CurrentTick>().0, expected_tick);
        assert_eq!(world.get::<Storage>(s2).unwrap().current_amount.to_raw(), 0);
    }

    // Tick 5: Exact arrival at tick 5 (tau=4 ticks after departure at tick 1)
    tick_simulation_world(&mut world).expect("tick 5 arrival");
    assert_eq!(world.resource::<CurrentTick>().0, 5);
    assert_eq!(world.get::<Storage>(s2).unwrap().current_amount.to_raw(), 100);
}

#[test]
fn test_contention_resolution_exact_pro_rata_with_remainder() {
    let mut world = initialize_simulation_world();

    // Source has 100 micro-units
    let src = world
        .spawn(Storage::new(
            1,
            ResourceAmount::from_raw(100),
            ResourceAmount::from_raw(100),
        ).unwrap())
        .id();

    let d1 = world
        .spawn(Storage::new(
            1,
            ResourceAmount::ZERO,
            ResourceAmount::from_raw(100),
        ).unwrap())
        .id();
    let d2 = world
        .spawn(Storage::new(
            1,
            ResourceAmount::ZERO,
            ResourceAmount::from_raw(100),
        ).unwrap())
        .id();
    let d3 = world
        .spawn(Storage::new(
            1,
            ResourceAmount::ZERO,
            ResourceAmount::from_raw(100),
        ).unwrap())
        .id();

    // Demands: 80, 60, 60 -> Total 200 vs 100 available (50% scale)
    world.spawn((
        FlowEdge {
            source: src,
            destination: d1,
            max_throughput: ResourceAmount::from_raw(80),
            latency: 1,
            efficiency_millionths: 1_000_000,
        },
        FlowQueue::default(),
    ));
    world.spawn((
        FlowEdge {
            source: src,
            destination: d2,
            max_throughput: ResourceAmount::from_raw(60),
            latency: 1,
            efficiency_millionths: 1_000_000,
        },
        FlowQueue::default(),
    ));
    world.spawn((
        FlowEdge {
            source: src,
            destination: d3,
            max_throughput: ResourceAmount::from_raw(60),
            latency: 1,
            efficiency_millionths: 1_000_000,
        },
        FlowQueue::default(),
    ));

    // Tick 1: Pro-rata allocation: 40 + 30 + 30 = 100.
    tick_simulation_world(&mut world).expect("tick 1");
    assert_eq!(world.get::<Storage>(src).unwrap().current_amount.to_raw(), 0);

    // Tick 2: Delivery to destinations
    tick_simulation_world(&mut world).expect("tick 2");
    assert_eq!(world.get::<Storage>(d1).unwrap().current_amount.to_raw(), 40);
    assert_eq!(world.get::<Storage>(d2).unwrap().current_amount.to_raw(), 30);
    assert_eq!(world.get::<Storage>(d3).unwrap().current_amount.to_raw(), 30);
}

#[test]
fn test_ssot_001_phase_order_transit_arrival_precedes_converter() {
    let mut world = initialize_simulation_world();

    let mut registry = RecipeRegistry::new();
    registry.register(Recipe {
        id: 1,
        inputs: vec![RecipeIngredient {
            element_id: 1,
            amount: ResourceAmount::from_raw(50),
        }],
        outputs: vec![RecipeIngredient {
            element_id: 2,
            amount: ResourceAmount::from_raw(50),
        }],
    });
    world.insert_resource(registry);

    let src = world
        .spawn(Storage::new(
            1,
            ResourceAmount::from_raw(50),
            ResourceAmount::from_raw(100),
        ).unwrap())
        .id();
    let conv_in = world
        .spawn(Storage::new(
            1,
            ResourceAmount::ZERO,
            ResourceAmount::from_raw(100),
        ).unwrap())
        .id();
    let conv_out = world
        .spawn(Storage::new(
            2,
            ResourceAmount::ZERO,
            ResourceAmount::from_raw(100),
        ).unwrap())
        .id();

    // Edge has latency 1
    world.spawn((
        FlowEdge {
            source: src,
            destination: conv_in,
            max_throughput: ResourceAmount::from_raw(50),
            latency: 1,
            efficiency_millionths: 1_000_000,
        },
        FlowQueue::default(),
    ));

    world.spawn((
        Converter::new(1, 10_000, true),
        ConverterBindings::new(vec![conv_in], vec![conv_out]),
    ));

    // Tick 1: Dispatched into queue (arrival_tick = 2)
    tick_simulation_world(&mut world).expect("tick 1");
    assert_eq!(world.get::<Storage>(conv_in).unwrap().current_amount.to_raw(), 0);
    assert_eq!(world.get::<Storage>(conv_out).unwrap().current_amount.to_raw(), 0);

    // Tick 2: In Phase 3, packet arrives in conv_in.
    // In Phase 4 of SAME tick, converter executes and transforms conv_in into conv_out!
    tick_simulation_world(&mut world).expect("tick 2");
    assert_eq!(
        world.get::<Storage>(conv_in).unwrap().current_amount.to_raw(),
        0,
        "Input was consumed by converter in same tick after arriving"
    );
    assert_eq!(
        world.get::<Storage>(conv_out).unwrap().current_amount.to_raw(),
        50,
        "Output was produced in Phase 4 of tick 2"
    );
}

#[test]
fn test_backpressure_throttling() {
    let mut world = initialize_simulation_world();

    let src = world
        .spawn(Storage::new(
            1,
            ResourceAmount::from_raw(500),
            ResourceAmount::from_raw(500),
        ).unwrap())
        .id();
    let dest = world
        .spawn(Storage::new(
            1,
            ResourceAmount::from_raw(40),
            ResourceAmount::from_raw(50),
        ).unwrap())
        .id();

    world.spawn((
        FlowEdge {
            source: src,
            destination: dest,
            max_throughput: ResourceAmount::from_raw(100),
            latency: 1,
            efficiency_millionths: 1_000_000,
        },
        FlowQueue::default(),
    ));

    // Destination has capacity 50 and current 40, so available capacity is only 10.
    // Edge max_throughput is 100, but demand should be throttled to 10!
    tick_simulation_world(&mut world).expect("tick 1");
    assert_eq!(world.get::<Storage>(src).unwrap().current_amount.to_raw(), 490);

    // At tick 2, the 10 arrives at dest, making dest 50 (full).
    tick_simulation_world(&mut world).expect("tick 2");
    assert_eq!(world.get::<Storage>(dest).unwrap().current_amount.to_raw(), 50);

    // At tick 3, dest has 0 available space, so no more can be pulled from src.
    tick_simulation_world(&mut world).expect("tick 3");
    assert_eq!(world.get::<Storage>(src).unwrap().current_amount.to_raw(), 490);
    assert_eq!(world.get::<Storage>(dest).unwrap().current_amount.to_raw(), 50);
}
