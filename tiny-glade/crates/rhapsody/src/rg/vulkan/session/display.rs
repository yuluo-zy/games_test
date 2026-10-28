//! Display-frame pacing and in-flight throttling.
use super::DisplayFrameId;
use super::VulkanGraphSession;

impl VulkanGraphSession {
    /// 等待在途提交全部完成。
    /// wait_frame 对每个在途帧执行有界等待，超时以 Err 返回。
    pub fn wait_submits_done(&mut self, timeout_ns: u64) -> crate::Result<()> {
        while let Some(pending) = self.pending.front() {
            let frame = pending.frame;
            self.wait_frame(frame, timeout_ns)?;
        }
        if self.result_cache.has_work() {
            let completed = self.backend.device.poll()?;
            self.collect_cached_results(&completed)?;
        }
        self.collect_frame_imports();
        self.backend.device.try_collect_all_completed()?;
        Ok(())
    }

    /// 配置限制在飞显示帧数量；只等待最旧帧，不等待其他会话或无关队列。
    pub(super) fn throttle_in_flight(&mut self) -> Result<(), crate::Error> {
        let limit = self.backend.device.frames_in_flight() as usize;
        while self.pending.in_flight_groups(|pending| pending.display) >= limit {
            let frame = self.pending.front().unwrap().frame;
            self.wait_frame(frame, 10_000_000_000)?;
        }
        Ok(())
    }

    pub(super) fn allocate_display(&mut self) -> u64 {
        let display = self.next_display;
        self.next_display += 1;
        display
    }

    /// Opens a display frame and throttles once. Graphs declared with
    /// [`Self::begin_frame`] before [`Self::end_display_frame`] (for example the
    /// main graph and the presentation graph) share this one in-flight slot, so
    /// CPU work for frame N+1 can overlap GPU work for frame N.
    ///
    /// A display frame left open by an aborted frame is replaced, not an error.
    pub fn begin_display_frame(&mut self) -> Result<DisplayFrameId, crate::Error> {
        self.active_display = None;
        self.backend.display_frames = true;
        self.throttle_in_flight()?;
        if self.is_poisoned() {
            return Err(crate::Error::DeviceLost);
        }
        let display = self.allocate_display();
        self.active_display = Some(display);
        Ok(DisplayFrameId(display))
    }

    /// Closes the explicit display frame. Idempotent.
    pub fn end_display_frame(&mut self) {
        self.active_display = None;
    }

    pub fn has_active_display_frame(&self) -> bool {
        self.active_display.is_some()
    }
}
