import re

with open('crates/synthetic-client/src/systems/astronomy.rs', encoding='utf-8') as f:
    content = f.read()

rep_old_fn = '''pub fn create_orbit_curve_mesh(
    semi_major_axis_m: f64,
    eccentricity: f64,
    longitude_of_periapsis_rad: f64,
) -> Mesh {
    let two_pi = std::f64::consts::TAU;
    let step_rad = two_pi / (ORBIT_SEGMENTS as f64);
    let e = eccentricity.clamp(0.0, 0.999_999);
    let a = semi_major_axis_m;
    let varpi = longitude_of_periapsis_rad;

    let mut positions = Vec::with_capacity(ORBIT_SEGMENTS + 1);
    let mut normals = Vec::with_capacity(ORBIT_SEGMENTS + 1);

    for i in 0..=ORBIT_SEGMENTS {
        let nu = (i % ORBIT_SEGMENTS) as f64 * step_rad;
        let r = a * (1.0 - e * e) / (1.0 + e * nu.cos());
        let angle = nu + varpi;
        let x = (r * angle.cos()) as f32;
        let y = (r * angle.sin()) as f32;
        let z = 0.0_f32;'''

rep_new_fn = '''pub fn create_orbit_curve_mesh(
    semi_major_axis_m: f64,
    eccentricity: f64,
    inclination_rad: f64,
    longitude_of_ascending_node_rad: f64,
    argument_of_periapsis_rad: f64,
) -> Mesh {
    let two_pi = std::f64::consts::TAU;
    let step_rad = two_pi / (ORBIT_SEGMENTS as f64);
    let e = eccentricity.clamp(0.0, 0.999_999);
    let a = semi_major_axis_m;
    let i_rad = inclination_rad;
    let omega_upper = longitude_of_ascending_node_rad;
    let omega_lower = argument_of_periapsis_rad;

    let cos_omega_upper = omega_upper.cos();
    let sin_omega_upper = omega_upper.sin();
    let cos_i = i_rad.cos();
    let sin_i = i_rad.sin();

    let mut positions = Vec::with_capacity(ORBIT_SEGMENTS + 1);
    let mut normals = Vec::with_capacity(ORBIT_SEGMENTS + 1);

    for i in 0..=ORBIT_SEGMENTS {
        let nu = (i % ORBIT_SEGMENTS) as f64 * step_rad;
        let r = a * (1.0 - e * e) / (1.0 + e * nu.cos());
        
        let cos_omega_nu = (omega_lower + nu).cos();
        let sin_omega_nu = (omega_lower + nu).sin();
        
        let x = (r * (cos_omega_upper * cos_omega_nu - sin_omega_upper * sin_omega_nu * cos_i)) as f32;
        let y = (r * (sin_omega_upper * cos_omega_nu + cos_omega_upper * sin_omega_nu * cos_i)) as f32;
        let z = (r * (sin_i * sin_omega_nu)) as f32;'''

content = content.replace(rep_old_fn, rep_new_fn)

rep_old_call = '''                    let orbit_mesh = meshes.add(create_orbit_curve_mesh(
                        config.semi_major_axis_m,
                        config.eccentricity,
                        config.longitude_of_periapsis_rad,
                    ));'''

rep_new_call = '''                    let orbit_mesh = meshes.add(create_orbit_curve_mesh(
                        config.semi_major_axis_m,
                        config.eccentricity,
                        config.inclination_rad,
                        config.longitude_of_ascending_node_rad,
                        config.argument_of_periapsis_rad,
                    ));'''

content = content.replace(rep_old_call, rep_new_call)

# Update config creations in tests
rep_test_conf = '''eccentricity: 0.0,
                true_anomaly_epoch_rad: 0.0,
                longitude_of_periapsis_rad: 0.0,'''
rep_test_conf_new = '''eccentricity: 0.0,
                inclination_rad: 0.0,
                longitude_of_ascending_node_rad: 0.0,
                argument_of_periapsis_rad: 0.0,
                true_anomaly_epoch_rad: 0.0,'''
content = content.replace(rep_test_conf, rep_test_conf_new)

# Update create_orbit_curve_mesh test call
rep_old_test_call = '''        let a = 1.0e11;
        let e = 0.1;
        let varpi = 0.0;
        let mesh = create_orbit_curve_mesh(a, e, varpi);'''
rep_new_test_call = '''        let a = 1.0e11;
        let e = 0.1;
        let mesh = create_orbit_curve_mesh(a, e, 0.0, 0.0, 0.0);'''
content = content.replace(rep_old_test_call, rep_new_test_call)

with open('crates/synthetic-client/src/systems/astronomy.rs', 'w', encoding='utf-8') as f:
    f.write(content)
