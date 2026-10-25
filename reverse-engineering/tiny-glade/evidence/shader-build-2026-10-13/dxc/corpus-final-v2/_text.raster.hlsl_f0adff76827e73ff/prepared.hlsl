#define SHADER_VARIANT_DEPTH_ONLY 1
#line 1 "shaders/text.raster.hlsl"






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
#line 7 "shaders/text.raster.hlsl"
            
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
#line 8 "shaders/text.raster.hlsl"

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
#line 9 "shaders/text.raster.hlsl"

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
#line 10 "shaders/text.raster.hlsl"

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
#line 11 "shaders/text.raster.hlsl"

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
#line 12 "shaders/text.raster.hlsl"

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
#line 13 "shaders/text.raster.hlsl"

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
#line 14 "shaders/text.raster.hlsl"

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
#line 15 "shaders/text.raster.hlsl"

#line 16 "shaders/text.raster.hlsl"

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
#line 17 "shaders/text.raster.hlsl"

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
#line 18 "shaders/text.raster.hlsl"

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
#line 19 "shaders/text.raster.hlsl"



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
#line 23 "shaders/text.raster.hlsl"

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
#line 24 "shaders/text.raster.hlsl"


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
