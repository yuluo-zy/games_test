//! world_raster_defs.rs — 命名世界栅格与 `AllWorldRasterWrite`。
//!
//! 六个资源（PathRaster、Cobblestone1/2/3Raster、WaterRaster、GardenRaster）都是
//! `WorldRaster<512>` 的新类型，`new` 直通 `WorldRaster::new`（O：`PathRaster::new`
//! 0x14083eaf0 只是 `call 0x140a15530` 后返回，其余五个共用同一模板，PDB 源码行 665）。
//! `deref`/`deref_mut` 在原版里被 ICF 折成恒等函数（0x14067dd80），这里用 `Deref`。

use std::ops::{Deref, DerefMut};

use crate::asset_libraries::ssbo_library::{AssetSsboLibrary, SsboType};
use crate::render::ssbo::rhapsody_boundary::GraphicsDevice;
use crate::render::ssbo::{MappedStorageBuffer, StorageElement};
use bevy_ecs::resource::Resource;
use bevy_ecs::system::{ResMut, SystemParam};

use super::world_raster::{DirtyTilePublisher, RasterBufferRef, WorldRaster};

macro_rules! named_raster {
    ($($(#[$meta:meta])* $name:ident),+ $(,)?) => {$(
        $(#[$meta])*
        #[derive(Clone, Resource)]
        pub struct $name(pub WorldRaster<512>);

        impl $name {
            pub fn new(publisher: DirtyTilePublisher) -> Self {
                Self(WorldRaster::new(publisher))
            }
            pub fn declaration_snapshot(&self) -> Self { Self(self.0.declaration_snapshot()) }
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

/// Owned stream record; this is not a native ABI layout. B's corresponding
/// 40-byte borrowed record has a name slice followed by `Option<Vec<u8>>`.
/// Names are validated before data is written.
pub type NamedRasterData = (String, Option<Vec<u8>>);

fn validate_named_data(lengths: [usize; 6], data: &[NamedRasterData]) -> anyhow::Result<()> {
    for (i, (name, expected)) in ORDERED_NAMES.iter().zip(lengths).enumerate() {
        let (provided, bytes) = data
            .get(i)
            .ok_or_else(|| anyhow::anyhow!("End of stream while expecting raster {name}"))?;
        anyhow::ensure!(
            provided == name,
            "Expected raster {name}, but got {provided}"
        );
        if let Some(bytes) = bytes {
            anyhow::ensure!(
                bytes.len() == expected,
                "Length mismatch; expected {}, but got {} while copying {name}",
                expected,
                bytes.len()
            );
        }
    }
    Ok(())
}

/// ECS ownership access for the six named rasters. The borrowed algorithm view
/// below shares these resources rather than maintaining a second raster store.
#[derive(SystemParam)]
pub struct WorldRasterResources<'w> {
    pub path: ResMut<'w, PathRaster>,
    pub cobblestone1: ResMut<'w, Cobblestone1Raster>,
    pub cobblestone2: ResMut<'w, Cobblestone2Raster>,
    pub cobblestone3: ResMut<'w, Cobblestone3Raster>,
    pub water: ResMut<'w, WaterRaster>,
    pub garden: ResMut<'w, GardenRaster>,
}

impl WorldRasterResources<'_> {
    pub fn would_write_succeed(&self, data: &[NamedRasterData]) -> anyhow::Result<()> {
        validate_named_data(
            self.read_view().read().map(|(_, view)| view.data.len()),
            data,
        )
    }

    /// Validate via shared borrows first, so rejected streams do not mark any
    /// resource changed. Successful writes then mutate each slot in source order.
    pub fn write_named(&mut self, data: &[NamedRasterData]) -> anyhow::Result<()> {
        self.would_write_succeed(data)?;
        let mut entries = data.iter();
        macro_rules! write_slot {
            ($field:ident) => {
                self.$field
                    .write_composite(entries.next().unwrap().1.as_deref());
            };
        }
        write_slot!(path);
        write_slot!(cobblestone1);
        write_slot!(cobblestone2);
        write_slot!(cobblestone3);
        write_slot!(water);
        write_slot!(garden);
        Ok(())
    }

    /// Preserve native prefix change ticks: dereference each resource only when
    /// that slot is actually reached. A mutable six-way view is too eager here.
    pub fn write(&mut self, data: &[Option<Vec<u8>>]) {
        let mut entries = data.iter();
        macro_rules! write_slot {
            ($field:ident) => {
                self.$field.write_composite(
                    entries
                        .next()
                        .expect("called `Option::unwrap()` on a `None` value")
                        .as_deref(),
                );
            };
        }
        write_slot!(path);
        write_slot!(cobblestone1);
        write_slot!(cobblestone2);
        write_slot!(cobblestone3);
        write_slot!(water);
        write_slot!(garden);
    }

    pub fn upload_to_staging(&mut self, library: &mut AssetSsboLibrary) {
        macro_rules! upload {
            ($field:ident, $staging:ty) => {{
                let buffer = library
                    .get_mut::<$staging>()
                    .expect("called `Option::unwrap()` on a `None` value");
                buffer.with_cpu_write(|mapped|
                    self.$field.copy_to_staging(0, bytemuck::cast_slice_mut(mapped)))
                    .expect("world raster staging write failed");
            }};
        }
        upload!(path, PathRasterStagingSsbo);
        upload!(cobblestone1, Cobblestone1RasterStagingSsbo);
        upload!(cobblestone2, Cobblestone2RasterStagingSsbo);
        upload!(cobblestone3, Cobblestone3RasterStagingSsbo);
        upload!(water, WaterRasterStagingSsbo);
        upload!(garden, GardenRasterStagingSsbo);
    }

    pub fn write_view(&mut self) -> AllWorldRasterWrite<'_> {
        AllWorldRasterWrite {
            path: &mut self.path,
            cobblestone1: &mut self.cobblestone1,
            cobblestone2: &mut self.cobblestone2,
            cobblestone3: &mut self.cobblestone3,
            water: &mut self.water,
            garden: &mut self.garden,
        }
    }

    pub fn read_view(&self) -> AllWorldRasterRead<'_> {
        AllWorldRasterRead {
            path: &self.path,
            cobblestone1: &self.cobblestone1,
            cobblestone2: &self.cobblestone2,
            cobblestone3: &self.cobblestone3,
            water: &self.water,
            garden: &self.garden,
        }
    }
}

#[derive(Clone, Copy)]
pub struct AllWorldRasterRead<'a> {
    pub path: &'a PathRaster,
    pub cobblestone1: &'a Cobblestone1Raster,
    pub cobblestone2: &'a Cobblestone2Raster,
    pub cobblestone3: &'a Cobblestone3Raster,
    pub water: &'a WaterRaster,
    pub garden: &'a GardenRaster,
}

impl AllWorldRasterRead<'_> {
    /// B read-side consumers select composite/scratch using +0x298.
    pub fn read(&self) -> [(&'static str, RasterBufferRef<'_, 512>); 6] {
        let rasters = [
            &self.path.0,
            &self.cobblestone1.0,
            &self.cobblestone2.0,
            &self.cobblestone3.0,
            &self.water.0,
            &self.garden.0,
        ];
        std::array::from_fn(|i| (ORDERED_NAMES[i], rasters[i].as_ref()))
    }
}

macro_rules! staging_rasters {
    ($($name:ident),+ $(,)?) => {$(
        /// B mapped upload SSBO: 512*512 bytes, byte alignment, memory kind 2.
        #[repr(transparent)]
        #[derive(Clone, Copy, Debug, Default)]
        pub struct $name(pub u8);
        // SAFETY: repr(transparent) over u8 has no padding and all bit patterns are valid.
        unsafe impl bytemuck::Zeroable for $name {}
        unsafe impl bytemuck::Pod for $name {}
        impl StorageElement for $name {
            const NAME: &'static str = stringify!($name);
            const BUFFER_LEN: usize = 512 * 512;
        }
        impl SsboType for $name {
            type Storage = MappedStorageBuffer<'static, Self>;
            fn new_storage_buffer(device: &dyn GraphicsDevice, initial: Option<&[u8]>) -> Self::Storage {
                MappedStorageBuffer::new_storage_buffer(device, initial)
            }
        }
    )+};
}

staging_rasters!(
    PathRasterStagingSsbo,
    Cobblestone1RasterStagingSsbo,
    Cobblestone2RasterStagingSsbo,
    Cobblestone3RasterStagingSsbo,
    WaterRasterStagingSsbo,
    GardenRasterStagingSsbo
);

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
    /// 每槽后 `cmp rbx, k`；不足六项时在已写前缀后 panic，额外项忽略）：
    /// `None` 清空 composite，`Some` 长度必须等于 composite 再整块拷入，
    /// 随后位图置全脏、`staging_dirty = 1`（见 [`WorldRaster::write_composite`]）。
    ///
    /// 原版每槽还会把 `ResMut` 的 changed tick 写成系统的 change tick；那是 Bevy 变更检测，
    /// 这里由调用方的 `ResMut` 负责。此低层入口只使用记录的后 24 B；
    /// 名字由 `would_write_succeed` 单独校验。
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
        assert!(
            data.len() >= 6,
            "called `Option::unwrap()` on a `None` value"
        );
    }

    /// O: 0x14083f410. Required slots are checked in order: presence, name,
    /// then length for Some(data). None skips the length check. Extra slots ignored.
    /// A failed check leaves all six resources untouched.
    pub fn would_write_succeed(&self, data: &[NamedRasterData]) -> anyhow::Result<()> {
        let rasters = [
            &self.path.0,
            &self.cobblestone1.0,
            &self.cobblestone2.0,
            &self.cobblestone3.0,
            &self.water.0,
            &self.garden.0,
        ];
        validate_named_data(rasters.map(|raster| raster.current_buffer().len()), data)
    }

    /// Core integration entry: validate the named save stream before any mutation.
    pub fn write_named(&mut self, data: &[NamedRasterData]) -> anyhow::Result<()> {
        self.would_write_succeed(data)?;
        let rasters: [&mut WorldRaster<512>; 6] = [
            &mut self.path.0,
            &mut self.cobblestone1.0,
            &mut self.cobblestone2.0,
            &mut self.cobblestone3.0,
            &mut self.water.0,
            &mut self.garden.0,
        ];
        for (raster, (_, bytes)) in rasters.into_iter().zip(data) {
            raster.write_composite(bytes.as_deref());
        }
        Ok(())
    }

    /// O: 0x14083fa60. Publish each composite into its registered mapped SSBO.
    /// Lookups and writes remain interleaved, preserving prefix side effects.
    pub fn upload_to_staging(&mut self, library: &mut AssetSsboLibrary) {
        macro_rules! upload {
            ($field:ident, $staging:ty) => {{
                let buffer = library
                    .get_mut::<$staging>()
                    .expect("called `Option::unwrap()` on a `None` value");
                buffer.with_cpu_write(|mapped|
                    self.$field.copy_to_staging(0, bytemuck::cast_slice_mut(mapped)))
                    .expect("world raster staging write failed");
            }};
        }
        upload!(path, PathRasterStagingSsbo);
        upload!(cobblestone1, Cobblestone1RasterStagingSsbo);
        upload!(cobblestone2, Cobblestone2RasterStagingSsbo);
        upload!(cobblestone3, Cobblestone3RasterStagingSsbo);
        upload!(water, WaterRasterStagingSsbo);
        upload!(garden, GardenRasterStagingSsbo);
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
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            all.write(&[Some(vec![7; 0x40000]), None]);
        }));
        assert!(
            result.is_err(),
            "B panics after the supplied prefix when a slot is missing"
        );
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
