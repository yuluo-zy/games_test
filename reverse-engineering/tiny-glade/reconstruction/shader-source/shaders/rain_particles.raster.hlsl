// ==RECONSTRUCTED== shaders/rain_particles.raster.hlsl | F-V | bins: _rain_particles.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Reconstructed from inlined vs/ps opcode patterns (spirv-cross scaffolding + target fingerprint).
// Model: rain streak quads. Per-instance seed hashed (inc/hash.hlsl 5-step hash + rand01); a
// streak falls from a random xz offset at speed scaled by rain_strength; rain bends along the
// world +x axis by strength*0.04; ps outputs white streak with vertical-fade opacity.
// Globals model_to_world + rain_strength are non-static globals -> land in $Globals (member 1, 2).

#include "inc/common.hlsl"          // samplers (unused here; dxc drops them)
#include "inc/tracked_buffer.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/math_const.hlsl"
#include "inc/current_view.hlsl"    // declares current_view_index global
#include "inc/ray_cone.hlsl"

// INC-issue workaround: the original samplers.hlsl declared 13 samplers
// (s0..s12); inc/samplers.hlsl only carries s0..s4,s9..s12. Declaring the
// unused s5..s8 here reproduces the original $Globals binding (set0/b13).

float4x4 model_to_world;
float rain_strength;

struct VsIn
{
    float3 Vertex_Position : TEXCOORD0;
    int id : TEXCOORD1;
};

struct VsOut
{
    float3 C0 : TEXCOORD0;
    float C1 : TEXCOORD1;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;

    float fall = lerp(0.6f, 1.2f, rand01(hash(uint(vin.id ^ 5678))));
    float3 offset = float3(
        lerp(0.0f, 60.0f, rand01(hash(uint(vin.id ^ 45845)))),
        lerp(0.0f, 1.0f, rand01(hash(uint(vin.id ^ 35934)))),
        lerp(0.0f, 60.0f, rand01(hash(uint(vin.id ^ 23475)))));

    offset.y = frac(offset.y - ((float(game_time_sec) + game_time_frac) * 0.5f) * fall) * 50.0f;

    float3 world = float3(0.0f, 1.0f, 0.0f) * vin.Vertex_Position.y
                 + normalize(mul(view_constants[current_view_index].view_to_world, float4(1.0f, 0.0f, 0.0f, 0.0f)).xyz)
                   * vin.Vertex_Position.x * (0.04f * rain_strength)
                 + offset + float3(-30.0f, 0.0f, -30.0f);

    o.position = mul(view_constants[current_view_index].world_to_sample, float4(world, 1.0f));
    o.C0 = float3(1.0f, 1.0f, 1.0f);
    o.C1 = lerp(0.3f, 0.0f, vin.Vertex_Position.y + 0.5f) * pow(rain_strength, 0.5f);
    return o;
}

struct PsIn
{
    float3 C0 : TEXCOORD0;
    float C1 : TEXCOORD1;
};

struct PsOut
{
    float4 color : SV_Target0;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;
    o.color = float4(pin.C0 * pin.C1, pin.C1);
    return o;
}
