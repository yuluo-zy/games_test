//! 为按行排列的规则网格追加三角形索引；每四列一块，保留已有的输出顺序。
//! 先检查尺寸和索引容量，再一次预留输出空间，不使用递归或整数回绕。

fn emit_block(
    indices: &mut Vec<u32>,
    column_start: u32,
    column_end: u32,
    rows: u32,
    cells_per_row: u32,
    counter_clockwise: bool,
) {
    let row_stride = cells_per_row + 1;
    for row in 0..rows {
        let top_row = row * row_stride;
        let bottom_row = (row + 1) * row_stride;
        for column in column_start..column_end {
            let top_left = top_row + column;
            let top_right = top_left + 1;
            let bottom_left = bottom_row + column;
            let bottom_right = bottom_left + 1;
            if counter_clockwise {
                indices.extend_from_slice(&[
                    top_left,
                    top_right,
                    bottom_left,
                    bottom_left,
                    top_right,
                    bottom_right,
                ]);
            } else {
                indices.extend_from_slice(&[
                    top_left,
                    bottom_left,
                    top_right,
                    top_right,
                    bottom_left,
                    bottom_right,
                ]);
            }
        }
    }
}

fn grid_gen(
    indices: &mut Vec<u32>,
    column_start: u32,
    column_end: u32,
    rows: u32,
    cells_per_row: u32,
    counter_clockwise: bool,
) {
    assert!(column_start <= column_end, "网格列区间不能反向");
    assert!(column_end <= cells_per_row, "网格列区间超出行宽");
    if column_start == column_end || rows == 0 {
        return;
    }
    let row_stride = cells_per_row.checked_add(1).expect("网格行宽溢出");
    rows.checked_mul(row_stride)
        .and_then(|last_row| last_row.checked_add(column_end))
        .expect("网格顶点索引超出 u32 范围");
    let additional = (column_end - column_start) as usize;
    let additional = additional
        .checked_mul(rows as usize)
        .and_then(|count| count.checked_mul(6))
        .expect("网格索引数量超出平台容量");
    indices.reserve(additional);
    let mut block_start = column_start;
    while block_start < column_end {
        let block_end = block_start + (column_end - block_start).min(4);
        emit_block(
            indices,
            block_start,
            block_end,
            rows,
            cells_per_row,
            counter_clockwise,
        );
        block_start = block_end;
    }
}

/// 追加顺时针三角形；无效尺寸在改变输出之前报错。
pub fn grid_gen_cw(
    indices: &mut Vec<u32>,
    column_start: u32,
    column_end: u32,
    rows: u32,
    cells_per_row: u32,
) {
    grid_gen(
        indices,
        column_start,
        column_end,
        rows,
        cells_per_row,
        false,
    );
}

/// 追加逆时针三角形；顶点布局和分块规则与顺时针版本一致。
pub fn grid_gen_ccw(
    indices: &mut Vec<u32>,
    column_start: u32,
    column_end: u32,
    rows: u32,
    cells_per_row: u32,
) {
    grid_gen(indices, column_start, column_end, rows, cells_per_row, true);
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
    #[should_panic(expected = "网格列区间不能反向")]
    fn reversed_range_panics() {
        grid_gen_cw(&mut Vec::new(), 5, 4, 1, 8);
    }
}
