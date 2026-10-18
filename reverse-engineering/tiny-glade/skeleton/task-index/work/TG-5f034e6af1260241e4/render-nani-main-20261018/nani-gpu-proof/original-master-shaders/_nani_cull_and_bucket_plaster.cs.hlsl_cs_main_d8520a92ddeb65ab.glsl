#version 450
#extension GL_EXT_buffer_reference2 : require
#extension GL_EXT_nonuniform_qualifier : require
#extension GL_EXT_buffer_reference_uvec2 : require
layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

layout(buffer_reference) buffer uvec4Pointer;

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

struct PlasterInstanceData
{
    vec4 bounding_sphere;
    uint wall_id;
    float wall_length;
    uint blend_uv_seam;
    uint quad_count;
    uvec2 verts_gpu_address;
    int row_stride;
    float peel_strength;
};

layout(buffer_reference, buffer_reference_align = 16) buffer uvec4Pointer
{
    uvec4 value;
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

layout(set = 0, binding = 23, std430) readonly buffer type_StructuredBuffer_PlasterInstanceData
{
    PlasterInstanceData _m0[];
} instance_buffer;

layout(push_constant, std430) uniform type_ConstantBuffer_PushConstants
{
    uvec2 instance_buffer_addr;
    uint world_subset;
    uint draw_list_index_base;
    uint input_instance_count;
} push_constants;

void main()
{
    do
    {
        if (gl_GlobalInvocationID.x >= push_constants.input_instance_count)
        {
            break;
        }
        uvec2 _125 = push_constants.instance_buffer_addr;
        uvec2 _123;
        _123.x = _125.x + (16u * (gl_GlobalInvocationID.x));
        _123.y = _125.y + uint(int(_123.x < _125.x));
        uvec4 _155 = uvec4Pointer(_123).value;
        uvec4 _124 = _155;
        vec4 _160 = instance_buffer._m0[gl_GlobalInvocationID.x].bounding_sphere;
        vec4 _116 = _160;
        vec4 _169 = vec4(_160.xyz, 1.0) * frame_constants.view_constants[0u].world_to_view;
        uint _176 = clamp(uint(clamp((32.0 - _169.z) * 0.00347222224809229373931884765625, 0.0, 1.0) * 1024.0), 0u, 1023u);
        mat4 _121 = frame_constants.view_constants[0u].view_to_clip;
        vec2 _119 = vec2(1.0);
        vec2 _120 = vec2(0.0);
        vec4 _193 = vec4((-_121[0u].x) * _119.x, _121[0u].y, _120.x, _121[0u].w);
        vec4 _205 = vec4(_121[1u].x, _121[1u].y * _119.y, _120.y, _121[1u].w);
        vec4 _212 = vec4(_121[3u].x, _121[3u].y, -1.0, _121[3u].w);
        vec4 _118[4];
        _118[0] = _212 + _193;
        _118[1] = _212 - _193;
        _118[2] = _212 + _205;
        _118[3] = _212 - _205;
        for (int _224 = 0; _224 < 4; )
        {
            _118[_224] /= vec4(length(_118[_224].xyz));
            _224++;
            continue;
        }
        vec4 _117[4] = _118;
        vec3 _122 = _169.xyz;
        bool _251;
        if (((-_122.z) + _116.w) > _121[2].w)
        {
            _251 = ((-_122.z) - _116.w) < 3.4028234663852885981170418348452e+38;
        }
        else
        {
            _251 = false;
        }
        bool _253;
        _253 = _251;
        bool _254;
        for (int _256 = 0; _256 < 4; _253 = _254, _256++)
        {
            if (_253)
            {
                _254 = dot(vec4(_169.xyz, 1.0), _117[_256]) > (-_116.w);
            }
            else
            {
                _254 = false;
            }
        }
        uint _273 = gl_GlobalInvocationID.x + draw_list_info._m0[push_constants.draw_list_index_base].global_instance_offset;
        bool _284;
        if (_253)
        {
            _284 = (prev_visibility_bitmask._m0[_273 / 32u] & (1u << ((_273 % 32u) & 31u))) != 0u;
        }
        else
        {
            _284 = false;
        }
        if (_284)
        {
            uint _292 = atomicOr(_alias_next_visibility_bitmask_1._m0[_273 / 32u], 1u << ((_273 % 32u) & 31u));
            uint _294 = atomicAdd(_alias_filtered_instance_allocator_buffer_1._m0[push_constants.draw_list_index_base], 1u);
            _alias_filtered_instance_buffer._m0[_294] = (gl_GlobalInvocationID.x * 1024u) + _176;
            _alias_filtered_instance_meta_buffer._m0[_294] = GpuFilteredInstanceMeta(_124.w >> 16u);
            uint _301 = atomicAdd(_alias_output_histogram_1._m0[(push_constants.draw_list_index_base * 1024u) + _176], 1u);
        }
        break;
    } while(false);
}

