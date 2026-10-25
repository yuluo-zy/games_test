//! B platform stair thresholding between raw masks and segment consumers.
//! This produces a separate processed mask list and step ranges; it neither
//! overwrites raw calc masks nor writes the third construction-height array.
use crate::resources::roofs::roof_shape as roof_state;
use crate::resources::walls::publication_fields::PublicWallBuildingState;
use crate::systems::wall::construction::segmentation::WallLoopRange;
use crate::systems::wall::construction::segmentation::WallPathSegmentationMask;
use crate::systems::wall::construction::segmentation::WallSegmentationError;
use crate::systems::wall::construction::segmentation::process_segment;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WallPlatformStepRange {
    pub start: usize,
    pub end: usize,
    pub source_range: WallLoopRange,
}
#[derive(Clone, Debug)]
pub struct WallPlatformSegmentation {
    pub processed_masks: Vec<WallPathSegmentationMask>,
    pub steps: Vec<WallPlatformStepRange>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WallPlatformSegmentationError {
    InvalidInput(WallSegmentationError),
    MissingMaximumY,
}
/// `roof` is the actual component from the validated PublicWalls relationship.
/// Missing or stale entities resolve to None at that relationship owner.
pub fn split_platform_wall_masks(
    public: &[u8; 672],
    prepared_positions: &[[f32; 3]],
    path_values: &[f32],
    raw_masks: &[WallPathSegmentationMask],
    roof: Option<&[u8; 88]>,
) -> Result<WallPlatformSegmentation, WallPlatformSegmentationError> {
    let state = PublicWallBuildingState::from_observed(*public);
    let flat = roof.is_some_and(|r| {
        !roof_state::ty_observed(f32::from_le_bytes(r[0x30..0x34].try_into().unwrap()))
    });
    if state.shape().tag() >= 2 || !flat {
        return Ok(WallPlatformSegmentation {
            processed_masks: raw_masks.to_vec(),
            steps: Vec::new(),
        });
    }
    if raw_masks.is_empty() {
        return Ok(WallPlatformSegmentation {
            processed_masks: Vec::new(),
            steps: Vec::new(),
        });
    }
    if prepared_positions.len() != path_values.len() {
        return Err(WallPlatformSegmentationError::InvalidInput(
            WallSegmentationError::ArrayCount,
        ));
    }
    let maximum = state
        .max_y()
        .map_err(|_| WallPlatformSegmentationError::MissingMaximumY)?;
    if !maximum.is_finite()
        || prepared_positions
            .iter()
            .flatten()
            .chain(path_values)
            .any(|v| !v.is_finite())
    {
        return Err(WallPlatformSegmentationError::InvalidInput(
            WallSegmentationError::InvalidGeometry,
        ));
    }
    let floor = f32::from_bits(0x3f0ccccd);
    let shifted: Vec<_> = prepared_positions
        .iter()
        .zip(path_values)
        .map(|(p, &v)| {
            let height = maximum - if p[1] > floor { p[1] } else { floor };
            let zero = v * 0.0;
            let dy = if height < 3.0 { zero } else { v };
            let dx = if height < 3.0 { zero * 0.0 } else { zero };
            [p[0] + dx, p[1] + dy, p[2] + dx]
        })
        .collect();
    let mut processed_masks = Vec::new();
    let mut steps = Vec::new();
    for mask in raw_masks {
        let range = mask.range;
        let mut heights = Vec::with_capacity(range.len());
        for j in 0..range.len() {
            let k = range
                .index_at(j)
                .map_err(WallPlatformSegmentationError::InvalidInput)?;
            let p =
                prepared_positions
                    .get(k)
                    .ok_or(WallPlatformSegmentationError::InvalidInput(
                        WallSegmentationError::InvalidRange,
                    ))?;
            heights.push(maximum - if p[1] > floor { p[1] } else { floor });
        }
        let mut begin = 0;
        while begin < heights.len() {
            let low = heights[begin] <= 3.0;
            let mut end = begin + 1;
            while end < heights.len() && (heights[end] <= 3.0) == low {
                end += 1;
            }
            if end - begin >= 2 {
                if low {
                    steps.push(WallPlatformStepRange {
                        start: begin,
                        end,
                        source_range: range,
                    });
                } else {
                    let sub = WallLoopRange {
                        start: range.start + begin as i32,
                        end: range.start + end as i32,
                        modulus: range.modulus,
                    };
                    let raw: Vec<_> = (0..sub.len())
                        .map(|i| sub.index_at(i).map(|j| prepared_positions[j]))
                        .collect::<Result<_, _>>()
                        .map_err(WallPlatformSegmentationError::InvalidInput)?;
                    let lifted: Vec<_> = (0..sub.len())
                        .map(|i| sub.index_at(i).map(|j| shifted[j]))
                        .collect::<Result<_, _>>()
                        .map_err(WallPlatformSegmentationError::InvalidInput)?;
                    let span = if public[0x296] != 0 { 3.0 } else { 4.0 };
                    processed_masks.push(
                        process_segment(&raw, &lifted, sub, span)
                            .map_err(WallPlatformSegmentationError::InvalidInput)?,
                    );
                }
            }
            begin = end;
        }
    }
    Ok(WallPlatformSegmentation {
        processed_masks,
        steps,
    })
}
