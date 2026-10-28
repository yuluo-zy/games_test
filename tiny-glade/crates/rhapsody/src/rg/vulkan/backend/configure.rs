//! Runtime configuration of budgets, arenas, aliasing and timing.
use super::TimelineSignal;
use super::TimelineWait;
use super::VulkanGraphBackend;

impl VulkanGraphBackend {
    pub fn set_managed_projection_limits(
        &mut self,
        config: crate::rg::pass_bindings::ManagedProjectionConfig,
    ) -> crate::Result<()> {
        if self.device.is_device_lost() {
            return Err(crate::Error::DeviceLost);
        }
        let replacement = crate::rg::pass_bindings::ManagedProjectionPool::new(
            self.device.generation(),
            self.device.memory.stats.clone(),
            config,
        )?;
        let mut projection = self.caches.projection();
        projection.snapshot.take();
        projection.pool = replacement;
        Ok(())
    }

    /// Physical aliasing of transient resources (on by default). Aliasing only
    /// uses orderings already in the graph; it never adds DAG edges.
    pub fn set_transient_aliasing(&mut self, enabled: bool) {
        self.transient_aliasing = enabled;
        if !enabled {
            self.return_unconsumed_aliases();
            self.active_alias = None;
            self.caches.alias().trim();
        }
    }

    /// Optional production command-span timestamps. Results are per physical
    /// queue; their sum is not multi-queue frame latency or pure GPU active time.
    pub fn configure_gpu_timing(
        &mut self,
        config: Option<crate::rg::profiling::GpuTimingConfig>,
    ) -> crate::Result<()> {
        if match (&self.gpu_timing, config) {
            (None, None) => true,
            (Some(ring), Some(config)) => ring.matches_config(config),
            _ => false,
        } {
            return if self.device.is_device_lost() { Err(crate::Error::DeviceLost) } else { Ok(()) };
        }
        self.poll_gpu_timings()?;
        if self
            .gpu_timing
            .as_ref()
            .is_some_and(|ring| ring.stats().pending != 0)
        {
            return Err(crate::Error::invalid_usage(
                "GPU timing still owns recorded or in-flight queries",
            ));
        }
        let replacement = config
            .map(|config| crate::rg::profiling::GpuTimingRing::new(self.device.clone(), config))
            .transpose()?;
        self.gpu_timing = replacement;
        Ok(())
    }

    pub fn configure_cpu_arenas(
        &mut self,
        config: crate::command::CpuArenaConfig,
    ) -> crate::Result<()> {
        let replacement =
            crate::command::CpuArenaPool::new(config, self.device.memory.stats.clone())?;
        self.cpu_arenas = replacement;
        Ok(())
    }

    pub fn configure_owner_storage(
        &mut self,
        config: crate::collections::temp_list_pool::TempListPoolConfig,
    ) -> crate::Result<()> {
        self.owner_pool = crate::collections::temp_list_pool::TempListPool::new(
            config,
            self.device.memory.stats.clone(),
        )?;
        Ok(())
    }

    pub fn configure_frame_storage(
        &mut self,
        config: crate::rg::frame_storage::FrameStorageConfig,
    ) -> crate::Result<()> {
        self.frame_storage =
            crate::rg::frame_storage::FrameStoragePool::new(config, self.device.memory.stats.clone())?;
        Ok(())
    }

    /// 图依赖只等待首批次，图完成信号只写入最后批次，semaphore Arc 保留到回执持有者释放。
    pub fn set_graph_synchronization(
        &mut self,
        waits: Vec<TimelineWait>,
        signals: Vec<TimelineSignal>,
    ) -> crate::Result<()> {
        self.check_synchronization(&waits, &signals)?;
        self.graph_waits = waits;
        self.graph_signals = signals;
        Ok(())
    }
}
