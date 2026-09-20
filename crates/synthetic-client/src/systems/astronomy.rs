//! Solar system configuration loading and Bevy ECS integration.
//!
//! Grounded in SSOT-PHY-004 and PLAN-012.

use bevy::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};
use synthetic_core::astronomy::{generate_fibonacci_nodes_by_density, AstroNodeConfig, SurfaceNode};

#[derive(Component, Debug, Clone)]
#[allow(dead_code)]
pub struct CelestialBody {
    pub config: AstroNodeConfig,
}

#[derive(Component, Debug, Clone)]
#[allow(dead_code)]
pub struct SurfaceNodesComponent {
    pub nodes: Vec<SurfaceNode>,
}

pub struct AstronomyPlugin;

impl Plugin for AstronomyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_solar_system);
    }
}

/// Resolves candidate paths for locating the solar system JSON dataset.
fn resolve_config_path(primary_path: &Path) -> Result<PathBuf, String> {
    if primary_path.exists() {
        return Ok(primary_path.to_path_buf());
    }

    let fallback_candidates = [
        PathBuf::from("assets/data/solar_system.json"),
        PathBuf::from("crates/synthetic-client/assets/data/solar_system.json"),
        PathBuf::from("../crates/synthetic-client/assets/data/solar_system.json"),
        PathBuf::from("../../assets/data/solar_system.json"),
    ];

    for candidate in &fallback_candidates {
        if candidate.exists() {
            return Ok(candidate.clone());
        }
    }

    Err(format!(
        "Unable to locate solar system configuration file at {} or fallback paths",
        primary_path.display()
    ))
}

/// Loads and deserializes solar system celestial configuration from JSON.
///
/// # Errors
///
/// Returns an error if the file cannot be found, read, or parsed.
pub fn load_solar_system_config(
    path: &Path,
) -> Result<Vec<AstroNodeConfig>, Box<dyn std::error::Error>> {
    let resolved_path = resolve_config_path(path)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::NotFound, e))?;
    let content = fs::read_to_string(resolved_path)?;
    let configs: Vec<AstroNodeConfig> = serde_json::from_str(&content)?;
    Ok(configs)
}

/// Default surface node density in inverse meters: 1 node per kilometer of radius (`1e-3 m^-1`).
pub const KM_SURFACE_NODE_DENSITY: f64 = 1e-3;

/// Bevy startup system that populates the ECS with celestial bodies and generated surface nodes.
pub fn setup_solar_system(mut commands: Commands) {
    let config_path = Path::new("assets/data/solar_system.json");
    match load_solar_system_config(config_path) {
        Ok(configs) => {
            info!("Spawning {} celestial bodies into ECS", configs.len());
            for config in configs {
                let nodes = generate_fibonacci_nodes_by_density(config.radius_m, KM_SURFACE_NODE_DENSITY);
                commands.spawn((
                    CelestialBody {
                        config: config.clone(),
                    },
                    SurfaceNodesComponent { nodes },
                ));
            }
        }
        Err(err) => {
            error!("Failed to load solar system configuration: {}", err);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_solar_system_config() {
        let test_path = Path::new("assets/data/solar_system.json");
        let result = load_solar_system_config(test_path);
        assert!(
            result.is_ok(),
            "Expected successful config load, got error: {:?}",
            result.err()
        );

        let configs = result.unwrap();
        assert!(!configs.is_empty(), "Solar system config must not be empty");

        let find_body = |name: &str| configs.iter().find(|b| b.name == name);

        let sun = find_body("Sun").expect("Sun must be present");
        assert!(sun.parent_name.is_none(), "Sun must have no parent");
        assert!(sun.mass_kg > 1.9e30);

        let earth = find_body("Earth").expect("Earth must be present");
        assert_eq!(earth.parent_name.as_deref(), Some("Sun"));

        let luna = find_body("Luna").expect("Luna must be present");
        assert_eq!(luna.parent_name.as_deref(), Some("Earth"));

        let jupiter = find_body("Jupiter").expect("Jupiter must be present");
        assert_eq!(jupiter.parent_name.as_deref(), Some("Sun"));

        let titan = find_body("Titan").expect("Titan must be present");
        assert_eq!(titan.parent_name.as_deref(), Some("Saturn"));

        let europa = find_body("Europa").expect("Europa must be present");
        assert_eq!(europa.parent_name.as_deref(), Some("Jupiter"));

        let triton = find_body("Triton").expect("Triton must be present");
        assert_eq!(triton.parent_name.as_deref(), Some("Neptune"));
    }

    #[test]
    fn test_km_surface_node_generation() {
        let earth_radius = 6_371_000.0;
        let earth_nodes =
            generate_fibonacci_nodes_by_density(earth_radius, KM_SURFACE_NODE_DENSITY);
        assert_eq!(earth_nodes.len(), 6_371);

        let jupiter_radius = 69_911_000.0;
        let jupiter_nodes =
            generate_fibonacci_nodes_by_density(jupiter_radius, KM_SURFACE_NODE_DENSITY);
        assert_eq!(jupiter_nodes.len(), 69_911);
    }
}
