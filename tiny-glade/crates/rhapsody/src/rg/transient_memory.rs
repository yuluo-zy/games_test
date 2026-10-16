//! 单队列瞬态内存着色（§4.5）：按生命周期区间复用物理内存。
//!
//! # 设计
//!
//! 图内短生命周期资源（中间附件、临时缓冲）不必各自独占一块 `VkDeviceMemory`。两个资源
//! 只要**生命周期区间不重叠**且内存兼容（同类、同族），就可以先后复用同一段字节。本模块
//! 是纯 CPU 放置器：输入每个资源的不可变事实（大小、对齐、兼容类、生命周期、是否必须独立
//! 分配），输出「backing + 偏移」方案与统计。
//!
//! 模型：把资源按兼容类分组着色——同一 `Backing` 内的资源两两不重叠，因此全部落在偏移 0、
//! 共享同一段字节；`Backing` 的大小取成员对齐后尺寸的最大值，对齐取成员对齐的最大值。
//! 重叠的资源必须落入不同 backing。这是 §4.5 的“冲突图着色 + 带容量贪心放置”在单队列
//! 非重叠语义下的等价形态：**默认不新增串行依赖**，只复用生命周期已证明不重叠的资源。
//!
//! 放置用“尺寸降序 + first-fit”贪心（大资源优先归组），§4.5 明确不要求最优图着色。重叠
//! 判定逐成员进行，复杂度 O(n²)，适合图内瞬态资源（几十~几百）。

use std::collections::HashMap;
use std::ops::Range;

/// 放置器输入：一个短生命周期资源的内存事实。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResourceSpec {
    /// 图内资源标识（由调用方保证唯一）。
    pub id: u32,
    /// 内存兼容类：只有同类资源才可能共享同一 backing。
    pub compatibility: u32,
    pub size: u64,
    /// 对齐要求，必须为 2 的幂且非 0。
    pub align: u64,
    /// 生命周期 `[start, end)`，以单队列访问序号表示；`end > start`。
    pub lifetime: Range<u32>,
    /// 是否必须独立分配（导入、temporal、持久导出/结果缓存输出等）。
    pub dedicated: bool,
}

/// 一个资源的最终放置。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Placement {
    pub resource: u32,
    pub backing: usize,
    /// 在 backing 内的字节偏移。当前非重叠模型下恒为 0；未来支持同 backing 多区间打包时扩展。
    pub offset: u64,
}

/// 一块 backing allocation 及其承载的资源（两两生命周期不重叠）。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Backing {
    pub compatibility: u32,
    /// 需要的字节数：成员对齐后尺寸的最大值（真实分配时按 `align` 再向上取整）。
    pub size: u64,
    /// 需要的内存对齐：成员对齐的最大值。
    pub align: u64,
    pub resources: Vec<u32>,
}

/// 放置方案的统计口径。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PlanStats {
    /// 方案需要的总字节数（各 backing 大小之和）。
    pub allocated_bytes: u64,
    /// 每个资源独立分配时需要的字节数（对齐后求和）。
    pub unaliased_bytes: u64,
    /// 别名复用节省的字节数（`unaliased - allocated`，饱和）。
    pub reuse_bytes: u64,
    /// backing 数量（= 物理分配次数）。
    pub allocation_count: usize,
}

/// 放置方案。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransientMemoryPlan {
    pub backings: Vec<Backing>,
    pub placements: Vec<Placement>,
    pub stats: PlanStats,
}

/// 放置输入的非法事实。
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum PlanError {
    #[error("资源 {0} 的对齐必须是 2 的幂且非 0")]
    InvalidAlignment(u32),
    #[error("资源 {0} 的生命周期区间必须满足 end > start")]
    InvalidLifetime(u32),
}

impl From<PlanError> for crate::Error {
    fn from(error: PlanError) -> Self {
        crate::Error::invalid_usage_with("瞬态内存放置输入非法", error)
    }
}

/// 计算单队列瞬态内存放置方案。
pub fn plan_transient_memory(resources: &[ResourceSpec]) -> Result<TransientMemoryPlan, PlanError> {
    for resource in resources {
        if resource.align == 0 || !resource.align.is_power_of_two() {
            return Err(PlanError::InvalidAlignment(resource.id));
        }
        if resource.lifetime.start >= resource.lifetime.end {
            return Err(PlanError::InvalidLifetime(resource.id));
        }
    }

    let by_id: HashMap<u32, &ResourceSpec> = resources.iter().map(|r| (r.id, r)).collect();
    let mut order: Vec<&ResourceSpec> = resources.iter().collect();
    // 大资源优先归组，减少“大资源各占一个 backing”造成的碎片；id 作 tie-break 保证确定性。
    order.sort_by_key(|r| (std::cmp::Reverse(r.size), r.id));

    let mut plan = TransientMemoryPlan {
        backings: Vec::new(),
        placements: Vec::new(),
        stats: PlanStats::default(),
    };

    for resource in order {
        let aligned = align_up(resource.size, resource.align);
        plan.stats.unaliased_bytes = plan.stats.unaliased_bytes.saturating_add(aligned);

        let target = if resource.dedicated {
            None
        } else {
            plan.backings.iter().position(|backing| {
                backing.compatibility == resource.compatibility
                    && backing
                        .resources
                        .iter()
                        .all(|&placed| !overlaps(&resource.lifetime, &by_id[&placed].lifetime))
            })
        };

        let backing_index = match target {
            Some(index) => {
                let backing = &mut plan.backings[index];
                backing.size = backing.size.max(aligned);
                backing.align = backing.align.max(resource.align);
                backing.resources.push(resource.id);
                index
            }
            None => {
                let index = plan.backings.len();
                plan.backings.push(Backing {
                    compatibility: resource.compatibility,
                    size: aligned,
                    align: resource.align,
                    resources: vec![resource.id],
                });
                index
            }
        };
        plan.placements.push(Placement {
            resource: resource.id,
            backing: backing_index,
            offset: 0,
        });
    }

    plan.stats.allocated_bytes = plan.backings.iter().map(|b| b.size).sum();
    plan.stats.allocation_count = plan.backings.len();
    plan.stats.reuse_bytes = plan
        .stats
        .unaliased_bytes
        .saturating_sub(plan.stats.allocated_bytes);
    Ok(plan)
}

fn overlaps(a: &Range<u32>, b: &Range<u32>) -> bool {
    a.start < b.end && b.start < a.end
}

fn align_up(value: u64, align: u64) -> u64 {
    let mask = align - 1;
    match value.checked_add(mask) {
        Some(rounded) => rounded & !mask,
        None => !mask,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn spec(
        id: u32,
        compatibility: u32,
        size: u64,
        align: u64,
        start: u32,
        end: u32,
    ) -> ResourceSpec {
        ResourceSpec {
            id,
            compatibility,
            size,
            align,
            lifetime: start..end,
            dedicated: false,
        }
    }

    #[test]
    fn serial_compatible_resources_share_bytes() {
        // 0 先于 1，不重叠：同一 backing、偏移 0，大小取 64。
        let plan =
            plan_transient_memory(&[spec(0, 1, 64, 8, 0, 2), spec(1, 1, 32, 8, 2, 4)]).unwrap();
        assert_eq!(plan.backings.len(), 1);
        assert_eq!(plan.backings[0].size, 64);
        assert_eq!(plan.backings[0].resources, [0, 1]);
        assert!(plan.placements.iter().all(|p| p.offset == 0));
        assert_eq!(plan.stats.unaliased_bytes, 96);
        assert_eq!(plan.stats.allocated_bytes, 64);
        assert_eq!(plan.stats.reuse_bytes, 32);
    }

    #[test]
    fn overlapping_lifetimes_are_never_shared() {
        let plan =
            plan_transient_memory(&[spec(0, 1, 64, 8, 0, 2), spec(1, 1, 64, 8, 1, 3)]).unwrap();
        assert_eq!(plan.backings.len(), 2);
        assert_eq!(plan.stats.reuse_bytes, 0);
        assert_eq!(plan.stats.allocated_bytes, 128);
    }

    #[test]
    fn incompatible_classes_never_share_even_when_disjoint() {
        let plan =
            plan_transient_memory(&[spec(0, 1, 64, 8, 0, 1), spec(1, 2, 64, 8, 1, 2)]).unwrap();
        assert_eq!(plan.backings.len(), 2);
    }

    #[test]
    fn dedicated_resources_are_never_aliased() {
        let mut dedicated = spec(1, 1, 64, 8, 2, 3);
        dedicated.dedicated = true;
        let plan = plan_transient_memory(&[spec(0, 1, 64, 8, 0, 1), dedicated]).unwrap();
        assert_eq!(plan.backings.len(), 2);
        assert_eq!(plan.backings[1].resources, [1]);
    }

    #[test]
    fn backing_size_and_align_are_the_max_of_members() {
        // 100 对齐到 8 = 104；第二个成员 16 对齐到 16 = 16。backing 大小 104、对齐 16。
        let plan =
            plan_transient_memory(&[spec(0, 1, 100, 8, 0, 1), spec(1, 1, 16, 16, 1, 2)]).unwrap();
        assert_eq!(plan.backings.len(), 1);
        assert_eq!(plan.backings[0].size, 104);
        assert_eq!(plan.backings[0].align, 16);
    }

    #[test]
    fn chain_places_serialized_resources_together() {
        // A[0,2) 与 B[2,4) 串行共享 64；C[0,1) 与 A 重叠，另开 16。
        let plan = plan_transient_memory(&[
            spec(0, 1, 64, 8, 0, 2),
            spec(1, 1, 32, 8, 2, 4),
            spec(2, 1, 16, 8, 0, 1),
        ])
        .unwrap();
        assert_eq!(plan.backings.len(), 2);
        assert!(plan.backings[0].resources.contains(&0) && plan.backings[0].resources.contains(&1));
        assert_eq!(plan.stats.allocation_count, 2);
        assert_eq!(plan.stats.unaliased_bytes, 64 + 32 + 16);
        assert_eq!(plan.stats.allocated_bytes, 64 + 16);
        assert_eq!(plan.stats.reuse_bytes, 32);
    }

    #[test]
    fn invalid_alignment_and_lifetime_are_rejected() {
        assert!(matches!(
            plan_transient_memory(&[spec(0, 1, 64, 3, 0, 1)]),
            Err(PlanError::InvalidAlignment(0))
        ));
        assert!(matches!(
            plan_transient_memory(&[spec(0, 1, 64, 8, 2, 2)]),
            Err(PlanError::InvalidLifetime(0))
        ));
    }
}
