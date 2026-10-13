// ==RECONSTRUCTED== shaders/gi/gi_settings.inc.hlsl | status: O (hi_px_subpixels table from
// _gi_* consumer debug names); I (settings constants)
// MERGE (gi-deferred review 2026-10-13): content adopted from candidate A (hi_px_subpixels
// pinned by the eval/blur/update bins; reproject/trace don't embed it -> unused static is
// dropped, so defining it in the shared inc matches the original). Consumers include this
// file instead of re-declaring the table locally.
// Evidence: _gi_eval/_blur/_reproject/_update bins carry a static `hi_px_subpixels`
// (uint2[4] half-res jitter offsets {0,0}/{0,1}/{1,1}/{1,0}) used to pick the matching
// full-res texel when sampling the half-res ssgi/depth buffers; include-only file,
// embedded in 170 bins (all lighting/vegetation/deferred passes). No $Globals is
// declared here (each consumer declares its own cbuffer with only the members it uses).
// include-graph lists fbm_noise/plant_material as possible outgoing edges but no target
// consumer closure contains them, so none are included (matches observed closures).

#ifndef TINYGLADE_GI_SETTINGS_INC_HLSL
#define TINYGLADE_GI_SETTINGS_INC_HLSL

// Half-resolution sub-pixel offsets (per-frame jitter phase). Selecting
// `hi_px_subpixels[frame_index & 3]` offsets a full-res texel load into the matching
// half-res sample, giving camera-jitter-consistent half-res reads.
static const uint2 hi_px_subpixels[4] = {
    uint2(0u, 0u),
    uint2(0u, 1u),
    uint2(1u, 1u),
    uint2(1u, 0u),
};

// GI screen-space settings. Values are Inference (inlined in consumers; only the
// subpixel table is pinned by debug names).
static const float GI_MIN_HALF_NORMAL_Z = 0.3f;          // max(0.3, normal.z) weight floor
static const float GI_DEPTH_WEIGHT_SCALE = 80.0f;        // exp2(-min(30, 80*depth_err))
static const float GI_DEPTH_WEIGHT_MAX = 30.0f;
static const float GI_NORMAL_DOT_DAMP = -0.9f;           // dot^2 * -0.9 + 1
static const float GI_FINAL_WEIGHT_NORM = 0.01f;         // 0.01 / (dot^2*-0.9+1)^2

#endif // TINYGLADE_GI_SETTINGS_INC_HLSL
