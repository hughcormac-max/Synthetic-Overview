//! Synthetic Overview desktop application library.
//!
//! Provides the Tauri IPC bridge between the Rust simulation brain
//! and the React/WebGPU presentation layer.

#![allow(clippy::needless_pass_by_value)]

use std::sync::Mutex;
use synthetic_core::{SimulationSession, SimulationStateDto};

/// Thread-safe managed state container for the active simulation session.
pub struct SimulationStateContainer(pub Mutex<SimulationSession>);

/// Tauri command returning the active simulation tick snapshot to the webview.
#[tauri::command]
fn fetch_simulation_tick(
    state: tauri::State<'_, SimulationStateContainer>,
) -> Result<SimulationStateDto, String> {
    let mut guard = state.0.lock().map_err(|err| format!("Lock poisoned: {err}"))?;
    Ok(guard.extract_dto())
}

/// Tauri command advancing the simulation by one discrete tick.
#[tauri::command]
fn step_simulation_tick(
    delta_seconds: Option<f64>,
    state: tauri::State<'_, SimulationStateContainer>,
) -> Result<SimulationStateDto, String> {
    let mut guard = state.0.lock().map_err(|err| format!("Lock poisoned: {err}"))?;
    let dt = delta_seconds.unwrap_or(86_400.0);
    guard.step(dt).map_err(|err| err.to_string())
}

/// Tauri command resetting the simulation back to initial baseline parameters.
#[tauri::command]
fn reset_simulation(
    state: tauri::State<'_, SimulationStateContainer>,
) -> Result<SimulationStateDto, String> {
    let mut guard = state.0.lock().map_err(|err| format!("Lock poisoned: {err}"))?;
    *guard = SimulationSession::new();
    Ok(guard.extract_dto())
}

/// Launches the Tauri desktop application.
///
/// # Panics
/// Panics if Tauri builder fails to initialize window context or run event loop.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(SimulationStateContainer(Mutex::new(SimulationSession::new())))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            fetch_simulation_tick,
            step_simulation_tick,
            reset_simulation
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
