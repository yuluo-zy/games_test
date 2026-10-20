"""Stage reviewed API additions without mutating shared production during Cargo."""
from pathlib import Path
import json, hashlib
ROOT=Path('D:/game');OUT=Path(__file__).parent;manifest=[]
def stage(path,changes):
    src=ROOT/path;base=src.read_bytes();txt=base.decode('utf8')
    for before,after in changes:
        assert txt.count(before)==1,(path,before,txt.count(before))
        txt=txt.replace(before,after)
    dst=OUT/'proposed'/path;dst.parent.mkdir(parents=True,exist_ok=True);dst.write_text(txt,encoding='utf8',newline='\n')
    manifest.append(dict(path=path,base_sha256=hashlib.sha256(base).hexdigest(),candidate_sha256=hashlib.sha256(dst.read_bytes()).hexdigest()))
stage('tiny-glade/crates/country-core/src/systems/ui/resources/widget_params.rs',[
('#[derive(Clone, Debug, PartialEq)]\npub struct WidgetParams {','''#[derive(Clone, Debug)]
pub struct WidgetParams {
    /// Native first24B Vec<SignifierFactory>; the active WidgetParams Arc
    /// retains actual factory closures after the frame registry is cleared.
    pub signifiers: Vec<super::super::signifiers::SignifierFactory>,'''),
('/// 0xb0 B 参数块（O：默认 u32 组 + UiSounds + WidgetVolumeShape + u16@0xa8=0x101 +', '''/// Complete native volume discriminants from closure2@140ac3f50's eight-entry
/// jump table142adfb28. Keep original owned Vec payloads and all scalar fields.
#[derive(Clone, Debug, PartialEq)]
pub enum WidgetGeometry {
    Sphere { center: Vec3, radius: f32 },
    SphereCloud { centers: Vec<Vec3>, radius: f32 },
    TriangleMesh(Vec<utils::geometry::intersect::Triangle>),
    ScreenAabb { min: Vec2, max: Vec2, distance: f32 },
    WorldCanvas { extent: Vec2, layer: u8 },
    TerrainCanvas { distance: f32, layer: u8 },
    PlaneDisc { point: Vec3, normal: Vec3, radius: f32 },
    Empty,
}

impl WidgetGeometry {
    pub fn native_tag(&self) -> u8 {
        match self {
            Self::Sphere { .. } => 0,
            Self::SphereCloud { .. } => 1,
            Self::TriangleMesh(_) => 2,
            Self::ScreenAabb { .. } => 3,
            Self::WorldCanvas { .. } => 4,
            Self::TerrainCanvas { .. } => 5,
            Self::PlaneDisc { .. } => 6,
            Self::Empty => 7,
        }
    }
}

/// 0xb0 B 参数块（O：默认 u32 组 + UiSounds + WidgetVolumeShape + u16@0xa8=0x101 +'''),
('    pub volume: WidgetVolumeShape,','''    pub volume: WidgetVolumeShape,
    /// Native volume tag2 owns a Vec<Triangle>, used both by actual roof
    /// trigger meshes and by box_trigger_volume. Preserve all triangles and
    /// their original ordering; scalar volume fields do not model this variant.
    pub geometry: Option<WidgetGeometry>,'''),
('            volume,\n            occlude_by_world: true,','            volume,\n            geometry: None,\n            occlude_by_world: true,'),
('            fields: DEFAULT_U32S,','            fields: DEFAULT_U32S,\n            signifiers: Vec::new(),'),
('impl WidgetParams {','''impl WidgetParams {
    pub fn with_signifiers(mut self, signifiers: Vec<super::super::signifiers::SignifierFactory>) -> Self {
        self.signifiers = signifiers;
        self
    }

    /// Exact owned tag2 volume; retain the current registry's Arc ownership.
    pub fn triangle_mesh(mut self, triangles: Vec<utils::geometry::intersect::Triangle>) -> Self {
        self.volume.tag = 2;
        self.geometry = Some(WidgetGeometry::TriangleMesh(triangles));
        self
    }

    /// Native tag3 stores the complete screen-space min/max rectangle.
    pub fn screen_aabb(mut self, min: Vec2, max: Vec2, distance: f32) -> Self {
        self.volume.tag = 3;
        self.geometry = Some(WidgetGeometry::ScreenAabb { min, max, distance });
        self
    }

    pub fn geometry(mut self, geometry: WidgetGeometry) -> Self {
        self.volume.tag = geometry.native_tag();
        self.geometry = Some(geometry);
        self
    }

    /// Older scalar producers encode complete sphere/canvas variants in their
    /// existing public fields. Never fabricate a missing Vec/plane/AABB.
    pub fn resolved_geometry(&self) -> std::borrow::Cow<'_, WidgetGeometry> {
        if let Some(geometry) = &self.geometry {
            assert_eq!(self.volume.tag, geometry.native_tag());
            return std::borrow::Cow::Borrowed(geometry);
        }
        std::borrow::Cow::Owned(match self.volume.tag {
            0 => WidgetGeometry::Sphere { center: self.volume.center, radius: self.volume.size.x },
            4 => WidgetGeometry::WorldCanvas { extent: self.volume.size, layer: self.volume.layer },
            5 => WidgetGeometry::TerrainCanvas { distance: self.volume.size.x, layer: self.volume.layer },
            7 => WidgetGeometry::Empty,
            tag => panic!("native UI volume tag{tag} requires its full owned payload"),
        })
    }
'''),
('        self.volume = volume;','        self.volume = volume;\n        self.geometry = None;'),
])
stage('tiny-glade/crates/country-core/src/systems/ui/resources/ui_state.rs',[
('    pub const STARTED: Self = Self(3);','    pub const STARTED: Self = Self(3);\n    /// Native ui_update140b2ec8b: continued dragging.\n    pub const CONTINUING: Self = Self(4);'),
('    interaction: UiInteraction,','    interaction: UiInteraction,\n    /// Native active Arc survives clearing of the frame registry.\n    active_params: Option<Arc<WidgetParams>>,'),
('    active_params: Option<Arc<WidgetParams>>,','    active_params: Option<Arc<WidgetParams>>,\n    /// Original +0 Vec of sorted intersections produced by hover_intersection.\n    hover_intersections: Vec<[u8; 56]>, '),
('            interaction,','            interaction,\n            active_params: None,'),
('            active_params: None,','            active_params: None,\n            hover_intersections: Vec::new(),'),
('    intersection_tag: u32,','''    intersection_tag: u32,
    /// Complete return projection of intersecting@14092e640. The input owner
    /// writes the real56B payload; no synthetic hit is manufactured by widgets.
    intersection_payload: [u8; 56],'''),
('            intersection_tag: 0,','            intersection_tag: 0,\n            intersection_payload: [0; 56],'),
('        state.intersection_tag = 2;','        state.set_intersection_tag(2);'),
('        self.intersection_tag = tag;','''        self.intersection_tag = tag;
        self.intersection_payload[..4].copy_from_slice(&tag.to_le_bytes());'''),
('    /// +0x38 active widget id.','''    /// Input arbitration supplies the original intersection result as one
    /// owned record. Its enum and further geometry fields stay lossless.
    pub fn replace_intersection(&mut self, payload: [u8; 56]) {
        self.intersection_tag = u32::from_le_bytes(payload[..4].try_into().unwrap());
        self.intersection_payload = payload;
    }

    /// +0x38 active widget id.'''),
('    /// `UiState::add<&str>`', '''    /// Snapshot in the current registry iteration order; arbitration owns
    /// native distance/layer ordering, not this storage API.
    pub fn registry_snapshot(&self) -> Vec<(WidgetId, Arc<WidgetParams>)> {
        self.widgets.read().iter().map(|(id, params)| (*id, Arc::clone(params))).collect()
    }

    pub fn set_hover_intersections(&mut self, intersections: Vec<[u8; 56]>) {
        self.hover_intersections = intersections;
    }

    pub fn hover_intersections(&self) -> &[[u8; 56]] {
        &self.hover_intersections
    }

    /// Clear only frame registrations. Active drag retains its own Arc.
    pub fn clear_widgets(&self) {
        self.widgets.write().clear();
        self.clearing_canvases.write().clear();
    }

    pub fn replace_interaction_full(
        &mut self,
        interaction: UiInteraction,
        params: Option<Arc<WidgetParams>>,
        payload: [u8; 56],
    ) {
        self.interaction = interaction;
        self.active_params = params;
        self.replace_intersection(payload);
    }

    pub fn active_params(&self) -> Option<Arc<WidgetParams>> {
        self.active_params.as_ref().map(Arc::clone)
    }

    /// `UiState::add<&str>`'''),
('        let id = WidgetId::clearing_canvas(name);','''        let id = WidgetId::clearing_canvas(name);
        // Same original params entry as any other widget; the additional
        // compatibility view below must not hide the canvas from real picking.
        self.add_with_id(
            id,
            WidgetParams::new(
                super::widget_params::WidgetVolumeShape::clearing_canvas(layer),
                [1 | ((argument_79 as u64) << 8), 0],
            ).occlude_by_world(false),
        );'''),
('''        let id = WidgetId::clearing_canvas(name);
        // Same original params entry''','''        self.clearing_canvas_with_sounds(name, argument_79, layer, super::widget_params::UiSounds::empty())
    }

    /// Complete native14092df50 caller contract. The original copies all96B
    /// supplied sounds and writes params+a8/a9=0x00/0x01 at14092e0e2.
    pub fn clearing_canvas_with_sounds(
        &self,
        name: &str,
        argument_79: u8,
        layer: u8,
        sounds: super::widget_params::UiSounds,
    ) -> UiWidgetContext<'_> {
        let id = WidgetId::clearing_canvas(name);
        // Same original params entry'''),
('''            ).occlude_by_world(false),''','''            ).sounds(sounds).occlude_by_world(false),'''),
('impl UiWidgetContext<\'_> {','''impl UiWidgetContext<'_> {
    /// B14092e640 returns the full intersection iff hover!=0 and id matches.
    /// The None variant writes only tag2; remaining bytes below are safe owned
    /// unused storage, not a fabricated intersection payload.
    pub fn intersecting(&self) -> [u8; 56] {
        let active = self.state.interaction;
        if active.native_tag != 0 && active.widget == self.id {
            self.state.intersection_payload
        } else {
            let mut none = [0; 56];
            none[..4].copy_from_slice(&2u32.to_le_bytes());
            none
        }
    }
'''),
])
(OUT/'candidate-bases.json').write_text(json.dumps(manifest,indent=2),encoding='utf8')
print(json.dumps(manifest,indent=2))
