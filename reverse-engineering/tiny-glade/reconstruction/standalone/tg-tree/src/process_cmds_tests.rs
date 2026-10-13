use super::*;
use crate::test_util::*;
use bevy_app::prelude::*;
use glam::{Vec2, Vec3};
use std::sync::{Mutex, Once};

static LOGS: Mutex<Vec<String>> = Mutex::new(Vec::new());
struct Capture;
impl log::Log for Capture {
    fn enabled(&self, _: &log::Metadata) -> bool { true }
    fn log(&self, r: &log::Record) { LOGS.lock().unwrap().push(r.args().to_string()); }
    fn flush(&self) {}
}
fn init_logger() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        log::set_logger(&Capture).unwrap();
        log::set_max_level(log::LevelFilter::Trace);
    });
}
fn logged(msg: &str) -> usize {
    LOGS.lock().unwrap().iter().filter(|l| l.contains(msg)).count()
}

fn app() -> App {
    init_logger();
    let mut app = App::new();
    app.add_event::<MoveTreeCmd>()
        .add_event::<DeleteTreeCmd>()
        .add_event::<InsertTreeCmd>()
        .add_event::<SpawnPrefabCmd>()
        .add_event::<DespawnPrefabCmd>()
        .add_event::<PlayTreeSoundCmd>()
        .init_resource::<TreeStorage>()
        .insert_resource(ActiveSession::default())
        .insert_resource(WaterRaster::rect(512, Vec2::new(10.0, 10.0), Vec2::new(20.0, 20.0)))
        .init_resource::<Seen<SpawnPrefabCmd>>()
        .init_resource::<Seen<DespawnPrefabCmd>>()
        .init_resource::<Seen<PlayTreeSoundCmd>>()
        .add_systems(
            Update,
            (
                process_tree_cmds,
                (consume_spawns, collect::<SpawnPrefabCmd>, collect::<DespawnPrefabCmd>,
                 collect::<PlayTreeSoundCmd>),
            )
                .chain(),
        );
    // Burn entity index 0 so no tree entity has a zero index (high32 of the packed value).
    app.world_mut().spawn_empty();
    app
}

fn insert(app: &mut App, idx: u64, p: Vec3) {
    app.world_mut().send_event(InsertTreeCmd { name: "oak".into(), position: p, height: 2.0,
        field_28_unknown: 0.0, flags: 7, byte_30_unknown: 0, index: idx });
}
fn storage(app: &App) -> &TreeStorage { app.world().resource::<TreeStorage>() }
fn sounds(app: &App) -> Vec<PlayTreeSoundCmd> { app.world().resource::<Seen<PlayTreeSoundCmd>>().0.clone() }

#[test]
fn insert_spawns_registers_and_sounds() {
    let mut app = app();
    insert(&mut app, 3, Vec3::new(1.0, 0.0, 1.0));
    app.update();
    assert_eq!(storage(&app).next_index(), 4);
    let e = storage(&app).get(3).unwrap().entity;
    let w = app.world();
    assert!(w.get::<TreeComponent>(e).is_some() && w.get::<TreeAnimation>(e).is_some());
    assert!(w.get::<TreeMoveBend>(e).is_some() && w.get::<TreeAmbience>(e).is_some());
    let s = &w.resource::<Seen<SpawnPrefabCmd>>().0;
    assert_eq!((s.len(), s[0].name.as_str(), s[0].flags, s[0].field_30_one), (1, "oak", 0, 1.0));
    assert_eq!(s[0].field_10_unknown, e.to_bits());
    let snd = sounds(&app);
    assert_eq!((snd[0].kind, snd[0].in_water, snd[0].height), (TreeSoundKind::Insert, false, 2.0));
    assert!(w.resource::<Seen<DespawnPrefabCmd>>().0.is_empty());
}

#[test]
fn dragging_keeps_low_two_flag_bits() {
    let mut app = app();
    app.world_mut().resource_mut::<ActiveSession>().dragging = true;
    insert(&mut app, 0, Vec3::ZERO);
    app.update();
    assert_eq!(app.world().resource::<Seen<SpawnPrefabCmd>>().0[0].flags, 3);
}

#[test]
fn insert_in_water_and_session_flag() {
    let mut app = app();
    app.world_mut().resource_mut::<ActiveSession>().corrupted = true;
    insert(&mut app, 0, Vec3::new(15.0, 0.0, 15.0));
    app.update();
    let s = sounds(&app)[0];
    assert!(s.in_water && s.flag);
}

#[test]
fn empty_name_uses_random_then_fallback() {
    assert_eq!(resolve_prefab_name("", || Some("r".into())), "r");
    assert_eq!(resolve_prefab_name("", || None), MISSING_PREFAB_NAME);
    assert_eq!(resolve_prefab_name("x", || None), "x");
    assert_eq!(random_tree_prefab(), None); // candidate list UNKNOWN -> empty
    assert_eq!(pick_random_prefab(&["only"]), Some("only".to_owned()));
    assert!(["a", "b"].contains(&pick_random_prefab(&["a", "b"]).unwrap().as_str()));
}

#[test]
fn insert_over_existing_slot_sends_despawn_with_packed_value() {
    let mut app = app();
    insert(&mut app, 1, Vec3::ZERO);
    app.update();
    let first = storage(&app).get(1).unwrap();
    let before = logged("insert replaced existing tree slot 1");
    insert(&mut app, 1, Vec3::ZERO);
    app.update();
    let d = &app.world().resource::<Seen<DespawnPrefabCmd>>().0;
    assert_eq!(d.len(), 1);
    assert_eq!(d[0].entity_bits, first.raw());
    assert_eq!(logged("insert replaced existing tree slot 1"), before + 1);
}

#[test]
fn move_writes_position_and_sounds_in_water() {
    let mut app = app();
    insert(&mut app, 1, Vec3::new(1.0, 0.0, 1.0));
    app.update();
    app.world_mut().send_event(MoveTreeCmd { index: 1, position: Vec3::new(15.0, 0.5, 15.0) });
    app.update();
    let e = storage(&app).get(1).unwrap().entity;
    assert_eq!(app.world().get::<PrefabInstanceData>(e).unwrap().position, Vec3::new(15.0, 0.5, 15.0));
    let s = *sounds(&app).last().unwrap();
    assert_eq!((s.kind, s.in_water), (TreeSoundKind::Move, true));
}

#[test]
fn move_unknown_tree_logs_and_skips() {
    let mut app = app();
    let before = logged(MOVE_MISSING_LOG);
    app.world_mut().send_event(MoveTreeCmd { index: 9, position: Vec3::ZERO });
    app.update();
    assert_eq!(logged(MOVE_MISSING_LOG), before + 1);
    assert!(sounds(&app).is_empty());
}

#[test]
fn delete_marks_and_sounds_without_despawn_cmd() {
    let mut app = app();
    insert(&mut app, 1, Vec3::new(2.0, 0.0, 2.0));
    app.update();
    let e = storage(&app).get(1).unwrap().entity;
    app.world_mut().send_event(DeleteTreeCmd { index: 1 });
    app.update();
    assert!(app.world().get::<TreeMarkedForDeletion>(e).is_some());
    assert!(storage(&app).get(1).is_none());
    assert_eq!(sounds(&app).last().unwrap().kind, TreeSoundKind::Delete);
    assert!(app.world().resource::<Seen<DespawnPrefabCmd>>().0.is_empty());
    assert_eq!(storage(&app).next_index(), 2);
}

#[test]
fn delete_missing_or_dead_entity_is_noop() {
    let mut app = app();
    app.world_mut().send_event(DeleteTreeCmd { index: 5 });
    app.update();
    insert(&mut app, 6, Vec3::ZERO);
    app.update();
    let e = storage(&app).get(6).unwrap().entity;
    app.world_mut().despawn(e);
    app.world_mut().send_event(DeleteTreeCmd { index: 6 });
    app.update();
    let mut q = app.world_mut().query::<&TreeMarkedForDeletion>();
    assert_eq!(q.iter(app.world()).count(), 0);
    assert_eq!(sounds(&app).len(), 1);
}

#[test]
fn order_is_move_insert_delete_within_one_run() {
    let mut app = app();
    insert(&mut app, 1, Vec3::ZERO);
    app.update();
    // same frame: delete 1, insert 2, move 1 -> sound order Move, Insert, Delete
    app.world_mut().send_event(DeleteTreeCmd { index: 1 });
    insert(&mut app, 2, Vec3::ZERO);
    app.world_mut().send_event(MoveTreeCmd { index: 1, position: Vec3::ONE });
    app.update();
    let kinds: Vec<_> = sounds(&app).iter().skip(1).map(|s| s.kind).collect();
    assert_eq!(kinds, vec![TreeSoundKind::Move, TreeSoundKind::Insert, TreeSoundKind::Delete]);
}

#[test]
fn events_are_consumed_once() {
    let mut app = app();
    insert(&mut app, 1, Vec3::ZERO);
    app.update();
    app.update();
    assert_eq!(app.world().resource::<Seen<SpawnPrefabCmd>>().0.len(), 1);
}
