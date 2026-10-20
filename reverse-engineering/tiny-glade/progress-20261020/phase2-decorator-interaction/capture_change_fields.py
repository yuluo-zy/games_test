"""Execute original apply_decorator_change, including its actual field updates.

Storage lookup/tracking primitives use a one-entry fixture. Remove returns the
fixture's full Info. Insert accepts the Info assembled by original instructions
and places those exact bytes into that entry; it does not calculate a new Info.
The real original hash-table probe and every rank/type/extra/delta branch execute.
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
rows=json.loads((p/'rotate-native-events.json').read_text())
for row in rows:
 if 'op' not in row:continue
 uc=Uc(UC_ARCH_X86,UC_MODE_64);uc.mem_map(base,(len(image)+4095)&~4095);uc.mem_write(base,image)
 uc.mem_map(0x20000000,0x10000);uc.mem_map(0x21000000,0x10000);uc.mem_map(0x22000000,0x1000)
 storage,request,bucket,ctrl,sp=0x20001000,0x20002000,0x20004000,0x20007000,0x21008008
 before=bytearray([0x91])*72
 for offset,value in [(0,2.),(4,4.),(8,.25),(12,7.),(16,.25)]:struct.pack_into('<f',before,offset,value)
 before[0x22]=row['dst'];before[0x44]=2;before[0x46]=row['ty']
 uc.mem_write(ctrl-80,struct.pack('<II',71,0)+bytes(before))
 uc.mem_write(ctrl,b'\x00'+b'\xff'*15)
 uc.mem_write(bucket+0x30,struct.pack('<QQQQ',ctrl,0,0,1))
 uc.mem_write(request,bytes.fromhex(row['op']));uc.mem_write(sp,struct.pack('<Q',0x22000000))
 for reg,value in [(UC_X86_REG_RCX,storage),(UC_X86_REG_RDX,request),(UC_X86_REG_RSP,sp)]:uc.reg_write(reg,value)
 def ret():
  s=uc.reg_read(UC_X86_REG_RSP);uc.reg_write(UC_X86_REG_RIP,struct.unpack('<Q',uc.mem_read(s,8))[0]);uc.reg_write(UC_X86_REG_RSP,s+8)
 def hook(u,a,size,unused):
  if a==0x22000000:uc.emu_stop()
  elif a==0x140a20670:
   out=uc.reg_read(UC_X86_REG_RCX);uc.mem_write(out,bytes(before));uc.reg_write(UC_X86_REG_RAX,out);ret()
  elif a==0x140a1cf30:uc.reg_write(UC_X86_REG_RAX,bucket);uc.reg_write(UC_X86_REG_RDX,0x20008000);ret()
  elif a==0x14155afe0:
   value=bytes(uc.mem_read(uc.reg_read(UC_X86_REG_R9),72));uc.mem_write(ctrl-72,value);ret()
  elif a==0x140a1cde0:uc.reg_write(UC_X86_REG_RAX,bucket);ret()
  elif a==0x1402c9200:ret()
  elif a==0x1405b0ed0:uc.reg_write(UC_X86_REG_RAX,0);ret()
  elif a in [0x1428d9310,0x1428d9390,0x1428d9430]:raise RuntimeError('native field validation panic')
 uc.hook_add(UC_HOOK_CODE,hook);uc.emu_start(0x140a20890,0x22000000,count=10000)
 row['native_before']=before.hex();row['native_after']=bytes(uc.mem_read(ctrl-72,72)).hex()
(p/'rotate-native-storage-fields.json').write_text(json.dumps(rows,indent=2));print('original storage field updates',sum('native_after'in r for r in rows))
