import ctypes, hashlib,json,struct
from pathlib import Path
import pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
raw=Path('B版本内容/tiny-glade.rne').read_bytes();sha=hashlib.sha256(raw).hexdigest();assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
p=pefile.PE(data=raw,fast_load=True); base=p.OPTIONAL_HEADER.ImageBase
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(p.OPTIONAL_HEADER.SizeOfImage+4095)&~4095);u.mem_write(base,p.get_memory_mapped_image());u.mem_map(0x30000000,0x10000);sp=0x30008000
lib=ctypes.CDLL('/usr/lib/libSystem.B.dylib');lib.fmodf.argtypes=[ctypes.c_float,ctypes.c_float];lib.fmodf.restype=ctypes.c_float
def bits(x):return struct.unpack('<I',struct.pack('<f',x))[0]
def num(x):return struct.unpack('<f',struct.pack('<I',x&0xffffffff))[0]
status="ok"
stop=0
calls=[]
def hook(c,pc,size,_):
 global status,stop,calls
 if pc==0x1428d9430:
  status="panic"; stop=pc; c.emu_stop(); return
 if pc==0x142923f10:
  a=num(c.reg_read(UC_X86_REG_XMM0));b=num(c.reg_read(UC_X86_REG_XMM1));calls.append([bits(a),bits(b)]);c.reg_write(UC_X86_REG_XMM0,bits(lib.fmodf(a,b)));r=c.reg_read(UC_X86_REG_RSP);c.reg_write(UC_X86_REG_RIP,struct.unpack('<Q',c.mem_read(r,8))[0]);c.reg_write(UC_X86_REG_RSP,r+8)
 if pc==0x30000000:c.emu_stop()
u.hook_add(UC_HOOK_CODE,hook)
shapes=[(0,[0.,0.,2.,0.,0.,0.]),(1,[1.,0.,0.,0.,4.,2.]),(1,[0.6,0.8,2.,-1.,4.,2.])];lines=[[[-3.,0.],[3.,0.]],[[-1.,0.],[1.,0.]],[[-4.,2.],[4.,2.]],[[0.,-3.],[0.,3.]],[[-3.,1.],[3.,1.]],[[2.,-1.],[2.,-1.]],[[0.,0.],[1.,0.]],[[float('nan'),0.],[1.,0.]],[[float('inf'),0.],[1.,0.]],[[num(0x3fffffff),0.],[num(0x40000001),0.]],[[num(0x7f812345),0.],[1.,0.]]];rows=[]
for tag,sh in shapes:
 for line in lines:
  for padbits in (bits(0.),bits(0.1),bits(0.6),bits(-0.2),bits(2.0),0x7fc12345,0x7f812345,0x7f800000):
   pad=num(padbits)
   for start,end in ((0.2,0.8),(0.8,0.2)):
    line_bits=[bits(v) for pair in line for v in pair]; line_bits[0]=0x7f812345 if line is lines[-1] else line_bits[0]; u.mem_write(0x30001000,struct.pack('<4I',*line_bits));u.mem_write(0x30002000,struct.pack('<I6f',tag,*sh));u.mem_write(0x30003000,bytes(16));u.mem_write(sp,struct.pack('<Q',0x30000000));u.mem_write(sp+0x28,struct.pack('<Q',0x30002000));u.mem_write(sp+0x30,struct.pack('<I',padbits));
    for reg,val in [(UC_X86_REG_RSP,sp),(UC_X86_REG_RCX,0x30003000),(UC_X86_REG_RDX,0x30001000),(UC_X86_REG_XMM2,bits(start)),(UC_X86_REG_XMM3,bits(end)),(UC_X86_REG_MXCSR,0x1f80)]:u.reg_write(reg,val)
    status="ok";stop=0;calls=[]
    try: u.emu_start(0x14175ada0,0x30000001,count=10000)
    except Exception as e: status="fault:"+str(e)+"@"+hex(u.reg_read(UC_X86_REG_RIP))
    out=struct.unpack('<III',u.mem_read(0x30003000,12));rows.append({'tag':tag,'shape':sh,'line_bits':[line_bits[:2],line_bits[2:]],'padding_bits':padbits,'start':start,'end':end,'status':status,'stop':hex(stop),'fmod_calls':calls,'words':list(out)})
Path('tiny-glade/reports/module-recovery-2026-10-23/m06-line-overlap-oracle.json').write_text(json.dumps({'sha256':sha,'fmod_bridge':'macOS libSystem; Windows UCRT equivalence not asserted','comparison':'finite return values exact bits; None and panic independently; NaN output compare category and record raw bits','input_boundary':'padding 0 and 0.6 are caller values; other values probe IEEE boundaries and are not claimed reachable','rows':rows},indent=2)+'\n');print('rows',len(rows))
