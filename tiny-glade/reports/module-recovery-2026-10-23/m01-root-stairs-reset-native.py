"""完整执行B复位80B及原unset helper；不桥接业务分支。"""
from pathlib import Path
import hashlib,json,struct
import pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64
from unicorn.x86_const import UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_RSP
root=Path(__file__).resolve().parents[3]
raw=(root/'B版本内容/tiny-glade.rne').read_bytes()
sha=hashlib.sha256(raw).hexdigest()
assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);image=pe.get_memory_mapped_image();base=pe.OPTIONAL_HEADER.ImageBase
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(image)+4095)&~4095);u.mem_write(base,image)
D,S,STOP=0x20000000,0x30000000,0x200ff000
u.mem_map(D,0x100000);u.mem_map(S,0x10000)
mode_ref,mode,state_ref,state,tick=D,D+0x100,D+0x200,D+0x400,D+0x600
q=lambda a,v:u.mem_write(a,struct.pack('<Q',v))
w=lambda a,v:u.mem_write(a,struct.pack('<I',v))
rows=[]
for tag in range(21):
 for payload in (0,56,62,63,64,255):
  endpoint=bytes((17+i*7)&255 for i in range(24))
  q(mode_ref,mode);w(mode,tag);w(mode+4,0xaabbcc00|payload)
  q(state_ref,state);q(state_ref+0x10,tick);w(state_ref+0x1c,0x12345678);w(tick,0x87654321)
  u.mem_write(state+0x10,endpoint)
  sp=S+0xff08;q(sp,STOP)
  for reg,value in ((UC_X86_REG_RCX,mode_ref),(UC_X86_REG_RDX,state_ref),(UC_X86_REG_RSP,sp)):u.reg_write(reg,value)
  u.emu_start(0x14079c350,STOP,count=100)
  result=bytes(u.mem_read(state+0x10,24));changed=struct.unpack('<I',u.mem_read(tick,4))[0]
  rows.append({'mode':tag,'payload':payload,'before':endpoint.hex(),'after':result.hex(),'changed_tick':changed})
out=Path(__file__).with_name('m01-root-stairs-reset-native.json')
with out.open('x') as f:json.dump({'binary_sha256':sha,'entry':'0x14079c350','body_bytes':80,'executed_helper':'0x1408fe880','business_bridges':[],'cases':rows},f,indent=2);f.write('\n')
print(json.dumps({'file':str(out),'cases':len(rows)}))
