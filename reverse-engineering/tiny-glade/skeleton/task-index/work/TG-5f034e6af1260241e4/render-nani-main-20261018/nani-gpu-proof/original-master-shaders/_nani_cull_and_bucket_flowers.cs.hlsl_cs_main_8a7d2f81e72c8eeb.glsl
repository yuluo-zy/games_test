#version 450
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

struct Affine3Packed
{
    float data[12];
};

struct FlowerXform
{
    Affine3Packed transform;
    vec3 color;
    uint flower_type;
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

layout(set = 0, binding = 23, std430) readonly buffer type_StructuredBuffer_FlowerXform
{
    FlowerXform _m0[];
} instance_buffer;

layout(set = 0, binding = 24, std430) readonly buffer type_StructuredBuffer_v4uint
{
    uvec4 _m0[];
} dispatch_args_buffer;

layout(push_constant, std430) uniform type_ConstantBuffer_PushConstants
{
    uint world_subset;
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
        if (gl_GlobalInvocationID.x >= dispatch_args_buffer._m0[0u].w)
        {
            break;
        }
        Affine3Packed _146 = instance_buffer._m0[gl_GlobalInvocationID.x].transform;
        uint _148 = instance_buffer._m0[gl_GlobalInvocationID.x].flower_type;
        mat3x4 _167 = transpose(mat4x3(vec3(_146.data[0], _146.data[1], _146.data[2]), vec3(_146.data[3], _146.data[4], _146.data[5]), vec3(_146.data[6], _146.data[7], _146.data[8]), vec3(_146.data[9], _146.data[10], _146.data[11])));
        vec3 _170 = _167[0].xyz;
        vec3 _173 = _167[1].xyz;
        vec3 _176 = _167[2].xyz;
        float _185 = sqrt(spvNMax(spvNMax(dot(_170, _170), dot(_173, _173)), dot(_176, _176))) * atlas_part_info._m0[push_constants.world_subset][_148].bounding_sphere_radius;
        vec4 _192 = vec4(vec4(0.0, 0.0, 0.0, 1.0) * _167, 1.0) * frame_constants.view_constants[0u].world_to_view;
        float _193 = _192.z;
        uint _199 = clamp(uint(clamp((32.0 - _193) * 0.00347222224809229373931884765625, 0.0, 1.0) * 1024.0), 0u, 1023u);
        vec4 _208 = vec4(-frame_constants.view_constants[0u].view_to_clip[0].x, frame_constants.view_constants[0u].view_to_clip[0].y, 0.0, frame_constants.view_constants[0u].view_to_clip[0].w);
        vec4 _212 = vec4(frame_constants.view_constants[0u].view_to_clip[1].x, frame_constants.view_constants[0u].view_to_clip[1].y, 0.0, frame_constants.view_constants[0u].view_to_clip[1].w);
        vec4 _216 = vec4(frame_constants.view_constants[0u].view_to_clip[3].x, frame_constants.view_constants[0u].view_to_clip[3].y, -1.0, frame_constants.view_constants[0u].view_to_clip[3].w);
        vec4 _135[4];
        _135[0] = _216 + _208;
        _135[1] = _216 - _208;
        _135[2] = _216 + _212;
        _135[3] = _216 - _212;
        for (int _227 = 0; _227 < 4; )
        {
            _135[_227] /= vec4(length(_135[_227].xyz));
            _227++;
            continue;
        }
        vec4 _134[4] = _135;
        vec3 _242 = _192.xyz;
        float _243 = -_193;
        bool _250;
        if ((_243 + _185) > frame_constants.view_constants[0u].view_to_clip[2].w)
        {
            _250 = (_243 - _185) < 3.4028234663852885981170418348452e+38;
        }
        else
        {
            _250 = false;
        }
        bool _252;
        _252 = _250;
        bool _253;
        for (int _255 = 0; _255 < 4; _252 = _253, _255++)
        {
            if (_252)
            {
                _253 = dot(vec4(_192.xy, _193, 1.0), _134[_255]) > (-_185);
            }
            else
            {
                _253 = false;
            }
        }
        uint _271 = draw_list_info._m0[push_constants.draw_list_index_base].global_instance_offset;
        uint _272 = gl_GlobalInvocationID.x + _271;
        _242.z = _243;
        float _275 = frame_constants.view_constants[0u].view_to_clip[0].x * 0.5;
        float _276 = frame_constants.view_constants[0u].view_to_clip[1].y * 0.5;
        vec3 _277 = _242 * _185;
        float _280 = (_243 * _243) - (_185 * _185);
        float _281 = _192.x;
        float _284 = sqrt((_281 * _281) + _280);
        float _285 = _284 * _281;
        float _286 = _277.z;
        float _288 = _284 * _243;
        float _289 = _277.x;
        float _295 = _192.y;
        float _298 = sqrt((_295 * _295) + _280);
        float _299 = _298 * _295;
        float _301 = _298 * _243;
        float _302 = _277.y;
        vec4 _319 = vec4(_Globals.hzb_tex_inner_dims.xyxy);
        vec4 _322 = clamp((vec4(((_285 - _286) / (_288 + _289)) * _275, (-((_299 + _286) / (_301 - _302))) * _276, ((_285 + _286) / (_288 - _289)) * _275, (-((_299 - _286) / (_301 + _302))) * _276) + vec4(0.5)) * _319, vec4(0.0), _319 - vec4(1.0));
        uint _333 = uint(spvNMax(0.0, ceil(log2(spvNMax(_322.z - _322.x, _322.w - _322.y)))));
        bool _370;
        if (_252 ? true : false)
        {
            uvec2 _340 = uvec2(_333) & uvec2(31u);
            uvec2 _341 = uvec2(_322.xy) >> _340;
            uvec2 _344 = uvec2(_322.zw) >> _340;
            _370 = (_193 + _185) > (1.0 / (spvNMin(spvNMin(texelFetch(hzb_tex, ivec2(_341), int(_333)).x, texelFetch(hzb_tex, ivec2(uvec2(_341.x, _344.y)), int(_333)).x), spvNMin(texelFetch(hzb_tex, ivec2(uvec2(_344.x, _341.y)), int(_333)).x, texelFetch(hzb_tex, ivec2(_344), int(_333)).x)) * (-frame_constants.view_constants[0u].clip_to_view[3].z)));
        }
        else
        {
            _370 = _252;
        }
        if (_370)
        {
            uint _373 = _272 / 32u;
            uint _377 = 1u << ((_272 % 32u) & 31u);
            uint _378 = atomicOr(_alias_next_visibility_bitmask_1._m0[_373], _377);
            if (!((prev_visibility_bitmask._m0[_373] & _377) != 0u))
            {
                uint _387 = atomicAdd(_alias_filtered_instance_allocator_buffer_1._m0[push_constants.draw_list_index_base], 1u);
                _alias_filtered_instance_buffer._m0[_387] = (gl_GlobalInvocationID.x * 1024u) + _199;
                _alias_filtered_instance_meta_buffer._m0[_387] = GpuFilteredInstanceMeta(_148);
                uint _394 = atomicAdd(_alias_output_histogram_1._m0[(push_constants.draw_list_index_base * 1024u) + _199], 1u);
            }
        }
        break;
    } while(false);
}

