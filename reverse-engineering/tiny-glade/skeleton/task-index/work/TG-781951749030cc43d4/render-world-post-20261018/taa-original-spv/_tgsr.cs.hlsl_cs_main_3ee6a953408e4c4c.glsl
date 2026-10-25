#version 450
#extension GL_EXT_nonuniform_qualifier : require
#extension GL_EXT_samplerless_texture_functions : require
#extension GL_EXT_spirv_intrinsics : require
#extension GL_KHR_shader_subgroup_basic : require
#extension GL_KHR_shader_subgroup_arithmetic : require
#extension GL_KHR_shader_subgroup_shuffle : require
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

struct FrexpStructType
{
    vec3 mantissa;
    ivec3 exponent;
};

float _156;
float _162;

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

shared float tile_luminance[8][8];

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
    vec2 _171 = vec2(_Globals.output_tex_dims);
    vec2 _173 = vec2(ivec2(gl_GlobalInvocationID.xy)) + vec2(0.5);
    vec2 _174 = _173 / _171;
    bool _177 = _Globals.bypass_neighborhood_clamping != 0u;
    float _179 = float(_177 ? 16 : 4);
    vec4 _183 = textureLod(sampler2D(reprojection_tex, sampler_nnc), _174, 0.0);
    vec2 _184 = _183.xy;
    bool _188 = _Globals.history_valid != 0u;
    bool _195;
    if (_188)
    {
        _195 = !(0u == frame_constants.frame_index);
    }
    else
    {
        _195 = false;
    }
    vec2 _204;
    if (_195)
    {
        _204 = textureLod(sampler2D(aux_tex, sampler_lnc), _174 + _184, 0.0).xy;
    }
    else
    {
        _204 = vec2(0.0);
    }
    float _208 = _204.y * _179;
    vec3 _216 = vec3(texelFetch(reprojected_history_tex_lum, ivec2(gl_GlobalInvocationID.xy), int(0u)).x, texelFetch(reprojected_history_tex_chroma, ivec2(gl_GlobalInvocationID.xy), int(0u)).xy);
    vec2 _224 = vec2(ivec2(_Globals.color_tex_dims)) / _171;
    ivec2 _226 = ivec2(_173 * _224);
    vec2 _231 = ((vec2(_226) + vec2(0.5)) + (frame_constants.view_constants[0u].sample_offset_pixels * vec2(1.0, -1.0))) / _224;
    float _232 = _224.x;
    float _236 = ((-10.0) * _232) * spvNMin(4.0 * _232, 1.0);
    float _238 = (-1.0) * (_232 * _232);
    vec3 _240;
    float _243;
    vec3 _245;
    float _247;
    vec4 _249;
    int _251;
    _240 = vec3(0.0);
    _243 = 0.0;
    _245 = vec3(0.0);
    _247 = 0.0;
    _249 = vec4(0.0);
    _251 = -1;
    vec3 _241;
    float _244;
    vec3 _246;
    float _248;
    vec4 _250;
    for (; _251 <= 1; _240 = _241, _243 = _244, _245 = _246, _247 = _248, _249 = _250, _251++)
    {
        _244 = _243;
        _241 = _240;
        _246 = _245;
        _248 = _247;
        _250 = _249;
        for (int _263 = -1; _263 <= 1; )
        {
            vec4 _281 = vec4(texelFetch(color_tex, ivec2(uvec2(_226 + ivec2(_263, _251))), int(0u)).xyz * mat3(vec3(0.2125999927520751953125, 0.715200006961822509765625, 0.072200000286102294921875), vec3(-0.114600002765655517578125, -0.385399997234344482421875, 0.5), vec3(0.5, -0.4541999995708465576171875, -0.0458000004291534423828125)), _156);
            vec2 _282 = (_231 + (vec2(float(_263), float(_251)) / _224)) - _173;
            float _283 = dot(_282, _282);
            float _285 = exp2(_283 * _238);
            float _287 = exp2(_283 * _236);
            vec3 _289 = _281.xyz;
            _244 += _285;
            _241 += ((_289 * _289) * _285);
            _246 += (_289 * _285);
            _248 += _287;
            _250 += (_281 * _287);
            _263++;
            continue;
        }
    }
    vec3 _293 = vec3(_243);
    vec3 _294 = _245 / _293;
    vec3 _296 = _249.xyz;
    float _298;
    float _301;
    int _303;
    _298 = 0.0;
    _301 = 0.0;
    _303 = -1;
    float _299;
    float _302;
    for (; _303 <= 1; _298 = _299, _301 = _302, _303++)
    {
        _299 = _298;
        _302 = _301;
        for (int _312 = -1; _312 <= 1; )
        {
            float _321 = exp(float((_312 * _312) + (_303 * _303)) * (-0.75));
            _299 += (textureLod(sampler2D(reprojected_history_tex_lum, sampler_lnc), _174 + ((vec2(float(_312), float(_303)) * 2.0) / vec2(_Globals.color_tex_dims)), 0.0).x * _321);
            _302 += _321;
            _312++;
            continue;
        }
    }
    float _336 = clamp(sqrt(_298 / _301), 0.0, 1.0);
    float _337 = _336 * _336;
    vec4 _341 = textureLod(sampler2D(blurred_luminance_tex, sampler_lnc), _174, 0.0);
    float _342 = _341.x;
    float _343 = _342 * _342;
    float _354;
    float _355;
    if (subgroupMax(gl_SubgroupInvocationID) >= 31u)
    {
        uint _349 = gl_SubgroupInvocationID ^ 16u;
        _354 = mix(_343, subgroupShuffle(_343, _349), 0.5);
        _355 = mix(_337, subgroupShuffle(_337, _349), 0.5);
    }
    else
    {
        _354 = _343;
        _355 = _337;
    }
    uint _356 = gl_SubgroupInvocationID ^ 2u;
    float _357 = subgroupShuffle(_355, _356);
    float _358 = mix(_355, _357, 0.5);
    float _359 = subgroupShuffle(_354, _356);
    float _360 = mix(_354, _359, 0.5);
    vec3 _364 = sqrt(spvNMax(vec3(0.0), (_240 / _293) - (_294 * _294)));
    vec2 _368 = vec2(0.5) / vec2(_Globals.reprojection_tex_dims);
    vec3 _369 = _364 * 2.0;
    vec3 _371 = (_294 + _369) + vec3(0.001000000047497451305389404296875);
    vec3 _373 = (_294 - _369) - vec3(0.001000000047497451305389404296875);
    float _375;
    float _378;
    vec3 _380;
    vec3 _382;
    float _384;
    int _386;
    _375 = 0.0;
    _378 = 0.0;
    _380 = vec3(0.0);
    _382 = vec3(0.0);
    _384 = 0.0;
    _386 = -1;
    float _376;
    float _379;
    vec3 _381;
    vec3 _383;
    float _385;
    for (; _386 <= 1; _375 = _376, _378 = _379, _380 = _381, _382 = _383, _384 = _385, _386++)
    {
        _376 = _375;
        _379 = _378;
        _381 = _380;
        _383 = _382;
        _385 = _384;
        for (int _398 = -1; _398 <= 1; )
        {
            uvec2 _404 = gl_GlobalInvocationID.xy + uvec2(ivec2(_398, _386));
            vec4 _405 = texelFetch(reprojected_history_tex_lum, ivec2(_404), int(0u));
            float _406 = _405.x;
            vec3 _409 = vec3(_406, _162, texelFetch(reprojected_history_tex_chroma, ivec2(_404), int(0u)).y);
            _376 += (_406 * _406);
            _379 += _406;
            _381 += (vec3(lessThan(_409, _373)) * 1.0);
            _383 += (vec3(greaterThan(_409, _371)) * 1.0);
            _385 += 1.0;
            _398++;
            continue;
        }
    }
    vec3 _417 = vec3(_384);
    float _420 = _378 / _384;
    float _425 = sqrt(spvNMax(0.0, (_375 / _384) - (_420 * _420)));
    vec4 _427 = textureGather(sampler2D(reprojection_tex, sampler_lnc), _174, 3);
    float _441 = clamp((20.0 * spvNMax(0.0, _425)) / spvNMax(9.9999999392252902907785028219223e-09, _420), 0.0, 1.0);
    vec2 _448 = (_382 / _417).xz + (_380 / _417).xz;
    float _454 = abs(_358 - _360);
    float _458 = (2.0 * _454) / spvNMax(9.9999997473787516355514526367188e-06, _360 + _358);
    float _460 = _294.x;
    float _476;
    float _479;
    int _481;
    _476 = 0.0;
    _479 = 0.0;
    _481 = -1;
    float _477;
    float _480;
    for (; _481 <= 1; _476 = _477, _479 = _480, _481++)
    {
        _477 = _476;
        _480 = _479;
        for (int _490 = -1; _490 <= 1; )
        {
            _477 += float(textureLod(sampler2D(reprojection_tex, sampler_nnc), _174 - ((_368 * 1.0) * vec2(float(_490), float(_481))), 0.0).z == 0.0);
            _480 += 1.0;
            _490++;
            continue;
        }
    }
    vec2 _507 = _184 * _171;
    vec2 _508 = abs(_507);
    vec4 _526 = textureGather(sampler2D(reprojection_tex, sampler_lnc), _174, 2);
    float _536 = (((3.0 * clamp(1.0 - ((2.5 * _425) / _420), 0.0, 1.0)) * _454) / (0.5 * spvNMax(9.9999997473787516355514526367188e-05, _358 + _360))) * float(spvNMin(spvNMin(_526.x, _526.y), spvNMin(_526.z, _526.w)) == 1.0);
    float _539 = ((((((_204.x * 2.0) - 0.5) + clamp(1.0 - _441, 0.0, 1.0)) + spvNMax(_448.x, _448.y)) + ((_458 * _458) * spvNMax(0.0, 1.0 - ((((-3.0) * abs(_460 - _360)) + (2.0 * abs((_249.x / _247) - _460))) / _360)))) + (sqrt(_476 / _479) * clamp(spvNMax(_508.x, _508.y), 0.0, 1.0))) + ((_536 * _536) * _536);
    bool _540 = !_177;
    float _559;
    float _560;
    if (_540)
    {
        float _548;
        float _549;
        if (clamp(1.0 - spvNMin(spvNMin(_427.x, _427.y), spvNMin(_427.z, _427.w)), 0.0, 1.0) < 0.75)
        {
            _548 = _539 + 1.0;
            _549 = spvNMin(_208, 0.5);
        }
        else
        {
            _548 = _539;
            _549 = _208;
        }
        float _550 = _549 * _441;
        vec2 _552 = abs(_507 * vec2(0.03125));
        _559 = spvNMin(_550, mix(_550, 1.0, clamp(spvNMax(_552.x, _552.y), 0.0, 1.0)));
        _560 = _548;
    }
    else
    {
        _559 = _208;
        _560 = _539;
    }
    float _566 = spvNMin(_560, 1.0);
    vec3 _572 = _364 * exp2((7.0 * clamp(1.0 - _560, 0.0, 1.0)) - 0.5);
    vec3 _579;
    if (_540)
    {
        _579 = clamp(_216, _294 - _572, _294 + _572);
    }
    else
    {
        _579 = _216;
    }
    float _580 = spvNMin(4.0, _559);
    float _581 = _247 + _580;
    vec3 _584 = vec3(_247);
    vec3 _587 = mix(_296 / _584, _579, vec3(_580 / spvNMax(9.9999997473787516355514526367188e-06, _581)));
    float _588 = _587.x;
    bool _597;
    if (_188)
    {
        _597 = !(0u == frame_constants.frame_index);
    }
    else
    {
        _597 = false;
    }
    vec3 _606;
    if (!_597)
    {
        _606 = spvNMax(vec3(0.0), (_296 / vec3(spvNMax(9.9999997473787516355514526367188e-06, _247))) * mat3(vec3(1.0, 0.0, 1.574800014495849609375), vec3(1.0, -0.1872999966144561767578125, -0.468100011348724365234375), vec3(1.0, 1.85559999942779541015625, 0.0)));
    }
    else
    {
        _606 = spvNMax(vec3(0.0), _587 * mat3(vec3(1.0, 0.0, 1.574800014495849609375), vec3(1.0, -0.1872999966144561767578125, -0.468100011348724365234375), vec3(1.0, 1.85559999942779541015625, 0.0)));
    }
    FrexpStructType _607;
    _607.mantissa = frexp(_606, _607.exponent);
    imageStore(history_output_tex, ivec2(gl_GlobalInvocationID.xy), (_606 + exp2(vec3(_607.exponent) - vec3(8.0, 8.0, 7.0))).xyzz);
    tile_luminance[gl_LocalInvocationID.x][gl_LocalInvocationID.y] = _588;
    barrier();
    uvec2 _617 = gl_LocalInvocationID.xy + uvec2(1u, 0u);
    uvec2 _618 = gl_LocalInvocationID.xy - uvec2(1u, 0u);
    uint _619 = _617.x;
    uint _620 = _617.y;
    uint _621 = _618.x;
    uint _622 = _618.y;
    float _648;
    float _649;
    if (all(lessThan(uvec4(_619, _620, _621, _622), uvec4(8u))))
    {
        float _639 = spvNMax(0.0, 1.0 - (6.0 * (abs(_588 - tile_luminance[_619][_620]) + abs(_588 - tile_luminance[_621][_622]))));
        float _644 = spvNMin(_639, (_Globals.sharpen_amount * _639) * 1.25);
        _648 = _644 * (tile_luminance[_619][_620] + tile_luminance[_621][_622]);
        _649 = _644 * 2.0;
    }
    else
    {
        _648 = 0.0;
        _649 = 0.0;
    }
    uvec2 _650 = gl_LocalInvocationID.xy + uvec2(0u, 1u);
    uvec2 _651 = gl_LocalInvocationID.xy - uvec2(0u, 1u);
    uint _652 = _650.x;
    uint _653 = _650.y;
    uint _654 = _651.x;
    uint _655 = _651.y;
    float _684;
    float _685;
    if (all(lessThan(uvec4(_652, _653, _654, _655), uvec4(8u))))
    {
        float _672 = spvNMax(0.0, 1.0 - (6.0 * (abs(_588 - tile_luminance[_652][_653]) + abs(_588 - tile_luminance[_654][_655]))));
        float _677 = spvNMin(_672, (_Globals.sharpen_amount * _672) * 1.25);
        _684 = (_648 + (tile_luminance[_652][_653] * _677)) + (tile_luminance[_654][_655] * _677);
        _685 = _649 + (_677 * 2.0);
    }
    else
    {
        _684 = _648;
        _685 = _649;
    }
    vec3 _693 = _606 * spvNMax(0.0, spvNMax(0.0, (_588 * (_685 + 1.0)) - _684) / spvNMax(9.9999997473787516355514526367188e-06, _588));
    FrexpStructType _694;
    _694.mantissa = frexp(_693, _694.exponent);
    imageStore(output_tex, ivec2(gl_GlobalInvocationID.xy), (_693 + exp2(vec3(_694.exponent) - vec3(8.0, 8.0, 7.0))).xyzz);
    imageStore(aux_output_tex, ivec2(gl_GlobalInvocationID.xy), vec2(clamp(_566 + (2.0 * clamp(_183.w * 1000.0, 0.0, 1.0)), 0.0, 2.0) * 0.5, _581 / _179).xyyy);
}

