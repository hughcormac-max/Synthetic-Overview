import { useState, useMemo } from "react";
import type {
  SimulationStateDto,
  OrbitalState,
  ConverterDto,
  StorageDto,
  FlowEdgeDto,
} from "../types/simulation.js";
import {
  calculateOrbitalPosition,
  formatOrbitalDistance,
  formatOrbitalPeriod,
} from "../domain/orbital.js";

export type ActiveTab = "nodes" | "storages" | "flows" | "orbital";

export interface NetworkTablesProps {
  readonly simState: SimulationStateDto;
}

const PAGE_SIZE = 15;

function getConverterCategory(recipeId: number): string {
  if (recipeId <= 10) return "Mine (Raw Extractor)";
  if (recipeId <= 30) return "Intermediate Factory";
  if (recipeId <= 40) return "Consumer Goods Factory";
  return "Population Center";
}

export function NetworkTables({ simState }: NetworkTablesProps) {
  const [activeTab, setActiveTab] = useState<ActiveTab>("nodes");
  const [searchQuery, setSearchQuery] = useState<string>("");
  const [activeFlowsOnly, setActiveFlowsOnly] = useState<boolean>(false);
  const [page, setPage] = useState<number>(0);

  const resourceMap = useMemo(() => {
    const map = new Map<number, string>();
    for (const res of simState.resources) {
      map.set(res.id, res.name);
    }
    return map;
  }, [simState.resources]);

  const filteredConverters = useMemo(() => {
    return simState.converters.filter((c: ConverterDto) => {
      const cat = getConverterCategory(c.recipe_id).toLowerCase();
      const idStr = String(c.entity_id);
      const query = searchQuery.toLowerCase();
      return !query || idStr.includes(query) || cat.includes(query);
    });
  }, [simState.converters, searchQuery]);

  const filteredStorages = useMemo(() => {
    return simState.storages.filter((s: StorageDto) => {
      const resName = (resourceMap.get(s.resource_id) ?? "").toLowerCase();
      const idStr = String(s.entity_id);
      const resIdStr = String(s.resource_id);
      const query = searchQuery.toLowerCase();
      return !query || idStr.includes(query) || resName.includes(query) || resIdStr.includes(query);
    });
  }, [simState.storages, resourceMap, searchQuery]);

  const filteredEdges = useMemo(() => {
    return simState.edges.filter((e: FlowEdgeDto) => {
      if (activeFlowsOnly && e.in_transit <= 0) return false;
      const idStr = String(e.edge_id);
      const srcStr = String(e.source_id);
      const dstStr = String(e.destination_id);
      const query = searchQuery.toLowerCase();
      return !query || idStr.includes(query) || srcStr.includes(query) || dstStr.includes(query);
    });
  }, [simState.edges, activeFlowsOnly, searchQuery]);

  const handleTabChange = (tab: ActiveTab) => {
    setActiveTab(tab);
    setPage(0);
  };

  return (
    <main className="panel-container">
      <div className="tab-row">
        <button
          type="button"
          className={`tab-btn ${activeTab === "nodes" ? "active" : ""}`}
          onClick={() => handleTabChange("nodes")}
        >
          CONVERTERS ({simState.converters.length})
        </button>
        <button
          type="button"
          className={`tab-btn ${activeTab === "storages" ? "active" : ""}`}
          onClick={() => handleTabChange("storages")}
        >
          STOCKPILES ({simState.storages.length})
        </button>
        <button
          type="button"
          className={`tab-btn ${activeTab === "flows" ? "active" : ""}`}
          onClick={() => handleTabChange("flows")}
        >
          FLOW EDGES ({simState.edges.length})
        </button>
        <button
          type="button"
          className={`tab-btn ${activeTab === "orbital" ? "active" : ""}`}
          onClick={() => handleTabChange("orbital")}
        >
          ORBITAL EPHEMERIS ({simState.entities.length})
        </button>
      </div>

      <div className="filter-bar">
        <input
          type="text"
          className="search-input"
          placeholder="Filter by ID or Name..."
          value={searchQuery}
          onChange={(e) => {
            setSearchQuery(e.target.value);
            setPage(0);
          }}
        />

        {activeTab === "flows" && (
          <label style={{ fontSize: "0.8rem", color: "#8b949e", cursor: "pointer" }}>
            <input
              type="checkbox"
              checked={activeFlowsOnly}
              onChange={(e) => {
                setActiveFlowsOnly(e.target.checked);
                setPage(0);
              }}
              style={{ marginRight: "0.4rem" }}
            />
            Active In-Transit Only
          </label>
        )}

        {activeTab !== "orbital" && (
          <div className="pagination-controls">
            <button
              type="button"
              className="btn-tactical"
              style={{ padding: "0.2rem 0.5rem", fontSize: "0.75rem" }}
              disabled={page === 0}
              onClick={() => setPage((p) => Math.max(0, p - 1))}
            >
              PREV
            </button>
            <span>PAGE {page + 1}</span>
            <button
              type="button"
              className="btn-tactical"
              style={{ padding: "0.2rem 0.5rem", fontSize: "0.75rem" }}
              disabled={
                activeTab === "nodes"
                  ? (page + 1) * PAGE_SIZE >= filteredConverters.length
                  : activeTab === "storages"
                  ? (page + 1) * PAGE_SIZE >= filteredStorages.length
                  : (page + 1) * PAGE_SIZE >= filteredEdges.length
              }
              onClick={() => setPage((p) => p + 1)}
            >
              NEXT
            </button>
          </div>
        )}
      </div>

      <div className="table-wrapper">
        {activeTab === "nodes" && (
          <table className="tactical-table">
            <thead>
              <tr>
                <th>ENTITY ID</th>
                <th>CLASSIFICATION</th>
                <th>RECIPE ID</th>
                <th>HEALTH</th>
                <th>STATUS</th>
              </tr>
            </thead>
            <tbody>
              {filteredConverters
                .slice(page * PAGE_SIZE, (page + 1) * PAGE_SIZE)
                .map((c: ConverterDto) => (
                  <tr key={c.entity_id}>
                    <td>#{c.entity_id}</td>
                    <td>{getConverterCategory(c.recipe_id)}</td>
                    <td>R-{c.recipe_id}</td>
                    <td>100.0%</td>
                    <td>
                      <span className="status-badge">ONLINE</span>
                    </td>
                  </tr>
                ))}
            </tbody>
          </table>
        )}

        {activeTab === "storages" && (
          <table className="tactical-table">
            <thead>
              <tr>
                <th>STORAGE ID</th>
                <th>RESOURCE</th>
                <th>STORED AMOUNT</th>
                <th>MAX CAPACITY</th>
                <th>UTILIZATION</th>
              </tr>
            </thead>
            <tbody>
              {filteredStorages
                .slice(page * PAGE_SIZE, (page + 1) * PAGE_SIZE)
                .map((s: StorageDto) => {
                  const name = resourceMap.get(s.resource_id) ?? `Resource ${s.resource_id}`;
                  const amountUnits = (s.amount / 1_000_000).toFixed(1);
                  const capUnits = (s.capacity / 1_000_000).toFixed(1);
                  const utilPercent = s.capacity > 0 ? ((s.amount / s.capacity) * 100).toFixed(1) : "0.0";
                  return (
                    <tr key={s.entity_id}>
                      <td>#{s.entity_id}</td>
                      <td>{name} (#{s.resource_id})</td>
                      <td>{amountUnits} units</td>
                      <td>{capUnits} units</td>
                      <td>{utilPercent}%</td>
                    </tr>
                  );
                })}
            </tbody>
          </table>
        )}

        {activeTab === "flows" && (
          <table className="tactical-table">
            <thead>
              <tr>
                <th>EDGE ID</th>
                <th>SOURCE STOCK</th>
                <th>DESTINATION STOCK</th>
                <th>IN-TRANSIT AMOUNT</th>
                <th>STATUS</th>
              </tr>
            </thead>
            <tbody>
              {filteredEdges
                .slice(page * PAGE_SIZE, (page + 1) * PAGE_SIZE)
                .map((e: FlowEdgeDto) => {
                  const inTransitUnits = (e.in_transit / 1_000_000).toFixed(1);
                  const isActive = e.in_transit > 0;
                  return (
                    <tr key={e.edge_id}>
                      <td>#{e.edge_id}</td>
                      <td>Stock #{e.source_id}</td>
                      <td>Stock #{e.destination_id}</td>
                      <td>{inTransitUnits} units</td>
                      <td>
                        {isActive ? (
                          <span className="status-badge" style={{ borderColor: "#1f6feb", color: "#58a6ff" }}>
                            IN TRANSIT
                          </span>
                        ) : (
                          <span style={{ color: "#8b949e" }}>IDLE</span>
                        )}
                      </td>
                    </tr>
                  );
                })}
            </tbody>
          </table>
        )}

        {activeTab === "orbital" && (
          <table className="tactical-table">
            <thead>
              <tr>
                <th>ID</th>
                <th>BARYCENTER</th>
                <th>TRUE ANOMALY</th>
                <th>SEMI-MAJOR AXIS</th>
                <th>ECCENTRICITY</th>
                <th>PERIOD</th>
                <th>PLANAR POS (X, Y)</th>
              </tr>
            </thead>
            <tbody>
              {simState.entities.map((entity: OrbitalState) => {
                const pos = calculateOrbitalPosition(entity);
                const anomalyDeg = ((entity.true_anomaly * 180.0) / Math.PI).toFixed(2);
                return (
                  <tr key={entity.entity_id}>
                    <td>#{entity.entity_id}</td>
                    <td>B-{entity.barycenter_id}</td>
                    <td>{anomalyDeg} deg</td>
                    <td>{formatOrbitalDistance(entity.semi_major_axis)}</td>
                    <td>{entity.eccentricity.toFixed(4)}</td>
                    <td>{formatOrbitalPeriod(entity.orbital_period)}</td>
                    <td>
                      {formatOrbitalDistance(pos.x)}, {formatOrbitalDistance(pos.y)}
                    </td>
                  </tr>
                );
              })}
            </tbody>
          </table>
        )}
      </div>
    </main>
  );
}
