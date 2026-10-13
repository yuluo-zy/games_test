// ==RECONSTRUCTED== shaders/snow_particles.raster.hlsl | F-V | bins: _snow_particles.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Snow particles: camera-facing quads in a 100-unit repeating volume around the
// active camera. vs_main rejects (NaN position, undefined attributes) in topdown
// mode and when the particle is too close / outside the focus range; ps dithers
// with a blue-noise hash, then packs the solid gbuffer.

#include "inc/common.hlsl"
#include "inc/tracked_buffer.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/math_const.hlsl"
#include "inc/current_view.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/coc.hlsl"
#include "inc/color_library.hlsl"
#include "inc/material/solid_material.inc.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/material_eval.inc.hlsl"
#include "inc/brdf.hlsl"
#include "inc/math.hlsl"
#include "inc/lighting.hlsl"
#include "inc/sun.inc.hlsl"
#include "inc/color/srgb.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/sh.hlsl"

// INC-issue workaround: the original common.hlsl declared 13 samplers (s0..s12);
// inc/samplers.hlsl only carries s0..s4,s9..s12. Declaring the unused s5..s8 here
// reproduces the original $Globals binding (set0/b13).

// Per-camera autofocus data (DofFocus record, 64-byte stride) from inc/coc.hlsl.
StructuredBuffer<DofFocus> autofocus_buf : register(t14);

// Accessor helpers matching the target's inlined access-chain structure.
DofFocus get_autofocus(uint index)
{
    return autofocus_buf[index];
}

ViewConstants get_camera0()
{
    return view_constants[0];
}

float4x4 model_to_world;
float snow_strength;
int is_photomode;
int is_topdown_mode;

// Camera-distance cutoff: 15m normally, 0 in photomode (never cull).
float snow_camera_cutoff()
{
    if (is_photomode != 0)
    {
        return 0.0f;
    }
    else
    {
        return 15.0f;
    }
}

struct VsIn
{
    uint vertex_id : SV_VertexID;
    uint instance_id : SV_InstanceID;
};

struct VsOut
{
    float2 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    nointerpolation uint C2 : TEXCOORD2;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;
    if (is_topdown_mode == 1)
    {
        o.position = asfloat(0x7FBFFFFFu).xxxx;
        return o;
    }

    float2 quad_uv = float2(float(vin.vertex_id & 1u), float((vin.vertex_id >> 1u) & 1u)) - 0.5f;
    int id = int(vin.instance_id);

    float fall = lerp(0.3f, 1.2f, rand01(hash(uint(id ^ 5678))));
    float3 offset = float3(rand01(hash(uint(id ^ 23475))), rand01(hash(uint(id ^ 35934))), rand01(hash(uint(id ^ 45845)))) * 100.0f;
    float drift = ((float(game_time_sec) + game_time_frac) * 2.0f) * fall;
    offset.y -= drift;
    offset.x -= drift;

    // Camera-0 basis for the depth range / tilt-shift math.
    float4 cam0_vw = mul(get_camera0().view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 cam0_pos = cam0_vw.xyz / cam0_vw.w.xxx;
    float3 cam0_fwd = normalize(mul(get_camera0().view_to_world, float4(0.0f, 0.0f, -1.0f, 0.0f)).xyz);

    float dof_scale = 5.0f / (get_autofocus(camera_idx).focus_scale * get_camera0().view_to_clip[1].y);
    bool tilt = get_autofocus(camera_idx).use_tilt_shift != 0u;
    float dof_far = dof_scale;
    if (tilt)
    {
        dof_far = dof_scale * max(0.01f, length(cam0_fwd.xz));
    }

    float inv_f0 = 1.0f / get_autofocus(camera_idx).focus0;
    float inv_f1 = 1.0f / get_autofocus(camera_idx).focus1;
    float focus_half = max(0.001f, max(inv_f0, inv_f1)) + 9.9999997473787516e-06f;
    float focus_near = max(0.001f, min(inv_f0, inv_f1)) - 9.9999997473787516e-06f;
    float dof_focus = 1.0f / ((dof_far + focus_half) - min(0.01f, 0.05f * (focus_half - focus_near)));
    float focus_dist = dof_focus;
    if (tilt)
    {
        focus_dist = (dof_focus + (cam0_pos.y * cam0_fwd.y)) / ((cam0_fwd.x * cam0_fwd.x) + (cam0_fwd.z * cam0_fwd.z));
    }

    // Current-camera facing volume (100 unit box) the particles wrap into.
    float4 cur_vw = mul(GetCurrentViewConstants().view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 cam_pos = cur_vw.xyz / cur_vw.w.xxx;
    float3 cam_fwd = normalize(mul(GetCurrentViewConstants().view_to_world, float4(0.0f, 0.0f, -1.0f, 0.0f)).xyz);
    float3 box_min = cam_pos + (cam_fwd * (focus_dist + 25.0f)) - 50.0f;
    float3 box_size = (cam_pos + (cam_fwd * (focus_dist + 25.0f)) + 50.0f) - box_min;
    float3 world = frac((offset - box_min) / box_size) * box_size + box_min;

    if (distance(world, cam_pos) < snow_camera_cutoff())
    {
        o.position = asfloat(0x7FBFFFFFu).xxxx;
        return o;
    }

    inv_f0 = 1.0f / get_autofocus(camera_idx).focus0;
    inv_f1 = 1.0f / get_autofocus(camera_idx).focus1;
    float near_plane = max(0.001f, max(inv_f0, inv_f1)) + 9.9999997473787516e-06f;
    float far_plane = max(0.001f, min(inv_f0, inv_f1)) - 9.9999997473787516e-06f;
    tilt = get_autofocus(camera_idx).use_tilt_shift != 0u;
    float3 depth_pos = world;
    depth_pos.y = world.y * float(tilt ? 0 : 1);
    float depth = 1.0f / max(1.00000001335143196e-10f, dot(depth_pos - cam0_pos, cam0_fwd));
    float plane_range = near_plane - far_plane;
    float edge_w = min(0.01f, 0.05f * plane_range);
    float coc;
    if (depth > near_plane)
    {
        coc = (depth - near_plane) + edge_w;
    }
    else if (depth < far_plane)
    {
        coc = (depth - far_plane) - edge_w;
    }
    else
    {
        coc = (-edge_w) + ((2.0f * edge_w * (depth - far_plane)) / plane_range);
    }
    float coc_world = coc;
    if (tilt)
    {
        coc_world = coc / max(0.01f, length(cam0_fwd.xz));
    }

    if (abs(((-coc_world) * get_autofocus(camera_idx).focus_scale) * get_camera0().view_to_clip[1].y) > 20.0f)
    {
        o.position = asfloat(0x7FBFFFFFu).xxxx;
        return o;
    }

    float scale = 0.16f * snow_strength;
    float3 right = normalize(mul(GetCurrentViewConstants().view_to_world, float4(1.0f, 0.0f, 0.0f, 0.0f)).xyz);
    float3 up = normalize(mul(GetCurrentViewConstants().view_to_world, float4(0.0f, 1.0f, 0.0f, 0.0f)).xyz);
    float3 quad = (up * quad_uv.y * scale) + (right * quad_uv.x * scale) + world;

    o.position = mul(GetCurrentViewConstants().world_to_sample, float4(quad, 1.0f));
    o.C0 = quad_uv;
    o.C1 = quad;
    o.C2 = vin.instance_id;
    return o;
}

struct PsIn
{
    float2 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float4 position : SV_Position;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;
    float3 world_pos = pin.C1;
    float inv_f0 = 1.0f / autofocus_buf[camera_idx].focus0;
    float inv_f1 = 1.0f / autofocus_buf[camera_idx].focus1;
    float near_plane = max(0.001f, max(inv_f0, inv_f1)) + 9.9999997473787516e-06f;
    float far_plane = max(0.001f, min(inv_f0, inv_f1)) - 9.9999997473787516e-06f;
    bool tilt = autofocus_buf[camera_idx].use_tilt_shift != 0u;
    float3 depth_pos = world_pos;
    depth_pos.y = world_pos.y * float(tilt ? 0 : 1);
    float4 cam0_vw = mul(view_constants[0].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 cam0_pos = cam0_vw.xyz / cam0_vw.w.xxx;
    float3 cam0_fwd = normalize(mul(view_constants[0].view_to_world, float4(0.0f, 0.0f, -1.0f, 0.0f)).xyz);
    float depth = 1.0f / max(1.00000001335143196e-10f, dot(depth_pos - cam0_pos, cam0_fwd));
    float plane_range = near_plane - far_plane;
    float edge_w = min(0.01f, 0.05f * plane_range);
    float coc;
    if (depth > near_plane)
    {
        coc = (depth - near_plane) + edge_w;
    }
    else if (depth < far_plane)
    {
        coc = (depth - far_plane) - edge_w;
    }
    else
    {
        coc = (-edge_w) + ((2.0f * edge_w * (depth - far_plane)) / plane_range);
    }
    float coc_world = coc;
    if (tilt)
    {
        coc_world = coc / max(0.01f, length(cam0_fwd.xz));
    }

    float4 cur_cam_vw = mul(view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 cur_cam_pos = cur_cam_vw.xyz / cur_cam_vw.w.xxx;

    float cutoff = snow_camera_cutoff();
    uint2 pixel = uint2(pin.position.xy);
    float dither = frac(52.98291778564453125f * frac((0.0671105608344078f * float(pixel.x)) + (0.0058371499180793762f * float(pixel.y))));
    if (smoothstep(0.75f, 1.0f, max(abs(((-coc_world) * autofocus_buf[camera_idx].focus_scale) * view_constants[0].view_to_clip[1].y) * 0.05f, 2.0f - (distance(world_pos, cur_cam_pos) / cutoff))) > dither)
    {
        discard;
    }
    if (length(pin.C0) > 0.5f)
    {
        discard;
    }

    float3 snow_normal = (mul(view_constants[2].view_to_world, float4(0.0f, 0.0f, 1.0f, 0.0f)).xyz * 0.5f) + 0.5f;
    uint normal_word = uint(clamp(snow_normal.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(snow_normal.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(snow_normal.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
                     + 4278190080u;
    o.gbuffer = uint2(normal_word, 14013909u);
    o.gbuffer.y = 1188419029u;
    return o;
}
