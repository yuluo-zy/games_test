//! 三版共享的优先建筑候选；implemented_unverified，验证暂停。
//! 原始函数/A/B/C地址见逐函数注释；完整身份与剩余边界见任务根mapping/CONTRACT。
//! 使用同一assembly crate的上批类型，原函数只保留一次实现；完整作者ABI未自动证明。
use std::slice;
/// 20B原record的语义字段。padding不补默认；实际owner应提供已初始化记录。
pub struct FinalizedWallHole {
    pub aabb: [f32; 4],
    pub kind: u8,
    pub origin: u8,
}
/// Vec/std内部不重写；借用owner的已最终化数组和原+24的wall_length。
pub struct FinalizedWallHoles<'a> {
    pub records: &'a [FinalizedWallHole],
    pub length: f32,
}
impl<'a> FinalizedWallHoles<'a> {
    // ORIGINAL: country_core::resources::walls::wall_holes::FinalizedWallHoles::wall_length
    // 三版对应：A=0x14061caf0；B=0x140f86cf0；C=0x140c4e280
    // 逻辑组：7cd32d6c912b8c59f3
    /// 原 getter直接读+24；不增加phase检查，phase由真实owner保障。
    pub fn wall_length(&self) -> f32 {
        self.length
    }
    // ORIGINAL: country_core::resources::walls::wall_holes::FinalizedWallHoles::holes_no_duplicates
    // 三版对应：A=0x140628e90；B=0x140f86cd0；C=0x140c4e290
    // 逻辑组：6f1520413bea17cf2a
    /// 短体构造ptr..ptr+20*len；closure消费证据补自三版iter_holes_with_padding的closed分支。
    /// 仅保留origin==0的主记录，不按AABB/ID HashSet去重，不重新排序。
    /// 这里只还原无重复迭代器，不实现padding/周期复制的完整消费者。
    pub fn holes_no_duplicates(&self) -> impl Iterator<Item = &'a FinalizedWallHole> {
        self.records.iter().filter(|hole| hole.origin == 0)
    }
    /// holes和holes_no_duplicates构造体共用原地址；未过滤别名须保持全部记录。
    pub fn holes(&self) -> slice::Iter<'a, FinalizedWallHole> {
        self.records.iter()
    }
}
/// 原newtype getter指针恒等；不猜内部存储全集。
#[repr(transparent)]
pub struct FinalizedHoleStorage<T>(pub T);
impl<T> FinalizedHoleStorage<T> {
    // ORIGINAL: country_core::resources::walls::wall_holes::FinalizedHoleStorage::inner
    // 三版对应：A=0x14043c290；B=0x14067dd80；C=0x1406ca440
    // 逻辑组：79616daf7021a80e26
    /// 返回内部原有对象引用，不分配或克隆。
    pub fn inner(&self) -> &T {
        &self.0
    }
}
pub struct InProgressHoleStorage<'a, T> {
    pub records: &'a [T],
}
impl<'a, T> InProgressHoleStorage<'a, T> {
    // ORIGINAL: country_core::resources::walls::wall_holes::InProgressHoleStorage::holes
    // 三版对应：A=0x1406299f0；B=0x140c37760；C=0x140698b50
    // 逻辑组：fbb822059231ebcdfa
    /// 原ptr/len借用，元素型由raw owner提供，不把它当Finalized record。
    pub fn holes(&self) -> &'a [T] {
        self.records
    }
}
