"""原closure完整控制流/投影/相对高度/缓存及目的载荷；曲线/高度依赖显式桥接。"""
from pathlib import Path
import struct,json,itertools,math
source=Path(__file__).with_name('m01_dispatch_start_place_native.py').read_text()
exec(compile(source[:source.index('\nrows=[]')],'identity-bound-pe-setup','exec'))
def fp(reg,v):u.reg_write(reg,int.from_bytes(struct.pack('<f',v).ljust(16,b'\0'),'little'))
def fv(reg):return struct.unpack('<f',struct.pack('<I',r(reg)&0xffffffff))[0]
def anchor_hook(uc,a,size,_):
 rcx,rdx,r8=[r(x)for x in [UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8]]
 if a==0x140a91ef0:ret(D+0x9000 if wall_present else 0)
 elif a==0x142923f10:fp(UC_X86_REG_XMM0,math.fmod(fv(UC_X86_REG_XMM0),fv(UC_X86_REG_XMM1)));ret()
 elif a==0x140b2a740:ret(int(loop));fp(UC_X86_REG_XMM0,length)
 elif a==0x140b2a890:
  dependency_calls.append(['height',fv(UC_X86_REG_XMM1),fv(UC_X86_REG_XMM2)]);fp(UC_X86_REG_XMM0,1.25);ret()
 elif a==0x140c92090:
  cu,cy=struct.unpack('<2f',u.mem_read(rdx,8));dependency_calls.append(['to_world',cu,cy]);u.mem_write(rcx,struct.pack('<3f',cu,cy,0.));ret(rcx)
 elif a==0x140c924f0:fp(UC_X86_REG_XMM0,tangent[0]);fp(UC_X86_REG_XMM1,tangent[1]);ret()
 elif a==0x140a1ada0:caches.append({'id':rdx&0xffffffff,'transform_bits':list(struct.unpack('<10I',u.mem_read(r8,40)))});ret()
 elif a in [0x1428d9310,0x1428d9390,0x1428d9430]:panics.append(hex(a));uc.emu_stop()
u.hook_add(UC_HOOK_CODE,anchor_hook)
rows=[]
for loop,source_u,offset,source_tag,tangent in itertools.product([False,True],[0.,5.,19.5],[(0.,0.),(-1.75,.5),(1.75,-.5)], [0,1],[(1.,0.),(.6,.8)]):
 for with_offset in [False,True]:
  u.mem_write(D,bytes(0x10000));u.mem_write(S,bytes([0xa5])*0x100000);length=20.;wall_present=True
  env,addr,coord,dst,out,desc=D+0x1000,D+0x2000,D+0x3000,D+0x4000,D+0x5000,D+0x6000
  for i,v in enumerate([D+0x8000,addr,coord,dst,D+0xa000]):q(env+i*8,v)
  q(addr,0);q(addr+8,88);w(coord,1);u.mem_write(coord+4,struct.pack('<2f',source_u,4.5));q(dst+8,9);u.mem_write(dst+0x20,bytes([3,1,source_tag]));w(desc,int(with_offset));u.mem_write(desc+4,struct.pack('<2f',*offset))
  sp=S+0xf0008;q(sp,STOP)
  for reg,value in [(UC_X86_REG_RCX,out),(UC_X86_REG_RDX,env),(UC_X86_REG_R8,42),(UC_X86_REG_R9,desc),(UC_X86_REG_RSP,sp)]:u.reg_write(reg,value)
  caches=[];panics=[];dependency_calls=[]
  try:u.emu_start(0x14127a480,STOP,count=20000)
  except Exception:
   print('FAILED',loop,source_u,offset,with_offset,source_tag,tangent,hex(r(UC_X86_REG_RIP)),dependency_calls,flush=True);raise
  if not panics:assert r(UC_X86_REG_RIP)==STOP
  rawout=bytes(u.mem_read(out,64))
  rows.append({'loop':loop,'source_u':source_u,'source_y':4.5,'offset':offset,'with_offset':with_offset,'source_tag':source_tag,'tangent':tangent,'panics':panics,'cache':caches,'output_tag':uq(out),'output64':rawout.hex(),'dependency_calls':dependency_calls})
Path(__file__).with_name('m01-split-anchor-native.json').write_text(json.dumps({'binary_sha256':sha,'entry':'0x14127a480','bridges':['PublicWalls HashMap lookup','PublicWall.wall_space field adapter returns loop,length','CRT fmodf import thunk142923f10 uses f32 math.fmod; project control flow unmodified','PublicWall attachment height returns explicit dependency output1.25','Curve position(u,y,0) and tangent supplied as dependency results; native WallSpace.project and xz_rotation_bad execute unmodified','cache map insert captures real10float transform; panic captured'],'rows':rows},indent=2)+'\n')
lines=['# loop source_u source_y offsetx offsety with_offset source_tag tangentx tangenty panic out64 cache10bits']
for row in rows:
 vals=[int(row['loop']),row['source_u'],row['source_y'],*row['offset'],int(row['with_offset']),row['source_tag'],*row['tangent'],int(bool(row['panics'])),row['output64'],','.join(map(str,row['cache'][0]['transform_bits']))if row['cache']else'-']
 lines.append('\t'.join(map(str,vals)))
Path(__file__).with_name('m01-split-anchor-native.tsv').write_text('\n'.join(lines)+'\n')
print(json.dumps({'cases':len(rows),'accepted':sum(not r['panics']for r in rows),'rejected':sum(bool(r['panics'])for r in rows)}))
