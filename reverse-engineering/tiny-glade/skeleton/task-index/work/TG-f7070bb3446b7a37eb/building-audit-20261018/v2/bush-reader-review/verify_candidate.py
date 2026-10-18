from pathlib import Path
import json,hashlib,tempfile,subprocess,capstone,pefile,struct
ROOT=Path('/Users/yuluo/Desktop/game');P=Path(__file__).resolve().parent;W=ROOT/'games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-6f2c1224768f58ae3c/core-bush-cpu-20261018';M=W/'frozen-inputs.json';assert hashlib.sha256(M.read_bytes()).hexdigest()=='8fdb38bab14ea4957cc716706813156978dc3413286ca312b744e31a7a3c7b18';x=json.loads(M.read_text())
for r in x['inputs']:
 b=Path(r['path']).read_bytes();assert len(b)==r['size'];assert hashlib.sha256(b).hexdigest()==r['sha256']
f=ROOT/'ljxsj_92385/Tiny Glade/tiny-glade.rne';assert hashlib.sha256(f.read_bytes()).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03';pe=pefile.PE(str(f),fast_load=True);md=capstone.Cs(capstone.CS_ARCH_X86,capstone.CS_MODE_64);rs=[]
for j in sorted((W/'native-pack').glob('*.json')):
 a=json.loads(j.read_text())
 if 'instructions' not in a:continue
 n=0
 for i in a['instructions']:
  va=int(i['va'],16);b=bytes.fromhex(i['bytes']);assert pe.get_data(va-0x140000000,len(b))==b;ins=list(md.disasm(b,va));assert len(ins)==1 and ins[0].size==len(b);n+=len(b)
 rs.append({'pack':j.name,'instructions':len(a['instructions']),'bytes':n,'overhang':a['range']['instruction_overhang_bytes']})
assert sum(r['instructions'] for r in rs)==394 and sum(r['bytes'] for r in rs)==1730
assert pe.get_data(0x142f3b268-0x140000000,8)==struct.pack('<Q',0x0deff5ab6a35972a)
name=b'country_core::systems::render_loop::compute_passes::read_bush_positions_for_cpu::BushCpuSsbo';assert len(name)==92 and pe.get_data(0x142f3b328-0x140000000,92)==name
assert pe.get_data(0x1429cd030-0x140000000,4)==struct.pack('<f',.7)
with tempfile.TemporaryDirectory(prefix='tg-bush-reader-review-') as d:
 t=Path(d)
 for f in (W/'base').rglob('*.rs'):
  out=t/f.relative_to(W/'base');out.parent.mkdir(parents=True,exist_ok=True);out.write_bytes(f.read_bytes())
 for patch in ['owned-reader.patch','integration-owner.patch']:
  for check in [True,False]:
   r=subprocess.run(['git','apply']+(['--check'] if check else [])+[str(W/patch)],cwd=t,capture_output=True,text=True);assert r.returncode==0,r.stderr
 for f in (W/'candidate').rglob('*.rs'):assert (t/f.relative_to(W/'candidate')).read_bytes()==f.read_bytes()
report={'status':'passed','frozen_inputs':25,'native':rs,'native_total_bytes':1730,'native_total_instructions':394,'native_name_key_radius_checked':True,'patch_same_batch_candidate_equality':2,'original_executed':False,'cargo_executed':False,'production_changed':False}
(P/'verification.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({k:v for k,v in report.items() if k!='native'}))
