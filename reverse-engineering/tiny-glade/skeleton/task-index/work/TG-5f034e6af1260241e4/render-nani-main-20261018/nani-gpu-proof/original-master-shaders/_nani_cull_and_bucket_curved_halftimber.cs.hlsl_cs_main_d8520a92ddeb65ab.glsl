#version 450
#extension GL_EXT_buffer_reference2 : require
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

struct CurvedHalftimberInstanceData
{
    vec3 position;
    uint color_id;
    float radius0;
    float radius1;
    float angle0;
    float angle1;
    int is_straight;
    int is_xz;
    float plank_depth;
    int caps;
    int mesh_atlas_part_idx;
    int pad0;
    int pad1;
    int pad2;
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

layout(set = 0, binding = 23, std430) readonly buffer type_StructuredBuffer_CurvedHalftimberInstanceData
{
    CurvedHalftimberInstanceData _m0[];
} instance_buffer;

layout(push_constant, std430) uniform type_ConstantBuffer_PushConstants
{
    uvec2 instance_buffer_addr;
    uint world_subset;
    uint draw_list_index_base;
    uint input_instance_count;
} push_constants;

spirv_instruction(set = "GLSL.std.450", id = 80) float spvNMax(float, float);
spirv_instruction(set = "GLSL.std.450", id = 80) vec2 spvNMax(vec2, vec2);
spirv_instruction(set = "GLSL.std.450", id = 80) vec3 spvNMax(vec3, vec3);
spirv_instruction(set = "GLSL.std.450", id = 80) vec4 spvNMax(vec4, vec4);

void main()
{
    do
    {
        if (gl_GlobalInvocationID.x >= push_constants.input_instance_count)
        {
            break;
        }
        vec3 _142 = instance_buffer._m0[gl_GlobalInvocationID.x].position;
        float _144 = instance_buffer._m0[gl_GlobalInvocationID.x].radius1;
        float _146 = instance_buffer._m0[gl_GlobalInvocationID.x].angle1;
        int _148 = instance_buffer._m0[gl_GlobalInvocationID.x].is_straight;
        int _150 = instance_buffer._m0[gl_GlobalInvocationID.x].mesh_atlas_part_idx;
        uint _151 = uint(_150);
        float _153 = (_148 > 0) ? _146 : _144;
        vec4 _159 = vec4(_142, 1.0);
        vec4 _160 = _159 * frame_constants.view_constants[0u].world_to_view;
        float _161 = _160.z;
        mat4 _125 = frame_constants.view_constants[0u].view_to_clip;
        uint _183 = clamp(uint(clamp((32.0 - _161) * 0.00347222224809229373931884765625, 0.0, 1.0) * 1024.0), 0u, 1023u);
        uint _184 = gl_GlobalInvocationID.x * 1024u;
        mat4 _130 = frame_constants.view_constants[0u].view_to_clip;
        vec2 _128 = vec2(1.0);
        vec2 _129 = vec2(0.0);
        vec4 _198 = vec4((-_130[0u].x) * _128.x, _130[0u].y, _129.x, _130[0u].w);
        vec4 _210 = vec4(_130[1u].x, _130[1u].y * _128.y, _129.y, _130[1u].w);
        vec4 _217 = vec4(_130[3u].x, _130[3u].y, -1.0, _130[3u].w);
        vec4 _126[4];
        _126[0] = _217 + _198;
        _126[1] = _217 - _198;
        _126[2] = _217 + _210;
        _126[3] = _217 - _210;
        for (int _229 = 0; _229 < 4; )
        {
            _126[_229] /= vec4(length(_126[_229].xyz));
            _229++;
            continue;
        }
        vec4 _124[4] = _126;
        vec3 _131 = _160.xyz;
        bool _256;
        if (((-_131.z) + _153) > _130[2].w)
        {
            _256 = ((-_131.z) - _153) < 3.4028234663852885981170418348452e+38;
        }
        else
        {
            _256 = false;
        }
        bool _258;
        _258 = _256;
        bool _259;
        for (int _261 = 0; _261 < 4; _258 = _259, _261++)
        {
            if (_258)
            {
                _259 = dot(vec4(_160.xy, _161, 1.0), _124[_261]) > (-_153);
            }
            else
            {
                _259 = false;
            }
        }
        uint _278 = gl_GlobalInvocationID.x + draw_list_info._m0[push_constants.draw_list_index_base].global_instance_offset;
        bool _289;
        if (_258)
        {
            _289 = (prev_visibility_bitmask._m0[_278 / 32u] & (1u << ((_278 % 32u) & 31u))) != 0u;
        }
        else
        {
            _289 = false;
        }
        if (_289)
        {
            uint _297 = atomicOr(_alias_next_visibility_bitmask_1._m0[_278 / 32u], 1u << ((_278 % 32u) & 31u));
            uint _299 = atomicAdd(_alias_filtered_instance_allocator_buffer_1._m0[push_constants.draw_list_index_base], 1u);
            _alias_filtered_instance_buffer._m0[_299] = _184 + _183;
            _alias_filtered_instance_meta_buffer._m0[_299] = GpuFilteredInstanceMeta(_151);
            uint _306 = atomicAdd(_alias_output_histogram_1._m0[(push_constants.draw_list_index_base * 1024u) + _183], 1u);
        }
        if ((((_153 / spvNMax(9.9999997473787516355514526367188e-06, -_161)) * ((0.5 * _125[1u].y) * float(clamp(frame_constants.inner_rendering_resolution.y, 1080u, 2160u)))) < 10.0) ? false : true)
        {
            vec4 _314 = (_159 * frame_constants.view_constants[2u].world_to_view) * frame_constants.view_constants[2u].view_to_clip;
            vec4 _127 = _314;
            uint _323 = clamp(uint((_314.xyz / vec3(_127.w)).z * 1024.0), 0u, 1023u);
            uint _324 = _184 + _323;
            float _333 = _153 * (2.0 * frame_constants.view_constants[2u].view_to_sample[0].x);
            vec2 _334 = abs((_159 * frame_constants.view_constants[2u].world_to_sample).xy);
            uint _344 = (any(greaterThanEqual(_334, vec2(-_333))) ? all(lessThanEqual(_334, vec2(1.0 + _333))) : false) ? 0u : 3u;
            bool _353 = any(greaterThanEqual(_334, vec2(1.0 - _333))) ? all(lessThanEqual(_334, vec2(2.0 + _333))) : false;
            uint _357;
            if (_353)
            {
                _357 = min(_344, 1u);
            }
            else
            {
                _357 = _344;
            }
            uint _358 = uint(_353);
            uint _372;
            uint _373;
            if (any(greaterThanEqual(_334, vec2(2.0 - _333))) ? all(lessThanEqual(_334, vec2(7.0 + _333))) : false)
            {
                _372 = max(_358, 2u);
                _373 = min(_357, 2u);
            }
            else
            {
                _372 = _358;
                _373 = _357;
            }
            for (uint _375 = _373; _375 <= _372; )
            {
                uint _381 = (push_constants.draw_list_index_base + 1u) + _375;
                uint _383 = atomicAdd(_alias_filtered_instance_allocator_buffer_1._m0[_381], 1u);
                _alias_filtered_instance_buffer._m0[_383] = _324;
                _alias_filtered_instance_meta_buffer._m0[_383] = GpuFilteredInstanceMeta(_151);
                uint _390 = atomicAdd(_alias_output_histogram_1._m0[(_381 * 1024u) + _323], 1u);
                _375++;
                continue;
            }
        }
        break;
    } while(false);
}

