//! `startup::startup_terrain::{grid_gen_cw, grid_gen_ccw}` — 地形网格的三角形索引。
//!
//! O 0x140928100（`grid_gen_cw`，B 构建）与 0x140928350（`grid_gen_ccw`）。两个函数结构相同、只差
//! 每个 quad 的两个三角形的顶点顺序。参数（Win64）：`(&mut Vec<u32>, col_start, col_end, rows, cells_per_row)`。
//!
//! 顶点按行排布，每行 `S = cells_per_row + 1` 个；对列区间 `[a, b)` 的每一行 `r`、每一列 `k`（相对 `a`），
//! 令 `p = r·S + k`、`q = (r+1)·S + k`、`s = (r+1)·S + k + 1`，顶点下标都再加 `a`，推入：
//!
//! * CW：`(p, q, p+1)`、`(p+1, q, s)`；
//! * CCW：`(p, p+1, q)`、`(q, p+1, s)`。
//!
//! 外层是缓存分块：区间 `[a, b)` 满足 `(b + 1) - a > 5` 时先递归处理 `[a, min(a+4, b))`，`a` 前移 4，
//! 反复直到余下不足 6 列，再在本层处理 `[a, b)`——所以输出顺序是"每 4 列一块、块内逐行逐列"，
//! 不是整行优先。三角形集合与顺序无关的整行实现相同。`a > b` 时落到 `Option::unwrap` 失败
//! （0x1428d9310，位置 `startup_terrain.rs:290:27`）。`rows == 0` 或区间为空不输出。

fn emit_block(
    indices: &mut Vec<u32>,
    start: u32,
    end: u32,
    rows: u32,
    cells_per_row: u32,
    ccw: bool,
) {
    if rows == 0 || start >= end {
        return;
    }
    let stride = cells_per_row.wrapping_add(1);
    for row in 0..rows {
        let top = row.wrapping_mul(stride);
        let bottom = row.wrapping_add(1).wrapping_mul(stride);
        for k in 0..end - start {
            let p = start.wrapping_add(top).wrapping_add(k);
            let q = start.wrapping_add(bottom).wrapping_add(k);
            let s = q.wrapping_add(1);
            if ccw {
                indices.extend_from_slice(&[p, p.wrapping_add(1), q, q, p.wrapping_add(1), s]);
            } else {
                indices.extend_from_slice(&[p, q, p.wrapping_add(1), p.wrapping_add(1), q, s]);
            }
        }
    }
}

fn grid_gen(
    indices: &mut Vec<u32>,
    mut start: u32,
    end: u32,
    rows: u32,
    cells_per_row: u32,
    ccw: bool,
) {
    assert!(start <= end, "called `Option::unwrap()` on a `None` value");
    let mut head = start;
    while end.wrapping_add(1).wrapping_sub(head) > 5 {
        head = start.wrapping_add(4).min(end);
        grid_gen(indices, start, head, rows, cells_per_row, ccw);
        start = head;
    }
    emit_block(indices, head, end, rows, cells_per_row, ccw);
}

/// O 0x140928100。
pub fn grid_gen_cw(indices: &mut Vec<u32>, start: u32, end: u32, rows: u32, cells_per_row: u32) {
    grid_gen(indices, start, end, rows, cells_per_row, false);
}

/// O 0x140928350。
pub fn grid_gen_ccw(indices: &mut Vec<u32>, start: u32, end: u32, rows: u32, cells_per_row: u32) {
    grid_gen(indices, start, end, rows, cells_per_row, true);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    fn triangles(indices: &[u32]) -> BTreeSet<[u32; 3]> {
        indices
            .chunks_exact(3)
            .map(|t| [t[0], t[1], t[2]])
            .collect()
    }

    #[test]
    fn single_quad_has_the_two_exe_triangles() {
        let mut cw = Vec::new();
        grid_gen_cw(&mut cw, 0, 1, 1, 1);
        // S = 2：顶点 0 1 / 2 3。
        assert_eq!(cw, [0, 2, 1, 1, 2, 3]);
        let mut ccw = Vec::new();
        grid_gen_ccw(&mut ccw, 0, 1, 1, 1);
        assert_eq!(ccw, [0, 1, 2, 2, 1, 3]);
    }

    #[test]
    fn the_two_windings_are_mirror_images() {
        let (mut cw, mut ccw) = (Vec::new(), Vec::new());
        grid_gen_cw(&mut cw, 0, 7, 3, 7);
        grid_gen_ccw(&mut ccw, 0, 7, 3, 7);
        assert_eq!(cw.len(), 7 * 3 * 6);
        // 每个三角形反转两个顶点就是另一种缠绕。
        let flipped: BTreeSet<[u32; 3]> = triangles(&cw)
            .into_iter()
            .map(|[a, b, c]| [a, c, b])
            .collect();
        let ccw_set = triangles(&ccw);
        for t in &flipped {
            let mut rotations = [[t[0], t[1], t[2]], [t[1], t[2], t[0]], [t[2], t[0], t[1]]];
            rotations.sort();
            assert!(
                rotations.iter().any(|r| ccw_set.contains(r)),
                "triangle {t:?} has no CCW counterpart"
            );
        }
    }

    #[test]
    fn blocking_changes_the_order_but_not_the_triangle_set() {
        let mut blocked = Vec::new();
        grid_gen_cw(&mut blocked, 0, 11, 2, 11);
        let mut plain = Vec::new();
        for row in 0..2u32 {
            for col in 0..11u32 {
                let mut one = Vec::new();
                emit_block(&mut one, col, col + 1, 1, 11, false);
                // 单列单行的块在行 0；平移到第 `row` 行。
                plain.extend(one.iter().map(|i| i + row * 12));
            }
        }
        assert_eq!(triangles(&blocked), triangles(&plain));
        assert_ne!(
            blocked, plain,
            "11 columns split into 4 + 4 + 3, so the order differs"
        );
        // 前 4 列、两行都在第一块里：第二行的第一个三角形出现在第 5 个 quad 之前。
        assert_eq!(&blocked[..6], &[0, 12, 1, 1, 12, 13]);
        assert_eq!(&blocked[4 * 6..4 * 6 + 6], &[12, 24, 13, 13, 24, 25]);
    }

    #[test]
    fn empty_ranges_and_zero_rows_emit_nothing() {
        let mut v = Vec::new();
        grid_gen_cw(&mut v, 3, 3, 4, 10);
        grid_gen_cw(&mut v, 0, 5, 0, 10);
        assert!(v.is_empty());
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn reversed_range_panics() {
        grid_gen_cw(&mut Vec::new(), 5, 4, 1, 8);
    }
}
