---
id: PLAN-020
title: "Simulation Time Warp Controls & Keyboard Shortcut Help Popup"
status: completed
author: "Antigravity"
created: 2026-09-29
updated: 2026-09-29
completed_at: 2026-09-29
branch: "main"
---

# PLAN-020: Simulation Time Warp Controls & Keyboard Shortcut Help Popup

> **Status:** `completed` | **Created:** 2026-09-29 | **Last Updated:** 2026-09-29
> **Author:** Antigravity | **Branch:** main
> **Permanent Location:** `.agents/docs/plans/PLAN-020.md`

---

# 🧑‍💻 PART 1: HUMAN PROBLEM ALIGNMENT
*(Authored via `/plan-human` — Aligns user-facing intent, boundaries, and acceptance criteria prior to technical design)*

## 🎯 1. Problem Statement & Context
Currently, the solar system simulation runs at a fixed 1:1 real-time rate (`1 sec/sec`), advancing `sim_time.elapsed_seconds` by real-world frame delta times. At this rate, observing planetary orbital motion is impossible without waiting real days, months, or years (e.g. Earth takes 365.25 days to complete one revolution, and Neptune takes 165 years).

To inspect orbital mechanics, verify satellite trajectories, and observe multi-body alignments, the user requires interactive time warp controls spanning multiple orders of magnitude (from pause `0x` up to `1 month/sec`). Furthermore, as the application introduces keyboard shortcuts for time warp and existing camera/selection features without persistent screen clutter, users need an on-demand keyboard shortcut help overlay (toggled with `H`) to discover and reference controls without obscuring the 3D viewport during normal simulation.

## 🚀 2. Core Objectives & Capabilities
- **Discrete Time Warp Ladder:** Provide a calibrated speed scale:
  - `0x`: Pause / Frozen time
  - `1x`: Real-time (`1 sec / sec`)
  - `60x`: Minute scale (`1 min / sec`)
  - `3,600x`: Hour scale (`1 hour / sec`)
  - `86,400x`: Day scale (`1 day / sec`)
  - `2,592,000x`: Month scale (`1 month / sec`, assuming 30 days)
- **Keyboard-Driven Warp Controls (Headless UX):**
  - `Space`: Instantaneous Pause / Resume toggle (restoring the previous non-zero warp speed).
  - Number keys `1` through `5`: Direct jump to presets (`1x`, `60x`, `3600x`, `86400x`, `2592000x`), and `0` for Pause.
  - Bracket keys `[` and `]`: Step down / step up through the warp ladder.
- **On-Demand Keyboard Shortcut Help Popup:**
  - Pressing `H` toggles a clean, semi-transparent HUD modal displaying all keybindings (Time warp, Camera navigation, Body selection).
  - Pressing `Escape` or pressing `H` again closes the modal immediately.

## 🚫 3. Non-Goals & Exclusions
- **Persistent Time Scrubber / Timeline Slider UI:** Complex timeline scrubbers, scrubber bars, or date-picker widgets are excluded from this plan and deferred to a dedicated HUD plan.
- **Negative / Reverse Time Warp:** Rewinding time is excluded from this plan (time only advances forward or pauses).
- **Relativistic or Dynamic Dot Rescaling:** Orbital dot sampling stays on the existing 401-dot local-space pipeline defined in PLAN-019 without changing sampling intervals during this phase.
- **International Keyboard Remapping:** Bracket keys `[` and `]` are used directly for ladder stepping; alternative layout keymaps (e.g. AZERTY/QWERTZ) are deferred.

## 👤 4. User Stories & Interaction Journeys
- **Story 1 (Fast-Forwarding Orbits):**
  - **As a** solar system observer,
  - **I want to** press `4` or step up with `]` to set simulation speed to `1 day/sec`,
  - **So that** I can watch the planets and Moon physically orbit their parents, with the Moon orbiting Earth in ~27 seconds and Earth orbiting the Sun in ~6 minutes.
- **Story 2 (Pausing for Close Inspection):**
  - **As an** observer tracking a fast-moving body,
  - **I want to** press `Space` to freeze the simulation,
  - **So that** I can inspect surface nodes and camera alignments without the body moving away.
- **Story 3 (Discovering Keybindings):**
  - **As a** new user,
  - **I want to** press `H` at any time,
  - **So that** a help modal appears detailing time warp keys, camera orbit/pan/zoom controls, and selection shortcuts.

*Key interaction walkthrough:*
1. User launches simulation running at `1 sec/sec`.
2. User presses `H`: an overlay displays keyboard shortcuts.
3. User presses `Escape` or `H`: help modal disappears.
4. User presses `Space`: simulation freezes (`0x`). Bodies, orbit dots, and camera hold steady.
5. User presses `Space` again: simulation resumes at previous warp (`1x`).
6. User presses `3`: simulation accelerates to `1 hour/sec` (`3,600x`). Celestial bodies noticeably move along their orbits.

## ✅ 5. Acceptance Criteria
- [x] Simulation time advances according to the active warp multiplier (`sim_time.elapsed_seconds += delta * warp`).
- [x] Discrete warp levels match exact values: `0x`, `1x`, `60x`, `3600x`, `86400x`, `2592000x`.
- [x] Pressing `Space` toggles between `0x` (paused) and the previous active warp multiplier.
- [x] Pressing keys `0` through `5` (and Numpad `0` through `5`) immediately sets the warp multiplier to the corresponding preset without triggering celestial body selection.
- [x] Pressing `[` steps down one speed level (clamped at `0x`), and `]` steps up one level (clamped at `2,592,000x`).
- [x] Celestial body global positions dynamically propagate along their Keplerian orbits every frame as `sim_time` advances, keeping camera focus locked to moving bodies.
- [x] Strict ECS ordering ensures `keyboard_time_warp_system` runs before `advance_simulation_time`, eliminating pause lag.
- [x] Pressing `H` toggles a visible help overlay modal explaining all keybindings.
- [x] Pressing `Escape` while the help modal is open dismisses the modal.
- [x] Existing camera navigation (LMB orbit, RMB pan, scroll zoom) and mouse picking remain functional without key conflicts.

## ⚠️ 6. Edge Cases & Boundary Behaviors
- **Boundary Clamping:** Stepping down past `0x` with `[` clamps at `0x`. Stepping up past `2,592,000x` with `]` clamps at `2,592,000x`.
- **Pause Memory Invariant:** In `TimeWarp`, `previous_non_zero` maintains the strict invariant `previous_non_zero != WarpLevel::Paused`. Pressing `0` or stepping down to `Paused` repeatedly never overwrites the stored non-zero resume speed.
- **Keycode Deconfliction:** `Digit1` through `Digit9` are removed from camera body selection (which retains `Tab`/`Shift-Tab`, arrow keys, mouse picking, and the UI selector list), preventing camera retargeting when warping.
- **UI Camera Startup Order:** `spawn_keybinding_help_modal` explicitly runs `.after(setup_ui)` to ensure `UiCameraMarker` exists when `TargetCamera` is assigned.
- **Help Modal Window Resizing:** The help modal uses centered flexbox layout with auto-wrapping to remain readable and properly aligned across different window resolutions.

---

# 🤖 PART 2: ROBOT TECHNICAL SPECIFICATIONS
*(Authored via `/plan-robot` — Defines contracts, interfaces, and <= ~50 line atomic implementation tasks)*

## 📐 7. Authoritative Domain References (Tier 0)
*Downlink to immutable domain specifications, formulas, constants, and truth tables in `.agents/docs/ssot/`:*
- [SSOT-SYS-000: Universal SI Units & Dimensional Metrology](../../ssot/SSOT-SYS-000-SI-Units.md) — Base SI unit of time is second (`s`). Discrete simulation time scaling `rate_per_tick = rate_per_second * 60`.
- [SSOT-SYS-001: Discrete Stock-and-Flow Simulation Kernel](../../ssot/SSOT-SYS-001-Tick-Kernel.md) — Chronological clock incrementation from epoch, regulated by discrete speed control multipliers.
- [SSOT-PHY-001: Astrodynamics](../../ssot/SSOT-PHY-001-Astrodynamics.md) — Keplerian orbit calculations for celestial body propagation over time.
- [SSOT-UIX-001: Presentation](../../ssot/SSOT-UIX-001-Presentation.md) — UI styling tokens, dark palette, translucent backdrops, cyan border highlights, non-intrusive modal overlays.

## 🧩 8. Domain Types & ECS Components
```rust
use bevy::prelude::*;

/// Discrete speed presets for simulation time acceleration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum WarpLevel {
    Paused,        // 0x (Frozen time)
    #[default]
    RealTime,      // 1x (1 second / second)
    MinutePerSec,  // 60x (1 minute / second)
    HourPerSec,    // 3,600x (1 hour / second)
    DayPerSec,     // 86,400x (1 day / second)
    MonthPerSec,   // 2,592,000x (1 month / second, 30 days)
}

/// Simulation time warp manager resource tracking current speed and last non-zero preset.
#[derive(Resource, Debug, Clone, PartialEq)]
pub struct TimeWarp {
    pub current: WarpLevel,
    pub previous_non_zero: WarpLevel,
}

/// Marker component attached to the root entity of the keybinding help modal overlay.
#[derive(Component, Debug, Clone)]
pub struct KeybindingHelpModal;
```

## 🔌 9. Public API & System Signatures
```rust
impl WarpLevel {
    pub const ALL: [WarpLevel; 6] = [
        WarpLevel::Paused,
        WarpLevel::RealTime,
        WarpLevel::MinutePerSec,
        WarpLevel::HourPerSec,
        WarpLevel::DayPerSec,
        WarpLevel::MonthPerSec,
    ];

    #[must_use]
    pub const fn multiplier(self) -> f64;

    #[must_use]
    pub const fn label(self) -> &'static str;

    #[must_use]
    pub fn step_up(self) -> Self;

    #[must_use]
    pub fn step_down(self) -> Self;
}

impl TimeWarp {
    pub fn toggle_pause(&mut self);
    pub fn set_level(&mut self, level: WarpLevel);
    pub fn step_up(&mut self);
    pub fn step_down(&mut self);
    #[must_use]
    pub const fn current_multiplier(&self) -> f64;
}

/// Advances simulation time scaled by the active TimeWarp multiplier.
pub fn advance_simulation_time(
    time: Res<Time>,
    time_warp: Res<TimeWarp>,
    mut sim_time: ResMut<SimulationTime>,
);

/// Propagates all celestial body positions and floating origin focus along orbits at t_sim.
pub fn update_celestial_positions_system(
    sim_time: Res<SimulationTime>,
    mut origin: ResMut<FloatingOrigin>,
    mut body_query: Query<(&mut CelestialBody, &mut Transform)>,
);

/// Maps keyboard inputs (Space, 0-5, [, ]) to TimeWarp state changes.
pub fn keyboard_time_warp_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut time_warp: ResMut<TimeWarp>,
);

/// Toggles keybinding help modal visibility on H and dismisses on Escape.
pub fn toggle_keybinding_help_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Visibility, With<KeybindingHelpModal>>,
);

/// Spawns the initial hidden keybinding help modal overlay node tree.
pub fn spawn_keybinding_help_modal(
    commands: Commands,
    ui_camera_query: Query<Entity, With<crate::UiCameraMarker>>,
);
```

## 🗺️ 10. File-by-File Module Mapping
- **Domain & Application Layer:**
  - `crates/synthetic-client/src/systems/time_warp.rs` (new module: `WarpLevel`, `TimeWarp`, `keyboard_time_warp_system`, pure tests).
- **Presentation & UI Layer:**
  - `crates/synthetic-client/src/systems/ui.rs` (`KeybindingHelpModal`, `spawn_keybinding_row`, `spawn_keybinding_help_modal`, `toggle_keybinding_help_system`).
  - `crates/synthetic-client/src/systems/astronomy.rs` (deconflict `Digit1`..`Digit9`, integrate `time_warp` in `advance_simulation_time`, implement `update_celestial_positions_system`).
  - `crates/synthetic-client/src/systems/mod.rs` (export `time_warp` module and `TimeWarpPlugin`).

## 🛠️ 11. Atomic Implementation Steps (STRICT <= ~50 LINES RULE)
*Ordered checklist broken down into atomic, testable steps. Any step involving more than ~50 lines of code changes is strictly split.*

- [x] **Step 1: Domain Types & Pure Logic in `time_warp.rs`** *(Target: ~50 lines)*
  - [x] Create `crates/synthetic-client/src/systems/time_warp.rs`.
  - [x] Define `WarpLevel` enum with `multiplier()`, `label()`, `step_up()`, `step_down()`.
  - [x] Define `TimeWarp` resource enforcing invariant `previous_non_zero != WarpLevel::Paused`.

- [x] **Step 2: Hermetic Unit Tests for Time Warp Pure Logic** *(Target: ~45 lines)*
  - [x] In `time_warp.rs` test module, implement `test_warp_multipliers` verifying exact numerical values (`0.0`, `1.0`, `60.0`, `3600.0`, `86400.0`, `2592000.0`).
  - [x] Implement `test_warp_stepping_and_clamping` ensuring upper and lower bounds clamp cleanly.
  - [x] Implement `test_pause_toggle_memory` asserting `Space` toggle restores last non-zero speed even after multiple pause calls.

- [x] **Step 3: Keyboard Input Handling & Deconflicting** *(Target: ~45 lines)*
  - [x] In `time_warp.rs`, implement `keyboard_time_warp_system` reading `ButtonInput<KeyCode>` (`Space`, `Digit0`..`Digit5`, `Numpad0`..`Numpad5`, `BracketLeft`, `BracketRight`).
  - [x] In `astronomy.rs`, remove `Digit1`..`Digit9` from `solar_system_camera_focus_system`, resolving the key collision.

- [x] **Step 4: Simulation Time Integration & Celestial Body Orbit Propagation** *(Target: ~50 lines)*
  - [x] Update `advance_simulation_time` to scale delta time by `time_warp.current_multiplier()`.
  - [x] Implement `update_celestial_positions_system` to recompute `body.global_position` at `t_sim` and update `origin.focused_position`.
  - [x] Add unit test `test_advance_simulation_time_scaled_by_warp` in `astronomy.rs`.

- [x] **Step 5: Keybinding Help Modal Component & Toggle System in `ui.rs`** *(Target: ~40 lines)*
  - [x] Define `KeybindingHelpModal` marker component in `ui.rs`.
  - [x] Implement `toggle_keybinding_help_system` checking `KeyCode::KeyH` (toggle) and `KeyCode::Escape` (close only).
  - [x] Add unit test verifying visibility state transitions on key inputs.

- [x] **Step 6a: Keybinding Row UI Helper in `ui.rs`** *(Target: ~35 lines)*
  - [x] Implement `spawn_keybinding_row(parent, key_label, description)` helper to construct standardized shortcut display rows with key tag badge and description text.

- [x] **Step 6b: Keybinding Help Modal UI Layout in `ui.rs`** *(Target: ~45 lines)*
  - [x] Implement `spawn_keybinding_help_modal` system using `spawn_keybinding_row`.
  - [x] Build centered card with dark translucent backdrop (`Color::srgba(0.04, 0.06, 0.10, 0.92)`) and cyan borders.
  - [x] Populate sections for Simulation Speed (`Space`, `0`-`5`, `[`/`]`), Camera Controls, and Navigation/Focus.

- [x] **Step 7: Plugin Wire-Up, System Scheduling & Workspace Regression** *(Target: ~40 lines)*
  - [x] Register `TimeWarpPlugin` in `crates/synthetic-client/src/systems/mod.rs` and `main.rs`.
  - [x] Enforce startup ordering: `spawn_keybinding_help_modal.after(crate::setup_ui)`.
  - [x] Enforce update ordering: `keyboard_time_warp_system.before(advance_simulation_time).in_set(SimulationTimeSystem)`.
  - [x] Run `cargo check --workspace --all-targets`.
  - [x] Run `cargo clippy --workspace --all-targets -- -D warnings`.
  - [x] Run `cargo test --workspace` to ensure 100% test pass rate.

## 🧪 12. Verification & Criteria

### 12.1 Unit & Invariant Test Targets
| Module / File | Test Name | Key Scenarios Covered |
| :--- | :--- | :--- |
| `time_warp.rs` | `test_warp_multipliers` | Assert exact numerical multipliers for all 6 levels. |
| `time_warp.rs` | `test_warp_stepping_and_clamping` | Assert `step_down` stops at `0x` and `step_up` stops at `2,592,000x`. |
| `time_warp.rs` | `test_pause_toggle_memory` | Assert pause toggles to `0x` and back to previous active speed across repeated pause inputs. |
| `astronomy.rs` | `test_advance_simulation_time_scaled_by_warp` | Assert `sim_time` advances by `delta * multiplier`. |
| `astronomy.rs` | `test_celestial_body_positions_advance_with_time` | Assert `global_position` changes deterministically as simulation time advances. |
| `ui.rs` | `test_toggle_keybinding_help_system` | Assert `KeyH` flips visibility and `Escape` hides when visible. |

### 12.2 Verification Commands
```bash
# Typecheck / Cargo Check
cargo check --workspace --all-targets

# Linter / Cargo Clippy
cargo clippy --workspace --all-targets -- -D warnings

# Hermetic Test Suite & Invariants
cargo test --workspace
```

---

## 📝 13. Deviations & Retrospective (Post-Implementation)

### 13.1 Architectural Deviations & Runtime Discoveries
- **Dynamic Celestial Body Propagation (Resolved "Frozen Planets"):** During technical planning, the adversarial audit identified that `CelestialBody.global_position` was previously only evaluated once at epoch `t=0`. Added `update_celestial_positions_system` to compute Keplerian positions at `t_sim` dynamically across all bodies and update `origin.focused_position`, ensuring planets physically orbit their parents and the focused camera tracks moving targets.
- **Digit Key Deconfliction:** `Digit1` through `Digit9` were previously hardcoded to camera focus selection in `astronomy.rs`. These were removed, allowing `Digit0` through `Digit5` to serve exclusively as warp presets without conflicting camera jumps. Body focus remains fully supported via `Tab` / `Shift-Tab`, arrow keys, direct mouse picking, and the UI selector list.
- **ECS Scheduling Rigor:** Explicitly ordered `keyboard_time_warp_system.before(advance_simulation_time).in_set(SimulationTimeSystem)` to prevent a 1-frame (~12-hour) lag when pausing from maximum warp.
- **Modal Row Builder Extraction:** To respect the <= ~50 line rule, UI construction was decomposed into `spawn_keybinding_row` and `spawn_keybinding_help_modal`.
- **Dynamic Bounding Box (AABB) Frustum Culling Resolution:** Discovered that as simulation time advances months, the 401 orbital points for each celestial body travel hundreds of millions of kilometers across space. Mutating vertex buffers in `Assets<Mesh>` in-place does not automatically update Bevy's entity `Aabb` component, leading to camera frustum culling at close/intermediate zooms. Updated `update_orbital_dots_system` to dynamically calculate and update `*aabb = new_aabb` on each entity every frame, maintaining performant frustum culling while preventing orbital dot disappearance.

## 🔮 14. Follow-Up Tasks & Next Steps
- **HUD Speed Indicator Widget:** Add an active warp rate badge (e.g. `WARP: 1 day/sec` or `PAUSED`) to the top HUD so users can verify current simulation speed without opening the help modal.
- **Timeline Scrubber / Date Picker:** Future HUD enhancements could incorporate a continuous date-time scrubber and epoch reset control.
- **International Keyboard Layout Remapping:** Support localized keys or configurable keybinds for stepping warp speed on keyboards without dedicated bracket keys.
