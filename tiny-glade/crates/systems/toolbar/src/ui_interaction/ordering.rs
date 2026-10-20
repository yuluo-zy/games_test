//! Original B14233e760/512B: Bevy query traversal plus the single business
//! assignment `UiDrawOrder = u64(UiInteractionState.state)`. Query metadata is
//! recovered from its run_unsafe host 141ea3c10, not inferred from the name.
use bevy_ecs::prelude::*;
use country_core::components::UiDrawOrder;
use crate::resouces_and_components::{SwitchAppModeButton, UiInteractionState};

pub fn sort_contextual_buttons_draw_order(
    mut items: Query<(&UiInteractionState, &mut UiDrawOrder), With<SwitchAppModeButton>>,
) {
    for (interaction, mut order) in &mut items {
        order.0 = u64::from(interaction.state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use country_core::systems::mode_manager::AppMode;
    #[test]
    fn original_query_filter_and_every_state_byte_determine_the_draw_sort_key() {
        let mut world = World::new();
        let affected: Vec<_> = (0..=255u8).map(|state| world.spawn((
            SwitchAppModeButton { mode: AppMode::Photomode },
            UiInteractionState { state, keep: state.wrapping_add(1), selected: 255 },
            UiDrawOrder(u64::MAX),
        )).id()).collect();
        let unrelated = world.spawn((UiInteractionState::default(), UiDrawOrder(123))).id();
        let mut schedule = Schedule::default();
        schedule.add_systems(sort_contextual_buttons_draw_order);
        schedule.run(&mut world);
        for (state, entity) in affected.into_iter().enumerate() {
            assert_eq!(world.get::<UiDrawOrder>(entity).unwrap().0, state as u64);
        }
        assert_eq!(world.get::<UiDrawOrder>(unrelated).unwrap().0, 123);
    }
}
