"""执行原完整global raycast与原predicate；仅隔离QueryPipeline/ColliderSet服务。

原17槽控制流、winner查表/世代验证、pose converter和barycentric均原码。
QueryPipeline回调提供明确的候选相交记录，不把该服务边界冒称原QBVH验证。
"""
from pathlib import Path
import hashlib,json,random,struct
import pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *

ROOT=Path(__file__).resolve().parents[3]
SOURCE=Path(__file__);raw=(ROOT/'B版本内容/tiny-glade.rne').read_bytes()
assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);image=pe.get_memory_mapped_image();base=pe.OPTIONAL_HEADER.ImageBase
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(image)+4095)&~4095);u.mem_write(base,image)
D=0x20000000;u.mem_map(D,0x200000)
W,E,RAY,OUT,STACK,STOP=D+0x1000,D+0x10000,D+0x12000,D+0x13000,D+0x1f0008,D+0x1ff000
INVERSE=[0,2,3,6,10,11,8,7,4,1,9,12,13,14,5,15,16]
state={}
def q(a,x):u.mem_write(a,struct.pack('<Q',x))
def uq(a):return struct.unpack('<Q',u.mem_read(a,8))[0]
def dw(a):return struct.unpack('<I',u.mem_read(a,4))[0]
def f(a,values):u.mem_write(a,struct.pack('<'+'f'*len(values),*values))
def fb(value):return struct.unpack('<I',struct.pack('<f',value))[0]
def ret(value=0):
 sp=u.reg_read(UC_X86_REG_RSP);u.reg_write(UC_X86_REG_RIP,uq(sp));u.reg_write(UC_X86_REG_RSP,sp+8);u.reg_write(UC_X86_REG_RAX,value)
def hook(uc,at,size,_):
 if at==0x140b945d0:
  slot=(u.reg_read(UC_X86_REG_RDX)-W-0x50)//0x390
  sp=u.reg_read(UC_X86_REG_RSP);limit=struct.unpack('<f',u.mem_read(sp+0x30,4))[0]
  solid=u.mem_read(sp+0x38,1)[0];ray=uq(sp+0x28)
  assert bytes(u.mem_read(ray,24))==state['ray_bytes'] and solid==1
  state['query_trace'].append(dict(slot=slot,limit=fb(limit),solid=solid))
  out=u.reg_read(UC_X86_REG_RCX);row=state['by_slot'].get(slot)
  if row is None or row['toi']>limit or row.get('service_miss'):
   u.mem_write(out+8,struct.pack('<I',4))
  else:
   u.mem_write(out,struct.pack('<IIIIf3f',row['index']+7,11,row.get('feature',2),row.get('face',0),row['toi'],*row.get('fallback',[0.,1.,0.])))
  ret()
 elif at==0x140b93940:
  slot=(u.reg_read(UC_X86_REG_RCX)-W-0x20)//0x390
  state['physical_trace'].append(slot)
  if state['mode']=='predicate':row=state['colliders'][0]
  else:row=state['by_slot'][slot]
  ret(0 if row.get('physical_missing') else D+0x50000+row['index']*0x100)
 elif at==0x1409e9a20:
  limit=struct.unpack('<f',struct.pack('<I',u.reg_read(UC_X86_REG_XMM3)&0xffffffff))[0]
  state['terrain_limit']=fb(limit);out=u.reg_read(UC_X86_REG_RCX);toi=state['terrain_toi']
  if toi is None or not toi<=limit:u.mem_write(out,struct.pack('<I',0))
  else:
   position=state.get('terrain_position',[0.,12.-toi,0.])
   u.mem_write(out,struct.pack('<If3f',1,toi,*position))
  ret(out)
 elif at in [0x1428d8725,0x1428d9310,0x1428d9330,0x1428d9518]:
  state['panic']={0x1428d8725:'receipt',0x1428d9310:'physical',0x1428d9330:'physical',0x1428d9518:'bounds'}[at]
  uc.emu_stop()
u.hook_add(UC_HOOK_CODE,hook)

def collider(slot,toi,variant=0,**kw):
 row=dict(slot=slot,toi=float(toi),primitive=False,uv=True,normals=True,
  translation=[0.,12.-float(toi),0.],rotation=[0.,0.,0.,1.],
  vertices=[-4.,0.,-4.,-4.,0.,8.,8.,0.,-4.],normal_values=[0.,1.,0.]*3,uv_values=[0.,0.,0.,1.,1.,0.])
 if variant%4==1:row['rotation']=[0.,.6,0.,.8];row['translation'][0]=1.;row['translation'][2]=-1.
 elif variant%4==2:row['normals']=False
 elif variant%4==3:row['uv']=False
 row.update(kw);return row

def run(case,mode='main'):
 state.clear();state.update(mode=mode,query_trace=[],physical_trace=[],colliders=case['colliders'],by_slot={})
 if mode=='terrain':state['terrain_toi']=case.get('terrain_toi');state.update({k:v for k,v in case.items()if k=='terrain_position'})
 u.mem_write(W,bytes(0x5000));u.mem_write(E,bytes(0x1000));u.mem_write(OUT,b'\xcd'*64)
 q(W+8,0 if case.get('missing_registry') else E);q(W+0x10,len(case['colliders']))
 for index,row in enumerate(case['colliders']):
  row['index']=index;state['by_slot'][row['slot']]=row
  entry=E+index*40;m=D+0x20000+index*0x1000;c=D+0x50000+index*0x100
  u.mem_write(m,bytes(0x500));u.mem_write(c,bytes(0x100))
  source=bytearray(24);source[0]=row.get('tag',INVERSE[row['slot']]);source[8:16]=struct.pack('<Q',row.get('wall',1000+row['slot']))
  row['source']=list(source);u.mem_write(entry,bytes(source));q(entry+0x18,m);u.mem_write(entry+0x20,struct.pack('<I',1))
  pose=list(row['translation'])
  if row['primitive']:pose[1]-=1. # 测试box的local center是-Y，顶面仍在输入translation.y。
  q(c,(row.get('raw_generation',1)<<32)|row.get('raw_index',index));f(c+0x34,row['rotation']+pose)
  if not row['primitive']:
   q(m+0x10,m+0x100);q(m+0x18,3);q(m+0x20,m+0x200);q(m+0x28,1)
   u.mem_write(m+0x200,struct.pack('<3I',*(row.get('indices',[0,1,2]))));f(m+0x100,row['vertices'])
   if row['normals']:q(m+0x40,m+0x300);q(m+0x48,len(row['normal_values'])//3);f(m+0x300,row['normal_values'])
   if row['uv']:q(m+0x50,m+0x400);q(m+0x58,len(row['uv_values'])//2);f(m+0x400,row['uv_values'])
 ray=case.get('ray',[0.,12.,0.,0.,-1.,0.]);state['ray_bytes']=struct.pack('<6f',*ray);u.mem_write(RAY,state['ray_bytes'])
 u.mem_write(STACK-0x1000,bytes(0x2000));q(STACK,STOP)
 if mode=='terrain':
  f(STACK+0x28,[case.get('max_toi',3.4028234663852886e38)]);u.mem_write(STACK+0x30,struct.pack('<I',case['mask']))
  registers=[(UC_X86_REG_RCX,OUT),(UC_X86_REG_RDX,W),(UC_X86_REG_R8,RAY),(UC_X86_REG_R9,D+0x60000)];entry=0x1408cd430
 elif mode=='main':
  q(STACK+0x28,case.get('excluded_wall',0));f(STACK+0x30,[case.get('max_toi',3.4028234663852886e38)]);u.mem_write(STACK+0x38,struct.pack('<I',case['mask']))
  registers=[(UC_X86_REG_RCX,OUT),(UC_X86_REG_RDX,W),(UC_X86_REG_R8,RAY),(UC_X86_REG_R9,int('excluded_wall'in case))];entry=0x1408cdc60
 else:
  env,option=D+0x15000,D+0x15100;q(env,W);q(env+8,W+0x20+case['colliders'][0]['slot']*0x390);q(env+16,option)
  q(option,int('excluded_wall'in case));q(option+8,case.get('excluded_wall',0))
  registers=[(UC_X86_REG_RCX,env),(UC_X86_REG_RDX,7),(UC_X86_REG_R8,11)];entry=0x1408cab80
 for register,value in registers+[(UC_X86_REG_RSP,STACK)]:u.reg_write(register,value)
 u.emu_start(entry,STOP,count=20000)
 assert state.get('panic') or u.reg_read(UC_X86_REG_RIP)==STOP
 result=dict(case);result['query_trace']=state['query_trace'];result['physical_trace']=state['physical_trace'];result['panic']=state.get('panic')
 if mode=='terrain':result['terrain_limit']=state.get('terrain_limit')
 if mode=='predicate':result['allowed']=None if result['panic'] else bool(u.reg_read(UC_X86_REG_RAX)&255)
 elif not result['panic']:
  words=list(struct.unpack('<16I',u.mem_read(OUT,64)));result['output']=words
 return result

cases=[]
for slot in range(17):cases.append(dict(name='slot'+str(slot),mask=1<<slot,colliders=[collider(slot,slot%5+1,slot)]))
for variant in range(24):
 mask=[0,0x1ffff,0xfb,1<<16,0x80000000][variant%5] if variant<5 else random.Random(variant).getrandbits(17)
 cases.append(dict(name='layers'+str(variant),mask=mask,colliders=[collider(slot,slot%5+1,slot+variant) for slot in range(17)]))
for variant in range(24):cases.append(dict(name='pose'+str(variant),mask=2,colliders=[collider(1,3.+variant%6,variant)]))
for kind in ['primitive','back_face','no_face','bad_face','bad_vertices','even','bad_generation','bad_index','physical_missing','missing_registry']:
 row=collider(1,4.);case=dict(name=kind,mask=2,colliders=[row])
 if kind=='primitive':row['primitive']=True
 if kind=='back_face':row['face']=1
 if kind=='no_face':row['feature']=0
 if kind=='bad_face':row['face']=2
 if kind=='bad_vertices':row['indices']=[0,1,9]
 if kind=='even':row['raw_generation']=0
 if kind=='bad_generation':row['raw_generation']=3
 if kind=='bad_index':row['raw_index']=0xffffffff
 if kind=='physical_missing':row['physical_missing']=True
 if kind=='missing_registry':case['missing_registry']=True
 cases.append(case)
for empty in ['normal_values','uv_values']:
 row=collider(1,4.);row[empty]=[];cases.append(dict(name='Some_empty_'+empty,mask=2,colliders=[row]))
# 原只转换最终winner：早槽坏face在后槽更近时不执行；后槽更远时仍须失败。
for near in [True,False]:cases.append(dict(name='deferred_bad_face_'+str(near),mask=3,
 colliders=[collider(0,5.,face=2),collider(1,3. if near else 7.)]))
for near in [True,False]:cases.append(dict(name='deferred_bad_vertices_'+str(near),mask=3,
 colliders=[collider(0,5.,indices=[0,1,9]),collider(1,3. if near else 7.)]))
cases.append(dict(name='bounded_miss',mask=2,max_toi=2.5,colliders=[collider(1,3.)]))
rows=[run(case) for case in cases]
terrain_cases=[]
for uv in [False,True]:
 for terrain_toi in [None,1.,4.,7.]:terrain_cases.append(dict(mask=2|0x20,terrain_toi=terrain_toi,colliders=[collider(1,4.,uv=uv)]))
for limit in [2.,4.]:
 for toi in [1.,3.]:terrain_cases.append(dict(mask=2|0x20,max_toi=limit,terrain_toi=toi,colliders=[collider(1,4.)]))
for mask in [2,0x20,0]:terrain_cases.append(dict(mask=mask,terrain_toi=1.,colliders=[collider(1,4.)]))
terrain_cases.append(dict(mask=2|0x20,terrain_toi=1.,terrain_position=[77.,-13.,101.],colliders=[collider(1,4.)]))
terrain_rows=[run(case,'terrain')for case in terrain_cases];assert len(terrain_rows)==16
predicates=[]
for kind in range(17):
 for excluded in [None,7,8]:
  for generation in [0,1,2,3]:
   case=dict(mask=1,colliders=[collider(0,1.,tag=kind,wall=7,raw_generation=generation)])
   if excluded is not None:case['excluded_wall']=excluded
   predicates.append(run(case,'predicate'))
fixture=ROOT/'tiny-glade/tests/fixtures/native/m01-global-ray.json'
fixture.write_text(json.dumps(dict(cases=rows,predicates=predicates,terrain_cases=terrain_rows),separators=(',',':'))+'\n')
report=dict(original_sha256=hashlib.sha256(raw).hexdigest(),entries=['0x1408cdc60','0x1408cab80','0x1408ce4a0','0x1416c2f20'],
 source_sha256=hashlib.sha256(SOURCE.read_bytes()).hexdigest(),fixture_sha256=hashlib.sha256(fixture.read_bytes()).hexdigest(),
 cases=len(rows),predicate_cases=len(predicates),terrain_wrapper_cases=len(terrain_rows),arithmetic_bridges=[],
 service_bridges=['QueryPipeline supplies fixture intersections and respects incoming max_toi; raw original slot loop/winner/validation/converter execute',
 'ColliderSet::get_mut_internal returns selected fixture collider','TerrainHeightsData::intersect_ray supplies explicit Option<TOI,point> fixture respecting limit','panic entry captures failure category'],
 scope='Full original global business control flow and full converter/barycentric. Does not execute original Rapier QBVH or generation arena; actual Rust World/Rapier is tested separately.')
SOURCE.with_suffix('.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps(dict(cases=len(rows),predicates=len(predicates),terrain_cases=len(terrain_rows),panics=sum(bool(r['panic'])for r in rows))))
