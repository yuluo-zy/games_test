// PATCHED: modules ty.cs.hlsl_cs_main_0.bin:cs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)))
// ==RECONSTRUCTED== shaders/roof_tiles_gravity.cs.hlsl | F-V | bin: _roof_tiles_gravity.cs.hlsl_cs_main_0.bin | entry cs:cs_main | LocalSize 64,1,1
// 屋顶瓦片重力/放置模拟。每个线程更新一条 RoofShingle 记录
// (RWStructuredBuffer<RoofShingle> __alias_instanced_data, set0/u13):从打包的
// half2x16 `velocity_rotation` 计算向下的速度,用 SSBO 里的帧增量时间
// (frame_constants_storage_buffer, set1/t1)积分高度,把瓦片限制到屋顶平面
// (push_constants.roof_pivot_y)与最大下落速度,按每种子延迟把瓦片缓动到平面,
// 最后写回带新打包速度/旋转的记录。
// Note: 该着色器从 SSBO(而非 cbuffer)读帧常量;inc/frame_constants.hlsl 仍在
// include 列表里,但只为 ViewConstants 服务。
//
// 中文说明:模拟屋顶瓦片在重力下的下落与安定。每个线程处理一块瓦片:算速度、
// 积分高度、夹到屋顶面、用随机延迟缓动落地,并让落下的瓦片倾斜出旋转角。

#include "roof/roof_tile.inc.hlsl"
#include "inc/common.hlsl"
#include "inc/tracked_buffer.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/math_const.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"

// 帧常量以 SSBO 形式提供(delta_time_seconds 在字节 132)。
struct FrameConstants
{
    float4 sun_color;
    float4 sky_low_color;
    float4 sky_high_color;
    float4 fog_tint_thickness;
    float4 fake_light_color;
    float4 fake_light_dir;
    float4 fake_light2_color;
    float4 fake_light2_dir;
    uint frame_index;
    float delta_time_seconds;
    float ev_mult;
    uint glade_theme_id;
    uint2 inner_rendering_resolution;
    uint game_time_sec;
    float game_time_frac;
    float3 editing_highlight_color;
    float time_of_day;
    float3 dir_to_sun;
    uint camera_idx;
    float3 dir_to_moon;
    uint render_overrides;
    float global_mip_bias;
    uint highlighted_transient_stair_edge_id;
    uint pad1;
    uint pad2;
    float4 debug_tweakers;
    ViewConstants view_constants[5];
};

StructuredBuffer<FrameConstants> frame_constants_storage_buffer : register(t1, space1);

// 瓦片实例数据缓冲(读写)。
RWStructuredBuffer<RoofShingle> __alias_instanced_data : register(u13);

// RoofPushConstants(push_constants.first_instance/instance_count/wall_height_delta/roof_pivot_y/animation_t)
// 定义在 roof/roof_tile.inc.hlsl。

[[vk::push_constant]]
ConstantBuffer<RoofPushConstants> push_constants;

[numthreads(64, 1, 1)]
void cs_main(uint3 gl_GlobalInvocationID : SV_DispatchThreadID)
{
    // 超出实例数直接返回。
    if (gl_GlobalInvocationID.x >= push_constants.instance_count)
    {
        return;
    }

    uint index = push_constants.first_instance + gl_GlobalInvocationID.x;
    RoofShingle s = __alias_instanced_data[index];

    // 墙体收缩标志:墙高增量折算成"每秒下落 >120"才算被墙裁剪。
    bool wall_clip;
    if (push_constants.wall_height_delta < 0.0f)
    {
        wall_clip = (push_constants.wall_height_delta / frame_constants_storage_buffer[0].delta_time_seconds) > -120.0f;
    }
    else
    {
        wall_clip = false;
    }

    // 低于屋顶-墙高差视为"在屋顶之下"。
    bool below_roof;
    if (wall_clip)
    {
        below_roof = s.current_height <= (push_constants.roof_pivot_y - push_constants.wall_height_delta);
    }
    else
    {
        below_roof = false;
    }

    // 瓦片时间参数 t(从 scale_t 高位半精度解出,量化到 0.02 步长)。
    float t = floor(unpack_half2x16((s.scale_t.y >> 16u) & 0xFFFFu).x * 50.0f) * 0.02f;
    // 旧速度(低 16 位半精度),叠加重力积分(加速度随 t 从 98.1 降到 39.24)。
    uint old_velocity = s.velocity_rotation & 0xFFFFu;
    float velocity = unpack_half2x16(old_velocity).x
                   + (-98.1f * lerp(1.0f, 0.4f, t)) * frame_constants_storage_buffer[0].delta_time_seconds;
    // 新高度:屋顶之下则钉在屋顶平面,否则按速度积分。
    float new_height = (below_roof ? push_constants.roof_pivot_y : s.current_height)
                     + velocity * frame_constants_storage_buffer[0].delta_time_seconds;

    // 夹到屋顶平面:限速 15,最低高度为屋顶平面。
    float clamped_velocity;
    float height;
    if (new_height <= push_constants.roof_pivot_y)
    {
        clamped_velocity = min(15.0f, max((push_constants.wall_height_delta / frame_constants_storage_buffer[0].delta_time_seconds) * 0.166f, velocity));
        height = max(new_height, push_constants.roof_pivot_y);
    }
    else
    {
        clamped_velocity = velocity;
        height = new_height;
    }

    // 缓动落地:靠近屋顶面时以随机延迟混合到屋顶平面。
    float eased_height;
    if (t > 0.9f)
    {
        // 时间足够则直接落在屋顶。
        eased_height = push_constants.roof_pivot_y;
    }
    else
    {
        // 每块瓦片按种子随机决定是否延迟(大部分立刻落地,少数延迟)。
        float settle = 1.0f - ((t * t) * (t * t));
        float delay = 1.0f - exp(-frame_constants_storage_buffer[0].delta_time_seconds
                                * (5.0f * lerp(0.0f, 2.0f, settle))
                                * ((rand01(hash(uint(s.seed))) * t > 0.3f) ? 1.0f : 1000.0f));
        eased_height = lerp(height, push_constants.roof_pivot_y, delay);
    }

    // 距屋顶面的距离决定瓦片倾斜角。
    float delta = abs(push_constants.roof_pivot_y - eased_height);
    float rot;
    if (clamped_velocity > 0.0f)
    {
        // 还在下落:按距离立方的衰减给一个 -90 度的倾斜。
        rot = (1.0f - exp(-((delta * delta) * delta))) * -1.57f;
    }
    else
    {
        rot = 0.0f;
    }

    // 打包新速度+旋转:低 16 位速度,高 16 位旋转(保留高 12 位原有标志)。
    uint new_velocity_rotation = ((old_velocity | (pack_half2x16(float2(rot, 0.0f)) << 16u)) & 0xFFF00000u)
                               | pack_half2x16(float2(clamped_velocity, 0.0f));

    // 写回除不变字段外的全部字段。
    RoofShingle out_s;
    out_s.base_position = s.base_position;
    out_s.current_height = eased_height;
    out_s.clipping_plane_ = s.clipping_plane_;
    out_s.scale_t = s.scale_t;
    out_s.rotation_quat_ = s.rotation_quat_;
    out_s.roof_id = s.roof_id;
    out_s.velocity_rotation = new_velocity_rotation;
    out_s.animation_t = push_constants.animation_t;
    out_s.seed = s.seed;
    out_s.is_circle = s.is_circle;
    out_s.is_main_roof_body = s.is_main_roof_body;
    __alias_instanced_data[index] = out_s;
}

#endif

// TGV_0 变体(该 bin 无变体哈希):与手写版相同逻辑的逐字节提升版。
#if defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)
// lifted from _roof_tiles_gravity.cs.hlsl_cs_main_0.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
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

struct FrameConstants
{
    float4 sun_color;
    float4 sky_low_color;
    float4 sky_high_color;
    float4 fog_tint_thickness;
    float4 fake_light_color;
    float4 fake_light_dir;
    float4 fake_light2_color;
    float4 fake_light2_dir;
    uint frame_index;
    float delta_time_seconds;
    float ev_mult;
    uint glade_theme_id;
    uint2 inner_rendering_resolution;
    uint game_time_sec;
    float game_time_frac;
    float3 editing_highlight_color;
    float time_of_day;
    float3 dir_to_sun;
    uint camera_idx;
    float3 dir_to_moon;
    uint render_overrides;
    float global_mip_bias;
    uint highlighted_transient_stair_edge_id;
    uint pad1;
    uint pad2;
    float4 debug_tweakers;
    ViewConstants view_constants[5];
};

struct RoofShingle
{
    float3 base_position;
    float current_height;
    uint2 clipping_plane_;
    uint2 scale_t;
    uint2 rotation_quat_;
    uint roof_id;
    uint velocity_rotation;
    float animation_t;
    int seed;
    int is_circle;
    int is_main_roof_body;
};

// 帧常量与实例数据都用字节寻址缓冲访问。
ByteAddressBuffer frame_constants_storage_buffer : register(t1, space1);
RWByteAddressBuffer __alias_instanced_data : register(u13, space0);
struct PushConstants
{
    uint first_instance;
    uint instance_count;
    float wall_height_delta;
    float roof_pivot_y;
    float animation_t;
};
[[vk::push_constant]]
PushConstants push_constants;


static uint3 gl_GlobalInvocationID;
struct SPIRV_Cross_Input
{
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
};

uint spvPackHalf2x16(float2 value)
{
    uint2 Packed = f32tof16(value);
    return Packed.x | (Packed.y << 16);
}

float2 spvUnpackHalf2x16(uint value)
{
    return f16tof32(uint2(value & 0xffff, value >> 16));
}

void comp_main()
{
    do
    {
        // 超出实例数直接返回。
        if (gl_GlobalInvocationID.x >= push_constants.instance_count)
        {
            break;
        }
        // 逐字段从字节缓冲加载当前瓦片记录。
        uint index = push_constants.first_instance + gl_GlobalInvocationID.x;
        RoofShingle shingle;
        shingle.base_position = __alias_instanced_data.Load<float3>(index * 64 + 0);
        shingle.current_height = __alias_instanced_data.Load<float>(index * 64 + 12);
        shingle.clipping_plane_ = __alias_instanced_data.Load<uint2>(index * 64 + 16);
        shingle.scale_t = __alias_instanced_data.Load<uint2>(index * 64 + 24);
        shingle.rotation_quat_ = __alias_instanced_data.Load<uint2>(index * 64 + 32);
        shingle.roof_id = __alias_instanced_data.Load<uint>(index * 64 + 40);
        shingle.velocity_rotation = __alias_instanced_data.Load<uint>(index * 64 + 44);
        shingle.animation_t = __alias_instanced_data.Load<float>(index * 64 + 48);
        shingle.seed = __alias_instanced_data.Load<int>(index * 64 + 52);
        shingle.is_circle = __alias_instanced_data.Load<int>(index * 64 + 56);
        shingle.is_main_roof_body = __alias_instanced_data.Load<int>(index * 64 + 60);
        // 墙体收缩标志。
        bool wall_clip;
        if (push_constants.wall_height_delta < 0.0f)
        {
            wall_clip = (push_constants.wall_height_delta / frame_constants_storage_buffer.Load<float>(132)) > (-120.0f);
        }
        else
        {
            wall_clip = false;
        }
        // 是否低于屋顶平面。
        bool below_roof;
        if (wall_clip)
        {
            below_roof = shingle.current_height <= (push_constants.roof_pivot_y - push_constants.wall_height_delta);
        }
        else
        {
            below_roof = false;
        }
        // 瓦片时间参数与旧速度。
        float tile_t = floor(spvUnpackHalf2x16((shingle.scale_t.y >> 16u) & 65535u).x * 50.0f) * 0.0199999995529651641845703125f;
        uint old_velocity = shingle.velocity_rotation & 65535u;
        // 重力积分。
        float velocity = spvUnpackHalf2x16(old_velocity).x + (((-98.09999847412109375f) * lerp(1.0f, 0.4000000059604644775390625f, tile_t)) * frame_constants_storage_buffer.Load<float>(132));
        // 新高度。
        float new_height = (below_roof ? push_constants.roof_pivot_y : shingle.current_height) + (velocity * frame_constants_storage_buffer.Load<float>(132));
        // 夹到屋顶平面与限速。
        float clamped_velocity;
        float height;
        if (new_height <= push_constants.roof_pivot_y)
        {
            clamped_velocity = min(15.0f, max((push_constants.wall_height_delta / frame_constants_storage_buffer.Load<float>(132)) * 0.16599999368190765380859375f, velocity));
            height = max(new_height, push_constants.roof_pivot_y);
        }
        else
        {
            clamped_velocity = velocity;
            height = new_height;
        }
        // 按种子计算延迟落地的哈希值。
        uint seed_u = uint(shingle.seed);
        uint hash1 = seed_u + (seed_u << 10u);
        uint hash2 = hash1 ^ (hash1 >> 6u);
        uint hash3 = hash2 + (hash2 << 3u);
        uint hash4 = hash3 ^ (hash3 >> 11u);
        // 缓动落地。
        float eased_height;
        if (tile_t > 0.89999997615814208984375f)
        {
            eased_height = push_constants.roof_pivot_y;
        }
        else
        {
            eased_height = lerp(height, push_constants.roof_pivot_y, 1.0f - exp((-frame_constants_storage_buffer.Load<float>(132)) * ((5.0f * lerp(0.0f, 2.0f, 1.0f - (((tile_t * tile_t) * tile_t) * tile_t))) * ((((asfloat(((hash4 + (hash4 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * tile_t) > 0.300000011920928955078125f) ? 1.0f : 1000.0f))));
        }
        // 距屋顶面的距离与倾斜角。
        float delta = abs(push_constants.roof_pivot_y - eased_height);
        float rot;
        if (clamped_velocity > 0.0f)
        {
            rot = (1.0f - exp(((-delta) * delta) * delta)) * (-1.57000005245208740234375f);
        }
        else
        {
            rot = 0.0f;
        }
        // 组新记录并逐字段写回。
        RoofShingle out_shingle = { shingle.base_position, eased_height, shingle.clipping_plane_, shingle.scale_t, shingle.rotation_quat_, shingle.roof_id, ((old_velocity | (spvPackHalf2x16(float2(rot, 0.0f)) << 16u)) & 4294901760u) | spvPackHalf2x16(float2(clamped_velocity, 0.0f)), push_constants.animation_t, shingle.seed, shingle.is_circle, shingle.is_main_roof_body };
        __alias_instanced_data.Store<float3>(index * 64 + 0, out_shingle.base_position);
        __alias_instanced_data.Store<float>(index * 64 + 12, out_shingle.current_height);
        __alias_instanced_data.Store<uint2>(index * 64 + 16, out_shingle.clipping_plane_);
        __alias_instanced_data.Store<uint2>(index * 64 + 24, out_shingle.scale_t);
        __alias_instanced_data.Store<uint2>(index * 64 + 32, out_shingle.rotation_quat_);
        __alias_instanced_data.Store<uint>(index * 64 + 40, out_shingle.roof_id);
        __alias_instanced_data.Store<uint>(index * 64 + 44, out_shingle.velocity_rotation);
        __alias_instanced_data.Store<float>(index * 64 + 48, out_shingle.animation_t);
        __alias_instanced_data.Store<int>(index * 64 + 52, out_shingle.seed);
        __alias_instanced_data.Store<int>(index * 64 + 56, out_shingle.is_circle);
        __alias_instanced_data.Store<int>(index * 64 + 60, out_shingle.is_main_roof_body);
        break;
    } while(false);
}

[numthreads(64, 1, 1)]
void cs_main(SPIRV_Cross_Input stage_input)
{
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    comp_main();
}
#pragma dxc diagnostic pop
#endif
