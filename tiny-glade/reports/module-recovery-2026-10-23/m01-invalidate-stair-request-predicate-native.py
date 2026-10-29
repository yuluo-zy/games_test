"""原141477ff0完整有界请求dirty折叠。显式ABI输入，hash/输出insert为库边界；不执行原World。"""
from pathlib import Path
import pefile,struct,hashlib,json
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
p=Path(__file__).resolve();root=p.parents[3];raw=(root/'B版本内容/tiny-glade.rne').read_bytes()
assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(pe.OPTIONAL_HEADER.SizeOfImage+4095)&~4095);u.mem_write(base,pe.get_memory_mapped_image());u.mem_map(0x200000,0x10000)
request=0x201000;iterator=0x202000;dirtyset=0x203000;output=0x204000;ctrl=0x205000;stack=0x208000;stop=0x209000
rows=[]
for table in ['empty','matching','different']:
 for old in [0,1]:
  for edge in [(3,8),(8,3),(17,17)]:
   u.mem_write(0x200000,bytes(0x10000));u.mem_write(request+0x128,struct.pack('<II',*edge));u.mem_write(request+0x148,bytes([old]));u.mem_write(iterator,struct.pack('<QQQ',request,request+0x150,dirtyset))
   u.mem_write(dirtyset,struct.pack('<QQQQ',ctrl,15,14,0 if table=='empty' else 1));u.mem_write(ctrl,bytes([0])+bytes([255])*15);u.mem_write(ctrl-8,struct.pack('<II',*(edge if table=='matching' else (91,92))))
   u.mem_write(stack,struct.pack('<Q',stop));u.reg_write(UC_X86_REG_RCX,iterator);u.reg_write(UC_X86_REG_RDX,output);u.reg_write(UC_X86_REG_RSP,stack)
   emitted=[];calls=[]
   def hook(mu,a,n,data):
    if a in [0x1405b1be0,0x1402c8640]:
     calls.append(hex(a))
     if a==0x1402c8640:emitted.append([mu.reg_read(UC_X86_REG_RDX)&0xffffffff,mu.reg_read(UC_X86_REG_R8)&0xffffffff])
     else:mu.reg_write(UC_X86_REG_RAX,0)
     sp=mu.reg_read(UC_X86_REG_RSP);ra=struct.unpack('<Q',mu.mem_read(sp,8))[0];mu.reg_write(UC_X86_REG_RSP,sp+8);mu.reg_write(UC_X86_REG_RIP,ra)
   h=u.hook_add(UC_HOOK_CODE,hook);u.emu_start(0x141477ff0,stop,count=5000);u.hook_del(h)
   rows.append(dict(table=table,old=old,edge=edge,dirty=u.mem_read(request+0x148,1)[0],emitted=emitted,library_boundaries=calls))
out=p.with_name('m01-invalidate-stair-request-predicate-native.json')
with out.open('x')as f:json.dump(dict(original_sha256=hashlib.sha256(raw).hexdigest(),range='141477ff0..1414781bb including natural entry/ret',scope=__doc__,rows=rows),f,indent=2)
print(json.dumps(dict(cases=len(rows),results=[(r['table'],r['old'],r['dirty'],r['emitted'])for r in rows])))
