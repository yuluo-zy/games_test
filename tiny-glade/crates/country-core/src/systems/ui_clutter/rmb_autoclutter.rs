//! B原140976ca0：ui_convert验证真实Entity后直接调用的RMB业务叶子。
//! 不单独注册成System，不经过人工镜像表或第二个点击事件队列。
use super::{
    DeletedAutoClutter,
    ui_autoclutter::{InstaCreateDeco, get_clutter_rot_in_roof_space, original_heading},
};
use crate::{
    components::drawable::{
        AutoClutter, AutoClutterConvertableToDecoClutter, AutoClutterDestination,
    },
    resources::{
        history::{EditType, History, auto_clutter_edit::AutoClutterEdit},
        roofs::Roof,
        walls::public_walls::PublicWalls,
    },
    systems::mode_manager::AppMode,
    utils::components::transform::Transform,
};
use bevy_ecs::prelude::*;

// 原事件归ui_autoclutter；旧调用路径仅转发同一TypeId。
pub use super::ui_autoclutter::EvRightClickedAutoClutter;

pub(super) fn handle_rmb_autoclutter<S: Send + Sync + 'static>(
    mode: &AppMode,
    clutter: &AutoClutter,
    source: &AutoClutterConvertableToDecoClutter,
    transform: &Transform,
    walls: &PublicWalls,
    roofs: &Query<&Roof>,
    history: &mut History<S>,
    deleted: &mut DeletedAutoClutter,
    creates: &mut EventWriter<InstaCreateDeco>,
) {
    let rotation = match source.destination {
        AutoClutterDestination::OnBuilding { wall, .. } => {
            // 原PublicWalls屋顶实体lookup和真实Roof组件均为required。
            let entity = walls
                .roof_entity(wall)
                .expect("original clutter roof association");
            let roof = roofs.get(entity).expect("original clutter roof component");
            get_clutter_rot_in_roof_space(transform.rotation, roof)
        }
        AutoClutterDestination::Terrain => original_heading(transform.rotation),
    };
    deleted.0.insert(clutter.key);
    creates.write(InstaCreateDeco {
        destination: source.destination.to_creation_words(),
        extra: None,
        rotation,
        position: transform.translation,
        ty: super::ui_autoclutter::clutter_mesh_type_from_name(source.mesh_name),
    });
    history
        .start_input_recording(mode.native_tag())
        .expect("original non-Replay recording invariant");
    history
        .add(EditType::AutoClutter(std::sync::Arc::new(
            AutoClutterEdit::Delete { id: clutter.key },
        )))
        .expect("original open history group");
}
