from pathlib import Path
import hashlib,json,re,tempfile,subprocess,pefile
R=Path('/Users/yuluo/Desktop/game');T=R/'games_test/reverse-engineering/tiny-glade/skeleton/task-index/work';S=T/'TG-c4a5b4adcb068a950d/terrain-curve-20261018/endpoint-v3';W=Path(__file__).resolve().parent
m=json.loads((S/'frozen-inputs.json').read_text());items=m.get('files',m.get('inputs'));checks=[]
for f in items:
 p=Path(f['path']);b=p.read_bytes();assert hashlib.sha256(b).hexdigest()==f['sha256'];assert len(b)==f.get('bytes',f.get('size'));checks.append({'path':str(p),'sha256':f['sha256'],'size':len(b),'match':True})
p=pefile.PE(str(R/'ljxsj_92385/Tiny Glade/tiny-glade.rne'),fast_load=True);x=json.loads((S/'byte-verification.json').read_text());spans=[]
for row in x['assemblies']:
 q=Path(row['path']);assert hashlib.sha256(q.read_bytes()).hexdigest()==row['sha256'];count=size=0
 for line in q.read_text().splitlines():
  match=re.match(r'^([0-9a-f]+):\s+((?:[0-9a-f]{2}\s+)+)',line)
  if not match:continue
  va=int(match.group(1),16);raw=bytes.fromhex(match.group(2));assert raw==p.get_data(va-0x140000000,len(raw));count+=1;size+=len(raw)
 assert count==row['instructions'] and size==row['instruction_bytes'];spans.append({'path':str(q),'instructions':count,'bytes':size,'all_bytes_match':True})
with tempfile.TemporaryDirectory(prefix='tg-endpoint-review-') as td:
 t=Path(td)
 for row in json.loads((S/'source-map.json').read_text()):
  src=Path(row['base_source']);blob=src.read_bytes();assert hashlib.sha256(blob).hexdigest()==row['base_sha256'];out=t/row['target'];out.parent.mkdir(parents=True,exist_ok=True);out.write_bytes(blob)
 r=subprocess.run(['git','apply',str(S/'candidate.patch')],cwd=t,capture_output=True,text=True);assert r.returncode==0,r.stderr
 for row in json.loads((S/'source-map.json').read_text()):assert (t/row['target']).read_bytes()==Path(row['source']).read_bytes()
(W/'verification.json').write_text(json.dumps({'target':'B','upstream_manifest_sha256':hashlib.sha256((S/'frozen-inputs.json').read_bytes()).hexdigest(),'inputs':checks,'native_assemblies':spans,'temporary_patch_apply_and_candidate_bytes':'passed','native_instructions':sum(x['instructions'] for x in spans),'native_bytes':sum(x['bytes'] for x in spans),'compiled':False,'native_executed':False},indent=2)+'\n');print(json.dumps({'hash_files':len(checks),'instructions':sum(x['instructions'] for x in spans),'bytes':sum(x['bytes'] for x in spans),'temporary_apply_exact':True}))
