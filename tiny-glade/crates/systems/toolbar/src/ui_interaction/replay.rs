//! Replay blocks ordinary toolbar inputs and displays the original cross.
//! B render_taffy_ui_to_widgets::closure0 0x142339f10, 604 bytes.
use country_core::{
    components::{Ui2DFollowMouse, Ui2dExtent, Ui2dImage},
    systems::ui::signifiers::{
        SignifierEntityBuilder, SignifierFactory, signifier_factory::SignifierMesh,
    },
};
use glam::Vec2;
use std::sync::Arc;

pub(super) fn denied_input_signifier(cursor: Vec2, size: f32) -> SignifierFactory {
    // Original name@14304dd58/18. The Arc owns captured cursor/size until the
    // existing signifier consumer invokes this real factory and reclaims it.
    SignifierFactory::from_closure(
        "deny_input_toolbar",
        Arc::new(move |meshes, shaders, commands| {
            let entity = SignifierEntityBuilder::new(SignifierMesh::Empty)
                .animation_duration(0.0)
                .build(meshes, shaders, commands);
            commands.entity(entity).insert((
                Ui2dExtent {
                    pos: cursor,
                    size: Vec2::splat(size),
                },
                Ui2dImage::new("icon_cross"),
            ));
            commands.entity(entity).insert(Ui2DFollowMouse);
            vec![entity]
        }),
    )
}
