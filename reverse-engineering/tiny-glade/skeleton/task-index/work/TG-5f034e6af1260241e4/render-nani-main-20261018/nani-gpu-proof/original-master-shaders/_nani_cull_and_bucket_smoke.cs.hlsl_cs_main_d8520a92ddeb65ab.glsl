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
        uvec2 _137 = push_constants.instance_buffer_addr;
        uvec2 _135;
        _135.x = _137.x + (16u * (gl_GlobalInvocationID.x));
        _135.y = _137.y + uint(int(_135.x < _137.x));
        uvec4 _167 = uvec4Pointer(_135).value;
        uvec4 _136 = _167;
        uint _172 = _136.w >> 16u;
        vec3 _126 = uintBitsToFloat(_167.xyz);
        _126.y += 7.5;
        vec4 _182 = vec4(_126, 1.0);
        vec4 _183 = _182 * frame_constants.view_constants[0u].world_to_view;
        float _184 = _183.z;
        mat4 _128 = frame_constants.view_constants[0u].view_to_clip;
        uint _206 = clamp(uint(clamp((32.0 - _184) * 0.00347222224809229373931884765625, 0.0, 1.0) * 1024.0), 0u, 1023u);
        uint _207 = gl_GlobalInvocationID.x * 1024u;
        mat4 _133 = frame_constants.view_constants[0u].view_to_clip;
        vec2 _131 = vec2(1.0);
        vec2 _132 = vec2(0.0);
        vec4 _221 = vec4((-_133[0u].x) * _131.x, _133[0u].y, _132.x, _133[0u].w);
        vec4 _233 = vec4(_133[1u].x, _133[1u].y * _131.y, _132.y, _133[1u].w);
        vec4 _240 = vec4(_133[3u].x, _133[3u].y, -1.0, _133[3u].w);
        vec4 _129[4];
        _129[0] = _240 + _221;
        _129[1] = _240 - _221;
        _129[2] = _240 + _233;
        _129[3] = _240 - _233;
        for (int _252 = 0; _252 < 4; )
        {
            _129[_252] /= vec4(length(_129[_252].xyz));
            _252++;
            continue;
        }
        vec4 _127[4] = _129;
        vec3 _134 = _183.xyz;
        bool _277;
        if ((7.5 - _134.z) > _133[2].w)
        {
            _277 = ((-7.5) - _134.z) < 3.4028234663852885981170418348452e+38;
        }
        else
        {
            _277 = false;
        }
        bool _279;
        _279 = _277;
        bool _280;
        for (int _282 = 0; _282 < 4; _279 = _280, _282++)
        {
            if (_279)
            {
                _280 = dot(vec4(_183.xy, _184, 1.0), _127[_282]) > (-7.5);
            }
            else
            {
                _280 = false;
            }
        }
        uint _298 = gl_GlobalInvocationID.x + draw_list_info._m0[push_constants.draw_list_index_base].global_instance_offset;
        bool _309;
        if (_279)
        {
            _309 = (prev_visibility_bitmask._m0[_298 / 32u] & (1u << ((_298 % 32u) & 31u))) != 0u;
        }
        else
        {
            _309 = false;
        }
        if (_309)
        {
            uint _317 = atomicOr(_alias_next_visibility_bitmask_1._m0[_298 / 32u], 1u << ((_298 % 32u) & 31u));
            uint _319 = atomicAdd(_alias_filtered_instance_allocator_buffer_1._m0[push_constants.draw_list_index_base], 1u);
            _alias_filtered_instance_buffer._m0[_319] = _207 + _206;
            _alias_filtered_instance_meta_buffer._m0[_319] = GpuFilteredInstanceMeta(_172);
            uint _326 = atomicAdd(_alias_output_histogram_1._m0[(push_constants.draw_list_index_base * 1024u) + _206], 1u);
        }
        if ((((7.5 / spvNMax(9.9999997473787516355514526367188e-06, -_184)) * ((0.5 * _128[1u].y) * float(clamp(frame_constants.inner_rendering_resolution.y, 1080u, 2160u)))) < 10.0) ? false : true)
        {
            vec4 _334 = (_182 * frame_constants.view_constants[2u].world_to_view) * frame_constants.view_constants[2u].view_to_clip;
            vec4 _130 = _334;
            uint _343 = clamp(uint((_334.xyz / vec3(_130.w)).z * 1024.0), 0u, 1023u);
            uint _344 = _207 + _343;
            float _352 = frame_constants.view_constants[2u].view_to_sample[0].x * 15.0;
            vec2 _353 = abs((_182 * frame_constants.view_constants[2u].world_to_sample).xy);
            uint _363 = (any(greaterThanEqual(_353, vec2(frame_constants.view_constants[2u].view_to_sample[0].x * (-15.0)))) ? all(lessThanEqual(_353, vec2(1.0 + _352))) : false) ? 0u : 3u;
            bool _372 = any(greaterThanEqual(_353, vec2(1.0 - _352))) ? all(lessThanEqual(_353, vec2(2.0 + _352))) : false;
            uint _376;
            if (_372)
            {
                _376 = min(_363, 1u);
            }
            else
            {
                _376 = _363;
            }
            uint _377 = uint(_372);
            uint _391;
            uint _392;
            if (any(greaterThanEqual(_353, vec2(2.0 - _352))) ? all(lessThanEqual(_353, vec2(7.0 + _352))) : false)
            {
                _391 = max(_377, 2u);
                _392 = min(_376, 2u);
            }
            else
            {
                _391 = _377;
                _392 = _376;
            }
            for (uint _394 = _392; _394 <= _391; )
            {
                uint _400 = (push_constants.draw_list_index_base + 1u) + _394;
                uint _402 = atomicAdd(_alias_filtered_instance_allocator_buffer_1._m0[_400], 1u);
                _alias_filtered_instance_buffer._m0[_402] = _344;
                _alias_filtered_instance_meta_buffer._m0[_402] = GpuFilteredInstanceMeta(_172);
                uint _409 = atomicAdd(_alias_output_histogram_1._m0[(_400 * 1024u) + _343], 1u);
                _394++;
                continue;
            }
        }
        break;
    } while(false);
}

