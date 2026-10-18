//! `Vao` / `VaoBinding` / `build_vao`（layer A1，0x140909690，463 B）。
//!
//! O（指令级核对）：
//!  * 入参 `(device, &Mesh)`；索引缓冲 = `BufferDesc::new_gpu_only(indices.len()*4, INDEX_BUFFER(0x40))`，
//!    名称 `"Index buffer"`，初始数据 = 索引数组字节；
//!  * 顶点缓冲按属性逐个创建（`from_iter` 0x141309060 + 闭包 `call_once` 0x1408ed0e0）：
//!    `new_gpu_only(len*elem, VERTEX_BUFFER(0x80))`、名称 `"Vertex buffer"`、数据 = 属性字节，
//!    `VaoBinding { name: Arc<str>（属性名克隆）, buffer }`（56 字节）；
//!  * `Vao` = `{ bindings: Vec<VaoBinding> @0x00, index_buffer: Buffer @0x18 }`，共 0x40 字节。
//! 旧实现把入参当作 `HashMap<String, V>`、只建一个顶点缓冲且丢掉索引——与原版不符，已替换。

// Preserve the original instruction-level expression order/NaN policy, API call shapes, and evidence/test transcription. See README.md.
#![allow(clippy::doc_lazy_continuation, clippy::field_reassign_with_default)]
use std::sync::Arc;

use crate::render::mesh::{Mesh, VertexAttributeValues};
use crate::render::ssbo::rhapsody_boundary::{
    Buffer, BufferDesc, GraphicsDevice, slice_as_bytes, usage,
};

/// 一个顶点属性及其 GPU 缓冲（O：56 字节 = `Arc<str>`(16) + `Buffer`(40)）。
#[derive(Clone, Debug)]
pub struct VaoBinding {
    pub name: Arc<str>,
    pub buffer: Buffer,
}

#[derive(Debug)]
pub struct Vao {
    pub bindings: Vec<VaoBinding>,
    pub index_buffer: Buffer,
}

/// 属性的元素字节宽度与数据（O：按判别式 0..=4 的跳转表，宽度 4/4/8/12/16）。
/// 对 Float2/3/4 三种，原版在长度为 0 时走 `panic_bounds_check`，这里同样 panic。
fn attribute_bytes(attr: &VertexAttributeValues) -> &[u8] {
    use VertexAttributeValues as V;
    match attr {
        V::Float(v) => slice_as_bytes(v),
        V::Int(v) => slice_as_bytes(v),
        V::Float2(v) => {
            assert!(
                !v.is_empty(),
                "index out of bounds: the len is 0 but the index is 0"
            );
            slice_as_bytes(v)
        }
        V::Float3(v) => {
            assert!(
                !v.is_empty(),
                "index out of bounds: the len is 0 but the index is 0"
            );
            slice_as_bytes(v)
        }
        V::Float4(v) => {
            assert!(
                !v.is_empty(),
                "index out of bounds: the len is 0 but the index is 0"
            );
            slice_as_bytes(v)
        }
    }
}

/// `build_vao`（O）。创建失败时原版 `Result::unwrap` → panic。
pub fn build_vao(device: &dyn GraphicsDevice, mesh: &Mesh) -> Vao {
    try_build_vao(device, mesh).expect("创建网格 GPU 缓冲失败")
}

/// 分配失败时释放本次已创建的缓冲，不发布半成品 VAO。
pub fn try_build_vao(device: &dyn GraphicsDevice, mesh: &Mesh) -> anyhow::Result<Vao> {
    let largest_index = mesh.indices.iter().copied().max();
    for (name, attribute) in &mesh.attributes {
        anyhow::ensure!(attribute.array_length() > 0, "顶点属性 {name} 不能为空");
        anyhow::ensure!(
            largest_index.is_none_or(|index| (index as usize) < attribute.array_length()),
            "顶点属性 {name} 无法覆盖网格索引"
        );
    }
    let index_bytes = slice_as_bytes(&mesh.indices);
    let desc = BufferDesc::new_gpu_only(index_bytes.len(), usage::INDEX_BUFFER);
    let index_buffer = device.create_buffer(&desc, "Index buffer", Some(index_bytes))?;
    let mut bindings: Vec<VaoBinding> = Vec::with_capacity(mesh.attributes.len());
    for (name, attribute) in &mesh.attributes {
        let bytes = attribute_bytes(attribute);
        let desc = BufferDesc::new_gpu_only(bytes.len(), usage::VERTEX_BUFFER);
        match device.create_buffer(&desc, "Vertex buffer", Some(bytes)) {
            Ok(buffer) => bindings.push(VaoBinding {
                name: Arc::from(name.as_str()),
                buffer,
            }),
            Err(error) => {
                device.release_buffer(&index_buffer);
                for binding in bindings {
                    device.release_buffer(&binding.buffer);
                }
                return Err(error.into());
            }
        }
    }
    Ok(Vao {
        bindings,
        index_buffer,
    })
}

/// CPU 句柄本身不拥有设备资源，释放必须回到创建它的宿主表。
pub fn release_vao(device: &dyn GraphicsDevice, vao: &Vao) {
    for binding in &vao.bindings {
        device.release_buffer(&binding.buffer);
    }
    device.release_buffer(&vao.index_buffer);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::ssbo::tests::MockDevice;
    use glam::Vec3;

    fn mesh() -> Mesh {
        let mut m = Mesh::default();
        m.indices = vec![0, 1, 2, 2, 1, 3];
        m.attributes.insert(
            "Vertex_Position".into(),
            VertexAttributeValues::Float3(vec![Vec3::X, Vec3::Y, Vec3::Z, Vec3::ONE]),
        );
        m
    }

    #[test]
    fn index_and_vertex_buffers_are_created_with_original_flags() {
        let dev = MockDevice::default();
        let vao = build_vao(&dev, &mesh());
        let c = dev.created.borrow();
        assert_eq!(c.len(), 2);
        assert_eq!(
            (c[0].1.as_str(), c[0].0.size, c[0].0.usage, c[0].0.gpu_only),
            ("Index buffer", 24, 0x40, true)
        );
        assert_eq!(
            (c[1].1.as_str(), c[1].0.size, c[1].0.usage, c[1].0.gpu_only),
            ("Vertex buffer", 48, 0x80, true)
        );
        assert_eq!(vao.bindings.len(), 1);
        assert_eq!(&*vao.bindings[0].name, "Vertex_Position");
        assert_eq!(c[0].2.as_ref().unwrap().len(), 24);
    }

    #[test]
    fn layout_sizes() {
        assert_eq!(core::mem::size_of::<VaoBinding>(), 56);
        assert_eq!(core::mem::size_of::<Vao>(), 0x40);
    }
}
