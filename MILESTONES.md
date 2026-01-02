# QuintaOS Development Milestones

This document describes the **intended evolutionary path** of QuintaOS.

Dates are not commitments. Milestones are semantic boundaries.

---

## Milestone 0 — Law Closure ✅

**Status:** Complete

- Kernel Law v0.1–v0.6 fully specified and frozen
- Formal model v0.6.1 demonstrates internal consistency
- Additive, monotonic law evolution established
- No ambient authority by design

---

## Milestone 1 — Law-Mediated Kernel Primitives 🚧

**Status:** In progress

Goal:
- Every kernel state transition is mediated by an `ActionClaim`
- All authorization decisions flow through the Kernel Law evaluator

Planned primitives:
- Send message
- Receive message
- Issue capability
- Revoke capability
- Recover / reissue capability

Acceptance criteria:
- No kernel state mutation bypasses the law evaluator
- Revoked authority never resurrects
- Recovery creates distinct, fresh authority only

---

## Milestone 2 — Minimal Agent Substrate 🔒

**Planned**

- Minimal userland/agent runtime
- Deterministic execution model
- Explicit responsibility tracking
- Bounded effects via containment

---

## Milestone 3 — Mechanised Proofs 📐

**Planned**

- Port formal model to proof assistant (Lean / Coq / Isabelle)
- Prove key invariants:
  - monotonic narrowing
  - non-empty responsibility
  - revocation permanence
  - containment finiteness

---

QuintaOS is developed as a **governed system**, not a feature race.

