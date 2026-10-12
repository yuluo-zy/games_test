"""Actual final platform104B registry segment→WallHoles::add source fields."""
import struct
from native_platform_stair_segments import call
from native_segmentation import *
from native_platform_segmentation import call
state_hole={}
def stop_hole(c,pc,size,user):
 if state_hole.get('active') and pc==0x140777849:state_hole['end']=pc;c.emu_stop()
cpu.hook_add(UC_HOOK_CODE,stop_hole)
def produce(raw_curve,segment_positions,maximum):
 allocation[0]=heap;cpu.mem_write(data,bytes(0x100000));cpu.mem_write(stack,bytes(0x200000));rp=data+0x1000;cpu.mem_write(rp,b''.join(struct.pack('<2f',*p)for p in raw_curve));v=data+0x1100;cpu.mem_write(v,struct.pack('<3Q',len(raw_curve),rp,len(raw_curve)));call(0x1411f9920,[(UC_X86_REG_RCX,data+0x1200),(UC_X86_REG_RDX,v),(UC_X86_REG_R8,0)]);cpu.mem_write(data,bytes(cpu.mem_read(data+0x1200,56)));cpu.mem_write(data+0x108,b'\1');wf(data+0x10c,maximum)
 seg=data+0x2000;pp=data+0x3000;n=len(segment_positions);cpu.mem_write(pp,b''.join(struct.pack('<3f',*p)for p in segment_positions));cpu.mem_write(seg+0x18,struct.pack('<3Q',n,pp,n));frame=stack+0x100080;sp=stack+0x100000;cpu.mem_write(frame+0x378,struct.pack('<Q',data+0x9000))
 for reg,val in [(UC_X86_REG_RBP,frame),(UC_X86_REG_RSP,sp),(UC_X86_REG_R12,data),(UC_X86_REG_R14,data+0x2a0),(UC_X86_REG_R13,seg),(UC_X86_REG_RBX,frame+0x220),(UC_X86_REG_XMM6,bits(1.)),(UC_X86_REG_XMM11,0),(UC_X86_REG_XMM12,bits(.4)),(UC_X86_REG_XMM13,bits(.5)),(UC_X86_REG_MXCSR,0x1f80)]:cpu.reg_write(reg,val)
 state_hole.clear();state_hole['active']=True
 try:cpu.emu_start(0x1407776b0,stop,count=500000)
 finally:state_hole['active']=False
 assert state_hole.get('end')==0x140777849,hex(cpu.reg_read(UC_X86_REG_RIP))
 return bytes(cpu.mem_read(frame+0x220,16))+bytes([cpu.reg_read(UC_X86_REG_R9)&255])
if __name__=='__main__':print(produce([[0.,0.],[0.,2.],[3.,2.],[3.,0.],[0.,0.]],[[0.,0.,0.],[0.,0.,1.]],2.5).hex())
