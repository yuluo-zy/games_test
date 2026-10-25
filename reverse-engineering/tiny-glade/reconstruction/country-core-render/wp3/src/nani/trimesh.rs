//! trimesh.rs — TrimeshBuilder（WP3）。
//!
//! 机器码锚点（B patched EXE）：
//! - `TrimeshBuilder::positions` 0x14129e890（行 ~9）。
//! - `TrimeshBuilder::attrib<Vec3>` 0x1412a90c0（行 ~29，412B）。
//! 完整契约见 contracts/trimesh.md。

#![allow(dead_code, unused_imports)]

use alloc::vec::Vec;
use glam::Vec3;

/// 属性记录（0x18B，O）：`{ name: &str(8B), byte_len: u64, element_size: u16, element_type: u8, pad }`。
#[derive(Clone, Debug)]
struct Attrib<'a> {
    name: &'a str,
    byte_len: usize,
    /// `<Vec3>` 为 0xc（12B）。
    element_size: u16,
    /// `<Vec3>` 为 4（O；与上游 mesh 属性枚举的映射 U）。
    element_type: u8,
}

/// 三角网格构建器（O）。
#[derive(Default)]
pub struct TrimeshBuilder {
    /// positions 已写标志（`*self = 1`；O）。
    positions_written: bool,
    /// 属性列表（O：`RawVec::grow_one<( &str, VertexAttributeValues )>`）。
    attribs: Vec<Attrib<'static>>,
    /// 期望顶点数（首个 attrib 前为 0 不校验；O）。
    expected_vertex_count: usize,
    /// 顶点数（首个 attrib 设值，后续必须相等；O `assert_failed<usize,usize>`）。
    vertex_count: usize,
}

impl TrimeshBuilder {
    /// 写入 positions 属性（0x14129e890，O）：置标志 + `attrib<Vec3>`。
    pub fn positions(&mut self, positions: &[Vec3]) -> &mut Self {
        self.positions_written = true;
        self.attrib("positions", positions);
        self
    }

    /// 写一个属性（0x1412a90c0，O）：
    /// 1. `data` 空 → panic；`data.len() < expected_vertex_count` → panic。
    /// 2. `vertex_count` 未定则取 `data.len()`，否则必须相等。
    /// 3. 记录 `{name, data.len()*element_size, element_size, element_type}` push。
    pub fn attrib<T>(&mut self, name: &str, data: &[T]) -> &mut Self {
        let element_size = core::mem::size_of::<T>();
        if data.is_empty() {
            panic!("attrib: empty data");
        }
        if self.expected_vertex_count != 0 && data.len() > self.expected_vertex_count {
            panic!("attrib: data longer than expected vertex count");
        }
        if self.vertex_count == 0 {
            self.vertex_count = data.len();
        } else {
            assert_eq!(self.vertex_count, data.len(), "attrib: vertex count mismatch");
        }
        self.attribs.push(Attrib {
            name,
            byte_len: data.len() * element_size,
            element_size: element_size as u16,
            element_type: 4, // O for Vec3
        });
        self
    }
}
