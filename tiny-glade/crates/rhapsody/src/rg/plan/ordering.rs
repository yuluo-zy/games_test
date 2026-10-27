//! Ordering edges between passes derived from resource frontiers.
use std::collections::BTreeSet;
use super::analysis::SemanticTable;
use super::definition::GraphDefinition;
use super::ranges::range_covers;
use super::ranges::ranges_overlap;
use super::super::graph::Descriptor;
use super::super::resource::AccessRange;

/// One live access in the per-resource ordering frontier.
#[derive(Clone, Copy)]
pub(super) struct FrontierEntry {
    pass: usize,
    range: AccessRange,
    native: Option<crate::tracking::OwnershipRange>,
    layout: ash::vk::ImageLayout,
    write: bool,
}

/// Storage-order constraints between retained passes: write/any overlaps and
/// overlapping image accesses that need different native layouts. Only the
/// frontier is kept: an entry fully covered by a later write is ordered before
/// that write, and every later access that would conflict with the entry also
/// conflicts with the write, so the edge is implied transitively.
pub(super) fn ordering_edges(
    definition: &GraphDefinition,
    retained: &[usize],
) -> crate::Result<Vec<(usize, usize)>> {
    let table = SemanticTable::new(&definition.access_semantics);
    let mut frontier = std::collections::HashMap::<u32, Vec<FrontierEntry>>::new();
    let mut edges = BTreeSet::new();
    for &pass in retained {
        for access in &definition.passes[pass].accesses {
            let semantic = table
                .get(access.access_code)
                .ok_or_else(|| crate::Error::invalid_usage("access semantics are absent"))?;
            let descriptor = definition.resources[access.resource_index as usize].descriptor;
            let native = match descriptor {
                Descriptor::Image(_) => Some(super::super::sync_plan::native_range(
                    access.range,
                    descriptor,
                    definition.context.separate_depth_stencil_layouts,
                )?),
                Descriptor::Buffer(_) => None,
            };
            let entries = frontier.entry(access.resource_index).or_default();
            for entry in entries.iter() {
                if entry.pass == pass {
                    continue;
                }
                let content = (semantic.scope.write || entry.write)
                    && ranges_overlap(access.range, entry.range);
                let layout = matches!((native, entry.native), (Some(a), Some(b))
                    if entry.layout != semantic.layout
                        && crate::tracking::range_state::intersection(a, b).is_some());
                if content || layout {
                    edges.insert((entry.pass, pass));
                }
            }
            if semantic.scope.write {
                // A widened native range (merged depth/stencil) may reach
                // entries this declaration does not cover; keep those.
                let exact = match (native, access.range) {
                    (
                        Some(crate::tracking::OwnershipRange::Image { aspects, .. }),
                        AccessRange::Texture { aspect_mask, .. },
                    ) => aspects == aspect_mask,
                    _ => true,
                };
                if exact {
                    entries.retain(|entry| !range_covers(access.range, entry.range));
                }
            }
            entries.push(FrontierEntry {
                pass,
                range: access.range,
                native,
                layout: semantic.layout,
                write: semantic.scope.write,
            });
        }
    }
    Ok(edges.into_iter().collect())
}
