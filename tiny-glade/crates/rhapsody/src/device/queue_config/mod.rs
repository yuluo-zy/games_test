//! 可配置的多队列规划。
//!
//! 输入是物理设备的队列族属性和 [`QueueConfig`]，输出 [`QueuePlan`]：要创建哪些物理队列
//! （`VkDeviceQueueCreateInfo`），以及每个 [`QueueClass`] 落在哪个逻辑队列上。
//!
//! 选择规则：
//! - **Graphics**：必需。必须同时支持 compute 的族，保证类别别名有效；`require_present` 时必须能呈现。
//! - **Compute**：优先专用族（有 COMPUTE、无 GRAPHICS），其次 graphics 族的空闲队列，
//!   然后是其它支持 COMPUTE 的可用族。
//! - **Transfer**：优先专用 DMA 族，其次专用 compute 族的空闲队列、graphics 族的空闲队列，
//!   最后是其它支持传输的可用族。图像传输粒度须为 1×1×1；graphics/compute 隐含传输能力。
//! - 无可用独立队列时 `Prefer` 别名到 graphics，`Require` 报 [`Error::Unsupported`]。
//!   `Off` 始终别名到 graphics。
//!
//! 别名队列共用同一个 `VkQueue` 和时间线，语义上等价于单队列，结果与"关闭"一致。
use crate::error::{Error, Result};
use crate::sync::{MAX_QUEUES, QueueId};
use ash::vk;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum QueuePreference {
    /// 不使用独立队列，别名到 graphics。
    #[default]
    Off,
    /// 有合适的独立队列就用，否则别名到 graphics。
    Prefer,
    /// 必须有独立队列，否则创建设备失败。
    Require,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct QueueConfig {
    pub async_compute: QueuePreference,
    pub transfer: QueuePreference,
    /// graphics 队列必须支持向目标 surface 呈现（无窗口离屏渲染时为 false）。
    pub require_present: bool,
}

impl Default for QueueConfig {
    fn default() -> Self {
        Self {
            async_compute: QueuePreference::Off,
            transfer: QueuePreference::Off,
            require_present: true,
        }
    }
}

impl QueueConfig {
    /// 所有可用的独立队列都用上。
    pub fn prefer_all() -> Self {
        Self {
            async_compute: QueuePreference::Prefer,
            transfer: QueuePreference::Prefer,
            require_present: true,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum QueueClass {
    Graphics,
    Compute,
    Transfer,
}

impl QueueClass {
    pub const ALL: [Self; 3] = [Self::Graphics, Self::Compute, Self::Transfer];
    const fn slot(self) -> usize {
        match self {
            Self::Graphics => 0,
            Self::Compute => 1,
            Self::Transfer => 2,
        }
    }
}

/// 一个队列族的规划所需属性（来自 `VkQueueFamilyProperties` 与呈现支持查询）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FamilyInfo {
    pub flags: vk::QueueFlags,
    pub count: u32,
    pub present: bool,
    pub min_image_transfer_granularity: vk::Extent3D,
}

impl FamilyInfo {
    pub fn from_properties(properties: &vk::QueueFamilyProperties, present: bool) -> Self {
        Self {
            flags: properties.queue_flags,
            count: properties.queue_count,
            present,
            min_image_transfer_granularity: properties.min_image_transfer_granularity,
        }
    }
    fn has(&self, flags: vk::QueueFlags) -> bool {
        self.flags.contains(flags)
    }
    fn fine_grained_transfer(&self) -> bool {
        let g = self.min_image_transfer_granularity;
        (g.width, g.height, g.depth) == (1, 1, 1)
    }
}

/// 要创建的一个物理队列。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PhysicalQueue {
    pub family: u32,
    pub index: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QueuePlan {
    /// 逻辑队列 `QueueId(i)` 对应 `queues[i]`；`queues[0]` 恒为 graphics。
    pub queues: Vec<PhysicalQueue>,
    classes: [QueueId; 3],
}

impl QueuePlan {
    /// 该类别实际使用的逻辑队列。
    pub fn queue(&self, class: QueueClass) -> QueueId {
        self.classes[class.slot()]
    }
    /// 该类别是否别名到了 graphics 队列（没有独立队列）。
    pub fn is_aliased(&self, class: QueueClass) -> bool {
        class != QueueClass::Graphics && self.queue(class) == self.queue(QueueClass::Graphics)
    }
    pub fn physical(&self, class: QueueClass) -> PhysicalQueue {
        self.queues[self.queue(class).index()]
    }
    /// 每个族要创建的队列数，按族编号升序，用于 `VkDeviceQueueCreateInfo`。
    pub fn create_counts(&self) -> Vec<(u32, u32)> {
        let mut counts: Vec<(u32, u32)> = Vec::new();
        for queue in &self.queues {
            match counts
                .iter_mut()
                .find(|(family, _)| *family == queue.family)
            {
                Some((_, count)) => *count = (*count).max(queue.index + 1),
                None => counts.push((queue.family, queue.index + 1)),
            }
        }
        counts.sort_unstable();
        counts
    }
    /// 不同的物理队列族集合（资源在多族间共享时用于 `CONCURRENT` 的族列表）。
    pub fn families(&self) -> Vec<u32> {
        let mut families: Vec<u32> = self.queues.iter().map(|q| q.family).collect();
        families.sort_unstable();
        families.dedup();
        families
    }
}

struct Allocator<'a> {
    families: &'a [FamilyInfo],
    used: Vec<u32>,
    queues: Vec<PhysicalQueue>,
}

impl Allocator<'_> {
    fn free(&self, family: usize) -> bool {
        self.used[family] < self.families[family].count
    }
    fn take(&mut self, family: usize) -> QueueId {
        let index = self.used[family];
        self.used[family] += 1;
        self.queues.push(PhysicalQueue {
            family: family as u32,
            index,
        });
        QueueId::new(self.queues.len() - 1).expect("规划的逻辑队列数不超过 3")
    }
    fn first(&self, accept: impl Fn(&FamilyInfo) -> bool) -> Option<usize> {
        (0..self.families.len()).find(|&family| self.free(family) && accept(&self.families[family]))
    }
}

pub fn plan_queues(families: &[FamilyInfo], config: &QueueConfig) -> Result<QueuePlan> {
    const _: () = assert!(MAX_QUEUES >= 3);
    let mut alloc = Allocator {
        families,
        used: vec![0; families.len()],
        queues: Vec::new(),
    };
    let present_ok = |f: &FamilyInfo| f.present || !config.require_present;
    let graphics_family = alloc
        .first(|f| f.has(vk::QueueFlags::GRAPHICS | vk::QueueFlags::COMPUTE) && present_ok(f))
        .ok_or_else(|| {
            Error::unsupported(if config.require_present {
                "没有同时支持图形、计算与呈现的队列族"
            } else {
                "没有同时支持图形与计算的队列族"
            })
        })?;
    let graphics = alloc.take(graphics_family);

    let compute = match config.async_compute {
        QueuePreference::Off => graphics,
        preference => {
            let dedicated = alloc
                .first(|f| f.has(vk::QueueFlags::COMPUTE) && !f.has(vk::QueueFlags::GRAPHICS))
                .or_else(|| alloc.free(graphics_family).then_some(graphics_family))
                .or_else(|| alloc.first(|f| f.has(vk::QueueFlags::COMPUTE)));
            match dedicated {
                Some(family) => alloc.take(family),
                None if preference == QueuePreference::Require => {
                    return Err(Error::unsupported("没有可用的独立计算队列"));
                }
                None => graphics,
            }
        }
    };

    let transfer = match config.transfer {
        QueuePreference::Off => graphics,
        preference => {
            let dma = alloc.first(|f| {
                f.has(vk::QueueFlags::TRANSFER)
                    && !f.has(vk::QueueFlags::GRAPHICS)
                    && !f.has(vk::QueueFlags::COMPUTE)
                    && f.fine_grained_transfer()
            });
            let spare_compute = || {
                alloc.first(|f| {
                    f.has(vk::QueueFlags::COMPUTE)
                        && !f.has(vk::QueueFlags::GRAPHICS)
                        && f.fine_grained_transfer()
                })
            };
            let spare_graphics = || {
                (alloc.free(graphics_family) && families[graphics_family].fine_grained_transfer())
                    .then_some(graphics_family)
            };
            let other_transfer = || {
                alloc.first(|f| {
                    f.flags.intersects(
                        vk::QueueFlags::GRAPHICS
                            | vk::QueueFlags::COMPUTE
                            | vk::QueueFlags::TRANSFER,
                    ) && f.fine_grained_transfer()
                })
            };
            match dma
                .or_else(spare_compute)
                .or_else(spare_graphics)
                .or_else(other_transfer)
            {
                Some(family) => alloc.take(family),
                None if preference == QueuePreference::Require => {
                    return Err(Error::unsupported("没有可用的独立传输队列"));
                }
                None => graphics,
            }
        }
    };

    Ok(QueuePlan {
        queues: alloc.queues,
        classes: [graphics, compute, transfer],
    })
}

#[cfg(test)]
mod tests;
