"""Forest consumer changes against current production, canonical types external.

Requires round12/clearing/forest_types.rs producer owner; no source mutation.
"""
from pathlib import Path
import difflib,hashlib,json,subprocess
ROOT=Path('D:/game');HERE=Path(__file__).resolve().parent;OUT=HERE/'forest-consumer-candidate'
changes={}
rel='tiny-glade/crates/systems/forest/src/manage_tree_prefabs.rs'
old=(ROOT/rel).read_text(encoding='utf8')
new=(HERE/'forest-manage-candidate.rs').read_text(encoding='utf8')
changes[rel]=(old,new)
rel='tiny-glade/crates/systems/forest/src/native_forest_fixtures.rs'
changes[rel]=('',(HERE/'forest-native-fixtures.rs').read_text(encoding='utf8'))
rel='tiny-glade/crates/systems/forest/src/tree_animation.rs'
old=(ROOT/rel).read_text(encoding='utf8');start=old.index('pub fn forest_tree_animation(')
new=old[:start]+'''pub fn forest_tree_animation(
    mut trees:Query<(Entity,&mut InteractableForestTree,&mut PrefabInstanceData,Option<&TreeMarkedForDeletion>)>,
    time:Res<GameTime>,mut despawns:EventWriter<DespawnPrefabCmd>,
    mut placements:ResMut<InteractableForestPlacements>,
) {
    for (entity,mut tree,mut instance,marked) in &mut trees {
        let (step,growing)=if marked.is_some() {(time.delta / -0.3,false)}
        else if tree.delay>0.0 {tree.delay-=time.delta;(0.0,true)}
        else {(time.delta+time.delta,true)};
        let previous=tree.progress;
        // Native14227a68a..69f MAXSS 0,raw / MINSS 1,result retains NaN/-0.
        let raw=step+tree.progress;
        let low=if 0.0>raw {0.0}else{raw};
        tree.progress=if 1.0<low {1.0}else{low};
        if tree.progress!=previous {
            instance.0.scale=EasingFunction::CubicOut.apply(tree.progress)*tree.amplitude;
        }
        if !growing && tree.progress==0.0 {
            despawns.write(DespawnPrefabCmd{entity:Some(entity),_gap:0});
            // Original writes the record Entity's generation@44=0. The same
            // indexed owner must be released before the next manage pass.
            placements.records[tree.placement_index].entity=None;
        }
    }
}
'''
changes[rel]=(old,new)
manifest={'production_files_changed':0,'requires':'../clearing/forest_types.rs and complete producer candidate',
    'execution':'rustfmt parsing and git apply --check only; no Cargo','files':{}}
patch=[]
for rel,(old,new) in changes.items():
    p=OUT/rel;p.parent.mkdir(parents=True,exist_ok=True);p.write_text(new,encoding='utf8')
    subprocess.run(['C:/Users/liyu/.rustup/toolchains/stable-x86_64-pc-windows-msvc/bin/rustfmt.exe',
        '--edition','2024','--config','skip_children=true','--emit','stdout',str(p)],
        check=True,stdout=subprocess.DEVNULL)
    source=ROOT/rel
    manifest['files'][rel]={'base_sha256':hashlib.sha256(source.read_bytes()).hexdigest() if source.exists() else None,
        'candidate_sha256':hashlib.sha256(p.read_bytes()).hexdigest()}
    patch.extend(difflib.unified_diff(old.splitlines(True),new.splitlines(True),fromfile='a/'+rel if source.exists() else '/dev/null',tofile='b/'+rel))
p=HERE/'forest-consumer-candidate.patch';p.write_text(''.join(patch),encoding='utf8')
subprocess.run(['git','-c','safe.directory=D:/game','-c','core.protectNTFS=false','apply','--check','--ignore-whitespace',str(p)],cwd=ROOT,check=True)
manifest['patch_sha256']=hashlib.sha256(p.read_bytes()).hexdigest()
(HERE/'forest-consumer-candidate-manifest.json').write_text(json.dumps(manifest,indent=2),encoding='utf8')
print(json.dumps({'production_files_changed':0,'files':len(changes),'patch_sha256':manifest['patch_sha256']},indent=2))
