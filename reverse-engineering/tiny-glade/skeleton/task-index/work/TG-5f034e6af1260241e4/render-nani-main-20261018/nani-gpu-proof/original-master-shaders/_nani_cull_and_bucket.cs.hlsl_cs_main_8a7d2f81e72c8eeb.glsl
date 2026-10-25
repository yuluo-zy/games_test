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

layout(set = 0, binding = 22, std140) uniform type_Globals
{
    uvec2 hzb_tex_inner_dims;
} _Globals;

layout(push_constant, std430) uniform type_ConstantBuffer_PushConstants
{
    uvec2 instance_buffer_addr;
    uint world_subset;
    uint input_instance_count;
    uint draw_list_index_base;
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
        uvec2 _149 = push_constants.instance_buffer_addr;
        uvec2 _147;
        _147.x = _149.x + (16u * (gl_GlobalInvocationID.x));
        _147.y = _149.y + uint(int(_147.x < _149.x));
        uvec4 _179 = uvec4Pointer(_147).value;
        uvec4 _148 = _179;
        uint _184 = _148.w >> 16u;
        vec2 _146 = vec2(unpackHalf2x16(_148.w & 65535u).x, unpackHalf2x16(_184 & 65535u).x);
        float _198 = _146.x * atlas_part_info._m0[push_constants.world_subset][_184].bounding_sphere_radius;
        vec4 _205 = vec4(uintBitsToFloat(_179.xyz), 1.0) * frame_constants.view_constants[0u].world_to_view;
        uint _212 = clamp(uint(clamp((32.0 - _205.z) * 0.00347222224809229373931884765625, 0.0, 1.0) * 1024.0), 0u, 1023u);
        mat4 _138 = frame_constants.view_constants[0u].view_to_clip;
        vec2 _136 = vec2(1.0);
        vec2 _137 = vec2(0.0);
        vec4 _229 = vec4((-_138[0u].x) * _136.x, _138[0u].y, _137.x, _138[0u].w);
        vec4 _241 = vec4(_138[1u].x, _138[1u].y * _136.y, _137.y, _138[1u].w);
        vec4 _248 = vec4(_138[3u].x, _138[3u].y, -1.0, _138[3u].w);
        vec4 _132[4];
        _132[0] = _248 + _229;
        _132[1] = _248 - _229;
        _132[2] = _248 + _241;
        _132[3] = _248 - _241;
        for (int _260 = 0; _260 < 4; )
        {
            _132[_260] /= vec4(length(_132[_260].xyz));
            _260++;
            continue;
        }
        vec4 _131[4] = _132;
        vec3 _275 = _205.xyz;
        vec3 _139 = _275;
        vec3 _140 = _275;
        bool _287;
        if (((-_140.z) + _198) > _138[2].w)
        {
            _287 = ((-_140.z) - _198) < 3.4028234663852885981170418348452e+38;
        }
        else
        {
            _287 = false;
        }
        bool _289;
        _289 = _287;
        bool _290;
        for (int _292 = 0; _292 < 4; _289 = _290, _292++)
        {
            if (_289)
            {
                _290 = dot(vec4(_205.xyz, 1.0), _131[_292]) > (-_198);
            }
            else
            {
                _290 = false;
            }
        }
        uint _308 = draw_list_info._m0[push_constants.draw_list_index_base].global_instance_offset;
        uint _309 = gl_GlobalInvocationID.x + _308;
        vec3 _142 = _275;
        _142.z = -_142.z;
        mat4 _134 = frame_constants.view_constants[0u].view_to_clip;
        float _318 = _134[0u].x * 0.5;
        mat4 _133 = frame_constants.view_constants[0u].view_to_clip;
        float _321 = _133[1u].y * 0.5;
        vec3 _135 = _142 * _198;
        float _328 = (_142.z * _142.z) - (_198 * _198);
        float _334 = sqrt((_142.x * _142.x) + _328);
        float _360 = sqrt((_142.y * _142.y) + _328);
        vec4 _391 = vec4(_Globals.hzb_tex_inner_dims.xyxy);
        vec4 _394 = clamp((vec4((((_334 * _142.x) - _135.z) / ((_334 * _142.z) + _135.x)) * _318, (-(((_360 * _142.y) + _135.z) / ((_360 * _142.z) - _135.y))) * _321, (((_334 * _142.x) + _135.z) / ((_334 * _142.z) - _135.x)) * _318, (-(((_360 * _142.y) - _135.z) / ((_360 * _142.z) + _135.y))) * _321) + vec4(0.5)) * _391, vec4(0.0), _391 - vec4(1.0));
        vec4 _141 = _394;
        vec2 _143 = vec2(_141.z - _141.x, _141.w - _141.y);
        uint _414 = uint(spvNMax(0.0, ceil(log2(spvNMax(_143.x, _143.y)))));
        bool _462;
        if (_289 ? true : false)
        {
            uvec2 _421 = uvec2(_414) & uvec2(31u);
            uvec2 _144 = uvec2(_394.xy) >> _421;
            uvec2 _145 = uvec2(_394.zw) >> _421;
            mat4 _130 = frame_constants.view_constants[0u].clip_to_view;
            _462 = (_139.z + _198) > (1.0 / (spvNMin(spvNMin(texelFetch(hzb_tex, ivec2(uvec2(_144.x, _144.y)), int(_414)).x, texelFetch(hzb_tex, ivec2(uvec2(_144.x, _145.y)), int(_414)).x), spvNMin(texelFetch(hzb_tex, ivec2(uvec2(_145.x, _144.y)), int(_414)).x, texelFetch(hzb_tex, ivec2(uvec2(_145.x, _145.y)), int(_414)).x)) * (-_130[3].z)));
        }
        else
        {
            _462 = _289;
        }
        if (_462)
        {
            uint _465 = _309 / 32u;
            uint _469 = 1u << ((_309 % 32u) & 31u);
            uint _470 = atomicOr(_alias_next_visibility_bitmask_1._m0[_465], _469);
            if (!((prev_visibility_bitmask._m0[_465] & _469) != 0u))
            {
                uint _479 = atomicAdd(_alias_filtered_instance_allocator_buffer_1._m0[push_constants.draw_list_index_base], 1u);
                _alias_filtered_instance_buffer._m0[_479] = (gl_GlobalInvocationID.x * 1024u) + _212;
                _alias_filtered_instance_meta_buffer._m0[_479] = GpuFilteredInstanceMeta(_184);
                uint _486 = atomicAdd(_alias_output_histogram_1._m0[(push_constants.draw_list_index_base * 1024u) + _212], 1u);
            }
        }
        break;
    } while(false);
}

