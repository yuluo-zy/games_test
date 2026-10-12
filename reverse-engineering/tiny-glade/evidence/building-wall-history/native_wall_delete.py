"""B resolved bulldoze producer business segment and full animate system."""
from native_wall_history import *
def animate_bulldoze(command,wall,source_pointer,present,dt,initialize=False):
 ctrl=data+0x18000
 if initialize:
  row=bytes(cpu.mem_read(source_pointer,112));cpu.mem_write(data,struct.pack('<4Q',ctrl,15,14,int(present)));cpu.mem_write(ctrl,bytes([0])+bytes([255])*15);cpu.mem_write(ctrl-0x78,struct.pack('<Q',wall)+row)
 buffer=data+0x10000;buffer_res=data+0x10100;source_res=data+0x10200;audio=data+0x11000;audio_res=data+0x11100;game=data+0x12000;game_res=data+0x12100
 for r,p in[(buffer_res,buffer),(source_res,data),(audio_res,audio),(game_res,game)]:cpu.mem_write(r,struct.pack('<QQQI',p,0,r+0x60,0)+struct.pack('<I',0x12345678))
 if initialize:cpu.mem_write(buffer,struct.pack('<3Q',1,command,1))
 cpu.mem_write(audio+0x20,struct.pack('<3Q',32,data+0x20000,0));cpu.mem_write(audio+0x40,struct.pack('<Q',17));cpu.mem_write(game+0x58,struct.pack('<f',dt))
 cpu.reg_write(UC_X86_REG_RCX,buffer_res);cpu.reg_write(UC_X86_REG_RDX,source_res);cpu.reg_write(UC_X86_REG_R8,audio_res);cpu.reg_write(UC_X86_REG_R9,game_res);author.notifications.clear();author.active=True
 try:author.call(0x140ac27e0)
 finally:author.active=False
 count=nf.u64(data+24);audio_count=nf.u64(audio+0x30)
 return {'actual_return':True,'source_count':count,'source':snapshot(ctrl-0x70)if count else None,'phase':struct.unpack('<I',cpu.mem_read(command+140,4))[0],'finished':cpu.mem_read(command+144,1)[0],'buffer_count':nf.u64(buffer+16),'notifications':list(author.notifications),'audio':bytes(cpu.mem_read(data+0x20000,audio_count*32)).hex(),'audio_count':audio_count}

def bulldoze_record_segment(source,wall,low,high,color,roof=None):
 """Actual original Wall-selected business segment through both History.add.
 Does not claim upstream UI hit selection or full wall_eraser_bulldoze entry.
 No private result callback; actual roof query and snapshot/slice code executes.
 """
 frame=stack+0x100800;history=data+0x3000;hres=data+0x3100;query=data+0x4000;pub=data+0x4200;color_res=data+0x4400;special_res=data+0x4500
 cpu.mem_write(history,bytes(0x200));cpu.mem_write(hres,struct.pack('<QQQI',history,0,hres+0x60,0)+struct.pack('<I',0x12345678));cpu.mem_write(history+0x30,struct.pack('<3Q',128,data+0x5000,0));cpu.mem_write(history+0x58,struct.pack('<QQii',1,2,-1,-1))
 author.bind_context(pub,query,data+0x4600,data+0x4700,None,roof,data+0x60000)
 if roof is not None:
  base=data+0x60000;cpu.mem_write(base+0x3118,struct.pack("<Q",1));cpu.mem_write(base+0x8040,struct.pack("<Q",2));cpu.mem_write(base+0xa008,struct.pack("<Q",0xfffffffffffffffe));cpu.mem_write(base+0x9040,struct.pack("<Q",base+0xc000))
 for wrapper,owner in[(color_res,data+0x4800),(special_res,data+0x4900)]:cpu.mem_write(wrapper,struct.pack('<Q',owner));cpu.mem_write(owner,struct.pack('<3Q',wall+1,data+0x80000,wall+1))
 cpu.mem_write(data+0x80000+wall*4,struct.pack('<I',color))
 writer=data+0x90000;owner=data+0x90100;cpu.mem_write(writer,struct.pack('<QQQI',owner,0,writer+0x60,0)+struct.pack('<I',0x12345678));cpu.mem_write(owner+0x20,struct.pack('<3Q',32,data+0x91000,0));cpu.mem_write(owner+0x40,struct.pack('<Q',17))
 for offset,value in[(0x478,wall),(0x490,source+24),(0x598,hres),(0x5c0,color_res),(0x5c8,special_res),(0xd0,query),(0x20,owner)]:cpu.mem_write(frame+offset,struct.pack('<Q',value))
 cpu.mem_write(frame+0x20,bytes(cpu.mem_read(writer,32)))
 cpu.reg_write(UC_X86_REG_RBP,frame);cpu.reg_write(UC_X86_REG_RSP,frame-0x80);cpu.reg_write(UC_X86_REG_RSI,source);cpu.reg_write(UC_X86_REG_RAX,1<<32 if roof is not None else 0);cpu.reg_write(UC_X86_REG_XMM6,nf.bits(low)|(nf.bits(high)<<32));cpu.reg_write(UC_X86_REG_XMM7,nf.bits(high));author.active=True
 try:cpu.emu_start(0x1409e1a11,0x1409df738,count=1000000);assert cpu.reg_read(UC_X86_REG_RIP)==0x1409df738
 finally:author.active=False
 count=nf.u64(history+0x40);owners=[]
 for i in range(count):
  p=data+0x5000+i*24;tag=nf.u64(p)
  owners.append({'tag':tag,'groups':compound(p)}if tag==7 else{'tag':tag,'roof_payload':bytes(cpu.mem_read(nf.u64(p+8)+16,56)).hex()})
 return {'actual_segment_stop':True,'upstream_selection':'resolved observed Wall/source/range/Roof context; full UI entry unclaimed','owners':owners,'delete_roof_events':bytes(cpu.mem_read(data+0x91000,nf.u64(owner+0x30)*24)).hex(),'owner_pointer':data+0x5000,'owner_count':count}
