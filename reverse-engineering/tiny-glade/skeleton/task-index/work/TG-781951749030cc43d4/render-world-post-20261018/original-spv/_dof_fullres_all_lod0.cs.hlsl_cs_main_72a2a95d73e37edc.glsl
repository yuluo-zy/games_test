#version 450
#extension GL_EXT_nonuniform_qualifier : require
#extension GL_EXT_scalar_block_layout : require
#extension GL_EXT_spirv_intrinsics : require
#extension GL_EXT_samplerless_texture_functions : require
#if defined(GL_EXT_control_flow_attributes)
#extension GL_EXT_control_flow_attributes : require
#define SPIRV_CROSS_FLATTEN [[flatten]]
#define SPIRV_CROSS_BRANCH [[dont_flatten]]
#define SPIRV_CROSS_UNROLL [[unroll]]
#define SPIRV_CROSS_LOOP [[dont_unroll]]
#else
#define SPIRV_CROSS_FLATTEN
#define SPIRV_CROSS_BRANCH
#define SPIRV_CROSS_UNROLL
#define SPIRV_CROSS_LOOP
#endif
#extension GL_KHR_shader_subgroup_basic : require
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

struct DofTileOffsets
{
    uint offsets[8];
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

layout(set = 0, binding = 13, scalar) uniform type_Globals
{
    layout(offset = 0) float bokeh_anisotropy;
    layout(offset = 4) uvec2 blue_noise_texture_dims;
    layout(offset = 16) uvec2 color_tex_dims;
    layout(offset = 24) uvec2 coc_tiles_dilated_tex_dims;
    layout(offset = 32) uvec2 depth_tex_dims;
    layout(offset = 40) uvec2 coc_tex_dims;
    layout(offset = 48) uvec2 img_output_dims;
} _Globals;

layout(set = 0, binding = 20, std430) readonly buffer type_StructuredBuffer_DofTileOffsets
{
    DofTileOffsets _m0[];
} tile_offsets_buffer;

layout(set = 0, binding = 21, std430) readonly buffer type_StructuredBuffer_v2uint
{
    uvec2 _m0[];
} tile_coords_buffer;

layout(set = 0, binding = 2) uniform sampler sampler_nnc;
layout(set = 0, binding = 3) uniform sampler sampler_llc;
layout(set = 0, binding = 14) uniform texture2D blue_noise_texture;
layout(set = 0, binding = 15) uniform texture2D color_tex;
layout(set = 0, binding = 16) uniform texture2D coc_tiles_dilated_tex;
layout(set = 0, binding = 18) uniform texture2D coc_tex;
layout(set = 0, binding = 19, rgba16f) uniform writeonly image2D img_output;

shared uint ring_samples[64][5];

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
    uvec2 _175 = (gl_GlobalInvocationID.xy & uvec2(7u)) + (uvec2(8u) * tile_coords_buffer._m0[tile_offsets_buffer._m0[0u].offsets[1] + gl_WorkGroupID.x]);
    vec2 _182 = (vec2(ivec2(_175)) + vec2(0.5)) / vec2(_Globals.img_output_dims);
    float _193 = spvNMin(float(_Globals.color_tex_dims.y) * 0.0416666679084300994873046875, textureLod(sampler2D(coc_tiles_dilated_tex, sampler_nnc), _182, 0.0).x);
    vec4 _209 = ((texelFetch(blue_noise_texture, ivec2(((_175 + uvec2(13u)) + uvec2(ivec2(int(frame_constants.frame_index * 59u), int(frame_constants.frame_index * 37u)))) & uvec2(255u)), int(0u)) * 255.0) * vec4(0.00390625)) + vec4(0.001953125);
    float _210 = _209.z;
    float _211 = _209.y;
    float _212 = _193 * 0.25;
    float _213 = _209.x;
    bool _214 = _210 < 0.5;
    float _218;
    if (_214)
    {
        _218 = 1.0 - _210;
    }
    else
    {
        _218 = _210;
    }
    float _219 = _214 ? (-1.0) : 1.0;
    vec4 _221;
    float _224;
    float _226;
    float _228;
    _221 = vec4(0.0);
    _224 = 0.0;
    _226 = 0.0;
    _228 = 0.0;
    vec4 _222;
    float _225;
    float _233;
    float _235;
    uint _157[4];
    float _227;
    float _229;
    uint _230 = 0u;
    float _232 = 10000000000.0;
    float _234 = -10000000000.0;
    SPIRV_CROSS_LOOP
    for (; _230 < 4u; _221 = _222, _224 = _225, _226 = _227, _228 = _229, _230++, _232 = _233, _234 = _235)
    {
        float _241 = (float(_230) + _213) * 0.785398185253143310546875;
        vec2 _244 = vec2(cos(_241), sin(_241));
        vec2 _282;
        if (_Globals.bokeh_anisotropy > 0.0)
        {
            vec4 _257 = vec4((_182 - vec2(0.5)) * vec2(2.0, -2.0), 0.0, 1.0) * frame_constants.view_constants[0u].clip_to_view;
            vec2 _261 = _257.xy / vec2(_257.z);
            _261.y = _261.y * (-1.0);
            vec2 _265 = normalize(_261);
            mat2 _270 = mat2(_265, vec2(-_265.y, _265.x));
            float _275 = mix(0.699999988079071044921875, 1.0, exp(((-4.0) * _Globals.bokeh_anisotropy) * dot(_261, _261)));
            _282 = ((_244 * _270) * vec2(_275, 1.0 / _275)) * transpose(_270);
        }
        else
        {
            _282 = _244;
        }
        vec2 _287 = _282 * (vec2(_219) / vec2(_Globals.color_tex_dims));
        float _296 = -_193;
        float _297 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _182 + (_287 * (((-4.0) + _211) * _212)), 0.0).x, _296, _193);
        float _298 = abs(_297);
        float _300 = _213 * 0.20000000298023223876953125;
        float _301 = mix(_224, spvNMax(_224, _298), _300);
        float _313 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _182 + (_287 * (((-3.0) + _211) * _212)), 0.0).x, _296, _193);
        float _316 = mix(_301, spvNMax(_301, abs(_313)), _300);
        ring_samples[gl_LocalInvocationIndex][0u] = packHalf2x16(vec2(_297, 0.0)) | (packHalf2x16(vec2(_313, 0.0)) << 16u);
        float _329 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _182 + (_287 * (((-2.0) + _211) * _212)), 0.0).x, _296, _193);
        float _330 = abs(_329);
        float _332 = mix(_316, spvNMax(_316, _330), _300);
        float _344 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _182 + (_287 * (((-1.0) + _211) * _212)), 0.0).x, _296, _193);
        float _347 = mix(_332, spvNMax(_332, abs(_344)), _300);
        ring_samples[gl_LocalInvocationIndex][1u] = packHalf2x16(vec2(_329, 0.0)) | (packHalf2x16(vec2(_344, 0.0)) << 16u);
        float _359 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _182 + (_287 * (_211 * _212)), 0.0).x, _296, _193);
        float _360 = abs(_359);
        float _362 = mix(_347, spvNMax(_347, _360), _300);
        float _374 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _182 + (_287 * ((1.0 + _211) * _212)), 0.0).x, _296, _193);
        float _377 = mix(_362, spvNMax(_362, abs(_374)), _300);
        ring_samples[gl_LocalInvocationIndex][2u] = packHalf2x16(vec2(_359, 0.0)) | (packHalf2x16(vec2(_374, 0.0)) << 16u);
        float _390 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _182 + (_287 * ((2.0 + _211) * _212)), 0.0).x, _296, _193);
        float _391 = abs(_390);
        float _393 = mix(_377, spvNMax(_377, _391), _300);
        float _405 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _182 + (_287 * ((3.0 + _211) * _212)), 0.0).x, _296, _193);
        float _408 = mix(_393, spvNMax(_393, abs(_405)), _300);
        ring_samples[gl_LocalInvocationIndex][3u] = packHalf2x16(vec2(_390, 0.0)) | (packHalf2x16(vec2(_405, 0.0)) << 16u);
        float _421 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _182 + (_287 * ((4.0 + _211) * _212)), 0.0).x, _296, _193);
        float _422 = abs(_421);
        _225 = mix(_408, spvNMax(_408, _422), _300);
        _233 = spvNMin(spvNMin(spvNMin(spvNMin(spvNMin(_232, _298), _330), _360), _391), _422);
        _235 = spvNMax(spvNMax(spvNMax(spvNMax(spvNMax(_234, _298), _330), _360), _391), _422);
        ring_samples[gl_LocalInvocationIndex][4u] = packHalf2x16(vec2(_421, 0.0));
        for (uint _428 = 0u; _428 < 4u; )
        {
            _157[_428] = 0u;
            _428++;
            continue;
        }
        for (uint _435 = 0u; _435 < 9u; _435++)
        {
            uint _443 = _435 & 1u;
            uint _444 = _435 >> 1u;
            vec2 _158 = vec2(unpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_444] & 65535u).x, unpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_444] >> 16u) & 65535u).x);
            float _459 = (float(int(_435 - 4u)) + _211) * _212;
            for (int _461 = 0; _461 < 4; _461++)
            {
                if ((_459 / (((float(_461) + _218) * 0.5) - 1.0)) >= _158[_443])
                {
                    _157[_461] |= (1u << (_435 & 31u));
                }
            }
        }
        vec4 _484;
        _229 = _228;
        _227 = _226;
        _484 = vec4(0.0);
        float _483;
        float _481;
        vec4 _485;
        for (int _486 = 0; _486 < 4; _229 = _481, _227 = _483, _484 = _485, _486++)
        {
            float _494 = ((float(_486) + _218) * 0.5) - 1.0;
            float _495 = abs(_494);
            _483 = _227 + _495;
            if (_157[_486] == 0u)
            {
                _481 = _229;
                _485 = _484;
                continue;
            }
            bool _501 = _494 >= 0.0;
            float _569;
            float _570;
            uint _571;
            if (_501)
            {
                uint _536 = uint(findLSB(_157[_486]));
                uint _540 = uint(max(0, (int(_536) - 1)));
                uint _542 = _536 >> 1u;
                vec2 _159 = vec2(unpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_542] & 65535u).x, unpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_542] >> 16u) & 65535u).x);
                uint _556 = _540 >> 1u;
                vec2 _160 = vec2(unpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_556] & 65535u).x, unpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_556] >> 16u) & 65535u).x);
                _569 = _159[_536 & 1u];
                _570 = _160[_540 & 1u];
                _571 = _536;
            }
            else
            {
                uint _505 = uint(findMSB(_157[_486]));
                uint _507 = min(8u, (_505 + 1u));
                uint _509 = _505 >> 1u;
                vec2 _161 = vec2(unpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_509] & 65535u).x, unpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_509] >> 16u) & 65535u).x);
                uint _523 = _507 >> 1u;
                vec2 _162 = vec2(unpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_523] & 65535u).x, unpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_523] >> 16u) & 65535u).x);
                _569 = _161[_505 & 1u];
                _570 = _162[_507 & 1u];
                _571 = _505;
            }
            float _577 = (float(int(_571 - 4u)) + _211) * _212;
            float _578 = _577 / _494;
            float _579 = _212 * (_501 ? 1.0 : (-1.0));
            float _582 = _577 - _579;
            float _583 = _570 - (_578 - (_579 / _494));
            float _587 = _495 * float(_157[_486] != 0u);
            float _592 = mix(_582, _577, clamp(_583 / (_583 + (-(_569 - _578))), 0.0, 1.0));
            float _598 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _182 + (_287 * _592), 0.0).x, _296, _193);
            float _606 = log2(spvNMax(0.0, abs(_569) * 0.125)) * smoothstep(4.0, 0.0, _235 - _233);
            float _609 = subgroupShuffle(_606, gl_SubgroupInvocationID ^ 1u);
            float _610 = spvNMin(_606, _609);
            float _612 = subgroupShuffle(_610, gl_SubgroupInvocationID ^ 8u);
            if (_569 > _193)
            {
                _481 = _229;
                _485 = _484;
                continue;
            }
            float _618 = _583 * 0.100000001490116119384765625;
            bool _624;
            if (((_592 / _494) - _598) < _618)
            {
                _624 = (_598 - _569) < _618;
            }
            else
            {
                _624 = false;
            }
            float _636;
            float _637;
            if (_624)
            {
                _636 = _229;
                _637 = _587;
            }
            else
            {
                float _631 = smoothstep(0.0, 30.0, _570) * smoothstep(0.20000000298023223876953125 * _193, 0.0, _569);
                _636 = _229 + (_587 * _631);
                _637 = _587 * (1.0 - _631);
            }
            _481 = _636;
            _485 = _484 + (textureLod(sampler2D(color_tex, sampler_nnc), _182 + (_287 * (_624 ? _592 : _582)), spvNMin(_610, _612)) * _637);
        }
        _222 = _221 + _484;
    }
    float _651 = _228 * smoothstep(0.00999999977648258209228515625 * _221.w, spvNMax(9.9999997473787516355514526367188e-06, 0.20000000298023223876953125 * _221.w), _228);
    vec4 _712;
    SPIRV_CROSS_BRANCH
    if (int(_651 > 0.0) != 0)
    {
        float _658 = -_193;
        vec4 _660;
        _660 = vec4(0.0);
        vec4 _661;
        float _664;
        float _663 = _658;
        uint _665 = 0u;
        for (; _665 < 16u; _660 = _661, _663 = _664, _665++)
        {
            vec2 _676 = fract(fract((vec2(0.75487768650054931640625, 0.56984031200408935546875) * float(_665)) + vec2(0.5)) + _209.xy);
            float _678 = sqrt(_676.x);
            float _680 = _676.y * 6.283185482025146484375;
            vec2 _691 = _182 + (((vec2(cos(_680) * _678, sin(_680) * _678) * 1.0) / vec2(_Globals.color_tex_dims)) * _224);
            vec4 _694 = textureLod(sampler2D(coc_tex, sampler_nnc), _691, 0.0);
            float _695 = _694.x;
            if (_695 > (_663 - 10.0))
            {
                _661 = _660 + textureLod(sampler2D(color_tex, sampler_llc), _691, 0.0);
                _664 = spvNMax(_663, _695);
            }
            else
            {
                _661 = _660;
                _664 = _663;
            }
        }
        _712 = _221 + ((_660 / vec4(spvNMax(1.0, _660.w))) * _651);
    }
    else
    {
        _712 = _221;
    }
    vec4 _714 = _712 / vec4(_226);
    imageStore(img_output, ivec2(_175), vec4((_714 / vec4(spvNMax(1.0000000133514319600180897396058e-10, _714.w))).xyz, 1.0));
}

