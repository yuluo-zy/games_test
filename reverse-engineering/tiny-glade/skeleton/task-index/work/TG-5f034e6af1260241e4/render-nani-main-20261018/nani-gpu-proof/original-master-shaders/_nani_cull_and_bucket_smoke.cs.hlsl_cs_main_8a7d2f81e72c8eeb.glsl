#version 450
#extension GL_EXT_buffer_reference2 : require
#extension GL_EXT_nonuniform_qualifier : require
#extension GL_EXT_buffer_reference_uvec2 : require
#extension GL_EXT_spirv_intrinsics : require
#extension GL_EXT_samplerless_texture_functions : require
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

layout(set = 0, binding = 22, std140) uniform type_Globals
{
    uvec2 hzb_tex_inner_dims;
} _Globals;

layout(push_constant, std430) uniform type_ConstantBuffer_PushConstants
{
    uvec2 instance_buffer_addr;
    uint world_subset;
    uint draw_list_index_base;
    uint input_instance_count;
} push_constants;

layout(set = 0, binding = 21) uniform texture2D hzb_tex;

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
        if (gl_GlobalInvocationID.x >= push_constants.input_instance_count)
        {
            break;
        }
        uvec2 _145 = push_constants.instance_buffer_addr;
        uvec2 _143;
        _143.x = _145.x + (16u * (gl_GlobalInvocationID.x));
        _143.y = _145.y + uint(int(_143.x < _145.x));
        uvec4 _175 = uvec4Pointer(_143).value;
        uvec4 _144 = _175;
        vec3 _126 = uintBitsToFloat(_175.xyz);
        _126.y += 7.5;
        vec4 _191 = vec4(_126, 1.0) * frame_constants.view_constants[0u].world_to_view;
        uint _198 = clamp(uint(clamp((32.0 - _191.z) * 0.00347222224809229373931884765625, 0.0, 1.0) * 1024.0), 0u, 1023u);
        mat4 _135 = frame_constants.view_constants[0u].view_to_clip;
        vec2 _133 = vec2(1.0);
        vec2 _134 = vec2(0.0);
        vec4 _215 = vec4((-_135[0u].x) * _133.x, _135[0u].y, _134.x, _135[0u].w);
        vec4 _227 = vec4(_135[1u].x, _135[1u].y * _133.y, _134.y, _135[1u].w);
        vec4 _234 = vec4(_135[3u].x, _135[3u].y, -1.0, _135[3u].w);
        vec4 _129[4];
        _129[0] = _234 + _215;
        _129[1] = _234 - _215;
        _129[2] = _234 + _227;
        _129[3] = _234 - _227;
        for (int _246 = 0; _246 < 4; )
        {
            _129[_246] /= vec4(length(_129[_246].xyz));
            _246++;
            continue;
        }
        vec4 _128[4] = _129;
        vec3 _261 = _191.xyz;
        vec3 _136 = _261;
        vec3 _137 = _261;
        bool _271;
        if ((7.5 - _137.z) > _135[2].w)
        {
            _271 = ((-7.5) - _137.z) < 3.4028234663852885981170418348452e+38;
        }
        else
        {
            _271 = false;
        }
        bool _273;
        _273 = _271;
        bool _274;
        for (int _276 = 0; _276 < 4; _273 = _274, _276++)
        {
            if (_273)
            {
                _274 = dot(vec4(_191.xyz, 1.0), _128[_276]) > (-7.5);
            }
            else
            {
                _274 = false;
            }
        }
        uint _291 = draw_list_info._m0[push_constants.draw_list_index_base].global_instance_offset;
        uint _292 = gl_GlobalInvocationID.x + _291;
        vec3 _139 = _261;
        _139.z = -_139.z;
        mat4 _131 = frame_constants.view_constants[0u].view_to_clip;
        float _301 = _131[0u].x * 0.5;
        mat4 _130 = frame_constants.view_constants[0u].view_to_clip;
        float _304 = _130[1u].y * 0.5;
        vec3 _132 = _139 * 7.5;
        float _310 = (_139.z * _139.z) - 56.25;
        float _316 = sqrt((_139.x * _139.x) + _310);
        float _342 = sqrt((_139.y * _139.y) + _310);
        vec4 _373 = vec4(_Globals.hzb_tex_inner_dims.xyxy);
        vec4 _376 = clamp((vec4((((_316 * _139.x) - _132.z) / ((_316 * _139.z) + _132.x)) * _301, (-(((_342 * _139.y) + _132.z) / ((_342 * _139.z) - _132.y))) * _304, (((_316 * _139.x) + _132.z) / ((_316 * _139.z) - _132.x)) * _301, (-(((_342 * _139.y) - _132.z) / ((_342 * _139.z) + _132.y))) * _304) + vec4(0.5)) * _373, vec4(0.0), _373 - vec4(1.0));
        vec4 _138 = _376;
        vec2 _140 = vec2(_138.z - _138.x, _138.w - _138.y);
        uint _396 = uint(spvNMax(0.0, ceil(log2(spvNMax(_140.x, _140.y)))));
        bool _444;
        if (_273 ? true : false)
        {
            uvec2 _403 = uvec2(_396) & uvec2(31u);
            uvec2 _141 = uvec2(_376.xy) >> _403;
            uvec2 _142 = uvec2(_376.zw) >> _403;
            mat4 _127 = frame_constants.view_constants[0u].clip_to_view;
            _444 = (_136.z + 7.5) > (1.0 / (spvNMin(spvNMin(texelFetch(hzb_tex, ivec2(uvec2(_141.x, _141.y)), int(_396)).x, texelFetch(hzb_tex, ivec2(uvec2(_141.x, _142.y)), int(_396)).x), spvNMin(texelFetch(hzb_tex, ivec2(uvec2(_142.x, _141.y)), int(_396)).x, texelFetch(hzb_tex, ivec2(uvec2(_142.x, _142.y)), int(_396)).x)) * (-_127[3].z)));
        }
        else
        {
            _444 = _273;
        }
        if (_444)
        {
            uint _447 = _292 / 32u;
            uint _451 = 1u << ((_292 % 32u) & 31u);
            uint _452 = atomicOr(_alias_next_visibility_bitmask_1._m0[_447], _451);
            if (!((prev_visibility_bitmask._m0[_447] & _451) != 0u))
            {
                uint _461 = atomicAdd(_alias_filtered_instance_allocator_buffer_1._m0[push_constants.draw_list_index_base], 1u);
                _alias_filtered_instance_buffer._m0[_461] = (gl_GlobalInvocationID.x * 1024u) + _198;
                _alias_filtered_instance_meta_buffer._m0[_461] = GpuFilteredInstanceMeta(_144.w >> 16u);
                uint _468 = atomicAdd(_alias_output_histogram_1._m0[(push_constants.draw_list_index_base * 1024u) + _198], 1u);
            }
        }
        break;
    } while(false);
}

