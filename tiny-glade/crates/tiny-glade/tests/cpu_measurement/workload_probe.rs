//! Fixed-size receipt observations. Serialize only after the measured interval.
use country_core::render::declare_probe::{self, CollectCounts};
use rhapsody::rg::ExecutionStats;
use rhapsody::profile::CommandWorkload;
use rhapsody::device::TextureUploadProbeSnapshot;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Default, Serialize)]
pub struct UploadSnapshot {
    /// Process-local identity only; never part of paired scene equivalence.
    pub manager_id: u64,
    pub submitted_jobs: u64,
    pub submitted_payload_bytes: u64,
}
impl From<(u64, u64, u64)> for UploadSnapshot {
    fn from((manager_id, submitted_jobs, submitted_payload_bytes): (u64, u64, u64)) -> Self {
        Self { manager_id, submitted_jobs, submitted_payload_bytes }
    }
}
fn upload_delta(before: UploadSnapshot, after: UploadSnapshot) -> Option<[u64; 2]> {
    if (before.manager_id == 0 && (before.submitted_jobs != 0 || before.submitted_payload_bytes != 0))
        || (after.manager_id == 0 && (after.submitted_jobs != 0 || after.submitted_payload_bytes != 0))
        || (before.manager_id != 0 && before.manager_id != after.manager_id)
    { return None; }
    Some([after.submitted_jobs.checked_sub(before.submitted_jobs)?,
        after.submitted_payload_bytes.checked_sub(before.submitted_payload_bytes)?])
}
fn collect_delta(before: CollectCounts, after: CollectCounts) -> Option<CollectCounts> {
    if !before.valid || !after.valid || before.overflow || after.overflow { return None; }
    let delta = CollectCounts {
        tasks: after.tasks.checked_sub(before.tasks)?,
        waves: after.waves.checked_sub(before.waves)?,
        cross_pass_tasks: after.cross_pass_tasks.checked_sub(before.cross_pass_tasks)?,
        cross_pass_waves: after.cross_pass_waves.checked_sub(before.cross_pass_waves)?,
        failed_waves: after.failed_waves.checked_sub(before.failed_waves)?,
        overflow: false, valid: true,
    };
    (delta.cross_pass_tasks <= delta.tasks && delta.cross_pass_waves <= delta.waves
        && delta.failed_waves == 0).then_some(delta)
}

#[derive(Clone, Copy, Debug, Serialize)]
pub struct FrameWorkload {
    pub world_tick: u64,
    pub receipt: u64,
    pub attempt: u64,
    pub valid: bool,
    pub native_commands: CommandWorkload,
    pub graph_commands_recorded: u32,
    pub graph_recording_chunks: u32,
    pub graph_parallel_batches: u32,
    pub graph_batches: u32,
    pub graph_retained_passes: usize,
    pub graph_culled_passes: u32,
    pub draw_list_lengths: [usize; 21],
    pub collect: CollectCounts,
    pub collect_cumulative: CollectCounts,
    pub queued_buffer_h2d_submitted_jobs: u64,
    pub queued_buffer_h2d_submitted_payload_bytes: u64,
    pub queued_upload_snapshot: UploadSnapshot,
    pub device_upload_submitted_payload: Option<TextureUploadProbeSnapshot>,
    pub device_upload_snapshot: TextureUploadProbeSnapshot,
}

pub struct WorkloadProbe {
    frames: Vec<FrameWorkload>,
    limit: usize,
    previous_collect: CollectCounts,
    previous_upload: Option<UploadSnapshot>,
    previous_receipt: Option<(u64, u64, u64)>,
    valid: bool,
    owns_collect: bool,
    final_collect: Option<CollectCounts>,
    baseline_upload: Option<UploadSnapshot>,
    previous_device_upload: TextureUploadProbeSnapshot,
    baseline_device_upload: TextureUploadProbeSnapshot,
    issues: u32,
}
impl WorkloadProbe {
    pub fn new(limit: usize, upload: rhapsody::Result<(u64, u64, u64)>,
        device_upload: TextureUploadProbeSnapshot) -> anyhow::Result<Self> {
        let mut frames = Vec::new();
        frames.try_reserve_exact(limit)?;
        let owns_collect = declare_probe::collect_counts_begin();
        let previous_collect = declare_probe::collect_counts_snapshot();
        let previous_upload = upload.ok().map(Into::into);
        Ok(Self { frames, limit, previous_collect, previous_upload,
            previous_receipt: None, valid: owns_collect && previous_collect.valid && previous_upload.is_some() && device_upload.valid,
            owns_collect, final_collect: None, baseline_upload: previous_upload,
            previous_device_upload: device_upload, baseline_device_upload: device_upload,
            issues: u32::from(!owns_collect || !previous_collect.valid)
                | (u32::from(previous_upload.is_none()) << 1)
                | (u32::from(!device_upload.valid) << 2) })
    }
    /// Observe every warmup frame too, so the first measured row is a per-frame
    /// delta, never the cumulative warmup cost. A failure is sticky session-wide.
    pub fn observe(&mut self, measured: bool, tick: u64, receipt: u64, attempt: u64,
        stats: ExecutionStats, completion_points: usize, retained_passes: usize,
        draw_list_lengths: [usize;21], upload: rhapsody::Result<(u64, u64, u64)>,
        device_upload: TextureUploadProbeSnapshot) {
        let now = declare_probe::collect_counts_snapshot();
        let collected = collect_delta(self.previous_collect, now);
        let uploaded = upload.ok().map(UploadSnapshot::from);
        let delta = self.previous_upload.zip(uploaded).and_then(|(a, b)| upload_delta(a, b));
        let device_delta = device_upload.checked_delta(self.previous_device_upload);
        let ordered = self.previous_receipt.is_none_or(|(t, r, a)| tick > t && receipt > r && attempt == a + 1);
        self.issues |= (u32::from(!ordered) << 3)
            | (u32::from(!stats.command_workload.valid) << 4)
            | (u32::from(collected.is_none()) << 5)
            | (u32::from(delta.is_none()) << 6)
            | (u32::from(device_delta.is_none()) << 7)
            | (u32::from(completion_points == 0 || stats.commands_recorded == 0) << 8);
        self.valid &= ordered && stats.command_workload.valid && collected.is_some()
            && delta.is_some() && device_delta.is_some() && completion_points > 0 && stats.commands_recorded > 0;
        self.previous_collect = now;
        self.previous_upload = uploaded;
        self.previous_device_upload = device_upload;
        self.previous_receipt = Some((tick, receipt, attempt));
        if !measured { self.baseline_upload = uploaded; self.baseline_device_upload = device_upload; return; }
        if self.frames.len() == self.limit { self.valid = false; self.issues |= 1 << 9; return; }
        let [jobs, bytes] = delta.unwrap_or([0, 0]);
        self.frames.push(FrameWorkload { world_tick: tick, receipt, attempt, valid: self.valid,
            native_commands: stats.command_workload, graph_commands_recorded: stats.commands_recorded,
            graph_recording_chunks: stats.chunks, graph_parallel_batches: stats.parallel_batches,
            graph_batches: stats.batches, collect: collected.unwrap_or_default(),
            collect_cumulative: now,
            graph_retained_passes: retained_passes, graph_culled_passes: stats.culled_passes,
            draw_list_lengths,
            queued_buffer_h2d_submitted_jobs: jobs, queued_buffer_h2d_submitted_payload_bytes: bytes,
            queued_upload_snapshot: uploaded.unwrap_or_default(),
            device_upload_submitted_payload: device_delta, device_upload_snapshot: device_upload });
    }
    pub fn finish(&mut self) {
        if self.owns_collect {
            let end = declare_probe::collect_counts_end();
            let no_unobserved_tail = end.tasks == self.previous_collect.tasks
                && end.waves == self.previous_collect.waves
                && end.cross_pass_tasks == self.previous_collect.cross_pass_tasks
                && end.cross_pass_waves == self.previous_collect.cross_pass_waves;
            let end_valid = end.valid && !end.overflow && end.failed_waves == 0 && no_unobserved_tail;
            self.valid &= end_valid;
            self.issues |= u32::from(!end_valid) << 10;
            self.final_collect = Some(end);
            self.owns_collect = false;
        } else { self.valid = false; }
    }
    pub fn valid(&self) -> bool { self.valid && self.frames.len() == self.limit && self.final_collect.is_some() }
    pub fn report(&self) -> serde_json::Value {
        let issue_names = ["collect_begin_or_initial_snapshot_invalid", "initial_upload_snapshot_failed",
            "initial_device_snapshot_invalid", "receipt_tick_attempt_not_monotonic", "native_command_workload_invalid",
            "collect_overflow_failure_or_invalid_delta", "queued_upload_error_reset_or_counter_regression",
            "device_upload_invalid_generation_or_regression", "missing_completion_points_or_recorded_commands",
            "frame_capacity_exhausted", "collect_end_failure_or_unobserved_tail"];
        let mut errors=issue_names.iter().enumerate().filter_map(|(i,name)|
            (self.issues & (1 << i) != 0).then_some(*name)).collect::<Vec<_>>();
        if self.frames.len()!=self.limit { errors.push("insufficient_observation_frames"); }
        serde_json::json!({"schema":"headless-actual-workload-v1", "valid":self.valid(),
            "errors":errors,
            "scope":"fixed App/session owner; full successful graph receipts; native API calls are not GPU draw counts",
            "coverage":{"graph_copy_scope":"recorded commands in complete successful graph submission, not GPU completion",
                "queued_upload_scope":"same BufferHost manager successful queued buffer H2D submit payload; includes successful prefix only, but any frame failure invalidates run",
                "device_upload_scope":"device-lifetime successful native texture setup/direct/transfer and buffer setup payload bytes; separate counters, not GPU completion or padded staging capacity",
                "included":["graph_staging_h2d_copy_recorded_bytes","graph_immutable_producer_h2d_copy_recorded_bytes","graph_cached_d2d_copy_recorded_bytes","queued_buffer_h2d_submitted_payload_bytes","device_texture_setup_submitted_payload_bytes","device_texture_direct_region_submitted_payload_bytes","device_texture_transfer_submitted_payload_bytes","device_buffer_setup_submitted_payload_bytes"],
                "excluded":["mapped_cpu_writes","device_to_host_readback","clear_fill_commands"],
                "no_all_upload_total":true,
                "collect_cross_pass_is_subset":true,
                "graph_chunks_are_not_collect_tasks":true},
            "identity_policy":"fixed BufferHost Arc; lazy manager id 0 may become nonzero once; nonzero manager replacement invalidates; ids are process-local and excluded from paired semantic identity",
            "frame_policy":"samples+1 full frame observations; row zero establishes CPU-recorder baseline and still reports only that frame workload; no warmup cumulative attributed to row zero",
            "capacity":self.limit, "frame_count":self.frames.len(), "truncated":self.frames.len()!=self.limit,
            "queued_upload_before_measurement":self.baseline_upload,
            "queued_upload_end":self.previous_upload, "collect_session_end":self.final_collect,
            "device_upload_before_measurement":self.baseline_device_upload,
            "device_upload_end":self.previous_device_upload,
            "frames":self.frames})
    }
}
impl Drop for WorkloadProbe {
    fn drop(&mut self) {
        if self.owns_collect { let _ = declare_probe::collect_counts_end(); }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn upload(id: u64, jobs: u64, bytes: u64) -> UploadSnapshot { (id, jobs, bytes).into() }
    #[test]
    fn upload_rejects_replacement_reset_and_backwards_even_with_same_totals() {
        assert_eq!(upload_delta(upload(0,0,0),upload(7,2,80)),Some([2,80]));
        assert_eq!(upload_delta(upload(7,2,80),upload(7,3,144)),Some([1,64]));
        for after in [upload(8,2,80), upload(0,0,0), upload(7,1,80), upload(7,2,79)] {
            assert!(upload_delta(upload(7,2,80),after).is_none());
        }
        assert!(upload_delta(upload(0,1,0),upload(7,2,80)).is_none());
    }
    #[test]
    fn collect_rejects_overflow_failures_and_non_subset_counts() {
        let before = CollectCounts { valid:true, tasks:20, waves:4, cross_pass_tasks:8,
            cross_pass_waves:2, ..Default::default() };
        let after = CollectCounts { tasks:23,waves:5,cross_pass_tasks:10,cross_pass_waves:3,..before };
        let delta=collect_delta(before,after).unwrap();
        assert_eq!((delta.tasks,delta.waves,delta.cross_pass_tasks),(3,1,2));
        for bad in [CollectCounts{overflow:true,..after},CollectCounts{valid:false,..after},
            CollectCounts{failed_waves:1,..after},CollectCounts{tasks:19,..after},
            CollectCounts{cross_pass_tasks:12,..after}] { assert!(collect_delta(before,bad).is_none()); }
    }
    fn device_snapshot() -> TextureUploadProbeSnapshot {
        TextureUploadProbeSnapshot { generation:1, valid:true, ..Default::default() }
    }
    fn stats() -> ExecutionStats {
        ExecutionStats {commands_recorded:1, ..Default::default()}
    }
    #[test]
    fn warmup_is_baseline_not_first_measured_upload_and_capacity_is_strict() {
        let device=device_snapshot();
        let mut probe=WorkloadProbe::new(1,Ok((0,0,0)),device).unwrap();
        probe.observe(false,1,1,1,stats(),1,2,[0;21],Ok((7,100,800)),device);
        probe.observe(true,2,2,2,stats(),1,2,[0;21],Ok((7,102,832)),device);
        probe.finish();
        let report=probe.report();
        assert_eq!(report["valid"],true);
        assert_eq!(report["frames"][0]["queued_buffer_h2d_submitted_jobs"],2);
        assert_eq!(report["frames"][0]["queued_buffer_h2d_submitted_payload_bytes"],32);
        assert_eq!(report["queued_upload_before_measurement"]["submitted_jobs"],100);
    }
    #[test]
    fn missing_receipt_or_native_failure_sticks_even_after_a_good_frame() {
        for failure in ["missing_receipt","invalid_native"] {
            let device=device_snapshot();
            let mut probe=WorkloadProbe::new(1,Ok((7,0,0)),device).unwrap();
            let mut bad=stats();
            if failure=="invalid_native" {bad.command_workload.valid=false;}
            probe.observe(false,1,1,1,bad,usize::from(failure!="missing_receipt"),2,[0;21],Ok((7,1,8)),device);
            probe.observe(true,2,2,2,stats(),1,2,[0;21],Ok((7,2,16)),device);
            probe.finish();
            assert_eq!(probe.report()["valid"],false);
            assert!(!probe.report()["errors"].as_array().unwrap().is_empty());
        }
    }
    #[test]
    fn insufficient_or_overflow_observation_is_invalid_without_silent_tail_drop() {
        let device=device_snapshot();
        let mut insufficient=WorkloadProbe::new(1,Ok((7,0,0)),device).unwrap();
        insufficient.finish();
        assert_eq!(insufficient.report()["valid"],false);
        let mut overflow=WorkloadProbe::new(1,Ok((7,0,0)),device).unwrap();
        overflow.observe(true,1,1,1,stats(),1,2,[0;21],Ok((7,1,8)),device);
        overflow.observe(true,2,2,2,stats(),1,2,[0;21],Ok((7,2,16)),device);
        overflow.finish();
        assert_eq!(overflow.report()["valid"],false);
        assert_eq!(overflow.report()["frame_count"],1);
        assert!(overflow.report()["errors"].as_array().unwrap().iter().any(|s|s=="frame_capacity_exhausted"));
    }
}
