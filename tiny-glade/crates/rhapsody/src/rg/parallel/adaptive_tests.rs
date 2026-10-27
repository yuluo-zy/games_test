use super::*;

const MODEL: ChunkStrategy = ChunkStrategy::Adaptive {
    max_chunks: 8,
    chunk_overhead_ns: 4_000,
    dispatch_overhead_ns: 27_000,
};
fn plan(costs: &[PassCost]) -> Chunks {
    let mut chunks = Chunks::new();
    plan_into(MODEL, costs, &[], &mut chunks);
    chunks
}

#[test]
fn light_batches_stay_in_one_inline_chunk() {
    // The old model split 6 x 3 us into six chunks that each cost more than the work.
    assert_eq!(plan(&[Some(3_000); 6]).as_slice(), [0..6]);
    assert_eq!(plan(&[Some(2_000); 8]).as_slice(), [0..8]);
    assert_eq!(plan(&[Some(10_000); 4]).as_slice(), [0..4]);
}

#[test]
fn unknown_costs_record_inline_until_the_first_frame_is_measured() {
    assert_eq!(plan(&[None; 54]).as_slice(), [0..54]);
    assert!(plan(&[]).is_empty());
}

#[test]
fn heavy_batches_still_split_and_partition_in_order() {
    let chunks = plan(&[Some(30_000); 54]);
    assert!(chunks.len() >= 2 && chunks.len() <= 8, "{chunks:?}");
    let mut next = 0;
    for chunk in &chunks {
        assert_eq!(chunk.start, next);
        assert!(!chunk.is_empty());
        next = chunk.end;
    }
    assert_eq!(next, 54);
}

#[test]
fn zero_chunk_overhead_uses_every_allowed_chunk() {
    let strategy = ChunkStrategy::Adaptive {
        max_chunks: 4,
        chunk_overhead_ns: 0,
        dispatch_overhead_ns: 0,
    };
    let mut chunks = Chunks::new();
    plan_into(strategy, &[Some(1_000); 8], &[], &mut chunks);
    assert_eq!(chunks.len(), 4);
}
