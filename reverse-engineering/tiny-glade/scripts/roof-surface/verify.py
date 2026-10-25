"""Differential oracle: real original x64 instructions, bounded pure functions.
No PE entry point is invoked. Input PE is read-only. get_roof_shape resolves its
original CRT powf import in this isolated process; profile allocator is a bridge
and the CurveU consumer is an explicit observation stop, NOT a tested stub.
"""
from pathlib import Path
import ctypes as C, struct, hashlib, json, random, subprocess, sys
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
ROOT=Path('D:/game/reverse-engineering/tiny-glade')
OUT=ROOT/'evidence/roof-surface'
source=Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne')
raw=source.read_bytes();sha=hashlib.sha256(raw).hexdigest()
assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=struct.unpack_from('<I',raw,60)[0]; opt=pe+24
base=struct.unpack_from('<Q',raw,opt+24)[0]
size=struct.unpack_from('<I',raw,opt+56)[0]
hdrsize=struct.unpack_from('<I',raw,opt+60)[0]
sec=[struct.unpack_from('<IIII',raw,opt+struct.unpack_from('<H',raw,pe+20)[0]+40*i+8) for i in range(struct.unpack_from('<H',raw,pe+6)[0])]
image=bytearray(size);image[:hdrsize]=raw[:hdrsize]
for vs,va,rs,ro in sec:image[va:va+rs]=raw[ro:ro+rs]
def cstr(rva):return image[rva:image.index(0,rva)].decode('ascii')
imp=struct.unpack_from('<I',raw,opt+112+8)[0]
pow_entry=None
while True:
    orig,_,_,name,iat=struct.unpack_from('<5I',image,imp)
    if not any((orig,name,iat)):break
    dll=cstr(name);i=0
    while (entry:=struct.unpack_from('<Q',image,(orig or iat)+i*8)[0]):
        if entry<2**63 and cstr(entry+2)=='powf':pow_entry=(dll,base+iat+i*8)
        i+=1
    imp+=20
assert pow_entry
kernel=C.WinDLL('kernel32',use_last_error=True)
kernel.VirtualAlloc.argtypes=[C.c_void_p,C.c_size_t,C.c_uint32,C.c_uint32];kernel.VirtualAlloc.restype=C.c_void_p
mem=kernel.VirtualAlloc(base,size,0x3000,0x40)
assert mem==base,('original preferred image base unavailable',mem,C.get_last_error())
C.memmove(mem,bytes(image),size)
dll=C.WinDLL(pow_entry[0]);powf=dll.powf;powf.argtypes=[C.c_float,C.c_float];powf.restype=C.c_float
address=C.cast(powf,C.c_void_p).value
C.c_uint64.from_address(pow_entry[1]).value=address
module=C.c_void_p()
kernel.GetModuleHandleExW.argtypes=[C.c_uint32,C.c_void_p,C.POINTER(C.c_void_p)]
assert kernel.GetModuleHandleExW(6,address,C.byref(module))
pathbuf=C.create_unicode_buffer(32768);kernel.GetModuleFileNameW(module,pathbuf,32768)
crtpath=Path(pathbuf.value)
crt_version=subprocess.run(['powershell','-NoProfile','-Command',f"(Get-Item -LiteralPath '{crtpath}').VersionInfo.FileVersion"],capture_output=True,text=True,check=True).stdout.strip()
section_fn=C.CFUNCTYPE(C.c_void_p,C.c_void_p,C.c_void_p,C.c_float)(base+0x8e4920)
height_fn=C.CFUNCTYPE(C.c_float,C.c_void_p)(base+0x8e3260)
eave_fn=C.CFUNCTYPE(C.c_float,C.c_void_p)(base+0x8e3c30)
def bits(x):return struct.unpack('<I',struct.pack('<f',x))[0]
def fl(b):return struct.unpack('<f',struct.pack('<I',b))[0]
def setf(buf,o,v):struct.pack_into('<f',buf,o,v)
rng=random.Random(20261011)
lines=[];expected=[];kinds=[]
profiles=[0.,.000001,.1,.25,.5,.75,1.,1.5,2.]
heights=[0.,.000001,.1,.25,.5,.75,1.]
for tag in [0,1,2,3]:
 for profile in profiles:
  for t in heights:
   for switch in [0,1]:
    roof=bytearray(rng.randbytes(88));struct.pack_into('<I',roof,8,tag)
    for o,v in [(0x0c,-2.5),(0x10,3.),(0x14,4.),(0x18,.25),(0x1c,6.),(0x20,9.),(0x2c,profile),(0x30,.4),(0x34,.6),(0x38,.7)]:setf(roof,o,v)
    struct.pack_into('<I',roof,0x3c,switch)
    out=C.create_string_buffer(bytes([0xa5]*28),28);inp=C.create_string_buffer(bytes(roof),88)
    assert section_fn(out,inp,t)==C.addressof(out)
    lines.append(f'section {roof.hex()} {bits(t):08x}');expected.append(out.raw.hex());kinds.append('section')
for _ in range(2500):
    roof=bytearray(rng.randbytes(88));struct.pack_into('<I',roof,8,rng.choice([0,1,2,3]))
    for o in [0x0c,0x10,0x14,0x18,0x1c,0x20]:setf(roof,o,rng.uniform(.05,50.))
    for o in [0x2c,0x30,0x34,0x38]:setf(roof,o,rng.random())
    struct.pack_into('<I',roof,0x3c,rng.randrange(2));t=rng.random()
    out=C.create_string_buffer(bytes([0xa5]*28),28);inp=C.create_string_buffer(bytes(roof),88)
    section_fn(out,inp,t)
    lines.append(f'section {roof.hex()} {bits(t):08x}');expected.append(out.raw.hex());kinds.append('section')
    lines.append(f'height {roof.hex()}');expected.append(f'{bits(height_fn(inp)):08x}');kinds.append('height')
    lines.append(f'eave {struct.unpack_from("<I",roof,0x2c)[0]:08x} {struct.unpack_from("<I",roof,0x38)[0]:08x}')
    expected.append(f'{bits(eave_fn(C.byref(inp,0x24))):08x}');kinds.append('eave')

# Emulate original point generators; observe the input to the original CurveU
# constructor instead of asserting its internal representation is recovered.
cpu=Uc(UC_ARCH_X86,UC_MODE_64);cpu.mem_map(base,(size+4095)&~4095);cpu.mem_write(base,bytes(image))
data=0x20000000;stack=0x30000000;alloc=0x21000000
cpu.mem_map(data,0x10000);cpu.mem_map(stack,0x10000);cpu.mem_map(alloc,0x10000)
seen=[];observe_points=True;bump_heap=False;heap_cursor=alloc
def hook(cpu,pc,length,user):
    if pc==0x140613c10:
        global heap_cursor
        amount=cpu.reg_read(UC_X86_REG_RCX);assert amount<=0x10000
        ptr=heap_cursor if bump_heap else alloc
        if bump_heap:heap_cursor+=(amount+15)&~15;assert heap_cursor<alloc+0x10000
        cpu.reg_write(UC_X86_REG_RAX,ptr)
        rsp=cpu.reg_read(UC_X86_REG_RSP);ret=struct.unpack('<Q',cpu.mem_read(rsp,8))[0]
        cpu.reg_write(UC_X86_REG_RSP,rsp+8);cpu.reg_write(UC_X86_REG_RIP,ret)
    elif pc==0x142923f70:
        a=fl(cpu.reg_read(UC_X86_REG_XMM0)&0xffffffff);b=fl(cpu.reg_read(UC_X86_REG_XMM1)&0xffffffff)
        cpu.reg_write(UC_X86_REG_XMM0,bits(powf(a,b)))
        rsp=cpu.reg_read(UC_X86_REG_RSP);ret=struct.unpack('<Q',cpu.mem_read(rsp,8))[0]
        cpu.reg_write(UC_X86_REG_RSP,rsp+8);cpu.reg_write(UC_X86_REG_RIP,ret)
    elif pc==0x140613c20:
        rsp=cpu.reg_read(UC_X86_REG_RSP);ret=struct.unpack('<Q',cpu.mem_read(rsp,8))[0]
        cpu.reg_write(UC_X86_REG_RSP,rsp+8);cpu.reg_write(UC_X86_REG_RIP,ret)
    elif pc==0x141469430 and observe_points:
        vec=cpu.reg_read(UC_X86_REG_RDX)
        _,ptr,n=struct.unpack('<QQQ',cpu.mem_read(vec,24))
        assert n<4096
        seen.append(bytes(cpu.mem_read(ptr,n*8)));cpu.emu_stop()
cpu.hook_add(UC_HOOK_CODE,hook)
def prepare():
    global heap_cursor
    heap_cursor=alloc
    seen.clear();cpu.reg_write(UC_X86_REG_RSP,stack+0x8008)
    cpu.reg_write(UC_X86_REG_MXCSR,0x1f80)
    cpu.reg_write(UC_X86_REG_RCX,data);cpu.reg_write(UC_X86_REG_RDX,data+0x100)
for profile in profiles+[rng.random() for _ in range(200)]:
    prepare();cpu.mem_write(data+0x100,bytes(88));cpu.mem_write(data+0x12c,struct.pack('<f',profile))
    cpu.emu_start(0x1408e3420,0,count=10000);assert len(seen)==1
    lines.append(f'profile {bits(profile):08x}')
    expected.append(''.join(f'{v:08x}' for v in struct.unpack('<40I',seen[0])));kinds.append('profile')
for n in [0,1,2,3,4,5,11,12,13,20,21,32]:
 for _ in range(20):
    points=b''.join(struct.pack('<ff',rng.random(),rng.random()) for _ in range(n))
    bottom,top,h=[rng.uniform(.1,30.) for _ in range(3)]
    prepare();cpu.mem_write(data+0x100,struct.pack('<QQQ',n,data+0x1000,n));cpu.mem_write(data+0x1000,points)
    cpu.reg_write(UC_X86_REG_XMM2,bits(bottom));cpu.reg_write(UC_X86_REG_XMM3,bits(top))
    cpu.mem_write(stack+0x8030,struct.pack('<f',h))
    cpu.emu_start(0x1408e3d40,0,count=10000);assert len(seen)==1
    lines.append(f'transform {bits(bottom):08x} {bits(top):08x} {bits(h):08x} '+(points.hex() or '-'))
    expected.append(seen[0].hex());kinds.append('transform')

observe_points=False
stop=0x40000000;cpu.mem_map(stop,0x1000)
curve_points=[]
curve_edge=[[],[[0.,0.]],[[0.,0.],[0.,0.]],[[0.,0.],[1.,0.]],
    [[0.,0.],[1.,0.],[1.,0.],[2.,0.]],[[0.,0.],[float('inf'),0.]],
    [[0.,0.],[float('nan'),0.]],[[1e-30,0.],[0.,0.]],[[1e30,0.],[0.,0.]],
    [[-0.,0.],[0.,-0.],[1.,0.]]]
for n in [2,3,4,5,11,12,13,20,21,32,63]:
 for _ in range(20):curve_edge.append([[rng.uniform(-100.,100.),rng.uniform(-100.,100.)] for _ in range(n)])
def run_curve(points,reject):
    packed=b''.join(struct.pack('<2f',*p) for p in points);n=len(points)
    prepare();cpu.mem_write(data,bytes([0xa5]*56));cpu.mem_write(data+0x100,struct.pack('<QQQ',n,data+0x1000,n));cpu.mem_write(data+0x1000,packed)
    cpu.reg_write(UC_X86_REG_R8,reject)
    cpu.mem_write(stack+0x8008,struct.pack('<Q',stop))
    cpu.emu_start(0x141469430,stop,count=10000)
    output=bytes(cpu.mem_read(data,56));first=struct.unpack_from('<Q',output)[0]
    if first==0x8000000000000000:
        code=output[8]
        if code==0:return f'error TooFewPoints({output[9]})',None
        if code==1:return 'error NonFiniteLength',None
        assert code==2
        return f'error DuplicateSegment({struct.unpack_from("<Q",output,16)[0]})',None
    assert first==n and struct.unpack_from('<Q',output,16)[0]==n
    ptr=struct.unpack_from('<Q',output,32)[0];us=bytes(cpu.mem_read(ptr,n*4))
    return 'ok '+f'{struct.unpack_from("<I",output,48)[0]:08x}'+' '+''.join(f'{v:08x}' for v in struct.unpack('<'+'I'*n,us)),(packed,output,us)
for points in curve_edge:
 for reject in [0,1]:
    expected_curve,state=run_curve(points,reject)
    packed=b''.join(struct.pack('<2f',*p) for p in points)
    lines.append(f'curve {reject} '+(packed.hex() or '-'));expected.append(expected_curve);kinds.append('curve')
    if state is not None and reject==0:
        curve_points.append(points)
for points in curve_points:
    _,state=run_curve(points,0);packed,output,us=state
    fractions=[-.1,0.,.0001,.1,.5,.9999,1.,1.1]+list(struct.unpack('<'+'f'*len(points),us))
    for u in fractions:
        # An interior zero-length segment is bypassed by the right-biased search.
        cpu.mem_write(data,output);cpu.reg_write(UC_X86_REG_RCX,data);cpu.reg_write(UC_X86_REG_XMM1,bits(u));cpu.reg_write(UC_X86_REG_RSP,stack+0x8008)
        cpu.mem_write(stack+0x8008,struct.pack('<Q',stop));cpu.emu_start(0x141467ce0,stop,count=10000)
        coord=(cpu.reg_read(UC_X86_REG_RAX)&0xffffffff,cpu.reg_read(UC_X86_REG_XMM0)&0xffffffff)
        values=[]
        for fn in [0x141467270,0x141468f90]:
            cpu.reg_write(UC_X86_REG_RCX,data);cpu.reg_write(UC_X86_REG_XMM1,bits(u));cpu.reg_write(UC_X86_REG_RSP,stack+0x8008)
            cpu.mem_write(stack+0x8008,struct.pack('<Q',stop));cpu.emu_start(fn,stop,count=10000)
            values.extend([cpu.reg_read(UC_X86_REG_XMM0)&0xffffffff,cpu.reg_read(UC_X86_REG_XMM1)&0xffffffff])
        lines.append(f'sample {bits(u):08x} {packed.hex()}');expected.append(f'{coord[0]} {coord[1]:08x} '+' '.join(f'{b:08x}' for b in values));kinds.append('sample')

fixtures=json.loads((ROOT/'evidence/roof-fixtures.json').read_text(encoding='utf-8'))
fixture_cases=[]
for index,item in enumerate(fixtures['samples']):
    p=item['params']
    for tag in [0,1]:
      # Width/Length encoding has not been confirmed; test both concrete bits.
      for switch in [0,1]:
        roof=bytearray(88);struct.pack_into('<I',roof,8,tag)
        for o,v in [(0x0c,-2.5),(0x10,3.),(0x14,4.),(0x18,.25),(0x1c,6.),(0x20,9.),
            (0x24,p['tip_offset_01'][0]),(0x28,p['tip_offset_01'][1]),(0x2c,p['profile_01']),
            (0x30,p['height_01']),(0x34,p['ridge_length_01']),(0x38,p['eave_length_01'])]:setf(roof,o,v)
        struct.pack_into('<I',roof,0x3c,switch)
        inp=C.create_string_buffer(bytes(roof),88)
        for t in [0.,.1,.5,.9,1.]:
            out=C.create_string_buffer(bytes([0xa5]*28),28);section_fn(out,inp,t)
            lines.append(f'section {roof.hex()} {bits(t):08x}');expected.append(out.raw.hex());kinds.append('fixture_section')
        lines.append(f'height {roof.hex()}');expected.append(f'{bits(height_fn(inp)):08x}');kinds.append('fixture_height')
        bottom_out=C.create_string_buffer(28);top_out=C.create_string_buffer(28)
        section_fn(bottom_out,inp,0.);section_fn(top_out,inp,1.)
        width_offset=20 if tag==1 else 12
        bottom=struct.unpack_from('<f',bottom_out.raw,width_offset)[0];top=struct.unpack_from('<f',top_out.raw,width_offset)[0]
        fixture_cases.append((p['profile_01'],bottom,top,height_fn(inp)))
bump_heap=True
for profile,bottom,top,h in [(rng.random(),rng.uniform(.1,30.),rng.uniform(.1,30.),rng.uniform(.1,30.)) for _ in range(200)]+fixture_cases:
    prepare();cpu.mem_write(data+0x100,bytes(88));cpu.mem_write(data+0x12c,struct.pack('<f',profile))
    cpu.mem_write(stack+0x8008,struct.pack('<Q',stop));cpu.emu_start(0x1408e3420,stop,count=20000)
    assert struct.unpack('<Q',cpu.mem_read(data,8))[0]!=0x8000000000000000
    cpu.reg_write(UC_X86_REG_RCX,data+0x200);cpu.reg_write(UC_X86_REG_RDX,data)
    cpu.reg_write(UC_X86_REG_XMM2,bits(bottom));cpu.reg_write(UC_X86_REG_XMM3,bits(top));cpu.reg_write(UC_X86_REG_RSP,stack+0x8008)
    cpu.mem_write(stack+0x8030,struct.pack('<f',h));cpu.mem_write(stack+0x8008,struct.pack('<Q',stop))
    cpu.emu_start(0x1408e3d40,stop,count=20000)
    result_curve=bytes(cpu.mem_read(data+0x200,56))
    assert struct.unpack_from('<Q',result_curve)[0]!=0x8000000000000000
    n=struct.unpack_from('<Q',result_curve,16)[0];points_ptr=struct.unpack_from('<Q',result_curve,8)[0];us_ptr=struct.unpack_from('<Q',result_curve,32)[0]
    point_bits=struct.unpack('<'+'I'*(n*2),cpu.mem_read(points_ptr,n*8));us_bits=struct.unpack('<'+'I'*n,cpu.mem_read(us_ptr,n*4))
    expected.append(f'{struct.unpack_from("<I",result_curve,48)[0]:08x} '+''.join(f'{v:08x}' for v in point_bits)+' '+''.join(f'{v:08x}' for v in us_bits))
    lines.append('chain '+' '.join(f'{bits(v):08x}' for v in [profile,bottom,top,h]));kinds.append('whole_profile_chain')
binary=ROOT/'workspace/roof-surface-reconstructed.exe'
subprocess.run(['C:/Users/liyu/.cargo/bin/rustc.exe','--edition=2021','-O',str(ROOT/'reconstruction/roof/surface.rs'),'-o',str(binary)],check=True)
result=subprocess.run([str(binary)],input='\n'.join(lines)+'\n',text=True,capture_output=True)
if result.returncode:print(result.stderr);raise SystemExit(result.returncode)
actual=result.stdout.splitlines();fail=[dict(index=i,kind=kinds[i],input=lines[i],original=a,candidate=b) for i,(a,b) in enumerate(zip(expected,actual)) if a!=b]
report=dict(source_sha256=sha,ok=not fail and len(actual)==len(expected),full_game_executed=False,pe_entry_invoked=False,
oracle='original x64 function bytes directly executed in isolated Python child, profile point generators emulated in Unicorn',
external_dependency=dict(import_dll=pow_entry[0],import_name='powf',actual_dll=str(crtpath),version=crt_version,sha256=hashlib.sha256(crtpath.read_bytes()).hexdigest()),
counts={k:kinds.count(k) for k in set(kinds)},fixture_source={'path':'evidence/roof-fixtures.json','sha256':hashlib.sha256((ROOT/'evidence/roof-fixtures.json').read_bytes()).hexdigest(),'snapshots':len(fixtures['samples']),'wall_shapes':'synthetic circle/rectangle; original scene replay not performed','ridge_dir_encoding':'both raw bits tested; history Width/Length mapping not claimed'},failures=fail[:30],failure_count=len(fail),output_comparison='exact bits and exact written bytes; circle unwritten tail preserved',
scope=dict(get_roof_shape='entire function and original powf imported API resolved in current Windows environment, no core stub',height='entire non-panic function',eave='entire function',profile='entire point generation until CurveU constructor; allocation bridge; native CRT powf bridge',transform='all SIMD/scalar loop paths through CurveU consumer boundary; allocation bridge',curve='entire Curve2 constructor including error branches and duplicate policy; allocation/deallocation bridge; compare ownership-independent metadata, cumulative points_u and length',sample='entire coordinate lookup, position and tangent functions with original inverse_lerp dependency, no numerical stubs',whole_profile_chain='full profile_curve_normalized -> original Curve2 ctor -> profile_curve_ws -> original Curve2 ctor; allocator/free/nativepowf bridges; compare all points, points_u and length'),
limitations=['Current host CRT dependency is recorded; original game process dependency version was not observed','Curve2 ownership pointer values and allocation failures not reconstructed','height panic path, coordinate panic formatting and MXCSR flags not covered','no runtime game execution, rendering or shader validation'])
(OUT/'verification.json').write_text(json.dumps(report,ensure_ascii=False,indent=2),encoding='utf-8')
print(json.dumps(dict(ok=report['ok'],counts=report['counts'],failures=len(fail)),ensure_ascii=False))
if fail:raise SystemExit(1)
