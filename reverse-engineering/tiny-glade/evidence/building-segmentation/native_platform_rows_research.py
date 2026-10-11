"""Unaccepted actual cell→shape slice→rise curves→constructor→96B writer.
No private geometry/instance result bridge; IAT-pinned CRT/allocator/TLS only.
"""
import struct,ctypes,pefile
from native_platform_geometry import produce as cell_produce
from native_platform_stair_segments import *
from native_structures import create_sparse_writer
imports={x.address:x.name.decode()for d in pefile.PE(data=raw).DIRECTORY_ENTRY_IMPORT for x in d.imports if x.name}
for pc,name in [(0x142923e20,'atan2f'),(0x142923e80,'cosf'),(0x142923cf0,'_hypotf')]:
 code=bytes(cpu.mem_read(pc,6));assert code[:2]==b'\xff\x25';iat=pc+6+struct.unpack('<i',code[2:])[0];assert imports[iat]==name
for name,n in [('atan2f',2),('cosf',1),('hypotf',2)]:fn=getattr(lib,name);fn.argtypes=[ctypes.c_float]*n;fn.restype=ctypes.c_float
def math_hook(c,pc,size,user):
 if cpu.reg_read(UC_X86_REG_RIP)!=pc:return
 if pc==0x142923e80:cpu.reg_write(UC_X86_REG_XMM0,bits(lib.cosf(fl(cpu.reg_read(UC_X86_REG_XMM0)))));ret()
 elif pc==0x142923e20:cpu.reg_write(UC_X86_REG_XMM0,bits(lib.atan2f(fl(cpu.reg_read(UC_X86_REG_XMM0)),fl(cpu.reg_read(UC_X86_REG_XMM1)))));ret()
 elif pc==0x142923cf0:cpu.reg_write(UC_X86_REG_XMM0,bits(lib.hypotf(fl(cpu.reg_read(UC_X86_REG_XMM0)),fl(cpu.reg_read(UC_X86_REG_XMM1)))));ret()
cpu.hook_add(UC_HOOK_CODE,math_hook)
state_rows={}
def stop_rows(c,pc,size,user):
 if state_rows.get('active') and pc==0x140776dd0:
  sp=cpu.reg_read(UC_X86_REG_RSP);curve=cpu.reg_read(UC_X86_REG_RDX);n=u64(curve+16);pp=u64(curve+8);up=u64(curve+32);count=u64(sp+32);archptr=cpu.reg_read(UC_X86_REG_R9);secondptr=u64(sp+40);thirdptr=u64(sp+56)
  f=lambda ptr,n:list(struct.unpack('<'+'f'*n,cpu.mem_read(ptr,n*4)))
  state_rows.setdefault('calls',[]).append({'points':list(struct.iter_unpack('<2f',cpu.mem_read(pp,n*8))),'us':f(up,n),'length':f(curve+48,1)[0],'arch':f(archptr,count),'second':f(secondptr,count),'third':f(thirdptr,count),'base_height':f(sp+112,1)[0],'perturb':f(sp+96,1)[0],'wall_float':f(sp+160,1)[0],'kind':bytes(cpu.mem_read(sp+72,1))[0],'seed':u64(sp+80),'material':cpu.reg_read(UC_X86_REG_R8)&0xffffffff,'holes_length':f(u64(sp+88)+24,1)[0],'closed':bytes(cpu.mem_read(u64(sp+88)+28,1))[0]!=0})
 if state_rows.get('active') and pc==0x140776f9b:state_rows['end']=pc;c.emu_stop()
cpu.hook_add(UC_HOOK_CODE,stop_rows)
def produce(points,coords,maximum,extra,encoded,minimum,upper,sampling_fix,shape,wall_id=27,views=False):
 cell=cell_produce(points,coords,maximum,extra,encoded,minimum,upper,sampling_fix)
 frame=stack+0x100080;sp=stack+0x100000;cpu.mem_write(frame+0x140,shape);cpu.mem_write(frame+0x5c0,struct.pack('<Q',wall_id));cpu.mem_write(frame+0x5b8,struct.pack('<Q',data+0x7000))
 for at in [0x328,0x340,0x310,0x3b8,0x418,0x358]:cpu.mem_write(frame+at,struct.pack('<3Q',0,4,0))
 view=create_sparse_writer(64);records=alloc(64*32*96);cpu.mem_write(view['writer']+40,struct.pack('<Q',records));view['records']=records;cpu.mem_write(frame+0x80,bytes(cpu.mem_read(view['writer'],88)))
 cpu.reg_write(UC_X86_REG_RSP,sp);state_rows.clear();state_rows['active']=True
 try:cpu.emu_start(0x140775ff2,stop,count=10000000)
 except Exception as e:raise RuntimeError(f'{e} RIP={cpu.reg_read(UC_X86_REG_RIP):x} last={last[-15:]}')from e
 finally:state_rows['active']=False
 assert state_rows.get('end')==0x140776f9b,hex(cpu.reg_read(UC_X86_REG_RIP))
 cap,p,n=struct.unpack('<3Q',cpu.mem_read(view['owned'],24));pages=struct.unpack('<'+'I'*n,cpu.mem_read(p,n*4));output=[]
 for page in pages:
  count=bytes(cpu.mem_read(view['counts']+page,1))[0];assert count<=32
  output.extend(bytes(cpu.mem_read(records+(page*32+i)*96,96))for i in range(count))
 if views:
  cap,p,n=struct.unpack('<3Q',cpu.mem_read(frame+0x560,24));uc,up,un=struct.unpack('<3Q',cpu.mem_read(frame+0x578,24));weightptr=u64(frame+0x400);wn=u64(frame+0x1e8)
  observed={'footprint_points':list(struct.iter_unpack('<2f',cpu.mem_read(p,n*8))),'footprint_us':list(struct.unpack('<'+'f'*un,cpu.mem_read(up,un*4))),'footprint_length':struct.unpack('<f',cpu.mem_read(frame+0x590,4))[0],'weights':list(struct.unpack('<'+'f'*wn,cpu.mem_read(weightptr,wn*4))),'constructor_calls':state_rows.get('calls',[])}
  return cell,output,observed
 return cell,output
if __name__=='__main__':
 shape=struct.pack('<I6f',1,1.,0.,0.,0.,6.,4.)
 cell,rows=produce([[-3.,0.,-2.],[-3.,0.,2.],[3.,0.,2.],[3.,0.,-2.],[-3.,0.,-2.]],[(0,0.),(0,1.)],2.5,.8,[0]*262144,0.,5.,True,shape)
 print(len(rows),rows[0].hex()if rows else 'empty')
