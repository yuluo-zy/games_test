//! 多 pass 大图的并行分块（wave3）。
//!
//! 图：32 个互相独立的填充 pass（各写自己的缓冲，常量不同）+ 8 个拷贝 pass（读第 2k 个填充缓冲写拷贝目标，
//! 即跨块依赖，需要块间屏障），共 40 个 pass、默认 1 个 batch。比较：
//! - 串行（`ParallelRecording::SERIAL`）、`batch_per_pass`、均衡分块（ComputePool / Inline）的缓冲内容逐字节一致且等于期望值；
//! - 分块数与均衡切分算法一致（手算 + `plan_chunks` 双重核对）；
//! - `ExecutionStats` 的 `parallel_batches/chunks`、命令缓冲数、屏障计数；
//! - 默认 `ADAPTIVE` 在无统计或轻帧时走串行且只用一条命令缓冲。
mod draw_support;
mod wave3_support;
use rhapsody::rg::executor::PassPgoNameId;
use rhapsody::rg::parallel::{
    ChunkStrategy, ParallelRecording, TaskRunner, balanced_chunks, plan_chunks,
};
use wave3_support::*;

const FILLS: usize = 32;
const COPIES: usize = 8;
const TOTAL: usize = FILLS + COPIES;

fn seeded_durations() -> Vec<Option<u64>> {
    // 32 个填充各 10ns，8 个拷贝各 20ns：total=480。
    (0..TOTAL)
        .map(|i| Some(if i < FILLS { 10 } else { 20 }))
        .collect()
}

struct Observed {
    run: BigRun,
    threads: usize,
}

/// 新会话上跑一帧大图。`seed`：先写入上一帧统计。
fn run_once(
    parallel: ParallelRecording,
    seed: Option<&[Option<u64>]>,
    batch_per_pass: bool,
    salt: u32,
) -> Observed {
    let device = rhapsody::raw::init::create_headless_device().expect("设备");
    let mut session = new_session(&device, parallel);
    let bufs = BigBuffers::new(&mut session, FILLS, COPIES);
    if let Some(seed) = seed {
        seed_pgo(&mut session, &pass_names(FILLS, COPIES), seed);
    }
    take_thread_count();
    let run = run_big_graph(&mut session, &bufs, salt, batch_per_pass);
    Observed {
        threads: take_thread_count(),
        run,
    }
}

fn have_vulkan() -> bool {
    rhapsody::raw::init::create_headless_device().is_ok()
}

#[test]
fn all_chunking_strategies_agree_byte_for_byte_and_match_the_budget_algorithm() {
    if !have_vulkan() {
        return;
    }
    let salt = 0x5eed;
    let expected = expected_buffers(FILLS, COPIES, salt);
    let seed = seeded_durations();

    // 手算分块（独立于 plan_chunks 的实现路径）：8 块时最重块下界 480/8=60，连续切分恰好达到：
    // 填充每 6 个一块（5 块）、30..34 = 10+10+20+20、其后拷贝每 3 个一块。
    let total: u64 = seed.iter().flatten().sum();
    assert_eq!(total, 480);
    let hand: Vec<std::ops::Range<usize>> = {
        let mut v: Vec<_> = (0..5).map(|k| 6 * k..6 * k + 6).collect();
        v.extend([30..34, 34..37, 37..40]);
        v
    };
    let strategy = ChunkStrategy::Balanced {
        max_chunks: 8,
        chunk_overhead_ns: 0,
    };
    assert_eq!(balanced_chunks(&seed, 8, 0, &[]), hand, "balanced_chunks 与手算一致");

    // 1. 串行基线：不分块。
    let serial = run_once(ParallelRecording::SERIAL, None, false, salt);
    assert_eq!(
        serial.run.buffers, expected,
        "串行结果必须等于期望常量/拷贝"
    );
    let s = serial.run.receipt.stats;
    assert_eq!(
        (s.batches, s.parallel_batches, s.chunks),
        (1, 0, 0),
        "串行：1 个 batch、无并行分块"
    );
    let serial_commands = serial.run.receipt.stats.commands_recorded as usize;
    // 初始屏障和 pass 复用同一命令缓冲。
    assert_eq!(serial_commands, 1, "串行：每批次一条命令缓冲");

    // 2. batch_per_pass：每 pass 一个 batch，每个 batch 单块（无并行）。
    let per_pass = run_once(ParallelRecording::SERIAL, None, true, salt);
    assert_eq!(
        per_pass.run.buffers, expected,
        "batch_per_pass 结果与期望一致"
    );
    assert_eq!(per_pass.run.buffers, serial.run.buffers);
    let s = per_pass.run.receipt.stats;
    assert_eq!(
        (s.batches as usize, s.parallel_batches, s.chunks),
        (TOTAL, 0, 0)
    );
    assert_eq!(per_pass.run.receipt.snapshot.batches.len(), TOTAL);

    // 3. 块数不限、第一帧无统计：每个 pass 一块。
    let first = run_once(ParallelRecording::EVERY_PASS, None, false, salt);
    assert_eq!(first.run.buffers, expected);
    let s = first.run.receipt.stats;
    assert_eq!(
        (s.batches, s.parallel_batches, s.chunks as usize),
        (1, 1, TOTAL)
    );
    println!(
        "无统计：并行分块 {} 块，命令缓冲 {}，涉及线程 {}",
        s.chunks, first.run.receipt.stats.commands_recorded as usize, first.threads
    );

    // 4. 均衡分块 + 预置统计：8 块。ComputePool 与 Inline 两种 runner 都要一致。
    for runner in [TaskRunner::ComputePool, TaskRunner::Inline] {
        let parallel = ParallelRecording {
            strategy,
            runner,
            min_total_ns: 0,
        };
        let seeded = run_once(parallel, Some(&seed), false, salt);
        assert_eq!(
            seeded.run.buffers, serial.run.buffers,
            "{runner:?}：并行分块与串行逐字节一致"
        );
        let s = seeded.run.receipt.stats;
        assert_eq!(
            (s.batches, s.parallel_batches, s.chunks as usize),
            (1, 1, hand.len()),
            "{runner:?}"
        );
        // 命令缓冲数 = 块数 + 1（执行器先开的首命令，并行批次另开每块一条）。
        assert_eq!(
            seeded.run.receipt.stats.commands_recorded as usize,
            hand.len() + 1,
            "{runner:?} 命令缓冲数"
        );
        // 屏障数量：串行与并行共用同一套事件，必须相同。
        let ss = serial.run.receipt.stats;
        assert_eq!(
            (s.barrier_commands, s.barrier_transitions),
            (ss.barrier_commands, ss.barrier_transitions),
            "{runner:?} 屏障统计"
        );
        println!(
            "{runner:?}：{} 块、{} 条命令缓冲、屏障命令 {}、转换 {}、涉及线程 {}",
            s.chunks,
            seeded.run.receipt.stats.commands_recorded as usize,
            s.barrier_commands,
            s.barrier_transitions,
            seeded.threads
        );
    }
}

#[test]
fn the_second_frame_chunks_by_the_first_frames_measured_durations() {
    if !have_vulkan() {
        return;
    }
    let device = rhapsody::raw::init::create_headless_device().unwrap();
    // 默认 ADAPTIVE 有门槛，这里显式用无门槛的均衡配置以便确定走分块。
    let parallel = ParallelRecording {
        strategy: ChunkStrategy::Balanced {
            max_chunks: 4,
            chunk_overhead_ns: 0,
        },
        runner: TaskRunner::ComputePool,
        min_total_ns: 0,
    };
    let mut session = new_session(&device, parallel);
    let bufs = BigBuffers::new(&mut session, FILLS, COPIES);
    let names = pass_names(FILLS, COPIES);
    let first = run_big_graph(&mut session, &bufs, 1, false);
    assert_eq!(first.buffers, expected_buffers(FILLS, COPIES, 1));
    // 第一帧结束：end_frame 把本帧测得的耗时换成 previous，覆盖全部 40 个 pass。
    let previous = &session.chain.pgo.previous;
    assert!(
        names.iter().all(|n| previous.get(n).is_some()),
        "上一帧统计必须包含全部 pass"
    );
    let planned = plan_chunks(parallel.strategy, &names, previous, &[]);
    println!(
        "第二帧按实测统计预计 {} 块（total {} ns, max {} ns）",
        planned.len(),
        previous.total_ns(),
        previous.max_ns()
    );
    let second = run_big_graph(&mut session, &bufs, 2, false);
    assert_eq!(
        second.buffers,
        expected_buffers(FILLS, COPIES, 2),
        "第二帧内容必须是新的（没有读到上一帧残留）"
    );
    let s = second.receipt.stats;
    assert_eq!(
        s.chunks as usize,
        planned.len(),
        "实际块数与按 previous 统计的均衡切分一致"
    );
    assert_eq!(s.parallel_batches, u32::from(planned.len() > 1));
    // 同一设备、同一会话，再用串行配置跑同样内容（salt=2）对比字节。
    session.chain.parallel = ParallelRecording::SERIAL;
    let third = run_big_graph(&mut session, &bufs, 2, false);
    assert_eq!(
        third.buffers, second.buffers,
        "同一会话切到串行后结果逐字节一致"
    );
    assert_eq!(third.receipt.stats.parallel_batches, 0);
}

#[test]
fn balanced_and_adaptive_strategies_match_the_serial_bytes() {
    if !have_vulkan() {
        return;
    }
    let salt = 7;
    let serial = run_once(ParallelRecording::SERIAL, None, false, salt);
    let seed = seeded_durations();
    let names: Vec<PassPgoNameId> = pass_names(FILLS, COPIES);
    // Balanced：至多 4 块；有/无统计都应成立（无统计按 pass 数均分）。
    for (label, seeded) in [("无统计", None), ("有统计", Some(seed.as_slice()))] {
        let parallel = ParallelRecording {
            strategy: ChunkStrategy::Balanced {
                max_chunks: 4,
                chunk_overhead_ns: 0,
            },
            runner: TaskRunner::ComputePool,
            min_total_ns: 0,
        };
        let run = run_once(parallel, seeded, false, salt);
        assert_eq!(run.run.buffers, serial.run.buffers, "Balanced {label}");
        let s = run.run.receipt.stats;
        assert_eq!((s.batches, s.parallel_batches), (1, 1), "Balanced {label}");
        assert!(
            (2..=4).contains(&(s.chunks as usize)),
            "Balanced {label} 块数 {}",
            s.chunks
        );
        if seeded.is_some() {
            let mut probe = rhapsody::rg::executor::PassPgoStats::default();
            for (n, d) in names.iter().zip(&seed) {
                probe.record(rhapsody::rg::executor::PassPgoDuration::new(
                    n.clone(),
                    d.unwrap(),
                ));
            }
            let planned = plan_chunks(parallel.strategy, &names, &probe, &[]);
            assert_eq!(s.chunks as usize, planned.len());
        }
        assert_eq!(
            run.run.receipt.stats.commands_recorded as usize,
            s.chunks as usize + 1
        );
        println!("Balanced {label}：{} 块，线程 {}", s.chunks, run.threads);
    }
    // ADAPTIVE：轻帧（上一帧总耗时 < 200µs 门槛，或无统计）退回串行；预置超过门槛的统计则分块。
    let light = run_once(ParallelRecording::ADAPTIVE, None, false, salt);
    assert_eq!(light.run.buffers, serial.run.buffers);
    assert_eq!(
        light.run.receipt.stats.parallel_batches, 0,
        "ADAPTIVE 无统计时走串行"
    );
    assert_eq!(
        light.run.receipt.stats.commands_recorded, 1,
        "退回串行的非 Serial 策略复用首条命令缓冲"
    );
    let light = run_once(ParallelRecording::ADAPTIVE, Some(&seed), false, salt);
    assert_eq!(
        light.run.receipt.stats.parallel_batches, 0,
        "总耗时 480ns < 200µs，ADAPTIVE 仍走串行"
    );
    let heavy: Vec<Option<u64>> = seed.iter().map(|d| d.map(|d| d * 1000)).collect();
    let heavy_run = run_once(ParallelRecording::ADAPTIVE, Some(&heavy), false, salt);
    assert_eq!(heavy_run.run.buffers, serial.run.buffers);
    assert_eq!(
        heavy_run.run.receipt.stats.parallel_batches, 1,
        "总耗时 480µs ≥ 200µs 门槛，ADAPTIVE 分块"
    );
    let mut probe = rhapsody::rg::executor::PassPgoStats::default();
    for (n, d) in names.iter().zip(&heavy) {
        probe.record(rhapsody::rg::executor::PassPgoDuration::new(
            n.clone(),
            d.unwrap(),
        ));
    }
    assert_eq!(
        heavy_run.run.receipt.stats.chunks as usize,
        plan_chunks(ParallelRecording::ADAPTIVE.strategy, &names, &probe, &[]).len(),
    );
}
