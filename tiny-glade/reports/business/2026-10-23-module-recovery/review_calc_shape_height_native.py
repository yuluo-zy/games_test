"""完整原calc/几何/地形业务链；只桥接allocator、内存拷贝及外部CRT数学依赖。"""
from pathlib import Path
import ctypes,hashlib,json,math,random,struct,pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE,UC_HOOK_MEM_INVALID
from unicorn.x86_const import *
root=Path(__file__).resolve().parents[4];raw=(root/'B版本内容/tiny-glade.rne').read_bytes();digest=hashlib.sha256(raw).hexdigest()
assert digest=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);pe.parse_data_directories(directories=[pefile.DIRECTORY_ENTRY['IMAGE_DIRECTORY_ENTRY_IMPORT']]);base=pe.OPTIONAL_HEADER.ImageBase;image=pe.get_memory_mapped_image()
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(image)+4095)&~4095);u.mem_write(base,image)
data,stack,terrain,heap=0x20000000,0x30000000,0x40000000,0x50000000
for address,size in [(data,0x10000),(stack,0x20000),(terrain,0x100000),(heap,0x1000000)]:u.mem_map(address,size)
entry=stack+0x18008;sentinel=data+0xff00;u.mem_write(sentinel,b'\xcc');top=heap;called=set();observed={}
lib=ctypes.CDLL(None)
math_imports={i.address:i.name.decode() for d in pe.DIRECTORY_ENTRY_IMPORT for i in d.imports if b'crt-math' in d.dll and i.name}
math_stubs={}
for address in range(0x142923000,0x142924000,16):
 code=image[address-base:address-base+6]
 if code[:2]==b'\xff\x25':
  target=address+6+struct.unpack('<i',code[2:])[0]
  if target in math_imports:math_stubs[address]=math_imports[target]
def q(address):return struct.unpack('<Q',u.mem_read(address,8))[0]
def putq(address,value):u.mem_write(address,struct.pack('<Q',value))
def bits(value):return struct.unpack('<I',struct.pack('<f',value))[0]
def floats(values):return struct.pack('<'+'f'*len(values),*values)
def words(address,count):return list(struct.unpack('<'+'I'*count,u.mem_read(address,count*4)))
def ret(value=None):
 if value is not None:u.reg_write(UC_X86_REG_RAX,value)
 sp=u.reg_read(UC_X86_REG_RSP);u.reg_write(UC_X86_REG_RIP,q(sp));u.reg_write(UC_X86_REG_RSP,sp+8)
def alloc(size,alignment):
 global top
 out=(top+alignment-1)&~(alignment-1);top=out+max(size,1);assert top<heap+0x1000000;return out
def hook(machine,address,size,opaque):
 if address==sentinel:machine.emu_stop();return
 if address==0x140ac8189:
  frame=machine.reg_read(UC_X86_REG_RBP);count=q(frame-0x48);pointer=q(frame-0x50);observed['points']=[words(pointer+i*12,3) for i in range(count)];observed['parameters']=words(q(frame-0x38),q(frame-0x30));observed['length']=words(frame-0x28,1)[0]
 if address in [0x140613c10,0x140613c20,0x140613c30,0x140613c40]:
  called.add(hex(address));rcx=machine.reg_read(UC_X86_REG_RCX);rdx=machine.reg_read(UC_X86_REG_RDX)
  if address==0x140613c20:ret();return
  if address==0x140613c30:
   new_size=machine.reg_read(UC_X86_REG_R9);out=alloc(new_size,machine.reg_read(UC_X86_REG_R8));u.mem_write(out,bytes(u.mem_read(rcx,min(rdx,new_size))));ret(out);return
  out=alloc(rcx,rdx)
  if address==0x140613c40:u.mem_write(out,bytes(rcx))
  ret(out);return
 if address==0x142923c00:
  called.add('memcpy');out=machine.reg_read(UC_X86_REG_RCX);u.mem_write(out,bytes(u.mem_read(machine.reg_read(UC_X86_REG_RDX),machine.reg_read(UC_X86_REG_R8))));ret(out);return
 if address in math_stubs:
  name=math_stubs[address];called.add('Darwin libm:'+name);f=getattr(lib,name);single=name.endswith('f');ty=ctypes.c_float if single else ctypes.c_double;count=2 if name in ['fmod','fmodf','pow','powf','atan2','atan2f'] else 1
  f.argtypes=[ty]*count;f.restype=ty;fmt='<f' if single else '<d';size=4 if single else 8
  args=[struct.unpack(fmt,int(machine.reg_read(UC_X86_REG_XMM0+i)).to_bytes(16,'little')[:size])[0] for i in range(count)];value=f(*args);packed=struct.pack(fmt,value);machine.reg_write(UC_X86_REG_XMM0,int.from_bytes(packed,'little'));ret();return
def invalid(machine,access,address,size,value,opaque):
 raise RuntimeError(f'unmapped RIP={machine.reg_read(UC_X86_REG_RIP):x}, address={address:x}')
u.hook_add(UC_HOOK_CODE,hook);u.hook_add(UC_HOOK_MEM_INVALID,invalid)
rng=random.Random(0x43414c4348454947);rows=[]
for index in range(96):
 top=heap;u.mem_write(data,bytes(0x9000));u.mem_write(stack,bytes(0x20000));putq(entry,sentinel)
 shape=index%2+1;mode=6 if index<92 else 0
 center=[rng.randrange(-48,49)/3. for _ in range(2)];radius=rng.randrange(4,25)/5.;dimensions=[rng.randrange(2,40)/5. for _ in range(2)]
 # 放置与时间是原业务输入，不由预览字段伪造。
 rotation=rng.randrange(-25,26)/5.;angle=ctypes.c_float(rng.randrange(-60,61)/17.).value;axis=[ctypes.c_float(math.cos(angle)).value,ctypes.c_float(math.sin(angle)).value]
 placement=bytearray(80);surface=index%3;fixed_tag=(index//3)%2;fixed=rng.randrange(-10,61)/3.;elevation=[0.,-0.,2.,12.][(index//5)%4]
 struct.pack_into('<IfI',placement,0,fixed_tag,fixed,surface);struct.pack_into('<f',placement,0x3c,elevation)
 cache=[(index//2)%2,bits(rng.randrange(-10,41)/3.),bits(99.),bits(-99.)];delta=[0.,1/60.,.04,-.01][(index//3)%4]
 # 描述符/Res包装只用于建立原函数的已解析输入，不模拟游戏业务。
 for wrapper,owner in [(0x100,0x110),(0x200,0x210),(0x300,0x400),(0x800,0x900),(0x1000,0x1100),(0x2000,0x2100),(0x3000,0x3100)]:
  putq(data+wrapper,data+owner);putq(data+wrapper+0x10,data+wrapper+0x30);u.mem_write(data+wrapper+0x1c,struct.pack('<I',12))
 putq(data+0x100,terrain);u.mem_write(data+0x210,struct.pack('<IB',mode,shape))
 u.mem_write(data+0x410,floats(axis+center+dimensions));u.mem_write(data+0x910,floats(center+[radius,rotation]))
 u.mem_write(data+0x1100,bytes(placement));u.mem_write(data+0x2100,struct.pack('<IIII',*cache));u.mem_write(data+0x3100+0x28,floats([delta]))
 grid_seed=index*101;grid=[(x*97+z*43+grid_seed)&65535 for z in range(512) for x in range(512)]
 u.mem_write(terrain+0x1000,struct.pack('<'+'H'*len(grid),*grid));u.mem_write(terrain,struct.pack('<QQQ',len(grid),terrain+0x1000,len(grid)))
 minimum,maximum=(-2.5,24.) if index%5 else (3.,3.);u.mem_write(terrain+0x50,floats([minimum,maximum]));putq(0x14362c308,data+0x7f00);u.mem_write(data+0x7f00,bytes([(index//7)%2]))
 putq(entry+0x28,data+0x1000);putq(entry+0x30,data+0x2000);putq(entry+0x38,data+0x3000)
 for register,value in [(UC_X86_REG_RCX,data+0x200),(UC_X86_REG_RDX,data+0x300),(UC_X86_REG_R8,data+0x800),(UC_X86_REG_R9,data+0x100),(UC_X86_REG_RSP,entry)]:u.reg_write(register,value)
 observed.clear()
 try:u.emu_start(0x140ac80e0,sentinel,count=2000000)
 except Exception:
  print('failed_case',index,'RIP',hex(u.reg_read(UC_X86_REG_RIP)),'RBP',hex(u.reg_read(UC_X86_REG_RBP)),'RCX',hex(u.reg_read(UC_X86_REG_RCX)),'RDX',hex(u.reg_read(UC_X86_REG_RDX)),'bridges',called,flush=True);raise
 assert u.reg_read(UC_X86_REG_RIP)==sentinel
 rows.append({'mode':mode,'shape':shape,'center':[bits(v) for v in center],'radius':bits(radius),'dimensions':[bits(v) for v in dimensions],'axis':[bits(v) for v in axis],'rotation':bits(rotation),'placement':list(placement),'cache':cache,'delta':bits(delta),'grid_seed':grid_seed,'centered':bool((index//7)%2),'terrain_range':[bits(minimum),bits(maximum)],'points':observed.get('points',[]),'parameters':observed.get('parameters',[]),'length':observed.get('length',0),'height':words(data+0x1100+0x40,1)[0],'output_cache':words(data+0x2100,4)})
p=root/'tiny-glade/tests/fixtures/native/toolbar-calc-shape-height.json';p.write_text(json.dumps(rows,separators=(',',':'))+'\n')
report={'binary_sha256':digest,'cases':len(rows),'entry':'0x140ac80e0','complete_original_business':True,'bridges':sorted(called),'limits':'allocator/memcpy模拟宿主环境；外部UCRT数学由Darwin libm同名函数桥接，不能声称UCRT实现跨平台逐位等价','fixture_sha256':hashlib.sha256(p.read_bytes()).hexdigest()}
Path(__file__).with_name('toolbar-calc-shape-height-native.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n');print(json.dumps(report,ensure_ascii=False))
