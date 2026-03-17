#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'EOF'
usage:
  ./scripts/verify_cycle_progress.sh \
    --cycle-dir /path/to/docs/experiments/cycle-x \
    [--manifest /path/to/manifest.json] \
    [--cycle-id cycle-x] \
    [--author codex] \
    [--checked-by codex] \
    [--report /path/to/report-a.json ...] \
    [--report-dir /path/to/reports-dir ...] \
    [--verify-command "cargo test -p proof-runtime" ...] \
    [--skip-default-workspace-test]

What it does:
  1. refreshes the governed cycle package
  2. runs verification commands
  3. writes VERIFY_STATUS.auto.md with the verification result

Defaults:
  - runs `cargo test --workspace` unless `--skip-default-workspace-test` is given
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
skip_default_workspace_test="false"
declare -a report_files=()
declare -a report_dirs=()
declare -a verify_commands=()

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
    --verify-command)
      verify_commands+=("${2:-}")
      shift 2
      ;;
    --skip-default-workspace-test)
      skip_default_workspace_test="true"
      shift
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

if [[ -z "${cycle_id}" ]]; then
  cycle_id="$(basename "${cycle_dir}")"
fi

verify_status_out="${cycle_dir}/VERIFY_STATUS.auto.md"
started_at="$(date -u +%Y-%m-%dT%H:%M:%SZ)"

refresh_cmd=(
  "${REPO_ROOT}/scripts/refresh_cycle_governance.sh"
  --cycle-dir "${cycle_dir}"
  --cycle-id "${cycle_id}"
  --author "${author}"
  --checked-by "${checked_by}"
)

if [[ -n "${manifest}" ]]; then
  refresh_cmd+=(--manifest "${manifest}")
fi

if [[ ${#report_files[@]} -gt 0 ]]; then
  for report in "${report_files[@]}"; do
    refresh_cmd+=(--report "${report}")
  done
fi

if [[ ${#report_dirs[@]} -gt 0 ]]; then
  for report_dir in "${report_dirs[@]}"; do
    refresh_cmd+=(--report-dir "${report_dir}")
  done
fi

declare -a commands_run=()
if [[ "${skip_default_workspace_test}" != "true" ]]; then
  verify_commands=("cargo test --workspace" "${verify_commands[@]}")
fi

write_status() {
  local result="$1"
  local failed_command="${2:-}"
  local finished_at
  finished_at="$(date -u +%Y-%m-%dT%H:%M:%SZ)"

  {
    echo "# Verify Status"
    echo
    echo "## Identity"
    echo "- cycle_id: ${cycle_id}"
    echo "- date_started_utc: ${started_at}"
    echo "- date_finished_utc: ${finished_at}"
    echo "- status: ${result}"
    echo
    echo "## Refresh"
    echo "- command: \`scripts/refresh_cycle_governance.sh\`"
    echo
    echo "## Verification commands"
    if [[ ${#commands_run[@]} -eq 0 && ${#verify_commands[@]} -eq 0 ]]; then
      echo "- none"
    else
      for command in "${commands_run[@]}"; do
        echo "- \`${command}\`"
      done
    fi
    echo
    if [[ -n "${failed_command}" ]]; then
      echo "## Failure"
      echo "- failed_command: \`${failed_command}\`"
      echo
    fi
    echo "## Meaning"
    if [[ "${result}" == "PASS" ]]; then
      echo "> The governed package was refreshed and the selected evidence commands completed successfully."
    else
      echo "> The governed package refresh or one of the selected evidence commands failed. Do not treat the cycle state as freshly verified."
    fi
  } > "${verify_status_out}"
}

failed_command=""
trap 'status=$?; if [[ $status -ne 0 ]]; then write_status "FAIL" "${failed_command}"; fi' EXIT

echo "Refreshing governed cycle package..."
(cd "${REPO_ROOT}" && "${refresh_cmd[@]}")

if [[ ${#verify_commands[@]} -eq 0 ]]; then
  echo "No verification commands requested. Writing PASS with refresh only."
else
  for command in "${verify_commands[@]}"; do
    failed_command="${command}"
    commands_run+=("${command}")
    echo "Running verification command: ${command}"
    (cd "${REPO_ROOT}" && bash -lc "${command}")
    failed_command=""
  done
fi

write_status "PASS"
trap - EXIT
echo "Cycle verification complete for ${cycle_id}."
