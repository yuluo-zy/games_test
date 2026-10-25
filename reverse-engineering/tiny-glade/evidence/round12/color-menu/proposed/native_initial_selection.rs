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
