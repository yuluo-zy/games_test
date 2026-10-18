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

struct BushXform
{
    Affine3Packed transform;
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

layout(set = 0, binding = 23, std430) readonly buffer type_StructuredBuffer_BushXform
{
    BushXform _m0[];
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
        BushXform _146 = instance_buffer._m0[gl_GlobalInvocationID.x];
        mat3x4 _166 = transpose(mat4x3(vec3(_146.transform.data[0], _146.transform.data[1], _146.transform.data[2]), vec3(_146.transform.data[3], _146.transform.data[4], _146.transform.data[5]), vec3(_146.transform.data[6], _146.transform.data[7], _146.transform.data[8]), vec3(_146.transform.data[9], _146.transform.data[10], _146.transform.data[11])));
        vec3 _169 = _166[0].xyz;
        vec3 _172 = _166[1].xyz;
        vec3 _175 = _166[2].xyz;
        float _184 = sqrt(spvNMax(spvNMax(dot(_169, _169), dot(_172, _172)), dot(_175, _175))) * atlas_part_info._m0[push_constants.world_subset][0u].bounding_sphere_radius;
        vec4 _191 = vec4(vec4(0.0, 0.0, 0.0, 1.0) * _166, 1.0) * frame_constants.view_constants[0u].world_to_view;
        float _192 = _191.z;
        uint _198 = clamp(uint(clamp((32.0 - _192) * 0.00347222224809229373931884765625, 0.0, 1.0) * 1024.0), 0u, 1023u);
        vec4 _207 = vec4(-frame_constants.view_constants[0u].view_to_clip[0].x, frame_constants.view_constants[0u].view_to_clip[0].y, 0.0, frame_constants.view_constants[0u].view_to_clip[0].w);
        vec4 _211 = vec4(frame_constants.view_constants[0u].view_to_clip[1].x, frame_constants.view_constants[0u].view_to_clip[1].y, 0.0, frame_constants.view_constants[0u].view_to_clip[1].w);
        vec4 _215 = vec4(frame_constants.view_constants[0u].view_to_clip[3].x, frame_constants.view_constants[0u].view_to_clip[3].y, -1.0, frame_constants.view_constants[0u].view_to_clip[3].w);
        vec4 _135[4];
        _135[0] = _215 + _207;
        _135[1] = _215 - _207;
        _135[2] = _215 + _211;
        _135[3] = _215 - _211;
        for (int _226 = 0; _226 < 4; )
        {
            _135[_226] /= vec4(length(_135[_226].xyz));
            _226++;
            continue;
        }
        vec4 _134[4] = _135;
        vec3 _241 = _191.xyz;
        float _242 = -_192;
        bool _249;
        if ((_242 + _184) > frame_constants.view_constants[0u].view_to_clip[2].w)
        {
            _249 = (_242 - _184) < 3.4028234663852885981170418348452e+38;
        }
        else
        {
            _249 = false;
        }
        bool _251;
        _251 = _249;
        bool _252;
        for (int _254 = 0; _254 < 4; _251 = _252, _254++)
        {
            if (_251)
            {
                _252 = dot(vec4(_191.xy, _192, 1.0), _134[_254]) > (-_184);
            }
            else
            {
                _252 = false;
            }
        }
        uint _270 = draw_list_info._m0[push_constants.draw_list_index_base].global_instance_offset;
        uint _271 = gl_GlobalInvocationID.x + _270;
        _241.z = _242;
        float _274 = frame_constants.view_constants[0u].view_to_clip[0].x * 0.5;
        float _275 = frame_constants.view_constants[0u].view_to_clip[1].y * 0.5;
        vec3 _276 = _241 * _184;
        float _279 = (_242 * _242) - (_184 * _184);
        float _280 = _191.x;
        float _283 = sqrt((_280 * _280) + _279);
        float _284 = _283 * _280;
        float _285 = _276.z;
        float _287 = _283 * _242;
        float _288 = _276.x;
        float _294 = _191.y;
        float _297 = sqrt((_294 * _294) + _279);
        float _298 = _297 * _294;
        float _300 = _297 * _242;
        float _301 = _276.y;
        vec4 _318 = vec4(_Globals.hzb_tex_inner_dims.xyxy);
        vec4 _321 = clamp((vec4(((_284 - _285) / (_287 + _288)) * _274, (-((_298 + _285) / (_300 - _301))) * _275, ((_284 + _285) / (_287 - _288)) * _274, (-((_298 - _285) / (_300 + _301))) * _275) + vec4(0.5)) * _318, vec4(0.0), _318 - vec4(1.0));
        uint _332 = uint(spvNMax(0.0, ceil(log2(spvNMax(_321.z - _321.x, _321.w - _321.y)))));
        bool _369;
        if (_251 ? true : false)
        {
            uvec2 _339 = uvec2(_332) & uvec2(31u);
            uvec2 _340 = uvec2(_321.xy) >> _339;
            uvec2 _343 = uvec2(_321.zw) >> _339;
            _369 = (_192 + _184) > (1.0 / (spvNMin(spvNMin(texelFetch(hzb_tex, ivec2(_340), int(_332)).x, texelFetch(hzb_tex, ivec2(uvec2(_340.x, _343.y)), int(_332)).x), spvNMin(texelFetch(hzb_tex, ivec2(uvec2(_343.x, _340.y)), int(_332)).x, texelFetch(hzb_tex, ivec2(_343), int(_332)).x)) * (-frame_constants.view_constants[0u].clip_to_view[3].z)));
        }
        else
        {
            _369 = _251;
        }
        if (_369)
        {
            uint _372 = _271 / 32u;
            uint _376 = 1u << ((_271 % 32u) & 31u);
            uint _377 = atomicOr(_alias_next_visibility_bitmask_1._m0[_372], _376);
            if (!((prev_visibility_bitmask._m0[_372] & _376) != 0u))
            {
                uint _386 = atomicAdd(_alias_filtered_instance_allocator_buffer_1._m0[push_constants.draw_list_index_base], 1u);
                _alias_filtered_instance_buffer._m0[_386] = (gl_GlobalInvocationID.x * 1024u) + _198;
                _alias_filtered_instance_meta_buffer._m0[_386] = GpuFilteredInstanceMeta(0u);
                uint _392 = atomicAdd(_alias_output_histogram_1._m0[(push_constants.draw_list_index_base * 1024u) + _198], 1u);
            }
        }
        break;
    } while(false);
}

