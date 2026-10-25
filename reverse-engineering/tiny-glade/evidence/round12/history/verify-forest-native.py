"""Run original rectangle helper in Unicorn, retain native-only expected bits.

No Rust codegen, Cargo or production writes. Fixtures validate the consumer's
existing-utility composition at the central Cargo gate.
"""
from pathlib import Path
import hashlib,json,struct
import pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_MEM_UNMAPPED
from unicorn.x86_const import *
HERE=Path(__file__).resolve().parent
raw=Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes()
assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw);base=pe.OPTIONAL_HEADER.ImageBase
u=Uc(UC_ARCH_X86,UC_MODE_64)
def missing(cpu,access,address,size,value,user):
    page=address&~4095
    if not base<=page<base+pe.OPTIONAL_HEADER.SizeOfImage:return False
    cpu.mem_map(page,4096)
    data=pe.get_data(page-base,4096)
    cpu.mem_write(page,data)
    return True
u.hook_add(UC_HOOK_MEM_UNMAPPED,missing)
u.mem_map(0x1000000,0x20000);u.mem_map(0x2000000,0x1000);u.mem_map(0x3000000,0x1000)
def bits(x):return struct.unpack('<I',struct.pack('<f',x))[0]
def native(rect,point):
    u.mem_write(0x2000000,struct.pack('<6f',*rect))
    u.mem_write(0x101fff8,struct.pack('<Q',0x3000000))
    u.reg_write(UC_X86_REG_RSP,0x101fff8);u.reg_write(UC_X86_REG_RCX,0x2000000)
    u.reg_write(UC_X86_REG_XMM1,bits(point[0]));u.reg_write(UC_X86_REG_XMM2,bits(point[1]))
    u.emu_start(0x140c9ae20,0x3000000,count=20000)
    assert u.reg_read(UC_X86_REG_RIP)==0x3000000
    return [u.reg_read(UC_X86_REG_XMM0)&0xffffffff,u.reg_read(UC_X86_REG_XMM1)&0xffffffff]
cases=[]
for axis in [(1,0),(0,1),(0.6,0.8),(-0.8,0.6)]:
    for center,size in [((0,0),(8,4)),((3.25,-7.5),(2,12)),((-65,65),(0.5,0.75))]:
        rect=[*axis,*center,*size]
        for point in [(0,0),(1,2),(-7,-3),(4,2),(65,65),(-65,-65),(100.25,-80.125)]:
            cases.append([list(map(bits,rect)),list(map(bits,point)),native(rect,point)])
progress_cases=[]
assert pe.get_data(0x2925900,4)==struct.pack('<f',-0.3)
for old in [0.0,-0.0,1.0,1.125,-0.25,float('nan'),float('inf')]:
    for delta in [0.0,-0.0,0.016,1.0,float('nan')]:
        for delay in [-0.1,0.0,0.2,float('nan')]:
            for marked in [False,True]:
                frame=0x1008000;tree=0x2000100;clock=0x2000300
                u.mem_write(tree,bytes(24));u.mem_write(tree+12,struct.pack('<ff',old,delay))
                u.mem_write(frame,bytes(0x80));u.mem_write(frame+8,struct.pack('<Q',clock))
                u.mem_write(frame+0x70,bytes([marked]));u.mem_write(clock+0x28,struct.pack('<f',delta))
                u.reg_write(UC_X86_REG_RBP,frame);u.reg_write(UC_X86_REG_R15,tree)
                u.reg_write(UC_X86_REG_RAX,0x2000200);u.reg_write(UC_X86_REG_XMM6,bits(-0.3))
                u.reg_write(UC_X86_REG_XMM7,0);u.reg_write(UC_X86_REG_XMM8,bits(1.0))
                u.emu_start(0x14227a60c,0x14227a6a5,count=100)
                assert u.reg_read(UC_X86_REG_RIP)==0x14227a6a5
                expected=list(struct.unpack('<II',u.mem_read(tree+12,8)))
                progress_cases.append([[bits(old),bits(delta),bits(delay),int(marked)],expected])
result={'original_sha256':hashlib.sha256(raw).hexdigest(),
    'helper':{'address':'0x140c9ae20','size':475,'sha256':hashlib.sha256(pe.get_data(0xc9ae20,475)).hexdigest()},
    'edge_selector':{'address':'0x140c9a9d0','size':726,'sha256':hashlib.sha256(pe.get_data(0xc9a9d0,726)).hexdigest()},
    'scope':'Native-only expected bits, candidate Rust not yet executed','cases':cases,
    'progress_segment':{'start':'0x14227a60c','end_exclusive':'0x14227a6a5',
        'sha256':hashlib.sha256(pe.get_data(0x227a60c,0x99)).hexdigest()},'progress_cases':progress_cases}
(HERE/'forest-native-rectangle-fixtures.json').write_text(json.dumps(result,indent=2),encoding='utf8')
rust='// Original RNE helper executed by verify-forest-native.py; no inferred expected formula.\n'
rust+='pub const RECTANGLE_CASES:&[([u32;6],[u32;2],[u32;2])]=&[\n'
for rect,p,out in cases:rust+='    ('+str(rect)+','+str(p)+','+str(out)+'),\n'
rust+='];\n'
rust+='pub const PROGRESS_CASES:&[([u32;4],[u32;2])]=&[\n'
for inputs,out in progress_cases:rust+='    ('+str(inputs)+','+str(out)+'),\n'
rust+='];\n'
(HERE/'forest-native-fixtures.rs').write_text(rust,encoding='utf8')
print(json.dumps({'native_rectangle_cases':len(cases),'native_progress_cases':len(progress_cases),'rust_executed':False},indent=2))
