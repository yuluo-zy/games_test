//! 当前输入构建的原作算法还原；每个模块必须链接到原始机器码验证。
pub const SOURCE_SHA256: &str = "f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03";

#[path = "../math/window_aspect.rs"]
pub mod window_aspect;
#[path = "../roof/roof_state.rs"]
pub mod roof_state;
#[path = "../roof/surface.rs"]
pub mod surface;
#[path = "../roof/ridge.rs"]
pub mod ridge;
#[path = "../roof/tiles.rs"]
pub mod tiles;
#[path = "../roof/rectangular_edges.rs"]
pub mod rectangular_edges;
#[path = "../roof/rectangular_caps.rs"]
pub mod rectangular_caps;
#[path = "../roof/rectangular_faces.rs"]
pub mod rectangular_faces;
#[path = "../roof/pipeline.rs"]
pub mod roof_pipeline;
#[path = "../wall/height_rules.rs"]
pub mod wall_height_rules;
#[path = "../roof/edit_rules.rs"]
pub mod roof_edit_rules;
