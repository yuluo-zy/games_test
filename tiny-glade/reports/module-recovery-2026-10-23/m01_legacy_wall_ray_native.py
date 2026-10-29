"""执行B原legacy参数包装与完整/simple射线predicate，库边界只桥接collider查询。"""
from pathlib import Path
import struct,json,hashlib,itertools
import pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
root=Path(__file__).resolve().parents[3];raw=(root/'B版本内容/tiny-glade.rne').read_bytes()
sha=hashlib.sha256(raw).hexdigest();assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase;blob=pe.get_memory_mapped_image()
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(blob)+4095)&~4095);u.mem_write(base,blob)
D,S,STOP=0x20000000,0x30000000,0x200ff000;u.mem_map(D,0x100000);u.mem_map(S,0x100000)
def q(a,x):u.mem_write(a,struct.pack('<Q',x))
def uq(a):return struct.unpack('<Q',u.mem_read(a,8))[0]
def dw(a):return struct.unpack('<I',u.mem_read(a,4))[0]
def ret(value):
 sp=u.reg_read(UC_X86_REG_RSP);u.reg_write(UC_X86_REG_RIP,uq(sp));u.reg_write(UC_X86_REG_RSP,sp+8);u.reg_write(UC_X86_REG_RAX,value)
def hook(uc,a,size,_):
 global panic,capture
 if a==0x140b93940:ret(0 if mode=='missing_collider'else D+0x3000)
 elif a in [0x1428d8725,0x1428d9310]:panic=hex(a);uc.emu_stop()
 elif a==0x1408cdc60:
  sp=uc.reg_read(UC_X86_REG_RSP)
  capture={'option_tag':uc.reg_read(UC_X86_REG_R9),'wall':uq(sp+0x28),'max_toi_bits':dw(sp+0x30),'mask':dw(sp+0x38)}
  out=uc.reg_read(UC_X86_REG_RCX);u.mem_write(out,bytes(0x40));u.mem_write(out+0x34,struct.pack('<I',2));ret(out)
u.hook_add(UC_HOOK_CODE,hook)
def predicate(kind,option_tag,source_wall,excluded_wall,problem,entry):
 global mode,panic,capture
 mode=problem;panic=None;capture=None;u.mem_write(D,bytes(0x10000));u.mem_write(S,bytes(0x100000))
 world,entries,collider,env,option=D+0x1000,D+0x2000,D+0x3000,D+0x4000,D+0x5000
 q(world+8,0 if problem=='missing_registry'else entries);q(world+16,3)
 q(collider,((5 if problem=='stale_generation'else 3)<<32)|(9 if problem=='bad_index'else 1))
 e=entries+40;u.mem_write(e,bytes([kind]));q(e+8,source_wall);u.mem_write(e+32,struct.pack('<I',3))
 q(env,world);q(env+8,D+0x6000);q(env+16,option);q(option,option_tag);q(option+8,excluded_wall)
 sp=S+0xf0008;q(sp,STOP)
 for reg,value in [(UC_X86_REG_RCX,env),(UC_X86_REG_RDX,0),(UC_X86_REG_R8,collider),(UC_X86_REG_RSP,sp)]:u.reg_write(reg,value)
 u.emu_start(entry,STOP,count=10000)
 assert panic or u.reg_read(UC_X86_REG_RIP)==STOP
 return {'query':'full'if entry==0x1408cab80 else 'simple','kind':kind,'option_tag':option_tag,'source_wall':source_wall,'excluded_wall':excluded_wall,'problem':problem,'panic':panic,'allowed':None if panic else bool(u.reg_read(UC_X86_REG_RAX)&255)}
rows=[];wall=0x123456789abcdef0;other=0xfedcba9876543210
for entry in [0x1408cab80,0x1408ca9a0]:
 for kind,tag,excluded in itertools.product(range(17),[0,1],[wall,other]):rows.append(predicate(kind,tag,wall,excluded,None,entry))
 for problem in ['missing_collider','missing_registry','bad_index','stale_generation']:rows.append(predicate(0,1,wall,wall,problem,entry))
wrappers=[]
for tag,excluded,extra in itertools.product([0,1],[0,wall],[0,1,8,1<<9,1<<16,0xffffffff]):
 mode=None;panic=None;capture=None;u.mem_write(D,bytes(0x10000));u.mem_write(S,bytes(0x100000));sp=S+0xf0008;q(sp,STOP)
 q(sp+0x28,excluded);q(sp+0x30,extra);out=D+0x8000;u.mem_write(out,bytes([0xa5])*64)
 for reg,value in [(UC_X86_REG_RCX,out),(UC_X86_REG_RDX,D+0x1000),(UC_X86_REG_R8,D+0x2000),(UC_X86_REG_R9,tag),(UC_X86_REG_RSP,sp)]:u.reg_write(reg,value)
 u.emu_start(0x1408cc770,STOP,count=10000)
 assert not panic and capture is not None and u.reg_read(UC_X86_REG_RIP)==STOP
 wrappers.append({'input':{'option_tag':tag,'wall':excluded,'additional_mask':extra},'callee':capture,'legacy_miss_bits':dw(out+0x30)})
report={'binary_sha256':sha,'predicate_entries':{'full':'0x1408cab80','simple':'0x1408ca9a0'},'wrapper_entry':'0x1408cc770','bridges':['ColliderSet::get_mut_internal','Rapier::raycast miss for wrapper capture','panic termination'],'rows':rows,'wrappers':wrappers}
Path(__file__).with_name('m01-legacy-wall-ray-native.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps({'predicate_cases':len(rows),'predicate_panics':sum(bool(r['panic'])for r in rows),'wrapper_cases':len(wrappers)}))
