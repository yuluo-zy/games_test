from pathlib import Path
import hashlib,json,shutil,sys
R=Path('D:/game');O=Path(__file__).resolve().parent
rel='tiny-glade/crates/tiny-glade/tests/startup_session_lifecycle.rs'
src=O/'candidate'/rel
assert not (R/rel).exists(), 'New-file production target already exists; review before overwriting'
f=O/(sys.argv[1] if len(sys.argv)>1 else 'frozen-v2');f.mkdir(exist_ok=True)
dst=f/'candidate'/rel;dst.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(src,dst)
raw=dst.read_bytes();lines=raw.decode().splitlines()
patch='*** Begin Patch\n*** Add File: '+rel+'\n'+''.join('+'+line+'\n' for line in lines)+'*** End Patch\n'
(f/'startup-acceptance.patch').write_text(patch,encoding='utf8',newline='\n')
sources=['tiny-glade/crates/tiny-glade/src/run_game.rs','tiny-glade/crates/tiny-glade/src/gameplay_startup.rs',
'tiny-glade/crates/tiny-glade/src/game_loop.rs','tiny-glade/crates/country-core/src/startup/startup_session.rs',
'tiny-glade/crates/country-core/src/startup/clearing.rs','tiny-glade/crates/country-core/src/startup/spawn_preplaced_trees.rs',
'tiny-glade/crates/country-core/src/startup/startup_meadow.rs','tiny-glade/crates/country-core/src/startup/init_photomode_state.rs',
'tiny-glade/crates/country-core/src/systems/save/load_game.rs','tiny-glade/crates/country-core/src/systems/audio/game_loading_audio.rs',
'tiny-glade/crates/country-core/src/asset_libraries/placement_data_library.rs','tiny-glade/crates/country-core/src/systems/time_update.rs',
'tiny-glade/crates/country-core/src/systems/backend/photomode.rs','tiny-glade/crates/country-core/src/systems/backend/input/reset_input_state.rs']
manifest={'status':'candidate only; rustfmt parsed; not compiled or executed; root owns Cargo',
'target':rel,'base_sha256':None,'proposed_sha256':hashlib.sha256(raw).hexdigest(),
'patch_sha256':hashlib.sha256((f/'startup-acceptance.patch').read_bytes()).hexdigest(),
'source_audit':{p:hashlib.sha256((R/p).read_bytes()).hexdigest() for p in sources}}
(f/'manifest.json').write_text(json.dumps(manifest,indent=2),encoding='utf8')
assert dst.read_bytes()==src.read_bytes()
assert ('\n'.join(line[1:] for line in patch.splitlines()[2:-1])+'\n').encode()==raw
print(json.dumps(manifest,indent=2))
