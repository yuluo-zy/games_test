"""Run unchanged B ordinary roof-drag branch, after actual input/query lookup.

Emits original64B EditRoofCmd and captures desired InnerWalls height setter
arguments. The wall container setter itself is outside this numerical oracle;
there is no synthetic implementation of roof edit logic. No original bytes are
patched. This does not accept whole UI input arbitration or collision archival.
"""
from pathlib import Path
import struct,json,hashlib,random
import pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
OUT=Path(__file__).parent
raw=Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes()
SHA='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
assert hashlib.sha256(raw).hexdigest()==SHA
pe=pefile.PE(data=raw);base=pe.OPTIONAL_HEADER.ImageBase
c=Uc(UC_ARCH_X86,UC_MODE_64);c.mem_map(base,(pe.OPTIONAL_HEADER.SizeOfImage+4095)&~4095);c.mem_write(base,pe.get_memory_mapped_image())
DATA,STACK=0x20000000,0x30000000;c.mem_map(DATA,0x100000);c.mem_map(STACK,0x200000)
def q(a):return struct.unpack('<Q',c.mem_read(a,8))[0]
def putq(a,v):c.mem_write(a,struct.pack('<Q',v))
def bits(f):return struct.unpack('<I',struct.pack('<f',f))[0]
def setreg(r,v):c.reg_write(r,v)
def ret():
 s=c.reg_read(UC_X86_REG_RSP);pc=q(s);setreg(UC_X86_REG_RSP,s+8);setreg(UC_X86_REG_RIP,pc)
height_requests=[];stopped=None
def hook(cpu,pc,size,user):
 global stopped
 if pc in (0x141a1a85f,0x141a1a5f8):stopped=pc;cpu.emu_stop()
 elif pc==0x14085ad60:
  height_requests.append(dict(wall=cpu.reg_read(UC_X86_REG_RDX),height_bits=cpu.reg_read(UC_X86_REG_XMM2)&0xffffffff))
  ret()
 elif 0x1428d8000<=pc<=0x1428f1800:raise RuntimeError('original panic '+hex(pc))
c.hook_add(UC_HOOK_CODE,hook)
def event_writer(p,events,tick):
 putq(p,events);putq(p+16,tick);c.mem_write(p+28,struct.pack('<I',7))
 putq(events+0x20,8);putq(events+0x28,events+0x100);putq(events+0x30,0);putq(events+0x40,0)
def run(height,dx,dy,amount,original):
 global height_requests,stopped
 height_requests=[];stopped=None;c.mem_write(DATA,bytes(0x100000));c.mem_write(STACK,bytes(0x200000))
 for r in (UC_X86_REG_RAX,UC_X86_REG_RBX,UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_RSI,UC_X86_REG_RDI,UC_X86_REG_RBP,UC_X86_REG_R8,UC_X86_REG_R9,UC_X86_REG_R10,UC_X86_REG_R11,UC_X86_REG_R12,UC_X86_REG_R13,UC_X86_REG_R14,UC_X86_REG_R15):setreg(r,0)
 for r in range(UC_X86_REG_XMM0,UC_X86_REG_XMM15+1):setreg(r,0)
 roof=bytearray(88);struct.pack_into('<Q',roof,0,91);struct.pack_into('<f',roof,0x30,height);c.mem_write(DATA,bytes(roof))
 bp=STACK+0x100000;setreg(UC_X86_REG_RSP,bp-0x80);setreg(UC_X86_REG_RBP,bp);setreg(UC_X86_REG_RBX,DATA);setreg(UC_X86_REG_RSI,DATA+0x400)
 setreg(UC_X86_REG_MXCSR,0x1f80);setreg(UC_X86_REG_RFLAGS,0x202)
 putq(DATA+0x400,DATA+0x800);putq(DATA+0x410,DATA+0x900)
 putq(bp+0xd8,DATA+0x100);putq(bp+0xe8,DATA+0x104)
 c.mem_write(DATA+0x100,struct.pack('<2f',amount,original))
 event_writer(bp+0x10,DATA+0x2000,DATA+0x900)
 event_writer(DATA+0x1000,DATA+0x3000,DATA+0x900);putq(bp+0x1f8,DATA+0x1000)
 setreg(UC_X86_REG_XMM6,bits(dx)|(bits(dy)<<32))
 c.emu_start(0x141a1a242,0,count=100000)
 assert stopped is not None,hex(c.reg_read(UC_X86_REG_RIP))
 n=q(DATA+0x3030);hint=q(DATA+0x2030)
 return dict(event_hex=bytes(c.mem_read(DATA+0x3100,64)).hex() if n else None,
             hint=[c.mem_read(DATA+0x2108+i*16,1)[0] for i in range(hint)],
             amount_bits=struct.unpack('<I',c.mem_read(DATA+0x100,4))[0],height_requests=height_requests,
             stop=hex(stopped))
if __name__=='__main__':
 cases=[]
 for h in [.05,.1,.10000001,.5,float('nan')]:
  for d in [(0.,0.),(10.,1.),(-10.,1.),(1.,10.),(1.,-10.),(10.,10.),(float('nan'),1.),(float('inf'),1.)]:
   for orig in [.5,1.6]:
    args=(h,*d,.19,orig);r=run(*args);cases.append(dict(input_bits=[bits(x) for x in args],**r))
 for amount in [0.,.195,.2,.20000002]:
  args=(.5,1.,0.,amount,.5);cases.append(dict(input_bits=[bits(x) for x in args],**run(*args)))
 for args in [(0.05,0.,-100.,0.,0.5),(0.5,0.,1000.,0.,0.5),(.5,1.,0.,float('nan'),.5)]:
  cases.append(dict(input_bits=[bits(x) for x in args],**run(*args)))
 table=0x142eaf5f8;off=pe.get_offset_from_rva(table-base)
 jump=[hex(table+struct.unpack_from('<i',raw,off+4*i)[0]) for i in range(4)]
 report=dict(source_sha256=SHA,entry='141a1a242',stops=['141a1a85f','141a1a5f8'],
   excluded=['input reader and ECS query lookup','InnerWalls container setter (captures actual call arguments only)'],
   executed=['all original arithmetic and branches in ordinary drag body','original Roof::ty','original EventWriter and event payload stores'],
   phase_switch=dict(table=hex(table),targets=jump,index='min((phase-2) as u8,3)'),cases=cases)
 (OUT/'native-drag-fixtures.json').write_text(json.dumps(report,indent=2),encoding='utf8')
 print('cases',len(cases),'events',sum(x['event_hex'] is not None for x in cases),'height requests',sum(len(x['height_requests']) for x in cases),'switch',jump)
