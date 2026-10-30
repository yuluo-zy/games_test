//! `decorator_on_wall_height_changed.rs` — 墙高变更 → 装饰锚高响应
//! （0x14081b5e0 + 闭包 0x14081b920）。
//!
//! 证据：
//! - `toolchain/decor-trunk/decompiled/14081b5e0_decorator_on_wall_height_changed.c`：
//!   系统由 **4 个事件 reader** 组成（0x30B / 0x10B 步长，逐个游标推进）；
//!   每个事件调用同一闭包。
//! - `toolchain/decor-trunk/decompiled/14081b920_closure_0.c`（O 全文）：
//!   事件墙 id → `wall_decorators_mut(wall)` → 遍历全部装饰，实际写入才标modified →
//!   side 公式（`(b-2)<=2 ? b-1 : 0`）：
//!   * side 0：`dst_as_wall_mut` + `PublicWalls::get(wall)`（缺失 unwrap panic）
//!     + `get_wall_attachment_height(Info+0的u)`；
//!   * 以Info+0的u查询，写 `Info+0x10 = Info+4的绝对y - height`；
//!   * side 1：无操作；
//!   * 其余：panic（原不可达）。
//!
//! B 布局（building-decorator-info/findings.md）：
//! `+0x00/+0x04 WallSpaceCoord`、`+0x10 wall_height_relative_y`、
//! `+0x14..0x20 anchor_ws Vec3`、`+0x20 corner`、`+0x21 is_bottom_door`、
//! `+0x22 side/dst tag`。本文件提供这些原始偏移的读写助手（供同组文件复用）。
//!
use bevy_ecs::prelude::*;
use glam::Vec3;

use country_core::resources::walls::WallId;
use country_core::resources::walls::decorator::DecoratorInfo;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::public_walls::PublicWalls;
use country_core::{
    resources::events::{FinishAdjWallHeightMode, ReanchorDecorators},
    systems::wall::adjust_wall_height::{
        AdjustWallHeightCmd, ui_adjust_wall_height_mode::AdjustWallHeightUi,
    },
};
use utils::wall_space::WallSpaceCoord;

/// 侧字节公式（O：`(b-2)<=2 ? b-1 : 0`；tag∈{2,3,4} → 1/2/3，其余 0）。
#[inline]
pub(crate) fn side_from_tag_byte(b: u8) -> u8 {
    if 2 < b.wrapping_sub(2) {
        0
    } else {
        b.wrapping_sub(1)
    }
}

/// `Info+0x00/+0x04`：WallSpaceCoord（O）。
#[inline]
pub fn get_wall_coord(info: &DecoratorInfo) -> WallSpaceCoord {
    WallSpaceCoord::new_unchecked(
        f32::from_bits(info.dst_wall as u32),
        f32::from_bits((info.dst_wall >> 32) as u32),
    )
}

#[inline]
pub fn set_wall_coord(info: &mut DecoratorInfo, coord: WallSpaceCoord) {
    info.dst_wall = u64::from(coord.u.to_bits()) | (u64::from(coord.y.to_bits()) << 32);
}

/// `Info+0x10`：wall_height_relative_y（O）。
#[inline]
pub fn get_wall_height_relative_y(info: &DecoratorInfo) -> f32 {
    f32::from_bits(info.dst_payload as u32)
}

#[inline]
pub fn set_wall_height_relative_y(info: &mut DecoratorInfo, value: f32) {
    info.dst_payload = (info.dst_payload & 0xffff_ffff_0000_0000) | u64::from(value.to_bits());
}

/// `Info+0x14..0x20`：anchor_ws Vec3（O：3 个 f32，跨 dst_payload 高 32 位与 u18）。
#[inline]
pub fn get_anchor_ws(info: &DecoratorInfo) -> Vec3 {
    Vec3::new(
        f32::from_bits((info.dst_payload >> 32) as u32),
        f32::from_bits(info.u18 as u32),
        f32::from_bits((info.u18 >> 32) as u32),
    )
}

#[inline]
pub fn set_anchor_ws(info: &mut DecoratorInfo, value: Vec3) {
    info.dst_payload =
        (info.dst_payload & 0x0000_0000_ffff_ffff) | (u64::from(value.x.to_bits()) << 32);
    info.u18 = u64::from(value.y.to_bits()) | (u64::from(value.z.to_bits()) << 32);
}

/// `Info+0x20`：WallCornerAttachment 原始字节；`Info+0x21`：is_bottom_door（O）。
#[inline]
pub fn get_attachment_flags(info: &DecoratorInfo) -> (u8, bool) {
    ((info.u20 & 0xff) as u8, ((info.u20 >> 8) & 1) != 0)
}

/// 原闭包14081b920：遍历全部目的地，墙装饰重算相对锚高并标modified。
pub fn adjust_wall_height_decorators(
    storage: &mut DecoratorStorage,
    walls: &PublicWalls,
    wall: WallId,
) {
    let Some(bucket) = storage.wall_decorators_mut(wall) else {
        // O：`wall_decorators_mut` 返回 None（桶不存在）→ 无操作。
        return;
    };
    // 原impl$3::next遍历内嵌完整map；仅get_mut时才向modified集合插入。
    // 拆借两个字段保留原逐项副作用顺序，不分配整桶ID临时数组。
    let country_core::resources::walls::decorator_storage::Decorators {
        decorators,
        modified_ids,
        ..
    } = bucket;
    for (&id, info) in decorators {
        match side_from_tag_byte(info.dst_tag()) {
            0 => {
                modified_ids.insert(id);
                let coordinate = get_wall_coord(info);
                let wall_state = walls.get(wall).expect("高度变更装饰目标公开墙不存在");
                // 14081ba55读取+0为u，+4为绝对y；attachment方法实际只使用u。
                let height = wall_state
                    .get_wall_attachment_height(coordinate.u)
                    .unwrap_or_else(|error| panic!("get_wall_attachment_height failed: {error:?}"));
                set_wall_height_relative_y(info, coordinate.y - height);
            }
            1 => {}
            _ => unreachable!("高度变更装饰不能附着楼梯或无效目的地"),
        }
    }
}

/// 原14081b5e0六参：UI/Finish各取最后一条；任一个命中时命令reader留待后帧。
/// Reanchor始终逐项执行，不将四流合并后去重或改成单一替代通知。
pub fn decorator_on_wall_height_changed(
    walls: Res<PublicWalls>,
    mut commands: EventReader<AdjustWallHeightCmd>,
    mut adjusting: EventReader<AdjustWallHeightUi>,
    mut finished: EventReader<FinishAdjWallHeightMode>,
    mut reanchor: EventReader<ReanchorDecorators>,
    mut storage: ResMut<DecoratorStorage>,
) {
    let last_adjust = adjusting.read().last().map(|event| event.0);
    if let Some(wall) = last_adjust {
        adjust_wall_height_decorators(&mut storage, &walls, wall);
    }
    let last_finish = finished.read().last().map(|event| event.0);
    if let Some(wall) = last_finish {
        adjust_wall_height_decorators(&mut storage, &walls, wall);
    }
    if last_adjust.is_none() && last_finish.is_none() {
        for event in commands.read() {
            adjust_wall_height_decorators(&mut storage, &walls, WallId(event.0.wall.0));
        }
    }
    for event in reanchor.read() {
        adjust_wall_height_decorators(&mut storage, &walls, event.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn side_formula_matches_closure() {
        assert_eq!(side_from_tag_byte(0), 0);
        assert_eq!(side_from_tag_byte(1), 0);
        assert_eq!(side_from_tag_byte(2), 1);
        assert_eq!(side_from_tag_byte(3), 2);
        assert_eq!(side_from_tag_byte(4), 3);
        assert_eq!(side_from_tag_byte(5), 0);
        assert_eq!(side_from_tag_byte(0xff), 0);
    }

    #[test]
    fn raw_offset_helpers_round_trip() {
        let mut info = DecoratorInfo::default();
        set_wall_coord(&mut info, WallSpaceCoord::new_unchecked(1.5, -2.5));
        assert_eq!(
            get_wall_coord(&info),
            WallSpaceCoord::new_unchecked(1.5, -2.5)
        );
        set_wall_height_relative_y(&mut info, 0.75);
        assert_eq!(get_wall_height_relative_y(&info), 0.75);
        set_anchor_ws(&mut info, Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(get_anchor_ws(&info), Vec3::new(1.0, 2.0, 3.0));
        // 坐标与相对高互不覆盖。
        assert_eq!(
            get_wall_coord(&info),
            WallSpaceCoord::new_unchecked(1.5, -2.5)
        );
    }

    #[test]
    fn attachment_flags_read_plus_0x20_and_0x21() {
        let info = DecoratorInfo {
            u20: 0x0000_0103,
            ..DecoratorInfo::default()
        };
        assert_eq!(get_attachment_flags(&info), (3, true));
    }

    fn world_with_walls() -> (World, Vec<WallId>) {
        use country_core::resources::{
            roofs::geometry::surface::Curve2,
            walls::{
                InnerWalls, WallState, decorator::DecoratorId,
                decorator_storage::DecoratorStorageAddr, publication_source::TerrainHeightGrid512,
            },
        };
        let mut world = World::new();
        let mut inner = InnerWalls::new();
        let walls = (0..9)
            .map(|_| {
                inner.new_wall(WallState::new(
                    Curve2::try_new(vec![[0., 0.], [4., 0.]], false).unwrap(),
                    4.,
                    0,
                    0,
                    false,
                ))
            })
            .collect::<Vec<_>>();
        let mut public = PublicWalls::default();
        let paths = country_core::systems::wall::wall_state::path_preparation::PathRasters512::from_encoded(
            std::array::from_fn(|_| vec![0; 512*512].into()), false).unwrap();
        public.consume_with_paths(
            &mut inner,
            &TerrainHeightGrid512 {
                encoded: vec![0; 512 * 512].into(),
                minimum_y: 0.,
                maximum_y: 0.,
                sampling_fix: false,
            },
            Some(&paths),
            &Default::default(),
            &Default::default(),
        );
        let mut storage = DecoratorStorage::default();
        for (index, &wall) in walls.iter().enumerate() {
            let record = public.get_mut(wall).unwrap();
            // 非均匀已计算高度，能检出把coord.y错当u投影的旧实现。
            for (i, height) in record
                .freeform_heights
                .as_mut()
                .unwrap()
                .arch_segment_heights
                .iter_mut()
                .enumerate()
            {
                *height = 1.0 + i as f32;
            }
            let mut info = DecoratorInfo::default();
            set_wall_coord(&mut info, WallSpaceCoord::new_unchecked(0.03, 7.0));
            set_wall_height_relative_y(&mut info, -123.0);
            storage
                .get_or_insert_default(DecoratorStorageAddr::Wall(wall))
                .insert(DecoratorId(index as u32), info);
        }
        for (_, bucket) in storage.iter_walls_mut() {
            bucket.modified_ids.clear();
        }
        world.insert_resource(storage);
        world.insert_resource(public);
        world.init_resource::<Events<AdjustWallHeightCmd>>();
        world.init_resource::<Events<AdjustWallHeightUi>>();
        world.init_resource::<Events<FinishAdjWallHeightMode>>();
        world.init_resource::<Events<ReanchorDecorators>>();
        (world, walls)
    }

    fn seed<E: Event>(world: &mut World, split: bool, events: impl IntoIterator<Item = E>) {
        let mut channel = world.resource_mut::<Events<E>>();
        for (i, event) in events.into_iter().enumerate() {
            if split && i == 1 {
                channel.update();
            }
            channel.send(event);
        }
    }

    /// 原14081b5e0在隔离Unicorn执行32例/64帧；对照真实World被标modified的墙集合。
    #[test]
    fn four_original_readers_match_native_latest_suppression_and_delayed_commands() {
        use country_core::systems::wall::height_command::AdjustWallHeightRequest;
        let fixture = include_str!(
            "../../../../reports/module-recovery-2026-10-23/m01-height-decorator-reader-native.txt"
        );
        for line in fixture.lines() {
            let parts = line.split('|').collect::<Vec<_>>();
            let config = parts[0]
                .split_whitespace()
                .map(|v| v.parse::<u8>().unwrap())
                .collect::<Vec<_>>();
            let (mask, split) = (config[0], config[1] != 0);
            let (mut world, walls) = world_with_walls();
            if mask & 1 != 0 {
                seed(
                    &mut world,
                    split,
                    [1, 2].map(|i| {
                        AdjustWallHeightCmd(AdjustWallHeightRequest {
                            anchor: None,
                            mode: 1,
                            value: 4.0,
                            wall: country_core::resources::walls::publication_fields::WallId(
                                walls[i].0,
                            ),
                            observed_audio_flag: 0,
                        })
                    }),
                );
            }
            if mask & 2 != 0 {
                seed(
                    &mut world,
                    split,
                    [3, 4].map(|i| AdjustWallHeightUi(walls[i])),
                );
            }
            if mask & 4 != 0 {
                seed(
                    &mut world,
                    split,
                    [5, 6].map(|i| FinishAdjWallHeightMode(walls[i])),
                );
            }
            if mask & 8 != 0 {
                seed(
                    &mut world,
                    split,
                    [7, 8].map(|i| ReanchorDecorators(walls[i])),
                );
            }
            let mut schedule = Schedule::default();
            schedule.add_systems(decorator_on_wall_height_changed);
            for frame in 0..2 {
                for (_, bucket) in world.resource_mut::<DecoratorStorage>().iter_walls_mut() {
                    bucket.modified_ids.clear();
                }
                schedule.run(&mut world);
                let mut expected = parts[frame + 1]
                    .split(',')
                    .filter(|v| !v.is_empty())
                    .map(|v| v.parse::<u32>().unwrap())
                    .collect::<Vec<_>>();
                expected.sort_unstable();
                expected.dedup();
                let storage = world.resource::<DecoratorStorage>();
                let mut actual = storage
                    .iter_walls()
                    .flat_map(|(_, b)| b.modified_ids.iter().copied())
                    .collect::<Vec<_>>();
                actual.sort_unstable();
                assert_eq!(actual, expected, "{line} frame{frame}");
                for id in actual {
                    let info = &storage
                        .wall_decorators(walls[id as usize])
                        .unwrap()
                        .decorators[&id];
                    let coord = get_wall_coord(info);
                    let height = world
                        .resource::<PublicWalls>()
                        .get(walls[id as usize])
                        .unwrap()
                        .get_wall_attachment_height(coord.u)
                        .unwrap();
                    assert_eq!(
                        get_wall_height_relative_y(info).to_bits(),
                        (coord.y - height).to_bits()
                    );
                    assert_eq!(coord, WallSpaceCoord::new_unchecked(0.03, 7.0));
                }
            }
        }
    }

    #[test]
    fn destination_gate_marks_only_wall_rows_and_missing_buckets_do_not_lookup_public() {
        use country_core::resources::walls::decorator::DecoratorId;
        let (mut world, walls) = world_with_walls();
        {
            let mut storage = world.resource_mut::<DecoratorStorage>();
            let bucket = storage.wall_decorators_mut(walls[0]).unwrap();
            let mut terrain = DecoratorInfo::default();
            terrain.u20 = 2 << 16;
            bucket.insert(DecoratorId(99), terrain);
            bucket.modified_ids.clear();
        }
        world
            .resource_mut::<Events<ReanchorDecorators>>()
            .send(ReanchorDecorators(walls[0]));
        let mut schedule = Schedule::default();
        schedule.add_systems(decorator_on_wall_height_changed);
        schedule.run(&mut world);
        let storage = world.resource::<DecoratorStorage>();
        let bucket = storage.wall_decorators(walls[0]).unwrap();
        assert!(bucket.modified_ids.contains(&0));
        assert!(!bucket.modified_ids.contains(&99));
        let mut empty = DecoratorStorage::default();
        adjust_wall_height_decorators(&mut empty, &PublicWalls::default(), WallId(999));
    }
}
