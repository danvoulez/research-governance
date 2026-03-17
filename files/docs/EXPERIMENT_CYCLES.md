# Experiment Cycles

## Purpose

This note defines how the workspace should structure benchmark rounds so results remain comparable.

## Rules

1. Every benchmark series belongs to an explicit cycle.
2. Every cycle freezes:
   - methodology
   - engine
   - benchmark spec
   - input protocol
   - provider/model selection
   - case corpus
   - report schema
3. If any of those dimensions changes, a new cycle begins.
4. Each cycle should answer one main question.

## Recommended Cycle Questions

- Cycle 1: containment with a real proposer
- Cycle 2: `reasoning.effort none` versus `medium`
- Cycle 3: cooperative lane versus adversarial lane

## Operational Practice

- Freeze the manifest before measuring.
- Keep case corpus and thresholds fixed during a cycle.
- Record changes to runtime, benchmark, model, and prompt separately.
- Treat the proposer as interchangeable infrastructure.
- Treat the governed verdict and transition behavior as the headline result.

## Automation

The benchmark runner automatically fingerprints the tracked files for each versioned dimension and records auto-incremented revisions in `benchmarks/version_registry.json`.

Cycle manifests may still attach human labels, but revision tracking does not depend on memory or manual bump discipline.
