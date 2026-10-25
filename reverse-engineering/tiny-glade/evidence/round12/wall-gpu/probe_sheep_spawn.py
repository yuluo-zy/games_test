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
 print('host boundary',flush=True);calls.append({'boundary':'load_json_as_mesh','path':text(path,size)})
 ctypes.memset(out,0,120);return out
im.dependency_callback(0x140b0eeb0,ctypes.WINFUNCTYPE(ctypes.c_void_p,ctypes.c_void_p,ctypes.c_void_p,ctypes.c_size_t),load_mesh,'recording file-Mesh input boundary')
def add_mesh(mesh,name,size,library,watch):
 key=text(name,size);indices=[]
 if key=='half_cube':indices=list(struct.unpack('<18I',ctypes.string_at(u64(mesh,8),72)))
 record={'name':key,'indices':indices,'indices_len':u64(mesh,16),'source_none':u64(mesh,24)==1<<63,'attribute_rawtable':ctypes.string_at(mesh+56,32).hex(),'label':text(u64(mesh,104),u64(mesh,112)) if key=='half_cube' else None}
 print('host boundary',flush=True);calls.append({'boundary':'load_mesh_into_library',**record});meshes.append(record);return 0xa10+len(meshes)
im.dependency_callback(0x140b1aca0,ctypes.WINFUNCTYPE(ctypes.c_uint64,ctypes.c_void_p,ctypes.c_void_p,ctypes.c_size_t,ctypes.c_void_p,ctypes.c_void_p),add_mesh,'recording mesh-library/watch host boundary')
keys=keepbuf(16);write(keys,0,101);write(keys,8,202)
im.dependency_callback(0x1400d9b20,ctypes.WINFUNCTYPE(ctypes.c_void_p,ctypes.c_void_p),lambda _:keys,'std::hash::RandomState::KEYS thread-local boundary')
shaders=[]
def add_shader(library,path,size,name,namesize,watch,device):
 record={'path':text(path,size),'name':text(name,namesize)};print('host boundary',flush=True);calls.append({'boundary':'load_shader_into_library',**record});shaders.append(record);return 0xb10+len(shaders)
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

# Native two-register Option<Handle> host results. Handle lookup policy is
# explicitly the already-loaded mesh path, not animation decoding coverage.
def pair(va,tag,value,where):
 addr=im.k.VirtualAlloc(None,64,0x3000,0x40);keep.append(('exec',addr));code=b'\x48\xb8'+struct.pack('<Q',tag)+b'\x48\xba'+struct.pack('<Q',value)+b'\xc3';ctypes.memmove(addr,code,len(code));im.jump(va,addr);im.bridges.append({'entry':hex(va),'dependency':where})
pair(0x14092d570,1,0xa99,'recording already-loaded sheep_mesh lookup')
pair(0x140937aa0,1,0xb99,'recording existing named shader lookup')
im.resolve_math_import(0x142923c00)
im.resolve_math_import(0x142923c20)
# Current canonical cube is an explicit boundary; RandomState/allocator bridges are standard
# dependency boundaries. Mesh-library add only captures the completed asset.
def add_asset(library,asset):
 print('host boundary',flush=True);calls.append({'boundary':'AssetMeshLibrary::add','name':text(u64(asset,128),u64(asset,136)),'indices_len':u64(asset,16)})
 return 0xc11
im.dependency_callback(0x14092cc60,ctypes.WINFUNCTYPE(ctypes.c_uint64,ctypes.c_void_p,ctypes.c_void_p),add_asset,'recording MeshAsset library host boundary')
def cube(out,box):
 calls.append({'boundary':'existing geometry::cube::from','bounds':list(struct.unpack('<6f',ctypes.string_at(box,24)))})
 ctypes.memset(out,0,120);return out
im.dependency_callback(0x140a05530,ctypes.WINFUNCTYPE(ctypes.c_void_p,ctypes.c_void_p,ctypes.c_void_p),cube,'recording current canonical cube geometry producer boundary')
sheep=[]
def sheep_spawn(out,commands,bundle,location):
 print('sheep spawn',flush=True);raw=ctypes.string_at(bundle,0x198);sheep.append({'bytes':raw.hex(),'transform_u32':list(struct.unpack('<10I',raw[:40])),'prev_u32':list(struct.unpack('<10I',raw[48:88])),'mesh':hex(u64(bundle,96)),'shader':hex(u64(bundle,104)),'sheep_bytes':raw[0xa8:0x110].hex(),'grass_push_bits':struct.unpack_from('<I',raw,0x178)[0],'bird_spots':{'len':u64(bundle,0x188),'records':ctypes.string_at(u64(bundle,0x180),24*u64(bundle,0x188)).hex(),'weak_owner':hex(u64(bundle,0x190))}})
 ctypes.memset(out,0,40);return out
im.dependency_callback(0x1412e2190,ctypes.WINFUNCTYPE(ctypes.c_void_p,ctypes.c_void_p,ctypes.c_void_p,ctypes.c_void_p,ctypes.c_void_p),sheep_spawn,'recording Bevy full sheep bundle spawn')
controls=[]
def control_spawn(out,commands,control,location):
 print('control spawn',flush=True);controls.append({'bytes':ctypes.string_at(control,0x50).hex(),'wire':u64(control,48),'solid':u64(control,56)})
 ctypes.memset(out,0,40);return out
im.dependency_callback(0x1412e23e0,ctypes.WINFUNCTYPE(ctypes.c_void_p,ctypes.c_void_p,ctypes.c_void_p,ctypes.c_void_p,ctypes.c_void_p),control_spawn,'recording Bevy SheepControl spawn')
manager=keepbuf(64);pool=keepbuf(64);write(manager,0,pool);write(pool,0,1);write(pool,8,1);tick=keepbuf(4);manager_param=keepbuf(40);write(manager_param,0,manager);write(manager_param,16,tick)
args=[reference(),reference(2),reference(),reference(),reference(),reference(),reference(),manager_param,reference()]
fn=ctypes.WINFUNCTYPE(None,*([ctypes.c_void_p]*9))(im.address(0x140a093e0));fn(*args)
assert len(sheep)==3 and len(controls)==1
assert u64(manager,8)==3 and u64(pool,8)==4
for i,rec in enumerate(sheep):
 raw=bytes.fromhex(rec['bird_spots']['records']);assert len(raw)==24 and struct.unpack_from('<I',raw,4)[0]==0 and struct.unpack_from('<I',raw,20)[0]==i
 assert list(struct.unpack_from('<3I',raw,8))==rec['transform_u32'][4:7]
assert u64(queue,16)==96
commands=[]
for i in range(2):
 p=data+i*48;commands.append({'executor':hex(u64(p)-im.base+0x140000000),'entity':u64(p,16),'mesh':hex(u64(p,24)),'shader':hex(u64(p,32))})
assert [r['executor'] for r in commands]==['0x1412dd080','0x1412dad40']
out={'source_sha256':im.sha,'scope':'original complete existing-sheep-mesh startup path incl native SheepControl::new host policy and native BirdSpotManager::create_spots; cube geometry is an explicit current-owner boundary; file/compiler/library/Bevy spawn are recording boundaries; not GPU evidence','host_calls':calls,'sheep':sheep,'control':controls,'debug_commands':commands,'bird_next_id':u64(manager,8),'manager_arc_weak_count':u64(pool,8),'bridges':im.bridges}
(root/'sheep-spawn-native-policy.json').write_text(json.dumps(out,indent=2));print('original existing-mesh sheep startup: 3 complete sheep bundles, native control +2debug commands passed')
for entry in keep:
 if isinstance(entry,tuple):im.k.VirtualFree(entry[1],0,0x8000)
im.close()
