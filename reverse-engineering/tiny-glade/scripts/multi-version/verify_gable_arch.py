"""Whole B gable arch consumer through final cache store; public-wall lookup is
an explicit standard map boundary. Original XZ mapping, CurveU, height/eave,
point-U, powf, numerical profile and closure-endpoint compensation execute."""
import ctypes,ctypes.util,json,random,struct,subprocess
from verify_parameters import cpu,DATA,STACK,ROOT,baseline,bits
from verify_building_state import reset
from verification_provenance import candidate_provenance
from unicorn import UC_HOOK_CODE
from unicorn.x86_const import *
HEAP=0x50000000;cpu.mem_map(HEAP,0x1000000);heap=HEAP
crt=ctypes.CDLL(ctypes.util.find_library('m') or ctypes.util.find_library('c'));powf=crt.powf;powf.argtypes=[ctypes.c_float]*2;powf.restype=ctypes.c_float
active=False
def fl(x):return struct.unpack('<f',struct.pack('<I',x&0xffffffff))[0]
def ret(c,value=None):
 if value is not None:c.reg_write(UC_X86_REG_RAX,value)
 sp=c.reg_read(UC_X86_REG_RSP);pc=struct.unpack('<Q',c.mem_read(sp,8))[0];c.reg_write(UC_X86_REG_RSP,sp+8);c.reg_write(UC_X86_REG_RIP,pc)
def hook(c,pc,size,user):
 global heap
 if not active:return
 if pc==0x14078be35:
  # Only generic map probing and hash_one are excluded; roof and wall values
  # were already resolved at entry, and original point-U calls have executed.
  c.reg_write(UC_X86_REG_R15,DATA+0x24);c.reg_write(UC_X86_REG_RBX,DATA+0x400+0x2a0);c.reg_write(UC_X86_REG_RSI,0);c.reg_write(UC_X86_REG_RIP,0x14078bf83)
 elif pc in [0x140613c10,0x140613c40]:
  n=c.reg_read(UC_X86_REG_RCX);align=c.reg_read(UC_X86_REG_RDX);ptr=(heap+max(align,16)-1)&~(max(align,16)-1);heap=ptr+n;assert heap<HEAP+0x1000000
  if pc==0x140613c40 and n:c.mem_write(ptr,bytes(n))
  ret(c,ptr)
 elif pc==0x140613c20:ret(c)
 elif pc==0x142923f70:
  value=powf(fl(c.reg_read(UC_X86_REG_XMM0)),fl(c.reg_read(UC_X86_REG_XMM1)));c.reg_write(UC_X86_REG_XMM0,bits(value));ret(c)
 elif pc in [0x1428d9390,0x1428d9518,0x1428d9760,0x1428d9310,0x1428d9430]:raise RuntimeError('Unexpected panic '+hex(pc))
cpu.hook_add(UC_HOOK_CODE,hook)
def original(roof,positions,base,length,style):
 global heap,active
 reset();heap=HEAP;frame=STACK+0x8000;cpu.mem_write(frame,bytes(0x300));cpu.mem_write(DATA,roof);wall=bytearray(672)
 struct.pack_into('<QQ',wall,0x90,DATA+0x1000,len(positions));struct.pack_into('<QQQ',wall,0xd8,128,DATA+0x3000,len(positions));struct.pack_into('<f',wall,0x28c,length);wall[0x299]=style
 cpu.mem_write(DATA+0x400,bytes(wall));cpu.mem_write(DATA+0x1000,b''.join(struct.pack('<3f',*p) for p in positions));cpu.mem_write(DATA+0x3000,struct.pack('<'+'f'*len(positions),*([base]*len(positions))))
 cpu.mem_write(frame-0x30,roof[12:36]);cpu.mem_write(frame+0x90,struct.pack('<Q',DATA+0x400+0x2a0));cpu.mem_write(frame+0xd0,struct.pack('<Q',DATA+0x700));cpu.mem_write(frame+0xf4,struct.pack('<I',23))
 cpu.reg_write(UC_X86_REG_RSI,DATA);cpu.reg_write(UC_X86_REG_RBP,frame);cpu.reg_write(UC_X86_REG_RSP,STACK+0x7000);cpu.reg_write(UC_X86_REG_XMM15,bits(base));cpu.reg_write(UC_X86_REG_XMM11,bits(.5));cpu.reg_write(UC_X86_REG_XMM13,bits(1.5))
 active=True
 try:cpu.emu_start(0x14078bda3,0x14078c42d,count=100000)
 finally:active=False
 assert cpu.reg_read(UC_X86_REG_RIP)==0x14078c42d
 heights=struct.unpack('<'+'I'*len(positions),cpu.mem_read(DATA+0x3000,len(positions)*4));maximum=struct.unpack('<I',cpu.mem_read(DATA+0x400+0x114,4))[0]
 return f'{maximum:08x} '+','.join(f'{v:08x}' for v in heights)
def main():
 rng=random.Random(20261011);inputs=[];expected=[];cases=[]
 for case in range(320):
  width=rng.uniform(1,12);length=rng.uniform(1,12);direction=case%2;style=(case//2)%4;profile=[0.,.25,.5,.75,1.][case%5]
  roof=bytearray(88);struct.pack_into('<I6f',roof,8,1,1,0,0,0,width,length)
  for o,v in [(0x2c,profile),(0x30,rng.uniform(.11,1)),(0x34,1),(0x38,rng.uniform(0,1))]:struct.pack_into('<f',roof,o,v)
  roof[0x3c]=direction
  # Closed and open original outlines, with corner/endpoint and interior samples.
  corners=[[-width*.5,0.,-length*.5],[-width*.5,1.,length*.5],[width*.5,2.,length*.5],[width*.5,3.,-length*.5],[-width*.5,0.,-length*.5]]
  points=[]
  for a,b in zip(corners,corners[1:]):
   points.append(a)
   for j in range(1,4):points.append([a[k]*(1-j*.25)+b[k]*(j*.25) for k in range(3)])
  points.append(corners[-1])
  # Round inputs to source f32 before deriving source_length/oracle arguments.
  points=[list(struct.unpack('<3f',struct.pack('<3f',*p))) for p in points];source_length=struct.unpack('<f',struct.pack('<f',2*(width+length)))[0];base=struct.unpack('<f',struct.pack('<f',rng.uniform(.96,10)))[0]
  expected.append(original(bytes(roof),points,base,source_length,style));inputs.append(f'gable_arch {roof.hex()} {bits(base):08x} {bits(source_length):08x} {int(style>=2)} '+' '.join(f'{bits(v):08x}' for p in points for v in p));cases.append({'case':case,'direction':direction,'style':style,'profile':profile,'points':len(points)})
 got=subprocess.run([str(ROOT/'reconstruction/target/release/lifecycle-probe')],input='\n'.join(inputs)+'\n',capture_output=True,text=True,check=True).stdout.splitlines();assert len(got)==len(expected)
 failures=[{'case':i,'original':e,'rust':a,'input':inputs[i]} for i,(e,a) in enumerate(zip(expected,got)) if e!=a]
 report={'ok':not failures,'cases':len(inputs),'failure_count':len(failures),'failures':failures[:10],'source_sha256':baseline['binary']['sha256'],
 'original_range':['0x14078bda3','0x14078c42d'],'scope':'Whole resolved rectangular-gable consumer including original XZ Curve construction, both arch profile sides, final/first closing sample compensation and maximum cache.',
 'bridges':['Resolved public wall lookup bypasses hash/slot probing only (14078be35..14078bf83)','Standard allocation/free preallocated host-memory bridge','Current-host powf CRT bridge'],**candidate_provenance()}
 out=ROOT/'evidence/building-gather/B-gable-verification.json';out.write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({'ok':not failures,'cases':len(inputs),'failures':len(failures)}));assert not failures
if __name__=='__main__':main()
