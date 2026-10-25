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

uint _119;

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
    layout(offset = 32) uvec2 buildings_mask_non_blurred_dims;
    layout(offset = 40) ivec2 grid_dims;
    layout(offset = 48) uvec2 path_mask_dims;
    layout(offset = 56) uvec2 rocky_terrain_dims;
    layout(offset = 64) uvec2 water_mask_dims;
    layout(offset = 72) uvec2 foliage_exclusion_mask_dims;
} _Globals;

layout(set = 0, binding = 19, std430) buffer type_RWStructuredBuffer_FlowerXform
{
    FlowerXform _m0[];
} _alias_flower_buffer;

layout(set = 0, binding = 4) uniform sampler sampler_llb;
layout(set = 0, binding = 9) uniform sampler sampler_default;
layout(set = 0, binding = 15) uniform texture2D terrain_heightmap;
layout(set = 0, binding = 17) uniform texture2D garden_mask;
layout(set = 0, binding = 18) uniform texture2D buildings_mask_non_blurred;
layout(set = 0, binding = 20) uniform texture2D path_mask;
layout(set = 0, binding = 21) uniform texture2D rocky_terrain;
layout(set = 0, binding = 22) uniform texture2D water_mask;
layout(set = 0, binding = 23) uniform texture2D foliage_exclusion_mask;

spirv_instruction(set = "GLSL.std.450", id = 79) float spvNMin(float, float);
spirv_instruction(set = "GLSL.std.450", id = 79) vec2 spvNMin(vec2, vec2);
spirv_instruction(set = "GLSL.std.450", id = 79) vec3 spvNMin(vec3, vec3);
spirv_instruction(set = "GLSL.std.450", id = 79) vec4 spvNMin(vec4, vec4);

void main()
{
    vec2 _204;
    vec2 _228;
    vec2 _152 = (((vec2(gl_GlobalInvocationID.xy) + vec2(0.5)) / vec2(_Globals.grid_dims)) - vec2(0.5)) * vec2(130.0);
    uint _157 = ((gl_GlobalInvocationID.x * gl_GlobalInvocationID.y) + gl_GlobalInvocationID.x) + 100u;
    uint _159 = _157 + (_157 << 10u);
    uint _161 = _159 ^ (_159 >> 6u);
    uint _163 = _161 + (_161 << 3u);
    uint _165 = _163 ^ (_163 >> 11u);
    float _171 = uintBitsToFloat(((_165 + (_165 << 15u)) & 8388607u) | 1065353216u) - 1.0;
    vec2 _174 = _152 + vec2(mix(-0.75, 0.75, _171));
    uint _177 = uint((_171 * 100.0) - 34.0);
    uint _179 = _177 + (_177 << 10u);
    uint _181 = _179 ^ (_179 >> 6u);
    uint _183 = _181 + (_181 << 3u);
    uint _185 = _183 ^ (_183 >> 11u);
    vec2 _194 = _152 + vec2(mix(-0.75, 0.75, uintBitsToFloat(((_185 + (_185 << 15u)) & 8388607u) | 1065353216u) - 1.0));
    ivec2 _195 = ivec2(gl_GlobalInvocationID.xy);
    uint _602;
    uint _603;
    bool _604;
    do
    {
        vec2 _201;
        bool _267;
        do
        {
            _201 = (_174 * vec2(0.0076923076994717121124267578125)) + vec2(0.5);
            _204 = vec2(_Globals.path_mask_dims);
            uvec2 _206 = uvec2(_201 * _204);
            vec4 _224 = textureGatherOffset(sampler2D(terrain_heightmap, sampler_default), _201, ivec2(0));
            _228 = vec2(130.0) / vec2(_Globals.terrain_heightmap_dims);
            float _229 = _224.z;
            float _230 = _224.y;
            float _232 = _224.w;
            float _233 = _224.x;
            bool _254;
            if (!(texelFetch(path_mask, ivec2(_206), int(0u)).x > 0.20000000298023223876953125))
            {
                _254 = (int(texelFetch(rocky_terrain, ivec2(_206), int(0u)).x > 0.0) & int(length(((vec2((_229 + _230) - (_232 + _233), (_233 + _230) - (_232 + _229)) * vec2(0.5)) * 26.5) / _228) > 0.89999997615814208984375)) != 0;
            }
            else
            {
                _254 = true;
            }
            bool _259;
            if (!_254)
            {
                _259 = texelFetch(water_mask, ivec2(_206), int(0u)).y > 0.300000011920928955078125;
            }
            else
            {
                _259 = true;
            }
            bool _264;
            if (!_259)
            {
                _264 = textureLod(sampler2D(foliage_exclusion_mask, sampler_llb), _201, 0.0).x > 0.0;
            }
            else
            {
                _264 = true;
            }
            if (_264)
            {
                _267 = true;
                break;
            }
            _267 = false;
            break;
        } while(false);
        if (_267)
        {
            _602 = _119;
            _603 = _119;
            _604 = false;
            break;
        }
        if (textureLod(sampler2D(buildings_mask_non_blurred, sampler_default), _201, 0.0).x > 0.5)
        {
            _602 = _119;
            _603 = _119;
            _604 = false;
            break;
        }
        vec4 _279 = textureLod(sampler2D(garden_mask, sampler_default), _201, 0.0);
        float _280 = _279.y;
        float _294;
        do
        {
            float _284 = abs(_280 - 0.60000002384185791015625);
            if (_284 > 0.20000000298023223876953125)
            {
                _294 = 0.0;
                break;
            }
            float _288 = _284 * 5.0;
            _294 = 1.0 - ((_288 * _288) * (3.0 - (_284 * 10.0)));
            break;
        } while(false);
        float _298 = smoothstep(0.89999997615814208984375, 1.0, textureLod(sampler2D(garden_mask, sampler_default), _201, 0.0).y);
        vec2 _301 = _174 * 0.5;
        ivec2 _303 = ivec2(floor(_301));
        vec2 _304 = fract(_301);
        uvec2 _305 = uvec2(_303);
        uint _307 = _305.y;
        uint _309 = _307 + (_307 << 10u);
        uint _311 = _309 ^ (_309 >> 6u);
        uint _313 = _311 + (_311 << 3u);
        uint _315 = _313 ^ (_313 >> 11u);
        uint _321 = (((_305.x * 1664525u) + (_315 + (_315 << 15u))) + 1013904223u) * 1664525u;
        uint _323 = _321 ^ (_321 >> 11u);
        uint _326 = _323 ^ ((_323 << 7u) & 2636928640u);
        uint _329 = _326 ^ ((_326 << 15u) & 4022730752u);
        uvec2 _337 = uvec2(_303 + ivec2(1, 0));
        uint _339 = _337.y;
        uint _341 = _339 + (_339 << 10u);
        uint _343 = _341 ^ (_341 >> 6u);
        uint _345 = _343 + (_343 << 3u);
        uint _347 = _345 ^ (_345 >> 11u);
        uint _353 = (((_337.x * 1664525u) + (_347 + (_347 << 15u))) + 1013904223u) * 1664525u;
        uint _355 = _353 ^ (_353 >> 11u);
        uint _358 = _355 ^ ((_355 << 7u) & 2636928640u);
        uint _361 = _358 ^ ((_358 << 15u) & 4022730752u);
        uvec2 _369 = uvec2(_303 + ivec2(0, 1));
        uint _371 = _369.y;
        uint _373 = _371 + (_371 << 10u);
        uint _375 = _373 ^ (_373 >> 6u);
        uint _377 = _375 + (_375 << 3u);
        uint _379 = _377 ^ (_377 >> 11u);
        uint _385 = (((_369.x * 1664525u) + (_379 + (_379 << 15u))) + 1013904223u) * 1664525u;
        uint _387 = _385 ^ (_385 >> 11u);
        uint _390 = _387 ^ ((_387 << 7u) & 2636928640u);
        uint _393 = _390 ^ ((_390 << 15u) & 4022730752u);
        uvec2 _401 = uvec2(_303 + ivec2(1));
        uint _403 = _401.y;
        uint _405 = _403 + (_403 << 10u);
        uint _407 = _405 ^ (_405 >> 6u);
        uint _409 = _407 + (_407 << 3u);
        uint _411 = _409 ^ (_409 >> 11u);
        uint _417 = (((_401.x * 1664525u) + (_411 + (_411 << 15u))) + 1013904223u) * 1664525u;
        uint _419 = _417 ^ (_417 >> 11u);
        uint _422 = _419 ^ ((_419 << 7u) & 2636928640u);
        uint _425 = _422 ^ ((_422 << 15u) & 4022730752u);
        vec2 _435 = (_304 * _304) * (vec2(3.0) - (_304 * 2.0));
        float _436 = _435.x;
        float _437 = 1.0 - _436;
        float _439 = _435.y;
        if ((int((((((1.0 - _439) * (((uintBitsToFloat(((_329 ^ (_329 >> 18u)) & 8388607u) | 1065353216u) - 1.0) * _437) + ((uintBitsToFloat(((_361 ^ (_361 >> 18u)) & 8388607u) | 1065353216u) - 1.0) * _436))) + (((uintBitsToFloat(((_393 ^ (_393 >> 18u)) & 8388607u) | 1065353216u) - 1.0) * _437) * _439)) + (((uintBitsToFloat(((_425 ^ (_425 >> 18u)) & 8388607u) | 1065353216u) - 1.0) * _436) * _439)) * 2.0) > 0.60000002384185791015625) & int(_280 > 0.425000011920928955078125)) != 0)
        {
            vec3 _459 = _174.xxy;
            _459.y = 0.5;
            int _461 = _195.x;
            int _462 = _195.y;
            uint _464 = uint(_461 + _462);
            uint _466 = _464 + (_464 << 10u);
            uint _468 = _466 ^ (_466 >> 6u);
            uint _470 = _468 + (_468 << 3u);
            uint _472 = _470 ^ (_470 >> 11u);
            float _479 = mix(0.800000011920928955078125, 1.2000000476837158203125, uintBitsToFloat(((_472 + (_472 << 15u)) & 8388607u) | 1065353216u) - 1.0);
            vec3 _482 = vec3(_479) * mix(0.5, 1.0, _294 + (_298 * 1.5));
            vec3 _491 = vec3(spvNMin(_482.x, 0.75), (_482.y * 1.2000000476837158203125) * _479, spvNMin(_482.z, 0.75)) * 1.5;
            bool _492 = _298 > 0.0;
            vec3 _496;
            if (_492)
            {
                vec3 _495 = _459;
                _495.y = 1.10000002384185791015625;
                _496 = _495;
            }
            else
            {
                _496 = _459;
            }
            uint _515 = uint((_461 * _462) + _461);
            uint _517 = _515 + (_515 << 10u);
            uint _519 = _517 ^ (_517 >> 6u);
            uint _521 = _519 + (_519 << 3u);
            uint _523 = _521 ^ (_521 >> 11u);
            float _530 = 3.1400001049041748046875 * (uintBitsToFloat(((_523 + (_523 << 15u)) & 8388607u) | 1065353216u) - 1.0);
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
            mat4 _564 = transpose(mat4(vec4((_536 * _531.x) + _533, _540 - _542, _545 + _546, 0.0), vec4(_540 + _542, (_549 * _531.y) + _533, _552 - _553, 0.0), vec4(_545 - _546, _552 + _553, (_544 * _531.z) + _533, 0.0), vec4(0.0, 0.0, 0.0, 1.0))) * ((mat4(vec4(_491.x, 0.0, 0.0, 0.0), vec4(0.0, _491.y, 0.0, 0.0), vec4(0.0, 0.0, _491.z, 0.0), vec4(0.0, 0.0, 0.0, 1.0)) * mat4(vec4(1.0, 0.0, 0.0, 0.0), vec4(0.0, 1.0, 0.0, 0.0), vec4(0.0, 0.0, 1.0, 0.0), vec4(0.0, 0.0, 0.0, 1.0))) * mat4(vec4(1.0, 0.0, 0.0, _496.x), vec4(0.0, 1.0, 0.0, _496.y), vec4(0.0, 0.0, 1.0, _496.z), vec4(0.0, 0.0, 0.0, 1.0)));
            uint _595 = atomicAdd(_alias_draw_commands._m0[11u].instanceCount, 1u);
            _alias_flower_buffer._m0[uint(int(_595))] = FlowerXform(Affine3Packed(float[](_564[0].x, _564[1].x, _564[2].x, _564[0].y, _564[1].y, _564[2].y, _564[0].z, _564[1].z, _564[2].z, _564[0].w, _564[1].w + mix(-2.5, 24.0, textureLod(sampler2D(terrain_heightmap, sampler_default), ((vec4(0.0, 0.0, 0.0, 1.0) * transpose(mat4x3(vec3(_564[0].x, _564[1].x, _564[2].x), vec3(_564[0].y, _564[1].y, _564[2].y), vec3(_564[0].z, _564[1].z, _564[2].z), vec3(_564[0].w, _564[1].w, _564[2].w)))).xz * vec2(0.0076923076994717121124267578125)) + vec2(0.5), 0.0).x), _564[2].w)), mix(vec3(-1.0), vec3(1.0), bvec3(_492)), 0u);
        }
        _602 = 1013904223u;
        _603 = 1664525u;
        _604 = true;
        break;
    } while(false);
    do
    {
        vec2 _610;
        bool _662;
        do
        {
            _610 = (_194 * vec2(0.0076923076994717121124267578125)) + vec2(0.5);
            uvec2 _612 = uvec2(_610 * _204);
            vec4 _623 = textureGatherOffset(sampler2D(terrain_heightmap, sampler_default), _610, ivec2(0));
            float _624 = _623.z;
            float _625 = _623.y;
            float _627 = _623.w;
            float _628 = _623.x;
            bool _649;
            if (!(texelFetch(path_mask, ivec2(_612), int(0u)).x > 0.20000000298023223876953125))
            {
                _649 = (int(texelFetch(rocky_terrain, ivec2(_612), int(0u)).x > 0.0) & int(length(((vec2((_624 + _625) - (_627 + _628), (_628 + _625) - (_627 + _624)) * vec2(0.5)) * 26.5) / _228) > 0.89999997615814208984375)) != 0;
            }
            else
            {
                _649 = true;
            }
            bool _654;
            if (!_649)
            {
                _654 = texelFetch(water_mask, ivec2(_612), int(0u)).y > 0.300000011920928955078125;
            }
            else
            {
                _654 = true;
            }
            bool _659;
            if (!_654)
            {
                _659 = textureLod(sampler2D(foliage_exclusion_mask, sampler_llb), _610, 0.0).x > 0.0;
            }
            else
            {
                _659 = true;
            }
            if (_659)
            {
                _662 = true;
                break;
            }
            _662 = false;
            break;
        } while(false);
        if (_662)
        {
            break;
        }
        if (textureLod(sampler2D(buildings_mask_non_blurred, sampler_default), _610, 0.0).x > 0.5)
        {
            break;
        }
        vec4 _674 = textureLod(sampler2D(garden_mask, sampler_default), _610, 0.0);
        float _675 = _674.y;
        float _689;
        do
        {
            float _679 = abs(_675 - 0.650000035762786865234375);
            if (_679 > 0.300000011920928955078125)
            {
                _689 = 0.0;
                break;
            }
            float _683 = _679 * 3.3333332538604736328125;
            _689 = 1.0 - ((_683 * _683) * (3.0 - (_679 * 6.666666507720947265625)));
            break;
        } while(false);
        float _693 = smoothstep(0.89999997615814208984375, 1.0, textureLod(sampler2D(garden_mask, sampler_default), _610, 0.0).y);
        vec2 _697 = (_194 * 0.5) + vec2(100.0);
        ivec2 _699 = ivec2(floor(_697));
        vec2 _700 = fract(_697);
        uvec2 _701 = uvec2(_699);
        uint _703 = _701.y;
        uint _705 = _703 + (_703 << 10u);
        uint _707 = _705 ^ (_705 >> 6u);
        uint _709 = _707 + (_707 << 3u);
        uint _711 = _709 ^ (_709 >> 11u);
        bool _714 = _604 ? _604 : true;
        uint _715 = _604 ? _603 : 1664525u;
        uint _716 = _604 ? _602 : 1013904223u;
        uint _720 = (((_701.x * _715) + (_711 + (_711 << 15u))) + _716) * _715;
        uint _722 = _720 ^ (_720 >> 11u);
        uint _725 = _722 ^ ((_722 << 7u) & 2636928640u);
        uint _728 = _725 ^ ((_725 << 15u) & 4022730752u);
        uvec2 _736 = uvec2(_699 + ivec2(1, 0));
        uint _738 = _736.y;
        uint _740 = _738 + (_738 << 10u);
        uint _742 = _740 ^ (_740 >> 6u);
        uint _744 = _742 + (_742 << 3u);
        uint _746 = _744 ^ (_744 >> 11u);
        bool _749 = _714 ? _714 : true;
        uint _750 = _714 ? _715 : 1664525u;
        uint _751 = _714 ? _716 : 1013904223u;
        uint _755 = (((_736.x * _750) + (_746 + (_746 << 15u))) + _751) * _750;
        uint _757 = _755 ^ (_755 >> 11u);
        uint _760 = _757 ^ ((_757 << 7u) & 2636928640u);
        uint _763 = _760 ^ ((_760 << 15u) & 4022730752u);
        uvec2 _771 = uvec2(_699 + ivec2(0, 1));
        uint _773 = _771.y;
        uint _775 = _773 + (_773 << 10u);
        uint _777 = _775 ^ (_775 >> 6u);
        uint _779 = _777 + (_777 << 3u);
        uint _781 = _779 ^ (_779 >> 11u);
        bool _784 = _749 ? _749 : true;
        uint _785 = _749 ? _750 : 1664525u;
        uint _786 = _749 ? _751 : 1013904223u;
        uint _790 = (((_771.x * _785) + (_781 + (_781 << 15u))) + _786) * _785;
        uint _792 = _790 ^ (_790 >> 11u);
        uint _795 = _792 ^ ((_792 << 7u) & 2636928640u);
        uint _798 = _795 ^ ((_795 << 15u) & 4022730752u);
        uvec2 _806 = uvec2(_699 + ivec2(1));
        uint _808 = _806.y;
        uint _810 = _808 + (_808 << 10u);
        uint _812 = _810 ^ (_810 >> 6u);
        uint _814 = _812 + (_812 << 3u);
        uint _816 = _814 ^ (_814 >> 11u);
        uint _819 = _784 ? _785 : 1664525u;
        uint _824 = (((_806.x * _819) + (_816 + (_816 << 15u))) + (_784 ? _786 : 1013904223u)) * _819;
        uint _826 = _824 ^ (_824 >> 11u);
        uint _829 = _826 ^ ((_826 << 7u) & 2636928640u);
        uint _832 = _829 ^ ((_829 << 15u) & 4022730752u);
        vec2 _842 = (_700 * _700) * (vec2(3.0) - (_700 * 2.0));
        float _843 = _842.x;
        float _844 = 1.0 - _843;
        float _846 = _842.y;
        if ((int((((((1.0 - _846) * (((uintBitsToFloat(((_728 ^ (_728 >> 18u)) & 8388607u) | 1065353216u) - 1.0) * _844) + ((uintBitsToFloat(((_763 ^ (_763 >> 18u)) & 8388607u) | 1065353216u) - 1.0) * _843))) + (((uintBitsToFloat(((_798 ^ (_798 >> 18u)) & 8388607u) | 1065353216u) - 1.0) * _844) * _846)) + (((uintBitsToFloat(((_832 ^ (_832 >> 18u)) & 8388607u) | 1065353216u) - 1.0) * _843) * _846)) * 2.0) > 0.60000002384185791015625) & int(_675 > 0.425000011920928955078125)) != 0)
        {
            vec3 _866 = _194.xxy;
            _866.y = 0.5;
            int _868 = _195.x;
            int _869 = _195.y;
            uint _871 = uint(_868 + _869);
            uint _873 = _871 + (_871 << 10u);
            uint _875 = _873 ^ (_873 >> 6u);
            uint _877 = _875 + (_875 << 3u);
            uint _879 = _877 ^ (_877 >> 11u);
            float _886 = mix(0.800000011920928955078125, 1.2000000476837158203125, uintBitsToFloat(((_879 + (_879 << 15u)) & 8388607u) | 1065353216u) - 1.0);
            vec3 _889 = vec3(_886) * mix(0.5, 1.0, _689 + (_693 * 1.5));
            vec3 _898 = vec3(spvNMin(_889.x, 0.75), (_889.y * 1.2000000476837158203125) * _886, spvNMin(_889.z, 0.75)) * 1.5;
            vec3 _903;
            if (_693 > 0.0)
            {
                vec3 _902 = _866;
                _902.y = 1.10000002384185791015625;
                _903 = _902;
            }
            else
            {
                _903 = _866;
            }
            uint _922 = uint((_868 * _869) + _868);
            uint _924 = _922 + (_922 << 10u);
            uint _926 = _924 ^ (_924 >> 6u);
            uint _928 = _926 + (_926 << 3u);
            uint _930 = _928 ^ (_928 >> 11u);
            float _937 = 3.1400001049041748046875 * (uintBitsToFloat(((_930 + (_930 << 15u)) & 8388607u) | 1065353216u) - 1.0);
            vec3 _938 = normalize(vec3(0.0, 1.0, 0.0));
            float _939 = sin(_937);
            float _940 = cos(_937);
            float _941 = 1.0 - _940;
            float _943 = _941 * _938.x;
            float _947 = _943 * _938.y;
            float _949 = _938.z * _939;
            float _951 = _941 * _938.z;
            float _952 = _951 * _938.x;
            float _953 = _938.y * _939;
            float _956 = _941 * _938.y;
            float _959 = _956 * _938.z;
            float _960 = _938.x * _939;
            mat4 _971 = transpose(mat4(vec4((_943 * _938.x) + _940, _947 - _949, _952 + _953, 0.0), vec4(_947 + _949, (_956 * _938.y) + _940, _959 - _960, 0.0), vec4(_952 - _953, _959 + _960, (_951 * _938.z) + _940, 0.0), vec4(0.0, 0.0, 0.0, 1.0))) * ((mat4(vec4(_898.x, 0.0, 0.0, 0.0), vec4(0.0, _898.y, 0.0, 0.0), vec4(0.0, 0.0, _898.z, 0.0), vec4(0.0, 0.0, 0.0, 1.0)) * mat4(vec4(1.0, 0.0, 0.0, 0.0), vec4(0.0, 1.0, 0.0, 0.0), vec4(0.0, 0.0, 1.0, 0.0), vec4(0.0, 0.0, 0.0, 1.0))) * mat4(vec4(1.0, 0.0, 0.0, _903.x), vec4(0.0, 1.0, 0.0, _903.y), vec4(0.0, 0.0, 1.0, _903.z), vec4(0.0, 0.0, 0.0, 1.0)));
            uint _1000 = atomicAdd(_alias_draw_commands._m0[11u].instanceCount, 1u);
            _alias_flower_buffer._m0[uint(int(_1000))] = FlowerXform(Affine3Packed(float[](_971[0].x, _971[1].x, _971[2].x, _971[0].y, _971[1].y, _971[2].y, _971[0].z, _971[1].z, _971[2].z, _971[0].w, _971[1].w + mix(-2.5, 24.0, textureLod(sampler2D(terrain_heightmap, sampler_default), ((vec4(0.0, 0.0, 0.0, 1.0) * transpose(mat4x3(vec3(_971[0].x, _971[1].x, _971[2].x), vec3(_971[0].y, _971[1].y, _971[2].y), vec3(_971[0].z, _971[1].z, _971[2].z), vec3(_971[0].w, _971[1].w, _971[2].w)))).xz * vec2(0.0076923076994717121124267578125)) + vec2(0.5), 0.0).x), _971[2].w)), vec3(-1.0), 1u);
        }
        break;
    } while(false);
}

