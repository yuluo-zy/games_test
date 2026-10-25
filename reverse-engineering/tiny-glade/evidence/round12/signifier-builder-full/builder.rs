/// Owned projection of original SignifierMesh's five native branches.
/// The original uses Vec/String niches; no raw application pointer is retained.
pub enum SignifierMesh {
    Named(String),
    Empty,
    Points(Vec<glam::Vec3>),
    PointsWithColors { points: Vec<glam::Vec3>, colors: Vec<glam::Vec3> },
    Generated(Box<dyn Fn() -> Vec<glam::Vec3> + Send + Sync>),
}
impl std::fmt::Debug for SignifierMesh {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Named(name) => f.debug_tuple("Named").field(name).finish(),
            Self::Empty => f.write_str("Empty"),
            Self::Points(points) => f.debug_tuple("Points").field(points).finish(),
            Self::PointsWithColors { points, colors } => f.debug_struct("PointsWithColors").field("points", points).field("colors", colors).finish(),
            Self::Generated(_) => f.write_str("Generated(..)"),
        }
    }
}

/// B14089d9b0/14089dd50: mesh resolution and complete drawable lifecycle.
/// Fields are owned typed values rather than a counterfeit native0x90B ABI.
#[derive(Debug)]
pub struct SignifierEntityBuilder {
    mesh: SignifierMesh,
    rotation: glam::Quat,
    position: glam::Vec3,
    shader: String,
    animation: SignifierAnimation,
    explicit_scale: Option<f32>,
    minimum_lifespan: Option<f32>,
    parent_to_volume_origin: bool,
    wireframe: bool,
    ui_pass: bool,
}

impl SignifierEntityBuilder {
    pub fn new(mesh: SignifierMesh) -> Self {
        Self {
            mesh,
            rotation: glam::Quat::IDENTITY,
            position: glam::Vec3::ZERO,
            shader: "vertex_color_shader".to_owned(),
            animation: SignifierAnimation {
                head: 1u64.to_le_bytes(), // Native default EasingFunction::CubicInOut.
                flag_8: 0,
                flag_9: 1,
                value: 1.0,
                duration: 0.2,
                t: 0.0,
                eased: 0.0,
                ..Default::default()
            },
            explicit_scale: None,
            minimum_lifespan: None,
            parent_to_volume_origin: false,
            wireframe: false,
            ui_pass: true,
        }
    }
    pub fn from_mesh_name(name: impl Into<String>) -> Self {
        Self::new(SignifierMesh::Named(name.into()))
    }
    pub fn from_points(points: Vec<glam::Vec3>) -> Self {
        Self::new(SignifierMesh::Points(points))
    }
    pub fn position(mut self, position: glam::Vec3) -> Self { self.position=position; self }
    pub fn rotation(mut self, rotation: glam::Quat) -> Self { self.rotation=rotation; self }
    pub fn scale(mut self, scale: f32) -> Self { self.explicit_scale=Some(scale); self }
    pub fn animation_duration(mut self, duration: f32) -> Self { self.animation.duration=duration; self }
    pub fn custom_shader(mut self, name: &str) -> Self { self.shader=name.to_owned(); self }
    pub fn shader_name(&self) -> &str { &self.shader }
    /// Native set_ui_pass writes pass^1 and build inserts UiPass only at zero.
    pub fn set_ui_pass(mut self, pass: u8) -> Self { self.ui_pass=(pass ^ 1)==0; self }
    pub fn mark_as_debug(mut self) -> Self { self.wireframe=true; self }
    pub fn disable_default_animation(mut self) -> Self { self.animation.flag_8=1; self }
    pub fn minimum_lifespan(mut self, lifespan: f32) -> Self { self.minimum_lifespan=Some(lifespan); self }
    pub fn parent_to_widget(mut self) -> Self { self.parent_to_volume_origin=true; self }
    pub fn animation_easing(mut self, easing: ::utils::easing_functions::EasingFunction) -> Self {
        use ::utils::easing_functions::EasingFunction as E;
        let (tag, value): (u32, f32) = match easing {
            E::CircularInOut => (0,0.0), E::CubicInOut => (1,0.0), E::SineInOut => (2,0.0),
            E::Bias(v) => (3,v), E::ElasticInOut => (4,0.0), E::BackOut => (5,0.0), E::CubicOut => (6,0.0),
        };
        self.animation.head[..4].copy_from_slice(&tag.to_le_bytes());
        self.animation.head[4..].copy_from_slice(&value.to_le_bytes());
        self
    }

    pub fn build(
        mut self,
        meshes: &AssetMeshLibrary,
        shaders: &AssetShaderLibrary,
        commands: &mut Commands,
    ) -> Entity {
        use crate::{
            asset_libraries::{mesh_library::MeshAsset, transient_mesh::TransientMesh},
            components::drawable::DrawableMeshBundle,
            render::mesh::{Mesh, ATTR_COLOR},
            systems::render_loop::draw_passes::draw_lists::{DontCastShadow, UiPass, WireframePass},
            utils::{components::transform::Transform, geo::mesh_from_pts},
        };
        use super::{animation::SignifierParentToVolumeOrigin,
            signifier_mark_for_delete::SignifierMinimumLifespan};

        // Only named meshes are library-owned. Every generated/empty variant
        // registers its actual CPU mesh and attaches the same deletion receipt.
        let (mesh, transient) = match self.mesh {
            SignifierMesh::Named(name) => (meshes.get_handle_by_name(&name), false),
            SignifierMesh::Empty => (meshes.add(MeshAsset::new(Mesh::new("signifier empty"), None)), true),
            SignifierMesh::Points(points) => (meshes.add(MeshAsset::new(mesh_from_pts("signifier custom", &points), None)), true),
            SignifierMesh::PointsWithColors { points, colors } => {
                let mut mesh = mesh_from_pts("signifier custom w colors", &points);
                mesh.set_attribute(ATTR_COLOR, colors);
                (meshes.add(MeshAsset::new(mesh, None)), true)
            }
            SignifierMesh::Generated(make_points) => {
                let points = make_points();
                (meshes.add(MeshAsset::new(mesh_from_pts("signifier custom fn", &points), None)), true)
            }
        };
        // Original reserves its Entity before shader lookup; missing shader
        // remains a failure, never a default shader or invisible success.
        let entity = commands.spawn_empty().id();
        let scale = self.explicit_scale.unwrap_or(1.0);
        let initial_scale = if self.animation.flag_8==0 {
            self.animation.value=scale;
            0.0
        } else { scale };
        let shader = shaders.get_raster_handle_by_name(&self.shader).unwrap();
        commands.entity(entity).insert((DrawableMeshBundle {
            mesh, shader,
            transform: Transform { rotation:self.rotation, translation:self.position, scale:glam::Vec3::splat(initial_scale) },
        }, self.animation));
        if self.wireframe { commands.entity(entity).insert(WireframePass); }
        else if self.ui_pass { commands.entity(entity).insert(UiPass); }
        commands.entity(entity).insert(DontCastShadow);
        if let Some(min_lifespan)=self.minimum_lifespan {
            commands.entity(entity).insert(SignifierMinimumLifespan { t:0.0, min_lifespan });
        }
        if self.parent_to_volume_origin { commands.entity(entity).insert(SignifierParentToVolumeOrigin); }
        if transient { commands.entity(entity).insert(TransientMesh::new_with_registry(mesh, meshes.transient_meshes())); }
        // SignifierId belongs to signifier_create's name-hash assignment.
        // Builder does not fabricate an id0 that can alias another request.
        entity
    }
}
