"""Actual platform step ranges -> blocker closure ->104B segment consumer."""
import sys,struct
from pathlib import Path
from native_platform_segmentation import process,call
sys.path.insert(0,str(Path(__file__).resolve().parents[1]/'building-structures'))
from native_structures import *
from native_platform_segmentation import call

def canonical(ptr,n):
 out=[]
 for i in range(n):
  p=ptr+i*104;parts=[]
  for at,width in [(0,4),(24,12),(48,4)]:
   cap,arr,count=struct.unpack('<3Q',cpu.mem_read(p+at,24));parts.append(struct.pack('<Q',count)+bytes(cpu.mem_read(arr,count*width)))
  out.append(b''.join(parts)+bytes(cpu.mem_read(p+72,28)))
 return out

def prepare_steps(raw_curve,points,us,values,arch,shape,special,maximum,blockers,shape_fields=(.2,2.,3.)):
 _,steps=process(points,values,shape,special,maximum,0,shape_fields)
 # Original raw Curve2 builder provides all copied headers/U/length; original
 # coordinate projection and distance run, no privategeometry bridge.
 rp=data+0x4000;cpu.mem_write(rp,b''.join(struct.pack('<2f',*p)for p in raw_curve));vec=data+0x4100;cpu.mem_write(vec,struct.pack('<3Q',len(raw_curve),rp,len(raw_curve)))
 call(0x1411f9920,[(UC_X86_REG_RCX,data+0x7000),(UC_X86_REG_RDX,vec),(UC_X86_REG_R8,0)])
 cpu.mem_write(data,bytes(cpu.mem_read(data+0x7000,56)));cpu.mem_write(data+0x28c,bytes(cpu.mem_read(data+0x30,4)))
 up=data+0x4200;ap=data+0x4300;cpu.mem_write(up,struct.pack('<'+'f'*len(us),*us));cpu.mem_write(ap,struct.pack('<'+'f'*len(arch),*arch));cpu.mem_write(data+0xa0,struct.pack('<3Q',len(us),up,len(us)));cpu.mem_write(data+0xd8,struct.pack('<3Q',len(arch),ap,len(arch)))
 bp=data+0x4400;cpu.mem_write(bp,b''.join(struct.pack('<3f',*coord,width)for coord,width in blockers));bv=data+0x4500;cpu.mem_write(bv,struct.pack('<3Q',len(blockers),bp,len(blockers)))
 output=[]
 for step in steps:
  original_range=bytes.fromhex(step['range']);sr=data+0x5000;cpu.mem_write(sr,original_range);sub=data+0x5010;call(0x141200510,[(UC_X86_REG_RCX,sub),(UC_X86_REG_RDX,sr),(UC_X86_REG_R8,step['start']),(UC_X86_REG_R9,step['end'])])
  view=data+0x5100;cpu.mem_write(view,struct.pack('<2Q',data+0x2000,len(points))+bytes(cpu.mem_read(sub,12))+bytes(4)+struct.pack('<2Q',data,bv));permit=data+0x5200
  call(0x141206070,[(UC_X86_REG_RCX,permit),(UC_X86_REG_RDX,view),(UC_X86_REG_R8,data+0x9000)])
  out=data+0x5300;cpu.mem_write(out,struct.pack('<3Q',0,8,0));start=data+0x5400;cpu.mem_write(start,struct.pack('<Q',step['start']));callback=data+0x5500;cpu.mem_write(callback,struct.pack('<5Q',sr,start,data,out,0));_,ptr,n=struct.unpack('<3Q',cpu.mem_read(permit,24))
  call(0x1411ff790,[(UC_X86_REG_RCX,ptr),(UC_X86_REG_RDX,n),(UC_X86_REG_R9,callback)],bits(0.))
  _,ptr,n=struct.unpack('<3Q',cpu.mem_read(out,24));output.extend(canonical(ptr,n))
 return output,steps
