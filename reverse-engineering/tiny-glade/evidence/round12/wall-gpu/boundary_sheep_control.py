from pathlib import Path
p=Path('reverse-engineering/tiny-glade/evidence/round12/wall-gpu/probe_sheep_spawn.py');s=p.read_text(encoding='utf-8');at="sheep=[]\n";repl='''def cube(out,box):
 calls.append({'boundary':'existing geometry::cube::from','bounds':list(struct.unpack('<6f',ctypes.string_at(box,24)))})
 ctypes.memset(out,0,120);return out
im.dependency_callback(0x140a05530,ctypes.WINFUNCTYPE(ctypes.c_void_p,ctypes.c_void_p,ctypes.c_void_p),cube,'recording current canonical cube geometry producer boundary')
sheep=[]
''';assert at in s;s=s.replace(at,repl).replace("incl native SheepControl::new and native BirdSpotManager::create_spots", "incl native SheepControl::new host policy and native BirdSpotManager::create_spots; cube geometry is an explicit current-owner boundary");p.write_text(s,encoding='utf-8')
