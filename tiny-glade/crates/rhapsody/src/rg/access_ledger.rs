//! PassBuilder::add_resource_access 的整个访问账本收集合同。
//! 原入口 0x1409dca90 / 0x140533b10，字段更正依据 SYNTHESIS 11.2。
//! 使用原访问码，不为 0x17/0x19/0x1d/0x1e 猜测用途名称。
use super::resource_info::AccessDeclaration;
use std::sync::atomic::{AtomicU32, Ordering};

static NEXT_SYNC_GROUP: AtomicU32 = AtomicU32::new(2);

/// 原 0x140bece50 返回 fetch_add 的旧值；不额外跳过溢出后的 0。
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

    /// 原 0x140bece60 只比较 u32，0 与 0 也等价。
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
    counter.fetch_add(1, Ordering::Relaxed)
}

/// 原账本的 12 字节投影；保留填充字节，不用 Rust 默认布局冒充原 ABI。
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
            reads_previous: matches!(self.access_code, 0x36 | 0x37),
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

/// 对应原不兼容访问 panic 的安全候选错误；不编造完整原 panic/unwind ABI。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IncompatibleAccess {
    pub resource_index: u32,
    pub previous_access_code: u8,
    pub previous_group: u32,
    pub new_access_code: u8,
    pub new_group: u32,
}

/// 输入必须是当前 pass 的账本，索引已经由 graph.check_handle 检查。
/// 原 per-resource {pass,index} 表的效果在这里投影为查找当前 pass 最后一条记录。
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
        matches!(access_code, 0x36 | 0x37),
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
        });
        return Ok(LedgerUpdate::Appended { index });
    };

    let previous = entries[previous_index];
    let same_group = ResourceSyncGroup::from_raw(previous.synchronization_group)
        .is_equivalent(ResourceSyncGroup::from_raw(synchronization_group));
    if previous.access_code == access_code && same_group {
        return Ok(LedgerUpdate::Deduplicated {
            index: previous_index,
        });
    }

    if same_group {
        match (previous.access_code, access_code) {
            (0x17, 0x1d) | (0x1d, 0x17) => {
                entries[previous_index].access_code = 0x1e;
                return Ok(LedgerUpdate::MergedInPlace {
                    index: previous_index,
                });
            }
            (0x1a, 0x02) => {
                entries[previous_index].access_code = 0x02;
                return Ok(LedgerUpdate::MergedInPlace {
                    index: previous_index,
                });
            }
            _ => {}
        }
    }

    // 原两组码 0x19/0x1a 接新 0x37 时，即使 group 不同也升级。
    // 必须先修改旧条，再追加新条，且两个 group 均清 0；ASCII '7' 是 0x37。
    if matches!(previous.access_code, 0x19 | 0x1a) && access_code == 0x37 {
        entries[previous_index].access_code = 0x37;
        entries[previous_index].synchronization_group = 0;
        entries[previous_index].reads_previous = true;
        let appended = entries.len();
        entries.push(AccessDeclaration {
            resource_index,
            synchronization_group: 0,
            access_code: 0x37,
            range,
            version,
            reads_previous: true,
        });
        return Ok(LedgerUpdate::UpgradedAndAppended {
            previous: previous_index,
            appended,
        });
    }

    // 原错误在条目更新之前产生；已经成功的其它资源访问保持原顺序和内容。
    Err(IncompatibleAccess {
        resource_index,
        previous_access_code: previous.access_code,
        previous_group: previous.synchronization_group,
        new_access_code: access_code,
        new_group: synchronization_group,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rg::resource_info::{OriginKind, ResourceInfo};

    #[test]
    fn duplicate_and_in_place_merge_keep_first_resource_order() {
        let mut entries = Vec::new();
        collect_resource_access(&mut entries, 5, 0x17, 9).unwrap();
        collect_resource_access(&mut entries, 3, 0x1a, 0).unwrap();
        assert_eq!(
            collect_resource_access(&mut entries, 5, 0x17, 9),
            Ok(LedgerUpdate::Deduplicated { index: 0 })
        );
        collect_resource_access(&mut entries, 5, 0x1d, 9).unwrap();
        collect_resource_access(&mut entries, 3, 0x02, 0).unwrap();
        assert_eq!(
            entries
                .iter()
                .map(|e| (e.resource_index, e.synchronization_group, e.access_code))
                .collect::<Vec<_>>(),
            [(5, 9, 0x1e), (3, 0, 0x02)]
        );
        let mut reverse = Vec::new();
        collect_resource_access(&mut reverse, 0, 0x1d, 2).unwrap();
        collect_resource_access(&mut reverse, 0, 0x17, 2).unwrap();
        assert_eq!(reverse[0].access_code, 0x1e);
    }

    #[test]
    fn upgrade_appends_and_calculate_consumes_group_code_without_swapping() {
        use crate::rg::graph::{BufferDesc, BuildError, Descriptor, OriginSource, RenderGraph};
        use crate::rg::resource::{ResourceHandle, ResourceOrigin};
        let mut graph = RenderGraph::new();
        let handles: Vec<_> = (0..2)
            .map(|index| {
                graph
                    .create_raw_resource(
                        format!("buffer-{index}"),
                        ResourceOrigin {
                            descriptor: Descriptor::Buffer(BufferDesc {
                                word_at_00: 16,
                                word_at_08: 0,
                                bytes_at_0c_through_0e: [0; 3],
                            }),
                            source: OriginSource::ImportUnowned {
                                token: index + 1,
                                initial_access: 0,
                            },
                        },
                    )
                    .unwrap()
            })
            .collect();
        let mut pass = graph.add_pass("访问升级");
        assert!(matches!(
            pass.add_resource_access(ResourceHandle::from_raw(99), 0x19, 3),
            Err(BuildError::InvalidHandle(99))
        ));
        pass.add_resource_access(handles[0], 0x19, 3).unwrap();
        pass.add_resource_access(handles[1], 0x24, 0).unwrap();
        pass.add_resource_access(handles[0], 0x37, 17).unwrap();
        pass.add_resource_access(handles[0], 0x37, 0).unwrap();
        pass.render(|_| true);
        let built = graph.build();
        let entries = &built.declaration.passes[0].accesses;
        assert_eq!(entries.len(), 3);
        assert_eq!(
            (entries[0].synchronization_group, entries[0].access_code),
            (0, 0x37)
        );
        assert_eq!(
            (entries[2].synchronization_group, entries[2].access_code),
            (0, 0x37)
        );
        let origins = [OriginKind::Tag1 { type_code: 1 }; 2];
        let mut info = ResourceInfo::default();
        info.calculate(&origins, &[], &[entries], |_| 0).unwrap();
        assert_eq!(info.accesses.len(), 3);
        assert_eq!(info.accesses[2].previous_access_index, 0);
        assert_eq!(info.accesses[0].next_access_index, 2);
        assert_eq!(info.accesses[2].access_code, 0x37);
        assert_eq!(info.accesses[2].synchronization_group, 0);
        assert_eq!(info.accesses[1].resource_index, 1);
    }

    #[test]
    fn conflict_preserves_prior_entries_and_group_equality_is_exact() {
        let mut entries = Vec::new();
        collect_resource_access(&mut entries, 0, 0x37, 3).unwrap();
        let before = entries.clone();
        let error = collect_resource_access(&mut entries, 0, 0x37, 4).unwrap_err();
        assert_eq!(error.previous_group, 3);
        assert_eq!(error.new_group, 4);
        assert_eq!(entries, before);
        assert!(ResourceSyncGroup::NONE.is_equivalent(ResourceSyncGroup::NONE));
        assert!(!ResourceSyncGroup::NONE.is_equivalent(ResourceSyncGroup::from_raw(2)));
        let mut asymmetric = Vec::new();
        collect_resource_access(&mut asymmetric, 0, 0x02, 0).unwrap();
        assert!(collect_resource_access(&mut asymmetric, 0, 0x1a, 0).is_err());
    }

    #[test]
    fn group_counter_and_twelve_byte_ledger_projection() {
        let counter = AtomicU32::new(2);
        assert_eq!(allocate_group(&counter), 2);
        assert_eq!(allocate_group(&counter), 3);
        counter.store(u32::MAX, Ordering::Relaxed);
        assert_eq!(allocate_group(&counter), u32::MAX);
        assert_eq!(allocate_group(&counter), 0);
        assert_eq!(std::mem::size_of::<AccessLedgerEntry>(), 12);
        assert_eq!(
            std::mem::offset_of!(AccessLedgerEntry, synchronization_group),
            4
        );
        assert_eq!(std::mem::offset_of!(AccessLedgerEntry, access_code), 8);
    }
}
