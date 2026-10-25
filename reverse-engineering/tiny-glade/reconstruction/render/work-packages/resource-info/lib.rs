use std::collections::HashSet;

use tg_rhapsody_reconstruction::resource_info::{
    NO_ACCESS, PassEndpoint, ResourceAccessRecord, ResourceLifetimeRecord,
    calculate_lifetime_events, link_up_next_access_chains,
};
use tg_rhapsody_reconstruction::resource_usage::{
    buffer_access_mask_to_usage_flags, image_access_mask_to_usage_flags,
};

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
    fn type_code(self) -> u8 {
        match self {
            Self::Tag0 {
                descriptor_code: 7, ..
            } => 1,
            Self::Tag0 {
                descriptor_code: 8, ..
            } => panic!("unsupported descriptor type"),
            Self::Tag0 { .. } => 0,
            Self::Tag1 { type_code } | Self::Tag2 { type_code } => type_code,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AccessDeclaration {
    pub resource_index: u32,
    pub synchronization_group: u32,
    pub access_code: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExportDeclaration {
    pub tag: u8,
    pub resource_index: u32,
    pub access_code: u8,
}

#[derive(Default, Debug)]
pub struct ResourceInfo {
    pub accesses: Vec<ResourceAccessRecord>,
    pub pass_access_ranges: Vec<(ResourceAccessIndex, ResourceAccessIndex)>,
    pub lifetimes: Vec<ResourceLifetimeRecord>,
    pub image_usage_flags: Vec<u32>,
    pub buffer_usage_flags: Vec<u32>,
    pub lifetime_events: Vec<u32>,
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
        mut dependency_access_mask: impl FnMut(u8) -> u32,
    ) {
        self.clear();
        self.lifetimes.reserve(origins.len());
        for origin in origins {
            let external = !matches!(origin, OriginKind::Tag0 { .. });
            self.lifetimes.push(ResourceLifetimeRecord {
                first_pass: PassEndpoint {
                    tag: u32::from(external),
                    pass_index: 0,
                },
                last_pass: PassEndpoint {
                    tag: u32::from(external && !passes.is_empty()),
                    pass_index: if external && !passes.is_empty() {
                        passes.len().wrapping_sub(1) as u32
                    } else {
                        0
                    },
                },
                first_access_index: NO_ACCESS,
                last_access_index: NO_ACCESS,
            });
        }
        self.image_usage_flags.resize(origins.len(), 0);
        self.buffer_usage_flags.resize(origins.len(), 0);
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
        for export in exports {
            let access_mask = dependency_access_mask(export.access_code);
            let resource_index =
                u16::try_from(export.resource_index).expect("export index exceeds u16");
            self.exported_indices.insert(resource_index);
            if access_mask != 0 {
                if export.tag & 1 == 0 {
                    let usage = &mut self.image_usage_flags[usize::from(resource_index)];
                    *usage |= image_access_mask_to_usage_flags(access_mask).as_raw();
                } else {
                    let usage = &mut self.buffer_usage_flags[usize::from(resource_index)];
                    *usage |= buffer_access_mask_to_usage_flags(access_mask).as_raw();
                }
            }
        }
        for (pass_index, pass) in passes.iter().enumerate() {
            let start = self.accesses.len();
            for declaration in *pass {
                let resource_index = declaration.resource_index as usize;
                let lifetime = &mut self.lifetimes[resource_index];
                let access_index = ResourceAccessIndex::from(self.accesses.len()).0;
                self.accesses.push(ResourceAccessRecord {
                    resource_index: declaration.resource_index,
                    previous_access_index: lifetime.last_access_index,
                    next_access_index: NO_ACCESS,
                    synchronization_group: declaration.synchronization_group,
                    access_code: declaration.access_code,
                    unobserved_padding: [0; 3],
                });
                let current_pass = PassIndex::from(pass_index).0;
                if lifetime.first_pass.tag & 1 == 0 {
                    lifetime.first_pass = PassEndpoint {
                        tag: 1,
                        pass_index: current_pass,
                    };
                }
                if lifetime.last_pass.tag == 1 {
                    lifetime.last_pass.pass_index = lifetime.last_pass.pass_index.max(current_pass);
                } else {
                    lifetime.last_pass = PassEndpoint {
                        tag: 1,
                        pass_index: current_pass,
                    };
                }
                if lifetime.first_access_index == NO_ACCESS {
                    lifetime.first_access_index = access_index;
                }
                lifetime.last_access_index = access_index;
                let access_mask = dependency_access_mask(declaration.access_code);
                match origins[resource_index].type_code() {
                    1 => {
                        let usage = &mut self.buffer_usage_flags[resource_index];
                        *usage |= buffer_access_mask_to_usage_flags(access_mask).as_raw();
                    }
                    2 => {}
                    _ => {
                        let usage = &mut self.image_usage_flags[resource_index];
                        *usage |= image_access_mask_to_usage_flags(access_mask).as_raw();
                    }
                }
            }
            self.pass_access_ranges.push((
                ResourceAccessIndex::from(start),
                ResourceAccessIndex::from(self.accesses.len()),
            ));
        }
        link_up_next_access_chains(&mut self.accesses);
        calculate_lifetime_events(&self.lifetimes, &mut self.lifetime_events);
    }
}
