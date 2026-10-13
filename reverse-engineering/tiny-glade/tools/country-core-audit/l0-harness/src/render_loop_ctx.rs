use crate::render::rg_helpers::rg_boundary::{RenderFn, RenderGraph, StagingAlloc};
use crate::render::rg_helpers;

/// 一个已登记的 pass（O：`RenderContext+0x90` 的元素，0x40 B）。
pub struct RecordedRgPass {
    pub name: &'static str,
    pub resource: u32,
    pub access: u8,
    pub render: RenderFn,
}

/// RG 缓冲句柄：资源号（`+0x10`，O）+ 描述里的字节数（`copy_from_slice` 的断言用）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RgBufferInfo {
    pub resource: u32,
    pub size: u64,
}

/// RG 颜色图像句柄（资源号 `+0x10`）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RgImageInfo {
    pub resource: u32,
}

/// Per-frame render context handed to the render_loop systems（pass 登记表 + 动态常量池）。
#[derive(Default)]
pub struct RenderContext {
    /// `+0x90`：已登记的 pass（按登记顺序）。
    pub passes: Vec<RecordedRgPass>,
    /// `DynamicConstants` 环形池的写游标（U：真实对齐/容量在 rhapsody）。
    dynamic_cursor: u32,
}

impl RenderGraph for RenderContext {
    fn add_pass(&mut self, name: &'static str, resource: u32, access: u8, render: RenderFn) {
        self.passes.push(RecordedRgPass { name, resource, access, render });
    }

    /// `DynamicConstants::allocate(size)`（外部库，U）：按 4 字节对齐推进游标，返回零填充暂存区。
    fn allocate_dynamic(&mut self, size: usize) -> StagingAlloc {
        let offset = self.dynamic_cursor;
        self.dynamic_cursor += ((size as u32) + 3) & !3;
        StagingAlloc { buffer: 0, offset, bytes: vec![0; size] }
    }
}

impl RenderContext {
    /// `RenderContext::finish`（0x1409dda50，348 B）：只释放持有的托管句柄，无业务逻辑 → 消费 `self`。
    pub fn finish(self) -> Vec<RecordedRgPass> {
        self.passes
    }
}

/// `clear_rg_buffer`（0x1409ddbb0）：登记 "Clear buffer"（访问 0x34）。
pub fn clear_rg_buffer(ctx: &mut RenderContext, buffer: &RgBufferInfo) {
    rg_helpers::clear_rg_buffer(ctx, buffer.resource);
}

/// `clear_rg_color_image`（0x1409dde90）：登记 "Clear image"（访问 0x34）。
pub fn clear_rg_color_image(ctx: &mut RenderContext, image: &RgImageInfo) {
    rg_helpers::clear_rg_color_image(ctx, image.resource);
}

/// `rg_buffer_copy_from_slice<u32>`（0x14090d9a0）：唯一观测到的实例是 `u32`。
/// `data.len() * 4 > buffer.size` → 断言失败；其余见 `rg_helpers::rg_buffer_copy_from_slice_u32`。
pub fn rg_buffer_copy_from_slice(ctx: &mut RenderContext, buffer: &RgBufferInfo, data: &[u32]) {
    rg_helpers::rg_buffer_copy_from_slice_u32(ctx, buffer.resource, buffer.size, data);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::rg_helpers::rg_boundary::{CopyRegion, RenderPassApi};

    #[derive(Default)]
    struct Api(Vec<String>);
    impl RenderPassApi for Api {
        fn fill_buffer(&mut self, b: u32, r: Option<core::ops::Range<u64>>, v: u32) {
            self.0.push(format!("fill {b} {r:?} {v}"));
        }
        fn clear_color_image(&mut self, i: u32) {
            self.0.push(format!("clear {i}"));
        }
        fn copy_buffer(&mut self, s: &StagingAlloc, d: u32, r: &[CopyRegion]) {
            self.0.push(format!("copy off={} len={} ->{d} {r:?}", s.offset, s.bytes.len()));
        }
    }

    #[test]
    fn helpers_register_passes_in_order_with_original_names() {
        let mut ctx = RenderContext::default();
        clear_rg_buffer(&mut ctx, &RgBufferInfo { resource: 4, size: 64 });
        clear_rg_color_image(&mut ctx, &RgImageInfo { resource: 9 });
        rg_buffer_copy_from_slice(&mut ctx, &RgBufferInfo { resource: 4, size: 64 }, &[1, 2]);
        let passes = ctx.finish();
        let names: Vec<_> = passes.iter().map(|p| (p.name, p.resource, p.access)).collect();
        assert_eq!(names, [("Clear buffer", 4, 0x34), ("Clear image", 9, 0x34), ("Buffer memcpy", 4, 0x34)]);
        let mut api = Api::default();
        for p in passes {
            (p.render)(&mut api);
        }
        assert_eq!(api.0[0], "fill 4 None 0");
        assert_eq!(api.0[1], "clear 9");
        assert!(api.0[2].contains("len=8") && api.0[2].contains("->4"), "{}", api.0[2]);
    }

    #[test]
    fn dynamic_allocations_advance_the_cursor() {
        let mut ctx = RenderContext::default();
        let a = ctx.allocate_dynamic(8);
        let b = ctx.allocate_dynamic(5);
        let c = ctx.allocate_dynamic(4);
        assert_eq!((a.offset, b.offset, c.offset), (0, 8, 16));
    }

    #[test]
    #[should_panic(expected = "assertion failed: data.len() <= buffer.desc().size")]
    fn oversized_copy_is_rejected_before_any_pass_is_registered() {
        let mut ctx = RenderContext::default();
        rg_buffer_copy_from_slice(&mut ctx, &RgBufferInfo { resource: 1, size: 4 }, &[1, 2]);
    }
}
