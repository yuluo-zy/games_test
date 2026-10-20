from pathlib import Path
import hashlib,json
here=Path(__file__).parent
source=here/'candidates-v23'
target=here/'snapshots/v23-22'
assert not target.exists(), 'Never overwrite a central integration snapshot'
manifest=json.loads((source/'manifest.json').read_text())
for record in manifest:
    candidate=Path(record['candidate']);payload=candidate.read_bytes()
    assert hashlib.sha256(payload).hexdigest()==record['candidate_sha256']
    relative=record['path'].removeprefix('tiny-glade/')
    copy=target/relative;copy.parent.mkdir(parents=True,exist_ok=True);copy.write_bytes(payload)
    record['candidate']=str(copy)
(target/'manifest.json').write_text(json.dumps(manifest,indent=2))
(target/'candidate.patch').write_bytes((source/'candidate.patch').read_bytes())
print(json.dumps({'files':len(manifest),'snapshot':str(target),'manifest_sha256':hashlib.sha256((target/'manifest.json').read_bytes()).hexdigest()}))
