//! 原 PDB 的 leek 模块存在错误格式化与转换函数，完整枚举字段尚未恢复。
//! 候选公开错误包装保留来源链，避免将原未知错误标签解释为固定消息。
use std::time::Duration;
#[derive(Debug)]
pub struct Error {
    pub context: String,
    pub source: Box<dyn std::error::Error + Send + Sync>,
}
impl Error {
    pub fn new(
        context: impl Into<String>,
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self {
            context: context.into(),
            source: Box::new(source),
        }
    }
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.context, self.source)
    }
}
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self.source.as_ref())
    }
}

/// 原 leek::wait_submits_done：有界等待全部在途提交完成。
/// 返回 true 表示在 timeout 内完成，false 表示超时；不弹出或改写任何队列状态。
pub fn wait_submits_done(
    mut poll: impl FnMut(Duration) -> bool,
    timeout: Duration,
) -> bool {
    let mut waited = Duration::ZERO;
    while !poll(Duration::from_millis(1)) {
        waited += Duration::from_millis(1);
        if waited >= timeout {
            return false;
        }
        std::thread::sleep(Duration::from_millis(1));
    }
    true
}
