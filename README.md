# QuintaOS

**A constitutionally governed microkernel with a formally specified authority system.**

## Overview

QuintaOS is **not** a Unix-like operating system.

It is a capability-based computational substrate designed to safely host autonomous intelligence under strict, explicit governance.

If you are looking for:
- A shell
- A filesystem
- User accounts
- POSIX APIs

This project is **not** for you.

QuintaOS is built on a foundation of immutable **Kernel Laws** that govern all authority within the system. These laws define *when actions may exist* and *how authority is created, constrained, revoked, and recovered*.

Authority is never implicit.  
Silence is always denial.  
The kernel is the sole evaluator of existence.

---

## Important Status Note

QuintaOS is a **research-stage system**.

- **Kernel Laws v0.1–v0.6** are fully specified, additive, and frozen.
- The **formal model** demonstrates internal consistency of these laws.
- The **kernel implementation is incomplete** and not yet fully verified.
- No production, safety, or security guarantees are claimed at this stage.

This repository exists to make the **constitutional design explicit, reviewable, and resistant to semantic drift**.

---

## Key Documents

### LAW.md
The **normative constitutional foundation** of QuintaOS.

- Contains **Kernel Law v0.1 — Genesis State** only.
- Immutable and authoritative.
- Defines the minimal conditions under which an action may exist.

Later laws (**v0.2–v0.6**) are defined as additive, frozen specifications under `kernel/src/law/`.

---

### FORMAL.md
**Formal Verification Outline — Version 0.6.1 (Consistency Closure)**

A non-normative formal model covering Kernel Law v0.1–v0.6.

Purpose:
- Make the law stack machine-verifiable
- Enable proof-assistant integration (Coq, Lean, Isabelle/HOL)
- Ensure monotonicity, consistency, and resistance to interpretive drift

In case of conflict, **LAW.md prevails**.

---

## Kernel Laws Summary

| Version | Focus                      | Core Guarantee |
|-------:|----------------------------|----------------|
| v0.1 | Permission | Explicit capability matching |
| v0.2 | Continuity & Revocation | Authority must remain live |
| v0.3 | Provenance & Intent | Authority has origin and purpose |
| v0.4 | Causality & Responsibility | Responsibility is non-empty |
| v0.5 | Containment & Blast Radius | Effects are bounded |
| v0.6 | Recovery & Reissuance | Revocation is permanent |

---

## Scope of Formalisation

The formal model covers **only authority semantics**.

It excludes:
- Scheduling
- IPC mechanics
- Storage
- Devices
- Concrete kernel implementations

All mechanisms must comply with the laws but are not formalised here.

---

## License

Apache License 2.0

---

**QuintaOS replaces ambiguity with provable constraints on authority, responsibility, and containment.**

