# System Architecture

> **Document:** `docs/ARCHITECTURE.md`  
> **Status:** Living Document | **Master Reference:** [AGENTS.md](file:///AGENTS.md)

---

## 🏛️ 1. Architectural Philosophy & Overview

This project is built around **strict layer decoupling**, **contract-first interface design**, **deterministic computation pipelines**, and **authoritative domain knowledge grounding**. By isolating domain logic from external side effects and anchoring code to immutable domain specifications, the codebase achieves maximum portability, high testability, and zero-hallucination reproducibility.

---

## 📐 2. Layer Topology & Separation of Concerns

```
+-------------------------------------------------------------------+
| Tier 0: Domain Grounding Vault (docs/references/REF-XXX.md)       |
|         - Formulas, constants, logic loops, truth tables          |
|         - Anti-hallucination traps & golden test vectors          |
|         - ZERO upward knowledge of source code or file paths      |
+-------------------------------------------------------------------+
                                  ^
                                  | (Downward Citation Dependency)
+-------------------------------------------------------------------+
| 1. Presentation / UI Layer                                        |
|    - User interfaces, CLI commands, HTTP controllers              |
|    - Strictly presentation logic & event dispatching              |
+-------------------------------------------------------------------+
                                  |
                                  v
+-------------------------------------------------------------------+
| 2. Application / Service Layer                                    |
|    - Workflow orchestration, use case coordinators                |
|    - Manages domain entity transactions & adapter routing         |
+-------------------------------------------------------------------+
                                  |
                                  v
+-------------------------------------------------------------------+
| 3. Domain / Core Logic Layer (Pure Business Logic)                |
|    - Pure calculation pipelines, domain entities, validators      |
|    - ZERO dependencies on UI, database, network, or file system  |
|    - Implements logic citing Tier 0 references                    |
+-------------------------------------------------------------------+
                                  ^
                                  | (Dependency Inversion)
+-------------------------------------------------------------------+
| 4. Infrastructure / I/O Adapter Layer                             |
|    - Database clients, HTTP clients, file system adapters         |
|    - Implements interfaces defined by Application/Domain layers   |
+-------------------------------------------------------------------+
```

### 2.1 Layer Responsibilities

| Layer | Responsibility | Allowed Inbound Dependencies | Prohibited Inbound Dependencies |
| :--- | :--- | :--- | :--- |
| **Tier 0 Grounding** | Immutable facts, formulas, constants, logic loops | None (Pure domain truth) | Upward links to `src/...` code files |
| **Presentation** | User interaction, rendering, CLI input handling | Application, Domain Types | Infrastructure directly |
| **Application** | Use case coordination, workflow orchestration | Domain, Infrastructure Interfaces | UI rendering logic |
| **Domain** | Pure business rules, math/state pipelines | None (Cites Tier 0 references) | UI, DB, Network, Filesystem |
| **Infrastructure** | Concrete I/O, external network, persistence | Domain Interfaces, Application Contracts | Core domain entity definitions |

---

## 🔄 3. Data Flow & State Management

1. **Unidirectional Data Ingress & Egress:**
   - Ingress data enters via Presentation or Infrastructure controllers.
   - External payloads are validated at the perimeter via schemas (e.g. Zod, Pydantic, Serde).
   - Validated data transfer objects (DTOs) are passed into Application Services.
   - Core calculation engines receive immutable domain structures, returning new immutable state.

2. **Deterministic Pipelines:**
   - All state transformations, data mapping, and calculation pipelines are implemented as pure, deterministic functions.
   - Non-deterministic factors (timestamps, random seeds, environment configs) must be passed explicitly as arguments or injected via adapters.

3. **Mathematical Notation Standards:**
   - All mathematical formulations and variable representations in documentation and code comments follow ASCII/plain-text standards (Strict Zero-LaTeX policy).
   - Example: `P = P_base * (1 + k * (D - S) / S)`.

---

## 🛡️ 4. Cross-Cutting Concerns

### 4.1 Error Handling & Result Types
- Operational errors must use explicit discriminated union / `Result<T, E>` patterns rather than untyped exceptions.
- Domain errors must carry clear context without leaking internal stack traces or secrets to users.

### 4.2 Security & Credential Isolation
- Secrets, API keys, and environment variables are strictly injected via the Infrastructure configuration layer and never hardcoded in Domain or Presentation layers.

---

## 📚 5. Architecture Decision Records (ADRs) & References

- Domain Knowledge & Specifications: [docs/references/INDEX.md](file:///docs/references/INDEX.md)
- Architectural Decision Records: [.antigravity/plans/archive/index.md](file:///.antigravity/plans/archive/index.md)
