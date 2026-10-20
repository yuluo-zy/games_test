from pathlib import Path
import pefile,struct,json,hashlib
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64
from unicorn.x86_const import *
root=Path('D:/game'); ev=root/'reverse-engineering/tiny-glade/progress-20261020/toolbar-layout';p=root/'reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe'
pe=pefile.PE(str(p));image=pe.get_memory_mapped_image();ib=pe.OPTIONAL_HEADER.ImageBase
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(ib,(len(image)+4095)&~4095);u.mem_write(ib,image);u.mem_map(0x100000,0x40000)
def w(a,*xs):u.mem_write(a,struct.pack('<'+'Q'*len(xs),*xs))
def q(a):return struct.unpack('<Q',u.mem_read(a,8))[0]
# Original EventReader plus Query membership structures. Empty second event buffer.
# Entities: index 1/live generation2/locked archetype3; index2/unlocked archetype4;
# index3 dead; index4 live generation2/locked archetype3.
clicked=[(2<<32)|1,(2<<32)|2,(1<<32)|1,(2<<32)|3,(2<<32)|4,(2<<32)|1]
w(0x100000,0x101000,0x102000);w(0x101000,0)
w(0x102008,0x103000,len(clicked),0);w(0x102028,8,0,len(clicked),len(clicked))
for i,e in enumerate(clicked):w(0x103000+i*16,i,e)
w(0x104000,0x105000,0x106000);w(0x105028,0x107000);w(0x105038,8);w(0x107000,1<<3)
w(0x106008,0x108000,5)
for index,generation,arch in [(1,2,3),(2,2,4),(3,2,0xffffffff),(4,2,3)]:
 u.mem_write(0x108000+index*20,struct.pack('<II',generation,arch))
w(0x109008,0x10a000);w(0x10a000,0x1000,0x10b000,0)
u.reg_write(UC_X86_REG_RCX,0x100000);u.reg_write(UC_X86_REG_RDX,0x104000);u.reg_write(UC_X86_REG_R8,0x10c000);u.reg_write(UC_X86_REG_R9,0x109000);u.reg_write(UC_X86_REG_RSP,0x130000)
u.emu_start(0x14233c8b0,0x14233caf1,count=2000)
length=q(0x10a010);data=u.mem_read(0x10b000,length);records=[]
for off in range(0,length,40):
 fn,handler,entity=struct.unpack_from('<QQQ',data,off);a,b,progress=struct.unpack_from('<fff',data,off+24)
 records.append(dict(command=hex(fn),handler=hex(handler),entity=hex(entity),a=a,b=b,progress=progress))
assert [r['entity'] for r in records]==[hex(clicked[0]),hex(clicked[4]),hex(clicked[5])]
assert all(struct.pack('<fff',r['a'],r['b'],r['progress'])==struct.pack('<fff',.1,.3,0.) for r in records)
result={'binary_sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'range':['0x14233c8b0','0x14233caf1'],'click_events':[hex(e) for e in clicked],'cursor_after':q(0x101000),'record_size':40,'commands':records,'assertions':'live generation + UiLocked membership only; event order and repeats preserved; amplitude .1 duration .3 progress0'}
(ev/'locked-click-native-receipt.json').write_text(json.dumps(result,indent=2));print(json.dumps(result,indent=2))
