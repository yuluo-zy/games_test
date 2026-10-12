"""Bounded resolved per-wall original preparation body. Explicit4encodedpaths."""
import sys,struct,ctypes
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
sys.path.insert(0,str(ROOT/'evidence/building-constructor'))
from native_fixture import *
expf=lib.expf;expf.argtypes=[ctypes.c_float];expf.restype=ctypes.c_float
counts={}
markers={0x140a14df0:'actual_grid_sample',0x14074d7d5:'low_to_high_transition',0x14074dac4:'high_to_low_transition',0x14074d692:'high_height_step_midpoint'}

def extra(c,pc,size,user):
 if pc in markers:counts[markers[pc]]=counts.get(markers[pc],0)+1
 if pc==0x142923ec0:cpu.reg_write(UC_X86_REG_XMM0,bits(expf(fl(cpu.reg_read(UC_X86_REG_XMM0)))));ret()
cpu.hook_add(UC_HOOK_CODE,extra)

def prepare(points,relative,source5c,sampling_fix,encoded=None):
 counts.clear()
 allocation[0]=heap+0x100000;cpu.mem_write(heap,bytes(0x100000));cpu.mem_write(data,bytes(0x100000));cpu.mem_write(stack,bytes(0x200000));n=len(points)
 cpu.mem_write(data+0x38,struct.pack('<QQQQQQf',n,data+0x1000,n,n,data+0x2000,n,1.));cpu.mem_write(data+0x1000,b''.join(struct.pack('<3f',*p)for p in points));cpu.mem_write(data+0x2000,struct.pack('<'+'f'*n,*[i/(n-1)for i in range(n)]));cpu.mem_write(data+0x70,struct.pack('<QQQ',n,data+0x3000,n));cpu.mem_write(data+0x3000,struct.pack('<'+'f'*n,*relative));cpu.mem_write(data+0xc0,struct.pack('<QQQ',n,data+0x4000,0));cpu.mem_write(data+0x4000,bytes(n*4));wf(data+0x288,source5c)
 frame=stack+0x100080;sp=stack+0x100000
 if encoded is not None:
  assert len(encoded)==4 and all(len(v)==0x40000 for v in encoded)
  for i,v in enumerate(encoded):cpu.mem_write(heap+i*0x40000,bytes(v))
 for i in range(4):cpu.mem_write(frame-0x40+i*16,struct.pack('<QQ',heap+i*0x40000,0x40000))
 cpu.mem_write(0x14362c308,struct.pack('<Q',data+0x8000));cpu.mem_write(data+0x8000,bytes([sampling_fix]))
 for r in [UC_X86_REG_RAX,UC_X86_REG_RBX,UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_RDI,UC_X86_REG_RSI,UC_X86_REG_RBP,UC_X86_REG_R8,UC_X86_REG_R9,UC_X86_REG_R10,UC_X86_REG_R11,UC_X86_REG_R12,UC_X86_REG_R13,UC_X86_REG_R14,UC_X86_REG_R15]:cpu.reg_write(r,0)
 cpu.reg_write(UC_X86_REG_RSP,sp);cpu.reg_write(UC_X86_REG_RBP,frame);cpu.reg_write(UC_X86_REG_RSI,data+0x2a0);cpu.reg_write(UC_X86_REG_R11,0);cpu.reg_write(UC_X86_REG_RDI,frame-0x30);cpu.reg_write(UC_X86_REG_RBX,frame-0x20);cpu.reg_write(UC_X86_REG_R14,frame-0x10);cpu.reg_write(UC_X86_REG_R15,frame-0x40);cpu.reg_write(UC_X86_REG_MXCSR,0x1f80)
 try:cpu.emu_start(0x14074cbe0,0x14074e3dc,count=5000000)
 except Exception as e:raise RuntimeError(f'{e} RIP {cpu.reg_read(UC_X86_REG_RIP):x} last {last[-10:]}')from e
 assert cpu.reg_read(UC_X86_REG_RIP)==0x14074e3dc
 ptr,count=u64(data+0x90),u64(data+0x98);ups,uncount=u64(data+0xa8),u64(data+0xb0);arrp,arrn=u64(data+0xc8),u64(data+0xd0)
 return {'points':bytes(cpu.mem_read(ptr,count*12)).hex(),'points_u':bytes(cpu.mem_read(ups,uncount*4)).hex(),'length':bytes(cpu.mem_read(data+0xb8,4)).hex(),'segments':bytes(cpu.mem_read(arrp,arrn*4)).hex(),'count':count,'segment_count':arrn,'observed_counts':dict(counts)}
if __name__=='__main__':print(prepare([[0.,0.,0.],[1.,.2,0.],[2.,.4,0.]],[0.,0.,0.],1.,True))
