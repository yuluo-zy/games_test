//! Execution errors.
use crate::command::recording;
use crate::rg::recorder as record;
use super::GraphExecution;

#[derive(Debug)]
pub enum Error {
    /// Some batches succeeded. The caller must retain the execution until its
    /// completion set is reached; error cleanup never blocks or rolls back GPU work.
    PartialSubmission {
        original: Box<Error>,
        execution: Box<GraphExecution>,
    },
    /// 并行录制的分块或规划无效，或后端未提供并行视图。
    ChunkPlan,
    CallbackPanic,
    Begin(recording::BeginFailure),
    Lifecycle(recording::Failure),
    Record(record::Outcome),
    MissingCommand(u64),
    EmptyCommands,
    InvalidLastState,
    Compile(crate::Error),
    ResourceUnavailable(usize),
    CallbackCount,
    ManagedResourceUnresolved(usize),
    QueueUnavailable(recording::Queue),
    InvalidSynchronization,
    UnsupportedQueueDomainSwitch,
    ResourceCountMismatch {
        expected: usize,
        actual: usize,
    },
    /// 调用方对编译结果的校验失败（如 temporal 导出映射）；发生在任何命令开始之前。
    Validation(crate::Error),
}

impl From<Error> for crate::Error {
    fn from(error: Error) -> Self {
        match error {
            Error::PartialSubmission { original, .. } => (*original).into(),
            Error::Validation(error) | Error::Compile(error) => error,
            other => Self::invalid_usage_with("图执行失败", other),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}
