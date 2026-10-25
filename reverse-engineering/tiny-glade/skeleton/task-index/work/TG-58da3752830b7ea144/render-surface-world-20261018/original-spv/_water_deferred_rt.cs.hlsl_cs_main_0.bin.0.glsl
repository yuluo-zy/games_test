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

vec3 _198;
vec3 _219;
vec4 _220;

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
    layout(offset = 80) uvec2 color_tex_dims;
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
} water_rt_count_buffer;

layout(set = 0, binding = 26, std430) readonly buffer water_rt_ray_buffer
{
    uvec4 _m0[];
} water_rt_ray_buffer_1;

layout(set = 0, binding = 0) uniform sampler sampler_lnc;
layout(set = 0, binding = 2) uniform sampler sampler_nnc;
layout(set = 0, binding = 9) uniform sampler sampler_default;
layout(set = 0, binding = 15) uniform texture2D blue_noise_texture;
layout(set = 0, binding = 18) uniform texture2D terrain_heightmap;
layout(set = 0, binding = 19) uniform texture2D visual_terrain_heightmap;
layout(set = 0, binding = 27) uniform texture2D half_depth_tex;
layout(set = 0, binding = 28) uniform texture2D taa_history_tex;
layout(set = 0, binding = 29) uniform texture2D reprojection_tex;
layout(set = 0, binding = 30, r11f_g11f_b10f) uniform image2D color_tex;

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
        if (gl_GlobalInvocationID.x >= water_rt_count_buffer._m0[0u].w)
        {
            break;
        }
        uvec4 _233 = water_rt_ray_buffer_1._m0[gl_GlobalInvocationID.x];
        uvec2 _238 = uvec2(_233.x & 65535u, (_233.x >> 16u) & 65535u);
        vec2 _245 = (vec2(ivec2(_238)) + vec2(0.5)) / vec2(_Globals.half_depth_tex_dims);
        vec2 _253 = (_245 - vec2(0.5)) * vec2(2.0, -2.0);
        float _254 = _253.x;
        vec4 _261 = (vec4(_254, _253.y, uintBitsToFloat(_233.w), 1.0) * frame_constants.view_constants[0u].sample_to_view) * frame_constants.view_constants[0u].view_to_world;
        vec3 _264 = -normalize(((vec4(_254, _253.y, 0.0, 1.0) * frame_constants.view_constants[0u].sample_to_view) * frame_constants.view_constants[0u].view_to_world).xyz);
        uint _269 = _233.w & 15u;
        vec2 _280 = (vec2(unpackHalf2x16(_233.y & 65535u).x, unpackHalf2x16((_233.y >> 16u) & 65535u).x) * 2.0) - vec2(1.0);
        float _281 = _280.x;
        float _286 = (1.0 - abs(_281)) - abs(_280.y);
        vec3 _287 = vec3(_281, _280.y, _286);
        vec2 _290 = _287.xy;
        vec2 _295 = _290 - (((step(vec2(0.0), _290) * 2.0) - vec2(1.0)) * clamp(-_286, 0.0, 1.0));
        vec3 _297 = normalize(vec3(_295.x, _295.y, _287.z));
        vec3 _301 = -reflect(_264, _297);
        float _302 = _301.y;
        vec3 _308;
        if (_302 < 0.0)
        {
            vec3 _307 = _301;
            _307.y = -_302;
            _308 = _307;
        }
        else
        {
            _308 = _301;
        }
        vec3 _310 = (_261.xyz / vec3(_261.w)) + (_308 * uintBitsToFloat(_233.z));
        vec4 _312;
        vec3 _315;
        vec3 _317;
        vec3 _319;
        _312 = vec4(0.0, 0.0, 0.0, 100000.0);
        _315 = _308;
        _317 = _198;
        _319 = _310;
        vec4 _313;
        vec3 _316;
        vec3 _318;
        vec3 _320;
        vec4 _1185;
        uint _321 = 0u;
        for (;;)
        {
            if (_321 < 1u)
            {
                float _349;
                vec4 _343 = ((texelFetch(blue_noise_texture, ivec2((_238 + uvec2(fract((vec2(0.75487768650054931640625, 0.56984031200408935546875) * float((frame_constants.frame_index + 1234567u) + (_321 * 137u))) + vec2(0.5)) * vec2(256.0))) % uvec2(256u)), int(0u)) * 255.0) * vec4(0.00390625)) + vec4(0.001953125);
                vec3 _980;
                uint _981;
                float _982;
                do
                {
                    uvec2 _222[8] = uvec2[](uvec2(0u), uvec2(0u), uvec2(0u), uvec2(0u), uvec2(0u), uvec2(0u), uvec2(0u), uvec2(0u));
                    bool _347 = _315.x < 0.0;
                    _349 = _315.y;
                    bool _350 = _349 < 0.0;
                    bool _354 = _315.z < 0.0;
                    uint _359 = (7u - uint(((_347 ? 4 : 0) | (_350 ? 2 : 0)) | int(_354))) * 16843009u;
                    uvec2 _361;
                    vec3 _370;
                    _361 = uvec2(0u, 2147483648u);
                    _370 = _317;
                    uvec2 _362;
                    float _365;
                    int _367;
                    vec3 _371;
                    int _373;
                    float _971;
                    int _972;
                    float _364 = 3.4028234663852885981170418348452e+38;
                    int _366 = 0;
                    uint _368 = 0u;
                    int _372 = -1;
                    SPIRV_CROSS_LOOP
                    for (;;)
                    {
                        if (_368 < 1024u)
                        {
                            uvec2 _867;
                            uvec2 _868;
                            int _869;
                            if ((_361.y & 4278190080u) != 0u)
                            {
                                uint _383 = uint(findMSB(_361.y));
                                uint _388 = _361.y & (~(1u << (_383 & 31u)));
                                uvec2 _389 = _361;
                                _389.y = _388;
                                int _396;
                                if ((_388 & 4278190080u) != 0u)
                                {
                                    _222[_366] = _389;
                                    _396 = _366 + 1;
                                }
                                else
                                {
                                    _396 = _366;
                                }
                                vec3 _432 = vec3(uintBitsToFloat(((rt_bl_bvh._m0[_361.x + uint(bitCount(_361.y & (~(4294967295u << (((_383 - 24u) ^ (_359 & 255u)) & 31u)))))].data[0].w >> 0u) & 255u) << 23u) / _315.x, uintBitsToFloat(((rt_bl_bvh._m0[_361.x + uint(bitCount(_361.y & (~(4294967295u << (((_383 - 24u) ^ (_359 & 255u)) & 31u)))))].data[0].w >> 8u) & 255u) << 23u) / _349, uintBitsToFloat(((rt_bl_bvh._m0[_361.x + uint(bitCount(_361.y & (~(4294967295u << (((_383 - 24u) ^ (_359 & 255u)) & 31u)))))].data[0].w >> 16u) & 255u) << 23u) / _315.z);
                                vec3 _434 = (uintBitsToFloat(rt_bl_bvh._m0[_361.x + uint(bitCount(_361.y & (~(4294967295u << (((_383 - 24u) ^ (_359 & 255u)) & 31u)))))].data[0].xyz) - _319) / _315;
                                uint _443 = (rt_bl_bvh._m0[_361.x + uint(bitCount(_361.y & (~(4294967295u << (((_383 - 24u) ^ (_359 & 255u)) & 31u)))))].data[1].z ^ (_359 & ((((rt_bl_bvh._m0[_361.x + uint(bitCount(_361.y & (~(4294967295u << (((_383 - 24u) ^ (_359 & 255u)) & 31u)))))].data[1].z & (rt_bl_bvh._m0[_361.x + uint(bitCount(_361.y & (~(4294967295u << (((_383 - 24u) ^ (_359 & 255u)) & 31u)))))].data[1].z << 1u)) & 269488144u) >> 4u) * 255u))) & 522133279u;
                                uint _445 = (rt_bl_bvh._m0[_361.x + uint(bitCount(_361.y & (~(4294967295u << (((_383 - 24u) ^ (_359 & 255u)) & 31u)))))].data[1].z >> 5u) & 117901063u;
                                uint _452 = _347 ? rt_bl_bvh._m0[_361.x + uint(bitCount(_361.y & (~(4294967295u << (((_383 - 24u) ^ (_359 & 255u)) & 31u)))))].data[2].z : rt_bl_bvh._m0[_361.x + uint(bitCount(_361.y & (~(4294967295u << (((_383 - 24u) ^ (_359 & 255u)) & 31u)))))].data[2].x;
                                uint _453 = _347 ? rt_bl_bvh._m0[_361.x + uint(bitCount(_361.y & (~(4294967295u << (((_383 - 24u) ^ (_359 & 255u)) & 31u)))))].data[2].x : rt_bl_bvh._m0[_361.x + uint(bitCount(_361.y & (~(4294967295u << (((_383 - 24u) ^ (_359 & 255u)) & 31u)))))].data[2].z;
                                uint _454 = _350 ? rt_bl_bvh._m0[_361.x + uint(bitCount(_361.y & (~(4294967295u << (((_383 - 24u) ^ (_359 & 255u)) & 31u)))))].data[3].z : rt_bl_bvh._m0[_361.x + uint(bitCount(_361.y & (~(4294967295u << (((_383 - 24u) ^ (_359 & 255u)) & 31u)))))].data[3].x;
                                uint _455 = _350 ? rt_bl_bvh._m0[_361.x + uint(bitCount(_361.y & (~(4294967295u << (((_383 - 24u) ^ (_359 & 255u)) & 31u)))))].data[3].x : rt_bl_bvh._m0[_361.x + uint(bitCount(_361.y & (~(4294967295u << (((_383 - 24u) ^ (_359 & 255u)) & 31u)))))].data[3].z;
                                uint _456 = _354 ? rt_bl_bvh._m0[_361.x + uint(bitCount(_361.y & (~(4294967295u << (((_383 - 24u) ^ (_359 & 255u)) & 31u)))))].data[4].z : rt_bl_bvh._m0[_361.x + uint(bitCount(_361.y & (~(4294967295u << (((_383 - 24u) ^ (_359 & 255u)) & 31u)))))].data[4].x;
                                uint _457 = _354 ? rt_bl_bvh._m0[_361.x + uint(bitCount(_361.y & (~(4294967295u << (((_383 - 24u) ^ (_359 & 255u)) & 31u)))))].data[4].x : rt_bl_bvh._m0[_361.x + uint(bitCount(_361.y & (~(4294967295u << (((_383 - 24u) ^ (_359 & 255u)) & 31u)))))].data[4].z;
                                vec3 _479 = (vec3(float((_452 >> 0u) & 255u), float((_454 >> 0u) & 255u), float((_456 >> 0u) & 255u)) * _432) + _434;
                                vec3 _481 = (vec3(float((_453 >> 0u) & 255u), float((_455 >> 0u) & 255u), float((_457 >> 0u) & 255u)) * _432) + _434;
                                uint _504;
                                if (spvNMax(_479.x, spvNMax(_479.y, spvNMax(_479.z, 9.9999997473787516355514526367188e-05))) <= spvNMin(_481.x, spvNMin(_481.y, spvNMin(_481.z, _364))))
                                {
                                    _504 = 0u | (((_445 >> 0u) & 255u) << (((_443 >> 0u) & 255u) & 31u));
                                }
                                else
                                {
                                    _504 = 0u;
                                }
                                vec3 _526 = (vec3(float((_452 >> 8u) & 255u), float((_454 >> 8u) & 255u), float((_456 >> 8u) & 255u)) * _432) + _434;
                                vec3 _528 = (vec3(float((_453 >> 8u) & 255u), float((_455 >> 8u) & 255u), float((_457 >> 8u) & 255u)) * _432) + _434;
                                uint _551;
                                if (spvNMax(_526.x, spvNMax(_526.y, spvNMax(_526.z, 9.9999997473787516355514526367188e-05))) <= spvNMin(_528.x, spvNMin(_528.y, spvNMin(_528.z, _364))))
                                {
                                    _551 = _504 | (((_445 >> 8u) & 255u) << (((_443 >> 8u) & 255u) & 31u));
                                }
                                else
                                {
                                    _551 = _504;
                                }
                                vec3 _573 = (vec3(float((_452 >> 16u) & 255u), float((_454 >> 16u) & 255u), float((_456 >> 16u) & 255u)) * _432) + _434;
                                vec3 _575 = (vec3(float((_453 >> 16u) & 255u), float((_455 >> 16u) & 255u), float((_457 >> 16u) & 255u)) * _432) + _434;
                                uint _598;
                                if (spvNMax(_573.x, spvNMax(_573.y, spvNMax(_573.z, 9.9999997473787516355514526367188e-05))) <= spvNMin(_575.x, spvNMin(_575.y, spvNMin(_575.z, _364))))
                                {
                                    _598 = _551 | (((_445 >> 16u) & 255u) << (((_443 >> 16u) & 255u) & 31u));
                                }
                                else
                                {
                                    _598 = _551;
                                }
                                vec3 _620 = (vec3(float((_452 >> 24u) & 255u), float((_454 >> 24u) & 255u), float((_456 >> 24u) & 255u)) * _432) + _434;
                                vec3 _622 = (vec3(float((_453 >> 24u) & 255u), float((_455 >> 24u) & 255u), float((_457 >> 24u) & 255u)) * _432) + _434;
                                uint _645;
                                if (spvNMax(_620.x, spvNMax(_620.y, spvNMax(_620.z, 9.9999997473787516355514526367188e-05))) <= spvNMin(_622.x, spvNMin(_622.y, spvNMin(_622.z, _364))))
                                {
                                    _645 = _598 | (((_445 >> 24u) & 255u) << (((_443 >> 24u) & 255u) & 31u));
                                }
                                else
                                {
                                    _645 = _598;
                                }
                                uint _654 = (rt_bl_bvh._m0[_361.x + uint(bitCount(_361.y & (~(4294967295u << (((_383 - 24u) ^ (_359 & 255u)) & 31u)))))].data[1].w ^ (_359 & ((((rt_bl_bvh._m0[_361.x + uint(bitCount(_361.y & (~(4294967295u << (((_383 - 24u) ^ (_359 & 255u)) & 31u)))))].data[1].w & (rt_bl_bvh._m0[_361.x + uint(bitCount(_361.y & (~(4294967295u << (((_383 - 24u) ^ (_359 & 255u)) & 31u)))))].data[1].w << 1u)) & 269488144u) >> 4u) * 255u))) & 522133279u;
                                uint _656 = (rt_bl_bvh._m0[_361.x + uint(bitCount(_361.y & (~(4294967295u << (((_383 - 24u) ^ (_359 & 255u)) & 31u)))))].data[1].w >> 5u) & 117901063u;
                                uint _663 = _347 ? rt_bl_bvh._m0[_361.x + uint(bitCount(_361.y & (~(4294967295u << (((_383 - 24u) ^ (_359 & 255u)) & 31u)))))].data[2].w : rt_bl_bvh._m0[_361.x + uint(bitCount(_361.y & (~(4294967295u << (((_383 - 24u) ^ (_359 & 255u)) & 31u)))))].data[2].y;
                                uint _664 = _347 ? rt_bl_bvh._m0[_361.x + uint(bitCount(_361.y & (~(4294967295u << (((_383 - 24u) ^ (_359 & 255u)) & 31u)))))].data[2].y : rt_bl_bvh._m0[_361.x + uint(bitCount(_361.y & (~(4294967295u << (((_383 - 24u) ^ (_359 & 255u)) & 31u)))))].data[2].w;
                                uint _665 = _350 ? rt_bl_bvh._m0[_361.x + uint(bitCount(_361.y & (~(4294967295u << (((_383 - 24u) ^ (_359 & 255u)) & 31u)))))].data[3].w : rt_bl_bvh._m0[_361.x + uint(bitCount(_361.y & (~(4294967295u << (((_383 - 24u) ^ (_359 & 255u)) & 31u)))))].data[3].y;
                                uint _666 = _350 ? rt_bl_bvh._m0[_361.x + uint(bitCount(_361.y & (~(4294967295u << (((_383 - 24u) ^ (_359 & 255u)) & 31u)))))].data[3].y : rt_bl_bvh._m0[_361.x + uint(bitCount(_361.y & (~(4294967295u << (((_383 - 24u) ^ (_359 & 255u)) & 31u)))))].data[3].w;
                                uint _667 = _354 ? rt_bl_bvh._m0[_361.x + uint(bitCount(_361.y & (~(4294967295u << (((_383 - 24u) ^ (_359 & 255u)) & 31u)))))].data[4].w : rt_bl_bvh._m0[_361.x + uint(bitCount(_361.y & (~(4294967295u << (((_383 - 24u) ^ (_359 & 255u)) & 31u)))))].data[4].y;
                                uint _668 = _354 ? rt_bl_bvh._m0[_361.x + uint(bitCount(_361.y & (~(4294967295u << (((_383 - 24u) ^ (_359 & 255u)) & 31u)))))].data[4].y : rt_bl_bvh._m0[_361.x + uint(bitCount(_361.y & (~(4294967295u << (((_383 - 24u) ^ (_359 & 255u)) & 31u)))))].data[4].w;
                                vec3 _690 = (vec3(float((_663 >> 0u) & 255u), float((_665 >> 0u) & 255u), float((_667 >> 0u) & 255u)) * _432) + _434;
                                vec3 _692 = (vec3(float((_664 >> 0u) & 255u), float((_666 >> 0u) & 255u), float((_668 >> 0u) & 255u)) * _432) + _434;
                                uint _715;
                                if (spvNMax(_690.x, spvNMax(_690.y, spvNMax(_690.z, 9.9999997473787516355514526367188e-05))) <= spvNMin(_692.x, spvNMin(_692.y, spvNMin(_692.z, _364))))
                                {
                                    _715 = _645 | (((_656 >> 0u) & 255u) << (((_654 >> 0u) & 255u) & 31u));
                                }
                                else
                                {
                                    _715 = _645;
                                }
                                vec3 _737 = (vec3(float((_663 >> 8u) & 255u), float((_665 >> 8u) & 255u), float((_667 >> 8u) & 255u)) * _432) + _434;
                                vec3 _739 = (vec3(float((_664 >> 8u) & 255u), float((_666 >> 8u) & 255u), float((_668 >> 8u) & 255u)) * _432) + _434;
                                uint _762;
                                if (spvNMax(_737.x, spvNMax(_737.y, spvNMax(_737.z, 9.9999997473787516355514526367188e-05))) <= spvNMin(_739.x, spvNMin(_739.y, spvNMin(_739.z, _364))))
                                {
                                    _762 = _715 | (((_656 >> 8u) & 255u) << (((_654 >> 8u) & 255u) & 31u));
                                }
                                else
                                {
                                    _762 = _715;
                                }
                                vec3 _784 = (vec3(float((_663 >> 16u) & 255u), float((_665 >> 16u) & 255u), float((_667 >> 16u) & 255u)) * _432) + _434;
                                vec3 _786 = (vec3(float((_664 >> 16u) & 255u), float((_666 >> 16u) & 255u), float((_668 >> 16u) & 255u)) * _432) + _434;
                                uint _809;
                                if (spvNMax(_784.x, spvNMax(_784.y, spvNMax(_784.z, 9.9999997473787516355514526367188e-05))) <= spvNMin(_786.x, spvNMin(_786.y, spvNMin(_786.z, _364))))
                                {
                                    _809 = _762 | (((_656 >> 16u) & 255u) << (((_654 >> 16u) & 255u) & 31u));
                                }
                                else
                                {
                                    _809 = _762;
                                }
                                vec3 _831 = (vec3(float((_663 >> 24u) & 255u), float((_665 >> 24u) & 255u), float((_667 >> 24u) & 255u)) * _432) + _434;
                                vec3 _833 = (vec3(float((_664 >> 24u) & 255u), float((_666 >> 24u) & 255u), float((_668 >> 24u) & 255u)) * _432) + _434;
                                uint _856;
                                if (spvNMax(_831.x, spvNMax(_831.y, spvNMax(_831.z, 9.9999997473787516355514526367188e-05))) <= spvNMin(_833.x, spvNMin(_833.y, spvNMin(_833.z, _364))))
                                {
                                    _856 = _809 | (((_656 >> 24u) & 255u) << (((_654 >> 24u) & 255u) & 31u));
                                }
                                else
                                {
                                    _856 = _809;
                                }
                                uvec2 _860 = uvec2(rt_bl_bvh._m0[_361.x + uint(bitCount(_361.y & (~(4294967295u << (((_383 - 24u) ^ (_359 & 255u)) & 31u)))))].data[1].x, _388);
                                _860.y = (_856 & 4278190080u) | ((rt_bl_bvh._m0[_361.x + uint(bitCount(_361.y & (~(4294967295u << (((_383 - 24u) ^ (_359 & 255u)) & 31u)))))].data[0].w >> 24u) & 255u);
                                _867 = _860;
                                _868 = uvec2(rt_bl_bvh._m0[_361.x + uint(bitCount(_361.y & (~(4294967295u << (((_383 - 24u) ^ (_359 & 255u)) & 31u)))))].data[1].y, _856 & 16777215u);
                                _869 = _396;
                            }
                            else
                            {
                                _867 = uvec2(0u);
                                _868 = _361;
                                _869 = _366;
                            }
                            uvec2 _871;
                            _871 = _868;
                            _371 = _370;
                            _365 = _364;
                            _373 = _372;
                            uvec2 _872;
                            vec3 _874;
                            float _875;
                            int _876;
                            uint _877;
                            for (;;)
                            {
                                _877 = _871.y;
                                if (_877 != 0u)
                                {
                                    uint _881 = uint(findMSB(_877));
                                    _872 = _871;
                                    _872.y = _877 & (~(1u << (_881 & 31u)));
                                    uint _887 = _871.x + _881;
                                    vec3 _920 = vec3(unpackHalf2x16((rt_triangles._m0[_887].e[0] >> 16u) & 65535u).x, unpackHalf2x16((rt_triangles._m0[_887].e[1] >> 16u) & 65535u).x, unpackHalf2x16((rt_triangles._m0[_887].e[2] >> 16u) & 65535u).x);
                                    vec3 _921 = vec3(unpackHalf2x16(rt_triangles._m0[_887].e[0] & 65535u).x, unpackHalf2x16(rt_triangles._m0[_887].e[1] & 65535u).x, unpackHalf2x16(rt_triangles._m0[_887].e[2] & 65535u).x);
                                    bool _955;
                                    do
                                    {
                                        vec3 _924 = cross(_315, _921);
                                        float _925 = dot(_920, _924);
                                        vec3 _926 = _319 - vec3(rt_triangles._m0[_887].v[0], rt_triangles._m0[_887].v[1], rt_triangles._m0[_887].v[2]);
                                        vec3 _927 = cross(_926, _920);
                                        float _929 = dot(_921, _927) / _925;
                                        bool _934;
                                        if (_929 >= 0.0)
                                        {
                                            _934 = _929 < _365;
                                        }
                                        else
                                        {
                                            _934 = false;
                                        }
                                        if (_934 ? true : false)
                                        {
                                            vec3 _939;
                                            _939.x = dot(_926, _924);
                                            _939.y = dot(_315, _927);
                                            vec2 _944 = _939.xy / vec2(_925);
                                            vec3 _945 = vec3(_944.x, _944.y, _939.z);
                                            _945.z = (1.0 - _944.x) - _944.y;
                                            if (all(greaterThanEqual(_945, vec3(-9.9999997473787516355514526367188e-05))))
                                            {
                                                _875 = _929;
                                                _955 = true;
                                                break;
                                            }
                                        }
                                        _875 = _365;
                                        _955 = false;
                                        break;
                                    } while(false);
                                    if (_955)
                                    {
                                        _874 = cross(_920, _921);
                                        _876 = int(_887);
                                    }
                                    else
                                    {
                                        _874 = _371;
                                        _876 = _373;
                                    }
                                    _871 = _872;
                                    _371 = _874;
                                    _365 = _875;
                                    _373 = _876;
                                    continue;
                                }
                                else
                                {
                                    break;
                                }
                            }
                            if ((_867.y & 4278190080u) == 0u)
                            {
                                if (_869 == 0)
                                {
                                    _318 = _371;
                                    _971 = _365;
                                    _972 = _373;
                                    break;
                                }
                                int _968 = _869 - 1;
                                _362 = _222[_968];
                                _367 = _968;
                            }
                            else
                            {
                                _362 = _867;
                                _367 = _869;
                            }
                            _361 = _362;
                            _364 = _365;
                            _366 = _367;
                            _368++;
                            _370 = _371;
                            _372 = _373;
                            continue;
                        }
                        else
                        {
                            _318 = _370;
                            _971 = _364;
                            _972 = _372;
                            break;
                        }
                    }
                    bool _977;
                    if (_972 != (-1))
                    {
                        _977 = _971 < 100000.0;
                    }
                    else
                    {
                        _977 = false;
                    }
                    if (_977)
                    {
                        _980 = _318;
                        _981 = 3u;
                        _982 = _971;
                        break;
                    }
                    _980 = vec3(0.0, 1.0, 0.0);
                    _981 = 0u;
                    _982 = 100000.0;
                    break;
                } while(false);
                uint _1097;
                float _1098;
                do
                {
                    vec3 _986 = (vec3(65.0, 24.0, 65.0) - _319) / _315;
                    vec3 _988 = (vec3(-65.0, 0.0, -65.0) - _319) / _315;
                    vec3 _989 = spvNMin(_986, _988);
                    vec3 _990 = spvNMax(_986, _988);
                    float _995 = spvNMax(spvNMax(_989.x, _989.y), _989.z);
                    float _1000 = spvNMin(spvNMin(_990.x, _990.y), _990.z);
                    bool _1005;
                    if (_995 <= _1000)
                    {
                        _1005 = _1000 >= 0.0;
                    }
                    else
                    {
                        _1005 = false;
                    }
                    if (!_1005)
                    {
                        _1097 = _981;
                        _1098 = _982;
                        break;
                    }
                    float _1009 = spvNMax(_995, 0.0);
                    float _1011 = (_1000 - _1009) * 0.03125;
                    float _1013;
                    _1013 = _1009;
                    float _1014;
                    uint _1017;
                    float _1036;
                    uint _1016 = 0u;
                    bool _1018;
                    for (;;)
                    {
                        _1018 = _1016 < 32u;
                        if (_1018)
                        {
                            _1014 = _1013 + _1011;
                            vec3 _1022 = _319 + (_315 * _1014);
                            if (_1022.y < mix(-2.5, 24.0, textureLod(sampler2D(visual_terrain_heightmap, sampler_lnc), (_1022.xz * vec2(0.0076923076994717121124267578125)) + vec2(0.5), 0.0).x))
                            {
                                _1036 = _1014;
                                break;
                            }
                            _1017 = _1016 + 1u;
                            _1013 = _1014;
                            _1016 = _1017;
                            continue;
                        }
                        else
                        {
                            _1036 = _1000;
                            break;
                        }
                    }
                    bool _1040;
                    if (_1018)
                    {
                        _1040 = _1013 < _982;
                    }
                    else
                    {
                        _1040 = false;
                    }
                    bool _1044;
                    if (_1040)
                    {
                        _1044 = _1013 < _1036;
                    }
                    else
                    {
                        _1044 = false;
                    }
                    if (_1044)
                    {
                        float _1052;
                        _1052 = _1013;
                        float _1049 = _1036;
                        uint _1054 = 0u;
                        for (; _1054 < 8u; )
                        {
                            float _1058 = _1052 + _1049;
                            float _1059 = _1058 * 0.5;
                            vec3 _1061 = _319 + (_315 * _1059);
                            bool _1072 = _1061.y < mix(-2.5, 24.0, textureLod(sampler2D(visual_terrain_heightmap, sampler_lnc), (_1061.xz * vec2(0.0076923076994717121124267578125)) + vec2(0.5), 0.0).x);
                            _1049 = _1072 ? _1059 : _1049;
                            _1052 = _1072 ? _1052 : _1059;
                            _1054++;
                            continue;
                        }
                        if (_1052 < _982)
                        {
                            vec2 _1081 = ((_319 + (_315 * _1052)).xz * vec2(0.0076923076994717121124267578125)) + vec2(0.5);
                            _1097 = ((mix(-2.5, 24.0, textureLod(sampler2D(terrain_heightmap, sampler_default), _1081, 0.0).x) + 0.100000001490116119384765625) > mix(-2.5, 24.0, textureLod(sampler2D(visual_terrain_heightmap, sampler_lnc), _1081, 0.0).x)) ? 1u : 2u;
                            _1098 = _1052;
                            break;
                        }
                        else
                        {
                            _1097 = _981;
                            _1098 = _982;
                            break;
                        }
                        break; // unreachable workaround
                    }
                    else
                    {
                        _1097 = _981;
                        _1098 = _982;
                        break;
                    }
                    break; // unreachable workaround
                } while(false);
                if (_1097 == 0u)
                {
                    vec4 _1184;
                    if (_349 > 0.0)
                    {
                        _1184 = vec4(_219.x, _219.y, _219.z, _312.w);
                    }
                    else
                    {
                        vec4 _1182;
                        if (frame_constants.glade_theme_id == 2u)
                        {
                            _1182 = vec4(_219.x, _219.y, _219.z, _312.w);
                        }
                        else
                        {
                            _1182 = _312;
                        }
                        _1184 = _1182;
                    }
                    _1185 = _1184;
                    break;
                }
                else
                {
                    vec4 _1107;
                    if (0u == _321)
                    {
                        vec4 _1106;
                        _1106.w = _1098;
                        _1107 = _1106;
                    }
                    else
                    {
                        _1107 = _312;
                    }
                    vec3 _1109 = _319 + (_315 * _1098);
                    bool _1115;
                    if (!(_1097 == 1u))
                    {
                        _1115 = _1097 == 2u;
                    }
                    else
                    {
                        _1115 = true;
                    }
                    if (_1115)
                    {
                        if ((_321 + 1u) == 1u)
                        {
                            _1185 = vec4(_219.x, _219.y, _219.z, _1107.w);
                            break;
                        }
                        float _1157 = 1.0 - (2.0 * _343.x);
                        float _1161 = sqrt(spvNMax(0.0, 1.0 - (_1157 * _1157)));
                        float _1163 = 6.283185482025146484375 * _343.y;
                        _313 = vec4(_219.x, _219.y, _219.z, _1107.w);
                        _316 = normalize(vec3(0.0, 1.0, 0.0) + vec3(cos(_1163) * _1161, sin(_1163) * _1161, _1157));
                        _320 = _1109 + vec3(0.0, 0.001000000047497451305389404296875, 0.0);
                    }
                    else
                    {
                        vec4 _1146;
                        vec3 _1147;
                        vec3 _1148;
                        if (_1097 == 3u)
                        {
                            if (dot(_980, _315) > 0.0)
                            {
                                _1185 = _1107;
                                break;
                            }
                            vec3 _1126 = normalize(_980);
                            float _1132 = 1.0 - (2.0 * _343.x);
                            float _1136 = sqrt(spvNMax(0.0, 1.0 - (_1132 * _1132)));
                            float _1138 = 6.283185482025146484375 * _343.y;
                            _1146 = vec4(_219.x, _219.y, _219.z, _1107.w);
                            _1147 = normalize(_1126 + vec3(cos(_1138) * _1136, sin(_1138) * _1136, _1132));
                            _1148 = _1109 + (_1126 * 0.001000000047497451305389404296875);
                        }
                        else
                        {
                            _1146 = _1107;
                            _1147 = _315;
                            _1148 = _319;
                        }
                        _313 = _1146;
                        _316 = _1147;
                        _320 = _1148;
                    }
                }
                _312 = _313;
                _315 = _316;
                _317 = _318;
                _319 = _320;
                _321++;
                continue;
            }
            else
            {
                _1185 = _312;
                break;
            }
        }
        if ((1u & _269) != 0u)
        {
            uvec2 _1190 = _238 * uvec2(2u);
            vec4 _1203 = (vec4(_310 + (_308 * _1185.w), 1.0) * frame_constants.view_constants[0u].world_to_view) * frame_constants.view_constants[0u].view_to_sample;
            vec3 _1207 = _1203.xyz / vec3(_1203.w);
            vec2 _1210 = (_1207.xy * vec2(0.5, -0.5)) + vec2(0.5);
            vec4 _1214 = textureLod(sampler2D(reprojection_tex, sampler_nnc), _1210, 0.0);
            bool _1222;
            if (all(lessThan(abs(_1207), vec3(1.0))))
            {
                _1222 = _1207.z > 0.0;
            }
            else
            {
                _1222 = false;
            }
            bool _1234;
            if (_1222)
            {
                _1234 = textureLod(sampler2D(half_depth_tex, sampler_nnc), _1214.xy + _1210, 0.0).x < (_1207.z * 1.10000002384185791015625);
            }
            else
            {
                _1234 = false;
            }
            vec3 _1259;
            float _1260;
            if (_1234)
            {
                bool _1249;
                if (_Globals.history_valid != 0u)
                {
                    _1249 = !(0u == frame_constants.frame_index);
                }
                else
                {
                    _1249 = false;
                }
                vec3 _1258;
                if (_1249)
                {
                    _1258 = textureLod(sampler2D(taa_history_tex, sampler_nnc), _1214.xy + _1210, 0.0).xyz;
                }
                else
                {
                    _1258 = vec3(0.0);
                }
                _1259 = _1258;
                _1260 = 0.0;
            }
            else
            {
                _1259 = vec3(0.0);
                _1260 = float(_1185.w > 10000.0);
            }
            vec3 _1286;
            if (_1260 > 0.0)
            {
                vec3 _1264 = spvNMax(vec3(0.0), vec3(0.0));
                _1286 = _1259 + (((mix(frame_constants.sky_low_color.xyz * 1.0, frame_constants.sky_high_color.xyz, vec3(pow(clamp(1.0 - pow(1.0 - clamp(_308.y + 0.20000000298023223876953125, 0.0, 1.0), 14.0), 0.0, 1.0), 0.64999997615814208984375))) * 1.0) + _1264) * _1260);
            }
            else
            {
                _1286 = _1259;
            }
            float _1288 = 1.0 - dot(_264, _297);
            float _1289 = _1288 * _1288;
            vec3 _1294 = _1286 * (0.039999999105930328369140625 + (0.959999978542327880859375 * ((_1289 * _1289) * _1288)));
            bool _1310;
            if (_Globals.history_valid != 0u)
            {
                _1310 = !(0u == frame_constants.frame_index);
            }
            else
            {
                _1310 = false;
            }
            bool _1314;
            if (_1310)
            {
                _1314 = _1260 < 1.0;
            }
            else
            {
                _1314 = false;
            }
            if (_1314 ? all(equal(vec4(1.0), textureGatherOffset(sampler2D(reprojection_tex, sampler_lnc), _245, ivec2(0), 2))) : false)
            {
                vec3 _1329 = textureLod(sampler2D(taa_history_tex, sampler_lnc), _245 + texelFetch(reprojection_tex, ivec2(_1190), int(0u)).xy, 0.0).xyz;
                vec4 _1331 = imageLoad(color_tex, ivec2(_1190));
                vec3 _1333 = _1331.xyz + _1294;
                float _1334 = dot(_1329, vec3(0.2125999927520751953125, 0.715200006961822509765625, 0.072200000286102294921875));
                float _1335 = dot(_1333, vec3(0.2125999927520751953125, 0.715200006961822509765625, 0.072200000286102294921875));
                float _1338 = clamp(_1335, 0.4000000059604644775390625 * _1334, 2.5 * _1334);
                if (_1335 > _1334)
                {
                    imageStore(color_tex, ivec2(_1190), (_1333 * (_1338 / spvNMax(9.9999999392252902907785028219223e-09, _1335))).xyzz);
                }
                else
                {
                    imageStore(color_tex, ivec2(_1190), (_1329 * (_1338 / spvNMax(9.9999999392252902907785028219223e-09, _1334))).xyzz);
                }
            }
            else
            {
                imageStore(color_tex, ivec2(_1190), (imageLoad(color_tex, ivec2(_1190)).xyz + _1294).xyzz);
            }
        }
        if ((2u & _269) != 0u)
        {
            uvec2 _1356 = (_238 * uvec2(2u)) + uvec2(1u, 0u);
            vec4 _1369 = (vec4(_310 + (_308 * _1185.w), 1.0) * frame_constants.view_constants[0u].world_to_view) * frame_constants.view_constants[0u].view_to_sample;
            vec3 _1373 = _1369.xyz / vec3(_1369.w);
            vec2 _1376 = (_1373.xy * vec2(0.5, -0.5)) + vec2(0.5);
            vec4 _1380 = textureLod(sampler2D(reprojection_tex, sampler_nnc), _1376, 0.0);
            bool _1388;
            if (all(lessThan(abs(_1373), vec3(1.0))))
            {
                _1388 = _1373.z > 0.0;
            }
            else
            {
                _1388 = false;
            }
            bool _1400;
            if (_1388)
            {
                _1400 = textureLod(sampler2D(half_depth_tex, sampler_nnc), _1380.xy + _1376, 0.0).x < (_1373.z * 1.10000002384185791015625);
            }
            else
            {
                _1400 = false;
            }
            vec3 _1425;
            float _1426;
            if (_1400)
            {
                bool _1415;
                if (_Globals.history_valid != 0u)
                {
                    _1415 = !(0u == frame_constants.frame_index);
                }
                else
                {
                    _1415 = false;
                }
                vec3 _1424;
                if (_1415)
                {
                    _1424 = textureLod(sampler2D(taa_history_tex, sampler_nnc), _1380.xy + _1376, 0.0).xyz;
                }
                else
                {
                    _1424 = vec3(0.0);
                }
                _1425 = _1424;
                _1426 = 0.0;
            }
            else
            {
                _1425 = vec3(0.0);
                _1426 = float(_1185.w > 10000.0);
            }
            vec3 _1452;
            if (_1426 > 0.0)
            {
                vec3 _1430 = spvNMax(vec3(0.0), vec3(0.0));
                _1452 = _1425 + (((mix(frame_constants.sky_low_color.xyz * 1.0, frame_constants.sky_high_color.xyz, vec3(pow(clamp(1.0 - pow(1.0 - clamp(_308.y + 0.20000000298023223876953125, 0.0, 1.0), 14.0), 0.0, 1.0), 0.64999997615814208984375))) * 1.0) + _1430) * _1426);
            }
            else
            {
                _1452 = _1425;
            }
            float _1454 = 1.0 - dot(_264, _297);
            float _1455 = _1454 * _1454;
            vec3 _1460 = _1452 * (0.039999999105930328369140625 + (0.959999978542327880859375 * ((_1455 * _1455) * _1454)));
            bool _1476;
            if (_Globals.history_valid != 0u)
            {
                _1476 = !(0u == frame_constants.frame_index);
            }
            else
            {
                _1476 = false;
            }
            bool _1480;
            if (_1476)
            {
                _1480 = _1426 < 1.0;
            }
            else
            {
                _1480 = false;
            }
            if (_1480 ? all(equal(vec4(1.0), textureGatherOffset(sampler2D(reprojection_tex, sampler_lnc), _245, ivec2(0), 2))) : false)
            {
                vec3 _1495 = textureLod(sampler2D(taa_history_tex, sampler_lnc), _245 + texelFetch(reprojection_tex, ivec2(_1356), int(0u)).xy, 0.0).xyz;
                vec4 _1497 = imageLoad(color_tex, ivec2(_1356));
                vec3 _1499 = _1497.xyz + _1460;
                float _1500 = dot(_1495, vec3(0.2125999927520751953125, 0.715200006961822509765625, 0.072200000286102294921875));
                float _1501 = dot(_1499, vec3(0.2125999927520751953125, 0.715200006961822509765625, 0.072200000286102294921875));
                float _1504 = clamp(_1501, 0.4000000059604644775390625 * _1500, 2.5 * _1500);
                if (_1501 > _1500)
                {
                    imageStore(color_tex, ivec2(_1356), (_1499 * (_1504 / spvNMax(9.9999999392252902907785028219223e-09, _1501))).xyzz);
                }
                else
                {
                    imageStore(color_tex, ivec2(_1356), (_1495 * (_1504 / spvNMax(9.9999999392252902907785028219223e-09, _1500))).xyzz);
                }
            }
            else
            {
                imageStore(color_tex, ivec2(_1356), (imageLoad(color_tex, ivec2(_1356)).xyz + _1460).xyzz);
            }
        }
        if ((4u & _269) != 0u)
        {
            uvec2 _1522 = (_238 * uvec2(2u)) + uvec2(0u, 1u);
            vec4 _1535 = (vec4(_310 + (_308 * _1185.w), 1.0) * frame_constants.view_constants[0u].world_to_view) * frame_constants.view_constants[0u].view_to_sample;
            vec3 _1539 = _1535.xyz / vec3(_1535.w);
            vec2 _1542 = (_1539.xy * vec2(0.5, -0.5)) + vec2(0.5);
            vec4 _1546 = textureLod(sampler2D(reprojection_tex, sampler_nnc), _1542, 0.0);
            bool _1554;
            if (all(lessThan(abs(_1539), vec3(1.0))))
            {
                _1554 = _1539.z > 0.0;
            }
            else
            {
                _1554 = false;
            }
            bool _1566;
            if (_1554)
            {
                _1566 = textureLod(sampler2D(half_depth_tex, sampler_nnc), _1546.xy + _1542, 0.0).x < (_1539.z * 1.10000002384185791015625);
            }
            else
            {
                _1566 = false;
            }
            vec3 _1591;
            float _1592;
            if (_1566)
            {
                bool _1581;
                if (_Globals.history_valid != 0u)
                {
                    _1581 = !(0u == frame_constants.frame_index);
                }
                else
                {
                    _1581 = false;
                }
                vec3 _1590;
                if (_1581)
                {
                    _1590 = textureLod(sampler2D(taa_history_tex, sampler_nnc), _1546.xy + _1542, 0.0).xyz;
                }
                else
                {
                    _1590 = vec3(0.0);
                }
                _1591 = _1590;
                _1592 = 0.0;
            }
            else
            {
                _1591 = vec3(0.0);
                _1592 = float(_1185.w > 10000.0);
            }
            vec3 _1618;
            if (_1592 > 0.0)
            {
                vec3 _1596 = spvNMax(vec3(0.0), vec3(0.0));
                _1618 = _1591 + (((mix(frame_constants.sky_low_color.xyz * 1.0, frame_constants.sky_high_color.xyz, vec3(pow(clamp(1.0 - pow(1.0 - clamp(_308.y + 0.20000000298023223876953125, 0.0, 1.0), 14.0), 0.0, 1.0), 0.64999997615814208984375))) * 1.0) + _1596) * _1592);
            }
            else
            {
                _1618 = _1591;
            }
            float _1620 = 1.0 - dot(_264, _297);
            float _1621 = _1620 * _1620;
            vec3 _1626 = _1618 * (0.039999999105930328369140625 + (0.959999978542327880859375 * ((_1621 * _1621) * _1620)));
            bool _1642;
            if (_Globals.history_valid != 0u)
            {
                _1642 = !(0u == frame_constants.frame_index);
            }
            else
            {
                _1642 = false;
            }
            bool _1646;
            if (_1642)
            {
                _1646 = _1592 < 1.0;
            }
            else
            {
                _1646 = false;
            }
            if (_1646 ? all(equal(vec4(1.0), textureGatherOffset(sampler2D(reprojection_tex, sampler_lnc), _245, ivec2(0), 2))) : false)
            {
                vec3 _1661 = textureLod(sampler2D(taa_history_tex, sampler_lnc), _245 + texelFetch(reprojection_tex, ivec2(_1522), int(0u)).xy, 0.0).xyz;
                vec4 _1663 = imageLoad(color_tex, ivec2(_1522));
                vec3 _1665 = _1663.xyz + _1626;
                float _1666 = dot(_1661, vec3(0.2125999927520751953125, 0.715200006961822509765625, 0.072200000286102294921875));
                float _1667 = dot(_1665, vec3(0.2125999927520751953125, 0.715200006961822509765625, 0.072200000286102294921875));
                float _1670 = clamp(_1667, 0.4000000059604644775390625 * _1666, 2.5 * _1666);
                if (_1667 > _1666)
                {
                    imageStore(color_tex, ivec2(_1522), (_1665 * (_1670 / spvNMax(9.9999999392252902907785028219223e-09, _1667))).xyzz);
                }
                else
                {
                    imageStore(color_tex, ivec2(_1522), (_1661 * (_1670 / spvNMax(9.9999999392252902907785028219223e-09, _1666))).xyzz);
                }
            }
            else
            {
                imageStore(color_tex, ivec2(_1522), (imageLoad(color_tex, ivec2(_1522)).xyz + _1626).xyzz);
            }
        }
        if ((8u & _269) != 0u)
        {
            uvec2 _1688 = (_238 * uvec2(2u)) + uvec2(1u);
            vec4 _1701 = (vec4(_310 + (_308 * _1185.w), 1.0) * frame_constants.view_constants[0u].world_to_view) * frame_constants.view_constants[0u].view_to_sample;
            vec3 _1705 = _1701.xyz / vec3(_1701.w);
            vec2 _1708 = (_1705.xy * vec2(0.5, -0.5)) + vec2(0.5);
            vec4 _1712 = textureLod(sampler2D(reprojection_tex, sampler_nnc), _1708, 0.0);
            bool _1720;
            if (all(lessThan(abs(_1705), vec3(1.0))))
            {
                _1720 = _1705.z > 0.0;
            }
            else
            {
                _1720 = false;
            }
            bool _1732;
            if (_1720)
            {
                _1732 = textureLod(sampler2D(half_depth_tex, sampler_nnc), _1712.xy + _1708, 0.0).x < (_1705.z * 1.10000002384185791015625);
            }
            else
            {
                _1732 = false;
            }
            vec3 _1757;
            float _1758;
            if (_1732)
            {
                bool _1747;
                if (_Globals.history_valid != 0u)
                {
                    _1747 = !(0u == frame_constants.frame_index);
                }
                else
                {
                    _1747 = false;
                }
                vec3 _1756;
                if (_1747)
                {
                    _1756 = textureLod(sampler2D(taa_history_tex, sampler_nnc), _1712.xy + _1708, 0.0).xyz;
                }
                else
                {
                    _1756 = vec3(0.0);
                }
                _1757 = _1756;
                _1758 = 0.0;
            }
            else
            {
                _1757 = vec3(0.0);
                _1758 = float(_1185.w > 10000.0);
            }
            vec3 _1784;
            if (_1758 > 0.0)
            {
                vec3 _1762 = spvNMax(vec3(0.0), vec3(0.0));
                _1784 = _1757 + (((mix(frame_constants.sky_low_color.xyz * 1.0, frame_constants.sky_high_color.xyz, vec3(pow(clamp(1.0 - pow(1.0 - clamp(_308.y + 0.20000000298023223876953125, 0.0, 1.0), 14.0), 0.0, 1.0), 0.64999997615814208984375))) * 1.0) + _1762) * _1758);
            }
            else
            {
                _1784 = _1757;
            }
            float _1786 = 1.0 - dot(_264, _297);
            float _1787 = _1786 * _1786;
            vec3 _1792 = _1784 * (0.039999999105930328369140625 + (0.959999978542327880859375 * ((_1787 * _1787) * _1786)));
            bool _1808;
            if (_Globals.history_valid != 0u)
            {
                _1808 = !(0u == frame_constants.frame_index);
            }
            else
            {
                _1808 = false;
            }
            bool _1812;
            if (_1808)
            {
                _1812 = _1758 < 1.0;
            }
            else
            {
                _1812 = false;
            }
            if (_1812 ? all(equal(vec4(1.0), textureGatherOffset(sampler2D(reprojection_tex, sampler_lnc), _245, ivec2(0), 2))) : false)
            {
                vec3 _1827 = textureLod(sampler2D(taa_history_tex, sampler_lnc), _245 + texelFetch(reprojection_tex, ivec2(_1688), int(0u)).xy, 0.0).xyz;
                vec4 _1829 = imageLoad(color_tex, ivec2(_1688));
                vec3 _1831 = _1829.xyz + _1792;
                float _1832 = dot(_1827, vec3(0.2125999927520751953125, 0.715200006961822509765625, 0.072200000286102294921875));
                float _1833 = dot(_1831, vec3(0.2125999927520751953125, 0.715200006961822509765625, 0.072200000286102294921875));
                float _1836 = clamp(_1833, 0.4000000059604644775390625 * _1832, 2.5 * _1832);
                if (_1833 > _1832)
                {
                    imageStore(color_tex, ivec2(_1688), (_1831 * (_1836 / spvNMax(9.9999999392252902907785028219223e-09, _1833))).xyzz);
                }
                else
                {
                    imageStore(color_tex, ivec2(_1688), (_1827 * (_1836 / spvNMax(9.9999999392252902907785028219223e-09, _1832))).xyzz);
                }
            }
            else
            {
                imageStore(color_tex, ivec2(_1688), (imageLoad(color_tex, ivec2(_1688)).xyz + _1792).xyzz);
            }
        }
        break;
    } while(false);
}

