//! `resources::render_ssbo` — frame-pass SSBO 载体（wp5 的 typed-gap 占位 + B 阶段 i_infra 恢复的真实库侧类型）。
//!
//! 原版里这些类型分属 `resources::render::lantern`、`resources::compute_mouse_depth_value`、`compute_passes::*`、
//! `indirect_params`；wp5 为了让 render_loop 的各 pass 能引用它们统一放在这里（`consume::core` / `consume::buffers` 再导出）。
//! 状态：
//! * `indirect_params` 四个间接缓冲（`AtomicCounterBuffer` / `DrawIndirectBuffer` / `DispatchIndirectBuffer` /
//!   `DrawIndexedIndirectBuffer`）= `pub use indirect_params::*`（旧 `(Gap)` 占位已删）。
//! * [`LanternSsbo`]、[`MouseDepthValueSsbo`]：按机器码补齐 `StorageElement` + 库侧 `SsboType`（下文逐项 O）。
//! * 其余（`DofAutofocusSsbo` 等）仍是 typed gap（原版由 compute_passes 里的真实类型承担，见各 pass 文件）。

use glam::Vec4;
use xxhash_rust::xxh3::xxh3_64;

use crate::asset_libraries::ssbo_library::SsboType as LibrarySsbo;
use crate::render::pass_builder::pass_resource_access::SsboType;
use crate::render::ssbo::rhapsody_boundary::GraphicsDevice;
use crate::render::ssbo::{
    InstanceBuffer, MAPPED_MEMORY_READBACK, MappedStorageBuffer, StorageElement,
};
use crate::typed_gap::Gap;

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

/// 鼠标处的世界坐标 + 深度（O：`resources::compute_mouse_depth_value::MouseDepthValueSsbo`；`read_mouse_depth_value.cs` 的
/// `out_buffer.Store<float4>(0, float4(mouse_world.xyz / w, mouse_depth))`，16 B，16 字节对齐）。
///
/// 机器码（O 0x14161e8f0，`impl$4::new_storage_buffer`，映射缓冲）：`BufferDesc::new(0x40, 0x22, 4).alignment(4)`，名
/// "MouseDepthValue"（0xf B）；元素大小 16（上限 `cmp rbx, 0x50` = 0x40 + 0x10）→ 4 个元素；无初始数据写全零 0x40 B（`xorps` + 4 个 `movups`）；
/// 数据指针按 16 对齐断言（`and rax, 0xf`）、按 4 对齐断言（`and rax, 3`）；memory = 4（回读，I）。
/// 与 A3 `MappedStorageBuffer` 的出入（报告）：`BufferDesc::alignment` 的实参恒为 4，而 A3 用 `align_of::<T>()`（此处 16）。
#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MouseDepthValueSsbo(pub Vec4);

impl StorageElement for MouseDepthValueSsbo {
    const NAME: &'static str = "MouseDepthValue";
    /// 0x40 / 16。
    const BUFFER_LEN: usize = 4;
    const MAPPED_MEMORY: u8 = MAPPED_MEMORY_READBACK;
}

impl LibrarySsbo for MouseDepthValueSsbo {
    type Storage = MappedStorageBuffer<'static, MouseDepthValueSsbo>;
    fn new_storage_buffer(device: &dyn GraphicsDevice, initial: Option<&[u8]>) -> Self::Storage {
        MappedStorageBuffer::<MouseDepthValueSsbo>::new_storage_buffer(device, initial)
    }
}

impl SsboType for MouseDepthValueSsbo {
    fn rg_buffer_hash() -> u64 {
        xxh3_64(<Self as StorageElement>::NAME.as_bytes())
    }
}

// ---- 仍是 typed gap 的载体
pub struct DofAutofocusSsbo(pub Gap);
pub struct DofFocusReadbackSsbo(pub Gap);
pub struct ShadowMaskHistogramSsbo(pub Gap);
pub struct ShadowMaskMeanSsbo(pub Gap);
pub struct WallColorIdSsbo(pub Gap);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::asset_libraries::ssbo_library::AssetSsboLibrary;
    use crate::rhapsody_contract::GraphicsDevice as Device;
    use core::mem::{align_of, size_of};

    #[test]
    fn element_layouts_match_the_machine_code() {
        assert_eq!(size_of::<LanternSsbo>(), 16);
        assert_eq!(LanternSsbo::BUFFER_LEN * size_of::<LanternSsbo>(), 0x80000);
        assert_eq!(size_of::<MouseDepthValueSsbo>(), 16);
        assert_eq!(align_of::<MouseDepthValueSsbo>(), 16);
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
