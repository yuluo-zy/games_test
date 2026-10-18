"""B movement-phase branch instructions with resolved row pointers.
Original code mutates timers and chooses transition/retention/goal refresh.
No RNG/camera/ECS query discovery/goal generation is executed by this probe.
"""
import json,struct,hashlib
from pathlib import Path
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
import pefile
ROOT=next(p for p in Path(__file__).resolve().parents if (p/'ljxsj_92385').is_dir())
raw=(ROOT/'ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes()
assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(pe.OPTIONAL_HEADER.SizeOfImage+4095)&~4095);u.mem_write(base,pe.get_memory_mapped_image())
DATA=0x20000000;STACK=0x30000000
u.mem_map(DATA,0x10000);u.mem_map(STACK,0x10000)
SHEEP=DATA;GOAL=DATA+0x100;TRANSFORM=DATA+0x200;TICK=DATA+0x300
B=lambda v:struct.unpack('<I',struct.pack('<f',v))[0]
exits={0x1423152a5:'keep',0x142315247:'start_walking',0x1423158a3:'start_idle',0x1423158d3:'refresh'}
seen=[]
def stop(c,pc,size,_):
 if pc in exits:seen.append(exits[pc]);c.emu_stop()
u.hook_add(UC_HOOK_CODE,stop)
cases=[]
for walking,timeout,cooldown,force,active,position,goal in [
 (False,2.,0.,False,True,[0.,0.,0.],[2.,0.,0.]),(False,.1,0.,False,True,[0.,0.,0.],[2.,0.,0.]),
 *[(True,t,c,f,True,[0.,0.,0.],[2.,0.,0.])for t in [2.,.1]for c in [2.,.1]for f in [False,True]],
 (True,2.,.1,True,True,[0.,0.,0.],[0.,100.,0.]),
 (True,2.,2.,True,False,[0.,0.,0.],[2.,0.,0.]),
 (True,2.,2.,True,True,[0.,0.,0.],[.1,0.,0.]),
 (True,.1,2.,True,True,[0.,0.,0.],[.1,0.,0.]),
 (True,2.,float('nan'),False,True,[0.,0.,0.],[2.,0.,0.]),
 (True,float('nan'),2.,False,True,[0.,0.,0.],[2.,0.,0.]),
]:
 u.mem_write(DATA,bytes(0x1000));u.mem_write(STACK,bytes(0x10000));sp=STACK+0x8000
 u.mem_write(SHEEP+0x30,struct.pack('<BB2x2f',int(walking),int(force),timeout,cooldown))
 u.mem_write(GOAL,struct.pack('<I3f',int(active),*goal));u.mem_write(TRANSFORM+0x10,struct.pack('<3f',*position));u.mem_write(sp+0xbc,bytes([0]))
 for r in [UC_X86_REG_RAX,UC_X86_REG_RBX,UC_X86_REG_RDX,UC_X86_REG_R8,UC_X86_REG_R9,UC_X86_REG_R10,UC_X86_REG_R11,UC_X86_REG_R12,UC_X86_REG_R15]:u.reg_write(r,0)
 u.reg_write(UC_X86_REG_RCX,TICK);u.reg_write(UC_X86_REG_R13,SHEEP);u.reg_write(UC_X86_REG_R14,GOAL);u.reg_write(UC_X86_REG_RSI,TRANSFORM);u.reg_write(UC_X86_REG_RSP,sp)
 u.reg_write(UC_X86_REG_XMM10,B(.25)|(B(.25)<<32));u.reg_write(UC_X86_REG_XMM11,B(.25));u.reg_write(UC_X86_REG_XMM12,0);u.reg_write(UC_X86_REG_MXCSR,0x1f80);u.reg_write(UC_X86_REG_RFLAGS,0x202)
 seen.clear();u.emu_start(0x1423157d8,0x142315e36,count=1000);assert len(seen)==1
 result=struct.unpack('<BB2x2I',u.mem_read(SHEEP+0x30,12))
 cases.append({'walking':walking,'timeout':B(timeout),'goal_change_timeout':B(cooldown),'force':force,'active':active,'position':[B(x)for x in position],'goal':[B(x)for x in goal],'dt':B(.25),'decision':seen[0],'movement_after_bits':list(result)})
direction_cases=[]
# Resolved disk samples; the original quaternion projection and normalization run.
sqrt_half=struct.unpack('<f',struct.pack('<f',.5**.5))[0]
for quaternion,scale,disk in [
 ([0.,0.,0.,1.],.8,[.2,.3]),([0.,0.,0.,1.],.5,[-.4,.7]),
 ([0.,sqrt_half,0.,sqrt_half],.8,[.2,.3]),([sqrt_half,0.,0.,sqrt_half],.8,[.2,.3]),
 ([0.,0.,sqrt_half,sqrt_half],.5,[-.4,.7]),
 ([0.,0.,0.,1.],.8,[0.,.4]),([0.,0.,0.,1.],.5,[0.,.25]),
 ([0.,0.,0.,1.],.8,[0.,0.]),
]:
 u.mem_write(DATA,bytes(0x1000));u.mem_write(STACK,bytes(0x10000));sp=STACK+0x8000
 u.mem_write(TRANSFORM,struct.pack('<4f',*quaternion));u.mem_write(sp+0x100,struct.pack('<3f',disk[0],0.,disk[1]))
 u.reg_write(UC_X86_REG_RSP,sp);u.reg_write(UC_X86_REG_RBP,sp+0x30);u.reg_write(UC_X86_REG_RSI,TRANSFORM);u.reg_write(UC_X86_REG_XMM9,B(scale));u.reg_write(UC_X86_REG_XMM13,B(1.));u.reg_write(UC_X86_REG_MXCSR,0x1f80)
 u.emu_start(0x14231596a,0x142315a43,count=1000);assert u.reg_read(UC_X86_REG_RIP)==0x142315a43
 xy=u.reg_read(UC_X86_REG_XMM0);z=u.reg_read(UC_X86_REG_XMM9)&0xffffffff
 unnormalized=[xy&0xffffffff,(xy>>32)&0xffffffff,z]
 u.emu_start(0x142315a43,0x142315acd,count=1000);assert u.reg_read(UC_X86_REG_RIP)==0x142315acd
 xy=u.reg_read(UC_X86_REG_XMM2);z=u.reg_read(UC_X86_REG_XMM9)&0xffffffff
 direction_cases.append({'quaternion':[B(x)for x in quaternion],'scale':B(scale),'disk':[B(x)for x in disk],'raw_direction':unnormalized,'direction':[xy&0xffffffff,(xy>>32)&0xffffffff,z]})
report={'target':'B','binary_sha256':hashlib.sha256(raw).hexdigest(),'entry':'0x1423157d8','target_executed':True,'candidate_executed':False,'scope':'Resolved movement row/goal/transform pointers. Original timer updates and force/goal-retention control-flow execute; stop before RNG transitions or target generation. Direction probes start after resolved disk sample and execute original quaternion/XZ projection and normalization. No ECS discovery, complete goals system, spawn or scheduling acceptance.','cases':cases,'direction_cases':direction_cases}
(Path(__file__).parent/'force-phase-native.json').write_text(json.dumps(report,indent=2)+'\n')
print({'cases':len(cases),'direction_cases':len(direction_cases),'decisions':[x['decision']for x in cases]})
