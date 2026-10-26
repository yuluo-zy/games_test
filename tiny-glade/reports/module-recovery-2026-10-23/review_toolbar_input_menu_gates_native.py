from pathlib import Path
import hashlib,json,struct,pefile,itertools
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
root=Path(__file__).resolve().parents[4];raw=(root/'B版本内容/tiny-glade.rne').read_bytes();assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase;im=pe.get_memory_mapped_image();u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(im)+4095)&~4095);u.mem_write(base,im)
data,stack,stop=0x20000000,0x30000000,0x2000f000;u.mem_map(data,0x10000);u.mem_map(stack,0x10000)
held=set();bridges={0x14089aef0,0x14089a890,0x14089ab50,0x14089a9f0}
def hook(uc,addr,size,ud):
 if addr in bridges:
  value=0
  if addr==0x14089aef0:
   key=struct.unpack('<I',uc.mem_read(uc.reg_read(UC_X86_REG_RDX),4))[0];value=data+0x8000 if key in held else 0
  rsp=uc.reg_read(UC_X86_REG_RSP);ret=struct.unpack('<Q',uc.mem_read(rsp,8))[0];uc.reg_write(UC_X86_REG_RAX,value);uc.reg_write(UC_X86_REG_RSP,rsp+8);uc.reg_write(UC_X86_REG_RIP,ret)
u.hook_add(UC_HOOK_CODE,hook)
def qw(addr,v):u.mem_write(addr,struct.pack('<Q',v))
def reader(addr,command=None):
 cursor,events,items=addr+0x20,addr+0x80,addr+0x100;qw(addr,cursor);qw(addr+8,events)
 if command is not None:
  qw(events+8,items);qw(events+0x10,1);qw(events+0x38,1);qw(events+0x40,1);u.mem_write(items,struct.pack('<QB7x',0,command))
def run(tag,flag,slot,key,modifier=False,demo=False,history=None):
 global held
 held={key} if key else set()
 if modifier:held.add(0x3d)
 u.mem_write(data,bytes(0xf000));qw(data,data+0x1000);qw(data+0x40,data+0x2000);qw(data+0x80,data+0x3000);qw(data+0xc0,0)
 # stack params: menu, values, tracker, wheel, MouseMoved, cursorDelta, history, profile
 qw(data+0x100,data+0x3500);u.mem_write(data+0x3500,struct.pack('<QB7x',tag,flag))
 for param,body in [(0x140,0x4000),(0x180,0x4600)]:qw(data+param,data+body);qw(data+param+0x10,data+0x4f00)
 reader(data+0x1c0);reader(data+0x200);reader(data+0x240,history);qw(data+0x280,data+0x5000)
 profile=bytearray(456)
 for i in range(37):struct.pack_into('<I',profile,i*12,195)
 if slot>=0:struct.pack_into('<III',profile,slot*12,20,0,0)
 for i in range(3):struct.pack_into('<HH',profile,444+i*4,6,0)
 u.mem_write(data+0x5000,bytes(profile));u.mem_write(data+0x1028,struct.pack('<f',0.02));u.mem_write(0x143634cd8,bytes([demo]))
 rsp=stack+0xf008;qw(rsp,stop)
 for off,value in [(0x28,data+0x100),(0x30,data+0x140),(0x38,data+0x180),(0x40,data+0x1c0),(0x48,data+0x4500),(0x50,data+0x200),(0x58,data+0x240),(0x60,data+0x280)]:qw(rsp+off,value)
 for reg,val in [(UC_X86_REG_RSP,rsp),(UC_X86_REG_RCX,data),(UC_X86_REG_RDX,data+0x40),(UC_X86_REG_R8,data+0x80),(UC_X86_REG_R9,data+0xc0)]:u.reg_write(reg,val)
 u.emu_start(0x1417cd0a0,stop,count=20000);assert u.reg_read(UC_X86_REG_RIP)==stop,hex(u.reg_read(UC_X86_REG_RIP))
 digital=list(u.mem_read(data+0x4088,40));bitmap=sum(bool(v)<<i for i,v in enumerate(digital));analog=struct.unpack('<12I',u.mem_read(data+0x4058,48))
 return bitmap,analog
rows=[]
for tag,flag in itertools.product(range(11),range(2)):
 for slot in range(37):
  bitmap,_=run(tag,flag,slot,20);rows.append((tag,flag,slot,20,False,False,-1,bitmap))
 for key in [115,0x35,0x40,0xab,0x25,0x23,0x29,0xa6,0xa7]:
  for demo in [False,True]:
   bitmap,_=run(tag,flag,-1,key,False,demo);rows.append((tag,flag,-1,key,False,demo,-1,bitmap))
 for command in [0,1,2]:
  bitmap,_=run(tag,flag,-1,0,False,False,command);rows.append((tag,flag,-1,0,False,False,command,bitmap))
 for modifier in [False,True]:
  bitmap,_=run(tag,flag,25,20,modifier);rows.append((tag,flag,25,20,modifier,False,-1,bitmap))
fixture=root/'tiny-glade/tests/fixtures/native/toolbar-input-menu-gates.ron';fixture.write_text('[\n'+''.join(f'({tag},{flag},{slot},{key},{str(mod).lower()},{str(demo).lower()},{history},{bitmap}),\n' for tag,flag,slot,key,mod,demo,history,bitmap in rows)+']\n')
report={'binary_sha256':hashlib.sha256(raw).hexdigest(),'function':'0x1417cd0a0','exit':'0x1417ce33e','whole_function':True,'bridges':[{'address':hex(a),'boundary':'locked Bevy/hashbrown ButtonInput pressed-set lookup, resolved input set; mouse set empty'} for a in sorted(bridges)],'cases':len(rows),'fixture_sha256':hashlib.sha256(fixture.read_bytes()).hexdigest(),'scope':'原函数全部执行；固定无Steam、无mouse/wheel/cursor，11 menu tags × 2 flags下37键字段、硬编码Escape与Demo键、截图修饰与History事件。比较40位动作bitmap；不宣称Hashbrown容器、Steam或OS全链差分。'}
(root/'tiny-glade/reports/business/2026-10-23-module-recovery/toolbar-input-menu-gates-native.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n');print(len(rows),'cases');print('settings confirm',run(7,0,29,20)[0]);print('settings escape',run(7,0,-1,115)[0])
