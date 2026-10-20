from pathlib import Path
import hashlib
import json
import difflib

root = Path('D:/game')
base_root = root / 'reverse-engineering/tiny-glade/evidence/round13-frame-graph-gpu/temporal/candidate'
out = root / 'reverse-engineering/tiny-glade/evidence/round13-frame-graph-gpu/gpu/deferred-cancel'
manifest = json.loads((out / 'manifest.json').read_text())
patch = ''
for item in manifest['files']:
    base = (base_root / item['path']).read_bytes()
    assert hashlib.sha256(base).hexdigest() == item['base_sha256']
    candidate = (out / 'candidate' / item['path']).read_bytes()
    item['candidate_sha256'] = hashlib.sha256(candidate).hexdigest()
    patch += ''.join(difflib.unified_diff(
        base.decode('utf-8').splitlines(keepends=True), candidate.decode('utf-8').splitlines(keepends=True),
        fromfile='a/' + item['path'], tofile='b/' + item['path'],
    ))
manifest['validation'] = 'rustfmt parse/check; no Cargo/GPU run by worker; base is frozen temporal v2'
(out / 'candidate.patch').write_text(patch, encoding='utf-8', newline='')
(out / 'manifest.json').write_text(json.dumps(manifest, indent=2), encoding='utf-8')
print(json.dumps(manifest))
