//! vao.rs — `Vao` / `VaoBinding` / `build_vao`.
//! O/I/U tags per `contracts/vao.md`.

use core::ops::Range;

use crate::render::ssbo::rhapsody_boundary::{self, Buffer, BufferDesc, GraphicsDevice};

/// WP4-owned (`render::mesh`) boundary for vertex attribute values.
/// `VaoBinding` field layout is owned with WP4; WP2 only restores `build_vao` orchestration.
pub trait VertexAttributeValues: Sized {
    /// O dep — `VertexAttributeValues::size_in_bytes`.
    fn size_in_bytes(&self) -> usize;
    fn as_bytes(&self) -> &[u8];
}

/// O — one vertex-attribute binding (layout owned with WP4 mesh).
#[derive(Debug)]
pub struct VaoBinding {
    /// U — exact fields not recovered (WP4 mesh contract).
    pub _gap: u64,
}

/// `Vao` — 64 bytes (8 qwords). O from `build_vao` 0x140909690.
#[derive(Debug)]
pub struct Vao {
    /// O — GPU vertex buffer handle (rhapsody Buffer).
    pub buffer: Buffer,
    /// O — Vec<VaoBinding> (ptr, len, cap).
    pub bindings: alloc::vec::Vec<VaoBinding>,
    /// U — typed gaps (plausibly vertex count / stride / attribute meta; not observed).
    pub _gap6: u64,
    pub _gap7: u64,
    pub _gap8: u64,
}

/// O — `build_vao` 0x140909690: create a GPU-only vertex buffer, then map each named
/// attribute to a `VaoBinding`, return the `Vao`.
pub fn build_vao<V: VertexAttributeValues>(
    attributes: &std::collections::HashMap<alloc::string::String, V>,
    device: &dyn GraphicsDevice,
) -> Vao {
    // GPU-only vertex buffer for the attribute data.
    let total_bytes: usize = attributes.values().map(|v| v.size_in_bytes()).sum();
    let mut desc = BufferDesc::new_gpu_only(total_bytes);
    desc.alignment();
    let buffer = match device.create_buffer(&desc) {
        rhapsody_boundary::GpuResult::Ok((b, _)) => b,
        rhapsody_boundary::GpuResult::Err => panic!("create_buffer failed"),
    };

    // Vec<VaoBinding> built from the attribute map (O: from_iter<Map<Iter, closure>>).
    let mut bindings: alloc::vec::Vec<VaoBinding> = alloc::vec::Vec::with_capacity(attributes.len());
    let mut byte_offset = 0usize;
    for (name, attr) in attributes {
        let _name = name;
        let bytes = attr.as_bytes();
        let stride = attr.size_in_bytes();
        let _range: Range<usize> = byte_offset..byte_offset + stride;
        byte_offset += stride;
        // VaoBinding content is WP4-owned (U here).
        let _ = bytes.len();
        bindings.push(VaoBinding { _gap: 0 });
    }

    Vao {
        buffer,
        bindings,
        _gap6: 0,
        _gap7: 0,
        _gap8: 0,
    }
}

/// I — inlined constructor.
pub struct VaoNew; // placeholder for `Vao::new` (not recovered)
