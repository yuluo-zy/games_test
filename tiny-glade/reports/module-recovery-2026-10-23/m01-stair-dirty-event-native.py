"""原1407d16b0完整入口到ret；空请求/空瓦片订阅的显式输入，记录真实事件选择与写出。"""
from pathlib import Path
import hashlib,json,struct
import pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
p=Path(__file__).resolve();root=p.parents[3];raw=(root/'B版本内容/tiny-glade.rne').read_bytes()
assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(pe.OPTIONAL_HEADER.SizeOfImage+4095)&~4095);u.mem_write(base,pe.get_memory_mapped_image());u.mem_map(0x200000,0x100000)
read=lambda a,n:bytes(u.mem_read(a,n));q=lambda a:struct.unpack('<Q',read(a,8))[0]
writeq=lambda a,*v:u.mem_write(a,struct.pack('<'+'Q'*len(v),*v))
def param(at,owner):writeq(at,owner,0,at+0x28,1<<32)
def ret(value=None):
 if value is not None:u.reg_write(UC_X86_REG_RAX,value)
 sp=u.reg_read(UC_X86_REG_RSP);u.reg_write(UC_X86_REG_RIP,q(sp));u.reg_write(UC_X86_REG_RSP,sp+8)
rows=[]
for disc in [0,1,2,3,4,5,0x100000000]:
 for tag in [0,1,2]:
  for oldtag in [0,1]:
   for height in [0xff800000,0,0x40400000,0x40400001,0x7fc12345]:
    for corner in [False,True]:
     u.mem_write(0x200000,bytes(0x100000));stack=0x2f0008;stop=0x2ff000
     subscriber=0x210000;visual=0x211000;requests=0x212000;params=0x213000;reader=0x214000;events=0x215000;writer=0x216000;payload=0x217000;output=0x218000;wall=0x219000
     param(0x200000,subscriber);param(0x200040,visual);param(0x200060,requests)
     writeq(subscriber,0,8,0);writeq(requests,0,8,0)
     writeq(events,1,payload,1,0,0,8,0,1,1);writeq(writer,0,8,0,0,8,output,0,0,0)
     writeq(params,reader,events,0,0,0,writer,0,params+0x70,1<<32,0x220000)
     raw_event=bytearray(136);raw_event[:8]=struct.pack('<Q',disc)
     # 非3/4当前地址就是首判别；其payload+8仍来自tag输入，用于核变体选择。
     raw_event[8:16]=struct.pack('<Q',tag);raw_event[16:24]=struct.pack('<Q',oldtag);raw_event[24:32]=struct.pack('<Q',41);raw_event[129]=63 if corner else 62
     u.mem_write(payload,bytes(raw_event)+bytes(8));u.mem_write(wall+0x284,struct.pack('<I',height))
     writeq(stack,stop);u.reg_write(UC_X86_REG_RSP,stack);u.reg_write(UC_X86_REG_RCX,0x200000);u.reg_write(UC_X86_REG_RDX,0x200040);u.reg_write(UC_X86_REG_R8,0);u.reg_write(UC_X86_REG_R9,params)
     lookups=[]
     def hook(mu,a,n,data):
      if a==0x140c16a00:ret(0)
      elif a==0x141293420:ret() # 明确空请求输入，只桥接库fold；不提供候选业务。
      elif a==0x140960ac0:
       ptr=mu.reg_read(UC_X86_REG_RCX);u.mem_write(ptr,bytes(512));ret(ptr) # new_clear的原memset库边界。
      elif a==0x140a91ef0:lookups.append(mu.reg_read(UC_X86_REG_RDX));ret(wall)
      elif a==0x1412945c0:ret() # 明确空订阅/全零DirtyTiles，单独由生产World验证发布消费。
     h=u.hook_add(UC_HOOK_CODE,hook)
     try:u.emu_start(0x1407d16b0,stop,count=20000)
     except Exception as e:raise RuntimeError((disc,tag,oldtag,hex(height),corner,hex(u.reg_read(UC_X86_REG_RIP))))from e
     finally:u.hook_del(h)
     count=q(writer+0x30);emitted=[q(output+i*16+8)for i in range(count)]
     rows.append(dict(disc=disc,tag=tag,oldtag=oldtag,height=height,corner=corner,lookups=lookups,emitted=emitted,read_count=q(reader)))
out=p.with_suffix('.json')
with out.open('x')as f:json.dump(dict(scope=__doc__,rne_sha256=hashlib.sha256(raw).hexdigest(),rows=rows),f,indent=2)
print(json.dumps(dict(cases=len(rows),emitted=sum(len(o['emitted'])for o in rows))))
