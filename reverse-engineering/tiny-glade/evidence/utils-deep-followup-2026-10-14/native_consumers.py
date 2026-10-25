"""Original B constructor -> real 2D dense-segment consumer, plus XZ map input.

The original UCRT ceilf is the only additional library boundary bridge. All
private sample count/range, point and tangent outputs execute original code.
"""
from pathlib import Path
import ctypes,hashlib,json,random,struct,sys
sys.path.insert(0,'D:/game/tools/reverse-runtime/utils-gap-20261014/python-deps')
OUT=Path(__file__).parent
sys.path.insert(0,str(OUT.parent/'shared-foundation-2026-10-13/utils-base'))
import native_spline_rdp as o
from unicorn import UC_HOOK_CODE
from unicorn.x86_const import *
crt=ctypes.CDLL('ucrtbase.dll');ceilf=crt.ceilf
ceilf.argtypes=[ctypes.c_float];ceilf.restype=ctypes.c_float
def library(uc,address,size,data):
    if address==0x142923e60:
        bits=o.u.reg_read(UC_X86_REG_XMM0)&0xffffffff
        value=struct.unpack('<f',struct.pack('<I',bits))[0]
        result=ceilf(value)
        o.u.reg_write(UC_X86_REG_XMM0,o.bits(result));o.ret()
o.u.hook_add(UC_HOOK_CODE,library)

def dense(points,index,periodic=False,start=None,end=None):
    source=o.spline(points,start=start,end=end,periodic=periodic)
    header=bytes(o.u.mem_read(o.OUTPUT,32))
    sp=o.reset();o.u.mem_write(o.INPUT+0x4000,header)
    o.u.reg_write(UC_X86_REG_RCX,o.OUTPUT)
    o.u.reg_write(UC_X86_REG_RDX,o.INPUT+0x4000)
    o.u.reg_write(UC_X86_REG_R8,index)
    try:o.run(0x140809ad0)
    except RuntimeError as error:
        if 'original panic branch' not in str(error):raise
        return {'source':source,'index':index,'panic':True}
    capacity,pointer,length=struct.unpack('<QQQ',o.u.mem_read(o.OUTPUT,24))
    assert length<100000
    values=list(struct.unpack('<'+'I'*(length*2),o.u.mem_read(pointer,length*8)))
    tangents=list(struct.unpack('<4I',o.u.mem_read(o.OUTPUT+24,16)))
    return {'source':source,'index':index,'panic':False,'count':length,'points':[values[i:i+2] for i in range(0,len(values),2)],'start_tangent':tangents[:2],'end_tangent':tangents[2:]}

def projection(points):
    o.reset();flat=[b for p in points for b in p]
    o.u.mem_write(o.INPUT,struct.pack('<'+'I'*len(flat),*flat))
    o.u.reg_write(UC_X86_REG_RCX,o.OUTPUT)
    o.u.reg_write(UC_X86_REG_RDX,o.INPUT)
    o.u.reg_write(UC_X86_REG_R8,o.INPUT+12*len(points))
    o.u.reg_write(UC_X86_REG_R9,o.INPUT+0x5000)
    o.run(0x1402ba9e0)
    cap,ptr,n=struct.unpack('<QQQ',o.u.mem_read(o.OUTPUT,24))
    values=list(struct.unpack('<'+'I'*(n*2),o.u.mem_read(ptr,n*8))) if n else []
    result=[values[i:i+2] for i in range(0,len(values),2)]
    assert result==[[p[0],p[2]] for p in points]
    return {'input':points,'output':result,'source_stride':12,'destination_stride':8}

rows=[]
for shape in [[[0.,0.],[.125,0.]],[[0.,0.],[1.,2.],[4.,1.]],[[0.,0.],[3.,4.],[6.,8.]],[[0.,0.],[1.,0.],[0.,1.]]]:
    for periodic in [False,True] if len(shape)>2 else [False]:
        n=len(shape) if periodic else len(shape)-1
        for index in range(n+1):rows.append(dense(shape,index,periodic))
rng=random.Random(141416)
for _ in range(12):
    points=[[rng.uniform(-5,5),rng.uniform(-5,5)] for _ in range(rng.randrange(2,6))]
    for index in range(len(points)-1):rows.append(dense(points,index))
for _ in range(10):
    points=[[rng.uniform(-5,5),rng.uniform(-5,5)] for _ in range(rng.randrange(3,7))]
    for index in range(len(points)):rows.append(dense(points,index,True))
for option in range(1,4):
    for _ in range(6):
        points=[[rng.uniform(-5,5),rng.uniform(-5,5)] for _ in range(rng.randrange(2,6))]
        start=[rng.uniform(-2,2),rng.uniform(-2,2)] if option&1 else None
        end=[rng.uniform(-2,2),rng.uniform(-2,2)] if option&2 else None
        for index in range(len(points)-1):rows.append(dense(points,index,start=start,end=end))
projected=[]
for n in [0,1,2,3,4,19,20,21,32,64]:
    points=[[rng.getrandbits(32) for _ in range(3)] for _ in range(n)]
    if n:points[0]=[0x7fc12345,0x7f800000,0x80000000]
    projected.append(projection(points))
report={'baseline':'B','original_sha256':hashlib.sha256(o.raw).hexdigest(),
 'dense_entry':'0x140809ad0','projection_entry':'0x1402ba9e0','dense':rows,'projection':projected,
 'bridges':['Existing allocation/memory library hooks','Original Windows UCRT ceilf called through ctypes'],
 'limits':'Dense consumer belongs to system_decorator, not proof of unused Spline2D global-query method ABI. Projection is Vec3->Vec2 map/collect, not attribution of folded as_vec2 identity or proof of borrowed wrapper type.'}
(OUT/'consumer-native-fixtures.json').write_text(json.dumps(report,indent=2)+'\n',encoding='utf8')
print(json.dumps({'dense':len(rows),'dense_panics':sum(x['panic'] for x in rows),'projection_arrays':len(projected),'projection_points':sum(len(x['input']) for x in projected)}))
