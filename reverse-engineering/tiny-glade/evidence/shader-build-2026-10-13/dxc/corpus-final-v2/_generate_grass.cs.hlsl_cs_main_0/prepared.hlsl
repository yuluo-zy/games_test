#line 1 "shaders/generate_grass.cs.hlsl"


















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
#line 19 "shaders/generate_grass.cs.hlsl"

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
#line 20 "shaders/generate_grass.cs.hlsl"

#line 1 "shaders/inc/consts.hlsl"






#ifndef TINYGLADE_INC_CONSTS_HLSL
#define TINYGLADE_INC_CONSTS_HLSL


static const float RENDER_SCALE = 1.0f;

#endif 
#line 21 "shaders/generate_grass.cs.hlsl"

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
#line 22 "shaders/generate_grass.cs.hlsl"

#line 1 "shaders/inc/gaussian_rand.hlsl"












float gaussian_rand(uint seed_a, uint seed_b)
{
    float u = (rand01(hash(seed_b)) * 2.0f) - 1.0f;
    float L = log(1.0f - u * u);
    float c = 4.5472841f + 0.5f * L;
    float r = (rand01(hash(seed_a)) * 1.5f) + 0.5f;
    return r + (sqrt(sqrt((c * c) - (L * 7.142857f)) - c) * sign(u)) * 0.3f;
}
#line 23 "shaders/generate_grass.cs.hlsl"

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
#line 24 "shaders/generate_grass.cs.hlsl"






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
#line 31 "shaders/generate_grass.cs.hlsl"





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





cbuffer color_library : register(b14, space0) {
    ColorLibrary color_library;
};

#endif 
#line 37 "shaders/generate_grass.cs.hlsl"

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





cbuffer color_library : register(b14, space0) {
    ColorLibrary color_library;
};

#endif 
#line 78 "shaders/generate_grass.cs.hlsl"

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
#line 79 "shaders/generate_grass.cs.hlsl"

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
#line 80 "shaders/generate_grass.cs.hlsl"

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
#line 81 "shaders/generate_grass.cs.hlsl"

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
#line 82 "shaders/generate_grass.cs.hlsl"

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
#line 83 "shaders/generate_grass.cs.hlsl"

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
#line 84 "shaders/generate_grass.cs.hlsl"

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
#line 85 "shaders/generate_grass.cs.hlsl"

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
#line 86 "shaders/generate_grass.cs.hlsl"

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
#line 87 "shaders/generate_grass.cs.hlsl"

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
#line 88 "shaders/generate_grass.cs.hlsl"

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
#line 89 "shaders/generate_grass.cs.hlsl"

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
#line 90 "shaders/generate_grass.cs.hlsl"

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
#line 91 "shaders/generate_grass.cs.hlsl"


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
