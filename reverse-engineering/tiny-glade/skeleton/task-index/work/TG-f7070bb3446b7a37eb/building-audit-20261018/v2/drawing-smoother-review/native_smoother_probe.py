"""Original B DrawingInputSmoother::process full entry, real ws_to_ss and Vec grow.
Only the shared allocation/memory boundary hooks are used. Existing mutable
last-accepted XZ survives sequential calls. No game leaf is bridged.
"""
from pathlib import Path
import json,struct,hashlib,math,random
ROOT=Path('/Users/yuluo/Desktop/game');HERE=Path(__file__).resolve().parent
recipe=(ROOT/'games_test/reverse-engineering/tiny-glade/evidence/shared-foundation-2026-10-13/utils-base/native_spline_rdp.py').read_text().split('def spline(')[0]
recipe=recipe.replace('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne',str(ROOT/'ljxsj_92385/Tiny Glade/tiny-glade.rne'))
exec(compile(recipe,'shared-native-memory-harness','exec'),globals());OUT=HERE
from capstone import Cs,CS_ARCH_X86,CS_MODE_64
STATE,POINT,TIME,CAMERA,WINDOW=INPUT,INPUT+0x1000,INPUT+0x2000,INPUT+0x3000,INPUT+0x4000
for name,start,size in [('smoother',0x14145c350,858),('ws-to-ss',0x14127c950,25)]:
 code=pe.get_data(start-base,size);(HERE/(name+'.bin')).write_bytes(code);(HERE/(name+'.asm')).write_text('\n'.join(f'{i.address:x}: {i.bytes.hex():28} {i.mnemonic} {i.op_str}'for i in Cs(CS_ARCH_X86,CS_MODE_64).disasm(code,start))+'\n')
def f(v):
 try:return struct.unpack('<f',struct.pack('<f',v))[0]
 except OverflowError:return math.copysign(math.inf,v)
def div(a,b):
 if math.isnan(a)or math.isnan(b):return math.nan
 if b==0:return math.nan if a==0 else math.copysign(math.inf,a*b if a*b else math.copysign(1,a)*math.copysign(1,b))
 return f(a/b)
def reference(prev,p,n,scale,height):
 # Order matches original separate SSE ops; the test view has only z-axis=1.
 z=f(f(f(f(0*p[0])+f(0*p[1]))+f(p[2]))+0);pixels=f(f(div(.5,-z)*scale)*f(height));start=prev if prev is not None else[p[0],p[2]];result=[]
 for i in range(1,n+1):
  t=div(f(i),f(n));a=f(1-t);q=[f(f(a*start[j])+f(t*p[j*2]))for j in range(2)]
  if prev is not None:
   d=[f(q[j]-prev[j])for j in range(2)];distance=f(math.sqrt(f(f(d[0]*d[0])+f(d[1]*d[1]))))
   if not(distance>f(.01)and f(distance*pixels)>3):continue
  result.append(q);prev=q
 return result,prev
def native(prev,point,n,scale,height):
 sp=reset();u.mem_write(STATE,struct.pack('<I',prev is not None)+pack(prev or[0,0]));u.mem_write(POINT,pack(point));u.mem_write(TIME,bytes(0x40));u.mem_write(TIME+0x18,struct.pack('<Q',n));u.mem_write(CAMERA,bytes(0xc0));u.mem_write(CAMERA+0x14,pack([scale]));u.mem_write(CAMERA+0xa8,pack([1]));u.mem_write(WINDOW,struct.pack('<II',1920,height));u.mem_write(sp+0x28,struct.pack('<QQ',CAMERA,WINDOW))
 for r,v in [(UC_X86_REG_RCX,OUTPUT),(UC_X86_REG_RDX,STATE),(UC_X86_REG_R8,POINT),(UC_X86_REG_R9,TIME)]:u.reg_write(r,v)
 run(0x14145c350);cap,ptr,length=struct.unpack('<3Q',u.mem_read(OUTPUT,24));out=[list(struct.unpack('<2I',u.mem_read(ptr+i*8,8)))for i in range(length)];tag=struct.unpack('<I',u.mem_read(STATE,4))[0];state=None if tag==0 else list(struct.unpack('<2I',u.mem_read(STATE+4,8)))
 return {'points':out,'last':state,'capacity':cap}
def fb(v):return [bits(x)for x in v]
rng=random.Random(20261018);inputs=[]
for prev in [None,[0.,0.],[2.,-5.]]:
 for n in [0,1,2,3,6,16]:
  for point in [[0,0,-1],[1,10,-1],[.01,0,-1],[1,0,-1000]]:inputs.append((prev,point,n,1,1080))
for distance in [0,.009999999,.01,.010000001,.99999994,1.,1.00000012,1.5,2.]:
 # camera height6 and z=-1 => 3 pixels per world unit, threshold exact1.
 inputs.append(([0,-1],[distance,0,-1],4,1,6))
for height in [0,1,6,1080,0xffffffff]:inputs.append(([0,-1],[1,0,-1],3,1,height))
for z in [0.,-0.,1.,-1.,float('inf'),-float('inf')]:inputs.append(([0,-1],[1,0,z],3,1,1080))
for n in [1,3,7,16]:
 for _ in range(12):inputs.append(([rng.uniform(-3,3),rng.uniform(-3,3)],[rng.uniform(-3,3),rng.uniform(-2,2),rng.uniform(-3,-.1)],n,rng.uniform(.2,2),rng.choice([6,480,1080])))
cases=[]
for prev,point,n,scale,height in inputs:
 prev=None if prev is None else list(map(f,prev));point=list(map(f,point));scale=f(scale)
 actual=native(prev,point,n,scale,height);expected,last=reference(prev,point,n,scale,height)
 assert actual['points']==list(map(fb,expected)),(prev,point,n,scale,height,actual,expected)
 assert actual['last']==(None if last is None else fb(last)),(actual,last)
 cases.append({'previous':None if prev is None else fb(prev),'point':fb(point),'ticks_60':n,'projection_scale':bits(scale),'height':height,'output':actual})
seq=[];prev=None
for point,n in [([0,0,-1],0),([0,0,-1],1),([.005,0,-1],6),([.009,0,-1],3),([.012,0,-1],3),([1,0,-1],4),([1.001,0,-1],0),([2,0,-1],2)]:
 actual=native(prev,point,n,1,1080);seq.append({'previous':None if prev is None else fb(prev),'point':fb(point),'ticks_60':n,'output':actual});prev=None if actual['last']is None else[struct.unpack('<f',struct.pack('<I',v))[0]for v in actual['last']]
nan_cases=[]
for prev,point,scale,expected_len in [(None,[float('nan'),0,-1],1,1),([0,-1],[float('nan'),0,-1],1,0),([float('nan'),-1],[1,0,-1],1,0),(None,[1,0,-1],float('nan'),1),([0,-1],[1,0,-1],float('nan'),0),(None,[1,0,float('nan')],1,1),([0,-1],[1,0,float('nan')],1,0)]:
 actual=native(prev,point,6,scale,1080);assert len(actual['points'])==expected_len
 if expected_len==0:assert actual['last']==fb(prev)
 nan_cases.append({'previous':None if prev is None else fb(prev),'point':fb(point),'projection_scale':bits(scale),'ticks_60':6,'height':1080,'output':actual,'expected_count':expected_len})
report={'binary_sha256':hashlib.sha256(raw).hexdigest(),'method':__doc__,'entry':'0x14145c350','constants':{'one':bits(1.),'minimum_world_distance':bits(.01),'minimum_pixel_distance':bits(3.),'ws_to_ss_half':bits(.5)},'cases':cases,'sequence':seq,'nan_cases':nan_cases}
(HERE/'native-smoother-fixtures.json').write_text(json.dumps(report,indent=2)+'\n');print({'cases':len(cases),'sequence':len(seq),'nan_cases':len(nan_cases)})
