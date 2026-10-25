"""Run the original complete startup policy with explicit host/ECS boundary recorders.
No file loader, shader compiler or Bevy command application is claimed tested.
Private game startup control/data instructions remain original.
"""
import ctypes,pathlib,struct,json
from native_cpu_image import NativeCpuImage
root=pathlib.Path(__file__).parent
im=NativeCpuImage();im.bridge_allocator();keep=[];calls=[];meshes=[]
def keepbuf(size):
 b=ctypes.create_string_buffer(size);keep.append(b);return ctypes.addressof(b)
def u64(p,n=0):return struct.unpack('<Q',ctypes.string_at(p+n,8))[0]
def write(p,off,val):ctypes.memmove(p+off,struct.pack('<Q',val),8)
def text(p,n):return ctypes.string_at(p,n).decode()
# Boundary: file-backed Mesh loader result. The startup forwards this Mesh
# unchanged; its decoder and filesystem are intentionally outside this fixture.
def load_mesh(out,path,size):
 calls.append({'boundary':'load_json_as_mesh','path':text(path,size)})
 ctypes.memset(out,0,120);return out
im.dependency_callback(0x140b0eeb0,ctypes.WINFUNCTYPE(ctypes.c_void_p,ctypes.c_void_p,ctypes.c_void_p,ctypes.c_size_t),load_mesh,'recording file-Mesh input boundary')
def add_mesh(mesh,name,size,library,watch):
 key=text(name,size);indices=[]
 if key=='half_cube':indices=list(struct.unpack('<18I',ctypes.string_at(u64(mesh,8),72)))
 record={'name':key,'indices':indices,'indices_len':u64(mesh,16),'source_none':u64(mesh,24)==1<<63,'attribute_rawtable':ctypes.string_at(mesh+56,32).hex(),'label':text(u64(mesh,104),u64(mesh,112)) if key=='half_cube' else None}
 calls.append({'boundary':'load_mesh_into_library',**record});meshes.append(record);return 0xa10+len(meshes)
im.dependency_callback(0x140b1aca0,ctypes.WINFUNCTYPE(ctypes.c_uint64,ctypes.c_void_p,ctypes.c_void_p,ctypes.c_size_t,ctypes.c_void_p,ctypes.c_void_p),add_mesh,'recording mesh-library/watch host boundary')
keys=keepbuf(16);write(keys,0,101);write(keys,8,202)
im.dependency_callback(0x1400d9b20,ctypes.WINFUNCTYPE(ctypes.c_void_p,ctypes.c_void_p),lambda _:keys,'std::hash::RandomState::KEYS thread-local boundary')
shaders=[]
def add_shader(library,path,size,name,namesize,watch,device):
 record={'path':text(path,size),'name':text(name,namesize)};calls.append({'boundary':'load_shader_into_library',**record});shaders.append(record);return 0xb10+len(shaders)
im.dependency_callback(0x140b3fe40,ctypes.WINFUNCTYPE(ctypes.c_uint64,ctypes.c_void_p,ctypes.c_void_p,ctypes.c_size_t,ctypes.c_void_p,ctypes.c_size_t,ctypes.c_void_p,ctypes.c_void_p),add_shader,'recording shader compilation/library host boundary')
queue=keepbuf(24);data=keepbuf(4096);ctypes.memset(data,0xcc,4096);write(queue,0,4096);write(queue,8,data);write(queue,16,0)
entity=[100]
def spawn(out,commands):
 entity[0]+=1;ctypes.memset(out,0,40);write(out,8,queue);write(out,32,entity[0]);return out
im.dependency_callback(0x140d146a0,ctypes.WINFUNCTYPE(ctypes.c_void_p,ctypes.c_void_p,ctypes.c_void_p),spawn,'Bevy Commands::spawn_empty host boundary')
def reference(depth=1):
 p=keepbuf(64)
 for _ in range(depth):q=keepbuf(8);write(q,0,p);p=q
 return p
args=[reference(2),reference(),reference(),reference(),reference(),reference()]
fn=ctypes.WINFUNCTYPE(None,*([ctypes.c_void_p]*6))(im.address(0x1414547d0));fn(*args)
assert u64(queue,16)==2*120,u64(queue,16)
records=[]
for i in range(2):
 p=data+i*120;records.append({'executor':hex(u64(p)-im.base+0x140000000),'transform_u32':list(struct.unpack('<10I',ctypes.string_at(p+8,40))),'mesh':hex(u64(p,56)),'shader':hex(u64(p,64)),'entity':u64(p,88)})
assert [r['executor'] for r in records]==['0x1412dd460']*2
assert all(r['transform_u32']==[0,0,0,0x3f800000,0,0,0,0x3f800000,0x3f800000,0x3f800000] for r in records)
assert [r['mesh'] for r in records]==['0xa11','0xa12'] and [r['shader'] for r in records]==['0xb11','0xb12']
assert [r['name'] for r in meshes]==['roof_spire','roof_support_beam','roof_button','roof_delete_scribble']
out={'source_sha256':im.sha,'range':'1414547d0..141454e60','scope':'complete original startup policy; explicit recording file/library/compiler/ECS boundaries, not GPU evidence','host_calls':calls,'command_records':records,'bridges':im.bridges}
(root/'roof-startup-native-policy.json').write_text(json.dumps(out,indent=2));print('original roof startup: interleaved 4 meshes,2shaders,2identity Drawables passed')
im.close()
