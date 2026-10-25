// Candidate only: the complete primary option-construction algorithm of
// B14229a7f0. The ECS lookup, 13-way snapshot, secondary rings and final nav
// publication remain separate producer stages; this file does not claim them.
// Native fixture evidence: show-{primary,special,wall-flat}-original-execution.json.
// Padding and unused enum payload bytes in the original are deliberately not
// interpreted as values. Every meaningful name/kind/variant/value/layer is kept.
use crate::radial_menu::IconInfo;

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
