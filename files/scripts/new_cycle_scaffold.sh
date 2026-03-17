#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'EOF'
usage:
  ./scripts/new_cycle_scaffold.sh \
    --cycle-id cycle-4 \
    --title "Short cycle title" \
    --question "What is the single question for this cycle?" \
    [--owner ubl-ops] \
    [--author ubl-ops] \
    [--manifest-id cycle-4-v1] \
    [--methodology-id cycle-4-v1] \
    [--notes "Optional note text"] \
    [--open]

What it does:
  1. creates docs/experiments/<cycle-id>/
  2. scaffolds EXPERIMENT_CHARTER.md
  3. scaffolds METHODOLOGY_SPEC.md
  4. scaffolds COMPARATIVE_NOTE.md
  5. scaffolds DECISION_NOTE.md
  6. writes a manifest in benchmarks/manifests/<manifest-id>.json
  7. refreshes the automatic checklist and canonical check

Notes:
  - It does not freeze anything.
  - It creates a governed starting point so the next work is thinking, not folder choreography.
EOF
}

die() {
  echo "error: $*" >&2
  exit 1
}

require_value() {
  local name="$1"
  local value="$2"
  [[ -n "${value}" ]] || die "${name} is required"
}

escape_sed() {
  printf '%s' "$1" | sed -e 's/[\/&]/\\&/g'
}

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

cycle_id=""
title=""
question=""
owner="$(id -un)"
author="$(id -un)"
manifest_id=""
methodology_id=""
notes=""
open_after="false"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --cycle-id)
      cycle_id="${2:-}"
      shift 2
      ;;
    --title)
      title="${2:-}"
      shift 2
      ;;
    --question)
      question="${2:-}"
      shift 2
      ;;
    --owner)
      owner="${2:-}"
      shift 2
      ;;
    --author)
      author="${2:-}"
      shift 2
      ;;
    --manifest-id)
      manifest_id="${2:-}"
      shift 2
      ;;
    --methodology-id)
      methodology_id="${2:-}"
      shift 2
      ;;
    --notes)
      notes="${2:-}"
      shift 2
      ;;
    --open)
      open_after="true"
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

require_value "--cycle-id" "${cycle_id}"
require_value "--title" "${title}"
require_value "--question" "${question}"

if [[ -z "${manifest_id}" ]]; then
  manifest_id="${cycle_id}-v1"
fi

if [[ -z "${methodology_id}" ]]; then
  methodology_id="${cycle_id}-v1"
fi

date_utc="$(date -u +%F)"
cycle_dir="${REPO_ROOT}/docs/experiments/${cycle_id}"
manifest_path="${REPO_ROOT}/benchmarks/manifests/${manifest_id}.json"
template_dir="${REPO_ROOT}/docs/experiments/TEMPLATE_FAMILY_v1"

[[ -d "${template_dir}" ]] || die "template family not found: ${template_dir}"
[[ ! -e "${cycle_dir}" ]] || die "cycle dir already exists: ${cycle_dir}"
[[ ! -e "${manifest_path}" ]] || die "manifest already exists: ${manifest_path}"

mkdir -p "${cycle_dir}"
mkdir -p "$(dirname "${manifest_path}")"

charter_path="${cycle_dir}/EXPERIMENT_CHARTER.md"
methodology_path="${cycle_dir}/METHODOLOGY_SPEC.md"
comparative_path="${cycle_dir}/COMPARATIVE_NOTE.md"
decision_path="${cycle_dir}/DECISION_NOTE.md"

cp "${template_dir}/TEMPLATE_EXPERIMENT_CHARTER.md" "${charter_path}"
cp "${template_dir}/TEMPLATE_METHODOLOGY_SPEC.md" "${methodology_path}"
cp "${template_dir}/TEMPLATE_COMPARATIVE_NOTE.md" "${comparative_path}"
cp "${template_dir}/TEMPLATE_DECISION_NOTE.md" "${decision_path}"

sed -i '' \
  -e "s/- experiment_id:/- experiment_id: $(escape_sed "${cycle_id}")/" \
  -e "s/- cycle_id:/- cycle_id: $(escape_sed "${cycle_id}")/" \
  -e "s/- title:/- title: $(escape_sed "${title}")/" \
  -e "s/- owner:/- owner: $(escape_sed "${owner}")/" \
  -e "s/- date_opened:/- date_opened: $(escape_sed "${date_utc}")/" \
  -e "s/- status: planned | running | frozen | closed/- status: planned/" \
  -e "s/> What is the single question this cycle is trying to answer?/> $(escape_sed "${question}")/" \
  "${charter_path}"

sed -i '' \
  -e "s/- methodology_id:/- methodology_id: $(escape_sed "${methodology_id}")/" \
  -e "s/- related_experiment_id:/- related_experiment_id: $(escape_sed "${cycle_id}")/" \
  -e "s/- author:/- author: $(escape_sed "${author}")/" \
  -e "s/- date:/- date: $(escape_sed "${date_utc}")/" \
  -e "s/- status: active | frozen | superseded/- status: planned/" \
  -e "s/> What does this methodology operationalize?/> Operationalize the governed question for $(escape_sed "${cycle_id}") without changing more than one main variable at once./" \
  "${methodology_path}"

sed -i '' \
  -e "s/- note_id:/- note_id: $(escape_sed "${cycle_id}")-comparison/" \
  -e "s/- date:/- date: $(escape_sed "${date_utc}")/" \
  -e "s/- author:/- author: $(escape_sed "${author}")/" \
  "${comparative_path}"

sed -i '' \
  -e "s/- decision_id:/- decision_id: $(escape_sed "${cycle_id}")-decision/" \
  -e "s/- date:/- date: $(escape_sed "${date_utc}")/" \
  -e "s/- author:/- author: $(escape_sed "${author}")/" \
  -e "s/- related_cycle:/- related_cycle: $(escape_sed "${cycle_id}")/" \
  "${decision_path}"

cat > "${manifest_path}" <<EOF
{
  "labels": {
    "methodology": "${methodology_id}",
    "engine": "engine-v1",
    "adapter": "adapter-v1",
    "benchmark_spec": "benchmark-spec-v1",
    "provider_selection": "provider-selection-v1",
    "input_protocol": "input-protocol-v1",
    "case_corpus": "case-corpus-v1",
    "report_schema": "report-schema-v1"
  },
  "tracked_paths": {
    "methodology": [
      "docs/EXPERIMENT_CYCLES.md",
      "docs/experiments/${cycle_id}/EXPERIMENT_CHARTER.md",
      "docs/experiments/${cycle_id}/METHODOLOGY_SPEC.md",
      "benchmarks/manifests/${manifest_id}.json"
    ],
    "engine": [
      "crates/epistemic-storage/src/lib.rs",
      "crates/proof-runtime/src/lib.rs",
      "crates/manager-plane/src/lib.rs"
    ],
    "adapter": [
      "crates/manager-plane/src/lib.rs",
      "crates/manager-plane/src/proposer.rs"
    ],
    "benchmark_spec": [
      "docs/BENCHMARK_PROTOCOL.md",
      "crates/manager-plane/src/benchmark.rs"
    ],
    "provider_selection": [
      "crates/manager-plane/src/proposer.rs",
      "crates/manager-plane/examples/run_containment_benchmark.rs"
    ],
    "input_protocol": [
      "docs/experiments/${cycle_id}/METHODOLOGY_SPEC.md"
    ],
    "report_schema": [
      "crates/manager-plane/src/benchmark.rs"
    ]
  },
  "notes": "${notes}",
  "tags": [
    "${cycle_id}"
  ],
  "registry_path": "benchmarks/version_registry.json"
}
EOF

"${REPO_ROOT}/scripts/refresh_cycle_governance.sh" \
  --cycle-dir "${cycle_dir}" \
  --cycle-id "${cycle_id}" \
  --manifest "${manifest_path}" \
  --author "${author}" \
  --checked-by "${author}"

echo "Scaffolded governed cycle package:"
echo "  - ${charter_path}"
echo "  - ${methodology_path}"
echo "  - ${comparative_path}"
echo "  - ${decision_path}"
echo "  - ${manifest_path}"

if [[ "${open_after}" == "true" ]]; then
  if command -v open >/dev/null 2>&1; then
    open "${cycle_dir}"
  fi
fi
