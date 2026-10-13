// ==RECONSTRUCTED== shaders/sparkles.raster.hlsl | F-V | bins: _sparkles.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Sparkle particle instances (critters_particles). Per-instance StructuredBuffer
// (position/scale/time_offset/speed_mult/color_id). Each sparkle pulses on a
// time phase: the quad is scaled by a bump envelope, shifted along +y by the
// phase, and jittered by an Ashima 2D simplex displacement (13.0 amplitude,
// inlined; see also inc/simplex_noise.hlsl). color_id picks a palette color
// (0 = vertex color, 1..4 = baked HDR color ids, else black). The ps forwards
// color + the current-vs-previous displacement delta to the swapchain targets.
// $Globals (set0/b13) holds only current_view_index.

#include "inc/common.hlsl"
#include "inc/current_view.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/math_const.hlsl"

// INC-issue workaround: the original samplers.hlsl declared 13 samplers
// (s0..s12); inc/samplers.hlsl carries only s0..s4,s9..s12. Declaring s5..s8
// here restores the 13 consecutive sampler slots that position $Globals at
// set0/b13.

struct SparkleInstanceData
{
    float3 position;
    float scale;
    float time_offset;
    float speed_mult;
    int color_id;
    float pad1;
};

StructuredBuffer<SparkleInstanceData> instanced_data : register(t14);

// Ashima-style 2D simplex noise, inlined exactly as compiled in the original
// (float3 permute path, 13.0 amplitude, no 130-scale like inc/simplex_noise.hlsl).
float sparkle_noise(float2 v)
{
    const float F2 = 0.366025403784439f;
    const float G2 = 0.211324865405187f;
    float2 i = floor(v + dot(v, F2.xx).xx);
    float2 x0 = (v - i) + dot(i, G2.xx).xx;
    float x0x = x0.x;
    float x0y = x0.y;
    bool2 ge = (x0x > x0y).xx;
    float2 i1 = select(ge, float2(1.0f, 0.0f), float2(0.0f, 1.0f));

    float4 x1x2 = x0.xyxy + float4(G2, G2, -0.577350269189626f, -0.577350269189626f);
    float2 x1 = x1x2.xy - i1;
    float4 x1x2b = float4(x1.x, x1.y, x1x2.z, x1x2.w);
    float2 x2 = x1x2.zw;

    float2 im = fmod(i, 289.0f);
    float3 p0 = im.y.xxx + float3(0.0f, i1.y, 1.0f);
    float3 p1 = (fmod((p0 * 34.0f + 1.0f) * p0, 289.0f) + im.x.xxx) + float3(0.0f, i1.x, 1.0f);

    float3 m = max(0.5f - float3(dot(x0, x0), dot(x1, x1), dot(x2, x2)), 0.0f);
    m = m * m;
    m = m * m;

    float3 n2 = frac(fmod((p1 * 34.0f + 1.0f) * p1, 289.0f) * (1.0f / 41.0f)) * 2.0f;
    float3 n = n2 - 1.0f;
    float3 s = abs(n) - 0.5f;
    float3 t = n - floor(n2 - 0.5f);

    float3 grad;
    grad.x = t.x * x0x + s.x * x0y;
    float2 gradyz = (t.yz * x1x2b.xz) + (s.yz * x1x2b.yw);
    return dot(m * (1.79284291400159f - ((t * t + s * s) * 0.85373472095314f)), float3(grad.x, gradyz)) * 13.0f;
}

struct VsIn
{
    [[vk::location(0)]] float3 Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 Vertex_Color: Vertex_Color;
    [[vk::location(2)]] float3 alt_pos: alt_pos;
    uint id : SV_InstanceID;
};

struct VsOut
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;

    // Current-frame phase and base quad position.
    float speed_mult = instanced_data[vin.id].speed_mult;
    float time_offset = instanced_data[vin.id].time_offset;
    float speed = (1.0f + speed_mult) * 0.7f;
    float phase = frac(((fmod(float(game_time_sec) * speed, TWO_PI) + (game_time_frac * speed)) * INV_TWO_PI) + time_offset);
    bool3 big = (time_offset * speed_mult > 0.4f).xxx;
    float pulse = smoothstep(0.0f, 0.6f, phase) * (1.0f - smoothstep(0.6f, 1.0f, phase));
    float3 base = select(big, vin.alt_pos, vin.Vertex_Position)
        * ((1.0f - pow(1.0f - pulse, 3.0f)) * (0.18f + instanced_data[vin.id].scale * 0.1f))
        + instanced_data[vin.id].position + float3(0.0f, phase * 1.4f - 0.5f, 0.0f);
    float3 displaced = base + (sparkle_noise(((base.xz + base.xy) + base.yz) * 0.6f)).xxx;

    // Previous-frame phase (time shifted back by delta_time) and base position.
    float prev_speed_mult = instanced_data[vin.id].speed_mult;
    float prev_time_offset = instanced_data[vin.id].time_offset;
    float prev_speed = (1.0f + prev_speed_mult) * 0.7f;
    float prev_phase = frac((((fmod(float(game_time_sec) * prev_speed, TWO_PI) + (game_time_frac * prev_speed))
        + (-delta_time_seconds * prev_speed)) * INV_TWO_PI) + prev_time_offset);
    bool3 prev_big = (prev_time_offset * prev_speed_mult > 0.4f).xxx;
    float prev_pulse = smoothstep(0.0f, 0.6f, prev_phase) * (1.0f - smoothstep(0.6f, 1.0f, prev_phase));
    float3 prev_base = select(prev_big, vin.alt_pos, vin.Vertex_Position)
        * ((1.0f - pow(1.0f - prev_pulse, 3.0f)) * (0.18f + instanced_data[vin.id].scale * 0.1f))
        + instanced_data[vin.id].position + float3(0.0f, prev_phase * 1.4f - 0.5f, 0.0f);
    float3 prev_displaced = prev_base + (sparkle_noise(((prev_base.xz + prev_base.xy) + prev_base.yz) * 0.6f)).xxx;

    // Palette selection by color_id.
    int color_id = instanced_data[vin.id].color_id;
    float3 color;
    if (color_id == 0)
    {
        color = vin.Vertex_Color;
    }
    else if (color_id == 1)
    {
        color = float3(0.0f, 0.75f, 10.0f);
    }
    else if (color_id == 2)
    {
        color = float3(2.0f, 1.0f, 8.0f);
    }
    else if (color_id == 3)
    {
        color = float3(8.0f, 3.0f, 0.0f);
    }
    else
    {
        color = select(color_id == 4, float3(2.0f, 9.0f, 2.0f), float3(0.0f, 0.0f, 0.0f));
    }

    o.position = mul(view_constants[current_view_index].world_to_sample, float4(displaced, 1.0f));
    o.C0 = color;
    o.C1 = displaced - prev_displaced;
    return o;
}

struct PsIn
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
};

struct PsOut
{
    float4 color : SV_Target0;
    float3 delta : SV_Target1;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;
    o.color = float4(pin.C0, 1.0f);
    o.delta = pin.C1;
    return o;
}
