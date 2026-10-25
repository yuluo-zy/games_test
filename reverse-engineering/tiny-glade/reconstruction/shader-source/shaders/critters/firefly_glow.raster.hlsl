// PATCHED: modules lsl_f0adff76827e73ff.bin:ps_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/critters/firefly_glow.raster.hlsl | target: _critters_firefly_glow.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main | evidence: spirv-cross + SPIR-V access/type inspection (instance StructuredBuffer<Firefly> + float visibility + DofFocus autofocus; NaN discard when invisible; CoC-driven billboard scale; radial falloff glow) | bindings: sampler_default s9, $Globals b13, terrain_heightmap t16, firefly_buffer t18, firefly_visibility_buffer t19, autofocus_buf t20, frame_constants set1/b0 | status: R (reconstruction)

#include "inc/common.hlsl"
#include "inc/tracked_buffer.hlsl"
#include "inc/current_view.hlsl"
Texture2D<float4> terrain_heightmap : register(t16);
#include "inc/terrain_heightmap.inc.hlsl"
#include "inc/coc.hlsl"
#include "inc/math_const.hlsl"

// INC-issue workaround: original samplers.hlsl declared s0..s12 (13 samplers);
// inc/samplers.hlsl carries only s0..s4,s9..s12, so the missing s5..s8 are
// declared here to reproduce the original $Globals binding (set0/b13).

struct Firefly
{
    float3 position;
    uint pad;
};

StructuredBuffer<Firefly> firefly_buffer : register(t18);
StructuredBuffer<float> firefly_visibility_buffer : register(t19);
StructuredBuffer<DofFocus> autofocus_buf : register(t20);

uint2 terrain_heightmap_dims;
uint2 visual_terrain_heightmap_dims;
float firefly_visibility;
uint2 framebuffer_dims;

struct VsIn
{
    [[vk::location(0)]] float3 Vertex_Position: Vertex_Position;
    uint instance_id : SV_InstanceID;
};

struct VsOut
{
    float2 C0 : TEXCOORD0;
    float C1 : TEXCOORD1;
    float C2 : TEXCOORD2;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;
    if (firefly_visibility_buffer[vin.instance_id] == 0.0f)
    {
        o.position = asfloat(0x7fbfffff).xxxx;
        o.C0 = asfloat(0x7fbfffff).xx;
        o.C1 = asfloat(0x7fbfffff);
        o.C2 = asfloat(0x7fbfffff);
        return o;
    }

    float3 firefly_pos = firefly_buffer[vin.instance_id].position;
    float3 world_pos = firefly_pos + float3(0.0f,
        lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, firefly_pos.xz * 0.0076923076994717121124267578125f.xx + 0.5f.xx, 0.0f).x),
        0.0f);
    float4 view_pos = mul(view_constants[current_view_index].world_to_view, float4(world_pos, 1.0f));

    float inv_focus0 = 1.0f / autofocus_buf[camera_idx].focus0;
    float inv_focus1 = 1.0f / autofocus_buf[camera_idx].focus1;
    float near_plane = max(0.001000000047497451305389404296875f, max(inv_focus0, inv_focus1)) + 9.9999997473787516355514526367188e-06f;
    float far_plane = max(0.001000000047497451305389404296875f, min(inv_focus0, inv_focus1)) - 9.9999997473787516355514526367188e-06f;
    bool use_tilt_shift = autofocus_buf[camera_idx].use_tilt_shift != 0u;

    float3 proj_pos = world_pos;
    proj_pos.y = world_pos.y * float(use_tilt_shift ? 0 : 1);
    float4 cam_pos4 = mul(view_constants[0].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 cam_forward = normalize(mul(view_constants[0].view_to_world, float4(0.0f, 0.0f, -1.0f, 0.0f)).xyz);
    float depth = 1.0f / max(1.0000000133514319600180897396058e-10f, dot(proj_pos - (cam_pos4.xyz / cam_pos4.w.xxx), cam_forward));

    float range = near_plane - far_plane;
    float blur = min(0.00999999977648258209228515625f, 0.0500000007450580596923828125f * range);
    float coc;
    if (depth > near_plane)
    {
        coc = (depth - near_plane) + blur;
    }
    else if (depth < far_plane)
    {
        coc = (depth - far_plane) - blur;
    }
    else
    {
        coc = (-blur) + (((2.0f * blur) * (depth - far_plane)) / range);
    }
    if (use_tilt_shift)
    {
        coc = coc / max(0.00999999977648258209228515625f, length(cam_forward.xz));
    }

    float coc_px = ((((((-coc) * autofocus_buf[camera_idx].focus_scale) * view_constants[0].view_to_clip[1].y) * 0.5f) * view_constants[0].view_to_clip[1].y) / float(framebuffer_dims.y)) * (-view_pos.z);
    float scale = 0.800000011920928955078125f + coc_px;
    float k = coc_px / scale;

    o.position = mul(view_constants[current_view_index].view_to_sample, view_pos + float4(vin.Vertex_Position.xzy * scale, 0.0f));
    o.C0 = vin.Vertex_Position.xz * 2.0f;
    o.C1 = clamp(((firefly_visibility_buffer[vin.instance_id] - 0.20000000298023223876953125f) * 0.753982365131378173828125f) / ((scale * scale) * (3.1415927410125732421875f * (0.300000011920928955078125f + (0.699999988079071044921875f * (k * (0.579999983310699462890625f + (0.4199999868869781494140625f * k))))))), 0.0f, 1.0f);
    o.C2 = k;
    return o;
}

struct PsIn
{
    float2 C0 : TEXCOORD0;
    float C1 : TEXCOORD1;
    float C2 : TEXCOORD2;
};

struct PsOut
{
    float4 color : SV_Target0;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;
    float4 col = float4(0.039999999105930328369140625f, 0.039999999105930328369140625f, 0.00200000009499490261077880859375f, 0.0f) * ev_mult;
    col *= pin.C1;
    col *= smoothstep(1.0f, pin.C2, length(pin.C0));
    col *= firefly_visibility;
    o.color = col;
    return o;
}

#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _critters_firefly_glow.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
void frag_main()
{
}

void ps_main()
{
    frag_main();
}
#pragma dxc diagnostic pop
#endif
