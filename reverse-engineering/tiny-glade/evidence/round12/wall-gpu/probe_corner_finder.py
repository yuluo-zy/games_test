"""Original complete inlined corner-index finder, not corner geometry.

Keep original prologue and finder instructions. Skip the preceding Curve3
conversion (irrelevant to this finder) and capture the resulting usize Vec at
the first instruction of the subsequent geometry loop. Standard Vec growth
and allocators execute their original code with std allocation bridges.
"""
import ctypes, json, math, pathlib, random, struct
from native_cpu_image import NativeCpuImage
root=pathlib.Path(__file__).parent
im=NativeCpuImage();im.bridge_allocator()
im.jump(0x1412155cc,im.address(0x141215683))
observed=[]
def capture(frame):
    cap,pointer,length=struct.unpack('<3Q',ctypes.string_at(frame+0x2c0,24))
    observed.append(list(struct.unpack('<'+'Q'*length,ctypes.string_at(pointer,8*length))))
    if cap: im.deallocate(pointer,cap*8,8)
callback=ctypes.WINFUNCTYPE(None,ctypes.c_void_p)(capture)
# mov rcx,rbp; mov rax,recording callback; call rax; jmp original epilogue.
code=b'\x48\x89\xe9\x48\xb8'+struct.pack('<Q',ctypes.cast(callback,ctypes.c_void_p).value)+b'\xff\xd0\x48\xb8'+struct.pack('<Q',im.address(0x14121705a))+b'\xff\xe0'
trampoline=im.k.VirtualAlloc(None,len(code),0x3000,0x40)
assert trampoline;ctypes.memmove(trampoline,code,len(code));im.jump(0x141215919,trampoline)
call=ctypes.WINFUNCTYPE(None,ctypes.c_void_p)(im.address(0x141215540))
def f32(v):
    try:return struct.unpack('<f',struct.pack('<f',v))[0]
    except OverflowError:return math.copysign(math.inf,v)
def bits(v):return struct.unpack('<I',struct.pack('<f',v))[0]
def angle(a,b,c):
    x,y=f32(c[0]-b[0]),f32(c[1]-b[1])
    u,v=f32(b[0]-a[0]),f32(b[1]-a[1])
    numerator=f32(f32(y*v)+f32(x*u))
    denominator=f32(math.sqrt(f32(f32(f32(x*x)+f32(y*y))*f32(f32(u*u)+f32(v*v)))))
    dot=f32(numerator/denominator) if denominator else math.nan
    low,high=struct.unpack('<2f',bytes.fromhex('d5d031bed7d0313e'))
    return math.isfinite(dot) and low<=dot<high
def reference(points,closed):
    out=[]
    if closed and angle(points[-2],points[0],points[1]):out.append(0)
    for i in range(1,len(points)-1):
        if angle(points[i-1],points[i],points[i+1]):out.append(i)
    return out
cases=[]
def run(points,closed,name):
    p=ctypes.create_string_buffer(struct.pack('<'+'f'*(len(points)*2),*(v for point in points for v in point)) or b'\0'*8)
    state=ctypes.create_string_buffer(0x2a0)
    struct.pack_into('<QQ',state,8,ctypes.addressof(p),len(points))
    struct.pack_into('<I',state,0x258,0 if closed else 2)
    call(ctypes.addressof(state));actual=observed.pop()
    expected=reference(points,closed)
    assert actual==expected,(name,closed,actual,expected,points)
    cases.append({'name':name,'closed':closed,'points_bits':[[bits(x),bits(y)] for x,y in points],'corners':actual})
run([],False,'empty-open');run([[0.,0.]],False,'one-open')
for closed in [False,True]:
    for p in [ [[0.,0.],[1.,0.]], [[0.,0.],[1.,0.],[1.,1.]],
               [[0.,0.],[1.,0.],[1.,1.],[2.,1.],[2.,2.]],
               [[0.,0.],[1.,0.],[2.,0.],[2.,1.],[3.,1.],[3.,2.]] ]:
        run(p+([p[0]] if closed else []),closed,'fixed')
rng=random.Random(0x141215540)
for case in range(240):
    n=[2,3,4,5,8,17,33,65,129][case%9];closed=bool(case&1)
    points=[]
    for i in range(n):
        if case%4==0:point=[float(rng.randrange(-3,4)),float(rng.randrange(-3,4))]
        elif case%4==1:point=[f32(rng.uniform(-40,40)),f32(rng.uniform(-40,40))]
        elif case%4==2:point=[f32(math.cos(i*math.pi/2)),f32(math.sin(i*math.pi/2))]
        else:point=[f32(i*0.1),f32((i//2)*0.2)]
        points.append(point)
    if closed:points.append(points[0])
    run(points,closed,'generated-'+str(case))
for value in [math.nan,math.inf,-math.inf,0.,-0.,1e-30,1e30]:
    run([[0.,0.],[value,0.],[value,1.],[2.,1.]],False,'nonfinite-and-range')
out={'source_sha256':im.sha,'scope':'original full inlined corner index finder only; excludes Curve3 preparation and complete add_wall_corners geometry/holes/emission',
     'entry_after_original_prologue':'141215683','output_boundary':'141215919','additional_original_closed_start_branch':'1412170bc..14121711f',
     'bridges':im.bridges,'cases':cases}
(root/'corner-finder-native-cases.json').write_text(json.dumps(out,indent=2),encoding='utf-8')
print(len(cases),'original complete corner-index finder cases passed; no full geometry claim')
im.k.VirtualFree(trampoline,0,0x8000);im.close()
