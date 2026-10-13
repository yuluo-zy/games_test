//! pass_resources.rs — PassResources（PassBuilder::render<UserData=PassResources> 的用户数据）。
//!
//! 机器码锚点：`PassResources::new_from` 0x140909310（行 24）。
//! 7 个资源指针按 hash 从 ECS world 资源表取出（56B = 7×u64）。

use bevy_ecs::world::World;

/// 7 个帧级核心资源指针。
/// 名称映射未恢复（typed gap）；推断为 frame_constants / frame_constants_storage_buffer /
/// camera / Globals / sampler set 等（见 INDEX.md descriptor 约定 set1/b0、set1/b1、set0/13）。
#[derive(Clone, Copy, Debug, Default)]
pub struct PassResources {
    pub r0: *const (),
    pub r1: *const (),
    pub r2: *const (),
    pub r3: *const (),
    pub r4: *const (),
    pub r5: *const (),
    pub r6: *const (),
}

impl PassResources {
    /// 按 7 个预计算 hash 从 ECS world 资源表取资源指针（0x140909310）。
    /// 首个必选，其余缺失填 0；链式查找任一必选缺失 → `unwrap_failed`。
    pub fn new_from(world: &World) -> PassResources {
        let mut out = PassResources::default();
        out.r0 = get_resource_ptr(world, 0x8597ee0727b461b3).unwrap_or_else(|| missing());
        out.r1 = get_resource_ptr(world, 0xf0b963d163fe760f).unwrap_or(core::ptr::null());
        out.r2 = get_resource_ptr(world, 0xa68561b396784eaa).unwrap_or_else(|| missing());
        out.r3 = get_resource_ptr(world, 0x8226cc786e679355).unwrap_or_else(|| missing());
        out.r4 = get_resource_ptr(world, 0x874b1541877dacfd).unwrap_or_else(|| missing());
        out.r5 = get_resource_ptr(world, 0x47c94cb78283a72e).unwrap_or_else(|| missing());
        out.r6 = get_resource_ptr(world, 0xb65bf3141f93e938).unwrap_or_else(|| missing());
        out
    }
}

/// `Components::get_resource_id` + world 资源表查找（hash → 指针）。
fn get_resource_ptr(_world: &World, _hash: u64) -> Option<*const ()> {
    None // typed gap：bevy world 内部资源表布局；经 unsafe_world_cell::get_resource 落实
}

#[cold]
fn missing() -> *const () {
    // core::option::unwrap_failed
    panic!("PassResources::new_from: mandatory resource missing")
}
