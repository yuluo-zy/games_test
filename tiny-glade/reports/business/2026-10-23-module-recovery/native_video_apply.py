"""完整执行原needs_apply三位mask；只有memcmp依赖桥，不替换业务比较。"""
from pathlib import Path
import hashlib,json,struct,random,pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
root=Path(__file__).resolve().parents[4];raw=(root/'B版本内容/tiny-glade.rne').read_bytes();sha=hashlib.sha256(raw).hexdigest();assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
p=pefile.PE(data=raw,fast_load=True);base=p.OPTIONAL_HEADER.ImageBase;image=p.get_memory_mapped_image();u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(image)+4095)&~4095);u.mem_write(base,image)
data,stack,stop=0x20000000,0x30000000,0x20003000;u.mem_map(data,0x4000);u.mem_map(stack,0x10000)
u.mem_write(data+0x2000,b'AB');u.mem_write(0x142923bf0,b'\xc3')
def memcmp(cpu,address,size,_):
 a,b,n=[cpu.reg_read(r) for r in [UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8]];x=bytes(cpu.mem_read(a,n));y=bytes(cpu.mem_read(b,n));cpu.reg_write(UC_X86_REG_RAX,0 if x==y else 1)
u.hook_add(UC_HOOK_CODE,memcmp,begin=0x142923bf0,end=0x142923bf0)
def encode(values):
 vs,mon,x,y,w,h,full,exclusive,mode,rw,rh,refresh,depth=values;b=bytearray(200)
 struct.pack_into('<Q',b,0x38,0x8000000000000001 if mon==0 else 0x8000000000000000 if mon==1 else 1)
 if mon>=2:struct.pack_into('<QQ',b,0x40,data+0x2000+mon-2,1)
 struct.pack_into('<4I',b,0x50,x,y,w,h);struct.pack_into('<I3IH',b,0x68,mode,rw,rh,refresh,depth);b[0x7c]=full;b[0x7d]=exclusive;b[0xa8]=vs;return bytes(b)
basecase=(1,0,0,0,1920,1080,1,1,0,1920,1080,60000,8)
variants=[basecase]
for i in range(13):
 for value in ([0,1,2,3] if i==1 else [0,1] if i in [0,6,7,8] else [0,1,24,32] if i==12 else [0,1,24,144000]):
  row=list(basecase);row[i]=value;variants.append(tuple(row))
rows=[]
rng=random.Random(2435630)
for i in range(420):
 a,b=(basecase,rng.choice(variants)) if i<100 else (rng.choice(variants),rng.choice(variants))
 if i>=200:a=tuple(rng.choice([0,1]) if j in [0,6,7,8] else rng.randrange(4) if j==1 else rng.randrange(0,100) for j in range(13));b=tuple(rng.choice([0,1]) if j in [0,6,7,8] else rng.randrange(4) if j==1 else rng.randrange(0,100) for j in range(13))
 u.mem_write(data,encode(a));u.mem_write(data+0x100,encode(b));rsp=stack+0xf008;u.mem_write(rsp,struct.pack('<Q',stop));u.reg_write(UC_X86_REG_RSP,rsp);u.reg_write(UC_X86_REG_RCX,data);u.reg_write(UC_X86_REG_RDX,data+0x100);u.emu_start(0x140245630,stop,count=300);assert u.reg_read(UC_X86_REG_RIP)==stop
 rows.append((a,b,u.reg_read(UC_X86_REG_RAX)&255))
fixture=root/'tiny-glade/tests/fixtures/native/toolbar-video-apply.ron';fixture.write_text('[\n'+''.join(f'  ({a}, {b}, {c}),\n' for a,b,c in rows)+']\n')
Path(__file__).with_name('toolbar-video-apply-native.json').write_text(json.dumps({'binary_sha256':sha,'function':'0x140245630','cases':len(rows),'fields':['vsync','monitor variant','position x','position y','physical width','physical height','fullscreen','allow exclusive','config variant','mode width','mode height','refresh','depth'],'scope':'完整原函数；memcmp仅桥接字节相等（该调用结果只判断零）；全部合法Option与enum变体。未初始化self的unwrap另由Rust测试覆盖，不执行原panic展开。','fixture_sha256':hashlib.sha256(fixture.read_bytes()).hexdigest()},ensure_ascii=False,indent=2)+'\n');print('native cases',len(rows))
