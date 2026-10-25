"""Original B calc_path_wall_segmentation. Std allocation/profile bridges only."""
import sys,struct
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
sys.path.insert(0,str(ROOT/'evidence/building-constructor'))
from native_fixture import *

def produce(points,values,shape_tag=1,special=0,shape_fields=(.2,2.,3.)):
 allocation[0]=heap;cpu.mem_write(data,bytes(0x100000));cpu.mem_write(stack,bytes(0x200000));n=len(points)
 cpu.mem_write(data+0x88,struct.pack('<QQQ',n,data+0x2000,n));cpu.mem_write(data+0xc0,struct.pack('<QQQ',len(values),data+0x3000,len(values)));cpu.mem_write(data+0x2000,b''.join(struct.pack('<3f',*p)for p in points));cpu.mem_write(data+0x3000,struct.pack('<'+'f'*len(values),*values));cpu.mem_write(data+0x258,struct.pack('<I6f',shape_tag,1.,0.,shape_fields[0],.3,shape_fields[1],shape_fields[2]));cpu.mem_write(data+0x296,bytes([special]))
 sp=stack+0x100008;cpu.mem_write(sp,struct.pack('<Q',stop));cpu.reg_write(UC_X86_REG_RSP,sp);cpu.reg_write(UC_X86_REG_RCX,data+0x1000);cpu.reg_write(UC_X86_REG_RDX,data);cpu.reg_write(UC_X86_REG_MXCSR,0x1f80)
 try:cpu.emu_start(0x14074f7a0,stop,count=5000000)
 except Exception as e:raise RuntimeError(f'{e} rip={cpu.reg_read(UC_X86_REG_RIP):x} last={last[-12:]}')from e
 assert cpu.reg_read(UC_X86_REG_RIP)==stop
 cap,p,count=struct.unpack('<3Q',cpu.mem_read(data+0x1000,24));result=[]
 for i in range(count):
  raw=bytes(cpu.mem_read(p+i*136,136));fields=struct.unpack_from('<6Q',raw)
  flag_cap,flag_ptr,flag_len,coord_cap,coord_ptr,coord_len=fields
  result.append({'raw':raw.hex(),'flags':bytes(cpu.mem_read(flag_ptr,flag_len)).hex(),'coords':bytes(cpu.mem_read(coord_ptr,coord_len*8)).hex(),'range':raw[120:132].hex(),'length':raw[132:136].hex(),'lazy_headers':[raw[j:j+24].hex()for j in [48,72,96]]})
 return result
if __name__=='__main__':
 print(produce([[0.,0.,0.],[1.,0.,0.],[2.,0.,0.]],[0.,0.,0.]))
 print(produce([[0.,0.,0.],[1.,0.,0.],[2.,0.,0.]],[0.,.3,0.],shape_tag=2))
