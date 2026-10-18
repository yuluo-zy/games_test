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

struct GpuFilteredInstanceMeta
{
    uint raw;
};

struct NaniDrawListInfo
{
    uint global_instance_offset;
};

struct RoofShingle
{
    vec3 base_position;
    float current_height;
    uvec2 clipping_plane_;
    uvec2 scale_t;
    uvec2 rotation_quat_;
    uint roof_id;
    uint velocity_rotation;
    float animation_t;
    int seed;
    int is_circle;
    int is_main_roof_body;
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

layout(set = 0, binding = 23, std430) readonly buffer type_StructuredBuffer_RoofShingle
{
    RoofShingle _m0[];
} instance_buffer;

layout(push_constant, std430) uniform type_ConstantBuffer_PushConstants
{
    uint world_subset;
    uint first_instance;
    uint instance_count;
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
        if (gl_GlobalInvocationID.x >= push_constants.instance_count)
        {
            break;
        }
        uint _138 = push_constants.first_instance + gl_GlobalInvocationID.x;
        vec3 _140 = instance_buffer._m0[_138].base_position;
        float _142 = instance_buffer._m0[_138].current_height;
        int _146 = instance_buffer._m0[_138].is_main_roof_body;
        float _162 = length(vec3(unpackHalf2x16(instance_buffer._m0[_138].scale_t.x & 65535u).x, unpackHalf2x16((instance_buffer._m0[_138].scale_t.x >> 16u) & 65535u).x, unpackHalf2x16(instance_buffer._m0[_138].scale_t.y & 65535u).x));
        float _163 = 0.59403002262115478515625 * _162;
        vec4 _170 = vec4(_140 + vec3(0.0, _142, 0.0), 1.0) * frame_constants.view_constants[0u].world_to_view;
        float _171 = _170.z;
        float _172 = -_171;
        uint _184 = clamp(uint(clamp((32.0 - _171) * 0.00347222224809229373931884765625, 0.0, 1.0) * 1024.0), 0u, 1023u);
        vec4 _193 = vec4(-frame_constants.view_constants[0u].view_to_clip[0].x, frame_constants.view_constants[0u].view_to_clip[0].y, 0.0, frame_constants.view_constants[0u].view_to_clip[0].w);
        vec4 _197 = vec4(frame_constants.view_constants[0u].view_to_clip[1].x, frame_constants.view_constants[0u].view_to_clip[1].y, 0.0, frame_constants.view_constants[0u].view_to_clip[1].w);
        vec4 _201 = vec4(frame_constants.view_constants[0u].view_to_clip[3].x, frame_constants.view_constants[0u].view_to_clip[3].y, -1.0, frame_constants.view_constants[0u].view_to_clip[3].w);
        vec4 _126[4];
        _126[0] = _201 + _193;
        _126[1] = _201 - _193;
        _126[2] = _201 + _197;
        _126[3] = _201 - _197;
        for (int _212 = 0; _212 < 4; )
        {
            _126[_212] /= vec4(length(_126[_212].xyz));
            _212++;
            continue;
        }
        vec4 _125[4] = _126;
        vec3 _227 = _170.xyz;
        bool _234;
        if ((_172 + _163) > frame_constants.view_constants[0u].view_to_clip[2].w)
        {
            _234 = (_172 - _163) < 3.4028234663852885981170418348452e+38;
        }
        else
        {
            _234 = false;
        }
        bool _236;
        _236 = _234;
        bool _237;
        for (int _239 = 0; _239 < 4; _236 = _237, _239++)
        {
            if (_236)
            {
                _237 = dot(vec4(_170.xy, _171, 1.0), _125[_239]) > (_162 * (-0.59403002262115478515625));
            }
            else
            {
                _237 = false;
            }
        }
        uint _255 = draw_list_info._m0[push_constants.draw_list_index_base].global_instance_offset;
        uint _256 = _138 + _255;
        _227.z = _172;
        float _259 = frame_constants.view_constants[0u].view_to_clip[0].x * 0.5;
        float _260 = frame_constants.view_constants[0u].view_to_clip[1].y * 0.5;
        vec3 _261 = _227 * _163;
        float _264 = (_172 * _172) - (_163 * _163);
        float _265 = _170.x;
        float _268 = sqrt((_265 * _265) + _264);
        float _269 = _268 * _265;
        float _270 = _261.z;
        float _272 = _268 * _172;
        float _273 = _261.x;
        float _279 = _170.y;
        float _282 = sqrt((_279 * _279) + _264);
        float _283 = _282 * _279;
        float _285 = _282 * _172;
        float _286 = _261.y;
        vec4 _303 = vec4(_Globals.hzb_tex_inner_dims.xyxy);
        vec4 _306 = clamp((vec4(((_269 - _270) / (_272 + _273)) * _259, (-((_283 + _270) / (_285 - _286))) * _260, ((_269 + _270) / (_272 - _273)) * _259, (-((_283 - _270) / (_285 + _286))) * _260) + vec4(0.5)) * _303, vec4(0.0), _303 - vec4(1.0));
        uint _317 = uint(spvNMax(0.0, ceil(log2(spvNMax(_306.z - _306.x, _306.w - _306.y)))));
        bool _354;
        if (_236 ? true : false)
        {
            uvec2 _324 = uvec2(_317) & uvec2(31u);
            uvec2 _325 = uvec2(_306.xy) >> _324;
            uvec2 _328 = uvec2(_306.zw) >> _324;
            _354 = (_171 + _163) > (1.0 / (spvNMin(spvNMin(texelFetch(hzb_tex, ivec2(_325), int(_317)).x, texelFetch(hzb_tex, ivec2(uvec2(_325.x, _328.y)), int(_317)).x), spvNMin(texelFetch(hzb_tex, ivec2(uvec2(_328.x, _325.y)), int(_317)).x, texelFetch(hzb_tex, ivec2(_328), int(_317)).x)) * (-frame_constants.view_constants[0u].clip_to_view[3].z)));
        }
        else
        {
            _354 = _236;
        }
        if (_354)
        {
            uint _357 = _256 / 32u;
            uint _361 = 1u << ((_256 % 32u) & 31u);
            uint _362 = atomicOr(_alias_next_visibility_bitmask_1._m0[_357], _361);
            if (!((prev_visibility_bitmask._m0[_357] & _361) != 0u))
            {
                uint _371 = atomicAdd(_alias_filtered_instance_allocator_buffer_1._m0[push_constants.draw_list_index_base], 1u);
                _alias_filtered_instance_buffer._m0[_371] = (_138 * 1024u) + _184;
                _alias_filtered_instance_meta_buffer._m0[_371] = GpuFilteredInstanceMeta((_146 == 0) ? 2u : ((_146 == 2) ? 2u : uint(_172 > 70.0)));
                uint _378 = atomicAdd(_alias_output_histogram_1._m0[(push_constants.draw_list_index_base * 1024u) + _184], 1u);
            }
        }
        break;
    } while(false);
}

