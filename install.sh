#!/usr/bin/env bash
set -euo pipefail

# Research Governance Kit — Installer
# Installs the governance layer into a target Rust workspace.

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TARGET="${1:-.}"

echo "=== Research Governance Kit Installer ==="
echo "Source:  $SCRIPT_DIR"
echo "Target:  $(cd "$TARGET" && pwd)"
echo ""

# --- Check dependencies ---
echo "Checking dependencies..."

if ! command -v cargo &>/dev/null; then
  echo "  [FAIL] cargo not found. This kit requires a Rust workspace."
  exit 1
else
  echo "  [ok] cargo $(cargo --version | cut -d' ' -f2)"
fi

if command -v just &>/dev/null; then
  echo "  [ok] just $(just --version | cut -d' ' -f2)"
else
  echo "  [warn] just not found. Install it for shortcut recipes: cargo install just"
fi

echo ""

# --- Create directories ---
echo "Creating directories..."

dirs=(
  "$TARGET/docs/policies"
  "$TARGET/docs/experiments/TEMPLATE_FAMILY_v1"
  "$TARGET/benchmarks/manifests"
  "$TARGET/scripts"
)

for d in "${dirs[@]}"; do
  mkdir -p "$d"
  echo "  [ok] $d"
done

echo ""

# --- Copy policies ---
echo "Copying policies..."
for f in "$SCRIPT_DIR/docs/policies/"*.md; do
  [ -f "$f" ] || continue
  cp "$f" "$TARGET/docs/policies/"
  echo "  [ok] $(basename "$f")"
done

# --- Copy docs ---
echo "Copying docs..."
for f in "$SCRIPT_DIR/docs/"*.md; do
  [ -f "$f" ] || continue
  cp "$f" "$TARGET/docs/"
  echo "  [ok] $(basename "$f")"
done

# --- Copy templates ---
echo "Copying templates..."
for f in "$SCRIPT_DIR/docs/experiments/TEMPLATE_FAMILY_v1/"*.md; do
  [ -f "$f" ] || continue
  cp "$f" "$TARGET/docs/experiments/TEMPLATE_FAMILY_v1/"
  echo "  [ok] $(basename "$f")"
done

# --- Copy files/ templates (original kit) ---
echo "Copying kit files..."
if [ -d "$SCRIPT_DIR/files/docs" ]; then
  cp -r "$SCRIPT_DIR/files/docs/"* "$TARGET/docs/" 2>/dev/null || true
  echo "  [ok] files/docs/ merged into docs/"
fi

if [ -d "$SCRIPT_DIR/files/scripts" ]; then
  cp "$SCRIPT_DIR/files/scripts/"*.sh "$TARGET/scripts/" 2>/dev/null || true
  chmod +x "$TARGET/scripts/"*.sh 2>/dev/null || true
  echo "  [ok] scripts/"
fi

if [ -d "$SCRIPT_DIR/files/benchmarks" ]; then
  cp -r "$SCRIPT_DIR/files/benchmarks/"* "$TARGET/benchmarks/" 2>/dev/null || true
  echo "  [ok] benchmarks/"
fi

if [ -f "$SCRIPT_DIR/files/justfile" ]; then
  if [ -f "$TARGET/justfile" ]; then
    echo "  [skip] justfile already exists in target. Merge manually from files/justfile."
  else
    cp "$SCRIPT_DIR/files/justfile" "$TARGET/justfile"
    echo "  [ok] justfile"
  fi
fi

if [ -f "$SCRIPT_DIR/files/codemeta.json" ]; then
  if [ -f "$TARGET/codemeta.json" ]; then
    echo "  [skip] codemeta.json already exists. Adjust manually."
  else
    cp "$SCRIPT_DIR/files/codemeta.json" "$TARGET/codemeta.json"
    echo "  [ok] codemeta.json (adjust metadata for your project)"
  fi
fi

echo ""

# --- Summary ---
echo "=== Installation complete ==="
echo ""
echo "Pending manual steps:"
echo "  1. Adapt scripts/ to point to your crate executor (default: manager-plane)"
echo "  2. Review and adjust codemeta.json metadata"
echo "  3. Review docs/policies/ and adjust guardrails if needed"
echo "  4. Review justfile recipes and adjust crate names"
echo "  5. Create your first cycle: just cycle-new --cycle-id cycle-1 --title '...' --question '...'"
echo ""
echo "See COMO_INSTALAR_E_USAR.md for full usage guide."
echo "See KIT_CONTENTS.md for what this kit delivers and what it does not."
