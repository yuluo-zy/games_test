"""Native event assembly from the actual captured rotate requests.

Archive/storage dependencies are hooks; all original112B request->136B event
assembly instructions execute unchanged. Padding is poisoned and not compared.
"""
from pathlib import Path
import hashlib,json,struct
import pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
p=Path(__file__).parent
raw=Path('D:/game/reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe').read_bytes()
assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase;image=pe.get_memory_mapped_image()
rows=json.loads((p/'rotate-native-ops.json').read_text())
for row in rows:
 if 'op' not in row:continue
 uc=Uc(UC_ARCH_X86,UC_MODE_64);uc.mem_map(base,(len(image)+4095)&~4095);uc.mem_write(base,image)
 uc.mem_map(0x20000000,0x10000);uc.mem_map(0x21000000,0x10000);uc.mem_map(0x22000000,0x1000)
 op,location,sp=0x20001000,0x20002000,0x21008008
 uc.mem_write(0x21000000,bytes([0xcd])*0x10000)
 uc.mem_write(op,bytes.fromhex(row['op']));uc.mem_write(location,bytes.fromhex(row['location']))
 uc.mem_write(sp,struct.pack('<Q',0x22000000));uc.mem_write(sp+0x28,struct.pack('<4Q',0x20004000,0,0,0x20005000))
 for reg,value in [(UC_X86_REG_RCX,op),(UC_X86_REG_RDX,row['kind_byte']),(UC_X86_REG_R8,location),(UC_X86_REG_R9,0x20003000),(UC_X86_REG_RSP,sp)]:uc.reg_write(reg,value)
 def ret():
  s=uc.reg_read(UC_X86_REG_RSP);uc.reg_write(UC_X86_REG_RIP,struct.unpack('<Q',uc.mem_read(s,8))[0]);uc.reg_write(UC_X86_REG_RSP,s+8)
 def hook(u,a,size,unused):
  if a in [0x1412b4a40,0x140a20890]:ret()
  elif a==0x1408336b0:row['native_event']=bytes(uc.mem_read(uc.reg_read(UC_X86_REG_RDX),136)).hex();uc.emu_stop()
 uc.hook_add(UC_HOOK_CODE,hook);uc.emu_start(0x141276e30,0x22000000,count=10000)
(p/'rotate-native-events.json').write_text(json.dumps(rows,indent=2));print('native request/event pairs',sum('native_event'in r for r in rows))
