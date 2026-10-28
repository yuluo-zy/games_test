//! Pure graph planning. A plan owns declarations and symbolic indices only;
//! physical objects, import states, callbacks and completion points belong to a frame.
mod resource;
mod definition;
mod compiled;
mod ranges;
mod analysis;
mod ordering;
mod compile;
pub use resource::{ResourceOriginKind, ResourceDefinition, AccessSemantics, PlanContext, ImportBinding};
pub use definition::GraphDefinition;
pub use compiled::CompiledPlan;
pub use ranges::ranges_overlap;
pub(crate) use analysis::{PassAnalysis, analyze_passes};
pub use compile::{compile_definition, compile_with_subgraphs};

pub const COMPILER_VERSION: u32 = 5;

#[cfg(test)]
mod upgraded_content_tests;

#[cfg(test)]
mod borrowed_definition_tests;

#[cfg(test)]
mod ordering_tests;
