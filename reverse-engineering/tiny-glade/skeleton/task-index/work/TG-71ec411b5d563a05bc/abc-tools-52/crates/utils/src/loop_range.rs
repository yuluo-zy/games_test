//! A/B/C 三版共享工具候选，只恢复本文件列出的观察函数。
//! 状态：implemented_unverified；按用户此前要求，构建/测试/原码执行继续暂停。
//! 此文件是原指令语义的 Rust 候选，不是取回的作者源码。
//! 非完整类型标为投影；未声称原 Rust 签名、ABI 或完整系统已还原。
//! 完整身份、指令、版本对应及消费者见本任务 work 根目录 mapping.json / CONTRACT.md。
/// 原生 +0/+4 是有符号逻辑起止索引，+8 随 LoopIdx 一起返回。
/// 第三字段的作者名称未知；不把它猜作已取模的长度。
#[derive(Clone, Copy, Debug)]
pub struct LoopRange {
    pub start_idx: i32,
    pub end_idx: i32,
    pub context: u32,
}
#[derive(Clone, Copy, Debug)]
pub struct LoopIdx {
    pub idx: i32,
    pub context: u32,
}
impl LoopRange {
    // ORIGINAL: utils::loop_range::LoopRange::start
    // 三版对应：A=0x140cb3c30；B=0x141200370；C=0x141405270
    // 逻辑组：500c28bf1892ecc574
    /// 同时返回起点和上下文；不是只返回一个整数。
    pub fn start(&self) -> LoopIdx {
        LoopIdx {
            idx: self.start_idx,
            context: self.context,
        }
    }
    // ORIGINAL: utils::loop_range::LoopRange::end
    // 三版对应：A=0x140cb3c40；B=0x141200380；C=0x141405250
    // 逻辑组：313af8f18830e83c0a
    /// 终点读取 +4，上下文读取 +8。
    pub fn end(&self) -> LoopIdx {
        LoopIdx {
            idx: self.end_idx,
            context: self.context,
        }
    }
    // ORIGINAL: utils::loop_range::LoopRange::is_empty
    // 三版对应：A=0x140cb3c20；B=0x141200360；C=0x141405280
    // 逻辑组：8b28ddbea9344eae30
    /// 原码仅判断相等；逆序区间不在此处判空。
    pub fn is_empty(&self) -> bool {
        self.end_idx == self.start_idx
    }
    // ORIGINAL: utils::loop_range::impl$3::next_back
    // 三版对应：A=0x140cb3be0；B=0x141200320；C=0x1414051c0
    // 逻辑组：a843960aacc0efb2f3
    /// 原码使用有符号比较；成功时先减 end，再返回索引和上下文。
    /// None 不写原生 Option 的未初始化 payload，Rust 使用安全 Option 表达。
    pub fn next_back(&mut self) -> Option<LoopIdx> {
        if self.start_idx >= self.end_idx {
            return None;
        }
        self.end_idx = self.end_idx.wrapping_sub(1);
        Some(LoopIdx {
            idx: self.end_idx,
            context: self.context,
        })
    }
}
impl LoopIdx {
    // ORIGINAL: utils::loop_range::LoopIdx::raw_idx
    // 三版对应：A=0x14049d3c0；B=0x140bece40；C=0x1400ee920
    // 逻辑组：1fb2d0e790331236cf
    /// 原码直接返回低 32 位；保留有符号索引语义。
    pub fn raw_idx(&self) -> i32 {
        self.idx
    }
    // ORIGINAL: utils::loop_range::impl$1::sub
    // 三版对应：A=0x140cb3ba0；B=0x1412002e0；C=0x141405490
    // 逻辑组：a130406276b5bac2b3
    /// 先做 32 位回绕减法，再符号扩展成 64 位，不是 64 位直接相减。
    pub fn sub(self, rhs: Self) -> i64 {
        self.idx.wrapping_sub(rhs.idx) as i64
    }
}
