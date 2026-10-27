//! Graph and batch synchronization validation.
use crate::rg::executor::ExecutionBackend;
use ash::vk::Handle;
use crate::rg::executor::ExecutionChain;
use crate::rg::graph;
use std::collections::BTreeMap;
use std::sync::Arc;
use super::TimelineSignal;
use super::TimelineWait;
use super::VulkanGraphBackend;
use super::super::resource_spec::GpuResource;
use super::super::resource_spec::validate_import_descriptor;

impl VulkanGraphBackend {
    /// 惰性创建顶层时间戳查询池（两条查询：begin/end）。
    pub(super) fn check_synchronization(
        &self,
        waits: &[TimelineWait],
        signals: &[TimelineSignal],
    ) -> crate::Result<()> {
        if waits
            .iter()
            .any(|(s, _, stage)| !Arc::ptr_eq(&s.device, &self.device) || stage.is_empty())
            || signals
                .iter()
                .any(|(s, _)| !Arc::ptr_eq(&s.device, &self.device))
        {
            return Err(crate::Error::invalid_usage(
                "外部时间线必须来自同一设备且等待阶段不能为空",
            ));
        }
        if waits
            .iter()
            .map(|(s, _, _)| s.handle)
            .chain(signals.iter().map(|(s, _)| s.handle))
            .any(|handle| self.queue_timelines.iter().any(|own| own.handle == handle))
        {
            return Err(crate::Error::invalid_usage(
                "队列内部时间线由提交生产者管理，不能重复作为外部同步输入",
            ));
        }
        Ok(())
    }

    pub(super) fn validate_graph(&self, declaration: &graph::GraphDeclaration) -> crate::Result<()> {
        self.import_validation.lock().with_reserved(
            declaration.resources.len(),
            |physical_imports| -> crate::Result<()> {
                for resource in &declaration.resources {
                    if let graph::OriginSource::ImportOwned { token, .. }
                    | graph::OriginSource::ImportUnowned { token, .. } = resource.source
                    {
                        let actual = self.resources.get(&token).ok_or_else(|| {
                            crate::Error::invalid_usage("导入 token 没有绑定实际资源")
                        })?;
                        actual.check_device(&self.device)?;
                        let identity = match actual {
                            GpuResource::Buffer(buffer) => (0_u8, buffer.handle.as_raw()),
                            GpuResource::Image(image) => (1_u8, image.handle.as_raw()),
                        };
                        physical_imports.push(identity);
                        validate_import_descriptor(resource.descriptor, actual.descriptor())?;
                        if actual.sharing_mode() != resource.sharing {
                            return Err(crate::Error::invalid_usage(
                                "import sharing mode does not match its physical resource",
                            ));
                        }
                        if !self.import_families.contains_key(&token) {
                            return Err(crate::Error::invalid_usage(
                                "import has no declared queue family",
                            ));
                        }
                    }
                }
                physical_imports.sort_unstable();
                if physical_imports.windows(2).any(|pair| pair[0] == pair[1]) {
                    return Err(crate::Error::invalid_usage(
                        "同一物理资源被多个逻辑资源重复导入；缺少显式别名计划",
                    ));
                }
                Ok(())
            },
        )??;
        for code in declaration
            .passes
            .iter()
            .flat_map(|p| &p.accesses)
            .map(|a| a.access_code)
            .chain(declaration.exports.iter().map(|e| e.access_code))
            .chain(declaration.resources.iter().filter_map(|r| match r.source {
                graph::OriginSource::Create => None,
                graph::OriginSource::ImportOwned { initial_access, .. }
                | graph::OriginSource::ImportUnowned { initial_access, .. } => Some(initial_access),
            }))
        {
            if !self.accesses.0.contains_key(&code) {
                return Err(crate::Error::invalid_usage(format!(
                    "缺少访问码 {code} 的显式 Vulkan 映射"
                )));
            }
        }
        Ok(())
    }

    pub(super) fn validate_batch_sync(&self, total: usize) -> crate::Result<()> {
        if self.synchronization.keys().any(|&index| index >= total)
            || (total == 0 && (!self.graph_waits.is_empty() || !self.graph_signals.is_empty()))
        {
            return Err(crate::Error::invalid_usage("同步计划指向不存在的图批次"));
        }
        let mut last_signals = BTreeMap::new();
        for index in 0..total {
            let plan = self.batch_synchronization(index, total);
            for signal in &plan.signals {
                let previous = if let Some(value) = last_signals.get(&signal.semaphore.as_raw()) {
                    *value
                } else {
                    unsafe {
                        self.device
                            .core
                            .device
                            .get_semaphore_counter_value(signal.semaphore)
                    }
                    .map_err(crate::Error::from)?
                };
                let required = plan
                    .waits
                    .iter()
                    .filter(|w| w.semaphore == signal.semaphore)
                    .map(|w| w.value)
                    .max()
                    .unwrap_or(0)
                    .max(previous);
                if signal.value <= required {
                    return Err(crate::Error::invalid_usage(
                        "图时间线信号必须严格大于该批等待值和前次信号值",
                    ));
                }
                last_signals.insert(signal.semaphore.as_raw(), signal.value);
            }
        }
        Ok(())
    }

    pub(in crate::rg::vulkan) fn new_execution_chain(&self) -> ExecutionChain {
        let family_count = self
            .device
            .queues
            .additional
            .iter()
            .map(|q| q.family)
            .chain(std::iter::once(self.device.queues.primary_family))
            .max()
            .unwrap_or(0)
            + 1;
        
        ExecutionChain::new(
            self.device.queues.primary,
            self.device
                .queues
                .additional
                .iter()
                .map(|q| q.handle)
                .collect(),
            family_count,
        )
    }
}
