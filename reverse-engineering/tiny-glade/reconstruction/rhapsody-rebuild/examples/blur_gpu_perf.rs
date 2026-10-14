//! 原作 blur SPIR-V 经现有 rg 图执行；独立 GPU 时间戳性能入口。
//! 沿用 real_pass_blur 正确性测试的图资源、descriptor 和回读链。
use ash::vk::{self, Handle};
use rhapsody::{
    raw::{
        buffer::BufferDesc,
        descriptor_set::DescriptorSetState,
        device::VulkanDevice,
        init::create_headless_device,
        managed::{buffer::ManagedBuffer, texture::ManagedTexture},
        profiler::GpuProfiler,
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
fn reference(size: u32, x: u32, y: u32, outer: u32, inner: u32) -> f32 {
    let dims = size as f32;
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

fn sample(device: &Arc<VulkanDevice>, size: u32, profiler: Arc<GpuProfiler>) -> serde_json::Value {
    let mut backend = VulkanGraphBackend::new(device.clone(), access_table()).unwrap();
    let mut graph = RenderGraph::new();
    let pixels = (size * size) as u64;
    let globals = backend
        .declare(
            &mut graph,
            "globals",
            ResourceSpec::Buffer(BufferDesc::new_cpu_to_gpu(
                16,
                vk::BufferUsageFlags::UNIFORM_BUFFER,
            )),
        )
        .unwrap();
    let staging = backend
        .declare(
            &mut graph,
            "staging",
            ResourceSpec::Buffer(BufferDesc::new_cpu_to_gpu(
                pixels * 16,
                vk::BufferUsageFlags::TRANSFER_SRC,
            )),
        )
        .unwrap();
    let img_in_desc = TextureDesc::new_2d(size, size, vk::Format::R32G32B32A32_SFLOAT)
        .usage(vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::TRANSFER_DST);
    let img_out_desc = TextureDesc::new_2d(size, size, vk::Format::R32G32B32A32_SFLOAT)
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
            ResourceSpec::Buffer(BufferDesc::new_gpu_to_cpu(
                pixels * 16,
                vk::BufferUsageFlags::TRANSFER_DST,
            )),
        )
        .unwrap();

    let subresource = vk::ImageSubresourceLayers::default()
        .aspect_mask(vk::ImageAspectFlags::COLOR)
        .layer_count(1);
    let region = vk::BufferImageCopy::default()
        .image_subresource(subresource)
        .image_extent(vk::Extent3D {
            width: size,
            height: size,
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
        for value in [size, size, size, size] {
            dims.extend(value.to_le_bytes());
        }
        buffer(&context, globals).write_bytes(0, &dims).unwrap();
        let mut texels = Vec::new();
        for y in 0..size {
            for x in 0..size {
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
    let pipeline = library
        .create_compute_pipeline("blur", &BTreeMap::new())
        .unwrap();
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
        .bind_view(
            TextureViewDesc::new(&img_out_desc),
            vk::ImageLayout::GENERAL,
            true,
        )
        .unwrap();
    let timer = profiler.clone();
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
                timer.reset(raw);
                let span = timer.begin(raw, "original_blur_dispatch").unwrap();
                pipeline.bind_pipeline(raw);
                bind_descriptor_set(raw, &pipeline, 0, &set, &[]).unwrap();
                pipeline
                    .device
                    .device
                    .cmd_dispatch(raw, size / 8, size / 8, 1);
                timer.end(raw, span);
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
    let total_ms = started.elapsed().as_secs_f64() * 1000.0;
    let gpu_ms = profiler.results().unwrap()[0].milliseconds;
    let mut bytes = vec![0u8; (pixels * 16) as usize];
    captured
        .lock()
        .unwrap()
        .as_ref()
        .unwrap()
        .read_bytes(0, &mut bytes)
        .unwrap();
    let texel = |x: u32, y: u32| {
        let at = ((y * size + x) * 16) as usize;
        let channel =
            |c: usize| f32::from_le_bytes(bytes[at + c * 4..at + c * 4 + 4].try_into().unwrap());
        [channel(0), channel(1), channel(2), channel(3)]
    };
    // 整幅图都必须被 dispatch 覆盖：shader 对每个像素都写 alpha=1（边缘像素的 rgb 依赖越界读取，不比较）。
    for y in 0..size {
        for x in 0..size {
            assert_eq!(texel(x, y)[3], 1.0, "像素 ({x},{y}) 没有被 shader 写入");
        }
    }
    // 只取内部 8×8 网格与 CPU 参考比较；所有像素的 alpha 仍在上面检查。
    let mut ranked = Vec::new();
    for (outer, inner) in [(30u32, 9u32), (30, 10), (31, 9), (31, 10)] {
        let mut errors = Vec::new();
        for j in 0..8 {
            for i in 0..8 {
                let x = MARGIN + i * (size - 2 * MARGIN - 1) / 7;
                let y = MARGIN + j * (size - 2 * MARGIN - 1) / 7;
                let got = texel(x, y);
                assert_eq!(got[0], got[1]);
                assert_eq!(got[0], got[2]);
                let want = reference(size, x, y, outer, inner);
                errors.push((got[0] - want).abs() / want.abs().max(1e-6));
            }
        }
        errors.sort_by(f32::total_cmp);
        ranked.push((
            errors[errors.len() / 2],
            errors[errors.len() - 1],
            outer,
            inner,
        ));
    }
    ranked.sort_by(|a, b| a.0.total_cmp(&b.0));
    let (median_error, max_error, outer, inner) = ranked[0];
    assert!(
        median_error < 1e-4,
        "blur median readback mismatch: {ranked:?}"
    );
    assert!(
        ranked[1].0 > median_error * 50.0,
        "ambiguous loop count: {ranked:?}"
    );
    serde_json::json!({
        "gpu_dispatch_ms": gpu_ms,
        "graph_record_submit_wait_ms": total_ms,
        "cpu_record_submit_ms": recorded.as_secs_f64()*1000.0,
        "alpha_pixels_verified": pixels,
        "reference_pixels": 64,
        "reference_median_error": median_error,
        "reference_max_error": max_error,
        "reference_max_error_under_one_percent": max_error < 1e-2,
        "outer": outer, "inner": inner,
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let device = create_headless_device()?;
    let properties = unsafe {
        device
            .instance
            .instance
            .get_physical_device_properties(device.physical)
    };
    let name =
        unsafe { std::ffi::CStr::from_ptr(properties.device_name.as_ptr()) }.to_string_lossy();
    assert!(
        matches!(
            properties.device_type,
            vk::PhysicalDeviceType::INTEGRATED_GPU | vk::PhysicalDeviceType::DISCRETE_GPU
        ),
        "requires physical GPU: {name}"
    );
    let queues = unsafe {
        device
            .instance
            .instance
            .get_physical_device_queue_family_properties(device.physical)
    };
    let valid_bits = queues[device.queue_family as usize].timestamp_valid_bits;
    let profiler = Arc::new(GpuProfiler::new(
        device.clone(),
        1,
        properties.limits.timestamp_period as f64,
        valid_bits,
    )?);
    let mut results = Vec::new();
    for size in [64u32, 256, 512, 1024] {
        for _ in 0..2 {
            sample(&device, size, profiler.clone());
        }
        let samples: Vec<_> = (0..15)
            .map(|_| sample(&device, size, profiler.clone()))
            .collect();
        let mut times: Vec<_> = samples
            .iter()
            .map(|s| s["gpu_dispatch_ms"].as_f64().unwrap())
            .collect();
        times.sort_by(f64::total_cmp);
        let median = times[times.len() / 2];
        eprintln!(
            "original blur {size}x{size}: GPU median {median:.4} ms, max {:.4} ms",
            times[times.len() - 1]
        );
        results.push(serde_json::json!({"size":size, "warmup":2, "samples":samples, "gpu_median_ms":median, "gpu_p95_ms":times[times.len()-1], "megapixels_per_second":(size as f64 * size as f64)/median/1000.0}));
    }
    println!(
        "{}",
        serde_json::json!({
            "schema":1,"device":name,"device_type":format!("{:?}",properties.device_type),
            "vendor_id":properties.vendor_id,"device_id":properties.device_id,"driver_version":properties.driver_version,
            "api_version":properties.api_version,"timestamp_period_ns":properties.limits.timestamp_period,
            "timestamp_valid_bits":valid_bits,"enabled_layers":device.instance.layer_names(),
        "method":"TOP_OF_PIPE/BOTTOM_OF_PIPE Vulkan timestamp interval around pipeline bind, descriptor bind and original blur dispatch; upload/readback commands outside bracket, but dependency waiting may contribute; CPU reference excluded; fresh graph/resources/pipeline per sample; not a steady-state game frame",
            "results":results,"original_full_game_comparison":false
        })
    );
    Ok(())
}
