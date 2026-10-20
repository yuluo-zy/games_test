"""Whole original private angle splitter; dependency allocation/CRT reused."""
import ctypes,json,pathlib,random,struct
from native_cpu_image import NativeCpuImage
image=NativeCpuImage();image.bridge_allocator();assert image.resolve_math_import(0x142923e80)=='cosf'
call=ctypes.WINFUNCTYPE(None,ctypes.c_void_p,ctypes.c_void_p,ctypes.c_uint64,ctypes.c_float)(image.address(0x141fdf9e0))
cases=[];rng=random.Random(0x141fdf9e0)
for i in range(128):
    points=[];x=z=0.
    for j in range(3+i%29):
        x+=rng.uniform(.2,2.);z+=rng.uniform(-1.,1.);points.append([x,rng.uniform(-2.,2.),z])
    packed=b''.join(struct.pack('<3f',*p) for p in points);points=list(struct.iter_unpack('<3f',packed))
    source=ctypes.create_string_buffer(packed);output=ctypes.create_string_buffer(24)
    call(ctypes.addressof(output),ctypes.addressof(source),len(points),struct.unpack('<f',struct.pack('<I',0x3f32b8c2))[0])
    cap,ptr,n=struct.unpack_from('<3Q',output)
    indices=list(struct.unpack('<'+'Q'*n,ctypes.string_at(ptr,n*8)))
    cases.append({'points_bits':[list(struct.unpack('<3I',struct.pack('<3f',*p))) for p in points],'indices':indices})
    image.deallocate(ptr,cap*8,8)
out=pathlib.Path(__file__).with_name('fence-angles-native-cases.json')
out.write_text(json.dumps({'source_sha256':image.sha,'entry':'141fdf9e0','bridges':image.bridges,'cases':cases,'scope':'Whole original angle splitter with actual allocator and exact imported CRT cosine dependency bridges. Finite nonduplicate points. Full fence resampler/RNG/64B output excluded.'},indent=2))
image.close();print(json.dumps({'cases':len(cases),'output':str(out)}))
