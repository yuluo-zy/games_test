struct ViewConstants
{
    column_major float4x4 view_to_clip;
    column_major float4x4 clip_to_view;
    column_major float4x4 view_to_sample;
    column_major float4x4 sample_to_view;
    column_major float4x4 world_to_view;
    column_major float4x4 view_to_world;
    column_major float4x4 world_to_sample;
    column_major float4x4 clip_to_prev_clip;
    column_major float4x4 prev_view_to_prev_clip;
    column_major float4x4 prev_clip_to_prev_view;
    column_major float4x4 prev_world_to_prev_view;
    column_major float4x4 prev_view_to_prev_world;
    float2 sample_offset_pixels;
    float2 sample_offset_clip;
    float2 prev_sample_offset_pixels;
    float2 prev_sample_offset_clip;
};

static const uint2 _92[4] = { uint2(0u, 0u), uint2(0u, 1u), uint2(1u, 1u), uint2(1u, 0u) };

RWByteAddressBuffer frame_constants[] : register(u2, space0);
RWByteAddressBuffer _131[] : register(u2, space0);
cbuffer _132_134
{
    uint _134_m0 : packoffset(c0);
};

Texture2D<float4> depth_tex[] : register(t0, space0);
Texture2D<float4> candidate_radiance_tex[] : register(t0, space0);
Texture2D<float4> candidate_hit_tex[] : register(t0, space0);
RWTexture2D<float4> restir_radiance_out_tex[] : register(u1, space0);
RWTexture2D<float4> restir_ray_dir_out_tex[] : register(u1, space0);
RWTexture2D<uint2> restir_reservoir_out_tex[] : register(u1, space0);

static uint3 gl_GlobalInvocationID;
struct SPIRV_Cross_Input
{
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
};

uint spvPackHalf2x16(float2 value)
{
    uint2 Packed = f32tof16(value);
    return Packed.x | (Packed.y << 16);
}

float2 spvUnpackHalf2x16(uint value)
{
    return f16tof32(uint2(value & 0xffff, value >> 16));
}

void comp_main()
{
    uint _99 = frame_constants[_131[_134_m0].Load(24)].Load(128);
    restir_radiance_out_tex[_131[_134_m0].Load(12)][gl_GlobalInvocationID.xy] = candidate_radiance_tex[_131[_134_m0].Load(4)].Load(int3(gl_GlobalInvocationID.xy, 0u));
    restir_ray_dir_out_tex[_131[_134_m0].Load(16)][gl_GlobalInvocationID.xy] = float4(candidate_hit_tex[_131[_134_m0].Load(8)].Load(int3(gl_GlobalInvocationID.xy, 0u)).xyz, depth_tex[_131[_134_m0].Load(0)].Load(int3((gl_GlobalInvocationID.xy * uint2(2u, 2u)) + _92[_99 & 3u], 0u)).x);
    uint _121 = spvPackHalf2x16(float2(1.0f, 0.0f));
    restir_reservoir_out_tex[_131[_134_m0].Load(20)][gl_GlobalInvocationID.xy] = uint2(gl_GlobalInvocationID.x | (gl_GlobalInvocationID.y << 16u), _121 | (_121 << 16u));
}

[numthreads(8, 8, 1)]
void main(SPIRV_Cross_Input stage_input)
{
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    comp_main();
}
