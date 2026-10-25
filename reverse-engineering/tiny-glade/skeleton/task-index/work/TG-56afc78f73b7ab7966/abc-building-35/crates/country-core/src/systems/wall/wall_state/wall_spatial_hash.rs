//! 三版共享的优先建筑候选；implemented_unverified，验证暂停。
//! 原始函数/A/B/C地址见逐函数注释；完整身份与剩余边界见任务根mapping/CONTRACT。
//! 使用同一assembly crate的上批类型，原函数只保留一次实现；完整作者ABI未自动证明。
use glam::Vec2;
/// 原对象+72/+76是signed32网格宽/高，+80/+84是世界宽/高。
/// 不重写HashMap/空间桶，此结构只表达这两个游戏几何接口的已解析输入。
pub struct WallSpatialHash {
    pub grid_size: [i32; 2],
    pub world_size: Vec2,
}
impl WallSpatialHash {
    // ORIGINAL: country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::cell_size
    // 三版对应：A=0x1405d37b0；B=0x140aa65e0；C=0x140ac3070
    // 逻辑组：dd658ffbb342c1c6b5
    /// 各世界尺寸除以对应有符号网格尺寸；零/负值不替换，按浮点规则保留。
    pub fn cell_size(&self) -> Vec2 {
        Vec2::new(
            self.world_size.x / self.grid_size[0] as f32,
            self.world_size.y / self.grid_size[1] as f32,
        )
    }
    // ORIGINAL: country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::grid_coord_to_cell_index
    // 三版对应：A=0x1405d37a0；B=0x140aa65d0；C=0x140ac2d80
    // 逻辑组：eb963c0ed1fcb61c5e
    /// 原y*width+x在i32回绕后扩展i64；不固定width=64、不夹取grid坐标。
    pub fn grid_coord_to_cell_index(&self, grid: [i32; 2]) -> i64 {
        grid[1]
            .wrapping_mul(self.grid_size[0])
            .wrapping_add(grid[0]) as i64
    }
}
