from pathlib import Path
p=Path('reverse-engineering/tiny-glade/evidence/round12/wall-gpu/probe_wall_startup.py');s=p.read_text(encoding='utf-8-sig');s=s[:s.index('args=[reference')]+'''
# Native two-register Option<Handle> host results. Handle lookup policy is
# explicitly the already-loaded mesh path, not animation decoding coverage.
def pair(va,tag,value,where):
 addr=im.k.VirtualAlloc(None,64,0x3000,0x40);keep.append(('exec',addr));code=b'\\x48\\xb8'+struct.pack('<Q',tag)+b'\\x48\\xba'+struct.pack('<Q',value)+b'\\xc3';ctypes.memmove(addr,code,len(code));im.jump(va,addr);im.bridges.append({'entry':hex(va),'dependency':where})
pair(0x14092d570,1,0xa99,'recording already-loaded sheep_mesh lookup')
pair(0x140937aa0,1,0xb99,'recording existing named shader lookup')
im.resolve_math_import(0x142923c00)
# Real original cube generation, RandomState/allocator bridges remain standard
# dependency boundaries. Mesh-library add only captures the completed asset.
def add_asset(library,asset):
 calls.append({'boundary':'AssetMeshLibrary::add','name':text(u64(asset,128),u64(asset,136)),'indices_len':u64(asset,16)})
 return 0xc11
im.dependency_callback(0x14092cc60,ctypes.WINFUNCTYPE(ctypes.c_uint64,ctypes.c_void_p,ctypes.c_void_p),add_asset,'recording MeshAsset library host boundary')
sheep=[]
def sheep_spawn(out,commands,bundle,location):
 raw=ctypes.string_at(bundle,0x180);sheep.append({'bytes':raw.hex(),'transform_u32':list(struct.unpack('<10I',raw[:40])),'prev_u32':list(struct.unpack('<10I',raw[48:88])),'mesh':hex(u64(bundle,96)),'shader':hex(u64(bundle,104)),'sheep_bytes':raw[0xa8:0x110].hex(),'grass_push_bits':struct.unpack_from('<I',raw,0x178)[0]})
 ctypes.memset(out,0,40);return out
im.dependency_callback(0x1412e2190,ctypes.WINFUNCTYPE(ctypes.c_void_p,ctypes.c_void_p,ctypes.c_void_p,ctypes.c_void_p,ctypes.c_void_p),sheep_spawn,'recording Bevy full sheep bundle spawn')
controls=[]
def control_spawn(out,commands,control,location):
 controls.append({'bytes':ctypes.string_at(control,0x50).hex(),'wire':u64(control,48),'solid':u64(control,56)})
 ctypes.memset(out,0,40);return out
im.dependency_callback(0x1412e23e0,ctypes.WINFUNCTYPE(ctypes.c_void_p,ctypes.c_void_p,ctypes.c_void_p,ctypes.c_void_p,ctypes.c_void_p),control_spawn,'recording Bevy SheepControl spawn')
manager=keepbuf(64);pool=keepbuf(64);write(manager,0,pool);tick=keepbuf(4);manager_param=keepbuf(40);write(manager_param,0,manager);write(manager_param,16,tick)
args=[reference(),reference(2),reference(),reference(),reference(),reference(),reference(),manager_param,reference()]
fn=ctypes.WINFUNCTYPE(None,*([ctypes.c_void_p]*9))(im.address(0x140a093e0));fn(*args)
assert len(sheep)==3 and len(controls)==1
assert u64(queue,16)==96
commands=[]
for i in range(2):
 p=data+i*48;commands.append({'executor':hex(u64(p)-im.base+0x140000000),'entity':u64(p,16),'mesh':hex(u64(p,24)),'shader':hex(u64(p,32))})
assert [r['executor'] for r in commands]==['0x1412dd080','0x1412dad40']
out={'source_sha256':im.sha,'scope':'original complete existing-sheep-mesh startup path incl native SheepControl::new and native BirdSpotManager::create_spots; file/compiler/library/Bevy spawn are recording boundaries; not GPU evidence','host_calls':calls,'sheep':sheep,'control':controls,'debug_commands':commands,'bird_next_id':u64(manager,8),'pool_loan_count':u64(pool,8),'bridges':im.bridges}
(root/'sheep-spawn-native-policy.json').write_text(json.dumps(out,indent=2));print('original existing-mesh sheep startup: 3 complete sheep bundles, native control +2debug commands passed')
for entry in keep:
 if isinstance(entry,tuple):im.k.VirtualFree(entry[1],0,0x8000)
im.close()
''';p.with_name('probe_sheep_spawn.py').write_text(s,encoding='utf-8')
