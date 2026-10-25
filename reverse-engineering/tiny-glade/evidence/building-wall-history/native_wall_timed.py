"""Full B timed WallReplayEditCmd process; real original source map/writers."""
import native_wall_history as base
from native_wall_history import *
from unicorn import UC_HOOK_CODE
system_active=False
system_sets={}
system_notifications=[]
def system_std_hook(c,pc,size,user):
 if not system_active:return
 if pc==0x1400d9b20: # std RandomState thread-local key Cell, not game output.
  nf.ret(data+0xe0000)
 elif pc==0x1405b0d70:nf.ret(0)
 elif pc==0x1402ca2f0:
  owner,key=(c.reg_read(r)for r in[UC_X86_REG_RCX,UC_X86_REG_RDX])
  if owner in[data+0x30,data+0x60,data+0x90]:system_notifications.append((owner,key))
  else:system_sets.setdefault(owner,set()).add(key)
  nf.ret(1)
 elif pc==0x1408b1fb0:
  a,b=(c.reg_read(r)for r in[UC_X86_REG_RCX,UC_X86_REG_RDX]);nf.ret(int(system_sets.get(a,set()).isdisjoint(system_sets.get(b,set()))));system_sets.pop(a,None);system_sets.pop(b,None)
 elif pc==0x141529bb0:
  result,owner,key,source=[c.reg_read(r)for r in[UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8,UC_X86_REG_R9]]
  control=data+0x18000;c.mem_write(owner,struct.pack('<4Q',control,15,14,1));c.mem_write(control,bytes([0])+bytes([255])*15);c.mem_write(control-0x78,struct.pack('<Q',key)+bytes(c.mem_read(source,112)));c.mem_write(result,struct.pack('<Q',0x8000000000000000));nf.ret(result)
 elif pc==0x140b76b50:
  result,owner=[c.reg_read(r)for r in[UC_X86_REG_RCX,UC_X86_REG_RDX]];control=nf.u64(owner);c.mem_write(result,bytes(c.mem_read(control-0x78,120)));c.mem_write(owner+24,bytes(8));c.mem_write(control,bytes([255])*16);nf.ret(result)
cpu.hook_add(UC_HOOK_CODE,system_std_hook)

def replay_wall_system(replay,undo,source_pointer,wall,present,dt,initialize=False):
 """Full real replay_wall; std set is_disjoint bridge retains actual key sets."""
 global system_active
 ctrl=data+0x18000
 if initialize:
  row=bytes(cpu.mem_read(source_pointer,112));cpu.mem_write(data,struct.pack('<4Q',ctrl,15,14,int(present)));cpu.mem_write(ctrl,bytes([0])+bytes([255])*15);cpu.mem_write(ctrl-0x78,struct.pack('<Q',wall)+row);cpu.mem_write(data+0xc0,struct.pack('<Q',100))
 resource=data+0xa000;game=data+0xa100;game_res=data+0xa200;source_res=data+0xa300
 for r,p in[(resource,replay),(game_res,game),(source_res,data)]:cpu.mem_write(r,struct.pack('<QQQI',p,0,r+0x60,0)+struct.pack('<I',0x12345678))
 cpu.mem_write(replay+0x28,struct.pack('<f',1.));cpu.mem_write(game+0x28,struct.pack('<f',dt))
 writers=[];owners=[]
 for i in range(4):
  writer=data+0x11000+i*0x100;owner=data+0x12000+i*0x100
  cpu.mem_write(writer,struct.pack('<QQQI',owner,0,writer+0x60,0)+struct.pack('<I',0x12345678));cpu.mem_write(owner+0x20,struct.pack('<3Q',32,data+0x20000+i*0x2000,0));cpu.mem_write(owner+0x40,struct.pack('<Q',17));writers.append(writer);owners.append(owner)
 terrain=data+0x14000;terrain_res=data+0x14100;cpu.mem_write(terrain_res,struct.pack('<Q',terrain));cpu.mem_write(terrain+8,struct.pack('<Q',data+0x40000));cpu.mem_write(data+0x40000,bytes(512*512*2));cpu.mem_write(terrain+0x50,struct.pack('<2f',.75,.75));cpu.mem_write(0x14362c308,struct.pack('<Q',data+0x14200))
 sp=stack+0x100008;cpu.mem_write(sp,struct.pack('<Q',stop))
 for offset,ptr in[(0x28,writers[3]),(0x30,writers[1]),(0x38,writers[2]),(0x40,terrain_res)]:cpu.mem_write(sp+offset,struct.pack('<Q',ptr))
 cpu.reg_write(UC_X86_REG_RSP,sp);cpu.reg_write(UC_X86_REG_RCX,resource);cpu.reg_write(UC_X86_REG_RDX,game_res);cpu.reg_write(UC_X86_REG_R8,source_res);cpu.reg_write(UC_X86_REG_R9,writers[0]);system_sets.clear();system_notifications.clear();system_active=True
 try:cpu.emu_start(0x140951b80,stop,count=1000000);assert cpu.reg_read(UC_X86_REG_RIP)==stop
 finally:system_active=False
 capacity,ptr,head,count=struct.unpack('<4Q',cpu.mem_read(replay,32));bucket=ptr+(head-(capacity if head>=capacity else 0))*64;command_n=nf.u64(bucket+16);commands=nf.u64(bucket+8)
 result={'actual_return':True,'bucket_count':count,'command_count':command_n,'source_count':nf.u64(data+24),'notifications':list(system_notifications),'events':[],'commands':[]}
 if result['source_count']:result['source']=snapshot(ctrl-0x70)
 for j in range(command_n):
  command=commands+j*48;tag=cpu.mem_read(command,1)[0];groups=nf.u64(command+32);result['commands'].append({'tag':tag,'group_count':groups,'head':nf.u64(command+24)})
 for i,owner in enumerate(owners):
  n=nf.u64(owner+0x30);result['events'].append({'slot':i,'count':n,'bytes':bytes(cpu.mem_read(nf.u64(owner+0x28),n*[48,24,24,32][i])).hex()})
 return result
def timed_process(arc,undo,source_pointer,wall,present,elapsed,dt,duration=.4,finished=False):
 """Complete instant process restores/mutates/removes actual authority -> RET."""
 ctrl=data+0x18000;row=bytes(cpu.mem_read(source_pointer,112));cpu.mem_write(data,struct.pack('<4Q',ctrl,15,14,int(present)));cpu.mem_write(ctrl,bytes([0])+bytes([255])*15);cpu.mem_write(ctrl-0x78,struct.pack('<Q',wall)+row);cpu.mem_write(data+0xc0,struct.pack('<Q',100))
 state=data+0x10000;cpu.mem_write(state,struct.pack('<IfQfBB2x',1,duration,arc,elapsed,int(undo),int(finished)))
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
