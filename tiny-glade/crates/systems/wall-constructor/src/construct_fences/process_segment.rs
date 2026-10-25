//! 围栏段处理（construct_fences/process_segment.rs）。
//!
//! `process_segment`（0x141238d80）：Perchance 0.4.0 状态；间距
//! smoothstep(clamp((extent−1)/5))（`.7` 与 `bits3fce147a` 之间）；柱 −0.1Y
//! 偏移；`crashy_resample` 保角点重采样；64B plank/post/lean/rail 随机装配。
//! `emit_fences`（0x1412388f0）本身只是 puffin scope + `utils::threshold_range`
//! 包装：真实发射体为两个 closure（因此无法从该符号单独还原）。
//!
//! 这里不再保留早期“只写位置”的伪造记录：两函数都委托 country-core 的
//! `systems::wall::construction::fences` CPU 拥有者（`process_fence_segment` /
//! `FenceSegmentInput`），该实现包含原版的端点抑制、概率斜撑与 plank 记录。
//! `FenceCpuOutput` 的 64B 记录是 `WallWoodenPlankInstance64B`，与
//! `construct_fences` 写入 `AssetSsboLibrary` 的记录同源。
use country_core::systems::wall::construction::fences::{
    FenceCpuOutput, FenceError, FenceSegmentInput, process_fence_segment as core_process_segment,
};

/// B `process_segment`（0x141238d80）：单段围栏柱距/柱/板构造。
///
/// `input.positions` 是 prepared 曲线切片（原始角点保留）；
/// `spacing_extent` 为墙包围盒最大边；`skip_first` 对应 `begin != 0`。
pub fn process_fence_segment(input: FenceSegmentInput<'_>) -> Result<FenceCpuOutput, FenceError> {
    core_process_segment(input)
}

/// B `emit_fences`（0x1412388f0）：把一段的 64B 实例追加到 `out`，返回
/// 相邻柱对数（`fence_segments_count`）。
pub fn emit_fences(
    input: FenceSegmentInput<'_>,
    out: &mut Vec<[u8; 64]>,
) -> Result<usize, FenceError> {
    let outcome = process_fence_segment(input)?;
    out.extend(
        outcome
            .instances
            .iter()
            .map(|instance| instance.observed_bytes()),
    );
    Ok(outcome.fence_segments_count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use country_core::resources::walls::publication_fields::WallId;

    fn input(positions: &[[f32; 3]]) -> FenceSegmentInput<'_> {
        FenceSegmentInput {
            wall: WallId(7),
            height: 2.0,
            positions,
            spacing_extent: 4.0,
            seed: 11,
            skip_first: false,
            segment_ordinal: 1,
            segment_count: 1,
        }
    }

    #[test]
    fn emit_writes_real_sixty_four_byte_records() {
        let points = [[0.0, 0.0, 0.0], [0.0, 0.0, 2.0], [0.0, 0.0, 4.0]];
        let mut out = Vec::new();
        let pairs = emit_fences(input(&points), &mut out).unwrap();
        // 采样柱 → 相邻对；每对至少 1 块横板 + 柱实例。
        assert!(pairs >= 1, "adjacent post pairs are counted");
        assert!(!out.is_empty());
        assert!(out.iter().all(|record| record.len() == 64));
    }

    #[test]
    fn too_short_input_is_empty_not_fabricated() {
        let points = [[0.0, 0.0, 0.0]];
        let outcome = process_fence_segment(input(&points)).unwrap();
        assert!(outcome.instances.is_empty());
        assert!(outcome.posts.is_empty());
    }
}
