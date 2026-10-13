#line 1 "shaders/gi/update_gi_radiance.cs.hlsl"












#define TINYGLADE_TERRAIN_HEIGHTMAP_NO_GLOBAL
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
#line 14 "shaders/gi/update_gi_radiance.cs.hlsl"

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
#line 15 "shaders/gi/update_gi_radiance.cs.hlsl"

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
#line 16 "shaders/gi/update_gi_radiance.cs.hlsl"

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
#line 17 "shaders/gi/update_gi_radiance.cs.hlsl"

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
#line 18 "shaders/gi/update_gi_radiance.cs.hlsl"

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
#line 19 "shaders/gi/update_gi_radiance.cs.hlsl"

#line 1 "shaders/inc/reservoir.hlsl"


#ifndef TINYGLADE_INC_RESERVOIR_HLSL
#define TINYGLADE_INC_RESERVOIR_HLSL


struct Reservoir1spp {
    float w_sum;
    uint payload;
    float M;
    float W;
};


struct Reservoir1sppStreamState {
    float p_q_sel; 
    float M_sum;   
};


void reservoir_init(out Reservoir1spp r, out Reservoir1sppStreamState s)
{
    r.w_sum = 0.0;
    r.payload = 0xFFFFFFFFu;
    r.M = 0.0;
    r.W = 0.0;
    s.p_q_sel = 0.0;
    s.M_sum = 0.0;
}




bool reservoir_update(inout Reservoir1spp r, inout Reservoir1sppStreamState s,
                      float w, float target_pdf, uint payload, float u)
{
    s.M_sum += 1.0;
    r.M += 1.0;
    r.w_sum += w;
    if (w / r.w_sum >= u) {
        r.payload = payload;
        s.p_q_sel = target_pdf;
        return true;
    }
    return false;
}


void reservoir_combine_streams(inout Reservoir1sppStreamState dst, Reservoir1sppStreamState src)
{
    dst.M_sum += src.M_sum;
}



void reservoir_resolve(inout Reservoir1spp r, Reservoir1sppStreamState s)
{
    r.M = s.M_sum;
    r.W = r.w_sum / max(1e-8, r.M * s.p_q_sel);
}


uint4 pack_reservoir1spp(Reservoir1spp r)
{
    return uint4(asuint(r.w_sum), r.payload, asuint(r.M), asuint(r.W));
}


Reservoir1spp unpack_reservoir1spp(uint4 v)
{
    Reservoir1spp r;
    r.w_sum = asfloat(v.x);
    r.payload = v.y;
    r.M = asfloat(v.z);
    r.W = asfloat(v.w);
    return r;
}

#endif 
#line 20 "shaders/gi/update_gi_radiance.cs.hlsl"

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
#line 21 "shaders/gi/update_gi_radiance.cs.hlsl"

#line 1 "shaders/inc/consts.hlsl"






#ifndef TINYGLADE_INC_CONSTS_HLSL
#define TINYGLADE_INC_CONSTS_HLSL


static const float RENDER_SCALE = 1.0f;

#endif 
#line 22 "shaders/gi/update_gi_radiance.cs.hlsl"

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
#line 23 "shaders/gi/update_gi_radiance.cs.hlsl"

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
#line 24 "shaders/gi/update_gi_radiance.cs.hlsl"

#line 1 "shaders/gi/restir.inc.hlsl"










#ifndef TINYGLADE_GI_RESTIR_INC_HLSL
#define TINYGLADE_GI_RESTIR_INC_HLSL


uint pack_restir_payload(uint2 pixel)
{
    return pixel.x | (pixel.y << 16u);
}


uint2 unpack_restir_payload(uint payload)
{
    return uint2(payload & 0xFFFFu, payload >> 16u);
}


uint pack_restir_weights(float2 w)
{
    return f32tof16(w.x) | (f32tof16(w.y) << 16u);
}


float2 unpack_restir_weights(uint packed)
{
    return float2(f16tof32(packed & 0xFFFFu), f16tof32(packed >> 16u));
}

#endif 
#line 25 "shaders/gi/update_gi_radiance.cs.hlsl"

#line 1 "shaders/inc/blue_noise.hlsl"

















float4 blue_noise(Texture2D<float4> blue_noise_texture, uint2 pixel_coords, uint frame_index)
{
    uint2 offset = uint2(frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f)
        * float(frame_index)) + 0.5f.xx) * 256.0f);
    uint2 p = (pixel_coords + uint2(13u, 13u) + offset) % uint2(256u, 256u);
    float4 texel = blue_noise_texture.Load(int3(p, 0));
    return texel * 255.0f * (1.0f / 256.0f) + (1.0f / 512.0f);
}
#line 26 "shaders/gi/update_gi_radiance.cs.hlsl"

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
#line 27 "shaders/gi/update_gi_radiance.cs.hlsl"

#line 1 "shaders/inc/rt.hlsl"

























#ifndef TINYGLADE_INC_RT_HLSL
#define TINYGLADE_INC_RT_HLSL

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
#line 29 "shaders/inc/rt.hlsl"







struct PackedBlBvhNode {
    uint4 data[5];
};


struct PackedTriangle {
    float v[3];
    uint e[3];
};


PackedTriangle load_packed_triangle(ByteAddressBuffer rt_triangles, uint index)
{
    PackedTriangle tri;
    [unroll]
    for (uint i = 0u; i < 3u; ++i) {
        tri.v[i] = rt_triangles.Load<float>(i * 4u + index * 24u + 0u);
        tri.e[i] = rt_triangles.Load<uint>(i * 4u + index * 24u + 12u);
    }
    return tri;
}


bool ray_triangle_intersect(float3 ro, float3 rd, float3 v0, float3 v1, float3 v2,
                            float t_max, out float t, out float2 bary)
{
    t = 0.0;
    bary = float2(0.0, 0.0);
    float3 e1 = v1 - v0;
    float3 e2 = v2 - v0;
    float3 p = cross(rd, e2);
    float det = dot(e1, p);
    if (abs(det) < 1e-9) {
        return false;
    }
    float inv_det = 1.0 / det;
    float3 s = ro - v0;
    float u = dot(s, p) * inv_det;
    if (u < 0.0 || u > 1.0) {
        return false;
    }
    float3 q = cross(s, e1);
    float v = dot(rd, q) * inv_det;
    if (v < 0.0 || (u + v) > 1.0) {
        return false;
    }
    t = dot(e2, q) * inv_det;
    bary = float2(u, v);
    return t >= 0.0 && t <= t_max;
}


float2 rt_unpack_half2x16(uint value)
{
    return f16tof32(uint2(value & 0xFFFFu, value >> 16u));
}







bool trace_software_bvh(ByteAddressBuffer rt_bl_bvh,
                        ByteAddressBuffer rt_triangles,
                        uint node_count,
                        uint lane,
                        float3 ro, float3 rd, float t_min, float t_max,
                        inout uint2 stack[4], inout uint stack_size,
                        out uint hit_triangle, out float hit_t, out float2 hit_bary)
{
    hit_triangle = 0xFFFFFFFFu;
    hit_t = t_max;
    hit_bary = float2(0.0f, 0.0f);

    uint2 trav = uint2(0u, 0x80000000u);
    float3 best_normal = 0.0f;
    stack_size = 0u;
    int best_triangle = -1;
    float best_t = 3.4028234663852885981170418348452e+38f;

    
    
    uint octant = (7u - uint((((rd.x < 0.0f) ? 4u : 0u) | ((rd.y < 0.0f) ? 2u : 0u)) | (rd.z < 0.0f ? 1u : 0u))) * 16843009u;

    for (uint iter = 0u; iter < 1024u; ++iter)
    {
        uint2 node_entry;
        uint2 child;
        int push_depth;
        if ((trav.y & 0xFF000000u) != 0u)
        {
            
            uint b = firstbithigh(trav.y);
            uint key = trav.y & ~(1u << (b & 31u));
            uint2 kept = trav;
            kept.y = key;
            int nd;
            if ((key & 0xFF000000u) != 0u)
            {
#ifdef TINYGLADE_RT_LDS_STACK_SPILL
                if (stack_size >= 4u)
                {
                    rt_lds_stack[lane + (stack_size - 4u) * 64u] = kept;
                }
                else
#endif
                {
                    stack[stack_size] = kept;
                }
                nd = int(stack_size) + 1;
            }
            else
            {
                nd = int(stack_size);
            }

            
            PackedBlBvhNode n;
            [unroll]
            for (uint i = 0u; i < 5u; ++i)
            {
                n.data[i] = rt_bl_bvh.Load<uint4>(i * 16u + (trav.x + countbits(trav.y & (~(0xFFFFFFFFu << (((b - 24u) ^ (octant & 255u)) & 31u))))) * 80u + 0u);
            }

            
            float3 inv_dir = float3(asfloat(((n.data[0].w >> 0u) & 255u) << 23u) / rd.x,
                                       asfloat(((n.data[0].w >> 8u) & 255u) << 23u) / rd.y,
                                       asfloat(((n.data[0].w >> 16u) & 255u) << 23u) / rd.z);
            float3 offset = (asfloat(n.data[0].xyz) - ro) / rd;

            
            uint xor0 = (n.data[1].z ^ (octant & ((((n.data[1].z & (n.data[1].z << 1u)) & 269488144u) >> 4u) * 255u))) & 522133279u;
            uint count0 = (n.data[1].z >> 5u) & 117901063u;
            uint c0 = (rd.x < 0.0f) ? n.data[2].z : n.data[2].x;
            uint c1 = (rd.x < 0.0f) ? n.data[2].x : n.data[2].z;
            uint c2 = (rd.y < 0.0f) ? n.data[3].z : n.data[3].x;
            uint c3 = (rd.y < 0.0f) ? n.data[3].x : n.data[3].z;
            uint c4 = (rd.z < 0.0f) ? n.data[4].z : n.data[4].x;
            uint c5 = (rd.z < 0.0f) ? n.data[4].x : n.data[4].z;

            
            uint xor1 = (n.data[1].w ^ (octant & ((((n.data[1].w & (n.data[1].w << 1u)) & 269488144u) >> 4u) * 255u))) & 522133279u;
            uint count1 = (n.data[1].w >> 5u) & 117901063u;
            uint d0 = (rd.x < 0.0f) ? n.data[2].w : n.data[2].y;
            uint d1 = (rd.x < 0.0f) ? n.data[2].y : n.data[2].w;
            uint d2 = (rd.y < 0.0f) ? n.data[3].w : n.data[3].y;
            uint d3 = (rd.y < 0.0f) ? n.data[3].y : n.data[3].w;
            uint d4 = (rd.z < 0.0f) ? n.data[4].w : n.data[4].y;
            uint d5 = (rd.z < 0.0f) ? n.data[4].y : n.data[4].w;

            
            uint accum = 0u;
            [unroll]
            for (uint s = 0u; s < 32u; s += 8u)
            {
                float3 lo = (float3(float((c0 >> s) & 255u), float((c2 >> s) & 255u), float((c4 >> s) & 255u)) * inv_dir) + offset;
                float3 hi = (float3(float((c1 >> s) & 255u), float((c3 >> s) & 255u), float((c5 >> s) & 255u)) * inv_dir) + offset;
                bool hit = max(lo.x, max(lo.y, max(lo.z, 9.9999997473787516355514526367188e-05f))) <= min(hi.x, min(hi.y, min(hi.z, best_t)));
                if (hit)
                {
                    accum |= (((count0 >> s) & 255u) << ((xor0 >> s) & 31u));
                }
            }
            [unroll]
            for (uint s = 0u; s < 32u; s += 8u)
            {
                float3 lo = (float3(float((d0 >> s) & 255u), float((d2 >> s) & 255u), float((d4 >> s) & 255u)) * inv_dir) + offset;
                float3 hi = (float3(float((d1 >> s) & 255u), float((d3 >> s) & 255u), float((d5 >> s) & 255u)) * inv_dir) + offset;
                bool hit = max(lo.x, max(lo.y, max(lo.z, 9.9999997473787516355514526367188e-05f))) <= min(hi.x, min(hi.y, min(hi.z, best_t)));
                if (hit)
                {
                    accum |= (((count1 >> s) & 255u) << ((xor1 >> s) & 31u));
                }
            }

            
            node_entry = uint2(n.data[1].x, key);
            node_entry.y = (accum & 0xFF000000u) | ((n.data[0].w >> 24u) & 255u);
            child = uint2(n.data[1].y, accum & 0x00FFFFFFu);
            push_depth = nd;
        }
        else
        {
            
            node_entry = uint2(0u, 0u);
            child = trav;
            push_depth = int(stack_size);
        }

        
        uint2 tri_state;
        tri_state = child;
        float3 tri_normal = best_normal;
        float tri_best = best_t;
        int tri_hit = best_triangle;
        while (tri_state.y != 0u)
        {
            uint tb = firstbithigh(tri_state.y);
            tri_state.y = tri_state.y & ~(1u << (tb & 31u));
            uint tindex = tri_state.x + tb;
            PackedTriangle tri;
            [unroll]
            for (uint j = 0u; j < 3u; ++j)
            {
                tri.v[j] = rt_triangles.Load<float>(j * 4u + tindex * 24u + 0u);
            }
            [unroll]
            for (uint j = 0u; j < 3u; ++j)
            {
                tri.e[j] = rt_triangles.Load<uint>(j * 4u + tindex * 24u + 12u);
            }
            float3 edge_a = float3(rt_unpack_half2x16((tri.e[0] >> 16u) & 65535u).x,
                                   rt_unpack_half2x16((tri.e[1] >> 16u) & 65535u).x,
                                   rt_unpack_half2x16((tri.e[2] >> 16u) & 65535u).x);
            float3 edge_b = float3(rt_unpack_half2x16(tri.e[0] & 65535u).x,
                                   rt_unpack_half2x16(tri.e[1] & 65535u).x,
                                   rt_unpack_half2x16(tri.e[2] & 65535u).x);
            float3 p = cross(rd, edge_b);
            float det = dot(edge_a, p);
            float3 s = ro - float3(tri.v[0], tri.v[1], tri.v[2]);
            float3 q = cross(s, edge_a);
            float t = dot(edge_b, q) / det;
            if (t >= 0.0f && t < tri_best)
            {
                float2 bary = float2(dot(s, p), dot(rd, q)) / det;
                float3 w = float3(bary.x, bary.y, (1.0f - bary.x) - bary.y);
                if (all(w >= -9.9999997473787516355514526367188e-05f))
                {
                    tri_best = t;
                    tri_normal = cross(edge_a, edge_b);
                    tri_hit = int(tindex);
                }
            }
        }
        best_normal = tri_normal;
        best_t = tri_best;
        best_triangle = tri_hit;

        
        if ((node_entry.y & 0xFF000000u) == 0u)
        {
            if (push_depth == 0)
            {
                break;
            }
            int pd = push_depth - 1;
#ifdef TINYGLADE_RT_LDS_STACK_SPILL
            if (pd >= 4)
            {
                trav = rt_lds_stack[lane + uint((push_depth - 5) * 64)];
            }
            else
#endif
            {
                trav = stack[pd];
            }
            stack_size = uint(pd);
        }
        else
        {
            trav = node_entry;
            stack_size = uint(push_depth);
        }
    }

    if (best_triangle != -1 && best_t < t_max)
    {
        hit_triangle = uint(best_triangle);
        hit_t = best_t;
        hit_bary = float2(0.0f, 0.0f);
        return true;
    }
    return false;
}

#endif 
#line 28 "shaders/gi/update_gi_radiance.cs.hlsl"

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
#line 29 "shaders/gi/update_gi_radiance.cs.hlsl"

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
#line 30 "shaders/gi/update_gi_radiance.cs.hlsl"

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
#line 31 "shaders/gi/update_gi_radiance.cs.hlsl"

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
#line 32 "shaders/gi/update_gi_radiance.cs.hlsl"

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
#line 33 "shaders/gi/update_gi_radiance.cs.hlsl"

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
#line 34 "shaders/gi/update_gi_radiance.cs.hlsl"

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
#line 35 "shaders/gi/update_gi_radiance.cs.hlsl"

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
#line 36 "shaders/gi/update_gi_radiance.cs.hlsl"

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
#line 37 "shaders/gi/update_gi_radiance.cs.hlsl"

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
#line 38 "shaders/gi/update_gi_radiance.cs.hlsl"

#line 1 "shaders/inc/sh.hlsl"










float4 sh_basis(float3 dir)
{
    return float4(0.2820949852466583251953125f, 0.48860299587249755859375f * dir);
}

float sh_eval(float4 sh, float3 dir)
{
    return dot(sh, sh_basis(dir));
}
#line 39 "shaders/gi/update_gi_radiance.cs.hlsl"

#line 1 "shaders/inc/lighting_indirect.hlsl"






#ifndef TINYGLADE_INC_LIGHTING_INDIRECT_HLSL
#define TINYGLADE_INC_LIGHTING_INDIRECT_HLSL



cbuffer spatial_resolve_offsets : register(b24, space0) {
    int4 spatial_resolve_offsets_array[512];
};


static const float3 LIGHT_GRID_WORLD_SCALE = float3(0.24615384638309478759765625, 0.123076923191547393798828125, 0.24615384638309478759765625);
static const float3 LIGHT_GRID_OFFSET = float3(16.0, 2.0, 16.0);
static const uint3 LIGHT_GRID_DIMS = uint3(32, 16, 32);


uint3 get_light_grid_cell(float3 world_pos)
{
    uint3 cell = uint3(clamp(floor(world_pos * LIGHT_GRID_WORLD_SCALE + LIGHT_GRID_OFFSET),
                             float3(0.0, 0.0, 0.0), float3(LIGHT_GRID_DIMS - 1u)));
    return cell;
}


uint get_light_grid_index(uint3 cell)
{
    return (LIGHT_GRID_DIMS.x * ((LIGHT_GRID_DIMS.y * cell.y) + cell.z)) + cell.x;
}



uint2 get_light_cell_range(ByteAddressBuffer light_indirection_grid, uint3 cell)
{
    uint packed = light_indirection_grid.Load<uint>(get_light_grid_index(cell) * 4u + 0u);
    return uint2(packed & 1048575u, min(64u, packed >> 20u));
}


uint get_light_grid_lantern_index(ByteAddressBuffer grid_light_indices, uint index)
{
    return grid_light_indices.Load<uint>(index * 4u + 0u);
}

#endif 
#line 40 "shaders/gi/update_gi_radiance.cs.hlsl"

#line 1 "shaders/inc/material/eval/solid_material.inc.hlsl"


#ifndef TINYGLADE_INC_MATERIAL_EVAL_SOLID_MATERIAL_HLSL
#define TINYGLADE_INC_MATERIAL_EVAL_SOLID_MATERIAL_HLSL

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
#line 6 "shaders/inc/material/eval/solid_material.inc.hlsl"

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
#line 7 "shaders/inc/material/eval/solid_material.inc.hlsl"



MaterialEvalData eval_solid_material(SolidMaterialParams p, float3 world_normal)
{
    MaterialEvalData d;
    init_material_eval_data(d);
    d.albedo = p.albedo;
    d.normal = normalize(world_normal);
    d.roughness = p.roughness;
    d.opacity = 1.0;
    d.emissive = float3(0.0, 0.0, 0.0);
    return d;
}

#endif 
#line 41 "shaders/gi/update_gi_radiance.cs.hlsl"

#line 1 "shaders/inc/brdf_lut.hlsl"


#ifndef TINYGLADE_INC_BRDF_LUT_HLSL
#define TINYGLADE_INC_BRDF_LUT_HLSL



float2 sample_brdf_fg_lut(Texture2D brdf_fg_lut_tex, SamplerState s, float NdotV, float roughness)
{
    float2 uv = float2(saturate(NdotV), roughness * roughness) * 0.984375 + 0.0078125;
    return brdf_fg_lut_tex.SampleLevel(s, uv, 0.0).xy;
}

#endif 
#line 42 "shaders/gi/update_gi_radiance.cs.hlsl"

#line 1 "shaders/inc/highlight_color.hlsl"


#ifndef TINYGLADE_INC_HIGHLIGHT_COLOR_HLSL
#define TINYGLADE_INC_HIGHLIGHT_COLOR_HLSL





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
#line 10 "shaders/inc/highlight_color.hlsl"




float3 get_highlight_color(uint glade_theme_id)
{
    float3 c;
    if (glade_theme_id == 0u) {
        c = color_library.temp_color_1.xyz;
    } else if (glade_theme_id == 1u) {
        c = color_library.temp_color_2.xyz;
    } else {
        c = color_library.temp_color_3.xyz;
    }
    return c;
}

#endif 
#line 43 "shaders/gi/update_gi_radiance.cs.hlsl"

#line 1 "shaders/inc/wall_color_ids.hlsl"


#ifndef TINYGLADE_INC_WALL_COLOR_IDS_HLSL
#define TINYGLADE_INC_WALL_COLOR_IDS_HLSL

#line 1 "shaders/inc/reservoir.hlsl"


#ifndef TINYGLADE_INC_RESERVOIR_HLSL
#define TINYGLADE_INC_RESERVOIR_HLSL


struct Reservoir1spp {
    float w_sum;
    uint payload;
    float M;
    float W;
};


struct Reservoir1sppStreamState {
    float p_q_sel; 
    float M_sum;   
};


void reservoir_init(out Reservoir1spp r, out Reservoir1sppStreamState s)
{
    r.w_sum = 0.0;
    r.payload = 0xFFFFFFFFu;
    r.M = 0.0;
    r.W = 0.0;
    s.p_q_sel = 0.0;
    s.M_sum = 0.0;
}




bool reservoir_update(inout Reservoir1spp r, inout Reservoir1sppStreamState s,
                      float w, float target_pdf, uint payload, float u)
{
    s.M_sum += 1.0;
    r.M += 1.0;
    r.w_sum += w;
    if (w / r.w_sum >= u) {
        r.payload = payload;
        s.p_q_sel = target_pdf;
        return true;
    }
    return false;
}


void reservoir_combine_streams(inout Reservoir1sppStreamState dst, Reservoir1sppStreamState src)
{
    dst.M_sum += src.M_sum;
}



void reservoir_resolve(inout Reservoir1spp r, Reservoir1sppStreamState s)
{
    r.M = s.M_sum;
    r.W = r.w_sum / max(1e-8, r.M * s.p_q_sel);
}


uint4 pack_reservoir1spp(Reservoir1spp r)
{
    return uint4(asuint(r.w_sum), r.payload, asuint(r.M), asuint(r.W));
}


Reservoir1spp unpack_reservoir1spp(uint4 v)
{
    Reservoir1spp r;
    r.w_sum = asfloat(v.x);
    r.payload = v.y;
    r.M = asfloat(v.z);
    r.W = asfloat(v.w);
    return r;
}

#endif 
#line 6 "shaders/inc/wall_color_ids.hlsl"



uint get_wall_color_id(ByteAddressBuffer wall_color_ids, uint index)
{
    return wall_color_ids.Load<uint>(index * 4u + 0u);
}

#endif 
#line 44 "shaders/gi/update_gi_radiance.cs.hlsl"

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
#line 45 "shaders/gi/update_gi_radiance.cs.hlsl"


Texture2D<float4> depth_tex : register(t26, space0);
Texture2D<float4> candidate_radiance_tex : register(t33, space0);
Texture2D<float4> candidate_hit_tex : register(t34, space0);
RWTexture2D<float4> restir_radiance_out_tex : register(u35, space0);
RWTexture2D<float4> restir_ray_dir_out_tex : register(u36, space0);
RWTexture2D<uint2> restir_reservoir_out_tex : register(u37, space0);

uint pack_half2x16(float2 v)
{
    uint2 p = f32tof16(v);
    return p.x | (p.y << 16u);
}

[numthreads(8, 8, 1)]
void cs_main(uint3 globalThreadID : SV_DispatchThreadID)
{
    uint2 xy = globalThreadID.xy;

    restir_radiance_out_tex[xy] = candidate_radiance_tex.Load(int3(xy, 0u));
    restir_ray_dir_out_tex[xy] = float4(candidate_hit_tex.Load(int3(xy, 0u)).xyz, depth_tex.Load(int3((xy * uint2(2u, 2u)) + hi_px_subpixels[frame_index & 3u], 0u)).x);

    uint packed = pack_half2x16(float2(1.0f, 0.0f));
    restir_reservoir_out_tex[xy] = uint2(xy.x | (xy.y << 16u), packed | (packed << 16u));
}
