from pathlib import Path
import difflib, hashlib, json

root = Path('D:/game')
parent = root / 'reverse-engineering/tiny-glade/evidence/round13-frame-graph-gpu/bindings'
out = parent / 'attachments-v2'
manifest = json.loads((out / 'manifest.json').read_text(encoding='utf-8'))
def digest(blob):
    return hashlib.sha256(blob).hexdigest()
def nl(blob):
    return blob.replace(b'\r\n', b'\n')
patch = []
for entry in manifest['files']:
    rel = entry['path']
    base = out / 'base' / rel
    candidate = out / 'candidate' / rel
    before = base.read_bytes() if base.exists() else b''
    after = candidate.read_bytes()
    if entry['base_sha256'] is not None:
        assert digest(before) == entry['base_sha256']
    entry['base_lf_sha256'] = digest(nl(before)) if base.exists() else None
    entry['candidate_sha256'] = digest(after)
    entry['candidate_lf_sha256'] = digest(nl(after))
    actual = (root / rel).read_bytes() if (root / rel).exists() else b''
    entry['current_source_sha256'] = digest(actual) if (root / rel).exists() else None
    entry['base_matches_current_lf'] = nl(before) == nl(actual)
    patch.extend(difflib.unified_diff(nl(before).decode('utf-8').splitlines(True),
        nl(after).decode('utf-8').splitlines(True),
        fromfile='a/' + rel if base.exists() else '/dev/null', tofile='b/' + rel))
(out / 'candidate.patch').write_text(''.join(patch), encoding='utf-8', newline='\n')
static_paths = [
    'tiny-glade/crates/rhapsody/src/rg/resource_info.rs',
    'tiny-glade/crates/rhapsody/src/rg/resource_contracts.rs',
    'tiny-glade/crates/rhapsody/src/rg/plan.rs',
    'tiny-glade/crates/rhapsody/src/rg/subgraph.rs',
    'tiny-glade/crates/rhapsody/src/rg/plan_check.rs',
    'tiny-glade/crates/rhapsody/src/rg/attachments.rs',
]
manifest.update({
    'status': 'implemented_unverified',
    'patch_sha256': digest((out / 'candidate.patch').read_bytes()),
    'lineage': {'bindings_v1_manifest': '../manifest.json',
                'bindings_v1_patch_sha256': 'b33c3d17a6e9c0f1bcba5c79a36097ec600b6c2ac37445c1e24351cb421a37cb',
                'old_manifest_preserved': True},
    'unchanged_dependency_audit': [{'path': rel, 'lf_sha256': digest(nl((root / rel).read_bytes()))} for rel in static_paths],
    'validation': {'rustfmt': 'passed', 'cargo': 'not_run_worker_contract', 'gpu': 'not_run',
                   'original': '6 full PDB ranges RNE/patched identical; static only'},
    'root_checks': [
        'cargo test -p tiny-glade --lib raster_attachment_range_tests --locked --offline',
        'cargo test -p rhapsody --test preserving_dedup --locked --offline',
        'actual nonempty frame/shadow cascades/ice clear GPU acceptance'],
    'remaining': [
        'Same selected LOAD range with partially defined contents stays strict and can fail graph validation; no blanket undefined-preserving exemption added',
        'Legacy View/view/ColorViewWords source names retained via clear_color_bits compatibility contract; semantic evidence is no longer unknown',
        'New regression tests and original-GPU equivalence are not executed by this worker'],
})
(out / 'manifest.json').write_text(json.dumps(manifest, indent=2), encoding='utf-8')
print(json.dumps(manifest, indent=2))
