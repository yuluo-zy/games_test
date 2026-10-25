//! CPU identity of a live physical resource owner, shared by frame imports
//! and managed projections. A retained Arc pins each indexed address.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) enum PhysicalIdentity {
    Buffer(usize),
    Image(usize),
}
