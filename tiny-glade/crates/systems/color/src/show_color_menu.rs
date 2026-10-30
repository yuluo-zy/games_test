//! Original color-menu producer and hide lifecycle. The two show dispatch tables each have 13 entries.
//! Original byte identities and corrected tables: evidence/round12/color-menu/REPORT.md.

use bevy_ecs::prelude::*;

use country_core::resources::color_icon::{CustomizationIcon, CustomizationIconMarkedForDeletion};

use crate::radial_menu::{ColorRadialMenu, IconInfo};
use crate::{HideColorMenuCmd, ShowColorMenuCmd};

// Candidate only: the complete primary option-construction algorithm of
// B14229a7f0. The ECS lookup, 13-way snapshot, secondary rings and final nav
// publication remain separate producer stages; this file does not claim them.
// Native fixture evidence: show-{primary,special,wall-flat}-original-execution.json.
// Padding and unused enum payload bytes in the original are deliberately not
// interpreted as values. Every meaningful name/kind/variant/value/layer is kept.


#[derive(Clone, Copy, Debug)]
pub struct WallOptionsSource {
    pub shape_tag: u32,
    pub color: u32,
    pub freehand_is_initial: bool,
    pub flat_roof_rim_is_initial: Option<bool>,
}
#[derive(Clone, Copy, Debug)]
pub struct FlatOptionsSource {
    pub wall_color: u32,
    pub halftimber_color: u32,
    pub appearance: Option<(u8, u8, u32, u32)>,
}
#[derive(Clone, Copy, Debug)]
pub struct FlagOptionsSource {
    pub attachment_tag: u8,
    pub connected: bool,
}
#[derive(Clone, Copy, Debug)]
pub enum PrimaryOptionsSource {
    Wall {
        demo: bool,
        wall: Option<WallOptionsSource>,
    },
    Roof {
        demo: bool,
    },
    FlatRoof(FlatOptionsSource),
    Window {
        demo: bool,
        target_variant: u8,
        derived_kind: u8,
    },
    Wood {
        demo: bool,
    }, // original target5,8,12 invoke the same helper
    Flag {
        decorator: Option<FlagOptionsSource>,
    },
    Lantern,
    Chimney {
        derived_kind: u8,
        smoke: Option<bool>,
    },
    Stairs,
}

fn icon(name: &str, kind: u8, variant: u8, value: u32, extra: u32, layer: u32) -> IconInfo {
    IconInfo {
        name: name.into(),
        kind,
        b19: variant,
        u1a: 0,
        v1c: value,
        v20: extra,
        v24: layer,
    }
}
fn separator() -> IconInfo {
    icon("white", 5, 0, 0, 0, 0)
}
fn swatch(name: &str, value: u32) -> IconInfo {
    icon(name, 3, 0, value, 0, 0)
}
fn palette(data: &[(&str, u8, u8, u32, u32, u32)]) -> Vec<IconInfo> {
    data.iter()
        .map(|&(name, kind, variant, value, extra, layer)| {
            icon(name, kind, variant, value, extra, layer)
        })
        .collect()
}

/// None is the original whole-system early return when a non-demo wall target
/// is absent. It must not be replaced with an empty/spawned menu or another wall.
pub fn primary_options(source: PrimaryOptionsSource) -> Option<Vec<IconInfo>> {
    Some(match source {
        PrimaryOptionsSource::Wall { demo: true, .. } => palette(WALL_DEMO),
        PrimaryOptionsSource::Wall { demo: false, wall } => {
            let wall = wall?;
            let mut icons = palette(WALL_COLORS);
            if wall.color.wrapping_sub(4) < 7 || wall.color == 12 {
                icons.extend([
                    separator(),
                    icon("color_icon/plaster_peel_low", 12, 0, 0, 0, 0),
                    icon("color_icon/plaster_peel_medium", 12, 0, 0x3f000000, 0, 0),
                    icon("color_icon/plaster_peel_high", 12, 0, 0x3f800000, 0, 0),
                ]);
            }
            if wall.shape_tag < 2 {
                icons.extend([
                    separator(),
                    icon("color_icon/halftimber", 4, 2, 0, 0, 0),
                    icon("color_icon/skirt", 4, 1, 0, 0, 0),
                    icon("color_icon/no_skirt", 4, 0, 0, 0, 0),
                ]);
                if let Some(initial) = wall.flat_roof_rim_is_initial {
                    icons.push(separator());
                    icons.push(icon(
                        if initial {
                            "color_icon/freehand_ty_terrain"
                        } else {
                            "color_icon/freehand_ty_constant"
                        },
                        8,
                        0,
                        0,
                        0,
                        0,
                    ));
                }
            } else {
                icons.extend([
                    separator(),
                    icon("color_icon/skirt", 4, 1, 0, 0, 0),
                    icon("color_icon/no_skirt", 4, 0, 0, 0, 0),
                    separator(),
                    icon(
                        if wall.freehand_is_initial {
                            "color_icon/freehand_ty_terrain"
                        } else {
                            "color_icon/freehand_ty_constant"
                        },
                        7,
                        0,
                        0,
                        0,
                        0,
                    ),
                ]);
            }
            icons
        }
        PrimaryOptionsSource::Roof { demo } => palette(if demo { ROOF_DEMO } else { ROOF_COLORS }),
        PrimaryOptionsSource::Wood { demo } => palette(if demo { WOOD_DEMO } else { WOOD_COLORS }),
        PrimaryOptionsSource::Window {
            demo,
            target_variant,
            derived_kind,
        } => {
            let mut icons = palette(if demo { WOOD_DEMO } else { WOOD_COLORS });
            if target_variant != 0 && derived_kind.wrapping_sub(9) > 1 {
                icons.insert(0, swatch("color_icon/windows/no_color", 999));
            }
            icons
        }
        PrimaryOptionsSource::Flag { decorator } => {
            let mut icons = palette(FLAGS);
            if let Some(flag) = decorator.filter(|flag| !matches!(flag.attachment_tag, 2 | 4)) {
                icons.push(separator());
                icons.push(icon(
                    if flag.connected {
                        "color_icon/flag_connection"
                    } else {
                        "color_icon/flag_no_connection"
                    },
                    6,
                    0,
                    0,
                    0,
                    0,
                ));
            }
            icons
        }
        PrimaryOptionsSource::Lantern => palette(LANTERN),
        PrimaryOptionsSource::Stairs => palette(STAIRS),
        PrimaryOptionsSource::Chimney {
            derived_kind,
            smoke,
        } => {
            let mut icons = if derived_kind == 20 {
                Vec::new()
            } else {
                let mut colors = palette(&STAIRS[..5]);
                colors.push(separator());
                colors
            };
            icons.push(icon(
                if smoke.unwrap_or(true) {
                    "color_icon/chimney_smoke_active"
                } else {
                    "color_icon/chimney_smoke_inactive"
                },
                10,
                0,
                0,
                0,
                0,
            ));
            icons
        }
        PrimaryOptionsSource::FlatRoof(source) => {
            let (variant, stone_color, wood_color, floor) = match source.appearance {
                Some((2, 0, color, floor)) => (0, color, source.halftimber_color, floor),
                Some((2, variant, color, floor)) => (variant, source.wall_color, color, floor),
                _ => (0, source.wall_color, source.halftimber_color, 0),
            };
            let mut icons = if variant != 0 {
                palette(WOOD_COLORS)
            } else {
                palette(&WALL_COLORS[..5])
            };
            for option in &mut icons {
                option.kind = 2;
                option.b19 = u8::from(variant != 0);
                option.v20 = floor;
            }
            icons.push(separator());
            for (name, choice) in [
                ("stone_floor/icon", 0),
                ("stone_floor_2/icon", 1),
                ("stone_floor_3/icon", 2),
            ] {
                icons.push(icon(name, 2, 0, stone_color, choice, 0));
            }
            for (name, choice) in [
                ("wooden_floor/icon", 0),
                ("wooden_floor_3/icon", 2),
                ("wooden_floor_2/icon", 1),
            ] {
                icons.push(icon(name, 2, 1, wood_color, choice, 0));
            }
            icons
        }
    })
}

const ROOF_COLORS: &[(&str, u8, u8, u32, u32, u32)] = &[
    ("color_icon/roof_ochre", 3, 0, 8, 0, 0),
    ("color_icon/roof_red", 3, 0, 0, 0, 0),
    ("color_icon/roof_purple", 3, 0, 10, 0, 0),
    ("color_icon/roof_blue", 3, 0, 2, 0, 0),
    ("color_icon/roof_light_blue", 3, 0, 9, 0, 0),
    ("color_icon/roof_turquoise", 3, 0, 3, 0, 0),
    ("color_icon/roof_green_light", 3, 0, 6, 0, 0),
    ("color_icon/roof_green", 3, 0, 4, 0, 0),
    ("color_icon/roof_dark_wood", 3, 0, 1, 0, 0),
    ("color_icon/roof_dark", 3, 0, 5, 0, 0),
    ("color_icon/roof_grey", 3, 0, 7, 0, 0),
];

const ROOF_DEMO: &[(&str, u8, u8, u32, u32, u32)] = &[
    ("color_icon/roof_red", 3, 0, 0, 0, 0),
    ("color_icon/roof_green", 3, 0, 4, 0, 0),
    ("color_icon/locked", 3, 0, 777, 0, 0),
    ("color_icon/locked", 3, 0, 777, 0, 0),
    ("color_icon/locked", 3, 0, 777, 0, 0),
    ("color_icon/locked", 3, 0, 777, 0, 0),
    ("color_icon/locked", 3, 0, 777, 0, 0),
    ("color_icon/locked", 3, 0, 777, 0, 0),
    ("color_icon/locked", 3, 0, 777, 0, 0),
];

const WOOD_COLORS: &[(&str, u8, u8, u32, u32, u32)] = &[
    ("color_icon/windows/white", 3, 0, 4, 0, 0),
    ("color_icon/windows/wood", 3, 0, 0, 0, 0),
    ("color_icon/windows/dark_wood", 3, 0, 1, 0, 0),
    ("color_icon/roof_dark", 3, 0, 7, 0, 0),
    ("color_icon/roof_red", 3, 0, 3, 0, 0),
    ("color_icon/pink", 3, 0, 9, 0, 0),
    ("color_icon/windows/purple", 3, 0, 8, 0, 0),
    ("color_icon/roof_blue", 3, 0, 2, 0, 0),
    ("color_icon/roof_green_light", 3, 0, 6, 0, 0),
    ("color_icon/roof_green", 3, 0, 5, 0, 0),
];

const WOOD_DEMO: &[(&str, u8, u8, u32, u32, u32)] = &[
    ("color_icon/windows/wood", 3, 0, 0, 0, 0),
    ("color_icon/roof_red", 3, 0, 3, 0, 0),
    ("color_icon/roof_green", 3, 0, 5, 0, 0),
    ("color_icon/locked", 3, 0, 777, 0, 0),
    ("color_icon/locked", 3, 0, 777, 0, 0),
    ("color_icon/locked", 3, 0, 777, 0, 0),
    ("color_icon/locked", 3, 0, 777, 0, 0),
    ("color_icon/locked", 3, 0, 777, 0, 0),
    ("color_icon/locked", 3, 0, 777, 0, 0),
    ("color_icon/locked", 3, 0, 777, 0, 0),
];

const LANTERN: &[(&str, u8, u8, u32, u32, u32)] = &[
    ("color_icon/flag_yellow", 3, 0, 0, 0, 0),
    ("color_icon/windows/red", 3, 0, 4, 0, 0),
    ("color_icon/windows/purple", 3, 0, 3, 0, 0),
    ("color_icon/windows/blue", 3, 0, 1, 0, 0),
    ("color_icon/roof_green_light", 3, 0, 2, 0, 0),
    ("white", 5, 0, 0, 0, 0),
    ("color_icon/light_1", 9, 0, 0, 0, 0),
    ("color_icon/light_2", 9, 1, 0, 0, 0),
    ("color_icon/light_3", 9, 2, 0, 0, 0),
];

const STAIRS: &[(&str, u8, u8, u32, u32, u32)] = &[
    ("color_icon/bricks_beige", 3, 0, 100000, 0, 0),
    ("color_icon/bricks_yellow", 3, 0, 100001, 0, 0),
    ("color_icon/bricks_light", 3, 0, 100003, 0, 0),
    ("color_icon/bricks_grey", 3, 0, 100011, 0, 0),
    ("color_icon/bricks_dark", 3, 0, 100002, 0, 0),
    ("white", 5, 0, 0, 0, 0),
    ("color_icon/railing_1", 13, 0, 0, 0, 0),
    ("color_icon/railing_2", 13, 1, 0, 0, 0),
    ("color_icon/railing_3", 13, 2, 0, 0, 0),
    ("color_icon/railing_4", 13, 3, 0, 0, 0),
];

const WALL_COLORS: &[(&str, u8, u8, u32, u32, u32)] = &[
    ("color_icon/bricks_beige", 3, 0, 0, 0, 0),
    ("color_icon/bricks_yellow", 3, 0, 1, 0, 0),
    ("color_icon/bricks_light", 3, 0, 3, 0, 0),
    ("color_icon/bricks_grey", 3, 0, 11, 0, 0),
    ("color_icon/bricks_dark", 3, 0, 2, 0, 0),
    ("plaster_colors", 3, 0, 4, 0, 0),
    ("plaster_colors", 3, 0, 12, 0, 7),
    ("plaster_colors", 3, 0, 5, 0, 1),
    ("plaster_colors", 3, 0, 10, 0, 6),
    ("plaster_colors", 3, 0, 8, 0, 4),
    ("plaster_colors", 3, 0, 9, 0, 5),
    ("plaster_colors", 3, 0, 6, 0, 2),
    ("plaster_colors", 3, 0, 7, 0, 3),
];

const WALL_DEMO: &[(&str, u8, u8, u32, u32, u32)] = &[
    ("color_icon/bricks_grey", 3, 0, 3, 0, 0),
    ("plaster_colors", 3, 0, 5, 0, 1),
    ("plaster_colors", 3, 0, 8, 0, 4),
    ("color_icon/locked", 3, 0, 777, 0, 0),
    ("color_icon/locked", 3, 0, 777, 0, 0),
    ("color_icon/locked", 3, 0, 777, 0, 0),
    ("color_icon/locked", 3, 0, 777, 0, 0),
    ("color_icon/locked", 3, 0, 777, 0, 0),
    ("color_icon/locked", 3, 0, 777, 0, 0),
    ("color_icon/locked", 3, 0, 777, 0, 0),
    ("color_icon/locked", 3, 0, 777, 0, 0),
    ("color_icon/locked", 3, 0, 777, 0, 0),
    ("white", 5, 0, 0, 0, 0),
    ("color_icon/locked", 3, 0, 777, 0, 0),
    ("color_icon/skirt", 4, 1, 0, 0, 0),
    ("color_icon/no_skirt", 4, 0, 0, 0, 0),
];

const FLAGS: &[(&str, u8, u8, u32, u32, u32)] = &[
    ("color_icon/flags/red", 3, 0, 1, 0, 0),
    ("color_icon/flags/pink", 3, 0, 7, 0, 0),
    ("color_icon/flags/yellow", 3, 0, 3, 0, 0),
    ("color_icon/flags/green", 3, 0, 2, 0, 0),
    ("color_icon/flags/dark_sea", 3, 0, 9, 0, 0),
    ("color_icon/flags/blue", 3, 0, 0, 0, 0),
    ("color_icon/flags/light_purple", 3, 0, 6, 0, 0),
    ("color_icon/flags/purple", 3, 0, 4, 0, 0),
    ("color_icon/flags/dark_bourbon", 3, 0, 8, 0, 0),
    ("color_icon/flags/dark", 3, 0, 5, 0, 0),
];

// Candidate for the complete 13-way initial selection policy in B14229a7f0.
// Real upstream resource lookups must supply these values. Required wall,
// roof and decorator lookups retain their original failure behavior there;
// optional fields below correspond only to proven native optional branches.
use crate::radial_menu::ColorSelectionSnapshot;

#[derive(Clone, Copy, Debug)]
pub enum InitialSelectionSource {
    Wall {
        color: u32,
        wall_style: u8,
        plaster_peel: f32,
        freehand_ty: u8,
        flat_rim_style: Option<u8>,
    },
    Roof { color: u32 },
    FlatRoof { color: u32, appearance: [u8; 12] },
    Decorator { color: u32 },
    Gate { color: u32 },
    Flag { color: u32, connected: Option<bool> },
    Halftimber { color: u32 },
    Lantern { color: u32, brightness: Option<f32> },
    Chimney { color: u32, smoke: Option<bool> },
    Stairs { color: u32, extra: Option<(u8, u32)> },
    Ladder { color: u32 },
}

pub fn initial_selection(source: InitialSelectionSource) -> ColorSelectionSnapshot {
    let mut initial = ColorSelectionSnapshot {
        brightness: 1.0, // XMM14 = original f32 constant142925904
        ..Default::default()
    };
    initial.color = match source {
        InitialSelectionSource::Wall {
            color,
            wall_style,
            plaster_peel,
            freehand_ty,
            flat_rim_style,
        } => {
            initial.wall_style = wall_style;
            initial.plaster_peel = plaster_peel;
            initial.freehand_ty = freehand_ty;
            initial.flat_roof_rim_style = flat_rim_style.unwrap_or(0);
            color
        }
        InitialSelectionSource::Roof { color }
        | InitialSelectionSource::Decorator { color }
        | InitialSelectionSource::Gate { color }
        | InitialSelectionSource::Halftimber { color }
        | InitialSelectionSource::Ladder { color } => color,
        InitialSelectionSource::FlatRoof { color, appearance } => {
            initial.flat_roof_style = appearance;
            color
        }
        InitialSelectionSource::Flag { color, connected } => {
            initial.flag_connection = u8::from(connected.unwrap_or(true));
            color
        }
        InitialSelectionSource::Lantern { color, brightness } => {
            initial.brightness = brightness.unwrap_or(1.0);
            color
        }
        InitialSelectionSource::Chimney { color, smoke } => {
            initial.chimney_smoke = u8::from(smoke.unwrap_or(true));
            color
        }
        InitialSelectionSource::Stairs { color, extra } => {
            if let Some((railing, magic)) = extra {
                initial.railing = railing;
                initial.magic_color = magic;
            }
            color
        }
    };
    initial
}

/// Resolve the wall palette and cancellation snapshot from the same wall ID.
/// A missing wall is the native primary-options early return. Color indexing
/// and the associated roof lookup retain the production owners' checks.
fn wall_menu_inputs(
    id: country_core::resources::walls::WallId,
    walls: &country_core::resources::walls::public_walls::PublicWalls,
    roofs: &Query<&country_core::resources::roofs::roof_shape::Roof>,
    colors: &country_core::resources::color_id::WallColorIds,
    demo: bool,
) -> Option<(Vec<IconInfo>, ColorSelectionSnapshot)> {
    let wall = walls.get(id)?;
    let color = colors.0[id.0 as usize];
    let roof = walls.roof_component(id, |entity| roofs.get(entity).ok());
    let rim = roof.map(|roof| roof.observed()[0x3d]);
    let source = WallOptionsSource {
        shape_tag: wall.shape().tag(),
        color,
        freehand_is_initial: wall.authority.freehand_ty() == 0,
        flat_roof_rim_is_initial: roof.filter(|roof| roof.is_flat()).map(|roof| roof.observed()[0x3d] == 0),
    };
    let options = primary_options(PrimaryOptionsSource::Wall { demo, wall: Some(source) })?;
    let initial = initial_selection(InitialSelectionSource::Wall {
        color,
        wall_style: wall.authority.style(),
        plaster_peel: wall.authority.plaster_peel_amount(),
        freehand_ty: wall.authority.freehand_ty(),
        flat_rim_style: rim,
    });
    Some((options, initial))
}

/// All native cancellation writers, kept in one SystemParam to preserve the
/// original event ownership while avoiding a flattened function parameter list.
#[derive(bevy_ecs::system::SystemParam)]
pub struct ColorMenuRestoreEvents<'w> {
    pub walls: EventWriter<'w, country_core::resources::color_id::SetWallColorIdCmd>,
    pub roofs: EventWriter<'w, crate::SetRoofColorIdCmd>,
    pub decorators: EventWriter<'w, crate::SetDecoratorColorIdCmd>,
    pub ladders: EventWriter<'w, crate::SetLadderColorIdCmd>,
    pub brightness: EventWriter<'w, crate::SetDecoratorBrightnessCmd>,
    pub gates: EventWriter<'w, country_core::resources::color_id::SetGateColorIdCmd>,
    pub halftimber: EventWriter<'w, country_core::resources::color_id::SetHalftimberColorIdCmd>,
    pub styles: EventWriter<'w, country_core::systems::wall::SetWallStyleCmd>,
    pub flags: EventWriter<'w, crate::SetFlagConnectionCmd>,
    pub window_lights: EventWriter<'w, crate::SetWindowLightCmd>,
    pub rims: EventWriter<'w, country_core::resources::roofs::commands::SetFlatRoofRimStyleCmd>,
    pub chimneys: EventWriter<'w, crate::SetChimneySmokeCmd>,
    pub magic: EventWriter<'w, crate::SetMagicColorIdCmd>,
    pub flat_styles: EventWriter<'w, crate::SetFlatRoofStyleCmd>,
    pub plaster: EventWriter<'w, country_core::systems::wall::SetPlasterPeelAmountCmd>,
}
impl ColorMenuRestoreEvents<'_> {
    /// B1422ae3e4 jump table0x14303cad8, targets0..12. The stored initial
    /// snapshot is restored on cancel; current preview values are not committed.
    pub fn restore(&mut self, menu: &ColorRadialMenu) {
        use country_core::resources::roofs::customization::{
            FlatRoofRimRequest, FlatRoofStyleObserved, FlatRoofStyleRequest,
        };
        use country_core::resources::walls::WallId;
        let target = menu.target;
        let initial = menu.initial;
        let wall = WallId(target.id);
        let id = target.payload;
        match target.kind {
            0 => {
                self.walls
                    .write(country_core::resources::color_id::SetWallColorIdCmd {
                        id: target.id as u32,
                        _gap4: (target.id >> 32) as u32,
                        color: initial.color,
                        custom: 0,
                        _pad: [0; 3],
                    });
                self.styles
                    .write(country_core::systems::wall::SetWallStyleCmd {
                        wall_id: wall,
                        from: initial.wall_style,
                        to: initial.wall_style,
                        record_history: 0,
                        play_sound: 0,
                    });
                self.rims.write(
                    country_core::resources::roofs::commands::SetFlatRoofRimStyleCmd(
                        FlatRoofRimRequest {
                            wall_id: country_core::resources::walls::publication_fields::WallId(
                                target.id,
                            ),
                            from: initial.flat_roof_rim_style,
                            to: initial.flat_roof_rim_style,
                            observed_record_history: 0,
                        },
                    ),
                );
                self.plaster
                    .write(country_core::systems::wall::SetPlasterPeelAmountCmd {
                        wall_id: wall,
                        from: initial.plaster_peel,
                        to: initial.plaster_peel,
                        record_history: 0,
                    });
            }
            1 => {
                self.roofs.write(crate::SetRoofColorIdCmd {
                    id: target.id,
                    color: initial.color,
                    custom: 0,
                });
            }
            2 => {
                self.flat_styles
                    .write(crate::SetFlatRoofStyleCmd(FlatRoofStyleRequest {
                        wall_id: country_core::resources::walls::publication_fields::WallId(
                            target.id,
                        ),
                        style: FlatRoofStyleObserved::from_observed(initial.flat_roof_style),
                        observed_play_sound: 0,
                    }));
            }
            3 => {
                self.decorators.write(crate::SetDecoratorColorIdCmd {
                    id,
                    color: initial.color,
                    custom: 0,
                });
            }
            4 => {
                self.window_lights.write(crate::SetWindowLightCmd {
                    id,
                    value: initial.color,
                    custom: 0,
                });
            }
            5 => {
                self.gates
                    .write(country_core::resources::color_id::SetGateColorIdCmd {
                        id: target.id as u32,
                        _gap4: (target.id >> 32) as u32,
                        color: initial.color,
                        custom: 0,
                        _pad: [0; 3],
                    });
            }
            6 => {
                self.decorators.write(crate::SetDecoratorColorIdCmd {
                    id,
                    color: initial.color,
                    custom: 0,
                });
                self.flags.write(crate::SetFlagConnectionCmd {
                    id,
                    value: initial.flag_connection,
                    custom: 0,
                    _pad: [0; 2],
                });
            }
            9 => {
                self.decorators.write(crate::SetDecoratorColorIdCmd {
                    id,
                    color: initial.color,
                    custom: 0,
                });
                self.brightness.write(crate::SetDecoratorBrightnessCmd {
                    id,
                    value: initial.brightness.to_bits(),
                    custom: 0,
                });
            }
            10 => {
                self.decorators.write(crate::SetDecoratorColorIdCmd {
                    id,
                    color: initial.color,
                    custom: 0,
                });
                self.chimneys.write(crate::SetChimneySmokeCmd {
                    id,
                    value: initial.chimney_smoke,
                    custom: 0,
                    _pad: [0; 2],
                });
            }
            11 | 12 => {
                if target.kind == 11 {
                    self.decorators.write(crate::SetDecoratorColorIdCmd {
                        id,
                        color: initial.color,
                        custom: 0,
                    });
                } else {
                    self.ladders.write(crate::SetLadderColorIdCmd {
                        id,
                        value: initial.color,
                        custom: 0,
                    });
                }
                self.magic.write(crate::SetMagicColorIdCmd {
                    id,
                    value: initial.magic_color,
                    custom: 0,
                });
            }
            _ => {
                self.halftimber
                    .write(country_core::resources::color_id::SetHalftimberColorIdCmd {
                        id: target.id as u32,
                        _gap4: (target.id >> 32) as u32,
                        color: initial.color,
                        custom: 0,
                        _pad: [0; 3],
                    });
            }
        }
    }
}

/// B1422adcf0. The latest command decides cancellation; keyboard ui_cancel18
/// supplies true only when there is no command. Menu despawn is immediate, while
/// icon entities retain their original animated deletion lifetime.
#[allow(clippy::too_many_arguments)]
pub fn hide_color_menu(
    mut cmds: EventReader<HideColorMenuCmd>,
    mut sounds: EventWriter<country_core::systems::ui::resources::widget_params::PlayUiSound>,
    menus: Query<(Entity, &ColorRadialMenu)>,
    icons: Query<(Entity, &CustomizationIcon)>,
    actions: Res<country_core::input::action_set::ActionSetStates>,
    mut commands: Commands,
    mut restoration: ColorMenuRestoreEvents,
) {
    let cancel = match cmds.read().last() {
        Some(cmd) => cmd.cancel,
        None if actions.digital[18].just_pressed() => true,
        None => return,
    };
    for (entity, menu) in &menus {
        if cancel {
            restoration.restore(menu);
            sounds.write(
                country_core::systems::ui::resources::widget_params::PlayUiSound {
                    kind: 9,
                    ..Default::default()
                },
            );
        }
        commands.entity(entity).despawn();
    }
    for (entity, _) in &icons {
        commands
            .entity(entity)
            .insert(CustomizationIconMarkedForDeletion);
    }
}

/// Show a menu from the authority associated with the command target.
#[allow(clippy::too_many_arguments)]
pub fn show_color_options<S: Send + Sync + 'static>(
    mut cmds: EventReader<ShowColorMenuCmd>,
    wall_colors: Res<country_core::resources::color_id::WallColorIds>,
    gate_colors: Res<country_core::resources::color_id::GateColorIds>,
    half_colors: Res<country_core::resources::color_id::HalftimberColorIds>,
    roofs: Query<&country_core::resources::roofs::roof_shape::Roof>,
    decorators: Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,
    walls: Res<country_core::resources::walls::public_walls::PublicWalls>,
    config: Res<country_core::game_config::GameConfig>,
    demo: Res<country_core::DevOverrideIsDemoMode>,
    subtypes: Res<country_core::resources::walls::decorator_subtype::DerivedDecoratorSubtypes>,
    stair_visuals: Res<country_core::resources::stairs::visual::StairsVisualState>,
    stairs: Res<country_core::resources::stairs::stairs_state::StairsState>,
    camera: Res<country_core::systems::camera_rig::main_camera::MainCamera>,
    mut nav: ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,
    mut commands: Commands,
) {
    use country_core::resources::walls::{WallId, decorator::DecoratorId};
    use crate::radial_menu::{ColorMenuTarget, RadialMenuCreate};
    for cmd in cmds.read() {
        let id = WallId(cmd.payload);
        let did = DecoratorId(cmd.decorator_id);
        let info = decorators.lookup_decorator_info(did);
        let roof = || walls.roof_component(id, |entity| roofs.get(entity).ok())
            .expect("color menu target requires a roof");
        let index = id.0 as usize;
        let (options, initial) = match cmd.kind {
            0 => match wall_menu_inputs(id, &walls, &roofs, &wall_colors, demo.0) {
                Some(value) => value,
                None if !demo.0 => return,
                None => panic!("color menu target requires a wall"),
            },
            1 => {
                let roof = roof();
                let color = u32::from_le_bytes(roof.observed()[0x50..0x54].try_into().unwrap());
                (primary_options(PrimaryOptionsSource::Roof { demo: demo.0 }).unwrap(),
                 initial_selection(InitialSelectionSource::Roof { color }))
            }
            2 => {
                let roof = roof();
                let raw: [u8; 12] = roof.observed()[0x40..0x4c].try_into().unwrap();
                let color = u32::from_le_bytes(roof.observed()[0x50..0x54].try_into().unwrap());
                let appearance = Some((raw[0], raw[1], u32::from_le_bytes(raw[4..8].try_into().unwrap()), u32::from_le_bytes(raw[8..12].try_into().unwrap())));
                let source = FlatOptionsSource { wall_color: wall_colors.0[index], halftimber_color: half_colors.0[index], appearance };
                (primary_options(PrimaryOptionsSource::FlatRoof(source)).unwrap(),
                 initial_selection(InitialSelectionSource::FlatRoof { color, appearance: raw }))
            }
            5 | 8 => {
                let color = if cmd.kind == 5 { gate_colors.0[index] } else { half_colors.0[index] };
                (primary_options(PrimaryOptionsSource::Wood { demo: demo.0 }).unwrap(),
                 initial_selection(if cmd.kind == 5 { InitialSelectionSource::Gate { color } } else { InitialSelectionSource::Halftimber { color } }))
            }
            3 | 6 | 9 | 10 | 11 | 12 => {
                let value = info.expect("color menu target requires a decorator");
                let color = value.u28;
                let derived = if matches!(cmd.kind, 3 | 10) { subtypes.get(cmd.decorator_id).0 } else { 0 };
                let flag = (value.extra_disc() == 5).then(|| value.extra().as_flag().0 != 0);
                let smoke = (value.extra_disc() == 4).then(|| value.extra().as_chimney().smoke != 0);
                let brightness = (value.extra_disc() == 7).then(|| value.extra().as_lantern().0);
                let stair = (value.extra_disc() <= 1).then(|| value.extra().as_stair());
                let stair_values = stair.map(|v| (v.0[16], u32::from_le_bytes(v.0[12..16].try_into().unwrap())));
                let (source, snapshot) = match cmd.kind {
                    3 => (PrimaryOptionsSource::Window { demo: demo.0, target_variant: cmd.subtype, derived_kind: derived }, InitialSelectionSource::Decorator { color }),
                    6 => (PrimaryOptionsSource::Flag { decorator: Some(FlagOptionsSource { attachment_tag: value.dst_tag(), connected: flag.expect("flag extra required") }) }, InitialSelectionSource::Flag { color, connected: flag }),
                    9 => (PrimaryOptionsSource::Lantern, InitialSelectionSource::Lantern { color, brightness }),
                    10 => (PrimaryOptionsSource::Chimney { derived_kind: derived, smoke }, InitialSelectionSource::Chimney { color, smoke }),
                    11 => (PrimaryOptionsSource::Stairs, InitialSelectionSource::Stairs { color, extra: stair_values }),
                    12 => (PrimaryOptionsSource::Wood { demo: demo.0 }, InitialSelectionSource::Ladder { color: u32::from_le_bytes(stair.expect("stair extra required").0[8..12].try_into().unwrap()) }),
                    _ => unreachable!(),
                };
                (primary_options(source).unwrap(), initial_selection(snapshot))
            }
            4 | 7 => panic!("secondary color target cannot be shown directly"),
            _ => panic!("invalid color target {}", cmd.kind),
        };
        let icon_size = 48.0 * config.ui.icon_size_mult;
        let spacing = 12.0 * config.ui.icon_size_mult * if cmd.kind == 0 { 0.5 } else { 1.0 };
        let radius = ((options.len() as f32 * (icon_size + spacing)) / f32::from_bits(0x4071463b)).max(60.0);
        let target = ColorMenuTarget { kind: cmd.kind, subtype: cmd.subtype, payload: cmd.decorator_id, id: cmd.payload };
        let (_, menu) = ColorRadialMenu::spawn(RadialMenuCreate {
            center: cmd.position, radius, start_angle: 0.0, icon_size, spacing,
            appearance_divisor: if cmd.kind == 0 { 1.5 } else { 1.0 },
            icons: options, target, initial, world_anchor: camera.eye_position(),
        }, &mut commands);
        let mut rings = vec![menu];
        if cmd.kind == 0 && walls.get(id).is_some_and(|wall| wall.authority.style() >= 2) {
            let secondary = primary_options(PrimaryOptionsSource::Wood { demo: demo.0 }).unwrap();
            let initial = initial_selection(InitialSelectionSource::Halftimber { color: half_colors.0[index] });
            let angle = if walls.roof_component(id, |entity| roofs.get(entity).ok()).is_some_and(|roof| roof.is_flat()) {
                f32::from_bits(0x3e7ae145)
            } else {
                f32::from_bits(0x3ec51eb7)
            };
            let (_, secondary_menu) = ColorRadialMenu::spawn(RadialMenuCreate {
                center: cmd.position, radius: radius + icon_size * 1.1, start_angle: angle,
                icon_size: icon_size * 0.8, spacing, appearance_divisor: 1.5,
                icons: secondary, target: ColorMenuTarget { kind: 7, ..target },
                initial, world_anchor: camera.eye_position(),
            }, &mut commands);
            rings.push(secondary_menu);
        }
        if cmd.kind == 3 {
            let subtype = subtypes.get(cmd.decorator_id).0;
            if !(subtype < 11 && (0x630u32 & (1u32 << subtype)) != 0) {
                let window = info.expect("window light menu requires its decorator").window_extra();
                let icons = LANTERN[..5].iter().map(|&(name, kind, variant, value, extra, layer)|
                    icon(name, kind, variant, value, extra, layer)).collect();
                let (_, secondary_menu) = ColorRadialMenu::spawn(RadialMenuCreate {
                    center: cmd.position, radius: radius + icon_size * 1.1,
                    start_angle: f32::from_bits(0x3ec51eb7), icon_size: icon_size * 0.8,
                    spacing, appearance_divisor: 1.0, icons,
                    target: ColorMenuTarget { kind: 4, ..target },
                    initial: initial_selection(InitialSelectionSource::Decorator { color: window.0 }),
                    world_anchor: camera.eye_position(),
                }, &mut commands);
                rings.push(secondary_menu);
            }
        }
        if matches!(cmd.kind, 11 | 12) {
            if let Some(extra) = info.and_then(|value| {
                (value.extra_disc() <= 1).then(|| value.extra().as_stair())
            }) {
                let connected = stairs.get_all_connected_nodes(cmd.decorator_id);
                let has_magic = stair_visuals.edges().iter().any(|(edge, visual)| {
                    let (a, b) = edge.nodes();
                    visual.magic_supports.has_magic() && (connected.contains(&a) || connected.contains(&b))
                });
                let ladder_color = u32::from_le_bytes(extra.0[8..12].try_into().unwrap());
                let magic_color = u32::from_le_bytes(extra.0[12..16].try_into().unwrap());
                let railing = extra.0[16];
                let magic_icons = || [
                    "color_icon/plaster_peel_low", "color_icon/windows/blue",
                    "color_icon/windows/purple", "color_icon/roof_ochre",
                    "color_icon/roof_green_light", "color_icon/roof_dark",
                ].into_iter().enumerate().map(|(value, name)| icon(name, 14, 0, value as u32, 0, 0)).collect::<Vec<_>>();
                if cmd.kind == 12 && has_magic {
                    let snapshot = initial_selection(InitialSelectionSource::Stairs {
                        color: info.unwrap().u28, extra: Some((railing, magic_color)),
                    });
                    let (_, secondary_menu) = ColorRadialMenu::spawn(RadialMenuCreate {
                        center: cmd.position, radius: radius + icon_size * 1.1,
                        start_angle: f32::from_bits(0x3ec51eb7), icon_size: icon_size * 0.8,
                        spacing, appearance_divisor: 1.0, icons: magic_icons(),
                        target: ColorMenuTarget { kind: 11, ..target }, initial: snapshot,
                        world_anchor: camera.eye_position(),
                    }, &mut commands);
                    rings.push(secondary_menu);
                } else if cmd.kind == 11 && (railing == 3 || has_magic) {
                    let mut icons = if has_magic { magic_icons() } else { Vec::new() };
                    if railing == 3 {
                        if has_magic { icons.push(separator()); }
                        icons.extend(primary_options(PrimaryOptionsSource::Wood { demo: demo.0 }).unwrap());
                    }
                    let mut snapshot = initial_selection(InitialSelectionSource::Ladder { color: ladder_color });
                    snapshot.magic_color = magic_color;
                    let (_, secondary_menu) = ColorRadialMenu::spawn(RadialMenuCreate {
                        center: cmd.position, radius: radius + icon_size * 1.1,
                        start_angle: f32::from_bits(0x3ec51eb7), icon_size: icon_size * 0.8,
                        spacing, appearance_divisor: 1.0, icons,
                        target: ColorMenuTarget { kind: 12, ..target }, initial: snapshot,
                        world_anchor: camera.eye_position(),
                    }, &mut commands);
                    rings.push(secondary_menu);
                }
            }
        }
        for index in 0..rings.len() {
            let next = rings[(index + 1) % rings.len()].to_bits();
            let previous = rings[(index + rings.len() - 1) % rings.len()].to_bits();
            let mut links = country_core::components::UiNav::vertical(previous, next);
            links.links[0].tag = 2;
            links.links[1].tag = 2;
            commands.entity(rings[index]).insert(links);
        }
        nav.focus = country_core::systems::ui::resources::widget_params::UiFocus { tag: 1, _pad: 0, payload: menu.to_bits() };
    }
}
#[cfg(test)]
mod native_hide_tests {
    use super::*;
    use crate::radial_menu::{ColorMenuTarget, ColorSelectionSnapshot};
    use country_core::resources::color_id::*;
    use country_core::systems::ui::resources::widget_params::PlayUiSound;
    fn setup() -> World {
        let mut world = World::new();
        crate::register_color_events(&mut world);
        world.init_resource::<country_core::input::action_set::ActionSetStates>();
        world
    }
    #[test]
    fn show_uses_target_color_and_real_screen_position() {
        use country_core::resources::{color_id::*, walls::{WallId, public_walls::PublicWalls, decorator_storage::DecoratorStorage, decorator_subtype::DerivedDecoratorSubtypes}};
        use country_core::systems::{camera_rig::main_camera::MainCamera, ui::resources::widget_params::UiNavState};
        let mut world = setup();
        world.init_resource::<PublicWalls>();
        world.init_resource::<DecoratorStorage>();
        world.init_resource::<DerivedDecoratorSubtypes>();
        world.init_resource::<country_core::DevOverrideIsDemoMode>();
        world.init_resource::<country_core::resources::stairs::visual::StairsVisualState>();
        world.init_resource::<country_core::resources::stairs::stairs_state::StairsState>();
        world.init_resource::<country_core::game_config::GameConfig>();
        world.init_resource::<MainCamera>();
        world.init_resource::<UiNavState>();
        world.resource_mut::<GateColorIds>().0[3] = 4;
        world.resource_mut::<HalftimberColorIds>().0[3] = 5;
        let roof = world.spawn(country_core::resources::roofs::roof_shape::Roof::new(
            3u64.to_le_bytes(), [0; 28], [0; 28], [0; 4], 6u32.to_le_bytes(), 0, [0; 12])).id();
        world.resource_mut::<PublicWalls>().associate_roof(WallId(3), roof);
        for kind in [1, 5, 8] {
            world.send_event(ShowColorMenuCmd { kind, subtype: 0, decorator_id: 0, payload: 3, position: glam::Vec2::new(321., 123.) });
        }
        let mut schedule = Schedule::default();
        schedule.add_systems(show_color_options::<()>);
        schedule.run(&mut world);
        let mut query = world.query::<&ColorRadialMenu>();
        let menus: Vec<_> = query.iter(&world).collect();
        assert_eq!(menus.len(), 3);
        for (kind, color) in [(1, 6), (5, 4), (8, 5)] {
            let menu = menus.iter().find(|menu| menu.target.kind == kind).unwrap();
            assert_eq!(menu.initial.color, color);
            assert_eq!(menu.center, glam::Vec2::new(321., 123.));
            assert!(!menu.icon_entities.is_empty());
            assert!(menu.radius >= 60.0);
        }
    }
    #[test]
    fn window_light_ring_uses_the_live_window_extra() {
        use country_core::resources::walls::{decorator::{DecoratorId, DecoratorInfo, DecoratorExtraInfo, WindowExtra}, decorator_storage::{DecoratorStorage, DecoratorStorageAddr}, decorator_subtype::{DecoratorSubtype, DerivedDecoratorSubtypes}, public_walls::PublicWalls};
        use country_core::systems::{camera_rig::main_camera::MainCamera, ui::resources::widget_params::UiNavState};
        let mut world = setup();
        world.init_resource::<PublicWalls>();
        world.init_resource::<DecoratorStorage>();
        world.init_resource::<DerivedDecoratorSubtypes>();
        world.init_resource::<country_core::DevOverrideIsDemoMode>();
        world.init_resource::<country_core::resources::stairs::visual::StairsVisualState>();
        world.init_resource::<country_core::resources::stairs::stairs_state::StairsState>();
        world.init_resource::<country_core::game_config::GameConfig>();
        world.init_resource::<MainCamera>();
        world.init_resource::<UiNavState>();
        let mut info = DecoratorInfo { u28: 8, ..Default::default() };
        info.set_extra(DecoratorExtraInfo::Window(WindowExtra(3)));
        world.resource_mut::<DecoratorStorage>().apply_decorator_place(DecoratorStorageAddr::Terrain, DecoratorId(41), &info);
        world.resource_mut::<DerivedDecoratorSubtypes>().insert(41, DecoratorSubtype(0));
        world.send_event(ShowColorMenuCmd { kind: 3, subtype: 0, decorator_id: 41, payload: 0, position: glam::Vec2::new(20., 30.) });
        let mut schedule = Schedule::default();
        schedule.add_systems(show_color_options::<()>);
        schedule.run(&mut world);
        let mut query = world.query::<&ColorRadialMenu>();
        let menus: Vec<_> = query.iter(&world).collect();
        assert_eq!(menus.len(), 2);
        let light = menus.iter().find(|menu| menu.target.kind == 4).unwrap();
        assert_eq!(light.initial.color, 3);
        assert_eq!(light.icons.len(), 5);
        assert_eq!(light.target.payload, 41);
        drop(menus);
        let mut nav_query = world.query::<(&ColorRadialMenu, &country_core::components::UiNav)>();
        let navs: Vec<_> = nav_query.iter(&world).collect();
        assert_eq!(navs.len(), 2);
        assert!(navs.iter().all(|(_, nav)| nav.links.map(|link| link.tag) == [2, 2, 3, 3]));
        drop(navs);
        world.resource_mut::<DerivedDecoratorSubtypes>().insert(41, DecoratorSubtype(4));
        world.send_event(ShowColorMenuCmd { kind: 3, subtype: 0, decorator_id: 41, payload: 0, position: glam::Vec2::ZERO });
        schedule.run(&mut world);
        let mut query = world.query::<&ColorRadialMenu>();
        assert_eq!(query.iter(&world).filter(|menu| menu.target.kind == 4).count(), 1,
            "excluded window subtype must not spawn another light ring");
    }
    #[test]
    fn stair_ring_uses_connected_visual_activity_and_railing() {
        use country_core::resources::{stairs::{stairs_state::{StairGraphEdge, StairsState}, visual::StairsVisualState}, walls::{decorator::{DecoratorId, DecoratorInfo, DecoratorExtraInfo, StairExtra}, decorator_storage::{DecoratorStorage, DecoratorStorageAddr}, decorator_subtype::DerivedDecoratorSubtypes, public_walls::PublicWalls}};
        use country_core::systems::{camera_rig::main_camera::MainCamera, ui::resources::widget_params::UiNavState};
        let mut world = setup();
        world.init_resource::<PublicWalls>();
        world.init_resource::<DecoratorStorage>();
        world.init_resource::<DerivedDecoratorSubtypes>();
        world.init_resource::<country_core::DevOverrideIsDemoMode>();
        world.init_resource::<country_core::game_config::GameConfig>();
        world.init_resource::<MainCamera>();
        world.init_resource::<UiNavState>();
        world.init_resource::<StairsVisualState>();
        world.init_resource::<StairsState>();
        let mut extra = [0u8; 20];
        extra[8..12].copy_from_slice(&4u32.to_le_bytes());
        extra[12..16].copy_from_slice(&2u32.to_le_bytes());
        extra[16] = 3;
        let mut info = DecoratorInfo { u28: 100003, ..Default::default() };
        info.set_extra(DecoratorExtraInfo::Stair(StairExtra(extra)));
        world.resource_mut::<DecoratorStorage>().apply_decorator_place(DecoratorStorageAddr::Stair, DecoratorId(41), &info);
        world.resource_mut::<StairsState>().add_edge(41, 42);
        let edge = StairGraphEdge::new(41, 42);
        world.resource_mut::<StairsVisualState>().reset_edge(edge);
        let mut schedule = Schedule::default();
        schedule.add_systems(show_color_options::<()>);
        let cmd = ShowColorMenuCmd { kind: 11, subtype: 0, decorator_id: 41, payload: 0, position: glam::Vec2::ZERO };
        world.send_event(cmd);
        schedule.run(&mut world);
        let mut query = world.query::<&ColorRadialMenu>();
        let first = query.iter(&world).find(|menu| menu.target.kind == 12).unwrap();
        assert_eq!(first.icons.len(), 10);
        assert_eq!(first.initial.color, 4);
        world.resource_mut::<StairsVisualState>().get_mut(&edge).unwrap().magic_supports.set_activity_byte(7);
        world.send_event(cmd);
        schedule.run(&mut world);
        let mut query = world.query::<&ColorRadialMenu>();
        let combined = query.iter(&world).find(|menu| menu.target.kind == 12 && menu.icons.len() == 17).unwrap();
        assert_eq!(combined.initial.magic_color, 2);
        assert_eq!(combined.icons[6].kind, 5);
        world.send_event(ShowColorMenuCmd { kind: 12, ..cmd });
        schedule.run(&mut world);
        let mut query = world.query::<&ColorRadialMenu>();
        let magic = query.iter(&world).find(|menu| menu.target.kind == 11 && menu.icons.len() == 6).unwrap();
        assert_eq!(magic.initial.color, 100003);
        assert_eq!(magic.initial.magic_color, 2);
    }
    #[test]
    fn cancellation_restores_every_native_target_then_keeps_exit_animation() {
        let mut world = setup();
        let icon = world
            .spawn(CustomizationIcon {
                alpha_18: 1.,
                ..Default::default()
            })
            .id();
        for kind in 0..13 {
            world.spawn(ColorRadialMenu {
                target: ColorMenuTarget {
                    kind,
                    subtype: 0,
                    payload: 67,
                    id: 0x100000009,
                },
                initial: ColorSelectionSnapshot {
                    color: 5,
                    brightness: 4.,
                    plaster_peel: 0.5,
                    magic_color: 8,
                    wall_style: 2,
                    flat_roof_rim_style: 1,
                    flag_connection: 1,
                    chimney_smoke: 1,
                    ..Default::default()
                },
                ..Default::default()
            });
        }
        world
            .resource_mut::<Events<HideColorMenuCmd>>()
            .send(HideColorMenuCmd { cancel: true });
        let mut schedule = Schedule::default();
        schedule.add_systems(hide_color_menu);
        schedule.run(&mut world);
        assert_eq!(world.query::<&ColorRadialMenu>().iter(&world).count(), 0);
        assert!(
            world
                .get::<CustomizationIconMarkedForDeletion>(icon)
                .is_some()
        );
        assert_eq!(world.get::<CustomizationIcon>(icon).unwrap().alpha_18, 1.);
        assert_eq!(world.resource::<Events<PlayUiSound>>().len(), 13);
        assert_eq!(world.resource::<Events<SetWallColorIdCmd>>().len(), 1);
        assert_eq!(
            world.resource::<Events<crate::SetRoofColorIdCmd>>().len(),
            1
        );
        assert_eq!(
            world.resource::<Events<crate::SetFlatRoofStyleCmd>>().len(),
            1
        );
        assert_eq!(
            world
                .resource::<Events<crate::SetDecoratorColorIdCmd>>()
                .len(),
            5
        );
        assert_eq!(
            world.resource::<Events<crate::SetWindowLightCmd>>().len(),
            1
        );
        assert_eq!(world.resource::<Events<SetGateColorIdCmd>>().len(), 1);
        assert_eq!(world.resource::<Events<SetHalftimberColorIdCmd>>().len(), 2);
        assert_eq!(
            world
                .resource::<Events<crate::SetFlagConnectionCmd>>()
                .len(),
            1
        );
        assert_eq!(
            world
                .resource::<Events<crate::SetDecoratorBrightnessCmd>>()
                .len(),
            1
        );
        assert_eq!(
            world.resource::<Events<crate::SetChimneySmokeCmd>>().len(),
            1
        );
        assert_eq!(
            world.resource::<Events<crate::SetMagicColorIdCmd>>().len(),
            2
        );
        assert_eq!(
            world.resource::<Events<crate::SetLadderColorIdCmd>>().len(),
            1
        );
        assert_eq!(
            world
                .resource::<Events<country_core::systems::wall::SetWallStyleCmd>>()
                .len(),
            1
        );
        assert_eq!(
            world
                .resource::<Events<country_core::systems::wall::SetPlasterPeelAmountCmd>>()
                .len(),
            1
        );
        assert_eq!(world.resource::<Events<country_core::resources::roofs::commands::SetFlatRoofRimStyleCmd>>().len(), 1);
        let wall = world
            .resource::<Events<SetWallColorIdCmd>>()
            .iter_current_update_events()
            .next()
            .unwrap();
        assert_eq!(wall.wall_id().0, 0x100000009);
        assert_eq!((wall.color, wall.custom), (5, 0));
    }
    #[test]
    fn latest_false_command_overrides_keyboard_cancel_without_restoring_values() {
        let mut world = setup();
        let menu = world.spawn(ColorRadialMenu::default()).id();
        world
            .resource_mut::<country_core::input::action_set::ActionSetStates>()
            .digital[18]
            .phase = country_core::input::action_set::DigitalActionPhase::JustPressed;
        let mut cmds = world.resource_mut::<Events<HideColorMenuCmd>>();
        cmds.send(HideColorMenuCmd { cancel: true });
        cmds.send(HideColorMenuCmd { cancel: false });
        let mut schedule = Schedule::default();
        schedule.add_systems(hide_color_menu);
        schedule.run(&mut world);
        assert!(world.get_entity(menu).is_err());
        assert!(world.resource::<Events<SetWallColorIdCmd>>().is_empty());
        assert!(world.resource::<Events<PlayUiSound>>().is_empty());
    }
}
