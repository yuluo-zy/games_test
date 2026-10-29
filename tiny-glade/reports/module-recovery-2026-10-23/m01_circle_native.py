"""执行B原get_tris完整函数；仅桥接allocator/memcpy/panic，保留几何和裁剪原指令。"""
from pathlib import Path
import struct, hashlib, json, random
import pefile
from unicorn import Uc, UC_ARCH_X86, UC_MODE_64, UC_HOOK_CODE
from unicorn.x86_const import *
root=Path(__file__).resolve().parents[3]
raw=(root/'B版本内容/tiny-glade.rne').read_bytes()
sha=hashlib.sha256(raw).hexdigest()
assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True); base=pe.OPTIONAL_HEADER.ImageBase
blob=pe.get_memory_mapped_image(); u=Uc(UC_ARCH_X86,UC_MODE_64)
u.mem_map(base,(len(blob)+4095)&~4095);u.mem_write(base,blob)
D,S,STOP=0x20000000,0x30000000,0x200ff000
u.mem_map(D,0x100000);u.mem_map(S,0x100000)
def q(a,x):u.mem_write(a,struct.pack('<Q',x))
def uq(a):return struct.unpack('<Q',u.mem_read(a,8))[0]
def fs(a,x):u.mem_write(a,struct.pack('<'+'f'*len(x),*x))
def ws(a,n):return list(struct.unpack('<'+'I'*n,u.mem_read(a,n*4)))
def alloc(n):
 global heap
 p=heap;heap=(heap+n+31)&~31;assert heap<D+0xf0000;return p

def ret(value=None):
 sp=u.reg_read(UC_X86_REG_RSP);u.reg_write(UC_X86_REG_RIP,uq(sp));u.reg_write(UC_X86_REG_RSP,sp+8)
 if value is not None:u.reg_write(UC_X86_REG_RAX,value)
def hook(uc,a,size,_):
 global panic
 r=uc.reg_read
 if a==0x140613c10:ret(alloc(r(UC_X86_REG_RCX)));return
 if a==0x140613c20:ret();return
 if a==0x142923c00:
  dst,src,n=r(UC_X86_REG_RCX),r(UC_X86_REG_RDX),r(UC_X86_REG_R8)
  uc.mem_write(dst,bytes(uc.mem_read(src,n)));ret(dst);return
 if a in (0x1428d9310,0x1428d9390,0x1428d9430,0x1428d9760,0x140d8b480):panic=hex(a);uc.emu_stop()
u.hook_add(UC_HOOK_CODE,hook)
def run(case):
 global heap,panic
 heap=D+0x20000;panic=None;u.mem_write(D,bytes(0x10000))
 out,vol,quad,params=D,D+0x100,D+0x200,D+0x300
 q(vol+8,quad);q(vol+16,1);u.mem_write(vol+0x60,bytes([case['special']]))
 fs(quad,[v for p in case['uv'] for v in p]);fs(quad+32,[v for p in case['points']for v in p])
 fs(quad+80,case['us']);fs(quad+88,[v for p in case['normals']for v in p]);fs(quad+112,case['heights'])
 if case['range'] is not None:u.mem_write(params,struct.pack('<Iff',1,*case['range']))
 if case['offset'] is not None:u.mem_write(params+12,struct.pack('<If',1,case['offset']))
 fs(params+20,[case['minimum'],case['top']]);u.mem_write(params+28,bytes([case['apply']]))
 sp=S+0xf0008;q(sp,STOP)
 for reg,value in [(UC_X86_REG_RCX,out),(UC_X86_REG_RDX,vol),(UC_X86_REG_R8,params),(UC_X86_REG_RSP,sp)]:u.reg_write(reg,value)
 try:u.emu_start(0x1408e80f0,STOP,count=100000)
 except Exception:print('failure rip',hex(u.reg_read(UC_X86_REG_RIP)),case);raise
 assert panic or u.reg_read(UC_X86_REG_RIP)==STOP
 return {'input':case,'panic':panic,'triangles':None if panic else [ws(uq(out+8)+i*36,9)for i in range(uq(out+16))]}
rng=random.Random(0xC1AC1E);cases=[]
for special in [0,1]:
 for apply in [False,True]:
  for urange in [None,[0.,1.],[.25,.75],[0.,0.],[1.,0.],[-.1,1.1]]:
   for offset in [None,0.,.2875,-.5]:
    for heights in [[0.,0.],[2.,3.],[-2.,1.]]:
     cases.append({'special':special,'apply':apply,'range':urange,'offset':offset,'minimum':.1,'top':-.5,'uv':[[0.,0.],[8.,0.],[0.,4.],[8.,5.]],'points':[[0.,0.,0.],[8.,1.,2.],[0.,4.,0.],[8.,5.,2.]],'us':[0.,1.],'normals':[[1.,0.,0.],[.8,.1,.6]],'heights':heights})
rows=[run(c)for c in cases]
report={'binary_sha256':sha,'entry':'0x1408e80f0','bridges':['allocator','deallocator','memcpy','panic termination'],'rows':rows}
Path(__file__).with_name('m01-circle-trigger-native.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps({'cases':len(rows),'panic':sum(bool(r['panic'])for r in rows)}))

# 原leftmost projection + 1.0/0.3偏移 + xz_rotation_bad，一直执行到真实盒构造边界。
geometry_mode=False
geometry_capture=None
def geometry_hook(uc,a,size,_):
 global geometry_capture
 if geometry_mode and a==0x140ada0e0:
  sp=uc.reg_read(UC_X86_REG_RSP); transform=uq(sp+0x28)
  geometry_capture={'dimensions':[uc.reg_read(reg)&0xffffffff for reg in [UC_X86_REG_XMM1,UC_X86_REG_XMM2,UC_X86_REG_XMM3]],'transform':ws(transform,10)}
  uc.emu_stop()
u.hook_add(UC_HOOK_CODE,geometry_hook)
def gizmo_case(case):
 global geometry_mode,geometry_capture,panic,heap
 geometry_mode=True;geometry_capture=None;panic=None;heap=D+0x20000
 u.mem_write(D,bytes(0x10000));u.mem_write(S,bytes(0x10000))
 frame=S+0x8000;public_end=D+0x2a8;camera=D+0x1000;window=D+0x2000;points=D+0x3000
 q(public_end-0x260,points);fs(points,[v for p in case['points']for v in p]);fs(camera,case['projection']);fs(camera+0x80,case['view']);q(window,case['width'])
 fs(frame+0x1b0,case['center']);q(frame+0x368,D+0x4000)
 sp=frame-0x200
 for reg,value in [(UC_X86_REG_RCX,public_end),(UC_X86_REG_RDX,len(case['points'])),(UC_X86_REG_RSI,camera),(UC_X86_REG_RDI,window),(UC_X86_REG_RBP,frame),(UC_X86_REG_RSP,sp)]:u.reg_write(reg,value)
 try:u.emu_start(0x1409acc74,0x1409acf01,count=30000)
 except Exception:print('gizmo rip',hex(u.reg_read(UC_X86_REG_RIP)),case);raise
 assert geometry_capture is not None and panic is None
 geometry_mode=False
 return {'input':case,**geometry_capture}
identity=[1.,0.,0.,0.,0.,1.,0.,0.,0.,0.,1.,0.,0.,0.,0.,1.]
gizmos=[]
for points in [[[2.,0.,0.],[0.,3.,2.],[-2.,1.,0.],[0.,2.,-2.]],[[5.,1.,3.],[2.,4.,5.],[-1.,2.,3.],[2.,0.,1.]]]:
 for center in [[0.,0.],[2.,3.]]:
  for width in [1,100,1920]:
   for skew in [False,True]:
    view=identity.copy();projection=identity.copy()
    if skew:view[4]=.125;view[8]=.25;projection[12]=.3;projection[3]=.05
    gizmos.append(gizmo_case({'points':points,'center':center,'width':width,'view':view,'projection':projection}))
Path(__file__).with_name('m01-circle-rotate-gizmo-native.json').write_text(json.dumps({'binary_sha256':sha,'entry':'0x1409acc74','stop':'0x140ada0e0','bridges':['allocator','deallocator','memcpy','box_trigger_volume argument capture'],'rows':gizmos},indent=2)+'\n')
print(json.dumps({'gizmo_cases':len(gizmos)}))

# 原半径上下限和释放阈值原指令；保留NaN、无穷和正负零的位模式。
def radius_limit(bits):
 global panic
 panic=None;u.mem_write(D,bytes(0x10000));frame=S+0x8000
 q(frame+0x150,D+0x1000);q(D+0x1028,D+0x2000)
 for reg,value in [(UC_X86_REG_RBP,frame),(UC_X86_REG_RSP,frame-0x200),(UC_X86_REG_RSI,0),(UC_X86_REG_RDI,0),(UC_X86_REG_XMM6,bits)]:u.reg_write(reg,value)
 u.emu_start(0x14165e6b8,0x14165e732,count=200)
 assert u.reg_read(UC_X86_REG_RIP)==0x14165e732 and panic is None
 return {'input':bits,'result':u.reg_read(UC_X86_REG_XMM1)&0xffffffff}
def history_branch(before,after):
 global panic
 panic=None;u.mem_write(D,bytes(0x10000))
 u.mem_write(D+0x10c,struct.pack('<I',before));u.mem_write(D+0x200-0x3c,struct.pack('<I',after))
 u.reg_write(UC_X86_REG_RDI,D+0x100);u.reg_write(UC_X86_REG_RBX,D+0x200)
 u.emu_start(0x14165e7eb,0x14165e809,count=5)
 pc=u.reg_read(UC_X86_REG_RIP)
 assert pc in (0x14165ebb3,0x14165e809) and panic is None
 return {'before':before,'after':after,'record':pc==0x14165e809}
limits=[radius_limit(bits)for bits in [0xff800000,0xc0a00000,0x80000000,0,0x3f666666,0x3f666667,0x3f800000,0x41000000,0x41100000,0x7f800000,0x7fc01234,0xffc05678,0x7f801234]]
releases=[history_branch(a,b)for a in [0,0x80000000,0x3f800000,0x7f800000,0x7fc01234]for b in [0,0x3c23d709,0x3c23d70a,0x3c23d70b,0x3f800000,0x7fc05678]]
Path(__file__).with_name('m01-circle-radius-policy-native.json').write_text(json.dumps({'binary_sha256':sha,'limit_entry':'0x14165e6b8','limit_stop':'0x14165e732','release_entry':'0x14165e7eb','limit_rows':limits,'release_rows':releases},indent=2)+'\n')
print(json.dumps({'radius_limit_cases':len(limits),'radius_release_cases':len(releases)}))

def float_input_bits(value):
 if isinstance(value,float):return struct.unpack('<I',struct.pack('<f',value))[0]
 if isinstance(value,list):return [float_input_bits(x)for x in value]
 if isinstance(value,dict):return {k:float_input_bits(v)for k,v in value.items()}
 return value
edge_rows=[]
for special in [0,1]:
 for offset in [None,0.,.2875]:
  for normals in [[[1.,0.,0.],[.8,.1,.6]],[[float('nan'),float('inf'),-0.],[0.,float('nan'),1.]]]:
   for heights in [[0.,0.],[float('nan'),1.]]:
    c={'special':special,'apply':True,'range':None,'offset':offset,'minimum':.1,'top':-.5,'uv':[[0.,0.],[8.,0.],[0.,4.],[8.,5.]],'points':[[0.,0.,0.],[8.,1.,2.],[0.,4.,0.],[8.,5.,2.]],'us':[0.,1.],'normals':normals,'heights':heights}
    row=run(c);row['input']=float_input_bits(c);edge_rows.append(row)
Path(__file__).with_name('m01-circle-trigger-edge-native.json').write_text(json.dumps({'binary_sha256':sha,'input_floats':'u32 IEEE754 bits','entry':'0x1408e80f0','rows':edge_rows},indent=2)+'\n')
print(json.dumps({'trigger_edge_cases':len(edge_rows)}))
