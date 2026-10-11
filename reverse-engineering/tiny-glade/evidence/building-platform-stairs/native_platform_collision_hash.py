"""Original new_indexed hash prefix: no private hash output bridge."""
import sys,struct
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
sys.path.insert(0,str(ROOT/'evidence/building-constructor'))
from native_fixture import *
state_hash={}
def capture(c,pc,size,user):
 if state_hash.get('active')and pc==0x1408cef4e:
  state_hash['value']=c.reg_read(UC_X86_REG_RDI);c.emu_stop()
cpu.hook_add(UC_HOOK_CODE,capture)
def original_hash(points,triangles,aux_triangles=None,aux_lines=None):
 allocation[0]=heap;cpu.mem_write(data,bytes(0x10000));cpu.mem_write(stack,bytes(0x200000))
 for at,arr,payload in [(0x100,triangles,b''.join(struct.pack('<3I',*p)for p in triangles)),(0x200,points,b''.join(struct.pack('<3I',*p)for p in points)),(0x300,aux_triangles,b''if aux_triangles is None else b''.join(struct.pack('<3I',*p)for p in aux_triangles)),(0x400,aux_lines,b''if aux_lines is None else b''.join(struct.pack('<2I',*p)for p in aux_lines))]:
  if arr is None:cpu.mem_write(data+at,struct.pack('<3Q',0x8000000000000000,4,0))
  else:
   ptr=alloc(len(payload));cpu.mem_write(ptr,payload);cpu.mem_write(data+at,struct.pack('<3Q',len(arr),ptr,len(arr)))
 sp=stack+0x100008;cpu.mem_write(sp,struct.pack('<Q',stop));cpu.mem_write(sp+0x28,struct.pack('<Q',data+0x400))
 for r,v in [(UC_X86_REG_RSP,sp),(UC_X86_REG_RCX,data),(UC_X86_REG_RDX,data+0x100),(UC_X86_REG_R8,data+0x200),(UC_X86_REG_R9,data+0x300)]:cpu.reg_write(r,v)
 state_hash.clear();state_hash['active']=True
 try:cpu.emu_start(0x1408cee40,stop,count=2000000)
 finally:state_hash['active']=False
 assert 'value'in state_hash,hex(cpu.reg_read(UC_X86_REG_RIP))
 return state_hash['value']
if __name__=='__main__':print(hex(original_hash([[0,0,0],[0x3f800000,0,0],[0,0x3f800000,0]],[[0,1,2]])))
