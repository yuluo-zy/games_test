#define TGV_0 1
#line 1 "shaders/generate_grass.cs.hlsl"


#if !((defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)))


















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
#line 22 "shaders/generate_grass.cs.hlsl"

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
#line 23 "shaders/generate_grass.cs.hlsl"

#line 1 "shaders/inc/consts.hlsl"






#ifndef TINYGLADE_INC_CONSTS_HLSL
#define TINYGLADE_INC_CONSTS_HLSL


static const float RENDER_SCALE = 1.0f;

#endif 
#line 24 "shaders/generate_grass.cs.hlsl"

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
#line 25 "shaders/generate_grass.cs.hlsl"

#line 1 "shaders/inc/gaussian_rand.hlsl"












float gaussian_rand(uint seed_a, uint seed_b)
{
    float u = (rand01(hash(seed_b)) * 2.0f) - 1.0f;
    float L = log(1.0f - u * u);
    float c = 4.5472841f + 0.5f * L;
    float r = (rand01(hash(seed_a)) * 1.5f) + 0.5f;
    return r + (sqrt(sqrt((c * c) - (L * 7.142857f)) - c) * sign(u)) * 0.3f;
}
#line 26 "shaders/generate_grass.cs.hlsl"

#line 1 "shaders/inc/simplex_noise.hlsl"











float incb_mod289(float x)
{
    return fmod(x, 289.0f);
}

float3 incb_mod289(float3 x)
{
    return fmod(x, 289.0f);
}

float incb_permute(float x)
{
    return fmod((x * 34.0f + 1.0f) * x, 289.0f);
}

float3 incb_permute(float3 x)
{
    return fmod((x * 34.0f + 1.0f) * x, 289.0f);
}

float snoise(float2 v)
{
    const float2 C = float2(0.211324865405187f, 0.366025403784439f); 
    const float2 Cz = float2(-0.577350269189626f, -0.577350269189626f); 

    float2 i = floor(v + dot(v, C.yy));
    float2 x0 = v - i + dot(i, C.xx);

    float2 g = step(x0.yx, x0.xy);
    float2 l = 1.0f - g;
    float2 i1 = min(g.xy, l.yx);
    float2 i2 = max(g.xy, l.yx);

    float2 x1 = x0 - i1 + C.xx;
    float2 x2 = x0 - i2 + Cz;

    float i4x = incb_mod289(i.x);
    float i4y = incb_mod289(i.y);

    float3 p0 = incb_permute(incb_permute(i4y + float3(0.0f, i1.y, 1.0f)) + i4x + float3(0.0f, i1.x, 1.0f));

    float3 m = max(0.5f - float3(dot(x0, x0), dot(x1, x1), dot(x2, x2)), 0.0f);
    m = m * m;
    m = m * m;

    float3 n = frac(incb_permute(p0) * (1.0f / 41.0f)) * 2.0f;
    n = n - 1.0f;
    float3 s = abs(n) - 0.5f;
    float3 t = n - floor(n + 0.5f);

    float3 grad = float3(t.x * x0.x + s.x * x0.y,
                         t.y * x1.x + s.y * x1.y,
                         t.z * x2.x + s.z * x2.y);

    return 130.0f * dot(m * (1.79284291400159f - (dot(t, t) + dot(s, s)) * 0.85373472095314f), grad);
}
#line 27 "shaders/generate_grass.cs.hlsl"






static int scatter_grid_dim;
#line 1 "shaders/inc/scatter.hlsl"











float2 scatter_point(uint index)
{
    int grid = scatter_grid_dim;
    int area = grid * grid;
    uint cell = index % uint(area);
    float g = float(grid);
    float2 grid_pos = (float2(float(cell % uint(grid)), float(cell / uint(grid))) / g - 0.5f) * 104.0f;
    float2 jitter = float2(rand01(hash(index)),
                           rand01(hash(index ^ 188053769u))) * 6.3f;
    return grid_pos + jitter;
}



float2 scatter_cell_point(int2 cell, int seed)
{
    float2 p = float2(cell) * 2.5f;
    p += (frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f)
        * float(uint(seed) & 65535u)) + 0.5f) - 0.5f) * 2.5f;
    return p;
}
#line 34 "shaders/generate_grass.cs.hlsl"





#define TINYGLADE_INC_COLOR_LIBRARY_HLSL
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
#line 40 "shaders/generate_grass.cs.hlsl"

cbuffer color_library : register(b13, space0)
{
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
#line 81 "shaders/generate_grass.cs.hlsl"

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
#line 82 "shaders/generate_grass.cs.hlsl"

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
#line 83 "shaders/generate_grass.cs.hlsl"

#line 1 "shaders/inc/grass_blade_data.hlsl"


#ifndef TINYGLADE_INC_GRASS_BLADE_DATA_HLSL
#define TINYGLADE_INC_GRASS_BLADE_DATA_HLSL



struct GrassBladeDataPacked {
    float3 position; 
    uint color;      
    uint packed0;    
    uint packed1;    
    uint packed2;    
    uint pad;
};


float3 grass_blade_color(GrassBladeDataPacked b)
{
    return float3(float(b.color & 255u) * (1.0 / 255.0),
                  float((b.color >> 8u) & 255u) * (1.0 / 255.0),
                  float((b.color >> 16u) & 255u) * (1.0 / 255.0));
}


float2 grass_blade_widths(GrassBladeDataPacked b)
{
    return float2(f16tof32(b.packed1 & 0xffffu), f16tof32((b.packed1 >> 16u) & 0xffffu));
}


float grass_blade_bend_xz(GrassBladeDataPacked b)
{
    return f16tof32((b.packed2 >> 16u) & 0xffffu);
}


float grass_blade_dir_x(GrassBladeDataPacked b)
{
    return float((b.packed0 >> 24u) & 255u) * (2.0 / 255.0) - 1.0;
}

#endif 
#line 84 "shaders/generate_grass.cs.hlsl"

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
#line 85 "shaders/generate_grass.cs.hlsl"

#line 1 "shaders/inc/atomics.hlsl"




#ifndef TINYGLADE_INC_ATOMICS_HLSL
#define TINYGLADE_INC_ATOMICS_HLSL


uint atomic_append(RWByteAddressBuffer counter_buffer, uint counter_byte_offset)
{
    uint previous;
    counter_buffer.InterlockedAdd(counter_byte_offset, 1u, previous);
    return previous;
}


uint atomic_add(RWByteAddressBuffer buffer, uint byte_offset, uint value)
{
    uint previous;
    buffer.InterlockedAdd(byte_offset, value, previous);
    return previous;
}

#endif 
#line 86 "shaders/generate_grass.cs.hlsl"

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
#line 87 "shaders/generate_grass.cs.hlsl"

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
#line 88 "shaders/generate_grass.cs.hlsl"

#line 1 "shaders/inc/fbm_noise.hlsl"






#line 7 "shaders/inc/fbm_noise.hlsl"


float fbm(float2 p)
{
    const float2x2 rot = float2x2(0.8f, -0.6f, 0.6f, 0.8f);
    float value = 0.0f;
    float amp = 0.5f;
    float2 pos = p;
    [unroll]
    for (int i = 0; i < 2; i++)
    {
        value += amp * (-1.0f + 2.0f * perlin(pos));
        amp *= 0.55f;
        pos = mul(rot, pos * 1.9f);
    }
    return value;
}
#line 89 "shaders/generate_grass.cs.hlsl"

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
#line 90 "shaders/generate_grass.cs.hlsl"

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
#line 91 "shaders/generate_grass.cs.hlsl"

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
#line 92 "shaders/generate_grass.cs.hlsl"

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
#line 93 "shaders/generate_grass.cs.hlsl"

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
#line 94 "shaders/generate_grass.cs.hlsl"


Texture2D<float4> terrain_heightmap : register(t17, space0);
Texture2D<float4> bg_terrain_heightmap : register(t16, space0);
Texture2D<float4> path_mask : register(t19, space0);
Texture2D<float4> grass_object_push : register(t20, space0);
Texture2D<float4> rocky_terrain : register(t21, space0);
Texture2D<float4> water_mask : register(t22, space0);
Texture2D<float4> foliage_exclusion_mask : register(t23, space0);
Texture2D<float4> buildings_mask : register(t24, space0);
Texture2D<float4> garden_mask : register(t25, space0);
StructuredBuffer<GrassTilePacked> tile_buffer : register(t26, space0);
StructuredBuffer<GrassBundlePacked> bundle_buffer : register(t27, space0);
StructuredBuffer<uint> grasslet_bundle_ranges : register(t28, space0);
RWStructuredBuffer<GrassBladeDataPacked> __alias_grass_blade_buffer : register(u29, space0);
RWStructuredBuffer<DrawIndirectCommand> __alias_lod1_commands : register(u30, space0);
RWStructuredBuffer<uint> __alias_atomics : register(u31, space0);
RWStructuredBuffer<DrawIndexedIndirectCommand> __alias_draw_commands : register(u14, space0);

cbuffer $Globals : register(b15, space0)
{
    int draw_indirect_buffer_offset;
    uint2 bg_terrain_heightmap_dims;
    uint2 terrain_heightmap_dims;
    uint2 visual_terrain_heightmap_dims;
    uint2 path_mask_dims;
    uint2 grass_object_push_dims;
    uint2 rocky_terrain_dims;
    uint2 water_mask_dims;
    uint2 foliage_exclusion_mask_dims;
    uint2 buildings_mask_dims;
    uint2 garden_mask_dims;
    uint lod1_commands_offset;
    uint grass_buffer_size;
    int brush_index;
};


static const float2 GRASS_GOLDEN_JITTER = float2(0.75487768650054931640625f, 0.56984031200408935546875f);


uint pack_half2(float2 v)
{
    uint2 h = f32tof16(v);
    return h.x | (h.y << 16u);
}

groupshared GrassBundlePacked group_bundles[64];

[numthreads(64, 1, 1)]
void cs_main(uint3 group_id : SV_GroupID, uint local_index : SV_GroupIndex)
{
    do
    {
        uint range = grasslet_bundle_ranges[group_id.x];
        uint bundle_count = range % 128u;
        uint bundle_start = range / 128u;
        if (local_index < bundle_count)
        {
            group_bundles[local_index] = bundle_buffer[bundle_start + local_index];
        }
        GroupMemoryBarrierWithGroupSync();

        
        uint2 fallback_xy = uint2(0u, 0u);
        uint grasslet = local_index;
        uint bundle_index = 0u;
        uint2 chosen_xy;
        uint chosen_grasslet;
        for (;;)
        {
            if (bundle_index < bundle_count)
            {
                uint bxy = group_bundles[bundle_index].raw;
                uint2 b_xy = uint2((bxy >> 24u) & 255u, (bxy >> 16u) & 255u);
                uint bundle_blades = bxy & 0xFFFFu;
                uint blade_count = bundle_blades % 128u;
                if (grasslet < blade_count)
                {
                    chosen_grasslet = (bundle_blades / 128u) + grasslet;
                    chosen_xy = b_xy;
                    break;
                }
                grasslet -= blade_count;
                bundle_index++;
                fallback_xy = b_xy;
                continue;
            }
            else
            {
                chosen_grasslet = 0u;
                chosen_xy = fallback_xy;
                break;
            }
        }
        uint tile_index = chosen_xy.x + (chosen_xy.y * 128u);

        GrassTilePacked tile = tile_buffer[tile_index];
        float tile_blades_f = float((tile.raw & 0xFFFFu) % 512u);

        
        uint hs = ((chosen_grasslet * 17u) % 256u) + 1u;
        float3 scatter_pos = frac(float2(float(hs) * (1.0f / 256.0f), radicalInverse_VdC(hs))
            + frac(GRASS_GOLDEN_JITTER * float((tile_index * 137u) & 65535u) + 0.5f)).xyy;
        float2 tile_center_ws = (scatter_pos.xz + (float2(chosen_xy) - 64.0f)) * (1.0f / 128.0f) * 260.0f;
        float3 blade_pos = float3(tile_center_ws.x, scatter_pos.y, tile_center_ws.y);
        blade_pos.y = 0.0f;

        
        int2 base_cell = int2(blade_pos.xz * 0.4f);
        float3 nearest = 0.0f.xxx;
        int nearest_seed = 0;
        float nearest_dist = 100000.0f;
        for (int cx = base_cell.x - 1; cx <= (base_cell.x + 1); cx++)
        {
            for (int cy = base_cell.y - 1; cy <= (base_cell.y + 1); cy++)
            {
                int seed = cx + (cy * 577);
                float2 cell_point = scatter_cell_point(int2(cx, cy), seed);
                float3 cp = float3(cell_point.x, 0.0f, cell_point.y);
                float d = distance(cp, blade_pos);
                if (d < nearest_dist)
                {
                    nearest_dist = d;
                    nearest = cp;
                    nearest_seed = seed;
                }
            }
        }

        uint blade_seed_full = hash((tile_index * 256u) + chosen_grasslet);
        uint blade_seed = blade_seed_full % 65536u;
        bool theme4 = (glade_theme_id == 4u);
        if (theme4)
        {
            
            if (snoise(blade_pos.xz * 0.2f) > 0.5f)
            {
                break;
            }
        }

        
        
        float gaussian = gaussian_rand(uint(nearest_seed * 13), blade_seed);
        bool spread_bool = rand01(hash(uint(nearest_seed * 2))) < 0.7f;
        float2 spread_dir;
        if (spread_bool)
        {
            float angle = (1.0f - rand01(hash(uint(nearest_seed)))) * 6.283185482025146484375f;
            spread_dir = normalize(float2(cos(angle), sin(angle)));
        }
        else
        {
            spread_dir = normalize(blade_pos - nearest).xz;
        }
        float2 dir2 = normalize(float2(cos((1.0f - rand01(hash(blade_seed + 69u))) * 6.283185482025146484375f),
                                      sin((1.0f - rand01(hash(blade_seed + 69u))) * 6.283185482025146484375f)));
        float slerp_t = (rand01(hash(blade_seed + 34u)) * 0.2f) + 0.5f;
        float2 spread;
        float ddot = dot(normalize(spread_dir), normalize(dir2));
        if ((ddot > 0.99989998340606689453125f) || (ddot < (-0.99989998340606689453125f)))
        {
            spread = (slerp_t <= 0.5f) ? spread_dir : dir2;
        }
        else
        {
            float a = acos(ddot);
            spread = ((spread_dir * sin((0.5f - slerp_t) * a)) + (dir2 * sin(slerp_t * a))) / sin(a);
        }

        
        
        float3 ground_pos = lerp(blade_pos, nearest, (rand01(hash(uint(nearest_seed + 4235))) - (rand01(hash(blade_seed + 15u)) * 0.9f)));
        float ground_height;
        if ((abs(ground_pos.x) < 65.0f) && (abs(ground_pos.z) < 65.0f))
        {
            ground_height = lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (ground_pos.xz * (1.0f / 130.0f)) + 0.5f, 0.0f).x);
        }
        else
        {
            float2 bg_uv = (ground_pos.xz * 0.001f) + 0.5f;
            ground_height = lerp(-30.1061992645263671875f, 27.7313995361328125f, bg_terrain_heightmap.SampleLevel(sampler_default, float2(bg_uv.y, 1.0f - bg_uv.x), 0.0f).x);
        }
        ground_pos.y = ground_height;

        float2 mask_uv = (ground_pos.xz * (1.0f / 130.0f)) + 0.5f;
        if (foliage_exclusion_mask.SampleLevel(sampler_llb, mask_uv, 0.0f).x > 0.0f)
        {
            break;
        }
        float path_factor = smoothstep(0.1f, 0.2f, path_mask.SampleLevel(sampler_llb, mask_uv, 0.0f).x);
        if (path_factor > 0.9f)
        {
            break;
        }
        bool in_bounds = !((mask_uv.x < 0.0f) || (mask_uv.x > 1.0f) || (mask_uv.y < 0.0f) || (mask_uv.y > 1.0f));
        float rocky_r = in_bounds ? rocky_terrain.SampleLevel(sampler_default, mask_uv, 0.0f).y : 0.0f;
        float rocky_g = in_bounds ? rocky_terrain.SampleLevel(sampler_default, mask_uv, 0.0f).z : 0.0f;
        float rocky_b = in_bounds ? rocky_terrain.SampleLevel(sampler_default, mask_uv, 0.0f).x : 0.0f;
        if ((rocky_b + smoothstep(0.0f, 0.5f, rocky_r)) + (rocky_g * 0.4f) > 0.2f)
        {
            break;
        }
        float water = in_bounds ? water_mask.SampleLevel(sampler_default, mask_uv, 0.0f).y : 0.0f;
        if (water > 0.25f)
        {
            break;
        }
        if (smoothstep(0.35f, 0.4f, garden_mask.SampleLevel(sampler_llb, mask_uv, 0.0f).y) > 0.0f)
        {
            break;
        }

        float width = (((rand01(hash(blade_seed + 45u)) * 1.5f) + 0.5f) * (spread_bool ? 1.0f : 0.5f)) * (1.0f - max(path_factor, 0.0f));
        float bend = rand01(hash(uint(nearest_seed * 2 - 334))) * 0.3f;
        float scale;
        if (in_bounds)
        {
            float building = buildings_mask.SampleLevel(sampler_default, mask_uv, 0.0f).y;
            scale = gaussian * (lerp(1.0f, 0.0f, smoothstep(0.3f, 0.4f, building)) + (smoothstep(0.5f, 0.7f, building) * 0.4f));
        }
        else
        {
            scale = gaussian;
        }
        float width_scale = theme4 ? (width * 2.0f) : width;
        if (scale < 0.01f)
        {
            break;
        }

        
        float3 color_low;
        float3 color_high;
        if (glade_theme_id == 2u)
        {
            color_low = float3(0.314999997615814208984375f, 0.2700000107288360595703125f, 0.1500000059604644775390625f);
            color_high = grass_1.xyz;
        }
        else if (glade_theme_id == 3u)
        {
            color_low = grass_2.xyz * 0.75f;
            color_high = grass_1.xyz * 0.75f;
        }
        else
        {
            color_low = grass_2.xyz;
            color_high = grass_1.xyz;
        }

        
        float noise1 = (perlin(ground_pos.xz * 0.4f) * 2.0f) - 1.0f;
        float noise2 = (perlin(ground_pos.xz * 0.2f) * 2.0f) - 1.0f;
        float4 grass_push = grass_object_push.SampleLevel(sampler_llb, mask_uv, 0.0f);
        float push_strength = clamp(grass_push.z, 0.0f, 1.0f);
        float2 final_spread;
        float bend_amount;
        if (push_strength > 0.0f)
        {
            float2 push_dir = normalize(grass_push.xy);
            float d2 = dot(normalize(spread), normalize(push_dir));
            float2 blended;
            if ((d2 > 0.99989998340606689453125f) || (d2 < (-0.99989998340606689453125f)))
            {
                blended = (push_strength <= 0.5f) ? spread : push_dir;
            }
            else
            {
                float a2 = acos(d2);
                blended = ((spread * sin((1.0f - push_strength) * a2)) + (push_dir * sin(push_strength * a2))) / sin(a2);
            }
            bend_amount = lerp(bend, 0.9f, push_strength);
            final_spread = blended;
        }
        else
        {
            bend_amount = bend;
            final_spread = spread;
        }
        float rand_jitter = rand01(hash(blade_seed + 34u)) * 10.0f;
        float lod = smoothstep(50.0f - rand_jitter, 40.0f - rand_jitter,
            ((-1.0f) / mul(view_constants[0].world_to_view, float4(ground_pos.x, ground_height, ground_pos.z, 1.0f)).z)
            * ((0.5f * view_constants[0].view_to_clip[1].y) * float(clamp(inner_rendering_resolution.y, 1080u, 2160u))));

        
        uint blade_slot;
        if (0u == uint(floor(lod)))
        {
            uint unused;
            InterlockedAdd(__alias_draw_commands[uint(draw_indirect_buffer_offset)].instanceCount, 1u, unused);
            uint atomics_prev;
            InterlockedAdd(__alias_atomics[0], 1u, atomics_prev);
            blade_slot = atomics_prev;
        }
        else
        {
            uint unused;
            InterlockedAdd(__alias_lod1_commands[lod1_commands_offset].vertexCount, 3u, unused);
            uint atomics_prev;
            InterlockedAdd(__alias_atomics[1], 0xFFFFFFFFu, atomics_prev);
            blade_slot = atomics_prev - 1u;
        }

        float2 dir_norm = normalize(final_spread);
        float color_t = clamp((noise1 * 0.25f) + (noise2 * 0.35f) + (noise1 * 0.125f)
            + ((rand01(blade_seed_full) - 0.5f) * 0.5f) + 0.2f, 0.0f, 1.0f);
        float3 blade_color = sqrt(clamp(clamp(lerp(color_high, color_low, color_t) * 1.2f, 0.0f, 1.0f), 0.0f, 1.0f));

        GrassBladeDataPacked blade;
        blade.position = ground_pos;
        blade.color = pack_rgb8(blade_color);
        blade.packed0 = (uint(clamp(lod, 0.0f, 1.0f) * 255.0f + 0.5f)
            + (uint(clamp(smoothstep(tile_blades_f, tile_blades_f * 0.9f, float(chosen_grasslet)), 0.0f, 1.0f) * 255.0f + 0.5f) << 8u))
            + (uint(clamp(bend_amount, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
            + (uint(clamp(clamp((dir_norm.x * 0.5f) + 0.5f, 0.0f, 1.0f), 0.0f, 1.0f) * 255.0f + 0.5f) << 24u);
        blade.packed1 = pack_half2(float2(scale * (1.0f - (pow(path_factor, 1.0f) * 0.7f)), 0.0f))
            | (pack_half2(float2(width_scale, 0.0f)) << 16u);
        blade.packed2 = pack_half2(float2(2.0f, 0.0f)) | (pack_half2(float2(dir_norm.y, 0.0f)) << 16u);
        blade.pad = 0u;
        __alias_grass_blade_buffer[blade_slot] = blade;
        break;
    } while (false);
}

#endif

#if defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)

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

struct DrawIndexedIndirectCommand
{
    uint count;
    uint instanceCount;
    uint firstIndex;
    uint vertexOffset;
    uint firstInstance;
};

struct GrassTilePacked
{
    uint raw;
};

struct GrassBundlePacked
{
    uint raw;
};

struct GrassBladeDataPacked
{
    float3 position;
    uint color;
    uint packed0;
    uint packed1;
    uint packed2;
    uint pad;
};

struct DrawIndirectCommand
{
    uint vertexCount;
    uint instanceCount;
    uint firstVertex;
    uint firstInstance;
};

static float3 _272;

cbuffer color_library : register(b13, space0)
{
    float4 color_library_temp_color_1 : packoffset(c0);
    float4 color_library_temp_color_2 : packoffset(c1);
    float4 color_library_temp_color_3 : packoffset(c2);
    float4 color_library_roof_color_1 : packoffset(c3);
    float4 color_library_roof_color_2 : packoffset(c4);
    float4 color_library_sky_low_day : packoffset(c5);
    float4 color_library_sky_high_day : packoffset(c6);
    float4 color_library_sun_tint_day : packoffset(c7);
    float4 color_library_sky_low_dusk : packoffset(c8);
    float4 color_library_sky_high_dusk : packoffset(c9);
    float4 color_library_sun_tint_dusk : packoffset(c10);
    float4 color_library_indirect_light_tint : packoffset(c11);
    float4 color_library_fence_color_1 : packoffset(c12);
    float4 color_library_fence_color_2 : packoffset(c13);
    float4 color_library_flower_color_1 : packoffset(c14);
    float4 color_library_flower_color_2 : packoffset(c15);
    float4 color_library_canopy_color_1 : packoffset(c16);
    float4 color_library_canopy_color_2 : packoffset(c17);
    float4 color_library_grass_1 : packoffset(c18);
    float4 color_library_grass_2 : packoffset(c19);
    float4 color_library_path : packoffset(c20);
    float4 color_library_sky_base : packoffset(c21);
    float4 color_library_sky_horizon : packoffset(c22);
    float4 color_library_sky_clouds_1 : packoffset(c23);
    float4 color_library_sky_clouds_2 : packoffset(c24);
    float4 color_library_terrain_1 : packoffset(c25);
    float4 color_library_terrain_2 : packoffset(c26);
    float4 color_library_terrain_flattened : packoffset(c27);
    float4 color_library_wall_1 : packoffset(c28);
    float4 color_library_wall_2 : packoffset(c29);
    float4 color_library_wall_moss : packoffset(c30);
    float4 color_library_wall_bricks : packoffset(c31);
    float4 color_library_ivy_branch : packoffset(c32);
    float4 color_library_ivy_leaf_1 : packoffset(c33);
    float4 color_library_ivy_leaf_2 : packoffset(c34);
    float4 color_library_terrain_highlight : packoffset(c35);
    float4 color_library_night_tint : packoffset(c36);
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

RWByteAddressBuffer __alias_draw_commands : register(u14, space0);
cbuffer $Globals : register(b15, space0)
{
    int draw_indirect_buffer_offset : packoffset(c0);
    uint2 bg_terrain_heightmap_dims : packoffset(c0.y);
    uint2 terrain_heightmap_dims : packoffset(c1);
    uint2 visual_terrain_heightmap_dims : packoffset(c1.z);
    uint2 path_mask_dims : packoffset(c2);
    uint2 grass_object_push_dims : packoffset(c2.z);
    uint2 rocky_terrain_dims : packoffset(c3);
    uint2 water_mask_dims : packoffset(c3.z);
    uint2 foliage_exclusion_mask_dims : packoffset(c4);
    uint2 buildings_mask_dims : packoffset(c4.z);
    uint2 garden_mask_dims : packoffset(c5);
    uint lod1_commands_offset : packoffset(c5.z);
    uint grass_buffer_size : packoffset(c5.w);
    int brush_index : packoffset(c6);
};

ByteAddressBuffer tile_buffer : register(t26, space0);
ByteAddressBuffer bundle_buffer : register(t27, space0);
ByteAddressBuffer grasslet_bundle_ranges : register(t28, space0);
RWByteAddressBuffer __alias_grass_blade_buffer : register(u29, space0);
RWByteAddressBuffer __alias_lod1_commands : register(u30, space0);
RWByteAddressBuffer __alias_atomics : register(u31, space0);
SamplerState sampler_llb : register(s4, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> bg_terrain_heightmap : register(t16, space0);
Texture2D<float4> terrain_heightmap : register(t17, space0);
Texture2D<float4> path_mask : register(t19, space0);
Texture2D<float4> grass_object_push : register(t20, space0);
Texture2D<float4> rocky_terrain : register(t21, space0);
Texture2D<float4> water_mask : register(t22, space0);
Texture2D<float4> foliage_exclusion_mask : register(t23, space0);
Texture2D<float4> buildings_mask : register(t24, space0);
Texture2D<float4> garden_mask : register(t25, space0);

static uint3 gl_WorkGroupID;
static uint gl_LocalInvocationIndex;
struct SPIRV_Cross_Input
{
    uint3 gl_WorkGroupID : SV_GroupID;
    uint gl_LocalInvocationIndex : SV_GroupIndex;
};

groupshared GrassBundlePacked group_bundles[64];

uint spvPackHalf2x16(float2 value)
{
    uint2 Packed = f32tof16(value);
    return Packed.x | (Packed.y << 16);
}

float2 spvUnpackHalf2x16(uint value)
{
    return f16tof32(uint2(value & 0xffff, value >> 16));
}

void comp_main()
{
    do
    {
        uint _282 = grasslet_bundle_ranges.Load<uint>(gl_WorkGroupID.x * 4 + 0);
        uint _283 = _282 % 128u;
        if (gl_LocalInvocationIndex < _283)
        {
            GrassBundlePacked _290;
            _290.raw = bundle_buffer.Load<uint>(((_282 / 128u) + gl_LocalInvocationIndex) * 4 + 0);
            GrassBundlePacked _293 = { _290.raw };
            group_bundles[gl_LocalInvocationIndex] = _293;
        }
        GroupMemoryBarrierWithGroupSync();
        uint2 _300;
        _300 = uint2(0u, 0u);
        uint _296;
        uint2 _301;
        uint _319;
        uint2 _320;
        uint _295 = gl_LocalInvocationIndex;
        uint _298 = 0u;
        for (;;)
        {
            if (_298 < _283)
            {
                _301 = uint2((group_bundles[_298].raw >> 24u) & 255u, (group_bundles[_298].raw >> 16u) & 255u);
                uint _312 = group_bundles[_298].raw & 65535u;
                uint _314 = _312 % 128u;
                if (_295 < _314)
                {
                    _319 = (_312 / 128u) + _295;
                    _320 = _301;
                    break;
                }
                _296 = _295 - _314;
                _295 = _296;
                _298++;
                _300 = _301;
                continue;
            }
            else
            {
                _319 = 0u;
                _320 = _300;
                break;
            }
        }
        uint _324 = _320.x + (_320.y * 128u);
        GrassTilePacked _326;
        _326.raw = tile_buffer.Load<uint>(_324 * 4 + 0);
        float _330 = float((_326.raw & 65535u) % 512u);
        uint _336 = ((_319 * 17u) % 256u) + 1u;
        uint _341 = (_336 << 16u) | (_336 >> 16u);
        uint _346 = ((_341 & 1431655765u) << 1u) | ((_341 & 2863311530u) >> 1u);
        uint _351 = ((_346 & 858993459u) << 2u) | ((_346 & 3435973836u) >> 2u);
        uint _356 = ((_351 & 252645135u) << 4u) | ((_351 & 4042322160u) >> 4u);
        float3 _374 = frac(float2(float(_336) * 0.00390625f, float(((_356 & 16711935u) << 8u) | ((_356 & 4278255360u) >> 8u)) * 2.3283064365386962890625e-10f).xyy + frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float((_324 * 137u) & 65535u)) + 0.5f.xx).xyy);
        float2 _382 = ((_374.xz + (float2(_320) - 64.0f.xx)).xy * 0.0078125f.xx).xy * 260.0f;
        float3 _383 = float3(_382.x, _374.y, _382.y);
        uint _385 = (_324 * 256u) + _319;
        _383.y = 0.0f;
        int2 _389 = int2(_383.xz * 0.4000000059604644775390625f.xx);
        int _390 = _389.x;
        float3 _393;
        int _396;
        int _398;
        _393 = 0.0f.xxx;
        _396 = 0;
        _398 = _390 - 1;
        float3 _394;
        int _397;
        float _401;
        for (float _400 = 100000.0f; _398 <= (_390 + 1); _393 = _394, _396 = _397, _398++, _400 = _401)
        {
            int _406 = _389.y;
            int _407 = _406 - 1;
            _401 = _400;
            _394 = _393;
            _397 = _396;
            for (int _411 = _407; _411 <= (_406 + 1); )
            {
                int _418 = _411 * 577;
                int _419 = _398 + _418;
                float _421 = float(_411);
                float2 _433 = (float2(float(_398), _421) * 2.5f).xy + ((frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(uint(_419) & 65535u)) + 0.5f.xx) - 0.5f.xx) * 2.5f);
                float3 _434 = float3(_433.x, 0.0f.xxx.y, _433.y);
                float _435 = distance(_434, _383);
                bool _436 = _435 < _401;
                bool3 _437 = _436.xxx;
                _401 = _436 ? _435 : _401;
                _411++;
                _394 = float3(_437.x ? _434.x : _394.x, _437.y ? _434.y : _394.y, _437.z ? _434.z : _394.z);
                _397 = _436 ? _419 : _397;
                continue;
            }
        }
        uint _439 = _385 + (_385 << 10u);
        uint _441 = _439 ^ (_439 >> 6u);
        uint _443 = _441 + (_441 << 3u);
        uint _445 = _443 ^ (_443 >> 11u);
        uint _447 = _445 + (_445 << 15u);
        uint _448 = _447 % 65536u;
        bool _451 = frame_constants_glade_theme_id == 4u;
        if (_451)
        {
            float2 _454 = _383.xz * 0.20000000298023223876953125f;
            float2 _458 = floor(_454 + dot(_454, 0.3660254180431365966796875f.xx).xx);
            float2 _462 = (_454 - _458) + dot(_458, 0.211324870586395263671875f.xx).xx;
            float _463 = _462.x;
            float _464 = _462.y;
            bool2 _466 = (_463 > _464).xx;
            float2 _467 = float2(_466.x ? float2(1.0f, 0.0f).x : float2(0.0f, 1.0f).x, _466.y ? float2(1.0f, 0.0f).y : float2(0.0f, 1.0f).y);
            float4 _469 = _462.xyxy + float4(0.211324870586395263671875f, 0.211324870586395263671875f, -0.57735025882720947265625f, -0.57735025882720947265625f);
            float2 _471 = _469.xy - _467;
            float4 _472 = float4(_471.x, _471.y, _469.z, _469.w);
            float2 _473 = fmod(_458, 289.0f.xx);
            float3 _478 = _473.y.xxx + float3(0.0f, _467.y, 1.0f);
            float3 _488 = (fmod(((_478 * 34.0f) + 1.0f.xxx) * _478, 289.0f.xxx) + _473.x.xxx) + float3(0.0f, _467.x, 1.0f);
            float2 _494 = _471.xy;
            float2 _496 = _469.zw;
            float3 _500 = max(0.5f.xxx - float3(dot(_462, _462), dot(_494, _494), dot(_496, _496)), 0.0f.xxx);
            float3 _501 = _500 * _500;
            float3 _505 = frac(fmod(((_488 * 34.0f) + 1.0f.xxx) * _488, 289.0f.xxx) * 0.024390242993831634521484375f.xxx) * 2.0f;
            float3 _506 = _505 - 1.0f.xxx;
            float3 _508 = abs(_506) - 0.5f.xxx;
            float3 _511 = _506 - floor(_505 + (-0.5f).xxx);
            float3 _523;
            _523.x = (_511.x * _463) + (_508.x * _464);
            float2 _530 = (_511.yz * _472.xz) + (_508.yz * _472.yw);
            if ((130.0f * dot((_501 * _501) * (1.792842864990234375f.xxx - (((_511 * _511) + (_508 * _508)) * 0.8537347316741943359375f)), float3(_523.x, _530.x, _530.y))) > 0.5f)
            {
                break;
            }
        }
        uint _538 = uint(_396 * 13);
        uint _540 = _538 + (_538 << 10u);
        uint _542 = _540 ^ (_540 >> 6u);
        uint _544 = _542 + (_542 << 3u);
        uint _546 = _544 ^ (_544 >> 11u);
        uint _556 = _448 + (_448 << 10u);
        uint _558 = _556 ^ (_556 >> 6u);
        uint _560 = _558 + (_558 << 3u);
        uint _562 = _560 ^ (_560 >> 11u);
        float _570 = ((asfloat(((_562 + (_562 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * 2.0f) - 1.0f;
        float _573 = log(1.0f - (_570 * _570));
        float _575 = 4.54728412628173828125f + (0.5f * _573);
        float _587 = (((asfloat(((_546 + (_546 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * 1.5f) + 0.5f) + ((sqrt(sqrt((_575 * _575) - (_573 * 7.142857074737548828125f)) - _575) * float(int(sign(_570)))) * 0.300000011920928955078125f);
        int _588 = _396 * 2;
        uint _589 = uint(_588);
        uint _591 = _589 + (_589 << 10u);
        uint _593 = _591 ^ (_591 >> 6u);
        uint _595 = _593 + (_593 << 3u);
        uint _597 = _595 ^ (_595 >> 11u);
        bool _604 = (asfloat(((_597 + (_597 << 15u)) & 8388607u) | 1065353216u) - 1.0f) < 0.699999988079071044921875f;
        float2 _631;
        if (_604)
        {
            uint _611 = uint(_396);
            uint _613 = _611 + (_611 << 10u);
            uint _615 = _613 ^ (_613 >> 6u);
            uint _617 = _615 + (_615 << 3u);
            uint _619 = _617 ^ (_617 >> 11u);
            float _626 = (1.0f - asfloat(((_619 + (_619 << 15u)) & 8388607u) | 1065353216u)) * 6.280000209808349609375f;
            _631 = normalize(float2(cos(_626), sin(_626)));
        }
        else
        {
            _631 = normalize(_383 - _393).xz;
        }
        uint _635 = uint(int(_448 + 69u));
        uint _637 = _635 + (_635 << 10u);
        uint _639 = _637 ^ (_637 >> 6u);
        uint _641 = _639 + (_639 << 3u);
        uint _643 = _641 ^ (_641 >> 11u);
        float _650 = (1.0f - asfloat(((_643 + (_643 << 15u)) & 8388607u) | 1065353216u)) * 6.280000209808349609375f;
        float2 _654 = normalize(float2(cos(_650), sin(_650)));
        uint _655 = _448 + 34u;
        uint _657 = _655 + (_655 << 10u);
        uint _659 = _657 ^ (_657 >> 6u);
        uint _661 = _659 + (_659 << 3u);
        uint _663 = _661 ^ (_661 >> 11u);
        float _669 = asfloat(((_663 + (_663 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
        float _670 = _669 * 0.199999988079071044921875f;
        float _671 = _670 + 0.5f;
        float2 _700;
        do
        {
            float _676 = dot(normalize(_631), normalize(_654));
            bool _682;
            if (!(_676 > 0.99989998340606689453125f))
            {
                _682 = _676 < (-0.99989998340606689453125f);
            }
            else
            {
                _682 = true;
            }
            if (_682)
            {
                if (_671 <= 0.5f)
                {
                    _700 = _631;
                    break;
                }
                _700 = _654;
                break;
            }
            float _688 = acos(_676);
            _700 = ((_631 * sin((0.5f - _670) * _688)) + (_654 * sin(_671 * _688))) / sin(_688).xx;
            break;
        } while(false);
        float _740;
        float _744;
        uint _702 = uint(_396 + 4235);
        uint _704 = _702 + (_702 << 10u);
        uint _706 = _704 ^ (_704 >> 6u);
        uint _708 = _706 + (_706 << 3u);
        uint _710 = _708 ^ (_708 >> 11u);
        uint _717 = _448 + 15u;
        uint _719 = _717 + (_717 << 10u);
        uint _721 = _719 ^ (_719 >> 6u);
        uint _723 = _721 + (_721 << 3u);
        uint _725 = _723 ^ (_723 >> 11u);
        float3 _735 = lerp(_383, _393, ((asfloat(((_710 + (_710 << 15u)) & 8388607u) | 1065353216u) - 1.0f) - ((asfloat(((_725 + (_725 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * 0.89999997615814208984375f)).xxx);
        float2 _736 = _735.xz;
        float _772;
        do
        {
            _740 = _735.x;
            _744 = _735.z;
            if ((int(abs(_740) < 65.0f) & int(abs(_744) < 65.0f)) != 0)
            {
                _772 = lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (_736 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                break;
            }
            else
            {
                float2 _754 = (_736 * 0.001000000047497451305389404296875f.xx) + 0.5f.xx;
                _772 = lerp(-30.1061992645263671875f, 27.7313995361328125f, bg_terrain_heightmap.SampleLevel(sampler_default, float2(_754.y, 1.0f - _754.x), 0.0f).x);
                break;
            }
            break; 
        } while(false);
        _735.y = _772;
        float2 _778 = (_735.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
        if (foliage_exclusion_mask.SampleLevel(sampler_llb, _778, 0.0f).x > 0.0f)
        {
            break;
        }
        float _789 = smoothstep(0.100000001490116119384765625f, 0.20000000298023223876953125f, path_mask.SampleLevel(sampler_llb, _778, 0.0f).x);
        if (_789 > 0.89999997615814208984375f)
        {
            break;
        }
        float _800;
        bool _802;
        float _827;
        do
        {
            _800 = _778.x;
            _802 = !(_800 < 0.0f);
            bool _806;
            if (_802)
            {
                _806 = _800 > 1.0f;
            }
            else
            {
                _806 = true;
            }
            bool _812;
            if (!_806)
            {
                _812 = _778.y < 0.0f;
            }
            else
            {
                _812 = true;
            }
            bool _818;
            if (!_812)
            {
                _818 = _778.y > 1.0f;
            }
            else
            {
                _818 = true;
            }
            if (_818)
            {
                _827 = 0.0f;
                break;
            }
            else
            {
                _827 = rocky_terrain.SampleLevel(sampler_default, _778, 0.0f).y;
                break;
            }
            break; 
        } while(false);
        float _854;
        do
        {
            bool _833;
            if (_802)
            {
                _833 = _800 > 1.0f;
            }
            else
            {
                _833 = true;
            }
            bool _839;
            if (!_833)
            {
                _839 = _778.y < 0.0f;
            }
            else
            {
                _839 = true;
            }
            bool _845;
            if (!_839)
            {
                _845 = _778.y > 1.0f;
            }
            else
            {
                _845 = true;
            }
            if (_845)
            {
                _854 = 0.0f;
                break;
            }
            else
            {
                _854 = rocky_terrain.SampleLevel(sampler_default, _778, 0.0f).z;
                break;
            }
            break; 
        } while(false);
        float _882;
        do
        {
            bool _861;
            if (_802)
            {
                _861 = _800 > 1.0f;
            }
            else
            {
                _861 = true;
            }
            bool _867;
            if (!_861)
            {
                _867 = _778.y < 0.0f;
            }
            else
            {
                _867 = true;
            }
            bool _873;
            if (!_867)
            {
                _873 = _778.y > 1.0f;
            }
            else
            {
                _873 = true;
            }
            if (_873)
            {
                _882 = 0.0f;
                break;
            }
            else
            {
                _882 = rocky_terrain.SampleLevel(sampler_default, _778, 0.0f).x;
                break;
            }
            break; 
        } while(false);
        if (((_882 + smoothstep(0.0f, 0.5f, _827)) + (_854 * 0.4000000059604644775390625f)) > 0.20000000298023223876953125f)
        {
            break;
        }
        float _915;
        do
        {
            bool _894;
            if (_802)
            {
                _894 = _800 > 1.0f;
            }
            else
            {
                _894 = true;
            }
            bool _900;
            if (!_894)
            {
                _900 = _778.y < 0.0f;
            }
            else
            {
                _900 = true;
            }
            bool _906;
            if (!_900)
            {
                _906 = _778.y > 1.0f;
            }
            else
            {
                _906 = true;
            }
            if (_906)
            {
                _915 = 0.0f;
                break;
            }
            else
            {
                _915 = water_mask.SampleLevel(sampler_default, _778, 0.0f).y;
                break;
            }
            break; 
        } while(false);
        if (_915 > 0.25f)
        {
            break;
        }
        if (smoothstep(0.3499999940395355224609375f, 0.4000000059604644775390625f, garden_mask.SampleLevel(sampler_llb, _778, 0.0f).y) > 0.0f)
        {
            break;
        }
        uint _927 = _448 + 45u;
        uint _929 = _927 + (_927 << 10u);
        uint _931 = _929 ^ (_929 >> 6u);
        uint _933 = _931 + (_931 << 3u);
        uint _935 = _933 ^ (_933 >> 11u);
        float _945 = ((((asfloat(((_935 + (_935 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * 1.5f) + 0.5f) * (_604 ? 1.0f : 0.5f)) * (1.0f - max(_789, 0.0f));
        uint _947 = uint(_588 - 334);
        uint _949 = _947 + (_947 << 10u);
        uint _951 = _949 ^ (_949 >> 6u);
        uint _953 = _951 + (_951 << 3u);
        uint _955 = _953 ^ (_953 >> 11u);
        float _962 = (asfloat(((_955 + (_955 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * 0.300000011920928955078125f;
        bool _966;
        if (_802)
        {
            _966 = _800 > 1.0f;
        }
        else
        {
            _966 = true;
        }
        bool _972;
        if (!_966)
        {
            _972 = _778.y < 0.0f;
        }
        else
        {
            _972 = true;
        }
        bool _978;
        if (!_972)
        {
            _978 = _778.y > 1.0f;
        }
        else
        {
            _978 = true;
        }
        float _993;
        if (!_978)
        {
            float4 _985 = buildings_mask.SampleLevel(sampler_default, _778, 0.0f);
            float _986 = _985.y;
            _993 = _587 * (lerp(1.0f, 0.0f, smoothstep(0.300000011920928955078125f, 0.4000000059604644775390625f, _986)) + (smoothstep(0.5f, 0.699999988079071044921875f, _986) * 0.4000000059604644775390625f));
        }
        else
        {
            _993 = _587;
        }
        float _997;
        if (_451)
        {
            _997 = _945 * 2.0f;
        }
        else
        {
            _997 = _945;
        }
        if (_993 < 0.00999999977648258209228515625f)
        {
            break;
        }
        float3 _1018;
        float3 _1019;
        if (frame_constants_glade_theme_id == 2u)
        {
            _1018 = float3(0.314999997615814208984375f, 0.2700000107288360595703125f, 0.1500000059604644775390625f);
            _1019 = color_library_grass_1.xyz;
        }
        else
        {
            float3 _1016;
            float3 _1017;
            if (frame_constants_glade_theme_id == 3u)
            {
                _1016 = color_library_grass_2.xyz * 0.75f;
                _1017 = color_library_grass_1.xyz * 0.75f;
            }
            else
            {
                _1016 = color_library_grass_2.xyz;
                _1017 = color_library_grass_1.xyz;
            }
            _1018 = _1016;
            _1019 = _1017;
        }
        float2 _1020 = _735.xz * 0.4000000059604644775390625f.xx;
        float2 _1021 = _1020 * 1.0f;
        int2 _1023 = int2(floor(_1021));
        float2 _1024 = frac(_1021);
        uint2 _1025 = uint2(_1023);
        uint _1027 = _1025.y;
        uint _1029 = _1027 + (_1027 << 10u);
        uint _1031 = _1029 ^ (_1029 >> 6u);
        uint _1033 = _1031 + (_1031 << 3u);
        uint _1035 = _1033 ^ (_1033 >> 11u);
        uint _1041 = (((_1025.x * 1664525u) + (_1035 + (_1035 << 15u))) + 1013904223u) * 1664525u;
        uint _1043 = _1041 ^ (_1041 >> 11u);
        uint _1046 = _1043 ^ ((_1043 << 7u) & 2636928640u);
        uint _1049 = _1046 ^ ((_1046 << 15u) & 4022730752u);
        uint2 _1057 = uint2(_1023 + int2(1, 0));
        uint _1059 = _1057.y;
        uint _1061 = _1059 + (_1059 << 10u);
        uint _1063 = _1061 ^ (_1061 >> 6u);
        uint _1065 = _1063 + (_1063 << 3u);
        uint _1067 = _1065 ^ (_1065 >> 11u);
        uint _1073 = (((_1057.x * 1664525u) + (_1067 + (_1067 << 15u))) + 1013904223u) * 1664525u;
        uint _1075 = _1073 ^ (_1073 >> 11u);
        uint _1078 = _1075 ^ ((_1075 << 7u) & 2636928640u);
        uint _1081 = _1078 ^ ((_1078 << 15u) & 4022730752u);
        uint2 _1089 = uint2(_1023 + int2(0, 1));
        uint _1091 = _1089.y;
        uint _1093 = _1091 + (_1091 << 10u);
        uint _1095 = _1093 ^ (_1093 >> 6u);
        uint _1097 = _1095 + (_1095 << 3u);
        uint _1099 = _1097 ^ (_1097 >> 11u);
        uint _1105 = (((_1089.x * 1664525u) + (_1099 + (_1099 << 15u))) + 1013904223u) * 1664525u;
        uint _1107 = _1105 ^ (_1105 >> 11u);
        uint _1110 = _1107 ^ ((_1107 << 7u) & 2636928640u);
        uint _1113 = _1110 ^ ((_1110 << 15u) & 4022730752u);
        uint2 _1121 = uint2(_1023 + int2(1, 1));
        uint _1123 = _1121.y;
        uint _1125 = _1123 + (_1123 << 10u);
        uint _1127 = _1125 ^ (_1125 >> 6u);
        uint _1129 = _1127 + (_1127 << 3u);
        uint _1131 = _1129 ^ (_1129 >> 11u);
        uint _1137 = (((_1121.x * 1664525u) + (_1131 + (_1131 << 15u))) + 1013904223u) * 1664525u;
        uint _1139 = _1137 ^ (_1137 >> 11u);
        uint _1142 = _1139 ^ ((_1139 << 7u) & 2636928640u);
        uint _1145 = _1142 ^ ((_1142 << 15u) & 4022730752u);
        float2 _1155 = (_1024 * _1024) * (3.0f.xx - (_1024 * 2.0f));
        float _1156 = _1155.x;
        float _1157 = 1.0f - _1156;
        float _1159 = _1155.y;
        float _1171 = (-1.0f) + (2.0f * ((((1.0f - _1159) * (((asfloat(((_1049 ^ (_1049 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _1157) + ((asfloat(((_1081 ^ (_1081 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _1156))) + (((asfloat(((_1113 ^ (_1113 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _1157) * _1159)) + (((asfloat(((_1145 ^ (_1145 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _1156) * _1159)));
        float2 _1173 = _1020 * 0.5f;
        int2 _1175 = int2(floor(_1173));
        float2 _1176 = frac(_1173);
        uint2 _1177 = uint2(_1175);
        uint _1179 = _1177.y;
        uint _1181 = _1179 + (_1179 << 10u);
        uint _1183 = _1181 ^ (_1181 >> 6u);
        uint _1185 = _1183 + (_1183 << 3u);
        uint _1187 = _1185 ^ (_1185 >> 11u);
        uint _1193 = (((_1177.x * 1664525u) + (_1187 + (_1187 << 15u))) + 1013904223u) * 1664525u;
        uint _1195 = _1193 ^ (_1193 >> 11u);
        uint _1198 = _1195 ^ ((_1195 << 7u) & 2636928640u);
        uint _1201 = _1198 ^ ((_1198 << 15u) & 4022730752u);
        uint2 _1209 = uint2(_1175 + int2(1, 0));
        uint _1211 = _1209.y;
        uint _1213 = _1211 + (_1211 << 10u);
        uint _1215 = _1213 ^ (_1213 >> 6u);
        uint _1217 = _1215 + (_1215 << 3u);
        uint _1219 = _1217 ^ (_1217 >> 11u);
        uint _1225 = (((_1209.x * 1664525u) + (_1219 + (_1219 << 15u))) + 1013904223u) * 1664525u;
        uint _1227 = _1225 ^ (_1225 >> 11u);
        uint _1230 = _1227 ^ ((_1227 << 7u) & 2636928640u);
        uint _1233 = _1230 ^ ((_1230 << 15u) & 4022730752u);
        uint2 _1241 = uint2(_1175 + int2(0, 1));
        uint _1243 = _1241.y;
        uint _1245 = _1243 + (_1243 << 10u);
        uint _1247 = _1245 ^ (_1245 >> 6u);
        uint _1249 = _1247 + (_1247 << 3u);
        uint _1251 = _1249 ^ (_1249 >> 11u);
        uint _1257 = (((_1241.x * 1664525u) + (_1251 + (_1251 << 15u))) + 1013904223u) * 1664525u;
        uint _1259 = _1257 ^ (_1257 >> 11u);
        uint _1262 = _1259 ^ ((_1259 << 7u) & 2636928640u);
        uint _1265 = _1262 ^ ((_1262 << 15u) & 4022730752u);
        uint2 _1273 = uint2(_1175 + int2(1, 1));
        uint _1275 = _1273.y;
        uint _1277 = _1275 + (_1275 << 10u);
        uint _1279 = _1277 ^ (_1277 >> 6u);
        uint _1281 = _1279 + (_1279 << 3u);
        uint _1283 = _1281 ^ (_1281 >> 11u);
        uint _1289 = (((_1273.x * 1664525u) + (_1283 + (_1283 << 15u))) + 1013904223u) * 1664525u;
        uint _1291 = _1289 ^ (_1289 >> 11u);
        uint _1294 = _1291 ^ ((_1291 << 7u) & 2636928640u);
        uint _1297 = _1294 ^ ((_1294 << 15u) & 4022730752u);
        float2 _1307 = (_1176 * _1176) * (3.0f.xx - (_1176 * 2.0f));
        float _1308 = _1307.x;
        float _1309 = 1.0f - _1308;
        float _1311 = _1307.y;
        float4 _1340 = grass_object_push.SampleLevel(sampler_llb, _778, 0.0f);
        float _1342 = clamp(_1340.z, 0.0f, 1.0f);
        float _1378;
        float2 _1379;
        if (_1342 > 0.0f)
        {
            float2 _1347 = normalize(_1340.xy);
            float2 _1376;
            do
            {
                float _1352 = dot(normalize(_700), normalize(_1347));
                bool _1358;
                if (!(_1352 > 0.99989998340606689453125f))
                {
                    _1358 = _1352 < (-0.99989998340606689453125f);
                }
                else
                {
                    _1358 = true;
                }
                if (_1358)
                {
                    if (_1342 <= 0.5f)
                    {
                        _1376 = _700;
                        break;
                    }
                    _1376 = _1347;
                    break;
                }
                float _1364 = acos(_1352);
                _1376 = ((_700 * sin((1.0f - _1342) * _1364)) + (_1347 * sin(_1342 * _1364))) / sin(_1364).xx;
                break;
            } while(false);
            _1378 = lerp(_962, 0.89999997615814208984375f, _1342);
            _1379 = _1376;
        }
        else
        {
            _1378 = _962;
            _1379 = _700;
        }
        float _1385 = _669 * 10.0f;
        float _1399 = smoothstep(50.0f - _1385, 40.0f - _1385, ((-1.0f) / mul(frame_constants_view_constants[0u].world_to_view, float4(_740, _772, _744, 1.0f)).z) * ((0.5f * frame_constants_view_constants[0u].view_to_clip[1].y) * float(clamp(frame_constants_inner_rendering_resolution.y, 1080u, 2160u))));
        uint _1420;
        if (0u == uint(floor(_1399)))
        {
            uint _1417;
            __alias_draw_commands.InterlockedAdd(uint(draw_indirect_buffer_offset) * 20 + 4, 1u, _1417);
            uint _1419;
            __alias_atomics.InterlockedAdd(0, 1u, _1419);
            _1420 = _1419;
        }
        else
        {
            uint _1409;
            __alias_lod1_commands.InterlockedAdd(lod1_commands_offset * 16 + 0, 3u, _1409);
            uint _1411;
            __alias_atomics.InterlockedAdd(4, 4294967295u, _1411);
            _1420 = _1411 - 1u;
        }
        float2 _1422 = normalize(_1379);
        float3 _1426 = sqrt(clamp(clamp(lerp(_1019, _1018, clamp(((((_1171 * 0.25f) + (((-1.0f) + (2.0f * ((((1.0f - _1311) * (((asfloat(((_1201 ^ (_1201 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _1309) + ((asfloat(((_1233 ^ (_1233 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _1308))) + (((asfloat(((_1265 ^ (_1265 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _1309) * _1311)) + (((asfloat(((_1297 ^ (_1297 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _1308) * _1311)))) * 0.3499999940395355224609375f)) + (_1171 * 0.125f)) + ((asfloat((_447 & 8388607u) | 1065353216u) - 1.5f) * 0.5f)) + 0.20000000298023223876953125f, 0.0f, 1.0f).xxx) * 1.2000000476837158203125f, 0.0f.xxx, 1.0f.xxx), 0.0f.xxx, 1.0f.xxx));
        GrassBladeDataPacked _1485 = { _735, (uint((clamp(_1426.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_1426.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_1426.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u), ((uint((clamp(_1399, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(smoothstep(_330, _330 * 0.89999997615814208984375f, float(_319)), 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_1378, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + (uint((clamp(clamp((_1422.x * 0.5f) + 0.5f, 0.0f, 1.0f), 0.0f, 1.0f) * 255.0f) + 0.5f) << 24u), spvPackHalf2x16(float2(_993 * (1.0f - (pow(_789, 1.0f) * 0.699999988079071044921875f)), 0.0f)) | (spvPackHalf2x16(float2(_997, 0.0f)) << 16u), spvPackHalf2x16(float2(2.0f, 0.0f)) | (spvPackHalf2x16(float2(_1422.y, 0.0f)) << 16u), 0u };
        __alias_grass_blade_buffer.Store<float3>(_1420 * 32 + 0, _1485.position);
        __alias_grass_blade_buffer.Store<uint>(_1420 * 32 + 12, _1485.color);
        __alias_grass_blade_buffer.Store<uint>(_1420 * 32 + 16, _1485.packed0);
        __alias_grass_blade_buffer.Store<uint>(_1420 * 32 + 20, _1485.packed1);
        __alias_grass_blade_buffer.Store<uint>(_1420 * 32 + 24, _1485.packed2);
        __alias_grass_blade_buffer.Store<uint>(_1420 * 32 + 28, _1485.pad);
        break;
    } while(false);
}

[numthreads(64, 1, 1)]
void cs_main(SPIRV_Cross_Input stage_input)
{
    gl_WorkGroupID = stage_input.gl_WorkGroupID;
    gl_LocalInvocationIndex = stage_input.gl_LocalInvocationIndex;
    comp_main();
}
#pragma dxc diagnostic pop
#endif
