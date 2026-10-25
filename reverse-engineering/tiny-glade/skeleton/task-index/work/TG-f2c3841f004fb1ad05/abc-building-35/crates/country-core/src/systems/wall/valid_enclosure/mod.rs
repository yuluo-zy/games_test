//! 三版共享的优先建筑候选；implemented_unverified，验证暂停。
//! 原始函数/A/B/C地址见逐函数注释；完整身份与剩余边界见任务根mapping/CONTRACT。
//! 使用同一assembly crate的上批类型，原函数只保留一次实现；完整作者ABI未自动证明。
use glam::Vec2;
/// 读取原tag及其选中payload的借用投影；不复造完整作者ShapeParameters枚举。
pub struct ShapeParameters<'a> {
    pub observed_record: &'a [u8],
}
impl ShapeParameters<'_> {
    // ORIGINAL: country_core::systems::wall::valid_enclosure::ShapeParameters::center
    // 三版对应：A=0x1406b0920；B=0x1411ff2a0；C=0x140a623d0
    // 逻辑组：178fa353001289e6ac
    /// 原索引为4+u32(tag)*8，读取两f32；tag0用+4/+8，tag1用+12/+16。
    /// 调用方必须提供合法已初始化记录；不将未知tag当默认圆或矩形。
    pub fn center(&self) -> Vec2 {
        let tag = u32::from_le_bytes(self.observed_record[..4].try_into().unwrap());
        let offset = 4 + tag as usize * 8;
        let x = f32::from_le_bytes(self.observed_record[offset..offset + 4].try_into().unwrap());
        let y = f32::from_le_bytes(
            self.observed_record[offset + 4..offset + 8]
                .try_into()
                .unwrap(),
        );
        Vec2::new(x, y)
    }
}
