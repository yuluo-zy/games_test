"""原degree1+实际archivist空集合/allow_any分支；边界是只读邻接Vec/真实布局空policy，非完整原World。"""
from pathlib import Path
import pefile,struct,hashlib,json
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
p=Path(__file__).resolve();root=p.parents[3]
raw=(root/'B版本内容/tiny-glade.rne').read_bytes()
assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(pe.OPTIONAL_HEADER.SizeOfImage+4095)&~4095);u.mem_write(base,pe.get_memory_mapped_image())
u.mem_map(0x200000,0x10000);frame=0x208000;vec=0x201000;archive=0x202000
rows=[]
for degree in [0,1,2,3,7,8,1024]:
 for allowed in [False,True]:
  u.mem_write(0x200000,bytes(0x10000));u.mem_write(vec+16,struct.pack('<Q',degree));u.mem_write(archive+0x1e0,bytes([allowed]));u.mem_write(frame+0x130,struct.pack('<Q',archive))
  u.reg_write(UC_X86_REG_RAX,vec);u.reg_write(UC_X86_REG_R13,71);u.reg_write(UC_X86_REG_RBP,frame);u.reg_write(UC_X86_REG_RSP,frame-0x300)
  outcome=[]
  def stop(mu,a,n,data):
   if a in [0x140806630,0x140806686]:outcome.append(a==0x140806686);mu.emu_stop()
  hook=u.hook_add(UC_HOOK_CODE,stop);u.emu_start(0x14080665c,0,count=1000);u.hook_del(hook)
  assert len(outcome)==1
  rows.append(dict(degree=degree,allow_any=allowed,cull=outcome[0]))
out=p.with_name('m01-maintain-stair-graph-original-predicate.json')
with out.open('x') as f:json.dump(dict(original_sha256=hashlib.sha256(raw).hexdigest(),range='14080665c..686',callee='140b2bd90 full original helper; empty permitted-ID set',scope=__doc__,rows=rows),f,indent=2)
print(json.dumps(rows))
