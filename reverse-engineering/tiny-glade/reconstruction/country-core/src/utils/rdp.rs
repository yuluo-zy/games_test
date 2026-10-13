//! utils/rdp.rs — `utils::rdp::Rdp` (wp12 wall-domain facade; Ramer-Douglas-Peucker
//! polyline simplifier shell).

use crate::typed_gap::Gap;

#[derive(Clone, Debug)]
pub struct Rdp {
    pub raw: Gap,
}
