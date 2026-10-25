// ==RECONSTRUCTED== shaders/grass/calc_tiles.cs.hlsl
// Evidence: _grass_calc_tiles.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23).
//   bindings: $Globals b13 (terrain_heightmap_dims/visual_terrain_heightmap_dims/
//   hzb_tex_inner_dims/freeze_occlusion_culling), terrain_heightmap t14, __alias_tile_buffer u16
//   (RWStructuredBuffer<GrassTilePacked>), __alias_blade_prefix_buffer u17 / __alias_blade_prefix_subtotal_buffer u18
//   (RWStructuredBuffer<uint>), hzb_tex t19, sampler_default s9, frame_constants set1/b0.
//   LocalSize [64,1,1], entry cs_main.
//   spirv-cross + disasm: per tile, view-frustum corner-plane culling then a 4-tap
//   hierarchical-Z occlusion test, produces a blade budget (0..256, capped) that is
//   packed back into GrassTilePacked{tile_x<<24 | tile_y<<16 | blades}, then a 64-lane
//   inclusive Hillis-Steele scan of the budgets into blade-prefix + group subtotal.
//   Status: O (structure verbatim).
//
// 草地瓦片裁剪与预算:每瓦片先做视锥角平面裁剪,再做 4 采样点 HZB 遮挡测试,
// 得到刀片预算(0..256)打包回瓦片记录,最后做 64 lane 前缀扫描得到刀片前缀。

#include "inc/common.hlsl"
#include "grass/grass_common.inc.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/morton.hlsl"
#include "inc/frustum.hlsl"
#include "inc/lod.hlsl"
#include "inc/math.hlsl"
#include "inc/projection.hlsl"

Texture2D<float4> terrain_heightmap : register(t14, space0);
#include "inc/terrain_heightmap.inc.hlsl"
#include "inc/prefix_scan.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint2 terrain_heightmap_dims;
    uint2 visual_terrain_heightmap_dims;
    uint2 hzb_tex_inner_dims;
    uint freeze_occlusion_culling;
};

RWStructuredBuffer<GrassTilePacked> __alias_tile_buffer : register(u16, space0);
RWStructuredBuffer<uint> __alias_blade_prefix_buffer : register(u17, space0);
RWStructuredBuffer<uint> __alias_blade_prefix_subtotal_buffer : register(u18, space0);
Texture2D<float4> hzb_tex : register(t19, space0);

// 远平面 / 瓦片锥半张角(基于 tan),草地遮挡裁剪的内嵌常量(1.43631 = sqrt(2.062988))。
static const float GRASS_FRUSTUM_FAR = 1.43631064891815185546875f;

groupshared uint shared_data1x[64];

[numthreads(64, 1, 1)]
void cs_main(uint3 group_id : SV_GroupID, uint3 dispatch_id : SV_DispatchThreadID, uint local_index : SV_GroupIndex)
{
    GrassTilePacked tile = __alias_tile_buffer[dispatch_id.x];
    uint tile_blades;
    if (freeze_occlusion_culling != 0u)
    {
        // 冻结遮挡裁剪:直接用原始刀片数。
        tile_blades = (tile.raw & 0xFFFFu) % 512u;
    }
    else
    {
        uint tile_x = dispatch_id.x % 128u;
        uint tile_y = dispatch_id.x / 128u;
        float2 tile_xy = float2(float(tile_x), float(tile_y));
        float2 tile_center_ws = (tile_xy - 63.5f) * (1.0f / 128.0f) * 260.0f;

        // 从 view_to_clip 的四行构造视锥角平面(归一化)。
        float4 row_x = view_constants[0].view_to_clip[0];
        float4 row_y = view_constants[0].view_to_clip[1];
        float4 row_z = view_constants[0].view_to_clip[3];
        float4 plane_x = float4(-row_x.x, row_x.y, 0.0f, row_x.w);
        float4 plane_y = float4(row_y.x, row_y.y, 0.0f, row_y.w);
        float4 plane_z = float4(row_z.x, row_z.y, -1.0f, row_z.w);
        float4 corners[4];
        corners[0] = plane_z + plane_x;
        corners[1] = plane_z - plane_x;
        corners[2] = plane_z + plane_y;
        corners[3] = plane_z - plane_y;
        for (int i = 0; i < 4; i++)
        {
            corners[i] /= length(corners[i].xyz).xxxx;
        }

        // 采样高度图得到瓦片中心高度,变换到视图空间。
        float tile_height = lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, tile_center_ws * (1.0f / 130.0f) + 0.5f, 0.0f).x);
        float4 view_pos = mul(view_constants[0].world_to_view, float4(tile_center_ws.x, tile_height, tile_center_ws.y, 1.0f));
        float z = view_pos.z;

        // 视锥锥体内测试:远平面在瓦片中心之后,且在四个角平面之内。
        bool inside;
        if ((GRASS_FRUSTUM_FAR - z) > view_constants[0].view_to_clip[2].w)
        {
            inside = ((-GRASS_FRUSTUM_FAR) - z) < 3.4028234663852885981170418348452e+38f;
        }
        else
        {
            inside = false;
        }
        bool running = inside;
        bool next;
        for (int i = 0; i < 4; running = next, i++)
        {
            if (running)
            {
                next = dot(float4(view_pos.x, view_pos.y, z, 1.0f), corners[i]) > (-GRASS_FRUSTUM_FAR);
            }
            else
            {
                next = false;
            }
        }

        // 瓦片锥在视图空间中的投影包围盒(HZB 上)。
        float inv_z = -z;
        float3 dir = float3(view_pos.x, view_pos.y, inv_z) * GRASS_FRUSTUM_FAR;
        float z_sq = (inv_z * inv_z) - (GRASS_FRUSTUM_FAR * GRASS_FRUSTUM_FAR);
        float rx = sqrt((view_pos.x * view_pos.x) + z_sq);
        float ry = sqrt((view_pos.y * view_pos.y) + z_sq);
        float2 proj_scale = float2(view_constants[0].view_to_clip[0].x * 0.5f, view_constants[0].view_to_clip[1].y * 0.5f);
        float4 rect;
        rect.x = ((rx * view_pos.x - dir.z) / (rx * inv_z + dir.x)) * proj_scale.x;
        rect.y = (-((ry * view_pos.y + dir.z) / (ry * inv_z - dir.y))) * proj_scale.y;
        rect.z = ((rx * view_pos.x + dir.z) / (rx * inv_z - dir.x)) * proj_scale.x;
        rect.w = (-((ry * view_pos.y - dir.z) / (ry * inv_z + dir.y))) * proj_scale.y;
        float4 hzb_dims = float4(hzb_tex_inner_dims.xyxy);
        float4 hzb_uv = clamp((rect + 0.5f) * hzb_dims, 0.0f, hzb_dims - 1.0f);
        // 选择覆盖盒大小对应的 mip 级。
        uint hzb_mip = uint(max(0.0f, ceil(log2(max(hzb_uv.z - hzb_uv.x, hzb_uv.w - hzb_uv.y)))));

        // 四角 HZB 遮挡测试:取覆盖盒四个角的最小深度作遮挡深度。
        bool visible;
        if (running)
        {
            uint2 mip_shift = hzb_mip.xx & uint2(31u, 31u);
            uint2 lo = uint2(hzb_uv.xy) >> mip_shift;
            uint2 hi = uint2(hzb_uv.zw) >> mip_shift;
            float min_depth = min(min(hzb_tex.Load(int3(lo, hzb_mip)).x, hzb_tex.Load(int3(uint2(lo.x, hi.y), hzb_mip)).x), min(hzb_tex.Load(int3(uint2(hi.x, lo.y), hzb_mip)).x, hzb_tex.Load(int3(hi, hzb_mip)).x));
            visible = (z + GRASS_FRUSTUM_FAR) > (1.0f / (min_depth * (-view_constants[0].clip_to_view[3].z)));
        }
        else
        {
            visible = running;
        }

        // 遮挡因子:距离越远越小,叠加渲染分辨率缩放。
        float occ;
        if (!visible)
        {
            occ = 0.0f;
        }
        else
        {
            occ = clamp((0.02f / max(1e-5f, inv_z)) * ((0.5f * view_constants[0].view_to_clip[1].y) * float(clamp(inner_rendering_resolution.y, 1080u, 2160u))), 0.0f, 1.0f);
        }
        // 距地图中心越远越淡出;最终刀片预算封顶 256。
        float fade = smoothstep(64.0f, 21.3333339691162109375f, length(tile_xy - 64.0f));
        tile_blades = uint(clamp(lerp(0.0f, 204.8000030517578125f, occ * fade), 0.0f, 256.0f));
        GrassTilePacked new_tile = { ((tile_x << 24u) | (tile_y << 16u)) | tile_blades };
        __alias_tile_buffer[dispatch_id.x] = new_tile;
    }

    // 64 lane Hillis-Steele 含式扫描:刀片预算 -> 前缀 + 组子合计。
    shared_data1x[local_index] = tile_blades;
    GroupMemoryBarrierWithGroupSync();
    uint i = (local_index >> 0u) << 1u;
    uint j = i + 1u;
    if (j < 64u)
    {
        shared_data1x[j] += shared_data1x[i];
    }
    GroupMemoryBarrierWithGroupSync();
    i = (local_index >> 1u) << 2u;
    j = i + 2u + (local_index & 1u);
    if (j < 64u)
    {
        shared_data1x[j] += shared_data1x[i + 1u];
    }
    GroupMemoryBarrierWithGroupSync();
    i = (local_index >> 2u) << 3u;
    j = i + 4u + (local_index & 3u);
    if (j < 64u)
    {
        shared_data1x[j] += shared_data1x[i + 3u];
    }
    GroupMemoryBarrierWithGroupSync();
    i = (local_index >> 3u) << 4u;
    j = i + 8u + (local_index & 7u);
    if (j < 64u)
    {
        shared_data1x[j] += shared_data1x[i + 7u];
    }
    GroupMemoryBarrierWithGroupSync();
    i = (local_index >> 4u) << 5u;
    j = i + 16u + (local_index & 15u);
    if (j < 64u)
    {
        shared_data1x[j] += shared_data1x[i + 15u];
    }
    GroupMemoryBarrierWithGroupSync();
    i = (local_index >> 5u) << 6u;
    j = i + 32u + (local_index & 31u);
    if (j < 64u)
    {
        shared_data1x[j] += shared_data1x[i + 31u];
    }
    GroupMemoryBarrierWithGroupSync();
    __alias_blade_prefix_buffer[dispatch_id.x] = (local_index == 0u) ? 0u : shared_data1x[local_index - 1u];
    if (local_index == 0u)
    {
        __alias_blade_prefix_subtotal_buffer[group_id.x] = shared_data1x[63];
    }
}
