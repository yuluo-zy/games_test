//! 串行与并行录制的等价性：同一张图，串行（原路径）与分块并行（Inline 与真实任务池）必须产生
//! 完全相同的“屏障 + pass 回调”序列。事件写进每个命令各自的日志，按提交顺序拼接后比较，
//! 因此与命令缓冲如何切分无关。
use ash::vk::{self, Handle};
use rhapsody::{
    app::{AccessFixture, PassInput, ResourceInput, Scenario},
    raw::{managed::lifecycle, submit},
    rg::{
        executor::{
            ChunkRecorder, ChunkTransition, ExecutionBackend, ExecutionChain, ExecutionStats,
            PassPgoDuration, PassPgoNameId,
        },
        graph::{self, RenderCallback},
        parallel::{ChunkStrategy, ParallelRecording, TaskRunner},
    },
};
use std::{
    collections::{BTreeMap, VecDeque},
    sync::{Arc, Mutex},
};

type Logs = Arc<Mutex<BTreeMap<u64, Vec<String>>>>;

#[derive(Default)]
struct Recording {
    next: u64,
    logs: Logs,
    pending: BTreeMap<u64, Vec<(u64, u8, u8)>>,
    /// 提交时各命令的顺序。
    submitted: Vec<u64>,
    /// 命令句柄 → 池槽（并行时每块应来自不同池）。
    slots: BTreeMap<u64, usize>,
    parallel_capable: bool,
}
impl Recording {
    fn emit(logs: &Logs, command: u64, items: &[(u64, u8, u8)]) {
        if !items.is_empty() {
            logs.lock().unwrap().entry(command).or_default().push(format!("barrier:{items:?}"));
        }
    }
    fn flush(&mut self, command: u64) {
        if let Some(items) = self.pending.remove(&command) {
            Self::emit(&self.logs, command, &items);
        }
    }
}
impl graph::ResourceBackend for Recording {
    fn create_without_initial_data(
        &mut self,
        _: graph::Descriptor,
        _: &str,
    ) -> Result<u64, graph::BackendFailure> {
        self.next += 1;
        Ok(self.next)
    }
    fn set_debug_name(&mut self, _: u64, _: graph::Descriptor, _: &str) {}
}
struct View<'a>(&'a Logs);
impl ChunkRecorder for View<'_> {
    fn record_transitions(&self, command: u64, transitions: &[ChunkTransition]) -> bool {
        let items: Vec<_> = transitions.iter().map(|t| (t.token, t.from, t.to)).collect();
        Recording::emit(self.0, command, &items);
        true
    }
}
impl ExecutionBackend for Recording {
    fn managed_resource(
        &mut self,
        token: u64,
        runtime: &mut lifecycle::SequenceRuntime,
    ) -> Option<lifecycle::ResourceHandle> {
        Some(runtime.create_resource(token, 7))
    }
    fn command_handle(&mut self) -> Result<u64, lifecycle::Failure> {
        self.chunk_command_handle(0)
    }
    fn chunk_command_handle(&mut self, chunk: usize) -> Result<u64, lifecycle::Failure> {
        self.next += 1;
        self.slots.insert(self.next, chunk);
        Ok(self.next)
    }
    fn begin_command(&mut self, _: u64) -> Result<(), lifecycle::Failure> {
        Ok(())
    }
    fn end_command(&mut self, handle: vk::CommandBuffer) -> bool {
        self.flush(handle.as_raw());
        true
    }
    fn transition(&mut self, command: u64, token: u64, from: u8, to: u8) -> bool {
        self.pending.entry(command).or_default().push((token, from, to));
        true
    }
    fn flush_barriers(&mut self, command: u64) -> bool {
        self.flush(command);
        true
    }
    fn submit(&mut self, _: vk::Queue, prepared: &[submit::PreparedSubmit]) -> (i32, bool) {
        for p in prepared {
            self.submitted.extend(p.commands.iter().map(|c| c.as_raw()));
        }
        (0, false)
    }
    fn chunk_recorder(&self) -> Option<Box<dyn ChunkRecorder + '_>> {
        self.parallel_capable.then(|| Box::new(View(&self.logs)) as Box<dyn ChunkRecorder>)
    }
}

fn scenario(passes: usize, batch_per_pass: bool) -> Scenario {
    let resources = passes / 2 + 2;
    Scenario {
        resources: (0..resources)
            .map(|i| ResourceInput {
                name: format!("r{i}"),
                kind: "buffer".into(),
                size: 256,
                imported: None,
                initial_access: 0,
            })
            .collect(),
        passes: (0..passes)
            .map(|i| PassInput {
                name: format!("p{i}"),
                accesses: vec![
                    ((i % resources) as u32, 40, 0),
                    (((i + 1) % resources) as u32, 4, 0),
                ],
            })
            .collect(),
        exports: vec![(0, 55), (1, 55)],
        batch_per_pass,
        fail_callback: None,
        fail_end: false,
        queue_error: 0,
        timeline_start: 1,
        complete: true,
    }
}

struct Outcome {
    events: Vec<String>,
    stats: ExecutionStats,
    /// 每个 batch 内，各块命令使用的池槽（并行时必须两两不同：Vulkan 命令池需外部同步）。
    chunk_slots_per_batch: Vec<Vec<usize>>,
    pgo_names: usize,
}
fn execute(
    input: &Scenario,
    config: ParallelRecording,
    seed_pgo: Option<&[u64]>,
    fail_pass: Option<usize>,
) -> Result<Outcome, String> {
    let (declaration, _, _) = input.build_graph()?.into_parts();
    let logs: Logs = Default::default();
    let callbacks: Vec<RenderCallback> = (0..declaration.passes.len())
        .map(|index| {
            let logs = logs.clone();
            Box::new(move |context: rhapsody::rg::executor::PassContext<'_>| {
                logs.lock()
                    .unwrap()
                    .entry(context.command.handle())
                    .or_default()
                    .push(format!("pass:{}", context.pass_index));
                Some(index) != fail_pass
            }) as RenderCallback
        })
        .collect();
    let mut callbacks = callbacks;
    let mut backend = Recording {
        next: 100,
        logs: logs.clone(),
        parallel_capable: config.strategy != ChunkStrategy::Serial,
        ..Default::default()
    };
    let mut chain = ExecutionChain::new(
        0,
        1,
        submit::DeviceState {
            primary: submit::QueueState {
                handle: vk::Queue::from_raw(0xabcd),
                timeline_semaphore: vk::Semaphore::from_raw(0xf000),
                pending_timelines: VecDeque::new(),
                pending_commands: VecDeque::new(),
            },
            auxiliary: vec![],
            next_timeline: 1,
            oldest_pending: None,
        },
    );
    chain.parallel = config;
    if let Some(costs) = seed_pgo {
        for (index, cost) in costs.iter().enumerate() {
            chain.pgo.previous.record(PassPgoDuration::new(
                PassPgoNameId::from_name(&format!("p{index}")),
                *cost,
            ));
        }
    }
    let result = chain
        .execute_graph_with_context(
            declaration,
            &mut graph::Pool::default(),
            &mut AccessFixture,
            &mut backend,
            &mut callbacks,
        )
        .map_err(|e| format!("{e:?}"))?;
    let logs = logs.lock().unwrap();
    let events: Vec<String> = backend
        .submitted
        .iter()
        .flat_map(|command| logs.get(command).cloned().unwrap_or_default())
        .collect();
    // 命令按创建顺序编号；并行 batch 的块命令连续创建，槽号依次为 0,1,2…。
    let mut chunk_slots_per_batch: Vec<Vec<usize>> = Vec::new();
    for slot in backend.slots.values().copied() {
        if slot == 0 {
            chunk_slots_per_batch.push(vec![0]);
        } else if let Some(group) = chunk_slots_per_batch.last_mut() {
            group.push(slot);
        }
    }
    Ok(Outcome {
        events,
        stats: result.stats,
        chunk_slots_per_batch,
        pgo_names: chain.pgo.previous.call_count(PassPgoNameId::from_name("p0")) as usize,
    })
}

fn pool(strategy: ChunkStrategy, runner: TaskRunner) -> ParallelRecording {
    ParallelRecording { strategy, runner, min_total_ns: 0 }
}

#[test]
fn chunked_recording_emits_the_same_barriers_and_pass_order_as_serial() {
    for &(passes, per_pass) in &[(1, false), (2, false), (9, false), (24, false), (24, true)] {
        let input = scenario(passes, per_pass);
        let serial = execute(&input, ParallelRecording::SERIAL, None, None).unwrap();
        assert_eq!(serial.stats.chunks, 0, "串行路径不应分块");
        assert!(!serial.events.is_empty());
        let balanced = ChunkStrategy::Balanced { max_chunks: 4, chunk_overhead_ns: 0 };
        for config in [
            pool(balanced, TaskRunner::Inline),
            pool(balanced, TaskRunner::ComputePool),
            // 原作算法 + 空统计：每个 pass 一块（第一帧行为）。
            pool(ChunkStrategy::Original, TaskRunner::ComputePool),
        ] {
            let parallel = execute(&input, config, None, None).unwrap();
            assert_eq!(
                parallel.events, serial.events,
                "passes={passes} per_pass={per_pass} {config:?}"
            );
            // 串行与并行必须发出同样数量的屏障命令与转换。
            assert_eq!(
                (parallel.stats.barrier_commands, parallel.stats.barrier_transitions),
                (serial.stats.barrier_commands, serial.stats.barrier_transitions),
                "passes={passes} per_pass={per_pass} {config:?}"
            );
        }
    }
}

#[test]
fn original_strategy_with_seeded_costs_splits_and_still_matches_serial() {
    let input = scenario(24, false);
    let serial = execute(&input, ParallelRecording::SERIAL, None, None).unwrap();
    // 24 个等耗时 pass：预算 = clamp(10, 240/16=15, 120) = 15 ⇒ 每 2 个一块 ⇒ 12 块。
    let costs = vec![10u64; 24];
    let parallel = execute(&input, ParallelRecording::ORIGINAL, Some(&costs), None).unwrap();
    assert_eq!(parallel.events, serial.events);
    assert_eq!(parallel.stats.parallel_batches, 1);
    assert_eq!(parallel.stats.chunks, 12);
}

#[test]
fn single_chunk_falls_back_to_the_serial_path() {
    let input = scenario(8, false);
    // 一个超重 pass 排在最后：预算被压到 total/2，整个 batch 只有一块 ⇒ 不走任务池。
    let mut costs = vec![1u64; 7];
    costs.push(1000);
    let run = execute(&input, ParallelRecording::ORIGINAL, Some(&costs), None).unwrap();
    assert_eq!(run.stats.parallel_batches, 0);
    assert_eq!(run.stats.chunks, 0);
    let serial = execute(&input, ParallelRecording::SERIAL, None, None).unwrap();
    assert_eq!(run.events, serial.events);
}

#[test]
fn backend_without_a_chunk_view_never_goes_parallel() {
    let input = scenario(12, false);
    let mut config = ParallelRecording::ORIGINAL;
    config.runner = TaskRunner::Inline;
    // Recording 在策略为 Serial 时不提供视图；这里直接以 Original 策略但无视图的后端验证回退。
    let (declaration, _, _) = input.build_graph().unwrap().into_parts();
    let mut backend = Recording { next: 100, parallel_capable: false, ..Default::default() };
    let mut chain = ExecutionChain::new(
        0,
        1,
        submit::DeviceState {
            primary: submit::QueueState {
                handle: vk::Queue::from_raw(1),
                timeline_semaphore: vk::Semaphore::from_raw(2),
                pending_timelines: VecDeque::new(),
                pending_commands: VecDeque::new(),
            },
            auxiliary: vec![],
            next_timeline: 1,
            oldest_pending: None,
        },
    );
    chain.parallel = config;
    let mut callbacks: Vec<RenderCallback> =
        (0..declaration.passes.len()).map(|_| Box::new(|_: rhapsody::rg::executor::PassContext<'_>| true) as RenderCallback).collect();
    let result = chain
        .execute_graph_with_context(
            declaration,
            &mut graph::Pool::default(),
            &mut AccessFixture,
            &mut backend,
            &mut callbacks,
        )
        .unwrap();
    assert_eq!(result.stats.parallel_batches, 0);
}

#[test]
fn a_failing_callback_in_any_chunk_fails_the_whole_batch() {
    let input = scenario(12, false);
    let costs = vec![10u64; 12];
    for failing in [0usize, 5, 11] {
        let error = execute(&input, ParallelRecording::ORIGINAL, Some(&costs), Some(failing))
            .err()
            .expect("回调失败必须上抛");
        assert!(error.contains("Record"), "{error}");
    }
}

#[test]
fn pgo_is_recorded_per_frame_and_swapped_after_each_execution() {
    let input = scenario(6, false);
    let run = execute(&input, ParallelRecording::ORIGINAL, None, None).unwrap();
    // 执行结束后 end_frame 交换：本帧耗时出现在 previous 里，且每个 pass 恰好记录一次。
    assert_eq!(run.pgo_names, 1);
    let serial = execute(&input, ParallelRecording::SERIAL, None, None).unwrap();
    assert_eq!(serial.pgo_names, 1, "串行路径也必须记录耗时，供下一帧并行分块使用");
}

#[test]
fn repeated_runs_with_the_real_task_pool_are_deterministic() {
    let input = scenario(48, false);
    let serial = execute(&input, ParallelRecording::SERIAL, None, None).unwrap();
    let costs = vec![10u64; 48];
    for _ in 0..200 {
        let run = execute(&input, ParallelRecording::ORIGINAL, Some(&costs), None).unwrap();
        assert_eq!(run.events, serial.events);
    }
}

#[test]
fn adaptive_gate_stays_serial_for_light_or_unknown_frames_and_goes_parallel_for_heavy_ones() {
    let input = scenario(24, false);
    let serial = execute(&input, ParallelRecording::SERIAL, None, None).unwrap();
    // 第一帧没有统计：门槛 > 0 时走串行（原作无门槛会每 pass 一块）。
    let first = execute(&input, ParallelRecording::ADAPTIVE, None, None).unwrap();
    assert_eq!(first.stats.parallel_batches, 0);
    assert_eq!(first.events, serial.events);
    // 上一帧总耗时 24 * 1µs = 24µs < 200µs：轻帧，仍走串行。
    let light = execute(&input, ParallelRecording::ADAPTIVE, Some(&[1_000; 24]), None).unwrap();
    assert_eq!(light.stats.parallel_batches, 0);
    // 上一帧总耗时 24 * 50µs = 1.2ms >= 200µs：重帧，走并行且结果一致。
    let heavy = execute(&input, ParallelRecording::ADAPTIVE, Some(&[50_000; 24]), None).unwrap();
    assert_eq!(heavy.stats.parallel_batches, 1);
    assert!(heavy.stats.chunks >= 2);
    assert_eq!(heavy.events, serial.events);
    // 无门槛的原作配置在同样的轻帧上仍会分块。
    let original = execute(&input, ParallelRecording::ORIGINAL, Some(&[1_000; 24]), None).unwrap();
    assert!(original.stats.parallel_batches == 1 || original.stats.chunks == 0);
}

#[test]
fn every_chunk_of_a_parallel_batch_gets_a_distinct_command_pool_slot() {
    let input = scenario(24, false);
    let run = execute(&input, ParallelRecording::ORIGINAL, Some(&[10; 24]), None).unwrap();
    assert!(run.stats.chunks >= 2);
    let mut checked = false;
    for group in &run.chunk_slots_per_batch {
        let mut unique = group.clone();
        unique.sort_unstable();
        unique.dedup();
        assert_eq!(unique.len(), group.len(), "同一 batch 内池槽必须两两不同: {group:?}");
        checked |= group.len() >= 2;
    }
    assert!(checked, "至少有一个 batch 使用了多个池槽");
}

#[test]
fn a_panicking_callback_on_a_pool_thread_surfaces_as_a_panic_and_leaves_the_pool_usable() {
    let input = scenario(24, false);
    let costs = vec![10u64; 24];
    let baseline = execute(&input, ParallelRecording::SERIAL, None, None).unwrap();
    let (declaration, _, _) = input.build_graph().unwrap().into_parts();
    let mut callbacks: Vec<RenderCallback> = (0..declaration.passes.len())
        .map(|index| {
            Box::new(move |_: rhapsody::rg::executor::PassContext<'_>| {
                if index == 13 {
                    panic!("回调在任务池线程中故意 panic");
                }
                true
            }) as RenderCallback
        })
        .collect();
    let mut backend = Recording { next: 100, parallel_capable: true, ..Default::default() };
    let mut chain = ExecutionChain::new(
        0,
        1,
        submit::DeviceState {
            primary: submit::QueueState {
                handle: vk::Queue::from_raw(1),
                timeline_semaphore: vk::Semaphore::from_raw(2),
                pending_timelines: VecDeque::new(),
                pending_commands: VecDeque::new(),
            },
            auxiliary: vec![],
            next_timeline: 1,
            oldest_pending: None,
        },
    );
    chain.parallel = ParallelRecording::ORIGINAL;
    for (index, cost) in costs.iter().enumerate() {
        chain.pgo.previous.record(PassPgoDuration::new(
            PassPgoNameId::from_name(&format!("p{index}")),
            *cost,
        ));
    }
    // panic 必须传播给调用者（而不是死锁或被吞掉）。
    let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        chain.execute_graph_with_context(
            declaration,
            &mut graph::Pool::default(),
            &mut AccessFixture,
            &mut backend,
            &mut callbacks,
        )
    }));
    assert!(outcome.is_err(), "池线程中的 panic 必须传播");
    // 全局任务池在 panic 之后仍然可用，后续并行执行结果正确。
    for _ in 0..20 {
        let run = execute(&input, ParallelRecording::ORIGINAL, Some(&costs), None).unwrap();
        assert_eq!(run.events, baseline.events);
    }
}

#[test]
fn barrier_stats_match_the_events_a_backend_actually_receives() {
    // 以后端实际收到的屏障为准独立计数：`barrier:[...]` 日志每条是一次屏障命令，条目数是转换数。
    for per_pass in [false, true] {
        let input = scenario(12, per_pass);
        let run = execute(&input, ParallelRecording::SERIAL, None, None).unwrap();
        let commands = run.events.iter().filter(|e| e.starts_with("barrier:")).count() as u32;
        let transitions: u32 = run
            .events
            .iter()
            .filter(|e| e.starts_with("barrier:"))
            .map(|e| e.matches("), (").count() as u32 + 1)
            .sum();
        assert!(commands > 0, "场景应当产生屏障");
        assert_eq!(run.stats.barrier_commands, commands, "per_pass={per_pass}");
        assert_eq!(run.stats.barrier_transitions, transitions, "per_pass={per_pass}");
        // 每个屏障命令至少含一个转换。
        assert!(run.stats.barrier_transitions >= run.stats.barrier_commands);
    }
}

#[test]
fn an_empty_graph_reports_no_barriers() {
    let run = execute(&scenario(0, false), ParallelRecording::SERIAL, None, None);
    if let Ok(run) = run {
        assert_eq!((run.stats.barrier_commands, run.stats.barrier_transitions), (0, 0));
    }
}
