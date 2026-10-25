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

layout(set = 0, binding = 22, std140) uniform type_Globals
{
    uvec2 hzb_tex_inner_dims;
} _Globals;

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
        uvec2 _148 = push_constants.instance_buffer_addr;
        uvec2 _146;
        _146.x = _148.x + (16u * (gl_GlobalInvocationID.x));
        _146.y = _148.y + uint(int(_146.x < _148.x));
        uvec4 _178 = uvec4Pointer(_146).value;
        uvec4 _147 = _178;
        vec4 _183 = instance_buffer._m0[gl_GlobalInvocationID.x].bounding_sphere;
        vec4 _129 = _183;
        vec4 _192 = vec4(_183.xyz, 1.0) * frame_constants.view_constants[0u].world_to_view;
        uint _199 = clamp(uint(clamp((32.0 - _192.z) * 0.00347222224809229373931884765625, 0.0, 1.0) * 1024.0), 0u, 1023u);
        mat4 _138 = frame_constants.view_constants[0u].view_to_clip;
        vec2 _136 = vec2(1.0);
        vec2 _137 = vec2(0.0);
        vec4 _216 = vec4((-_138[0u].x) * _136.x, _138[0u].y, _137.x, _138[0u].w);
        vec4 _228 = vec4(_138[1u].x, _138[1u].y * _136.y, _137.y, _138[1u].w);
        vec4 _235 = vec4(_138[3u].x, _138[3u].y, -1.0, _138[3u].w);
        vec4 _132[4];
        _132[0] = _235 + _216;
        _132[1] = _235 - _216;
        _132[2] = _235 + _228;
        _132[3] = _235 - _228;
        for (int _247 = 0; _247 < 4; )
        {
            _132[_247] /= vec4(length(_132[_247].xyz));
            _247++;
            continue;
        }
        vec4 _131[4] = _132;
        vec3 _262 = _192.xyz;
        vec3 _139 = _262;
        vec3 _140 = _262;
        bool _274;
        if (((-_140.z) + _129.w) > _138[2].w)
        {
            _274 = ((-_140.z) - _129.w) < 3.4028234663852885981170418348452e+38;
        }
        else
        {
            _274 = false;
        }
        bool _276;
        _276 = _274;
        bool _277;
        for (int _279 = 0; _279 < 4; _276 = _277, _279++)
        {
            if (_276)
            {
                _277 = dot(vec4(_192.xyz, 1.0), _131[_279]) > (-_129.w);
            }
            else
            {
                _277 = false;
            }
        }
        uint _295 = draw_list_info._m0[push_constants.draw_list_index_base].global_instance_offset;
        uint _296 = gl_GlobalInvocationID.x + _295;
        vec3 _142 = _262;
        _142.z = -_142.z;
        mat4 _134 = frame_constants.view_constants[0u].view_to_clip;
        float _305 = _134[0u].x * 0.5;
        mat4 _133 = frame_constants.view_constants[0u].view_to_clip;
        float _308 = _133[1u].y * 0.5;
        vec3 _135 = _142 * _129.w;
        float _315 = (_142.z * _142.z) - (_129.w * _129.w);
        float _321 = sqrt((_142.x * _142.x) + _315);
        float _347 = sqrt((_142.y * _142.y) + _315);
        vec4 _378 = vec4(_Globals.hzb_tex_inner_dims.xyxy);
        vec4 _381 = clamp((vec4((((_321 * _142.x) - _135.z) / ((_321 * _142.z) + _135.x)) * _305, (-(((_347 * _142.y) + _135.z) / ((_347 * _142.z) - _135.y))) * _308, (((_321 * _142.x) + _135.z) / ((_321 * _142.z) - _135.x)) * _305, (-(((_347 * _142.y) - _135.z) / ((_347 * _142.z) + _135.y))) * _308) + vec4(0.5)) * _378, vec4(0.0), _378 - vec4(1.0));
        vec4 _141 = _381;
        vec2 _143 = vec2(_141.z - _141.x, _141.w - _141.y);
        uint _401 = uint(spvNMax(0.0, ceil(log2(spvNMax(_143.x, _143.y)))));
        bool _449;
        if (_276 ? true : false)
        {
            uvec2 _408 = uvec2(_401) & uvec2(31u);
            uvec2 _144 = uvec2(_381.xy) >> _408;
            uvec2 _145 = uvec2(_381.zw) >> _408;
            mat4 _130 = frame_constants.view_constants[0u].clip_to_view;
            _449 = (_139.z + _129.w) > (1.0 / (spvNMin(spvNMin(texelFetch(hzb_tex, ivec2(uvec2(_144.x, _144.y)), int(_401)).x, texelFetch(hzb_tex, ivec2(uvec2(_144.x, _145.y)), int(_401)).x), spvNMin(texelFetch(hzb_tex, ivec2(uvec2(_145.x, _144.y)), int(_401)).x, texelFetch(hzb_tex, ivec2(uvec2(_145.x, _145.y)), int(_401)).x)) * (-_130[3].z)));
        }
        else
        {
            _449 = _276;
        }
        if (_449)
        {
            uint _452 = _296 / 32u;
            uint _456 = 1u << ((_296 % 32u) & 31u);
            uint _457 = atomicOr(_alias_next_visibility_bitmask_1._m0[_452], _456);
            if (!((prev_visibility_bitmask._m0[_452] & _456) != 0u))
            {
                uint _466 = atomicAdd(_alias_filtered_instance_allocator_buffer_1._m0[push_constants.draw_list_index_base], 1u);
                _alias_filtered_instance_buffer._m0[_466] = (gl_GlobalInvocationID.x * 1024u) + _199;
                _alias_filtered_instance_meta_buffer._m0[_466] = GpuFilteredInstanceMeta(_147.w >> 16u);
                uint _473 = atomicAdd(_alias_output_histogram_1._m0[(push_constants.draw_list_index_base * 1024u) + _199], 1u);
            }
        }
        break;
    } while(false);
}

