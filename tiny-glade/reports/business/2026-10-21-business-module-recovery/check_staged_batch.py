"""Read-only source-drift and candidate inventory audit; never applies changes."""
from pathlib import Path
import hashlib
import json

root = Path('D:/game')
report = root / 'tiny-glade/reports/business-module-recovery-2026-10-21'
stage = report / 'proposed'

def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest() if path.exists() else None

known = {}
for name in ['sound-writer-migration.json', 'mode-event-migration.json']:
    data = json.loads((report / name).read_text(encoding='utf-8'))
    entries = data['files'] if isinstance(data, dict) else data
    for entry in entries:
        relative = entry.get('source', entry.get('path'))
        assert relative is not None, entry
        known[relative] = {
            'base': entry.get('source_sha256', entry.get('original_sha256')),
            'candidate': entry.get('proposed_sha256', entry.get('candidate_sha256')),
            'manifest': name,
        }

drift = []
inventory = []
for path in sorted(stage.rglob('*')):
    if not path.is_file():
        continue
    relative = path.relative_to(stage).as_posix()
    entry = {'source': relative, 'live_sha256': sha(root / relative), 'candidate_sha256': sha(path)}
    if relative in known:
        original = known[relative]
        assert original['base'] is not None and original['candidate'] is not None, (relative, original)
        entry['baseline_manifest'] = original['manifest']
        entry['live_matches_manifest'] = entry['live_sha256'] == original['base']
        entry['candidate_matches_manifest'] = entry['candidate_sha256'] == original['candidate']
        if not entry['live_matches_manifest'] or not entry['candidate_matches_manifest']:
            drift.append(relative)
    else:
        # A current hash is only an inventory, never retroactive baseline proof.
        entry['baseline_manifest'] = None
        entry['status'] = 'requires narrow-diff review before integration'
    inventory.append(entry)

result = {'production_modified': False, 'compiled': False, 'drift': drift, 'files': inventory}
(report / 'staged-batch-audit.json').write_text(json.dumps(result, indent=2) + '\n', encoding='utf-8')
print(json.dumps({'candidate_files': len(inventory), 'manifested_files': len(known), 'drift': drift}))
raise SystemExit(bool(drift))
