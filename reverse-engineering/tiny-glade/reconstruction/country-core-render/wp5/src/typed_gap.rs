//! Typed-gap marker: a field whose layout/semantics were not recovered from the
//! original machine code. Never fabricated.

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash)]
pub struct Gap;
