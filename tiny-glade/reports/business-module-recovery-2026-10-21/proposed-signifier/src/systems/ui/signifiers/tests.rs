//! CPU ECS regression cases for original lifecycle; no real OS/GPU claim.
use super::*;
use crate::{
    asset_libraries::{mesh_library::AssetMeshLibrary, shader_library::AssetShaderLibrary},
    resources::time::GameTime,
    systems::ui::resources::{
        ui_state::{UiInteraction, UiState, WidgetId},
        widget_params::{WidgetGeometry, WidgetParams},
    },
    utils::components::transform::Transform,
};
use glam::Vec3;
use std::sync::Arc;

fn world() -> World {
    let mut w = World::new();
    register_signifier_resources(&mut w);
    w.init_resource::<AssetMeshLibrary>();
    w.insert_resource(AssetShaderLibrary::new());
    w.init_resource::<GameTime>();
    w.resource_mut::<GameTime>().delta = 0.25;
    w.insert_resource(UiState::at_startup());
    w
}
fn payload(widget: WidgetId, element: u64) -> [u8;56] {
    let mut p=[0;56];
    p[16..24].copy_from_slice(&widget.0.to_le_bytes());
    p[24..32].copy_from_slice(&element.to_le_bytes());
    p
}
fn activate(w: &mut World, widget: WidgetId, geometry: WidgetGeometry,
            signifiers: Vec<SignifierFactory>, element: u64) {
    let params = WidgetParams::default().geometry(geometry).with_signifiers(signifiers);
    w.resource_mut::<UiState>().replace_interaction_full(
        UiInteraction { native_tag: 1, native_phase: 1, widget },
        Some(Arc::new(params)), payload(widget,element),
    );
}
fn factory(name: &str, animated: bool) -> SignifierFactory {
    SignifierFactory::from_closure(name, Arc::new(move |_,_,commands| {
        let mut e = commands.spawn_empty();
        if animated {
            e.insert((SignifierAnimation { value: 1.0, duration: 0.25,
                ..Default::default() }, Transform::identity()));
        }
        vec![e.id()]
    }))
}
fn ids(w: &mut World) -> Vec<(Entity,SignifierId)> {
    w.query::<(Entity,&SignifierId)>().iter(w).map(|(e,id)|(e,*id)).collect()
}
fn creation() -> Schedule {
    let mut s=Schedule::default(); s.add_systems(signifier_create::signifier_create); s
}

#[test]
fn id_keeps_original_hash_and_widget_halves() {
    assert_eq!(std::mem::size_of::<SignifierId>(),16);
    let id=SignifierId(7,WidgetId(19));
    let base=&id as *const _ as usize;
    assert_eq!(&id.1 as *const _ as usize-base,8);
}

#[test]
fn inactive_keeps_requests_then_active_captures_the_full_factory_list() {
    let mut w=world(); let mut s=creation(); let f=factory("pending",false);
    w.resource_mut::<UiSignifierStream>().write_factory(&f);
    s.run(&mut w);
    assert_eq!(w.resource::<UiSignifierStream>().committed_len(),1);
    assert!(ids(&mut w).is_empty());
    activate(&mut w,WidgetId(19),WidgetGeometry::Empty,vec![],0);
    s.run(&mut w);
    assert_eq!(w.resource::<UiSignifierStream>().committed_len(),0);
    let first=ids(&mut w); assert_eq!(first.len(),1);
    assert_eq!(first[0].1.1,WidgetId(19));
    assert_eq!(w.resource::<UiState>().active_params_ref().unwrap().signifiers.len(),1);
    // Empty stream does not replace/drop the active owner's captured factories.
    s.run(&mut w); assert_eq!(ids(&mut w),first);
}

#[test]
fn same_batch_equal_names_both_wake_but_existing_entities_prevent_later_creation() {
    let mut w=world(); let mut s=creation(); let f=factory("duplicate",false);
    activate(&mut w,WidgetId(4),WidgetGeometry::Empty,vec![],0);
    w.resource_mut::<UiSignifierStream>().write_factory(&f);
    w.resource_mut::<UiSignifierStream>().write_factory(&f);
    s.run(&mut w);
    let first=ids(&mut w); assert_eq!(first.len(),2);
    assert_eq!(first[0].1,first[1].1);
    s.run(&mut w); assert_eq!(ids(&mut w).len(),2);
}

#[test]
#[should_panic(expected="uniquely owned active WidgetParams")]
fn appending_rejects_shared_active_arc_instead_of_copying_geometry() {
    let mut w=world(); let mut s=creation();
    activate(&mut w,WidgetId(4),WidgetGeometry::Empty,vec![],0);
    let _held=w.resource::<UiState>().active_params().unwrap();
    w.resource_mut::<UiSignifierStream>().write_factory(&factory("one",false));
    s.run(&mut w);
}

#[test]
fn no_append_does_not_require_active_arc_uniqueness() {
    let mut w=world(); let mut s=creation();
    activate(&mut w,WidgetId(4),WidgetGeometry::Empty,vec![factory("one",false)],0);
    let _held=w.resource::<UiState>().active_params().unwrap();
    s.run(&mut w); assert_eq!(ids(&mut w).len(),1);
}

#[test]
fn mark_uses_captured_list_and_waits_for_deferred_reverse_animation() {
    let mut w=world(); let mut s=Schedule::default(); add_signifier_systems(&mut s);
    activate(&mut w,WidgetId(4),WidgetGeometry::Empty,vec![factory("one",true)],0);
    s.run(&mut w);
    let e=ids(&mut w)[0].0;
    // create Commands are not visible to animation until the next schedule run.
    assert_eq!(w.get::<SignifierAnimation>(e).unwrap().t,0.0);
    s.run(&mut w); assert_eq!(w.get::<SignifierAnimation>(e).unwrap().t,1.0);
    // A genuine ui_update refresh may install an empty factory list.
    activate(&mut w,WidgetId(4),WidgetGeometry::Empty,vec![],0);
    s.run(&mut w);
    assert!(w.get::<SignifierMarkedForDeletion>(e).is_some());
    assert_eq!(w.get::<SignifierAnimation>(e).unwrap().t,1.0);
    s.run(&mut w);
    assert!(w.get_entity(e).is_err());
    s.run(&mut w); assert!(ids(&mut w).is_empty());
}

#[test]
fn lifetime_unordered_marks_and_despawn_requires_exact_zero() {
    let mut w=world();
    let marked=w.spawn((SignifierId(9,WidgetId(5)),SignifierMinimumLifespan {
        t:0.0,min_lifespan:f32::NAN,
    })).id();
    let mut mark=Schedule::default(); mark.add_systems(signifier_mark_for_delete::signfier_mark_for_delete);
    mark.run(&mut w); assert!(w.get::<SignifierMarkedForDeletion>(marked).is_some());
    let mut entities=vec![];
    for t in [0.0,-0.0,0.2,-0.2,f32::INFINITY,f32::NEG_INFINITY,f32::NAN] {
        entities.push((w.spawn((SignifierMarkedForDeletion,SignifierAnimation {t,..Default::default()})).id(),t));
    }
    let mut despawn=Schedule::default();despawn.add_systems(signifier_mark_for_delete::despawn_marked);
    despawn.run(&mut w);
    assert!(w.get_entity(marked).is_err());
    for (e,t) in entities { assert_eq!(w.get_entity(e).is_err(),t==0.0); }
}

#[test]
fn translation_uses_widget_half_and_is_independent_of_scale_flag() {
    let mut w=world(); let mut s=Schedule::default(); s.add_systems(animation::signifier_animation);
    let good=w.spawn((SignifierId(17,WidgetId(4)),SignifierParentToVolumeOrigin,
        SignifierAnimation {flag_8:1,..Default::default()},Transform::identity())).id();
    let other=w.spawn((SignifierId(17,WidgetId(5)),SignifierParentToVolumeOrigin,
        SignifierAnimation {flag_8:1,..Default::default()},Transform::identity())).id();
    let shapes=[
        (WidgetGeometry::Sphere {center:Vec3::X,radius:3.0},0,Vec3::X),
        (WidgetGeometry::SphereCloud {centers:vec![Vec3::X,Vec3::Y],radius:2.0},1,Vec3::Y),
        (WidgetGeometry::PlaneDisc {point:Vec3::Z,normal:Vec3::Y,radius:2.0},0,Vec3::Z),
    ];
    for (shape,element,expected) in shapes {
        activate(&mut w,WidgetId(4),shape,vec![],element); s.run(&mut w);
        assert_eq!(w.get::<Transform>(good).unwrap().translation,expected);
        assert_eq!(w.get::<Transform>(good).unwrap().scale,Vec3::ONE);
        assert_eq!(w.get::<Transform>(other).unwrap().translation,Vec3::ZERO);
    }
    // Other real volume variants preserve the prior translation.
    activate(&mut w,WidgetId(4),WidgetGeometry::Empty,vec![],0);s.run(&mut w);
    assert_eq!(w.get::<Transform>(good).unwrap().translation,Vec3::Z);
}

#[test]
#[should_panic]
fn sphere_cloud_origin_preserves_original_bounds_failure() {
    let mut w=world(); let mut s=Schedule::default();s.add_systems(animation::signifier_animation);
    w.spawn((SignifierId(17,WidgetId(4)),SignifierParentToVolumeOrigin,
        SignifierAnimation::default(),Transform::identity()));
    activate(&mut w,WidgetId(4),WidgetGeometry::SphereCloud {centers:vec![Vec3::X],radius:2.0},vec![],1);
    s.run(&mut w);
}
