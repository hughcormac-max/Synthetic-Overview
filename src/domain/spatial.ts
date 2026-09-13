import * as h3 from "h3-js";

/**
 * Deterministically generates all H3 cells for a given resolution across the global sphere.
 */
export function generateH3Grid(resolution: number): string[] {
  const res0Cells = h3.getRes0Cells();
  const cells: string[] = [];
  for (const cell of res0Cells) {
    const children = h3.cellToChildren(cell, resolution);
    for (let i = 0; i < children.length; i++) {
      const child = children[i];
      if (child !== undefined) {
        cells.push(child);
      }
    }
  }
  return cells;
}

/**
 * Retrieves the latitude and longitude centroid [lat, lng] for a given H3 cell.
 */
export function getCellCentroid(cellIndex: string): [number, number] {
  return h3.cellToLatLng(cellIndex);
}

/**
 * Geometric polyline representation for astronomical body graticules (lat/long lines).
 */
export interface GraticuleLine {
  readonly path: readonly [number, number][];
  readonly isMajor: boolean;
  readonly label: string;
}

/**
 * Deterministically generates lat/long graticule lines across the sphere.
 * Used to render rough lat/long spherical proxy lines without requiring a surface texture.
 */
export function generateGraticuleLines(stepDegrees: number = 30): readonly GraticuleLine[] {
  const lines: GraticuleLine[] = [];

  // Parallels (Latitudes from -75 to +75 degrees)
  for (let lat = -75; lat <= 75; lat += stepDegrees / 2) {
    const isMajor = lat === 0;
    const path: [number, number][] = [];
    for (let lng = -180; lng <= 180; lng += 5) {
      path.push([lng, lat]);
    }
    const label = lat === 0 ? "EQUATOR" : `${Math.abs(lat)}deg ${lat > 0 ? "N" : "S"}`;
    lines.push({ path, isMajor, label });
  }

  // Meridians (Longitudes from -180 to +150 degrees)
  for (let lng = -180; lng < 180; lng += stepDegrees) {
    const isMajor = lng === 0;
    const path: [number, number][] = [];
    for (let lat = -85; lat <= 85; lat += 5) {
      path.push([lng, lat]);
    }
    const label = lng === 0 ? "PRIME MERIDIAN" : `${Math.abs(lng)}deg ${lng > 0 ? "E" : "W"}`;
    lines.push({ path, isMajor, label });
  }

  return lines;
}
