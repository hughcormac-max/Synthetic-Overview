# System Architecture

> **Document:** `docs/ARCHITECTURE.md`
> **Status:** Living Document | **Master Reference:** [AGENTS.md](../AGENTS.md)

---

## 🏛️ 1. Architectural Philosophy & Overview

This project is built around **strict layer decoupling**, **contract-first interface design**, **deterministic computation pipelines**, and **authoritative domain knowledge grounding**. By isolating domain logic from external side effects and anchoring code to immutable domain specifications, the codebase achieves maximum portability, high testability, and zero-hallucination reproducibility.

---

## 📐 2. Layer Topology & Separation of Concerns

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

## 📚 5. Architecture Decision Records (ADRs) & Knowledge Vault

- Domain Knowledge & Specifications: [docs/ssot/INDEX.md](ssot/INDEX.md)
- Research Reports Ledger: [docs/research/INDEX.md](research/INDEX.md)
- Architectural Decision Records: [.agents/plans/archive/INDEX.md](../.agents/plans/archive/INDEX.md)
