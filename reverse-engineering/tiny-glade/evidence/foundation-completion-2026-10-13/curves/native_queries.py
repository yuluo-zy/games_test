from pathlib import Path
import sys,json,struct,random
OUT=Path(__file__).resolve().parent
sys.path.insert(0,str(OUT.parents[1]/'shared-foundation-2026-10-13/geometry-curves'))
import native_oracle as n
from native_oracle import *
OUT=Path(__file__).resolve().parent
exit_inline=0x14159a578
cpu.hook_add(UC_HOOK_CODE,lambda c,pc,size,_:c.emu_stop() if pc==exit_inline else None)
def main():
 rng=random.Random(977512);rows=[]
 for _ in range(50):
  points=[[rng.uniform(-10,10)for _ in range(3)]for _ in range(rng.randint(2,9))]
  setup();cpu.mem_write(DATA+0x1000,struct.pack('<'+'f'*(len(points)*3),*[v for p in points for v in p]));cpu.mem_write(DATA+0x100,struct.pack('<QQQ',len(points),DATA+0x1000,len(points)));arguments(DATA,DATA+0x100);call(0x1411fa190)
  rows.append({'method':'threshold_constructor','points':[[bits(v)for v in p]for p in points],'expected_length':floats(DATA+48,1)[0],'expected_u':floats(u64(DATA+32),len(points))})
 for _ in range(150):
  points=[[rng.uniform(-10,10)for _ in range(2)]for _ in range(rng.randint(2,9))];point=[rng.uniform(-12,12),rng.uniform(-12,12)];n.curve(points);arguments(DATA);cpu.reg_write(UC_X86_REG_XMM1,bits(point[0]));cpu.reg_write(UC_X86_REG_XMM2,bits(point[1]));call(0x141469000)
  rows.append({'method':'right_side','points':[[bits(v)for v in p]for p in points],'point':[bits(v)for v in point],'expected':bool(cpu.reg_read(UC_X86_REG_RAX)&1)})
 for suffix in [[[2,0]],[[1,0]],[[1,0],[2,0]],[[float('nan'),0]],[],[[2,0],[2,0]]]:
  points=[[0,0],[1,0]];n.curve(points);before=bytes(cpu.mem_read(DATA,56));cpu.mem_write(DATA+0x4000,struct.pack('<'+'f'*(len(suffix)*2),*[v for p in suffix for v in p]));arguments(DATA+0x800,DATA,DATA+0x4000);cpu.reg_write(UC_X86_REG_R9,len(suffix));call(0x141466860)
  tag=cpu.mem_read(DATA+0x800,1)[0];count=u64(DATA+16);cache_count=u64(DATA+40)
  rows.append({'method':'extend','points':[[bits(v)for v in p]for p in points],'suffix':[[bits(v)for v in p]for p in suffix],'tag':tag,'detail':u64(DATA+0x808)if tag==2 else None,'expected_points':[floats(u64(DATA+8)+i*8,2)for i in range(count)],'expected_u':floats(u64(DATA+32),cache_count),'expected_length':floats(DATA+48,1)[0]})
 for value in [-10,-.1,-0.0,0,.2,.9,1,2,1e30,1e40,float('nan'),float('inf'),-float('inf')]:
  setup();cpu.reg_write(UC_X86_REG_XMM0,struct.unpack('<Q',struct.pack('<d',value))[0]);cpu.reg_write(UC_X86_REG_RSI,DATA);cpu.reg_write(UC_X86_REG_RBP,DATA+0x8000)
  try:cpu.emu_start(0x14159a539,STOP,count=1000000);r={'panic':False,'expected':floats(DATA+4,1)[0]}
  except RuntimeError as e:
   if 'original panic'not in str(e):raise
   r={'panic':True}
  rows.append({'method':'deserialize_scalar','value_bits':struct.unpack('<Q',struct.pack('<d',value))[0],**r})
 for _ in range(40):
  points=[[rng.uniform(-10,10)for _ in range(2)]for _ in range(rng.randint(3,12))];weights=[rng.randint(0,7)for _ in points];iterations=rng.choice([1,2,5]);n.curve(points);cpu.mem_write(DATA+0x400,bytes(cpu.mem_read(DATA,56)));cpu.mem_write(DATA+0x3000,struct.pack('<'+'Q'*len(weights),*weights));arguments(DATA+0x800,DATA+0x400,iterations);cpu.reg_write(UC_X86_REG_R9,DATA+0x3000);cpu.mem_write(STACK+0x100008+0x28,struct.pack('<Q',len(weights)));call(0x141468880)
  count=u64(DATA+0x810);rows.append({'method':'weighted_smooth','points':[[bits(v)for v in p]for p in points],'weights':weights,'iterations':iterations,'expected':[floats(u64(DATA+0x808)+i*8,2)for i in range(count)]})
 (OUT/'query-fixtures.json').write_text(json.dumps({'original_sha256':SHA,'cases':rows},indent=2)+'\n');print(json.dumps({'cases':len(rows)}))
if __name__=='__main__':main()
