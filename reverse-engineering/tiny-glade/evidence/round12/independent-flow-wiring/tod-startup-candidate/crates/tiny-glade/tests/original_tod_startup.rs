//! Real original15-keyframe JSON through the actual Startup consumer.
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
