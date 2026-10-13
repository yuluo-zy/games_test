// ==RECONSTRUCTED== Instanced ivy branches. Each IvySegment is instanced as a quad strip; the
// SOURCE PATH : shaders/ivy/instanced_ivy_branch.raster.hlsl
// KIND        : raster_main
// COMPILED AS MAIN IN (2 bins):
//   _ivy_instanced_ivy_branch.raster.hlsl_b903e43ffb3da915.bin
//     file_sha256  : a5b102c2f95b0270b88f4731273a29b1fc35f053d1114819e1ad5573ab8444d3
//     pass/stage   : _ivy_instanced_ivy_branch (vegetation_grass) / raster
//     entry        : vs:vs_main; ps:ps_main
//     variant      : b903e43ffb3da915
//     local_size   : -
//     names        : sampler_llb, sampler_default, ViewConstants, frame_constants, $Globals, IvySegment, ivy_buffer, terrain_heightmap, path_mask, vs_main, sampler_llr, ivy_branch_color
//   _ivy_instanced_ivy_branch.raster.hlsl_f0adff76827e73ff.bin
//     file_sha256  : 1648414082036e0da543cb7944b2a10f2fb90a18a6b0965f9f95e074400bfc61
//     pass/stage   : _ivy_instanced_ivy_branch (vegetation_grass) / raster
//     entry        : vs:vs_main; ps:ps_main
//     variant      : f0adff76827e73ff
//     local_size   : -
//     names        : sampler_llb, sampler_default, ViewConstants, frame_constants, $Globals, IvySegment, ivy_buffer, terrain_heightmap, path_mask, vs_main, ps_main
// ==RECONSTRUCTED==
// Instanced ivy branches. Each IvySegment is instanced as a quad strip; the
// vertex is positioned along the segment (start/end lifted to terrain height),
// width-scaled by the segment thickness, and displaced by 3D value noise
// (arch noise) scaled by a path-mask fade. Vertices inside the path-mask fade
// shape are culled by emitting a NaN position. The fragment writes the packed
// world normal and a sqrt'd branch color to a uint2 target.

#include "inc/common.hlsl"
#include "inc/current_view.hlsl"
#include "arch_noise_discard.inc.hlsl"

// Reproduce the original shared 13-sampler $Globals layout (set0/b13).

// Per-shader $Globals members (implicit $Globals UBO, set0/b13).
float4x4 model_to_world;
uint2 ivy_branch_color_dims;
uint2 terrain_heightmap_dims;
uint2 visual_terrain_heightmap_dims;
uint2 path_mask_dims;

struct IvySegment
{
    float3 start_pos;
    float start_thickness;
    float3 end_pos;
    float end_thickness;
    float3 wall_normal;
    float arch_height;
};

StructuredBuffer<IvySegment> ivy_buffer : register(t14, space0);
Texture2D<float4> terrain_heightmap : register(t16, space0);
Texture2D<float4> path_mask : register(t18, space0);
Texture2D<float4> ivy_branch_color : register(t15, space0);

struct VSInput
{
    float3 position : TEXCOORD0;
    uint instance : SV_InstanceID;
};

struct VSOutput
{
    float4 position : SV_Position;
    float3 normal : TEXCOORD1;
    float2 uv : TEXCOORD2;
    float3 world_pos : TEXCOORD3;
};

VSOutput vs_main(VSInput input)
{
    VSOutput output;
    float4 position;
    float3 normal;
    float2 uv;
    float3 world_pos;
    switch (0u)
    {
    default:
        float3 start = ivy_buffer[input.instance].start_pos;
        float3 end = ivy_buffer[input.instance].end_pos;
        float2 start_uv = start.xz * (0.0076923076994717121124267578125f).xx + 0.5f.xx;
        float hstart = lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, start_uv, 0.0f).x);
        float2 end_uv = end.xz * (0.0076923076994717121124267578125f).xx + 0.5f.xx;
        float hend = lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, end_uv, 0.0f).x);
        float start_thick = ivy_buffer[input.instance].start_thickness;
        float end_thick = ivy_buffer[input.instance].end_thickness;
        float3 wall_normal = ivy_buffer[input.instance].wall_normal;
        float arch_height = ivy_buffer[input.instance].arch_height;
        float base = arch_height - min(hstart, 0.0f);
        float path = path_mask.SampleLevel(sampler_llb, start_uv, 0.0f).x;
        bool over_path = path > 0.1f;

        float t0, t1;
        if (over_path)
        {
            float fade = (1.0f - exp(-5.0f * path)) * base;
            float n = perlin(start.xz * 0.6f + start.y * 0.6f);
            float t = clamp(base - start.y, 0.0f, 1.0f);
            float tt = t * t;
            float d = fade * (tt * clamp(smoothstep(0.0f, 0.3f, n), 0.8f, 1.0f));
            if (start.y < d)
            {
                position = asfloat(0x7fbfffffu).xxxx;
                normal = 0.0f.xxx;
                uv = 0.0f.xx;
                world_pos = 0.0f.xxx;
                break;
            }
            else if (start.y < fade)
            {
                float k = 1.0f - tt;
                t0 = min(start_thick * k, 0.2f);
                t1 = min(end_thick * k, 0.2f);
            }
            else
            {
                t0 = start_thick;
                t1 = end_thick;
            }
        }
        else
        {
            t0 = start_thick;
            t1 = end_thick;
        }

        float3 s = start;
        s.y = start.y + hstart;
        float3 e = end;
        e.y = end.y + hend;
        float3 dir = normalize(e - s);
        float3 side = normalize(cross(dir, wall_normal));
        float3 vpos;
        if (input.position.y < 0.1f)
        {
            float2 sc = input.position.xz * lerp(0.01679999940097332000732421875f, 0.083999998867511749267578125f, t0);
            vpos = float3(sc.x, input.position.y, sc.y);
        }
        else
        {
            float2 sc = input.position.xz * lerp(0.01679999940097332000732421875f, 0.083999998867511749267578125f, t1);
            vpos = float3(sc.x, input.position.y, sc.y);
        }
        float4x4 rot = transpose(float4x4(float4(side, 0.0f), float4(dir, 0.0f),
                                          float4(normalize(cross(side, dir)), 0.0f),
                                          float4(0.0f, 0.0f, 0.0f, 1.0f)));
        float4x4 trans = float4x4(float4(1.0f, 0.0f, 0.0f, start.x),
                                  float4(0.0f, 1.0f, 0.0f, s.y),
                                  float4(0.0f, 0.0f, 1.0f, start.z),
                                  float4(0.0f, 0.0f, 0.0f, 1.0f));
        float3 local = mul(trans, mul(rot, float4(vpos.x, vpos.y * (distance(s, e) * 1.1f), vpos.z, 1.0f))).xyz;
        uint alt_seed;
        float2 disp = clamp((perlin3(local * 0.4f, over_path, alt_seed) - 0.5f) * 2.5f, -0.5f, 0.5f);
        float3 world = mul(model_to_world, float4(local + (float3(disp.x, 0.0f, disp.y) * 0.06f), 1.0f)).xyz;
        position = mul(view_constants[current_view_index].world_to_sample, float4(world, 1.0f));
        normal = mul(rot, float4(normalize(float3(input.position.x, 0.0f, input.position.z)), 0.0f)).xyz;
        uv = float2(end.x + e.y * 0.5f, end.z + e.y * 0.5f) * 0.2f;
        world_pos = world;
        break;
    }
    output.position = position;
    output.normal = normal;
    output.uv = uv;
    output.world_pos = world_pos;
    return output;
}

struct PSInput
{
    float3 normal : TEXCOORD1;
    float2 uv : TEXCOORD2;
    float3 world_pos : TEXCOORD3;
};

uint2 ps_main(PSInput input) : SV_Target0
{
    float3 color = ivy_branch_color.Sample(sampler_llr, input.uv).xyz;
    float3 c;
    if (glade_theme_id == 4u)
    {
        float fade = smoothstep(-0.1f, 0.0f, input.world_pos.y) * smoothstep(0.0f, 0.7f, clamp(input.normal.y, 0.0f, 1.0f));
        c = lerp(lerp(color, 0.7f.xxx, 0.2f.xxx), 0.7f.xxx, fade.xxx);
    }
    else
    {
        c = color;
    }
    float3 packed_normal = input.normal * 0.5f + 0.5f.xxx;
    uint n = uint(clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f + 0.5f)
           + (uint(clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
           + (uint(clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
           + 4278190080u;
    float3 sc = sqrt(clamp(c, 0.0f.xxx, 1.0f.xxx));
    uint m = uint(clamp(sc.x, 0.0f, 1.0f) * 255.0f + 0.5f)
           + (uint(clamp(sc.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
           + (uint(clamp(sc.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
           + 436207616u;
    return uint2(n, m);
}
