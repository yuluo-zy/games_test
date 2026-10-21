//! B game WallPathSegmentationMask producer and original local curve coordinates.
//! Vec/HashMap/Arc ownership and LoopRange's modulo arithmetic reuse Rust std.
use crate::resources::roofs::geometry::surface::Curve2;
use crate::resources::walls::publication_fields::WallShapeSnapshot;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WallSegmentationError {
    InvalidRange,
    InvalidCoordinate,
    ArrayCount,
    InvalidGeometry,
}

/// Original 12-byte LoopRange. The modulus is prepared_count-1 for a closed
/// outline with its duplicated closing endpoint, prepared_count for open paths.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WallLoopRange {
    pub start: i32,
    pub end: i32,
    pub modulus: i32,
}
impl WallLoopRange {
    pub fn for_closed(
        start: i32,
        end: i32,
        point_count: usize,
    ) -> Result<Self, WallSegmentationError> {
        if point_count < 2 || point_count > i32::MAX as usize {
            return Err(WallSegmentationError::InvalidRange);
        }
        let modulus = point_count as i32 - 1;
        let len = end
            .checked_sub(start)
            .ok_or(WallSegmentationError::InvalidRange)?;
        if start <= -modulus
            || start > modulus
            || end < 0
            || end > modulus + 1
            || len < 0
            || len > modulus + 1
        {
            return Err(WallSegmentationError::InvalidRange);
        }
        Ok(Self {
            start,
            end,
            modulus,
        })
    }
    pub fn for_open(
        start: i32,
        end: i32,
        point_count: usize,
    ) -> Result<Self, WallSegmentationError> {
        if point_count == 0 || point_count >= i32::MAX as usize {
            return Err(WallSegmentationError::InvalidRange);
        }
        let modulus = point_count as i32;
        let len = end
            .checked_sub(start)
            .ok_or(WallSegmentationError::InvalidRange)?;
        if start <= -modulus
            || start >= modulus
            || end < 0
            || end > modulus + 1
            || len < 0
            || len > modulus
        {
            return Err(WallSegmentationError::InvalidRange);
        }
        Ok(Self {
            start,
            end,
            modulus,
        })
    }
    pub fn len(&self) -> usize {
        self.end.saturating_sub(self.start).max(0) as usize
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn index_at(&self, local_index: usize) -> Result<usize, WallSegmentationError> {
        if self.modulus <= 0 || local_index >= self.len() {
            return Err(WallSegmentationError::InvalidRange);
        }
        let index = self
            .start
            .checked_add(
                i32::try_from(local_index).map_err(|_| WallSegmentationError::InvalidRange)?,
            )
            .ok_or(WallSegmentationError::InvalidRange)?;
        Ok(index.rem_euclid(self.modulus) as usize)
    }
    pub fn observed_bytes(&self) -> [u8; 12] {
        let mut out = [0; 12];
        out[..4].copy_from_slice(&self.start.to_le_bytes());
        out[4..8].copy_from_slice(&self.end.to_le_bytes());
        out[8..12].copy_from_slice(&self.modulus.to_le_bytes());
        out
    }
}

/// Original thresholded coordinate is local to a mask's LoopRange, not a
/// global Public Curve3 point ordinal. Last point with fraction0 maps to
/// (count-2,1), preserving standard segment sampling semantics.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ThresholdedCurveCoord {
    pub point_index: u32,
    pub fraction: f32,
}
impl ThresholdedCurveCoord {
    pub fn for_point_count(
        index: usize,
        fraction: f32,
        point_count: usize,
    ) -> Result<Self, WallSegmentationError> {
        if !(0.0..=1.0).contains(&fraction) || index >= point_count || index > u32::MAX as usize {
            return Err(WallSegmentationError::InvalidCoordinate);
        }
        if index + 1 == point_count {
            if fraction != 0.0 || index == 0 {
                return Err(WallSegmentationError::InvalidCoordinate);
            }
            return Ok(Self {
                point_index: (index - 1) as u32,
                fraction: 1.0,
            });
        }
        Ok(Self {
            point_index: index as u32,
            fraction,
        })
    }
    pub fn observed_bytes(&self) -> [u8; 8] {
        let mut out = [0; 8];
        out[..4].copy_from_slice(&self.point_index.to_le_bytes());
        out[4..].copy_from_slice(&self.fraction.to_le_bytes());
        out
    }
}

/// Safe projection of original 136B WallPathSegmentationMask. The three lazy
/// cache Vec fields start empty in process_segment and are not read by the
/// currently recovered materializer; their element schemas remain opaque.
#[derive(Clone, Debug)]
pub struct WallPathSegmentationMask {
    pub flags: Vec<u8>,
    pub coords: Vec<ThresholdedCurveCoord>,
    pub range: WallLoopRange,
    pub length: f32,
}

#[derive(Clone, Copy)]
pub struct WallSegmentationInput<'a> {
    pub prepared_positions: &'a [[f32; 3]],
    pub path_values: &'a [f32],
    pub shape: &'a WallShapeSnapshot,
    pub observed_special_flag: u8,
}

/// Original game producer: positive runs, closed seam merge, private candidate
/// selection, and evenly spaced XZ anchors. Generic curve construction and Vec
/// ownership reuse the already recovered geometry layer and Rust std.
pub fn wall_path_segmentation(
    input: WallSegmentationInput<'_>,
) -> Result<Vec<WallPathSegmentationMask>, WallSegmentationError> {
    let n = input.prepared_positions.len();
    if n != input.path_values.len() {
        return Err(WallSegmentationError::ArrayCount);
    }
    if input
        .prepared_positions
        .iter()
        .flatten()
        .chain(input.path_values)
        .any(|v| !v.is_finite())
    {
        return Err(WallSegmentationError::InvalidGeometry);
    }
    let mut runs = Vec::<(i32, i32)>::new();
    let mut i = 0;
    while i < n {
        if input.path_values[i] <= 0.0 {
            i += 1;
            continue;
        }
        let start = i.saturating_sub(1);
        while i < n && input.path_values[i] > 0.0 {
            i += 1;
        }
        let end = (i + usize::from(i < n)).min(n);
        if end - start >= 2 {
            runs.push((start as i32, end as i32));
        }
    }
    let closed = input.shape.tag() != 2;
    if closed && runs.len() > 1 && runs[0].0 == 0 && runs.last().unwrap().1 == n as i32 {
        runs[0].0 = runs.last().unwrap().0 - n as i32 + 1;
        runs.pop();
    }
    let bytes = input.shape.bytes();
    let f = |at: usize| f32::from_le_bytes(bytes[at..at + 4].try_into().unwrap());
    let mut span = match input.shape.tag() {
        0 => {
            let t = ((f(12) * f32::from_bits(0x40c90fdb) - 10.0) / 8.0).clamp(0.0, 1.0);
            4.0 * t + (1.0 - t) * 2.0
        }
        1 => {
            let t = (((f(20) + f(24)) * 2.0 - 9.0) / 3.0).clamp(0.0, 1.0);
            4.0 * t + (1.0 - t) * 2.0
        }
        _ => 4.0,
    };
    if input.observed_special_flag != 0 {
        span = 3.0;
    }
    runs.into_iter()
        .map(|(start, end)| {
            let range = if closed {
                WallLoopRange::for_closed(start, end, n)?
            } else {
                WallLoopRange::for_open(start, end, n)?
            };
            let raw: Vec<_> = (0..range.len())
                .map(|j| input.prepared_positions[range.index_at(j).unwrap()])
                .collect();
            let shifted: Vec<_> = (0..range.len())
                .map(|j| {
                    let k = range.index_at(j).unwrap();
                    let p = input.prepared_positions[k];
                    let v = input.path_values[k];
                    [p[0] + v * 0.0, p[1] + v, p[2] + v * 0.0]
                })
                .collect();
            process_segment(&raw, &shifted, range, span)
        })
        .collect()
}
fn norm(a: [f32; 3], b: [f32; 3]) -> f32 {
    (glam::Vec3::from_array(b) - glam::Vec3::from_array(a)).length()
}
fn sample(points: &[[f32; 3]], i: usize, t: f32) -> [f32; 3] {
    let a = points[i];
    let b = points[i + 1];
    let s = 1.0 - t;
    [
        b[0] * t + a[0] * s,
        b[1] * t + a[1] * s,
        b[2] * t + a[2] * s,
    ]
}
/// B `score_split_candidate`（0x140751610）：候选拐点评分（原件的
/// `extend_left/right_until` closure 即 `flags & 15 != 0` 停止，距离 2.0）。
pub fn score(index: usize, points: &[[f32; 3]], flags: &[u8]) -> f32 {
    if flags[index] & 15 != 0 {
        return 0.0;
    }
    let mut k = index;
    let mut rem = -2.0;
    let left;
    loop {
        if k == index && flags[index + 1] & 15 != 0 {
            left = points[index];
            break;
        }
        if k == 0 {
            left = points[0];
            break;
        }
        let len = norm(points[k - 1], points[k]);
        // Native reverse iterator starts at index+1; its initial segment
        // contributes zero. Preceding segment lengths then accumulate.
        rem += len;
        if rem >= 0.0 {
            left = sample(points, k - 1, rem / len);
            break;
        }
        k -= 1;
        if flags[k] & 15 != 0 {
            left = points[k];
            break;
        }
    }
    k = index;
    let right;
    let mut rem = norm(points[k], points[k + 1]) - 2.0;
    loop {
        let len = norm(points[k], points[k + 1]);
        if rem >= 0.0 {
            right = sample(points, k, 1.0 - rem / len);
            break;
        }
        k += 1;
        if k + 1 >= points.len() || flags[k] & 15 != 0 {
            right = points[k];
            break;
        }
        rem += norm(points[k], points[k + 1]);
    }
    let p = points[index];
    let dx = right[0] - left[0];
    let dy = right[1] - left[1];
    let dz = right[2] - left[2];
    let dot = ((p[0] - left[0]) * dx + (p[1] - left[1]) * dy) + (p[2] - left[2]) * dz;
    let t = (dot / ((dx * dx + dy * dy) + dz * dz).max(1e-20)).clamp(0.0, 1.0);
    let s = 1.0 - t;
    let projected = [
        right[0] * t + left[0] * s,
        right[1] * t + left[1] * s,
        right[2] * t + left[2] * s,
    ];
    let x = p[0] - projected[0];
    let z = p[2] - projected[2];
    (((x * x + z * z).sqrt() + projected[1]) - p[1]) * 0.5
}
/// B `process_segment`（0x140750420）：单段掩码构造。
pub fn process_segment(
    raw: &[[f32; 3]],
    shifted: &[[f32; 3]],
    range: WallLoopRange,
    span: f32,
) -> Result<WallPathSegmentationMask, WallSegmentationError> {
    let n = raw.len();
    let mut flags = vec![0; n];
    let mut coords = vec![
        ThresholdedCurveCoord {
            point_index: 0,
            fraction: 0.0
        };
        n
    ];
    flags[0] = 2;
    flags[n - 1] = 4;
    coords[n - 1] = ThresholdedCurveCoord::for_point_count(n - 1, 0.0, n)?;
    let mut candidates: Vec<_> = (1..n - 1).collect();
    loop {
        let mut best = None;
        let mut highest = 0.0;
        let mut retained = 0;
        candidates.retain(|&index| {
            let value = (score(index, shifted, &flags) - 0.4).max(0.0);
            if value <= 0.0 {
                return false;
            }
            if value > highest {
                highest = value;
                best = Some(retained);
            }
            retained += 1;
            true
        });
        let Some(best) = best else {
            break;
        };
        let point = candidates.swap_remove(best);
        flags[point] |= 1;
        coords[point] = ThresholdedCurveCoord::for_point_count(point, 0.0, n)?;
    }
    let mut previous = 0;
    for end in 1..n {
        if flags[end] & 15 != 0 {
            if end > previous + 1 {
                add_splits(raw, &mut flags, &mut coords, previous, end + 1, span)?;
            }
            previous = end;
        }
    }
    let first = coords[0];
    let last = coords[n - 1];
    let mut previous = sample(raw, first.point_index as usize, first.fraction);
    let mut length = 0.0;
    for point in &raw[first.point_index as usize + 1..=last.point_index as usize] {
        length += norm(previous, *point);
        previous = *point;
    }
    length += norm(
        previous,
        sample(raw, last.point_index as usize, last.fraction),
    );
    Ok(WallPathSegmentationMask {
        flags,
        coords,
        range,
        length,
    })
}
fn add_splits(
    raw: &[[f32; 3]],
    flags: &mut [u8],
    coords: &mut [ThresholdedCurveCoord],
    start: usize,
    end: usize,
    span: f32,
) -> Result<(), WallSegmentationError> {
    let Ok(curve) = Curve2::try_new(
        raw[start..end].iter().map(|p| [p[0], p[2]]).collect(),
        false,
    ) else {
        return Ok(());
    };
    let mut count = (curve.length / span).floor() as usize;
    if count == 1 && curve.length < span * 1.5 {
        count = 0;
    }
    for i in 1..=count {
        let (index, t) = curve.coord_at_u(i as f32 / (count + 1) as f32);
        let point = index + start;
        let slot = point + usize::from(t > 0.5);
        flags[slot] |= 8;
        coords[slot] = ThresholdedCurveCoord::for_point_count(point, t, raw.len())?;
    }
    Ok(())
}

/// Identified projection of MaterialStats::init_this_update followed by
/// set_path_segmentation_count. Original also resets opaque accumulator/cache
/// fields; their meanings are not manufactured by this typed projection.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WallPathMaterialStatsUpdate {
    pub regular_path_length: f32,
    pub special_path_length: f32,
    pub segmentation_count: u32,
    /// Original MaterialStats+a4; init_this_update resets this to positive0.
    pub stairs_length: f32,
}
impl WallPathMaterialStatsUpdate {
    /// Original add_stairs_length adds+a4 and subtracts regular length+90.
    /// World has exclusive mutable access; atomic CAS storage is a dependency.
    pub fn add_stairs_length(&mut self, length: f32) {
        self.stairs_length += length;
        self.regular_path_length -= length;
    }
}
pub fn material_stats_for_update(
    prepared_length: f32,
    observed_special_flag: u8,
    mask_count: usize,
) -> WallPathMaterialStatsUpdate {
    let (regular_path_length, special_path_length) = if observed_special_flag == 0 {
        (prepared_length, 0.0)
    } else {
        (0.0, prepared_length)
    };
    WallPathMaterialStatsUpdate {
        regular_path_length,
        special_path_length,
        segmentation_count: mask_count as u32,
        stairs_length: 0.0,
    }
}
