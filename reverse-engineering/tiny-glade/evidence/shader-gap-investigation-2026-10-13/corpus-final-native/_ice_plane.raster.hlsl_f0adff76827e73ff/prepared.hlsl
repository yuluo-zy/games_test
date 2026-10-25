#define SHADER_VARIANT_DEPTH_ONLY 1
#define TGV_f0adff76827e73ff 1
#line 1 "shaders/ice_plane.raster.hlsl"


#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
















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
#line 20 "shaders/ice_plane.raster.hlsl"

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
#line 21 "shaders/ice_plane.raster.hlsl"

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
#line 22 "shaders/ice_plane.raster.hlsl"

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
#line 23 "shaders/ice_plane.raster.hlsl"

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
#line 24 "shaders/ice_plane.raster.hlsl"

#line 1 "shaders/inc/consts.hlsl"






#ifndef TINYGLADE_INC_CONSTS_HLSL
#define TINYGLADE_INC_CONSTS_HLSL


static const float RENDER_SCALE = 1.0f;

#endif 
#line 25 "shaders/ice_plane.raster.hlsl"

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
#line 26 "shaders/ice_plane.raster.hlsl"

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
#line 27 "shaders/ice_plane.raster.hlsl"

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
#line 28 "shaders/ice_plane.raster.hlsl"

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
#line 29 "shaders/ice_plane.raster.hlsl"

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
#line 30 "shaders/ice_plane.raster.hlsl"

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
#line 31 "shaders/ice_plane.raster.hlsl"

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
#line 32 "shaders/ice_plane.raster.hlsl"

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
#line 33 "shaders/ice_plane.raster.hlsl"

#line 1 "shaders/inc/halfres.hlsl"




#ifndef TINYGLADE_INC_HALFRES_HLSL
#define TINYGLADE_INC_HALFRES_HLSL


int2 to_halfres_pixel(int2 full_pixel)
{
    return full_pixel >> 1;
}


int2 from_halfres_pixel(int2 half_pixel)
{
    return half_pixel << 1;
}


uint2 halfres_dims(uint2 full_dims)
{
    return (full_dims + 1u) >> 1;
}

#endif 
#line 34 "shaders/ice_plane.raster.hlsl"

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
#line 35 "shaders/ice_plane.raster.hlsl"

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
#line 36 "shaders/ice_plane.raster.hlsl"

#line 1 "shaders/inc/shadow.hlsl"


#ifndef TINYGLADE_INC_SHADOW_HLSL
#define TINYGLADE_INC_SHADOW_HLSL

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
#line 6 "shaders/inc/shadow.hlsl"




float sample_shadow(Texture2DArray<float4> shadow_map,
                    SamplerState nearest_sampler,
                    SamplerComparisonState cmp_sampler,
                    float3 world_pos,
                    float3 world_normal,
                    float3 dir_to_sun,
                    float spiral_seed,
                    out float penumbra_size)
{
    ShadowCascadeSelection sel = select_shadow_cascade(world_pos);
    penumbra_size = 0.0;
    if (sel.slice < 0) {
        return 1.0;
    }

    float scale = max(sel.sample_scale, 1e-6);
    float bias = max((0.3 / scale) * (1.0 - abs(dot(world_normal, -dir_to_sun))), 0.025);
    float texel_uv = 0.00083333335f * sel.texel_scale;
    float3 base = float3(sel.shadow_uv, float(sel.slice));

    
    float blocker_depth = 0.0;
    float lit_depth = 0.0;
    int blocker_count = 0;
    for (int i = 0; i < 5; ++i) {
        float angle = (float(i) + spiral_seed) * 2.3999631404876708984375f;
        float2 off = (float2(cos(angle), sin(angle)) * (float(i) + frac(52.98291778564453125f * frac((0.067110560834407806396484375f * spiral_seed) + (0.005837149918079376220703125f * 1.0f))))) * 0.20000000298023223876953125f * texel_uv;
        float3 sample_pos = float3(base.xy + off, base.z - bias * 0.0009765625f);
        float d = shadow_map.SampleLevel(nearest_sampler, sample_pos, 0.0).x;
        if (sample_pos.z > d) {
            blocker_depth += d;
            blocker_count++;
        } else {
            lit_depth += d;
        }
    }
    penumbra_size = 0.0;
    if (blocker_count == 0) {
        return 1.0;
    }
    float avg_blocker = blocker_depth / float(blocker_count);
    float penumbra_uv = max((base.z - avg_blocker) * (17.0666675567626953125f * texel_uv) * pow(10.0, 0.0) / max(avg_blocker, 1e-6), 0.0);
    float filter_scale = (penumbra_uv / pow(pow(penumbra_uv / (texel_uv - 1.0), 5.0) + 1.0, 0.20000000298023223876953125f)) + 1.0;

    
    float4 light_dir_proj = mul(view_constants[2 + sel.slice].world_to_sample, float4(dir_to_sun, 0.0));
    float2 slope_bias = (light_dir_proj.xy * float(int(sign(light_dir_proj.z)))) / max(0.1500000059604644775390625f, abs(light_dir_proj.z)) * 0.0009765625f * 2.0;

    
    float lit = 0.0;
    for (int j = 0; j < 10; ++j) {
        float angle = (float(j) + spiral_seed) * 2.3999631404876708984375f;
        float2 off = (float2(cos(angle), sin(angle)) * (float(j) + frac(52.98291778564453125f * 0.5f))) * 0.100000001490116119384765625f * filter_scale;
        float3 sample_pos = float3(base.xy + off * texel_uv, base.z);
        lit += 1.0 - shadow_map.SampleCmpLevelZero(cmp_sampler, sample_pos, sample_pos.z + min(0.0, dot(slope_bias, off)));
    }
    penumbra_size = filter_scale;
    return lit * 0.1;
}

#endif 
#line 37 "shaders/ice_plane.raster.hlsl"

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
#line 38 "shaders/ice_plane.raster.hlsl"

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
#line 39 "shaders/ice_plane.raster.hlsl"

#line 1 "shaders/inc/stochastic_tiling.hlsl"





#ifndef TINYGLADE_INC_STOCHASTIC_TILING_HLSL
#define TINYGLADE_INC_STOCHASTIC_TILING_HLSL

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
#line 9 "shaders/inc/stochastic_tiling.hlsl"

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
#line 10 "shaders/inc/stochastic_tiling.hlsl"




float3 stochastic_tile_sample(Texture2D<float4> tex, SamplerState samp, float2 uv, float2 tile_scale, uint seed, uint tiles)
{
    float2 tile_id = floor(uv * tile_scale);
    float2 f = frac(uv * tile_scale);
    uint rng = hash(uint(tile_id.x) * 73856093u ^ uint(tile_id.y) * 19349663u ^ seed);

    float3 result = float3(0.0f, 0.0f, 0.0f);
    float total = 0.0f;
    for (uint t = 0u; t < tiles; t++)
    {
        uint h = hash(rng + t);
        float2 offset = float2(rand01(h), rand01(h >> 8u));
        float2 sample_uv = (tile_id + f) / tile_scale + (offset - 0.5f) * (1.0f / tile_scale);
        result += tex.SampleLevel(samp, sample_uv, 0.0f).rgb;
        total += 1.0f;
    }
    return result / total;
}

#endif 
#line 40 "shaders/ice_plane.raster.hlsl"

#line 1 "shaders/inc/history_valid.hlsl"












#ifndef TINYGLADE_INC_HISTORY_VALID_HLSL
#define TINYGLADE_INC_HISTORY_VALID_HLSL

struct FrexpScalarType {
    float mantissa;
    float exponent;
};

FrexpScalarType incb_frexp_scalar(float v)
{
    FrexpScalarType r;
    uint bits = asuint(v);
    int biased = int((bits >> 23u) & 0xFFu);
    int e = biased - 127;
    float m = asfloat((bits & 0x807FFFFFu) | 0x3F800000u);
    bool big = (m >= 1.0f) | (m <= -1.0f);
    m = lerp(m, m * 0.5f, big);
    e += big ? 1 : 0;
    bool special = (biased == 0) | (biased == 0xFF);
    r.exponent = special ? 0.0f : float(e);
    r.mantissa = special ? v : m;
    return r;
}



bool history_depth_valid(float current_depth, float history_depth, float epsilon)
{
    FrexpScalarType c = incb_frexp_scalar(current_depth);
    float scale = exp2(c.exponent);
    return abs(current_depth - history_depth) * scale < epsilon;
}






bool is_history_valid(bool reprojected_available, uint history_valid_flag, uint frame_index)
{
    return reprojected_available && (history_valid_flag != 0u) && (frame_index != 0u);
}

#endif 
#line 41 "shaders/ice_plane.raster.hlsl"



#line 1 "shaders/inc/blue_noise.hlsl"

















float4 blue_noise(Texture2D<float4> blue_noise_texture, uint2 pixel_coords, uint frame_index)
{
    uint2 offset = uint2(frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f)
        * float(frame_index)) + 0.5f.xx) * 256.0f);
    uint2 p = (pixel_coords + uint2(13u, 13u) + offset) % uint2(256u, 256u);
    float4 texel = blue_noise_texture.Load(int3(p, 0));
    return texel * 255.0f * (1.0f / 256.0f) + (1.0f / 512.0f);
}
#line 44 "shaders/ice_plane.raster.hlsl"

#line 1 "shaders/water/ice_shared.inc.hlsl"



















#ifndef TINYGLADE_WATER_ICE_SHARED_HLSL
#define TINYGLADE_WATER_ICE_SHARED_HLSL

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
#line 23 "shaders/water/ice_shared.inc.hlsl"



uint pack_half2x16(float2 value)
{
    uint2 packed = f32tof16(value);
    return packed.x | (packed.y << 16u);
}


float2 unpack_half2x16(uint value)
{
    return f16tof32(uint2(value & 0xFFFFu, value >> 16u));
}





#define frexp ice_frexp

struct FrexpStructType
{
    float3 mantissa;
    int3 exponent;
};

float3 ice_frexp(float3 x, out int3 exponent)
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

#endif 
#line 45 "shaders/ice_plane.raster.hlsl"

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
#line 46 "shaders/ice_plane.raster.hlsl"


cbuffer $Globals : register(b13, space0)
{
    uint current_view_index;
    uint history_valid;
    uint2 blue_noise_texture_dims;
    uint2 framebuffer_dims;
    uint2 water_mask_dims;
    uint2 half_depth_tex_dims;
    uint2 refraction_tex_dims;
    uint2 reprojection_tex_dims;
    uint2 shadows_history_tex_dims;
    uint2 surface_roughness_tex_dims;
    float4x4 model_to_world;
};

RWByteAddressBuffer __alias_ice_refl_count_buffer : register(u23, space0);
RWByteAddressBuffer __alias_ice_refl_px_buffer : register(u24, space0);
Texture2DArray<float4> shadow_map : register(t15, space0);
Texture2D<float4> half_depth_tex : register(t18, space0);
Texture2D<float4> refraction_tex : register(t19, space0);
Texture2D<float4> shadows_history_tex : register(t21, space0);
Texture2D<float4> surface_roughness_tex : register(t22, space0);

struct VsIn
{
    [[vk::location(0)]] float3 Vertex_Position: Vertex_Position;
};

struct VsOut
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;
    float4 world = mul(model_to_world, float4(vin.Vertex_Position, 1.0f));
    float2 scaled = world.xz * 130.0f.xx;
    float4 p = float4(scaled.x, world.y, scaled.y, world.w);
    p.y = 0.0f;
    o.position = mul(view_constants[current_view_index].world_to_sample, p);
    o.C0 = float3(0.0f, 1.0f, 0.0f);
    o.C1 = p.xyz;
    return o;
}


float3 swap_yz(float3 v)
{
    return mul(v, float3x3(float3(1.0f, -0.0f, 0.0f),
                          float3(-0.0f, 0.0f, 1.0f),
                          float3(-0.0f, -1.0f, 0.0f)));
}


float cell_hash01(int2 cell)
{
    uint y = uint(cell.y);
    y += (y << 10u);
    y ^= (y >> 6u);
    y += (y << 3u);
    y ^= (y >> 11u);
    uint h = ((uint(cell.x) * 1664525u) + (y + (y << 15u))) + 1013904223u;
    h *= 1664525u;
    h ^= (h >> 11u);
    h ^= (h << 7u) & 2636928640u;
    h ^= (h << 15u) & 4022730752u;
    return asfloat(((h ^ (h >> 18u)) & 0x7FFFFFu) | 0x3F800000u) - 1.0f;
}



bool depth_march(float3 start, float3 end, float steps_max, float2 frame_hash,
                 out float hit_t)
{
    float3 seg = end - start;
    float t_count = (end.z > 9.5367431640625e-07f) ? steps_max : max(1.0f, steps_max - 1.0f);
    float inv = 1.0f / t_count;
    float depth_eps = 0.100000001490116119384765625f * view_constants[0].clip_to_view[3].z;

    float prev_e = 0.0f;
    float prev_ray = 0.0f;
    bool prev_under = false;
    if (steps_max > 0.0f)
    {
        float t_first = lerp(0.0f, 1.0f, pow(frame_hash.x * inv, 1.0f));
        float3 sp = start + (seg * t_first);
        float inv_z = 1.0f / sp.z;
        float d = half_depth_tex.SampleLevel(sampler_nnc, (sp.xy * float2(0.5f, -0.5f)) + 0.5f, 0.0f).x;
        float diff = (1.00000202655792236328125f / d) - inv_z;
        bool under = (diff < 0.0f);
        prev_e = under ? 0.0f : diff;
        prev_ray = under ? t_first : 0.0f;
        prev_under = under;
        for (uint i = 1u; i < uint(t_count); ++i)
        {
            float tt = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(i) + frame_hash.x) * inv, 1.0f)));
            float3 sp2 = start + (seg * tt);
            float inv_z2 = 1.0f / sp2.z;
            float d2 = half_depth_tex.SampleLevel(sampler_nnc, (sp2.xy * float2(0.5f, -0.5f)) + 0.5f, 0.0f).x;
            float diff2 = (1.00000202655792236328125f / d2) - inv_z2;
            bool under2 = (diff2 < 0.0f);
            if (under2)
            {
                prev_e = diff2;
                prev_ray = tt;
                prev_under = true;
                break;
            }
            prev_e = diff2;
            prev_under = under2;
        }
    }

    if (!prev_under)
    {
        hit_t = prev_e;
        return false;
    }

    float lo = prev_e;
    float hi = prev_ray;
    float d_lo = 0.0f;
    float d_hi = prev_e;
    float ray_hi = prev_ray;
    float t_hi = prev_ray;
    for (uint j = 0u; j < 1u; ++j)
    {
        float mid = (lo + hi) * 0.5f;
        float3 sm = start + (seg * mid);
        float inv_zm = 1.0f / sm.z;
        float dm = half_depth_tex.SampleLevel(sampler_nnc, (sm.xy * float2(0.5f, -0.5f)) + 0.5f, 0.0f).x;
        float diffm = (1.00000202655792236328125f / dm) - inv_zm;
        bool under_m = (diffm < 0.0f);
        lo = under_m ? lo : mid;
        d_hi = under_m ? diffm : d_hi;
        ray_hi = under_m ? ray_hi : (inv_zm - (1.0f / dm));
        t_hi = under_m ? mid : t_hi;
    }
    float interp = lerp(lo, t_hi, d_lo / (d_lo + (-d_hi)));
    hit_t = interp;
    return true;
}

struct PsIn
{
    float3 C1 : TEXCOORD1;
};

struct PsOut
{
    float3 C0 : SV_Target0;
    float C1 : SV_Target1;
    uint C2 : SV_Target2;
    uint C3 : SV_Target3;
    float C4 : SV_Target4;
};

[earlydepthstencil]
PsOut ps_main(PsIn vin, float4 position : SV_Position)
{
    PsOut o;
    bool helper = IsHelperLane();
    uint2 px = uint2(position.xy);
    uint2 half_px = px / uint2(2u, 2u);
    float2 half_dims = float2(half_depth_tex_dims);
    float2 ndc = (((float2(int2(half_px)) + 0.5f) / half_dims) - 0.5f) * float2(2.0f, -2.0f);

    float4 pos_vs = float4(ndc, position.z, 1.0f);
    float3 view_ray = -normalize(mul(view_constants[0].view_to_world,
        mul(view_constants[0].sample_to_view, float4(ndc, 0.0f, 1.0f))).xyz);
    float3 up_world = mul(view_constants[2].view_to_world, float4(0.0f, 0.0f, 1.0f, 0.0f)).xyz;

    
    float2 roughness_dims = float2(surface_roughness_tex_dims);
    float2 rough_p = vin.C1.xz * 0.100000001490116119384765625f;
    float2 rough_hex = mul(rough_p * 3.4639999866485595703125f,
        float2x2(float2(1.0f, 0.0f), float2(-0.57735025882720947265625f, 1.1547005176544189453125f)));
    float2 rough_cell = floor(rough_hex);
    float2 rough_frac = frac(rough_hex);
    bool tri_up = ((1.0f - rough_frac.x) - rough_frac.y) > 0.0f;
    float3 c0 = tri_up ? float3(rough_cell, (1.0f - rough_frac.x) - rough_frac.y)
                       : float3(rough_cell + float2(1.0f, 1.0f), rough_frac.x - (1.0f - rough_frac.y) + rough_frac.y);
    float3 c1 = tri_up ? float3(rough_cell + float2(0.0f, 1.0f), rough_frac.y)
                       : float3(rough_cell + float2(1.0f, 0.0f), 1.0f - rough_frac.y);
    float3 c2 = tri_up ? float3(rough_cell + float2(1.0f, 0.0f), rough_frac.x)
                       : float3(rough_cell + float2(0.0f, 1.0f), 1.0f - rough_frac.x);

    float4 grad = float4(ddx(rough_p), -ddy(rough_p));
    float2 grad_x = grad.xy;
    float2 grad_y = grad.zw;
    float4 rough_scale4 = surface_roughness_tex.SampleLevel(sampler_llr, 0.0f,
        (log(min(roughness_dims.x, roughness_dims.y)) * 1.44269502162933349609375f) + 1.0f);

    uint2 cell0 = uint2(c0.xy);
    uint2 h0 = ((cell0 >> uint2(8u, 8u)) ^ cell0.yx) * uint2(1103515245u, 1103515245u);
    uint2 cell1 = uint2(c1.xy);
    uint2 h1 = ((cell1 >> uint2(8u, 8u)) ^ cell1.yx) * uint2(1103515245u, 1103515245u);
    uint2 cell2 = uint2(c2.xy);
    uint2 h2 = ((cell2 >> uint2(8u, 8u)) ^ cell2.yx) * uint2(1103515245u, 1103515245u);
    float2 j0 = float2((h0 >> uint2(8u, 8u)) ^ h0.yx) * uint2(1103515245u, 1103515245u) * 2.3283064365386962890625e-10f;
    float2 j1 = float2((h1 >> uint2(8u, 8u)) ^ h1.yx) * uint2(1103515245u, 1103515245u) * 2.3283064365386962890625e-10f;
    float2 j2 = float2((h2 >> uint2(8u, 8u)) ^ h2.yx) * uint2(1103515245u, 1103515245u) * 2.3283064365386962890625e-10f;

    float rough_sum = (surface_roughness_tex.SampleGrad(sampler_llr, rough_p + j0, grad_x, grad_y) * c0.z).x
        + (surface_roughness_tex.SampleGrad(sampler_llr, rough_p + j1, grad_x, grad_y) * c1.z).x
        + (surface_roughness_tex.SampleGrad(sampler_llr, rough_p + j2, grad_x, grad_y) * c2.z).x;
    float roughness = 0.0199999995529651641845703125f + (0.300000011920928955078125f
        * (1.0f - ((rough_sum - rough_scale4.x) / length(float3(c0.z, c1.z, c2.z)) + rough_scale4.x)));

    float3 view_dir = swap_yz(view_ray);
    uint quad_seed = ((px.x & 1u) | ((px.y & 1u) * 2u));
    uint hilbert;
    {
        uint acc = 0u;
        uint xi = px.x;
        uint yi = px.y;
        for (uint s = 32u; s > 0u; s /= 2u)
        {
            uint bx = uint((xi & s) > 0u);
            uint by = uint((yi & s) > 0u);
            acc += (s * s) * ((3u * bx) ^ by);
            if (by == 0u && bx == 1u)
            {
                xi = 63u - xi;
                yi = 63u - yi;
            }
        }
        hilbert = acc;
    }
    uint frame_mod = frame_index % 64u;
    float2 bn = frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(hilbert + (288u * frame_mod))) + 0.5f);

    float3 rough_dir = normalize(float3(view_dir.xy * 0.07500000298023223876953125f, view_dir.z));
    float a = 6.283185482025146484375f * bn.x;
    float c = 1.0f - (bn.y * 0.300000011920928955078125f);
    float s = sqrt(clamp(1.0f - (c * c), 0.0f, 1.0f));
    float3 refl_dir = float3(s * cos(a), s * sin(a), c) + rough_dir;

    float frame_rand = 5.588237762451171875f * float(frame_mod);
    float hash = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(px.x) + frame_rand))
        + (0.005837149918079376220703125f * (float(px.y) + frame_rand))));

    float4 wp4 = mul(view_constants[0].view_to_world,
        mul(view_constants[0].sample_to_view, float4(ndc, half_depth_tex.Load(int3(half_px, 0u)).x, 1.0f)));
    float3 world_pos = wp4.xyz / wp4.w;
    float3 start_world = vin.C1;
    float4 ref_end4 = mul(view_constants[0].view_to_sample,
        mul(view_constants[0].world_to_view, float4(vin.C1
            + (swap_yz(normalize(float3(refl_dir.xy * 0.07500000298023223876953125f, refl_dir.z)))
                * float3(-1.0f, 1.0f, -1.0f)) * min(4.0f, 4.0f * length(vin.C1 - world_pos)), 1.0f)));
    float3 ref_dir = ref_end4.xyz / ref_end4.w;
    float4 march_vs = pos_vs + (float4(ref_dir - pos_vs.xyz, 0.0f) * float(int(sign(ref_dir.z))));

    
    float3 m0 = march_vs.xyz;
    float3 m1 = ref_dir;
    bool3 neg3 = (m1 - m0) < 0.0f;
    float3 t0 = (float3(neg3.x ? 1.0f : -1.0f, neg3.y ? 1.0f : -1.0f, neg3.z ? 1.0f : -1.0f) - m0) / (m1 - m0);
    float3 a0 = m0 + ((m1 - m0) * max(0.0f, max(t0.x, t0.y)));
    float3 r3 = (m1 - m0) - (a0 - m0);
    bool3 p3 = r3 >= 0.0f;
    float3 t1 = (float3(p3.x ? 1.0f : -1.0f, p3.y ? 1.0f : -1.0f, p3.z ? 1.0f : -1.0f) - a0) / r3;
    float3 a1 = a0 + (r3 * min(1.0f, min(min(t1.x, t1.y), t1.z)));

    float hit_t;
    bool depth_hit = depth_march(a0, a1, 3.0f, float2(hash, bn.y), hit_t);
    float3 march_hit = lerp(a0, a1, hit_t);
    float2 hit_uv = (march_hit.xy * float2(0.5f, -0.5f)) + 0.5f;
    float4 wp_hit = mul(view_constants[0].view_to_world,
        mul(view_constants[0].sample_to_view, float4(march_hit.x, march_hit.y, half_depth_tex.SampleLevel(sampler_nnc, hit_uv, 0.0f).x, 1.0f)));
    float3 hit_world = wp_hit.xyz / wp_hit.w;
    float3 hit_offset = hit_world - vin.C1;
    float3 hit_normal = -up_world;

    
    float2 shadow = 0.0f;
    float shadow_blur = 0.0f;
    float4 sc = mul(view_constants[2].world_to_sample, float4(vin.C1, 1.0f));
    if (sc.z <= 1.0f)
    {
        float2 sc_uv = sc.xy;
        bool out0 = any(abs(sc_uv) > 0.9833333492279052734375f);
        float2 uv1 = out0 ? sc_uv * 0.5f : sc_uv;
        float texel_scale = out0 ? 5.0f : 10.0f;
        float sample_scale = out0 ? 0.5f : 1.0f;
        bool out1 = any(abs(uv1) > (1.0f - (0.000833333353511989116668701171875f * (2.0f * texel_scale))));
        if (out1)
        {
            texel_scale = texel_scale * 0.2857142984867095947265625f;
            uv1 = uv1 * 0.2857142984867095947265625f;
            sample_scale = sample_scale * 0.2857142984867095947265625f;
        }
        uint slice = uint(out1 ? 2u : uint(out0));
        uint slice_idx = 2u + slice;
        float ndl = up_world.y;
        float4 proj = float4(uv1, sc.z, sc.w) + mul(view_constants[slice_idx].world_to_sample,
            float4((float3(0.0f, 1.0f, 0.0f) * (0.066666670143604278564453125f * clamp(1.0f - ndl, 0.0f, 1.0f))) / sample_scale, 0.0f));
        float2 shadow_uv = (proj.xy * float2(0.5f, -0.5f)) + 0.5f;
        float3 shadow_xyz = float3(shadow_uv, proj.z);
        float bias = max((0.300000011920928955078125f / sample_scale) * (1.0f - abs(ndl)),
                         0.02500000037252902984619140625f);
        float spiral_seed = float((uint((px.x * 11) + (px.y * 7)) + (frame_index * 5u)) % 64u) * 1.61803400516510009765625f;
        float hash2 = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(px.x) + frame_rand))
            + (0.005837149918079376220703125f * (float(px.y) + frame_rand))));
        float4 up4 = mul(view_constants[slice_idx].world_to_sample, float4(0.0f, 1.0f, 0.0f, 0.0f));
        float blocked = 0.0f;
        int hits = 0;
        for (int i = 0; i < 5; ++i)
        {
            float ang = (float(i) + spiral_seed) * 2.3999631404876708984375f;
            float3 sp = shadow_xyz + float3((float2(cos(ang), sin(ang)) * (float(i) + hash2)) * 0.20000000298023223876953125f * texel_scale * 0.000833333353511989116668701171875f, bias * (-0.0009765625f));
            float d = shadow_map.SampleLevel(sampler_shadow_nearest, float3(sp.xy, float(slice)), 0.0f).x;
            if (sp.z > d)
            {
                blocked += d;
                ++hits;
            }
        }
        if (hits != 0)
        {
            float avg = blocked / float(hits);
            float pen = max((proj.z - avg) * ((17.0666675567626953125f * texel_scale) * pow(10.0f, smoothstep(0.0f, 0.25f, fog_tint_thickness.w))) / avg, 0.0f);
            shadow_blur = (pen / pow(pow(pen / (texel_scale - 1.0f), 5.0f) + 1.0f, 0.20000000298023223876953125f)) + 1.0f;
            float2 up_uv = ((up4.xy * float(int(sign(up4.z))) / max(0.1500000059604644775390625f, abs(up4.z))) * 0.0009765625f) * 2.0f;
            float2 acc = 0.0f;
            for (int j = 0; j < 10; ++j)
            {
                float ang2 = (float(j) + spiral_seed) * 2.3999631404876708984375f;
                float2 sp2 = (float2(cos(ang2), sin(ang2)) * (float(j) + hash2)) * 0.100000001490116119384765625f * shadow_blur;
                float3 scp = shadow_xyz + float3(sp2 * 0.000833333353511989116668701171875f, bias * (-0.0009765625f));
                float lit = 1.0f - shadow_map.SampleCmpLevelZero(sampler_shadow_linear,
                    float3(scp.xy, float(slice)), scp.z + min(0.0f, dot(up_uv, sp2)));
                acc += float2(lit, lit * lit);
            }
            shadow = acc * 0.100000001490116119384765625f;
        }
    }

    float shadow_m = clamp((shadow.x - 0.100000001490116119384765625f) * 1.4285714626312255859375f, 0.0f, 1.0f);
    float2 shadow_std = shadow_m + (float2(-1.0f, 1.0f) * sqrt(max(0.0f, shadow.y - (shadow.x * shadow.x))) * (3.0f * max(0.0f, 1.0f - (shadow_blur * 0.20000000298023223876953125f))));

    
    float2 fb_uv = (float2(px) + 0.5f) / float2(framebuffer_dims);
    float4 prev4 = mul(view_constants[0].clip_to_prev_clip,
        mul(view_constants[0].view_to_clip,
            mul(view_constants[0].clip_to_view, float4((fb_uv - 0.5f) * float2(2.0f, -2.0f), position.z, 1.0f))));
    float2 prev_uv = ((prev4.xy / prev4.w) * float2(0.5f, -0.5f)) + 0.5f;
    float2 prev_clamped = clamp(prev_uv, 0.0f, 1.0f);
    float4 motion = float4(prev_uv - ((float4((fb_uv - 0.5f) * float2(2.0f, -2.0f), 0.0f, 0.0f).xy * float2(0.5f, -0.5f)) + 0.5f),
        float(all(prev_clamped == prev_uv)), 1.0f) * float4(1.0f, 1.0f, clamp(1.0f - length(view_constants[0].clip_to_prev_clip[3].xyz), 0.0f, 1.0f), 1.0f);
    float shadow_hist;
    if (motion.z > 0.0f && history_valid != 0u && frame_index != 0u)
    {
        shadow_hist = shadows_history_tex.SampleLevel(sampler_lnc, fb_uv + motion.xy, 0.0f).x;
    }
    else
    {
        shadow_hist = shadow_m;
    }
    float shadow_temporal = lerp(clamp(shadow_hist, shadow_std.x, shadow_std.y), shadow_m, lerp(1.0f, 0.125f, motion.z));
    float inv_shadow = 1.0f - shadow_temporal;

    
    float4 rp = mul(view_constants[0].view_to_sample,
        mul(view_constants[0].world_to_view, float4(vin.C1 + (up_world * 0.5f), 1.0f)));
    float3 rp_w = rp.xyz / rp.w;
    float4 rv = pos_vs + (float4(rp_w - pos_vs.xyz, 0.0f) * float(int(sign(rp_w.z))));
    float3 r0 = rv.xyz;
    float3 r1 = rp_w;
    bool3 rneg = (r1 - r0) < 0.0f;
    float3 rt0 = (float3(rneg.x ? 1.0f : -1.0f, rneg.y ? 1.0f : -1.0f, rneg.z ? 1.0f : -1.0f) - r0) / (r1 - r0);
    float3 ra0 = r0 + ((r1 - r0) * max(0.0f, max(rt0.x, rt0.y)));
    float3 rr3 = (r1 - r0) - (ra0 - r0);
    bool3 rp3 = rr3 >= 0.0f;
    float3 rt1 = (float3(rp3.x ? 1.0f : -1.0f, rp3.y ? 1.0f : -1.0f, rp3.z ? 1.0f : -1.0f) - ra0) / rr3;
    float3 ra1 = ra0 + (rr3 * min(1.0f, min(min(rt1.x, rt1.y), rt1.z)));

    float refr_hit_t;
    bool refr_hit = depth_march(ra0, ra1, 3.0f, float2(hash, bn.y), refr_hit_t);
    float refr_factor;
    if (refr_hit)
    {
        float3 rhp = lerp(ra0, ra1, refr_hit_t);
        float2 rhuv = (rhp.xy * float2(0.5f, -0.5f)) + 0.5f;
        float rl = 1.0f / rhp.z;
        float rd_lo = 1.0f / half_depth_tex.SampleLevel(sampler_llc, rhuv, 0.0f).x;
        float rd_hi = 1.0f / half_depth_tex.SampleLevel(sampler_nnc, rhuv, 0.0f).x;
        float ddiff = (max(rd_lo, rd_hi) * 1.00000202655792236328125f) - rl;
        float ddiff2 = rl - min(rd_lo, rd_hi);
        refr_factor = min(inv_shadow, smoothstep(0.5f, 0.0f, (1.0f - (ddiff2 / (0.5f * view_constants[0].clip_to_view[3].z)))
            * (1.0f - ddiff)));
    }
    else
    {
        refr_factor = inv_shadow;
    }

    
    uint quad_flag = helper ? 0u : (1u << quad_seed);
    uint quad_sum = quad_flag + QuadReadAcrossX(quad_flag);
    if ((px.x & 1u) + ((px.y & 1u) * 2u) == firstbithigh(quad_sum + QuadReadAcrossY(quad_sum)))
    {
        uint slot;
        __alias_ice_refl_count_buffer.InterlockedAdd(12, 1u, slot);
        __alias_ice_refl_px_buffer.Store<uint>(slot * 4u + 0u, half_px.x + (half_px.y * 65536u));
        if ((slot & 63u) == 0u)
        {
            uint group;
            __alias_ice_refl_count_buffer.InterlockedAdd(0, 1u, group);
        }
    }

    
    float3 spec = 0.0f;
    float up_w = up_world.y;
    bool spec_ok = up_w > 0.0f && view_ray.y > 0.0f;
    if (spec_ok)
    {
        float r2 = roughness * roughness;
        float3 h2 = normalize(view_ray + up_world);
        float ndh = max(0.0f, 1.0f - dot(h2, up_world));
        float g2 = ndh * ndh;
        float4 denom = float4(1.0f, 1.0f, 1.0f, (h2.y * h2.y * (r2 - 1.0f)) + 1.0f);
        float3 fres = lerp(0.02999999932944774627685546875f, 1.0f, (g2 * g2) * ndh);
        float gterm = (2.0f * up_w * view_ray.y) / (up_w * sqrt(((((-view_ray.y) * r2) + view_ray.y) * view_ray.y) + r2) + (view_ray.y * sqrt(((((-up_w) * r2) + up_w) * up_w) + r2)));
        float3 brdf = (fres * gterm) * (r2 / ((3.1415927410125732421875f * denom.w) * denom.w)) / ((4.0f * view_ray.y) * up_w);
        spec = brdf;
    }

    float3 hit_off = hit_offset * float3(1.0f, -1.0f, 1.0f);
    float3 hit_off_swap = swap_yz(hit_off);
    float hit_len = length(hit_off_swap);
    float3 hit_n = hit_off_swap * (1.0f / max(1e-9f, hit_len));
    float3 refr_col = clamp(exp(float3(-0.5950000286102294921875f, -0.374000012874603271484375f, -0.3060000240802764892578125f)
        * ((abs(vin.C1.y - hit_world.y) + length(hit_offset)) + 0.699999988079071044921875f))
        * (0.959999978542327880859375f - (0.959999978542327880859375f * ((view_ray.y * view_ray.y) * (1.0f - view_ray.y))))
        * refraction_tex.SampleLevel(sampler_lnc, hit_uv, 0.0f).xyz, 0.0f, 65408.0f);

    float scale = asfloat((asuint(max(max(1.52587890625e-05f, refr_col.x), max(refr_col.y, refr_col.z))) + 125845504u) & 2139095040u);
    uint3 mantissa = asuint(refr_col + scale);
    uint radiance = ((((asuint(scale) << 4u) + 268435456u) | (mantissa.z << 18u)) | (mantissa.y << 9u)) | (mantissa.x & 511u);

    o.C0 = lerp(sun_color.xyz * spec * (refr_factor * clamp(up_w, 0.0f, 1.0f)),
        (sun_color.xyz * (0.699999988079071044921875f * refr_factor) * 0.3183098733425140380859375f) + sky_high_color.xyz,
        0.00999999977648258209228515625f);
    o.C1 = sqrt(roughness);
    o.C2 = ((uint((clamp((hit_n.x * 0.5f) + 0.5f, 0.0f, 1.0f) * 255.0f) + 0.5f) << 24u)
        | (uint((clamp((hit_n.y * 0.5f) + 0.5f, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u))
        | pack_half2x16(float2(hit_len, 0.0f));
    o.C3 = radiance;
    o.C4 = shadow_temporal + 0.00196078442968428134918212890625f;
    return o;
}

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

static float _195;
static float _228;

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
    uint current_view_index : packoffset(c0);
    uint history_valid : packoffset(c0.y);
    uint2 blue_noise_texture_dims : packoffset(c0.z);
    uint2 framebuffer_dims : packoffset(c1);
    uint2 water_mask_dims : packoffset(c1.z);
    uint2 half_depth_tex_dims : packoffset(c2);
    uint2 refraction_tex_dims : packoffset(c2.z);
    uint2 reprojection_tex_dims : packoffset(c3);
    uint2 shadows_history_tex_dims : packoffset(c3.z);
    uint2 surface_roughness_tex_dims : packoffset(c4);
    column_major float4x4 model_to_world : packoffset(c5);
};

RWByteAddressBuffer __alias_ice_refl_count_buffer : register(u23, space0);
RWByteAddressBuffer __alias_ice_refl_px_buffer : register(u24, space0);
SamplerState sampler_lnc : register(s0, space0);
SamplerState sampler_llr : register(s1, space0);
SamplerState sampler_nnc : register(s2, space0);
SamplerState sampler_llc : register(s3, space0);
SamplerState sampler_shadow_nearest : register(s11, space0);
SamplerComparisonState sampler_shadow_linear : register(s12, space0);
Texture2DArray<float4> shadow_map : register(t15, space0);
Texture2D<float4> half_depth_tex : register(t18, space0);
Texture2D<float4> refraction_tex : register(t19, space0);
Texture2D<float4> shadows_history_tex : register(t21, space0);
Texture2D<float4> surface_roughness_tex : register(t22, space0);

static float4 gl_FragCoord;
static float3 in_var_C1;
static float3 out_var_SV_TARGET0;
static float out_var_SV_TARGET1;
static uint out_var_SV_TARGET2;
static uint out_var_SV_TARGET3;
static float out_var_SV_TARGET4;

struct SPIRV_Cross_Input
{
    [[vk::location(1)]] float3 in_var_C1 : TEXCOORD1;
    float4 gl_FragCoord : SV_Position;
};

struct SPIRV_Cross_Output
{
    float3 out_var_SV_TARGET0 : SV_Target0;
    float out_var_SV_TARGET1 : SV_Target1;
    uint out_var_SV_TARGET2 : SV_Target2;
    uint out_var_SV_TARGET3 : SV_Target3;
    float out_var_SV_TARGET4 : SV_Target4;
};

uint spvPackHalf2x16(float2 value)
{
    uint2 Packed = f32tof16(value);
    return Packed.x | (Packed.y << 16);
}

float2 spvUnpackHalf2x16(uint value)
{
    return f16tof32(uint2(value & 0xffff, value >> 16));
}

void frag_main()
{
    bool _244 = IsHelperLane();
    uint2 _248 = uint2(gl_FragCoord.xy);
    uint2 _249 = _248 / uint2(2u, 2u);
    float2 _253 = float2(half_depth_tex_dims);
    float2 _263 = (((float2(int2(_249)) + 0.5f.xx) / _253) - 0.5f.xx) * float2(2.0f, -2.0f);
    float _264 = _263.x;
    float4 _269 = float4(_264, _263.y, gl_FragCoord.z, 1.0f);
    float3 _272 = -normalize(mul(frame_constants_view_constants[0u].view_to_world, mul(frame_constants_view_constants[0u].sample_to_view, float4(_264, _263.y, 0.0f, 1.0f))).xyz);
    float4 _275 = mul(frame_constants_view_constants[2u].view_to_world, float4(0.0f, 0.0f, 1.0f, 0.0f));
    float3 _276 = _275.xyz;
    float2 _280 = float2(surface_roughness_tex_dims);
    float2 _282 = in_var_C1.xz * 0.100000001490116119384765625f;
    float2 _284 = mul(_282 * 3.4639999866485595703125f, float2x2(float2(1.0f, 0.0f), float2(-0.57735025882720947265625f, 1.1547005176544189453125f)));
    float2 _285 = floor(_284);
    float2 _286 = frac(_284);
    float _287 = _286.x;
    float _288 = _286.y;
    float _289 = 1.0f - _287;
    float _290 = _289 - _288;
    bool _291 = _290 > 0.0f;
    float3 _303;
    if (_291)
    {
        _303 = float3(_285, _290);
    }
    else
    {
        _303 = float3(_285 + 1.0f.xx, -_290);
    }
    float3 _316;
    if (_291)
    {
        _316 = float3(_285 + float2(0.0f, 1.0f), _288);
    }
    else
    {
        _316 = float3(_285 + float2(1.0f, 0.0f), 1.0f - _288);
    }
    float3 _328;
    if (_291)
    {
        _328 = float3(_285 + float2(1.0f, 0.0f), _287);
    }
    else
    {
        _328 = float3(_285 + float2(0.0f, 1.0f), _289);
    }
    float4 _336 = float4(ddx(_282), -ddy(_282));
    uint2 _339 = uint2(_303.xy);
    uint2 _343 = ((_339 >> uint2(8u, 8u)) ^ _339.yx) * uint2(1103515245u, 1103515245u);
    float2 _351 = _336.xy;
    float2 _352 = _336.zw;
    uint2 _358 = uint2(_316.xy);
    uint2 _362 = ((_358 >> uint2(8u, 8u)) ^ _358.yx) * uint2(1103515245u, 1103515245u);
    uint2 _376 = uint2(_328.xy);
    uint2 _380 = ((_376 >> uint2(8u, 8u)) ^ _376.yx) * uint2(1103515245u, 1103515245u);
    float4 _400 = surface_roughness_tex.SampleLevel(sampler_llr, 0.0f.xx, (log(min(_280.x, _280.y)) * 1.44269502162933349609375f) + 1.0f);
    float _410 = 0.0199999995529651641845703125f + (0.300000011920928955078125f * (1.0f - ((((((surface_roughness_tex.SampleGrad(sampler_llr, _282 + (float2(((_343 >> uint2(8u, 8u)) ^ _343.yx) * uint2(1103515245u, 1103515245u)) * 2.3283064365386962890625e-10f), _351, _352) * _303.z) + (surface_roughness_tex.SampleGrad(sampler_llr, _282 + (float2(((_362 >> uint2(8u, 8u)) ^ _362.yx) * uint2(1103515245u, 1103515245u)) * 2.3283064365386962890625e-10f), _351, _352) * _316.z)) + (surface_roughness_tex.SampleGrad(sampler_llr, _282 + (float2(((_380 >> uint2(8u, 8u)) ^ _380.yx) * uint2(1103515245u, 1103515245u)) * 2.3283064365386962890625e-10f), _351, _352) * _328.z)) - _400) / length(float3(_303.z, _316.z, _328.z)).xxxx) + _400).x));
    float3 _411 = mul(_272, float3x3(float3(1.0f, -0.0f, 0.0f), float3(-0.0f, 0.0f, 1.0f), float3(-0.0f, -1.0f, 0.0f)));
    uint _412 = _248.x;
    uint _413 = _248.y;
    uint _420;
    _420 = 0u;
    uint _421;
    uint _416;
    uint _419;
    for (uint _415 = _413, _418 = _412, _422 = 32u; _422 > 0u; _415 = _416, _418 = _419, _420 = _421, _422 /= 2u)
    {
        uint _429 = uint((_418 & _422) > 0u);
        uint _432 = uint((_415 & _422) > 0u);
        _421 = _420 + ((_422 * _422) * ((3u * _429) ^ _432));
        if (_432 == 0u)
        {
            uint _445;
            uint _446;
            if (_429 == 1u)
            {
                _445 = 63u - _415;
                _446 = 63u - _418;
            }
            else
            {
                _445 = _415;
                _446 = _418;
            }
            _416 = _446;
            _419 = _445;
        }
        else
        {
            _416 = _415;
            _419 = _418;
        }
    }
    float4x4 _603;
    float _604;
    uint _449 = frame_constants_frame_index % 64u;
    float2 _455 = frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(_420 + (288u * _449))) + 0.5f.xx);
    float3 _464 = normalize(float3(_411.xy * 0.07500000298023223876953125f.xx, _411.z));
    float _466 = 6.283185482025146484375f * _455.x;
    float _468 = _464.z;
    precise float _67 = mad(1.0f - (_455.y * 0.300000011920928955078125f), 1.0f + _468, -_468);
    float _474 = sqrt(clamp(1.0f - (_67 * _67), 0.0f, 1.0f));
    float3 _480 = float3(_474 * cos(_466), _474 * sin(_466), _67) + _464;
    float _489 = _272.y;
    float _490 = 1.0f - _489;
    float _491 = _490 * _490;
    float _497 = 5.588237762451171875f * float(_449);
    float _506 = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(_412) + _497)) + (0.005837149918079376220703125f * (float(_413) + _497))));
    float4 _512 = mul(frame_constants_view_constants[0u].view_to_world, mul(frame_constants_view_constants[0u].sample_to_view, float4(_264, _263.y, half_depth_tex.Load(int3(_249, 0u)).x, 1.0f)));
    float3 _519 = _269.xyz;
    float4 _535 = mul(frame_constants_view_constants[0u].view_to_sample, mul(frame_constants_view_constants[0u].world_to_view, float4(in_var_C1 + ((reflect(_272, mul(float3x3(float3(1.0f, -0.0f, 0.0f), float3(-0.0f, 0.0f, 1.0f), float3(-0.0f, -1.0f, 0.0f)), normalize(float3(_480.xy * 0.07500000298023223876953125f.xx, _480.z)))) * float3(-1.0f, 1.0f, -1.0f)) * min(4.0f, 4.0f * length(in_var_C1 - (_512.xyz / _512.w.xxx)))), 1.0f)));
    float3 _539 = _535.xyz / _535.w.xxx;
    float4 _550 = _269 + (float4(_539 - _519, 0.0f) * float(int(sign(_539.z))));
    float _551 = _550.w;
    float3 _560 = (_550 / (float((_551 >= 0.0f) ? 1 : (-1)) * max(1.0000000133514319600180897396058e-10f, abs(_551))).xxxx).xyz;
    float3 _561 = _560 - _519;
    bool3 _562 = bool3(_561.x < 0.0f.xxx.x, _561.y < 0.0f.xxx.y, _561.z < 0.0f.xxx.z);
    float3 _565 = (float3(_562.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, _562.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, _562.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - _519) / _561;
    float3 _571 = _519 + (_561 * max(0.0f, max(_565.x, _565.y)));
    float3 _572 = _560 - _571;
    bool3 _573 = bool3(_572.x >= 0.0f.xxx.x, _572.y >= 0.0f.xxx.y, _572.z >= 0.0f.xxx.z);
    float3 _576 = (float3(_573.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, _573.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, _573.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - _571) / _572;
    float3 _584 = _571 + (_572 * min(1.0f, min(min(_576.x, _576.y), _576.z)));
    float _782;
    do
    {
        _603 = frame_constants_view_constants[0u].clip_to_view;
        _604 = _603[3].z;
        float _605 = 0.100000001490116119384765625f * _604;
        uint _606 = uint(int(max(2u, min(3u, uint(int(floor(length((((_584.xy * float2(0.5f, -0.5f)) + 0.5f.xx) - ((_571.xy * float2(0.5f, -0.5f)) + 0.5f.xx)) * _253))))))));
        float _699;
        float _767;
        float _768;
        float _769;
        float _770;
        bool _771;
        do
        {
            float3 _611 = _584 - _571;
            float _619;
            if (_584.z > 9.5367431640625e-07f)
            {
                _619 = float(_606);
            }
            else
            {
                _619 = max(1.0f, float(_606) - 1.0f);
            }
            float _620 = 1.0f / _619;
            float _693;
            float _694;
            float _695;
            float _696;
            float _697;
            bool _698;
            if (_606 > 0u)
            {
                float _626 = lerp(0.0f, 1.0f, pow(_506 * _620, 1.0f));
                float3 _628 = _571 + (_611 * _626);
                float _633 = 1.0f / _628.z;
                float4 _636 = half_depth_tex.SampleLevel(sampler_nnc, (_628.xy * float2(0.5f, -0.5f)) + 0.5f.xx, 0.0f);
                float _637 = _636.x;
                float _640 = (1.00000202655792236328125f / _637) - _633;
                bool _643 = (_640 < 0.0f) ? true : false;
                float _686;
                float _687;
                float _688;
                float _689;
                float _690;
                bool _691;
                float _692;
                if (_643)
                {
                    _686 = 0.0f;
                    _687 = _640;
                    _688 = _633 - (1.0f / _637);
                    _689 = _605;
                    _690 = _626;
                    _691 = _643;
                    _692 = 0.0f;
                }
                else
                {
                    float _651;
                    float _655;
                    _651 = _640;
                    _655 = _626;
                    float _652;
                    bool _654;
                    float _656;
                    float _681;
                    float _682;
                    float _683;
                    float _684;
                    bool _685;
                    uint _648 = 1u;
                    bool _653 = _643;
                    for (;;)
                    {
                        if (_648 < _606)
                        {
                            _656 = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(_648) + _506) * _620, 1.0f)));
                            float3 _666 = _571 + (_611 * _656);
                            float _671 = 1.0f / _666.z;
                            float4 _673 = half_depth_tex.SampleLevel(sampler_nnc, (_666.xy * float2(0.5f, -0.5f)) + 0.5f.xx, 0.0f);
                            float _674 = _673.x;
                            _652 = (1.00000202655792236328125f / _674) - _671;
                            _654 = (_652 < 0.0f) ? true : false;
                            if (_654)
                            {
                                _681 = _652;
                                _682 = _671 - (1.0f / _674);
                                _683 = _605;
                                _684 = _656;
                                _685 = _654;
                                break;
                            }
                            _648++;
                            _651 = _652;
                            _653 = _654;
                            _655 = _656;
                            continue;
                        }
                        else
                        {
                            _681 = 0.0f;
                            _682 = 0.0f;
                            _683 = 0.0f;
                            _684 = 1.0f;
                            _685 = _653;
                            break;
                        }
                    }
                    _686 = _651;
                    _687 = _681;
                    _688 = _682;
                    _689 = _683;
                    _690 = _684;
                    _691 = _685;
                    _692 = _655;
                }
                _693 = _686;
                _694 = _687;
                _695 = _688;
                _696 = _689;
                _697 = _690;
                _698 = _691;
                _699 = _692;
            }
            else
            {
                _693 = 0.0f;
                _694 = 0.0f;
                _695 = 0.0f;
                _696 = 0.0f;
                _697 = 1.0f;
                _698 = false;
                _699 = 0.0f;
            }
            if (_698)
            {
                float _704;
                float _707;
                float _709;
                float _711;
                float _713;
                float _715;
                _704 = _699;
                _707 = _693;
                _709 = _694;
                _711 = _695;
                _713 = _696;
                _715 = _697;
                for (uint _717 = 0u; _717 < 1u; )
                {
                    float _721 = _704 + _715;
                    float _722 = _721 * 0.5f;
                    float3 _724 = _571 + (_611 * _722);
                    float _729 = 1.0f / _724.z;
                    float4 _732 = half_depth_tex.SampleLevel(sampler_nnc, (_724.xy * float2(0.5f, -0.5f)) + 0.5f.xx, 0.0f);
                    float _733 = _732.x;
                    float _736 = (1.00000202655792236328125f / _733) - _729;
                    bool _739 = (_736 < 0.0f) ? true : false;
                    _704 = _739 ? _704 : _722;
                    _707 = _739 ? _707 : _736;
                    _709 = _739 ? _736 : _709;
                    _711 = _739 ? (_729 - (1.0f / _733)) : _711;
                    _713 = _739 ? _605 : _713;
                    _715 = _739 ? _722 : _715;
                    _717++;
                    continue;
                }
                float _743 = lerp(_704, _715, _707 / (_707 + (-_709)));
                float3 _745 = _571 + (_611 * _743);
                float _750 = 1.0f / _745.z;
                float4 _753 = half_depth_tex.SampleLevel(sampler_nnc, (_745.xy * float2(0.5f, -0.5f)) + 0.5f.xx, 0.0f);
                float _754 = _753.x;
                float _757 = (1.00000202655792236328125f / _754) - _750;
                bool _762 = (abs(_757) < (_707 * 0.89999997615814208984375f)) ? true : false;
                _767 = _762 ? _743 : _715;
                _768 = _762 ? _757 : _709;
                _769 = _762 ? _605 : _713;
                _770 = _762 ? (_750 - (1.0f / _754)) : _711;
                _771 = true;
                break;
            }
            else
            {
                _767 = _699;
                _768 = _195;
                _769 = _195;
                _770 = _195;
                _771 = false;
                break;
            }
            break; 
        } while(false);
        bool _779;
        if (_771)
        {
            bool _778;
            if (_770 < _769)
            {
                _778 = _768 < _769;
            }
            else
            {
                _778 = false;
            }
            _779 = _778;
        }
        else
        {
            _779 = false;
        }
        if (_779)
        {
            _782 = _767;
            break;
        }
        _782 = _699;
        break;
    } while(false);
    float3 _784 = lerp(_571, _584, _782.xxx);
    float _785 = _784.x;
    float2 _790 = (float4(_785, _784.y, _228, 1.0f).xy * float2(0.5f, -0.5f)) + 0.5f.xx;
    float4 _797 = mul(frame_constants_view_constants[0u].view_to_world, mul(frame_constants_view_constants[0u].sample_to_view, float4(_785, _784.y, half_depth_tex.SampleLevel(sampler_nnc, _790, 0.0f).x, 1.0f)));
    float3 _801 = _797.xyz / _797.w.xxx;
    float3 _807 = _801 - in_var_C1;
    float3 _813 = -_276;
    int2 _814 = int2(_248);
    float2 _1022;
    float _1023;
    do
    {
        float4 _822 = mul(frame_constants_view_constants[2u].world_to_sample, float4(in_var_C1, 1.0f));
        if (_822.z > 1.0f)
        {
            _1022 = 0.0f.xx;
            _1023 = 0.0f;
            break;
        }
        float2 _827 = _822.xy;
        float2 _828 = abs(_827);
        bool _830 = any(bool2(_828.x > 0.9833333492279052734375f.xx.x, _828.y > 0.9833333492279052734375f.xx.y));
        float2 _834;
        if (_830)
        {
            _834 = _827 * 0.5f;
        }
        else
        {
            _834 = _827;
        }
        float _835 = _830 ? 5.0f : 10.0f;
        float _837 = _830 ? 0.5f : 1.0f;
        float2 _838 = abs(_834);
        float2 _841 = 1.0f.xx - (0.000833333353511989116668701171875f.xx * (2.0f * _835));
        bool _843 = any(bool2(_838.x > _841.x, _838.y > _841.y));
        float _849;
        float2 _850;
        float _851;
        if (_843)
        {
            _849 = _835 * 0.2857142984867095947265625f;
            _850 = _834 * 0.2857142984867095947265625f;
            _851 = _837 * 0.2857142984867095947265625f;
        }
        else
        {
            _849 = _835;
            _850 = _834;
            _851 = _837;
        }
        float _854 = float(_843 ? 2u : uint(_830));
        uint _856 = 2u + uint(_854);
        float _859 = _813.y;
        float4 _871 = float4(_850.x, _850.y, _822.z, _822.w) + mul(frame_constants_view_constants[_856].world_to_sample, float4((float3(0.0f, 1.0f, 0.0f) * (0.066666670143604278564453125f * clamp(1.0f - _859, 0.0f, 1.0f))) / _851.xxx, 0.0f));
        float2 _874 = (_871.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
        float3 _875 = float3(_874.x, _874.y, _871.z);
        float _880 = max((0.300000011920928955078125f / _851) * (1.0f - abs(_859)), 0.02500000037252902984619140625f);
        float _891 = float((uint((_814.x * 11) + (_814.y * 7)) + (frame_constants_frame_index * 5u)) % 64u) * 1.61803400516510009765625f;
        uint2 _892 = uint2(_814);
        float _904 = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(_892.x) + _497)) + (0.005837149918079376220703125f * (float(_892.y) + _497))));
        float4 _906 = mul(frame_constants_view_constants[_856].world_to_sample, float4(0.0f, 1.0f, 0.0f, 0.0f));
        float _908;
        int _911;
        _908 = 0.0f;
        _911 = 0;
        float _909;
        int _912;
        for (int _913 = 0; _913 < 5; _908 = _909, _911 = _912, _913++)
        {
            float _918 = float(_913);
            float _920 = (_918 + _891) * 2.3999631404876708984375f;
            float3 _933 = _875 + float3((((float2(cos(_920), sin(_920)) * (_918 + _904)) * 0.20000000298023223876953125f.xx) * _849) * 0.000833333353511989116668701171875f.xx, _880 * (-0.0009765625f));
            float4 _940 = shadow_map.SampleLevel(sampler_shadow_nearest, float3(_933.xy, _854), 0.0f);
            float _941 = _940.x;
            if (_933.z > _941)
            {
                _909 = _908 + _941;
                _912 = _911 + 1;
            }
            else
            {
                _909 = _908;
                _912 = _911;
            }
        }
        float _949 = _908 / float(_911);
        if (_911 == 0)
        {
            _1022 = 0.0f.xx;
            _1023 = 0.0f;
            break;
        }
        float _963 = max(((_871.z - _949) * ((17.0666675567626953125f * _849) * pow(10.0f, smoothstep(0.0f, 0.25f, frame_constants_fog_tint_thickness.w)))) / _949, 0.0f);
        float _970 = (_963 / pow(pow(_963 / (_849 - 1.0f), 5.0f) + 1.0f, 0.20000000298023223876953125f)) + 1.0f;
        float _972 = _906.z;
        float2 _982 = (((_906.xy * float(int(sign(_972)))) / max(0.1500000059604644775390625f, abs(_972)).xx) * 0.0009765625f.xx) * 2.0f;
        float2 _984;
        _984 = 0.0f.xx;
        for (int _987 = 0; _987 < 10; )
        {
            float _991 = float(_987);
            float _993 = (_991 + _891) * 2.3999631404876708984375f;
            float2 _1000 = ((float2(cos(_993), sin(_993)) * (_991 + _904)) * 0.100000001490116119384765625f.xx) * _970;
            float3 _1006 = _875 + float3(_1000 * 0.000833333353511989116668701171875f.xx, _880 * (-0.0009765625f));
            float _1018 = 1.0f - shadow_map.SampleCmpLevelZero(sampler_shadow_linear, float3(_1006.xy, _854), _1006.z + min(0.0f, dot(_982, _1000)));
            _984 += float2(_1018, _1018 * _1018);
            _987++;
            continue;
        }
        _1022 = _984 * 0.100000001490116119384765625f;
        _1023 = _970;
        break;
    } while(false);
    float _1027 = clamp((_1022.x - 0.100000001490116119384765625f) * 1.4285714626312255859375f, 0.0f, 1.0f);
    float2 _1040 = _1027.xx + ((float2(-1.0f, 1.0f) * sqrt(max(0.0f, _1022.y - (_1022.x * _1022.x)))) * (3.0f * max(0.0f, 1.0f - (_1023 * 0.20000000298023223876953125f))));
    float2 _1046 = (float2(_814) + 0.5f.xx) / float2(framebuffer_dims);
    float4 _1055 = float4((_1046 - 0.5f.xx) * float2(2.0f, -2.0f), gl_FragCoord.z, 1.0f);
    float4 _1058 = mul(frame_constants_view_constants[0u].clip_to_prev_clip, mul(frame_constants_view_constants[0u].view_to_clip, mul(_603, _1055)));
    float2 _1064 = ((_1058.xy / _1058.w.xx) * float2(0.5f, -0.5f)) + 0.5f.xx;
    float2 _1069 = clamp(_1064, 0.0f.xx, 1.0f.xx);
    float4 _1082 = float4(_1064 - ((_1055.xy * float2(0.5f, -0.5f)) + 0.5f.xx), float(all(bool2(_1069.x == _1064.x, _1069.y == _1064.y))), 1.0f) * float4(1.0f, 1.0f, clamp(1.0f - length(frame_constants_view_constants[0u].clip_to_prev_clip[3].xyz), 0.0f, 1.0f), 1.0f);
    float _1085 = _1082.z;
    bool _1099;
    if (_1085 > 0.0f)
    {
        bool _1098;
        if (history_valid != 0u)
        {
            _1098 = !(0u == frame_constants_frame_index);
        }
        else
        {
            _1098 = false;
        }
        _1099 = _1098;
    }
    else
    {
        _1099 = false;
    }
    float _1107;
    if (_1099)
    {
        _1107 = shadows_history_tex.SampleLevel(sampler_lnc, _1046 + _1082.xy, 0.0f).x;
    }
    else
    {
        _1107 = _1027;
    }
    float _1112 = lerp(clamp(_1107, _1040.x, _1040.y), _1027, lerp(1.0f, 0.125f, _1085));
    float _1113 = 1.0f - _1112;
    float4 _1121 = mul(frame_constants_view_constants[0u].view_to_sample, mul(frame_constants_view_constants[0u].world_to_view, float4(in_var_C1 + (_276 * 0.5f), 1.0f)));
    float3 _1125 = _1121.xyz / _1121.w.xxx;
    float4 _1136 = _269 + (float4(_1125 - _519, 0.0f) * float(int(sign(_1125.z))));
    float _1137 = _1136.w;
    float3 _1146 = (_1136 / (float((_1137 >= 0.0f) ? 1 : (-1)) * max(1.0000000133514319600180897396058e-10f, abs(_1137))).xxxx).xyz;
    float3 _1147 = _1146 - _519;
    bool3 _1148 = bool3(_1147.x < 0.0f.xxx.x, _1147.y < 0.0f.xxx.y, _1147.z < 0.0f.xxx.z);
    float3 _1151 = (float3(_1148.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, _1148.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, _1148.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - _519) / _1147;
    float3 _1157 = _519 + (_1147 * max(0.0f, max(_1151.x, _1151.y)));
    float3 _1158 = _1146 - _1157;
    bool3 _1159 = bool3(_1158.x >= 0.0f.xxx.x, _1158.y >= 0.0f.xxx.y, _1158.z >= 0.0f.xxx.z);
    float3 _1162 = (float3(_1159.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, _1159.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, _1159.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - _1157) / _1158;
    float3 _1170 = _1157 + (_1158 * min(1.0f, min(min(_1162.x, _1162.y), _1162.z)));
    bool _1313;
    float _1314;
    float _1315;
    do
    {
        float _1188 = 0.5f * _604;
        uint _1189 = uint(int(max(2u, min(3u, uint(int(floor(length((((_1170.xy * float2(0.5f, -0.5f)) + 0.5f.xx) - ((_1157.xy * float2(0.5f, -0.5f)) + 0.5f.xx)) * _253))))))));
        float _1293;
        float _1297;
        float _1298;
        float _1299;
        float _1300;
        bool _1301;
        do
        {
            float3 _1194 = _1170 - _1157;
            float _1202;
            if (_1170.z > 9.5367431640625e-07f)
            {
                _1202 = float(_1189);
            }
            else
            {
                _1202 = max(1.0f, float(_1189) - 1.0f);
            }
            float _1203 = 1.0f / _1202;
            float _1288;
            float _1289;
            float _1290;
            float _1291;
            bool _1292;
            if (_1189 > 0u)
            {
                float _1209 = lerp(0.0f, 1.0f, pow(_506 * _1203, 1.0f));
                float3 _1211 = _1157 + (_1194 * _1209);
                float2 _1214 = (_1211.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                float _1216 = 1.0f / _1211.z;
                float _1221 = 1.0f / half_depth_tex.SampleLevel(sampler_llc, _1214, 0.0f).x;
                float _1225 = 1.0f / half_depth_tex.SampleLevel(sampler_nnc, _1214, 0.0f).x;
                float _1229 = (max(_1221, _1225) * 1.00000202655792236328125f) - _1216;
                float _1230 = _1216 - min(_1221, _1225);
                bool _1233 = (_1229 < 0.0f) ? (_1230 < _1188) : false;
                float _1282;
                float _1283;
                float _1284;
                float _1285;
                bool _1286;
                float _1287;
                if (_1233)
                {
                    _1282 = _1229;
                    _1283 = _1230;
                    _1284 = _1188;
                    _1285 = _1209;
                    _1286 = _1233;
                    _1287 = 0.0f;
                }
                else
                {
                    float _1243;
                    _1243 = _1209;
                    bool _1242;
                    float _1244;
                    float _1277;
                    float _1278;
                    float _1279;
                    float _1280;
                    bool _1281;
                    uint _1238 = 1u;
                    bool _1241 = _1233;
                    for (;;)
                    {
                        if (_1238 < _1189)
                        {
                            _1244 = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(_1238) + _506) * _1203, 1.0f)));
                            float3 _1254 = _1157 + (_1194 * _1244);
                            float2 _1257 = (_1254.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                            float _1259 = 1.0f / _1254.z;
                            float _1263 = 1.0f / half_depth_tex.SampleLevel(sampler_llc, _1257, 0.0f).x;
                            float _1267 = 1.0f / half_depth_tex.SampleLevel(sampler_nnc, _1257, 0.0f).x;
                            float _1271 = (max(_1263, _1267) * 1.00000202655792236328125f) - _1259;
                            float _1272 = _1259 - min(_1263, _1267);
                            _1242 = (_1271 < 0.0f) ? (_1272 < _1188) : false;
                            if (_1242)
                            {
                                _1277 = _1271;
                                _1278 = _1272;
                                _1279 = _1188;
                                _1280 = _1244;
                                _1281 = _1242;
                                break;
                            }
                            _1238++;
                            _1241 = _1242;
                            _1243 = _1244;
                            continue;
                        }
                        else
                        {
                            _1277 = 0.0f;
                            _1278 = 0.0f;
                            _1279 = 0.0f;
                            _1280 = 1.0f;
                            _1281 = _1241;
                            break;
                        }
                    }
                    _1282 = _1277;
                    _1283 = _1278;
                    _1284 = _1279;
                    _1285 = _1280;
                    _1286 = _1281;
                    _1287 = _1243;
                }
                _1288 = _1282;
                _1289 = _1283;
                _1290 = _1284;
                _1291 = _1285;
                _1292 = _1286;
                _1293 = _1287;
            }
            else
            {
                _1288 = 0.0f;
                _1289 = 0.0f;
                _1290 = 0.0f;
                _1291 = 1.0f;
                _1292 = false;
                _1293 = 0.0f;
            }
            if (_1292)
            {
                _1297 = _1291;
                _1298 = _1288;
                _1299 = _1290;
                _1300 = _1289;
                _1301 = true;
                break;
            }
            else
            {
                _1297 = _1293;
                _1298 = _195;
                _1299 = _195;
                _1300 = _195;
                _1301 = false;
                break;
            }
            break; 
        } while(false);
        bool _1309;
        if (_1301)
        {
            bool _1308;
            if (_1300 < _1299)
            {
                _1308 = _1298 < _1299;
            }
            else
            {
                _1308 = false;
            }
            _1309 = _1308;
        }
        else
        {
            _1309 = false;
        }
        if (_1309)
        {
            _1313 = true;
            _1314 = _1297;
            _1315 = _1300 / _1299;
            break;
        }
        _1313 = false;
        _1314 = _1293;
        _1315 = 0.0f;
        break;
    } while(false);
    float _1323;
    if (_1313)
    {
        _1323 = min(_1113, smoothstep(0.5f, 0.0f, (1.0f - _1315) * (1.0f - _1314)));
    }
    else
    {
        _1323 = _1113;
    }
    uint _1334;
    if (_244)
    {
        _1334 = 0u;
    }
    else
    {
        uint2 _1327 = _248 & uint2(1u, 1u);
        _1334 = 1u << ((_1327.x + (_1327.y * 2u)) & 31u);
    }
    uint _1336 = _1334 + QuadReadAcrossX(_1334);
    uint2 _1339 = _248 & uint2(1u, 1u);
    if ((_1339.x + (_1339.y * 2u)) == firstbithigh(_1336 + QuadReadAcrossY(_1336)))
    {
        uint _1349;
        __alias_ice_refl_count_buffer.InterlockedAdd(12, 1u, _1349);
        __alias_ice_refl_px_buffer.Store<uint>(_1349 * 4 + 0, _249.x + (_249.y * 65536u));
        if ((_1349 & 63u) == 0u)
        {
            uint _1360;
            __alias_ice_refl_count_buffer.InterlockedAdd(0, 1u, _1360);
        }
    }
    float _1363;
    float3 _1415;
    do
    {
        _1363 = _275.y;
        bool _1369;
        if (!(_1363 <= 0.0f))
        {
            _1369 = _489 <= 0.0f;
        }
        else
        {
            _1369 = true;
        }
        if (_1369)
        {
            _1415 = 0.0f.xxx;
            break;
        }
        float _1372 = _410 * _410;
        float3 _1374 = normalize(_272 + _276);
        float _1375 = _1374.y;
        float _1378 = max(0.0f, 1.0f - dot(_1374, _276));
        float _1379 = _1378 * _1378;
        float _1406 = ((_1375 * _1375) * (_1372 - 1.0f)) + 1.0f;
        _1415 = ((lerp(0.02999999932944774627685546875f.xxx, 1.0f.xxx, ((_1379 * _1379) * _1378).xxx) * (((2.0f * _1363) * _489) / ((_1363 * sqrt(((((-_489) * _1372) + _489) * _489) + _1372)) + (_489 * sqrt(((((-_1363) * _1372) + _1363) * _1363) + _1372))))) * (_1372 / ((3.1415927410125732421875f * _1406) * _1406))) / ((4.0f * _489) * _1363).xxx;
        break;
    } while(false);
    float3 _1440 = mul(_807 * float3(1.0f, -1.0f, 1.0f), float3x3(float3(1.0f, -0.0f, 0.0f), float3(-0.0f, 0.0f, 1.0f), float3(-0.0f, -1.0f, 0.0f)));
    float _1442 = length(_1440);
    float3 _1444 = _1440 * (1.0f / _1442);
    float3 _1465 = clamp((exp(float3(-0.5950000286102294921875f, -0.374000012874603271484375f, -0.3060000240802764892578125f) * ((abs(in_var_C1.y - _801.y) + length(_807)) + 0.699999988079071044921875f)) * (0.959999978542327880859375f - (0.959999978542327880859375f * ((_491 * _491) * _490)))) * refraction_tex.SampleLevel(sampler_lnc, _790, 0.0f).xyz, 0.0f.xxx, 65408.0f.xxx);
    float _1475 = asfloat((asuint(max(max(1.52587890625e-05f, _1465.x), max(_1465.y, _1465.z))) + 125845504u) & 2139095040u);
    uint3 _1478 = asuint(_1465 + _1475.xxx);
    out_var_SV_TARGET0 = lerp((frame_constants_sun_color.xyz * _1415.x) * (_1323 * clamp(_1363, 0.0f, 1.0f)), ((frame_constants_sun_color.xyz * (0.699999988079071044921875f * _1323)) * 0.3183098733425140380859375f.xxx) + frame_constants_sky_high_color.xyz, 0.00999999977648258209228515625f.xxx);
    out_var_SV_TARGET1 = sqrt(_410);
    out_var_SV_TARGET2 = ((uint((clamp((_1444.x * 0.5f) + 0.5f, 0.0f, 1.0f) * 255.0f) + 0.5f) << 24u) | (uint((clamp((_1444.y * 0.5f) + 0.5f, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) | spvPackHalf2x16(float2(_1442, 0.0f));
    out_var_SV_TARGET3 = ((((asuint(_1475) << 4u) + 268435456u) | (_1478.z << 18u)) | (_1478.y << 9u)) | (_1478.x & 511u);
    out_var_SV_TARGET4 = _1112 + 0.00196078442968428134918212890625f;
}

[earlydepthstencil]
SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    gl_FragCoord = stage_input.gl_FragCoord;
    gl_FragCoord.w = 1.0 / gl_FragCoord.w;
    in_var_C1 = stage_input.in_var_C1;
    frag_main();
    SPIRV_Cross_Output stage_output;
    stage_output.out_var_SV_TARGET0 = out_var_SV_TARGET0;
    stage_output.out_var_SV_TARGET1 = out_var_SV_TARGET1;
    stage_output.out_var_SV_TARGET2 = out_var_SV_TARGET2;
    stage_output.out_var_SV_TARGET3 = out_var_SV_TARGET3;
    stage_output.out_var_SV_TARGET4 = out_var_SV_TARGET4;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)

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

static float _195;
static float _228;

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
    uint current_view_index : packoffset(c0);
    uint history_valid : packoffset(c0.y);
    uint2 blue_noise_texture_dims : packoffset(c0.z);
    uint2 framebuffer_dims : packoffset(c1);
    uint2 water_mask_dims : packoffset(c1.z);
    uint2 half_depth_tex_dims : packoffset(c2);
    uint2 refraction_tex_dims : packoffset(c2.z);
    uint2 reprojection_tex_dims : packoffset(c3);
    uint2 shadows_history_tex_dims : packoffset(c3.z);
    uint2 surface_roughness_tex_dims : packoffset(c4);
    column_major float4x4 model_to_world : packoffset(c5);
};

RWByteAddressBuffer __alias_ice_refl_count_buffer : register(u23, space0);
RWByteAddressBuffer __alias_ice_refl_px_buffer : register(u24, space0);
SamplerState sampler_lnc : register(s0, space0);
SamplerState sampler_llr : register(s1, space0);
SamplerState sampler_nnc : register(s2, space0);
SamplerState sampler_llc : register(s3, space0);
SamplerState sampler_shadow_nearest : register(s11, space0);
SamplerComparisonState sampler_shadow_linear : register(s12, space0);
Texture2DArray<float4> shadow_map : register(t15, space0);
Texture2D<float4> half_depth_tex : register(t18, space0);
Texture2D<float4> refraction_tex : register(t19, space0);
Texture2D<float4> shadows_history_tex : register(t21, space0);
Texture2D<float4> surface_roughness_tex : register(t22, space0);

static float4 gl_FragCoord;
static float3 in_var_C1;
static float3 out_var_SV_TARGET0;
static float out_var_SV_TARGET1;
static uint out_var_SV_TARGET2;
static uint out_var_SV_TARGET3;
static float out_var_SV_TARGET4;

struct SPIRV_Cross_Input
{
    [[vk::location(1)]] float3 in_var_C1 : TEXCOORD1;
    float4 gl_FragCoord : SV_Position;
};

struct SPIRV_Cross_Output
{
    float3 out_var_SV_TARGET0 : SV_Target0;
    float out_var_SV_TARGET1 : SV_Target1;
    uint out_var_SV_TARGET2 : SV_Target2;
    uint out_var_SV_TARGET3 : SV_Target3;
    float out_var_SV_TARGET4 : SV_Target4;
};

uint spvPackHalf2x16(float2 value)
{
    uint2 Packed = f32tof16(value);
    return Packed.x | (Packed.y << 16);
}

float2 spvUnpackHalf2x16(uint value)
{
    return f16tof32(uint2(value & 0xffff, value >> 16));
}

void frag_main()
{
    bool _244 = IsHelperLane();
    uint2 _248 = uint2(gl_FragCoord.xy);
    uint2 _249 = _248 / uint2(2u, 2u);
    float2 _253 = float2(half_depth_tex_dims);
    float2 _263 = (((float2(int2(_249)) + 0.5f.xx) / _253) - 0.5f.xx) * float2(2.0f, -2.0f);
    float _264 = _263.x;
    float4 _269 = float4(_264, _263.y, gl_FragCoord.z, 1.0f);
    float3 _272 = -normalize(mul(frame_constants_view_constants[0u].view_to_world, mul(frame_constants_view_constants[0u].sample_to_view, float4(_264, _263.y, 0.0f, 1.0f))).xyz);
    float4 _275 = mul(frame_constants_view_constants[2u].view_to_world, float4(0.0f, 0.0f, 1.0f, 0.0f));
    float3 _276 = _275.xyz;
    float2 _280 = float2(surface_roughness_tex_dims);
    float2 _282 = in_var_C1.xz * 0.100000001490116119384765625f;
    float2 _284 = mul(_282 * 3.4639999866485595703125f, float2x2(float2(1.0f, 0.0f), float2(-0.57735025882720947265625f, 1.1547005176544189453125f)));
    float2 _285 = floor(_284);
    float2 _286 = frac(_284);
    float _287 = _286.x;
    float _288 = _286.y;
    float _289 = 1.0f - _287;
    float _290 = _289 - _288;
    bool _291 = _290 > 0.0f;
    float3 _303;
    if (_291)
    {
        _303 = float3(_285, _290);
    }
    else
    {
        _303 = float3(_285 + 1.0f.xx, -_290);
    }
    float3 _316;
    if (_291)
    {
        _316 = float3(_285 + float2(0.0f, 1.0f), _288);
    }
    else
    {
        _316 = float3(_285 + float2(1.0f, 0.0f), 1.0f - _288);
    }
    float3 _328;
    if (_291)
    {
        _328 = float3(_285 + float2(1.0f, 0.0f), _287);
    }
    else
    {
        _328 = float3(_285 + float2(0.0f, 1.0f), _289);
    }
    float4 _336 = float4(ddx(_282), -ddy(_282));
    uint2 _339 = uint2(_303.xy);
    uint2 _343 = ((_339 >> uint2(8u, 8u)) ^ _339.yx) * uint2(1103515245u, 1103515245u);
    float2 _351 = _336.xy;
    float2 _352 = _336.zw;
    uint2 _358 = uint2(_316.xy);
    uint2 _362 = ((_358 >> uint2(8u, 8u)) ^ _358.yx) * uint2(1103515245u, 1103515245u);
    uint2 _376 = uint2(_328.xy);
    uint2 _380 = ((_376 >> uint2(8u, 8u)) ^ _376.yx) * uint2(1103515245u, 1103515245u);
    float4 _400 = surface_roughness_tex.SampleLevel(sampler_llr, 0.0f.xx, (log(min(_280.x, _280.y)) * 1.44269502162933349609375f) + 1.0f);
    float _410 = 0.0199999995529651641845703125f + (0.300000011920928955078125f * (1.0f - ((((((surface_roughness_tex.SampleGrad(sampler_llr, _282 + (float2(((_343 >> uint2(8u, 8u)) ^ _343.yx) * uint2(1103515245u, 1103515245u)) * 2.3283064365386962890625e-10f), _351, _352) * _303.z) + (surface_roughness_tex.SampleGrad(sampler_llr, _282 + (float2(((_362 >> uint2(8u, 8u)) ^ _362.yx) * uint2(1103515245u, 1103515245u)) * 2.3283064365386962890625e-10f), _351, _352) * _316.z)) + (surface_roughness_tex.SampleGrad(sampler_llr, _282 + (float2(((_380 >> uint2(8u, 8u)) ^ _380.yx) * uint2(1103515245u, 1103515245u)) * 2.3283064365386962890625e-10f), _351, _352) * _328.z)) - _400) / length(float3(_303.z, _316.z, _328.z)).xxxx) + _400).x));
    float3 _411 = mul(_272, float3x3(float3(1.0f, -0.0f, 0.0f), float3(-0.0f, 0.0f, 1.0f), float3(-0.0f, -1.0f, 0.0f)));
    uint _412 = _248.x;
    uint _413 = _248.y;
    uint _420;
    _420 = 0u;
    uint _421;
    uint _416;
    uint _419;
    for (uint _415 = _413, _418 = _412, _422 = 32u; _422 > 0u; _415 = _416, _418 = _419, _420 = _421, _422 /= 2u)
    {
        uint _429 = uint((_418 & _422) > 0u);
        uint _432 = uint((_415 & _422) > 0u);
        _421 = _420 + ((_422 * _422) * ((3u * _429) ^ _432));
        if (_432 == 0u)
        {
            uint _445;
            uint _446;
            if (_429 == 1u)
            {
                _445 = 63u - _415;
                _446 = 63u - _418;
            }
            else
            {
                _445 = _415;
                _446 = _418;
            }
            _416 = _446;
            _419 = _445;
        }
        else
        {
            _416 = _415;
            _419 = _418;
        }
    }
    float4x4 _603;
    float _604;
    uint _449 = frame_constants_frame_index % 64u;
    float2 _455 = frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(_420 + (288u * _449))) + 0.5f.xx);
    float3 _464 = normalize(float3(_411.xy * 0.07500000298023223876953125f.xx, _411.z));
    float _466 = 6.283185482025146484375f * _455.x;
    float _468 = _464.z;
    precise float _67 = mad(1.0f - (_455.y * 0.300000011920928955078125f), 1.0f + _468, -_468);
    float _474 = sqrt(clamp(1.0f - (_67 * _67), 0.0f, 1.0f));
    float3 _480 = float3(_474 * cos(_466), _474 * sin(_466), _67) + _464;
    float _489 = _272.y;
    float _490 = 1.0f - _489;
    float _491 = _490 * _490;
    float _497 = 5.588237762451171875f * float(_449);
    float _506 = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(_412) + _497)) + (0.005837149918079376220703125f * (float(_413) + _497))));
    float4 _512 = mul(frame_constants_view_constants[0u].view_to_world, mul(frame_constants_view_constants[0u].sample_to_view, float4(_264, _263.y, half_depth_tex.Load(int3(_249, 0u)).x, 1.0f)));
    float3 _519 = _269.xyz;
    float4 _535 = mul(frame_constants_view_constants[0u].view_to_sample, mul(frame_constants_view_constants[0u].world_to_view, float4(in_var_C1 + ((reflect(_272, mul(float3x3(float3(1.0f, -0.0f, 0.0f), float3(-0.0f, 0.0f, 1.0f), float3(-0.0f, -1.0f, 0.0f)), normalize(float3(_480.xy * 0.07500000298023223876953125f.xx, _480.z)))) * float3(-1.0f, 1.0f, -1.0f)) * min(4.0f, 4.0f * length(in_var_C1 - (_512.xyz / _512.w.xxx)))), 1.0f)));
    float3 _539 = _535.xyz / _535.w.xxx;
    float4 _550 = _269 + (float4(_539 - _519, 0.0f) * float(int(sign(_539.z))));
    float _551 = _550.w;
    float3 _560 = (_550 / (float((_551 >= 0.0f) ? 1 : (-1)) * max(1.0000000133514319600180897396058e-10f, abs(_551))).xxxx).xyz;
    float3 _561 = _560 - _519;
    bool3 _562 = bool3(_561.x < 0.0f.xxx.x, _561.y < 0.0f.xxx.y, _561.z < 0.0f.xxx.z);
    float3 _565 = (float3(_562.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, _562.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, _562.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - _519) / _561;
    float3 _571 = _519 + (_561 * max(0.0f, max(_565.x, _565.y)));
    float3 _572 = _560 - _571;
    bool3 _573 = bool3(_572.x >= 0.0f.xxx.x, _572.y >= 0.0f.xxx.y, _572.z >= 0.0f.xxx.z);
    float3 _576 = (float3(_573.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, _573.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, _573.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - _571) / _572;
    float3 _584 = _571 + (_572 * min(1.0f, min(min(_576.x, _576.y), _576.z)));
    float _782;
    do
    {
        _603 = frame_constants_view_constants[0u].clip_to_view;
        _604 = _603[3].z;
        float _605 = 0.100000001490116119384765625f * _604;
        uint _606 = uint(int(max(2u, min(3u, uint(int(floor(length((((_584.xy * float2(0.5f, -0.5f)) + 0.5f.xx) - ((_571.xy * float2(0.5f, -0.5f)) + 0.5f.xx)) * _253))))))));
        float _699;
        float _767;
        float _768;
        float _769;
        float _770;
        bool _771;
        do
        {
            float3 _611 = _584 - _571;
            float _619;
            if (_584.z > 9.5367431640625e-07f)
            {
                _619 = float(_606);
            }
            else
            {
                _619 = max(1.0f, float(_606) - 1.0f);
            }
            float _620 = 1.0f / _619;
            float _693;
            float _694;
            float _695;
            float _696;
            float _697;
            bool _698;
            if (_606 > 0u)
            {
                float _626 = lerp(0.0f, 1.0f, pow(_506 * _620, 1.0f));
                float3 _628 = _571 + (_611 * _626);
                float _633 = 1.0f / _628.z;
                float4 _636 = half_depth_tex.SampleLevel(sampler_nnc, (_628.xy * float2(0.5f, -0.5f)) + 0.5f.xx, 0.0f);
                float _637 = _636.x;
                float _640 = (1.00000202655792236328125f / _637) - _633;
                bool _643 = (_640 < 0.0f) ? true : false;
                float _686;
                float _687;
                float _688;
                float _689;
                float _690;
                bool _691;
                float _692;
                if (_643)
                {
                    _686 = 0.0f;
                    _687 = _640;
                    _688 = _633 - (1.0f / _637);
                    _689 = _605;
                    _690 = _626;
                    _691 = _643;
                    _692 = 0.0f;
                }
                else
                {
                    float _651;
                    float _655;
                    _651 = _640;
                    _655 = _626;
                    float _652;
                    bool _654;
                    float _656;
                    float _681;
                    float _682;
                    float _683;
                    float _684;
                    bool _685;
                    uint _648 = 1u;
                    bool _653 = _643;
                    for (;;)
                    {
                        if (_648 < _606)
                        {
                            _656 = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(_648) + _506) * _620, 1.0f)));
                            float3 _666 = _571 + (_611 * _656);
                            float _671 = 1.0f / _666.z;
                            float4 _673 = half_depth_tex.SampleLevel(sampler_nnc, (_666.xy * float2(0.5f, -0.5f)) + 0.5f.xx, 0.0f);
                            float _674 = _673.x;
                            _652 = (1.00000202655792236328125f / _674) - _671;
                            _654 = (_652 < 0.0f) ? true : false;
                            if (_654)
                            {
                                _681 = _652;
                                _682 = _671 - (1.0f / _674);
                                _683 = _605;
                                _684 = _656;
                                _685 = _654;
                                break;
                            }
                            _648++;
                            _651 = _652;
                            _653 = _654;
                            _655 = _656;
                            continue;
                        }
                        else
                        {
                            _681 = 0.0f;
                            _682 = 0.0f;
                            _683 = 0.0f;
                            _684 = 1.0f;
                            _685 = _653;
                            break;
                        }
                    }
                    _686 = _651;
                    _687 = _681;
                    _688 = _682;
                    _689 = _683;
                    _690 = _684;
                    _691 = _685;
                    _692 = _655;
                }
                _693 = _686;
                _694 = _687;
                _695 = _688;
                _696 = _689;
                _697 = _690;
                _698 = _691;
                _699 = _692;
            }
            else
            {
                _693 = 0.0f;
                _694 = 0.0f;
                _695 = 0.0f;
                _696 = 0.0f;
                _697 = 1.0f;
                _698 = false;
                _699 = 0.0f;
            }
            if (_698)
            {
                float _704;
                float _707;
                float _709;
                float _711;
                float _713;
                float _715;
                _704 = _699;
                _707 = _693;
                _709 = _694;
                _711 = _695;
                _713 = _696;
                _715 = _697;
                for (uint _717 = 0u; _717 < 1u; )
                {
                    float _721 = _704 + _715;
                    float _722 = _721 * 0.5f;
                    float3 _724 = _571 + (_611 * _722);
                    float _729 = 1.0f / _724.z;
                    float4 _732 = half_depth_tex.SampleLevel(sampler_nnc, (_724.xy * float2(0.5f, -0.5f)) + 0.5f.xx, 0.0f);
                    float _733 = _732.x;
                    float _736 = (1.00000202655792236328125f / _733) - _729;
                    bool _739 = (_736 < 0.0f) ? true : false;
                    _704 = _739 ? _704 : _722;
                    _707 = _739 ? _707 : _736;
                    _709 = _739 ? _736 : _709;
                    _711 = _739 ? (_729 - (1.0f / _733)) : _711;
                    _713 = _739 ? _605 : _713;
                    _715 = _739 ? _722 : _715;
                    _717++;
                    continue;
                }
                float _743 = lerp(_704, _715, _707 / (_707 + (-_709)));
                float3 _745 = _571 + (_611 * _743);
                float _750 = 1.0f / _745.z;
                float4 _753 = half_depth_tex.SampleLevel(sampler_nnc, (_745.xy * float2(0.5f, -0.5f)) + 0.5f.xx, 0.0f);
                float _754 = _753.x;
                float _757 = (1.00000202655792236328125f / _754) - _750;
                bool _762 = (abs(_757) < (_707 * 0.89999997615814208984375f)) ? true : false;
                _767 = _762 ? _743 : _715;
                _768 = _762 ? _757 : _709;
                _769 = _762 ? _605 : _713;
                _770 = _762 ? (_750 - (1.0f / _754)) : _711;
                _771 = true;
                break;
            }
            else
            {
                _767 = _699;
                _768 = _195;
                _769 = _195;
                _770 = _195;
                _771 = false;
                break;
            }
            break; 
        } while(false);
        bool _779;
        if (_771)
        {
            bool _778;
            if (_770 < _769)
            {
                _778 = _768 < _769;
            }
            else
            {
                _778 = false;
            }
            _779 = _778;
        }
        else
        {
            _779 = false;
        }
        if (_779)
        {
            _782 = _767;
            break;
        }
        _782 = _699;
        break;
    } while(false);
    float3 _784 = lerp(_571, _584, _782.xxx);
    float _785 = _784.x;
    float2 _790 = (float4(_785, _784.y, _228, 1.0f).xy * float2(0.5f, -0.5f)) + 0.5f.xx;
    float4 _797 = mul(frame_constants_view_constants[0u].view_to_world, mul(frame_constants_view_constants[0u].sample_to_view, float4(_785, _784.y, half_depth_tex.SampleLevel(sampler_nnc, _790, 0.0f).x, 1.0f)));
    float3 _801 = _797.xyz / _797.w.xxx;
    float3 _807 = _801 - in_var_C1;
    float3 _813 = -_276;
    int2 _814 = int2(_248);
    float2 _1022;
    float _1023;
    do
    {
        float4 _822 = mul(frame_constants_view_constants[2u].world_to_sample, float4(in_var_C1, 1.0f));
        if (_822.z > 1.0f)
        {
            _1022 = 0.0f.xx;
            _1023 = 0.0f;
            break;
        }
        float2 _827 = _822.xy;
        float2 _828 = abs(_827);
        bool _830 = any(bool2(_828.x > 0.9833333492279052734375f.xx.x, _828.y > 0.9833333492279052734375f.xx.y));
        float2 _834;
        if (_830)
        {
            _834 = _827 * 0.5f;
        }
        else
        {
            _834 = _827;
        }
        float _835 = _830 ? 5.0f : 10.0f;
        float _837 = _830 ? 0.5f : 1.0f;
        float2 _838 = abs(_834);
        float2 _841 = 1.0f.xx - (0.000833333353511989116668701171875f.xx * (2.0f * _835));
        bool _843 = any(bool2(_838.x > _841.x, _838.y > _841.y));
        float _849;
        float2 _850;
        float _851;
        if (_843)
        {
            _849 = _835 * 0.2857142984867095947265625f;
            _850 = _834 * 0.2857142984867095947265625f;
            _851 = _837 * 0.2857142984867095947265625f;
        }
        else
        {
            _849 = _835;
            _850 = _834;
            _851 = _837;
        }
        float _854 = float(_843 ? 2u : uint(_830));
        uint _856 = 2u + uint(_854);
        float _859 = _813.y;
        float4 _871 = float4(_850.x, _850.y, _822.z, _822.w) + mul(frame_constants_view_constants[_856].world_to_sample, float4((float3(0.0f, 1.0f, 0.0f) * (0.066666670143604278564453125f * clamp(1.0f - _859, 0.0f, 1.0f))) / _851.xxx, 0.0f));
        float2 _874 = (_871.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
        float3 _875 = float3(_874.x, _874.y, _871.z);
        float _880 = max((0.300000011920928955078125f / _851) * (1.0f - abs(_859)), 0.02500000037252902984619140625f);
        float _891 = float((uint((_814.x * 11) + (_814.y * 7)) + (frame_constants_frame_index * 5u)) % 64u) * 1.61803400516510009765625f;
        uint2 _892 = uint2(_814);
        float _904 = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(_892.x) + _497)) + (0.005837149918079376220703125f * (float(_892.y) + _497))));
        float4 _906 = mul(frame_constants_view_constants[_856].world_to_sample, float4(0.0f, 1.0f, 0.0f, 0.0f));
        float _908;
        int _911;
        _908 = 0.0f;
        _911 = 0;
        float _909;
        int _912;
        for (int _913 = 0; _913 < 5; _908 = _909, _911 = _912, _913++)
        {
            float _918 = float(_913);
            float _920 = (_918 + _891) * 2.3999631404876708984375f;
            float3 _933 = _875 + float3((((float2(cos(_920), sin(_920)) * (_918 + _904)) * 0.20000000298023223876953125f.xx) * _849) * 0.000833333353511989116668701171875f.xx, _880 * (-0.0009765625f));
            float4 _940 = shadow_map.SampleLevel(sampler_shadow_nearest, float3(_933.xy, _854), 0.0f);
            float _941 = _940.x;
            if (_933.z > _941)
            {
                _909 = _908 + _941;
                _912 = _911 + 1;
            }
            else
            {
                _909 = _908;
                _912 = _911;
            }
        }
        float _949 = _908 / float(_911);
        if (_911 == 0)
        {
            _1022 = 0.0f.xx;
            _1023 = 0.0f;
            break;
        }
        float _963 = max(((_871.z - _949) * ((17.0666675567626953125f * _849) * pow(10.0f, smoothstep(0.0f, 0.25f, frame_constants_fog_tint_thickness.w)))) / _949, 0.0f);
        float _970 = (_963 / pow(pow(_963 / (_849 - 1.0f), 5.0f) + 1.0f, 0.20000000298023223876953125f)) + 1.0f;
        float _972 = _906.z;
        float2 _982 = (((_906.xy * float(int(sign(_972)))) / max(0.1500000059604644775390625f, abs(_972)).xx) * 0.0009765625f.xx) * 2.0f;
        float2 _984;
        _984 = 0.0f.xx;
        for (int _987 = 0; _987 < 10; )
        {
            float _991 = float(_987);
            float _993 = (_991 + _891) * 2.3999631404876708984375f;
            float2 _1000 = ((float2(cos(_993), sin(_993)) * (_991 + _904)) * 0.100000001490116119384765625f.xx) * _970;
            float3 _1006 = _875 + float3(_1000 * 0.000833333353511989116668701171875f.xx, _880 * (-0.0009765625f));
            float _1018 = 1.0f - shadow_map.SampleCmpLevelZero(sampler_shadow_linear, float3(_1006.xy, _854), _1006.z + min(0.0f, dot(_982, _1000)));
            _984 += float2(_1018, _1018 * _1018);
            _987++;
            continue;
        }
        _1022 = _984 * 0.100000001490116119384765625f;
        _1023 = _970;
        break;
    } while(false);
    float _1027 = clamp((_1022.x - 0.100000001490116119384765625f) * 1.4285714626312255859375f, 0.0f, 1.0f);
    float2 _1040 = _1027.xx + ((float2(-1.0f, 1.0f) * sqrt(max(0.0f, _1022.y - (_1022.x * _1022.x)))) * (3.0f * max(0.0f, 1.0f - (_1023 * 0.20000000298023223876953125f))));
    float2 _1046 = (float2(_814) + 0.5f.xx) / float2(framebuffer_dims);
    float4 _1055 = float4((_1046 - 0.5f.xx) * float2(2.0f, -2.0f), gl_FragCoord.z, 1.0f);
    float4 _1058 = mul(frame_constants_view_constants[0u].clip_to_prev_clip, mul(frame_constants_view_constants[0u].view_to_clip, mul(_603, _1055)));
    float2 _1064 = ((_1058.xy / _1058.w.xx) * float2(0.5f, -0.5f)) + 0.5f.xx;
    float2 _1069 = clamp(_1064, 0.0f.xx, 1.0f.xx);
    float4 _1082 = float4(_1064 - ((_1055.xy * float2(0.5f, -0.5f)) + 0.5f.xx), float(all(bool2(_1069.x == _1064.x, _1069.y == _1064.y))), 1.0f) * float4(1.0f, 1.0f, clamp(1.0f - length(frame_constants_view_constants[0u].clip_to_prev_clip[3].xyz), 0.0f, 1.0f), 1.0f);
    float _1085 = _1082.z;
    bool _1099;
    if (_1085 > 0.0f)
    {
        bool _1098;
        if (history_valid != 0u)
        {
            _1098 = !(0u == frame_constants_frame_index);
        }
        else
        {
            _1098 = false;
        }
        _1099 = _1098;
    }
    else
    {
        _1099 = false;
    }
    float _1107;
    if (_1099)
    {
        _1107 = shadows_history_tex.SampleLevel(sampler_lnc, _1046 + _1082.xy, 0.0f).x;
    }
    else
    {
        _1107 = _1027;
    }
    float _1112 = lerp(clamp(_1107, _1040.x, _1040.y), _1027, lerp(1.0f, 0.125f, _1085));
    float _1113 = 1.0f - _1112;
    float4 _1121 = mul(frame_constants_view_constants[0u].view_to_sample, mul(frame_constants_view_constants[0u].world_to_view, float4(in_var_C1 + (_276 * 0.5f), 1.0f)));
    float3 _1125 = _1121.xyz / _1121.w.xxx;
    float4 _1136 = _269 + (float4(_1125 - _519, 0.0f) * float(int(sign(_1125.z))));
    float _1137 = _1136.w;
    float3 _1146 = (_1136 / (float((_1137 >= 0.0f) ? 1 : (-1)) * max(1.0000000133514319600180897396058e-10f, abs(_1137))).xxxx).xyz;
    float3 _1147 = _1146 - _519;
    bool3 _1148 = bool3(_1147.x < 0.0f.xxx.x, _1147.y < 0.0f.xxx.y, _1147.z < 0.0f.xxx.z);
    float3 _1151 = (float3(_1148.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, _1148.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, _1148.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - _519) / _1147;
    float3 _1157 = _519 + (_1147 * max(0.0f, max(_1151.x, _1151.y)));
    float3 _1158 = _1146 - _1157;
    bool3 _1159 = bool3(_1158.x >= 0.0f.xxx.x, _1158.y >= 0.0f.xxx.y, _1158.z >= 0.0f.xxx.z);
    float3 _1162 = (float3(_1159.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, _1159.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, _1159.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - _1157) / _1158;
    float3 _1170 = _1157 + (_1158 * min(1.0f, min(min(_1162.x, _1162.y), _1162.z)));
    bool _1313;
    float _1314;
    float _1315;
    do
    {
        float _1188 = 0.5f * _604;
        uint _1189 = uint(int(max(2u, min(3u, uint(int(floor(length((((_1170.xy * float2(0.5f, -0.5f)) + 0.5f.xx) - ((_1157.xy * float2(0.5f, -0.5f)) + 0.5f.xx)) * _253))))))));
        float _1293;
        float _1297;
        float _1298;
        float _1299;
        float _1300;
        bool _1301;
        do
        {
            float3 _1194 = _1170 - _1157;
            float _1202;
            if (_1170.z > 9.5367431640625e-07f)
            {
                _1202 = float(_1189);
            }
            else
            {
                _1202 = max(1.0f, float(_1189) - 1.0f);
            }
            float _1203 = 1.0f / _1202;
            float _1288;
            float _1289;
            float _1290;
            float _1291;
            bool _1292;
            if (_1189 > 0u)
            {
                float _1209 = lerp(0.0f, 1.0f, pow(_506 * _1203, 1.0f));
                float3 _1211 = _1157 + (_1194 * _1209);
                float2 _1214 = (_1211.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                float _1216 = 1.0f / _1211.z;
                float _1221 = 1.0f / half_depth_tex.SampleLevel(sampler_llc, _1214, 0.0f).x;
                float _1225 = 1.0f / half_depth_tex.SampleLevel(sampler_nnc, _1214, 0.0f).x;
                float _1229 = (max(_1221, _1225) * 1.00000202655792236328125f) - _1216;
                float _1230 = _1216 - min(_1221, _1225);
                bool _1233 = (_1229 < 0.0f) ? (_1230 < _1188) : false;
                float _1282;
                float _1283;
                float _1284;
                float _1285;
                bool _1286;
                float _1287;
                if (_1233)
                {
                    _1282 = _1229;
                    _1283 = _1230;
                    _1284 = _1188;
                    _1285 = _1209;
                    _1286 = _1233;
                    _1287 = 0.0f;
                }
                else
                {
                    float _1243;
                    _1243 = _1209;
                    bool _1242;
                    float _1244;
                    float _1277;
                    float _1278;
                    float _1279;
                    float _1280;
                    bool _1281;
                    uint _1238 = 1u;
                    bool _1241 = _1233;
                    for (;;)
                    {
                        if (_1238 < _1189)
                        {
                            _1244 = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(_1238) + _506) * _1203, 1.0f)));
                            float3 _1254 = _1157 + (_1194 * _1244);
                            float2 _1257 = (_1254.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                            float _1259 = 1.0f / _1254.z;
                            float _1263 = 1.0f / half_depth_tex.SampleLevel(sampler_llc, _1257, 0.0f).x;
                            float _1267 = 1.0f / half_depth_tex.SampleLevel(sampler_nnc, _1257, 0.0f).x;
                            float _1271 = (max(_1263, _1267) * 1.00000202655792236328125f) - _1259;
                            float _1272 = _1259 - min(_1263, _1267);
                            _1242 = (_1271 < 0.0f) ? (_1272 < _1188) : false;
                            if (_1242)
                            {
                                _1277 = _1271;
                                _1278 = _1272;
                                _1279 = _1188;
                                _1280 = _1244;
                                _1281 = _1242;
                                break;
                            }
                            _1238++;
                            _1241 = _1242;
                            _1243 = _1244;
                            continue;
                        }
                        else
                        {
                            _1277 = 0.0f;
                            _1278 = 0.0f;
                            _1279 = 0.0f;
                            _1280 = 1.0f;
                            _1281 = _1241;
                            break;
                        }
                    }
                    _1282 = _1277;
                    _1283 = _1278;
                    _1284 = _1279;
                    _1285 = _1280;
                    _1286 = _1281;
                    _1287 = _1243;
                }
                _1288 = _1282;
                _1289 = _1283;
                _1290 = _1284;
                _1291 = _1285;
                _1292 = _1286;
                _1293 = _1287;
            }
            else
            {
                _1288 = 0.0f;
                _1289 = 0.0f;
                _1290 = 0.0f;
                _1291 = 1.0f;
                _1292 = false;
                _1293 = 0.0f;
            }
            if (_1292)
            {
                _1297 = _1291;
                _1298 = _1288;
                _1299 = _1290;
                _1300 = _1289;
                _1301 = true;
                break;
            }
            else
            {
                _1297 = _1293;
                _1298 = _195;
                _1299 = _195;
                _1300 = _195;
                _1301 = false;
                break;
            }
            break; 
        } while(false);
        bool _1309;
        if (_1301)
        {
            bool _1308;
            if (_1300 < _1299)
            {
                _1308 = _1298 < _1299;
            }
            else
            {
                _1308 = false;
            }
            _1309 = _1308;
        }
        else
        {
            _1309 = false;
        }
        if (_1309)
        {
            _1313 = true;
            _1314 = _1297;
            _1315 = _1300 / _1299;
            break;
        }
        _1313 = false;
        _1314 = _1293;
        _1315 = 0.0f;
        break;
    } while(false);
    float _1323;
    if (_1313)
    {
        _1323 = min(_1113, smoothstep(0.5f, 0.0f, (1.0f - _1315) * (1.0f - _1314)));
    }
    else
    {
        _1323 = _1113;
    }
    uint _1334;
    if (_244)
    {
        _1334 = 0u;
    }
    else
    {
        uint2 _1327 = _248 & uint2(1u, 1u);
        _1334 = 1u << ((_1327.x + (_1327.y * 2u)) & 31u);
    }
    uint _1336 = _1334 + QuadReadAcrossX(_1334);
    uint2 _1339 = _248 & uint2(1u, 1u);
    if ((_1339.x + (_1339.y * 2u)) == firstbithigh(_1336 + QuadReadAcrossY(_1336)))
    {
        uint _1349;
        __alias_ice_refl_count_buffer.InterlockedAdd(12, 1u, _1349);
        __alias_ice_refl_px_buffer.Store<uint>(_1349 * 4 + 0, _249.x + (_249.y * 65536u));
        if ((_1349 & 63u) == 0u)
        {
            uint _1360;
            __alias_ice_refl_count_buffer.InterlockedAdd(0, 1u, _1360);
        }
    }
    float _1363;
    float3 _1415;
    do
    {
        _1363 = _275.y;
        bool _1369;
        if (!(_1363 <= 0.0f))
        {
            _1369 = _489 <= 0.0f;
        }
        else
        {
            _1369 = true;
        }
        if (_1369)
        {
            _1415 = 0.0f.xxx;
            break;
        }
        float _1372 = _410 * _410;
        float3 _1374 = normalize(_272 + _276);
        float _1375 = _1374.y;
        float _1378 = max(0.0f, 1.0f - dot(_1374, _276));
        float _1379 = _1378 * _1378;
        float _1406 = ((_1375 * _1375) * (_1372 - 1.0f)) + 1.0f;
        _1415 = ((lerp(0.02999999932944774627685546875f.xxx, 1.0f.xxx, ((_1379 * _1379) * _1378).xxx) * (((2.0f * _1363) * _489) / ((_1363 * sqrt(((((-_489) * _1372) + _489) * _489) + _1372)) + (_489 * sqrt(((((-_1363) * _1372) + _1363) * _1363) + _1372))))) * (_1372 / ((3.1415927410125732421875f * _1406) * _1406))) / ((4.0f * _489) * _1363).xxx;
        break;
    } while(false);
    float3 _1440 = mul(_807 * float3(1.0f, -1.0f, 1.0f), float3x3(float3(1.0f, -0.0f, 0.0f), float3(-0.0f, 0.0f, 1.0f), float3(-0.0f, -1.0f, 0.0f)));
    float _1442 = length(_1440);
    float3 _1444 = _1440 * (1.0f / _1442);
    float3 _1465 = clamp((exp(float3(-0.5950000286102294921875f, -0.374000012874603271484375f, -0.3060000240802764892578125f) * ((abs(in_var_C1.y - _801.y) + length(_807)) + 0.699999988079071044921875f)) * (0.959999978542327880859375f - (0.959999978542327880859375f * ((_491 * _491) * _490)))) * refraction_tex.SampleLevel(sampler_lnc, _790, 0.0f).xyz, 0.0f.xxx, 65408.0f.xxx);
    float _1475 = asfloat((asuint(max(max(1.52587890625e-05f, _1465.x), max(_1465.y, _1465.z))) + 125845504u) & 2139095040u);
    uint3 _1478 = asuint(_1465 + _1475.xxx);
    out_var_SV_TARGET0 = lerp((frame_constants_sun_color.xyz * _1415.x) * (_1323 * clamp(_1363, 0.0f, 1.0f)), ((frame_constants_sun_color.xyz * (0.699999988079071044921875f * _1323)) * 0.3183098733425140380859375f.xxx) + frame_constants_sky_high_color.xyz, 0.00999999977648258209228515625f.xxx);
    out_var_SV_TARGET1 = sqrt(_410);
    out_var_SV_TARGET2 = ((uint((clamp((_1444.x * 0.5f) + 0.5f, 0.0f, 1.0f) * 255.0f) + 0.5f) << 24u) | (uint((clamp((_1444.y * 0.5f) + 0.5f, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) | spvPackHalf2x16(float2(_1442, 0.0f));
    out_var_SV_TARGET3 = ((((asuint(_1475) << 4u) + 268435456u) | (_1478.z << 18u)) | (_1478.y << 9u)) | (_1478.x & 511u);
    out_var_SV_TARGET4 = _1112 + 0.00196078442968428134918212890625f;
}

[earlydepthstencil]
SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    gl_FragCoord = stage_input.gl_FragCoord;
    gl_FragCoord.w = 1.0 / gl_FragCoord.w;
    in_var_C1 = stage_input.in_var_C1;
    frag_main();
    SPIRV_Cross_Output stage_output;
    stage_output.out_var_SV_TARGET0 = out_var_SV_TARGET0;
    stage_output.out_var_SV_TARGET1 = out_var_SV_TARGET1;
    stage_output.out_var_SV_TARGET2 = out_var_SV_TARGET2;
    stage_output.out_var_SV_TARGET3 = out_var_SV_TARGET3;
    stage_output.out_var_SV_TARGET4 = out_var_SV_TARGET4;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif
