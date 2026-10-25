from pathlib import Path
import hashlib,json,difflib,datetime
root=Path('D:/game/tiny-glade');out=Path(__file__).parent/'completion-candidate'
rows={};diff=[]
def save(rel,text):
 p=root/rel;old=p.read_bytes() if p.exists() else b''
 data = text if isinstance(text,bytes) else (text.replace('\n','\r\n').encode() if b'\r\n' in old else text.encode())
 dst=out/rel;dst.parent.mkdir(parents=True,exist_ok=True);dst.write_bytes(data)
 rows[rel]={'base_sha256':hashlib.sha256(old).hexdigest() if old else None,'candidate_sha256':hashlib.sha256(data).hexdigest()}
 diff.append(''.join(difflib.unified_diff(old.decode().splitlines(True),data.decode().splitlines(True),'a/'+rel if old else '/dev/null','b/'+rel)))
rel='crates/country-core/src/systems/save/load_completion_checker.rs';src=(root/rel).read_text(encoding='utf-8')
src='''//! Original B completion observer140a9f810: consume LoadingGameComplete,
//! retain its sticky Local bit, and unlink loading.tmp once mode is neither
//! replay13 nor loading14. All unlink errors are dropped; mode_manager owns
//! the separate restore transition. The older pure filesystem-readiness helper
//! remains a compatibility API and must not be mistaken for this ECS consumer.
''' + src[src.index('use crate::'):]
needle='''pub struct LoadingCompletionState {
    /// 最近一次就绪度判定；游戏循环消费。'''
assert needle in src
src=src.replace(needle,'''pub struct LoadingCompletionState {
    /// Canonical original Local bit: once a LoadingGameComplete was observed,
    /// later nonloading/nonreplay modes retry marker deletion every update.
    observed_complete: bool,
    /// Historical global readiness projection retained for the older tree
    /// adapter; it is not read by the original completion observer.
    /// 最近一次就绪度判定；游戏循环消费。''',1)
src += '''
/// Original B140a9f810/608B, mounted in host Update set36.
/// Mode restoration belongs to mode_manager, not this marker observer. The
/// native unlink result is dropped for every error kind and the Local bit is
/// deliberately sticky, including across a later session.
pub fn check_load_completion_system(
    mode: bevy_ecs::prelude::Res<crate::systems::mode_manager::AppMode>,
    mut complete: bevy_ecs::prelude::EventReader<super::load_game::LoadingGameComplete>,
    mut local: bevy_ecs::prelude::Local<LoadingCompletionState>,
    session: bevy_ecs::prelude::Res<crate::resources::session::ActiveSession>,
) {
    if complete.read().last().is_some() {
        local.observed_complete = true;
    }
    if !local.observed_complete || matches!(mode.native_tag(), 13 | 14) {
        return;
    }
    let _ = std::fs::remove_file(session.id.save_folder_path().join("loading.tmp"));
}
'''
save(rel,src)
rel='crates/tiny-glade/src/run_game.rs';src=(root/rel).read_bytes()
needle=b'    // Original full-game registrations: weather in set36 and the complete'
assert needle in src
insertion='''    // Original14059fc33 supplies36 to typed in_set helper140524400.
    // No invented on_event/AppState gate: sticky Local retries after replay.
    app.add_systems(bevy_app::Update,
        country_core::systems::save::load_completion_checker::check_load_completion_system
            .in_set(GameSystemSet(36)));
'''
pos=src.index(needle);lineend=src.find(b'\n',pos)
eol=b'\r\n' if src[lineend-1:lineend]==b'\r' else b'\n'
src=src.replace(needle,insertion.encode().replace(b'\n',eol)+needle,1)
save(rel,src)
save('crates/tiny-glade/tests/original_load_completion.rs','''//! Actual ECS completion observer with a nonempty marker in an isolated account.
//! Original608B only unlinks after completion and a non13/14 mode; it does not
//! announce readiness, restore AppMode, or wait for converter/render work.
use bevy_app::{App, Update};
use country_core::{resources::session::{ActiveSession,SessionId,SessionMeta},
    systems::{mode_manager::AppMode,save::{load_game::LoadingGameComplete,
        load_completion_checker::check_load_completion_system}}};

#[test]
fn original_complete_event_sticky_local_mode_gate_and_unlink_error_policy() {
    let account=tempfile::tempdir().unwrap();
    paths::initialize_paths_with_installation(0,Some(account.path()),account.path()).unwrap();
    let id=SessionId::new();
    let folder=id.save_folder_path();
    std::fs::create_dir_all(&folder).unwrap();
    let marker=folder.join("loading.tmp");
    std::fs::write(&marker,b"actual unfinished load").unwrap();
    let mut app=App::new();
    app.add_event::<LoadingGameComplete>();
    app.insert_resource(ActiveSession::new(id,SessionMeta::default()));
    app.insert_resource(AppMode::default());
    app.add_systems(Update,check_load_completion_system);
    app.update();
    assert!(marker.exists(),"no observed complete event must preserve marker");
    app.insert_resource(AppMode::LoadGame);
    app.world_mut().send_event(LoadingGameComplete{restore_to:AppMode::default()});
    app.world_mut().send_event(LoadingGameComplete{restore_to:AppMode::default()});
    app.update();
    assert!(marker.exists(),"native14 postpones unlink");
    assert_eq!(*app.world().resource::<AppMode>(),AppMode::LoadGame);
    app.insert_resource(AppMode::Replay);
    app.update();
    assert!(marker.exists(),"native13 postpones unlink after event buffer advances");
    app.insert_resource(AppMode::default());
    app.update();
    assert!(!marker.exists());
    // The Local bit is not reset after success or when switching sessions.
    std::fs::write(&marker,b"new marker after first unlink").unwrap();
    app.update();
    assert!(!marker.exists());
    let second=SessionId::new();
    std::fs::create_dir_all(second.save_folder_path()).unwrap();
    let next=second.save_folder_path().join("loading.tmp");
    std::fs::write(&next,b"second session marker").unwrap();
    app.insert_resource(ActiveSession::new(second,SessionMeta::default()));
    app.update();
    assert!(!next.exists());
    // Original drops NotFound and all other unlink errors; an existing directory
    // is a deterministic non-NotFound error and must not become App failure.
    std::fs::create_dir(&next).unwrap();
    app.update();
    assert!(next.is_dir());
    assert_eq!(*app.world().resource::<AppMode>(),AppMode::default());
}
''')
(out.parent/'completion-candidate.patch').write_text(''.join(diff),encoding='utf-8',newline='')
(out.parent/'completion-candidate-manifest.json').write_text(json.dumps({'created_at':datetime.datetime.now().astimezone().isoformat(),'files':rows,'status':'evidence only, no production write, no Cargo or Rust compilation'},indent=2),encoding='utf-8')
print(json.dumps(rows,indent=2))
