# Populate rejection parameter: bounded original B contract V1

This recovers concrete dependencies and reader lanes for the actual populate chain. It is not a completed filter, source candidate, App registration or test. Root is the sole production/test writer; original B only.

## Actual parameter, not a fabricated resource

DecoratorRejectionRes is a derived SystemParam declared in the original populate module at line61. Full init1422e1b40/762B (including its cleanup funclets) and nested path init140ae5b20/107B are bound to PDB and direct PE. Its state initializer invokes the exact existing resource/query init procedures; the field-use index binds the eventual borrowed parameter slots to actual consumers. Native offsets are evidence of routing, not a promised Rust ABI.

| Borrowed native slot | Actual owner / access | Current source status |
| --- | --- | --- |
| +00 | Res<AssetSsboLibrary> | canonical library exists |
| +20/+40/+60/+80 | nested ResMut<PathRaster>, ResMut<Cobblestone1Raster>, ResMut<Cobblestone2Raster>, ResMut<Cobblestone3Raster> | canonical named types exist; original nested4-param aggregate may be represented without a second resource |
| +a0 | Res<WaterRaster> | canonical type exists |
| +c0 | Res<GardenRaster> | canonical type exists |
| +e0 | Res<BushGrid> | same system-clutter owner, actual typed mapped-readback writer now exists |
| +100 | Res<WallSpatialHash> | canonical actual segment owner exists |
| +120 | Res<PublicWalls> | canonical publication owner exists |
| +140/+148/+150 | Query<&country_core::components::BalconyDoor> | canonical component missing; actual producer currently inserts a local raw24 component |
| +158/+160/+168 | Query<&country_core::components::WoodenGate> | canonical component missing; actual producer currently inserts WoodenGateState plus separate render View |
| +170 | Res<RaycastWorld> | same canonical physics owner exists |

The seven Res initializers have exact concrete generic type names in the retained procs table index. Four nested path init calls are mutable SystemParams (impl12), not four aliases of WaterRaster. Queries initialize BalconyDoor then WoodenGate (each QueryState0x118) and are copied into one state; they are not synthetic input position Vecs.

The common filter unconditionally obtains the actual TerrainRockMaskSsbo from AssetSsboLibrary around1422d909c..9116, checks its complete63-byte type string, and obtains its typed mapped view. The retained literal matches country_core::resources::terrain_rock_mask::TerrainRockMaskSsbo. Current canonical type exposes the real mapped storage and original bit sampler; no RockMaskData shadow, clear mask or implicit missing-buffer success is authorized.

## BalconyDoor: current real writer and exact original reader

Original producer is the already retained full VisualEntityBuilder::build1412a9b30/5949B, supplied by04. Native condition self+14f is tested at1412aa71d..724 and1412aaa78..a7f. Rotation*Vec3::Z is evaluated1412aaa85..aab19 and the actual insert closure's24B component payload is written1412aab3a..aab74. Current producer at visual_entity_builder.rs stores translation xyz then forward=(rotation*Z) xyz into its pending_core::BalconyDoor([u8;24]); that is a real current nonempty producer, but it has a different TypeId from a future core query.

Complete retained populate's exact query reader1422da218..a57b is preserved as162 continuous instructions. At da3ee it reads forward.z@14; da407 forward.xy@c; da402 position.z@8; da431 position.xy@0. It constructs center=position+forward using original doubling then0.5 sequence (not an algebraic reassociation promise), dims=(1.5,2,2), and axis=(-forward.z,forward.x)/sqrt(fx^2+fz^2); no normalize_or_zero/NaN sanitizer is inserted. The actual Rectangle3d constructor is called da4a9, then the candidate overlap dependency da56d. This independently supports the semantic24B value {translation:Vec3,forward:Vec3}, not a Default or empty marker.

Root has asked04 to freeze the complete writer/component contract. Only root will choose the one drawable candidate author after07 admission. A canonical type alone does not repair the chain: the same producer must import/insert it and the actual query must use that TypeId.04 remains visual_entity_builder's unique writer; no other group writes that production file or overwrites its frozen candidate.

## WoodenGate: real current values exist, but no canonical query identity yet

Current actual construct_gates consumes real leaf outcomes and spawns DrawableMeshBundle/TransientMesh with WoodenGateState {owned top_positions,wall,endpoint_directions,seed,clutter_version}; separately it inserts WoodenGateView {q18,d30} for the actual draw callback. The core construction result already carries those actual leaf values. This is not evidence that clutter's independent GateHost mirror has a writer or the same component identity. A correct canonical migration must preserve the owned vector and actual draw consumer, not create an empty marker or a third active owner.

Original ComponentDescriptor::drop_ptr<WoodenGate>14014b850/32B confirms the owned vector prefix: cap at0, pointer8, allocation12*cap with alignment4. Original gates-region full2910B and the current common rejection reader cover the native56B component. Its full writer/Clone/required-component lifecycle needs its own bounded contract before a canonical migration. The existing two-field Copy render View cannot simply alias a new Vec-owning type while keeping '*comp' copies; the actual draw callback needs a reviewed borrow-based adaptation. No such foreign source is claimed or changed in this increment.

## Actual remaining producer/filter work

The full common filter after object preparation around1422d9008 contains real water/garden/rock sampling, optional bushes, wall heights/curves/spatial segments, BalconyDoor and WoodenGate query geometry, and shape_overlaps_with on the same physics world. The retained direct-call index gives exact business helper PCs. A dry raster alone is not success through this filter, and missing canonical query identities cannot be assumed empty to bypass it.

The fixed Olden placement modes2/3/4 are genuine finite original table entries, but their original nonempty upstream core PreplacedAutoClutter source file is missing. The current local preplaced Vec has no actual writer. No test injection or default Vec is counted as a live producer. Weighted modes0/1 and source4 retain additional placement/ownership work. The next candidate must recover a real route plus these admitted gates, not attach metadata to the old nearest-cell/Halloween/raycast-seam approximation.

No source has been changed here. Hash/PE/PDB reads are static evidence checks, not CPU gameplay tests; no Cargo, CPU test, GPU or original target execution occurred. ActualApp owner registration/order remains outside this dependency contract.
