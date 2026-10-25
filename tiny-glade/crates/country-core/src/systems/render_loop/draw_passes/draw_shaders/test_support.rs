//! test_support.rs — draw_shaders 单元测试用的记录型 `PassHost`（仅 `cfg(test)`）。
//!
//! `MockHost`（A4，`draw_stream::test_host`）能提供反射 / 预设纹理 / SSBO 资源，但不记录**调用顺序**；本文件在其外面
//! 包一层 [`RecHost`]：每次 `rg_texture`（预设）、`texture_by_name`（磁盘纹理）、`rg_buffer_resource`（SSBO）调用都
//! 追加一条 [`Call`]，测试据此逐条对照机器码里的绑定顺序。反射里登记的名字哈希各自对应唯一的 binding 序号，
//! 常量块里每个 uniform 占独立偏移，便于读回写入的值。

use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::cell::RefCell;

use hashbrown::HashMap;

use crate::render::pass_builder::bind_resources::{RtBvhSsbo, RtTrianglesSsbo};
use crate::render::pass_builder::draw_builder::DrawBuilder;
use crate::render::pass_builder::draw_stream::test_host::{Log, MockHost};
use crate::render::pass_builder::draw_stream::{
    DrawPacket, DynamicConstantsBinding, PassHost, RasterPass, RasterPassSubmission,
    RasterRenderPassDesc, RasterState, ShaderReflection,
};
use crate::render::pass_builder::framebuffer_binding::AttachmentTexture;
use crate::render::pass_builder::pass_resource_access::SsboType;
use crate::render::shader::RasterShaderProgram;
use crate::rhapsody_contract::{Handle, ManagedHandle};

/// 宿主观察到的一次外部调用。
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Call {
    /// `AssetTextureLibrary::get_rg_texture(TexturePreset(n))`。
    Preset(u8),
    /// `AssetTextureLibrary::get_by_name(TextureFromDisk(key))`。
    Disk(u64),
    /// `AssetSsboLibrary::get_rg_buffer<T>`：参数是 `SsboId`。
    RgBuffer(u64),
}

/// 某个 SSBO 类型对应的 [`Call::RgBuffer`]。
pub fn rg<T: SsboType>() -> Call {
    Call::RgBuffer(T::rg_buffer_hash())
}

/// `Rig` 预置的 `bind_ray_tracing` 缓冲字（Triangles / Bvh）。
pub const RT_TRIANGLES_WORD: u32 = 0x7001;
pub const RT_BVH_WORD: u32 = 0x7002;

/// uniform 在常量块里的间距（够放 `Mat4`）。
pub const UNIFORM_STRIDE: u32 = 0x50;

pub struct RecHost {
    inner: MockHost,
    calls: Rc<RefCell<Vec<Call>>>,
    rg_ids: Rc<RefCell<HashMap<u64, u32>>>,
}

impl PassHost for RecHost {
    fn raster_shader(
        &self,
        s: &Handle<RasterShaderProgram>,
        v: u8,
    ) -> Option<Arc<ShaderReflection>> {
        self.inner.raster_shader(s, v)
    }
    fn allocate_dynamic_constants(
        &mut self,
        r: &ShaderReflection,
    ) -> Option<DynamicConstantsBinding> {
        self.inner.allocate_dynamic_constants(r)
    }
    fn ssbo_instance_count(&self, h: u64) -> Option<u32> {
        // `bind_ray_tracing<DrawBuilder>` 读取 Tri / Bvh 实例数量（不记 `Call`：原版里是库查询，不是 `get_rg_buffer`）。
        self.inner.ssbo_instance_count(h)
    }
    fn rg_buffer_resource(&self, h: u64) -> Option<u32> {
        self.calls.borrow_mut().push(Call::RgBuffer(h));
        let mut ids = self.rg_ids.borrow_mut();
        let next = 10 + ids.len() as u32;
        Some(*ids.entry(h).or_insert(next))
    }
    fn placement_buffer(&self, id: u64) -> Option<(ManagedHandle, u32)> {
        self.inner.placement_buffer(id)
    }
    fn vk_buffer(&self, b: &ManagedHandle) -> Option<u64> {
        // 任何受管缓冲都有对应的 vk 缓冲（真实宿主里 `GraphicsDevice::get_vk_buffer`）。
        Some(0xB000 + b.0)
    }
    fn vk_image_view(&self, t: &ManagedHandle) -> Result<u64, alloc::string::String> {
        self.inner.vk_image_view(t)
    }
    fn texture_by_name(&self, h: u64) -> (ManagedHandle, AttachmentTexture) {
        self.calls.borrow_mut().push(Call::Disk(h));
        (
            ManagedHandle(h),
            AttachmentTexture {
                id: 0x8000 + (h & 0xff) as u32,
                extent: (4, 4),
                format: 1,
                sample_count: 1,
            },
        )
    }
    fn rg_texture(&self, p: u8) -> AttachmentTexture {
        self.calls.borrow_mut().push(Call::Preset(p));
        AttachmentTexture {
            id: 0x100 + p as u32,
            extent: (8, 8),
            format: 1,
            sample_count: 1,
        }
    }
    fn with_suffix_dims(&self, h: u64) -> u64 {
        self.inner.with_suffix_dims(h)
    }
    fn default_raster_state(&self) -> RasterState {
        self.inner.default_raster_state()
    }
    fn create_render_pass(&mut self, d: &RasterRenderPassDesc) -> ManagedHandle {
        self.inner.create_render_pass(d)
    }
    fn render_pass_cache(&mut self) -> &mut HashMap<RasterRenderPassDesc, ManagedHandle> {
        self.inner.render_pass_cache()
    }
    fn submit_raster_pass(&mut self, s: RasterPassSubmission) {
        self.inner.submit_raster_pass(s)
    }
}

pub struct Rig {
    pub pass: RasterPass,
    calls: Rc<RefCell<Vec<Call>>>,
    rg_ids: Rc<RefCell<HashMap<u64, u32>>>,
    #[allow(dead_code)] // Shared test fixture: only some shader cases inspect this log.
    pub log: Rc<RefCell<Log>>,
}

impl Rig {
    /// `names`：反射里的名字哈希（`names[i]` → set 0 / binding `i`）；`uniforms`：常量块里的 uniform 名字
    /// （`uniforms[i]` → 偏移 `i * UNIFORM_STRIDE`）。`uniforms` 为空则反射没有 uniform 块。
    pub fn new(names: &[u64], uniforms: &[u64]) -> Rig {
        let (mut host, log) = MockHost::new();
        let mut bindings = HashMap::new();
        for (i, h) in names.iter().enumerate() {
            bindings.insert(*h, (0u32, i as u32));
        }
        let mut offsets = HashMap::new();
        for (i, h) in uniforms.iter().enumerate() {
            offsets.insert(*h, i as u32 * UNIFORM_STRIDE);
        }
        let refl = ShaderReflection {
            program_name: "draw_shaders_test",
            bindings,
            has_uniform_block: !uniforms.is_empty(),
            uniform_offsets: offsets,
            uniform_block_size: (uniforms.len() as u32 + 1) * UNIFORM_STRIDE,
            uniform_block_binding: 0x7f,
        };
        host.shaders.insert(1, Arc::new(refl));
        // `bind_ray_tracing` 读的两个缓冲字（`AssetSsboLibrary::get_buffer` → 首字低 32 位；值任意，供测试读回）。
        host.buffer_words
            .insert(RtTrianglesSsbo::rg_buffer_hash(), RT_TRIANGLES_WORD);
        host.buffer_words
            .insert(RtBvhSsbo::rg_buffer_hash(), RT_BVH_WORD);
        let calls = Rc::new(RefCell::new(Vec::new()));
        let rg_ids = Rc::new(RefCell::new(HashMap::new()));
        let host = RecHost {
            inner: host,
            calls: calls.clone(),
            rg_ids: rg_ids.clone(),
        };
        let pass = RasterPass::new("draw_shaders_test", &[], None, Box::new(host));
        Rig {
            pass,
            calls,
            rg_ids,
            log,
        }
    }

    pub fn builder(&mut self) -> DrawBuilder<'_> {
        let st = self.pass.host.default_raster_state();
        let shader = Handle(
            ManagedHandle(1),
            core::marker::PhantomData::<fn() -> RasterShaderProgram>,
        );
        DrawBuilder::new(&mut self.pass, DrawPacket::new(shader, [0, 1], st), 0)
    }

    /// 到目前为止宿主见到的调用（保序）。
    pub fn calls(&self) -> Vec<Call> {
        self.calls.borrow().clone()
    }

    /// `AssetSsboLibrary::get_rg_buffer` 给某个 `SsboId` 分配的 rg 资源下标。
    pub fn rg_id(&self, ssbo_id: u64) -> u32 {
        *self
            .rg_ids
            .borrow()
            .get(&ssbo_id)
            .expect("该 SSBO 从未被请求过")
    }

    /// 取走已提交的 draw 包并让 pass 通过 `render()` 检查。
    pub fn finish(&mut self) -> Vec<DrawPacket> {
        let d = core::mem::take(&mut self.pass.draws);
        self.pass.rendered.0 = true;
        d
    }
}

/// 读回常量块里第 `index` 个 uniform（`Rig::new` 的 `uniforms[index]`）起始的 `len` 字节。只在 draw 之前、
/// 持有 builder 时有数据可读（`DrawBuilder::constants_data`）。
pub fn uniform_at(data: &[u8], index: usize, len: usize) -> Vec<u8> {
    let off = index * UNIFORM_STRIDE as usize;
    data[off..off + len].to_vec()
}

/// 读 `i32` / `u32` / `f32` 的小工具。
pub fn as_i32(b: &[u8]) -> i32 {
    i32::from_le_bytes(b[..4].try_into().unwrap())
}
pub fn as_f32(b: &[u8]) -> f32 {
    f32::from_le_bytes(b[..4].try_into().unwrap())
}
