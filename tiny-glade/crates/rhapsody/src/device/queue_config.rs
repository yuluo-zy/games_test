//! 可配置的多队列规划。
//!
//! 输入是物理设备的队列族属性和 [`QueueConfig`]，输出 [`QueuePlan`]：要创建哪些物理队列
//! （`VkDeviceQueueCreateInfo`），以及每个 [`QueueClass`] 落在哪个逻辑队列上。
//!
//! 选择规则：
//! - **Graphics**：必需。优先同时支持 compute 的族；`require_present` 时必须能呈现。
//! - **Compute**：优先专用族（有 COMPUTE、无 GRAPHICS）；否则用 graphics 族里的第二个队列；
//!   都没有时 `Prefer`/`Off` 别名到 graphics，`Require` 报 [`Error::Unsupported`]。
//! - **Transfer**：优先专用 DMA 族（只有 TRANSFER，且图像传输粒度为 1×1×1）；否则用专用
//!   compute 族的空闲队列；都没有时别名到 graphics（graphics/compute 族都隐含传输能力）。
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
            match counts.iter_mut().find(|(family, _)| *family == queue.family) {
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
        .or_else(|| alloc.first(|f| f.has(vk::QueueFlags::GRAPHICS) && present_ok(f)))
        .ok_or_else(|| {
            Error::unsupported(if config.require_present {
                "没有支持呈现的图形队列族"
            } else {
                "没有图形队列族"
            })
        })?;
    let graphics = alloc.take(graphics_family);

    let compute = match config.async_compute {
        QueuePreference::Off => graphics,
        preference => {
            let dedicated = alloc
                .first(|f| f.has(vk::QueueFlags::COMPUTE) && !f.has(vk::QueueFlags::GRAPHICS))
                .or_else(|| alloc.free(graphics_family).then_some(graphics_family));
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
            match dma.or_else(spare_compute) {
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
mod tests {
    use super::*;
    use crate::error::ErrorKind;

    const G: vk::QueueFlags = vk::QueueFlags::GRAPHICS;
    const C: vk::QueueFlags = vk::QueueFlags::COMPUTE;
    const T: vk::QueueFlags = vk::QueueFlags::TRANSFER;

    fn family(flags: vk::QueueFlags, count: u32, present: bool) -> FamilyInfo {
        FamilyInfo {
            flags,
            count,
            present,
            min_image_transfer_granularity: vk::Extent3D { width: 1, height: 1, depth: 1 },
        }
    }
    fn coarse(mut f: FamilyInfo) -> FamilyInfo {
        f.min_image_transfer_granularity = vk::Extent3D { width: 8, height: 8, depth: 8 };
        f
    }
    fn pq(family: u32, index: u32) -> PhysicalQueue {
        PhysicalQueue { family, index }
    }

    #[test]
    fn single_queue_device_aliases_everything_or_refuses_requirements() {
        // lavapipe / 多数 Intel 集显：一个族、一个队列。
        let families = [family(G | C | T, 1, true)];
        let plan = plan_queues(&families, &QueueConfig::prefer_all()).unwrap();
        assert_eq!(plan.queues, [pq(0, 0)]);
        assert!(plan.is_aliased(QueueClass::Compute) && plan.is_aliased(QueueClass::Transfer));
        assert_eq!(plan.create_counts(), [(0, 1)]);

        let require = QueueConfig {
            async_compute: QueuePreference::Require,
            ..QueueConfig::default()
        };
        assert_eq!(plan_queues(&families, &require).unwrap_err().kind(), ErrorKind::Unsupported);
    }

    #[test]
    fn off_never_creates_extra_queues() {
        let families = [family(G | C | T, 16, true), family(C | T, 8, false), family(T, 2, false)];
        let plan = plan_queues(&families, &QueueConfig::default()).unwrap();
        assert_eq!(plan.queues, [pq(0, 0)]);
    }

    #[test]
    fn discrete_layout_uses_dedicated_compute_and_dma_families() {
        // 常见独显：通用族 + 专用计算族 + 专用 DMA 族。
        let families = [family(G | C | T, 16, true), family(T, 2, false), family(C | T, 8, false)];
        let plan = plan_queues(&families, &QueueConfig::prefer_all()).unwrap();
        assert_eq!(plan.physical(QueueClass::Graphics), pq(0, 0));
        assert_eq!(plan.physical(QueueClass::Compute), pq(2, 0));
        assert_eq!(plan.physical(QueueClass::Transfer), pq(1, 0));
        assert_eq!(plan.create_counts(), [(0, 1), (1, 1), (2, 1)]);
        assert_eq!(plan.families(), [0, 1, 2]);
        assert!(!plan.is_aliased(QueueClass::Compute) && !plan.is_aliased(QueueClass::Transfer));
    }

    #[test]
    fn coarse_granularity_dma_is_skipped_for_a_spare_compute_queue() {
        let families = [family(G | C | T, 1, true), family(C | T, 4, false), coarse(family(T, 2, false))];
        let plan = plan_queues(&families, &QueueConfig::prefer_all()).unwrap();
        assert_eq!(plan.physical(QueueClass::Compute), pq(1, 0));
        assert_eq!(plan.physical(QueueClass::Transfer), pq(1, 1), "粗粒度 DMA 族不用于图像传输");
        assert_eq!(plan.create_counts(), [(0, 1), (1, 2)]);
    }

    #[test]
    fn second_queue_of_the_graphics_family_serves_async_compute() {
        let families = [family(G | C | T, 2, true)];
        let plan = plan_queues(&families, &QueueConfig::prefer_all()).unwrap();
        assert_eq!(plan.physical(QueueClass::Compute), pq(0, 1));
        assert!(plan.is_aliased(QueueClass::Transfer));
        assert_eq!(plan.create_counts(), [(0, 2)]);
    }

    #[test]
    fn graphics_family_must_present_when_required() {
        let families = [family(G | C | T, 1, false), family(G | T, 1, true)];
        let plan = plan_queues(&families, &QueueConfig::default()).unwrap();
        assert_eq!(plan.physical(QueueClass::Graphics), pq(1, 0));

        let headless = QueueConfig {
            require_present: false,
            ..QueueConfig::default()
        };
        assert_eq!(plan_queues(&families, &headless).unwrap().physical(QueueClass::Graphics), pq(0, 0));

        let no_present = [family(G | C | T, 1, false)];
        assert_eq!(
            plan_queues(&no_present, &QueueConfig::default()).unwrap_err().kind(),
            ErrorKind::Unsupported
        );
        assert_eq!(plan_queues(&[], &headless).unwrap_err().kind(), ErrorKind::Unsupported);
    }

    #[test]
    fn transfer_requirement_without_any_candidate_fails() {
        let families = [family(G | C | T, 4, true)];
        let config = QueueConfig {
            transfer: QueuePreference::Require,
            ..QueueConfig::default()
        };
        assert_eq!(plan_queues(&families, &config).unwrap_err().kind(), ErrorKind::Unsupported);
    }
}
