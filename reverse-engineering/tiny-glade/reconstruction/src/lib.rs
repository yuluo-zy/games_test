//! 当前输入构建的原作算法还原；每个模块必须链接到原始机器码验证。
pub const SOURCE_SHA256: &str = "f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03";

#[path = "../building/roof_associations.rs"]
pub mod building_roof_associations;
#[path = "../building/roof_components.rs"]
pub mod building_roof_components;
#[path = "../building/roof_creation.rs"]
pub mod building_roof_creation;
#[path = "../building/roof_deletion.rs"]
pub mod building_roof_deletion;
#[path = "../building/roof_lifecycle.rs"]
pub mod building_roof_lifecycle;
#[path = "../building/roof_spatial.rs"]
pub mod building_roof_spatial;
#[path = "../building/wall_cache.rs"]
pub mod building_wall_cache;
#[path = "../building/wall_collection.rs"]
pub mod building_wall_collection;
#[path = "../building/wall_gable.rs"]
pub mod building_wall_gable;
#[path = "../building/wall_publication.rs"]
pub mod building_wall_publication;
#[path = "../building/wall_spatial_index.rs"]
pub mod building_wall_spatial_index;
#[path = "../building/wall_state.rs"]
pub mod building_wall_state;
#[path = "../roof/rectangular_caps.rs"]
pub mod rectangular_caps;
#[path = "../roof/rectangular_edges.rs"]
pub mod rectangular_edges;
#[path = "../roof/rectangular_faces.rs"]
pub mod rectangular_faces;
#[path = "../roof/ridge.rs"]
pub mod ridge;
#[path = "../roof/edit_rules.rs"]
pub mod roof_edit_rules;
#[path = "../roof/parameters.rs"]
pub mod roof_parameters;
#[path = "../roof/pipeline.rs"]
pub mod roof_pipeline;
#[path = "../roof/replay.rs"]
pub mod roof_replay;
#[path = "../roof/roof_state.rs"]
pub mod roof_state;
#[path = "../roof/surface.rs"]
pub mod surface;
#[path = "../roof/tiles.rs"]
pub mod tiles;
#[path = "../wall/height_rules.rs"]
pub mod wall_height_rules;
#[path = "../math/window_aspect.rs"]
pub mod window_aspect;

#[path = "../building/roof_direction.rs"]
pub mod building_roof_direction;

#[path = "../building/roof_customization.rs"]
pub mod building_roof_customization;

#[path = "../building/wall_holes.rs"]
pub mod building_wall_holes;

#[path = "../building/wall_hole_invalidation.rs"]
pub mod building_wall_hole_invalidation;

#[path = "../history/transaction.rs"]
pub mod history_transactions;

#[path = "../building/opening_rules.rs"]
pub mod building_opening_rules;

#[path = "../building/decorator_placement.rs"]
pub mod building_decorator_placement;

#[path = "../history/building_transactions.rs"]
pub mod building_history_transactions;

#[path = "../building/wall_height_authority.rs"]
pub mod building_wall_height_authority;

#[path = "../building/wall_constructor.rs"]
pub mod building_wall_constructor;

#[path = "../building/decorator_subtype.rs"]
pub mod building_decorator_subtype;

#[path = "../building/wall_height_command.rs"]
pub mod building_wall_height_command;

#[path = "../building/wall_production.rs"]
pub mod building_wall_production;

#[path = "../building/wall_construction_sources.rs"]
pub mod building_wall_construction_sources;

#[path = "../building/wall_path_preparation.rs"]
pub mod building_wall_path_preparation;

#[path = "../building/wall_freeform_cache.rs"]
pub mod building_wall_freeform_cache;

#[path = "../building/roof_invalid_notifications.rs"]
pub mod building_roof_invalid_notifications;
#[path = "../building/wall_arches.rs"]
pub mod building_wall_arches;
#[path = "../building/wall_arches_freehand.rs"]
pub mod building_wall_arches_freehand;
#[path = "../building/wall_authoring.rs"]
pub mod building_wall_authoring;
#[path = "../building/wall_bulldoze.rs"]
pub mod building_wall_bulldoze;
#[path = "../building/wall_history_edit.rs"]
pub mod building_wall_history_edit;
#[path = "../building/wall_history_replay.rs"]
pub mod building_wall_history_replay;
#[path = "../building/wall_history_transition.rs"]
pub mod building_wall_history_transition;
#[path = "../building/wall_platform_segmentation.rs"]
pub mod building_wall_platform_segmentation;
#[path = "../building/wall_platform_stair_rows.rs"]
pub mod building_wall_platform_stair_rows;
#[path = "../building/wall_platform_stairs.rs"]
pub mod building_wall_platform_stairs;
#[path = "../building/wall_segmentation.rs"]
pub mod building_wall_segmentation;
#[path = "../building/wall_structures.rs"]
pub mod building_wall_structures;

#[path = "../building/wall_platform_collision.rs"]
pub mod building_wall_platform_collision;
#[path = "../building/wall_rectangle_edit.rs"]
pub mod building_wall_rectangle_edit;
#[path = "../building/wall_pending_changed.rs"]
pub mod building_wall_pending_changed;

#[path = "../building/gates.rs"]
pub mod building_gates;
#[path = "../building/wall_platform_rapier_backend.rs"]
pub mod building_wall_platform_rapier_backend;
#[path = "../building/wall_decorator_archivist.rs"]
pub mod building_wall_decorator_archivist;

#[path = "../building/nani_solid_vertex_color.rs"]
pub mod building_nani_solid_vertex_color;

#[path = "../building/dirty_tiles.rs"]
pub mod building_dirty_tiles;

#[path = "../building/decorator_info.rs"]
pub mod building_decorator_info;

#[path = "../building/fences.rs"]
pub mod building_fences;
