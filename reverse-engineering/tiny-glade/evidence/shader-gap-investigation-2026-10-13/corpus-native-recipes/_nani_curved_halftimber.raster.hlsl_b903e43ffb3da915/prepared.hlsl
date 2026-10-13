#define SHADER_VARIANT_DEPTH_ONLY 0
#define TGV_b903e43ffb3da915 1
#line 1 "shaders/nani_curved_halftimber.raster.hlsl"


#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))





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
#line 9 "shaders/nani_curved_halftimber.raster.hlsl"

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
#line 10 "shaders/nani_curved_halftimber.raster.hlsl"

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
#line 11 "shaders/nani_curved_halftimber.raster.hlsl"

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
#line 12 "shaders/nani_curved_halftimber.raster.hlsl"

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
#line 13 "shaders/nani_curved_halftimber.raster.hlsl"

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
#line 14 "shaders/nani_curved_halftimber.raster.hlsl"

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
#line 15 "shaders/nani_curved_halftimber.raster.hlsl"

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
#line 16 "shaders/nani_curved_halftimber.raster.hlsl"

#line 1 "shaders/buildings/curved_halftimber.inc.hlsl"


#ifndef TINYGLADE_BUILDINGS_CURVED_HALFTIMBER_HLSL
#define TINYGLADE_BUILDINGS_CURVED_HALFTIMBER_HLSL

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
#line 6 "shaders/buildings/curved_halftimber.inc.hlsl"



struct CurvedHalftimberParams
{
    float3 timber_color;
    float3 fill_color;
    float timber_thickness;
    float2 uv_scale;
    uint flags;
};



SolidMaterialParams evaluate_curved_halftimber(CurvedHalftimberParams p, float2 uv, float3 normal)
{
    float timber = 0.0f;
    
    float2 line_uv = uv * p.uv_scale;
    timber = max(timber, 1.0f - smoothstep(0.0f, p.timber_thickness, abs(frac(line_uv.x) - 0.5f)));
    timber = max(timber, 1.0f - smoothstep(0.0f, p.timber_thickness, abs(frac(line_uv.y) - 0.5f)));
    float3 albedo = lerp(p.fill_color, p.timber_color, saturate(timber));
    return init_solid_material(albedo, 0.5f, p.flags);
}

#endif 
#line 17 "shaders/nani_curved_halftimber.raster.hlsl"

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
#line 18 "shaders/nani_curved_halftimber.raster.hlsl"

#line 1 "shaders/inc/parallax_mapping.hlsl"




#ifndef TINYGLADE_INC_PARALLAX_MAPPING_HLSL
#define TINYGLADE_INC_PARALLAX_MAPPING_HLSL




float2 parallax_mapping(Texture2D<float4> height_tex, SamplerState samp,
                        float2 uv, float3 view_dir, float height_scale, uint layers)
{
    float layer_depth = 1.0f / float(layers);
    float current_depth = 1.0f;
    float2 p = view_dir.xy * (height_scale / float(layers));
    float2 delta = p;

    float2 current_uv = uv;
    float height = 1.0f - height_tex.SampleLevel(samp, current_uv, 0.0f).r;
    while (current_depth > 0.0f && height < current_depth)
    {
        current_uv -= delta;
        height = 1.0f - height_tex.SampleLevel(samp, current_uv, 0.0f).r;
        current_depth -= layer_depth;
    }

    
    float2 prev_uv = current_uv + delta;
    float prev_depth = current_depth + layer_depth;
    float prev_height = 1.0f - height_tex.SampleLevel(samp, prev_uv, 0.0f).r;
    float w = (current_depth - height) / max((current_depth - height) - (prev_depth - prev_height), 1.0e-5f);
    return lerp(current_uv, prev_uv, saturate(w));
}

#endif 
#line 19 "shaders/nani_curved_halftimber.raster.hlsl"

#line 1 "shaders/inc/raymarch.hlsl"


#ifndef TINYGLADE_INC_RAYMARCH_HLSL
#define TINYGLADE_INC_RAYMARCH_HLSL





float raymarch_depth_texture(Texture2D depth_tex, SamplerState s, float2 texel_size,
                             float3 ray_start_view, float3 ray_end_view)
{
    float3 delta = ray_end_view - ray_start_view;
    float dist = length(delta);
    float t = 0.0;
    
    float step = max(0.05, dist / 16.0);
    for (uint i = 0u; i < 64u; ++i) {
        t += step / max(dist, 1e-6);
        if (t >= 1.0) {
            return 1.0;
        }
        float3 p = ray_start_view + delta * t;
        if (p.z <= 0.0) {
            return 1.0;
        }
        float2 uv = p.xy * 0.5 + 0.5;
        uv.y = 1.0 - uv.y;
        float sampled_depth = depth_tex.SampleLevel(s, uv, 0.0).x;
        float inv_depth = 1.0 / sampled_depth;
        float my_inv = 1.0 / p.z;
        if (my_inv > inv_depth) {
            
            float lo = max(0.0, t - step / max(dist, 1e-6));
            for (uint j = 0u; j < 8u; ++j) {
                float mid = (lo + t) * 0.5;
                float3 pm = ray_start_view + delta * mid;
                float2 uvm = pm.xy * 0.5 + 0.5;
                uvm.y = 1.0 - uvm.y;
                if (depth_tex.SampleLevel(s, uvm, 0.0).x > 1.0 / pm.z) {
                    t = mid;
                } else {
                    lo = mid;
                }
            }
            return t;
        }
    }
    return 1.0;
}



float raymarch_opacity(Texture2D depth_tex, SamplerState s, float2 texel_size,
                       float3 ray_start_view, float3 ray_end_view, float density)
{
    float3 delta = ray_end_view - ray_start_view;
    float dist = length(delta);
    float acc = 0.0;
    uint steps = 12u;
    for (uint i = 0u; i < steps; ++i) {
        float t = (float(i) + 0.5) / float(steps);
        float3 p = ray_start_view + delta * t;
        if (p.z <= 0.0) {
            break;
        }
        float2 uv = p.xy * 0.5 + 0.5;
        uv.y = 1.0 - uv.y;
        float inv_depth = 1.0 / max(depth_tex.SampleLevel(s, uv, 0.0).x, 1e-6);
        if (inv_depth < 1.0 / max(p.z, 1e-6)) {
            acc += density * (dist / float(steps));
        }
    }
    return saturate(acc);
}

#endif 
#line 20 "shaders/nani_curved_halftimber.raster.hlsl"

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
#line 21 "shaders/nani_curved_halftimber.raster.hlsl"

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
#line 22 "shaders/nani_curved_halftimber.raster.hlsl"

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
#line 23 "shaders/nani_curved_halftimber.raster.hlsl"

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
#line 24 "shaders/nani_curved_halftimber.raster.hlsl"

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
#line 25 "shaders/nani_curved_halftimber.raster.hlsl"

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
#line 26 "shaders/nani_curved_halftimber.raster.hlsl"

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
#line 27 "shaders/nani_curved_halftimber.raster.hlsl"

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
#line 28 "shaders/nani_curved_halftimber.raster.hlsl"

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
#line 29 "shaders/nani_curved_halftimber.raster.hlsl"

#line 1 "shaders/inc/consts.hlsl"






#ifndef TINYGLADE_INC_CONSTS_HLSL
#define TINYGLADE_INC_CONSTS_HLSL


static const float RENDER_SCALE = 1.0f;

#endif 
#line 30 "shaders/nani_curved_halftimber.raster.hlsl"

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
#line 31 "shaders/nani_curved_halftimber.raster.hlsl"

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
#line 32 "shaders/nani_curved_halftimber.raster.hlsl"

#line 1 "shaders/inc/sh.hlsl"










float4 sh_basis(float3 dir)
{
    return float4(0.2820949852466583251953125f, 0.48860299587249755859375f * dir);
}

float sh_eval(float4 sh, float3 dir)
{
    return dot(sh, sh_basis(dir));
}
#line 33 "shaders/nani_curved_halftimber.raster.hlsl"


uint2 wood_normal_dims;
uint2 wood_roughness_dims;
uint2 wood_edge_damage_dims;
uint2 dirthpath_heightmap_dims;

struct InstanceData
{
    Affine3Packed xform;
    uint flags;
    float beam_count;
    uint curve_type;
    uint color_id;
    uint wall_id;
};

StructuredBuffer<uint> instance_indirection_buffer : register(t14);
StructuredBuffer<InstanceData> nani_instance_data : register(t15);
Texture2D<float4> wood_normal : register(t16);
Texture2D<float4> wood_roughness : register(t17);
Texture2D<float4> wood_edge_damage : register(t18);
Texture2DArray<float4> halftimber_colors : register(t20);
Texture2D<float4> dirthpath_heightmap : register(t21);

struct VsIn
{
    [[vk::location(0)]] float3 Vertex_Position: Vertex_Position;
    uint vertex_id : SV_VertexID;
    uint instance_id : SV_InstanceID;
};

struct VsOut
{
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    nointerpolation uint C4 : TEXCOORD4;
    float C5 : TEXCOORD5;
    float C6 : TEXCOORD6;
    float2 C7 : TEXCOORD7;
    float C8 : TEXCOORD8;
    nointerpolation uint C9 : TEXCOORD9;
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
    ViewConstants vc = view_constants[current_view_index];
    o.position = mul(vc.world_to_sample, float4(world, 1.0f));
    o.C1 = mul(xform, float4(0.0f, 1.0f, 0.0f, 0.0f));
    o.C2 = world;
    o.C4 = idx;
    o.C5 = vin.Vertex_Position.z;
    o.C6 = 1.0f;
    o.C7 = vin.Vertex_Position.xy;
    o.C8 = vin.Vertex_Position.x;
    o.C9 = 0u;
    return o;
}

struct PsIn
{
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    nointerpolation uint C4 : TEXCOORD4;
    float C5 : TEXCOORD5;
    float C6 : TEXCOORD6;
    float2 C7 : TEXCOORD7;
    float C8 : TEXCOORD8;
    nointerpolation uint C9 : TEXCOORD9;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;
    float2 uv = pin.C7 * float2(0.4f, 0.06f);
    float3 normal = pin.C1;
    float2 beam_uv = uv;
    float edge = 0.0f;
    if (pin.C5 <= 0.01f)
    {
        normal = normalize(cross(ddx(pin.C2), -ddy(pin.C2)));
        beam_uv = float2(uv.y, uv.x * 2.0f) + 0.87f.xx;
        edge = 1.0f - wood_edge_damage.SampleBias(sampler_llr, pin.C7 * float2(1.0f, 0.05f), global_mip_bias).x;
    }
    int color_id = int(nani_instance_data[pin.C4].color_id);
    float2 sample_uv = beam_uv + (edge * 0.3f);
    float3 albedo = halftimber_colors.SampleBias(sampler_llr, float3(sample_uv.x, sample_uv.y, float(color_id)), global_mip_bias).xyz;
    float2 rough_uv = sample_uv * float2(0.5f, (pin.C8 < 0.0f) ? 3.0f : 2.0f);
    float roughness = lerp(0.7f, 0.9f, 1.0f - wood_roughness.SampleBias(sampler_llr, rough_uv, global_mip_bias).x);
    if (pin.C5 <= 0.01f)
    {
        albedo = lerp(albedo, 0.75f.xxx, (pin.C8 < 0.0f ? 0.2f : 0.0f).xxx);
    }
    float3 normal_map = wood_normal.SampleBias(sampler_llr, sample_uv, global_mip_bias).xyz * 2.0f - 1.0f.xxx;
    float3 shaded_normal = normalize(normal + (normal_map * 0.4f));
    if (glade_theme_id == 4u)
    {
        float path = dirthpath_heightmap.SampleBias(sampler_llr, pin.C2.xz * 0.05f, global_mip_bias).x;
        albedo = lerp(albedo, 0.7f.xxx, (((smoothstep(0.0f, 0.2f, smoothstep(0.5f, 1.0f, roughness) * lerp(0.0f, 0.5f, path)) * 0.6f) + (path * 0.2f)) * clamp(shaded_normal.y, 0.0f, 1.0f)).xxx);
    }
    float3 packed_normal = (normalize(shaded_normal) * 0.5f) + 0.5f.xxx;
    float3 srgb_albedo = sqrt(clamp(albedo, 0.0f.xxx, 1.0f.xxx));
    uint albedo_word = uint(clamp(srgb_albedo.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(srgb_albedo.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(srgb_albedo.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u);
    uint normal_word = uint(clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
                     + (uint(clamp(roughness, 0.0f, 1.0f) * 255.0f + 0.5f) << 24u);
    o.gbuffer = uint2(normal_word, albedo_word);
    o.gbuffer.y = o.gbuffer.y | 117440512u;
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

struct CurvedHalftimberInstanceData
{
    float3 position;
    uint color_id;
    float radius0;
    float radius1;
    float angle0;
    float angle1;
    int is_straight;
    int is_xz;
    float plank_depth;
    int caps;
    int mesh_atlas_part_idx;
    int pad0;
    int pad1;
    int pad2;
};

static const float2 _126[7] = { 0.0f.xx, float2(0.0f, 0.89999997615814208984375f), float2(0.100000001490116119384765625f, 1.0f), float2(0.89999997615814208984375f, 1.0f), float2(1.0f, 0.89999997615814208984375f), float2(1.0f, 0.0f), 0.0f.xx };
static const float _130[7] = { 0.0f, 0.89999997615814208984375f, 1.10000002384185791015625f, 1.89999997615814208984375f, 2.099999904632568359375f, 3.0f, 4.0f };
static const float2 _134[7] = { float2(-1.0f, 0.0f), float2(-1.0f, 0.0f), float2(0.0f, 1.0f), float2(0.0f, 1.0f), float2(1.0f, 0.0f), float2(1.0f, 0.0f), float2(0.0f, -1.0f) };

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
    uint2 _Globals_wood_normal_dims : packoffset(c0.y);
    uint2 _Globals_wood_roughness_dims : packoffset(c1);
    uint2 _Globals_wood_edge_damage_dims : packoffset(c1.z);
    uint2 _Globals_dirthpath_heightmap_dims : packoffset(c2);
};

ByteAddressBuffer instance_indirection_buffer : register(t14, space0);
ByteAddressBuffer nani_instance_data : register(t15, space0);

static float4 gl_Position;
static int gl_VertexIndex;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 out_var_C1;
static float3 out_var_C2;
static uint out_var_C4;
static float out_var_C5;
static float out_var_C6;
static float2 out_var_C7;
static float out_var_C8;
static uint out_var_C9;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    uint gl_VertexIndex : SV_VertexID;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(4)]] nointerpolation uint out_var_C4 : TEXCOORD4;
    [[vk::location(5)]] float out_var_C5 : TEXCOORD5;
    [[vk::location(6)]] float out_var_C6 : TEXCOORD6;
    [[vk::location(7)]] float2 out_var_C7 : TEXCOORD7;
    [[vk::location(8)]] float out_var_C8 : TEXCOORD8;
    [[vk::location(9)]] nointerpolation uint out_var_C9 : TEXCOORD9;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    uint _155;
    float4 _440;
    float3 _441;
    float3 _442;
    float _443;
    float _444;
    float2 _445;
    float _446;
    uint _447;
    do
    {
        _155 = instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0);
        int _179 = int(1u << (uint(nani_instance_data.Load<int>(_155 * 64 + 48)) & 31u));
        uint _180 = uint(gl_VertexIndex) % 7u;
        float _187 = _130[_180] / _130[5];
        bool _190 = nani_instance_data.Load<int>(_155 * 64 + 36) == 0;
        float2 _199;
        float2 _200;
        if (_190)
        {
            _199 = float2(_134[_180].yx);
            _200 = float2(_126[_180].yx);
        }
        else
        {
            _199 = _134[_180];
            _200 = _126[_180];
        }
        float _202 = 1.0f / float(_179);
        if (nani_instance_data.Load<int>(_155 * 64 + 32) > 0)
        {
            float3 _353 = float3(nani_instance_data.Load<float>(_155 * 64 + 16), 0.0f, nani_instance_data.Load<float>(_155 * 64 + 20));
            float _392;
            if (nani_instance_data.Load<int>(_155 * 64 + 32) == 2)
            {
                float _391;
                if (in_var_Vertex_Position.z < 0.5f)
                {
                    _391 = nani_instance_data.Load<float>(_155 * 64 + 28) + (nani_instance_data.Load<float>(_155 * 64 + 24) * lerp(-1.0f, 1.0f, _200.x));
                }
                else
                {
                    _391 = nani_instance_data.Load<float>(_155 * 64 + 28);
                }
                _392 = _391;
            }
            else
            {
                float _382;
                if (nani_instance_data.Load<int>(_155 * 64 + 32) == 3)
                {
                    float _381;
                    if (in_var_Vertex_Position.z > 0.5f)
                    {
                        _381 = nani_instance_data.Load<float>(_155 * 64 + 28) + (nani_instance_data.Load<float>(_155 * 64 + 24) * lerp(-1.0f, 1.0f, _200.x));
                    }
                    else
                    {
                        _381 = nani_instance_data.Load<float>(_155 * 64 + 28);
                    }
                    _382 = _381;
                }
                else
                {
                    float _372;
                    if (nani_instance_data.Load<int>(_155 * 64 + 32) == 4)
                    {
                        _372 = nani_instance_data.Load<float>(_155 * 64 + 28) + (nani_instance_data.Load<float>(_155 * 64 + 24) * lerp(-1.0f, 1.0f, _200.x));
                    }
                    else
                    {
                        _372 = nani_instance_data.Load<float>(_155 * 64 + 28);
                    }
                    _382 = _372;
                }
                _392 = _382;
            }
            float3 _402 = (nani_instance_data.Load<float3>(_155 * 64 + 0) + ((_353 * lerp(-0.5f, 0.5f, _200.x)) * nani_instance_data.Load<float>(_155 * 64 + 24))) + (((-float3(-nani_instance_data.Load<float>(_155 * 64 + 20), 0.0f, nani_instance_data.Load<float>(_155 * 64 + 16))) * lerp(-0.5f, 0.5f, in_var_Vertex_Position.z)) * _392);
            uint _408 = _155 + (_155 << 10u);
            uint _410 = _408 ^ (_408 >> 6u);
            uint _412 = _410 + (_410 << 3u);
            uint _414 = _412 ^ (_412 >> 11u);
            float _422 = (_402.y + (_200.y * nani_instance_data.Load<float>(_155 * 64 + 40))) + ((asfloat(((_414 + (_414 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * 0.00999999977648258209228515625f);
            float3 _423 = _402;
            _423.y = _422;
            _440 = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, float4(_402.x, _422, _402.z, 1.0f));
            _441 = (_353 * _199.x) + float3(0.0f, _199.y, 0.0f);
            _442 = _423;
            _443 = 0.0f;
            _444 = in_var_Vertex_Position.z;
            _445 = float2(_187, in_var_Vertex_Position.z * _392);
            _446 = -1.0f;
            _447 = uint(_179);
            break;
        }
        else
        {
            float _272;
            float _273;
            if (_190)
            {
                _272 = 0.0f;
                _273 = lerp(nani_instance_data.Load<float>(_155 * 64 + 24), nani_instance_data.Load<float>(_155 * 64 + 28), 1.0f - in_var_Vertex_Position.z);
            }
            else
            {
                float _266;
                float _267;
                if (nani_instance_data.Load<int>(_155 * 64 + 44) == 1)
                {
                    float _264;
                    float _265;
                    if (in_var_Vertex_Position.z >= _202)
                    {
                        _264 = 0.0f;
                        _265 = (in_var_Vertex_Position.z - _202) / (1.0f - _202);
                    }
                    else
                    {
                        bool _258 = in_var_Vertex_Position.z < _202;
                        _264 = float(_258);
                        _265 = _258 ? 0.0f : in_var_Vertex_Position.z;
                    }
                    _266 = _264;
                    _267 = _265;
                }
                else
                {
                    float _252;
                    float _253;
                    if (nani_instance_data.Load<int>(_155 * 64 + 44) == 2)
                    {
                        float _241 = 1.0f - _202;
                        float _250;
                        float _251;
                        if (in_var_Vertex_Position.z <= _241)
                        {
                            _250 = 0.0f;
                            _251 = in_var_Vertex_Position.z / _241;
                        }
                        else
                        {
                            bool _246 = in_var_Vertex_Position.z > _241;
                            _250 = float(_246);
                            _251 = _246 ? 1.0f : in_var_Vertex_Position.z;
                        }
                        _252 = _250;
                        _253 = _251;
                    }
                    else
                    {
                        float _239;
                        float _240;
                        if (nani_instance_data.Load<int>(_155 * 64 + 44) == 3)
                        {
                            bool _227;
                            if (in_var_Vertex_Position.z >= _202)
                            {
                                _227 = in_var_Vertex_Position.z <= (1.0f - _202);
                            }
                            else
                            {
                                _227 = false;
                            }
                            float _237;
                            if (_227)
                            {
                                _237 = (in_var_Vertex_Position.z - _202) / ((1.0f - _202) - _202);
                            }
                            else
                            {
                                _237 = (in_var_Vertex_Position.z < _202) ? 0.0f : 1.0f;
                            }
                            _239 = _227 ? 0.0f : 1.0f;
                            _240 = _237;
                        }
                        else
                        {
                            _239 = 0.0f;
                            _240 = in_var_Vertex_Position.z;
                        }
                        _252 = _239;
                        _253 = _240;
                    }
                    _266 = _252;
                    _267 = _253;
                }
                _272 = _266;
                _273 = lerp(nani_instance_data.Load<float>(_155 * 64 + 24), nani_instance_data.Load<float>(_155 * 64 + 28), _267);
            }
            float _276 = cos(_273);
            float _278 = -sin(_273);
            float3 _281 = (float3(_276, 0.0f, _278) * lerp(nani_instance_data.Load<float>(_155 * 64 + 16), nani_instance_data.Load<float>(_155 * 64 + 20), _200.x)) + nani_instance_data.Load<float3>(_155 * 64 + 0);
            float3 _322;
            if (nani_instance_data.Load<int>(_155 * 64 + 44) == 0)
            {
                float3 _321 = _281;
                _321.y = _281.y + (_200.y * nani_instance_data.Load<float>(_155 * 64 + 40));
                _322 = _321;
            }
            else
            {
                float _310;
                if (nani_instance_data.Load<int>(_155 * 64 + 44) == 1)
                {
                    _310 = ((in_var_Vertex_Position.z >= _202) ? 1.0f : 0.0f);
                }
                else
                {
                    float _306;
                    if (nani_instance_data.Load<int>(_155 * 64 + 44) == 2)
                    {
                        _306 = ((in_var_Vertex_Position.z <= (1.0f - _202)) ? 1.0f : 0.0f);
                    }
                    else
                    {
                        bool _300;
                        if (in_var_Vertex_Position.z >= _202)
                        {
                            _300 = in_var_Vertex_Position.z <= (1.0f - _202);
                        }
                        else
                        {
                            _300 = false;
                        }
                        _306 = float(_300);
                    }
                    _310 = _306;
                }
                float3 _316 = _281;
                _316.y = _281.y + ((_310 * _200.y) * nani_instance_data.Load<float>(_155 * 64 + 40));
                _322 = _316;
            }
            float _330 = 0.001953125f * (floor(_322.y * 512.0f) + frac(float(_155) * 1.61803400516510009765625f));
            float3 _331 = _322;
            _331.y = _330;
            _440 = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, float4(_322.x, _330, _322.z, 1.0f));
            _441 = float3(_276, 1.0f, _278) * float3(_199.xyx);
            _442 = _331;
            _443 = (uint(gl_VertexIndex) == 6u) ? 1.0f : _272;
            _444 = in_var_Vertex_Position.z;
            _445 = float2(_187, (_273 * nani_instance_data.Load<float>(_155 * 64 + 16)) * 3.1400001049041748046875f);
            _446 = _273 * 0.159235656261444091796875f;
            _447 = uint(_179);
            break;
        }
        break; 
    } while(false);
    gl_Position = _440;
    out_var_C1 = _441;
    out_var_C2 = _442;
    out_var_C4 = _155;
    out_var_C5 = _443;
    out_var_C6 = _444;
    out_var_C7 = _445;
    out_var_C8 = _446;
    out_var_C9 = _447;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_VertexIndex = int(stage_input.gl_VertexIndex);
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C4 = out_var_C4;
    stage_output.out_var_C5 = out_var_C5;
    stage_output.out_var_C6 = out_var_C6;
    stage_output.out_var_C7 = out_var_C7;
    stage_output.out_var_C8 = out_var_C8;
    stage_output.out_var_C9 = out_var_C9;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)

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

struct CurvedHalftimberInstanceData
{
    float3 position;
    uint color_id;
    float radius0;
    float radius1;
    float angle0;
    float angle1;
    int is_straight;
    int is_xz;
    float plank_depth;
    int caps;
    int mesh_atlas_part_idx;
    int pad0;
    int pad1;
    int pad2;
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

ByteAddressBuffer nani_instance_data : register(t15, space0);
SamplerState sampler_llr : register(s1, space0);
Texture2D<float4> wood_normal : register(t16, space0);
Texture2D<float4> wood_roughness : register(t17, space0);
Texture2D<float4> wood_edge_damage : register(t18, space0);
Texture2DArray<float4> halftimber_colors : register(t20, space0);
Texture2D<float4> dirthpath_heightmap : register(t21, space0);

static float3 in_var_C1;
static float3 in_var_C2;
static uint in_var_C4;
static float in_var_C5;
static float2 in_var_C7;
static float in_var_C8;
static uint2 out_var_SV_TARGET0;

struct SPIRV_Cross_Input
{
    [[vk::location(1)]] float3 in_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 in_var_C2 : TEXCOORD2;
    [[vk::location(4)]] nointerpolation uint in_var_C4 : TEXCOORD4;
    [[vk::location(5)]] float in_var_C5 : TEXCOORD5;
    [[vk::location(7)]] float2 in_var_C7 : TEXCOORD7;
    [[vk::location(8)]] float in_var_C8 : TEXCOORD8;
};

struct SPIRV_Cross_Output
{
    uint2 out_var_SV_TARGET0 : SV_Target0;
};

void frag_main()
{
    float2 _126 = in_var_C7 * float2(0.4000000059604644775390625f, 0.0599999986588954925537109375f);
    float3 _150;
    float2 _151;
    float _152;
    if (in_var_C5 > 0.00999999977648258209228515625f)
    {
        _150 = normalize(cross(ddx(in_var_C2), -ddy(in_var_C2)));
        _151 = float2(_126.y, _126.x * 2.0f) + 0.87000000476837158203125f.xx;
        _152 = 0.0f;
    }
    else
    {
        _150 = in_var_C1;
        _151 = _126;
        _152 = 1.0f - wood_edge_damage.SampleBias(sampler_llr, in_var_C7 * float2(1.0f, 0.0500000007450580596923828125f), frame_constants_global_mip_bias).x;
    }
    int _155 = int(nani_instance_data.Load<uint>(in_var_C4 * 64 + 12));
    float2 _158 = _151 + (_152.xx * 0.300000011920928955078125f);
    float _161 = float(_155);
    float _162 = _158.x;
    float4 _168 = halftimber_colors.SampleBias(sampler_llr, float3(_162, _158.y, _161), frame_constants_global_mip_bias);
    float3 _169 = _168.xyz;
    float2 _173 = _158 * float2(0.5f, (in_var_C8 < 0.0f) ? 3.0f : 2.0f);
    float4 _176 = wood_roughness.SampleBias(sampler_llr, _173, frame_constants_global_mip_bias);
    float _179 = lerp(0.699999988079071044921875f, 0.89999997615814208984375f, 1.0f - _176.x);
    float3 _220;
    if (in_var_C5 <= 0.00999999977648258209228515625f)
    {
        float2 _186 = ddx(_173);
        float2 _188 = -ddy(_173);
        float3 _189 = cross(in_var_C1, -ddy(in_var_C2));
        float3 _190 = cross(ddx(in_var_C2), in_var_C1);
        float _191 = _186.x;
        float _192 = _188.y;
        float _194 = _186.y;
        float _195 = _188.x;
        float _198 = 1.0f / ((_191 * _192) - (_194 * _195));
        float3 _214 = (wood_normal.SampleBias(sampler_llr, _173, frame_constants_global_mip_bias).xyz * 2.0f) - 1.0f.xxx;
        _214.z = _214.z * 18.1818180084228515625f;
        _220 = normalize(mul(transpose(float3x3(((_189 * _191) + (_190 * _195)) * _198, ((_189 * _194) + (_190 * _192)) * _198, in_var_C1)), _214));
    }
    else
    {
        _220 = _150;
    }
    float3 _252;
    float _253;
    float3 _254;
    if (in_var_C8 > 0.0f)
    {
        float3 _249;
        float _250;
        float3 _251;
        if (in_var_C8 > 0.89999997615814208984375f)
        {
            float _228 = smoothstep(0.89999997615814208984375f, 1.0f, in_var_C8);
            float3 _235 = _228.xxx;
            _249 = lerp(_220, in_var_C1, _235);
            _250 = lerp(_179, 0.89999997615814208984375f, _228);
            _251 = lerp(_169, halftimber_colors.SampleBias(sampler_llr, float3(_151.x, -_228, _161), frame_constants_global_mip_bias).xyz, _235);
        }
        else
        {
            float3 _247;
            float _248;
            if (in_var_C8 < 0.100000001490116119384765625f)
            {
                float _243 = (0.100000001490116119384765625f - in_var_C8) * 10.0f;
                _247 = lerp(_220, in_var_C1, _243.xxx);
                _248 = lerp(_179, 0.89999997615814208984375f, _243);
            }
            else
            {
                _247 = _220;
                _248 = _179;
            }
            _249 = _247;
            _250 = _248;
            _251 = _169;
        }
        _252 = _249;
        _253 = _250;
        _254 = _251;
    }
    else
    {
        _252 = _220;
        _253 = _179;
        _254 = _169;
    }
    float4 _257 = halftimber_colors.SampleBias(sampler_llr, float3(_162, _158.y, 0.0f), frame_constants_global_mip_bias);
    float3 _264 = lerp(_254, _257.xyz * ((_155 == 0) ? 2.0f : 1.0f), (_152 * 0.4000000059604644775390625f).xxx);
    float3 _288;
    if (frame_constants_glade_theme_id == 4u)
    {
        float4 _274 = dirthpath_heightmap.SampleBias(sampler_llr, in_var_C2.xz * 0.0500000007450580596923828125f, frame_constants_global_mip_bias);
        float _275 = _274.x;
        _288 = lerp(_264, 0.699999988079071044921875f.xxx, (((smoothstep(0.0f, 0.20000000298023223876953125f, smoothstep(0.5f, 1.0f, _253) * lerp(0.0f, 0.5f, _275)) * 0.60000002384185791015625f) + (_275 * 0.20000000298023223876953125f)) * clamp(_252.y, 0.0f, 1.0f)).xxx);
    }
    else
    {
        _288 = _264;
    }
    float3 _290 = (_252 * 0.5f) + 0.5f.xxx;
    float3 _317 = sqrt(clamp(_288, 0.0f.xxx, 1.0f.xxx));
    uint _336 = (uint((clamp(_317.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_317.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_317.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u);
    uint2 _337 = uint2(((uint((clamp(_290.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_290.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_290.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + (uint((clamp(_253, 0.0f, 1.0f) * 255.0f) + 0.5f) << 24u), _336);
    _337.y = _336 | 117440512u;
    out_var_SV_TARGET0 = _337;
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C1 = stage_input.in_var_C1;
    in_var_C2 = stage_input.in_var_C2;
    in_var_C4 = stage_input.in_var_C4;
    in_var_C5 = stage_input.in_var_C5;
    in_var_C7 = stage_input.in_var_C7;
    in_var_C8 = stage_input.in_var_C8;
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

struct CurvedHalftimberInstanceData
{
    float3 position;
    uint color_id;
    float radius0;
    float radius1;
    float angle0;
    float angle1;
    int is_straight;
    int is_xz;
    float plank_depth;
    int caps;
    int mesh_atlas_part_idx;
    int pad0;
    int pad1;
    int pad2;
};

static const float2 _126[7] = { 0.0f.xx, float2(0.0f, 0.89999997615814208984375f), float2(0.100000001490116119384765625f, 1.0f), float2(0.89999997615814208984375f, 1.0f), float2(1.0f, 0.89999997615814208984375f), float2(1.0f, 0.0f), 0.0f.xx };
static const float _130[7] = { 0.0f, 0.89999997615814208984375f, 1.10000002384185791015625f, 1.89999997615814208984375f, 2.099999904632568359375f, 3.0f, 4.0f };
static const float2 _134[7] = { float2(-1.0f, 0.0f), float2(-1.0f, 0.0f), float2(0.0f, 1.0f), float2(0.0f, 1.0f), float2(1.0f, 0.0f), float2(1.0f, 0.0f), float2(0.0f, -1.0f) };

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
    uint2 _Globals_wood_normal_dims : packoffset(c0.y);
    uint2 _Globals_wood_roughness_dims : packoffset(c1);
    uint2 _Globals_wood_edge_damage_dims : packoffset(c1.z);
    uint2 _Globals_dirthpath_heightmap_dims : packoffset(c2);
};

ByteAddressBuffer instance_indirection_buffer : register(t14, space0);
ByteAddressBuffer nani_instance_data : register(t15, space0);

static float4 gl_Position;
static int gl_VertexIndex;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 out_var_C1;
static float3 out_var_C2;
static uint out_var_C4;
static float out_var_C5;
static float out_var_C6;
static float2 out_var_C7;
static float out_var_C8;
static uint out_var_C9;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    uint gl_VertexIndex : SV_VertexID;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(4)]] nointerpolation uint out_var_C4 : TEXCOORD4;
    [[vk::location(5)]] float out_var_C5 : TEXCOORD5;
    [[vk::location(6)]] float out_var_C6 : TEXCOORD6;
    [[vk::location(7)]] float2 out_var_C7 : TEXCOORD7;
    [[vk::location(8)]] float out_var_C8 : TEXCOORD8;
    [[vk::location(9)]] nointerpolation uint out_var_C9 : TEXCOORD9;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    uint _155;
    float4 _440;
    float3 _441;
    float3 _442;
    float _443;
    float _444;
    float2 _445;
    float _446;
    uint _447;
    do
    {
        _155 = instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0);
        int _179 = int(1u << (uint(nani_instance_data.Load<int>(_155 * 64 + 48)) & 31u));
        uint _180 = uint(gl_VertexIndex) % 7u;
        float _187 = _130[_180] / _130[5];
        bool _190 = nani_instance_data.Load<int>(_155 * 64 + 36) == 0;
        float2 _199;
        float2 _200;
        if (_190)
        {
            _199 = float2(_134[_180].yx);
            _200 = float2(_126[_180].yx);
        }
        else
        {
            _199 = _134[_180];
            _200 = _126[_180];
        }
        float _202 = 1.0f / float(_179);
        if (nani_instance_data.Load<int>(_155 * 64 + 32) > 0)
        {
            float3 _353 = float3(nani_instance_data.Load<float>(_155 * 64 + 16), 0.0f, nani_instance_data.Load<float>(_155 * 64 + 20));
            float _392;
            if (nani_instance_data.Load<int>(_155 * 64 + 32) == 2)
            {
                float _391;
                if (in_var_Vertex_Position.z < 0.5f)
                {
                    _391 = nani_instance_data.Load<float>(_155 * 64 + 28) + (nani_instance_data.Load<float>(_155 * 64 + 24) * lerp(-1.0f, 1.0f, _200.x));
                }
                else
                {
                    _391 = nani_instance_data.Load<float>(_155 * 64 + 28);
                }
                _392 = _391;
            }
            else
            {
                float _382;
                if (nani_instance_data.Load<int>(_155 * 64 + 32) == 3)
                {
                    float _381;
                    if (in_var_Vertex_Position.z > 0.5f)
                    {
                        _381 = nani_instance_data.Load<float>(_155 * 64 + 28) + (nani_instance_data.Load<float>(_155 * 64 + 24) * lerp(-1.0f, 1.0f, _200.x));
                    }
                    else
                    {
                        _381 = nani_instance_data.Load<float>(_155 * 64 + 28);
                    }
                    _382 = _381;
                }
                else
                {
                    float _372;
                    if (nani_instance_data.Load<int>(_155 * 64 + 32) == 4)
                    {
                        _372 = nani_instance_data.Load<float>(_155 * 64 + 28) + (nani_instance_data.Load<float>(_155 * 64 + 24) * lerp(-1.0f, 1.0f, _200.x));
                    }
                    else
                    {
                        _372 = nani_instance_data.Load<float>(_155 * 64 + 28);
                    }
                    _382 = _372;
                }
                _392 = _382;
            }
            float3 _402 = (nani_instance_data.Load<float3>(_155 * 64 + 0) + ((_353 * lerp(-0.5f, 0.5f, _200.x)) * nani_instance_data.Load<float>(_155 * 64 + 24))) + (((-float3(-nani_instance_data.Load<float>(_155 * 64 + 20), 0.0f, nani_instance_data.Load<float>(_155 * 64 + 16))) * lerp(-0.5f, 0.5f, in_var_Vertex_Position.z)) * _392);
            uint _408 = _155 + (_155 << 10u);
            uint _410 = _408 ^ (_408 >> 6u);
            uint _412 = _410 + (_410 << 3u);
            uint _414 = _412 ^ (_412 >> 11u);
            float _422 = (_402.y + (_200.y * nani_instance_data.Load<float>(_155 * 64 + 40))) + ((asfloat(((_414 + (_414 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * 0.00999999977648258209228515625f);
            float3 _423 = _402;
            _423.y = _422;
            _440 = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, float4(_402.x, _422, _402.z, 1.0f));
            _441 = (_353 * _199.x) + float3(0.0f, _199.y, 0.0f);
            _442 = _423;
            _443 = 0.0f;
            _444 = in_var_Vertex_Position.z;
            _445 = float2(_187, in_var_Vertex_Position.z * _392);
            _446 = -1.0f;
            _447 = uint(_179);
            break;
        }
        else
        {
            float _272;
            float _273;
            if (_190)
            {
                _272 = 0.0f;
                _273 = lerp(nani_instance_data.Load<float>(_155 * 64 + 24), nani_instance_data.Load<float>(_155 * 64 + 28), 1.0f - in_var_Vertex_Position.z);
            }
            else
            {
                float _266;
                float _267;
                if (nani_instance_data.Load<int>(_155 * 64 + 44) == 1)
                {
                    float _264;
                    float _265;
                    if (in_var_Vertex_Position.z >= _202)
                    {
                        _264 = 0.0f;
                        _265 = (in_var_Vertex_Position.z - _202) / (1.0f - _202);
                    }
                    else
                    {
                        bool _258 = in_var_Vertex_Position.z < _202;
                        _264 = float(_258);
                        _265 = _258 ? 0.0f : in_var_Vertex_Position.z;
                    }
                    _266 = _264;
                    _267 = _265;
                }
                else
                {
                    float _252;
                    float _253;
                    if (nani_instance_data.Load<int>(_155 * 64 + 44) == 2)
                    {
                        float _241 = 1.0f - _202;
                        float _250;
                        float _251;
                        if (in_var_Vertex_Position.z <= _241)
                        {
                            _250 = 0.0f;
                            _251 = in_var_Vertex_Position.z / _241;
                        }
                        else
                        {
                            bool _246 = in_var_Vertex_Position.z > _241;
                            _250 = float(_246);
                            _251 = _246 ? 1.0f : in_var_Vertex_Position.z;
                        }
                        _252 = _250;
                        _253 = _251;
                    }
                    else
                    {
                        float _239;
                        float _240;
                        if (nani_instance_data.Load<int>(_155 * 64 + 44) == 3)
                        {
                            bool _227;
                            if (in_var_Vertex_Position.z >= _202)
                            {
                                _227 = in_var_Vertex_Position.z <= (1.0f - _202);
                            }
                            else
                            {
                                _227 = false;
                            }
                            float _237;
                            if (_227)
                            {
                                _237 = (in_var_Vertex_Position.z - _202) / ((1.0f - _202) - _202);
                            }
                            else
                            {
                                _237 = (in_var_Vertex_Position.z < _202) ? 0.0f : 1.0f;
                            }
                            _239 = _227 ? 0.0f : 1.0f;
                            _240 = _237;
                        }
                        else
                        {
                            _239 = 0.0f;
                            _240 = in_var_Vertex_Position.z;
                        }
                        _252 = _239;
                        _253 = _240;
                    }
                    _266 = _252;
                    _267 = _253;
                }
                _272 = _266;
                _273 = lerp(nani_instance_data.Load<float>(_155 * 64 + 24), nani_instance_data.Load<float>(_155 * 64 + 28), _267);
            }
            float _276 = cos(_273);
            float _278 = -sin(_273);
            float3 _281 = (float3(_276, 0.0f, _278) * lerp(nani_instance_data.Load<float>(_155 * 64 + 16), nani_instance_data.Load<float>(_155 * 64 + 20), _200.x)) + nani_instance_data.Load<float3>(_155 * 64 + 0);
            float3 _322;
            if (nani_instance_data.Load<int>(_155 * 64 + 44) == 0)
            {
                float3 _321 = _281;
                _321.y = _281.y + (_200.y * nani_instance_data.Load<float>(_155 * 64 + 40));
                _322 = _321;
            }
            else
            {
                float _310;
                if (nani_instance_data.Load<int>(_155 * 64 + 44) == 1)
                {
                    _310 = ((in_var_Vertex_Position.z >= _202) ? 1.0f : 0.0f);
                }
                else
                {
                    float _306;
                    if (nani_instance_data.Load<int>(_155 * 64 + 44) == 2)
                    {
                        _306 = ((in_var_Vertex_Position.z <= (1.0f - _202)) ? 1.0f : 0.0f);
                    }
                    else
                    {
                        bool _300;
                        if (in_var_Vertex_Position.z >= _202)
                        {
                            _300 = in_var_Vertex_Position.z <= (1.0f - _202);
                        }
                        else
                        {
                            _300 = false;
                        }
                        _306 = float(_300);
                    }
                    _310 = _306;
                }
                float3 _316 = _281;
                _316.y = _281.y + ((_310 * _200.y) * nani_instance_data.Load<float>(_155 * 64 + 40));
                _322 = _316;
            }
            float _330 = 0.001953125f * (floor(_322.y * 512.0f) + frac(float(_155) * 1.61803400516510009765625f));
            float3 _331 = _322;
            _331.y = _330;
            _440 = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, float4(_322.x, _330, _322.z, 1.0f));
            _441 = float3(_276, 1.0f, _278) * float3(_199.xyx);
            _442 = _331;
            _443 = (uint(gl_VertexIndex) == 6u) ? 1.0f : _272;
            _444 = in_var_Vertex_Position.z;
            _445 = float2(_187, (_273 * nani_instance_data.Load<float>(_155 * 64 + 16)) * 3.1400001049041748046875f);
            _446 = _273 * 0.159235656261444091796875f;
            _447 = uint(_179);
            break;
        }
        break; 
    } while(false);
    gl_Position = _440;
    out_var_C1 = _441;
    out_var_C2 = _442;
    out_var_C4 = _155;
    out_var_C5 = _443;
    out_var_C6 = _444;
    out_var_C7 = _445;
    out_var_C8 = _446;
    out_var_C9 = _447;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_VertexIndex = int(stage_input.gl_VertexIndex);
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C4 = out_var_C4;
    stage_output.out_var_C5 = out_var_C5;
    stage_output.out_var_C6 = out_var_C6;
    stage_output.out_var_C7 = out_var_C7;
    stage_output.out_var_C8 = out_var_C8;
    stage_output.out_var_C9 = out_var_C9;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)

#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
void frag_main()
{
}

void ps_main()
{
    frag_main();
}
#pragma dxc diagnostic pop
#endif
