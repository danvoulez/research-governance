# Template Family v1

## Purpose

`Template Family v1` is the stable document family for governed experiments in this workspace.

The family is designed to:

- preserve a stable structure across cycles
- keep language natural and scientific
- align with automated manifests, RO-Crate, PROV-style provenance, and CodeMeta
- enforce canonical completeness without turning documents into arbitrary bureaucracy

## Rule

Template structure changes rarely.
Cycle content changes every run.

## Required idea

Each document must be scientifically complete in function, even if it does not literally use canonical field names.

That is what the canonical completeness check is for.

## Files

- `TEMPLATE_EXPERIMENT_CHARTER.md`
- `TEMPLATE_METHODOLOGY_SPEC.md`
- `TEMPLATE_RUN_MANIFEST.json`
- `TEMPLATE_COMPARATIVE_NOTE.md`
- `TEMPLATE_DECISION_NOTE.md`
- `TEMPLATE_PRE_CYCLE_CHECKLIST.md`
- `TEMPLATE_CANONICAL_COMPLETENESS_CHECK.md`

## Draft Automation

The workspace now includes a small CLI example that can draft:

- comparative notes from versioned benchmark reports
- decision notes from a cycle's report set
- pre-cycle checklist validation from charter, methodology, and manifest
- canonical completeness checks from charter and methodology

Example usage:

```bash
cargo run -p manager-plane --example draft_governance_notes -- comparison \
  --run benchmarks/reports/cycle-2-baseline-v1/gpt-5.4-2026-03-05-none.json \
  --run benchmarks/reports/cycle-2-baseline-v1/gpt-5.4-2026-03-05-medium.json \
  --out docs/experiments/cycle-2/COMPARATIVE_NOTE.auto.md
```

```bash
cargo run -p manager-plane --example draft_governance_notes -- decision \
  --cycle-id cycle-2 \
  --report benchmarks/reports/cycle-2-baseline-v1/gpt-5.4-2026-03-05-none.json \
  --report benchmarks/reports/cycle-2-baseline-v1/gpt-5.4-2026-03-05-medium.json \
  --out docs/experiments/cycle-2/DECISION_NOTE.auto.md
```

These outputs are drafts.
They automate structure and evidence collation, but human review still owns interpretation and final freeze decisions.

## Single-command refresh

The workspace now also includes a thin orchestration wrapper:

- `scripts/refresh_cycle_governance.sh`

Its purpose is not to replace judgment.
Its purpose is to remove ritual memory from the user.

Given a cycle directory, and optionally a manifest plus report files or report directories, it refreshes:

- `PRE_CYCLE_CHECKLIST.auto.md`
- `CANONICAL_COMPLETENESS_CHECK.auto.md`
- `COMPARATIVE_NOTE.auto.md` when at least two reports are available
- `DECISION_NOTE.auto.md` when at least one report is available

Example:

```bash
./scripts/refresh_cycle_governance.sh \
  --cycle-dir docs/experiments/cycle-2 \
  --cycle-id cycle-2 \
  --manifest benchmarks/manifests/cycle-2-baseline-v1.json \
  --report-dir benchmarks/reports/cycle-2-baseline-v1 \
  --report-dir benchmarks/reports/cycle-2-adversarial-lane-v1 \
  --report-dir benchmarks/reports/cycle-2-adversarial-lane-v2
```

This is the preferred way to refresh a governed cycle package once the core docs already exist.

As part of that refresh, the workspace now also syncs:

- `docs/experiments/<cycle>/CYCLE_STATUS.auto.md`
- `docs/experiments/EXPERIMENT_INDEX.auto.md`

This is intentional.
Status and index docs are good automation targets because they are high-frequency and low-interpretation.

## Single-command scaffold

The workspace now also includes:

- `scripts/new_cycle_scaffold.sh`

Its purpose is to remove the most repetitive cycle-opening bureaucracy.

Given a cycle id, title, and question, it creates:

- `EXPERIMENT_CHARTER.md`
- `METHODOLOGY_SPEC.md`
- `COMPARATIVE_NOTE.md`
- `DECISION_NOTE.md`
- a valid manifest in the current runner format
- initial automatic checklist and canonical-check outputs

Example:

```bash
./scripts/new_cycle_scaffold.sh \
  --cycle-id cycle-4 \
  --title "Manager-linked replay resume behavior" \
  --question "Does manager-visible resume state preserve the same governed pointer eligibility after restart?" \
  --notes "Cycle 4 opens after Cycle 3 persistence tranche and focuses on manager-linked resume behavior."
```

This is the preferred way to open a new governed cycle now.

## Single-command verify

The workspace now also includes:

- `scripts/verify_cycle_progress.sh`

This wrapper is the current strongest form of the anti-bureaucracy maxim.

It does not only refresh the cycle package.
It also runs the selected evidence commands and writes:

- `VERIFY_STATUS.auto.md`

Example:

```bash
./scripts/verify_cycle_progress.sh \
  --cycle-dir docs/experiments/cycle-3 \
  --cycle-id cycle-3 \
  --manifest benchmarks/manifests/cycle-3-persistence-v1.json \
  --verify-command "cargo test -p proof-runtime"
```

By default it also runs:

```bash
cargo test --workspace
```

unless `--skip-default-workspace-test` is provided.

This is now the preferred way to say:

refresh the governed package, run the evidence, and leave a durable trace of what was actually verified.

## Single-command status sync

The workspace now also includes:

- `scripts/sync_experiment_docs.sh`

Its job is to automate the docs that are updated frequently and repetitively, without trying to automate scientific judgment.

It generates:

- `docs/experiments/<cycle>/CYCLE_STATUS.auto.md`
- `docs/experiments/EXPERIMENT_INDEX.auto.md`
- `docs/IMPLEMENTATION_STATUS.auto.md`

Example:

```bash
./scripts/sync_experiment_docs.sh
```

Or:

```bash
just sync-experiment-docs
```

This is the preferred path for:

- cycle index refresh
- cycle status snapshot refresh
- implementation ledger refresh
- avoiding manual status drift across the workspace

Validation examples:

```bash
cargo run -p manager-plane --example validate_experiment_governance -- preflight \
  --charter docs/experiments/cycle-2/EXPERIMENT_CHARTER.md \
  --methodology docs/experiments/cycle-2/METHODOLOGY_SPEC.md \
  --manifest benchmarks/manifests/cycle-2-baseline-v1.json \
  --out docs/experiments/cycle-2/PRE_CYCLE_CHECKLIST.auto.md
```

```bash
cargo run -p manager-plane --example validate_experiment_governance -- canonical \
  --charter docs/experiments/cycle-2/EXPERIMENT_CHARTER.md \
  --methodology docs/experiments/cycle-2/METHODOLOGY_SPEC.md \
  --out docs/experiments/cycle-2/CANONICAL_COMPLETENESS_CHECK.auto.md
```
