from pathlib import Path
import json, hashlib, subprocess, tempfile, shutil
import pefile, capstone
ROOT=Path('/Users/yuluo/Desktop/game'); P=Path(__file__).resolve().parent; E=P/'evidence'
PE=ROOT/'ljxsj_92385/Tiny Glade/tiny-glade.rne'; PDB=PE.with_name('tiny_glade.pdb')
def sha(f):
 h=hashlib.sha256()
 with f.open('rb') as r:
  for b in iter(lambda:r.read(1<<20),b''):h.update(b)
 return h.hexdigest()
assert sha(PE)=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
assert sha(PDB)=='2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911'
old=P.parent/'frozen-inputs.json'; assert sha(old)=='e690557ad3af91006228a88fa5182858a1a7d59974f1806bf77fdabdec0824ee'
for x in json.loads(old.read_text())['inputs']:
 f=Path(x['path']); assert f.stat().st_size==x['size'] and sha(f)==x['sha256']
assert sha(P/'base.rs')=='218d62d0e86c94bd40396254489667fb82ee13bf13e742847807109b3a43bf4e'
pe=pefile.PE(str(PE),fast_load=True);md=capstone.Cs(capstone.CS_ARCH_X86,capstone.CS_MODE_64);md.detail=True
ranges=json.loads((E/'ranges.json').read_text());ins={};n=0;total=0
for row in ranges.values():
 va=int(row['va'],16); raw=(E/f'{va:x}.bin').read_bytes();pdb=json.loads((E/f'{va:x}.pdb.json').read_text());ii=list(md.disasm(raw,va))
 assert raw==pe.get_data(va-0x140000000,len(raw)) and hashlib.sha256(raw).hexdigest()==row['sha256']
 assert pdb['code_size']==len(raw)==row['bytes'] and pdb['name']==row['name']
 assert len(ii)==row['instructions'] and sum(i.size for i in ii)==len(raw)
 assert (E/f'{va:x}.asm').read_text()=='\n'.join(f'{i.address:#x} {i.bytes.hex()} {i.mnemonic} {i.op_str}' for i in ii)+'\n'
 ins.update({i.address:i for i in ii});n+=len(ii);total+=len(raw)
checks=[]
def asm(a,s):
 i=ins[a];assert i.mnemonic+' '+i.op_str==s,(hex(a),i.mnemonic+' '+i.op_str,s);checks.append({'site':hex(a),'asm':s})
def call(a,b):asm(a,'call '+hex(b))
def rip(a,b):
 i=ins[a];o=next(o for o in i.operands if o.type==capstone.x86.X86_OP_MEM and o.mem.base==capstone.x86.X86_REG_RIP);va=i.address+i.size+o.mem.disp
 assert pe.get_data(va-0x140000000,len(b))==b,(hex(a),hex(va),b);checks.append({'site':hex(a),'target':hex(va),'bytes':b.hex()})
for a,b in [(0x1415c5638,0x14159d190),(0x1415c56fd,0x14159cfa0),(0x1415c5766,0x1416ec620),(0x1415c57df,0x14159a370),(0x1415c58a1,0x1415a1b30),(0x1415c593a,0x1416e8680),(0x1415c5998,0x1416e91c0),(0x1415a1b80,0x1415c7ed0),(0x1415a1bd7,0x1415ab620),(0x1415c8028,0x1416eadf0),(0x1415d9757,0x1416e99c0),(0x1416ec753,0x141282e60),(0x1416ec822,0x141283000),(0x141282e93,0x1416ecda0),(0x141283033,0x1416ed020),(0x1416ece03,0x141284050),(0x1416ece46,0x1412830a0),(0x1416ed092,0x141283ad0),(0x1416ec89a,0x140c97540),(0x14128335e,0x141283740)]:call(a,b)
for a,s in [
 (0x1415c54c4,'mov byte ptr [rbp + 0x126], 2'),(0x1415c54cb,'mov al, 3'),
 (0x1415c5ac3,'cmp al, 3'),(0x1415c5ac8,'mov eax, 1'),(0x1415c5acd,'cmovne eax, ecx'),
 (0x1415c5b81,'and al, 1'),(0x1415a956a,'xor edi, edi'),
 (0x1415c5b34,'mov qword ptr [rdx], r8'),(0x1415c5b37,'mov dword ptr [rdx + 8], r9d'),
 (0x1415c5b46,'mov eax, dword ptr [rbp + 0x94]'),(0x1415c5b4c,'mov dword ptr [rdx + 0x24], eax'),
 (0x1415c5b17,'mov byte ptr [rdx + 0x40], al'),(0x1415c5b6c,'movss dword ptr [rdx + 0x4c], xmm6'),
 (0x1415c5b77,'mov dword ptr [rdx + 0x50], eax'),(0x1415c5b83,'mov byte ptr [rdx + 0x54], al'),
 (0x1415c7f46,'mov byte ptr [rbp + 0x96], 2'),(0x1415c7f5f,'mov byte ptr [rbp + 0x97], 2'),
 (0x1415c8366,'test byte ptr [rbp + 0x64], 1'),(0x1415c8370,'test byte ptr [rbp + 0x68], 1'),
 (0x1415c837d,'test byte ptr [rbp + 0x6c], 1'),(0x1415c83c5,'and dl, 1'),(0x1415c83cf,'and cl, 1'),
 (0x1415c83d2,'movss dword ptr [r13 + 4], xmm9'),(0x1415c83d8,'mov qword ptr [r13 + 8], rax'),
 (0x1415c83dc,'movss dword ptr [r13 + 0x10], xmm8'),(0x1415c83e2,'movss dword ptr [r13 + 0x14], xmm7'),
 (0x1415c83e8,'movss dword ptr [r13 + 0x18], xmm6'),(0x1415c83ee,'mov byte ptr [r13 + 0x1c], dl'),(0x1415c83f2,'mov byte ptr [r13 + 0x1d], cl'),
 (0x1415d94b8,'cmp eax, 5'),(0x1415d98e2,'xor ecx, ecx'),(0x1415d98eb,'cmove rdx, rcx'),
 (0x141284574,'movss dword ptr [r15 + 4], xmm6'),(0x14128457a,'mov qword ptr [r15 + 8], rax'),(0x14128457e,'movss dword ptr [r15 + 0x10], xmm10'),(0x141284584,'movss dword ptr [r15 + 0x14], xmm12'),(0x14128458a,'movss dword ptr [r15 + 0x18], xmm13'),
 (0x140c97543,'movss dword ptr [rcx], xmm1'),(0x140c97547,'movss dword ptr [rcx + 4], xmm2'),(0x140c9754c,'movss dword ptr [rcx + 8], xmm3'),(0x140c97551,'mov dword ptr [rcx + 0xc], 0'),(0x1416ec880,'test ebx, ebx'),(0x1416ec882,'jne 0x1416ec90a')]:asm(a,s)
for a,b in [(0x1415c83a4,bytes.fromhex('cdcc4c3e')),(0x1415c83b6,bytes.fromhex('cdcccc3e')),(0x1415d3fc3,b'Stepped'),(0x1415d3ff6,b'Overhang'),(0x1415d2713,b'Stone'),(0x1415d274a,b'Wood'),(0x1415d2403,b'Width'),(0x1415d243a,b'Length'),(0x1415d1a24,b'Ri'),(0x1415d1a5d,b'NoRi'),(0x141283cd1,b'origin'),(0x141283cef,b'radius')]:rip(a,b)
# The rectangular reader performs no constructor call anywhere in its full PDB range.
rect_calls=json.loads((E/'141284050.calls.json').read_text()); assert not any(any('::new' in a['name'] for a in c['aliases']) for c in rect_calls)
source=[]
for base,can,path,patch in [('base.rs','candidate.rs','games_test/tiny-glade/crates/country-core/src/resources/roofs/roof_shape.rs','candidate.patch'),('shape-base.rs','shape-candidate.rs','games_test/tiny-glade/crates/country-core/src/systems/wall/valid_enclosure/mod.rs','integration-owner.patch')]:
 source.append({'path':path,'base':base,'candidate':can,'patch':patch,'base_sha256':sha(P/base),'candidate_sha256':sha(P/can)})
with tempfile.TemporaryDirectory(prefix='roof-reader-apply-',dir='/private/tmp') as t:
 for s in source:
  dest=Path(t)/s['path'];dest.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(P/s['base'],dest)
  r=subprocess.run(['git','apply',str(P/s['patch'])],cwd=t,capture_output=True,text=True);assert r.returncode==0,r.stderr
  assert dest.read_bytes()==(P/s['candidate']).read_bytes()
  r=subprocess.run(['rustfmt','--edition','2024','--check',str(P/s['candidate'])],capture_output=True,text=True);assert r.returncode==0,r.stderr
(P/'source-map.json').write_text(json.dumps(source,indent=2)+'\n')
(E/'checked-semantics.json').write_text(json.dumps(checks,indent=2)+'\n')
result={'target':'B','full_PDB_ranges':len(ranges),'native_bytes':total,'instructions':n,'PE_PDB_ASM_agree':True,'semantic_assertions':len(checks),'writer139_unchanged':True,'patches_apply_exact':True,'rustfmt_check':True,'production_changed':False,'cargo_executed':False,'native_or_JIT_executed':False,'rust_tests':'10 authored, unexecuted','scope':'Roof reader JSON-value projection plus necessary canonical ShapeParameters compatibility'}
(P/'validation.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result,indent=2))
