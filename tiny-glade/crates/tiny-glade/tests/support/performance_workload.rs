//! Shared stimuli for the real window loop and the existing headless renderer.
//! Call once AFTER each completed gameplay frame, before the next App update.
//! No renderer, clock, quality or worker policy is installed here.
use anyhow::{Context, Result};
use bevy_ecs::world::World;
use serde_json::{Value, json};

#[path = "performance_input.rs"]
mod input;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize)]
pub struct FrameWorkloadDelta {
    pub frame: u64,
    pub clones_created: usize,
    pub edits: u64,
    /// Bytes intentionally written to the logical 16x16 source rectangle.
    pub logical_edit_bytes: u64,
    /// Exact producer-emitted staging spans, before upload merge/admission.
    /// This is NOT GPU accepted/completed upload bytes; rejection keeps them dirty.
    pub producer_dirty_bytes: u64,
    pub producer_dirty_ranges: usize,
}

pub struct PerformanceWorkload {
    scenario: String,
    last_frame: u64,
    clones_created: usize,
    edits: u64,
    logical_edit_bytes: u64,
    producer_dirty_bytes: u64,
    producer_dirty_ranges: usize,
    source_manifest: Option<Value>,
    first_scene: Option<Value>,
    schedule: blake3::Hasher,
    draws: blake3::Hasher,
}

impl PerformanceWorkload {
    pub fn from_environment() -> Result<Option<Self>> {
        std::env::var_os("TG_PERF_SCENARIO").map(|value| {
            Self::new(value.into_string().map_err(|_| anyhow::anyhow!("invalid performance scenario"))?)
        }).transpose()
    }

    pub fn new(scenario: impl Into<String>) -> Result<Self> {
        let scenario = scenario.into();
        anyhow::ensure!(["static", "many_draws", "local_edit", "continuous_edit", "loading_switch"]
            .contains(&scenario.as_str()), "unknown scenario {scenario}");
        let mut schedule = blake3::Hasher::new();
        schedule.update(b"real-ecs-raster-workload-v1");
        schedule.update(scenario.as_bytes());
        Ok(Self { scenario, last_frame: 0, clones_created: 0, edits: 0,
            logical_edit_bytes: 0, producer_dirty_bytes: 0, producer_dirty_ranges: 0,
            source_manifest: None, first_scene: None, schedule, draws: blake3::Hasher::new() })
    }

    pub fn scenario(&self) -> &str { &self.scenario }
    pub fn first_scene(&self) -> Option<&Value> { self.first_scene.as_ref() }

    pub fn after_frame(&mut self, world: &mut World, frame: u64, output_extent: [u32; 2],
        draw_list_lengths: &[usize]) -> Result<FrameWorkloadDelta> {
        let mut delta = FrameWorkloadDelta { frame, ..Default::default() };
        if frame == 0 || frame == self.last_frame { return Ok(delta); }
        anyhow::ensure!(frame > self.last_frame, "performance frame counter moved backwards");
        self.last_frame = frame;
        let config = world.resource::<country_core::render::render_config::FrameRenderConfig>();
        anyhow::ensure!(output_extent == [1280, 720]
            && config.world_resolution_scale.to_bits() == 1.0_f32.to_bits(),
            "benchmark did not retain requested extent/scale");
        let expected = std::time::Duration::from_secs_f64(1.0 / 60.0);
        anyhow::ensure!(matches!(world.get_resource::<bevy_time::TimeUpdateStrategy>(),
            Some(bevy_time::TimeUpdateStrategy::ManualDuration(duration)) if *duration == expected),
            "actual gameplay lost its fixed time strategy");
        self.draws.update(&frame.to_le_bytes());
        for &length in draw_list_lengths { self.draws.update(&(length as u64).to_le_bytes()); }

        if self.scenario == "many_draws" && frame == 10 {
            let sources = input::select_sources(world)?;
            let all = input::candidates(world)?;
            self.source_manifest = Some(Value::Array(sources.iter().map(|source| all.iter()
                .find(|candidate| candidate.entity == *source).unwrap().manifest.clone()).collect()));
            // Preserve all real ECS components and actual asset/buffer owners.
            for source in sources {
                for _ in 0..15 {
                    let target = world.spawn_empty().id();
                    bevy_ecs::entity::EntityCloner::build(world).clone_entity(source, target);
                    delta.clones_created += 1;
                }
            }
            self.clones_created += delta.clones_created;
        }
        if frame == 199 {
            self.first_scene = Some(input::scene_manifest(world, draw_list_lengths)?);
            if self.scenario == "many_draws" {
                let draws = draw_list_lengths.iter().sum::<usize>();
                anyhow::ensure!(draws >= 1000, "stress sources failed actual draw lists: {draws}");
                anyhow::ensure!(self.clones_created == 32 * 15, "missing real ECS stress clones");
            }
        }
        if frame >= 200 && (self.scenario == "continuous_edit"
            || (self.scenario == "local_edit" && frame % 30 == 0)) {
            use country_core::{resources::world_raster_defs::{PathRaster, PathRasterStagingSsbo},
                asset_libraries::ssbo_library::AssetSsboLibrary};
            let value = if self.edits % 2 == 0 { 255 } else { 0 };
            world.resource_scope(|world, mut raster: bevy_ecs::change_detection::Mut<PathRaster>| -> Result<()> {
                let pixels = raster.composite_mut();
                for y in 248..264 { pixels[y*512+248..y*512+264].fill(value); }
                for y in 31..33 { for x in 31..33 { raster.dirty_mut().set_cell(x,y); } }
                world.resource_mut::<AssetSsboLibrary>().get_mut::<PathRasterStagingSsbo>()
                    .context("missing actual PathRaster staging owner")?
                    .with_cpu_write_regions(|mapped, ranges| {
                        let start = ranges.len();
                        raster.copy_to_staging_with_ranges(0, bytemuck::cast_slice_mut(mapped), ranges);
                        for range in &ranges[start..] {
                            delta.producer_dirty_bytes += (range.end - range.start) as u64;
                            delta.producer_dirty_ranges += 1;
                        }
                    })?;
                Ok(())
            })?;
            delta.edits = 1;
            delta.logical_edit_bytes = 16 * 16;
            self.edits += 1;
            self.logical_edit_bytes += delta.logical_edit_bytes;
            self.producer_dirty_bytes += delta.producer_dirty_bytes;
            self.producer_dirty_ranges += delta.producer_dirty_ranges;
        }
        self.schedule.update(&frame.to_le_bytes());
        self.schedule.update(&(delta.clones_created as u64).to_le_bytes());
        self.schedule.update(&delta.edits.to_le_bytes());
        self.schedule.update(&delta.logical_edit_bytes.to_le_bytes());
        self.schedule.update(&delta.producer_dirty_bytes.to_le_bytes());
        self.schedule.update(&(delta.producer_dirty_ranges as u64).to_le_bytes());
        Ok(delta)
    }

    /// Serialize after measurement; equality is required for paired input runs.
    pub fn summary(&self) -> Value {
        json!({"schema":"real-ecs-raster-workload-v1", "scenario":self.scenario,
            "completed_frames":self.last_frame, "clones_created":self.clones_created,
            "edits":self.edits, "logical_edit_bytes":self.logical_edit_bytes,
            "producer_dirty_bytes":self.producer_dirty_bytes,
            "producer_dirty_ranges":self.producer_dirty_ranges,
            "sources":self.source_manifest,
            "schedule_blake3":self.schedule.finalize().to_hex().to_string(),
            "draw_lengths_blake3":self.draws.finalize().to_hex().to_string(),
            "policy":{"clones_after_frame":10,"source_meshes":32,"copies_each":15,
                "scene_before_edits_frame":199,"edit_begin_frame":200,"local_edit_period":30,
                "rectangle":[248,248,16,16],"dirty_tile_rectangle":[31,31,2,2],
                "edit_values":[255,0],"fixed_delta":"1/60","extent":[1280,720],"world_scale":1.0}})
    }
}
