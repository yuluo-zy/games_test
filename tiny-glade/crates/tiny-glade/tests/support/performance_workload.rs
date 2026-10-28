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
    verified_edits: u64,
    verified_changed_pixels: u64,
    edit_effects: blake3::Hasher,
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
            verified_edits: 0, verified_changed_pixels: 0, edit_effects: blake3::Hasher::new(),
            source_manifest: None, first_scene: None, schedule, draws: blake3::Hasher::new() })
    }

    pub fn scenario(&self) -> &str { &self.scenario }
    pub fn first_scene(&self) -> Option<&Value> { self.first_scene.as_ref() }

    pub fn after_frame(&mut self, world: &mut World, frame: u64, output_extent: [u32; 2],
        draw_list_lengths: &[usize]) -> Result<FrameWorkloadDelta> {
        let mut delta = FrameWorkloadDelta { frame, ..Default::default() };
        if frame == 0 || frame == self.last_frame { return Ok(delta); }
        anyhow::ensure!(frame > self.last_frame, "performance frame counter moved backwards");
        anyhow::ensure!(frame == self.last_frame + 1, "missing workload frame; cannot infer omitted edits");
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
        if self.scenario == "many_draws" && frame >= 199 {
            anyhow::ensure!(draw_list_lengths.iter().sum::<usize>() >= 1000,
                "many_draws dropped below 1000 actual draw-list entries after settlement");
        }
        if edit_due(&self.scenario, frame) {
            use country_core::{resources::world_raster_defs::{PathRaster, PathRasterStagingSsbo},
                asset_libraries::ssbo_library::AssetSsboLibrary};
            let value = if self.edits % 2 == 0 { 255 } else { 0 };
            let edit_ordinal = self.edits;
            let mut changed_pixels = 0u64;
            let mut before_hashes = [0u32; 4];
            let mut after_hashes = [0u32; 4];
            world.resource_scope(|world, mut raster: bevy_ecs::change_detection::Mut<PathRaster>| -> Result<()> {
                let previous_value = 255 - value;
                let mut previous_value_retained = true;
                let changed_per_tile: [usize; 4] = std::array::from_fn(|slot| {
                    let x = 248 + (slot % 2) * 8;
                    let y = 248 + (slot / 2) * 8;
                    let mut changed = 0;
                    for row in y..y+8 {
                        for &byte in &raster.composite()[row*512+x..row*512+x+8] {
                            changed += usize::from(byte != value);
                            previous_value_retained &= edit_ordinal == 0 || byte == previous_value;
                        }
                    }
                    changed
                });
                changed_pixels = changed_per_tile.iter().sum::<usize>() as u64;
                anyhow::ensure!(changed_pixels > 0, "scheduled PathRaster edit is a CPU no-op");
                anyhow::ensure!(edit_ordinal == 0 || (previous_value_retained && changed_pixels == 256),
                    "PathRaster no longer retains the previous alternating fixture value");
                before_hashes = std::array::from_fn(|slot| raster.tile_hash(edit_tile_index(slot)));
                let pixels = raster.composite_mut();
                for y in 248..264 { pixels[y*512+248..y*512+264].fill(value); }
                anyhow::ensure!((248..264).all(|row|
                    raster.composite()[row*512+248..row*512+264].iter().all(|&byte| byte == value)),
                    "PathRaster logical edit did not write its intended bytes");
                for y in 31..33 { for x in 31..33 { raster.dirty_mut().set_cell(x,y); } }
                let mut staging_proved = false;
                world.resource_mut::<AssetSsboLibrary>().get_mut::<PathRasterStagingSsbo>()
                    .context("missing actual PathRaster staging owner")?
                    .with_cpu_write_regions(|mapped, ranges| {
                        let start = ranges.len();
                        let bytes = bytemuck::cast_slice_mut(mapped);
                        raster.copy_to_staging_with_ranges(0, bytes, ranges);
                        // Check only four existing 64-byte tiles, without scanning the SSBO.
                        staging_proved = (0..4).all(|slot| {
                            let begin = edit_tile_index(slot) * 64;
                            let end = begin + 64;
                            bytes.get(begin..end).is_some_and(|tile| tile.iter().all(|&byte| byte == value))
                                && (changed_per_tile[slot] == 0 || ranges[start..].iter()
                                    .any(|range| range.start <= begin && range.end >= end))
                        });
                        for range in &ranges[start..] {
                            delta.producer_dirty_bytes += (range.end - range.start) as u64;
                            delta.producer_dirty_ranges += 1;
                        }
                    })?;
                after_hashes = std::array::from_fn(|slot| raster.tile_hash(edit_tile_index(slot)));
                let expected_hash = country_core::resources::world_raster::build_tile_hasher()
                    .hash_tile(&[value; 64]);
                anyhow::ensure!(staging_proved && raster.published() && raster.staged_selector() == 0,
                    "PathRaster edit lacks exact CPU staging bytes/ranges/publication proof");
                anyhow::ensure!((0..4).all(|slot| after_hashes[slot] == expected_hash
                    && (changed_per_tile[slot] == 0 || before_hashes[slot] != after_hashes[slot])),
                    "PathRaster edit lacks expected dirty-tile hash transition");
                Ok(())
            })?;
            delta.edits = 1;
            delta.logical_edit_bytes = 16 * 16;
            self.edits += 1;
            self.verified_edits += 1;
            self.verified_changed_pixels += changed_pixels;
            self.edit_effects.update(&frame.to_le_bytes());
            self.edit_effects.update(&[value]);
            self.edit_effects.update(&changed_pixels.to_le_bytes());
            for hash in before_hashes.into_iter().chain(after_hashes) {
                self.edit_effects.update(&hash.to_le_bytes());
            }
            self.logical_edit_bytes += delta.logical_edit_bytes;
            self.producer_dirty_bytes += delta.producer_dirty_bytes;
            self.producer_dirty_ranges += delta.producer_dirty_ranges;
        }
        self.validate_completed(frame)?;
        self.schedule.update(&frame.to_le_bytes());
        self.schedule.update(&(delta.clones_created as u64).to_le_bytes());
        self.schedule.update(&delta.edits.to_le_bytes());
        self.schedule.update(&delta.logical_edit_bytes.to_le_bytes());
        self.schedule.update(&delta.producer_dirty_bytes.to_le_bytes());
        self.schedule.update(&(delta.producer_dirty_ranges as u64).to_le_bytes());
        Ok(delta)
    }

    /// Call again outside the measurement envelope at the exact loop end.
    /// This proves only logical/staging edits, never GPU acceptance or consumption.
    pub fn validate_completed(&self, completed_frames: u64) -> Result<()> {
        anyhow::ensure!(self.last_frame == completed_frames, "workload completion count differs");
        let expected = expected_edits(&self.scenario, completed_frames);
        anyhow::ensure!(self.edits == expected && self.verified_edits == expected
            && self.logical_edit_bytes == expected * 256, "scheduled Raster edits missing or unverified");
        let clones = if self.scenario == "many_draws" && completed_frames >= 10 { 32*15 } else { 0 };
        anyhow::ensure!(self.clones_created == clones, "scheduled real ECS clones missing");
        Ok(())
    }

    /// Serialize after measurement; equality is required for paired input runs.
    pub fn summary(&self) -> Value {
        json!({"schema":"real-ecs-raster-workload-v1", "scenario":self.scenario,
            "completed_frames":self.last_frame, "clones_created":self.clones_created,
            "edits":self.edits, "logical_edit_bytes":self.logical_edit_bytes,
            "edit_effect_proof":{"schema":"logical-raster-edit-effect-v1",
                "expected_edits":expected_edits(&self.scenario,self.last_frame),
                "verified_edits":self.verified_edits,"changed_pixels":self.verified_changed_pixels,
                "blake3":self.edit_effects.finalize().to_hex().to_string(),
                "scope":"CPU logical pixels, four staging tiles/ranges and tile-hash publication only; not GPU consumption"},
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

fn edit_tile_index(slot: usize) -> usize { (31 + slot / 2) * 64 + 31 + slot % 2 }
fn edit_due(scenario: &str, frame: u64) -> bool {
    frame >= 200 && (scenario == "continuous_edit" || (scenario == "local_edit" && frame % 30 == 0))
}
fn expected_edits(scenario: &str, through: u64) -> u64 {
    match scenario {
        "continuous_edit" => through.saturating_sub(199),
        "local_edit" => (through / 30).saturating_sub(199 / 30),
        _ => 0,
    }
}

#[cfg(test)]
mod edit_protocol_tests {
    use super::*;
    #[test]
    fn exact_schedule_boundaries_and_formal_interval() {
        for frame in 0..=3601 {
            for scene in ["static", "many_draws", "local_edit", "continuous_edit"] {
                let enumerated = (1..=frame).filter(|&f| edit_due(scene, f)).count() as u64;
                assert_eq!(expected_edits(scene, frame), enumerated, "{scene}:{frame}");
            }
        }
        assert_eq!(expected_edits("local_edit", 600), 14);
        assert_eq!(expected_edits("local_edit", 3601), 114);
        assert_eq!(expected_edits("continuous_edit", 600), 401);
        assert_eq!(expected_edits("continuous_edit", 3601), 3402);
    }
    #[test]
    fn exact_four_tiles_and_no_gpu_claim() {
        assert_eq!((0..4).map(edit_tile_index).collect::<Vec<_>>(), vec![2015,2016,2079,2080]);
        assert!(!edit_due("local_edit", 200));
        assert!(edit_due("local_edit", 210));
        assert!(edit_due("continuous_edit", 200));
    }
}
