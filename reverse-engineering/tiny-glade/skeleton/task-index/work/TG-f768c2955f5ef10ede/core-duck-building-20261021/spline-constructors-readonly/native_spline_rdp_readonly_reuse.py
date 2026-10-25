"""B business-entry oracle; only allocation/memory library boundaries are hooked."""
import hashlib,json,random,struct
from pathlib import Path
import pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE,UcError
from unicorn.x86_const import *
OUT=Path(__file__).parent
raw=Path('/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes()
assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(pe.OPTIONAL_HEADER.SizeOfImage+4095)&~4095)
u.mem_write(base,pe.get_memory_mapped_image())
STACK,INPUT,OUTPUT,BUMP,HEADER,HEAP,STOP=0x200000000,0x210000000,0x210010000,0x210011000,0x210012000,0x230000000,0x240000000
u.mem_map(STACK,0x20000);u.mem_map(INPUT,0x20000);u.mem_map(HEAP,0x1000000);u.mem_map(STOP,0x1000)
heap_top=HEAP
trace=[]
def pack(values):return struct.pack('<'+'f'*len(values),*values)
def bits(v):return struct.unpack('<I',pack([v]))[0]
def ret(result=0):
    sp=u.reg_read(UC_X86_REG_RSP);address=struct.unpack('<Q',u.mem_read(sp,8))[0]
    u.reg_write(UC_X86_REG_RAX,result);u.reg_write(UC_X86_REG_RSP,sp+8);u.reg_write(UC_X86_REG_RIP,address)
def alloc(size,align):
    global heap_top
    result=(heap_top+align-1)&~(align-1);heap_top=result+size
    assert heap_top<HEAP+0x800000
    return result
def hook(uc,address,size,data):
    if address in [0x140613c10,0x140613c40]:
        ret(alloc(u.reg_read(UC_X86_REG_RCX),u.reg_read(UC_X86_REG_RDX)))
    elif address==0x140613c20:ret()
    elif address==0x140613c30:
        old,old_len,align,new_len=[u.reg_read(r) for r in [UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8,UC_X86_REG_R9]]
        pointer=alloc(new_len,align);u.mem_write(pointer,bytes(u.mem_read(old,min(old_len,new_len))));ret(pointer)
    elif address in [0x142923c00,0x142923c10]:
        dst,src,n=[u.reg_read(r) for r in [UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8]]
        u.mem_write(dst,bytes(u.mem_read(src,n)));ret(dst)
    elif address==0x142923c20:
        dst,v,n=[u.reg_read(r) for r in [UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8]]
        u.mem_write(dst,bytes([v&255])*n);ret(dst)
    elif address in [0x1428d9430,0x1428d9330,0x1428d9518,0x1428d9310,0x1428d79d0,0x1428d8fe3]:
        raise RuntimeError(f'original panic branch {address:#x}')
u.hook_add(UC_HOOK_CODE,hook)
def reset():
    global heap_top
    heap_top=HEAP
    sp=STACK+0x10008;u.reg_write(UC_X86_REG_RSP,sp);u.mem_write(sp,struct.pack('<Q',STOP)+bytes(128))
    u.mem_write(OUTPUT,bytes(512));u.mem_write(BUMP,bytes(64));u.mem_write(HEADER,bytes(64))
    u.mem_write(BUMP+0x10,struct.pack('<Q',HEADER))
    u.mem_write(HEADER,struct.pack('<Q',HEAP+0x800000));u.mem_write(HEADER+0x20,struct.pack('<Q',HEAP+0x1000000))
    for r in [UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8,UC_X86_REG_R9,UC_X86_REG_XMM0,UC_X86_REG_XMM1,UC_X86_REG_XMM2,UC_X86_REG_XMM3]:u.reg_write(r,0)
    return sp
def run(address):
    try:u.emu_start(address,STOP,count=3000000)
    except Exception as e:raise RuntimeError(f'{e}; pc={u.reg_read(UC_X86_REG_RIP):#x}') from e
    assert u.reg_read(UC_X86_REG_RIP)==STOP,hex(u.reg_read(UC_X86_REG_RIP))
def spline(points,start=None,end=None,periodic=False):
    sp=reset();dims=len(points[0]);u.mem_write(INPUT,pack([v for p in points for v in p]))
    u.reg_write(UC_X86_REG_RCX,OUTPUT);u.reg_write(UC_X86_REG_RDX,BUMP);u.reg_write(UC_X86_REG_R8,INPUT);u.reg_write(UC_X86_REG_R9,len(points))
    tangent_start,tangent_end=INPUT+0x4000,INPUT+0x4040
    for pointer,tangent in [(tangent_start,start),(tangent_end,end)]:
        u.mem_write(pointer,struct.pack('<I',int(tangent is not None))+pack(tangent or [0]*dims))
    u.mem_write(sp+0x28,struct.pack('<QQ',tangent_start,tangent_end))
    run(0x140c930f0 if periodic else 0x140c926b0 if dims==2 else 0x140c93970)
    if dims==3:
        storage,ptr,n,min_t,max_t=struct.unpack('<QQQII',u.mem_read(OUTPUT,32));flag=0
        assert storage==0x8000000000000000
    else:
        ptr,n,min_t,max_t,flag=struct.unpack('<QQIIB',u.mem_read(OUTPUT,25))
    assert n==(len(points) if periodic else len(points)-1),(n,points)
    # Native segment layout = start_t then scalar polynomial per axis, each
    # scalar segment's own start_t retained in its trailing slot (44/64 bytes).
    stride=44 if dims==2 else 64
    values=[list(struct.unpack('<'+'I'*(stride//4),u.mem_read(ptr+i*stride,stride))) for i in range(n)]
    samples=[]
    if dims==3:
        maximum=struct.unpack('<f',struct.pack('<I',max_t))[0]
        for t in [-1.0,0.0,maximum*.2,maximum*.5,maximum,maximum+1.0]:
            outputs=[]
            for function in [0x140c946a0,0x140c94800]:
                u.reg_write(UC_X86_REG_RSP,sp);u.mem_write(sp,struct.pack('<Q',STOP))
                u.reg_write(UC_X86_REG_RCX,OUTPUT+0x100);u.reg_write(UC_X86_REG_RDX,OUTPUT);u.reg_write(UC_X86_REG_XMM2,bits(t))
                run(function);outputs.append(list(struct.unpack('<3I',u.mem_read(OUTPUT+0x100,12))))
            samples.append({'t':bits(t),'position':outputs[0],'tangent':outputs[1]})
    return {'points':[[bits(v) for v in p] for p in points],'start':None if start is None else [bits(v) for v in start],'end':None if end is None else [bits(v) for v in end],'periodic':periodic,'segments':values,'min_t':min_t,'max_t':max_t,'flag':flag,'samples':samples}
def rdp(points,epsilon,spacing):
    sp=reset();u.mem_write(INPUT,pack([v for p in points for v in p]))
    u.reg_write(UC_X86_REG_RCX,OUTPUT);u.reg_write(UC_X86_REG_RDX,INPUT);u.reg_write(UC_X86_REG_R8,len(points));u.reg_write(UC_X86_REG_XMM3,bits(epsilon))
    u.mem_write(sp+0x28,pack([spacing]))
    run(0x141fe0b90)
    cap,ptr,n,pcap,pptr,pn=struct.unpack('<6Q',u.mem_read(OUTPUT,48))
    assert n<=len(points) and pn==n,(cap,ptr,n,pcap,pptr,pn)
    indices=list(struct.unpack('<'+'Q'*n,u.mem_read(ptr,n*8))) if n else []
    return {'points':[[bits(v) for v in p] for p in points],'epsilon':bits(epsilon),'spacing':bits(spacing),'indices':indices}
if __name__=='__main__':
    import sys
    if '--probe' in sys.argv:
        print('spline',spline([[0,0],[1,1],[2,0]]))
        print('rdp',rdp([[0,0,0],[1,1,0],[2,0,0]],.1,0))
    else:
        rng=random.Random(1301323);fixtures={'spline2d':[],'spline3d':[],'spline_periodic':[],'rdp':[]}
        for dims in [2,3]:
            for n in [2,3,4,8,18]:
                for option in range(4):
                    points=[[rng.uniform(-10,10) for _ in range(dims)] for _ in range(n)]
                    start=[rng.uniform(-2,2) for _ in range(dims)] if option&1 else None
                    end=[rng.uniform(-2,2) for _ in range(dims)] if option&2 else None
                    fixtures[f'spline{dims}d'].append(spline(points,start,end))
        for n in [3,4,8,18]:
            fixtures['spline_periodic'].append(spline([[rng.uniform(-10,10),rng.uniform(-10,10)] for _ in range(n)],periodic=True))
        for n in [0,1,2,3,4,5,8,16,31]:
            for _ in range(12):
                points=[[i+rng.uniform(-1,1),rng.uniform(-8,8),rng.uniform(-8,8)] for i in range(n)]
                fixtures['rdp'].append(rdp(points,rng.uniform(.01,3),rng.choice([0,.5,2,10])))
        report={'original_sha256':hashlib.sha256(raw).hexdigest(),'method':'Original business entries execute. Hooks are allocation/deallocation/reallocation/memcpy/memmove/memset only. Original std and bumpalo code executes, not reimplemented as game source. Sufficient bump storage supplied by fixture.', 'fixtures':fixtures}
        (OUT/'spline-rdp-native-fixtures.json').write_text(json.dumps(report,indent=2)+'\n',encoding='utf8')
        print({k:len(v) for k,v in fixtures.items()})
