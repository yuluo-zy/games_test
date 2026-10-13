#define SHADER_VARIANT_DEPTH_ONLY 0
#line 1 "shaders/deferred/canopy_light_sun_sky.raster.hlsl"






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

cbuffer color_library : register(b23, space0) {
    ColorLibrary color_library;
};
#line 1 "shaders/deferred/light_sun_sky_prelude.inc.hlsl"











#ifndef TINYGLADE_DEFERRED_LIGHT_SUN_SKY_PRELUDE_INC_HLSL
#define TINYGLADE_DEFERRED_LIGHT_SUN_SKY_PRELUDE_INC_HLSL

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
#line 15 "shaders/deferred/light_sun_sky_prelude.inc.hlsl"

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
#line 16 "shaders/deferred/light_sun_sky_prelude.inc.hlsl"

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
#line 17 "shaders/deferred/light_sun_sky_prelude.inc.hlsl"

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
#line 18 "shaders/deferred/light_sun_sky_prelude.inc.hlsl"

#line 1 "shaders/fullscreen_triangle_vert.inc.hlsl"





#ifndef TINYGLADE_FULLSCREEN_TRIANGLE_VERT_INC_HLSL
#define TINYGLADE_FULLSCREEN_TRIANGLE_VERT_INC_HLSL

struct VsToPs
{
    float4 position : SV_Position;
    float2 uv : TEXCOORD0;
};

VsToPs vs_main(uint vertex_id : SV_VertexID)
{
    VsToPs o;

    if (vertex_id == 0u)
    {
        o.position = float4(-1.0f, -1.0f, 0.0f, 1.0f);
        o.uv = float2(0.0f, 1.0f);
    }
    else if (vertex_id == 1u)
    {
        o.position = float4(3.0f, -1.0f, 0.0f, 1.0f);
        o.uv = float2(2.0f, 1.0f);
    }
    else
    {
        o.position = float4(-1.0f, 3.0f, 0.0f, 1.0f);
        o.uv = float2(0.0f, -1.0f);
    }

    return o;
}

#endif 
#line 19 "shaders/deferred/light_sun_sky_prelude.inc.hlsl"


#endif 
#line 51 "shaders/deferred/canopy_light_sun_sky.raster.hlsl"

#line 1 "shaders/deferred/light_sun_sky.inc.hlsl"



















#ifndef TINYGLADE_DEFERRED_LIGHT_SUN_SKY_INC_HLSL
#define TINYGLADE_DEFERRED_LIGHT_SUN_SKY_INC_HLSL

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
#line 23 "shaders/deferred/light_sun_sky.inc.hlsl"

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
#line 24 "shaders/deferred/light_sun_sky.inc.hlsl"

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
#line 25 "shaders/deferred/light_sun_sky.inc.hlsl"

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
#line 26 "shaders/deferred/light_sun_sky.inc.hlsl"

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
#line 27 "shaders/deferred/light_sun_sky.inc.hlsl"

#line 1 "shaders/inc/lanterns.hlsl"


#ifndef TINYGLADE_INC_LANTERNS_HLSL
#define TINYGLADE_INC_LANTERNS_HLSL


struct LanternPacked {
    float3 position;
    uint intensity__color_id; 
};


float3 get_lantern_position(ByteAddressBuffer lantern_buf, uint index)
{
    return lantern_buf.Load<float3>(index * 16u + 0u);
}


float get_lantern_intensity(ByteAddressBuffer lantern_buf, uint index)
{
    return f16tof32(lantern_buf.Load<uint>(index * 16u + 12u) & 0xffffu);
}


uint get_lantern_color_id(ByteAddressBuffer lantern_buf, uint index)
{
    return lantern_buf.Load<uint>(index * 16u + 12u) >> 16u;
}


float3 get_lantern_color(uint color_id)
{
    if (color_id == 0u) {
        return float3(0.7919800281524658203125, 0.63358402252197265625, 0.07919800281524658203125) / max(1e-10, 0.7919800281524658203125);
    } else if (color_id == 1u) {
        return float3(0.0, 0.1583960056304931640625, 0.7919800281524658203125) / max(1e-10, 0.215240001678466796875);
    } else if (color_id == 2u) {
        return float3(0.1583960056304931640625, 0.7919800281524658203125, 0.1583960056304931640625) / max(1e-10, 0.772159993648529052734375);
    } else if (color_id == 3u) {
        return float3(0.23759402334690093994140625, 0.1583960056304931640625, 0.7919800281524658203125) / max(1e-10, 0.279020011425018310546875);
    }
    return float3(0.7919800281524658203125, 0.23759402334690093994140625, 0.07919800281524658203125) / max(1e-10, 0.4343800246715545654296875);
}



float3 evaluate_lantern_light(float3 lantern_position, float lantern_intensity, uint color_id,
                              float3 sample_pos, float3 normal)
{
    float3 to_light = lantern_position - sample_pos;
    float dist2 = dot(to_light, to_light);
    float dist = length(to_light);
    float3 l = to_light / max(dist, 1e-6);
    float3 color = get_lantern_color(color_id) * 4.0 * lantern_intensity;
    float atten = (1.0 / (0.3359999954700469970703125 + dist2)) *
                  smoothstep(36.0 * lantern_intensity, 16.0 * lantern_intensity, dist2);
    float ndl = max(0.0, dot(normal, l));
    float3 up = to_light + float3(0.0, 0.1500000059604644775390625, 0.0);
    float up_atten = smoothstep(-0.5, 0.20000000298023223876953125, up.y / max(length(up), 1e-6));
    return color * ndl * atten * up_atten;
}

#endif 
#line 28 "shaders/deferred/light_sun_sky.inc.hlsl"

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
#line 29 "shaders/deferred/light_sun_sky.inc.hlsl"




#define TINYGLADE_TERRAIN_HEIGHTMAP_NO_GLOBAL
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
#line 34 "shaders/deferred/light_sun_sky.inc.hlsl"




float3 reconstruct_view_position(float2 uv, float depth, float2 depth_dims)
{
    float2 ndc = (uv - 0.5f.xx) * float2(2.0f, -2.0f);
    float4 view_pos = mul(view_constants[0u].sample_to_view, float4(ndc, depth, 1.0f));
    return view_pos.xyz / view_pos.w;
}





float3 decode_gbuffer_normal(uint packed)
{
    float3 n = float3(float(packed & 255u),
                      float((packed >> 8u) & 255u),
                      float((packed >> 16u) & 255u)) * (1.0f / 255.0f);
    return normalize(n * 2.0f - 1.0f);
}



float3 evaluate_deferred_sun_light(float3 albedo, float roughness, float3 normal,
                                   float3 view_dir, float3 sun_dir, float NdotV)
{
    float NdotL = saturate(dot(normal, sun_dir));
    float3 H = normalize(view_dir + sun_dir);
    float NdotH = saturate(dot(normal, H));
    float HdotL = saturate(dot(H, sun_dir));
    float3 diffuse = albedo * diffuse_ggx(NdotL, NdotV, NdotH, roughness);
    float3 specular = cook_torrance_specular(NdotL, NdotV, NdotH, HdotL, roughness);
    return get_sun_color() * (diffuse + specular) * NdotL;
}



float3 sample_indirect_radiance(Texture2D indirect_radiance_tex, uint2 half_pixel, uint2 dims)
{
    float3 ir = float3(indirect_radiance_tex.Load(int3(half_pixel + uint2(0u * dims.x, 0u), 0u)).x,
                       indirect_radiance_tex.Load(int3(half_pixel + uint2(1u * dims.x, 0u), 0u)).x,
                       indirect_radiance_tex.Load(int3(half_pixel + uint2(2u * dims.x, 0u), 0u)).x);
    return ir;
}

#endif 
#line 52 "shaders/deferred/canopy_light_sun_sky.raster.hlsl"

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
#line 53 "shaders/deferred/canopy_light_sun_sky.raster.hlsl"

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
#line 54 "shaders/deferred/canopy_light_sun_sky.raster.hlsl"

#line 1 "shaders/inc/consts.hlsl"






#ifndef TINYGLADE_INC_CONSTS_HLSL
#define TINYGLADE_INC_CONSTS_HLSL


static const float RENDER_SCALE = 1.0f;

#endif 
#line 55 "shaders/deferred/canopy_light_sun_sky.raster.hlsl"

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
#line 56 "shaders/deferred/canopy_light_sun_sky.raster.hlsl"

#line 1 "shaders/inc/blue_noise.hlsl"

















float4 blue_noise(Texture2D<float4> blue_noise_texture, uint2 pixel_coords, uint frame_index)
{
    uint2 offset = uint2(frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f)
        * float(frame_index)) + 0.5f.xx) * 256.0f);
    uint2 p = (pixel_coords + uint2(13u, 13u) + offset) % uint2(256u, 256u);
    float4 texel = blue_noise_texture.Load(int3(p, 0));
    return texel * 255.0f * (1.0f / 256.0f) + (1.0f / 512.0f);
}
#line 57 "shaders/deferred/canopy_light_sun_sky.raster.hlsl"

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
#line 58 "shaders/deferred/canopy_light_sun_sky.raster.hlsl"

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
#line 59 "shaders/deferred/canopy_light_sun_sky.raster.hlsl"

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
#line 60 "shaders/deferred/canopy_light_sun_sky.raster.hlsl"

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
#line 61 "shaders/deferred/canopy_light_sun_sky.raster.hlsl"

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
#line 62 "shaders/deferred/canopy_light_sun_sky.raster.hlsl"

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
#line 63 "shaders/deferred/canopy_light_sun_sky.raster.hlsl"

#line 1 "shaders/inc/sh.hlsl"










float4 sh_basis(float3 dir)
{
    return float4(0.2820949852466583251953125f, 0.48860299587249755859375f * dir);
}

float sh_eval(float4 sh, float3 dir)
{
    return dot(sh, sh_basis(dir));
}
#line 64 "shaders/deferred/canopy_light_sun_sky.raster.hlsl"

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
#line 65 "shaders/deferred/canopy_light_sun_sky.raster.hlsl"

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
#line 66 "shaders/deferred/canopy_light_sun_sky.raster.hlsl"

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
#line 67 "shaders/deferred/canopy_light_sun_sky.raster.hlsl"

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
#line 68 "shaders/deferred/canopy_light_sun_sky.raster.hlsl"

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
#line 69 "shaders/deferred/canopy_light_sun_sky.raster.hlsl"

#line 1 "shaders/inc/material/eval/canopy_material.inc.hlsl"


#ifndef TINYGLADE_INC_MATERIAL_EVAL_CANOPY_MATERIAL_HLSL
#define TINYGLADE_INC_MATERIAL_EVAL_CANOPY_MATERIAL_HLSL

#line 1 "shaders/inc/material/canopy_material.inc.hlsl"


#ifndef TINYGLADE_INC_MATERIAL_CANOPY_MATERIAL_HLSL
#define TINYGLADE_INC_MATERIAL_CANOPY_MATERIAL_HLSL

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
#line 6 "shaders/inc/material/canopy_material.inc.hlsl"




struct CanopyMaterialParams {
    float3 leaf_color;
    float3 flower_color;
    float roughness;
    float alpha;
};

CanopyMaterialParams init_canopy_material(float3 leaf_color, float3 flower_color, float roughness, float alpha)
{
    CanopyMaterialParams p;
    p.leaf_color = leaf_color;
    p.flower_color = flower_color;
    p.roughness = saturate(roughness);
    p.alpha = saturate(alpha);
    return p;
}

#endif 
#line 6 "shaders/inc/material/eval/canopy_material.inc.hlsl"




MaterialEvalData eval_canopy_material(CanopyMaterialParams p, float3 world_normal)
{
    MaterialEvalData d;
    init_material_eval_data(d);
    d.albedo = p.leaf_color;
    d.normal = normalize(world_normal);
    d.roughness = p.roughness;
    d.opacity = p.alpha;
    d.emissive = p.flower_color * 0.0; 
    return d;
}

#endif 
#line 70 "shaders/deferred/canopy_light_sun_sky.raster.hlsl"

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
#line 71 "shaders/deferred/canopy_light_sun_sky.raster.hlsl"

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
#line 72 "shaders/deferred/canopy_light_sun_sky.raster.hlsl"

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
#line 73 "shaders/deferred/canopy_light_sun_sky.raster.hlsl"

#line 1 "shaders/inc/brdf_lut.hlsl"


#ifndef TINYGLADE_INC_BRDF_LUT_HLSL
#define TINYGLADE_INC_BRDF_LUT_HLSL



float2 sample_brdf_fg_lut(Texture2D brdf_fg_lut_tex, SamplerState s, float NdotV, float roughness)
{
    float2 uv = float2(saturate(NdotV), roughness * roughness) * 0.984375 + 0.0078125;
    return brdf_fg_lut_tex.SampleLevel(s, uv, 0.0).xy;
}

#endif 
#line 74 "shaders/deferred/canopy_light_sun_sky.raster.hlsl"

#line 1 "shaders/terrain_editing/inc_highlight_active_terrain_chunk.inc.hlsl"


#ifndef TINYGLADE_TERRAIN_EDITING_INC_HIGHLIGHT_ACTIVE_TERRAIN_CHUNK_HLSL
#define TINYGLADE_TERRAIN_EDITING_INC_HIGHLIGHT_ACTIVE_TERRAIN_CHUNK_HLSL




float3 highlight_active_terrain_chunk(float3 surface_color, float3 highlight_color, float2 chunk_uv, float strength)
{
    return lerp(surface_color, highlight_color, strength);
}

#endif 
#line 75 "shaders/deferred/canopy_light_sun_sky.raster.hlsl"

#line 1 "shaders/light_binning/grid.inc.hlsl"












#ifndef TINYGLADE_LIGHT_BINNING_GRID_INC_HLSL
#define TINYGLADE_LIGHT_BINNING_GRID_INC_HLSL


static const float3 GRID_WORLD_SCALE = float3(0.24615384638309478759765625f, 0.123076923191547393798828125f, 0.24615384638309478759765625f);
static const float3 GRID_WORLD_OFFSET = float3(16.0f, 2.0f, 16.0f);
static const uint3 GRID_DIMS = uint3(32u, 16u, 32u);
static const float3 GRID_CELL_SIZE = float3(4.0625f, 8.125f, 4.0625f);


uint3 grid_world_pos_to_cell(float3 world_pos)
{
    return uint3(clamp(floor(world_pos * GRID_WORLD_SCALE + GRID_WORLD_OFFSET),
                       float3(0.0f, 0.0f, 0.0f), float3(GRID_DIMS) - 1.0f));
}



uint grid_cell_index(uint3 cell)
{
    return 32u * (32u * cell.y + cell.z) + cell.x;
}


float3 grid_cell_min_world(uint3 cell)
{
    return (float3(cell) - GRID_WORLD_OFFSET) * GRID_CELL_SIZE;
}

float3 grid_cell_max_world(uint3 cell)
{
    return (float3(cell) - GRID_WORLD_OFFSET + float3(1.0f, 1.0f, 1.0f)) * GRID_CELL_SIZE;
}



float grid_point_to_cell_dist2(float3 p, float r2, uint3 cell)
{
    float3 mn = grid_cell_min_world(cell);
    float3 mx = grid_cell_max_world(cell);
    float d2 = r2;
    [unroll]
    for (uint i = 0u; i < 3u; ++i) {
        if (p[i] < mn[i]) {
            float e = p[i] - mn[i];
            d2 -= e * e;
        } else if (p[i] > mx[i]) {
            float e = p[i] - mx[i];
            d2 -= e * e;
        }
    }
    return d2;
}

#endif 
#line 76 "shaders/deferred/canopy_light_sun_sky.raster.hlsl"


#ifndef DEFERRED_FULL_LIGHTING
#define DEFERRED_FULL_LIGHTING 1
#endif

cbuffer $Globals : register(b13, space0)
{
    uint2 gbuffer_tex_dims;
    uint2 depth_tex_dims;
    uint2 half_depth_tex_dims;
    uint2 taa_history_tex_dims;
    uint taa_history_valid;
    uint2 blue_noise_texture_dims;
    uint2 ssgi_tex_dims;
    uint2 screen_shadows_tex_dims;
    uint2 terrain_lighting_texture_dims;
    uint2 highlight_active_hill_dims;
    float time;
    uint lantern_count;
    uint2 indirect_radiance_tex_dims;
    uint2 rdi_reservoir_tex_dims;
    uint2 terrain_heightmap_dims;
    uint2 visual_terrain_heightmap_dims;
    uint2 brdf_fg_lut_dims;
    uint rt_bvh_node_count;
    uint rt_triangle_count;
};

Texture2D<uint4> gbuffer_tex : register(t14, space0);
Texture2D<float4> depth_tex : register(t15, space0);
Texture2D<float4> half_depth_tex : register(t16, space0);
Texture2D<float4> blue_noise_texture : register(t18, space0);
Texture2D<uint4> rdi_reservoir_tex : register(t28, space0);
ByteAddressBuffer light_indirection_grid : register(t36, space0);

#if DEFERRED_FULL_LIGHTING
ByteAddressBuffer lantern_buf : register(t26, space0);
Texture2D<float4> indirect_radiance_tex : register(t27, space0);
Texture2D<float4> shadow_map : register(t25, space0);
ByteAddressBuffer grid_light_indices : register(t37, space0);
#endif





#if DEFERRED_FULL_LIGHTING


#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
static float _262;
static uint _263;
static const float4 _269[3] = { 0.0f.xxxx, 0.0f.xxxx, 0.0f.xxxx };
static const Reservoir1sppStreamState _303 = { 0.0f, 0.0f };
static const Reservoir1spp _304 = { 0.0f, 0u, 0.0f, 0.0f };
float2 spvUnpackHalf2x16(uint value)
{
    return f16tof32(uint2(value & 0xffff, value >> 16));
}
float4 ps_main(VsToPs input) : SV_Target0
{
    float4 out_var_SV_TARGET;
    int2 _327 = int2(input.position.xy);
    uint2 _328 = uint2(_327);
    float4 _343 = ((blue_noise_texture.Load(int3((_328 + uint2(frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(frame_index)) + 0.5f.xx) * 256.0f.xx)) % uint2(256u, 256u), 0u)) * 255.0f) * 0.00390625f.xxxx) + 0.001953125f.xxxx;
    uint4 _345 = gbuffer_tex.Load(int3(_328, 0u));
    float4 _347 = depth_tex.Load(int3(_328, 0u));
    float _348 = _347.x;
    uint _349 = _345.x;
    float _363 = float((_349 >> 24u) & 255u);
    float _364 = _363 * 0.0039215688593685626983642578125f;
    float3 _368 = (float4(float(_349 & 255u) * 0.0039215688593685626983642578125f, float((_349 >> 8u) & 255u) * 0.0039215688593685626983642578125f, float((_349 >> 16u) & 255u) * 0.0039215688593685626983642578125f, _364).xyz * 2.0f) - 1.0f.xxx;
    uint _369 = _345.y;
    float _384 = float((_369 >> 24u) & 255u) * 0.0039215688593685626983642578125f;
    float3 _386 = float4(float(_369 & 255u) * 0.0039215688593685626983642578125f, float((_369 >> 8u) & 255u) * 0.0039215688593685626983642578125f, float((_369 >> 16u) & 255u) * 0.0039215688593685626983642578125f, _384).xyz;
    float3 _387 = _386 * _386;
    float3 _391 = mul(view_constants[2u].view_to_world, float4(0.0f, 0.0f, 1.0f, 0.0f)).xyz;
    float2 _394 = float2(depth_tex_dims);
    float2 _403 = (((float2(_327) + 0.5f.xx) / _394) - 0.5f.xx) * float2(2.0f, -2.0f);
    float _404 = _403.x;
    float4 _409 = float4(_404, _403.y, _348, 1.0f);
    float4 _411 = mul(view_constants[0u].view_to_world, mul(view_constants[0u].sample_to_view, _409));
    float3 _413 = normalize(mul(view_constants[0u].view_to_world, mul(view_constants[0u].sample_to_view, float4(_404, _403.y, 0.0f, 1.0f))).xyz);
    float3 _414 = -_413;
    float3 _422 = (_411.xyz / _411.w.xxx) + (((_413 * 1.0f) * _384) * 1.60000002384185791015625f);
    bool3 _430 = ((render_overrides & 1u) != 0u).xxx;
    float3 _431 = float3(_430.x ? 0.5f.xxx.x : _387.x, _430.y ? 0.5f.xxx.y : _387.y, _430.z ? 0.5f.xxx.z : _387.z);
    bool _432 = _364 > 0.0f;
    float3 _475;
    if (_432)
    {
        uint _436 = uint(_363);
        uint _438 = _436 + (_436 << 10u);
        uint _440 = _438 ^ (_438 >> 6u);
        uint _442 = _440 + (_440 << 3u);
        uint _444 = _442 ^ (_442 >> 11u);
        uint _450 = _436 ^ 188053769u;
        uint _452 = _450 + (_450 << 10u);
        uint _454 = _452 ^ (_452 >> 6u);
        uint _456 = _454 + (_454 << 3u);
        uint _458 = _456 ^ (_456 >> 11u);
        float _464 = asfloat(((_458 + (_458 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
        float _465 = asfloat(((_444 + (_444 << 15u)) & 8388607u) | 1065353216u) * 3.141499996185302734375f;
        float _468 = sqrt(1.0f - (_464 * _464));
        _475 = float3(_468 * cos(_465), _468 * sin(_465), _464) * 0.5f;
    }
    else
    {
        _475 = 0.0f.xxx;
    }
    uint _476 = _432 ? 1065353216u : _263;
    uint _477 = _432 ? 8388607u : _263;
    bool _478 = _432 ? true : false;
    float _582;
    do
    {
        float4 _489 = mul(view_constants[2u].world_to_sample, float4(_422, 1.0f));
        float _490 = _489.z;
        if (_490 > 1.0f)
        {
            _582 = 1.0f;
            break;
        }
        float2 _494 = _489.xy;
        float2 _495 = abs(_494);
        bool _497 = any(bool2(_495.x > 0.98666667938232421875f.xx.x, _495.y > 0.98666667938232421875f.xx.y));
        float2 _501;
        if (_497)
        {
            _501 = _494 * 0.5f;
        }
        else
        {
            _501 = _494;
        }
        float _502 = _497 ? 4.0f : 8.0f;
        float2 _504 = abs(_501);
        float2 _507 = 1.0f.xx - (0.000833333353511989116668701171875f.xx * (2.0f * _502));
        bool _509 = any(bool2(_504.x > _507.x, _504.y > _507.y));
        float _514;
        float2 _515;
        if (_509)
        {
            _514 = _502 * 0.2857142984867095947265625f;
            _515 = _501 * 0.2857142984867095947265625f;
        }
        else
        {
            _514 = _502;
            _515 = _501;
        }
        float _517 = float(_509 ? 2u : uint(_497));
        float2 _520 = (_515.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
        float _531 = float((uint((_327.x * 11) + (_327.y * 7)) + (frame_index * 5u)) % 64u) * 1.61803400516510009765625f;
        float _536 = 5.588237762451171875f * float(frame_index % 64u);
        float _546 = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(_328.x) + _536)) + (0.005837149918079376220703125f * (float(_328.y) + _536))));
        float _548;
        _548 = 0.0f;
        for (int _551 = 0; _551 < 6; )
        {
            float _555 = float(_551);
            float _557 = (_555 + _531) * 2.3999631404876708984375f;
            _548 += exp((-1.0f) * max(0.0f, (_490 - shadow_map.SampleLevel(sampler_shadow_nearest, float3(_520.xy + ((((float2(cos(_557), sin(_557)) * (_555 + _546)) * 0.16666667163372039794921875f.xx) * _514) * 0.000833333353511989116668701171875f.xx), _517), 0.0f).x) * 1024.0f));
            _551++;
            continue;
        }
        _582 = _548 * 0.16666667163372039794921875f;
        break;
    } while(false);
    float _591 = dot(_368, (_422 - _422) + ((normalize(_414) * (((1.0f - _384) * 2.0f) - 1.0f)) * 5.0f));
    float3 _597 = normalize(_368);
    float _601 = lerp(0.20000000298023223876953125f, 1.0f, smoothstep(-0.20000000298023223876953125f, 1.0f, dot(_597, _391))) * 0.800000011920928955078125f;
    float _605 = max(0.0f, 1.0f - abs(dot(_597, _414)));
    float _606 = _605 * _605;
    float _617 = abs(dot(normalize(_368 + _475), normalize(_391 + _414)));
    float _620 = ((_617 * _617) * (-0.5f)) + 1.0f;
    float _625 = max(0.0f, (-0.75f) * dot(_414, _391));
    float4 _321[3] = _269;
    float _641 = _343.x;
    uint _645 = uint((_641 * 3.0f) + 0.5f) & 3u;
    float _647;
    _647 = 0.0f;
    float _648;
    for (uint _650 = 0u; _650 < 3u; _647 = _648, _650++)
    {
        uint2 _662 = (_328 / uint2(2u, 2u)) + uint2(spatial_resolve_offsets_array[(_645 * 16u) + _650].xy);
        float _672 = exp2(-min(30.0f, 80.0f * abs(1.0f - (half_depth_tex.Load(int3(_662, 0u)).x / _348))));
        for (uint _674 = 0u; _674 < 3u; )
        {
            _321[_674] += (indirect_radiance_tex.Load(int3(_662 + uint2((_674 * indirect_radiance_tex_dims.x) / 3u, 0u), 0u)) * _672);
            _674++;
            continue;
        }
        _648 = _647 + _672;
    }
    float _739;
    if (_647 < 0.001000000047497451305389404296875f)
    {
        float _695;
        _695 = _647;
        float _696;
        for (uint _698 = 3u; _698 < 6u; _695 = _696, _698++)
        {
            uint2 _710 = (_328 / uint2(2u, 2u)) + uint2(spatial_resolve_offsets_array[(_645 * 16u) + _698].xy);
            float _720 = exp2(-min(30.0f, 80.0f * abs(1.0f - (half_depth_tex.Load(int3(_710, 0u)).x / _348))));
            for (uint _722 = 0u; _722 < 3u; )
            {
                _321[_722] += (indirect_radiance_tex.Load(int3(_710 + uint2((_722 * indirect_radiance_tex_dims.x) / 3u, 0u), 0u)) * _720);
                _722++;
                continue;
            }
            _696 = _695 + _720;
        }
        _739 = _695;
    }
    else
    {
        _739 = _647;
    }
    float _741 = 1.0f / max(9.9999997473787516355514526367188e-05f, _739);
    _321[0] *= _741;
    _321[1] *= _741;
    _321[2] *= _741;
    float3 _759 = (_321[0].yzw + _321[1].yzw) + _321[2].yzw;
    float3 _767 = float3(_321[0].x, _321[1].x, _321[2].x);
    float3 _770 = _321[0].yzw / max(9.9999996826552253889678874634872e-21f, _321[0].x).xxx;
    float3 _773 = _321[1].yzw / max(9.9999996826552253889678874634872e-21f, _321[1].x).xxx;
    float3 _776 = _321[2].yzw / max(9.9999996826552253889678874634872e-21f, _321[2].x).xxx;
    float3 _782 = max(0.0f.xxx, 1.0f.xxx - (color_library.indirect_light_tint.xyz * 0.920000016689300537109375f));
    float3 _790 = max(0.0f.xxx, lerp((_782 / dot(_782, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f)).xxx) * dot(_767, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f)), _767, 1.16999995708465576171875f.xxx) * 1.2000000476837158203125f);
    uint _839;
    uint _840;
    bool _841;
    float3 _842;
    if (_432)
    {
        uint _794 = uint(_363);
        uint _796 = _794 + (_794 << 10u);
        uint _798 = _796 ^ (_796 >> 6u);
        uint _800 = _798 + (_798 << 3u);
        uint _802 = _800 ^ (_800 >> 11u);
        bool _805 = _478 ? _478 : true;
        uint _806 = _478 ? _477 : 8388607u;
        uint _807 = _478 ? _476 : 1065353216u;
        uint _811 = _794 ^ 188053769u;
        uint _813 = _811 + (_811 << 10u);
        uint _815 = _813 ^ (_813 >> 6u);
        uint _817 = _815 + (_815 << 3u);
        uint _819 = _817 ^ (_817 >> 11u);
        uint _823 = _805 ? _806 : 8388607u;
        uint _824 = _805 ? _807 : 1065353216u;
        float _828 = asfloat(((_819 + (_819 << 15u)) & _823) | _824) - 1.0f;
        float _829 = asfloat(((_802 + (_802 << 15u)) & _806) | _807) * 3.141499996185302734375f;
        float _832 = sqrt(1.0f - (_828 * _828));
        _839 = _824;
        _840 = _823;
        _841 = _805 ? _805 : true;
        _842 = float3(_832 * cos(_829), _832 * sin(_829), _828) * 0.5f;
    }
    else
    {
        _839 = _476;
        _840 = _477;
        _841 = _478;
        _842 = 0.0f.xxx;
    }
    float3 _850 = (-float3(_770.y, _773.y, _776.y)) * _790;
    float3 _855 = float3(_770.z, _773.z, _776.z) * _790;
    float3 _861 = (-float3(_770.x, _773.x, _776.x)) * _790;
    float3 _868 = normalize(float3(-dot(_861, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f)), -dot(_850, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f)), dot(_855, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f))));
    float3 _894 = float3(abs(dot(float3(-_861.x, -_850.x, _855.x), _868)), abs(dot(float3(-_861.y, -_850.y, _855.y), _868)), abs(dot(float3(-_861.z, -_850.z, _855.z), _868))) / _790;
    float _900 = dot(_868, _368);
    float3 _914 = 0.886227548122406005859375f.xxx * _790;
    float3 _921 = (_790 * ((_894 * 0.07999999821186065673828125f) + ((_894 * 0.60000002384185791015625f) * _894))) * 0.25f;
    float _928 = clamp(exp(_591 * (-0.20000000298023223876953125f)), 0.0f, 1.0f);
    float3 _937 = normalize(reflect(_413, normalize(_368 + _842)));
    float _938 = dot(_868, _937);
    float _964 = clamp(dot(normalize(reflect(_413, _368)), _759 / max(9.9999996826552253889678874634872e-21f, length(_759)).xxx), 0.0f, 1.0f);
    float3 _971 = (((((_431 * 0.3183098733425140380859375f) * _601) * sun_color.xyz) * _582) + (((((((lerp(0.20000000298023223876953125f, 0.699999988079071044921875f, _606 * _606).xxx * (0.15915493667125701904296875f / ((3.1415927410125732421875f * _620) * _620))) * lerp(_601, 1.0f, _625)) * lerp(lerp(float(_432), clamp(exp(_591 * (-0.4000000059604644775390625f)), 0.0f, 1.0f), 0.5f), 1.0f, _625)) * sun_color.xyz) * lerp(0.800000011920928955078125f, 0.300000011920928955078125f, smoothstep(50.0f, 150.0f, length(_422.xz)))) * _582) * 1.5f)) + ((((lerp(0.300000011920928955078125f, 1.0f, _928).xxx * 1.0f) * _431) * (max(0.0f.xxx, (((_914 + ((1.0f.xxx * (_368.y * (-1.02332770824432373046875f))) * _850)) + ((1.0f.xxx * (_368.z * 1.02332770824432373046875f)) * _855)) + ((1.0f.xxx * (_368.x * (-1.02332770824432373046875f))) * _861)) + (_921 * (0.3153915703296661376953125f * (((3.0f * _900) * _900) - 1.0f)))) * 1.2999999523162841796875f)) + ((max(0.0f.xxx, (((_914 + ((1.0f.xxx * (_937.y * (-1.02332770824432373046875f))) * _850)) + ((1.0f.xxx * (_937.z * 1.02332770824432373046875f)) * _855)) + ((1.0f.xxx * (_937.x * (-1.02332770824432373046875f))) * _861)) + (_921 * (0.3153915703296661376953125f * (((3.0f * _938) * _938) - 1.0f)))) * (0.039999999105930328369140625f * lerp(_928, (0.75f * _928) + 0.75f, _964 * _964))) * 2.0f));
    float3 _2444;
    if ((lantern_count > 0u) ? true : false)
    {
        uint3 _982 = uint3(clamp(floor((_422 * float3(0.24615384638309478759765625f, 0.123076923191547393798828125f, 0.24615384638309478759765625f)) + float3(16.0f, 2.0f, 16.0f)), 0.0f.xxx, float3(31.0f, 15.0f, 31.0f)));
        uint _989 = (32u * ((32u * _982.y) + _982.z)) + _982.x;
        bool _994 = WaveActiveAllTrue((light_indirection_grid.Load<uint>(_989 * 4 + 0) >> 20u) <= 2u);
        uint _995 = _994 ? 2u : 6u;
        float3 _2443;
        if (6u == _995)
        {
            uint _322[8];
            for (uint _1547 = 0u; _1547 < 8u; )
            {
                _322[_1547] = 4294967295u;
                _1547++;
                continue;
            }
            uint2 _1554 = uint2(_327 & int2(1, 1));
            uint _1559 = (_1554.x + (_1554.y * 2u)) & 3u;
            uint _1561;
            _1561 = 0u;
            uint _1562;
            uint _1564 = 0u;
            for (;;)
            {
                bool _1572;
                if (_1564 < 8u)
                {
                    _1572 = _1561 < 8u;
                }
                else
                {
                    _1572 = false;
                }
                if (_1572)
                {
                    uint4 _1584 = rdi_reservoir_tex.Load(int3(uint2((_327 / int2(2, 2)) + spatial_resolve_offsets_array[((_1559 * 16u) + _1564) + 256u].xy), 0u));
                    uint2 _1585 = _1584.xy;
                    uint _1586 = _1584.x;
                    bool _1592;
                    if (any(bool2(_1585.x != uint2(0u, 0u).x, _1585.y != uint2(0u, 0u).y)))
                    {
                        _1592 = _1586 < lantern_count;
                    }
                    else
                    {
                        _1592 = false;
                    }
                    bool _1598;
                    if (_1592)
                    {
                        _1598 = _322[0u] != _1586;
                    }
                    else
                    {
                        _1598 = false;
                    }
                    bool _1604;
                    if (_1598)
                    {
                        _1604 = _322[1u] != _1586;
                    }
                    else
                    {
                        _1604 = false;
                    }
                    bool _1610;
                    if (_1604)
                    {
                        _1610 = _322[2u] != _1586;
                    }
                    else
                    {
                        _1610 = false;
                    }
                    bool _1616;
                    if (_1610)
                    {
                        _1616 = _322[3u] != _1586;
                    }
                    else
                    {
                        _1616 = false;
                    }
                    bool _1622;
                    if (_1616)
                    {
                        _1622 = _322[4u] != _1586;
                    }
                    else
                    {
                        _1622 = false;
                    }
                    bool _1628;
                    if (_1622)
                    {
                        _1628 = _322[5u] != _1586;
                    }
                    else
                    {
                        _1628 = false;
                    }
                    bool _1634;
                    if (_1628)
                    {
                        _1634 = _322[6u] != _1586;
                    }
                    else
                    {
                        _1634 = false;
                    }
                    bool _1640;
                    if (_1634)
                    {
                        _1640 = _322[7u] != _1586;
                    }
                    else
                    {
                        _1640 = false;
                    }
                    if (_1640)
                    {
                        _322[_1561] = _1586;
                        _1562 = _1561 + 1u;
                    }
                    else
                    {
                        _1562 = _1561;
                    }
                    _1561 = _1562;
                    _1564++;
                    continue;
                }
                else
                {
                    break;
                }
            }
            uint _1646;
            _1646 = _1561;
            uint _1647;
            for (uint _1649 = 0u; _1649 < 8u; _1646 = _1647, _1649++)
            {
                uint _1656 = QuadReadAcrossDiagonal(_322[_1649]);
                bool _1661;
                if (_1656 < lantern_count)
                {
                    _1661 = _1646 < 8u;
                }
                else
                {
                    _1661 = false;
                }
                if (_1661)
                {
                    bool _1672;
                    if (_322[0u] != _1656)
                    {
                        _1672 = _322[1u] != _1656;
                    }
                    else
                    {
                        _1672 = false;
                    }
                    bool _1678;
                    if (_1672)
                    {
                        _1678 = _322[2u] != _1656;
                    }
                    else
                    {
                        _1678 = false;
                    }
                    bool _1684;
                    if (_1678)
                    {
                        _1684 = _322[3u] != _1656;
                    }
                    else
                    {
                        _1684 = false;
                    }
                    bool _1690;
                    if (_1684)
                    {
                        _1690 = _322[4u] != _1656;
                    }
                    else
                    {
                        _1690 = false;
                    }
                    bool _1696;
                    if (_1690)
                    {
                        _1696 = _322[5u] != _1656;
                    }
                    else
                    {
                        _1696 = false;
                    }
                    bool _1702;
                    if (_1696)
                    {
                        _1702 = _322[6u] != _1656;
                    }
                    else
                    {
                        _1702 = false;
                    }
                    bool _1708;
                    if (_1702)
                    {
                        _1708 = _322[7u] != _1656;
                    }
                    else
                    {
                        _1708 = false;
                    }
                    uint _1713;
                    if (_1708)
                    {
                        _322[_1646] = _1656;
                        _1713 = _1646 + 1u;
                    }
                    else
                    {
                        _1713 = _1646;
                    }
                    _1647 = _1713;
                }
                else
                {
                    _1647 = _1646;
                }
            }
            Reservoir1sppStreamState _323[2];
            Reservoir1spp _324[2];
            for (uint _1715 = 0u; _1715 < 2u; )
            {
                _323[_1715] = _303;
                _324[_1715] = _304;
                _1715++;
                continue;
            }
            for (uint _1723 = 0u; _1723 < _1646; _1723++)
            {
                float2 _1736 = spvUnpackHalf2x16(lantern_buf.Load<uint>(_322[_1723] * 16 + 12) & 65535u);
                float _1737 = _1736.x;
                uint _1739 = lantern_buf.Load<uint>(_322[_1723] * 16 + 12) >> 16u;
                float3 _1740 = lantern_buf.Load<float3>(_322[_1723] * 16 + 0) - _422;
                float _1742 = dot(_1740, _1740);
                float3 _1749 = _1740 + float3(0.0f, 0.1500000059604644775390625f, 0.0f);
                float3 _1794;
                do
                {
                    if (_1739 == 0u)
                    {
                        _1794 = float3(0.7919800281524658203125f, 0.63358402252197265625f, 0.07919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.7919800281524658203125f).xxx;
                        break;
                    }
                    else
                    {
                        if (_1739 == 1u)
                        {
                            _1794 = float3(0.0f, 0.1583960056304931640625f, 0.7919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.215240001678466796875f).xxx;
                            break;
                        }
                        else
                        {
                            if (_1739 == 2u)
                            {
                                _1794 = float3(0.1583960056304931640625f, 0.7919800281524658203125f, 0.1583960056304931640625f) / max(1.0000000133514319600180897396058e-10f, 0.772159993648529052734375f).xxx;
                                break;
                            }
                            else
                            {
                                if (_1739 == 3u)
                                {
                                    _1794 = float3(0.23759402334690093994140625f, 0.1583960056304931640625f, 0.7919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.279020011425018310546875f).xxx;
                                    break;
                                }
                                else
                                {
                                    _1794 = float3(0.7919800281524658203125f, 0.23759402334690093994140625f, 0.07919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.4343800246715545654296875f).xxx;
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
                float _1798 = dot(((_1794 * 4.0f) * _1737) * ((max(0.0f, dot(_368, _1740 / length(_1740).xxx)) * ((1.0f / (0.3359999954700469970703125f + _1742)) * smoothstep(36.0f * _1737, 16.0f * _1737, _1742))) * smoothstep(-0.5f, 0.20000000298023223876953125f, (_1749 / length(_1749).xxx).y)), float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f));
                float _1799 = float(_1646);
                float _1803 = frac(_641 + (float(_1723) / _1799));
                if (_324[0].w_sum <= _324[1].w_sum)
                {
                    do
                    {
                        _323[0].M_sum += 1.0f;
                        float _1842 = _1798 * _1799;
                        bool _1857;
                        do
                        {
                            _324[0].w_sum += _1842;
                            _324[0].M += 1.0f;
                            if ((_1842 / _324[0].w_sum) >= _1803)
                            {
                                _324[0].payload = _322[_1723];
                                _1857 = true;
                                break;
                            }
                            else
                            {
                                _1857 = false;
                                break;
                            }
                            break; 
                        } while(false);
                        if (_1857)
                        {
                            _323[0].p_q_sel = _1798;
                            break;
                        }
                        else
                        {
                            break;
                        }
                        break; 
                    } while(false);
                }
                else
                {
                    do
                    {
                        _323[1].M_sum += 1.0f;
                        float _1817 = _1798 * _1799;
                        bool _1832;
                        do
                        {
                            _324[1].w_sum += _1817;
                            _324[1].M += 1.0f;
                            if ((_1817 / _324[1].w_sum) >= _1803)
                            {
                                _324[1].payload = _322[_1723];
                                _1832 = true;
                                break;
                            }
                            else
                            {
                                _1832 = false;
                                break;
                            }
                            break; 
                        } while(false);
                        if (_1832)
                        {
                            _323[1].p_q_sel = _1798;
                            break;
                        }
                        else
                        {
                            break;
                        }
                        break; 
                    } while(false);
                }
            }
            for (uint _1863 = 1u; _1863 < 2u; )
            {
                _323[0].M_sum += _323[_1863].M_sum;
                _1863++;
                continue;
            }
            for (uint _1874 = 1u; _1874 < 2u; )
            {
                _323[_1874].M_sum = _323[0].M_sum;
                _1874++;
                continue;
            }
            float3 _1890;
            _1890 = _971;
            float _1884;
            float _1887;
            float _1889;
            float3 _1891;
            uint _1895;
            uint _1897;
            bool _1899;
            bool _1901;
            float _1883 = 0.0f;
            float _1886 = 0.0f;
            float _1888 = 0.0f;
            uint _1892 = 0u;
            uint _1894 = _839;
            uint _1896 = _840;
            bool _1898 = _841;
            bool _1900 = _841;
            for (; _1892 < 2u; _1883 = _1884, _1886 = _1887, _1888 = _1889, _1890 = _1891, _1892++, _1894 = _1895, _1896 = _1897, _1898 = _1899, _1900 = _1901)
            {
                _324[_1892].M = _323[_1892].M_sum;
                _324[_1892].W = _324[_1892].w_sum / max(9.9999999392252902907785028219223e-09f, _324[_1892].M * _323[_1892].p_q_sel);
                if (_324[_1892].W > 0.0f)
                {
                    float2 _1928 = spvUnpackHalf2x16(lantern_buf.Load<uint>(_324[_1892].payload * 16 + 12) & 65535u);
                    float _1929 = _1928.x;
                    uint _1931 = lantern_buf.Load<uint>(_324[_1892].payload * 16 + 12) >> 16u;
                    float3 _1932 = lantern_buf.Load<float3>(_324[_1892].payload * 16 + 0) - _422;
                    float _1933 = length(_1932);
                    float _1934 = dot(_1932, _1932);
                    float3 _1941 = _1932 + float3(0.0f, 0.1500000059604644775390625f, 0.0f);
                    float3 _1947 = _1933.xxx;
                    float3 _1986;
                    do
                    {
                        if (_1931 == 0u)
                        {
                            _1986 = float3(0.7919800281524658203125f, 0.63358402252197265625f, 0.07919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.7919800281524658203125f).xxx;
                            break;
                        }
                        else
                        {
                            if (_1931 == 1u)
                            {
                                _1986 = float3(0.0f, 0.1583960056304931640625f, 0.7919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.215240001678466796875f).xxx;
                                break;
                            }
                            else
                            {
                                if (_1931 == 2u)
                                {
                                    _1986 = float3(0.1583960056304931640625f, 0.7919800281524658203125f, 0.1583960056304931640625f) / max(1.0000000133514319600180897396058e-10f, 0.772159993648529052734375f).xxx;
                                    break;
                                }
                                else
                                {
                                    if (_1931 == 3u)
                                    {
                                        _1986 = float3(0.23759402334690093994140625f, 0.1583960056304931640625f, 0.7919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.279020011425018310546875f).xxx;
                                        break;
                                    }
                                    else
                                    {
                                        _1986 = float3(0.7919800281524658203125f, 0.23759402334690093994140625f, 0.07919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.4343800246715545654296875f).xxx;
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
                    float3 _1989 = ((_1986 * 4.0f) * _1929) * ((max(0.0f, dot(_368, _1932 / _1947)) * ((1.0f / (0.3359999954700469970703125f + _1934)) * smoothstep(36.0f * _1929, 16.0f * _1929, _1934))) * smoothstep(-0.5f, 0.20000000298023223876953125f, (_1941 / length(_1941).xxx).y));
                    float _2434;
                    float _2435;
                    float _2436;
                    uint _2437;
                    uint _2438;
                    bool _2439;
                    bool _2440;
                    float3 _2441;
                    if ((_324[_1892].W * max(max(_1989.x, _1989.y), _1989.z)) > 0.0f)
                    {
                        float _2249;
                        float _2250;
                        float _2251;
                        float3 _2252;
                        if (_1933 > 0.5f)
                        {
                            float3 _2009 = (_422 + ((_1932 * (_1933 - 0.5f)) / _1947)) + (_368 * (0.0199999995529651641845703125f * _343.y));
                            float3 _2010 = _409.xyz;
                            float4 _2020 = mul(view_constants[0u].view_to_sample, mul(view_constants[0u].world_to_view, float4(_2009, 1.0f)));
                            float3 _2024 = _2020.xyz / _2020.w.xxx;
                            float4 _2035 = _409 + (float4(_2024 - _2010, 0.0f) * float(int(sign(_2024.z))));
                            float _2036 = _2035.w;
                            float3 _2045 = (_2035 / (float((_2036 >= 0.0f) ? 1 : (-1)) * max(1.0000000133514319600180897396058e-10f, abs(_2036))).xxxx).xyz;
                            float3 _2046 = _2045 - _2010;
                            bool3 _2047 = bool3(_2046.x < 0.0f.xxx.x, _2046.y < 0.0f.xxx.y, _2046.z < 0.0f.xxx.z);
                            float3 _2050 = (float3(_2047.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, _2047.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, _2047.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - _2010) / _2046;
                            float3 _2056 = _2010 + (_2046 * max(0.0f, max(_2050.x, _2050.y)));
                            float3 _2057 = _2045 - _2056;
                            bool3 _2058 = bool3(_2057.x >= 0.0f.xxx.x, _2057.y >= 0.0f.xxx.y, _2057.z >= 0.0f.xxx.z);
                            float3 _2061 = (float3(_2058.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, _2058.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, _2058.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - _2056) / _2057;
                            float3 _2069 = _2056 + (_2057 * min(1.0f, min(min(_2061.x, _2061.y), _2061.z)));
                            float _2201;
                            float _2202;
                            float _2203;
                            bool _2216;
                            float _2217;
                            float _2218;
                            do
                            {
                                float _2090 = 0.800000011920928955078125f * view_constants[0u].clip_to_view[3].z;
                                uint _2091 = uint(int(max(2u, min(4u, uint(int(floor(length((((_2069.xy * float2(0.5f, -0.5f)) + 0.5f.xx) - ((_2056.xy * float2(0.5f, -0.5f)) + 0.5f.xx)) * _394))))))));
                                float _2196;
                                float _2200;
                                bool _2204;
                                do
                                {
                                    float3 _2096 = _2069 - _2056;
                                    float _2104;
                                    if (_2069.z > 9.5367431640625e-07f)
                                    {
                                        _2104 = float(_2091);
                                    }
                                    else
                                    {
                                        _2104 = max(1.0f, float(_2091) - 1.0f);
                                    }
                                    float _2105 = 1.0f / _2104;
                                    float _2191;
                                    float _2192;
                                    float _2193;
                                    float _2194;
                                    bool _2195;
                                    if (_2091 > 0u)
                                    {
                                        float _2111 = lerp(0.0f, 1.0f, pow(_641 * _2105, 1.0f));
                                        float3 _2113 = _2056 + (_2096 * _2111);
                                        float2 _2116 = (_2113.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                                        float _2118 = 1.0f / _2113.z;
                                        float _2123 = 1.0f / depth_tex.SampleLevel(sampler_llc, _2116, 0.0f).x;
                                        float _2128 = 1.0f / depth_tex.SampleLevel(sampler_nnc, _2116, 0.0f).x;
                                        float _2132 = (max(_2123, _2128) * 1.00000202655792236328125f) - _2118;
                                        float _2133 = _2118 - min(_2123, _2128);
                                        bool _2136 = (_2132 < 0.0f) ? (_2133 < _2090) : false;
                                        float _2185;
                                        float _2186;
                                        float _2187;
                                        float _2188;
                                        bool _2189;
                                        float _2190;
                                        if (_2136)
                                        {
                                            _2185 = _2132;
                                            _2186 = _2133;
                                            _2187 = _2090;
                                            _2188 = _2111;
                                            _2189 = _2136;
                                            _2190 = 0.0f;
                                        }
                                        else
                                        {
                                            float _2146;
                                            _2146 = _2111;
                                            bool _2145;
                                            float _2147;
                                            float _2180;
                                            float _2181;
                                            float _2182;
                                            float _2183;
                                            bool _2184;
                                            uint _2141 = 1u;
                                            bool _2144 = _2136;
                                            for (;;)
                                            {
                                                if (_2141 < _2091)
                                                {
                                                    _2147 = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(_2141) + _641) * _2105, 1.0f)));
                                                    float3 _2157 = _2056 + (_2096 * _2147);
                                                    float2 _2160 = (_2157.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                                                    float _2162 = 1.0f / _2157.z;
                                                    float _2166 = 1.0f / depth_tex.SampleLevel(sampler_llc, _2160, 0.0f).x;
                                                    float _2170 = 1.0f / depth_tex.SampleLevel(sampler_nnc, _2160, 0.0f).x;
                                                    float _2174 = (max(_2166, _2170) * 1.00000202655792236328125f) - _2162;
                                                    float _2175 = _2162 - min(_2166, _2170);
                                                    _2145 = (_2174 < 0.0f) ? (_2175 < _2090) : false;
                                                    if (_2145)
                                                    {
                                                        _2180 = _2174;
                                                        _2181 = _2175;
                                                        _2182 = _2090;
                                                        _2183 = _2147;
                                                        _2184 = _2145;
                                                        break;
                                                    }
                                                    _2141++;
                                                    _2144 = _2145;
                                                    _2146 = _2147;
                                                    continue;
                                                }
                                                else
                                                {
                                                    _2180 = 0.0f;
                                                    _2181 = 0.0f;
                                                    _2182 = 0.0f;
                                                    _2183 = 1.0f;
                                                    _2184 = _2144;
                                                    break;
                                                }
                                            }
                                            _2185 = _2180;
                                            _2186 = _2181;
                                            _2187 = _2182;
                                            _2188 = _2183;
                                            _2189 = _2184;
                                            _2190 = _2146;
                                        }
                                        _2191 = _2185;
                                        _2192 = _2186;
                                        _2193 = _2187;
                                        _2194 = _2188;
                                        _2195 = _2189;
                                        _2196 = _2190;
                                    }
                                    else
                                    {
                                        _2191 = 0.0f;
                                        _2192 = 0.0f;
                                        _2193 = 0.0f;
                                        _2194 = 1.0f;
                                        _2195 = false;
                                        _2196 = 0.0f;
                                    }
                                    if (_2195)
                                    {
                                        _2200 = _2194;
                                        _2201 = _2191;
                                        _2202 = _2193;
                                        _2203 = _2192;
                                        _2204 = true;
                                        break;
                                    }
                                    else
                                    {
                                        _2200 = _2196;
                                        _2201 = _1883;
                                        _2202 = _1886;
                                        _2203 = _1888;
                                        _2204 = false;
                                        break;
                                    }
                                    break; 
                                } while(false);
                                bool _2212;
                                if (_2204)
                                {
                                    bool _2211;
                                    if (_2203 < _2202)
                                    {
                                        _2211 = _2201 < _2202;
                                    }
                                    else
                                    {
                                        _2211 = false;
                                    }
                                    _2212 = _2211;
                                }
                                else
                                {
                                    _2212 = false;
                                }
                                if (_2212)
                                {
                                    _2216 = true;
                                    _2217 = _2200;
                                    _2218 = _2203 / view_constants[0u].clip_to_view[3].z;
                                    break;
                                }
                                _2216 = false;
                                _2217 = _2196;
                                _2218 = 0.0f;
                                break;
                            } while(false);
                            float3 _2248;
                            if (_2216)
                            {
                                float4 _2228 = mul(view_constants[0u].view_to_world, mul(view_constants[0u].sample_to_view, float4(lerp(_2056, _2069, _2217.xxx), 1.0f)));
                                float3 _2235 = ((_2228.xyz / _2228.w.xxx) + (_414 * _2218)) - lantern_buf.Load<float3>(_324[_1892].payload * 16 + 0);
                                bool _2245;
                                if (!(abs(_2235.y) > 0.5f))
                                {
                                    float2 _2242 = _2235.xz;
                                    _2245 = dot(_2242, _2242) > 0.136900007724761962890625f;
                                }
                                else
                                {
                                    _2245 = true;
                                }
                                bool3 _2246 = _2245.xxx;
                                _2248 = float3(_2246.x ? 0.0f.xxx.x : _1989.x, _2246.y ? 0.0f.xxx.y : _1989.y, _2246.z ? 0.0f.xxx.z : _1989.z);
                            }
                            else
                            {
                                _2248 = _1989;
                            }
                            _2249 = _2201;
                            _2250 = _2202;
                            _2251 = _2203;
                            _2252 = _2248;
                        }
                        else
                        {
                            _2249 = _1883;
                            _2250 = _1886;
                            _2251 = _1888;
                            _2252 = _1989;
                        }
                        float2 _2262 = float2((float(game_time_sec) + game_time_frac) * 4.0f, float(_324[_1892].payload * 10u));
                        int2 _2264 = int2(floor(_2262));
                        float2 _2265 = frac(_2262);
                        uint2 _2266 = uint2(_2264);
                        uint _2268 = _2266.y;
                        uint _2270 = _2268 + (_2268 << 10u);
                        uint _2272 = _2270 ^ (_2270 >> 6u);
                        uint _2274 = _2272 + (_2272 << 3u);
                        uint _2276 = _2274 ^ (_2274 >> 11u);
                        uint _2282 = (((_2266.x * 1664525u) + (_2276 + (_2276 << 15u))) + 1013904223u) * 1664525u;
                        uint _2284 = _2282 ^ (_2282 >> 11u);
                        uint _2287 = _2284 ^ ((_2284 << 7u) & 2636928640u);
                        uint _2290 = _2287 ^ ((_2287 << 15u) & 4022730752u);
                        bool _2293 = _1900 ? _1900 : true;
                        uint _2294 = _1900 ? _1896 : 8388607u;
                        bool _2295 = _1898 ? _1898 : true;
                        uint _2296 = _1898 ? _1894 : 1065353216u;
                        uint2 _2302 = uint2(_2264 + int2(1, 0));
                        uint _2304 = _2302.y;
                        uint _2306 = _2304 + (_2304 << 10u);
                        uint _2308 = _2306 ^ (_2306 >> 6u);
                        uint _2310 = _2308 + (_2308 << 3u);
                        uint _2312 = _2310 ^ (_2310 >> 11u);
                        uint _2318 = (((_2302.x * 1664525u) + (_2312 + (_2312 << 15u))) + 1013904223u) * 1664525u;
                        uint _2320 = _2318 ^ (_2318 >> 11u);
                        uint _2323 = _2320 ^ ((_2320 << 7u) & 2636928640u);
                        uint _2326 = _2323 ^ ((_2323 << 15u) & 4022730752u);
                        bool _2329 = _2293 ? _2293 : true;
                        uint _2330 = _2293 ? _2294 : 8388607u;
                        bool _2331 = _2295 ? _2295 : true;
                        uint _2332 = _2295 ? _2296 : 1065353216u;
                        uint2 _2338 = uint2(_2264 + int2(0, 1));
                        uint _2340 = _2338.y;
                        uint _2342 = _2340 + (_2340 << 10u);
                        uint _2344 = _2342 ^ (_2342 >> 6u);
                        uint _2346 = _2344 + (_2344 << 3u);
                        uint _2348 = _2346 ^ (_2346 >> 11u);
                        uint _2354 = (((_2338.x * 1664525u) + (_2348 + (_2348 << 15u))) + 1013904223u) * 1664525u;
                        uint _2356 = _2354 ^ (_2354 >> 11u);
                        uint _2359 = _2356 ^ ((_2356 << 7u) & 2636928640u);
                        uint _2362 = _2359 ^ ((_2359 << 15u) & 4022730752u);
                        bool _2365 = _2329 ? _2329 : true;
                        uint _2366 = _2329 ? _2330 : 8388607u;
                        bool _2367 = _2331 ? _2331 : true;
                        uint _2368 = _2331 ? _2332 : 1065353216u;
                        uint2 _2374 = uint2(_2264 + int2(1, 1));
                        uint _2376 = _2374.y;
                        uint _2378 = _2376 + (_2376 << 10u);
                        uint _2380 = _2378 ^ (_2378 >> 6u);
                        uint _2382 = _2380 + (_2380 << 3u);
                        uint _2384 = _2382 ^ (_2382 >> 11u);
                        uint _2390 = (((_2374.x * 1664525u) + (_2384 + (_2384 << 15u))) + 1013904223u) * 1664525u;
                        uint _2392 = _2390 ^ (_2390 >> 11u);
                        uint _2395 = _2392 ^ ((_2392 << 7u) & 2636928640u);
                        uint _2398 = _2395 ^ ((_2395 << 15u) & 4022730752u);
                        uint _2402 = _2365 ? _2366 : 8388607u;
                        uint _2404 = _2367 ? _2368 : 1065353216u;
                        float2 _2412 = (_2265 * _2265) * (3.0f.xx - (_2265 * 2.0f));
                        float _2413 = _2412.x;
                        float _2414 = 1.0f - _2413;
                        float _2416 = _2412.y;
                        _2434 = _2249;
                        _2435 = _2250;
                        _2436 = _2251;
                        _2437 = _2404;
                        _2438 = _2402;
                        _2439 = _2367 ? _2367 : true;
                        _2440 = _2365 ? _2365 : true;
                        _2441 = (((_2252 * lerp(0.5f, 1.0f, (((1.0f - _2416) * (((asfloat(((_2290 ^ (_2290 >> 18u)) & _2294) | _2296) - 1.0f) * _2414) + ((asfloat(((_2326 ^ (_2326 >> 18u)) & _2330) | _2332) - 1.0f) * _2413))) + (((asfloat(((_2362 ^ (_2362 >> 18u)) & _2366) | _2368) - 1.0f) * _2414) * _2416)) + (((asfloat(((_2398 ^ (_2398 >> 18u)) & _2402) | _2404) - 1.0f) * _2413) * _2416))) * _324[_1892].W) * _431) * ev_mult;
                    }
                    else
                    {
                        _2434 = _1883;
                        _2435 = _1886;
                        _2436 = _1888;
                        _2437 = _1894;
                        _2438 = _1896;
                        _2439 = _1898;
                        _2440 = _1900;
                        _2441 = 0.0f.xxx;
                    }
                    _1884 = _2434;
                    _1887 = _2435;
                    _1889 = _2436;
                    _1891 = _1890 + _2441;
                    _1895 = _2437;
                    _1897 = _2438;
                    _1899 = _2439;
                    _1901 = _2440;
                }
                else
                {
                    _1884 = _1883;
                    _1887 = _1886;
                    _1889 = _1888;
                    _1891 = _1890;
                    _1895 = _1894;
                    _1897 = _1896;
                    _1899 = _1898;
                    _1901 = _1900;
                }
            }
            _2443 = _1890;
        }
        else
        {
            float3 _1545;
            if (2u == _995)
            {
                uint _1005 = light_indirection_grid.Load<uint>(_989 * 4 + 0) & 1048575u;
                uint _1007 = min(64u, (light_indirection_grid.Load<uint>(_989 * 4 + 0) >> 20u));
                float3 _1009;
                _1009 = _971;
                float3 _1010;
                float _1013;
                float _1015;
                float _1017;
                uint _1021;
                uint _1023;
                bool _1025;
                bool _1027;
                float _1012 = 0.0f;
                float _1014 = 0.0f;
                float _1016 = 0.0f;
                uint _1018 = 0u;
                uint _1020 = _839;
                uint _1022 = _840;
                bool _1024 = _841;
                bool _1026 = _841;
                for (; _1018 < _1007; _1009 = _1010, _1012 = _1013, _1014 = _1015, _1016 = _1017, _1018++, _1020 = _1021, _1022 = _1023, _1024 = _1025, _1026 = _1027)
                {
                    uint _1031 = _1005 + _1018;
                    float2 _1039 = spvUnpackHalf2x16(lantern_buf.Load<uint>(grid_light_indices.Load<uint>(_1031 * 4 + 0) * 16 + 12) & 65535u);
                    float _1040 = _1039.x;
                    uint _1042 = lantern_buf.Load<uint>(grid_light_indices.Load<uint>(_1031 * 4 + 0) * 16 + 12) >> 16u;
                    float3 _1043 = lantern_buf.Load<float3>(grid_light_indices.Load<uint>(_1031 * 4 + 0) * 16 + 0) - _422;
                    float _1044 = length(_1043);
                    float _1045 = dot(_1043, _1043);
                    float3 _1052 = _1043 + float3(0.0f, 0.1500000059604644775390625f, 0.0f);
                    float3 _1058 = _1044.xxx;
                    float3 _1097;
                    do
                    {
                        if (_1042 == 0u)
                        {
                            _1097 = float3(0.7919800281524658203125f, 0.63358402252197265625f, 0.07919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.7919800281524658203125f).xxx;
                            break;
                        }
                        else
                        {
                            if (_1042 == 1u)
                            {
                                _1097 = float3(0.0f, 0.1583960056304931640625f, 0.7919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.215240001678466796875f).xxx;
                                break;
                            }
                            else
                            {
                                if (_1042 == 2u)
                                {
                                    _1097 = float3(0.1583960056304931640625f, 0.7919800281524658203125f, 0.1583960056304931640625f) / max(1.0000000133514319600180897396058e-10f, 0.772159993648529052734375f).xxx;
                                    break;
                                }
                                else
                                {
                                    if (_1042 == 3u)
                                    {
                                        _1097 = float3(0.23759402334690093994140625f, 0.1583960056304931640625f, 0.7919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.279020011425018310546875f).xxx;
                                        break;
                                    }
                                    else
                                    {
                                        _1097 = float3(0.7919800281524658203125f, 0.23759402334690093994140625f, 0.07919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.4343800246715545654296875f).xxx;
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
                    float3 _1100 = ((_1097 * 4.0f) * _1040) * ((max(0.0f, dot(_368, _1043 / _1058)) * ((1.0f / (0.3359999954700469970703125f + _1045)) * smoothstep(36.0f * _1040, 16.0f * _1040, _1045))) * smoothstep(-0.5f, 0.20000000298023223876953125f, (_1052 / length(_1052).xxx).y));
                    float3 _1544;
                    if (max(max(_1100.x, _1100.y), _1100.z) > 0.0f)
                    {
                        float _1359;
                        float _1360;
                        float _1361;
                        float3 _1362;
                        if (_1044 > 0.5f)
                        {
                            float3 _1119 = (_422 + ((_1043 * (_1044 - 0.5f)) / _1058)) + (_368 * (0.0199999995529651641845703125f * _343.y));
                            float3 _1120 = _409.xyz;
                            float4 _1130 = mul(view_constants[0u].view_to_sample, mul(view_constants[0u].world_to_view, float4(_1119, 1.0f)));
                            float3 _1134 = _1130.xyz / _1130.w.xxx;
                            float4 _1145 = _409 + (float4(_1134 - _1120, 0.0f) * float(int(sign(_1134.z))));
                            float _1146 = _1145.w;
                            float3 _1155 = (_1145 / (float((_1146 >= 0.0f) ? 1 : (-1)) * max(1.0000000133514319600180897396058e-10f, abs(_1146))).xxxx).xyz;
                            float3 _1156 = _1155 - _1120;
                            bool3 _1157 = bool3(_1156.x < 0.0f.xxx.x, _1156.y < 0.0f.xxx.y, _1156.z < 0.0f.xxx.z);
                            float3 _1160 = (float3(_1157.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, _1157.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, _1157.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - _1120) / _1156;
                            float3 _1166 = _1120 + (_1156 * max(0.0f, max(_1160.x, _1160.y)));
                            float3 _1167 = _1155 - _1166;
                            bool3 _1168 = bool3(_1167.x >= 0.0f.xxx.x, _1167.y >= 0.0f.xxx.y, _1167.z >= 0.0f.xxx.z);
                            float3 _1171 = (float3(_1168.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, _1168.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, _1168.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - _1166) / _1167;
                            float3 _1179 = _1166 + (_1167 * min(1.0f, min(min(_1171.x, _1171.y), _1171.z)));
                            float _1311;
                            float _1312;
                            float _1313;
                            bool _1326;
                            float _1327;
                            float _1328;
                            do
                            {
                                float _1200 = 0.800000011920928955078125f * view_constants[0u].clip_to_view[3].z;
                                uint _1201 = uint(int(max(2u, min(4u, uint(int(floor(length((((_1179.xy * float2(0.5f, -0.5f)) + 0.5f.xx) - ((_1166.xy * float2(0.5f, -0.5f)) + 0.5f.xx)) * _394))))))));
                                float _1306;
                                float _1310;
                                bool _1314;
                                do
                                {
                                    float3 _1206 = _1179 - _1166;
                                    float _1214;
                                    if (_1179.z > 9.5367431640625e-07f)
                                    {
                                        _1214 = float(_1201);
                                    }
                                    else
                                    {
                                        _1214 = max(1.0f, float(_1201) - 1.0f);
                                    }
                                    float _1215 = 1.0f / _1214;
                                    float _1301;
                                    float _1302;
                                    float _1303;
                                    float _1304;
                                    bool _1305;
                                    if (_1201 > 0u)
                                    {
                                        float _1221 = lerp(0.0f, 1.0f, pow(_641 * _1215, 1.0f));
                                        float3 _1223 = _1166 + (_1206 * _1221);
                                        float2 _1226 = (_1223.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                                        float _1228 = 1.0f / _1223.z;
                                        float _1233 = 1.0f / depth_tex.SampleLevel(sampler_llc, _1226, 0.0f).x;
                                        float _1238 = 1.0f / depth_tex.SampleLevel(sampler_nnc, _1226, 0.0f).x;
                                        float _1242 = (max(_1233, _1238) * 1.00000202655792236328125f) - _1228;
                                        float _1243 = _1228 - min(_1233, _1238);
                                        bool _1246 = (_1242 < 0.0f) ? (_1243 < _1200) : false;
                                        float _1295;
                                        float _1296;
                                        float _1297;
                                        float _1298;
                                        bool _1299;
                                        float _1300;
                                        if (_1246)
                                        {
                                            _1295 = _1242;
                                            _1296 = _1243;
                                            _1297 = _1200;
                                            _1298 = _1221;
                                            _1299 = _1246;
                                            _1300 = 0.0f;
                                        }
                                        else
                                        {
                                            float _1256;
                                            _1256 = _1221;
                                            bool _1255;
                                            float _1257;
                                            float _1290;
                                            float _1291;
                                            float _1292;
                                            float _1293;
                                            bool _1294;
                                            uint _1251 = 1u;
                                            bool _1254 = _1246;
                                            for (;;)
                                            {
                                                if (_1251 < _1201)
                                                {
                                                    _1257 = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(_1251) + _641) * _1215, 1.0f)));
                                                    float3 _1267 = _1166 + (_1206 * _1257);
                                                    float2 _1270 = (_1267.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                                                    float _1272 = 1.0f / _1267.z;
                                                    float _1276 = 1.0f / depth_tex.SampleLevel(sampler_llc, _1270, 0.0f).x;
                                                    float _1280 = 1.0f / depth_tex.SampleLevel(sampler_nnc, _1270, 0.0f).x;
                                                    float _1284 = (max(_1276, _1280) * 1.00000202655792236328125f) - _1272;
                                                    float _1285 = _1272 - min(_1276, _1280);
                                                    _1255 = (_1284 < 0.0f) ? (_1285 < _1200) : false;
                                                    if (_1255)
                                                    {
                                                        _1290 = _1284;
                                                        _1291 = _1285;
                                                        _1292 = _1200;
                                                        _1293 = _1257;
                                                        _1294 = _1255;
                                                        break;
                                                    }
                                                    _1251++;
                                                    _1254 = _1255;
                                                    _1256 = _1257;
                                                    continue;
                                                }
                                                else
                                                {
                                                    _1290 = 0.0f;
                                                    _1291 = 0.0f;
                                                    _1292 = 0.0f;
                                                    _1293 = 1.0f;
                                                    _1294 = _1254;
                                                    break;
                                                }
                                            }
                                            _1295 = _1290;
                                            _1296 = _1291;
                                            _1297 = _1292;
                                            _1298 = _1293;
                                            _1299 = _1294;
                                            _1300 = _1256;
                                        }
                                        _1301 = _1295;
                                        _1302 = _1296;
                                        _1303 = _1297;
                                        _1304 = _1298;
                                        _1305 = _1299;
                                        _1306 = _1300;
                                    }
                                    else
                                    {
                                        _1301 = 0.0f;
                                        _1302 = 0.0f;
                                        _1303 = 0.0f;
                                        _1304 = 1.0f;
                                        _1305 = false;
                                        _1306 = 0.0f;
                                    }
                                    if (_1305)
                                    {
                                        _1310 = _1304;
                                        _1311 = _1301;
                                        _1312 = _1303;
                                        _1313 = _1302;
                                        _1314 = true;
                                        break;
                                    }
                                    else
                                    {
                                        _1310 = _1306;
                                        _1311 = _1012;
                                        _1312 = _1014;
                                        _1313 = _1016;
                                        _1314 = false;
                                        break;
                                    }
                                    break; 
                                } while(false);
                                bool _1322;
                                if (_1314)
                                {
                                    bool _1321;
                                    if (_1313 < _1312)
                                    {
                                        _1321 = _1311 < _1312;
                                    }
                                    else
                                    {
                                        _1321 = false;
                                    }
                                    _1322 = _1321;
                                }
                                else
                                {
                                    _1322 = false;
                                }
                                if (_1322)
                                {
                                    _1326 = true;
                                    _1327 = _1310;
                                    _1328 = _1313 / view_constants[0u].clip_to_view[3].z;
                                    break;
                                }
                                _1326 = false;
                                _1327 = _1306;
                                _1328 = 0.0f;
                                break;
                            } while(false);
                            float3 _1358;
                            if (_1326)
                            {
                                float4 _1338 = mul(view_constants[0u].view_to_world, mul(view_constants[0u].sample_to_view, float4(lerp(_1166, _1179, _1327.xxx), 1.0f)));
                                float3 _1345 = ((_1338.xyz / _1338.w.xxx) + (_414 * _1328)) - lantern_buf.Load<float3>(grid_light_indices.Load<uint>(_1031 * 4 + 0) * 16 + 0);
                                bool _1355;
                                if (!(abs(_1345.y) > 0.5f))
                                {
                                    float2 _1352 = _1345.xz;
                                    _1355 = dot(_1352, _1352) > 0.136900007724761962890625f;
                                }
                                else
                                {
                                    _1355 = true;
                                }
                                bool3 _1356 = _1355.xxx;
                                _1358 = float3(_1356.x ? 0.0f.xxx.x : _1100.x, _1356.y ? 0.0f.xxx.y : _1100.y, _1356.z ? 0.0f.xxx.z : _1100.z);
                            }
                            else
                            {
                                _1358 = _1100;
                            }
                            _1359 = _1311;
                            _1360 = _1312;
                            _1361 = _1313;
                            _1362 = _1358;
                        }
                        else
                        {
                            _1359 = _1012;
                            _1360 = _1014;
                            _1361 = _1016;
                            _1362 = _1100;
                        }
                        float2 _1372 = float2((float(game_time_sec) + game_time_frac) * 4.0f, float(grid_light_indices.Load<uint>(_1031 * 4 + 0) * 10u));
                        int2 _1374 = int2(floor(_1372));
                        float2 _1375 = frac(_1372);
                        uint2 _1376 = uint2(_1374);
                        uint _1378 = _1376.y;
                        uint _1380 = _1378 + (_1378 << 10u);
                        uint _1382 = _1380 ^ (_1380 >> 6u);
                        uint _1384 = _1382 + (_1382 << 3u);
                        uint _1386 = _1384 ^ (_1384 >> 11u);
                        uint _1392 = (((_1376.x * 1664525u) + (_1386 + (_1386 << 15u))) + 1013904223u) * 1664525u;
                        uint _1394 = _1392 ^ (_1392 >> 11u);
                        uint _1397 = _1394 ^ ((_1394 << 7u) & 2636928640u);
                        uint _1400 = _1397 ^ ((_1397 << 15u) & 4022730752u);
                        bool _1403 = _1026 ? _1026 : true;
                        uint _1404 = _1026 ? _1022 : 8388607u;
                        bool _1405 = _1024 ? _1024 : true;
                        uint _1406 = _1024 ? _1020 : 1065353216u;
                        uint2 _1412 = uint2(_1374 + int2(1, 0));
                        uint _1414 = _1412.y;
                        uint _1416 = _1414 + (_1414 << 10u);
                        uint _1418 = _1416 ^ (_1416 >> 6u);
                        uint _1420 = _1418 + (_1418 << 3u);
                        uint _1422 = _1420 ^ (_1420 >> 11u);
                        uint _1428 = (((_1412.x * 1664525u) + (_1422 + (_1422 << 15u))) + 1013904223u) * 1664525u;
                        uint _1430 = _1428 ^ (_1428 >> 11u);
                        uint _1433 = _1430 ^ ((_1430 << 7u) & 2636928640u);
                        uint _1436 = _1433 ^ ((_1433 << 15u) & 4022730752u);
                        bool _1439 = _1403 ? _1403 : true;
                        uint _1440 = _1403 ? _1404 : 8388607u;
                        bool _1441 = _1405 ? _1405 : true;
                        uint _1442 = _1405 ? _1406 : 1065353216u;
                        uint2 _1448 = uint2(_1374 + int2(0, 1));
                        uint _1450 = _1448.y;
                        uint _1452 = _1450 + (_1450 << 10u);
                        uint _1454 = _1452 ^ (_1452 >> 6u);
                        uint _1456 = _1454 + (_1454 << 3u);
                        uint _1458 = _1456 ^ (_1456 >> 11u);
                        uint _1464 = (((_1448.x * 1664525u) + (_1458 + (_1458 << 15u))) + 1013904223u) * 1664525u;
                        uint _1466 = _1464 ^ (_1464 >> 11u);
                        uint _1469 = _1466 ^ ((_1466 << 7u) & 2636928640u);
                        uint _1472 = _1469 ^ ((_1469 << 15u) & 4022730752u);
                        bool _1475 = _1439 ? _1439 : true;
                        uint _1476 = _1439 ? _1440 : 8388607u;
                        bool _1477 = _1441 ? _1441 : true;
                        uint _1478 = _1441 ? _1442 : 1065353216u;
                        uint2 _1484 = uint2(_1374 + int2(1, 1));
                        uint _1486 = _1484.y;
                        uint _1488 = _1486 + (_1486 << 10u);
                        uint _1490 = _1488 ^ (_1488 >> 6u);
                        uint _1492 = _1490 + (_1490 << 3u);
                        uint _1494 = _1492 ^ (_1492 >> 11u);
                        uint _1500 = (((_1484.x * 1664525u) + (_1494 + (_1494 << 15u))) + 1013904223u) * 1664525u;
                        uint _1502 = _1500 ^ (_1500 >> 11u);
                        uint _1505 = _1502 ^ ((_1502 << 7u) & 2636928640u);
                        uint _1508 = _1505 ^ ((_1505 << 15u) & 4022730752u);
                        uint _1512 = _1475 ? _1476 : 8388607u;
                        uint _1514 = _1477 ? _1478 : 1065353216u;
                        float2 _1522 = (_1375 * _1375) * (3.0f.xx - (_1375 * 2.0f));
                        float _1523 = _1522.x;
                        float _1524 = 1.0f - _1523;
                        float _1526 = _1522.y;
                        _1013 = _1359;
                        _1015 = _1360;
                        _1017 = _1361;
                        _1021 = _1514;
                        _1023 = _1512;
                        _1025 = _1477 ? _1477 : true;
                        _1027 = _1475 ? _1475 : true;
                        _1544 = (((_1362 * lerp(0.5f, 1.0f, (((1.0f - _1526) * (((asfloat(((_1400 ^ (_1400 >> 18u)) & _1404) | _1406) - 1.0f) * _1524) + ((asfloat(((_1436 ^ (_1436 >> 18u)) & _1440) | _1442) - 1.0f) * _1523))) + (((asfloat(((_1472 ^ (_1472 >> 18u)) & _1476) | _1478) - 1.0f) * _1524) * _1526)) + (((asfloat(((_1508 ^ (_1508 >> 18u)) & _1512) | _1514) - 1.0f) * _1523) * _1526))) * 1.0f) * _431) * ev_mult;
                    }
                    else
                    {
                        _1013 = _1012;
                        _1015 = _1014;
                        _1017 = _1016;
                        _1021 = _1020;
                        _1023 = _1022;
                        _1025 = _1024;
                        _1027 = _1026;
                        _1544 = 0.0f.xxx;
                    }
                    _1010 = _1009 + _1544;
                }
                _1545 = _1009;
            }
            else
            {
                _1545 = _971;
            }
            _2443 = _1545;
        }
        _2444 = _2443;
    }
    else
    {
        _2444 = _971;
    }
    out_var_SV_TARGET = float4(_2444, 0.0f);
    return out_var_SV_TARGET;
}
#pragma dxc diagnostic pop
#else


#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"

void ps_main(VsToPs input)
{
    int2 _148 = int2(input.position.xy);
    uint2 _149 = uint2(_148);
    float2 _171 = (((float2(_148) + 0.5f.xx) / float2(depth_tex_dims)) - 0.5f.xx) * float2(2.0f, -2.0f);
    float _172 = _171.x;
    float4 _179 = mul(view_constants[0u].view_to_world, mul(view_constants[0u].sample_to_view, float4(_172, _171.y, depth_tex.Load(int3(_149, 0u)).x, 1.0f)));
    if ((lantern_count > 0u) ? true : false)
    {
        uint3 _200 = uint3(clamp(floor((((_179.xyz / _179.w.xxx) + (((normalize(mul(view_constants[0u].view_to_world, mul(view_constants[0u].sample_to_view, float4(_172, _171.y, 0.0f, 1.0f))).xyz) * 1.0f) * (float((gbuffer_tex.Load(int3(_149, 0u)).y >> 24u) & 255u) * 0.0039215688593685626983642578125f)) * 1.60000002384185791015625f)) * float3(0.24615384638309478759765625f, 0.123076923191547393798828125f, 0.24615384638309478759765625f)) + float3(16.0f, 2.0f, 16.0f)), 0.0f.xxx, float3(31.0f, 15.0f, 31.0f)));
        if (6u == (WaveActiveAllTrue((light_indirection_grid.Load<uint>(((32u * ((32u * _200.y) + _200.z)) + _200.x) * 4 + 0) >> 20u) <= 2u) ? 2u : 6u))
        {
            uint _145[8];
            for (uint _219 = 0u; _219 < 8u; )
            {
                _145[_219] = 4294967295u;
                _219++;
                continue;
            }
            uint2 _226 = uint2(_148 & int2(1, 1));
            uint _231 = (_226.x + (_226.y * 2u)) & 3u;
            uint _233;
            _233 = 0u;
            uint _234;
            uint _236 = 0u;
            for (;;)
            {
                bool _244;
                if (_236 < 8u)
                {
                    _244 = _233 < 8u;
                }
                else
                {
                    _244 = false;
                }
                if (_244)
                {
                    uint4 _256 = rdi_reservoir_tex.Load(int3(uint2((_148 / int2(2, 2)) + spatial_resolve_offsets_array[((_231 * 16u) + _236) + 256u].xy), 0u));
                    uint2 _257 = _256.xy;
                    uint _258 = _256.x;
                    bool _264;
                    if (any(bool2(_257.x != uint2(0u, 0u).x, _257.y != uint2(0u, 0u).y)))
                    {
                        _264 = _258 < lantern_count;
                    }
                    else
                    {
                        _264 = false;
                    }
                    bool _270;
                    if (_264)
                    {
                        _270 = _145[0u] != _258;
                    }
                    else
                    {
                        _270 = false;
                    }
                    bool _276;
                    if (_270)
                    {
                        _276 = _145[1u] != _258;
                    }
                    else
                    {
                        _276 = false;
                    }
                    bool _282;
                    if (_276)
                    {
                        _282 = _145[2u] != _258;
                    }
                    else
                    {
                        _282 = false;
                    }
                    bool _288;
                    if (_282)
                    {
                        _288 = _145[3u] != _258;
                    }
                    else
                    {
                        _288 = false;
                    }
                    bool _294;
                    if (_288)
                    {
                        _294 = _145[4u] != _258;
                    }
                    else
                    {
                        _294 = false;
                    }
                    bool _300;
                    if (_294)
                    {
                        _300 = _145[5u] != _258;
                    }
                    else
                    {
                        _300 = false;
                    }
                    bool _306;
                    if (_300)
                    {
                        _306 = _145[6u] != _258;
                    }
                    else
                    {
                        _306 = false;
                    }
                    bool _312;
                    if (_306)
                    {
                        _312 = _145[7u] != _258;
                    }
                    else
                    {
                        _312 = false;
                    }
                    if (_312)
                    {
                        _145[_233] = _258;
                        _234 = _233 + 1u;
                    }
                    else
                    {
                        _234 = _233;
                    }
                    _233 = _234;
                    _236++;
                    continue;
                }
                else
                {
                    break;
                }
            }
            uint _319;
            for (uint _318 = _233, _321 = 0u; _321 < 8u; _318 = _319, _321++)
            {
                uint _328 = QuadReadAcrossDiagonal(_145[_321]);
                bool _333;
                if (_328 < lantern_count)
                {
                    _333 = _318 < 8u;
                }
                else
                {
                    _333 = false;
                }
                if (_333)
                {
                    bool _344;
                    if (_145[0u] != _328)
                    {
                        _344 = _145[1u] != _328;
                    }
                    else
                    {
                        _344 = false;
                    }
                    bool _350;
                    if (_344)
                    {
                        _350 = _145[2u] != _328;
                    }
                    else
                    {
                        _350 = false;
                    }
                    bool _356;
                    if (_350)
                    {
                        _356 = _145[3u] != _328;
                    }
                    else
                    {
                        _356 = false;
                    }
                    bool _362;
                    if (_356)
                    {
                        _362 = _145[4u] != _328;
                    }
                    else
                    {
                        _362 = false;
                    }
                    bool _368;
                    if (_362)
                    {
                        _368 = _145[5u] != _328;
                    }
                    else
                    {
                        _368 = false;
                    }
                    bool _374;
                    if (_368)
                    {
                        _374 = _145[6u] != _328;
                    }
                    else
                    {
                        _374 = false;
                    }
                    bool _380;
                    if (_374)
                    {
                        _380 = _145[7u] != _328;
                    }
                    else
                    {
                        _380 = false;
                    }
                    uint _385;
                    if (_380)
                    {
                        _145[_318] = _328;
                        _385 = _318 + 1u;
                    }
                    else
                    {
                        _385 = _318;
                    }
                    _319 = _385;
                }
                else
                {
                    _319 = _318;
                }
            }
        }
        else
        {
        }
    }
}
#pragma dxc diagnostic pop
#endif
