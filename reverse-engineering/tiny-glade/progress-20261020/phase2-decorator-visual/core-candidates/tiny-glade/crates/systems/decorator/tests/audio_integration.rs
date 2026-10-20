//! DEC-E 集成测试：装饰音频三系统（confirm / preview / stairs）。
//!
//! 断言真实资源写回：系统运行后 `Events<PlayDecoratorSoundCmd>` /
//! `Events<PlayStairsAudioCmd>` 中的命令载荷（kind / position / values /
//! bytes），以及 `DecoratorAudioEvents` 的帧内去重。事件记录按反编译
//! 0x88B 字段偏移构造（见 `audio::DecoratorOpEventView`）。

use bevy_ecs::prelude::*;
use bevy_ecs::system::RunSystemOnce;
use glam::{Quat, Vec3};

use country_core::resources::roofs::roof_shape::Roof;
use country_core::resources::stairs::stairs_state::StairsState;
use country_core::resources::terrain_height::TerrainHeightsData;
use country_core::resources::walls::cached_decorator_transforms::{
    CachedDecoratorTransform, CachedDecoratorTransforms,
};
use country_core::resources::walls::decorator::{
    DecoratorExtraInfo, DecoratorId, DecoratorInfo, DecoratorOpEventKind, StairExtra,
};
use country_core::resources::walls::decorator_storage::{
    DecoratorStorage, DecoratorStorageAddr,
};
use country_core::resources::walls::public_walls::PublicWalls;
use country_core::resources::walls::WallId;
use country_core::systems::audio::{PlayDecoratorSoundCmd, PlayStairsAudioCmd};

use system_decorator::audio::op_confirm_audio::{op_confirm_audio, place_sound};
use system_decorator::audio::op_preview_audio::{op_preview_audio, preview_kind};
use system_decorator::audio::op_stairs_audio::{kind as stair_kind, op_stairs_audio};
use system_decorator::audio::DecoratorAudioEvents;

/// 构造 0x88B 操作事件；`disc` 写 +0x00，位置写 +0x70..+0x7c。
fn op_event(disc: u64, decorator_type: u8, state: u8, position: Vec3) -> DecoratorOpEventKind {
    let mut raw = [0u8; 0x88];
    raw[0..8].copy_from_slice(&disc.to_le_bytes());
    raw[0x70..0x74].copy_from_slice(&position.x.to_le_bytes());
    raw[0x74..0x78].copy_from_slice(&position.y.to_le_bytes());
    raw[0x78..0x7c].copy_from_slice(&position.z.to_le_bytes());
    raw[0x7c] = state;
    raw[0x81] = decorator_type;
    DecoratorOpEventKind(raw)
}

fn set_u32(event: &mut DecoratorOpEventKind, offset: usize, value: u32) {
    event.0[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

fn set_u64(event: &mut DecoratorOpEventKind, offset: usize, value: u64) {
    event.0[offset..offset + 8].copy_from_slice(&value.to_le_bytes());
}

fn drain_sounds(world: &mut World) -> Vec<PlayDecoratorSoundCmd> {
    world
        .resource_mut::<Events<PlayDecoratorSoundCmd>>()
        .drain()
        .collect()
}

fn drain_stairs(world: &mut World) -> Vec<PlayStairsAudioCmd> {
    world
        .resource_mut::<Events<PlayStairsAudioCmd>>()
        .drain()
        .collect()
}

fn audio_world() -> World {
    let mut world = World::new();
    world.init_resource::<DecoratorAudioEvents>();
    world.init_resource::<Events<DecoratorOpEventKind>>();
    world.init_resource::<Events<PlayDecoratorSoundCmd>>();
    world.init_resource::<Events<PlayStairsAudioCmd>>();
    world.init_resource::<PublicWalls>();
    world
}

fn send_op(world: &mut World, event: DecoratorOpEventKind) {
    world
        .resource_mut::<Events<DecoratorOpEventKind>>()
        .send(event);
}

fn insert_stair_decorator(
    storage: &mut DecoratorStorage,
    id: DecoratorId,
    extra: [u8; 20],
) {
    let addr = DecoratorStorageAddr::Stair;
    let mut info = DecoratorInfo {
        decorator_id: id.0,
        ..DecoratorInfo::default()
    };
    info.set_extra(DecoratorExtraInfo::Stair(StairExtra(extra)));
    storage.get_or_insert_default(addr).insert(id, info);
    storage.decorator_to_storage.insert(id, addr);
}

#[test]
fn place_sound_matches_the_original_type_table() {
    let window = place_sound(
        country_core::resources::walls::decorator::DecoratorType(0x3a),
        7,
        0,
    );
    assert_eq!((window.tag, window.kind), (1, 2));

    let lantern = place_sound(
        country_core::resources::walls::decorator::DecoratorType(0x3c),
        0,
        3,
    );
    assert_eq!((lantern.tag, lantern.kind, lantern.flag), (0xb, 0x10, 1));

    let window_family = place_sound(
        country_core::resources::walls::decorator::DecoratorType(0x3b),
        0x14,
        0,
    );
    assert_eq!(window_family.tag, 0x11);
    assert_eq!(window_family.subtype, 1, "0x100 >> ((0x14-0x13)<<3)");

    let chimney = place_sound(
        country_core::resources::walls::decorator::DecoratorType(0x3e),
        3,
        0,
    );
    assert_eq!((chimney.tag, chimney.kind, chimney.subtype), (0x23, 0x20, 0x3e));
}

#[test]
fn confirm_writes_one_shot_commands_with_event_position() {
    let mut world = audio_world();
    let pos = Vec3::new(3.0, 4.0, 5.0);
    let cases = [(0x3bu8, 0x17u8), (0x3c, 0x10), (0x3d, 0x1e), (0x3e, 0x20)];
    for (ty, _kind) in cases {
        let mut event = op_event(0, ty, 0, pos);
        set_u32(&mut event, 0x10, 1); // != 3：走 place_sound/记录级分派
        send_op(&mut world, event);
    }
    world.run_system_once(op_confirm_audio).unwrap();

    let sounds = drain_sounds(&mut world);
    let kinds: Vec<u8> = sounds.iter().map(|sound| sound.kind).collect();
    assert_eq!(kinds, vec![0x17, 0x10, 0x1e, 0x20]);
    assert!(sounds.iter().all(|sound| sound.position == pos));
}

#[test]
fn confirm_disc4_remove_emits_delete_kind() {
    let mut world = audio_world();
    let mut event = op_event(4, 0x38, 0, Vec3::new(1.0, 0.0, 2.0));
    event.0[0x44] = 1;
    event.0[0x45] = 2;
    set_u64(&mut event, 0x60, 9); // disc4 的 id 字段（DecoratorOpEventKind::id）
    send_op(&mut world, event);

    world.run_system_once(op_confirm_audio).unwrap();
    let sounds = drain_sounds(&mut world);
    assert_eq!(sounds.len(), 1);
    assert_eq!(sounds[0].kind, 1, "0x38 → place_sound kind 1");
}

#[test]
fn confirm_dedups_same_audio_key_in_one_frame() {
    let mut world = audio_world();
    let pos = Vec3::ZERO;
    send_op(&mut world, op_event(0, 0x3c, 0, pos));
    send_op(&mut world, op_event(0, 0x3c, 0, pos));
    // 不同 type → 不同 key，应保留。
    send_op(&mut world, op_event(0, 0x3e, 0, pos));

    world.run_system_once(op_confirm_audio).unwrap();
    let kinds: Vec<u8> = drain_sounds(&mut world)
        .iter()
        .map(|sound| sound.kind)
        .collect();
    assert_eq!(kinds, vec![0x10, 0x20]);
    let events = world.resource::<DecoratorAudioEvents>();
    assert_eq!(events.sounds().count(), 2);
}

#[test]
fn preview_kind_table_matches_original_jump_table() {
    use country_core::resources::walls::decorator::DecoratorType;
    assert_eq!(preview_kind(DecoratorType(0x38), 2), (7, 1));
    assert_eq!(preview_kind(DecoratorType(0x3b), 2), (0x16, 1));
    assert_eq!(preview_kind(DecoratorType(0x3c), 3), (0xc, 1));
    assert_eq!(preview_kind(DecoratorType(0x3d), 2), (0x1c, 1));
    assert_eq!(preview_kind(DecoratorType(0x3d), 3), (0x1d, 0));
    assert_eq!(preview_kind(DecoratorType(0x3e), 9), (0xc, 0));
    assert_eq!(preview_kind(DecoratorType(0x3e), 3), (0x24, 0));
    assert_eq!(preview_kind(DecoratorType(0x40), 0), (0x16, 0));
}

#[test]
fn preview_emits_window_preview_kind_and_dedups() {
    let mut world = audio_world();
    let pos = Vec3::new(0.5, 1.5, -2.0);
    send_op(&mut world, op_event(0, 0x38, 0, pos));
    send_op(&mut world, op_event(0, 0x38, 0, pos));
    world.run_system_once(op_preview_audio).unwrap();

    let sounds = drain_sounds(&mut world);
    assert_eq!(sounds.len(), 1);
    assert_eq!(sounds[0].kind, 7);
    assert_eq!(sounds[0].position, pos);
}

#[test]
fn preview_emits_wooden_flat_roof_loop_kind() {
    let mut world = audio_world();
    // 平屋顶（height_01 @ params[0xc..0x10] = 0）且 flat style tag=2/pattern=1
    // → is_flat + is_wooden。
    let mut flat_style = [0u8; 12];
    flat_style[0] = 2;
    flat_style[1] = 1;
    let roof = Roof::new(
        WallId(5).0.to_le_bytes(),
        [0u8; 28],
        [0u8; 28],
        [0u8; 4],
        [0u8; 4],
        0,
        flat_style,
    );
    let entity = world.spawn(roof).id();
    world
        .resource_mut::<PublicWalls>()
        .associate_roof(WallId(5), entity);

    let mut event = op_event(0, 0x42, 0, Vec3::new(1.0, 2.0, 3.0));
    set_u64(&mut event, 0x08, 5); // 目的墙 id
    event.0[0x42] = 2; // 目的是屋顶
    send_op(&mut world, event);

    world.run_system_once(op_preview_audio).unwrap();
    let sounds = drain_sounds(&mut world);
    assert_eq!(sounds.len(), 1);
    assert_eq!(sounds[0].kind, 6, "DecoratorMoveWoodenFloorTEMP");
}

#[test]
fn stairs_disc4_emits_delete_with_stair_payload() {
    let mut world = audio_world();
    let id = DecoratorId(7);
    let mut storage = DecoratorStorage::default();
    let mut extra = [0u8; 20];
    extra[0..4].copy_from_slice(&2.5f32.to_le_bytes());
    extra[4] = 0x11;
    extra[5] = 0x22;
    insert_stair_decorator(&mut storage, id, extra);

    let mut transforms = CachedDecoratorTransforms::default();
    transforms.insert(
        id.0,
        CachedDecoratorTransform {
            rotation: Quat::IDENTITY,
            translation: Vec3::new(9.0, 1.0, 9.0),
            scale: Vec3::ONE,
        },
    );
    world.insert_resource(storage);
    world.insert_resource(transforms);
    world.insert_resource(TerrainHeightsData::<country_core::resources::terrain_height::TerrainHeightSsbo>::new());
    world.insert_resource(StairsState::new());

    let pos = Vec3::new(4.0, 5.0, 6.0);
    let mut event = op_event(4, 0x3d, 0, pos);
    set_u64(&mut event, 0x60, id.0 as u64); // disc4 id @ +0x60
    event.0[0x44] = 1;
    send_op(&mut world, event);

    world.run_system_once(op_stairs_audio).unwrap();
    let cmds = drain_stairs(&mut world);
    assert_eq!(cmds.len(), 1);
    assert_eq!(cmds[0].kind, stair_kind::STAIR_DELETE);
    assert_eq!(cmds[0].position, pos);
    assert_eq!(cmds[0].values[0], 2.5);
    assert_eq!(cmds[0].bytes[0], 0);
    assert_eq!(&cmds[0].bytes[1..3], &[0x11, 0x22]);
}

#[test]
fn stairs_change_emits_place_or_ladder_from_graph_presence() {
    let id = DecoratorId(7);

    let run = |with_edge: bool| {
        let mut world = audio_world();
        let mut storage = DecoratorStorage::default();
        let mut extra = [0u8; 20];
        extra[0..4].copy_from_slice(&1.25f32.to_le_bytes());
        insert_stair_decorator(&mut storage, id, extra);
        let mut transforms = CachedDecoratorTransforms::default();
        transforms.insert(
            id.0,
            CachedDecoratorTransform {
                rotation: Quat::IDENTITY,
                translation: Vec3::new(1.0, 0.5, 1.0),
                scale: Vec3::ONE,
            },
        );
        let mut stairs = StairsState::new();
        if with_edge {
            stairs.add_edge(id.0, 8);
        }
        world.insert_resource(storage);
        world.insert_resource(transforms);
        world.insert_resource(TerrainHeightsData::<country_core::resources::terrain_height::TerrainHeightSsbo>::new());
        world.insert_resource(stairs);

        let mut event = op_event(3, 0x3d, 0, Vec3::new(1.0, 0.5, 1.0));
        set_u64(&mut event, 0x40, id.0 as u64); // disc3 id @ +0x40
        set_u32(&mut event, 0x10, 0);
        event.0[0x44] = 1;
        send_op(&mut world, event);
        world.run_system_once(op_stairs_audio).unwrap();
        drain_stairs(&mut world)
    };

    let without = run(false);
    assert_eq!(without.len(), 1);
    assert_eq!(without[0].kind, stair_kind::LADDER_PLACE);

    let with = run(true);
    assert_eq!(with.len(), 1);
    assert_eq!(with[0].kind, stair_kind::STAIR_PLACE);
}

#[test]
fn stairs_requires_transform_cache_and_skips_roof_events() {
    let mut world = audio_world();
    let id = DecoratorId(7);
    let mut storage = DecoratorStorage::default();
    insert_stair_decorator(&mut storage, id, [0u8; 20]);
    world.insert_resource(storage);
    // 未写变换缓存 → try_transform 未命中，跳过。
    world.insert_resource(CachedDecoratorTransforms::default());
    world.insert_resource(TerrainHeightsData::<country_core::resources::terrain_height::TerrainHeightSsbo>::new());
    world.insert_resource(StairsState::new());

    let mut event = op_event(4, 0x3d, 0, Vec3::ONE);
    set_u64(&mut event, 0x60, id.0 as u64);
    event.0[0x44] = 1;
    send_op(&mut world, event);
    // 状态字节 == 2 的屋顶事件跳过。
    let mut roof_event = op_event(4, 0x3d, 2, Vec3::ONE);
    set_u64(&mut roof_event, 0x60, id.0 as u64);
    send_op(&mut world, roof_event);

    world.run_system_once(op_stairs_audio).unwrap();
    assert!(drain_stairs(&mut world).is_empty());
}

#[test]
fn new_frame_clears_dedup_tables_and_state() {
    let mut events = DecoratorAudioEvents::default();
    let cmd = PlayDecoratorSoundCmd {
        kind: 7,
        position: Vec3::ONE,
    };
    let key = system_decorator::audio::AudioEventKey {
        decorator_type: country_core::resources::walls::decorator::DecoratorType(0x38),
        flag: 0,
    };
    assert!(events.play_sound(key, cmd));
    assert!(!events.play_sound(key, cmd), "same key must dedupe");
    events.set_state(7, 0);
    assert_eq!(events.sounds().count(), 1);
    events.new_frame();
    assert_eq!(events.sounds().count(), 0);
    assert_eq!(events.stair_sounds().count(), 0);
    assert_eq!(events.state(), [0; 4]);
}
