//! Range overlap and coverage helpers.
use super::super::resource::AccessRange;

pub fn ranges_overlap(a: AccessRange, b: AccessRange) -> bool {
    match (a, b) {
        (AccessRange::Whole, _) | (_, AccessRange::Whole) => true,
        (
            AccessRange::Buffer {
                offset: a,
                size: an,
            },
            AccessRange::Buffer {
                offset: b,
                size: bn,
            },
        ) => a < b.saturating_add(bn) && b < a.saturating_add(an),
        (
            AccessRange::Texture {
                aspect_mask: a,
                base_mip_level: am,
                level_count: an,
                base_array_layer: al,
                layer_count: ac,
            },
            AccessRange::Texture {
                aspect_mask: b,
                base_mip_level: bm,
                level_count: bn,
                base_array_layer: bl,
                layer_count: bc,
            },
        ) => {
            a & b != 0
                && am < bm.saturating_add(bn)
                && bm < am.saturating_add(an)
                && al < bl.saturating_add(bc)
                && bl < al.saturating_add(ac)
        }
        _ => false,
    }
}

pub(super) fn range_covers(a: AccessRange, b: AccessRange) -> bool {
    match (a, b) {
        (AccessRange::Whole, _) => true,
        (_, AccessRange::Whole) => false,
        (
            AccessRange::Buffer {
                offset: a,
                size: an,
            },
            AccessRange::Buffer {
                offset: b,
                size: bn,
            },
        ) => a <= b && a.saturating_add(an) >= b.saturating_add(bn),
        (
            AccessRange::Texture {
                aspect_mask: a,
                base_mip_level: am,
                level_count: an,
                base_array_layer: al,
                layer_count: ac,
            },
            AccessRange::Texture {
                aspect_mask: b,
                base_mip_level: bm,
                level_count: bn,
                base_array_layer: bl,
                layer_count: bc,
            },
        ) => {
            a & b == b
                && am <= bm
                && am.saturating_add(an) >= bm.saturating_add(bn)
                && al <= bl
                && al.saturating_add(ac) >= bl.saturating_add(bc)
        }
        _ => false,
    }
}
