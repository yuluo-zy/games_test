//! uniform_binder.rs — UniformBinder（uniform 数据缓冲 + 绑定偏移表）。
//!
//! 机器码锚点：
//! - `UniformBinder::new` 0x14096b420（行 21）。
//! - `write_uniform<C,T>` 0x14096b570..0x14096bc10（行 37）。
//! - `write_optional_uniform<C,T>` 0x14096bd20..0x14096bf50（行 53）。

use alloc::vec::Vec;
use hashbrown::HashMap;

use crate::shader_pipeline_contract::BindingIndex;

/// uniform 绑定偏移表项。
#[derive(Clone, Copy, Debug)]
pub struct UniformBinding {
    pub offset: u32,
}

/// UniformBinder：uniform 数据缓冲（+0x00 ptr / +0x08 len）+ 绑定表（+0x30/+0x38 hashbrown）。
#[derive(Clone, Debug)]
pub struct UniformBinder {
    pub(crate) data: Vec<u8>,
    pub(crate) bindings: HashMap<BindingIndex, UniformBinding>,
}

impl UniformBinder {
    /// `new`（0x14096b420，行 21）：创建空数据缓冲 + 绑定表。
    pub fn new() -> UniformBinder {
        UniformBinder {
            data: Vec::new(),
            bindings: HashMap::new(),
        }
    }

    /// `write_uniform<C,T>`（行 37）：按绑定索引偏移把 `value` 写入数据缓冲。
    /// 守卫：data 非空 且 绑定表非空；越界 `slice_start_index_len_fail`。
    pub fn write_uniform<T: UniformBytes>(&mut self, binding: BindingIndex, value: &T) {
        if self.data.is_empty() || self.bindings.is_empty() {
            return;
        }
        let Some(b) = self.bindings.get(&binding) else {
            return;
        };
        let offset = b.offset as usize;
        assert!(offset + value.len() <= self.data.len(), "slice_start_index_len_fail");
        value.write_bytes(&mut self.data[offset..]);
    }

    /// `write_optional_uniform<C,T>`（行 53）：值由闭包派生（如纹理 extent）。
    pub fn write_optional_uniform<T: UniformBytes>(&mut self, binding: BindingIndex, value: Option<T>) {
        if let Some(value) = value {
            self.write_uniform(binding, &value);
        }
    }

    /// 登记绑定（UniformBinder::new 后由 pass 构建填表）。
    pub(crate) fn insert_binding(&mut self, binding: BindingIndex, offset: u32) {
        self.bindings.insert(binding, UniformBinding { offset });
    }
}

impl Default for UniformBinder {
    fn default() -> Self {
        Self::new()
    }
}

/// 可写 uniform 的值。
pub trait UniformBytes {
    fn len(&self) -> usize;
    fn write_bytes(&self, dst: &mut [u8]);
}

impl UniformBytes for f32 {
    fn len(&self) -> usize {
        4
    }
    fn write_bytes(&self, dst: &mut [u8]) {
        dst[..4].copy_from_slice(&self.to_le_bytes());
    }
}

impl UniformBytes for u32 {
    fn len(&self) -> usize {
        4
    }
    fn write_bytes(&self, dst: &mut [u8]) {
        dst[..4].copy_from_slice(&self.to_le_bytes());
    }
}

impl UniformBytes for [u32; 2] {
    fn len(&self) -> usize {
        8
    }
    fn write_bytes(&self, dst: &mut [u8]) {
        for (i, v) in self.iter().enumerate() {
            dst[i * 4..i * 4 + 4].copy_from_slice(&v.to_le_bytes());
        }
    }
}
