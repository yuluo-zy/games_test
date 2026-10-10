from pathlib import Path
import struct,json,random,subprocess,math
from unicorn import *
from unicorn.x86_const import *
R=Path('D:/game/reverse-engineering/tiny-glade');O=R/'evidence/roof-tiles'
raw=Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes();pe=struct.unpack_from('<I',raw,60)[0];op=struct.unpack_from('<H',raw,pe+20)[0]
base=struct.unpack_from('<Q',raw,pe+48)[0];size=struct.unpack_from('<I',raw,pe+24+56)[0]
ss=[struct.unpack_from('<IIII',raw,pe+24+op+40*i+8) for i in range(struct.unpack_from('<H',raw,pe+6)[0])]
cpu=Uc(UC_ARCH_X86,UC_MODE_64);cpu.mem_map(base,(size+4095)&~4095)
for z,v,n,r in ss:cpu.mem_write(base+v,raw[r:r+n])
data=0x20000000;stack=0x30000000;stop=0x40000000;cpu.mem_map(data,0x10000);cpu.mem_map(stack,0x10000);cpu.mem_map(stop,0x1000)
def xf(reg,v):cpu.reg_write(reg,int.from_bytes(struct.pack('<f',v),'little'))
def rf(reg):return struct.unpack('<f',struct.pack('<I',cpu.reg_read(reg)&0xffffffff))[0]
def hook(cpu,a,n,u):
 if a==0x142923e60:
  v=rf(UC_X86_REG_XMM0); xf(UC_X86_REG_XMM0,v if not math.isfinite(v) else float(math.ceil(v)))
  sp=cpu.reg_read(UC_X86_REG_RSP);ra=int.from_bytes(cpu.mem_read(sp,8),'little');cpu.reg_write(UC_X86_REG_RSP,sp+8);cpu.reg_write(UC_X86_REG_RIP,ra)
cpu.hook_add(UC_HOOK_CODE,hook)
runner=O/'helpers_runner.rs'
runner.write_text('''#[path="D:/game/reverse-engineering/tiny-glade/reconstruction/roof/surface.rs"] mod surface;
#[path="D:/game/reverse-engineering/tiny-glade/reconstruction/roof/tiles.rs"] mod tiles;
use std::io::{self, BufRead};
fn main(){for l in io::stdin().lock().lines(){let l=l.unwrap();let x:Vec<_>=l.split_whitespace().collect();
if x[0]=="split" {let n:usize=x[1].parse().unwrap();let amp=f32::from_bits(u32::from_str_radix(x[2],16).unwrap());let mut rng=tiles::TileRng(u64::from_str_radix(x[3],16).unwrap());let y=tiles::random_splits(n,amp,&mut rng);print!("{:016x}",rng.0);for v in y {print!(" {:08x}",v.to_bits());}println!();}
else if x[0]=="row"{let p=tiles::circular_row_plan(f32::from_bits(u32::from_str_radix(x[1],16).unwrap()));println!("{} {:08x} {:08x}",p.row_count,p.tile_height.to_bits(),p.denominator.to_bits());}
}}
''',encoding='utf-8')
exe=R/'workspace/tiles-helper-reconstructed.exe'
import sys
sys.path.insert(0,str(R/'scripts'))
from dependency_build import build_runner
build_runner(runner,exe)
rand=random.Random(20261010);cases=[];expected=[]
ampedges=[0,0x80000000,0x3f800000,0xbf800000,0x3e4ccccd,0x7f800000,0xff800000,0x7fc00000,0x7f800001]
for i in range(800):
 n=([2,3,5,16,65,128,257,1024][i%8] if i<72 else rand.randrange(2,128));amp=(ampedges[i%9] if i<72 else rand.getrandbits(32));seed=rand.getrandbits(64)
 cpu.mem_write(data,struct.pack('<QQQ',n,data+0x100,n*0));cpu.mem_write(data+0x80,struct.pack('<Q',seed));sp=stack+0xff00;cpu.mem_write(sp,struct.pack('<Q',stop));cpu.reg_write(UC_X86_REG_RSP,sp)
 cpu.reg_write(UC_X86_REG_RCX,n);cpu.reg_write(UC_X86_REG_R8,data+0x80);cpu.reg_write(UC_X86_REG_R9,data);cpu.reg_write(UC_X86_REG_XMM1,amp);cpu.reg_write(UC_X86_REG_MXCSR,0x1f80)
 cpu.emu_start(0x140c90050,stop,count=200000)
 length=struct.unpack('<Q',cpu.mem_read(data+16,8))[0]; state=struct.unpack('<Q',cpu.mem_read(data+0x80,8))[0];vals=struct.unpack('<'+'I'*length,cpu.mem_read(data+0x100,length*4))
 expected.append(f'{state:016x}'+''.join(f' {v:08x}' for v in vals));cases.append(f'split {n} {amp:08x} {seed:016x}')
lengths=[0,0x80000000,0x3f800000,0x3f600000,0x3f600001,0x3f5fffff,0xbf800000,0x7f800000,0xff800000,0x7fc00000,0x7f800001]+[struct.unpack('<I',struct.pack('<f',rand.uniform(0,1000)))[0] for _ in range(512)]
for bits in lengths:
 rbp=stack+0xfe00;cpu.reg_write(UC_X86_REG_RSP,stack+0xfd00);cpu.reg_write(UC_X86_REG_RBP,rbp);cpu.reg_write(UC_X86_REG_XMM6,bits);cpu.reg_write(UC_X86_REG_MXCSR,0x1f80)
 cpu.mem_write(rbp+0x88,struct.pack('<I',bits));cpu.emu_start(0x1421b3201,0x1421b3248,count=200)
 n=cpu.reg_read(UC_X86_REG_RCX)&0xffffffff;n=n-(1<<32) if n>>31 else n;height=cpu.reg_read(UC_X86_REG_XMM6)&0xffffffff;denom=struct.unpack('<I',cpu.mem_read(rbp+0x148,4))[0]
 # Block stops before MULSS; the later literal height multiplication is included.
 cpu.reg_write(UC_X86_REG_RDI,data);cpu.emu_start(0x1421b3248,0x1421b3260,count=20);height=cpu.reg_read(UC_X86_REG_XMM6)&0xffffffff
 expected.append(f'{n} {height:08x} {denom:08x}');cases.append(f'row {bits:08x}')
result=subprocess.run([str(exe)],input='\n'.join(cases)+'\n',capture_output=True,text=True,check=True).stdout.splitlines()
fail=[dict(index=i,input=cases[i],original=a,rust=b) for i,(a,b) in enumerate(zip(expected,result)) if a!=b]
import hashlib
report=dict(ok=len(result)==len(expected) and not fail,source_sha256=hashlib.sha256(raw).hexdigest(),split_cases=800,row_cases=len(lengths),failures=fail,oracle='Original RNE instructions; full random_splits_into and original fastrand f32; row block only ceilf host hook',full_game_executed=False,limitations=['random_splits Vec preallocated to avoid allocator paths; comparison covers float output, RNG state, and result length','ceilf external call handled with IEEE integral rounding preserving nonfinite values','MXCSR 0x1f80 only','This validates partition/row layout kernels, not the full tile transform pipeline'])
(O/'helpers-verification.json').write_text(json.dumps(report,indent=2),encoding='utf-8');print(json.dumps(report)[:5000])
if not report['ok']:raise SystemExit(1)
