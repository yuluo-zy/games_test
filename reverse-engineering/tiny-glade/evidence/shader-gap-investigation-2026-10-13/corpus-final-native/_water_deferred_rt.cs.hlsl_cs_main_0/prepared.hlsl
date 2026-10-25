#define TGV_0 1
#line 1 "shaders/water/deferred_rt.cs.hlsl"


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
#line 24 "shaders/water/deferred_rt.cs.hlsl"

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
#line 25 "shaders/water/deferred_rt.cs.hlsl"

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
#line 26 "shaders/water/deferred_rt.cs.hlsl"

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
#line 27 "shaders/water/deferred_rt.cs.hlsl"

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
#line 28 "shaders/water/deferred_rt.cs.hlsl"



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
#line 31 "shaders/water/deferred_rt.cs.hlsl"

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
#line 32 "shaders/water/deferred_rt.cs.hlsl"

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
#line 33 "shaders/water/deferred_rt.cs.hlsl"

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
#line 34 "shaders/water/deferred_rt.cs.hlsl"

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
#line 35 "shaders/water/deferred_rt.cs.hlsl"

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
#line 36 "shaders/water/deferred_rt.cs.hlsl"

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
#line 37 "shaders/water/deferred_rt.cs.hlsl"

#line 1 "shaders/inc/consts.hlsl"






#ifndef TINYGLADE_INC_CONSTS_HLSL
#define TINYGLADE_INC_CONSTS_HLSL


static const float RENDER_SCALE = 1.0f;

#endif 
#line 38 "shaders/water/deferred_rt.cs.hlsl"

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
#line 39 "shaders/water/deferred_rt.cs.hlsl"

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
#line 40 "shaders/water/deferred_rt.cs.hlsl"

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
#line 41 "shaders/water/deferred_rt.cs.hlsl"

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
#line 42 "shaders/water/deferred_rt.cs.hlsl"

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
#line 43 "shaders/water/deferred_rt.cs.hlsl"

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
#line 44 "shaders/water/deferred_rt.cs.hlsl"

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
#line 45 "shaders/water/deferred_rt.cs.hlsl"

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
#line 46 "shaders/water/deferred_rt.cs.hlsl"



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
#line 49 "shaders/water/deferred_rt.cs.hlsl"

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
#line 50 "shaders/water/deferred_rt.cs.hlsl"

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
#line 51 "shaders/water/deferred_rt.cs.hlsl"

#line 1 "shaders/inc/sh.hlsl"










float4 sh_basis(float3 dir)
{
    return float4(0.2820949852466583251953125f, 0.48860299587249755859375f * dir);
}

float sh_eval(float4 sh, float3 dir)
{
    return dot(sh, sh_basis(dir));
}
#line 52 "shaders/water/deferred_rt.cs.hlsl"

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
#line 53 "shaders/water/deferred_rt.cs.hlsl"

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
#line 54 "shaders/water/deferred_rt.cs.hlsl"

#line 1 "shaders/inc/brdf_lut.hlsl"


#ifndef TINYGLADE_INC_BRDF_LUT_HLSL
#define TINYGLADE_INC_BRDF_LUT_HLSL



float2 sample_brdf_fg_lut(Texture2D brdf_fg_lut_tex, SamplerState s, float NdotV, float roughness)
{
    float2 uv = float2(saturate(NdotV), roughness * roughness) * 0.984375 + 0.0078125;
    return brdf_fg_lut_tex.SampleLevel(s, uv, 0.0).xy;
}

#endif 
#line 55 "shaders/water/deferred_rt.cs.hlsl"

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
#line 56 "shaders/water/deferred_rt.cs.hlsl"

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
#line 57 "shaders/water/deferred_rt.cs.hlsl"

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
#line 58 "shaders/water/deferred_rt.cs.hlsl"


cbuffer $Globals : register(b13, space0)
{
    uint2 terrain_lighting_texture_dims;
    uint2 blue_noise_texture_dims;
    uint2 terrain_heightmap_dims;
    uint2 visual_terrain_heightmap_dims;
    uint2 brdf_fg_lut_dims;
    uint rt_bvh_node_count;
    uint rt_triangle_count;
    uint history_valid;
    uint2 half_depth_tex_dims;
    uint2 taa_history_tex_dims;
    uint2 reprojection_tex_dims;
    uint2 color_tex_dims;
};

ByteAddressBuffer rt_bl_bvh : register(t23, space0);
ByteAddressBuffer rt_triangles : register(t24, space0);
ByteAddressBuffer water_rt_count_buffer : register(t25, space0);
ByteAddressBuffer water_rt_ray_buffer : register(t26, space0);
Texture2D<float4> blue_noise_texture : register(t15, space0);
Texture2D<float4> terrain_heightmap : register(t18, space0);
Texture2D<float4> visual_terrain_heightmap : register(t19, space0);
Texture2D<float4> half_depth_tex : register(t27, space0);
Texture2D<float4> taa_history_tex : register(t28, space0);
Texture2D<float4> reprojection_tex : register(t29, space0);
[[vk::image_format("r11g11b10f")]] RWTexture2D<float3> color_tex : register(u30, space0);


float4 sample_blue_noise(uint2 pixel, uint scroll)
{
    float2 offset = frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(scroll)) + 0.5f);
    return ((blue_noise_texture.Load(int3((pixel + uint2(offset * 256.0f)) % uint2(256u, 256u), 0u)) * 255.0f)
        * 0.00390625f) + 0.001953125f;
}


uint pack_half2x16(float2 value)
{
    uint2 packed = f32tof16(value);
    return packed.x | (packed.y << 16u);
}

float2 unpack_half2x16(uint value)
{
    return f16tof32(uint2(value & 0xFFFFu, value >> 16u));
}

struct RayResult
{
    uint type;
    float t;
    float3 normal;
};





RayResult trace_bl_bvh(ByteAddressBuffer rt_bl_bvh_, ByteAddressBuffer rt_triangles_,
                       float3 ro, float3 rd)
{
    RayResult res;
    res.type = 0u;
    res.t = 100000.0f;
    res.normal = float3(0.0f, 1.0f, 0.0f);

    uint2 stack[8];
    uint2 trav = uint2(0u, 0x80000000u);
    float3 best_normal = 0.0f;
    uint stack_depth = 0u;
    int best_triangle = -1;
    float best_t = 3.4028234663852885981170418348452e+38f;

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
                stack[stack_depth] = kept;
                nd = int(stack_depth) + 1;
            }
            else
            {
                nd = int(stack_depth);
            }

            uint octant = (7u - uint((((rd.x < 0.0f) ? 4u : 0u) | ((rd.y < 0.0f) ? 2u : 0u)) | (rd.z < 0.0f ? 1u : 0u))) * 16843009u;

            PackedBlBvhNode n;
            [unroll]
            for (uint i = 0u; i < 5u; ++i)
            {
                n.data[i] = rt_bl_bvh_.Load<uint4>(i * 16u + (trav.x + countbits(trav.y & (~(0xFFFFFFFFu << (((b - 24u) ^ (octant & 255u)) & 31u))))) * 80u + 0u);
            }

            float3 inv_dir = float3(asfloat(((n.data[0].w >> 0u) & 255u) << 23u) / rd.x,
                                       asfloat(((n.data[0].w >> 8u) & 255u) << 23u) / rd.y,
                                       asfloat(((n.data[0].w >> 16u) & 255u) << 23u) / rd.z);
            float3 offset = (asfloat(n.data[0].xyz) - ro) / rd;

            uint accum = 0u;

            
            uint xor0 = (n.data[1].z ^ (octant & ((((n.data[1].z & (n.data[1].z << 1u)) & 269488144u) >> 4u) * 255u))) & 522133279u;
            uint count0 = (n.data[1].z >> 5u) & 117901063u;
            uint c0 = (rd.x < 0.0f) ? n.data[2].z : n.data[2].x;
            uint c1 = (rd.x < 0.0f) ? n.data[2].x : n.data[2].z;
            uint c2 = (rd.y < 0.0f) ? n.data[3].z : n.data[3].x;
            uint c3 = (rd.y < 0.0f) ? n.data[3].x : n.data[3].z;
            uint c4 = (rd.z < 0.0f) ? n.data[4].z : n.data[4].x;
            uint c5 = (rd.z < 0.0f) ? n.data[4].x : n.data[4].z;
            [unroll]
            for (uint s0 = 0u; s0 < 32u; s0 += 8u)
            {
                float3 lo = (float3(float((c0 >> s0) & 255u), float((c2 >> s0) & 255u), float((c4 >> s0) & 255u)) * inv_dir) + offset;
                float3 hi = (float3(float((c1 >> s0) & 255u), float((c3 >> s0) & 255u), float((c5 >> s0) & 255u)) * inv_dir) + offset;
                bool hit0 = max(lo.x, max(lo.y, max(lo.z, 9.9999997473787516355514526367188e-05f))) <= min(hi.x, min(hi.y, min(hi.z, best_t)));
                if (hit0)
                {
                    accum |= (((count0 >> s0) & 255u) << ((xor0 >> s0) & 31u));
                }
            }

            
            uint xor1 = (n.data[1].w ^ (octant & ((((n.data[1].w & (n.data[1].w << 1u)) & 269488144u) >> 4u) * 255u))) & 522133279u;
            uint count1 = (n.data[1].w >> 5u) & 117901063u;
            uint d0 = (rd.x < 0.0f) ? n.data[2].w : n.data[2].y;
            uint d1 = (rd.x < 0.0f) ? n.data[2].y : n.data[2].w;
            uint d2 = (rd.y < 0.0f) ? n.data[3].w : n.data[3].y;
            uint d3 = (rd.y < 0.0f) ? n.data[3].y : n.data[3].w;
            uint d4 = (rd.z < 0.0f) ? n.data[4].w : n.data[4].y;
            uint d5 = (rd.z < 0.0f) ? n.data[4].y : n.data[4].w;
            [unroll]
            for (uint s1 = 0u; s1 < 32u; s1 += 8u)
            {
                float3 lo = (float3(float((d0 >> s1) & 255u), float((d2 >> s1) & 255u), float((d4 >> s1) & 255u)) * inv_dir) + offset;
                float3 hi = (float3(float((d1 >> s1) & 255u), float((d3 >> s1) & 255u), float((d5 >> s1) & 255u)) * inv_dir) + offset;
                bool hit1 = max(lo.x, max(lo.y, max(lo.z, 9.9999997473787516355514526367188e-05f))) <= min(hi.x, min(hi.y, min(hi.z, best_t)));
                if (hit1)
                {
                    accum |= (((count1 >> s1) & 255u) << ((xor1 >> s1) & 31u));
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
            push_depth = int(stack_depth);
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
                tri.v[j] = rt_triangles_.Load<float>(j * 4u + tindex * 24u + 0u);
            }
            [unroll]
            for (uint j = 0u; j < 3u; ++j)
            {
                tri.e[j] = rt_triangles_.Load<uint>(j * 4u + tindex * 24u + 12u);
            }
            float3 edge_a = float3(unpack_half2x16((tri.e[0] >> 16u) & 65535u).x,
                                   unpack_half2x16((tri.e[1] >> 16u) & 65535u).x,
                                   unpack_half2x16((tri.e[2] >> 16u) & 65535u).x);
            float3 edge_b = float3(unpack_half2x16(tri.e[0] & 65535u).x,
                                   unpack_half2x16(tri.e[1] & 65535u).x,
                                   unpack_half2x16(tri.e[2] & 65535u).x);
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
            trav = stack[pd];
            stack_depth = uint(pd);
        }
        else
        {
            trav = node_entry;
            stack_depth = uint(push_depth);
        }
    }

    if (best_triangle != -1 && best_t < 100000.0f)
    {
        res.type = 3u;
        res.t = best_t;
        res.normal = best_normal;
    }
    return res;
}


RayResult water_terrain_march(float3 origin, float3 dir, float max_t)
{
    RayResult res;
    res.type = 0u;
    res.t = max_t;
    res.normal = float3(0.0f, 1.0f, 0.0f);

    float3 bb_lo = (float3(-65.0f, 0.0f, -65.0f) - origin) / dir;
    float3 bb_hi = (float3(65.0f, 24.0f, 65.0f) - origin) / dir;
    float3 tmin = min(bb_lo, bb_hi);
    float3 tmax = max(bb_lo, bb_hi);
    float t_near = max(max(tmin.x, tmin.y), tmin.z);
    float t_far = min(min(tmax.x, tmax.y), tmax.z);
    if (!(t_near <= t_far && t_far >= 0.0f))
    {
        return res;
    }

    float t_lo = max(t_near, 0.0f);
    float step = (t_far - t_lo) * 0.03125f;
    float t = t_lo;
    uint step_idx = 0u;
    bool found = false;
    for (;;)
    {
        if (step_idx < 32u)
        {
            float t_next = t + step;
            float3 p = origin + (dir * t_next);
            if (p.y < lerp(-2.5f, 24.0f, visual_terrain_heightmap.SampleLevel(sampler_lnc,
                    (p.xz * 0.0076923076994717121124267578125f) + 0.5f, 0.0f).x))
            {
                found = true;
                break;
            }
            ++step_idx;
            t = t_next;
            continue;
        }
        else
        {
            break;
        }
    }
    if (!(found && t < res.t && t < t_far))
    {
        return res;
    }

    float lo = t;
    float hi = t_far;
    float last_inside = lo;
    for (uint bi = 0u; bi < 8u; ++bi)
    {
        float mid = (last_inside + hi) * 0.5f;
        float3 p = origin + (dir * mid);
        bool under = p.y < lerp(-2.5f, 24.0f, visual_terrain_heightmap.SampleLevel(sampler_lnc,
            (p.xz * 0.0076923076994717121124267578125f) + 0.5f, 0.0f).x);
        hi = under ? mid : hi;
        last_inside = under ? last_inside : mid;
    }
    if (!(last_inside < res.t))
    {
        return res;
    }

    float2 uv = ((origin + (dir * last_inside)).xz * 0.0076923076994717121124267578125f) + 0.5f;
    res.type = ((lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, uv, 0.0f).x) + 0.100000001490116119384765625f)
        > lerp(-2.5f, 24.0f, visual_terrain_heightmap.SampleLevel(sampler_lnc, uv, 0.0f).x)) ? 1u : 2u;
    res.t = last_inside;
    return res;
}




void resolve_quad_pixel(float2 pixel_uv, uint2 full_pixel,
                        float3 ray_origin, float3 view_ray, float3 tangent, float3 reflect_dir,
                        float hit_w)
{
    float4 sp4 = mul(view_constants[0].view_to_sample,
        mul(view_constants[0].world_to_view, float4(ray_origin + (reflect_dir * hit_w), 1.0f)));
    float3 sp = sp4.xyz / sp4.w;
    float2 sp_uv = (sp.xy * float2(0.5f, -0.5f)) + 0.5f;
    float4 reproj = reprojection_tex.SampleLevel(sampler_nnc, sp_uv, 0.0f);
    float3 sky = 0.0f;
    float mix_factor = 0.0f;
    bool occluded = all(abs(sp) < 1.0f) && sp.z > 0.0f;
    if (occluded && half_depth_tex.SampleLevel(sampler_nnc, reproj.xy + sp_uv, 0.0f).x < (sp.z * 1.10000002384185791015625f))
    {
        sky = (history_valid != 0u && frame_index != 0u)
            ? taa_history_tex.SampleLevel(sampler_nnc, reproj.xy + sp_uv, 0.0f).xyz
            : 0.0f;
        mix_factor = 0.0f;
    }
    else
    {
        mix_factor = float(hit_w > 10000.0f);
    }
    if (mix_factor > 0.0f)
    {
        sky += lerp(sky_low_color.xyz, sky_high_color.xyz,
            pow(clamp(1.0f - pow(1.0f - clamp(reflect_dir.y + 0.20000000298023223876953125f, 0.0f, 1.0f), 14.0f), 0.0f, 1.0f),
                0.64999997615814208984375f)) * mix_factor;
    }

    
    float fresnel = 1.0f - dot(view_ray, tangent);
    float fresnel2 = fresnel * fresnel;
    float3 refl = sky * (0.039999999105930328369140625f + (0.959999978542327880859375f * ((fresnel2 * fresnel2) * fresnel)));

    float4 gathered = reprojection_tex.GatherBlue(sampler_lnc, pixel_uv, int2(0, 0));
    bool valid_hist = (history_valid != 0u) && (frame_index != 0u);
    bool need_history = valid_hist && (mix_factor < 1.0f) && all(1.0f == gathered);
    if (need_history)
    {
        float3 hist = taa_history_tex.SampleLevel(sampler_lnc,
            pixel_uv + reprojection_tex.Load(int3(full_pixel, 0u)).xy, 0.0f).xyz;
        float3 acc = color_tex[full_pixel].xyz + refl;
        float hist_lum = dot(hist, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f));
        float acc_lum = dot(acc, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f));
        float weight = clamp(acc_lum, 0.4000000059604644775390625f * hist_lum, 2.5f * hist_lum);
        if (acc_lum > hist_lum)
        {
            color_tex[full_pixel] = acc * (weight / max(9.9999999392252902907785028219223e-09f, acc_lum));
        }
        else
        {
            color_tex[full_pixel] = hist * (weight / max(9.9999999392252902907785028219223e-09f, hist_lum));
        }
    }
    else
    {
        color_tex[full_pixel] = color_tex[full_pixel].xyz + refl;
    }
}

[numthreads(64, 1, 1)]
void cs_main(uint3 gtid : SV_DispatchThreadID)
{
    if (gtid.x >= water_rt_count_buffer.Load<uint>(12))
    {
        return;
    }

    uint4 ray = water_rt_ray_buffer.Load<uint4>(gtid.x * 16u + 0u);
    uint2 pixel = uint2(ray.x & 65535u, (ray.x >> 16u) & 65535u);
    float2 ndc = ((float2(int2(pixel)) + 0.5f) / float2(half_depth_tex_dims) - 0.5f) * float2(2.0f, -2.0f);
    float3 view_ray = -normalize(mul(view_constants[0].view_to_world,
        mul(view_constants[0].sample_to_view, float4(ndc, 0.0f, 1.0f))).xyz);
    float2 refl_oct = (float2(unpack_half2x16(ray.y & 65535u).x,
                              unpack_half2x16((ray.y >> 16u) & 65535u).x) * 2.0f) - 1.0f;
    
    float zsum = (1.0f - abs(refl_oct.x)) - abs(refl_oct.y);
    float2 xy = refl_oct - (((step(0.0f, refl_oct) * 2.0f) - 1.0f) * clamp(-zsum, 0.0f, 1.0f));
    float3 tangent = normalize(float3(xy, zsum));
    float3 reflect_dir = -reflect(view_ray, tangent);
    if (reflect_dir.y < 0.0f)
    {
        reflect_dir.y = -reflect_dir.y;
    }

    float4 depth4 = mul(view_constants[0].view_to_world,
        mul(view_constants[0].sample_to_view, float4(ndc, asfloat(ray.w), 1.0f)));
    float3 origin = (depth4.xyz / depth4.w) + (reflect_dir * asfloat(ray.z));
    float3 ray_dir = reflect_dir;

    float4 hit = float4(0.0f, 0.0f, 0.0f, 100000.0f);
    float3 cur_origin = origin;
    float3 cur_dir = ray_dir;
    uint bounce = 0u;
    for (;;)
    {
        if (bounce < 1u)
        {
            
            float4 bn = sample_blue_noise(pixel, (frame_index + 1234567u) + (bounce * 137u));

            RayResult tri = trace_bl_bvh(rt_bl_bvh, rt_triangles, cur_origin, cur_dir);
            RayResult water = water_terrain_march(cur_origin, cur_dir, tri.t);
            uint type = (water.type != 0u) ? water.type : ((tri.type == 3u) ? 3u : 0u);
            float t = (water.type != 0u) ? water.t : tri.t;
            if (type == 0u)
            {
                
                float3 sky = lerp(sky_low_color.xyz, sky_high_color.xyz,
                    pow(clamp(1.0f - pow(1.0f - clamp(cur_dir.y + 0.20000000298023223876953125f, 0.0f, 1.0f), 14.0f), 0.0f, 1.0f),
                        0.64999997615814208984375f));
                if (cur_dir.y > 0.0f)
                {
                    hit = float4(sky, 100000.0f);
                }
                else if (glade_theme_id == 2u)
                {
                    hit = float4(sky, 100000.0f);
                }
                else
                {
                    hit = float4(0.0f, 0.0f, 0.0f, 100000.0f);
                }
                break;
            }
            float4 hit_t4 = (bounce == 0u) ? float4(0.0f, 0.0f, 0.0f, t) : hit;
            if (type == 1u || type == 2u)
            {
                if ((bounce + 1u) == 1u)
                {
                    hit = float4(0.0f, 0.0f, 0.0f, hit_t4.w);
                    break;
                }
                float c = 1.0f - (2.0f * bn.x);
                float s = sqrt(max(0.0f, 1.0f - (c * c)));
                float a = 6.283185482025146484375f * bn.y;
                hit = float4(0.0f, 0.0f, 0.0f, hit_t4.w);
                cur_dir = normalize(float3(0.0f, 1.0f, 0.0f) + float3(cos(a) * s, sin(a) * s, c));
                cur_origin = hit_t4.xyz + float3(0.0f, 0.001000000047497451305389404296875f, 0.0f);
            }
            else
            {
                if (dot(tri.normal, cur_dir) > 0.0f)
                {
                    hit = hit_t4;
                    break;
                }
                float c = 1.0f - (2.0f * bn.x);
                float s = sqrt(max(0.0f, 1.0f - (c * c)));
                float a = 6.283185482025146484375f * bn.y;
                float3 nn = normalize(tri.normal);
                hit = float4(0.0f, 0.0f, 0.0f, hit_t4.w);
                cur_dir = normalize(nn + float3(cos(a) * s, sin(a) * s, c));
                cur_origin = hit_t4.xyz + (nn * 0.001000000047497451305389404296875f);
            }
            ++bounce;
            continue;
        }
        else
        {
            break;
        }
    }

    
    
    float2 pixel_uv = ((float2(int2(pixel)) + 0.5f) / float2(half_depth_tex_dims));
    if ((ray.w & 1u) != 0u)
    {
        resolve_quad_pixel(pixel_uv, pixel * 2u, origin, view_ray, tangent, reflect_dir, hit.w);
    }
    if ((ray.w & 2u) != 0u)
    {
        resolve_quad_pixel(pixel_uv, (pixel * 2u) + uint2(1u, 0u), origin, view_ray, tangent, reflect_dir, hit.w);
    }
    if ((ray.w & 4u) != 0u)
    {
        resolve_quad_pixel(pixel_uv, (pixel * 2u) + uint2(0u, 1u), origin, view_ray, tangent, reflect_dir, hit.w);
    }
    if ((ray.w & 8u) != 0u)
    {
        resolve_quad_pixel(pixel_uv, (pixel * 2u) + uint2(1u, 1u), origin, view_ray, tangent, reflect_dir, hit.w);
    }
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

struct PackedBlBvhNode
{
    uint4 data[5];
};

struct PackedTriangle
{
    float v[3];
    uint e[3];
};

static float3 _198;
static float3 _219;
static float4 _220;

static const uint2 _200[8] = { uint2(0u, 0u), uint2(0u, 0u), uint2(0u, 0u), uint2(0u, 0u), uint2(0u, 0u), uint2(0u, 0u), uint2(0u, 0u), uint2(0u, 0u) };

cbuffer $Globals : register(b13, space0)
{
    uint2 terrain_lighting_texture_dims : packoffset(c0);
    uint2 blue_noise_texture_dims : packoffset(c0.z);
    uint2 terrain_heightmap_dims : packoffset(c1);
    uint2 visual_terrain_heightmap_dims : packoffset(c1.z);
    uint2 brdf_fg_lut_dims : packoffset(c2);
    uint rt_bvh_node_count : packoffset(c2.z);
    uint rt_triangle_count : packoffset(c2.w);
    uint history_valid : packoffset(c3);
    uint2 half_depth_tex_dims : packoffset(c3.y);
    uint2 taa_history_tex_dims : packoffset(c4);
    uint2 reprojection_tex_dims : packoffset(c4.z);
    uint2 color_tex_dims : packoffset(c5);
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

ByteAddressBuffer rt_bl_bvh : register(t23, space0);
ByteAddressBuffer rt_triangles : register(t24, space0);
ByteAddressBuffer water_rt_count_buffer : register(t25, space0);
ByteAddressBuffer water_rt_ray_buffer : register(t26, space0);
SamplerState sampler_lnc : register(s0, space0);
SamplerState sampler_nnc : register(s2, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> blue_noise_texture : register(t15, space0);
Texture2D<float4> terrain_heightmap : register(t18, space0);
Texture2D<float4> visual_terrain_heightmap : register(t19, space0);
Texture2D<float4> half_depth_tex : register(t27, space0);
Texture2D<float4> taa_history_tex : register(t28, space0);
Texture2D<float4> reprojection_tex : register(t29, space0);
RWTexture2D<float3> color_tex : register(u30, space0);

static uint3 gl_GlobalInvocationID;
struct SPIRV_Cross_Input
{
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
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

void comp_main()
{
    do
    {
        if (gl_GlobalInvocationID.x >= water_rt_count_buffer.Load<uint>(12))
        {
            break;
        }
        uint4 _233 = water_rt_ray_buffer.Load<uint4>(gl_GlobalInvocationID.x * 16 + 0);
        uint2 _238 = uint2(_233.x & 65535u, (_233.x >> 16u) & 65535u);
        float2 _245 = (float2(int2(_238)) + 0.5f.xx) / float2(half_depth_tex_dims);
        float2 _253 = (_245 - 0.5f.xx) * float2(2.0f, -2.0f);
        float _254 = _253.x;
        float4 _261 = mul(frame_constants_view_constants[0u].view_to_world, mul(frame_constants_view_constants[0u].sample_to_view, float4(_254, _253.y, asfloat(_233.w), 1.0f)));
        float3 _264 = -normalize(mul(frame_constants_view_constants[0u].view_to_world, mul(frame_constants_view_constants[0u].sample_to_view, float4(_254, _253.y, 0.0f, 1.0f))).xyz);
        uint _269 = _233.w & 15u;
        float2 _280 = (float2(spvUnpackHalf2x16(_233.y & 65535u).x, spvUnpackHalf2x16((_233.y >> 16u) & 65535u).x) * 2.0f) - 1.0f.xx;
        float _281 = _280.x;
        float _286 = (1.0f - abs(_281)) - abs(_280.y);
        float3 _287 = float3(_281, _280.y, _286);
        float2 _290 = _287.xy;
        float2 _295 = _290 - (((step(0.0f.xx, _290) * 2.0f) - 1.0f.xx) * clamp(-_286, 0.0f, 1.0f));
        float3 _297 = normalize(float3(_295.x, _295.y, _287.z));
        float3 _301 = -reflect(_264, _297);
        float _302 = _301.y;
        float3 _308;
        if (_302 < 0.0f)
        {
            float3 _307 = _301;
            _307.y = -_302;
            _308 = _307;
        }
        else
        {
            _308 = _301;
        }
        float3 _310 = (_261.xyz / _261.w.xxx) + (_308 * asfloat(_233.z));
        float4 _312;
        float3 _315;
        float3 _317;
        float3 _319;
        _312 = float4(0.0f, 0.0f, 0.0f, 100000.0f);
        _315 = _308;
        _317 = _198;
        _319 = _310;
        float4 _313;
        float3 _316;
        float3 _318;
        float3 _320;
        float4 _1185;
        uint _321 = 0u;
        for (;;)
        {
            if (_321 < 1u)
            {
                float _349;
                float4 _343 = ((blue_noise_texture.Load(int3((_238 + uint2(frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float((frame_constants_frame_index + 1234567u) + (_321 * 137u))) + 0.5f.xx) * 256.0f.xx)) % uint2(256u, 256u), 0u)) * 255.0f) * 0.00390625f.xxxx) + 0.001953125f.xxxx;
                float3 _980;
                uint _981;
                float _982;
                do
                {
                    uint2 _222[8] = _200;
                    bool _347 = _315.x < 0.0f;
                    _349 = _315.y;
                    bool _350 = _349 < 0.0f;
                    bool _354 = _315.z < 0.0f;
                    uint _359 = (7u - uint(((_347 ? 4 : 0) | (_350 ? 2 : 0)) | int(_354))) * 16843009u;
                    uint2 _361;
                    float3 _370;
                    _361 = uint2(0u, 2147483648u);
                    _370 = _317;
                    uint2 _362;
                    float _365;
                    int _367;
                    float3 _371;
                    int _373;
                    float _971;
                    int _972;
                    float _364 = 3.4028234663852885981170418348452e+38f;
                    int _366 = 0;
                    uint _368 = 0u;
                    int _372 = -1;
                    [loop]
                    for (;;)
                    {
                        if (_368 < 1024u)
                        {
                            uint2 _867;
                            uint2 _868;
                            int _869;
                            if ((_361.y & 4278190080u) != 0u)
                            {
                                uint _383 = firstbithigh(_361.y);
                                uint _388 = _361.y & (~(1u << (_383 & 31u)));
                                uint2 _389 = _361;
                                _389.y = _388;
                                int _396;
                                if ((_388 & 4278190080u) != 0u)
                                {
                                    _222[_366] = _389;
                                    _396 = _366 + 1;
                                }
                                else
                                {
                                    _396 = _366;
                                }
                                PackedBlBvhNode _407;
                                [unroll]
                                for (int _6ident = 0; _6ident < 5; _6ident++)
                                {
                                    _407.data[_6ident] = rt_bl_bvh.Load<uint4>(_6ident * 16 + (_361.x + countbits(_361.y & (~(4294967295u << (((_383 - 24u) ^ (_359 & 255u)) & 31u))))) * 80 + 0);
                                }
                                float3 _432 = float3(asfloat(((_407.data[0].w >> 0u) & 255u) << 23u) / _315.x, asfloat(((_407.data[0].w >> 8u) & 255u) << 23u) / _349, asfloat(((_407.data[0].w >> 16u) & 255u) << 23u) / _315.z);
                                float3 _434 = (asfloat(_407.data[0].xyz) - _319) / _315;
                                uint _443 = (_407.data[1].z ^ (_359 & ((((_407.data[1].z & (_407.data[1].z << 1u)) & 269488144u) >> 4u) * 255u))) & 522133279u;
                                uint _445 = (_407.data[1].z >> 5u) & 117901063u;
                                uint _452 = _347 ? _407.data[2].z : _407.data[2].x;
                                uint _453 = _347 ? _407.data[2].x : _407.data[2].z;
                                uint _454 = _350 ? _407.data[3].z : _407.data[3].x;
                                uint _455 = _350 ? _407.data[3].x : _407.data[3].z;
                                uint _456 = _354 ? _407.data[4].z : _407.data[4].x;
                                uint _457 = _354 ? _407.data[4].x : _407.data[4].z;
                                float3 _479 = (float3(float((_452 >> 0u) & 255u), float((_454 >> 0u) & 255u), float((_456 >> 0u) & 255u)) * _432) + _434;
                                float3 _481 = (float3(float((_453 >> 0u) & 255u), float((_455 >> 0u) & 255u), float((_457 >> 0u) & 255u)) * _432) + _434;
                                uint _504;
                                if (max(_479.x, max(_479.y, max(_479.z, 9.9999997473787516355514526367188e-05f))) <= min(_481.x, min(_481.y, min(_481.z, _364))))
                                {
                                    _504 = 0u | (((_445 >> 0u) & 255u) << (((_443 >> 0u) & 255u) & 31u));
                                }
                                else
                                {
                                    _504 = 0u;
                                }
                                float3 _526 = (float3(float((_452 >> 8u) & 255u), float((_454 >> 8u) & 255u), float((_456 >> 8u) & 255u)) * _432) + _434;
                                float3 _528 = (float3(float((_453 >> 8u) & 255u), float((_455 >> 8u) & 255u), float((_457 >> 8u) & 255u)) * _432) + _434;
                                uint _551;
                                if (max(_526.x, max(_526.y, max(_526.z, 9.9999997473787516355514526367188e-05f))) <= min(_528.x, min(_528.y, min(_528.z, _364))))
                                {
                                    _551 = _504 | (((_445 >> 8u) & 255u) << (((_443 >> 8u) & 255u) & 31u));
                                }
                                else
                                {
                                    _551 = _504;
                                }
                                float3 _573 = (float3(float((_452 >> 16u) & 255u), float((_454 >> 16u) & 255u), float((_456 >> 16u) & 255u)) * _432) + _434;
                                float3 _575 = (float3(float((_453 >> 16u) & 255u), float((_455 >> 16u) & 255u), float((_457 >> 16u) & 255u)) * _432) + _434;
                                uint _598;
                                if (max(_573.x, max(_573.y, max(_573.z, 9.9999997473787516355514526367188e-05f))) <= min(_575.x, min(_575.y, min(_575.z, _364))))
                                {
                                    _598 = _551 | (((_445 >> 16u) & 255u) << (((_443 >> 16u) & 255u) & 31u));
                                }
                                else
                                {
                                    _598 = _551;
                                }
                                float3 _620 = (float3(float((_452 >> 24u) & 255u), float((_454 >> 24u) & 255u), float((_456 >> 24u) & 255u)) * _432) + _434;
                                float3 _622 = (float3(float((_453 >> 24u) & 255u), float((_455 >> 24u) & 255u), float((_457 >> 24u) & 255u)) * _432) + _434;
                                uint _645;
                                if (max(_620.x, max(_620.y, max(_620.z, 9.9999997473787516355514526367188e-05f))) <= min(_622.x, min(_622.y, min(_622.z, _364))))
                                {
                                    _645 = _598 | (((_445 >> 24u) & 255u) << (((_443 >> 24u) & 255u) & 31u));
                                }
                                else
                                {
                                    _645 = _598;
                                }
                                uint _654 = (_407.data[1].w ^ (_359 & ((((_407.data[1].w & (_407.data[1].w << 1u)) & 269488144u) >> 4u) * 255u))) & 522133279u;
                                uint _656 = (_407.data[1].w >> 5u) & 117901063u;
                                uint _663 = _347 ? _407.data[2].w : _407.data[2].y;
                                uint _664 = _347 ? _407.data[2].y : _407.data[2].w;
                                uint _665 = _350 ? _407.data[3].w : _407.data[3].y;
                                uint _666 = _350 ? _407.data[3].y : _407.data[3].w;
                                uint _667 = _354 ? _407.data[4].w : _407.data[4].y;
                                uint _668 = _354 ? _407.data[4].y : _407.data[4].w;
                                float3 _690 = (float3(float((_663 >> 0u) & 255u), float((_665 >> 0u) & 255u), float((_667 >> 0u) & 255u)) * _432) + _434;
                                float3 _692 = (float3(float((_664 >> 0u) & 255u), float((_666 >> 0u) & 255u), float((_668 >> 0u) & 255u)) * _432) + _434;
                                uint _715;
                                if (max(_690.x, max(_690.y, max(_690.z, 9.9999997473787516355514526367188e-05f))) <= min(_692.x, min(_692.y, min(_692.z, _364))))
                                {
                                    _715 = _645 | (((_656 >> 0u) & 255u) << (((_654 >> 0u) & 255u) & 31u));
                                }
                                else
                                {
                                    _715 = _645;
                                }
                                float3 _737 = (float3(float((_663 >> 8u) & 255u), float((_665 >> 8u) & 255u), float((_667 >> 8u) & 255u)) * _432) + _434;
                                float3 _739 = (float3(float((_664 >> 8u) & 255u), float((_666 >> 8u) & 255u), float((_668 >> 8u) & 255u)) * _432) + _434;
                                uint _762;
                                if (max(_737.x, max(_737.y, max(_737.z, 9.9999997473787516355514526367188e-05f))) <= min(_739.x, min(_739.y, min(_739.z, _364))))
                                {
                                    _762 = _715 | (((_656 >> 8u) & 255u) << (((_654 >> 8u) & 255u) & 31u));
                                }
                                else
                                {
                                    _762 = _715;
                                }
                                float3 _784 = (float3(float((_663 >> 16u) & 255u), float((_665 >> 16u) & 255u), float((_667 >> 16u) & 255u)) * _432) + _434;
                                float3 _786 = (float3(float((_664 >> 16u) & 255u), float((_666 >> 16u) & 255u), float((_668 >> 16u) & 255u)) * _432) + _434;
                                uint _809;
                                if (max(_784.x, max(_784.y, max(_784.z, 9.9999997473787516355514526367188e-05f))) <= min(_786.x, min(_786.y, min(_786.z, _364))))
                                {
                                    _809 = _762 | (((_656 >> 16u) & 255u) << (((_654 >> 16u) & 255u) & 31u));
                                }
                                else
                                {
                                    _809 = _762;
                                }
                                float3 _831 = (float3(float((_663 >> 24u) & 255u), float((_665 >> 24u) & 255u), float((_667 >> 24u) & 255u)) * _432) + _434;
                                float3 _833 = (float3(float((_664 >> 24u) & 255u), float((_666 >> 24u) & 255u), float((_668 >> 24u) & 255u)) * _432) + _434;
                                uint _856;
                                if (max(_831.x, max(_831.y, max(_831.z, 9.9999997473787516355514526367188e-05f))) <= min(_833.x, min(_833.y, min(_833.z, _364))))
                                {
                                    _856 = _809 | (((_656 >> 24u) & 255u) << (((_654 >> 24u) & 255u) & 31u));
                                }
                                else
                                {
                                    _856 = _809;
                                }
                                uint2 _860 = uint2(_407.data[1].x, _388);
                                _860.y = (_856 & 4278190080u) | ((_407.data[0].w >> 24u) & 255u);
                                _867 = _860;
                                _868 = uint2(_407.data[1].y, _856 & 16777215u);
                                _869 = _396;
                            }
                            else
                            {
                                _867 = uint2(0u, 0u);
                                _868 = _361;
                                _869 = _366;
                            }
                            uint2 _871;
                            _871 = _868;
                            _371 = _370;
                            _365 = _364;
                            _373 = _372;
                            uint2 _872;
                            float3 _874;
                            float _875;
                            int _876;
                            uint _877;
                            for (;;)
                            {
                                _877 = _871.y;
                                if (_877 != 0u)
                                {
                                    uint _881 = firstbithigh(_877);
                                    _872 = _871;
                                    _872.y = _877 & (~(1u << (_881 & 31u)));
                                    uint _887 = _871.x + _881;
                                    PackedTriangle _889;
                                    [unroll]
                                    for (int _7ident = 0; _7ident < 3; _7ident++)
                                    {
                                        _889.v[_7ident] = rt_triangles.Load<float>(_7ident * 4 + _887 * 24 + 0);
                                    }
                                    [unroll]
                                    for (int _8ident = 0; _8ident < 3; _8ident++)
                                    {
                                        _889.e[_8ident] = rt_triangles.Load<uint>(_8ident * 4 + _887 * 24 + 12);
                                    }
                                    float3 _920 = float3(spvUnpackHalf2x16((_889.e[0] >> 16u) & 65535u).x, spvUnpackHalf2x16((_889.e[1] >> 16u) & 65535u).x, spvUnpackHalf2x16((_889.e[2] >> 16u) & 65535u).x);
                                    float3 _921 = float3(spvUnpackHalf2x16(_889.e[0] & 65535u).x, spvUnpackHalf2x16(_889.e[1] & 65535u).x, spvUnpackHalf2x16(_889.e[2] & 65535u).x);
                                    bool _955;
                                    do
                                    {
                                        float3 _924 = cross(_315, _921);
                                        float _925 = dot(_920, _924);
                                        float3 _926 = _319 - float3(_889.v[0], _889.v[1], _889.v[2]);
                                        float3 _927 = cross(_926, _920);
                                        float _929 = dot(_921, _927) / _925;
                                        bool _934;
                                        if (_929 >= 0.0f)
                                        {
                                            _934 = _929 < _365;
                                        }
                                        else
                                        {
                                            _934 = false;
                                        }
                                        if (_934 ? true : false)
                                        {
                                            float3 _939;
                                            _939.x = dot(_926, _924);
                                            _939.y = dot(_315, _927);
                                            float2 _944 = _939.xy / _925.xx;
                                            float3 _945 = float3(_944.x, _944.y, _939.z);
                                            _945.z = (1.0f - _944.x) - _944.y;
                                            if (all(bool3(_945.x >= (-9.9999997473787516355514526367188e-05f).xxx.x, _945.y >= (-9.9999997473787516355514526367188e-05f).xxx.y, _945.z >= (-9.9999997473787516355514526367188e-05f).xxx.z)))
                                            {
                                                _875 = _929;
                                                _955 = true;
                                                break;
                                            }
                                        }
                                        _875 = _365;
                                        _955 = false;
                                        break;
                                    } while(false);
                                    if (_955)
                                    {
                                        _874 = cross(_920, _921);
                                        _876 = int(_887);
                                    }
                                    else
                                    {
                                        _874 = _371;
                                        _876 = _373;
                                    }
                                    _871 = _872;
                                    _371 = _874;
                                    _365 = _875;
                                    _373 = _876;
                                    continue;
                                }
                                else
                                {
                                    break;
                                }
                            }
                            if ((_867.y & 4278190080u) == 0u)
                            {
                                if (_869 == 0)
                                {
                                    _318 = _371;
                                    _971 = _365;
                                    _972 = _373;
                                    break;
                                }
                                int _968 = _869 - 1;
                                _362 = _222[_968];
                                _367 = _968;
                            }
                            else
                            {
                                _362 = _867;
                                _367 = _869;
                            }
                            _361 = _362;
                            _364 = _365;
                            _366 = _367;
                            _368++;
                            _370 = _371;
                            _372 = _373;
                            continue;
                        }
                        else
                        {
                            _318 = _370;
                            _971 = _364;
                            _972 = _372;
                            break;
                        }
                    }
                    bool _977;
                    if (_972 != (-1))
                    {
                        _977 = _971 < 100000.0f;
                    }
                    else
                    {
                        _977 = false;
                    }
                    if (_977)
                    {
                        _980 = _318;
                        _981 = 3u;
                        _982 = _971;
                        break;
                    }
                    _980 = float3(0.0f, 1.0f, 0.0f);
                    _981 = 0u;
                    _982 = 100000.0f;
                    break;
                } while(false);
                uint _1097;
                float _1098;
                do
                {
                    float3 _986 = (float3(65.0f, 24.0f, 65.0f) - _319) / _315;
                    float3 _988 = (float3(-65.0f, 0.0f, -65.0f) - _319) / _315;
                    float3 _989 = min(_986, _988);
                    float3 _990 = max(_986, _988);
                    float _995 = max(max(_989.x, _989.y), _989.z);
                    float _1000 = min(min(_990.x, _990.y), _990.z);
                    bool _1005;
                    if (_995 <= _1000)
                    {
                        _1005 = _1000 >= 0.0f;
                    }
                    else
                    {
                        _1005 = false;
                    }
                    if (!_1005)
                    {
                        _1097 = _981;
                        _1098 = _982;
                        break;
                    }
                    float _1009 = max(_995, 0.0f);
                    float _1011 = (_1000 - _1009) * 0.03125f;
                    float _1013;
                    _1013 = _1009;
                    float _1014;
                    uint _1017;
                    float _1036;
                    uint _1016 = 0u;
                    bool _1018;
                    for (;;)
                    {
                        _1018 = _1016 < 32u;
                        if (_1018)
                        {
                            _1014 = _1013 + _1011;
                            float3 _1022 = _319 + (_315 * _1014);
                            if (_1022.y < lerp(-2.5f, 24.0f, visual_terrain_heightmap.SampleLevel(sampler_lnc, (_1022.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x))
                            {
                                _1036 = _1014;
                                break;
                            }
                            _1017 = _1016 + 1u;
                            _1013 = _1014;
                            _1016 = _1017;
                            continue;
                        }
                        else
                        {
                            _1036 = _1000;
                            break;
                        }
                    }
                    bool _1040;
                    if (_1018)
                    {
                        _1040 = _1013 < _982;
                    }
                    else
                    {
                        _1040 = false;
                    }
                    bool _1044;
                    if (_1040)
                    {
                        _1044 = _1013 < _1036;
                    }
                    else
                    {
                        _1044 = false;
                    }
                    if (_1044)
                    {
                        float _1052;
                        _1052 = _1013;
                        float _1049 = _1036;
                        uint _1054 = 0u;
                        for (; _1054 < 8u; )
                        {
                            float _1058 = _1052 + _1049;
                            float _1059 = _1058 * 0.5f;
                            float3 _1061 = _319 + (_315 * _1059);
                            bool _1072 = _1061.y < lerp(-2.5f, 24.0f, visual_terrain_heightmap.SampleLevel(sampler_lnc, (_1061.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                            _1049 = _1072 ? _1059 : _1049;
                            _1052 = _1072 ? _1052 : _1059;
                            _1054++;
                            continue;
                        }
                        if (_1052 < _982)
                        {
                            float2 _1081 = ((_319 + (_315 * _1052)).xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                            _1097 = ((lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, _1081, 0.0f).x) + 0.100000001490116119384765625f) > lerp(-2.5f, 24.0f, visual_terrain_heightmap.SampleLevel(sampler_lnc, _1081, 0.0f).x)) ? 1u : 2u;
                            _1098 = _1052;
                            break;
                        }
                        else
                        {
                            _1097 = _981;
                            _1098 = _982;
                            break;
                        }
                        break; 
                    }
                    else
                    {
                        _1097 = _981;
                        _1098 = _982;
                        break;
                    }
                    break; 
                } while(false);
                if (_1097 == 0u)
                {
                    float4 _1184;
                    if (_349 > 0.0f)
                    {
                        _1184 = float4(_219.x, _219.y, _219.z, _312.w);
                    }
                    else
                    {
                        float4 _1182;
                        if (frame_constants_glade_theme_id == 2u)
                        {
                            _1182 = float4(_219.x, _219.y, _219.z, _312.w);
                        }
                        else
                        {
                            _1182 = _312;
                        }
                        _1184 = _1182;
                    }
                    _1185 = _1184;
                    break;
                }
                else
                {
                    float4 _1107;
                    if (0u == _321)
                    {
                        float4 _1106;
                        _1106.w = _1098;
                        _1107 = _1106;
                    }
                    else
                    {
                        _1107 = _312;
                    }
                    float3 _1109 = _319 + (_315 * _1098);
                    bool _1115;
                    if (!(_1097 == 1u))
                    {
                        _1115 = _1097 == 2u;
                    }
                    else
                    {
                        _1115 = true;
                    }
                    if (_1115)
                    {
                        if ((_321 + 1u) == 1u)
                        {
                            _1185 = float4(_219.x, _219.y, _219.z, _1107.w);
                            break;
                        }
                        float _1157 = 1.0f - (2.0f * _343.x);
                        float _1161 = sqrt(max(0.0f, 1.0f - (_1157 * _1157)));
                        float _1163 = 6.283185482025146484375f * _343.y;
                        _313 = float4(_219.x, _219.y, _219.z, _1107.w);
                        _316 = normalize(float3(0.0f, 1.0f, 0.0f) + float3(cos(_1163) * _1161, sin(_1163) * _1161, _1157));
                        _320 = _1109 + float3(0.0f, 0.001000000047497451305389404296875f, 0.0f);
                    }
                    else
                    {
                        float4 _1146;
                        float3 _1147;
                        float3 _1148;
                        if (_1097 == 3u)
                        {
                            if (dot(_980, _315) > 0.0f)
                            {
                                _1185 = _1107;
                                break;
                            }
                            float3 _1126 = normalize(_980);
                            float _1132 = 1.0f - (2.0f * _343.x);
                            float _1136 = sqrt(max(0.0f, 1.0f - (_1132 * _1132)));
                            float _1138 = 6.283185482025146484375f * _343.y;
                            _1146 = float4(_219.x, _219.y, _219.z, _1107.w);
                            _1147 = normalize(_1126 + float3(cos(_1138) * _1136, sin(_1138) * _1136, _1132));
                            _1148 = _1109 + (_1126 * 0.001000000047497451305389404296875f);
                        }
                        else
                        {
                            _1146 = _1107;
                            _1147 = _315;
                            _1148 = _319;
                        }
                        _313 = _1146;
                        _316 = _1147;
                        _320 = _1148;
                    }
                }
                _312 = _313;
                _315 = _316;
                _317 = _318;
                _319 = _320;
                _321++;
                continue;
            }
            else
            {
                _1185 = _312;
                break;
            }
        }
        if ((1u & _269) != 0u)
        {
            uint2 _1190 = _238 * uint2(2u, 2u);
            float4 _1203 = mul(frame_constants_view_constants[0u].view_to_sample, mul(frame_constants_view_constants[0u].world_to_view, float4(_310 + (_308 * _1185.w), 1.0f)));
            float3 _1207 = _1203.xyz / _1203.w.xxx;
            float2 _1210 = (_1207.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
            float4 _1214 = reprojection_tex.SampleLevel(sampler_nnc, _1210, 0.0f);
            float3 _1215 = abs(_1207);
            bool _1222;
            if (all(bool3(_1215.x < 1.0f.xxx.x, _1215.y < 1.0f.xxx.y, _1215.z < 1.0f.xxx.z)))
            {
                _1222 = _1207.z > 0.0f;
            }
            else
            {
                _1222 = false;
            }
            bool _1234;
            if (_1222)
            {
                _1234 = half_depth_tex.SampleLevel(sampler_nnc, _1214.xy + _1210, 0.0f).x < (_1207.z * 1.10000002384185791015625f);
            }
            else
            {
                _1234 = false;
            }
            float3 _1259;
            float _1260;
            if (_1234)
            {
                bool _1249;
                if (history_valid != 0u)
                {
                    _1249 = !(0u == frame_constants_frame_index);
                }
                else
                {
                    _1249 = false;
                }
                float3 _1258;
                if (_1249)
                {
                    _1258 = taa_history_tex.SampleLevel(sampler_nnc, _1214.xy + _1210, 0.0f).xyz;
                }
                else
                {
                    _1258 = 0.0f.xxx;
                }
                _1259 = _1258;
                _1260 = 0.0f;
            }
            else
            {
                _1259 = 0.0f.xxx;
                _1260 = ((_1185.w > 10000.0f) ? 1.0f : 0.0f);
            }
            float3 _1286;
            if (_1260 > 0.0f)
            {
                _1286 = _1259 + (((lerp(frame_constants_sky_low_color.xyz * 1.0f, frame_constants_sky_high_color.xyz, pow(clamp(1.0f - pow(1.0f - clamp(_308.y + 0.20000000298023223876953125f, 0.0f, 1.0f), 14.0f), 0.0f, 1.0f), 0.64999997615814208984375f).xxx) * 1.0f) + max(0.0f.xxx, 0.0f.xxx)) * _1260);
            }
            else
            {
                _1286 = _1259;
            }
            float _1288 = 1.0f - dot(_264, _297);
            float _1289 = _1288 * _1288;
            float3 _1294 = _1286 * (0.039999999105930328369140625f + (0.959999978542327880859375f * ((_1289 * _1289) * _1288)));
            float4 _1298 = reprojection_tex.GatherBlue(sampler_lnc, _245, int2(0, 0));
            bool _1310;
            if (history_valid != 0u)
            {
                _1310 = !(0u == frame_constants_frame_index);
            }
            else
            {
                _1310 = false;
            }
            bool _1314;
            if (_1310)
            {
                _1314 = _1260 < 1.0f;
            }
            else
            {
                _1314 = false;
            }
            if (_1314 ? all(bool4(1.0f.xxxx.x == _1298.x, 1.0f.xxxx.y == _1298.y, 1.0f.xxxx.z == _1298.z, 1.0f.xxxx.w == _1298.w)) : false)
            {
                float3 _1329 = taa_history_tex.SampleLevel(sampler_lnc, _245 + reprojection_tex.Load(int3(_1190, 0u)).xy, 0.0f).xyz;
                float3 _1333 = color_tex[_1190].xyzz.xyz + _1294;
                float _1334 = dot(_1329, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f));
                float _1335 = dot(_1333, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f));
                float _1338 = clamp(_1335, 0.4000000059604644775390625f * _1334, 2.5f * _1334);
                if (_1335 > _1334)
                {
                    color_tex[_1190] = _1333 * (_1338 / max(9.9999999392252902907785028219223e-09f, _1335));
                }
                else
                {
                    color_tex[_1190] = _1329 * (_1338 / max(9.9999999392252902907785028219223e-09f, _1334));
                }
            }
            else
            {
                color_tex[_1190] = color_tex[_1190].xyzz.xyz + _1294;
            }
        }
        if ((2u & _269) != 0u)
        {
            uint2 _1356 = (_238 * uint2(2u, 2u)) + uint2(1u, 0u);
            float4 _1369 = mul(frame_constants_view_constants[0u].view_to_sample, mul(frame_constants_view_constants[0u].world_to_view, float4(_310 + (_308 * _1185.w), 1.0f)));
            float3 _1373 = _1369.xyz / _1369.w.xxx;
            float2 _1376 = (_1373.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
            float4 _1380 = reprojection_tex.SampleLevel(sampler_nnc, _1376, 0.0f);
            float3 _1381 = abs(_1373);
            bool _1388;
            if (all(bool3(_1381.x < 1.0f.xxx.x, _1381.y < 1.0f.xxx.y, _1381.z < 1.0f.xxx.z)))
            {
                _1388 = _1373.z > 0.0f;
            }
            else
            {
                _1388 = false;
            }
            bool _1400;
            if (_1388)
            {
                _1400 = half_depth_tex.SampleLevel(sampler_nnc, _1380.xy + _1376, 0.0f).x < (_1373.z * 1.10000002384185791015625f);
            }
            else
            {
                _1400 = false;
            }
            float3 _1425;
            float _1426;
            if (_1400)
            {
                bool _1415;
                if (history_valid != 0u)
                {
                    _1415 = !(0u == frame_constants_frame_index);
                }
                else
                {
                    _1415 = false;
                }
                float3 _1424;
                if (_1415)
                {
                    _1424 = taa_history_tex.SampleLevel(sampler_nnc, _1380.xy + _1376, 0.0f).xyz;
                }
                else
                {
                    _1424 = 0.0f.xxx;
                }
                _1425 = _1424;
                _1426 = 0.0f;
            }
            else
            {
                _1425 = 0.0f.xxx;
                _1426 = ((_1185.w > 10000.0f) ? 1.0f : 0.0f);
            }
            float3 _1452;
            if (_1426 > 0.0f)
            {
                _1452 = _1425 + (((lerp(frame_constants_sky_low_color.xyz * 1.0f, frame_constants_sky_high_color.xyz, pow(clamp(1.0f - pow(1.0f - clamp(_308.y + 0.20000000298023223876953125f, 0.0f, 1.0f), 14.0f), 0.0f, 1.0f), 0.64999997615814208984375f).xxx) * 1.0f) + max(0.0f.xxx, 0.0f.xxx)) * _1426);
            }
            else
            {
                _1452 = _1425;
            }
            float _1454 = 1.0f - dot(_264, _297);
            float _1455 = _1454 * _1454;
            float3 _1460 = _1452 * (0.039999999105930328369140625f + (0.959999978542327880859375f * ((_1455 * _1455) * _1454)));
            float4 _1464 = reprojection_tex.GatherBlue(sampler_lnc, _245, int2(0, 0));
            bool _1476;
            if (history_valid != 0u)
            {
                _1476 = !(0u == frame_constants_frame_index);
            }
            else
            {
                _1476 = false;
            }
            bool _1480;
            if (_1476)
            {
                _1480 = _1426 < 1.0f;
            }
            else
            {
                _1480 = false;
            }
            if (_1480 ? all(bool4(1.0f.xxxx.x == _1464.x, 1.0f.xxxx.y == _1464.y, 1.0f.xxxx.z == _1464.z, 1.0f.xxxx.w == _1464.w)) : false)
            {
                float3 _1495 = taa_history_tex.SampleLevel(sampler_lnc, _245 + reprojection_tex.Load(int3(_1356, 0u)).xy, 0.0f).xyz;
                float3 _1499 = color_tex[_1356].xyzz.xyz + _1460;
                float _1500 = dot(_1495, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f));
                float _1501 = dot(_1499, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f));
                float _1504 = clamp(_1501, 0.4000000059604644775390625f * _1500, 2.5f * _1500);
                if (_1501 > _1500)
                {
                    color_tex[_1356] = _1499 * (_1504 / max(9.9999999392252902907785028219223e-09f, _1501));
                }
                else
                {
                    color_tex[_1356] = _1495 * (_1504 / max(9.9999999392252902907785028219223e-09f, _1500));
                }
            }
            else
            {
                color_tex[_1356] = color_tex[_1356].xyzz.xyz + _1460;
            }
        }
        if ((4u & _269) != 0u)
        {
            uint2 _1522 = (_238 * uint2(2u, 2u)) + uint2(0u, 1u);
            float4 _1535 = mul(frame_constants_view_constants[0u].view_to_sample, mul(frame_constants_view_constants[0u].world_to_view, float4(_310 + (_308 * _1185.w), 1.0f)));
            float3 _1539 = _1535.xyz / _1535.w.xxx;
            float2 _1542 = (_1539.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
            float4 _1546 = reprojection_tex.SampleLevel(sampler_nnc, _1542, 0.0f);
            float3 _1547 = abs(_1539);
            bool _1554;
            if (all(bool3(_1547.x < 1.0f.xxx.x, _1547.y < 1.0f.xxx.y, _1547.z < 1.0f.xxx.z)))
            {
                _1554 = _1539.z > 0.0f;
            }
            else
            {
                _1554 = false;
            }
            bool _1566;
            if (_1554)
            {
                _1566 = half_depth_tex.SampleLevel(sampler_nnc, _1546.xy + _1542, 0.0f).x < (_1539.z * 1.10000002384185791015625f);
            }
            else
            {
                _1566 = false;
            }
            float3 _1591;
            float _1592;
            if (_1566)
            {
                bool _1581;
                if (history_valid != 0u)
                {
                    _1581 = !(0u == frame_constants_frame_index);
                }
                else
                {
                    _1581 = false;
                }
                float3 _1590;
                if (_1581)
                {
                    _1590 = taa_history_tex.SampleLevel(sampler_nnc, _1546.xy + _1542, 0.0f).xyz;
                }
                else
                {
                    _1590 = 0.0f.xxx;
                }
                _1591 = _1590;
                _1592 = 0.0f;
            }
            else
            {
                _1591 = 0.0f.xxx;
                _1592 = ((_1185.w > 10000.0f) ? 1.0f : 0.0f);
            }
            float3 _1618;
            if (_1592 > 0.0f)
            {
                _1618 = _1591 + (((lerp(frame_constants_sky_low_color.xyz * 1.0f, frame_constants_sky_high_color.xyz, pow(clamp(1.0f - pow(1.0f - clamp(_308.y + 0.20000000298023223876953125f, 0.0f, 1.0f), 14.0f), 0.0f, 1.0f), 0.64999997615814208984375f).xxx) * 1.0f) + max(0.0f.xxx, 0.0f.xxx)) * _1592);
            }
            else
            {
                _1618 = _1591;
            }
            float _1620 = 1.0f - dot(_264, _297);
            float _1621 = _1620 * _1620;
            float3 _1626 = _1618 * (0.039999999105930328369140625f + (0.959999978542327880859375f * ((_1621 * _1621) * _1620)));
            float4 _1630 = reprojection_tex.GatherBlue(sampler_lnc, _245, int2(0, 0));
            bool _1642;
            if (history_valid != 0u)
            {
                _1642 = !(0u == frame_constants_frame_index);
            }
            else
            {
                _1642 = false;
            }
            bool _1646;
            if (_1642)
            {
                _1646 = _1592 < 1.0f;
            }
            else
            {
                _1646 = false;
            }
            if (_1646 ? all(bool4(1.0f.xxxx.x == _1630.x, 1.0f.xxxx.y == _1630.y, 1.0f.xxxx.z == _1630.z, 1.0f.xxxx.w == _1630.w)) : false)
            {
                float3 _1661 = taa_history_tex.SampleLevel(sampler_lnc, _245 + reprojection_tex.Load(int3(_1522, 0u)).xy, 0.0f).xyz;
                float3 _1665 = color_tex[_1522].xyzz.xyz + _1626;
                float _1666 = dot(_1661, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f));
                float _1667 = dot(_1665, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f));
                float _1670 = clamp(_1667, 0.4000000059604644775390625f * _1666, 2.5f * _1666);
                if (_1667 > _1666)
                {
                    color_tex[_1522] = _1665 * (_1670 / max(9.9999999392252902907785028219223e-09f, _1667));
                }
                else
                {
                    color_tex[_1522] = _1661 * (_1670 / max(9.9999999392252902907785028219223e-09f, _1666));
                }
            }
            else
            {
                color_tex[_1522] = color_tex[_1522].xyzz.xyz + _1626;
            }
        }
        if ((8u & _269) != 0u)
        {
            uint2 _1688 = (_238 * uint2(2u, 2u)) + uint2(1u, 1u);
            float4 _1701 = mul(frame_constants_view_constants[0u].view_to_sample, mul(frame_constants_view_constants[0u].world_to_view, float4(_310 + (_308 * _1185.w), 1.0f)));
            float3 _1705 = _1701.xyz / _1701.w.xxx;
            float2 _1708 = (_1705.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
            float4 _1712 = reprojection_tex.SampleLevel(sampler_nnc, _1708, 0.0f);
            float3 _1713 = abs(_1705);
            bool _1720;
            if (all(bool3(_1713.x < 1.0f.xxx.x, _1713.y < 1.0f.xxx.y, _1713.z < 1.0f.xxx.z)))
            {
                _1720 = _1705.z > 0.0f;
            }
            else
            {
                _1720 = false;
            }
            bool _1732;
            if (_1720)
            {
                _1732 = half_depth_tex.SampleLevel(sampler_nnc, _1712.xy + _1708, 0.0f).x < (_1705.z * 1.10000002384185791015625f);
            }
            else
            {
                _1732 = false;
            }
            float3 _1757;
            float _1758;
            if (_1732)
            {
                bool _1747;
                if (history_valid != 0u)
                {
                    _1747 = !(0u == frame_constants_frame_index);
                }
                else
                {
                    _1747 = false;
                }
                float3 _1756;
                if (_1747)
                {
                    _1756 = taa_history_tex.SampleLevel(sampler_nnc, _1712.xy + _1708, 0.0f).xyz;
                }
                else
                {
                    _1756 = 0.0f.xxx;
                }
                _1757 = _1756;
                _1758 = 0.0f;
            }
            else
            {
                _1757 = 0.0f.xxx;
                _1758 = ((_1185.w > 10000.0f) ? 1.0f : 0.0f);
            }
            float3 _1784;
            if (_1758 > 0.0f)
            {
                _1784 = _1757 + (((lerp(frame_constants_sky_low_color.xyz * 1.0f, frame_constants_sky_high_color.xyz, pow(clamp(1.0f - pow(1.0f - clamp(_308.y + 0.20000000298023223876953125f, 0.0f, 1.0f), 14.0f), 0.0f, 1.0f), 0.64999997615814208984375f).xxx) * 1.0f) + max(0.0f.xxx, 0.0f.xxx)) * _1758);
            }
            else
            {
                _1784 = _1757;
            }
            float _1786 = 1.0f - dot(_264, _297);
            float _1787 = _1786 * _1786;
            float3 _1792 = _1784 * (0.039999999105930328369140625f + (0.959999978542327880859375f * ((_1787 * _1787) * _1786)));
            float4 _1796 = reprojection_tex.GatherBlue(sampler_lnc, _245, int2(0, 0));
            bool _1808;
            if (history_valid != 0u)
            {
                _1808 = !(0u == frame_constants_frame_index);
            }
            else
            {
                _1808 = false;
            }
            bool _1812;
            if (_1808)
            {
                _1812 = _1758 < 1.0f;
            }
            else
            {
                _1812 = false;
            }
            if (_1812 ? all(bool4(1.0f.xxxx.x == _1796.x, 1.0f.xxxx.y == _1796.y, 1.0f.xxxx.z == _1796.z, 1.0f.xxxx.w == _1796.w)) : false)
            {
                float3 _1827 = taa_history_tex.SampleLevel(sampler_lnc, _245 + reprojection_tex.Load(int3(_1688, 0u)).xy, 0.0f).xyz;
                float3 _1831 = color_tex[_1688].xyzz.xyz + _1792;
                float _1832 = dot(_1827, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f));
                float _1833 = dot(_1831, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f));
                float _1836 = clamp(_1833, 0.4000000059604644775390625f * _1832, 2.5f * _1832);
                if (_1833 > _1832)
                {
                    color_tex[_1688] = _1831 * (_1836 / max(9.9999999392252902907785028219223e-09f, _1833));
                }
                else
                {
                    color_tex[_1688] = _1827 * (_1836 / max(9.9999999392252902907785028219223e-09f, _1832));
                }
            }
            else
            {
                color_tex[_1688] = color_tex[_1688].xyzz.xyz + _1792;
            }
        }
        break;
    } while(false);
}

[numthreads(64, 1, 1)]
void cs_main(SPIRV_Cross_Input stage_input)
{
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    comp_main();
}
#pragma dxc diagnostic pop
#endif
