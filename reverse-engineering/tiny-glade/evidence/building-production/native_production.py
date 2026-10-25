"""Full original B WallConstructor::from_curve ABI oracle. No game-rule bridges.
Reuse pinnedPE execution environment and IAT-identified standard bridges only.
Input buffers and output reserves own distinct allocations for every case.
"""
import sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
sys.path.insert(0,str(ROOT/'evidence/building-constructor'))
from native_fixture import *

def produce(points,us,length,arch,second,third,holes,seed,material=23,closed=True,base_height=0.,perturb=0.,width_padding=None,thickness_scale=None,wall_float=1.,holes_length=None,kind=2,top_bounds=(0.,0.,0.,0.,0.,2),fence_fit=False):
 allocation[0]=heap;cpu.mem_write(data,bytes(0x100000));cpu.mem_write(stack,bytes(0x200000));n=len(points)
 assert 2<=n<=65536 and all(len(a)==n for a in [us,arch,second,third]), 'source array count mismatch or fixture capacity exceeded'
 assert len(holes)<=65536, 'fixture hole capacity exceeded'
 def block(payload):
  p=alloc(len(payload));cpu.mem_write(p,payload);return p
 points_ptr=block(b''.join(struct.pack('<2f',*p)for p in points))
 us_ptr=block(struct.pack('<'+'f'*n,*us))
 arch_ptr=block(struct.pack('<'+'f'*n,*arch));second_ptr=block(struct.pack('<'+'f'*n,*second));third_ptr=block(struct.pack('<'+'f'*n,*third))
 holes_ptr=block(b''.join(struct.pack('<4fBBxx',*a,k,origin)for a,k,origin in holes))
 scratch_ptr=alloc(8192*96);output_ptr=alloc(8192*96)
 curve=data+0x100;cpu.mem_write(curve,struct.pack('<QQQQQQf',n,points_ptr,n,n,us_ptr,n,length))
 cpu.mem_write(data+0x4000,struct.pack('<QQQfB',len(holes),holes_ptr,len(holes),(length if holes_length is None else holes_length),int(closed)))
 cpu.mem_write(data,struct.pack('<QQQ',8192,scratch_ptr,0));cpu.mem_write(data+0x5000,struct.pack('<QQQ',8192,output_ptr,0));cpu.mem_write(data+0x6000,struct.pack('<5fB',*top_bounds))
 sp=stack+0x100008;cpu.mem_write(sp,struct.pack('<Q',stop));cpu.reg_write(UC_X86_REG_RSP,sp);cpu.reg_write(UC_X86_REG_RCX,data);cpu.reg_write(UC_X86_REG_RDX,curve);cpu.reg_write(UC_X86_REG_R8,material);cpu.reg_write(UC_X86_REG_R9,arch_ptr);cpu.reg_write(UC_X86_REG_MXCSR,0x1f80)
 args={0x28:n,0x30:second_ptr,0x38:n,0x40:third_ptr,0x48:n,0x50:kind,0x58:seed,0x60:data+0x4000,0x68:bits(perturb),0x70:data+0x5000,0x78:bits(base_height),0x80:data+0x6000,0x88:int(width_padding is not None),0x90:bits(width_padding or 0.),0x98:int(thickness_scale is not None),0xa0:bits(thickness_scale or 0.),0xa8:bits(wall_float),0xb0:int(fence_fit)}
 for o,v in args.items():cpu.mem_write(sp+o,struct.pack('<Q',v))
 cpu.emu_start(0x140769760,stop,count=10000000)
 assert cpu.reg_read(UC_X86_REG_RIP)==stop, f'original from_curve did not return, RIP={cpu.reg_read(UC_X86_REG_RIP):x}'
 cap,p,count=struct.unpack('<3Q',cpu.mem_read(data+0x5000,24))
 assert count<=cap and heap<=p and p+cap*96<=heap+0x1000000, 'output Vec bounds invalid'
 return [bytes(cpu.mem_read(p+i*96,96))for i in range(count)]
