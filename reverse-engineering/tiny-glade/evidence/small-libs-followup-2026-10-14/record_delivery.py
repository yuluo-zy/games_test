"""Register this narrow follow-up without replacing historical import snapshots."""
from pathlib import Path
import hashlib
import json

out = Path(__file__).resolve().parent
repo = out.parents[3]
research = repo / 'reverse-engineering/tiny-glade'
workspace = repo / 'tiny-glade'
read = lambda p: json.loads(p.read_text(encoding='utf-8'))
write = lambda p, data: p.write_text(json.dumps(data, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
verification = read(out / 'verification.json')
assert verification['ok'], 'Delivery requires successful source-bound checks'
initial = read(out / 'input-source-hashes.json')
files = []
for relative, sha in verification['after'].items():
    if not relative.startswith('crates/'):
        continue
    path = workspace / relative
    assert hashlib.sha256(path.read_bytes()).hexdigest() == sha, 'Source changed after verification: ' + relative
    key = str(Path('tiny-glade') / relative)
    before = initial.get(key)
    files.append({'path': relative, 'sha256': sha, 'before_sha256': before,
                  'changed_or_added': sha != before})
audit = {
    'date': '2026-10-14', 'baseline': 'B', 'development_workspace': 'tiny-glade',
    'packages': ['tg-package-integrity', 'tg-text'],
    'files': files, 'verification': '../../reverse-engineering/tiny-glade/evidence/small-libs-followup-2026-10-14/verification.json',
    'report': '../../reverse-engineering/tiny-glade/evidence/small-libs-followup-2026-10-14/REPORT.md',
    'dependency_identities': read(out / 'dependency-identities.json'),
    'historical_import_snapshots_preserved': True,
    'scope': 'Direct recovery follow-up in development crates; original source text/ABI, whole game and GPU consumer equivalence not claimed.',
    'whole_original_libraries_accepted': False,
}
write(workspace / 'provenance/foundation-followup-2026-10-14.json', audit)
inventory_path = workspace / 'provenance/sources.json'
inventory = read(inventory_path)
by_path = {row['path']: row for row in files}
indexed = set()
for group in ('files', 'generated'):
    for row in inventory[group]:
        current = by_path.get(row['path'])
        if current is None:
            continue
        indexed.add(row['path'])
        if row['sha256'] != current['sha256']:
            row['pre_followup_sha256'] = row['sha256']
            row['sha256'] = current['sha256']
            row['post_import_recovery'] = 'provenance/foundation-followup-2026-10-14.json'
for path, row in by_path.items():
    if path not in indexed:
        inventory['generated'].append({'path': path, 'sha256': row['sha256'],
            'kind': 'original_code_bound_regression_fixture_or_test',
            'audit': 'provenance/foundation-followup-2026-10-14.json'})
inventory.setdefault('followup_audits', []).append('provenance/foundation-followup-2026-10-14.json')
write(inventory_path, inventory)
project_path = research / 'tracking/project.json'
project = read(project_path)
project['small_libraries_followup'] = {
    'date': '2026-10-14', 'baseline': 'B Windows x86_64', 'status': 'verified_scoped_followup',
    'source_workspace': '../../tiny-glade', 'packages': audit['packages'],
    'report': 'evidence/small-libs-followup-2026-10-14/REPORT.md',
    'verification': 'evidence/small-libs-followup-2026-10-14/verification.json',
    'evidence': ['evidence/integrity-followup-2026-10-14/REPORT.md', 'evidence/text-followup-2026-10-14/REPORT.md'],
    'whole_original_libraries_accepted': False, 'original_game_consumers_accepted': False,
}
write(project_path, project)
modules_path = research / 'tracking/module-progress.json'
modules = read(modules_path)
rows = modules.get('modules', [])
assert isinstance(rows, list) and rows, 'Unexpected module-progress schema'
for row in rows:
    name = row.get('module')
    if name not in ('crates/tg-package-integrity/src', 'crates/tg-text/src'):
        continue
    package = name.split('/')[1]
    row['current_development'] = {
        'workspace': '../../tiny-glade', 'package': package, 'date': '2026-10-14',
        'report': 'evidence/small-libs-followup-2026-10-14/REPORT.md',
        'verification': 'evidence/small-libs-followup-2026-10-14/verification.json',
        'source_files': [f['path'] for f in files if f['path'].startswith('crates/' + package + '/src/')],
        'whole_original_library_accepted': False,
    }
    if package == 'tg-text':
        row['recovered_scope'] = 'B FontTable/ShapeStream、TextFragment 编辑/metrics重建/排版、glyph cache/atlas/primitive/consume CPU链；render getter的count与借用atlas descriptor；真实GPU纹理和消费者未验收。'
    row.setdefault('evidence', []).append('evidence/small-libs-followup-2026-10-14/REPORT.md')
write(modules_path, modules)
print(json.dumps({'source_files': len(files), 'changed_or_added': sum(f['changed_or_added'] for f in files)}, ensure_ascii=False))
