"""Original B draw_wall entry for fresh unsnapped wall/fence chains.
Original reader, nursery, smoother, curve construction/growth/finalization,
metadata rules and command writes execute. Wall map hashing and authority
methods resolve one real row; allocator/CRT use the shared validated bridge.
The original new-wall constructor is checked separately, not replaced by a
claim that this resolved row bridge proves its entire HashMap implementation.
"""
from pathlib import Path
import json,struct,hashlib
ROOT=Path('/Users/yuluo/Desktop/game');OUT=Path(__file__).resolve().parent
recipe=ROOT/'games_test/reverse-engineering/tiny-glade/evidence/shared-foundation-2026-10-13/utils-base/native_spline_rdp.py'
s=recipe.read_text().split('def spline(')[0].replace("Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne')", "Path('/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne')")
exec(compile(s,str(recipe),'exec'),globals());OUT=Path(__file__).resolve().parent
at=INPUT+0x100;calls=[];WALLSTATE=0;PROGRESS=0
MIN=0x8000000000000000
def q(p):return struct.unpack('<Q',u.mem_read(p,8))[0]
def put(p,*vs):u.mem_write(p,struct.pack('<'+'Q'*len(vs),*vs))
def new(n):
 global at
 p=at;at+=(n+15)&~15;assert at<INPUT+0x1e000;return p
def res(p):
 r=new(32);t=new(8);put(r,p,0,t);u.mem_write(r+28,struct.pack('<I',99));return r
def event(stride):
 p=new(72);buf=new(stride*32);put(p+32,32,buf,0,0,0);return res(p),p,buf,stride
def curve(p):
 cap,ptr,n,ucap,uptr,un=struct.unpack('<6Q',u.mem_read(p,48))
 return {'points':list(struct.unpack('<'+'I'*(2*n),u.mem_read(ptr,8*n)))if n else [],'points_u':list(struct.unpack('<'+'I'*un,u.mem_read(uptr,4*un)))if un else [],'length':struct.unpack('<I',u.mem_read(p+48,4))[0]}
def deps(uc,a,size,data):
 trace.append(a)
 if len(trace)>80:del trace[:-80]
 if a==0x140c16a00:ret(0)
 elif a==0x1405b0d70:ret(0)
 elif a==0x14085a200:
  ptr=u.reg_read(UC_X86_REG_RDX);kind=u.reg_read(UC_X86_REG_R8)&255
  calls.append({'type':'create','kind':kind,'curve':curve(ptr)})
  u.mem_write(WALLSTATE,bytes(u.mem_read(ptr,56)));u.mem_write(WALLSTATE+56,struct.pack('<I',2));u.mem_write(WALLSTATE+84,pack([3.9 if kind==0 else 1.2,0,2.8,0]));u.mem_write(WALLSTATE+100,bytes([0,int(kind!=0),0,0,0,5]));ret(7)
 elif a==0x140aa99e0:
  ty=u.mem_read(u.reg_read(UC_X86_REG_R8),1)[0];u.mem_write(WALLSTATE+102,bytes([ty]));calls.append({'type':'freehand','value':ty});ret(1)
 elif a==0x140aaa090:
  p=u.reg_read(UC_X86_REG_R8);calls.append({'type':'curve','curve':curve(p)});u.mem_write(WALLSTATE,bytes(u.mem_read(p,56)));ret(1)
u.hook_add(UC_HOOK_CODE,deps)
def chain(kind,positions,ticks=1,delta=1/60,settings_freehand=0):
 global at,calls,WALLSTATE,PROGRESS
 sp=reset();u.mem_write(INPUT,bytes(0x1f000));at=INPUT+0x100;calls=[]
 walls=new(256);row=new(128);WALLSTATE=row+8;ctrl=new(32);put(row,7);u.mem_write(ctrl,b'\x00'+b'\xff'*31);put(walls,ctrl,7,7,1);walls_r=res(walls)
 # row stride120 immediately precedes the controls (including the wall id).
 WALLSTATE=ctrl-120+8;put(ctrl-120,7)
 PROGRESS=new(176);put(PROGRESS,MIN);progress=res(PROGRESS)
 nursery=new(32);pts=new(128);put(nursery,16,pts,0);u.mem_write(nursery+24,bytes([kind]));nursery_r=res(nursery)
 snap=new(64);cursor=new(32);continuation=new(128);put(continuation,MIN);time=new(64);put(time+24,ticks);u.mem_write(time+40,pack([delta]));cam=new(512);u.mem_write(cam+0x14,pack([1]));u.mem_write(cam+0xb8,pack([-10]));window=new(8);u.mem_write(window,struct.pack('<II',1920,1080))
 tuple4=new(192)
 for i,p in enumerate([snap,cursor,continuation,time,cam,window]):u.mem_write(tuple4+32*i,bytes(u.mem_read(res(p),32)))
 reader=new(16);cursor_id=new(8);events=new(72);buf=new(16);put(events+32,2,buf,0,0,0);put(reader,cursor_id,events)
 audio=event(32);height=event(48);wall_color=event(24);half_color=event(24);pins=event(56)
 cols=new(256);put(cols,64,new(256),64)
 walltuple=new(64);u.mem_write(walltuple,bytes(u.mem_read(wall_color[0],32)));u.mem_write(walltuple+32,bytes(u.mem_read(res(cols),32)))
 halftuple=new(64);u.mem_write(halftuple,bytes(u.mem_read(half_color[0],32)));u.mem_write(halftuple+32,bytes(u.mem_read(res(cols),32)))
 settings=new(96);u.mem_write(settings+0x18,struct.pack('<I',23));u.mem_write(settings+0x2c,struct.pack('<I',42));u.mem_write(settings+0x40,bytes([settings_freehand]));settings=res(settings)
 args=[walls_r,progress,nursery_r,tuple4,reader,audio[0],height[0],walltuple,halftuple,settings,pins[0]]
 frames=[]
 for idx,point in enumerate(positions):
  put(events+48,1,idx,idx+1);put(buf,idx);u.mem_write(cursor,struct.pack('<I3f',1,*point))
  u.reg_write(UC_X86_REG_RSP,sp);put(sp,STOP)
  for r,v in zip([UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8,UC_X86_REG_R9],args[:4]):u.reg_write(r,v)
  for i,v in enumerate(args[4:]):put(sp+40+8*i,v)
  before=len(calls)
  try:run(0x140a7cba0)
  except Exception:
   print({'frame':idx,'calls':calls,'trace':[hex(x)for x in trace]});raise
  n=q(nursery+16);active=q(PROGRESS)!=MIN
  fs={'point':[bits(x)for x in point],'calls':calls[before:],'nursery_points':list(struct.unpack('<'+'I'*(2*n),u.mem_read(pts,n*8)))if n else [],'active':active,'reader_cursor':q(cursor_id)}
  if active:
   wn=q(PROGRESS+72);wp=q(PROGRESS+64)
   fs.update(raw_curve=curve(PROGRESS),weights=list(struct.unpack('<'+'Q'*wn,u.mem_read(wp,wn*8)))if wn else [],wall=q(PROGRESS+128),first_cursor=list(struct.unpack('<3I',u.mem_read(PROGRESS+136,12))),metadata=list(struct.unpack('<2I3B',u.mem_read(PROGRESS+148,11))),authority=curve(WALLSTATE))
  ev={}
  for name,e in [('audio',audio),('height',height),('wall_color',wall_color),('half_color',half_color),('pins',pins)]:
   count=q(e[1]+48);ev[name]=[bytes(u.mem_read(e[2]+i*e[3],e[3])).hex()for i in range(count)]
  fs['events']=ev;frames.append(fs)
 return {'kind':kind,'ticks':ticks,'delta':bits(delta),'freehand':settings_freehand,'frames':frames}
if __name__=='__main__':
 cases=[chain(k,[(0,0,0),(.1,0,0),(.2,0,0),(.3,0,.1),(.3,0,.1),(.6,0,.4)],settings_freehand=f)for k in [0,1]for f in [0,1]]
 report={'target':'B','binary_sha256':hashlib.sha256(raw).hexdigest(),'entry':'0x140a7cba0','method':__doc__,'cases':cases};(OUT/'native-draw-wall.json').write_text(json.dumps(report,indent=2)+'\n');print({'chains':len(cases),'frames':sum(len(c['frames'])for c in cases)})
