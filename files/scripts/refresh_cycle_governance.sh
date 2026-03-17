#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'EOF'
usage:
  ./scripts/refresh_cycle_governance.sh \
    --cycle-dir /path/to/docs/experiments/cycle-x \
    [--manifest /path/to/manifest.json] \
    [--cycle-id cycle-x] \
    [--author codex] \
    [--checked-by codex] \
    [--report /path/to/report-a.json ...] \
    [--report-dir /path/to/reports-dir ...]

What it does:
  1. refreshes PRE_CYCLE_CHECKLIST.auto.md
  2. refreshes CANONICAL_COMPLETENESS_CHECK.auto.md
  3. if reports are provided, refreshes COMPARATIVE_NOTE.auto.md
  4. if reports are provided, refreshes DECISION_NOTE.auto.md

Defaults:
  - charter:    <cycle-dir>/EXPERIMENT_CHARTER.md
  - methodology:<cycle-dir>/METHODOLOGY_SPEC.md
  - cycle-id:   basename(<cycle-dir>)
EOF
}

die() {
  echo "error: $*" >&2
  exit 1
}

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

cycle_dir=""
manifest=""
cycle_id=""
author="codex"
checked_by="codex"
declare -a report_files=()
declare -a report_dirs=()

while [[ $# -gt 0 ]]; do
  case "$1" in
    --cycle-dir)
      cycle_dir="${2:-}"
      shift 2
      ;;
    --manifest)
      manifest="${2:-}"
      shift 2
      ;;
    --cycle-id)
      cycle_id="${2:-}"
      shift 2
      ;;
    --author)
      author="${2:-}"
      shift 2
      ;;
    --checked-by)
      checked_by="${2:-}"
      shift 2
      ;;
    --report)
      report_files+=("${2:-}")
      shift 2
      ;;
    --report-dir)
      report_dirs+=("${2:-}")
      shift 2
      ;;
    --help|-h)
      usage
      exit 0
      ;;
    *)
      die "unknown flag '$1'"
      ;;
  esac
done

[[ -n "${cycle_dir}" ]] || die "--cycle-dir is required"
[[ -d "${cycle_dir}" ]] || die "cycle dir not found: ${cycle_dir}"

charter="${cycle_dir}/EXPERIMENT_CHARTER.md"
methodology="${cycle_dir}/METHODOLOGY_SPEC.md"
preflight_out="${cycle_dir}/PRE_CYCLE_CHECKLIST.auto.md"
canonical_out="${cycle_dir}/CANONICAL_COMPLETENESS_CHECK.auto.md"
comparison_out="${cycle_dir}/COMPARATIVE_NOTE.auto.md"
decision_out="${cycle_dir}/DECISION_NOTE.auto.md"

[[ -f "${charter}" ]] || die "missing charter: ${charter}"
[[ -f "${methodology}" ]] || die "missing methodology: ${methodology}"

if [[ -z "${cycle_id}" ]]; then
  cycle_id="$(basename "${cycle_dir}")"
fi

declare -a resolved_reports=()
if [[ ${#report_files[@]} -gt 0 ]]; then
  for report in "${report_files[@]}"; do
    [[ -f "${report}" ]] || die "report not found: ${report}"
    resolved_reports+=("${report}")
  done
fi

if [[ ${#report_dirs[@]} -gt 0 ]]; then
  for report_dir in "${report_dirs[@]}"; do
    [[ -d "${report_dir}" ]] || die "report dir not found: ${report_dir}"
    while IFS= read -r -d '' file; do
      resolved_reports+=("${file}")
    done < <(find "${report_dir}" -maxdepth 1 -type f -name '*.json' -print0 | sort -z)
  done
fi

if [[ -n "${manifest}" && ! -f "${manifest}" ]]; then
  die "manifest not found: ${manifest}"
fi

echo "Refreshing preflight checklist..."
preflight_cmd=(
  cargo run -q -p manager-plane --example validate_experiment_governance -- preflight
  --charter "${charter}"
  --methodology "${methodology}"
  --out "${preflight_out}"
)
if [[ -n "${manifest}" ]]; then
  preflight_cmd+=(--manifest "${manifest}")
fi
(cd "${REPO_ROOT}" && "${preflight_cmd[@]}")

echo "Refreshing canonical completeness check..."
(cd "${REPO_ROOT}" && cargo run -q -p manager-plane --example validate_experiment_governance -- canonical \
  --charter "${charter}" \
  --methodology "${methodology}" \
  --out "${canonical_out}" \
  --checked-by "${checked_by}" \
  --document-id "${cycle_id}-governed-docs-auto")

if [[ ${#resolved_reports[@]} -ge 2 ]]; then
  echo "Refreshing comparative note draft..."
  comparison_cmd=(
    cargo run -q -p manager-plane --example draft_governance_notes -- comparison
    --out "${comparison_out}"
    --note-id "${cycle_id}-comparison-auto"
    --author "${author}"
  )
  for report in "${resolved_reports[@]}"; do
    comparison_cmd+=(--run "${report}")
  done
  (cd "${REPO_ROOT}" && "${comparison_cmd[@]}")
else
  echo "Skipping comparative note draft: fewer than 2 reports provided."
fi

if [[ ${#resolved_reports[@]} -ge 1 ]]; then
  echo "Refreshing decision note draft..."
  decision_cmd=(
    cargo run -q -p manager-plane --example draft_governance_notes -- decision
    --cycle-id "${cycle_id}"
    --out "${decision_out}"
    --decision-id "${cycle_id}-decision-auto"
    --author "${author}"
  )
  for report in "${resolved_reports[@]}"; do
    decision_cmd+=(--report "${report}")
  done
  (cd "${REPO_ROOT}" && "${decision_cmd[@]}")
else
  echo "Skipping decision note draft: no reports provided."
fi

echo "Refreshing experiment status docs..."
(cd "${REPO_ROOT}" && ./scripts/sync_experiment_docs.sh)

echo "Cycle governance refresh complete for ${cycle_id}."
