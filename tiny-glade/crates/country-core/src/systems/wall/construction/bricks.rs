//! Original wall constructor hole consumer. std owns the proposal queue.
//! Producer/96-byte emission remain separate links; no mesh defaults invented.

// Preserve the original instruction-level expression order/NaN policy, API call shapes, and evidence/test transcription. See README.md.
#![allow(clippy::neg_cmp_op_on_partial_ord)]
use std::collections::VecDeque;
use ::utils::scalar::clamp01;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WallBrick {
    pub aabb: [f32; 4],
    pub deformation: [f32; 4],
    pub seed: u32,
    pub split_y: u8,
    pub split_x: u8,
}
impl WallBrick {
    pub fn observed_bytes(self) -> [u8; 38] {
        let mut out = [0; 38];
        for (i, v) in self.aabb.into_iter().chain(self.deformation).enumerate() {
            out[i * 4..i * 4 + 4].copy_from_slice(&v.to_bits().to_le_bytes());
        }
        out[32..36].copy_from_slice(&self.seed.to_le_bytes());
        out[36] = self.split_y;
        out[37] = self.split_x;
        out
    }
}
fn dims(a: [f32; 4]) -> [f32; 2] {
    [a[2] - a[0], a[3] - a[1]]
}
fn center(a: [f32; 4]) -> [f32; 2] {
    [(a[0] + a[2]) * 0.5, (a[1] + a[3]) * 0.5]
}
fn from_center(c: [f32; 2], d: [f32; 2]) -> [f32; 4] {
    let h = [d[0] * 0.5, d[1] * 0.5];
    [c[0] - h[0], c[1] - h[1], c[0] + h[0], c[1] + h[1]]
}
fn contains(h: [f32; 4], p: [f32; 2]) -> bool {
    h[0] <= p[0] && p[0] <= h[2] && h[1] <= p[1] && p[1] <= h[3]
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WallConstructionError {
    ImpossibleCornerPattern,
    NonOrderableHole,
    InvalidScale,
}
/// Entire B resolve_hole_overlap (1412411c0), including split recursion, native
/// seed/weight/flag updates and ordered deque operations. Not generic AABB minus.
pub fn resolve_hole_overlap(
    hole: [f32; 4],
    bricks: &mut VecDeque<WallBrick>,
) -> Result<(), WallConstructionError> {
    let hd = dims(hole);
    let hc = center(hole);
    let hh = [hd[0] * 0.5, hd[1] * 0.5];
    if !(hd[0] > 0.0 && hd[1] > 0.0) {
        return Ok(());
    }
    let mut index = 0;
    while index < bricks.len() {
        let b = bricks[index];
        let a = b.aabb;
        let c = center(a);
        let d = dims(a);
        let h = [d[0] * 0.5, d[1] * 0.5];
        let corners = [
            [c[0] - h[0], c[1] - h[1]],
            [c[0] + h[0], c[1] - h[1]],
            [c[0] - h[0], c[1] + h[1]],
            [c[0] + h[0], c[1] + h[1]],
        ];
        let inside = corners.map(|p| contains(hole, p));
        let count = inside.into_iter().filter(|x| *x).count();
        match count {
            0 => {
                if b.split_x == 0 && a[2] > hole[2] && hole[0] > a[0] {
                    let half = d[0] * 0.5;
                    let mut first = b;
                    let mut second = b;
                    first.aabb = from_center([a[0] + half * 0.5, c[1]], [half, d[1]]);
                    second.aabb = from_center([a[2] - half * 0.5, c[1]], [half, d[1]]);
                    first.deformation = [0.0, 0.0, b.deformation[2] * 0.5, b.deformation[3] * 0.5];
                    second.deformation = first.deformation;
                    first.split_x = 1;
                    second.split_x = 1;
                    second.seed = b.seed.wrapping_add(100);
                    bricks[index] = first;
                    bricks.push_back(second);
                } else if b.split_y == 0 && a[3] > hole[3] && hole[1] > a[1] {
                    let half = d[1] * 0.5;
                    let mut first = b;
                    let mut second = b;
                    first.aabb = from_center([c[0], a[1] + half * 0.5], [d[0], half]);
                    second.aabb = from_center([c[0], a[3] - half * 0.5], [d[0], half]);
                    first.deformation = [0.0, 0.0, b.deformation[2] * 0.5, b.deformation[3] * 0.5];
                    second.deformation = first.deformation;
                    first.split_y = 1;
                    second.split_y = 1;
                    second.seed = b.seed.wrapping_add(100);
                    bricks[index] = first;
                    bricks.push_back(second);
                } else {
                    index += 1;
                }
            }
            1 => {
                let corner = inside.into_iter().position(|x| x).unwrap();
                let left = corner % 2 == 0;
                let bottom = corner < 2;
                let edge = if left { hc[0] + hh[0] } else { hc[0] - hh[0] };
                let t = clamp01((edge - corners[0][0]) / (corners[1][0] - corners[0][0]));
                let s = 1.0 - t;
                let leftw = d[0] * t;
                let rightw = d[0] * s;
                let leftc = a[0] + leftw * 0.5;
                let rightc = a[2] - rightw * 0.5;
                let overlap = if bottom {
                    (hc[1] + hh[1]) - corners[0][1]
                } else {
                    corners[2][1] - (hc[1] - hh[1])
                };
                let clipc = c[1] + overlap * if bottom { 0.5 } else { -0.5 };
                let cliph = d[1] - overlap;
                let mut full = b;
                let mut small = b;
                full.aabb = from_center(
                    [if left { rightc } else { leftc }, c[1]],
                    [if left { rightw } else { leftw }, d[1]],
                );
                small.aabb = from_center(
                    [if left { leftc } else { rightc }, clipc],
                    [if left { leftw } else { rightw }, cliph],
                );
                full.deformation = b.deformation.map(|v| v * s);
                small.deformation = if bottom {
                    [0.0, 0.0, b.deformation[2] * t, b.deformation[3] * t]
                } else {
                    [b.deformation[0] * t, b.deformation[1] * t, 0.0, 0.0]
                };
                full.seed = b.seed.wrapping_add(u32::from(left)) % 10;
                small.seed = b.seed.wrapping_add(u32::from(!left)) % 10;
                let (first, second) = if left { (full, small) } else { (small, full) };
                bricks.push_front(second);
                bricks.push_front(first);
                bricks.swap_remove_back(index + 2);
                index += 2;
            }
            2 => {
                let mut piece = b;
                piece.seed = b.seed % 10;
                match inside {
                    [true, true, false, false] => {
                        let overlap = (hc[1] + hh[1]) - corners[0][1];
                        piece.aabb =
                            from_center([c[0], c[1] + overlap * 0.5], [d[0], d[1] - overlap]);
                        piece.deformation[0] = 0.0;
                        piece.deformation[1] = 0.0;
                    }
                    [false, false, true, true] => {
                        let overlap = corners[2][1] - (hc[1] - hh[1]);
                        piece.aabb =
                            from_center([c[0], c[1] + overlap * (-0.5)], [d[0], d[1] - overlap]);
                        piece.deformation[2] = 0.0;
                        piece.deformation[3] = 0.0;
                    }
                    [true, false, true, false] => {
                        let overlap = (hc[0] + hh[0]) - corners[0][0];
                        piece.aabb =
                            from_center([c[0] + overlap * 0.5, c[1]], [d[0] - overlap, d[1]]);
                        piece.deformation[0] = 0.0;
                        piece.deformation[2] = 0.0;
                    }
                    [false, true, false, true] => {
                        let overlap = corners[1][0] - (hc[0] - hh[0]);
                        piece.aabb =
                            from_center([c[0] + overlap * (-0.5), c[1]], [d[0] - overlap, d[1]]);
                        piece.deformation[1] = 0.0;
                        piece.deformation[3] = 0.0;
                    }
                    _ => return Err(WallConstructionError::ImpossibleCornerPattern),
                }
                bricks[index] = piece;
                index += 1;
            }
            4 => {
                bricks.swap_remove_back(index);
            }
            _ => return Err(WallConstructionError::ImpossibleCornerPattern),
        }
    }
    Ok(())
}
/// Consumer of actual proposal records; native row production remains separate.
pub fn cut_wall_bricks(
    mut bricks: VecDeque<WallBrick>,
    holes: &[[f32; 4]],
) -> Result<VecDeque<WallBrick>, WallConstructionError> {
    for &h in holes {
        resolve_hole_overlap(h, &mut bricks)?;
    }
    Ok(bricks)
}

/// Original game utils::random_splits_into. fastrand 1.8 remains the dependency.
/// count denotes boundary count; fewer than three yields exactly [0, 1].
pub fn random_splits_into(count: usize, max_jitter: f32, rng: &fastrand::Rng, out: &mut Vec<f32>) {
    if count < 3 {
        out.extend([0.0, 1.0]);
        return;
    }
    let spacing = 1.0 / ((count - 1) as f32);
    let jitter = (spacing * 0.495).min(max_jitter);
    out.push(0.0);
    for i in 1..count - 1 {
        out.push(((rng.f32() + (-0.5)) * jitter) + (i as f32) * spacing);
    }
    out.push(1.0);
}
/// B split_into_random_rows 14123e110. Adjacent physical height bands become
/// normalized random row boundaries; near consecutive boundaries are filtered.
pub fn split_into_random_rows(heights: &[f32], height_scale: f32, seed: u64) -> Vec<f32> {
    let rng = fastrand::Rng::with_seed(seed);
    let mut out = Vec::new();
    let distance = 0.046 / height_scale;
    for h in heights.windows(2) {
        let span = h[1] - h[0];
        let segments = ((span / 0.46).floor() as usize).max(2);
        let mut split = Vec::new();
        random_splits_into(segments + 1, 0.21 / span, &rng, &mut split);
        let low = h[0] / height_scale;
        let high = h[1] / height_scale;
        let mut previous = -100.0;
        for t in split {
            let value = t * high + (1.0 - t) * low;
            if previous > 0.0 && (previous - value).abs() < distance {
                continue;
            }
            out.push(value);
            previous = value;
        }
    }
    out
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WallRow {
    pub next_rows: [usize; 2],
    pub height: [f32; 2],
    pub u: [f32; 2],
}
impl WallRow {
    pub fn observed_bytes(self) -> [u8; 32] {
        let mut b = [0; 32];
        for (i, v) in self.next_rows.into_iter().enumerate() {
            b[i * 8..i * 8 + 8].copy_from_slice(&(v as u64).to_le_bytes());
        }
        for (i, v) in self.height.into_iter().chain(self.u).enumerate() {
            b[16 + i * 4..20 + i * 4].copy_from_slice(&v.to_le_bytes());
        }
        b
    }
}
/// Original trim_rows + split_heightmap_by_threshold<RowTrimmerSink>. The curve
/// U samples and per-point authority heights are caller inputs, not invented.
pub fn trim_rows(
    boundaries: &[f32],
    curve_u: &[f32],
    authority_heights: &[f32],
    height_scale: f32,
    length: f32,
    special: bool,
) -> Result<Vec<WallRow>, WallConstructionError> {
    if curve_u.len() != authority_heights.len() || curve_u.is_empty() || !(length > 0.0) {
        return Err(WallConstructionError::InvalidScale);
    }
    let mut rows: Vec<WallRow> = Vec::new();
    for y in boundaries.windows(2) {
        let threshold = ((y[1] + y[0]) * 0.5) * height_scale.max(0.1);
        let start = rows.len();
        if threshold < authority_heights[0] {
            rows.push(WallRow {
                next_rows: [0, 0],
                height: [y[0], y[1]],
                u: [curve_u[0], 1.0],
            });
        }
        for i in 1..authority_heights.len() {
            let a = authority_heights[i - 1];
            let b = authority_heights[i];
            let crosses = (a <= threshold && threshold < b) || (threshold < a && b <= threshold);
            if crosses {
                let t = (threshold - a) / (b - a);
                let u = t * curve_u[i] + (1.0 - t) * curve_u[i - 1];
                if a <= threshold {
                    rows.push(WallRow {
                        next_rows: [0, 0],
                        height: [y[0], y[1]],
                        u: [u, 1.0],
                    });
                } else {
                    rows.last_mut()
                        .ok_or(WallConstructionError::ImpossibleCornerPattern)?
                        .u[1] = u;
                }
            } else if threshold < b {
                if rows.len() == start {
                    rows.push(WallRow {
                        next_rows: [0, 0],
                        height: [y[0], y[1]],
                        u: [curve_u[i], 1.0],
                    });
                } else {
                    rows.last_mut().unwrap().u[1] = curve_u[i];
                }
            }
        }
    }
    let spacing = 0.46 / length;
    let step = if special { 0.2 } else { 0.5 } * spacing;
    let reciprocal = 1.0 / step;
    for r in &mut rows {
        if r.u[0] != 0.0 {
            r.u[0] = clamp01((r.u[0] / step).round() * step);
        }
        if r.u[1] != 1.0 {
            r.u[1] = clamp01((r.u[1] * reciprocal).round() * step);
        }
    }
    if special {
        for r in &mut rows {
            if r.u[1] - r.u[0] <= spacing {
                r.u[0] = clamp01((r.u[0] + r.u[1]) * 0.5 - 0.5 * spacing);
                r.u[1] = clamp01((r.u[0] + r.u[1]) * 0.5 + 0.5 * spacing);
            }
        }
    } else if length > 0.92 {
        rows.retain(|r| r.u[1] - r.u[0] > spacing);
    }
    for i in 0..rows.len() {
        let mut begin = 0;
        let mut end = 0;
        for j in i + 1..rows.len() {
            if rows[i].height[1] == rows[j].height[0]
                && rows[i].u[0] <= rows[j].u[1]
                && rows[j].u[0] <= rows[i].u[1]
            {
                if begin == end {
                    begin = j;
                }
                end = j + 1;
            }
        }
        rows[i].next_rows = [begin, end];
    }
    Ok(rows)
}
/// 墙砖 CPU 生产与 GPU 上传使用同一载荷，避免重复结构和整批转换。
pub use crate::geometry::instanced_wall::BrickTransformSsbo as WallBrickInstance;
/// Resolved row context entering the native 14076c0ed..14076c092 consumer.
/// `depth_axis`, `surface`, `flags` originate upstream; no defaults are supplied.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WallEmissionContext {
    pub curve_ends: [[f32; 2]; 2],
    pub row_u: [f32; 2],
    pub length: f32,
    pub height_scale: f32,
    pub width_padding: f32,
    pub depth_axis: [f32; 3],
    pub surface: [f32; 3],
    pub material: u32,
    pub flags: u32,
}
/// Actual CPU instance producer for resolved proposals. Full row creation,
/// surface choice, curve ownership, thickness noise and fence fitting are outer
/// links, so this API explicitly requires their observed context.
pub fn emit_resolved_wall_brick(
    brick: WallBrick,
    context: WallEmissionContext,
) -> Option<WallBrickInstance> {
    let d = dims(brick.aabb);
    let c = center(brick.aabb);
    let width = d[0] * context.length;
    if !(d[0] > 0.0 && d[1] > 0.0) || width < 0.03 {
        return None;
    }
    let p0 = context.curve_ends[0];
    let p1 = context.curve_ends[1];
    let delta = [p1[0] - p0[0], p1[1] - p0[1]];
    let inv = 1.0 / (delta[1] * delta[1] + delta[0] * delta[0]).sqrt();
    let dir = [delta[0] * inv, delta[1] * inv];
    let t = (c[0] - context.row_u[0]) * (1.0 / (context.row_u[1] - context.row_u[0]));
    let pos = [
        t * p1[0] + (1.0 - t) * p0[0],
        c[1] * context.height_scale,
        t * p1[1] + (1.0 - t) * p0[1],
    ];
    let w = width + context.width_padding;
    let h = d[1] * context.height_scale;
    let half = d[0] * 0.5;
    Some(WallBrickInstance {
        affine_columns: [
            [w * dir[0], 0.0, w * dir[1]],
            [0.0, h, 0.0],
            context.depth_axis,
            pos,
        ],
        deformation: brick.deformation,
        surface: context.surface,
        seed: brick.seed,
        material: context.material,
        wall_distance: [
            (c[0] - half) * context.length,
            (c[0] + half) * context.length,
        ],
        flags: context.flags,
    })
}
/// Original normalization from FinalizedWallHoles: kind is ignored; origin is
/// the original primary/wrap discriminator. Padding creates periodic candidates
/// and never expands the emitted bounds themselves. Stable max-x sort is std.
pub fn normalize_wall_holes(
    holes: &[([f32; 4], u8)],
    length: f32,
    height_scale: f32,
    closed: bool,
) -> Result<Vec<[f32; 4]>, WallConstructionError> {
    // Native normalization computes reciprocal scales before iterating, but an
    // empty actual FinalizedWallHoles storage emits no scaled coordinates.
    if holes.is_empty() {
        return Ok(Vec::new());
    }
    if !(length > 0.0 && height_scale > 0.0) {
        return Err(WallConstructionError::InvalidScale);
    }
    let scale = [1.0 / length, 1.0 / height_scale];
    let mut out = Vec::new();
    for &(a, origin) in holes {
        if closed && origin != 0 {
            continue;
        }
        let norm = |a: [f32; 4]| {
            [
                a[0] * scale[0],
                a[1] * scale[1],
                a[2] * scale[0],
                a[3] * scale[1],
            ]
        };
        out.push(norm(a));
        if closed {
            let shift = if a[0] < 0.46 {
                length
            } else if length < a[2] + 0.46 {
                -length
            } else {
                continue;
            };
            out.push(norm([a[0] + shift, a[1], a[2] + shift, a[3]]));
        }
    }
    if out.iter().any(|a| a[2].is_nan()) {
        return Err(WallConstructionError::NonOrderableHole);
    }
    out.sort_by(|a, b| a[2].partial_cmp(&b[2]).unwrap());
    Ok(out)
}
fn intersects(a: [f32; 4], b: [f32; 4]) -> bool {
    a[2] >= b[0] && b[2] >= a[0] && a[1] <= b[3] && b[1] <= a[3]
}
/// Resolved plain-wall proposal's real owner after raw production. Padding only
/// controls seed modulation; actual intersection controls destructive clipping.
/// If no hole intersects, the original row split decision and split_horizontally
/// consume the supplied actual per-proposal random draw (seed owner is upstream).
pub fn consume_wall_proposal(
    brick: WallBrick,
    holes: &[[f32; 4]],
    length: f32,
    height_scale: f32,
    row_height: f32,
    split_enabled: bool,
    split_draw: f32,
) -> Result<VecDeque<WallBrick>, WallConstructionError> {
    if !(length > 0.0 && height_scale > 0.0 && row_height > 0.0) {
        return Err(WallConstructionError::InvalidScale);
    }
    let mut out = VecDeque::from([brick]);
    let pad = [(0.46 / length) * 0.5, (0.46 / height_scale) * 0.5];
    let mut had_cut = false;
    for &hole in holes {
        let expanded = [
            hole[0] - pad[0],
            hole[1] - pad[1],
            hole[2] + pad[0],
            hole[3] + pad[1],
        ];
        if intersects(expanded, brick.aabb) {
            for b in &mut out {
                b.seed %= 10;
            }
        }
        if intersects(hole, brick.aabb) {
            resolve_hole_overlap(hole, &mut out)?;
            had_cut = true;
        }
    }
    if !had_cut {
        let de = brick.deformation;
        let span = (de[2].max(de[3]) - de[0].min(de[1])) + row_height;
        let q = 1.0 - span / row_height;
        let chance = if split_enabled {
            q * 0.8 + (1.0 - q) * 0.4
        } else {
            0.0
        };
        if span > 0.46 && split_draw < chance {
            let t = (split_draw / 0.4) * f32::from_bits(0x3ecccccc) + 0.3;
            let inv = 1.0 - t;
            let d = dims(brick.aabb);
            let c = center(brick.aabb);
            let lower = d[1] * t;
            let upper = d[1] * inv;
            let mid = [t * de[2] + inv * de[0], t * de[3] + inv * de[1]];
            let mut a = brick;
            let mut b = brick;
            a.aabb = from_center([c[0], brick.aabb[1] + lower * 0.5], [d[0], lower]);
            b.aabb = from_center([c[0], brick.aabb[3] - upper * 0.5], [d[0], upper]);
            a.deformation = [de[0], de[1], mid[0], mid[1]];
            b.deformation = [mid[0], mid[1], de[2], de[3]];
            a.split_y = 0;
            a.split_x = 0;
            b.split_y = 0;
            b.split_x = 0;
            b.seed = b.seed.wrapping_add(1);
            out = VecDeque::from([a, b]);
        }
    }
    Ok(out)
}
/// Actual regular plain-wall metadata branch (construction kind2, top tag2,
/// fence-fit bool false). Surface/depth belong to the source proposal and remain
/// shared across cuts; flags are selected freshly from each cut brick center.
pub fn emit_plain_wall_brick(
    brick: WallBrick,
    mut context: WallEmissionContext,
) -> Option<WallBrickInstance> {
    let y = center(brick.aabb)[1] * context.height_scale;
    context.flags = if (y - context.height_scale).abs() < 0.95 {
        0x14
    } else {
        4
    };
    emit_resolved_wall_brick(brick, context)
}
/// Complete actual consumer for one resolved plain proposal: seed modulation,
/// destructive holes or stochastic row split, then real per-piece CPU instances.
/// Raw proposal producer, sampled context and per-proposal random owner remain
/// explicit upstream inputs; this never fills them with default game rules.
pub fn construct_resolved_plain_wall_proposal(
    brick: WallBrick,
    holes: &[[f32; 4]],
    context: WallEmissionContext,
    row_height: f32,
    split_draw: f32,
) -> Result<Vec<WallBrickInstance>, WallConstructionError> {
    let bricks = consume_wall_proposal(
        brick,
        holes,
        context.length,
        context.height_scale,
        row_height,
        true,
        split_draw,
    )?;
    Ok(bricks
        .into_iter()
        .filter_map(|b| emit_plain_wall_brick(b, context))
        .collect())
}

#[derive(Clone, Debug, PartialEq)]
pub struct NativeRowHoleCandidates {
    /// Stable source indices into the already-normalized max-x ordered list.
    pub source_indices: Vec<usize>,
    pub holes: Vec<[f32; 4]>,
    /// Original reverse minima used for conservative early exit during proposals.
    pub suffix_min_x: Vec<f32>,
}
/// Actual from_curve row collector (B14076a293..14076a498). `row_bounds` is
/// [u_min,y_min,u_max,y_max], normalized like the native32B WallRow. Input hole
/// order is preserved; upstream normalize owns its stable max-x ordering.
pub fn select_native_row_holes(
    row_bounds: [f32; 4],
    normalized_holes: &[[f32; 4]],
    length: f32,
    height_scale: f32,
) -> Result<NativeRowHoleCandidates, WallConstructionError> {
    if !(length > 0.0 && height_scale > 0.0) {
        return Err(WallConstructionError::InvalidScale);
    }
    let half = [(0.46 / length) * 0.5, (0.46 / height_scale) * 0.5];
    let mut out = NativeRowHoleCandidates {
        source_indices: Vec::new(),
        holes: Vec::new(),
        suffix_min_x: Vec::new(),
    };
    for (index, &h) in normalized_holes.iter().enumerate() {
        let expanded = [
            h[0] - half[0],
            h[1] - half[1],
            half[0] + h[2],
            half[1] + h[3],
        ];
        if intersects(row_bounds, expanded) {
            out.source_indices.push(index);
            out.holes.push(h);
        }
    }
    out.suffix_min_x.resize(out.holes.len(), 0.0);
    let mut minimum = f32::MAX;
    for i in (0..out.holes.len()).rev() {
        let edge = out.holes[i][0] - half[0];
        minimum = if minimum.is_nan() || edge < minimum {
            edge
        } else {
            minimum
        };
        out.suffix_min_x[i] = minimum;
    }
    Ok(out)
}
/// Source constructor kind2/top-tag2 row partition owner, before raw brick
/// noise/authority cropping. `boundary_count` is original vertical split vector
/// length, while `row_index` counts actual trimmed32B rows (not curve points).
pub fn split_plain_row_horizontal(
    row_u: [f32; 2],
    length: f32,
    source_seed: u64,
    boundary_count: usize,
    row_index: usize,
) -> Result<Vec<f32>, WallConstructionError> {
    let physical_span = (row_u[1] - row_u[0]) * length;
    if !(physical_span > 0.0 && physical_span.is_finite()) {
        return Err(WallConstructionError::InvalidScale);
    }
    let seed = source_seed
        .wrapping_mul(0x154df)
        .wrapping_add((boundary_count as u64).wrapping_mul(0x159))
        .wrapping_add((row_index as u64).wrapping_mul(0x96a3dd));
    let rng = fastrand::Rng::with_seed(seed);
    let mut split = Vec::new();
    let segments = (physical_span / 0.46).ceil() as usize;
    random_splits_into(segments + 1, 0.32 / physical_span, &rng, &mut split);
    Ok(split
        .into_iter()
        .map(|t| t * row_u[1] + (1.0 - t) * row_u[0])
        .collect())
}
