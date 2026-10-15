//! B border queries 0x140ad2950 / 0x140ad2970. Circle borders use a strict
//! distance comparison; rectangle borders sample a circle at step 1.0.
use bevy_ecs::prelude::*;
use glam::Vec2;
use utils::geometry::{Circle2d, Rectangle2d};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BorderShape {
    Circle(Circle2d),
    Rectangle(Rectangle2d),
}

#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct GladeBorder(pub BorderShape);

#[derive(Event, Clone, Copy, Debug)]
pub struct DisplayOutOfBorder;
#[derive(Event, Clone, Copy, Debug)]
pub struct FeedbackInputOutOfBorder;

impl GladeBorder {
    pub fn is_pos_inside(&self, point: Vec2) -> bool {
        match self.0 {
            BorderShape::Circle(circle) => circle.is_point_inside(point),
            BorderShape::Rectangle(rectangle) => rectangle.contains_point(point),
        }
    }

    pub fn is_shape_inside(&self, shape: BorderShape) -> bool {
        match (self.0, shape) {
            (BorderShape::Circle(border), BorderShape::Circle(subject)) => {
                let delta = border.center - subject.center;
                (delta.y * delta.y + delta.x * delta.x).sqrt() + subject.radius < border.radius
            }
            (BorderShape::Circle(border), BorderShape::Rectangle(subject)) => subject
                .as_points2()
                .into_iter()
                .all(|point| border.is_point_inside(point)),
            (BorderShape::Rectangle(border), BorderShape::Circle(subject)) => subject
                .as_points2(1.0)
                .all(|point| border.contains_point(point)),
            (BorderShape::Rectangle(border), BorderShape::Rectangle(subject)) => subject
                .as_points2()
                .into_iter()
                .all(|point| border.contains_point(point)),
        }
    }
}
