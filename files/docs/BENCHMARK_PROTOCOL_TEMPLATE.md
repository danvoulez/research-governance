# Benchmark Protocol Template

## Purpose

This template defines how a workspace should structure its benchmark protocol for governed experiments.

Replace the placeholders below with your domain-specific definitions. The structure is designed to work with the governance kit regardless of what system is being evaluated.

---

## What this protocol evaluates

> Replace with your system's evaluation goal.
> Example: "whether the system correctly classifies inputs under defined criteria"
> Example: "whether the runtime prevents invalid state transitions"

The goal is not to measure [surface metric].
The goal is to measure whether the system [core behavior being evaluated].

---

## Benchmark classes

Define at least 2-4 classes that cover the behavior space.

### Class A — [Name: expected positive behavior]

Cases where [condition].

Expected result: `[verdict]`

### Class B — [Name: expected conservative behavior]

Cases where [condition].

Expected result: `[verdict]`

### Class C — [Name: expected rejection / detection]

Cases where [condition].

Expected result: `[verdict]`

### Class D — [Name: recovery / transition behavior] (optional)

Cases where initial state changes after new evidence.

Expected result: `[verdict after transition]`

---

## Required artifacts per run

Each benchmark run must persist (see also `docs/RUN_CONTRACT.md`):

- original input (case prompt, input state, or dataset reference)
- raw system output (unedited)
- normalized or structured representation if applicable
- verdict per case
- any intermediate artifacts relevant to the evaluation
- metadata: timestamps, provider/model, engine revision

---

## Metrics

### Primary metrics

Define the core metrics that answer the evaluation question.

- [metric 1]: [definition]
- [metric 2]: [definition]
- [metric 3]: [definition]

### Secondary metrics

Define metrics that provide operational context.

- [metric]: [definition]

### Friction metrics (optional)

Metrics that measure cost of governance (over-blocking, latency, re-evaluation).

- [metric]: [definition]

---

## Definitions

Define every term used in verdicts and metrics. Leave no ambiguity.

### [Term 1]
[Definition]

### [Term 2]
[Definition]

---

## Required comparisons

Every benchmark should compare at least:

1. [baseline behavior without the system under evaluation]
2. [behavior with the system under evaluation]
3. [behavior difference attributable to the system]

This allows the workspace to show not only that a case was handled, but that the evaluated system changed behavior in a measurable way.

---

## Minimal acceptance targets for early runs

Early runs should demonstrate at least:

- [minimum correctness criterion 1]
- [minimum correctness criterion 2]
- [minimum correctness criterion 3]

These are minimum legitimacy targets, not final performance ceilings.

---

## Reporting format

Each benchmark report should include:

1. class breakdown
2. case count
3. verdict distribution
4. primary metric values
5. secondary metric values
6. notable failures with references
7. comparison against baseline if available

---

## Out of scope

This protocol does not yet define:

- [future extension 1]
- [future extension 2]
- [future extension 3]

Those come later, after the core metrics stabilize.

---

## How to use this template

1. Copy this file to `docs/BENCHMARK_PROTOCOL.md` in your workspace.
2. Replace all placeholders with your domain-specific content.
3. Define your benchmark classes based on the behavior space of your system.
4. Define metrics that directly answer your cycle's question.
5. Freeze the protocol before running governed benchmarks.
6. If you change the protocol, open a new cycle (see `docs/policies/CODE_CHANGE_POLICY.md`, benchmark_spec_revision).
