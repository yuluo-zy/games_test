// ==RECONSTRUCTED== shaders/inc/frustum.hlsl | evidence: consumers _nani_cull_and_bucket/_generate_grass/_grass_calc_tiles cull against the main view; spirv-cross unavailable for the PhysicalStorageBufferAddresses nani variant so the helper surface follows the standard plane-extraction pattern | status: O/I (function/struct names Inference, inlined)

#ifndef TINYGLADE_INC_FRUSTUM_HLSL
#define TINYGLADE_INC_FRUSTUM_HLSL

// Six clip planes (left, right, bottom, top, near, far) in plane form
// (xyz = normal, w = negative distance), extracted from a view-projection.
struct Frustum {
    float4 planes[6];
};

// Extract frustum planes from a view-projection matrix (row-major, as dxc
// stores matrices). Plane normals point inward.
Frustum extract_frustum(float4x4 m)
{
    Frustum f;
    f.planes[0] = m[3] + m[0]; // left
    f.planes[1] = m[3] - m[0]; // right
    f.planes[2] = m[3] + m[1]; // bottom
    f.planes[3] = m[3] - m[1]; // top
    f.planes[4] = m[3] + m[2]; // near
    f.planes[5] = m[3] - m[2]; // far
    return f;
}

// Signed distance from a point to a plane (positive = inside).
float frustum_plane_distance(float4 plane, float3 p)
{
    return dot(plane.xyz, p) + plane.w;
}

// Sphere vs frustum test (positive/zero on every plane => inside).
bool frustum_intersects_sphere(Frustum f, float3 center, float radius)
{
    for (uint i = 0u; i < 6u; ++i) {
        if (frustum_plane_distance(f.planes[i], center) < -radius) {
            return false;
        }
    }
    return true;
}

// AABB vs frustum test using the p-vertex (positive-extent) test.
bool frustum_intersects_aabb(Frustum f, float3 aabb_min, float3 aabb_max)
{
    for (uint i = 0u; i < 6u; ++i) {
        float3 p;
        p.x = (f.planes[i].x >= 0.0) ? aabb_max.x : aabb_min.x;
        p.y = (f.planes[i].y >= 0.0) ? aabb_max.y : aabb_min.y;
        p.z = (f.planes[i].z >= 0.0) ? aabb_max.z : aabb_min.z;
        if (frustum_plane_distance(f.planes[i], p) < 0.0) {
            return false;
        }
    }
    return true;
}

#endif // TINYGLADE_INC_FRUSTUM_HLSL
