//! add_birdnests（B 主基线；0x1422c9c70，源码 37–141；closure$0 L84、closure$1 L105）。
//!
//! fresh 反编译（2026-10-17，--worker shared）控制流：
//! 1. puffin 范围；
//! 2. `Subscriber::take<DirtyTiles>`（0x141247c90）→ 复制 0x200 字节位集 → 合入本地
//!    `DirtyTiles::new_clear`；
//! 3. 遍历脏格记录（stride 0x40；cell 坐标 @+0x28、位置 @+0x30，来源宿主装配 U）：
//!    每格 `hash_one<&GlobalId>` 查已处理集（`HashMap<u64,bool>`，0x140eec230）；
//!    未处理 → `closure_0`（0x1422ca8c0，鸟巢几何/合法性，未逐行还原 U）+ 集插入；
//! 4. 直接写输出候选（state+0x48，stride 0x68）：+0x00=4（type）、+0x08 Arc 强计数++
//!    （LOCK 语义）、位置（+0x15 偏移写入 +0x48）、+0x60=1.0 缩放。
//! 即每脏格 1 鸟巢候选。

use bevy_ecs::prelude::*;
use hashbrown::HashMap;

use country_core::systems::collision::world::RaycastWorld;
use country_core::utils::dirty_tiles::DirtyTiles;
use std::sync::Mutex;
use tabloid::Subscriber;

use super::populate_autoclutter_regions::{dirty_tiles_cells, DirtyTilesResource};
use super::{AutoClutterSpawn, AutoClutterState, SpawnShared};

/// `BirdNestLazySubscriber`（原版 `add_birdnests::BirdNestLazySubscriber`）：`add_birdnests` 的
/// `ResMut` 参数（校验串 0x142f330c3），由 Startup 系统 [`birdnests_subscribe_to_tabloids`] 经
/// `Commands::insert_resource` 在运行期插入（原版值 = 一个 8 B `Subscriber<DirtyTiles>` 句柄，
/// `*param + 0` 处被 `Subscriber::take` 取用）。Subscriber 含 Cell 非 Sync，以 Mutex 包裹。
#[derive(Resource)]
pub struct BirdNestLazySubscriber(pub Mutex<Subscriber<DirtyTiles>>);

/// `birdnests_subscribe_to_tabloids`（Startup 第三成员；PDB 无独立符号，系统名串 0x143040d48；
/// FunctionSystem 的 `run_without_applying_deferred` 0x141cde5d0 内联了函数体）。
///
/// 机器码证据：参数 `(ResMut<RaycastWorld>, Commands)`（`validate_param_unsafe` 0x1422c2ea0 只校验
/// `ResMut<RaycastWorld>` 串 0x142f3317d）；体为
/// `commands.insert_resource(BirdNestLazySubscriber(raycast.subscribe_to_changes(1)))`——
/// `RaycastWorld::subscribe_to_changes`（0x1408cdc20）实参 `DL = 1`（槽 1），随后
/// `Commands::insert_resource`（类型元数据 0x143041ff8）。
pub fn birdnests_subscribe_to_tabloids(mut raycast: ResMut<RaycastWorld>, mut commands: Commands) {
    commands.insert_resource(BirdNestLazySubscriber(Mutex::new(raycast.subscribe_to_changes(1))));
}

/// add_birdnests（0x1422c9c70）：订阅脏格 → 每脏格 1 鸟巢候选（type 4）。
pub fn add_birdnests(
    mut state: ResMut<AutoClutterState>,
    dirty: ResMut<DirtyTilesResource>,
    sub: ResMut<BirdNestLazySubscriber>,
) {
    // O：Subscriber::take → 复制 0x200B 位集 → new_clear 合入；以合入位集为遍历源。
    // 原无资源位集回退：take 无结果即空位集（无脏格可处理）。
    let mut tiles = country_core::utils::dirty_tiles::DirtyTiles::new_clear();
    if let Some(reader) = sub.0.lock().unwrap().take() {
        tiles.extend(&*reader);
    }
    // O：cell→已处理去重集（HashMap<u64,bool>）。
    let mut processed: HashMap<u64, bool> = HashMap::new();
    for (x, y) in dirty_tiles_cells(&tiles) {
        let cell_key = crate::autoclutter::populate_autoclutter_regions::hash_cell_key(x, y);
        if processed.contains_key(&cell_key) {
            continue;
        }
        // O：closure_0（0x1422ca8c0）鸟巢几何/合法性门控——未逐行还原（U）。
        // 本重建按证据以 cell 中心作位置，门控恒通过（登记 U）。
        processed.insert(cell_key, true);
        let (wx, wz) = crate::autoclutter::populate_autoclutter_regions::cell_center_world(x, y);
        // 输出候选（O：+0x00=4、+0x08 Arc 强计数 LOCK++、位置 +0.15 偏移、+0x60=1.0）。
        state.spawn_candidates.push(AutoClutterSpawn {
            kind: 4,
            // +0x08 = Arc 强计数（O：LOCK 递增；u64 承载）。
            shared: SpawnShared::ArcStrong(cell_key),
            f18: 1,
            key: cell_key,
            marker: 5,
            position: glam::Vec3::new(wx, 0.0, wz + 0.15),
            host_id: cell_key,
            f48: cell_key,
            f50: 0,
            f54: 0,
            f58: 0,
            f5c: 0,
            scale: 1.0,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_ecs::system::RunSystemOnce;

    /// Startup 系统经 `Commands::insert_resource` 插入 `BirdNestLazySubscriber`（延迟到命令应用后才存在）。
    #[test]
    fn birdnests_subscribe_inserts_the_lazy_subscriber_resource() {
        let mut world = World::new();
        world.insert_resource(RaycastWorld::new());
        assert!(!world.contains_resource::<BirdNestLazySubscriber>());
        world.run_system_once(birdnests_subscribe_to_tabloids).unwrap();
        let sub = world.resource::<BirdNestLazySubscriber>();
        assert!(sub.0.lock().unwrap().take().is_none(), "订阅之前的发布不可见");
    }

    /// 缺 `RaycastWorld` 时参数校验失败，不插入资源。
    #[test]
    fn birdnests_subscribe_requires_the_host_raycast_world() {
        let mut world = World::new();
        assert!(world.run_system_once(birdnests_subscribe_to_tabloids).is_err());
        assert!(!world.contains_resource::<BirdNestLazySubscriber>());
    }
}
