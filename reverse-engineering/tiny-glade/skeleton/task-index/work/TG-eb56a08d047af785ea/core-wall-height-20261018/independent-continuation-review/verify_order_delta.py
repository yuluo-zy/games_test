from pathlib import Path
import json,hashlib,tempfile,subprocess,shutil
R=Path('/Users/yuluo/Desktop/game');W=Path(__file__).resolve().parent;T=R/'games_test/reverse-engineering/tiny-glade/skeleton/task-index/work';S=T/'TG-9ae382a5028cc58515/terrain-curve-20261018';D=T/'TG-96136a8afd6a171528/terrain-curve-20261018/continuation-order-v3'
m=json.loads((D/'frozen-inputs.json').read_text());rows=[]
for f in m['files']:
 p=Path(f['path']);b=p.read_bytes();assert hashlib.sha256(b).hexdigest()==f['sha256'];assert len(b)==f['bytes'];rows.append({'path':str(p),'sha256':f['sha256'],'size':len(b),'match':True})
with tempfile.TemporaryDirectory(prefix='tg-continue-review-') as t:
 td=Path(t)
 for row in json.loads((S/'continuation-source-map.json').read_text()):
  if row['base_sha256'] is None:continue
  source=R/row['target'];blob=source.read_bytes();assert hashlib.sha256(blob).hexdigest()==row['base_sha256'];out=td/row['target'];out.parent.mkdir(parents=True,exist_ok=True);out.write_bytes(blob)
 for patch in [S/'continuation-candidate.patch',D/'candidate.patch']:
  p=subprocess.run(['git','apply',str(patch)],cwd=td,capture_output=True,text=True);assert p.returncode==0,p.stderr
 final=td/'games_test/tiny-glade/crates/country-core/src/systems/wall/finish_wall.rs';assert final.read_bytes()==(D/'finish_wall.rs').read_bytes()
(W/'order-delta-verification.json').write_text(json.dumps({'target':'B','delta_manifest_sha256':hashlib.sha256((D/'frozen-inputs.json').read_bytes()).hexdigest(),'inputs':rows,'original29_then_delta_apply_in_temporary_tree':'passed','final_finish_equals_frozen_v3':True,'finding':'closed: remove precedes color lookup; actual World panic/removal regression source present','compiled':False,'native_executed':False},indent=2)+'\n');print('4hashes and full29 + delta apply + finalhash passed')
