"""Unaccepted resolved single80B original row/terrain/getter oracle; sourcefreeze research."""
import struct
from native_platform_segmentation import call
from native_fixture import *
from native_platform_segmentation import call
stopped={}
def stop_filter(c,pc,size,user):
 if stopped.get('active') and pc in [0x140774ed9,0x140774edc,0x1408394cb]:
  stopped['end']=pc;c.emu_stop()
cpu.hook_add(UC_HOOK_CODE,stop_filter)

def evaluate(raw_curve,record,encoded,minimum,maximum,sampling_fix):
 allocation[0]=heap;cpu.mem_write(data,bytes(0x100000));cpu.mem_write(stack,bytes(0x200000));rp=data+0x1000;cpu.mem_write(rp,b''.join(struct.pack('<2f',*p)for p in raw_curve));v=data+0x1100;cpu.mem_write(v,struct.pack('<3Q',len(raw_curve),rp,len(raw_curve)))
 call(0x1411f9920,[(UC_X86_REG_RCX,data+0x1200),(UC_X86_REG_RDX,v),(UC_X86_REG_R8,0)]);cpu.mem_write(data,bytes(cpu.mem_read(data+0x1200,56)))
 tp=alloc(len(encoded)*2);cpu.mem_write(tp,struct.pack('<'+'H'*len(encoded),*encoded));terrain=data+0x2000;cpu.mem_write(terrain,struct.pack('<3Q',len(encoded),tp,len(encoded)));cpu.mem_write(terrain+0x50,struct.pack('<2f',minimum,maximum));cpu.mem_write(0x14362c308,struct.pack('<Q',data+0x2100));cpu.mem_write(data+0x2100,bytes([sampling_fix]))
 rec=data+0xa000;cpu.mem_write(rec,bytes(record));frame=stack+0x100080;sp=stack+0x100000;cpu.mem_write(frame+0x388,struct.pack('<Q',data));cpu.mem_write(frame+0x378,struct.pack('<Q',terrain));cpu.mem_write(frame+0x500,struct.pack('<3Q',4,data+0xb000,0))
 cpu.reg_write(UC_X86_REG_RBP,frame);cpu.reg_write(UC_X86_REG_RSP,sp);cpu.reg_write(UC_X86_REG_RSI,rec+80);cpu.reg_write(UC_X86_REG_RCX,0);cpu.reg_write(UC_X86_REG_R13,0);cpu.reg_write(UC_X86_REG_MXCSR,0x1f80);stopped.clear();stopped['active']=True
 try:cpu.emu_start(0x140774f3d,stop,count=500000)
 finally:stopped['active']=False
 assert stopped.get('end')in [0x140774ed9,0x140774edc,0x1408394cb],hex(cpu.reg_read(UC_X86_REG_RIP))
 if stopped['end']==0x1408394cb:return {'invalid_extra':True}
 n=u64(frame+0x510);return {'selected':n,'payload':bytes(cpu.mem_read(data+0xb000,n*12)).hex()}
if __name__=='__main__':
 r=bytearray(80);r[0x4e]=63;r[0x2a]=0;r[0x49]=0;struct.pack_into('<2f',r,8,2.,1.);struct.pack_into('<f',r,0x3c,1.2)
 print(evaluate([[0.,0.],[0.,2.],[3.,2.],[3.,0.],[0.,0.]],r,[0]*262144,0.,5.,True))
