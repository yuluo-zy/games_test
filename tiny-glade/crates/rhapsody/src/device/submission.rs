//! 所有生产队列调用的共同入口：每队列锁、成功提交后确认时间线。
use crate::raw::{device::VulkanDevice, submit::PreparedSubmit};
use crate::sync::{QueueId, SyncPoint};
use crate::{Error, Result};
use ash::vk;

impl VulkanDevice {
    pub fn queue_id(&self, queue: vk::Queue) -> Result<QueueId> {
        let index = if queue == self.queue {
            0
        } else {
            self.additional_queues
                .iter()
                .position(|q| q.handle == queue)
                .map(|i| i + 1)
                .ok_or_else(|| Error::invalid_usage("队列不属于设备"))?
        };
        QueueId::new(index).ok_or_else(|| Error::invalid_usage("队列数超过支持上限"))
    }

    pub(crate) fn submit_prepared(
        &self,
        queue: vk::Queue,
        batches: &[PreparedSubmit],
        fence: vk::Fence,
    ) -> Result<SyncPoint> {
        let id = self.queue_id(queue)?;
        for batch in batches {
            for semaphore in &batch.signal_semaphores {
                for index in 0..self.timelines.queue_count() {
                    let own = self
                        .timelines
                        .semaphore(QueueId::new(index).expect("已验证的队列数"))?;
                    if *semaphore == own {
                        return Err(Error::invalid_usage(
                            "设备时间线只允许由统一提交入口 signal",
                        ));
                    }
                }
            }
            if batch.wait_semaphores.len() != batch.wait_values.len()
                || batch.wait_semaphores.len() != batch.wait_stages.len()
                || batch.signal_semaphores.len() != batch.signal_values.len()
                || batch.wait_stages.iter().any(|stage| stage.is_empty())
            {
                return Err(Error::invalid_usage("提交的信号量/值/阶段数量不匹配"));
            }
        }
        let (point, ()) = self.timelines.submit(id, |signal| {
            let mut batches = batches.to_vec();
            if batches.is_empty() {
                batches.push(PreparedSubmit::default());
            }
            // 同一次提交的最后一个批次附带时间线，不额外插入空 SubmitInfo。
            let last = batches.last_mut().expect("空输入已补入同步批次");
            last.signal_semaphores.push(signal.semaphore);
            last.signal_values.push(signal.value);
            let mut timelines = batches
                .iter()
                .map(|b| {
                    vk::TimelineSemaphoreSubmitInfo::default()
                        .wait_semaphore_values(&b.wait_values)
                        .signal_semaphore_values(&b.signal_values)
                })
                .collect::<Vec<_>>();
            let infos = batches
                .iter()
                .zip(&mut timelines)
                .map(|(b, t)| {
                    vk::SubmitInfo::default()
                        .wait_semaphores(&b.wait_semaphores)
                        .wait_dst_stage_mask(&b.wait_stages)
                        .signal_semaphores(&b.signal_semaphores)
                        .command_buffers(&b.commands)
                        .push_next(t)
                })
                .collect::<Vec<_>>();
            unsafe { self.device.queue_submit(queue, &infos, fence) }.map_err(Into::into)
        })?;
        Ok(point)
    }

    pub(crate) fn queue_wait_idle(&self, queue: vk::Queue) -> Result<()> {
        self.timelines.with_queue(self.queue_id(queue)?, || {
            unsafe { self.device.queue_wait_idle(queue) }.map_err(Into::into)
        })
    }
}
