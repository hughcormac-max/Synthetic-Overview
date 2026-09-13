export interface MetricCardsProps {
  readonly tick: number;
  readonly elapsedDays: string;
  readonly stepHours: string;
  readonly nodeCount: number;
  readonly tps: number;
  readonly computeLatencyMs: number;
}

export function MetricCards({
  tick,
  elapsedDays,
  stepHours,
  nodeCount,
  tps,
  computeLatencyMs,
}: MetricCardsProps) {
  return (
    <div className="metrics-grid">
      <div className="metric-card">
        <span className="metric-label">TICK INDEX</span>
        <span className="metric-value">{tick}</span>
      </div>
      <div className="metric-card">
        <span className="metric-label">ELAPSED TIME</span>
        <span className="metric-value">{elapsedDays} d</span>
      </div>
      <div className="metric-card">
        <span className="metric-label">STEP RESOLUTION</span>
        <span className="metric-value">{stepHours} h</span>
      </div>
      <div className="metric-card">
        <span className="metric-label">NETWORK NODES</span>
        <span className="metric-value">{nodeCount}</span>
      </div>
      <div className="metric-card">
        <span className="metric-label">SIMULATION TPS</span>
        <span className="metric-value">{tps.toFixed(1)}</span>
      </div>
      <div className="metric-card">
        <span className="metric-label">COMPUTE LATENCY</span>
        <span className="metric-value">{computeLatencyMs.toFixed(1)} ms</span>
      </div>
    </div>
  );
}
