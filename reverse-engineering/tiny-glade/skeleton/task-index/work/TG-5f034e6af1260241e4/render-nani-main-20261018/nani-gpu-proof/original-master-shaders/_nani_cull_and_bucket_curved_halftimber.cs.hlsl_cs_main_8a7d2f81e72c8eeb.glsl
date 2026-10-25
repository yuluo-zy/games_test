#version 450
#extension GL_EXT_buffer_reference2 : require
#extension GL_EXT_nonuniform_qualifier : require
#extension GL_EXT_spirv_intrinsics : require
#extension GL_EXT_samplerless_texture_functions : require
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

layout(set = 0, binding = 22, std140) uniform type_Globals
{
    uvec2 hzb_tex_inner_dims;
} _Globals;

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
        vec3 _155 = instance_buffer._m0[gl_GlobalInvocationID.x].position;
        int _163 = instance_buffer._m0[gl_GlobalInvocationID.x].mesh_atlas_part_idx;
        float _166 = (instance_buffer._m0[gl_GlobalInvocationID.x].is_straight > 0) ? instance_buffer._m0[gl_GlobalInvocationID.x].angle1 : instance_buffer._m0[gl_GlobalInvocationID.x].radius1;
        vec4 _173 = vec4(_155, 1.0) * frame_constants.view_constants[0u].world_to_view;
        uint _180 = clamp(uint(clamp((32.0 - _173.z) * 0.00347222224809229373931884765625, 0.0, 1.0) * 1024.0), 0u, 1023u);
        mat4 _137 = frame_constants.view_constants[0u].view_to_clip;
        vec2 _135 = vec2(1.0);
        vec2 _136 = vec2(0.0);
        vec4 _197 = vec4((-_137[0u].x) * _135.x, _137[0u].y, _136.x, _137[0u].w);
        vec4 _209 = vec4(_137[1u].x, _137[1u].y * _135.y, _136.y, _137[1u].w);
        vec4 _216 = vec4(_137[3u].x, _137[3u].y, -1.0, _137[3u].w);
        vec4 _131[4];
        _131[0] = _216 + _197;
        _131[1] = _216 - _197;
        _131[2] = _216 + _209;
        _131[3] = _216 - _209;
        for (int _228 = 0; _228 < 4; )
        {
            _131[_228] /= vec4(length(_131[_228].xyz));
            _228++;
            continue;
        }
        vec4 _130[4] = _131;
        vec3 _243 = _173.xyz;
        vec3 _138 = _243;
        vec3 _139 = _243;
        bool _255;
        if (((-_139.z) + _166) > _137[2].w)
        {
            _255 = ((-_139.z) - _166) < 3.4028234663852885981170418348452e+38;
        }
        else
        {
            _255 = false;
        }
        bool _257;
        _257 = _255;
        bool _258;
        for (int _260 = 0; _260 < 4; _257 = _258, _260++)
        {
            if (_257)
            {
                _258 = dot(vec4(_173.xyz, 1.0), _130[_260]) > (-_166);
            }
            else
            {
                _258 = false;
            }
        }
        uint _276 = draw_list_info._m0[push_constants.draw_list_index_base].global_instance_offset;
        uint _277 = gl_GlobalInvocationID.x + _276;
        vec3 _141 = _243;
        _141.z = -_141.z;
        mat4 _133 = frame_constants.view_constants[0u].view_to_clip;
        float _286 = _133[0u].x * 0.5;
        mat4 _132 = frame_constants.view_constants[0u].view_to_clip;
        float _289 = _132[1u].y * 0.5;
        vec3 _134 = _141 * _166;
        float _296 = (_141.z * _141.z) - (_166 * _166);
        float _302 = sqrt((_141.x * _141.x) + _296);
        float _328 = sqrt((_141.y * _141.y) + _296);
        vec4 _359 = vec4(_Globals.hzb_tex_inner_dims.xyxy);
        vec4 _362 = clamp((vec4((((_302 * _141.x) - _134.z) / ((_302 * _141.z) + _134.x)) * _286, (-(((_328 * _141.y) + _134.z) / ((_328 * _141.z) - _134.y))) * _289, (((_302 * _141.x) + _134.z) / ((_302 * _141.z) - _134.x)) * _286, (-(((_328 * _141.y) - _134.z) / ((_328 * _141.z) + _134.y))) * _289) + vec4(0.5)) * _359, vec4(0.0), _359 - vec4(1.0));
        vec4 _140 = _362;
        vec2 _142 = vec2(_140.z - _140.x, _140.w - _140.y);
        uint _382 = uint(spvNMax(0.0, ceil(log2(spvNMax(_142.x, _142.y)))));
        bool _430;
        if (_257 ? true : false)
        {
            uvec2 _389 = uvec2(_382) & uvec2(31u);
            uvec2 _143 = uvec2(_362.xy) >> _389;
            uvec2 _144 = uvec2(_362.zw) >> _389;
            mat4 _129 = frame_constants.view_constants[0u].clip_to_view;
            _430 = (_138.z + _166) > (1.0 / (spvNMin(spvNMin(texelFetch(hzb_tex, ivec2(uvec2(_143.x, _143.y)), int(_382)).x, texelFetch(hzb_tex, ivec2(uvec2(_143.x, _144.y)), int(_382)).x), spvNMin(texelFetch(hzb_tex, ivec2(uvec2(_144.x, _143.y)), int(_382)).x, texelFetch(hzb_tex, ivec2(uvec2(_144.x, _144.y)), int(_382)).x)) * (-_129[3].z)));
        }
        else
        {
            _430 = _257;
        }
        if (_430)
        {
            uint _433 = _277 / 32u;
            uint _437 = 1u << ((_277 % 32u) & 31u);
            uint _438 = atomicOr(_alias_next_visibility_bitmask_1._m0[_433], _437);
            if (!((prev_visibility_bitmask._m0[_433] & _437) != 0u))
            {
                uint _447 = atomicAdd(_alias_filtered_instance_allocator_buffer_1._m0[push_constants.draw_list_index_base], 1u);
                _alias_filtered_instance_buffer._m0[_447] = (gl_GlobalInvocationID.x * 1024u) + _180;
                _alias_filtered_instance_meta_buffer._m0[_447] = GpuFilteredInstanceMeta(uint(_163));
                uint _454 = atomicAdd(_alias_output_histogram_1._m0[(push_constants.draw_list_index_base * 1024u) + _180], 1u);
            }
        }
        break;
    } while(false);
}

