//! 描述符精确匹配与后进先出复用的实现位于注册表模块。
//! 此模块提供帧退役后的所有权归还入口；导出资源不归还资源池。
use super::graph::{Exports, Pool, RegistryResource};
pub use super::resource_registry::ResourcePool;
pub fn recycle_exports(
    pool: &mut Pool,
    exports: Exports,
) -> std::collections::HashMap<u16, RegistryResource<u64, u64, ()>> {
    for resource in exports.remaining {
        match resource {
            RegistryResource::OwnedTexture(texture) => pool.insert_texture(texture),
            RegistryResource::OwnedBuffer(buffer) => pool.insert_buffer(buffer),
            _ => {}
        }
    }
    exports.exported
}
