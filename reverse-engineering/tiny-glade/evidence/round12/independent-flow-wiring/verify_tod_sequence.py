from pathlib import Path
import json,struct,hashlib,math,ctypes
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE,UC_HOOK_MEM_UNMAPPED
from unicorn.x86_const import *
base=Path(__file__).parent
exec((base/'extract_startup_followup.py').read_text().split('addresses=')[0])
out=base
crt=ctypes.CDLL('ucrtbase');fmod=crt.fmodf;fmod.argtypes=[ctypes.c_float,ctypes.c_float];fmod.restype=ctypes.c_float
f32=lambda x:struct.unpack('<f',struct.pack('<f',x))[0]
bits=lambda x:struct.unpack('<I',struct.pack('<f',x))[0]
def native(entries,t):
 u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(0x60000000,0x40000);u.mem_map(0x70000000,0x100000);u.mem_map(0x50000000,0x1000);u.mem_write(0x50000000,b'\xc3')
 status=[]
 def mem(uc,access,addr,size,value,user):
  page=addr&~4095
  if 0x140000000<=page<0x144000000:
   try:b=read(page,4096)
   except ValueError:return False
   uc.mem_map(page,4096);uc.mem_write(page,b);return True
  return False
 def code(uc,addr,size,user):
  if addr==0x50000000:status.append('ok');uc.emu_stop()
  elif addr==0x142923f10:
   x=struct.unpack('<f',struct.pack('<I',uc.reg_read(UC_X86_REG_XMM0)&0xffffffff))[0];y=struct.unpack('<f',struct.pack('<I',uc.reg_read(UC_X86_REG_XMM1)&0xffffffff))[0]
   uc.reg_write(UC_X86_REG_XMM0,bits(fmod(x,y)));sp=uc.reg_read(UC_X86_REG_RSP);ret=struct.unpack('<Q',uc.mem_read(sp,8))[0];uc.reg_write(UC_X86_REG_RSP,sp+8);uc.reg_write(UC_X86_REG_RIP,ret)
  elif addr in [0x1428d9518,0x1428d9310]:status.append('panic');uc.emu_stop()
 u.hook_add(UC_HOOK_MEM_UNMAPPED,mem);u.hook_add(UC_HOOK_CODE,code)
 u.mem_write(0x60000000,struct.pack('<QQQ',len(entries),0x60001000,len(entries)))
 for i,(t0,s) in enumerate(entries):u.mem_write(0x60001000+i*80,struct.pack('<Qf17f',100+i,t0,*s))
 sp=0x70080008;u.mem_write(sp,struct.pack('<Q',0x50000000));u.reg_write(UC_X86_REG_RSP,sp);u.reg_write(UC_X86_REG_RCX,0x60020000);u.reg_write(UC_X86_REG_RDX,0x60000000);u.reg_write(UC_X86_REG_XMM2,bits(t))
 u.emu_start(0x140b067d0,0,count=100000)
 assert status,hex(u.reg_read(UC_X86_REG_RIP))
 return status[0],list(struct.unpack('<17I',u.mem_read(0x60020000,68))) if status[0]=='ok' else None
def model(entries,t):
 if len(entries)==1:return 'ok',[bits(x) for x in entries[0][1]]
 if not entries:return 'panic',None
 t=f32(fmod(t,1));t=f32(t+1) if t<0 else t
 first=entries[0];last=entries[-1]
 if t<first[0]:a,b,lo,hi=last,first,f32(last[0]-1),first[0]
 else:
  pair=next(((a,b) for a,b in zip(entries,entries[1:]) if a[0]<=t and t<=b[0]),None)
  if pair:a,b=pair;lo,hi=a[0],b[0]
  else:a,b,lo,hi=last,first,last[0],f32(first[0]+1)
 num=f32(t-lo);den=f32(hi-lo)
 if den==0:raw=math.nan if num==0 else math.copysign(math.inf,num*math.copysign(1,den))
 else:raw=f32(num/den)
 if math.isnan(raw):return 'ok',[bits(x) for x in a[1]]
 weight=min(1,max(0,raw));inverse=f32(1-weight);s=[]
 for i,(x,y) in enumerate(zip(a[1],b[1])):
  if i in [12,16]:v=f32(f32(f32(y-x)*weight)+x)
  else:v=f32(f32(y*weight)+f32(x*inverse))
  s.append(bits(v))
 return 'ok',s
def vec(i):return [f32((j+1)*(i+1)*0.137-2.25) for j in range(17)]
fixture=json.loads(Path('D:/game/ljxsj_92385/Tiny Glade/assets/tod/default.json').read_text())
original=[]
for e in fixture['entries']:
 s=e['snapshot'];c=s['sun_sky'];flat=sum([c[x] for x in ['sun_tint_color','moon_tint_color','sky_low_color','sky_high_color']],[])+[s['base_fog_thickness']]+s['fog_tint']+[s['speed_bias']]
 original.append((f32(e['time_of_day']),[f32(x) for x in flat]))
seqs=[[],[(0.2,vec(0))],[(0.2,vec(0)),(0.7,vec(1))],[(0,vec(0)),(.5,vec(1)),(.95,vec(2))],[(.2,vec(0)),(.2,vec(1))],[(.7,vec(0)),(.2,vec(1)),(.8,vec(2))],[(math.nan,vec(0)),(.5,vec(1)),(.8,vec(2))],[(.2,vec(0)),(.5,vec(1)),(math.nan,vec(2))],original]
times=[-2.125,-1.0,-.25,-0.0,0.0,.05,.19999999,.2,.20000002,.5,.7,.8,.95,.975,.99999994,1.0,1.125,2.5,math.nan,math.inf,-math.inf]
rows=[]
for si,entries in enumerate(seqs):
 entries=[(f32(x),s) for x,s in entries]
 for t in times:
  t=f32(t);actual=native(entries,t);expected=model(entries,t)
  # NaN payload signs are not normalized; ordinary stored snapshots are finite.
  assert actual==expected,(si,t,actual,expected)
  rows.append({'sequence':si,'tod_bits':hex(bits(t)),'native_status':actual[0],'native_bits':actual[1]})
(out/'tod-sequence-native-policy.json').write_text(json.dumps({'original_sha256':sha,'functions':{hex(v):{'size':procs[v][0],'sha256':hashlib.sha256(read(v,procs[v][0])).hexdigest()} for v in [0x140b067d0,0x140c904a0,0x140a97650]},'method':'Unicorn executes complete original snapshot wrapper + original inverse_lerp + original SunSkyColors lerp. Only fmodf imported std/C runtime is delegated to host ucrtbase.fmodf; original code/constants read from SHA-bound PE. No Rust candidate execution.','cases':len(rows),'mismatches':0,'results':rows},indent=2),encoding='utf-8')
(out/'tod-sequence-native-fixture.json').write_text(json.dumps({'original_sha256':sha,'sequences':[[{'tod_bits':bits(t0),'snapshot_bits':[bits(v) for v in s]} for t0,s in entries] for entries in seqs],'results':rows},separators=(',',':')),encoding='utf-8')
print('native full TOD snapshot',len(rows),'cases passed')
