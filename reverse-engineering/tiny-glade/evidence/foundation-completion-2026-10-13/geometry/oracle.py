"""B original-instruction geometry comparisons; no full game launched."""
from pathlib import Path
import sys,json,struct,math,random,hashlib
OUT=Path(__file__).resolve().parent
ROOT=OUT.parents[2]
sys.path.insert(0,str(ROOT/'evidence/shared-foundation-2026-10-13/geometry-curves'))
import boundary_oracle as b
import native_oracle as n
from native_oracle import *
OUT=Path(__file__).resolve().parent
ROOT=OUT.parents[2]
def blob(a,values):cpu.mem_write(a,struct.pack('<'+'f'*len(values),*values))
def enc(values):return [bits(v)for v in values]
def rust(x):return json.dumps(x,separators=(',',':'))
def transform(value):
 setup();blob(DATA,value);arguments(DATA+0x300,DATA);call(0x1407a4700);forward=floats(DATA+0x300,3)
 arguments(DATA+0x400,DATA);call(0x1407a45b0);raw=floats(DATA+0x400,16)
 return {'forward':forward,'affine':[[raw[i+j]for j in range(3)]for i in range(0,16,4)]}
def circle_curve(value,step,height=None,start=None,end=None):
 setup();blob(DATA,value);arguments(DATA+0x300,DATA);cpu.reg_write(UC_X86_REG_XMM2,bits(step))
 if start is not None:
  cpu.reg_write(UC_X86_REG_XMM3,bits(start));cpu.mem_write(STACK+0x100008+0x28,struct.pack('<f',end));address=0x140c97f50
 elif height is not None:cpu.reg_write(UC_X86_REG_XMM3,bits(height));address=0x140c97e10
 else:address=0x140c97cf0
 call(address);p=u64(DATA+0x308);count=u64(DATA+0x310);dim=3 if height is not None else 2
 vals=floats(p,count*dim)
 return {'points':[vals[i:i+dim]for i in range(0,len(vals),dim)],'u':floats(u64(DATA+0x320),count),'length':floats(DATA+0x330,1)[0]}
def circle_lerp(a,other,t):
 setup();blob(DATA,a);blob(DATA+0x100,other);arguments(DATA+0x300,DATA,DATA+0x100);cpu.reg_write(UC_X86_REG_XMM3,bits(t));call(0x140c98170);return floats(DATA+0x300,4)
def rect_distance(rect,p):
 setup();blob(DATA,rect);result={}
 for name,address in [('distance',0x140c9a750),('signed',0x140c9a820),('closest_normal',0x140c9a8f0),('edge_id',0x140c9acb0)]:
  arguments(DATA);cpu.reg_write(UC_X86_REG_XMM1,bits(p[0]));cpu.reg_write(UC_X86_REG_XMM2,bits(p[1]));call(address)
  result[name]=int(cpu.reg_read(UC_X86_REG_RAX)&0xffffffff)if name=='edge_id'else [cpu.reg_read(UC_X86_REG_XMM0)&0xffffffff,cpu.reg_read(UC_X86_REG_XMM1)&0xffffffff]if name=='closest_normal'else cpu.reg_read(UC_X86_REG_XMM0)&0xffffffff
 return result
def rect_slice(rect,start,end):
 setup();blob(DATA,rect);arguments(DATA+0x300,DATA);cpu.reg_write(UC_X86_REG_XMM2,bits(start));cpu.reg_write(UC_X86_REG_XMM3,bits(end));call(0x140c9bd90);return floats(u64(DATA+0x308),u64(DATA+0x310))
def rect_normal(rect,u,unnormal=False):
 setup();blob(DATA,rect);arguments(DATA);cpu.reg_write(UC_X86_REG_XMM1,bits(u));call(0x140c99e20 if unnormal else 0x140c99e10);return [cpu.reg_read(UC_X86_REG_XMM0)&0xffffffff,cpu.reg_read(UC_X86_REG_XMM1)&0xffffffff]
def rect_expand(rect,edge,amount,limits):
 setup();blob(DATA,rect);arguments(DATA,edge);cpu.reg_write(UC_X86_REG_XMM2,bits(amount));cpu.reg_write(UC_X86_REG_XMM3,bits(limits[0]));cpu.mem_write(STACK+0x100008+0x28,struct.pack('<f',limits[1]));call(0x140c9c620);return floats(DATA,6)
def rect_rotate(rect,m):
 setup();blob(DATA,rect);blob(DATA+0x100,m);arguments(DATA,DATA+0x100);call(0x140c984f0);return floats(DATA,8)
def rect_curve3(rect,height):
 setup();blob(DATA,rect);arguments(DATA+0x300,DATA);cpu.reg_write(UC_X86_REG_XMM2,bits(height));call(0x140c991f0)
 count=u64(DATA+0x310);vals=floats(u64(DATA+0x308),count*3)
 return {'points':[vals[i:i+3]for i in range(0,len(vals),3)],'u':floats(u64(DATA+0x320),count),'length':floats(DATA+0x330,1)[0]}
def rect_transform(rect):
 setup();blob(DATA,rect);arguments(DATA+0x300,DATA);call(0x140c995c0);raw=floats(DATA+0x300,10)
 return raw[:6]+raw[8:10]
def rect_edge_u(rect,u):
 setup();blob(DATA,rect);arguments(DATA);cpu.reg_write(UC_X86_REG_XMM1,bits(u));call(0x140c9acc0);return int(cpu.reg_read(UC_X86_REG_RAX)&0xffffffff)
uv_result=None
def uv_hook(c,pc,size,user):
 global uv_result
 if pc==0x1416f3598:
  uv_result={'index':int(c.reg_read(UC_X86_REG_RAX)), 'weights':[c.reg_read(r)&0xffffffff for r in (UC_X86_REG_XMM5,UC_X86_REG_XMM4,UC_X86_REG_XMM3)]};c.emu_stop()
 elif pc==0x1416f35f1:uv_result=None;c.emu_stop()
cpu.hook_add(UC_HOOK_CODE,uv_hook)
def uv_coords(triangles,p):
 global uv_result
 setup();uv_result=None;blob(DATA+0x1000,[v for tri in triangles for point in tri for v in point]);arguments(DATA,DATA+0x100,len(triangles));cpu.reg_write(UC_X86_REG_R9,DATA+0x1000)
 sp=STACK+0x100008;cpu.mem_write(sp+0x28,struct.pack('<Q',len(triangles)));cpu.mem_write(sp+0x30,struct.pack('<f',p[0]));cpu.mem_write(sp+0x38,struct.pack('<f',p[1]));cpu.emu_start(0x1416f34a0,STOP,count=100000)
 return uv_result
def bary2(point,triangle):
 setup();blob(DATA,[v for p in triangle for v in p]);cpu.reg_write(UC_X86_REG_R9,DATA);cpu.reg_write(UC_X86_REG_XMM0,bits(point[0])|(bits(point[1])<<32));cpu.reg_write(UC_X86_REG_XMM1,bits(1));cpu.reg_write(UC_X86_REG_R10,24)
 cpu.emu_start(0x1416f34f9,0x1416f3569,count=1000);cpu.emu_start(0x1416f3581,0x1416f358c,count=1000)
 return [cpu.reg_read(r)&0xffffffff for r in (UC_X86_REG_XMM5,UC_X86_REG_XMM4,UC_X86_REG_XMM3)]
events=[]
def receiver_hook(c,pc,size,user):
 if pc==0x1412b2540:
  sp=c.reg_read(UC_X86_REG_RSP)
  events.append({'position':[c.reg_read(UC_X86_REG_XMM1)&0xffffffff,c.reg_read(UC_X86_REG_XMM2)&0xffffffff], 'normal':[c.reg_read(UC_X86_REG_XMM3)&0xffffffff,floats(sp+0x28,1)[0]],'relative_u':floats(sp+0x30,1)[0], 'tag':c.mem_read(sp+0x38,1)[0]})
  ret()
cpu.hook_add(UC_HOOK_CODE,receiver_hook)
def rect_segments(rect,start,end,pending=None,join=True):
 global events
 setup();events=[];blob(DATA,rect)
 cpu.mem_write(DATA+0x100,struct.pack('<4Q',DATA+0x200,DATA+0x201,DATA+0x300,DATA+0x220))
 cpu.mem_write(DATA+0x200,bytes([int(pending is not None),int(join)]));blob(DATA+0x220,pending or [0,0]);arguments(DATA)
 cpu.reg_write(UC_X86_REG_XMM1,bits(start));cpu.reg_write(UC_X86_REG_XMM2,bits(end));cpu.reg_write(UC_X86_REG_R9,DATA+0x100);call(0x1412b1b60)
 return {'points':events.copy(),'pending':floats(DATA+0x220,2)if cpu.mem_read(DATA+0x200,1)[0]else None,'join':bool(cpu.mem_read(DATA+0x201,1)[0])}
def aabb_inline(points,dim):
 setup();blob(DATA+0x1000,[v for p in points for v in (p if dim==3 else [p[0],0,p[1]])])
 if dim==2:
  cpu.mem_write(DATA+0x2000-0x260,struct.pack('<Q',DATA+0x1000));cpu.reg_write(UC_X86_REG_RCX,DATA+0x2000);cpu.reg_write(UC_X86_REG_RDX,len(points));cpu.reg_write(UC_X86_REG_RBP,DATA+0x4000)
  cpu.emu_start(0x1409e1170,0x1409e12c3,count=10000);return floats(DATA+0x4350,4)
 cpu.mem_write(DATA+0x2000-0x10,struct.pack('<Q',DATA+0x1000));cpu.reg_write(UC_X86_REG_RCX,DATA+0x2000);cpu.reg_write(UC_X86_REG_RAX,len(points));cpu.reg_write(UC_X86_REG_RSI,DATA+0x3000)
 cpu.emu_start(0x140a07972,0x140a07a1f,count=10000);return floats(DATA+0x3000,6)
def aabb_segment(value,start,end):
 setup();arguments(DATA)
 values=[*value,*start,*end]
 for register,v in zip([UC_X86_REG_XMM1,UC_X86_REG_XMM2,UC_X86_REG_XMM3],values[:3]):cpu.reg_write(register,bits(v))
 for i,v in enumerate(values[3:]):cpu.mem_write(STACK+0x100008+0x28+i*8,struct.pack('<f',v))
 call(0x141240b30);return floats(DATA+4,2)if u64(DATA)&0xffffffff else None
def rect_intersects(value,other,circle=False):
 setup();blob(DATA,value);blob(DATA+0x100,other);arguments(DATA,DATA+0x100);call(0x140c9b6e0 if circle else 0x140c9b990);return bool(cpu.reg_read(UC_X86_REG_RAX)&255)
def ws(distance,scale,height=None):
 setup();blob(DATA+0x14,[scale]);cpu.reg_write(UC_X86_REG_XMM0,bits(distance));cpu.reg_write(UC_X86_REG_RDX,DATA)
 if height is not None:cpu.reg_write(UC_X86_REG_XMM2,bits(height))
 call(0x14127c970 if height is None else 0x14127c950);return cpu.reg_read(UC_X86_REG_XMM0)&0xffffffff
def ray(mouse,viewport,inv_proj,inv_view):
 setup();blob(DATA+0x100+0x40,inv_proj);blob(DATA+0x100+0xc0,inv_view);arguments(DATA)
 cpu.reg_write(UC_X86_REG_XMM1,bits(mouse[0]));cpu.reg_write(UC_X86_REG_XMM2,bits(mouse[1]));cpu.reg_write(UC_X86_REG_R9,DATA+0x100)
 cpu.mem_write(STACK+0x100008+0x28,struct.pack('<f',viewport[0]));cpu.mem_write(STACK+0x100008+0x30,struct.pack('<f',viewport[1]));call(0x14127cab0);return floats(DATA,6)
if __name__=='__main__':
 diagnostics={'transform_identity':transform([0,0,0,1,2,3,4,1,1,1]),'circle':circle_curve([2,3,1,0],2),'circle_range':circle_curve([2,3,1,0],2,start=.1,end=.6),'circle3':circle_curve([2,3,1,0],2,height=4),'circle_lerp':circle_lerp([0,0,1,6],[2,3,4,.2],.5),'rect_rotated_dist':rect_distance([.6,.8,0,0,2,4],[2,1]),'slice':rect_slice([1,0,0,0,2,4],.8,.2),'normal':rect_normal([1,0,0,0,2,4],0),'unnormal':rect_normal([1,0,0,0,2,4],0,True),'expand':rect_expand([1,0,0,0,2,4],0,2,[0,10]),'ws':ws(10,2,100),'ray_identity':ray([400,300],[800,600],[1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1],[1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1])}
 (OUT/'diagnostics.json').write_text(json.dumps(diagnostics,indent=2)+'\n',encoding='utf-8');print(json.dumps(diagnostics))
