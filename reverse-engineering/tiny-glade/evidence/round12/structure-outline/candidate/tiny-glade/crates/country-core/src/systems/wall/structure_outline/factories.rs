//! Common owned equivalent of original outline factory closures12..19.
//! All inspected factories select Points, duration0, dashed_line, Alt(strength)
//! and DashedLineLength(length*3). Geometry and name are supplied by the producer.
use std::sync::Arc;
use glam::Vec3;
use crate::components::DashedLineLength;
use crate::render::pass_builder::draw_list::DrawTopology;
use crate::systems::ui::signifiers::signifier_factory::{SignifierEntityBuilder, SignifierFactory};

pub(super) fn outline_factory(name: String, points: Vec<Vec3>, strength: f32, length: f32) -> SignifierFactory {
    SignifierFactory::from_closure(&name, Arc::new(move |meshes, shaders, commands| {
        let entity = SignifierEntityBuilder::from_points(points.clone())
            .animation_duration(0.0)
            .custom_shader("dashed_line")
            .build(meshes, shaders, commands);
        commands.entity(entity).insert(DrawTopology::Alt(strength));
        commands.entity(entity).insert(DashedLineLength(length * 3.0));
        vec![entity]
    }))
}
