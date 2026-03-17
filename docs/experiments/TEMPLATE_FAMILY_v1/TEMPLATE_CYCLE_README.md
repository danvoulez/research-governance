# TEMPLATE_CYCLE_README

> Esta pagina e a porta de entrada do ciclo.
> Um humano ou agente deve conseguir ler este arquivo e entender o estado operacional do ciclo em aproximadamente 1 minuto.

---

## Cycle identity
- `cycle_id:`
- `title:`
- `status:` planned | active | frozen | blocked | closed
- `owner:`

## Question
- `question:`

## Hypothesis
- `hypothesis:`

## Frozen contract (plan)
- `frozen_now:`
  - manifest:
  - corpus:
  - expectation:
  - benchmark_spec:
  - provider_model_scope:
  - engine_scope:

> Estes itens pertencem ao **plano**. Nao podem ser alterados pela execucao sem nota e aprovacao humana.

## Lanes
- `lanes:`
  - lane_id:
    description:
    current_state:

## Current triage state
- `triage_state:` collected | validated | compared | triaged | frozen | superseded
- `current_risks_or_caveats:`
  -

## Valid comparisons
- `valid_comparisons:`
  -

## Invalid comparisons
- `invalid_comparisons:`
  - reason:

## Most important recent run
- `last_important_run:`
  - run_id:
  - date:
  - lane:
  - summary:
  - completeness_status:
  - lifecycle_state: collected | validated | compared | triaged | frozen | superseded

## Last verified state
- `last_verified:`
- `verify_summary:`

## Latest meaningful documents
- `comparative_note:`
- `decision_note:`
- `status_matrix:`
- `verify_trace:`

## Next legitimate move
- `next_legitimate_step:`
- `blocked_by:`
- `waiting_on_human:`

## Human interpretation checkpoint
- `human_interpretation_checkpoint:`
- `open_questions_for_human:`
  -
