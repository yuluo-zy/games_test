from pathlib import Path
import hashlib,json,difflib,datetime
root=Path('D:/game/tiny-glade');here=Path(__file__).parent;out=here/'tod-startup-candidate';rows={};patch=[]
def save(rel,new):
 p=root/rel;old=p.read_bytes() if p.exists() else b'';dst=out/rel;dst.parent.mkdir(parents=True,exist_ok=True);dst.write_bytes(new)
 rows[rel]={'base_sha256':hashlib.sha256(old).hexdigest() if old else None,'candidate_sha256':hashlib.sha256(new).hexdigest()}
 lines=difflib.unified_diff(old.decode().splitlines(True),new.decode().splitlines(True),'a/'+rel if old else '/dev/null','b/'+rel)
 patch.append(''.join(line if line.endswith('\n') else line+'\n\\ No newline at end of file\n' for line in lines))
rel='crates/country-core/src/startup/startup_time_of_day.rs';old=(root/rel).read_bytes();nl=b'\r\n' if b'\r\n' in old else b'\n'
new='''//! Original B startup_time_of_day14089d6f0/448B. The complete native
//! wrapper reads the required JSON string and deserializes TodSequence, then
//! replaces that existing owner. File and serde failures both unwrap-panic.
//! Registered in original Startup batch4, not per-session OnEnter(Game).
use crate::resources::time_of_day::TodSequence;
use bevy_ecs::prelude::ResMut;
use std::path::Path;

pub const DEFAULT_TOD_PATH: &str = "assets/tod/default.json";

pub fn load_default_tod_from_str(text: &str) -> anyhow::Result<TodSequence> {
    Ok(serde_json::from_str(text)?)
}

pub fn load_default_tod(path: impl AsRef<Path>) -> anyhow::Result<TodSequence> {
    let text = std::fs::read_to_string(path)?;
    load_default_tod_from_str(&text)
}

pub fn startup_time_of_day(mut sequence: ResMut<TodSequence>) {
    // Host relocation only: B reads the same literal relative to installation;
    // a Cargo-built host keeps installed assets separate from its working dir.
    let loaded = load_default_tod(paths::installed().join(DEFAULT_TOD_PATH)).unwrap();
    *sequence = loaded;
}
'''.encode().replace(b'\n',nl)
save(rel,new)
rel='crates/tiny-glade/src/run_game.rs';raw=(root/rel).read_bytes();needle=b'        country_core::startup::startup_time_of_day::startup_time_of_day';pos=raw.index(needle)
start=raw.rfind(b'    app.add_systems(',0,pos);end=raw.index(b'    );',pos)+len(b'    );');block=raw[start:end]
assert b'OnEnter' in block and b'.before(country_core::startup::load_glade_tod_and_weather::load_glade_tod_and_weather)' in block
nl=b'\r\n' if b'\r\n' in block else b'\n'
new='''    // Original batch4 typed add_systems helper140b67b50 is Startup.
    app.add_systems(
        bevy_app::Startup,
        country_core::startup::startup_time_of_day::startup_time_of_day,
    );'''.encode().replace(b'\n',nl)
save(rel,raw[:start]+new+raw[end:])
fixture=Path('D:/game/ljxsj_92385/Tiny Glade/assets/tod/default.json').read_bytes()
assert len(json.loads(fixture)['entries'])==15
save('crates/tiny-glade/tests/fixtures/original-default-tod-b.json',fixture)
test='''//! Real original15-keyframe JSON through the actual Startup consumer.
use bevy_app::{App, Startup};
use country_core::{resources::time_of_day::{TimeOfDay,TodSequence},
    startup::startup_time_of_day::{startup_time_of_day,load_default_tod_from_str}};

#[test]
fn original_default_sequence_replaces_owner_once_and_preserves_saved_time() {
    let installation=tempfile::tempdir().unwrap();
    let target=installation.path().join("assets/tod/default.json");
    std::fs::create_dir_all(target.parent().unwrap()).unwrap();
    let original=include_str!("fixtures/original-default-tod-b.json");
    std::fs::write(&target,original).unwrap();
    paths::initialize_paths_with_installation(0,Some(installation.path()),installation.path()).unwrap();
    let expected=load_default_tod_from_str(original).unwrap();
    assert_eq!(expected.entries.len(),15);
    assert_eq!(expected.entries[0].time_of_day,0.0);
    assert_eq!(expected.entries[14].time_of_day,0.95);
    let mut previous=expected.clone();
    previous.entries.truncate(3);
    previous.entries[0].snapshot.speed_bias=-88.0;
    let saved=TimeOfDay{time_of_day:0.77,sun_xz_rotation:0.42,..Default::default()};
    let mut app=App::new();
    app.insert_resource(previous.clone());
    app.insert_resource(saved.clone());
    app.add_systems(Startup,startup_time_of_day);
    app.update();
    assert_eq!(app.world().resource::<TodSequence>(),&expected);
    assert_eq!(app.world().resource::<TimeOfDay>(),&saved);
    // Startup must not reload on subsequent updates, even after the file changes.
    std::fs::write(&target,"{").unwrap();
    app.update();
    assert_eq!(app.world().resource::<TodSequence>(),&expected);
    assert!(load_default_tod_from_str("{").is_err());
    assert!(load_default_tod_from_str(r#"{"time_of_day":0.3}"#).is_err());
    let mut invalid=App::new();
    invalid.insert_resource(previous.clone());
    invalid.add_systems(Startup,startup_time_of_day);
    assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||invalid.update())).is_err());
    assert_eq!(invalid.world().resource::<TodSequence>(),&previous);
    std::fs::remove_file(target).unwrap();
    let mut missing=App::new();
    missing.insert_resource(previous.clone());
    missing.add_systems(Startup,startup_time_of_day);
    assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||missing.update())).is_err());
    assert_eq!(missing.world().resource::<TodSequence>(),&previous);
}
'''
save('crates/tiny-glade/tests/original_tod_startup.rs',test.encode())
(here/'tod-startup-candidate.patch').write_text(''.join(patch),encoding='utf-8',newline='')
(here/'tod-startup-candidate-manifest.json').write_text(json.dumps({'created_at':datetime.datetime.now().astimezone().isoformat(),'files':rows,'original_asset_sha256':hashlib.sha256(fixture).hexdigest(),'original_entry_count':15,'status':'Evidence only. Complete448B wrapper original reads + typed serde + owner replacement; original Startup identity. No Cargo execution.'},indent=2),encoding='utf-8')
print(json.dumps(rows,indent=2))
