//! 六帧真实纹理双缓冲：原状态合同驱动构图，清除、跨帧读取、尺寸替换及回收。
//! 每帧后端重建是验证适配，不宣称原持久设备和调度策略已对齐。
use ash::vk::{self, Handle};
use rhapsody::{
    raw::{
        buffer::{Buffer, BufferDesc},
        init::create_headless_device,
        texture::TextureDesc,
    },
    rg::{
        graph::{Descriptor, RenderGraph},
        state::{DetachedCompileState, PersistentCompileState},
        vulkan::{
            AccessSpec, AccessTable, GpuResource, ResourceSpec, TemporalGpuResources,
            VulkanGraphBackend,
        },
    },
};
use std::sync::Arc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let device = create_headless_device()?;
    let props = unsafe {
        device
            .instance
            .instance
            .get_physical_device_properties(device.physical)
    };
    let name = unsafe { std::ffi::CStr::from_ptr(props.device_name.as_ptr()) }
        .to_string_lossy()
        .into_owned();
    let image_usage = vk::ImageUsageFlags::TRANSFER_SRC | vk::ImageUsageFlags::TRANSFER_DST;
    let buffer_usage = vk::BufferUsageFlags::TRANSFER_SRC | vk::BufferUsageFlags::TRANSFER_DST;
    let mut table = AccessTable::default();
    for (code, write, stages, access) in [
        (
            40,
            true,
            vk::PipelineStageFlags::TRANSFER,
            vk::AccessFlags::TRANSFER_WRITE,
        ),
        (
            4,
            false,
            vk::PipelineStageFlags::TRANSFER,
            vk::AccessFlags::TRANSFER_READ,
        ),
        (
            55,
            true,
            vk::PipelineStageFlags::ALL_COMMANDS,
            vk::AccessFlags::MEMORY_READ | vk::AccessFlags::MEMORY_WRITE,
        ),
        (
            3,
            false,
            vk::PipelineStageFlags::HOST,
            vk::AccessFlags::HOST_READ,
        ),
    ] {
        table.0.insert(
            code,
            AccessSpec {
                original_mask: if write { 0x1000 } else { 0x800 },
                write,
                stages,
                access,
                layout: vk::ImageLayout::GENERAL,
            },
        );
    }
    let mut persistent = PersistentCompileState {
        graph: None,
        temporal: Some(Default::default()),
    };
    let mut owner = TemporalGpuResources::new(device.clone());
    let mut weak = vec![];
    let mut frames = vec![];
    for frame in 0..6u32 {
        let width = if frame < 3 { 4 } else { 8 };
        let spec = ResourceSpec::Image(
            TextureDesc::new_2d(width, 4, vk::Format::R32_UINT).usage(image_usage),
        );
        let descriptor = owner.register("history:0", spec);
        owner.register("history:1", spec);
        persistent.graph = Some(RenderGraph::new());
        let detached = persistent
            .take_for_compile()
            .map_err(|_| std::io::Error::other("跨帧状态取出失败"))?;
        let mut graph = detached.graph;
        let mut state = detached.temporal;
        let pair = state
            .get_or_create_owned_pair(&mut graph, "history", descriptor, &mut owner)
            .map_err(|e| std::io::Error::other(format!("{e:?}")))?;
        let Some(GpuResource::Image(before)) = owner.resource(pair.first.resource.token) else {
            return Err("前帧纹理丢失".into());
        };
        let Some(GpuResource::Image(current)) = owner.resource(pair.second.resource.token) else {
            return Err("当前纹理丢失".into());
        };
        if !pair.both_reused {
            weak.push(Arc::downgrade(&before));
            weak.push(Arc::downgrade(&current));
        }
        let byte_count = width as u64 * 4 * 4;
        let previous_readback = Arc::new(Buffer::new(
            device.clone(),
            &BufferDesc::new_gpu_to_cpu(byte_count, buffer_usage),
        )?);
        let current_readback = Arc::new(Buffer::new(
            device.clone(),
            &BufferDesc::new_gpu_to_cpu(byte_count, buffer_usage),
        )?);
        let mut backend = VulkanGraphBackend::new(device.clone(), table.clone())?;
        // 缓存拥有者与提交后端分别持有 Arc；token 由真实创建者产生，沿图物理映射传递。
        let (first_name, second_name) = if pair.parity == 0 {
            ("history:0", "history:1")
        } else {
            ("history:1", "history:0")
        };
        owner
            .import_into(&mut backend, first_name, pair.first.resource.token)
            .map_err(|e| std::io::Error::other(format!("{e:?}")))?;
        owner
            .import_into(&mut backend, second_name, pair.second.resource.token)
            .map_err(|e| std::io::Error::other(format!("{e:?}")))?;
        let previous_token = backend.bind_import(GpuResource::Buffer(previous_readback.clone()));
        let current_token = backend.bind_import(GpuResource::Buffer(current_readback.clone()));
        let buffer_desc = Descriptor::Buffer(rhapsody::rg::graph::BufferDesc {
            word_at_00: byte_count,
            word_at_08: buffer_usage.as_raw(),
            bytes_at_0c_through_0e: [0; 3],
        });
        let previous_output = graph
            .import_owned("previous-output", buffer_desc, previous_token, 0)
            .unwrap();
        let current_output = graph
            .import_owned("current-output", buffer_desc, current_token, 0)
            .unwrap();
        let range = vk::ImageSubresourceRange::default()
            .aspect_mask(vk::ImageAspectFlags::COLOR)
            .level_count(1)
            .layer_count(1);
        if !pair.both_reused {
            let mut pass = graph.add_pass("initialize-history");
            pass.add_resource_access(pair.first.handle, 40, 0).unwrap();
            let d = device.clone();
            let image = before.handle;
            pass.render(move |handle| {
                unsafe {
                    d.device.cmd_clear_color_image(
                        vk::CommandBuffer::from_raw(handle),
                        image,
                        vk::ImageLayout::GENERAL,
                        &vk::ClearColorValue {
                            uint32: [77, 0, 0, 0],
                        },
                        &[range],
                    );
                }
                true
            });
        }
        let mut pass = graph.add_pass("write-current");
        pass.add_resource_access(pair.second.handle, 40, 0).unwrap();
        let d = device.clone();
        let image = current.handle;
        pass.render(move |handle| {
            unsafe {
                d.device.cmd_clear_color_image(
                    vk::CommandBuffer::from_raw(handle),
                    image,
                    vk::ImageLayout::GENERAL,
                    &vk::ClearColorValue {
                        uint32: [frame + 1, 0, 0, 0],
                    },
                    &[range],
                );
            }
            true
        });
        let mut pass = graph.add_pass("read-both");
        for input in [pair.first.handle, pair.second.handle] {
            pass.add_resource_access(input, 4, 0).unwrap();
        }
        for output in [previous_output, current_output] {
            pass.add_resource_access(output, 40, 0).unwrap();
        }
        let d = device.clone();
        let previous_image = before.handle;
        let current_image = current.handle;
        let previous_buffer = previous_readback.handle;
        let current_buffer = current_readback.handle;
        pass.render(move |handle| {
            let region = vk::BufferImageCopy::default()
                .image_subresource(
                    vk::ImageSubresourceLayers::default()
                        .aspect_mask(vk::ImageAspectFlags::COLOR)
                        .layer_count(1),
                )
                .image_extent(vk::Extent3D {
                    width,
                    height: 4,
                    depth: 1,
                });
            unsafe {
                d.device.cmd_copy_image_to_buffer(
                    vk::CommandBuffer::from_raw(handle),
                    previous_image,
                    vk::ImageLayout::GENERAL,
                    previous_buffer,
                    &[region],
                );
                d.device.cmd_copy_image_to_buffer(
                    vk::CommandBuffer::from_raw(handle),
                    current_image,
                    vk::ImageLayout::GENERAL,
                    current_buffer,
                    &[region],
                );
            }
            true
        });
        // 从此只有缓存/后端持有物理 Arc，回调保存的只是合法 Vulkan handle。
        drop(before);
        drop(current);
        graph.export(previous_output, 3).unwrap();
        graph.export(current_output, 3).unwrap();
        // 已恢复原 take/export/restore 顺序，population 随后由真实 GPU 后端执行。
        let built = DetachedCompileState {
            graph,
            temporal: state,
        }
        .export_and_restore_owned(&mut persistent, &mut owner)
        .unwrap();
        let mut run = backend.execute(built).map_err(std::io::Error::other)?;
        persistent
            .temporal
            .as_ref()
            .unwrap()
            .validate_compiled_exports(&run.execution.compiled)
            .unwrap();
        if frame == 5 {
            // CPU 清缓存可先于等待完成；提交后端必须保留资源直到回执被等待和持有者释放。
            persistent
                .temporal
                .as_mut()
                .unwrap()
                .clear_owned(&mut owner);
            assert!(owner.is_empty());
            assert!(weak[2..].iter().all(|r| r.upgrade().is_some()));
        }
        run.wait(10_000_000_000).map_err(std::io::Error::other)?;
        let mut previous_bytes = vec![0; byte_count as usize];
        let mut current_bytes = vec![0; byte_count as usize];
        previous_readback.read_bytes(0, &mut previous_bytes)?;
        current_readback.read_bytes(0, &mut current_bytes)?;
        let expected_previous = if pair.both_reused { frame } else { 77 };
        let previous_ok = previous_bytes
            .chunks_exact(4)
            .all(|v| v == expected_previous.to_le_bytes());
        let current_ok = current_bytes
            .chunks_exact(4)
            .all(|v| v == (frame + 1).to_le_bytes());
        if !previous_ok || !current_ok {
            return Err(format!("第 {frame} 帧读回不一致").into());
        }
        frames.push(serde_json::json!({"frame":frame,"width":width,"parity":pair.parity,"both_reused":pair.both_reused,"previous":expected_previous,"current":frame+1,"tokens":[pair.first.resource.token,pair.second.resource.token],"export_count":run.execution.compiled.snapshot.exports.len(),"readback_passed":true}));
    }
    let creations = weak.len();
    persistent
        .temporal
        .as_mut()
        .unwrap()
        .clear_owned(&mut owner);
    let reclaimed = weak.iter().filter(|weak| weak.upgrade().is_none()).count();
    assert_eq!(creations, 4);
    assert_eq!(reclaimed, creations);
    assert!(owner.is_empty());
    println!(
        "{}",
        serde_json::json!({"status":"passed","real_gpu":true,"device":name,"frames":frames,"physical_creations":creations,"cache_resources_after_clear":owner.len(),"reclaimed_after_completion":reclaimed,"cache_clear_before_completion_wait":true,"backend_arc_preserves_pending_resources":true,"limits":["显式候选Vulkan访问映射","每帧后端重建","不证明原作完整游戏调度或画面一致"]})
    );
    Ok(())
}
