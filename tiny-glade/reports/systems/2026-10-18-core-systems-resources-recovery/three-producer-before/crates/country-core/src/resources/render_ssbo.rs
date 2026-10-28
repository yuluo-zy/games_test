//! `resources::render_ssbo` — frame-pass SSBO 载体（wp5 的 typed-gap 占位 + B 阶段 i_infra 恢复的真实库侧类型）。
//!
//! 原版里这些类型分属 `resources::render::lantern`、`resources::compute_mouse_depth_value`、`compute_passes::*`、
//! `indirect_params`；wp5 为了让 render_loop 的各 pass 能引用它们统一放在这里（`consume::core` / `consume::buffers` 再导出）。
//! 状态：
//! * `indirect_params` 四个间接缓冲（`AtomicCounterBuffer` / `DrawIndirectBuffer` / `DispatchIndirectBuffer` /
//!   `DrawIndexedIndirectBuffer`）= `pub use indirect_params::*`（旧 `(Gap)` 占位已删）。
//! * [`LanternSsbo`]、[`MouseDepthValueSsbo`]：按机器码补齐 `StorageElement` + 库侧 `SsboType`（下文逐项 O）。
//! * `DofAutofocusSsbo`、`DofFocusReadbackSsbo`、`ShadowMaskHistogramSsbo`、`ShadowMaskMeanSsbo` 已迁到它们原版 NAME 路径所在的
//!   `compute_passes::{dof_autofocus, apply_screen_shadows}`（真实元素布局）。
//! * `WallColorIdSsbo` typed-gap 已由 `resources::color_id::WallColorIdSsbo` 落库
//!   （coregap-2，2026-10-17；本处占位移除）。

use xxhash_rust::xxh3::xxh3_64;

use crate::asset_libraries::ssbo_library::SsboType as LibrarySsbo;
use crate::render::pass_builder::pass_resource_access::SsboType;
use crate::render::ssbo::rhapsody_boundary::GraphicsDevice;
use crate::render::ssbo::{InstanceBuffer, StorageElement};

pub use crate::resources::compute_mouse_depth_value::MouseDepthValueSsbo;
pub use crate::resources::render::spatial_resolve_offsets::SpatialResolveOffsetsSsbo;
pub use crate::systems::render_loop::indirect_params::*;

/// 灯笼（O：`resources::render::lantern::LanternSsbo`，着色器侧 `LanternPacked { float3 position; uint intensity__color_id; }`，
/// `intensity__color_id` = half16 强度 | color_id << 16；16 B）。
///
/// 机器码（O）：
/// * `TypedStorageBuffer::new<LanternSsbo>`（0x141615bb0）：`BufferDesc::new(0x80000, 0x22, 1).alignment(4)`，名 "LanternSsbo"（11 B）；
///   元素大小 16（`shl r14, 4`）→ 容量 0x8000 个；有初始数据且 `len > 0x8000`（`cmp r9, 0x8001`）→ panic
///   "Excessive data size supplied for buffer …LanternSsbo"（类型名 0x35 B）。
/// * 存储是 `InstanceBuffer<LanternSsbo>`（`type_id<InstanceBuffer<LanternSsbo>>` 0x14160ec30；`bin_lights` / `restir_di` 读它的实例数）。
/// * `SsboId` = `xxh3_64("LanternSsbo")` = 0x8b2da1ef1bff9619（`get_rg_buffer<LanternSsbo>` 0x1412d5470 立即数）。
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LanternSsbo {
    pub position: [f32; 3],
    pub intensity_color_id: u32,
}

impl StorageElement for LanternSsbo {
    const NAME: &'static str = "LanternSsbo";
    /// 0x80000 / 16。
    const BUFFER_LEN: usize = 0x8000;
}

impl LibrarySsbo for LanternSsbo {
    type Storage = InstanceBuffer<LanternSsbo>;
    fn new_storage_buffer(device: &dyn GraphicsDevice, initial: Option<&[u8]>) -> Self::Storage {
        InstanceBuffer::<LanternSsbo>::new_storage_buffer(device, initial)
    }
}

impl SsboType for LanternSsbo {
    fn rg_buffer_hash() -> u64 {
        xxh3_64(<Self as StorageElement>::NAME.as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::asset_libraries::ssbo_library::AssetSsboLibrary;
    use crate::render::test_device::TestDevice as Device;
    use core::mem::{align_of, size_of};

    #[test]
    fn element_layouts_match_the_machine_code() {
        assert_eq!(size_of::<LanternSsbo>(), 16);
        assert_eq!(LanternSsbo::BUFFER_LEN * size_of::<LanternSsbo>(), 0x80000);
        assert_eq!(size_of::<MouseDepthValueSsbo>(), 16);
        assert_eq!(align_of::<MouseDepthValueSsbo>(), 4);
        assert_eq!(
            MouseDepthValueSsbo::BUFFER_LEN * size_of::<MouseDepthValueSsbo>(),
            0x40
        );
        assert_eq!(LanternSsbo::NAME.len(), 11);
        assert_eq!(MouseDepthValueSsbo::NAME.len(), 0xf);
    }

    #[test]
    fn ssbo_ids_are_the_exe_immediates() {
        // `get_rg_buffer<LanternSsbo>` 的立即数（deferred_raster 里 `-0x74d25e10e40069e7`）。
        assert_eq!(LanternSsbo::rg_buffer_hash(), 0x8b2d_a1ef_1bff_9619);
        assert_eq!(
            LanternSsbo::rg_buffer_hash(),
            (-0x74d2_5e10_e400_69e7_i64) as u64
        );
    }

    #[test]
    fn lantern_is_an_instance_buffer_in_the_library() {
        let dev = Device(1);
        let lib = AssetSsboLibrary::new();
        lib.add_new::<LanternSsbo>(&dev);
        // 无初始数据：`default_value` 缺省（None）→ 空实例缓冲、容量仍是 0x80000 B。
        assert_eq!(lib.instance_count::<LanternSsbo>(), 0);
        assert_eq!(
            lib.get::<LanternSsbo>().unwrap().buffer.byte_size(),
            0x80000
        );
    }

    #[test]
    fn mouse_depth_value_is_a_mapped_readback_buffer() {
        let dev = Device(1);
        let lib = AssetSsboLibrary::new();
        lib.add_new::<MouseDepthValueSsbo>(&dev);
        let s = lib.get::<MouseDepthValueSsbo>().unwrap();
        assert_eq!(s.buffer.byte_size, 0x40);
        assert_eq!(s.mapped.len(), 4);
        assert_eq!(MouseDepthValueSsbo::MAPPED_MEMORY, 4);
    }
}
