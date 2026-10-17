//! 并行录制的分块算法：原作预算公式（PE 0x1402c21fb..0x1402c2220）、贪心切分（0x1402c22a0..0x1402c22e6）
//! 与双缓冲 PGO（end_frame 0x140bec400）。证据：evidence/render-recovery/parallel-recording/FINDINGS.md。
use rhapsody::rg::{
    executor::{PassPgoDuration, PassPgoFrames, PassPgoNameId, PassPgoStats},
    parallel::{ChunkStrategy, original_budget, original_chunks, plan_chunks},
};

#[cfg(feature = "candidate-optimizations")]
use rhapsody::rg::parallel::balanced_chunks;

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
fn budget_is_max_clamped_between_total_over_16_and_total_over_2() {
    // max 落在区间内：取 max。
    assert_eq!(original_budget(1600, 300), 300);
    // max 太小：抬到 total/16。
    assert_eq!(original_budget(1600, 10), 100);
    // max 太大：压到 total/2。
    assert_eq!(original_budget(1600, 1500), 800);
    // 无数据：预算为 0。
    assert_eq!(original_budget(0, 0), 0);
}

#[test]
fn original_greedy_closes_a_chunk_when_accumulated_reaches_budget() {
    // 预算 5：累计 3,3→6 关块；再 2,2,2→6 关块；剩余 1 单独成块。
    let costs = [3, 3, 2, 2, 2, 1].map(Some);
    assert_eq!(original_chunks(&costs, 5, &[]), vec![0..2, 2..5, 5..6]);
    // 恰好等于预算也关块（`>=`）。
    let costs = [5, 5, 5].map(Some);
    assert_eq!(original_chunks(&costs, 5, &[]), vec![0..1, 1..2, 2..3]);
}

#[test]
fn missing_durations_count_as_zero() {
    // 缺失记 0：它们并入后面第一个有耗时的块。
    let costs = [None, None, Some(10), None, Some(10)];
    assert_eq!(original_chunks(&costs, 10, &[]), vec![0..3, 3..5]);
}

#[test]
fn first_frame_without_stats_makes_every_pass_its_own_chunk() {
    // total=max=0 ⇒ budget=0 ⇒ 累计 0 >= 0 恒真 ⇒ 每个 pass 一块（原作第一帧行为的推断）。
    let (names, empty) = stats(&[None; 6]);
    let chunks = plan_chunks(ChunkStrategy::Original, &names, &empty, &[]);
    assert_eq!(chunks, (0..6).map(|i| i..i + 1).collect::<Vec<_>>());
}

#[test]
fn steady_state_produces_between_two_and_sixteen_chunks() {
    // 64 个等耗时 pass：budget = clamp(10, 640/16=40, 320) = 40 ⇒ 每块 4 个 ⇒ 16 块。
    let (names, s) = stats(&[Some(10); 64]);
    let chunks = plan_chunks(ChunkStrategy::Original, &names, &s, &[]);
    assert_eq!(chunks.len(), 16);
    assert!(chunks.iter().all(|c| c.len() == 4));
}

#[test]
fn heavy_pass_at_the_end_swallows_everything_before_it_under_the_original_budget() {
    // 31 个耗时 1 的 pass + 末尾 1 个耗时 1000：total=1031, max=1000，
    // budget = clamp(1000, 1031/16=64, 1031/2=515) = 515。前 31 个累计只有 31，到不了预算，
    // 切分点只会出现在累计超过预算之后 ⇒ 整个 batch 只有 1 块，轻 pass 完全无法并行。
    let mut d = vec![Some(1); 31];
    d.push(Some(1000));
    let (names, s) = stats(&d);
    let original = plan_chunks(ChunkStrategy::Original, &names, &s, &[]);
    assert_eq!(original, vec![0..32]);
    // 改进版（候选）把重 pass 单独切出，其余轻 pass 可并行。
    #[cfg(feature = "candidate-optimizations")]
    {
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
}

#[test]
fn heavy_pass_in_front_gets_its_own_chunk_under_the_original_budget() {
    // 重 pass 在最前：它一到就达到预算，独占一块；其余轻 pass 在预算 total/16 内累计。
    let mut d = vec![Some(1000)];
    d.extend(vec![Some(1); 31]);
    let (names, s) = stats(&d);
    let chunks = plan_chunks(ChunkStrategy::Original, &names, &s, &[]);
    assert_eq!(chunks[0], 0..1);
    assert!(chunks.len() >= 2);
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
        for strategy in [
            ChunkStrategy::Serial,
            ChunkStrategy::Original,
            #[cfg(feature = "candidate-optimizations")]
            ChunkStrategy::Balanced {
                max_chunks: 4,
                chunk_overhead_ns: 3,
            },
        ] {
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
    // 禁止在 pass 1、3 之后切开（候选约束：不得切断同一 render pass 实例）。
    let costs = [4, 4, 4, 4, 4, 4].map(Some);
    let allowed = [true, false, true, false, true, true];
    for chunk in original_chunks(&costs, 4, &allowed) {
        for forbidden in [1usize, 3] {
            assert_ne!(chunk.end, forbidden + 1, "不得在被禁止的边界切开");
        }
    }
    #[cfg(feature = "candidate-optimizations")]
    for chunk in balanced_chunks(&costs, 6, 0, &allowed) {
        for forbidden in [1usize, 3] {
            assert_ne!(chunk.end, forbidden + 1);
        }
    }
}

#[cfg(feature = "candidate-optimizations")]
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

#[cfg(feature = "candidate-optimizations")]
#[test]
fn balanced_estimates_unknown_costs_from_the_known_mean_and_caps_chunks() {
    // 第一帧完全无数据：按 pass 数均分，且受 max_chunks 限制（原作此时会每 pass 一块）。
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
    // record_batch 内联体（0x1402c1d40）：insert 覆盖 + total += d + max = max(max, d)。
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
    assert_eq!(s.duration_ns(&a), 5, "同名后者覆盖前者（hashbrown insert）");
    assert_eq!(s.duration_ns(&b), 70);
    s.extend(std::iter::empty());
    assert_eq!(s.total_ns(), 85);
}
