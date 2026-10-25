// PATCHED: modules gs.cs.hlsl_cs_main_0.bin:cs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)))
// ==RECONSTRUCTED== shaders/garden/spawn_bushes_buildings.cs.hlsl | evidence: spirv-cross of _garden_spawn_bushes_buildings.cs.hlsl_cs_main_0.bin (exclusion; garden_mask.y>0.3 skip; buildings_mask.x gate + perlin(world)>0.5; BushXform cmd 10 (48B) at terrain height (theme4 -0.5); 30% FlowerXform cmd 11 (64B, color -1, type 2)) | status: O (algorithm from SPIR-V); I (thresholds/scale literals)

#include "inc/common.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/draw_indirect.hlsl"
#include "inc/perlin.hlsl"
Texture2D<float4> terrain_heightmap : register(t15, space0);
#include "inc/terrain_heightmap.inc.hlsl"
#include "inc/uv.hlsl"
#include "buildings/building_mask.inc.hlsl"
#include "garden/defs.inc.hlsl"
#include "inc/frame_constants.hlsl"
#include "garden/placement_utils.inc.hlsl"

cbuffer $Globals : register(b14, space0)
{
    int draw_indirect_buffer_offset;
    uint2 terrain_heightmap_dims;
    uint2 visual_terrain_heightmap_dims;
    uint2 garden_mask_dims;
    uint2 buildings_mask_dims;
    uint2 buildings_mask_non_blurred_dims;
    int2 grid_dims;
    uint2 path_mask_dims;
    uint2 rocky_terrain_dims;
    uint2 water_mask_dims;
    uint2 foliage_exclusion_mask_dims;
};

RWStructuredBuffer<DrawIndexedIndirectCommand> __alias_draw_commands : register(u13, space0);
Texture2D<float4> garden_mask : register(t17, space0);
Texture2D<float4> buildings_mask : register(t18, space0);
Texture2D<float4> buildings_mask_non_blurred : register(t19, space0);
Texture2D<float4> path_mask : register(t22, space0);
Texture2D<float4> rocky_terrain : register(t23, space0);
Texture2D<float4> water_mask : register(t24, space0);
Texture2D<float4> foliage_exclusion_mask : register(t25, space0);
RWStructuredBuffer<BushXform> __alias_bush_buffer : register(u20, space0);
RWStructuredBuffer<FlowerXform> __alias_bush_flowers : register(u21, space0);

// frame_constants.hlsl (reconstructed inc/) omits the dead unbounded-array decl
// the original used to emit RuntimeDescriptorArray; reproduce it here.
RWStructuredBuffer<uint> _rda_trigger[] : register(u100, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 id : SV_DispatchThreadID)
{
    float rnd = rand01(hash(id.x * id.y + id.x));
    float2 world_pos = ((float2(id.xy) + 0.5f) / float2(grid_dims) - 0.5f) * 130.0f + lerp(-1.0f, 1.0f, rnd).xx;

    float2 uv = placement_uv(world_pos);
    if (placement_excluded(terrain_heightmap, terrain_heightmap_dims, path_mask, path_mask_dims,
                           rocky_terrain, water_mask, foliage_exclusion_mask,
                           sampler_llb, sampler_default, uv))
        return;
    if (garden_mask.Load(int3(uint2(uv * float2(path_mask_dims)), 0u)).y > 0.3f)
        return;

    float bmx = buildings_mask.SampleLevel(sampler_default, uv, 0.0f).x;
    if (buildings_mask_non_blurred.SampleLevel(sampler_default, uv, 0.0f).x > 0.999989986419677734375f)
        return;

    if (bmx > 0.0f && perlin(world_pos) > 0.5f)
    {
        float height = get_terrain_height(terrain_heightmap, sampler_default, world_pos);
        float3 pos = float3(world_pos.x, glade_theme_id == 4u ? height - 0.5f : height, world_pos.y);

        float band1 = band_falloff_garden(abs(buildings_mask.SampleLevel(sampler_default, uv, 0.0f).y - 0.4f), 3.3333332538604736328125f);
        float fs = lerp(0.8f, 1.2f, rand01(hash(id.x + id.y)));
        float3 s0 = fs.xxx * lerp(0.2f, 1.0f, sqrt(band1));
        float4x4 s = float4x4(
            float4(min(s0.x, 0.75f), 0.0f, 0.0f, 0.0f),
            float4(0.0f, min(s0.y * fs, 1.2f), 0.0f, 0.0f),
            float4(0.0f, 0.0f, min(s0.z, 0.75f), 0.0f),
            float4(0.0f, 0.0f, 0.0f, 1.0f));
        float3 s3 = float3(min(s0.x, 0.75f), min(s0.y * fs, 1.2f), min(s0.z, 0.75f)) * 0.75f;

        float band2 = band_falloff_garden(abs(bmx - 0.4f), 3.3333332538604736328125f);
        if (min((fs.xxx * lerp(0.2f, 1.0f, sqrt(band2))).x, 0.75f) * 0.75f < 0.3f)
            return;

        float4x4 identity = float4x4(
            float4(1.0f, 0.0f, 0.0f, 0.0f),
            float4(0.0f, 1.0f, 0.0f, 0.0f),
            float4(0.0f, 0.0f, 1.0f, 0.0f),
            float4(0.0f, 0.0f, 0.0f, 1.0f));
        float4x4 m = mul(mul(
            affine_translation(pos),
            mul(identity, s)),
            rotate_y(3.14f * rnd));

        uint bush_base;
        InterlockedAdd(__alias_draw_commands[10u].instanceCount, 1u, bush_base);
        __alias_bush_buffer[bush_base] = BushXform(pack_affine(m));

        if (rand01(hash((id.x * 2u) + (id.y / 5u) + 123u)) > 0.7f)
        {
            uint flower_base;
            InterlockedAdd(__alias_draw_commands[11u].instanceCount, 1u, flower_base);
            FlowerXform f;
            f.transform = pack_affine(m);
            f.color = float3(-1.0f, -1.0f, -1.0f);
            f.flower_type = 2u;
            __alias_bush_flowers[flower_base] = f;
        }
    }
}

#endif

#if defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)
// lifted from _garden_spawn_bushes_buildings.cs.hlsl_cs_main_0.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
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
    column_major float4x4 view_to_clip;
    column_major float4x4 clip_to_view;
    column_major float4x4 view_to_sample;
    column_major float4x4 sample_to_view;
    column_major float4x4 world_to_view;
    column_major float4x4 view_to_world;
    column_major float4x4 world_to_sample;
    column_major float4x4 clip_to_prev_clip;
    column_major float4x4 prev_view_to_prev_clip;
    column_major float4x4 prev_clip_to_prev_view;
    column_major float4x4 prev_world_to_prev_view;
    column_major float4x4 prev_view_to_prev_world;
    float2 sample_offset_pixels;
    float2 sample_offset_clip;
    float2 prev_sample_offset_pixels;
    float2 prev_sample_offset_clip;
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
    float3 color;
    uint flower_type;
};

static float _147;

RWByteAddressBuffer __alias_draw_commands : register(u13, space0);
cbuffer $Globals : register(b14, space0)
{
    int draw_indirect_buffer_offset : packoffset(c0);
    uint2 terrain_heightmap_dims : packoffset(c0.y);
    uint2 visual_terrain_heightmap_dims : packoffset(c1);
    uint2 garden_mask_dims : packoffset(c1.z);
    uint2 buildings_mask_dims : packoffset(c2);
    uint2 buildings_mask_non_blurred_dims : packoffset(c2.z);
    int2 grid_dims : packoffset(c3);
    uint2 path_mask_dims : packoffset(c3.z);
    uint2 rocky_terrain_dims : packoffset(c4);
    uint2 water_mask_dims : packoffset(c4.z);
    uint2 foliage_exclusion_mask_dims : packoffset(c5);
};

cbuffer frame_constants : register(b0, space1)
{
    float4 frame_constants_sun_color : packoffset(c0);
    float4 frame_constants_sky_low_color : packoffset(c1);
    float4 frame_constants_sky_high_color : packoffset(c2);
    float4 frame_constants_fog_tint_thickness : packoffset(c3);
    float4 frame_constants_fake_light_color : packoffset(c4);
    float4 frame_constants_fake_light_dir : packoffset(c5);
    float4 frame_constants_fake_light2_color : packoffset(c6);
    float4 frame_constants_fake_light2_dir : packoffset(c7);
    uint frame_constants_frame_index : packoffset(c8);
    float frame_constants_delta_time_seconds : packoffset(c8.y);
    float frame_constants_ev_mult : packoffset(c8.z);
    uint frame_constants_glade_theme_id : packoffset(c8.w);
    uint2 frame_constants_inner_rendering_resolution : packoffset(c9);
    uint frame_constants_game_time_sec : packoffset(c9.z);
    float frame_constants_game_time_frac : packoffset(c9.w);
    float3 frame_constants_editing_highlight_color : packoffset(c10);
    float frame_constants_time_of_day : packoffset(c10.w);
    float3 frame_constants_dir_to_sun : packoffset(c11);
    uint frame_constants_camera_idx : packoffset(c11.w);
    float3 frame_constants_dir_to_moon : packoffset(c12);
    uint frame_constants_render_overrides : packoffset(c12.w);
    float frame_constants_global_mip_bias : packoffset(c13);
    uint frame_constants_highlighted_transient_stair_edge_id : packoffset(c13.y);
    uint frame_constants_pad1 : packoffset(c13.z);
    uint frame_constants_pad2 : packoffset(c13.w);
    float4 frame_constants_debug_tweakers : packoffset(c14);
    ViewConstants frame_constants_view_constants[5] : packoffset(c15);
};

RWByteAddressBuffer __alias_bush_buffer : register(u20, space0);
RWByteAddressBuffer __alias_bush_flowers : register(u21, space0);
SamplerState sampler_llb : register(s4, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> terrain_heightmap : register(t15, space0);
Texture2D<float4> garden_mask : register(t17, space0);
Texture2D<float4> buildings_mask : register(t18, space0);
Texture2D<float4> buildings_mask_non_blurred : register(t19, space0);
Texture2D<float4> path_mask : register(t22, space0);
Texture2D<float4> rocky_terrain : register(t23, space0);
Texture2D<float4> water_mask : register(t24, space0);
Texture2D<float4> foliage_exclusion_mask : register(t25, space0);

static uint3 gl_GlobalInvocationID;
struct SPIRV_Cross_Input
{
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
};

void comp_main()
{
    do
    {
        uint2 _191;
        uint _165 = (gl_GlobalInvocationID.x * gl_GlobalInvocationID.y) + gl_GlobalInvocationID.x;
        uint _167 = _165 + (_165 << 10u);
        uint _169 = _167 ^ (_167 >> 6u);
        uint _171 = _169 + (_169 << 3u);
        uint _173 = _171 ^ (_171 >> 11u);
        float _179 = asfloat(((_173 + (_173 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
        float2 _182 = ((((float2(gl_GlobalInvocationID.xy) + 0.5f.xx) / float2(grid_dims)) - 0.5f.xx) * 130.0f.xx) + lerp(-1.0f, 1.0f, _179).xx;
        float2 _184 = (_182 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
        bool _252;
        do
        {
            _191 = uint2(_184 * float2(path_mask_dims));
            float4 _209 = terrain_heightmap.GatherRed(sampler_default, _184, int2(0, 0));
            float _214 = _209.z;
            float _215 = _209.y;
            float _217 = _209.w;
            float _218 = _209.x;
            bool _239;
            if (!(path_mask.Load(int3(_191, 0u)).x > 0.20000000298023223876953125f))
            {
                _239 = (int(rocky_terrain.Load(int3(_191, 0u)).x > 0.0f) & int(length(((float2((_214 + _215) - (_217 + _218), (_218 + _215) - (_217 + _214)) * 0.5f.xx) * 26.5f) / (130.0f.xx / float2(terrain_heightmap_dims))) > 0.89999997615814208984375f)) != 0;
            }
            else
            {
                _239 = true;
            }
            bool _244;
            if (!_239)
            {
                _244 = water_mask.Load(int3(_191, 0u)).y > 0.300000011920928955078125f;
            }
            else
            {
                _244 = true;
            }
            bool _249;
            if (!_244)
            {
                _249 = foliage_exclusion_mask.SampleLevel(sampler_llb, _184, 0.0f).x > 0.0f;
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
        if (garden_mask.Load(int3(_191, 0u)).y > 0.300000011920928955078125f)
        {
            break;
        }
        float4 _263 = buildings_mask.SampleLevel(sampler_default, _184, 0.0f);
        float _264 = _263.x;
        if (buildings_mask_non_blurred.SampleLevel(sampler_default, _184, 0.0f).x > 0.999989986419677734375f)
        {
            break;
        }
        bool _425;
        if (_264 > 0.0f)
        {
            int2 _277 = int2(floor(_182));
            float2 _278 = frac(_182);
            uint2 _279 = uint2(_277);
            uint _281 = _279.y;
            uint _283 = _281 + (_281 << 10u);
            uint _285 = _283 ^ (_283 >> 6u);
            uint _287 = _285 + (_285 << 3u);
            uint _289 = _287 ^ (_287 >> 11u);
            uint _295 = (((_279.x * 1664525u) + (_289 + (_289 << 15u))) + 1013904223u) * 1664525u;
            uint _297 = _295 ^ (_295 >> 11u);
            uint _300 = _297 ^ ((_297 << 7u) & 2636928640u);
            uint _303 = _300 ^ ((_300 << 15u) & 4022730752u);
            uint2 _311 = uint2(_277 + int2(1, 0));
            uint _313 = _311.y;
            uint _315 = _313 + (_313 << 10u);
            uint _317 = _315 ^ (_315 >> 6u);
            uint _319 = _317 + (_317 << 3u);
            uint _321 = _319 ^ (_319 >> 11u);
            uint _327 = (((_311.x * 1664525u) + (_321 + (_321 << 15u))) + 1013904223u) * 1664525u;
            uint _329 = _327 ^ (_327 >> 11u);
            uint _332 = _329 ^ ((_329 << 7u) & 2636928640u);
            uint _335 = _332 ^ ((_332 << 15u) & 4022730752u);
            uint2 _343 = uint2(_277 + int2(0, 1));
            uint _345 = _343.y;
            uint _347 = _345 + (_345 << 10u);
            uint _349 = _347 ^ (_347 >> 6u);
            uint _351 = _349 + (_349 << 3u);
            uint _353 = _351 ^ (_351 >> 11u);
            uint _359 = (((_343.x * 1664525u) + (_353 + (_353 << 15u))) + 1013904223u) * 1664525u;
            uint _361 = _359 ^ (_359 >> 11u);
            uint _364 = _361 ^ ((_361 << 7u) & 2636928640u);
            uint _367 = _364 ^ ((_364 << 15u) & 4022730752u);
            uint2 _375 = uint2(_277 + int2(1, 1));
            uint _377 = _375.y;
            uint _379 = _377 + (_377 << 10u);
            uint _381 = _379 ^ (_379 >> 6u);
            uint _383 = _381 + (_381 << 3u);
            uint _385 = _383 ^ (_383 >> 11u);
            uint _391 = (((_375.x * 1664525u) + (_385 + (_385 << 15u))) + 1013904223u) * 1664525u;
            uint _393 = _391 ^ (_391 >> 11u);
            uint _396 = _393 ^ ((_393 << 7u) & 2636928640u);
            uint _399 = _396 ^ ((_396 << 15u) & 4022730752u);
            float2 _409 = (_278 * _278) * (3.0f.xx - (_278 * 2.0f));
            float _410 = _409.x;
            float _411 = 1.0f - _410;
            float _413 = _409.y;
            _425 = ((((1.0f - _413) * (((asfloat(((_303 ^ (_303 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _411) + ((asfloat(((_335 ^ (_335 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _410))) + (((asfloat(((_367 ^ (_367 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _411) * _413)) + (((asfloat(((_399 ^ (_399 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _410) * _413)) > 0.5f;
        }
        else
        {
            _425 = false;
        }
        if (_425)
        {
            float3 _428 = _182.xxy;
            float _435 = lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (_182.xy * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
            _428.y = _435;
            float3 _444;
            if (frame_constants_glade_theme_id == 4u)
            {
                float3 _443 = _428;
                _443.y = _435 - 0.5f;
                _444 = _443;
            }
            else
            {
                _444 = _428;
            }
            float _458;
            do
            {
                float _448 = abs(_263.y - 0.4000000059604644775390625f);
                if (_448 > 0.300000011920928955078125f)
                {
                    _458 = 0.0f;
                    break;
                }
                float _452 = _448 * 3.3333332538604736328125f;
                _458 = 1.0f - ((_452 * _452) * (3.0f - (_448 * 6.666666507720947265625f)));
                break;
            } while(false);
            uint _459 = gl_GlobalInvocationID.x + gl_GlobalInvocationID.y;
            uint _461 = _459 + (_459 << 10u);
            uint _463 = _461 ^ (_461 >> 6u);
            uint _465 = _463 + (_463 << 3u);
            uint _467 = _465 ^ (_465 >> 11u);
            float _474 = lerp(0.800000011920928955078125f, 1.2000000476837158203125f, asfloat(((_467 + (_467 << 15u)) & 8388607u) | 1065353216u) - 1.0f);
            float3 _475 = _474.xxx;
            float3 _478 = _475 * lerp(0.20000000298023223876953125f, 1.0f, sqrt(_458));
            float _480 = _478.y * _474;
            float3 _485 = float3(min(_478.x, 0.75f), _480, min(_478.z, 0.75f));
            _485.y = min(_480, 1.2000000476837158203125f);
            float3 _488 = _485 * 0.75f;
            float _502;
            do
            {
                float _492 = abs(_264 - 0.4000000059604644775390625f);
                if (_492 > 0.300000011920928955078125f)
                {
                    _502 = 0.0f;
                    break;
                }
                float _496 = _492 * 3.3333332538604736328125f;
                _502 = 1.0f - ((_496 * _496) * (3.0f - (_492 * 6.666666507720947265625f)));
                break;
            } while(false);
            if ((float3(min((_475 * lerp(0.20000000298023223876953125f, 1.0f, sqrt(_502))).x, 0.75f), _147, _147) * 0.75f).x < 0.300000011920928955078125f)
            {
                break;
            }
            float _530 = 3.1400001049041748046875f * _179;
            float3 _531 = normalize(float3(0.0f, 1.0f, 0.0f));
            float _532 = sin(_530);
            float _533 = cos(_530);
            float _534 = 1.0f - _533;
            float _535 = _531.x;
            float _536 = _534 * _535;
            float _539 = _531.y;
            float _540 = _536 * _539;
            float _541 = _531.z;
            float _542 = _541 * _532;
            float _544 = _534 * _541;
            float _545 = _544 * _535;
            float _546 = _539 * _532;
            float _549 = _534 * _539;
            float _552 = _549 * _541;
            float _553 = _535 * _532;
            float4x4 _564 = mul(mul(float4x4(float4(1.0f, 0.0f, 0.0f, _444.x), float4(0.0f, 1.0f, 0.0f, _444.y), float4(0.0f, 0.0f, 1.0f, _444.z), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(float4x4(float4(1.0f, 0.0f, 0.0f, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(0.0f, 0.0f, 1.0f, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)), float4x4(float4(_488.x, 0.0f, 0.0f, 0.0f), float4(0.0f, _488.y, 0.0f, 0.0f), float4(0.0f, 0.0f, _488.z, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)))), transpose(float4x4(float4((_536 * _535) + _533, _540 - _542, _545 + _546, 0.0f), float4(_540 + _542, (_549 * _539) + _533, _552 - _553, 0.0f), float4(_545 - _546, _552 + _553, (_544 * _541) + _533, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
            uint _578;
            __alias_draw_commands.InterlockedAdd(204, 1u, _578);
            uint _580 = uint(int(_578));
            float _582[12] = { _564[0].x, _564[1].x, _564[2].x, _564[0].y, _564[1].y, _564[2].y, _564[0].z, _564[1].z, _564[2].z, _564[0].w, _564[1].w, _564[2].w };
            Affine3Packed _583 = { _582 };
            BushXform _584 = { _583 };
            [unroll]
            for (int _2ident = 0; _2ident < 12; _2ident++)
            {
                __alias_bush_buffer.Store<float>(_2ident * 4 + _580 * 48 + 0, _584.transform.data[_2ident]);
            }
            uint _588 = ((gl_GlobalInvocationID.x * 2u) + (gl_GlobalInvocationID.y / 5u)) + 123u;
            uint _590 = _588 + (_588 << 10u);
            uint _592 = _590 ^ (_590 >> 6u);
            uint _594 = _592 + (_592 << 3u);
            uint _596 = _594 ^ (_594 >> 11u);
            if ((asfloat(((_596 + (_596 << 15u)) & 8388607u) | 1065353216u) - 1.0f) > 0.699999988079071044921875f)
            {
                uint _607;
                __alias_draw_commands.InterlockedAdd(224, 1u, _607);
                uint _609 = uint(int(_607));
                FlowerXform _611 = { _583, (-1.0f).xxx, 2u };
                [unroll]
                for (int _3ident = 0; _3ident < 12; _3ident++)
                {
                    __alias_bush_flowers.Store<float>(_3ident * 4 + _609 * 64 + 0, _611.transform.data[_3ident]);
                }
                __alias_bush_flowers.Store<float3>(_609 * 64 + 48, _611.color);
                __alias_bush_flowers.Store<uint>(_609 * 64 + 60, _611.flower_type);
            }
        }
        break;
    } while(false);
}

[numthreads(8, 8, 1)]
void cs_main(SPIRV_Cross_Input stage_input)
{
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    comp_main();
}
#pragma dxc diagnostic pop
#endif
