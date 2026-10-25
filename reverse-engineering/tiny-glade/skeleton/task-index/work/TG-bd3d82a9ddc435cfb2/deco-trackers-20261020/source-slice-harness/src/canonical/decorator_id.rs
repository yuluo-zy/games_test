/// B DecoratorId：装饰唯一 id。serde = u32 十进制化（newtype serde，0x1402ddd20）。
/// `new` 0x14067dd80 = 恒等返回；`Display` 0x1408331d0。
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize,
)]
pub struct DecoratorId(pub u32);

impl DecoratorId {
    /// 0x14067dd80：恒等返回（也复用为 DeferredDecoratorOpEvents::events）。
    #[inline]
    pub fn new(id: u32) -> Self {
        Self(id)
    }

    /// 行 166（内联）。
    #[inline]
    pub fn get_inner(self) -> u32 {
        self.0
    }
}

impl std::fmt::Display for DecoratorId {
    /// 0x1408331d0：十进制 u32 输出。
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

