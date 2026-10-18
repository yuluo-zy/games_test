// DERIVED by SPIRV-Cross from original packaged SPIR-V; NOT original HLSL/OpSource.
#version 460
#extension GL_EXT_scalar_block_layout : require
#extension GL_EXT_samplerless_texture_functions : require
layout(local_size_x = 8, local_size_y = 8, local_size_z = 1) in;

struct DrawIndexedIndirectCommand
{
    uint count;
    uint instanceCount;
    uint firstIndex;
    uint vertexOffset;
    uint firstInstance;
};

struct Flower03Xform
{
    vec4 position;
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
    layout(offset = 32) ivec2 grid_dims;
    layout(offset = 40) uvec2 path_mask_dims;
    layout(offset = 48) uvec2 rocky_terrain_dims;
    layout(offset = 56) uvec2 water_mask_dims;
    layout(offset = 64) uvec2 foliage_exclusion_mask_dims;
} _Globals;

layout(set = 0, binding = 18, std430) buffer type_RWStructuredBuffer_Flower03Xform
{
    Flower03Xform _m0[];
} _alias_flower03_buffer;

layout(set = 0, binding = 4) uniform sampler sampler_llb;
layout(set = 0, binding = 9) uniform sampler sampler_default;
layout(set = 0, binding = 15) uniform texture2D terrain_heightmap;
layout(set = 0, binding = 17) uniform texture2D garden_mask;
layout(set = 0, binding = 19) uniform texture2D path_mask;
layout(set = 0, binding = 20) uniform texture2D rocky_terrain;
layout(set = 0, binding = 21) uniform texture2D water_mask;
layout(set = 0, binding = 22) uniform texture2D foliage_exclusion_mask;

void main()
{
    do
    {
        vec2 _136;
        uint _115 = ((gl_GlobalInvocationID.x * gl_GlobalInvocationID.y) + gl_GlobalInvocationID.x) + 100u;
        uint _117 = _115 + (_115 << 10u);
        uint _119 = _117 ^ (_117 >> 6u);
        uint _121 = _119 + (_119 << 3u);
        uint _123 = _121 ^ (_121 >> 11u);
        vec2 _132 = ((((vec2(gl_GlobalInvocationID.xy) + vec2(0.5)) / vec2(_Globals.grid_dims)) - vec2(0.5)) * vec2(130.0)) + vec2(mix(-0.75, 0.75, uintBitsToFloat(((_123 + (_123 << 15u)) & 8388607u) | 1065353216u) - 1.0));
        bool _202;
        do
        {
            _136 = (_132 * vec2(0.0076923076994717121124267578125)) + vec2(0.5);
            uvec2 _141 = uvec2(_136 * vec2(_Globals.path_mask_dims));
            vec4 _159 = textureGatherOffset(sampler2D(terrain_heightmap, sampler_default), _136, ivec2(0));
            float _164 = _159.z;
            float _165 = _159.y;
            float _167 = _159.w;
            float _168 = _159.x;
            bool _189;
            if (!(texelFetch(path_mask, ivec2(_141), int(0u)).x > 0.20000000298023223876953125))
            {
                _189 = (int(texelFetch(rocky_terrain, ivec2(_141), int(0u)).x > 0.0) & int(length(((vec2((_164 + _165) - (_167 + _168), (_168 + _165) - (_167 + _164)) * vec2(0.5)) * 26.5) / (vec2(130.0) / vec2(_Globals.terrain_heightmap_dims))) > 0.89999997615814208984375)) != 0;
            }
            else
            {
                _189 = true;
            }
            bool _194;
            if (!_189)
            {
                _194 = texelFetch(water_mask, ivec2(_141), int(0u)).y > 0.300000011920928955078125;
            }
            else
            {
                _194 = true;
            }
            bool _199;
            if (!_194)
            {
                _199 = textureLod(sampler2D(foliage_exclusion_mask, sampler_llb), _136, 0.0).x > 0.0;
            }
            else
            {
                _199 = true;
            }
            if (_199)
            {
                _202 = true;
                break;
            }
            _202 = false;
            break;
        } while(false);
        if (_202)
        {
            break;
        }
        float _209 = textureLod(sampler2D(garden_mask, sampler_default), _136, 0.0).y - 0.0500000007450580596923828125;
        bool _214;
        if (_209 > 0.3499999940395355224609375)
        {
            _214 = _209 < 0.800000011920928955078125;
        }
        else
        {
            _214 = false;
        }
        if (_214)
        {
            uint _217 = gl_GlobalInvocationID.x + gl_GlobalInvocationID.y;
            uint _219 = _217 + (_217 << 10u);
            uint _221 = _219 ^ (_219 >> 6u);
            uint _223 = _221 + (_221 << 3u);
            uint _225 = _223 ^ (_223 >> 11u);
            uint _237 = atomicAdd(_alias_draw_commands._m0[16u].instanceCount, 1u);
            _alias_flower03_buffer._m0[uint(int(_237))] = Flower03Xform(vec4(_132.x, 0.0, _132.y, mix(0.60000002384185791015625, 1.0, uintBitsToFloat(((_225 + (_225 << 15u)) & 8388607u) | 1065353216u) - 1.0)));
        }
        break;
    } while(false);
}

