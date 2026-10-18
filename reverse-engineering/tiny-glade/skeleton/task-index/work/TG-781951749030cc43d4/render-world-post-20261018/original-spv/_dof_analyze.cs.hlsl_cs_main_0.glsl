#version 450
#extension GL_EXT_nonuniform_qualifier : require
#extension GL_EXT_spirv_intrinsics : require
#extension GL_EXT_samplerless_texture_functions : require
#extension GL_KHR_shader_subgroup_quad : require
#extension GL_KHR_shader_subgroup_arithmetic : require
#extension GL_KHR_shader_subgroup_basic : require
#extension GL_KHR_shader_subgroup_ballot : require
layout(local_size_x = 8, local_size_y = 8, local_size_z = 1) in;

struct DofTileOffsets
{
    uint offsets[8];
};

layout(set = 0, binding = 13, std140) uniform type_Globals
{
    uvec2 coc_tiles_tex_dims;
    uvec2 coc_tiles_dilated_tex_dims;
} _Globals;

layout(set = 0, binding = 16, std430) readonly buffer type_StructuredBuffer_DofTileOffsets
{
    DofTileOffsets _m0[];
} tile_offsets_buffer;

layout(set = 0, binding = 17, std430) buffer type_RWStructuredBuffer_v4uint
{
    uvec4 _m0[];
} _alias_dispatch_args_buffer;

layout(set = 0, binding = 18, std430) buffer type_RWStructuredBuffer_v2uint
{
    uvec2 _m0[];
} _alias_tile_coords_buffer;

layout(set = 0, binding = 14) uniform texture2D coc_tiles_tex;
layout(set = 0, binding = 15, r16f) uniform writeonly image2D coc_tiles_dilated_tex;

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
    uint _106 = gl_LocalInvocationIndex & 1431655765u;
    uint _109 = (_106 ^ (_106 >> 1u)) & 858993459u;
    uint _112 = (_109 ^ (_109 >> 2u)) & 252645135u;
    uint _115 = (_112 ^ (_112 >> 4u)) & 16711935u;
    uint _120 = (gl_LocalInvocationIndex >> 1u) & 1431655765u;
    uint _123 = (_120 ^ (_120 >> 1u)) & 858993459u;
    uint _126 = (_123 ^ (_123 >> 2u)) & 252645135u;
    uint _129 = (_126 ^ (_126 >> 4u)) & 16711935u;
    uvec2 _134 = (gl_GlobalInvocationID.xy & uvec2(4294967288u)) + uvec2((_115 ^ (_115 >> 8u)) & 65535u, (_129 ^ (_129 >> 8u)) & 65535u);
    bool _138 = all(lessThan(_134, _Globals.coc_tiles_tex_dims));
    int _147 = int(uint(spvNMin(16.0, ceil(float(_Globals.coc_tiles_tex_dims.y * 8u) * 0.0052083334885537624359130859375))));
    int _148 = -_147;
    float _150;
    float _153;
    _150 = 10000000000.0;
    _153 = 0.0;
    float _151;
    float _154;
    for (int _155 = _148; _155 <= _147; _150 = _151, _153 = _154, _155++)
    {
        _154 = _153;
        _151 = _150;
        float _161;
        float _163;
        for (int _164 = _148; _164 <= _147; _154 = _161, _151 = _163, _164++)
        {
            uvec2 _171 = _134 + uvec2(ivec2(_155, _164));
            int _176 = (_155 * _155) + (_164 * _164);
            if (all(lessThan(_171, _Globals.coc_tiles_tex_dims)) ? (_176 < (_147 * _147)) : false)
            {
                vec4 _183 = texelFetch(coc_tiles_tex, ivec2(_171), int(0u));
                float _184 = _183.y;
                float _195;
                float _196;
                if ((_184 + 8.0) > (sqrt(float(_176)) * 8.0))
                {
                    _195 = spvNMax(_154, _184);
                    _196 = spvNMin(_151, _183.x);
                }
                else
                {
                    _195 = _154;
                    _196 = _151;
                }
                _161 = _195;
                _163 = _196;
            }
            else
            {
                _161 = _154;
                _163 = _151;
            }
        }
    }
    imageStore(coc_tiles_dilated_tex, ivec2(_134), vec4(_153));
    uint _199 = uint(_153 <= 7.0);
    uint _200 = subgroupQuadSwapHorizontal(_199);
    uint _201 = _199 & _200;
    uint _202 = subgroupQuadSwapVertical(_201);
    uint _203 = _201 & _202;
    uint _205 = uint(_150 > 11.0);
    uint _206 = subgroupQuadSwapHorizontal(_205);
    uint _207 = _205 & _206;
    uint _208 = subgroupQuadSwapVertical(_207);
    uint _209 = _207 & _208;
    uint _101[7];
    _101[0u] = 0u;
    _101[1u] = 0u;
    _101[2u] = 0u;
    _101[3u] = 0u;
    _101[4u] = 0u;
    _101[5u] = 0u;
    _101[6u] = 0u;
    if (_138)
    {
        if (_203 != 0u)
        {
            if (all(equal((_134 & uvec2(1u)), uvec2(0u))))
            {
                _101[6]++;
            }
            int _232 = (_153 <= 5.0) ? 2 : 1;
            _101[_232]++;
        }
        else
        {
            if (all(equal((_134 & uvec2(1u)), uvec2(0u))))
            {
                if (_209 != 0u)
                {
                    _101[5]++;
                    _101[3] += 4u;
                }
                else
                {
                    _101[4]++;
                    _101[0] += 4u;
                }
            }
        }
    }
    uint _258 = subgroupAdd(_101[0u]);
    uint _100[7];
    if (subgroupElect())
    {
        uint _263 = atomicAdd(_alias_dispatch_args_buffer._m0[0u].x, _258);
        _100[0u] = tile_offsets_buffer._m0[0u].offsets[0u] + _263;
    }
    uint _269 = subgroupAdd(_101[1u]);
    if (subgroupElect())
    {
        uint _274 = atomicAdd(_alias_dispatch_args_buffer._m0[1u].x, _269);
        _100[1u] = tile_offsets_buffer._m0[0u].offsets[1u] + _274;
    }
    uint _280 = subgroupAdd(_101[2u]);
    if (subgroupElect())
    {
        uint _285 = atomicAdd(_alias_dispatch_args_buffer._m0[2u].x, _280);
        _100[2u] = tile_offsets_buffer._m0[0u].offsets[2u] + _285;
    }
    uint _291 = subgroupAdd(_101[3u]);
    if (subgroupElect())
    {
        uint _296 = atomicAdd(_alias_dispatch_args_buffer._m0[3u].x, _291);
        _100[3u] = tile_offsets_buffer._m0[0u].offsets[3u] + _296;
    }
    uint _302 = subgroupAdd(_101[4u]);
    if (subgroupElect())
    {
        uint _307 = atomicAdd(_alias_dispatch_args_buffer._m0[4u].x, _302);
        _100[4u] = tile_offsets_buffer._m0[0u].offsets[4u] + _307;
    }
    uint _313 = subgroupAdd(_101[5u]);
    if (subgroupElect())
    {
        uint _318 = atomicAdd(_alias_dispatch_args_buffer._m0[5u].x, _313);
        _100[5u] = tile_offsets_buffer._m0[0u].offsets[5u] + _318;
    }
    uint _324 = subgroupAdd(_101[6u]);
    if (subgroupElect())
    {
        uint _329 = atomicAdd(_alias_dispatch_args_buffer._m0[6u].x, _324);
        _100[6u] = tile_offsets_buffer._m0[0u].offsets[6u] + _329;
    }
    _100[0u] = subgroupBroadcastFirst(_100[0u]);
    _100[1u] = subgroupBroadcastFirst(_100[1u]);
    _100[2u] = subgroupBroadcastFirst(_100[2u]);
    _100[3u] = subgroupBroadcastFirst(_100[3u]);
    _100[4u] = subgroupBroadcastFirst(_100[4u]);
    _100[5u] = subgroupBroadcastFirst(_100[5u]);
    _100[6u] = subgroupBroadcastFirst(_100[6u]);
    _100[0u] += subgroupExclusiveAdd(_101[0u]);
    _100[1u] += subgroupExclusiveAdd(_101[1u]);
    _100[2u] += subgroupExclusiveAdd(_101[2u]);
    _100[3u] += subgroupExclusiveAdd(_101[3u]);
    _100[4u] += subgroupExclusiveAdd(_101[4u]);
    _100[5u] += subgroupExclusiveAdd(_101[5u]);
    _100[6u] += subgroupExclusiveAdd(_101[6u]);
    if (_138)
    {
        if (_203 != 0u)
        {
            if (all(equal((_134 & uvec2(1u)), uvec2(0u))))
            {
                _alias_tile_coords_buffer._m0[_100[6]] = _134 / uvec2(2u);
            }
            _alias_tile_coords_buffer._m0[_100[(_153 <= 5.0) ? 2 : 1]] = _134;
        }
        else
        {
            if (all(equal((_134 & uvec2(1u)), uvec2(0u))))
            {
                if (_209 != 0u)
                {
                    _alias_tile_coords_buffer._m0[_100[5]] = _134 / uvec2(2u);
                    _alias_tile_coords_buffer._m0[_100[3]] = _134;
                    _alias_tile_coords_buffer._m0[_100[3] + 1u] = _134 + uvec2(1u, 0u);
                    _alias_tile_coords_buffer._m0[_100[3] + 2u] = _134 + uvec2(0u, 1u);
                    _alias_tile_coords_buffer._m0[_100[3] + 3u] = _134 + uvec2(1u);
                }
                else
                {
                    _alias_tile_coords_buffer._m0[_100[4]] = _134 / uvec2(2u);
                    _alias_tile_coords_buffer._m0[_100[0]] = _134;
                    _alias_tile_coords_buffer._m0[_100[0] + 1u] = _134 + uvec2(1u, 0u);
                    _alias_tile_coords_buffer._m0[_100[0] + 2u] = _134 + uvec2(0u, 1u);
                    _alias_tile_coords_buffer._m0[_100[0] + 3u] = _134 + uvec2(1u);
                }
            }
        }
    }
}

