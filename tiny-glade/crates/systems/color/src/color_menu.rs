//! Radial color-menu interaction, animation and deletion.
//! B color_menu 0x142291230: corrected 13-entry primary and 9-entry secondary tables.
//! Original comparison: evidence/round12/color-menu/color_menu-analysis.c.

use bevy_ecs::prelude::*;
use ::utils::scalar::{max_f32, min_f32};

use country_core::resources::color_icon::{CustomizationIcon, CustomizationIconMarkedForDeletion};

/// 菜单交互状态（O：`Local<Option<(usize,usize)>>`——上次选中图标/菜单对）。
#[derive(Default)]
pub struct ColorMenuSelection(pub Option<(usize, usize)>);

/// The native main loop keeps these values independently: hovering one kind
/// must not erase another kind selected earlier in the same traversal.
#[derive(Default)]
struct MenuPreview {
    color: Option<u32>,
    flat: Option<[u8; 12]>,
    style: Option<u8>,
    freehand: Option<u8>,
    rim: Option<u8>,
    brightness: Option<f32>,
    peel: Option<f32>,
    railing: Option<u8>,
    magic: Option<u32>,
    has_magic: bool,
    flag: bool,
    smoke: bool,
    commit: bool,
    brightness_commit: bool,
}

/// Original writer tuple plus its separate flat/peel/railing/magic writers.
/// These are the same event identities consumed by the existing game systems.
#[derive(bevy_ecs::system::SystemParam)]
pub struct ColorMenuEvents<'w> {
    wall: EventWriter<'w, country_core::resources::color_id::SetWallColorIdCmd>,
    roof: EventWriter<'w, crate::SetRoofColorIdCmd>,
    flat: EventWriter<'w, crate::SetFlatRoofStyleCmd>,
    decorator: EventWriter<'w, crate::SetDecoratorColorIdCmd>,
    window: EventWriter<'w, crate::SetWindowLightCmd>,
    gate: EventWriter<'w, country_core::resources::color_id::SetGateColorIdCmd>,
    half: EventWriter<'w, country_core::resources::color_id::SetHalftimberColorIdCmd>,
    flag: EventWriter<'w, crate::SetFlagConnectionCmd>,
    brightness: EventWriter<'w, crate::SetDecoratorBrightnessCmd>,
    smoke: EventWriter<'w, crate::SetChimneySmokeCmd>,
    ladder: EventWriter<'w, crate::SetLadderColorIdCmd>,
    railing: EventWriter<'w, crate::SetStairRailingCmd>,
    magic: EventWriter<'w, crate::SetMagicColorIdCmd>,
    style: EventWriter<'w, country_core::systems::wall::SetWallStyleCmd>,
    freehand: EventWriter<'w, country_core::systems::wall::SetFreehandHeightTypeCmd>,
    peel: EventWriter<'w, country_core::systems::wall::SetPlasterPeelAmountCmd>,
    rim: EventWriter<'w, country_core::resources::roofs::commands::SetFlatRoofRimStyleCmd>,
    hide: EventWriter<'w, crate::HideColorMenuCmd>,
    finish: EventWriter<'w, country_core::resources::history::FinishInputRecordingCmd>,
    sounds: EventWriter<'w, country_core::systems::ui::resources::widget_params::PlayUiSound>,
    demo: EventWriter<'w, country_core::resources::glade_border::NotInDemoHint>,
    lantern_on: EventWriter<'w, country_core::resources::events::LightLanternCmd>,
    window_on: EventWriter<'w, country_core::resources::events::LightWindowCmd>,
}

fn record_color<S>(
    history: &mut country_core::resources::history::History<S>,
    edit: country_core::resources::history::color_edit::ColorEdit,
) {
    use country_core::resources::history::EditType;
    let edit = EditType::Color(std::sync::Arc::new(edit));
    history.add(edit.clone()).unwrap_or_else(|_| {
        panic!("Tried writing {edit:?} to History when the input recording is finished")
    });
}

#[derive(Clone, Copy)]
enum IconAction {
    Flat([u8; 12]),
    Smoke,
    Railing(u8),
    Magic(u32),
    Flag,
    Freehand,
    Rim,
    Style(u8),
    Brightness(f32),
    Peel(f32),
    Color(u32),
}
impl IconAction {
    fn prefix(self) -> &'static str {
        match self {
            Self::Flat(_) => "flatroof_icon_",
            Self::Smoke => "smoke_icon_",
            Self::Railing(_) => "railing_icon_",
            Self::Magic(_) => "magic_color_icon_",
            Self::Flag => "flag_icon_",
            Self::Freehand => "freehand_icon_",
            Self::Rim => "flat_roof_icon_",
            Self::Style(_) => "skirt_icon_",
            Self::Brightness(_) => "brightness_icon_",
            Self::Peel(_) => "plaster_peel_icon_",
            Self::Color(_) => "color_icon_",
        }
    }
}

// A single optional-component query preserves the original handler order,
// including the original behavior if an entity carries multiple icon kinds.
type IconPayloads<'a> = (
    Option<&'a country_core::resources::color_icon::FlatRoofStyleIcon>,
    Option<&'a country_core::resources::color_icon::ChimneySmokeToggle>,
    Option<&'a country_core::resources::color_icon::StairRailingIcon>,
    Option<&'a country_core::resources::color_icon::MagicColorIcon>,
    Option<&'a country_core::resources::color_icon::FlagConnectionIcon>,
    Option<&'a country_core::resources::color_icon::FreehandHeightTypeToggleIcon>,
    Option<&'a country_core::resources::color_icon::FlatRoofRimTypeToggle>,
    Option<&'a country_core::resources::color_icon::WallStyleIcon>,
    Option<&'a country_core::resources::color_icon::ColorMenuBrightnessIcon>,
    Option<&'a country_core::resources::color_icon::PlasterPeelIcon>,
    Option<&'a country_core::resources::color_icon::ColorIcon>,
);

fn step_selection(menu: &mut crate::radial_menu::ColorRadialMenu, forward: bool) {
    let count = menu.icons.len();
    for _ in 0..count {
        let selected = menu.interaction_start_index.as_mut()
            .expect("native radial navigation requires an initialized selection");
        *selected = if forward { selected.wrapping_add(1) % count }
            else { selected.wrapping_add(count - 1) % count };
        if menu.icons[*selected].kind != 5 { break; }
    }
}

/// B142291df8 compares the constant against alpha with UCOMISS/JBE.
fn icon_accepts_input(alpha: f32) -> bool { !(alpha < 0.7) }

fn resolve_retained_color(menu: &mut crate::radial_menu::ColorRadialMenu, preview: &MenuPreview) {
    if (menu.target.kind == 10 && preview.smoke) || (menu.target.kind == 6 && preview.flag) {
        menu.preview_color = None;
    }
    if let Some(color) = preview.color { menu.preview_color = Some(color); }
    else if preview.style.is_some() { menu.preview_color = None; }
}

fn apply_menu_preview<S>(
    menu: &crate::radial_menu::ColorRadialMenu,
    preview: &MenuPreview,
    protected: bool,
    events: &mut ColorMenuEvents,
    history: &mut country_core::resources::history::History<S>,
    defaults: &mut country_core::resources::glade_color_customization::GladeColorCustomization,
    walls: &country_core::resources::walls::public_walls::PublicWalls,
    wall_colors: &country_core::resources::color_id::WallColorIds,
    half_colors: &country_core::resources::color_id::HalftimberColorIds,
    rim_now: u8,
) {
    use country_core::resources::{
        color_id::{ColorId, SetGateColorIdCmd, SetHalftimberColorIdCmd, SetWallColorIdCmd},
        history::color_edit::ColorEdit,
        roofs::{customization::{FlatRoofStyleObserved, FlatRoofStyleRequest, FlatRoofRimRequest}, roof_shape::FlatRoofStyle},
        terrain_editing::StrokeId,
        walls::{WallId, decorator::DecoratorId},
    };
    let target = menu.target;
    let initial = menu.initial;
    let wall = WallId(target.id);
    let id = target.payload;
    let commit = preview.commit;
    let mut color_feedback = true;

    if target.kind == 6 {
        let value = initial.flag_connection ^ u8::from(preview.flag);
        events.flag.write(crate::SetFlagConnectionCmd { id, value, custom: u8::from(commit), _pad: [0; 2] });
        if preview.flag {
            color_feedback = false;
            if commit {
                record_color(history, ColorEdit::FlagBunting { decorator_id: DecoratorId(id),
                    from: initial.flag_connection != 0, to: value != 0 });
            }
        }
    }
    if target.kind == 9 {
        let value = preview.brightness.unwrap_or(initial.brightness);
        events.brightness.write(crate::SetDecoratorBrightnessCmd { id, value: value.to_bits(),
            custom: u8::from(preview.brightness_commit) });
        if preview.brightness.is_some() { color_feedback = false; }
        if preview.brightness.is_some() && preview.brightness_commit {
            record_color(history, ColorEdit::Brightness { decorator_id: DecoratorId(id),
                from: initial.brightness, to: value });
        }
    }
    if target.kind == 10 && preview.smoke {
        color_feedback = false;
        if commit {
            let value = initial.chimney_smoke ^ 1;
            events.smoke.write(crate::SetChimneySmokeCmd { id, value, custom: 1, _pad: [0; 2] });
            record_color(history, ColorEdit::ChimneySmoke { decorator_id: DecoratorId(id),
                from: initial.chimney_smoke != 0, to: value != 0 });
        }
    }
    if target.kind == 2 {
        if let Some(bytes) = preview.flat {
            let style = FlatRoofStyleObserved::from_observed(bytes);
            events.flat.write(crate::SetFlatRoofStyleCmd(FlatRoofStyleRequest {
                wall_id: country_core::resources::walls::publication_fields::WallId(target.id),
                style, observed_play_sound: u8::from(commit),
            }));
            if commit && !style.notification_eq(&FlatRoofStyleObserved::from_observed(initial.flat_roof_style)) {
                record_color(history, ColorEdit::FlatRoof { wall_id: wall,
                    from: FlatRoofStyle::from_observed(initial.flat_roof_style), to: FlatRoofStyle::from_observed(bytes) });
            }
        } else if !protected {
            events.flat.write(crate::SetFlatRoofStyleCmd(FlatRoofStyleRequest {
                wall_id: country_core::resources::walls::publication_fields::WallId(target.id),
                style: FlatRoofStyleObserved::from_observed(initial.flat_roof_style),
                observed_play_sound: u8::from(commit),
            }));
        }
    }
    if target.kind == 11 || target.kind == 12 {
        if target.kind == 11 {
          if let Some(value) = preview.railing {
            events.railing.write(crate::SetStairRailingCmd { id, value, custom: u8::from(commit) });
            if commit {
                if initial.railing != value {
                    record_color(history, ColorEdit::StairRailing { decorator_id: DecoratorId(id), from: initial.railing, to: value });
                    defaults.stairs.railing_ty = value.try_into().expect("native stair railing tag");
                }
            }
          } else if !protected {
            events.railing.write(crate::SetStairRailingCmd { id, value: initial.railing, custom: 0 });
          }
        }
        if let Some(value) = preview.magic {
            events.magic.write(crate::SetMagicColorIdCmd { id, value, custom: u8::from(commit) });
            if commit {
                if initial.magic_color != value {
                    record_color(history, ColorEdit::MagicColor { decorator_id: DecoratorId(id),
                        from: StrokeId(initial.magic_color), to: StrokeId(value) });
                    defaults.stairs.magic_color = ColorId(value);
                }
            }
        } else if preview.has_magic && !protected {
            events.magic.write(crate::SetMagicColorIdCmd { id, value: initial.magic_color, custom: 0 });
        }
    }
    if target.kind == 0 {
        let state = walls.get(wall).expect("original color menu requires its public wall");
        let style = preview.style.unwrap_or(initial.wall_style);
        if preview.style.is_some() || state.authority.style() != initial.wall_style {
            events.style.write(country_core::systems::wall::SetWallStyleCmd {
                wall_id: wall, from: initial.wall_style, to: style,
                record_history: u8::from(commit && preview.style.is_some()),
                play_sound: u8::from(commit && preview.style.is_some()),
            });
            if preview.style.is_some() {
                if commit { defaults.building_wall_style = country_core::resources::walls::inner_wall_state::WallStyle(style); }
            }
        }
        let peel = preview.peel.unwrap_or(initial.plaster_peel);
        if preview.peel.is_some() || state.authority.plaster_peel_amount() != initial.plaster_peel {
            events.peel.write(country_core::systems::wall::SetPlasterPeelAmountCmd {
                wall_id: wall, from: initial.plaster_peel, to: peel,
                record_history: u8::from(commit && preview.peel.is_some()),
            });
        }
        let freehand = preview.freehand.unwrap_or(initial.freehand_ty);
        if preview.freehand.is_some() || state.authority.freehand_ty() != initial.freehand_ty {
            events.freehand.write(country_core::systems::wall::SetFreehandHeightTypeCmd {
                wall_id: wall, from: initial.freehand_ty, to: freehand,
                record_history: u8::from(commit && preview.freehand.is_some()),
            });
            if commit && preview.freehand.is_some() {
                defaults.freehand_ty = freehand.try_into().expect("native freehand height tag");
                defaults.freeform_wall = ColorId(initial.color);
            }
        }
        let rim = preview.rim.unwrap_or(initial.flat_roof_rim_style);
        if preview.rim.is_some() || rim_now != initial.flat_roof_rim_style {
            events.rim.write(country_core::resources::roofs::commands::SetFlatRoofRimStyleCmd(FlatRoofRimRequest {
                wall_id: country_core::resources::walls::publication_fields::WallId(target.id),
                from: initial.flat_roof_rim_style, to: rim,
                observed_record_history: u8::from(commit && preview.rim.is_some()),
            }));
            if commit && preview.rim.is_some() { defaults.flat_rim_style = rim.try_into().expect("native flat rim tag"); }
        }
    }

    let color = menu.preview_color.unwrap_or(initial.color);
    if color == 777 {
        // B142295d73 branches to the common close case even for demo lock.
        if commit { events.hide.write(crate::HideColorMenuCmd { cancel: false }); }
        return;
    }
    let changed = commit && initial.color != color;
    let custom = u8::from(commit);
    let edit = match target.kind {
        0 => {
            events.wall.write(SetWallColorIdCmd { id: target.id as u32, _gap4: (target.id >> 32) as u32,
                color, custom: u8::from(commit && color_feedback), _pad: [0; 3] });
            if commit {
                defaults.freeform_wall = ColorId(color);
                defaults.building = ColorId(color);
                if let Some(state) = walls.get(wall) {
                    if state.shape().tag() < 2 {
                        if state.authority.style() >= 2 { defaults.halftimber = ColorId(half_colors.0[wall.0 as usize]); }
                        defaults.building_wall_style = country_core::resources::walls::inner_wall_state::WallStyle(state.authority.style());
                    } else {
                        defaults.freehand_ty = state.authority.freehand_ty().try_into().expect("native public freehand type");
                    }
                }
            }
            ColorEdit::Wall { wall_id: wall, from: StrokeId(initial.color), to: StrokeId(color) }
        }
        1 => {
            events.roof.write(crate::SetRoofColorIdCmd { id: target.id, color, custom });
            if commit { defaults.roof = ColorId(color); }
            ColorEdit::Roof { wall_id: wall, from: StrokeId(initial.color), to: StrokeId(color) }
        }
        2 => { if commit { events.hide.write(crate::HideColorMenuCmd { cancel: false }); } return; }
        4 => {
            events.window.write(crate::SetWindowLightCmd { id, value: color, custom });
            if changed { defaults.window_light = ColorId(color); }
            ColorEdit::WindowLight { decorator_id: DecoratorId(id), from: StrokeId(initial.color), to: StrokeId(color) }
        }
        5 => {
            events.gate.write(SetGateColorIdCmd { id: target.id as u32, _gap4: (target.id >> 32) as u32, color, custom, _pad: [0; 3] });
            if commit { defaults.gates = ColorId(color); }
            ColorEdit::Gate { wall_id: wall, from: StrokeId(initial.color), to: StrokeId(color) }
        }
        7 | 8 => {
            events.half.write(SetHalftimberColorIdCmd { id: target.id as u32, _gap4: (target.id >> 32) as u32, color, custom, _pad: [0; 3] });
            if commit {
                if target.kind == 7 {
                    defaults.halftimber = ColorId(color);
                    defaults.building = ColorId(wall_colors.0[wall.0 as usize]);
                    defaults.building_wall_style = country_core::resources::walls::inner_wall_state::WallStyle::HALF_TIMBERED;
                } else { defaults.fences = ColorId(color); }
            }
            ColorEdit::Halftimber { wall_id: wall, from: StrokeId(initial.color), to: StrokeId(color) }
        }
        12 => {
            events.ladder.write(crate::SetLadderColorIdCmd { id, value: color, custom });
            if changed { defaults.stairs.ladder_color = ColorId(color); }
            ColorEdit::StairLadder { decorator_id: DecoratorId(id), from: StrokeId(initial.color), to: StrokeId(color) }
        }
        _ => {
            events.decorator.write(crate::SetDecoratorColorIdCmd { id, color, custom: u8::from(commit && color_feedback) });
            if commit {
                match target.kind {
                    3 => defaults.windows = if color == 999 || (color == 0 && target.subtype == 0) { None } else { Some(ColorId(color)) },
                    6 => defaults.flag = ColorId(color),
                    9 => defaults.lantern_light = ColorId(color),
                    11 => defaults.stairs.stairs_color = ColorId(color),
                    _ => {}
                }
            }
            ColorEdit::Decorator { decorator_id: DecoratorId(id), from: StrokeId(initial.color), to: StrokeId(color) }
        }
    };
    if changed { record_color(history, edit); }
    if commit { events.hide.write(crate::HideColorMenuCmd { cancel: false }); }
}

/// B1422979b0: animate the actual 0x24-byte component, not the unrelated
/// 0xa0-byte radial-menu owner. Both marked/unmarked branches are preserved.
pub fn color_button_animations(
    mut icons: Query<(
        ComponentWrite<CustomizationIcon>,
        ComponentWrite<country_core::components::Ui2dExtent>,
        ComponentWrite<country_core::components::UiDrawOrder>,
        Option<&CustomizationIconMarkedForDeletion>,
    )>,
    time: Res<country_core::resources::time::GameTime>,
) {
    for (mut icon, mut extent, mut order, marked) in icons.iter_mut() {
        animate_icon(
            &mut icon,
            &mut extent,
            &mut order,
            marked.is_some(),
            time.delta,
        );
    }
}


fn animate_icon(
    icon: &mut CustomizationIcon,
    extent: &mut country_core::components::Ui2dExtent,
    order: &mut country_core::components::UiDrawOrder,
    deleting: bool,
    delta: f32,
) {
    use utils::easing_functions::EasingFunction;
    icon.alpha_18 += if deleting { delta / -0.15 } else { delta * 4.0 };
    order.0 = u64::from(!deleting);
    icon.alpha_18 = min_f32(icon.alpha_18, 1.0);
    let progress = min_f32(1.0, max_f32(0.0, icon.alpha_18));
    let eased = EasingFunction::BackOut.apply(progress);
    extent.size = icon.size * eased;
    if deleting {
        return;
    }
    // Native evaluates BackOut again before the position interpolation.
    let eased = EasingFunction::BackOut.apply(progress);
    let mix = (1.0 - eased) * 0.8 + eased;
    extent.pos = icon.position * mix + icon.initial_position * (1.0 - mix);
    // This is a per-frame increment in the original, not multiplied by dt.
    let step = if icon.hovered != 0 { 0.15 } else { -0.15 };
    icon.hover_progress = min_f32(1.0, max_f32(0.0, icon.hover_progress + step));
    let eased = EasingFunction::CubicOut.apply(icon.hover_progress);
    let scale = (1.0 - eased) * 0.0 + eased * 0.2 + 1.0;
    extent.size *= scale;
}

/// Production interaction for the recovered B color-menu owner.
#[allow(clippy::too_many_arguments)]
pub fn color_menu<S: Send + Sync + 'static>(
    mut icons: Query<(ComponentWrite<CustomizationIcon>, &country_core::components::Ui2dExtent,
        ComponentWrite<country_core::components::Ui2dImage>)>,
    payloads: Query<IconPayloads<'static>>,
    mut menus: Query<(Entity, &mut crate::radial_menu::ColorRadialMenu)>,
    mut selection: Local<ColorMenuSelection>,
    mut history: ResMut<country_core::resources::history::History<S>>,
    actions: Res<country_core::input::action_set::MainActionSet>,
    mut nav: ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,
    ui: Res<country_core::systems::ui::resources::ui_state::UiState>,
    cursor: Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,
    config: Res<country_core::game_config::GameConfig>,
    mode: Res<country_core::systems::mode_manager::AppMode>,
    mut defaults: ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,
    walls: Res<country_core::resources::walls::public_walls::PublicWalls>,
    colors: (Res<country_core::resources::color_id::WallColorIds>, Res<country_core::resources::color_id::HalftimberColorIds>),
    roofs: Query<&country_core::resources::roofs::roof_shape::Roof>,
    mut events: ColorMenuEvents,
) {
    use country_core::systems::ui::resources::widget_params::{PlayUiSound, UiFocus, UiSounds, WidgetParams, WidgetVolumeShape};
    let next = actions.digital[20].just_pressed();
    let previous = actions.digital[21].just_pressed();
    let confirm = actions.digital[19].just_pressed();
    let mut any_pointer_active = false;
    let mut pointer_selection = None;
    let mut marked_selection = None;
    let mut marked_sound = 0;
    let mut recording_started = false;
    let mut any_selected_icon = false;
    for (menu_index, (entity, mut menu)) in menus.iter_mut().enumerate() {
        let keyboard_focus = nav.focus.tag == 1 && nav.focus.payload == entity.to_bits();
        if next || previous {
            let initial = menu.selected_index.unwrap_or(0);
            menu.interaction_start_index.get_or_insert(initial);
            if keyboard_focus {
                if next { step_selection(&mut menu, true); }
                if previous { step_selection(&mut menu, false); }
            }
        }
        let selected_at_start = menu.interaction_start_index;
        any_selected_icon |= selected_at_start.is_some_and(|index| index < menu.icon_entities.len());
        let mut preview = MenuPreview::default();
        for index in 0..menu.icon_entities.len() {
            let icon_entity = menu.icon_entities[index];
            if menu.target.kind == 9 {
                events.lantern_on.write(country_core::resources::events::LightLanternCmd(menu.target.payload));
            }
            let (mut icon, extent, mut image) = icons.get_mut(icon_entity)
                .expect("original color menu icon entity is missing");
            icon.hovered = 0;
            // UCOMISS 0.7, alpha; JBE enters on equality and unordered too.
            if !icon_accepts_input(icon.alpha_18) { continue; }
            let (flat, smoke, railing, magic, flag, freehand, rim, style, brightness, peel, color) =
                payloads.get(icon_entity).expect("original color icon payload entity is missing");
            let handlers = [
                flat.map(|v| IconAction::Flat(*v.0.bytes())), smoke.map(|_| IconAction::Smoke),
                railing.map(|v| IconAction::Railing(v.0)), magic.map(|v| IconAction::Magic(v.0)),
                flag.map(|_| IconAction::Flag), freehand.map(|_| IconAction::Freehand), rim.map(|_| IconAction::Rim),
                style.map(|v| IconAction::Style(v.0)), brightness.map(|v| IconAction::Brightness(v.0.to_f32())),
                peel.map(|v| IconAction::Peel(v.0)), color.map(|v| IconAction::Color(v.0)),
            ];
            for handler in handlers.into_iter().flatten() {
                let locked = matches!(handler, IconAction::Color(777));
                let hover_sound = if locked { 10u32 } else { 12 };
                let click_sound = if locked { 11u32 } else { 13 };
                let mut sounds = UiSounds::empty();
                sounds.slots[0][..4].copy_from_slice(&hover_sound.to_le_bytes());
                sounds.slots[3][..4].copy_from_slice(&click_sound.to_le_bytes());
                let mut params = WidgetParams::new(WidgetVolumeShape::default(), [0; 2])
                    .screen_aabb(extent.pos - extent.size * 0.5, extent.pos + extent.size * 0.5, 0.0)
                    .sounds(sounds);
                if matches!(handler, IconAction::Color(_)) { params = params.occlude_by_world(false); }
                // The native literal pieces are (prefix, ""): no extra separator.
                let name = format!("{}{index}{menu_index}", handler.prefix());
                let widget = ui.add(&name, params);
                let active = widget.is_active();
                let keyboard_selected = selected_at_start == Some(index);
                let use_value = active || (!any_pointer_active && keyboard_selected);
                if active {
                    any_pointer_active = true;
                    pointer_selection = Some((entity, index));
                }
                if active || (!any_pointer_active && keyboard_focus && keyboard_selected) {
                    icon.hovered = 1;
                    marked_selection = Some((menu_index, index));
                    marked_sound = hover_sound;
                }
                let committed = widget.clicked() || (confirm && keyboard_selected);
                preview.commit |= committed;
                match handler {
                    IconAction::Flat(value) if use_value => { preview.flat = Some(value); }
                    IconAction::Smoke => {
                        preview.smoke |= use_value;
                        image.name = if (menu.initial.chimney_smoke ^ u8::from(use_value)) != 0 {
                            "color_icon/chimney_smoke_active"
                        } else { "color_icon/chimney_smoke_inactive" }.to_owned();
                    }
                    IconAction::Flag => {
                        preview.flag |= use_value;
                        image.name = if (menu.initial.flag_connection ^ u8::from(use_value)) != 0 {
                            "color_icon/flag_connection"
                        } else { "color_icon/flag_no_connection" }.to_owned();
                    }
                    IconAction::Freehand => {
                        let value = menu.initial.freehand_ty ^ u8::from(use_value);
                        if use_value { preview.freehand = Some(value); }
                        image.name = if value != 0 { "color_icon/freehand_ty_constant" }
                            else { "color_icon/freehand_ty_terrain" }.to_owned();
                    }
                    IconAction::Rim => {
                        let value = menu.initial.flat_roof_rim_style ^ u8::from(use_value);
                        if use_value { preview.rim = Some(value); }
                        // B1422937xx calls ClutterVersion::is_initial and uses
                        // these same two original freehand-named textures.
                        image.name = if value == 0 { "color_icon/freehand_ty_terrain" }
                            else { "color_icon/freehand_ty_constant" }.to_owned();
                    }
                    IconAction::Style(value) if use_value => preview.style = Some(value),
                    IconAction::Brightness(value) => {
                        if use_value { preview.brightness = Some(value); }
                        if committed { preview.brightness_commit = true; defaults.lantern_brightness = value; }
                    }
                    IconAction::Railing(value) if use_value => preview.railing = Some(value),
                    IconAction::Magic(value) => {
                        preview.has_magic = true;
                        if use_value { preview.magic = Some(value); }
                    }
                    IconAction::Peel(value) => {
                        if use_value { preview.peel = Some(value); }
                        if use_value || committed { menu.preview_color = None; }
                    }
                    IconAction::Color(value) if use_value => {
                        preview.color = Some(value);
                        if value == 777 { events.demo.write(country_core::resources::glade_border::NotInDemoHint(cursor.0)); }
                        if menu.target.kind == 4 {
                            events.window_on.write(country_core::resources::events::LightWindowCmd(menu.target.payload));
                        }
                    }
                    _ => {}
                }
                if committed { events.sounds.write(PlayUiSound { kind: click_sound, ..Default::default() }); }
            }
        }
        // This is independent of widget hit-testing and deduplicated across
        // every radial owner in the same invocation, as in1422947f0..4871.
        if confirm {
            if recording_started || history.try_start_input_recording(mode.native_tag()) {
                recording_started = true;
                events.finish.write(country_core::resources::history::FinishInputRecordingCmd);
            } else {
                preview.commit = false;
                preview.brightness_commit = false;
            }
        }
        // B1422948f6/4a3b clear the retained color while smoke/flag is
        // previewed; a color selected in this traversal takes precedence at
        // 142294f10. Style clears only when no color was selected.
        resolve_retained_color(&mut menu, &preview);
        let delta = cursor.0 - menu.center;
        let angle = delta.y.atan2(delta.x);
        let angle = if angle < 0.0 { angle + std::f32::consts::TAU } else { angle };
        let angle = menu.angle_range.expect("native radial menu angle range missing").clamp(angle);
        let point = glam::Vec2::new(angle.cos(), angle.sin()) * menu.radius + menu.center;
        let protected = (point - cursor.0).length() <= menu.icon_size * config.ui.icon_size_mult * 0.5 + 5.0
            || any_selected_icon;
        if protected {
            ui.add(&format!("hackyhack_{}", cursor.0),
                WidgetParams::new(WidgetVolumeShape::default(), [0; 2])
                    .screen_aabb(cursor.0 - glam::Vec2::splat(10.0), cursor.0 + glam::Vec2::splat(10.0), 0.01)
                    .sounds(UiSounds::empty()));
        } else { menu.preview_color = None; }
        let rim_now = walls.roof_entity(country_core::resources::walls::WallId(menu.target.id))
            .and_then(|id| roofs.get(id).ok()).map(|roof| roof.observed()[0x3d]).unwrap_or(0);
        apply_menu_preview(&menu, &preview, protected, &mut events, &mut history, &mut defaults,
            &walls, &colors.0, &colors.1, rim_now);
    }
    if selection.0 != marked_selection && marked_sound != 0 {
        events.sounds.write(PlayUiSound { kind: marked_sound, ..Default::default() });
    }
    selection.0 = marked_selection;
    if let Some((entity, index)) = pointer_selection {
        for (_, mut menu) in &mut menus { menu.interaction_start_index = None; }
        menus.get_mut(entity).expect("active radial owner missing").1.interaction_start_index = Some(index);
        nav.focus = UiFocus { tag: 1, _pad: 0, payload: entity.to_bits() };
    }
}

/// B142297ff0: delete marked entities only when the animation is finished.
pub fn delete_color_icon_entities(
    icons: Query<(
        Entity,
        &CustomizationIcon,
        &CustomizationIconMarkedForDeletion,
    )>,
    mut commands: Commands,
) {
    for (entity, icon, _) in icons.iter() {
        // UCOMISS 0,alpha; JB keeps positive or unordered values.
        if icon.alpha_18 <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

#[cfg(test)]
mod animation_tests {
    use super::*;
    #[test]
    fn flag_and_smoke_clear_retained_color_but_current_color_wins() {
        for kind in [6, 10] {
            let mut menu = crate::radial_menu::ColorRadialMenu::default();
            menu.target.kind = kind;
            menu.preview_color = Some(42);
            let mut preview = MenuPreview { flag: kind == 6, smoke: kind == 10, ..Default::default() };
            resolve_retained_color(&mut menu, &preview);
            assert_eq!(menu.preview_color, None);
            preview.color = Some(7);
            preview.style = Some(2);
            resolve_retained_color(&mut menu, &preview);
            assert_eq!(menu.preview_color, Some(7));
        }
    }
    #[test]
    fn original_alpha_gate_keeps_equal_and_unordered_values() {
        assert!(!icon_accepts_input(f32::from_bits(0x3f33_3332)));
        assert!(icon_accepts_input(0.7));
        assert!(icon_accepts_input(f32::NAN));
    }

    #[test]
    fn selection_wraps_and_skips_separator_in_both_directions() {
        use crate::radial_menu::{ColorRadialMenu, IconInfo};
        let mut menu = ColorRadialMenu {
            icons: vec![IconInfo::default(), IconInfo { kind: 5, ..Default::default() }, IconInfo::default()],
            interaction_start_index: Some(0), ..Default::default()
        };
        step_selection(&mut menu, true);
        assert_eq!(menu.interaction_start_index, Some(2));
        step_selection(&mut menu, true);
        assert_eq!(menu.interaction_start_index, Some(0));
        step_selection(&mut menu, false);
        assert_eq!(menu.interaction_start_index, Some(2));
    }
    #[test]
    fn retained_icon_animates_then_deletes_without_overwriting_exit_position() {
        use country_core::components::{Ui2dExtent, UiDrawOrder};
        let mut icon = CustomizationIcon {
            size: glam::Vec2::splat(30.),
            position: glam::Vec2::new(100., 200.),
            initial_position: glam::Vec2::ZERO,
            hovered: 1,
            ..Default::default()
        };
        let mut extent = Ui2dExtent::default();
        let mut order = UiDrawOrder::default();
        animate_icon(&mut icon, &mut extent, &mut order, false, 0.25);
        assert_eq!(icon.alpha_18, 1.);
        assert_eq!(order.0, 1);
        assert_eq!(icon.hover_progress, 0.15);
        assert_eq!(extent.pos, icon.position);
        assert!(extent.size.x > 30.);
        let position = extent.pos;
        animate_icon(&mut icon, &mut extent, &mut order, true, 0.15);
        assert_eq!(order.0, 0);
        assert_eq!(icon.alpha_18, 0.);
        assert_eq!(extent.pos, position);
        // Native BackOut keeps sin(pi)'s floating-point endpoint residue.
        assert!(extent.size.abs().max_element() < 0.0001);
        let mut world = World::new();
        let exiting = world.spawn((icon, CustomizationIconMarkedForDeletion)).id();
        let pending = world
            .spawn((
                CustomizationIcon {
                    alpha_18: 0.1,
                    ..Default::default()
                },
                CustomizationIconMarkedForDeletion,
            ))
            .id();
        let mut schedule = Schedule::default();
        schedule.add_systems(delete_color_icon_entities);
        schedule.run(&mut world);
        assert!(world.get_entity(exiting).is_err());
        assert!(world.get_entity(pending).is_ok());
    }
}

use country_core::render::extract::ComponentWrite;
