//! 预制件配置扫描和热更新。监听回调只入队；文件 I/O 在锁外执行。
//! 原 reload 只更新新条目和本批已通知条目；旧条目不会因磁盘删除而自动移除。
//! Original mesh processors, render Arc/subset, and independent RT/audio bounds
//! use the current canonical resource and mesh owners.

use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use glam::Vec3;

use crate::render::mesh::{Mesh, VertexAttributeValues};
use crate::render::nani::gpu_world::GpuWorldSubsetKind;
use crate::resources::prefab::{Prefab, PrefabAudioTrigger, PrefabLibrary, PrefabPart};
use crate::resources::render::mesh_atlas_library::PrefabName;
use crate::utils_contract::Aabb3;

/// 文件监听防抖时间。
pub const WATCH_DELAY: Duration = Duration::from_millis(100);

/// 预制件和图集的变更集合；同一资源的重复通知会自动合并。
#[derive(bevy_ecs::resource::Resource)]
pub struct PrefabWatcher {
    /// 关闭监听时不创建操作系统 watcher。
    watcher: Option<hotwatch::Hotwatch>,
    /// 等待重载的预制件名称。
    pub prefab_changes: Arc<Mutex<HashSet<PrefabName>>>,
    /// 等待重建的图集子集。
    pub atlas_changes: Arc<Mutex<HashSet<GpuWorldSubsetKind>>>,
}

impl Default for PrefabWatcher {
    fn default() -> Self {
        Self::new(false)
    }
}

impl PrefabWatcher {
    /// 建立可选监听；需要处理平台初始化错误的宿主可以使用 try_new。
    pub fn new(watching: bool) -> Self {
        Self::try_new(watching).expect("无法启动预制件文件监听")
    }

    /// 复用 hotwatch 的平台实现，不自行创建轮询线程。
    pub fn try_new(watching: bool) -> Result<Self, hotwatch::Error> {
        let watcher = if watching {
            Some(hotwatch::Hotwatch::new_with_custom_delay(WATCH_DELAY)?)
        } else {
            None
        };
        Ok(Self {
            watcher,
            prefab_changes: Arc::new(Mutex::new(HashSet::new())),
            atlas_changes: Arc::new(Mutex::new(HashSet::new())),
        })
    }

    /// 监听源文件；命中事件时合并其依赖预制件名称。
    pub fn watch_prefab(
        &mut self,
        path: impl AsRef<Path>,
        names: Vec<PrefabName>,
    ) -> Result<(), hotwatch::Error> {
        let Some(watcher) = &mut self.watcher else {
            return Ok(());
        };
        let changes = Arc::clone(&self.prefab_changes);
        watcher.watch(path, move |event| {
            if is_relevant_event(&event) {
                let mut set = changes.lock().expect("prefab watcher mutex poisoned");
                set.extend(names.iter().cloned());
            }
        })
    }

    /// 监听图集源文件；图集通知直接交给宿主重建，无需重读 RON。
    pub fn watch_atlas(
        &mut self,
        path: impl AsRef<Path>,
        kind: GpuWorldSubsetKind,
    ) -> Result<(), hotwatch::Error> {
        let Some(watcher) = &mut self.watcher else {
            return Ok(());
        };
        let changes = Arc::clone(&self.atlas_changes);
        watcher.watch(path, move |event| {
            if is_relevant_event(&event) {
                let mut set = changes.lock().expect("prefab watcher mutex poisoned");
                set.insert(kind);
            }
        })
    }

    /// Original load_prefabs_impl drains only the prefab-name owner before I/O.
    /// New notifications arriving during the scan remain in the fresh set.
    pub fn take_prefab_changes(&self) -> HashSet<PrefabName> {
        std::mem::take(
            &mut *self
                .prefab_changes
                .lock()
                .expect("prefab watcher mutex poisoned"),
        )
    }

    /// The independent atlas stage owns this queue; prefab reload never drains it.
    pub fn take_atlas_changes(&self) -> HashSet<GpuWorldSubsetKind> {
        std::mem::take(
            &mut *self
                .atlas_changes
                .lock()
                .expect("atlas watcher mutex poisoned"),
        )
    }
}

/// 支持普通写入与编辑器的临时文件 rename 原子保存。
pub fn is_relevant_event(event: &hotwatch::Event) -> bool {
    matches!(
        event,
        hotwatch::Event::Write(_)
            | hotwatch::Event::Create(_)
            | hotwatch::Event::Rename(_, _)
            | hotwatch::Event::Remove(_)
            | hotwatch::Event::Rescan
    )
}

/// Original reload_prefabs@1412a1a60 checks only prefab_changes, then lets
/// load_prefabs_impl consume that batch. The output contains actually rebuilt
/// names; deleted files are absent from the scan and retain their old entries.
/// A host Result propagates the original failure; it does not invent rollback
/// or notification retry (the native error path panics after draining).
pub fn reload_prefabs(
    root: &Path,
    watcher: &mut PrefabWatcher,
    library: &mut PrefabLibrary,
) -> anyhow::Result<HashSet<PrefabName>> {
    if watcher
        .prefab_changes
        .lock()
        .expect("prefab watcher mutex poisoned")
        .is_empty()
    {
        return Ok(HashSet::new());
    }
    let changed = watcher.take_prefab_changes();
    load_prefabs_selected(root, library, Some(watcher), true, &changed)
}

/// Initial producer, sharing the same per-entry conversion as hot reload.
/// It adds/replaces scanned entries; it does not clear the existing library.
pub fn load_prefabs_impl(root: &Path, library: &mut PrefabLibrary) -> anyhow::Result<()> {
    load_prefabs_selected(root, library, None, false, &HashSet::new())?;
    Ok(())
}

/// Original filter closure@140a04cf0: !reload || !library.has(name) ||
/// changed.contains(name). Preserve the current canonical library ownership.
fn load_prefabs_selected(
    root: &Path,
    library: &mut PrefabLibrary,
    mut watcher: Option<&mut PrefabWatcher>,
    reload: bool,
    changed: &HashSet<PrefabName>,
) -> anyhow::Result<HashSet<PrefabName>> {
    let mut scan = asset::prefab::scan::scan_asset_tree(root)?;
    let mesh_root = root
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Prefab root has no assets parent"))?
        .join("meshes");
    let mut source_meshes = HashMap::<String, Arc<Mesh>>::new();
    let mut processed_meshes = HashMap::<(String, String), Arc<Mesh>>::new();
    let mut modified = HashSet::new();
    let mut replacements = Vec::new();
    for (name, config) in scan.prefabs {
        let runtime_name = PrefabName::new(name.as_str());
        if reload && library.has(runtime_name.as_name_ref()) && !changed.contains(&runtime_name) {
            continue;
        }
        let mut parts = Vec::new();
        for (part_name, part_config) in &config.parts {
            if *part_config.delete.as_concrete() {
                continue;
            }
            let render = part_config.render.as_concrete();
            let mesh_name = render.mesh.as_concrete();
            let source_path = mesh_root.join(format!("{mesh_name}.json"));
            scan.dependencies
                .add_file_to_prefab_dependency(&source_path, name.clone());
            let source = if let Some(mesh) = source_meshes.get(mesh_name) {
                mesh.clone()
            } else {
                let path_text = source_path
                    .to_str()
                    .ok_or_else(|| anyhow::anyhow!("Mesh path is not UTF-8"))?;
                let mesh = Arc::new(crate::utils::load_json::try_load_json_as_mesh(path_text)?);
                source_meshes.insert(mesh_name.clone(), mesh.clone());
                mesh
            };
            let mesh = match render.mesh_processor.as_concrete() {
                None => source,
                Some(asset::prefab::config::MeshProcessorConfig::Code { name: processor }) => {
                    let key = (mesh_name.clone(), processor.clone());
                    processed_meshes
                        .entry(key)
                        .or_insert_with(|| Arc::new(process_prefab_mesh(&source, processor)))
                        .clone()
                }
            };
            let subset_index = crate::startup::startup_nani::SHADER_BY_SUBSET
                .iter()
                .position(|shader| *shader == render.shader.as_concrete().as_str())
                .ok_or_else(|| {
                    anyhow::anyhow!("Unknown Nani shader {:?}", render.shader.as_concrete())
                })?;
            let render_subset = GpuWorldSubsetKind::ALL[subset_index];
            // Native RT and audio both query the final processed render mesh,
            // then apply their independently configured source-local scales.
            let (bounds, rt_kind, albedos) = if let Some(rt) = part_config.rt.as_concrete() {
                match rt.mesh.as_concrete() {
                    asset::prefab::config::RtMesh::Aabb => {}
                }
                let semantic = match rt.semantic.as_concrete() {
                    asset::prefab::config::RtMeshSemantic::None => 0,
                    asset::prefab::config::RtMeshSemantic::Canopy => 1,
                };
                (
                    mesh.aabb_bounds()
                        .scaled(Vec3::from_array(*rt.mesh_scale.as_concrete())),
                    semantic,
                    [
                        Vec3::from_array(*rt.summer_albedo.as_concrete()),
                        Vec3::from_array(*rt.autumn_albedo.as_concrete()),
                        Vec3::from_array(*rt.winter_albedo.as_concrete()),
                    ],
                )
            } else {
                // Only the None tag is active in the native part. Safe inactive
                // payload values here do not assert native uninitialized bytes.
                (
                    Aabb3 {
                        min: Vec3::ZERO,
                        max: Vec3::ZERO,
                    },
                    2,
                    [Vec3::ZERO; 3],
                )
            };
            let audio_trigger = part_config.audio.as_concrete().as_ref().map(|audio| {
                let trigger = audio.trigger.as_concrete();
                match trigger.shape.as_concrete() {
                    asset::prefab::config::AudioTriggerShape::Aabb => {}
                }
                PrefabAudioTrigger {
                    kind_name: trigger.kind.as_concrete().clone(),
                    bounds: mesh
                        .aabb_bounds()
                        .scaled(Vec3::from_array(*trigger.shape_scale.as_concrete())),
                }
            });
            parts.push(PrefabPart {
                name: part_name.clone(),
                audio_trigger,
                render_mesh: mesh,
                render_subset,
                rt_kind,
                albedos,
                bounds,
            });
        }
        modified.insert(runtime_name.clone());
        replacements.push((runtime_name, Prefab { parts }));
    }
    // The native selected map is fully materialized before its commit loop
    // @1412a4e80..4ebf calls PrefabLibrary::add for each selected value.
    for (name, prefab) in replacements {
        library.add(name, prefab);
    }
    if let Some(watcher) = watcher.as_deref_mut() {
        for (path, names) in scan.dependencies.by_file {
            watcher.watch_prefab(
                path,
                names
                    .iter()
                    .map(|name| PrefabName::new(name.as_str()))
                    .collect(),
            )?;
        }
    }
    Ok(modified)
}

/// Same producer plus the actual dependency owner; source JSON and inherited
/// directory config changes both identify the same runtime prefab names.
pub fn load_prefabs_with_watcher(
    root: &Path,
    library: &mut PrefabLibrary,
    mut watcher: Option<&mut PrefabWatcher>,
) -> anyhow::Result<()> {
    load_prefabs_selected(root, library, watcher.take(), false, &HashSet::new())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn watcher_starts_disabled_with_empty_sets() {
        let mut watcher = PrefabWatcher::new(false);
        assert!(watcher.watcher.is_none());
        assert!(
            watcher
                .watch_prefab("nope", vec![PrefabName::new("x")])
                .is_ok()
        );
        let prefabs = watcher.take_prefab_changes();
        let atlases = watcher.take_atlas_changes();
        assert!(prefabs.is_empty() && atlases.is_empty());
    }

    #[test]
    fn relevant_event_is_the_write_variant() {
        // O：原始谓词 `*event >> 1 == 0x4000000000000001`（具名变体映射登记）。
        let event = hotwatch::Event::Write(std::path::PathBuf::from("a.ron"));
        assert!(is_relevant_event(&event));
        assert!(is_relevant_event(&hotwatch::Event::Rescan));
    }

    #[test]
    fn load_prefabs_impl_loads_real_mesh_and_distinct_rt_audio_boxes() {
        let dir = std::env::temp_dir().join(format!("prefab_scan_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let root = dir.join("assets/prefabs");
        let meshes = dir.join("assets/meshes");
        std::fs::create_dir_all(&root).unwrap();
        std::fs::create_dir_all(&meshes).unwrap();
        std::fs::write(
            meshes.join("canopy.json"),
            r#"{
            "indices":{"buffer":[0,1,2]},"attributes":["Vertex_Position"],
            "Vertex_Position":{"type":["float",3],"buffer":[[0,1,-1],[2,5,3],[1,2,0]]}
        }"#,
        )
        .unwrap();
        std::fs::write(
            root.join("tree.ron"),
            r#"(
            parts: {"canopy": (
                render: (mesh: "canopy", shader: "tree_canopy"),
                rt: (mesh: Aabb, mesh_scale:[0.9,0.9,0.9], semantic: Canopy,
                     summer_albedo:[1.0,0.5,0.25]),
                audio: (trigger: (shape: Aabb, shape_scale:[0.8,1.0,0.8], kind: "Tree")),
            )},
        )"#,
        )
        .unwrap();
        let mut library = PrefabLibrary::default();
        load_prefabs_impl(&root, &mut library).unwrap();
        let _ = std::fs::remove_dir_all(&dir);
        let part = &library.get(&PrefabName::new("tree")).parts[0];
        assert_eq!(part.name, "canopy");
        assert_eq!(part.render_subset, GpuWorldSubsetKind::Canopy);
        assert_eq!(part.render_mesh.indices, [0, 1, 2]);
        assert_eq!(part.render_mesh.positions_without_indices().len(), 3);
        assert_eq!(part.rt_kind, 1);
        assert_eq!(part.albedos[0], Vec3::new(1.0, 0.5, 0.25));
        assert_eq!(
            part.bounds,
            Aabb3 {
                min: Vec3::new(0.0, 0.9, -0.9),
                max: Vec3::new(1.8, 4.5, 3.0f32 * 0.9)
            }
        );
        let audio = part.audio_trigger.as_ref().unwrap();
        assert!(audio.is_tree());
        assert_eq!(
            audio.bounds,
            Aabb3 {
                min: Vec3::new(0.0, 1.0, -0.8),
                max: Vec3::new(1.6, 5.0, 2.4)
            }
        );
    }
}

/// B 140a08680 / 140a08d30. Source requires RGB Float3; the predicate reads
/// only blue. Existing Mesh::retain_vertices performs original attribute
/// filtering and index remapping; do not implement a second remapper.
fn filter_mesh_by_vertex_color(mesh: &Mesh, canopy: bool) -> Mesh {
    let colors = match mesh.attributes.get("Vertex_Color") {
        Some(VertexAttributeValues::Float3(values)) => values,
        Some(_) => panic!("incorrect attribute type"),
        None => panic!("No Vertex_Color for mesh code processor"),
    };
    let mut keep = colors
        .iter()
        .map(|color| {
            i32::from(if canopy {
                color.z <= 0.0
            } else {
                color.z > 0.0
            })
        })
        .collect::<Vec<_>>();
    let mut output = mesh.clone();
    output.retain_vertices(&mut keep);
    output
}

fn float3_attribute<'a>(mesh: &'a Mesh, name: &str) -> &'a [Vec3] {
    match mesh.attributes.get(name).expect("no entry found for key") {
        VertexAttributeValues::Float3(values) => values,
        _ => panic!("incorrect attribute type"),
    }
}

/// B inlined 1412a2e51..39a8: prim_center's float BIT PATTERN is the quad key,
/// each quad accumulates original Vertex_Normal and all member indices. The
/// resulting normal is normalize_or_zero(sum); its center is mean(position).
/// This is not grouping by rounded coordinates or per-six-index triangles.
fn recalculate_canopy_quad_attribs(mesh: &mut Mesh) {
    struct Quad {
        members: Vec<u32>,
        normal: Vec3,
    }
    let centers = float3_attribute(mesh, "prim_center");
    let normals = float3_attribute(mesh, "Vertex_Normal");
    let positions = float3_attribute(mesh, "Vertex_Position");
    let mut quads = HashMap::<[u32; 3], Quad>::new();
    for (index, (center, normal)) in centers.iter().zip(normals).enumerate() {
        let key = center.to_array().map(f32::to_bits);
        let quad = quads.entry(key).or_insert_with(|| Quad {
            members: Vec::new(),
            normal: Vec3::ZERO,
        });
        quad.members.push(index as u32);
        quad.normal += *normal;
    }
    for quad in quads.values_mut() {
        quad.normal = quad.normal.normalize_or_zero();
    }
    let mut primitive_normals = vec![Vec3::ZERO; centers.len()];
    // Original assertion at 1412a368b precedes output expansion.
    assert_eq!(primitive_normals.len(), normals.len());
    for quad in quads.values() {
        for &index in &quad.members {
            primitive_normals[index as usize] = quad.normal;
        }
    }
    let mut primitive_centers = vec![Vec3::ZERO; centers.len()];
    for quad in quads.values() {
        let mut center = Vec3::ZERO;
        for &index in &quad.members {
            center += positions[index as usize];
        }
        center /= quad.members.len() as f32;
        for &index in &quad.members {
            primitive_centers[index as usize] = center;
        }
    }
    mesh.set_attribute("prim_normals", primitive_normals);
    mesh.set_attribute("prim_center", primitive_centers);
}

/// Original optional Code processor branches. Unknown names fail rather than
/// returning the raw input or an empty processed mesh.
fn process_prefab_mesh(source: &Mesh, processor: &str) -> Mesh {
    let canopy = match processor {
        "extract_canopy" => true,
        "extract_trunk" => false,
        _ => panic!("Unknown mesh code processor {processor:?}"),
    };
    let mut mesh = filter_mesh_by_vertex_color(source, canopy);
    // B count is read from one retained attribute before deleting id:
    // canopy saves it at1412a2ccc to[rbp+308]; trunk keeps RBX at1412a2e47.
    let count = mesh
        .attributes
        .values()
        .next()
        .map_or(0, VertexAttributeValues::array_length);
    if canopy && mesh.has_attribute("prim_center") {
        recalculate_canopy_quad_attribs(&mut mesh);
    }
    mesh.attributes.remove("id");
    // Original trunk count0 reaches1412a3a91(test RBX)/3a94(JE4121)
    // AFTER id removal, skipping only these two additions. The independent
    // canopy path uses saved[rbp+308] and has no equivalent count0 gate.
    // Keep Mesh::From<Vec3>'s genuine empty assertion; do not weaken it.
    if canopy || count != 0 {
        if !mesh.has_attribute("appear_pos") {
            mesh.set_attribute("appear_pos", vec![Vec3::ZERO; count]);
        }
        if !mesh.has_attribute("age") {
            mesh.set_attribute("age", vec![1.0f32; count]);
        }
    }
    mesh
}

#[cfg(test)]
mod native_empty_trunk_tests {
    use super::*;

    #[test]
    fn shipped_far_tree_empty_trunk_keeps_empty_attributes_and_removes_id() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../ljxsj_92385/Tiny Glade");
        let path = root.join("assets/meshes/far_distance_tree.json");
        let source =
            crate::utils::load_json::try_load_json_as_mesh(path.to_str().unwrap()).unwrap();
        let colors = float3_attribute(&source, "Vertex_Color");
        assert!(!colors.is_empty());
        assert!(colors.iter().all(|color| color.z == 0.0));
        assert!(source.has_attribute("id"));
        assert!(!source.has_attribute("appear_pos"));
        assert!(!source.has_attribute("age"));
        let trunk = process_prefab_mesh(&source, "extract_trunk");
        assert!(trunk.indices.is_empty());
        assert!(!trunk.attributes.is_empty());
        assert!(trunk.attributes.values().all(|v| v.array_length() == 0));
        assert!(!trunk.has_attribute("id"));
        assert!(!trunk.has_attribute("appear_pos"));
        assert!(!trunk.has_attribute("age"));
        assert_eq!(trunk.source_file, source.source_file);
        // The other real part remains nonempty; this is not a general return
        // of the unprocessed source or a blanket empty-attribute exception.
        let canopy = process_prefab_mesh(&source, "extract_canopy");
        assert_eq!(canopy.indices, source.indices);
        let count = float3_attribute(&canopy, "Vertex_Position").len();
        assert_eq!(count, colors.len());
        assert_eq!(
            float3_attribute(&canopy, "appear_pos"),
            vec![Vec3::ZERO; count]
        );
        assert!(matches!(&canopy.attributes["age"],
            VertexAttributeValues::Float(v) if v.len() == count && v.iter().all(|v| *v == 1.0)));
        assert!(!canopy.has_attribute("id"));
    }

    #[test]
    fn nonempty_trunk_still_adds_real_per_vertex_defaults() {
        let mut source = Mesh::new("nonempty trunk filter fixture");
        source.set_indices(vec![0, 1, 2]);
        source.set_attribute("Vertex_Position", vec![Vec3::ZERO, Vec3::X, Vec3::Y]);
        source.set_attribute("Vertex_Color", vec![Vec3::Z; 3]);
        source.set_attribute("id", vec![7i32; 3]);
        let trunk = process_prefab_mesh(&source, "extract_trunk");
        assert_eq!(trunk.indices, [0, 1, 2]);
        assert!(!trunk.has_attribute("id"));
        assert_eq!(float3_attribute(&trunk, "appear_pos"), [Vec3::ZERO; 3]);
        assert!(matches!(&trunk.attributes["age"],
            VertexAttributeValues::Float(v) if v == &[1.0, 1.0, 1.0]));
    }
}
