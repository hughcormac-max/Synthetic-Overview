import { useEffect, useState, useCallback } from "react";
import "./App.css";
import type { SimulationStateDto, OrbitalState } from "./types/simulation.js";
import {
  calculateOrbitalPosition,
  formatOrbitalDistance,
  formatOrbitalPeriod,
} from "./domain/orbital.js";

const DEFAULT_SIMULATION_STATE: SimulationStateDto = {
  tick: 0,
  timestamp_seconds: 0.0,
  delta_time_seconds: 86400.0,
  entities: [
    {
      entity_id: 1,
      barycenter_id: 0,
      true_anomaly: 0.0,
      semi_major_axis: 1.49598e11,
      eccentricity: 0.0167086,
      orbital_period: 31558149.0,
    },
    {
      entity_id: 2,
      barycenter_id: 0,
      true_anomaly: 0.5,
      semi_major_axis: 2.27939e11,
      eccentricity: 0.0934,
      orbital_period: 59355072.0,
    },
  ],
};

function hasTauriBackend(): boolean {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

export function App() {
  const [simState, setSimState] = useState<SimulationStateDto>(DEFAULT_SIMULATION_STATE);
  const [isRunning, setIsRunning] = useState<boolean>(false);
  const [errorMessage, setErrorMessage] = useState<string | null>(null);

  const fetchState = useCallback(async () => {
    if (hasTauriBackend()) {
      try {
        const { invoke } = await import("@tauri-apps/api/core");
        const res = await invoke<SimulationStateDto>("fetch_simulation_tick");
        setSimState(res);
        setErrorMessage(null);
      } catch (err) {
        setErrorMessage(String(err));
      }
    }
  }, []);

  const handleStep = useCallback(async () => {
    if (hasTauriBackend()) {
      try {
        const { invoke } = await import("@tauri-apps/api/core");
        const res = await invoke<SimulationStateDto>("step_simulation_tick", {
          deltaSeconds: simState.delta_time_seconds,
        });
        setSimState(res);
        setErrorMessage(null);
      } catch (err) {
        setErrorMessage(String(err));
      }
    } else {
      // Local fallback in browser development mode
      setSimState((prev) => ({
        ...prev,
        tick: prev.tick + 1,
        timestamp_seconds: prev.timestamp_seconds + prev.delta_time_seconds,
        entities: prev.entities.map((entity) => ({
          ...entity,
          true_anomaly: (entity.true_anomaly + 0.05) % (2.0 * Math.PI),
        })),
      }));
    }
  }, [simState.delta_time_seconds]);

  const handleReset = useCallback(async () => {
    if (hasTauriBackend()) {
      try {
        const { invoke } = await import("@tauri-apps/api/core");
        const res = await invoke<SimulationStateDto>("reset_simulation");
        setSimState(res);
        setErrorMessage(null);
      } catch (err) {
        setErrorMessage(String(err));
      }
    } else {
      setSimState(DEFAULT_SIMULATION_STATE);
    }
    setIsRunning(false);
  }, []);

  useEffect(() => {
    void fetchState();
  }, [fetchState]);

  useEffect(() => {
    if (!isRunning) return;
    const interval = setInterval(() => {
      void handleStep();
    }, 200);
    return () => clearInterval(interval);
  }, [isRunning, handleStep]);

  const elapsedDays = (simState.timestamp_seconds / 86400.0).toFixed(2);

  return (
    <div className="app-container">
      <header className="header-bar">
        <div className="system-title">
          <span>SYNTHETIC OVERVIEW</span>
          <span className="status-badge">TIER-0 ACTIVE</span>
        </div>
        <div className="controls-row">
          <button
            type="button"
            className={`btn-tactical ${isRunning ? "btn-primary" : ""}`}
            onClick={() => setIsRunning(!isRunning)}
          >
            {isRunning ? "PAUSE CLOCK" : "START CLOCK"}
          </button>
          <button type="button" className="btn-tactical" onClick={() => void handleStep()}>
            STEP TICK
          </button>
          <button type="button" className="btn-tactical" onClick={() => void handleReset()}>
            RESET
          </button>
        </div>
      </header>

      {errorMessage && (
        <div style={{ color: "#f85149", padding: "0.5rem", background: "rgba(248, 81, 73, 0.1)" }}>
          {errorMessage}
        </div>
      )}

      <div className="metrics-grid">
        <div className="metric-card">
          <span className="metric-label">TICK INDEX</span>
          <span className="metric-value">{simState.tick}</span>
        </div>
        <div className="metric-card">
          <span className="metric-label">ELAPSED TIME</span>
          <span className="metric-value">{elapsedDays} days</span>
        </div>
        <div className="metric-card">
          <span className="metric-label">STEP RESOLUTION</span>
          <span className="metric-value">{(simState.delta_time_seconds / 3600.0).toFixed(1)} hrs</span>
        </div>
        <div className="metric-card">
          <span className="metric-label">TRACKED ENTITIES</span>
          <span className="metric-value">{simState.entities.length}</span>
        </div>
      </div>

      <main className="panel-container">
        <div className="panel-header">RADAR TELEMETRY // ORBITAL EPHEMERIS</div>
        <div className="table-wrapper">
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
        </div>
      </main>
    </div>
  );
}

export default App;
