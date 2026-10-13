//! uniform_binder.rs — UniformBinder（uniform 块的映射内存 + 按名字哈希写入）。
//!
//! 机器码锚点：
//! - `UniformBinder::new` 0x14096b420（324 B）。
//! - `write_uniform<C,T>` 0x14096b570..0x14096bc10（9 个实例：u32/i32/f32/bool/[u32;2]/Vec2/Vec3/Vec4/Mat4，
//!   227..271 B，逻辑同构，仅写入宽度不同）。
//! - `write_optional_uniform<C,T>` 0x14096bd20 / 0x14096be40 / 0x14096bf50（3 个实例，272..283 B）。
//!
//! 观测（O）：
//!  * `new(out, device, reflection, dynamic_constants)`：`reflection` 里 `+0xd8` 为 0（着色器无 uniform 块）→
//!    直接返回 `None`（在 `out+0x30` 写 niche 标签 0xa）；否则
//!    `DynamicConstants::allocate(dyn_const, device, size = reflection[+0x128], 0)` →
//!    `DynamicConstantsChunk::into_uniform_buffer(device)`，并记录 `reflection[+0x130]`（set/绑定号）到 `out+0x28`。
//!  * `write_uniform`：守卫 ① 反射的 uniform 描述（`reflection+0xe8` 处结构）首字非 0；② 其字段表项数
//!    （`+0x48`）非 0；③ 以名字哈希（64 位，直接用作 hashbrown 哈希）查表，命中取 u32 偏移；
//!    ④ `chunk_len < offset` → `core::slice::index::slice_start_index_len_fail`；⑤ 在 `chunk_ptr + offset`
//!    写值。任何守卫失败都**静默返回**。bool 写成 `u32`（`movzx eax, r9b; mov [..], eax`）。
//!  * `write_optional_uniform`：同样的守卫与查表，但**值由闭包惰性求出**（只在命中后调用，例如读纹理 extent），
//!    返回 `bool`：写了 = true。
//!
//! 偏离：`DynamicConstants` / `GraphicsDevice` 为外部库，经 [`UniformAllocator`] 接缝；`chunk` 用 `Vec<u8>` 代表
//! 映射内存。键由旧版的 `BindingIndex(u32)` 改为 u64 名字哈希（O：查表比较 `cmp [..], r8` 是 64 位）。

use std::collections::HashMap;

/// 着色器反射里的 uniform 块信息（`reflection[+0xd8 != 0]`、`+0x128` 大小、`+0x130` 绑定号）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UniformBlockInfo {
    /// `+0x128`：块字节数。
    pub size: u32,
    /// `+0x130`：记入 `UniformBinder+0x28` 的 u32。
    pub set_index: u32,
}

/// uniform 字段反射（`pipeline+0xe8` 处结构）：名字哈希 → 字节偏移。
#[derive(Clone, Debug, Default)]
pub struct UniformReflection {
    /// 首字非 0（O：`cmp qword ptr [rdx], 0`）。
    pub valid: bool,
    /// hashbrown 表（`+0x30` 控制字节 / `+0x38` 掩码 / `+0x48` 项数），键为 64 位名字哈希，值为 u32 偏移。
    pub fields: HashMap<u64, u32>,
}

impl UniformReflection {
    /// 守卫 ①② 后查表；任一守卫失败 → `None`。
    fn offset_of(&self, key: u64) -> Option<u32> {
        if !self.valid || self.fields.is_empty() {
            return None;
        }
        self.fields.get(&key).copied()
    }
}

/// `DynamicConstants::allocate` + `DynamicConstantsChunk::into_uniform_buffer` 的外部库接缝。
pub trait UniformAllocator {
    /// 分配 `size` 字节的逐帧 uniform 内存（映射内存）。
    fn allocate_uniform(&mut self, size: u32) -> Vec<u8>;
}

/// UniformBinder：一块 uniform 映射内存 + set/绑定号。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UniformBinder {
    /// 映射内存（`+0x00` ptr / `+0x08` len）。
    pub(crate) chunk: Vec<u8>,
    /// `+0x28`：反射 `+0x130`。
    pub(crate) set_index: u32,
}

impl UniformBinder {
    /// `new`（0x14096b420）：着色器没有 uniform 块 → `None`。
    pub fn new(block: Option<UniformBlockInfo>, allocator: &mut dyn UniformAllocator) -> Option<UniformBinder> {
        let block = block?;
        let chunk = allocator.allocate_uniform(block.size);
        Some(UniformBinder {
            chunk,
            set_index: block.set_index,
        })
    }

    /// 映射内存只读视图。
    pub fn data(&self) -> &[u8] {
        &self.chunk
    }

    /// `write_uniform<C,T>`（0x14096b570..）：按名字哈希把 `value` 写入映射内存；守卫失败静默返回。
    pub fn write_uniform<T: UniformBytes>(&mut self, reflection: &UniformReflection, key: u64, value: T) {
        let Some(offset) = reflection.offset_of(key) else {
            return;
        };
        self.write_at(offset, &value);
    }

    /// `write_optional_uniform<C,T>`（0x14096bd20..）：值由闭包惰性求出（命中后才调用）；返回是否写入。
    pub fn write_optional_uniform<T: UniformBytes>(
        &mut self,
        reflection: &UniformReflection,
        key: u64,
        value: impl FnOnce() -> T,
    ) -> bool {
        let Some(offset) = reflection.offset_of(key) else {
            return false;
        };
        self.write_at(offset, &value());
        true
    }

    /// ④⑤：起点越界 → `slice_start_index_len_fail`；写入越出末尾由 `copy_from_slice` 的长度检查拦下
    /// （原版此处为无检查写，I）。
    fn write_at(&mut self, offset: u32, value: &impl UniformBytes) {
        let offset = offset as usize;
        let len = self.chunk.len();
        if len < offset {
            panic!("range start index {offset} out of range for slice of length {len}");
        }
        let dst = &mut self.chunk[offset..];
        value.write_bytes(&mut dst[..value.byte_len()]);
    }
}

/// 可写 uniform 的值（对应 9 个 `write_uniform<_, T>` 实例）。
pub trait UniformBytes {
    /// 写入宽度（字节）。
    fn byte_len(&self) -> usize;
    /// `dst.len() == byte_len()`。
    fn write_bytes(&self, dst: &mut [u8]);
}

fn put_f32s(dst: &mut [u8], values: &[f32]) {
    for (chunk, v) in dst.chunks_exact_mut(4).zip(values) {
        chunk.copy_from_slice(&v.to_le_bytes());
    }
}

impl UniformBytes for f32 {
    fn byte_len(&self) -> usize {
        4
    }
    fn write_bytes(&self, dst: &mut [u8]) {
        dst.copy_from_slice(&self.to_le_bytes());
    }
}

impl UniformBytes for u32 {
    fn byte_len(&self) -> usize {
        4
    }
    fn write_bytes(&self, dst: &mut [u8]) {
        dst.copy_from_slice(&self.to_le_bytes());
    }
}

impl UniformBytes for i32 {
    fn byte_len(&self) -> usize {
        4
    }
    fn write_bytes(&self, dst: &mut [u8]) {
        dst.copy_from_slice(&self.to_le_bytes());
    }
}

/// O：`movzx eax, r9b; mov dword ptr [..], eax` —— bool 以 u32 写入。
impl UniformBytes for bool {
    fn byte_len(&self) -> usize {
        4
    }
    fn write_bytes(&self, dst: &mut [u8]) {
        dst.copy_from_slice(&(*self as u32).to_le_bytes());
    }
}

impl UniformBytes for [u32; 2] {
    fn byte_len(&self) -> usize {
        8
    }
    fn write_bytes(&self, dst: &mut [u8]) {
        for (chunk, v) in dst.chunks_exact_mut(4).zip(self) {
            chunk.copy_from_slice(&v.to_le_bytes());
        }
    }
}

impl UniformBytes for glam::Vec2 {
    fn byte_len(&self) -> usize {
        8
    }
    fn write_bytes(&self, dst: &mut [u8]) {
        put_f32s(dst, &self.to_array());
    }
}

/// O：Vec3 实例写 12 字节（`mov [..+8], eax; mov [..], rax`）。
impl UniformBytes for glam::Vec3 {
    fn byte_len(&self) -> usize {
        12
    }
    fn write_bytes(&self, dst: &mut [u8]) {
        put_f32s(dst, &self.to_array());
    }
}

impl UniformBytes for glam::Vec4 {
    fn byte_len(&self) -> usize {
        16
    }
    fn write_bytes(&self, dst: &mut [u8]) {
        put_f32s(dst, &self.to_array());
    }
}

impl UniformBytes for glam::Mat4 {
    fn byte_len(&self) -> usize {
        64
    }
    fn write_bytes(&self, dst: &mut [u8]) {
        put_f32s(dst, &self.to_cols_array());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Pool {
        asked: Vec<u32>,
    }
    impl UniformAllocator for Pool {
        fn allocate_uniform(&mut self, size: u32) -> Vec<u8> {
            self.asked.push(size);
            vec![0xAA; size as usize]
        }
    }

    fn refl(fields: &[(u64, u32)]) -> UniformReflection {
        UniformReflection {
            valid: true,
            fields: fields.iter().copied().collect(),
        }
    }

    fn binder(size: u32) -> UniformBinder {
        UniformBinder::new(
            Some(UniformBlockInfo { size, set_index: 3 }),
            &mut Pool { asked: vec![] },
        )
        .unwrap()
    }

    #[test]
    fn new_without_uniform_block_is_none_and_does_not_allocate() {
        let mut pool = Pool { asked: vec![] };
        assert!(UniformBinder::new(None, &mut pool).is_none());
        assert!(pool.asked.is_empty());
    }

    #[test]
    fn new_allocates_reflection_size_and_keeps_set_index() {
        let mut pool = Pool { asked: vec![] };
        let b = UniformBinder::new(Some(UniformBlockInfo { size: 96, set_index: 7 }), &mut pool).unwrap();
        assert_eq!(pool.asked, [96]);
        assert_eq!(b.set_index, 7);
        assert_eq!(b.data().len(), 96);
    }

    #[test]
    fn write_widths_match_the_nine_instances() {
        let r = refl(&[(1, 0)]);
        let cases: [(usize, Box<dyn Fn(&mut UniformBinder)>); 9] = [
            (4, Box::new(|b| b.write_uniform(&r_of(), 1, 1u32))),
            (4, Box::new(|b| b.write_uniform(&r_of(), 1, -1i32))),
            (4, Box::new(|b| b.write_uniform(&r_of(), 1, 1.0f32))),
            (4, Box::new(|b| b.write_uniform(&r_of(), 1, true))),
            (8, Box::new(|b| b.write_uniform(&r_of(), 1, [1u32, 2u32]))),
            (8, Box::new(|b| b.write_uniform(&r_of(), 1, glam::Vec2::new(1.0, 2.0)))),
            (
                12,
                Box::new(|b| b.write_uniform(&r_of(), 1, glam::Vec3::new(1.0, 2.0, 3.0))),
            ),
            (
                16,
                Box::new(|b| b.write_uniform(&r_of(), 1, glam::Vec4::new(1.0, 2.0, 3.0, 4.0))),
            ),
            (64, Box::new(|b| b.write_uniform(&r_of(), 1, glam::Mat4::IDENTITY))),
        ];
        let _ = r;
        for (width, f) in cases {
            let mut b = binder(128);
            f(&mut b);
            let written = b.data().iter().take_while(|x| **x != 0xAA).count();
            // 0 字节被覆盖为 0 时 take_while 会提前结束，所以改为统计被改写的字节区间。
            let touched = b.data().iter().filter(|x| **x != 0xAA).count();
            assert!(touched <= width, "width {width} wrote {touched} (prefix {written})");
            assert!(b.data()[width..].iter().all(|x| *x == 0xAA), "wrote past {width}");
        }
    }

    fn r_of() -> UniformReflection {
        refl(&[(1, 0)])
    }

    #[test]
    fn bool_is_written_as_u32() {
        let mut b = binder(16);
        b.write_uniform(&refl(&[(9, 4)]), 9, true);
        assert_eq!(&b.data()[4..8], &1u32.to_le_bytes());
        b.write_uniform(&refl(&[(9, 4)]), 9, false);
        assert_eq!(&b.data()[4..8], &0u32.to_le_bytes());
    }

    #[test]
    fn mat4_and_vec3_bytes() {
        let mut b = binder(80);
        b.write_uniform(
            &refl(&[(1, 0), (2, 64)]),
            1,
            glam::Mat4::from_diagonal(glam::Vec4::new(1.0, 2.0, 3.0, 4.0)),
        );
        assert_eq!(&b.data()[0..4], &1.0f32.to_le_bytes());
        assert_eq!(&b.data()[20..24], &2.0f32.to_le_bytes());
        assert_eq!(&b.data()[40..44], &3.0f32.to_le_bytes());
        assert_eq!(&b.data()[60..64], &4.0f32.to_le_bytes());
        b.write_uniform(&refl(&[(2, 64)]), 2, glam::Vec3::new(5.0, 6.0, 7.0));
        assert_eq!(&b.data()[72..76], &7.0f32.to_le_bytes());
        assert_eq!(b.data()[76], 0xAA, "Vec3 must stop at 12 bytes");
    }

    #[test]
    fn guards_return_silently() {
        let mut b = binder(8);
        // 守卫 ①：反射首字为 0。
        let mut r = refl(&[(1, 0)]);
        r.valid = false;
        b.write_uniform(&r, 1, 5u32);
        // 守卫 ②：字段表为空。
        b.write_uniform(&refl(&[]), 1, 5u32);
        // 守卫 ③：键未命中。
        b.write_uniform(&refl(&[(2, 0)]), 1, 5u32);
        assert_eq!(b.data(), &[0xAA; 8]);
    }

    #[test]
    fn optional_closure_runs_only_on_hit_and_reports_bool() {
        let mut b = binder(8);
        let mut calls = 0;
        let hit = b.write_optional_uniform(&refl(&[(1, 0)]), 1, || {
            calls += 1;
            [640u32, 480u32]
        });
        assert!(hit);
        assert_eq!(calls, 1);
        assert_eq!(&b.data()[0..4], &640u32.to_le_bytes());
        assert_eq!(&b.data()[4..8], &480u32.to_le_bytes());

        let miss = b.write_optional_uniform(&refl(&[(1, 0)]), 2, || {
            calls += 1;
            [0u32, 0u32]
        });
        assert!(!miss);
        assert_eq!(calls, 1, "closure must not run on a miss");
        let mut invalid = refl(&[(1, 0)]);
        invalid.valid = false;
        assert!(!b.write_optional_uniform(&invalid, 1, || {
            calls += 1;
            1u32
        }));
        assert_eq!(calls, 1);
    }

    #[test]
    #[should_panic(expected = "range start index 9 out of range for slice of length 8")]
    fn offset_past_end_is_slice_start_index_len_fail() {
        let mut b = binder(8);
        b.write_uniform(&refl(&[(1, 9)]), 1, 1u32);
    }

    #[test]
    fn offset_equal_to_len_passes_the_start_check_like_the_original() {
        // `cmp rdx(len), rcx(offset); jb fail` —— offset == len 不触发起点检查；随后写入长度检查才 panic（I）。
        let mut b = binder(8);
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            b.write_uniform(&refl(&[(1, 8)]), 1, 1u32);
        }));
        let msg = format!("{:?}", r.err().and_then(|e| e.downcast_ref::<String>().cloned()));
        assert!(!msg.contains("range start index"), "{msg}");
    }
}
