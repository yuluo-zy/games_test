"""原main两独立写者、developer getter、maintain入口门、Place u45/墙metadata和glade判定。"""
from pathlib import Path
import hashlib,json,struct,pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
p=Path(__file__).resolve();root=p.parents[3];raw=(root/'B版本内容/tiny-glade.rne').read_bytes()
assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(pe.OPTIONAL_HEADER.SizeOfImage+4095)&~4095);u.mem_write(base,pe.get_memory_mapped_image());u.mem_map(0x200000,0x100000)
def writeq(at,v):u.mem_write(at,struct.pack('<Q',v))
def reg(r):return u.reg_read(r)
writeq(0x14362c098,0x143634d90);writeq(0x14362c020,0x143634cd8)
rows=[];shapes=[]
for demo in [0,1]:
 for hotkeys in [0,1]:
  frame=0x220000;stack=0x2f0008;stop=0x2ff000
  u.mem_write(frame+0x290+0xe6,bytes([demo]));u.mem_write(frame+0x290+0xe2,bytes([hotkeys]));u.reg_write(UC_X86_REG_RBP,frame)
  u.emu_start(0x1403dcf6a,0x1403dcf7a,count=10);u.emu_start(0x1403dd616,0x1403dd626,count=10)
  u.mem_write(stack,struct.pack('<Q',stop));u.reg_write(UC_X86_REG_RSP,stack);u.emu_start(0x14090e070,stop,count=10);getter=reg(UC_X86_REG_RAX)&255
  u.mem_write(stack,struct.pack('<Q',stop));u.reg_write(UC_X86_REG_RSP,stack);entered=[]
  def hook(mu,a,n,data):
   if a==0x140c16a00:entered.append(True);mu.emu_stop()
  h=u.hook_add(UC_HOOK_CODE,hook);u.emu_start(0x140806110,stop,count=100);u.hook_del(h)
  u.reg_write(UC_X86_REG_R11,0);u.emu_start(0x140a20c9a,0x140a20ca9,count=10);place=reg(UC_X86_REG_R11)&255
  u.reg_write(UC_X86_REG_RSP,stack);u.emu_start(0x14085a37c,0x14085a393,count=10);shape=reg(UC_X86_REG_RAX)&255
  rows.append(dict(demo=demo,internal_hotkeys=hotkeys,demo_byte=list(u.mem_read(0x143634d90,1))[0],hotkey_byte=list(u.mem_read(0x143634cd8,1))[0],developer_getter=getter,maintain_enters_business=bool(entered),place_u45=place,new_wall_source69=shape))
  for x,z in [(0.,0.),(65.,65.),(69.,69.),(70.,0.),(70.,70.),(71.,0.),(0.,71.),(float('nan'),0.),(float('inf'),0.),(-0.,0.)]:
   u.mem_write(stack,struct.pack('<Q',stop));u.reg_write(UC_X86_REG_RSP,stack)
   xb,zb=[struct.unpack('<I',struct.pack('<f',v))[0]for v in [x,z]];u.reg_write(UC_X86_REG_XMM0,xb);u.reg_write(UC_X86_REG_XMM1,zb);u.emu_start(0x14090dfe0,stop,count=100)
   shapes.append(dict(demo=demo,internal_hotkeys=hotkeys,x=xb,z=zb,within=reg(UC_X86_REG_RAX)&255))
out=p.with_suffix('.json')
with out.open('x')as f:json.dump(dict(scope=__doc__,rne_sha256=hashlib.sha256(raw).hexdigest(),loader_binding='Only two original IAT/data pointer slots are bound to matching PDB static addresses; main writer instructions run unchanged.',rows=rows,glade_shapes=shapes),f,indent=2)
fixture=p.with_name('m01-demo-gates-native-cases.rs')
with fixture.open('x')as f:
 f.write('// 原main独立写者→原maintain入口门，未执行后续业务。\nconst ORIGINAL_DEMO_GATES:&[(bool,bool,bool,u8,u8)]=&[\n')
 for o in rows:f.write('(%s,%s,%s,%d,%d),\n'%(str(bool(o['demo'])).lower(),str(bool(o['internal_hotkeys'])).lower(),str(o['maintain_enters_business']).lower(),o['place_u45'],o['new_wall_source69']))
 f.write('];\n')
print(json.dumps(dict(rows=rows,glade_shapes=len(shapes)),indent=2))
