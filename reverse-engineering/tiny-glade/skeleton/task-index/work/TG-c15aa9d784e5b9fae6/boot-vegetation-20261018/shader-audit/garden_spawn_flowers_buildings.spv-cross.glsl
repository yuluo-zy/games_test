// DERIVED by SPIRV-Cross from original packaged SPIR-V; NOT original HLSL/OpSource.
#version 460
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

struct Affine3Packed
{
    float data[12];
};

struct FlowerXform
{
    Affine3Packed transform;
    vec3 color;
    uint flower_type;
};

uint _123;

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

layout(set = 0, binding = 20, std430) buffer type_RWStructuredBuffer_FlowerXform
{
    FlowerXform _m0[];
} _alias_flower_buffer;

layout(set = 0, binding = 4) uniform sampler sampler_llb;
layout(set = 0, binding = 9) uniform sampler sampler_default;
layout(set = 0, binding = 15) uniform texture2D terrain_heightmap;
layout(set = 0, binding = 17) uniform texture2D garden_mask;
layout(set = 0, binding = 18) uniform texture2D buildings_mask;
layout(set = 0, binding = 19) uniform texture2D buildings_mask_non_blurred;
layout(set = 0, binding = 21) uniform texture2D path_mask;
layout(set = 0, binding = 22) uniform texture2D rocky_terrain;
layout(set = 0, binding = 23) uniform texture2D water_mask;
layout(set = 0, binding = 24) uniform texture2D foliage_exclusion_mask;

spirv_instruction(set = "GLSL.std.450", id = 79) float spvNMin(float, float);
spirv_instruction(set = "GLSL.std.450", id = 79) vec2 spvNMin(vec2, vec2);
spirv_instruction(set = "GLSL.std.450", id = 79) vec3 spvNMin(vec3, vec3);
spirv_instruction(set = "GLSL.std.450", id = 79) vec4 spvNMin(vec4, vec4);

void main()
{
    vec2 _208;
    vec2 _232;
    vec2 _154 = (((vec2(gl_GlobalInvocationID.xy) + vec2(0.5)) / vec2(_Globals.grid_dims)) - vec2(0.5)) * vec2(130.0);
    uint _161 = (((gl_GlobalInvocationID.x * gl_GlobalInvocationID.y) / 2u) + (gl_GlobalInvocationID.x * 20u)) + 100u;
    uint _163 = _161 + (_161 << 10u);
    uint _165 = _163 ^ (_163 >> 6u);
    uint _167 = _165 + (_165 << 3u);
    uint _169 = _167 ^ (_167 >> 11u);
    float _175 = uintBitsToFloat(((_169 + (_169 << 15u)) & 8388607u) | 1065353216u) - 1.0;
    vec2 _178 = _154 + vec2(mix(-0.75, 0.75, _175));
    uint _181 = uint((_175 * 100.0) - 341.0);
    uint _183 = _181 + (_181 << 10u);
    uint _185 = _183 ^ (_183 >> 6u);
    uint _187 = _185 + (_185 << 3u);
    uint _189 = _187 ^ (_187 >> 11u);
    vec2 _198 = _154 + vec2(mix(-0.75, 0.75, uintBitsToFloat(((_189 + (_189 << 15u)) & 8388607u) | 1065353216u) - 1.0));
    ivec2 _199 = ivec2(gl_GlobalInvocationID.xy);
    uint _598;
    uint _599;
    bool _600;
    do
    {
        vec2 _205;
        uvec2 _210;
        bool _271;
        do
        {
            _205 = (_178 * vec2(0.0076923076994717121124267578125)) + vec2(0.5);
            _208 = vec2(_Globals.path_mask_dims);
            _210 = uvec2(_205 * _208);
            vec4 _228 = textureGatherOffset(sampler2D(terrain_heightmap, sampler_default), _205, ivec2(0));
            _232 = vec2(130.0) / vec2(_Globals.terrain_heightmap_dims);
            float _233 = _228.z;
            float _234 = _228.y;
            float _236 = _228.w;
            float _237 = _228.x;
            bool _258;
            if (!(texelFetch(path_mask, ivec2(_210), int(0u)).x > 0.20000000298023223876953125))
            {
                _258 = (int(texelFetch(rocky_terrain, ivec2(_210), int(0u)).x > 0.0) & int(length(((vec2((_233 + _234) - (_236 + _237), (_237 + _234) - (_236 + _233)) * vec2(0.5)) * 26.5) / _232) > 0.89999997615814208984375)) != 0;
            }
            else
            {
                _258 = true;
            }
            bool _263;
            if (!_258)
            {
                _263 = texelFetch(water_mask, ivec2(_210), int(0u)).y > 0.300000011920928955078125;
            }
            else
            {
                _263 = true;
            }
            bool _268;
            if (!_263)
            {
                _268 = textureLod(sampler2D(foliage_exclusion_mask, sampler_llb), _205, 0.0).x > 0.0;
            }
            else
            {
                _268 = true;
            }
            if (_268)
            {
                _271 = true;
                break;
            }
            _271 = false;
            break;
        } while(false);
        if (_271)
        {
            _598 = _123;
            _599 = _123;
            _600 = false;
            break;
        }
        if (texelFetch(garden_mask, ivec2(_210), int(0u)).y > 0.300000011920928955078125)
        {
            _598 = _123;
            _599 = _123;
            _600 = false;
            break;
        }
        vec4 _282 = textureLod(sampler2D(buildings_mask, sampler_default), _205, 0.0);
        float _298;
        do
        {
            float _288 = abs(_282.y - 0.550000011920928955078125);
            if (_288 > 0.300000011920928955078125)
            {
                _298 = 0.0;
                break;
            }
            float _292 = _288 * 3.3333332538604736328125;
            _298 = 1.0 - ((_292 * _292) * (3.0 - (_288 * 6.666666507720947265625)));
            break;
        } while(false);
        if (textureLod(sampler2D(buildings_mask_non_blurred, sampler_default), _205, 0.0).x > 0.999989986419677734375)
        {
            _598 = _123;
            _599 = _123;
            _600 = false;
            break;
        }
        vec2 _306 = _178 * 0.5;
        ivec2 _308 = ivec2(floor(_306));
        vec2 _309 = fract(_306);
        uvec2 _310 = uvec2(_308);
        uint _312 = _310.y;
        uint _314 = _312 + (_312 << 10u);
        uint _316 = _314 ^ (_314 >> 6u);
        uint _318 = _316 + (_316 << 3u);
        uint _320 = _318 ^ (_318 >> 11u);
        uint _326 = (((_310.x * 1664525u) + (_320 + (_320 << 15u))) + 1013904223u) * 1664525u;
        uint _328 = _326 ^ (_326 >> 11u);
        uint _331 = _328 ^ ((_328 << 7u) & 2636928640u);
        uint _334 = _331 ^ ((_331 << 15u) & 4022730752u);
        uvec2 _342 = uvec2(_308 + ivec2(1, 0));
        uint _344 = _342.y;
        uint _346 = _344 + (_344 << 10u);
        uint _348 = _346 ^ (_346 >> 6u);
        uint _350 = _348 + (_348 << 3u);
        uint _352 = _350 ^ (_350 >> 11u);
        uint _358 = (((_342.x * 1664525u) + (_352 + (_352 << 15u))) + 1013904223u) * 1664525u;
        uint _360 = _358 ^ (_358 >> 11u);
        uint _363 = _360 ^ ((_360 << 7u) & 2636928640u);
        uint _366 = _363 ^ ((_363 << 15u) & 4022730752u);
        uvec2 _374 = uvec2(_308 + ivec2(0, 1));
        uint _376 = _374.y;
        uint _378 = _376 + (_376 << 10u);
        uint _380 = _378 ^ (_378 >> 6u);
        uint _382 = _380 + (_380 << 3u);
        uint _384 = _382 ^ (_382 >> 11u);
        uint _390 = (((_374.x * 1664525u) + (_384 + (_384 << 15u))) + 1013904223u) * 1664525u;
        uint _392 = _390 ^ (_390 >> 11u);
        uint _395 = _392 ^ ((_392 << 7u) & 2636928640u);
        uint _398 = _395 ^ ((_395 << 15u) & 4022730752u);
        uvec2 _406 = uvec2(_308 + ivec2(1));
        uint _408 = _406.y;
        uint _410 = _408 + (_408 << 10u);
        uint _412 = _410 ^ (_410 >> 6u);
        uint _414 = _412 + (_412 << 3u);
        uint _416 = _414 ^ (_414 >> 11u);
        uint _422 = (((_406.x * 1664525u) + (_416 + (_416 << 15u))) + 1013904223u) * 1664525u;
        uint _424 = _422 ^ (_422 >> 11u);
        uint _427 = _424 ^ ((_424 << 7u) & 2636928640u);
        uint _430 = _427 ^ ((_427 << 15u) & 4022730752u);
        vec2 _440 = (_309 * _309) * (vec2(3.0) - (_309 * 2.0));
        float _441 = _440.x;
        float _442 = 1.0 - _441;
        float _444 = _440.y;
        if ((int(((((1.0 - _444) * (((uintBitsToFloat(((_334 ^ (_334 >> 18u)) & 8388607u) | 1065353216u) - 1.0) * _442) + ((uintBitsToFloat(((_366 ^ (_366 >> 18u)) & 8388607u) | 1065353216u) - 1.0) * _441))) + (((uintBitsToFloat(((_398 ^ (_398 >> 18u)) & 8388607u) | 1065353216u) - 1.0) * _442) * _444)) + (((uintBitsToFloat(((_430 ^ (_430 >> 18u)) & 8388607u) | 1065353216u) - 1.0) * _441) * _444)) > 0.60000002384185791015625) & int(_282.x > 0.0)) != 0)
        {
            int _463 = _199.x;
            int _464 = _199.y;
            uint _466 = uint(_463 + _464);
            uint _468 = _466 + (_466 << 10u);
            uint _470 = _468 ^ (_468 >> 6u);
            uint _472 = _470 + (_470 << 3u);
            uint _474 = _472 ^ (_472 >> 11u);
            float _481 = mix(0.800000011920928955078125, 1.2000000476837158203125, uintBitsToFloat(((_474 + (_474 << 15u)) & 8388607u) | 1065353216u) - 1.0);
            vec3 _484 = vec3(_481) * mix(0.20000000298023223876953125, 1.0, _298);
            float _489 = spvNMin(_484.x, 0.75);
            if (_489 < 0.4000000059604644775390625)
            {
                _598 = 1013904223u;
                _599 = 1664525u;
                _600 = true;
                break;
            }
            vec3 _496 = vec3(_489, (_484.y * 1.2000000476837158203125) * _481, spvNMin(_484.z, 0.75)) * 2.0;
            uint _513 = uint((_463 * _464) + _463);
            uint _515 = _513 + (_513 << 10u);
            uint _517 = _515 ^ (_515 >> 6u);
            uint _519 = _517 + (_517 << 3u);
            uint _521 = _519 ^ (_519 >> 11u);
            float _528 = 3.1400001049041748046875 * (uintBitsToFloat(((_521 + (_521 << 15u)) & 8388607u) | 1065353216u) - 1.0);
            vec3 _529 = normalize(vec3(0.0, 1.0, 0.0));
            float _530 = sin(_528);
            float _531 = cos(_528);
            float _532 = 1.0 - _531;
            float _534 = _532 * _529.x;
            float _538 = _534 * _529.y;
            float _540 = _529.z * _530;
            float _542 = _532 * _529.z;
            float _543 = _542 * _529.x;
            float _544 = _529.y * _530;
            float _547 = _532 * _529.y;
            float _550 = _547 * _529.z;
            float _551 = _529.x * _530;
            mat4 _562 = transpose(mat4(vec4((_534 * _529.x) + _531, _538 - _540, _543 + _544, 0.0), vec4(_538 + _540, (_547 * _529.y) + _531, _550 - _551, 0.0), vec4(_543 - _544, _550 + _551, (_542 * _529.z) + _531, 0.0), vec4(0.0, 0.0, 0.0, 1.0))) * ((mat4(vec4(_496.x, 0.0, 0.0, 0.0), vec4(0.0, _496.y, 0.0, 0.0), vec4(0.0, 0.0, _496.z, 0.0), vec4(0.0, 0.0, 0.0, 1.0)) * mat4(vec4(1.0, 0.0, 0.0, 0.0), vec4(0.0, 1.0, 0.0, 0.0), vec4(0.0, 0.0, 1.0, 0.0), vec4(0.0, 0.0, 0.0, 1.0))) * mat4(vec4(1.0, 0.0, 0.0, _178.x), vec4(0.0, 1.0, 0.0, 0.5), vec4(0.0, 0.0, 1.0, _178.y), vec4(0.0, 0.0, 0.0, 1.0)));
            float _563 = _562[0].x;
            float _564 = _562[1].x;
            float _565 = _562[2].x;
            float _566 = _562[0].y;
            float _567 = _562[1].y;
            float _568 = _562[2].y;
            float _569 = _562[0].z;
            float _570 = _562[1].z;
            float _571 = _562[2].z;
            float _572 = _562[0].w;
            float _573 = _562[1].w;
            float _574 = _562[2].w;
            uint _591 = atomicAdd(_alias_draw_commands._m0[11u].instanceCount, 1u);
            _alias_flower_buffer._m0[uint(int(_591))] = FlowerXform(Affine3Packed(float[](_563, _564, _565, _566, _567, _568, _569, _570, _571, _572, _573 + mix(-2.5, 24.0, textureLod(sampler2D(terrain_heightmap, sampler_default), ((vec4(0.0, 0.0, 0.0, 1.0) * transpose(mat4x3(vec3(_563, _564, _565), vec3(_566, _567, _568), vec3(_569, _570, _571), vec3(_572, _573, _574)))).xz * vec2(0.0076923076994717121124267578125)) + vec2(0.5), 0.0).x), _574)), vec3(-1.0), 0u);
        }
        _598 = 1013904223u;
        _599 = 1664525u;
        _600 = true;
        break;
    } while(false);
    do
    {
        vec2 _606;
        uvec2 _608;
        bool _658;
        do
        {
            _606 = (_198 * vec2(0.0076923076994717121124267578125)) + vec2(0.5);
            _608 = uvec2(_606 * _208);
            vec4 _619 = textureGatherOffset(sampler2D(terrain_heightmap, sampler_default), _606, ivec2(0));
            float _620 = _619.z;
            float _621 = _619.y;
            float _623 = _619.w;
            float _624 = _619.x;
            bool _645;
            if (!(texelFetch(path_mask, ivec2(_608), int(0u)).x > 0.20000000298023223876953125))
            {
                _645 = (int(texelFetch(rocky_terrain, ivec2(_608), int(0u)).x > 0.0) & int(length(((vec2((_620 + _621) - (_623 + _624), (_624 + _621) - (_623 + _620)) * vec2(0.5)) * 26.5) / _232) > 0.89999997615814208984375)) != 0;
            }
            else
            {
                _645 = true;
            }
            bool _650;
            if (!_645)
            {
                _650 = texelFetch(water_mask, ivec2(_608), int(0u)).y > 0.300000011920928955078125;
            }
            else
            {
                _650 = true;
            }
            bool _655;
            if (!_650)
            {
                _655 = textureLod(sampler2D(foliage_exclusion_mask, sampler_llb), _606, 0.0).x > 0.0;
            }
            else
            {
                _655 = true;
            }
            if (_655)
            {
                _658 = true;
                break;
            }
            _658 = false;
            break;
        } while(false);
        if (_658)
        {
            break;
        }
        if (texelFetch(garden_mask, ivec2(_608), int(0u)).y > 0.300000011920928955078125)
        {
            break;
        }
        vec4 _669 = textureLod(sampler2D(buildings_mask, sampler_default), _606, 0.0);
        float _685;
        do
        {
            float _675 = abs(_669.y - 0.550000011920928955078125);
            if (_675 > 0.4000000059604644775390625)
            {
                _685 = 0.0;
                break;
            }
            float _679 = _675 * 2.5;
            _685 = 1.0 - ((_679 * _679) * (3.0 - (_675 * 5.0)));
            break;
        } while(false);
        if (textureLod(sampler2D(buildings_mask_non_blurred, sampler_default), _606, 0.0).x > 0.999989986419677734375)
        {
            break;
        }
        vec2 _694 = (_198 * 0.5) + vec2(100.0);
        ivec2 _696 = ivec2(floor(_694));
        vec2 _697 = fract(_694);
        uvec2 _698 = uvec2(_696);
        uint _700 = _698.y;
        uint _702 = _700 + (_700 << 10u);
        uint _704 = _702 ^ (_702 >> 6u);
        uint _706 = _704 + (_704 << 3u);
        uint _708 = _706 ^ (_706 >> 11u);
        bool _711 = _600 ? _600 : true;
        uint _712 = _600 ? _599 : 1664525u;
        uint _713 = _600 ? _598 : 1013904223u;
        uint _717 = (((_698.x * _712) + (_708 + (_708 << 15u))) + _713) * _712;
        uint _719 = _717 ^ (_717 >> 11u);
        uint _722 = _719 ^ ((_719 << 7u) & 2636928640u);
        uint _725 = _722 ^ ((_722 << 15u) & 4022730752u);
        uvec2 _733 = uvec2(_696 + ivec2(1, 0));
        uint _735 = _733.y;
        uint _737 = _735 + (_735 << 10u);
        uint _739 = _737 ^ (_737 >> 6u);
        uint _741 = _739 + (_739 << 3u);
        uint _743 = _741 ^ (_741 >> 11u);
        bool _746 = _711 ? _711 : true;
        uint _747 = _711 ? _712 : 1664525u;
        uint _748 = _711 ? _713 : 1013904223u;
        uint _752 = (((_733.x * _747) + (_743 + (_743 << 15u))) + _748) * _747;
        uint _754 = _752 ^ (_752 >> 11u);
        uint _757 = _754 ^ ((_754 << 7u) & 2636928640u);
        uint _760 = _757 ^ ((_757 << 15u) & 4022730752u);
        uvec2 _768 = uvec2(_696 + ivec2(0, 1));
        uint _770 = _768.y;
        uint _772 = _770 + (_770 << 10u);
        uint _774 = _772 ^ (_772 >> 6u);
        uint _776 = _774 + (_774 << 3u);
        uint _778 = _776 ^ (_776 >> 11u);
        bool _781 = _746 ? _746 : true;
        uint _782 = _746 ? _747 : 1664525u;
        uint _783 = _746 ? _748 : 1013904223u;
        uint _787 = (((_768.x * _782) + (_778 + (_778 << 15u))) + _783) * _782;
        uint _789 = _787 ^ (_787 >> 11u);
        uint _792 = _789 ^ ((_789 << 7u) & 2636928640u);
        uint _795 = _792 ^ ((_792 << 15u) & 4022730752u);
        uvec2 _803 = uvec2(_696 + ivec2(1));
        uint _805 = _803.y;
        uint _807 = _805 + (_805 << 10u);
        uint _809 = _807 ^ (_807 >> 6u);
        uint _811 = _809 + (_809 << 3u);
        uint _813 = _811 ^ (_811 >> 11u);
        uint _816 = _781 ? _782 : 1664525u;
        uint _821 = (((_803.x * _816) + (_813 + (_813 << 15u))) + (_781 ? _783 : 1013904223u)) * _816;
        uint _823 = _821 ^ (_821 >> 11u);
        uint _826 = _823 ^ ((_823 << 7u) & 2636928640u);
        uint _829 = _826 ^ ((_826 << 15u) & 4022730752u);
        vec2 _839 = (_697 * _697) * (vec2(3.0) - (_697 * 2.0));
        float _840 = _839.x;
        float _841 = 1.0 - _840;
        float _843 = _839.y;
        if ((int(((((1.0 - _843) * (((uintBitsToFloat(((_725 ^ (_725 >> 18u)) & 8388607u) | 1065353216u) - 1.0) * _841) + ((uintBitsToFloat(((_760 ^ (_760 >> 18u)) & 8388607u) | 1065353216u) - 1.0) * _840))) + (((uintBitsToFloat(((_795 ^ (_795 >> 18u)) & 8388607u) | 1065353216u) - 1.0) * _841) * _843)) + (((uintBitsToFloat(((_829 ^ (_829 >> 18u)) & 8388607u) | 1065353216u) - 1.0) * _840) * _843)) > 0.60000002384185791015625) & int(_669.x > 0.0)) != 0)
        {
            int _862 = _199.x;
            int _863 = _199.y;
            uint _865 = uint(_862 + _863);
            uint _867 = _865 + (_865 << 10u);
            uint _869 = _867 ^ (_867 >> 6u);
            uint _871 = _869 + (_869 << 3u);
            uint _873 = _871 ^ (_871 >> 11u);
            float _880 = mix(0.800000011920928955078125, 1.2000000476837158203125, uintBitsToFloat(((_873 + (_873 << 15u)) & 8388607u) | 1065353216u) - 1.0);
            vec3 _883 = vec3(_880) * mix(0.20000000298023223876953125, 1.0, _685);
            float _888 = spvNMin(_883.x, 0.75);
            if (_888 < 0.4000000059604644775390625)
            {
                break;
            }
            vec3 _895 = vec3(_888, (_883.y * 1.2000000476837158203125) * _880, spvNMin(_883.z, 0.75)) * 2.0;
            uint _912 = uint((_862 * _863) + _862);
            uint _914 = _912 + (_912 << 10u);
            uint _916 = _914 ^ (_914 >> 6u);
            uint _918 = _916 + (_916 << 3u);
            uint _920 = _918 ^ (_918 >> 11u);
            float _927 = 3.1400001049041748046875 * (uintBitsToFloat(((_920 + (_920 << 15u)) & 8388607u) | 1065353216u) - 1.0);
            vec3 _928 = normalize(vec3(0.0, 1.0, 0.0));
            float _929 = sin(_927);
            float _930 = cos(_927);
            float _931 = 1.0 - _930;
            float _933 = _931 * _928.x;
            float _937 = _933 * _928.y;
            float _939 = _928.z * _929;
            float _941 = _931 * _928.z;
            float _942 = _941 * _928.x;
            float _943 = _928.y * _929;
            float _946 = _931 * _928.y;
            float _949 = _946 * _928.z;
            float _950 = _928.x * _929;
            mat4 _961 = transpose(mat4(vec4((_933 * _928.x) + _930, _937 - _939, _942 + _943, 0.0), vec4(_937 + _939, (_946 * _928.y) + _930, _949 - _950, 0.0), vec4(_942 - _943, _949 + _950, (_941 * _928.z) + _930, 0.0), vec4(0.0, 0.0, 0.0, 1.0))) * ((mat4(vec4(_895.x, 0.0, 0.0, 0.0), vec4(0.0, _895.y, 0.0, 0.0), vec4(0.0, 0.0, _895.z, 0.0), vec4(0.0, 0.0, 0.0, 1.0)) * mat4(vec4(1.0, 0.0, 0.0, 0.0), vec4(0.0, 1.0, 0.0, 0.0), vec4(0.0, 0.0, 1.0, 0.0), vec4(0.0, 0.0, 0.0, 1.0))) * mat4(vec4(1.0, 0.0, 0.0, _198.x), vec4(0.0, 1.0, 0.0, 0.5), vec4(0.0, 0.0, 1.0, _198.y), vec4(0.0, 0.0, 0.0, 1.0)));
            float _962 = _961[0].x;
            float _963 = _961[1].x;
            float _964 = _961[2].x;
            float _965 = _961[0].y;
            float _966 = _961[1].y;
            float _967 = _961[2].y;
            float _968 = _961[0].z;
            float _969 = _961[1].z;
            float _970 = _961[2].z;
            float _971 = _961[0].w;
            float _972 = _961[1].w;
            float _973 = _961[2].w;
            uint _990 = atomicAdd(_alias_draw_commands._m0[11u].instanceCount, 1u);
            _alias_flower_buffer._m0[uint(int(_990))] = FlowerXform(Affine3Packed(float[](_962, _963, _964, _965, _966, _967, _968, _969, _970, _971, _972 + mix(-2.5, 24.0, textureLod(sampler2D(terrain_heightmap, sampler_default), ((vec4(0.0, 0.0, 0.0, 1.0) * transpose(mat4x3(vec3(_962, _963, _964), vec3(_965, _966, _967), vec3(_968, _969, _970), vec3(_971, _972, _973)))).xz * vec2(0.0076923076994717121124267578125)) + vec2(0.5), 0.0).x), _973)), vec3(-1.0), 1u);
        }
        break;
    } while(false);
}

