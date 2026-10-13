#line 1 "shaders/nani/cull_and_bucket_flowers.cs.hlsl"





#line 1 "shaders/nani/cull_and_bucket_flowers.cs.hlsl"

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
#line 8 "shaders/nani/cull_and_bucket_flowers.cs.hlsl"

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
#line 9 "shaders/nani/cull_and_bucket_flowers.cs.hlsl"

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
#line 10 "shaders/nani/cull_and_bucket_flowers.cs.hlsl"

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
#line 11 "shaders/nani/cull_and_bucket_flowers.cs.hlsl"

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
#line 12 "shaders/nani/cull_and_bucket_flowers.cs.hlsl"

#line 1 "shaders/nani/draw_lists.inc.hlsl"






#ifndef TINYGLADE_NANI_DRAW_LISTS_HLSL
#define TINYGLADE_NANI_DRAW_LISTS_HLSL


struct GpuFilteredInstance
{
    uint raw;
};


struct GpuFilteredInstanceMeta
{
    uint raw;
};


struct NaniDrawListInfo
{
    uint global_instance_offset;
};


struct DrawIndexedIndirectCommand
{
    uint index_count;
    uint instance_count;
    uint first_index;
    uint vertex_offset;
    uint first_instance;
};

#endif 
#line 13 "shaders/nani/cull_and_bucket_flowers.cs.hlsl"

#line 1 "shaders/nani/cull_and_bucket.inc.hlsl"
















#ifndef TINYGLADE_NANI_CULL_AND_BUCKET_INC_HLSL
#define TINYGLADE_NANI_CULL_AND_BUCKET_INC_HLSL

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
#line 20 "shaders/nani/cull_and_bucket.inc.hlsl"

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
#line 21 "shaders/nani/cull_and_bucket.inc.hlsl"

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
#line 22 "shaders/nani/cull_and_bucket.inc.hlsl"

#line 1 "shaders/inc/cascades.hlsl"


#ifndef TINYGLADE_INC_CASCADES_HLSL
#define TINYGLADE_INC_CASCADES_HLSL


struct ShadowCascadeSelection {
    int slice;          
    float2 shadow_uv;   
    float texel_scale;  
    float sample_scale; 
};



ShadowCascadeSelection select_shadow_cascade(float3 world_pos)
{
    ShadowCascadeSelection sel;
    sel.slice = -1;
    sel.shadow_uv = float2(0.0, 0.0);
    sel.texel_scale = 10.0;
    sel.sample_scale = 1.0;

    float4 c0 = mul(view_constants[2].world_to_sample, float4(world_pos, 1.0));
    if (c0.z > 1.0) {
        return sel; 
    }
    float2 uv = c0.xy;
    bool c0_out = any(abs(uv) > 0.983333349f);
    float texel_scale = c0_out ? 5.0 : 10.0;
    float sample_scale = c0_out ? 0.5 : 1.0;
    uint slice = c0_out ? 1u : 0u;
    if (c0_out) {
        uv *= 0.5;
    }
    bool c1_out = any(abs(uv) > (1.0 - (0.00083333335f * (2.0 * texel_scale))));
    if (c1_out) {
        texel_scale = 10.0 * 0.2857142984867095947265625f;
        uv *= 0.2857142984867095947265625f;
        sample_scale = sample_scale * 0.2857142984867095947265625f;
        slice = 2u;
    }
    
    float4 final_uv = mul(view_constants[2 + slice].world_to_sample, float4(world_pos, 1.0));
    sel.slice = int(slice);
    sel.shadow_uv = float2(final_uv.x, -final_uv.y) * 0.5 + 0.5;
    sel.texel_scale = texel_scale;
    sel.sample_scale = sample_scale;
    return sel;
}

#endif 
#line 23 "shaders/nani/cull_and_bucket.inc.hlsl"

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
#line 24 "shaders/nani/cull_and_bucket.inc.hlsl"





#ifndef TINYGLADE_INC_UTILS_HLSL
struct Affine3Packed
{
    float data[12];
};
#endif


StructuredBuffer<uint> prev_visibility_bitmask : register(t18, space0);
RWStructuredBuffer<uint> __alias_next_visibility_bitmask : register(u19, space0);
RWStructuredBuffer<uint> __alias_filtered_instance_buffer : register(u14, space0);
RWStructuredBuffer<uint> __alias_filtered_instance_allocator_buffer : register(u15, space0);
RWStructuredBuffer<GpuFilteredInstanceMeta> __alias_filtered_instance_meta_buffer : register(u16, space0);
RWStructuredBuffer<uint> __alias_output_histogram : register(u17, space0);




float2 decode_instance_meta_size(uint meta)
{
    return float2(f16tof32(meta & 0xFFFFu),
                  f16tof32((meta >> 16u) & 0xFFFFu));
}

uint decode_instance_mesh_index(uint meta)
{
    return meta >> 16u;
}


bool cull_frustum_side_planes(Frustum f, float3 center, float radius)
{
    for (uint i = 0u; i < 4u; ++i)
    {
        if (frustum_plane_distance(f.planes[i], center) < -radius)
        {
            return false;
        }
    }
    return true;
}



void cull_append_visible(uint draw_list_index, uint global_instance_index, uint mesh_id, uint bucket)
{
    uint bit_index = global_instance_index / 32u;
    uint bit_mask = 1u << (global_instance_index % 32u);
    InterlockedOr(__alias_next_visibility_bitmask[bit_index], bit_mask);

    if ((prev_visibility_bitmask[bit_index] & bit_mask) == 0u)
    {
        uint slot;
        InterlockedAdd(__alias_filtered_instance_allocator_buffer[draw_list_index], 1u, slot);
        __alias_filtered_instance_buffer[slot] = global_instance_index;
        GpuFilteredInstanceMeta meta = GpuFilteredInstanceMeta(mesh_id);
        __alias_filtered_instance_meta_buffer[slot] = meta;
        InterlockedAdd(__alias_output_histogram[draw_list_index * 1024u + bucket], 1u);
    }
}

#endif 
#line 14 "shaders/nani/cull_and_bucket_flowers.cs.hlsl"

#line 1 "shaders/inc/pack_unpack.hlsl"




#ifndef TINYGLADE_INC_PACK_UNPACK_HLSL
#define TINYGLADE_INC_PACK_UNPACK_HLSL


uint pack_rgba8(float4 c)
{
    uint r = uint(clamp(c.x, 0.0f, 1.0f) * 255.0f + 0.5f);
    uint g = uint(clamp(c.y, 0.0f, 1.0f) * 255.0f + 0.5f);
    uint b = uint(clamp(c.z, 0.0f, 1.0f) * 255.0f + 0.5f);
    uint a = uint(clamp(c.w, 0.0f, 1.0f) * 255.0f + 0.5f);
    return (a << 24u) | (b << 16u) | (g << 8u) | r;
}


uint pack_rgb8(float3 c)
{
    uint r = uint(clamp(c.x, 0.0f, 1.0f) * 255.0f + 0.5f);
    uint g = uint(clamp(c.y, 0.0f, 1.0f) * 255.0f + 0.5f);
    uint b = uint(clamp(c.z, 0.0f, 1.0f) * 255.0f + 0.5f);
    return (b << 16u) | (g << 8u) | r;
}


float3 unpack_rgb8(uint p)
{
    float3 c;
    c.r = float(p & 0xFFu) * (1.0f / 255.0f);
    c.g = float((p >> 8u) & 0xFFu) * (1.0f / 255.0f);
    c.b = float((p >> 16u) & 0xFFu) * (1.0f / 255.0f);
    return c;
}


float4 unpack_rgba8(uint p)
{
    return float4(unpack_rgb8(p), float((p >> 24u) & 0xFFu) * (1.0f / 255.0f));
}


float3 unpack_normal_rgb8(uint p)
{
    return unpack_rgb8(p) * 2.0f - 1.0f;
}

#endif 
#line 15 "shaders/nani/cull_and_bucket_flowers.cs.hlsl"

#line 1 "shaders/nani/atlas_part_info.inc.hlsl"






#ifndef TINYGLADE_NANI_ATLAS_PART_INFO_HLSL
#define TINYGLADE_NANI_ATLAS_PART_INFO_HLSL


struct MeshAtlasPartInfo
{
    uint first_index;
    uint index_count;
    float bounding_sphere_radius;
    uint pad0;
};

#endif 
#line 16 "shaders/nani/cull_and_bucket_flowers.cs.hlsl"

#line 1 "shaders/garden/defs.inc.hlsl"


#ifndef TINYGLADE_GARDEN_DEFS_HLSL
#define TINYGLADE_GARDEN_DEFS_HLSL



struct BushXform
{
    Affine3Packed transform;
};

struct FlowerXform
{
    Affine3Packed transform;
    float3 color;
    uint flower_type;
};

#endif 
#line 17 "shaders/nani/cull_and_bucket_flowers.cs.hlsl"


StructuredBuffer<MeshAtlasPartInfo[64]> atlas_part_info : register(t13, space0);
StructuredBuffer<NaniDrawListInfo> draw_list_info : register(t20, space0);
#if !defined(TINYGLADE_NANI_NO_HZB)
Texture2D<float> hzb_tex : register(t21, space0);
#endif

StructuredBuffer<FlowerXform> instance_buffer : register(t23, space0);
StructuredBuffer<uint4> dispatch_args_buffer : register(t24, space0);


#if !defined(TINYGLADE_NANI_NO_HZB)
cbuffer $Globals : register(b22, space0)
{
    uint2 hzb_tex_inner_dims;
};
#endif


struct PushConstants
{
    uint world_subset;
    uint draw_list_index_base;
};

[[vk::push_constant]]
PushConstants push_constants;

[numthreads(64, 1, 1)]
void cs_main(uint3 thread_id : SV_DispatchThreadID)
{
    if (dispatch_args_buffer[0].x == 0xDEADBEEFu)
    {
        return;
    }

    uint global_instance_offset = draw_list_info[push_constants.draw_list_index_base].global_instance_offset;
    uint global_instance_index = thread_id.x + global_instance_offset;

    FlowerXform xform = instance_buffer[global_instance_index];
    float4x4 m;
    m[0] = float4(xform.transform.data[0], xform.transform.data[3], xform.transform.data[6], xform.transform.data[9]);
    m[1] = float4(xform.transform.data[1], xform.transform.data[4], xform.transform.data[7], xform.transform.data[10]);
    m[2] = float4(xform.transform.data[2], xform.transform.data[5], xform.transform.data[8], xform.transform.data[11]);
    m[3] = float4(0.0f, 0.0f, 0.0f, 1.0f);
    float3 center = m[3].xyz;

    float3 basis_x = m[0].xyz;
    float3 basis_y = m[1].xyz;
    float3 basis_z = m[2].xyz;
    float scale = sqrt(max(max(dot(basis_x, basis_x), dot(basis_y, basis_y)), dot(basis_z, basis_z)));
    uint mesh_id = global_instance_index % 64u;
    float radius = scale * atlas_part_info[push_constants.world_subset][mesh_id].bounding_sphere_radius;

    float4 view_pos = mul(float4(center, 1.0f), view_constants[0].world_to_view);
    float view_z = view_pos.z;

    uint bucket = clamp(uint(clamp((32.0f - view_z) * 0.0034285f, 0.0f, 1.0f) * 1024.0f), 0u, 1023u);

    Frustum f = extract_frustum(view_constants[0].world_to_view);
    bool visible = cull_frustum_side_planes(f, view_pos.xyz, radius);

#if !defined(TINYGLADE_NANI_NO_HZB)
    if (visible)
    {
        float mip = max(0.0f, ceil(log2(max(2.0f, 2.0f))));
        uint mip_u = uint(mip);
        uint2 coord = min(uint2(0u, 0u) >> (uint2(mip_u, mip_u) & 31u), hzb_tex_inner_dims - 1u);
        float h0 = hzb_tex.Load(uint3(coord, mip_u));
        float h1 = hzb_tex.Load(uint3(coord + uint2(1u, 0u), mip_u));
        float h2 = hzb_tex.Load(uint3(coord + uint2(0u, 1u), mip_u));
        float h3 = hzb_tex.Load(uint3(coord + uint2(1u, 1u), mip_u));
        float hzb_min = min(min(h0, h1), min(h2, h3));
        visible = (view_z + radius) > (1.0f / (hzb_min * -view_constants[0].clip_to_view[3][2]));
    }
#endif

    if (visible)
    {
        cull_append_visible(push_constants.draw_list_index_base, global_instance_index, mesh_id, bucket);
    }
}
