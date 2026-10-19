//! 三角形绘制测试的公共装置：着色器、渲染通道、管线、命令录制与读回。全部走 rhapsody 自己的接口。
#![allow(dead_code)]
#[path = "../gpu_test_support/mod.rs"]
pub mod gpu_test_support;
use ash::vk;
use rhapsody::raw::{
    buffer::BufferDesc,
    command_buffer::{PendingCommand, VulkanCommandPool},
    device::Device,
    init::create_headless_device,
    managed::{
        buffer::ManagedBuffer,
        texture::{ManagedTexture, ManagedTextureView},
    },
    shader::{
        DynamicRenderingFormats, PipelineCache, PipelineLayout, RasterPipelineDesc,
        RasterPipelineOpts, ShaderModule, VulkanPipeline,
    },
    texture::{TextureDesc, TextureViewDesc},
};
use std::sync::Arc;

pub const SIZE: u32 = 64;
pub const FILL: [u8; 4] = [255, 0, 0, 255];
pub type Triangle = [[f32; 4]; 3];

/// 顶点 `gl_Position = position`（location 0 的 vec4）；片元输出常量红色。
pub fn shaders() -> (Vec<u8>, Vec<u8>) {
    use rspirv::binary::Assemble;
    use rspirv::dr::{Builder, Operand};
    use rspirv::spirv::{
        AddressingModel, BuiltIn, Capability, Decoration, ExecutionMode, ExecutionModel,
        FunctionControl, MemoryModel, StorageClass,
    };
    fn bytes(words: Vec<u32>) -> Vec<u8> {
        words.into_iter().flat_map(u32::to_le_bytes).collect()
    }
    let vertex = {
        let mut b = Builder::new();
        b.set_version(1, 3);
        b.capability(Capability::Shader);
        b.memory_model(AddressingModel::Logical, MemoryModel::GLSL450);
        let void = b.type_void();
        let fn_ty = b.type_function(void, vec![]);
        let float = b.type_float(32);
        let vec4 = b.type_vector(float, 4);
        let in_ptr = b.type_pointer(None, StorageClass::Input, vec4);
        let out_ptr = b.type_pointer(None, StorageClass::Output, vec4);
        let position = b.variable(in_ptr, None, StorageClass::Input, None);
        b.decorate(position, Decoration::Location, [Operand::LiteralInt32(0)]);
        let gl_position = b.variable(out_ptr, None, StorageClass::Output, None);
        b.decorate(
            gl_position,
            Decoration::BuiltIn,
            [Operand::BuiltIn(BuiltIn::Position)],
        );
        let main = b
            .begin_function(void, None, FunctionControl::NONE, fn_ty)
            .unwrap();
        b.begin_block(None).unwrap();
        let v = b.load(vec4, None, position, None, []).unwrap();
        b.store(gl_position, v, None, []).unwrap();
        b.ret().unwrap();
        b.end_function().unwrap();
        b.entry_point(
            ExecutionModel::Vertex,
            main,
            "vs_main",
            [position, gl_position],
        );
        bytes(b.module().assemble())
    };
    let fragment = {
        let mut b = Builder::new();
        b.set_version(1, 3);
        b.capability(Capability::Shader);
        b.memory_model(AddressingModel::Logical, MemoryModel::GLSL450);
        let void = b.type_void();
        let fn_ty = b.type_function(void, vec![]);
        let float = b.type_float(32);
        let vec4 = b.type_vector(float, 4);
        let out_ptr = b.type_pointer(None, StorageClass::Output, vec4);
        let color = b.variable(out_ptr, None, StorageClass::Output, None);
        b.decorate(color, Decoration::Location, [Operand::LiteralInt32(0)]);
        let one = b.constant_f32(float, 1.0);
        let zero = b.constant_f32(float, 0.0);
        let value = b.constant_composite(vec4, [one, zero, zero, one]);
        let main = b
            .begin_function(void, None, FunctionControl::NONE, fn_ty)
            .unwrap();
        b.begin_block(None).unwrap();
        b.store(color, value, None, []).unwrap();
        b.ret().unwrap();
        b.end_function().unwrap();
        b.entry_point(ExecutionModel::Fragment, main, "ps_main", [color]);
        b.execution_mode(main, ExecutionMode::OriginUpperLeft, []);
        bytes(b.module().assemble())
    };
    (vertex, fragment)
}

fn image_barrier(
    device: &ash::Device,
    cb: vk::CommandBuffer,
    image: vk::Image,
    (old, new): (vk::ImageLayout, vk::ImageLayout),
    src: (vk::PipelineStageFlags, vk::AccessFlags),
    dst: (vk::PipelineStageFlags, vk::AccessFlags),
) {
    let barrier = vk::ImageMemoryBarrier::default()
        .old_layout(old)
        .new_layout(new)
        .src_access_mask(src.1)
        .dst_access_mask(dst.1)
        .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .image(image)
        .subresource_range(vk::ImageSubresourceRange {
            aspect_mask: vk::ImageAspectFlags::COLOR,
            base_mip_level: 0,
            level_count: 1,
            base_array_layer: 0,
            layer_count: 1,
        });
    unsafe {
        device.cmd_pipeline_barrier(
            cb,
            src.0,
            dst.0,
            vk::DependencyFlags::empty(),
            &[],
            &[],
            &[barrier],
        )
    };
}

/// 一个渲染目标：纹理 + 视图 + 帧缓冲 + 读回缓冲。
pub struct Target {
    pub texture: ManagedTexture,
    pub view: ManagedTextureView,
    pub readback: ManagedBuffer,
}

pub struct Rig {
    pub device: Arc<Device>,
    pub gd: Arc<Device>,
    pub pipeline: Arc<VulkanPipeline>,
    pub pool: Arc<VulkanCommandPool>,
}

impl Rig {
    /// 无 Vulkan 设备时返回 None；严格 GPU 作业中的初始化错误直接失败。
    pub fn new() -> Option<Self> {
        let device = gpu_test_support::initialize("triangle device", create_headless_device())?;
        Some(Self::with_device(device))
    }

    /// 用调用方配置好 `frames_in_flight`/队列的设备构造同一套场景装置。
    pub fn with_device(device: Arc<Device>) -> Self {
        let gd = device.clone();
        let (vs, fs) = shaders();
        let vertex = ShaderModule::from_spirv(device.clone(), &vs).unwrap();
        let fragment = ShaderModule::from_spirv(device.clone(), &fs).unwrap();
        let layout = PipelineLayout::new(device.clone(), vec![], vec![]).unwrap();
        let cache = PipelineCache::new(device.clone(), &[]).unwrap();
        let mut desc = RasterPipelineDesc::new(vertex, layout);
        desc.fragment = Some(fragment);
        desc.fragment_entry = Some(std::ffi::CString::new("ps_main").unwrap());
        desc.use_dynamic_rendering(DynamicRenderingFormats::color(vk::Format::R8G8B8A8_UNORM));
        desc.vertex_bindings = vec![vk::VertexInputBindingDescription {
            binding: 0,
            stride: 16,
            input_rate: vk::VertexInputRate::VERTEX,
        }];
        desc.vertex_attributes = vec![vk::VertexInputAttributeDescription {
            location: 0,
            binding: 0,
            format: vk::Format::R32G32B32A32_SFLOAT,
            offset: 0,
        }];
        // 位域全 0 = TRIANGLE_LIST、cull NONE、depth LESS、写/测试关、无模板、混合模式 0（不混合）。
        RasterPipelineOpts(0).apply_to(&mut desc, 1).unwrap();
        let pipeline = VulkanPipeline::create_raster_pipeline(&cache, desc).unwrap();
        let pool = VulkanCommandPool::new(device.clone()).unwrap();
        Self {
            device,
            gd,
            pipeline,
            pool,
        }
    }

    pub fn new_target(&self, name: &str) -> Target {
        let mut desc =
            TextureDesc::new_2d(SIZE, SIZE, vk::Format::R8G8B8A8_UNORM).color_attachment();
        desc.usage |= vk::ImageUsageFlags::TRANSFER_SRC;
        let texture = self.gd.create_texture(&desc, name, ()).unwrap();
        let view = self
            .gd
            .get_vk_image_view(&texture, &TextureViewDesc::new(&desc).unwrap())
            .unwrap();
        let readback = self
            .gd
            .create_buffer(
                &BufferDesc::new_gpu_to_cpu(
                    u64::from(SIZE * SIZE * 4),
                    vk::BufferUsageFlags::TRANSFER_DST,
                ),
                "readback",
                (),
            )
            .unwrap();
        Target {
            texture,
            view,
            readback,
        }
    }

    /// 录制并提交一帧：布局转换 → 清除 + 绘制三角形 → 拷到读回缓冲。顶点缓冲每次新建，
    /// 由命令缓冲持有到执行完成。返回待完成的提交。
    pub fn draw(&self, target: &Target, triangle: &Triangle, clear: [u8; 4]) -> PendingCommand {
        let gd = &self.gd;
        let raw = &self.device.raw_device();
        let bytes: Vec<u8> = triangle
            .iter()
            .flatten()
            .flat_map(|v| v.to_le_bytes())
            .collect();
        let vbuf = gd
            .create_buffer(
                &BufferDesc::new_cpu_to_gpu(
                    bytes.len() as u64,
                    vk::BufferUsageFlags::VERTEX_BUFFER,
                ),
                "triangle vb",
                bytes.as_slice(),
            )
            .unwrap();
        let mut cb = self.pool.allocate().unwrap();
        cb.begin().unwrap();
        let handle = cb.raw_handle();
        let image = target.texture.raw_handle();
        image_barrier(
            raw,
            handle,
            image,
            (
                vk::ImageLayout::UNDEFINED,
                vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
            ),
            (
                vk::PipelineStageFlags::TOP_OF_PIPE,
                vk::AccessFlags::empty(),
            ),
            (
                vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
            ),
        );
        let extent = vk::Extent2D {
            width: SIZE,
            height: SIZE,
        };
        let clear_value = [vk::ClearValue {
            color: vk::ClearColorValue {
                float32: clear.map(|c| f32::from(c) / 255.0),
            },
        }];
        let colors = [vk::RenderingAttachmentInfo::default()
            .image_view(target.view.raw())
            .image_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .store_op(vk::AttachmentStoreOp::STORE)
            .clear_value(clear_value[0])];
        let info = vk::RenderingInfo::default()
            .render_area(vk::Rect2D {
                offset: vk::Offset2D::default(),
                extent,
            })
            .layer_count(1)
            .color_attachments(&colors);
        let mut backend =
            unsafe { rhapsody::rg::pass_api::AshCommandBackend::new_for_device(&self.device) };
        let mut rendering =
            rhapsody::rg::pass_api::RenderPassApi::new(&mut backend, handle, 1, false);
        rendering.begin_dynamic_rendering(&info).unwrap();
        unsafe {
            raw.cmd_set_viewport(
                handle,
                0,
                &[vk::Viewport {
                    x: 0.0,
                    y: 0.0,
                    width: SIZE as f32,
                    height: SIZE as f32,
                    min_depth: 0.0,
                    max_depth: 1.0,
                }],
            );
            raw.cmd_set_scissor(
                handle,
                0,
                &[vk::Rect2D {
                    offset: vk::Offset2D::default(),
                    extent,
                }],
            );
            raw.cmd_set_stencil_reference(handle, vk::StencilFaceFlags::FRONT_AND_BACK, 0);
            raw.cmd_bind_pipeline(
                handle,
                vk::PipelineBindPoint::GRAPHICS,
                self.pipeline.raw_handle(),
            );
            raw.cmd_bind_vertex_buffers(handle, 0, &[gd.get_vk_buffer(&vbuf).unwrap()], &[0]);
            raw.cmd_draw(handle, 3, 1, 0, 0);
            rendering.end_render_pass();
        }
        image_barrier(
            raw,
            handle,
            image,
            (
                vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
                vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
            ),
            (
                vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
            ),
            (
                vk::PipelineStageFlags::TRANSFER,
                vk::AccessFlags::TRANSFER_READ,
            ),
        );
        let readback = gd.get_vk_buffer(&target.readback).unwrap();
        unsafe {
            raw.cmd_copy_image_to_buffer(
                handle,
                image,
                vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                readback,
                &[vk::BufferImageCopy {
                    buffer_offset: 0,
                    buffer_row_length: 0,
                    buffer_image_height: 0,
                    image_subresource: vk::ImageSubresourceLayers {
                        aspect_mask: vk::ImageAspectFlags::COLOR,
                        mip_level: 0,
                        base_array_layer: 0,
                        layer_count: 1,
                    },
                    image_offset: vk::Offset3D::default(),
                    image_extent: vk::Extent3D {
                        width: SIZE,
                        height: SIZE,
                        depth: 1,
                    },
                }],
            );
            raw.cmd_pipeline_barrier(
                handle,
                vk::PipelineStageFlags::TRANSFER,
                vk::PipelineStageFlags::HOST,
                vk::DependencyFlags::empty(),
                &[],
                &[vk::BufferMemoryBarrier::default()
                    .src_access_mask(vk::AccessFlags::TRANSFER_WRITE)
                    .dst_access_mask(vk::AccessFlags::HOST_READ)
                    .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                    .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                    .buffer(readback)
                    .offset(0)
                    .size(vk::WHOLE_SIZE)],
                &[],
            );
        }
        // 命令缓冲引用的对象必须存活到执行完。
        cb.retain_opaque(target.view.view.clone()).unwrap();
        cb.retain_opaque(Arc::new(target.texture.clone())).unwrap();
        cb.retain_opaque(Arc::new(target.readback.clone())).unwrap();
        cb.retain_object(self.pipeline.clone()).unwrap();
        cb.retain_opaque(Arc::new(vbuf)).unwrap();
        cb.end().unwrap();
        cb.submit().unwrap()
    }
}

/// 读回像素（RGBA8，行优先）。必须在对应提交完成之后调用。
pub fn read_pixels(target: &Target) -> Vec<u8> {
    let mut pixels = vec![0u8; (SIZE * SIZE * 4) as usize];
    target.readback.read_bytes(0, &mut pixels).unwrap();
    pixels
}

pub fn pixel(pixels: &[u8], x: u32, y: u32) -> [u8; 4] {
    let i = ((y * SIZE + x) * 4) as usize;
    pixels[i..i + 4].try_into().unwrap()
}

/// 以像素中心做边函数的 CPU 参考光栅化（不剔除）。返回每像素是否被覆盖。
pub fn reference_coverage(tri: &Triangle) -> Vec<bool> {
    let to_px = |v: &[f32; 4]| {
        (
            (v[0] + 1.0) * 0.5 * SIZE as f32,
            (v[1] + 1.0) * 0.5 * SIZE as f32,
        )
    };
    let [a, b, c] = [to_px(&tri[0]), to_px(&tri[1]), to_px(&tri[2])];
    let edge = |p: (f32, f32), q: (f32, f32), r: (f32, f32)| {
        (q.0 - p.0) * (r.1 - p.1) - (q.1 - p.1) * (r.0 - p.0)
    };
    let area = edge(a, b, c);
    let mut covered = vec![false; (SIZE * SIZE) as usize];
    for y in 0..SIZE {
        for x in 0..SIZE {
            let p = (x as f32 + 0.5, y as f32 + 0.5);
            let (w0, w1, w2) = (edge(b, c, p), edge(c, a, p), edge(a, b, p));
            let inside = if area > 0.0 {
                w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0
            } else {
                w0 <= 0.0 && w1 <= 0.0 && w2 <= 0.0
            };
            covered[(y * SIZE + x) as usize] = inside;
        }
    }
    covered
}

// ---- 动画测试共用：旋转三角形、每帧清除色、逐像素 + 质心校验（triangle_animation_graph 使用） ----

/// 基础三角形绕 NDC 原点旋转 `angle` 弧度。
pub fn rotated_triangle(angle: f32) -> Triangle {
    let base: Triangle = [
        [-0.5, -0.5, 0.0, 1.0],
        [0.5, -0.5, 0.0, 1.0],
        [0.0, 0.5, 0.0, 1.0],
    ];
    let (s, c) = angle.sin_cos();
    base.map(|v| [v[0] * c - v[1] * s, v[0] * s + v[1] * c, 0.0, 1.0])
}

/// 每帧不同的清除色（蓝分量始终 ≥ 155，不会和红色三角形混淆）。
pub fn frame_clear_color(frame: usize) -> [u8; 4] {
    [
        (frame * 5 % 100) as u8,
        (frame * 9 % 200) as u8,
        255 - (frame * 3 % 100) as u8,
        255,
    ]
}

pub struct FrameCheck {
    pub mismatches: usize,
    pub filled: usize,
    pub centroid_error: f32,
}

/// 校验第 `frame` 帧（旋转 `frame * step_deg` 度）的像素：只含填充色/本帧清除色、
/// 与 CPU 参考光栅化的不一致像素数、质心到解析轨迹的距离。断言失败直接 panic。
pub fn check_animation_frame(frame: usize, step_deg: f32, pixels: &[u8]) -> FrameCheck {
    let angle = frame as f32 * step_deg.to_radians();
    let covered = reference_coverage(&rotated_triangle(angle));
    let clear = frame_clear_color(frame);
    let mut mismatches = 0;
    let (mut sum_x, mut sum_y, mut filled) = (0.0f32, 0.0f32, 0usize);
    for y in 0..SIZE {
        for x in 0..SIZE {
            let p = pixel(pixels, x, y);
            let is_fill = p == FILL;
            assert!(
                is_fill || p == clear,
                "第 {frame} 帧 ({x},{y}) 出现既非填充色也非本帧清除色的像素 {p:?}（上一帧内容泄漏？）"
            );
            if is_fill != covered[(y * SIZE + x) as usize] {
                mismatches += 1;
            }
            if is_fill {
                sum_x += x as f32 + 0.5;
                sum_y += y as f32 + 0.5;
                filled += 1;
            }
        }
    }
    assert!(
        mismatches <= 8,
        "第 {frame} 帧与 CPU 参考光栅化有 {mismatches} 个像素不一致"
    );
    assert!(
        (470..=555).contains(&filled),
        "第 {frame} 帧三角形像素数 {filled} 偏离 512（旋转不改面积）"
    );
    let (s, c) = angle.sin_cos();
    let (cx, cy) = (s * (1.0 / 6.0), -c * (1.0 / 6.0));
    let expected = (
        (cx + 1.0) * 0.5 * SIZE as f32,
        (cy + 1.0) * 0.5 * SIZE as f32,
    );
    let measured = (sum_x / filled as f32, sum_y / filled as f32);
    let centroid_error =
        ((measured.0 - expected.0).powi(2) + (measured.1 - expected.1).powi(2)).sqrt();
    assert!(
        centroid_error < 0.8,
        "第 {frame} 帧质心 {measured:?} 偏离预期 {expected:?}（{centroid_error:.2} px）"
    );
    FrameCheck {
        mismatches,
        filled,
        centroid_error,
    }
}
