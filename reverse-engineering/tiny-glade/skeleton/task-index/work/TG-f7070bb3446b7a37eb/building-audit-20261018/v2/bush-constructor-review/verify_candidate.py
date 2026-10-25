from pathlib import Path
import json,hashlib,tempfile,subprocess,re
import pefile,capstone
ROOT=Path('/Users/yuluo/Desktop/game');P=Path(__file__).resolve().parent;W=ROOT/'games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-6b43449ef07afe08ea/core-bush-cpu-20261018';M=W/'constructor-frozen-inputs.json'
assert hashlib.sha256(M.read_bytes()).hexdigest()=='c3fcae39e2fbea28955f935670ae41e232d2b594b59de0d3329a7fe88bf137c7'
x=json.loads(M.read_text())
for r in x['inputs']:
 b=Path(r['path']).read_bytes();assert len(b)==r['bytes'];assert hashlib.sha256(b).hexdigest()==r['sha256'],r['path']
rne=ROOT/'ljxsj_92385/Tiny Glade/tiny-glade.rne';pe=pefile.PE(str(rne),fast_load=True);o=json.loads((W/'original-inputs.json').read_text());assert hashlib.sha256(rne.read_bytes()).hexdigest()==o['binary_sha256'];md=capstone.Cs(capstone.CS_ARCH_X86,capstone.CS_MODE_64)
rs=[]
for r,stem in zip(o['ranges'],['constructor','reader']):
 va=int(r['entry'],16);b=(W/(stem+'-B.bin')).read_bytes();assert len(b)==r['bytes'];assert hashlib.sha256(b).hexdigest()==r['sha256'];assert b==pe.get_data(va-0x140000000,len(b));ins=list(md.disasm(b,va));decoded=sum(i.size for i in ins)
 if stem=='constructor':assert decoded==len(b)
 else:assert decoded==744 and b[decoded:]==bytes.fromhex('e8')
 lines=(W/(stem+'-B.asm')).read_text().splitlines();assert len(lines)==len(ins)
 for i,line in zip(ins,lines):
  m=re.match(r'([0-9a-f]+):\s+([0-9a-f]+)\s+(.*)',line);assert m;assert int(m[1],16)==i.address and m[2]==i.bytes.hex();assert m[3].strip()==(i.mnemonic+' '+i.op_str).strip()
 rs.append({'name':stem,'bytes':len(b),'instructions':len(ins),'PE_and_ASM_equal':True,'undecoded_tail':b[decoded:].hex()})
name=pe.get_data(0x142c80120-0x140000000,22);assert name==b'Bush positions for CPU',name
with tempfile.TemporaryDirectory(prefix='tg-bush-constructor-review-') as d:
 t=Path(d);rel=Path('crates/country-core/src/systems/render_loop/compute_passes/read_bush_positions_for_cpu.rs');f=t/rel;f.parent.mkdir(parents=True);f.write_bytes((W/'base'/rel).read_bytes())
 for check in [True,False]:
  args=['git','apply']+(['--check'] if check else [])+[str(W/'constructor-candidate.patch')];r=subprocess.run(args,cwd=t,capture_output=True,text=True);assert r.returncode==0,r.stderr
 assert f.read_bytes()==(W/'candidate'/rel).read_bytes()
key=None
try:
 import xxhash
 key=xxhash.xxh3_64_intdigest(name);assert key==0x0deff5ab6a35972a
except ImportError:pass
report={'status':'passed','frozen_inputs':len(x['inputs']),'native':rs,'original_name':name.decode(),'independently_computed_xxh3':hex(key) if key is not None else None,'patch_same_batch_apply':True,'candidate_exact':True,'original_executed':False,'cargo_executed':False,'production_changed':False}
(P/'verification.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps(report))
