"""执行原完整2670B切片/高度映射；通用容器和系统CRT依赖桥接明确记录。"""
from pathlib import Path
source=Path(__file__).with_name('toolbar_shape_placement_native.py').read_text().split('rows=[]\ncases=[]')[0]
exec(source,globals())
terrain_data=0x40000000;u.mem_map(terrain_data,0x100000)
for name in ['floorf','roundf','sqrtf','ceilf']:
 fn=getattr(libm,name);fn.restype=ctypes.c_float;fn.argtypes=[ctypes.c_float]
p.parse_data_directories(directories=[1])
imports={}
for entry in p.DIRECTORY_ENTRY_IMPORT:
 for imp in entry.imports:
  if imp.name:imports[imp.address]=imp.name.decode(errors='replace')
# 原 wrapper 仍执行其 jmp；IAT 写到每个探针拥有的独立 ret stub。
crt_stubs={}
for address,name in imports.items():
 if name in ['floorf','roundf','sqrtf','ceilf','memset','memmove','memcpy']:
  stub=0x200f0000+len(crt_stubs)*16;u.mem_write(stub,b'\xc3');q(address,stub);crt_stubs[stub]=name

def extra(uc,address,size,_):
 def r(reg):return uc.reg_read(reg)
 if address==0x140613c30:
  old,oldsize,newsize=r(UC_X86_REG_RCX),r(UC_X86_REG_RDX),r(UC_X86_REG_R9);new=alloc(newsize);u.mem_write(new,bytes(u.mem_read(old,min(oldsize,newsize))));ret(new);return
 if address==0x140613c40:
  new=alloc(r(UC_X86_REG_RCX));u.mem_write(new,b'\0'*r(UC_X86_REG_RCX));ret(new);return
 name=crt_stubs.get(address)
 if name:
  if name in ['floorf','roundf','sqrtf','ceilf']:
   x=struct.unpack('<f',struct.pack('<I',r(UC_X86_REG_XMM0)&0xffffffff))[0];uc.reg_write(UC_X86_REG_XMM0,bits(getattr(libm,name)(x)));ret();return
  if name=='memset':
   dst,n=r(UC_X86_REG_RCX),r(UC_X86_REG_R8);u.mem_write(dst,bytes([r(UC_X86_REG_RDX)&255])*n);ret(dst);return
  dst,src,n=r(UC_X86_REG_RCX),r(UC_X86_REG_RDX),r(UC_X86_REG_R8);u.mem_write(dst,bytes(u.mem_read(src,n)));ret(dst);return
u.hook_add(UC_HOOK_CODE,extra)

def preview(c):
 global heap,case,panic
 heap=D+0x20000;case=c;panic=None;u.mem_write(D,b'\0'*0x20000)
 state=D+0x1000;position=D+0x2000;terrain=D+0x3000;out=D+0x4000
 points=c.get('points',[[0.,0.,0.],[4.,2.,0.],[8.,0.,0.]])
 us=c.get('us',[0.,.5,1.]);curve(state,[[v[0],v[2]]for v in points],us);curve(state+0x38,points,us)
 f(state+0x68,c.get('mapped_length',(80.**.5)))
 shape=c.get('shape',2);u.mem_write(state+0x258,struct.pack('<I',shape));fs(state+0x25c,[1.,0.,4.,0.,8.,6.]if shape==1 else[4.,0.,4.,0.])
 fs(position,c.get('position',[2.,3.,1.]));q(terrain+8,terrain_data);f(terrain+0x50,-2.5);f(terrain+0x54,24.)
 terrain_values=[(x*137+z*71)&65535 for z in range(512)for x in range(512)]
 u.mem_write(terrain_data,struct.pack('<'+'H'*len(terrain_values),*terrain_values));q(0x14362c308,D+0x1f000);u.mem_write(D+0x1f000,bytes([int(c.get('centered',False))]))
 rsp=S+0xf0008;q(rsp,STOP);f(rsp+0x28,c.get('length',4.5));f(rsp+0x30,c.get('elevation',0.));f(rsp+0x38,c.get('placement_y',3.))
 for reg,value in [(UC_X86_REG_RCX,out),(UC_X86_REG_RDX,state),(UC_X86_REG_R8,position),(UC_X86_REG_R9,terrain),(UC_X86_REG_RSP,rsp)]:u.reg_write(reg,value)
 try:u.emu_start(0x1408dfc60,STOP,count=5000000)
 except Exception:print('FAILED',c,'rip',hex(u.reg_read(UC_X86_REG_RIP)));raise
 assert u.reg_read(UC_X86_REG_RIP)==STOP or panic,(c,hex(u.reg_read(UC_X86_REG_RIP)))
 if panic:return {'input':c,'panic':panic}
 if uq(out+16)==0x8000000000000000:return {'input':c,'none':True}
 n=uq(out+32);assert n<10000,n
 return {'input':c,'anchor':words(out,3),'points':[words(uq(out+24)+i*12,3)for i in range(n)]}
rows=[]
cases=[]
for shape in [0,1,2,3]:
 for position in [[-1.,3.,1.],[0.,3.,0.],[2.,3.,1.],[7.99,3.,0.],[9.,3.,3.]]:
  for elevation in [0.,1.,-1.]:
   cases.append({'shape':shape,'position':position,'elevation':elevation})
for shape in [0,1,2,3]:
 for length in [0.,-1.,.1,50.]:cases.append({'shape':shape,'length':length})
# 开放/闭合/重复参数缓存，避免仅以规则形状覆盖切片边界。
for shape in [2,3]:
 for position in [[0.,3.,0.],[4.,3.,0.],[5.,3.,3.]]:
  cases.append({'shape':shape,'points':[[0.,0.,0.],[0.,0.,0.],[4.,2.,0.],[8.,0.,0.]],'us':[0.,0.,.5,1.],'position':position})
for c in cases:rows.append(preview(c))
Path(__file__).with_name('toolbar-placement-signifier-preview-native.json').write_text(json.dumps({'binary_sha256':sha,'rows':rows},indent=2)+'\n');(root/'tiny-glade/tests/fixtures/native/toolbar-placement-signifier-preview.json').write_text(json.dumps(rows,separators=(',',':'))+'\n');print('preview rows',len(rows))

capturing_signifiers=False
factories=[]
factory_owners=[]
builder_observations=[]
def capture_factory(uc,address,size,_):
 if not capturing_signifiers:return
 if address==0x14146c880:
  record=uc.reg_read(UC_X86_REG_RDX);factory_owners.append((uq(record+24),uq(record+32)));name=bytes(u.mem_read(uq(record+8),uq(record+16))).decode();owner=uq(record+24);vtable=uq(record+32);payload=owner+16
  if vtable==0x142aa4e90:
   points=[words(uq(payload+8)+i*12,3)for i in range(uq(payload+16))];factories.append({'name':name,'kind':'curve','points':points})
  else:factories.append({'name':name,'kind':'sphere','position':words(payload,3)})
  ret();return
 if address==0x14089dd50:
  out=uc.reg_read(UC_X86_REG_RCX);builder=uc.reg_read(UC_X86_REG_RDX)
  tag=uq(builder+16);mesh={'tag':hex(tag)}
  if tag==0x8000000000000000:mesh['name']=bytes(u.mem_read(uq(builder+32),uq(builder+40))).decode()
  elif tag==0x8000000000000002:mesh['points']=[words(uq(builder+32)+i*12,3)for i in range(uq(builder+40))]
  builder_observations.append({'mesh':mesh,'rotation':words(builder,4),'animation':words(builder+64,4),'scale_tag':words(builder+80,1)[0],'scale_bits':words(builder+84,1)[0],'duration_bits':words(builder+120,1)[0],'flags':list(u.mem_read(builder+136,3)),'shader':bytes(u.mem_read(uq(builder+104),uq(builder+112))).decode(),'position':words(builder+124,3)})
  u.mem_write(out,b'\0'*40);q(out+8,D+0x9000);q(out+16,D+0x9000);q(out+32,0x100000001);ret(out);return
 if address==0x140d56700:
  u.mem_write(uc.reg_read(UC_X86_REG_RCX),b'\0'*48);ret();return # std backtrace资源
 if address==0x1428a8940:ret(alloc(32));return # anyhow格式/错误所有权，不替换失败判据
 if address==0x140d4cc30:ret();return # anyhow::Drop
u.hook_add(UC_HOOK_CODE,capture_factory)

def signifiers(c):
 global capturing_signifiers,factories,factory_owners,builder_observations,case,panic,heap
 preview({k:v for k,v in c.items()if k not in ['surface','flat','mode','tool','public','base','roof_point']})
 case=c;panic=None;factories=[];factory_owners=[];builder_observations=[];capturing_signifiers=True
 # 原公共存储布局与标准hash查找分离；正常查询仍执行原内联验证和分支。
 ctrl=D+0x10000;value=ctrl-0x2a0;u.mem_write(value,bytes(u.mem_read(D+0x1000,672)));q(value-8,42);u.mem_write(ctrl,b'\0'+b'\xff'*31)
 publicmap=D+0x5000;q(publicmap,ctrl);q(publicmap+8,0);q(publicmap+24,int(c.get('public',True)))
 args=[D+0x5100+i*0x100 for i in range(5)];mode=D+0x6000;placement=D+0x6100
 q(args[0],mode);u.mem_write(mode,struct.pack('<IB',c.get('mode',6),c.get('tool',1)))
 q(args[1],placement);u.mem_write(placement,b'\0'*80);u.mem_write(placement+8,struct.pack('<I',c.get('surface',1)));u.mem_write(placement+44,bytes([c.get('flat',0)]));q(placement+32,42)
 fs(placement+12,c.get('roof_point',[2.,3.,1.]));fs(placement+48,c.get('position',[2.,3.,1.]));f(placement+60,c.get('elevation',0.));fs(placement+68,c.get('base',[2.,3.,1.]))
 q(args[2],D+0x6200);q(args[3],publicmap);q(args[4],D+0x3000)
 rsp=S+0xf0008;q(rsp,STOP);q(rsp+0x28,args[4])
 for reg,value in zip([UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8,UC_X86_REG_R9],args):u.reg_write(reg,value)
 u.reg_write(UC_X86_REG_RSP,rsp)
 try:u.emu_start(0x1408dec70,STOP,count=5000000)
 except Exception:print('SIGNIFIER FAILED',c,'rip',hex(u.reg_read(UC_X86_REG_RIP)));raise
 assert u.reg_read(UC_X86_REG_RIP)==STOP or panic
 if not panic:
  for owner,vtable in factory_owners:
   q(D+0x9000,4096);q(D+0x9008,D+0xa000);q(D+0x9010,0);u.mem_write(D+0xa000,b'\0'*4096)
   rsp=S+0xf0008;q(rsp,STOP);q(rsp+0x28,D+0x8000)
   for reg,value in [(UC_X86_REG_RCX,D+0x7000),(UC_X86_REG_RDX,owner+16),(UC_X86_REG_R8,D+0x8100),(UC_X86_REG_R9,D+0x8200),(UC_X86_REG_RSP,rsp)]:u.reg_write(reg,value)
   try:u.emu_start(0x1408df6c0 if vtable==0x142aa4e90 else 0x1408dfa00,STOP,count=100000)
   except Exception:print('WAKE FAIL',hex(u.reg_read(UC_X86_REG_RIP)),hex(owner),hex(vtable),builder_observations);raise
   assert u.reg_read(UC_X86_REG_RIP)==STOP
   builder_observations[-1]['draw_topology']=words(D+0xa000+16,2)if uq(D+0x9010)>0 else None
 capturing_signifiers=False
 assert u.reg_read(UC_X86_REG_RIP)==STOP or panic
 return {'input':c,'factories':factories,'builders':builder_observations,'panic':panic}
factory_rows=[]
for c in [{'shape':1},{'shape':0},{'shape':2},{'shape':3},{'surface':2},{'surface':2,'base':[1.,2.,3.],'roof_point':[4.,5.,6.]},{'surface':0},{'flat':2},{'mode':5},{'tool':0},{'public':False}]:factory_rows.append(signifiers(c))
Path(__file__).with_name('toolbar-placement-signifier-factories-native.json').write_text(json.dumps({'binary_sha256':sha,'rows':factory_rows},indent=2)+'\n');(root/'tiny-glade/tests/fixtures/native/toolbar-placement-signifier-factories.json').write_text(json.dumps(factory_rows,separators=(',',':'))+'\n');print('factory rows',len(factory_rows))
