//! B terrain business rules. Not an ECS/ABI replica or a terrain rasterizer.
//! Evidence: evidence/terrain-editing/{findings,second-batch-findings}.md.
pub mod edit;
pub mod replay;
pub mod schema;
pub mod stroke;

pub const ORIGINAL_B_SHA256: &str =
    "f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03";
