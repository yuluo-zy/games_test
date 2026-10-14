//! world_raster_defs.rs — 命名世界栅格与 `AllWorldRasterWrite`。
//!
//! 六个资源（PathRaster、Cobblestone1/2/3Raster、WaterRaster、GardenRaster）都是
//! `WorldRaster<512>` 的新类型，`new` 直通 `WorldRaster::new`（O：`PathRaster::new`
//! 0x14083eaf0 只是 `call 0x140a15530` 后返回，其余五个共用同一模板，PDB 源码行 665）。
//! `deref`/`deref_mut` 在原版里被 ICF 折成恒等函数（0x14067dd80），这里用 `Deref`。

use std::ops::{Deref, DerefMut};

use bevy_ecs::resource::Resource;

use super::world_raster::{DirtyTilePublisher, WorldRaster};

macro_rules! named_raster {
    ($($(#[$meta:meta])* $name:ident),+ $(,)?) => {$(
        $(#[$meta])*
        #[derive(Resource)]
        pub struct $name(pub WorldRaster<512>);

        impl $name {
            pub fn new(publisher: DirtyTilePublisher) -> Self {
                Self(WorldRaster::new(publisher))
            }
        }

        impl Deref for $name {
            type Target = WorldRaster<512>;
            fn deref(&self) -> &WorldRaster<512> {
                &self.0
            }
        }

        impl DerefMut for $name {
            fn deref_mut(&mut self) -> &mut WorldRaster<512> {
                &mut self.0
            }
        }
    )+};
}

named_raster!(
    PathRaster,
    Cobblestone1Raster,
    Cobblestone2Raster,
    Cobblestone3Raster,
    WaterRaster,
    GardenRaster,
);

/// 六个槽位的顺序（O：`ordered_names` 0x14083eb10 分配 0x60 B，写入六个胖指针，
/// 长度 10/0x12/0x12/0x12/0xb/0xc）。
pub const ORDERED_NAMES: [&str; 6] = [
    "PathRaster",
    "Cobblestone1Raster",
    "Cobblestone2Raster",
    "Cobblestone3Raster",
    "WaterRaster",
    "GardenRaster",
];

/// `AllWorldRasterWrite`：对六个栅格资源的可变借用（SystemParam，六个 `ResMut`）。
pub struct AllWorldRasterWrite<'a> {
    pub path: &'a mut PathRaster,
    pub cobblestone1: &'a mut Cobblestone1Raster,
    pub cobblestone2: &'a mut Cobblestone2Raster,
    pub cobblestone3: &'a mut Cobblestone3Raster,
    pub water: &'a mut WaterRaster,
    pub garden: &'a mut GardenRaster,
}

impl AllWorldRasterWrite<'_> {
    /// O：0x14083eb10。
    pub fn ordered_names() -> [&'static str; 6] {
        ORDERED_NAMES
    }

    /// O：0x14083ebe0。按槽位顺序逐个处理前 `data.len()` 个条目（原版第三个参数就是个数，
    /// 每槽后 `cmp rbx, k; je 结束`，最多 6 个；多出的条目被忽略）：
    /// `None` 清空 composite，`Some` 长度必须等于 composite 再整块拷入，
    /// 随后位图置全脏、`staging_dirty = 1`（见 [`WorldRaster::write_composite`]）。
    ///
    /// 原版每槽还会把 `ResMut` 的 changed tick 写成系统的 change tick；那是 Bevy 变更检测，
    /// 这里由调用方的 `ResMut` 负责。条目在原版里是 40 B（前 16 B 的用途未恢复，U），
    /// 写入只用后 24 B 的 `Option<Vec<u8>>`。
    pub fn write(&mut self, data: &[Option<Vec<u8>>]) {
        let rasters: [&mut WorldRaster<512>; 6] = [
            &mut self.path.0,
            &mut self.cobblestone1.0,
            &mut self.cobblestone2.0,
            &mut self.cobblestone3.0,
            &mut self.water.0,
            &mut self.garden.0,
        ];
        for (raster, item) in rasters.into_iter().zip(data) {
            raster.write_composite(item.as_deref());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rasters() -> [WorldRaster<512>; 6] {
        std::array::from_fn(|_| WorldRaster::new(DirtyTilePublisher::unique()))
    }

    #[test]
    fn six_slots_in_the_exe_order() {
        assert_eq!(
            AllWorldRasterWrite::ordered_names(),
            [
                "PathRaster",
                "Cobblestone1Raster",
                "Cobblestone2Raster",
                "Cobblestone3Raster",
                "WaterRaster",
                "GardenRaster"
            ]
        );
        // 长度与 0x14083eb10 写入的 10/0x12/0x12/0x12/0xb/0xc 一致。
        let lengths: Vec<usize> = ORDERED_NAMES.iter().map(|n| n.len()).collect();
        assert_eq!(lengths, [10, 0x12, 0x12, 0x12, 0xb, 0xc]);
    }

    #[test]
    fn named_rasters_deref_to_the_world_raster() {
        let mut path = PathRaster::new(DirtyTilePublisher::unique());
        path.composite_mut()[1] = 5;
        assert_eq!(path.composite()[1], 5);
        assert_eq!(rasters().len(), 6);
    }

    #[test]
    fn write_fills_leading_slots_and_dirties_only_those() {
        let mut path = PathRaster::new(DirtyTilePublisher::unique());
        let mut c1 = Cobblestone1Raster::new(DirtyTilePublisher::unique());
        let mut c2 = Cobblestone2Raster::new(DirtyTilePublisher::unique());
        let mut c3 = Cobblestone3Raster::new(DirtyTilePublisher::unique());
        let mut water = WaterRaster::new(DirtyTilePublisher::unique());
        let mut garden = GardenRaster::new(DirtyTilePublisher::unique());
        water.composite_mut()[0] = 9;
        let mut all = AllWorldRasterWrite {
            path: &mut path,
            cobblestone1: &mut c1,
            cobblestone2: &mut c2,
            cobblestone3: &mut c3,
            water: &mut water,
            garden: &mut garden,
        };
        all.write(&[Some(vec![7; 0x40000]), None]);
        assert_eq!(path.composite()[0], 7);
        assert!(path.staging_dirty());
        assert!(c1.staging_dirty());
        // 第三个起没有条目：既不清空也不置脏。
        assert!(!c2.staging_dirty());
        assert_eq!(water.composite()[0], 9);
        assert!(!water.staging_dirty());
    }

    #[test]
    fn write_ignores_entries_past_the_sixth_slot() {
        let mut path = PathRaster::new(DirtyTilePublisher::unique());
        let mut c1 = Cobblestone1Raster::new(DirtyTilePublisher::unique());
        let mut c2 = Cobblestone2Raster::new(DirtyTilePublisher::unique());
        let mut c3 = Cobblestone3Raster::new(DirtyTilePublisher::unique());
        let mut water = WaterRaster::new(DirtyTilePublisher::unique());
        let mut garden = GardenRaster::new(DirtyTilePublisher::unique());
        let mut all = AllWorldRasterWrite {
            path: &mut path,
            cobblestone1: &mut c1,
            cobblestone2: &mut c2,
            cobblestone3: &mut c3,
            water: &mut water,
            garden: &mut garden,
        };
        all.write(&vec![None; 9]);
        assert!(garden.staging_dirty());
    }

    #[test]
    #[should_panic(expected = "Length mismatch; expected 262144, but got 3")]
    fn write_with_wrong_length_panics_with_the_exe_message() {
        let mut path = PathRaster::new(DirtyTilePublisher::unique());
        let mut c1 = Cobblestone1Raster::new(DirtyTilePublisher::unique());
        let mut c2 = Cobblestone2Raster::new(DirtyTilePublisher::unique());
        let mut c3 = Cobblestone3Raster::new(DirtyTilePublisher::unique());
        let mut water = WaterRaster::new(DirtyTilePublisher::unique());
        let mut garden = GardenRaster::new(DirtyTilePublisher::unique());
        AllWorldRasterWrite {
            path: &mut path,
            cobblestone1: &mut c1,
            cobblestone2: &mut c2,
            cobblestone3: &mut c3,
            water: &mut water,
            garden: &mut garden,
        }
        .write(&[Some(vec![1, 2, 3])]);
    }
}
