use super::*;
#[derive(Default)]
struct Capture { emitted: usize, panic: bool }
impl CommandBackend for Capture {
    fn dispatch(&mut self, _: vk::CommandBuffer, _: Command<'_>) {
        assert!(!self.panic, "injected backend rejection before native call");
        self.emitted += 1;
    }
}
#[test]
fn native_command_workload_counts_api_calls_not_indirect_or_instance_parameters() {
    let mut backend = Capture::default();
    let mut api = RenderPassApi::new(&mut backend, vk::CommandBuffer::null(), 0, false);
    api.draw(3, 200, 0, 0);
    api.draw_indexed(12, 700, 0, 0, 0);
    api.draw_indirect(vk::Buffer::null(), 0, 3000, 16);
    api.draw_indirect_count(vk::Buffer::null(), 0, vk::Buffer::null(), 0, 9000, 16);
    api.draw_indexed_indirect(vk::Buffer::null(), 0, 4000, 20);
    api.draw_indexed_indirect_count(vk::Buffer::null(), 0, vk::Buffer::null(), 0, 8000, 20);
    api.dispatch_groups_3d(4, 5, 6);
    api.dispatch_threads_3d(15, 17, 19,
        std::num::NonZeroU32::new(8).unwrap(), std::num::NonZeroU32::new(8).unwrap(),
        std::num::NonZeroU32::new(8).unwrap());
    api.dispatch_indirect(vk::Buffer::null(), 0);
    assert_eq!(api.take_command_workload(), crate::profile::CommandWorkload {
        direct_draw_api_calls: 1, indexed_draw_api_calls: 1,
        indirect_draw_api_calls: 1, indirect_count_draw_api_calls: 1,
        indexed_indirect_draw_api_calls: 1, indexed_indirect_count_draw_api_calls: 1,
        direct_dispatch_api_calls: 2, indirect_dispatch_api_calls: 1,
        ..Default::default()
    });
    assert_eq!(api.take_command_workload(), crate::profile::CommandWorkload::default(), "take cannot duplicate accounting");
    drop(api);
    assert_eq!(backend.emitted, 9);
}
#[test]
fn separate_instanced_ranges_count_each_native_call_including_zero_instances() {
    let mut backend = Capture::default();
    let mut api = RenderPassApi::new(&mut backend, vk::CommandBuffer::null(), 0, false);
    // The host MultiInstanced path emits one indexed call for every pair.
    for [first, count] in [[0, 7], [12, 0], [30, 1000]] {
        api.draw_indexed(6, count, 0, 0, first);
    }
    let measured = api.take_command_workload();
    assert!(measured.valid);
    assert_eq!(measured.indexed_draw_api_calls, 3);
    assert_eq!(measured.direct_draw_api_calls, 0);
    drop(api);
    assert_eq!(backend.emitted, 3);
}
#[test]
fn rejected_commands_and_panicking_backend_do_not_count_as_recorded_calls() {
    let mut backend = Capture::default();
    let mut api = RenderPassApi::new(&mut backend, vk::CommandBuffer::null(), 0, false);
    assert!(api.begin_dynamic_rendering(&vk::RenderingInfo::default()).is_err());
    assert!(api.fill_buffer(vk::Buffer::null(), Some((1, 4)), 0).is_err());
    api.set_line_width(2.0); // disabled capability emits nothing
    assert_eq!(api.take_command_workload(), crate::profile::CommandWorkload::default());
    drop(api);
    assert_eq!(backend.emitted, 0);
    backend.panic = true;
    let mut api = RenderPassApi::new(&mut backend, vk::CommandBuffer::null(), 0, false);
    assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| api.draw(3, 1, 0, 0))).is_err());
    assert_eq!(api.take_command_workload().direct_draw_api_calls, 0);
}
