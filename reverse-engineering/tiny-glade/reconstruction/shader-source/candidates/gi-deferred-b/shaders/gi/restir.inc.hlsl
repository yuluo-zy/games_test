// ==RECONSTRUCTED== shaders/gi/restir.inc.hlsl (candidate gi-deferred-b)
// Evidence: embedded as include in 4 bins (_gi_blur_screen_radiance,
// _gi_eval_screen_radiance, _gi_trace_gi_candidates, _gi_update_gi_radiance);
// fully inlined/folded, no OpName survives. ReSTIR helper/constant layer for
// the GI screen-radiance chain; the reservoir update/resolve arithmetic is
// provided by inc/reservoir.hlsl, so this file is kept minimal (anything here
// folds to the same literals). Status: U (body not recoverable).

#ifndef TINYGLADE_GI_RESTIR_INC_HLSL
#define TINYGLADE_GI_RESTIR_INC_HLSL

// ReSTIR spatial-resolve candidate count and screen-radiance reservoir bits.
static const uint RESTIR_SPATIAL_CANDIDATES = 8u;
static const uint RESTIR_GI_CANDIDATE_COUNT = 1u;

#endif // TINYGLADE_GI_RESTIR_INC_HLSL
