# Security Policy

## Status

QuintaOS is a **research-stage system**.

At this stage:
- The kernel implementation is incomplete.
- No production, safety, or security guarantees are claimed.
- The formal model specifies authority semantics, not full system security.

**Do not deploy QuintaOS in production or security-critical environments.**

---

## Reporting Vulnerabilities

If you believe you have identified a security-relevant issue related to:
- authority enforcement
- capability handling
- revocation or recovery semantics
- containment or blast radius violations

please report it privately by opening a **GitHub Security Advisory** or contacting the repository owner directly.

Public disclosure before discussion is discouraged at this stage.

---

## Scope of Security Reasoning

The following are within scope for discussion and review:
- Kernel Law semantics (v0.1–v0.6)
- Law enforcement architecture
- Authority provenance, causality, and revocation models

The following are **explicitly out of scope** at this stage:
- Side-channel resistance
- Hardware attacks
- Memory safety hardening
- Scheduler fairness
- Device or driver security

These areas may be addressed in future milestones.

