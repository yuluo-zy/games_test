//! 原版访问码 → (stage, access, layout) 表。
//!
//! 原 `vk-sync-fork` 的 `AccessType` 查表函数 0x140c10dd0：`out = { stage_tab[code], access_tab[code], layout_tab[code] }`，
//! 三张 64 项 u32 表在 `.rdata` 的 0x142b1a6c8 / 0x142b1a7c8 / 0x142b1a8c8，以 `code` 为下标，输出写在
//! `+0 / +4 / +8`。下面的 64 项逐项从 EXE 读出。公开的 `vk-sync-fork 0.4.0` 枚举与它不同（原表在前部多
//! 6 项：原 `TransferWrite = 0x34`、`Present = 0x27`，公开版分别是 0x2e、0x21），所以不能直接用公开 crate。
//!
//! `is_write`（0x140c10e10）：`code - 0x28 < 0x10`，即写访问码恰为 `0x28..=0x37`。
//! 越界下标原版会读到相邻数据（无边界检查）；这里 `code >= 64` 返回 `None`。
use ash::vk;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AccessInfo {
    pub stage: u32,
    pub access: u32,
    /// 原始 `VkImageLayout` 值。
    pub layout: i32,
}

pub const ACCESS_CODE_COUNT: usize = 64;

pub static ACCESS_TABLE: [AccessInfo; ACCESS_CODE_COUNT] = [
    // 0x00
    AccessInfo {
        stage: 0x0,
        access: 0x0,
        layout: 0,
    },
    // 0x01
    AccessInfo {
        stage: 0x20000,
        access: 0x20000,
        layout: 0,
    },
    // 0x02
    AccessInfo {
        stage: 0x10000,
        access: 0x21,
        layout: 0,
    },
    // 0x03
    AccessInfo {
        stage: 0x4,
        access: 0x2,
        layout: 0,
    },
    // 0x04
    AccessInfo {
        stage: 0x4,
        access: 0x4,
        layout: 0,
    },
    // 0x05
    AccessInfo {
        stage: 0x8,
        access: 0x20,
        layout: 0,
    },
    // 0x06
    AccessInfo {
        stage: 0x8,
        access: 0x20,
        layout: 5,
    },
    // 0x07
    AccessInfo {
        stage: 0x8,
        access: 0x20,
        layout: 1,
    },
    // 0x08
    AccessInfo {
        stage: 0x10,
        access: 0x8,
        layout: 0,
    },
    // 0x09
    AccessInfo {
        stage: 0x10,
        access: 0x20,
        layout: 5,
    },
    // 0x0a
    AccessInfo {
        stage: 0x10,
        access: 0x20,
        layout: 1,
    },
    // 0x0b
    AccessInfo {
        stage: 0x20,
        access: 0x8,
        layout: 0,
    },
    // 0x0c
    AccessInfo {
        stage: 0x20,
        access: 0x20,
        layout: 5,
    },
    // 0x0d
    AccessInfo {
        stage: 0x20,
        access: 0x20,
        layout: 1,
    },
    // 0x0e
    AccessInfo {
        stage: 0x40,
        access: 0x8,
        layout: 0,
    },
    // 0x0f
    AccessInfo {
        stage: 0x40,
        access: 0x20,
        layout: 5,
    },
    // 0x10
    AccessInfo {
        stage: 0x40,
        access: 0x20,
        layout: 1,
    },
    // 0x11
    AccessInfo {
        stage: 0x80,
        access: 0x8,
        layout: 0,
    },
    // 0x12
    AccessInfo {
        stage: 0x80,
        access: 0x20,
        layout: 5,
    },
    // 0x13
    AccessInfo {
        stage: 0x80,
        access: 0x10,
        layout: 5,
    },
    // 0x14
    AccessInfo {
        stage: 0x80,
        access: 0x10,
        layout: 4,
    },
    // 0x15
    AccessInfo {
        stage: 0x80,
        access: 0x20,
        layout: 1,
    },
    // 0x16
    AccessInfo {
        stage: 0x400,
        access: 0x80,
        layout: 2,
    },
    // 0x17
    AccessInfo {
        stage: 0x300,
        access: 0x200,
        layout: 4,
    },
    // 0x18
    AccessInfo {
        stage: 0x800,
        access: 0x8,
        layout: 0,
    },
    // 0x19
    AccessInfo {
        stage: 0x800,
        access: 0x20,
        layout: 5,
    },
    // 0x1a
    AccessInfo {
        stage: 0x800,
        access: 0x20,
        layout: 1,
    },
    // 0x1b
    AccessInfo {
        stage: 0xf8,
        access: 0x8,
        layout: 0,
    },
    // 0x1c
    AccessInfo {
        stage: 0xf8,
        access: 0xc,
        layout: 0,
    },
    // 0x1d
    AccessInfo {
        stage: 0xf8,
        access: 0x20,
        layout: 5,
    },
    // 0x1e
    AccessInfo {
        stage: 0x3f8,
        access: 0x220,
        layout: 4,
    },
    // 0x1f
    AccessInfo {
        stage: 0x3f8,
        access: 0x20,
        layout: 1,
    },
    // 0x20
    AccessInfo {
        stage: 0x10000,
        access: 0x8,
        layout: 0,
    },
    // 0x21
    AccessInfo {
        stage: 0x10000,
        access: 0xc,
        layout: 0,
    },
    // 0x22
    AccessInfo {
        stage: 0x10000,
        access: 0x20,
        layout: 5,
    },
    // 0x23
    AccessInfo {
        stage: 0x10000,
        access: 0x220,
        layout: 4,
    },
    // 0x24
    AccessInfo {
        stage: 0x10000,
        access: 0x20,
        layout: 1,
    },
    // 0x25
    AccessInfo {
        stage: 0x1000,
        access: 0x800,
        layout: 6,
    },
    // 0x26
    AccessInfo {
        stage: 0x4000,
        access: 0x2000,
        layout: 1,
    },
    // 0x27
    AccessInfo {
        stage: 0x0,
        access: 0x0,
        layout: 1000001002,
    },
    // 0x28
    AccessInfo {
        stage: 0x20000,
        access: 0x40000,
        layout: 0,
    },
    // 0x29
    AccessInfo {
        stage: 0x8,
        access: 0x40,
        layout: 1,
    },
    // 0x2a
    AccessInfo {
        stage: 0x10,
        access: 0x40,
        layout: 1,
    },
    // 0x2b
    AccessInfo {
        stage: 0x20,
        access: 0x40,
        layout: 1,
    },
    // 0x2c
    AccessInfo {
        stage: 0x40,
        access: 0x40,
        layout: 1,
    },
    // 0x2d
    AccessInfo {
        stage: 0x80,
        access: 0x40,
        layout: 1,
    },
    // 0x2e
    AccessInfo {
        stage: 0x400,
        access: 0x100,
        layout: 2,
    },
    // 0x2f
    AccessInfo {
        stage: 0x300,
        access: 0x600,
        layout: 3,
    },
    // 0x30
    AccessInfo {
        stage: 0x300,
        access: 0x600,
        layout: 1000117001,
    },
    // 0x31
    AccessInfo {
        stage: 0x300,
        access: 0x600,
        layout: 1000117000,
    },
    // 0x32
    AccessInfo {
        stage: 0x800,
        access: 0x40,
        layout: 1,
    },
    // 0x33
    AccessInfo {
        stage: 0x10000,
        access: 0x40,
        layout: 1,
    },
    // 0x34
    AccessInfo {
        stage: 0x1000,
        access: 0x1000,
        layout: 7,
    },
    // 0x35
    AccessInfo {
        stage: 0x4000,
        access: 0x4000,
        layout: 1,
    },
    // 0x36
    AccessInfo {
        stage: 0x400,
        access: 0x180,
        layout: 2,
    },
    // 0x37
    AccessInfo {
        stage: 0x10000,
        access: 0x18000,
        layout: 1,
    },
    // 0x38
    AccessInfo {
        stage: 0x200000,
        access: 0x20,
        layout: 5,
    },
    // 0x39
    AccessInfo {
        stage: 0x200000,
        access: 0x10,
        layout: 5,
    },
    // 0x3a
    AccessInfo {
        stage: 0x200000,
        access: 0x10,
        layout: 4,
    },
    // 0x3b
    AccessInfo {
        stage: 0x200000,
        access: 0x200000,
        layout: 0,
    },
    // 0x3c
    AccessInfo {
        stage: 0x200000,
        access: 0x20,
        layout: 1,
    },
    // 0x3d
    AccessInfo {
        stage: 0x2000000,
        access: 0x400000,
        layout: 0,
    },
    // 0x3e
    AccessInfo {
        stage: 0x2000000,
        access: 0x200000,
        layout: 0,
    },
    // 0x3f
    AccessInfo {
        stage: 0x2000000,
        access: 0x1000,
        layout: 0,
    },
];

pub fn access_info(code: u8) -> Option<AccessInfo> {
    ACCESS_TABLE.get(usize::from(code)).copied()
}

/// 0x140c10e10。
pub const fn is_write(code: u8) -> bool {
    code.wrapping_sub(0x28) < 0x10
}

impl AccessInfo {
    pub fn stages(self) -> vk::PipelineStageFlags {
        vk::PipelineStageFlags::from_raw(self.stage)
    }
    pub fn access_flags(self) -> vk::AccessFlags {
        vk::AccessFlags::from_raw(self.access)
    }
    pub fn image_layout(self) -> vk::ImageLayout {
        vk::ImageLayout::from_raw(self.layout)
    }
}

/// 原 `vk_sync_fork::ImageLayout`（`ImageBarrier` 偏移 +0x44/+0x45 的字节：0 = Optimal，1 = General，
/// 其余值在原版里走 `unimplemented!`——"not implemented"，crates/rhapsody/crates/vk-sync-rs/src/lib.rs）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum BarrierLayout {
    #[default]
    Optimal,
    General,
}

const PRESENT: u8 = 0x27;
const TOP_OF_PIPE: u32 = 0x1;
const BOTTOM_OF_PIPE: u32 = 0x2000;

struct Accumulated {
    stages: u32,
    /// 只累计写访问码的 access（`is_write` 判定，0x140c107c0 内循环）。
    write_access: u32,
    all_access: u32,
    /// 最后一个访问码的布局（原版循环里后写覆盖前写）。
    layout: i32,
}

fn accumulate(codes: &[u8]) -> Accumulated {
    let mut acc = Accumulated {
        stages: 0,
        write_access: 0,
        all_access: 0,
        layout: 0,
    };
    for &code in codes {
        // 原版对越界码不检查，直接读相邻 `.rdata`；这里取 `Nothing`（全 0）。
        let info = access_info(code).unwrap_or(ACCESS_TABLE[0]);
        acc.stages |= info.stage;
        acc.all_access |= info.access;
        if is_write(code) {
            acc.write_access |= info.access;
        }
        acc.layout = info.layout;
    }
    acc
}

fn stage_pair(
    prev: &Accumulated,
    next: &Accumulated,
) -> (vk::PipelineStageFlags, vk::PipelineStageFlags) {
    // `cmp r,1; adc r,0`（0→1）与 `test; cmovne eax,0x2000`：空阶段回落到 TOP / BOTTOM_OF_PIPE。
    let src = if prev.stages == 0 {
        TOP_OF_PIPE
    } else {
        prev.stages
    };
    let dst = if next.stages == 0 {
        BOTTOM_OF_PIPE
    } else {
        next.stages
    };
    (
        vk::PipelineStageFlags::from_raw(src),
        vk::PipelineStageFlags::from_raw(dst),
    )
}

/// 原 `vk_sync_fork::get_memory_barrier`（0x140c107c0）。逻辑与公开 vk-sync-fork 0.4.0 一致：
/// src access 只收写访问；dst access 仅当 src access 非 0 时才收（WAR/RAR 无需可见性）。
pub fn get_memory_barrier(
    previous: &[u8],
    next: &[u8],
) -> (
    vk::PipelineStageFlags,
    vk::PipelineStageFlags,
    vk::MemoryBarrier<'static>,
) {
    let (p, n) = (accumulate(previous), accumulate(next));
    let dst_access = if p.write_access != 0 { n.all_access } else { 0 };
    let (src, dst) = stage_pair(&p, &n);
    let barrier = vk::MemoryBarrier::default()
        .src_access_mask(vk::AccessFlags::from_raw(p.write_access))
        .dst_access_mask(vk::AccessFlags::from_raw(dst_access));
    (src, dst, barrier)
}

/// `get_buffer_memory_barrier`（0x140c10940）的输入：访问码切片 + 队列族/缓冲/范围（透传）。
#[derive(Clone, Copy, Debug)]
pub struct BufferBarrier<'a> {
    pub previous: &'a [u8],
    pub next: &'a [u8],
    pub src_queue_family_index: u32,
    pub dst_queue_family_index: u32,
    pub buffer: vk::Buffer,
    pub offset: u64,
    pub size: u64,
}

/// 原 `get_buffer_memory_barrier`：访问掩码规则同 [`get_memory_barrier`]（dst 仅当 src 有写时收）。
pub fn get_buffer_memory_barrier(
    barrier: &BufferBarrier<'_>,
) -> (
    vk::PipelineStageFlags,
    vk::PipelineStageFlags,
    vk::BufferMemoryBarrier<'static>,
) {
    let (p, n) = (accumulate(barrier.previous), accumulate(barrier.next));
    let dst_access = if p.write_access != 0 { n.all_access } else { 0 };
    let (src, dst) = stage_pair(&p, &n);
    let out = vk::BufferMemoryBarrier::default()
        .src_access_mask(vk::AccessFlags::from_raw(p.write_access))
        .dst_access_mask(vk::AccessFlags::from_raw(dst_access))
        .src_queue_family_index(barrier.src_queue_family_index)
        .dst_queue_family_index(barrier.dst_queue_family_index)
        .buffer(barrier.buffer)
        .offset(barrier.offset)
        .size(barrier.size);
    (src, dst, out)
}

/// `get_image_memory_barrier`（0x140c10af0）的输入。
#[derive(Clone, Copy, Debug)]
pub struct ImageBarrier<'a> {
    pub previous: &'a [u8],
    pub next: &'a [u8],
    pub previous_layout: BarrierLayout,
    pub next_layout: BarrierLayout,
    /// 为真时 `old_layout = UNDEFINED`（原版 `+0x46` 字节）。
    pub discard_contents: bool,
    pub src_queue_family_index: u32,
    pub dst_queue_family_index: u32,
    pub image: vk::Image,
    pub range: vk::ImageSubresourceRange,
}

fn resolve_layout(mode: BarrierLayout, codes: &[u8], optimal: i32) -> i32 {
    match mode {
        BarrierLayout::Optimal => optimal,
        // 原版逐码覆盖：Present(0x27) → PRESENT_SRC_KHR(0x3b9acdea)，其余 → GENERAL(1)；空切片保持 0。
        BarrierLayout::General => codes
            .last()
            .map_or(0, |&c| if c == PRESENT { 1_000_001_002 } else { 1 }),
    }
}

/// 原 `get_image_memory_barrier`：与公开 vk-sync-fork 0.4.0 一致；image 的 dst access 对所有 next 码无条件收
/// （0x140c10c52..0x140c10c56，不像 memory/buffer 版那样受 src 有写的条件约束）。
pub fn get_image_memory_barrier(
    barrier: &ImageBarrier<'_>,
) -> (
    vk::PipelineStageFlags,
    vk::PipelineStageFlags,
    vk::ImageMemoryBarrier<'static>,
) {
    let (p, n) = (accumulate(barrier.previous), accumulate(barrier.next));
    let (src, dst) = stage_pair(&p, &n);
    let old_layout = if barrier.discard_contents || barrier.previous.is_empty() {
        0
    } else {
        resolve_layout(barrier.previous_layout, barrier.previous, p.layout)
    };
    let new_layout = if barrier.next.is_empty() {
        0
    } else {
        resolve_layout(barrier.next_layout, barrier.next, n.layout)
    };
    let out = vk::ImageMemoryBarrier::default()
        .src_access_mask(vk::AccessFlags::from_raw(p.write_access))
        .dst_access_mask(vk::AccessFlags::from_raw(n.all_access))
        .old_layout(vk::ImageLayout::from_raw(old_layout))
        .new_layout(vk::ImageLayout::from_raw(new_layout))
        .src_queue_family_index(barrier.src_queue_family_index)
        .dst_queue_family_index(barrier.dst_queue_family_index)
        .image(barrier.image)
        .subresource_range(barrier.range);
    (src, dst, out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spot_checks_against_the_exe_table() {
        // TransferWrite：stage TRANSFER、access TRANSFER_WRITE、layout TRANSFER_DST_OPTIMAL。
        assert_eq!(
            access_info(0x34),
            Some(AccessInfo {
                stage: 0x1000,
                access: 0x1000,
                layout: 7
            })
        );
        // Present：PRESENT_SRC_KHR。
        assert_eq!(access_info(0x27).unwrap().layout, 1000001002);
        assert_eq!(
            access_info(0),
            Some(AccessInfo {
                stage: 0,
                access: 0,
                layout: 0
            })
        );
        assert_eq!(access_info(64), None);
    }

    #[test]
    fn writes_are_exactly_0x28_to_0x37() {
        let writes: Vec<u8> = (0..=255u8).filter(|c| is_write(*c)).collect();
        assert_eq!(writes, (0x28..=0x37).collect::<Vec<u8>>());
    }
}
