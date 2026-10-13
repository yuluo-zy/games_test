//! Typed-gap marker (shared pattern with the integrating country-core crate).

/// A field whose layout/semantics were not recovered from the original machine
/// code. Never fabricated into a real type.
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash)]
pub struct Gap;
