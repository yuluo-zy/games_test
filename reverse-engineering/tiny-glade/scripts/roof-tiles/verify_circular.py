from pathlib import Path
import struct,json,subprocess,math,ctypes,hashlib,random
from unicorn import *
from unicorn.x86_const import *
R=Path('D:/game/reverse-engineering/tiny-glade');O=R/'evidence/roof-tiles'
raw=Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes();assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03';pe=struct.unpack_from('<I',raw,60)[0];op=struct.unpack_from('<H',raw,pe+20)[0]
base=struct.unpack_from('<Q',raw,pe+48)[0];size=struct.unpack_from('<I',raw,pe+24+56)[0]
ss=[struct.unpack_from('<IIII',raw,pe+24+op+40*i+8) for i in range(struct.unpack_from('<H',raw,pe+6)[0])]
cpu=Uc(UC_ARCH_X86,UC_MODE_64);cpu.mem_map(base,(size+4095)&~4095)
for z,v,n,r in ss:cpu.mem_write(base+v,raw[r:r+n])
data=0x20000000;stack=0x30000000;heap=0x50000000;cpu.mem_map(data,0x10000);cpu.mem_map(stack,0x10000);cpu.mem_map(heap,0x400000)
crt=ctypes.CDLL('ucrtbase.dll');maths={0x142923e60:'ceilf',0x142923fd0:'sinf',0x142923e80:'cosf',0x142923e30:'atanf',0x142923f10:'fmodf',0x142923f70:'powf'}
for name in maths.values():f=getattr(crt,name);f.argtypes=[ctypes.c_float]* (2 if name in ['fmodf','powf'] else 1);f.restype=ctypes.c_float
def rf(reg):return struct.unpack('<f',struct.pack('<I',cpu.reg_read(reg)&0xffffffff))[0]
def ret():
 sp=cpu.reg_read(UC_X86_REG_RSP);ra=int.from_bytes(cpu.mem_read(sp,8),'little');cpu.reg_write(UC_X86_REG_RSP,sp+8);cpu.reg_write(UC_X86_REG_RIP,ra)
tiles=[];nextheap=heap;calls={};trace=[]
def hook(cpu,a,n,u):
 global nextheap
 trace.append(a)
 if len(trace)>50:trace.pop(0)
 if a in maths:
  calls[maths[a]]=calls.get(maths[a],0)+1;v=getattr(crt,maths[a])(rf(UC_X86_REG_XMM0),rf(UC_X86_REG_XMM1)) if maths[a] in ['fmodf','powf'] else getattr(crt,maths[a])(rf(UC_X86_REG_XMM0))
  cpu.reg_write(UC_X86_REG_XMM0,int.from_bytes(struct.pack('<f',v),'little'));ret()
 elif a==0x140613c10:
  n=cpu.reg_read(UC_X86_REG_RCX);align=cpu.reg_read(UC_X86_REG_RDX);nextheap=(nextheap+align-1)&~(align-1);cpu.reg_write(UC_X86_REG_RAX,nextheap);nextheap+=(n+15)&~15;ret()
 elif a==0x140613c20:ret()
 elif a==0x1421b4510:tiles.append(bytes(cpu.mem_read(cpu.reg_read(UC_X86_REG_RDX),64)).hex());ret()
cpu.hook_add(UC_HOOK_CODE,hook)
runner=O/'circular_runner.rs'
runner.write_text('''#[path="D:/game/reverse-engineering/tiny-glade/reconstruction/roof/surface.rs"] mod surface;
#[path="D:/game/reverse-engineering/tiny-glade/reconstruction/roof/tiles.rs"] mod tiles;
use std::io::{self,BufRead};fn f(s:&str)->f32{f32::from_bits(u32::from_str_radix(s,16).unwrap())}
fn main(){for l in io::stdin().lock().lines(){let l=l.unwrap();let x:Vec<_>=l.split_whitespace().collect();if x[0]=="roof" {let roof:[u8;88]=(0..176).step_by(2).map(|i|u8::from_str_radix(&x[1][i..i+2],16).unwrap()).collect::<Vec<_>>().try_into().unwrap();for t in tiles::assemble_circular_from_observed(&roof,x[2]=="1").unwrap(){for b in t.0{print!("{:02x}",b);}print!(" ");}println!();continue;}let mut a=0;let mut next=||{let v=f(x[a]);a+=1;v};let len=next();let center=[next(),next()];let radius=next();let tip=[next(),next()];let rotation=next();let roof=next().to_bits();let mode=next().to_bits()!=0;let mut pts=Vec::new();let mut us=Vec::new();for _ in 0..(x.len()-9)/3{pts.push([next(),next()]);us.push(next());}
let input=tiles::CircularTileInput{profile_points:&pts,profile_u:&us,profile_length:len,center_xz:center,roof_radius:radius,tip_offset_normalized:tip,circle_rotation:rotation,roof_id:roof,special_mode:mode};let y=tiles::assemble_circular_tiles(&input);for t in y{for b in t.0{print!("{:02x}",b);}print!(" ");}println!();}}
''',encoding='utf-8')
exe=R/'workspace/circular-reconstructed.exe'
import sys
sys.path.insert(0,str(R/'scripts'))
from dependency_build import build_runner
build_runner(runner,exe)
def fbits(v):return struct.unpack('<I',struct.pack('<f',v))[0]
def native(case):
 global tiles,nextheap
 tiles=[];nextheap=heap
 pts,us,length,center,radius,tip,rotation,roof,mode=case
 rbp=stack+0xe000;cpu.mem_write(stack,bytes(0x10000));cpu.mem_write(data,bytes(0x10000));cpu.reg_write(UC_X86_REG_RBP,rbp);cpu.reg_write(UC_X86_REG_RSP,rbp-0x80);cpu.reg_write(UC_X86_REG_MXCSR,0x1f80)
 for reg in [UC_X86_REG_RBX,UC_X86_REG_RDI,UC_X86_REG_R13,UC_X86_REG_R14]:cpu.reg_write(reg,0)
 cpu.mem_write(rbp+0x58,struct.pack('<QQQQQQf',len(pts),data+0x1000,len(pts),len(us),data+0x2000,len(us),length));cpu.mem_write(data+0x1000,b''.join(struct.pack('<ff',*p) for p in pts));cpu.mem_write(data+0x2000,struct.pack('<'+'f'*len(us),*us))
 cpu.mem_write(data,struct.pack('<I',roof));cpu.mem_write(data+0x14,struct.pack('<f',radius));cpu.mem_write(data+0x24,struct.pack('<ff',*tip));cpu.mem_write(data+0x100,struct.pack('<ffff',center[0],center[1],radius,rotation))
 cpu.reg_write(UC_X86_REG_RBX,data);cpu.reg_write(UC_X86_REG_RDI,data+0x100);cpu.reg_write(UC_X86_REG_R14,mode)
 try:
  cpu.emu_start(0x1421b31f9,0x1421b3f37,count=4000000)
  assert cpu.reg_read(UC_X86_REG_RIP)==0x1421b3f37,'prepared numeric body did not reach cleanup boundary'
 except Exception as e:raise RuntimeError(f'{e}; trace='+str([hex(a) for a in trace]))
 return tiles[:]
cases=[([[2.0,0.0],[1.0,2.0],[0.1,3.0]],[0.0,0.6,1.0],4.0,[3.0,-1.0],2.0,[0.0,0.0],0.0,100,False)]
rand=random.Random(20261010)
for i in range(24):
 radius=rand.uniform(.2,8);height=rand.uniform(.2,7);pts=[[radius,0],[radius*.5,height*.7],[.1,height]];us=[0,.6,1];length=rand.uniform(.9,6)
 cases.append((pts,us,length,[rand.uniform(-3,3),rand.uniform(-3,3)],radius,[rand.uniform(-.8,.8),rand.uniform(-.8,.8)],rand.uniform(-8,8),i,i%2==0))
lines=[];expected=[]
for c in cases:
 pts,us,length,center,radius,tip,rot,roof,mode=c;bits=[fbits(v) for v in [length,*center,radius,*tip,rot]]+[roof,int(mode)]
 for p,u in zip(pts,us):bits += [fbits(p[0]),fbits(p[1]),fbits(u)]
 lines.append(' '.join(f'{b:08x}' for b in bits));expected.append(native(c))
stop=0x40000000;cpu.mem_map(stop,4096)
def full_native(roof,mode):
 global tiles,nextheap
 roof=bytes(roof)
 tiles=[];nextheap=heap;cpu.mem_write(data,bytes(0x10000));cpu.mem_write(stack,bytes(0x10000));cpu.mem_write(data,roof);cpu.mem_write(data+0x100,roof[0xc:0x1c]);cpu.mem_write(stack+0xe008,struct.pack('<Q',stop))
 cpu.reg_write(UC_X86_REG_RSP,stack+0xe008);cpu.reg_write(UC_X86_REG_RCX,data+0x100);cpu.reg_write(UC_X86_REG_RDX,data);cpu.reg_write(UC_X86_REG_R8,mode);cpu.reg_write(UC_X86_REG_R9,0);cpu.reg_write(UC_X86_REG_MXCSR,0x1f80)
 try:
  cpu.emu_start(0x1421b3120,stop,count=6000000)
  assert cpu.reg_read(UC_X86_REG_RIP)==stop,'whole circular function did not return'
 except Exception as e:raise RuntimeError(f'{e}; trace='+str([hex(a) for a in trace]))
 return tiles[:]
fixtures=json.loads((R/'evidence/roof-fixtures.json').read_text(encoding='utf-8'))['samples'];chosen=[];seen_profiles=set()
for sample in fixtures:
 p=sample['params'];profile=p['profile_01']
 if profile not in seen_profiles:chosen.append(sample);seen_profiles.add(profile)
 if len(chosen)>=20:break
full_cases=[]
for i,sample in enumerate(chosen):
 p=sample['params'];roof=bytearray(88);struct.pack_into('<I',roof,0,i+500);radius=0.3+(i%5)*.8
 for off,val in [(0x0c,float(i%4)),(0x10,-float(i%3)),(0x14,radius),(0x18,(i%4)*.7),(0x24,p['tip_offset_01'][0]),(0x28,p['tip_offset_01'][1]),(0x2c,p['profile_01']),(0x30,p['height_01']),(0x34,p['ridge_length_01']),(0x38,p['eave_length_01'])]:struct.pack_into('<f',roof,off,val)
 expected.append(full_native(roof,i%2));lines.append(f'roof {roof.hex()} {i%2}');full_cases.append(dict(source=sample,synthetic_circle_radius=radius,roof_bytes=roof.hex(),mode=i%2))
result=subprocess.run([str(exe)],input='\n'.join(lines)+'\n',capture_output=True,text=True,check=True).stdout.splitlines();actual=[l.split() for l in result]
fail=[]
for i,(a,b) in enumerate(zip(expected,actual)):
 if a!=b:
  diffs=[]
  for j,(aa,bb) in enumerate(zip(a,b)):
   if aa!=bb:diffs.append(dict(tile=j,offsets=[k for k,(av,bv) in enumerate(zip(bytes.fromhex(aa),bytes.fromhex(bb))) if av!=bv],original=aa,rust=bb))
  fail.append(dict(case=i,original_count=len(a),rust_count=len(b),differences=diffs[:6]))
report=dict(ok=not fail and len(result)==len(lines),source_sha256=hashlib.sha256(raw).hexdigest(),prepared_profile_cases=len(cases),whole_function_cases=len(full_cases),original_tiles=sum(map(len,expected)),failures=fail,math_call_counts=calls,scope='Prepared-profile cases and entire original circular function entry→RET; original curve construction, profile generation, geometry, random splits, RNG and f16 instructions; only allocator, free, CRT math and emit sink bridges',full_game_executed=False,limitations=['Prepared-profile tests inject curve at exact profile_curve_ws return boundary; full-function tests run original constructors too','Full-function roof parameters drawn from shipped histories; circle sizes/poses are synthetic because history-to-wall-state replay is not yet recovered','CRT bridges invoke current host Windows ucrtbase float functions','ECS sparse writer replaced by capture at original closure; compare all 64 emitted bytes'])
report['rust_candidate_sha256']=hashlib.sha256((R/'reconstruction/roof/tiles.rs').read_bytes()).hexdigest()
kernel=ctypes.WinDLL('kernel32',use_last_error=True);kernel.GetModuleFileNameW.argtypes=[ctypes.c_void_p,ctypes.c_wchar_p,ctypes.c_uint32];buf=ctypes.create_unicode_buffer(32768);assert kernel.GetModuleFileNameW(crt._handle,buf,len(buf));crtpath=Path(buf.value)
report['crt_dependency']=dict(path=str(crtpath),sha256=hashlib.sha256(crtpath.read_bytes()).hexdigest(),functions=list(maths.values()))
(O/'circular-verification.json').write_text(json.dumps(report,indent=2),encoding='utf-8');(O/'circular-fixtures.json').write_text(json.dumps(dict(cases=cases,full_cases=full_cases,inputs=lines,original=expected),indent=2),encoding='utf-8');print(json.dumps(report)[:6000])
if not report['ok']:raise SystemExit(1)
