"""Evidence-only resource setup; never mounts the old UI/system helpers."""
from pathlib import Path
import hashlib,json,difflib,datetime
root=Path('D:/game/tiny-glade');out=Path(__file__).parent/'startup-resources-candidate'
rows={};patch=[]
def save(rel,new):
 old=(root/rel).read_bytes();p=out/rel;p.parent.mkdir(parents=True,exist_ok=True);p.write_bytes(new)
 rows[rel]={'base_sha256':hashlib.sha256(old).hexdigest(),'candidate_sha256':hashlib.sha256(new).hexdigest()}
 patch.append(''.join(difflib.unified_diff(old.decode().splitlines(True),new.decode().splitlines(True),'a/'+rel,'b/'+rel)))
def eol_at(raw,pos):
 end=raw.index(b'\n',pos);return b'\r\n' if raw[end-1:end]==b'\r' else b'\n'
rel='crates/systems/decorator/src/ui_systems/mod.rs';raw=(root/rel).read_bytes()
start=raw.index(b'pub fn register_ui_systems<');brace=raw.index(b'{',start);body=raw.index(b'\n',brace)+1
systems=raw.index(b'    app.add_systems(',body);setup=raw[body:systems];nl=eol_at(raw,start)
helper=b'pub fn register_ui_resources(app: &mut bevy_app::App) {'+nl+setup+b'}'+nl+nl
helper+=b'/// Historical UI registration; startup calls only register_ui_resources.'+nl
new=raw[:start]+helper+raw[start:body]+b'    register_ui_resources(app);'+nl+raw[systems:]
save(rel,new)
rel='crates/tiny-glade/src/decorator_plugin.rs';raw=(root/rel).read_bytes();nl=eol_at(raw,0)
new=raw+('''
/// Genuine startup owners, independent of incomplete edit/UI system adapters.
/// No updater, replay, or UI leaf is mounted by this resource constructor.
pub fn build_startup_resources(app: &mut bevy_app::App) {
    system_decorator::add_decorator_events_and_resources(app.world_mut());
    system_decorator::ui_systems::register_ui_resources(app);
    app.init_resource::<country_core::resources::walls::decorator_archivist::LiveDecoratorArchivist>();
    app.add_event::<system_decorator::handle_external_decorator_cmds::ExternalDecoratorCmd>();
}
''').encode().replace(b'\n',nl)
save(rel,new)
rel='crates/tiny-glade/src/run_game.rs';raw=(root/rel).read_bytes()
needle=b'    install_history_and_wall_runtime(&mut app, config.dev.demo);';assert raw.count(needle)==1
nl=eol_at(raw,raw.index(needle));addition=('''
    // Preserve actual domain owner constructors without mounting incomplete
    // edit/UI helpers merely to initialize their resources.
    toolbar::configure_app(&mut app);
    crate::decorator_plugin::build_startup_resources(&mut app);
    crate::wall_constructor_plugin::build(&mut app);
    app.add_event::<toolbar::ui_interaction::UiDragEvent>();
    app.add_event::<toolbar::ui_interaction::UiJustBecameInFocusEvent>();
    app.add_event::<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>();
    app.add_event::<country_core::resources::events::ModeJustChanged>();
    app.add_event::<country_core::systems::ui::resources::widget_params::PlayUiSound>();
    app.add_event::<country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed>();
''').encode().replace(b'\n',nl)
new=raw.replace(needle,needle+addition,1)
save(rel,new)
(out.parent/'startup-resources-candidate.patch').write_text(''.join(patch),encoding='utf-8',newline='')
(out.parent/'startup-resources-candidate-manifest.json').write_text(json.dumps({'created_at':datetime.datetime.now().astimezone().isoformat(),'files':rows,'status':'Resources only. No production write, Cargo, runtime UI, or algorithm acceptance.'},indent=2),encoding='utf-8')
print(json.dumps(rows,indent=2))
