// ==RECONSTRUCTED== shaders/texture_test.raster.hlsl | F-D
// Evidence: _texture_test.raster.hlsl_b903e43ffb3da915.bin (vs_main + ps_main).
//   bindings: sampler_default s9, $Globals b13 (current_view_index/model_to_world/
//   test_texture_dims/stroke_max_height), test_texture t14, frame_constants set1/b0.
//   spirv-cross: vs displaces the quad's y by the test_texture sample at
//   (vertex.xz*1/130 + 0.5) times stroke_max_height, then world_to_sample; ps samples
//   test_texture at the interpolated UV. test_texture_dims is read in vs (reserved).
// Status: O (structure from disassembly).

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/math_const.hlsl"
#include "inc/consts.hlsl"
#include "inc/utils.hlsl"
#include "inc/hash.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint current_view_index;
    float4x4 model_to_world;
    uint2 test_texture_dims;
    float stroke_max_height;
};

Texture2D<float4> test_texture : register(t14, space0);

struct VSIn
{
    float3 position : TEXCOORD0;
    float3 color : TEXCOORD1;
};

struct VSOut
{
    float3 c0 : TEXCOORD0;
    float2 c1 : TEXCOORD1;
    float4 position : SV_Position;
};

VSOut vs_main(VSIn input)
{
    VSOut output;
    float2 uv = (input.position.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
    output.position = mul(view_constants[current_view_index].world_to_sample, mul(model_to_world, float4(input.position.x, test_texture.SampleLevel(sampler_default, uv, 0.0f).x * stroke_max_height, input.position.z, 1.0f)));
    output.c0 = input.color;
    output.c1 = uv;
    return output;
}

float4 ps_main(VSOut input) : SV_Target0
{
    return test_texture.Sample(sampler_default, input.c1);
}
