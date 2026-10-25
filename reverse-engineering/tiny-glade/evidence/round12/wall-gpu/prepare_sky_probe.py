from pathlib import Path
p=Path('reverse-engineering/tiny-glade/evidence/round12/wall-gpu/probe_wall_startup.py');s=p.read_text(encoding='utf-8-sig');s=s[:s.index('fn=ctypes.WINFUNCTYPE')]+'''fn=ctypes.WINFUNCTYPE(None,*([ctypes.c_void_p]*6))(im.address(0x140864430));fn(*args)
assert u64(queue,16)==120,u64(queue,16)
p=data
record={'executor':hex(u64(p)-im.base+0x140000000),'transform':ctypes.string_at(p+8,40).hex(),'transform_u32':list(struct.unpack('<10I',ctypes.string_at(p+8,40))),'mesh':hex(u64(p,56)),'shader':hex(u64(p,64)),'entity':u64(p,88)}
assert record['executor']=='0x1412dd460'
assert record['transform_u32']==[0,0,0,0x3f800000,0x80000000,0xc0000000,0x80000000,0x3f800000,0x3f800000,0x3f800000],record
assert len(meshes)==1 and meshes[0]['name']=='skydome' and len(shaders)==1 and shaders[0]['name']=='skydome_shader'
out={'source_sha256':im.sha,'range':'140864430..1408646ac','scope':'complete original startup policy; explicit recording file/library/compiler/ECS boundaries, not GPU evidence','host_calls':calls,'command_record':record,'bridges':im.bridges}
(root/'sky-startup-native-policy.json').write_text(json.dumps(out,indent=2));print('original sky startup: mesh,shader,single DrawableMeshBundle; exact (-0,-2,-0) translation passed')
im.close()
''';p.with_name('probe_sky_startup.py').write_text(s,encoding='utf-8')

