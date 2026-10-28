//! Original presentation consumers: focus14233ba40, appear14233c020,
//! sounds14233a7f0. Full native ranges/fixtures: phase2-ui-presentation.
use crate::{
    resouces_and_components::{
        PhotomodeButton, SwitchAppModeButton, SwitchToolbarCategoryButton, TaffyNode,
        UiAnimationState, UiCustomSize, UiLocked, UiSlider, UiSliderReset,
    },
    ui_interaction::{UiClickEvent, UiDragEvent, UiJustBecameInFocusEvent},
};
use bevy_ecs::prelude::*;
use country_core::systems::ui::resources::widget_params::PlayUiSound;
use utils::easing_functions::EasingFunction;

pub fn focus_scale(
    select: f32,
    slider: bool,
    reset: bool,
    custom: Option<&UiCustomSize>,
    scale: f32,
) -> f32 {
    let mut target = if slider || reset { 1.2 } else { 1.5 };
    if let Some(custom) = custom {
        if custom.kind == 1 {
            target = custom.value;
        }
    }
    let ease = EasingFunction::CubicOut.apply(select);
    (target * ease + (1. - ease)) * scale
}

pub fn appear_scale(appear: f32, scale: f32) -> f32 {
    let ease = EasingFunction::CubicOut.apply(appear);
    // Preserve native floating operations, including NaN/Infinity behavior.
    ((1. - ease) * 0. + ease) * scale
}

pub fn ui_focus_animation(
    mut nodes: Query<(
        &UiAnimationState,
        &mut TaffyNode,
        Option<&UiSlider>,
        Option<&UiSliderReset>,
        Option<&UiCustomSize>,
    )>,
) {
    for (animation, mut node, slider, reset, custom) in &mut nodes {
        let scale = focus_scale(
            animation.select,
            slider.is_some(),
            reset.is_some(),
            custom,
            node.scale(),
        );
        node.set_scale(scale);
    }
}
pub fn ui_appear_animation(mut nodes: Query<(&UiAnimationState, &mut TaffyNode)>) {
    for (animation, mut node) in &mut nodes {
        let scale = appear_scale(animation.appear, node.scale());
        node.set_scale(scale);
    }
}

/// Complete native click precedence. Marker combinations are deliberately
/// supported: the original independent Queries do not assume exclusivity.
pub fn click_sound(
    slider: bool,
    locked: bool,
    photo: Option<u8>,
    app: bool,
    category: bool,
) -> Option<u32> {
    if slider {
        None
    } else if locked {
        Some(11)
    } else if photo == Some(9) {
        Some(29)
    } else if app || category {
        None
    } else {
        Some(14)
    }
}
pub fn drag_sound(slider: bool, state: u8) -> Option<u32> {
    if !slider {
        None
    } else {
        match state {
            2 | 4 => None,
            3 => Some(26),
            _ => Some(27),
        }
    }
}

/// Original nine SystemParams; consumes focus, click, then drag event streams
/// once each, preserving order and repeats. Payload slots are unused variants.
pub fn ui_sounds(
    mut focus: EventReader<UiJustBecameInFocusEvent>,
    mut clicks: EventReader<UiClickEvent>,
    mut drags: EventReader<UiDragEvent>,
    mut sounds: EventWriter<PlayUiSound>,
    locked: Query<&UiLocked>,
    app: Query<&SwitchAppModeButton>,
    category: Query<&SwitchToolbarCategoryButton>,
    sliders: Query<&UiSlider>,
    photo: Query<&PhotomodeButton>,
) {
    let mut emit = |kind| {
        sounds.write(PlayUiSound {
            kind,
            _payload: [0; 20],
        });
    };
    for event in focus.read() {
        emit(if locked.contains(event.entity) {
            10
        } else {
            15
        });
    }
    for event in clicks.read() {
        if let Some(kind) = click_sound(
            sliders.contains(event.entity),
            locked.contains(event.entity),
            photo.get(event.entity).ok().map(|button| button.tag),
            app.contains(event.entity),
            category.contains(event.entity),
        ) {
            emit(kind);
        }
    }
    for event in drags.read() {
        if let Some(kind) = drag_sound(sliders.contains(event.entity), event.state) {
            emit(kind);
        }
    }
}
