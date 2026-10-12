"""Original creator cell geometry after actual Curve3 endpoint coordinates.
Resolved original coordinates are inputs, not injected geometry output.
"""
import struct
from native_segmentation import *
state={}
def stop_cell(c,pc,size,user):
 if state.get('active') and pc==0x140775ff2: state['end']=pc;c.emu_stop()
cpu.hook_add(UC_HOOK_CODE,stop_cell)
def produce(points,coords,maximum,extra,encoded,minimum,upper,sampling_fix,index_offset=0):
 allocation[0]=heap;cpu.mem_write(data,bytes(0x100000));cpu.mem_write(stack,bytes(0x200000))
 pp=data+0x1000;cpu.mem_write(pp,b''.join(struct.pack('<3f',*p)for p in points));cpu.mem_write(data+0x90,struct.pack('<2Q',pp,len(points)));cpu.mem_write(data+0x108,b'\1');wf(data+0x10c,maximum);wf(data+0x284,extra)
 tp=alloc(len(encoded)*2);cpu.mem_write(tp,struct.pack('<'+'H'*len(encoded),*encoded));terrain=data+0x2000;cpu.mem_write(terrain,struct.pack('<3Q',len(encoded),tp,len(encoded)));cpu.mem_write(terrain+0x50,struct.pack('<2f',minimum,upper));cpu.mem_write(0x14362c308,struct.pack('<Q',data+0x2100));cpu.mem_write(data+0x2100,bytes([sampling_fix]))
 frame=stack+0x100080;sp=stack+0x100000;cpu.mem_write(frame+0x388,struct.pack('<Q',data));cpu.mem_write(frame+0x378,struct.pack('<Q',terrain));wf(frame+0x5a0,coords[0][1]);wf(frame+0x4f0,coords[1][1]);cpu.mem_write(frame+0x338,struct.pack('<Q',index_offset))
 for reg,val in [(UC_X86_REG_RBP,frame),(UC_X86_REG_RSP,sp),(UC_X86_REG_RDI,data+0x2a0),(UC_X86_REG_RBX,coords[0][0]),(UC_X86_REG_R12,coords[1][0]),(UC_X86_REG_R13,frame+0x220),(UC_X86_REG_MXCSR,0x1f80)]:cpu.reg_write(reg,val)
 state.clear();state['active']=True
 try:cpu.emu_start(0x140775920,stop,count=1000000)
 finally:state['active']=False
 assert state.get('end')==0x140775ff2,hex(cpu.reg_read(UC_X86_REG_RIP))
 return {'top':bytes(cpu.mem_read(frame+0x548,4)).hex(),'bottom':bytes(cpu.mem_read(frame+0x550,4)).hex(),'rise_count':u64(frame+0x4e0),'depth':bytes(cpu.mem_read(frame+0x100,4)).hex(),'vertices':bytes(cpu.mem_read(frame+0x220,96)).hex(),'indices':bytes(cpu.mem_read(frame+0x280,144)).hex(),'top_corners':bytes(cpu.mem_read(frame+0x500,48)).hex(),'bottom_corners':bytes(cpu.mem_read(frame+0x430,48)).hex()}
if __name__=='__main__':
 print(produce([[0.,0.,0.],[4.,.2,0.],[4.,.2,4.]],[(0,.25),(0,.75)],2.5,.8,[0]*262144,0.,5.,True))
