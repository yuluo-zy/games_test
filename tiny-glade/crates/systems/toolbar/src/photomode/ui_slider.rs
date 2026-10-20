//! B 1423486d0..142349de0: complete slider draw/input/lifetime consumer.
//! Parameter owners remain in the verified parent module. Bevy owns iteration,
//! entity generations and deferred command application; no ECS storage mirror.
use super::{apply_slider_with_eraser, slider_current_position, slider_math};
use crate::{resouces_and_components::UiSlider, ui_interaction::UiDragEvent};
use bevy_ecs::prelude::*;
use country_core::{
    components::{Ui2dExtent, Ui2dImage, UiDrawOrder},
    input::action_set::{ActionSetStates, DigitalActionState},
    resources::{
        brush_sizes::BrushSizes, image_formation::ImageFormationConfigs,
        mouse_terrain_raycast::CursorPositionSS, time::GameTime, walls::EraserSizeMultiplier,
        window::WindowFocusEvent,
    },
    systems::{
        camera_rig::main_camera::MainCamera,
        mode_manager::{AppMode, RequestToolbarCategoryChange},
        ui::resources::widget_params::{PlayUiSound, UiFocus, UiNavState},
    },
};
use glam::Vec2;
use utils::{easing_functions::EasingFunction, geometry::aabb::Aabb2};

/// Original Local<GrabOffset> is an optional scalar, retained between events.
/// A finish/cancel event does not clear it in the original consumer.
#[derive(Default, Debug)]
pub struct GrabOffset(Option<f32>);

#[derive(Clone, Copy, Debug)]
pub struct SliderGeometry {
    pub lower: f32,
    pub upper: f32,
    pub y: f32,
    pub side: f32,
}

pub fn geometry(center: Vec2, dimensions: Vec2) -> SliderGeometry {
    let side = dimensions.y * 0.3;
    let half_travel = (side * -2.5 + dimensions.x) * 0.5;
    SliderGeometry {
        lower: center.x - half_travel,
        upper: center.x + half_travel,
        y: center.y,
        side,
    }
}

impl SliderGeometry {
    pub fn knob_center(self, fraction: f32) -> Vec2 {
        Vec2::new(
            fraction * self.upper + (1.0 - fraction) * self.lower,
            self.y,
        )
    }
    pub fn hit_knob(self, fraction: f32, cursor: Vec2) -> bool {
        Aabb2::from_center_dims(self.knob_center(fraction), Vec2::splat(self.side * 1.3))
            .contains(cursor)
    }
    pub fn fraction_at(self, cursor_x: f32, offset: f32) -> f32 {
        slider_math::clamp_01(slider_math::inverse_lerp(
            self.lower,
            self.upper,
            cursor_x + offset,
        ))
    }
}

pub fn visual_state(
    shape: SliderGeometry,
    fraction: f32,
    hover: f32,
    delta: f32,
    cursor: Vec2,
) -> (f32, Ui2dExtent) {
    let step = delta / 0.2;
    let hover = slider_math::clamp_01(
        hover
            + if shape.hit_knob(fraction, cursor) {
                step
            } else {
                -step
            },
    );
    // Original static EasingFunction discriminant 6 at 14304ed20.
    let eased = EasingFunction::CubicOut.apply(hover);
    let side = shape.side * (eased * 1.3 + (1.0 - eased));
    (
        hover,
        Ui2dExtent {
            pos: shape.knob_center(fraction),
            size: Vec2::splat(side),
        },
    )
}

/// Original f32->u64 saturation and truncation before comparing 14 bins.
pub fn sound_bucket(fraction: f32) -> u64 {
    (fraction * 14.0) as u64
}

/// Original final four-way switch 142349a85..142349b3c. The payload is a
/// toolbar-category command, despite its ICF EventInstance<WindowFocusEvent>.
pub fn requested_category(kind: u8, mode_tag: u32) -> Option<u8> {
    match kind {
        9 if mode_tag != 9 => Some(4),
        10 if !(7..9).contains(&mode_tag) => Some(1),
        11 if mode_tag != 15 => Some(8),
        12 if mode_tag != 17 => Some(9),
        _ => None,
    }
}

/// Native focused-slider navigation block 142348cfc..142348ef9.
#[allow(clippy::too_many_arguments)]
pub fn navigation_input(entity_bits: u64, focus: UiFocus, previous: UiFocus,
    left: DigitalActionState, right: DigitalActionState, delta: f32,
    fraction: f32, armed: &mut bool) -> (Option<f32>, bool)
{
    if focus.tag != 1 || focus.payload != entity_bits { return (None, false); }
    if (left.just_pressed() || right.just_pressed()) && previous == focus { *armed = true; }
    if left.released() && right.released() {
        *armed = false;
        (None, false)
    } else if *armed {
        let mut next = None;
        if left.pressed() { next = Some(slider_math::clamp_01(fraction - delta)); }
        if right.pressed() { next = Some(slider_math::clamp_01(fraction + delta)); }
        (next, left.just_pressed() || right.just_pressed())
    } else { (None, false) }
}

/// The original sixteen SystemParams, including the three persistent locals.
/// Draw first, navigate second, then consume all drag phases in event order.
/// At most the final selected candidate writes parameter state this frame.
#[allow(clippy::too_many_arguments)]
pub fn ui_slider(
    mut drags: EventReader<UiDragEvent>,
    cursor: Res<CursorPositionSS>,
    mut sliders: Query<(Entity, &Ui2dExtent, &mut UiSlider)>,
    mut commands: Commands,
    mut grab: Local<GrabOffset>,
    time: Res<GameTime>,
    navigation: Res<UiNavState>,
    actions: Res<ActionSetStates>,
    (mut nav_armed, mut previous_focus): (Local<bool>, Local<UiFocus>),
    mut formation: ResMut<ImageFormationConfigs>,
    mut camera: ResMut<MainCamera>,
    mut sounds: EventWriter<PlayUiSound>,
    mut eraser: ResMut<EraserSizeMultiplier>,
    mut brushes: ResMut<BrushSizes>,
    mode: Res<AppMode>,
    mut categories: EventWriter<RequestToolbarCategoryChange>,
) {
    let mut candidate: Option<(Entity, f32)> = None;
    let mut category_candidate = None;
    let left = actions.digital[20];
    let right = actions.digital[21];
    for (entity, extent, mut slider) in &mut sliders {
        if let Some(old) = slider.knob_entity() {
            commands.entity(old).despawn();
        }
        slider.set_knob_entity(None);
        // Original sqrt/ordered comparison skips zero or NaN length, even
        // after queuing the previous knob's despawn.
        if !(extent.size.length() > 0.0) {
            continue;
        }
        let shape = geometry(extent.pos, extent.size);
        let fraction =
            slider_current_position(slider.kind(), &formation, &camera, &eraser, &brushes)
                .expect("invalid native UiSliderType");
        let (hover, knob_extent) = visual_state(
            shape,
            fraction,
            slider.hover_progress(),
            time.delta,
            cursor.0,
        );
        slider.set_hover_progress(hover);
        let knob = commands
            .spawn((
                knob_extent,
                Ui2dImage::new("toolbar/slider_widget"),
                UiDrawOrder(1),
            ))
            .id();
        slider.set_knob_entity(Some(knob));

        let (next, category) = navigation_input(entity.to_bits(), navigation.focus,
            *previous_focus, left, right, time.delta, fraction, &mut nav_armed);
        if let Some(fraction) = next { candidate = Some((entity, fraction)); }
        if category { category_candidate = Some(entity); }
    }

    for drag in drags.read() {
        let Ok((entity, extent, mut slider)) = sliders.get_mut(drag.entity) else {
            continue;
        };
        let shape = geometry(extent.pos, extent.size);
        let fraction =
            slider_current_position(slider.kind(), &formation, &camera, &eraser, &brushes)
                .expect("invalid native UiSliderType");
        slider.set_hover_progress(1.0);
        if drag.state == 3 {
            (*grab).0 = Some(if shape.hit_knob(fraction, cursor.0) {
                shape.knob_center(fraction).x - cursor.0.x
            } else {
                0.0
            });
        }
        if drag.state < 2 {
            category_candidate = Some(entity);
        }
        if let Some(offset) = (*grab).0 {
            candidate = Some((entity, shape.fraction_at(cursor.0.x, offset)));
        }
    }

    if let Some((entity, fraction)) = candidate {
        // Original chosen-entity requery unwraps, unlike the event loop's
        // fallible query lookup. Do not silently discard that lifecycle error.
        let (_, _, mut slider) = sliders
            .get_mut(entity)
            .expect("selected slider no longer matches query");
        if fraction.is_finite() {
            if sound_bucket(slider.previous_value_01()) != sound_bucket(fraction) {
                sounds.write(PlayUiSound {
                    kind: 0x1c,
                    _payload: [0; 0x10],
                    id: 0,
                });
            }
            let value = slider_math::parameter(slider.kind(), fraction)
                .expect("invalid native UiSliderType");
            apply_slider_with_eraser(
                slider.kind(),
                value,
                &mut formation,
                &mut brushes,
                *mode == AppMode::Photomode,
                *mode,
                &mut camera,
                &mut eraser,
            );
            // The original keeps the normalized position even when AppMode
            // prevents a target write; sound/bin state follows that position.
            slider.set_previous_value_01(fraction);
        }
    }
    if let Some(entity) = category_candidate {
        let (_, _, slider) = sliders
            .get(entity)
            .expect("completed slider no longer matches query");
        if let Some(category) = requested_category(slider.kind(), mode.native_tag()) {
            categories.write(RequestToolbarCategoryChange(WindowFocusEvent(category)));
        }
    }
    *previous_focus = navigation.focus;
}
