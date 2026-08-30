# Verification & Quality Assurance Rules

> **File:** `.antigravity/rules/02-verification.md`  
> **Scope:** Testing mandates, regression enforcement, definition of done, and subagent verification protocols.

---

## 🧪 1. Mandatory Unit Testing & Definition of Done

1. **No Task Complete Without Tests:**
   - No feature, bug fix, refactor, or architectural change is marked complete without passing automated unit tests.
   - Code changes and their corresponding unit tests must be committed together.

2. **Comprehensive Coverage Requirements:**
   - **Nominal Paths:** Verify expected outputs for standard valid inputs.
   - **Edge Cases:** Test boundary conditions (e.g., zero values, empty collections, maximum sizes, boundary thresholds).
   - **Error Handling:** Verify that invalid inputs, malformed data, and network errors trigger expected domain error responses.

3. **Deterministic & Isolated Tests:**
   - Unit tests must be hermetic and fast, executing in any order without inter-test dependencies or side effects.
   - Mock external I/O, timers, or random generators to ensure 100% deterministic test results.

---

## 🔄 2. Mandatory Regression Runs

1. **Full Scope Regression Testing:**
   - Whenever a module or interface is modified, the test suite for that module and all dependent upstream and downstream modules must be executed.
   - Any broken existing test constitutes a blocker that must be resolved before proceeding.

2. **Validation Pipeline:**
   - Before finishing any task or step, execute the standard verification triad:
     1. **Typecheck:** Clean compilation with zero type errors.
     2. **Lint:** Zero linting errors or style warnings.
     3. **Test Suite:** 100% passing tests with zero regressions.

---

## 📋 3. Subagent Verification Checklist

Prior to closing or archiving an active plan (`PLAN-XXX.md`), every agent and subagent must systematically verify the following checklist:

- [ ] **1. Type Check:** Project compiles with zero type errors (`strict: true`).
- [ ] **2. Unit Tests:** All unit tests for modified and newly created modules pass.
- [ ] **3. Regression Tests:** All existing regression test suites pass without failure.
- [ ] **4. Contract Adherence:** Implemented code strictly matches the technical contracts and API signatures approved in `PLAN-XXX.md`.
- [ ] **5. Zero LaTeX Verification:** All documentation, code comments, and strings contain **zero LaTeX math delimiters** (`$...$`, `$$...$$`, `\dot{}`, `\approx`, etc.) and adhere to ASCII/plain-text standards.
- [ ] **6. Plan Checklist Synchronized:** All implementation checkboxes in `.antigravity/plans/active/PLAN-XXX.md` are completed.
- [ ] **7. Clean Git Status:** No temporary scratch files, debug logging statements, or unintended artifacts remain in the workspace.

