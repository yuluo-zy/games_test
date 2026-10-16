//! 配置队列的真实创建、类别别名与辅助队列读回。测试覆盖当前设备实际提供的队列。
mod draw_support;
mod wave3_support;
use ash::vk::{self, Handle};
use rhapsody::raw::managed::lifecycle::Queue;
use rhapsody::{
    raw::buffer::{Buffer, BufferDesc},
    rg::{
        executor::PassContext,
        vulkan::{AccessTable, GpuResource, ResourceSpec, VulkanGraphBackend, VulkanGraphSession},
    },
};
use std::sync::Arc;
use wave3_support::*;

fn families(device: &rhapsody::raw::device::VulkanDevice) -> Vec<vk::QueueFamilyProperties> {
    unsafe {
        device
            .instance
            .instance
            .get_physical_device_queue_family_properties(device.physical)
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
        assert_eq!(device.queue_plan.as_ref(), Some(&expected));
        assert_eq!(device.timelines.queue_count(), expected.queues.len());
        assert_eq!(device.additional_queues.len() + 1, expected.queues.len());
        for (index, physical) in expected.queues.iter().enumerate() {
            let handle = unsafe {
                device
                    .device
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

#[test]
fn an_import_owned_by_another_queue_family_is_rejected_without_a_release_acquire_plan() {
    let Some(env) = validated_env() else { return };
    let backend = VulkanGraphBackend::new(env.device.clone(), AccessTable::original()).unwrap();
    let mut session = VulkanGraphSession::new(backend);
    let desc = BufferDesc::new_gpu_to_cpu(BUF_BYTES, vk::BufferUsageFlags::TRANSFER_DST);
    let buffer = Arc::new(Buffer::new(env.device.clone(), &desc).unwrap());
    // 归属于一个不是图队列族的族号（不要求该族真的存在：规则只比较归属与图所在族）。
    let foreign = env.device.queue_family + 1;
    let token = session
        .backend
        .bind_import_owned_by(GpuResource::Buffer(buffer), foreign);
    let mut graph = session.begin_frame().unwrap();
    let handle = graph
        .import_unowned(
            "foreign",
            ResourceSpec::Buffer(desc).descriptor(),
            token,
            HOST_READ,
        )
        .unwrap();
    let index = handle.raw();
    let mut pass = graph.add_pass("touch");
    pass.add_resource_access(handle, TRANSFER_WRITE, 0).unwrap();
    pass.render_with_context(move |_: PassContext<'_>| {
        let _ = index;
        true
    });
    graph.export(handle, HOST_READ).unwrap();
    let error = session
        .execute_frame(graph.build())
        .expect_err("跨族归属必须拒绝");
    assert_eq!(error.kind(), rhapsody::ErrorKind::InvalidUsage);
    assert!(!session.is_poisoned(), "校验阶段拒绝不应使会话停止");
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
    let Some(aux) = device.additional_queues.first().copied() else {
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
                .bind_import_owned_by(GpuResource::Buffer(buffer.clone()), aux.family),
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
        let mut pass = graph.add_pass("aux_fill");
        pass.add_resource_access(s, TRANSFER_WRITE, 0).unwrap();
        pass.render_with_context(move |ctx: PassContext<'_>| {
            let Some(m) = ctx.managed else { return false };
            let Ok(b) = m.buffer_for(si) else {
                return false;
            };
            ctx.keep_alive(Arc::new(b.clone()));
            let cb = vk::CommandBuffer::from_raw(ctx.command.handle());
            unsafe {
                raw.device
                    .cmd_fill_buffer(cb, b.raw(), 0, vk::WHOLE_SIZE, 0xA5A5_1234)
            };
            true
        });
    }
    {
        let mut pass = graph.add_pass("aux_copy");
        pass.add_resource_access(s, TRANSFER_READ, 0).unwrap();
        pass.add_resource_access(d, TRANSFER_WRITE, 0).unwrap();
        pass.render_with_context(move |ctx: PassContext<'_>| {
            let Some(m) = ctx.managed else { return false };
            let (Ok(a), Ok(b)) = (m.buffer_for(si), m.buffer_for(di)) else {
                return false;
            };
            ctx.keep_alive(Arc::new(a.clone()));
            ctx.keep_alive(Arc::new(b.clone()));
            let cb = vk::CommandBuffer::from_raw(ctx.command.handle());
            unsafe {
                raw.device.cmd_copy_buffer(
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
    session.execute_frame(graph.build()).expect("辅助队列提交");
    session.wait_submits_done(10_000_000_000).unwrap();
    let want = 0xA5A5_1234u32.to_le_bytes().repeat(BUF_BYTES as usize / 4);
    for buffer in [&src, &dst] {
        let mut bytes = vec![0u8; BUF_BYTES as usize];
        unsafe { buffer.read_bytes_after_completion(0, &mut bytes).unwrap() };
        assert_eq!(bytes, want);
    }
    let timeline = device
        .timelines
        .semaphore(device.queue_id(aux.handle).unwrap())
        .unwrap();
    let value = unsafe { device.device.get_semaphore_counter_value(timeline) }.unwrap();
    assert!(value > 0, "辅助队列时间线必须推进");
    if aux.family == device.queue_family {
        return;
    }
    // 不同族之间尚未提供 release/acquire 计划，必须拒绝。
    let primary_buffer = Arc::new(Buffer::new(device.clone(), &desc).unwrap());
    let token = session
        .backend
        .bind_import(GpuResource::Buffer(primary_buffer));
    let mut graph = session.begin_frame().unwrap();
    let h = graph.import_unowned("p", spec, token, HOST_READ).unwrap();
    let mut pass = graph.add_pass("bad");
    pass.add_resource_access(h, TRANSFER_WRITE, 0).unwrap();
    pass.render_with_context(|_: PassContext<'_>| true);
    graph.export(h, HOST_READ).unwrap();
    assert_eq!(
        session.execute_frame(graph.build()).unwrap_err().kind(),
        rhapsody::ErrorKind::InvalidUsage
    );
}
