"""Record the B font/cache closure and update only this package's current hashes."""
from pathlib import Path
import hashlib
import json
import subprocess
import tomllib

out = Path(__file__).resolve().parent
repo = out.parents[3]
workspace = repo / 'tiny-glade'
research = repo / 'reverse-engineering/tiny-glade'
read = lambda p: json.loads(p.read_text(encoding='utf-8'))
write = lambda p, data: p.write_text(json.dumps(data, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
verification = read(out / 'verification.json')
assert verification['ok']
before = read(out / 'before.json')
files = []
for path, digest in verification['after'].items():
    if not path.startswith('crates/tg-text/'):
        continue
    assert hashlib.sha256((workspace / path).read_bytes()).hexdigest() == digest
    files.append({'path': path, 'sha256': digest, 'before_sha256': before.get(path), 'changed': before.get(path) != digest})
assert not next(f for f in files if f['path'] == 'crates/tg-text/src/gpu.rs')['changed']
result = subprocess.run(['C:/Users/liyu/.cargo/bin/cargo.exe', 'metadata', '--offline', '--locked', '--format-version', '1', '--filter-platform', 'x86_64-pc-windows-msvc'], cwd=workspace, capture_output=True)
assert result.returncode == 0, result.stderr.decode(errors='replace')
(out / 'metadata.json').write_bytes(result.stdout)
metadata = json.loads(result.stdout)
packages = {p['id']: p for p in metadata['packages']}
nodes = {n['id']: n for n in metadata['resolve']['nodes']}
root = next(p['id'] for p in packages.values() if p['name'] == 'tg-text')
pending, seen = [root], set()
while pending:
    key = pending.pop()
    if key in seen:
        continue
    seen.add(key)
    pending.extend(n['pkg'] for n in nodes[key]['deps'])
current = tomllib.loads((workspace / 'Cargo.lock').read_text(encoding='utf-8'))
original = tomllib.loads((workspace / 'provenance/original-Cargo.lock').read_text(encoding='utf-8'))
identity = lambda p: tuple(p.get(k) for k in ('name','version','source','checksum'))
original_keys = {identity(p) for p in original['package']}
identities = []
for key in sorted(seen):
    package = packages[key]
    if not package.get('source'):
        continue
    row = next(p for p in current['package'] if all(p.get(k) == package.get(k) for k in ('name','version','source')))
    identities.append({'identity': identity(row), 'matches_original': identity(row) in original_keys})
assert all(p['matches_original'] for p in identities)
write(out / 'dependencies.json', {'root': root, 'features': nodes[root]['features'], 'external_count': len(identities), 'all_original': True, 'records': identities})
native = {name: read(out / path) for name, path in [
    ('font', 'font/native-font.json'), ('cache', 'cache/native-cache-oracle.json'), ('same_id', 'cache/native-same-id-oracle.json')]}
assert native['cache']['mismatches'] == native['same_id']['mismatches'] == 0
audit = {
    'date': '2026-10-14', 'baseline': 'B Windows x86_64', 'package': 'tg-text',
    'requested_internal_rules': ['owned shared font table / role selection / base-vs-scaled metrics', 'u32 frame/stamp and exact glyph cache validation/reclamation'],
    'status': 'restored_and_verified_scoped_internal_rules',
    'author_metrics_type_name_known': False, 'recovered_metrics_type_name': 'FontMetrics',
    'unproven_tuple_conversion_added': False,
    'game_callers_changed': False, 'gpu_wiring_changed': False, 'gpu_executed': False,
    'original_source_text_claimed': False, 'whole_library_or_game_accepted': False,
    'native_cases': {name: value['passed'] for name, value in native.items()},
    'public_dependency_identities_original': len(identities),
    'report': '../../reverse-engineering/tiny-glade/evidence/text-internal-2026-10-14/REPORT.md',
    'verification': '../../reverse-engineering/tiny-glade/evidence/text-internal-2026-10-14/verification.json',
    'checks': verification['checks'], 'files': files,
}
write(workspace / 'provenance/text-internal-audit.json', audit)
write(out / 'delivery.json', audit)
inventory_path = workspace / 'provenance/sources.json'
inventory = read(inventory_path)
by_path = {f['path']: f for f in files}
indexed = set()
for group in ('files','generated'):
    for row in inventory[group]:
        file = by_path.get(row['path'])
        if file is None:
            continue
        indexed.add(row['path'])
        if row['sha256'] != file['sha256']:
            row['pre_text_internal_sha256'] = row['sha256']
            row['sha256'] = file['sha256']
            row['text_internal_recovery'] = 'provenance/text-internal-audit.json'
for path, row in by_path.items():
    if path not in indexed:
        inventory['generated'].append({'path': path, 'sha256': row['sha256'], 'kind': 'original_instruction_bound_regression_fixture', 'audit': 'provenance/text-internal-audit.json'})
inventory.setdefault('followup_audits', []).append('provenance/text-internal-audit.json')
write(inventory_path, inventory)
project_path = research / 'tracking/project.json'
project = read(project_path)
project['tg_text_internal_recovery'] = {
    'date': audit['date'], 'baseline': audit['baseline'], 'status': audit['status'],
    'requested_internal_rules': audit['requested_internal_rules'],
    'report': 'evidence/text-internal-2026-10-14/REPORT.md',
    'verification': 'evidence/text-internal-2026-10-14/verification.json',
    'native_cases': audit['native_cases'], 'game_callers_changed': False, 'gpu_wiring_changed': False,
    'remaining_in_requested_scope': [], 'whole_library_accepted': False,
}
write(project_path, project)
module_path = research / 'tracking/module-progress.json'
modules = read(module_path)
row = next(m for m in modules['modules'] if m['module'] == 'crates/tg-text/src')
row.setdefault('current_development', {'workspace': '../../tiny-glade', 'package': 'tg-text', 'date': audit['date']})['internal_font_cache'] = project['tg_text_internal_recovery']
row['recovered_scope'] = 'B FontTable/ShapeStream、TextFragment持有Arc字体表/原角色选择/基准与scaled metrics、glyph/atlas/primitive CPU链；u32 frame/stamp、<与回绕、原etagere slot/gen、前缀验证和回收规则已恢复；真实GPU Texture/游戏调用方未接线。'
row.setdefault('evidence', []).append('evidence/text-internal-2026-10-14/REPORT.md')
write(module_path, modules)
print(json.dumps({'files':len(files), 'changed':sum(f['changed'] for f in files), 'native_cases':audit['native_cases'], 'public_dependencies':len(identities)}, ensure_ascii=False))
