"""Standalone full forest-audio pool candidate. Does not touch shared audio core."""
from pathlib import Path
import difflib,hashlib,json,subprocess
ROOT=Path('D:/game');HERE=Path(__file__).resolve().parent
rel='tiny-glade/crates/country-core/src/systems/audio/forest_audio.rs'
source=ROOT/rel;old=source.read_text(encoding='utf8')
new=(HERE/'forest-audio-candidate.rs').read_text(encoding='utf8')
target=HERE/'forest-audio-candidate'/rel
target.parent.mkdir(parents=True,exist_ok=True);target.write_text(new,encoding='utf8')
subprocess.run(['C:/Users/liyu/.rustup/toolchains/stable-x86_64-pc-windows-msvc/bin/rustfmt.exe',
    '--edition','2024','--config','skip_children=true','--emit','stdout',str(target)],check=True,stdout=subprocess.DEVNULL)
patch=HERE/'forest-audio-pool-candidate.patch'
patch.write_text(''.join(difflib.unified_diff(old.splitlines(True),new.splitlines(True),fromfile='a/'+rel,tofile='b/'+rel)),encoding='utf8')
subprocess.run(['git','-c','safe.directory=D:/game','-c','core.protectNTFS=false','apply','--check','--ignore-whitespace',str(patch)],cwd=ROOT,check=True)
manifest={'production_files_changed':0,'depends_on':['clearing ForestTree/InteractableForestTree canonical types',
    'independent_ui_acceptance canonical AudioCore::instantiate_loop count+1/50ms/error semantics'],
    'execution':'rustfmt parsing and git apply --check only; no Cargo',
    'files':{rel:{'base_sha256':hashlib.sha256(source.read_bytes()).hexdigest(),
        'candidate_sha256':hashlib.sha256(target.read_bytes()).hexdigest()}},
    'patch_sha256':hashlib.sha256(patch.read_bytes()).hexdigest()}
(HERE/'forest-audio-pool-candidate-manifest.json').write_text(json.dumps(manifest,indent=2),encoding='utf8')
print(json.dumps({'production_files_changed':0,'files':1,'patch_sha256':manifest['patch_sha256']},indent=2))
