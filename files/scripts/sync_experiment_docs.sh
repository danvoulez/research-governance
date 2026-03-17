#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'EOF'
usage:
  ./scripts/sync_experiment_docs.sh

What it does:
  1. generates docs/experiments/<cycle>/CYCLE_STATUS.auto.md for each governed cycle
  2. generates docs/experiments/EXPERIMENT_INDEX.auto.md as a workspace-wide cycle index
  3. generates docs/IMPLEMENTATION_STATUS.auto.md as an operational ledger

Notes:
  - This script automates status, index, and ledger docs only.
  - It does not replace human comparative, decision, or interpretation writing.
EOF
}

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
EXPERIMENTS_DIR="${REPO_ROOT}/docs/experiments"
DOCS_DIR="${REPO_ROOT}/docs"

trim() {
  local value="$1"
  value="${value#"${value%%[![:space:]]*}"}"
  value="${value%"${value##*[![:space:]]}"}"
  printf '%s' "${value}"
}

extract_field() {
  local file="$1"
  local field="$2"
  if [[ ! -f "${file}" ]]; then
    return 0
  fi
  local line
  line="$(grep -m1 "^- ${field}:" "${file}" || true)"
  line="${line#- ${field}:}"
  trim "${line}"
}

extract_first_quote() {
  local file="$1"
  if [[ ! -f "${file}" ]]; then
    return 0
  fi
  local line
  line="$(grep -m1 '^> ' "${file}" || true)"
  line="${line#> }"
  trim "${line}"
}

extract_verify_status() {
  local file="$1"
  if [[ ! -f "${file}" ]]; then
    printf '%s' "not_run"
    return 0
  fi
  extract_field "${file}" "status"
}

render_cycle_status() {
  local cycle_dir="$1"
  local cycle_id
  cycle_id="$(basename "${cycle_dir}")"

  local charter="${cycle_dir}/EXPERIMENT_CHARTER.md"
  local methodology="${cycle_dir}/METHODOLOGY_SPEC.md"
  local comparative="${cycle_dir}/COMPARATIVE_NOTE.md"
  local decision="${cycle_dir}/DECISION_NOTE.md"
  local next_step="${cycle_dir}/NEXT_STEP_NOTE.md"
  local preflight="${cycle_dir}/PRE_CYCLE_CHECKLIST.auto.md"
  local canonical="${cycle_dir}/CANONICAL_COMPLETENESS_CHECK.auto.md"
  local verify="${cycle_dir}/VERIFY_STATUS.auto.md"
  local out="${cycle_dir}/CYCLE_STATUS.auto.md"

  local title charter_status methodology_status comparative_status decision_status
  local question decision_quote verify_status checklist_ready canonical_status

  title="$(extract_field "${charter}" "title")"
  charter_status="$(extract_field "${charter}" "status")"
  methodology_status="$(extract_field "${methodology}" "status")"
  comparative_status="$(extract_field "${comparative}" "status")"
  decision_status="$(extract_field "${decision}" "status")"
  question="$(extract_first_quote "${charter}")"
  decision_quote="$(extract_first_quote "${decision}")"
  verify_status="$(extract_verify_status "${verify}")"
  checklist_ready="$(grep -q '\[x\] This cycle is ready to run' "${preflight}" && printf 'ready' || printf 'not_confirmed')"
  canonical_status="$(extract_field "${canonical}" "status")"

  {
    echo "# Cycle Status"
    echo
    echo "## Identity"
    echo "- cycle_id: ${cycle_id}"
    echo "- title: ${title:-unknown}"
    echo "- charter_status: ${charter_status:-unknown}"
    echo "- methodology_status: ${methodology_status:-unknown}"
    echo "- comparative_status: ${comparative_status:-missing}"
    echo "- decision_status: ${decision_status:-missing}"
    echo "- verify_status: ${verify_status:-not_run}"
    echo
    echo "## Question"
    if [[ -n "${question}" ]]; then
      echo "> ${question}"
    else
      echo "> question not available"
    fi
    echo
    echo "## Current governed posture"
    if [[ -n "${decision_quote}" ]]; then
      echo "> ${decision_quote}"
    else
      echo "> no decision note statement available"
    fi
    echo
    echo "## Auto checks"
    echo "- pre_cycle_checklist: ${checklist_ready}"
    echo "- canonical_completeness: ${canonical_status:-missing}"
    echo "- verification_trace: ${verify_status:-not_run}"
    echo
    echo "## Core docs"
    echo "- [EXPERIMENT_CHARTER.md](./EXPERIMENT_CHARTER.md)"
    echo "- [METHODOLOGY_SPEC.md](./METHODOLOGY_SPEC.md)"
    if [[ -f "${comparative}" ]]; then
      echo "- [COMPARATIVE_NOTE.md](./COMPARATIVE_NOTE.md)"
    fi
    if [[ -f "${decision}" ]]; then
      echo "- [DECISION_NOTE.md](./DECISION_NOTE.md)"
    fi
    if [[ -f "${next_step}" ]]; then
      echo "- [NEXT_STEP_NOTE.md](./NEXT_STEP_NOTE.md)"
    fi
    if [[ -f "${verify}" ]]; then
      echo "- [VERIFY_STATUS.auto.md](./VERIFY_STATUS.auto.md)"
    fi
  } > "${out}"

  return 0
}

render_experiment_index() {
  local out="${EXPERIMENTS_DIR}/EXPERIMENT_INDEX.auto.md"

  {
    echo "# Experiment Index"
    echo
    echo "## Purpose"
    echo "This is an automatically generated index of governed experiment cycles."
    echo
    echo "It is meant to reduce manual status-sync work."
    echo
    echo "| Cycle | Title | Charter | Decision | Verify | Notes |"
    echo "|---|---|---|---|---|---|"

    local cycle_dir cycle_id title charter_status decision_status verify_status note
    while IFS= read -r -d '' cycle_dir; do
      [[ -f "${cycle_dir}/EXPERIMENT_CHARTER.md" ]] || continue
      cycle_id="$(basename "${cycle_dir}")"
      title="$(extract_field "${cycle_dir}/EXPERIMENT_CHARTER.md" "title")"
      charter_status="$(extract_field "${cycle_dir}/EXPERIMENT_CHARTER.md" "status")"
      decision_status="$(extract_field "${cycle_dir}/DECISION_NOTE.md" "status")"
      verify_status="$(extract_verify_status "${cycle_dir}/VERIFY_STATUS.auto.md")"
      note="$(extract_first_quote "${cycle_dir}/DECISION_NOTE.md")"
      echo "| [${cycle_id}](./${cycle_id}/CYCLE_STATUS.auto.md) | ${title:-unknown} | ${charter_status:-unknown} | ${decision_status:-missing} | ${verify_status:-not_run} | ${note:-no decision statement} |"
    done < <(find "${EXPERIMENTS_DIR}" -maxdepth 1 -type d -name 'cycle-*' -print0 | sort -z)
  } > "${out}"
}

render_implementation_status_auto() {
  local out="${DOCS_DIR}/IMPLEMENTATION_STATUS.auto.md"
  local cycle_count=0
  local frozen_count=0
  local partial_freeze_count=0
  local verify_pass_count=0

  while IFS= read -r -d '' cycle_dir; do
    [[ -f "${cycle_dir}/EXPERIMENT_CHARTER.md" ]] || continue
    cycle_count=$((cycle_count + 1))

    local charter_status decision_status verify_status
    charter_status="$(extract_field "${cycle_dir}/EXPERIMENT_CHARTER.md" "status")"
    decision_status="$(extract_field "${cycle_dir}/DECISION_NOTE.md" "status")"
    verify_status="$(extract_verify_status "${cycle_dir}/VERIFY_STATUS.auto.md")"

    if [[ "${charter_status}" == "frozen" || "${decision_status}" == "frozen" ]]; then
      frozen_count=$((frozen_count + 1))
    fi
    if [[ "${charter_status}" == *"frozen"* || "${decision_status}" == "partial_freeze" ]]; then
      partial_freeze_count=$((partial_freeze_count + 1))
    fi
    if [[ "${verify_status}" == "PASS" ]]; then
      verify_pass_count=$((verify_pass_count + 1))
    fi
  done < <(find "${EXPERIMENTS_DIR}" -maxdepth 1 -type d -name 'cycle-*' -print0 | sort -z)

  {
    echo "# Implementation Status (Auto)"
    echo
    echo "## Purpose"
    echo "This file is generated from current workspace state."
    echo
    echo "It exists to reduce manual status-sync work."
    echo
    echo "Use [IMPLEMENTATION_STATUS.md](./IMPLEMENTATION_STATUS.md) for the strategic, human-written narrative."
    echo
    echo "## Workspace ledger"
    echo "- governed_cycles_detected: ${cycle_count}"
    echo "- frozen_cycles_detected: ${frozen_count}"
    echo "- cycles_with_frozen_tranche_detected: ${partial_freeze_count}"
    echo "- cycles_with_pass_verify_trace: ${verify_pass_count}"
    echo
    echo "## Automation surfaces detected"
    if [[ -x "${REPO_ROOT}/scripts/new_cycle_scaffold.sh" ]]; then
      echo "- new_cycle_scaffold: present"
    fi
    if [[ -x "${REPO_ROOT}/scripts/refresh_cycle_governance.sh" ]]; then
      echo "- refresh_cycle_governance: present"
    fi
    if [[ -x "${REPO_ROOT}/scripts/verify_cycle_progress.sh" ]]; then
      echo "- verify_cycle_progress: present"
    fi
    if [[ -x "${REPO_ROOT}/scripts/sync_experiment_docs.sh" ]]; then
      echo "- sync_experiment_docs: present"
    fi
    echo
    echo "## Generated governed docs"
    echo "- [docs/experiments/EXPERIMENT_INDEX.auto.md](./experiments/EXPERIMENT_INDEX.auto.md)"
    while IFS= read -r -d '' cycle_dir; do
      [[ -f "${cycle_dir}/CYCLE_STATUS.auto.md" ]] || continue
      local cycle_id
      cycle_id="$(basename "${cycle_dir}")"
      echo "- [docs/experiments/${cycle_id}/CYCLE_STATUS.auto.md](./experiments/${cycle_id}/CYCLE_STATUS.auto.md)"
    done < <(find "${EXPERIMENTS_DIR}" -maxdepth 1 -type d -name 'cycle-*' -print0 | sort -z)
    echo
    echo "## Cycle ledger"
    echo "| Cycle | Charter | Decision | Verify | Auto status |"
    echo "|---|---|---|---|---|"
    while IFS= read -r -d '' cycle_dir; do
      [[ -f "${cycle_dir}/EXPERIMENT_CHARTER.md" ]] || continue
      local cycle_id charter_status decision_status verify_status
      cycle_id="$(basename "${cycle_dir}")"
      charter_status="$(extract_field "${cycle_dir}/EXPERIMENT_CHARTER.md" "status")"
      decision_status="$(extract_field "${cycle_dir}/DECISION_NOTE.md" "status")"
      verify_status="$(extract_verify_status "${cycle_dir}/VERIFY_STATUS.auto.md")"
      echo "| [${cycle_id}](./experiments/${cycle_id}/CYCLE_STATUS.auto.md) | ${charter_status:-unknown} | ${decision_status:-missing} | ${verify_status:-not_run} | present |"
    done < <(find "${EXPERIMENTS_DIR}" -maxdepth 1 -type d -name 'cycle-*' -print0 | sort -z)
  } > "${out}"
}

main() {
  if [[ "${1:-}" == "--help" || "${1:-}" == "-h" ]]; then
    usage
    exit 0
  fi

  [[ -d "${EXPERIMENTS_DIR}" ]] || {
    echo "error: experiments dir not found: ${EXPERIMENTS_DIR}" >&2
    exit 1
  }

  while IFS= read -r -d '' cycle_dir; do
    [[ -f "${cycle_dir}/EXPERIMENT_CHARTER.md" ]] || continue
    render_cycle_status "${cycle_dir}"
  done < <(find "${EXPERIMENTS_DIR}" -maxdepth 1 -type d -name 'cycle-*' -print0 | sort -z)

  render_experiment_index
  render_implementation_status_auto
  echo "Experiment docs synced."
}

main "$@"
