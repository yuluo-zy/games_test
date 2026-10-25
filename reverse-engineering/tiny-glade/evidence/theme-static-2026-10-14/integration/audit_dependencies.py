import tomllib,json,hashlib
from pathlib import Path
root=Path('D:/game')
out=root/'reverse-engineering/tiny-glade/evidence/theme-static-2026-10-14/integration'
lock=root/'tiny-glade/Cargo.lock'
orig=root/'ljxsj_92385/Tiny Glade/build-info/Cargo.lock'
if not orig.exists(): orig=root/'tiny-glade/provenance/original-Cargo.lock'
cur=tomllib.loads(lock.read_text(encoding='utf-8'))['package']
old=tomllib.loads(orig.read_text(encoding='utf-8'))['package']
old_index={(p['name'],p['version'],p.get('source')):p for p in old}
by_name={}
for p in cur: by_name.setdefault(p['name'],[]).append(p)
def resolve(s):
 fields=s.split()
 candidates=by_name[fields[0]]
 if len(fields)>1: candidates=[p for p in candidates if p['version']==fields[1]]
 if len(fields)>2: candidates=[p for p in candidates if p.get('source')==fields[2].strip('()')]
 if len(candidates)!=1: raise ValueError((s,candidates))
 return candidates[0]
todo=[resolve(n) for n in ['theme-defs','theme-fetcher','theme-server-proto']]
seen={}
while todo:
 p=todo.pop(); k=(p['name'],p['version'],p.get('source'))
 if k in seen:continue
 seen[k]=p
 todo.extend(resolve(d) for d in p.get('dependencies',[]))
rows=[]
for k,p in sorted(seen.items()):
 if not p.get('source'):continue
 o=old_index.get(k)
 rows.append(dict(name=p['name'],version=p['version'],source=p['source'],checksum=p.get('checksum'),matched_original=o is not None and p.get('checksum')==o.get('checksum')))
report=dict(baseline='B',lock=str(lock),lock_sha256=hashlib.sha256(lock.read_bytes()).hexdigest(),original_lock=str(orig),original_lock_sha256=hashlib.sha256(orig.read_bytes()).hexdigest(),packages=len(seen),public_packages=len(rows),mismatches=[r for r in rows if not r['matched_original']],rows=rows)
(out/'dependency-audit.json').write_text(json.dumps(report,indent=2),encoding='utf-8')
print(json.dumps({k:v for k,v in report.items() if k!='rows'}))
