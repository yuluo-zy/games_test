#version 450
#extension GL_EXT_nonuniform_qualifier : require
#extension GL_EXT_scalar_block_layout : require
#extension GL_EXT_samplerless_texture_functions : require
#extension GL_EXT_spirv_intrinsics : require
layout(local_size_x = 16, local_size_y = 16, local_size_z = 1) in;

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

float _151;

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

layout(set = 0, binding = 14, scalar) uniform type_Globals
{
    uvec2 gbuffer_tex_dims;
    uvec2 depth_tex_dims;
    uvec2 history_tex_dims;
    uvec2 reprojection_tex_dims;
    uint history_valid;
    uvec2 output_tex_dims;
} _Globals;

layout(set = 0, binding = 21, std430) buffer type_RWStructuredBuffer_uint
{
    uint _m0[];
} _alias_shadow_mask_histogram;

layout(set = 0, binding = 0) uniform sampler sampler_lnc;
layout(set = 0, binding = 11) uniform sampler sampler_shadow_nearest;
layout(set = 0, binding = 12) uniform samplerShadow sampler_shadow_linear;
layout(set = 0, binding = 15) uniform utexture2D gbuffer_tex;
layout(set = 0, binding = 16) uniform texture2D depth_tex;
layout(set = 0, binding = 17) uniform texture2D history_tex;
layout(set = 0, binding = 18) uniform texture2D reprojection_tex;
layout(set = 0, binding = 19) uniform texture2DArray shadow_map;
layout(set = 0, binding = 20, r8) uniform writeonly image2D output_tex;

shared uint thread_group_shadow_mask_sum;
shared uint thread_group_alive;

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
    bool _162 = 0u == gl_LocalInvocationIndex;
    if (_162)
    {
        thread_group_alive = 0u;
        thread_group_shadow_mask_sum = 0u;
    }
    barrier();
    uvec4 _169 = texelFetch(gbuffer_tex, ivec2(gl_GlobalInvocationID.xy), int(0u));
    uint _170 = _169.x;
    vec3 _185 = (vec4(float(_170 & 255u) * 0.0039215688593685626983642578125, float((_170 >> 8u) & 255u) * 0.0039215688593685626983642578125, float((_170 >> 16u) & 255u) * 0.0039215688593685626983642578125, _151).xyz * 2.0) - vec3(1.0);
    vec3 _189 = (vec4(0.0, 0.0, 1.0, 0.0) * frame_constants.view_constants[2u].view_to_world).xyz;
    ivec2 _190 = ivec2(gl_GlobalInvocationID.xy);
    vec2 _196 = (vec2(_190) + vec2(0.5)) / vec2(_Globals.depth_tex_dims);
    vec2 _201 = _196 - vec2(0.5);
    vec4 _207 = (vec4(_201 * vec2(2.0, -2.0), texelFetch(depth_tex, ivec2(gl_GlobalInvocationID.xy), int(0u)).x, 1.0) * frame_constants.view_constants[0u].sample_to_view) * frame_constants.view_constants[0u].view_to_world;
    vec2 _436;
    float _437;
    do
    {
        vec4 _227 = vec4((_207.xyz / vec3(_207.w)) + (_185 * (0.066666670143604278564453125 * clamp(1.0 - dot(_185, _189), 0.0, 1.0))), 1.0) * frame_constants.view_constants[2u].world_to_sample;
        if (_227.z > 1.0)
        {
            _436 = vec2(0.0);
            _437 = 0.0;
            break;
        }
        vec2 _232 = _227.xy;
        bool _235 = any(greaterThan(abs(_232), vec2(0.9833333492279052734375)));
        vec2 _239;
        if (_235)
        {
            _239 = _232 * 0.5;
        }
        else
        {
            _239 = _232;
        }
        float _240 = _235 ? 5.0 : 10.0;
        float _242 = _235 ? 0.5 : 1.0;
        bool _248 = any(greaterThan(abs(_239), vec2(1.0) - (vec2(0.000833333353511989116668701171875) * (2.0 * _240))));
        float _254;
        vec2 _255;
        float _256;
        if (_248)
        {
            _254 = _240 * 0.2857142984867095947265625;
            _255 = _239 * 0.2857142984867095947265625;
            _256 = _242 * 0.2857142984867095947265625;
        }
        else
        {
            _254 = _240;
            _255 = _239;
            _256 = _242;
        }
        float _259 = float(_248 ? 2u : uint(_235));
        uint _261 = 2u + uint(_259);
        float _264 = dot(_185, -_189);
        vec4 _276 = vec4(_255.x, _255.y, _227.z, _227.w) + (vec4((_185 * (0.066666670143604278564453125 * clamp(1.0 - _264, 0.0, 1.0))) / vec3(_256), 0.0) * frame_constants.view_constants[_261].world_to_sample);
        vec2 _279 = (_276.xy * vec2(0.5, -0.5)) + vec2(0.5);
        float _285 = spvNMax((0.300000011920928955078125 / _256) * (1.0 - abs(_264)), 0.02500000037252902984619140625);
        float _298 = float((uint((_190.x * 11) + (_190.y * 7)) + (frame_constants.frame_index * 5u)) % 64u) * 1.61803400516510009765625;
        uvec2 _299 = uvec2(_190);
        float _304 = 5.588237762451171875 * float(frame_constants.frame_index % 64u);
        float _314 = fract(52.98291778564453125 * fract((0.067110560834407806396484375 * (float(_299.x) + _304)) + (0.005837149918079376220703125 * (float(_299.y) + _304))));
        vec4 _320 = vec4(_185, 0.0) * frame_constants.view_constants[_261].world_to_sample;
        float _322;
        int _325;
        _322 = 0.0;
        _325 = 0;
        float _323;
        int _326;
        for (int _327 = 0; _327 < 5; _322 = _323, _325 = _326, _327++)
        {
            float _332 = float(_327);
            float _334 = (_332 + _298) * 2.3999631404876708984375;
            vec3 _347 = vec3(_279.x, _279.y, _276.z) + vec3((((vec2(cos(_334), sin(_334)) * (_332 + _314)) * vec2(0.20000000298023223876953125)) * _254) * vec2(0.000833333353511989116668701171875), _285 * (-0.0009765625));
            vec4 _354 = textureLod(sampler2DArray(shadow_map, sampler_shadow_nearest), vec3(_347.xy, _259), 0.0);
            float _355 = _354.x;
            if (_347.z > _355)
            {
                _323 = _322 + _355;
                _326 = _325 + 1;
            }
            else
            {
                _323 = _322;
                _326 = _325;
            }
        }
        float _363 = _322 / float(_325);
        if (_325 == 0)
        {
            _436 = vec2(0.0);
            _437 = 0.0;
            break;
        }
        float _377 = spvNMax(((_276.z - _363) * ((17.0666675567626953125 * _254) * pow(10.0, smoothstep(0.0, 0.25, frame_constants.fog_tint_thickness.w)))) / _363, 0.0);
        float _384 = (_377 / pow(pow(_377 / (_254 - 1.0), 5.0) + 1.0, 0.20000000298023223876953125)) + 1.0;
        float _386 = _320.z;
        vec2 _396 = (((_320.xy * float(int(sign(_386)))) / vec2(spvNMax(0.1500000059604644775390625, abs(_386)))) * vec2(0.0009765625)) * 2.0;
        vec2 _398;
        _398 = vec2(0.0);
        for (int _401 = 0; _401 < 10; )
        {
            float _405 = float(_401);
            float _407 = (_405 + _298) * 2.3999631404876708984375;
            vec2 _414 = ((vec2(cos(_407), sin(_407)) * (_405 + _314)) * vec2(0.100000001490116119384765625)) * _384;
            vec3 _420 = vec3(_279.x, _279.y, _276.z) + vec3(_414 * vec2(0.000833333353511989116668701171875), _285 * (-0.0009765625));
            float _432 = 1.0 - textureGrad(sampler2DArrayShadow(shadow_map, sampler_shadow_linear), vec4(vec3(_420.xy, _259), _420.z + spvNMin(0.0, dot(_396, _414))), vec2(0.0), vec2(0.0));
            _398 += vec2(_432, _432 * _432);
            _401++;
            continue;
        }
        _436 = _398 * 0.100000001490116119384765625;
        _437 = _384;
        break;
    } while(false);
    float _438 = _436.x;
    float _441 = clamp((_438 - 0.100000001490116119384765625) * 1.111111164093017578125, 0.0, 1.0);
    float _443 = _436.y;
    float _448 = _437 * 0.20000000298023223876953125;
    vec2 _454 = vec2(_441) + ((vec2(-1.0, 1.0) * sqrt(spvNMax(0.0, _443 - (_438 * _438)))) * (3.0 * spvNMax(0.0, 1.0 - _448)));
    vec4 _456 = texelFetch(reprojection_tex, ivec2(gl_GlobalInvocationID.xy), int(0u));
    bool _473;
    if (_456.z > 0.0)
    {
        bool _472;
        if (_Globals.history_valid != 0u)
        {
            _472 = !(0u == frame_constants.frame_index);
        }
        else
        {
            _472 = false;
        }
        _473 = _472;
    }
    else
    {
        _473 = false;
    }
    float _482;
    if (_473)
    {
        _482 = textureLod(sampler2D(history_tex, sampler_lnc), _196 + _456.xy, 0.0).x;
    }
    else
    {
        _482 = _441;
    }
    float _485 = clamp(_482, _454.x, _454.y);
    float _486 = mix(_485, _441, 0.125);
    if (length(_201 * vec2(float(_Globals.depth_tex_dims.x) / float(_Globals.depth_tex_dims.y), 1.0)) < 0.449999988079071044921875)
    {
        uint _500 = atomicAdd(thread_group_alive, 1u);
        uint _506 = atomicAdd(thread_group_shadow_mask_sum, clamp(uint(clamp(1.0 - _486, 0.0, 1.0) * 256.0), 0u, 255u));
    }
    barrier();
    if (_162)
    {
        if (thread_group_alive > 16u)
        {
            uint _520 = atomicAdd(_alias_shadow_mask_histogram._m0[clamp(uint(float(thread_group_shadow_mask_sum) / float(thread_group_alive)), 0u, 255u)], 1u);
        }
    }
    imageStore(output_tex, ivec2(gl_GlobalInvocationID.xy), vec4(_486 + 0.00196078442968428134918212890625));
}

