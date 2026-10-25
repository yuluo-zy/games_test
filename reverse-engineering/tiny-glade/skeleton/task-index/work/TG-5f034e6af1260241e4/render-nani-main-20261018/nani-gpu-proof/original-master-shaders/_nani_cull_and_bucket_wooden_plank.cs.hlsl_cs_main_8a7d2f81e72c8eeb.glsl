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

struct Affine3Packed
{
    float data[12];
};

struct WoodenPlank
{
    Affine3Packed transform;
    int gate_seed;
    int piece_seed;
    uint wall_id;
    uint flags;
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

layout(set = 0, binding = 23, std430) readonly buffer type_StructuredBuffer_WoodenPlank
{
    WoodenPlank _m0[];
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
        uint _143 = push_constants.first_instance + gl_GlobalInvocationID.x;
        Affine3Packed _145 = instance_buffer._m0[_143].transform;
        mat3x4 _166 = transpose(mat4x3(vec3(_145.data[0], _145.data[1], _145.data[2]), vec3(_145.data[3], _145.data[4], _145.data[5]), vec3(_145.data[6], _145.data[7], _145.data[8]), vec3(_145.data[9], _145.data[10], _145.data[11])));
        vec3 _168 = _166[0].xyz;
        vec3 _171 = _166[1].xyz;
        vec3 _174 = _166[2].xyz;
        float _178 = sqrt(spvNMax(spvNMax(dot(_168, _168), dot(_171, _171)), dot(_174, _174)));
        float _182 = _178 * 0.5;
        mat3 _184 = transpose(mat3(_168, _171, _174));
        vec3 _185 = _184[0];
        float _186 = dot(_185, _185);
        vec3 _187 = _184[1];
        float _188 = dot(_187, _187);
        vec3 _189 = _184[2];
        float _190 = dot(_189, _189);
        vec4 _203 = vec4((vec4(0.0, 0.0, 0.0, 1.0) * _166) + (vec4(0.0, 0.5, 0.0, 0.0) * _166), 1.0) * frame_constants.view_constants[0u].world_to_view;
        float _204 = _203.z;
        float _205 = -_204;
        float _206 = _205 - _182;
        uint _221 = uint((((sqrt(spvNMax(spvNMin(_186, _188), spvNMax(spvNMin(_188, _190), spvNMin(_186, _190)))) / spvNMax(9.9999997473787516355514526367188e-06, _206)) * ((0.5 * frame_constants.view_constants[0u].view_to_clip[1].y) * float(clamp(frame_constants.inner_rendering_resolution.y, 1080u, 2160u)))) > 7.0) ? 0 : 1);
        bool _227;
        if ((instance_buffer._m0[_143].flags & 1u) != 0u)
        {
            _227 = _221 == 0u;
        }
        else
        {
            _227 = false;
        }
        uint _234 = clamp(uint(clamp((32.0 - _204) * 0.00347222224809229373931884765625, 0.0, 1.0) * 1024.0), 0u, 1023u);
        vec4 _241 = vec4(-frame_constants.view_constants[0u].view_to_clip[0].x, frame_constants.view_constants[0u].view_to_clip[0].y, 0.0, frame_constants.view_constants[0u].view_to_clip[0].w);
        vec4 _244 = vec4(frame_constants.view_constants[0u].view_to_clip[1].x, frame_constants.view_constants[0u].view_to_clip[1].y, 0.0, frame_constants.view_constants[0u].view_to_clip[1].w);
        vec4 _248 = vec4(frame_constants.view_constants[0u].view_to_clip[3].x, frame_constants.view_constants[0u].view_to_clip[3].y, -1.0, frame_constants.view_constants[0u].view_to_clip[3].w);
        vec4 _131[4];
        _131[0] = _248 + _241;
        _131[1] = _248 - _241;
        _131[2] = _248 + _244;
        _131[3] = _248 - _244;
        for (int _259 = 0; _259 < 4; )
        {
            _131[_259] /= vec4(length(_131[_259].xyz));
            _259++;
            continue;
        }
        vec4 _130[4] = _131;
        vec3 _274 = _203.xyz;
        bool _280;
        if ((_205 + _182) > frame_constants.view_constants[0u].view_to_clip[2].w)
        {
            _280 = _206 < 3.4028234663852885981170418348452e+38;
        }
        else
        {
            _280 = false;
        }
        bool _282;
        _282 = _280;
        bool _283;
        for (int _285 = 0; _285 < 4; _282 = _283, _285++)
        {
            if (_282)
            {
                _283 = dot(vec4(_203.xy, _204, 1.0), _130[_285]) > (_178 * (-0.5));
            }
            else
            {
                _283 = false;
            }
        }
        uint _301 = draw_list_info._m0[push_constants.draw_list_index_base].global_instance_offset;
        uint _302 = _143 + _301;
        _274.z = _205;
        float _305 = frame_constants.view_constants[0u].view_to_clip[0].x * 0.5;
        float _306 = frame_constants.view_constants[0u].view_to_clip[1].y * 0.5;
        vec3 _307 = _274 * _182;
        float _310 = (_205 * _205) - (_182 * _182);
        float _311 = _203.x;
        float _314 = sqrt((_311 * _311) + _310);
        float _315 = _314 * _311;
        float _316 = _307.z;
        float _318 = _314 * _205;
        float _319 = _307.x;
        float _325 = _203.y;
        float _328 = sqrt((_325 * _325) + _310);
        float _329 = _328 * _325;
        float _331 = _328 * _205;
        float _332 = _307.y;
        vec4 _349 = vec4(_Globals.hzb_tex_inner_dims.xyxy);
        vec4 _352 = clamp((vec4(((_315 - _316) / (_318 + _319)) * _305, (-((_329 + _316) / (_331 - _332))) * _306, ((_315 + _316) / (_318 - _319)) * _305, (-((_329 - _316) / (_331 + _332))) * _306) + vec4(0.5)) * _349, vec4(0.0), _349 - vec4(1.0));
        uint _363 = uint(spvNMax(0.0, ceil(log2(spvNMax(_352.z - _352.x, _352.w - _352.y)))));
        bool _400;
        if (_282 ? true : false)
        {
            uvec2 _370 = uvec2(_363) & uvec2(31u);
            uvec2 _371 = uvec2(_352.xy) >> _370;
            uvec2 _374 = uvec2(_352.zw) >> _370;
            _400 = (_204 + _182) > (1.0 / (spvNMin(spvNMin(texelFetch(hzb_tex, ivec2(_371), int(_363)).x, texelFetch(hzb_tex, ivec2(uvec2(_371.x, _374.y)), int(_363)).x), spvNMin(texelFetch(hzb_tex, ivec2(uvec2(_374.x, _371.y)), int(_363)).x, texelFetch(hzb_tex, ivec2(_374), int(_363)).x)) * (-frame_constants.view_constants[0u].clip_to_view[3].z)));
        }
        else
        {
            _400 = _282;
        }
        if (_400)
        {
            uint _403 = _302 / 32u;
            uint _407 = 1u << ((_302 % 32u) & 31u);
            uint _408 = atomicOr(_alias_next_visibility_bitmask_1._m0[_403], _407);
            if (!((prev_visibility_bitmask._m0[_403] & _407) != 0u))
            {
                uint _417 = atomicAdd(_alias_filtered_instance_allocator_buffer_1._m0[push_constants.draw_list_index_base], 1u);
                _alias_filtered_instance_buffer._m0[_417] = (_143 * 1024u) + _234;
                _alias_filtered_instance_meta_buffer._m0[_417] = GpuFilteredInstanceMeta(_227 ? 2u : _221);
                uint _424 = atomicAdd(_alias_output_histogram_1._m0[(push_constants.draw_list_index_base * 1024u) + _234], 1u);
            }
        }
        break;
    } while(false);
}

