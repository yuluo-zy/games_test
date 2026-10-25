#version 450
#extension GL_EXT_nonuniform_qualifier : require
#extension GL_EXT_samplerless_texture_functions : require
#extension GL_EXT_spirv_intrinsics : require
#extension GL_KHR_shader_subgroup_basic : require
#extension GL_KHR_shader_subgroup_arithmetic : require
#extension GL_KHR_shader_subgroup_shuffle : require
layout(local_size_x = 8, local_size_y = 4, local_size_z = 1) in;

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

struct FrexpStructType
{
    vec3 mantissa;
    ivec3 exponent;
};

float _157;
float _163;

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
    uvec2 reprojected_history_tex_lum_dims;
    uvec2 reprojected_history_tex_chroma_dims;
    uvec2 color_tex_dims;
    uvec2 blurred_luminance_tex_dims;
    uvec2 history_tex_dims;
    uvec2 reprojection_tex_dims;
    uvec2 aux_tex_dims;
    uvec2 history_output_tex_dims;
    uvec2 output_tex_dims;
    uvec2 aux_output_tex_dims;
    uint bypass_neighborhood_clamping;
    float sharpen_amount;
    uint history_valid;
} _Globals;

layout(set = 0, binding = 0) uniform sampler sampler_lnc;
layout(set = 0, binding = 2) uniform sampler sampler_nnc;
layout(set = 0, binding = 14) uniform texture2D reprojected_history_tex_lum;
layout(set = 0, binding = 15) uniform texture2D reprojected_history_tex_chroma;
layout(set = 0, binding = 16) uniform texture2D color_tex;
layout(set = 0, binding = 17) uniform texture2D blurred_luminance_tex;
layout(set = 0, binding = 19) uniform texture2D reprojection_tex;
layout(set = 0, binding = 20) uniform texture2D aux_tex;
layout(set = 0, binding = 21, r11f_g11f_b10f) uniform writeonly image2D history_output_tex;
layout(set = 0, binding = 22, r11f_g11f_b10f) uniform writeonly image2D output_tex;
layout(set = 0, binding = 23, rg8) uniform writeonly image2D aux_output_tex;

shared float tile_luminance[8][4];

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
    vec2 _172 = vec2(_Globals.output_tex_dims);
    vec2 _174 = vec2(ivec2(gl_GlobalInvocationID.xy)) + vec2(0.5);
    vec2 _175 = _174 / _172;
    bool _178 = _Globals.bypass_neighborhood_clamping != 0u;
    float _180 = float(_178 ? 16 : 4);
    vec4 _184 = textureLod(sampler2D(reprojection_tex, sampler_nnc), _175, 0.0);
    vec2 _185 = _184.xy;
    bool _189 = _Globals.history_valid != 0u;
    bool _196;
    if (_189)
    {
        _196 = !(0u == frame_constants.frame_index);
    }
    else
    {
        _196 = false;
    }
    vec2 _205;
    if (_196)
    {
        _205 = textureLod(sampler2D(aux_tex, sampler_lnc), _175 + _185, 0.0).xy;
    }
    else
    {
        _205 = vec2(0.0);
    }
    float _209 = _205.y * _180;
    vec3 _217 = vec3(texelFetch(reprojected_history_tex_lum, ivec2(gl_GlobalInvocationID.xy), int(0u)).x, texelFetch(reprojected_history_tex_chroma, ivec2(gl_GlobalInvocationID.xy), int(0u)).xy);
    vec2 _225 = vec2(ivec2(_Globals.color_tex_dims)) / _172;
    ivec2 _227 = ivec2(_174 * _225);
    vec2 _232 = ((vec2(_227) + vec2(0.5)) + (frame_constants.view_constants[0u].sample_offset_pixels * vec2(1.0, -1.0))) / _225;
    float _233 = _225.x;
    float _237 = ((-10.0) * _233) * spvNMin(4.0 * _233, 1.0);
    float _239 = (-1.0) * (_233 * _233);
    vec3 _241;
    float _244;
    vec3 _246;
    float _248;
    vec4 _250;
    int _252;
    _241 = vec3(0.0);
    _244 = 0.0;
    _246 = vec3(0.0);
    _248 = 0.0;
    _250 = vec4(0.0);
    _252 = -1;
    vec3 _242;
    float _245;
    vec3 _247;
    float _249;
    vec4 _251;
    for (; _252 <= 1; _241 = _242, _244 = _245, _246 = _247, _248 = _249, _250 = _251, _252++)
    {
        _245 = _244;
        _242 = _241;
        _247 = _246;
        _249 = _248;
        _251 = _250;
        for (int _264 = -1; _264 <= 1; )
        {
            vec4 _282 = vec4(texelFetch(color_tex, ivec2(uvec2(_227 + ivec2(_264, _252))), int(0u)).xyz * mat3(vec3(0.2125999927520751953125, 0.715200006961822509765625, 0.072200000286102294921875), vec3(-0.114600002765655517578125, -0.385399997234344482421875, 0.5), vec3(0.5, -0.4541999995708465576171875, -0.0458000004291534423828125)), _157);
            vec2 _283 = (_232 + (vec2(float(_264), float(_252)) / _225)) - _174;
            float _284 = dot(_283, _283);
            float _286 = exp2(_284 * _239);
            float _288 = exp2(_284 * _237);
            vec3 _290 = _282.xyz;
            _245 += _286;
            _242 += ((_290 * _290) * _286);
            _247 += (_290 * _286);
            _249 += _288;
            _251 += (_282 * _288);
            _264++;
            continue;
        }
    }
    vec3 _294 = vec3(_244);
    vec3 _295 = _246 / _294;
    vec3 _297 = _250.xyz;
    float _299;
    float _302;
    int _304;
    _299 = 0.0;
    _302 = 0.0;
    _304 = -1;
    float _300;
    float _303;
    for (; _304 <= 1; _299 = _300, _302 = _303, _304++)
    {
        _300 = _299;
        _303 = _302;
        for (int _313 = -1; _313 <= 1; )
        {
            float _322 = exp(float((_313 * _313) + (_304 * _304)) * (-0.75));
            _300 += (textureLod(sampler2D(reprojected_history_tex_lum, sampler_lnc), _175 + ((vec2(float(_313), float(_304)) * 2.0) / vec2(_Globals.color_tex_dims)), 0.0).x * _322);
            _303 += _322;
            _313++;
            continue;
        }
    }
    float _337 = clamp(sqrt(_299 / _302), 0.0, 1.0);
    float _338 = _337 * _337;
    vec4 _342 = textureLod(sampler2D(blurred_luminance_tex, sampler_lnc), _175, 0.0);
    float _343 = _342.x;
    float _344 = _343 * _343;
    float _355;
    float _356;
    if (subgroupMax(gl_SubgroupInvocationID) >= 31u)
    {
        uint _350 = gl_SubgroupInvocationID ^ 16u;
        _355 = mix(_344, subgroupShuffle(_344, _350), 0.5);
        _356 = mix(_338, subgroupShuffle(_338, _350), 0.5);
    }
    else
    {
        _355 = _344;
        _356 = _338;
    }
    uint _357 = gl_SubgroupInvocationID ^ 2u;
    float _358 = subgroupShuffle(_356, _357);
    float _359 = mix(_356, _358, 0.5);
    float _360 = subgroupShuffle(_355, _357);
    float _361 = mix(_355, _360, 0.5);
    vec3 _365 = sqrt(spvNMax(vec3(0.0), (_241 / _294) - (_295 * _295)));
    vec2 _369 = vec2(0.5) / vec2(_Globals.reprojection_tex_dims);
    vec3 _370 = _365 * 2.0;
    vec3 _372 = (_295 + _370) + vec3(0.001000000047497451305389404296875);
    vec3 _374 = (_295 - _370) - vec3(0.001000000047497451305389404296875);
    float _376;
    float _379;
    vec3 _381;
    vec3 _383;
    float _385;
    int _387;
    _376 = 0.0;
    _379 = 0.0;
    _381 = vec3(0.0);
    _383 = vec3(0.0);
    _385 = 0.0;
    _387 = -1;
    float _377;
    float _380;
    vec3 _382;
    vec3 _384;
    float _386;
    for (; _387 <= 1; _376 = _377, _379 = _380, _381 = _382, _383 = _384, _385 = _386, _387++)
    {
        _377 = _376;
        _380 = _379;
        _382 = _381;
        _384 = _383;
        _386 = _385;
        for (int _399 = -1; _399 <= 1; )
        {
            uvec2 _405 = gl_GlobalInvocationID.xy + uvec2(ivec2(_399, _387));
            vec4 _406 = texelFetch(reprojected_history_tex_lum, ivec2(_405), int(0u));
            float _407 = _406.x;
            vec3 _410 = vec3(_407, _163, texelFetch(reprojected_history_tex_chroma, ivec2(_405), int(0u)).y);
            _377 += (_407 * _407);
            _380 += _407;
            _382 += (vec3(lessThan(_410, _374)) * 1.0);
            _384 += (vec3(greaterThan(_410, _372)) * 1.0);
            _386 += 1.0;
            _399++;
            continue;
        }
    }
    vec3 _418 = vec3(_385);
    float _421 = _379 / _385;
    float _426 = sqrt(spvNMax(0.0, (_376 / _385) - (_421 * _421)));
    vec4 _428 = textureGather(sampler2D(reprojection_tex, sampler_lnc), _175, 3);
    float _442 = clamp((20.0 * spvNMax(0.0, _426)) / spvNMax(9.9999999392252902907785028219223e-09, _421), 0.0, 1.0);
    vec2 _449 = (_383 / _418).xz + (_381 / _418).xz;
    float _455 = abs(_359 - _361);
    float _459 = (2.0 * _455) / spvNMax(9.9999997473787516355514526367188e-06, _361 + _359);
    float _461 = _295.x;
    float _477;
    float _480;
    int _482;
    _477 = 0.0;
    _480 = 0.0;
    _482 = -1;
    float _478;
    float _481;
    for (; _482 <= 1; _477 = _478, _480 = _481, _482++)
    {
        _478 = _477;
        _481 = _480;
        for (int _491 = -1; _491 <= 1; )
        {
            _478 += float(textureLod(sampler2D(reprojection_tex, sampler_nnc), _175 - ((_369 * 1.0) * vec2(float(_491), float(_482))), 0.0).z == 0.0);
            _481 += 1.0;
            _491++;
            continue;
        }
    }
    vec2 _508 = _185 * _172;
    vec2 _509 = abs(_508);
    vec4 _527 = textureGather(sampler2D(reprojection_tex, sampler_lnc), _175, 2);
    float _537 = (((3.0 * clamp(1.0 - ((2.5 * _426) / _421), 0.0, 1.0)) * _455) / (0.5 * spvNMax(9.9999997473787516355514526367188e-05, _359 + _361))) * float(spvNMin(spvNMin(_527.x, _527.y), spvNMin(_527.z, _527.w)) == 1.0);
    float _540 = ((((((_205.x * 2.0) - 0.5) + clamp(1.0 - _442, 0.0, 1.0)) + spvNMax(_449.x, _449.y)) + ((_459 * _459) * spvNMax(0.0, 1.0 - ((((-3.0) * abs(_461 - _361)) + (2.0 * abs((_250.x / _248) - _461))) / _361)))) + (sqrt(_477 / _480) * clamp(spvNMax(_509.x, _509.y), 0.0, 1.0))) + ((_537 * _537) * _537);
    bool _541 = !_178;
    float _560;
    float _561;
    if (_541)
    {
        float _549;
        float _550;
        if (clamp(1.0 - spvNMin(spvNMin(_428.x, _428.y), spvNMin(_428.z, _428.w)), 0.0, 1.0) < 0.75)
        {
            _549 = _540 + 1.0;
            _550 = spvNMin(_209, 0.5);
        }
        else
        {
            _549 = _540;
            _550 = _209;
        }
        float _551 = _550 * _442;
        vec2 _553 = abs(_508 * vec2(0.03125));
        _560 = spvNMin(_551, mix(_551, 1.0, clamp(spvNMax(_553.x, _553.y), 0.0, 1.0)));
        _561 = _549;
    }
    else
    {
        _560 = _209;
        _561 = _540;
    }
    float _567 = spvNMin(_561, 1.0);
    vec3 _573 = _365 * exp2((7.0 * clamp(1.0 - _561, 0.0, 1.0)) - 0.5);
    vec3 _580;
    if (_541)
    {
        _580 = clamp(_217, _295 - _573, _295 + _573);
    }
    else
    {
        _580 = _217;
    }
    float _581 = spvNMin(4.0, _560);
    float _582 = _248 + _581;
    vec3 _585 = vec3(_248);
    vec3 _588 = mix(_297 / _585, _580, vec3(_581 / spvNMax(9.9999997473787516355514526367188e-06, _582)));
    float _589 = _588.x;
    bool _598;
    if (_189)
    {
        _598 = !(0u == frame_constants.frame_index);
    }
    else
    {
        _598 = false;
    }
    vec3 _607;
    if (!_598)
    {
        _607 = spvNMax(vec3(0.0), (_297 / vec3(spvNMax(9.9999997473787516355514526367188e-06, _248))) * mat3(vec3(1.0, 0.0, 1.574800014495849609375), vec3(1.0, -0.1872999966144561767578125, -0.468100011348724365234375), vec3(1.0, 1.85559999942779541015625, 0.0)));
    }
    else
    {
        _607 = spvNMax(vec3(0.0), _588 * mat3(vec3(1.0, 0.0, 1.574800014495849609375), vec3(1.0, -0.1872999966144561767578125, -0.468100011348724365234375), vec3(1.0, 1.85559999942779541015625, 0.0)));
    }
    FrexpStructType _608;
    _608.mantissa = frexp(_607, _608.exponent);
    imageStore(history_output_tex, ivec2(gl_GlobalInvocationID.xy), (_607 + exp2(vec3(_608.exponent) - vec3(8.0, 8.0, 7.0))).xyzz);
    tile_luminance[gl_LocalInvocationID.x][gl_LocalInvocationID.y] = _589;
    barrier();
    uvec2 _618 = gl_LocalInvocationID.xy + uvec2(1u, 0u);
    uvec2 _619 = gl_LocalInvocationID.xy - uvec2(1u, 0u);
    uint _620 = _618.x;
    uint _621 = _618.y;
    uint _622 = _619.x;
    uint _623 = _619.y;
    float _649;
    float _650;
    if (all(lessThan(uvec4(_620, _621, _622, _623), uvec4(8u, 4u, 8u, 4u))))
    {
        float _640 = spvNMax(0.0, 1.0 - (6.0 * (abs(_589 - tile_luminance[_620][_621]) + abs(_589 - tile_luminance[_622][_623]))));
        float _645 = spvNMin(_640, (_Globals.sharpen_amount * _640) * 1.25);
        _649 = _645 * (tile_luminance[_620][_621] + tile_luminance[_622][_623]);
        _650 = _645 * 2.0;
    }
    else
    {
        _649 = 0.0;
        _650 = 0.0;
    }
    uvec2 _651 = gl_LocalInvocationID.xy + uvec2(0u, 1u);
    uvec2 _652 = gl_LocalInvocationID.xy - uvec2(0u, 1u);
    uint _653 = _651.x;
    uint _654 = _651.y;
    uint _655 = _652.x;
    uint _656 = _652.y;
    float _685;
    float _686;
    if (all(lessThan(uvec4(_653, _654, _655, _656), uvec4(8u, 4u, 8u, 4u))))
    {
        float _673 = spvNMax(0.0, 1.0 - (6.0 * (abs(_589 - tile_luminance[_653][_654]) + abs(_589 - tile_luminance[_655][_656]))));
        float _678 = spvNMin(_673, (_Globals.sharpen_amount * _673) * 1.25);
        _685 = (_649 + (tile_luminance[_653][_654] * _678)) + (tile_luminance[_655][_656] * _678);
        _686 = _650 + (_678 * 2.0);
    }
    else
    {
        _685 = _649;
        _686 = _650;
    }
    vec3 _694 = _607 * spvNMax(0.0, spvNMax(0.0, (_589 * (_686 + 1.0)) - _685) / spvNMax(9.9999997473787516355514526367188e-06, _589));
    FrexpStructType _695;
    _695.mantissa = frexp(_694, _695.exponent);
    imageStore(output_tex, ivec2(gl_GlobalInvocationID.xy), (_694 + exp2(vec3(_695.exponent) - vec3(8.0, 8.0, 7.0))).xyzz);
    imageStore(aux_output_tex, ivec2(gl_GlobalInvocationID.xy), vec2(clamp(_567 + (2.0 * clamp(_184.w * 1000.0, 0.0, 1.0)), 0.0, 2.0) * 0.5, _582 / _180).xyyy);
}

