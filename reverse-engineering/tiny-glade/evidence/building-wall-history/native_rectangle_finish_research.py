"""Original complete rectangle finish research; no Rust candidate acceptance."""
import struct,sys,ctypes,json,hashlib
from pathlib import Path
sys.path.insert(0,str(Path(__file__).resolve().parent))
import native_wall_history as n
from unicorn import UC_HOOK_CODE
from unicorn.x86_const import *
cpu=n.cpu;data=n.data;author=n.author;nf=n.nf
acos=nf.lib.acosf;acos.argtypes=[ctypes.c_float];acos.restype=ctypes.c_float
def math_hook(c,pc,size,user):
 if pc==0x142923de0:
  c.reg_write(UC_X86_REG_XMM0,nf.bits(acos(nf.fl(c.reg_read(UC_X86_REG_XMM0)))));nf.ret()
cpu.hook_add(UC_HOOK_CODE,math_hook)
def run(mode,old,new,finish_event=True,already_read=False,actual_source=None,prepare_archivist=None):
 author.reset();shape=struct.pack('<I6f',1,*(new if actual_source is None else actual_source));curve=author.create_geometry(shape);raw=bytes(cpu.mem_read(curve['pointer'],56))+shape+struct.pack('<3fI',3.2,.7,2.8,0)+bytes([0,0,0,0,1,5])+bytes(6)
 ctrl=data+0x18000;cpu.mem_write(ctrl,bytes([0])+bytes([255])*15);cpu.mem_write(ctrl-0x78,struct.pack('<Q',9)+raw);cpu.mem_write(data,struct.pack('<4Q',ctrl,15,14,1));source_res=data+0x2000;ui=data+0x3000;ui_res=data+0x3100;history=data+0x4000;history_res=data+0x4100
 state=bytearray(80);struct.pack_into('<I',state,0,mode)
 if mode:
  wall_offset=16 if mode==1 else 8;axis_offset=32 if mode==1 else 24
  struct.pack_into('<Q',state,wall_offset,9);struct.pack_into('<6f',state,axis_offset,*old)
  struct.pack_into('<2f',state,0x38 if mode==1 else 0x30,*new[:2]);struct.pack_into('<2f',state,0x48 if mode==1 else 0x40,*new[4:6])
 cpu.mem_write(ui,bytes(state));cpu.mem_write(history+0x30,struct.pack('<3Q',128,data+0x5000,0));cpu.mem_write(history+0x58,struct.pack('<QQii',1,2,-1,-1))
 wrappers=[];owners=[]
 for i in range(3):
  wrapper=data+0x6000+i*0x100;owner=data+0x7000+i*0x100;wrappers.append(wrapper);owners.append(owner);cpu.mem_write(wrapper,struct.pack('<QQQI',owner,0,wrapper+0x60,0)+struct.pack('<I',0x12345678));cpu.mem_write(owner+0x20,struct.pack('<3Q',32,data+0x20000+i*0x3000,0));cpu.mem_write(owner+0x40,struct.pack('<Q',17))
 for r,p in[(source_res,data),(ui_res,ui),(history_res,history)]:cpu.mem_write(r,struct.pack('<QQQI',p,0,r+0x60,0)+struct.pack('<I',0x12345678))
 reader=data+0x8000;cursor=data+0x8060;events=data+0x8100;rows=data+0x8200;count=int(finish_event);cpu.mem_write(cursor,struct.pack('<Q',count if already_read else 0));cpu.mem_write(rows,struct.pack('<Q',0));cpu.mem_write(reader,struct.pack('<QQ',cursor,events));cpu.mem_write(events,struct.pack('<9Q',0,8,0,0,count,rows,count,0,count))
 archive=data+0xa000;storage=data+0xb000;archive_res=data+0xc000;storage_res=data+0xc100;empty=data+0xc200;cpu.mem_write(empty,bytes([255])*16)
 # Real empty std HashMap headers and empty optional Decorators. Actual
 # cancellation helper still executes its clear/drain/restore/drop business.
 for o in[0,0x60,0x90]:cpu.mem_write(archive+o,struct.pack('<4Q',empty,0,0,0))
 for r,p in[(archive_res,archive),(storage_res,storage)]:cpu.mem_write(r,struct.pack('<QQQI',p,0,r+0x60,0)+struct.pack('<I',0x12345678))
 if prepare_archivist is not None:prepare_archivist(archive,storage)
 sp=n.stack+0x100008;cpu.mem_write(sp,struct.pack('<Q',n.stop))
 for o,p in[(0x28,archive_res),(0x30,reader),(0x38,wrappers[0]),(0x40,wrappers[1]),(0x48,wrappers[2])]:cpu.mem_write(sp+o,struct.pack('<Q',p))
 cpu.reg_write(UC_X86_REG_RSP,sp);cpu.reg_write(UC_X86_REG_RCX,source_res);cpu.reg_write(UC_X86_REG_RDX,ui_res);cpu.reg_write(UC_X86_REG_R8,history_res);cpu.reg_write(UC_X86_REG_R9,storage_res);author.active=True
 try:cpu.emu_start(0x141464a60,n.stop,count=1000000);assert cpu.reg_read(UC_X86_REG_RIP)==n.stop
 finally:author.active=False
 records=[]
 for j in range(nf.u64(history+0x40)):
  owner=data+0x5000+j*24;tag,ptr,count=struct.unpack('<3Q',cpu.mem_read(owner,24));groups=[]
  for g in range(count):
   array,number=struct.unpack('<2Q',cpu.mem_read(ptr+g*16,16));group=[]
   for k in range(number):
    arc=nf.u64(array+k*8);group.append({'niche':hex(nf.u64(arc+16)),'wall':nf.u64(arc+24),'previous':bytes(cpu.mem_read(arc+32,28)).hex(),'next':bytes(cpu.mem_read(arc+60,28)).hex()})
   groups.append(group)
  records.append({'tag':tag,'groups':groups})
 return {'actual_return':True,'ui_tag':struct.unpack('<I',cpu.mem_read(ui,4))[0],'cursor':nf.u64(cursor),'history':records,'events':[{'stride':stride,'count':nf.u64(owner+0x30),'bytes':bytes(cpu.mem_read(nf.u64(owner+0x28),nf.u64(owner+0x30)*stride)).hex()}for owner,stride in zip(owners,[88,8,16])],'archivist_header':bytes(cpu.mem_read(archive,32)).hex()}
if __name__=='__main__':
 import math
 f32=lambda x:struct.unpack('<f',struct.pack('<f',x))[0];cases=[]
 for mode in[1,2]:
  for event in[False,True]:
   for consumed in[False,True]:
    for delta in[0.,.005,.01,.011,.05,.2]:
     old=(1.,0.,2.,3.,4.,5.);new=(1.,0.,2.,3.,4.+delta,5.);r=run(mode,old,new,event,consumed);active=event and not consumed;changed=f32(f32(4.+delta)-4.)>f32(.01)
     assert r['actual_return']and len(r['history'])==int(active and changed)
     assert r['ui_tag']==(0 if active else mode)
     if r['history']:assert r['history'][0]['groups'][0][0]['niche']=='0x8000000000000004'
     cases.append({'mode':mode,'event':event,'consumed':consumed,'dimension_delta':delta,'native':r})
 for mode in[1,2]:
  for angle in[0.,.0001,.001,.01]:
   old=(1.,0.,2.,3.,4.,5.);new=(math.cos(angle),math.sin(angle),2.,3.,4.,5.);r=run(mode,old,new);assert r['actual_return'];cases.append({'mode':mode,'orientation_angle':angle,'native':r})
 p=Path(__file__);root=p.resolve().parents[2];sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
 report={'ok':True,'full_finish_returns':len(cases),'cases':cases,'original_binary_sha256':nf.b['binary']['sha256'],'oracle_sources':{str(p.relative_to(root)):sha(p)for p in[p,root/'evidence/building-wall-history/native_wall_history.py',root/'evidence/building-authoring/native_authoring.py',root/'evidence/building-constructor/native_fixture.py']},'scope':'Native-only complete141464a60 entry-toRET, activeUI snapshot88/finishreader/niche4owner/8B+16Bnotify/clear. Threshold.01 dimension andorientation branches/consumed/noevent. Actual DecoratorArchivist cancellation helper executed with genuinely empty std maps/options; nonempty decorator restoration not proven. No Rust source was edited during parent freeze.'}
 p.with_name('B-rectangle-finish-native-research.json').write_text(json.dumps(report,indent=2)+'\n');print({'ok':True,'full_returns':len(cases)})
