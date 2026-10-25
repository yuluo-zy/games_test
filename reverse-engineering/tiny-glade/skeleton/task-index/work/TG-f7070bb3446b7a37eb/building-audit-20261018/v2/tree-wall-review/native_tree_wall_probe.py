"""Original B complete wall-grid circle callback 0x1425df320.
Only imported libc floorf is bridged; all spatial helpers, bucket scanning,
wall filtering and exact SSE segment arithmetic execute original bytes.
A separate narrow distance invocation retains precise scalar/interpolation bits.
"""
from pathlib import Path
import json,struct,hashlib,math,random
ROOT=Path('/Users/yuluo/Desktop/game');HERE=Path(__file__).resolve().parent
s=(ROOT/'games_test/reverse-engineering/tiny-glade/evidence/shared-foundation-2026-10-13/utils-base/native_spline_rdp.py').read_text().split('def spline(')[0]
s=s.replace('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne',str(ROOT/'ljxsj_92385/Tiny Glade/tiny-glade.rne'))
exec(compile(s,'shared-native-memory-harness','exec'),globals());OUT=HERE
from capstone import Cs,CS_ARCH_X86,CS_MODE_64
SPATIAL,CIRCLE,CLOSURE,WALL,POINT,VISIBLE=INPUT,INPUT+0x1000,INPUT+0x2000,INPUT+0x3000,INPUT+0x4000,INPUT+0x5000
CELLS=HEAP+0x900000;SEGMENTS=HEAP+0x940000
obs={}
def f(v):
 try:return struct.unpack('<f',struct.pack('<f',v))[0]
 except OverflowError:return math.copysign(math.inf,v)
def fb(v):return [bits(x)for x in v]
def value_bits(v):return struct.unpack('<f',struct.pack('<I',v))[0]
def hooks(uc,a,size,data):
 if a==0x1425df666 and obs.get('narrow'):
  uc.emu_stop();return
 if a==0x142923ee0:
  v=value_bits(u.reg_read(UC_X86_REG_XMM0)&0xffffffff);v=v if math.isnan(v)or math.isinf(v)else f(math.floor(v));u.reg_write(UC_X86_REG_XMM0,bits(v));ret()
 elif a==0x140aa65d0:obs.setdefault('cells',[]).append([u.reg_read(UC_X86_REG_RDX)&0xffffffff,u.reg_read(UC_X86_REG_R8)&0xffffffff])
 elif a==0x1425df58b:obs.setdefault('accepted_cells',[]).append(u.reg_read(UC_X86_REG_RSI))
 elif a==0x1425df5e4:obs.setdefault('distance_checks',[]).append(u.reg_read(UC_X86_REG_R8)//56)
 elif a==0x1425df5dd:obs.setdefault('segment_checks',[]).append(u.reg_read(UC_X86_REG_R8)//56)
 elif a==0x1425df670:obs['hit']=True
u.hook_add(UC_HOOK_CODE,hooks)
for name,start,size in[('callback',0x1425df320,1079),('bounds',0x140aa6600,19),('circle-aabb',0x140c98140,33),('cell-size',0x140aa65e0,29),('world-grid',0x140aa6520,166),('grid-index',0x140aa65d0,12),('cell-center',0x140aa5c70,345)]:
 rawcode=pe.get_data(start-base,size);(HERE/(name+'.bin')).write_bytes(rawcode);(HERE/(name+'.asm')).write_text('\n'.join(f'{i.address:x}: {i.bytes.hex():28} {i.mnemonic} {i.op_str}'for i in Cs(CS_ARCH_X86,CS_MODE_64).disasm(rawcode,start))+'\n')
def query(center,radius,point,segments,wall=73,bucket=None):
 global obs
 sp=reset();obs={'cells':[],'segment_checks':[],'hit':False};u.mem_write(SPATIAL,bytes(0x60));u.mem_write(SPATIAL+8,struct.pack('<QQ',CELLS,4096));u.mem_write(SPATIAL+0x48,struct.pack('<IIff',64,64,130,130));u.mem_write(CELLS,bytes(4096*48));u.mem_write(CIRCLE,pack(center+[radius,0]));u.mem_write(POINT,pack(point));u.mem_write(WALL,struct.pack('<Q',wall));u.mem_write(VISIBLE,b'\x01');u.mem_write(CLOSURE,struct.pack('<3Q',WALL,POINT,VISIBLE))
 for j,(a,b,w)in enumerate(segments):u.mem_write(SEGMENTS+j*56,pack(a+b)+bytes(24)+struct.pack('<Q',w)+bytes(8))
 if bucket is None:
  bucket=[math.floor(f(f(center[0]+65)/f(130/64))),math.floor(f(f(center[1]+65)/f(130/64)))] if all(math.isfinite(v)for v in center)else[32,32]
 if segments:
  k=bucket[1]*64+bucket[0];u.mem_write(CELLS+k*48,struct.pack('<3Q',len(segments),SEGMENTS,len(segments)))
 for r,v in[(UC_X86_REG_RCX,SPATIAL),(UC_X86_REG_RDX,CIRCLE),(UC_X86_REG_R8,CLOSURE)]:u.reg_write(r,v)
 run(0x1425df320);return{'visible':u.mem_read(VISIBLE,1)[0],**obs}
# Narrow original arithmetic without cell geometry: source wall already matched.
def distance(a,b,p):
 global obs
 obs={'narrow':True}
 sp=reset();u.mem_write(SEGMENTS,pack(a+b)+bytes(40));u.mem_write(POINT,pack(p));u.reg_write(UC_X86_REG_RAX,SEGMENTS);u.reg_write(UC_X86_REG_R8,0);u.reg_write(UC_X86_REG_R15,POINT);u.reg_write(UC_X86_REG_XMM7,bits(1e-20));u.reg_write(UC_X86_REG_XMM8,bits(1));u.reg_write(UC_X86_REG_XMM9,bits(.4))
 try:u.emu_start(0x1425df5e4,0x1425df666,count=1000)
 except Exception as e:raise RuntimeError(f'{e}; pc={u.reg_read(UC_X86_REG_RIP):#x} rax={u.reg_read(UC_X86_REG_RAX):#x} r8={u.reg_read(UC_X86_REG_R8):#x} r15={u.reg_read(UC_X86_REG_R15):#x}') from e
 return{'distance':u.reg_read(UC_X86_REG_XMM0)&0xffffffff,'t':u.reg_read(UC_X86_REG_XMM4)&0xffffffff}
nextdown=value_bits(bits(.4)-1);nextup=value_bits(bits(.4)+1)
geometry=[([-1,0],[1,0],[0,0,0]),([-1,0],[1,0],[0,0,nextdown]),([-1,0],[1,0],[0,0,.4]),([-1,0],[1,0],[0,0,nextup]),([0,0],[0,0],[.1,0,.1]),([0,0],[0,0],[.4,0,0]),([0,0],[1e-12,0],[.1,0,.1]),([-1,0],[1,0],[float('nan'),0,0]),([float('nan'),0],[1,0],[0,0,0]),([-1,0],[float('inf'),0],[0,0,0])]
cases=[]
for a,b,p in geometry:
 a=list(map(f,a));b=list(map(f,b));p=list(map(f,p));rawdist=distance(a,b,p);full=query([p[0]if math.isfinite(p[0])else 0,p[2]if math.isfinite(p[2])else 0],.4,p,[(a,b,73)])
 expected=int(not(value_bits(rawdist['distance'])<f(.4)));assert full['visible']==expected,(a,b,p,rawdist,full)
 cases.append({'a':fb(a),'b':fb(b),'point':fb(p),'narrow':rawdist,'query':full})
# ID filter must happen before arithmetic; first matching hit ends iteration.
order=[query([0,0],.4,[0,0,0],[([float('nan'),0],[float('nan'),0],74),([-1,0],[1,0],73),([0,0],[0,0],73)]),query([0,0],.4,[0,0,0],[([-1,0],[1,0],74)]),query([0,0],.4,[0,0,0],[([-1,0],[1,0],73),([-1,0],[1,0],73)])]
assert order[0]['visible']==0 and order[0]['segment_checks']==[0,1]and order[1]['visible']==1 and order[2]['segment_checks']==[0]
grid=[]
for center,radius in[([0,0],.4),([-65,-65],.4),([65,65],.4),([-100,0],.4),([100,0],.4),([0,0],0),([0,0],-.4),([0,0],3),([float('nan'),0],.4),([0,float('nan')],.4)]:
 actual=query(center,radius,[0,0,0],[]);grid.append({'center':fb(center),'radius':bits(radius),'query':actual})
def grid_reference(c,r):
 lo=[max(-65.,f(v-r))if not math.isnan(f(v-r))else -65. for v in c];hi=[min(65.,f(v+r))if not math.isnan(f(v+r))else 65. for v in c]
 togrid=lambda v:math.floor(f(f(65+v)/f(130/64)))
 lo=[max(0,togrid(v))for v in lo];hi=[min(63,togrid(v))for v in hi]
 return[[x,z]for z in range(lo[1],hi[1]+1)for x in range(lo[0],hi[0]+1)]
for case in grid:
 c=[value_bits(v)for v in case['center']];r=value_bits(case['radius']);assert case['query']['cells']==grid_reference(c,r)
def arithmetic_reference(a,b,p):
 q=[f(p[0]-a[0]),f(p[2]-a[1])];d=[f(b[j]-a[j])for j in range(2)];dot=f(f(q[0]*d[0])+f(q[1]*d[1]));length=f(f(d[0]*d[0])+f(d[1]*d[1]));den=length if length>f(1e-20)else f(1e-20);ratio=f(dot/den);clamped=0. if 0.>ratio else ratio;clamped=1. if 1.<clamped else clamped;nearest=[f(f(f(1-clamped)*a[j])+f(clamped*b[j]))for j in range(2)];delta=[f(p[0]-nearest[0]),f(p[2]-nearest[1])];dist=f(math.sqrt(f(f(delta[0]*delta[0])+f(delta[1]*delta[1]))));return{'distance':bits(dist),'t':bits(clamped)}
rng=random.Random(73);finite=[]
for _ in range(48):
 a=[f(rng.uniform(-60,60))for _ in range(2)];b=[f(rng.uniform(-60,60))for _ in range(2)];point=[f(rng.uniform(-60,60)),0.,f(rng.uniform(-60,60))];actual=distance(a,b,point);expected=arithmetic_reference(a,b,point);assert actual==expected;(finite.append({'a':fb(a),'b':fb(b),'point':fb(point),'narrow':actual}))
for a,b,point in[([0.,0.],[1e-12,0.],[1e-12,0.,.1]),([1.,1.],[1.,1.],[1.,0.,1.]),([-0.,0.],[0.,-0.],[-0.,0.,0.])]:
 a=list(map(f,a));b=list(map(f,b));point=list(map(f,point));actual=distance(a,b,point);assert actual==arithmetic_reference(a,b,point);finite.append({'a':fb(a),'b':fb(b),'point':fb(point),'narrow':actual})
assert order[0].get('distance_checks')==[1]and order[1].get('distance_checks',[])==[]and order[2].get('distance_checks')==[0]
report={'binary_sha256':hashlib.sha256(raw).hexdigest(),'method':__doc__,'geometry':cases,'filter_order':order,'grid':grid,'finite_arithmetic':finite}
(HERE/'native-tree-wall-fixtures.json').write_text(json.dumps(report,indent=2)+'\n');print({'distance_cases':len(cases),'full_geometry_queries':len(cases),'filter_order':len(order),'grid':len(grid),'finite_arithmetic':len(finite)})
