from pathlib import Path
import hashlib, json, difflib, shutil

root = Path('D:/game')
out = root / 'reverse-engineering/tiny-glade/evidence/round13-frame-graph-gpu/bindings'
manifest = json.loads((out / 'manifest.json').read_text(encoding='utf-8'))
patches = []
for entry in manifest['files']:
    rel = entry['path']
    base = out / 'base' / rel
    candidate = out / 'candidate' / rel
    assert hashlib.sha256(base.read_bytes()).hexdigest() == entry['base_sha256']
    entry['candidate_sha256'] = hashlib.sha256(candidate.read_bytes()).hexdigest()
    entry['current_source_sha256'] = hashlib.sha256((root / rel).read_bytes()).hexdigest()
    entry['base_still_matches_production'] = entry['base_sha256'] == entry['current_source_sha256']
    patches.extend(difflib.unified_diff(base.read_text(encoding='utf-8').splitlines(True),
        candidate.read_text(encoding='utf-8').splitlines(True), fromfile='a/' + rel, tofile='b/' + rel))
(out / 'candidate.patch').write_text(''.join(patches), encoding='utf-8', newline='\n')
shutil.copyfile(root / 'tools/reverse-runtime/round13-bindings/backend.log', out / 'mcp-backend-failure.log')
manifest.update({
    'patch': 'candidate.patch',
    'patch_sha256': hashlib.sha256((out / 'candidate.patch').read_bytes()).hexdigest(),
    'status': 'implemented_unverified',
    'ownership': 'root integrates; this worker made no production changes',
    'validation': {'rustfmt': 'passed', 'cargo': 'not_run_worker_contract', 'gpu': 'not_run',
                   'original_range_identity': 'passed_4_full_pdb_ranges_rne_equal_patched'},
    'requested_root_checks': [
        'cargo test -p tiny-glade --lib raster_binding_range_tests --locked --offline',
        'cargo test -p tiny-glade --lib original_surface_raster_intent_tests --locked --offline',
        'nonempty production frame/GPU acceptance after integrating other workers'],
    'remaining': [
        'raster attachments still use current owner-wide range; depth layer and opaque ColorViewWords require separate complete attachment projection',
        'external SSBO original reflection contains no readonly qualifier; conservative RW left intact',
        'no actual GPU equivalence/runtime claim from these static facts or unexecuted tests'],
})
(out / 'manifest.json').write_text(json.dumps(manifest, indent=2), encoding='utf-8')
print(json.dumps(manifest, indent=2))
