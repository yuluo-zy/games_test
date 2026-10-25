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

layout(set = 0, binding = 19, std430) buffer type_RWStructuredBuffer_BushXform
{
    BushXform _m0[];
} _alias_bush_buffer;

layout(set = 0, binding = 20, std430) buffer type_RWStructuredBuffer_FlowerXform
{
    FlowerXform _m0[];
} _alias_bush_flowers;

layout(set = 0, binding = 4) uniform sampler sampler_llb;
layout(set = 0, binding = 9) uniform sampler sampler_default;
layout(set = 0, binding = 15) uniform texture2D terrain_heightmap;
layout(set = 0, binding = 17) uniform texture2D garden_mask;
layout(set = 0, binding = 18) uniform texture2D buildings_mask_non_blurred;
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
    do
    {
        vec2 _161;
        uint _140 = (gl_GlobalInvocationID.x * gl_GlobalInvocationID.y) + gl_GlobalInvocationID.x;
        uint _142 = _140 + (_140 << 10u);
        uint _144 = _142 ^ (_142 >> 6u);
        uint _146 = _144 + (_144 << 3u);
        uint _148 = _146 ^ (_146 >> 11u);
        float _154 = uintBitsToFloat(((_148 + (_148 << 15u)) & 8388607u) | 1065353216u) - 1.0;
        vec2 _157 = ((((vec2(gl_GlobalInvocationID.xy) + vec2(0.5)) / vec2(_Globals.grid_dims)) - vec2(0.5)) * vec2(130.0)) + vec2(mix(-1.0, 1.0, _154));
        bool _227;
        do
        {
            _161 = (_157 * vec2(0.0076923076994717121124267578125)) + vec2(0.5);
            uvec2 _166 = uvec2(_161 * vec2(_Globals.path_mask_dims));
            vec4 _184 = textureGatherOffset(sampler2D(terrain_heightmap, sampler_default), _161, ivec2(0));
            float _189 = _184.z;
            float _190 = _184.y;
            float _192 = _184.w;
            float _193 = _184.x;
            bool _214;
            if (!(texelFetch(path_mask, ivec2(_166), int(0u)).x > 0.20000000298023223876953125))
            {
                _214 = (int(texelFetch(rocky_terrain, ivec2(_166), int(0u)).x > 0.0) & int(length(((vec2((_189 + _190) - (_192 + _193), (_193 + _190) - (_192 + _189)) * vec2(0.5)) * 26.5) / (vec2(130.0) / vec2(_Globals.terrain_heightmap_dims))) > 0.89999997615814208984375)) != 0;
            }
            else
            {
                _214 = true;
            }
            bool _219;
            if (!_214)
            {
                _219 = texelFetch(water_mask, ivec2(_166), int(0u)).y > 0.300000011920928955078125;
            }
            else
            {
                _219 = true;
            }
            bool _224;
            if (!_219)
            {
                _224 = textureLod(sampler2D(foliage_exclusion_mask, sampler_llb), _161, 0.0).x > 0.0;
            }
            else
            {
                _224 = true;
            }
            if (_224)
            {
                _227 = true;
                break;
            }
            _227 = false;
            break;
        } while(false);
        if (_227)
        {
            break;
        }
        if (textureLod(sampler2D(buildings_mask_non_blurred, sampler_default), _161, 0.0).x > 0.5)
        {
            break;
        }
        float _241 = textureLod(sampler2D(garden_mask, sampler_default), _161, 0.0).y + 0.004999999888241291046142578125;
        if (_241 > 0.5)
        {
            uint _257 = gl_GlobalInvocationID.x + gl_GlobalInvocationID.y;
            uint _259 = _257 + (_257 << 10u);
            uint _261 = _259 ^ (_259 >> 6u);
            uint _263 = _261 + (_261 << 3u);
            uint _265 = _263 ^ (_263 >> 11u);
            float _272 = mix(0.800000011920928955078125, 1.2000000476837158203125, uintBitsToFloat(((_265 + (_265 << 15u)) & 8388607u) | 1065353216u) - 1.0);
            vec3 _276 = vec3(_272) * mix(0.300000011920928955078125, 0.800000011920928955078125, sqrt(clamp(smoothstep(0.449999988079071044921875, 1.0, _241) + (smoothstep(0.60000002384185791015625, 1.0, _241) * 0.20000000298023223876953125), 0.0, 1.0)));
            float _296 = 3.1400001049041748046875 * _154;
            vec3 _297 = normalize(vec3(0.0, 1.0, 0.0));
            float _298 = sin(_296);
            float _299 = cos(_296);
            float _300 = 1.0 - _299;
            float _302 = _300 * _297.x;
            float _306 = _302 * _297.y;
            float _308 = _297.z * _298;
            float _310 = _300 * _297.z;
            float _311 = _310 * _297.x;
            float _312 = _297.y * _298;
            float _315 = _300 * _297.y;
            float _318 = _315 * _297.z;
            float _319 = _297.x * _298;
            mat4 _330 = transpose(mat4(vec4((_302 * _297.x) + _299, _306 - _308, _311 + _312, 0.0), vec4(_306 + _308, (_315 * _297.y) + _299, _318 - _319, 0.0), vec4(_311 - _312, _318 + _319, (_310 * _297.z) + _299, 0.0), vec4(0.0, 0.0, 0.0, 1.0))) * ((mat4(vec4(spvNMin(_276.x, 0.75), 0.0, 0.0, 0.0), vec4(0.0, spvNMin(_276.y * _272, 1.2000000476837158203125), 0.0, 0.0), vec4(0.0, 0.0, spvNMin(_276.z, 0.75), 0.0), vec4(0.0, 0.0, 0.0, 1.0)) * mat4(vec4(1.0, 0.0, 0.0, 0.0), vec4(0.0, 1.0, 0.0, 0.0), vec4(0.0, 0.0, 1.0, 0.0), vec4(0.0, 0.0, 0.0, 1.0))) * mat4(vec4(1.0, 0.0, 0.0, _157.x), vec4(0.0, 1.0, 0.0, mix(-2.5, 24.0, textureLod(sampler2D(terrain_heightmap, sampler_default), (_157.xy * vec2(0.0076923076994717121124267578125)) + vec2(0.5), 0.0).x)), vec4(0.0, 0.0, 1.0, _157.y), vec4(0.0, 0.0, 0.0, 1.0)));
            uint _344 = atomicAdd(_alias_draw_commands._m0[10u].instanceCount, 1u);
            Affine3Packed _349 = Affine3Packed(float[](_330[0].x, _330[1].x, _330[2].x, _330[0].y, _330[1].y, _330[2].y, _330[0].z, _330[1].z, _330[2].z, _330[0].w, _330[1].w, _330[2].w));
            _alias_bush_buffer._m0[uint(int(_344))] = BushXform(_349);
            uint _354 = ((gl_GlobalInvocationID.x * 2u) + (gl_GlobalInvocationID.y / 5u)) + 1233u;
            uint _356 = _354 + (_354 << 10u);
            uint _358 = _356 ^ (_356 >> 6u);
            uint _360 = _358 + (_358 << 3u);
            uint _362 = _360 ^ (_360 >> 11u);
            bool _373;
            if ((uintBitsToFloat(((_362 + (_362 << 15u)) & 8388607u) | 1065353216u) - 1.0) > 0.300000011920928955078125)
            {
                _373 = _241 > 0.800000011920928955078125;
            }
            else
            {
                _373 = false;
            }
            if (_373)
            {
                uint _377 = atomicAdd(_alias_draw_commands._m0[11u].instanceCount, 1u);
                _alias_bush_flowers._m0[uint(int(_377))] = FlowerXform(_349, vec3(-1.0), 2u);
            }
        }
        break;
    } while(false);
}

