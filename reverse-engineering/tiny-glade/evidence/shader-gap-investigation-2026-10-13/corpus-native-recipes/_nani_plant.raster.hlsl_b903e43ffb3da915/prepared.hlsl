#define SHADER_VARIANT_DEPTH_ONLY 0
#define TGV_b903e43ffb3da915 1
#line 1 "shaders/nani_plant.raster.hlsl"


#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)))







Texture2D terrain_heightmap : register(t63);

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
#line 13 "shaders/nani_plant.raster.hlsl"

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
#line 14 "shaders/nani_plant.raster.hlsl"

#line 1 "shaders/inc/current_view.hlsl"





#ifndef TINYGLADE_INC_CURRENT_VIEW_HLSL
#define TINYGLADE_INC_CURRENT_VIEW_HLSL

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
#line 9 "shaders/inc/current_view.hlsl"


uint current_view_index;


ViewConstants GetCurrentViewConstants()
{
    return view_constants[current_view_index];
}

#endif 
#line 15 "shaders/nani_plant.raster.hlsl"

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
#line 16 "shaders/nani_plant.raster.hlsl"

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
#line 17 "shaders/nani_plant.raster.hlsl"

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
#line 18 "shaders/nani_plant.raster.hlsl"

#line 1 "shaders/inc/utils.hlsl"





#ifndef TINYGLADE_INC_UTILS_HLSL
#define TINYGLADE_INC_UTILS_HLSL

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
#line 9 "shaders/inc/utils.hlsl"







#ifndef TINYGLADE_AFFINE3PACKED
#define TINYGLADE_AFFINE3PACKED
struct Affine3Packed
{
    float data[12];
};
#endif



float4x4 affine_basis(float3 forward)
{
    float3 f = normalize(forward);
    float3 up = float3(0.0f, 1.0f, 0.0f);
    float3 right = cross(f, up);
    float4x4 m = float4x4(
        float4(f, 0.0f),
        float4(up, 0.0f),
        float4(right, 0.0f),
        float4(0.0f, 0.0f, 0.0f, 1.0f));
    return transpose(m);
}


float4x4 affine_translation(float3 pos)
{
    return float4x4(
        float4(1.0f, 0.0f, 0.0f, pos.x),
        float4(0.0f, 1.0f, 0.0f, pos.y),
        float4(0.0f, 0.0f, 1.0f, pos.z),
        float4(0.0f, 0.0f, 0.0f, 1.0f));
}


float4x4 affine_scale(float3 s)
{
    return float4x4(
        float4(s.x, 0.0f, 0.0f, 0.0f),
        float4(0.0f, s.y, 0.0f, 0.0f),
        float4(0.0f, 0.0f, s.z, 0.0f),
        float4(0.0f, 0.0f, 0.0f, 1.0f));
}



Affine3Packed pack_affine(float4x4 m)
{
    Affine3Packed o;
    o.data[0] = m[0].x; o.data[1] = m[1].x; o.data[2] = m[2].x;
    o.data[3] = m[0].y; o.data[4] = m[1].y; o.data[5] = m[2].y;
    o.data[6] = m[0].z; o.data[7] = m[1].z; o.data[8] = m[2].z;
    o.data[9] = m[0].w; o.data[10] = m[1].w; o.data[11] = m[2].w;
    return o;
}


float4x4 unpack_affine(Affine3Packed a)
{
    float4x4 m;
    m[0] = float4(a.data[0], a.data[3], a.data[6], a.data[9]);
    m[1] = float4(a.data[1], a.data[4], a.data[7], a.data[10]);
    m[2] = float4(a.data[2], a.data[5], a.data[8], a.data[11]);
    m[3] = float4(0.0f, 0.0f, 0.0f, 1.0f);
    return m;
}


Affine3Packed compose_affine_packed(float3 pos, float3 forward, float3 scale)
{
    return pack_affine(mul(affine_translation(pos), mul(affine_basis(forward), affine_scale(scale))));
}

#endif 
#line 19 "shaders/nani_plant.raster.hlsl"

#line 1 "shaders/nani/inc/user.hlsl"





#ifndef TINYGLADE_NANI_INC_USER_HLSL
#define TINYGLADE_NANI_INC_USER_HLSL

#line 1 "shaders/inc/hsv.hlsl"










float3 rgb2hsv(float3 c)
{
    float4 K = float4(0.0f, -1.0f / 3.0f, 2.0f / 3.0f, -1.0f);
    float4 p = lerp(float4(c.zy, K.wz), float4(c.yz, K.xy), step(c.z, c.y));
    float4 q = lerp(float4(p.xyw, c.x), float4(c.x, p.yzx), step(p.x, c.x));
    float d = q.x - min(q.w, q.y);
    float e = 1.0e-10f;
    return float3(abs(q.z + (q.w - q.y) / (6.0f * d + e)), d / (q.x + e), q.x);
}

float3 hsv2rgb(float3 c)
{
    float3 p = abs(float3(abs(c.x - 3.0f) - 1.0f, 2.0f - abs(c.x - 2.0f), 2.0f - abs(c.x - 4.0f)));
    p = clamp(p, 0.0f, 1.0f);
    return lerp(float3(1.0f, 1.0f, 1.0f), p, c.y) * c.z;
}
#line 9 "shaders/nani/inc/user.hlsl"

#line 1 "shaders/stairs_floor.inc.hlsl"








#ifndef TINYGLADE_STAIRS_FLOOR_INC_HLSL
#define TINYGLADE_STAIRS_FLOOR_INC_HLSL


struct NaniTrimeshChunk
{
    uint64_t gpu_address;
    uint vertex_index_count;
    uint vertex_byte_offset;
    float4 bounding_sphere;
};


struct StairsFloorInstanceData
{
    NaniTrimeshChunk mesh;
    uint color_id;
    uint pad0;
    uint pad1;
    uint pad2;
};

#endif 
#line 10 "shaders/nani/inc/user.hlsl"

#line 1 "shaders/wall/inc/brick_vs_out.hlsl"




#ifndef TINYGLADE_WALL_INC_BRICK_VS_OUT_HLSL
#define TINYGLADE_WALL_INC_BRICK_VS_OUT_HLSL

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
#line 8 "shaders/wall/inc/brick_vs_out.hlsl"


#endif 
#line 11 "shaders/nani/inc/user.hlsl"


#endif 
#line 20 "shaders/nani_plant.raster.hlsl"

#line 1 "shaders/inc/perlin.hlsl"












#line 1 "shaders/inc/hash.hlsl"







#ifndef TINYGLADE_INC_HASH_HLSL
#define TINYGLADE_INC_HASH_HLSL

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
#line 11 "shaders/inc/hash.hlsl"



uint hash(uint x)
{
    x += (x << 10u);
    x ^= (x >> 6u);
    x += (x << 3u);
    x ^= (x >> 11u);
    x += (x << 15u);
    return x;
}


float rand01(uint x)
{
    return asfloat((x & 0x7FFFFFu) | 0x3F800000u) - 1.0f;
}


float hash_angle(uint x)
{
    return frac(0.9f + (float(x) * GOLDEN_RATIO)) * TWO_PI;
}


float hash01(float x)
{
    return frac(sin(x) * 43758.5453f);
}

#endif 
#line 13 "shaders/inc/perlin.hlsl"






uint hash(uint2 p)
{
    uint y = p.y;
    y += (y << 10u);
    y ^= (y >> 6u);
    y += (y << 3u);
    y ^= (y >> 11u);
    uint h = ((p.x * 1664525u) + (y + (y << 15u))) + 1013904223u;
    h *= 1664525u;
    h ^= (h >> 11u);
    h ^= (h << 7u) & 2636928640u;
    h ^= (h << 15u) & 4022730752u;
    return h;
}

float incb_perlin_hash01(uint h)
{
    return asfloat(((h ^ (h >> 18u)) & 0x7FFFFFu) | 0x3F800000u) - 1.0f;
}

float perlin(float2 p)
{
    int2 cell = int2(floor(p));
    float2 f = frac(p);
    float2 u = f * f * (3.0f - 2.0f * f);

    float n00 = incb_perlin_hash01(hash(uint2(cell + int2(0, 0))));
    float n10 = incb_perlin_hash01(hash(uint2(cell + int2(1, 0))));
    float n01 = incb_perlin_hash01(hash(uint2(cell + int2(0, 1))));
    float n11 = incb_perlin_hash01(hash(uint2(cell + int2(1, 1))));

    
    return (1.0f - u.y) * ((n00 * (1.0f - u.x)) + (n10 * u.x))
         + (n01 * (1.0f - u.x)) * u.y + (n11 * u.x) * u.y;
}
#line 21 "shaders/nani_plant.raster.hlsl"

#line 1 "shaders/inc/wind.hlsl"


#ifndef TINYGLADE_INC_WIND_HLSL
#define TINYGLADE_INC_WIND_HLSL



float wind_noise(float2 p)
{
    uint2 ip = uint2(int2(p * 0.5));
    float2 fp = frac(p * 0.5);
    float2 u = fp * fp * (3.0 - 2.0 * fp);
    uint h00 = (ip.x * 374761393u) + (ip.y * 668265263u);
    uint h01 = ((ip.x + 1u) * 374761393u) + (ip.y * 668265263u);
    uint h10 = (ip.x * 374761393u) + ((ip.y + 1u) * 668265263u);
    uint h11 = ((ip.x + 1u) * 374761393u) + ((ip.y + 1u) * 668265263u);
    float n00 = float(h00 & 0x00FFFFFFu) / 16777215.0;
    float n01 = float(h01 & 0x00FFFFFFu) / 16777215.0;
    float n10 = float(h10 & 0x00FFFFFFu) / 16777215.0;
    float n11 = float(h11 & 0x00FFFFFFu) / 16777215.0;
    return lerp(lerp(n00, n01, u.x), lerp(n10, n11, u.x), u.y) * 2.0 - 1.0;
}




float3 evaluate_wind(float3 world_pos, float seed, float amplitude)
{
    float t = game_time_sec + game_time_frac;
    float2 xz = world_pos.xz;
    float gust = wind_noise(xz * 0.02 + float2(t * 0.03, 0.0) + float2(seed * 13.0, 0.0));
    float flutter = wind_noise(xz * 0.3 + float2(t * 0.5, seed) );
    float bend = saturate(gust * 0.5 + 0.5) * amplitude;
    
    float3 disp = float3(-gust * bend, flutter * 0.05 * amplitude, 0.0);
    disp *= smoothstep(0.0, 1.0, max(world_pos.y, 0.0));
    return disp;
}

#endif 
#line 22 "shaders/nani_plant.raster.hlsl"

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
#line 23 "shaders/nani_plant.raster.hlsl"

#line 1 "shaders/inc/material/plant_material.inc.hlsl"


#ifndef TINYGLADE_INC_MATERIAL_PLANT_MATERIAL_HLSL
#define TINYGLADE_INC_MATERIAL_PLANT_MATERIAL_HLSL

#line 1 "shaders/inc/material_eval.inc.hlsl"


#ifndef TINYGLADE_INC_MATERIAL_EVAL_HLSL
#define TINYGLADE_INC_MATERIAL_EVAL_HLSL



struct MaterialEvalData {
    float3 albedo;
    float3 normal;
    float roughness;
    float opacity;
    float3 emissive;
};



float3 unpack_gbuffer_normal(uint packed_normal)
{
    float3 n = float3(float(packed_normal & 255u),
                      float((packed_normal >> 8u) & 255u),
                      float((packed_normal >> 16u) & 255u)) * (1.0 / 255.0);
    n = n * 2.0 - 1.0;
    return normalize(n);
}


void init_material_eval_data(out MaterialEvalData d)
{
    d.albedo = float3(1.0, 1.0, 1.0);
    d.normal = float3(0.0, 1.0, 0.0);
    d.roughness = 1.0;
    d.opacity = 1.0;
    d.emissive = float3(0.0, 0.0, 0.0);
}

#endif 
#line 6 "shaders/inc/material/plant_material.inc.hlsl"



struct PlantMaterialParams {
    float3 base_color;
    float3 flower_color;
    float roughness;
    float translucency;
    uint is_flower;
};

PlantMaterialParams init_plant_material(float3 base_color, float3 flower_color, float roughness, uint is_flower)
{
    PlantMaterialParams p;
    p.base_color = base_color;
    p.flower_color = flower_color;
    p.roughness = saturate(roughness);
    p.translucency = 1.0;
    p.is_flower = is_flower;
    return p;
}

#endif 
#line 24 "shaders/nani_plant.raster.hlsl"

#line 1 "shaders/inc/material/solid_material.inc.hlsl"


#ifndef TINYGLADE_INC_MATERIAL_SOLID_MATERIAL_HLSL
#define TINYGLADE_INC_MATERIAL_SOLID_MATERIAL_HLSL


struct SolidMaterialParams {
    float3 albedo;
    float roughness;
    uint flags; 
};


SolidMaterialParams init_solid_material(float3 albedo, float roughness, uint flags)
{
    SolidMaterialParams p;
    p.albedo = albedo;
    p.roughness = saturate(roughness);
    p.flags = flags;
    return p;
}

#endif 
#line 25 "shaders/nani_plant.raster.hlsl"

#line 1 "shaders/inc/material_eval.inc.hlsl"


#ifndef TINYGLADE_INC_MATERIAL_EVAL_HLSL
#define TINYGLADE_INC_MATERIAL_EVAL_HLSL



struct MaterialEvalData {
    float3 albedo;
    float3 normal;
    float roughness;
    float opacity;
    float3 emissive;
};



float3 unpack_gbuffer_normal(uint packed_normal)
{
    float3 n = float3(float(packed_normal & 255u),
                      float((packed_normal >> 8u) & 255u),
                      float((packed_normal >> 16u) & 255u)) * (1.0 / 255.0);
    n = n * 2.0 - 1.0;
    return normalize(n);
}


void init_material_eval_data(out MaterialEvalData d)
{
    d.albedo = float3(1.0, 1.0, 1.0);
    d.normal = float3(0.0, 1.0, 0.0);
    d.roughness = 1.0;
    d.opacity = 1.0;
    d.emissive = float3(0.0, 0.0, 0.0);
}

#endif 
#line 26 "shaders/nani_plant.raster.hlsl"

#line 1 "shaders/inc/brdf.hlsl"


#ifndef TINYGLADE_INC_BRDF_HLSL
#define TINYGLADE_INC_BRDF_HLSL



float distribution_ggx(float NdotH, float alphaSq)
{
    float d = NdotH * NdotH * (alphaSq - 1.0) + 1.0;
    return alphaSq / (3.14159265358979323846 * d * d);
}


float visibility_smith_ggx(float NdotL, float NdotV, float alphaSq)
{
    float l2 = NdotL * NdotL;
    float v2 = NdotV * NdotV;
    return (2.0 * NdotL * NdotV) /
        (NdotL * sqrt((1.0 - alphaSq) * v2 + alphaSq) +
         NdotV * sqrt((1.0 - alphaSq) * l2 + alphaSq));
}


float3 fresnel_schlick(float cosTheta, float3 F0)
{
    float c = 1.0 - saturate(cosTheta);
    return F0 + (1.0 - F0) * (c * c * c * c * c);
}



float diffuse_ggx(float NdotL, float NdotV, float NdotH, float roughness)
{
    float r2 = roughness * roughness;
    float f = (2.0 * r2 * NdotH * NdotH) - 0.5;
    float omL = 1.0 - saturate(NdotL);
    float omV = 1.0 - saturate(NdotV);
    return ((1.0 + f * pow(omL, 5.0)) * (1.0 + f * pow(omV, 5.0))) * (1.0 / 3.14159265358979323846);
}


float3 cook_torrance_specular(float NdotL, float NdotV, float NdotH, float HdotL, float roughness)
{
    float alphaSq = roughness * roughness;
    alphaSq = alphaSq * alphaSq; 
    float3 F = fresnel_schlick(HdotL, float3(0.04, 0.04, 0.04));
    float D = distribution_ggx(NdotH, alphaSq);
    float G = visibility_smith_ggx(NdotL, NdotV, alphaSq);
    return (F * D * G) / max(4.0 * NdotV * NdotL, 1e-6);
}


float3 evaluate_brdf(float3 albedo, float roughness, float3 N, float3 V, float3 L)
{
    float NdotL = saturate(dot(N, L));
    float NdotV = saturate(dot(N, V));
    float3 H = normalize(L + V);
    float NdotH = saturate(dot(N, H));
    float HdotL = saturate(dot(H, L));
    float3 diffuse = albedo * diffuse_ggx(NdotL, NdotV, NdotH, roughness);
    float3 specular = cook_torrance_specular(NdotL, NdotV, NdotH, HdotL, roughness);
    return diffuse + specular;
}

#endif 
#line 27 "shaders/nani_plant.raster.hlsl"

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
#line 28 "shaders/nani_plant.raster.hlsl"

#line 1 "shaders/inc/lighting.hlsl"


#ifndef TINYGLADE_INC_LIGHTING_HLSL
#define TINYGLADE_INC_LIGHTING_HLSL

#line 1 "shaders/inc/sun.inc.hlsl"


#ifndef TINYGLADE_INC_SUN_HLSL
#define TINYGLADE_INC_SUN_HLSL

#line 1 "shaders/inc/color_library.hlsl"


#ifndef TINYGLADE_INC_COLOR_LIBRARY_HLSL
#define TINYGLADE_INC_COLOR_LIBRARY_HLSL



struct ColorLibrary {
    float4 temp_color_1;
    float4 temp_color_2;
    float4 temp_color_3;
    float4 roof_color_1;
    float4 roof_color_2;
    float4 sky_low_day;
    float4 sky_high_day;
    float4 sun_tint_day;
    float4 sky_low_dusk;
    float4 sky_high_dusk;
    float4 sun_tint_dusk;
    float4 indirect_light_tint;
    float4 fence_color_1;
    float4 fence_color_2;
    float4 flower_color_1;
    float4 flower_color_2;
    float4 canopy_color_1;
    float4 canopy_color_2;
    float4 grass_1;
    float4 grass_2;
    float4 path;
    float4 sky_base;
    float4 sky_horizon;
    float4 sky_clouds_1;
    float4 sky_clouds_2;
    float4 terrain_1;
    float4 terrain_2;
    float4 terrain_flattened;
    float4 wall_1;
    float4 wall_2;
    float4 wall_moss;
    float4 wall_bricks;
    float4 ivy_branch;
    float4 ivy_leaf_1;
    float4 ivy_leaf_2;
    float4 terrain_highlight;
    float4 night_tint;
};






#ifndef TINYGLADE_COLOR_LIBRARY_REGISTER
#define TINYGLADE_COLOR_LIBRARY_REGISTER b14
#endif
cbuffer color_library : register(TINYGLADE_COLOR_LIBRARY_REGISTER, space0) {
    ColorLibrary color_library;
};

#endif 
#line 6 "shaders/inc/sun.inc.hlsl"



float3 get_sun_color()
{
    return sun_color.xyz;
}

float3 get_sky_low_color()
{
    return sky_low_color.xyz;
}

float3 get_sky_high_color()
{
    return sky_high_color.xyz;
}

float3 get_sun_direction()
{
    return dir_to_sun;
}


float3 get_sky_color(float3 day_low, float3 day_high, float3 dusk_low, float3 dusk_high)
{
    float t = saturate(time_of_day);
    float3 low = lerp(day_low, dusk_low, t);
    float3 high = lerp(day_high, dusk_high, t);
    return low;
}

#endif 
#line 6 "shaders/inc/lighting.hlsl"

#line 1 "shaders/inc/brdf.hlsl"


#ifndef TINYGLADE_INC_BRDF_HLSL
#define TINYGLADE_INC_BRDF_HLSL



float distribution_ggx(float NdotH, float alphaSq)
{
    float d = NdotH * NdotH * (alphaSq - 1.0) + 1.0;
    return alphaSq / (3.14159265358979323846 * d * d);
}


float visibility_smith_ggx(float NdotL, float NdotV, float alphaSq)
{
    float l2 = NdotL * NdotL;
    float v2 = NdotV * NdotV;
    return (2.0 * NdotL * NdotV) /
        (NdotL * sqrt((1.0 - alphaSq) * v2 + alphaSq) +
         NdotV * sqrt((1.0 - alphaSq) * l2 + alphaSq));
}


float3 fresnel_schlick(float cosTheta, float3 F0)
{
    float c = 1.0 - saturate(cosTheta);
    return F0 + (1.0 - F0) * (c * c * c * c * c);
}



float diffuse_ggx(float NdotL, float NdotV, float NdotH, float roughness)
{
    float r2 = roughness * roughness;
    float f = (2.0 * r2 * NdotH * NdotH) - 0.5;
    float omL = 1.0 - saturate(NdotL);
    float omV = 1.0 - saturate(NdotV);
    return ((1.0 + f * pow(omL, 5.0)) * (1.0 + f * pow(omV, 5.0))) * (1.0 / 3.14159265358979323846);
}


float3 cook_torrance_specular(float NdotL, float NdotV, float NdotH, float HdotL, float roughness)
{
    float alphaSq = roughness * roughness;
    alphaSq = alphaSq * alphaSq; 
    float3 F = fresnel_schlick(HdotL, float3(0.04, 0.04, 0.04));
    float D = distribution_ggx(NdotH, alphaSq);
    float G = visibility_smith_ggx(NdotL, NdotV, alphaSq);
    return (F * D * G) / max(4.0 * NdotV * NdotL, 1e-6);
}


float3 evaluate_brdf(float3 albedo, float roughness, float3 N, float3 V, float3 L)
{
    float NdotL = saturate(dot(N, L));
    float NdotV = saturate(dot(N, V));
    float3 H = normalize(L + V);
    float NdotH = saturate(dot(N, H));
    float HdotL = saturate(dot(H, L));
    float3 diffuse = albedo * diffuse_ggx(NdotL, NdotV, NdotH, roughness);
    float3 specular = cook_torrance_specular(NdotL, NdotV, NdotH, HdotL, roughness);
    return diffuse + specular;
}

#endif 
#line 7 "shaders/inc/lighting.hlsl"




float3 evaluate_sun_light(float3 albedo, float roughness, float3 normal, float3 view_dir,
                          float3 sun_dir, float shadow)
{
    float NdotL = saturate(dot(normal, sun_dir));
    float NdotV = saturate(dot(normal, view_dir));
    float3 H = normalize(view_dir + sun_dir);
    float NdotH = saturate(dot(normal, H));
    float HdotL = saturate(dot(H, sun_dir));
    float3 diffuse = albedo * diffuse_ggx(NdotL, NdotV, NdotH, roughness);
    float3 specular = cook_torrance_specular(NdotL, NdotV, NdotH, HdotL, roughness);
    return get_sun_color() * (diffuse + specular) * NdotL * shadow;
}



float3 evaluate_sky_ambient(float3 normal)
{
    float up = saturate(normal.y) * 0.5 + 0.5;
    float3 sky = lerp(get_sky_low_color(), get_sky_high_color(), up);
    return sky * color_library.indirect_light_tint.xyz;
}


float3 evaluate_lighting(float3 albedo, float roughness, float3 normal, float3 view_dir,
                        float3 world_pos, float shadow, float ao)
{
    float3 sun = evaluate_sun_light(albedo, roughness, normal, view_dir, get_sun_direction(), shadow);
    float3 ambient = evaluate_sky_ambient(normal) * albedo * ao;
    return sun + ambient;
}

#endif 
#line 29 "shaders/nani_plant.raster.hlsl"

#line 1 "shaders/inc/sun.inc.hlsl"


#ifndef TINYGLADE_INC_SUN_HLSL
#define TINYGLADE_INC_SUN_HLSL

#line 1 "shaders/inc/color_library.hlsl"


#ifndef TINYGLADE_INC_COLOR_LIBRARY_HLSL
#define TINYGLADE_INC_COLOR_LIBRARY_HLSL



struct ColorLibrary {
    float4 temp_color_1;
    float4 temp_color_2;
    float4 temp_color_3;
    float4 roof_color_1;
    float4 roof_color_2;
    float4 sky_low_day;
    float4 sky_high_day;
    float4 sun_tint_day;
    float4 sky_low_dusk;
    float4 sky_high_dusk;
    float4 sun_tint_dusk;
    float4 indirect_light_tint;
    float4 fence_color_1;
    float4 fence_color_2;
    float4 flower_color_1;
    float4 flower_color_2;
    float4 canopy_color_1;
    float4 canopy_color_2;
    float4 grass_1;
    float4 grass_2;
    float4 path;
    float4 sky_base;
    float4 sky_horizon;
    float4 sky_clouds_1;
    float4 sky_clouds_2;
    float4 terrain_1;
    float4 terrain_2;
    float4 terrain_flattened;
    float4 wall_1;
    float4 wall_2;
    float4 wall_moss;
    float4 wall_bricks;
    float4 ivy_branch;
    float4 ivy_leaf_1;
    float4 ivy_leaf_2;
    float4 terrain_highlight;
    float4 night_tint;
};






#ifndef TINYGLADE_COLOR_LIBRARY_REGISTER
#define TINYGLADE_COLOR_LIBRARY_REGISTER b14
#endif
cbuffer color_library : register(TINYGLADE_COLOR_LIBRARY_REGISTER, space0) {
    ColorLibrary color_library;
};

#endif 
#line 6 "shaders/inc/sun.inc.hlsl"



float3 get_sun_color()
{
    return sun_color.xyz;
}

float3 get_sky_low_color()
{
    return sky_low_color.xyz;
}

float3 get_sky_high_color()
{
    return sky_high_color.xyz;
}

float3 get_sun_direction()
{
    return dir_to_sun;
}


float3 get_sky_color(float3 day_low, float3 day_high, float3 dusk_low, float3 dusk_high)
{
    float t = saturate(time_of_day);
    float3 low = lerp(day_low, dusk_low, t);
    float3 high = lerp(day_high, dusk_high, t);
    return low;
}

#endif 
#line 30 "shaders/nani_plant.raster.hlsl"

#line 1 "shaders/inc/color_library.hlsl"


#ifndef TINYGLADE_INC_COLOR_LIBRARY_HLSL
#define TINYGLADE_INC_COLOR_LIBRARY_HLSL



struct ColorLibrary {
    float4 temp_color_1;
    float4 temp_color_2;
    float4 temp_color_3;
    float4 roof_color_1;
    float4 roof_color_2;
    float4 sky_low_day;
    float4 sky_high_day;
    float4 sun_tint_day;
    float4 sky_low_dusk;
    float4 sky_high_dusk;
    float4 sun_tint_dusk;
    float4 indirect_light_tint;
    float4 fence_color_1;
    float4 fence_color_2;
    float4 flower_color_1;
    float4 flower_color_2;
    float4 canopy_color_1;
    float4 canopy_color_2;
    float4 grass_1;
    float4 grass_2;
    float4 path;
    float4 sky_base;
    float4 sky_horizon;
    float4 sky_clouds_1;
    float4 sky_clouds_2;
    float4 terrain_1;
    float4 terrain_2;
    float4 terrain_flattened;
    float4 wall_1;
    float4 wall_2;
    float4 wall_moss;
    float4 wall_bricks;
    float4 ivy_branch;
    float4 ivy_leaf_1;
    float4 ivy_leaf_2;
    float4 terrain_highlight;
    float4 night_tint;
};






#ifndef TINYGLADE_COLOR_LIBRARY_REGISTER
#define TINYGLADE_COLOR_LIBRARY_REGISTER b14
#endif
cbuffer color_library : register(TINYGLADE_COLOR_LIBRARY_REGISTER, space0) {
    ColorLibrary color_library;
};

#endif 
#line 31 "shaders/nani_plant.raster.hlsl"

#line 1 "shaders/inc/color/srgb.hlsl"




#ifndef TINYGLADE_INC_COLOR_SRGB_HLSL
#define TINYGLADE_INC_COLOR_SRGB_HLSL

static const float SRGB_ALPHA = 0.04045f;
static const float SRGB_OFFSET = 0.055f;
static const float SRGB_GAMMA = 2.4f;


float srgb_to_linear(float c)
{
    float r;
    if (c <= SRGB_ALPHA)
        r = c * 0.077399380505084991455078125f; 
    else
        r = pow((c + SRGB_OFFSET) * 0.947867333889007568359375f, SRGB_GAMMA); 
    return r;
}

float3 srgb_to_linear(float3 c)
{
    return float3(srgb_to_linear(c.r), srgb_to_linear(c.g), srgb_to_linear(c.b));
}

float4 srgb_to_linear(float4 c)
{
    return float4(srgb_to_linear(c.rgb), c.a);
}


float linear_to_srgb(float c)
{
    float r;
    if (c <= 0.0031308f)
        r = c * 12.92f;
    else
        r = 1.055f * pow(c, 1.0f / SRGB_GAMMA) - SRGB_OFFSET;
    return r;
}

float3 linear_to_srgb(float3 c)
{
    return float3(linear_to_srgb(c.r), linear_to_srgb(c.g), linear_to_srgb(c.b));
}

float4 linear_to_srgb(float4 c)
{
    return float4(linear_to_srgb(c.rgb), c.a);
}

#endif 
#line 32 "shaders/nani_plant.raster.hlsl"

#line 1 "shaders/inc/consts.hlsl"






#ifndef TINYGLADE_INC_CONSTS_HLSL
#define TINYGLADE_INC_CONSTS_HLSL


static const float RENDER_SCALE = 1.0f;

#endif 
#line 33 "shaders/nani_plant.raster.hlsl"

#line 1 "shaders/inc/hash.hlsl"







#ifndef TINYGLADE_INC_HASH_HLSL
#define TINYGLADE_INC_HASH_HLSL

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
#line 11 "shaders/inc/hash.hlsl"



uint hash(uint x)
{
    x += (x << 10u);
    x ^= (x >> 6u);
    x += (x << 3u);
    x ^= (x >> 11u);
    x += (x << 15u);
    return x;
}


float rand01(uint x)
{
    return asfloat((x & 0x7FFFFFu) | 0x3F800000u) - 1.0f;
}


float hash_angle(uint x)
{
    return frac(0.9f + (float(x) * GOLDEN_RATIO)) * TWO_PI;
}


float hash01(float x)
{
    return frac(sin(x) * 43758.5453f);
}

#endif 
#line 34 "shaders/nani_plant.raster.hlsl"

#line 1 "shaders/gi/gi_settings.inc.hlsl"














#ifndef TINYGLADE_GI_SETTINGS_INC_HLSL
#define TINYGLADE_GI_SETTINGS_INC_HLSL




static const uint2 hi_px_subpixels[4] = {
    uint2(0u, 0u),
    uint2(0u, 1u),
    uint2(1u, 1u),
    uint2(1u, 0u),
};



static const float GI_MIN_HALF_NORMAL_Z = 0.3f;          
static const float GI_DEPTH_WEIGHT_SCALE = 80.0f;        
static const float GI_DEPTH_WEIGHT_MAX = 30.0f;
static const float GI_NORMAL_DOT_DAMP = -0.9f;           
static const float GI_FINAL_WEIGHT_NORM = 0.01f;         

#endif 
#line 35 "shaders/nani_plant.raster.hlsl"

#line 1 "shaders/inc/sh.hlsl"










float4 sh_basis(float3 dir)
{
    return float4(0.2820949852466583251953125f, 0.48860299587249755859375f * dir);
}

float sh_eval(float4 sh, float3 dir)
{
    return dot(sh, sh_basis(dir));
}
#line 36 "shaders/nani_plant.raster.hlsl"


uint2 terrain_heightmap_dims;
uint2 visual_terrain_heightmap_dims;
uint2 grass_object_push_dims;
int bool_render_shadows;

struct InstanceData
{
    Affine3Packed xform;
};

StructuredBuffer<uint> instance_indirection_buffer : register(t14);
StructuredBuffer<InstanceData> nani_instance_data : register(t15);
Texture2D<float4> grass_object_push : register(t18);

uint hash2(uint2 p)
{
    uint y = p.y;
    y += (y << 10u);
    y ^= (y >> 6u);
    y += (y << 3u);
    y ^= (y >> 11u);
    p.x = (p.x * 1664525u) + (y + (y << 15u)) + 1013904223u;
    p.x *= 1664525u;
    uint x = p.x ^ (p.x >> 11u);
    x = x ^ ((x << 7u) & 2636928640u);
    x = x ^ ((x << 15u) & 4022730752u);
    return x;
}

float hash2_rand(uint h)
{
    return asfloat(((h ^ (h >> 18u)) & 8388607u) | 1065353216u) - 1.0f;
}

struct VsIn
{
    [[vk::location(0)]] float3 Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 Vertex_Normal: Vertex_Normal;
    [[vk::location(2)]] float3 Vertex_Color: Vertex_Color;
    uint instance_id : SV_InstanceID;
};

struct VsOut
{
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float3 C3 : TEXCOORD3;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;
    uint idx = instance_indirection_buffer[vin.instance_id];
    Affine3Packed packed = nani_instance_data[idx].xform;
    float3x4 xform = transpose(float4x3(float3(packed.data[0], packed.data[1], packed.data[2]),
                                        float3(packed.data[3], packed.data[4], packed.data[5]),
                                        float3(packed.data[6], packed.data[7], packed.data[8]),
                                        float3(packed.data[9], packed.data[10], packed.data[11])));
    float3 world = mul(xform, float4(vin.Vertex_Position, 1.0f));
    float3 origin = mul(xform, float4(0.0f, 0.0f, 0.0f, 1.0f));

    uint origin_hash = uint((((origin.x * 20.0f) - (origin.z * 20.0f)) + 2000.0f) + vin.Vertex_Position.x + vin.Vertex_Position.z);
    origin_hash = hash(origin_hash);

    float4 grass_push = grass_object_push.SampleLevel(sampler_default, (origin.xz * 0.0076923077f) + 0.5f.xx, 0.0f);
    float3 wind_pos = world;
    if (clamp(grass_push.z, 0.0f, 1.0f) > 0.0f)
    {
        float2 displaced_xz = world.xz + (grass_push.xy * (pow(clamp(lerp(0.1f, 1.0f, vin.Vertex_Position.y * vin.Vertex_Position.y), 0.0f, 1.0f), 0.5f) * 0.25f));
        wind_pos = float3(displaced_xz.x, world.y, displaced_xz.y);
    }

    float vertex_h = vin.Vertex_Position.y + 0.5f;
    float time = game_time_sec + game_time_frac;
    float2 wind_uv = (origin.xz * 0.1f) + float2(time, 0.0f) * 1.2f;
    int2 wind_cell = int2(floor(wind_uv));
    float2 wind_frac = frac(wind_uv);
    float2 wind_smooth = wind_frac * wind_frac * (3.0f.xx - (wind_frac * 2.0f));
    float n00 = hash2_rand(hash2(uint2(wind_cell))) * (1.0f - wind_smooth.x) * (1.0f - wind_smooth.y);
    float n10 = hash2_rand(hash2(uint2(wind_cell + int2(1, 0)))) * wind_smooth.x * (1.0f - wind_smooth.y);
    float n01 = hash2_rand(hash2(uint2(wind_cell + int2(0, 1)))) * (1.0f - wind_smooth.x) * wind_smooth.y;
    float n11 = hash2_rand(hash2(uint2(wind_cell + int2(1, 1)))) * wind_smooth.x * wind_smooth.y;
    float wind_noise = (n00 + n10) + (n01 + n11);
    float instance_rand = hash2_rand(origin_hash);
    float sway = (vertex_h * vertex_h) * (sin((wind_noise * 3.14f) - 1.57f + 0.3f) * 0.035f) * lerp(0.7f, 1.0f, instance_rand);
    wind_pos.x = wind_pos.x - sway;

    ViewConstants vc = view_constants[current_view_index];
    o.position = mul(vc.world_to_sample, float4(wind_pos, 1.0f));
    o.C1 = normalize(mul(xform, float4(vin.Vertex_Normal, 0.0f)));
    o.C2 = wind_pos;
    o.C3 = pow(vin.Vertex_Color, lerp(1.0f, 2.0f, hash2_rand(origin_hash)).xxx);
    return o;
}

struct PsIn
{
    float3 C1 : TEXCOORD1;
    float3 C3 : TEXCOORD3;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;
    float3 packed_normal = (pin.C1 * 0.5f) + 0.5f.xxx;
    float3 srgb_albedo = sqrt(clamp(pin.C3, 0.0f.xxx, 1.0f.xxx));
    uint albedo_word = uint(clamp(srgb_albedo.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(srgb_albedo.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(srgb_albedo.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
                     + 4278190080u;
    uint normal_word = uint(clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
                     + 4278190080u;
    o.gbuffer = uint2(normal_word, albedo_word);
    return o;
}

#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)

#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
struct ViewConstants
{
    column_major float4x4 view_to_clip;
    column_major float4x4 clip_to_view;
    column_major float4x4 view_to_sample;
    column_major float4x4 sample_to_view;
    column_major float4x4 world_to_view;
    column_major float4x4 view_to_world;
    column_major float4x4 world_to_sample;
    column_major float4x4 clip_to_prev_clip;
    column_major float4x4 prev_view_to_prev_clip;
    column_major float4x4 prev_clip_to_prev_view;
    column_major float4x4 prev_world_to_prev_view;
    column_major float4x4 prev_view_to_prev_world;
    float2 sample_offset_pixels;
    float2 sample_offset_clip;
    float2 prev_sample_offset_pixels;
    float2 prev_sample_offset_clip;
};

struct Affine3Packed
{
    float data[12];
};

struct InstanceData
{
    Affine3Packed xform;
};

cbuffer frame_constants : register(b0, space1)
{
    float4 frame_constants_sun_color : packoffset(c0);
    float4 frame_constants_sky_low_color : packoffset(c1);
    float4 frame_constants_sky_high_color : packoffset(c2);
    float4 frame_constants_fog_tint_thickness : packoffset(c3);
    float4 frame_constants_fake_light_color : packoffset(c4);
    float4 frame_constants_fake_light_dir : packoffset(c5);
    float4 frame_constants_fake_light2_color : packoffset(c6);
    float4 frame_constants_fake_light2_dir : packoffset(c7);
    uint frame_constants_frame_index : packoffset(c8);
    float frame_constants_delta_time_seconds : packoffset(c8.y);
    float frame_constants_ev_mult : packoffset(c8.z);
    uint frame_constants_glade_theme_id : packoffset(c8.w);
    uint2 frame_constants_inner_rendering_resolution : packoffset(c9);
    uint frame_constants_game_time_sec : packoffset(c9.z);
    float frame_constants_game_time_frac : packoffset(c9.w);
    float3 frame_constants_editing_highlight_color : packoffset(c10);
    float frame_constants_time_of_day : packoffset(c10.w);
    float3 frame_constants_dir_to_sun : packoffset(c11);
    uint frame_constants_camera_idx : packoffset(c11.w);
    float3 frame_constants_dir_to_moon : packoffset(c12);
    uint frame_constants_render_overrides : packoffset(c12.w);
    float frame_constants_global_mip_bias : packoffset(c13);
    uint frame_constants_highlighted_transient_stair_edge_id : packoffset(c13.y);
    uint frame_constants_pad1 : packoffset(c13.z);
    uint frame_constants_pad2 : packoffset(c13.w);
    float4 frame_constants_debug_tweakers : packoffset(c14);
    ViewConstants frame_constants_view_constants[5] : packoffset(c15);
};

cbuffer $Globals : register(b13, space0)
{
    uint _Globals_current_view_index : packoffset(c0);
    uint2 _Globals_terrain_heightmap_dims : packoffset(c0.y);
    uint2 _Globals_visual_terrain_heightmap_dims : packoffset(c1);
    uint2 _Globals_grass_object_push_dims : packoffset(c1.z);
    int _Globals_bool_render_shadows : packoffset(c2);
};

ByteAddressBuffer instance_indirection_buffer : register(t14, space0);
ByteAddressBuffer nani_instance_data : register(t15, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> grass_object_push : register(t18, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Normal;
static float3 in_var_Vertex_Color;
static float3 out_var_C1;
static float3 out_var_C2;
static float3 out_var_C3;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 in_var_Vertex_Normal: Vertex_Normal;
    [[vk::location(2)]] float3 in_var_Vertex_Color: Vertex_Color;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 out_var_C3 : TEXCOORD3;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    InstanceData _141;
    [unroll]
    for (int _1ident = 0; _1ident < 12; _1ident++)
    {
        _141.xform.data[_1ident] = nani_instance_data.Load<float>(_1ident * 4 + instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 48 + 0);
    }
    float3x4 _161 = transpose(float4x3(float3(_141.xform.data[0], _141.xform.data[1], _141.xform.data[2]), float3(_141.xform.data[3], _141.xform.data[4], _141.xform.data[5]), float3(_141.xform.data[6], _141.xform.data[7], _141.xform.data[8]), float3(_141.xform.data[9], _141.xform.data[10], _141.xform.data[11])));
    float3 _166 = mul(_161, float4(in_var_Vertex_Position, 1.0f));
    float3 _167 = mul(_161, float4(0.0f, 0.0f, 0.0f, 1.0f));
    uint _176 = uint(((((_167.x * 20.0f) - (_167.z * 20.0f)) + 2000.0f) + in_var_Vertex_Position.x) + in_var_Vertex_Position.z);
    uint _178 = _176 + (_176 << 10u);
    uint _180 = _178 ^ (_178 >> 6u);
    uint _182 = _180 + (_180 << 3u);
    uint _184 = _182 ^ (_182 >> 11u);
    float2 _193 = _167.xz;
    float4 _197 = grass_object_push.SampleLevel(sampler_default, (_193 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f);
    float3 _213;
    if (clamp(_197.z, 0.0f, 1.0f) > 0.0f)
    {
        float2 _211 = _166.xz + (_197.xy * (pow(clamp(lerp(0.100000001490116119384765625f, 1.0f, in_var_Vertex_Position.y * in_var_Vertex_Position.y), 0.0f, 1.0f), 0.5f) * 0.25f));
        _213 = float3(_211.x, _166.y, _211.y);
    }
    else
    {
        _213 = _166;
    }
    float _214 = in_var_Vertex_Position.y + 0.5f;
    float2 _225 = (_193 * 0.100000001490116119384765625f) + (float2(float(frame_constants_game_time_sec) + frame_constants_game_time_frac, 0.0f) * 1.2000000476837158203125f);
    int2 _227 = int2(floor(_225));
    float2 _228 = frac(_225);
    uint2 _229 = uint2(_227);
    uint _231 = _229.y;
    uint _233 = _231 + (_231 << 10u);
    uint _235 = _233 ^ (_233 >> 6u);
    uint _237 = _235 + (_235 << 3u);
    uint _239 = _237 ^ (_237 >> 11u);
    uint _245 = (((_229.x * 1664525u) + (_239 + (_239 << 15u))) + 1013904223u) * 1664525u;
    uint _247 = _245 ^ (_245 >> 11u);
    uint _250 = _247 ^ ((_247 << 7u) & 2636928640u);
    uint _253 = _250 ^ ((_250 << 15u) & 4022730752u);
    uint2 _261 = uint2(_227 + int2(1, 0));
    uint _263 = _261.y;
    uint _265 = _263 + (_263 << 10u);
    uint _267 = _265 ^ (_265 >> 6u);
    uint _269 = _267 + (_267 << 3u);
    uint _271 = _269 ^ (_269 >> 11u);
    uint _277 = (((_261.x * 1664525u) + (_271 + (_271 << 15u))) + 1013904223u) * 1664525u;
    uint _279 = _277 ^ (_277 >> 11u);
    uint _282 = _279 ^ ((_279 << 7u) & 2636928640u);
    uint _285 = _282 ^ ((_282 << 15u) & 4022730752u);
    uint2 _293 = uint2(_227 + int2(0, 1));
    uint _295 = _293.y;
    uint _297 = _295 + (_295 << 10u);
    uint _299 = _297 ^ (_297 >> 6u);
    uint _301 = _299 + (_299 << 3u);
    uint _303 = _301 ^ (_301 >> 11u);
    uint _309 = (((_293.x * 1664525u) + (_303 + (_303 << 15u))) + 1013904223u) * 1664525u;
    uint _311 = _309 ^ (_309 >> 11u);
    uint _314 = _311 ^ ((_311 << 7u) & 2636928640u);
    uint _317 = _314 ^ ((_314 << 15u) & 4022730752u);
    uint2 _325 = uint2(_227 + int2(1, 1));
    uint _327 = _325.y;
    uint _329 = _327 + (_327 << 10u);
    uint _331 = _329 ^ (_329 >> 6u);
    uint _333 = _331 + (_331 << 3u);
    uint _335 = _333 ^ (_333 >> 11u);
    uint _341 = (((_325.x * 1664525u) + (_335 + (_335 << 15u))) + 1013904223u) * 1664525u;
    uint _343 = _341 ^ (_341 >> 11u);
    uint _346 = _343 ^ ((_343 << 7u) & 2636928640u);
    uint _349 = _346 ^ ((_346 << 15u) & 4022730752u);
    float2 _359 = (_228 * _228) * (3.0f.xx - (_228 * 2.0f));
    float _360 = _359.x;
    float _361 = 1.0f - _360;
    float _363 = _359.y;
    uint _380 = uint(int(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0)));
    uint _382 = _380 + (_380 << 10u);
    uint _384 = _382 ^ (_382 >> 6u);
    uint _386 = _384 + (_384 << 3u);
    uint _388 = _386 ^ (_386 >> 11u);
    float _399 = _213.x - ((((_214 * _214) * (sin((((((1.0f - _363) * (((asfloat(((_253 ^ (_253 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _361) + ((asfloat(((_285 ^ (_285 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _360))) + (((asfloat(((_317 ^ (_317 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _361) * _363)) + (((asfloat(((_349 ^ (_349 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _360) * _363)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f)) * 0.0350000001490116119384765625f) * lerp(0.699999988079071044921875f, 1.0f, asfloat(((_388 + (_388 << 15u)) & 8388607u) | 1065353216u) - 1.0f));
    float3 _400 = _213;
    _400.x = _399;
    gl_Position = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, float4(_399, _213.yz, 1.0f));
    out_var_C1 = normalize(mul(_161, float4(in_var_Vertex_Normal, 0.0f)));
    out_var_C2 = _400;
    out_var_C3 = pow(in_var_Vertex_Color, lerp(1.0f, 2.0f, asfloat(((_184 + (_184 << 15u)) & 8388607u) | 1065353216u) - 1.0f).xxx);
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_Vertex_Color = stage_input.in_var_Vertex_Color;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)

#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
static float3 in_var_C1;
static float3 in_var_C3;
static uint2 out_var_SV_TARGET0;

struct SPIRV_Cross_Input
{
    [[vk::location(1)]] float3 in_var_C1: Vertex_Normal;
    [[vk::location(3)]] float3 in_var_C3 : TEXCOORD3;
};

struct SPIRV_Cross_Output
{
    uint2 out_var_SV_TARGET0 : SV_Target0;
};

void frag_main()
{
    float3 _58 = (in_var_C1 * 0.5f) + 0.5f.xxx;
    float3 _80 = sqrt(clamp(in_var_C3, 0.0f.xxx, 1.0f.xxx));
    out_var_SV_TARGET0 = uint2(((uint((clamp(_58.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_58.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_58.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 4278190080u, ((uint((clamp(_80.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_80.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_80.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 4278190080u);
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C1 = stage_input.in_var_C1;
    in_var_C3 = stage_input.in_var_C3;
    frag_main();
    SPIRV_Cross_Output stage_output;
    stage_output.out_var_SV_TARGET0 = out_var_SV_TARGET0;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)

#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
struct ViewConstants
{
    column_major float4x4 view_to_clip;
    column_major float4x4 clip_to_view;
    column_major float4x4 view_to_sample;
    column_major float4x4 sample_to_view;
    column_major float4x4 world_to_view;
    column_major float4x4 view_to_world;
    column_major float4x4 world_to_sample;
    column_major float4x4 clip_to_prev_clip;
    column_major float4x4 prev_view_to_prev_clip;
    column_major float4x4 prev_clip_to_prev_view;
    column_major float4x4 prev_world_to_prev_view;
    column_major float4x4 prev_view_to_prev_world;
    float2 sample_offset_pixels;
    float2 sample_offset_clip;
    float2 prev_sample_offset_pixels;
    float2 prev_sample_offset_clip;
};

struct Affine3Packed
{
    float data[12];
};

struct InstanceData
{
    Affine3Packed xform;
};

cbuffer frame_constants : register(b0, space1)
{
    float4 frame_constants_sun_color : packoffset(c0);
    float4 frame_constants_sky_low_color : packoffset(c1);
    float4 frame_constants_sky_high_color : packoffset(c2);
    float4 frame_constants_fog_tint_thickness : packoffset(c3);
    float4 frame_constants_fake_light_color : packoffset(c4);
    float4 frame_constants_fake_light_dir : packoffset(c5);
    float4 frame_constants_fake_light2_color : packoffset(c6);
    float4 frame_constants_fake_light2_dir : packoffset(c7);
    uint frame_constants_frame_index : packoffset(c8);
    float frame_constants_delta_time_seconds : packoffset(c8.y);
    float frame_constants_ev_mult : packoffset(c8.z);
    uint frame_constants_glade_theme_id : packoffset(c8.w);
    uint2 frame_constants_inner_rendering_resolution : packoffset(c9);
    uint frame_constants_game_time_sec : packoffset(c9.z);
    float frame_constants_game_time_frac : packoffset(c9.w);
    float3 frame_constants_editing_highlight_color : packoffset(c10);
    float frame_constants_time_of_day : packoffset(c10.w);
    float3 frame_constants_dir_to_sun : packoffset(c11);
    uint frame_constants_camera_idx : packoffset(c11.w);
    float3 frame_constants_dir_to_moon : packoffset(c12);
    uint frame_constants_render_overrides : packoffset(c12.w);
    float frame_constants_global_mip_bias : packoffset(c13);
    uint frame_constants_highlighted_transient_stair_edge_id : packoffset(c13.y);
    uint frame_constants_pad1 : packoffset(c13.z);
    uint frame_constants_pad2 : packoffset(c13.w);
    float4 frame_constants_debug_tweakers : packoffset(c14);
    ViewConstants frame_constants_view_constants[5] : packoffset(c15);
};

cbuffer $Globals : register(b13, space0)
{
    uint _Globals_current_view_index : packoffset(c0);
    uint2 _Globals_terrain_heightmap_dims : packoffset(c0.y);
    uint2 _Globals_visual_terrain_heightmap_dims : packoffset(c1);
    uint2 _Globals_grass_object_push_dims : packoffset(c1.z);
    int _Globals_bool_render_shadows : packoffset(c2);
};

ByteAddressBuffer instance_indirection_buffer : register(t14, space0);
ByteAddressBuffer nani_instance_data : register(t15, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> grass_object_push : register(t18, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Normal;
static float3 in_var_Vertex_Color;
static float3 out_var_C1;
static float3 out_var_C2;
static float3 out_var_C3;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 in_var_Vertex_Normal: Vertex_Normal;
    [[vk::location(2)]] float3 in_var_Vertex_Color: Vertex_Color;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 out_var_C3 : TEXCOORD3;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    InstanceData _141;
    [unroll]
    for (int _1ident = 0; _1ident < 12; _1ident++)
    {
        _141.xform.data[_1ident] = nani_instance_data.Load<float>(_1ident * 4 + instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 48 + 0);
    }
    float3x4 _161 = transpose(float4x3(float3(_141.xform.data[0], _141.xform.data[1], _141.xform.data[2]), float3(_141.xform.data[3], _141.xform.data[4], _141.xform.data[5]), float3(_141.xform.data[6], _141.xform.data[7], _141.xform.data[8]), float3(_141.xform.data[9], _141.xform.data[10], _141.xform.data[11])));
    float3 _166 = mul(_161, float4(in_var_Vertex_Position, 1.0f));
    float3 _167 = mul(_161, float4(0.0f, 0.0f, 0.0f, 1.0f));
    uint _176 = uint(((((_167.x * 20.0f) - (_167.z * 20.0f)) + 2000.0f) + in_var_Vertex_Position.x) + in_var_Vertex_Position.z);
    uint _178 = _176 + (_176 << 10u);
    uint _180 = _178 ^ (_178 >> 6u);
    uint _182 = _180 + (_180 << 3u);
    uint _184 = _182 ^ (_182 >> 11u);
    float2 _193 = _167.xz;
    float4 _197 = grass_object_push.SampleLevel(sampler_default, (_193 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f);
    float3 _213;
    if (clamp(_197.z, 0.0f, 1.0f) > 0.0f)
    {
        float2 _211 = _166.xz + (_197.xy * (pow(clamp(lerp(0.100000001490116119384765625f, 1.0f, in_var_Vertex_Position.y * in_var_Vertex_Position.y), 0.0f, 1.0f), 0.5f) * 0.25f));
        _213 = float3(_211.x, _166.y, _211.y);
    }
    else
    {
        _213 = _166;
    }
    float _214 = in_var_Vertex_Position.y + 0.5f;
    float2 _225 = (_193 * 0.100000001490116119384765625f) + (float2(float(frame_constants_game_time_sec) + frame_constants_game_time_frac, 0.0f) * 1.2000000476837158203125f);
    int2 _227 = int2(floor(_225));
    float2 _228 = frac(_225);
    uint2 _229 = uint2(_227);
    uint _231 = _229.y;
    uint _233 = _231 + (_231 << 10u);
    uint _235 = _233 ^ (_233 >> 6u);
    uint _237 = _235 + (_235 << 3u);
    uint _239 = _237 ^ (_237 >> 11u);
    uint _245 = (((_229.x * 1664525u) + (_239 + (_239 << 15u))) + 1013904223u) * 1664525u;
    uint _247 = _245 ^ (_245 >> 11u);
    uint _250 = _247 ^ ((_247 << 7u) & 2636928640u);
    uint _253 = _250 ^ ((_250 << 15u) & 4022730752u);
    uint2 _261 = uint2(_227 + int2(1, 0));
    uint _263 = _261.y;
    uint _265 = _263 + (_263 << 10u);
    uint _267 = _265 ^ (_265 >> 6u);
    uint _269 = _267 + (_267 << 3u);
    uint _271 = _269 ^ (_269 >> 11u);
    uint _277 = (((_261.x * 1664525u) + (_271 + (_271 << 15u))) + 1013904223u) * 1664525u;
    uint _279 = _277 ^ (_277 >> 11u);
    uint _282 = _279 ^ ((_279 << 7u) & 2636928640u);
    uint _285 = _282 ^ ((_282 << 15u) & 4022730752u);
    uint2 _293 = uint2(_227 + int2(0, 1));
    uint _295 = _293.y;
    uint _297 = _295 + (_295 << 10u);
    uint _299 = _297 ^ (_297 >> 6u);
    uint _301 = _299 + (_299 << 3u);
    uint _303 = _301 ^ (_301 >> 11u);
    uint _309 = (((_293.x * 1664525u) + (_303 + (_303 << 15u))) + 1013904223u) * 1664525u;
    uint _311 = _309 ^ (_309 >> 11u);
    uint _314 = _311 ^ ((_311 << 7u) & 2636928640u);
    uint _317 = _314 ^ ((_314 << 15u) & 4022730752u);
    uint2 _325 = uint2(_227 + int2(1, 1));
    uint _327 = _325.y;
    uint _329 = _327 + (_327 << 10u);
    uint _331 = _329 ^ (_329 >> 6u);
    uint _333 = _331 + (_331 << 3u);
    uint _335 = _333 ^ (_333 >> 11u);
    uint _341 = (((_325.x * 1664525u) + (_335 + (_335 << 15u))) + 1013904223u) * 1664525u;
    uint _343 = _341 ^ (_341 >> 11u);
    uint _346 = _343 ^ ((_343 << 7u) & 2636928640u);
    uint _349 = _346 ^ ((_346 << 15u) & 4022730752u);
    float2 _359 = (_228 * _228) * (3.0f.xx - (_228 * 2.0f));
    float _360 = _359.x;
    float _361 = 1.0f - _360;
    float _363 = _359.y;
    uint _380 = uint(int(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0)));
    uint _382 = _380 + (_380 << 10u);
    uint _384 = _382 ^ (_382 >> 6u);
    uint _386 = _384 + (_384 << 3u);
    uint _388 = _386 ^ (_386 >> 11u);
    float _399 = _213.x - ((((_214 * _214) * (sin((((((1.0f - _363) * (((asfloat(((_253 ^ (_253 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _361) + ((asfloat(((_285 ^ (_285 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _360))) + (((asfloat(((_317 ^ (_317 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _361) * _363)) + (((asfloat(((_349 ^ (_349 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _360) * _363)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f)) * 0.0350000001490116119384765625f) * lerp(0.699999988079071044921875f, 1.0f, asfloat(((_388 + (_388 << 15u)) & 8388607u) | 1065353216u) - 1.0f));
    float3 _400 = _213;
    _400.x = _399;
    gl_Position = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, float4(_399, _213.yz, 1.0f));
    out_var_C1 = normalize(mul(_161, float4(in_var_Vertex_Normal, 0.0f)));
    out_var_C2 = _400;
    out_var_C3 = pow(in_var_Vertex_Color, lerp(1.0f, 2.0f, asfloat(((_184 + (_184 << 15u)) & 8388607u) | 1065353216u) - 1.0f).xxx);
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_Vertex_Color = stage_input.in_var_Vertex_Color;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif
