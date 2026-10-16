//! 鸟点池（country-core `resources::critters::BirdSpotManager` 的消费侧模型）。
//! 证据（chapters/resources/critters.rs.md）：`gen_id`（0x140f86330）= +8 自增**返回旧值**；
//! `create_spots`（0x140f86340）= 池内复用/分配；`recycle`（0x1422f86e0）= 借用守卫后池回收。
//! 实际分配/释放由宿主（render_loop）持有；本模块记录计数语义。

use bevy_ecs::resource::Resource;


    /// 鸟点 id 生成器（O：+8 自增返回旧值）。
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub struct BirdSpotId(pub u64);

    /// 鸟点管理器（资源；池语义，布局 U）。
    #[derive(Resource, Debug, Default)]
    pub struct BirdSpotManager {
        next_id: u64,
        /// 回收池（`recycle` 0x1422f86e0：+0x10 原子借用守卫后池回收）。
        pool: Vec<BirdSpotId>,
    }

    impl BirdSpotManager {
        /// `gen_id`（0x140f86330）：`+8` 自增，返回旧值。
        pub fn gen_id(&mut self) -> BirdSpotId {
            let old = self.next_id;
            self.next_id += 8;
            BirdSpotId(old)
        }
        /// `recycle`（0x1422f86e0）：借用守卫后池回收（宿主在鸟点释放时调用）。
        pub fn recycle(&mut self, id: BirdSpotId) {
            self.pool.push(id);
        }
        /// 批量回收（屋顶移除路径）。
        pub fn recycle_all(&mut self) {
            self.pool.clear();
        }
        /// `create_spots`（0x140f86340）：池内复用/分配。
        pub fn create_spots(&mut self, count: usize) -> Vec<BirdSpotId> {
            (0..count)
                .map(|_| self.pool.pop().unwrap_or_else(|| self.gen_id()))
                .collect()
        }
    }
