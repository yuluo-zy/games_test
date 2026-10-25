from pathlib import Path
import hashlib,json,random,struct
import pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
ROOT=Path('/Users/yuluo/Desktop/game')
B=ROOT/'ljxsj_92385/Tiny Glade/tiny-glade.rne'
assert hashlib.sha256(B.read_bytes()).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
p=pefile.PE(str(B),fast_load=True)
u=Uc(UC_ARCH_X86,UC_MODE_64)
base=p.OPTIONAL_HEADER.ImageBase
u.mem_map(base,(p.OPTIONAL_HEADER.SizeOfImage+4095)&~4095)
for s in p.sections:
 u.mem_write(base+s.VirtualAddress,s.get_data())
u.mem_map(0x100000,0x100000)
rbp=0x180000;rsp=0x17f000;rows=0x110000;data=0x120000;instances=0x130000
u.reg_write(UC_X86_REG_RBP,rbp);u.reg_write(UC_X86_REG_RSP,rsp)
def q(a,v):u.mem_write(a,struct.pack('<Q',v))
def getq(a):return struct.unpack('<Q',u.mem_read(a,8))[0]
observed=[]
projected=[]
half_inputs=[]
def hook(uc,ip,size,user):
 if ip==0x141556130: # resolved PrefabName::as_name_ref dependency
  uc.reg_write(UC_X86_REG_RAX,0x140000);uc.reg_write(UC_X86_REG_RDX,6)
  sp=uc.reg_read(UC_X86_REG_RSP);ret=getq(sp);uc.reg_write(UC_X86_REG_RSP,sp+8);uc.reg_write(UC_X86_REG_RIP,ret)
 elif ip==0x1408b9f20: # half conversion dependency: observe input, return a resolved half
  half_inputs.append(uc.reg_read(UC_X86_REG_XMM0)&0xffffffff)
  uc.reg_write(UC_X86_REG_RAX,0)
  sp=uc.reg_read(UC_X86_REG_RSP);ret=getq(sp);uc.reg_write(UC_X86_REG_RSP,sp+8);uc.reg_write(UC_X86_REG_RIP,ret)
 elif ip==0x14156d0d0:
  sp=uc.reg_read(UC_X86_REG_RSP);ptr=getq(sp+0x28);cull=uc.reg_read(UC_X86_REG_R8)
  projected.append({'placement_bits':list(struct.unpack('<12I',uc.mem_read(ptr,48))),'cull_position_bits':list(struct.unpack('<3I',uc.mem_read(cull,12))),'radius_input_bits':half_inputs[-1]})
  uc.emu_stop()
 elif ip==0x140841b00:
  # call pushed return address: Windows fifth argument at callee rsp+0x28
  ptr=getq(uc.reg_read(UC_X86_REG_RSP)+0x28)
  observed.append((ptr,bytes(uc.mem_read(ptr,48))));uc.emu_stop()
u.hook_add(UC_HOOK_CODE,hook)
rng=random.Random(0x843740);out=[]
for i in range(64):
 index=[0,1,3,7][i%4];slot=i%3
 bits=[rng.getrandbits(32) for _ in range(12)];record=struct.pack('<12I',*bits)
 u.mem_write(data+index*48,record)
 q(rbp+0x170,slot);q(rbp+0x60,rows);q(rbp+0x120,instances);q(rbp+0xd8,0x150000);q(rbp+0x180,data)
 q(rows+slot*16,123);u.mem_write(rows+slot*16+8,struct.pack('<I',index))
 u.emu_start(0x140843ea5,0x140843eeb,count=100)
 assert getq(rbp+0x188)==data+index*48
 q(rbp+0x168,0x160000);q(rbp+0x58,0x170000);q(rbp+0x18,0x175000)
 u.reg_write(UC_X86_REG_RSP,rsp)
 u.emu_start(0x14084415c,0x140844193,count=100)
 ptr,actual=observed[-1];assert ptr==data+index*48 and actual==record
 out.append({'selected_index':index,'placement_bits':bits,'forwarded_bits':list(struct.unpack('<12I',actual))})
callee=[]
for i,row in enumerate(out):
 record=struct.pack('<12I',*row['placement_bits']);u.mem_write(data,record)
 q(rbp+0x128,0x1a0000);q(rbp+0x120,data);q(0x1a0000,0x1b0000);q(0x1a0010,0x1c0000);u.mem_write(0x1a001c,struct.pack('<I',123))
 u.mem_write(0x160000,struct.pack('<IB3x',17,2));u.reg_write(UC_X86_REG_RDI,0x160000);u.reg_write(UC_X86_REG_RBX,1);u.reg_write(UC_X86_REG_RSP,rsp)
 u.emu_start(0x140841c3b,0x140841cc2,count=100)
 actual=projected[-1];assert actual['placement_bits']==row['placement_bits'] and actual['cull_position_bits']==row['placement_bits'][:3] and actual['radius_input_bits']==row['placement_bits'][3]
 callee.append(actual)
print(json.dumps({'target':'B','binary_sha256':hashlib.sha256(B.read_bytes()).hexdigest(),'query_projection_range':['0x140843ea5','0x140843eeb'],'forward_range':['0x14084415c','0x140844193'],'callee_range':['0x140841c3b','0x140841cc2'],'cases':out,'callee_cases':callee,'boundary':'Resolved query row and atlas entry only; no hash/query discovery, allocation, hot-reload filtering, GPU execution or C behavior accepted. Name conversion and half conversion are bridged; original instructions select a48-byte record, forward it, copy cull XYZ and supply exact scale bits to the half dependency. Half conversion output itself is not accepted here.'},indent=2))
