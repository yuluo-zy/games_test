"""Unaccepted full original indexed mesh owner incl dependency calls. No private output bridge."""
from native_platform_collision_hash import *
def full(points,triangles):
 allocation[0]=heap;cpu.mem_write(data,bytes(0x10000));cpu.mem_write(stack,bytes(0x200000));
 for at,arr,payload in [(0x100,triangles,b''.join(struct.pack('<3I',*p)for p in triangles)),(0x200,points,b''.join(struct.pack('<3f',*p)for p in points))]:
  p=alloc(len(payload));cpu.mem_write(p,payload);cpu.mem_write(data+at,struct.pack('<3Q',len(arr),p,len(arr)))
 cpu.mem_write(data+0x300,struct.pack('<3Q',0x8000000000000000,4,0));cpu.mem_write(data+0x400,struct.pack('<3Q',0x8000000000000000,4,0));sp=stack+0x100008;cpu.mem_write(sp,struct.pack('<Q',stop));cpu.mem_write(sp+0x28,struct.pack('<Q',data+0x400))
 for r,v in [(UC_X86_REG_RSP,sp),(UC_X86_REG_RCX,data),(UC_X86_REG_RDX,data+0x100),(UC_X86_REG_R8,data+0x200),(UC_X86_REG_R9,data+0x300)]:cpu.reg_write(r,v)
 cpu.emu_start(0x1408cee40,stop,count=10000000)
 return bytes(cpu.mem_read(data,144))
if __name__=='__main__':
 import json
 try:out=full([[0.,0.,0.],[1.,0.,0.],[0.,1.,0.]],[[0,1,2]]);print(out.hex(),hex(cpu.reg_read(UC_X86_REG_RIP)))
 except Exception as e:print(type(e).__name__,str(e),'rip',hex(cpu.reg_read(UC_X86_REG_RIP)),last[-12:])
