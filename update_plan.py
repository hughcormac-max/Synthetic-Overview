with open('.agents/plans/active/PLAN-016.md', encoding='utf-8') as f:
    content = f.read()

content = content.replace('- [ ] **Step 1: SSOT Documentation Updates**', '- [x] **Step 1: SSOT Documentation Updates**')
content = content.replace('  - [ ] Modify `SSOT-PHY-001` and `SSOT-PHY-004` to eliminate the 2D coplanar constraints.', '  - [x] Modify `SSOT-PHY-001` and `SSOT-PHY-004` to eliminate the 2D coplanar constraints.')
content = content.replace('  - [ ] Document the 3D Keplerian positional formulas.', '  - [x] Document the 3D Keplerian positional formulas.')

content = content.replace('- [ ] **Step 2: Core Domain Logic & Data Types**', '- [x] **Step 2: Core Domain Logic & Data Types**')
content = content.replace('  - [ ] Update `AstroNodeConfig` in `crates/synthetic-core/src/astronomy/models.rs`.', '  - [x] Update `AstroNodeConfig` in `crates/synthetic-core/src/astronomy/models.rs`.')
content = content.replace('  - [ ] Update `calculate_kepler_position` in `crates/synthetic-core/src/astronomy/kinematics.rs` to apply 3D transformation matrices.', '  - [x] Update `calculate_kepler_position` in `crates/synthetic-core/src/astronomy/kinematics.rs` to apply 3D transformation matrices.')
content = content.replace('  - [ ] Update kinematic unit tests to assert 3D positional calculations and adapt existing 2D tests.', '  - [x] Update kinematic unit tests to assert 3D positional calculations and adapt existing 2D tests.')

content = content.replace('- [ ] **Step 3: Presentation Integration**', '- [x] **Step 3: Presentation Integration**')
content = content.replace('  - [ ] Update `create_orbit_curve_mesh` and its usages in `crates/synthetic-client/src/systems/astronomy.rs`.', '  - [x] Update `create_orbit_curve_mesh` and its usages in `crates/synthetic-client/src/systems/astronomy.rs`.')
content = content.replace('  - [ ] Update rendering tests.', '  - [x] Update rendering tests.')

content = content.replace('- [ ] **Step 4: Dataset Migration**', '- [x] **Step 4: Dataset Migration**')
content = content.replace('  - [ ] Migrate `assets/data/solar_system.json` to replace `longitude_of_periapsis_rad` with `argument_of_periapsis_rad` and introduce `inclination_rad: 0.0` and `longitude_of_ascending_node_rad: 0.0`.', '  - [x] Migrate `assets/data/solar_system.json` to replace `longitude_of_periapsis_rad` with `argument_of_periapsis_rad` and introduce `inclination_rad: 0.0` and `longitude_of_ascending_node_rad: 0.0`.')

content = content.replace('- [ ] **Step 5: Full Regression & Verification**', '- [x] **Step 5: Full Regression & Verification**')
content = content.replace('  - [ ] Execute project typecheck (`cargo check`).', '  - [x] Execute project typecheck (`cargo check`).')
content = content.replace('  - [ ] Execute full unit test suite (`cargo test`).', '  - [x] Execute full unit test suite (`cargo test`).')

with open('.agents/plans/active/PLAN-016.md', 'w', encoding='utf-8') as f:
    f.write(content)
