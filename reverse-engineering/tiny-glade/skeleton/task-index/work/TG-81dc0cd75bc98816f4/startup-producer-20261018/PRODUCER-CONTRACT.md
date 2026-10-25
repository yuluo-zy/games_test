# Complete SSBO startup producer candidate

Status: full72 source candidate, not production-installed or compiler/runtime accepted. No substitute types, partial registered list, skipped missing entries or fabricated cursor pointer. The source becomes callable after the explicitly listed canonical factory prerequisites are merged by their owners. The current production tree cannot safely execute the complete original registration as-is.

Original full body140909e80 is2540B. Three original inputs are Res<GraphicsDeviceResource>,Res<AssetSsboLibrary>,EventWriter<CustomCursorSsboCreatedCmd>. The new startup module maps the original resources-level function into the current architecture without recreating rhapsody. Its non-ECS helper uses exactly the same library/device and returns the actual mapped event only after entry72.

## Original order and data

Entries1..68 call each canonical add_new, preserving each concrete None policy. Nani fixed4 remain at62..65, then BushCpu/RippleSource/Sparkle66..68. Entries69..71 are the actual Nani draw-list zero20B records[total_max_instances], original second params Vec108u32 offsets, and zero u32 indirection[total_cmd_offset]. Entry72 reuses the existing actual CustomCursor factory+mapped alignment/length checks. It is not a separate earlier CustomCursor registration or a second Nani subset call. registration-order.json binds every native site and current type.

## Integration placement

Export startup::init_populate_ssbo_library and place its ECS system in original PreStartup inner ten-system tuple. Preserve outer create_preset_textures → inner tuple → load_mesh_atlases → startup_nani chain. The inner tuple itself is not proven chained. Do not install the incomplete producer first, auto-fill unknown types or suppress registration panics. The same GraphicsDeviceResource already carries the actual BufferHost; its backing host and library must outlive the mapped cursor event. Runtime adds the canonical event owner before PreStartup. Any older isolated Nani/CustomCursor registration must be removed from that same App when full producer is installed to avoid duplicate registrations.

## Precise prerequisites

### Entries 18

Types: `systems::audio::terrain_stairs_audio::StairsCounterSsbo`.

Missing canonical type; StairsCounterBuf is a foreign zero-sized pass tag.

14161ddb0: u32 x1; NAME Stairs Counter; size4/usage22/memory4/align4; None uploads4 zero bytes then maps. add_new1412c0850 stores MappedStorageBuffer56.

Define canonical u32 newtype+StorageElement+library/pass SsboType. Use real MappedStorageBuffer; pass_kit StairsCounterBuf must reexport canonical identity, not keep duplicate NAME.

### Entries 25,26

Types: `resources::walls::dormers::DormerHoleSsbo`, `resources::walls::dormers::DormerHoleCountPerCellSsbo`.

No canonical module/types; draw ssbo_tags are consumer tags, not real library storage.

Full newly extracted14161a760:48B x11250,size0x83d60,NAME Dormer Holes.1416159e0:u32 x225,size0x384,NAME Dormer Hole Count. Both usage22/mem1/align4, None no upload; native add_new wrappers allocate InstanceBuffer48 with count0.

Actual canonical elements (raw words acceptable for unobserved fields) and InstanceBuffer factories preserving count0/None. Reuse TypedStorageBuffer::from_created_buffer; do not invent semantic CPU fields or register tags.

### Entries 31,45

Types: `dof_autofocus::DofAutofocusSsbo`, `apply_screen_shadows::ShadowMaskHistogramSsbo`.

Existing factories omit original None payloads.

Dof 1416101f0: full512B repeated0x41f00000(30.0), original constant142ac7e60. Histogram141612170: full1024B zero. Mean remains no upload.

Delivered narrow3-constructor patch and family claims; see constructor-corrections/CONTRACT.md.

### Entries 35,55

Types: `compute_passes::stairs::StairsTransformSsbo`, `compute_passes::spawn_boats::BoatSsbo`.

Gap/one-element carriers still in production; old OS legal candidates already exist.

Stairs48B x41000,0x1e0780,NAME Stairs transforms,Typed40/align4. Boat64B x4096,0x40000,NAME Boats,Instance48/align16,count0. None uploads nothing.

Merge candidate-stairs.rs from TG-cdd2907758d9e3a316/frameinputs-20261018 and candidate-spawn_boats.rs from TG-26b5e861be8fcfa7dd/frameinputs-20261018 by bounded hunks, preserving external-owner current algorithms.

### Entries 39

Types: `path_pebbles::PebblePlacementSsbo`.

None incorrectly uses zero-uploading TypedStorageBuffer::new.

141610db0 None skips directly to descriptor,48B x576,0x6c00/usage22/mem1/align4, no upload.

Delivered narrow3-constructor patch; use fixed byte constructor for None/Some.

### Entries 43,44

Types: `draw_passes::ui_2d_pass::TextPrimSsbo`, `draw_passes::ui_2d_pass::ShapeStreamSsbo`.

Both Gap tags have no canonical library factory.

TextPrim141611730:48B x131072,size0x600000,NAME TextPrim. ShapeStream14160fb70:u32 x1048576,size0x400000,NAME ShapeStream. Both Typed40,usage22/mem1/align4,None no upload.

Define actual canonical element rows and library SsboType; keep existing pass consumers on the same identities. Existing tg-text/shape-stream payload types may be reused only after matching original byte schema; raw words do not imply recovered field semantics.

### Entries 47,48,49,53,56

Types: `compute_water::LilypadSsbo`, `compute_water::LilypadFlowerSsbo`, `compute_water::ReedSsbo`, `compute_garden::Flower03Ssbo`, `compute_garden::GardenStoneSsbo`.

Water3 are Gap and lack library factories; garden2 are fake one-element carriers.

Seven-constructor original pack: Lilypad64x50625/NAME Lilypads/align16; Flower64x50625/NAME Lilypad Flowers/align16; Reed48x22500/NAME Reeds/align4; Flower03 16x160000/NAME Small lowpoly flowers/align4; GardenStone64x32400/NAME Garden Stone/align16. All Instance48,count0,usage22/mem1,None no upload.

Coordinate existing water/garden owners using os-populate-review-20261018/remaining-seven-constructors/SEVEN-CONSTRUCTOR-CONTRACT.md; no parallel substitute types.

### Entries 50

Types: `resources::render_ssbo::LanternSsbo`.

Existing unique type is render_ssbo, original was render::lantern. InstanceBuffer generic None calls TypedStorageBuffer::new and adds a zero upload.

Fresh full141615bb0 confirms16B x32768,size0x80000,NAME LanternSsbo,Instance count0,usage22/mem1/align4,None no upload.

Fix this actual factory locally. Candidate imports the current unique type so consumers remain same identity. A future original-path module must reexport/move that definition, not duplicate it.

### Entries 51,52

Types: `compute_garden::BushSsbo`, `compute_garden::FlowerSsbo`.

fixed_len_typed_storage(None) calls TypedStorageBuffer::new, adding zero upload.

141610b60 Bush48B x9600,size0x70800,NAME Bushes;141613b90 Flower64B x53120,size0x33e000,NAME FlowerSsbo. Both Typed40,usage22/mem1/align4,None no upload.

Per-type factories can directly use TypedStorageBuffer::new_storage_buffer; do not change generic None policy globally.

### Entries 54

Types: `rasterize_buildings::GroundedBuildingsSsbo`.

NAME currently GroundedBuildingsSsbo disagrees with original registration key; generic Instance None additionally uploads zeros.

Fresh full141617840 confirms NAME Building Shapes,32B x5000,size0x27100,usage22/mem1/align4,None no upload. add_new1412c0350 registers Instance count0 with key1c53e91811919db1.

Correct actual type NAME and local Instance factory; preserve actual source/GPU schema, do not add another Grounded type.

### Entries 59,60,61

Types: `resources::water_mask::WaterMaskSsbo`, `resources::water_exclude_clutter_mask::WaterExcludeClutterMaskSsbo`, `resources::garden_mask::GardenMaskSsbo`.

Water is Gap and wrong NAME/capacity; exclusion/garden u32 types still capacity0x8000 elements and suffixed NAME, four times native buffer size.

14161d570 WaterMask,14161c080 WaterExcludeClutterMask,14161f660 GardenMask: u32 x8192, size0x8000/usage22/mem4/align4,None fullzero upload and actual map.

Canonical u32 types with short native NAME and BUFFER_LEN0x2000, same existing MappedStorageBuffer factory after correction. Use real pass hash from canonical NAME; Water current hash0 is invalid.

### Entries 66

Types: `read_bush_positions_for_cpu::BushCpuSsbo`.

Gap/BUFFER_LEN1/NAME BushCpuSsbo; old comment0x40000 is contradicted by full original body.

14161bd30: element8B(len&7, mapped len>>3),capacity9601,size0x12c08,NAME Bush positions for CPU,usage22/mem4/align4,None NO upload; explicit actual mapping returned.

Actual8B raw row type plus local mapped factory preserving None, checked len%8/ptr%4/len<0x12c10. Existing MappedStorageBuffer::new_storage_buffer unconditionally uploads zero defaults, so this one needs bounded manual create/map or an explicit specialized adapter; fields mapped/buffer are already public.

## Validation boundary

Only original bytes/PDB metadata, current source and rustfmt parsing were used. No Cargo/GPU, shared DB mutation or production edits. Existing constructors not called out above have not thereby received new full behavior validation. Root must merge per-family candidates and perform one complete producer invocation with real mapping and captured descriptors/data before claiming this startup node closed.
