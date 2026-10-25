#define SHADER_VARIANT_DEPTH_ONLY 0
#line 1 "shaders/ice_plane.raster.hlsl"
















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
#line 17 "shaders/ice_plane.raster.hlsl"

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
#line 18 "shaders/ice_plane.raster.hlsl"

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
#line 19 "shaders/ice_plane.raster.hlsl"

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
#line 20 "shaders/ice_plane.raster.hlsl"

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
#line 21 "shaders/ice_plane.raster.hlsl"

#line 1 "shaders/inc/consts.hlsl"






#ifndef TINYGLADE_INC_CONSTS_HLSL
#define TINYGLADE_INC_CONSTS_HLSL


static const float RENDER_SCALE = 1.0f;

#endif 
#line 22 "shaders/ice_plane.raster.hlsl"

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





cbuffer color_library : register(b14, space0) {
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
#line 23 "shaders/ice_plane.raster.hlsl"

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





cbuffer color_library : register(b14, space0) {
    ColorLibrary color_library;
};

#endif 
#line 24 "shaders/ice_plane.raster.hlsl"

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
#line 25 "shaders/ice_plane.raster.hlsl"

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
#line 26 "shaders/ice_plane.raster.hlsl"

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
#line 27 "shaders/ice_plane.raster.hlsl"

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
#line 28 "shaders/ice_plane.raster.hlsl"

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
#line 29 "shaders/ice_plane.raster.hlsl"

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
#line 30 "shaders/ice_plane.raster.hlsl"

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
#line 31 "shaders/ice_plane.raster.hlsl"

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
#line 32 "shaders/ice_plane.raster.hlsl"

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





cbuffer color_library : register(b14, space0) {
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
#line 33 "shaders/ice_plane.raster.hlsl"

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
#line 34 "shaders/ice_plane.raster.hlsl"

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
#line 35 "shaders/ice_plane.raster.hlsl"

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
#line 36 "shaders/ice_plane.raster.hlsl"

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
#line 37 "shaders/ice_plane.raster.hlsl"

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
#line 38 "shaders/ice_plane.raster.hlsl"



#line 1 "shaders/inc/blue_noise.hlsl"

















float4 blue_noise(Texture2D<float4> blue_noise_texture, uint2 pixel_coords, uint frame_index)
{
    uint2 offset = uint2(frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f)
        * float(frame_index)) + 0.5f.xx) * 256.0f);
    uint2 p = (pixel_coords + uint2(13u, 13u) + offset) % uint2(256u, 256u);
    float4 texel = blue_noise_texture.Load(int3(p, 0));
    return texel * 255.0f * (1.0f / 256.0f) + (1.0f / 512.0f);
}
#line 41 "shaders/ice_plane.raster.hlsl"

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
#line 42 "shaders/ice_plane.raster.hlsl"

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
#line 43 "shaders/ice_plane.raster.hlsl"


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
    float3 Vertex_Position : TEXCOORD0;
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
