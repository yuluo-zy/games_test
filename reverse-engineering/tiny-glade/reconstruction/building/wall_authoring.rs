//! Original B shape creation inputs and InnerWalls authority mutations.
//! std owns IDs/maps/dirty sets; geometry uses the existing recovered helpers.
//! UI acceptance and selected building colors remain separate real consumers.
use crate::{
    building_wall_collection::RawWallPublicationSource,
    building_wall_freeform_cache::{
        PreviewCircleObserved, PreviewRectangleObserved, ShapeHeightError,
        circle_curve, rectangle_curve,
    },
    building_wall_state::{WallId, WallShapeSnapshot},
    roof_parameters::RoofParameterBlock,
};

#[derive(Clone, Copy, Debug)]
pub enum AuthoringShape {
    Circle(PreviewCircleObserved),
    Rectangle(PreviewRectangleObserved),
}

#[derive(Clone, Copy, Debug)]
pub struct CreationSettingsObserved {
    pub wall_color_id: u32,
    pub roof_color_id: u32,
    pub timber_color_id: u32,
    pub wall_style: u8,
    pub rim_style: u8,
}

#[derive(Clone, Copy, Debug)]
pub struct PreviousBuildingObserved {
    pub public_point_count: u64,
    pub public_raw_length: f32,
    pub public_shape_tag: u32,
    pub public_style: u8,
    /// Actual color-ID arrays are indexed by the referenced existing WallId.
    pub wall_color_id: u32,
    pub timber_color_id: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SelectedCreationAppearance {
    pub wall_color_id: u32,
    pub roof_color_id: u32,
    pub timber_color_id: u32,
    pub wall_style: u8,
    /// Original Roof+40..4c, inherited only after a valid regular public host.
    /// None is canonical zero; its inactive payload has no claimed native bits.
    pub roof_appearance: [u8; 12],
}

/// Actual get_building_and_roof_colors business policy. The caller resolves the
/// existing public/roof objects using standard World queries; this function
/// retains the game's validity guard and the freeform skip before inheritance.
pub fn select_creation_appearance(
    settings: CreationSettingsObserved,
    previous: Option<PreviousBuildingObserved>,
    prior_roof: Option<&[u8; 88]>,
) -> Result<SelectedCreationAppearance, crate::building_wall_state::WallReadError> {
    let mut selected = SelectedCreationAppearance {
        wall_color_id: settings.wall_color_id,
        roof_color_id: settings.roof_color_id,
        timber_color_id: settings.timber_color_id,
        wall_style: settings.wall_style,
        roof_appearance: [0; 12],
    };
    if let Some(previous) = previous {
        crate::building_wall_state::validate_public_wall_read(previous.public_point_count, previous.public_raw_length)?;
        if previous.public_shape_tag < 2 {
            selected.wall_color_id = previous.wall_color_id;
            selected.timber_color_id = previous.timber_color_id;
            selected.wall_style = previous.public_style;
            if let Some(roof) = prior_roof {
                selected.roof_color_id = u32::from_le_bytes(roof[0x50..0x54].try_into().unwrap());
                if roof[0x40] != 0 { selected.roof_appearance = roof[0x40..0x4c].try_into().unwrap(); }
            }
        }
    }
    Ok(selected)
}

fn source_max(a: f32, b: f32) -> f32 { if a > b { a } else { b } }
fn source_min(a: f32, b: f32) -> f32 { if a < b { a } else { b } }
fn clamp01(value: f32) -> f32 { source_min(1.0, source_max(0.0, value)) }
fn smoothstep(value: f32) -> f32 { (3.0 - (value + value)) * (value * value) }
fn sine_ease_in_out(value: f32) -> f32 {
    // Original private EasingFunction tag2 dispatches to this cosf expression.
    ((value * f32::from_bits(0x40490fdb)).cos() + (-1.0)) * (-0.5)
}

/// The two creation systems' actual shape-derived Roof presets. Actual selected
/// prior Roof has an independent lookup from PublicWalls/color/style selection.
pub fn creation_roof_parameters(
    shape: AuthoringShape,
    selected_rim: u8,
    prior_roof: Option<&[u8; 88]>,
) -> RoofParameterBlock {
    let (mut pitch, ridge, direction) = match shape {
        AuthoringShape::Circle(circle) => {
            let easing = sine_ease_in_out((circle.radius + f32::from_bits(0xbf666667)) / f32::from_bits(0x40e33333));
            (clamp01(((1.0 - easing) * 4.0 + easing * 10.0) / 12.0), 0.4, 0)
        }
        AuthoringShape::Rectangle(rectangle) => {
            let [width, length] = rectangle.dimensions;
            let w_ratio = width / length;
            let l_ratio = length / width;
            let aspect = if w_ratio.is_nan() { l_ratio } else { source_max(l_ratio, w_ratio) };
            let longest = if width.is_nan() { length } else { source_max(length, width) };
            let offset = longest + (-2.25);
            let scale = smoothstep(clamp01(offset / 5.25));
            let ridge = smoothstep(clamp01(((scale * 1.5 + aspect) + (-1.5)) / 1.5));
            let easing = sine_ease_in_out(offset / 12.75);
            (clamp01(((1.0 - easing) * 3.0 + easing * 6.0) / 12.0), ridge, u8::from(length > width))
        }
    };
    let mut rim = selected_rim;
    if let Some(roof) = prior_roof {
        rim = roof[0x3d];
        let old_pitch = f32::from_le_bytes(roof[0x30..0x34].try_into().unwrap());
        if old_pitch <= f32::from_bits(0x3dcccccd) { pitch = old_pitch; }
    }
    let mut parameters = [0; 28];
    for (offset, value) in [(0, 0.0f32), (4, 0.0), (8, 0.5), (12, pitch), (16, ridge), (20, 0.2)] {
        parameters[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
    }
    parameters[24] = direction;
    parameters[25] = rim;
    RoofParameterBlock::from_bytes(parameters)
}

#[derive(Clone, Copy, Debug)]
pub struct CreateShapeRequest {
    pub shape: AuthoringShape,
    pub previous_wall_hint: Option<WallId>,
    /// Create event's first scalar, copied into new_wall_w_params XMM2.
    pub height: f32,
    /// Event's second scalar; update_elevation writes source+58 afterward.
    pub elevation: f32,
    /// Actual get_building_and_roof_colors selected style, never a guessed default.
    pub selected_style: u8,
}

#[derive(Clone, Debug)]
pub struct AuthoredWall {
    pub wall_id: WallId,
    pub source: RawWallPublicationSource,
    /// New source insertion (+30 collection) and later updates (+60 collection)
    /// are separate original notifications even though each contains this ID.
    pub new_source_notification: WallId,
    pub updated_source_notification: WallId,
}

#[derive(Clone, Debug)]
pub struct AuthoredBuilding {
    pub wall: AuthoredWall,
    pub appearance: SelectedCreationAppearance,
    pub roof_request: crate::building_roof_creation::RoofCreateRequestObserved,
}

#[derive(Clone, Debug)]
pub struct RectangleEditOutcome {
    pub wall_id: WallId,
    /// The original curve and enclosure writes each notify +60 when found.
    pub updated_source_notifications: Option<[WallId; 2]>,
    pub audio_anchor: [f32; 3],
    pub material_audio_kind: u16,
    pub motion_audio_amplitude: f32,
    pub motion_audio_flag: bool,
}

/// Complete rectangle edit consumer after standard EventReader::last selection.
/// It builds geometry even for missing IDs and still emits both audio requests.
/// Source scalar/style metadata remain intact; the parent owns source lookup,
/// changed ticks and standard audio Event IDs/counters.
pub fn edit_rectangle_source(
    source: Option<&mut RawWallPublicationSource>,
    event: &[u8; 64],
    game_time_step: f32,
) -> Result<RectangleEditOutcome, ShapeHeightError> {
    let read = |at| f32::from_le_bytes(event[at..at + 4].try_into().unwrap());
    let rectangle = PreviewRectangleObserved {
        axes: [read(16), read(20)], center: [read(24), read(28)],
        dimensions: [read(32), read(36)],
    };
    let curve = rectangle_curve(rectangle)?;
    let wall_id = WallId(u64::from_le_bytes(event[8..16].try_into().unwrap()));
    let found = source.is_some();
    if let Some(source) = source {
        source.curve = curve;
        source.curve_kind_tag = 1;
        source.shape = shape_snapshot(AuthoringShape::Rectangle(rectangle));
    }
    let flag = event[56] != 0;
    let speed = read(52);
    let quotient = speed / game_time_step;
    // DIVSS's invalid result is the x86 indefinite NaN on 0/0 or Inf/Inf.
    let velocity = if quotient.is_nan() && !speed.is_nan() && !game_time_step.is_nan() {
        f32::from_bits(0xffc00000)
    } else { quotient };
    let amplitude = clamp01((velocity - 0.0) / if flag { 20.0 } else { 200.0 });
    Ok(RectangleEditOutcome {
        wall_id, updated_source_notifications: found.then_some([wall_id; 2]),
        audio_anchor: [read(40), read(44), read(48)], material_audio_kind: 4,
        motion_audio_amplitude: amplitude, motion_audio_flag: flag,
    })
}

#[derive(Clone, Debug, PartialEq)]
pub enum AuthoringCreationError {
    PriorHost(crate::building_wall_state::WallReadError),
    Geometry(ShapeHeightError),
}

/// Authority creation from actual ShapePlacement and resolved original context.
/// Valid previous host is required; corrupt host panic/partial-side-effects are
/// outside this adapter. Caller dispatches returned source and Roof/colour events
/// through actual World owners rather than writing public caches itself.
pub fn create_building_from_placement(
    id_counter: &mut u64,
    shape: AuthoringShape,
    placement: &[u8; 80],
    settings: CreationSettingsObserved,
    previous: Option<PreviousBuildingObserved>,
    prior_roof: Option<&[u8; 88]>,
) -> Result<AuthoredBuilding, AuthoringCreationError> {
    let has_hint = placement[0x2c] != 2;
    let previous = previous.filter(|_| has_hint);
    let prior_roof = prior_roof.filter(|_| has_hint);
    let appearance = select_creation_appearance(settings, previous, prior_roof).map_err(AuthoringCreationError::PriorHost)?;
    let request = request_from_shape_placement(shape, placement, appearance.wall_style);
    let wall = create_shape_source(id_counter, request).map_err(AuthoringCreationError::Geometry)?;
    let parameters = creation_roof_parameters(shape, settings.rim_style, prior_roof);
    let mut record = [0; 64];
    record[8..16].copy_from_slice(&wall.wall_id.0.to_le_bytes());
    record[16..20].copy_from_slice(&appearance.roof_color_id.to_le_bytes());
    record[20..32].copy_from_slice(&appearance.roof_appearance);
    record[32..60].copy_from_slice(parameters.bytes());
    Ok(AuthoredBuilding {
        wall,
        appearance,
        roof_request: crate::building_roof_creation::RoofCreateRequestObserved::from_observed(record),
    })
}

fn shape_snapshot(shape: AuthoringShape) -> WallShapeSnapshot {
    let mut bytes = [0; 28];
    let (tag, fields) = match shape {
        AuthoringShape::Circle(c) => (0u32, vec![c.center[0], c.center[1], c.radius, c.angle]),
        AuthoringShape::Rectangle(r) => (1u32, vec![
            r.axes[0], r.axes[1], r.center[0], r.center[1], r.dimensions[0], r.dimensions[1],
        ]),
    };
    bytes[..4].copy_from_slice(&tag.to_le_bytes());
    for (index, value) in fields.into_iter().enumerate() {
        bytes[4 + index * 4..8 + index * 4].copy_from_slice(&value.to_le_bytes());
    }
    WallShapeSnapshot::from_observed(bytes)
}

/// Build the exact raw authority source from an accepted original create event.
/// Curve construction occurs before allocating the ID, as in the real caller.
/// Initial counter is a required owner input; no startup value is manufactured.
pub fn create_shape_source(
    id_counter: &mut u64,
    request: CreateShapeRequest,
) -> Result<AuthoredWall, ShapeHeightError> {
    let curve = match request.shape {
        AuthoringShape::Circle(circle) => circle_curve(circle, 0.5)?,
        AuthoringShape::Rectangle(rectangle) => rectangle_curve(rectangle)?,
    };
    let shape = shape_snapshot(request.shape);
    *id_counter = id_counter.wrapping_add(1);
    let wall_id = WallId(*id_counter);
    Ok(AuthoredWall {
        wall_id,
        source: RawWallPublicationSource {
            curve,
            curve_kind_tag: shape.tag(),
            terrain_floor: request.elevation,
            shape,
            style: request.selected_style,
            extra: request.height,
            special: false,
            // Literal new_wall_w_params source defaults, not inferred from a fixture.
            observed_path_scalar: f32::from_bits(0x40333333),
            clutter_version: 0,
            observed_source_flag_64: 0,
        },
        new_source_notification: wall_id,
        updated_source_notification: wall_id,
    })
}

/// The successful original UI event-writing instructions copy +40 and +3c
/// without constraints; acceptance/cancel guards happen before this function.
pub fn request_from_shape_placement(
    shape: AuthoringShape,
    observed_placement: &[u8; 80],
    selected_style: u8,
) -> CreateShapeRequest {
    let read = |offset| f32::from_le_bytes(observed_placement[offset..offset + 4].try_into().unwrap());
    CreateShapeRequest {
        shape,
        previous_wall_hint: (observed_placement[0x2c] != 2).then(|| WallId(u64::from_le_bytes(observed_placement[0x20..0x28].try_into().unwrap()))),
        height: read(0x40),
        elevation: read(0x3c),
        selected_style,
    }
}

/// Active CreateCircle/CreateRectangle payload fields emitted by the original
/// successful UI branch. An inactive None hint payload is intentionally absent.
pub fn active_create_event_bytes(request: CreateShapeRequest) -> Vec<u8> {
    let mut output = Vec::new();
    output.extend(u64::from(request.previous_wall_hint.is_some()).to_le_bytes());
    if let Some(id) = request.previous_wall_hint { output.extend(id.0.to_le_bytes()); }
    let shape = shape_snapshot(request.shape);
    let field_count = match request.shape { AuthoringShape::Circle(_) => 16, AuthoringShape::Rectangle(_) => 24 };
    output.extend(&shape.bytes()[4..4 + field_count]);
    output.extend(request.height.to_le_bytes());
    output.extend(request.elevation.to_le_bytes());
    output
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SourceMutation {
    Elevation(u32),
    Enclosure([u8; 28]),
    Style(u8),
}

/// Actual update wrappers dirty before each unconditional assignment, even when
/// the new bits equal the old bits. Missing lookup leaves source and dirty alone.
pub fn mutate_observed_source(
    resolved_source: Option<&mut [u8; 112]>,
    wall: WallId,
    mutation: SourceMutation,
) -> Option<WallId> {
    let source = resolved_source?;
    match mutation {
        SourceMutation::Elevation(bits) => source[0x58..0x5c].copy_from_slice(&bits.to_le_bytes()),
        SourceMutation::Enclosure(shape) => source[0x38..0x54].copy_from_slice(&shape),
        SourceMutation::Style(style) => source[0x68] = style,
    }
    Some(wall)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuthoringRemovalError { MissingSource }

/// Original source removal requires existence and schedules the +90 collection
/// before removing the row. Missing original IDs panic; this typed boundary
/// reports the failed precondition without changing the owner or its ID counter.
pub fn remove_source<T>(
    sources: &mut std::collections::HashMap<WallId, T>,
    removed_ids: &mut std::collections::HashSet<WallId>,
    wall: WallId,
) -> Result<T, AuthoringRemovalError> {
    if !sources.contains_key(&wall) { return Err(AuthoringRemovalError::MissingSource); }
    removed_ids.insert(wall);
    Ok(sources.remove(&wall).unwrap())
}

/// Complete early mode-mismatch path of the circle/rectangle UI systems.
/// Cancels the preview's presence tag and marks that resource changed; it does
/// not allocate a WallId or send a create event. Other preview bytes survive.
pub fn clear_preview_for_inactive_mode(
    app_mode: u32,
    app_subtype: u8,
    shape_tag: u32,
    preview: &mut [u8; 40],
) -> bool {
    let subtype = match shape_tag { 0 => 1, 1 => 2, _ => return false };
    if app_mode == 6 && app_subtype == subtype { return false; }
    preview[..4].fill(0);
    true
}
