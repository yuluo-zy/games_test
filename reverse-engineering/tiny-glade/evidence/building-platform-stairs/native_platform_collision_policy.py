"""Resolved get_collider pose output -> original upsert pose gate -> full mesh eq.
Library physics state acquisition is outside this bounded business contract.
"""
from native_platform_collision_hash import *
state_policy={}
def capture_policy(c,pc,size,user):
 if state_policy.get('active')and pc in [0x141245c94,0x141246026]:state_policy['pose_equal']=pc==0x141245c94;c.emu_stop()
cpu.hook_add(UC_HOOK_CODE,capture_policy)
def call(pc,regs):
 sp=stack+0x100008;cpu.mem_write(sp,struct.pack('<Q',stop));cpu.reg_write(UC_X86_REG_RSP,sp)
 for r,v in regs:cpu.reg_write(r,v)
 cpu.emu_start(pc,stop,count=200000)
 return cpu.reg_read(UC_X86_REG_RAX)
def original_mesh_eq(a,z,old_payload=0,new_payload=0):
 cpu.mem_write(data,bytes(0x10000));cpu.mem_write(data,struct.pack('<3Q',7,old_payload,0)+struct.pack('<Q',data+0x1000));cpu.mem_write(data+0x100,struct.pack('<3Q',7,new_payload,0)+struct.pack('<Q',data+0x2000));cpu.mem_write(data+0x1080,struct.pack('<Q',a));cpu.mem_write(data+0x2080,struct.pack('<Q',z))
 return bool(call(0x1408cea30,[(UC_X86_REG_RCX,data),(UC_X86_REG_RDX,data+0x100)])&255)
def original_action(exists,valid,a,z,old,new,old_wall=0,new_wall=0):
 if not exists:return 0
 cpu.mem_write(data,bytes(0x10000));cpu.mem_write(stack,bytes(0x200000));frame=stack+0x100080;cpu.mem_write(frame-0x60,struct.pack('<4I',*old[3:]));cpu.mem_write(frame-0x50,struct.pack('<3I',*old[:3]));cpu.mem_write(0x14362c008,struct.pack('<Q',data+0x3000));cpu.mem_write(data+0x3000,b'\0')
 for r,v in [(UC_X86_REG_RSP,stack+0x100000),(UC_X86_REG_RBP,frame),(UC_X86_REG_R15,int(valid)),(UC_X86_REG_XMM7,int.from_bytes(struct.pack('<4I',*new[3:]),'little')),(UC_X86_REG_XMM9,int.from_bytes(struct.pack('<2I',*new[:2]),'little')),(UC_X86_REG_XMM10,new[2])]:cpu.reg_write(r,v)
 state_policy.clear();state_policy['active']=True
 try:cpu.emu_start(0x141245ade,stop,count=200000)
 finally:state_policy['active']=False
 assert 'pose_equal'in state_policy,hex(cpu.reg_read(UC_X86_REG_RIP))
 return 2 if state_policy['pose_equal']and original_mesh_eq(a,z,old_wall,new_wall)else 1
