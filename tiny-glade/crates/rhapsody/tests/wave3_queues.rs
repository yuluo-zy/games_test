//! 辅助队列 / 跨队列族（wave3）。
//!
//! `init.rs::create_device` 只在存在“独立 compute 族”（flags & 3 == COMPUTE）时才创建辅助队列。lavapipe 只暴露
//! 一个通用族，所以真正的跨族执行在本机不可能；这里分三层：
//! 1. 始终执行：列出队列族；没有辅助队列时 `Queue::Auxiliary(0)` 必须被拒绝（不伪造辅助队列）；
//!    跨族归属的导入资源必须被 `validate_graph` 拒绝（“未提供 release/acquire 计划”）——用不存在的族号触发，真实走该规则。
//! 2. 仅当设备有辅助队列时执行：在辅助队列上提交一个填充+拷贝 pass 图并读回，验证辅助队列时间线推进。
//! 3. 否则打印跳过原因。
mod draw_support;
mod wave3_support;
use ash::vk::{self, Handle};
use rhapsody::{
    raw::buffer::{Buffer, BufferDesc},
    rg::{
        executor::PassContext,
        vulkan::{AccessTable, GpuResource, ResourceSpec, VulkanGraphBackend, VulkanGraphSession},
    },
};
use rhapsody::raw::managed::lifecycle::Queue;
use std::sync::Arc;
use wave3_support::*;

fn families(device: &rhapsody::raw::device::VulkanDevice) -> Vec<vk::QueueFamilyProperties> {
    unsafe { device.instance.instance.get_physical_device_queue_family_properties(device.physical) }
}

#[test]
fn queue_families_are_listed_and_auxiliary_queues_exist_only_for_an_independent_compute_family() {
    let Some(env) = validated_env() else { return };
    let device = &env.device;
    let props = families(device);
    for (i, f) in props.iter().enumerate() {
        println!("队列族 {i}: 数量 {} flags {:?}", f.queue_count, f.queue_flags);
    }
    let independent_compute: Vec<usize> = props
        .iter()
        .enumerate()
        .filter(|(_, f)| f.queue_count > 0 && (f.queue_flags.as_raw() & 3) == vk::QueueFlags::COMPUTE.as_raw())
        .map(|(i, _)| i)
        .collect();
    // create_device 的规则：辅助队列 = 独立 compute 族的全部队列（主族若恰是它则排除主队列本身）。
    let expected: usize = independent_compute.iter().map(|&i| props[i].queue_count as usize).sum();
    let expected = expected - usize::from(independent_compute.contains(&(device.queue_family as usize)));
    assert_eq!(device.additional_queues.len(), expected, "辅助队列数应等于独立 compute 族的队列总数");
    assert!(device.additional_queues.iter().all(|q| independent_compute.contains(&(q.family as usize))));
    if device.additional_queues.is_empty() {
        println!("本机没有独立 compute 队列族（{} 个族），辅助队列路径不可测", props.len());
        // 不伪造：请求辅助队列的后端必须失败。
        assert!(VulkanGraphBackend::new_for_queue(device.clone(), AccessTable::original(), Queue::Auxiliary(0)).is_err());
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
    let token = session.backend.bind_import_owned_by(GpuResource::Buffer(buffer), foreign);
    let mut graph = session.begin_frame().unwrap();
    let handle = graph
        .import_unowned("foreign", ResourceSpec::Buffer(desc).descriptor(), token, HOST_READ)
        .unwrap();
    let index = handle.raw();
    let mut pass = graph.add_pass("touch");
    pass.add_resource_access(handle, TRANSFER_WRITE, 0).unwrap();
    pass.render_with_context(move |_: PassContext<'_>| {
        let _ = index;
        true
    });
    graph.export(handle, HOST_READ).unwrap();
    let error = session.execute_frame(graph.build()).expect_err("跨族归属必须拒绝");
    assert!(error.contains("release/acquire"), "{error}");
    assert!(!session.is_poisoned(), "校验阶段拒绝不应使会话停止");
}

#[test]
fn a_graph_runs_on_the_auxiliary_queue_when_the_device_has_one() {
    let Some(env) = validated_env() else { return };
    let device = &env.device;
    let Some(aux) = device.additional_queues.first().copied() else {
        println!("跳过：设备没有辅助队列（无独立 compute 族），不伪造辅助队列");
        return;
    };
    let backend = VulkanGraphBackend::new_for_queue(device.clone(), AccessTable::original(), Queue::Auxiliary(0)).unwrap();
    assert_eq!(backend.selected_family(), aux.family);
    let mut session = VulkanGraphSession::new(backend);
    let desc = BufferDesc::new_gpu_to_cpu(BUF_BYTES, vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::TRANSFER_SRC);
    let make = |session: &mut VulkanGraphSession| {
        let buffer = Arc::new(Buffer::new(device.clone(), &desc).unwrap());
        (session.backend.bind_import_owned_by(GpuResource::Buffer(buffer.clone()), aux.family), buffer)
    };
    let (src_token, src) = make(&mut session);
    let (dst_token, dst) = make(&mut session);
    let mut graph = session.begin_frame().unwrap();
    let spec = ResourceSpec::Buffer(desc).descriptor();
    let s = graph.import_unowned("src", spec, src_token, HOST_READ).unwrap();
    let d = graph.import_unowned("dst", spec, dst_token, HOST_READ).unwrap();
    let raw = device.clone();
    let (si, di) = (s.raw(), d.raw());
    {
        let raw = raw.clone();
        let mut pass = graph.add_pass("aux_fill");
        pass.add_resource_access(s, TRANSFER_WRITE, 0).unwrap();
        pass.render_with_context(move |ctx: PassContext<'_>| {
            let Some(m) = ctx.managed else { return false };
            let Ok(b) = m.buffer_for(si) else { return false };
            ctx.keep_alive(Arc::new(b.clone()));
            let cb = vk::CommandBuffer::from_raw(ctx.command.handle());
            unsafe { raw.device.cmd_fill_buffer(cb, b.raw(), 0, vk::WHOLE_SIZE, 0xA5A5_1234) };
            true
        });
    }
    {
        let mut pass = graph.add_pass("aux_copy");
        pass.add_resource_access(s, TRANSFER_READ, 0).unwrap();
        pass.add_resource_access(d, TRANSFER_WRITE, 0).unwrap();
        pass.render_with_context(move |ctx: PassContext<'_>| {
            let Some(m) = ctx.managed else { return false };
            let (Ok(a), Ok(b)) = (m.buffer_for(si), m.buffer_for(di)) else { return false };
            ctx.keep_alive(Arc::new(a.clone()));
            ctx.keep_alive(Arc::new(b.clone()));
            let cb = vk::CommandBuffer::from_raw(ctx.command.handle());
            unsafe {
                raw.device.cmd_copy_buffer(cb, a.raw(), b.raw(), &[vk::BufferCopy { src_offset: 0, dst_offset: 0, size: BUF_BYTES }])
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
    let timeline = session.chain.device.auxiliary[0].timeline_semaphore;
    let value = unsafe { device.device.get_semaphore_counter_value(timeline) }.unwrap();
    assert!(value > 0, "辅助队列时间线必须推进");
    // 归属于主族的资源在辅助队列图里必须被拒绝（跨族无 release/acquire 计划）。
    let primary_buffer = Arc::new(Buffer::new(device.clone(), &desc).unwrap());
    let token = session.backend.bind_import(GpuResource::Buffer(primary_buffer));
    let mut graph = session.begin_frame().unwrap();
    let h = graph.import_unowned("p", spec, token, HOST_READ).unwrap();
    let mut pass = graph.add_pass("bad");
    pass.add_resource_access(h, TRANSFER_WRITE, 0).unwrap();
    pass.render_with_context(|_: PassContext<'_>| true);
    graph.export(h, HOST_READ).unwrap();
    assert!(session.execute_frame(graph.build()).unwrap_err().contains("release/acquire"));
}
