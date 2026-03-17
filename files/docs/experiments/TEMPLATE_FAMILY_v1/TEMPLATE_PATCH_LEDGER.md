# TEMPLATE_PATCH_LEDGER

> Registrar aqui cada tranche de mudanca de codigo, configuracao ou artefato metodologicamente relevante.
> O ledger deve ser curto, estruturado e auditavel. Nao depende de narrativa longa.

---

## Patch entry
- `patch_id:`
- `related_cycle:`
- `related_session_id:`
- `opened_at:`
- `closed_at:`
- `status:` proposed | in_progress | applied | reverted | superseded | rejected

## Trace metadata
- `trace_id:`
- `parent_session_id:`
- `step_id:`
- `tool_call_id:`

## Hypothesis of patch
- `hypothesis_of_patch:`
- `expected_effect:`

## Touched files
- `touched_files:`
  - path:
    change_type: created | modified | deleted

## Affected dimension
- `affected_dimension:` engine | harness | adapter | benchmark_spec | corpus | expectation | docs | tests | report_schema | other
- `version_impact:` new_run | new_lane | revision_bump | new_cycle | none

## Comparison context
- `comparison_target:`
- `delta_vs_baseline:`
- `actual_effect:`

## Evidence that motivated patch
- `evidence_that_motivated_patch:`
  - artifact_or_signal:
    source:
    note:

## Input and output artifacts
- `input_artifacts:`
  -
- `output_artifacts:`
  -

## Execution and resulting status
- `commands_or_actions:`
  -
- `resulting_status:`
- `followup_required:` yes | no
- `followup_note:`

## Methodological impact
- `rollback_needed:` yes | no
- `invalidates_previous_runs:` yes | no
- `requires_new_baseline:` yes | no

## Human review
- `requires_human_review:` yes | no
- `human_review_reason:`
- `decision_blocked_by:`
- `escalation_reason:`
