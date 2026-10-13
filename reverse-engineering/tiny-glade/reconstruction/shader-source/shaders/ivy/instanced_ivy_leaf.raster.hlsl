// ==RECONSTRUCTED== Instanced ivy leaves. Each IvyLeaf instance orients a leaf quad along its
// SOURCE PATH : shaders/ivy/instanced_ivy_leaf.raster.hlsl
// KIND        : raster_main
// COMPILED AS MAIN IN (2 bins):
//   _ivy_instanced_ivy_leaf.raster.hlsl_b903e43ffb3da915.bin
//     file_sha256  : da151193d8d424836c573bf743671b7b9907bf8dca37de3feb609e02d9f13b61
//     pass/stage   : _ivy_instanced_ivy_leaf (vegetation_grass) / raster
//     entry        : vs:vs_main; ps:ps_main
//     variant      : b903e43ffb3da915
//     local_size   : -
//     names        : sampler_llb, sampler_default, ViewConstants, frame_constants, $Globals, IvyLeaf, ivy_leaf_buffer, terrain_heightmap, path_mask, vs_main, sampler_llr, ivy_leaf_color
//   _ivy_instanced_ivy_leaf.raster.hlsl_f0adff76827e73ff.bin
//     file_sha256  : 19f7ffcc2aa8298617a09ccef06d3605284a93d07c0df0e52d5463fe76bc3829
//     pass/stage   : _ivy_instanced_ivy_leaf (vegetation_grass) / raster
//     entry        : vs:vs_main; ps:ps_main
//     variant      : f0adff76827e73ff
//     local_size   : -
//     names        : sampler_llb, sampler_default, ViewConstants, frame_constants, $Globals, IvyLeaf, ivy_leaf_buffer, terrain_heightmap, path_mask, vs_main, ps_main
// ==RECONSTRUCTED==
// Instanced ivy leaves. Each IvyLeaf instance orients a leaf quad along its
// forward/branch direction (flower vs leaf variants), lifts the anchor to the
// terrain height, culls leaves inside the path-mask arch fade via a NaN
// position, displaces by 3D value noise, and jitters UVs by a per-leaf seed.
// The fragment writes a packed world normal and sqrt'd leaf color, with a
// checkerboard cull when shadow rendering is disabled.

#include "inc/common.hlsl"
#include "inc/current_view.hlsl"
#include "arch_noise_discard.inc.hlsl"

// Reproduce the original shared 13-sampler $Globals layout (set0/b13).

// Per-shader $Globals members (implicit $Globals UBO, set0/b13).
int is_flower;
float4x4 model_to_world;
uint2 ivy_leaf_color_dims;
uint2 terrain_heightmap_dims;
uint2 visual_terrain_heightmap_dims;
uint2 path_mask_dims;
int bool_render_shadows;

struct IvyLeaf
{
    float3 position;
    float size;
    float3 forward_floattor;
    int seed;
    float3 branch_dir;
    float arch_height;
};

StructuredBuffer<IvyLeaf> ivy_leaf_buffer : register(t16, space0);
Texture2D<float4> terrain_heightmap : register(t17, space0);
Texture2D<float4> path_mask : register(t19, space0);
Texture2D<float4> ivy_leaf_color : register(t15, space0);

struct VSInput
{
    float3 position : TEXCOORD0;
    float3 normal : TEXCOORD1;
    float2 uv : TEXCOORD2;
    uint instance : SV_InstanceID;
};

struct VSOutput
{
    float4 position : SV_Position;
    float3 world_pos : TEXCOORD1;
    float3 normal : TEXCOORD2;
    float2 uv : TEXCOORD4;
};

VSOutput vs_main(VSInput input)
{
    VSOutput output;
    float4 position;
    float3 world_pos;
    float3 normal;
    float2 uv;
    switch (0u)
    {
    default:
        float3 pos = ivy_leaf_buffer[input.instance].position;
    float h = lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default,
                 pos.xz * (0.0076923076994717121124267578125f).xx + 0.5f.xx, 0.0f).x);
    float arch_height = ivy_leaf_buffer[input.instance].arch_height;
    float size = ivy_leaf_buffer[input.instance].size;
    float base = arch_height - min(h, 0.0f);
    float3 vp = (input.position * 0.75f) * size;
    float3 scaled = vp;
    if (glade_theme_id == 4u)
    {
        scaled = vp * 0.85f;
    }
    bool flower = (is_flower == 1);

    float3 bdir = ivy_leaf_buffer[input.instance].branch_dir;
    float3 vx, vy, vz, voff;
    if (flower)
    {
        float3 fwd = ivy_leaf_buffer[input.instance].forward_floattor;
        float3 up_cross = cross(float3(0.0f, 1.0f, 0.0f), fwd);
        vz = up_cross;
        vy = fwd;
        vx = cross(up_cross, fwd);
        voff = scaled + (fwd * 0.1f);
    }
    else
    {
        float3 fwd = ivy_leaf_buffer[input.instance].forward_floattor;
        float3 w = normalize(cross(fwd, normalize(lerp(bdir, float3(0.0f, 1.0f, 0.0f),
                       lerp(0.5f, 1.0f, abs(bdir.y)).xxx))));
        vz = fwd;
        vy = normalize(cross(w, fwd));
        vx = w;
        voff = scaled;
    }
    float4x4 rot = transpose(float4x4(float4(vx, 0.0f), float4(vy, 0.0f), float4(vz, 0.0f),
                                      float4(0.0f, 0.0f, 0.0f, 1.0f)));
    float3 offset_pos = mul(rot, float4(voff, 1.0f)).xyz;
    float3 anchor = mul(rot, float4(0.0f, 0.0f, 0.0f, 1.0f)).xyz + pos;
    float path = path_mask.SampleLevel(sampler_llb,
                 anchor.xz * (0.0076923076994717121124267578125f).xx + 0.5f.xx, 0.0f).x;
    bool over_path = path > 0.1f;
    if (over_path)
    {
        float y = anchor.y;
        float n = perlin(anchor.xz * 0.6f + y * 0.6f);
        float t = clamp(base - y, 0.0f, 1.0f);
        float d = (1.0f - exp(-5.0f * path)) * base * (t * t * clamp(smoothstep(0.0f, 0.3f, n), 0.8f, 1.0f));
        if (y < d)
        {
            position = asfloat(0x7fbfffffu).xxxx;
            world_pos = 0.0f.xxx;
            normal = 0.0f.xxx;
            uv = 0.0f.xx;
            break;
        }
    }
    float3 lifted = pos;
    lifted.y = pos.y + h;
    bool at_base = flower ? (pos.y < 0.9f) : false;
    float3 world = (at_base ? float3(0.0f, 0.0f, 0.0f) : offset_pos) + lifted;
    uint alt_seed;
    float2 disp = clamp((perlin3(world * 0.4f, over_path, alt_seed) - 0.5f) * 2.5f, -0.5f, 0.5f);
    world += float3(disp.x, 0.0f, disp.y) * 0.06f;

    uint jh1 = uint(ivy_leaf_buffer[input.instance].seed) ^ 2342304u;
    jh1 += (jh1 << 10u);
    jh1 ^= (jh1 >> 6u);
    jh1 += (jh1 << 3u);
    jh1 ^= (jh1 >> 11u);
    jh1 += (jh1 << 15u);
    uint jh2 = uint(ivy_leaf_buffer[input.instance].seed) ^ 9344565u;
    jh2 += (jh2 << 10u);
    jh2 ^= (jh2 >> 6u);
    jh2 += (jh2 << 3u);
    jh2 ^= (jh2 >> 11u);
    jh2 += (jh2 << 15u);
    float2 jitter = float2(asfloat((jh1 & 0x7FFFFFu) | 0x3F800000u) - 1.0f,
                           asfloat((jh2 & 0x7FFFFFu) | 0x3F800000u) - 1.0f);

    position = mul(view_constants[current_view_index].world_to_sample, float4(world, 1.0f));
    world_pos = world;
    normal = mul(rot, float4(input.normal, 0.0f)).xyz;
    uv = input.uv * (flower ? 0.3f : 0.1f) + jitter;
    break;
    }
    output.position = position;
    output.world_pos = world_pos;
    output.normal = normal;
    output.uv = uv;
    return output;
}

struct PSInput
{
    float3 normal : TEXCOORD2;
    float2 uv : TEXCOORD4;
    float4 position : SV_Position;
};

uint2 ps_main(PSInput input) : SV_Target0
{
    if (!(bool_render_shadows == 1))
    {
        uint2 checker = uint2(input.position.xy) & uint2(1u, 1u);
        if ((frame_index & 3u) == (checker.x + checker.y * 2u))
        {
            discard;
        }
    }
    float3 packed_normal = input.normal * 0.5f + 0.5f.xxx;
    uint n = uint(clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f + 0.5f)
           + (uint(clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
           + (uint(clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
           + 4278190080u;
    float3 sc = sqrt(clamp(ivy_leaf_color.Sample(sampler_llr, input.uv).xyz * 1.1f, 0.0f.xxx, 1.0f.xxx));
    uint m = uint(clamp(sc.x, 0.0f, 1.0f) * 255.0f + 0.5f)
           + (uint(clamp(sc.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
           + (uint(clamp(sc.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
           + 1291845632u;
    return uint2(n, m);
}
