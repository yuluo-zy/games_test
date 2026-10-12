//! Actual shipped JSON mesh input and game collision projection. This adapter
//! owns only recovered plain wall-family assets, not the original engine IO.
use bevy_ecs::prelude::Resource;
use serde_json::Value;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use tiny_glade_reconstruction::{
    building_decorator_placement::{
        CollisionBoundingBox, DecoratorLibKey, PlacementError, WallDecoratorPlacement,
        WallDecoratorRequest, b_wall_collision_asset_name, mesh_collision_bbx,
        place_wall_decorator, wall_decorator_key,
    },
    building_decorator_subtype::{
        SubtypeError, WallDecoratorSubtypeContext, WallWindowKind,
        b_wall_subtype_collision_asset_name, derive_wall_subtype, wall_opening_family,
    },
    building_opening_rules::OpeningFamily,
    building_wall_state::WallShapeSnapshot,
    surface::Curve2,
};

#[derive(Clone, Debug)]
pub struct DecoratorCollisionAsset {
    pub source_path: PathBuf,
    pub vertices: Vec<[f32; 3]>,
    pub bounding_box: CollisionBoundingBox,
}
#[derive(Resource, Default, Debug)]
pub struct DecoratorAssetLibrary {
    pub collisions: HashMap<DecoratorLibKey, DecoratorCollisionAsset>,
    pub revision: u64,
}
#[derive(Clone, Debug)]
pub struct DecoratorAssetBinding {
    pub family: OpeningFamily,
    pub rank: u8,
    /// Relative to the explicit assets directory. No process/global game root.
    pub mesh_path: PathBuf,
}
#[derive(Debug)]
pub enum DecoratorAssetError {
    Io {
        path: PathBuf,
        error: std::io::Error,
    },
    Json {
        path: PathBuf,
        error: serde_json::Error,
    },
    InvalidPositionSchema(PathBuf),
    Placement(PlacementError),
    DuplicateBinding(DecoratorLibKey),
    Subtype(SubtypeError),
}

#[derive(Debug)]
pub enum DecoratorSubtypePlacementError {
    Placement(PlacementError),
    Subtype(SubtypeError),
}

/// JSON schema confirmed in shipped meshes and Mesh::aabb_bounds: a selected
/// Vertex_Position attribute of type float3. Serde internals are reused.
pub fn decode_collision_mesh(path: &Path) -> Result<DecoratorCollisionAsset, DecoratorAssetError> {
    let input = std::fs::read(path).map_err(|error| DecoratorAssetError::Io {
        path: path.to_owned(),
        error,
    })?;
    let json: Value =
        serde_json::from_slice(&input).map_err(|error| DecoratorAssetError::Json {
            path: path.to_owned(),
            error,
        })?;
    let schema_error = || DecoratorAssetError::InvalidPositionSchema(path.to_owned());
    let attributes = json
        .get("attributes")
        .and_then(Value::as_array)
        .ok_or_else(schema_error)?;
    if !attributes
        .iter()
        .any(|v| v.as_str() == Some("Vertex_Position"))
    {
        return Err(schema_error());
    }
    let attribute = json.get("Vertex_Position").ok_or_else(schema_error)?;
    let kind = attribute
        .get("type")
        .and_then(Value::as_array)
        .ok_or_else(schema_error)?;
    if kind.len() != 2 || kind[0].as_str() != Some("float") || kind[1].as_u64() != Some(3) {
        return Err(schema_error());
    }
    let raw_vertices = attribute
        .get("buffer")
        .and_then(Value::as_array)
        .ok_or_else(schema_error)?;
    let mut vertices = Vec::with_capacity(raw_vertices.len());
    for vertex in raw_vertices {
        let raw = vertex.as_array().ok_or_else(schema_error)?;
        if raw.len() != 3 {
            return Err(schema_error());
        }
        let mut point = [0.0; 3];
        for i in 0..3 {
            point[i] = raw[i].as_f64().ok_or_else(schema_error)? as f32;
        }
        vertices.push(point);
    }
    let bounding_box = mesh_collision_bbx(&vertices).map_err(DecoratorAssetError::Placement)?;
    Ok(DecoratorCollisionAsset {
        source_path: path.to_owned(),
        vertices,
        bounding_box,
    })
}

impl DecoratorAssetLibrary {
    /// B six accepted wall-opening subtypes: four plain families plus actual
    /// cottage/gothic half-dormer keys. Gothic 7 intentionally shares mesh 6.
    pub fn load_b_wall_opening_assets(
        &mut self,
        assets_root: &Path,
    ) -> Result<(), DecoratorAssetError> {
        let mut collisions = HashMap::with_capacity(18);
        for subtype in [0, 1, 6, 7, 4, 9] {
            for rank in 1..=3 {
                let name = b_wall_subtype_collision_asset_name(subtype, rank)
                    .map_err(DecoratorAssetError::Subtype)?;
                let asset =
                    decode_collision_mesh(&assets_root.join(format!("meshes/{name}.json")))?;
                collisions.insert(DecoratorLibKey { subtype, rank }, asset);
            }
        }
        self.collisions = collisions;
        self.revision = self.revision.wrapping_add(1);
        Ok(())
    }

    /// Complete CPU wall placement consumer using actual source/publication/
    /// roof context. Family supplies window style; attachment state determines
    /// original door/corner precedence and current half/plain subtype.
    pub fn derive_and_place_wall(
        &self,
        request: WallDecoratorRequest,
        door_attachment: bool,
        corner_attached: bool,
        context: WallDecoratorSubtypeContext<'_>,
    ) -> Result<WallDecoratorPlacement, DecoratorSubtypePlacementError> {
        let kind = match request.family {
            OpeningFamily::CottageWindow | OpeningFamily::CottageBalconyDoor => {
                WallWindowKind::Cottage
            }
            _ => WallWindowKind::Gothic,
        };
        wall_decorator_key(request.family, request.rank)
            .map_err(DecoratorSubtypePlacementError::Placement)?;
        let coord = tiny_glade_reconstruction::building_decorator_placement::wall_coord_from_world(
            context.source_curve,
            request.world_position,
        )
        .map_err(DecoratorSubtypePlacementError::Placement)?;
        let subtype = if let Some(subtype) =
            tiny_glade_reconstruction::building_decorator_subtype::wall_attachment_subtype(
                kind,
                door_attachment,
                corner_attached,
            ) {
            subtype
        } else {
            let plain_family = match kind {
                WallWindowKind::Cottage => OpeningFamily::CottageWindow,
                WallWindowKind::Gothic => OpeningFamily::GothicWindow,
            };
            let plain_key = wall_decorator_key(plain_family, request.rank)
                .map_err(DecoratorSubtypePlacementError::Placement)?;
            let plain = self.collisions.get(&plain_key).ok_or(
                DecoratorSubtypePlacementError::Placement(PlacementError::MissingCollisionAsset),
            )?;
            derive_wall_subtype(kind, coord, false, false, context, plain.bounding_box)
                .map_err(DecoratorSubtypePlacementError::Subtype)?
        };
        let key = DecoratorLibKey {
            subtype,
            rank: request.rank,
        };
        let family =
            wall_opening_family(subtype).map_err(DecoratorSubtypePlacementError::Subtype)?;
        let collision =
            self.collisions
                .get(&key)
                .ok_or(DecoratorSubtypePlacementError::Placement(
                    PlacementError::MissingCollisionAsset,
                ))?;
        let mut result = place_wall_decorator(
            WallDecoratorRequest { family, ..request },
            context.source_curve,
            context.wall_shape,
            collision.bounding_box,
        )
        .map_err(DecoratorSubtypePlacementError::Placement)?;
        result.key = key;
        Ok(result)
    }
    /// Explicit B asset-name policy, confirmed by original rank dispatch. This
    /// loads all four requested plain families; it is not B's availability gate
    /// or full renderer/interaction/outline library initialization.
    pub fn load_b_plain_wall_assets(
        &mut self,
        assets_root: &Path,
    ) -> Result<(), DecoratorAssetError> {
        self.load_bindings(assets_root, &b_plain_wall_bindings())
    }
    /// Explicit bindings are also available for version-specific loaders. A
    /// failed load leaves the existing library intact; no default dimensions.
    pub fn load_bindings(
        &mut self,
        assets_root: &Path,
        bindings: &[DecoratorAssetBinding],
    ) -> Result<(), DecoratorAssetError> {
        let mut collisions = HashMap::with_capacity(bindings.len());
        for binding in bindings {
            let key = wall_decorator_key(binding.family, binding.rank)
                .map_err(DecoratorAssetError::Placement)?;
            if collisions.contains_key(&key) {
                return Err(DecoratorAssetError::DuplicateBinding(key));
            }
            let asset = decode_collision_mesh(&assets_root.join(&binding.mesh_path))?;
            collisions.insert(key, asset);
        }
        self.collisions = collisions;
        self.revision = self.revision.wrapping_add(1);
        Ok(())
    }

    pub fn place(
        &self,
        request: WallDecoratorRequest,
        curve: &Curve2,
        shape: &WallShapeSnapshot,
    ) -> Result<WallDecoratorPlacement, PlacementError> {
        let key = wall_decorator_key(request.family, request.rank)?;
        let collision = self
            .collisions
            .get(&key)
            .ok_or(PlacementError::MissingCollisionAsset)?;
        place_wall_decorator(request, curve, shape, collision.bounding_box)
    }
}

pub fn b_plain_wall_bindings() -> Vec<DecoratorAssetBinding> {
    let families = [
        OpeningFamily::CottageWindow,
        OpeningFamily::GothicWindow,
        OpeningFamily::CottageBalconyDoor,
        OpeningFamily::GothicBalconyDoor,
    ];
    families
        .into_iter()
        .flat_map(|family| {
            (1..=3).map(move |rank| DecoratorAssetBinding {
                family,
                rank,
                mesh_path: PathBuf::from(format!(
                    "meshes/{}.json",
                    b_wall_collision_asset_name(family, rank).expect("fixed valid B ranks")
                )),
            })
        })
        .collect()
}
