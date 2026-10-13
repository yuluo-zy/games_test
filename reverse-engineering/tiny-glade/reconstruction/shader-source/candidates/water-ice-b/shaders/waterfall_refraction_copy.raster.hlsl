// ==RECONSTRUCTED== shaders/waterfall_refraction_copy.raster.hlsl
// Evidence: _waterfall_refraction_copy.raster.hlsl_b903e43ffb3da915.bin (SPIR-V, offsets 23/35386).
//   VS: $Globals b13 (current_view_index, refraction_source_tex_dims), instance_buffer t14
//   (WaterfallMesh), frame_constants set1/b0; procedural waterfall mesh from water/waterfall_vert.inc.hlsl.
//   PS: refraction_source_tex t15, SV_Position in, SV_Target0 out (full-res copy of the source image).
// Status: O (algorithm + bindings from SPIR-V).
// NOTE: the f0ad PS is an empty stub (no bindings); single source cannot reproduce both — b903 is the
// reconstruction target (recorded residual).

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/consts.hlsl"
#include "inc/math.hlsl"
#include "inc/utils.hlsl"
#include "inc/hash.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/perlin.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint current_view_index;
    uint2 refraction_source_tex_dims;
};

#include "water/waterfall_vert.inc.hlsl"

Texture2D<float4> refraction_source_tex : register(t15, space0);

float4 ps_main(float4 gl_FragCoord : SV_Position) : SV_Target0
{
    return float4(refraction_source_tex.Load(int3(uint2(gl_FragCoord.xy), 0)).xyz, 1.0f);
}
