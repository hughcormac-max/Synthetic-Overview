# Coding Standards & Quality Guidelines

> **File:** `.antigravity/rules/01-coding-standards.md`  
> **Scope:** Repository-wide code formatting, naming conventions, immutability patterns, error handling, and modularity.

---

## 🏷️ 1. Naming Conventions

All identifiers must be descriptive, concise, intention-revealing, and follow strict casing conventions:

| Entity Type | Convention | Example |
| :--- | :--- | :--- |
| **Types, Interfaces, Classes, Enums** | `PascalCase` | `CalculationPipeline`, `SessionState`, `LogLevel` |
| **Functions, Methods, Variables** | `camelCase` | `validatePayload`, `computeMetrics`, `retryCount` |
| **Constants & Static Enums** | `SCREAMING_SNAKE_CASE` | `MAX_PAYLOAD_BYTES`, `DEFAULT_TIMEOUT_MS` |
| **Files & Directories** | `kebab-case` | `pipeline-runner.ts`, `data-contracts.ts` |
| **Boolean Flags & Predicates** | `is*`, `has*`, `can*`, `should*` | `isValid`, `hasCompleted`, `canRetry` |

- **Avoid Ambiguous Abbreviations:** Use full words (`request`, `response`, `transaction`) rather than truncated tokens (`req`, `res`, `tx`) in public APIs and domain models.
- **Single Source of Truth for Enums/Constants:** Avoid magic numbers and strings; define them as exported constants or union types.

---

## 🔒 2. Immutability Defaults

1. **Immutable by Default:**
   - Prefer immutable data structures. Mark properties, arrays, and collections as readonly (`readonly`, `ReadonlyArray<T>`, `Object.freeze()`, frozen dataclasses).
   - Variables must default to `const` rather than `let` (and never `var`).

2. **State Updates via Pure Transformations:**
   - Transform data by creating new state instances (e.g., using structural cloning or object spread `{ ...state, field: newValue }`) rather than mutating existing objects in place.
   - Arrays must be transformed via non-mutating operations (`map`, `filter`, `reduce`, `concat`, `slice`) instead of mutating operations (`push`, `pop`, `splice`, `reverse`).

3. **Defensive Copies at Boundaries:**
   - If an internal structure must be exposed, return a deep/shallow copy or an immutable view to prevent downstream mutations from corrupting internal state.

---

## 🛡️ 3. Explicit & Robust Error Handling

1. **Domain-Specific Error Types:**
   - Define explicit domain errors or result types (`Result<T, E>`) for expected failure modes rather than throwing untyped generic exceptions.
   - Distinguish between expected operational failures (e.g. invalid input, resource not found) and unexpected system bugs (e.g. memory corruption, invariant violations).

2. **Zero Swallowed Errors:**
   - Never write empty `catch` blocks or discard errors silently.
   - Always log, rethrow with contextual wrapping, or return an explicit failure result.

3. **Fail Fast at Boundaries:**
   - Validate invariants and input payloads immediately upon entry.
   - Throw or return descriptive errors before executing computational logic.

4. **Secret Safety in Error Messages:**
   - Ensure error messages, telemetry, and stack traces never expose sensitive tokens, passwords, or personal identifiable information (PII).

---

## 🧱 4. Modular Structure & Unit Sizing

1. **Single Responsibility Principle (SRP):**
   - Each module and class must have exactly one well-defined responsibility and reason to change.
   - Avoid monolithic files: individual source files should aim to remain under 250-300 lines of code.

2. **Small, Testable Units:**
   - Functions should focus on a single operation (aim for under 30-40 lines per function).
   - Extract helper routines into pure, isolated utility functions to enable independent unit testing.

3. **Explicit Exports & Clean Surface Area:**
   - Only export symbols intended for public consumption.
   - Keep internal helpers scoped privately within the module.

