//! 三版共享的优先建筑候选；implemented_unverified，验证暂停。
//! 原始函数/A/B/C地址见逐函数注释；完整身份与剩余边界见任务根mapping/CONTRACT。
//! 使用同一assembly crate的上批类型，原函数只保留一次实现；完整作者ABI未自动证明。
pub struct TransientDecoratorIdGen {
    pub source_word: u64,
    pub counter: u64,
    pub tag: u8,
}
impl TransientDecoratorIdGen {
    // ORIGINAL: country_core::resources::walls::decorator::TransientDecoratorIdGen::new
    // 三版对应：A=0x140597770；B=0x1408339c0；C=0x1408fb8a0
    // 逻辑组：b172f9392f59e3556a
    /// 原+0保存输入word、+8counter=0、+16保存tag；不自行产生第一条ID或推断递增规则。
    pub fn new(source_word: u64, tag: u8) -> Self {
        Self {
            source_word,
            counter: 0,
            tag,
        }
    }
}
/// 原构造短体只证64位寄存器恒等；保留word，不宣称和Info72的u32 ID同型。
pub struct DecoratorId(pub u64);
impl DecoratorId {
    // ORIGINAL: country_core::resources::walls::decorator::DecoratorId::new
    // 三版对应：A=0x14043c290；B=0x14067dd80；C=0x1406ca440
    // 逻辑组：06ef7a86823192f49d
    /// 不新增全局ID表，也不缩窄原输入。
    pub fn new(word: u64) -> Self {
        Self(word)
    }
}
pub struct DecoratorRank(pub u32);
impl DecoratorRank {
    // ORIGINAL: country_core::resources::walls::decorator::DecoratorRank::new
    // 三版对应：A=0x140593250；B=0x140833250；C=0x1408fb460
    // 逻辑组：a7cc60ea758f9f6036
    /// ECX->EAX原32位恒等封装，不猜rank上限。
    pub fn new(rank: u32) -> Self {
        Self(rank)
    }
}
