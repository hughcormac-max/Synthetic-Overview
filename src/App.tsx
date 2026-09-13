import { useEffect, useState, useCallback, useRef } from "react";
import "./App.css";
import type { SimulationStateDto } from "./types/simulation.js";
import { MetricCards } from "./components/MetricCards.js";
import { NetworkTables } from "./components/NetworkTables.js";
import { SurfaceMap } from "./components/SurfaceMap.js";

import {
  createDefaultSimulationState,
  stepBrowserSimulation,
} from "./domain/generator.js";

const DEFAULT_SIMULATION_STATE: SimulationStateDto = createDefaultSimulationState();

function hasTauriBackend(): boolean {
  return (
    typeof window !== "undefined" &&
    ("__TAURI_INTERNALS__" in window || "__TAURI__" in window)
  );
}

export function App() {
  const [simState, setSimState] = useState<SimulationStateDto>(DEFAULT_SIMULATION_STATE);
  const [isRunning, setIsRunning] = useState<boolean>(false);
  const [errorMessage, setErrorMessage] = useState<string | null>(null);
  const [tps, setTps] = useState<number>(0.0);
  const [computeLatencyMs, setComputeLatencyMs] = useState<number>(0.0);

  const lastStepTimeRef = useRef<number>(0);

  const fetchState = useCallback(async () => {
    if (hasTauriBackend()) {
      try {
        const { invoke } = await import("@tauri-apps/api/core");
        const t0 = performance.now();
        const res = await invoke<SimulationStateDto>("fetch_simulation_tick");
        setComputeLatencyMs(performance.now() - t0);
        setSimState(res);
        setErrorMessage(null);
      } catch (err) {
        setErrorMessage(String(err));
      }
    }
  }, []);

  const handleStep = useCallback(async () => {
    const stepStart = performance.now();
    if (lastStepTimeRef.current > 0) {
      const deltaSec = (stepStart - lastStepTimeRef.current) / 1000.0;
      if (deltaSec > 0) {
        const instantTps = 1.0 / deltaSec;
        setTps((prev) => (prev === 0 ? instantTps : prev * 0.7 + instantTps * 0.3));
      }
    }
    lastStepTimeRef.current = stepStart;

    if (hasTauriBackend()) {
      try {
        const { invoke } = await import("@tauri-apps/api/core");
        const t0 = performance.now();
        const res = await invoke<SimulationStateDto>("step_simulation_tick", {
          deltaSeconds: simState.delta_time_seconds,
        });
        setComputeLatencyMs(performance.now() - t0);
        setSimState(res);
        setErrorMessage(null);
      } catch (err) {
        setErrorMessage(String(err));
      }
    } else {
      // Local fallback in browser development mode with 200 nodes
      setSimState(stepBrowserSimulation);
      setComputeLatencyMs(performance.now() - stepStart);
    }
  }, [simState.delta_time_seconds]);

  const handleReset = useCallback(async () => {
    if (hasTauriBackend()) {
      try {
        const { invoke } = await import("@tauri-apps/api/core");
        const t0 = performance.now();
        const res = await invoke<SimulationStateDto>("reset_simulation");
        setComputeLatencyMs(performance.now() - t0);
        setSimState(res);
        setErrorMessage(null);
      } catch (err) {
        setErrorMessage(String(err));
      }
    } else {
      setSimState(createDefaultSimulationState());
      setComputeLatencyMs(0.0);
    }
    setIsRunning(false);
    setTps(0.0);
    lastStepTimeRef.current = 0;
  }, []);

  useEffect(() => {
    void fetchState();
  }, [fetchState]);

  useEffect(() => {
    if (!isRunning) {
      setTps(0.0);
      lastStepTimeRef.current = 0;
      return;
    }
    const interval = setInterval(() => {
      void handleStep();
    }, 150);
    return () => clearInterval(interval);
  }, [isRunning, handleStep]);

  const elapsedDays = (simState.timestamp_seconds / 86400.0).toFixed(2);
  const stepHours = (simState.delta_time_seconds / 3600.0).toFixed(1);

  return (
    <div className="app-container">
      <header className="header-bar">
        <div className="system-title">
          <span>SYNTHETIC OVERVIEW</span>
          <span className="status-badge">TIER-0 ACTIVE</span>
          <span
            className="status-badge"
            style={{
              borderColor: hasTauriBackend() ? "#3fb950" : "#d29922",
              color: hasTauriBackend() ? "#3fb950" : "#d29922",
            }}
          >
            {hasTauriBackend() ? "RUST BEVY ECS" : "BROWSER PREVIEW"}
          </span>
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

      <MetricCards
        tick={simState.tick}
        elapsedDays={elapsedDays}
        stepHours={stepHours}
        nodeCount={simState.converters.length}
        tps={tps}
        computeLatencyMs={computeLatencyMs}
      />

      <SurfaceMap simState={simState} />

      <NetworkTables simState={simState} />
    </div>
  );
}

export default App;
