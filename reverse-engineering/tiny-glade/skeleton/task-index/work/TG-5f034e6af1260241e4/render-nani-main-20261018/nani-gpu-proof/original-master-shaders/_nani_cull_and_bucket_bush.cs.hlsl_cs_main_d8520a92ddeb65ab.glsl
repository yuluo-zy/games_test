#version 450
#extension GL_EXT_nonuniform_qualifier : require
#extension GL_EXT_spirv_intrinsics : require
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
        float _183 = atlas_part_info._m0[push_constants.world_subset][0u].bounding_sphere_radius;
        float _184 = sqrt(spvNMax(spvNMax(dot(_169, _169), dot(_172, _172)), dot(_175, _175))) * _183;
        vec4 _190 = vec4(vec4(0.0, 0.0, 0.0, 1.0) * _166, 1.0);
        vec4 _191 = _190 * frame_constants.view_constants[0u].world_to_view;
        float _192 = _191.z;
        float _193 = -_192;
        uint _213 = clamp(uint(clamp((32.0 - _192) * 0.00347222224809229373931884765625, 0.0, 1.0) * 1024.0), 0u, 1023u);
        uint _214 = gl_GlobalInvocationID.x * 1024u;
        vec4 _220 = vec4(-frame_constants.view_constants[0u].view_to_clip[0].x, frame_constants.view_constants[0u].view_to_clip[0].y, 0.0, frame_constants.view_constants[0u].view_to_clip[0].w);
        vec4 _223 = vec4(frame_constants.view_constants[0u].view_to_clip[1].x, frame_constants.view_constants[0u].view_to_clip[1].y, 0.0, frame_constants.view_constants[0u].view_to_clip[1].w);
        vec4 _227 = vec4(frame_constants.view_constants[0u].view_to_clip[3].x, frame_constants.view_constants[0u].view_to_clip[3].y, -1.0, frame_constants.view_constants[0u].view_to_clip[3].w);
        vec4 _135[4];
        _135[0] = _227 + _220;
        _135[1] = _227 - _220;
        _135[2] = _227 + _223;
        _135[3] = _227 - _223;
        for (int _238 = 0; _238 < 4; )
        {
            _135[_238] /= vec4(length(_135[_238].xyz));
            _238++;
            continue;
        }
        vec4 _134[4] = _135;
        bool _259;
        if ((_193 + _184) > frame_constants.view_constants[0u].view_to_clip[2].w)
        {
            _259 = (_193 - _184) < 3.4028234663852885981170418348452e+38;
        }
        else
        {
            _259 = false;
        }
        bool _261;
        _261 = _259;
        bool _262;
        for (int _264 = 0; _264 < 4; _261 = _262, _264++)
        {
            if (_261)
            {
                _262 = dot(vec4(_191.xy, _192, 1.0), _134[_264]) > (-_184);
            }
            else
            {
                _262 = false;
            }
        }
        uint _281 = gl_GlobalInvocationID.x + draw_list_info._m0[push_constants.draw_list_index_base].global_instance_offset;
        bool _292;
        if (_261)
        {
            _292 = (prev_visibility_bitmask._m0[_281 / 32u] & (1u << ((_281 % 32u) & 31u))) != 0u;
        }
        else
        {
            _292 = false;
        }
        if (_292)
        {
            uint _300 = atomicOr(_alias_next_visibility_bitmask_1._m0[_281 / 32u], 1u << ((_281 % 32u) & 31u));
            uint _302 = atomicAdd(_alias_filtered_instance_allocator_buffer_1._m0[push_constants.draw_list_index_base], 1u);
            _alias_filtered_instance_buffer._m0[_302] = _214 + _213;
            _alias_filtered_instance_meta_buffer._m0[_302] = GpuFilteredInstanceMeta(0u);
            uint _308 = atomicAdd(_alias_output_histogram_1._m0[(push_constants.draw_list_index_base * 1024u) + _213], 1u);
        }
        if ((((_184 / spvNMax(9.9999997473787516355514526367188e-06, _193)) * ((0.5 * frame_constants.view_constants[0u].view_to_clip[1].y) * float(clamp(frame_constants.inner_rendering_resolution.y, 1080u, 2160u)))) < 10.0) ? false : true)
        {
            vec4 _316 = (_190 * frame_constants.view_constants[2u].world_to_view) * frame_constants.view_constants[2u].view_to_clip;
            uint _324 = clamp(uint((_316.xyz / vec3(_316.w)).z * 1024.0), 0u, 1023u);
            uint _325 = _214 + _324;
            float _334 = _184 * (2.0 * frame_constants.view_constants[2u].view_to_sample[0].x);
            vec2 _335 = abs((_190 * frame_constants.view_constants[2u].world_to_sample).xy);
            uint _345 = (any(greaterThanEqual(_335, vec2(-_334))) ? all(lessThanEqual(_335, vec2(1.0 + _334))) : false) ? 0u : 3u;
            bool _354 = any(greaterThanEqual(_335, vec2(1.0 - _334))) ? all(lessThanEqual(_335, vec2(2.0 + _334))) : false;
            uint _358;
            if (_354)
            {
                _358 = min(_345, 1u);
            }
            else
            {
                _358 = _345;
            }
            uint _359 = uint(_354);
            uint _373;
            uint _374;
            if (any(greaterThanEqual(_335, vec2(2.0 - _334))) ? all(lessThanEqual(_335, vec2(7.0 + _334))) : false)
            {
                _373 = max(_359, 2u);
                _374 = min(_358, 2u);
            }
            else
            {
                _373 = _359;
                _374 = _358;
            }
            for (uint _376 = _374; _376 <= _373; )
            {
                uint _382 = (push_constants.draw_list_index_base + 1u) + _376;
                uint _384 = atomicAdd(_alias_filtered_instance_allocator_buffer_1._m0[_382], 1u);
                _alias_filtered_instance_buffer._m0[_384] = _325;
                _alias_filtered_instance_meta_buffer._m0[_384] = GpuFilteredInstanceMeta(0u);
                uint _390 = atomicAdd(_alias_output_histogram_1._m0[(_382 * 1024u) + _324], 1u);
                _376++;
                continue;
            }
        }
        break;
    } while(false);
}

