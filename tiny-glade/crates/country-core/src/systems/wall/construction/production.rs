//! Original WallConstructor producer. Standard RNG/curve math are reused.
//! Decoded construction kinds, top bounds and fence fit remain explicit inputs.

// Preserve the original instruction-level expression order/NaN policy, API call shapes, and evidence/test transcription. See README.md.
#![allow(
    clippy::excessive_precision,
    clippy::manual_range_patterns,
    clippy::neg_cmp_op_on_partial_ord
)]
use crate::resources::roofs::geometry::surface::Curve2;
use crate::systems::wall::construction::bricks::*;
use fastrand::Rng;
#[derive(Clone, Copy, Debug)]
pub struct PlainWallProductionInput<'a> {
    pub curve: &'a Curve2,
    pub arch_heights: &'a [f32],
    /// Original second per-point array, prepared outline's point Y values.
    pub second_heights: &'a [f32],
    /// Original third per-point array; the actual cache producer initializes it.
    pub third_heights: &'a [f32],
    pub holes: &'a [([f32; 4], u8)],
    pub holes_length: f32,
    pub closed: bool,
    pub seed: u64,
    pub brick_source_id: u32,
    pub base: f32,
    pub row_end_perturbation: f32,
    pub width_padding: Option<f32>,
    pub thickness_scale: Option<f32>,
    /// Original caller scalar; spelling is not guessed from render behavior.
    pub wall_float: f32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WallProductionError {
    InvalidInput,
    Constructor(WallConstructionError),
    UnsupportedBranch,
}
impl From<WallConstructionError> for WallProductionError {
    fn from(e: WallConstructionError) -> Self {
        Self::Constructor(e)
    }
}
#[derive(Clone, Copy)]
struct Coordinate {
    index: usize,
    t: f32,
}
fn start_coordinate(us: &[f32], u: f32) -> Coordinate {
    let mut i = 0;
    let mut len = (us[1] - us[0]).abs();
    let mut remaining = len - u;
    while remaining < 0.0 && i + 2 < us.len() {
        i += 1;
        len = (us[i + 1] - us[i]).abs();
        remaining += len;
    }
    Coordinate {
        index: i,
        t: if remaining < 0.0 {
            1.0
        } else {
            1.0 - remaining / len
        },
    }
}
fn advance_coordinate(us: &[f32], old: Coordinate, step: f32) -> Coordinate {
    let mut i = old.index;
    let mut len = (us[i + 1] - us[i]).abs();
    let mut remaining = (1.0 - old.t) * len - step;
    while remaining < 0.0 && i + 2 < us.len() {
        i += 1;
        len = (us[i + 1] - us[i]).abs();
        remaining += len;
    }
    Coordinate {
        index: i,
        t: if remaining < 0.0 {
            1.0
        } else {
            1.0 - remaining / len
        },
    }
}
fn sample(a: &[f32], c: Coordinate) -> f32 {
    c.t * a[c.index + 1] + (1.0 - c.t) * a[c.index]
}
fn hash_raw(value: f32) -> f32 {
    let x = (value * 12.9898).sin() * 43758.546875;
    x - x.trunc()
}
fn hash_phase(y: f32, seed: f32) -> f32 {
    let x = ((y * 10.0 + seed) * 12.9898).sin() * 43758.546875;
    x - x.trunc()
}
fn noise(x: f32, hash: f32, phase: f32) -> f32 {
    let envelope = (x * 0.2).sin() * 0.5 + 0.5;
    ((hash * envelope + 1.0) * x + phase).sin() * 0.06
}
fn contains_u(row: &WallRow, u: f32, padding: f32) -> bool {
    row.u[0] - padding <= u && u <= row.u[1] + padding
}
fn perturb(
    a: &mut [f32; 4],
    index: usize,
    edges: &[f32],
    last_boundary: usize,
    limit: f32,
    draw: f32,
) {
    let c = (a[0] + a[2]) * 0.5;
    if index == 0 {
        let tail = (edges[last_boundary] + edges[last_boundary - 1]) * (-0.5) + 1.0;
        a[0] += ((tail.min(c)) * 1.99).min(limit) * draw;
    } else if index == last_boundary - 1 {
        let head = (edges[0] + edges[1]) * 0.5;
        a[2] += (head.min(1.0 - c) * 1.99).min(limit) * draw;
    }
}
/// Original24B auxiliary block:5f32 followed by StructureType tag at20.
/// Tag2 ignores the fields; tag0/1 supply a height boundary and two U intervals.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WallTopBounds {
    pub height: f32,
    pub intervals: [[f32; 2]; 2],
    pub tag: u8,
}
impl WallTopBounds {
    pub const PLAIN: Self = Self {
        height: 0.0,
        intervals: [[0.0, 0.0]; 2],
        tag: 2,
    };
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WallProductionBranch {
    pub kind: u8,
    pub top: WallTopBounds,
    pub fence_fit: bool,
}
/// Entire plain producer -> real96B CPU array; source values are supplied once.
/// No captured proposal, derived context or per-proposal RNG draw is injectable.
pub fn produce_plain_wall(
    input: PlainWallProductionInput<'_>,
) -> Result<Vec<WallBrickInstance>, WallProductionError> {
    produce_wall_kind(input, 2)
}
/// Complete top-tag2/fence-fit-false producer for the native construction kind.
/// Kinds retain their real last-row split and fit policies.
pub fn produce_wall_kind(
    input: PlainWallProductionInput<'_>,
    kind: u8,
) -> Result<Vec<WallBrickInstance>, WallProductionError> {
    produce_wall(
        input,
        WallProductionBranch {
            kind,
            top: WallTopBounds::PLAIN,
            fence_fit: false,
        },
    )
}
/// Full decoded original ABI branch. Source input remains proposal-free.
pub fn produce_wall(
    input: PlainWallProductionInput<'_>,
    branch: WallProductionBranch,
) -> Result<Vec<WallBrickInstance>, WallProductionError> {
    let kind = branch.kind;
    let top = branch.top;
    if !matches!(kind, 0 | 1 | 2 | 3) || !matches!(top.tag, 0 | 1 | 2) {
        return Err(WallProductionError::UnsupportedBranch);
    }
    if top.tag != 2
        && (!top.height.is_finite() || !top.intervals.into_iter().flatten().all(f32::is_finite))
    {
        return Err(WallProductionError::InvalidInput);
    }
    let n = input.curve.points.len();
    if n < 2
        || input.curve.points_u.len() != n
        || [
            input.arch_heights.len(),
            input.second_heights.len(),
            input.third_heights.len(),
        ]
        .iter()
        .any(|&l| l != n)
        || !(input.curve.length > 0.0)
    {
        return Err(WallProductionError::InvalidInput);
    }
    let arch_max_height = input
        .arch_heights
        .iter()
        .copied()
        .fold(f32::NEG_INFINITY, f32::max);
    if arch_max_height == 0.0 {
        return Ok(Vec::new());
    }
    if !(arch_max_height > 0.0 && arch_max_height.is_finite()) {
        return Err(WallProductionError::InvalidInput);
    }
    let max_height = if kind == 3 && top.tag == 2 {
        ((arch_max_height + (-0.79)) + 0.15).max(0.1 + input.base)
    } else {
        arch_max_height
    };
    let height_bands = if top.tag == 2 {
        vec![input.base, max_height]
    } else {
        vec![input.base, top.height, max_height]
    };
    let boundaries = split_into_random_rows(&height_bands, max_height, input.seed);
    let fit_heights: Vec<f32>;
    let trim_heights = if branch.fence_fit {
        fit_heights = input.arch_heights.iter().map(|h| *h + 0.46).collect();
        fit_heights.as_slice()
    } else {
        input.arch_heights
    };
    let rows = trim_rows(
        &boundaries,
        &input.curve.points_u,
        trim_heights,
        max_height,
        input.curve.length,
        top.tag != 2,
    )?;
    let holes = normalize_wall_holes(input.holes, input.holes_length, max_height, input.closed)?;
    let count = boundaries.len() as u64;
    let length = input.curve.length;
    let row_rng = Rng::with_seed(
        input
            .seed
            .wrapping_mul(0x17d32)
            .wrapping_add(count.wrapping_mul(0x8ba)),
    );
    let mut output = Vec::new();
    for (row_index, row) in rows.iter().enumerate() {
        let row_draw = (row_rng.f32() * 2.0 - 1.0) * input.row_end_perturbation;
        if row.height[1] <= row.height[0] {
            continue;
        }
        let row_holes = select_native_row_holes(
            [row.u[0], row.height[0], row.u[1], row.height[1]],
            &holes,
            length,
            max_height,
        )?;
        let regular_split = kind != 1 || row_index + 1 != rows.len();
        let (edges, last_boundary) = if top.tag == 2 {
            let edges = if regular_split {
                split_plain_row_horizontal(row.u, length, input.seed, boundaries.len(), row_index)?
            } else {
                last_row_edges(row.u, length, input.seed, boundaries.len(), row_index)
            };
            let count = edges.len() - 1;
            (edges, count)
        } else {
            bounded_row_edges(
                row.u,
                length,
                input.seed,
                boundaries.len(),
                row_index,
                regular_split,
                top.intervals,
            )
        };
        let split_rng = Rng::with_seed(
            input
                .seed
                .wrapping_mul(0x91e)
                .wrapping_add(count.wrapping_mul(0xdf))
                .wrapping_add((row_index as u64).wrapping_mul(0x91fd)),
        );
        let depth_rng = Rng::with_seed(
            input
                .seed
                .wrapping_mul(0x15b3)
                .wrapping_add(count.wrapping_mul(count))
                .wrapping_add(0x2a6)
                .wrapping_add(row_index as u64),
        );
        let yl = row.height[0];
        let yh = row.height[1];
        let hl = hash_phase(yl * max_height, input.seed as f32);
        let hh = hash_phase(yh * max_height, input.seed as f32);
        let pl = hash_raw(hl * 2.0) * 1000.0;
        let ph = hash_raw(hh * 2.0) * 1000.0;
        let mut coordinate = start_coordinate(&input.curve.points_u, row.u[0]);
        let mut prev = [
            sample(input.arch_heights, coordinate),
            sample(input.second_heights, coordinate),
            sample(input.third_heights, coordinate),
        ];
        let mut cache: Option<(f32, f32, f32)> = None;
        for (column, edge) in edges.windows(2).enumerate() {
            let draw = split_rng.f32();
            let depth = ((depth_rng.f32() - 0.5) * 0.07
                + 0.56 * input.thickness_scale.unwrap_or(1.0))
            .max(0.05);
            let half = (edge[1] - edge[0]) * 0.5;
            let middle_coord = advance_coordinate(&input.curve.points_u, coordinate, half);
            let end_coord = advance_coordinate(&input.curve.points_u, middle_coord, half);
            coordinate = end_coord;
            let mid = [
                sample(input.arch_heights, middle_coord),
                sample(input.second_heights, middle_coord),
                sample(input.third_heights, middle_coord),
            ];
            let end = [
                sample(input.arch_heights, end_coord),
                sample(input.second_heights, end_coord),
                sample(input.third_heights, end_coord),
            ];
            let start = prev;
            prev = end;
            let physical_upper = yh * max_height;
            if start[1].min(end[1]) > physical_upper {
                continue;
            }
            let support_sum = (start[2] + mid[2]) + end[2];
            let support = if support_sum > 0.0 {
                [start[2], mid[2], end[2]]
            } else {
                [-100.0; 3]
            };
            let surface = [
                start[1] + support[0],
                mid[1] + support[1],
                end[1] + support[2],
            ];
            if surface.iter().all(|v| *v > physical_upper) {
                continue;
            }
            let padding = 0.46 / length;
            let mut above = rows[row.next_rows[0]..row.next_rows[1]]
                .iter()
                .any(|r| contains_u(r, edge[0], padding) || contains_u(r, edge[1], padding));
            if !regular_split {
                if column & 1 == 0 {
                    continue;
                }
                above = true;
            }
            if kind == 2 && !above && length > 1.12 && input.wall_float > 0.7 && draw < 0.35 {
                continue;
            }
            let first = if let Some((u, l, h)) = cache {
                if u == edge[0] {
                    [l, h]
                } else {
                    [
                        if yl >= 0.01 {
                            noise(edge[0] * length, hl, pl)
                        } else {
                            0.0
                        },
                        if above {
                            noise(edge[0] * length, hh, ph)
                        } else {
                            0.0
                        },
                    ]
                }
            } else {
                [
                    if yl >= 0.01 {
                        noise(edge[0] * length, hl, pl)
                    } else {
                        0.0
                    },
                    if above {
                        noise(edge[0] * length, hh, ph)
                    } else {
                        0.0
                    },
                ]
            };
            let last = [
                if yl >= 0.01 {
                    noise(edge[1] * length, hl, pl)
                } else {
                    0.0
                },
                if above {
                    noise(edge[1] * length, hh, ph)
                } else {
                    0.0
                },
            ];
            cache = Some((edge[1], last[0], last[1]));
            let mut aabb = [edge[0], yl, edge[1], yh];
            if input.row_end_perturbation > 0.0 {
                perturb(
                    &mut aabb,
                    column,
                    &edges,
                    last_boundary,
                    0.13800001 / length,
                    row_draw,
                );
            }
            let brick = WallBrick {
                aabb,
                deformation: [
                    first[0],
                    last[0],
                    if regular_split {
                        first[1]
                    } else {
                        first[1] * 0.1 + 0.1
                    },
                    if regular_split {
                        last[1]
                    } else {
                        last[1] * 0.1 + 0.1
                    },
                ],
                seed: (count as u32)
                    .wrapping_mul(0x924)
                    .wrapping_add(input.seed as u32)
                    .wrapping_add(10)
                    .wrapping_add((row_index as u32).wrapping_mul(0x54))
                    .wrapping_add((column as u32).wrapping_mul(0x171)),
                split_y: 0,
                split_x: 0,
            };
            let midpoint = edge[0] * 0.5 + edge[1] * 0.5;
            let inside = top
                .intervals
                .iter()
                .any(|r| r[0] <= midpoint && midpoint < r[1]);
            let arch_caps = if top.tag != 2 && !inside {
                [top.height; 3]
            } else {
                [start[0], mid[0], end[0]]
            };
            let p0 = input.curve.pos_at_u(edge[0]);
            let p1 = input.curve.pos_at_u(edge[1]);
            let delta = [p1[0] - p0[0], p1[1] - p0[1]];
            let inv = 1.0 / (delta[1] * delta[1] + delta[0] * delta[0]).sqrt();
            let direction = [delta[0] * inv, delta[1] * inv];
            let context = WallEmissionContext {
                curve_ends: [p0, p1],
                row_u: [edge[0], edge[1]],
                length,
                height_scale: max_height,
                width_padding: input.width_padding.unwrap_or(0.0),
                depth_axis: [(-direction[1]) * depth, 0.0, direction[0] * depth],
                surface,
                material: input.brick_source_id,
                flags: 0,
            };
            let pieces = consume_wall_proposal(
                brick,
                &row_holes.holes,
                length,
                max_height,
                (yh - yl) * max_height,
                regular_split,
                draw,
            )?;
            for b in pieces {
                if let Some(mut instance) = emit_plain_wall_brick(b, context) {
                    if !regular_split {
                        let width = (b.aabb[2] - b.aabb[0]) * length;
                        if width < 0.184 {
                            continue;
                        }
                        let height = (b.aabb[3] - b.aabb[1]) * max_height;
                        if width < height {
                            instance.affine_columns[3][1] += (height - width) * (-0.5);
                            instance.affine_columns[1][1] = width;
                        }
                        instance.flags =
                            if (instance.affine_columns[3][1] - max_height).abs() < 0.95 {
                                0x14
                            } else {
                                4
                            };
                    }
                    let center_y = instance.affine_columns[3][1];
                    let height = instance.affine_columns[1][1];
                    let brick_top = center_y + height * 0.5;
                    let clip = brick_top > arch_caps[0] || brick_top > arch_caps[2];
                    if top.tag == 2 {
                        if branch.fence_fit && clip {
                            instance.surface = arch_caps;
                            instance.flags = 0xc;
                        }
                    } else if top.tag == 1 && top.height < center_y && clip {
                        instance.surface = arch_caps;
                        instance.flags = 0xc;
                    } else {
                        instance.flags = 4;
                    }
                    output.push(instance);
                }
            }
        }
    }
    Ok(output)
}

fn last_row_edges(
    row_u: [f32; 2],
    length: f32,
    seed: u64,
    boundary_count: usize,
    row_index: usize,
) -> Vec<f32> {
    let span = (row_u[1] - row_u[0]) * length;
    let count = (span / 0.69).ceil() as usize;
    let count = count + (count & 1);
    let seed = seed
        .wrapping_mul(0x154df)
        .wrapping_add((boundary_count as u64).wrapping_mul(0x159))
        .wrapping_add((row_index as u64).wrapping_mul(0x96a3dd));
    let rng = Rng::with_seed(seed);
    let mut splits = Vec::new();
    random_splits_into(count + 1, 0.05 / span, &rng, &mut splits);
    splits
        .into_iter()
        .map(|t| t * row_u[1] + (1.0 - t) * row_u[0])
        .collect()
}

fn bounded_row_edges(
    row_u: [f32; 2],
    length: f32,
    seed: u64,
    boundary_count: usize,
    row_index: usize,
    regular: bool,
    intervals: [[f32; 2]; 2],
) -> (Vec<f32>, usize) {
    let row_span = row_u[1] - row_u[0];
    let mut breaks = vec![0.0];
    for x in intervals.into_iter().flatten() {
        if x != 0.0 && x != 1.0 && row_u[0] <= x && x < row_u[1] {
            breaks.push((x - row_u[0]) / row_span);
        }
    }
    breaks.push(1.0);
    let seed = seed
        .wrapping_mul(0x154df)
        .wrapping_add((boundary_count as u64).wrapping_mul(0x159))
        .wrapping_add((row_index as u64).wrapping_mul(0x96a3dd));
    let rng = Rng::with_seed(seed);
    let mut edges = Vec::new();
    // Native keeps the sum of subsegment counts separately from Vec length.
    // Appending every subsegment includes duplicate shared endpoints, which
    // must remain for window indices, seeds and the perturb helper arguments.
    let mut last_boundary = 0;
    for b in breaks.windows(2) {
        if b[1] <= b[0] {
            continue;
        }
        let span = (b[1] - b[0]) * (row_span * length);
        let mut count = (span / if regular { 0.46 } else { 0.69 }).ceil() as usize;
        if !regular {
            count += count & 1;
        }
        last_boundary += count;
        let mut local = Vec::new();
        random_splits_into(
            count + 1,
            if regular { 0.32 } else { 0.05 } / span,
            &rng,
            &mut local,
        );
        edges.extend(local.into_iter().map(|t| t * b[1] + (1.0 - t) * b[0]));
    }
    let edges = edges
        .into_iter()
        .map(|t| t * row_u[1] + (1.0 - t) * row_u[0])
        .collect();
    (edges, last_boundary)
}
