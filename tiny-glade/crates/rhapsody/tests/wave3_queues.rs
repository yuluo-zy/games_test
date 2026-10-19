//! 配置队列的真实创建、类别别名与辅助队列读回。测试覆盖当前设备实际提供的队列。
mod draw_support;
mod wave3_support;
use ash::vk::{self, Handle};
use rhapsody::command::recording::Queue;
use rhapsody::resource::{OwnershipRange, range_state::RangeOwner};
use rhapsody::{
    raw::buffer::{Buffer, BufferDesc},
    rg::{
        executor::{ExecutionBackend, PassContext},
        graph::RenderGraph,
        sync_plan::{BarrierKind, DependencyRef},
        vulkan::{AccessTable, GpuResource, ResourceSpec, VulkanGraphBackend, VulkanGraphSession},
    },
};
use std::sync::Arc;
use wave3_support::*;

fn families(device: &rhapsody::raw::device::Device) -> Vec<vk::QueueFamilyProperties> {
    unsafe {
        device
            .instance()
            .raw_instance()
            .get_physical_device_queue_family_properties(device.physical_device())
    }
}

#[test]
fn configured_queue_plan_matches_created_handles_and_timeline_slots() {
    use rhapsody::device::{FamilyInfo, QueueClass, QueueConfig, plan_queues};
    for config in [
        QueueConfig {
            require_present: false,
            ..Default::default()
        },
        QueueConfig {
            require_present: false,
            ..QueueConfig::prefer_all()
        },
    ] {
        let Some(env) = validated_env_configured(&config) else {
            return;
        };
        let device = &env.device;
        let families = families(device)
            .iter()
            .map(|props| FamilyInfo::from_properties(props, false))
            .collect::<Vec<_>>();
        let expected = plan_queues(&families, &config).unwrap();
        assert_eq!(*device.queue_plan(), expected);
        assert_eq!(device.timelines().queue_count(), expected.queues.len());
        assert_eq!(device.additional_queues().len() + 1, expected.queues.len());
        for (index, physical) in expected.queues.iter().enumerate() {
            let handle = unsafe {
                device
                    .raw_device()
                    .get_device_queue(physical.family, physical.index)
            };
            assert_eq!(device.queue_id(handle).unwrap().index(), index);
        }
        for class in QueueClass::ALL {
            let backend =
                VulkanGraphBackend::new_for_class(device.clone(), AccessTable::original(), class)
                    .unwrap();
            assert_eq!(backend.selected_family(), expected.physical(class).family);
        }
    }
}

fn execute_checked(
    session: &mut VulkanGraphSession,
    graph: RenderGraph,
) -> rhapsody::rg::vulkan::SessionFrameReceipt {
    let built = graph.build();
    let context = session.backend.plan_context();
    let definition = rhapsody::rg::plan::GraphDefinition::from_declaration(
        &built.declaration,
        &mut session.backend.accesses,
        context,
    )
    .unwrap();
    let plan = rhapsody::rg::plan::compile_definition(definition).unwrap();
    let receipt = session.execute_frame(built).unwrap();
    rhapsody::rg::plan_check::check_bound_synchronization(
        &plan.range_program,
        session.last_synchronization(),
    )
    .unwrap();
    receipt
}

/// Establish ownership by a real primary-queue submission, then let the graph
/// perform the exact release/acquire and copy on the selected auxiliary queue.
/// A family hint on a never-used buffer cannot establish physical ownership.
fn copy_from_actual_primary_owner(
    env: &ValidatedEnv,
    destination: &mut VulkanGraphSession,
    auxiliary: rhapsody::raw::device::DeviceQueue,
) {
    let device = &env.device;
    assert_ne!(auxiliary.family, device.queue_family_index());
    let primary_queue = device.queue_id(device.primary_queue()).unwrap();
    let destination_queue = device.queue_id(auxiliary.handle).unwrap();
    let desc = BufferDesc::new_gpu_to_cpu(
        BUF_BYTES,
        vk::BufferUsageFlags::TRANSFER_SRC | vk::BufferUsageFlags::TRANSFER_DST,
    );
    let source = Arc::new(Buffer::new(device.clone(), &desc).unwrap());
    let output = Arc::new(Buffer::new(device.clone(), &desc).unwrap());
    let spec = ResourceSpec::Buffer(desc).descriptor();
    let mut primary = VulkanGraphSession::new(
        VulkanGraphBackend::new(device.clone(), AccessTable::original()).unwrap(),
    );
    let token = primary
        .backend
        .bind_import(GpuResource::Buffer(source.clone()))
        .unwrap();
    let mut graph = primary.begin_frame().unwrap();
    let src = graph
        .import_unowned("primary actual owner", spec, token, HOST_READ)
        .unwrap();
    let mut fill = graph.add_pass("primary fill establishes real ownership");
    fill.add_resource_access(src, TRANSFER_WRITE, 0).unwrap();
    let raw = device.clone();
    fill.render_with_result_context(move |ctx| {
        let buffer = ctx.managed.unwrap().buffer_for(src.raw())?;
        unsafe {
            raw.raw_device().cmd_fill_buffer(
                vk::CommandBuffer::from_raw(ctx.command.handle()),
                buffer.raw(),
                0,
                BUF_BYTES,
                0x1A2B_3C4D,
            );
        }
        Ok(())
    });
    graph.export(src, TRANSFER_READ).unwrap();
    let source_receipt = execute_checked(&mut primary, graph);
    let source_point = *source_receipt
        .receipts
        .iter()
        .find(|point| point.queue == primary_queue)
        .unwrap();
    assert!(source_point.value > 0);
    primary
        .backend
        .resource_native_state(token, |cells| {
            assert!(
                cells
                    .iter()
                    .all(|cell| cell.state.owner == RangeOwner::Owned(device.queue_family_index()))
            );
            assert!(
                cells
                    .iter()
                    .all(|cell| cell.state.dependencies(true).get(primary_queue)
                        >= source_point.value)
            );
        })
        .unwrap();

    let input_token = destination
        .backend
        .bind_import(GpuResource::Buffer(source.clone()))
        .unwrap();
    let output_token = destination
        .backend
        .bind_import_owned_by(GpuResource::Buffer(output.clone()), auxiliary.family)
        .unwrap();
    let mut graph = destination.begin_frame().unwrap();
    let input = graph
        .import_unowned("real primary source", spec, input_token, TRANSFER_READ)
        .unwrap();
    let output_handle = graph
        .import_unowned("auxiliary readback", spec, output_token, HOST_READ)
        .unwrap();
    let mut copy = graph.add_pass("automatic primary to auxiliary handoff");
    copy.add_resource_access(input, TRANSFER_READ, 0).unwrap();
    copy.add_resource_access(output_handle, TRANSFER_WRITE, 0)
        .unwrap();
    let raw = device.clone();
    copy.render_with_result_context(move |ctx| {
        let managed = ctx.managed.unwrap();
        unsafe {
            raw.raw_device().cmd_copy_buffer(
                vk::CommandBuffer::from_raw(ctx.command.handle()),
                managed.buffer_for(input.raw())?.raw(),
                managed.buffer_for(output_handle.raw())?.raw(),
                &[vk::BufferCopy {
                    src_offset: 0,
                    dst_offset: 0,
                    size: BUF_BYTES,
                }],
            );
        }
        Ok(())
    });
    graph.export(input, HOST_READ).unwrap();
    graph.export(output_handle, HOST_READ).unwrap();
    let receipt = execute_checked(destination, graph);
    let point = *receipt
        .receipts
        .iter()
        .filter(|point| point.queue == destination_queue)
        .max_by_key(|point| point.value)
        .unwrap();
    assert!(point.value > 0);
    let bound = destination.last_synchronization();
    let whole = OwnershipRange::Buffer {
        offset: 0,
        size: BUF_BYTES,
    };
    let release = bound
        .transitions()
        .iter()
        .find(|transition| {
            transition.resource == input.raw()
                && transition.range == whole
                && matches!(transition.kind, BarrierKind::Release { .. })
                && transition.source_family == device.queue_family_index()
                && transition.destination_family == auxiliary.family
        })
        .unwrap();
    let BarrierKind::Release { pair } = release.kind else {
        unreachable!()
    };
    assert_eq!(
        bound.batches()[release.batch].domain.queue_index,
        primary_queue.index() as u32
    );
    let acquire = bound
        .transitions()
        .iter()
        .find(|transition| {
            transition.resource == input.raw()
                && transition.range == whole
                && transition.kind == BarrierKind::Acquire { pair }
                && transition.source_family == release.source_family
                && transition.destination_family == release.destination_family
        })
        .unwrap();
    assert_eq!(
        bound.batches()[acquire.batch].domain.queue_index,
        destination_queue.index() as u32
    );
    assert!(bound.waits().iter().any(|wait| wait.batch == acquire.batch
        && wait.dependency == DependencyRef::Batch(release.batch)
        && !wait.stages.is_empty()));
    // Check native queue submissions too: the destination really waits on the
    // primary release signal, rather than only drawing a symbolic pair.
    let primary_semaphore = device.timelines().semaphore(primary_queue).unwrap();
    let captures = &destination.backend.submissions;
    assert!(captures.iter().any(|capture| {
        capture.queue == device.primary_queue()
            && capture.result == 0
            && capture
                .batches
                .iter()
                .any(|batch| !batch.commands.is_empty())
    }));
    // Device::submit_prepared appends its own timeline signal only inside the
    // native call; it is intentionally absent from the caller-owned capture.
    // The returned successful-submit receipt is its authoritative real value.
    let release_point = *receipt
        .receipts
        .iter()
        .filter(|point| point.queue == primary_queue)
        .max_by_key(|point| point.value)
        .expect("the actual primary ownership release must publish a receipt");
    assert!(device.timelines().submitted(primary_queue).unwrap() >= release_point.value);
    let release_value = release_point.value;
    assert!(release_value > source_point.value);
    assert!(
        captures
            .iter()
            .any(|capture| capture.queue == auxiliary.handle
                && capture.result == 0
                && capture.batches.iter().any(|batch| batch
                    .wait_semaphores
                    .iter()
                    .zip(&batch.wait_values)
                    .any(|(semaphore, value)| *semaphore == primary_semaphore
                        && *value >= release_value)))
    );
    destination.wait_submits_done(10_000_000_000).unwrap();
    let expected = 0x1A2B_3C4Du32.to_le_bytes().repeat(BUF_BYTES as usize / 4);
    for buffer in [&source, &output] {
        let mut bytes = vec![0; BUF_BYTES as usize];
        unsafe {
            buffer.read_bytes_after_completion(0, &mut bytes).unwrap();
        }
        assert_eq!(bytes, expected);
    }
    for token in [input_token, output_token] {
        destination
            .backend
            .resource_native_state(token, |cells| {
                assert!(
                    cells
                        .iter()
                        .all(|cell| cell.state.owner == RangeOwner::Owned(auxiliary.family))
                );
                assert!(cells.iter().all(
                    |cell| cell.state.dependencies(true).get(destination_queue) >= point.value
                ));
            })
            .unwrap();
    }
    primary.wait_submits_done(10_000_000_000).unwrap();
    assert!(!destination.is_poisoned());
}

#[test]
fn a_real_primary_owner_is_released_and_acquired_before_an_auxiliary_copy() {
    let Some(env) = validated_env_configured(&rhapsody::device::QueueConfig {
        require_present: false,
        ..rhapsody::device::QueueConfig::prefer_all()
    }) else {
        return;
    };
    let Some((index, auxiliary)) = env
        .device
        .additional_queues()
        .iter()
        .copied()
        .enumerate()
        .find(|(_, queue)| queue.family != env.device.queue_family_index())
    else {
        eprintln!("SKIP actual ownership handoff: no configured auxiliary family");
        return;
    };
    let mut destination = VulkanGraphSession::new(
        VulkanGraphBackend::new_for_queue(
            env.device.clone(),
            AccessTable::original(),
            Queue::Auxiliary(index as u32),
        )
        .unwrap(),
    );
    copy_from_actual_primary_owner(&env, &mut destination, auxiliary);
    drop(destination);
    assert!(
        env.messenger.collector.errors().is_empty(),
        "{:?}",
        env.messenger.collector.errors()
    );
    assert!(
        env.messenger.collector.warnings().is_empty(),
        "{:?}",
        env.messenger.collector.warnings()
    );
}

#[test]
fn a_graph_runs_on_the_auxiliary_queue_when_the_device_has_one() {
    let Some(env) = validated_env_configured(&rhapsody::device::QueueConfig {
        require_present: false,
        ..rhapsody::device::QueueConfig::prefer_all()
    }) else {
        return;
    };
    let device = &env.device;
    let Some(aux) = device.additional_queues().first().copied() else {
        println!("跳过：设备没有辅助队列（无独立 compute 族），不伪造辅助队列");
        return;
    };
    let backend = VulkanGraphBackend::new_for_queue(
        device.clone(),
        AccessTable::original(),
        Queue::Auxiliary(0),
    )
    .unwrap();
    assert_eq!(backend.selected_family(), aux.family);
    let mut session = VulkanGraphSession::new(backend);
    let desc = BufferDesc::new_gpu_to_cpu(
        BUF_BYTES,
        vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::TRANSFER_SRC,
    );
    let make = |session: &mut VulkanGraphSession| {
        let buffer = Arc::new(Buffer::new(device.clone(), &desc).unwrap());
        (
            session
                .backend
                .bind_import_owned_by(GpuResource::Buffer(buffer.clone()), aux.family)
                .unwrap(),
            buffer,
        )
    };
    let (src_token, src) = make(&mut session);
    let (dst_token, dst) = make(&mut session);
    let mut graph = session.begin_frame().unwrap();
    let spec = ResourceSpec::Buffer(desc).descriptor();
    let s = graph
        .import_unowned("src", spec, src_token, HOST_READ)
        .unwrap();
    let d = graph
        .import_unowned("dst", spec, dst_token, HOST_READ)
        .unwrap();
    let raw = device.clone();
    let (si, di) = (s.raw(), d.raw());
    {
        let raw = raw.clone();
        let mut pass = graph.add_pass("aux_fill").with_side_effect();
        pass.add_resource_access(s, TRANSFER_WRITE, 0).unwrap();
        pass.render_with_context(move |ctx: PassContext<'_>| {
            let Some(m) = ctx.managed else { return false };
            let Ok(b) = m.buffer_for(si) else {
                return false;
            };

            let cb = vk::CommandBuffer::from_raw(ctx.command.handle());
            unsafe {
                raw.raw_device()
                    .cmd_fill_buffer(cb, b.raw(), 0, vk::WHOLE_SIZE, 0xA5A5_1234)
            };
            true
        });
    }
    {
        let mut pass = graph.add_pass("aux_copy").with_side_effect();
        pass.add_resource_access(s, TRANSFER_READ, 0).unwrap();
        pass.add_resource_access(d, TRANSFER_WRITE, 0).unwrap();
        pass.render_with_context(move |ctx: PassContext<'_>| {
            let Some(m) = ctx.managed else { return false };
            let (Ok(a), Ok(b)) = (m.buffer_for(si), m.buffer_for(di)) else {
                return false;
            };

            let cb = vk::CommandBuffer::from_raw(ctx.command.handle());
            unsafe {
                raw.raw_device().cmd_copy_buffer(
                    cb,
                    a.raw(),
                    b.raw(),
                    &[vk::BufferCopy {
                        src_offset: 0,
                        dst_offset: 0,
                        size: BUF_BYTES,
                    }],
                )
            };
            true
        });
    }
    graph.export(s, HOST_READ).unwrap();
    graph.export(d, HOST_READ).unwrap();
    let receipt = execute_checked(&mut session, graph);
    let auxiliary_queue = device.queue_id(aux.handle).unwrap();
    assert!(
        receipt
            .receipts
            .iter()
            .any(|point| point.queue == auxiliary_queue && point.value > 0)
    );
    assert!(
        session
            .backend
            .submissions
            .iter()
            .any(|capture| capture.queue == aux.handle && capture.result == 0)
    );
    session.wait_submits_done(10_000_000_000).unwrap();
    let want = 0xA5A5_1234u32.to_le_bytes().repeat(BUF_BYTES as usize / 4);
    for buffer in [&src, &dst] {
        let mut bytes = vec![0u8; BUF_BYTES as usize];
        unsafe { buffer.read_bytes_after_completion(0, &mut bytes).unwrap() };
        assert_eq!(bytes, want);
    }
    let timeline = device
        .timelines()
        .semaphore(device.queue_id(aux.handle).unwrap())
        .unwrap();
    let value = unsafe { device.raw_device().get_semaphore_counter_value(timeline) }.unwrap();
    assert!(value > 0, "辅助队列时间线必须推进");
    if aux.family != device.queue_family_index() {
        copy_from_actual_primary_owner(&env, &mut session, aux);
    }
    drop(session);
    assert!(
        env.messenger.collector.errors().is_empty(),
        "{:?}",
        env.messenger.collector.errors()
    );
    assert!(
        env.messenger.collector.warnings().is_empty(),
        "{:?}",
        env.messenger.collector.warnings()
    );
}
