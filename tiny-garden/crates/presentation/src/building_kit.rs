//! Offline-compiled DCC geometry. No asset handles or renderer types are needed.
//! Instantiation merges all attachments by semantic material, never per-window entities.
use crate::catalog::CatalogError;
use garden_domain::Roof;
use garden_generation::incremental::{
    Cancellation, PartInput, PartKind, PreparedBuilding, plan, reconcile,
};
use garden_generation::mesh::{
    BuildingMesh, GeometryProfile, MaterialKey, MeshBatch, MeshData, MeshError,
    compile_mesh_with_frames,
};
use garden_generation::{BlockLayout, BuildingLayout, Face, PartKey};
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct KitFile {
    schema_version: u32,
    units: String,
    up_axis: String,
    front_axis: String,
    window_opening: [f32; 2],
    modules: BTreeMap<String, ModuleFile>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ModuleFile {
    batches: BTreeMap<String, BatchFile>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct BatchFile {
    positions: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,
    indices: Vec<u32>,
    fit: Vec<String>,
}
#[derive(Clone)]
struct KitBatch {
    material: MaterialKey,
    mesh: MeshData,
    fit: Vec<bool>,
}
#[derive(Clone)]
pub struct BuildingKit {
    modules: BTreeMap<String, Vec<KitBatch>>,
}

impl BuildingKit {
    pub fn parse(json: &str) -> Result<Self, CatalogError> {
        if json.len() > 2 * 1024 * 1024 {
            return Err(CatalogError("kit exceeds 2 MiB".into()));
        }
        let file: KitFile = serde_json::from_str(json).map_err(|e| CatalogError(e.to_string()))?;
        if file.schema_version != 1
            || file.units != "meter"
            || file.up_axis != "+Y"
            || file.front_axis != "+Z"
            || file.window_opening != [0.9, 1.2]
        {
            return Err(CatalogError("unsupported kit contract".into()));
        }
        let mut modules = BTreeMap::new();
        for (name, triangles, material_count) in [
            ("window-basic", 500, 2),
            ("window-shutter", 800, 2),
            ("door-timber", 500, 2),
            ("chimney-stone", 500, 1),
        ] {
            let module = file
                .modules
                .get(name)
                .ok_or_else(|| CatalogError(format!("missing kit module {name}")))?;
            let mut batches = Vec::new();
            for (role, b) in &module.batches {
                let material = MaterialKey::ALL
                    .into_iter()
                    .find(|k| k.name() == role)
                    .ok_or_else(|| CatalogError("unknown kit material".into()))?;
                let mesh = MeshData {
                    positions: b.positions.clone(),
                    normals: b.normals.clone(),
                    uvs: b.uvs.clone(),
                    indices: b.indices.clone(),
                };
                mesh.validate().map_err(|e| CatalogError(e.to_string()))?;
                if mesh.positions.is_empty()
                    || mesh.positions.iter().flatten().any(|p| p.abs() > 3.0)
                    || b.fit.len() != mesh.positions.len()
                    || b.fit
                        .iter()
                        .any(|f| !["center", "border"].contains(&f.as_str()))
                {
                    return Err(CatalogError("invalid kit dimensions/fit".into()));
                }
                batches.push(KitBatch {
                    material,
                    mesh,
                    fit: b.fit.iter().map(|f| f == "center").collect(),
                });
            }
            if batches.len() != material_count
                || batches.iter().map(|b| b.mesh.triangles()).sum::<usize>() > triangles
            {
                return Err(CatalogError(format!("kit budget exceeded: {name}")));
            }
            if batches.iter().any(|b| {
                if name == "chimney-stone" {
                    b.material != MaterialKey::Stone
                } else {
                    !matches!(b.material, MaterialKey::Timber | MaterialKey::Window)
                }
            }) {
                return Err(CatalogError(
                    "module uses incompatible material roles".into(),
                ));
            }
            modules.insert(name.to_owned(), batches);
        }
        if file.modules.len() != modules.len() {
            return Err(CatalogError("unexpected kit module".into()));
        }
        Ok(Self { modules })
    }
    pub fn warm_stone() -> Self {
        Self::parse(include_str!(
            "../../../assets/themes/warm-stone/house-kit-v4/kit.json"
        ))
        .expect("shipped kit must validate")
    }
    pub fn compile(
        &self,
        layout: &mut BuildingLayout,
        profile: GeometryProfile,
    ) -> Result<BuildingMesh, MeshError> {
        // One real entrance aperture on a sufficiently wide ground-level front.
        // The SAME modified layout produces structure, attachments and picking.
        let mut entrance: Option<PartKey> = None;
        for block in &mut layout.blocks {
            if block.base_elevation == 0.0 && block.footprint.width >= 3.0 {
                if let Some(w) = block
                    .windows
                    .iter_mut()
                    .find(|w| w.key.face == Face::Front && w.key.story == 0 && w.key.slot == 0)
                {
                    w.width = 0.9;
                    w.height = 2.0;
                    w.elevation = 1.0;
                    entrance = Some(w.key);
                }
                break;
            }
        }
        let base = compile_mesh_with_frames(layout, profile, false)?;
        let mut batches = base
            .batches
            .into_iter()
            .map(|b| (b.material, b.data))
            .collect::<BTreeMap<_, _>>();
        for block in &layout.blocks {
            for w in &block.windows {
                let door = entrance == Some(w.key);
                let length = if matches!(w.key.face, Face::Front | Face::Back) {
                    block.footprint.width
                } else {
                    block.footprint.depth
                };
                let shutter = !door
                    && w.width >= 0.8
                    && w.along_wall > 0.95
                    && length - w.along_wall > 0.95
                    && (w.key.slot + u16::from(w.key.story)).is_multiple_of(2);
                let name = if door {
                    "door-timber"
                } else if shutter {
                    "window-shutter"
                } else {
                    "window-basic"
                };
                self.append(&mut batches, name, |p, center| {
                    let [mut x, mut y, z] = p;
                    if !door {
                        x = fit_axis(x, -0.45, 0.45, w.width as f32, center);
                        x += (0.9 - w.width as f32) / 2.;
                        y = fit_axis(y, 0., 1.2, w.height as f32, center);
                    }
                    wall_position(
                        block,
                        w.key.face,
                        w.along_wall as f32 - x,
                        w.elevation as f32 - w.height as f32 / 2. + y,
                        z,
                    )
                })?;
            }
            if block.effective_roof != Roof::Flat
                && block.footprint.width.min(block.footprint.depth) >= 2.0
            {
                let r = block.footprint;
                let half = (r.width.min(r.depth) / 2. + profile.eave) as f32;
                let rise = (half * (profile.roof_pitch_degrees as f32).to_radians().tan())
                    .min(profile.max_roof_rise as f32);
                // Ridge midpoint; base embedded 0.55m to avoid floating on slopes.
                let origin = [
                    (r.x + r.width / 2.) as f32,
                    (block.base_elevation + block.height) as f32 + rise - 0.55,
                    (r.z + r.depth / 2.) as f32,
                ];
                self.append(&mut batches, "chimney-stone", |p, _| {
                    std::array::from_fn(|i| p[i] + origin[i])
                })?;
            }
        }
        let result = BuildingMesh {
            batches: batches
                .into_iter()
                .map(|(material, data)| MeshBatch { material, data })
                .collect(),
        };
        for batch in &result.batches {
            batch.data.validate()?;
        }
        Ok(result)
    }
    /// Real production path: one wall face, roof, or floor/face attachment group
    /// at a time. No whole-house compile or whole-house merge on cache hits.
    pub fn compile_incremental(
        &self,
        layout: &mut BuildingLayout,
        profile: GeometryProfile,
        baseline: Option<&PreparedBuilding>,
        cancel: &Cancellation,
    ) -> Result<PreparedBuilding, MeshError> {
        cancel.check()?;
        for block in &mut layout.blocks {
            if block.base_elevation == 0. && block.footprint.width >= 3. {
                if let Some(w) = block
                    .windows
                    .iter_mut()
                    .find(|w| w.key.face == Face::Front && w.key.story == 0 && w.key.slot == 0)
                {
                    w.width = 0.9;
                    w.height = 2.;
                    w.elevation = 1.;
                }
                break;
            }
        }
        let mut inputs = plan(layout, profile)?;
        for input in &mut inputs {
            input.key.revision = (garden_generation::RULES_REVISION << 32) | 4;
        }
        reconcile(inputs, baseline, cancel, |input| {
            self.compile_part(input, cancel)
        })
    }
    fn compile_part(
        &self,
        input: &PartInput,
        cancel: &Cancellation,
    ) -> Result<BuildingMesh, MeshError> {
        let block = &input.key.block;
        let profile = input.key.profile;
        match input.id.kind {
            PartKind::Wall(face) => {
                return garden_generation::mesh::compile_structure_part(
                    block,
                    profile,
                    Some(face),
                    false,
                );
            }
            PartKind::Roof => {
                return garden_generation::mesh::compile_structure_part(
                    block, profile, None, false,
                );
            }
            _ => {}
        }
        let mut batches = BTreeMap::new();
        if input.id.kind == PartKind::Chimney {
            let r = block.footprint;
            let half = (r.width.min(r.depth) / 2. + profile.eave) as f32;
            let rise = (half * (profile.roof_pitch_degrees as f32).to_radians().tan())
                .min(profile.max_roof_rise as f32);
            let origin = [(r.width / 2.) as f32, rise - 0.55, (r.depth / 2.) as f32];
            self.append(&mut batches, "chimney-stone", |p, _| {
                std::array::from_fn(|i| p[i] + origin[i])
            })?;
        } else {
            for w in &block.windows {
                cancel.check()?;
                let door = input.id.kind == PartKind::Door;
                let length = if matches!(w.key.face, Face::Front | Face::Back) {
                    block.footprint.width
                } else {
                    block.footprint.depth
                };
                let shutter = !door
                    && w.width >= 0.8
                    && w.along_wall > 0.95
                    && length - w.along_wall > 0.95
                    && (w.key.slot + u16::from(w.key.story)).is_multiple_of(2);
                let name = if door {
                    "door-timber"
                } else if shutter {
                    "window-shutter"
                } else {
                    "window-basic"
                };
                self.append(&mut batches, name, |p, center| {
                    let [mut x, mut y, z] = p;
                    if !door {
                        x = fit_axis(x, -0.45, 0.45, w.width as f32, center);
                        x += (0.9 - w.width as f32) / 2.;
                        y = fit_axis(y, 0., 1.2, w.height as f32, center);
                    }
                    wall_position(
                        block,
                        w.key.face,
                        w.along_wall as f32 - x,
                        w.elevation as f32 - w.height as f32 / 2. + y,
                        z,
                    )
                })?;
            }
        }
        let result = BuildingMesh {
            batches: batches
                .into_iter()
                .map(|(material, data)| MeshBatch { material, data })
                .collect(),
        };
        for batch in &result.batches {
            cancel.check()?;
            batch.data.validate()?;
        }
        Ok(result)
    }
    fn append(
        &self,
        output: &mut BTreeMap<MaterialKey, MeshData>,
        name: &str,
        transform: impl Fn([f32; 3], bool) -> [f32; 3],
    ) -> Result<(), MeshError> {
        for batch in &self.modules[name] {
            let data = output.entry(batch.material).or_default();
            for tri in batch.mesh.indices.chunks_exact(3) {
                let points = std::array::from_fn::<_, 3, _>(|k| {
                    let i = tri[k] as usize;
                    transform(batch.mesh.positions[i], batch.fit[i])
                });
                let a = std::array::from_fn::<_, 3, _>(|i| points[1][i] - points[0][i]);
                let b = std::array::from_fn::<_, 3, _>(|i| points[2][i] - points[0][i]);
                let n = [
                    a[1] * b[2] - a[2] * b[1],
                    a[2] * b[0] - a[0] * b[2],
                    a[0] * b[1] - a[1] * b[0],
                ];
                let length = n.iter().map(|v| v * v).sum::<f32>().sqrt();
                if length < 1e-8 {
                    return Err(MeshError("degenerate fitted attachment"));
                }
                for (point, &i) in points.into_iter().zip(tri) {
                    data.indices.push(data.positions.len() as u32);
                    data.positions.push(point);
                    data.normals.push(n.map(|v| v / length));
                    data.uvs.push(batch.mesh.uvs[i as usize]);
                }
            }
        }
        Ok(())
    }
}
fn fit_axis(v: f32, min: f32, max: f32, length: f32, center: bool) -> f32 {
    if center || (min..=max).contains(&v) {
        min + (v - min) * length / (max - min)
    } else if v < min {
        v
    } else {
        v + length - (max - min)
    }
}
fn wall_position(block: &BlockLayout, face: Face, a: f32, y: f32, out: f32) -> [f32; 3] {
    let r = block.footprint;
    match face {
        Face::Front => [r.x as f32 + a, y, r.z as f32 - out],
        Face::Back => [r.right() as f32 - a, y, r.back() as f32 + out],
        Face::Left => [r.x as f32 - out, y, r.back() as f32 - a],
        Face::Right => [r.right() as f32 + out, y, r.z as f32 + a],
    }
}
#[cfg(feature = "desktop")]
impl garden_bevy::ProjectionCompiler for BuildingKit {
    fn compile(
        &self,
        layout: &mut BuildingLayout,
        profile: GeometryProfile,
    ) -> Result<BuildingMesh, MeshError> {
        BuildingKit::compile(self, layout, profile)
    }
    fn compile_parts(
        &self,
        layout: &mut BuildingLayout,
        profile: GeometryProfile,
        baseline: Option<&PreparedBuilding>,
        cancel: &Cancellation,
    ) -> Result<PreparedBuilding, MeshError> {
        self.compile_incremental(layout, profile, baseline, cancel)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use garden_domain::{Building, BuildingId, sample_building};
    use std::sync::Arc;
    fn incremental(
        kit: &BuildingKit,
        building: &Building,
        old: Option<&PreparedBuilding>,
    ) -> PreparedBuilding {
        kit.compile_incremental(
            &mut garden_generation::compile(building),
            GeometryProfile::default(),
            old,
            &Cancellation::default(),
        )
        .unwrap()
    }
    fn triangles(
        mesh: &PreparedBuilding,
        axes: &[Vec<f32>; 3],
    ) -> Vec<(MaterialKey, [[usize; 3]; 3])> {
        let mut tris = Vec::new();
        for b in &mesh.batches {
            for t in b.data.indices.chunks_exact(3) {
                let mut points = std::array::from_fn::<_, 3, _>(|i| {
                    std::array::from_fn(|a| {
                        let value = b.data.positions[t[i] as usize][a] + b.offset[a];
                        axes[a].partition_point(|v| *v + 0.0001 < value)
                    })
                });
                points.sort();
                tris.push((b.material, points));
            }
        }
        tris.sort();
        tris
    }
    fn equivalent(a: &PreparedBuilding, b: &PreparedBuilding) {
        // Normalize representational f32 differences from (local + transform)
        // vs baked coordinates. Do not quantize on a grid boundary: many bevel
        // points sit exactly on half-grid values and would round differently.
        let axes: [Vec<f32>; 3] = std::array::from_fn(|axis| {
            let mut points = a
                .batches
                .iter()
                .chain(&b.batches)
                .flat_map(|b| {
                    b.data
                        .positions
                        .iter()
                        .map(move |p| p[axis] + b.offset[axis])
                })
                .collect::<Vec<_>>();
            points.sort_by(f32::total_cmp);
            let mut unique = Vec::<f32>::new();
            for p in points {
                if unique.last().is_none_or(|v| p - *v > 0.0001) {
                    unique.push(p);
                }
            }
            unique
        });
        let a = triangles(a, &axes);
        let b = triangles(b, &axes);
        assert_eq!(a.len(), b.len(), "triangle count");
        let mismatch = a.iter().zip(&b).find(|(a, b)| a != b);
        assert!(mismatch.is_none(), "first different triangle: {mismatch:?}");
    }
    #[test]
    fn incremental_geometry_matches_full_oracle_all_roofs_sizes_and_stacking() {
        use garden_domain::{BlockDraft, BlockId, Facade, Stories};
        let kit = BuildingKit::warm_stone();
        for roof in [Roof::Gabled, Roof::Hipped, Roof::Flat] {
            for (width, height) in [(1.2, 3.), (3., 3.), (6., 6.), (10., 12.), (100., 12.)] {
                let mut draft = sample_building(BuildingId::new(1).unwrap(), width, height);
                draft.blocks[0].roof_intent = roof;
                let b = Building::try_new(draft).unwrap();
                let full = kit
                    .compile(
                        &mut garden_generation::compile(&b),
                        GeometryProfile::default(),
                    )
                    .unwrap();
                equivalent(&incremental(&kit, &b, None), &PreparedBuilding::whole(full));
            }
        }
        let mut draft = sample_building(BuildingId::new(1).unwrap(), 10., 3.);
        draft.blocks.push(BlockDraft {
            id: BlockId::new(2).unwrap(),
            parent: Some(BlockId::new(1).unwrap()),
            footprint: garden_geometry::Rect {
                x: 2.,
                z: 1.,
                width: 5.,
                depth: 4.,
            },
            height: 6.,
            stories: Stories::Locked(2),
            roof_intent: Roof::Hipped,
            facade: Facade::Timber,
        });
        let b = Building::try_new(draft).unwrap();
        let full = kit
            .compile(
                &mut garden_generation::compile(&b),
                GeometryProfile::default(),
            )
            .unwrap();
        equivalent(&incremental(&kit, &b, None), &PreparedBuilding::whole(full));
    }
    #[test]
    fn move_reuses_every_buffer_and_roof_edit_retains_walls_and_windows() {
        use garden_domain::{BlockId, BuildingEdit, Placement};
        let kit = BuildingKit::warm_stone();
        let b = Building::try_new(sample_building(BuildingId::new(1).unwrap(), 6., 6.)).unwrap();
        let first = incremental(&kit, &b, None);
        let moved = b
            .edited(BuildingEdit::Move(Placement {
                x: 8.,
                z: -7.,
                yaw: 0.7,
                elevation: 0.,
            }))
            .unwrap();
        let next = incremental(&kit, &moved, Some(&first));
        assert_eq!(next.rebuilt_parts, 0);
        assert_eq!(next.generated_vertices, 0);
        assert!(
            next.batches
                .iter()
                .zip(&first.batches)
                .all(|(a, b)| Arc::ptr_eq(&a.data, &b.data))
        );
        let roof = moved
            .edited(BuildingEdit::SetRoof {
                block: BlockId::new(1).unwrap(),
                roof: Roof::Flat,
            })
            .unwrap();
        let third = incremental(&kit, &roof, Some(&next));
        assert_eq!(third.rebuilt_parts, 1);
        for batch in &third.batches {
            if batch.id.part.kind != PartKind::Roof {
                let old = next.batches.iter().find(|b| b.id == batch.id).unwrap();
                assert!(Arc::ptr_eq(&batch.data, &old.data));
            }
        }
    }
    #[test]
    fn height_updates_positions_reuses_art_and_propagates_support_offsets() {
        use garden_domain::{BlockDraft, BlockId, BuildingEdit, Facade, Stories};
        let kit = BuildingKit::warm_stone();
        let mut draft = sample_building(BuildingId::new(1).unwrap(), 10., 6.);
        draft.blocks[0].stories = Stories::Locked(2);
        draft.blocks.push(BlockDraft {
            id: BlockId::new(2).unwrap(),
            parent: Some(BlockId::new(1).unwrap()),
            footprint: garden_geometry::Rect {
                x: 2.,
                z: 1.,
                width: 5.,
                depth: 4.,
            },
            height: 6.,
            stories: Stories::Locked(2),
            roof_intent: Roof::Hipped,
            facade: Facade::Timber,
        });
        let b = Building::try_new(draft).unwrap();
        let first = incremental(&kit, &b, None);
        let changed = b
            .edited(BuildingEdit::Resize {
                block: BlockId::new(1).unwrap(),
                footprint: b.blocks()[0].footprint,
                height: 8.,
            })
            .unwrap();
        let next = incremental(&kit, &changed, Some(&first));
        assert_eq!(next.rebuilt_parts, 4); // lower walls only, not art or upper tier
        let mut moved_windows = 0;
        for batch in &next.batches {
            let old = first.batches.iter().find(|b| b.id == batch.id).unwrap();
            if batch.id.part.block == BlockId::new(2).unwrap() {
                assert!(Arc::ptr_eq(&batch.data, &old.data));
                assert!((batch.offset[1] - old.offset[1] - 2.).abs() < 1e-5);
            } else if matches!(batch.id.part.kind, PartKind::Windows(..)) {
                assert!(Arc::ptr_eq(&batch.data, &old.data));
                assert_ne!(batch.offset, old.offset);
                moved_windows += 1;
            }
        }
        assert!(moved_windows > 0);
        let full = kit
            .compile(
                &mut garden_generation::compile(&changed),
                GeometryProfile::default(),
            )
            .unwrap();
        equivalent(&next, &PreparedBuilding::whole(full));
        let restored = incremental(&kit, &b, Some(&next));
        equivalent(&restored, &first);
    }
    #[test]
    fn cancellation_and_geometry_key_changes_never_reuse_invalid_parts() {
        let kit = BuildingKit::warm_stone();
        let b = Building::try_new(sample_building(BuildingId::new(1).unwrap(), 6., 6.)).unwrap();
        let first = incremental(&kit, &b, None);
        let cancel = Cancellation::default();
        cancel.cancel();
        assert!(
            kit.compile_incremental(
                &mut garden_generation::compile(&b),
                GeometryProfile::default(),
                Some(&first),
                &cancel
            )
            .is_err()
        );
        let changed = kit
            .compile_incremental(
                &mut garden_generation::compile(&b),
                GeometryProfile {
                    wall_thickness: 0.3,
                    ..GeometryProfile::default()
                },
                Some(&first),
                &Cancellation::default(),
            )
            .unwrap();
        assert_eq!(changed.reused_parts, 0);
        for batch in &changed.batches {
            assert!(!Arc::ptr_eq(
                &batch.data,
                &first
                    .batches
                    .iter()
                    .find(|b| b.id == batch.id)
                    .unwrap()
                    .data
            ));
        }
    }
    #[test]
    fn real_kit_fits_small_large_multistorey_and_all_roofs_in_five_batches() {
        let kit = BuildingKit::warm_stone();
        for roof in [Roof::Gabled, Roof::Hipped, Roof::Flat] {
            for (width, height) in [(1.2, 3.), (3., 3.), (6., 6.), (10., 12.)] {
                let mut draft = sample_building(BuildingId::new(1).unwrap(), width, height);
                draft.blocks[0].roof_intent = roof;
                let mut layout = garden_generation::compile(&Building::try_new(draft).unwrap());
                let mesh = kit
                    .compile(&mut layout, GeometryProfile::default())
                    .unwrap();
                assert!(mesh.batches.len() <= 5 && mesh.triangles() > 300);
                let door = layout.blocks[0]
                    .windows
                    .iter()
                    .find(|w| w.key.face == Face::Front && w.key.slot == 0 && w.key.story == 0)
                    .unwrap();
                assert_eq!(door.height == 2., width >= 3.);
            }
        }
    }
    #[test]
    fn malformed_assets_never_reach_async_jobs() {
        let mut data: serde_json::Value = serde_json::from_str(include_str!(
            "../../../assets/themes/warm-stone/house-kit-v4/kit.json"
        ))
        .unwrap();
        data["modules"]["window-basic"]["batches"]["timber"]["indices"][0] = 999999.into();
        assert!(BuildingKit::parse(&data.to_string()).is_err());
    }
    #[test]
    fn fixed_corner_profile_and_attachment_face_preserve_orientation() {
        assert_eq!(fit_axis(-0.55, -0.45, 0.45, 0.6, false), -0.55);
        assert!((fit_axis(0.55, -0.45, 0.45, 0.6, false) - 0.25).abs() < 1e-6);
        // Re-centering fitted windows is applied by the caller, not fit_axis.
    }
    #[test]
    fn upper_tier_and_terrace_keep_door_at_ground_and_only_top_roof_gets_chimney() {
        use garden_domain::{BlockDraft, BlockId, Facade, Stories};
        let mut draft = sample_building(BuildingId::new(1).unwrap(), 10., 3.);
        draft.blocks.push(BlockDraft {
            id: BlockId::new(2).unwrap(),
            parent: Some(BlockId::new(1).unwrap()),
            footprint: garden_geometry::Rect {
                x: 2.,
                z: 1.,
                width: 5.,
                depth: 4.,
            },
            height: 6.,
            stories: Stories::Locked(2),
            roof_intent: Roof::Hipped,
            facade: Facade::Timber,
        });
        let mut layout = garden_generation::compile(&Building::try_new(draft).unwrap());
        let mesh = BuildingKit::warm_stone()
            .compile(&mut layout, GeometryProfile::default())
            .unwrap();
        assert_eq!(layout.blocks[0].effective_roof, Roof::Flat);
        assert!(!layout.blocks[0].terraces.is_empty());
        assert!(layout.blocks[1].windows.iter().all(|w| w.height != 2.));
        assert!(mesh.batches.len() <= 5);
        for b in &mesh.batches {
            b.data.validate().unwrap();
        }
    }
}
