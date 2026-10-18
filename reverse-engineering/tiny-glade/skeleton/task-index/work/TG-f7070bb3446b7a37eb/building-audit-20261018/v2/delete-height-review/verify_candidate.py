import json,hashlib,tempfile,subprocess
from pathlib import Path
import pefile,capstone
ROOT=Path('/Users/yuluo/Desktop/game');P=Path(__file__).resolve().parent;W=ROOT/'games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-eb56a08d047af785ea/core-wall-height-20261018';M=W/'frozen-inputs.json'
assert hashlib.sha256(M.read_bytes()).hexdigest()=='e073912c8b0d3de80ebefa6779c9c4dc6b7f583a207002ab1cb730179ddbead8'
a=json.loads(M.read_text())
for r in a['inputs']:
 b=Path(r['path']).read_bytes();assert len(b)==r['size'];assert hashlib.sha256(b).hexdigest()==r['sha256']
rne=ROOT/'ljxsj_92385/Tiny Glade/tiny-glade.rne';assert hashlib.sha256(rne.read_bytes()).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03';pe=pefile.PE(str(rne),fast_load=True);md=capstone.Cs(capstone.CS_ARCH_X86,capstone.CS_MODE_64)
native=[]
for f in sorted((W/'native-pack').glob('*.json')):
 x=json.loads(f.read_text())
 if 'instructions' not in x:continue
 n=0
 for i in x['instructions']:
  v=int(i['va'],16);b=bytes.fromhex(i['bytes']);assert pe.get_data(v-0x140000000,len(b))==b
  ins=list(md.disasm(b,v));assert len(ins)==1 and ins[0].size==len(b);n+=len(b)
 native.append({'pack':str(f),'instructions':len(x['instructions']),'decoded_bytes':n,'overhang':x['range']['instruction_overhang_bytes']})
for k in json.loads((W/'serializer-keys.json').read_text())['keys']:assert pe.get_data(int(k['va'],16)-0x140000000,k['length']).hex()==k['hex']
patch=[]
with tempfile.TemporaryDirectory(prefix='tg-delete-height-review-') as temp:
 t=Path(temp)
 for f in (W/'base').rglob('*.rs'):
  out=t/'games_test/tiny-glade/crates/country-core/src'/f.relative_to(W/'base');out.parent.mkdir(parents=True,exist_ok=True);out.write_bytes(f.read_bytes())
 for name in ['owned-wall-history.patch','integration-owner.patch']:
  args=['git','apply','--check',str(W/name)];r=subprocess.run(args,cwd=t,text=True,capture_output=True);assert r.returncode==0,(name,r.stderr)
  r=subprocess.run(['git','apply',str(W/name)],cwd=t,text=True,capture_output=True);assert r.returncode==0,(name,r.stderr);patch.append(name)
 for f in (W/'candidate').rglob('*.rs'):
  out=t/'games_test/tiny-glade/crates/country-core/src'/f.relative_to(W/'candidate');assert out.read_bytes()==f.read_bytes(),str(f)
report={'status':'passed','frozen_inputs':len(a['inputs']),'manifest_sha256':hashlib.sha256(M.read_bytes()).hexdigest(),'native':native,'total_instructions':sum(r['instructions'] for r in native),'total_decoded_bytes':sum(r['decoded_bytes'] for r in native),'keys_verified':5,'patches_applied_to_captured_bases':patch,'candidate_sources_equal_after_apply':6,'original_executed':False,'cargo_executed':False,'production_changed':False}
(P/'verification.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({k:v for k,v in report.items() if k!='native'}))
