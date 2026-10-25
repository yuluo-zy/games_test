from pathlib import Path
import hashlib, json, difflib, datetime

ROOT = Path('D:/game/tiny-glade')
OUT = Path(__file__).parent
files = {}

def take(rel):
    data = (ROOT / rel).read_bytes()
    return data, data.decode().replace('\r\n', '\n')

def save(rel, old, text):
    # Keep current per-file newline convention, without rustfmt or source writes.
    new = text.replace('\n', '\r\n').encode() if b'\r\n' in old else text.encode()
    target = OUT / 'candidate' / rel
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_bytes(new)
    files[rel] = dict(base_sha256=hashlib.sha256(old).hexdigest(), candidate_sha256=hashlib.sha256(new).hexdigest())
    return ''.join(difflib.unified_diff(old.decode().splitlines(True), new.decode().splitlines(True), 'a/'+rel, 'b/'+rel))

patches = []
rel = 'crates/tiny-glade/src/run_game.rs'
old, src = take(rel)
stage_marker = '        use country_core::systems::wall::registration::WallCpuStage;'
if stage_marker in src:
    start = src.index(stage_marker)
    end = src.index('\n    app.insert_resource(window);', start)
    assert src[start-6:start] == '    {\n'
    src = src[:start-6] + '    install_verified_constructor_slice(&mut app);\n' + src[end:]
else:
    # A concurrent source revision removed the late plugin group. Restore only
    # its genuine owner setup and reviewed forwarding; never restore old wall CPU.
    anchor = '    install_history_and_wall_runtime(&mut app, config.dev.demo);'
    assert src.count(anchor) == 1
    src = src.replace(anchor, anchor + '''
    app.add_plugins(crate::toolbar_plugin::ToolbarPlugin);
    crate::decorator_plugin::build(&mut app);
    crate::wall_constructor_plugin::build(&mut app);
    install_verified_constructor_slice(&mut app);''',1)
old_shader = '''#[derive(Resource, Clone)]
pub struct ApplicationShaderLibrary(
    pub Arc<country_core::asset_libraries::shader_library::AssetShaderLibrary>,
);'''
assert old_shader in src
src = src.replace(old_shader, 'pub use country_core::asset_libraries::shader_library::SharedAssetShaderLibrary as ApplicationShaderLibrary;', 1)
available = (OUT.parent/'app-edit-recording/available-registration.rs').read_text()
available = available[available.index('pub fn install_available_wall_recording'):]
available = available.replace('pub fn install_available_wall_recording', 'fn install_available_wall_recording', 1)
# Use qualified imports in this local adapter, leaving the production module imports intact.
available = available.replace('    wall::registration::register_wall_drawing_resources', '''    use bevy_app::Update;
    use country_core::resources::history::MaybePendingSnapshotId;
    use country_core::systems::{history, wall};
    wall::registration::register_wall_drawing_resources''',1)
src += '\n' + available
src += '''
/// Evidence-backed partial native registration. Missing platform/pillar/floor
/// producers are explicit gaps, never mounted empty legacy leaves.
fn install_verified_constructor_slice(app: &mut App) {
    use bevy_app::Update;
    use country_core::systems::wall;
    wall::registration::register_wall_cpu_resources(app.world_mut());
    app.init_resource::<country_core::resources::walls::wooden_gates::Gates>();
    app.init_resource::<country_core::resources::walls::arches::ArchSegments>();
    app.init_resource::<country_core::resources::walls::arches::BuildingHadArchesLastFrame>();
    app.add_event::<system_wall_constructor::construct_gates::systems::ConstructGateCmd>();
    app.add_event::<wall::calc_wall_bend::WallBendChangedEvent>();
    app.add_event::<wall::history_recording::RecordWallHistory>();
    app.add_event::<wall::construct_wall_bricks::WallBricksChanged>();
    app.add_event::<country_core::resources::history::FinishInputRecordingCmd>();
    app.add_event::<wall::SetWallStyleCmd>();
    app.add_event::<wall::SetPlasterPeelAmountCmd>();
    app.add_event::<wall::SetFreehandHeightTypeCmd>();
    app.add_event::<country_core::systems::ui::resources::widget_params::PlayUiSound>();
    app.add_event::<toolbar::ui_interaction::UiDragEvent>();
    app.add_event::<toolbar::ui_interaction::UiJustBecameInFocusEvent>();
    app.add_event::<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>();
    app.add_event::<country_core::resources::events::ModeJustChanged>();
    app.add_event::<country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed>();
    install_available_wall_recording(app);
    app.add_systems(Update, (
        wall::calc_wall_bend::calc_wall_bend,
        system_wall_constructor::gpu_output::clear_constructor_planks,
        system_wall_constructor::segmentation::calculate_wall_path_segmentation::calculate_wall_path_segmentation,
        system_wall_constructor::init_wall_height_and_arch_segment_height::init_wall_height_and_arch_segment_height,
    ).chain_ignore_deferred().after(wall::wall_state::publish_public_wall_state::publish_public_wall_state)
        .in_set(GameSystemSet(19)));
    app.add_systems(Update, (
        system_roof::create_roof::wall_roof_hashmap,
        system_roof::delete_roof::process_invalid_roof_notifications,
        system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs,
        system_wall_constructor::construct_arches::construct_arches,
        system_wall_constructor::construct_lintels::construct_lintels,
    ).chain_ignore_deferred().in_set(GameSystemSet(22)));
    app.add_systems(Update, (
        system_wall_constructor::construct_gates::systems::update_gates_hashmap,
        system_wall_constructor::construct_gates::systems::construct_gates,
        system_wall_constructor::construct_fences::construct_fences,
        wall::construct_wall_bricks::finalize_wall_holes,
        wall::construct_wall_bricks::construct_wall_bricks,
    ).chain_ignore_deferred().in_set(GameSystemSet(28)));
    app.add_systems(Update,
        system_wall_constructor::gpu_output::flush_constructor_output.in_set(GameSystemSet(44)));
}
'''
patches.append(save(rel, old, src))

rel = 'crates/tiny-glade/src/history_runtime.rs'
old, src = take(rel)
start = src.index('    app.add_systems(\n        Update,\n        (\n            system_roof::create_roof::create_roof,')
end = src.index('    app.add_systems(\n        Update,\n        (\n            system_color::write_roof_color_id_to_gpu', start)
roof = (OUT.parent/'roof-visual/roof-registration.integration.rs').read_text(encoding='utf-8')
roof = roof[roof.index('app.add_systems(Update, ('):roof.index('// UI17/18')]
# Single replay was already installed immediately above the replaced CPU group.
roof = roof.replace('app.add_systems(Update,\n    system_roof::replay_roof::replay_roof.in_set(GameSystemSet(40)));\n', '')
preserved = '''
    // Existing real command adapters retained pending their original individual
    // caller-stage adjudication; these are not part of the native20 tuple.
    app.add_systems(Update, (
        system_roof::edit_roof::switch_roof_direction,
        system_roof::ui_edit_roof_ridge_dir::switch_roof_ridge_direction,
        system_roof::edit_roof::set_flat_roof_rim_style,
        system_roof::update_roof_spatial::update_wall_gable_heights,
    ).chain().after(system_roof::replay_roof::replay_roof).in_set(GameSystemSet(40)));
'''
src = src[:start] + '\n'.join('    '+line if line else '' for line in roof.splitlines())+'\n'+preserved + src[end:]
needle = '    app.add_event::<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>();'
assert needle in src
src = src.replace(needle, needle+'\n    app.add_event::<system_roof::ui_edit_roof_ridge_dir::RecordRidgeSwitchHistoryCmd>();',1)
src = src.replace('        RoofEditFeedback,', '        RoofEditFeedback,') # ordinary roof edit feedback is not T13
patches.append(save(rel, old, src))

# Exact suppression of the hidden duplicate replay; all other existing generic
# plugin leaves and owners are preserved for later original-stage remapping.
rel = 'crates/systems/decorator/src/lib.rs'
old, src = take(rel)
needle = '''pub fn add_decorator_systems<S: Send + Sync + 'static, C: Send + Sync + 'static>(
    app: &mut bevy_app::App,
) {
    use bevy_app::Update;'''
replacement = '''pub fn add_decorator_systems<S: Send + Sync + 'static, C: Send + Sync + 'static>(
    app: &mut bevy_app::App,
) {
    add_decorator_systems_impl::<S, C>(app, true);
}

/// The production host already owns canonical decorator replay in numeric40.
/// Preserve every other existing consumer without installing a second tick.
pub fn add_decorator_systems_without_replay<S: Send + Sync + 'static, C: Send + Sync + 'static>(
    app: &mut bevy_app::App,
) {
    add_decorator_systems_impl::<S, C>(app, false);
}

fn add_decorator_systems_impl<S: Send + Sync + 'static, C: Send + Sync + 'static>(
    app: &mut bevy_app::App, install_replay: bool,
) {
    use bevy_app::Update;'''
assert needle in src
src = src.replace(needle,replacement,1)
needle = '''    app.add_systems(
        Update,
        replay_decorator::replay_decorator
            .after(deferred_decorator_history::decorator_record_history::<S>),
    );'''
assert needle in src
src = src.replace(needle,'''    if install_replay {
        app.add_systems(
            Update,
            replay_decorator::replay_decorator
                .after(deferred_decorator_history::decorator_record_history::<S>),
        );
    }''',1)
patches.append(save(rel,old,src))
rel='crates/tiny-glade/src/decorator_plugin.rs'
old,src=take(rel)
needle='system_decorator::add_decorator_systems::<MaybePendingSnapshotId, CollisionPayload>(app);'
assert needle in src
src=src.replace(needle,'system_decorator::add_decorator_systems_without_replay::<MaybePendingSnapshotId, CollisionPayload>(app);',1)
patches.append(save(rel,old,src))

(OUT/'assembly-candidate.patch').write_text(''.join(patches),encoding='utf-8',newline='')
(OUT/'candidate-checkpoint.json').write_text(json.dumps(dict(created_at=datetime.datetime.now().astimezone().isoformat(),files=files,dependencies=['wall-gpu/candidates-v23 (SharedAssetShaderLibrary + typed pools + masks + stats)', 'Existing full roof production source', 'app-edit-recording actual available nonempty leaves'],status='Evidence only. Uncompiled. Event type names and all new SystemParams require central review against current candidates before apply.'),indent=2))
print(json.dumps(files,indent=2))
