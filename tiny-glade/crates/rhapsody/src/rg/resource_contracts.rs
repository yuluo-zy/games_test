//! Descriptor-only resource validation shared by the production graph planner.
use super::{
    graph::{BuildError, GraphDeclaration, OriginSource},
    resource::{AccessRange, AccessVersion, RangeError, ResourceContractError as Contract},
    resource_info::AccessDeclaration,
};

pub(crate) fn range_error(resource: u32, error: RangeError) -> BuildError {
    BuildError::Contract(match error {
        RangeError::WrongKind => Contract::WrongRangeKind { resource },
        RangeError::InvalidBounds => Contract::InvalidRange { resource },
    })
}

pub fn validate_contracts(
    graph: &GraphDeclaration,
    is_write: impl FnMut(u8) -> bool,
) -> Result<(), BuildError> {
    resolve_contracts(graph, is_write).map(|_| ())
}

/// Resolve the raw access-code adapter into the same version contract used by
/// typed IDs. Imports provide version zero; created resources require a write.
pub fn resolve_contracts(
    graph: &GraphDeclaration,
    is_write: impl FnMut(u8) -> bool,
) -> Result<Vec<Vec<AccessDeclaration>>, BuildError> {
    resolve_with_initialization(graph, is_write).map(|(passes, _)| passes)
}

/// Recheck dynamic import initialization after a complete comparison with a
/// validated normalized definition. Resource indices, ranges, versions and
/// access modes are already identical; only boundary state can invalidate reads.
/// Whole-range writes and initialized imports require no temporary storage.
pub(crate) fn validate_matching_initialization(
    graph: &GraphDeclaration,
    is_write: impl Fn(u8) -> bool + Copy,
) -> Result<(), BuildError> {
    let initialized = |resource: usize| {
        let resource = &graph.resources[resource];
        !matches!(resource.source, OriginSource::Create)
            && (!matches!(resource.descriptor, super::graph::Descriptor::Image(_))
                || !matches!(
                    resource.source,
                    OriginSource::ImportOwned {
                        initial_access: 0,
                        ..
                    } | OriginSource::ImportUnowned {
                        initial_access: 0,
                        ..
                    }
                ))
    };
    for (pass_index, pass) in graph.passes.iter().enumerate() {
        for (access_index, access) in pass.accesses.iter().enumerate() {
            if access.reads_contents(is_write(access.access_code))
                && !access.allow_undefined_contents
                && !initialized(access.resource_index as usize)
            {
                let writes = graph.passes[..=pass_index]
                    .iter()
                    .enumerate()
                    .flat_map(|(index, pass)| {
                        let count = if index == pass_index {
                            access_index
                        } else {
                            pass.accesses.len()
                        };
                        pass.accesses[..count].iter()
                    })
                    .filter(|previous| previous.resource_index == access.resource_index)
                    .filter(|previous| is_write(previous.access_code))
                    .map(|previous| previous.range);
                if !covers_iter(
                    writes,
                    access.range,
                    graph.resources[access.resource_index as usize].descriptor,
                )
                .map_err(|error| range_error(access.resource_index, error))?
                {
                    return Err(BuildError::Contract(Contract::UninitializedRead {
                        resource: access.resource_index,
                        pass: pass_index as u32,
                    }));
                }
            }
        }
    }
    for export in &graph.exports {
        if !initialized(export.resource_index as usize) {
            let writes = graph
                .passes
                .iter()
                .flat_map(|pass| &pass.accesses)
                .filter(|access| access.resource_index == export.resource_index)
                .filter(|access| is_write(access.access_code))
                .map(|access| access.range);
            if !covers_iter(
                writes,
                export.range,
                graph.resources[export.resource_index as usize].descriptor,
            )
            .map_err(|error| range_error(export.resource_index, error))?
            {
                return Err(BuildError::Contract(Contract::UninitializedRead {
                    resource: export.resource_index,
                    pass: graph.passes.len() as u32,
                }));
            }
        }
    }
    Ok(())
}

pub(crate) fn covers_iter(
    writes: impl Iterator<Item = AccessRange> + Clone,
    read: AccessRange,
    descriptor: super::graph::Descriptor,
) -> Result<bool, RangeError> {
    let mut any = false;
    for write in writes.clone() {
        any = true;
        if write.contains(read) {
            return Ok(true);
        }
    }
    if !any {
        return Ok(false);
    }
    match read.normalized(descriptor)? {
        AccessRange::Buffer { offset, size } => {
            let end = offset + size;
            let mut cursor = offset;
            while cursor < end {
                let mut covered_end = cursor;
                for write in writes.clone() {
                    if let AccessRange::Buffer { offset, size } = write.normalized(descriptor)?
                        && offset <= cursor
                    {
                        covered_end = covered_end.max(offset + size);
                    }
                }
                if covered_end == cursor {
                    return Ok(false);
                }
                cursor = covered_end.min(end);
            }
        }
        AccessRange::Texture {
            aspect_mask,
            base_mip_level,
            level_count,
            base_array_layer,
            layer_count,
        } => {
            // Sweep only existing rectangle boundaries. Within each mip slab,
            // initialized layers form a one-dimensional interval union. This
            // avoids temporary fragment Vecs and never enumerates image bytes
            // or every array layer. All ranges were descriptor validated.
            let mip_end = base_mip_level + level_count;
            let layer_end = base_array_layer + layer_count;
            for bit in 0..u32::BITS {
                let aspect = 1u32 << bit;
                if aspect_mask & aspect == 0 {
                    continue;
                }
                let mut mip = base_mip_level;
                while mip < mip_end {
                    let mut next_mip = mip_end;
                    for write in writes.clone() {
                        if let AccessRange::Texture {
                            aspect_mask,
                            base_mip_level,
                            level_count,
                            ..
                        } = write.normalized(descriptor)?
                            && aspect_mask & aspect != 0
                        {
                            let end = base_mip_level + level_count;
                            if base_mip_level > mip {
                                next_mip = next_mip.min(base_mip_level);
                            }
                            if end > mip {
                                next_mip = next_mip.min(end);
                            }
                        }
                    }
                    let mut layer = base_array_layer;
                    while layer < layer_end {
                        let mut covered_end = layer;
                        for write in writes.clone() {
                            if let AccessRange::Texture {
                                aspect_mask,
                                base_mip_level,
                                level_count,
                                base_array_layer,
                                layer_count,
                            } = write.normalized(descriptor)?
                                && aspect_mask & aspect != 0
                                && base_mip_level <= mip
                                && base_mip_level + level_count >= next_mip
                                && base_array_layer <= layer
                            {
                                covered_end = covered_end.max(base_array_layer + layer_count);
                            }
                        }
                        if covered_end == layer {
                            return Ok(false);
                        }
                        layer = covered_end.min(layer_end);
                    }
                    mip = next_mip;
                }
            }
        }
        AccessRange::Whole => unreachable!("whole range is normalized above"),
    }
    Ok(true)
}

pub(crate) fn initialized_resources(
    graph: &GraphDeclaration,
    is_write: impl FnMut(u8) -> bool,
) -> Result<Vec<bool>, BuildError> {
    resolve_with_initialization(graph, is_write).map(|(_, initialized)| initialized)
}

fn resolve_with_initialization(
    graph: &GraphDeclaration,
    mut is_write: impl FnMut(u8) -> bool,
) -> Result<(Vec<Vec<AccessDeclaration>>, Vec<bool>), BuildError> {
    graph.validate()?;
    let mut versions = vec![0u32; graph.resources.len()];
    let mut initialized: Vec<Vec<AccessRange>> = graph
        .resources
        .iter()
        .map(|resource| {
            let undefined = matches!(resource.source, OriginSource::Create)
                || matches!(resource.descriptor, super::graph::Descriptor::Image(_))
                    && matches!(
                        resource.source,
                        OriginSource::ImportOwned {
                            initial_access: 0,
                            ..
                        } | OriginSource::ImportUnowned {
                            initial_access: 0,
                            ..
                        }
                    );
            if undefined {
                Vec::new()
            } else {
                vec![AccessRange::Whole]
            }
        })
        .collect();
    let mut passes = Vec::with_capacity(graph.passes.len());
    for (pass_index, pass) in graph.passes.iter().enumerate() {
        let mut accesses = Vec::with_capacity(pass.accesses.len());
        for access in &pass.accesses {
            if usize::from(access.access_code) >= crate::raw::access_table::ACCESS_CODE_COUNT {
                return Err(BuildError::Contract(Contract::InvalidAccessCode(
                    access.access_code,
                )));
            }
            let resource = access.resource_index as usize;
            let descriptor = graph.resources[resource].descriptor;
            access
                .range
                .validate(descriptor)
                .map_err(|error| range_error(access.resource_index, error))?;
            let current = versions[resource];
            let write = is_write(access.access_code);
            if (access.reads_previous && !write)
                || (access.contents_unused && (write || access.reads_previous))
                || (access.allow_undefined_contents && (write || access.reads_previous || access.contents_unused))
            {
                return Err(BuildError::Contract(Contract::WrongAccessMode {
                    resource: access.resource_index,
                    access_code: access.access_code,
                }));
            }
            let resolved = match (access.version, write) {
                (AccessVersion::Automatic, false) => AccessVersion::Read(current),
                (AccessVersion::Automatic, true) => AccessVersion::Write {
                    previous: current,
                    current: current.checked_add(1).ok_or(BuildError::Contract(
                        Contract::VersionOverflow(access.resource_index),
                    ))?,
                },
                (AccessVersion::Read(version), false) => {
                    check_version(access.resource_index, current, version)?;
                    access.version
                }
                (
                    AccessVersion::Write {
                        previous,
                        current: next,
                    },
                    true,
                ) => {
                    check_version(access.resource_index, current, previous)?;
                    if previous.checked_add(1) != Some(next) {
                        return Err(BuildError::Contract(Contract::InvalidWriteVersion {
                            resource: access.resource_index,
                            previous,
                            current: next,
                        }));
                    }
                    access.version
                }
                _ => {
                    return Err(BuildError::Contract(Contract::WrongAccessMode {
                        resource: access.resource_index,
                        access_code: access.access_code,
                    }));
                }
            };
            if access.reads_contents(write)
                && !access.allow_undefined_contents
                && !covers(&initialized[resource], access.range, descriptor)
                    .map_err(|error| range_error(access.resource_index, error))?
            {
                return Err(BuildError::Contract(Contract::UninitializedRead {
                    resource: access.resource_index,
                    pass: pass_index as u32,
                }));
            }
            if let AccessVersion::Write { current, .. } = resolved {
                versions[resource] = current;
                initialized[resource].push(access.range);
            }
            accesses.push(AccessDeclaration {
                version: resolved,
                ..*access
            });
        }
        passes.push(accesses);
    }
    for export in &graph.exports {
        if usize::from(export.access_code) >= crate::raw::access_table::ACCESS_CODE_COUNT {
            return Err(BuildError::Contract(Contract::InvalidAccessCode(
                export.access_code,
            )));
        }
        let resource = export.resource_index as usize;
        if !covers(
            &initialized[resource],
            export.range,
            graph.resources[resource].descriptor,
        )
        .map_err(|error| range_error(export.resource_index, error))?
        {
            return Err(BuildError::Contract(Contract::UninitializedRead {
                resource: export.resource_index,
                pass: graph.passes.len() as u32,
            }));
        }
    }
    let full = graph
        .resources
        .iter()
        .zip(&initialized)
        .enumerate()
        .map(|(index, (resource, ranges))| {
            covers(ranges, AccessRange::Whole, resource.descriptor)
                .map_err(|error| range_error(index as u32, error))
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok((passes, full))
}

fn check_version(resource: u32, expected: u32, actual: u32) -> Result<(), BuildError> {
    if expected != actual {
        Err(BuildError::Contract(Contract::StaleVersion {
            resource,
            expected,
            actual,
        }))
    } else {
        Ok(())
    }
}

pub(crate) fn covers(
    writes: &[AccessRange],
    read: AccessRange,
    descriptor: super::graph::Descriptor,
) -> Result<bool, RangeError> {
    covers_iter(writes.iter().copied(), read, descriptor)
}

#[cfg(test)]
mod coverage_tests {
    use super::*;
    fn next(seed: &mut u32) -> u32 {
        *seed ^= *seed << 13;
        *seed ^= *seed >> 17;
        *seed ^= *seed << 5;
        *seed
    }
    #[test]
    fn interval_union_matches_independent_byte_cells_for_gaps_and_overlaps() {
        let descriptor = super::super::graph::Descriptor::Buffer(super::super::graph::BufferDesc {
            word_at_00: 8,
            word_at_08: 0,
            bytes_at_0c_through_0e: [0; 3],
        });
        let mut seed = 0x91e1_0da5;
        for _ in 0..4096 {
            let mut writes = [AccessRange::Whole; 6];
            for range in &mut writes {
                let offset = u64::from(next(&mut seed) % 8);
                *range = AccessRange::Buffer {
                    offset,
                    size: 1 + u64::from(next(&mut seed)) % (8 - offset),
                };
            }
            let offset = u64::from(next(&mut seed) % 8);
            let size = 1 + u64::from(next(&mut seed)) % (8 - offset);
            let expected = (offset..offset + size).all(|byte| {
                writes.iter().any(|range| {
                    let AccessRange::Buffer { offset, size } = *range else {
                        unreachable!()
                    };
                    offset <= byte && byte < offset + size
                })
            });
            assert_eq!(
                covers(&writes, AccessRange::Buffer { offset, size }, descriptor),
                Ok(expected)
            );
            writes.reverse();
            assert_eq!(
                covers(&writes, AccessRange::Buffer { offset, size }, descriptor),
                Ok(expected)
            );
        }
    }
    #[test]
    fn rectangle_union_matches_independent_aspect_mip_layer_cells() {
        let descriptor = super::super::vulkan::ResourceSpec::Image(
            crate::raw::texture::TextureDesc::new_2d_array(
                16,
                16,
                3,
                ash::vk::Format::D24_UNORM_S8_UINT,
            )
            .mip_levels(3),
        )
        .descriptor();
        let mut seed = 0x71a3_c9e7;
        let mut random_range = || {
            let aspect_mask = 2 * (1 + next(&mut seed) % 3);
            let base_mip_level = next(&mut seed) % 3;
            let level_count = 1 + next(&mut seed) % (3 - base_mip_level);
            let base_array_layer = next(&mut seed) % 3;
            let layer_count = 1 + next(&mut seed) % (3 - base_array_layer);
            AccessRange::Texture {
                aspect_mask,
                base_mip_level,
                level_count,
                base_array_layer,
                layer_count,
            }
        };
        for _ in 0..4096 {
            let writes = std::array::from_fn::<_, 6, _>(|_| random_range());
            let read = random_range();
            let AccessRange::Texture {
                aspect_mask,
                base_mip_level,
                level_count,
                base_array_layer,
                layer_count,
            } = read
            else {
                unreachable!()
            };
            let expected = [2, 4]
                .into_iter()
                .filter(|bit| bit & aspect_mask != 0)
                .all(|bit| {
                    (base_mip_level..base_mip_level + level_count).all(|mip| {
                        (base_array_layer..base_array_layer + layer_count).all(|layer| {
                            writes.iter().any(|range| {
                                let AccessRange::Texture {
                                    aspect_mask,
                                    base_mip_level,
                                    level_count,
                                    base_array_layer,
                                    layer_count,
                                } = *range
                                else {
                                    unreachable!()
                                };
                                aspect_mask & bit != 0
                                    && base_mip_level <= mip
                                    && mip < base_mip_level + level_count
                                    && base_array_layer <= layer
                                    && layer < base_array_layer + layer_count
                            })
                        })
                    })
                });
            assert_eq!(covers(&writes, read, descriptor), Ok(expected));
            assert_eq!(
                covers_iter(writes.into_iter().rev(), read, descriptor),
                Ok(expected)
            );
        }
    }
}
