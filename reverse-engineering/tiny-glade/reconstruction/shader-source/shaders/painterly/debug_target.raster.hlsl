// ==RECONSTRUCTED== shaders/painterly/debug_target.raster.hlsl | evidence: _painterly_debug_target.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin — vs_main = fullscreen triangle (shared fullscreen_triangle_vert.inc.hlsl); ps_main = pure discard (OpKill, no binds, no $Globals); both variants identical | status: O (vs triangle + ps discard from inlined SPIR-V)
//
// Painterly debug-target pass: draws nothing (fragment is discarded). The
// vertex pass is the standard fullscreen triangle.

#include "inc/common.hlsl"            // samplers (sampler_lnc s0)
#include "inc/tracked_buffer.hlsl"
#include "inc/uv.hlsl"
#include "fullscreen_triangle_vert.inc.hlsl"

float4 ps_main() : SV_Target0
{
    discard;
    return float4(0.0f, 0.0f, 0.0f, 1.0f);
}
