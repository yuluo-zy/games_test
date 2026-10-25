//! 图资源导入与导出入口。所有权标记传入已有资源注册表消费者。
use super::{
    graph::{BuildError, Descriptor, OriginSource, RenderGraph},
    resource::{ResourceHandle, ResourceOrigin},
};
impl RenderGraph {
    pub fn import_unowned(
        &mut self,
        name: impl Into<String>,
        descriptor: Descriptor,
        token: u64,
        initial_access: u8,
    ) -> Result<ResourceHandle, BuildError> {
        self.create_raw_resource(
            name,
            ResourceOrigin {
                descriptor,
                source: OriginSource::ImportUnowned {
                    token,
                    initial_access,
                },
            },
        )
    }
    pub fn import_owned(
        &mut self,
        name: impl Into<String>,
        descriptor: Descriptor,
        token: u64,
        initial_access: u8,
    ) -> Result<ResourceHandle, BuildError> {
        self.create_raw_resource(
            name,
            ResourceOrigin {
                descriptor,
                source: OriginSource::ImportOwned {
                    token,
                    initial_access,
                },
            },
        )
    }
}
