//! Spatial components for astronomical bodies and surface entities.
//!
//! Provides H3 hexagonal geospatial indexing for planetary, lunar,
//! and asteroidal surfaces within the Tier 0 domain.

use bevy_ecs::prelude::*;
use h3o::CellIndex;

/// Macro-node representing an astronomical body (planet, moon, asteroid).
///
/// An astronomical body serves as the spatial parent for surface entities
/// and defines the base H3 resolution used for geographic surface clustering.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AstroNode {
    /// Unique identifier of the astronomical body (e.g., Earth = 1, Luna = 2, Mars = 3).
    pub body_id: u32,
    /// Mean radius of the astronomical body in kilometers.
    pub radius_km: u32,
    /// Base H3 simulation resolution (e.g., 3 for Earth, 2 for Mars).
    pub h3_resolution: u8,
}

impl AstroNode {
    /// Creates a new astronomical body component.
    #[must_use]
    pub const fn new(body_id: u32, radius_km: u32, h3_resolution: u8) -> Self {
        Self {
            body_id,
            radius_km,
            h3_resolution,
        }
    }
}

/// Micro-node attaching an entity to a physical H3 cell on an astronomical body.
///
/// Anchors facilities such as mines, refineries, and habitats to a discrete
/// hexagonal coordinate cell referenced to a parent astronomical body.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SurfaceNode {
    /// Entity reference to the parent astronomical body (`AstroNode`).
    pub parent_body: Entity,
    /// H3 hexagonal cell index defining the geographic position.
    pub cell: CellIndex,
}

impl SurfaceNode {
    /// Creates a new surface node attaching an entity to an H3 cell on an astronomical body.
    #[must_use]
    pub const fn new(parent_body: Entity, cell: CellIndex) -> Self {
        Self { parent_body, cell }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::network::{Converter, ResourceAmount, Storage};
    use h3o::{LatLng, Resolution};

    #[test]
    fn test_astro_node_initialization() {
        let earth = AstroNode::new(1, 6371, 3);
        assert_eq!(earth.body_id, 1);
        assert_eq!(earth.radius_km, 6371);
        assert_eq!(earth.h3_resolution, 3);
    }

    #[test]
    fn test_surface_node_attachment_and_query() {
        let mut world = World::new();

        let earth_entity = world
            .spawn(AstroNode::new(1, 6371, 3))
            .id();

        let coord = LatLng::new(0.0, 0.0).expect("valid lat lng");
        let cell = coord.to_cell(Resolution::Three);

        let facility_entity = world
            .spawn(SurfaceNode::new(earth_entity, cell))
            .id();

        let mut query = world.query::<(Entity, &SurfaceNode)>();
        let results: Vec<(Entity, SurfaceNode)> = query
            .iter(&world)
            .map(|(e, sn)| (e, *sn))
            .collect();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].0, facility_entity);
        assert_eq!(results[0].1.parent_body, earth_entity);
        assert_eq!(results[0].1.cell, cell);
    }

    #[test]
    fn test_surface_node_composition_with_storage_and_converter() {
        let mut world = World::new();

        let mars_entity = world
            .spawn(AstroNode::new(3, 3389, 2))
            .id();

        let coord = LatLng::new(15.0, -30.0).expect("valid lat lng");
        let cell = coord.to_cell(Resolution::Two);

        let storage = Storage::new(
            101,
            ResourceAmount::from_raw(500),
            ResourceAmount::from_raw(1000),
        ).expect("valid storage");

        let converter = Converter::new(42, 100, true);

        let mine_entity = world
            .spawn((
                SurfaceNode::new(mars_entity, cell),
                storage.clone(),
                converter.clone(),
            ))
            .id();

        let mut query = world.query::<(Entity, &SurfaceNode, &Storage, &Converter)>();
        let mut matched = 0;
        for (e, sn, st, cv) in query.iter(&world) {
            assert_eq!(e, mine_entity);
            assert_eq!(sn.parent_body, mars_entity);
            assert_eq!(sn.cell, cell);
            assert_eq!(st.element_id, 101);
            assert_eq!(cv.recipe_id, 42);
            matched += 1;
        }

        assert_eq!(matched, 1);
    }

    #[test]
    fn test_multiple_facilities_on_distinct_cells() {
        let mut world = World::new();

        let earth = world.spawn(AstroNode::new(1, 6371, 3)).id();

        let cell_a = LatLng::new(10.0, 10.0)
            .expect("valid coord a")
            .to_cell(Resolution::Three);
        let cell_b = LatLng::new(50.0, 50.0)
            .expect("valid coord b")
            .to_cell(Resolution::Three);

        assert_ne!(cell_a, cell_b);

        let mine_a = world.spawn(SurfaceNode::new(earth, cell_a)).id();
        let mine_b = world.spawn(SurfaceNode::new(earth, cell_b)).id();

        let mut query = world.query::<(Entity, &SurfaceNode)>();
        let mut cells_found = Vec::new();
        for (e, sn) in query.iter(&world) {
            assert_eq!(sn.parent_body, earth);
            cells_found.push((e, sn.cell));
        }

        assert_eq!(cells_found.len(), 2);
        assert!(cells_found.contains(&(mine_a, cell_a)));
        assert!(cells_found.contains(&(mine_b, cell_b)));
    }
}
