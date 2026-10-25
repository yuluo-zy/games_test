from pathlib import Path
import hashlib,difflib,json
r=Path('D:/game');out=r/'reverse-engineering/tiny-glade/evidence/round12/wall-gpu/sheep-shared-current';out.mkdir(exist_ok=True)
changes=[];patch=[]
for rel in ['crates/country-core/src/startup/run_game.rs','crates/systems/critters/src/components.rs','crates/systems/critters/src/lib.rs']:
 p=r/'tiny-glade'/rel;before=p.read_bytes();s=before.decode('utf-8');nl='\r\r\n' if '\r\r\n' in s else '\r\n' if '\r\n' in s else '\n'
 if rel.endswith('run_game.rs'):
  assert 'debug::sheep_controls::{SheepControl, sheep_controls}' in s
  s=s.replace('debug::sheep_controls::{SheepControl, sheep_controls}','debug::sheep_controls::sheep_controls')
  a=s.index('    world.init_resource::<SheepControl>();');z=s.index('\n',a)+1;s=s[:a]+s[z:]
 elif rel.endswith('components.rs'):
  if 'pub struct Wren {' in s:
   a=s.index('pub struct Wren {');a=s.rfind('#[derive',0,a);z=s.index('\n}',a)+2
   s=s[:a]+'pub use country_core::resources::critters::Wren;'+s[z:]
  a=s.index('pub struct BirdSpots {');a=s.rfind('#[derive',0,a);z=s.index('pub struct BirdSpot {',a);z=s.index('\n}',z)+2
  s=s[:a]+'pub use crate::bird_spots::{BirdSpot, BirdSpots};'+s[z:]
  a=s.index('pub struct PrevTransform {');a=s.rfind('#[derive',0,a);z=s.index('impl Default for Duck {',a)
  s=s[:a]+'pub use country_core::components::drawable::PrevTransform;'+nl+nl+s[z:]
 else:
  assert 'pub mod startup_sheep;' not in s;s=s.replace('pub mod sheep;','pub mod sheep;'+nl+'pub mod startup_sheep;')
 after=s.encode('utf-8');pout=out/rel;pout.parent.mkdir(parents=True,exist_ok=True);pout.write_bytes(after)
 changes.append({'path':rel,'base_sha256':hashlib.sha256(before).hexdigest(),'candidate_sha256':hashlib.sha256(after).hexdigest(),'original_newline':repr(nl),'bytes':len(after)})
 # Compare logical lines so CRCRLF is preserved in files and doesn't corrupt a textual patch.
 clean=lambda x:x.replace('\r\r\n','\n').replace('\r\n','\n').splitlines(True)
 patch.extend(difflib.unified_diff(clean(before.decode()),clean(s),fromfile='a/tiny-glade/'+rel,tofile='b/tiny-glade/'+rel,n=3))
(out/'manifest.json').write_text(json.dumps(changes,indent=2),encoding='utf-8');(out/'narrow.patch').write_text(''.join(patch),encoding='utf-8')
print('3 exact current-byte candidates; narrow patch',len(''.join(patch).splitlines()),'lines')
