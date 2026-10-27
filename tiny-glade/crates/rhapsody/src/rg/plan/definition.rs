//! Graph definitions and their declaration-based construction.
use std::collections::BTreeSet;
use super::COMPILER_VERSION;
use super::resource::AccessSemantics;
use super::resource::ImportBinding;
use super::resource::PlanContext;
use super::resource::ResourceDefinition;
use super::resource::ResourceOriginKind;
use super::super::graph::AccessDependency;
use super::super::graph::GraphDeclaration;
use super::super::graph::OriginSource;
use super::super::graph::PassDeclaration;
use super::super::graph::ResourceDeclaration;
use super::super::resource::AccessVersion;
use super::super::resource_info::AccessDeclaration;
use super::super::resource_info::ExportDeclaration;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GraphDefinition {
    pub subgraphs: Vec<super::super::subgraph::SubgraphDeclaration>,
    pub resources: Vec<ResourceDefinition>,
    pub passes: Vec<PassDeclaration>,
    pub exports: Vec<ExportDeclaration>,
    pub batch_per_pass: bool,
    pub gated_resources: Vec<u32>,
    pub access_semantics: Vec<AccessSemantics>,
    pub context: PlanContext,
    pub compiler_version: u32,
}

impl GraphDefinition {
    pub(crate) fn differing_key_reason(&self, new: &Self) -> crate::collections::cache::CacheReason {
        use crate::collections::cache::CacheReason;
        if self.context.device_generation != new.context.device_generation {
            CacheReason::DeviceGeneration
        } else if self.context != new.context {
            CacheReason::QueuePolicy
        } else if self.compiler_version != new.compiler_version {
            CacheReason::CompilerVersion
        } else if self.resources.len() == new.resources.len()
            && self
                .resources
                .iter()
                .zip(&new.resources)
                .any(|(a, b)| a.descriptor != b.descriptor)
        {
            CacheReason::Descriptor
        } else if self.resources.len() == new.resources.len()
            && self
                .resources
                .iter()
                .zip(&new.resources)
                .any(|(a, b)| a.sharing != b.sharing)
        {
            CacheReason::Sharing
        } else if self.subgraphs.iter().zip(&new.subgraphs).any(|(a, b)| {
            a.interface.implementation_signature != b.interface.implementation_signature
        }) {
            CacheReason::ShaderImplementation
        } else if self.subgraphs.iter().zip(&new.subgraphs).any(|(a, b)| {
            a.interface.shader_interface_signature != b.interface.shader_interface_signature
        }) {
            CacheReason::ShaderInterface
        } else if self.subgraphs != new.subgraphs {
            CacheReason::SubgraphInterface
        } else if self.access_semantics != new.access_semantics {
            CacheReason::AccessSemantics
        } else {
            CacheReason::GraphStructure
        }
    }
    /// Compare a structurally validated declaration at the execution boundary. Only graph scope,
    /// import identity and initial access are dynamic. Explicit versions still
    /// have to equal the already validated normalized version stream, and an
    /// undefined import must be initialized again on this frame.
    pub(crate) fn matches_declaration(
        &self,
        graph: &GraphDeclaration,
        dependency: &impl AccessDependency,
        context: PlanContext,
    ) -> crate::Result<bool> {
        if self.compiler_version != COMPILER_VERSION
            || self.context != context
            || self.batch_per_pass != graph.batch_per_pass
            || self.gated_resources != graph.gated_resources
            || self.subgraphs != graph.subgraphs
            || self.exports != graph.exports
            || self.resources.len() != graph.resources.len()
            || self.passes.len() != graph.passes.len()
        {
            return Ok(false);
        }
        for (cached, frame) in self.resources.iter().zip(&graph.resources) {
            let origin = match frame.source {
                OriginSource::Create => ResourceOriginKind::Create,
                OriginSource::ImportOwned { .. } => ResourceOriginKind::ImportOwned,
                OriginSource::ImportUnowned { .. } => ResourceOriginKind::ImportUnowned,
            };
            if cached.sharing != frame.sharing
                || cached.name != frame.name
                || cached.descriptor != frame.descriptor
                || cached.origin != origin
            {
                return Ok(false);
            }
        }
        for (cached, frame) in self.passes.iter().zip(&graph.passes) {
            if cached.name != frame.name
                || cached.queue_class != frame.queue_class
                || cached.side_effect != frame.side_effect
                || cached.accesses.len() != frame.accesses.len()
            {
                return Ok(false);
            }
            for (normalized, access) in cached.accesses.iter().zip(&frame.accesses) {
                if (AccessDeclaration {
                    version: normalized.version,
                    ..*access
                }) != *normalized
                    || access.version != AccessVersion::Automatic
                        && access.version != normalized.version
                {
                    return Ok(false);
                }
            }
        }
        // The identical pass/export/port access codes imply the identical set of
        // semantic keys. Compare the entire table, including unused-in-pass ports.
        for semantics in &self.access_semantics {
            let (native, layout) = dependency.native_semantics(semantics.code);
            if semantics.declared_access != dependency.access_mask(semantics.code)
                || semantics.scope.write != dependency.is_write(semantics.code)
                || semantics.scope != native
                || semantics.layout != layout
            {
                return Ok(false);
            }
        }
        super::super::resource_contracts::validate_matching_initialization(graph, |code| {
            self.is_write(code)
        })?;
        Ok(true)
    }

    /// Import identity and boundary state are deliberately dynamic parameters.
    /// Contracts are checked on every frame, including cache hits.
    pub fn from_declaration(
        graph: &GraphDeclaration,
        dependency: &impl AccessDependency,
        context: PlanContext,
    ) -> crate::Result<Self> {
        graph.validate()?;
        Self::from_validated_declaration(graph, dependency, context)
    }

    /// The execution owner has validated this immutable declaration once,
    /// before backend preparation or cache lookup.
    pub(crate) fn from_validated_declaration(
        graph: &GraphDeclaration,
        dependency: &impl AccessDependency,
        context: PlanContext,
    ) -> crate::Result<Self> {
        if graph
            .subgraphs
            .iter()
            .flat_map(|scope| {
                scope
                    .interface
                    .inputs
                    .iter()
                    .chain(&scope.interface.outputs)
            })
            .any(|port| {
                usize::from(port.access_code) >= crate::native::access_table::ACCESS_CODE_COUNT
            })
        {
            return Err(crate::Error::invalid_usage(
                "subgraph port access code is invalid",
            ));
        }
        let resolved =
            super::super::resource_contracts::resolve_contracts(graph, |code| dependency.is_write(code))?;
        let mut codes = BTreeSet::new();
        for pass in &graph.passes {
            codes.extend(pass.accesses.iter().map(|a| a.access_code));
        }
        codes.extend(graph.exports.iter().map(|e| e.access_code));
        codes.extend(graph.subgraphs.iter().flat_map(|scope| {
            scope
                .interface
                .inputs
                .iter()
                .chain(&scope.interface.outputs)
                .map(|port| port.access_code)
        }));
        let access_semantics = codes
            .into_iter()
            .map(|code| {
                let (native, layout) = dependency.native_semantics(code);
                let write = dependency.is_write(code);
                if native.write != write {
                    return Err(crate::Error::invalid_usage(
                        "native access scope disagrees with graph write semantics",
                    ));
                }
                Ok(AccessSemantics {
                    code,
                    declared_access: dependency.access_mask(code),
                    scope: native,
                    layout,
                })
            })
            .collect::<crate::Result<Vec<_>>>()?;
        let mut passes = graph.passes.clone();
        for (pass, accesses) in passes.iter_mut().zip(resolved) {
            pass.graph_id = None;
            pass.accesses = accesses;
        }
        Ok(Self {
            subgraphs: graph.subgraphs.clone(),
            resources: graph
                .resources
                .iter()
                .map(|r| ResourceDefinition {
                    sharing: r.sharing,
                    name: r.name.clone(),
                    descriptor: r.descriptor,
                    origin: match r.source {
                        OriginSource::Create => ResourceOriginKind::Create,
                        OriginSource::ImportOwned { .. } => ResourceOriginKind::ImportOwned,
                        OriginSource::ImportUnowned { .. } => ResourceOriginKind::ImportUnowned,
                    },
                })
                .collect(),
            passes,
            exports: graph.exports.clone(),
            batch_per_pass: graph.batch_per_pass,
            gated_resources: graph.gated_resources.clone(),
            access_semantics,
            context,
            compiler_version: COMPILER_VERSION,
        })
    }
    pub fn is_write(&self, code: u8) -> bool {
        self.access_semantics
            .iter()
            .any(|s| s.code == code && s.scope.write)
    }

    /// The declaration a frame that produced this definition would have
    /// declared, rebuilt without running any declaration code. Created
    /// resources stay created; `imports` must name every imported resource once
    /// with the source kind the definition recorded (identity and boundary
    /// state are the per-frame parameters). Pass accesses are the resolved ones;
    /// instantiation overwrites them from the plan either way.
    pub fn declaration_for_frame(
        &self,
        graph_id: Option<super::super::resource::GraphId>,
        imports: &[ImportBinding],
    ) -> crate::Result<GraphDeclaration> {
        let mut sources: Vec<Option<OriginSource>> = vec![None; self.resources.len()];
        for import in imports {
            let slot = sources
                .get_mut(import.logical as usize)
                .ok_or_else(|| crate::Error::invalid_usage("retained import index is out of range"))?;
            let definition = &self.resources[import.logical as usize];
            let kind = match import.source {
                OriginSource::Create => ResourceOriginKind::Create,
                OriginSource::ImportOwned { .. } => ResourceOriginKind::ImportOwned,
                OriginSource::ImportUnowned { .. } => ResourceOriginKind::ImportUnowned,
            };
            if slot.is_some() || kind == ResourceOriginKind::Create || kind != definition.origin {
                return Err(crate::Error::invalid_usage(
                    "retained import is duplicated or differs from the planned origin",
                ));
            }
            *slot = Some(import.source);
        }
        let mut resources = Vec::with_capacity(self.resources.len());
        for (definition, source) in self.resources.iter().zip(sources) {
            let source = match (definition.origin, source) {
                (ResourceOriginKind::Create, None) => OriginSource::Create,
                (ResourceOriginKind::Create, Some(_)) => unreachable!("create origin rejected above"),
                (_, Some(source)) => source,
                (_, None) => {
                    return Err(crate::Error::invalid_usage(
                        "retained frame misses an imported resource",
                    ));
                }
            };
            resources.push(ResourceDeclaration {
                sharing: definition.sharing,
                name: definition.name.clone(),
                descriptor: definition.descriptor,
                source,
            });
        }
        let mut passes = self.passes.clone();
        for pass in &mut passes {
            pass.graph_id = graph_id;
        }
        Ok(GraphDeclaration {
            subgraphs: self.subgraphs.clone(),
            resources,
            passes,
            exports: self.exports.clone(),
            batch_per_pass: self.batch_per_pass,
            gated_resources: self.gated_resources.clone(),
        })
    }
}
