#!/usr/bin/env bash
# Installation des git hooks depuis scripts/hooks/
# Usage : bash scripts/install-hooks.sh

set -euo pipefail

REPO_ROOT="$(git rev-parse --show-toplevel)"
HOOKS_SRC="$REPO_ROOT/scripts/hooks"
HOOKS_DST="$REPO_ROOT/.git/hooks"

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

if [ ! -d "$HOOKS_SRC" ]; then
  echo "Dossier scripts/hooks/ introuvable." >&2
  exit 1
fi

echo "📦 Installation des git hooks..."

for hook in "$HOOKS_SRC"/*; do
  name="$(basename "$hook")"
  dest="$HOOKS_DST/$name"

  cp "$hook" "$dest"
  chmod +x "$dest"
  echo -e "  ${GREEN}✓${NC} $name installé"
done

echo ""
echo -e "${GREEN}✅ Hooks installés dans .git/hooks/${NC}"
echo -e "${YELLOW}ℹ️  Pour désinstaller : rm .git/hooks/pre-commit${NC}"
