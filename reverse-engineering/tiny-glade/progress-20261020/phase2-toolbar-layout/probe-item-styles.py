from pathlib import Path
import pefile,struct,json
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
p=Path('D:/game/reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe'); ev=Path('D:/game/reverse-engineering/tiny-glade/progress-20261020/phase2-toolbar-layout');pe=pefile.PE(str(p),fast_load=True);im=pe.get_memory_mapped_image();u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(pe.OPTIONAL_HEADER.ImageBase,(len(im)+4095)&~4095);u.mem_write(pe.OPTIONAL_HEADER.ImageBase,im);u.mem_map(0x100000,0x100000)
styles={}
def hook(u,addr,size,data):
 if addr==0x140fe7600:
  styles[str(tag)]=bytes(u.mem_read(u.reg_read(UC_X86_REG_R8),344)).hex();u.emu_stop()
old_hook=u.hook_add(UC_HOOK_CODE,hook)
for tag in [0,4,5]:
 u.mem_write(0x100000,bytes([tag])+bytes(63));u.mem_write(0x180000,bytes(0x10000));u.reg_write(UC_X86_REG_RSP,0x18fff8);u.reg_write(UC_X86_REG_R9,0x100000);u.reg_write(UC_X86_REG_RCX,0x110000);u.emu_start(0x141e4dc40,0,count=1000)
for k,v in styles.items():
 b=bytes.fromhex(v);print(k,'gap',struct.unpack_from('<IfIf',b,0xa8),'size',b[0xd8:0xe8].hex(),'end',b[0x148:].hex())
(ev/'native-item-styles.json').write_text(json.dumps(styles,indent=2))

tag=8
u.mem_write(0x100000,struct.pack('<QQQ',1,0x120000,1));u.mem_write(0x120000,bytes(32));u.mem_write(0x110000,struct.pack('<QQQ',1,0x130000,1));u.mem_write(0x130000,bytes(24)+bytes([5])+bytes(7));u.reg_write(UC_X86_REG_RSP,0x18fff8);u.reg_write(UC_X86_REG_RCX,0x110000);u.reg_write(UC_X86_REG_RDX,0x140000);u.reg_write(UC_X86_REG_R8,5);u.reg_write(UC_X86_REG_R9,0x100000);u.emu_start(0x141e4d1e0,0,count=1000)
b=bytes.fromhex(styles['8']);print('contextual-menu', 'gap',struct.unpack_from('<IfIf',b,0xa8),'end',b[0x148:].hex());(ev/'native-item-styles.json').write_text(json.dumps(styles,indent=2))

root_styles=[]
def setup_hook(u,addr,size,data):
 if addr==0x140fe7600:
  root_styles.append(bytes(u.mem_read(u.reg_read(UC_X86_REG_R8),344)))
  dest=u.reg_read(UC_X86_REG_RCX);u.mem_write(dest,struct.pack('<III',4,len(root_styles),1))
  sp=u.reg_read(UC_X86_REG_RSP);u.reg_write(UC_X86_REG_RIP,struct.unpack('<Q',u.mem_read(sp,8))[0]);u.reg_write(UC_X86_REG_RSP,sp+8)
 elif addr==0x140fe7790:
  dest=u.reg_read(UC_X86_REG_RCX);u.mem_write(dest,struct.pack('<I',4))
  sp=u.reg_read(UC_X86_REG_RSP);u.reg_write(UC_X86_REG_RIP,struct.unpack('<Q',u.mem_read(sp,8))[0]);u.reg_write(UC_X86_REG_RSP,sp+8)
u.hook_del(old_hook)
u.hook_add(UC_HOOK_CODE,setup_hook);u.mem_write(0x110098,struct.pack('<QQQ',64,0x140000,0));u.mem_write(0x18fff8,struct.pack('<Q',0x100000));u.reg_write(UC_X86_REG_RSP,0x18fff8);u.reg_write(UC_X86_REG_RCX,0x120000);u.reg_write(UC_X86_REG_RDX,0x110000);u.emu_start(0x141e4ccb0,0x100000,count=2000)
for i,b in enumerate(root_styles): (ev/f'native-setup-root-style-{i}.bin').write_bytes(b)
print('root-styles',len(root_styles))

