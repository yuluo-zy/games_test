//! Graph build errors and their conversions.

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BuildError {
    WrongGraph {
        expected: super::super::resource::GraphId,
        actual: super::super::resource::GraphId,
    },
    Contract(super::super::resource::ResourceContractError),
    IncompatibleAccess(super::super::access_ledger::IncompatibleAccess),
    ResourceLimit,
    InvalidHandle(u32),
    ResourceKindMismatch(u32),
    AccessLimit,
    MissingTemporal(String),
    DuplicateResourceName(String),
    AliasedImport(u64),
    ResourceCreationFailed(String),
    BackendFailure(crate::error::SharedError),
    /// pass 数超过生命周期事件的打包域（pass 下标左移 17 位存入 u32，见 `MAX_PASSES`）。
    PassLimit,
}

impl std::fmt::Display for BuildError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "构图失败: {self:?}")
    }
}

impl std::error::Error for BuildError {}

impl From<crate::Error> for BuildError {
    fn from(error: crate::Error) -> Self {
        Self::BackendFailure(error.into())
    }
}

impl From<BuildError> for crate::Error {
    fn from(error: BuildError) -> Self {
        match error {
            BuildError::BackendFailure(error) => error.into_owned(),
            other => Self::invalid_usage_with("graph declaration", other),
        }
    }
}
