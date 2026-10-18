#version 450
#extension GL_EXT_nonuniform_qualifier : require
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

struct GpuFilteredInstanceMeta
{
    uint raw;
};

struct NaniDrawListInfo
{
    uint global_instance_offset;
};

struct Affine3Packed
{
    float data[12];
};

struct WoodenPlank
{
    Affine3Packed transform;
    int gate_seed;
    int piece_seed;
    uint wall_id;
    uint flags;
};

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

layout(set = 0, binding = 14, std430) buffer type_RWStructuredBuffer_uint
{
    uint _m0[];
} _alias_filtered_instance_buffer;

layout(set = 0, binding = 15, std430) buffer _alias_filtered_instance_allocator_buffer
{
    uint _m0[];
} _alias_filtered_instance_allocator_buffer_1;

layout(set = 0, binding = 16, std430) buffer type_RWStructuredBuffer_GpuFilteredInstanceMeta
{
    GpuFilteredInstanceMeta _m0[];
} _alias_filtered_instance_meta_buffer;

layout(set = 0, binding = 17, std430) buffer _alias_output_histogram
{
    uint _m0[];
} _alias_output_histogram_1;

layout(set = 0, binding = 18, std430) readonly buffer type_StructuredBuffer_uint
{
    uint _m0[];
} prev_visibility_bitmask;

layout(set = 0, binding = 19, std430) buffer _alias_next_visibility_bitmask
{
    uint _m0[];
} _alias_next_visibility_bitmask_1;

layout(set = 0, binding = 20, std430) readonly buffer type_StructuredBuffer_NaniDrawListInfo
{
    NaniDrawListInfo _m0[];
} draw_list_info;

layout(set = 0, binding = 23, std430) readonly buffer type_StructuredBuffer_WoodenPlank
{
    WoodenPlank _m0[];
} instance_buffer;

layout(push_constant, std430) uniform type_ConstantBuffer_PushConstants
{
    uint world_subset;
    uint first_instance;
    uint instance_count;
    uint draw_list_index_base;
} push_constants;

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
        if (gl_GlobalInvocationID.x >= push_constants.instance_count)
        {
            break;
        }
        uint _137 = push_constants.first_instance + gl_GlobalInvocationID.x;
        Affine3Packed _139 = instance_buffer._m0[_137].transform;
        mat3x4 _160 = transpose(mat4x3(vec3(_139.data[0], _139.data[1], _139.data[2]), vec3(_139.data[3], _139.data[4], _139.data[5]), vec3(_139.data[6], _139.data[7], _139.data[8]), vec3(_139.data[9], _139.data[10], _139.data[11])));
        vec3 _162 = _160[0].xyz;
        vec3 _165 = _160[1].xyz;
        vec3 _168 = _160[2].xyz;
        float _172 = sqrt(spvNMax(spvNMax(dot(_162, _162), dot(_165, _165)), dot(_168, _168)));
        float _176 = _172 * 0.5;
        mat3 _178 = transpose(mat3(_162, _165, _168));
        vec3 _179 = _178[0];
        float _180 = dot(_179, _179);
        vec3 _181 = _178[1];
        float _182 = dot(_181, _181);
        vec3 _183 = _178[2];
        float _184 = dot(_183, _183);
        vec4 _196 = vec4((vec4(0.0, 0.0, 0.0, 1.0) * _160) + (vec4(0.0, 0.5, 0.0, 0.0) * _160), 1.0);
        vec4 _197 = _196 * frame_constants.view_constants[0u].world_to_view;
        float _198 = _197.z;
        float _199 = -_198;
        float _200 = _199 - _176;
        float _211 = (0.5 * frame_constants.view_constants[0u].view_to_clip[1].y) * float(clamp(frame_constants.inner_rendering_resolution.y, 1080u, 2160u));
        uint _215 = uint((((sqrt(spvNMax(spvNMin(_180, _182), spvNMax(spvNMin(_182, _184), spvNMin(_180, _184)))) / spvNMax(9.9999997473787516355514526367188e-06, _200)) * _211) > 7.0) ? 0 : 1);
        bool _221;
        if ((instance_buffer._m0[_137].flags & 1u) != 0u)
        {
            _221 = _215 == 0u;
        }
        else
        {
            _221 = false;
        }
        uint _233 = clamp(uint(clamp((32.0 - _198) * 0.00347222224809229373931884765625, 0.0, 1.0) * 1024.0), 0u, 1023u);
        uint _234 = _137 * 1024u;
        vec4 _240 = vec4(-frame_constants.view_constants[0u].view_to_clip[0].x, frame_constants.view_constants[0u].view_to_clip[0].y, 0.0, frame_constants.view_constants[0u].view_to_clip[0].w);
        vec4 _243 = vec4(frame_constants.view_constants[0u].view_to_clip[1].x, frame_constants.view_constants[0u].view_to_clip[1].y, 0.0, frame_constants.view_constants[0u].view_to_clip[1].w);
        vec4 _247 = vec4(frame_constants.view_constants[0u].view_to_clip[3].x, frame_constants.view_constants[0u].view_to_clip[3].y, -1.0, frame_constants.view_constants[0u].view_to_clip[3].w);
        vec4 _125[4];
        _125[0] = _247 + _240;
        _125[1] = _247 - _240;
        _125[2] = _247 + _243;
        _125[3] = _247 - _243;
        for (int _258 = 0; _258 < 4; )
        {
            _125[_258] /= vec4(length(_125[_258].xyz));
            _258++;
            continue;
        }
        vec4 _124[4] = _125;
        bool _278;
        if ((_199 + _176) > frame_constants.view_constants[0u].view_to_clip[2].w)
        {
            _278 = _200 < 3.4028234663852885981170418348452e+38;
        }
        else
        {
            _278 = false;
        }
        bool _280;
        _280 = _278;
        bool _281;
        for (int _283 = 0; _283 < 4; _280 = _281, _283++)
        {
            if (_280)
            {
                _281 = dot(vec4(_197.xy, _198, 1.0), _124[_283]) > (_172 * (-0.5));
            }
            else
            {
                _281 = false;
            }
        }
        uint _300 = _137 + draw_list_info._m0[push_constants.draw_list_index_base].global_instance_offset;
        bool _311;
        if (_280)
        {
            _311 = (prev_visibility_bitmask._m0[_300 / 32u] & (1u << ((_300 % 32u) & 31u))) != 0u;
        }
        else
        {
            _311 = false;
        }
        if (_311)
        {
            uint _319 = atomicOr(_alias_next_visibility_bitmask_1._m0[_300 / 32u], 1u << ((_300 % 32u) & 31u));
            uint _321 = atomicAdd(_alias_filtered_instance_allocator_buffer_1._m0[push_constants.draw_list_index_base], 1u);
            _alias_filtered_instance_buffer._m0[_321] = _234 + _233;
            _alias_filtered_instance_meta_buffer._m0[_321] = GpuFilteredInstanceMeta(_221 ? 2u : _215);
            uint _328 = atomicAdd(_alias_output_histogram_1._m0[(push_constants.draw_list_index_base * 1024u) + _233], 1u);
        }
        if ((((_176 / spvNMax(9.9999997473787516355514526367188e-06, _199)) * _211) < 10.0) ? false : true)
        {
            vec4 _336 = (_196 * frame_constants.view_constants[2u].world_to_view) * frame_constants.view_constants[2u].view_to_clip;
            uint _344 = clamp(uint((_336.xyz / vec3(_336.w)).z * 1024.0), 0u, 1023u);
            uint _345 = _234 + _344;
            float _354 = _176 * (2.0 * frame_constants.view_constants[2u].view_to_sample[0].x);
            vec2 _355 = abs((_196 * frame_constants.view_constants[2u].world_to_sample).xy);
            uint _365 = (any(greaterThanEqual(_355, vec2(-_354))) ? all(lessThanEqual(_355, vec2(1.0 + _354))) : false) ? 0u : 3u;
            bool _374 = any(greaterThanEqual(_355, vec2(1.0 - _354))) ? all(lessThanEqual(_355, vec2(2.0 + _354))) : false;
            uint _378;
            if (_374)
            {
                _378 = min(_365, 1u);
            }
            else
            {
                _378 = _365;
            }
            uint _379 = uint(_374);
            uint _393;
            uint _394;
            if (any(greaterThanEqual(_355, vec2(2.0 - _354))) ? all(lessThanEqual(_355, vec2(7.0 + _354))) : false)
            {
                _393 = max(_379, 2u);
                _394 = min(_378, 2u);
            }
            else
            {
                _393 = _379;
                _394 = _378;
            }
            for (uint _396 = _394; _396 <= _393; )
            {
                uint _402 = (push_constants.draw_list_index_base + 1u) + _396;
                uint _404 = atomicAdd(_alias_filtered_instance_allocator_buffer_1._m0[_402], 1u);
                _alias_filtered_instance_buffer._m0[_404] = _345;
                _alias_filtered_instance_meta_buffer._m0[_404] = GpuFilteredInstanceMeta(1u);
                uint _410 = atomicAdd(_alias_output_histogram_1._m0[(_402 * 1024u) + _344], 1u);
                _396++;
                continue;
            }
        }
        break;
    } while(false);
}

