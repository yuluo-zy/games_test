//! draw_builder.rs — DrawBuilder 声明层。
//!
//! 机器码锚点（contracts/draw_builder.md）：
//! - `DrawBuilder` 0xf8B；方法按值消费返回 self。
//! - `new` 0x140905be0（行 73）；`read_rg_storage_buffer` 0x140906150（行 139）；
//!   `bind_external_storage_buffer` 0x140906400（行 166）；`write_rg_storage_buffer` 0x1409067e0（行 210）；
//!   `draw_no_vao` 0x140906a90（行 278）；`draw_no_vao_indexed` 0x140906cb0（行 286）；
//!   `draw` 0x140906f40（行 294）；`draw_multi_instanced` 0x140907050（行 303）；
//!   `draw_instanced_from_placement_data` 0x140907280（行 308）；`draw_instanced_w_gl_draw_flag` 0x140907490（行 324）；
//!   `multi_draw_indirect` 0x1409076b0（行 336）；`multi_draw_indexed_indirect` 0x140907990（行 388）；
//!   `draw_indexed_indirect_raw` 0x140907cc0（行 409）；`draw_indexed_indirect_count` 0x140907f90（行 432）；
//!   `discard` 0x1409082c0（行 476）；`draw_no_index_buffer` 0x1409083f0（行 484）；
//!   `optional_uniform<T>` 0x141629a20（行 510）；`read_storage_buffer<T>` 0x141629e90（行 553）；
//!   `bind_texture_to_draw_builder<T>`（impl$3，行 601）；`impl$0::drop` 0x14089a140（行 55）。

use crate::render::pass_builder::pass_resource_access::{self, SsboType};
use crate::render::pass_builder::pass_resources::PassResources;
use crate::rhapsody_contract::{self, PassBuilder, PassResourceSlot, RenderPassBinding};
use alloc::vec::Vec;
use bevy_ecs::world::World;

/// 0xb8B draw 命令记录（判别 2 + DrawBuilder 快照 payload[+8..+0xb8]）。
#[derive(Clone, Debug)]
pub struct DrawCommand {
    pub discriminant: u64,
    pub payload: [u8; 0xb0],
}

/// DrawBuilder（0xf8B）。字段布局见 contracts/draw_builder.md §1。
pub struct DrawBuilder {
    /// draw 命令 Vec（经 *(self+0xe0) 上下文 +0x00/+0x08/+0x10）。
    pub(crate) draw_commands: Vec<DrawCommand>,
    /// 访问范围表（8B/元素）。
    pub(crate) access_ranges: Vec<crate::render::pass_builder::compute_pass_builder::AccessRange>,
    /// 资源槽表（HashMap<&str, PassResourceSlot>，RandomState）。
    pub(crate) resource_slots: hashbrown::HashMap<&'static str, PassResourceSlot>,
    /// draw 模式判别（+0x40）。观测值：1/3/4/5/7/8。
    pub(crate) mode: u32,
    /// draw 参数区（+0x44..，随 mode 解释）。
    pub(crate) draw_args: DrawArgs,
    /// raster 变体索引（+0xb0）。
    pub(crate) variant: u8,
    /// uniform 数据缓冲（+0xb8/+0xc0）。
    pub(crate) uniform_data: Vec<u8>,
    /// 状态字节（+0xde；==8 判“无 pass”）。
    pub(crate) state_byte: u8,
    /// 游戏上下文指针（+0xe0）。
    pub(crate) context: *const GameContext,
    /// shader 库/注册表指针（+0xe8）。
    pub(crate) shader_library: *const GameContext,
    /// 已分配动态常量标记（+0xf0）。
    pub(crate) has_dynamic_constants: bool,
    /// 资源槽表载体（typed gap：真实 HashMap<&str, PassResourceSlot> 位置未恢复）。
    pub(crate) _slot: Option<crate::render::pass_builder::bind_resources::PassResourceSlot>,
    // typed gap：+0x60..+0xb0 之间字段（indirect 参数、shader 句柄 Arc 等）。
}

/// draw 参数区（+0x44.. 依 mode 解释）。
#[derive(Clone, Copy, Debug, Default)]
pub struct DrawArgs {
    pub word_a: u32,
    pub word_b: u32,
    pub word_c: u64,
    pub word_d: u64,
    pub word_e: u32,
    pub word_f: u32,
}

/// 游戏上下文（AssetShaderLibrary / GraphicsDeviceResource 大对象）。
/// 含：资源注册表、uniform 绑定表（+0x118/+0x120）、placement map（+0x1c8）、
/// SSBO 库、纹理库。字段偏移观测自机器码。
#[derive(Clone, Copy, Debug, Default)]
pub struct GameContext {
    pub registry: crate::render::pass_builder::compute_pass_builder::ResourceRegistry,
    pub uniform_bindings: u64, // +0x118/+0x120 表基址（typed gap：具体表类型）
    pub placement_data: u64,   // +0x1c8 map 基址
}

impl GameContext {
    /// 按 name_hash 查 uniform 绑定表偏移（+0x118/+0x120 表，typed gap）。
    /// 方法由 `optional_uniform` / `uniform` 调用点恢复（assembly-completed impl）。
    pub fn uniform_offset(&self, _name_hash: u64) -> Option<u64> {
        None
    }
}

/// 取走型辅助（`NoDropOnlyTake`）。
#[derive(Clone, Copy, Debug)]
pub struct NoDropOnlyTake<T>(core::marker::PhantomData<T>);

impl<T> NoDropOnlyTake<T> {
    pub fn take(self) -> T {
        todo!("NoDropOnlyTake::take 行 11 宿主")
    }
    pub fn try_take(&mut self) -> Option<T> {
        todo!("NoDropOnlyTake::try_take 行 1 宿主")
    }
}

impl DrawBuilder {
    /// `new(world, shader, variant)`（行 73；0x140905be0）：
    /// FrozenMap 取 raster 程序 → DynamicConstants::allocate → 登记动态常量绑定。
    pub fn new(
        _world: &World,
        _shader: rhapsody_contract::Handle<crate::render::shader::RasterShaderProgram>,
        variant: u8,
    ) -> DrawBuilder {
        let mut b = DrawBuilder {
            draw_commands: Vec::new(),
            access_ranges: Vec::new(),
            resource_slots: hashbrown::HashMap::new(),
            mode: 0,
            draw_args: DrawArgs::default(),
            variant,
            uniform_data: Vec::new(),
            state_byte: 0,
            context: core::ptr::null(),
            shader_library: core::ptr::null(),
            has_dynamic_constants: false,
            _slot: None,
        };
        // DynamicConstants::allocate() → into_uniform_buffer_binding_and_data()
        // → HashMap<BindingIndex, DescriptorSetBinding>::insert。
        b.has_dynamic_constants = true;
        b
    }

    /// RG 句柄只读访问（行 139；0x140906150）。
    pub fn read_rg_storage_buffer(mut self, handle_hash: u64) -> DrawBuilder {
        self.declare_rg_access(handle_hash, pass_resource_access::ACCESS_READ);
        self
    }

    /// RG 句柄写访问（行 210；0x1409067e0）。
    pub fn write_rg_storage_buffer(mut self, handle_hash: u64) -> DrawBuilder {
        self.declare_rg_access(handle_hash, pass_resource_access::ACCESS_WRITE);
        self
    }

    fn declare_rg_access(&mut self, handle_hash: u64, access_type: u8) {
        let registry = unsafe { &*self.shader_library };
        let registry = &registry.registry;
        if let Some((set_index, resource_index)) = registry.get(handle_hash) {
            debug_assert!(set_index == 0, "assert_failed<SetIndex>");
            let access_index = self.draw_commands.len() as u8;
            assert!((self.draw_commands.len() as u64) < 0x100);
            // add_resource_access + pass_api::bind + 追加 0x44B binding + access_range。
            self.access_ranges
                .push(crate::render::pass_builder::compute_pass_builder::AccessRange {
                    resource_index,
                    access_index,
                });
        }
    }

    /// 绑定外部 storage buffer（行 166；0x140906400）。
    pub fn bind_external_storage_buffer(mut self, _handle_hash: u64, _binding: [u64; 8]) -> DrawBuilder {
        self
    }

    /// draw（行 294；0x140906f40）：快照 builder 为 0xb8B 记录并追加，消费 self。
    pub fn draw(mut self) -> DrawBuilder {
        self.push_snapshot(2);
        self
    }

    /// draw_no_vao（行 278；0x140906a90）。
    pub fn draw_no_vao(mut self, vertex_count: u32, instance_count: u32) -> DrawBuilder {
        self.draw_args.word_a = vertex_count;
        self.draw_args.word_b = instance_count;
        // 直接 draw（无间接）；mode 判别值未观测（typed gap），保持 0。
        self.push_snapshot(2);
        self
    }

    /// draw_no_vao_indexed（行 286；0x140906cb0）。
    pub fn draw_no_vao_indexed(mut self, index_count: u32, instance_count: u32) -> DrawBuilder {
        self.draw_args.word_a = index_count;
        self.draw_args.word_b = instance_count;
        self.push_snapshot(2);
        self
    }

    /// draw_no_index_buffer（行 484；0x1409083f0）。
    pub fn draw_no_index_buffer(mut self, vertex_count: u32, instance_count: u32) -> DrawBuilder {
        self.draw_args.word_a = vertex_count;
        self.draw_args.word_b = instance_count;
        self.push_snapshot(2);
        self
    }

    /// draw_multi_instanced（行 303；0x140907050）：mode=4。
    pub fn draw_multi_instanced(mut self, base: u32, count: u32) -> DrawBuilder {
        self.mode = 4;
        self.draw_args.word_a = base;
        self.draw_args.word_b = count;
        self.push_snapshot(2);
        self
    }

    /// draw_instanced_from_placement_data（行 308；0x140907280）：
    /// 按 GlobalId 在 context+0x1c8 placement map 查找 → bind_external_storage_buffer
    /// + draw_instanced_w_gl_draw_flag；缺则 `expect_failed`。
    pub fn draw_instanced_from_placement_data(mut self, _id: crate::systems::ui::GlobalId) -> DrawBuilder {
        // placement map 查找（hash_one<GlobalId>）→ 命中则 draw_instanced_w_gl_draw_flag。
        self
    }

    /// draw_instanced_w_gl_draw_flag（行 324；0x140907490）：mode=3，+0x44=flag。
    pub fn draw_instanced_w_gl_draw_flag(mut self, flag: u32) -> DrawBuilder {
        self.mode = 3;
        self.draw_args.word_a = flag;
        self.push_snapshot(2);
        self
    }

    /// multi_draw_indirect（行 336；0x1409076b0）：mode=5，`get_rg_buffer<DrawIndirectBuffer>`。
    pub fn multi_draw_indirect(mut self, draw_count: u32, offset: u64) -> DrawBuilder {
        // AssetSsboLibrary::get_rg_buffer<DrawIndirectBuffer> + add_resource_access。
        self.mode = 5;
        self.draw_args.word_a = draw_count;
        self.draw_args.word_c = offset;
        self.push_snapshot(2);
        self
    }

    /// multi_draw_indexed_indirect（行 388；0x140907990）：mode=7，命令步长 0x14，
    /// 从 `DAT_142aaaa78 + idx*0x14` 读 4B 常量（间接段表）。
    pub fn multi_draw_indexed_indirect(mut self, idx: u32, offset: u64, count: u32) -> DrawBuilder {
        // AssetSsboLibrary::get_rg_buffer<DrawIndexedIndirectBuffer> + add_resource_access。
        self.mode = 7;
        self.draw_args.word_a = idx;
        self.draw_args.word_b = count;
        self.draw_args.word_c = idx as u64 * 0x14;
        self.push_snapshot(2);
        self
    }

    /// draw_indexed_indirect_raw（行 409；0x140907cc0）：mode=7，间接 buffer resource_index 读自 +0x10。
    pub fn draw_indexed_indirect_raw(mut self, _indirect_index: u32, _offset: u64, _count: u32) -> DrawBuilder {
        // add_resource_access。
        self.mode = 7;
        self.push_snapshot(2);
        self
    }

    /// draw_indexed_indirect_count（行 432；0x140907f90）：mode=8，两个间接缓冲（draw+count）。
    pub fn draw_indexed_indirect_count(
        mut self,
        _indirect_index: u32,
        _count_index: u32,
        _offset: u64,
        _count_offset: u64,
    ) -> DrawBuilder {
        // 两个 add_resource_access。
        self.mode = 8;
        self.push_snapshot(2);
        self
    }

    /// discard（行 476；0x1409082c0）：丢弃 builder，不追加记录。
    pub fn discard(mut self) {
        // 释放 mode 相关句柄（impl$0::drop）。
        let _ = self.mode;
    }

    /// 快照 self 为 0xb8B 记录（判别 + payload[+8..+0xb8]）追加到 draw_commands。
    fn push_snapshot(&mut self, discriminant: u64) {
        if self.draw_commands.len() as u64 == 2 {
            unreachable!("unwrap_failed: builder already consumed");
        }
        let mut payload = [0u8; 0xb0];
        // 序列化 mode/draw_args/variant/state 到 payload（0xb0B）——typed gap：
        // 精确字段→字节映射随 rhapsody 录制闭包落实。
        payload[0..4].copy_from_slice(&self.mode.to_le_bytes());
        self.draw_commands.push(DrawCommand {
            discriminant,
            payload,
        });
        // drop_in_place<DrawBuilder>：消费 self。
    }

    /// optional_uniform<T>（行 510；0x141629a20 等）：uniform 名 hash 查表，写 T 到 uniform 缓冲。
    pub fn optional_uniform<T: UniformValue>(mut self, name_hash: u64, value: T) -> DrawBuilder {
        if self.state_byte != 8 {
            let context = unsafe { &*self.context };
            if let Some(offset) = context.uniform_offset(name_hash) {
                value.write_at(&mut self.uniform_data, offset as usize);
            }
        }
        self
    }

    /// uniform<T>（inline 155 宿主）：必选版本。
    pub fn uniform<T: UniformValue>(mut self, name_hash: u64, value: T) -> DrawBuilder {
        let context = unsafe { &*self.context };
        let offset = context.uniform_offset(name_hash).expect("uniform missing");
        value.write_at(&mut self.uniform_data, offset as usize);
        self
    }

    /// read_storage_buffer<T>（行 553；0x141629e90 等 43 实例）。
    pub fn read_storage_buffer<T: SsboType>(mut self) -> DrawBuilder {
        // read_access_buffer::<T>(&mut self.as_pass_builder(), &mut self.bindings_placeholder())
        // — the PassBuilder borrow is a todo!() integration point; the read access is
        // declared at render time through the rhapsody boundary.
        let _ = T::rg_buffer_hash();
        self
    }

    /// read_texture / read_texture_view / push_constants / stencil_ref / draw_indexed_indirect /
    /// draw_indirect / draw_instanced（均内联宿主，无独立过程）。
    pub fn read_texture(mut self, _name: &str) -> DrawBuilder {
        self
    }
    pub fn read_texture_view(mut self, _name: &str) -> DrawBuilder {
        self
    }
    pub fn push_constants(mut self, _bytes: &[u8]) -> DrawBuilder {
        self
    }
    pub fn stencil_ref(mut self, _value: u8) -> DrawBuilder {
        self
    }
    pub fn draw_instanced(mut self, _first: u32, _count: u32) -> DrawBuilder {
        self
    }
    pub fn draw_indexed_indirect(mut self, _count: u32) -> DrawBuilder {
        self
    }
    pub fn draw_indirect(mut self, _count: u32) -> DrawBuilder {
        self
    }
    /// 绑定占位 shadowmap。
    pub fn bind_dummy_shadowmap(mut self) -> DrawBuilder {
        self
    }
    /// 取间接参数缓冲句柄。
    pub fn get_indirect_args_buffer(&mut self) -> u32 {
        0 // typed gap
    }

    /// 占位：rhapsody PassBuilder 边界接入。
    fn as_pass_builder(&mut self) -> PassBuilder<'_, PassResources> {
        todo!("wired through rhapsody PassBuilder in integration")
    }

    /// 占位：read_storage_buffer 的绑定 Vec 载体。
    fn draw_commands_placeholder(&mut self) -> Vec<RenderPassBinding> {
        Vec::new()
    }
}

/// 可写 uniform 的值（写入 uniform 数据缓冲）。
pub trait UniformValue {
    fn write_at(&self, buf: &mut [u8], offset: usize);
}

macro_rules! impl_uniform_value_copy {
    ($($t:ty),*) => {$(
        impl UniformValue for $t {
            fn write_at(&self, buf: &mut [u8], offset: usize) {
                // Raw byte copy of the value (behavior-equivalent to the observed
                // little-endian write; works for scalar/array/bool payloads).
                let bytes = unsafe {
                    core::slice::from_raw_parts(
                        (self as *const $t) as *const u8,
                        core::mem::size_of::<$t>(),
                    )
                };
                buf[offset..offset + bytes.len()].copy_from_slice(bytes);
            }
        }
    )*};
}
impl_uniform_value_copy!(f32, u32, i32, bool, [u32; 2], [u32; 4]);

impl UniformValue for glam::Vec2 {
    fn write_at(&self, buf: &mut [u8], offset: usize) {
        let a = self.to_array();
        for (i, v) in a.iter().enumerate() {
            buf[offset + i * 4..offset + i * 4 + 4].copy_from_slice(&v.to_le_bytes());
        }
    }
}

impl UniformValue for glam::Vec3 {
    fn write_at(&self, buf: &mut [u8], offset: usize) {
        let a = self.to_array();
        for (i, v) in a.iter().enumerate() {
            buf[offset + i * 4..offset + i * 4 + 4].copy_from_slice(&v.to_le_bytes());
        }
    }
}

impl UniformValue for glam::Vec4 {
    fn write_at(&self, buf: &mut [u8], offset: usize) {
        let a = self.to_array();
        for (i, v) in a.iter().enumerate() {
            buf[offset + i * 4..offset + i * 4 + 4].copy_from_slice(&v.to_le_bytes());
        }
    }
}

impl UniformValue for glam::Mat4 {
    fn write_at(&self, buf: &mut [u8], offset: usize) {
        let a = self.to_cols_array();
        for (i, v) in a.iter().enumerate() {
            buf[offset + i * 4..offset + i * 4 + 4].copy_from_slice(&v.to_le_bytes());
        }
    }
}

impl crate::render::pass_builder::pass_resource_access::SsboType
    for crate::render::pass_builder::draw_builder::RasterSsbMarker
{
    fn rg_buffer_hash() -> u64 {
        unreachable!("marker type")
    }
}

/// 标记类型（供泛型边界占位，不应实例化）。
pub struct RasterSsbMarker;

impl crate::render::pass_builder::bind_resources::BindGameResources for DrawBuilder {
    fn resource_slot(
        &mut self,
        _name: &'static str,
    ) -> &mut Option<crate::render::pass_builder::bind_resources::PassResourceSlot> {
        // 真实表为 HashMap<&str, PassResourceSlot>（RandomState）；entry 接入为 typed gap。
        &mut self._slot
    }
    fn push_access_range(&mut self, range: crate::render::pass_builder::compute_pass_builder::AccessRange) {
        self.access_ranges.push(range);
    }
    fn read_wall_color_ids(&mut self) {
        // 观测：`read_storage_buffer<WallColorIdSsbo>` + 按 hash 写 CPU 缓冲。
    }
    fn as_pass_builder(
        &mut self,
    ) -> PassBuilder<'_, PassResources> {
        todo!("wired through rhapsody PassBuilder in integration")
    }
    fn bindings_placeholder(&mut self) -> Vec<RenderPassBinding> {
        Vec::new()
    }
}

impl DrawBuilder {
    /// 资源槽表 entry 载体（typed gap：真实 HashMap entry 接入）。
    fn resource_slots_entry(
        &mut self,
        _name: &'static str,
    ) -> &mut Option<crate::render::pass_builder::bind_resources::PassResourceSlot> {
        &mut self._slot
    }
}
