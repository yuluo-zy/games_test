//! `rasterize_buildings.rs` — 建筑遮罩栅格化 + 模糊 + 动画（W11，`rasterize_buildings` 0x140a32770，4791 B）。
//!
//! 机器码（O）：
//! 1. `TexturePreset(0x15)` → `TextureDesc`（`impl$1::from`）；`mask_extent = extent_2d`。
//! 2. world 资源（TypeId `0x9b126e795fb22adf / 0xcb7c6a0645e229de`）是 `HashMap<_, V>`（`V` 每项 0x2a8 B）。`HashMap::iter` →
//!    `filter_map`（`spec_from_iter` 0x1412fc510，见 [`GroundedSource::to_element`]）→ `Vec<GroundedBuildingsSsbo>`（32 B/项）。
//! 3. world 资源 `AssetSsboLibrary`（`0x8226cc786e679355 / 0x65cfda6a55a841ac`）：`FrozenMap::get(SsboId)` 取 `InstanceBuffer<GroundedBuildingsSsbo>`：
//!    * 未注册、或 `downcast` 失败 → **跳过上传**（不 panic），直接进入 pass 声明；
//!    * 类型名核对 `"country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo"`（0x5e 字节）不符 → `assert_failed`。
//! 4. `count >= 5000` → `panic!("Exceeded maximum number of buildings per level")`（在取 `GraphicsDevice` 资源之前）。
//! 5. world 资源 `GraphicsDevice`（`0x2ed189581b92decf / 0x15d3d99396065f38`，缺失 → `Option::unwrap`）。
//!    `count == 0` → 只把 `instance_count` 置 0；否则 `update_buffer_bytes(buffer, data, offset 0, count*32)`，`Err`（含超出容量 5000×32）→
//!    `Result::unwrap` panic；成功 → `instance_count = count`。
//! 6. **"Rasterize buildings"** `/rasterize_buildings.cs.hlsl`（名长 0x13、URI 长 0x1c）：`read_storage_buffer<GroundedBuildingsSsbo>(buildings_buffer)` →
//!    uniform i32 `buildings_count` → `out_mask`（预设 0x15，写 0x32）→ `out_mask2`（预设 0x16，写 0x32）→ `dispatch_2d(mask_extent)`。
//! 7. `create_raw_resource("buildings blur temp")`（名长 0x13）：`new_2d((512, 512), preset 0x15 的格式)`。
//!    **"Blur buildings mask"** `/gauss_blur_building_mask.cs.hlsl`（名长 0x13、URI 长 0x21）×2：
//!    ① `is_horizontal = true`、`building_mask`（预设 0x15，读）→ `out_building_mask`（temp，写 0x32）；
//!    ② `is_horizontal = false`、`building_mask`（temp，读）→ `out_building_mask`（预设 0x15，写 0x32）；均 `dispatch_2d(mask_extent)`。
//! 8. **"Animate buildings mask"** `/rasterize_buildings_animate.cs.hlsl`（名长 0x16、URI 长 0x24）：`out_mask`（预设 0x15，写 0x32）→
//!    `dispatch_2d(mask_extent)`。
//!
//! `GroundedBuildingsSsbo`：`SsboId` 立即数 0x1c53e91811919db1（与 NAME 的 xxh3 不吻合，NAME U）；缓冲 0x27100 B = 5000 × 32，
//! `InstanceBuffer` 存储。源记录 `V` 的字段名全部 U（只知偏移与用法）。

use crate::rhapsody_types::TextureDesc;
use std::collections::HashMap;

use crate::asset_libraries::ssbo_library::{AssetSsboLibrary, SsboType};
use crate::render::pass_builder::compute_pass_builder::RecordedComputePass;
use crate::render::pass_builder::pass_resource_access::WriteTextureUnchecked;
use crate::render::pass_builder::pass_resources::TypeId128;
use crate::render::ssbo::rhapsody_boundary::GraphicsDevice;
use crate::render::ssbo::{InstanceBuffer, StorageElement};
use crate::resources::texture_presets::TexturePreset;
use crate::systems::render_loop::compute_passes::pass_kit::{PassEnv, binding, binding_hash};

pub const MASK: (&str, &str) = ("Rasterize buildings", "/rasterize_buildings.cs.hlsl");
pub const BLUR: (&str, &str) = ("Blur buildings mask", "/gauss_blur_building_mask.cs.hlsl");
pub const ANIMATE: (&str, &str) = (
    "Animate buildings mask",
    "/rasterize_buildings_animate.cs.hlsl",
);
pub const BLUR_TEMP_NAME: &str = "buildings blur temp";
pub const BLUR_TEMP_EXTENT: [u32; 2] = [512, 512];
pub const BUILDINGS_BUFFER: u64 = binding_hash("buildings_buffer");
pub const BUILDINGS_COUNT: u64 = binding_hash("buildings_count");
pub const OUT_MASK: u64 = binding_hash("out_mask");
pub const OUT_MASK2: u64 = binding_hash("out_mask2");
pub const BUILDING_MASK: u64 = binding_hash("building_mask");
pub const OUT_BUILDING_MASK: u64 = binding_hash("out_building_mask");
/// 单关卡建筑数上限（O：缓冲 5000 项；`count >= 5000` 即 panic）。
pub const MAX_BUILDINGS: usize = 5000;
pub const TOO_MANY_PANIC: &str = "Exceeded maximum number of buildings per level";
/// 建筑表资源 TypeId（O）。
pub const BUILDINGS_TYPE_ID: TypeId128 = TypeId128 {
    lo: 0x9b12_6e79_5fb2_2adf,
    hi: 0xcb7c_6a06_45e2_29de,
};
/// `GraphicsDevice` 资源 TypeId（O）。
pub const DEVICE_TYPE_ID: TypeId128 = TypeId128 {
    lo: 0x2ed1_8958_1b92_decf,
    hi: 0x15d3_d993_9606_5f38,
};
/// 库里该缓冲的原版 SsboId 立即数（O）。
pub const GROUNDED_BUILDINGS_SSBO_ID: u64 = 0x1c53_e918_1191_9db1;
/// 原版类型名核对串（O：0x5e 字节）。
pub const GROUNDED_TYPE_NAME: &str = "country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo";
/// `filter_map` 的进度阈值（O：0.05f；大于它的记录被丢弃）。
pub const SKIP_ABOVE: f32 = 0.05;

/// GPU 端建筑元素（O：32 B；字段 U，以 u32 位模式保存）。
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GroundedBuildingsSsbo(pub [u32; 8]);

impl StorageElement for GroundedBuildingsSsbo {
    /// U：真实 `T::NAME` 与 SsboId 不吻合，这里用类型名。
    const NAME: &'static str = "GroundedBuildingsSsbo";
    const BUFFER_LEN: usize = MAX_BUILDINGS;
}

impl SsboType for GroundedBuildingsSsbo {
    type Storage = InstanceBuffer<GroundedBuildingsSsbo>;
    fn new_storage_buffer(device: &dyn GraphicsDevice, initial: Option<&[u8]>) -> Self::Storage {
        InstanceBuffer::<GroundedBuildingsSsbo>::new_storage_buffer(device, initial)
    }
}

/// 建筑表里一条记录中被读取的字段（O：偏移相对桶尾；每条记录 0x2a8 B，其余字段不读）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GroundedSource {
    /// `@-0x48`：形状判别式；> 1 的记录被丢弃。
    pub kind: u32,
    /// `@-0x44`、`@-0x40`、`@-0x3c`、`@-0x38`：四个 4 字节字（位模式）。
    pub w44: u32,
    pub w40: u32,
    pub w3c: u32,
    pub w38: u32,
    /// `@-0x34`：8 字节（两个字）。
    pub v34: [u32; 2],
    /// `@-0x20`：f32，`> 0.05` 的记录被丢弃（NaN 不丢，`ucomiss`+`ja`）。
    pub progress: f32,
}

impl GroundedSource {
    /// `filter_map` 的闭包（O 0x1412fc510）。
    ///
    /// * `progress > 0.05` 或 `kind > 1` → 丢弃。
    /// * `kind == 1`：`[w3c, w38, v34.0, v34.1, w44, w40, 1, 0]`。
    /// * `kind == 0`：`[w44, w40, w3c, w3c, 0, 0, 0, 0]`（`movsldup`：`w3c` 复制成两个相邻字）。
    pub fn to_element(&self) -> Option<GroundedBuildingsSsbo> {
        if self.progress > SKIP_ABOVE || self.kind > 1 {
            return None;
        }
        Some(GroundedBuildingsSsbo(if self.kind == 1 {
            [
                self.w3c,
                self.w38,
                self.v34[0],
                self.v34[1],
                self.w44,
                self.w40,
                1,
                0,
            ]
        } else {
            [self.w44, self.w40, self.w3c, self.w3c, 0, 0, 0, 0]
        }))
    }
}

/// 预设 0x15 的描述（`impl$1::from(TexturePreset(0x15))`）：栅格化遮罩的 extent 与格式。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MaskDesc {
    pub extent: [u32; 2],
    pub format: u32,
}

/// 收集建筑并上传（O：步骤 2–5）。返回 `count`。
pub fn upload_buildings<K>(
    buildings: &HashMap<K, GroundedSource>,
    ssbos: &mut AssetSsboLibrary,
    device: &dyn GraphicsDevice,
) -> usize {
    let elements: Vec<GroundedBuildingsSsbo> = buildings
        .values()
        .filter_map(GroundedSource::to_element)
        .collect();
    let count = elements.len();
    let Some(buffer) = ssbos.get_mut::<GroundedBuildingsSsbo>() else {
        return count; // 未注册：跳过上传（O）
    };
    assert!(count < MAX_BUILDINGS, "{TOO_MANY_PANIC}");
    if count == 0 {
        buffer.instance_count = 0;
    } else {
        buffer
            .buffer
            .gpu_write(device, &elements)
            .unwrap_or_else(|e| panic!("called `Result::unwrap()` on an `Err` value: {e:?}"));
        buffer.instance_count = count;
    }
    count
}

/// W11 的 pass 声明部分（步骤 6–8）。
pub fn declare_passes(
    env: &mut PassEnv<'_>,
    mask: MaskDesc,
    count: usize,
) -> Vec<RecordedComputePass> {
    let [w, h] = mask.extent;
    let mut passes = Vec::with_capacity(4);
    passes.push(
        env.pass(MASK.0, MASK.1)
            .read_storage_buffer::<GroundedBuildingsSsbo>(BUILDINGS_BUFFER)
            .uniform(BUILDINGS_COUNT, count as i32)
            .bind_texture(OUT_MASK, &TexturePreset(0x15), 0x32)
            .bind_texture(OUT_MASK2, &TexturePreset(0x16), 0x32)
            .dispatch_2d(w, h),
    );
    let temp = env.create_texture(
        BLUR_TEMP_NAME,
        TextureDesc::new_2d(BLUR_TEMP_EXTENT, mask.format),
    );
    passes.push(
        env.pass(BLUR.0, BLUR.1)
            .uniform(binding::IS_HORIZONTAL, true)
            .bind_texture(BUILDING_MASK, &TexturePreset(0x15), 0x19)
            .bind_texture(OUT_BUILDING_MASK, &WriteTextureUnchecked(temp), 0x32)
            .dispatch_2d(w, h),
    );
    passes.push(
        env.pass(BLUR.0, BLUR.1)
            .uniform(binding::IS_HORIZONTAL, false)
            .bind_texture(BUILDING_MASK, &&temp, 0x19)
            .bind_texture(OUT_BUILDING_MASK, &TexturePreset(0x15), 0x32)
            .dispatch_2d(w, h),
    );
    passes.push(
        env.pass(ANIMATE.0, ANIMATE.1)
            .bind_texture(OUT_MASK, &TexturePreset(0x15), 0x32)
            .dispatch_2d(w, h),
    );
    passes
}

/// W11：`upload_buildings` + `declare_passes`。
pub fn rasterize_buildings<K>(
    env: &mut PassEnv<'_>,
    buildings: &HashMap<K, GroundedSource>,
    ssbos: &mut AssetSsboLibrary,
    device: &dyn GraphicsDevice,
    mask: MaskDesc,
) -> Vec<RecordedComputePass> {
    let count = upload_buildings(buildings, ssbos, device);
    declare_passes(env, mask, count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::ssbo::tests::MockDevice;
    use crate::systems::render_loop::compute_passes::pass_kit::test_env::{
        accesses, env_with, threads,
    };

    fn src(kind: u32, progress: f32) -> GroundedSource {
        GroundedSource {
            kind,
            w44: 0x44,
            w40: 0x40,
            w3c: 0x3c,
            w38: 0x38,
            v34: [0x34, 0x35],
            progress,
        }
    }

    #[test]
    fn names_match_exe_immediates() {
        assert_eq!(BUILDINGS_BUFFER, 0x4a99_c896_9e0b_36cd);
        assert_eq!(BUILDINGS_COUNT, 0xce4c_fd9d_d902_baae);
        assert_eq!(OUT_MASK, 0xbdf6_ed93_e1e6_13b8);
        assert_eq!(OUT_MASK2, 0xb0a9_3c48_d9f3_337e);
        assert_eq!(BUILDING_MASK, 0x2de8_7fdb_6674_0ece);
        assert_eq!(OUT_BUILDING_MASK, 0x960d_27b7_b6f8_1d59);
        assert_eq!(GROUNDED_TYPE_NAME.len(), 0x5e);
        assert_eq!(TOO_MANY_PANIC.len(), 0x2e);
        assert_eq!(
            core::mem::size_of::<GroundedBuildingsSsbo>() * MAX_BUILDINGS,
            0x27100
        );
    }

    #[test]
    fn filter_map_matches_the_exe_closure() {
        // 丢弃：进度 > 0.05、kind > 1
        assert_eq!(src(0, 0.0501).to_element(), None);
        assert_eq!(src(2, 0.0).to_element(), None);
        assert_eq!(src(u32::MAX, 0.0).to_element(), None);
        // 边界：恰为 0.05 保留；NaN 不丢（ucomiss unordered 时 ja 不跳）
        assert!(src(0, 0.05).to_element().is_some());
        assert!(src(1, f32::NAN).to_element().is_some());
        // kind 0（圆）
        assert_eq!(
            src(0, 0.0).to_element().unwrap().0,
            [0x44, 0x40, 0x3c, 0x3c, 0, 0, 0, 0]
        );
        // kind 1（矩形）
        assert_eq!(
            src(1, 0.0).to_element().unwrap().0,
            [0x3c, 0x38, 0x34, 0x35, 0x44, 0x40, 1, 0]
        );
    }

    fn lib_with_buffer(dev: &MockDevice) -> AssetSsboLibrary {
        let l = AssetSsboLibrary::new();
        l.add_new::<GroundedBuildingsSsbo>(dev);
        l
    }

    #[test]
    fn upload_sets_instance_count_and_writes_32_bytes_per_building() {
        let dev = MockDevice::default();
        let mut lib = lib_with_buffer(&dev);
        let mut m = HashMap::new();
        m.insert(1u32, src(0, 0.0));
        m.insert(2, src(1, 0.0));
        m.insert(3, src(0, 0.9)); // 丢弃
        assert_eq!(upload_buildings(&m, &mut lib, &dev), 2);
        assert_eq!(
            lib.get_mut::<GroundedBuildingsSsbo>()
                .unwrap()
                .instance_count,
            2
        );
    }

    #[test]
    fn no_buildings_zeroes_the_instance_count_without_writing() {
        let dev = MockDevice::default();
        let mut lib = lib_with_buffer(&dev);
        lib.get_mut::<GroundedBuildingsSsbo>()
            .unwrap()
            .instance_count = 9;
        assert_eq!(upload_buildings::<u32>(&HashMap::new(), &mut lib, &dev), 0);
        assert_eq!(
            lib.get_mut::<GroundedBuildingsSsbo>()
                .unwrap()
                .instance_count,
            0
        );
    }

    #[test]
    fn unregistered_buffer_skips_the_upload_silently() {
        let dev = MockDevice::default();
        let mut lib = AssetSsboLibrary::new();
        let mut m = HashMap::new();
        m.insert(1u32, src(0, 0.0));
        assert_eq!(upload_buildings(&m, &mut lib, &dev), 1);
    }

    #[test]
    #[should_panic(expected = "Exceeded maximum number of buildings per level")]
    fn five_thousand_buildings_panic() {
        let dev = MockDevice::default();
        let mut lib = lib_with_buffer(&dev);
        let m: HashMap<u32, GroundedSource> = (0..5000).map(|i| (i, src(0, 0.0))).collect();
        upload_buildings(&m, &mut lib, &dev);
    }

    #[test]
    fn four_thousand_nine_hundred_ninety_nine_is_accepted() {
        let dev = MockDevice::default();
        let mut lib = lib_with_buffer(&dev);
        let m: HashMap<u32, GroundedSource> = (0..4999).map(|i| (i, src(1, 0.0))).collect();
        assert_eq!(upload_buildings(&m, &mut lib, &dev), 4999);
    }

    #[test]
    fn four_passes_with_exe_names_extents_and_blur_direction() {
        let names = [
            BUILDINGS_BUFFER,
            OUT_MASK,
            OUT_MASK2,
            BUILDING_MASK,
            OUT_BUILDING_MASK,
        ];
        let mut e = env_with(
            &names,
            &[BUILDINGS_COUNT, binding::IS_HORIZONTAL],
            |lib, dev| {
                lib.add_new::<GroundedBuildingsSsbo>(dev);
            },
        );
        let mask = MaskDesc {
            extent: [640, 480],
            format: 0x26,
        };
        let passes = e.run(|env| declare_passes(env, mask, 17));
        let n: Vec<&str> = passes.iter().map(|p| p.name).collect();
        assert_eq!(
            n,
            [
                "Rasterize buildings",
                "Blur buildings mask",
                "Blur buildings mask",
                "Animate buildings mask"
            ]
        );
        for p in &passes {
            assert_eq!(threads(p), (640, 480, 1));
        }
        assert_eq!(e.uniform_u32(&passes[0], BUILDINGS_COUNT), 17);
        assert_eq!(e.uniform_u32(&passes[1], binding::IS_HORIZONTAL), 1);
        assert_eq!(e.uniform_u32(&passes[2], binding::IS_HORIZONTAL), 0);
        let (name, spec, t) = &e.frame.textures[0];
        assert_eq!(
            (name.as_str(), spec.extent_2d(), spec.format),
            ("buildings blur temp", [512, 512], 0x26)
        );
        // ① 写 temp ② 读 temp
        assert_eq!(accesses(&passes[1])[1], (t.resource, 0x32, 0));
        assert_eq!(accesses(&passes[2])[0], (t.resource, 0x19, 1));
    }
}
