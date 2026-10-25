# Original B W7 StairsTopDownMesh producer/consumer contract

Legal W7 draw family claim TG-b4f30a32cb343cc349; work-only B evidence. Do not modify the broad foreign core resources owner or decorator implementation from this family. Original RNE identity and complete PDB-bounded body SHAs are in native-manifest.json. No Cargo/GPU was run here.

The actual optional resource is `country_core::resources::stairs::stairs_top_down::StairsTopDownMesh`, proven TypeId lo1e9ac9b6648cf0a6/hi e6222c4af0fcfd90 in original W7, insert-command apply and remove-command. It is **not** the existing invented StairsFoliageExclusion view and must have one canonical identity used by decorator producer and W7 consumer.

Producer exists in `system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>` @140008c90/62374B and `<false>` @140018040/62822B. Both complete original bodies are retained. Two direct insert calls are instruction-validated in stairs-resource-xrefs.json: true @140010d8c and false @14002024c → Commands::insert_resource @14014dd30. Native generic symbol `closure_env$26<false>` shared by both collect bodies is an ICF alias, not evidence that true skips this branch.

At true @140010cb6 (false @140020176), native `Vec<Vec4>::from_iter<FlatMap<HashMap::Values<StairGraphEdge,StairsEdgeVisualState>,Copied<slice::Iter<Vec4>>,closure26>>` flattens the actual per-edge vectors. The complete 1165B @1402bcd50 is retained: bucket stride0x110, per-bucket Vec4 pointer read relative bucket-end -0xe8 and len -0xe0, item step16; **do not** project zeros from current opaque StairsEdgeVisual or assume its key/value field ABI. Those per-edge geometry vectors require actual original assembly generation.

The flattened Vec length is r14 @140010cc9; count0 branches @140010cd3→140010dfb to enqueue `Commands::remove_resource<StairsTopDownMesh>` via function pointer14014d5c0 (@140010e20). No zero-sized buffer or Some(empty) resource is inserted. The false equivalent is @140020193→1400202bb.

For nonzero count:
- byte length = count << 4 (@140010ce8/ceb), BufferDesc::new_gpu_only(bytes, usage0x20) @140010d02;
- create buffer with actual full Vec byte content, name exactly `top-down stairs` (15 bytes; VA1429264f0, bytes retained in producer-constants.json) @140010d34;
- unwrap successful original Result and move the actual 40-byte Buffer into the resource @140010d46..d6c; low u32 count @resource+0x28 written by @140010d74; Commands::insert_resource @d8c copies all48B including native padding (wrapper @14014dd53..dd5e). This is a typed ownership observation, not a reason to recreate original raw layouts in refactored Rhapsody.
- native error branch @140010e35 drops Result; no insertion of a fake successful owner is permitted. Remaining exact error reporting may be recovered separately from surrounding original flow.

Required current canonical interface for the foreign owner: Resource `StairsTopDownMesh { buffer: <existing canonical GPU Buffer owner identity>, vertex_count:u32 }` or equivalent typed accessors `buffer()` / `vertex_count()`. Actual BufferHost owner/lease must be retained through graph/command execution; raw integer or pointer alone is not a new owner. No Default is justified; absence is a real, legal no-stairs state. Producer should flatten **actual generated** per-edge Vec4s, remove resource if empty, otherwise upload and insert this same resource, rather than installing it during application Startup with zero content.

W7 @1409cb7b0 actual consumer:
1. Resource absent and shared global143634cf8 false → no pass.
2. Resource absent and globaltrue → render clear-only Foliage exclusion pass to preset16 → globalfalse.
3. Resource present → original shader `stairs_foliage_exclusion`, bind actual buffer under92930b0d61fef16e, preset9/10 sampled textures under61b08b5e0470c5f2 and297d1e4cdae8f657, draw_no_vao(actual count lowu32 @+28), render → globaltrue.
The global initialfalse is now proven: .data section offset40184 >= RawSize39936 and <VirtualSize66344 (zero-filled PE virtual tail). Do not label it unknown. Last-writer clear/draw state must reflect admission success in the current error-returning graph adapter, analogously to W3 adapter's existing rollback; a failed frame cannot be submitted partially.

Current genuine blockers: core resources/stairs/visual.rs StairsEdgeVisual still stores only sparse allocation slots plus `_opaque`; it has no generated Vec4 vector. systems/decorator/src/decorator_visual/stairs_assemble.rs's playermade_stairs_assemble remains a no-argument empty function. Adding only canonical StairsTopDownMesh is insufficient to close those actual producer algorithms. Root should forward this typed interface and exact original producer evidence to their authorized owners. The absent-resource W7 branch is safe only as its original branch, not evidence of scene completion.
