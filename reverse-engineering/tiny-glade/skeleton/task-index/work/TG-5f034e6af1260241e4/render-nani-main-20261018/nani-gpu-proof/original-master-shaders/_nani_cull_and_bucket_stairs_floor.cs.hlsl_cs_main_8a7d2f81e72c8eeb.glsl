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

layout(set = 0, binding = 22, std140) uniform type_Globals
{
    uvec2 hzb_tex_inner_dims;
} _Globals;

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
        uvec2 _150 = push_constants.instance_buffer_addr;
        uvec2 _148;
        _148.x = _150.x + (16u * (gl_GlobalInvocationID.x));
        _148.y = _150.y + uint(int(_148.x < _150.x));
        uvec4 _180 = uvec4Pointer(_148).value;
        uvec4 _149 = _180;
        vec4 _185 = instance_buffer._m0[gl_GlobalInvocationID.x].mesh.bounding_sphere;
        vec4 _131 = _185;
        vec4 _194 = vec4(_185.xyz, 1.0) * frame_constants.view_constants[0u].world_to_view;
        uint _201 = clamp(uint(clamp((32.0 - _194.z) * 0.00347222224809229373931884765625, 0.0, 1.0) * 1024.0), 0u, 1023u);
        mat4 _140 = frame_constants.view_constants[0u].view_to_clip;
        vec2 _138 = vec2(1.0);
        vec2 _139 = vec2(0.0);
        vec4 _218 = vec4((-_140[0u].x) * _138.x, _140[0u].y, _139.x, _140[0u].w);
        vec4 _230 = vec4(_140[1u].x, _140[1u].y * _138.y, _139.y, _140[1u].w);
        vec4 _237 = vec4(_140[3u].x, _140[3u].y, -1.0, _140[3u].w);
        vec4 _134[4];
        _134[0] = _237 + _218;
        _134[1] = _237 - _218;
        _134[2] = _237 + _230;
        _134[3] = _237 - _230;
        for (int _249 = 0; _249 < 4; )
        {
            _134[_249] /= vec4(length(_134[_249].xyz));
            _249++;
            continue;
        }
        vec4 _133[4] = _134;
        vec3 _264 = _194.xyz;
        vec3 _141 = _264;
        vec3 _142 = _264;
        bool _276;
        if (((-_142.z) + _131.w) > _140[2].w)
        {
            _276 = ((-_142.z) - _131.w) < 3.4028234663852885981170418348452e+38;
        }
        else
        {
            _276 = false;
        }
        bool _278;
        _278 = _276;
        bool _279;
        for (int _281 = 0; _281 < 4; _278 = _279, _281++)
        {
            if (_278)
            {
                _279 = dot(vec4(_194.xyz, 1.0), _133[_281]) > (-_131.w);
            }
            else
            {
                _279 = false;
            }
        }
        uint _297 = draw_list_info._m0[push_constants.draw_list_index_base].global_instance_offset;
        uint _298 = gl_GlobalInvocationID.x + _297;
        vec3 _144 = _264;
        _144.z = -_144.z;
        mat4 _136 = frame_constants.view_constants[0u].view_to_clip;
        float _307 = _136[0u].x * 0.5;
        mat4 _135 = frame_constants.view_constants[0u].view_to_clip;
        float _310 = _135[1u].y * 0.5;
        vec3 _137 = _144 * _131.w;
        float _317 = (_144.z * _144.z) - (_131.w * _131.w);
        float _323 = sqrt((_144.x * _144.x) + _317);
        float _349 = sqrt((_144.y * _144.y) + _317);
        vec4 _380 = vec4(_Globals.hzb_tex_inner_dims.xyxy);
        vec4 _383 = clamp((vec4((((_323 * _144.x) - _137.z) / ((_323 * _144.z) + _137.x)) * _307, (-(((_349 * _144.y) + _137.z) / ((_349 * _144.z) - _137.y))) * _310, (((_323 * _144.x) + _137.z) / ((_323 * _144.z) - _137.x)) * _307, (-(((_349 * _144.y) - _137.z) / ((_349 * _144.z) + _137.y))) * _310) + vec4(0.5)) * _380, vec4(0.0), _380 - vec4(1.0));
        vec4 _143 = _383;
        vec2 _145 = vec2(_143.z - _143.x, _143.w - _143.y);
        uint _403 = uint(spvNMax(0.0, ceil(log2(spvNMax(_145.x, _145.y)))));
        bool _451;
        if (_278 ? true : false)
        {
            uvec2 _410 = uvec2(_403) & uvec2(31u);
            uvec2 _146 = uvec2(_383.xy) >> _410;
            uvec2 _147 = uvec2(_383.zw) >> _410;
            mat4 _132 = frame_constants.view_constants[0u].clip_to_view;
            _451 = (_141.z + _131.w) > (1.0 / (spvNMin(spvNMin(texelFetch(hzb_tex, ivec2(uvec2(_146.x, _146.y)), int(_403)).x, texelFetch(hzb_tex, ivec2(uvec2(_146.x, _147.y)), int(_403)).x), spvNMin(texelFetch(hzb_tex, ivec2(uvec2(_147.x, _146.y)), int(_403)).x, texelFetch(hzb_tex, ivec2(uvec2(_147.x, _147.y)), int(_403)).x)) * (-_132[3].z)));
        }
        else
        {
            _451 = _278;
        }
        if (_451)
        {
            uint _454 = _298 / 32u;
            uint _458 = 1u << ((_298 % 32u) & 31u);
            uint _459 = atomicOr(_alias_next_visibility_bitmask_1._m0[_454], _458);
            if (!((prev_visibility_bitmask._m0[_454] & _458) != 0u))
            {
                uint _468 = atomicAdd(_alias_filtered_instance_allocator_buffer_1._m0[push_constants.draw_list_index_base], 1u);
                _alias_filtered_instance_buffer._m0[_468] = (gl_GlobalInvocationID.x * 1024u) + _201;
                _alias_filtered_instance_meta_buffer._m0[_468] = GpuFilteredInstanceMeta(_149.w >> 16u);
                uint _475 = atomicAdd(_alias_output_histogram_1._m0[(push_constants.draw_list_index_base * 1024u) + _201], 1u);
            }
        }
        break;
    } while(false);
}

