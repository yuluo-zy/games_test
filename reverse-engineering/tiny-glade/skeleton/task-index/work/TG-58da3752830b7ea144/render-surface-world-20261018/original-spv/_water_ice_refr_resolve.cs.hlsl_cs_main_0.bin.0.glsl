#version 450
#extension GL_EXT_nonuniform_qualifier : require
#extension GL_EXT_samplerless_texture_functions : require
#extension GL_EXT_spirv_intrinsics : require
layout(local_size_x = 8, local_size_y = 8, local_size_z = 1) in;

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

layout(set = 0, binding = 13, std140) uniform type_Globals
{
    uvec2 depth_tex_dims;
    uvec2 refr_hit_tex_dims;
    uvec2 refr_hit_radiance_tex_dims;
    uvec2 refl_refr_radiance_out_tex_dims;
} _Globals;

layout(set = 0, binding = 14) uniform texture2D depth_tex;
layout(set = 0, binding = 15) uniform utexture2D refr_hit_tex;
layout(set = 0, binding = 16) uniform texture2D refr_hit_radiance_tex;
layout(set = 0, binding = 17, r11f_g11f_b10f) uniform writeonly image2D refl_refr_radiance_out_tex;

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
        uvec4 _116 = texelFetch(refr_hit_tex, ivec2(gl_GlobalInvocationID.xy), int(0u));
        uint _117 = _116.x;
        if (0u == _117)
        {
            break;
        }
        vec3 _134 = vec3((float((_117 >> 24u) & 255u) * 0.007843137718737125396728515625) - 1.0, (float((_117 >> 16u) & 255u) * 0.007843137718737125396728515625) - 1.0, 0.0);
        vec2 _135 = _134.xy;
        _134.z = intBitsToFloat(532487669 + (floatBitsToInt(spvNMax(0.0, 1.0 - dot(_135, _135))) >> 1));
        vec2 _160 = (((vec2(ivec2(gl_GlobalInvocationID.xy)) + vec2(0.5)) / vec2(_Globals.depth_tex_dims)) - vec2(0.5)) * vec2(2.0, -2.0);
        float _161 = _160.x;
        vec4 _164 = vec4(_161, _160.y, 0.0, 1.0) * frame_constants.view_constants[0u].sample_to_view;
        vec4 _168 = (vec4(_161, _160.y, texelFetch(depth_tex, ivec2(gl_GlobalInvocationID.xy), int(0u)).x, 1.0) * frame_constants.view_constants[0u].sample_to_view) * frame_constants.view_constants[0u].view_to_world;
        vec3 _172 = _168.xyz / vec3(_168.w);
        vec3 _183 = normalize(normalize((_164 * frame_constants.view_constants[0u].view_to_world).xyz));
        vec3 _185 = mat3(vec3(1.0, -0.0, 0.0), vec3(-0.0, 0.0, 1.0), vec3(-0.0, -1.0, 0.0)) * (-_183);
        float _186 = length(_134 * unpackHalf2x16(_117 & 65535u).x);
        float _187 = length((vec4(_172, 1.0) * frame_constants.view_constants[0u].world_to_view).xyz);
        float _191 = -normalize(_164.xyz).z;
        float _201 = spvNMax(_186, (((_187 / _191) * frame_constants.view_constants[0u].clip_to_view[1].y) * 0.20000000298023223876953125) * smoothstep(0.0, 0.0500000007450580596923828125 * _187, _186));
        float _208 = (_187 * _191) * frame_constants.view_constants[0u].clip_to_view[1].y;
        vec3 _221 = reflect(-normalize(mix(vec3(0.0, 1.0, 0.0), reflect(_183, vec3(0.0, 1.0, 0.0)), vec3(0.959011614322662353515625))), vec3(0.0, 1.0, 0.0));
        vec3 _224 = normalize(cross(vec3(0.0, 1.0, 0.0), _221)) * spvNMax(spvNMin((((_201 / (_201 + _187)) * _187) * _191) * 0.0684653222560882568359375, 0.100000001490116119384765625 * _208), (4.0 / float(_Globals.refl_refr_radiance_out_tex_dims.y)) * _208);
        vec3 _225 = cross(_221, _224);
        float _236 = (float((frame_constants.frame_index * 59u) % 128u) * 1.324717998504638671875) + (dot(vec2(gl_GlobalInvocationID.xy & uvec2(1u)), vec2(1.0, 2.0)) * 0.654508531093597412109375);
        vec4 _238;
        _238 = vec4(0.0);
        vec4 _239;
        float _241 = 0.0;
        int _243 = 0;
        for (; uint(_243) < 8u; _238 = _239, _241 += 0.125, _243++)
        {
            float _251 = (float(_243) + _236) * 2.3999631404876708984375;
            vec3 _258 = _172 + (((_224 * cos(_251)) + (_225 * sin(_251))) * _241);
            vec4 _266 = (vec4(_258, 1.0) * frame_constants.view_constants[0u].world_to_view) * frame_constants.view_constants[0u].view_to_sample;
            uvec2 _280 = uvec2(ivec2(floor((((_266.xyz / vec3(_266.w)).xy * vec2(0.5, -0.5)) + vec2(0.5)) * vec2(_Globals.refr_hit_tex_dims))));
            uvec4 _281 = texelFetch(refr_hit_tex, ivec2(_280), int(0u));
            uint _282 = _281.x;
            if (_282 != 0u)
            {
                vec3 _299 = vec3((float((_282 >> 24u) & 255u) * 0.007843137718737125396728515625) - 1.0, (float((_282 >> 16u) & 255u) * 0.007843137718737125396728515625) - 1.0, 0.0);
                vec2 _300 = _299.xy;
                _299.z = intBitsToFloat(532487669 + (floatBitsToInt(spvNMax(0.0, 1.0 - dot(_300, _300))) >> 1));
                vec3 _315 = normalize(_185 + normalize((_299 * unpackHalf2x16(_282 & 65535u).x) + (mat3(vec3(1.0, -0.0, 0.0), vec3(-0.0, 0.0, 1.0), vec3(-0.0, -1.0, 0.0)) * (_258 - _172))));
                float _316 = _315.z;
                float _319 = ((_316 * _316) * (-0.9943749904632568359375)) + 1.0;
                _239 = _238 + (vec4(texelFetch(refr_hit_radiance_tex, ivec2(_280), int(0u)).xyz, 1.0) * (3.1640625820728018879890441894531e-05 / (_319 * _319)));
            }
            else
            {
                _239 = _238;
            }
        }
        imageStore(refl_refr_radiance_out_tex, ivec2(gl_GlobalInvocationID.xy), (_238.xyz / vec3(spvNMax(9.9999998245167004418121337039338e-15, _238.w))).xyz.xyzz);
        break;
    } while(false);
}

