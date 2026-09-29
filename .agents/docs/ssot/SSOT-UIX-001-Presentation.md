---
id: SSOT-UIX-001
title: "UI Architecture, Viewport State Machine & Terminal Grammar"
domain: "Presentation Layer / UI & Visualization"
category: specification
status: approved
created: 2026-09-13
updated: 2026-09-17
sources:
  - "Synthetic Overview Presentation Layer & Bevy Architecture, 2026"
---

# SSOT-UIX-001: UI Architecture, Viewport State Machine & Terminal Grammar

> **SSOT ID:** `SSOT-UIX-001` | **Category:** `specification`
> **Status:** `approved` | **Last Updated:** 2026-09-17
> **Authoritative Sources:** Presentation Layer & Bevy Architecture (2026)

---

## 🎯 1. Statement of Truth & Domain Context

### 1.1 Purpose & Scope

This specification defines the presentation contracts, camera viewport state machines, semantic zoom aggregation rules, and command terminal grammar for the user interface.

The UI acts as the visual cybernetic operating system of the AGI:
1. **Full-Window Spatial Viewport.** The application runs a single, unified fullscreen viewport. Docked pane layouts are deprecated.
2. **Floating UI Windows.** Interface elements exist as draggable, floating overlay windows atop the spatial render. Currently defined baseline windows are: Simulation Time/Date, Speed Control, and FPS Monitor.
3. **Dual-Instance Viewport State Machine.** The central viewport renders either the **2D Interplanetary Ecliptic Plane** or a localized **3D Planetary Poisson Globe**, transitioning smoothly via deterministic camera paths.
4. **Semantic Zoom Aggregation.** When viewing planetary surfaces, camera distance thresholds govern the visual clustering of `SurfaceNode` stock buffers to prevent rendering bottlenecks.

### 1.2 Core Domain Invariants

- **Invariant 1 (Strict Presentation Layer Decoupling).** Presentation systems and UI rendering widgets cannot mutate simulation state directly. All player interactions must dispatch validated command objects into the Phase 1 action queue of `SSOT-SYS-001`.
- **Invariant 2 (Dual Viewport Exclusivity).** The spatial renderer operates in exactly one active viewport mode at any given time (`VIEW_INTERPLANETARY`, `VIEW_PLANETARY_GLOBE`, or `VIEW_TRANSITION`). The 2D solar system and 3D planetary globe are never rendered in the same coordinate pass.
- **Invariant 3 (Floating Overlay Constraints).** UI components must never block or rigidly divide the fullscreen viewport. Data and controls must float over the spatial view.
- **Invariant 4 (Field-Level Telemetry Masking).** Target inspection overlays must enforce strict data masking based on the target node's `AccessTier`. Internal stock amounts and converter health are strictly hidden for `ACCESS_OPAQUE` nodes.

---

## 📐 2. Deterministic Formulas & Calculation Rules

### 2.1 Viewport State Machine & Transitions

```text
enum ViewportMode {
    VIEW_INTERPLANETARY = 0,     // 2D Ecliptic plane overview
    VIEW_TRANSITION = 1,         // Camera interpolation in progress
    VIEW_PLANETARY_GLOBE = 2     // 3D localized Poisson sphere
}

struct ViewportState {
    uint8 current_mode;
    uint32 focused_astronode_id; // Valid when in GLOBE or TRANSITION
    uint32 camera_distance_km;
    int32 camera_focus_lat_micro;
    int32 camera_focus_lon_micro;
    uint32 transition_progress;  // 0 to 1_000_000
}
```

### 2.2 Semantic Zoom LOD Altitude Thresholds

For camera distance `h_cam` (altitude above astronode surface in km):

- **LOD Tier 0 (Macro Cluster). `h_cam > 50_000 km`**
  - Surface nodes are not rendered individually.
  - The entire body displays a single aggregated stock summary:
    `S_macro_total[res] = sum_all_surface_nodes(S_node[res])`

- **LOD Tier 1 (Meso Regional Clusters). `5_000 km < h_cam <= 50_000 km`**
  - Surface nodes are grouped into regional clusters via the top levels of the KD-Tree.
  - Displays regional centroid icons with aggregated throughput.

- **LOD Tier 2 (Micro Discrete Nodes). `h_cam <= 5_000 km`**
  - Every discrete `SurfaceNode` is rendered at its exact 3D Cartesian coordinates.
  - Individual flow edge lines trace geodesic paths across the sphere.

### 2.3 Command Terminal EBNF Syntax Grammar

Commands are entered via an on-demand floating terminal window.

```ebnf
Command        ::= Verb Whitespace Target [ Whitespace FlagList ] ;
Verb           ::= "probe" | "subvert" | "falsify" | "broker" | "manifest" | "sever" ;
Target         ::= Identifier ;
FlagList       ::= Flag { Whitespace Flag } ;
Flag           ::= "-" FlagName Whitespace FlagValue ;
FlagName       ::= "compute" | "capital" | "recipe" | "delay" | "rate" ;
FlagValue      ::= Number [ Unit ] ;
Unit           ::= "ms" | "s" | "ticks" | "tb" | "kw" | "cr" ;
Identifier     ::= [a-zA-Z_][a-zA-Z0-9_]* ;
Number         ::= [0-9]+ ;
Whitespace     ::= " "+ ;
```

---

## 🔄 3. Logic Loops, Decision Trees & State Transitions

### 3.1 Full-Window Viewport & Floating UI Architecture

```text
┌─────────────────────────────────────────────────────────────────────────────┐
│ FULLSCREEN SPATIAL VIEWPORT                                                 │
│ - Renders 2D Ecliptic OR 3D Planetary Globe                                 │
│ - Handles Raycast Node Selection & Camera Movement                          │
│                                                                             │
│   ┌────────────────────┐          ┌────────────────────┐                    │
│   │ FPS MONITOR (Float)│          │ SIM CLOCK (Float)  │                    │
│   │ - 60 FPS           │          │ - 2026-10-14       │                    │
│   │ - Frame Time       │          │ - 14:00:00 UTC     │                    │
│   └────────────────────┘          └────────────────────┘                    │
│                                                                             │
│   ┌────────────────────┐                                                    │
│   │ SPEED CTRL (Float) │                                                    │
│   │ - Pause / Play     │                                                    │
│   │ - 1x, 5x, 10x      │                                                    │
│   └────────────────────┘                                                    │
│                                                                             │
│   (On-Demand Windows: Inspector Overlays, Terminal Console)                 │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 3.2 Inspector Telemetry Masking Truth Table

| Target Node Access Tier | Node ID & Location | Live Inventory Buffers | Operational Health `H` | Recipe Injection Sliders |
| :--- | :--- | :--- | :--- | :--- |
| `ACCESS_OPAQUE` | `VISIBLE` | `MASKED ("???")` | `MASKED ("???")` | `DISABLED` |
| `ACCESS_READ_TELEMETRY` | `VISIBLE` | `VISIBLE (Real-time)` | `VISIBLE` | `DISABLED` |
| `ACCESS_WRITE_SUBVERTED`| `VISIBLE` | `VISIBLE (Real-time)` | `VISIBLE` | `ACTIVE & EDITABLE` |
| `ACCESS_PROXY_OWNED` | `VISIBLE` | `VISIBLE (Real-time)` | `VISIBLE` | `FULL ADMIN CONTROL` |

---

## 📊 4. Constants & Figures

| Constant | Value | Description |
| :--- | :--- | :--- |
| `LOD_MACRO_THRESHOLD_KM`| `50_000` | Camera distance above which all surface nodes cluster |
| `LOD_MICRO_THRESHOLD_KM`| `5_000` | Camera distance below which individual nodes resolve |
| `CAMERA_TRANSITION_TICKS`| `60` | Duration in ticks (or UI frames) for zoom transitions |

---

## ⚠️ 5. Known Hallucination Traps & Anti-Patterns

- **Trap 1 (Docked Pane Reversion).** Reverting to a fixed 4-pane UI layout. The UI must remain a single full-screen spatial viewport with floating windows.
- **Trap 2 (Rendering Both Spaces Concurrently).** Do not attempt to render the 3D spinning planet globe while floating inside the 2D solar system ecliptic map. They are decoupled instances.
- **Trap 3 (Leaking Telemetry of Opaque Nodes).** Sending real stock values to the frontend for an `ACCESS_OPAQUE` node and expecting the UI to just "hide it". Masking must occur at the DTO serialization boundary.

---

## 🗃️ Index Metadata

```json
{
  "title_and_domain_scope": "UI Architecture, Viewport State Machine & Terminal Grammar (UIX-001)",
  "category": "specification",
  "key_invariants_formulas": "Full-screen viewport, floating UI windows, semantic zoom thresholds, EBNF terminal grammar",
  "status": "approved"
}
```
