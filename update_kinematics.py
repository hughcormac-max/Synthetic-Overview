import re

with open('crates/synthetic-core/src/astronomy/kinematics.rs', encoding='utf-8') as f:
    content = f.read()

# Update 2D references
content = content.replace('//! 2D Keplerian orbital kinematics and ephemeris calculations.', '//! 3D Keplerian orbital kinematics and ephemeris calculations.')
content = content.replace('/// While orbital trajectories are constrained to the ecliptic plane (z = 0.0),\n/// the state representation supports 3D coordinates for surface nodes and spatial queries.', '/// State representation supports full 3D coordinates.')

rep_old = '''    let radius = config.semi_major_axis_m * (1.0 - e * eccentric_anomaly.cos());
    let angle = true_anomaly + config.longitude_of_periapsis_rad;

    let x_rel = radius * angle.cos();
    let y_rel = radius * angle.sin();

    GlobalPosition {
        x: parent_pos.x + x_rel,
        y: parent_pos.y + y_rel,
        z: parent_pos.z,
    }'''

rep_new = '''    let radius = config.semi_major_axis_m * (1.0 - e * eccentric_anomaly.cos());
    
    let i = config.inclination_rad;
    let omega_upper = config.longitude_of_ascending_node_rad;
    let omega_lower = config.argument_of_periapsis_rad;

    let cos_omega_upper = omega_upper.cos();
    let sin_omega_upper = omega_upper.sin();
    let cos_omega_nu = (omega_lower + true_anomaly).cos();
    let sin_omega_nu = (omega_lower + true_anomaly).sin();
    let cos_i = i.cos();
    let sin_i = i.sin();

    let x_rel = radius * (cos_omega_upper * cos_omega_nu - sin_omega_upper * sin_omega_nu * cos_i);
    let y_rel = radius * (sin_omega_upper * cos_omega_nu + cos_omega_upper * sin_omega_nu * cos_i);
    let z_rel = radius * (sin_i * sin_omega_nu);

    GlobalPosition {
        x: parent_pos.x + x_rel,
        y: parent_pos.y + y_rel,
        z: parent_pos.z + z_rel,
    }'''

content = content.replace(rep_old, rep_new)

# Update tests
content = content.replace('longitude_of_periapsis_rad: 0.0,', 'inclination_rad: 0.0,\n            longitude_of_ascending_node_rad: 0.0,\n            argument_of_periapsis_rad: 0.0,')

# Add a 3D test at the very end of the file
test_3d = '''
    #[test]
    fn test_3d_inclined_orbit() {
        let inclined_config = AstroNodeConfig {
            name: "Inclined".into(),
            parent_name: Some("Sun".into()),
            mass_kg: 1e20,
            radius_m: 1e5,
            semi_major_axis_m: 1e10,
            eccentricity: 0.0,
            inclination_rad: std::f64::consts::PI / 2.0,
            longitude_of_ascending_node_rad: 0.0,
            argument_of_periapsis_rad: 0.0,
            true_anomaly_epoch_rad: std::f64::consts::PI / 2.0,
            mean_motion_rad_s: 1.0,
        };

        let pos = calculate_epoch_position(&inclined_config, GlobalPosition::ZERO);
        assert!(pos.x.abs() < 1.0);
        assert!(pos.y.abs() < 1.0);
        assert!((pos.z - 1e10).abs() < 1.0);
    }
}
'''
# Using regex to replace the final '}'
content = re.sub(r'}\s*$', test_3d, content)

with open('crates/synthetic-core/src/astronomy/kinematics.rs', 'w', encoding='utf-8') as f:
    f.write(content)
