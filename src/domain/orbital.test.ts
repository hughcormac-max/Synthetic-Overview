import { describe, expect, it } from "vitest";
import {
  calculateOrbitalPosition,
  formatOrbitalDistance,
  formatOrbitalPeriod,
  normalizeAngle,
} from "./orbital.js";
import type { OrbitalState } from "../types/simulation.js";

describe("Orbital Domain Calculations", () => {
  it("normalizes negative angles into [0, 2*PI)", () => {
    const angle = -Math.PI / 2.0;
    const normalized = normalizeAngle(angle);
    expect(normalized).toBeCloseTo(1.5 * Math.PI, 10);
  });

  it("normalizes angles exceeding 2*PI", () => {
    const angle = 2.5 * Math.PI;
    const normalized = normalizeAngle(angle);
    expect(normalized).toBeCloseTo(0.5 * Math.PI, 10);
  });

  it("calculates accurate periapsis coordinates (nu = 0)", () => {
    const state: OrbitalState = {
      entity_id: 1,
      barycenter_id: 0,
      true_anomaly: 0.0,
      semi_major_axis: 1000.0,
      eccentricity: 0.2,
      orbital_period: 3600.0,
    };

    const pos = calculateOrbitalPosition(state);
    // At periapsis: r = a * (1 - e) = 1000 * 0.8 = 800
    expect(pos.x).toBeCloseTo(800.0, 6);
    expect(pos.y).toBeCloseTo(0.0, 6);
  });

  it("calculates accurate apoapsis coordinates (nu = PI)", () => {
    const state: OrbitalState = {
      entity_id: 1,
      barycenter_id: 0,
      true_anomaly: Math.PI,
      semi_major_axis: 1000.0,
      eccentricity: 0.2,
      orbital_period: 3600.0,
    };

    const pos = calculateOrbitalPosition(state);
    // At apoapsis: r = a * (1 + e) = 1000 * 1.2 = 1200, direction is -x
    expect(pos.x).toBeCloseTo(-1200.0, 6);
    expect(pos.y).toBeCloseTo(0.0, 6);
  });

  it("formats orbital distance accurately", () => {
    const auDistance = 1.495978707e11;
    expect(formatOrbitalDistance(auDistance)).toBe("1.000 AU");
    expect(formatOrbitalDistance(384400000)).toBe("384,400 km");
  });

  it("formats orbital period into days and hours", () => {
    expect(formatOrbitalPeriod(86400 * 365.25)).toBe("365.25 days");
    expect(formatOrbitalPeriod(5400)).toBe("1.50 hours");
  });
});
