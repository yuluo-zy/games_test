//! Typed gap carrier for un-recovered original types (project rule: unknown
//! fields/signatures are typed gaps — never fabricated).

/// Opaque holder for a type whose layout/ABI has not been recovered.
/// Same convention as the integrated crate (`crate::typed_gap::Gap`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Gap;
