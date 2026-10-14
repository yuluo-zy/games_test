//! 图调用辅助：保留顺序收集资源索引，不给通用集合实现另起逆向版本。
use super::{
    graph::{BuildError, RenderGraph},
    resource::ResourceHandle,
};
pub fn validate_handles(
    graph: &RenderGraph,
    handles: impl IntoIterator<Item = ResourceHandle>,
) -> Result<(), BuildError> {
    for handle in handles {
        graph.check_handle(handle)?;
    }
    Ok(())
}
pub fn checked_u16_index(index: usize) -> Result<u16, BuildError> {
    u16::try_from(index)
        .ok()
        .filter(|v| *v != u16::MAX)
        .ok_or(BuildError::ResourceLimit)
}
