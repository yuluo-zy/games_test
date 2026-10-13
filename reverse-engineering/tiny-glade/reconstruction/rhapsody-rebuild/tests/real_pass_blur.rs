//! 真实游戏 pass 端到端：原作 `blur.cs.hlsl` 编译出的 SPIR-V（`tools/render-passes/self_test/blur_mod0.spv`，
//! 取自发行 `_blur.cs.hlsl_cs_main_0.bin` 的第 0 个模块，5160 字节）经 rhapsody 的 rg 图执行：
//! 上传 → 原 shader dispatch → 回读，结果与按 `reconstruction/shader-source/shaders/blur.cs.hlsl` 写的 CPU 参考比对。
//!
//! 这是本工程第一条使用原作 shader 的图内 pass（此前的基准全是人造回调）。
//! 无可用 Vulkan 设备时跳过；本机用 `VK_ICD_FILENAMES=.../lvp_icd.json` 走 lavapipe。
use ash::vk::{self, Handle};
use rhapsody::{
    raw::{
        buffer::BufferDesc,
        descriptor_set::DescriptorSetState,
        init::create_headless_device,
        managed::{buffer::ManagedBuffer, texture::ManagedTexture},
        shader::{ReflectedEntry, ReflectedShader, ShaderLibrary, ShaderModule},
        texture::{TextureDesc, TextureViewDesc},
    },
    rg::{
        binding::bind_descriptor_set,
        executor::PassContext,
        graph::{RegistryResource, RenderGraph},
        resource::ResourceHandle,
        vulkan::{AccessSpec, AccessTable, ResourceSpec, VulkanGraphBackend},
    },
};
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

const SIZE: u32 = 64;
/// 半径 7 像素的采样 + 取整余量；只比较离边界更远的像素，避免依赖越界 `OpImageFetch` 的未定义结果。
const MARGIN: u32 = 9;

fn blur_spirv() -> Vec<u8> {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tools/render-passes/self_test/blur_mod0.spv");
    std::fs::read(path).expect("缺少原 blur SPIR-V 模块")
}
fn buffer(context: &PassContext<'_>, resource: ResourceHandle) -> ManagedBuffer {
    match context.managed.unwrap().registry.resource(resource.raw()).0 {
        RegistryResource::OwnedBuffer(v) | RegistryResource::UnownedBuffer(v) => v.handle.clone(),
        _ => panic!("图资源类型不符"),
    }
}
fn image(context: &PassContext<'_>, resource: ResourceHandle) -> ManagedTexture {
    match context.managed.unwrap().registry.resource(resource.raw()).0 {
        RegistryResource::OwnedTexture(v) | RegistryResource::UnownedTexture(v) => v.handle.clone(),
        _ => panic!("图资源类型不符"),
    }
}
fn access_table() -> AccessTable {
    let mut accesses = AccessTable::default();
    for (code, write, stages, access, layout) in [
        (
            1,
            true,
            vk::PipelineStageFlags::TRANSFER,
            vk::AccessFlags::TRANSFER_WRITE,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
        ),
        (
            2,
            false,
            vk::PipelineStageFlags::TRANSFER,
            vk::AccessFlags::TRANSFER_READ,
            vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
        ),
        (
            4,
            false,
            vk::PipelineStageFlags::COMPUTE_SHADER,
            vk::AccessFlags::SHADER_READ | vk::AccessFlags::UNIFORM_READ,
            vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
        ),
        (
            40,
            true,
            vk::PipelineStageFlags::COMPUTE_SHADER,
            vk::AccessFlags::SHADER_WRITE,
            vk::ImageLayout::GENERAL,
        ),
    ] {
        accesses.0.insert(
            code,
            AccessSpec {
                original_mask: 0x40,
                write,
                stages,
                access,
                layout,
            },
        );
    }
    accesses
}
/// 输入图像：只用 R 通道，确定性的非平凡图案。
fn input_pixel(x: u32, y: u32) -> f32 {
    ((x * 7 + y * 13) % 17) as f32 / 16.0 + (x % 5) as f32 * 0.01
}
/// 与 `blur.cs.hlsl` 逐式对应（f32 运算顺序一致），但循环次数由调用方给定。
///
/// HLSL 的两层循环以浮点累加器做条件（`angle < 2π` 步长 0.2094395、`t <= 1.0` 步长 0.1）。严格 IEEE f32 下
/// 外层 31 次、内层 9 次（`t` 第 10 次累加得 1.0000001 > 1.0）；但 GPU/驱动编译器可以改写这类循环，
/// 实测 lavapipe 的内层是 10 次。所以不把次数写死，而是在候选里找 GPU 实际采用的那组。
fn reference(x: u32, y: u32, outer: u32, inner: u32) -> f32 {
    let dims = SIZE as f32;
    let (u, v) = (x as f32 / dims, y as f32 / dims);
    let mut acc = input_pixel(x, y);
    let mut angle = 0.0f32;
    for _ in 0..outer {
        let mut sum = acc;
        let mut t = 0.1f32;
        for _ in 0..inner {
            let su = u + angle.cos() * (7.0 / dims) * t;
            let sv = v + angle.sin() * (7.0 / dims) * t;
            let (sx, sy) = ((su * dims) as i32, (sv * dims) as i32);
            sum += input_pixel(sx as u32, sy as u32);
            t += 0.1f32;
        }
        acc = sum;
        angle += 0.209_439_52f32;
    }
    acc * 0.003_508_772f32
}

#[test]
fn original_blur_shader_runs_through_the_render_graph_and_matches_the_hlsl_reference() {
    let device = match create_headless_device() {
        Ok(device) => device,
        Err(error) => {
            eprintln!("跳过：无可用 Vulkan 设备（{error:?}）");
            return;
        }
    };
    let mut backend = VulkanGraphBackend::new(device.clone(), access_table()).unwrap();
    let mut graph = RenderGraph::new();
    let pixels = (SIZE * SIZE) as u64;
    let globals = backend
        .declare(
            &mut graph,
            "globals",
            ResourceSpec::Buffer(BufferDesc::new_cpu_to_gpu(16, vk::BufferUsageFlags::UNIFORM_BUFFER)),
        )
        .unwrap();
    let staging = backend
        .declare(
            &mut graph,
            "staging",
            ResourceSpec::Buffer(BufferDesc::new_cpu_to_gpu(pixels * 16, vk::BufferUsageFlags::TRANSFER_SRC)),
        )
        .unwrap();
    let img_in_desc = TextureDesc::new_2d(SIZE, SIZE, vk::Format::R32G32B32A32_SFLOAT)
        .usage(vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::TRANSFER_DST);
    let img_out_desc = TextureDesc::new_2d(SIZE, SIZE, vk::Format::R32G32B32A32_SFLOAT)
        .usage(vk::ImageUsageFlags::STORAGE | vk::ImageUsageFlags::TRANSFER_SRC);
    let img_in = backend
        .declare(&mut graph, "img_in", ResourceSpec::Image(img_in_desc))
        .unwrap();
    let img_out = backend
        .declare(&mut graph, "img_out", ResourceSpec::Image(img_out_desc))
        .unwrap();
    let readback = backend
        .declare(
            &mut graph,
            "readback",
            ResourceSpec::Buffer(BufferDesc::new_gpu_to_cpu(pixels * 16, vk::BufferUsageFlags::TRANSFER_DST)),
        )
        .unwrap();

    let subresource = vk::ImageSubresourceLayers::default()
        .aspect_mask(vk::ImageAspectFlags::COLOR)
        .layer_count(1);
    let region = vk::BufferImageCopy::default()
        .image_subresource(subresource)
        .image_extent(vk::Extent3D {
            width: SIZE,
            height: SIZE,
            depth: 1,
        });

    // Pass 1：写入 $Globals 与输入图像。
    let mut upload = graph.add_pass("upload");
    upload
        .add_resource_access(globals, 1, 0)
        .unwrap()
        .add_resource_access(staging, 2, 0)
        .unwrap()
        .add_resource_access(img_in, 1, 0)
        .unwrap();
    let upload_device = device.clone();
    upload.render_with_context(move |context| {
        let mut dims = Vec::new();
        for value in [SIZE, SIZE, SIZE, SIZE] {
            dims.extend(value.to_le_bytes());
        }
        buffer(&context, globals).write_bytes(0, &dims).unwrap();
        let mut texels = Vec::new();
        for y in 0..SIZE {
            for x in 0..SIZE {
                for value in [input_pixel(x, y), 0.0, 0.0, 1.0f32] {
                    texels.extend(value.to_le_bytes());
                }
            }
        }
        let staging = buffer(&context, staging);
        staging.write_bytes(0, &texels).unwrap();
        let texture = image(&context, img_in);
        unsafe {
            upload_device.device.cmd_copy_buffer_to_image(
                vk::CommandBuffer::from_raw(context.command.handle()),
                staging.handle,
                texture.handle,
                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                &[region],
            );
        }
        context.keep_alive(Arc::new(texture));
        context.keep_alive(Arc::new(staging));
        true
    });

    // Pass 2：原 blur shader。描述符名与原 SPIR-V 的 OpName 一致：$Globals(b13)、img_in(b14)、img_out(b15)。
    let entry: ReflectedEntry = serde_json::from_value(serde_json::json!({
        "entry_point": "cs_main", "stage": "GLCompute",
        "descriptors": [
            {"set":0,"binding":13,"name":"$Globals","count":1,"descriptor_type":"UniformBuffer"},
            {"set":0,"binding":14,"name":"img_in","count":1,"descriptor_type":"SampledImage"},
            {"set":0,"binding":15,"name":"img_out","count":1,"descriptor_type":"StorageImage(ReadWrite)"}]
    }))
    .unwrap();
    let mut library = ShaderLibrary::new(device.clone()).unwrap();
    library
        .register(
            "blur".into(),
            ReflectedShader {
                module: ShaderModule::from_spirv(device.clone(), &blur_spirv()).unwrap(),
                entry: entry.clone(),
            },
        )
        .unwrap();
    let pipeline = library.create_compute_pipeline("blur", &BTreeMap::new()).unwrap();
    let state = Arc::new(DescriptorSetState::new(pipeline.layout.sets[0].clone(), 2).unwrap());
    let mut pass = graph.add_pass("blur");
    let g = pass
        .buffer_ref(globals, 4, 0)
        .unwrap()
        .bind_uniform_buffer(0, 16)
        .unwrap();
    let i = pass
        .texture_ref(img_in, 4, 0)
        .unwrap()
        .into_sampled_rg_texture(
            TextureViewDesc::new(&img_in_desc),
            vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
        )
        .unwrap();
    let o = pass
        .texture_ref(img_out, 40, 0)
        .unwrap()
        .bind_view(TextureViewDesc::new(&img_out_desc), vk::ImageLayout::GENERAL, true)
        .unwrap();
    pass.render_bound_with_context(
        BTreeMap::from([
            ("$Globals".into(), vec![g]),
            ("img_in".into(), vec![i]),
            ("img_out".into(), vec![o]),
        ]),
        move |context, plan| {
            let writes = context
                .managed
                .unwrap()
                .descriptor_writes(plan, &entry, 0, 0, context.command_state())
                .unwrap();
            let set = unsafe { state.allocate_descriptor_set(&writes) }.unwrap();
            let raw = vk::CommandBuffer::from_raw(context.command.handle());
            unsafe {
                pipeline.bind_pipeline(raw);
                bind_descriptor_set(raw, &pipeline, 0, &set, &[]).unwrap();
                pipeline.device.device.cmd_dispatch(raw, SIZE / 8, SIZE / 8, 1);
            }
            context.keep_alive(set);
            context.keep_alive(pipeline);
            true
        },
    );

    // Pass 3：把输出图像拷回宿主可读缓冲。
    let mut download = graph.add_pass("readback");
    download
        .add_resource_access(img_out, 2, 0)
        .unwrap()
        .add_resource_access(readback, 1, 0)
        .unwrap();
    let download_device = device.clone();
    let captured = Arc::new(Mutex::new(None));
    let sink = captured.clone();
    download.render_with_context(move |context| {
        let readback = buffer(&context, readback);
        let texture = image(&context, img_out);
        unsafe {
            download_device.device.cmd_copy_image_to_buffer(
                vk::CommandBuffer::from_raw(context.command.handle()),
                texture.handle,
                vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                readback.handle,
                &[region],
            );
        }
        context.keep_alive(Arc::new(texture));
        *sink.lock().unwrap() = Some(readback);
        true
    });

    let started = std::time::Instant::now();
    let mut run = backend.execute(graph.build()).unwrap();
    let recorded = started.elapsed();
    run.wait(u64::MAX).unwrap();
    // 仅供参考：lavapipe（CPU 软件渲染）上的绝对耗时不能外推真显卡。
    eprintln!("blur: 编译+录制+提交 {recorded:?}，GPU 等待 {:?}", started.elapsed() - recorded);
    let mut bytes = vec![0u8; (pixels * 16) as usize];
    captured
        .lock()
        .unwrap()
        .as_ref()
        .unwrap()
        .read_bytes(0, &mut bytes)
        .unwrap();
    let texel = |x: u32, y: u32| {
        let at = ((y * SIZE + x) * 16) as usize;
        let channel = |c: usize| f32::from_le_bytes(bytes[at + c * 4..at + c * 4 + 4].try_into().unwrap());
        [channel(0), channel(1), channel(2), channel(3)]
    };
    // 整幅图都必须被 dispatch 覆盖：shader 对每个像素都写 alpha=1（边缘像素的 rgb 依赖越界读取，不比较）。
    for y in 0..SIZE {
        for x in 0..SIZE {
            assert_eq!(texel(x, y)[3], 1.0, "像素 ({x},{y}) 没有被 shader 写入");
        }
    }
    let mut texels = Vec::new();
    for y in MARGIN..SIZE - MARGIN {
        for x in MARGIN..SIZE - MARGIN {
            let got = texel(x, y);
            assert_eq!(got[3], 1.0, "alpha 必须为 1 @({x},{y})");
            assert_eq!(got[0], got[1], "rgb 三通道应相同 @({x},{y})");
            assert_eq!(got[0], got[2], "rgb 三通道应相同 @({x},{y})");
            texels.push((x, y, got[0]));
        }
    }
    // 候选循环次数：IEEE 严格值与驱动可能改写后的值。每个候选记录相对误差的升序分布。
    let mut ranked: Vec<(f32, f32, u32, u32)> = Vec::new();
    for (outer, inner) in [(30u32, 9u32), (30, 10), (31, 9), (31, 10)] {
        let mut errors: Vec<f32> = texels
            .iter()
            .map(|&(x, y, got)| {
                let want = reference(x, y, outer, inner);
                (got - want).abs() / want.abs().max(1e-6)
            })
            .collect();
        errors.sort_by(f32::total_cmp);
        let (median, worst) = (errors[errors.len() / 2], errors[errors.len() - 1]);
        eprintln!("blur: 候选 outer={outer} inner={inner} 中位误差 {median:.2e} 最大误差 {worst:.2e}");
        ranked.push((median, worst, outer, inner));
    }
    ranked.sort_by(|a, b| a.0.total_cmp(&b.0));
    let (median, worst, outer, inner) = ranked[0];
    eprintln!(
        "blur: 比较 {} 个像素；GPU 采用 outer={outer} inner={inner}，中位误差 {median:.2e}，最大误差 {worst:.2e}",
        texels.len()
    );
    // 正确的循环次数下，绝大多数像素与参考位级接近（仅 sin/cos 实现差异使个别采样点落到相邻纹素）；
    // 错误的次数会让所有像素系统性偏差几个百分点，所以用"中位误差远小于次优候选"来判别。
    assert!(median < 1e-4, "中位误差过大：{median}");
    assert!(worst < 1e-2, "最大误差过大：{worst}");
    assert!(ranked[1].0 > median * 50.0, "最优候选没有显著优于次优候选，无法确认 GPU 的循环次数");
}
