//! Validates the scenario wire format before creating runtime events.
use country_core::resources::walls::publication_fields::WallId;
use country_core::resources::walls::publication_fields::WallShapeSnapshot;
use serde_json::Value;
use std::error::Error;
pub(super) fn decode_hex<const N: usize>(value: &Value) -> Result<[u8; N], Box<dyn Error>> {
    let encoded = value.as_str().ok_or("Expected hex string")?;
    if encoded.len() != N * 2 || !encoded.is_ascii() {
        return Err("Wrong hex length".into());
    }
    let mut bytes = [0; N];
    for (index, byte) in bytes.iter_mut().enumerate() {
        *byte = u8::from_str_radix(&encoded[index * 2..index * 2 + 2], 16)?;
    }
    Ok(bytes)
}

pub(super) fn wall_id(value: &Value) -> Result<WallId, Box<dyn Error>> {
    Ok(WallId(
        value
            .as_str()
            .ok_or("Wall ID must be a decimal string")?
            .parse()?,
    ))
}

pub(super) fn finite_f32(value: &Value) -> Result<f32, Box<dyn Error>> {
    let value = value.as_f64().ok_or("Expected number")? as f32;
    if !value.is_finite() {
        return Err("Number must fit a finite f32".into());
    }
    Ok(value)
}

pub(super) fn authoring_shape(
    value: &Value,
) -> Result<country_core::systems::wall::shapes::authoring::AuthoringShape, Box<dyn Error>>
{
    use country_core::systems::wall::shapes::authoring::AuthoringShape;
    use country_core::resources::walls::freeform_height_cache::PreviewCircleObserved;
    use country_core::resources::walls::freeform_height_cache::PreviewRectangleObserved;
    let bytes = decode_hex::<28>(value)?;
    let coordinates: [f32; 6] = std::array::from_fn(|index| {
        let start = 4 + index * 4;
        f32::from_le_bytes(
            bytes[start..start + 4]
                .try_into()
                .expect("fixed shape record"),
        )
    });
    Ok(match u32::from_le_bytes(bytes[..4].try_into().unwrap()) {
        0 => AuthoringShape::Circle(PreviewCircleObserved {
            center: [coordinates[0], coordinates[1]],
            radius: coordinates[2],
            angle: coordinates[3],
        }),
        1 => AuthoringShape::Rectangle(PreviewRectangleObserved {
            axes: [coordinates[0], coordinates[1]],
            center: [coordinates[2], coordinates[3]],
            dimensions: [coordinates[4], coordinates[5]],
        }),
        _ => return Err("Creation requires a circle or rectangle shape".into()),
    })
}

pub(super) fn opening_family(
    value: &Value,
) -> Result<country_core::resources::walls::wall_holes::opening_rules::OpeningFamily, Box<dyn Error>>
{
    use country_core::resources::walls::wall_holes::opening_rules::OpeningFamily;
    Ok(match value.as_str() {
        Some("cottage_window") => OpeningFamily::CottageWindow,
        Some("gothic_window") => OpeningFamily::GothicWindow,
        Some("cottage_door") => OpeningFamily::CottageBalconyDoor,
        Some("gothic_door") => OpeningFamily::GothicBalconyDoor,
        _ => return Err("Unknown opening family".into()),
    })
}

// Storage addresses are explicit: absent UI state must not choose an owner.
fn observed_decorator_address(
    value: &Value,
) -> Result<
    country_core::resources::walls::decorator_archivist::changes::DecoratorStorageAddress,
    Box<dyn Error>,
> {
    use country_core::resources::walls::decorator_archivist::changes::DecoratorStorageAddress;
    Ok(match value["kind"].as_str() {
        Some("wall") => DecoratorStorageAddress::Wall(wall_id(&value["wall_id"])?),
        Some("special1") => DecoratorStorageAddress::ObservedSpecial1,
        Some("special2") => DecoratorStorageAddress::ObservedSpecial2,
        _ => return Err("Unknown observed decorator address".into()),
    })
}

pub(super) fn decorator_id(value: &Value) -> Result<u32, Box<dyn Error>> {
    Ok(u32::try_from(
        value.as_u64().ok_or("Decorator ID must be an integer")?,
    )?)
}

fn observed_tracked_decorators(
    value: &Value,
) -> Result<
    country_core::resources::walls::decorator_archivist::changes::TrackedDecorators,
    Box<dyn Error>,
> {
    use country_core::resources::walls::decorator_archivist::changes::ObservedDecoratorInfo;
    use country_core::resources::walls::decorator_archivist::changes::TrackedDecorators;
    let removed = value["removed"]
        .as_array()
        .ok_or("Missing observed removed IDs")?
        .iter()
        .map(decorator_id)
        .collect::<Result<_, _>>()?;
    let added = value["added"]
        .as_array()
        .ok_or("Missing observed added IDs")?
        .iter()
        .map(decorator_id)
        .collect::<Result<_, _>>()?;
    let mut items = std::collections::HashMap::new();
    for row in value["items"]
        .as_array()
        .ok_or("Missing observed live decorators")?
    {
        let key = decorator_id(&row["decorator_id"])?;
        if items
            .insert(key, ObservedDecoratorInfo(decode_hex(&row["info_hex"])?))
            .is_some()
        {
            return Err("Duplicate live decorator ID in observed input".into());
        }
    }
    Ok(TrackedDecorators {
        removed,
        items,
        added,
    })
}

pub(super) fn observed_decorator_containers(
    value: &Value,
) -> Result<
    country_core::resources::walls::decorator_archivist::changes::DecoratorStorage,
    Box<dyn Error>,
> {
    use country_core::resources::walls::decorator_archivist::changes::DecoratorStorage;
    let mut storage = DecoratorStorage::default();
    for row in value["walls"]
        .as_array()
        .ok_or("Missing observed wall decorator containers")?
    {
        if storage
            .walls
            .insert(wall_id(&row["wall_id"])?, observed_tracked_decorators(row)?)
            .is_some()
        {
            return Err("Duplicate wall decorator container".into());
        }
    }
    for (key, target) in [
        ("special1", &mut storage.special1),
        ("special2", &mut storage.special2),
    ] {
        let value = value
            .get(key)
            .ok_or("Special decorator container must be explicit, possibly null")?;
        if !value.is_null() {
            *target = Some(observed_tracked_decorators(value)?);
        }
    }
    Ok(storage)
}

pub(super) fn observed_decorator_storage(
    value: &Value,
) -> Result<
    country_core::resources::walls::decorator_archivist::changes::DecoratorStorage,
    Box<dyn Error>,
> {
    let mut storage = observed_decorator_containers(value)?;
    for row in value["id_to_address"]
        .as_array()
        .ok_or("Missing observed decorator live index")?
    {
        if storage
            .id_to_address
            .insert(
                decorator_id(&row["decorator_id"])?,
                observed_decorator_address(&row["address"])?,
            )
            .is_some()
        {
            return Err("Duplicate ID in decorator live index".into());
        }
    }
    for row in value["removed_by_address"]
        .as_array()
        .ok_or("Missing observed saved removed IDs")?
    {
        let ids = row["removed"]
            .as_array()
            .ok_or("Missing observed removed IDs")?
            .iter()
            .map(decorator_id)
            .collect::<Result<_, _>>()?;
        if storage
            .removed_by_address
            .insert(observed_decorator_address(&row["address"])?, ids)
            .is_some()
        {
            return Err("Duplicate saved decorator address".into());
        }
    }
    Ok(storage)
}

pub(super) fn observed_wall_snapshot(
    value: &Value,
) -> Result<
    country_core::resources::history::wall_edit::WallAuthoritySnapshot,
    Box<dyn Error>,
> {
    use country_core::resources::walls::publication_source::RawWallPublicationSource;
    use country_core::resources::history::wall_edit::WallAuthoritySnapshot;
    use country_core::resources::history::wall_edit::WallHistoryMetadata;
    use country_core::resources::roofs::geometry::surface::Curve2;
    let points = value["points_xz"]
        .as_array()
        .ok_or("Missing next raw Curve2")?
        .iter()
        .map(|p| Ok([finite_f32(&p[0])?, finite_f32(&p[1])?]))
        .collect::<Result<Vec<_>, Box<dyn Error>>>()?;
    let curve = Curve2::try_new(
        points,
        value["closed"]
            .as_bool()
            .ok_or("Missing observed closed flag")?,
    )
    .map_err(|e| format!("Invalid next raw curve: {e:?}"))?;
    let byte = |key: &str| -> Result<u8, Box<dyn Error>> {
        Ok(u8::try_from(
            value[key].as_u64().ok_or("Missing observed byte")?,
        )?)
    };
    Ok(WallAuthoritySnapshot {
        source: RawWallPublicationSource {
            curve,
            curve_kind_tag: u32::try_from(
                value["curve_kind_tag"]
                    .as_u64()
                    .ok_or("Missing observed raw curve kind")?,
            )?,
            terrain_floor: finite_f32(&value["terrain_floor"])?,
            shape: WallShapeSnapshot::from_observed(decode_hex(&value["shape_hex"])?),
            style: byte("style")?,
            extra: finite_f32(&value["extra"])?,
            observed_path_scalar: finite_f32(&value["observed_path_scalar"])?,
            clutter_version: byte("clutter_version")?,
            observed_source_flag_64: byte("observed_source_flag_64")?,
            special: value["special"]
                .as_bool()
                .ok_or("Missing observed special flag")?,
        },
        metadata: WallHistoryMetadata {
            source_66: value
                .get("source_66")
                .map(|_| byte("source_66"))
                .transpose()?,
            source_60_bits: u32::from_str_radix(
                value["source_60_bits"]
                    .as_str()
                    .ok_or("Missing source60 bits")?,
                16,
            )?,
            source_67: byte("source_67")?,
            source_69: byte("source_69")?,
        },
    })
}

/// Encodes records without per-byte formatting, reserving the known size hint.
pub(super) fn encode_hex(bytes: impl IntoIterator<Item = u8>) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let bytes = bytes.into_iter();
    let mut encoded = String::with_capacity(bytes.size_hint().0.saturating_mul(2));
    for byte in bytes {
        encoded.push(char::from(DIGITS[usize::from(byte >> 4)]));
        encoded.push(char::from(DIGITS[usize::from(byte & 0x0f)]));
    }
    encoded
}
