//! An explicitly separate timestamp experiment over the same renderer/input.
//! A command-span sum/max is not whole-frame latency or GPU active time.
use anyhow::{Context, Result};
use rhapsody::{rg::{GpuPassStatistics, GpuTimingConfig, GpuTimingStats, SessionFrameReceipt}, sync::SyncPoint};
use serde::Serialize;
use tiny_glade::render_bridge::CoreRenderBridge;

#[derive(Clone, Copy, Debug, Serialize)]
struct Point { queue: usize, value: u64, frame: usize }
#[derive(Clone, Copy, Debug, Serialize)]
struct Frame {
    world_tick: u64, receipt: u64, attempt: u64, measured: bool,
    expected_commands: u32,
}
#[derive(Clone, Copy, Debug, Serialize)]
struct Sample { recording_epoch: u64, queue: usize, completion_value: u64, duration_ns: u64 }
#[derive(Clone, Copy, Debug, Serialize)]
struct PassSample {
    recording_epoch: u64,
    queue: usize,
    completion_value: u64,
    duration_ns: u64,
    pass_index: usize,
    pass_name: &'static str,
    statistics: Option<GpuPassStatistics>,
}

fn after_drain(mut stats: GpuTimingStats, drained_this_call: u64) -> Option<GpuTimingStats> {
    // No second poll: it could harvest a new, undrained batch. The iterator is
    // fully consumed synchronously, so subtract exactly what it yielded.
    stats.retained_samples=stats.retained_samples.checked_sub(usize::try_from(drained_this_call).ok()?)?;
    (stats.retained_samples==0).then_some(stats)
}

pub struct GpuDiagnostic {
    frames: Vec<Frame>, points: Vec<Point>, samples: Vec<Sample>, pass_samples: Vec<PassSample>,
    frame_limit: usize, capacity: usize, drained: u64, valid: bool,
    stats: Option<GpuTimingStats>, last_poll_stats: Option<GpuTimingStats>, period_ns: f32, queue_bits: Vec<u32>,
    pass_spans: bool, isolate_passes: bool, pass_statistics: bool,
}
impl GpuDiagnostic {
    pub fn new(bridge: &mut CoreRenderBridge, device: &rhapsody::device::Device,
        frames: usize, capacity: usize, pass_spans: bool, isolate_passes: bool,
        pass_statistics: bool) -> Result<Self> {
        anyhow::ensure!(frames > 0 && capacity > 0 && capacity <= 2_000_000, "invalid GPU diagnostic capacity");
        anyhow::ensure!(pass_spans || (!isolate_passes && !pass_statistics),
            "GPU pass isolation/statistics require pass spans");
        let mut frame_rows=Vec::new(); frame_rows.try_reserve_exact(frames)?;
        let mut points=Vec::new(); points.try_reserve_exact(capacity)?;
        let mut samples=Vec::new(); samples.try_reserve_exact(capacity)?;
        let mut pass_samples=Vec::new(); pass_samples.try_reserve_exact(capacity)?;
        let families=unsafe { device.instance().raw_instance()
            .get_physical_device_queue_family_properties(device.physical_device()) };
        let queue_bits=device.queue_plan().queues.iter().map(|q| families[q.family as usize].timestamp_valid_bits)
            .collect::<Vec<_>>();
        let period_ns=device.properties().limits.timestamp_period;
        anyhow::ensure!(period_ns.is_finite() && period_ns > 0.0
            && queue_bits.iter().all(|b| (1..=64).contains(b)), "unusable GPU timestamp clock");
        // Configure exactly once before workload warmup. Pass spans remain a
        // separate diagnostic series and are never added to command spans.
        bridge.with_session(|s| s.backend.configure_gpu_timing(Some(GpuTimingConfig {
            pass_spans, isolate_passes, pass_statistics,
            pending_commands:4096, retained_samples:4096,
        })))?;
        Ok(Self {frames:frame_rows,points,samples,pass_samples,frame_limit:frames,capacity,drained:0,
            valid:true,stats:None,last_poll_stats:None,period_ns,queue_bits,
            pass_spans,isolate_passes,pass_statistics})
    }
    pub fn observe(&mut self, measured:bool,tick:u64,attempt:u64,receipt:&SessionFrameReceipt) {
        if self.frames.len() == self.frame_limit {self.valid=false;return;}
        let frame=self.frames.len();
        self.frames.push(Frame{world_tick:tick,receipt:receipt.frame,attempt,measured,
            expected_commands:receipt.stats.commands_recorded});
        if receipt.stats.commands_recorded == 0 || receipt.receipts.is_empty() {self.valid=false;}
        for &SyncPoint{queue,value} in &receipt.receipts {
            if self.points.len()==self.capacity {self.valid=false;continue;}
            self.points.push(Point{queue:queue.index(),value,frame});
        }
    }
    /// Nonblocking, bounded Copy capture. Never serialize or grow while running.
    pub fn drain(&mut self, bridge:&mut CoreRenderBridge) -> Result<()> {
        let drained_before=self.drained;
        let stats=bridge.with_session(|s| -> rhapsody::Result<_> {
            let stats=s.backend.poll_gpu_timings()?;
            for sample in s.backend.drain_gpu_timings() {
                match (sample.pass_index, sample.pass_name) {
                    (None, None) if sample.statistics.is_none() => {
                        if self.samples.len()==self.capacity {self.valid=false;}
                        else {self.samples.push(Sample{recording_epoch:sample.recording_epoch,queue:sample.queue,
                            completion_value:sample.completion_value,duration_ns:sample.duration_ns});}
                    }
                    (Some(pass_index), Some(pass_name)) if self.pass_spans && !pass_name.is_empty() => {
                        if self.pass_samples.len()==self.capacity {self.valid=false;}
                        else {self.pass_samples.push(PassSample{recording_epoch:sample.recording_epoch,
                            queue:sample.queue,completion_value:sample.completion_value,
                            duration_ns:sample.duration_ns,pass_index,pass_name,
                            statistics:sample.statistics});}
                    }
                    _ => self.valid=false,
                }
                match self.drained.checked_add(1) {Some(n)=>self.drained=n,None=>self.valid=false}
            }
            Ok(stats)
        })?.context("timestamp ring unexpectedly disabled")?;
        // completed includes every harvested pair, even samples evicted from
        // the retained ring. Exact equality therefore detects silent eviction.
        self.valid &= stats.capacity_skips==0 && stats.completed==self.drained;
        self.last_poll_stats=Some(stats);
        let after=self.drained.checked_sub(drained_before).and_then(|n| after_drain(stats,n));
        self.valid &= after.is_some();
        self.stats=after;
        Ok(())
    }
    pub fn report(&self) -> serde_json::Value {
        use std::collections::BTreeMap;
        let mut valid=self.valid && self.frames.len()==self.frame_limit
            && self.stats.is_some_and(|s| s.pending==0 && s.retained_samples==0 && s.capacity_skips==0
                && s.completed==self.drained)
            && (self.samples.len()+self.pass_samples.len()) as u64==self.drained;
        let mut owners=BTreeMap::new();
        for point in &self.points {
            if point.value==0 || point.queue>=self.queue_bits.len()
                || owners.insert((point.queue,point.value),point.frame).is_some() {valid=false;}
        }
        let associations=self.samples.iter().map(|s| owners.get(&(s.queue,s.completion_value)).copied())
            .collect::<Vec<_>>();
        let mut counts=vec![0u64;self.frames.len()];
        let mut epochs=vec![None;self.frames.len()];
        let mut pass_counts=vec![0u64;self.frames.len()];
        let mut pass_epochs=vec![None;self.frames.len()];
        let mut unique_passes=std::collections::BTreeSet::new();
        let mut queue_spans=BTreeMap::<(usize,usize),(u64,u64,u64)>::new();
        for (sample,owner) in self.samples.iter().zip(&associations) {
            let Some(frame)=*owner else {valid=false;continue;};
            counts[frame]+=1;
            if epochs[frame].is_some_and(|epoch| epoch!=sample.recording_epoch) {valid=false;}
            epochs[frame]=Some(sample.recording_epoch);
            let span=queue_spans.entry((frame,sample.queue)).or_default();
            match span.0.checked_add(sample.duration_ns) {Some(n)=>span.0=n,None=>valid=false}
            span.1=span.1.max(sample.duration_ns); span.2+=1;
        }
        let pass_associations=self.pass_samples.iter().map(|sample| {
            match owners.get(&(sample.queue,sample.completion_value)).copied() {
                Some(frame) => {
                    pass_counts[frame]+=1;
                    if pass_epochs[frame].is_some_and(|epoch| epoch!=sample.recording_epoch) {valid=false;}
                    pass_epochs[frame]=Some(sample.recording_epoch);
                    if !unique_passes.insert((frame,sample.pass_index)) {valid=false;}
                    if self.pass_statistics && sample.statistics.is_none() {valid=false;}
                    if !self.pass_statistics && sample.statistics.is_some() {valid=false;}
                    Some(frame)
                }
                None => {valid=false;None}
            }
        }).collect::<Vec<_>>();
        for (frame,count) in self.frames.iter().zip(&counts) {
            valid &= *count==u64::from(frame.expected_commands);
        }
        if self.pass_spans {
            valid &= pass_counts.iter().all(|count| *count>0);
        } else {
            valid &= self.pass_samples.is_empty();
        }
        // Keep every command, including multiple commands sharing one timeline
        // point. Grouped spans are optional summaries, never a substitute.
        serde_json::json!({"schema":"headless-gpu-command-diagnostic-v1","valid":valid,
            "acceptance_scope":"diagnostic_only_not_clean_cpu_or_whole_frame_gpu_latency",
            "pass_spans":self.pass_spans,"isolate_passes":self.isolate_passes,
            "pass_statistics":self.pass_statistics,
            "timestamp_period_ns":self.period_ns,"queue_timestamp_valid_bits":self.queue_bits,
            "capacity":self.capacity,"frame_capacity":self.frame_limit,
            "capture_capacity":self.capacity,"ring_pending_capacity":4096,"ring_retained_capacity":4096,
            "ring_stats":self.stats,"last_poll_stats_before_drain":self.last_poll_stats,
            "drained_samples":self.drained,"sample_count":self.samples.len(),
            "pass_sample_count":self.pass_samples.len(),
            "command_span_sum_and_max_are_not_frame_latency":true,
            "frames":self.frames,"receipt_points":self.points,"samples":self.samples,
            "sample_frame_indices":associations,"pass_samples":self.pass_samples,
            "pass_sample_frame_indices":pass_associations,
            "queue_command_spans":queue_spans.iter().map(|(&(frame,queue),&(sum,max,count))|
                serde_json::json!({"frame_index":frame,"queue":queue,"command_span_sum_ns":sum,
                    "command_span_max_ns":max,"commands":count})).collect::<Vec<_>>()})
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture()->GpuDiagnostic {
        GpuDiagnostic{frames:vec![Frame{world_tick:2,receipt:3,attempt:4,measured:true,expected_commands:2}],
            points:vec![Point{queue:0,value:9,frame:0}],
            samples:vec![Sample{recording_epoch:1,queue:0,completion_value:9,duration_ns:20},
                Sample{recording_epoch:1,queue:0,completion_value:9,duration_ns:30}],
            pass_samples:Vec::new(),pass_spans:false,isolate_passes:false,pass_statistics:false,
            frame_limit:1,capacity:4,drained:2,valid:true,period_ns:1.0,queue_bits:vec![64],
            stats:Some(GpuTimingStats{completed:2,..Default::default()}),last_poll_stats:None}
    }
    #[test]
    fn keeps_two_commands_on_same_point_and_does_not_claim_latency() {
        let report=fixture().report();assert_eq!(report["valid"],true);
        assert_eq!(report["samples"].as_array().unwrap().len(),2);
        assert_eq!(report["queue_command_spans"][0]["command_span_sum_ns"],50);
        assert_eq!(report["command_span_sum_and_max_are_not_frame_latency"],true);
    }
    #[test]
    fn missing_unmatched_evicted_or_skipped_samples_are_invalid() {
        let mut a=fixture();a.samples.pop();assert_eq!(a.report()["valid"],false);
        let mut a=fixture();a.samples[1].completion_value=10;assert_eq!(a.report()["valid"],false);
        let mut a=fixture();a.stats.as_mut().unwrap().completed=3;assert_eq!(a.report()["valid"],false);
        let mut a=fixture();a.stats.as_mut().unwrap().pending=1;assert_eq!(a.report()["valid"],false);
        let mut a=fixture();a.stats.as_mut().unwrap().capacity_skips=1;assert_eq!(a.report()["valid"],false);
        let mut a=fixture();a.samples[1].recording_epoch=2;assert_eq!(a.report()["valid"],false);
    }
    #[test]
    fn nonempty_final_drain_preserves_completed_and_records_exact_post_drain_retention() {
        let before=GpuTimingStats{completed:2,retained_samples:2,..Default::default()};
        let after=after_drain(before,2).unwrap();
        assert_eq!(after.completed,2); assert_eq!(after.retained_samples,0);
        let mut a=fixture();a.stats=Some(after);a.last_poll_stats=Some(before);
        assert_eq!(a.report()["valid"],true);
        assert_eq!(a.report()["last_poll_stats_before_drain"]["retained_samples"],2);
        assert!(after_drain(before,1).is_none());
        assert!(after_drain(before,3).is_none());
    }
    #[test]
    fn pass_spans_and_statistics_are_separate_from_command_counts() {
        let mut diagnostic=fixture();
        diagnostic.pass_spans=true;
        diagnostic.pass_statistics=true;
        diagnostic.pass_samples.push(PassSample{recording_epoch:1,queue:0,completion_value:9,
            duration_ns:40,pass_index:3,pass_name:"G-buffer",statistics:Some(GpuPassStatistics::default())});
        diagnostic.drained=3;
        diagnostic.stats=Some(GpuTimingStats{completed:3,..Default::default()});
        let report=diagnostic.report();
        assert_eq!(report["valid"],true);
        assert_eq!(report["pass_spans"],true);
        assert_eq!(report["pass_statistics"],true);
        assert_eq!(report["sample_count"],2);
        assert_eq!(report["pass_sample_count"],1);
        assert_eq!(report["pass_sample_frame_indices"],serde_json::json!([0]));
    }
}
