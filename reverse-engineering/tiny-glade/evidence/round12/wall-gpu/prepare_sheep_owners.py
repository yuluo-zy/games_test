from pathlib import Path
r=Path('D:/game');b=r/'reverse-engineering/tiny-glade/evidence/round12/wall-gpu/candidates-sheep';
def rd(rel):return (r/'tiny-glade'/rel).read_text(encoding='utf-8')
def wr(rel,s):p=b/rel;p.parent.mkdir(parents=True,exist_ok=True);p.write_text(s,encoding='utf-8')
rel=Path('crates/country-core/src/components/drawable.rs');s=rd(rel);s+='''\n/// One canonical previous object Transform, read by actual sheep/duck draw callbacks.\n#[derive(bevy_ecs::component::Component, Clone, Copy, Debug, PartialEq)]\npub struct PrevTransform {\n    pub rotation: glam::Quat,\n    pub translation: glam::Vec3,\n    pub scale: glam::Vec3,\n}\nimpl Default for PrevTransform {\n    fn default() -> Self { Self::from(Transform::identity()) }\n}\nimpl From<Transform> for PrevTransform {\n    fn from(value: Transform) -> Self { Self { rotation:value.rotation,translation:value.translation,scale:value.scale } }\n}\n''';wr(rel,s)
old='''#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct PrevTransformView {
    pub rotation: Quat,
    pub translation: Vec3,
    pub scale: Vec3,
}'''
for rel in [Path('crates/country-core/src/systems/render_loop/draw_passes/draw_callbacks_a_views.rs'),Path('crates/country-core/src/systems/render_loop/draw_passes/draw_callbacks_b.rs')]:
 s=rd(rel);assert old in s;s=s.replace(old,'pub use crate::components::drawable::PrevTransform as PrevTransformView;');wr(rel,s)
rel=Path('crates/systems/critters/src/components.rs');s=rd(rel);start=s.index('#[derive(Component, Clone, Copy, Debug, PartialEq)]\npub struct PrevTransform {');end=s.index('\nimpl Default for Duck {',start);s=s[:start]+'pub use country_core::components::drawable::PrevTransform;\n'+s[end:];wr(rel,s)
# Preserve the existing split walk-state design while making the render reader
# borrow its actual owner through a canonical core type (no extra ECS View).
rel=Path('crates/systems/critters/src/sheep/sheep_walk_animation.rs');s=rd(rel);start=s.index('/// 羊行走动画记录');end=s.index('/// sheep_walk_animation（',start);record=s[start:end];s=s[:start]+'pub use country_core::resources::critters::sheep::SheepWalkRecord;\n\n'+s[end:];wr(rel,s)
rel=Path('crates/country-core/src/resources/critters/sheep.rs');s=rd(rel);s+='\n'+record.replace('pub last_pos: Vec3,','pub last_pos: glam::Vec3,');wr(rel,s)
rel=Path('crates/country-core/src/systems/render_loop/draw_passes/draw_callbacks_a.rs');s=rd(rel);old='''        sheep: *comp::<SheepView>(w, e),''';assert old in s;s=s.replace(old,'''        sheep: {
            let sheep = comp::<crate::resources::critters::sheep::Sheep>(w,e);
            let walk = comp::<crate::resources::critters::sheep::SheepWalkRecord>(w,e);
            SheepView { q28: walk.frame_index, f58: sheep.amplitude, b60: sheep.control_flag }
        },''');wr(rel,s)
rel=Path('crates/country-core/src/systems/render_loop/draw_passes/draw_callbacks_a_tests.rs');s=rd(rel);needle='    let e = w.spawn((sheep, prev)).id();';assert needle in s;s=s.replace(needle,'''    let state = crate::resources::critters::sheep::Sheep {
        behavior_state:0,goal_timer:0.0,pet_phase:0,pet_progress:0.0,pet_target:0,
        move_state:crate::resources::critters::sheep::SheepMoveState::Idle{timeout_s:0.0},
        amplitude:sheep.f58,control_flag:sheep.b60,
    };
    let walk = crate::resources::critters::sheep::SheepWalkRecord { frame_index:sheep.q28,..Default::default() };
    let e = w.spawn((state, walk, prev)).id();''');wr(rel,s)
print('canonical previous/walk-state owner candidates prepared')
