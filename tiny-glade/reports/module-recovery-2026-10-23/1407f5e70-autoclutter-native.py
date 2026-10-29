"""原add_door_autoclutter完整执行，真实init_rng/Sip13/主题判别，无业务stub。"""
from pathlib import Path
import struct,json,hashlib,itertools,pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_MEM_INVALID
from unicorn.x86_const import *
root=Path(__file__).resolve().parents[3];raw=(root/'B版本内容/tiny-glade.rne').read_bytes();sha=hashlib.sha256(raw).hexdigest();assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
p=pefile.PE(data=raw,fast_load=True);base=p.OPTIONAL_HEADER.ImageBase;image=p.get_memory_mapped_image();u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(image)+4095)&~4095);u.mem_write(base,image)
D,S,STOP=0x20000000,0x30000000,0x200ff000;u.mem_map(D,0x100000);u.mem_map(S,0x100000)
def invalid(uc,access,address,size,value,data):
 print('native_invalid',hex(uc.reg_read(UC_X86_REG_RIP)),hex(address),size,id,state,theme)
 return False
u.hook_add(UC_HOOK_MEM_INVALID,invalid)
def q(a,v):u.mem_write(a,struct.pack('<Q',v))
def uq(a):return struct.unpack('<Q',u.mem_read(a,8))[0]
def bits(v):return list(struct.unpack('<'+'I'*len(v),struct.pack('<'+'f'*len(v),*v)))
poses=[[0.,0.,0.,1., 3.,4.,5., 1.,1.,1.],[0.1,0.2,0.3,0.92736185, -8.5,1.75,13., 2.,3.,4.]]
wall=0x100000007;rows=[]
for id,state,theme,pose in itertools.product(range(512),[3,4],[0,3],poses):
 u.mem_write(D,b'\0'*0x4000);info=D;transform=D+0x100;blueprint=D+0x200;output=D+0x1000;session=D+0x800;session_res=D+0x900
 u.mem_write(info+0x2c,struct.pack('<I',id));u.mem_write(info+0x45,bytes([state]));u.mem_write(transform,struct.pack('<10f',*pose));u.mem_write(session+0x58,bytes([theme]));q(session_res,session);q(blueprint+0xf8,8);q(blueprint+0x100,output)
 rsp=S+0xf0008;q(rsp,STOP);q(rsp+0x28,wall);q(rsp+0x30,session_res)
 for r,v in [(UC_X86_REG_RCX,info),(UC_X86_REG_RDX,transform),(UC_X86_REG_R8,blueprint),(UC_X86_REG_R9,id+700),(UC_X86_REG_RSP,rsp)]:u.reg_write(r,v)
 u.emu_start(0x1407f5e70,STOP,count=10000)
 assert u.reg_read(UC_X86_REG_RIP)==STOP
 count=uq(blueprint+0x108)
 result=[]
 if count:
  assert count==1
  result=[u.mem_read(output,1)[0],*struct.unpack('<10I',u.mem_read(output+0x10,40)),*struct.unpack('<4I',u.mem_read(output+0x40,16))]
 rows.append([id,state,theme,*bits(pose),count,*result])
out=Path(__file__).with_suffix('.txt');out.write_text('\n'.join(' '.join(f'{v:08x}'for v in row)for row in rows)+'\n')
Path(__file__).with_suffix('.json').write_text(json.dumps({'binary_sha256':sha,'original':'0x1407f5e70','cases':len(rows),'bridges':[],'nested_original':['140c96240 init_rng','14128cc30 Sip13','14092e6c0 is_autumn'],'outer_modified_id':'Info.id + 700','session_abi':'sixth physical arg is Res<ActiveSession> wrapper','fixture_sha256':hashlib.sha256(out.read_bytes()).hexdigest()},indent=2))
print(json.dumps({'cases':len(rows),'outputs':sum(row[13]for row in rows)}))
