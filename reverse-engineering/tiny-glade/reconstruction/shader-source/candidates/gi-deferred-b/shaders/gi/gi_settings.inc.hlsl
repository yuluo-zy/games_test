// ==RECONSTRUCTED== shaders/gi/gi_settings.inc.hlsl (candidate gi-deferred-b)
// Evidence: embedded as include in 170 bins; fully inlined/folded, no OpName
// survives. All values collapse into literals at use sites, so named constants
// are not recoverable. Kept as a minimal settings header so consumers embed
// the file for OpSource parity; the constants below are Inference and only
// used where a consumer's literal matches (none referenced by name today).
// Status: U (values/named constants cannot be recovered from compiled SPIR-V).

#ifndef TINYGLADE_GI_SETTINGS_INC_HLSL
#define TINYGLADE_GI_SETTINGS_INC_HLSL

// GI / screen-radiance settings. Values are provisional; individual passes
// carry their own literals where the compiled code requires them.
static const uint GI_TRACE_RAY_COUNT = 1u;
static const float GI_TRACE_RAY_LENGTH = 30.0f;
static const float GI_BLUR_RADIUS = 3.0f;

#endif // TINYGLADE_GI_SETTINGS_INC_HLSL
