#version 450
#extension GL_EXT_nonuniform_qualifier : require
#extension GL_EXT_scalar_block_layout : require
#extension GL_EXT_samplerless_texture_functions : require
#if defined(GL_EXT_control_flow_attributes)
#extension GL_EXT_control_flow_attributes : require
#define SPIRV_CROSS_FLATTEN [[flatten]]
#define SPIRV_CROSS_BRANCH [[dont_flatten]]
#define SPIRV_CROSS_UNROLL [[unroll]]
#define SPIRV_CROSS_LOOP [[dont_unroll]]
#else
#define SPIRV_CROSS_FLATTEN
#define SPIRV_CROSS_BRANCH
#define SPIRV_CROSS_UNROLL
#define SPIRV_CROSS_LOOP
#endif
#extension GL_EXT_spirv_intrinsics : require
layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

struct ViewConstants
{
    mat4 view_to_clip;
    mat4 clip_to_view;
    mat4 view_to_sample;
    mat4 sample_to_view;
    mat4 world_to_view;
    mat4 view_to_world;
    mat4 world_to_sample;
    mat4 clip_to_prev_clip;
    mat4 prev_view_to_prev_clip;
    mat4 prev_clip_to_prev_view;
    mat4 prev_world_to_prev_view;
    mat4 prev_view_to_prev_world;
    vec2 sample_offset_pixels;
    vec2 sample_offset_clip;
    vec2 prev_sample_offset_pixels;
    vec2 prev_sample_offset_clip;
};

struct PackedBlBvhNode
{
    uvec4 data[5];
};

struct PackedTriangle
{
    float v[3];
    uint e[3];
};

vec3 _190;
vec3 _210;
vec4 _211;

layout(set = 0, binding = 13, scalar) uniform type_Globals
{
    layout(offset = 0) uvec2 terrain_lighting_texture_dims;
    layout(offset = 8) uvec2 blue_noise_texture_dims;
    layout(offset = 16) uvec2 terrain_heightmap_dims;
    layout(offset = 24) uvec2 visual_terrain_heightmap_dims;
    layout(offset = 32) uvec2 brdf_fg_lut_dims;
    layout(offset = 40) uint rt_bvh_node_count;
    layout(offset = 44) uint rt_triangle_count;
    layout(offset = 48) uint history_valid;
    layout(offset = 52) uvec2 half_depth_tex_dims;
    layout(offset = 64) uvec2 taa_history_tex_dims;
    layout(offset = 72) uvec2 reprojection_tex_dims;
    layout(offset = 80) uvec2 refl_hit_tex_dims;
    layout(offset = 88) uvec2 refl_hit_radiance_tex_dims;
} _Globals;

layout(set = 1, binding = 0, std140) uniform type_ConstantBuffer_FrameConstants
{
    vec4 sun_color;
    vec4 sky_low_color;
    vec4 sky_high_color;
    vec4 fog_tint_thickness;
    vec4 fake_light_color;
    vec4 fake_light_dir;
    vec4 fake_light2_color;
    vec4 fake_light2_dir;
    uint frame_index;
    float delta_time_seconds;
    float ev_mult;
    uint glade_theme_id;
    uvec2 inner_rendering_resolution;
    uint game_time_sec;
    float game_time_frac;
    vec3 editing_highlight_color;
    float time_of_day;
    vec3 dir_to_sun;
    uint camera_idx;
    vec3 dir_to_moon;
    uint render_overrides;
    float global_mip_bias;
    uint highlighted_transient_stair_edge_id;
    uint pad1;
    uint pad2;
    vec4 debug_tweakers;
    layout(row_major) ViewConstants view_constants[5];
} frame_constants;

layout(set = 0, binding = 23, std430) readonly buffer type_StructuredBuffer_PackedBlBvhNode
{
    PackedBlBvhNode _m0[];
} rt_bl_bvh;

layout(set = 0, binding = 24, std430) readonly buffer type_StructuredBuffer_PackedTriangle
{
    PackedTriangle _m0[];
} rt_triangles;

layout(set = 0, binding = 25, std430) readonly buffer type_StructuredBuffer_v4uint
{
    uvec4 _m0[];
} ice_rt_count_buffer;

layout(set = 0, binding = 26, std430) readonly buffer ice_rt_ray_buffer
{
    uvec4 _m0[];
} ice_rt_ray_buffer_1;

layout(set = 0, binding = 0) uniform sampler sampler_lnc;
layout(set = 0, binding = 2) uniform sampler sampler_nnc;
layout(set = 0, binding = 9) uniform sampler sampler_default;
layout(set = 0, binding = 15) uniform texture2D blue_noise_texture;
layout(set = 0, binding = 18) uniform texture2D terrain_heightmap;
layout(set = 0, binding = 19) uniform texture2D visual_terrain_heightmap;
layout(set = 0, binding = 27) uniform texture2D half_depth_tex;
layout(set = 0, binding = 28) uniform texture2D taa_history_tex;
layout(set = 0, binding = 29) uniform texture2D reprojection_tex;
layout(set = 0, binding = 30, rg16f) uniform image2D refl_hit_tex;
layout(set = 0, binding = 31, r32ui) uniform writeonly uimage2D refl_hit_radiance_tex;

spirv_instruction(set = "GLSL.std.450", id = 79) float spvNMin(float, float);
spirv_instruction(set = "GLSL.std.450", id = 79) vec2 spvNMin(vec2, vec2);
spirv_instruction(set = "GLSL.std.450", id = 79) vec3 spvNMin(vec3, vec3);
spirv_instruction(set = "GLSL.std.450", id = 79) vec4 spvNMin(vec4, vec4);
spirv_instruction(set = "GLSL.std.450", id = 80) float spvNMax(float, float);
spirv_instruction(set = "GLSL.std.450", id = 80) vec2 spvNMax(vec2, vec2);
spirv_instruction(set = "GLSL.std.450", id = 80) vec3 spvNMax(vec3, vec3);
spirv_instruction(set = "GLSL.std.450", id = 80) vec4 spvNMax(vec4, vec4);

void main()
{
    do
    {
        if (gl_GlobalInvocationID.x >= ice_rt_count_buffer._m0[0u].w)
        {
            break;
        }
        uvec4 _224 = ice_rt_ray_buffer_1._m0[gl_GlobalInvocationID.x];
        uvec2 _229 = uvec2(_224.x & 65535u, (_224.x >> 16u) & 65535u);
        vec4 _232 = vec4(0.0, 0.0, 0.0, 1.0) * frame_constants.view_constants[0u].view_to_world;
        vec3 _250 = (_232.xyz / vec3(_232.w)) + vec3(unpackHalf2x16(_224.y & 65535u).x, unpackHalf2x16(_224.z & 65535u).x, unpackHalf2x16(_224.w & 65535u).x);
        vec2 _257 = unpackHalf2x16((_224.z >> 16u) & 65535u);
        float _258 = _257.x;
        vec3 _263 = vec3(unpackHalf2x16((_224.y >> 16u) & 65535u).x, _258, unpackHalf2x16((_224.w >> 16u) & 65535u).x);
        vec4 _265;
        vec3 _268;
        vec3 _270;
        vec3 _272;
        _265 = vec4(0.0, 0.0, 0.0, 100000.0);
        _268 = _263;
        _270 = _190;
        _272 = _250;
        vec4 _266;
        vec3 _269;
        vec3 _271;
        vec3 _273;
        vec4 _1138;
        uint _274 = 0u;
        for (;;)
        {
            if (_274 < 1u)
            {
                float _302;
                vec4 _296 = ((texelFetch(blue_noise_texture, ivec2((_229 + uvec2(fract((vec2(0.75487768650054931640625, 0.56984031200408935546875) * float((frame_constants.frame_index + 1234567u) + (_274 * 137u))) + vec2(0.5)) * vec2(256.0))) % uvec2(256u)), int(0u)) * 255.0) * vec4(0.00390625)) + vec4(0.001953125);
                vec3 _933;
                uint _934;
                float _935;
                do
                {
                    uvec2 _213[8] = uvec2[](uvec2(0u), uvec2(0u), uvec2(0u), uvec2(0u), uvec2(0u), uvec2(0u), uvec2(0u), uvec2(0u));
                    bool _300 = _268.x < 0.0;
                    _302 = _268.y;
                    bool _303 = _302 < 0.0;
                    bool _307 = _268.z < 0.0;
                    uint _312 = (7u - uint(((_300 ? 4 : 0) | (_303 ? 2 : 0)) | int(_307))) * 16843009u;
                    uvec2 _314;
                    vec3 _323;
                    _314 = uvec2(0u, 2147483648u);
                    _323 = _270;
                    uvec2 _315;
                    float _318;
                    int _320;
                    vec3 _324;
                    int _326;
                    float _924;
                    int _925;
                    float _317 = 3.4028234663852885981170418348452e+38;
                    int _319 = 0;
                    uint _321 = 0u;
                    int _325 = -1;
                    SPIRV_CROSS_LOOP
                    for (;;)
                    {
                        if (_321 < 1024u)
                        {
                            uvec2 _820;
                            uvec2 _821;
                            int _822;
                            if ((_314.y & 4278190080u) != 0u)
                            {
                                uint _336 = uint(findMSB(_314.y));
                                uint _341 = _314.y & (~(1u << (_336 & 31u)));
                                uvec2 _342 = _314;
                                _342.y = _341;
                                int _349;
                                if ((_341 & 4278190080u) != 0u)
                                {
                                    _213[_319] = _342;
                                    _349 = _319 + 1;
                                }
                                else
                                {
                                    _349 = _319;
                                }
                                vec3 _385 = vec3(uintBitsToFloat(((rt_bl_bvh._m0[_314.x + uint(bitCount(_314.y & (~(4294967295u << (((_336 - 24u) ^ (_312 & 255u)) & 31u)))))].data[0].w >> 0u) & 255u) << 23u) / _268.x, uintBitsToFloat(((rt_bl_bvh._m0[_314.x + uint(bitCount(_314.y & (~(4294967295u << (((_336 - 24u) ^ (_312 & 255u)) & 31u)))))].data[0].w >> 8u) & 255u) << 23u) / _302, uintBitsToFloat(((rt_bl_bvh._m0[_314.x + uint(bitCount(_314.y & (~(4294967295u << (((_336 - 24u) ^ (_312 & 255u)) & 31u)))))].data[0].w >> 16u) & 255u) << 23u) / _268.z);
                                vec3 _387 = (uintBitsToFloat(rt_bl_bvh._m0[_314.x + uint(bitCount(_314.y & (~(4294967295u << (((_336 - 24u) ^ (_312 & 255u)) & 31u)))))].data[0].xyz) - _272) / _268;
                                uint _396 = (rt_bl_bvh._m0[_314.x + uint(bitCount(_314.y & (~(4294967295u << (((_336 - 24u) ^ (_312 & 255u)) & 31u)))))].data[1].z ^ (_312 & ((((rt_bl_bvh._m0[_314.x + uint(bitCount(_314.y & (~(4294967295u << (((_336 - 24u) ^ (_312 & 255u)) & 31u)))))].data[1].z & (rt_bl_bvh._m0[_314.x + uint(bitCount(_314.y & (~(4294967295u << (((_336 - 24u) ^ (_312 & 255u)) & 31u)))))].data[1].z << 1u)) & 269488144u) >> 4u) * 255u))) & 522133279u;
                                uint _398 = (rt_bl_bvh._m0[_314.x + uint(bitCount(_314.y & (~(4294967295u << (((_336 - 24u) ^ (_312 & 255u)) & 31u)))))].data[1].z >> 5u) & 117901063u;
                                uint _405 = _300 ? rt_bl_bvh._m0[_314.x + uint(bitCount(_314.y & (~(4294967295u << (((_336 - 24u) ^ (_312 & 255u)) & 31u)))))].data[2].z : rt_bl_bvh._m0[_314.x + uint(bitCount(_314.y & (~(4294967295u << (((_336 - 24u) ^ (_312 & 255u)) & 31u)))))].data[2].x;
                                uint _406 = _300 ? rt_bl_bvh._m0[_314.x + uint(bitCount(_314.y & (~(4294967295u << (((_336 - 24u) ^ (_312 & 255u)) & 31u)))))].data[2].x : rt_bl_bvh._m0[_314.x + uint(bitCount(_314.y & (~(4294967295u << (((_336 - 24u) ^ (_312 & 255u)) & 31u)))))].data[2].z;
                                uint _407 = _303 ? rt_bl_bvh._m0[_314.x + uint(bitCount(_314.y & (~(4294967295u << (((_336 - 24u) ^ (_312 & 255u)) & 31u)))))].data[3].z : rt_bl_bvh._m0[_314.x + uint(bitCount(_314.y & (~(4294967295u << (((_336 - 24u) ^ (_312 & 255u)) & 31u)))))].data[3].x;
                                uint _408 = _303 ? rt_bl_bvh._m0[_314.x + uint(bitCount(_314.y & (~(4294967295u << (((_336 - 24u) ^ (_312 & 255u)) & 31u)))))].data[3].x : rt_bl_bvh._m0[_314.x + uint(bitCount(_314.y & (~(4294967295u << (((_336 - 24u) ^ (_312 & 255u)) & 31u)))))].data[3].z;
                                uint _409 = _307 ? rt_bl_bvh._m0[_314.x + uint(bitCount(_314.y & (~(4294967295u << (((_336 - 24u) ^ (_312 & 255u)) & 31u)))))].data[4].z : rt_bl_bvh._m0[_314.x + uint(bitCount(_314.y & (~(4294967295u << (((_336 - 24u) ^ (_312 & 255u)) & 31u)))))].data[4].x;
                                uint _410 = _307 ? rt_bl_bvh._m0[_314.x + uint(bitCount(_314.y & (~(4294967295u << (((_336 - 24u) ^ (_312 & 255u)) & 31u)))))].data[4].x : rt_bl_bvh._m0[_314.x + uint(bitCount(_314.y & (~(4294967295u << (((_336 - 24u) ^ (_312 & 255u)) & 31u)))))].data[4].z;
                                vec3 _432 = (vec3(float((_405 >> 0u) & 255u), float((_407 >> 0u) & 255u), float((_409 >> 0u) & 255u)) * _385) + _387;
                                vec3 _434 = (vec3(float((_406 >> 0u) & 255u), float((_408 >> 0u) & 255u), float((_410 >> 0u) & 255u)) * _385) + _387;
                                uint _457;
                                if (spvNMax(_432.x, spvNMax(_432.y, spvNMax(_432.z, 9.9999997473787516355514526367188e-05))) <= spvNMin(_434.x, spvNMin(_434.y, spvNMin(_434.z, _317))))
                                {
                                    _457 = 0u | (((_398 >> 0u) & 255u) << (((_396 >> 0u) & 255u) & 31u));
                                }
                                else
                                {
                                    _457 = 0u;
                                }
                                vec3 _479 = (vec3(float((_405 >> 8u) & 255u), float((_407 >> 8u) & 255u), float((_409 >> 8u) & 255u)) * _385) + _387;
                                vec3 _481 = (vec3(float((_406 >> 8u) & 255u), float((_408 >> 8u) & 255u), float((_410 >> 8u) & 255u)) * _385) + _387;
                                uint _504;
                                if (spvNMax(_479.x, spvNMax(_479.y, spvNMax(_479.z, 9.9999997473787516355514526367188e-05))) <= spvNMin(_481.x, spvNMin(_481.y, spvNMin(_481.z, _317))))
                                {
                                    _504 = _457 | (((_398 >> 8u) & 255u) << (((_396 >> 8u) & 255u) & 31u));
                                }
                                else
                                {
                                    _504 = _457;
                                }
                                vec3 _526 = (vec3(float((_405 >> 16u) & 255u), float((_407 >> 16u) & 255u), float((_409 >> 16u) & 255u)) * _385) + _387;
                                vec3 _528 = (vec3(float((_406 >> 16u) & 255u), float((_408 >> 16u) & 255u), float((_410 >> 16u) & 255u)) * _385) + _387;
                                uint _551;
                                if (spvNMax(_526.x, spvNMax(_526.y, spvNMax(_526.z, 9.9999997473787516355514526367188e-05))) <= spvNMin(_528.x, spvNMin(_528.y, spvNMin(_528.z, _317))))
                                {
                                    _551 = _504 | (((_398 >> 16u) & 255u) << (((_396 >> 16u) & 255u) & 31u));
                                }
                                else
                                {
                                    _551 = _504;
                                }
                                vec3 _573 = (vec3(float((_405 >> 24u) & 255u), float((_407 >> 24u) & 255u), float((_409 >> 24u) & 255u)) * _385) + _387;
                                vec3 _575 = (vec3(float((_406 >> 24u) & 255u), float((_408 >> 24u) & 255u), float((_410 >> 24u) & 255u)) * _385) + _387;
                                uint _598;
                                if (spvNMax(_573.x, spvNMax(_573.y, spvNMax(_573.z, 9.9999997473787516355514526367188e-05))) <= spvNMin(_575.x, spvNMin(_575.y, spvNMin(_575.z, _317))))
                                {
                                    _598 = _551 | (((_398 >> 24u) & 255u) << (((_396 >> 24u) & 255u) & 31u));
                                }
                                else
                                {
                                    _598 = _551;
                                }
                                uint _607 = (rt_bl_bvh._m0[_314.x + uint(bitCount(_314.y & (~(4294967295u << (((_336 - 24u) ^ (_312 & 255u)) & 31u)))))].data[1].w ^ (_312 & ((((rt_bl_bvh._m0[_314.x + uint(bitCount(_314.y & (~(4294967295u << (((_336 - 24u) ^ (_312 & 255u)) & 31u)))))].data[1].w & (rt_bl_bvh._m0[_314.x + uint(bitCount(_314.y & (~(4294967295u << (((_336 - 24u) ^ (_312 & 255u)) & 31u)))))].data[1].w << 1u)) & 269488144u) >> 4u) * 255u))) & 522133279u;
                                uint _609 = (rt_bl_bvh._m0[_314.x + uint(bitCount(_314.y & (~(4294967295u << (((_336 - 24u) ^ (_312 & 255u)) & 31u)))))].data[1].w >> 5u) & 117901063u;
                                uint _616 = _300 ? rt_bl_bvh._m0[_314.x + uint(bitCount(_314.y & (~(4294967295u << (((_336 - 24u) ^ (_312 & 255u)) & 31u)))))].data[2].w : rt_bl_bvh._m0[_314.x + uint(bitCount(_314.y & (~(4294967295u << (((_336 - 24u) ^ (_312 & 255u)) & 31u)))))].data[2].y;
                                uint _617 = _300 ? rt_bl_bvh._m0[_314.x + uint(bitCount(_314.y & (~(4294967295u << (((_336 - 24u) ^ (_312 & 255u)) & 31u)))))].data[2].y : rt_bl_bvh._m0[_314.x + uint(bitCount(_314.y & (~(4294967295u << (((_336 - 24u) ^ (_312 & 255u)) & 31u)))))].data[2].w;
                                uint _618 = _303 ? rt_bl_bvh._m0[_314.x + uint(bitCount(_314.y & (~(4294967295u << (((_336 - 24u) ^ (_312 & 255u)) & 31u)))))].data[3].w : rt_bl_bvh._m0[_314.x + uint(bitCount(_314.y & (~(4294967295u << (((_336 - 24u) ^ (_312 & 255u)) & 31u)))))].data[3].y;
                                uint _619 = _303 ? rt_bl_bvh._m0[_314.x + uint(bitCount(_314.y & (~(4294967295u << (((_336 - 24u) ^ (_312 & 255u)) & 31u)))))].data[3].y : rt_bl_bvh._m0[_314.x + uint(bitCount(_314.y & (~(4294967295u << (((_336 - 24u) ^ (_312 & 255u)) & 31u)))))].data[3].w;
                                uint _620 = _307 ? rt_bl_bvh._m0[_314.x + uint(bitCount(_314.y & (~(4294967295u << (((_336 - 24u) ^ (_312 & 255u)) & 31u)))))].data[4].w : rt_bl_bvh._m0[_314.x + uint(bitCount(_314.y & (~(4294967295u << (((_336 - 24u) ^ (_312 & 255u)) & 31u)))))].data[4].y;
                                uint _621 = _307 ? rt_bl_bvh._m0[_314.x + uint(bitCount(_314.y & (~(4294967295u << (((_336 - 24u) ^ (_312 & 255u)) & 31u)))))].data[4].y : rt_bl_bvh._m0[_314.x + uint(bitCount(_314.y & (~(4294967295u << (((_336 - 24u) ^ (_312 & 255u)) & 31u)))))].data[4].w;
                                vec3 _643 = (vec3(float((_616 >> 0u) & 255u), float((_618 >> 0u) & 255u), float((_620 >> 0u) & 255u)) * _385) + _387;
                                vec3 _645 = (vec3(float((_617 >> 0u) & 255u), float((_619 >> 0u) & 255u), float((_621 >> 0u) & 255u)) * _385) + _387;
                                uint _668;
                                if (spvNMax(_643.x, spvNMax(_643.y, spvNMax(_643.z, 9.9999997473787516355514526367188e-05))) <= spvNMin(_645.x, spvNMin(_645.y, spvNMin(_645.z, _317))))
                                {
                                    _668 = _598 | (((_609 >> 0u) & 255u) << (((_607 >> 0u) & 255u) & 31u));
                                }
                                else
                                {
                                    _668 = _598;
                                }
                                vec3 _690 = (vec3(float((_616 >> 8u) & 255u), float((_618 >> 8u) & 255u), float((_620 >> 8u) & 255u)) * _385) + _387;
                                vec3 _692 = (vec3(float((_617 >> 8u) & 255u), float((_619 >> 8u) & 255u), float((_621 >> 8u) & 255u)) * _385) + _387;
                                uint _715;
                                if (spvNMax(_690.x, spvNMax(_690.y, spvNMax(_690.z, 9.9999997473787516355514526367188e-05))) <= spvNMin(_692.x, spvNMin(_692.y, spvNMin(_692.z, _317))))
                                {
                                    _715 = _668 | (((_609 >> 8u) & 255u) << (((_607 >> 8u) & 255u) & 31u));
                                }
                                else
                                {
                                    _715 = _668;
                                }
                                vec3 _737 = (vec3(float((_616 >> 16u) & 255u), float((_618 >> 16u) & 255u), float((_620 >> 16u) & 255u)) * _385) + _387;
                                vec3 _739 = (vec3(float((_617 >> 16u) & 255u), float((_619 >> 16u) & 255u), float((_621 >> 16u) & 255u)) * _385) + _387;
                                uint _762;
                                if (spvNMax(_737.x, spvNMax(_737.y, spvNMax(_737.z, 9.9999997473787516355514526367188e-05))) <= spvNMin(_739.x, spvNMin(_739.y, spvNMin(_739.z, _317))))
                                {
                                    _762 = _715 | (((_609 >> 16u) & 255u) << (((_607 >> 16u) & 255u) & 31u));
                                }
                                else
                                {
                                    _762 = _715;
                                }
                                vec3 _784 = (vec3(float((_616 >> 24u) & 255u), float((_618 >> 24u) & 255u), float((_620 >> 24u) & 255u)) * _385) + _387;
                                vec3 _786 = (vec3(float((_617 >> 24u) & 255u), float((_619 >> 24u) & 255u), float((_621 >> 24u) & 255u)) * _385) + _387;
                                uint _809;
                                if (spvNMax(_784.x, spvNMax(_784.y, spvNMax(_784.z, 9.9999997473787516355514526367188e-05))) <= spvNMin(_786.x, spvNMin(_786.y, spvNMin(_786.z, _317))))
                                {
                                    _809 = _762 | (((_609 >> 24u) & 255u) << (((_607 >> 24u) & 255u) & 31u));
                                }
                                else
                                {
                                    _809 = _762;
                                }
                                uvec2 _813 = uvec2(rt_bl_bvh._m0[_314.x + uint(bitCount(_314.y & (~(4294967295u << (((_336 - 24u) ^ (_312 & 255u)) & 31u)))))].data[1].x, _341);
                                _813.y = (_809 & 4278190080u) | ((rt_bl_bvh._m0[_314.x + uint(bitCount(_314.y & (~(4294967295u << (((_336 - 24u) ^ (_312 & 255u)) & 31u)))))].data[0].w >> 24u) & 255u);
                                _820 = _813;
                                _821 = uvec2(rt_bl_bvh._m0[_314.x + uint(bitCount(_314.y & (~(4294967295u << (((_336 - 24u) ^ (_312 & 255u)) & 31u)))))].data[1].y, _809 & 16777215u);
                                _822 = _349;
                            }
                            else
                            {
                                _820 = uvec2(0u);
                                _821 = _314;
                                _822 = _319;
                            }
                            uvec2 _824;
                            _824 = _821;
                            _324 = _323;
                            _318 = _317;
                            _326 = _325;
                            uvec2 _825;
                            vec3 _827;
                            float _828;
                            int _829;
                            uint _830;
                            for (;;)
                            {
                                _830 = _824.y;
                                if (_830 != 0u)
                                {
                                    uint _834 = uint(findMSB(_830));
                                    _825 = _824;
                                    _825.y = _830 & (~(1u << (_834 & 31u)));
                                    uint _840 = _824.x + _834;
                                    vec3 _873 = vec3(unpackHalf2x16((rt_triangles._m0[_840].e[0] >> 16u) & 65535u).x, unpackHalf2x16((rt_triangles._m0[_840].e[1] >> 16u) & 65535u).x, unpackHalf2x16((rt_triangles._m0[_840].e[2] >> 16u) & 65535u).x);
                                    vec3 _874 = vec3(unpackHalf2x16(rt_triangles._m0[_840].e[0] & 65535u).x, unpackHalf2x16(rt_triangles._m0[_840].e[1] & 65535u).x, unpackHalf2x16(rt_triangles._m0[_840].e[2] & 65535u).x);
                                    bool _908;
                                    do
                                    {
                                        vec3 _877 = cross(_268, _874);
                                        float _878 = dot(_873, _877);
                                        vec3 _879 = _272 - vec3(rt_triangles._m0[_840].v[0], rt_triangles._m0[_840].v[1], rt_triangles._m0[_840].v[2]);
                                        vec3 _880 = cross(_879, _873);
                                        float _882 = dot(_874, _880) / _878;
                                        bool _887;
                                        if (_882 >= 0.0)
                                        {
                                            _887 = _882 < _318;
                                        }
                                        else
                                        {
                                            _887 = false;
                                        }
                                        if (_887 ? true : false)
                                        {
                                            vec3 _892;
                                            _892.x = dot(_879, _877);
                                            _892.y = dot(_268, _880);
                                            vec2 _897 = _892.xy / vec2(_878);
                                            vec3 _898 = vec3(_897.x, _897.y, _892.z);
                                            _898.z = (1.0 - _897.x) - _897.y;
                                            if (all(greaterThanEqual(_898, vec3(-9.9999997473787516355514526367188e-05))))
                                            {
                                                _828 = _882;
                                                _908 = true;
                                                break;
                                            }
                                        }
                                        _828 = _318;
                                        _908 = false;
                                        break;
                                    } while(false);
                                    if (_908)
                                    {
                                        _827 = cross(_873, _874);
                                        _829 = int(_840);
                                    }
                                    else
                                    {
                                        _827 = _324;
                                        _829 = _326;
                                    }
                                    _824 = _825;
                                    _324 = _827;
                                    _318 = _828;
                                    _326 = _829;
                                    continue;
                                }
                                else
                                {
                                    break;
                                }
                            }
                            if ((_820.y & 4278190080u) == 0u)
                            {
                                if (_822 == 0)
                                {
                                    _271 = _324;
                                    _924 = _318;
                                    _925 = _326;
                                    break;
                                }
                                int _921 = _822 - 1;
                                _315 = _213[_921];
                                _320 = _921;
                            }
                            else
                            {
                                _315 = _820;
                                _320 = _822;
                            }
                            _314 = _315;
                            _317 = _318;
                            _319 = _320;
                            _321++;
                            _323 = _324;
                            _325 = _326;
                            continue;
                        }
                        else
                        {
                            _271 = _323;
                            _924 = _317;
                            _925 = _325;
                            break;
                        }
                    }
                    bool _930;
                    if (_925 != (-1))
                    {
                        _930 = _924 < 100000.0;
                    }
                    else
                    {
                        _930 = false;
                    }
                    if (_930)
                    {
                        _933 = _271;
                        _934 = 3u;
                        _935 = _924;
                        break;
                    }
                    _933 = vec3(0.0, 1.0, 0.0);
                    _934 = 0u;
                    _935 = 100000.0;
                    break;
                } while(false);
                uint _1050;
                float _1051;
                do
                {
                    vec3 _939 = (vec3(65.0, 24.0, 65.0) - _272) / _268;
                    vec3 _941 = (vec3(-65.0, 0.0, -65.0) - _272) / _268;
                    vec3 _942 = spvNMin(_939, _941);
                    vec3 _943 = spvNMax(_939, _941);
                    float _948 = spvNMax(spvNMax(_942.x, _942.y), _942.z);
                    float _953 = spvNMin(spvNMin(_943.x, _943.y), _943.z);
                    bool _958;
                    if (_948 <= _953)
                    {
                        _958 = _953 >= 0.0;
                    }
                    else
                    {
                        _958 = false;
                    }
                    if (!_958)
                    {
                        _1050 = _934;
                        _1051 = _935;
                        break;
                    }
                    float _962 = spvNMax(_948, 0.0);
                    float _964 = (_953 - _962) * 0.03125;
                    float _966;
                    _966 = _962;
                    float _967;
                    uint _970;
                    float _989;
                    uint _969 = 0u;
                    bool _971;
                    for (;;)
                    {
                        _971 = _969 < 32u;
                        if (_971)
                        {
                            _967 = _966 + _964;
                            vec3 _975 = _272 + (_268 * _967);
                            if (_975.y < mix(-2.5, 24.0, textureLod(sampler2D(visual_terrain_heightmap, sampler_lnc), (_975.xz * vec2(0.0076923076994717121124267578125)) + vec2(0.5), 0.0).x))
                            {
                                _989 = _967;
                                break;
                            }
                            _970 = _969 + 1u;
                            _966 = _967;
                            _969 = _970;
                            continue;
                        }
                        else
                        {
                            _989 = _953;
                            break;
                        }
                    }
                    bool _993;
                    if (_971)
                    {
                        _993 = _966 < _935;
                    }
                    else
                    {
                        _993 = false;
                    }
                    bool _997;
                    if (_993)
                    {
                        _997 = _966 < _989;
                    }
                    else
                    {
                        _997 = false;
                    }
                    if (_997)
                    {
                        float _1005;
                        _1005 = _966;
                        float _1002 = _989;
                        uint _1007 = 0u;
                        for (; _1007 < 8u; )
                        {
                            float _1011 = _1005 + _1002;
                            float _1012 = _1011 * 0.5;
                            vec3 _1014 = _272 + (_268 * _1012);
                            bool _1025 = _1014.y < mix(-2.5, 24.0, textureLod(sampler2D(visual_terrain_heightmap, sampler_lnc), (_1014.xz * vec2(0.0076923076994717121124267578125)) + vec2(0.5), 0.0).x);
                            _1002 = _1025 ? _1012 : _1002;
                            _1005 = _1025 ? _1005 : _1012;
                            _1007++;
                            continue;
                        }
                        if (_1005 < _935)
                        {
                            vec2 _1034 = ((_272 + (_268 * _1005)).xz * vec2(0.0076923076994717121124267578125)) + vec2(0.5);
                            _1050 = ((mix(-2.5, 24.0, textureLod(sampler2D(terrain_heightmap, sampler_default), _1034, 0.0).x) + 0.100000001490116119384765625) > mix(-2.5, 24.0, textureLod(sampler2D(visual_terrain_heightmap, sampler_lnc), _1034, 0.0).x)) ? 1u : 2u;
                            _1051 = _1005;
                            break;
                        }
                        else
                        {
                            _1050 = _934;
                            _1051 = _935;
                            break;
                        }
                        break; // unreachable workaround
                    }
                    else
                    {
                        _1050 = _934;
                        _1051 = _935;
                        break;
                    }
                    break; // unreachable workaround
                } while(false);
                if (_1050 == 0u)
                {
                    vec4 _1137;
                    if (_302 > 0.0)
                    {
                        _1137 = vec4(_210.x, _210.y, _210.z, _265.w);
                    }
                    else
                    {
                        vec4 _1135;
                        if (frame_constants.glade_theme_id == 2u)
                        {
                            _1135 = vec4(_210.x, _210.y, _210.z, _265.w);
                        }
                        else
                        {
                            _1135 = _265;
                        }
                        _1137 = _1135;
                    }
                    _1138 = _1137;
                    break;
                }
                else
                {
                    vec4 _1060;
                    if (0u == _274)
                    {
                        vec4 _1059;
                        _1059.w = _1051;
                        _1060 = _1059;
                    }
                    else
                    {
                        _1060 = _265;
                    }
                    vec3 _1062 = _272 + (_268 * _1051);
                    bool _1068;
                    if (!(_1050 == 1u))
                    {
                        _1068 = _1050 == 2u;
                    }
                    else
                    {
                        _1068 = true;
                    }
                    if (_1068)
                    {
                        if ((_274 + 1u) == 1u)
                        {
                            _1138 = vec4(_210.x, _210.y, _210.z, _1060.w);
                            break;
                        }
                        float _1110 = 1.0 - (2.0 * _296.x);
                        float _1114 = sqrt(spvNMax(0.0, 1.0 - (_1110 * _1110)));
                        float _1116 = 6.283185482025146484375 * _296.y;
                        _266 = vec4(_210.x, _210.y, _210.z, _1060.w);
                        _269 = normalize(vec3(0.0, 1.0, 0.0) + vec3(cos(_1116) * _1114, sin(_1116) * _1114, _1110));
                        _273 = _1062 + vec3(0.0, 0.001000000047497451305389404296875, 0.0);
                    }
                    else
                    {
                        vec4 _1099;
                        vec3 _1100;
                        vec3 _1101;
                        if (_1050 == 3u)
                        {
                            if (dot(_933, _268) > 0.0)
                            {
                                _1138 = _1060;
                                break;
                            }
                            vec3 _1079 = normalize(_933);
                            float _1085 = 1.0 - (2.0 * _296.x);
                            float _1089 = sqrt(spvNMax(0.0, 1.0 - (_1085 * _1085)));
                            float _1091 = 6.283185482025146484375 * _296.y;
                            _1099 = vec4(_210.x, _210.y, _210.z, _1060.w);
                            _1100 = normalize(_1079 + vec3(cos(_1091) * _1089, sin(_1091) * _1089, _1085));
                            _1101 = _1062 + (_1079 * 0.001000000047497451305389404296875);
                        }
                        else
                        {
                            _1099 = _1060;
                            _1100 = _268;
                            _1101 = _272;
                        }
                        _266 = _1099;
                        _269 = _1100;
                        _273 = _1101;
                    }
                }
                _265 = _266;
                _268 = _269;
                _270 = _271;
                _272 = _273;
                _274++;
                continue;
            }
            else
            {
                _1138 = _265;
                break;
            }
        }
        vec4 _1151 = (vec4(_250 + (_263 * _1138.w), 1.0) * frame_constants.view_constants[0u].world_to_view) * frame_constants.view_constants[0u].view_to_sample;
        vec3 _1155 = _1151.xyz / vec3(_1151.w);
        vec2 _1158 = (_1155.xy * vec2(0.5, -0.5)) + vec2(0.5);
        vec4 _1162 = textureLod(sampler2D(reprojection_tex, sampler_nnc), _1158, 0.0);
        vec4 _1164 = imageLoad(refl_hit_tex, ivec2(_229));
        bool _1175;
        if (all(lessThan(abs(_1155), vec3(1.0))))
        {
            _1175 = _1155.z > 0.0;
        }
        else
        {
            _1175 = false;
        }
        bool _1187;
        if (_1175)
        {
            _1187 = textureLod(sampler2D(half_depth_tex, sampler_nnc), _1162.xy + _1158, 0.0).x < (_1155.z * 1.10000002384185791015625);
        }
        else
        {
            _1187 = false;
        }
        vec3 _1212;
        float _1213;
        if (_1187)
        {
            bool _1202;
            if (_Globals.history_valid != 0u)
            {
                _1202 = !(0u == frame_constants.frame_index);
            }
            else
            {
                _1202 = false;
            }
            vec3 _1211;
            if (_1202)
            {
                _1211 = textureLod(sampler2D(taa_history_tex, sampler_nnc), _1162.xy + _1158, 0.0).xyz;
            }
            else
            {
                _1211 = vec3(0.0);
            }
            _1212 = _1211;
            _1213 = 0.0;
        }
        else
        {
            _1212 = vec3(0.0);
            _1213 = float(_1138.w > 10000.0);
        }
        vec3 _1238;
        if (_1213 > 0.0)
        {
            vec3 _1217 = spvNMax(vec3(0.0), vec3(0.0));
            _1238 = _1212 + (((mix(frame_constants.sky_low_color.xyz * 1.0, frame_constants.sky_high_color.xyz, vec3(pow(clamp(1.0 - pow(1.0 - clamp(_258 + 0.20000000298023223876953125, 0.0, 1.0), 14.0), 0.0, 1.0), 0.64999997615814208984375))) * 1.0) + _1217) * _1213);
        }
        else
        {
            _1238 = _1212;
        }
        imageStore(refl_hit_tex, ivec2(_229), vec2(_1164.x + _1138.w, _1164.y).xyyy);
        vec3 _1241 = clamp(_1238, vec3(0.0), vec3(65408.0));
        float _1251 = uintBitsToFloat((floatBitsToUint(spvNMax(spvNMax(1.52587890625e-05, _1241.x), spvNMax(_1241.y, _1241.z))) + 125845504u) & 2139095040u);
        uvec3 _1254 = floatBitsToUint(_1241 + vec3(_1251));
        imageStore(refl_hit_radiance_tex, ivec2(_229), uvec4(((((floatBitsToUint(_1251) << 4u) + 268435456u) | (_1254.z << 18u)) | (_1254.y << 9u)) | (_1254.x & 511u)));
        break;
    } while(false);
}

