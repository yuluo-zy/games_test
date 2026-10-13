#!/bin/bash
# Run remaining Ghidra jobs sequentially on the shared read-only worker.
OP=/Users/yuluo/Desktop/game/tools/mcp-servers/ghidra-mcp/.venv/bin/python
RO=/Users/yuluo/.codex/mcp-tools/reverse-operator.py
EVID=/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/reconstruction/country-core-render/wp21/evidence
for j in g3-stairs-state-1 g4-stairs-2-graph g5-stairs-assembly g6-ivy-rt-ext g7-misc-1 g8-misc-2 g9-misc-3; do
  echo "=== running $j ==="
  "$OP" "$RO" ghidra "$EVID/$j.json" "$EVID/$j.result.json" --worker shared 2>&1 | tail -2
done
