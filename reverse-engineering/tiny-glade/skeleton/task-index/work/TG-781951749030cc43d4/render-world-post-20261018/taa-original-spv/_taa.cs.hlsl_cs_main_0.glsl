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

struct FrexpStructType
{
    vec3 mantissa;
    ivec3 exponent;
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
    uvec2 color_tex_dims;
    uvec2 history_tex_dims;
    uvec2 reprojection_tex_dims;
    uvec2 history_output_tex_dims;
    uvec2 output_tex_dims;
    uvec2 temp_debug_tex_dims;
    uint bypass_neighborhood_clamping;
    float sharpen_amount;
    uint history_valid;
} _Globals;

layout(set = 0, binding = 9) uniform sampler sampler_default;
layout(set = 0, binding = 14) uniform texture2D color_tex;
layout(set = 0, binding = 15) uniform texture2D history_tex;
layout(set = 0, binding = 16) uniform texture2D reprojection_tex;
layout(set = 0, binding = 17, r11f_g11f_b10f) uniform writeonly image2D history_output_tex;
layout(set = 0, binding = 18, r11f_g11f_b10f) uniform writeonly image2D output_tex;

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
    vec2 _147 = vec2(_Globals.output_tex_dims);
    vec2 _149 = vec2(ivec2(gl_GlobalInvocationID.xy)) + vec2(0.5);
    vec2 _165 = ((_149 / _147) + texelFetch(reprojection_tex, ivec2(uvec2((vec2(gl_GlobalInvocationID.xy) + vec2(0.5)) * vec2(_Globals.color_tex_dims / _Globals.output_tex_dims))), int(0u)).xy) * _147;
    vec2 _167 = floor(_165 - vec2(0.5));
    vec2 _168 = _167 + vec2(0.5);
    vec2 _169 = _165 - _168;
    vec2 _170 = _169 * 0.5;
    vec2 _174 = _169 * (vec2(-0.5) + (_169 * (vec2(1.0) - _170)));
    vec2 _175 = _169 * _169;
    vec2 _176 = _169 * 1.5;
    vec2 _183 = _169 * (vec2(0.5) + (_169 * (vec2(2.0) - _176)));
    vec2 _185 = _175 * (vec2(-0.5) + _170);
    vec2 _186 = (vec2(1.0) + (_175 * (vec2(-2.5) + _176))) + _183;
    vec2 _191 = (_167 + vec2(-0.5)) / _147;
    vec2 _192 = (_167 + vec2(2.5)) / _147;
    vec2 _193 = (_168 + (_183 / _186)) / _147;
    float _194 = _193.x;
    float _203 = _186.x;
    float _205 = _174.y;
    float _208 = _193.y;
    float _216 = _174.x;
    float _218 = _186.y;
    float _238 = _185.x;
    float _251 = _185.y;
    vec3 _265 = (((((((vec4(textureLod(sampler2D(history_tex, sampler_default), vec2(_194, _191.y), 0.0).xyz, 1.0) * _203) * _205) + ((vec4(textureLod(sampler2D(history_tex, sampler_default), vec2(_191.x, _208), 0.0).xyz, 1.0) * _216) * _218)) + ((vec4(textureLod(sampler2D(history_tex, sampler_default), _193, 0.0).xyz, 1.0) * _203) * _218)) + ((vec4(textureLod(sampler2D(history_tex, sampler_default), vec2(_192.x, _208), 0.0).xyz, 1.0) * _238) * _218)) + ((vec4(textureLod(sampler2D(history_tex, sampler_default), vec2(_194, _192.y), 0.0).xyz, 1.0) * _203) * _251)) / vec4(((((_203 * _205) + (_216 * _218)) + (_203 * _218)) + (_238 * _218)) + (_203 * _251))).xyz;
    vec3 _266 = _265 * mat3(vec3(0.2125999927520751953125, 0.715200006961822509765625, 0.072200000286102294921875), vec3(-0.114600002765655517578125, -0.385399997234344482421875, 0.5), vec3(0.5, -0.4541999995708465576171875, -0.0458000004291534423828125));
    vec2 _272 = vec2(ivec2(_Globals.color_tex_dims)) / _147;
    ivec2 _274 = ivec2(_149 * _272);
    vec2 _279 = ((vec2(_274) + vec2(0.5)) + (frame_constants.view_constants[0u].sample_offset_pixels * vec2(1.0, -1.0))) / _272;
    float _280 = _272.x;
    float _284 = ((-10.0) * _280) * spvNMin(4.0 * _280, 1.0);
    float _286 = (-1.0) * (_280 * _280);
    vec3 _288;
    float _291;
    vec3 _293;
    float _295;
    vec4 _297;
    int _299;
    _288 = vec3(0.0);
    _291 = 0.0;
    _293 = vec3(0.0);
    _295 = 0.0;
    _297 = vec4(0.0);
    _299 = -1;
    vec3 _289;
    float _292;
    vec3 _294;
    float _296;
    vec4 _298;
    for (; _299 <= 1; _288 = _289, _291 = _292, _293 = _294, _295 = _296, _297 = _298, _299++)
    {
        _292 = _291;
        _289 = _288;
        _294 = _293;
        _296 = _295;
        _298 = _297;
        for (int _311 = -1; _311 <= 1; )
        {
            vec4 _329 = vec4(texelFetch(color_tex, ivec2(uvec2(_274 + ivec2(_311, _299))), int(0u)).xyz * mat3(vec3(0.2125999927520751953125, 0.715200006961822509765625, 0.072200000286102294921875), vec3(-0.114600002765655517578125, -0.385399997234344482421875, 0.5), vec3(0.5, -0.4541999995708465576171875, -0.0458000004291534423828125)), 1.0);
            vec2 _330 = (_279 + (vec2(float(_311), float(_299)) / _272)) - _149;
            float _331 = dot(_330, _330);
            float _333 = exp2(_331 * _286);
            float _335 = exp2(_331 * _284);
            vec3 _337 = _329.xyz;
            _292 += _333;
            _289 += ((_337 * _337) * _333);
            _294 += (_337 * _333);
            _296 += _335;
            _298 += (_329 * _335);
            _311++;
            continue;
        }
    }
    vec3 _341 = vec3(_291);
    vec3 _342 = _293 / _341;
    vec3 _347 = _297.xyz / vec3(spvNMax(9.9999997473787516355514526367188e-06, _295));
    vec3 _351 = sqrt(spvNMax(vec3(0.0), (_288 / _341) - (_342 * _342)));
    float _353 = _342.x;
    vec3 _359 = (_351 * mix(0.5, 1.0, smoothstep(-0.100000001490116119384765625, 0.300000011920928955078125, _351.x / (_353 + 9.9999997473787516355514526367188e-06)))) * 1.0;
    vec3 _360 = _342 - _359;
    vec3 _361 = _342 + _359;
    float _384;
    vec3 _385;
    if (!(_Globals.bypass_neighborhood_clamping != 0u))
    {
        float _370 = _266.x;
        _384 = 0.125 * mix(0.20000000298023223876953125, 1.0, smoothstep(0.0, 2.0, spvNMin(abs(_370 - _360.x), abs(_370 - _361.x)) / spvNMax(spvNMax(_370, _353), 9.9999997473787516355514526367188e-06)));
        _385 = clamp(_266, _360, _361);
    }
    else
    {
        _384 = 0.125;
        _385 = _266;
    }
    bool _399;
    if (_Globals.history_valid != 0u)
    {
        _399 = !(0u == frame_constants.frame_index);
    }
    else
    {
        _399 = false;
    }
    vec3 _405;
    if (!_399)
    {
        _405 = spvNMax(vec3(0.0), _347 * mat3(vec3(1.0, 0.0, 1.574800014495849609375), vec3(1.0, -0.1872999966144561767578125, -0.468100011348724365234375), vec3(1.0, 1.85559999942779541015625, 0.0)));
    }
    else
    {
        _405 = spvNMax(vec3(0.0), mix(_385, _347, vec3(_384)) * mat3(vec3(1.0, 0.0, 1.574800014495849609375), vec3(1.0, -0.1872999966144561767578125, -0.468100011348724365234375), vec3(1.0, 1.85559999942779541015625, 0.0)));
    }
    FrexpStructType _406;
    _406.mantissa = frexp(_405, _406.exponent);
    imageStore(history_output_tex, ivec2(gl_GlobalInvocationID.xy), (_405 + exp2(vec3(_406.exponent) - vec3(8.0, 8.0, 7.0))).xyzz);
    vec3 _495;
    if (_Globals.sharpen_amount > 0.0)
    {
        float _418 = dot(_405, vec3(0.2125999927520751953125, 0.715200006961822509765625, 0.072200000286102294921875));
        tile_luminance[gl_LocalInvocationID.x][gl_LocalInvocationID.y] = _418;
        barrier();
        uvec2 _422 = gl_LocalInvocationID.xy + uvec2(1u, 0u);
        uvec2 _423 = gl_LocalInvocationID.xy - uvec2(1u, 0u);
        uint _424 = _422.x;
        uint _425 = _422.y;
        uint _426 = _423.x;
        uint _427 = _423.y;
        float _451;
        float _452;
        if (all(lessThan(uvec4(_424, _425, _426, _427), uvec4(8u))))
        {
            float _444 = spvNMax(0.0, 1.0 - (6.0 * (abs(_418 - tile_luminance[_424][_425]) + abs(_418 - tile_luminance[_426][_427]))));
            float _447 = spvNMin(_444, (_Globals.sharpen_amount * _444) * 1.25);
            _451 = _447 * (tile_luminance[_424][_425] + tile_luminance[_426][_427]);
            _452 = _447 * 2.0;
        }
        else
        {
            _451 = 0.0;
            _452 = 0.0;
        }
        uvec2 _453 = gl_LocalInvocationID.xy + uvec2(0u, 1u);
        uvec2 _454 = gl_LocalInvocationID.xy - uvec2(0u, 1u);
        uint _455 = _453.x;
        uint _456 = _453.y;
        uint _457 = _454.x;
        uint _458 = _454.y;
        float _485;
        float _486;
        if (all(lessThan(uvec4(_455, _456, _457, _458), uvec4(8u))))
        {
            float _475 = spvNMax(0.0, 1.0 - (6.0 * (abs(_418 - tile_luminance[_455][_456]) + abs(_418 - tile_luminance[_457][_458]))));
            float _478 = spvNMin(_475, (_Globals.sharpen_amount * _475) * 1.25);
            _485 = (_451 + (tile_luminance[_455][_456] * _478)) + (tile_luminance[_457][_458] * _478);
            _486 = _452 + (_478 * 2.0);
        }
        else
        {
            _485 = _451;
            _486 = _452;
        }
        _495 = _405 * spvNMax(0.0, spvNMax(0.0, (_418 * (_486 + 1.0)) - _485) / spvNMax(9.9999997473787516355514526367188e-06, _418));
    }
    else
    {
        _495 = _405;
    }
    imageStore(output_tex, ivec2(gl_GlobalInvocationID.xy), _495.xyzz);
}

