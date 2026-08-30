# Core Architecture Rules

> **File:** `.antigravity/rules/00-core-architecture.md`  
> **Scope:** Repository-wide architectural boundaries, layer separation, data contracts, domain knowledge grounding, and dependency constraints.

---

## 🏛️ 1. Layer Separation & Decoupling

All codebase modules must strictly adhere to clean architectural layer boundaries. Higher layers may depend on lower layers, but lower layers must never import or depend on higher layers.

```
+-------------------------------------------------------------+
| Presentation / UI Layer (Components, CLI handlers, Views)   |
+-------------------------------------------------------------+
                              |
                              v
+-------------------------------------------------------------+
| Application / Service Layer (Use cases, Workflows, Services)|
+-------------------------------------------------------------+
                              |
                              v
+-------------------------------------------------------------+
| Domain / Core Logic Layer (Entities, Pure calculation logic)|
+-------------------------------------------------------------+
                              ^
                              |
+-------------------------------------------------------------+
| Infrastructure / I/O Layer  (DB, Network, FileSystem, APIs) |
+-------------------------------------------------------------+
```

### 1.1 Domain / Core Logic Layer
- **Pure Business Logic:** Contains core domain entities, business validation, and mathematical/computational pipelines.
- **Zero I/O Dependencies:** The domain layer must be 100% free of external side effects, file system access, network I/O, database drivers, or UI presentation libraries.
- **Portability & Determinism:** Domain models and transformation logic must execute deterministically in any environment (browser, Node.js, worker, CLI).

### 1.2 Application / Service Layer
- Coordinates workflows, use cases, and cross-domain interactions.
- Orchestrates calls between the domain layer and infrastructure adapters.
- Never directly mutates UI state or embeds framework-specific rendering logic.

### 1.3 Infrastructure / I/O Layer
- Implements external communication (HTTP clients, file system persistence, third-party SDKs, database drivers).
- All infrastructure access must satisfy interfaces defined by the application or domain layers (Dependency Inversion Principle).
- Encapsulates network retry logic, serialization, and telemetry.

### 1.4 Presentation / UI Layer
- Handles rendering, user interaction, layout, and event dispatching.
- Must remain a thin projection over application state.
- Strictly forbidden from executing raw business logic or bypass-querying infrastructure directly.

---

## 📐 2. Typed Public Interfaces & Strict Contracts

1. **Strict Type Safety:**
   - Every module, function, and service must expose strictly typed public interfaces.
   - Never use `any` types. Use generics, discriminated unions, and specific type contracts.
   - Unknown external data must be validated at boundaries before entering core domain logic.

2. **Boundary Validation:**
   - All external inputs (API payloads, file reads, environment variables, user inputs) must pass through schema validation at the I/O ingress.
   - Data crossing layer boundaries must adhere to immutable data transfer objects (DTOs) or domain models.

3. **Public Interface Immutability:**
   - Established public API signatures, schemas, and exported contracts must not be altered without auditing all dependent invocation sites and updating associated test suites.

---

## ⚡ 3. Deterministic & Pure Function Pipelines

1. **Pure State Transformations:**
   - Calculation pipelines, state reducers, and data transformations must be deterministic, pure functions.
   - Given the same input arguments, a pure function must always return the exact same output.
   - Do not read or mutate shared global state within calculation pipelines.

2. **Isolate Non-Determinism:**
   - System time (`Date.now()`, `new Date()`), random generators (`Math.random()`), and environment variables must be passed explicitly as parameters or injected via interfaces, never sampled directly within domain calculations.

3. **Zero LaTeX / Plain-Text Mathematical Standards:**
   - Mathematical expressions, variables, units, and rates in code comments, schemas, and documentation must strictly use clean plain text, ASCII, or inline code notation.
   - **STRICT PROHIBITION:** Never use LaTeX formatting, dollar sign delimiters (`$...$`, `$$...$$`), or LaTeX escape sequences (`\dot{}`, `\frac{}{}`, `\approx`, `\Omega`, `\varpi`).
   - Standard notation examples:
     - `a = a0 + a_dot * T`
     - `P = P_base * (1 + k * (D - S) / S)`
     - `eps = v^2 / 2 - mu / r`

---

## 🔒 4. Dependency Governance & Third-Party Restrictions

1. **Unauthorized Dependencies Banned:**
   - No third-party packages or libraries may be installed without explicit architectural review and plan approval.
   - Always evaluate standard library or existing internal utilities before introducing external dependencies.

2. **Supply Chain & Bundle Discipline:**
   - Every external dependency must be evaluated for footprint size, licensing compliance, security posture, and tree-shakeability.
   - Wrapper adapters must encapsulate external dependencies so that vendor lock-in is isolated to the infrastructure layer.

---

## 📖 5. Domain Knowledge Grounding & Downward Reference Rule

1. **Authoritative Domain Truth (`docs/references/`):**
   - Formulas, physical constants, decision trees, baseline figures, and known hallucination traps must be formalized in `docs/references/REF-XXX.md`.
   - These documents serve as the immutable Tier 0 ground truth for the system.

2. **Strict Downward Dependency (Zero Upward References):**
   - Reference documents (`REF-XXX.md`) must be 100% self-contained and pure domain knowledge.
   - **PROHIBITION:** Reference files must NEVER contain upward references, pointers, or imports to specific codebase file paths (`src/...`).
   - Code, docstrings, and active plans (`PLAN-XXX.md`) must cite downward to `[REF-XXX](file:///docs/references/REF-XXX.md)`. This ensures that codebase refactoring never breaks or dirties the authoritative domain specifications.

3. **Golden Vector Verification:**
   - Unit tests covering domain logic must assert against the golden benchmark vectors recorded in the cited `REF-XXX` document.
