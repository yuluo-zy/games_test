#line 1 "shaders/grass/calc_tiles.cs.hlsl"













#line 1 "shaders/inc/common.hlsl"





#ifndef TINYGLADE_INC_COMMON_HLSL
#define TINYGLADE_INC_COMMON_HLSL

#line 1 "shaders/inc/samplers.hlsl"







#ifndef TINYGLADE_INC_SAMPLERS_HLSL
#define TINYGLADE_INC_SAMPLERS_HLSL

#line 1 "shaders/inc/debug_utils.hlsl"





#ifndef TINYGLADE_INC_DEBUG_UTILS_HLSL
#define TINYGLADE_INC_DEBUG_UTILS_HLSL

#line 1 "shaders/inc/tracked_buffer.hlsl"






#ifndef TINYGLADE_INC_TRACKED_BUFFER_HLSL
#define TINYGLADE_INC_TRACKED_BUFFER_HLSL




uint tracked_buffer_reserve(RWByteAddressBuffer counter, uint counter_offset)
{
    uint previous;
    counter.InterlockedAdd(counter_offset, 1u, previous);
    return previous;
}


void tracked_buffer_store_float(RWByteAddressBuffer buffer, uint base_byte_offset, uint element_index, float value)
{
    buffer.Store<float>(base_byte_offset + element_index * 4u, value);
}

#endif 
#line 9 "shaders/inc/debug_utils.hlsl"



float3 debug_heatmap(float v)
{
    float t = clamp(v, 0.0f, 1.0f);
    return lerp(float3(0.0f, 0.0f, 1.0f), float3(1.0f, 0.0f, 0.0f), t);
}

#endif 
#line 11 "shaders/inc/samplers.hlsl"


SamplerState sampler_lnc : register(s0, space0);
SamplerState sampler_llr : register(s1, space0);
SamplerState sampler_nnc : register(s2, space0);
SamplerState sampler_llc : register(s3, space0);
SamplerState sampler_llb : register(s4, space0);
SamplerState sampler_s5 : register(s5, space0);
SamplerState sampler_s6 : register(s6, space0);
SamplerState sampler_s7 : register(s7, space0);
SamplerState sampler_s8 : register(s8, space0);
SamplerState sampler_default : register(s9, space0);
SamplerState sampler_mipmap : register(s10, space0);
SamplerState sampler_shadow_nearest : register(s11, space0);
SamplerComparisonState sampler_shadow_linear : register(s12, space0);

#endif 
#line 9 "shaders/inc/common.hlsl"


#endif 
#line 14 "shaders/grass/calc_tiles.cs.hlsl"

#line 1 "shaders/grass/grass_common.inc.hlsl"










#ifndef TINYGLADE_GRASS_GRASS_COMMON_INC_HLSL
#define TINYGLADE_GRASS_GRASS_COMMON_INC_HLSL

#line 1 "shaders/inc/consts.hlsl"






#ifndef TINYGLADE_INC_CONSTS_HLSL
#define TINYGLADE_INC_CONSTS_HLSL


static const float RENDER_SCALE = 1.0f;

#endif 
#line 14 "shaders/grass/grass_common.inc.hlsl"





struct GrassTilePacked
{
    uint raw;
};




struct GrassBundlePacked
{
    uint raw;
};

#endif 
#line 15 "shaders/grass/calc_tiles.cs.hlsl"

#line 1 "shaders/inc/frame_constants.hlsl"








#ifndef TINYGLADE_INC_FRAME_CONSTANTS_HLSL
#define TINYGLADE_INC_FRAME_CONSTANTS_HLSL

#line 1 "shaders/inc/uv.hlsl"






#ifndef TINYGLADE_INC_UV_HLSL
#define TINYGLADE_INC_UV_HLSL


float2 pixel_to_uv(int2 pixel, float2 dims)
{
    return (float2(pixel) + 0.5f) / dims;
}




float2 uv_to_sample(float2 uv)
{
    return uv - 0.5f;
}


float2 sample_to_clip(float2 sample)
{
    return sample * float2(2.0f, -2.0f);
}



float3 sample_to_view(float4x4 sample_to_view_mat, float2 sample_uv, float depth)
{
    return mul(sample_to_view_mat, float4(sample_uv * float2(2.0f, -2.0f), depth, 1.0f)).xyz;
}


float2 view_to_sample(float4x4 view_to_sample_mat, float3 view_pos)
{
    return mul(view_to_sample_mat, float4(view_pos, 1.0f)).xy;
}


float2 view_to_clip(float4x4 view_to_clip_mat, float3 view_pos)
{
    float4 clip = mul(view_to_clip_mat, float4(view_pos, 1.0f));
    return clip.xy / clip.w;
}


float3 world_to_view(float4x4 world_to_view_mat, float3 world_pos)
{
    return mul(world_to_view_mat, float4(world_pos, 1.0f)).xyz;
}


float2 world_to_sample(float4x4 world_to_sample_mat, float3 world_pos)
{
    return mul(world_to_sample_mat, float4(world_pos, 1.0f)).xy;
}

#endif 
#line 12 "shaders/inc/frame_constants.hlsl"










Texture2D<float4> bindless_textures_dyn[] : register(t0, space2);


struct ViewConstants
{
    float4x4 view_to_clip;
    float4x4 clip_to_view;
    float4x4 view_to_sample;
    float4x4 sample_to_view;
    float4x4 world_to_view;
    float4x4 view_to_world;
    float4x4 world_to_sample;
    float4x4 clip_to_prev_clip;
    float4x4 prev_view_to_prev_clip;
    float4x4 prev_clip_to_prev_view;
    float4x4 prev_world_to_prev_view;
    float4x4 prev_view_to_prev_world;
    float2 sample_offset_pixels;
    float2 sample_offset_clip;
    float2 prev_sample_offset_pixels;
    float2 prev_sample_offset_clip;
};

cbuffer frame_constants : register(b0, space1)
{
    float4 sun_color;
    float4 sky_low_color;
    float4 sky_high_color;
    float4 fog_tint_thickness;
    float4 fake_light_color;
    float4 fake_light_dir;
    float4 fake_light2_color;
    float4 fake_light2_dir;
    uint frame_index;
    float delta_time_seconds;
    float ev_mult;
    uint glade_theme_id;
    uint2 inner_rendering_resolution;
    uint game_time_sec;
    float game_time_frac;
    float3 editing_highlight_color;
    float time_of_day;
    float3 dir_to_sun;
    uint camera_idx;
    float3 dir_to_moon;
    uint render_overrides;
    float global_mip_bias;
    uint highlighted_transient_stair_edge_id;
    uint pad1;
    uint pad2;
    float4 debug_tweakers;
    ViewConstants view_constants[5];
};

#endif 
#line 16 "shaders/grass/calc_tiles.cs.hlsl"

#line 1 "shaders/inc/ray_cone.hlsl"







#ifndef TINYGLADE_INC_RAY_CONE_HLSL
#define TINYGLADE_INC_RAY_CONE_HLSL



struct RayCone
{
    float3 origin;
    float3 direction;
    float spread; 
};


float ray_cone_half_width(RayCone cone, float t)
{
    return t * cone.spread;
}



float ray_cone_mip(float2 footprint_world, float2 world_per_texel)
{
    float2 footprint_texels = max(footprint_world / max(world_per_texel, 1e-6f.xx), 1.0f.xx);
    return log2(max(footprint_texels.x, footprint_texels.y));
}

#endif 
#line 17 "shaders/grass/calc_tiles.cs.hlsl"

#line 1 "shaders/inc/math_const.hlsl"





#ifndef TINYGLADE_INC_MATH_CONST_HLSL
#define TINYGLADE_INC_MATH_CONST_HLSL

static const float PI = 3.14159265358979323846264338327950288f;
static const float TWO_PI = 6.28318530717958647692528676655900577f;
static const float HALF_PI = 1.57079632679489661923132169163975144f;
static const float INV_PI = 0.31830988618379067153776752674502872f;
static const float INV_TWO_PI = 0.15915494309189533576888376337251436f;

static const float SQRT_TWO = 1.41421356237309504880168872420969808f;
static const float INV_SQRT_TWO = 0.70710678118654752440084436210484904f;

static const float GOLDEN_RATIO = 1.61803398874989484820458683436563812f;
static const float INV_GOLDEN_RATIO = 0.61803398874989484820458683436563812f;

static const float EPSILON = 1.0e-6f;
static const float FLT_EPSILON = 1.19209290e-7f;

#endif 
#line 18 "shaders/grass/calc_tiles.cs.hlsl"

#line 1 "shaders/inc/morton.hlsl"








uint morton_encode(uint2 p)
{
    uint x = p.x & 0xFFFFu;
    x = (x | (x << 8u)) & 0x00FF00FFu;
    x = (x | (x << 4u)) & 0x0F0F0F0Fu;
    x = (x | (x << 2u)) & 0x33333333u;
    x = (x | (x << 1u)) & 0x55555555u;
    uint y = p.y & 0xFFFFu;
    y = (y | (y << 8u)) & 0x00FF00FFu;
    y = (y | (y << 4u)) & 0x0F0F0F0Fu;
    y = (y | (y << 2u)) & 0x33333333u;
    y = (y | (y << 1u)) & 0x55555555u;
    return x | (y << 1u);
}

uint2 morton_decode(uint m)
{
    uint x = m & 0x55555555u;
    x = (x ^ (x >> 1u)) & 0x33333333u;
    x = (x ^ (x >> 2u)) & 0x0F0F0F0Fu;
    x = (x ^ (x >> 4u)) & 0x00FF00FFu;
    x = (x ^ (x >> 8u)) & 0xFFFFu;
    uint y = (m >> 1u) & 0x55555555u;
    y = (y ^ (y >> 1u)) & 0x33333333u;
    y = (y ^ (y >> 2u)) & 0x0F0F0F0Fu;
    y = (y ^ (y >> 4u)) & 0x00FF00FFu;
    y = (y ^ (y >> 8u)) & 0xFFFFu;
    return uint2(x, y);
}
#line 19 "shaders/grass/calc_tiles.cs.hlsl"

#line 1 "shaders/inc/frustum.hlsl"


#ifndef TINYGLADE_INC_FRUSTUM_HLSL
#define TINYGLADE_INC_FRUSTUM_HLSL



struct Frustum {
    float4 planes[6];
};



Frustum extract_frustum(float4x4 m)
{
    Frustum f;
    f.planes[0] = m[3] + m[0]; 
    f.planes[1] = m[3] - m[0]; 
    f.planes[2] = m[3] + m[1]; 
    f.planes[3] = m[3] - m[1]; 
    f.planes[4] = m[3] + m[2]; 
    f.planes[5] = m[3] - m[2]; 
    return f;
}


float frustum_plane_distance(float4 plane, float3 p)
{
    return dot(plane.xyz, p) + plane.w;
}


bool frustum_intersects_sphere(Frustum f, float3 center, float radius)
{
    for (uint i = 0u; i < 6u; ++i) {
        if (frustum_plane_distance(f.planes[i], center) < -radius) {
            return false;
        }
    }
    return true;
}


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

#endif 
#line 20 "shaders/grass/calc_tiles.cs.hlsl"

#line 1 "shaders/inc/lod.hlsl"


#ifndef TINYGLADE_INC_LOD_HLSL
#define TINYGLADE_INC_LOD_HLSL


struct MeshLod {
    uint first_index;
    uint index_count;
    float bounding_sphere_radius;
    uint pad0;
};



uint select_lod(float distance_sq, float lod0_distance_sq, float lod1_distance_sq)
{
    if (distance_sq < lod0_distance_sq) {
        return 0u;
    }
    if (distance_sq < lod1_distance_sq) {
        return 1u;
    }
    return 2u;
}


uint select_grass_lod(float distance_sq, float lod1_distance_sq)
{
    return (distance_sq < lod1_distance_sq) ? 0u : 1u;
}

#endif 
#line 21 "shaders/grass/calc_tiles.cs.hlsl"

#line 1 "shaders/inc/math.hlsl"





#ifndef TINYGLADE_INC_MATH_HLSL
#define TINYGLADE_INC_MATH_HLSL


float clamp01(float v)
{
    return clamp(v, 0.0f, 1.0f);
}

float3 clamp01(float3 v)
{
    return clamp(v, 0.0f, 1.0f);
}


float length_squared(float3 v)
{
    return dot(v, v);
}


float3 safe_normalize(float3 v)
{
    float len_sq = dot(v, v);
    float len = sqrt(len_sq);
    return len > 1.0e-6f ? v / len : float3(0.0f, 0.0f, 0.0f);
}


float safe_rcp(float v)
{
    return 1.0f / max(abs(v), 1.0e-6f);
}


float rcp_keep_sign(float v)
{
    return sign(v) * safe_rcp(v);
}

#endif 
#line 22 "shaders/grass/calc_tiles.cs.hlsl"

#line 1 "shaders/inc/projection.hlsl"





#ifndef TINYGLADE_INC_PROJECTION_HLSL
#define TINYGLADE_INC_PROJECTION_HLSL



float4x4 perspective_projection(float fovy_radians, float aspect, float znear, float zfar)
{
    float y = 1.0f / tan(fovy_radians * 0.5f);
    float x = y / aspect;
    float a = zfar / (zfar - znear);
    float b = -znear * a;
    return float4x4(
        float4(x, 0.0f, 0.0f, 0.0f),
        float4(0.0f, y, 0.0f, 0.0f),
        float4(0.0f, 0.0f, a, b),
        float4(0.0f, 0.0f, 1.0f, 0.0f));
}


float4x4 orthographic_projection(float left, float right, float bottom, float top, float znear, float zfar)
{
    float2 inv = float2(1.0f / (right - left), 1.0f / (top - bottom));
    float a = 1.0f / (zfar - znear);
    return float4x4(
        float4(2.0f * inv.x, 0.0f, 0.0f, 0.0f),
        float4(0.0f, 2.0f * inv.y, 0.0f, 0.0f),
        float4(0.0f, 0.0f, a, -znear * a),
        float4(-(right + left) * inv.x, -(top + bottom) * inv.y, 0.0f, 1.0f));
}


float3 unproject_to_view(float4x4 clip_to_view, float2 ndc, float depth)
{
    float4 clip = float4(ndc, depth, 1.0f);
    float4 view = mul(clip_to_view, clip);
    return view.xyz / view.w;
}

#endif 
#line 23 "shaders/grass/calc_tiles.cs.hlsl"


Texture2D<float4> terrain_heightmap : register(t14, space0);
#line 1 "shaders/inc/terrain_heightmap.inc.hlsl"







#ifndef TINYGLADE_INC_TERRAIN_HEIGHTMAP_HLSL
#define TINYGLADE_INC_TERRAIN_HEIGHTMAP_HLSL


static const float TERRAIN_WORLD_HALF_EXTENT = 65.0;
static const float TERRAIN_HEIGHT_MIN = -2.5;
static const float TERRAIN_HEIGHT_MAX = 24.0;
static const float TERRAIN_SAMPLE_SCALE = 0.0076923076994717121124267578125; 


float2 terrain_heightmap_uv(float2 world_xz)
{
    return world_xz * TERRAIN_SAMPLE_SCALE + 0.5;
}


float get_terrain_height(Texture2D terrain_heightmap, SamplerState s, float2 world_xz)
{
    float t = terrain_heightmap.SampleLevel(s, terrain_heightmap_uv(world_xz), 0.0).x;
    return lerp(TERRAIN_HEIGHT_MIN, TERRAIN_HEIGHT_MAX, t);
}










#ifndef TINYGLADE_TERRAIN_HEIGHTMAP_NO_GLOBAL
float terrain_height(float2 world_xz)
{
    return get_terrain_height(terrain_heightmap, sampler_default, world_xz);
}
#endif


float3 get_terrain_normal(Texture2D terrain_heightmap, SamplerState s, float2 world_xz, float2 heightmap_dims)
{
    float4 g = terrain_heightmap.GatherRed(s, terrain_heightmap_uv(world_xz), int2(0, 0));
    float2 grad = (float2((g.z + g.y) - (g.w + g.x), (g.x + g.y) - (g.w + g.z)) * 0.5) * 26.5;
    grad /= (130.0 / max(heightmap_dims, 1e-6));
    float3 right = normalize(float3(1.0, grad.x, 0.0));
    float3 fwd = normalize(float3(0.0, grad.y, 1.0));
    return normalize(cross(fwd, right));
}

#endif 
#line 26 "shaders/grass/calc_tiles.cs.hlsl"

#line 1 "shaders/inc/prefix_scan.hlsl"


#ifndef TINYGLADE_INC_PREFIX_SCAN_HLSL
#define TINYGLADE_INC_PREFIX_SCAN_HLSL






#define PREFIX_SCAN_UINT(shared, lane, element_count) \
    { \
        for (uint _ps_s = 0u; (1u << _ps_s) < (element_count); ++_ps_s) { \
            GroupMemoryBarrierWithGroupSync(); \
            uint _ps_d = 1u << _ps_s; \
            uint _ps_base = ((lane) >> _ps_s) << (_ps_s + 1u); \
            uint _ps_i = _ps_base + _ps_d + ((lane) & (_ps_d - 1u)); \
            if (_ps_i < (element_count)) { \
                shared[_ps_i] += shared[_ps_i - _ps_d]; \
            } \
        } \
        GroupMemoryBarrierWithGroupSync(); \
    }





#define PREFIX_SCAN_2_PER_LANE(inout_buf, subtotals_buf, shared, lane, group_id, block_words, value_clamp) \
    { \
        uint _ps2_idx0 = 4u * ((2u * (lane)) + ((group_id) * (block_words))); \
        uint _ps2_idx1 = _ps2_idx0 + 1u; \
        uint _ps2_v0 = min((inout_buf).Load<uint>(_ps2_idx0 * 4u + 0u), (value_clamp)); \
        uint _ps2_v1 = min((inout_buf).Load<uint>(_ps2_idx1 * 4u + 0u), (value_clamp)); \
        (shared)[(lane) * 2u] = _ps2_v0; \
        (shared)[(lane) * 2u + 1u] = _ps2_v1; \
        GroupMemoryBarrierWithGroupSync(); \
        PREFIX_SCAN_UINT((shared), (lane), (block_words) * 2u); \
        uint _ps2_ex0 = ((lane) == 0u) ? 0u : (shared)[(lane) * 2u - 1u]; \
        uint _ps2_ex1 = (shared)[(lane) * 2u]; \
        (inout_buf).Store<uint>(_ps2_idx0 * 4u + 0u, _ps2_ex0); \
        (inout_buf).Store<uint>(_ps2_idx1 * 4u + 0u, _ps2_ex1); \
        if ((lane) == 0u) { \
            (subtotals_buf).Store<uint>((group_id) * 4u + 0u, (shared)[(block_words) * 2u - 1u]); \
        } \
    }

#endif 
#line 27 "shaders/grass/calc_tiles.cs.hlsl"


cbuffer $Globals : register(b13, space0)
{
    uint2 terrain_heightmap_dims;
    uint2 visual_terrain_heightmap_dims;
    uint2 hzb_tex_inner_dims;
    uint freeze_occlusion_culling;
};

RWStructuredBuffer<GrassTilePacked> __alias_tile_buffer : register(u16, space0);
RWStructuredBuffer<uint> __alias_blade_prefix_buffer : register(u17, space0);
RWStructuredBuffer<uint> __alias_blade_prefix_subtotal_buffer : register(u18, space0);
Texture2D<float4> hzb_tex : register(t19, space0);



static const float GRASS_FRUSTUM_FAR = 1.43631064891815185546875f;

groupshared uint shared_data1x[64];

[numthreads(64, 1, 1)]
void cs_main(uint3 group_id : SV_GroupID, uint3 dispatch_id : SV_DispatchThreadID, uint local_index : SV_GroupIndex)
{
    GrassTilePacked tile = __alias_tile_buffer[dispatch_id.x];
    uint tile_blades;
    if (freeze_occlusion_culling != 0u)
    {
        tile_blades = (tile.raw & 0xFFFFu) % 512u;
    }
    else
    {
        uint tile_x = dispatch_id.x % 128u;
        uint tile_y = dispatch_id.x / 128u;
        float2 tile_xy = float2(float(tile_x), float(tile_y));
        float2 tile_center_ws = (tile_xy - 63.5f) * (1.0f / 128.0f) * 260.0f;

        
        float4 row_x = view_constants[0].view_to_clip[0];
        float4 row_y = view_constants[0].view_to_clip[1];
        float4 row_z = view_constants[0].view_to_clip[3];
        float4 plane_x = float4(-row_x.x, row_x.y, 0.0f, row_x.w);
        float4 plane_y = float4(row_y.x, row_y.y, 0.0f, row_y.w);
        float4 plane_z = float4(row_z.x, row_z.y, -1.0f, row_z.w);
        float4 corners[4];
        corners[0] = plane_z + plane_x;
        corners[1] = plane_z - plane_x;
        corners[2] = plane_z + plane_y;
        corners[3] = plane_z - plane_y;
        for (int i = 0; i < 4; i++)
        {
            corners[i] /= length(corners[i].xyz).xxxx;
        }

        float tile_height = lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, tile_center_ws * (1.0f / 130.0f) + 0.5f, 0.0f).x);
        float4 view_pos = mul(view_constants[0].world_to_view, float4(tile_center_ws.x, tile_height, tile_center_ws.y, 1.0f));
        float z = view_pos.z;

        
        
        bool inside;
        if ((GRASS_FRUSTUM_FAR - z) > view_constants[0].view_to_clip[2].w)
        {
            inside = ((-GRASS_FRUSTUM_FAR) - z) < 3.4028234663852885981170418348452e+38f;
        }
        else
        {
            inside = false;
        }
        bool running = inside;
        bool next;
        for (int i = 0; i < 4; running = next, i++)
        {
            if (running)
            {
                next = dot(float4(view_pos.x, view_pos.y, z, 1.0f), corners[i]) > (-GRASS_FRUSTUM_FAR);
            }
            else
            {
                next = false;
            }
        }

        
        float inv_z = -z;
        float3 dir = float3(view_pos.x, view_pos.y, inv_z) * GRASS_FRUSTUM_FAR;
        float z_sq = (inv_z * inv_z) - (GRASS_FRUSTUM_FAR * GRASS_FRUSTUM_FAR);
        float rx = sqrt((view_pos.x * view_pos.x) + z_sq);
        float ry = sqrt((view_pos.y * view_pos.y) + z_sq);
        float2 proj_scale = float2(view_constants[0].view_to_clip[0].x * 0.5f, view_constants[0].view_to_clip[1].y * 0.5f);
        float4 rect;
        rect.x = ((rx * view_pos.x - dir.z) / (rx * inv_z + dir.x)) * proj_scale.x;
        rect.y = (-((ry * view_pos.y + dir.z) / (ry * inv_z - dir.y))) * proj_scale.y;
        rect.z = ((rx * view_pos.x + dir.z) / (rx * inv_z - dir.x)) * proj_scale.x;
        rect.w = (-((ry * view_pos.y - dir.z) / (ry * inv_z + dir.y))) * proj_scale.y;
        float4 hzb_dims = float4(hzb_tex_inner_dims.xyxy);
        float4 hzb_uv = clamp((rect + 0.5f) * hzb_dims, 0.0f, hzb_dims - 1.0f);
        uint hzb_mip = uint(max(0.0f, ceil(log2(max(hzb_uv.z - hzb_uv.x, hzb_uv.w - hzb_uv.y)))));

        
        bool visible;
        if (running)
        {
            uint2 mip_shift = hzb_mip.xx & uint2(31u, 31u);
            uint2 lo = uint2(hzb_uv.xy) >> mip_shift;
            uint2 hi = uint2(hzb_uv.zw) >> mip_shift;
            float min_depth = min(min(hzb_tex.Load(int3(lo, hzb_mip)).x, hzb_tex.Load(int3(uint2(lo.x, hi.y), hzb_mip)).x), min(hzb_tex.Load(int3(uint2(hi.x, lo.y), hzb_mip)).x, hzb_tex.Load(int3(hi, hzb_mip)).x));
            visible = (z + GRASS_FRUSTUM_FAR) > (1.0f / (min_depth * (-view_constants[0].clip_to_view[3].z)));
        }
        else
        {
            visible = running;
        }

        float occ;
        if (!visible)
        {
            occ = 0.0f;
        }
        else
        {
            occ = clamp((0.02f / max(1e-5f, inv_z)) * ((0.5f * view_constants[0].view_to_clip[1].y) * float(clamp(inner_rendering_resolution.y, 1080u, 2160u))), 0.0f, 1.0f);
        }
        float fade = smoothstep(64.0f, 21.3333339691162109375f, length(tile_xy - 64.0f));
        tile_blades = uint(clamp(lerp(0.0f, 204.8000030517578125f, occ * fade), 0.0f, 256.0f));
        GrassTilePacked new_tile = { ((tile_x << 24u) | (tile_y << 16u)) | tile_blades };
        __alias_tile_buffer[dispatch_id.x] = new_tile;
    }

    shared_data1x[local_index] = tile_blades;
    GroupMemoryBarrierWithGroupSync();
    uint i = (local_index >> 0u) << 1u;
    uint j = i + 1u;
    if (j < 64u)
    {
        shared_data1x[j] += shared_data1x[i];
    }
    GroupMemoryBarrierWithGroupSync();
    i = (local_index >> 1u) << 2u;
    j = i + 2u + (local_index & 1u);
    if (j < 64u)
    {
        shared_data1x[j] += shared_data1x[i + 1u];
    }
    GroupMemoryBarrierWithGroupSync();
    i = (local_index >> 2u) << 3u;
    j = i + 4u + (local_index & 3u);
    if (j < 64u)
    {
        shared_data1x[j] += shared_data1x[i + 3u];
    }
    GroupMemoryBarrierWithGroupSync();
    i = (local_index >> 3u) << 4u;
    j = i + 8u + (local_index & 7u);
    if (j < 64u)
    {
        shared_data1x[j] += shared_data1x[i + 7u];
    }
    GroupMemoryBarrierWithGroupSync();
    i = (local_index >> 4u) << 5u;
    j = i + 16u + (local_index & 15u);
    if (j < 64u)
    {
        shared_data1x[j] += shared_data1x[i + 15u];
    }
    GroupMemoryBarrierWithGroupSync();
    i = (local_index >> 5u) << 6u;
    j = i + 32u + (local_index & 31u);
    if (j < 64u)
    {
        shared_data1x[j] += shared_data1x[i + 31u];
    }
    GroupMemoryBarrierWithGroupSync();
    __alias_blade_prefix_buffer[dispatch_id.x] = (local_index == 0u) ? 0u : shared_data1x[local_index - 1u];
    if (local_index == 0u)
    {
        __alias_blade_prefix_subtotal_buffer[group_id.x] = shared_data1x[63];
    }
}
