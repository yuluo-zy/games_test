"""Rectangle enclosure ABI: original leaf to RET, including finished instant skips."""
from native_wall_history import *

def instant_enclosure_process(arc,undo,source_pointer,wall,present,elapsed,dt,duration,finished):
 """Complete instant process restores/mutates/removes actual authority -> RET."""
 ctrl=data+0x18000;row=bytes(cpu.mem_read(source_pointer,112));cpu.mem_write(data,struct.pack('<4Q',ctrl,15,14,int(present)));cpu.mem_write(ctrl,bytes([0])+bytes([255])*15);cpu.mem_write(ctrl-0x78,struct.pack('<Q',wall)+row);cpu.mem_write(data+0xc0,struct.pack('<Q',100))
 state=data+0x10000;cpu.mem_write(state,struct.pack('<IfQfBB2x',0,duration,arc,elapsed,int(undo),int(finished)))
 writers=[];owners=[]
 for i in range(4):
  writer=data+0x11000+i*0x100;owner=data+0x12000+i*0x100
  cpu.mem_write(writer,struct.pack('<QQQI',owner,0,writer+0x60,0)+struct.pack('<I',0x12345678));cpu.mem_write(owner+0x20,struct.pack('<3Q',32,data+0x20000+i*0x2000,0));cpu.mem_write(owner+0x40,struct.pack('<Q',17));writers.append(writer);owners.append(owner)
 terrain=data+0x14000;terrain_res=data+0x14100;cpu.mem_write(terrain_res,struct.pack('<Q',terrain));cpu.mem_write(terrain+8,struct.pack('<Q',data+0x40000));cpu.mem_write(data+0x40000,bytes(512*512*2));cpu.mem_write(terrain+0x50,struct.pack('<2f',.75,.75));cpu.mem_write(0x14362c308,struct.pack('<Q',data+0x14200))
 sp=stack+0x100008;cpu.mem_write(sp,struct.pack('<Q',stop))
 for offset,ptr in[(0x28,writers[1]),(0x30,writers[2]),(0x38,terrain_res),(0x40,writers[3])]:cpu.mem_write(sp+offset,struct.pack('<Q',ptr))
 cpu.reg_write(UC_X86_REG_RSP,sp);cpu.reg_write(UC_X86_REG_RCX,state);cpu.reg_write(UC_X86_REG_XMM1,nf.bits(dt));cpu.reg_write(UC_X86_REG_R8,data);cpu.reg_write(UC_X86_REG_R9,writers[0]);author.notifications.clear();author.inserted.clear();author.active=True
 try:cpu.emu_start(0x140abfb30,stop,count=1000000);assert cpu.reg_read(UC_X86_REG_RIP)==stop
 finally:author.active=False
 result={'elapsed_bits':bytes(cpu.mem_read(state+16,4)).hex(),'actual_return':True,'finished':cpu.mem_read(state+21,1)[0],'source_count':nf.u64(data+24),'counter':nf.u64(data+0xc0),'notifications':list(author.notifications),'events':[]}
 if result['source_count']:result['source']=snapshot(ctrl-0x70)
 for i,owner in enumerate(owners):
  n=nf.u64(owner+0x30);result['events'].append({'slot':i,'count':n,'bytes':bytes(cpu.mem_read(nf.u64(owner+0x28),n*[48,24,24,32][i])).hex()})
 return result
