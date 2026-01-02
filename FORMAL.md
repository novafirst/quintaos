# QuintaOS Formal Verification Outline — v0.6.1 (Consistency Closure)

Status: Non-normative. Descriptive.
Authority: In all cases, LAW.md prevails.

------------------------------------------------------------

1. Purpose

This document provides a formal model of the QuintaOS Kernel Laws
from v0.1 through v0.6.

Its goals are to:

- Demonstrate internal logical consistency of the law stack
- Prevent semantic drift during implementation
- Enable mechanised verification (Coq, Lean, Isabelle/HOL)
- Serve as a reference model for kernel law evaluation

This document does not grant authority.
It explains and constrains authority defined elsewhere.

------------------------------------------------------------

2. Scope

The formal model covers only authority semantics as defined by the Kernel Laws.

Included:
- Entities
- Capabilities
- Rights
- Actions
- Law predicates and their ordering

Explicitly excluded:
- Scheduling
- IPC mechanisms
- Storage
- Devices and drivers
- Memory models
- Performance or liveness guarantees

All excluded mechanisms must comply with the laws, but are not formalised here.

------------------------------------------------------------

3. Ontology

3.1 Primitive Sets

- Entity — locus of authority
- Capability — explicit grant of authority
- Right — atomic permission
- Action — attempted state transition

3.2 Basic Relations

subject(capability) -> Entity
object(capability) -> Entity
rights(capability) -> Set(Right)
parent(capability) -> Option(Capability)
revoked(capability) -> Bool
expired(capability) -> Bool
issuer_exists(capability) -> Bool

------------------------------------------------------------

4. Law v0.1 — Permission

Predicate:

Permitted_v0_1(action) :=
    has_capability(action) AND
    right_matches(action) AND
    subject_matches(action) AND
    object_matches(action)

Semantics:

- Authority is explicit.
- No implicit or ambient authority exists.
- If any predicate is unevaluable, permission fails.

------------------------------------------------------------

5. Law v0.2 — Continuity & Revocation

Predicate:

Permitted_v0_2(action) :=
    Permitted_v0_1(action) AND
    Continuous(capability)

Continuity Conditions:

A capability is continuous if and only if:
- It is known to have existed
- It is not revoked
- It is not expired
- Its issuer exists
- All ancestors are continuous

Silence Is Denial:
Any unevaluable continuity predicate yields denial.

------------------------------------------------------------

6. Law v0.3 — Provenance & Intent

Predicate:

Permitted_v0_3(action) :=
    Permitted_v0_2(action) AND
    ProvenanceOK(capability) AND
    IntentOK(capability, action)

Provenance Requirements:
- Determinable origin
- Finite ancestry chain
- Acyclic ancestry
- Kernel-rooted origin

Intent Requirements:
- Explicit declaration exists
- Intent is binding and restrictive
- Intent permits the requested action

------------------------------------------------------------

7. Law v0.4 — Causality & Responsibility

Predicate:

Permitted_v0_4(action) :=
    Permitted_v0_3(action) AND
    CausalChainFinite(capability) AND
    ResponsibilityNonEmpty(action)

Semantics:
- Every action has a finite causal chain
- Responsibility follows authority
- Responsibility cannot be delegated away
- The responsible entity set is non-empty

------------------------------------------------------------

8. Law v0.5 — Containment & Blast Radius

Predicate:

Permitted_v0_5(action) :=
    Permitted_v0_4(action) AND
    WithinContainment(action)

Semantics:
- Every capability declares containment
- Containment bounds effects spatially or logically
- Blast radius is finite
- Actions may not escape declared bounds

------------------------------------------------------------

9. Law v0.6 — Recovery & Reissuance

Predicate:

Permitted_v0_6(action) :=
    Permitted_v0_5(action) AND
    RecoveryValid(capability)

Recovery Rules:
- Revoked authority never returns
- Recovery creates new, distinct authority
- Reissued authority must satisfy v0.1–v0.5
- No implicit lineage is permitted

------------------------------------------------------------

10. Ordering and Monotonicity

For all actions:

Permitted_v0_(n+1)(action) IMPLIES Permitted_v0_n(action)

Later laws only restrict authority.
They never widen it.

------------------------------------------------------------

11. Proven Theorems

- Monotonic Narrowing
- Revocation Permanence
- Acyclic Ancestry
- Finite Causal Chains
- Non-Empty Responsibility
- Containment Finiteness

------------------------------------------------------------

12. Consistency Witness

A constructive model exists such that:
- All axioms hold
- All predicates are satisfiable
- No contradictions arise

Therefore, Kernel Laws v0.1–v0.6 are internally consistent.

------------------------------------------------------------

13. Closing Statement

This document explains why the Kernel Laws cohere.

It does not create authority.
It does not override the constitution.

LAW.md remains supreme.

