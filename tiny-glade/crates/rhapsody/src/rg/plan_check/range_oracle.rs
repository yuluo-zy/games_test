//! Verifier-owned spatial state. Production range partitioning is deliberately
//! not used here: slabs are checked against a separate small dense-cell oracle.
use super::{Cell, invalid};
use crate::resource::OwnershipRange;

fn cells(range: OwnershipRange) -> crate::Result<Vec<Cell>> {
    match range {
        OwnershipRange::Buffer { offset, size } => {
            let end = offset
                .checked_add(size)
                .filter(|_| size != 0)
                .ok_or_else(|| invalid("native buffer range is empty or overflows"))?;
            Ok(vec![Cell([(offset, end), (0, 1), (0, 1)])])
        }
        OwnershipRange::Image {
            aspects,
            base_mip,
            mip_count,
            base_layer,
            layer_count,
        } => {
            let mip_end = base_mip.checked_add(mip_count).filter(|_| mip_count != 0);
            let layer_end = base_layer
                .checked_add(layer_count)
                .filter(|_| layer_count != 0);
            let (Some(mip_end), Some(layer_end)) = (mip_end, layer_end) else {
                return Err(invalid("native image range is empty or overflows"));
            };
            if aspects == 0 {
                return Err(invalid("native image range has no aspect"));
            }
            Ok((0..u32::BITS)
                .filter(|bit| aspects & (1 << bit) != 0)
                .map(|bit| {
                    Cell([
                        (u64::from(bit), u64::from(bit) + 1),
                        (u64::from(base_mip), u64::from(mip_end)),
                        (u64::from(base_layer), u64::from(layer_end)),
                    ])
                })
                .collect())
        }
    }
}
fn intersection(a: Cell, b: Cell) -> Option<Cell> {
    a.intersects(b).then(|| {
        Cell(std::array::from_fn(|axis| {
            (a.0[axis].0.max(b.0[axis].0), a.0[axis].1.min(b.0[axis].1))
        }))
    })
}
fn covered(mut missing: Vec<Cell>, coverage: impl IntoIterator<Item = Cell>) -> bool {
    for defined in coverage {
        missing = missing
            .into_iter()
            .flat_map(|cell| cell.without(defined))
            .collect();
    }
    missing.is_empty()
}

pub(super) fn contains(outer: OwnershipRange, inner: OwnershipRange) -> crate::Result<bool> {
    if matches!(outer, OwnershipRange::Image { .. })
        != matches!(inner, OwnershipRange::Image { .. })
    {
        return Ok(false);
    }
    Ok(covered(cells(inner)?, cells(outer)?))
}

/// Cold-path checker state; each row is a disjoint spatial region. State is
/// replaced transactionally so a rejected precondition cannot partly advance
/// the verifier, including when only a later row detects the mismatch.
#[derive(Clone)]
pub(super) struct SpatialLedger<T> {
    image: bool,
    bounds: Vec<Cell>,
    rows: Vec<(Cell, T)>,
}
impl<T: Clone> SpatialLedger<T> {
    pub(super) fn new(
        bounds: OwnershipRange,
        initial: impl IntoIterator<Item = (OwnershipRange, T)>,
    ) -> crate::Result<Self> {
        let image = matches!(bounds, OwnershipRange::Image { .. });
        let bounds = cells(bounds)?;
        let mut rows: Vec<(Cell, T)> = Vec::new();
        for (range, state) in initial {
            if image != matches!(range, OwnershipRange::Image { .. }) {
                return Err(invalid("native state input has the wrong resource type"));
            }
            for cell in cells(range)? {
                if !covered(vec![cell], bounds.iter().copied())
                    || rows.iter().any(|(old, _)| old.intersects(cell))
                {
                    return Err(invalid(
                        "native state input overlaps or leaves its resource bounds",
                    ));
                }
                rows.push((cell, state.clone()));
            }
        }
        if !covered(bounds.clone(), rows.iter().map(|(cell, _)| *cell)) {
            return Err(invalid("native state input does not cover its resource"));
        }
        Ok(Self {
            image,
            bounds,
            rows,
        })
    }
    fn checked_cells(&self, range: OwnershipRange) -> crate::Result<Vec<Cell>> {
        if self.image != matches!(range, OwnershipRange::Image { .. }) {
            return Err(invalid("native state query has the wrong resource type"));
        }
        let selected = cells(range)?;
        if !covered(selected.clone(), self.bounds.iter().copied()) {
            return Err(invalid("native state query leaves its resource bounds"));
        }
        Ok(selected)
    }
    pub(super) fn visit(
        &self,
        range: OwnershipRange,
        mut visit: impl FnMut(&T) -> crate::Result<()>,
    ) -> crate::Result<()> {
        let selected = self.checked_cells(range)?;
        for (cell, state) in &self.rows {
            if selected.iter().any(|selected| cell.intersects(*selected)) {
                visit(state)?;
            }
        }
        Ok(())
    }
    pub(super) fn update(
        &mut self,
        range: OwnershipRange,
        mut update: impl FnMut(&T) -> crate::Result<T>,
    ) -> crate::Result<()> {
        let selected = self.checked_cells(range)?;
        let mut next = self.rows.clone();
        for selected in selected {
            let mut replaced = Vec::new();
            for (cell, state) in next {
                if let Some(inside) = intersection(cell, selected) {
                    let new = update(&state)?;
                    replaced.extend(
                        cell.without(selected)
                            .into_iter()
                            .map(|outside| (outside, state.clone())),
                    );
                    replaced.push((inside, new));
                } else {
                    replaced.push((cell, state));
                }
            }
            next = replaced;
        }
        self.rows = next;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn buffer(offset: u64, size: u64) -> OwnershipRange {
        OwnershipRange::Buffer { offset, size }
    }
    fn image(aspects: u32, mip: u32, mips: u32, layer: u32, layers: u32) -> OwnershipRange {
        OwnershipRange::Image {
            aspects,
            base_mip: mip,
            mip_count: mips,
            base_layer: layer,
            layer_count: layers,
        }
    }
    #[test]
    fn buffer_slabs_match_every_dense_byte_after_overlapping_updates_and_rejected_patch() {
        let whole = buffer(0, 16);
        let mut ledger = SpatialLedger::new(whole, [(whole, 0u8)]).unwrap();
        let mut dense = [0u8; 16];
        for (start, size, value) in [(3, 8, 1), (0, 5, 2), (9, 7, 3), (4, 9, 4), (0, 16, 5)] {
            ledger.update(buffer(start, size), |_| Ok(value)).unwrap();
            dense[start as usize..(start + size) as usize].fill(value);
            for (byte, expected) in dense.iter().enumerate() {
                ledger
                    .visit(buffer(byte as u64, 1), |state| {
                        assert_eq!(state, expected);
                        Ok(())
                    })
                    .unwrap();
            }
        }
        ledger.update(buffer(4, 4), |_| Ok(8)).unwrap();
        dense[4..8].fill(8);
        assert!(
            ledger
                .update(whole, |old| if *old == 8 {
                    Err(invalid("late precondition mismatch"))
                } else {
                    Ok(9)
                })
                .is_err()
        );
        for (byte, expected) in dense.iter().enumerate() {
            ledger
                .visit(buffer(byte as u64, 1), |state| {
                    assert_eq!(state, expected);
                    Ok(())
                })
                .unwrap();
        }
        assert!(ledger.update(buffer(15, 2), |_| Ok(1)).is_err());
        assert!(ledger.visit(image(1, 0, 1, 0, 1), |_| Ok(())).is_err());
    }
    #[test]
    fn aspect_mip_layer_slabs_match_an_independent_dense_grid() {
        let whole = image(0b110, 0, 3, 0, 4);
        let mut ledger = SpatialLedger::new(whole, [(whole, 0u8)]).unwrap();
        let mut dense = [[[0u8; 4]; 3]; 2];
        for (aspects, mip, mips, layer, layers, value) in [
            (0b010, 1, 2, 0, 3, 1),
            (0b100, 0, 2, 1, 3, 2),
            (0b110, 1, 1, 2, 2, 3),
        ] {
            ledger
                .update(image(aspects, mip, mips, layer, layers), |_| Ok(value))
                .unwrap();
            for (aspect, plane) in dense.iter_mut().enumerate() {
                if aspects & (1 << (aspect + 1)) == 0 {
                    continue;
                }
                for row in &mut plane[mip as usize..(mip + mips) as usize] {
                    row[layer as usize..(layer + layers) as usize].fill(value);
                }
            }
            for (aspect, plane) in dense.iter().enumerate() {
                for (mip, row) in plane.iter().enumerate() {
                    for (layer, expected) in row.iter().enumerate() {
                        ledger
                            .visit(
                                image(1 << (aspect + 1), mip as u32, 1, layer as u32, 1),
                                |state| {
                                    assert_eq!(state, expected);
                                    Ok(())
                                },
                            )
                            .unwrap();
                    }
                }
            }
        }
        assert!(ledger.visit(image(1, 0, 1, 0, 1), |_| Ok(())).is_err());
        assert!(ledger.visit(image(2, 2, 2, 0, 1), |_| Ok(())).is_err());
        assert!(SpatialLedger::new(whole, [(image(2, 0, 3, 0, 4), 0)]).is_err());
        assert!(SpatialLedger::new(whole, [(whole, 0), (image(2, 0, 1, 0, 1), 1)]).is_err());
        assert!(SpatialLedger::new(image(1, 0, 1, 0, 1), [(buffer(0, 1), 0)]).is_err());
    }

    #[test]
    fn split_revisions_follow_each_byte_without_advancing_disjoint_cells() {
        let whole = buffer(0, 31);
        let mut ledger = SpatialLedger::new(whole, [(whole, 0u64)]).unwrap();
        let mut revisions = [0u64; 31];
        let mut seed = 0xf53e_29a7_u64;
        for _ in 0..300 {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            let start = seed as usize % revisions.len();
            let size = 1 + (seed >> 32) as usize % (revisions.len() - start);
            ledger
                .update(buffer(start as u64, size as u64), |old| Ok(old + 1))
                .unwrap();
            for revision in &mut revisions[start..start + size] {
                *revision += 1;
            }
            for (byte, revision) in revisions.iter().enumerate() {
                ledger
                    .visit(buffer(byte as u64, 1), |state| {
                        assert_eq!(state, revision);
                        Ok(())
                    })
                    .unwrap();
            }
        }
    }
}
