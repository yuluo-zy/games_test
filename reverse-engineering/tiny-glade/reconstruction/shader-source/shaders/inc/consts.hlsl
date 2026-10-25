// ==RECONSTRUCTED== shaders/inc/consts.hlsl | evidence: embedded in 250 bins; no named constants recoverable (constants fold to literals; only TEXTURE_SIZE_WS appears as a named define, in the water family, not here); no inlined code attributed to this file | status: U (member names/values Inference — cannot be recovered from SPIR-V; kept minimal so consumers that need specific game constants carry them locally)
//
// Shared game constants. Content is provisional: named constants are not
// recoverable from compiled SPIR-V (they fold into literals). Keep additions
// here only when a consumer family confirms a shared constant.

#ifndef TINYGLADE_INC_CONSTS_HLSL
#define TINYGLADE_INC_CONSTS_HLSL

// Render-scale / half-res factor used across multiple passes.
static const float RENDER_SCALE = 1.0f;

#endif // TINYGLADE_INC_CONSTS_HLSL
