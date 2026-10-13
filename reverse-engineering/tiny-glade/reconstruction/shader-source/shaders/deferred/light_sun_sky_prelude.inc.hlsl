// ==RECONSTRUCTED== shaders/deferred/light_sun_sky_prelude.inc.hlsl | status: O (include edge); I (any extra helpers — inlined away)
// MERGE (gi-deferred review 2026-10-13): adopted candidate B (include-umbrella shape matches
// the target: prelude included early, right after common, pulling frame_constants /
// halfres / fullscreen_triangle_vert).
// Evidence: embedded as include in 8 bins (deferred_{canopy,grass,plant,solid}_light_sun_sky);
// direct include edge light_sun_sky_prelude.inc.hlsl -> inc/frame_constants.hlsl. Shared
// prelude for the deferred sun+sky lighting passes: pulls the frame constants / common
// umbrella and declares the fullscreen-triangle VS entry. The per-pass bindings and the
// lighting body live in the raster source / light_sun_sky.inc.hlsl.
// Status: O (include edge); I (any extra helpers — inlined away).

#ifndef TINYGLADE_DEFERRED_LIGHT_SUN_SKY_PRELUDE_INC_HLSL
#define TINYGLADE_DEFERRED_LIGHT_SUN_SKY_PRELUDE_INC_HLSL

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/halfres.hlsl"
#include "inc/pack_unpack.hlsl"
#include "fullscreen_triangle_vert.inc.hlsl"

#endif // TINYGLADE_DEFERRED_LIGHT_SUN_SKY_PRELUDE_INC_HLSL
