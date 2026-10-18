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

struct GpuFilteredInstanceMeta
{
    uint raw;
};

struct NaniDrawListInfo
{
    uint global_instance_offset;
};

struct NaniTrimeshChunk
{
    uvec2 gpu_address;
    uint vertex_index_count;
    uint vertex_byte_offset;
    vec4 bounding_sphere;
};

struct StairsFloorInstanceData
{
    NaniTrimeshChunk mesh;
    uint color_id;
    uint pad0;
    uint pad1;
    uint pad2;
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

layout(set = 0, binding = 23, std430) readonly buffer type_StructuredBuffer_StairsFloorInstanceData
{
    StairsFloorInstanceData _m0[];
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
        uvec2 _141 = push_constants.instance_buffer_addr;
        uvec2 _139;
        _139.x = _141.x + (16u * (gl_GlobalInvocationID.x));
        _139.y = _141.y + uint(int(_139.x < _141.x));
        uvec4 _171 = uvec4Pointer(_139).value;
        uvec4 _140 = _171;
        uint _174 = _140.w >> 16u;
        vec4 _176 = instance_buffer._m0[gl_GlobalInvocationID.x].mesh.bounding_sphere;
        vec4 _130 = _176;
        vec4 _184 = vec4(_176.xyz, 1.0);
        vec4 _185 = _184 * frame_constants.view_constants[0u].world_to_view;
        float _186 = _185.z;
        mat4 _132 = frame_constants.view_constants[0u].view_to_clip;
        uint _208 = clamp(uint(clamp((32.0 - _186) * 0.00347222224809229373931884765625, 0.0, 1.0) * 1024.0), 0u, 1023u);
        uint _209 = gl_GlobalInvocationID.x * 1024u;
        mat4 _137 = frame_constants.view_constants[0u].view_to_clip;
        vec2 _135 = vec2(1.0);
        vec2 _136 = vec2(0.0);
        vec4 _223 = vec4((-_137[0u].x) * _135.x, _137[0u].y, _136.x, _137[0u].w);
        vec4 _235 = vec4(_137[1u].x, _137[1u].y * _135.y, _136.y, _137[1u].w);
        vec4 _242 = vec4(_137[3u].x, _137[3u].y, -1.0, _137[3u].w);
        vec4 _133[4];
        _133[0] = _242 + _223;
        _133[1] = _242 - _223;
        _133[2] = _242 + _235;
        _133[3] = _242 - _235;
        for (int _254 = 0; _254 < 4; )
        {
            _133[_254] /= vec4(length(_133[_254].xyz));
            _254++;
            continue;
        }
        vec4 _131[4] = _133;
        vec3 _138 = _185.xyz;
        bool _281;
        if (((-_138.z) + _130.w) > _137[2].w)
        {
            _281 = ((-_138.z) - _130.w) < 3.4028234663852885981170418348452e+38;
        }
        else
        {
            _281 = false;
        }
        bool _283;
        _283 = _281;
        bool _284;
        for (int _286 = 0; _286 < 4; _283 = _284, _286++)
        {
            if (_283)
            {
                _284 = dot(vec4(_185.xy, _186, 1.0), _131[_286]) > (-_130.w);
            }
            else
            {
                _284 = false;
            }
        }
        uint _303 = gl_GlobalInvocationID.x + draw_list_info._m0[push_constants.draw_list_index_base].global_instance_offset;
        bool _314;
        if (_283)
        {
            _314 = (prev_visibility_bitmask._m0[_303 / 32u] & (1u << ((_303 % 32u) & 31u))) != 0u;
        }
        else
        {
            _314 = false;
        }
        if (_314)
        {
            uint _322 = atomicOr(_alias_next_visibility_bitmask_1._m0[_303 / 32u], 1u << ((_303 % 32u) & 31u));
            uint _324 = atomicAdd(_alias_filtered_instance_allocator_buffer_1._m0[push_constants.draw_list_index_base], 1u);
            _alias_filtered_instance_buffer._m0[_324] = _209 + _208;
            _alias_filtered_instance_meta_buffer._m0[_324] = GpuFilteredInstanceMeta(_174);
            uint _331 = atomicAdd(_alias_output_histogram_1._m0[(push_constants.draw_list_index_base * 1024u) + _208], 1u);
        }
        if ((((_130.w / spvNMax(9.9999997473787516355514526367188e-06, -_186)) * ((0.5 * _132[1u].y) * float(clamp(frame_constants.inner_rendering_resolution.y, 1080u, 2160u)))) < 10.0) ? false : true)
        {
            vec4 _339 = (_184 * frame_constants.view_constants[2u].world_to_view) * frame_constants.view_constants[2u].view_to_clip;
            vec4 _134 = _339;
            uint _348 = clamp(uint((_339.xyz / vec3(_134.w)).z * 1024.0), 0u, 1023u);
            uint _349 = _209 + _348;
            float _358 = _130.w * (2.0 * frame_constants.view_constants[2u].view_to_sample[0].x);
            vec2 _359 = abs((_184 * frame_constants.view_constants[2u].world_to_sample).xy);
            uint _369 = (any(greaterThanEqual(_359, vec2(-_358))) ? all(lessThanEqual(_359, vec2(1.0 + _358))) : false) ? 0u : 3u;
            bool _378 = any(greaterThanEqual(_359, vec2(1.0 - _358))) ? all(lessThanEqual(_359, vec2(2.0 + _358))) : false;
            uint _382;
            if (_378)
            {
                _382 = min(_369, 1u);
            }
            else
            {
                _382 = _369;
            }
            uint _383 = uint(_378);
            uint _397;
            uint _398;
            if (any(greaterThanEqual(_359, vec2(2.0 - _358))) ? all(lessThanEqual(_359, vec2(7.0 + _358))) : false)
            {
                _397 = max(_383, 2u);
                _398 = min(_382, 2u);
            }
            else
            {
                _397 = _383;
                _398 = _382;
            }
            for (uint _400 = _398; _400 <= _397; )
            {
                uint _406 = (push_constants.draw_list_index_base + 1u) + _400;
                uint _408 = atomicAdd(_alias_filtered_instance_allocator_buffer_1._m0[_406], 1u);
                _alias_filtered_instance_buffer._m0[_408] = _349;
                _alias_filtered_instance_meta_buffer._m0[_408] = GpuFilteredInstanceMeta(_174);
                uint _415 = atomicAdd(_alias_output_histogram_1._m0[(_406 * 1024u) + _348], 1u);
                _400++;
                continue;
            }
        }
        break;
    } while(false);
}

