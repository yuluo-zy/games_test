//! Scripted scenario on a real bevy_app::App: insert -> move into water -> delete -> frames
//! until despawn. The SpawnPrefabCmd/DespawnPrefabCmd consumers and the animation fade are
//! HARNESS systems (outside the tree-tool module).
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use glam::{Vec2, Vec3};
use tg_tree::despawn::despawn_trees;
use tg_tree::process_cmds::process_tree_cmds;
use tg_tree::replay::ReplayBuffer;
use tg_tree::*;

#[derive(Resource, Default)]
struct Frame(u32);

fn script(frame: Res<Frame>, mut ins: EventWriter<InsertTreeCmd>, mut mv: EventWriter<MoveTreeCmd>,
          mut del: EventWriter<DeleteTreeCmd>) {
    match frame.0 {
        0 => { ins.write(InsertTreeCmd { name: String::new(), position: Vec3::new(1.0, 0.0, 1.0),
            height: 1.5, field_28_unknown: 0.0, flags: 0, byte_30_unknown: 0, index: 0 }); }
        2 => { mv.write(MoveTreeCmd { index: 0, position: Vec3::new(25.0, 0.0, 25.0) }); }
        4 => { del.write(DeleteTreeCmd { index: 0 }); }
        _ => {}
    }
}

fn consume_spawns(frame: Res<Frame>, mut r: EventReader<SpawnPrefabCmd>, mut commands: Commands) {
    for s in r.read() {
        println!("[{:02}] SpawnPrefabCmd {} at {:?} flags={}", frame.0, s.name, s.position, s.flags);
        commands.entity(Entity::from_bits(s.field_10_unknown)).insert(PrefabInstanceData::at(s.position));
    }
}

fn print_sounds(frame: Res<Frame>, mut r: EventReader<PlayTreeSoundCmd>) {
    for s in r.read() {
        println!("[{:02}] PlayTreeSoundCmd {:?} in_water={} pos={:?}", frame.0, s.kind, s.in_water, s.position);
    }
}

fn fade(mut q: Query<&mut TreeAnimation, With<TreeMarkedForDeletion>>) {
    for mut a in &mut q { a.value_at_4 *= 0.7; }
}

fn consume_despawns(frame: Res<Frame>, mut r: EventReader<DespawnPrefabCmd>, mut commands: Commands) {
    for d in r.read() {
        println!("[{:02}] DespawnPrefabCmd {:#x}", frame.0, d.entity_bits);
        if let Ok(mut e) = commands.get_entity(Entity::from_bits(d.entity_bits)) { e.despawn(); }
    }
}

fn next_frame(mut f: ResMut<Frame>) { f.0 += 1; }

fn main() {
    let mut app = App::new();
    app.add_plugins(TreeToolPlugin)
        .insert_resource(ActiveSession::default())
        .insert_resource(WaterRaster::rect(512, Vec2::new(20.0, 20.0), Vec2::new(30.0, 30.0)))
        .insert_resource(GameTime { delta: 1.0 / 60.0 })
        .insert_resource(TerrainHeights::flat(0.0))
        .insert_resource(ReplayBuffer { front: vec![], divisor: 1.0 })
        .init_resource::<Frame>()
        .add_systems(Update, (
            script.before(process_tree_cmds),
            (consume_spawns, print_sounds, fade).after(process_tree_cmds).before(despawn_trees),
            consume_despawns.after(despawn_trees),
            next_frame.after(consume_despawns),
        ));
    for _ in 0..40 {
        app.update();
        let f = app.world().resource::<Frame>().0;
        let mut q = app.world_mut().query::<&TreeComponent>();
        if f > 5 && q.iter(app.world()).count() == 0 { println!("[{f:02}] all trees gone"); break; }
    }
    let s = app.world().resource::<TreeStorage>();
    println!("storage: len={} next_index={}", s.len(), s.next_index());
}
