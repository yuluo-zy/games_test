"""Original system business block: source negative-Y ratio -> posts/door branch.
Hash lookup/reader/resource acquisition remain the surrounding system boundary.
No candidate math or game-rule hooks; actual Nani append instructions execute.
"""
from native_gates_research import *
import native_gates_research as gates

def project_system_business(segment):
 view=setup(segment,segment['positions'],segment['us'],0.,12345,3,5,0.,0.)
 sp=stack+0x100000;bp=sp+0x80;vec=data+0x90000;ptr=alloc(8192);cpu.mem_write(vec,struct.pack('<3Q',1024,ptr,0));cpu.mem_write(bp+0x100,struct.pack('<Q',data+0x6000))
 for r,v in[(UC_X86_REG_RSP,sp),(UC_X86_REG_RBP,bp),(UC_X86_REG_R13,view['seg']),(UC_X86_REG_RAX,0),(UC_X86_REG_R15,vec+24)]:cpu.reg_write(r,v)
 for r,v in[(UC_X86_REG_XMM7,.5),(UC_X86_REG_XMM8,-.2),(UC_X86_REG_XMM9,1.),(UC_X86_REG_XMM10,.2),(UC_X86_REG_XMM11,0.)]:cpu.reg_write(r,bits(v))
 edges=[0x140756d14,0x140756e20]
 def stop_edge(c,pc,size,user):
  if pc in edges:c.emu_stop()
 h=cpu.hook_add(UC_HOOK_CODE,stop_edge)
 try:cpu.emu_start(0x140756970,0,count=30000000)
 finally:cpu.hook_del(h);state['active']=False
 edge=cpu.reg_read(UC_X86_REG_RIP);assert edge in edges,'original business block did not reach either actual exit'
 cap,p,n=struct.unpack('<3Q',cpu.mem_read(state['queue'],24));queue=bytes(cpu.mem_read(p,n));assert n%136==0
 nanis=[(queue[i+8:i+48]+queue[i+56:i+60]+queue[i+64:i+73]).hex()for i in range(0,n,136)]
 return dict(branch='wooden'if edge==0x140756d14 else'posts',actual_edge=hex(edge),nanis=nanis,entity_count=struct.unpack('<Q',cpu.mem_read(vec+16,8))[0])
if __name__=='__main__':
 s=dict(positions=[[0.,-1.,0.],[1.,-1.,0.],[2.,-1.,0.]],heights=[2.,2.,2.],us=[0.,.5,1.]);print(project_system_business(s))
