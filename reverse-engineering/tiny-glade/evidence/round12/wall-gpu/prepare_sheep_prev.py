from pathlib import Path
p=Path('D:/game/reverse-engineering/tiny-glade/evidence/round12/wall-gpu/candidates-sheep/crates/country-core/src/components/drawable.rs');s=p.read_text(encoding='utf-8');s+='''
/// B14082bd80/628B, PDB FunctionSystem140b51320: unfiltered
/// Query<(&Transform, &mut PrevTransform)>. Bevy owns changed ticks.
pub fn cache_prev_object_transforms(
    mut transforms: bevy_ecs::system::Query<(&Transform, &mut PrevTransform)>,
) {
    for (current, mut previous) in &mut transforms { *previous = PrevTransform::from(*current); }
}
''';p.write_text(s,encoding='utf-8')
p=Path('D:/game/reverse-engineering/tiny-glade/evidence/round12/wall-gpu/candidates-sheep/crates/country-core/src/systems/render_loop/draw_passes/draw_callbacks_a_views.rs');s=p.read_text(encoding='utf-8');a=s.index('pub struct SheepView {');z=s.rfind('#[derive(',0,a);s=s[:z]+s[z:a].replace('Component, ','')+s[a:];s=s.replace('（TypeId 0x60556123d75da34f/0x847d370ca9cbbae9；crate 里不存在）',' rendering payload projection (the actual ECS owners are Sheep + SheepWalkRecord)');p.write_text(s,encoding='utf-8')
