/**
 * Core deterministic Keplerian orbital state contract.
 * Immutability enforced by readonly properties.
 */
export interface OrbitalState {
  readonly entity_id: number;
  readonly barycenter_id: number;
  readonly true_anomaly: number;
  readonly semi_major_axis: number;
  readonly eccentricity: number;
  readonly orbital_period: number;
}

/**
 * 2D Planar coordinate vector relative to barycenter.
 */
export interface Vector2D {
  readonly x: number;
  readonly y: number;
}

/**
 * Simulation tick state transfer object received across Tauri IPC.
 */
export interface SimulationStateDto {
  readonly tick: number;
  readonly timestamp_seconds: number;
  readonly delta_time_seconds: number;
  readonly entities: readonly OrbitalState[];
}

/**
 * Cyber-OS telemetry status metric.
 */
export interface TelemetryMetric {
  readonly label: string;
  readonly value: string;
  readonly unit?: string;
}
