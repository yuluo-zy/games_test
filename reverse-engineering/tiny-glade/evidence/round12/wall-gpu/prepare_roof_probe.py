from pathlib import Path
p=Path('reverse-engineering/tiny-glade/evidence/round12/wall-gpu/probe_wall_startup.py');s=p.read_text(encoding='utf-8-sig');s=s[:s.index('fn=ctypes.WINFUNCTYPE')]+'''fn=ctypes.WINFUNCTYPE(None,*([ctypes.c_void_p]*6))(im.address(0x1414547d0));fn(*args)
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
''';p.with_name('probe_roof_startup.py').write_text(s,encoding='utf-8')
