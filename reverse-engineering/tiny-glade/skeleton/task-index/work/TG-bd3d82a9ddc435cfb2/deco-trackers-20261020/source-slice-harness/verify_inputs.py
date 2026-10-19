from pathlib import Path
import hashlib, json, sys, tomllib
root = Path(__file__).resolve().parent
repo = Path('/Users/yuluo/Desktop/game/games_test')
def sha(p): return hashlib.sha256(p.read_bytes()).hexdigest()
spans = json.loads((root/'SOURCE_SPANS.json').read_text())['spans']
for s in spans:
    p = repo/s['source']
    assert sha(p) == s['source_sha256'], ('changed source input', str(p))
    original = p.read_bytes()[s['start_byte']:s['end_byte_exclusive']]
    sliced = (root/s['output']).read_bytes()
    assert original == sliced and hashlib.sha256(sliced).hexdigest() == s['slice_sha256']
deps = json.loads((root/'DEPENDENCY_INPUTS.json').read_text())
assert sha(root/'WORKSPACE_LOCK_INPUT.snapshot') == deps['workspace_lock_sha256']
for s in deps['candidate_includes'].values():
    assert sha(Path(s['path'])) == s['sha256'], ('changed candidate', s['path'])
if '--prepared-lock' in sys.argv:
    baseline = tomllib.loads((root/'WORKSPACE_LOCK_INPUT.snapshot').read_text())['package']
    locked = tomllib.loads((root/'Cargo.lock').read_text())['package']
    identities = {(p['name'], p['version'], p.get('source'), p.get('checksum')) for p in baseline}
    for p in locked:
        if p['name'] == 'deco-trackers-source-slice' and 'source' not in p:
            continue
        assert (p['name'], p['version'], p.get('source'), p.get('checksum')) in identities, ('new dependency identity', p['name'], p['version'])
print('exact source/candidate inputs verified' + ('; prepared lock is identity subset of frozen workspace lock' if '--prepared-lock' in sys.argv else ''))
