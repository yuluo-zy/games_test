// DERIVED by SPIRV-Cross from original packaged SPIR-V; NOT original HLSL/OpSource.
#version 460
#extension GL_EXT_nonuniform_qualifier : require
#extension GL_EXT_scalar_block_layout : require
#extension GL_EXT_samplerless_texture_functions : require
#extension GL_EXT_spirv_intrinsics : require
layout(local_size_x = 8, local_size_y = 8, local_size_z = 1) in;

struct DrawIndexedIndirectCommand
{
    uint count;
    uint instanceCount;
    uint firstIndex;
    uint vertexOffset;
    uint firstInstance;
};

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

struct Affine3Packed
{
    float data[12];
};

struct BushXform
{
    Affine3Packed transform;
};

struct FlowerXform
{
    Affine3Packed transform;
    vec3 color;
    uint flower_type;
};

float _147;

layout(set = 0, binding = 13, std430) buffer type_RWStructuredBuffer_DrawIndexedIndirectCommand
{
    DrawIndexedIndirectCommand _m0[];
} _alias_draw_commands;

layout(set = 0, binding = 14, scalar) uniform type_Globals
{
    layout(offset = 0) int draw_indirect_buffer_offset;
    layout(offset = 4) uvec2 terrain_heightmap_dims;
    layout(offset = 16) uvec2 visual_terrain_heightmap_dims;
    layout(offset = 24) uvec2 garden_mask_dims;
    layout(offset = 32) uvec2 buildings_mask_dims;
    layout(offset = 40) uvec2 buildings_mask_non_blurred_dims;
    layout(offset = 48) ivec2 grid_dims;
    layout(offset = 56) uvec2 path_mask_dims;
    layout(offset = 64) uvec2 rocky_terrain_dims;
    layout(offset = 72) uvec2 water_mask_dims;
    layout(offset = 80) uvec2 foliage_exclusion_mask_dims;
} _Globals;

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

layout(set = 0, binding = 20, std430) buffer type_RWStructuredBuffer_BushXform
{
    BushXform _m0[];
} _alias_bush_buffer;

layout(set = 0, binding = 21, std430) buffer type_RWStructuredBuffer_FlowerXform
{
    FlowerXform _m0[];
} _alias_bush_flowers;

layout(set = 0, binding = 4) uniform sampler sampler_llb;
layout(set = 0, binding = 9) uniform sampler sampler_default;
layout(set = 0, binding = 15) uniform texture2D terrain_heightmap;
layout(set = 0, binding = 17) uniform texture2D garden_mask;
layout(set = 0, binding = 18) uniform texture2D buildings_mask;
layout(set = 0, binding = 19) uniform texture2D buildings_mask_non_blurred;
layout(set = 0, binding = 22) uniform texture2D path_mask;
layout(set = 0, binding = 23) uniform texture2D rocky_terrain;
layout(set = 0, binding = 24) uniform texture2D water_mask;
layout(set = 0, binding = 25) uniform texture2D foliage_exclusion_mask;

spirv_instruction(set = "GLSL.std.450", id = 79) float spvNMin(float, float);
spirv_instruction(set = "GLSL.std.450", id = 79) vec2 spvNMin(vec2, vec2);
spirv_instruction(set = "GLSL.std.450", id = 79) vec3 spvNMin(vec3, vec3);
spirv_instruction(set = "GLSL.std.450", id = 79) vec4 spvNMin(vec4, vec4);

void main()
{
    do
    {
        uvec2 _191;
        uint _165 = (gl_GlobalInvocationID.x * gl_GlobalInvocationID.y) + gl_GlobalInvocationID.x;
        uint _167 = _165 + (_165 << 10u);
        uint _169 = _167 ^ (_167 >> 6u);
        uint _171 = _169 + (_169 << 3u);
        uint _173 = _171 ^ (_171 >> 11u);
        float _179 = uintBitsToFloat(((_173 + (_173 << 15u)) & 8388607u) | 1065353216u) - 1.0;
        vec2 _182 = ((((vec2(gl_GlobalInvocationID.xy) + vec2(0.5)) / vec2(_Globals.grid_dims)) - vec2(0.5)) * vec2(130.0)) + vec2(mix(-1.0, 1.0, _179));
        vec2 _184 = (_182 * vec2(0.0076923076994717121124267578125)) + vec2(0.5);
        bool _252;
        do
        {
            _191 = uvec2(_184 * vec2(_Globals.path_mask_dims));
            vec4 _209 = textureGatherOffset(sampler2D(terrain_heightmap, sampler_default), _184, ivec2(0));
            float _214 = _209.z;
            float _215 = _209.y;
            float _217 = _209.w;
            float _218 = _209.x;
            bool _239;
            if (!(texelFetch(path_mask, ivec2(_191), int(0u)).x > 0.20000000298023223876953125))
            {
                _239 = (int(texelFetch(rocky_terrain, ivec2(_191), int(0u)).x > 0.0) & int(length(((vec2((_214 + _215) - (_217 + _218), (_218 + _215) - (_217 + _214)) * vec2(0.5)) * 26.5) / (vec2(130.0) / vec2(_Globals.terrain_heightmap_dims))) > 0.89999997615814208984375)) != 0;
            }
            else
            {
                _239 = true;
            }
            bool _244;
            if (!_239)
            {
                _244 = texelFetch(water_mask, ivec2(_191), int(0u)).y > 0.300000011920928955078125;
            }
            else
            {
                _244 = true;
            }
            bool _249;
            if (!_244)
            {
                _249 = textureLod(sampler2D(foliage_exclusion_mask, sampler_llb), _184, 0.0).x > 0.0;
            }
            else
            {
                _249 = true;
            }
            if (_249)
            {
                _252 = true;
                break;
            }
            _252 = false;
            break;
        } while(false);
        if (_252)
        {
            break;
        }
        if (texelFetch(garden_mask, ivec2(_191), int(0u)).y > 0.300000011920928955078125)
        {
            break;
        }
        vec4 _263 = textureLod(sampler2D(buildings_mask, sampler_default), _184, 0.0);
        float _264 = _263.x;
        if (textureLod(sampler2D(buildings_mask_non_blurred, sampler_default), _184, 0.0).x > 0.999989986419677734375)
        {
            break;
        }
        bool _425;
        if (_264 > 0.0)
        {
            ivec2 _277 = ivec2(floor(_182));
            vec2 _278 = fract(_182);
            uvec2 _279 = uvec2(_277);
            uint _281 = _279.y;
            uint _283 = _281 + (_281 << 10u);
            uint _285 = _283 ^ (_283 >> 6u);
            uint _287 = _285 + (_285 << 3u);
            uint _289 = _287 ^ (_287 >> 11u);
            uint _295 = (((_279.x * 1664525u) + (_289 + (_289 << 15u))) + 1013904223u) * 1664525u;
            uint _297 = _295 ^ (_295 >> 11u);
            uint _300 = _297 ^ ((_297 << 7u) & 2636928640u);
            uint _303 = _300 ^ ((_300 << 15u) & 4022730752u);
            uvec2 _311 = uvec2(_277 + ivec2(1, 0));
            uint _313 = _311.y;
            uint _315 = _313 + (_313 << 10u);
            uint _317 = _315 ^ (_315 >> 6u);
            uint _319 = _317 + (_317 << 3u);
            uint _321 = _319 ^ (_319 >> 11u);
            uint _327 = (((_311.x * 1664525u) + (_321 + (_321 << 15u))) + 1013904223u) * 1664525u;
            uint _329 = _327 ^ (_327 >> 11u);
            uint _332 = _329 ^ ((_329 << 7u) & 2636928640u);
            uint _335 = _332 ^ ((_332 << 15u) & 4022730752u);
            uvec2 _343 = uvec2(_277 + ivec2(0, 1));
            uint _345 = _343.y;
            uint _347 = _345 + (_345 << 10u);
            uint _349 = _347 ^ (_347 >> 6u);
            uint _351 = _349 + (_349 << 3u);
            uint _353 = _351 ^ (_351 >> 11u);
            uint _359 = (((_343.x * 1664525u) + (_353 + (_353 << 15u))) + 1013904223u) * 1664525u;
            uint _361 = _359 ^ (_359 >> 11u);
            uint _364 = _361 ^ ((_361 << 7u) & 2636928640u);
            uint _367 = _364 ^ ((_364 << 15u) & 4022730752u);
            uvec2 _375 = uvec2(_277 + ivec2(1));
            uint _377 = _375.y;
            uint _379 = _377 + (_377 << 10u);
            uint _381 = _379 ^ (_379 >> 6u);
            uint _383 = _381 + (_381 << 3u);
            uint _385 = _383 ^ (_383 >> 11u);
            uint _391 = (((_375.x * 1664525u) + (_385 + (_385 << 15u))) + 1013904223u) * 1664525u;
            uint _393 = _391 ^ (_391 >> 11u);
            uint _396 = _393 ^ ((_393 << 7u) & 2636928640u);
            uint _399 = _396 ^ ((_396 << 15u) & 4022730752u);
            vec2 _409 = (_278 * _278) * (vec2(3.0) - (_278 * 2.0));
            float _410 = _409.x;
            float _411 = 1.0 - _410;
            float _413 = _409.y;
            _425 = ((((1.0 - _413) * (((uintBitsToFloat(((_303 ^ (_303 >> 18u)) & 8388607u) | 1065353216u) - 1.0) * _411) + ((uintBitsToFloat(((_335 ^ (_335 >> 18u)) & 8388607u) | 1065353216u) - 1.0) * _410))) + (((uintBitsToFloat(((_367 ^ (_367 >> 18u)) & 8388607u) | 1065353216u) - 1.0) * _411) * _413)) + (((uintBitsToFloat(((_399 ^ (_399 >> 18u)) & 8388607u) | 1065353216u) - 1.0) * _410) * _413)) > 0.5;
        }
        else
        {
            _425 = false;
        }
        if (_425)
        {
            vec3 _428 = _182.xxy;
            float _435 = mix(-2.5, 24.0, textureLod(sampler2D(terrain_heightmap, sampler_default), (_182.xy * vec2(0.0076923076994717121124267578125)) + vec2(0.5), 0.0).x);
            _428.y = _435;
            vec3 _444;
            if (frame_constants.glade_theme_id == 4u)
            {
                vec3 _443 = _428;
                _443.y = _435 - 0.5;
                _444 = _443;
            }
            else
            {
                _444 = _428;
            }
            float _458;
            do
            {
                float _448 = abs(_263.y - 0.4000000059604644775390625);
                if (_448 > 0.300000011920928955078125)
                {
                    _458 = 0.0;
                    break;
                }
                float _452 = _448 * 3.3333332538604736328125;
                _458 = 1.0 - ((_452 * _452) * (3.0 - (_448 * 6.666666507720947265625)));
                break;
            } while(false);
            uint _459 = gl_GlobalInvocationID.x + gl_GlobalInvocationID.y;
            uint _461 = _459 + (_459 << 10u);
            uint _463 = _461 ^ (_461 >> 6u);
            uint _465 = _463 + (_463 << 3u);
            uint _467 = _465 ^ (_465 >> 11u);
            float _474 = mix(0.800000011920928955078125, 1.2000000476837158203125, uintBitsToFloat(((_467 + (_467 << 15u)) & 8388607u) | 1065353216u) - 1.0);
            vec3 _475 = vec3(_474);
            vec3 _478 = _475 * mix(0.20000000298023223876953125, 1.0, sqrt(_458));
            float _480 = _478.y * _474;
            vec3 _485 = vec3(spvNMin(_478.x, 0.75), _480, spvNMin(_478.z, 0.75));
            _485.y = spvNMin(_480, 1.2000000476837158203125);
            vec3 _488 = _485 * 0.75;
            float _502;
            do
            {
                float _492 = abs(_264 - 0.4000000059604644775390625);
                if (_492 > 0.300000011920928955078125)
                {
                    _502 = 0.0;
                    break;
                }
                float _496 = _492 * 3.3333332538604736328125;
                _502 = 1.0 - ((_496 * _496) * (3.0 - (_492 * 6.666666507720947265625)));
                break;
            } while(false);
            if ((vec3(spvNMin((_475 * mix(0.20000000298023223876953125, 1.0, sqrt(_502))).x, 0.75), _147, _147) * 0.75).x < 0.300000011920928955078125)
            {
                break;
            }
            float _530 = 3.1400001049041748046875 * _179;
            vec3 _531 = normalize(vec3(0.0, 1.0, 0.0));
            float _532 = sin(_530);
            float _533 = cos(_530);
            float _534 = 1.0 - _533;
            float _536 = _534 * _531.x;
            float _540 = _536 * _531.y;
            float _542 = _531.z * _532;
            float _544 = _534 * _531.z;
            float _545 = _544 * _531.x;
            float _546 = _531.y * _532;
            float _549 = _534 * _531.y;
            float _552 = _549 * _531.z;
            float _553 = _531.x * _532;
            mat4 _564 = transpose(mat4(vec4((_536 * _531.x) + _533, _540 - _542, _545 + _546, 0.0), vec4(_540 + _542, (_549 * _531.y) + _533, _552 - _553, 0.0), vec4(_545 - _546, _552 + _553, (_544 * _531.z) + _533, 0.0), vec4(0.0, 0.0, 0.0, 1.0))) * ((mat4(vec4(_488.x, 0.0, 0.0, 0.0), vec4(0.0, _488.y, 0.0, 0.0), vec4(0.0, 0.0, _488.z, 0.0), vec4(0.0, 0.0, 0.0, 1.0)) * mat4(vec4(1.0, 0.0, 0.0, 0.0), vec4(0.0, 1.0, 0.0, 0.0), vec4(0.0, 0.0, 1.0, 0.0), vec4(0.0, 0.0, 0.0, 1.0))) * mat4(vec4(1.0, 0.0, 0.0, _444.x), vec4(0.0, 1.0, 0.0, _444.y), vec4(0.0, 0.0, 1.0, _444.z), vec4(0.0, 0.0, 0.0, 1.0)));
            uint _578 = atomicAdd(_alias_draw_commands._m0[10u].instanceCount, 1u);
            Affine3Packed _583 = Affine3Packed(float[](_564[0].x, _564[1].x, _564[2].x, _564[0].y, _564[1].y, _564[2].y, _564[0].z, _564[1].z, _564[2].z, _564[0].w, _564[1].w, _564[2].w));
            _alias_bush_buffer._m0[uint(int(_578))] = BushXform(_583);
            uint _588 = ((gl_GlobalInvocationID.x * 2u) + (gl_GlobalInvocationID.y / 5u)) + 123u;
            uint _590 = _588 + (_588 << 10u);
            uint _592 = _590 ^ (_590 >> 6u);
            uint _594 = _592 + (_592 << 3u);
            uint _596 = _594 ^ (_594 >> 11u);
            if ((uintBitsToFloat(((_596 + (_596 << 15u)) & 8388607u) | 1065353216u) - 1.0) > 0.699999988079071044921875)
            {
                uint _607 = atomicAdd(_alias_draw_commands._m0[11u].instanceCount, 1u);
                _alias_bush_flowers._m0[uint(int(_607))] = FlowerXform(_583, vec3(-1.0), 2u);
            }
        }
        break;
    } while(false);
}

