//! 并行录制的均衡分块算法与双缓冲 PGO 统计。
use rhapsody::rg::{
    executor::{PassPgoDuration, PassPgoFrames, PassPgoNameId, PassPgoStats},
    parallel::{ChunkStrategy, balanced_chunks, plan_chunks},
};

fn ids(n: usize) -> Vec<PassPgoNameId> {
    (0..n)
        .map(|i| PassPgoNameId::from_name(&format!("p{i}")))
        .collect()
}
fn stats(durations: &[Option<u64>]) -> (Vec<PassPgoNameId>, PassPgoStats) {
    let names = ids(durations.len());
    let mut s = PassPgoStats::default();
    for (name, d) in names.iter().zip(durations) {
        if let Some(d) = d {
            s.record(PassPgoDuration::new(name.clone(), *d));
        }
    }
    (names, s)
}

#[test]
fn heavy_pass_at_the_end_gets_its_own_chunk() {
    // 31 个耗时 1 的 pass + 末尾 1 个耗时 1000：重 pass 单独成块，其余轻 pass 可并行。
    let mut d = vec![Some(1); 31];
    d.push(Some(1000));
    let (names, s) = stats(&d);
    let balanced = plan_chunks(
        ChunkStrategy::Balanced {
            max_chunks: 4,
            chunk_overhead_ns: 0,
        },
        &names,
        &s,
        &[],
    );
    assert!(balanced.len() >= 2);
    assert_eq!(balanced.last().unwrap(), &(31..32));
}

#[test]
fn unlimited_chunks_without_statistics_split_every_pass() {
    let (names, empty) = stats(&[None; 6]);
    let chunks = plan_chunks(
        rhapsody::rg::parallel::ParallelRecording::EVERY_PASS.strategy,
        &names,
        &empty,
        &[],
    );
    assert_eq!(chunks, (0..6).map(|i| i..i + 1).collect::<Vec<_>>());
}

#[test]
fn chunks_always_partition_the_batch_in_order() {
    for pattern in [
        vec![Some(7); 33],
        (0..40)
            .map(|i| Some(1 + (i * 37 % 91) as u64))
            .collect::<Vec<_>>(),
        vec![None; 9],
        vec![Some(0); 5],
    ] {
        let (names, s) = stats(&pattern);
        for strategy in [ChunkStrategy::Balanced {
            max_chunks: 4,
            chunk_overhead_ns: 3,
        }] {
            let chunks = plan_chunks(strategy, &names, &s, &[]);
            let mut next = 0;
            for c in &chunks {
                assert_eq!(c.start, next, "{strategy:?} 块必须首尾相接");
                assert!(!c.is_empty(), "{strategy:?} 不得有空块");
                next = c.end;
            }
            assert_eq!(next, names.len(), "{strategy:?} 必须覆盖整个 batch");
        }
    }
}

#[test]
fn split_forbidden_boundaries_are_never_cut() {
    // 禁止在 pass 1、3 之后切开（例如不得切断同一 render pass 实例）。
    let costs = [4, 4, 4, 4, 4, 4].map(Some);
    let allowed = [true, false, true, false, true, true];
    for chunk in balanced_chunks(&costs, 6, 0, &allowed) {
        for forbidden in [1usize, 3] {
            assert_ne!(chunk.end, forbidden + 1);
        }
    }
}

#[test]
fn balanced_respects_max_chunks_and_minimizes_the_heaviest_chunk() {
    let costs = [9, 1, 1, 1, 1, 1, 1, 9, 1, 1].map(Some);
    let chunks = balanced_chunks(&costs, 3, 0, &[]);
    assert!(chunks.len() <= 3);
    let load = |c: &std::ops::Range<usize>| c.clone().map(|i| costs[i].unwrap()).sum::<u64>();
    let heaviest = chunks.iter().map(load).max().unwrap();
    // 总和 26、最重单个 pass 9：3 块的最优最大块为 9（[9,1] [1,1,1,1,1,1] [9,1,1]=11? 需 ≤ 最优）。
    // 用暴力枚举所有 ≤3 块的连续切分验证最优性。
    let n = costs.len();
    let mut best = u64::MAX;
    for a in 1..n {
        for b in a..=n {
            let parts: Vec<std::ops::Range<usize>> = if b == n {
                vec![0..a, a..n]
            } else {
                vec![0..a, a..b, b..n]
            };
            best = best.min(parts.iter().map(load).max().unwrap());
        }
    }
    assert_eq!(heaviest, best);
}

#[test]
fn balanced_estimates_unknown_costs_from_the_known_mean_and_caps_chunks() {
    // 第一帧完全无数据：按 pass 数均分，且受 max_chunks 限制。
    let (names, empty) = stats(&[None; 12]);
    let chunks = plan_chunks(
        ChunkStrategy::Balanced {
            max_chunks: 4,
            chunk_overhead_ns: 0,
        },
        &names,
        &empty,
        &[],
    );
    assert_eq!(chunks.len(), 4);
    assert!(chunks.iter().all(|c| c.len() == 3));
}

#[test]
fn pgo_double_buffer_uses_only_the_previous_frame_and_never_accumulates() {
    let name = PassPgoNameId::from_name("a");
    let mut frames = PassPgoFrames::default();
    frames
        .current
        .record(PassPgoDuration::new(name.clone(), 100));
    // 当前帧的数据在本帧内不可见于分块（分块读 previous）。
    assert_eq!(frames.previous.total_ns(), 0);
    frames.end_frame();
    assert_eq!(
        (frames.previous.total_ns(), frames.previous.max_ns()),
        (100, 100)
    );
    assert_eq!(frames.current.total_ns(), 0);
    // 下一帧结束：previous 变成第二帧的数据，第一帧的 100 不残留（统计不跨帧累积）。
    frames
        .current
        .record(PassPgoDuration::new(name.clone(), 30));
    frames.end_frame();
    assert_eq!(
        (frames.previous.total_ns(), frames.previous.max_ns()),
        (30, 30)
    );
    assert_eq!(frames.previous.duration_ns(&name), 30);
    assert_eq!(frames.current.call_count(&name), 0);
}

#[test]
fn stats_extend_overwrites_per_name_but_accumulates_total_and_max() {
    // 同名覆盖单项耗时，总和与最大值累计。
    let a = PassPgoNameId::from_name("a");
    let b = PassPgoNameId::from_name("b");
    let mut s = PassPgoStats::default();
    s.extend([
        PassPgoDuration::new(a.clone(), 10),
        PassPgoDuration::new(b.clone(), 70),
        PassPgoDuration::new(a.clone(), 5),
    ]);
    assert_eq!(s.total_ns(), 85);
    assert_eq!(s.max_ns(), 70);
    assert_eq!(s.duration_ns(&a), 5, "同名后者覆盖前者");
    assert_eq!(s.duration_ns(&b), 70);
    s.extend(std::iter::empty());
    assert_eq!(s.total_ns(), 85);
}
