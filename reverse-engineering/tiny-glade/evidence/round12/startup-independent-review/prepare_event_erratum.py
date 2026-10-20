from pathlib import Path
import difflib, hashlib, json

here = Path(__file__).resolve().parent
root = here.parents[4]
groups = []
def group(module, names):
    groups.append((module, names.split()))

group('res::walls::public_walls', 'OnWallChanged OnWallDeleted')
group('sys::wall::calc_wall_bend', 'WallBendChangedEvent')
group('sys::wall::construct_wall_bricks', 'WallBricksChanged')
group('res::rt', 'UpdateRtWorldEvent')
group('sys::wall::adjust_wall_height', 'AdjustWallHeightCmd WallHeightAudio WallHeightCommandFailed')
group('sys::wall::snapping', 'DisableSnapping')
group('sys::wall::eraser', 'AnimateWallBulldozeCmd WallBulldozeAudio WallEraserStroke')
group('sys::audio', 'PlayWallUiSound WallBendAudioParamReset RequestWallMaterialAudio')
group('sys::wall::shapes', 'CreateCircleCmd CreateRectangleCmd EditCircleCmd EditRectangleCmd FinishEditCircleCmd FinishEditRectangleCmd RotateCircleCmd')
group('sys::wall::history_recording', 'RecordWallHistory RecordColorHistory')
group('sys::camera_rig::main_camera', 'MainCameraMoved')
group('sys::wall::draw_wall', 'RecordMouseInputIntoWall')
group('sys::wall::finish_wall', 'FinishWall')
group('res::color_id', 'SetWallColorIdCmd SetGateColorIdCmd SetHalftimberColorIdCmd')
group('res::tree_storage', 'HideTreesNextToWallCmd HideTreesinAreaCmd MoveTreeCmd')
group('res::glade_border', 'DisplayOutOfBorder FeedbackInputOutOfBorder NotInDemoHint')
group('res::onboarding', 'NotifyHintSystemToolWasJustUsed')
group('sys::history::history_process', 'DebugHistoryMsg')
# RecalculateRoofCmd aliases RoofGeometryChanged; the wall constructor's
# RecalculateWallBricks aliases this exact roof-owned event. One entry each.
group('res::roofs::commands', 'CreateRoofCmd DeleteRoofCmd SwitchRoofDirectionCmd SetFlatRoofStyleCmd SetFlatRoofRimStyleCmd EditRoofCmd RoofIsInvalid ResetRoofs WallRoofPresenceChanged WallGableChanged RoofGeometryChanged RecalculateWallHoles RecalculateWallBricks SwitchRoofTypeCmd RoofSound RecordRoofHistory RoofEditFeedback ReplayRoofEvent')
group('system_roof::ui_edit_roof_ridge_dir', 'SwitchRoofRidgeDirCmd RecordRidgeSwitchHistoryCmd')
group('system_color', 'ShowColorMenuCmd HideColorMenuCmd SetRoofColorIdCmd SetDecoratorColorIdCmd SetDecoratorBrightnessCmd SetLadderColorIdCmd SetStairRailingCmd SetMagicColorIdCmd SetWindowLightCmd SetFlagConnectionCmd SetChimneySmokeCmd')
group('sys::wall', 'SetFreehandHeightTypeCmd SetPlasterPeelAmountCmd SetWallStyleCmd')
group('sys::ui::resources::widget_params', 'PlayUiSound')
group('system_tree', 'PlaceTreeCmd')
group('res::history::tree_edit', 'InsertTreeCmd DeleteTreeCmd SpawnPreplacedTrees')
group('toolbar::ui_interaction', 'UiClickEvent UiDragEvent UiJustBecameInFocusEvent')
group('toolbar::toolbar_button', 'UiToolbarButtonResetAllCooldowns')
group('res::events', 'ModeJustChanged')
group('sys::mode_manager', 'RequestToolbarCategoryChange RequestAppModeChange')
group('sys::backend::input::calculate_input_delta', 'UiHistoryButtonPressed')
group('system_decorator::handle_external_decorator_cmds', 'ExternalDecoratorCmd')
group('sys::save::save_game', 'SaveGameCmd SavingGameComplete DeleteSessionCmd')
group('sys::save::load_game', 'LoadingGameComplete StartupRevealStarted StartupRevealFinished')
group('sys::save::load_completion_checker', 'DisplayCorruptedSaveFileWarning')
group('res::session', 'NewSessionStartedCmd SessionLoadedCmd')

body = '''//! Register real Bevy event maintenance before resources-only domain helpers.
//! Bevy 0.16 App::add_event skips Events<T> that already exists. A helper's
//! init_resource<Events<T>> therefore cannot precede this App registration.
//! This list uses canonical domain identities and leaves all consumers intact.
use bevy_app::App;
use country_core::{resources as res, systems as sys};

pub(super) fn install(app: &mut App) {
'''
types = []
for module, names in groups:
    body += '    // ' + module + '\n'
    for name in names:
        ty = module + '::' + name
        assert ty not in types
        types.append(ty)
        body += f'    app.add_event::<{ty}>();\n'
body += '}\n'

relative = 'tiny-glade/crates/tiny-glade/src/startup_event_runtime.rs'
updates = {relative: (b'', body.encode())}
run_rel = 'tiny-glade/crates/tiny-glade/src/run_game.rs'
before = (root / run_rel).read_bytes()
text = before.decode().replace('\r\n','\n')
anchor = '#[path = "startup_construction_runtime.rs"]\nmod startup_construction_runtime;'
assert text.count(anchor) == 1
text = text.replace(anchor, '#[path = "startup_event_runtime.rs"]\nmod startup_event_runtime;\n\n' + anchor)
# Only the gameplay App runs color/tree/roof/wall/toolbar/decorator/session
# helpers. The loading App's input helper already registers via EventRegistry.
anchor = '    let mut app = App::new();\n'
assert text.count(anchor) >= 2
text = text.replace(anchor, anchor + '    startup_event_runtime::install(&mut app);\n', 1)
after = text.replace('\n','\r\n').encode() if b'\r\n' in before else text.encode()
updates[run_rel] = (before, after)
patch = ''
manifest = []
for rel, (old, new) in updates.items():
    target = here/'event-candidate'/rel
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_bytes(new)
    patch += ''.join(difflib.unified_diff(old.decode().splitlines(True), new.decode().splitlines(True), '/dev/null' if not old else 'a/'+rel, 'b/'+rel))
    manifest.append({'path':rel,'base_sha256':hashlib.sha256(old).hexdigest() if old else None,'candidate_sha256':hashlib.sha256(new).hexdigest()})
(here/'event-maintenance-erratum.patch').write_bytes(patch.encode())
(here/'event-maintenance-erratum.json').write_text(json.dumps({'canonical_event_types':types,'count':len(types),'files':manifest,'source':'Locked bevy_app/bevy_ecs 0.16.0 App::add_event and EventRegistry; no engine reconstruction','ordering':'Install immediately after gameplay App::new, before the first World registration helper'},indent=2),encoding='utf8')
print(f'Generated {len(types)} explicit event registrations in two-file candidate')
