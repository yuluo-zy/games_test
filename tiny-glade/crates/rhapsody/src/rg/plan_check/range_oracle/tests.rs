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
