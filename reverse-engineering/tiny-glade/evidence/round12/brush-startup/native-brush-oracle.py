"""Execute unchanged entire B spawn body; bridge asset I/O and Bevy queue allocation only.
Meshes are opaque I/O receipts, not invented geometry; actual source tests load real assets.
Captured white-color calls, registrations, sphere args and all deferred spawn bytes come
from the native game body. No spawn/kind/mesh/suffix logic is implemented in this oracle.
"""
from pathlib import Path
import json,hashlib,struct
import pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
ROOT=Path('D:/game');OUT=Path(__file__).parent
pe=pefile.PE(str(ROOT/'ljxsj_92385/Tiny Glade/tiny-glade.rne'));BASE=pe.OPTIONAL_HEADER.ImageBase
procs={}
for l in (ROOT/'reverse-engineering/tiny-glade/evidence/country-core-audit/data/procs.tsv').read_text(encoding='utf8').splitlines():
 c=l.split('\t')
 try:a=int(c[0],16)
 except:continue
 procs.setdefault(a,[]).append(c[-1])
def run(fail_mesh=None,missing_shader=None):
 u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(BASE,(pe.OPTIONAL_HEADER.SizeOfImage+4095)&~4095)
 for s in pe.sections:u.mem_write(BASE+s.VirtualAddress,s.get_data())
 MEM=0x10000000;u.mem_map(MEM,0x1000000);STACK=0x10f00000;STOP=MEM
 def q(a):return struct.unpack('<Q',u.mem_read(a,8))[0]
 def w(a,v):u.mem_write(a,struct.pack('<Q',v))
 def reg(r):return u.reg_read(r)
 def string(a,n):return bytes(u.mem_read(a,n)).decode('utf8')
 heap=MEM+0x20000;shader_lookup=0;queue=MEM+0x1000;buf=MEM+0x3000
 w(queue,0x10000);w(queue+8,buf);w(queue+16,0)
 # Genuine six indirect args: device, mesh library, mesh watch, shader library,
 # shader watch, Commands. Asset methods consume opaque library values.
 args=[MEM+0x11000+i*0x100 for i in range(6)]
 for a in args:w(a,a+0x10);w(a+0x10,a+0x20)
 rsp=STACK+0x80008;w(rsp,STOP);w(rsp+0x28,args[4]);w(rsp+0x30,args[5])
 for r,v in [(UC_X86_REG_RSP,rsp),(UC_X86_REG_RCX,args[0]),(UC_X86_REG_RDX,args[1]),(UC_X86_REG_R8,args[2]),(UC_X86_REG_R9,args[3])]:u.reg_write(r,v)
 meshes=[];registrations=[];spawns=[];shader=[];colors=[];spheres=[];error=None;last_start=None;entity=0
 def ret(value=None,value2=None):
  sp=reg(UC_X86_REG_RSP)
  if value is not None:u.reg_write(UC_X86_REG_RAX,value)
  if value2 is not None:u.reg_write(UC_X86_REG_RDX,value2)
  target=q(sp);u.reg_write(UC_X86_REG_RSP,sp+8);u.reg_write(UC_X86_REG_RIP,target)
 def flush_spawn():
  nonlocal last_start
  if last_start is None:return
  length=q(queue+16)-last_start;raw=bytes(u.mem_read(buf+last_start,length));fn=struct.unpack_from('<Q',raw)[0]
  spawns.append({'entity':entity,'command_fn':hex(fn),'command_types':procs.get(fn,[]),'bytes':raw.hex(),'command_size':length})
  last_start=None
 def mesh_result(out,path):
  meshes.append(path);u.mem_write(out,b'\0'*0x90)
  if fail_mesh==len(meshes):w(out,0x8000000000000000)
  else:w(out+0x68,len(meshes))
 def hook(uc,address,size,user):
  nonlocal heap,last_start,entity,error,shader_lookup
  rcx,rdx,r8,r9=[reg(x) for x in [UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8,UC_X86_REG_R9]]
  if address==STOP:flush_spawn();uc.emu_stop()
  elif address==0x140b3fe40:
   shader.append({'path':string(rdx,r8),'name':string(r9,q(reg(UC_X86_REG_RSP)+0x28))});ret(900)
  elif address==0x140b0eeb0:mesh_result(rcx,string(rdx,r8));ret(rcx)
  elif address==0x140b10ed0:mesh_result(rcx,string(q(rdx+8),q(rdx+16)));ret(rcx)
  elif address==0x140a07680:
   colors.append({'mesh_ordinal':q(rdx+0x68),'color_bits':list(struct.unpack('<III',u.mem_read(r8,12)))});u.mem_write(rcx,bytes(u.mem_read(rdx,0x78)));ret(rcx)
  elif address==0x140b1aca0:
   registrations.append({'mesh_ordinal':q(rcx+0x68),'name':string(rdx,r8),'handle':1000+len(registrations)});ret(registrations[-1]['handle'])
  elif address==0x140d79f90:
   pieces,n,args,nargs=[q(rdx+i*8) for i in range(4)];assert n==2 and nargs==1
   arg=q(args);value=string(q(arg),q(arg+8));rendered=string(q(pieces),q(pieces+8))+value+string(q(pieces+16),q(pieces+24));data=rendered.encode();u.mem_write(heap,data);w(rcx,len(data));w(rcx+8,heap);w(rcx+16,len(data));heap+=0x200;ret(rcx)
  elif address==0x140937aa0:
   shader_lookup+=1;ret(0 if missing_shader==shader_lookup else 1,901)
  elif address==0x140d146a0:
   flush_spawn();entity+=1;u.mem_write(rcx,b'\0'*0x30);w(rcx+8,queue);w(rcx+32,0x100000000+entity);last_start=q(queue+16);ret(rcx)
  elif address==0x140a06b00:
   spheres.append({'subdivisions':rdx,'radius_bits':reg(UC_X86_REG_XMM2)&0xffffffff});mesh_result(rcx,'<procedural sphere>');ret(rcx)
  elif address==0x140c9d180:
   data=struct.pack('<4f3f3f',0.,0.,0.,1.,0.,0.,0.,1.,1.,1.);u.mem_write(rcx,data);ret(rcx)
  elif address in [0x140613c20,0x140d4cc30]:ret()
  elif address in [0x1428d9310,0x1428d9760]:error='missing shader' if address==0x1428d9310 else 'asset error';flush_spawn();uc.emu_stop()
  elif address==0x1428a9a60:raise RuntimeError('native queue overflow: oracle capacity too small')
 u.hook_add(UC_HOOK_CODE,hook)
 try:
  u.emu_start(0x140924b40,STOP,count=100000);flush_spawn()
 except Exception as e:raise RuntimeError(f'{e}; RIP={hex(reg(UC_X86_REG_RIP))}, meshes={len(meshes)}, spawns={len(spawns)}')
 return dict(meshes=meshes,registrations=registrations,color_calls=colors,spheres=spheres,shaders=shader,spawns=spawns,error=error)
if __name__=='__main__':
 result=run();assert len(result['spawns'])==17
 failures=[]
 for n in range(1,len(result['meshes'])+1):
  z=run(fail_mesh=n);failures.append({'mesh_ordinal':n,'error':z['error'],'spawns':len(z['spawns']),'registered':len(z['registrations'])})
 result['failure_cases']=failures;result['missing_shader_cases']=[]
 for i in range(1,4):
  z=run(missing_shader=i);result['missing_shader_cases'].append({'lookup_ordinal':i,'error':z['error'],'spawns':len(z['spawns'])})
 (OUT/'native-startup-receipt.json').write_text(json.dumps(result,indent=2),encoding='utf8')
 print({'meshes':len(result['meshes']),'spawns':len(result['spawns']),'colors':len(result['color_calls']),'failure_cases':len(failures)})
 for x in result['registrations']:print(x)
 for i,x in enumerate(result['spawns']):print(i,x['command_size'],x['bytes'][:105])
