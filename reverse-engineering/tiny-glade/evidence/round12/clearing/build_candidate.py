from pathlib import Path
import difflib,json,hashlib
HERE=Path(__file__).resolve().parent
ROOT=HERE.parents[4]
base=ROOT/'tiny-glade'
candidate=HERE/'candidate/tiny-glade'
files=[]
def put(relative,text):
    path=candidate/relative;path.parent.mkdir(parents=True,exist_ok=True);path.write_text(text,encoding='utf8');files.append(relative)
tree='crates/country-core/src/resources/tree_storage.rs'
original=(base/tree).read_text(encoding='utf8')
start=original.index('/// 可交互森林树')
end=original.index('/// 树移动命令',start)
put(tree,original[:start]+(HERE/'forest_types.rs').read_text(encoding='utf8')+'\n\n'+original[end:])
clearing='crates/country-core/src/startup/clearing.rs'
text=(base/clearing).read_text(encoding='utf8')
text='''//! Original clearing lifecycle: complete per-session producer below, plus
//! the existing canonical unload/despawn owners. The small asset inventory
//! helper is retained for legacy CPU-only tests; actual gameplay registers
//! init_clearing_complete on every OnEnter(Game), without run_once.
'''+text[text.index('use crate::asset_libraries::mesh_library::AssetMeshLibrary;'):]
text=text.replace('pub fn init_clearing<H:', 'pub fn clearing_asset_inventory<H:')
text=text.replace('init_clearing(&mut summer, false)', 'clearing_asset_inventory(&mut summer, false)')
text=text.replace('init_clearing(&mut winter, true)', 'clearing_asset_inventory(&mut winter, true)')
text=text.replace('// U: 两条 spawn 的 bundle/pass；主题产者与 trees spawn 不在 init_clearing 证据内。', '// Asset inventory only; full spawning and theme policy are below.')
put(clearing,text+'\n\n'+(HERE/'init_clearing_complete.rs').read_text(encoding='utf8').replace('//!','//'))
put('crates/country-core/src/startup/native_placement_policy.rs',(HERE/'native_placement_policy.rs').read_text(encoding='utf8'))
audio='crates/country-core/src/systems/audio/game_loading_audio.rs'
text=(base/audio).read_text(encoding='utf8')
assert 'records:vec![InteractableForestTree::default()]' in text
text=text.replace('tree_storage::{InteractableForestPlacements,InteractableForestTree}', 'tree_storage::{InteractableForestPlacements,ForestTree}')
text=text.replace('records:vec![InteractableForestTree::default()]', '''records:vec![ForestTree {
            prefab_name:crate::resources::render::mesh_atlas_library::PrefabName::new("clearing/tree/mid1"),
            placement:crate::resources::placement_data::PlacementDataSsbo {
                position:glam::Vec3::new(70.0,2.0,30.0),scale:1.25,rotation_angle:0.5,id:7,
                custom0:0,custom1:0,anim_t_packed:0x3c003c00,bend_vector:glam::Vec2::ZERO,pad0:0,
            },entity:None,billboard_expansion_scale:2.2,
        }]''')
put(audio,text)
runtime='crates/tiny-glade/src/gameplay_startup.rs'
text=(base/runtime).read_text(encoding='utf8')
marker='    let meadow_root = installed.clone();'
assert marker in text
registration='''    // Original clearing is a per-session producer. AssetPlacements and forest
    // records are cleared by OnExit(Game), then rebuilt for the new theme.
    app.init_resource::<country_core::systems::render_loop::compute_passes::leaf_particles::LoadedParticleEmitterCount>();
    let clearing_root = installed.clone();
    let clearing_backend = hosts.shaders.clone();
    let clearing_device = hosts.buffers.clone();
    app.add_systems(
        bevy_state::state::OnEnter(country_core::app_loading_state::AppLoadingState::Game),
        (move |session: Res<country_core::resources::session::ActiveSession>,
               meshes: Res<AssetMeshLibrary>, watch: Res<MeshWatch>,
               shaders: Res<ApplicationShaderLibrary>,
               mut placements: ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>,
               mut ssbos: ResMut<AssetSsboLibrary>,
               mut count: ResMut<country_core::systems::render_loop::compute_passes::leaf_particles::LoadedParticleEmitterCount>,
               mut forest: ResMut<country_core::resources::tree_storage::InteractableForestPlacements>,
               mut commands: Commands| {
            country_core::startup::clearing::init_clearing_complete(
                &clearing_root, &session, clearing_device.as_ref(), clearing_backend.as_ref(),
                &meshes, &watch, &shaders.0, &mut placements, &mut ssbos,
                &mut count, &mut forest, &mut commands,
            ).expect("original complete per-session clearing startup");
        }).after(country_core::startup::startup_session::load_game_session),
    );
'''
put(runtime,text.replace(marker,registration+marker))
runtime='crates/tiny-glade/src/run_game.rs'
text=(base/runtime).read_text(encoding='utf8')
legacy='            country_core::startup::clearing::init_clearing_meshes,\n'
assert text.count(legacy)==1
put(runtime,text.replace(legacy,''))
rows=[];patch=[]
for relative in files:
    src=base/relative;dst=candidate/relative
    before=src.read_text(encoding='utf8') if src.exists() else ''
    after=dst.read_text(encoding='utf8')
    patch.extend(difflib.unified_diff(before.splitlines(True),after.splitlines(True),fromfile=('a/tiny-glade/'+relative) if src.exists() else '/dev/null',tofile='b/tiny-glade/'+relative))
    rows.append({'file':'tiny-glade/'+relative,'production_sha256':hashlib.sha256(src.read_bytes()).hexdigest() if src.exists() else None,'candidate_sha256':hashlib.sha256(dst.read_bytes()).hexdigest()})
(HERE/'production-review.patch').write_text(''.join(patch),encoding='utf8')
(HERE/'candidate-files.json').write_text(json.dumps(rows,indent=2),encoding='utf8')
print(len(rows),'candidate files prepared; no production changes')
