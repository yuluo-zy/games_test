"""Migrate the stopped, unaccepted State draft over the current owner's base.

Only this delegated handoff directory is written; no production source is edited.
"""
from pathlib import Path
import difflib, hashlib, json, subprocess, tempfile

WORK = Path(__file__).resolve().parent
ROOT = next(p for p in WORK.parents if (p / 'games_test/tiny-glade').is_dir())
OLD = ROOT / 'games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-57a33750ac4a866a76/core-producer-20261018/snapshot-state-deserialize-v1'
REL = 'crates/country-core/src/systems/save/snapshot.rs'
SOURCE = ROOT / 'games_test/tiny-glade' / REL
before = (OLD / 'base/snapshot.rs').read_text()
draft = (OLD / 'candidate/snapshot.rs').read_text()
current_base = SOURCE.read_text()
start = current_base.index('/// 快照 Decorators 节 addr 判别')
end = current_base.index('impl TerrainSectionJson {', start)
draft_start = draft.index('/// B snapshot addresses use unit "Stairs"')
draft_end = draft.index('impl TerrainSectionJson {', draft_start)
candidate = current_base[:start] + draft[draft_start:draft_end] + current_base[end:]
candidate = candidate.replace('        assert!(section.into_storage_snapshot().is_err());',
    '        let snapshot = section.into_storage_snapshot().unwrap();\n        assert!(snapshot.states.is_empty());')
(WORK / 'base/snapshot.rs').write_bytes(SOURCE.read_bytes())
(WORK / 'candidate/snapshot.rs').write_text(candidate)
# Parse/format an isolated copy, so pre-existing formatting outside this task
# does not become an unrelated production diff.
with tempfile.TemporaryDirectory(prefix='state-reader-format-') as directory:
    formatted = Path(directory) / 'snapshot.rs'
    formatted.write_text(candidate)
    subprocess.run(['rustfmt', '--edition', '2021', str(formatted)], check=True)
# The draft predates the real runtime generator propagation. Its API remains
# wholly present in the migrated current base; reject accidental old-base reuse.
current = (WORK / 'candidate/snapshot.rs').read_text()
assert current.count('decorator_ids: &mut DecoratorIdGen') == 2
assert 'restore_snapshot_with_id_gen(&snapshot, decorator_ids)' in current
assert 'DefaultInfo' not in current
rows = []
for file in sorted(OLD.rglob('*')):
    if file.is_file():
        rows.append({'path': str(file), 'size': file.stat().st_size, 'sha256': hashlib.sha256(file.read_bytes()).hexdigest()})
(WORK / 'prior-draft-inputs.json').write_text(json.dumps(rows, indent=2) + '\n')
print(json.dumps({'base_sha256': hashlib.sha256(SOURCE.read_bytes()).hexdigest(), 'candidate_sha256': hashlib.sha256((WORK / 'candidate/snapshot.rs').read_bytes()).hexdigest(), 'prior_draft_inputs': len(rows)}))
