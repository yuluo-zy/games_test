//! Per-pass producer analysis and semantic lookup tables.
use std::collections::BTreeSet;
use super::definition::GraphDefinition;
use super::ranges::range_covers;
use super::ranges::ranges_overlap;
use super::resource::AccessSemantics;
use super::super::graph::PassDeclaration;
use super::super::resource::AccessRange;
use super::super::resource_info::AccessDeclaration;
use super::super::resource_info::entry_content_resources;

/// Direct lookup from an access code to its semantics (codes are `u8`).
pub(crate) struct SemanticTable<'a> {
    index: [u8; 256],
    semantics: &'a [AccessSemantics],
}

impl<'a> SemanticTable<'a> {
    pub(crate) fn new(semantics: &'a [AccessSemantics]) -> Self {
        let mut index = [u8::MAX; 256];
        // The first entry wins, matching a linear `find`; definitions carry at
        // most one entry per code (one per distinct u8, so < 256 entries).
        for (position, semantic) in semantics.iter().enumerate().rev() {
            if let Ok(position) = u8::try_from(position) {
                index[semantic.code as usize] = position;
            }
        }
        Self { index, semantics }
    }
    pub(crate) fn get(&self, code: u8) -> Option<&'a AccessSemantics> {
        match self.index[code as usize] {
            u8::MAX => self.semantics.iter().find(|semantic| semantic.code == code),
            position => Some(&self.semantics[position as usize]),
        }
    }
    pub(crate) fn is_write(&self, code: u8) -> bool {
        self.get(code).is_some_and(|semantic| semantic.scope.write)
    }
}

/// Dependency analysis contains only local symbolic indices and ranges:
/// content producers (for culling) and the current producer frontier.
#[derive(Clone, Debug)]
pub(crate) struct PassAnalysis {
    pub dependencies: Vec<BTreeSet<usize>>,
    pub producers: std::collections::BTreeMap<u32, Vec<(usize, AccessRange)>>,
}

impl PassAnalysis {
    pub(crate) fn estimated_bytes(&self) -> usize {
        std::mem::size_of::<Self>()
            .saturating_add(
                self.dependencies
                    .iter()
                    .map(|set| std::mem::size_of_val(set) + set.len() * 48)
                    .sum::<usize>(),
            )
            .saturating_add(
                self.producers
                    .values()
                    .map(|entries| 64 + entries.len() * std::mem::size_of::<(usize, AccessRange)>())
                    .sum::<usize>(),
            )
    }
}

/// Record real input sources independently from the broader native scope.
pub(super) fn add_producer_sources(
    analysis: &mut PassAnalysis,
    pass_index: usize,
    access: &AccessDeclaration,
    write: bool,
) {
    if access.reads_contents(write)
        && let Some(producers) = analysis.producers.get(&access.resource_index)
    {
        for &(producer, range) in producers {
            if producer != pass_index && ranges_overlap(access.range, range) {
                analysis.dependencies[pass_index].insert(producer);
            }
        }
    }
}

pub(super) fn publish_producer(analysis: &mut PassAnalysis, pass_index: usize, access: &AccessDeclaration) {
    let producers = analysis.producers.entry(access.resource_index).or_default();
    producers.retain(|&(_, range)| !range_covers(access.range, range));
    producers.push((pass_index, access.range));
}

pub(super) fn add_pass_producers(
    analysis: &mut PassAnalysis,
    pass_index: usize,
    pass: &PassDeclaration,
    table: &SemanticTable<'_>,
) {
    let entry_resources = entry_content_resources(&pass.accesses);
    for access in &pass.accesses {
        let write = table.is_write(access.access_code);
        add_producer_sources(analysis, pass_index, access, write);
        if write && !entry_resources.contains(&access.resource_index) {
            publish_producer(analysis, pass_index, access);
        }
    }
    // Native upgrade records execute simultaneously. Every alias of an
    // affected resource must consume the frontier from before this callback.
    if !entry_resources.is_empty() {
        for access in &pass.accesses {
            if table.is_write(access.access_code)
                && entry_resources.contains(&access.resource_index)
            {
                publish_producer(analysis, pass_index, access);
            }
        }
    }
}

pub(crate) fn analyze_passes(
    passes: &[PassDeclaration],
    semantics: &[AccessSemantics],
) -> PassAnalysis {
    let table = SemanticTable::new(semantics);
    let mut analysis = PassAnalysis {
        dependencies: vec![BTreeSet::new(); passes.len()],
        producers: Default::default(),
    };
    for (pass_index, pass) in passes.iter().enumerate() {
        add_pass_producers(&mut analysis, pass_index, pass, &table);
    }
    analysis
}

pub(super) fn compose_analysis(
    definition: &GraphDefinition,
    table: &SemanticTable<'_>,
    range: std::ops::Range<usize>,
    local: &PassAnalysis,
    resources: Option<&[u32]>,
    global: &mut PassAnalysis,
) {
    for (pass, dependencies) in local.dependencies.iter().enumerate() {
        global.dependencies[range.start + pass]
            .extend(dependencies.iter().map(|&source| range.start + source));
    }
    // Producers contain earlier scopes only. The cached local analysis
    // supplies all inner-scope relations without replaying them.
    for pass_index in range.clone() {
        let accesses = &definition.passes[pass_index].accesses;
        let entry_resources = entry_content_resources(accesses);
        for access in accesses {
            let write = table.is_write(access.access_code);
            let producers = global.producers.entry(access.resource_index).or_default();
            if access.reads_contents(write) {
                for &(source, written) in producers.iter() {
                    if ranges_overlap(access.range, written) {
                        global.dependencies[pass_index].insert(source);
                    }
                }
            }
            if write && !entry_resources.contains(&access.resource_index) {
                producers.retain(|&(_, written)| !range_covers(access.range, written));
            }
        }
        if !entry_resources.is_empty() {
            for access in accesses {
                if table.is_write(access.access_code)
                    && entry_resources.contains(&access.resource_index)
                {
                    global
                        .producers
                        .entry(access.resource_index)
                        .or_default()
                        .retain(|&(_, written)| !range_covers(access.range, written));
                }
            }
        }
    }
    for (&resource, producers) in &local.producers {
        let global_resource = resources.map_or(resource, |mapping| mapping[resource as usize]);
        global.producers.entry(global_resource).or_default().extend(
            producers
                .iter()
                .map(|&(source, range_written)| (range.start + source, range_written)),
        );
    }
}
