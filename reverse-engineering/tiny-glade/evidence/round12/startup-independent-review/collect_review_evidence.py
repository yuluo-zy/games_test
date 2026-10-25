from pathlib import Path
import hashlib,json,struct,subprocess
OUT=Path(__file__).resolve().parent
ROOT=OUT.parents[4]
EXE=ROOT/'reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe'
raw=EXE.read_bytes();assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=struct.unpack_from('<I',raw,60)[0];opt=pe+24;sh=opt+struct.unpack_from('<H',raw,pe+20)[0]
sections=[struct.unpack_from('<4I',raw,sh+i*40+8) for i in range(struct.unpack_from('<H',raw,pe+6)[0])]
def read(va,n):
 for _,rva,size,off in sections:
  if rva<=va-0x140000000 and va-0x140000000+n<=rva+size:
   start=off+va-0x140000000-rva;return raw[start:start+n]
 raise ValueError(hex(va))
functions={0x140a92670:2320,0x140864430:636,0x1414547d0:1680,0x140b1aca0:456,0x14089d6f0:448,0x140ae7810:1406,0x140b67b50:316,
 0x140493120:1413,0x1405389b0:1399,0x14033def0:2586,0x1400f50e0:971}
rows=[]
for va,n in functions.items():
 result=subprocess.run(['C:/Program Files/LLVM/bin/llvm-objdump.exe','-d',f'--start-address={hex(va)}',f'--stop-address={hex(va+n)}',str(EXE)],check=True,capture_output=True)
 (OUT/(hex(va)+'-original.asm')).write_bytes(result.stdout)
 rows.append({'va':hex(va),'size':n,'sha256':hashlib.sha256(read(va,n)).hexdigest()})
symbols=[]
with (ROOT/'reverse-engineering/tiny-glade/evidence/country-core-audit/data/procs.tsv').open(encoding='utf8') as f:
 for line in f:
  parts=line.rstrip('\n').split('\t',4)
  if len(parts)==5 and int(parts[0],16) in functions:symbols.append(parts)
(OUT/'native-review-identity.json').write_text(json.dumps({'original_pe_sha256':hashlib.sha256(raw).hexdigest(),'ranges':rows,'pdb_records':symbols},indent=2),encoding='utf8')
paths=[
 'wall-gpu/candidates-startup/crates/country-core/src/startup/startup_walls.rs',
 'wall-gpu/candidates-startup/crates/country-core/src/systems/render_loop/draw_passes/draw_callbacks_a_views.rs',
 'wall-gpu/candidates-startup/crates/country-core/src/geometry/instanced_wall.rs',
 'wall-gpu/candidates-sky/crates/country-core/src/startup/startup_sky_and_sun.rs',
 'wall-gpu/candidates-roof-startup/crates/country-core/src/startup/startup_roof.rs',
 'independent-flow-wiring/startup-constructor-candidate/crates/tiny-glade/src/startup_construction_runtime.rs',
 'independent-flow-wiring/startup-constructor-candidate/crates/tiny-glade/src/run_game.rs',
 'independent-flow-wiring/startup-constructor-candidate/crates/tiny-glade/src/history_runtime.rs',
 'independent-flow-wiring/startup-constructor-candidate/crates/country-core/src/systems/wall/construct_wall_bricks.rs',
 'independent-flow-wiring/tod-startup-candidate/crates/country-core/src/startup/startup_time_of_day.rs',
]
(OUT/'reviewed-source-hashes.json').write_text(json.dumps({p:hashlib.sha256((OUT.parent/p).read_bytes()).hexdigest() for p in paths},indent=2),encoding='utf8')
for domain in ['wall','sky','roof']:
 source=OUT.parent/'wall-gpu'/('probe_'+domain+'_startup.py')
 wrapper='import sys\nfrom pathlib import Path\nsys.path.insert(0,'+repr(str(source.parent))+')\nexec(compile(Path('+repr(str(source))+').read_text(encoding="utf8"),'+repr(str(source))+',"exec"),{"__file__":__file__,"__name__":"__main__"})\n'
 (OUT/('review_'+domain+'_policy.py')).write_text(wrapper,encoding='utf8')
print('11 complete original review ranges,10 reviewed source hashes and independent-output execution wrappers ready')
