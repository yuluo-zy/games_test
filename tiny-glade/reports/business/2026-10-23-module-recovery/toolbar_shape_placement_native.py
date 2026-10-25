"""原 5171B 放置系统与1395B候选闭包；桥接仅容器/物理查询/绘制factory边界。"""
from pathlib import Path
import struct,hashlib,json,random,pefile,traceback,ctypes
libm=ctypes.CDLL(None)
for name in ["cosf","sinf","fmodf"]:
 fn=getattr(libm,name);fn.restype=ctypes.c_float;fn.argtypes=[ctypes.c_float]*(2 if name=="fmodf" else 1)
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
root=Path(__file__).resolve().parents[4]; binary=root/'B版本内容/tiny-glade.rne';raw=binary.read_bytes();sha=hashlib.sha256(raw).hexdigest();assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
p=pefile.PE(data=raw,fast_load=True);base=p.OPTIONAL_HEADER.ImageBase;blob=p.get_memory_mapped_image();u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(blob)+4095)&~4095);u.mem_write(base,blob)
D,S,STOP=0x20000000,0x30000000,0x200ff000;u.mem_map(D,0x100000);u.mem_map(S,0x100000)
def q(addr,x):u.mem_write(addr,struct.pack('<Q',x))
def uq(addr):return struct.unpack('<Q',u.mem_read(addr,8))[0]
def f(addr,x):u.mem_write(addr,struct.pack('<f',x))
def fs(addr,xs):u.mem_write(addr,struct.pack('<'+'f'*len(xs),*xs))
def bits(x):return struct.unpack('<I',struct.pack('<f',x))[0]
def words(addr,n):return list(struct.unpack('<'+'I'*n,u.mem_read(addr,n*4)))
def alloc(n):
 global heap
 a=heap;heap=(heap+n+31)&~31;assert heap<D+0xf0000;return a

def ret(value=None):
 rsp=u.reg_read(UC_X86_REG_RSP);target=uq(rsp);u.reg_write(UC_X86_REG_RSP,rsp+8);u.reg_write(UC_X86_REG_RIP,target)
 if value is not None:u.reg_write(UC_X86_REG_RAX,value)

query_mode=False
query_bounds=[]
def hook(uc,addr,size,_):
 global panic,draws
 def r(reg):return uc.reg_read(reg)
 if addr==0x1409f12e0 and query_mode:
  query_bounds.extend([r(reg)&0xffffffff for reg in [UC_X86_REG_XMM1,UC_X86_REG_XMM2,UC_X86_REG_XMM3,UC_X86_REG_XMM4]])
  dst=r(UC_X86_REG_R8);q(dst,len(segments));q(dst+8,segment_array);q(dst+16,len(segments));ret();return
 if addr in (0x142923e80,0x142923fd0,0x142923f10):
  xs=[struct.unpack('<f',struct.pack('<I',r(reg)&0xffffffff))[0]for reg in [UC_X86_REG_XMM0,UC_X86_REG_XMM1]]
  name={0x142923e80:'cosf',0x142923fd0:'sinf',0x142923f10:'fmodf'}[addr]
  result=getattr(libm,name)(*xs[:2 if name=='fmodf' else 1]);uc.reg_write(UC_X86_REG_XMM0,bits(result));ret();return
 if addr in (0x140613c10,):ret(alloc(r(UC_X86_REG_RCX)));return
 if addr==0x1409766a0:ret(alloc(24));return
 if addr==0x140613c20:ret();return
 if addr==0x142923c00:
  dst,src,n=r(UC_X86_REG_RCX),r(UC_X86_REG_RDX),r(UC_X86_REG_R8);u.mem_write(dst,bytes(u.mem_read(src,n)));ret(dst);return
 if addr==0x1405b0d70:ret(0);return # std hash_one，单桶键由测试拥有
 if addr==0x140a91ef0:ret(public if case.get('public',True) else 0);return # 权威存储查找
 if addr==0x140a922b0:ret(0x100000001 if case.get('roof',False) else 0);return
 if addr==0x1409765f0:ret(roof if case.get('roof',False) else 0);return
 if addr==0x1409767a0:
  dst=r(UC_X86_REG_RCX);u.mem_write(dst,struct.pack('<I',3));q(dst+8,roof);ret(dst);return
 if addr==0x1408cdc60:
  dst=r(UC_X86_REG_RCX);u.mem_write(dst,hitraw);ret(dst);return
 if addr==0x140aa5a10 and not query_mode:
  dst=r(UC_X86_REG_R8);q(dst,len(segments));q(dst+8,segment_array);q(dst+16,len(segments));ret();return
 if addr in (0x140b02520,0x140b04f70):
  ptr=r(UC_X86_REG_RCX)
  if addr==0x140b02520:draws.append(('sphere',words(ptr+16,5)))
  else:
   n=uq(ptr+16);draws.append(('line',words(uq(ptr+8),n*3)))
  ret();return
 if addr in (0x1428d9310,0x1428d9390,0x1428d9430,0x1428d9760,0x140d8b480):panic=hex(addr);uc.emu_stop();return
u.hook_add(UC_HOOK_CODE,hook)
def curve(addr,points,us):
 n=len(points);pt=alloc(n*len(points[0])*4);ut=alloc(n*4);fs(pt,[v for row in points for v in row]);fs(ut,us)
 for o,x in [(0,n),(8,pt),(16,n),(24,n),(32,ut),(40,n)]:q(addr+o,x)
 f(addr+48,8.)
def vector(c,key,default):
 if key+'_bits' in c:return list(struct.unpack('<3f',struct.pack('<3I',*c[key+'_bits'])))
 return c.get(key,default)
def run_case(c):
 global heap,panic,case,draws,public,roof,hitraw,segments,segment_array
 case=c;heap=D+0x20000;panic=None;draws=[];u.mem_write(D,b'\0'*0x20000)
 args=[D+i*0x100 for i in range(14)]
 mode=D+0x2000;u.mem_write(mode,struct.pack('<IB',c.get('mode',6),c.get('tool',1)));q(args[0],mode)
 cursor=D+0x2100;u.mem_write(cursor,struct.pack('<I3f',c.get('cursor_tag',1),*vector(c,'cursor',[2.,0.,1.])));q(args[1],cursor)
 # PublicWalls 单桶：key 后接 PublicWallState。
 ctrl=D+0x5000;public=ctrl-0x2a0;publicmap=D+0x3000;q(args[2],publicmap)
 q(publicmap,ctrl);q(publicmap+8,0);q(publicmap+24,1);u.mem_write(ctrl,b'\0'+b'\xff'*31);q(public-8,42)
 points=c.get('points',[[0.,0.,0.],[4.,1.,0.],[8.,0.,0.]])
 us=c.get('us',[0.,.5,1.]);source=[[v[0],v[2]]for v in points];curve(public,source,us);curve(public+0x38,points,us)
 shape=c.get('shape',1);u.mem_write(public+0x258,struct.pack('<I',shape));fs(public+0x25c,[1.,0.,4.,0.,8.,6.] if shape==1 else [4.,0.,4.,0.])
 f(public+0x280,c.get('elevation',0.));f(public+0x288,c.get('arch',2.8))
 roof=D+0x6000;rb=bytearray(88);struct.pack_into('<I',rb,8,c.get('roof_shape',1));struct.pack_into('<6f',rb,12,1.,0.,4.,0.,8.,6.);
 if c.get('roof_shape',1)==0:struct.pack_into('<4f',rb,12,4.,0.,4.,0.)
 struct.pack_into('<f',rb,0x30,c.get('roof_height',0.));struct.pack_into('<f',rb,0x34,c.get('ridge_length',.5));rb[0x3c]=c.get('ridge_dir',0);struct.pack_into('<f',rb,0x4c,c.get('floor',0.));u.mem_write(roof,bytes(rb))
 # ResMut/EventWriter 的 ticks 指针均为真实可写存储；Events 预留容量避免扩容。
 for idx in [5,6,7,11]:
  owned=D+0x7000+idx*0x200;q(args[idx],owned);q(args[idx]+16,D+0x6800+idx*4)
 placement=uq(args[6]);u.mem_write(placement,b'\xa5'*80);snap=uq(args[7]);u.mem_write(snap,bytes([c.get('snap',0)]))
 for idx in [5,11]:
  evt=uq(args[idx]);q(evt+32,100);q(evt+40,alloc(3200));q(evt+48,0);q(evt+64,0)
 mods=D+0x6700;u.mem_write(mods,struct.pack('<I',c.get('modifier',0)));q(args[8],mods)
 kind=c.get('hit_kind',-1);hitraw=struct.pack('<QQQ3f3ffI2f',max(kind,0)|(c.get('registry_flag',0)<<8),42,0,*vector(c,'hit',[2.,3.,.25]),0.,1.,0.,1.,2 if kind<0 else 0,0.,0.)
 q(args[9],D+0x6900 if kind>=0 else 0);q(args[10],D+0x6a00)
 # EventReader; frozen 的未读数量由第二缓冲区计数表达。
 q(args[12],D+0x6b00);q(args[12]+8,D+0x6c00);q(D+0x6c00+48,int(c.get('frozen',False)));q(D+0x6c00+64,int(c.get('frozen',False)))
 segments=[];segment_array=alloc(8*100)
 for i,endpoints in enumerate(c.get('segments',[[[0.,0.],[4.,0.]],[[4.,0.],[4.,4.]]]) if c.get('candidate',False) else []):
  segment=alloc(56);fs(segment,[v for row in endpoints for v in row]);q(segment+40,42);u.mem_write(segment+48,struct.pack('<I',i));segments.append(segment);q(segment_array+i*8,segment)
 rsp=S+0xf0008;q(rsp,STOP)
 for i in range(4,14):q(rsp+0x28+(i-4)*8,args[i])
 for r,a in zip([UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8,UC_X86_REG_R9],args):u.reg_write(r,a)
 u.reg_write(UC_X86_REG_RSP,rsp)
 try:u.emu_start(0x14097f970,STOP,count=2000000)
 except Exception:
  print('FAILED',c,'rip',hex(u.reg_read(UC_X86_REG_RIP)));raise
 assert u.reg_read(UC_X86_REG_RIP)==STOP or panic,(c,hex(u.reg_read(UC_X86_REG_RIP)))
 ev=uq(args[11]);sound=uq(args[5]);hint=[u.mem_read(uq(ev+40)+i*16+8,1)[0]for i in range(uq(ev+48))];audio=[words(uq(sound+40)+i*32+16,3)for i in range(uq(sound+48))]
 return {'input':c,'placement':words(placement,20),'snap':u.mem_read(snap,1)[0],'hint':hint,'audio':audio,'draws':draws,'panic':panic,'reader':uq(D+0x6b00),'roof':list(rb)}
rows=[]
cases=[]
for shape in [0,1,2]:
 for tool in [1,2]:
  for y in [-2.0,0.0,1.4999999,1.5,3.0,60.0,61.0]:
   for modifier in [0,1]:
    cases.append({'hit_kind':0,'shape':shape,'tool':tool,'hit':[2.,y,.25],'modifier':modifier})
for kind in [1,3,4,5,6,7,8,9,10,11,12,13,14,15,16]:cases.append({'hit_kind':kind,'candidate':True})
for modifier in [0,1]:
 for snap in [0,1,2]:
  for candidate in [False,True]:cases.append({'modifier':modifier,'snap':snap,'candidate':candidate})
for elevation in [-2.0,1.5,1.5000001]:
 for z in [0.,1.5,1.5000001,3.]:cases.append({'candidate':True,'elevation':elevation,'cursor':[2.,5.,z]})
for tool in [1,2]:
 for h in [0.,.1,.10000001,.5,1.]:
  for xz in [[2.,.25],[4.,0.],[10.,10.]]:
   cases.append({'hit_kind':2,'roof':True,'tool':tool,'roof_height':h,'hit':[xz[0],3.,xz[1]]})
cases.extend([{'frozen':True},{'mode':5},{'tool':0},{'candidate':True,'cursor_tag':0},{'hit_kind':0,'roof':True,'roof_height':.1},{'hit_kind':0,'roof':True,'roof_height':.2},{'hit_kind':2,'roof':True,'shape':2},{'hit_kind':2,'roof':False}])
for tool in [1,2]:
 for height in [0.,.5,1.]:
  for pos in [[4.,3.,0.],[4.,6.2,0.],[10.,3.,10.]]:
   cases.append({'hit_kind':2,'roof':True,'shape':0,'roof_shape':0,'roof_height':height,'tool':tool,'hit':pos,'registry_flag':1})
for shape in [0,2]:
 for modifier in [0,1]:cases.append({'candidate':True,'shape':shape,'modifier':modifier})
cases.append({'candidate':True,'shape':2,'segments':[[[0.,0.],[0.,0.]]]})
for value in [0x7fc12345,0x7f800000,0xff800000]:
 cases.append({'hit_kind':0,'hit_bits':[bits(2.),value,bits(.25)]})
 cases.append({'candidate':True,'cursor_bits':[value,bits(0.),bits(1.)]})
for c in cases:rows.append(run_case(c))
path=Path(__file__).with_name('toolbar-shape-placement-native-raw.json');path.write_text(json.dumps({'binary_sha256':sha,'rows':rows},ensure_ascii=False,indent=2)+'\n');(root/'tiny-glade/tests/fixtures/native/toolbar-shape-placement.json').write_text(json.dumps(rows,separators=(',',':'))+'\n');print('native rows',len(rows))

query_rows=[]
query_mode=True
rng=random.Random(0x140aa5a10)
for n in [0,1,2,3,20,21,22,50,100,511]:
 for bounds in [[-2.,-3.,5.,6.],[-100.,-100.,100.,100.],[70.,70.,80.,80.],[-80.,-80.,-70.,-70.],[10.,5.,-5.,-10.]]:
  heap=D+0x20000;segments=[];segment_array=alloc(max(n*8,8));pointers={};input_ids=[rng.randrange(max(1,n//2))for _ in range(n)]
  for i,segment_id in enumerate(input_ids):
   if segment_id not in pointers:
    ptr=alloc(56);q(ptr+40,42);u.mem_write(ptr+48,struct.pack('<I',segment_id));pointers[segment_id]=ptr
   segments.append(pointers[segment_id]);q(segment_array+i*8,pointers[segment_id])
  query_bounds=[];native_spatial=D+0x1000;f(native_spatial+0x50,130.);f(native_spatial+0x54,130.);aabb=D+0x1100;fs(aabb,bounds);out=D+0x1200
  rsp=S+0xf0008;q(rsp,STOP);u.reg_write(UC_X86_REG_RSP,rsp);u.reg_write(UC_X86_REG_RCX,native_spatial);u.reg_write(UC_X86_REG_RDX,aabb);u.reg_write(UC_X86_REG_R8,out);u.emu_start(0x140aa5a10,STOP,count=2000000)
  assert u.reg_read(UC_X86_REG_RIP)==STOP
  ids=[struct.unpack('<I',u.mem_read(uq(uq(out+8)+i*8)+48,4))[0]for i in range(uq(out+16))]
  assert ids==sorted(set(input_ids))
  query_rows.append({'input_ids':input_ids,'bounds':bounds,'clamped_bits':query_bounds,'ids':ids})
Path(__file__).with_name('toolbar-shape-placement-query-native.json').write_text(json.dumps({'binary_sha256':sha,'whole_function':'0x140aa5a10/394B','bridge':'get_cells_from_aabb only supplies test-owned collected segment refs; original bounds clipping and complete sort/dedup executed','rows':query_rows},indent=2)+'\n');print('query rows',len(query_rows))
