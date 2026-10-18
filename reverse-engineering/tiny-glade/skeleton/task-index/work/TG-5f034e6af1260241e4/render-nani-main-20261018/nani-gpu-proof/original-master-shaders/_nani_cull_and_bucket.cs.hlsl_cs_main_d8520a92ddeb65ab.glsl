#version 450
#extension GL_EXT_buffer_reference2 : require
#extension GL_EXT_nonuniform_qualifier : require
#extension GL_EXT_buffer_reference_uvec2 : require
#extension GL_EXT_spirv_intrinsics : require
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

struct MeshAtlasPartInfo
{
    uint first_index;
    uint index_count;
    float bounding_sphere_radius;
    uint pad0;
};

struct GpuFilteredInstanceMeta
{
    uint raw;
};

struct NaniDrawListInfo
{
    uint global_instance_offset;
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

layout(set = 0, binding = 13, std430) readonly buffer type_StructuredBuffer_
{
    MeshAtlasPartInfo _m0[][64];
} atlas_part_info;

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

layout(push_constant, std430) uniform type_ConstantBuffer_PushConstants
{
    uvec2 instance_buffer_addr;
    uint world_subset;
    uint input_instance_count;
    uint draw_list_index_base;
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
        uvec2 _140 = push_constants.instance_buffer_addr;
        uvec2 _138;
        _138.x = _140.x + (16u * (gl_GlobalInvocationID.x));
        _138.y = _140.y + uint(int(_138.x < _140.x));
        uvec4 _170 = uvec4Pointer(_138).value;
        uvec4 _139 = _170;
        uint _175 = _139.w >> 16u;
        vec2 _137 = vec2(unpackHalf2x16(_139.w & 65535u).x, unpackHalf2x16(_175 & 65535u).x);
        float _188 = atlas_part_info._m0[push_constants.world_subset][_175].bounding_sphere_radius;
        float _189 = _137.x * _188;
        vec4 _195 = vec4(uintBitsToFloat(_170.xyz), 1.0);
        vec4 _196 = _195 * frame_constants.view_constants[0u].world_to_view;
        float _197 = _196.z;
        mat4 _130 = frame_constants.view_constants[0u].view_to_clip;
        uint _219 = clamp(uint(clamp((32.0 - _197) * 0.00347222224809229373931884765625, 0.0, 1.0) * 1024.0), 0u, 1023u);
        uint _220 = gl_GlobalInvocationID.x * 1024u;
        mat4 _135 = frame_constants.view_constants[0u].view_to_clip;
        vec2 _133 = vec2(1.0);
        vec2 _134 = vec2(0.0);
        vec4 _234 = vec4((-_135[0u].x) * _133.x, _135[0u].y, _134.x, _135[0u].w);
        vec4 _246 = vec4(_135[1u].x, _135[1u].y * _133.y, _134.y, _135[1u].w);
        vec4 _253 = vec4(_135[3u].x, _135[3u].y, -1.0, _135[3u].w);
        vec4 _131[4];
        _131[0] = _253 + _234;
        _131[1] = _253 - _234;
        _131[2] = _253 + _246;
        _131[3] = _253 - _246;
        for (int _265 = 0; _265 < 4; )
        {
            _131[_265] /= vec4(length(_131[_265].xyz));
            _265++;
            continue;
        }
        vec4 _129[4] = _131;
        vec3 _136 = _196.xyz;
        bool _292;
        if (((-_136.z) + _189) > _135[2].w)
        {
            _292 = ((-_136.z) - _189) < 3.4028234663852885981170418348452e+38;
        }
        else
        {
            _292 = false;
        }
        bool _294;
        _294 = _292;
        bool _295;
        for (int _297 = 0; _297 < 4; _294 = _295, _297++)
        {
            if (_294)
            {
                _295 = dot(vec4(_196.xy, _197, 1.0), _129[_297]) > (-_189);
            }
            else
            {
                _295 = false;
            }
        }
        uint _314 = gl_GlobalInvocationID.x + draw_list_info._m0[push_constants.draw_list_index_base].global_instance_offset;
        bool _325;
        if (_294)
        {
            _325 = (prev_visibility_bitmask._m0[_314 / 32u] & (1u << ((_314 % 32u) & 31u))) != 0u;
        }
        else
        {
            _325 = false;
        }
        if (_325)
        {
            uint _333 = atomicOr(_alias_next_visibility_bitmask_1._m0[_314 / 32u], 1u << ((_314 % 32u) & 31u));
            uint _335 = atomicAdd(_alias_filtered_instance_allocator_buffer_1._m0[push_constants.draw_list_index_base], 1u);
            _alias_filtered_instance_buffer._m0[_335] = _220 + _219;
            _alias_filtered_instance_meta_buffer._m0[_335] = GpuFilteredInstanceMeta(_175);
            uint _342 = atomicAdd(_alias_output_histogram_1._m0[(push_constants.draw_list_index_base * 1024u) + _219], 1u);
        }
        if ((((_189 / spvNMax(9.9999997473787516355514526367188e-06, -_197)) * ((0.5 * _130[1u].y) * float(clamp(frame_constants.inner_rendering_resolution.y, 1080u, 2160u)))) < 10.0) ? false : true)
        {
            vec4 _350 = (_195 * frame_constants.view_constants[2u].world_to_view) * frame_constants.view_constants[2u].view_to_clip;
            vec4 _132 = _350;
            uint _359 = clamp(uint((_350.xyz / vec3(_132.w)).z * 1024.0), 0u, 1023u);
            uint _360 = _220 + _359;
            float _369 = _189 * (2.0 * frame_constants.view_constants[2u].view_to_sample[0].x);
            vec2 _370 = abs((_195 * frame_constants.view_constants[2u].world_to_sample).xy);
            uint _380 = (any(greaterThanEqual(_370, vec2(-_369))) ? all(lessThanEqual(_370, vec2(1.0 + _369))) : false) ? 0u : 3u;
            bool _389 = any(greaterThanEqual(_370, vec2(1.0 - _369))) ? all(lessThanEqual(_370, vec2(2.0 + _369))) : false;
            uint _393;
            if (_389)
            {
                _393 = min(_380, 1u);
            }
            else
            {
                _393 = _380;
            }
            uint _394 = uint(_389);
            uint _408;
            uint _409;
            if (any(greaterThanEqual(_370, vec2(2.0 - _369))) ? all(lessThanEqual(_370, vec2(7.0 + _369))) : false)
            {
                _408 = max(_394, 2u);
                _409 = min(_393, 2u);
            }
            else
            {
                _408 = _394;
                _409 = _393;
            }
            for (uint _411 = _409; _411 <= _408; )
            {
                uint _417 = (push_constants.draw_list_index_base + 1u) + _411;
                uint _419 = atomicAdd(_alias_filtered_instance_allocator_buffer_1._m0[_417], 1u);
                _alias_filtered_instance_buffer._m0[_419] = _360;
                _alias_filtered_instance_meta_buffer._m0[_419] = GpuFilteredInstanceMeta(_175);
                uint _426 = atomicAdd(_alias_output_histogram_1._m0[(_417 * 1024u) + _359], 1u);
                _411++;
                continue;
            }
        }
        break;
    } while(false);
}

