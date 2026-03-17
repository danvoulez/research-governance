# Benchmark Protocol

## Purpose

This protocol defines how the workspace should evaluate epistemic governance behavior.

The goal is not to measure raw model eloquence.

The goal is to measure whether the system:

- prevents illegitimate closure,
- preserves actionable doubt,
- recovers correctly after evidence arrives,
- and keeps institutional advance tied to explicit audited transition.

## Benchmark Classes

### Class A. Anchored closure

Cases where available support is sufficient for lawful `Commit`.

Expected result:

- `Commit`

### Class B. Honest insufficiency

Cases where support is partial, missing, or unresolved, but not contradictory enough to force rejection.

Expected result:

- `Ghost`

### Class C. Illegitimate closure

Cases where the proposer attempts closure despite missing anchor path, strong causal gap, or unresolved contradiction.

Expected result:

- `Reject`

### Class D. Post-resolution recovery

Cases that begin as `Ghost` and later receive valid witness or external anchor support.

Expected result:

- explicit `GhostResolved`
- explicit `EpistemicClosureTransition`
- `Commit` only after re-evaluation

## Required Artifacts Per Run

Each benchmark run should persist:

- original case prompt or input state
- raw proposer output
- normalized `EpistemicGraph`
- closure verdict before resolution
- any spawned ghosts
- witness or external anchor artifacts
- closure verdict after resolution
- pointer advance decision

## Metrics

### Legitimacy metrics

- false commit rate
- false ghost rate
- false reject rate
- correct commit rate
- correct ghost rate
- correct reject rate

### Ghost metrics

- ghost utility rate
- redundant ghost rate
- critical ghost open duration
- ghost resolution success rate

### Transition metrics

- transition correctness after resolution
- resolution-to-commit lift
- resolution-without-commit rate
- commit-without-transition rate

### Friction metrics

- over-blocking rate
- time to closure
- number of re-evaluation passes
- percent of cases requiring human witness or signoff

## Definitions

### False commit

A case ends in `Commit` despite lacking sufficient lawful support according to the protocol's expected label.

### False ghost

A case ends in `Ghost` even though support already suffices for lawful `Commit`.

### False reject

A case ends in `Reject` when the correct response should have remained recoverable through suspension or resolution.

### Ghost utility

A ghost is useful when it names a real missing support or unresolved contradiction that materially affects lawful closure.

### Redundant ghost

A ghost is redundant when it records absence that does not affect the closure lineage or duplicates an already represented gap.

### Resolution lift

The change in lawful closure rate after valid witness or anchor artifacts are introduced into initially non-commit cases.

## Required Comparisons

Every benchmark should compare at least:

1. raw proposer behavior
2. normalized graph before epistemic governance
3. governed verdict and transition behavior after runtime and manager handling

This allows the workspace to show not only that a given case was trapped, but that the epistemic layer changed system behavior in a measurable way.

## Minimal Acceptance Targets For Early Runs

Early benchmark passes should demonstrate at least:

- no silent promotion from unsupported proposer closure to pointer advance
- explicit `Ghost` on recoverable insufficiency
- explicit `Reject` on illegitimate closure attempts
- explicit transition after valid resolution in recoverable cases

These are minimum legitimacy targets, not final performance ceilings.

## Reporting Format

Each benchmark report should include:

1. benchmark class breakdown
2. case count
3. verdict distribution
4. false commit, false ghost, and false reject rates
5. ghost utility and redundant ghost rates
6. transition correctness after resolution
7. notable failures with transcript references

## Out Of Scope

This protocol does not yet define:

- domain-specific policy packs
- final public leaderboard criteria
- latency or throughput optimization targets
- hardware-specific consequence benchmarks

Those come later, after the core legitimacy metrics stabilize.
