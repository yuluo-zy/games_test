//! Buffer update planning: regions, plans and update errors.
use ash::vk;
use crate::Error;
use crate::Result;
use super::desc::BufferDesc;

/// Validation failures of a sparse buffer update.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BufferUpdateError {
    /// 区间 `start > end`。
    RangeInverted { start: usize, end: usize },
    /// 区间 `end > src_data.len()`。
    RangeOutOfData { end: usize, len: usize },
    /// `total_bytes + src_data.len() + dst_offset` exceeds the buffer size.
    ExceedsCapacity {
        total: u64,
        dst_offset: u64,
        capacity: u64,
    },
    /// `"Buffer {} must be created with vk::BufferUsageFlags::TRANSFER_DST in order to support updates"`。
    MissingTransferDst,
    /// The update has no bytes.
    Empty,
}

impl std::fmt::Display for BufferUpdateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RangeInverted { start, end } => write!(f, "更新区间 {start}..{end} 起点大于终点"),
            Self::RangeOutOfData { end, len } => write!(f, "更新区间终点 {end} 超过源数据长度 {len}"),
            Self::ExceedsCapacity { total, dst_offset, capacity } => write!(
                f,
                "assertion failed: total_bytes + src_data.len() + dst_offset as usize <= dst_buffer.desc.size（{total}+{dst_offset}>{capacity}）"
            ),
            Self::MissingTransferDst => f.write_str(
                "Buffer must be created with vk::BufferUsageFlags::TRANSFER_DST in order to support updates",
            ),
            Self::Empty => f.write_str("assertion failed: size > 0"),
        }
    }
}

impl std::error::Error for BufferUpdateError {}

impl From<BufferUpdateError> for crate::Error {
    fn from(source: BufferUpdateError) -> Self {
        Self::invalid_usage_with("buffer update", source)
    }
}

/// 一条 `vkCmdCopyBuffer` 区域（`VkBufferCopy` 的 `{src,dst,size}`，不带 `Eq` 依赖）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BufferRegion {
    pub src_offset: u64,
    pub dst_offset: u64,
    pub size: u64,
}

/// `update_buffer_bytes` 的执行计划：暂存缓冲大小与 `vkCmdBufferCopy` 区域。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BufferUpdatePlan {
    /// 暂存缓冲大小 = 各区间长度之和。
    pub total: u64,
    /// 区域 i：`{src = 前面各区间累计, dst = range.start + dst_offset, size = end - start}`
    pub regions: Vec<BufferRegion>,
}

/// Validate sparse updates against the actual buffer description before staging.
/// Source ranges preserve their destination positions; staging bytes are packed.
pub fn plan_buffer_update(
    desc: &BufferDesc,
    data_len: usize,
    ranges: &[std::ops::Range<usize>],
    dst_offset: u64,
) -> Result<BufferUpdatePlan> {
    let mut total = 0u64;
    let mut regions = Vec::new();
    regions
        .try_reserve(ranges.len())
        .map_err(|_| Error::OutOfHostMemory)?;
    for r in ranges {
        if r.start > r.end {
            return Err(BufferUpdateError::RangeInverted {
                start: r.start,
                end: r.end,
            }
            .into());
        }
        if r.end > data_len {
            return Err(BufferUpdateError::RangeOutOfData {
                end: r.end,
                len: data_len,
            }
            .into());
        }
        let len = (r.end - r.start) as u64;
        let exceeds = || BufferUpdateError::ExceedsCapacity {
            total: total.saturating_add(len),
            dst_offset,
            capacity: desc.size,
        };
        let next = total
            .checked_add(len)
            .ok_or_else(|| Error::from(exceeds()))?;
        if next
            .checked_add(dst_offset)
            .is_none_or(|end| end > desc.size)
        {
            return Err(exceeds().into());
        }
        let dst_end = (r.end as u64)
            .checked_add(dst_offset)
            .filter(|&end| end <= desc.size)
            .ok_or_else(|| Error::from(exceeds()))?;
        if len != 0 {
            regions.push(BufferRegion {
                src_offset: total,
                dst_offset: dst_end - len,
                size: len,
            });
        }
        total = next;
    }
    if !desc.usage.contains(vk::BufferUsageFlags::TRANSFER_DST) {
        return Err(BufferUpdateError::MissingTransferDst.into());
    }
    if total == 0 {
        return Err(BufferUpdateError::Empty.into());
    }
    Ok(BufferUpdatePlan { total, regions })
}
