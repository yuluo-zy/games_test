from pathlib import Path
import pefile,struct,json,hashlib
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64
from unicorn.x86_const import *
H=Path(__file__).resolve().parent;R=H.parents[3];binary=(R/'reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe').read_bytes();assert hashlib.sha256(binary).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
p=pefile.PE(data=binary,fast_load=True);img=p.get_memory_mapped_image();base=p.OPTIONAL_HEADER.ImageBase
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(img)+4095)&~4095);u.mem_write(base,img);u.mem_map(0x100000,0x1000000)
def q(a,v):u.mem_write(a,struct.pack('<Q',v))
def getq(a):return struct.unpack('<Q',u.mem_read(a,8))[0]
cases=[{'mask':mask,'photo':photo,'entity':(2<<32)|(i+1)} for i,(mask,photo) in enumerate((m,t) for m in range(32) for t in range(18))]
cases+=[{'mask':0,'photo':0,'entity':(3<<32)|1,'stale':True},{'mask':0,'photo':0,'entity':(2<<32)|100000,'stale':True}]
q(0x130008,0x140000);q(0x130010,len(cases)-1);q(0x1301a8,0x300000);q(0x350000,0xffffffffffffffff)
for i,c in enumerate(cases[:576],1):
 u.mem_write(0x140000+i*20,struct.pack('<5I',2,i,0,i,0));a=0x300000+i*72
 q(a+0x18,0x320000+i*64);q(a+0x38,0x350000);q(0x320000+i*64+0x10,0x340000+i*8);u.mem_write(0x340000+i*8,bytes([c['photo']]))
for j in range(5):
 state=0x160000+j*0x1000;bits=0x170000+j*0x1000;wrapper=0x180000+j*0x100
 q(wrapper,state);q(wrapper+8,0x130000);q(state+0x28,bits);q(state+0x38,577);q(state+0x110,0)
 for i,c in enumerate(cases[:576],1):
  if c['mask']&(1<<j):q(bits+(i//64)*8,getq(bits+(i//64)*8)|(1<<(i%64)))
# All three readers exercise both native buffers and their independent cursor.
def reader(wrapper,events,cursor,oldbuf,newbuf,records,stride):
 mid=len(records)//2;first=records[:mid];second=records[mid:]
 q(wrapper,cursor);q(wrapper+8,events);q(cursor,0)
 for off,buf,part,start in [(0,oldbuf,first,0),(32,newbuf,second,mid)]:
  q(events+off,len(part));q(events+off+8,buf);q(events+off+16,len(part));q(events+off+24,start)
  for i,record in enumerate(part):u.mem_write(buf+i*stride,record)
 q(events+64,len(records))
focus=[struct.pack('<QQ',i,c['entity']) for i,c in enumerate(cases)];click=focus
states=list(range(256));drag=[struct.pack('<QQB7x',i,c['entity'],state) for i,(c,state) in enumerate((c,st) for c in cases for st in states)]
# Large drag buffers use separate mapped storage; other streams remain small.
reader(0x110000,0x120000,0x150000,0x200000,0x210000,focus,16)
reader(0x110100,0x120100,0x150100,0x220000,0x230000,click,16)
reader(0x110200,0x120200,0x150200,0x500000,0x700000,drag,24)
q(0x110300,0x120300);q(0x110310,0x150300);q(0x120320,200000);q(0x120328,0x900000);q(0x120330,0);q(0x120340,99)
bp=0x1f8000;q(bp-8,0x110200);q(bp+0x20,0x110100);u.mem_write(bp+0x6f,b'\0')
for off,j in [(0xe0,0),(0xe8,1),(0xf0,2),(0xf8,3),(0x100,4)]:q(bp+off,0x180000+j*0x100)
for reg,v in [(UC_X86_REG_RBP,bp),(UC_X86_REG_RSP,bp-128),(UC_X86_REG_RSI,0x110000),(UC_X86_REG_R12,0x110300),(UC_X86_REG_RAX,0)]:u.reg_write(reg,v)
u.emu_start(0x14233aa60,0x14233b271,count=30000000);assert u.reg_read(UC_X86_REG_RIP)==0x14233b271
n=getq(0x120330);sounds=[struct.unpack('<I',u.mem_read(0x900000+i*32,4))[0] for i in range(n)]
(H/'native-sounds-cases.json').write_text(json.dumps(cases,indent=2));(H/'native-sounds-output.bin').write_bytes(struct.pack('<'+'I'*n,*sounds))
result={'cases':len(cases),'focus_events':len(focus),'click_events':len(click),'drag_events':len(drag),'sound_events':n,'read_cursors':[getq(0x150000),getq(0x150100),getq(0x150200)],'range':['14233aa60','14233b271'],'hooks':0,'excluded':'tracing span enter/exit only; no business instructions excluded','binary_sha256':hashlib.sha256(binary).hexdigest(),'output_sha256':hashlib.sha256((H/'native-sounds-output.bin').read_bytes()).hexdigest()};(H/'native-sounds-manifest.json').write_text(json.dumps(result,indent=2));print(json.dumps(result,indent=2))
