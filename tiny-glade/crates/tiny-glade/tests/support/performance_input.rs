//! One-time, test-only semantic inputs. Entity IDs, asset handles, GPU addresses
//! and filesystem modification times never participate in ordering or identity.
use anyhow::{Context, Result};
use bevy_ecs::{entity::Entity, world::World};
use country_core::{
    asset_libraries::{Handle, mesh_library::AssetMeshLibrary,
        shader_library::SharedAssetShaderLibrary, vao_library::AssetVAOLibrary},
    render::{mesh::{Mesh, VertexAttributeValues}, shader::RasterShaderProgram, vao::Vao,
        pass_builder::draw_list::DrawTopology},
    systems::render_loop::draw_passes::draw_lists::{DesignatedRenderPass, UiPass,
        WireframePass, DontCastShadow, OcclusionCulled},
    utils::components::transform::Transform,
};
use serde_json::{Value, json};

pub fn mesh_digest(mesh: &Mesh) -> String {
    fn word(hash: &mut blake3::Hasher, value: u32) { hash.update(&value.to_le_bytes()); }
    fn text(hash: &mut blake3::Hasher, value: &str) {
        hash.update(&(value.len() as u64).to_le_bytes()); hash.update(value.as_bytes());
    }
    let mut hash = blake3::Hasher::new();
    hash.update(&(mesh.indices.len() as u64).to_le_bytes());
    for &index in &mesh.indices { word(&mut hash, index); }
    let mut attributes: Vec<_> = mesh.attributes.iter().collect();
    attributes.sort_unstable_by(|a, b| a.0.cmp(b.0));
    for (name, values) in attributes {
        text(&mut hash, name);
        hash.update(&(values.array_length() as u64).to_le_bytes());
        match values {
            VertexAttributeValues::Int(values) => {
                word(&mut hash, 0); for &value in values { word(&mut hash, value as u32); }
            }
            VertexAttributeValues::Float(values) => {
                word(&mut hash, 1); for &value in values { word(&mut hash, value.to_bits()); }
            }
            VertexAttributeValues::Float2(values) => {
                word(&mut hash, 2); for value in values { for part in value.to_array() { word(&mut hash, part.to_bits()); } }
            }
            VertexAttributeValues::Float3(values) => {
                word(&mut hash, 3); for value in values { for part in value.to_array() { word(&mut hash, part.to_bits()); } }
            }
            VertexAttributeValues::Float4(values) => {
                word(&mut hash, 4); for value in values { for part in value.to_array() { word(&mut hash, part.to_bits()); } }
            }
        }
    }
    hash.finalize().to_hex().to_string()
}

pub struct Candidate {
    pub entity: Entity,
    pub manifest: Value,
    key: String,
}

pub fn candidates(world: &mut World) -> Result<Vec<Candidate>> {
    let values: Vec<_> = world.query::<(Entity, &Handle<Vao>, &Handle<RasterShaderProgram>,
        &Handle<Mesh>, &Transform)>().iter(world)
        .map(|(entity, vao, shader, mesh, transform)| (entity, *vao, *shader, *mesh, *transform)).collect();
    let shaders = world.resource::<SharedAssetShaderLibrary>();
    let meshes = world.resource::<AssetMeshLibrary>();
    let vaos = world.resource::<AssetVAOLibrary>();
    let mut digests = std::collections::HashMap::new();
    let mut result = Vec::new();
    for (entity, vao, shader, mesh, transform) in values {
        let source = meshes.get(mesh).context("benchmark source CPU mesh is absent")?;
        let vao = vaos.get_by_handle(vao).context("benchmark source VAO is absent")?;
        anyhow::ensure!(vao.index_count == source.indices.len(), "benchmark source VAO/mesh mismatch");
        let shader = shaders.0.get_raster(shader).context("benchmark source shader is absent")?;
        let mesh_hash = digests.entry(mesh).or_insert_with(|| mesh_digest(source));
        let entity_ref = world.entity(entity);
        let mut components: Vec<_> = entity_ref.archetype().components()
            .map(|id| world.components().get_info(id).unwrap().name().to_string()).collect();
        components.sort_unstable();
        let mut defines = shader.desc.defines.clone(); defines.sort_unstable();
        let topology = entity_ref.get::<DrawTopology>().map(|value| (value.tag(), value.payload().to_bits()));
        let manifest = json!({
            "shader":shader.name, "pipeline":{"path":shader.desc.path,"entry":shader.desc.entry,"defines":defines},
            "mesh":{"source":source.source_file.as_ref().map(|value| value.path.replace('\\', "/")),
                "label":source.label,"blake3":mesh_hash,"index_count":source.indices.len()},
            "transform_bits":transform.rotation.to_array().into_iter().chain(transform.translation.to_array())
                .chain(transform.scale.to_array()).map(f32::to_bits).collect::<Vec<_>>(),
            "topology":topology,
            "passes":{"designated":entity_ref.get::<DesignatedRenderPass>().map(|v|v.0),
                "ui":entity_ref.contains::<UiPass>(),"wireframe":entity_ref.contains::<WireframePass>(),
                "no_shadow":entity_ref.contains::<DontCastShadow>(),"occlusion":entity_ref.contains::<OcclusionCulled>()},
            "components":components,
        });
        let key = serde_json::to_string(&manifest)?;
        result.push(Candidate { entity, manifest, key });
    }
    result.sort_unstable_by(|a,b|a.key.cmp(&b.key));
    Ok(result)
}

pub fn select_sources(world: &mut World) -> Result<Vec<Entity>> {
    let all = candidates(world)?;
    // This pressure case deliberately clones ordinary shadow-casting meshes.
    // UI/explicit-pass actors have a different workload and are excluded here.
    let eligible: Vec<_> = all.iter().filter(|candidate| {
        let entity = world.entity(candidate.entity);
        !entity.contains::<UiPass>() && !entity.contains::<WireframePass>()
            && !entity.contains::<DesignatedRenderPass>() && !entity.contains::<DontCastShadow>()
    }).collect();
    anyhow::ensure!(eligible.len() >= 32, "benchmark requires32 ordinary source meshes, got{}",eligible.len());
    for index in 0..32 {
        anyhow::ensure!((index == 0 || eligible[index-1].key != eligible[index].key)
            && (index+1 == eligible.len() || eligible[index+1].key != eligible[index].key),
            "ambiguous benchmark source requires a semantic tie key: {}",eligible[index].key);
    }
    let selected: Vec<_> = eligible[..32].iter().map(|value|value.manifest.clone()).collect();
    eprintln!("PERF_INPUT_SOURCES {}",serde_json::to_string(&json!({"schema":2,
        "policy":"32 unique ordinary shadow-casting CPU meshes; canonical semantic ordering",
        "candidates":all.len(),"eligible":eligible.len(),"selected":selected}))?);
    Ok(eligible[..32].iter().map(|value|value.entity).collect())
}

pub fn scene_manifest(world: &mut World, draw_lists: &[usize]) -> Result<Value> {
    use country_core::render::nani::gpu_world::{GpuWorld, GpuWorldSubset, GpuWorldSubsetKind};
    let meshes: Vec<_> = candidates(world)?.into_iter().map(|value|value.manifest).collect();
    let gpu = world.resource::<GpuWorld>();
    let mut subsets = Vec::new();
    for kind in GpuWorldSubsetKind::all() {
        let slot = gpu.get_subset(kind);
        subsets.push(match &*slot {
            None => json!({"kind":format!("{kind:?}"),"state":"absent"}),
            Some(GpuWorldSubset::External(value)) => json!({"kind":format!("{kind:?}"),
                "state":"external","mesh_parts":value.mesh_part_count}),
            Some(GpuWorldSubset::Owned(value)) => {
                let mut positions: Vec<_> = value.positions()[..value.instance_count()].iter()
                    .map(|value| [value.position.x.to_bits(),value.position.y.to_bits(),
                        value.position.z.to_bits(),u32::from(value.radius.to_bits()),u32::from(value.mesh_index)])
                    .collect(); positions.sort_unstable();
                json!({"kind":format!("{kind:?}"),"state":"owned","instances":value.instance_count(),
                    "mesh_parts":value.mesh_part_count(),"cull_inputs":positions})
            }
        });
    }
    let session = world.resource::<country_core::resources::session::ActiveSession>();
    Ok(json!({"schema":2,"session":session.meta,"draw_lists":draw_lists,"meshes":meshes,"nani":subsets}))
}
