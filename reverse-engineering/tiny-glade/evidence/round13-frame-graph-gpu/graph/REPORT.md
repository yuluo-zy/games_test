# Round 13 graph reconstruction

B only, T13 capture/state producer excluded. These are behavior reconstruction candidates from original release evidence, not recovered author text. This worker writes evidence candidates only and starts no Cargo command. Parent owns three-way production integration, compiler checks and independent GPU/runtime acceptance. Existing rhapsody implementation and architecture are preserved.

## Accepted stage 2

`graph-stage2.patch` SHA `e3aa50aa9ccf9740bff106ba06fe8ebce4700b497393eff984d34bbc5ab0a013` restores the original compile_main dynamic six-step before-world selector, Primary RT/idle/full-world branches, depth/CoC dummy construction, screenshot-purpose-driven grading/logo/cursor, actual painterly depth/CoC and DebugGui/FrameConstants inputs, disabled effect state paths, MSAA target sample metadata, conditional MEI shadows and the loading/egui order outside reveal UI gating. The no-request screenshot selection is native purpose 7, not Keep. Initial grading configuration is selected only for flattening purposes 1/5; final_post independently consumes current configuration.

Independent bounded review: `../review/STAGE2_REVIEW.md`. `frozen-stage2/candidate/` was reconstructed from the accepted patch and every file SHA matched the original eight-entry `candidate-sha256.json`. The root reported integrating this patch. Original phase 1 remains historical and is not a patch to apply.

## Stage 3 incremental patch

`graph-stage3.patch` SHA `dacccf147a3bd142da301c865966a68c6cd56a6e284f6eac694bc4bfeef2d674` is relative to accepted stage 2. Its raster_host base specifically includes bindings worker `attachments-v2`, SHA `9b43123150034fd90e431ca4fd48a0cd93a0b0e0fd5e4180c45378bf578e4932`; integrate that parent before this raster hunk. Complete input/output hashes are `stage3-base-sha256.json` and `stage3-candidate-sha256.json`. `frozen-stage3/candidate/` retains the frozen files.

The original idle branch creates a raw main color without a writer and still runs blur. It does not preserve a main-color backing from a preceding frame. The original final_post shader samples the image but selects constant 0.18 when is_input_valid is zero. Likewise reverse blur samples its own highest mip before selecting the already-produced tail for sample_self=false. These reads must remain actual read/synchronization dependencies. The current graph already supports `add_undefined_content_read_range`; this patch forwards precise caller evidence through compute and raster binding metadata, without changing rhapsody, omitting passes, adding clears, manufacturing initialized contents or weakening ordinary aliases. Only idle blur input, idle final_post input, reverse highest self-input and the native unwritten depth/CoC auxiliaries use that policy.

Blur input and final_post main input expose base_mip_level=0/level_count=1 through existing view APIs. Full original SPIR-V module scanning (`shader-input-lod-proof.json`, reproducible via `inspect_shader_lod.py`) found blur's sole ImageFetch Lod constant 0, reverse blur's ImageFetch and explicit sample Lod constant 0, final_post b903's 39 explicit Lod-zero samples plus one Gather, and f0ad's one Gather. No source-image query, implicit-LOD, Grad or MinLod operation exists. The [Khronos texel gathering specification](https://docs.vulkan.org/spec/latest/chapters/textures.html#textures-gather) defines gathers over the image view's base level. No shader bytes are changed. This preserves base texels and dimensions while avoiding an incorrect six-mip initialization requirement when only main-color mip 0 was written.

Fresh B data additionally proves multires byte 0x14362c080 starts at 1 and calc_dof@0x140a5a5b4 reads it directly, contrary to the previous zero/pointer comment. The recovered default is now true. Pointer 0x14362c098 targets the same demo byte 0x143634d90 read by Vermeer alternate background selection, so the existing real GameConfig.dev.demo source now drives that selection too.

## Evidence and validation boundary

Primary original compile body is [0x140161cd0,0x1401640c4), 9204 bytes, as independently recovered from original PE unwind data. Earlier 10961-byte requests cross its primary body and are not an ABI/boundary authority. `../review/original-full.asm` and `../review/REVIEW.md` hold the bounded caller review. `original-evidence.json`, `resource-typeid-registration-proof.json`, the bounded entry/default/control assembly and direct source hashes preserve this worker's additional evidence. `extract_evidence.py` is only an evidence exporter.

All 13 stage 3 candidate Rust files passed rustfmt syntax parsing with `skip_children=true` and stdout-only output. Candidate sources were not broadly formatted. No Cargo check, test, Clippy, graph compile, GPU submit, frame readback or visual acceptance is claimed by this worker. Added regression cases cover idle/forced menu selection, conditional lookup boundaries, explicit undefined-read scope and ordinary alias tightening; parent must run them against integrated sources. Existing root tests and static proof cannot substitute for integrated execution.

## Work still in progress

- ICC is not permanently replaced with None. Independent review identified final_post's canonical ActiveIccProfile owner, and the ICC producer worker is restoring that module/path/upload/replacement/failure chain. This graph worker will consume the canonical `.texture()` API when delivered.
- Screenshot opportunities depend on screenshot worker's unified request/GPU owner/completion implementation. Same-frame pending captures must remain visible to downstream menu/gallery consumers before successful submit without leaking canceled owners.
- The restored Primary RT checkbox leaf has its original process static, but its enclosing do_gui_debug body is still unrecovered. Do not claim its full interactive producer is closed.
- MetaCommander.is_profiling is the correct selector getter; its actual profiler sink producer/finish lifecycle is being recovered by the independent worker. Current getter tests do not prove the true branch is player/runtime reachable.
- Dynamic writers for multires, occlusion freeze and temporal-history-disable are not yet closed. The multires initial byte is positive evidence, not a claimed completed debug UI.
- First/repeated gameplay, effect invalidation, cancellation/OOM retry and full GPU/WSI acceptance remain root-owned validations. This report does not mark three flows or the whole renderer complete.
