//! Per-plan resource metadata: access chains, lifetimes, usage flags and the
//! ordered acquire/release events used to bind physical resources.
use std::collections::HashSet;

use crate::native::resource_usage::{buffer_usage_for_access, image_usage_for_access};
use ash::vk::AccessFlags;

/// Marks an absent previous/next/first/last access index.
pub const NO_ACCESS: u32 = u32::MAX;

/// One resource access in declaration order, linked to the same resource's
/// previous and next access.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResourceAccessRecord {
    pub resource_index: u32,
    pub previous_access_index: u32,
    pub next_access_index: u32,
    pub synchronization_group: u32,
    pub access_code: u8,
}

/// Retained-pass span and first/last access of one resource. External
/// resources are live for the whole graph.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResourceLifetimeRecord {
    pub first_pass: Option<u32>,
    pub last_pass: Option<u32>,
    pub first_access_index: u32,
    pub last_access_index: u32,
}

/// A resource becomes live (`release == false`) or dead at `pass`. Events sort
/// by pass, acquisitions before releases, then by resource.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LifetimeEvent {
    pub pass: u32,
    pub release: bool,
    pub resource: u16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PassIndex(pub u32);

impl From<usize> for PassIndex {
    fn from(value: usize) -> Self {
        Self(u32::try_from(value).expect("pass index exceeds u32"))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResourceAccessIndex(pub u32);

impl From<usize> for ResourceAccessIndex {
    fn from(value: usize) -> Self {
        Self(u32::try_from(value).expect("access index exceeds u32"))
    }
}

impl ResourceAccessIndex {
    /// `NO_ACCESS` 的原值用 u32::MAX 表示"无前驱/后继访问"；换算成裸索引失败。
    pub fn index(self) -> u32 {
        self.0
    }


    pub fn valid(self) -> bool {
        self.0 != NO_ACCESS
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OriginKind {
    Tag0 {
        descriptor_code: u8,
        image_creation_flags: u32,
        buffer_creation_flags: u32,
    },
    Tag1 {
        type_code: u8,
    },
    Tag2 {
        type_code: u8,
    },
}

impl OriginKind {
    fn type_code(self) -> crate::Result<u8> {
        Ok(match self {
            Self::Tag0 {
                descriptor_code: 7, ..
            } => 1,
            Self::Tag0 {
                descriptor_code: 8, ..
            } => return Err(crate::Error::invalid_usage("unsupported descriptor type")),
            Self::Tag0 { .. } => 0,
            Self::Tag1 { type_code } | Self::Tag2 { type_code } => type_code,
        })
    }
}

/// Content intent saved before a native sampled-read/general-write upgrade.
/// This is graph metadata, not part of the original twelve-byte native ledger.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum PreviousContentRead {
    #[default]
    Inferred,
    NotRead,
    UndefinedAllowed,
    DefinedRequired,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct AccessDeclaration {
    pub resource_index: u32,
    pub synchronization_group: u32,
    pub access_code: u8,
    pub range: super::resource::AccessRange,
    pub version: super::resource::AccessVersion,
    /// LOAD and in-place updates consume the contents of the previous version.
    pub reads_previous: bool,
    /// Current graph capability: the configured shader cannot read these
    /// contents. Binding, native access, range and lifetime remain declared.
    /// This is not a field recovered from the original twelve-byte ledger.
    pub contents_unused: bool,
    /// Explicit caller contract: this read may consume undefined contents.
    /// Unlike contents_unused it remains a real content read, preserving every
    /// available producer, native access, synchronization and lifetime. It
    /// neither initializes a resource nor permits a later ordinary read/export.
    /// This is a current architecture capability, not original ledger ABI.
    pub allow_undefined_contents: bool,
    /// Only native ledger upgrades construct a nondefault saved role. It keeps
    /// the original occurrence's content contract when native scope becomes RW.
    pub previous_content_read: PreviousContentRead,
}
impl AccessDeclaration {
    pub const fn content_read(&self, write: bool) -> PreviousContentRead {
        match self.previous_content_read {
            PreviousContentRead::Inferred => {
                if self.contents_unused || write && !self.reads_previous {
                    PreviousContentRead::NotRead
                } else if self.allow_undefined_contents {
                    PreviousContentRead::UndefinedAllowed
                } else {
                    PreviousContentRead::DefinedRequired
                }
            }
            saved => saved,
        }
    }
    pub const fn reads_contents(&self, write: bool) -> bool {
        !matches!(self.content_read(write), PreviousContentRead::NotRead)
    }
    pub const fn requires_defined_contents(&self, write: bool) -> bool {
        matches!(
            self.content_read(write),
            PreviousContentRead::DefinedRequired
        )
    }
    pub const fn has_saved_content_read(&self) -> bool {
        !matches!(self.previous_content_read, PreviousContentRead::Inferred)
    }
    pub(crate) const fn valid_content_mode(&self, write: bool) -> bool {
        if self.has_saved_content_read() {
            write
                && self.access_code == 0x37
                && self.synchronization_group == 0
                && self.reads_previous
                && !self.contents_unused
                && !self.allow_undefined_contents
        } else {
            let modifies = write || self.reads_previous;
            !(self.reads_previous && !write
                || self.contents_unused && modifies
                || self.allow_undefined_contents && (modifies || self.contents_unused))
        }
    }
}

/// Only resources with a broadened native occurrence need a pass-entry content
/// frontier. Ordinary passes keep their existing allocation-free path.
pub(crate) fn entry_content_resources(
    accesses: &[AccessDeclaration],
) -> smallvec::SmallVec<[u32; 2]> {
    let mut resources = smallvec::SmallVec::new();
    for access in accesses {
        if access.has_saved_content_read() && !resources.contains(&access.resource_index) {
            resources.push(access.resource_index);
        }
    }
    resources
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ExportDeclaration {
    pub tag: u8,
    pub resource_index: u32,
    pub access_code: u8,
    pub range: super::resource::AccessRange,
}

#[derive(Clone, Default, Debug)]
pub struct ResourceInfo {
    pub accesses: Vec<ResourceAccessRecord>,
    pub pass_access_ranges: Vec<(ResourceAccessIndex, ResourceAccessIndex)>,
    pub lifetimes: Vec<ResourceLifetimeRecord>,
    pub image_usage_flags: Vec<u32>,
    pub buffer_usage_flags: Vec<u32>,
    pub lifetime_events: Vec<LifetimeEvent>,
    pub exported_indices: HashSet<u16>,
}

impl ResourceInfo {
    pub fn clear(&mut self) {
        self.accesses.clear();
        self.pass_access_ranges.clear();
        self.lifetimes.clear();
        self.image_usage_flags.clear();
        self.buffer_usage_flags.clear();
        self.exported_indices.clear();
        self.lifetime_events.clear();
    }

    pub fn calculate(
        &mut self,
        origins: &[OriginKind],
        exports: &[ExportDeclaration],
        passes: &[&[AccessDeclaration]],
        mut dependency_access_mask: impl FnMut(u8) -> AccessFlags,
    ) -> crate::Result<()> {
        // Validate every caller-controlled index and usage before mutating the
        // current metadata. The same checks protect the public CPU entry point.
        if origins.len() > usize::from(u16::MAX) || passes.len() > super::graph::MAX_PASSES {
            return Err(crate::Error::invalid_usage(
                "resource/pass count exceeds graph limits",
            ));
        }
        let mut access_count = 0usize;
        let mut masks = [None; crate::native::access_table::ACCESS_CODE_COUNT];
        for origin in origins {
            origin.type_code()?;
        }
        for pass in passes {
            access_count = access_count
                .checked_add(pass.len())
                .filter(|count| *count < NO_ACCESS as usize)
                .ok_or_else(|| {
                    crate::Error::invalid_usage("resource access count exceeds graph limits")
                })?;
            for access in *pass {
                let origin = origins.get(access.resource_index as usize).ok_or_else(|| {
                    crate::Error::invalid_usage("resource access index is out of bounds")
                })?;
                let mask = masks
                    .get_mut(usize::from(access.access_code))
                    .ok_or_else(|| {
                        crate::Error::invalid_usage("resource access code is out of bounds")
                    })?;
                let mask = *mask.get_or_insert_with(|| dependency_access_mask(access.access_code));
                if origin.type_code()? == 0 {
                    image_usage_for_access(mask)?;
                }
            }
        }
        for export in exports {
            let origin = origins.get(export.resource_index as usize).ok_or_else(|| {
                crate::Error::invalid_usage("export resource index is out of bounds")
            })?;
            if export.tag != origin.type_code()? {
                return Err(crate::Error::invalid_usage(
                    "export resource kind differs from its declaration",
                ));
            }
            let mask = masks
                .get_mut(usize::from(export.access_code))
                .ok_or_else(|| {
                    crate::Error::invalid_usage("export access code is out of bounds")
                })?;
            let mask = *mask.get_or_insert_with(|| dependency_access_mask(export.access_code));
            if export.tag == 0 {
                image_usage_for_access(mask)?;
            }
        }
        let mut dependency_access_mask =
            |code: u8| masks[usize::from(code)].unwrap_or(AccessFlags::empty());
        self.clear();
        self.lifetimes.reserve(origins.len());
        for origin in origins {
            let external = !matches!(origin, OriginKind::Tag0 { .. });
            self.lifetimes.push(ResourceLifetimeRecord {
                first_pass: external.then_some(0),
                last_pass: (external && !passes.is_empty()).then(|| (passes.len() - 1) as u32),
                first_access_index: NO_ACCESS,
                last_access_index: NO_ACCESS,
            });
        }
        self.image_usage_flags.resize(origins.len(), 0);
        self.buffer_usage_flags.resize(origins.len(), 0);
        self.extract_creation_flags(origins);
        self.extend_usage_flags_for_exports(exports, &mut dependency_access_mask)?;
        for (pass_index, pass) in passes.iter().enumerate() {
            self.extract_pass_resource_access(
                PassIndex::from(pass_index),
                pass,
                origins,
                &mut dependency_access_mask,
            )?;
        }
        self.link_next_accesses();
        self.calculate_lifetime_events();
        Ok(())
    }

    /// Point every access's predecessor at it.
    fn link_next_accesses(&mut self) {
        for current in 0..self.accesses.len() {
            let previous = self.accesses[current].previous_access_index;
            if previous != NO_ACCESS {
                self.accesses[previous as usize].next_access_index = current as u32;
            }
        }
    }

    /// Two events per live resource, sorted.
    fn calculate_lifetime_events(&mut self) {
        for (resource, lifetime) in self.lifetimes.iter().enumerate() {
            if let (Some(first), Some(last)) = (lifetime.first_pass, lifetime.last_pass) {
                let resource = resource as u16;
                self.lifetime_events.push(LifetimeEvent {
                    pass: first,
                    release: false,
                    resource,
                });
                self.lifetime_events.push(LifetimeEvent {
                    pass: last,
                    release: true,
                    resource,
                });
            }
        }
        self.lifetime_events.sort_unstable();
    }

    /// 把 create 来源（Tag0）声明的创建期 usage 位写入对应资源表。
    /// 导入/导出来源（Tag1/Tag2）没有创建期标志，usage 表保持已 resize 的零值。
    fn extract_creation_flags(&mut self, origins: &[OriginKind]) {
        for (resource_index, origin) in origins.iter().enumerate() {
            if let OriginKind::Tag0 {
                descriptor_code,
                image_creation_flags,
                buffer_creation_flags,
            } = origin
            {
                match descriptor_code {
                    7 => self.buffer_usage_flags[resource_index] = *buffer_creation_flags,
                    8 => {}
                    _ => self.image_usage_flags[resource_index] = *image_creation_flags,
                }
            }
        }
    }

    /// 导出声明本身不产生 pass 访问记录，但要把导出访问码折算的
    /// usage 位并入资源表，并记录已导出的资源索引集合。
    fn extend_usage_flags_for_exports(
        &mut self,
        exports: &[ExportDeclaration],
        dependency_access_mask: &mut impl FnMut(u8) -> AccessFlags,
    ) -> crate::Result<()> {
        for export in exports {
            let access_mask = dependency_access_mask(export.access_code);
            let resource_index =
                u16::try_from(export.resource_index).expect("export index exceeds u16");
            self.exported_indices.insert(resource_index);
            if !access_mask.is_empty() {
                if export.tag & 1 == 0 {
                    let usage = &mut self.image_usage_flags[usize::from(resource_index)];
                    *usage |= image_usage_for_access(access_mask)?.as_raw();
                } else {
                    let usage = &mut self.buffer_usage_flags[usize::from(resource_index)];
                    *usage |= buffer_usage_for_access(access_mask).as_raw();
                }
            }
        }
        Ok(())
    }

    /// 按单个 pass 的访问声明产出 access 记录、串联资源生命周期的
    /// 首/末访问与首/末 pass，并把访问码折算的 usage 位并入资源表。
    fn extract_pass_resource_access(
        &mut self,
        pass_index: PassIndex,
        pass: &[AccessDeclaration],
        origins: &[OriginKind],
        dependency_access_mask: &mut impl FnMut(u8) -> AccessFlags,
    ) -> crate::Result<()> {
        let start = self.accesses.len();
        for declaration in pass {
            let resource_index = declaration.resource_index as usize;
            let lifetime = &mut self.lifetimes[resource_index];
            let access_index = ResourceAccessIndex::from(self.accesses.len()).0;
            self.accesses.push(ResourceAccessRecord {
                resource_index: declaration.resource_index,
                previous_access_index: lifetime.last_access_index,
                next_access_index: NO_ACCESS,
                synchronization_group: declaration.synchronization_group,
                access_code: declaration.access_code,
            });
            let current_pass = pass_index.0;
            lifetime.first_pass.get_or_insert(current_pass);
            lifetime.last_pass = Some(
                lifetime
                    .last_pass
                    .map_or(current_pass, |last| last.max(current_pass)),
            );
            if lifetime.first_access_index == NO_ACCESS {
                lifetime.first_access_index = access_index;
            }
            lifetime.last_access_index = access_index;
            let access_mask = dependency_access_mask(declaration.access_code);
            match origins[resource_index].type_code()? {
                1 => {
                    let usage = &mut self.buffer_usage_flags[resource_index];
                    *usage |= buffer_usage_for_access(access_mask).as_raw();
                }
                2 => {}
                _ => {
                    let usage = &mut self.image_usage_flags[resource_index];
                    *usage |= image_usage_for_access(access_mask)?.as_raw();
                }
            }
        }
        self.pass_access_ranges.push((
            ResourceAccessIndex::from(start),
            ResourceAccessIndex::from(self.accesses.len()),
        ));
        Ok(())
    }
}
