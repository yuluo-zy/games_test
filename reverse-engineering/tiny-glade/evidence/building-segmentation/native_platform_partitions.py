"""Actual creator partition/getCoord code, before its cell geometry body."""
import struct
from native_platform_stair_segments import call
from native_platform_geometry import *
state_part={}
def stop_part(c,pc,size,user):
 if state_part.get('active') and pc in [0x140775745,0x140775920]:state_part['end']=pc;c.emu_stop()
cpu.hook_add(UC_HOOK_CODE,stop_part)
def partitions(segment_positions,segment_us,prepared_positions,prepared_us):
 allocation[0]=heap;cpu.mem_write(data,bytes(0x100000));cpu.mem_write(stack,bytes(0x200000));n=len(segment_positions);seg=data+0x1000;pp=data+0x2000;up=data+0x3000
 cpu.mem_write(pp,b''.join(struct.pack('<3f',*p)for p in segment_positions));cpu.mem_write(up,struct.pack('<'+'f'*n,*segment_us));cpu.mem_write(seg+0x18,struct.pack('<3Q',n,pp,n));cpu.mem_write(seg+0x30,struct.pack('<3Q',n,up,n))
 curve=data+0x4000;gp=data+0x5000;gu=data+0x6000;gn=len(prepared_positions);cpu.mem_write(gp,b''.join(struct.pack('<3f',*p)for p in prepared_positions));cpu.mem_write(gu,struct.pack('<'+'f'*gn,*prepared_us));cpu.mem_write(curve,struct.pack('<6Q',gn,gp,gn,gn,gu,gn))
 # Actual length leaf first; parent creator skips <.5 before this body.
 sp=stack+0x100000;cpu.mem_write(sp,struct.pack('<Q',stop));cpu.reg_write(UC_X86_REG_RSP,sp);cpu.reg_write(UC_X86_REG_RCX,seg);cpu.emu_start(0x141234980,stop,count=100000);length=fl(cpu.reg_read(UC_X86_REG_XMM0))
 if length<.5:return [],length
 frame=stack+0x100080;cpu.mem_write(frame+0x1b0,struct.pack('<Q',curve));cpu.mem_write(frame+0x544,bytes(4))
 for reg,val in [(UC_X86_REG_RBP,frame),(UC_X86_REG_RSP,sp),(UC_X86_REG_R12,seg),(UC_X86_REG_R13,frame+0x220),(UC_X86_REG_XMM13,bits(.5)),(UC_X86_REG_XMM15,0),(UC_X86_REG_XMM12,bits(1.)),(UC_X86_REG_MXCSR,0x1f80)]:cpu.reg_write(reg,val)
 state_part.clear();state_part['active']=True
 try:cpu.emu_start(0x1407755b5,stop,count=1000000)
 finally:state_part['active']=False
 assert state_part.get('end')==0x140775745
 bp=u64(frame+0x130);bn=u64(frame+0x138);result=[]
 # Preserve original prepared header and actual iterator/RNG result. Execute
 # original endpoint interpolation/modulo/get_coord in each boundary pair.
 cpu.mem_write(frame+0x390,bytes(8));cpu.reg_write(UC_X86_REG_RDI,0);cpu.reg_write(UC_X86_REG_RSI,0)
 for j in range(bn-1):
  cpu.reg_write(UC_X86_REG_RDX,bp+j*4);cpu.reg_write(UC_X86_REG_RBX,bp+(j+1)*4);cpu.reg_write(UC_X86_REG_RAX,bp+(j+2)*4);cpu.reg_write(UC_X86_REG_RCX,0)
  cpu.reg_write(UC_X86_REG_XMM6,int.from_bytes(cpu.mem_read(frame+0x110,16),'little'));cpu.reg_write(UC_X86_REG_XMM7,int.from_bytes(cpu.mem_read(frame+0x53c,4),'little'));cpu.reg_write(UC_X86_REG_XMM9,int.from_bytes(cpu.mem_read(frame+0x540,4),'little'));cpu.reg_write(UC_X86_REG_XMM12,bits(1.));cpu.reg_write(UC_X86_REG_XMM15,0);cpu.reg_write(UC_X86_REG_RSP,sp)
  state_part.clear();state_part['active']=True
  try:cpu.emu_start(0x14077579a,stop,count=100000)
  finally:state_part['active']=False
  assert state_part.get('end')==0x140775920
  result.append([(cpu.reg_read(UC_X86_REG_RBX),struct.unpack('<f',cpu.mem_read(frame+0x5a0,4))[0]),(cpu.reg_read(UC_X86_REG_R12),struct.unpack('<f',cpu.mem_read(frame+0x4f0,4))[0])])
 return result,length
if __name__=='__main__':print(partitions([[0.,0.,0.],[4.,0.,0.]],[0.,.5],[[0.,0.,0.],[4.,0.,0.],[8.,0.,0.]],[0.,.5,1.]))
