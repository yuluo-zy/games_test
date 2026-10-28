//! PassBuilder::add_resource_access 的整个访问账本收集合同。
//! 直接使用访问码，不为 0x17/0x19/0x1d/0x1e 等未命名访问码猜测用途名称。
use super::resource_info::{AccessDeclaration, PreviousContentRead};
use std::sync::atomic::{AtomicU32, Ordering};

static NEXT_SYNC_GROUP: AtomicU32 = AtomicU32::new(2);

/// 同步组编号：全局单调分配；耗尽时终止分配，不复用仍可存活的编号。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ResourceSyncGroup(u32);

impl ResourceSyncGroup {
    pub const NONE: Self = Self(0);

    pub fn new() -> Self {
        Self(allocate_group(&NEXT_SYNC_GROUP))
    }

    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }

    pub const fn raw(self) -> u32 {
        self.0
    }

    /// 只比较 u32 编号，0 与 0 也等价。
    pub const fn is_equivalent(self, other: Self) -> bool {
        self.0 == other.0
    }
}

impl Default for ResourceSyncGroup {
    fn default() -> Self {
        Self::new()
    }
}

fn allocate_group(counter: &AtomicU32) -> u32 {
    counter.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |next| next.checked_add(1))
        .expect("resource synchronization group identifiers exhausted")
}

/// 一条访问账本记录（12 字节，显式填充）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct AccessLedgerEntry {
    pub resource_index: u32,
    pub synchronization_group: u32,
    pub access_code: u8,
    pub unobserved_padding: [u8; 3],
}

impl AccessLedgerEntry {
    pub fn declaration(self) -> AccessDeclaration {
        AccessDeclaration {
            resource_index: self.resource_index,
            synchronization_group: self.synchronization_group,
            access_code: self.access_code,
            // Original B's twelve-byte ledger carries a synchronization mask,
            // not a promise that a shader reads prior contents. blur0 is an
            // original first write to a fresh image with code 0x37.
            reads_previous: false,
            ..Default::default()
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LedgerUpdate {
    Appended { index: usize },
    Deduplicated { index: usize },
    MergedInPlace { index: usize },
    UpgradedAndAppended { previous: usize, appended: usize },
}

/// 同一 pass 内对同一资源声明了不兼容访问。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IncompatibleAccess {
    pub resource_index: u32,
    pub previous_access_code: u8,
    pub previous_group: u32,
    pub new_access_code: u8,
    pub new_group: u32,
}

/// 输入必须是当前 pass 的账本，索引已经由 graph.check_handle 检查。
/// 查找当前 pass 对该资源的最后一条记录。
/// 不跨 pass 去重，不排序，不把 read→write 的两条记录压成一条。
pub fn collect_resource_access(
    entries: &mut Vec<AccessDeclaration>,
    resource_index: u32,
    access_code: u8,
    synchronization_group: u32,
) -> Result<LedgerUpdate, IncompatibleAccess> {
    collect_resource_access_range(
        entries,
        resource_index,
        access_code,
        synchronization_group,
        super::resource::AccessRange::Whole,
        super::resource::AccessVersion::Automatic,
        false,
    )
}

pub fn collect_resource_access_range(
    entries: &mut Vec<AccessDeclaration>,
    resource_index: u32,
    access_code: u8,
    synchronization_group: u32,
    range: super::resource::AccessRange,
    version: super::resource::AccessVersion,
    reads_previous: bool,
) -> Result<LedgerUpdate, IncompatibleAccess> {
    collect_resource_access_range_with_contents(
        entries,
        resource_index,
        access_code,
        synchronization_group,
        range,
        version,
        reads_previous,
        false,
    )
}

#[allow(clippy::too_many_arguments)]
pub fn collect_resource_access_range_with_contents(
    entries: &mut Vec<AccessDeclaration>,
    resource_index: u32,
    access_code: u8,
    synchronization_group: u32,
    range: super::resource::AccessRange,
    version: super::resource::AccessVersion,
    reads_previous: bool,
    contents_unused: bool,
) -> Result<LedgerUpdate, IncompatibleAccess> {
    collect_resource_access_range_with_policy(
        entries,
        resource_index,
        access_code,
        synchronization_group,
        range,
        version,
        reads_previous,
        contents_unused,
        false,
    )
}

#[allow(clippy::too_many_arguments)]
pub fn collect_resource_access_range_with_policy(
    entries: &mut Vec<AccessDeclaration>,
    resource_index: u32,
    access_code: u8,
    synchronization_group: u32,
    range: super::resource::AccessRange,
    version: super::resource::AccessVersion,
    reads_previous: bool,
    contents_unused: bool,
    allow_undefined_contents: bool,
) -> Result<LedgerUpdate, IncompatibleAccess> {
    let Some(previous_index) = entries.iter().rposition(|entry| {
        entry.resource_index == resource_index && entry.range == range && entry.version == version
    }) else {
        let index = entries.len();
        entries.push(AccessDeclaration {
            resource_index,
            synchronization_group,
            access_code,
            range,
            version,
            reads_previous,
            contents_unused,
            allow_undefined_contents,
            previous_content_read: PreviousContentRead::Inferred,
        });
        return Ok(LedgerUpdate::Appended { index });
    };

    let previous = entries[previous_index];
    let same_group = ResourceSyncGroup::from_raw(previous.synchronization_group)
        .is_equivalent(ResourceSyncGroup::from_raw(synchronization_group));
    if previous.access_code == access_code && same_group {
        // Any active binding to this same range still consumes its contents.
        merge_content_policy(
            &mut entries[previous_index],
            access_code,
            reads_previous,
            contents_unused,
            allow_undefined_contents,
        );
        return Ok(LedgerUpdate::Deduplicated {
            index: previous_index,
        });
    }

    if same_group {
        match (previous.access_code, access_code) {
            (0x17, 0x1d) | (0x1d, 0x17) => {
                entries[previous_index].access_code = 0x1e;
                merge_content_policy(
                    &mut entries[previous_index],
                    access_code,
                    reads_previous,
                    contents_unused,
                    allow_undefined_contents,
                );
                return Ok(LedgerUpdate::MergedInPlace {
                    index: previous_index,
                });
            }
            (0x1a, 0x02) => {
                entries[previous_index].access_code = 0x02;
                merge_content_policy(
                    &mut entries[previous_index],
                    access_code,
                    reads_previous,
                    contents_unused,
                    allow_undefined_contents,
                );
                return Ok(LedgerUpdate::MergedInPlace {
                    index: previous_index,
                });
            }
            _ => {}
        }
    }

    // 0x19/0x1a 后接 0x37 时，即使 group 不同也升级。
    // 必须先修改旧条，再追加新条，且两个 group 均清 0；ASCII '7' 是 0x37。
    if matches!(previous.access_code, 0x19 | 0x1a) && access_code == 0x37 {
        let incoming = AccessDeclaration {
            access_code,
            reads_previous,
            contents_unused,
            allow_undefined_contents,
            ..Default::default()
        };
        if !previous.valid_content_mode(false) || !incoming.valid_content_mode(true) {
            return Err(IncompatibleAccess {
                resource_index,
                previous_access_code: previous.access_code,
                previous_group: previous.synchronization_group,
                new_access_code: access_code,
                new_group: synchronization_group,
            });
        }
        let previous_content = previous.content_read(false);
        let incoming_content = incoming.content_read(true);
        entries[previous_index].access_code = 0x37;
        entries[previous_index].synchronization_group = 0;
        entries[previous_index].previous_content_read = previous_content;
        entries[previous_index].reads_previous = true;
        entries[previous_index].contents_unused = false;
        entries[previous_index].allow_undefined_contents = false;
        let appended = entries.len();
        entries.push(AccessDeclaration {
            resource_index,
            synchronization_group: 0,
            access_code: 0x37,
            range,
            version,
            reads_previous: true,
            contents_unused: false,
            allow_undefined_contents: false,
            previous_content_read: incoming_content,
        });
        return Ok(LedgerUpdate::UpgradedAndAppended {
            previous: previous_index,
            appended,
        });
    }

    // 错误在条目更新之前产生；已经成功的其它资源访问保持原顺序和内容。
    Err(IncompatibleAccess {
        resource_index,
        previous_access_code: previous.access_code,
        previous_group: previous.synchronization_group,
        new_access_code: access_code,
        new_group: synchronization_group,
    })
}

// A normal content-consuming alias must win over a permissive read, while
// an inactive alias imposes no content requirement of its own. Keep the native
// access-code/group merging rules above independent of this source capability.
fn merge_content_policy(
    previous: &mut AccessDeclaration,
    access_code: u8,
    reads_previous: bool,
    unused: bool,
    allow_undefined: bool,
) {
    if previous.has_saved_content_read() {
        let incoming = AccessDeclaration {
            access_code,
            reads_previous,
            contents_unused: unused,
            allow_undefined_contents: allow_undefined,
            ..Default::default()
        };
        // Keep malformed caller metadata visible to the normal validator.
        // A genuine preserving write tightens the saved output; a duplicate
        // pure write never turns the saved real read into an invented RW read.
        if !incoming.valid_content_mode(crate::native::is_write(access_code)) {
            previous.contents_unused |= unused;
            previous.allow_undefined_contents |= allow_undefined;
            return;
        }
        use PreviousContentRead::{DefinedRequired, NotRead, UndefinedAllowed};
        previous.previous_content_read = match (
            previous.content_read(true),
            incoming.content_read(crate::native::is_write(access_code)),
        ) {
            (DefinedRequired, _) | (_, DefinedRequired) => DefinedRequired,
            (UndefinedAllowed, _) | (_, UndefinedAllowed) => UndefinedAllowed,
            _ => NotRead,
        };
        return;
    }
    // Aliases of this exact range/version retain every previous-content
    // dependency in either order. Do not erase inactive/undefined evidence:
    // the normal graph contract still rejects preserving metadata on reads.
    previous.reads_previous |= reads_previous;
    let requires_defined = (!previous.contents_unused && !previous.allow_undefined_contents)
        || (!unused && !allow_undefined);
    previous.contents_unused &= unused;
    previous.allow_undefined_contents = !previous.contents_unused && !requires_defined;
}

#[cfg(test)]
mod tests;
