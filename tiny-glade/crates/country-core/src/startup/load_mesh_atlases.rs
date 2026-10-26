//! B mesh-atlas producer, using current canonical Mesh/MeshAtlasLibrary owners.
//! RNE 1415f7190/1415f5b60 load the same typed RON schema.
//! These producers reuse the current Mesh/MeshAtlasLibrary and JSON loader.
//! Complete initial/reload wrappers consume the same canonical prefab parts.
use crate::asset_libraries::mesh_library::AssetMeshLibrary;
use crate::resources::prefab::PrefabLibrary;
use crate::render::mesh::{Mesh, VertexAttributeValues};
use crate::render::nani::gpu_world::GpuWorldSubsetKind;
use crate::resources::render::mesh_atlas_library::{MeshAtlasLibrary, MeshAtlasesModifiedCmd, PrefabName};
use crate::startup::load_prefabs::PrefabWatcher;
use anyhow::Context;
use glam::{Vec2, Vec3, Vec4};
use serde::Deserialize;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

/// Original enum visitor 1415f8c40: exact I32/F32/F32x2/F32x3/F32x4 names.
#[derive(Clone, Copy, Debug, Deserialize)]
pub enum MeshAttribType {
    I32,
    F32,
    F32x2,
    F32x3,
    F32x4,
}
#[derive(Debug, Deserialize)]
pub struct MeshAttribAdd {
    pub name: String,
    pub ty: MeshAttribType,
}
#[derive(Default, Debug, Deserialize)]
#[serde(default)]
pub struct MeshAttribConfig {
    pub remove: Vec<String>,
    pub add: Vec<MeshAttribAdd>,
}
#[derive(Debug, Deserialize)]
pub struct MeshConfig {
    pub name: String,
    #[serde(default)]
    pub skip_in_demo: bool,
}
#[derive(Debug, Deserialize)]
pub struct NaniWorldSubsetConfig {
    pub subset: GpuWorldSubsetKind,
    #[serde(default)]
    pub attribs: MeshAttribConfig,
    pub meshes: Vec<MeshConfig>,
}

/// Original removal precedes additions. The element count comes from an
/// arbitrary existing attribute BEFORE removal; additions never replace an
/// already present attribute. Actual MeshAtlas validates the resulting schema.
pub fn apply_mesh_attributes(mesh: &mut Mesh, config: &MeshAttribConfig) {
    let count = mesh
        .attributes
        .values()
        .next()
        .map_or(0, VertexAttributeValues::array_length);
    for name in &config.remove {
        mesh.attributes.remove(name);
    }
    for add in &config.add {
        if mesh.attributes.contains_key(&add.name) {
            continue;
        }
        let data = match add.ty {
            MeshAttribType::I32 => VertexAttributeValues::Int(vec![0; count]),
            MeshAttribType::F32 => VertexAttributeValues::Float(vec![0.0; count]),
            MeshAttribType::F32x2 => VertexAttributeValues::Float2(vec![Vec2::ZERO; count]),
            MeshAttribType::F32x3 => VertexAttributeValues::Float3(vec![Vec3::ZERO; count]),
            MeshAttribType::F32x4 => VertexAttributeValues::Float4(vec![Vec4::ZERO; count]),
        };
        // The original set_attribute<Vec<T>> rejects empty arrays.
        assert!(count != 0, "assertion failed: !v.is_empty()");
        mesh.set_attribute(&add.name, data);
    }
}

/// Shared original RON/mesh producer; explicit root/demo are current host
/// inputs replacing a process-relative CWD and the original global demo byte.
pub fn load_nani_meshes(
    installation: &Path,
    atlases: &mut MeshAtlasLibrary,
    watcher: Option<&mut PrefabWatcher>,
    demo: bool,
) -> anyhow::Result<()> {
    load_nani_meshes_if(installation, atlases, watcher, demo, |_| true)
}

/// Same original RON loader with the reload closure@1415f5b60. The predicate
/// is evaluated before the RoofFloor assertion or any JSON read/watch/add.
fn load_nani_meshes_if(
    installation: &Path,
    atlases: &mut MeshAtlasLibrary,
    mut watcher: Option<&mut PrefabWatcher>,
    demo: bool,
    mut selected: impl FnMut(GpuWorldSubsetKind) -> bool,
) -> anyhow::Result<()> {
    let source = installation.join("assets/nani_meshes.ron");
    let configs: Vec<NaniWorldSubsetConfig> = ron::de::from_reader(BufReader::new(
        File::open(&source).context("open nani_meshes.ron")?,
    ))
    .context("deserialize nani_meshes.ron")?;
    for config in configs {
        if !selected(config.subset) {
            continue;
        }
        assert_ne!(
            config.subset,
            GpuWorldSubsetKind::RoofFloor,
            "RoofFloor meshes are generated procedurally"
        );
        for mesh in config.meshes {
            if demo && mesh.skip_in_demo {
                continue;
            }
            let path = installation
                .join("assets/meshes")
                .join(format!("{}.json", mesh.name));
            let path_text = path.to_str().context("Mesh path is not UTF-8")?;
            let mut data = crate::utils::load_json::try_load_json_as_mesh(path_text)?;
            if let Some(watcher) = watcher.as_deref_mut() {
                watcher.watch_atlas(&path, config.subset)?;
            }
            apply_mesh_attributes(&mut data, &config.attribs);
            atlases.add(config.subset, mesh.name.as_str(), &data);
        }
    }
    Ok(())
}

/// Atlas ordinals are assigned once in semantic prefab order. Parts retain
/// their canonical published order so every GPU/RT/audio consumer agrees.
fn add_prefab_meshes(
    prefabs: &PrefabLibrary,
    atlases: &mut MeshAtlasLibrary,
    mut selected: impl FnMut(&PrefabName, GpuWorldSubsetKind) -> bool,
) {
    let mut ordered: Vec<_> = prefabs.iter().collect();
    ordered.sort_unstable_by(|(left, _), (right, _)| left.0.cmp(&right.0));
    for (name, prefab) in ordered {
        for part in &prefab.parts {
            if !part.render_mesh.indices.is_empty() && selected(name, part.render_subset) {
                atlases.add(part.render_subset, name.clone(), &part.render_mesh);
            }
        }
    }
}

/// Original load_mesh_atlases@14129f400: runtime prefab render meshes first,
/// then RON meshes, five procedural families and register_meshes@1412a00ae.
/// This must run after load_prefabs and before startup_nani in PreStartup's
/// original outer four-node chain, with the same canonical asset owners.
pub fn load_mesh_atlases(
    installation: &Path,
    prefabs: &PrefabLibrary,
    atlases: &mut MeshAtlasLibrary,
    meshes: &mut AssetMeshLibrary,
    watcher: Option<&mut PrefabWatcher>,
    demo: bool,
) -> anyhow::Result<()> {
    add_prefab_meshes(prefabs, atlases, |_, _| true);
    load_nani_meshes(installation, atlases, watcher, demo)?;
    generate_procedural_meshes(atlases);
    atlases.register_meshes(meshes);
    Ok(())
}

/// Original reload_mesh_atlases@1412a0820. Changed atlas subsets and the
/// PrefabsModifiedCmd name owner are distinct inputs. No procedural rerun.
/// The returned event is emitted only after real register_meshes succeeds.
pub fn reload_mesh_atlases(
    installation: &Path,
    prefabs: &PrefabLibrary,
    atlases: &mut MeshAtlasLibrary,
    meshes: &mut AssetMeshLibrary,
    watcher: &mut PrefabWatcher,
    modified_prefabs: &HashSet<PrefabName>,
    demo: bool,
) -> anyhow::Result<Option<MeshAtlasesModifiedCmd>> {
    let mut changed = watcher.take_atlas_changes();
    // @1412a0b95..bd3: each modified prefab contributes all current part subsets.
    for name in modified_prefabs {
        for part in &prefabs.get(name).parts {
            changed.insert(part.render_subset);
        }
    }
    if changed.is_empty() && modified_prefabs.is_empty() {
        return Ok(None);
    }
    // All subset clears precede all name clears (@0cff, then @0d95).
    for &subset in &changed {
        atlases.clear_world_subset_atlas(subset);
    }
    for name in modified_prefabs {
        atlases.clear_atlas_by_name(name);
    }
    // @1153/117d..123e: subset hit OR prefab name hit; @119c tests
    // Arc header16 + Mesh.indices.len@10, not an attributes-table counter.
    add_prefab_meshes(prefabs, atlases, |name, subset|
        changed.contains(&subset) || modified_prefabs.contains(name));
    if !changed.is_empty() {
        load_nani_meshes_for_subsets(installation, atlases, Some(watcher), demo, &changed)?;
    }
    // @1412a1512 runs for every nonempty input batch, even if changed is empty.
    atlases.register_meshes(meshes);
    Ok(Some(MeshAtlasesModifiedCmd { subsets: changed }))
}

/// Original reload RON predicate selects only affected subset entries before
/// assertion/I/O/mesh modification. This helper is independent of PrefabPart.
pub fn load_nani_meshes_for_subsets(
    installation: &Path,
    atlases: &mut MeshAtlasLibrary,
    watcher: Option<&mut PrefabWatcher>,
    demo: bool,
    subsets: &HashSet<GpuWorldSubsetKind>,
) -> anyhow::Result<()> {
    load_nani_meshes_if(installation, atlases, watcher, demo, |kind| {
        subsets.contains(&kind)
    })
}

/// Five original procedural families inside load_mesh_atlases@14129f400.
/// No dummy vertex attributes are added to the encoded-index-only families.
pub fn generate_procedural_meshes(atlases: &mut MeshAtlasLibrary) {
    use crate::startup::startup_terrain::{grid_gen_ccw, grid_gen_cw};
    for level in 0..=5u32 {
        let n = 1u32 << level;
        let mut positions = Vec::with_capacity((2 * (n + 1)) as usize);
        for i in 0..=n {
            let z = i as f32 / n as f32 - 0.5;
            positions.push(Vec3::new(-0.5, 0.0, z));
            positions.push(Vec3::new(0.5, 0.0, z));
        }
        if level != 0 {
            for p in &mut positions {
                p.z = (p.z * f32::from_bits(0x40490fdb)).sin() * 0.5;
                let z2 = p.z + p.z;
                p.x = (1.0 - z2 * z2).clamp(0.0, 1.0).sqrt() * p.x.signum() * 0.5;
            }
        }
        let mut mesh = Mesh::new("grid cw");
        grid_gen_cw(&mut mesh.indices, 0, 1, n, 1);
        mesh.set_attribute("Vertex_Position", positions);
        atlases.add(
            GpuWorldSubsetKind::RoofFloor,
            format!("stone_floor_{level}").as_str(),
            &mesh,
        );
    }
    // Constant pool values are independently read in procedural-mesh-constants.json.
    let xs = [
        0.0,
        f32::from_bits(0x3e2aaaab),
        f32::from_bits(0x3eaaaaab),
        0.5,
        f32::from_bits(0x3f2aaaab),
        f32::from_bits(0x3f555555),
        1.0,
    ];
    for level in 0..=5u32 {
        let n = 1u32 << level;
        let mut positions = Vec::with_capacity((7 * (n + 1)) as usize);
        for i in 0..=n {
            let z = i as f32 / n as f32;
            positions.extend(xs.into_iter().map(|x| Vec3::new(x, 0.0, z)));
        }
        let mut mesh = Mesh::new("grid ccw");
        grid_gen_ccw(&mut mesh.indices, 0, 6, n, 6);
        mesh.set_attribute("Vertex_Position", positions);
        atlases.add(
            GpuWorldSubsetKind::CurvedHalfTimber,
            format!("timber_{level}").as_str(),
            &mesh,
        );
    }
    let mut mesh = Mesh::new("plaster");
    for i in 0..32u32 {
        mesh.indices
            .extend_from_slice(&[i | 0x10000, i, i + 1, i + 1, i + 0x10001, i | 0x10000]);
    }
    atlases.add(GpuWorldSubsetKind::Plaster, "plaster_strip", &mesh);
    for n in [4, 8, 16, 32, 64] {
        let mut mesh = Mesh::new("stairs floor");
        for i in 0..n {
            mesh.indices
                .extend_from_slice(&[3 * i, 3 * i + 1, 3 * i + 2]);
        }
        atlases.add(
            GpuWorldSubsetKind::StairsFloor,
            format!("stairs_floor_chunk_{n}").as_str(),
            &mesh,
        );
    }
    let mut mesh = Mesh::new("grid cw");
    grid_gen_cw(&mut mesh.indices, 0, 12, 50, 12);
    atlases.add(GpuWorldSubsetKind::Smoke, "smoke", &mesh);
}

#[cfg(test)]
mod deterministic_prefab_atlas_tests {
    use super::*;
    use crate::resources::prefab::{Prefab, PrefabPart};
    use std::sync::Arc;

    #[test]
    fn initial_and_selected_atlas_assembly_share_stable_semantic_ordinals() {
        let kind = GpuWorldSubsetKind::Canopy;
        for order in [[2, 0, 1], [1, 2, 0], [0, 1, 2], [2, 1, 0]] {
            let mut prefabs = PrefabLibrary::default();
            for index in order {
                let mut mesh = Mesh::new("fixture");
                mesh.indices = vec![0, 1, 2];
                mesh.set_attribute("Vertex_Position", vec![Vec3::new(index as f32, 0.0, 0.0),
                    Vec3::new(index as f32, 1.0, 0.0), Vec3::new(index as f32, 0.0, 1.0)]);
                prefabs.add(PrefabName::new(["a", "m", "z"][index]), Prefab { parts: vec![PrefabPart {
                    name: "body".into(), render_mesh: Arc::new(mesh), render_subset: kind,
                    ..Default::default()
                }] });
            }
            for selected in [false, true] {
                let mut atlases = MeshAtlasLibrary::default();
                add_prefab_meshes(&prefabs, &mut atlases, |_, subset| !selected || subset == kind);
                for (index, name) in ["a", "m", "z"].iter().enumerate() {
                    assert_eq!(atlases.get_by_name(&PrefabName::new(name))[0].part_index, index as u32);
                }
                let atlas = &atlases.get_by_world_subset(kind).atlas;
                assert_eq!(atlas.layouts.iter().map(|layout| layout.base_index).collect::<Vec<_>>(), [0, 3, 6]);
                assert_eq!(atlas.mesh.indices, [0, 1, 2, 3, 4, 5, 6, 7, 8]);
                let positions = atlas.mesh.positions_without_indices();
                assert_eq!([positions[0].x, positions[3].x, positions[6].x], [0.0, 1.0, 2.0]);
            }
        }
    }
}
