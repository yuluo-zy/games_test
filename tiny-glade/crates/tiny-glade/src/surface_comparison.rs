//! Single-prefix surface correctness diagnostic. Never a CPU benchmark, WSI
//! acceptance result, or replacement for the outstanding whole-scene test.
use super::*;
use country_core::render::pass_builder::draw_stream::{
    DescriptorSetBinding, DrawKind, RasterPassSubmission,
};
use rhapsody::{
    resource::ManagedBuffer,
    rg::{AccessRange, ResourceHandle, SessionFrameReceipt},
};
use serde_json::{Value, json};
use std::{collections::BTreeMap, path::Path, sync::Arc};

#[derive(Clone, Copy, Debug)]
pub enum Mode {
    Inline,
    Collect,
}
impl Mode {
    fn inline(self) -> bool {
        matches!(self, Self::Inline)
    }
    fn label(self) -> &'static str {
        if self.inline() { "inline" } else { "collect" }
    }
}

/// The integration fixture explicitly arms only the two observation frames.
/// Nothing in the renderer reads an environment variable.
#[derive(bevy_ecs::resource::Resource)]
pub struct SurfaceComparison {
    modes: [Mode; 2],
    armed: Option<u32>,
    frames: Vec<ComparedFrame>,
}
struct ComparedFrame {
    frame: u32,
    graph: u64,
    modes: [Mode; 2],
    histories: Vec<Value>,
    raster: [Vec<Value>; 2],
    storage: [Vec<Value>; 2],
    execution: [Value; 2],
    raw: Vec<RawCapture>,
    submitted: bool,
    receipt_frame: Option<u64>,
    retained_pass_order: Vec<String>,
    undefined_history_read_possible: bool,
}
struct RawCapture {
    branch: usize,
    role: &'static str,
    owner: ManagedBuffer,
    logical: ResourceHandle,
    bytes: u64,
    extent: [u32; 2],
    format: u32,
    aspect: u32,
    receipt_owner_confirmed: bool,
}
impl SurfaceComparison {
    pub fn new(modes: [Mode; 2]) -> Self {
        Self {
            modes,
            armed: None,
            frames: Vec::new(),
        }
    }
    pub fn arm(&mut self, frame: u32) -> rhapsody::Result<()> {
        if self.armed.is_some()
            || self.frames.len() >= 2
            || !matches!(frame, 20 | 30)
            || self.frames.iter().any(|f| f.frame == frame)
        {
            return Err(rhapsody::Error::invalid_usage(
                "local surface observation must be exactly once at frames20/30",
            ));
        }
        self.armed = Some(frame);
        Ok(())
    }
    /// Called only after the actual graph successfully submitted. Every raw
    /// capture must be exported by that receipt and map to this exact Arc.
    pub fn confirm_submission(
        &mut self,
        bridge: &mut crate::render_bridge::CoreRenderBridge,
        receipt: &SessionFrameReceipt,
    ) -> rhapsody::Result<()> {
        for frame in &mut self.frames {
            if frame.submitted {
                continue;
            }
            if receipt.receipts.is_empty() {
                return Err(rhapsody::Error::invalid_usage(
                    "local capture has no submitted receipt",
                ));
            }
            let expected = frame.raster[0]
                .first()
                .and_then(|p| p["pass"].as_str())
                .ok_or_else(|| {
                    rhapsody::Error::invalid_usage(
                        "local fixture has no actual surface declaration",
                    )
                })?;
            if receipt
                .snapshot
                .pass_order
                .iter()
                .filter(|name| name.as_str() == expected)
                .count()
                != 2
            {
                return Err(rhapsody::Error::invalid_usage(
                    "both local surface passes must be retained by actual receipt",
                ));
            }
            for raw in &mut frame.raw {
                if raw.logical.graph_id().raw() != frame.graph {
                    return Err(rhapsody::Error::invalid_usage(
                        "local capture graph changed",
                    ));
                }
                if !u16::try_from(raw.logical.raw())
                    .is_ok_and(|i| receipt.snapshot.exports.contains(&i))
                {
                    return Err(rhapsody::Error::invalid_usage(
                        "local capture absent from actual exports",
                    ));
                }
                let mapping = receipt
                    .snapshot
                    .physical_mapping
                    .iter()
                    .find(|m| m.logical == raw.logical.raw())
                    .ok_or_else(|| {
                        rhapsody::Error::invalid_usage("local capture has no physical mapping")
                    })?;
                let token = mapping.token.ok_or_else(|| {
                    rhapsody::Error::invalid_usage("local capture physical owner missing")
                })?;
                let actual = bridge.with_session(|s| s.backend.resource_clone(token));
                match actual {
                    Some(rhapsody::rg::GpuResource::Buffer(b))
                        if Arc::ptr_eq(&b, &raw.owner.shared()) => {}
                    _ => {
                        return Err(rhapsody::Error::invalid_usage(
                            "local capture receipt owner differs",
                        ));
                    }
                }
                raw.receipt_owner_confirmed = true;
            }
            frame.submitted = true;
            frame.receipt_frame = Some(receipt.frame);
            frame.retained_pass_order = receipt.snapshot.pass_order.clone();
        }
        Ok(())
    }

    /// Reuses the caller's already completed GPU drain. The tracked read uses
    /// zero additional waiting budget, including non-coherent invalidation.
    pub fn write_after_existing_drain(&self, out: &Path) -> anyhow::Result<Value> {
        anyhow::ensure!(
            self.armed.is_none() && self.frames.len() == 2,
            "missing local observations"
        );
        std::fs::create_dir_all(out)?;
        let mut reports = Vec::new();
        let mut all = true;
        for frame in &self.frames {
            anyhow::ensure!(frame.submitted, "unsubmitted local comparison");
            let mut data: BTreeMap<(usize, &str), Vec<u8>> = BTreeMap::new();
            let mut raw_reports = Vec::new();
            for raw in &frame.raw {
                anyhow::ensure!(
                    raw.receipt_owner_confirmed
                        && raw.owner.size == raw.bytes
                        && raw
                            .owner
                            .memory_properties()
                            .contains(ash::vk::MemoryPropertyFlags::HOST_VISIBLE),
                    "raw capture owner/size/host visibility invalid"
                );
                let mut bytes = vec![0; usize::try_from(raw.bytes)?];
                raw.owner.read_bytes_timeout(0, &mut bytes, 0)?;
                let file = format!("frame-{}-{}-{}.raw", frame.frame, raw.branch, raw.role);
                std::fs::write(out.join(&file), &bytes)?;
                raw_reports.push(
                    json!({"branch":raw.branch,"mode":frame.modes[raw.branch].label(),
                    "role":raw.role,"file":file,"bytes":raw.bytes,"extent":raw.extent,
                    "format":raw.format,"aspect":raw.aspect,"receipt_owner_confirmed":true}),
                );
                anyhow::ensure!(
                    data.insert((raw.branch, raw.role), bytes).is_none(),
                    "duplicate raw role"
                );
            }
            let mut comparisons = Vec::new();
            let mut pixels_equal = true;
            for role in ["color", "depth"] {
                let a = data
                    .get(&(0, role))
                    .ok_or_else(|| anyhow::anyhow!("missing A {role}"))?;
                let b = data
                    .get(&(1, role))
                    .ok_or_else(|| anyhow::anyhow!("missing B {role}"))?;
                anyhow::ensure!(a.len() == b.len(), "raw extents differ");
                let changed = a.iter().zip(b).filter(|(a, b)| a != b).count();
                let texel_bytes = 4;
                let changed_pixels = a
                    .chunks_exact(texel_bytes)
                    .zip(b.chunks_exact(texel_bytes))
                    .filter(|(a, b)| a != b)
                    .count();
                pixels_equal &= changed == 0;
                comparisons.push(json!({"role":role,"changed_bytes":changed,"changed_pixels":changed_pixels,"tolerance":0}));
            }
            let observation_supported = frame.raster.iter().all(|passes| {
                passes.len() == 1 && passes.iter().all(|p| p["observation_valid"] == true)
            });
            let execution_valid = frame.execution.iter().all(|e| e["valid"] == true);
            let declaration_equal = frame.raster[0] == frame.raster[1] && observation_supported;
            let valid =
                declaration_equal && execution_valid && !frame.undefined_history_read_possible;
            let pass = valid && pixels_equal;
            all &= pass;
            reports.push(json!({"frame":frame.frame,"graph":frame.graph,"receipt_frame":frame.receipt_frame,
                "actual_retained_pass_order":frame.retained_pass_order,"modes":frame.modes.map(Mode::label),
                "actual_temporal_inputs":frame.histories,"ordered_raster_declarations":frame.raster,
                "actual_storage_placement":frame.storage,"actual_surface_execution":frame.execution,
                "observation_supported":observation_supported,"execution_valid":execution_valid,
                "declarations_equal":declaration_equal,
                "undefined_history_read_possible":frame.undefined_history_read_possible,
                "valid_same_prefix_comparison":valid,"raw":raw_reports,"comparisons":comparisons,"passes":pass}));
        }
        let report = json!({"schema":"same-prefix-surface-correctness-v1","passes":all,"frames":reports,
            "scope":"local real water/ice surface prepare+raster+finish under one real opaque prefix; correctness only",
            "excluded_from_raw_comparison":["DONT_CARE/unwritten auxiliary MRT regions","uninitialized SSBO tails","previous temporal image raw pixels","unproven initialized stencil aspect","main color mips1..5"],
            "copied_subresource":"base mip0 layer0; original six-mip color descriptions unchanged",
            "stencil_policy":"stencil bits copied without clear to preserve original behavior; never read back/compared; actual stencil_test must be false on every observed surface draw",
            "not_proven":["whole-scene cross-process pixel determinism","production WSI","CPU performance acceptance","all native compute uniform payloads"],
            "normalization":"only known branch-private RG resource roles and dynamic-uniform storage placement; all observed constant bytes and ordered draw parameters compared",
            "no_additional_read_wait_ns":0,"pixel_tolerance":0});
        std::fs::write(
            out.join("local-comparison.json"),
            serde_json::to_vec_pretty(&report)?,
        )?;
        Ok(report)
    }
}

#[derive(Default)]
struct ForkState {
    roles: BTreeMap<u32, String>,
    histories: BTreeMap<String, (TextureDesc, TemporalTexturePair)>,
    b_histories: BTreeMap<String, TemporalTexturePair>,
}
struct ForkResources<'a, 'b, 'c, 'd> {
    original: SurfaceResources<'a, 'b, 'c>,
    second: bool,
    state: &'d RefCell<ForkState>,
}
impl ForkResources<'_, '_, '_, '_> {
    fn name(&self, name: &str) -> String {
        if self.second {
            format!("surface-local/B/{name}")
        } else {
            name.into()
        }
    }
    fn role(&self, id: u32, role: String) {
        self.state.borrow_mut().roles.insert(id, role);
    }
}
impl RgResources for ForkResources<'_, '_, '_, '_> {
    fn create_raw_texture(&mut self, name: &str, desc: &TextureDesc) -> RgTexture {
        let actual_name = self.name(name);
        let t = self.original.create_raw_texture(&actual_name, desc);
        self.role(t.resource, format!("private/{name}"));
        t
    }
    fn create_raw_buffer(&mut self, name: &str, desc: &BufferDesc) -> RgBuffer {
        let actual_name = self.name(name);
        let b = self.original.create_raw_buffer(&actual_name, desc);
        self.role(b.resource, format!("private/{name}"));
        b
    }
    fn get_or_create_temporal_texture_pair(
        &mut self,
        name: &str,
        desc: &TextureDesc,
    ) -> Result<TemporalTexturePair, RgError> {
        if !self.second {
            let pair = self
                .original
                .get_or_create_temporal_texture_pair(name, desc)?;
            let mut s = self.state.borrow_mut();
            if s.histories.insert(name.into(), (*desc, pair)).is_some() {
                return Err(RgError::msg("duplicate A temporal request"));
            }
            s.roles
                .insert(pair.current.resource, format!("current/{name}"));
            Ok(pair)
        } else {
            let (expected, pair) = self
                .state
                .borrow()
                .histories
                .get(name)
                .copied()
                .ok_or_else(|| RgError::msg("B requested temporal pair absent from real A"))?;
            if expected != *desc {
                return Err(RgError::msg("B temporal description differs"));
            }
            let current = self
                .original
                .create_raw_texture(&format!("surface-local/B/current/{name}"), desc);
            let mut s = self.state.borrow_mut();
            if s.b_histories.contains_key(name) {
                return Err(RgError::msg("duplicate B temporal request"));
            }
            let fork = TemporalTexturePair {
                current,
                previous: pair.previous,
                fresh: pair.fresh,
            };
            s.b_histories.insert(name.into(), fork);
            s.roles.insert(current.resource, format!("current/{name}"));
            Ok(fork)
        }
    }
    fn get_or_create_temporal_texture(
        &mut self,
        _: &str,
        _: &TextureDesc,
    ) -> Result<TemporalTexture, RgError> {
        Err(RgError::msg(
            "single temporal texture unsupported by local surface fixture",
        ))
    }
    fn get_or_create_temporal_buffer(
        &mut self,
        _: &str,
        _: &BufferDesc,
    ) -> Result<TemporalBuffer, RgError> {
        Err(RgError::msg(
            "temporal buffer unsupported by local surface fixture",
        ))
    }
}

fn fork_env<T>(
    frame: &mut CoreGraphFrame<'_>,
    world: &World,
    second: bool,
    state: &RefCell<ForkState>,
    build: impl FnOnce(&mut SurfaceEnv<'_>) -> rhapsody::Result<T>,
) -> rhapsody::Result<T> {
    with_env(frame, world, |env| {
        let shared = RefCell::new(env);
        let mut rg = ForkResources {
            original: SurfaceResources(&shared),
            second,
            state,
        };
        let mut graph = SurfaceGraph(&shared);
        let mut compute = SurfacePasses(&shared);
        build(&mut SurfaceEnv::new(&mut rg, &mut graph, &mut compute))
    })
}

struct Observer {
    roles: BTreeMap<u32, String>,
    semantic: Vec<Value>,
    storage: Vec<Value>,
}
thread_local! {static OBSERVER:RefCell<Option<Observer>>=const {RefCell::new(None)};}
fn observed<T>(
    roles: BTreeMap<u32, String>,
    run: impl FnOnce() -> rhapsody::Result<T>,
) -> rhapsody::Result<(T, Vec<Value>, Vec<Value>)> {
    struct Reset;
    impl Drop for Reset {
        fn drop(&mut self) {
            OBSERVER.with(|s| {
                s.borrow_mut().take();
            });
        }
    }
    let occupied = OBSERVER.with(|s| s.borrow().is_some());
    if occupied {
        return Err(rhapsody::Error::invalid_usage("nested surface observer"));
    }
    OBSERVER.with(|s| {
        *s.borrow_mut() = Some(Observer {
            roles,
            semantic: Vec::new(),
            storage: Vec::new(),
        })
    });
    let _reset = Reset;
    let value = run()?;
    let observer = OBSERVER
        .with(|s| s.borrow_mut().take())
        .expect("owner-local observer retained");
    Ok((value, observer.semantic, observer.storage))
}
fn resource(roles: &BTreeMap<u32, String>, id: u32) -> Value {
    roles.get(&id).map_or_else(
        || json!({"shared_rg":id}),
        |role| json!({"branch_private_role":role}),
    )
}
pub(crate) fn record_raster_submission(pass: &RasterPassSubmission) {
    OBSERVER.with(|slot| {
        let mut active=slot.borrow_mut();let Some(o)=active.as_mut() else {return};
        let mut draws=Vec::new();let mut placements=Vec::new();
        let mut observation_valid=!pass.draws.is_empty() && matches!(pass.name,"Water plane"|"Ice plane");
        for draw in &pass.draws {
            let native_stencil_test=rhapsody::pipeline::RasterPipelineOpts(draw.state.0).stencil_test();
            observation_valid &= !native_stencil_test;
            let uniforms=draw.bindings.values().filter(|b|matches!(&b.binding,DescriptorSetBinding::UniformBuffer {..})).count();
            observation_valid &= uniforms>0 && draw.observed_dynamic_constants.len()==uniforms;
            let mut external=draw.bindings.iter().collect::<Vec<_>>();external.sort_unstable_by_key(|(binding,_)|**binding);
            let mut bindings=Vec::new();let mut storage=Vec::new();
            for (&binding,owned) in external {
                let data=match &owned.binding {
                    DescriptorSetBinding::SampledImage {image_view,layout_word,keepalive}=>json!({"kind":"external_image","image_view":image_view,"layout_word":layout_word,"keepalive":keepalive.0}),
                    DescriptorSetBinding::StorageBuffer {buffer,offset,range,keepalive}=>json!({"kind":"external_buffer","buffer":buffer,"offset":offset,"range":range,"keepalive":keepalive.0}),
                    DescriptorSetBinding::UniformBuffer {size,keepalive,..}=>{
                        let matching=draw.observed_dynamic_constants.iter().filter(|(index,bytes)|
                            *index==binding && bytes.len()==*size as usize && !bytes.is_empty()).count();
                        observation_valid &= matching==1;
                        storage.push(json!({"binding":binding,"actual_uniform_keepalive":keepalive.0}));
                        json!({"kind":"dynamic_uniform","size":size})
                    },
                };
                bindings.push(json!({"binding":binding,"state":owned.state,"value":data}));
            }
            let accesses=draw.accesses.iter().map(|a| {
                let b=&pass.bindings[a.slot as usize];
                json!({"binding":a.binding_index,"resource":resource(&o.roles,b.resource_index),
                    "access_type":b.access_type,"view":format!("{:?}",b.view)})
            }).collect::<Vec<_>>();
            // This real fixture's water_planes callback uses direct instanced
            // geometry. Other draw kinds must be added explicitly, never hidden.
            let kind=match &draw.kind {
                DrawKind::InstancedGlDrawFlag {flag}=>json!({"kind":3,"flag":flag}),
                other=>{ observation_valid=false;json!({"unsupported_draw_kind":format!("{other:?}")}) },
            };
            draws.push(json!({"head":draw.head,"shader":draw.shader.0.0,"variant":draw.variant,
                "state":draw.state.0,"blend_bits":draw.blend.to_bits(),"unknown_b1":draw.unknown_b1,
                "unknown_bytes":draw.unknown_bytes,"kind":kind,"bindings":bindings,"accesses":accesses,
                "native_stencil_test":native_stencil_test,
                "dynamic_constants":draw.observed_dynamic_constants,
                "preserving_buffer_bindings":draw.preserving_buffer_bindings,
                "inactive_texture_bindings":draw.inactive_texture_bindings,
                "undefined_texture_bindings":draw.undefined_texture_bindings}));
            placements.push(storage);
        }
        let colors=pass.colors.iter().map(|a|json!({"resource":resource(&o.roles,a.id),"format":a.format,
            "tag":a.tag,"samples":a.sample_count,"clear_bits":a.clear_color_bits()})).collect::<Vec<_>>();
        let depth=pass.depth.map(|a|json!({"resource":resource(&o.roles,a.texture_id),"format":a.format,
            "samples":a.sample_count,"clear":format!("{:?}",a.clear),"writable":a.writable,"layer":a.layer}));
        let declared_bindings=pass.bindings.iter().map(|b|json!({
            "resource":resource(&o.roles,b.resource_index),"access_type":b.access_type,
            "view":format!("{:?}",b.view)})).collect::<Vec<_>>();
        let declared_accesses=pass.accesses.iter().map(|a|json!({
            "resource":resource(&o.roles,a.resource),"access_type":a.access,"group":a.group})).collect::<Vec<_>>();
        o.semantic.push(json!({"pass":pass.name,"viewport":pass.viewport,"colors":colors,"depth":depth,
            "draws":draws,"observation_valid":observation_valid,"declared_bindings":declared_bindings,
            "declared_accesses":declared_accesses,"binding_accesses":pass.binding_accesses}));
        o.storage.push(json!({"pass":pass.name,"native_render_pass":pass.render_pass.0,
            "uniform_storage":placements,"raw_binding_slots":format!("{:?}",pass.bindings),
            "binding_accesses":pass.binding_accesses,"raw_accesses":format!("{:?}",pass.accesses)}));
    });
}

fn copy(
    frame: &mut CoreGraphFrame<'_>,
    source: RgTexture,
    target: RgTexture,
) -> rhapsody::Result<()> {
    frame.with_graph(|g| {
        let src = g.resource_handle(source.resource)?;
        let dst = g.resource_handle(target.resource)?;
        let mut pass = g.add_pass("Surface diagnostic exact image copy");
        pass.on_queue(rhapsody::device::QueueClass::Graphics);
        let aspect_mask = if source.desc.format == 0x82 { 6 } else { 1 };
        let selected = AccessRange::Texture {
            aspect_mask,
            base_mip_level: 0,
            level_count: 1,
            base_array_layer: 0,
            layer_count: 1,
        };
        pass.add_resource_access_range(src, 0x25, 0, selected)?;
        pass.add_resource_access_range(dst, 0x34, 0, selected)?;
        pass.render_with_result_context(move |ctx| {
            rhapsody::rg::test_image_copy::copy_image(&ctx, src, dst)
        });
        Ok(())
    })
}
fn cloned_target(
    frame: &mut CoreGraphFrame<'_>,
    source: RgTexture,
    name: &str,
) -> rhapsody::Result<RgTexture> {
    frame.with_compute(|compute| Ok(compute.create_raw_texture(name, &source.desc)))
}
fn raw_capture(
    frame: &mut CoreGraphFrame<'_>,
    branch: usize,
    source: RgTexture,
    role: &'static str,
    aspect: ash::vk::ImageAspectFlags,
) -> rhapsody::Result<RawCapture> {
    let extent = source.desc.extent_2d();
    let format = ash::vk::Format::from_raw(source.desc.format as i32);
    let bytes = rhapsody::rg::test_image_copy::raw_byte_count(format, extent, aspect)?;
    let owner = frame.hosts().buffers.graphics_device().create_buffer(
        &rhapsody::resource::BufferDesc::new_gpu_to_cpu(
            bytes,
            ash::vk::BufferUsageFlags::TRANSFER_DST,
        ),
        "Surface raw comparison readback",
        (),
    )?;
    let logical = frame.import_native_buffer(owner.shared(), 0)?;
    frame.with_graph(|g| {
        let src = g.resource_handle(source.resource)?;
        let mut pass = g.add_pass("Surface diagnostic raw aspect capture");
        pass.on_queue(rhapsody::device::QueueClass::Graphics);
        pass.add_resource_access_range(
            src,
            0x25,
            0,
            AccessRange::Texture {
                aspect_mask: aspect.as_raw(),
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1,
            },
        )?;
        pass.add_resource_access_range(
            logical,
            0x34,
            0,
            AccessRange::Buffer {
                offset: 0,
                size: bytes,
            },
        )?;
        pass.render_with_result_context(move |ctx| {
            rhapsody::rg::test_image_copy::copy_image_to_buffer(&ctx, src, logical, aspect)
        });
        g.export(logical, 0x26)?;
        Ok(())
    })?;
    Ok(RawCapture {
        branch,
        role,
        owner,
        logical,
        bytes,
        extent,
        format: source.desc.format,
        aspect: aspect.as_raw(),
        receipt_owner_confirmed: false,
    })
}

fn execution_delta(
    before:country_core::systems::render_loop::draw_passes::transparency_pass::surface_support::SurfaceCollectTestStats,
    after:country_core::systems::render_loop::draw_passes::transparency_pass::surface_support::SurfaceCollectTestStats,
    mode: Mode,
    draws: usize,
) -> rhapsody::Result<Value> {
    let before = [
        before.inline_calls,
        before.inline_draws,
        before.worker_calls,
        before.worker_tasks,
        before.worker_draws,
        before.owner_collect_tasks,
    ];
    let after = [
        after.inline_calls,
        after.inline_draws,
        after.worker_calls,
        after.worker_tasks,
        after.worker_draws,
        after.owner_collect_tasks,
    ];
    let mut delta = [0; 6];
    for i in 0..6 {
        delta[i] = after[i].checked_sub(before[i]).ok_or_else(|| {
            rhapsody::Error::invalid_usage("surface execution counter reset inside paired branch")
        })?;
    }
    let [
        inline_calls,
        inline_draws,
        worker_calls,
        worker_tasks,
        worker_draws,
        owner_collect_tasks,
    ] = delta;
    let valid = if mode.inline() {
        inline_calls == 1
            && inline_draws == draws as u64
            && worker_calls == 0
            && worker_tasks == 0
            && worker_draws == 0
            && owner_collect_tasks == 0
    } else {
        inline_calls == 0
            && inline_draws == 0
            && worker_calls == 1
            && worker_tasks > 0
            && worker_draws == draws as u64
            && owner_collect_tasks == 0
    };
    Ok(
        json!({"requested_mode":mode.label(),"inline_calls":inline_calls,"inline_draws":inline_draws,
        "worker_calls":worker_calls,"worker_tasks":worker_tasks,"worker_draws":worker_draws,
        "owner_collect_tasks":owner_collect_tasks,"valid":valid}),
    )
}

pub(super) fn run_if_armed(
    frame: &mut CoreGraphFrame<'_>,
    world: &mut World,
    inputs: &SurfaceInputs<'_>,
    glade: GladeType,
) -> rhapsody::Result<bool> {
    if !world
        .get_resource::<SurfaceComparison>()
        .is_some_and(|s| s.armed.is_some())
    {
        return Ok(false);
    }
    let mut owner = world
        .remove_resource::<SurfaceComparison>()
        .expect("armed diagnostic owner");
    let result = (|| {
        let number = owner.armed.take().expect("one armed frame");
        let state = RefCell::new(ForkState::default());
        let colors = [
            cloned_target(frame, *inputs.color, "surface-local/A/color")?,
            cloned_target(frame, *inputs.color, "surface-local/B/color")?,
        ];
        let depths = [
            cloned_target(frame, *inputs.depth, "surface-local/A/depth")?,
            cloned_target(frame, *inputs.depth, "surface-local/B/depth")?,
        ];
        for branch in 0..2 {
            copy(frame, *inputs.color, colors[branch])?;
            copy(frame, *inputs.depth, depths[branch])?;
            state
                .borrow_mut()
                .roles
                .insert(colors[branch].resource, "surface-color".into());
            state
                .borrow_mut()
                .roles
                .insert(depths[branch].resource, "surface-depth".into());
        }
        let mut raster: [Vec<Value>; 2] = Default::default();
        let mut storage: [Vec<Value>; 2] = Default::default();
        let mut execution: [Value; 2] = Default::default();
        let mut raw = Vec::new();
        for branch in 0..2 {
            let input = SurfaceInputs {
                color: &colors[branch],
                depth: &depths[branch],
                ..*inputs
            };
            let second = branch == 1;
            use country_core::systems::render_loop::draw_passes::transparency_pass::surface_support::{
                with_surface_collect_mode_for_test,surface_collect_test_stats_for_test};
            let before = surface_collect_test_stats_for_test();
            match transparency_pass::select_water_surface(glade) {
                transparency_pass::WaterSurfaceKind::Water => {
                    let prepared = fork_env(frame, world, second, &state, |env| {
                        Ok(prepare_water_surface(env, &input))
                    })?;
                    let (_, trace, locations) = observed(state.borrow().roles.clone(), || {
                        with_surface_collect_mode_for_test(owner.modes[branch].inline(), || {
                            with_draw_context(frame, world, |ctx| {
                                raster_water_surface(ctx, &prepared, &input)
                            })
                        })
                    })?;
                    raster[branch] = trace;
                    storage[branch] = locations;
                    fork_env(frame, world, second, &state, |env| {
                        finish_water_surface(env, prepared, &input);
                        Ok(())
                    })?;
                }
                transparency_pass::WaterSurfaceKind::Ice => {
                    let prepared = fork_env(frame, world, second, &state, |env| {
                        prepare_ice_surface(env, &input)
                    })?;
                    let (_, trace, locations) = observed(state.borrow().roles.clone(), || {
                        with_surface_collect_mode_for_test(owner.modes[branch].inline(), || {
                            with_draw_context(frame, world, |ctx| {
                                raster_ice_surface(ctx, &prepared, &input)
                            })
                        })
                    })?;
                    raster[branch] = trace;
                    storage[branch] = locations;
                    fork_env(frame, world, second, &state, |env| {
                        finish_ice_surface(env, prepared, &input)
                    })?;
                }
            }
            execution[branch] = execution_delta(
                before,
                surface_collect_test_stats_for_test(),
                owner.modes[branch],
                input.lists.list(transparency_pass::SURFACE_LIST).len(),
            )?;
            raw.push(raw_capture(
                frame,
                branch,
                colors[branch],
                "color",
                ash::vk::ImageAspectFlags::COLOR,
            )?);
            raw.push(raw_capture(
                frame,
                branch,
                depths[branch],
                "depth",
                ash::vk::ImageAspectFlags::DEPTH,
            )?);
        }
        let state = state.into_inner();
        if state.histories.len() != state.b_histories.len() {
            return Err(rhapsody::Error::invalid_usage(
                "A/B temporal request sets differ",
            ));
        }
        let mut histories = Vec::new();
        let mut undefined = false;
        for (name, (desc, pair)) in &state.histories {
            let b = state
                .b_histories
                .get(name)
                .ok_or_else(|| rhapsody::Error::invalid_usage("B temporal pair missing"))?;
            if pair.previous != b.previous || pair.fresh != b.fresh || pair.current == b.current {
                return Err(rhapsody::Error::invalid_usage(
                    "paired history identity or current isolation failed",
                ));
            }
            undefined |= name == "ice refl ray len" && !pair.fresh;
            histories.push(json!({"name":name,"a_previous_rg":pair.previous.resource,"b_previous_rg":b.previous.resource,
                "a_current_rg":pair.current.resource,"b_current_rg":b.current.resource,
                "a_history_valid":pair.fresh,"b_history_valid":b.fresh,"actual_desc":format!("{desc:?}")}));
        }
        let graph = raw
            .first()
            .expect("four real raw captures")
            .logical
            .graph_id()
            .raw();
        // A alone advances the existing production images/history. These exact
        // copies let the unchanged pre-TAA/postprocess tail keep its real IDs.
        copy(frame, colors[0], *inputs.color)?;
        copy(frame, depths[0], *inputs.depth)?;
        owner.frames.push(ComparedFrame {
            frame: number,
            graph,
            modes: owner.modes,
            histories,
            raster,
            storage,
            execution,
            raw,
            submitted: false,
            receipt_frame: None,
            retained_pass_order: Vec::new(),
            undefined_history_read_possible: undefined,
        });
        Ok(true)
    })();
    world.insert_resource(owner);
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use country_core::systems::render_loop::draw_passes::transparency_pass::surface_support::SurfaceCollectTestStats as Stats;
    #[test]
    fn branch_mode_requires_real_worker_and_no_owner_fallback() {
        let before = Stats {
            inline_calls: 10,
            worker_calls: 20,
            worker_draws: 60,
            ..Default::default()
        };
        let mut after = before;
        after.worker_calls += 1;
        after.worker_draws += 3;
        after.worker_tasks += 1;
        assert_eq!(
            execution_delta(before, after, Mode::Collect, 3).unwrap()["valid"],
            true
        );
        after.owner_collect_tasks += 1;
        assert_eq!(
            execution_delta(before, after, Mode::Collect, 3).unwrap()["valid"],
            false
        );
    }
    #[test]
    fn counter_reset_is_not_silent_underflow() {
        let before = Stats {
            worker_calls: 1,
            ..Default::default()
        };
        assert!(execution_delta(before, Stats::default(), Mode::Inline, 1).is_err());
    }
    #[test]
    fn request_arming_is_explicit_and_bounded() {
        let mut s = SurfaceComparison::new([Mode::Inline, Mode::Collect]);
        assert!(s.arm(1).is_err());
        assert!(s.arm(20).is_ok());
        assert!(s.arm(30).is_err());
    }
}
