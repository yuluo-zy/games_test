from pathlib import Path
import difflib, hashlib, json
root = Path('D:/game')
out = Path(__file__).parent
manifest = json.loads((out/'base.json').read_text())
patch = []
for record in manifest['files']:
    path = record['path']
    assert hashlib.sha256((root/path).read_bytes()).hexdigest() == record['base_sha256'], path
    before = (root/path).read_text(encoding='utf-8')
    after = (out/'candidate'/path).read_text(encoding='utf-8')
    record['candidate_sha256'] = hashlib.sha256((out/'candidate'/path).read_bytes()).hexdigest()
    patch += difflib.unified_diff(before.splitlines(keepends=True), after.splitlines(keepends=True), fromfile='a/'+path, tofile='b/'+path)
(out/'candidate.patch').write_text(''.join(patch), encoding='utf-8', newline='\n')
(out/'base.json').write_text(json.dumps(manifest, indent=2)+'\n', encoding='utf-8')
print('Candidate patch rebuilt from formatted sources; original source hashes unchanged.')
