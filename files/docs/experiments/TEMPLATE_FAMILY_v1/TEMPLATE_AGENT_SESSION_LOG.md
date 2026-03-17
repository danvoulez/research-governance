# TEMPLATE_AGENT_SESSION_LOG

> Preencher para cada sessao relevante de operacao de agente dentro de um ciclo.
> Abrir um session log no inicio de uma sessao que possa tocar codigo, manifests, corpus, docs governados ou artefatos de run.
> Fechar o session log ao encerrar a janela de trabalho ou ao escalar uma decisao sem resolucao.

---

## Session metadata
- `session_id:`
- `cycle_id:`
- `opened_at:`
- `closed_at:`
- `status:` open | closed | escalated | aborted
- `agent_or_tool_identity:`
- `operator_context:` local IDE | CI | scripted automation | other

## Trace metadata
- `trace_id:`
- `parent_session_id:`
- `step_id:`
- `tool_call_id:`

## Objective
- `objective:`
- `requested_by:` human | automation | system ritual
- `related_hypothesis_or_task:`

## Scope
- `expected_scope:`
- `out_of_scope_guardrail_notes:`

## Input artifacts
- `input_artifacts:`
  - artifact:
    source:
    role: baseline | corpus | manifest | config | other

## Files touched
- `files_touched:`
  - path:
    action: created | modified | read | deleted
    relevance:

## Commands run
- `commands_run:`
  - command:
    purpose:
    result: success | failed | partial

## Output artifacts
- `output_artifacts:`
  - artifact:
    type: run | doc_draft | verification | patch | other
    location:
    status:

## Decisions and escalations
- `decisions_taken_within_scope:`
  -
- `blocked_decisions:`
  - decision:
    blocked_by:
    impact:
- `unresolved_decisions_escalated_to_human:`
  - issue:
    why_escalated:
    escalation_reason:
    blocking: yes | no

## Integrity notes
- `manifest_used:`
- `freeze_state_seen:`
- `baseline_reference_if_any:`
- `did_methodology_change:` yes | no
- `if_yes_explain:`

## Closing summary
- `summary_of_work:`
- `next_legitimate_action:`
- `handoff_notes:`
