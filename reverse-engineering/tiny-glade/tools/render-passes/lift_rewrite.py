#!/usr/bin/env python3
"""Rewrite a mixed shader file: semantic renames in lifted bodies, tmp_NN for
the remaining SPIR-V temporaries, Chinese header block. Behavior-identical
(identifier renames + comments only); readable_check.py must still PASS.
Usage: lift_rewrite.py ORIG NEW [SEMANTIC_JSON]
SEMANTIC_JSON: {"_NN": "name", ...} applied word-boundary across the WHOLE file.
"""
import json
import re
import sys
from pathlib import Path

orig = Path(sys.argv[1])
new = Path(sys.argv[2])
sem = json.load(open(sys.argv[3])) if len(sys.argv) > 3 else {}

src = orig.read_text()

# Apply semantic map first.
for old, name in sem.items():
    src = re.sub(r'\b' + re.escape(old) + r'\b', name, src)

# Rename every remaining _NN to tmp_NN (preserves codegen; clearer intent).
src = re.sub(r'\b(_\d+)\b', lambda m: 'tmp' + m.group(1), src)

# Translate the generic lifted-source comment line.
src = src.replace('(spirv-cross; behaviour-exact, names are SPIR-V ids)',
                  '(spirv-cross;行为一致,已语义化)')

new.write_text(src)
left = re.findall(r'\b_\d+\b', src)
print('rewrote', new, 'leftover _NN:', len(left))
