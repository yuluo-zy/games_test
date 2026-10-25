#version 450
#extension GL_EXT_nonuniform_qualifier : require
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

struct RoofShingle
{
    vec3 base_position;
    float current_height;
    uvec2 clipping_plane_;
    uvec2 scale_t;
    uvec2 rotation_quat_;
    uint roof_id;
    uint velocity_rotation;
    float animation_t;
    int seed;
    int is_circle;
    int is_main_roof_body;
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

layout(set = 0, binding = 23, std430) readonly buffer type_StructuredBuffer_RoofShingle
{
    RoofShingle _m0[];
} instance_buffer;

layout(push_constant, std430) uniform type_ConstantBuffer_PushConstants
{
    uint world_subset;
    uint first_instance;
    uint instance_count;
    uint draw_list_index_base;
} push_constants;

void main()
{
    do
    {
        if (gl_GlobalInvocationID.x >= push_constants.instance_count)
        {
            break;
        }
        uint _130 = push_constants.first_instance + gl_GlobalInvocationID.x;
        vec3 _132 = instance_buffer._m0[_130].base_position;
        float _134 = instance_buffer._m0[_130].current_height;
        uvec2 _136 = instance_buffer._m0[_130].scale_t;
        int _138 = instance_buffer._m0[_130].is_main_roof_body;
        float _154 = length(vec3(unpackHalf2x16(_136.x & 65535u).x, unpackHalf2x16((_136.x >> 16u) & 65535u).x, unpackHalf2x16(_136.y & 65535u).x));
        float _155 = 0.59403002262115478515625 * _154;
        vec4 _161 = vec4(_132 + vec3(0.0, _134, 0.0), 1.0);
        vec4 _162 = _161 * frame_constants.view_constants[0u].world_to_view;
        float _163 = _162.z;
        float _164 = -_163;
        uint _176 = clamp(uint(clamp((32.0 - _163) * 0.00347222224809229373931884765625, 0.0, 1.0) * 1024.0), 0u, 1023u);
        uint _177 = _130 * 1024u;
        vec4 _185 = vec4(-frame_constants.view_constants[0u].view_to_clip[0].x, frame_constants.view_constants[0u].view_to_clip[0].y, 0.0, frame_constants.view_constants[0u].view_to_clip[0].w);
        vec4 _189 = vec4(frame_constants.view_constants[0u].view_to_clip[1].x, frame_constants.view_constants[0u].view_to_clip[1].y, 0.0, frame_constants.view_constants[0u].view_to_clip[1].w);
        vec4 _193 = vec4(frame_constants.view_constants[0u].view_to_clip[3].x, frame_constants.view_constants[0u].view_to_clip[3].y, -1.0, frame_constants.view_constants[0u].view_to_clip[3].w);
        vec4 _118[4];
        _118[0] = _193 + _185;
        _118[1] = _193 - _185;
        _118[2] = _193 + _189;
        _118[3] = _193 - _189;
        for (int _204 = 0; _204 < 4; )
        {
            _118[_204] /= vec4(length(_118[_204].xyz));
            _204++;
            continue;
        }
        vec4 _117[4] = _118;
        bool _225;
        if ((_164 + _155) > frame_constants.view_constants[0u].view_to_clip[2].w)
        {
            _225 = (_164 - _155) < 3.4028234663852885981170418348452e+38;
        }
        else
        {
            _225 = false;
        }
        bool _227;
        _227 = _225;
        bool _228;
        for (int _230 = 0; _230 < 4; _227 = _228, _230++)
        {
            if (_227)
            {
                _228 = dot(vec4(_162.xy, _163, 1.0), _117[_230]) > (_154 * (-0.59403002262115478515625));
            }
            else
            {
                _228 = false;
            }
        }
        uint _247 = _130 + draw_list_info._m0[push_constants.draw_list_index_base].global_instance_offset;
        bool _258;
        if (_227)
        {
            _258 = (prev_visibility_bitmask._m0[_247 / 32u] & (1u << ((_247 % 32u) & 31u))) != 0u;
        }
        else
        {
            _258 = false;
        }
        if (_258)
        {
            uint _266 = atomicOr(_alias_next_visibility_bitmask_1._m0[_247 / 32u], 1u << ((_247 % 32u) & 31u));
            uint _268 = atomicAdd(_alias_filtered_instance_allocator_buffer_1._m0[push_constants.draw_list_index_base], 1u);
            _alias_filtered_instance_buffer._m0[_268] = _177 + _176;
            _alias_filtered_instance_meta_buffer._m0[_268] = GpuFilteredInstanceMeta((_138 == 0) ? 2u : ((_138 == 2) ? 2u : uint(_164 > 70.0)));
            uint _275 = atomicAdd(_alias_output_histogram_1._m0[(push_constants.draw_list_index_base * 1024u) + _176], 1u);
        }
        vec4 _281 = (_161 * frame_constants.view_constants[2u].world_to_view) * frame_constants.view_constants[2u].view_to_clip;
        uint _289 = clamp(uint((_281.xyz / vec3(_281.w)).z * 1024.0), 0u, 1023u);
        uint _290 = _177 + _289;
        float _299 = _155 * (2.0 * frame_constants.view_constants[2u].view_to_sample[0].x);
        vec2 _300 = abs((_161 * frame_constants.view_constants[2u].world_to_sample).xy);
        uint _310 = (any(greaterThanEqual(_300, vec2(-_299))) ? all(lessThanEqual(_300, vec2(1.0 + _299))) : false) ? 0u : 3u;
        bool _319 = any(greaterThanEqual(_300, vec2(1.0 - _299))) ? all(lessThanEqual(_300, vec2(2.0 + _299))) : false;
        uint _323;
        if (_319)
        {
            _323 = min(_310, 1u);
        }
        else
        {
            _323 = _310;
        }
        uint _324 = uint(_319);
        uint _338;
        uint _339;
        if (any(greaterThanEqual(_300, vec2(2.0 - _299))) ? all(lessThanEqual(_300, vec2(7.0 + _299))) : false)
        {
            _338 = max(_324, 2u);
            _339 = min(_323, 2u);
        }
        else
        {
            _338 = _324;
            _339 = _323;
        }
        for (uint _341 = _339; _341 <= _338; )
        {
            uint _347 = (push_constants.draw_list_index_base + 1u) + _341;
            uint _349 = atomicAdd(_alias_filtered_instance_allocator_buffer_1._m0[_347], 1u);
            _alias_filtered_instance_buffer._m0[_349] = _290;
            _alias_filtered_instance_meta_buffer._m0[_349] = GpuFilteredInstanceMeta(1u);
            uint _355 = atomicAdd(_alias_output_histogram_1._m0[(_347 * 1024u) + _289], 1u);
            _341++;
            continue;
        }
        break;
    } while(false);
}

