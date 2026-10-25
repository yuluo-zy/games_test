from pathlib import Path
import pefile,struct,json
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
OUT=Path(__file__).parent;pe=pefile.PE('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne',fast_load=True);BASE=pe.OPTIONAL_HEADER.ImageBase

def run(indices,positions):
 u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(BASE,(pe.OPTIONAL_HEADER.SizeOfImage+4095)&~4095)
 for s in pe.sections:u.mem_write(BASE+s.VirtualAddress,s.get_data())
 M=0x10000000;u.mem_map(M,0x100000);STOP=M
 def w(a,v):u.mem_write(a,struct.pack('<Q',v))
 def q(a):return struct.unpack('<Q',u.mem_read(a,8))[0]
 def f(v):return struct.unpack('<f',struct.pack('<f',v))[0]
 out,it,idx,pos,terrain=M+0x1000,M+0x2000,M+0x3000,M+0x4000,M+0x5000
 w(it,idx);w(it+8,idx+4*len(indices));w(it+16,pos);w(it+24,len(positions));w(it+32,terrain);w(terrain,terrain+16)
 for i,x in enumerate(indices):u.mem_write(idx+i*4,struct.pack('<I',x))
 for i,v in enumerate(positions):u.mem_write(pos+i*12,struct.pack('<3f',*v))
 rsp=M+0xf0008;w(rsp,STOP)
 for r,v in [(UC_X86_REG_RSP,rsp),(UC_X86_REG_RCX,out),(UC_X86_REG_RDX,it),(UC_X86_REG_R8,0)]:u.reg_write(r,v)
 calls=[];error=None
 def ret(value=None):
  sp=u.reg_read(UC_X86_REG_RSP)
  if value is not None:u.reg_write(UC_X86_REG_RAX,value)
  u.reg_write(UC_X86_REG_RIP,q(sp));u.reg_write(UC_X86_REG_RSP,sp+8)
 def hook(uc,a,size,ctx):
  nonlocal error
  if a==0x140613c10:ret(M+0x6000)
  elif a==0x1409e9640:
   x,z=[struct.unpack('<f',struct.pack('<I',u.reg_read(r)&0xffffffff))[0] for r in [UC_X86_REG_XMM1,UC_X86_REG_XMM2]];height=f(f(x*2.)+f(z*3.));calls.append([x,z]);u.reg_write(UC_X86_REG_XMM0,struct.unpack('<I',struct.pack('<f',height))[0]);ret()
  elif a==0x1428d9518:error='bounds';u.emu_stop()
  elif a==STOP:u.emu_stop()
 u.hook_add(UC_HOOK_CODE,hook);u.emu_start(0x1412fd550,STOP,count=10000)
 return {'sample_calls':calls,'error':error,'result':[] if error else [list(struct.unpack('<III',u.mem_read(q(out+8)+i*12,12))) for i in range(q(out+16))]}

if __name__=='__main__':
 cases=[]
 positions=[[-3.,8.,2.],[4.,9.,-5.],[7.,10.,6.]]
 for idx in [[],[0],[2,0,2,1],[1,0,2],[2,2,2],[3],[0,4]]:cases.append({'indices':idx,'positions':positions,'native':run(idx,positions)})
 (OUT/'native-mesh-indexed-fixtures.json').write_text(json.dumps(cases,indent=2),encoding='utf8');print('indexed map',len(cases))
