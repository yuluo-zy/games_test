// ============================================================
// inc/frustum.hlsl —— 视锥体平面提取与相交测试
// 用途:对主视图做球体/AABB 剔除。
//   消费方:_nani_cull_and_bucket、_generate_grass、_grass_calc_tiles。
// 入口:无(纯头文件)
// 关键算法:从 view-projection 提取 6 个平面(法向朝内),p-vertex 测试
// ============================================================
// ==RECONSTRUCTED== shaders/inc/frustum.hlsl | evidence: consumers _nani_cull_and_bucket/_generate_grass/_grass_calc_tiles cull against the main view; spirv-cross unavailable for the PhysicalStorageBufferAddresses nani variant so the helper surface follows the standard plane-extraction pattern | status: O/I (function/struct names Inference, inlined)

#ifndef TINYGLADE_INC_FRUSTUM_HLSL
#define TINYGLADE_INC_FRUSTUM_HLSL

// 六个裁剪平面(left/right/bottom/top/near/far),平面形式
// (xyz = 法向,w = 负距离),从 view-projection 提取。
struct Frustum {
    float4 planes[6];
};

// 从 view-projection 矩阵(行主序,dxc 存储方式)提取视锥平面。
// 平面法向朝内。
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

// 点到平面的有符号距离(正 = 在平面内侧)。
float frustum_plane_distance(float4 plane, float3 p)
{
    return dot(plane.xyz, p) + plane.w;
}

// 球体与视锥相交测试(每个平面上都为正/零 => 在内)。
bool frustum_intersects_sphere(Frustum f, float3 center, float radius)
{
    for (uint i = 0u; i < 6u; ++i) {
        if (frustum_plane_distance(f.planes[i], center) < -radius) {
            return false;
        }
    }
    return true;
}

// AABB 与视锥相交测试,用 p-vertex(正极端点)测试。
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
