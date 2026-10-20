from pathlib import Path
import pefile,struct,json
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
r=Path('D:/game/reverse-engineering/tiny-glade');ev=r/'progress-20261020/phase2-toolbar-layout';pe=pefile.PE(str(r/'workspace/input/tiny-glade.exe'),fast_load=True);im=pe.get_memory_mapped_image();ib=pe.OPTIONAL_HEADER.ImageBase;all_results=[]
for start in [0x140fee0e0,0x140fed8a0,0x140fef7c0,0x140ff0be0]:
 u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(ib,(len(im)+4095)&~4095);u.mem_write(ib,im);u.mem_map(0x100000,0x2000000);heap=[0x300000];allocs={};records=[]
 def q(a):return struct.unpack('<Q',u.mem_read(a,8))[0]
 def ret(v=0):
  sp=u.reg_read(UC_X86_REG_RSP);u.reg_write(UC_X86_REG_RAX,v);u.reg_write(UC_X86_REG_RIP,q(sp));u.reg_write(UC_X86_REG_RSP,sp+8)
 def hook(u,a,size,user):
  if a==0x140613c10:
   n=u.reg_read(UC_X86_REG_RCX);p=heap[0];heap[0]+=(n+31)&~31;allocs[p]=n;ret(p)
  elif a==0x142923c00:
   dst=u.reg_read(UC_X86_REG_RCX);src=u.reg_read(UC_X86_REG_RDX);n=u.reg_read(UC_X86_REG_R8);u.mem_write(dst,bytes(u.mem_read(src,n)));ret(dst)
  elif a==0x140ce4b10:
   obj=u.reg_read(UC_X86_REG_RDX);n=allocs.get(obj,0);names=[]
   if n:
    b=bytes(u.mem_read(obj,n))
    for j in range(0,len(b)-16,8):
     ptr,ln=struct.unpack_from('<QQ',b,j)
     if ib<=ptr<ib+len(im) and 4<ln<200:
      val=bytes(u.mem_read(ptr,ln))
      if val.startswith((b'toolbar::',b'tg_text::')):names.append({'offset':hex(j),'name':val.decode()})
   records.append({'callsite':hex(q(u.reg_read(UC_X86_REG_RSP))-5),'box':hex(obj),'box_size':n,'vtable':hex(u.reg_read(UC_X86_REG_R8)),'vtable_slots':[hex(q(u.reg_read(UC_X86_REG_R8)+z*8)) for z in range(22)],'names':names})
   u.mem_write(u.reg_read(UC_X86_REG_RCX),bytes(112));ret()
  elif a==0x140fd4720:u.emu_stop()
 u.hook_add(UC_HOOK_CODE,hook);u.reg_write(UC_X86_REG_RSP,0x1ffff8);u.reg_write(UC_X86_REG_RCX,0x200000);u.mem_write(0x1ffff8,struct.pack('<Q',0x100000));u.emu_start(start,0x100000,count=50000)
 for rec in records:
  obj=int(rec['box'],16);vf=int(rec['vtable_slots'][3],16)
  u.mem_write(0x1ffff8,struct.pack('<Q',0x100000));u.reg_write(UC_X86_REG_RSP,0x1ffff8);u.reg_write(UC_X86_REG_RCX,0x280000);u.reg_write(UC_X86_REG_RDX,obj);u.emu_start(vf,0x100000,count=1000)
  rec['canonical_name']=bytes(u.mem_read(q(0x280008),q(0x280010))).decode()
 all_results.append({'chain':hex(start),'records':records});print(hex(start),len(records),[x['canonical_name'] for x in records])
(ev/'native-schedule-config-names.json').write_text(json.dumps(all_results,indent=2))
