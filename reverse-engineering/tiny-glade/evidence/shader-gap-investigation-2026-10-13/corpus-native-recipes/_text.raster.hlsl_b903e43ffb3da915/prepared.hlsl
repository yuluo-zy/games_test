#define SHADER_VARIANT_DEPTH_ONLY 0
#define TGV_b903e43ffb3da915 1
#line 1 "shaders/text.raster.hlsl"


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
#line 10 "shaders/text.raster.hlsl"
            
#line 1 "shaders/inc/quad_mesh.hlsl"




#ifndef TINYGLADE_INC_QUAD_MESH_HLSL
#define TINYGLADE_INC_QUAD_MESH_HLSL


float2 quad_corner(uint vertex_id)
{
    return float2(float(vertex_id & 1u), float((vertex_id >> 1u) & 1u));
}



float4 fullscreen_triangle_clip(uint vertex_id)
{
    float2 uv = float2(float(vertex_id & 1u), float(vertex_id >> 1u));
    return float4(uv * 2.0f - 1.0f, 0.0f, 1.0f);
}


float4 unit_quad_clip(uint vertex_id)
{
    float2 corner = quad_corner(vertex_id);
    return float4(corner * 2.0f - 1.0f, 0.0f, 1.0f);
}

#endif 
#line 11 "shaders/text.raster.hlsl"

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
#line 12 "shaders/text.raster.hlsl"

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
#line 13 "shaders/text.raster.hlsl"

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
#line 14 "shaders/text.raster.hlsl"

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
#line 15 "shaders/text.raster.hlsl"

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
#line 16 "shaders/text.raster.hlsl"

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
#line 17 "shaders/text.raster.hlsl"

#line 1 "shaders/inc/image.hlsl"




#ifndef TINYGLADE_INC_IMAGE_HLSL
#define TINYGLADE_INC_IMAGE_HLSL

#line 1 "shaders/inc/curve.hlsl"















#define frexp incb_frexp

struct FrexpStructType
{
    float3 mantissa;
    int3 exponent;
};

float3 incb_frexp(float3 x, out int3 exponent)
{
    uint3 bits = asuint(x);
    int3 biased = int3(int((bits.x >> 23u) & 0xFFu),
                       int((bits.y >> 23u) & 0xFFu),
                       int((bits.z >> 23u) & 0xFFu));
    int3 e = biased - int3(127, 127, 127);
    float3 m = asfloat((bits & 0x807FFFFFu) | 0x3F800000u);

    
    bool3 big = (m >= 1.0f) | (m <= -1.0f);
    m = lerp(m, m * 0.5f, float3(big));
    e += int3(big);

    
    bool3 special = (biased == 0) | (biased == 0xFF);
    exponent = select(special, int3(0, 0, 0), e);
    return select(special, x, m);
}

float3 curve(float3 x)
{
    
    
    
    FrexpStructType f;
    int3 e;
    f.mantissa = frexp(x, e);
    f.exponent = e;
    return x + exp2(float3(f.exponent) - float3(8.0f, 8.0f, 7.0f));
}
#line 8 "shaders/inc/image.hlsl"



float2 texture_dims(Texture2D<float4> tex)
{
    uint w, h;
    tex.GetDimensions(w, h);
    return float2(w, h);
}


float sample_mask(Texture2D<float4> mask_tex, SamplerState samp, float2 uv)
{
    return mask_tex.SampleLevel(samp, uv, 0.0f).r;
}

#endif 
#line 18 "shaders/text.raster.hlsl"

#line 19 "shaders/text.raster.hlsl"

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
#line 20 "shaders/text.raster.hlsl"

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
#line 21 "shaders/text.raster.hlsl"

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
#line 22 "shaders/text.raster.hlsl"



Texture2D<float4> blue_noise_texture : register(t14, space0);
#line 1 "shaders/inc/blue_noise.hlsl"

















float4 blue_noise(Texture2D<float4> blue_noise_texture, uint2 pixel_coords, uint frame_index)
{
    uint2 offset = uint2(frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f)
        * float(frame_index)) + 0.5f.xx) * 256.0f);
    uint2 p = (pixel_coords + uint2(13u, 13u) + offset) % uint2(256u, 256u);
    float4 texel = blue_noise_texture.Load(int3(p, 0));
    return texel * 255.0f * (1.0f / 256.0f) + (1.0f / 512.0f);
}
#line 26 "shaders/text.raster.hlsl"

#line 1 "shaders/inc/hammersley.hlsl"










float radicalInverse_VdC(uint bits)
{
    bits = (bits << 16u) | (bits >> 16u);
    bits = ((bits & 0x55555555u) << 1u) | ((bits & 0xAAAAAAAAu) >> 1u);
    bits = ((bits & 0x33333333u) << 2u) | ((bits & 0xCCCCCCCCu) >> 2u);
    bits = ((bits & 0x0F0F0F0Fu) << 4u) | ((bits & 0xF0F0F0F0u) >> 4u);
    bits = ((bits & 0x00FF00FFu) << 8u) | ((bits & 0xFF00FF00u) >> 8u);
    return float(bits) * 2.3283064365386963e-10f;
}

float2 hammersley(uint i, uint N)
{
    return float2(float(i) / float(N), radicalInverse_VdC(i));
}
#line 27 "shaders/text.raster.hlsl"


cbuffer $Globals : register(b13, space0)
{
    uint2 blue_noise_texture_dims;
    float2 window_size_pixels;
    uint2 font_atlas_tex_dims;
};

struct PrimPacked
{
    float2 pos_min;
    float2 pos_extent;
    float2 uv_min;
    float2 uv_extent;
    uint color;
    uint stream;
    uint pad0;
    uint pad1;
};

ByteAddressBuffer primitives : register(t15, space0);
ByteAddressBuffer shape_stream : register(t16, space0);
Texture2D<float4> font_atlas_tex : register(t17, space0);

struct VsOut
{
    float2 uv : TEXCOORD0;
    float4 local : TEXCOORD1;
    float4 color : TEXCOORD2;
    nointerpolation uint stream : TEXCOORD3;
    float4 position : SV_Position;
};

VsOut vs_main(uint vertex_id : SV_VertexID)
{
    VsOut o;

    uint vid = vertex_id % 6u;
    uint prim = vertex_id / 6u;

    float4 corner;
    if (vid == 0u) corner = float4(0.5f, -0.5f, 1.0f, 0.0f);
    else if (vid == 1u) corner = float4(0.5f, 0.5f, 1.0f, 1.0f);
    else if (vid == 2u) corner = float4(-0.5f, 0.5f, 0.0f, 1.0f);
    else if (vid == 3u) corner = float4(-0.5f, 0.5f, 0.0f, 1.0f);
    else if (vid == 4u) corner = float4(-0.5f, -0.5f, 0.0f, 0.0f);
    else corner = float4(0.5f, -0.5f, 1.0f, 0.0f);

    float pixel_scale = window_size_pixels.y * 0.001f;

    uint packed_color = primitives.Load<uint>(prim * 48u + 32u);
    float r = float(packed_color & 255u) * (1.0f / 255.0f);
    float g_raw = float((packed_color >> 8u) & 255u);
    float b_raw = float((packed_color >> 16u) & 255u);
    float a_raw = float((packed_color >> 24u) & 255u);
    float g = g_raw * (1.0f / 255.0f);
    float b = b_raw * (1.0f / 255.0f);
    float a = a_raw * (1.0f / 255.0f);

    float da;
    float dg;
    float db;
    if (0.04045f < a) da = pow((a + 0.055f) * 0.947867333f, 2.4f);
    else da = a_raw * 0.000303526991f;
    if (0.04045f < b) db = pow((b + 0.055f) * 0.947867333f, 2.4f);
    else db = b_raw * 0.000303526991f;
    if (0.04045f < g) dg = pow((g + 0.055f) * 0.947867333f, 2.4f);
    else dg = g_raw * 0.000303526991f;

    uint stream = primitives.Load<uint>(prim * 48u + 36u);
    bool snap = (stream == 0u) || (stream == 0xFFFFFFFFu);

    float2 pos_min = snap ? round(primitives.Load<float2>(prim * 48u + 0u) * pixel_scale) / pixel_scale
                          : primitives.Load<float2>(prim * 48u + 0u);
    float2 pos = pos_min + primitives.Load<float2>(prim * 48u + 8u) * (corner.xy + 0.5f);
    float2 uv = (primitives.Load<float2>(prim * 48u + 16u) + primitives.Load<float2>(prim * 48u + 24u) * corner.zw)
              / float2(font_atlas_tex_dims);

    o.position = float4((pos / (window_size_pixels / pixel_scale) - 0.5f) * float2(2.0f, -2.0f), 0.0f, 1.0f);
    o.uv = (stream != 0u) ? corner.zw : uv;
    o.local = float4(pos - (pos_min + primitives.Load<float2>(prim * 48u + 8u) * 0.5f), primitives.Load<float2>(prim * 48u + 8u));
    o.color = float4(float3(da, db, dg) * r, r);
    o.stream = stream;
    return o;
}


float4 decode_rgba8(uint p)
{
    float r = float(p & 255u) * (1.0f / 255.0f);
    float g = float((p >> 8u) & 255u) * (1.0f / 255.0f);
    float b = float((p >> 16u) & 255u) * (1.0f / 255.0f);
    float a = float((p >> 24u) & 255u) * (1.0f / 255.0f);
    float dr;
    float dg;
    float db;
    if (0.04045f < r) dr = pow((r + 0.055f) * 0.947867333f, 2.4f);
    else dr = float(p & 255u) * 0.000303526991f;
    if (0.04045f < g) dg = pow((g + 0.055f) * 0.947867333f, 2.4f);
    else dg = float((p >> 8u) & 255u) * 0.000303526991f;
    if (0.04045f < b) db = pow((b + 0.055f) * 0.947867333f, 2.4f);
    else db = float((p >> 16u) & 255u) * 0.000303526991f;
    return float4(dr, dg, db, a);
}


float hash_noise_octave(float2 p)
{
    int2 base = int2(floor(p));
    float2 f = frac(p);
    float2 s = f * f * (3.0f - f * 2.0f);

    uint2 c00 = uint2(base);
    uint2 c10 = uint2(base + int2(1, 0));
    uint2 c01 = uint2(base + int2(0, 1));
    uint2 c11 = uint2(base + int2(1, 1));

    uint h00 = c00.y + (c00.y << 10u); h00 ^= h00 >> 6u; h00 += h00 << 3u; h00 ^= h00 >> 11u;
    h00 = ((c00.x * 1664525u) + (h00 + (h00 << 15u)) + 1013904223u) * 1664525u;
    h00 ^= h00 >> 11u; h00 ^= (h00 << 7u) & 2636928640u; h00 ^= (h00 << 15u) & 4022730752u;
    float n00 = asfloat((h00 & 0x7FFFFFu) | 0x3F800000u) - 1.0f;

    uint h10 = c10.y + (c10.y << 10u); h10 ^= h10 >> 6u; h10 += h10 << 3u; h10 ^= h10 >> 11u;
    h10 = ((c10.x * 1664525u) + (h10 + (h10 << 15u)) + 1013904223u) * 1664525u;
    h10 ^= h10 >> 11u; h10 ^= (h10 << 7u) & 2636928640u; h10 ^= (h10 << 15u) & 4022730752u;
    float n10 = asfloat((h10 & 0x7FFFFFu) | 0x3F800000u) - 1.0f;

    uint h01 = c01.y + (c01.y << 10u); h01 ^= h01 >> 6u; h01 += h01 << 3u; h01 ^= h01 >> 11u;
    h01 = ((c01.x * 1664525u) + (h01 + (h01 << 15u)) + 1013904223u) * 1664525u;
    h01 ^= h01 >> 11u; h01 ^= (h01 << 7u) & 2636928640u; h01 ^= (h01 << 15u) & 4022730752u;
    float n01 = asfloat((h01 & 0x7FFFFFu) | 0x3F800000u) - 1.0f;

    uint h11 = c11.y + (c11.y << 10u); h11 ^= h11 >> 6u; h11 += h11 << 3u; h11 ^= h11 >> 11u;
    h11 = ((c11.x * 1664525u) + (h11 + (h11 << 15u)) + 1013904223u) * 1664525u;
    h11 ^= h11 >> 11u; h11 ^= (h11 << 7u) & 2636928640u; h11 ^= (h11 << 15u) & 4022730752u;
    float n11 = asfloat((h11 & 0x7FFFFFu) | 0x3F800000u) - 1.0f;

    float nx = lerp(n00, n10, s.x);
    float ny = lerp(n01, n11, s.x);
    return lerp(nx, ny, s.y);
}

struct PsIn
{
    float2 uv : TEXCOORD0;
    float4 local : TEXCOORD1;
    float4 color : TEXCOORD2;
    nointerpolation uint stream : TEXCOORD3;
};


float4 glyph_sample(PsIn vin, float4 frag_coord)
{
    float2 n = (frag_coord.xy / window_size_pixels.y) * 1440.0f * 0.3f;
    float2 n2 = n * 2.5f;

    float rnd1 = hash_noise_octave(n2 * 0.7192f);
    float rnd2 = hash_noise_octave(n2 * 0.89812f);
    float rnd3 = hash_noise_octave(n * 0.13128f);

    float4 atlas = font_atlas_tex.SampleLevel(sampler_lnc, vin.uv, 0.0f);

    float4 c = atlas;
    c = lerp(c, min(c, font_atlas_tex.SampleLevel(sampler_lnc, vin.uv + float2(0.008f, -0.008f) * (rnd1 - 0.5f), 0.0f)), (rnd2 - 0.48f).xxxx);
    c = lerp(c, min(c, font_atlas_tex.SampleLevel(sampler_lnc, vin.uv + 0.004f * (rnd2 - 0.5f), 0.0f)), (rnd3 - 0.48f).xxxx);
    c.w *= 0.97f;
    c = clamp(c, 0.0f, 1.0f);

    float4 col = float4(c.rgb * c.a, c.a);
    if (all(atlas.rgb == float3(1.0f, 1.0f, 1.0f)))
        col = vin.color * col;
    return col;
}


float4 apply_sprite(PsIn vin, float4 acc, uint tex_idx, uint mode, float4 frag_coord)
{
    float2 uv0 = vin.uv;
    float2 px = 0.0f;
    float2 nx = 0.0f;
    if (mode == 1u)
    {
        uint w, h;
        bindless_textures_dyn[NonUniformResourceIndex(tex_idx)].GetDimensions(w, h);
        float2 dims = float2(w, h);
        uv0.x = (vin.uv.x - 0.5f) * (vin.local.z / vin.local.w) / (dims.x / dims.y) + 0.5f;
        nx = -ddy(uv0) * 0.5f;
        px = ddx(uv0) * 0.5f;
    }
    else if (mode == 2u)
    {
        uint w, h;
        bindless_textures_dyn[NonUniformResourceIndex(tex_idx)].GetDimensions(w, h);
        float2 dims = float2(w, h);
        float u = (vin.uv.x - 0.5f) * (vin.local.z / vin.local.w) / (dims.x / dims.y);
        uv0 = float2(u + 0.5f, vin.uv.y);
        float aa = smoothstep(0.0f, 1.0f, max(0.0f, (abs(u) - 0.4f) * 2.0f)) * 0.25f;
        float2 n = -ddy(uv0) * 0.5f;
        n.y += float(int(sign(n.y))) * aa * 0.5f;
        nx = n;
        float2 blue = blue_noise_texture.Load(int3((uint2(frag_coord.xy) + uint2(frac(float2(9.5585f, 7.3381f)) * 256.0f)) % uint2(256u, 256u), 0u)).rg;
        uv0 += (((blue * 255.0f) * (1.0f / 256.0f) + (1.0f / 512.0f)) - 0.5f) * aa;
        px = ddx(uv0) * 0.5f;
    }
    else if (mode == 3u)
    {
        
        uint w, h;
        bindless_textures_dyn[NonUniformResourceIndex(tex_idx)].GetDimensions(w, h);
        float2 dims = float2(w, h);
        float2 size = dims * 0.33333f;
        float2 uv_rep = vin.local.zw;
        float scale = max(1.0f, (0.66667f * max(size.x, size.y)) / vin.local.z);
        uv_rep *= scale;
        float2 uv_scaled = vin.uv * scale;
        float2 block = size * 0.33333f;
        float texel_a[2][2];
        float weight[2][2];
        for (int c = 0; c < 2; ++c)
        {
            if (uv_scaled[c] < block[c])
            {
                texel_a[c][0] = texel_a[c][1] = (uv_scaled[c] / block[c]) * 0.33333f;
                weight[c][0] = weight[c][1] = 0.5f;
            }
            else if (uv_scaled[c] < (uv_rep[c] - block[c]))
            {
                float d = uv_scaled[c] - block[c];
                float denom = uv_rep[c] - 2.0f * block[c];
                float wfrac = denom / block[c];
                texel_a[c][0] = (abs(1.0f - 2.0f * frac(0.5f + ((wfrac * d / denom) * 0.5f))) * 0.33333f) + 0.33333f;
                texel_a[c][1] = (abs(1.0f - 2.0f * frac(((wfrac * ((uv_rep[c] - block[c]) - uv_scaled[c])) / denom) * 0.5f)) * 0.33333f) + 0.33333f;
                weight[c][1] = d / denom;
                weight[c][0] = 1.0f - weight[c][1];
            }
            else
            {
                float v = (((uv_scaled[c] - (uv_rep[c] - block[c])) / block[c]) * 0.33333f) + 0.66667f;
                texel_a[c][0] = texel_a[c][1] = v;
                weight[c][0] = weight[c][1] = 0.5f;
            }
        }
        float wsum = 0.0f;
        float4 sum = 0.0f;
        for (int j = 0; j < 2; ++j)
        {
            for (int i = 0; i < 2; ++i)
            {
                float w = weight[0][i] * weight[1][j];
                w = w * w * w;
                wsum += w;
                sum += bindless_textures_dyn[NonUniformResourceIndex(tex_idx)].SampleLevel(sampler_llc, float2(texel_a[0][i], texel_a[1][j]), 0.0f) * w;
            }
        }
        float4 s = sum / wsum;
        return acc * float4(s.rgb * s.a, s.a);
    }
    float4 s = bindless_textures_dyn[NonUniformResourceIndex(tex_idx)].SampleGrad(sampler_llc, uv0, px, nx);
    return acc * float4(s.rgb * s.a, s.a);
}

float4 ps_main(PsIn pin, float4 position : SV_Position) : SV_Target0
{
    if (pin.stream == 0u)
        return pin.color;
    if (pin.stream == 0xFFFFFFFFu)
        return glyph_sample(pin, position);

    
    uint start = pin.stream >> 8u;
    uint end = (pin.stream & 255u) + start;

    float4 acc = pin.color;
    uint op = start;
    uint count = 0u;
    while (op < end)
    {
        if (count >= 256u)
            return float4(1.0f, 0.0f, 1.0f, 1.0f); 

        uint word = shape_stream.Load<uint>(((op * 4u) >> 2u) * 4u + 0u);
        uint type = word & 255u;
        if (type == 0u)
        {
            
            float4 tint = decode_rgba8(shape_stream.Load<uint>((((op + 1u) * 4u) >> 2u) * 4u + 0u));
            acc *= tint;
            op += 2u;
        }
        else if (type == 1u)
        {
            
            uint tex_idx = (word >> 8u) & 0xFFFFu;
            uint mode = (word >> 24u) & 3u;
            acc = apply_sprite(pin, acc, tex_idx, mode, position);
            op += 1u;
        }
        else if (type == 2u)
        {
            
            float radius = float((word >> 8u) & 255u);
            float2 d = (abs(pin.local.xy) - (pin.local.zw * 0.5f)) + radius;
            float mask = clamp(-(length(max(d, 0.0f)) + min(max(d.x, d.y), 0.0f) - radius - 0.5f), 0.0f, 1.0f);
            acc *= mask;
            op += 1u;
        }
        else
        {
            break;
        }
        count++;
    }

    return acc;
}

#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)

#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
struct PrimPacked
{
    float2 pos_min;
    float2 pos_extent;
    float2 uv_min;
    float2 uv_extent;
    uint color;
    uint stream;
    uint pad0;
    uint pad1;
};

cbuffer $Globals : register(b13, space0)
{
    uint2 _Globals_blue_noise_texture_dims : packoffset(c0);
    float2 _Globals_window_size_pixels : packoffset(c0.z);
    uint2 _Globals_font_atlas_tex_dims : packoffset(c1);
};

ByteAddressBuffer primitives : register(t15, space0);

static float4 gl_Position;
static int gl_VertexIndex;
static float2 out_var_C0;
static float4 out_var_C1;
static float4 out_var_C2;
static uint out_var_C3;

struct SPIRV_Cross_Input
{
    uint gl_VertexIndex : SV_VertexID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float2 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float4 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float4 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] nointerpolation uint out_var_C3 : TEXCOORD3;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    uint _96 = uint(gl_VertexIndex) % 6u;
    float4 _119;
    do
    {
        if (_96 == 0u)
        {
            _119 = float4(0.5f, -0.5f, 1.0f, 0.0f);
            break;
        }
        else
        {
            if (_96 == 1u)
            {
                _119 = float4(0.5f, 0.5f, 1.0f, 1.0f);
                break;
            }
            else
            {
                if (_96 == 2u)
                {
                    _119 = float4(-0.5f, 0.5f, 0.0f, 1.0f);
                    break;
                }
                else
                {
                    if (_96 == 3u)
                    {
                        _119 = float4(-0.5f, 0.5f, 0.0f, 1.0f);
                        break;
                    }
                    else
                    {
                        if (_96 == 4u)
                        {
                            _119 = float4(-0.5f, -0.5f, 0.0f, 0.0f);
                            break;
                        }
                        else
                        {
                            _119 = float4(0.5f, -0.5f, 1.0f, 0.0f);
                            break;
                        }
                        break; 
                    }
                    break; 
                }
                break; 
            }
            break; 
        }
        break; 
    } while(false);
    float _123 = _Globals_window_size_pixels.y * 0.001000000047497451305389404296875f;
    uint _124 = uint(gl_VertexIndex) / 6u;
    float _137 = float(primitives.Load<uint>(_124 * 48 + 32) & 255u) * 0.0039215688593685626983642578125f;
    float _140 = float((primitives.Load<uint>(_124 * 48 + 32) >> 8u) & 255u);
    float _141 = _140 * 0.0039215688593685626983642578125f;
    float _144 = float((primitives.Load<uint>(_124 * 48 + 32) >> 16u) & 255u);
    float _145 = _144 * 0.0039215688593685626983642578125f;
    float _148 = float((primitives.Load<uint>(_124 * 48 + 32) >> 24u) & 255u);
    float _149 = _148 * 0.0039215688593685626983642578125f;
    float _158;
    if (0.040449999272823333740234375f < _149)
    {
        _158 = pow((_149 + 0.054999999701976776123046875f) * 0.947867333889007568359375f, 2.400000095367431640625f);
    }
    else
    {
        _158 = _148 * 0.000303526991046965122222900390625f;
    }
    float _167;
    if (0.040449999272823333740234375f < _145)
    {
        _167 = pow((_145 + 0.054999999701976776123046875f) * 0.947867333889007568359375f, 2.400000095367431640625f);
    }
    else
    {
        _167 = _144 * 0.000303526991046965122222900390625f;
    }
    float _176;
    if (0.040449999272823333740234375f < _141)
    {
        _176 = pow((_141 + 0.054999999701976776123046875f) * 0.947867333889007568359375f, 2.400000095367431640625f);
    }
    else
    {
        _176 = _140 * 0.000303526991046965122222900390625f;
    }
    bool _185;
    if (!(primitives.Load<uint>(_124 * 48 + 36) == 0u))
    {
        _185 = primitives.Load<uint>(_124 * 48 + 36) == 4294967295u;
    }
    else
    {
        _185 = true;
    }
    float2 _192;
    if (_185)
    {
        _192 = round(primitives.Load<float2>(_124 * 48 + 0) * _123) / _123.xx;
    }
    else
    {
        _192 = primitives.Load<float2>(_124 * 48 + 0);
    }
    float2 _196 = _192 + (primitives.Load<float2>(_124 * 48 + 8) * (_119.xy + 0.5f.xx));
    float2 _203 = (primitives.Load<float2>(_124 * 48 + 16) + (primitives.Load<float2>(_124 * 48 + 24) * _119.zw)) / float2(_Globals_font_atlas_tex_dims);
    bool2 _205 = (primitives.Load<uint>(_124 * 48 + 36) != 0u).xx;
    gl_Position = float4(((_196 / (_Globals_window_size_pixels / _123.xx)) - 0.5f.xx) * float2(2.0f, -2.0f), 0.0f, 1.0f);
    out_var_C0 = float2(_205.x ? _119.zw.x : _203.x, _205.y ? _119.zw.y : _203.y);
    out_var_C1 = float4(_196 - (_192 + (primitives.Load<float2>(_124 * 48 + 8) * 0.5f)), primitives.Load<float2>(_124 * 48 + 8));
    out_var_C2 = float4(float3(_158, _167, _176).xyz * _137, _137);
    out_var_C3 = primitives.Load<uint>(_124 * 48 + 36);
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_VertexIndex = int(stage_input.gl_VertexIndex);
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
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
cbuffer $Globals : register(b13, space0)
{
    uint2 _Globals_blue_noise_texture_dims : packoffset(c0);
    float2 _Globals_window_size_pixels : packoffset(c0.z);
    uint2 _Globals_font_atlas_tex_dims : packoffset(c1);
};

ByteAddressBuffer shape_stream : register(t16, space0);
SamplerState sampler_lnc : register(s0, space0);
SamplerState sampler_llc : register(s3, space0);
Texture2D<float4> bindless_textures_dyn[] : register(t0, space2);
Texture2D<float4> blue_noise_texture : register(t14, space0);
Texture2D<float4> font_atlas_tex : register(t17, space0);

static float4 gl_FragCoord;
static float2 in_var_C0;
static float4 in_var_C1;
static float4 in_var_C2;
static uint in_var_C3;
static float4 out_var_SV_TARGET;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float2 in_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float4 in_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float4 in_var_C2 : TEXCOORD2;
    [[vk::location(3)]] nointerpolation uint in_var_C3 : TEXCOORD3;
    float4 gl_FragCoord : SV_Position;
};

struct SPIRV_Cross_Output
{
    float4 out_var_SV_TARGET : SV_Target0;
};

uint2 spvTextureSize(Texture2D<float4> Tex, uint Level, out uint Param)
{
    uint2 ret;
    Tex.GetDimensions(Level, ret.x, ret.y, Param);
    return ret;
}

void frag_main()
{
    float2 _162 = in_var_C0;
    uint2 _167 = uint2(gl_FragCoord.xy);
    bool _172;
    if (in_var_C3 != 0u)
    {
        _172 = in_var_C3 != 4294967295u;
    }
    else
    {
        _172 = false;
    }
    float4 _1173;
    if (_172)
    {
        uint _821 = in_var_C3 >> 8u;
        uint _823 = (in_var_C3 & 255u) + _821;
        float4 _825;
        uint _830;
        _825 = in_var_C2;
        _830 = 0u;
        float _159[2][2];
        float _160[2][2];
        float4 _826;
        uint _829;
        uint _828 = _821;
        for (;;)
        {
            bool _838;
            if (_830 < 256u)
            {
                _838 = _828 < _823;
            }
            else
            {
                _838 = false;
            }
            if (_838)
            {
                uint _841 = (_828 * 4u) >> 2u;
                float4 _1168;
                uint _1169;
                switch (int(shape_stream.Load<uint>(_841 * 4 + 0) & 255u))
                {
                    case 0:
                    {
                        uint _1117 = ((_828 + 1u) * 4u) >> 2u;
                        float _1122 = float(shape_stream.Load<uint>(_1117 * 4 + 0) & 255u) * 0.0039215688593685626983642578125f;
                        float _1125 = float((shape_stream.Load<uint>(_1117 * 4 + 0) >> 8u) & 255u);
                        float _1126 = _1125 * 0.0039215688593685626983642578125f;
                        float _1129 = float((shape_stream.Load<uint>(_1117 * 4 + 0) >> 16u) & 255u);
                        float _1130 = _1129 * 0.0039215688593685626983642578125f;
                        float _1133 = float((shape_stream.Load<uint>(_1117 * 4 + 0) >> 24u) & 255u);
                        float _1134 = _1133 * 0.0039215688593685626983642578125f;
                        float _1144;
                        if (0.040449999272823333740234375f < _1134)
                        {
                            _1144 = pow((_1134 + 0.054999999701976776123046875f) * 0.947867333889007568359375f, 2.400000095367431640625f);
                        }
                        else
                        {
                            _1144 = _1133 * 0.000303526991046965122222900390625f;
                        }
                        float _1153;
                        if (0.040449999272823333740234375f < _1130)
                        {
                            _1153 = pow((_1130 + 0.054999999701976776123046875f) * 0.947867333889007568359375f, 2.400000095367431640625f);
                        }
                        else
                        {
                            _1153 = _1129 * 0.000303526991046965122222900390625f;
                        }
                        float _1162;
                        if (0.040449999272823333740234375f < _1126)
                        {
                            _1162 = pow((_1126 + 0.054999999701976776123046875f) * 0.947867333889007568359375f, 2.400000095367431640625f);
                        }
                        else
                        {
                            _1162 = _1125 * 0.000303526991046965122222900390625f;
                        }
                        float3 _1164 = float3(_1144, _1153, _1162) * _1122;
                        _1168 = _825 * float4(_1164.x, _1164.y, _1164.z, float4(_1122, _1126, _1130, _1134).x);
                        _1169 = _828 + 2u;
                        break;
                    }
                    case 1:
                    {
                        uint _42 = (shape_stream.Load<uint>(_841 * 4 + 0) >> 8u) & 65535u;
                        float2 _1103;
                        float2 _1104;
                        float2 _1105;
                        switch (int((shape_stream.Load<uint>(_841 * 4 + 0) >> 24u) & 3u))
                        {
                            case 0:
                            {
                                _1103 = (-ddy(_162)) * 0.5f;
                                _1104 = _162;
                                _1105 = ddx(_162) * 0.5f;
                                break;
                            }
                            case 1:
                            {
                                uint _1081_dummy_parameter;
                                uint2 _1081 = spvTextureSize(bindless_textures_dyn[NonUniformResourceIndex(_42)], uint(0), _1081_dummy_parameter);
                                float2 _1092 = _162;
                                _1092.x = (((_162.x - 0.5f) * (in_var_C1.z / in_var_C1.w)) / (float(_1081.x) / float(_1081.y))) + 0.5f;
                                _1103 = (-ddy(_1092)) * 0.5f;
                                _1104 = _1092;
                                _1105 = ddx(_1092) * 0.5f;
                                break;
                            }
                            case 2:
                            {
                                uint _1033_dummy_parameter;
                                uint2 _1033 = spvTextureSize(bindless_textures_dyn[NonUniformResourceIndex(_42)], uint(0), _1033_dummy_parameter);
                                float _1042 = ((_162.x - 0.5f) * (in_var_C1.z / in_var_C1.w)) / (float(_1033.x) / float(_1033.y));
                                float2 _1044 = _162;
                                _1044.x = _1042 + 0.5f;
                                float _1050 = smoothstep(0.0f, 1.0f, max(0.0f, (abs(_1042) - 0.4000000059604644775390625f) * 2.0f)) * 0.25f;
                                float2 _1067 = (-ddy(_1044)) * 0.5f;
                                float _1070 = _1067.y;
                                _1067.y = _1070 + ((float(int(sign(_1070))) * _1050) * 0.5f);
                                _1103 = _1067;
                                _1104 = _1044 + (((((blue_noise_texture.Load(int3((_167 + uint2(frac(float2(9.55853271484375f, 7.338083744049072265625f)) * 256.0f.xx)) % uint2(256u, 256u), 0u)) * 255.0f) * 0.00390625f.xxxx) + 0.001953125f.xxxx).xy - 0.5f.xx) * _1050);
                                _1105 = ddx(_1044) * 0.5f;
                                break;
                            }
                            case 3:
                            {
                                uint _883_dummy_parameter;
                                uint2 _883 = spvTextureSize(bindless_textures_dyn[NonUniformResourceIndex(_42)], uint(0), _883_dummy_parameter);
                                float2 _889 = float2(float(_883.x), float(_883.y)) * 0.3333300054073333740234375f;
                                float2 _156 = in_var_C1.zw;
                                float2 _898 = _156;
                                float2 _899 = _898 * max(1.0f, (0.666666686534881591796875f * max(_889.x, _889.y)) / in_var_C1.z);
                                _156 = _899;
                                float2 _157 = _162 * _899;
                                float2 _158 = _889 * 0.3333333432674407958984375f.xx;
                                for (uint _903 = 0u; _903 < 2u; _903++)
                                {
                                    if (_157[_903] < _158[_903])
                                    {
                                        float _982 = (_157[_903] / _158[_903]) * 0.3333333432674407958984375f;
                                        _159[_903][1] = _982;
                                        _159[_903][0] = _982;
                                        _160[_903][1] = 0.5f;
                                        _160[_903][0] = 0.5f;
                                    }
                                    else
                                    {
                                        if (_157[_903] < (_156[_903] - _158[_903]))
                                        {
                                            float _941 = _157[_903] - _158[_903];
                                            float _950 = _156[_903] - (2.0f * _158[_903]);
                                            float _952 = _950 / _158[_903];
                                            _159[_903][0] = (abs(1.0f - (2.0f * frac(0.5f + (((_952 * _941) / _950) * 0.5f)))) * 0.3333333432674407958984375f) + 0.3333333432674407958984375f;
                                            _159[_903][1] = (abs(1.0f - (2.0f * frac(((_952 * ((_156[_903] - _158[_903]) - _157[_903])) / _950) * 0.5f))) * 0.3333333432674407958984375f) + 0.3333333432674407958984375f;
                                            _160[_903][1] = _941 / _950;
                                            _160[_903][0] = 1.0f - _160[_903][1];
                                        }
                                        else
                                        {
                                            float _934 = (((_157[_903] - (_156[_903] - _158[_903])) / _158[_903]) * 0.3333333432674407958984375f) + 0.666666686534881591796875f;
                                            _159[_903][1] = _934;
                                            _159[_903][0] = _934;
                                            _160[_903][1] = 0.5f;
                                            _160[_903][0] = 0.5f;
                                        }
                                    }
                                }
                                float _988;
                                float4 _991;
                                uint _993;
                                _988 = 0.0f;
                                _991 = 0.0f.xxxx;
                                _993 = 0u;
                                float _989;
                                float4 _992;
                                for (; _993 < 2u; _988 = _989, _991 = _992, _993++)
                                {
                                    _989 = _988;
                                    _992 = _991;
                                    for (uint _1002 = 0u; _1002 < 2u; )
                                    {
                                        float _1010 = _160[0][_1002] * _160[1][_993];
                                        float _1012 = (_1010 * _1010) * _1010;
                                        _989 += _1012;
                                        _992 += (bindless_textures_dyn[NonUniformResourceIndex(_42)].SampleLevel(sampler_llc, float2(_159[0][_1002], _159[1][_993]), 0.0f) * _1012);
                                        _1002++;
                                        continue;
                                    }
                                }
                                float4 _1023 = _991 / _988.xxxx;
                                float3 _1026 = _1023.xyz * _1023.w;
                                _826 = _825 * float4(_1026.x, _1026.y, _1026.z, _1023.w);
                                _829 = _828 + 1u;
                                uint _831 = _830 + 1u;
                                _825 = _826;
                                _828 = _829;
                                _830 = _831;
                                continue;
                            }
                            default:
                            {
                                _1103 = 0.0f.xx;
                                _1104 = _162;
                                _1105 = 0.0f.xx;
                                break;
                            }
                        }
                        float4 _1108 = bindless_textures_dyn[NonUniformResourceIndex(_42)].SampleGrad(sampler_llc, _1104, _1105, _1103);
                        float3 _1111 = _1108.xyz * _1108.w;
                        _1168 = _825 * float4(_1111.x, _1111.y, _1111.z, _1108.w);
                        _1169 = _828 + 1u;
                        break;
                    }
                    case 2:
                    {
                        float _855 = float((shape_stream.Load<uint>(_841 * 4 + 0) >> 8u) & 255u);
                        float2 _859 = (abs(in_var_C1.xy) - (in_var_C1.zw * 0.5f.xx)) + _855.xx;
                        _1168 = _825 * clamp((((length(max(_859, 0.0f.xx)) + min(max(_859.x, _859.y), 0.0f)) - _855) - 0.5f) * (-1.0f), 0.0f, 1.0f);
                        _1169 = _828 + 1u;
                        break;
                    }
                    default:
                    {
                        _1168 = _825;
                        _1169 = _828;
                        break;
                    }
                }
                _826 = _1168;
                _829 = _1169;
                uint _831 = _830 + 1u;
                _825 = _826;
                _828 = _829;
                _830 = _831;
                continue;
            }
            else
            {
                break;
            }
        }
        bool4 _1171 = (_830 == 256u).xxxx;
        _1173 = float4(_1171.x ? float4(1.0f, 0.0f, 1.0f, 1.0f).x : _825.x, _1171.y ? float4(1.0f, 0.0f, 1.0f, 1.0f).y : _825.y, _1171.z ? float4(1.0f, 0.0f, 1.0f, 1.0f).z : _825.z, _1171.w ? float4(1.0f, 0.0f, 1.0f, 1.0f).w : _825.w);
    }
    else
    {
        float4 _820;
        if (in_var_C3 != 4294967295u)
        {
            float2 _184 = ((gl_FragCoord.xy / _Globals_window_size_pixels.y.xx) * 1440.0f) * 0.300000011920928955078125f;
            float2 _185 = _184 * 2.5f;
            float4 _189 = font_atlas_tex.SampleLevel(sampler_lnc, _162, 0.0f);
            float3 _190 = _189.xyz;
            float2 _193 = _185 * 0.71920001506805419921875f;
            int2 _195 = int2(floor(_193));
            float2 _196 = frac(_193);
            uint2 _197 = uint2(_195);
            uint _199 = _197.y;
            uint _201 = _199 + (_199 << 10u);
            uint _203 = _201 ^ (_201 >> 6u);
            uint _205 = _203 + (_203 << 3u);
            uint _207 = _205 ^ (_205 >> 11u);
            uint _213 = (((_197.x * 1664525u) + (_207 + (_207 << 15u))) + 1013904223u) * 1664525u;
            uint _215 = _213 ^ (_213 >> 11u);
            uint _218 = _215 ^ ((_215 << 7u) & 2636928640u);
            uint _221 = _218 ^ ((_218 << 15u) & 4022730752u);
            uint2 _229 = uint2(_195 + int2(1, 0));
            uint _231 = _229.y;
            uint _233 = _231 + (_231 << 10u);
            uint _235 = _233 ^ (_233 >> 6u);
            uint _237 = _235 + (_235 << 3u);
            uint _239 = _237 ^ (_237 >> 11u);
            uint _245 = (((_229.x * 1664525u) + (_239 + (_239 << 15u))) + 1013904223u) * 1664525u;
            uint _247 = _245 ^ (_245 >> 11u);
            uint _250 = _247 ^ ((_247 << 7u) & 2636928640u);
            uint _253 = _250 ^ ((_250 << 15u) & 4022730752u);
            uint2 _261 = uint2(_195 + int2(0, 1));
            uint _263 = _261.y;
            uint _265 = _263 + (_263 << 10u);
            uint _267 = _265 ^ (_265 >> 6u);
            uint _269 = _267 + (_267 << 3u);
            uint _271 = _269 ^ (_269 >> 11u);
            uint _277 = (((_261.x * 1664525u) + (_271 + (_271 << 15u))) + 1013904223u) * 1664525u;
            uint _279 = _277 ^ (_277 >> 11u);
            uint _282 = _279 ^ ((_279 << 7u) & 2636928640u);
            uint _285 = _282 ^ ((_282 << 15u) & 4022730752u);
            uint2 _293 = uint2(_195 + int2(1, 1));
            uint _295 = _293.y;
            uint _297 = _295 + (_295 << 10u);
            uint _299 = _297 ^ (_297 >> 6u);
            uint _301 = _299 + (_299 << 3u);
            uint _303 = _301 ^ (_301 >> 11u);
            uint _309 = (((_293.x * 1664525u) + (_303 + (_303 << 15u))) + 1013904223u) * 1664525u;
            uint _311 = _309 ^ (_309 >> 11u);
            uint _314 = _311 ^ ((_311 << 7u) & 2636928640u);
            uint _317 = _314 ^ ((_314 << 15u) & 4022730752u);
            float2 _327 = (_196 * _196) * (3.0f.xx - (_196 * 2.0f));
            float _328 = _327.x;
            float _329 = 1.0f - _328;
            float _331 = _327.y;
            float2 _347 = _185 * 0.898119986057281494140625f;
            int2 _349 = int2(floor(_347));
            float2 _350 = frac(_347);
            uint2 _351 = uint2(_349);
            uint _353 = _351.y;
            uint _355 = _353 + (_353 << 10u);
            uint _357 = _355 ^ (_355 >> 6u);
            uint _359 = _357 + (_357 << 3u);
            uint _361 = _359 ^ (_359 >> 11u);
            uint _367 = (((_351.x * 1664525u) + (_361 + (_361 << 15u))) + 1013904223u) * 1664525u;
            uint _369 = _367 ^ (_367 >> 11u);
            uint _372 = _369 ^ ((_369 << 7u) & 2636928640u);
            uint _375 = _372 ^ ((_372 << 15u) & 4022730752u);
            uint2 _383 = uint2(_349 + int2(1, 0));
            uint _385 = _383.y;
            uint _387 = _385 + (_385 << 10u);
            uint _389 = _387 ^ (_387 >> 6u);
            uint _391 = _389 + (_389 << 3u);
            uint _393 = _391 ^ (_391 >> 11u);
            uint _399 = (((_383.x * 1664525u) + (_393 + (_393 << 15u))) + 1013904223u) * 1664525u;
            uint _401 = _399 ^ (_399 >> 11u);
            uint _404 = _401 ^ ((_401 << 7u) & 2636928640u);
            uint _407 = _404 ^ ((_404 << 15u) & 4022730752u);
            uint2 _415 = uint2(_349 + int2(0, 1));
            uint _417 = _415.y;
            uint _419 = _417 + (_417 << 10u);
            uint _421 = _419 ^ (_419 >> 6u);
            uint _423 = _421 + (_421 << 3u);
            uint _425 = _423 ^ (_423 >> 11u);
            uint _431 = (((_415.x * 1664525u) + (_425 + (_425 << 15u))) + 1013904223u) * 1664525u;
            uint _433 = _431 ^ (_431 >> 11u);
            uint _436 = _433 ^ ((_433 << 7u) & 2636928640u);
            uint _439 = _436 ^ ((_436 << 15u) & 4022730752u);
            uint2 _447 = uint2(_349 + int2(1, 1));
            uint _449 = _447.y;
            uint _451 = _449 + (_449 << 10u);
            uint _453 = _451 ^ (_451 >> 6u);
            uint _455 = _453 + (_453 << 3u);
            uint _457 = _455 ^ (_455 >> 11u);
            uint _463 = (((_447.x * 1664525u) + (_457 + (_457 << 15u))) + 1013904223u) * 1664525u;
            uint _465 = _463 ^ (_463 >> 11u);
            uint _468 = _465 ^ ((_465 << 7u) & 2636928640u);
            uint _471 = _468 ^ ((_468 << 15u) & 4022730752u);
            float2 _481 = (_350 * _350) * (3.0f.xx - (_350 * 2.0f));
            float _482 = _481.x;
            float _483 = 1.0f - _482;
            float _485 = _481.y;
            float2 _502 = _184 * 0.131280004978179931640625f;
            int2 _504 = int2(floor(_502));
            float2 _505 = frac(_502);
            uint2 _506 = uint2(_504);
            uint _508 = _506.y;
            uint _510 = _508 + (_508 << 10u);
            uint _512 = _510 ^ (_510 >> 6u);
            uint _514 = _512 + (_512 << 3u);
            uint _516 = _514 ^ (_514 >> 11u);
            uint _522 = (((_506.x * 1664525u) + (_516 + (_516 << 15u))) + 1013904223u) * 1664525u;
            uint _524 = _522 ^ (_522 >> 11u);
            uint _527 = _524 ^ ((_524 << 7u) & 2636928640u);
            uint _530 = _527 ^ ((_527 << 15u) & 4022730752u);
            uint2 _538 = uint2(_504 + int2(1, 0));
            uint _540 = _538.y;
            uint _542 = _540 + (_540 << 10u);
            uint _544 = _542 ^ (_542 >> 6u);
            uint _546 = _544 + (_544 << 3u);
            uint _548 = _546 ^ (_546 >> 11u);
            uint _554 = (((_538.x * 1664525u) + (_548 + (_548 << 15u))) + 1013904223u) * 1664525u;
            uint _556 = _554 ^ (_554 >> 11u);
            uint _559 = _556 ^ ((_556 << 7u) & 2636928640u);
            uint _562 = _559 ^ ((_559 << 15u) & 4022730752u);
            uint2 _570 = uint2(_504 + int2(0, 1));
            uint _572 = _570.y;
            uint _574 = _572 + (_572 << 10u);
            uint _576 = _574 ^ (_574 >> 6u);
            uint _578 = _576 + (_576 << 3u);
            uint _580 = _578 ^ (_578 >> 11u);
            uint _586 = (((_570.x * 1664525u) + (_580 + (_580 << 15u))) + 1013904223u) * 1664525u;
            uint _588 = _586 ^ (_586 >> 11u);
            uint _591 = _588 ^ ((_588 << 7u) & 2636928640u);
            uint _594 = _591 ^ ((_591 << 15u) & 4022730752u);
            uint2 _602 = uint2(_504 + int2(1, 1));
            uint _604 = _602.y;
            uint _606 = _604 + (_604 << 10u);
            uint _608 = _606 ^ (_606 >> 6u);
            uint _610 = _608 + (_608 << 3u);
            uint _612 = _610 ^ (_610 >> 11u);
            uint _618 = (((_602.x * 1664525u) + (_612 + (_612 << 15u))) + 1013904223u) * 1664525u;
            uint _620 = _618 ^ (_618 >> 11u);
            uint _623 = _620 ^ ((_620 << 7u) & 2636928640u);
            uint _626 = _623 ^ ((_623 << 15u) & 4022730752u);
            float2 _636 = (_505 * _505) * (3.0f.xx - (_505 * 2.0f));
            float _637 = _636.x;
            float _638 = 1.0f - _637;
            float _640 = _636.y;
            float4 _653 = lerp(_189, min(_189, font_atlas_tex.SampleLevel(sampler_lnc, _162 + (float2(0.008000000379979610443115234375f, -0.008000000379979610443115234375f) * (((((1.0f - _331) * (((asfloat(((_221 ^ (_221 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _329) + ((asfloat(((_253 ^ (_253 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _328))) + (((asfloat(((_285 ^ (_285 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _329) * _331)) + (((asfloat(((_317 ^ (_317 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _328) * _331)) - 0.5f)), 0.0f)), (((((1.0f - _640) * (((asfloat(((_530 ^ (_530 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _638) + ((asfloat(((_562 ^ (_562 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _637))) + (((asfloat(((_594 ^ (_594 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _638) * _640)) + (((asfloat(((_626 ^ (_626 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _637) * _640)) + (-0.4799999892711639404296875f)).xxxx);
            float2 _655 = _184 * (-0.08247999846935272216796875f);
            int2 _657 = int2(floor(_655));
            float2 _658 = frac(_655);
            uint2 _659 = uint2(_657);
            uint _661 = _659.y;
            uint _663 = _661 + (_661 << 10u);
            uint _665 = _663 ^ (_663 >> 6u);
            uint _667 = _665 + (_665 << 3u);
            uint _669 = _667 ^ (_667 >> 11u);
            uint _675 = (((_659.x * 1664525u) + (_669 + (_669 << 15u))) + 1013904223u) * 1664525u;
            uint _677 = _675 ^ (_675 >> 11u);
            uint _680 = _677 ^ ((_677 << 7u) & 2636928640u);
            uint _683 = _680 ^ ((_680 << 15u) & 4022730752u);
            uint2 _691 = uint2(_657 + int2(1, 0));
            uint _693 = _691.y;
            uint _695 = _693 + (_693 << 10u);
            uint _697 = _695 ^ (_695 >> 6u);
            uint _699 = _697 + (_697 << 3u);
            uint _701 = _699 ^ (_699 >> 11u);
            uint _707 = (((_691.x * 1664525u) + (_701 + (_701 << 15u))) + 1013904223u) * 1664525u;
            uint _709 = _707 ^ (_707 >> 11u);
            uint _712 = _709 ^ ((_709 << 7u) & 2636928640u);
            uint _715 = _712 ^ ((_712 << 15u) & 4022730752u);
            uint2 _723 = uint2(_657 + int2(0, 1));
            uint _725 = _723.y;
            uint _727 = _725 + (_725 << 10u);
            uint _729 = _727 ^ (_727 >> 6u);
            uint _731 = _729 + (_729 << 3u);
            uint _733 = _731 ^ (_731 >> 11u);
            uint _739 = (((_723.x * 1664525u) + (_733 + (_733 << 15u))) + 1013904223u) * 1664525u;
            uint _741 = _739 ^ (_739 >> 11u);
            uint _744 = _741 ^ ((_741 << 7u) & 2636928640u);
            uint _747 = _744 ^ ((_744 << 15u) & 4022730752u);
            uint2 _755 = uint2(_657 + int2(1, 1));
            uint _757 = _755.y;
            uint _759 = _757 + (_757 << 10u);
            uint _761 = _759 ^ (_759 >> 6u);
            uint _763 = _761 + (_761 << 3u);
            uint _765 = _763 ^ (_763 >> 11u);
            uint _771 = (((_755.x * 1664525u) + (_765 + (_765 << 15u))) + 1013904223u) * 1664525u;
            uint _773 = _771 ^ (_771 >> 11u);
            uint _776 = _773 ^ ((_773 << 7u) & 2636928640u);
            uint _779 = _776 ^ ((_776 << 15u) & 4022730752u);
            float2 _789 = (_658 * _658) * (3.0f.xx - (_658 * 2.0f));
            float _790 = _789.x;
            float _791 = 1.0f - _790;
            float _793 = _789.y;
            float4 _806 = lerp(_653, min(_653, font_atlas_tex.SampleLevel(sampler_lnc, _162 + (0.0040000001899898052215576171875f.xx * (((((1.0f - _485) * (((asfloat(((_375 ^ (_375 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _483) + ((asfloat(((_407 ^ (_407 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _482))) + (((asfloat(((_439 ^ (_439 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _483) * _485)) + (((asfloat(((_471 ^ (_471 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _482) * _485)) - 0.5f)), 0.0f)), (((((1.0f - _793) * (((asfloat(((_683 ^ (_683 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _791) + ((asfloat(((_715 ^ (_715 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _790))) + (((asfloat(((_747 ^ (_747 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _791) * _793)) + (((asfloat(((_779 ^ (_779 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _790) * _793)) + (-0.4799999892711639404296875f)).xxxx);
            _806.w = _806.w * 0.9700000286102294921875f;
            float4 _810 = clamp(_806, 0.0f.xxxx, 1.0f.xxxx);
            float3 _813 = _810.xyz * _810.w;
            float4 _814 = float4(_813.x, _813.y, _813.z, _810.w);
            float4 _819;
            if (all(bool3(_190.x == 1.0f.xxx.x, _190.y == 1.0f.xxx.y, _190.z == 1.0f.xxx.z)))
            {
                _819 = in_var_C2 * _814;
            }
            else
            {
                _819 = _814;
            }
            _820 = _819;
        }
        else
        {
            _820 = in_var_C2;
        }
        _1173 = _820;
    }
    out_var_SV_TARGET = _1173;
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    gl_FragCoord = stage_input.gl_FragCoord;
    gl_FragCoord.w = 1.0 / gl_FragCoord.w;
    in_var_C0 = stage_input.in_var_C0;
    in_var_C1 = stage_input.in_var_C1;
    in_var_C2 = stage_input.in_var_C2;
    in_var_C3 = stage_input.in_var_C3;
    frag_main();
    SPIRV_Cross_Output stage_output;
    stage_output.out_var_SV_TARGET = out_var_SV_TARGET;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)

#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
struct PrimPacked
{
    float2 pos_min;
    float2 pos_extent;
    float2 uv_min;
    float2 uv_extent;
    uint color;
    uint stream;
    uint pad0;
    uint pad1;
};

cbuffer $Globals : register(b13, space0)
{
    uint2 _Globals_blue_noise_texture_dims : packoffset(c0);
    float2 _Globals_window_size_pixels : packoffset(c0.z);
    uint2 _Globals_font_atlas_tex_dims : packoffset(c1);
};

ByteAddressBuffer primitives : register(t15, space0);

static float4 gl_Position;
static int gl_VertexIndex;
static float2 out_var_C0;
static float4 out_var_C1;
static float4 out_var_C2;
static uint out_var_C3;

struct SPIRV_Cross_Input
{
    uint gl_VertexIndex : SV_VertexID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float2 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float4 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float4 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] nointerpolation uint out_var_C3 : TEXCOORD3;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    uint _96 = uint(gl_VertexIndex) % 6u;
    float4 _119;
    do
    {
        if (_96 == 0u)
        {
            _119 = float4(0.5f, -0.5f, 1.0f, 0.0f);
            break;
        }
        else
        {
            if (_96 == 1u)
            {
                _119 = float4(0.5f, 0.5f, 1.0f, 1.0f);
                break;
            }
            else
            {
                if (_96 == 2u)
                {
                    _119 = float4(-0.5f, 0.5f, 0.0f, 1.0f);
                    break;
                }
                else
                {
                    if (_96 == 3u)
                    {
                        _119 = float4(-0.5f, 0.5f, 0.0f, 1.0f);
                        break;
                    }
                    else
                    {
                        if (_96 == 4u)
                        {
                            _119 = float4(-0.5f, -0.5f, 0.0f, 0.0f);
                            break;
                        }
                        else
                        {
                            _119 = float4(0.5f, -0.5f, 1.0f, 0.0f);
                            break;
                        }
                        break; 
                    }
                    break; 
                }
                break; 
            }
            break; 
        }
        break; 
    } while(false);
    float _123 = _Globals_window_size_pixels.y * 0.001000000047497451305389404296875f;
    uint _124 = uint(gl_VertexIndex) / 6u;
    float _137 = float(primitives.Load<uint>(_124 * 48 + 32) & 255u) * 0.0039215688593685626983642578125f;
    float _140 = float((primitives.Load<uint>(_124 * 48 + 32) >> 8u) & 255u);
    float _141 = _140 * 0.0039215688593685626983642578125f;
    float _144 = float((primitives.Load<uint>(_124 * 48 + 32) >> 16u) & 255u);
    float _145 = _144 * 0.0039215688593685626983642578125f;
    float _148 = float((primitives.Load<uint>(_124 * 48 + 32) >> 24u) & 255u);
    float _149 = _148 * 0.0039215688593685626983642578125f;
    float _158;
    if (0.040449999272823333740234375f < _149)
    {
        _158 = pow((_149 + 0.054999999701976776123046875f) * 0.947867333889007568359375f, 2.400000095367431640625f);
    }
    else
    {
        _158 = _148 * 0.000303526991046965122222900390625f;
    }
    float _167;
    if (0.040449999272823333740234375f < _145)
    {
        _167 = pow((_145 + 0.054999999701976776123046875f) * 0.947867333889007568359375f, 2.400000095367431640625f);
    }
    else
    {
        _167 = _144 * 0.000303526991046965122222900390625f;
    }
    float _176;
    if (0.040449999272823333740234375f < _141)
    {
        _176 = pow((_141 + 0.054999999701976776123046875f) * 0.947867333889007568359375f, 2.400000095367431640625f);
    }
    else
    {
        _176 = _140 * 0.000303526991046965122222900390625f;
    }
    bool _185;
    if (!(primitives.Load<uint>(_124 * 48 + 36) == 0u))
    {
        _185 = primitives.Load<uint>(_124 * 48 + 36) == 4294967295u;
    }
    else
    {
        _185 = true;
    }
    float2 _192;
    if (_185)
    {
        _192 = round(primitives.Load<float2>(_124 * 48 + 0) * _123) / _123.xx;
    }
    else
    {
        _192 = primitives.Load<float2>(_124 * 48 + 0);
    }
    float2 _196 = _192 + (primitives.Load<float2>(_124 * 48 + 8) * (_119.xy + 0.5f.xx));
    float2 _203 = (primitives.Load<float2>(_124 * 48 + 16) + (primitives.Load<float2>(_124 * 48 + 24) * _119.zw)) / float2(_Globals_font_atlas_tex_dims);
    bool2 _205 = (primitives.Load<uint>(_124 * 48 + 36) != 0u).xx;
    gl_Position = float4(((_196 / (_Globals_window_size_pixels / _123.xx)) - 0.5f.xx) * float2(2.0f, -2.0f), 0.0f, 1.0f);
    out_var_C0 = float2(_205.x ? _119.zw.x : _203.x, _205.y ? _119.zw.y : _203.y);
    out_var_C1 = float4(_196 - (_192 + (primitives.Load<float2>(_124 * 48 + 8) * 0.5f)), primitives.Load<float2>(_124 * 48 + 8));
    out_var_C2 = float4(float3(_158, _167, _176).xyz * _137, _137);
    out_var_C3 = primitives.Load<uint>(_124 * 48 + 36);
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_VertexIndex = int(stage_input.gl_VertexIndex);
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
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
