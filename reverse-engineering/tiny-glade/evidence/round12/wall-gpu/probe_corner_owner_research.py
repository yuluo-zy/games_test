"""Research whole B corner owner ABI, no success/production claim until return."""
import ctypes,json,pathlib,struct
from native_cpu_image import NativeCpuImage
im=NativeCpuImage();im.bridge_allocator()
im.resolve_math_import(0x142923e60)
im.resolve_math_import(0x142923c00)
im.resolve_math_import(0x142923c20)
keep=[]
def buf(data):
    b=ctypes.create_string_buffer(data);keep.append(b);return ctypes.addressof(b)
def vec_header():return buf(struct.pack('<3Q',0,8,0))
points=[[0.,0.],[3.,0.],[3.,3.]]
positions=buf(struct.pack('<9f',*(v for x,z in points for v in [x,0.,z])))
outline=buf(struct.pack('<6f',*(v for p in points for v in p)))
us=buf(struct.pack('<3f',0.,.5,1.))
bottom=buf(struct.pack('<3f',0.,0.,0.));top=buf(struct.pack('<3f',3.,3.,3.))
public=ctypes.create_string_buffer(0x2a0);keep.append(public)
struct.pack_into('<3Q',public,0,3,outline,3)
struct.pack_into('<3Q',public,24,3,us,3)
struct.pack_into('<f',public,48,6.)
struct.pack_into('<QQ',public,0x90,positions,3)
struct.pack_into('<QQ',public,0xf8,bottom,3)
struct.pack_into('<I',public,0x258,2)
struct.pack_into('<f',public,0x28c,6.)
options=bytearray(24);struct.pack_into('<5f',options,0,0.,1.,1.,0.,0.);options[20]=2
options=buf(bytes(options));output=vec_header();holes=buf(struct.pack('<3Q',0,4,0)+bytes(0x68-24))
call=ctypes.WINFUNCTYPE(None,ctypes.c_void_p,ctypes.c_uint64,ctypes.c_void_p,ctypes.c_size_t,
    ctypes.c_void_p,ctypes.c_size_t,ctypes.c_void_p,ctypes.c_void_p,ctypes.c_uint8,ctypes.c_void_p)(im.address(0x141215540))
print('calling whole9427B original corner owner research',flush=True)
call(ctypes.addressof(public),17,bottom,3,top,3,output,holes,0,options)
cap,pointer,length=struct.unpack('<3Q',ctypes.string_at(output,24))
hcap,hptr,hlen=struct.unpack('<3Q',ctypes.string_at(holes,24))
result={'status':'whole owner returned; synthetic ABI research input, not a Rust/source or game-chain acceptance',
    'source_sha256':im.sha,'source_range':['141215540','141217a13'],
    'output_records':length,'output':ctypes.string_at(pointer,length*96).hex(),
    'holes_records':hlen,'holes':ctypes.string_at(hptr,hlen*20).hex(),'bridges':im.bridges}
pathlib.Path(__file__).with_name('corner-whole-owner-research.json').write_text(json.dumps(result,indent=2))
print('original whole corner returned:',length,'96B records,',hlen,'20B holes',flush=True)
im.close()
