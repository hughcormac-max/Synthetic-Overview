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
 * Resource definition in the global simulation directory.
 */
export interface ResourceDto {
  readonly id: number;
  readonly name: string;
}

/**
 * Storage reservoir snapshot.
 */
export interface StorageDto {
  readonly entity_id: number;
  readonly resource_id: number;
  readonly amount: number;
  readonly capacity: number;
}

/**
 * Converter node snapshot.
 */
export interface ConverterDto {
  readonly entity_id: number;
  readonly recipe_id: number;
}

/**
 * Directed flow edge snapshot.
 */
export interface FlowEdgeDto {
  readonly edge_id: number;
  readonly source_id: number;
  readonly destination_id: number;
  readonly in_transit: number;
}

/**
 * Astronomical body proxy snapshot.
 */
export interface AstroNodeDto {
  readonly entity_id: number;
  readonly body_id: number;
  readonly radius_km: number;
  readonly h3_resolution: number;
}

/**
 * Surface facility node snapshot attached to an H3 cell.
 */
export interface SurfaceNodeDto {
  readonly entity_id: number;
  readonly parent_body_id: number;
  readonly h3_cell_index: string;
}

/**
 * Simulation tick state transfer object received across Tauri IPC.
 */
export interface SimulationStateDto {
  readonly tick: number;
  readonly timestamp_seconds: number;
  readonly delta_time_seconds: number;
  readonly entities: readonly OrbitalState[];
  readonly resources: readonly ResourceDto[];
  readonly storages: readonly StorageDto[];
  readonly converters: readonly ConverterDto[];
  readonly edges: readonly FlowEdgeDto[];
  readonly astro_nodes: readonly AstroNodeDto[];
  readonly surface_nodes: readonly SurfaceNodeDto[];
}

/**
 * Cyber-OS telemetry status metric.
 */
export interface TelemetryMetric {
  readonly label: string;
  readonly value: string;
  readonly unit?: string;
}
