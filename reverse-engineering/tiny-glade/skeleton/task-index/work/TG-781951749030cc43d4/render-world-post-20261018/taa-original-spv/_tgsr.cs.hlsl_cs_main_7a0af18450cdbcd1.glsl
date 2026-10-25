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

float _141;
float _145;

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
    vec2 _152 = vec2(_Globals.output_tex_dims);
    vec2 _154 = vec2(ivec2(gl_GlobalInvocationID.xy)) + vec2(0.5);
    vec2 _155 = _154 / _152;
    bool _158 = _Globals.bypass_neighborhood_clamping != 0u;
    float _160 = float(_158 ? 16 : 4);
    vec4 _164 = textureLod(sampler2D(reprojection_tex, sampler_nnc), _155, 0.0);
    vec2 _165 = _164.xy;
    bool _169 = _Globals.history_valid != 0u;
    bool _176;
    if (_169)
    {
        _176 = !(0u == frame_constants.frame_index);
    }
    else
    {
        _176 = false;
    }
    vec2 _185;
    if (_176)
    {
        _185 = textureLod(sampler2D(aux_tex, sampler_lnc), _155 + _165, 0.0).xy;
    }
    else
    {
        _185 = vec2(0.0);
    }
    float _189 = _185.y * _160;
    vec3 _197 = vec3(texelFetch(reprojected_history_tex_lum, ivec2(gl_GlobalInvocationID.xy), int(0u)).x, texelFetch(reprojected_history_tex_chroma, ivec2(gl_GlobalInvocationID.xy), int(0u)).xy);
    vec2 _205 = vec2(ivec2(_Globals.color_tex_dims)) / _152;
    ivec2 _207 = ivec2(_154 * _205);
    vec2 _212 = ((vec2(_207) + vec2(0.5)) + (frame_constants.view_constants[0u].sample_offset_pixels * vec2(1.0, -1.0))) / _205;
    float _213 = _205.x;
    float _217 = ((-10.0) * _213) * spvNMin(4.0 * _213, 1.0);
    float _219 = (-1.0) * (_213 * _213);
    vec3 _221;
    float _224;
    vec3 _226;
    float _228;
    vec4 _230;
    int _232;
    _221 = vec3(0.0);
    _224 = 0.0;
    _226 = vec3(0.0);
    _228 = 0.0;
    _230 = vec4(0.0);
    _232 = -1;
    vec3 _222;
    float _225;
    vec3 _227;
    float _229;
    vec4 _231;
    for (; _232 <= 1; _221 = _222, _224 = _225, _226 = _227, _228 = _229, _230 = _231, _232++)
    {
        _225 = _224;
        _222 = _221;
        _227 = _226;
        _229 = _228;
        _231 = _230;
        for (int _244 = -1; _244 <= 1; )
        {
            vec4 _262 = vec4(texelFetch(color_tex, ivec2(uvec2(_207 + ivec2(_244, _232))), int(0u)).xyz * mat3(vec3(0.2125999927520751953125, 0.715200006961822509765625, 0.072200000286102294921875), vec3(-0.114600002765655517578125, -0.385399997234344482421875, 0.5), vec3(0.5, -0.4541999995708465576171875, -0.0458000004291534423828125)), _141);
            vec2 _263 = (_212 + (vec2(float(_244), float(_232)) / _205)) - _154;
            float _264 = dot(_263, _263);
            float _266 = exp2(_264 * _219);
            float _268 = exp2(_264 * _217);
            vec3 _270 = _262.xyz;
            _225 += _266;
            _222 += ((_270 * _270) * _266);
            _227 += (_270 * _266);
            _229 += _268;
            _231 += (_262 * _268);
            _244++;
            continue;
        }
    }
    vec3 _274 = vec3(_224);
    vec3 _275 = _226 / _274;
    float _279;
    float _282;
    int _284;
    _279 = 0.0;
    _282 = 0.0;
    _284 = -1;
    float _280;
    float _283;
    for (; _284 <= 1; _279 = _280, _282 = _283, _284++)
    {
        _280 = _279;
        _283 = _282;
        for (int _293 = -1; _293 <= 1; )
        {
            float _302 = exp(float((_293 * _293) + (_284 * _284)) * (-0.75));
            _280 += (textureLod(sampler2D(reprojected_history_tex_lum, sampler_lnc), _155 + ((vec2(float(_293), float(_284)) * 2.0) / vec2(_Globals.color_tex_dims)), 0.0).x * _302);
            _283 += _302;
            _293++;
            continue;
        }
    }
    float _317 = clamp(sqrt(_279 / _282), 0.0, 1.0);
    float _318 = _317 * _317;
    vec4 _322 = textureLod(sampler2D(blurred_luminance_tex, sampler_lnc), _155, 0.0);
    float _323 = _322.x;
    float _324 = _323 * _323;
    float _335;
    float _336;
    if (subgroupMax(gl_SubgroupInvocationID) >= 31u)
    {
        uint _330 = gl_SubgroupInvocationID ^ 16u;
        _335 = mix(_324, subgroupShuffle(_324, _330), 0.5);
        _336 = mix(_318, subgroupShuffle(_318, _330), 0.5);
    }
    else
    {
        _335 = _324;
        _336 = _318;
    }
    uint _337 = gl_SubgroupInvocationID ^ 2u;
    float _338 = subgroupShuffle(_336, _337);
    float _339 = mix(_336, _338, 0.5);
    float _340 = subgroupShuffle(_335, _337);
    float _341 = mix(_335, _340, 0.5);
    vec3 _345 = sqrt(spvNMax(vec3(0.0), (_221 / _274) - (_275 * _275)));
    vec2 _349 = vec2(0.5) / vec2(_Globals.reprojection_tex_dims);
    vec3 _350 = _345 * 2.0;
    vec3 _352 = (_275 + _350) + vec3(0.001000000047497451305389404296875);
    vec3 _354 = (_275 - _350) - vec3(0.001000000047497451305389404296875);
    float _356;
    float _359;
    vec3 _361;
    vec3 _363;
    float _365;
    int _367;
    _356 = 0.0;
    _359 = 0.0;
    _361 = vec3(0.0);
    _363 = vec3(0.0);
    _365 = 0.0;
    _367 = -1;
    float _357;
    float _360;
    vec3 _362;
    vec3 _364;
    float _366;
    for (; _367 <= 1; _356 = _357, _359 = _360, _361 = _362, _363 = _364, _365 = _366, _367++)
    {
        _357 = _356;
        _360 = _359;
        _362 = _361;
        _364 = _363;
        _366 = _365;
        for (int _379 = -1; _379 <= 1; )
        {
            uvec2 _385 = gl_GlobalInvocationID.xy + uvec2(ivec2(_379, _367));
            vec4 _386 = texelFetch(reprojected_history_tex_lum, ivec2(_385), int(0u));
            float _387 = _386.x;
            vec3 _390 = vec3(_387, _145, texelFetch(reprojected_history_tex_chroma, ivec2(_385), int(0u)).y);
            _357 += (_387 * _387);
            _360 += _387;
            _362 += (vec3(lessThan(_390, _354)) * 1.0);
            _364 += (vec3(greaterThan(_390, _352)) * 1.0);
            _366 += 1.0;
            _379++;
            continue;
        }
    }
    vec3 _398 = vec3(_365);
    float _401 = _359 / _365;
    float _406 = sqrt(spvNMax(0.0, (_356 / _365) - (_401 * _401)));
    vec4 _408 = textureGather(sampler2D(reprojection_tex, sampler_lnc), _155, 3);
    float _422 = clamp((20.0 * spvNMax(0.0, _406)) / spvNMax(9.9999999392252902907785028219223e-09, _401), 0.0, 1.0);
    vec2 _429 = (_363 / _398).xz + (_361 / _398).xz;
    float _435 = abs(_339 - _341);
    float _439 = (2.0 * _435) / spvNMax(9.9999997473787516355514526367188e-06, _341 + _339);
    float _441 = _275.x;
    float _457;
    float _460;
    int _462;
    _457 = 0.0;
    _460 = 0.0;
    _462 = -1;
    float _458;
    float _461;
    for (; _462 <= 1; _457 = _458, _460 = _461, _462++)
    {
        _458 = _457;
        _461 = _460;
        for (int _471 = -1; _471 <= 1; )
        {
            _458 += float(textureLod(sampler2D(reprojection_tex, sampler_nnc), _155 - ((_349 * 1.0) * vec2(float(_471), float(_462))), 0.0).z == 0.0);
            _461 += 1.0;
            _471++;
            continue;
        }
    }
    vec2 _488 = _165 * _152;
    vec2 _489 = abs(_488);
    vec4 _507 = textureGather(sampler2D(reprojection_tex, sampler_lnc), _155, 2);
    float _517 = (((3.0 * clamp(1.0 - ((2.5 * _406) / _401), 0.0, 1.0)) * _435) / (0.5 * spvNMax(9.9999997473787516355514526367188e-05, _339 + _341))) * float(spvNMin(spvNMin(_507.x, _507.y), spvNMin(_507.z, _507.w)) == 1.0);
    float _520 = ((((((_185.x * 2.0) - 0.5) + clamp(1.0 - _422, 0.0, 1.0)) + spvNMax(_429.x, _429.y)) + ((_439 * _439) * spvNMax(0.0, 1.0 - ((((-3.0) * abs(_441 - _341)) + (2.0 * abs((_230.x / _228) - _441))) / _341)))) + (sqrt(_457 / _460) * clamp(spvNMax(_489.x, _489.y), 0.0, 1.0))) + ((_517 * _517) * _517);
    bool _521 = !_158;
    float _540;
    float _541;
    if (_521)
    {
        float _529;
        float _530;
        if (clamp(1.0 - spvNMin(spvNMin(_408.x, _408.y), spvNMin(_408.z, _408.w)), 0.0, 1.0) < 0.75)
        {
            _529 = _520 + 1.0;
            _530 = spvNMin(_189, 0.5);
        }
        else
        {
            _529 = _520;
            _530 = _189;
        }
        float _531 = _530 * _422;
        vec2 _533 = abs(_488 * vec2(0.03125));
        _540 = spvNMin(_531, mix(_531, 1.0, clamp(spvNMax(_533.x, _533.y), 0.0, 1.0)));
        _541 = _529;
    }
    else
    {
        _540 = _189;
        _541 = _520;
    }
    vec3 _553 = _345 * exp2((7.0 * clamp(1.0 - _541, 0.0, 1.0)) - 0.5);
    vec3 _560;
    if (_521)
    {
        _560 = clamp(_197, _275 - _553, _275 + _553);
    }
    else
    {
        _560 = _197;
    }
    float _561 = spvNMin(4.0, _540);
    float _562 = _228 + _561;
    bool _577;
    if (_169)
    {
        _577 = !(0u == frame_constants.frame_index);
    }
    else
    {
        _577 = false;
    }
    vec3 _586;
    if (!_577)
    {
        _586 = spvNMax(vec3(0.0), (_230.xyz / vec3(spvNMax(9.9999997473787516355514526367188e-06, _228))) * mat3(vec3(1.0, 0.0, 1.574800014495849609375), vec3(1.0, -0.1872999966144561767578125, -0.468100011348724365234375), vec3(1.0, 1.85559999942779541015625, 0.0)));
    }
    else
    {
        _586 = spvNMax(vec3(0.0), mix(_230.xyz / vec3(_228), _560, vec3(_561 / spvNMax(9.9999997473787516355514526367188e-06, _562))) * mat3(vec3(1.0, 0.0, 1.574800014495849609375), vec3(1.0, -0.1872999966144561767578125, -0.468100011348724365234375), vec3(1.0, 1.85559999942779541015625, 0.0)));
    }
    FrexpStructType _587;
    _587.mantissa = frexp(_586, _587.exponent);
    vec3 _592 = _586 + exp2(vec3(_587.exponent) - vec3(8.0, 8.0, 7.0));
    imageStore(history_output_tex, ivec2(gl_GlobalInvocationID.xy), _592.xyzz);
    imageStore(output_tex, ivec2(gl_GlobalInvocationID.xy), _592.xyzz);
    imageStore(aux_output_tex, ivec2(gl_GlobalInvocationID.xy), vec2(clamp(spvNMin(_541, 1.0) + (2.0 * clamp(_164.w * 1000.0, 0.0, 1.0)), 0.0, 2.0) * 0.5, _562 / _160).xyyy);
}

