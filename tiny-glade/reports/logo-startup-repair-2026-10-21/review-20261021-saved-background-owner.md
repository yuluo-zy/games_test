# Saved loading-background owner / handoff independent source review

2026-10-21. Read-only; no Cargo/GPU. Current source reviewed after owned-background fix; final real saved-account run remains central validation.

## Result

No must-fix P1/P2 found in this owner/import/handoff change. It correctly replaces a cross-App frame-local logical reference with a retained native texture owner plus a fresh import/reference in every graph. No rhapsody algorithm changes are required.

## Ownership and synchronization proof from current source

- loading_graph.rs48 stores `SavedLoadingBackgroundTexture(Arc<rhapsody::resource::Texture>)`, not TextureHost slot or logical ID. build_loading~295 creates/uploads actual RGBA through current app TextureHost, takes its actual managed `.shared()` Arc, then releases only temporary host slot. TextureHost::release(render_host.rs415) clears registry handle/views and increments slot generation; it does not invalidate retained Arc/native resource state. No other consumer keeps that released handle.
- import_saved_background~50–59 clones this Arc and calls CoreGraphFrame::import_native_texture_owned(texture,0x22), deriving dimensions/format from actual texture. Compatibility desc is appropriate for this single-mip/single-layer/single-sample uploaded image; actual rhapsody import still uses full native descriptor.
- render_bridge.rs803→rhapsody rg/vulkan.rs2242/2250 calls import_frame_resource(owned=true). It checks exact device identity, access range, real sharing/descriptor, reuses only the same physical Arc declaration, allocates a frame import lease and backend token bound to the actual resource, and attaches graph on_retire marker. No pointer/graph-ID guessing and no fictitious completion receipt.
- raw/device_api.rs142→setup.rs464/469 performs real setup upload. Setup upload is asynchronous, not falsely marked completed: transfer/texture.rs244–263 records state-derived transfer destination and final shader-read layout; same texture's last-use state owns upload completion. rg/vulkan.rs1905–1950 collects/locks real use cells, prepares states, adds staged timeline waits, and publishes true submit point after successful submission. Moving the same Arc across app/session does not erase this dependency; no extra ad-hoc SyncPoint field or idle wait is needed for this saved texture.
- validate_saved_background_handoff~74 rejects `VermeerState.background=Some` without its owner, wrong actual device for either App, or conflicting destination owner **before six runtime-owner moves**. transfer_loading_resources~1556 then moves this same owner alongside incoming VermeerState; no default shader/device fallback added.
- world_postprocess.rs618–621 reimports only when original state.background isSome; refresh_saved_background_reference replaces obsolete graph-local TexRef with this graph's actual imported index. EffectsHost.register_texture~660 registers that same index, making texture()/bindings resolve locally. Loading's build does the same every frame. This prevents stale prior-frame/loading-App logical-index use.
- Natural reveal completion clears original state.background. world_postprocess.rs692–695 removes World owner only after successful declaration. The built graph's imported backend resource/lease still owns its Arc through cancellation/submission/retirement, so clearing the World resource before GPU completion does not destroy an in-use image. Error paths retain the World owner and report missing owner explicitly instead of erasing background.

## Pending input / resize review

- Established pending mode still collects true worker completion/errors first; delays only Logo release/handoff, dispatches captured OS batch, and polls background without re-running update/egui/reveal.
- update_before_render latch preserves the one already-run update; input_consumed_this_call clears only those custom key/mouse edges that an actual update consumed. New edges/events while pending remain for the following real update. Completed/frame/temporal counters only advance after actual render success.
- Pending close is handled immediately; fixed absolute16ms poll deadline cannot be postponed indefinitely by repeated input. Terminal lifecycle guard remains above these paths.
- Resize updates current WindowSize/config/frame extents and requests real renderer resize; zero extent defers graph use, with pending CPU image still safe. Saved background is an independent source image whose dimensions remain original, reimported anew, not treated as a window-sized render target.
- One observable adaptation boundary remains: if resize occurs after the first update is latched but before the first loading render, that frame can contain the previous update's MEI/layout snapshot rendered into a current-size target; the next actual update rebuilds layout. No native resource/stale-ID/lifetime fault is indicated, but this read-only review does not certify visual layout continuity for resize-during-pending. Do not rerun full App.update to fix it at the cost of duplicated Logo/input advancement.

## Related Logo alpha fix checked

Read supplied original asm: ui-pass @140837ee2/ee9/eeb/eed and loading-ui-pass @140838292/299/29b/29d apply `(byte3 &0x9f)|0x40` after bind_common and before callback. Current ui_pass.rs ListPass ItemMode::Ui and d_main_support.rs279 call exactly RasterState::set_byte3_field_2. Both passes share this corrected original state operation. No host shader/rhapsody fallback or arbitrary alpha fill added. Actual pixel result is central/GPU evidence, not newly executed by this reviewer.
