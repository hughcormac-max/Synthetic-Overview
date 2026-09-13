import { describe, it, expect } from "vitest";
import { generateH3Grid, getCellCentroid, generateGraticuleLines } from "./spatial.js";

describe("spatial domain grid generation", () => {
  it("generates 122 base cells at resolution 0", () => {
    const grid = generateH3Grid(0);
    expect(grid).toHaveLength(122);
  });

  it("generates 41,162 cells at resolution 3", () => {
    const grid = generateH3Grid(3);
    expect(grid).toHaveLength(41162);
    // Verify sample cell format
    expect(grid[0]).toMatch(/^[0-9a-f]{15}$/);
  });

  it("calculates accurate centroid coordinates for London cell", () => {
    const [lat, lng] = getCellCentroid("83194afffffffff");
    expect(lat).toBeGreaterThan(50.0);
    expect(lat).toBeLessThan(52.0);
    expect(lng).toBeGreaterThan(-1.0);
    expect(lng).toBeLessThan(1.0);
  });

  it("generates complete set of lat/long graticule lines", () => {
    const lines = generateGraticuleLines(30);
    expect(lines.length).toBe(23); // 11 parallels + 12 meridians
    const equator = lines.find((l) => l.label === "EQUATOR");
    expect(equator).toBeDefined();
    expect(equator?.isMajor).toBe(true);

    const primeMeridian = lines.find((l) => l.label === "PRIME MERIDIAN");
    expect(primeMeridian).toBeDefined();
    expect(primeMeridian?.isMajor).toBe(true);
  });
});
