"""Check the two default CPU dependency closures against the original B lock."""
from pathlib import Path
import json
import tomllib

out = Path(__file__).resolve().parent
repo = out.parents[3]
metadata_file = out / 'metadata-final.json'
if not metadata_file.is_file():
    metadata_file = out / 'metadata.json'
metadata = json.loads(metadata_file.read_bytes())
packages = {p['id']: p for p in metadata['packages']}
nodes = {n['id']: n for n in metadata['resolve']['nodes']}
roots = [p['id'] for p in packages.values() if p['name'] in ('tg-text', 'tg-package-integrity')]
pending = roots.copy()
seen = set()
while pending:
    key = pending.pop()
    if key in seen:
        continue
    seen.add(key)
    pending.extend(n['pkg'] for n in nodes[key]['deps'])
current = tomllib.loads((repo / 'tiny-glade/Cargo.lock').read_text())
original = tomllib.loads((repo / 'tiny-glade/provenance/original-Cargo.lock').read_text())
def identity(package):
    return tuple(package.get(k) for k in ('name', 'version', 'source', 'checksum'))
original_keys = {identity(p) for p in original['package']}
records = []
for key in sorted(seen):
    package = packages[key]
    if not package.get('source'):
        continue
    row = next(p for p in current['package'] if all(p.get(k) == package.get(k) for k in ('name', 'version', 'source')))
    records.append({'identity': identity(row), 'matches_original': identity(row) in original_keys})
report = {'roots': roots, 'platform': 'x86_64-pc-windows-msvc', 'features': {packages[i]['name']: nodes[i]['features'] for i in roots},
          'external_count': len(records), 'all_original': all(r['matches_original'] for r in records), 'records': records}
(out / 'dependency-identities.json').write_text(json.dumps(report, indent=2) + '\n', encoding='utf-8')
print(json.dumps({k: v for k, v in report.items() if k not in ('roots', 'records')}))
raise SystemExit(0 if report['all_original'] else 1)
