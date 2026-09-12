import type { OrbitalState, Vector2D } from "../types/simulation.js";

const TWO_PI = 2.0 * Math.PI;

/**
 * Normalizes any angle into the half-open interval [0.0, 2.0 * PI).
 */
export function normalizeAngle(angle: number): number {
  const remainder = angle % TWO_PI;
  return remainder < 0.0 ? remainder + TWO_PI : remainder;
}

/**
 * Computes 2D planar position (x, y) relative to the focal barycenter.
 * Formula:
 *   semi_latus_rectum = a * (1 - e^2)
 *   r = semi_latus_rectum / (1 + e * cos(nu))
 *   x = r * cos(nu)
 *   y = r * sin(nu)
 */
export function calculateOrbitalPosition(state: OrbitalState): Vector2D {
  const e = state.eccentricity;
  const a = state.semi_major_axis;
  const nu = state.true_anomaly;

  const semiLatusRectum = a * (1.0 - e * e);
  const radius = semiLatusRectum / (1.0 + e * Math.cos(nu));
  const x = radius * Math.cos(nu);
  const y = radius * Math.sin(nu);

  return { x, y };
}

/**
 * Formats distance in meters to clean plain-text AU or kilometers.
 */
export function formatOrbitalDistance(meters: number): string {
  const astronomicalUnit = 1.495978707e11;
  if (meters >= astronomicalUnit * 0.1) {
    const au = meters / astronomicalUnit;
    return `${au.toFixed(3)} AU`;
  }
  const km = meters / 1000.0;
  return `${km.toLocaleString("en-US", { maximumFractionDigits: 1 })} km`;
}

/**
 * Formats duration in seconds to clean human-readable duration (days/hours).
 */
export function formatOrbitalPeriod(seconds: number): string {
  const dayInSeconds = 86400.0;
  if (seconds >= dayInSeconds) {
    const days = seconds / dayInSeconds;
    return `${days.toFixed(2)} days`;
  }
  const hours = seconds / 3600.0;
  return `${hours.toFixed(2)} hours`;
}
