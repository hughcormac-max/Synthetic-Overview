import type {
  SimulationStateDto,
  ResourceDto,
  StorageDto,
  ConverterDto,
  FlowEdgeDto,
} from "../types/simulation.js";

/**
 * Generates the canonical 40-resource directory:
 * - 10 Raw Resources (IDs 1..=10)
 * - 20 Intermediate Goods (IDs 11..=30)
 * - 10 Consumer Goods (IDs 31..=40)
 */
export function createDefaultResources(): ResourceDto[] {
  const resources: ResourceDto[] = [];
  for (let i = 1; i <= 10; i++) {
    resources.push({ id: i, name: `Raw Resource ${i}` });
  }
  for (let i = 1; i <= 20; i++) {
    resources.push({ id: 10 + i, name: `Intermediate Good ${i}` });
  }
  for (let i = 1; i <= 10; i++) {
    resources.push({ id: 30 + i, name: `Consumer Good ${i}` });
  }
  return resources;
}

interface GeneratedScenario {
  readonly storages: StorageDto[];
  readonly converters: ConverterDto[];
  readonly edges: FlowEdgeDto[];
}

/**
 * Deterministically generates the 200 converters, 400 storages, and 250 flow edges
 * mirroring the synthetic-core Rust generator.
 */
export function generateDeterministicScenario(): GeneratedScenario {
  const storages: StorageDto[] = [];
  const converters: ConverterDto[] = [];
  const edges: FlowEdgeDto[] = [];

  let entityIdCounter = 100;
  const outputsByResource = new Map<number, number[]>();
  const inputsByResource = new Map<number, number[]>();

  const spawnStorage = (resourceId: number, amount: number, capacity: number): number => {
    const id = entityIdCounter++;
    storages.push({
      entity_id: id,
      resource_id: resourceId,
      amount,
      capacity,
    });
    return id;
  };

  // 1. 50 Mines (Recipes 1..=10, 5 instances each)
  for (let recipeId = 1; recipeId <= 10; recipeId++) {
    for (let inst = 0; inst < 5; inst++) {
      const convId = entityIdCounter++;
      const outStorageId = spawnStorage(recipeId, 20_000_000, 1_000_000_000);
      converters.push({ entity_id: convId, recipe_id: recipeId });

      const list = outputsByResource.get(recipeId) ?? [];
      list.push(outStorageId);
      outputsByResource.set(recipeId, list);
    }
  }

  // 2. 60 Intermediate Factories (Recipes 11..=30, 3 instances each)
  for (let j = 0; j < 20; j++) {
    const recipeId = 11 + j;
    const in1 = (j % 10) + 1;
    const in2 = ((j + 1) % 10) + 1;

    for (let inst = 0; inst < 3; inst++) {
      const convId = entityIdCounter++;
      const inStorage1 = spawnStorage(in1, 50_000_000, 1_000_000_000);
      const inStorage2 = spawnStorage(in2, 50_000_000, 1_000_000_000);
      const outStorage = spawnStorage(recipeId, 20_000_000, 1_000_000_000);

      converters.push({ entity_id: convId, recipe_id: recipeId });

      const inList1 = inputsByResource.get(in1) ?? [];
      inList1.push(inStorage1);
      inputsByResource.set(in1, inList1);

      const inList2 = inputsByResource.get(in2) ?? [];
      inList2.push(inStorage2);
      inputsByResource.set(in2, inList2);

      const outList = outputsByResource.get(recipeId) ?? [];
      outList.push(outStorage);
      outputsByResource.set(recipeId, outList);
    }
  }

  // 3. 40 Consumer Factories (Recipes 31..=40, 4 instances each)
  for (let k = 0; k < 10; k++) {
    const recipeId = 31 + k;
    const in1 = 11 + ((k * 2) % 20);
    const in2 = 11 + ((k * 2 + 1) % 20);

    for (let inst = 0; inst < 4; inst++) {
      const convId = entityIdCounter++;
      const inStorage1 = spawnStorage(in1, 50_000_000, 1_000_000_000);
      const inStorage2 = spawnStorage(in2, 50_000_000, 1_000_000_000);
      const outStorage = spawnStorage(recipeId, 20_000_000, 1_000_000_000);

      converters.push({ entity_id: convId, recipe_id: recipeId });

      const inList1 = inputsByResource.get(in1) ?? [];
      inList1.push(inStorage1);
      inputsByResource.set(in1, inList1);

      const inList2 = inputsByResource.get(in2) ?? [];
      inList2.push(inStorage2);
      inputsByResource.set(in2, inList2);

      const outList = outputsByResource.get(recipeId) ?? [];
      outList.push(outStorage);
      outputsByResource.set(recipeId, outList);
    }
  }

  // 4. 50 Population Centers (Recipes 41..=50, 5 instances each)
  for (let m = 0; m < 10; m++) {
    const recipeId = 41 + m;
    const consumerId = 31 + m;

    for (let inst = 0; inst < 5; inst++) {
      const convId = entityIdCounter++;
      const inStorage = spawnStorage(consumerId, 50_000_000, 1_000_000_000);
      converters.push({ entity_id: convId, recipe_id: recipeId });

      const inList = inputsByResource.get(consumerId) ?? [];
      inList.push(inStorage);
      inputsByResource.set(consumerId, inList);
    }
  }

  // 5. 250 Directed Flow Edges
  let edgeCounter = 1000;
  for (let resId = 1; resId <= 40; resId++) {
    const outs = outputsByResource.get(resId) ?? [];
    const ins = inputsByResource.get(resId) ?? [];

    if (outs.length === 0 || ins.length === 0) continue;

    for (let idx = 0; idx < outs.length; idx++) {
      const src = outs[idx];
      const dst = ins[idx % ins.length];
      if (src !== undefined && dst !== undefined) {
        edges.push({
          edge_id: edgeCounter++,
          source_id: src,
          destination_id: dst,
          in_transit: 5_000_000,
        });
      }
    }
  }

  return { storages, converters, edges };
}

/**
 * Creates the complete baseline simulation state with full 200 nodes and 40 resources.
 */
export function createDefaultSimulationState(): SimulationStateDto {
  const scenario = generateDeterministicScenario();
  return {
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
    resources: createDefaultResources(),
    storages: scenario.storages,
    converters: scenario.converters,
    edges: scenario.edges,
    astro_nodes: [
      {
        entity_id: 1,
        body_id: 1,
        radius_km: 6371,
        h3_resolution: 3,
      },
      {
        entity_id: 2,
        body_id: 2,
        radius_km: 3389,
        h3_resolution: 2,
      },
    ],
    surface_nodes: [
      {
        entity_id: 1001,
        parent_body_id: 1,
        h3_cell_index: "83194afffffffff",
      },
      {
        entity_id: 1002,
        parent_body_id: 1,
        h3_cell_index: "832f5afffffffff",
      },
      {
        entity_id: 1003,
        parent_body_id: 1,
        h3_cell_index: "832a10fffffffff",
      },
      {
        entity_id: 1004,
        parent_body_id: 1,
        h3_cell_index: "83be0efffffffff",
      },
      {
        entity_id: 1005,
        parent_body_id: 1,
        h3_cell_index: "837a6efffffffff",
      },
      {
        entity_id: 1006,
        parent_body_id: 2,
        h3_cell_index: "82502ffffffffff",
      },
    ],
  };
}

/**
 * Advances the local browser-mode simulation state by one tick.
 */
export function stepBrowserSimulation(prev: SimulationStateDto): SimulationStateDto {
  const nextTick = prev.tick + 1;
  const nextTimestamp = prev.timestamp_seconds + prev.delta_time_seconds;

  // Step orbital ephemeris
  const entities = prev.entities.map((e) => ({
    ...e,
    true_anomaly: (e.true_anomaly + 0.05) % (2.0 * Math.PI),
  }));

  // Step storages: simulate slight production and consumption variance
  const storages = prev.storages.map((s) => {
    let delta = 0;
    if (s.resource_id <= 10) {
      // Raw extraction
      delta = (s.entity_id % 3 === 0 ? 1 : -1) * 2_000_000;
    } else if (s.resource_id <= 30) {
      // Intermediate conversion
      delta = (s.entity_id % 2 === 0 ? 1 : -1) * 1_500_000;
    } else {
      // Consumer / sink
      delta = (s.entity_id % 4 === 0 ? 1 : -1) * 1_000_000;
    }
    const newAmount = Math.max(0, Math.min(s.capacity, s.amount + delta));
    return { ...s, amount: newAmount };
  });

  // Step edges: fluctuate in-transit packets
  const edges = prev.edges.map((edge) => {
    const wave = Math.sin(nextTick * 0.2 + edge.edge_id) * 2_000_000;
    const inTransit = Math.max(0, Math.round(5_000_000 + wave));
    return { ...edge, in_transit: inTransit };
  });

  return {
    ...prev,
    tick: nextTick,
    timestamp_seconds: nextTimestamp,
    entities,
    storages,
    edges,
    astro_nodes: prev.astro_nodes,
    surface_nodes: prev.surface_nodes,
  };
}
