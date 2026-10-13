// ==RECONSTRUCTED== shaders/roof/roof_tile.inc.hlsl | F-V | embedded in nani_cull_and_bucket_roof_tile.cs (2), nani_instanced_roof.raster (2), roof_tiles_gravity.cs
// Shared roof-tile record types + helpers used by the roof family (instanced roof
// raster and the gravity sim). Member layouts and names recovered verbatim from the
// consumer bins' OpMemberName / raw-per-bin.jsonl:
//   RoofShingle   64B: base_position f3 +0, current_height f +12, clipping_plane_ u2 +16,
//                     scale_t u2 +24, rotation_quat_ u2 +32, roof_id u +40,
//                     velocity_rotation u +44, animation_t f +48, seed i +52,
//                     is_circle i +56, is_main_roof_body i +60.
//   PushConstants (cbuffer 'push_constants'): first_instance, instance_count,
//                     wall_height_delta, roof_pivot_y, animation_t.
// Helper names (pack_half2x16 / unpack_half2x16 / quat_to_matrix) are Inference
// (inlined in consumers; only types/globals carry OpName).

#ifndef TINYGLADE_ROOF_ROOF_TILE_HLSL
#define TINYGLADE_ROOF_ROOF_TILE_HLSL

// Per-roof-tile instance record (also used as the StructuredBuffer element type of
// `instanced_data` / `__alias_instanced_data`).
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

// Roof-family push constants (bound as the shader's push-constant block).
struct RoofPushConstants
{
    uint first_instance;
    uint instance_count;
    float wall_height_delta;
    float roof_pivot_y;
    float animation_t;
};

// Pack two half-precision floats into one uint (low .x, high .y).
uint pack_half2x16(float2 value)
{
    uint2 packed = f32tof16(value);
    return packed.x | (packed.y << 16);
}

// Unpack a half2x16 uint into two floats.
float2 unpack_half2x16(uint value)
{
    return f16tof32(uint2(value & 0xFFFFu, value >> 16));
}

// Quaternion (x,y,z,w) -> 3x4 rotation+translation matrix (columns).
float3x4 quat_to_matrix(float4 q, float3 translation)
{
    float x = q.x;
    float y = q.y;
    float z = q.z;
    float w = q.w;
    float x2 = x + x;
    float y2 = y + y;
    float z2 = z + z;
    float xx = x * x2;
    float xy = x * y2;
    float xz = x * z2;
    float yy = y * y2;
    float yz = y * z2;
    float zz = z * z2;
    float wx = w * x2;
    float wy = w * y2;
    float wz = w * z2;
    return float3x4(
        float4(1.0f - (yy + zz), xy + wz, xz - wy, translation.x),
        float4(xy - wz, 1.0f - (xx + zz), yz + wx, translation.y),
        float4(xz + wy, yz - wx, 1.0f - (xx + yy), translation.z));
}

#endif // TINYGLADE_ROOF_ROOF_TILE_HLSL
