//! buffers.rs — nani 绘制参数与 SSBO（WP3）。
//!
//! 机器码锚点（B patched EXE）：
//! - `nani_calculate_draw_list_params` 0x1408baed0（行 133）。
//! - `impl$9::default_value` 0x1408bb650（行 258）。
//! 注意两入口反编译重叠（跳表错位，Ghidra 解析到同一代码体）。
//! 完整契约见 contracts/buffers.md。

#![allow(dead_code, unused_imports)]

use alloc::vec::Vec;

/// 绘制参数（0x70B，O；`nani_calculate_draw_list_params` 末尾写回结构）。
#[derive(Debug, Default)]
pub struct DrawListParams {
    /// 每 kind 累计 cmd_offset（前缀和）。
    pub cmd_offsets: Vec<u32>,
    /// 每 kind 累计 max_instances。
    pub max_instance_offsets: Vec<u32>,
    /// 每 kind 累计 fine-grained 子列表数。
    pub fine_grained_offsets: Vec<u32>,
    /// 每 kind 分批汇总。
    pub batch_offsets: Vec<u32>,
    /// 汇总：总 cmd 偏移。
    pub total_cmd_offset: u32,
    /// 汇总：总最大实例数。
    pub total_max_instances: u32,
    /// 汇总：总细粒度子列表数。
    pub total_fine_grained: u32,
    /// 汇总：总批次。
    pub total_batches: u32,
}

/// 计算 27 种 subset 的绘制参数（0x1408baed0，O）。
/// 对每个 kind（switch 0..0x1b）：push 累计 cmd_offset；`max_instances += max_draws_per_list[kind]`；
/// `fine = max_draws >> 5`（受 NANI_FORCE_NO_FINE_GRAINED_SORTING / kind<2 /
/// NANI_FORCE_FINE_GRAINED_SORTING 调整）；`batch = fine/1024 + (fine%1024!=0)`。
pub fn nani_calculate_draw_list_params() -> DrawListParams {
    // per-kind 常量表 DAT_142aa14c0 未读内存（typed gap）。
    todo!("recover per-kind table scan; see contracts/buffers.md §2")
}

/// SSBO 载体：draw-list 长度。
pub struct NaniDrawListLengthSsbo;

/// SSBO 载体：间接 draw 列表（元素推断 = `VkDrawIndexedIndirectCommand` 20B）。
pub struct NaniDrawListSsbo;

/// SSBO 载体：实例间接寻址表。
pub struct NaniInstanceIndirectionSsbo;
