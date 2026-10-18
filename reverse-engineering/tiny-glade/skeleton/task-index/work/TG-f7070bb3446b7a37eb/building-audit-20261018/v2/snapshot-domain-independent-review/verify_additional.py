from pathlib import Path
import json,hashlib,re,subprocess,tempfile,shutil
import capstone,pefile
ROOT=Path('/Users/yuluo/Desktop/game');P=ROOT/'games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-f7070bb3446b7a37eb/building-audit-20261018/v2/snapshot-domain-independent-review';W=P/'inputs';pe=pefile.PE(str(ROOT/'ljxsj_92385/Tiny Glade/tiny-glade.rne'),fast_load=True);md=capstone.Cs(capstone.CS_ARCH_X86,capstone.CS_MODE_64);md.detail=True
ins={}
for f in W.glob('*.bin'):
 text=f.with_suffix('.asm').read_text();a=int(text.split(':',1)[0],16);data=f.read_bytes();ii=list(md.disasm(data,a));assert sum(i.size for i in ii)==len(data);ins.update({i.address:i for i in ii})
# Existing frozen69 State serializer is a declared dependency of snapshot61.
state_pack=ROOT/'games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-889aa76ca030a2e3cd/terrain-curve-20261018/serde-pack/history-state-serialize-value.json'
state_evidence=json.loads(state_pack.read_text())
for row in state_evidence['instructions']:
 a=int(row['va'],16);raw=bytes.fromhex(row['bytes']);assert raw==pe.get_data(a-0x140000000,len(raw));i=next(md.disasm(raw,a));assert i.size==len(raw);ins[a]=i
# Bind every field string to the actual RIP-derived address and actual encoded length.
fieldchecks=[]
for row in json.loads((W/'field-string-bindings.json').read_text()):
 i=ins[int(row['lea'],16)];o=next(o for o in i.operands if o.type==capstone.x86.X86_OP_MEM and o.mem.base==capstone.x86.X86_REG_RIP);va=i.address+i.size+o.mem.disp;assert va==int(row['va'],16)
 i=ins[int(row['length_site'],16)];assert i.mnemonic=='mov' and i.operands[0].type==capstone.x86.X86_OP_REG and i.operands[0].reg==capstone.x86.X86_REG_R8D and i.operands[1].imm==row['length']
 raw=pe.get_data(va-0x140000000,row['length']);assert raw.decode()==row['value'];fieldchecks.append(row)
# Walk only straight-line enum string cases, including their shared length tail.
rows=json.loads((W/'clutter-mesh-string-bindings.json').read_text());clutterchecks=[]
source=(W/'candidate/snapshot.rs').read_text();names=json.loads('['+re.search(r'const CLUTTER_MESH_NAMES: \[&str; 56\] = \[(.*?)\];',source,re.S)[1].rstrip().rstrip(',')+']')
for row in rows['bindings']:
 pc=int(row['case_va'],16);name_va=length=None;sites=[]
 for step in range(8):
  i=ins[pc];sites.append(hex(pc))
  if i.mnemonic=='lea' and i.operands[0].reg==capstone.x86.X86_REG_RDX:
   o=i.operands[1];assert o.mem.base==capstone.x86.X86_REG_RIP;name_va=i.address+i.size+o.mem.disp
  if i.mnemonic=='mov' and i.operands[0].type==capstone.x86.X86_OP_REG and i.operands[0].reg==capstone.x86.X86_REG_R8D:length=i.operands[1].imm
  if i.mnemonic=='jmp':pc=i.operands[0].imm;continue
  if i.mnemonic=='call':assert i.operands[0].imm==0x14179a0e0;break
  pc+=i.size
 else:raise AssertionError(('unterminated',row))
 assert name_va==int(row['string_va'],16) and length==row['length'];assert pe.get_data(name_va-0x140000000,length).decode()==row['value']==names[row['index']]
 clutterchecks.append({'index':row['index'],'value':row['value'],'case_sites':sites,'string_va':hex(name_va),'length':length})
# Native dispatch tables for the four destination arms, special decorator types, and railings.
def table(site,n,expected):
 i=ins[site];o=next(o for o in i.operands if o.type==capstone.x86.X86_OP_MEM and o.mem.base==capstone.x86.X86_REG_RIP);a=i.address+i.size+o.mem.disp;raw=pe.get_data(a-0x140000000,n*4);values=[a+int.from_bytes(raw[k:k+4],'little',signed=True) for k in range(0,len(raw),4)];assert values==expected;return {'site':hex(site),'table':hex(a),'bytes':raw.hex(),'targets':[hex(v) for v in values]}
tables=[table(0x140a4a0ee,4,[0x140a4a0fe,0x140a4a1a3,0x140a4a135,0x140a4a16c]),table(0x140a47232,8,[0x140a47242,0x140a473a1,0x140a472c6,0x140a47312,0x140a47298,0x140a473f0,0x140a47431,0x140a4735a]),table(0x140a48fd3,4,[0x140a48fe3,0x140a49050,0x140a49008,0x140a49029])]
# Additional dependency: the side value itself is Inner for zero / Outer otherwise.
a=0x141593c80;b=(P/'side-native/141593c80.bin').read_bytes();pd=json.loads((P/'side-native/141593c80.pdb.json').read_text());assert len(b)==pd['code_size']==678 and b==pe.get_data(a-0x140000000,len(b));ii=list(md.disasm(b,a));assert sum(i.size for i in ii)==678;side={i.address:i for i in ii};assert (P/'side-native/141593c80.asm').read_text()==''.join(f'{i.address:x}: {i.bytes.hex()}  {i.mnemonic} {i.op_str}\n' for i in ii)
for site,expected in [(0x141593d73,b'Outer'),(0x141593d8e,b'Inner')]:
 i=side[site];o=i.operands[1];assert o.mem.base==capstone.x86.X86_REG_RIP;va=i.address+i.size+o.mem.disp;assert pe.get_data(va-0x140000000,5)==expected
assert side[0x141593d5c].mnemonic=='test' and side[0x141593d5e].mnemonic=='je' and side[0x141593d5e].operands[0].imm==0x141593d7b
# Actual typed raster collector proves a Vec payload, not an inferred Option/missing-raster channel.
import gzip
raster=None
for line in gzip.open(ROOT/'games_test/reverse-engineering/tiny-glade/skeleton/_pdb/B/modules/01610.jsonl.gz','rt'):
 r=json.loads(line)
 if r.get('record_offset')==69784:raster=r;break
assert raster and 'tuple$<ref$<str$>,alloc::vec::Vec<u8,alloc::alloc::Global> >' in raster['name'];(P/'raster-fold-pdb.json').write_text(json.dumps(raster,indent=2)+'\n')
# Exact sequential delta application: frozen61 -> test API v2 -> borrowed Wall ID v3.
with tempfile.TemporaryDirectory(prefix='snapshot-final-api-review-',dir='/private/tmp') as temp:
 dst=Path(temp)/'crates/country-core/src/systems/save/snapshot.rs';dst.parent.mkdir(parents=True);shutil.copyfile(W/'candidate/snapshot.rs',dst)
 for directory in [P/'test-v2',P/'wall-v3']:
  assert dst.read_bytes()==(directory/'base.rs').read_bytes()
  result=subprocess.run(['git','apply','--check',str(directory/'candidate.patch')],cwd=temp,capture_output=True,text=True);assert result.returncode==0,result.stderr
  result=subprocess.run(['git','apply',str(directory/'candidate.patch')],cwd=temp,capture_output=True,text=True);assert result.returncode==0,result.stderr
  assert dst.read_bytes()==(directory/'snapshot.rs').read_bytes()
 final=hashlib.sha256(dst.read_bytes()).hexdigest();assert final=='8508fceddfca138c2621a95d19f08ee6e125d6b8b5b0622b69225a4074210835'
# Current API checkpoints are evidence for the trait fix and outstanding live handoffs.
paths=['crates/country-core/src/resources/walls/mod.rs','crates/country-core/src/resources/walls/inner_wall_state.rs','crates/country-core/src/resources/tree_storage.rs','crates/country-core/src/resources/color_id.rs','crates/country-core/src/resources/stairs/stairs_removed_supports.rs','crates/systems/terrain/src/stroke.rs','crates/systems/terrain/src/schema.rs','crates/motomoto/src/writer.rs','crates/motomoto/src/tag.rs','crates/motomoto/src/lib.rs','crates/country-core/src/systems/save/save_game.rs','crates/country-core/src/systems/save/save_thread.rs'];checkpoints=[]
for rel in paths:
 f=ROOT/'games_test/tiny-glade'/rel;d=P/'api-checkpoints'/rel;d.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(f,d);checkpoints.append({'source':str(f),'copy':str(d),'sha256':hashlib.sha256(f.read_bytes()).hexdigest()})
res={'native_execution':False,'cargo_execution':False,'field_RIP_targets_and_encoded_lengths_verified':len(fieldchecks),'clutter_original_case_pointer_length_and_candidate_names_verified':clutterchecks,'three_native_dispatch_tables':tables,'side_dependency_complete_bytes':678,'side_dependency_instructions':len(ii),'final_sequential_candidate_sha256':final,'delta_chain_exact':True,'current_API_checkpoints':checkpoints,'remaining_worker_to_json_is_not_native_file_writer':True}
(P/'additional-verification.json').write_text(json.dumps(res,indent=2)+'\n');print(json.dumps({'keys':len(fieldchecks),'clutter':len(clutterchecks),'side_bytes':678,'sequential_final_sha':final,'current_API_checkpoints':len(checkpoints),'cargo':False,'native':False},indent=2))
