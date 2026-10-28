//! B `systems/wall/shapes/circle/signifiers/edit_arrow.rs` (VA `0x140b35120`, 3 params).
//!
//! PDB: 1 `Query<(&EditCircleSignifier,&mut Transform)>`, 2 `Res<AnimatedCircleSignifierResource>`,
//! 3 `Res<UiSignifierStream>`.
//!
//! Gate `tag != 2` (circle 0 / rectangle 1 emit; freehand 2 does not). `tag != 2` →
//! `quat = rotation_from_normal(resource.normal)`, `pos = resource.position`; emit
//! `edit_circle_{WidgetId-hash}_arrow` through the real factory path and write
//! `Transform{rotation,translation}` to every `EditCircleSignifier` entity. Marker only —
//! `EditCircleSignifier` fields are not read.

use bevy_ecs::prelude::*;
use glam::Vec3;
use std::sync::Arc;

use crate::systems::ui::signifiers::signifier_create::UiSignifierStream;
use crate::systems::ui::signifiers::signifier_factory::{
    SignifierEntityBuilder, SignifierFactory, SignifierMesh,
};
use crate::systems::wall::shapes::circle::ui_edit_circle::AnimatedCircleSignifierResource;
use crate::utils::components::transform::Transform;

/// Native `EditCircleSignifier` (size `0x58`). Query marker; fields not read here.
#[derive(Component, Clone)]
pub struct EditCircleSignifier(pub [u8; 0x58]);

impl Default for EditCircleSignifier {
    fn default() -> Self {
        Self([0; 0x58])
    }
}

/// Native required-components descriptor (`0x140b35ba0`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ComponentRegistration {
    pub size: usize,
    pub type_id: (u64, u64),
    pub name: &'static str,
}

/// B `0x140b35ba0`: `EditCircleSignifier` size `0x58`, TypeId
/// `(0x1ae899de41d987a3, 0xc0085b7b93938bcb)`, name `...edit_arrow::EditCircleSignifier`.
pub fn register_required_components() -> ComponentRegistration {
    ComponentRegistration {
        size: 0x58,
        type_id: (0x1ae899de41d987a3, 0xc0085b7b93938bcb),
        name: "country_core::systems::wall::shapes::circle::signifiers::edit_arrow::EditCircleSignifier",
    }
}

/// B `0x140002370`: clone behavior marker (returns 0).
pub fn clone_behavior() -> u8 {
    0
}

/// B `signifier_edit_circle_arrow` (`0x140b35120`).
pub fn signifier_edit_circle_arrow(
    mut query: Query<(&EditCircleSignifier, &mut Transform)>,
    resource: Res<AnimatedCircleSignifierResource>,
    mut stream: ResMut<UiSignifierStream>,
) {
    let tag = resource.tag;
    if tag != 2 {
        let quat = ::utils::rotation_from_normal(resource.normal);
        let pos = resource.position;
        // WidgetId lives at resource +0x00; name = `edit_circle_{WidgetId:?}_arrow`
        // (native `FUN_140b35b50`). Arrow mesh = Named asset `"flat_arrow"` (builder tag 0 →
        // Named branch, resolved via `FUN_14092d570`; missing asset panics) + `vertex_color_shader`.
        // Record 0x30 `{1, 1, Quat, Vec3}` is carried by the closure (rotation/position).
        let widget_id = u64::from_le_bytes(resource.unknown_00[..8].try_into().unwrap());
        let name = format!(
            "edit_circle_{:?}_arrow",
            crate::systems::ui::resources::ui_state::WidgetId(widget_id)
        );
        let mut factory = SignifierFactory::from_closure(
            &name,
            Arc::new(move |meshes, shaders, commands| {
                let entity = SignifierEntityBuilder::new(SignifierMesh::Named(
                    "flat_arrow".to_string(),
                ))
                .rotation(quat)
                .position(pos)
                .disable_default_animation()
                .animation_duration(0.0)
                .custom_shader("vertex_color_shader")
                .build(meshes, shaders, commands);
                vec![entity]
            }),
        );
        // Native factory/record marker `0x142aed888`.
        factory.marker = 0x142aed888;
        stream.write_factory(&factory);

        for (_marker, mut transform) in query.iter_mut() {
            transform.rotation = quat;
            transform.translation = pos;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::components::transform::Transform;
    use bevy_ecs::system::RunSystemOnce;
    use glam::Quat;
    use std::mem::{offset_of, size_of};

    fn resource(tag: u8) -> AnimatedCircleSignifierResource {
        AnimatedCircleSignifierResource {
            angle_base: 0.0,
            position: Vec3::new(1.0, 2.0, 3.0),
            normal: Vec3::Y,
            tag,
            ..AnimatedCircleSignifierResource::default()
        }
    }

    fn setup(tag: u8) -> (World, Entity) {
        let mut world = World::new();
        world.insert_resource(resource(tag));
        world.init_resource::<UiSignifierStream>();
        let entity = world
            .spawn((
                EditCircleSignifier::default(),
                Transform {
                    rotation: Quat::IDENTITY,
                    translation: Vec3::ZERO,
                    scale: Vec3::ONE,
                },
            ))
            .id();
        (world, entity)
    }

    #[test]
    fn freehand_tag2_neither_emits_nor_writes_transform() {
        let (mut world, entity) = setup(2);
        world.run_system_once(signifier_edit_circle_arrow).unwrap();
        let transform = world.get::<Transform>(entity).unwrap();
        assert_eq!(transform.rotation, Quat::IDENTITY);
        assert_eq!(transform.translation, Vec3::ZERO);
        assert_eq!(world.resource::<UiSignifierStream>().committed_len(), 0);
    }

    #[test]
    fn circle_tag0_rotates_writes_transform_and_emits() {
        let (mut world, entity) = setup(0);
        world.run_system_once(signifier_edit_circle_arrow).unwrap();
        let transform = world.get::<Transform>(entity).unwrap();
        assert_eq!(transform.translation, Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(transform.rotation, ::utils::rotation_from_normal(Vec3::Y));
        assert!(world.resource::<UiSignifierStream>().committed_len() > 0);
    }

    #[test]
    fn rectangle_tag1_rotates_writes_transform_and_emits() {
        let (mut world, entity) = setup(1);
        world.run_system_once(signifier_edit_circle_arrow).unwrap();
        let transform = world.get::<Transform>(entity).unwrap();
        assert_eq!(transform.translation, Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(transform.rotation, ::utils::rotation_from_normal(Vec3::Y));
        assert!(world.resource::<UiSignifierStream>().committed_len() > 0);
    }

    #[test]
    fn resource_layout_matches_native_offsets() {
        assert_eq!(size_of::<AnimatedCircleSignifierResource>(), 64);
        assert_eq!(offset_of!(AnimatedCircleSignifierResource, angle_base), 0x18);
        assert_eq!(offset_of!(AnimatedCircleSignifierResource, position), 0x20);
        assert_eq!(offset_of!(AnimatedCircleSignifierResource, normal), 0x2c);
        assert_eq!(offset_of!(AnimatedCircleSignifierResource, tag), 0x38);
    }

    #[test]
    fn component_registration_matches_native() {
        let registration = register_required_components();
        assert_eq!(registration.size, 0x58);
        assert_eq!(
            registration.type_id,
            (0x1ae899de41d987a3, 0xc0085b7b93938bcb)
        );
        assert!(registration.name.ends_with("edit_arrow::EditCircleSignifier"));
    }
}
