//! 原 PDB 的 leek 模块存在错误格式化与转换函数，完整枚举字段尚未恢复。
//! 候选公开错误包装保留来源链，避免将原未知错误标签解释为固定消息。
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
