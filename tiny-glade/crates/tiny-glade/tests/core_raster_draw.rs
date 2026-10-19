//! core 的光栅 pass 链经宿主适配层在 lavapipe 上真画：像素读回 + 多帧。
//!
//! 走的全是 core 的 API：`RasterShaderProgram::new_draw`（经 `ShaderHost` 读发行 .bin 的两个变体）、
//! `raster_single_object` → `RasterPass` → `DrawBuilder::uniform` → `draw_no_vao(3)` → `RasterPass::render`，
//! core 的 `BufferHost`/`TextureHost` 建的纹理/读回缓冲，core 的 `RenderGraph`（`import_unowned`/`export`）。
//! 宿主（`raster_host::RasterPassHost`）把 `RasterPassSubmission` 落到 rhapsody 的渲染通道缓存、帧缓冲、管线、
//! 描述符集和 `vkCmdDraw`，由 `VulkanGraphSession` 执行。
//!
//! 测试脚手架里 rhapsody 直写的只有与游戏逻辑无关的两件事：预清除（带每帧不同的颜色，用来证明 LOAD 附件
//! 保留内容、上一帧不泄漏）和 image→buffer 读回。没有 Vulkan 设备时跳过。
use ash::vk::{self, Handle};
use country_core::render::load_texture::boundary::TextureDevice;
use country_core::{
    render::{
        pass_builder::{
            draw_builder::DrawBuilder,
            draw_stream::{DrawPacket, PassHost, RasterPass, RasterState},
            framebuffer_binding::{
                AttachmentTexture, ColorAttachment, DepthAttachment, DepthClear,
            },
            pass_resource_access::RgTexture,
        },
        raster_single::raster_single_object,
        shader::{
            RasterShaderProgram,
            boundary::{RasterPipelineDesc, ShaderBackend},
        },
        ssbo::rhapsody_boundary::{
            BufferDesc as CoreBufferDesc, GraphicsDevice as CoreGraphicsDevice,
        },
    },
    rhapsody_contract::Handle as CoreHandle,
    rhapsody_types::{RenderGraph as CoreRenderGraph, TextureDesc as CoreTextureDesc},
};
use parking_lot::Mutex;
use rhapsody::{
    raw::device::Device as GraphicsDevice,
    rg::{
        executor::PassContext,
        graph::RenderGraph as RhGraph,
        vulkan::{AccessTable, GpuResource, VulkanGraphBackend, VulkanGraphSession},
    },
    shader_pipeline::{
        compiled::{CompiledShaderPipeline, CompiledShaderStage, ShaderStage},
        pipeline_desc::RasterPipelineDesc as FileRasterDesc,
        reflection::{
            Binding, BindingArray, BindingKind, BindingLocation, BindingNameHash, ShaderReflection,
            UniformBlock,
        },
        shader_defines::ShaderDefines,
    },
};
use std::{cell::RefCell, path::PathBuf, rc::Rc, sync::Arc};
use tiny_glade::{
    raster_host::{GraphDeclarer, RasterShared},
    render_host::{BufferHost, GraphHost, ShaderHost, TextureHost, graph_token},
};

const SIZE: u32 = 64;
const FILL: [u8; 4] = [255, 0, 0, 255];
const TIMEOUT_NS: u64 = 10_000_000_000;
/// 原版访问码（`AccessTable::original()`）。
const TRANSFER_READ: u8 = 0x25;
const HOST_READ: u8 = 0x26;
const TRANSFER_WRITE: u8 = 0x34;
const VK_FORMAT_R8G8B8A8_UNORM: u8 = 37;

// ---------------------------------------------------------------- 着色器（rspirv）

/// 顶点：`gl_Position = u.m * BASE[gl_VertexIndex]`，`u` 是 set 0 binding 0 的 uniform 块（`mat4 m`，偏移 0）。
fn vertex_words(with_color: bool) -> Vec<u32> {
    use rspirv::binary::Assemble;
    use rspirv::dr::{Builder, Operand};
    use rspirv::spirv::{
        AddressingModel, BuiltIn, Capability, Decoration, ExecutionModel, FunctionControl,
        MemoryModel, StorageClass,
    };
    let mut b = Builder::new();
    b.set_version(1, 3);
    b.capability(Capability::Shader);
    b.memory_model(AddressingModel::Logical, MemoryModel::GLSL450);
    let void = b.type_void();
    let fn_ty = b.type_function(void, vec![]);
    let float = b.type_float(32);
    let vec4 = b.type_vector(float, 4);
    let mat4 = b.type_matrix(vec4, 4);
    let int = b.type_int(32, 1);
    let uint = b.type_int(32, 0);

    let block = if with_color {
        b.type_struct([mat4, vec4])
    } else {
        b.type_struct([mat4])
    };
    b.decorate(block, Decoration::Block, []);
    b.member_decorate(block, 0, Decoration::Offset, [Operand::LiteralInt32(0)]);
    b.member_decorate(block, 0, Decoration::ColMajor, []);
    b.member_decorate(
        block,
        0,
        Decoration::MatrixStride,
        [Operand::LiteralInt32(16)],
    );
    if with_color {
        b.member_decorate(block, 1, Decoration::Offset, [Operand::LiteralInt32(64)]);
    }
    let ubo_ptr = b.type_pointer(None, StorageClass::Uniform, block);
    let ubo = b.variable(ubo_ptr, None, StorageClass::Uniform, None);
    b.decorate(ubo, Decoration::DescriptorSet, [Operand::LiteralInt32(0)]);
    b.decorate(ubo, Decoration::Binding, [Operand::LiteralInt32(0)]);
    let mat_ptr = b.type_pointer(None, StorageClass::Uniform, mat4);

    let in_int_ptr = b.type_pointer(None, StorageClass::Input, int);
    let vertex_index = b.variable(in_int_ptr, None, StorageClass::Input, None);
    b.decorate(
        vertex_index,
        Decoration::BuiltIn,
        [Operand::BuiltIn(BuiltIn::VertexIndex)],
    );

    let three = b.constant_u32(uint, 3);
    let arr = b.type_array(vec4, three);
    let c = |b: &mut Builder, v: f32| b.constant_f32(float, v);
    let (n5, p5, z, o) = (
        c(&mut b, -0.5),
        c(&mut b, 0.5),
        c(&mut b, 0.0),
        c(&mut b, 1.0),
    );
    let p0 = b.constant_composite(vec4, [n5, n5, z, o]);
    let p1 = b.constant_composite(vec4, [p5, n5, z, o]);
    let p2 = b.constant_composite(vec4, [z, p5, z, o]);
    let init = b.constant_composite(arr, [p0, p1, p2]);
    let priv_arr_ptr = b.type_pointer(None, StorageClass::Private, arr);
    let positions = b.variable(priv_arr_ptr, None, StorageClass::Private, Some(init));
    let priv_vec_ptr = b.type_pointer(None, StorageClass::Private, vec4);

    let out_ptr = b.type_pointer(None, StorageClass::Output, vec4);
    let gl_position = b.variable(out_ptr, None, StorageClass::Output, None);
    b.decorate(
        gl_position,
        Decoration::BuiltIn,
        [Operand::BuiltIn(BuiltIn::Position)],
    );

    let zero = b.constant_u32(uint, 0);
    let main = b
        .begin_function(void, None, FunctionControl::NONE, fn_ty)
        .unwrap();
    b.begin_block(None).unwrap();
    let idx = b.load(int, None, vertex_index, None, []).unwrap();
    let p = b
        .access_chain(priv_vec_ptr, None, positions, [idx])
        .unwrap();
    let v = b.load(vec4, None, p, None, []).unwrap();
    let mp = b.access_chain(mat_ptr, None, ubo, [zero]).unwrap();
    let m = b.load(mat4, None, mp, None, []).unwrap();
    let r = b.matrix_times_vector(vec4, None, m, v).unwrap();
    b.store(gl_position, r, None, []).unwrap();
    b.ret().unwrap();
    b.end_function().unwrap();
    b.entry_point(
        ExecutionModel::Vertex,
        main,
        "vs_main",
        [vertex_index, gl_position],
    );
    b.module().assemble()
}

/// 片元：`with_color` 时输出 uniform 块里的 `vec4 c`（偏移 64），否则常量红色。
fn fragment_words(with_color: bool) -> Vec<u32> {
    use rspirv::binary::Assemble;
    use rspirv::dr::{Builder, Operand};
    use rspirv::spirv::{
        AddressingModel, Capability, Decoration, ExecutionMode, ExecutionModel, FunctionControl,
        MemoryModel, StorageClass,
    };
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
    let constant = b.constant_composite(vec4, [one, zero, zero, one]);
    let uint = b.type_int(32, 0);
    let (ubo, vec_ptr, one_u) = if with_color {
        let mat4 = b.type_matrix(vec4, 4);
        let block = b.type_struct([mat4, vec4]);
        b.decorate(block, Decoration::Block, []);
        b.member_decorate(block, 0, Decoration::Offset, [Operand::LiteralInt32(0)]);
        b.member_decorate(block, 0, Decoration::ColMajor, []);
        b.member_decorate(
            block,
            0,
            Decoration::MatrixStride,
            [Operand::LiteralInt32(16)],
        );
        b.member_decorate(block, 1, Decoration::Offset, [Operand::LiteralInt32(64)]);
        let ubo_ptr = b.type_pointer(None, StorageClass::Uniform, block);
        let ubo = b.variable(ubo_ptr, None, StorageClass::Uniform, None);
        b.decorate(ubo, Decoration::DescriptorSet, [Operand::LiteralInt32(0)]);
        b.decorate(ubo, Decoration::Binding, [Operand::LiteralInt32(0)]);
        let vec_ptr = b.type_pointer(None, StorageClass::Uniform, vec4);
        (Some(ubo), Some(vec_ptr), Some(b.constant_u32(uint, 1)))
    } else {
        (None, None, None)
    };
    let main = b
        .begin_function(void, None, FunctionControl::NONE, fn_ty)
        .unwrap();
    b.begin_block(None).unwrap();
    let value = match (ubo, vec_ptr, one_u) {
        (Some(ubo), Some(vec_ptr), Some(one_u)) => {
            let p = b.access_chain(vec_ptr, None, ubo, [one_u]).unwrap();
            b.load(vec4, None, p, None, []).unwrap()
        }
        _ => constant,
    };
    b.store(color, value, None, []).unwrap();
    b.ret().unwrap();
    b.end_function().unwrap();
    b.entry_point(ExecutionModel::Fragment, main, "ps_main", [color]);
    b.execution_mode(main, ExecutionMode::OriginUpperLeft, []);
    b.module().assemble()
}

/// Real fragment accesses to two resources that originate outside the graph.
fn external_fragment_words() -> Vec<u32> {
    use rspirv::{
        binary::Assemble,
        dr::{Builder, Operand},
        spirv::*,
    };
    let mut b = Builder::new();
    b.set_version(1, 3);
    b.capability(Capability::Shader);
    b.memory_model(AddressingModel::Logical, MemoryModel::GLSL450);
    let void = b.type_void();
    let fn_ty = b.type_function(void, vec![]);
    let float = b.type_float(32);
    let vec4 = b.type_vector(float, 4);
    let int = b.type_int(32, 1);
    let ivec2 = b.type_vector(int, 2);
    let zero = b.constant_u32(int, 0);
    let coord = b.constant_composite(ivec2, [zero, zero]);
    let block = b.type_struct([vec4]);
    b.decorate(block, Decoration::Block, []);
    b.member_decorate(block, 0, Decoration::Offset, [Operand::LiteralInt32(0)]);
    let block_ptr = b.type_pointer(None, StorageClass::StorageBuffer, block);
    let input = b.variable(block_ptr, None, StorageClass::StorageBuffer, None);
    b.decorate(input, Decoration::DescriptorSet, [Operand::LiteralInt32(0)]);
    b.decorate(input, Decoration::Binding, [Operand::LiteralInt32(1)]);
    let value_ptr = b.type_pointer(None, StorageClass::StorageBuffer, vec4);
    let image_ty = b.type_image(float, Dim::Dim2D, 0, 0, 0, 1, ImageFormat::Unknown, None);
    let image_ptr = b.type_pointer(None, StorageClass::UniformConstant, image_ty);
    let image = b.variable(image_ptr, None, StorageClass::UniformConstant, None);
    b.decorate(image, Decoration::DescriptorSet, [Operand::LiteralInt32(0)]);
    b.decorate(image, Decoration::Binding, [Operand::LiteralInt32(2)]);
    let output_ptr = b.type_pointer(None, StorageClass::Output, vec4);
    let output = b.variable(output_ptr, None, StorageClass::Output, None);
    b.decorate(output, Decoration::Location, [Operand::LiteralInt32(0)]);
    let main = b
        .begin_function(void, None, FunctionControl::NONE, fn_ty)
        .unwrap();
    b.begin_block(None).unwrap();
    let address = b.access_chain(value_ptr, None, input, [zero]).unwrap();
    let color = b.load(vec4, None, address, None, []).unwrap();
    let image = b.load(image_ty, None, image, None, []).unwrap();
    let texel = b
        .image_fetch(
            vec4,
            None,
            image,
            coord,
            Some(ImageOperands::LOD),
            [Operand::IdRef(zero)],
        )
        .unwrap();
    let product = b.f_mul(vec4, None, color, texel).unwrap();
    b.store(output, product, None, []).unwrap();
    b.ret().unwrap();
    b.end_function().unwrap();
    b.entry_point(ExecutionModel::Fragment, main, "ps_main", [output]);
    b.execution_mode(main, ExecutionMode::OriginUpperLeft, []);
    b.module().assemble()
}

fn compiled_external_container() -> CompiledShaderPipeline {
    let mut compiled = compiled_container(false);
    compiled.stages[1].spirv = external_fragment_words();
    for (binding, kind, name) in [
        (1, BindingKind::StorageBuffer, "external_color"),
        (2, BindingKind::SampledImage, "external_texture"),
    ] {
        compiled.reflection.descriptor_sets[0].push((
            binding,
            Binding {
                kind,
                array: BindingArray::Fixed(1),
                name: name.into(),
            },
        ));
        let location = BindingLocation { set: 0, binding };
        compiled
            .reflection
            .binding_names
            .push((name.into(), location));
        compiled
            .reflection
            .binding_name_hashes
            .push((name_hash(name), location));
    }
    compiled
}

fn compiled_container(with_color: bool) -> CompiledShaderPipeline {
    let (size, names, hashes): (u32, Vec<(String, u32)>, Vec<(u64, u32)>) = if with_color {
        (
            80,
            vec![
                ("m".into(), 0),
                ("c_r".into(), 64),
                ("c_g".into(), 68),
                ("c_b".into(), 72),
                ("c_a".into(), 76),
            ],
            vec![
                (m_hash(), 0),
                (name_hash("c_r"), 64),
                (name_hash("c_g"), 68),
                (name_hash("c_b"), 72),
                (name_hash("c_a"), 76),
            ],
        )
    } else {
        (64, vec![("m".into(), 0)], vec![(m_hash(), 0)])
    };
    CompiledShaderPipeline {
        stages: vec![
            CompiledShaderStage {
                stage: ShaderStage::Vertex,
                entry_point: "vs_main".into(),
                spirv: vertex_words(with_color),
            },
            CompiledShaderStage {
                stage: ShaderStage::Fragment,
                entry_point: "ps_main".into(),
                spirv: fragment_words(with_color),
            },
        ],
        source_dependencies: vec![],
        reflection: ShaderReflection {
            descriptor_sets: vec![vec![(
                0,
                Binding {
                    kind: BindingKind::UniformBuffer,
                    array: BindingArray::Fixed(1),
                    name: "u".into(),
                },
            )]],
            uniform_block: Some(UniformBlock {
                byte_size: size,
                name_offsets: names,
                name_hash_offsets: hashes,
                binding: BindingLocation { set: 0, binding: 0 },
            }),
            binding_names: vec![],
            binding_name_hashes: vec![],
            vertex_inputs: vec![],
            local_size: None,
            push_constant_size: 0,
        },
        trailing_bytes: vec![],
    }
}

fn name_hash(name: &str) -> u64 {
    BindingNameHash::new(name).0
}

fn m_hash() -> u64 {
    name_hash("m")
}

// ---------------------------------------------------------------- 参考光栅化（与 rhapsody/tests/draw_support 同一套）

type Triangle = [[f32; 4]; 3];

fn rotated_triangle(angle: f32) -> Triangle {
    let base: Triangle = [
        [-0.5, -0.5, 0.0, 1.0],
        [0.5, -0.5, 0.0, 1.0],
        [0.0, 0.5, 0.0, 1.0],
    ];
    let (s, c) = angle.sin_cos();
    base.map(|v| [v[0] * c - v[1] * s, v[0] * s + v[1] * c, 0.0, 1.0])
}

/// 以像素中心做边函数的 CPU 参考光栅化（不剔除）。
fn reference_coverage(tri: &Triangle) -> Vec<bool> {
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
            covered[(y * SIZE + x) as usize] = if area > 0.0 {
                w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0
            } else {
                w0 <= 0.0 && w1 <= 0.0 && w2 <= 0.0
            };
        }
    }
    covered
}

fn pixel(pixels: &[u8], x: u32, y: u32) -> [u8; 4] {
    let i = ((y * SIZE + x) * 4) as usize;
    pixels[i..i + 4].try_into().unwrap()
}

/// core 的 `uniform("m", Mat4)`：着色器直接用它变换常量三角形。`Flip` 抵消 rg 默认视口的负高度翻转。
fn transform(angle: f32) -> glam::Mat4 {
    glam::Mat4::from_diagonal(glam::Vec4::new(1.0, -1.0, 1.0, 1.0))
        * glam::Mat4::from_rotation_z(angle)
}

struct Check {
    mismatches: usize,
    filled: usize,
    centroid_error: f32,
}

/// 校验一帧：像素只能是填充色或 `clear`、与参考光栅化的不一致数、像素数、质心。
fn check_frame(label: &str, angle: f32, clear: [u8; 4], pixels: &[u8]) -> Check {
    let covered = reference_coverage(&rotated_triangle(angle));
    let (mut mismatches, mut filled) = (0, 0usize);
    let (mut sx, mut sy) = (0.0f32, 0.0f32);
    for y in 0..SIZE {
        for x in 0..SIZE {
            let p = pixel(pixels, x, y);
            assert!(
                p == FILL || p == clear,
                "{label}: ({x},{y}) 既非填充色也非清除色 {clear:?}：{p:?}"
            );
            let is_fill = p == FILL;
            if is_fill != covered[(y * SIZE + x) as usize] {
                mismatches += 1;
            }
            if is_fill {
                sx += x as f32 + 0.5;
                sy += y as f32 + 0.5;
                filled += 1;
            }
        }
    }
    assert!(
        mismatches <= 8,
        "{label}: 与 CPU 参考光栅化有 {mismatches} 个像素不一致"
    );
    assert!(
        (470..=555).contains(&filled),
        "{label}: 三角形像素数 {filled} 偏离理论值 512"
    );
    let (s, c) = angle.sin_cos();
    let expected = (
        (s / 6.0 + 1.0) * 0.5 * SIZE as f32,
        (-c / 6.0 + 1.0) * 0.5 * SIZE as f32,
    );
    let measured = (sx / filled as f32, sy / filled as f32);
    let centroid_error =
        ((measured.0 - expected.0).powi(2) + (measured.1 - expected.1).powi(2)).sqrt();
    assert!(
        centroid_error < 0.8,
        "{label}: 质心 {measured:?} 偏离预期 {expected:?}（{centroid_error:.2} px）"
    );
    Check {
        mismatches,
        filled,
        centroid_error,
    }
}

// ---------------------------------------------------------------- 装置

struct Env {
    gd: Arc<GraphicsDevice>,
    device: Arc<rhapsody::device::Device>,
    buffers: Arc<BufferHost>,
    textures: Arc<TextureHost>,
    shaders: Arc<ShaderHost>,
    raster: Arc<RasterShared>,
    program: CoreHandle<RasterShaderProgram>,
    program_arc: Arc<RasterShaderProgram>,
    /// uniform 块带 `vec4 c`（偏移 64）的版本：片元输出 `c`。
    color_program: CoreHandle<RasterShaderProgram>,
    session: Rc<RefCell<VulkanGraphSession>>,
    root: PathBuf,
}

impl Env {
    fn new(tag: &str) -> Option<Self> {
        let device = match GraphicsDevice::new_headless(&rhapsody::device::DeviceConfig {
            validation: true,
            ..rhapsody::device::DeviceConfig::headless()
        }) {
            Ok(device) => device,
            Err(error) => {
                assert!(
                    !std::env::var("RHAPSODY_REQUIRE_VULKAN").is_ok_and(|value| value == "1"),
                    "strict tiny raster Vulkan initialization failed: {error}"
                );
                eprintln!("SKIP tiny raster Vulkan initialization: {error}");
                return None;
            }
        };
        let gd = device.clone();
        let buffers = Arc::new(BufferHost::new(gd.clone()));
        // core 的 TextureDesc 没有 TRANSFER_SRC 位：读回需要宿主附加（宿主策略，见 `with_extra_usage`）。
        let textures = Arc::new(
            TextureHost::new(gd.clone()).with_extra_usage(vk::ImageUsageFlags::TRANSFER_SRC),
        );
        let root =
            std::env::temp_dir().join(format!("tg-core-raster-{tag}-{}", std::process::id()));
        let shaders = Arc::new(ShaderHost::new(gd.clone(), &root).expect("production shader host"));

        // 发行容器：每个程序两个变体（`SHADER_VARIANT_DEPTH_ONLY` = 0/1）各一份 .bin。
        let make = |name: &'static str, path: &str, with_color: bool| {
            let desc = RasterPipelineDesc {
                path: path.into(),
                entry: "ps_main".into(),
                defines: vec![],
            };
            for variant in shaders.raster_shader_variants(&desc) {
                let file = FileRasterDesc {
                    source_path: variant.path.clone(),
                    vertex_entry: String::new(),
                    fragment_entry: variant.entry.clone(),
                    defines: ShaderDefines(variant.defines.clone()),
                };
                let file_path = root.join(file.compiled_file_path());
                std::fs::create_dir_all(file_path.parent().unwrap()).unwrap();
                std::fs::write(&file_path, compiled_container(with_color).encode().unwrap())
                    .unwrap();
            }
            Arc::new(RasterShaderProgram::new_draw(name, desc, &*shaders))
        };
        let program_arc = make("tri", "test/tri", false);
        let color_arc = make("tri_color", "test/tri_color", true);
        let raster = RasterShared::new(
            gd.clone(),
            buffers.clone(),
            textures.clone(),
            shaders.clone(),
        )
        .unwrap();
        let program = raster.register_program(program_arc.clone());
        let color_program = raster.register_program(color_arc);

        let backend = VulkanGraphBackend::new(device.clone(), AccessTable::original()).unwrap();
        let session = VulkanGraphSession::new(backend);

        Some(Self {
            gd,
            device,
            buffers,
            textures,
            shaders,
            raster,
            program,
            program_arc,
            color_program,
            session: Rc::new(RefCell::new(session)),
            root,
        })
    }

    fn begin(&self) -> Rc<RefCell<RhGraph>> {
        Rc::new(RefCell::new(
            self.session
                .borrow_mut()
                .begin_frame()
                .expect("begin_frame"),
        ))
    }

    fn core_graph(&self, graph: &Rc<RefCell<RhGraph>>) -> CoreRenderGraph {
        CoreRenderGraph::with_backend(GraphHost::new(
            graph.clone(),
            self.buffers.clone(),
            self.textures.clone(),
        ))
    }

    /// core 建的读回缓冲（主机可见）+ 跨帧 token（= core 句柄 id）。
    fn readback_buffer(&self, name: &str) -> country_core::render::ssbo::rhapsody_boundary::Buffer {
        let buffer = self
            .buffers
            .create_buffer(
                &CoreBufferDesc::new(
                    (SIZE * SIZE * 4) as usize,
                    vk::BufferUsageFlags::TRANSFER_DST.as_raw(),
                    2,
                ),
                name,
                None,
            )
            .unwrap();
        let managed = self.buffers.get_by_id(buffer.id).unwrap();
        self.session
            .borrow_mut()
            .backend
            .bind_import_named(
                &format!("imported-{:x}", buffer.id),
                graph_token(buffer.id),
                GpuResource::Buffer(managed.shared()),
            )
            .unwrap();
        buffer
    }

    fn read_pixels(
        &self,
        buffer: &country_core::render::ssbo::rhapsody_boundary::Buffer,
    ) -> Vec<u8> {
        let managed = self.buffers.get_by_id(buffer.id).unwrap();
        let mut pixels = vec![0u8; (SIZE * SIZE * 4) as usize];
        // SAFETY: 调用点已等到对应帧的时间线（`begin_frame` 排空或 `wait_submits_done`）。
        unsafe {
            managed
                .shared()
                .read_bytes_after_completion(0, &mut pixels)
                .unwrap()
        };
        pixels
    }

    /// 脚手架：以 `clear` 清除 `color`（TRANSFER_WRITE）。
    fn add_clear_pass(&self, graph: &Rc<RefCell<RhGraph>>, color: u32, clear: [f32; 4]) {
        let device = self.device.clone();
        let mut graph = graph.borrow_mut();
        let mut pass = graph.add_pass("scaffold clear");
        pass.add_resource_access(pass.resource_handle(color).unwrap(), TRANSFER_WRITE, 0)
            .unwrap();
        pass.render_with_context(move |ctx: PassContext<'_>| {
            let Some(managed) = ctx.managed else {
                return false;
            };
            let Ok(texture) = managed.texture_for(color) else {
                return false;
            };
            let cb = vk::CommandBuffer::from_raw(ctx.command.handle());
            unsafe {
                device.raw_device().cmd_clear_color_image(
                    cb,
                    texture.raw(),
                    vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                    &vk::ClearColorValue { float32: clear },
                    &[vk::ImageSubresourceRange {
                        aspect_mask: vk::ImageAspectFlags::COLOR,
                        base_mip_level: 0,
                        level_count: 1,
                        base_array_layer: 0,
                        layer_count: 1,
                    }],
                );
            }
            true
        });
    }

    /// 脚手架：`color`（TRANSFER_READ）→ `readback`（TRANSFER_WRITE）。
    fn add_readback_pass(&self, graph: &Rc<RefCell<RhGraph>>, color: u32, readback: u32) {
        let device = self.device.clone();
        let mut graph = graph.borrow_mut();
        let mut pass = graph.add_pass("scaffold readback");
        pass.add_resource_access(pass.resource_handle(color).unwrap(), TRANSFER_READ, 0)
            .unwrap();
        pass.add_resource_access(pass.resource_handle(readback).unwrap(), TRANSFER_WRITE, 0)
            .unwrap();
        pass.render_with_context(move |ctx: PassContext<'_>| {
            let Some(managed) = ctx.managed else {
                return false;
            };
            let (Ok(texture), Ok(buffer)) =
                (managed.texture_for(color), managed.buffer_for(readback))
            else {
                return false;
            };
            let cb = vk::CommandBuffer::from_raw(ctx.command.handle());
            unsafe {
                device.raw_device().cmd_copy_image_to_buffer(
                    cb,
                    texture.raw(),
                    vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                    buffer.raw(),
                    &[vk::BufferImageCopy {
                        image_subresource: vk::ImageSubresourceLayers {
                            aspect_mask: vk::ImageAspectFlags::COLOR,
                            mip_level: 0,
                            base_array_layer: 0,
                            layer_count: 1,
                        },
                        image_extent: vk::Extent3D {
                            width: SIZE,
                            height: SIZE,
                            depth: 1,
                        },
                        ..Default::default()
                    }],
                );
            }
            true
        });
    }

    fn finish_graph(graph: Rc<RefCell<RhGraph>>) -> rhapsody::rg::graph::BuiltGraph {
        Rc::try_unwrap(graph)
            .ok()
            .expect("图仍被宿主/核心句柄引用")
            .into_inner()
            .build()
    }
}

impl Drop for Env {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.root).ok();
    }
}

fn color_desc() -> CoreTextureDesc {
    // `new_2d` 现在写 kind = 2（此前恒 0 = 1D，宿主建出 1D 图像）；颜色附件。
    CoreTextureDesc::new_2d([SIZE, SIZE], u32::from(VK_FORMAT_R8G8B8A8_UNORM)).color_attachment()
}

fn attachment_texture(id: u32) -> AttachmentTexture {
    AttachmentTexture {
        id,
        extent: (SIZE, SIZE),
        format: u32::from(VK_FORMAT_R8G8B8A8_UNORM),
        sample_count: 1,
    }
}

// ---------------------------------------------------------------- 测试 1：单帧

#[test]
fn core_raster_pass_draws_a_triangle_into_a_core_texture_and_reads_back() {
    let Some(env) = Env::new("single") else {
        return;
    };

    // core 的纹理 host 建颜色目标；core 的缓冲 host 建读回缓冲。
    let texture = env
        .textures
        .create_texture("color", &color_desc(), &[])
        .unwrap();
    let managed = env.textures.get(&texture).unwrap();
    env.session
        .borrow_mut()
        .backend
        .bind_import_named(
            "color",
            graph_token(texture.id),
            GpuResource::Image(managed.shared()),
        )
        .unwrap();
    let readback = env.readback_buffer("readback");

    let graph = env.begin();
    let mut core_rg = env.core_graph(&graph);
    // 新建纹理：未定义布局，导入访问码 0（无）。
    let color = core_rg.import_unowned(texture.id, 0);
    let rb = core_rg.import_unowned(readback.id, HOST_READ);
    let (color_resource, rb_resource) = (color.resource as u32, rb.resource as u32);

    // core 的链：RasterPass（tag1 = CLEAR 附件）→ DrawBuilder.uniform → draw_no_vao(3) → render。
    let angle = 20f32.to_radians();
    let host = env.raster.pass_host(graph.clone(), None);
    raster_single_object(
        "core triangle",
        &[ColorAttachment::View {
            view: [0; 4],
            texture: attachment_texture(color_resource),
        }],
        env.program.clone(),
        Box::new(host),
        |builder| builder.uniform(m_hash(), transform(angle)),
    );
    env.add_readback_pass(&graph, color_resource, rb_resource);
    core_rg.export(color, TRANSFER_READ);
    core_rg.export(rb, HOST_READ);
    drop(core_rg);

    let built = Env::finish_graph(graph);
    env.shaders
        .prewarm(std::time::Duration::from_secs(10))
        .unwrap();
    env.session
        .borrow_mut()
        .execute_frame(built)
        .expect("execute_frame");
    env.session
        .borrow_mut()
        .wait_submits_done(TIMEOUT_NS)
        .unwrap();
    assert!(!env.session.borrow().is_poisoned());

    let pixels = env.read_pixels(&readback);
    let check = check_frame("单帧", angle, [0, 0, 0, 0], &pixels);

    // 逐点：质心附近在内部、四个角在外部。
    let covered = reference_coverage(&rotated_triangle(angle));
    let inside: Vec<_> = (0..SIZE * SIZE).filter(|i| covered[*i as usize]).collect();
    assert!(!inside.is_empty());
    for (x, y) in [(0, 0), (SIZE - 1, 0), (0, SIZE - 1), (SIZE - 1, SIZE - 1)] {
        assert_eq!(
            pixel(&pixels, x, y),
            [0, 0, 0, 0],
            "角点 ({x},{y}) 应是 CLEAR 的透明零"
        );
    }
    let centroid = (
        ((angle.sin() / 6.0 + 1.0) * 0.5 * SIZE as f32) as u32,
        ((-angle.cos() / 6.0 + 1.0) * 0.5 * SIZE as f32) as u32,
    );
    assert_eq!(
        pixel(&pixels, centroid.0, centroid.1),
        FILL,
        "质心像素应是填充色"
    );

    // 宿主缓存：一个渲染通道、变体 0 一条管线。
    assert_eq!(env.shaders.passes().len(), 1);
    assert_eq!(env.program_arc.variants[0].cached(), 1);
    assert_eq!(env.program_arc.variants[1].cached(), 0, "没用深度变体");
    println!(
        "单帧：三角形像素 {}（理论 512），与参考光栅化不一致 {}，质心误差 {:.3}px",
        check.filled, check.mismatches, check.centroid_error
    );
    env.raster.clear_attachment_cache();
    let _ = (&env.gd, &env.session);
}

#[test]
fn indexed_draw_declares_the_actual_index_bytes_and_outlives_the_host_handle() {
    let Some(env) = Env::new("indexed-declaration") else {
        return;
    };
    let texture = env
        .textures
        .create_texture("indexed color", &color_desc(), &[])
        .unwrap();
    let managed = env.textures.get(&texture).unwrap();
    env.session
        .borrow_mut()
        .backend
        .bind_import_named(
            "indexed color",
            graph_token(texture.id),
            GpuResource::Image(managed.shared()),
        )
        .unwrap();
    let index_bytes: Vec<_> = [0_u32, 1, 2]
        .into_iter()
        .flat_map(u32::to_ne_bytes)
        .collect();
    let index = env
        .buffers
        .create_buffer(
            &CoreBufferDesc::new(12, vk::BufferUsageFlags::INDEX_BUFFER.as_raw(), 2),
            "declared indices",
            Some(&index_bytes),
        )
        .unwrap();
    let readback = env.readback_buffer("indexed readback");
    let graph = env.begin();
    let mut core = env.core_graph(&graph);
    let color = core.import_unowned(texture.id, 0);
    let rb = core.import_unowned(readback.id, HOST_READ);
    let declarer: Rc<dyn GraphDeclarer> = env.session.clone();
    let host = env.raster.pass_host(graph.clone(), Some(declarer));
    let mut pass = RasterPass::new(
        "declared indexed triangle",
        &[ColorAttachment::View {
            view: [0; 4],
            texture: attachment_texture(color.resource as u32),
        }],
        None,
        Box::new(host),
    );
    let packet = DrawPacket::new(env.program.clone(), [0, 0], RasterState(0));
    DrawBuilder::new(&mut pass, packet, 0)
        .uniform(m_hash(), transform(0.0))
        .draw_no_vao_indexed(&country_core::rhapsody_contract::ManagedHandle(index.id), 3);
    pass.render();
    env.add_readback_pass(&graph, color.resource as u32, rb.resource as u32);
    core.export(color, TRANSFER_READ);
    core.export(rb, HOST_READ);
    drop(core);
    let built = Env::finish_graph(graph);
    assert!(
        built
            .declaration
            .passes
            .iter()
            .flat_map(|pass| &pass.accesses)
            .any(|access| {
                access.access_code == 0x03
                    && access.range
                        == rhapsody::rg::resource::AccessRange::Buffer {
                            offset: 0,
                            size: 12,
                        }
            })
    );
    assert!(
        env.buffers.release(&index),
        "the graph owns the physical import after declaration"
    );
    env.shaders
        .prewarm(std::time::Duration::from_secs(10))
        .unwrap();
    env.session.borrow_mut().execute_frame(built).unwrap();
    env.session
        .borrow_mut()
        .wait_submits_done(TIMEOUT_NS)
        .unwrap();
    check_frame(
        "indexed declared range",
        0.0,
        [0; 4],
        &env.read_pixels(&readback),
    );
    assert_eq!(env.session.borrow().frame_import_count(), 0);
    env.raster.clear_attachment_cache();
}

#[test]
fn owned_external_draw_resources_are_declared_before_recording_and_outlive_host_handles() {
    use country_core::{
        render::pass_builder::draw_builder::BindTexture, rhapsody_contract::ManagedHandle,
    };
    use rhapsody::rg::{graph::Descriptor, resource::AccessRange};
    let Some(env) = Env::new("owned-externals") else {
        return;
    };
    let desc = RasterPipelineDesc {
        path: "test/external".into(),
        entry: "ps_main".into(),
        defines: vec![],
    };
    for variant in env.shaders.raster_shader_variants(&desc) {
        let file = FileRasterDesc {
            source_path: variant.path,
            vertex_entry: String::new(),
            fragment_entry: variant.entry,
            defines: ShaderDefines(variant.defines),
        };
        let path = env.root.join(file.compiled_file_path());
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, compiled_external_container().encode().unwrap()).unwrap();
    }
    let program = env
        .raster
        .register_program(Arc::new(RasterShaderProgram::new_draw(
            "owned-external",
            desc,
            &*env.shaders,
        )));
    let color_bytes: Vec<_> = [1.0_f32, 0.0, 0.0, 1.0]
        .into_iter()
        .flat_map(f32::to_ne_bytes)
        .collect();
    let source = env
        .buffers
        .create_buffer(
            &CoreBufferDesc::new(16, vk::BufferUsageFlags::STORAGE_BUFFER.as_raw(), 2),
            "owned fragment color",
            Some(&color_bytes),
        )
        .unwrap();
    let sample = env
        .textures
        .create_texture(
            "owned fragment texture",
            &CoreTextureDesc::new([1, 1, 1], 2, 37).sampled(),
            &[255; 4],
        )
        .unwrap();
    let readback = env.readback_buffer("external readback");
    let graph = env.begin();
    let mut core = env.core_graph(&graph);
    let rb = core.import_unowned(readback.id, HOST_READ);
    let declarer: Rc<dyn GraphDeclarer> = env.session.clone();
    let mut host = env.raster.pass_host(graph.clone(), Some(declarer));
    let target = host
        .rg_resources()
        .unwrap()
        .create_raw_texture("external target", &color_desc());
    let mut pass = RasterPass::new(
        "owned externals",
        &[ColorAttachment::View {
            view: [0; 4],
            texture: attachment_texture(target.resource),
        }],
        None,
        Box::new(host),
    );
    let sample_handle = ManagedHandle(sample.id);
    let sample_desc = AttachmentTexture {
        id: 0,
        extent: (1, 1),
        format: 37,
        sample_count: 1,
    };
    DrawBuilder::new(
        &mut pass,
        DrawPacket::new(program, [0, 0], RasterState(0)),
        0,
    )
    .uniform(m_hash(), transform(0.0))
    .bind_external_storage_buffer(name_hash("external_color"), &ManagedHandle(source.id))
    .bind_texture_to_draw_builder(
        BindTexture::Raw {
            texture: &sample_handle,
            desc: &sample_desc,
        },
        name_hash("external_texture"),
        0x1d,
        None,
    )
    .draw_no_vao(3);
    pass.render();
    env.add_readback_pass(&graph, target.resource, rb.resource as u32);
    core.export(rb, HOST_READ);
    drop(core);
    let built = Env::finish_graph(graph);
    let draw = built
        .declaration
        .passes
        .iter()
        .find(|pass| pass.name.as_ref() == "owned externals")
        .unwrap();
    assert!(draw.accesses.iter().any(|a| a.access_code == 0x37
        && a.range
            == AccessRange::Buffer {
                offset: 0,
                size: 16
            }));
    assert!(draw.accesses.iter().any(|a| a.access_code == 0x1d
        && a.range
            == AccessRange::Texture {
                aspect_mask: vk::ImageAspectFlags::COLOR.as_raw(),
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1,
            }));
    assert_eq!(
        built
            .declaration
            .resources
            .iter()
            .filter(|r| matches!(r.descriptor, Descriptor::Image(_)))
            .count(),
        2
    );
    assert_eq!(env.session.borrow().frame_import_count(), 2);
    assert!(env.buffers.release(&source));
    assert!(env.textures.release(&sample));
    env.shaders
        .prewarm(std::time::Duration::from_secs(10))
        .unwrap();
    env.session.borrow_mut().execute_frame(built).unwrap();
    env.session
        .borrow_mut()
        .wait_submits_done(TIMEOUT_NS)
        .unwrap();
    check_frame(
        "declared external owners",
        0.0,
        [0; 4],
        &env.read_pixels(&readback),
    );
    assert_eq!(env.session.borrow().frame_import_count(), 0);
    env.raster.clear_attachment_cache();
}

// ---------------------------------------------------------------- 测试 2：多帧

const FRAMES: usize = 30;
const STEP_DEG: f32 = 15.0;

// 捕获 `log::warn!`：帧账本异常只告警不 panic。
struct Capture;
static WARNINGS: Mutex<Vec<String>> = Mutex::new(Vec::new());
impl log::Log for Capture {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Warn
    }
    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            WARNINGS
                .lock()
                .push(format!("[{}] {}", record.target(), record.args()));
        }
    }
    fn flush(&self) {}
}
static CAPTURE: Capture = Capture;

/// 每帧不同的清除色（蓝分量 ≥ 155，不会和红色三角形混淆）。
fn frame_clear_color(frame: usize) -> [u8; 4] {
    [
        (frame * 5 % 100) as u8,
        (frame * 9 % 200) as u8,
        255 - (frame * 3 % 100) as u8,
        255,
    ]
}

#[test]
fn core_raster_pass_chain_animates_over_frames_with_two_in_flight() {
    let _ = log::set_logger(&CAPTURE);
    log::set_max_level(log::LevelFilter::Warn);
    let Some(env) = Env::new("anim") else { return };
    let declarer: Rc<dyn GraphDeclarer> = env.session.clone();
    // 两个读回槽（2 帧在飞）。
    let slots = [env.readback_buffer("rb0"), env.readback_buffer("rb1")];

    let (mut max_pending, mut next_verify) = (0usize, 0usize);
    let (mut worst_mismatch, mut worst_centroid, mut checked) = (0usize, 0f32, 0usize);
    let mut previous_pending_before = Vec::new();
    let mut verify = |frame: usize, env: &Env| {
        let pixels = env.read_pixels(&slots[frame % 2]);
        let angle = frame as f32 * STEP_DEG.to_radians();
        let check = check_frame(
            &format!("第 {frame} 帧"),
            angle,
            frame_clear_color(frame),
            &pixels,
        );
        if frame % 6 == 0 {
            println!(
                "第 {frame} 帧（{:.0}°）三角形像素 {}，不一致 {}，质心误差 {:.2}px",
                frame as f32 * STEP_DEG,
                check.filled,
                check.mismatches,
                check.centroid_error
            );
        }
        worst_mismatch = worst_mismatch.max(check.mismatches);
        worst_centroid = worst_centroid.max(check.centroid_error);
        checked += 1;
    };

    for frame in 0..FRAMES {
        let graph = env.begin();
        assert!(
            env.session.borrow().pending_frame_count() <= 1,
            "begin_frame 后在飞帧数必须 ≤ 1"
        );
        previous_pending_before.push(env.session.borrow().pending_frame_count());
        // begin_frame 已把 frame-2 排空：在它的读回槽被本帧覆盖之前校验。
        while next_verify + 2 <= frame {
            verify(next_verify, &env);
            next_verify += 1;
        }

        let mut core_rg = env.core_graph(&graph);
        let rb = core_rg.import_unowned(slots[frame % 2].id, HOST_READ);

        // 颜色目标：core 的 `RgResources::create_raw_texture` 在图内创建（由执行器的资源池分配/复用）。
        let color: RgTexture = {
            let mut host = env.raster.pass_host(graph.clone(), Some(declarer.clone()));
            use country_core::render::pass_builder::draw_stream::PassHost;
            host.rg_resources()
                .expect("host RgResources")
                .create_raw_texture("color", &color_desc())
        };
        let clear = frame_clear_color(frame).map(|c| f32::from(c) / 255.0);
        env.add_clear_pass(&graph, color.resource, clear);
        // core 的链：tag0 = LOAD 附件（访问 0x36 读写）→ 清除色保留、三角形叠上去。
        let angle = frame as f32 * STEP_DEG.to_radians();
        let host = env.raster.pass_host(graph.clone(), None);
        raster_single_object(
            "core triangle",
            &[ColorAttachment::Texture(attachment_texture(color.resource))],
            env.program.clone(),
            Box::new(host),
            |builder| builder.uniform(m_hash(), transform(angle)),
        );
        env.add_readback_pass(&graph, color.resource, rb.resource as u32);
        core_rg.export(rb, HOST_READ);
        drop(core_rg);

        let built = Env::finish_graph(graph);
        env.shaders
            .prewarm(std::time::Duration::from_secs(10))
            .unwrap();
        let receipt = env
            .session
            .borrow_mut()
            .execute_frame(built)
            .expect("execute_frame");
        assert_eq!(receipt.frame, frame as u64 + 1, "帧回执序号单调递增");
        let pending = env.session.borrow().pending_frame_count();
        assert!(pending <= 2, "在飞帧数不得超过 FrameThrottle 容量 2");
        max_pending = max_pending.max(pending);
    }
    env.session
        .borrow_mut()
        .wait_submits_done(TIMEOUT_NS)
        .unwrap();
    assert_eq!(env.session.borrow().pending_frame_count(), 0);
    while next_verify < FRAMES {
        verify(next_verify, &env);
        next_verify += 1;
    }

    assert_eq!(max_pending, 2, "必须真的出现 2 帧同时在飞");
    assert_eq!(checked, FRAMES);
    for (frame, n) in previous_pending_before.iter().enumerate() {
        assert_eq!(
            *n,
            usize::from(frame != 0),
            "第 {frame} 帧 begin_frame 之后的在飞数"
        );
    }
    // 宿主缓存：30 帧共用一个渲染通道、一条管线（不是每帧新建）。
    assert_eq!(env.shaders.passes().len(), 1, "渲染通道按 0x98B 键去重");
    assert_eq!(
        env.program_arc.variants[0].cached(),
        1,
        "管线按 (通道, 状态位域) 缓存"
    );
    assert!(!env.session.borrow().is_poisoned());
    let warnings: Vec<_> = WARNINGS
        .lock()
        .iter()
        .filter(|w| w.contains("账本"))
        .cloned()
        .collect();
    assert!(warnings.is_empty(), "帧账本出现告警：{warnings:?}");
    env.raster.clear_attachment_cache();
    env.session
        .borrow_mut()
        .wait_submits_done(TIMEOUT_NS)
        .unwrap();
    env.device.wait_idle().unwrap();
    println!(
        "共校验 {checked} 帧；最大在飞帧数 {max_pending}；与参考光栅化最多 {worst_mismatch} 个像素不一致；质心最大误差 {worst_centroid:.2} px"
    );
    let _ = &env.gd;
}

// ---------------------------------------------------------------- 测试 3：深度附件 + 多 draw

/// 深度测试 LESS + 深度写（`RasterPipelineOpts` 位域：byte0 bit6 写、bit7 测试、bits4-5 比较 = LESS）。
const DEPTH_TEST_WRITE: RasterState = RasterState(0xC0);
const D32_SFLOAT: u8 = 126;

/// 一个 draw：三角形在参考坐标系里平移 `(dx, dy)`、深度 `z`、颜色 `rgb`。
struct DepthDraw {
    dx: f32,
    z: f32,
    rgb: [f32; 3],
}

fn shifted(dx: f32) -> Triangle {
    rotated_triangle(0.0).map(|v| [v[0] + dx, v[1], v[2], v[3]])
}

fn run_depth_frame(env: &Env, draws: &[DepthDraw]) -> Vec<u8> {
    let declarer: Rc<dyn GraphDeclarer> = env.session.clone();
    let readback = env.readback_buffer("rb");
    let graph = env.begin();
    let mut core_rg = env.core_graph(&graph);
    let rb = core_rg.import_unowned(readback.id, HOST_READ);
    let (color, depth) = {
        let mut host = env.raster.pass_host(graph.clone(), Some(declarer));
        let rg = host.rg_resources().expect("host RgResources");
        let color = rg.create_raw_texture("color", &color_desc());
        let depth = rg.create_raw_texture(
            "depth",
            &CoreTextureDesc::new_2d([SIZE, SIZE], u32::from(D32_SFLOAT))
                .depth_stencil_attachment(),
        );
        (color, depth)
    };
    let depth_attachment = DepthAttachment {
        writable: true,
        layer: 0,
        texture: AttachmentTexture {
            id: depth.resource,
            extent: (SIZE, SIZE),
            format: u32::from(D32_SFLOAT),
            sample_count: 1,
        },
        clear: DepthClear::Clear(1.0),
    };
    let host = env.raster.pass_host(graph.clone(), None);
    let mut pass = RasterPass::new(
        "core depth pass",
        &[ColorAttachment::View {
            view: [0; 4],
            texture: attachment_texture(color.resource),
        }],
        Some(&depth_attachment),
        Box::new(host),
    )
    .with_viewport([SIZE, SIZE]);
    for d in draws {
        let packet = DrawPacket::new(env.color_program.clone(), [0, 0], DEPTH_TEST_WRITE);
        // Flip 抵消视口翻转；平移先于 Flip（参考坐标系里平移），z 最后加。
        let m = glam::Mat4::from_translation(glam::Vec3::new(0.0, 0.0, d.z))
            * glam::Mat4::from_diagonal(glam::Vec4::new(1.0, -1.0, 1.0, 1.0))
            * glam::Mat4::from_translation(glam::Vec3::new(d.dx, 0.0, 0.0));
        DrawBuilder::new(&mut pass, packet, 0)
            .uniform(m_hash(), m)
            .uniform(name_hash("c_r"), d.rgb[0])
            .uniform(name_hash("c_g"), d.rgb[1])
            .uniform(name_hash("c_b"), d.rgb[2])
            .uniform(name_hash("c_a"), 1.0f32)
            .draw_no_vao(3);
    }
    pass.render();
    env.add_readback_pass(&graph, color.resource, rb.resource as u32);
    core_rg.export(rb, HOST_READ);
    drop(core_rg);
    let built = Env::finish_graph(graph);
    env.shaders
        .prewarm(std::time::Duration::from_secs(10))
        .unwrap();
    env.session
        .borrow_mut()
        .execute_frame(built)
        .expect("execute_frame");
    env.session
        .borrow_mut()
        .wait_submits_done(TIMEOUT_NS)
        .unwrap();
    assert!(!env.session.borrow().is_poisoned());
    env.read_pixels(&readback)
}

#[test]
fn core_raster_pass_depth_attachment_resolves_overlap_independent_of_draw_order() {
    let Some(env) = Env::new("depth") else { return };
    let far = DepthDraw {
        dx: -0.2,
        z: 0.8,
        rgb: [0.0, 1.0, 0.0],
    };
    let near = DepthDraw {
        dx: 0.2,
        z: 0.2,
        rgb: [1.0, 0.0, 0.0],
    };
    let (cov_far, cov_near) = (
        reference_coverage(&shifted(far.dx)),
        reference_coverage(&shifted(near.dx)),
    );
    let overlap = cov_far
        .iter()
        .zip(&cov_near)
        .filter(|(a, b)| **a && **b)
        .count();
    assert!(overlap > 100, "测试几何必须有足够的重叠像素：{overlap}");

    for (label, draws) in [("先远后近", [&far, &near]), ("先近后远", [&near, &far])] {
        let draws: Vec<DepthDraw> = draws
            .iter()
            .map(|d| DepthDraw {
                dx: d.dx,
                z: d.z,
                rgb: d.rgb,
            })
            .collect();
        let pixels = run_depth_frame(&env, &draws);
        let mut mismatches = 0;
        let (mut red_in_overlap, mut green, mut red) = (0, 0, 0);
        for y in 0..SIZE {
            for x in 0..SIZE {
                let i = (y * SIZE + x) as usize;
                let expected = if cov_near[i] {
                    [255, 0, 0, 255]
                } else if cov_far[i] {
                    [0, 255, 0, 255]
                } else {
                    [0, 0, 0, 0]
                };
                let p = pixel(&pixels, x, y);
                if p != expected {
                    mismatches += 1;
                }
                if cov_far[i] && cov_near[i] && p == [255, 0, 0, 255] {
                    red_in_overlap += 1;
                }
                green += usize::from(p == [0, 255, 0, 255]);
                red += usize::from(p == [255, 0, 0, 255]);
            }
        }
        println!(
            "{label}：红 {red}、绿 {green}、重叠区红 {red_in_overlap}/{overlap}、与参考不一致 {mismatches}"
        );
        assert!(
            mismatches <= 16,
            "{label}：与参考合成有 {mismatches} 个像素不一致"
        );
        assert!(
            red_in_overlap + 16 >= overlap,
            "{label}：重叠区必须是近处的红色（深度测试），实际 {red_in_overlap}/{overlap}"
        );
    }
    assert_eq!(env.shaders.passes().len(), 1);
}

// ---------------------------------------------------------------- 桥接与边界

#[test]
fn core_buffer_handle_checks_generation_and_reports_real_description() {
    use rhapsody::raw::buffer::MemoryLocation;
    let Some(env) = Env::new("buffer handle") else {
        return;
    };
    let handle = env
        .buffers
        .create_buffer(&CoreBufferDesc::new(256, 0x22, 2), "handle", None)
        .unwrap();
    let desc = env.buffers.description(&handle).expect("live handle");
    assert_eq!(desc.size, handle.byte_size);
    assert_eq!(desc.size, 256);
    assert!(
        desc.usage
            .contains(vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::STORAGE_BUFFER)
    );
    assert_eq!(desc.memory_location, MemoryLocation::CpuToGpu);
    assert!(env.buffers.release(&handle));
    assert!(env.buffers.description(&handle).is_none());
    let replacement = env
        .buffers
        .create_buffer(&CoreBufferDesc::new(128, 0x22, 2), "replacement", None)
        .unwrap();
    assert_eq!(handle.id, replacement.id);
    assert_ne!(handle.generation, replacement.generation);
    assert!(
        env.buffers.description(&handle).is_none(),
        "stale handle must not resolve a reused slot"
    );
    assert_eq!(env.buffers.description(&replacement).unwrap().size, 128);
}

#[test]
fn consumer_resource_errors_keep_device_loss_including_cached_texture_views() {
    let Some(env) = Env::new("resource terminal errors") else {
        return;
    };
    let desc = CoreTextureDesc::new([4, 4, 1], 2, 37).sampled();
    let texture = env
        .textures
        .create_texture("cached terminal view", &desc, &[])
        .unwrap();
    env.textures.default_view(texture.id).unwrap();
    env.device.timelines().mark_device_lost();
    assert!(matches!(
        env.textures.default_view(texture.id),
        Err(rhapsody::Error::DeviceLost)
    ));
    assert!(matches!(
        env.textures.create_texture("lost texture", &desc, &[]),
        Err(rhapsody::Error::DeviceLost)
    ));
    assert!(matches!(
        env.buffers
            .create_buffer(&CoreBufferDesc::new(64, 0x22, 2), "lost buffer", None),
        Err(rhapsody::Error::DeviceLost)
    ));
}

#[test]
fn render_pass_desc_maps_attachment_tags_to_load_ops_and_depth_access() {
    use country_core::render::pass_builder::draw_stream::{
        AttachmentDesc, DEPTH_STATE_NONE, RasterRenderPassDesc,
    };
    use rhapsody::rg::attachments::contract::DepthAccess;
    use tiny_glade::raster_host::rh_render_pass_desc;
    let mut colors = [AttachmentDesc::default(); 8];
    for (i, tag) in [0u32, 1, 2].into_iter().enumerate() {
        colors[i] = AttachmentDesc::new(37).with_sample_count(1);
        colors[i].kind_tag = tag;
    }
    let desc = RasterRenderPassDesc {
        colors,
        color_count: 3,
        depth: AttachmentDesc::new(126)
            .with_sample_count(1)
            .with_clear_input(),
        depth_state: 1,
    };
    let rh = rh_render_pass_desc(&desc).unwrap();
    // tag0 LOAD、tag1 CLEAR、tag2 DONT_CARE：`+4` 被覆盖为 tag，`vkCreateRenderPass` 原样当 loadOp。
    let loads: Vec<u32> = rh.colors.as_slice().iter().map(|a| a.clear_input).collect();
    assert_eq!(loads, [0, 1, 2]);
    assert!(
        rh.colors
            .as_slice()
            .iter()
            .all(|a| a.format == vk::Format::R8G8B8A8_UNORM && a.store_op == 0)
    );
    let (depth, access) = rh.depth.expect("depth");
    assert_eq!(
        (depth.format.as_raw(), depth.clear_input, access),
        (126, 1, DepthAccess::ReadWrite)
    );
    // 只读深度；无深度。
    assert_eq!(
        rh_render_pass_desc(&RasterRenderPassDesc {
            depth_state: 0,
            ..desc
        })
        .unwrap()
        .depth
        .unwrap()
        .1,
        DepthAccess::ReadOnly
    );
    assert!(
        rh_render_pass_desc(&RasterRenderPassDesc {
            depth_state: DEPTH_STATE_NONE,
            ..desc
        })
        .unwrap()
        .depth
        .is_none()
    );
    assert_eq!(
        rh_render_pass_desc(&RasterRenderPassDesc {
            color_count: 9,
            ..desc
        })
        .unwrap_err()
        .kind(),
        rhapsody::ErrorKind::InvalidUsage
    );
    assert_eq!(
        rh_render_pass_desc(&RasterRenderPassDesc {
            depth_state: u8::MAX,
            ..desc
        })
        .unwrap_err()
        .kind(),
        rhapsody::ErrorKind::InvalidUsage
    );
    // 键：不同 tag → 不同通道；相同描述 → 相同键（缓存去重）。
    let mut other = desc;
    other.colors[0].kind_tag = 1;
    assert_ne!(
        rh_render_pass_desc(&desc).unwrap(),
        rh_render_pass_desc(&other).unwrap()
    );
    assert_eq!(
        rh_render_pass_desc(&desc).unwrap(),
        rh_render_pass_desc(&desc).unwrap()
    );
}

#[test]
fn mesh_draw_without_an_actual_vao_is_rejected_without_submission() {
    let Some(env) = Env::new("reject") else {
        return;
    };
    let graph = env.begin();
    let host = env.raster.pass_host(graph.clone(), None);
    let mut pass = RasterPass::new(
        "unverified",
        &[ColorAttachment::Texture(attachment_texture(0))],
        None,
        Box::new(host),
    );
    let packet = DrawPacket::new(env.program.clone(), [0, 0], RasterState(0));
    DrawBuilder::new(&mut pass, packet, 0).draw_instanced_w_gl_draw_flag(1);
    pass.render();
    let error = env
        .session
        .borrow_mut()
        .execute_frame(Env::finish_graph(graph))
        .err()
        .expect("missing VAO must fail through the frame transaction");
    assert_eq!(error.kind(), rhapsody::ErrorKind::InvalidUsage);
    assert_eq!(env.session.borrow().pending_frame_count(), 0);
}

#[test]
fn host_temporal_resources_use_session_owned_named_slots() {
    let Some(env) = Env::new("temporal") else {
        return;
    };
    let declarer: Rc<dyn GraphDeclarer> = env.session.clone();
    let graph = env.begin();
    let mut host = env.raster.pass_host(graph, Some(declarer));
    let rg = host.rg_resources().unwrap();
    let slot = rg
        .get_or_create_temporal_texture("h", &color_desc())
        .unwrap();
    assert_eq!(slot.flag, 0);
    let pair = rg
        .get_or_create_temporal_texture_pair("pair", &color_desc())
        .unwrap();
    assert!(!pair.fresh);
    assert_ne!(pair.current.resource, pair.previous.resource);
    assert_ne!(slot.texture.resource, pair.current.resource);
    assert!(
        rg.get_or_create_temporal_texture("h", &color_desc())
            .is_err()
    );
    assert_eq!(env.session.borrow().named_resource_count(), 3);
}

/// The native bit is sent to `history_valid`, so the production adapter must
/// never advertise newly allocated/unsubmitted images as usable history. The
/// current session additionally invalidates resized/cancelled allocations.
#[test]
fn host_temporal_validity_tracks_submitted_history_resize_and_cancellation() {
    let Some(env) = Env::new("temporal-validity") else {
        return;
    };
    for step in 0..5 {
        let resized = step >= 2;
        let mut desc = color_desc().sampled().storage();
        if resized {
            desc.extent[0] /= 2;
        }
        let buffer_desc = CoreBufferDesc::new_gpu_only(if resized { 32 } else { 16 }, 0x22);
        let graph = env.begin();
        let (slot, pair, buffer) = {
            let declarer: Rc<dyn GraphDeclarer> = env.session.clone();
            let mut host = env.raster.pass_host(graph.clone(), Some(declarer));
            let rg = host.rg_resources().unwrap();
            (
                rg.get_or_create_temporal_texture("single", &desc).unwrap(),
                rg.get_or_create_temporal_texture_pair("pair", &desc)
                    .unwrap(),
                rg.get_or_create_temporal_buffer("counter", &buffer_desc)
                    .unwrap(),
            )
        };
        let valid = matches!(step, 1 | 4);
        assert_eq!(slot.flag, u8::from(valid), "single, step {step}");
        assert_eq!(pair.fresh, valid, "pair history, step {step}");
        assert_eq!(buffer.flag, u8::from(valid), "buffer, step {step}");
        // Both allocation and graph declaration are real. Step 2 deliberately
        // abandons resized resources before any GPU command can publish them.
        if step == 2 {
            env.session
                .borrow_mut()
                .cancel_frame(Env::finish_graph(graph))
                .unwrap();
            continue;
        }
        for texture in [slot.texture, pair.current] {
            let host = env.raster.pass_host(graph.clone(), None);
            let mut attachment = attachment_texture(texture.resource);
            attachment.extent = (desc.extent[0], desc.extent[1]);
            raster_single_object(
                "initialize temporal image",
                &[ColorAttachment::View {
                    view: [0; 4],
                    texture: attachment,
                }],
                env.program.clone(),
                Box::new(host),
                |builder| builder.uniform(m_hash(), transform(0.0)),
            );
        }
        {
            let device = env.device.clone();
            let mut raw = graph.borrow_mut();
            let mut pass = raw.add_pass("initialize temporal counter");
            pass.add_resource_access(
                pass.resource_handle(buffer.buffer.resource).unwrap(),
                TRANSFER_WRITE,
                0,
            )
            .unwrap();
            pass.render_with_result_context(move |ctx| {
                let managed = ctx.managed.ok_or_else(|| {
                    rhapsody::Error::invalid_usage("missing temporal buffer owner")
                })?;
                let target = managed.buffer_for(buffer.buffer.resource)?;
                unsafe {
                    device.raw_device().cmd_fill_buffer(
                        vk::CommandBuffer::from_raw(ctx.command.handle()),
                        target.raw(),
                        0,
                        vk::WHOLE_SIZE,
                        7,
                    );
                }
                Ok(())
            });
        }
        env.shaders
            .prewarm(std::time::Duration::from_secs(10))
            .unwrap();
        env.session
            .borrow_mut()
            .execute_frame(Env::finish_graph(graph))
            .unwrap();
        env.session
            .borrow_mut()
            .wait_submits_done(TIMEOUT_NS)
            .unwrap();
    }
}
