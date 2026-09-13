import React, { useMemo, useState } from "react";
import { _GlobeView } from "@deck.gl/core";
import DeckGL from "@deck.gl/react";
import { H3HexagonLayer } from "@deck.gl/geo-layers";
import { PathLayer } from "@deck.gl/layers";
import type { PickingInfo } from "@deck.gl/core";
import type { SimulationStateDto, SurfaceNodeDto, AstroNodeDto } from "../types/simulation.js";
import { generateH3Grid, generateGraticuleLines } from "../domain/spatial.js";
import type { GraticuleLine } from "../domain/spatial.js";

interface SurfaceMapProps {
  readonly simState: SimulationStateDto;
}

const INITIAL_VIEW_STATE = {
  longitude: 0,
  latitude: 10,
  zoom: 0,
  minZoom: -1,
  maxZoom: 8,
};

const GLOBE_VIEW = new _GlobeView({ id: "globe-view", controller: true });

const FALLBACK_BODY: AstroNodeDto = {
  entity_id: 1,
  body_id: 1,
  radius_km: 6371,
  h3_resolution: 3,
};

export const SurfaceMap: React.FC<SurfaceMapProps> = ({ simState }) => {
  const { astro_nodes = [], surface_nodes = [] } = simState;

  const [selectedBodyId, setSelectedBodyId] = useState<number>(1);

  const activeBody: AstroNodeDto = useMemo(() => {
    return (
      astro_nodes.find((b) => b.body_id === selectedBodyId) ??
      astro_nodes[0] ??
      FALLBACK_BODY
    );
  }, [astro_nodes, selectedBodyId]);

  // Pre-generate the global H3 hexagonal cells for the selected resolution
  const baseGridCells = useMemo(() => {
    return generateH3Grid(activeBody.h3_resolution);
  }, [activeBody.h3_resolution]);

  // Generate rough lat/long spherical proxy graticule lines
  const graticuleLines = useMemo(() => {
    return generateGraticuleLines(30);
  }, []);

  // Surface facilities placed on the currently selected body
  const activeFacilities: SurfaceNodeDto[] = useMemo(() => {
    return surface_nodes.filter((s) => s.parent_body_id === activeBody.body_id);
  }, [surface_nodes, activeBody.body_id]);

  const isMars = activeBody.body_id === 2;

  // 1. Base H3 Hexagon Grid Layer (solid planetary proxy surface + hex outlines)
  const baseGridLayer = useMemo(() => {
    const fillColor: [number, number, number, number] = isMars
      ? [44, 20, 16, 255]
      : [12, 22, 38, 255];
    const lineColor: [number, number, number, number] = isMars
      ? [230, 110, 40, 100]
      : [0, 190, 230, 90];

    return new H3HexagonLayer<string>({
      id: `h3-base-grid-${activeBody.body_id}`,
      data: baseGridCells,
      getHexagon: (cell: string) => cell,
      filled: true,
      wireframe: true,
      getFillColor: fillColor,
      getLineColor: lineColor,
      lineWidthMinPixels: 1,
      pickable: true,
      autoHighlight: false,
    });
  }, [baseGridCells, activeBody.body_id, isMars]);

  // 2. Lat/Long Graticule Lines (rough lat/long lines for the Rimworld planetary map aesthetic)
  const graticuleLayer = useMemo(() => {
    return new PathLayer<GraticuleLine>({
      id: `graticule-${activeBody.body_id}`,
      data: graticuleLines as GraticuleLine[],
      getPath: (d: GraticuleLine) => d.path as [number, number][],
      getColor: (d: GraticuleLine) =>
        d.isMajor ? [0, 240, 255, 220] : [0, 150, 210, 100],
      getWidth: (d: GraticuleLine) => (d.isMajor ? 2 : 1),
      widthUnits: "pixels",
      widthMinPixels: 1,
      pickable: false,
    });
  }, [graticuleLines, activeBody.body_id]);

  // 3. Surface Facilities Highlight Layer
  const facilitiesLayer = useMemo(() => {
    const facilityColor: [number, number, number, number] = isMars
      ? [255, 200, 60, 255]
      : [0, 255, 180, 255];

    return new H3HexagonLayer<SurfaceNodeDto>({
      id: `h3-facilities-${activeBody.body_id}`,
      data: activeFacilities,
      getHexagon: (node: SurfaceNodeDto) => node.h3_cell_index,
      filled: true,
      wireframe: true,
      getFillColor: facilityColor,
      getLineColor: [255, 255, 255, 255],
      lineWidthMinPixels: 2,
      extruded: true,
      getElevation: 60_000,
      pickable: true,
      autoHighlight: true,
      highlightColor: [255, 230, 0, 255],
    });
  }, [activeFacilities, activeBody.body_id, isMars]);

  const layers = useMemo(
    () => [baseGridLayer, graticuleLayer, facilitiesLayer],
    [baseGridLayer, graticuleLayer, facilitiesLayer]
  );

  const renderTooltip = (info: PickingInfo) => {
    if (!info.object) return null;
    if (typeof info.object === "string") {
      return {
        text: `H3 CELL: ${info.object}\nRESOLUTION: ${activeBody.h3_resolution}`,
        style: {
          backgroundColor: "#070e1a",
          color: "#00f0ff",
          border: "1px solid #00f0ff55",
          fontSize: "11px",
          fontFamily: "monospace",
          padding: "6px 10px",
          borderRadius: "4px",
        },
      };
    }
    const node = info.object as SurfaceNodeDto;
    return {
      text: `[FACILITY #${node.entity_id}]\nBODY: ${node.parent_body_id}\nCELL: ${node.h3_cell_index}`,
      style: {
        backgroundColor: "#070e1a",
        color: "#00ffb0",
        border: "1px solid #00ffb088",
        fontSize: "11px",
        fontFamily: "monospace",
        padding: "6px 10px",
        borderRadius: "4px",
      },
    };
  };

  return (
    <section className="surface-map-panel">
      <div className="surface-map-header">
        <div className="surface-map-title">
          <span className="pulse-indicator" />
          <span>Surface Layer Spatial Grid // H3 Hex Visualization</span>
        </div>

        <div className="body-selector-group">
          <span className="body-selector-label">SELECT BODY:</span>
          {astro_nodes.map((node) => {
            const isSelected = node.body_id === activeBody.body_id;
            const bodyName =
              node.body_id === 1 ? "Earth" : node.body_id === 2 ? "Mars" : `Body ${node.body_id}`;
            return (
              <button
                key={node.body_id}
                type="button"
                onClick={() => setSelectedBodyId(node.body_id)}
                className={`btn-tactical ${isSelected ? "btn-primary" : ""}`}
              >
                {bodyName.toUpperCase()} (ID: {node.body_id})
              </button>
            );
          })}
        </div>
      </div>

      <div className="surface-meta-strip">
        <div className="meta-item">
          <span className="meta-label">TARGET:</span>
          <span className="meta-val">
            {activeBody.body_id === 1 ? "EARTH" : activeBody.body_id === 2 ? "MARS" : `BODY ${activeBody.body_id}`}
          </span>
        </div>
        <div className="meta-item">
          <span className="meta-label">RADIUS:</span>
          <span className="meta-val">{activeBody.radius_km.toLocaleString()} km</span>
        </div>
        <div className="meta-item">
          <span className="meta-label">H3 RESOLUTION:</span>
          <span className="meta-val-accent">Level {activeBody.h3_resolution}</span>
        </div>
        <div className="meta-item">
          <span className="meta-label">SURFACE FACILITIES:</span>
          <span className="meta-val-accent">{activeFacilities.length} active</span>
        </div>
      </div>

      <div className="surface-map-viewport" style={{ height: "520px" }}>
        <DeckGL
          views={GLOBE_VIEW}
          initialViewState={INITIAL_VIEW_STATE}
          controller={true}
          layers={layers}
          getTooltip={renderTooltip}
          width="100%"
          height="100%"
          style={{ position: "absolute", top: "0px", left: "0px", width: "100%", height: "100%" }}
        />

        <div className="surface-hud-badge surface-hud-badge-bottom-left">
          WEBGPU / DECK.GL // GLOBE VIEWPORT // DRAG: ROTATE | SCROLL: ZOOM
        </div>

        <div className="surface-hud-badge surface-hud-badge-top-right">
          HEX GRID CELLS: {baseGridCells.length.toLocaleString()} // LAT/LONG GRATICULES: ACTIVE
        </div>
      </div>
    </section>
  );
};
