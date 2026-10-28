//! Range arithmetic: intersection, difference, adjacency and coalescing.
use crate::Error;
use crate::Result;
use smallvec::SmallVec;
use super::native::NativeRange;
use super::owner::RangeTransfer;
use super::patch::RangeCell;

/// A use of a range whose queue-family release has no recorded acquire yet.
/// Render graphs acquire such ranges themselves; other command producers must
/// first record the device's pending acquires.
pub(crate) fn released_range_error() -> Error {
    Error::invalid_usage(
        "range is released by a queue-family transfer: record its pending acquire first \
         (Device::record_pending_acquires or TransferManager::flush_acquires)",
    )
}

pub fn validate_transfer(transfer: RangeTransfer) -> Result<()> {
    validate_range(transfer.range)?;
    if transfer.pair == 0
        || transfer.source_family == transfer.destination_family
        || transfer.source_family == u32::MAX
        || transfer.destination_family == u32::MAX
    {
        return Err(Error::invalid_usage("invalid native range ownership pair"));
    }
    Ok(())
}

pub fn validate_range(range: NativeRange) -> Result<()> {
    let valid = match range {
        NativeRange::Buffer { offset, size } => size != 0 && offset.checked_add(size).is_some(),
        NativeRange::Image {
            aspects,
            base_mip,
            mip_count,
            base_layer,
            layer_count,
        } => {
            aspects != 0
                && mip_count != 0
                && layer_count != 0
                && base_mip.checked_add(mip_count).is_some()
                && base_layer.checked_add(layer_count).is_some()
        }
    };
    if valid {
        Ok(())
    } else {
        Err(Error::invalid_usage("invalid native synchronization range"))
    }
}

pub fn contains(outer: NativeRange, inner: NativeRange) -> bool {
    intersection(outer, inner) == Some(inner)
}

pub fn intersection(a: NativeRange, b: NativeRange) -> Option<NativeRange> {
    match (a, b) {
        (
            NativeRange::Buffer { offset: a, size: n },
            NativeRange::Buffer { offset: b, size: m },
        ) => {
            let start = a.max(b);
            let end = a.checked_add(n)?.min(b.checked_add(m)?);
            (start < end).then(|| NativeRange::Buffer {
                offset: start,
                size: end - start,
            })
        }
        (
            NativeRange::Image {
                aspects: a,
                base_mip: am,
                mip_count: an,
                base_layer: al,
                layer_count: ac,
            },
            NativeRange::Image {
                aspects: b,
                base_mip: bm,
                mip_count: bn,
                base_layer: bl,
                layer_count: bc,
            },
        ) => {
            let aspects = a & b;
            let mip = am.max(bm);
            let end_mip = am.checked_add(an)?.min(bm.checked_add(bn)?);
            let layer = al.max(bl);
            let end_layer = al.checked_add(ac)?.min(bl.checked_add(bc)?);
            (aspects != 0 && mip < end_mip && layer < end_layer).then(|| NativeRange::Image {
                aspects,
                base_mip: mip,
                mip_count: end_mip - mip,
                base_layer: layer,
                layer_count: end_layer - layer,
            })
        }
        _ => None,
    }
}

/// Slab subtraction preserves exact union and never fills a hole.
pub fn difference(whole: NativeRange, cut: NativeRange) -> SmallVec<[NativeRange; 5]> {
    let mut pieces = SmallVec::new();
    let Some(overlap) = intersection(whole, cut) else {
        pieces.push(whole);
        return pieces;
    };
    match (whole, overlap) {
        (
            NativeRange::Buffer { offset, size },
            NativeRange::Buffer {
                offset: start,
                size: bytes,
            },
        ) => {
            if offset < start {
                pieces.push(NativeRange::Buffer {
                    offset,
                    size: start - offset,
                });
            }
            if start + bytes < offset + size {
                pieces.push(NativeRange::Buffer {
                    offset: start + bytes,
                    size: offset + size - start - bytes,
                });
            }
        }
        (
            NativeRange::Image {
                aspects,
                base_mip,
                mip_count,
                base_layer,
                layer_count,
            },
            NativeRange::Image {
                aspects: mask,
                base_mip: mip,
                mip_count: mips,
                base_layer: layer,
                layer_count: layers,
            },
        ) => {
            let image =
                |aspects, base_mip, mip_count, base_layer, layer_count| NativeRange::Image {
                    aspects,
                    base_mip,
                    mip_count,
                    base_layer,
                    layer_count,
                };
            if aspects & !mask != 0 {
                pieces.push(image(
                    aspects & !mask,
                    base_mip,
                    mip_count,
                    base_layer,
                    layer_count,
                ));
            }
            if base_mip < mip {
                pieces.push(image(
                    mask,
                    base_mip,
                    mip - base_mip,
                    base_layer,
                    layer_count,
                ));
            }
            if mip + mips < base_mip + mip_count {
                pieces.push(image(
                    mask,
                    mip + mips,
                    base_mip + mip_count - mip - mips,
                    base_layer,
                    layer_count,
                ));
            }
            if base_layer < layer {
                pieces.push(image(mask, mip, mips, base_layer, layer - base_layer));
            }
            if layer + layers < base_layer + layer_count {
                pieces.push(image(
                    mask,
                    mip,
                    mips,
                    layer + layers,
                    base_layer + layer_count - layer - layers,
                ));
            }
        }
        _ => unreachable!("intersection kind"),
    }
    pieces
}

pub(super) fn key(range: NativeRange) -> (u32, u64, u64, u64, u64) {
    match range {
        NativeRange::Buffer { offset, size } => (0, offset, size, 0, 0),
        NativeRange::Image {
            aspects,
            base_mip,
            mip_count,
            base_layer,
            layer_count,
        } => (
            aspects,
            u64::from(base_mip),
            u64::from(base_layer),
            u64::from(mip_count),
            u64::from(layer_count),
        ),
    }
}

pub(super) fn adjacent(a: NativeRange, b: NativeRange) -> Option<NativeRange> {
    match (a, b) {
        (
            NativeRange::Buffer { offset, size },
            NativeRange::Buffer {
                offset: next,
                size: n,
            },
        ) if offset + size == next => Some(NativeRange::Buffer {
            offset,
            size: size + n,
        }),
        (
            NativeRange::Image {
                aspects: a,
                base_mip: am,
                mip_count: an,
                base_layer: al,
                layer_count: ac,
            },
            NativeRange::Image {
                aspects: b,
                base_mip: bm,
                mip_count: bn,
                base_layer: bl,
                layer_count: bc,
            },
        ) => {
            if a == b && am == bm && an == bn && al + ac == bl {
                Some(NativeRange::Image {
                    aspects: a,
                    base_mip: am,
                    mip_count: an,
                    base_layer: al,
                    layer_count: ac + bc,
                })
            } else if a == b && al == bl && ac == bc && am + an == bm {
                Some(NativeRange::Image {
                    aspects: a,
                    base_mip: am,
                    mip_count: an + bn,
                    base_layer: al,
                    layer_count: ac,
                })
            } else if a & b == 0 && am == bm && an == bn && al == bl && ac == bc {
                Some(NativeRange::Image {
                    aspects: a | b,
                    base_mip: am,
                    mip_count: an,
                    base_layer: al,
                    layer_count: ac,
                })
            } else {
                None
            }
        }
        _ => None,
    }
}

pub(super) fn coalesce(cells: &mut Vec<RangeCell>) {
    loop {
        cells.sort_unstable_by_key(|cell| key(cell.range));
        let before = cells.len();
        let mut written = 0usize;
        for index in 0..before {
            let next = cells[index];
            if written != 0
                && cells[written - 1].state == next.state
                && let Some(range) = adjacent(cells[written - 1].range, next.range)
            {
                cells[written - 1].range = range;
            } else {
                cells[written] = next;
                written += 1;
            }
        }
        cells.truncate(written);
        if before == written {
            break;
        }
    }
}
