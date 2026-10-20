from pathlib import Path
import struct,json,math,ctypes
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE,UC_HOOK_MEM_UNMAPPED
from unicorn.x86_const import *
here=Path(__file__).parent
exec((here.parent/'extract_startup_followup.py').read_text().split('addresses=')[0])
out=here
f32=lambda x:struct.unpack('<f',struct.pack('<f',x))[0]
bits=lambda x:struct.unpack('<I',struct.pack('<f',x))[0]
crt=ctypes.CDLL('ucrtbase');power=crt.powf;power.argtypes=[ctypes.c_float,ctypes.c_float];power.restype=ctypes.c_float
def execute(case):
 u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(0x60000000,0x40000);u.mem_map(0x70000000,0x100000)
 rbp=0x70080000;transform=0x60001000;velocity=0x60002000;multiplier=0x60003000;loc=0x60004000;tick=0x60005000;wallptr=0x60010000;spotvec=0x60020000;spotptr=0x60021000
 result={};calls=[];water_index=0;garden_index=0
 def floats(addr,n):return list(struct.unpack('<'+'f'*n,u.mem_read(addr,n*4)))
 def sf(reg):return struct.unpack('<f',struct.pack('<I',u.reg_read(reg)&0xffffffff))[0]
 def ret():
  sp=u.reg_read(UC_X86_REG_RSP);pc=struct.unpack('<Q',u.mem_read(sp,8))[0];u.reg_write(UC_X86_REG_RSP,sp+8);u.reg_write(UC_X86_REG_RIP,pc)
 def memory(uc,access,addr,size,value,user):
  page=addr&~4095
  if 0x140000000<=page<0x144000000:
   try:b=read(page,4096)
   except ValueError:return False
   uc.mem_map(page,4096);uc.mem_write(page,b);return True
  return False
 def code(uc,addr,size,user):
  nonlocal water_index,garden_index
  if addr==0x142319dbf:result['advanced']=True
  if addr in [0x1423194ff,0x142319506]:result['status']='ok';uc.emu_stop();return
  if addr in [0x14231a371,0x14231a37f,0x1428d9310]:result['status']='panic';uc.emu_stop();return
  if addr==0x140a14df0:
   view=uc.reg_read(UC_X86_REG_RCX);channel='garden' if view==rbp+0x118 else 'water';values=case[channel];index=garden_index if channel=='garden' else water_index
   value=values[min(index,len(values)-1)];garden_index+=channel=='garden';water_index+=channel=='water'
   calls.append({'kind':channel,'position':[bits(sf(UC_X86_REG_XMM1)),bits(sf(UC_X86_REG_XMM2))]});uc.reg_write(UC_X86_REG_XMM0,bits(value));ret()
  elif addr==0x1409ea800:
   calls.append({'kind':'rock','position':[bits(sf(UC_X86_REG_XMM2)),bits(sf(UC_X86_REG_XMM3))]});uc.reg_write(UC_X86_REG_RAX,case['rock']);ret()
  elif addr==0x140aa5730:
   calls.append({'kind':'walls','position':[bits(sf(UC_X86_REG_XMM1)),bits(sf(UC_X86_REG_XMM2))]});uc.reg_write(UC_X86_REG_RAX,wallptr);uc.reg_write(UC_X86_REG_RDX,len(case['walls']));ret()
  elif addr==0x140a91ef0:
   wall=case['walls'][int(uc.reg_read(UC_X86_REG_RDX))-1]
   calls.append({'kind':'public','wall':int(uc.reg_read(UC_X86_REG_RDX))});uc.reg_write(UC_X86_REG_RAX,0 if wall.get('missing',False) else 0x60030000+int(uc.reg_read(UC_X86_REG_RDX))*4);ret()
  elif addr==0x140b2a680:
   i=(uc.reg_read(UC_X86_REG_RCX)-0x60030000)//4-1;uc.reg_write(UC_X86_REG_XMM0,bits(case['walls'][i]['height']));ret()
  elif addr==0x1408ccc90:
   ray=uc.reg_read(UC_X86_REG_R8);origin=floats(ray,3);direction=floats(ray+12,3);radius=sf(UC_X86_REG_XMM3)
   calls.append({'kind':'sphere','origin':[bits(x) for x in origin],'direction':[bits(x) for x in direction],'radius':bits(radius)})
   dst=uc.reg_read(UC_X86_REG_RCX);uc.mem_write(dst,bytes(64));hit=case['sphere']
   uc.mem_write(dst+0x30,struct.pack('<I',2 if hit is None else 0))
   if hit is not None:uc.mem_write(dst+0x18,struct.pack('<3f',*hit))
   ret()
 u.hook_add(UC_HOOK_MEM_UNMAPPED,memory);u.hook_add(UC_HOOK_CODE,code)
 q=case['rotation'];p=case['position'];scale=case['scale'];u.mem_write(transform,struct.pack('<10f',*q,*p,*scale)+bytes(8));u.mem_write(velocity,struct.pack('<3f',*case['velocity']));u.mem_write(loc,struct.pack('<II',case['loc'],0))
 if case['multiplier'] is not None:u.mem_write(multiplier,struct.pack('<f',case['multiplier']))
 for i,wall in enumerate(case['walls']):
  u.mem_write(wallptr+i*56,struct.pack('<4f',*wall['a'],*wall['b'])+bytes(24)+struct.pack('<Q',i+1)+bytes(8))
 u.mem_write(spotvec,struct.pack('<QQQ',spotptr,2,2));u.mem_write(spotptr,bytes(48))
 for offset,value in [(0x190,tick),(0x1d8,spotvec if case['spots'] else 0),(0x128,0x60030000),(0x150,0x60030000),(0x158,0x60030000)]:u.mem_write(rbp+offset,struct.pack('<Q',value))
 for offset,value in [(0x1fc,17),(0x1a4,19),(0x238,case['duck']),(0x23c,case['sheep'])]:u.mem_write(rbp+offset,struct.pack('<I',value))
 for reg,value in [(UC_X86_REG_RBP,rbp),(UC_X86_REG_RSP,rbp-0x80),(UC_X86_REG_RSI,transform),(UC_X86_REG_RAX,velocity),(UC_X86_REG_R13,loc),(UC_X86_REG_RCX,0 if case['multiplier'] is None else multiplier),(UC_X86_REG_R9,tick+4)]:u.reg_write(reg,value)
 dt=case['dt'];retention=power(.01,dt);weight=f32(1-retention)
 for reg,values in [(UC_X86_REG_XMM6,[0]*4),(UC_X86_REG_XMM7,[retention]*4),(UC_X86_REG_XMM8,[weight]*4),(UC_X86_REG_XMM10,[1,0,0,0]),(UC_X86_REG_XMM13,[dt,dt,0,0]),(UC_X86_REG_XMM14,[dt,0,0,0]),(UC_X86_REG_XMM15,[130,0,0,0])]:u.reg_write(reg,int.from_bytes(struct.pack('<4f',*values),'little'))
 try:u.emu_start(0x142319b8b,0,count=100000)
 except Exception as e:raise RuntimeError((case['name'],hex(u.reg_read(UC_X86_REG_RIP)),str(e)))
 assert 'status' in result,(case['name'],hex(u.reg_read(UC_X86_REG_RIP)))
 result.setdefault('advanced',False)
 result['transform_bits']=list(struct.unpack('<10I',u.mem_read(transform,40)));result['spot_bits']=list(struct.unpack('<3I',u.mem_read(spotptr+8,12))) if case['spots'] else None;result['calls']=calls
 return result
def base(name):return {'name':name,'dt':.25,'position':[0,-.25,0],'scale':[2,3,4],'rotation':[0,0,0,1],'velocity':[2,1,-3],'multiplier':1,'loc':3,'duck':False,'sheep':False,'spots':True,'water':[0],'garden':[0],'rock':0,'walls':[],'sphere':None}
cases=[]
for loc in range(9):
 for species in ['other','duck','sheep','both']:
  c=base('loc%d-%s'%(loc,species));c['loc']=loc;c['duck']=species in ['duck','both'];c['sheep']=species in ['sheep','both'];c['water']=[1 if c['duck'] else 0];cases.append(c)
for mult in [None,0,-0.0,-1,math.nan]:
 c=base('mult-'+str(mult));c['multiplier']=mult;cases.append(c)
for dt in [0,-.125,1e-12]:
 c=base('dt-'+str(dt));c['dt']=dt;cases.append(c)
for v in [[0,0,0],[0,2,0],[1e-15,0,0],[math.inf,0,1],[0,0,1],[0,0,-1]]:
 c=base('velocity-'+str(v));c['velocity']=v;cases.append(c)
for kind in ['intersect','parallel','low','missing','flying-missing']:
 c=base('wall-'+kind);c['loc']=3 if kind=='flying-missing' else 2;c['water']=[0];c['velocity']=[4,0,0];c['walls']=[{'a':[.5,-1],'b':[.5,1] if kind!='parallel' else [1.5,-1],'height':-1 if kind=='low' else 2,'missing':kind in ['missing','flying-missing']}];cases.append(c)
for distance in [None,0,.39999998,.4,.99999994,1,1.0000001,math.nan]:
 c=base('sphere-'+str(distance));c['position']=[0,0,0];c['sheep']=True;c['loc']=3;c['sphere']=None if distance is None else [distance,0,0];cases.append(c)
for values in [[0,1],[1,0],[math.nan,1],[0,math.nan]]:
 c=base('garden-'+str(values));c['sheep']=True;c['garden']=values;cases.append(c)
for y in [math.nan,-0.,0.,1.]:
 c=base('y-'+str(y));c['position'][1]=y;cases.append(c)
rows=[]
for case in cases:
 actual=execute(case);rows.append({'input':case,'output':actual})
def encode(v):
 if isinstance(v,float):return {'f32':bits(v)}
 if isinstance(v,list):return [encode(x) for x in v]
 if isinstance(v,dict):return {k:encode(x) for k,x in v.items()}
 return v
(here/'native-move-policy.json').write_text(json.dumps({'original_sha256':sha,'scope':'Original complete per-entity business CFG142319b8b onward; Bevy query traversal is outside harness. Water/garden/rock/public-wall/spherecast owner results are explicit injected inputs. Original segment geometry and original glam calls execute unchanged; std powf delegated to ucrtbase.','cases':len(rows),'results':encode(rows)},indent=2),encoding='utf-8')
print('native movement cases',len(rows),'success',sum(r['output']['status']=='ok' for r in rows),'panic',sum(r['output']['status']=='panic' for r in rows))
