from pathlib import Path
import json,hashlib,tempfile,subprocess,capstone,pefile
ROOT=Path('/Users/yuluo/Desktop/game');P=Path(__file__).resolve().parent;W=ROOT/'games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-27aa1146cf8ff64436/core-wall-ui-adapter-20261018';M=W/'frozen-inputs-v2.json';assert hashlib.sha256(M.read_bytes()).hexdigest()=='cad8ef60501bb06766b2ea9dff22e666c3a60bd602c3a87d315c474030414f3f';x=json.loads(M.read_text())
for r in x['inputs']:
 b=Path(r['path']).read_bytes();assert len(b)==r['bytes'];assert hashlib.sha256(b).hexdigest()==r['sha256']
f=ROOT/'ljxsj_92385/Tiny Glade/tiny-glade.rne';assert hashlib.sha256(f.read_bytes()).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03';pe=pefile.PE(str(f),fast_load=True);md=capstone.Cs(capstone.CS_ARCH_X86,capstone.CS_MODE_64)
for r in x['ranges']:
 b=(W/'native-bounded-v2'/(r['name']+'.bin')).read_bytes();assert pe.get_data(int(r['va'],16)-0x140000000,r['bytes'])==b;ins=list(md.disasm(b,int(r['va'],16)));assert len(ins)==r['instructions'] and sum(i.size for i in ins)==r['bytes']
extra=[]
for a,b in [(0x140aae652,0x140aae66c),(0x140a8d7ce,0x140a8db88)]:
 data=pe.get_data(a-0x140000000,b-a);ins=list(md.disasm(data,a));assert sum(i.size for i in ins)==b-a;extra.append({'start':hex(a),'end':hex(b),'bytes_hex':data.hex(),'instructions':[{'va':hex(i.address),'bytes':i.bytes.hex(),'asm':i.mnemonic+' '+i.op_str} for i in ins]})
(P/'independent-extra-raw.json').write_text(json.dumps({'boundary':'static original XYZ payload writes and CMP-to-SETNE flag-preservation continuity only','ranges':extra},indent=2)+'\n')
with tempfile.TemporaryDirectory(prefix='tg-wall-ui-public-review-') as d:
 t=Path(d);prefix=t/'games_test/tiny-glade'
 for f in (W/'base').rglob('*.rs'):
  out=prefix/f.relative_to(W/'base');out.parent.mkdir(parents=True,exist_ok=True);out.write_bytes(f.read_bytes())
 pub=prefix/'crates/country-core/src/systems/wall/wall_state/publish_public_wall_state.rs';pub.parent.mkdir(parents=True,exist_ok=True);pub.write_bytes((W/'publisher-before.rs').read_bytes())
 for patch,cwd in [('candidate.patch',t),('publisher-owner.patch',prefix)]:
  for check in [True,False]:
   r=subprocess.run(['git','apply']+(['--check'] if check else [])+[str(W/patch)],cwd=cwd,capture_output=True,text=True);assert r.returncode==0,r.stderr
 for f in (W/'candidate').rglob('*.rs'):assert (prefix/f.relative_to(W/'candidate')).read_bytes()==f.read_bytes()
 assert pub.read_bytes()==(W/'publisher-owner-candidate.rs').read_bytes()
report={'status':'passed','frozen_inputs':36,'original_ranges':5,'original_instructions':45,'original_bytes':305,'additional_XYZ_and_flags_continuity_bytes':sum((int(r['end'],16)-int(r['start'],16)) for r in extra),'same_batch_apply_candidate_equality':10,'original_executed':False,'cargo_executed':False,'production_changed':False,'actual_publisher_handoff_applied':False}
(P/'verification.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps(report))
