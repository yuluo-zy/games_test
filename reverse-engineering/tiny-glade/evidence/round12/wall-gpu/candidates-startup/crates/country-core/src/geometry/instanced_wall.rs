//! 墙砖的来源编码、实例载荷与待上传状态。
//!
//! 墙对象独占稀疏分配；更新按整批提交，容量不足保留旧几何和 CPU 数据。
//! 布局来源见 README 的历史证据，运行期不依赖原作对象偏移或 moved-from 哨兵。
use crate::asset_libraries::ssbo_library::SsboType;
use crate::render::sparse_instance_buffer::{
    NoFreeChunks, SparseInstanceBuffer, SparseInstanceBufferAllocation, SparseSsbo,
};
use crate::render::ssbo::rhapsody_boundary::GraphicsDevice;
use crate::render::ssbo::StorageElement;
use crate::resources::color_id::ColorId;
use crate::resources::walls::publication_fields::WallId;

/// 墙或楼梯砖的来源编码；不承担全局对象身份。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BrickSourceId(pub u32);

pub const TRANSIENT_STAIR_FLAG: u32 = 0x8000_0000;
const TRANSIENT_EDGE_FLAG: u32 = 0x4000_0000;
const FIRST_TRANSIENT_STAIR_ID: u32 = 100_000;

/// 上传准备错误；两种失败都保留旧分配和待上传列表。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WallUploadError {
    CapacityExceeded,
    ForeignStorage,
}

impl std::fmt::Display for WallUploadError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::CapacityExceeded => "砖实例缓冲容量不足，更新仍待提交",
            Self::ForeignStorage => "墙的分配不属于当前稀疏缓冲",
        })
    }
}

impl std::error::Error for WallUploadError {}

impl From<NoFreeChunks> for WallUploadError {
    fn from(_: NoFreeChunks) -> Self {
        Self::CapacityExceeded
    }
}

fn encode_source_id(source_id: u32) -> u32 {
    let stair_index = source_id.wrapping_sub(FIRST_TRANSIENT_STAIR_ID);
    match stair_index {
        0..=3 | 11 => TRANSIENT_STAIR_FLAG | stair_index,
        _ => source_id,
    }
}

impl BrickSourceId {
    pub fn from_color_id(color_id: ColorId) -> Self {
        Self(encode_source_id(color_id.0))
    }

    /// 砖来源格式只保留墙 ID 的低 32 位。
    pub fn from_wall_id(wall_id: WallId) -> Self {
        Self(encode_source_id(wall_id.0 as u32))
    }

    /// 将边编号放入第 4 位起的编码区域；调用方应使用受控的边编号。
    pub fn with_transient_stair_edge_id(self, edge_id: u32) -> u32 {
        (edge_id << 4) | self.0 | TRANSIENT_EDGE_FLAG
    }
}

/// 一堵墙的待提交砖列表与独占稀疏分配。
///
/// 不实现 Clone，避免复制实体时无意共享 GPU 分配；新实体应显式传入砖列表。
#[derive(bevy_ecs::component::Component, Debug)]
pub struct InstancedWall<T> {
    pub id: u64,
    brick_transforms: Vec<T>,
    upload_pending: bool,
    allocation: Option<SparseInstanceBufferAllocation<T>>,
}

impl<T> InstancedWall<T> {
    pub fn new(id: u64, brick_transforms: Vec<T>) -> Self {
        Self {
            id,
            brick_transforms,
            upload_pending: true,
            allocation: None,
        }
    }

    /// 合并同帧多次更新，只上传最后一份列表；空列表明确表示清除全部砖。
    pub fn update(&mut self, brick_transforms: Vec<T>) {
        self.brick_transforms = brick_transforms;
        self.upload_pending = true;
    }

    /// 只读访问待上传数据，防止绕过更新标记直接修改列表。
    pub fn pending_instances(&self) -> &[T] {
        &self.brick_transforms
    }

    pub fn is_upload_pending(&self) -> bool {
        self.upload_pending
    }

    pub fn allocation(&self) -> Option<&SparseInstanceBufferAllocation<T>> {
        self.allocation.as_ref()
    }

    /// 从未分配过 GPU 槽的空更新无需创建整个共享缓冲。
    pub(crate) fn finish_unallocated_empty_update(&mut self) {
        if self.allocation.is_none() && self.brick_transforms.is_empty() {
            self.upload_pending = false;
        }
    }
}

impl<T: SparseSsbo> InstancedWall<T> {
    /// 返回是否写入了新版本。容量失败不清空 CPU 数据、不破坏旧分配；无更新直接返回。
    /// GPU 提交由共享缓冲的 try_gpu_flush 统一完成。
    pub fn upload_to_storage_buffer(
        &mut self,
        storage: &mut SparseInstanceBuffer<T>,
    ) -> Result<bool, WallUploadError> {
        if !self.upload_pending {
            return Ok(false);
        }
        let allocation = self
            .allocation
            .get_or_insert_with(|| storage.new_allocation());
        if !storage.owns_allocation(allocation) {
            return Err(WallUploadError::ForeignStorage);
        }
        let mut writer = storage.writer(allocation);
        writer.replace_all(&self.brick_transforms)?;
        // 生产者每次交付拥有的 Vec，成功后及时释放，避免每堵墙额外保留一份容量。
        self.brick_transforms = Vec::new();
        self.upload_pending = false;
        Ok(true)
    }
}

/// CPU 生产者和 GPU 缓冲共用的砖实例格式。字段按 shader 合同固定，不能随意重排。
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct BrickTransformSsbo {
    /// 四列三分量仿射矩阵：三个基向量与平移。
    pub affine_columns: [[f32; 3]; 4],
    /// 变形参数，零值表示没有附加变形。
    pub deformation: [f32; 4],
    /// 表面参数；默认 -100 是禁用哨兵，不能用零初始化代替。
    pub surface: [f32; 3],
    /// 逐砖序号或随机种子，由生产者赋值。
    pub seed: u32,
    /// 砖来源编码，由 BrickSourceId 生成。
    pub material: u32,
    /// 墙方向的两个距离参数。
    pub wall_distance: [f32; 2],
    /// 着色器解释的实例标志。
    pub flags: u32,
}

impl BrickTransformSsbo {
    /// 存档/诊断使用固定小端字节序；GPU 上传直接复用 Pod 字节视图。
    pub fn observed_bytes(self) -> [u8; 96] {
        let mut bytes = [0; 96];
        for (index, value) in self
            .affine_columns
            .into_iter()
            .flatten()
            .chain(self.deformation)
            .chain(self.surface)
            .enumerate()
        {
            bytes[index * 4..index * 4 + 4].copy_from_slice(&value.to_le_bytes());
        }
        bytes[76..80].copy_from_slice(&self.seed.to_le_bytes());
        bytes[80..84].copy_from_slice(&self.material.to_le_bytes());
        bytes[84..88].copy_from_slice(&self.wall_distance[0].to_le_bytes());
        bytes[88..92].copy_from_slice(&self.wall_distance[1].to_le_bytes());
        bytes[92..96].copy_from_slice(&self.flags.to_le_bytes());
        bytes
    }
}

impl Default for BrickTransformSsbo {
    fn default() -> Self {
        Self {
            affine_columns: [[0.0; 3]; 4],
            deformation: [0.0; 4],
            surface: [-100.0; 3],
            seed: 0,
            material: 0,
            wall_distance: [0.0; 2],
            flags: 0,
        }
    }
}

impl StorageElement for BrickTransformSsbo {
    const NAME: &'static str = "BrickTransforms";
    const BUFFER_LEN: usize = 0x200000;
}

// SAFETY: Pod 派生验证无填充；容量等于 CHUNK_COUNT × SLOTS_PER_CHUNK。
unsafe impl SparseSsbo for BrickTransformSsbo {
    const CHUNK_COUNT: usize = 0x10000;
}

impl SsboType for BrickTransformSsbo {
    type Storage = SparseInstanceBuffer<Self>;

    fn new_storage_buffer(device: &dyn GraphicsDevice, initial: Option<&[u8]>) -> Self::Storage {
        SparseInstanceBuffer::<Self>::new_storage_buffer(device, initial)
    }
}

impl crate::render::pass_builder::pass_resource_access::SsboType for BrickTransformSsbo {
    fn rg_buffer_hash() -> u64 {
        xxhash_rust::xxh3::xxh3_64(<Self as StorageElement>::NAME.as_bytes())
    }
}

#[cfg(test)]
#[path = "tests/instanced_wall.rs"]
mod tests;

// B init_populate's other three wall buffers. Canonical storage elements,
// distinct from draw_shaders' read-only type-name tags.
macro_rules! brick_render_ssbo {
    ($name:ident, $len:literal, $zero_on_none:literal) => {
        #[repr(transparent)]
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, bytemuck::Pod, bytemuck::Zeroable)]
        pub struct $name(pub u32);
        impl StorageElement for $name {
            const NAME: &'static str = stringify!($name);
            const BUFFER_LEN: usize = $len;
        }
        impl SsboType for $name {
            type Storage = crate::render::ssbo::TypedStorageBuffer<Self>;
            fn new_storage_buffer(
                device: &dyn GraphicsDevice,
                initial: Option<&[u8]>,
            ) -> Self::Storage {
                if $zero_on_none && initial.is_none() {
                    // The two native visibility constructors clear all65536
                    // words before creation. DrawList does not upload None.
                    crate::render::ssbo::TypedStorageBuffer::new(device, None)
                } else {
                    crate::render::ssbo::TypedStorageBuffer::new_storage_buffer(device, initial)
                }
            }
        }
        impl crate::render::pass_builder::pass_resource_access::SsboType for $name {
            fn rg_buffer_hash() -> u64 {
                crate::asset_libraries::ssbo_library::SsboId::of::<Self>()
                    .0
                     .0
            }
        }
    };
}

// Constructor@141612a90: 0x3800000B, usage22/memory1/alignment4;
// element4, BUFFER_LEN0xe00000. None is forwarded without an upload.
brick_render_ssbo!(BrickDrawListSsbo, 0xe00000, false);
// Constructors@141611000 and141611df0: 0x40000B each, same descriptor
// but original None actually uploads all zero words.
brick_render_ssbo!(BrickPrevVisibilityBitmaskSsbo, 0x10000, true);
brick_render_ssbo!(BrickNextVisibilityBitmaskSsbo, 0x10000, true);

/// Exactly the first four entries of init_populate_ssbo_library@140909e80,
/// preserving actual SparseInstanceBuffer default shadow and three distinct
/// constructor policies. This explicit prefix is not the full72 system.
pub fn populate_brick_ssbo_library(
    library: &crate::asset_libraries::ssbo_library::AssetSsboLibrary,
    device: &dyn GraphicsDevice,
) {
    library.add_new::<BrickTransformSsbo>(device);
    library.add_new::<BrickDrawListSsbo>(device);
    library.add_new::<BrickPrevVisibilityBitmaskSsbo>(device);
    library.add_new::<BrickNextVisibilityBitmaskSsbo>(device);
}

/// Original three-byte global wall draw policy component, written by
/// glade_startup_walls and consumed by the LOD0/LOD1 indirect callbacks.
/// Field names remain observed byte offsets: semantic enum names are unproved.
#[repr(C)]
#[derive(bevy_ecs::component::Component, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WallBrickInstancedDrawProxy {
    pub b0: u8,
    pub b1: u8,
    pub b2: u8,
}
