from pathlib import Path
import sys,json,struct,random,collections,math
OUT=Path(__file__).resolve().parent
OLD=OUT.parents[1]/'shared-foundation-2026-10-13/geometry-curves'
sys.path.insert(0,str(OLD))
import native_oracle as n
from native_oracle import *
OUT=Path(__file__).resolve().parent
trace=collections.deque(maxlen=20)
cpu.hook_add(UC_HOOK_CODE,lambda c,pc,size,_: trace.append(hex(pc)))
def dependency_hook(c,pc,size,_):
 if pc==0x142923fa0:
  x=struct.unpack('<f',struct.pack('<I',c.reg_read(UC_X86_REG_XMM0)&0xffffffff))[0]
  y=math.copysign(math.floor(abs(x)+0.5),x) if math.isfinite(x) else x
  c.reg_write(UC_X86_REG_XMM0,bits(y));ret()
cpu.hook_add(UC_HOOK_CODE,dependency_hook)
def decode_curve(address,dimension):
 tag=u64(address)
 if tag==1<<63:return {'error':cpu.mem_read(address+8,1)[0]}
 count=u64(address+16);p=u64(address+8)
 return {'points':[floats(p+i*dimension*4,dimension)for i in range(count)],'points_u':floats(u64(address+32),count),'length':floats(address+48,1)[0]}
def run(points,address,arg,dimension=3,projected=False):
 built=n.curve(points,dimension,projected)
 if 'error'in built:return {'input_error':built}
 cpu.mem_write(DATA+0x400,bytes(cpu.mem_read(DATA,56)))
 arguments(DATA+0x800,DATA+0x400)
 cpu.reg_write(UC_X86_REG_R8,int(arg))
 cpu.reg_write(UC_X86_REG_XMM2,bits(float(arg)))
 cpu.reg_write(UC_X86_REG_R9,0)
 call(address)
 return decode_curve(DATA+0x800,dimension)
def main():
 rng=random.Random(801249)
 paths=[[[0,0,0],[3,0,4],[6,0,4]],[[0,0,0],[1,3,0],[0,6,0]],[[0,0,0],[2,1,0],[2,3,2],[4,5,2]],[[0,0,0],[0,0,0],[1,0,0]]]
 for i in range(40):
  p=[[rng.uniform(-8,8)for _ in range(3)]for _ in range(rng.randint(2,12))]
  if i%3==0:p[-1]=p[0].copy()
  paths.append(p)
 rows=[]
 for points in paths:
  for step in [.5,1,2,3,10]:
   for target in [('resample',0x140152b00,False),('per_segment',0x14146a2a0,True)]:
    try: result=run(points,target[1],step,projected=target[2]);rows.append({'method':target[0],'points':[[bits(v)for v in p]for p in points],'argument':bits(step),'expected':result})
    except Exception as e:print(target,repr(e),hex(cpu.reg_read(UC_X86_REG_RIP)),list(trace));raise
 for points in paths[:15]:
  for passes in [0,1,2,5]:rows.append({'method':'smooth','points':[[bits(v)for v in p]for p in points],'argument':passes,'expected':run(points,0x140154490,passes)})
 for points in paths[:15]:
  for closed in [False,True]:
   setup();cpu.mem_write(DATA+0x1000,struct.pack('<'+'f'*(len(points)*3),*[v for p in points for v in p]));arguments(DATA+0x1000,len(points),3);cpu.reg_write(UC_X86_REG_R9,int(closed));call(0x141465260)
   rows.append({'method':'smooth_points','points':[[bits(v)for v in p]for p in points],'argument':3,'closed':closed,'expected':{'points':[floats(DATA+0x1000+i*12,3)for i in range(len(points))]}})
 (OUT/'completion-fixtures.json').write_text(json.dumps({'original_sha256':SHA,'cases':rows},indent=2)+'\n')
 print(json.dumps({'cases':len(rows)}))
 for r in rows[:10]:print(r)
if __name__=='__main__':main()
