"""B 三个完整投影函数 oracle。仅观察原 clamp panic 边界；不桥接数学调用。"""
from pathlib import Path
import struct,hashlib,json,random,pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
root=Path(__file__).resolve().parents[4]; raw=(root/'B版本内容/tiny-glade.rne').read_bytes();digest=hashlib.sha256(raw).hexdigest()
assert digest=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
p=pefile.PE(data=raw,fast_load=True);base=p.OPTIONAL_HEADER.ImageBase;blob=p.get_memory_mapped_image();u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(blob)+4095)&~4095);u.mem_write(base,blob)
data,stack,stop=0x20000000,0x30000000,0x2000f000;u.mem_map(data,0x10000);u.mem_map(stack,0x10000)
panic=0x140d8b480
u.hook_add(UC_HOOK_CODE,lambda uc,addr,size,ud:uc.emu_stop() if addr==panic else None)
def bits(x):return struct.unpack('<I',struct.pack('<f',x))[0]
def pack(xs):return struct.pack('<'+'f'*len(xs),*xs)
def words(addr,n):return list(struct.unpack('<'+'I'*n,u.mem_read(addr,n*4)))
def run(addr):
 rsp=stack+0xf008;u.mem_write(rsp,struct.pack('<Q',stop));u.reg_write(UC_X86_REG_RSP,rsp);u.emu_start(addr,stop,count=10000);rip=u.reg_read(UC_X86_REG_RIP);assert rip in [stop,panic],hex(rip);return rip==panic
windows=[[0,0],[0,100],[100,0],[1,1],[200,100],[1920,1080],[4294967295,16777217],[16777217,3]]
window_rows=[];transform_rows=[]
for window in windows:
 u.mem_write(data+0x100,struct.pack('<II',*window));u.reg_write(UC_X86_REG_RCX,data);u.reg_write(UC_X86_REG_RDX,data+0x100);assert not run(0x140ac8420);native_screen=words(data,4);window_rows.append((window,native_screen))
 for cursor in [[0.,0.],[.5,.5],[50.,50.],[100.,25.],[-1.,2.],[float('inf'),float('-inf')],[float('nan'),1.]]:
  u.reg_write(UC_X86_REG_RCX,data);u.reg_write(UC_X86_REG_XMM1,bits(cursor[0]));u.reg_write(UC_X86_REG_XMM2,bits(cursor[1]));assert not run(0x140ac8450)
  transform_rows.append((native_screen,list(map(bits,cursor)),[u.reg_read(UC_X86_REG_XMM0)&0xffffffff,u.reg_read(UC_X86_REG_XMM1)&0xffffffff]))
id=[1.,0.,0.,0.,0.,1.,0.,0.,0.,0.,1.,0.,0.,0.,0.,1.]
project_rows=[]
def project(screen,coord,view,projection,origin,direction,limits):
 u.mem_write(data+0x100,pack(screen));u.mem_write(data+0x200,pack(projection)+b'\x00'*64+pack(view));u.mem_write(data+0x400,pack(origin+direction));u.mem_write(data+0x500,pack(limits));u.mem_write(data,b'\xa5'*16)
 u.reg_write(UC_X86_REG_RCX,data);u.reg_write(UC_X86_REG_R9,data+0x100);u.reg_write(UC_X86_REG_XMM1,bits(coord[0]));u.reg_write(UC_X86_REG_XMM2,bits(coord[1]));rsp=stack+0xf008;u.mem_write(rsp+0x28,struct.pack('<QQQ',data+0x200,data+0x400,data+0x500));panicked=run(0x140ac84a0)
 project_rows.append(tuple(list(map(bits,x)) for x in [screen,coord,view,projection,origin,direction,limits])+(panicked,words(data,3)))
for w in [1.,-1.,0.,1e-12,1e12]:
 for d in [[1.,0.,0.],[0.,1.,0.],[0.,0.,0.],[1e-12,0.,0.],[2.,0.,1.]]:
  for coord in [[0.,0.],[.5,.5],[.2,0.],[float('nan'),0.],[float('inf'),1.]]:
   proj=id.copy();proj[15]=w;proj[11]=1.
   for limits in [[0.,1.],[-1e10,1e10],[-float('inf'),float('inf')]]:project([100.,100.,1.,1.],coord,id,proj,[0.,0.,0.],d,limits)
for limits in [[1.,0.],[float('nan'),1.],[0.,float('nan')]]:project([100.,100.,1.,1.],[0.,0.],id,id,[0.,0.,0.],[1.,0.,0.],limits)
rng=random.Random(0x140ac84a0)
for _ in range(160):
 view=[rng.uniform(-4,4) for _ in range(16)];projection=[rng.uniform(-4,4) for _ in range(16)]
 project([300.,200.,1.5,2.],[rng.uniform(-2,2),rng.uniform(-2,2)],view,projection,[rng.uniform(-3,3) for _ in range(3)],[rng.uniform(-3,3) for _ in range(3)],[-10.,10.])
def ron(v,depth=0):
 if isinstance(v,bool):return str(v).lower()
 if isinstance(v,(tuple,list)):
  left,right=('(',')') if depth>0 or isinstance(v,tuple) else ('[',']')
  return left+','.join(ron(x,depth+1) for x in v)+right
 return str(v)
fixtures={}
for name,rows in [('window',window_rows),('transform',transform_rows),('ray',project_rows)]:
 path=root/f'tiny-glade/tests/fixtures/native/toolbar-raise-projection-{name}.ron';path.write_text(ron(rows)+'\n');fixtures[name]={'cases':len(rows),'sha256':hashlib.sha256(path.read_bytes()).hexdigest()}
Path(__file__).with_name('toolbar-raise-projection-native.json').write_text(json.dumps({'binary_sha256':digest,'functions':{'0x140ac8420':44,'0x140ac8450':70,'0x140ac84a0':722},'whole_functions':True,'callees_executed':['0x140c906d0','0x140c904d0'],'bridges':[],'panic_observation':'只在原clamp panic入口0x140d8b480观察并终止，不执行panic runtime；合法范围全函数返回。','fixtures':fixtures,'scope':'有限浮点输入，包含零窗口/负w/零w/退化/非仿射矩阵/NaN/Inf/range panic。ARM与x86的NaN符号/载荷按NaN分类比较；所有非NaN结果要求逐bit。'},ensure_ascii=False,indent=2)+'\n')
print(fixtures)
