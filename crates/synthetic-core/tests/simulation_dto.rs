//! Hermetic integration tests for `SimulationStateDto` and `AstroNode`/`SurfaceNode` serialization.

use synthetic_core::network::{
    initialize_simulation_world, Converter, FlowEdge, FlowQueue, ResourceAmount, Storage,
    TransitPacket,
};
use synthetic_core::simulation::ResourceDirectory;
use synthetic_core::{
    AstroNode, ResourceDto, SimulationSession, SimulationStateDto, SurfaceNode,
};

#[test]
fn test_dto_serialization_from_world() {
    let mut world = initialize_simulation_world();
    world.insert_resource(ResourceDirectory::new(vec![
        ResourceDto {
            id: 1,
            name: "Iron Ore".to_string(),
        },
        ResourceDto {
            id: 2,
            name: "Steel Plate".to_string(),
        },
    ]));

    let earth = world.spawn(AstroNode::new(1, 6371, 3)).id();
    let coord = h3o::LatLng::new(51.5074, -0.1278).expect("valid coord");
    let cell = coord.to_cell(h3o::Resolution::Three);
    let facility = world.spawn(SurfaceNode::new(earth, cell)).id();

    let storage_a = world
        .spawn(
            Storage::new(
                1,
                ResourceAmount::from_raw(500),
                ResourceAmount::from_raw(1000),
            )
            .expect("valid storage"),
        )
        .id();

    let storage_b = world
        .spawn(
            Storage::new(
                2,
                ResourceAmount::from_raw(100),
                ResourceAmount::from_raw(2000),
            )
            .expect("valid storage"),
        )
        .id();

    let converter = world.spawn(Converter::new(10, 10_000, true)).id();

    let edge = world
        .spawn((
            FlowEdge {
                source: storage_a,
                destination: storage_b,
                max_throughput: ResourceAmount::from_raw(100),
                latency: 2,
                efficiency_millionths: 1_000_000,
            },
            FlowQueue {
                packets: vec![
                    TransitPacket {
                        element_id: 1,
                        amount: ResourceAmount::from_raw(50),
                        arrival_tick: 5,
                        destination: storage_b,
                    },
                    TransitPacket {
                        element_id: 1,
                        amount: ResourceAmount::from_raw(25),
                        arrival_tick: 6,
                        destination: storage_b,
                    },
                ],
            },
        ))
        .id();

    let dto = SimulationStateDto::from_world(&mut world);

    assert_eq!(dto.resources.len(), 2);
    assert_eq!(dto.resources[0].name, "Iron Ore");
    assert_eq!(dto.resources[1].name, "Steel Plate");

    assert_eq!(dto.storages.len(), 2);
    assert_eq!(dto.storages[0].entity_id, storage_a.index());
    assert_eq!(dto.storages[0].amount, 500);
    assert_eq!(dto.storages[0].capacity, 1000);

    assert_eq!(dto.converters.len(), 1);
    assert_eq!(dto.converters[0].entity_id, converter.index());
    assert_eq!(dto.converters[0].recipe_id, 10);

    assert_eq!(dto.edges.len(), 1);
    assert_eq!(dto.edges[0].edge_id, edge.index());
    assert_eq!(dto.edges[0].source_id, storage_a.index());
    assert_eq!(dto.edges[0].destination_id, storage_b.index());
    assert_eq!(dto.edges[0].in_transit, 75);

    assert_eq!(dto.astro_nodes.len(), 1);
    assert_eq!(dto.astro_nodes[0].body_id, 1);
    assert_eq!(dto.astro_nodes[0].radius_km, 6371);
    assert_eq!(dto.astro_nodes[0].h3_resolution, 3);

    assert_eq!(dto.surface_nodes.len(), 1);
    assert_eq!(dto.surface_nodes[0].entity_id, facility.index());
    assert_eq!(dto.surface_nodes[0].parent_body_id, 1);
    assert_eq!(dto.surface_nodes[0].h3_cell_index, cell.to_string());
}

#[test]
fn test_default_simulation_session_has_astro_and_surface_nodes() {
    let mut session = SimulationSession::new();
    let dto = session.extract_dto();
    assert_eq!(dto.astro_nodes.len(), 2);
    assert_eq!(dto.surface_nodes.len(), 3);
}
