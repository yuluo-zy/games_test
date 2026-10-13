// PATCHED: modules ze.cs.hlsl_cs_main_0.bin:cs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)))
// ==RECONSTRUCTED== shaders/dof/analyze.cs.hlsl | A (dof-aa review, 2026-10-13): A winner (accumulator inits max=0/min=1e10, max-over-y written to dilated tex, near<=7 / far>11; B's inits inverted = behaviorally broken).
//   Dilates the per-tile min/max CoC over a circular ring, classifies 8x8 tile quads into
//   near/far/both buckets, and appends tile coords + dispatch counts with wave-atomic prefix sums.

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/math.hlsl"
#include "inc/hash.hlsl"
#include "inc/morton.hlsl"
#include "inc/coc.hlsl"
#include "dof/tiles.inc.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint2 coc_tiles_tex_dims;
    uint2 coc_tiles_dilated_tex_dims;
};

StructuredBuffer<DofTileOffsets> tile_offsets_buffer : register(t16, space0);
RWStructuredBuffer<uint4> __alias_dispatch_args_buffer : register(u17, space0);
RWStructuredBuffer<uint2> __alias_tile_coords_buffer : register(u18, space0);
Texture2D<float4> coc_tiles_tex : register(t14, space0);
RWTexture2D<half> coc_tiles_dilated_tex : register(u15, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 dispatch_id : SV_DispatchThreadID, uint local_index : SV_GroupIndex)
{
    uint2 tile = (dispatch_id.xy & uint2(0xFFFFFFF8u, 0xFFFFFFF8u)) + morton_decode(local_index);
    bool in_bounds = all(tile < coc_tiles_tex_dims);

    int radius = int(min(16.0f, ceil(float(coc_tiles_tex_dims.y * 8u) * 0.0052083334885537624f)));
    float min_coc = 1.0e10f;
    float max_coc = 0.0f;
    for (int dy = -radius; dy <= radius; ++dy)
    {
        for (int dx = -radius; dx <= radius; ++dx)
        {
            int dist_sq = (dy * dy) + (dx * dx);
            uint2 sample_tile = tile + uint2(dx, dy);
            if (in_bounds ? (dist_sq < (radius * radius)) : false)
            {
                float4 c = coc_tiles_tex.Load(int3(sample_tile, 0));
                float coc = c.y;
                if ((coc + 8.0f) > (sqrt(float(dist_sq)) * 8.0f))
                {
                    max_coc = max(max_coc, coc);
                    min_coc = min(min_coc, c.x);
                }
            }
        }
    }
    coc_tiles_dilated_tex[tile] = max_coc;

    uint near_quad = uint(max_coc <= 7.0f);
    near_quad = QuadReadAcrossX(near_quad) & near_quad;
    near_quad = QuadReadAcrossY(near_quad) & near_quad;
    uint far_quad = uint(min_coc > 11.0f);
    far_quad = QuadReadAcrossX(far_quad) & far_quad;
    far_quad = QuadReadAcrossY(far_quad) & far_quad;

    uint counts[7];
    [unroll]
    for (uint b = 0u; b < 7u; ++b)
    {
        counts[b] = 0u;
    }

    if (in_bounds)
    {
        if (near_quad != 0u)
        {
            if (all((tile & uint2(1u, 1u)) == uint2(0u, 0u)))
            {
                counts[6]++;
            }
            counts[(max_coc <= 5.0f) ? 2u : 1u]++;
        }
        else
        {
            if (all((tile & uint2(1u, 1u)) == uint2(0u, 0u)))
            {
                if (far_quad != 0u)
                {
                    counts[5]++;
                    counts[3] += 4u;
                }
                else
                {
                    counts[4]++;
                    counts[0] += 4u;
                }
            }
        }
    }

    uint sums[7];
    [unroll]
    for (uint g = 0u; g < 7u; ++g)
    {
        uint wave_count = WaveActiveSum(counts[g]);
        if (WaveIsFirstLane())
        {
            uint prev;
            InterlockedAdd(__alias_dispatch_args_buffer[g].x, wave_count, prev);
            sums[g] = tile_offsets_buffer[0].offsets[g] + prev;
        }
        sums[g] = WaveReadLaneFirst(sums[g]);
        sums[g] += WavePrefixSum(counts[g]);
    }

    if (in_bounds)
    {
        if (near_quad != 0u)
        {
            if (all((tile & uint2(1u, 1u)) == uint2(0u, 0u)))
            {
                __alias_tile_coords_buffer[sums[6]] = tile / uint2(2u, 2u);
            }
            __alias_tile_coords_buffer[sums[(max_coc <= 5.0f) ? 2u : 1u]] = tile;
        }
        else
        {
            if (all((tile & uint2(1u, 1u)) == uint2(0u, 0u)))
            {
                if (far_quad != 0u)
                {
                    __alias_tile_coords_buffer[sums[5]] = tile / uint2(2u, 2u);
                    __alias_tile_coords_buffer[sums[3]] = tile;
                    __alias_tile_coords_buffer[sums[3] + 1u] = tile + uint2(1u, 0u);
                    __alias_tile_coords_buffer[sums[3] + 2u] = tile + uint2(0u, 1u);
                    __alias_tile_coords_buffer[sums[3] + 3u] = tile + uint2(1u, 1u);
                }
                else
                {
                    __alias_tile_coords_buffer[sums[4]] = tile / uint2(2u, 2u);
                    __alias_tile_coords_buffer[sums[0]] = tile;
                    __alias_tile_coords_buffer[sums[0] + 1u] = tile + uint2(1u, 0u);
                    __alias_tile_coords_buffer[sums[0] + 2u] = tile + uint2(0u, 1u);
                    __alias_tile_coords_buffer[sums[0] + 3u] = tile + uint2(1u, 1u);
                }
            }
        }
    }
}

#endif

#if defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)
// lifted from _dof_analyze.cs.hlsl_cs_main_0.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
struct DofTileOffsets
{
    uint offsets[8];
};

cbuffer $Globals : register(b13, space0)
{
    uint2 _Globals_coc_tiles_tex_dims : packoffset(c0);
    uint2 _Globals_coc_tiles_dilated_tex_dims : packoffset(c0.z);
};

ByteAddressBuffer tile_offsets_buffer : register(t16, space0);
RWByteAddressBuffer __alias_dispatch_args_buffer : register(u17, space0);
RWByteAddressBuffer __alias_tile_coords_buffer : register(u18, space0);
Texture2D<float4> coc_tiles_tex : register(t14, space0);
RWTexture2D<float> coc_tiles_dilated_tex : register(u15, space0);

static uint3 gl_GlobalInvocationID;
static uint gl_LocalInvocationIndex;
struct SPIRV_Cross_Input
{
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
    uint gl_LocalInvocationIndex : SV_GroupIndex;
};

void comp_main()
{
    uint _106 = gl_LocalInvocationIndex & 1431655765u;
    uint _109 = (_106 ^ (_106 >> 1u)) & 858993459u;
    uint _112 = (_109 ^ (_109 >> 2u)) & 252645135u;
    uint _115 = (_112 ^ (_112 >> 4u)) & 16711935u;
    uint _120 = (gl_LocalInvocationIndex >> 1u) & 1431655765u;
    uint _123 = (_120 ^ (_120 >> 1u)) & 858993459u;
    uint _126 = (_123 ^ (_123 >> 2u)) & 252645135u;
    uint _129 = (_126 ^ (_126 >> 4u)) & 16711935u;
    uint2 _134 = (gl_GlobalInvocationID.xy & uint2(4294967288u, 4294967288u)) + uint2((_115 ^ (_115 >> 8u)) & 65535u, (_129 ^ (_129 >> 8u)) & 65535u);
    bool _138 = all(bool2(_134.x < _Globals_coc_tiles_tex_dims.x, _134.y < _Globals_coc_tiles_tex_dims.y));
    int _147 = int(uint(min(16.0f, ceil(float(_Globals_coc_tiles_tex_dims.y * 8u) * 0.0052083334885537624359130859375f))));
    int _148 = -_147;
    float _150;
    float _153;
    _150 = 10000000000.0f;
    _153 = 0.0f;
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
            uint2 _171 = _134 + uint2(int2(_155, _164));
            int _176 = (_155 * _155) + (_164 * _164);
            if (all(bool2(_171.x < _Globals_coc_tiles_tex_dims.x, _171.y < _Globals_coc_tiles_tex_dims.y)) ? (_176 < (_147 * _147)) : false)
            {
                float4 _183 = coc_tiles_tex.Load(int3(_171, 0u));
                float _184 = _183.y;
                float _195;
                float _196;
                if ((_184 + 8.0f) > (sqrt(float(_176)) * 8.0f))
                {
                    _195 = max(_154, _184);
                    _196 = min(_151, _183.x);
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
    coc_tiles_dilated_tex[_134] = _153;
    uint _199 = uint(_153 <= 7.0f);
    uint _200 = QuadReadAcrossX(_199);
    uint _201 = _199 & _200;
    uint _202 = QuadReadAcrossY(_201);
    uint _203 = _201 & _202;
    uint _205 = uint(_150 > 11.0f);
    uint _206 = QuadReadAcrossX(_205);
    uint _207 = _205 & _206;
    uint _208 = QuadReadAcrossY(_207);
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
            uint2 _223 = _134 & uint2(1u, 1u);
            if (all(bool2(_223.x == uint2(0u, 0u).x, _223.y == uint2(0u, 0u).y)))
            {
                _101[6]++;
            }
            int _232 = (_153 <= 5.0f) ? 2 : 1;
            _101[_232]++;
        }
        else
        {
            uint2 _236 = _134 & uint2(1u, 1u);
            if (all(bool2(_236.x == uint2(0u, 0u).x, _236.y == uint2(0u, 0u).y)))
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
    uint _258 = WaveActiveSum(_101[0u]);
    uint _100[7];
    if (WaveIsFirstLane())
    {
        uint _263;
        __alias_dispatch_args_buffer.InterlockedAdd(0, _258, _263);
        _100[0u] = tile_offsets_buffer.Load<uint>(0) + _263;
    }
    uint _269 = WaveActiveSum(_101[1u]);
    if (WaveIsFirstLane())
    {
        uint _274;
        __alias_dispatch_args_buffer.InterlockedAdd(16, _269, _274);
        _100[1u] = tile_offsets_buffer.Load<uint>(4) + _274;
    }
    uint _280 = WaveActiveSum(_101[2u]);
    if (WaveIsFirstLane())
    {
        uint _285;
        __alias_dispatch_args_buffer.InterlockedAdd(32, _280, _285);
        _100[2u] = tile_offsets_buffer.Load<uint>(8) + _285;
    }
    uint _291 = WaveActiveSum(_101[3u]);
    if (WaveIsFirstLane())
    {
        uint _296;
        __alias_dispatch_args_buffer.InterlockedAdd(48, _291, _296);
        _100[3u] = tile_offsets_buffer.Load<uint>(12) + _296;
    }
    uint _302 = WaveActiveSum(_101[4u]);
    if (WaveIsFirstLane())
    {
        uint _307;
        __alias_dispatch_args_buffer.InterlockedAdd(64, _302, _307);
        _100[4u] = tile_offsets_buffer.Load<uint>(16) + _307;
    }
    uint _313 = WaveActiveSum(_101[5u]);
    if (WaveIsFirstLane())
    {
        uint _318;
        __alias_dispatch_args_buffer.InterlockedAdd(80, _313, _318);
        _100[5u] = tile_offsets_buffer.Load<uint>(20) + _318;
    }
    uint _324 = WaveActiveSum(_101[6u]);
    if (WaveIsFirstLane())
    {
        uint _329;
        __alias_dispatch_args_buffer.InterlockedAdd(96, _324, _329);
        _100[6u] = tile_offsets_buffer.Load<uint>(24) + _329;
    }
    _100[0u] = WaveReadLaneFirst(_100[0u]);
    _100[1u] = WaveReadLaneFirst(_100[1u]);
    _100[2u] = WaveReadLaneFirst(_100[2u]);
    _100[3u] = WaveReadLaneFirst(_100[3u]);
    _100[4u] = WaveReadLaneFirst(_100[4u]);
    _100[5u] = WaveReadLaneFirst(_100[5u]);
    _100[6u] = WaveReadLaneFirst(_100[6u]);
    _100[0u] += WavePrefixSum(_101[0u]);
    _100[1u] += WavePrefixSum(_101[1u]);
    _100[2u] += WavePrefixSum(_101[2u]);
    _100[3u] += WavePrefixSum(_101[3u]);
    _100[4u] += WavePrefixSum(_101[4u]);
    _100[5u] += WavePrefixSum(_101[5u]);
    _100[6u] += WavePrefixSum(_101[6u]);
    if (_138)
    {
        if (_203 != 0u)
        {
            uint2 _389 = _134 & uint2(1u, 1u);
            if (all(bool2(_389.x == uint2(0u, 0u).x, _389.y == uint2(0u, 0u).y)))
            {
                __alias_tile_coords_buffer.Store<uint2>(_100[6] * 8 + 0, _134 / uint2(2u, 2u));
            }
            __alias_tile_coords_buffer.Store<uint2>(_100[(_153 <= 5.0f) ? 2 : 1] * 8 + 0, _134);
        }
        else
        {
            uint2 _403 = _134 & uint2(1u, 1u);
            if (all(bool2(_403.x == uint2(0u, 0u).x, _403.y == uint2(0u, 0u).y)))
            {
                if (_209 != 0u)
                {
                    __alias_tile_coords_buffer.Store<uint2>(_100[5] * 8 + 0, _134 / uint2(2u, 2u));
                    __alias_tile_coords_buffer.Store<uint2>(_100[3] * 8 + 0, _134);
                    __alias_tile_coords_buffer.Store<uint2>((_100[3] + 1u) * 8 + 0, _134 + uint2(1u, 0u));
                    __alias_tile_coords_buffer.Store<uint2>((_100[3] + 2u) * 8 + 0, _134 + uint2(0u, 1u));
                    __alias_tile_coords_buffer.Store<uint2>((_100[3] + 3u) * 8 + 0, _134 + uint2(1u, 1u));
                }
                else
                {
                    __alias_tile_coords_buffer.Store<uint2>(_100[4] * 8 + 0, _134 / uint2(2u, 2u));
                    __alias_tile_coords_buffer.Store<uint2>(_100[0] * 8 + 0, _134);
                    __alias_tile_coords_buffer.Store<uint2>((_100[0] + 1u) * 8 + 0, _134 + uint2(1u, 0u));
                    __alias_tile_coords_buffer.Store<uint2>((_100[0] + 2u) * 8 + 0, _134 + uint2(0u, 1u));
                    __alias_tile_coords_buffer.Store<uint2>((_100[0] + 3u) * 8 + 0, _134 + uint2(1u, 1u));
                }
            }
        }
    }
}

[numthreads(8, 8, 1)]
void cs_main(SPIRV_Cross_Input stage_input)
{
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    gl_LocalInvocationIndex = stage_input.gl_LocalInvocationIndex;
    comp_main();
}
#pragma dxc diagnostic pop
#endif
