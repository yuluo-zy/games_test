//! Generation retirement and shutdown.
use ash::vk;
use crate::Error;
use crate::Result;
use super::RETAINED_GENERATIONS;
use super::Swapchain;
use super::wsi::wsi_call;

impl Swapchain {
    /// Poll acquisition and presentation completion proofs of this and all
    /// retired generations, releasing completed ancestors. Non-blocking.
    pub fn collect_presentations(&self) -> Result<()> {
        let inner = self.inner.read();
        for slot in &inner.leases {
            let mut state = slot.state.lock();
            if state.acquired
                && wsi_call(&self.device.queues.timelines, || unsafe {
                    self.device.core.device.get_fence_status(slot.acquire_ready)
                })?
            {
                slot.finish_acquire(&mut state, 0)?;
            }
        }
        let mut new_generation_present_completed = false;
        for image in &inner.presentations {
            image.poll()?;
            new_generation_present_completed |= image.progress.lock().completed > 0;
        }
        drop(inner);
        let mut retired = self.retired_generations.lock();
        for previous in retired.iter() {
            previous.collect_presentations()?;
        }
        if new_generation_present_completed {
            // Completion of this generation's first present also proves that
            // the presentation engine has stopped using all older generations.
            for previous in retired.iter() {
                previous.confirm_generation_complete();
            }
            retired.clear();
        } else {
            // Each generation may also finish through its own present fences
            // or reacquire proofs. Acquire-only generations have no presents.
            let mut index = 0;
            while index < retired.len() {
                if retired[index].retirement_ready()? {
                    retired.swap_remove(index);
                } else {
                    index += 1;
                }
            }
        }
        Ok(())
    }

    fn confirm_generation_complete(&self) {
        let inner = self.inner.read();
        for image in &inner.presentations {
            let mut progress = image.progress.lock();
            progress.generation_completed();
        }
        for previous in self.retired_generations.lock().iter() {
            previous.confirm_generation_complete();
        }
    }

    /// Which proof stands in for present completion on this device.
    pub(super) fn present_proof(&self) -> super::policy::PresentProof {
        self.policy.present_proof(
            self.device
                .caps()
                .is_enabled(crate::device::Feature::SwapchainMaintenance1),
        )
    }

    /// Without present fences, submit an empty batch to the presentation queue
    /// and wait, bounded by `timeout`, for its timeline value. Queue work
    /// completes in submission order, so that value proves every earlier
    /// operation on the queue (including presents) has finished. Every issued
    /// present of this and all retired generations is then complete. The
    /// caller holds the WSI lock, so no present races the mark. A timeout or
    /// device loss returns an error before anything is marked complete.
    pub(super) fn prove_presents_by_queue_idle(&self, timeout: u64) -> Result<()> {
        let device = &self.device;
        let point = device.submit_prepared(
            device.queues.primary,
            &[crate::native::submit::PreparedSubmit {
                commands: Default::default(),
                wait_semaphores: Default::default(),
                wait_values: Default::default(),
                wait_stages: Default::default(),
                signal_semaphores: Default::default(),
                signal_values: Default::default(),
            }],
            vk::Fence::null(),
        )?;
        device.queues.timelines.wait(&point.into(), timeout)?;
        self.confirm_generation_complete();
        Ok(())
    }

    /// Stop new acquisitions and wait for actual acquire, GPU and presentation
    /// completion proofs. On failure the caller still owns this generation and
    /// may retry. With present fences no queue/device-idle result is used as a
    /// present proof; without them (no maintenance1) a queue-idle is.
    pub fn shutdown(&self, timeout: u64) -> Result<()> {
        let wsi = self.wsi.lock();
        let (leases, presentations) = {
            let mut inner = self.inner.write();
            if !inner.can_replace() {
                return Err(vk::Result::NOT_READY.into());
            }
            inner.closing = true;
            self.release_fullscreen(&mut inner)?;
            if inner.unproven_external_acquire {
                return Err(Error::Timeout);
            }
            (inner.leases.clone(), inner.presentations.clone())
        };
        // `closing` and the WSI lock exclude new acquisitions and presents;
        // the bounded waits below run without the state lock.
        for slot in &leases {
            slot.finish_acquire(&mut slot.state.lock(), timeout)?;
        }
        for image in &presentations {
            image.finish_gpu_use(timeout)?;
        }
        if self.present_proof() == super::policy::PresentProof::QueueIdle {
            self.prove_presents_by_queue_idle(timeout)?;
        }
        for image in &presentations {
            image.finish_shutdown(timeout)?;
        }
        drop(wsi);
        // A completed present in this generation can finish unextended older
        // generations; acquire-only generations cannot supply that proof.
        self.collect_presentations()?;
        for previous in self.retired_generations.lock().iter() {
            previous.shutdown(timeout)?;
        }
        Ok(())
    }

    fn retirement_ready(&self) -> Result<bool> {
        let inner = self.inner.read();
        if !inner.can_replace() {
            return Ok(false);
        }
        for slot in &inner.leases {
            let state = slot.state.lock();
            if state.acquired
                || (state.submitted
                    && !wsi_call(&self.device.queues.timelines, || unsafe {
                        self.device.core.device.get_fence_status(slot.fence)
                    })?)
            {
                return Ok(false);
            }
        }
        let completed = self.device.queues.timelines.poll()?;
        Ok(inner
            .presentations
            .iter()
            .all(|image| image.progress.lock().retirement_ready(&completed)))
    }

    /// Pending generations, including all ancestors retained by older chains.
    /// The replacement budget bounds this traversal to MAX_RETIRED_GENERATIONS.
    pub fn retired_generation_count(&self) -> usize {
        self.retired_generations
            .lock()
            .iter()
            .map(|previous| 1 + previous.retired_generation_count())
            .sum()
    }

    pub fn any_present_successful(&self) -> bool {
        self.inner.read().any_presented
    }

    /// 当前交换链代中 native present 返回 SUCCESS/SUBOPTIMAL 的次数；不代表扫描输出完成。
    pub fn native_present_success_count(&self) -> u64 {
        self.inner.read().native_present_successes
    }

    /// Generations permanently retained because teardown could not prove that
    /// all WSI operations completed. A successful strict shutdown leaves this
    /// process-wide diagnostic unchanged.
    pub fn retained_generation_count() -> u64 {
        RETAINED_GENERATIONS.load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn is_closing(&self) -> bool {
        self.inner.read().closing
    }

    /// Process-unique identity of this generation; recreation and surface
    /// replacement produce a new value.
    pub fn generation(&self) -> u64 {
        self.generation
    }
}
