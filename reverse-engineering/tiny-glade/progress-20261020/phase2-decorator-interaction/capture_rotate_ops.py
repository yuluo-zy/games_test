"""Execute full original rotate control flow until apply_decorator_op boundary.

Only dependency boundaries are substituted: UI state, storage lookup, cached
transform/terrain point, cursor-plane intersection, allocator/debug output and
CRT math. No original branch or request assembly is replaced.
"""
from pathlib import Path
import hashlib,json,math,struct,itertools
import pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
p=Path(__file__).parent
raw=Path('D:/game/reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe').read_bytes()
assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase;image=pe.get_memory_mapped_image()
def q(x):return struct.pack('<Q',x)
def fs(*x):return struct.pack('<'+'f'*len(x),*x)
rows=[]
for dst,drag,ty,small in itertools.product([2,3,4],[0,1,2,3,4],[0x3c,0x3d,0x22],[False,True]):
   uc=Uc(UC_ARCH_X86,UC_MODE_64);uc.mem_map(base,(len(image)+4095)&~4095);uc.mem_write(base,image)
   mem,stack,stop=0x20000000,0x21000000,0x22000000
   uc.mem_map(mem,0x40000);uc.mem_map(stack,0x10000);uc.mem_map(stop,0x1000)
   wrappers=[0]+[mem+i*0x1000 for i in range(1,15)]
   for i in range(1,15):
    for off in range(0,0xc0,8):uc.mem_write(wrappers[i]+off,q(mem+0x18000+i*0x400+off*2))
    uc.mem_write(wrappers[i]+0x10,q(mem+0x35000+i*8));uc.mem_write(wrappers[i]+0x1c,struct.pack('<I',1))
   dispatcher=struct.unpack('<Q',uc.mem_read(wrappers[8],8))[0]
   uc.mem_write(dispatcher,struct.pack('<QQ',3,0));uc.mem_write(dispatcher+0x10,q(0x1122334455667788))
   uc.mem_write(dispatcher+0x30,fs(4.,7.,5.));uc.mem_write(dispatcher+0x58,struct.pack('<I',71));uc.mem_write(dispatcher+0x5c,fs(1.25));uc.mem_write(dispatcher+0x60,bytes([ty]))
   info=mem+0x30000; b=bytearray([0x91]*72);struct.pack_into('<4f',b,0,2.,4.,.25,7.);struct.pack_into('<f',b,16,.25);b[0x22]=dst;b[0x44]=2;b[0x46]=ty
   uc.mem_write(info,bytes(b));uc.mem_write(wrappers[3],fs(0.,0.,0.,0.,0.,0.,0.))
   queue=struct.unpack('<Q',uc.mem_read(wrappers[9],8))[0]
   uc.mem_write(queue+0x20,q(100)+q(mem+0x2c000)+q(0));uc.mem_write(queue+0x40,q(0))
   time=struct.unpack('<Q',uc.mem_read(wrappers[10],8))[0];uc.mem_write(time+0x28,fs(.1))
   sp=stack+0x8008;uc.mem_write(sp,q(stop))
   for i in range(5,15):uc.mem_write(sp+0x28+(i-5)*8,q(wrappers[i]))
   uc.reg_write(UC_X86_REG_RSP,sp)
   for reg,arg in zip([UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8,UC_X86_REG_R9],wrappers[1:5]):uc.reg_write(reg,arg)
   receipt={};heap=[mem+0x31000]
   def rf(reg):return struct.unpack('<f',uc.reg_read(reg).to_bytes(16,'little')[:4])[0]
   def setf(reg,v):uc.reg_write(reg,int.from_bytes(fs(v),'little'))
   def ret():
    s=uc.reg_read(UC_X86_REG_RSP);uc.reg_write(UC_X86_REG_RIP,struct.unpack('<Q',uc.mem_read(s,8))[0]);uc.reg_write(UC_X86_REG_RSP,s+8)
   def hook(u,a,size,unused):
    if a==stop:receipt['unapplied']=True;uc.emu_stop()
    elif a==0x14092e680:uc.reg_write(UC_X86_REG_RAX,1);ret()
    elif a==0x14092df10:uc.reg_write(UC_X86_REG_RAX,drag);ret()
    elif a==0x140a1c3e0:uc.reg_write(UC_X86_REG_RAX,1 if dst!=4 else 2);uc.reg_write(UC_X86_REG_RDX,0);ret()
    elif a==0x140a1c5f0:uc.reg_write(UC_X86_REG_RAX,info);ret()
    elif a in [0x140a1aa20,0x140a1a9f0]:
     out=uc.reg_read(UC_X86_REG_RCX);uc.mem_write(out,fs(0.,0.,0.,1.,2.,3.,4.,1.,1.,1.) if a==0x140a1aa20 else fs(2.,3.,4.));ret()
    elif a==0x140257ba0:ret()
    elif a==0x140c9cde0:uc.mem_write(uc.reg_read(UC_X86_REG_RCX),fs(2.,3.,4.));ret()
    elif a==0x1408ef180:uc.mem_write(uc.reg_read(UC_X86_REG_RCX),fs(0. if small else 4.,rf(UC_X86_REG_XMM2),0. if small else 5.));ret()
    elif a==0x14127c950:setf(UC_X86_REG_XMM0,1.);ret()
    elif a==0x140833a00:setf(UC_X86_REG_XMM0,math.atan2(-rf(UC_X86_REG_XMM0),-rf(UC_X86_REG_XMM1)));ret()
    elif a==0x140833b80:setf(UC_X86_REG_XMM0,math.fmod(rf(UC_X86_REG_XMM0)+rf(UC_X86_REG_XMM1),struct.unpack('<f',fs(math.tau))[0]));ret()
    elif a==0x140613c10:uc.reg_write(UC_X86_REG_RAX,heap[0]);heap[0]+=0x100;ret()
    elif a in [0x140b02490,0x140b02500,0x140b02520,0x140b04e70,0x140b04f70]:ret()
    elif a==0x141276e30:
     receipt['op']=bytes(uc.mem_read(uc.reg_read(UC_X86_REG_RCX),112)).hex();receipt['kind_byte']=uc.reg_read(UC_X86_REG_RDX)&0xff;receipt['location']=bytes(uc.mem_read(uc.reg_read(UC_X86_REG_R8),20)).hex();ret()
    elif a==0x1407a8ec8:
     count=struct.unpack('<Q',uc.mem_read(queue+0x30,8))[0]
     receipt['local']=bytes(uc.mem_read(wrappers[3],28)).hex();receipt['audio_count']=count;receipt['audio_event']=bytes(uc.mem_read(mem+0x2c000,40)).hex() if count else None;uc.emu_stop()
    elif a in [0x1428d9430,0x1428d9310]:raise RuntimeError('native panic '+hex(a))
   uc.hook_add(UC_HOOK_CODE,hook)
   try:uc.emu_start(0x1407a8580,stop,count=10000)
   except Exception as e:raise RuntimeError(f'dst{dst} drag{drag} ty{ty} rip{uc.reg_read(UC_X86_REG_RIP):x}') from e
   rows.append({'dst':dst,'drag':drag,'ty':ty,'small':small,**receipt})
(p/'rotate-native-ops.json').write_text(json.dumps(rows,indent=2));print('captured',len(rows),'original rotate operation requests')
