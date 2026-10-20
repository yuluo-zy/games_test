"""Read production; emit reviewable candidates only. Never apply them."""
from pathlib import Path
import difflib, hashlib, json, subprocess

ROOT = Path('D:/game')
OUT = Path(__file__).parent / 'candidates-v23'
OUT.mkdir(exist_ok=True)
patches = []
manifest = []
def candidate(rel, transform):
    source = ROOT / rel
    old = source.read_text(encoding='utf-8')
    new = transform(old)
    assert old != new, rel
    target = OUT / rel.removeprefix('tiny-glade/')
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_text(new, encoding='utf-8', newline='\n')
    if target.suffix == '.rs':
        subprocess.run(['C:/Users/liyu/.rustup/toolchains/stable-x86_64-pc-windows-msvc/bin/rustfmt.exe', '--edition', '2024', '--config', 'skip_children=true', str(target)],check=True)
    new = target.read_text(encoding='utf-8')
    patches.append(''.join(difflib.unified_diff(old.splitlines(True), new.splitlines(True), fromfile='a/'+rel, tofile='b/'+rel)))
    manifest.append({'path':rel,'source_sha256':hashlib.sha256(source.read_bytes()).hexdigest(),'candidate':str(target),'candidate_sha256':hashlib.sha256(target.read_bytes()).hexdigest()})

def new_candidate(rel,new):
    assert not (ROOT/rel).exists(),rel
    target=OUT/rel.removeprefix('tiny-glade/')
    target.parent.mkdir(parents=True,exist_ok=True);target.write_text(new,encoding='utf-8',newline='\n')
    subprocess.run(['C:/Users/liyu/.rustup/toolchains/stable-x86_64-pc-windows-msvc/bin/rustfmt.exe','--edition','2024','--config','skip_children=true',str(target)],check=True)
    new=target.read_text(encoding='utf-8')
    patches.append(''.join(difflib.unified_diff([],new.splitlines(True),fromfile='/dev/null',tofile='b/'+rel)))
    manifest.append({'path':rel,'source_sha256':None,'candidate':str(target),'candidate_sha256':hashlib.sha256(target.read_bytes()).hexdigest()})

candidate('tiny-glade/crates/country-core/src/asset_libraries/shader_library.rs', lambda s: s.replace('pub struct AssetShaderLibrary {', '''/// One Arc owner of the current App's physical shader registry. Consumers in
/// other crates borrow the same TypeId; no second AssetShaderLibrary is created.
#[derive(bevy_ecs::resource::Resource, Clone)]
pub struct SharedAssetShaderLibrary(pub std::sync::Arc<AssetShaderLibrary>);

pub struct AssetShaderLibrary {''',1))
def gates_source(s):
    s=s.replace('shaders: Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>', 'shaders: Res<country_core::asset_libraries::shader_library::SharedAssetShaderLibrary>').replace('let shader = shaders\n', 'let shader = shaders.0\n')
    s=s.replace('    ssbo: Res<AssetSsboLibrary>,','    mut ssbo: ResMut<AssetSsboLibrary>,',1)
    s=s.replace('''    let mut brick_updates = hashbrown::HashMap::<u64, Vec<_>>::new();''','''    let mut exhausted = false;''',1)
    s=s.replace('''        brick_updates
            .entry(wall)
            .or_default()
            .extend(outcome.bricks);''','''        let storage = ssbo.get_mut::<country_core::geometry::instanced_wall::BrickTransformSsbo>()
            .expect("canonical BrickTransformSsbo must exist before gate construction");
        exhausted |= output.gpu.append_bricks(storage,crate::gpu_output::ConstructorKey {
            wall, part:crate::gpu_output::ConstructorPart::Gates,
        },&outcome.bricks) != 0;''',1)
    a=s.index('    for (wall, records) in brick_updates {');b=s.index('\n}\n',a)
    s=s[:a]+'''    if exhausted {
        log::error!(target:"system_wall_constructor::construct_gates",
            "Gate constructor ran out of bricks");
    }'''+s[b:]
    # Native update_gates_hashmap drains tracked entities only. Shared arch/gate
    # brick allocation is cleared once by construct_arches in22, not here28.
    start=s.index('pub fn update_gates_hashmap(')
    prefix=s[:start];tail=s[start:]
    tail=tail.replace('''    mut output: Option<ResMut<crate::construct_arches::WallConstructorOutput>>,''','',1)
    block='''        if let Some(output) = output.as_mut() {
            output.gpu.replace_bricks(
                crate::gpu_output::ConstructorKey {
                    wall: wall.0,
                    part: crate::gpu_output::ConstructorPart::Gates,
                },
                Vec::new(),
            );
        }
'''
    assert tail.count(block)==2
    tail=tail.replace(block,'')
    return prefix+tail
candidate('tiny-glade/crates/systems/wall-constructor/src/construct_gates/systems.rs', gates_source)
candidate('tiny-glade/crates/tiny-glade/src/run_game.rs', lambda s: s.replace('''#[derive(Resource, Clone)]
pub struct ApplicationShaderLibrary(
    pub Arc<country_core::asset_libraries::shader_library::AssetShaderLibrary>,
);''', '''pub use country_core::asset_libraries::shader_library::SharedAssetShaderLibrary
    as ApplicationShaderLibrary;''',1))

def stats(s):
    s=s.replace('pub fences_segments_seed: u64,', 'pub previous_fences_segments_seed: u64,\n    pub fences_segments_seed: u64,',1)
    s=s.replace('pub counts: [u32; 8],','pub previous_counts: [u32; 8],\n    pub counts: [u32; 8],',1)
    return s.replace('impl MaterialStats {', '''impl MaterialStats {
    /// Original publisher140a8b530 moves old current half into new previous;
    /// the new current half starts empty. This happens only on row publication.
    pub fn for_new_publication(old: &mut Self) -> Self {
        Self {
            previous_lengths: old.lengths,
            previous_pillar_placements: std::mem::take(&mut old.pillar_placements),
            previous_fences_segments_seed: old.fences_segments_seed,
            previous_counts: old.counts,
            ..Self::default()
        }
    }
''',1)
candidate('tiny-glade/crates/country-core/src/resources/walls/material_stats.rs', stats)
def public_state(s):
    s=s.replace('pub struct PublicWallState {','''pub struct PublicWallState {
    pub material_stats: super::material_stats::MaterialStats,
    /// Original +110/+114 readiness and scalar. Distinct from optional profile
    /// geometry: a correctly initialized no-roof wall still owns a base value.
    gable_maximum_y: Option<f32>,''',1)
    s=s.replace('''        Ok(Self {
            authority:''','''        Ok(Self {
            material_stats: super::material_stats::MaterialStats::default(),
            gable_maximum_y: None,
            authority:''',1)
    s=s.replace('''        self.gable_heights = None;
        self.segment_heights.clear();''','''        self.gable_heights = None;
        self.gable_maximum_y = None;
        self.segment_heights.clear();''',1)
    s=s.replace('''        self.gable_heights
            .as_ref()
            .expect(MISSING_GABLE_HEIGHT)
            .maximum_y''','''        self.gable_maximum_y.expect(MISSING_GABLE_HEIGHT)''',1)
    s=s.replace('''        self.gable_heights = None;
        if let (Some(base), Some(roof)) = (&self.regular_heights, roof) {
            self.gable_heights = Some(apply_gable_arch_heights(''','''        self.gable_heights = None;
        // Original78bc70..bc96 writes both ready flags and the base scalar
        // before it even tests rectangle/roof. Never fallback in the getter.
        self.gable_maximum_y = Some(self.calculated_max_y().expect(MISSING_HEIGHT));
        if self.is_rectangle().is_some()
            && let (Some(base), Some(roof)) = (&self.regular_heights, roof)
        {
            let gable = apply_gable_arch_heights(''',1)
    s=s.replace('''                roof,
            )?);
        }
        Ok(())
    }

    pub fn get_wall_attachment_height''','''                roof,
            )?;
            self.gable_maximum_y = Some(gable.maximum_y);
            self.gable_heights = Some(gable);
        }
        Ok(())
    }

    pub fn get_wall_attachment_height''',1)
    old='''            bytes[0x110..0x114].copy_from_slice(&1u32.to_le_bytes());
            let gable = self
                .gable_heights
                .as_ref()
                .map(|h| h.maximum_y)
                .unwrap_or(base);
            bytes[0x114..0x118].copy_from_slice(&gable.to_le_bytes());'''
    assert old in s
    s=s.replace(old,'''        }
        if let Some(gable) = self.gable_maximum_y {
            bytes[0x110..0x114].copy_from_slice(&1u32.to_le_bytes());
            bytes[0x114..0x118].copy_from_slice(&gable.to_le_bytes());''',1)
    s+='''
#[cfg(test)]
mod initialization_tests {
    use super::*;
    #[test]
    fn no_roof_gable_maximum_has_explicit_ready_reset_lifecycle() {
        use crate::resources::roofs::geometry::surface::Curve2;
        use std::sync::Arc;
        let authority = WallState::new(
            Curve2::try_new(vec![[0.,0.],[8.,0.]],false).unwrap(),
            3.,0,2,false,
        );
        let terrain = TerrainHeightGrid512 {
            encoded: vec![0;512*512].into(), minimum_y:0., maximum_y:0.,
            sampling_fix:false,
        };
        let paths = PathRasters512::from_encoded(
            std::array::from_fn(|_| Arc::<[u8]>::from(vec![0;512*512])), false,
        ).unwrap();
        let mut wall = PublicWallState::collect(&authority,&terrain).unwrap();
        wall.prepare_paths(&paths).unwrap();
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| wall.max_y_gable())).is_err());
        assert!(wall.building_projection().max_y_gable().is_err());
        wall.update_gable_heights(None).unwrap();
        assert_eq!(wall.max_y_gable().to_bits(), wall.max_y().to_bits());
        assert_eq!(wall.building_projection().max_y_gable().unwrap().to_bits(),wall.max_y_gable().to_bits());
        assert!(wall.gable_heights.is_none());
        wall.prepare_paths(&paths).unwrap();
        assert!(wall.building_projection().max_y_gable().is_err());
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| wall.max_y_gable())).is_err());
        wall.update_gable_heights(None).unwrap();
        assert_eq!(wall.max_y_gable().to_bits(), wall.max_y().to_bits());
    }
}
'''
    return s
candidate('tiny-glade/crates/country-core/src/resources/walls/public_wall_state.rs',public_state)
candidate('tiny-glade/crates/country-core/src/resources/walls/public_walls.rs', lambda s: s.replace('''                self.records.insert(id, record);''','''                if let Some(old) = self.records.get_mut(&id) {
                    record.material_stats = super::material_stats::MaterialStats::for_new_publication(
                        &mut old.material_stats,
                    );
                }
                self.records.insert(id, record);''',1))

def segmentation(s):
    s=s.replace('    walls: Res<PublicWalls>,','    mut walls: ResMut<PublicWalls>,',1).replace('''        let Some(wall) = walls.get(event.0) else {
            continue;
        };''','''        let wall = walls.get_mut(event.0)
            .expect("segmentation changed event must reference a published wall");''',1)
    s=s.replace('observed_special_flag: projection.observed_294()', 'observed_special_flag: projection.observed_296()',1)
    a=s.index('        if let Ok(mask) = wall_path_segmentation(input) {')
    b=s.index('\n    }\n}',a)
    return s[:a]+'''        let mask = wall_path_segmentation(input)
            .expect("invalid authoritative wall path segmentation input");
        wall.material_stats.init_this_update(path.length, projection.observed_296() != 0);
        wall.material_stats.set_path_segmentation_count(mask.len() as u32);
        masks.masks.insert(event.0.0, mask);'''+s[b:]
candidate('tiny-glade/crates/systems/wall-constructor/src/segmentation/calculate_wall_path_segmentation.rs',segmentation)
candidate('tiny-glade/crates/country-core/src/resources/walls/publication_fields.rs', lambda s: s.replace('''    pub fn validate_read(&self)''','''    /// Identified observed fields only; owning Rust vectors are never encoded.
    pub fn observed_bytes(&self) -> &[u8; 0x2a0] { &self.0 }
    pub fn validate_read(&self)''',1))
candidate('tiny-glade/crates/systems/wall-constructor/src/construct_platform_stairs/mod.rs', lambda s: s.replace('''    /// 墙 id → 平台段记录（布局 I）。
    pub segments: hashbrown::HashMap<u64, Vec<[u8; 104]>>,''','''    /// Native32B threshold ranges; the28 creator materializes104B segments.
    pub segments: hashbrown::HashMap<u64, Vec<country_core::systems::wall::construction::platform_segmentation::WallPlatformStepRange>>,''',1))

def splitter(_s):
    return '''//! B140751960: raw136B masks -> processed136B masks + native32B step ranges.
//! Original set22 runs this after relationship/invalid-roof updates and before
//! arches/lintels, using chain_ignore_deferred (no inserted Commands barrier).
use bevy_ecs::prelude::*;
use country_core::resources::{
    roofs::{Roof, commands::{CreateRoofCmd, SwitchRoofTypeCmd}},
    walls::{WallId, public_walls::{OnWallChanged, PublicWalls}},
};
use country_core::systems::ui::signifiers::signifier_create::UiSignifierStream;
use country_core::systems::wall::construction::platform_segmentation::split_platform_wall_masks;
use crate::{
    construct_arches::WallPathSegmentationMasks,
    construct_lintels::WallPathSegmentationMasksMinusStairs,
    construct_platform_stairs::PlatformStairsStorage,
};

pub fn split_wall_path_segment_into_platform_stairs(
    mut changed: EventReader<OnWallChanged>,
    mut create_roof: EventReader<CreateRoofCmd>,
    mut switch_type: EventReader<SwitchRoofTypeCmd>,
    walls: Res<PublicWalls>,
    roofs: Query<&Roof>,
    mut raw: ResMut<WallPathSegmentationMasks>,
    mut processed: ResMut<WallPathSegmentationMasksMinusStairs>,
    mut steps: ResMut<PlatformStairsStorage>,
    _signifiers: Res<UiSignifierStream>,
) {
    let mut recalc: Vec<u64> = changed.read().map(|e| e.0.0)
        .chain(create_roof.read().map(|e| e.0.wall_id().0))
        .chain(switch_type.read().map(|e| e.wall.0)).collect();
    // Native751c..751d performs unsigned sort followed by adjacent dedup.
    recalc.sort_unstable();
    recalc.dedup();
    for wall_id in recalc {
        let masks = raw.masks.get_mut(&wall_id)
            .expect("platform segmentation requires raw masks for every event wall");
        let id = WallId(wall_id);
        let wall = walls.get(id)
            .expect("platform segmentation event must reference a published wall");
        let ranges = steps.segments.entry(wall_id).or_default();
        ranges.clear(); // Native clears even when the non-flat/shape>=2 clone branch follows.
        let roof = walls.roof_component(id, |entity| roofs.get(entity).ok());
        let projection = wall.building_projection();
        let path = wall.prepared_path.as_ref()
            .expect("platform segmentation requires prepared wall path");
        let result = split_platform_wall_masks(
            projection.observed_bytes(), &path.positions, &path.segment_values,
            masks, roof.map(Roof::observed),
        ).expect("invalid authoritative platform segmentation input");
        processed.masks.insert(wall_id, result.processed_masks);
        *ranges = result.steps;
    }
}
'''
candidate('tiny-glade/crates/systems/wall-constructor/src/segmentation/split_wall_path_segment_into_platform_stairs.rs',splitter)
def arches_system(s):
    s=s.replace('    ssbo: Res<AssetSsboLibrary>,','    mut ssbo: ResMut<AssetSsboLibrary>,',1)
    s=s.replace('    masks: Res<WallPathSegmentationMasks>,', '    masks: Res<crate::construct_lintels::WallPathSegmentationMasksMinusStairs>,',1)
    s=s.replace('''    roofs: Query<&Roof>,''','''    roofs: Query<&Roof>,
    mut previous: ResMut<country_core::resources::walls::arches::BuildingHadArchesLastFrame>,
    mut recalculate: EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,''',1)
    s=s.replace('''        let has_roof = walls
            .roof_entity(id)
            .and_then(|entity| roofs.get(entity).ok())
            .is_some();''','''        // Native76047b calls PublicWalls::has_roof, including its failure-map
        // and generation test. Query validation belongs only to the tail below.
        let has_roof = walls.has_roof(id);''',1)
    s=s.replace('''        let Some(segments) = masks.masks.get(&wall_id) else {
            continue;
        };
        let Some(wall) = walls.get_mut(id) else {
            continue;
        };
        let outcome = wall.construct_arches(id, segments, has_roof);
        if let Ok(outcome) = outcome {''','''
        let wall = walls.get_mut(id)
            .expect("arch event must reference a published wall");
        let storage = ssbo.get_mut::<country_core::geometry::instanced_wall::BrickTransformSsbo>()
            .expect("canonical BrickTransformSsbo must exist before arch construction");
        output.gpu.clear_bricks(storage,wall_id);
        let outcome = if wall.building_projection().observed_296() != 0 {
            country_core::systems::wall::construction::arches::WallArchesOutput::default()
        } else {
            let segments = masks.masks.get(&wall_id)
                .expect("arches require processed masks for the source wall");
            wall.construct_arches(id, segments, has_roof)
                .expect("invalid or unrecovered authoritative arch input")
        };
        {
            let stats = &outcome.material_stats;
            wall.material_stats.set_arches_segments_count(stats.arches_segments_count());
            wall.material_stats.add_arches_length(stats.arches_length);
            wall.material_stats.add_gate_length(stats.gates_length);
            wall.material_stats.set_gates_segments_count(stats.gates_count as u32);
            wall.material_stats.add_sewer_length(stats.sewer_length);
            wall.material_stats.set_sewer_segments_count(stats.sewer_count as u32);''',1)
    s=s.replace('''    let recalc: Vec<u64>''','''    let mut recalc: Vec<u64>''',1)
    s=s.replace('''    for wall_id in recalc {''','''    recalc.sort_unstable();
    recalc.dedup();
    let mut exhausted = false;
    for wall_id in recalc {''',1)
    s=s.replace('''            output.gpu.replace_bricks(
                crate::gpu_output::ConstructorKey''','''            exhausted |= output.gpu.append_bricks(
                storage,
                crate::gpu_output::ConstructorKey''',1)
    s=s.replace('''                outcome.instances,
            );''','''                &outcome.instances,
            ) != 0;''',1)
    start=s.index('        // shape2 分支（style==2 → MaterialStats 直写）')
    end=s.index('\n    }',start)
    s=s[:start]+s[end:]
    at=s.index('\n}\n',s.index('pub fn construct_arches('))
    s=s[:at]+'''
    if exhausted {
        log::error!(target:"system_wall_constructor::construct_arches",
            "Arch constructor ran out of bricks");
    }
    // Original762ce0..7630a5 scans all public rows every invocation, not only
    // this frame's event IDs. Only a live, non-flat associated roof participates.
    for (id, _) in walls.all() {
        let Some(entity) = walls.roof_entity(id)
            .filter(|entity| entity.to_bits() >> 32 != 0) else { continue; };
        let Ok(roof) = roofs.get(entity) else { continue; };
        if roof.is_flat() { continue; }
        let now = arch_segments.0.get(&id).is_some_and(|segments| segments.len() >= 2);
        let before = previous.0.get(&id.0).copied().unwrap_or(false);
        if before != now {
            recalculate.write(country_core::resources::roofs::commands::RecalculateRoofCmd {
                entity, wall:id, kind:3,
            });
        }
        previous.0.insert(id.0,now);
    }'''+s[at:]
    return s
candidate('tiny-glade/crates/systems/wall-constructor/src/construct_arches/mod.rs', arches_system)

candidate('tiny-glade/crates/country-core/src/resources/walls/arches.rs',lambda s:s.replace('''#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct BuildingHadArchesLastFrame(pub bool);''','''#[derive(Resource, Clone, Debug, Default, PartialEq)]
pub struct BuildingHadArchesLastFrame(pub std::collections::HashMap<u64, bool>);''',1))

new_candidate('tiny-glade/crates/country-core/src/systems/wall/construction/arch_stats.rs','''//! Original760480..761142 /7629b1..2a49 material classification and counters.
//! This observes pre-profile materialized segments and raw mask length.
use super::structures::MaterializedWallSegment;
#[derive(Clone,Debug,Default)]
pub struct WallArchMaterialStats {
    pub total_segments:u64,
    pub arches_length:f32,
    pub gates_length:f32,
    pub sewer_length:f32,
    pub gates_count:u64,
    pub sewer_count:u64,
}
impl WallArchMaterialStats {
    pub fn observe_mask(&mut self,length:f32,segments:&[MaterializedWallSegment],has_roof:bool) {
        self.total_segments += segments.len() as u64;
        if !has_roof || segments.len() != 1 {
            self.arches_length += length;
            return;
        }
        let points=&segments[0].positions;
        let mut negative=0.0f32;
        for p in points { if p[1] < 0.0 { negative += 1.0; } }
        // Original UCOMISS/JBE: unordered ratio takes the gate side.
        if negative / points.len() as f32 > f32::from_bits(0x3f000000) {
            self.sewer_length += length;
            self.sewer_count += 1;
        } else {
            self.gates_length += length;
            self.gates_count += 1;
        }
    }
    pub fn arches_segments_count(&self)->u32 {
        // The native subtracts gate count only; sewer remains included.
        self.total_segments.wrapping_sub(self.gates_count) as u32
    }
}
''')
candidate('tiny-glade/crates/country-core/src/systems/wall/construction/mod.rs',lambda s:s+'\npub mod arch_stats;\n')

def arches_cpu(s):
    s=s.replace('#[derive(Clone, Debug)]\npub struct WallArchesOutput {','#[derive(Clone, Debug, Default)]\npub struct WallArchesOutput {\n    pub material_stats: super::arch_stats::WallArchMaterialStats,',1)
    s=s.replace('''    let mut out = WallArchesOutput {
        instances:''','''    let mut out = WallArchesOutput {
        material_stats: Default::default(),
        instances:''',1)
    s=s.replace('''        let override_height = if input.observed_294 != 0 {''','''        out.material_stats.observe_mask(mask.length,&segments,input.has_roof);
        let override_height = if input.observed_294 != 0 {''',1)
    return s
candidate('tiny-glade/crates/country-core/src/systems/wall/construction/arches.rs',arches_cpu)
def freehand_stats(s):
    s=s.replace('''    let mut out = WallArchesOutput {
        instances:''','''    let mut out = WallArchesOutput {
        material_stats: Default::default(),
        instances:''',1)
    s=s.replace('''        let override_height = if input.observed_294 != 0 {''','''        out.material_stats.observe_mask(mask.length,&ss,input.has_roof);
        let override_height = if input.observed_294 != 0 {''',1)
    return s
candidate('tiny-glade/crates/country-core/src/systems/wall/construction/freehand_arches.rs',freehand_stats)

def heights(_s):
    return '''//! B14078b550 relationship portion of wall height initialization.
//! Prepared base/third caches belong to the actual PublicWallState path owner.
use bevy_ecs::prelude::*;
use country_core::resources::{
    roofs::Roof,
    walls::public_walls::{OnWallChanged, PublicWalls},
};

pub fn init_wall_height_and_arch_segment_height(
    mut changed: EventReader<OnWallChanged>,
    mut walls: ResMut<PublicWalls>,
    roofs: Query<&Roof>,
) {
    for event in changed.read() {
        // Native PublicWalls table absence is unwrap_failed, not silent skip.
        let wall = walls.get(event.0)
            .expect("wall height event must reference a published wall");
        // Native query is reached only for a rectangle. A present association
        // must resolve the same generation; another roof with equal WallId
        // cannot replace a stale/missing associated component.
        let roof = if wall.is_rectangle().is_some() {
            walls.roof_entity(event.0)
                .filter(|entity| entity.to_bits() >> 32 != 0)
                .map(|entity| roofs.get(entity)
                    .expect("associated rectangle roof component must resolve"))
        } else {
            None
        };
        walls.get_mut(event.0).unwrap()
            .update_gable_heights(roof.map(Roof::observed))
            .expect("invalid authoritative gable height input");
    }
}
'''
candidate('tiny-glade/crates/systems/wall-constructor/src/init_wall_height_and_arch_segment_height.rs', heights)

def plank_output(s):
    s=s.replace('bricks: HashMap<ConstructorKey, Part<BrickTransformSsbo>>,','bricks: HashMap<u64, SparseInstanceBufferAllocation<BrickTransformSsbo>>,',1)
    s=s.replace('planks: HashMap<ConstructorKey, Part<WoodenPlankSsbo>>,','planks: HashMap<u64, SparseInstanceBufferAllocation<WoodenPlankSsbo>>,',1)
    a=s.index('    pub fn replace_bricks(');b=s.index('    pub fn replace_planks(',a)
    s=s[:a]+'''    pub fn append_bricks(
        &mut self,
        storage: &mut SparseInstanceBuffer<BrickTransformSsbo>,
        key: ConstructorKey,
        records: &[BrickTransformSsbo],
    ) -> usize {
        let owner = self.bricks.entry(key.wall)
            .or_insert_with(|| storage.new_allocation());
        assert!(storage.owns_allocation(owner), "arch/gate owner belongs to another physical storage");
        let mut writer = storage.append_writer(owner);
        let mut failures = 0usize;
        for record in records { if writer.push(record).is_err() { failures += 1; } }
        if failures != 0 {
            self.failures.insert(key, format!("{failures} constructor brick pushes exhausted capacity; successful prefix remains"));
        }
        failures
    }
    /// Original22 arch constructor clears the one shared arch/gate owner.
    pub fn clear_bricks(&mut self, storage: &mut SparseInstanceBuffer<BrickTransformSsbo>, wall:u64) {
        let owner = self.bricks.entry(wall).or_insert_with(||storage.new_allocation());
        assert!(storage.owns_allocation(owner), "arch/gate owner belongs to another physical storage");
        storage.writer(owner).clear();
        self.failures.retain(|key,_|key.wall!=wall || !matches!(key.part,ConstructorPart::Arches|ConstructorPart::Gates));
    }
'''+s[b:]
    a=s.index('    pub fn replace_planks(');b=s.index('    pub fn pending_bricks(',a)
    s=s[:a]+'''    /// Original per-wall64B owner is shared by lintels/fences. Source systems
    /// append at their actual22/28 execution points, never deferred/reordered by
    /// HashMap iteration during44 upload. Every failed push is still attempted.
    pub fn append_planks(
        &mut self,
        storage: &mut SparseInstanceBuffer<WoodenPlankSsbo>,
        key: ConstructorKey,
        records: &[WoodenPlankSsbo],
    ) -> usize {
        let owner = self.planks.entry(key.wall)
            .or_insert_with(|| storage.new_allocation());
        assert!(storage.owns_allocation(owner), "plank owner belongs to another physical storage");
        let mut writer = storage.append_writer(owner);
        let mut failures = 0usize;
        for record in records {
            if writer.push(record).is_err() {
                failures += 1;
            }
        }
        if failures != 0 {
            self.failures.insert(key, format!("{failures} constructor plank pushes exhausted capacity; successful prefix remains"));
        }
        failures
    }
    pub fn clear_planks(
        &mut self,
        storage: &mut SparseInstanceBuffer<WoodenPlankSsbo>,
        wall: u64,
    ) {
        let owner = self.planks.entry(wall)
            .or_insert_with(|| storage.new_allocation());
        assert!(storage.owns_allocation(owner), "plank owner belongs to another physical storage");
        storage.writer(owner).clear();
        self.failures.retain(|key,_| key.wall != wall ||
            !matches!(key.part,ConstructorPart::Lintels|ConstructorPart::Fences));
    }
'''+s[b:]
    a=s.index('    pub fn pending_bricks(');b=s.index('    pub fn brick_allocation(',a);s=s[:a]+s[b:]
    s=s.replace('self.bricks.get(&key)?.allocation.as_ref()', 'self.bricks.get(&key.wall)',1)
    s=s.replace('self.bricks.retain(|key, _| key.wall != wall);','self.bricks.retain(|id, _| *id != wall);',1)
    s=s.replace('self.planks.get(&key)?.allocation.as_ref()', 'self.planks.get(&key.wall)',1)
    s=s.replace('self.planks.retain(|key, _| key.wall != wall);','self.planks.retain(|id, _| *id != wall);',1)
    s=s.replace('        write_parts(storage, planks, failures);\n','',1)
    s=s.replace('        write_parts(storage, bricks, failures);\n','',1)
    s=s.replace('        failures,\n        last_device_error,','        failures: _,\n        last_device_error,',1)
    s=s.replace('struct Part<T> {','#[cfg(test)]\nstruct Part<T> {',1)
    s=s.replace('impl<T> Default for Part<T> {','#[cfg(test)]\nimpl<T> Default for Part<T> {',1)
    s=s.replace('fn write_parts<T: SparseSsbo>(', '#[cfg(test)]\nfn write_parts<T: SparseSsbo>(',1)
    start=s.index('pub fn delete_constructor_output(')
    s=s[:start]+'''/// Original19 clear, after publication and before22/28 source appenders.
pub fn clear_constructor_planks(
    mut changed: EventReader<country_core::resources::walls::public_walls::OnWallChanged>,
    walls: Res<country_core::resources::walls::public_walls::PublicWalls>,
    mut library: ResMut<AssetSsboLibrary>,
    mut output: ResMut<crate::construct_arches::WallConstructorOutput>,
) {
    for event in changed.read() {
        walls.get(event.0).expect("plank clear changed event requires published wall");
        let storage = library.get_mut::<WoodenPlankSsbo>()
            .expect("canonical WoodenPlankSsbo must exist before wall construction");
        output.gpu.clear_planks(storage,event.0.0);
    }
}
'''+s[start:]
    # Replace obsolete transactional-batch test with the recovered source-owner
    # contract. Keep the actual user's sparse implementation in every path.
    a=s.index('#[cfg(test)]\nstruct Part<T>');b=s.index('#[derive(Resource, Default)]',a)
    s=s[:a]+s[b:]
    a=s.index('#[cfg(test)]\nfn write_parts<');b=s.index('/// Run after source constructors',a)
    s=s[:a]+s[b:]
    s=s.replace('SparseInstanceBuffer, SparseInstanceBufferAllocation, SparseSsbo,','SparseInstanceBuffer, SparseInstanceBufferAllocation,',1)
    a=s.index('    #[test]\n    fn source_replacement_deletion_and_foreign_storage');b=s.index('    #[test]\n    fn plank_projection',a)
    s=s[:a]+'''    #[test]
    fn arch_gate_tail_owner_clear_delete_and_foreign_storage_are_real() {
        let key=ConstructorKey{wall:44,part:ConstructorPart::Arches};
        let gates=ConstructorKey{part:ConstructorPart::Gates,..key};
        let mut owners=ConstructorGpuOutput::default();
        let mut storage=SparseInstanceBuffer::<BrickTransformSsbo>::new_storage_buffer(&TestDevice(1),None);
        owners.clear_bricks(&mut storage,key.wall);
        assert_eq!(owners.append_bricks(&mut storage,key,&vec![BrickTransformSsbo{seed:7,..Default::default()};35]),0);
        assert_eq!(owners.append_bricks(&mut storage,gates,&vec![BrickTransformSsbo{seed:9,..Default::default()};2]),0);
        assert!(std::ptr::eq(owners.brick_allocation(key).unwrap(),owners.brick_allocation(gates).unwrap()));
        assert_eq!(storage.to_draw_multi_instanced(),vec![(0,37)]);
        assert_eq!(storage.data[34].seed,7);assert_eq!(storage.data[35].seed,9);
        owners.clear_bricks(&mut storage,key.wall);
        assert!(storage.to_draw_multi_instanced().is_empty());
        assert_eq!(owners.append_bricks(&mut storage,key,&vec![BrickTransformSsbo{seed:11,..Default::default()};2]),0);
        assert_eq!(storage.to_draw_multi_instanced(),vec![(0,2)]);
        owners.remove_wall(44);storage.take_out_waste();
        assert!(storage.to_draw_multi_instanced().is_empty());
        owners.append_bricks(&mut storage,key,&[BrickTransformSsbo::default()]);
        drop(storage); // Avoid simultaneously allocating two full192MiB shadows.
        let mut foreign=SparseInstanceBuffer::<BrickTransformSsbo>::new_storage_buffer(&TestDevice(2),None);
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||
            owners.append_bricks(&mut foreign,key,&[BrickTransformSsbo::default()])
        )).is_err());
        assert!(foreign.to_draw_multi_instanced().is_empty());
        owners.remove_wall(44);
    }
'''+s[b:]
    s=s.replace('''//! Rebuild replaces a complete part, deletion releases it, failed allocation or
//! device upload retains the pending revision for the next flush.''','''//! Original source points clear and append shared per-wall owners directly.
//! Capacity failure keeps successful prefixes; physical upload failures keep
//! dirty chunks for the real device retry. Deletion releases the owned chunks.''',1)
    return s
candidate('tiny-glade/crates/systems/wall-constructor/src/gpu_output.rs', plank_output)

def lintel_append(s):
    s=s.replace('    ssbo: Res<AssetSsboLibrary>,','    mut ssbo: ResMut<AssetSsboLibrary>,',1)
    s=s.replace('''    let recalc: Vec<u64>''','''    let mut recalc: Vec<u64>''',1)
    s=s.replace('''    for wall_id in recalc {''','''    recalc.sort_unstable();
    recalc.dedup();
    for wall_id in recalc {''',1)
    s=s.replace('''        let projection = wall.building_projection();
        let Some(segments) = masks.masks.get(&wall_id) else {
            continue;
        };''','''        let projection = wall.building_projection();
        if projection.observed_296() == 0 {
            // Native76e198..19d bypasses the complete ordinary-wall mask loop.
            wall.material_stats.set_lintels_segments_count(0);
            wall.material_stats.add_lintels_length(0.0);
            continue;
        }
        let segments = masks.masks.get(&wall_id)
            .expect("lintels require processed masks for the source wall");''',1)
    s=s.replace('''        let mut instances = Vec::new();''','''        let mut instances = Vec::new();
        let mut stats_count = 0u64;
        let mut stats_length = 0.0f32;''',1)
    s=s.replace('''            if let Ok(planks) = construct_lintel_supports(''','''            // Observe the same pre-profile materializer; complete retained
            // LintelSegments/pillar-proposal ownership is not implied here.
            let materialized = country_core::systems::wall::construction::structures::materialize_wall_segments(source)
                .expect("invalid authoritative lintel materialization input");
            stats_count += materialized.len() as u64;
            stats_length += mask.length;
            if let Ok(planks) = construct_lintel_supports(''',1)
    s=s.replace('''        output.gpu.replace_planks(
            crate::gpu_output::ConstructorKey''','''        let storage = ssbo.get_mut::<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>()
            .expect("canonical WoodenPlankSsbo must exist before lintel construction");
        output.gpu.append_planks(
            storage,
            crate::gpu_output::ConstructorKey''',1)
    s=s.replace('''            instances,
        );
        let _ = ssbo;''','''            &instances,
        );
        wall.material_stats.set_lintels_segments_count(stats_count as u32);
        wall.material_stats.add_lintels_length(stats_length);''',1)
    return s
candidate('tiny-glade/crates/systems/wall-constructor/src/construct_lintels/mod.rs',lintel_append)

def fence_append(s):
    s=s.replace('    walls: Res<PublicWalls>,','    mut walls: ResMut<PublicWalls>,',1)
    s=s.replace('    ssbo: Res<AssetSsboLibrary>,','    mut ssbo: ResMut<AssetSsboLibrary>,',1)
    s=s.replace('''    for event in changed.read() {
        let Some(wall) = walls.get(event.0) else {
            continue;
        };''','''    let mut exhausted = false;
    for event in changed.read() {
        let wall = walls.get_mut(event.0)
            .expect("fence changed event requires a published wall");''',1)
    a=s.index('            output.gpu.replace_planks(');b=s.index('            continue;',a)
    s=s[:a]+'''            wall.material_stats.set_fences_segments_count(0);
            wall.material_stats.set_fences_segments_seed(0);
'''+s[b:]
    a=s.index('        if let Ok(outcome) = construct_fences_cpu(input) {');b=s.index('\n    }\n}',a)
    s=s[:a]+'''        let outcome = construct_fences_cpu(input)
            .expect("invalid or unrecovered authoritative fence input");
        let instances:Vec<_> = outcome.instances.iter()
            .map(crate::gpu_output::wooden_plank).collect();
        let storage = ssbo.get_mut::<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>()
            .expect("canonical WoodenPlankSsbo must exist before fence construction");
        exhausted |= output.gpu.append_planks(storage,crate::gpu_output::ConstructorKey {
            wall:event.0.0, part:crate::gpu_output::ConstructorPart::Fences,
        }, &instances) != 0;
        // Native765e20..e38 writes these even after partial capacity failure.
        wall.material_stats.set_fences_segments_count(outcome.fence_segments_count as u32);
        wall.material_stats.set_fences_segments_seed(outcome.fence_segments_seed);
'''+s[b:]
    end=s.index('\n}\n\n/// B `split_curve_based_on_angle`')
    s=s[:end]+'''
    // Original record level=1 (Error), one enabled log after all wall rows.
    if exhausted {
        log::error!(target:"system_wall_constructor::construct_fences",
            "Fence-pillar constructor ran out of fence pieces");
    }'''+s[end:]
    return s
candidate('tiny-glade/crates/systems/wall-constructor/src/construct_fences/mod.rs',fence_append)

# log0.4.27 is already the original workspace dependency; no new implementation.
candidate('tiny-glade/crates/systems/wall-constructor/Cargo.toml',lambda s:s.replace('lerp = "=0.4.0"','lerp = "=0.4.0"\nlog.workspace = true',1))

candidate('tiny-glade/crates/country-core/src/render/sparse_instance_buffer.rs',lambda s:s.replace('''    /// 防止相同元素类型的两个缓冲误用彼此的 chunk 编号。
    pub fn owns_allocation''','''    /// Resume an existing source owner's tail chunk. Native constructor
    /// callers choose the allocation's last chunk (or the full sentinel when
    /// empty); generic `writer` retains its existing sentinel-first contract.
    /// All allocation, push, clear and batch-replace algorithms stay shared.
    pub fn append_writer<'a>(
        &'a mut self,
        allocation: &'a SparseInstanceBufferAllocation<T>,
    ) -> SparseInstanceBufferWriter<'a,T> {
        let mut writer = self.writer(allocation);
        if let Some(&chunk) = writer.allocation_chunks.last() {
            writer.current_chunk = chunk;
        }
        writer
    }

    /// 防止相同元素类型的两个缓冲误用彼此的 chunk 编号。
    pub fn owns_allocation''',1))

(OUT/'candidate.patch').write_text(''.join(patches),encoding='utf-8',newline='\n')
(OUT/'manifest.json').write_text(json.dumps(manifest,indent=2),encoding='utf-8')
print(json.dumps({'candidates':len(manifest),'patch':str(OUT/'candidate.patch')}))
