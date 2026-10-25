from pathlib import Path
import json, hashlib, subprocess, tempfile, shutil, struct
import pefile, capstone
ROOT=Path('/Users/yuluo/Desktop/game');P=Path(__file__).resolve().parent;E=P/'evidence'
PE=ROOT/'ljxsj_92385/Tiny Glade/tiny-glade.rne';PDB=PE.with_name('tiny_glade.pdb')
def sha(path):
 h=hashlib.sha256()
 with path.open('rb') as f:
  for b in iter(lambda:f.read(1<<20),b''):h.update(b)
 return h.hexdigest()
assert sha(PE)=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
assert sha(PDB)=='2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911'
pe=pefile.PE(str(PE),fast_load=True);md=capstone.Cs(capstone.CS_ARCH_X86,capstone.CS_MODE_64);md.detail=True;ins={};total=count=0
ranges=json.loads((E/'ranges.json').read_text())
for row in ranges.values():
 a=int(row['va'],16);raw=(E/f'{a:x}.bin').read_bytes();pdb=json.loads((E/f'{a:x}.pdb.json').read_text())
 assert raw==pe.get_data(a-0x140000000,row['bytes'])
 assert hashlib.sha256(raw).hexdigest()==row['sha256']
 assert len(raw)==pdb['code_size'] and pdb['name']==row['name']
 ii=list(md.disasm(raw,a));assert sum(i.size for i in ii)==len(raw)
 assert (E/f'{a:x}.asm').read_text()=='\n'.join(f'{i.address:#x} {i.bytes.hex()} {i.mnemonic} {i.op_str}' for i in ii)+'\n'
 ins.update({i.address:i for i in ii});count+=len(ii);total+=len(raw)
# Existing whole capture pack is reused at this bounded caller span; no new whole capture analysis.
span=(0x1417a2216,0x1417a22aa);raw=pe.get_data(span[0]-0x140000000,span[1]-span[0]);ii=list(md.disasm(raw,span[0]));assert sum(i.size for i in ii)==len(raw)
original=ROOT/'games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-57a33750ac4a866a76/core-producer-20261018/snapshot-domain-candidate/capture-original.asm'
old={int(l.split(':',1)[0],16):l.split(':',1)[1].strip().split()[0] for l in original.read_text().splitlines() if ':' in l and len(l.split(':',1)[0])==9}
for i in ii:assert old[i.address]==i.bytes.hex()
(E/'capture-roofs-caller.bin').write_bytes(raw);(E/'capture-roofs-caller.asm').write_text('\n'.join(f'{i.address:#x} {i.bytes.hex()} {i.mnemonic} {i.op_str}' for i in ii)+'\n');ins.update({i.address:i for i in ii})
refs=[]
def call(a,b):
 i=ins[a];assert i.mnemonic=='call' and i.operands[0].imm==b,(hex(a),hex(b),i.op_str)
def rip(a,expected):
 i=ins[a];o=next(o for o in i.operands if o.type==capstone.x86.X86_OP_MEM and o.mem.base==capstone.x86.X86_REG_RIP);va=i.address+i.size+o.mem.disp;b=pe.get_data(va-0x140000000,len(expected));assert b==expected,(hex(a),b,expected);refs.append({'site':hex(a),'target':hex(va),'bytes':b.hex(),'text':b.decode('ascii')})
def asm(a,expected):assert ins[a].mnemonic+' '+ins[a].op_str==expected,(hex(a),ins[a].mnemonic+' '+ins[a].op_str,expected)
for a,b in [(0x1417a2252,0x14158faf0),(0x14158fbb2,0x1416ea700),(0x1416ea756,0x141599600),(0x1416ea77d,0x141594190),(0x1416ea7a4,0x141597950),(0x1416ea7cb,0x141595a20),(0x1416ea7ee,0x141599250),(0x1416ea811,0x141599e60),(0x1416ea837,0x141596b30),(0x14159425e,0x1416eaa40),(0x141596bfe,0x1416e8f20),(0x1416e7a5b,0x141592910),(0x1415929de,0x140a44a00),(0x140a44a52,0x141595a20)]:call(a,b)
for a,x in [(0x14158fb80,'add rdi, 0x58'),(0x1416ea764,'lea r9, [rdi + 0x24]'),(0x1416ea78b,'lea r9, [rdi + 8]'),(0x1416ea7b2,'lea r9, [rdi + 0x4c]'),(0x1416ea7d5,'lea r9, [rdi + 0x50]'),(0x1416ea7f8,'lea r9, [rdi + 0x54]'),(0x1416ea81b,'add rdi, 0x40'),(0x1416eaa7e,'lea r9, [rdx + 8]'),(0x1416eaaa5,'lea r9, [rdi + 0xc]'),(0x1416eaaf2,'lea r9, [rdi + 0x10]'),(0x1416eab15,'lea r9, [rdi + 0x18]'),(0x1416eab38,'lea r9, [rdi + 0x14]'),(0x1416eab5b,'add rdi, 0x19'),(0x1415996c4,'mov rax, qword ptr [rbx]'),(0x141599314,'mov eax, dword ptr [rbx]'),(0x1416e9047,'lea r9, [rdi + 4]'),(0x1416e9072,'lea r9, [rdi + 1]'),(0x1416e9095,'add rdi, 8'),(0x14158e2b0,'mov eax, dword ptr [rbx]'),(0x14158e120,'mov eax, dword ptr [rbx]'),(0x1416e7a01,'jne 0x1416e7a3f'),(0x1416e7a03,'jp 0x1416e7a3f')]:asm(a,x)
for a,s in [(0x1416ea742,b'id'),(0x1416ea76c,b'params'),(0x1416ea793,b'wall_shape'),(0x1416ea7ba,b'pivot_y'),(0x1416ea7dd,b'color_id'),(0x1416ea800,b'gable_style'),(0x1416ea823,b'flat_roof_style'),(0x1416eaa86,b'profile_01'),(0x1416eaaad,b'height_01'),(0x1416eaad0,b'tip_offset_01'),(0x1416eaafa,b'ridge_length_01'),(0x1416eab1d,b'ridge_dir'),(0x1416eab40,b'eave_length_01'),(0x1416eab63,b'flat_rim_style'),(0x1416e8f6c,b'MatchingStoneOnl'),(0x1416e8fe1,b'Override'),(0x1416e9057,b'color_id'),(0x1416e907a,b'material'),(0x1416e909d,b'pattern_id'),(0x141599f78,b'pped'),(0x141599f81,b'Step'),(0x141594c67,b'Leng'),(0x141594c96,b'Widt'),(0x1415935e6,b'NoRi'),(0x141593617,b'Ri'),(0x14158e847,b'Ston'),(0x140a44a41,b'val')]:rip(a,s)
(E/'checked-field-map.json').write_text(json.dumps({'root_fields':[['id',0,'u64'],['params',36,'RoofShapeParams28'],['wall_shape',8,'ShapeParameters28'],['pivot_y',76,'f32'],['color_id',80,'u32'],['gable_style',84,'enum byte'],['flat_roof_style',64,'enum12']], 'rips':refs,'capture_caller_span':{'start':hex(span[0]),'end':hex(span[1]),'bytes':len(raw),'sha256':hashlib.sha256(raw).hexdigest(),'reused_whole_capture':str(original),'whole_capture_sha256':sha(original)}},indent=2)+'\n')
source=json.loads((P/'source-map.json').read_text())
with tempfile.TemporaryDirectory(prefix='roof-apply-',dir='/private/tmp') as temp:
 for r in source:
  rel=Path(r['path']).relative_to(ROOT);d=Path(temp)/rel;d.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(P/r['base'],d)
  assert sha(P/r['base'])==r['base_sha256'] and sha(P/r['candidate'])==r['candidate_sha256']
  result=subprocess.run(['git','apply','--check',str(P/r['patch'])],cwd=temp,capture_output=True,text=True);assert result.returncode==0,result.stderr
  result=subprocess.run(['git','apply',str(P/r['patch'])],cwd=temp,capture_output=True,text=True);assert result.returncode==0,result.stderr
  assert d.read_bytes()==(P/r['candidate']).read_bytes()
  result=subprocess.run(['rustfmt','--edition','2024','--check',str(P/r['candidate'])],capture_output=True,text=True);assert result.returncode==0,result.stderr
result={'version':'B','range_count':len(ranges),'native_bytes':total,'decoded_instructions':count,'full_range_PE_PDB_ASM_agree':True,'caller_span_PE_reused_ASM_agree':True,'checked_original_data_refs':len(refs),'patches_apply_exact':True,'rustfmt_check':True,'production_changed':False,'native_or_JIT_executed':False,'cargo_executed':False,'rust_tests':'six new tests written, unexecuted','scope':'Roof JSON capture schema + necessary existing ShapeParameters rotation boundary'}
(E/'static-verification.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result,indent=2))
