// ==RECONSTRUCTED== shaders/clearing/tree_canopy.raster.hlsl
// Evidence: _clearing_tree_canopy.raster.hlsl_b903e43ffb3da915.bin / _f0adff76827e73ff.bin.
//   bindings (vs): $Globals b13 (current_view_index/is_firstperson_mode/wind_strength/
//   blue_noise_texture_dims/cone_angle_mult/max_cone_cull_dist_range), nani_instance_data t15,
//   instance_indirection_buffer t16, frame_constants. (ps): sampler_llr s1, sampler_llc s3,
//   $Globals b13, color_library b14, canopy_alpha_texture t17, canopy_flowers_color t18,
//   canopy_flowers_depth t19, canopy_color t20, blue_noise_texture t21, frame_constants.
//   vs_main: canopy growth/sway, two-stage placement (prim center + leaf billboard),
//   frustum/facing cull, per-leaf depth push. ps_main: flower depth/color blending, alpha,
//   cone cull with blue-noise jitter, detail-canopy texturing, packed uint2 + SV_DepthLessEqual.
//   Status: O (structure verbatim).

#include "inc/common.hlsl"
#include "inc/current_view.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"

// color_library.hlsl guard-skipped; flat-member cbuffer at set0/b14.
#define TINYGLADE_INC_COLOR_LIBRARY_HLSL
#include "inc/color_library.hlsl"
cbuffer color_library : register(b14, space0)
{
    float4 temp_color_1;
    float4 temp_color_2;
    float4 temp_color_3;
    float4 roof_color_1;
    float4 roof_color_2;
    float4 sky_low_day;
    float4 sky_high_day;
    float4 sun_tint_day;
    float4 sky_low_dusk;
    float4 sky_high_dusk;
    float4 sun_tint_dusk;
    float4 indirect_light_tint;
    float4 fence_color_1;
    float4 fence_color_2;
    float4 flower_color_1;
    float4 flower_color_2;
    float4 canopy_color_1;
    float4 canopy_color_2;
    float4 grass_1;
    float4 grass_2;
    float4 path;
    float4 sky_base;
    float4 sky_horizon;
    float4 sky_clouds_1;
    float4 sky_clouds_2;
    float4 terrain_1;
    float4 terrain_2;
    float4 terrain_flattened;
    float4 wall_1;
    float4 wall_2;
    float4 wall_moss;
    float4 wall_bricks;
    float4 ivy_branch;
    float4 ivy_leaf_1;
    float4 ivy_leaf_2;
    float4 terrain_highlight;
    float4 night_tint;
};
#include "inc/hammersley.hlsl"
#include "inc/pack_unpack.hlsl"
#include "clearing/canopy_vert.inc.hlsl"
#include "inc/math.hlsl"
#include "inc/perlin.hlsl"
#include "clearing/inc/instanced_placement.hlsl"
#include "clearing/inc/canopy_common.inc.hlsl"
#include "nani/inc/user.hlsl"
// NOTE: inc/blue_noise.hlsl omitted (declares blue_noise_texture at t14; this shader
// binds it at t21 and samples it directly, not via the blue_noise() helper).
#include "inc/material/canopy_material.inc.hlsl"
#include "inc/material_eval.inc.hlsl"
#include "inc/brdf.hlsl"
// NOTE: inc/lighting.hlsl omitted (nested color_library reference; see billboard_forest note).
#include "inc/sun.inc.hlsl"
#include "inc/color/srgb.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/sh.hlsl"

// $Globals members (implicit cbuffer at set0/b13).
int is_firstperson_mode;
float wind_strength;
uint2 blue_noise_texture_dims;
float cone_angle_mult;
float max_cone_cull_dist_range;

ByteAddressBuffer nani_instance_data : register(t15, space0);
ByteAddressBuffer instance_indirection_buffer : register(t16, space0);
Texture2D<float4> canopy_alpha_texture : register(t17, space0);
Texture2D<float4> canopy_flowers_color : register(t18, space0);
Texture2D<float4> canopy_flowers_depth : register(t19, space0);
Texture2D<float4> canopy_color : register(t20, space0);
Texture2D<float4> blue_noise_texture : register(t21, space0);

struct VS_IN
{
    float3 vertex_position : TEXCOORD0;
    float3 vertex_color : TEXCOORD1;
    float3 vertex_normal : TEXCOORD2;
    float3 prim_center : TEXCOORD3;
    float3 prim_normals : TEXCOORD4;
    float3 appear_pos : TEXCOORD5;
    float age : TEXCOORD6;
};

struct VS_OUT
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float3 C3 : TEXCOORD3;
    nointerpolation int C4 : TEXCOORD4;
    nointerpolation int C5 : TEXCOORD5;
    nointerpolation float C6 : TEXCOORD6;
    float3 C7 : TEXCOORD7;
    nointerpolation uint C8 : TEXCOORD8;
    nointerpolation uint C9 : TEXCOORD9;
    nointerpolation float C10 : TEXCOORD10;
    float2 C11 : TEXCOORD11;
    float C12 : TEXCOORD12;
    float4 position : SV_Position;
};

VS_OUT vs_main(VS_IN i, uint instance_id : SV_InstanceID)
{
    VS_OUT o;
    float4 wp;
    float3 canopy_pos;
    float3 leaf_color;
    float3 leaf_normal;
    float3 world_pos;
    int tree_id;
    int leaf_seed;
    float anim_high;
    float3 prim_center;
    bool flower_bool;
    uint part_id;
    float leaf_noise;
    float2 clip_zw;
    float depth_fade;
    do
    {
        uint indir = instance_indirection_buffer.Load<uint>(instance_id * 4u + 0);
        float anim_low = clamp(f16tof32(nani_instance_data.Load<uint>(indir * 48u + 32) & 0xFFFFu), 0.0f, 1.0f);
        float growth = clamp(1.0f - clamp(lerp(-1.0f, 1.0f, anim_low) + (1.0f - i.age), 0.0f, 1.0f), 0.0f, 1.0f);
        float grow_f = 1.0f - ((growth * growth) - (growth * sin(growth * 3.14f)));
        float3 pos = lerp(i.appear_pos, i.vertex_position, grow_f);
        if (grow_f < 0.01f)
        {
            wp = asfloat(0x7fbfffffu);
            canopy_pos = 0.0f;
            leaf_color = 0.0f;
            leaf_normal = 0.0f;
            world_pos = 0.0f;
            tree_id = 0;
            leaf_seed = 0;
            anim_high = 0.0f;
            prim_center = 0.0f;
            flower_bool = false;
            part_id = 0u;
            leaf_noise = 0.0f;
            clip_zw = 0.0f;
            depth_fade = 0.0f;
            break;
        }

        // Canopy sway.
        float wind_amp = pos.y * 0.03f;
        float t_sec = float(game_time_sec);
        float t_frac = game_time_frac;
        float x_amp = 0.2f * pos.z;
        float sway_x = pos.x + ((wind_amp * sin(fmod(t_sec * 0.823529f, 6.2831855f) + (t_frac * 0.823529f) + (0.2f * pos.x)))
            * sin(fmod(t_sec * 0.893617f, 6.2831855f) + (t_frac * 0.893617f) + x_amp));
        float sway_z = pos.z + ((wind_amp * sin(fmod(t_sec * 0.792453f, 6.2831855f) + (t_frac * 0.792453f) + (0.2f * sway_x)))
            * sin(fmod(t_sec * 0.688525f, 6.2831855f) + (t_frac * 0.688525f) + x_amp));

        uint idx = uint(int(indir));
        float rot = radians(nani_instance_data.Load<float>(idx * 48u + 16));
        float3 fwd = normalize(float3(cos(rot), 0.0f, sin(rot)));
        float4x4 basis = transpose(float4x4(float4(fwd, 0.0f),
                                            float4(0.0f, 1.0f, 0.0f, 0.0f),
                                            float4(normalize(cross(fwd, float3(0.0f, 1.0f, 0.0f))), 0.0f),
                                            float4(0.0f, 0.0f, 0.0f, 1.0f)));
        float px = nani_instance_data.Load<float3>(idx * 48u + 0).x;
        float py = nani_instance_data.Load<float3>(idx * 48u + 0).y;
        float pz = nani_instance_data.Load<float3>(idx * 48u + 0).z;
        float scale = nani_instance_data.Load<float>(idx * 48u + 12);
        float4 prim_world = mul(float4x4(float4(1.0f, 0.0f, 0.0f, px),
                                         float4(0.0f, 1.0f, 0.0f, py),
                                         float4(0.0f, 0.0f, 1.0f, pz),
                                         float4(0.0f, 0.0f, 0.0f, 1.0f)),
                                mul(basis, mul(float4x4(float4(scale, 0.0f, 0.0f, 0.0f),
                                                       float4(0.0f, scale, 0.0f, 0.0f),
                                                       float4(0.0f, 0.0f, scale, 0.0f),
                                                       float4(0.0f, 0.0f, 0.0f, 1.0f)), float4(i.prim_center, 1.0f))));
        float3 prim_center_world = prim_world.xyz;
        float4 prim_view = mul(view_constants[current_view_index].world_to_view, float4(prim_world.x, prim_world.y, prim_world.z, 1.0f));
        if (length(prim_view.xyz) < (((render_overrides & 2u) != 0u) ? 5.0f : -5.0f))
        {
            wp = asfloat(0x7fbfffffu);
            canopy_pos = 0.0f;
            leaf_color = 0.0f;
            leaf_normal = 0.0f;
            world_pos = 0.0f;
            tree_id = 0;
            leaf_seed = 0;
            anim_high = 0.0f;
            prim_center = 0.0f;
            flower_bool = false;
            part_id = 0u;
            leaf_noise = 0.0f;
            clip_zw = 0.0f;
            depth_fade = 0.0f;
            break;
        }

        float4 cam4 = mul(view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
        float3 cam = cam4.xyz / cam4.w;
        float3 to_cam = prim_center_world - cam;
        float dist = length(to_cam);
        float facing = dot(mul(basis, float4(i.prim_normals, 0.0f)).xyz, to_cam * (1.0f / dist));
        float facing_falloff = facing - (abs(facing) * clamp((dist - 5.0f) * (-0.2f), 0.0f, 1.0f));
        if (facing_falloff >= 0.3f)
        {
            wp = asfloat(0x7fbfffffu);
            canopy_pos = 0.0f;
            leaf_color = 0.0f;
            leaf_normal = 0.0f;
            world_pos = 0.0f;
            tree_id = 0;
            leaf_seed = 0;
            anim_high = 0.0f;
            prim_center = 0.0f;
            flower_bool = false;
            part_id = 0u;
            leaf_noise = 0.0f;
            clip_zw = 0.0f;
            depth_fade = 0.0f;
            break;
        }
        float facing_blend = 1.0f - smoothstep(0.3f, -0.2f, facing_falloff);

        // Leaf placement (sway position through the second placement basis).
        float3 leaf_world = mul(float4x4(float4(1.0f, 0.0f, 0.0f, px),
                                         float4(0.0f, 1.0f, 0.0f, py),
                                         float4(0.0f, 0.0f, 1.0f, pz),
                                         float4(0.0f, 0.0f, 0.0f, 1.0f)),
                                mul(basis, mul(float4x4(float4(scale, 0.0f, 0.0f, 0.0f),
                                                       float4(0.0f, scale, 0.0f, 0.0f),
                                                       float4(0.0f, 0.0f, scale, 0.0f),
                                                       float4(0.0f, 0.0f, 0.0f, 1.0f)), float4(sway_x, pos.y, sway_z, 1.0f)))).xyz;

        float wind = (glade_theme_id == 4u) ? 0.4f : wind_strength;
        float3 leaf_dir = (float3(i.vertex_color.xy, 0.0f) * 2.0f) - 1.0f;
        float freq1 = 3.529416f * wind;
        float freq2 = 1.538462f * wind;
        float leaf_wind = (0.2f * sin(fmod(t_sec * freq1, 6.2831855f) + (t_frac * freq1) + sway_x)) * sin(fmod(t_sec * freq2, 6.2831855f) + (t_frac * freq2) + sway_z);
        float cosw = cos(leaf_wind);
        float sinw = sin(leaf_wind);
        float rand_angle = lerp(-0.5f, 0.5f, 0.3f * frac(52.982918f * frac(dot(float2(0.0671106f, 0.00583715f), prim_world.xz))));
        float3 view_axis = normalize(cam - prim_center_world);
        float sina = sin(rand_angle);
        float cosa = cos(rand_angle);
        float oca = 1.0f - cosa;
        float ax = view_axis.x;
        float ay = view_axis.y;
        float az = view_axis.z;
        float4x4 spin = transpose(float4x4(float4((oca * ax * ax) + cosa, (oca * ax * ay) - (az * sina), (oca * az * ax) + (ay * sina), 0.0f),
                                           float4((oca * ax * ay) + (az * sina), (oca * ay * ay) + cosa, (oca * ay * az) - (ax * sina), 0.0f),
                                           float4((oca * az * ax) - (ay * sina), (oca * ay * az) + (ax * sina), (oca * az * az) + cosa, 0.0f),
                                           float4(0.0f, 0.0f, 0.0f, 1.0f)));
        float grow_scaled = asfloat(nani_instance_data.Load<uint>(indir * 48u + 24));
        float3 billboard_pos = leaf_world
            + (mul(view_constants[current_view_index].view_to_world, float4(mul(float2x2(float2(cosw, sinw), float2(-sinw, cosw)), leaf_dir.xy), leaf_dir.z, 0.0f)).xyz * (grow_scaled * grow_f));
        float3 leaf_out = (mul(spin, float4(billboard_pos - prim_center_world, 1.0f)).xyz * ((1.0f - (facing_blend * facing_blend)) * lerp(0.8f, 1.0f, clamp(prim_view.z * (-0.05f), 0.0f, 1.0f)))) + prim_center_world;
        float2 bend = nani_instance_data.Load<float2>(indir * 48u + 36);
        float2 bent = leaf_out.xz + (bend * smoothstep(3.0f, 20.0f, i.vertex_position.y));
        float4 clip = mul(view_constants[current_view_index].world_to_sample, float4(bent.x, leaf_out.y, bent.y, 1.0f));
        float dist_plane = -view_constants[current_view_index].clip_to_view[3].z;
        float view_dist = (-1.0f) / ((clip.z / clip.w) * dist_plane);
        float depth_fade = smoothstep(1.6f, 3.2f, view_dist);
        float4 clip_final = clip;
        clip_final.z = (1.0f / ((-(view_dist - (1.6f * depth_fade))) * dist_plane)) * clip.w;

        int tree_id = int(uint(nani_instance_data.Load<int>(idx * 48u + 20)));
        int leaf_seed = int(10.0f * ((prim_world.x + prim_world.y) + prim_world.z));
        uint h1 = hash(uint(tree_id + 4));
        uint h2 = hash(uint(leaf_seed));
        float2 leaf_cell = nani_instance_data.Load<float3>(indir * 48u + 0).xz * 0.05f;
        int2 leaf_cell_int = int2(floor(leaf_cell));
        float2 leaf_cell_frac = frac(leaf_cell);
        float2 leaf_smooth = leaf_cell_frac * leaf_cell_frac * (3.0f - (2.0f * leaf_cell_frac));
        float leaf_noise = ((1.0f - leaf_smooth.y) * ((incb_perlin_hash01(hash(uint2(leaf_cell_int + int2(0, 0)))) * (1.0f - leaf_smooth.x)) + (incb_perlin_hash01(hash(uint2(leaf_cell_int + int2(1, 0)))) * leaf_smooth.x)))
            + ((incb_perlin_hash01(hash(uint2(leaf_cell_int + int2(0, 1)))) * (1.0f - leaf_smooth.x)) * leaf_smooth.y)
            + ((incb_perlin_hash01(hash(uint2(leaf_cell_int + int2(1, 1)))) * leaf_smooth.x) * leaf_smooth.y);

        wp = clip_final;
        canopy_pos = leaf_world;
        leaf_color = i.vertex_color;
        leaf_normal = mul(basis, float4(i.vertex_normal, 0.0f)).xyz;
        world_pos = float3(bent.x, leaf_out.y, bent.y);
        tree_id = tree_id;
        leaf_seed = leaf_seed;
        anim_high = f16tof32((nani_instance_data.Load<uint>(indir * 48u + 32) >> 16u) & 0xFFFFu);
        prim_center = prim_center_world;
        flower_bool = ((int(rand01(h1) < 0.4f)) & int(prim_world.y > 3.0f)) & int(rand01(h2) < 0.9f);
        part_id = uint(nani_instance_data.Load<int>(indir * 48u + 44));
        leaf_noise = leaf_noise;
        clip_zw = clip_final.zw;
        depth_fade = depth_fade;
        break;
    } while (false);
    o.position = wp;
    o.C0 = canopy_pos;
    o.C1 = leaf_color;
    o.C2 = leaf_normal;
    o.C3 = world_pos;
    o.C4 = tree_id;
    o.C5 = leaf_seed;
    o.C6 = anim_high;
    o.C7 = prim_center;
    o.C8 = uint(flower_bool);
    o.C9 = part_id;
    o.C10 = leaf_noise;
    o.C11 = clip_zw;
    o.C12 = depth_fade;
    return o;
}

struct PS_IN
{
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float3 C3 : TEXCOORD3;
    nointerpolation int C4 : TEXCOORD4;
    nointerpolation int C5 : TEXCOORD5;
    nointerpolation float C6 : TEXCOORD6;
    float3 C7 : TEXCOORD7;
    nointerpolation uint C8 : TEXCOORD8;
    nointerpolation uint C9 : TEXCOORD9;
    nointerpolation float C10 : TEXCOORD10;
    float C12 : TEXCOORD12;
};

struct PS_OUT
{
    uint2 target : SV_Target0;
    float depth : SV_DepthLessEqual;
};

PS_OUT ps_main(PS_IN i, float4 frag_coord : SV_Position)
{
    PS_OUT o;
    bool has_flower = (i.C8 != 0u);
    float3 flower_col;
    float flower_depth;
    float flower_alpha;
    if (has_flower)
    {
        float4 depth_tex = canopy_flowers_depth.SampleBias(sampler_llc, i.C1.xy, global_mip_bias);
        float fd = depth_tex.x;
        flower_col = canopy_flowers_color.SampleBias(sampler_llc, i.C1.xy, global_mip_bias).xyz * lerp(0.5f, 2.0f, fd);
        flower_depth = fd;
        flower_alpha = float(fd > 0.01f);
    }
    else
    {
        flower_col = 0.0f;
        flower_depth = 0.0f;
        flower_alpha = 0.0f;
    }
    float4 alpha_tex = canopy_alpha_texture.SampleBias(sampler_llc, i.C1.xy, global_mip_bias);
    float wind_cone = ((render_overrides & 2u) != 0u) ? i.C6 : 0.0f;
    float4 view4 = mul(view_constants[current_view_index].world_to_view, float4(i.C7, 1.0f));
    float2 cone_range = float2(6.66f, max_cone_cull_dist_range) * view_constants[0].view_to_clip[1].y;
    if ((wind_cone > 0.0f) && (is_firstperson_mode == 0))
    {
        if (-view4.z < cone_range.y)
        {
            float3 p = float3(view4.xy + (i.C1.xy - 0.5f), view4.z);
            p.z = min(view4.z, -10.0f);
            float cone = cone_angle_mult * smoothstep(cone_range.y, cone_range.x, -view4.z);
            if ((-normalize(p * float3(0.75f, 1.0f, 1.0f)).z) > cos((cone * wind_cone) + ((min(1.0f, cone * 16.0f) * 0.075f) * (blue_noise_texture.SampleLevel(sampler_llr, (i.C1.xy * 2.0f) * 4.0f * 0.00390625f, 0.0f).x - 0.5f))))
            {
                discard;
            }
        }
    }

    float alpha_threshold = has_flower ? (alpha_tex.y - 0.3f) : alpha_tex.y;
    bool flower_visible = has_flower && (flower_alpha > 0.0f);
    bool flower_blend = flower_visible && (i.C10 < 0.6f);
    float blend_depth = alpha_threshold;
    bool use_flower;
    if (flower_blend)
    {
        float flower_t = lerp(0.0f, 1.5f, rand01(hash(uint(i.C5))) * flower_depth);
        use_flower = flower_t > alpha_threshold;
        blend_depth = max(alpha_threshold, flower_t);
    }
    else
    {
        use_flower = false;
    }
    if ((use_flower ? 1.0f : alpha_tex.x) < 0.2f)
    {
        discard;
    }

    int detail_kind;
    bool theme24 = (glade_theme_id == 2u) || (glade_theme_id == 4u);
    if (theme24)
    {
        detail_kind = 1;
    }
    else
    {
        if (glade_theme_id == 1u)
        {
            bool part_test;
            if (i.C9 != 1u)
            {
                part_test = (i.C9 == 2u) || (i.C10 > 0.4f);
            }
            else
            {
                part_test = false;
            }
            detail_kind = part_test ? 2 : 0;
        }
        else
        {
            detail_kind = 0;
        }
    }

    float3 detail_color;
    if (detail_kind == 1)
    {
        uint h1 = hash(uint(i.C4 ^ 1128782));
        uint h2 = hash(uint(i.C4 ^ 872));
        uint h3 = hash(uint(i.C4 ^ 11282));
        uint h4 = hash(uint(i.C4 ^ 82));
        uint h5 = hash(uint(i.C5));
        float2 hash2 = float2(rand01(h1), rand01(h2));
        float2 hash3 = float2(rand01(h3), rand01(h4));
        float2 sum = i.C3.yz + i.C3.yx;
        float3 c1 = canopy_color.SampleBias(sampler_llr, hash2 + (i.C1.xy * 0.01f) + (sum * 0.0005f), global_mip_bias).xyz;
        float3 c2 = canopy_color.SampleBias(sampler_llr, hash3 + (i.C1.xy * 0.2f) + (sum * 0.05f), global_mip_bias).xyz * 0.5f;
        detail_color = lerp(c1, c2, lerp(0.1f, 0.5f, rand01(h5)));
    }
    else if (detail_kind == 2)
    {
        bool leaf_part = (i.C9 == 2u);
        uint h1 = hash(uint(i.C4 ^ 1128782));
        uint h2 = hash(uint(i.C4 ^ 872));
        float2 hash2 = float2(rand01(h1), rand01(h2));
        float2 sum = i.C1.xy * 0.2f;
        float2 detail_uv1 = hash2 + sum + (float2(i.C3.yz + i.C3.yx) * 0.005f);
        float2 uv1 = leaf_part ? float2(detail_uv1.x, frac(detail_uv1.y) * 0.6f) : detail_uv1;
        float3 c1 = canopy_color.SampleBias(sampler_llr, uv1, global_mip_bias).xyz;
        uint h3 = hash(uint(i.C4 ^ 11282));
        uint h4 = hash(uint(i.C4 ^ 82));
        float2 hash3 = float2(rand01(h3), rand01(h4));
        float2 detail_uv2 = hash3 + sum + (float2(i.C3.yz + i.C3.yx) * 0.05f);
        float2 uv2 = leaf_part ? float2(detail_uv2.x, frac(uv1.y) * 0.6f) : detail_uv2;
        uint h5 = hash(uint(i.C5 ^ 1128782));
        detail_color = lerp(c1, canopy_color.SampleBias(sampler_llr, uv2, global_mip_bias).xyz, smoothstep(0.0f, 1.0f, rand01(h5)));
    }
    else
    {
        uint h = hash(uint(i.C4 * 1128782));
        float3 c2 = (glade_theme_id == 3u) ? (canopy_color_2.xyz * 0.75f) : canopy_color_2.xyz;
        detail_color = lerp(canopy_color_1.xyz, c2, rand01(h));
    }

    float3 final_albedo = use_flower ? flower_col : detail_color;
    if (glade_theme_id == 4u)
    {
        final_albedo = lerp(final_albedo, 0.7f, smoothstep(0.0f, 1.0f, clamp(i.C2.y, 0.0f, 1.0f)));
    }
    float3 packed_n = (i.C2 * 0.5f) + 0.5f;
    float3 color = sqrt(clamp(final_albedo, 0.0f, 1.0f));
    uint normal_packed = (uint(clamp(packed_n.x, 0.0f, 1.0f) * 255.0f + 0.5f)
        + (uint(clamp(packed_n.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
        + (uint(clamp(packed_n.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u))
        + (uint(clamp(clamp(alpha_tex.z - 0.00390625f, 0.0f, 1.0f), 0.0f, 1.0f) * 255.0f + 0.5f) << 24u);
    uint albedo_packed = (uint(clamp(color.x, 0.0f, 1.0f) * 255.0f + 0.5f)
        + (uint(clamp(color.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
        + (uint(clamp(color.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u))
        + (uint(clamp(blend_depth, 0.0f, 1.0f) * 255.0f + 0.5f) << 24u);
    o.target = uint2(normal_packed, albedo_packed);
    o.depth = (frag_coord.z / frag_coord.w) / ((1.0f / frag_coord.w) + ((i.C12 * (1.0f - blend_depth)) * 1.6f));
    return o;
}
