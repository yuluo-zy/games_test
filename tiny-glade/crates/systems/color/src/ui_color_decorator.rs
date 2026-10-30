//! B1422b0e00 decorator/stair color widget; all identities come from the real raycast owner.
use bevy_ecs::prelude::*;
use country_core::{resources::{mouse_terrain_raycast::{CameraCursorRay, CursorPositionSS},
    stairs::{stairs_assembly_params::{PlayermadeStairsAssemblyRequests, StairGraphEdgeIdentity}, stairs_state::{StairSegment, StairsState}},
    walls::{WallId, cached_decorator_transforms::CachedDecoratorTransforms,
        decorator::{DecoratorId, DecoratorRank}, decorator_storage::{DecoratorStorage, DecoratorStorageAddr},
        decorator_subtype::{DecoratorLibKey, DerivedDecoratorInfoResources}, public_walls::PublicWalls}},
    systems::{collision::world::RaycastWorld, mode_manager::AppMode,
        ui::{resources::{ui_state::UiState, widget_params::{UiSounds, WidgetGeometry, WidgetParams}},
            signifiers::UiSignifierStream}, wall::structure_outline::OutlineStairsCmd}};
use crate::{HideColorMenuCmd, ShowColorMenuCmd};

fn has_magic(requests: &PlayermadeStairsAssemblyRequests, a: u32, b: u32) -> bool {
    let key = StairGraphEdgeIdentity::from(StairSegment::new(DecoratorId(a), DecoratorId(b)));
    requests.previous.sub_segments.get(&key).and_then(|list| list.last()).is_some_and(|last| last.kind == 3)
}

#[allow(clippy::too_many_arguments)]
pub fn ui_color_windows_and_flags(mode: Res<AppMode>, ui: Res<UiState>, storage: Res<DecoratorStorage>,
    mut shows: EventWriter<ShowColorMenuCmd>, mut hides: EventWriter<HideColorMenuCmd>,
    cursor: Res<CursorPositionSS>, transforms: Res<CachedDecoratorTransforms>,
    derived: DerivedDecoratorInfoResources, mut signifiers: ResMut<UiSignifierStream>,
    walls: Res<PublicWalls>, world: Res<RaycastWorld>, ray: Res<CameraCursorRay>,
    mut outlines: EventWriter<OutlineStairsCmd>, requests: Res<PlayermadeStairsAssemblyRequests>,
    stairs: Res<StairsState>) {
    if mode.native_tag() != 12 { return; }
    let hit = world.legacy_raycast_w_params(ray.origin, ray.dir, 0x3108);
    if !hit.is_hit() { return; }
    let tag = hit.handle[0] as u8;
    let id = DecoratorId((hit.handle[0] >> 32) as u32);
    let (info, addr, enabled, magic, other) = match tag {
        13 | 14 => {
            let Some(info) = storage.lookup_decorator_info(id) else { return; };
            let Some(addr) = storage.lookup_storage(id) else { return; };
            let other = hit.handle[1] as u32;
            (info, addr, true, has_magic(&requests, id.0, other), other)
        }
        4 => {
            let addr = match hit.handle[1] {
                0 => DecoratorStorageAddr::Wall(WallId(hit.handle[2])),
                1 => DecoratorStorageAddr::Terrain,
                2 => DecoratorStorageAddr::Stair,
                3 => return,
                _ => panic!("invalid native decorator storage address"),
            };
            let Some(info) = storage.get(addr).and_then(|decorators| decorators.get(id)) else { return; };
            let magic = stairs.graph().neighbors(id.0).any(|other| has_magic(&requests, id.0, other));
            (info, addr, info.ty.wrapping_sub(0x38) < 6, magic, hit.handle[2] as u32)
        }
        _ => return,
    };
    let subtype = derived.as_view().subtype(id).0;
    let distance = (ray.origin - hit.position).length();
    let mut sounds = UiSounds::empty();
    sounds.slots[0][..4].copy_from_slice(&1u32.to_le_bytes());
    sounds.slots[3][..4].copy_from_slice(&(u32::from(enabled) * 4 + 4).to_le_bytes());
    let mut params = WidgetParams::default().geometry(WidgetGeometry::TerrainCanvas { distance, layer: 0 })
        .sounds(sounds).with_signifiers(vec![crate::ui_color_wall_and_roof::cursor_factory_named("my_signifier", cursor.0, enabled)]);
    params.flags.0 |= 2;
    let widget = ui.add(&format!("color_decorator_{}", id.0), params);
    if !enabled { return; }
    if widget.is_active() {
        if info.ty == b'?' {
            outlines.write(OutlineStairsCmd::connected_color_hover(StairSegment::new(id, DecoratorId(other))));
        } else {
            let transform = transforms.transform(id.0);
            let offset = info.dst_tag() < 2 && walls.get(addr.as_wall()).is_some_and(|wall| wall.shape().tag() < 2);
            derived.as_view().mesh_library().get_outline(DecoratorLibKey::new(subtype, info.rank))
                .display(&glam::Affine3A::from_scale_rotation_translation(transform.scale, transform.rotation, transform.translation),
                    &mut signifiers, id, DecoratorRank(info.rank), offset, true);
        }
    }
    if widget.clicked() {
        hides.write(HideColorMenuCmd { cancel: false });
        let (kind, subtype) = match info.ty {
            b'8' => (3, 0), b'9' | b':' => (3, 1), b';' => (10, info.ty),
            b'<' => (9, info.ty), b'=' => (6, info.ty), b'?' => (11 + u8::from(magic), info.ty),
            _ => panic!("invalid original color decorator type"),
        };
        shows.write(ShowColorMenuCmd { kind, subtype, decorator_id: id.0, payload: 0, position: cursor.0 });
    }
}
