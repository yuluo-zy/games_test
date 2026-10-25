"""Execute full original startup policy; explicit host/asset/ECS boundaries.
Original fastrand, radical_inverse, uniform_sample_disk and Transform execute.
Geometry calls are recorded boundaries; their separate algorithms are not
claimed validated by this recorder. No semantic game branch is bypassed.
"""
from pathlib import Path
import ctypes,struct,json,sys
O=Path(__file__).resolve().parent
sys.path.insert(0,str(O.parent/'wall-gpu'))
from native_cpu_image import NativeCpuImage
im=NativeCpuImage();im.bridge_allocator();keep=[];calls=[];butterflies=[]
def mem(n):
 b=ctypes.create_string_buffer(n);keep.append(b);return ctypes.addressof(b)
def q(p,o=0):return struct.unpack('<Q',ctypes.string_at(p+o,8))[0]
def write(p,o,v):ctypes.memmove(p+o,struct.pack('<Q',v),8)
def text(p,n):return ctypes.string_at(p,n).decode()
def load(out,path,n):
 calls.append({'call':'load_file_mesh','path':text(path,n)});ctypes.memset(out,0,120);return out
im.dependency_callback(0x140b0eeb0,ctypes.WINFUNCTYPE(ctypes.c_void_p,ctypes.c_void_p,ctypes.c_void_p,ctypes.c_size_t),load,'file mesh decoder boundary')
meshids=[]
def add(mesh,name,n,library,watch):
 name=text(name,n);meshids.append(name);calls.append({'call':'add_mesh','name':name});return 0xa00+len(meshids)
im.dependency_callback(0x140b1aca0,ctypes.WINFUNCTYPE(ctypes.c_uint64,*([ctypes.c_void_p]*2),ctypes.c_size_t,*([ctypes.c_void_p]*2)),add,'mesh/watch publication boundary')
shaderids=[]
def shader(library,path,n,name,nn,watch,device):
 name=text(name,nn);shaderids.append(name);calls.append({'call':'add_shader','path':text(path,n),'name':name});return 0xb00+len(shaderids)
im.dependency_callback(0x140b3fe40,ctypes.WINFUNCTYPE(ctypes.c_uint64,ctypes.c_void_p,ctypes.c_void_p,ctypes.c_size_t,ctypes.c_void_p,ctypes.c_size_t,ctypes.c_void_p,ctypes.c_void_p),shader,'shader/device publication boundary')
def sphere(out,sub,radius):
 calls.append({'call':'sphere','subdivisions':sub,'radius':radius});ctypes.memset(out,0,120);return out
def plane(out,size):
 calls.append({'call':'plane','size':size});ctypes.memset(out,0,120);return out
def color(out,source,value):
 calls.append({'call':'add_color','color':list(struct.unpack('<3f',ctypes.string_at(value,12)))});ctypes.memmove(out,source,120);return out
im.dependency_callback(0x140a06b00,ctypes.WINFUNCTYPE(ctypes.c_void_p,ctypes.c_void_p,ctypes.c_uint,ctypes.c_float),sphere,'existing game sphere mesh producer recording boundary')
im.dependency_callback(0x140a06370,ctypes.WINFUNCTYPE(ctypes.c_void_p,ctypes.c_void_p,ctypes.c_float),plane,'existing game plane mesh producer recording boundary')
im.dependency_callback(0x140a07680,ctypes.WINFUNCTYPE(ctypes.c_void_p,*([ctypes.c_void_p]*3)),color,'existing Mesh::add_color_self recording boundary')
queue=mem(24);data=mem(65536);entity=[0]
def spawned(out,commands):
 entity[0]+=1;ctypes.memset(out,0,40);write(out,8,queue);write(out,32,entity[0]);return out
def butterfly(out,commands,bundle,location):
 butterflies.append(ctypes.string_at(bundle,176).hex());calls.append({'call':'spawn_butterfly','index':len(butterflies)-1});return spawned(out,commands)
im.dependency_callback(0x140d146a0,ctypes.WINFUNCTYPE(ctypes.c_void_p,ctypes.c_void_p,ctypes.c_void_p),spawned,'Bevy Commands::spawn_empty boundary')
im.dependency_callback(0x1412e1f20,ctypes.WINFUNCTYPE(ctypes.c_void_p,*([ctypes.c_void_p]*4)),butterfly,'Bevy typed nine-component butterfly spawn boundary')
for thunk in [0x142923f10,0x142923e80,0x142923fd0]:im.resolve_math_import(thunk)
def reference(depth=1):
 p=mem(192)
 for _ in range(depth):v=mem(8);write(v,0,p);p=v
 return p
args=[reference(2)]+[reference() for _ in range(5)]+[mem(64),reference(),reference()]
fn=ctypes.WINFUNCTYPE(None,*([ctypes.c_void_p]*9))(im.address(0x1409d49a0))
results=[]
for butterflies_enabled,winter in [(False,True),(True,True),(False,False),(True,False)]:
 calls.clear();butterflies.clear();meshids.clear();shaderids.clear();entity[0]=0
 ctypes.memset(data,0xcc,65536);write(queue,0,65536);write(queue,8,data);write(queue,16,0)
 ctypes.memmove(q(args[7])+0x1f,bytes([butterflies_enabled]),1)
 ctypes.memmove(q(args[8])+0x58,bytes([4 if winter else 0]),1)
 fn(*args)
 records=[];off=0
 while off<q(queue,16):
  p=data+off;executor=q(p)-im.base+0x140000000
  size={0x1412dd460:120,0x1412d7e60:168,0x1412d9de0:120,0x1412db500:136}[executor]
  records.append({'executor':hex(executor),'size':size,'raw':ctypes.string_at(p,size).hex()});off+=size
 assert off==q(queue,16)
 assert len(butterflies)==(24 if butterflies_enabled else 0)
 assert sum(r['executor']=='0x1412d7e60' for r in records)==(0 if winter else 96)
 results.append({'butterflies':butterflies_enabled,'winter':winter,'calls':list(calls),'butterfly_bundles':list(butterflies),'command_records':records})
(O/'original-policy-fixtures.json').write_text(json.dumps({'original_sha256':im.sha,'scope':__doc__,'bridges':im.bridges,'cases':results},indent=2),encoding='utf8')
print(json.dumps([{'butterflies':r['butterflies'],'winter':r['winter'],'butterfly_count':len(r['butterfly_bundles']),'queued_records':len(r['command_records']),'calls':len(r['calls'])} for r in results]))
im.close()
