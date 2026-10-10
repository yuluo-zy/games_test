from pathlib import Path
import struct,json,random,hashlib,subprocess
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
ROOT=Path('D:/game/reverse-engineering/tiny-glade')
raw=Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes()
sha=hashlib.sha256(raw).hexdigest();assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=struct.unpack_from('<I',raw,60)[0];ns=struct.unpack_from('<H',raw,pe+6)[0];os=struct.unpack_from('<H',raw,pe+20)[0]
base=struct.unpack_from('<Q',raw,pe+48)[0];sz=struct.unpack_from('<I',raw,pe+80)[0]
cpu=Uc(UC_ARCH_X86,UC_MODE_64);cpu.mem_map(base,(sz+4095)&~4095)
for i in range(ns):
 z,a,s,o=struct.unpack_from('<IIII',raw,pe+24+os+40*i+8);cpu.mem_write(base+a,raw[o:o+s])
DATA=0x20000000;STACK=0x30000000;STOP=0x40000000
cpu.mem_map(DATA,0x2000);cpu.mem_map(STACK,0x2000);cpu.mem_map(STOP,0x1000)
panic=[]
def hook(c,pc,n,user):
 if pc==0x1428d9390:panic.append(pc);c.emu_stop()
cpu.hook_add(UC_HOOK_CODE,hook)
rng=random.Random(20261013);lines=[];expected=[];counts={}
def record(line,result):lines.append(line);expected.append(result);counts[line.split()[0]]=counts.get(line.split()[0],0)+1
edges=[0,0x80000000,1,0x80000001,0x3dcccccc,0x3dcccccd,0x3dccccce,0x3f000000,0x3f333333,0x3fcccccd,0x40200000,0x41600000,0x7f800000,0xff800000,0x7fc00000,0x7f800001,0xffc00001]
for rb in [None]+edges:
 for flag in [0,1,2,255]:
  for desired in edges+[rng.getrandbits(32) for _ in range(48)]:
   cpu.reg_write(UC_X86_REG_RAX,0 if rb is None else DATA)
   if rb is not None:cpu.mem_write(DATA+0x30,struct.pack('<I',rb))
   cpu.reg_write(UC_X86_REG_XMM6,desired);cpu.reg_write(UC_X86_REG_RBP,flag)
   cpu.reg_write(UC_X86_REG_MXCSR,0x1f80);cpu.reg_write(UC_X86_REG_RSP,STACK+0x800)
   cpu.mem_write(STACK+0x868,struct.pack('<Q',STOP))
   # Start at the game-owned numeric policy after PublicWalls::get_roof returns.
   # Hash/ECS lookup is outside this reconstructed rule and is not stubbed.
   cpu.emu_start(0x140ac8040,STOP,count=70);assert cpu.reg_read(UC_X86_REG_RIP)==STOP
   assert cpu.reg_read(UC_X86_REG_RAX)==1
   result=cpu.reg_read(UC_X86_REG_XMM0)&0xffffffff
   record(f'clamp {desired:08x} '+('none' if rb is None else f'{rb:08x}')+f' {flag}',f'{result:08x}')
for mode,va,flag_off,value_off in [('max',0x140b2a620,0x108,0x10c),('gable',0x140b2a680,0x110,0x114),('flat',0x140b2a6e0,0x108,0x10c)]:
 for flag in [0,1,2,3,127,128,254,255]:
  for vb in edges+[rng.getrandbits(32) for _ in range(64)]:
   panic.clear();cpu.mem_write(DATA+flag_off,bytes([flag]));cpu.mem_write(DATA+value_off,struct.pack('<I',vb))
   cpu.reg_write(UC_X86_REG_RCX,DATA);cpu.reg_write(UC_X86_REG_MXCSR,0x1f80);cpu.reg_write(UC_X86_REG_RSP,STACK+0x808);cpu.mem_write(STACK+0x808,struct.pack('<Q',STOP))
   cpu.emu_start(va,STOP,count=50)
   result='panic' if panic else f'{cpu.reg_read(UC_X86_REG_XMM0)&0xffffffff:08x}'
   record(f'{mode} {flag} {vb:08x}',result)
exe=ROOT/'workspace/wall-height-rules.exe'
subprocess.run(['C:/Users/liyu/.cargo/bin/rustc.exe','--edition=2024','-O',str(ROOT/'reconstruction/wall/height_rules.rs'),'-o',str(exe)],check=True)
actual=subprocess.run([str(exe)],input='\n'.join(lines)+'\n',text=True,capture_output=True,check=True).stdout.splitlines()
fails=[dict(case=i,input=lines[i],original=a,candidate=b) for i,(a,b) in enumerate(zip(expected,actual)) if a!=b]
report=dict(ok=len(actual)==len(expected) and not fails,source_sha256=sha,total_cases=len(lines),counts=counts,failures=fails,
 candidate_sources={'reconstruction/wall/height_rules.rs':hashlib.sha256((ROOT/'reconstruction/wall/height_rules.rs').read_bytes()).hexdigest()},
 functions=[dict(name='clamp_wall_height resolved numeric policy',entry='0x140ac8040',end='0x140ac80bd',scope='suffix of PDB-confirmed 422-byte original game function; map/query lookup represented by prepared resolved state'),dict(name='PublicWallState::{max_y,max_y_gable,flat_roof_y}',scope='complete original game leaf functions including observed panic on absent cache')],
 limitations=['Clamp hash lookup, wall existence/validity checks and Bevy query lookup are outside tested numeric policy','Special flag producer semantic is unknown','MXCSR=0x1f80; floating exception flags and panic message/unwind not compared','No std/Bevy/glam/hash implementation was reconstructed'])
(ROOT/'evidence/scope-priorities/wall-rules-verification.json').write_text(json.dumps(report,indent=2),encoding='utf8');print(json.dumps({k:v for k,v in report.items() if k not in ['functions','limitations']},indent=2))
if not report['ok']:raise SystemExit(1)
