"""Check the audited foundation package README/source references."""
from pathlib import Path
import json
import re

ROOT = Path(__file__).resolve().parents[1]
audit = json.loads((ROOT / 'provenance/foundation-audit.json').read_text(encoding='utf-8'))
missing = []
for package in audit['packages']:
    readme = ROOT / 'crates' / package / 'README.md'
    for link in re.findall(r'\]\(([^)]+)\)', readme.read_text(encoding='utf-8')):
        if not link.startswith('https://') and not (readme.parent / link).exists():
            missing.append(f'{package}: {link}')
for file in (ROOT / 'crates/utils').rglob('*.rs'):
    content = file.read_text(encoding='utf-8')
    if '/../../../../evidence/' in content:
        missing.append(f'nonlocal fixture: {file}')
print('\n'.join(missing) if missing else 'All foundation README and fixture references resolve')
raise SystemExit(bool(missing))
