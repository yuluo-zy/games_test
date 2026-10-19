//! Typed commands recorded by render-graph callbacks.
use ash::vk;

/// 数据只在当前调用中有效，后端不能保留这些裸指针。
pub enum Command<'a> {
    BeginRendering(&'a vk::RenderingInfo<'a>),
    EndRendering,
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

pub trait CommandBackend {
    fn dispatch(&mut self, command_buffer: vk::CommandBuffer, command: Command<'_>);
    fn supports_dynamic_rendering(&self) -> bool {
        false
    }
}

/// 直接调用 ash 的设备方法。
pub struct AshCommandBackend<'a> {
    device: &'a ash::Device,
    rendering: Option<DynamicRenderingCommands>,
}
enum DynamicRenderingCommands {
    Core,
    Khr(ash::khr::dynamic_rendering::Device),
}

impl<'a> AshCommandBackend<'a> {
    /// # Safety
    /// 调用者必须确保后续传入本后端的命令缓冲、资源、布局、范围与录制状态
    /// 均满足 Vulkan 有效性要求，在使用期间保持存活；本适配器不拥有它们。
    pub unsafe fn new(device: &'a ash::Device) -> Self {
        Self {
            device,
            rendering: None,
        }
    }
    /// Select only enabled commands, including the KHR aliases on Vulkan 1.2.
    ///
    /// # Safety
    /// The command/resource/recording requirements of `new` also apply here.
    pub unsafe fn new_for_device(device: &'a crate::raw::device::Device) -> Self {
        let rendering = if !device
            .caps()
            .is_enabled(crate::device::Feature::DynamicRendering)
        {
            None
        } else if device
            .enabled_extensions
            .iter()
            .any(|extension| extension == "VK_KHR_dynamic_rendering")
        {
            Some(DynamicRenderingCommands::Khr(
                ash::khr::dynamic_rendering::Device::new(&device.instance.instance, &device.device),
            ))
        } else {
            let properties = unsafe {
                device
                    .instance
                    .instance
                    .get_physical_device_properties(device.physical)
            };
            (properties.api_version.min(device.instance.api_version) >= vk::API_VERSION_1_3)
                .then_some(DynamicRenderingCommands::Core)
        };
        Self {
            device: &device.device,
            rendering,
        }
    }
}

impl CommandBackend for AshCommandBackend<'_> {
    fn supports_dynamic_rendering(&self) -> bool {
        self.rendering.is_some()
    }
    fn dispatch(&mut self, cb: vk::CommandBuffer, command: Command<'_>) {
        // 资源有效性由构造本后端时的安全契约及上层 managed 解析负责。
        unsafe {
            match command {
                Command::BeginRendering(info) => match self
                    .rendering
                    .as_ref()
                    .expect("dynamic command capability checked before begin")
                {
                    DynamicRenderingCommands::Core => self.device.cmd_begin_rendering(cb, info),
                    DynamicRenderingCommands::Khr(loader) => loader.cmd_begin_rendering(cb, info),
                },
                Command::EndRendering => match self
                    .rendering
                    .as_ref()
                    .expect("dynamic command capability checked before begin")
                {
                    DynamicRenderingCommands::Core => self.device.cmd_end_rendering(cb),
                    DynamicRenderingCommands::Khr(loader) => loader.cmd_end_rendering(cb),
                },
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

pub struct RenderPassApi<'a, B> {
    backend: &'a mut B,
    command_buffer: vk::CommandBuffer,
    stencil_ref: u8,
    wide_lines: bool,
    draw_count: usize,
    dynamic_rendering_active: bool,
}

impl<'a, B: CommandBackend> RenderPassApi<'a, B> {
    pub fn find_resource_access_type(
        pass: &super::graph::PassDeclaration,
        resource: super::resource::ResourceHandle,
    ) -> crate::Result<u8> {
        if pass
            .graph_id
            .is_some_and(|expected| expected != resource.graph_id())
        {
            return Err(crate::Error::invalid_usage(
                "resource belongs to another render graph",
            ));
        }
        pass.accesses
            .iter()
            .find(|access| access.resource_index == resource.raw())
            .map(|access| access.access_code)
            .ok_or_else(|| {
                crate::Error::invalid_usage(format!(
                    "当前 pass 未声明资源 {} 的访问",
                    resource.raw()
                ))
            })
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
            dynamic_rendering_active: false,
        }
    }

    pub fn draw_call_count(&self) -> usize {
        self.draw_count
    }

    fn emit(&mut self, command: Command<'_>) {
        self.backend.dispatch(self.command_buffer, command);
    }

    /// Begin the selected production attachment path. The prepared owner must
    /// remain owned by the recorded command until GPU completion.
    pub fn begin_prepared(
        &mut self,
        prepared: &super::attachments::PreparedAttachments,
    ) -> crate::Result<()> {
        match prepared {
            super::attachments::PreparedAttachments::Dynamic(rendering) => {
                self.begin_dynamic_rendering(&rendering.begin_info())?;
            }
        }
        Ok(())
    }

    /// The graph has already transitioned the supplied attachments to their
    /// rendering layouts. All image views must remain alive until GPU completion.
    pub fn begin_dynamic_rendering(&mut self, info: &vk::RenderingInfo<'_>) -> crate::Result<()> {
        if !self.backend.supports_dynamic_rendering() {
            return Err(crate::Error::unsupported(
                "command backend has no enabled dynamic rendering entry points",
            ));
        }
        self.emit(Command::BeginRendering(info));
        self.dynamic_rendering_active = true;
        Ok(())
    }

    pub fn end_render_pass(&mut self) {
        if std::mem::take(&mut self.dynamic_rendering_active) {
            self.emit(Command::EndRendering);
        }
    }

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

    pub fn bind_vertex_buffer_by_binding_point(&mut self, binding: u32, buffer: vk::Buffer) {
        self.emit(Command::BindVertexBuffer { binding, buffer });
    }

    pub fn bind_index_buffer(&mut self, buffer: vk::Buffer, ty: vk::IndexType) {
        self.emit(Command::BindIndexBuffer { buffer, ty });
    }

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
    /// 参数无效时返回可处理的错误。
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

    /// 清除为透明零色；mip/layer 总数来自已解析的纹理描述符。
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

    /// `dispatch_groups_3d`；这里保留同样两段式顺序，不重复计数两次的语义差异
    /// 由调用方感知（只触发一次 `cmd_dispatch`）。
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
