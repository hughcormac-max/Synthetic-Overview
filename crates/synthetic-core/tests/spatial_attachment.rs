//! Hermetic integration test for H3 spatial grid attachment.
//!
//! Verifies that entities representing mines, refineries, and stations can be
//! attached to H3 hexagonal cells on astronomical bodies, and queried alongside
//! network storage and converter components.

use bevy_ecs::prelude::*;
use h3o::{LatLng, Resolution};
use synthetic_core::network::{Converter, ResourceAmount, Storage};
use synthetic_core::spatial::{AstroNode, SurfaceNode};

#[test]
fn test_hermetic_entity_attachment_to_h3_cells() {
    let mut world = World::new();

    // Spawn astronomical bodies
    let earth = world.spawn(AstroNode::new(1, 6371, 3)).id();
    let luna = world.spawn(AstroNode::new(2, 1737, 3)).id();
    let mars = world.spawn(AstroNode::new(3, 3389, 2)).id();

    // London coordinates for Earth facility
    let coord_london = LatLng::new(51.5074, -0.1278).expect("valid coordinates");
    let cell_london = coord_london.to_cell(Resolution::Three);

    // Tokyo coordinates for Earth facility
    let coord_tokyo = LatLng::new(35.6762, 139.6503).expect("valid coordinates");
    let cell_tokyo = coord_tokyo.to_cell(Resolution::Three);

    // Lunar South Pole coordinates
    let coord_shackleton = LatLng::new(-89.9, 0.0).expect("valid coordinates");
    let cell_shackleton = coord_shackleton.to_cell(Resolution::Three);

    // Mars Olympus Mons coordinates
    let coord_olympus = LatLng::new(18.65, -133.8).expect("valid coordinates");
    let cell_olympus = coord_olympus.to_cell(Resolution::Two);

    // Attach facilities
    let facility_london = world.spawn(SurfaceNode::new(earth, cell_london)).id();
    let facility_tokyo = world.spawn(SurfaceNode::new(earth, cell_tokyo)).id();
    let outpost_luna = world.spawn(SurfaceNode::new(luna, cell_shackleton)).id();
    let base_mars = world.spawn(SurfaceNode::new(mars, cell_olympus)).id();

    // Verify all four entities are attached properly
    let mut query = world.query::<(Entity, &SurfaceNode)>();
    let mut nodes: Vec<(Entity, SurfaceNode)> = query
        .iter(&world)
        .map(|(e, sn)| (e, *sn))
        .collect();

    assert_eq!(nodes.len(), 4);

    nodes.sort_by_key(|(e, _)| e.index());
    assert!(nodes.iter().any(|(e, sn)| *e == facility_london && sn.parent_body == earth && sn.cell == cell_london));
    assert!(nodes.iter().any(|(e, sn)| *e == facility_tokyo && sn.parent_body == earth && sn.cell == cell_tokyo));
    assert!(nodes.iter().any(|(e, sn)| *e == outpost_luna && sn.parent_body == luna && sn.cell == cell_shackleton));
    assert!(nodes.iter().any(|(e, sn)| *e == base_mars && sn.parent_body == mars && sn.cell == cell_olympus));
}

#[test]
fn test_spatial_composition_with_storage_and_converter() {
    let mut world = World::new();

    let mars = world.spawn(AstroNode::new(3, 3389, 2)).id();

    let coord = LatLng::new(18.65, -133.8).expect("valid coordinates");
    let cell = coord.to_cell(Resolution::Two);

    let storage = Storage::new(
        10, // Iron ore
        ResourceAmount::from_units(500).unwrap(),
        ResourceAmount::from_units(2000).unwrap(),
    ).expect("valid storage");

    let converter = Converter::new(1, 100, true);

    let refinery = world.spawn((
        SurfaceNode::new(mars, cell),
        storage,
        converter,
    )).id();

    // Query composite entity with spatial, storage, and converter components
    let mut composite_query = world.query::<(Entity, &SurfaceNode, &mut Storage, &Converter)>();
    let mut count = 0;

    for (e, sn, mut st, cv) in composite_query.iter_mut(&mut world) {
        assert_eq!(e, refinery);
        assert_eq!(sn.parent_body, mars);
        assert_eq!(sn.cell, cell);
        assert_eq!(cv.recipe_id, 1);
        assert!(cv.operational_status);

        // Perform storage mutation while attached to surface node
        st.deposit(ResourceAmount::from_units(100).unwrap()).expect("deposit succeeded");
        assert_eq!(st.current_amount, ResourceAmount::from_units(600).unwrap());

        count += 1;
    }

    assert_eq!(count, 1);
}

#[test]
fn test_query_filtering_by_parent_astronomical_body() {
    let mut world = World::new();

    let earth = world.spawn(AstroNode::new(1, 6371, 3)).id();
    let mars = world.spawn(AstroNode::new(3, 3389, 2)).id();

    let cell_earth_1 = LatLng::new(10.0, 10.0).expect("coord").to_cell(Resolution::Three);
    let cell_earth_2 = LatLng::new(20.0, 20.0).expect("coord").to_cell(Resolution::Three);
    let cell_mars_1 = LatLng::new(0.0, 0.0).expect("coord").to_cell(Resolution::Two);

    let e1 = world.spawn(SurfaceNode::new(earth, cell_earth_1)).id();
    let e2 = world.spawn(SurfaceNode::new(earth, cell_earth_2)).id();
    let m1 = world.spawn(SurfaceNode::new(mars, cell_mars_1)).id();

    let mut query = world.query::<(Entity, &SurfaceNode)>();

    let earth_entities: Vec<Entity> = query
        .iter(&world)
        .filter(|(_, sn)| sn.parent_body == earth)
        .map(|(e, _)| e)
        .collect();

    assert_eq!(earth_entities.len(), 2);
    assert!(earth_entities.contains(&e1));
    assert!(earth_entities.contains(&e2));
    assert!(!earth_entities.contains(&m1));
}
