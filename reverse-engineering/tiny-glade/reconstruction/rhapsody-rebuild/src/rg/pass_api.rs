//! 渲染图回调的命令接口。函数名来自原 PDB，公开 Rust 签名是重建接口。
//! 原函数 0x1416f7870..0x1416f7e7a 的命令参数已用限定汇编校正。
//! 附件视图及 render pass/framebuffer 自动生产由 attachments 模块接入。

use ash::vk;

/// 数据只在当前调用中有效，后端不能保留这些裸指针。
pub enum Command<'a> {
    BeginRenderPass(&'a vk::RenderPassBeginInfo<'a>),
    EndRenderPass,
    SetViewport(vk::Viewport),
    SetScissor(vk::Rect2D),
    SetStencilRef(u8),
    SetLineWidth(f32),
    BindVertexBuffer {
        binding: u32,
        buffer: vk::Buffer,
    },
    BindIndexBuffer {
        buffer: vk::Buffer,
        ty: vk::IndexType,
    },
    PushConstants {
        layout: vk::PipelineLayout,
        stages: vk::ShaderStageFlags,
        bytes: &'a [u8],
    },
    CopyBuffer {
        source: vk::Buffer,
        destination: vk::Buffer,
        regions: &'a [vk::BufferCopy],
    },
    FillBuffer {
        buffer: vk::Buffer,
        offset: u64,
        size: u64,
        data: u32,
    },
    ClearColorImage {
        image: vk::Image,
        range: vk::ImageSubresourceRange,
    },
    Draw {
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    },
    DrawIndexed {
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        vertex_offset: i32,
        first_instance: u32,
    },
    DrawIndirect {
        buffer: vk::Buffer,
        offset: u64,
        draw_count: u32,
        stride: u32,
    },
    DrawIndirectCount {
        buffer: vk::Buffer,
        offset: u64,
        count_buffer: vk::Buffer,
        count_buffer_offset: u64,
        max_draw_count: u32,
        stride: u32,
    },
    DrawIndexedIndirect {
        buffer: vk::Buffer,
        offset: u64,
        draw_count: u32,
        stride: u32,
    },
    DrawIndexedIndirectCount {
        buffer: vk::Buffer,
        offset: u64,
        count_buffer: vk::Buffer,
        count_buffer_offset: u64,
        max_draw_count: u32,
        stride: u32,
    },
    DispatchGroups {
        x: u32,
        y: u32,
        z: u32,
    },
    DispatchIndirect {
        buffer: vk::Buffer,
        offset: u64,
    },
}

/// 实际 Vulkan 后端或候选测试后端实现同一调用边界。
pub trait CommandBackend {
    fn dispatch(&mut self, command_buffer: vk::CommandBuffer, command: Command<'_>);
}

/// 调用原锁定版本 ash 的设备方法，不重新实现 Vulkan。
pub struct AshCommandBackend<'a> {
    device: &'a ash::Device,
}

impl<'a> AshCommandBackend<'a> {
    /// # Safety
    /// 调用者必须确保后续传入本后端的命令缓冲、资源、布局、范围与录制状态
    /// 均满足 Vulkan 有效性要求，在使用期间保持存活；本适配器不拥有它们。
    pub unsafe fn new(device: &'a ash::Device) -> Self {
        Self { device }
    }
}

impl CommandBackend for AshCommandBackend<'_> {
    fn dispatch(&mut self, cb: vk::CommandBuffer, command: Command<'_>) {
        // 资源有效性由构造本后端时的安全契约及上层 managed 解析负责。
        unsafe {
            match command {
                Command::BeginRenderPass(info) => {
                    self.device
                        .cmd_begin_render_pass(cb, info, vk::SubpassContents::INLINE)
                }
                Command::EndRenderPass => self.device.cmd_end_render_pass(cb),
                Command::SetViewport(value) => self.device.cmd_set_viewport(cb, 0, &[value]),
                Command::SetScissor(value) => self.device.cmd_set_scissor(cb, 0, &[value]),
                Command::SetStencilRef(value) => self.device.cmd_set_stencil_reference(
                    cb,
                    vk::StencilFaceFlags::FRONT_AND_BACK,
                    value.into(),
                ),
                Command::SetLineWidth(value) => self.device.cmd_set_line_width(cb, value),
                Command::BindVertexBuffer { binding, buffer } => self
                    .device
                    .cmd_bind_vertex_buffers(cb, binding, &[buffer], &[0]),
                Command::BindIndexBuffer { buffer, ty } => {
                    self.device.cmd_bind_index_buffer(cb, buffer, 0, ty)
                }
                Command::PushConstants {
                    layout,
                    stages,
                    bytes,
                } => self.device.cmd_push_constants(cb, layout, stages, 0, bytes),
                Command::CopyBuffer {
                    source,
                    destination,
                    regions,
                } => self
                    .device
                    .cmd_copy_buffer(cb, source, destination, regions),
                Command::FillBuffer {
                    buffer,
                    offset,
                    size,
                    data,
                } => self.device.cmd_fill_buffer(cb, buffer, offset, size, data),
                Command::ClearColorImage { image, range } => self.device.cmd_clear_color_image(
                    cb,
                    image,
                    vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                    &vk::ClearColorValue { float32: [0.0; 4] },
                    &[range],
                ),
                Command::Draw {
                    vertex_count,
                    instance_count,
                    first_vertex,
                    first_instance,
                } => self.device.cmd_draw(
                    cb,
                    vertex_count,
                    instance_count,
                    first_vertex,
                    first_instance,
                ),
                Command::DrawIndexed {
                    index_count,
                    instance_count,
                    first_index,
                    vertex_offset,
                    first_instance,
                } => self.device.cmd_draw_indexed(
                    cb,
                    index_count,
                    instance_count,
                    first_index,
                    vertex_offset,
                    first_instance,
                ),
                Command::DrawIndirect {
                    buffer,
                    offset,
                    draw_count,
                    stride,
                } => self
                    .device
                    .cmd_draw_indirect(cb, buffer, offset, draw_count, stride),
                Command::DrawIndirectCount {
                    buffer,
                    offset,
                    count_buffer,
                    count_buffer_offset,
                    max_draw_count,
                    stride,
                } => self.device.cmd_draw_indirect_count(
                    cb,
                    buffer,
                    offset,
                    count_buffer,
                    count_buffer_offset,
                    max_draw_count,
                    stride,
                ),
                Command::DrawIndexedIndirect {
                    buffer,
                    offset,
                    draw_count,
                    stride,
                } => self
                    .device
                    .cmd_draw_indexed_indirect(cb, buffer, offset, draw_count, stride),
                Command::DrawIndexedIndirectCount {
                    buffer,
                    offset,
                    count_buffer,
                    count_buffer_offset,
                    max_draw_count,
                    stride,
                } => self.device.cmd_draw_indexed_indirect_count(
                    cb,
                    buffer,
                    offset,
                    count_buffer,
                    count_buffer_offset,
                    max_draw_count,
                    stride,
                ),
                Command::DispatchGroups { x, y, z } => self.device.cmd_dispatch(cb, x, y, z),
                Command::DispatchIndirect { buffer, offset } => {
                    self.device.cmd_dispatch_indirect(cb, buffer, offset)
                }
            }
        }
    }
}

/// 原 pipeline 对象 +0x30 的种类：0=图形，1=计算。
#[derive(Clone, Copy, Debug)]
pub enum PipelineKind {
    Graphics,
    Compute,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PassApiError {
    UnalignedFillOffset,
    UnalignedFillSize,
}

/// 原 command +0x56 是 stencil 缓存，初值由 command 生产者提供。
/// Rust 布局不是原 ABI；wide_lines 对应 device +0xad9 的门控语义。
pub struct RenderPassApi<'a, B> {
    backend: &'a mut B,
    command_buffer: vk::CommandBuffer,
    stencil_ref: u8,
    wide_lines: bool,
    /// 原入口在每次 draw/dispatch 调用上做 `AtomicUsize::fetch_add`；
    /// 候选用线程局部计数复刻可观察的调用计数行为。
    draw_count: usize,
}

impl<'a, B: CommandBackend> RenderPassApi<'a, B> {
    /// 原入口扫描当前 pass 的访问范围，返回第一个匹配项；不会借用其他 pass 的访问。
    /// 没有声明的资源与原入口一样失败。公开参数形式不声称等于原 ABI。
    pub fn find_resource_access_type(
        pass: &super::graph::PassDeclaration,
        resource: super::resource::ResourceHandle,
    ) -> u8 {
        pass.accesses
            .iter()
            .find(|access| access.resource_index == resource.raw())
            .unwrap_or_else(|| panic!("当前 pass 未声明资源 {} 的访问", resource.raw()))
            .access_code
    }
    pub fn new(
        backend: &'a mut B,
        command_buffer: vk::CommandBuffer,
        initial_stencil_ref: u8,
        wide_lines: bool,
    ) -> Self {
        Self {
            backend,
            command_buffer,
            stencil_ref: initial_stencil_ref,
            wide_lines,
            draw_count: 0,
        }
    }

    /// 原入口暴露的累计调用计数；仅用于诊断/统计，不影响命令语义。
    pub fn draw_call_count(&self) -> usize {
        self.draw_count
    }

    fn emit(&mut self, command: Command<'_>) {
        self.backend.dispatch(self.command_buffer, command);
    }

    /// 原入口还包含附件视图与缓存获取；本接口接收已解析对象，只恢复末端
    /// VkRenderPassBeginInfo/INLINE 调用，附件缓存生产者尚未恢复。
    pub fn begin_render_pass(&mut self, info: &vk::RenderPassBeginInfo<'_>) {
        self.emit(Command::BeginRenderPass(info));
    }

    /// 完整附件生产的末端入口；调用前由 PreparedRenderPass::keep_alive 登记资源。
    /// 外部渲染图负责进入附件布局的同步，避免在此重复插入未知原作屏障。
    pub fn begin_attachments(&mut self, prepared: &super::attachments::PreparedRenderPass) {
        self.begin_render_pass(&prepared.begin_info());
    }

    pub fn end_render_pass(&mut self) {
        self.emit(Command::EndRenderPass);
    }

    /// 原函数使用负 viewport 高度翻转 Y，scissor 仍覆盖整个正向范围。
    pub fn set_default_view_and_scissor(&mut self, extent: vk::Extent2D) {
        self.emit(Command::SetViewport(vk::Viewport {
            x: 0.0,
            y: extent.height as f32,
            width: extent.width as f32,
            height: -(extent.height as f32),
            min_depth: 0.0,
            max_depth: 1.0,
        }));
        self.emit(Command::SetScissor(vk::Rect2D {
            offset: vk::Offset2D::default(),
            extent,
        }));
    }

    pub fn set_stencil_ref(&mut self, value: u8) {
        if self.stencil_ref != value {
            self.emit(Command::SetStencilRef(value));
            self.stencil_ref = value;
        }
    }

    pub fn set_line_width(&mut self, value: f32) {
        if self.wide_lines {
            self.emit(Command::SetLineWidth(value));
        }
    }

    /// 原函数绑定一个 buffer，offset 固定零；managed 解析及 track 由调用者供给。
    pub fn bind_vertex_buffer_by_binding_point(&mut self, binding: u32, buffer: vk::Buffer) {
        self.emit(Command::BindVertexBuffer { binding, buffer });
    }

    pub fn bind_index_buffer(&mut self, buffer: vk::Buffer, ty: vk::IndexType) {
        self.emit(Command::BindIndexBuffer { buffer, ty });
    }

    /// 原图形掩码固定 0x1f、计算掩码固定 0x20，offset 固定零。
    pub fn push_constants(&mut self, layout: vk::PipelineLayout, kind: PipelineKind, bytes: &[u8]) {
        let stages = match kind {
            PipelineKind::Graphics => vk::ShaderStageFlags::ALL_GRAPHICS,
            PipelineKind::Compute => vk::ShaderStageFlags::COMPUTE,
        };
        self.emit(Command::PushConstants {
            layout,
            stages,
            bytes,
        });
    }

    pub fn copy_buffer(
        &mut self,
        source: vk::Buffer,
        destination: vk::Buffer,
        regions: &[vk::BufferCopy],
    ) {
        self.emit(Command::CopyBuffer {
            source,
            destination,
            regions,
        });
    }

    /// None 生成 offset=0、size=WHOLE_SIZE；显式范围的两个值都检查 4 字节对齐。
    /// 原失败会 panic；重建公开接口暂返回可处理的错误，该差异必须由上层处理。
    pub fn fill_buffer(
        &mut self,
        buffer: vk::Buffer,
        range: Option<(u64, u64)>,
        data: u32,
    ) -> Result<(), PassApiError> {
        let (offset, size) = range.unwrap_or((0, vk::WHOLE_SIZE));
        if range.is_some() {
            if offset & 3 != 0 {
                return Err(PassApiError::UnalignedFillOffset);
            }
            if size & 3 != 0 {
                return Err(PassApiError::UnalignedFillSize);
            }
        }
        self.emit(Command::FillBuffer {
            buffer,
            offset,
            size,
            data,
        });
        Ok(())
    }

    /// 原 clear 固定透明零色；mip/layer 总数来自已解析的纹理描述符。
    pub fn clear_color_image(&mut self, image: vk::Image, mip_levels: u8, array_layers: u32) {
        self.emit(Command::ClearColorImage {
            image,
            range: vk::ImageSubresourceRange {
                aspect_mask: vk::ImageAspectFlags::COLOR,
                base_mip_level: 0,
                level_count: mip_levels.into(),
                base_array_layer: 0,
                layer_count: array_layers,
            },
        });
    }

    pub fn draw(
        &mut self,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    ) {
        self.draw_count += 1;
        self.emit(Command::Draw {
            vertex_count,
            instance_count,
            first_vertex,
            first_instance,
        });
    }

    pub fn draw_indexed(
        &mut self,
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        vertex_offset: i32,
        first_instance: u32,
    ) {
        self.draw_count += 1;
        self.emit(Command::DrawIndexed {
            index_count,
            instance_count,
            first_index,
            vertex_offset,
            first_instance,
        });
    }

    pub fn draw_indirect(&mut self, buffer: vk::Buffer, offset: u64, draw_count: u32, stride: u32) {
        self.draw_count += 1;
        self.emit(Command::DrawIndirect {
            buffer,
            offset,
            draw_count,
            stride,
        });
    }

    /// 需要设备启用 `drawIndirectCount`；候选不校验特性是否已启用，调用者负责。
    pub fn draw_indirect_count(
        &mut self,
        buffer: vk::Buffer,
        offset: u64,
        count_buffer: vk::Buffer,
        count_buffer_offset: u64,
        max_draw_count: u32,
        stride: u32,
    ) {
        self.draw_count += 1;
        self.emit(Command::DrawIndirectCount {
            buffer,
            offset,
            count_buffer,
            count_buffer_offset,
            max_draw_count,
            stride,
        });
    }

    pub fn draw_indexed_indirect(
        &mut self,
        buffer: vk::Buffer,
        offset: u64,
        draw_count: u32,
        stride: u32,
    ) {
        self.draw_count += 1;
        self.emit(Command::DrawIndexedIndirect {
            buffer,
            offset,
            draw_count,
            stride,
        });
    }

    /// 需要设备启用 `drawIndirectCount`；候选不校验特性是否已启用，调用者负责。
    pub fn draw_indexed_indirect_count(
        &mut self,
        buffer: vk::Buffer,
        offset: u64,
        count_buffer: vk::Buffer,
        count_buffer_offset: u64,
        max_draw_count: u32,
        stride: u32,
    ) {
        self.draw_count += 1;
        self.emit(Command::DrawIndexedIndirectCount {
            buffer,
            offset,
            count_buffer,
            count_buffer_offset,
            max_draw_count,
            stride,
        });
    }

    pub fn dispatch_groups_3d(&mut self, x: u32, y: u32, z: u32) {
        self.draw_count += 1;
        self.emit(Command::DispatchGroups { x, y, z });
    }

    /// 原入口按线程总数 `div_ceil` group 大小换算 group 数，再内联调用
    /// `dispatch_groups_3d`；这里保留同样两段式顺序，不重复计数两次的语义差异
    /// 由调用方感知（原机器码内联后仍只触发一次 `cmd_dispatch`）。
    pub fn dispatch_threads_3d(
        &mut self,
        threads_x: u32,
        threads_y: u32,
        threads_z: u32,
        group_size_x: u32,
        group_size_y: u32,
        group_size_z: u32,
    ) {
        self.dispatch_groups_3d(
            threads_x.div_ceil(group_size_x.max(1)),
            threads_y.div_ceil(group_size_y.max(1)),
            threads_z.div_ceil(group_size_z.max(1)),
        );
    }

    pub fn dispatch_indirect(&mut self, buffer: vk::Buffer, offset: u64) {
        self.draw_count += 1;
        self.emit(Command::DispatchIndirect { buffer, offset });
    }
}
