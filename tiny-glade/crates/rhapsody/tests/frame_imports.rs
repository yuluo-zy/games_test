//! Actual frame imports are scoped declarations, including cancelled graphs.
mod gpu_test_support;
use ash::vk::{self, Handle};
use rhapsody::{device::{Device, DeviceConfig}, rg::{frame_imports::FrameImportConfig, resource::AccessRange, vulkan::{AccessTable, GpuResource, VulkanGraphBackend, VulkanGraphSession}}};
use rhapsody::resource::buffer::{Buffer, BufferDesc};
use std::sync::Arc;
const TIMEOUT: u64 = 10_000_000_000;

#[test]
fn recorded_imports_release_admission_while_gpu_commands_keep_the_resources_alive() {
    exercise_import_lifetime(false);
}

#[test]
fn exported_imports_remain_addressable_for_post_submit_consumers_until_retirement() {
    exercise_import_lifetime(true);
}

fn exercise_import_lifetime(export_target: bool) {
    let Some(device) = gpu_test_support::initialize(
        "completed import admission",
        Device::new_headless(&DeviceConfig::headless()),
    ) else {
        return;
    };
    let source = Arc::new(
        Buffer::new(
            device.clone(),
            &BufferDesc::new_cpu_to_gpu(16, vk::BufferUsageFlags::TRANSFER_SRC),
        )
        .unwrap(),
    );
    let target = Arc::new(
        Buffer::new(
            device.clone(),
            &BufferDesc::new_gpu_to_cpu(16, vk::BufferUsageFlags::TRANSFER_DST),
        )
        .unwrap(),
    );
    source.write_bytes(0, &[73; 16]).unwrap();
    let mut session = VulkanGraphSession::new(
        VulkanGraphBackend::new(device.clone(), AccessTable::original()).unwrap(),
    );
    session
        .configure_frame_imports(FrameImportConfig {
            max_imports: 2 + usize::from(export_target),
        })
        .unwrap();
    let mut graph = session.begin_frame().unwrap();
    let input = session
        .import_frame_buffer(&mut graph, source.clone(), 0x25)
        .unwrap();
    let output = session
        .import_frame_buffer(&mut graph, target.clone(), 0x26)
        .unwrap();
    let mut pass = graph
        .add_pass("real copy before import admission")
        .with_side_effect();
    pass.add_resource_access(input, 0x25, 0).unwrap();
    pass.add_resource_access(output, 0x34, 0).unwrap();
    let raw = device.clone();
    pass.render_with_result_context(move |ctx| {
        let resources = ctx.managed.unwrap();
        unsafe {
            raw.raw_device().cmd_copy_buffer(
                vk::CommandBuffer::from_raw(ctx.command.handle()),
                resources.buffer_for(input.raw()).unwrap().raw_handle(),
                resources.buffer_for(output.raw()).unwrap().raw_handle(),
                &[vk::BufferCopy {
                    src_offset: 0,
                    dst_offset: 0,
                    size: 16,
                }],
            );
        }
        Ok(())
    });
    if export_target {
        graph.export(output, 0x26).unwrap();
    }
    struct Unblock(Arc<rhapsody::device::semaphore::TimelineSemaphore>);
    impl Drop for Unblock {
        fn drop(&mut self) {
            unsafe {
                self.0
                    .device()
                    .raw_device()
                    .signal_semaphore(
                        &vk::SemaphoreSignalInfo::default()
                            .semaphore(self.0.raw_handle())
                            .value(1),
                    )
                    .unwrap();
            }
        }
    }
    let gate = rhapsody::device::semaphore::TimelineSemaphore::new(device.clone(), 0).unwrap();
    let unblock = Unblock(gate.clone());
    session
        .backend
        .set_graph_synchronization(
            vec![(gate, 1, vk::PipelineStageFlags::ALL_COMMANDS)],
            vec![],
        )
        .unwrap();
    let receipt = session.execute_frame(graph.build()).unwrap();
    assert_eq!(session.pending_frame_count(), 1);
    assert_eq!(session.frame_import_count(), usize::from(export_target));
    let output_token = receipt
        .snapshot
        .physical_mapping
        .iter()
        .find(|mapping| mapping.logical == output.raw())
        .unwrap()
        .token
        .unwrap();
    match session.backend.resource_clone(output_token) {
        Some(GpuResource::Buffer(actual)) if export_target => {
            assert!(Arc::ptr_eq(&target, &actual))
        }
        None if !export_target => {}
        _ => panic!("only exported bindings survive recording"),
    }
    let source_owner = Arc::downgrade(&source);
    let mut next = session.begin_frame().unwrap();
    session
        .import_frame_buffer(&mut next, source, 0x25)
        .expect("recorded declarations must not consume the next graph's capacity");
    session
        .import_frame_buffer(&mut next, target.clone(), 0x26)
        .unwrap();
    let extra = Arc::new(
        Buffer::new(
            device.clone(),
            &BufferDesc::new_cpu_to_gpu(16, vk::BufferUsageFlags::TRANSFER_SRC),
        )
        .unwrap(),
    );
    assert!(matches!(
        session.import_frame_buffer(&mut next, extra, 0x25),
        Err(rhapsody::Error::Exhausted(_))
    ));
    assert_eq!(session.frame_import_count(), 2 + usize::from(export_target));
    session.cancel_frame(next.build()).unwrap();
    assert_eq!(session.frame_import_count(), usize::from(export_target));
    assert_eq!(session.pending_frame_count(), 1);
    assert!(
        source_owner.upgrade().is_some(),
        "in-flight commands own the input"
    );
    assert_eq!(session.collect_completed_frames().unwrap(), 0);
    drop(unblock);
    session.wait_frame(receipt.frame, TIMEOUT).unwrap();
    assert_eq!(session.pending_frame_count(), 0);
    assert_eq!(session.frame_import_count(), 0);
    assert!(session.backend.resource_clone(output_token).is_none());
    let mut actual = [0; 16];
    target.read_bytes(0, &mut actual).unwrap();
    assert_eq!(actual, [73; 16]);
}

#[test]
fn frame_imports_reuse_existing_physical_declarations_and_release_cancelled_graphs() {
    let Some(device) = gpu_test_support::initialize(
        "frame import declaration lifetime",
        Device::new_headless(&DeviceConfig::headless()),
    ) else {
        return;
    };
    let buffer = Arc::new(
        Buffer::new(
            device.clone(),
            &BufferDesc::new_cpu_to_gpu(16, vk::BufferUsageFlags::TRANSFER_SRC),
        )
        .unwrap(),
    );
    let backend = VulkanGraphBackend::new(device, AccessTable::original()).unwrap();
    let mut session = VulkanGraphSession::new(backend);
    session
        .configure_frame_imports(FrameImportConfig { max_imports: 1 })
        .unwrap();
    let baseline = session.resource_count();
    let mut graph = session.begin_frame().unwrap();
    let first = session
        .import_frame_buffer(&mut graph, buffer.clone(), 0x25)
        .unwrap();
    assert!(matches!(
        session.import_frame_buffer(&mut graph, buffer.clone(), 0x04),
        Err(rhapsody::Error::InvalidUsage { .. })
    ));
    assert_eq!(
        session
            .import_frame_buffer(&mut graph, buffer.clone(), 0x25)
            .unwrap(),
        first
    );
    assert_eq!(graph.declaration().resources.len(), 1);
    assert_eq!(session.frame_import_count(), 1);
    assert_eq!(session.resource_count(), baseline + 1);
    drop(graph);
    drop(session.begin_frame().unwrap());
    assert_eq!(session.frame_import_count(), 0);
    assert_eq!(session.resource_count(), baseline);

    let token = session
        .backend
        .bind_import(GpuResource::Buffer(buffer.clone()))
        .unwrap();
    let mut graph = session.begin_frame().unwrap();
    let manual = graph
        .import_unowned(
            "manual host buffer",
            GpuResource::Buffer(buffer.clone()).descriptor(),
            token,
            0x25,
        )
        .unwrap();
    assert_eq!(
        session
            .import_frame_buffer(&mut graph, buffer, 0x25)
            .unwrap(),
        manual
    );
    assert_eq!(graph.declaration().resources.len(), 1);
    assert_eq!(session.frame_import_count(), 0);
    drop(graph);
    session.release_import(token).unwrap();
}

#[test]
fn declared_frame_sources_copy_real_bytes_and_failed_recording_does_not_leak_bindings() {
    let Some(device) = gpu_test_support::initialize(
        "declared frame upload sources",
        Device::new_headless(&DeviceConfig::headless()),
    ) else {
        return;
    };
    let source = Arc::new(
        Buffer::new(
            device.clone(),
            &BufferDesc::new_cpu_to_gpu(64, vk::BufferUsageFlags::TRANSFER_SRC),
        )
        .unwrap(),
    );
    let output = Arc::new(
        Buffer::new(
            device.clone(),
            &BufferDesc::new_gpu_to_cpu(16, vk::BufferUsageFlags::TRANSFER_DST),
        )
        .unwrap(),
    );
    let backend = VulkanGraphBackend::new(device.clone(), AccessTable::original()).unwrap();
    let mut session = VulkanGraphSession::new(backend);
    for frame in 0..4 {
        let expected = [frame as u8 + 17; 16];
        source.write_bytes(16, &expected).unwrap();
        let mut graph = session.begin_frame().unwrap();
        let input = session
            .import_frame_buffer(&mut graph, source.clone(), 0x25)
            .unwrap();
        let target = session
            .import_frame_buffer(&mut graph, output.clone(), 0x26)
            .unwrap();
        let mut pass = graph.add_pass("declared physical upload");
        pass.add_resource_access_range(
            input,
            0x25,
            0,
            AccessRange::Buffer {
                offset: 16,
                size: 16,
            },
        )
        .unwrap();
        pass.add_resource_access(target, 0x34, 0).unwrap();
        let raw_device = device.clone();
        pass.render_with_result_context(move |ctx| {
            let managed = ctx.managed.unwrap();
            let input = managed.buffer_for(input.raw()).unwrap();
            let output = managed.buffer_for(target.raw()).unwrap();
            unsafe {
                raw_device.raw_device().cmd_copy_buffer(
                    vk::CommandBuffer::from_raw(ctx.command.handle()),
                    input.raw_handle(),
                    output.raw_handle(),
                    &[vk::BufferCopy {
                        src_offset: 16,
                        dst_offset: 0,
                        size: 16,
                    }],
                );
            }
            Ok(())
        });
        graph.export(target, 0x26).unwrap();
        let receipt = session.execute_frame(graph.build()).unwrap();
        session.wait_frame(receipt.frame, TIMEOUT).unwrap();
        assert_eq!(session.frame_import_count(), 0);
        let mut actual = [0; 16];
        output.read_bytes(0, &mut actual).unwrap();
        assert_eq!(actual, expected);
    }
    let mut graph = session.begin_frame().unwrap();
    let input = session
        .import_frame_buffer(&mut graph, source.clone(), 0x25)
        .unwrap();
    let mut pass = graph.add_pass("failed producer");
    pass.add_resource_access(input, 0x25, 0).unwrap();
    pass.with_side_effect()
        .render_with_result_context(|_| Err(rhapsody::Error::OutOfHostMemory));
    assert!(matches!(
        session.execute_frame(graph.build()),
        Err(rhapsody::Error::OutOfHostMemory)
    ));
    assert_eq!(session.frame_import_count(), 0);
    session.wait_submits_done(TIMEOUT).unwrap();

    let mut graph = session.begin_frame().unwrap();
    graph.batch_per_pass(true);
    let input = session
        .import_frame_buffer(&mut graph, source, 0x25)
        .unwrap();
    let target = session
        .import_frame_buffer(&mut graph, output.clone(), 0x26)
        .unwrap();
    let mut pass = graph.add_pass("submitted upload prefix");
    pass.add_resource_access_range(
        input,
        0x25,
        0,
        AccessRange::Buffer {
            offset: 16,
            size: 16,
        },
    )
    .unwrap();
    pass.add_resource_access(target, 0x34, 0).unwrap();
    let raw_device = device.clone();
    pass.render_with_result_context(move |ctx| {
        let managed = ctx.managed.unwrap();
        unsafe {
            raw_device.raw_device().cmd_copy_buffer(
                vk::CommandBuffer::from_raw(ctx.command.handle()),
                managed.buffer_for(input.raw()).unwrap().raw_handle(),
                managed.buffer_for(target.raw()).unwrap().raw_handle(),
                &[vk::BufferCopy {
                    src_offset: 16,
                    dst_offset: 0,
                    size: 16,
                }],
            );
        }
        Ok(())
    });
    let mut fail = graph.add_pass("failed upload suffix").with_side_effect();
    fail.add_resource_access(target, 0x26, 0).unwrap();
    fail.render_with_result_context(|_| Err(rhapsody::Error::OutOfHostMemory));
    assert!(matches!(
        session.execute_frame(graph.build()),
        Err(rhapsody::Error::OutOfHostMemory)
    ));
    assert_eq!(session.pending_frame_count(), 1);
    assert_eq!(session.frame_import_count(), 0);
    session.wait_submits_done(TIMEOUT).unwrap();
    assert_eq!(session.frame_import_count(), 0);
    assert_eq!(session.resource_count(), 0);
    let mut actual = [0; 16];
    output.read_bytes(0, &mut actual).unwrap();
    assert_eq!(
        actual, [20; 16],
        "submitted prefix keeps its physical owners"
    );
}

#[test]
fn frame_texture_import_deduplicates_and_releases_cancelled_declarations() {
    use rhapsody::resource::texture::{Texture, TextureDesc};
    let Some(device) = gpu_test_support::initialize(
        "frame texture import lifetime",
        Device::new_headless(&DeviceConfig::headless()),
    ) else {
        return;
    };
    let texture = Arc::new(
        Texture::new(
            device.clone(),
            &TextureDesc::new_2d(1, 1, vk::Format::R8G8B8A8_UNORM)
                .usage(vk::ImageUsageFlags::SAMPLED),
        )
        .unwrap(),
    );
    let mut session =
        VulkanGraphSession::new(VulkanGraphBackend::new(device, AccessTable::original()).unwrap());
    session
        .configure_frame_imports(FrameImportConfig { max_imports: 1 })
        .unwrap();
    let baseline = session.resource_count();
    let mut graph = session.begin_frame().unwrap();
    let first = session
        .import_frame_texture(&mut graph, texture.clone(), 0x1d)
        .unwrap();
    assert_eq!(
        session
            .import_frame_texture(&mut graph, texture.clone(), 0x1d)
            .unwrap(),
        first
    );
    assert_eq!(graph.declaration().resources.len(), 1);
    assert_eq!(session.frame_import_count(), 1);
    assert_eq!(session.resource_count(), baseline + 1);
    drop(graph);
    drop(session.begin_frame().unwrap());
    assert_eq!(session.frame_import_count(), 0);
    assert_eq!(session.resource_count(), baseline);
    let token = session
        .backend
        .bind_import(GpuResource::Image(texture.clone()))
        .unwrap();
    let mut graph = session.begin_frame().unwrap();
    let manual = graph
        .import_unowned(
            "manual frame texture",
            GpuResource::Image(texture.clone()).descriptor(),
            token,
            0x1d,
        )
        .unwrap();
    assert_eq!(
        session
            .import_frame_texture(&mut graph, texture, 0x1d)
            .unwrap(),
        manual
    );
    assert_eq!(graph.declaration().resources.len(), 1);
    assert_eq!(session.frame_import_count(), 0);
    drop(graph);
    session.release_import(token).unwrap();
}
