from pathlib import Path
p=Path('D:/game/tiny-glade/crates/systems/toolbar/src/ui_interaction.rs');s=p.read_text(encoding='utf-8');ev=Path('D:/game/reverse-engineering/tiny-glade/progress-20261020/toolbar-layout');(ev/'ui_interaction-before.rs').write_text(s,encoding='utf-8')
s=s.replace('use bevy_ecs::event::{Event, EventWriter};','use bevy_ecs::event::{Event, EventReader, EventWriter};')
s=s.replace('    HistoryButton, TaffyNode, UiAnimationState, UiInteractionState, UiMouseInteractable, UiNav,','    HistoryButton, TaffyNode, UiAnimationState, UiInteractionState, UiLocked, UiMouseInteractable, UiNav,')
s=s.replace('''/// O `sinc` (inline, 2 instances; host ui_locked_animation): `sin(x) / x`,
/// with the removable singularity at 0 returning 1.''','''/// O `sinc` (inline, 2 instances; host ui_locked_animation): `sin(x) / x`.
/// The original 0x14233cd34..0x14233cd39 has no zero branch: preserve its
/// NaN at the removable singularity instead of changing the game algorithm.''')
s=s.replace('''    if x == 0.0 {
        1.0
    } else {
        x.sin() / x
    }
}''','''    x.sin() / x
}''',1)
s=s.replace('''    /// O: sinc is sin(x)/x with sinc(0) = 1.
    #[test]
    fn sinc_has_removable_singularity() {
        assert_eq!(sinc(0.0), 1.0);''','''    /// O: the original sinf/divss sequence preserves the zero singularity.
    #[test]
    fn sinc_preserves_original_zero_singularity() {
        assert!(sinc(0.0).is_nan());
        assert!(sinc(-0.0).is_nan());''')
start=s.index('/// O 0x14233c8b0 `ui_locked_animation`:');end=s.index('\n/// O 0x14233a7f0',start)
s=s[:start]+'''/// O 0x14233c8b0 `ui_locked_animation`.
///
/// Exact five-parameter PDB caller spelling is retained in skeleton/B
/// ui_interaction.rs, function information da21103e06ec109a0ec604e3.
/// First drain clicks and queue `UiPlayLockedAnimation { a: 0.1, b: 0.3,
/// progress: 0.0 }` only for live entities carrying UiLocked. Then animate
/// components already in the query: deferred click inserts become visible
/// at the original command-application boundary, not during this loop.
/// Original 0x14233cd34..0x14233cd39 performs sinf(x)/x with no zero guard.
pub fn ui_locked_animation(
    mut clicks: EventReader<UiClickEvent>,
    locked: Query<&UiLocked>,
    mut query: Query<(Entity, &mut UiPlayLockedAnimation, &mut TaffyNode)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for click in clicks.read() {
        if locked.get(click.entity).is_ok() {
            commands.entity(click.entity).insert(UiPlayLockedAnimation {
                a: 0.1,
                b: 0.3,
                progress: 0.0,
            });
        }
    }
    let dt = time.delta_secs();
    for (entity, mut animation, mut taffy) in &mut query {
        let progress = (dt / animation.b + animation.progress).clamp(0.0, 1.0);
        animation.progress = progress;
        let x = (progress * 5.0 + -1.0) * std::f32::consts::PI;
        let amplitude = sinc(x);
        let scale = (amplitude * animation.a + 1.0) * taffy.scale();
        taffy.set_scale(scale);
        if progress == 1.0 {
            // Native removal command uses the warning handler (insertion uses panic).
            commands.entity(entity).try_remove::<UiPlayLockedAnimation>();
        }
    }
}
'''+s[end:]
p.write_text(s,encoding='utf-8')
