//! A/B/C 三版共享工具候选，只恢复本文件列出的观察函数。
//! 状态：implemented_unverified；按用户此前要求，构建/测试/原码执行继续暂停。
//! 此文件是原指令语义的 Rust 候选，不是取回的作者源码。
//! 非完整类型标为投影；未声称原 Rust 签名、ABI 或完整系统已还原。
//! 完整身份、指令、版本对应及消费者见本任务 work 根目录 mapping.json / CONTRACT.md。
/// 原名对象的已解析借用；allocation 包含 16B header，byte_len 来自原 +8。
/// 这是借用视图，不实现 Arc，也不猜字符串编码/原所有权。
pub struct PrefabName<'a> {
    pub allocation: &'a [u8],
    pub byte_len: usize,
}
impl<'a> PrefabName<'a> {
    // ORIGINAL: asset::prefab::PrefabName::as_name_ref
    // 三版对应：A=0x141024ed0；B=0x141556130；C=0x14181c960
    // 逻辑组：4a823a0286d2cad82b
    /// 原首指针加 16、长度不变。有效输入须满足 allocation.len()>=16+byte_len。
    /// 返回借用字节；转 UTF-8 字符串的类型证明留给调用方。
    pub fn as_name_ref(&self) -> &'a [u8] {
        &self.allocation[16..][..self.byte_len]
    }
    // ORIGINAL: asset::prefab::impl$5::as_ref
    // 三版对应：A=0x141024ed0；B=0x141556130；C=0x14181c960
    // 逻辑组：379ba315e0702f9717
    /// 与 as_name_ref 三版均共用同一地址，复用一次已恢复实现。
    pub fn as_ref(&self) -> &'a [u8] {
        self.as_name_ref()
    }
}
// ORIGINAL: asset::prefab::impl$4::as_ref
// 三版对应：A=0x14043c290；B=0x14067dd80；C=0x1406ca440
// 逻辑组：0eac107991895c74ce
/// 原生 rcx->rax 指针恒等；具体 AsRef 作者目标类型未证，保留泛型借用边界。
pub fn as_ref_identity<T: ?Sized>(value: &T) -> &T {
    value
}
