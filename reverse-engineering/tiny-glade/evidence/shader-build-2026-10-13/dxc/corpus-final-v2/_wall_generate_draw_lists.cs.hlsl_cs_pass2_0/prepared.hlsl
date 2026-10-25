#line 1 "shaders/wall/generate_draw_lists.cs.hlsl"

























#line 1 "shaders/wall/generate_draw_lists.cs.hlsl"

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
#line 28 "shaders/wall/generate_draw_lists.cs.hlsl"

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
#line 29 "shaders/wall/generate_draw_lists.cs.hlsl"

#line 1 "shaders/inc/draw_indirect.hlsl"


#ifndef TINYGLADE_INC_DRAW_INDIRECT_HLSL
#define TINYGLADE_INC_DRAW_INDIRECT_HLSL

#line 1 "shaders/inc/draw_indirect_types.hlsl"


#ifndef TINYGLADE_INC_DRAW_INDIRECT_TYPES_HLSL
#define TINYGLADE_INC_DRAW_INDIRECT_TYPES_HLSL



struct DrawIndexedIndirectCommand {
    uint count;
    uint instanceCount;
    uint firstIndex;
    uint vertexOffset;
    uint firstInstance;
};


struct DrawIndirectCommand {
    uint vertexCount;
    uint instanceCount;
    uint firstVertex;
    uint firstInstance;
};

#endif 
#line 6 "shaders/inc/draw_indirect.hlsl"



static const uint DRAW_INDEXED_INDIRECT_STRIDE_BYTES = 20;




uint increment_draw_indexed_indirect_instance_count(
    RWByteAddressBuffer draw_commands,
    uint command_base_offset,
    uint cmd_index,
    uint n)
{
    uint previous;
    draw_commands.InterlockedAdd((command_base_offset + cmd_index) * 20u + 4u, n, previous);
    return previous;
}



uint reserve_draw_indexed_indirect_instances(
    RWByteAddressBuffer draw_commands,
    uint command_base_offset,
    uint cmd_index,
    uint n)
{
    return increment_draw_indexed_indirect_instance_count(draw_commands, command_base_offset, cmd_index, n);
}

#endif 
#line 30 "shaders/wall/generate_draw_lists.cs.hlsl"

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
#line 31 "shaders/wall/generate_draw_lists.cs.hlsl"

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
#line 32 "shaders/wall/generate_draw_lists.cs.hlsl"

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
#line 33 "shaders/wall/generate_draw_lists.cs.hlsl"

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
#line 34 "shaders/wall/generate_draw_lists.cs.hlsl"

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
#line 35 "shaders/wall/generate_draw_lists.cs.hlsl"

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
#line 36 "shaders/wall/generate_draw_lists.cs.hlsl"

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
#line 37 "shaders/wall/generate_draw_lists.cs.hlsl"

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
#line 38 "shaders/wall/generate_draw_lists.cs.hlsl"

#line 1 "shaders/wall/inc/brick_instance.hlsl"




#ifndef TINYGLADE_WALL_INC_BRICK_INSTANCE_HLSL
#define TINYGLADE_WALL_INC_BRICK_INSTANCE_HLSL

#line 1 "shaders/wall/inc/draw_lists.hlsl"





#ifndef TINYGLADE_WALL_INC_DRAW_LISTS_HLSL
#define TINYGLADE_WALL_INC_DRAW_LISTS_HLSL

#ifndef TINYGLADE_AFFINE3PACKED
#define TINYGLADE_AFFINE3PACKED
struct Affine3Packed
{
    float data[12];
};
#endif


struct InstancedWallData
{
    Affine3Packed transform;
    float4 sin_deform_y;
    float3 global_arch_height_vals;
    uint seed;
    uint source_id;
    float2 wallspace_x_range;
    uint flags;
};

#endif 
#line 8 "shaders/wall/inc/brick_instance.hlsl"


#endif 
#line 39 "shaders/wall/generate_draw_lists.cs.hlsl"

#line 1 "shaders/wall/inc/draw_lists.hlsl"





#ifndef TINYGLADE_WALL_INC_DRAW_LISTS_HLSL
#define TINYGLADE_WALL_INC_DRAW_LISTS_HLSL

#ifndef TINYGLADE_AFFINE3PACKED
#define TINYGLADE_AFFINE3PACKED
struct Affine3Packed
{
    float data[12];
};
#endif


struct InstancedWallData
{
    Affine3Packed transform;
    float4 sin_deform_y;
    float3 global_arch_height_vals;
    uint seed;
    uint source_id;
    float2 wallspace_x_range;
    uint flags;
};

#endif 
#line 40 "shaders/wall/generate_draw_lists.cs.hlsl"

#line 1 "shaders/wall/inc/brick_instance_flags.hlsl"




#ifndef TINYGLADE_WALL_INC_BRICK_INSTANCE_FLAGS_HLSL
#define TINYGLADE_WALL_INC_BRICK_INSTANCE_FLAGS_HLSL

#line 1 "shaders/wall/inc/brick_instance.hlsl"




#ifndef TINYGLADE_WALL_INC_BRICK_INSTANCE_HLSL
#define TINYGLADE_WALL_INC_BRICK_INSTANCE_HLSL

#line 1 "shaders/wall/inc/draw_lists.hlsl"





#ifndef TINYGLADE_WALL_INC_DRAW_LISTS_HLSL
#define TINYGLADE_WALL_INC_DRAW_LISTS_HLSL

#ifndef TINYGLADE_AFFINE3PACKED
#define TINYGLADE_AFFINE3PACKED
struct Affine3Packed
{
    float data[12];
};
#endif


struct InstancedWallData
{
    Affine3Packed transform;
    float4 sin_deform_y;
    float3 global_arch_height_vals;
    uint seed;
    uint source_id;
    float2 wallspace_x_range;
    uint flags;
};

#endif 
#line 8 "shaders/wall/inc/brick_instance.hlsl"


#endif 
#line 8 "shaders/wall/inc/brick_instance_flags.hlsl"


#endif 
#line 41 "shaders/wall/generate_draw_lists.cs.hlsl"





struct FrameConstants
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




#define WALL_BRICK_FLAG_BEVEL 0x80u
#define WALL_BRICK_FLAG_PASS2 0x80000000u
#define WALL_BRICK_INSTANCE_COUNT_MASK 0xFFFFFFu
#define WALL_BRICK_DRAW_LIST_BUCKET_CAPACITY 0x200000u

RWStructuredBuffer<DrawIndexedIndirectCommand> __alias_draw_commands : register(u13, space0);

cbuffer $Globals : register(b14, space0)
{
    int draw_indirect_buffer_offset;
    uint2 hzb_tex_inner_dims;
};

StructuredBuffer<FrameConstants> frame_constants_storage_buffer : register(t1, space1);
StructuredBuffer<InstancedWallData> brick_instance_data : register(t15, space0);
StructuredBuffer<uint> prev_visibility_bitmask : register(t16, space0);
RWStructuredBuffer<uint> __alias_next_visibility_bitmask : register(u17, space0);
RWStructuredBuffer<uint> __alias_brick_draw_list : register(u18, space0);
Texture2D<float> hzb_tex : register(t19, space0);



uint wall_brick_draw_list_slot(uint bucket, uint prev)
{
    return bucket * WALL_BRICK_DRAW_LIST_BUCKET_CAPACITY + prev;
}


static float3x4 load_brick_transform(uint inst)
{
    Affine3Packed affine = brick_instance_data[inst].transform;
    return transpose(float4x3(
        float3(affine.data[0], affine.data[1], affine.data[2]),
        float3(affine.data[3], affine.data[4], affine.data[5]),
        float3(affine.data[6], affine.data[7], affine.data[8]),
        float3(affine.data[9], affine.data[10], affine.data[11])));
}

struct PushConstants
{
    uint first_instance;
    uint instance_count_and_flags;
};

[[vk::push_constant]]
PushConstants push_constants;


[numthreads(64, 1, 1)]
void cs_pass1(uint3 dispatchThreadID : SV_DispatchThreadID)
{
    if (dispatchThreadID.x >= (push_constants.instance_count_and_flags & WALL_BRICK_INSTANCE_COUNT_MASK))
    {
        return;
    }
    uint inst = push_constants.first_instance + dispatchThreadID.x;

    float3x4 brick_mat = load_brick_transform(inst);
    float half_extent = 0.5f * length(mul(float3x4(abs(brick_mat[0]), abs(brick_mat[1]), abs(brick_mat[2])), float4(1.0f, 1.0f, 1.0f, 0.0f)).xxx);

    ViewConstants view = frame_constants_storage_buffer[0].view_constants[0];
    float4x4 world_to_view = view.world_to_view;
    float4x4 view_to_clip = view.view_to_clip;

    
    float4 plane_lr = float4(-view_to_clip[0].x, view_to_clip[0].y, 0.0f, view_to_clip[0].w);
    float4 plane_tb = float4(view_to_clip[1].x, view_to_clip[1].y, 0.0f, view_to_clip[1].w);
    float4 plane_origin = float4(view_to_clip[3].x, view_to_clip[3].y, -1.0f, view_to_clip[3].w);
    float4 planes[4];
    planes[0] = plane_origin + plane_lr;
    planes[1] = plane_origin - plane_lr;
    planes[2] = plane_origin + plane_tb;
    planes[3] = plane_origin - plane_tb;
    [unroll]
    for (uint p = 0; p < 4u; ++p)
    {
        planes[p] /= length(planes[p].xyz).xxxx;
    }

    float4 brick_center = mul(world_to_view, float4(mul(brick_mat, float4(0.0f, 0.0f, 0.0f, 1.0f)), 1.0f));
    float center_depth = -brick_center.z;

    bool in_frustum;
    if ((center_depth + half_extent) > view_to_clip[2].w)
    {
        in_frustum = (center_depth - half_extent) < 3.4028234663852886e+38f;
    }
    else
    {
        in_frustum = false;
    }
    [unroll]
    for (uint q = 0; q < 4u && in_frustum; ++q)
    {
        in_frustum = dot(float4(brick_center.xyz, 1.0f), planes[q]) > (half_extent * (-0.5f));
    }

    uint bit = 1u << (inst % 32u);

    
    if (in_frustum && (prev_visibility_bitmask[inst / 32u] & bit) != 0u)
    {
        InterlockedOr(__alias_next_visibility_bitmask[inst / 32u], bit);

        
        uint lod;
        if ((brick_instance_data[inst].flags & WALL_BRICK_FLAG_BEVEL) != 0u)
        {
            lod = 0u;
        }
        else
        {
            float proj_brick_height = (half_extent / center_depth)
                * ((0.5f * view_to_clip[1].y) * float(clamp(frame_constants_storage_buffer[0].inner_rendering_resolution.y, 1080u, 2160u)));
            lod = uint(proj_brick_height > 30.0f ? 0 : 1);
        }
        uint bucket = clamp(lod, 0u, 1u) + (2u * uint((push_constants.instance_count_and_flags & WALL_BRICK_FLAG_PASS2) != 0u));
        uint prev;
        InterlockedAdd(__alias_draw_commands[uint(draw_indirect_buffer_offset) + bucket].instanceCount, 1u, prev);
        __alias_brick_draw_list[wall_brick_draw_list_slot(bucket, prev)] = inst;
    }

    
    
    
    float4x4 ring_mat = frame_constants_storage_buffer[0].view_constants[2].world_to_sample;
    float4x4 ring_view = frame_constants_storage_buffer[0].view_constants[2].view_to_sample;
    float ring = half_extent * (2.0f * ring_view[0].x);
    float2 proj_xy = abs(mul(ring_mat, brick_center).xy);
    float2 ring_lo0 = (-ring).xx;
    float2 ring_hi0 = (1.0f + ring).xx;
    uint ring_lod = (any(bool2(proj_xy.x >= ring_lo0.x, proj_xy.y >= ring_lo0.y)) ? all(bool2(proj_xy.x <= ring_hi0.x, proj_xy.y <= ring_hi0.y)) : false) ? 0u : 3u;
    float2 ring_lo1 = (1.0f - ring).xx;
    float2 ring_hi1 = (2.0f + ring).xx;
    bool in_ring1 = any(bool2(proj_xy.x >= ring_lo1.x, proj_xy.y >= ring_lo1.y)) ? all(bool2(proj_xy.x <= ring_hi1.x, proj_xy.y <= ring_hi1.y)) : false;
    uint ring_lod_min;
    if (in_ring1)
    {
        ring_lod_min = min(ring_lod, 1u);
    }
    else
    {
        ring_lod_min = ring_lod;
    }
    uint ring_lod_max = uint(in_ring1);

    float2 ring_lo2 = (2.0f - ring).xx;
    float2 ring_hi2 = (7.0f + ring).xx;
    bool in_ring2 = any(bool2(proj_xy.x >= ring_lo2.x, proj_xy.y >= ring_lo2.y)) ? all(bool2(proj_xy.x <= ring_hi2.x, proj_xy.y <= ring_hi2.y)) : false;
    if (in_ring2)
    {
        ring_lod_max = max(ring_lod_max, 2u);
        ring_lod_min = min(ring_lod_min, 2u);
    }
    [unroll]
    for (uint r = ring_lod_min; r <= ring_lod_max; ++r)
    {
        uint ring_bucket = 4u + r;
        uint prev_ring;
        InterlockedAdd(__alias_draw_commands[uint(draw_indirect_buffer_offset) + ring_bucket].instanceCount, 1u, prev_ring);
        __alias_brick_draw_list[wall_brick_draw_list_slot(ring_bucket, prev_ring)] = inst;
    }
}


[numthreads(64, 1, 1)]
void cs_pass2(uint3 dispatchThreadID : SV_DispatchThreadID)
{
    if (dispatchThreadID.x >= (push_constants.instance_count_and_flags & WALL_BRICK_INSTANCE_COUNT_MASK))
    {
        return;
    }
    uint inst = push_constants.first_instance + dispatchThreadID.x;

    float3x4 brick_mat = load_brick_transform(inst);
    float half_extent = 0.5f * length(mul(float3x4(abs(brick_mat[0]), abs(brick_mat[1]), abs(brick_mat[2])), float4(1.0f, 1.0f, 1.0f, 0.0f)).xxx);

    ViewConstants view = frame_constants_storage_buffer[0].view_constants[0];
    float4x4 world_to_view = view.world_to_view;
    float4x4 view_to_clip = view.view_to_clip;

    float4 plane_lr = float4(-view_to_clip[0].x, view_to_clip[0].y, 0.0f, view_to_clip[0].w);
    float4 plane_tb = float4(view_to_clip[1].x, view_to_clip[1].y, 0.0f, view_to_clip[1].w);
    float4 plane_origin = float4(view_to_clip[3].x, view_to_clip[3].y, -1.0f, view_to_clip[3].w);
    float4 planes[4];
    planes[0] = plane_origin + plane_lr;
    planes[1] = plane_origin - plane_lr;
    planes[2] = plane_origin + plane_tb;
    planes[3] = plane_origin - plane_tb;
    [unroll]
    for (uint p = 0; p < 4u; ++p)
    {
        planes[p] /= length(planes[p].xyz).xxxx;
    }

    float4 brick_center = mul(world_to_view, float4(mul(brick_mat, float4(0.0f, 0.0f, 0.0f, 1.0f)), 1.0f));
    float center_depth = -brick_center.z;

    bool in_frustum;
    if ((center_depth + half_extent) > view_to_clip[2].w)
    {
        in_frustum = (center_depth - half_extent) < 3.4028234663852886e+38f;
    }
    else
    {
        in_frustum = false;
    }
    [unroll]
    for (uint q = 0; q < 4u && in_frustum; ++q)
    {
        in_frustum = dot(float4(brick_center.xyz, 1.0f), planes[q]) > (half_extent * (-0.5f));
    }

    
    float4x4 mvp = frame_constants_storage_buffer[0].view_constants[0].world_to_sample;
    float4 axis_x = mul(mvp, float4(mul(brick_mat, float4(1.0f, 0.0f, 0.0f, 0.0f)), 0.0f));
    float4 axis_y = mul(mvp, float4(mul(brick_mat, float4(0.0f, 1.0f, 0.0f, 0.0f)), 0.0f));
    float4 axis_z = mul(mvp, float4(mul(brick_mat, float4(0.0f, 0.0f, 1.0f, 0.0f)), 0.0f));
    float4 corner0 = mul(mvp, float4(mul(brick_mat, float4(-0.5f, -0.5f, -0.5f, 1.0f)), 1.0f));

    float4 corners[8];
    corners[0] = corner0;
    corners[1] = corner0 + axis_z;
    corners[2] = corner0 + axis_y;
    corners[3] = corner0 + axis_y + axis_z;
    corners[4] = corner0 + axis_x;
    corners[5] = corner0 + axis_x + axis_z;
    corners[6] = corner0 + axis_x + axis_y;
    corners[7] = corner0 + axis_x + axis_y + axis_z;

    float4 bounds;
    float max_depth;
    bool has_bounds;
    if (in_frustum)
    {
        float w[8];
        [unroll]
        for (uint c = 0; c < 8u; ++c)
        {
            w[c] = corners[c].w;
        }
        if (max(max(max(w[0], w[1]), max(w[2], w[3])), max(max(w[4], w[5]), max(w[6], w[7]))) < view_to_clip[2].w)
        {
            bounds = 0.0f.xxxx;
            max_depth = 0.0f;
            has_bounds = false;
        }
        else
        {
            float2 ndc_min = min(min(min(corners[0].xy / w[0].xx, corners[1].xy / w[1].xx), min(corners[2].xy / w[2].xx, corners[3].xy / w[3].xx)), min(min(corners[4].xy / w[4].xx, corners[5].xy / w[5].xx), min(corners[6].xy / w[6].xx, corners[7].xy / w[7].xx)));
            float2 ndc_max = max(max(max(corners[0].xy / w[0].xx, corners[1].xy / w[1].xx), max(corners[2].xy / w[2].xx, corners[3].xy / w[3].xx)), max(max(corners[4].xy / w[4].xx, corners[5].xy / w[5].xx), max(corners[6].xy / w[6].xx, corners[7].xy / w[7].xx)));
            bounds = (float4(ndc_min.x, ndc_min.y, ndc_max.x, ndc_max.y).xwzy * float4(0.5f, -0.5f, 0.5f, -0.5f)) + 0.5f.xxxx;
            max_depth = -min(min(min(w[0], w[1]), min(w[2], w[3])), min(min(w[4], w[5]), min(w[6], w[7])));
            has_bounds = true;
        }
    }
    else
    {
        bounds = 0.0f.xxxx;
        max_depth = 0.0f;
        has_bounds = false;
    }

    float4 hzb_dims = float4(hzb_tex_inner_dims.xyxy);
    float4 uv = clamp(bounds * hzb_dims, 0.0f.xxxx, hzb_dims - 1.0f.xxxx);
    uint mip = uint(max(0.0f, ceil(log2(max(uv.z - uv.x, uv.w - uv.y)))));

    bool occluded;
    if (has_bounds)
    {
        uint2 mip_mask = mip.xx & uint2(31u, 31u);
        uint2 uv_min = uint2(uv.xy) >> mip_mask;
        uint2 uv_max = uint2(uv.zw) >> mip_mask;
        float4x4 clip_to_view = frame_constants_storage_buffer[0].view_constants[0].clip_to_view;
        float hzb_depth = min(min(hzb_tex.Load(int3(uv_min, mip)).x, hzb_tex.Load(int3(uint2(uv_min.x, uv_max.y), mip)).x), min(hzb_tex.Load(int3(uint2(uv_max.x, uv_min.y), mip)).x, hzb_tex.Load(int3(uv_max, mip)).x));
        occluded = max_depth > (1.0f / (hzb_depth * (-clip_to_view[3].z)));
    }
    else
    {
        occluded = has_bounds;
    }
    if (!occluded)
    {
        return;
    }

    uint bit = 1u << (inst % 32u);
    InterlockedOr(__alias_next_visibility_bitmask[inst / 32u], bit);

    if ((prev_visibility_bitmask[inst / 32u] & bit) == 0u)
    {
        uint lod;
        if ((brick_instance_data[inst].flags & WALL_BRICK_FLAG_BEVEL) != 0u)
        {
            lod = 0u;
        }
        else
        {
            float proj_brick_height = (half_extent / center_depth)
                * ((0.5f * view_to_clip[1].y) * float(clamp(frame_constants_storage_buffer[0].inner_rendering_resolution.y, 1080u, 2160u)));
            lod = uint(proj_brick_height > 30.0f ? 0 : 1);
        }
        uint bucket = clamp(lod, 0u, 1u) + (2u * uint((push_constants.instance_count_and_flags & WALL_BRICK_FLAG_PASS2) != 0u));
        uint prev;
        InterlockedAdd(__alias_draw_commands[uint(draw_indirect_buffer_offset) + bucket].instanceCount, 1u, prev);
        __alias_brick_draw_list[wall_brick_draw_list_slot(bucket, prev)] = inst;
    }
}
