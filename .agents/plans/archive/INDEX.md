# Historical Plan Ledger

> **Location:** `.agents/plans/archive/INDEX.md`
> **Purpose:** Master index and historical record of all completed, superseded, and abandoned architectural plans.

---

## 📚 Archive Directory Structure

Archived plans are categorized by year under `.agents/plans/archive/YYYY/PLAN-XXX.md`.

```text
.agents/plans/archive/
├── INDEX.md                 # Master historical ledger (this file)
└── 2026/
    └── PLAN-001.md          # Completed / archived plan record
```

---

## 🗂️ Master Plan Index

| Plan ID | Title | Status | Author | Created | Completed | Archive Path |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| [PLAN-001](2026/PLAN-001.md) | Define Tech Stack, Engine, and System Architecture | `completed` | Antigravity | 2026-09-12 | 2026-09-12 | [2026/PLAN-001.md](2026/PLAN-001.md) |
| [PLAN-002](2026/PLAN-002.md) | Scaffold Core ECS and Node/Processor Network using bevy_ecs | `completed` | Antigravity | 2026-09-12 | 2026-09-12 | [2026/PLAN-002.md](2026/PLAN-002.md) |
| [PLAN-003](2026/PLAN-003.md) | Scaffold H3 Spatial Grid for Astronomical Bodies | `completed` | Antigravity | 2026-09-12 | 2026-09-12 | [2026/PLAN-003.md](2026/PLAN-003.md) |
| [PLAN-004](2026/PLAN-004.md) | Scaffold Initial Economic Simulation Scenario and IPC DTOs | `completed` | Antigravity | 2026-09-13 | 2026-09-13 | [2026/PLAN-004.md](2026/PLAN-004.md) |
| [PLAN-005](2026/PLAN-005.md) | Scaffold Surface Layer Visualisation UI/UX Pipeline | `completed` | Antigravity | 2026-09-13 | 2026-09-13 | [2026/PLAN-005.md](2026/PLAN-005.md) |
| [PLAN-006](2026/PLAN-006.md) | Pivot UI/Rendering Stack to Bevy | `completed` | Antigravity | 2026-09-13 | 2026-09-13 | [2026/PLAN-006.md](2026/PLAN-006.md) |
| [PLAN-007](2026/PLAN-007.md) | Restructure and Standardize SSOT Library | `completed` | Antigravity | 2026-09-13 | 2026-09-16 | [2026/PLAN-007.md](2026/PLAN-007.md) |
| [PLAN-008](2026/PLAN-008.md) | Re-architect SSOT for SI Units, Astrodynamics, and Topography | `completed` | Antigravity | 2026-09-13 | 2026-09-16 | [2026/PLAN-008.md](2026/PLAN-008.md) |
| [PLAN-009](2026/PLAN-009.md) | Implement WebGPU Rendered Surface View with Backface Culling | `completed` | Antigravity | 2026-09-14 | 2026-09-16 | [2026/PLAN-009.md](2026/PLAN-009.md) |
| [PLAN-010](2026/PLAN-010.md) | Implement Interactive Orbit & Zoom Camera Controls for Planetary Globe View | `completed` | Antigravity | 2026-09-16 | 2026-09-16 | [2026/PLAN-010.md](2026/PLAN-010.md) |
| [PLAN-011](2026/PLAN-011.md) | Transition SSOT to f64 Math & Unified 3D Coordinates | `completed` | Antigravity | 2026-09-17 | 2026-09-17 | [2026/PLAN-011.md](2026/PLAN-011.md) |

---

## 📝 Archival Workflow Instructions

When completing, superseding, or abandoning an active plan:

1. Ensure all test criteria in the plan are met and verified.
2. Prompt for and log any architectural deviations in Section 5 of the plan.
3. Update the YAML frontmatter:

   ```yaml
   status: completed # completed | superseded | abandoned
   completed_at: YYYY-MM-DD
   ```

4. Move the file from `.agents/plans/active/PLAN-XXX.md` to `.agents/plans/archive/YYYY/PLAN-XXX.md`.
5. Add a new entry to the table above.
