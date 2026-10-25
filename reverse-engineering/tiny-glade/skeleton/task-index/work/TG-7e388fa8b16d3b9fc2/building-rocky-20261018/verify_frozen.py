from pathlib import Path
import json,hashlib
P=Path(__file__).resolve().parent
j=json.loads((P/'frozen-inputs.json').read_text());bad=[]
for r in j['inputs']:
 p=Path(r['path']);h=hashlib.sha256()
 with p.open('rb') as f:
  for chunk in iter(lambda:f.read(1<<20),b''):h.update(chunk)
 if p.stat().st_size!=r['size'] or h.hexdigest()!=r['sha256']:bad.append(str(p))
assert not bad,bad
print({'verified_inputs':len(j['inputs']),'manifest_sha256':hashlib.sha256((P/'frozen-inputs.json').read_bytes()).hexdigest(),'native_executed':False,'cargo_executed':False})
