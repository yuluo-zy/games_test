from pathlib import Path
import pefile,struct,json
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
r=Path('D:/game/reverse-engineering/tiny-glade');ev=r/'progress-20261020/phase2-toolbar-layout';pe=pefile.PE(str(r/'workspace/input/tiny-glade.exe'),fast_load=True);im=pe.get_memory_mapped_image();ib=pe.OPTIONAL_HEADER.ImageBase;out=[]
for demo in [0,1]:
 for tag in range(12):
  for opt in [None,0.75]:
   u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(ib,(len(im)+4095)&~4095);u.mem_write(ib,im);u.mem_map(0x100000,0x200000);heap=[0x200000];entry=0x18fff8;bundle=[];styles=[]
   def q(a):return struct.unpack('<Q',u.mem_read(a,8))[0]
   def ret(v=0):
    sp=u.reg_read(UC_X86_REG_RSP);u.reg_write(UC_X86_REG_RAX,v);u.reg_write(UC_X86_REG_RIP,q(sp));u.reg_write(UC_X86_REG_RSP,sp+8)
   def hook(u,a,size,user):
    if a==0x140fe7600:
     styles.append(bytes(u.mem_read(u.reg_read(UC_X86_REG_R8),344)).hex());u.mem_write(u.reg_read(UC_X86_REG_RCX),struct.pack('<III',4,31,1));ret()
    elif a==0x140fe7790:u.mem_write(u.reg_read(UC_X86_REG_RCX),struct.pack('<I',4));ret()
    elif a==0x140613c10:
     n=u.reg_read(UC_X86_REG_RCX);p=heap[0];heap[0]+=(n+31)&~31;ret(p)
    elif a==0x142923c00:
     dst=u.reg_read(UC_X86_REG_RCX);src=u.reg_read(UC_X86_REG_RDX);n=u.reg_read(UC_X86_REG_R8);u.mem_write(dst,bytes(u.mem_read(src,n)));ret(dst)
    elif a==0x140fe0470:
     bundle.append(bytes(u.mem_read(u.reg_read(UC_X86_REG_R8),104)).hex());u.mem_write(u.reg_read(UC_X86_REG_RCX),struct.pack('<QQQQQ',0,0x140000,0,0,0x100000007));ret()
    elif not 0x1426235d0<=a<0x142623a94:raise RuntimeError(hex(a))
   u.hook_add(UC_HOOK_CODE,hook)
   u.mem_write(0x14362c098,struct.pack('<Q',0x110000));u.mem_write(0x110000,bytes([demo]));u.mem_write(0x120000,b'toolbar/probe');u.mem_write(0x130000,bytes([tag]));u.mem_write(0x140000,struct.pack('<QQQ',0x10000,0x150000,0));u.mem_write(entry+0x28,struct.pack('<QQQI',13,0x140000,0x130000,int(opt is not None)));u.mem_write(entry+0x48,struct.pack('<f',opt or 0));
   for reg,val in [(UC_X86_REG_RCX,0x160000),(UC_X86_REG_RDX,0x170000),(UC_X86_REG_R8,tag),(UC_X86_REG_R9,0x120000),(UC_X86_REG_RSP,entry)]:u.reg_write(reg,val)
   u.emu_start(0x1426235d0,0x142623a94,count=5000);record={'demo':demo,'tag':tag,'size_mult':opt,'style':styles[0],'bundle':bundle[0],'commands':bytes(u.mem_read(0x150000,q(0x140010))).hex()};out.append(record)
(ev/'native-category-constructors.json').write_text(json.dumps(out,indent=2));print('captured',len(out),'complete original root bundle + four insertion records')
