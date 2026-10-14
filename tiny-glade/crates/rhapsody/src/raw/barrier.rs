// 候选实现迁移来源：reconstruction/render/work-packages/barrier-record-r5/src/lib.rs.
// 这是行为重建，不是原作者源码。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Access {
    pub code: u8,
    pub group: u32,
}

/// 一个原同步scope汇总三类数组与阶段掩码，在phase边界一次提交。
pub struct SynchronizationScope {
    pub memory: Vec<ash::vk::MemoryBarrier<'static>>,
    pub buffers: Vec<ash::vk::BufferMemoryBarrier<'static>>,
    pub images: Vec<ash::vk::ImageMemoryBarrier<'static>>,
    pub source_stages: ash::vk::PipelineStageFlags,
    pub destination_stages: ash::vk::PipelineStageFlags,
}
impl Default for SynchronizationScope {
    fn default() -> Self {
        Self {
            memory: Vec::new(),
            buffers: Vec::new(),
            images: Vec::new(),
            source_stages: ash::vk::PipelineStageFlags::from_raw(1),
            destination_stages: ash::vk::PipelineStageFlags::from_raw(0x2000),
        }
    }
}
impl SynchronizationScope {
    pub fn is_empty(&self) -> bool {
        self.memory.is_empty() && self.buffers.is_empty() && self.images.is_empty()
    }
}

/// 一条 buffer 内存屏障；ownership/queue 字段默认 EXCLUSIVE。
#[derive(Clone, Copy, Debug)]
pub struct BufferBarrier(pub ash::vk::BufferMemoryBarrier<'static>);
impl BufferBarrier {
    pub fn new(
        src_access: ash::vk::AccessFlags,
        dst_access: ash::vk::AccessFlags,
        buffer: ash::vk::Buffer,
        offset: u64,
        size: u64,
    ) -> Self {
        Self(
            ash::vk::BufferMemoryBarrier::default()
                .src_access_mask(src_access)
                .dst_access_mask(dst_access)
                .src_queue_family_index(ash::vk::QUEUE_FAMILY_IGNORED)
                .dst_queue_family_index(ash::vk::QUEUE_FAMILY_IGNORED)
                .buffer(buffer)
                .offset(offset)
                .size(size),
        )
    }
}

/// 一条 image 内存屏障；layout 迁移语义由调用方阶段决定。
#[derive(Clone, Copy, Debug)]
pub struct ImageBarrier(pub ash::vk::ImageMemoryBarrier<'static>);
impl ImageBarrier {
    pub fn new(
        src_access: ash::vk::AccessFlags,
        dst_access: ash::vk::AccessFlags,
        old_layout: ash::vk::ImageLayout,
        new_layout: ash::vk::ImageLayout,
        image: ash::vk::Image,
        subresource_range: ash::vk::ImageSubresourceRange,
    ) -> Self {
        Self(
            ash::vk::ImageMemoryBarrier::default()
                .src_access_mask(src_access)
                .dst_access_mask(dst_access)
                .old_layout(old_layout)
                .new_layout(new_layout)
                .src_queue_family_index(ash::vk::QUEUE_FAMILY_IGNORED)
                .dst_queue_family_index(ash::vk::QUEUE_FAMILY_IGNORED)
                .image(image)
                .subresource_range(subresource_range),
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Resource {
    pub kind: u8,
    pub initial: u8,
    pub reused: bool,
    pub validity: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Transition {
    pub kind: u8,
    pub from: u8,
    pub to: u8,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Outcome {
    pub transitions: Vec<Transition>,
    pub panic: Option<&'static str>,
    pub pending_length: usize,
    pub tracked: usize,
    pub emitted: usize,
    pub resets: usize,
}

pub fn concurrent(current: Access, previous: Access) -> bool {
    // 原 can_be_concurrent_with 使用访问码区间，不使用后端为 Vulkan 投影提供的 write 标志。
    let needs_group = |code: u8| code.wrapping_sub(0x28) < 0x10;
    current.code == previous.code
        && (!(needs_group(current.code) || needs_group(previous.code))
            || (current.group != 0 && current.group == previous.group))
}

fn validate(resource: Resource) -> Result<(), &'static str> {
    if resource.kind > 1 {
        return Err("kind");
    }
    match resource.validity {
        1 => Err("device"),
        2 => Err("managed-kind"),
        3 => Err("generation"),
        _ => Ok(()),
    }
}

pub fn pre(resource: Resource, previous: Option<Access>, current: Access) -> Outcome {
    let mut outcome = Outcome::default();
    let from = match previous {
        Some(previous) => {
            if resource.kind > 1 {
                outcome.panic = Some("kind");
                return outcome;
            }
            if concurrent(current, previous) {
                return outcome;
            }
            previous.code
        }
        None => {
            if !resource.reused {
                return outcome;
            }
            if resource.initial != 0 {
                outcome.panic = Some("initial-nonzero");
                return outcome;
            }
            0
        }
    };
    if let Err(panic) = validate(resource) {
        outcome.panic = Some(panic);
        return outcome;
    }
    outcome.transitions.push(Transition {
        kind: resource.kind,
        from,
        to: current.code,
    });
    outcome
}

pub fn initial(pending: &mut Vec<(Resource, Option<Access>)>, command_present: bool) -> Outcome {
    let mut outcome = Outcome::default();
    if !command_present {
        outcome.panic = Some("command-empty");
        outcome.pending_length = pending.len();
        return outcome;
    }
    let drained = std::mem::take(pending);
    for (resource, first) in drained {
        let Some(first) = first else { continue };
        if resource.reused {
            continue;
        }
        if let Err(panic) = validate(resource) {
            outcome.panic = Some(panic);
            return outcome;
        }
        outcome.tracked += 1;
        outcome.transitions.push(Transition {
            kind: resource.kind,
            from: resource.initial,
            to: first.code,
        });
    }
    outcome.emitted = usize::from(!outcome.transitions.is_empty());
    outcome.resets = 1;
    outcome
}

pub fn export(resource: Resource, last: Option<Access>, target: u8, final_batch: bool) -> Outcome {
    let mut outcome = Outcome::default();
    if !final_batch {
        return outcome;
    }
    if let Err(panic) = validate(resource) {
        outcome.panic = Some(panic);
        return outcome;
    }
    outcome.tracked = 1;
    outcome.transitions.push(Transition {
        kind: resource.kind,
        from: last.map_or(resource.initial, |access| access.code),
        to: target,
    });
    outcome.emitted = 1;
    outcome.resets = 1;
    outcome
}
