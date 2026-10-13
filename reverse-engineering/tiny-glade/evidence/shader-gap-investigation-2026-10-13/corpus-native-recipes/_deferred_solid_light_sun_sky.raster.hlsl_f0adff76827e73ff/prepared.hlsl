#define SHADER_VARIANT_DEPTH_ONLY 1
#define DEFERRED_FULL_LIGHTING 0
#line 1 "shaders/deferred/solid_light_sun_sky.raster.hlsl"




















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
#line 65 "shaders/deferred/solid_light_sun_sky.raster.hlsl"

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
#line 66 "shaders/deferred/solid_light_sun_sky.raster.hlsl"

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
#line 67 "shaders/deferred/solid_light_sun_sky.raster.hlsl"

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
#line 68 "shaders/deferred/solid_light_sun_sky.raster.hlsl"

#line 1 "shaders/inc/consts.hlsl"






#ifndef TINYGLADE_INC_CONSTS_HLSL
#define TINYGLADE_INC_CONSTS_HLSL


static const float RENDER_SCALE = 1.0f;

#endif 
#line 69 "shaders/deferred/solid_light_sun_sky.raster.hlsl"

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
#line 70 "shaders/deferred/solid_light_sun_sky.raster.hlsl"

#line 1 "shaders/inc/blue_noise.hlsl"

















float4 blue_noise(Texture2D<float4> blue_noise_texture, uint2 pixel_coords, uint frame_index)
{
    uint2 offset = uint2(frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f)
        * float(frame_index)) + 0.5f.xx) * 256.0f);
    uint2 p = (pixel_coords + uint2(13u, 13u) + offset) % uint2(256u, 256u);
    float4 texel = blue_noise_texture.Load(int3(p, 0));
    return texel * 255.0f * (1.0f / 256.0f) + (1.0f / 512.0f);
}
#line 71 "shaders/deferred/solid_light_sun_sky.raster.hlsl"

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
#line 72 "shaders/deferred/solid_light_sun_sky.raster.hlsl"

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
#line 73 "shaders/deferred/solid_light_sun_sky.raster.hlsl"

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
#line 74 "shaders/deferred/solid_light_sun_sky.raster.hlsl"

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
#line 75 "shaders/deferred/solid_light_sun_sky.raster.hlsl"

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
#line 76 "shaders/deferred/solid_light_sun_sky.raster.hlsl"

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
#line 77 "shaders/deferred/solid_light_sun_sky.raster.hlsl"

#line 1 "shaders/inc/sh.hlsl"










float4 sh_basis(float3 dir)
{
    return float4(0.2820949852466583251953125f, 0.48860299587249755859375f * dir);
}

float sh_eval(float4 sh, float3 dir)
{
    return dot(sh, sh_basis(dir));
}
#line 78 "shaders/deferred/solid_light_sun_sky.raster.hlsl"

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
#line 79 "shaders/deferred/solid_light_sun_sky.raster.hlsl"

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
#line 80 "shaders/deferred/solid_light_sun_sky.raster.hlsl"

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
#line 81 "shaders/deferred/solid_light_sun_sky.raster.hlsl"

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
#line 82 "shaders/deferred/solid_light_sun_sky.raster.hlsl"

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
#line 83 "shaders/deferred/solid_light_sun_sky.raster.hlsl"

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
#line 84 "shaders/deferred/solid_light_sun_sky.raster.hlsl"

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
#line 85 "shaders/deferred/solid_light_sun_sky.raster.hlsl"

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
#line 86 "shaders/deferred/solid_light_sun_sky.raster.hlsl"

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
#line 87 "shaders/deferred/solid_light_sun_sky.raster.hlsl"

#line 1 "shaders/inc/brdf_lut.hlsl"


#ifndef TINYGLADE_INC_BRDF_LUT_HLSL
#define TINYGLADE_INC_BRDF_LUT_HLSL



float2 sample_brdf_fg_lut(Texture2D brdf_fg_lut_tex, SamplerState s, float NdotV, float roughness)
{
    float2 uv = float2(saturate(NdotV), roughness * roughness) * 0.984375 + 0.0078125;
    return brdf_fg_lut_tex.SampleLevel(s, uv, 0.0).xy;
}

#endif 
#line 88 "shaders/deferred/solid_light_sun_sky.raster.hlsl"

#line 1 "shaders/terrain_editing/inc_highlight_active_terrain_chunk.inc.hlsl"


#ifndef TINYGLADE_TERRAIN_EDITING_INC_HIGHLIGHT_ACTIVE_TERRAIN_CHUNK_HLSL
#define TINYGLADE_TERRAIN_EDITING_INC_HIGHLIGHT_ACTIVE_TERRAIN_CHUNK_HLSL




float3 highlight_active_terrain_chunk(float3 surface_color, float3 highlight_color, float2 chunk_uv, float strength)
{
    return lerp(surface_color, highlight_color, strength);
}

#endif 
#line 89 "shaders/deferred/solid_light_sun_sky.raster.hlsl"

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
#line 90 "shaders/deferred/solid_light_sun_sky.raster.hlsl"


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
    uint2 brdf_fg_lut_dims;
    uint2 terrain_heightmap_dims;
    uint2 visual_terrain_heightmap_dims;
    uint rt_bvh_node_count;
    uint rt_triangle_count;
};

Texture2D<uint4> gbuffer_tex : register(t14, space0);
Texture2D<float4> depth_tex : register(t15, space0);
Texture2D<float4> blue_noise_texture : register(t18, space0);
Texture2D<uint4> rdi_reservoir_tex : register(t28, space0);
ByteAddressBuffer light_indirection_grid : register(t36, space0);

#if DEFERRED_FULL_LIGHTING
Texture2D<float4> half_depth_tex : register(t16, space0);
Texture2D<float4> ssgi_tex : register(t19, space0);
Texture2D<float4> screen_shadows_tex : register(t20, space0);
Texture2D<float4> highlight_active_hill : register(t22, space0);
ByteAddressBuffer lantern_buf : register(t26, space0);
Texture2D<float4> indirect_radiance_tex : register(t27, space0);
Texture2D<float4> brdf_fg_lut : register(t29, space0);
ByteAddressBuffer grid_light_indices : register(t37, space0);
#endif





#if DEFERRED_FULL_LIGHTING


static float _252;
static uint _253;
static float _303;
static const float4 _265[3] = { 0.0f.xxxx, 0.0f.xxxx, 0.0f.xxxx };
static const Reservoir1sppStreamState _292 = { 0.0f, 0.0f };
static const Reservoir1spp _293 = { 0.0f, 0u, 0.0f, 0.0f };
float2 spvUnpackHalf2x16(uint value)
{
    return f16tof32(uint2(value & 0xffff, value >> 16));
}
float4 ps_main(VsToPs input) : SV_Target0
{
    float4 out_var_SV_TARGET;
    int2 _316 = int2(input.position.xy);
    uint2 _317 = uint2(_316);
    float4 _332 = ((blue_noise_texture.Load(int3((_317 + uint2(frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(frame_index)) + 0.5f.xx) * 256.0f.xx)) % uint2(256u, 256u), 0u)) * 255.0f) * 0.00390625f.xxxx) + 0.001953125f.xxxx;
    float _347 = frac(frac(52.98291778564453125f * frac((0.067110560834407806396484375f * float(_317.x)) + (0.005837149918079376220703125f * float(_317.y)))) + (float(frame_index % 4u) * 0.25f));
    uint4 _349 = gbuffer_tex.Load(int3(_317, 0u));
    float4 _351 = depth_tex.Load(int3(_317, 0u));
    float _352 = _351.x;
    uint _353 = _349.x;
    float _368 = float((_353 >> 24u) & 255u) * 0.0039215688593685626983642578125f;
    uint _370 = _349.y;
    float3 _386 = normalize((float4(float(_353 & 255u) * 0.0039215688593685626983642578125f, float((_353 >> 8u) & 255u) * 0.0039215688593685626983642578125f, float((_353 >> 16u) & 255u) * 0.0039215688593685626983642578125f, _368).xyz * 2.0f) - 1.0f.xxx);
    float3 _387 = float4(float(_370 & 255u) * 0.0039215688593685626983642578125f, float((_370 >> 8u) & 255u) * 0.0039215688593685626983642578125f, float((_370 >> 16u) & 255u) * 0.0039215688593685626983642578125f, _303).xyz;
    float3 _388 = _387 * _387;
    uint _389 = _370 >> 24u;
    bool _392 = ((_389 >> 0u) & 1u) != 0u;
    bool _395 = ((_389 >> 1u) & 1u) != 0u;
    bool _410 = ((_389 >> 6u) & 1u) != 0u;
    float3 _414 = mul(view_constants[2u].view_to_world, float4(0.0f, 0.0f, 1.0f, 0.0f)).xyz;
    float2 _417 = float2(depth_tex_dims);
    float2 _420 = (float2(_316) + 0.5f.xx) / _417;
    float2 _426 = (_420 - 0.5f.xx) * float2(2.0f, -2.0f);
    float _427 = _426.x;
    float4 _432 = float4(_427, _426.y, _352, 1.0f);
    float4 _433 = mul(view_constants[0u].sample_to_view, _432);
    float4 _434 = mul(view_constants[0u].view_to_world, _433);
    float3 _436 = normalize(mul(view_constants[0u].view_to_world, mul(view_constants[0u].sample_to_view, float4(_427, _426.y, 0.0f, 1.0f))).xyz);
    float3 _437 = -_436;
    float3 _441 = _434.xyz / _434.w.xxx;
    float _448 = clamp(0.100000001490116119384765625f + ssgi_tex.SampleLevel(sampler_nnc, _420, 0.0f).x, 0.0f, 1.0f);
    float _660;
    if (_392)
    {
        float3 _454 = _432.xyz;
        float4 _466 = mul(view_constants[0u].view_to_sample, mul(view_constants[0u].world_to_view, float4(_441 + (_414 * 0.300000011920928955078125f), 1.0f)));
        float3 _470 = _466.xyz / _466.w.xxx;
        float4 _481 = _432 + (float4(_470 - _454, 0.0f) * float(int(sign(_470.z))));
        float _482 = _481.w;
        float3 _491 = (_481 / (float((_482 >= 0.0f) ? 1 : (-1)) * max(1.0000000133514319600180897396058e-10f, abs(_482))).xxxx).xyz;
        float3 _492 = _491 - _454;
        bool3 _493 = bool3(_492.x < 0.0f.xxx.x, _492.y < 0.0f.xxx.y, _492.z < 0.0f.xxx.z);
        float3 _496 = (float3(_493.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, _493.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, _493.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - _454) / _492;
        float3 _502 = _454 + (_492 * max(0.0f, max(_496.x, _496.y)));
        float3 _503 = _491 - _502;
        bool3 _504 = bool3(_503.x >= 0.0f.xxx.x, _503.y >= 0.0f.xxx.y, _503.z >= 0.0f.xxx.z);
        float3 _507 = (float3(_504.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, _504.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, _504.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - _502) / _503;
        float3 _515 = _502 + (_503 * min(1.0f, min(min(_507.x, _507.y), _507.z)));
        bool _654;
        float _655;
        do
        {
            float _536 = 0.5f * view_constants[0u].clip_to_view[3].z;
            uint _537 = uint(int(max(2u, min(3u, uint(int(floor(length((((_515.xy * float2(0.5f, -0.5f)) + 0.5f.xx) - ((_502.xy * float2(0.5f, -0.5f)) + 0.5f.xx)) * _417))))))));
            float _639;
            float _640;
            float _641;
            bool _642;
            do
            {
                float3 _542 = _515 - _502;
                float _550;
                if (_515.z > 9.5367431640625e-07f)
                {
                    _550 = float(_537);
                }
                else
                {
                    _550 = max(1.0f, float(_537) - 1.0f);
                }
                float _551 = 1.0f / _550;
                float _632;
                float _633;
                float _634;
                bool _635;
                if (_537 > 0u)
                {
                    float3 _559 = _502 + (_542 * lerp(0.0f, 1.0f, pow(_347 * _551, 1.0f)));
                    float2 _562 = (_559.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                    float _564 = 1.0f / _559.z;
                    float _569 = 1.0f / depth_tex.SampleLevel(sampler_llc, _562, 0.0f).x;
                    float _573 = 1.0f / depth_tex.SampleLevel(sampler_nnc, _562, 0.0f).x;
                    float _577 = (max(_569, _573) * 1.00000202655792236328125f) - _564;
                    float _578 = _564 - min(_569, _573);
                    bool _581 = (_577 < 0.0f) ? (_578 < _536) : false;
                    float _628;
                    float _629;
                    float _630;
                    bool _631;
                    if (_581)
                    {
                        _628 = _577;
                        _629 = _578;
                        _630 = _536;
                        _631 = _581;
                    }
                    else
                    {
                        bool _590;
                        float _624;
                        float _625;
                        float _626;
                        bool _627;
                        uint _586 = 1u;
                        bool _589 = _581;
                        for (;;)
                        {
                            if (_586 < _537)
                            {
                                float3 _601 = _502 + (_542 * lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(_586) + _347) * _551, 1.0f))));
                                float2 _604 = (_601.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                                float _606 = 1.0f / _601.z;
                                float _610 = 1.0f / depth_tex.SampleLevel(sampler_llc, _604, 0.0f).x;
                                float _614 = 1.0f / depth_tex.SampleLevel(sampler_nnc, _604, 0.0f).x;
                                float _618 = (max(_610, _614) * 1.00000202655792236328125f) - _606;
                                float _619 = _606 - min(_610, _614);
                                _590 = (_618 < 0.0f) ? (_619 < _536) : false;
                                if (_590)
                                {
                                    _624 = _618;
                                    _625 = _619;
                                    _626 = _536;
                                    _627 = _590;
                                    break;
                                }
                                _586++;
                                _589 = _590;
                                continue;
                            }
                            else
                            {
                                _624 = 0.0f;
                                _625 = 0.0f;
                                _626 = 0.0f;
                                _627 = _589;
                                break;
                            }
                        }
                        _628 = _624;
                        _629 = _625;
                        _630 = _626;
                        _631 = _627;
                    }
                    _632 = _628;
                    _633 = _629;
                    _634 = _630;
                    _635 = _631;
                }
                else
                {
                    _632 = 0.0f;
                    _633 = 0.0f;
                    _634 = 0.0f;
                    _635 = false;
                }
                if (_635)
                {
                    _639 = _632;
                    _640 = _634;
                    _641 = _633;
                    _642 = true;
                    break;
                }
                else
                {
                    _639 = _252;
                    _640 = _252;
                    _641 = _252;
                    _642 = false;
                    break;
                }
                break; 
            } while(false);
            bool _650;
            if (_642)
            {
                bool _649;
                if (_641 < _640)
                {
                    _649 = _639 < _640;
                }
                else
                {
                    _649 = false;
                }
                _650 = _649;
            }
            else
            {
                _650 = false;
            }
            if (_650)
            {
                _654 = true;
                _655 = _641 / _640;
                break;
            }
            _654 = false;
            _655 = 0.0f;
            break;
        } while(false);
        float _659;
        if (_654)
        {
            _659 = smoothstep(1.0f, 0.5f, _655);
        }
        else
        {
            _659 = 0.0f;
        }
        _660 = _659;
    }
    else
    {
        _660 = 0.0f;
    }
    bool3 _665 = ((render_overrides & 1u) != 0u).xxx;
    float3 _666 = float3(_665.x ? 0.5f.xxx.x : _388.x, _665.y ? 0.5f.xxx.y : _388.y, _665.z ? 0.5f.xxx.z : _388.z);
    float3 _673 = normalize(_386);
    float _679 = clamp(dot(_673, _414), 0.0f, 1.0f);
    float _681 = clamp(dot(_414, normalize(_414 + _437)), 0.0f, 1.0f);
    float _682 = _368 * _368;
    float _687;
    if (_410)
    {
        _687 = 0.20000000298023223876953125f + (_679 * 0.60000002384185791015625f);
    }
    else
    {
        _687 = _679;
    }
    float _691 = (((2.0f * _682) * _681) * _681) + (-0.5f);
    float _692 = 1.0f - _687;
    float _693 = _692 * _692;
    float _698 = 1.0f - max(dot(_673, _437), 9.9999997473787516355514526367188e-05f);
    float _699 = _698 * _698;
    float3 _763;
    do
    {
        float _709 = dot(_386, _414);
        bool _716;
        if (!(_709 <= 0.0f))
        {
            _716 = dot(_386, _437) <= 0.0f;
        }
        else
        {
            _716 = true;
        }
        if (_716)
        {
            _763 = 0.0f.xxx;
            break;
        }
        float _719 = _682 * _682;
        float3 _721 = normalize(_437 + _414);
        float _722 = dot(_386, _721);
        float _725 = max(0.0f, 1.0f - dot(_721, _414));
        float _726 = _725 * _725;
        float _731 = dot(_386, _437);
        float _754 = ((_722 * _722) * (_719 - 1.0f)) + 1.0f;
        _763 = ((lerp(0.039999999105930328369140625f.xxx, 1.0f.xxx, ((_726 * _726) * _725).xxx) * (((2.0f * _709) * _731) / ((_709 * sqrt(((((-_731) * _719) + _731) * _731) + _719)) + (_731 * sqrt(((((-_709) * _719) + _709) * _709) + _719))))) * (_719 / ((3.1415927410125732421875f * _754) * _754))) / ((4.0f * _731) * _709).xxx;
        break;
    } while(false);
    float3 _767 = ((sun_color.xyz * ((_666 * (((1.0f + (_691 * ((_693 * _693) * _692))) * (1.0f + (_691 * ((_699 * _699) * _698)))) * 0.3183098733425140380859375f)) + _763)) * _687) * 1.0f;
    float3 _771;
    if (((_389 >> 2u) & 1u) != 0u)
    {
        _771 = _767 * ((1.0f - _660) * (1.0f - screen_shadows_tex.Load(int3(_317, 0u)).x));
    }
    else
    {
        _771 = _767;
    }
    float3 _776;
    if (((_389 >> 5u) & 1u) != 0u)
    {
        _776 = _771 * lerp(0.60000002384185791015625f, 1.0f, _448);
    }
    else
    {
        _776 = _771;
    }
    float3 _807;
    [branch]
    if (((_389 >> 4u) & 1u) != 0u)
    {
        float3 _805;
        do
        {
            bool _798;
            if (!(abs(_441.x) > 65.0f))
            {
                _798 = abs(_441.z) > 65.0f;
            }
            else
            {
                _798 = true;
            }
            if (_798)
            {
                _805 = 0.0f.xxx;
                break;
            }
            _805 = color_library.terrain_highlight.xyz * highlight_active_hill.SampleLevel(sampler_default, (_441.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x;
            break;
        } while(false);
        _807 = _776 + _805;
    }
    else
    {
        _807 = _776;
    }
    float3 _819;
    if (((_389 >> 3u) & 1u) != 0u)
    {
        _819 = _807 + ((editing_highlight_color * dot(sky_high_color.xyz, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f))) * 2.0f);
    }
    else
    {
        _819 = _807;
    }
    float4 _309[3] = _265;
    float _820 = _332.x;
    uint _824 = uint((_820 * 3.0f) + 0.5f) & 3u;
    float _826;
    _826 = 0.0f;
    float _827;
    for (uint _829 = 0u; _829 < 3u; _826 = _827, _829++)
    {
        uint2 _841 = (_317 / uint2(2u, 2u)) + uint2(spatial_resolve_offsets_array[(_824 * 16u) + _829].xy);
        float _851 = exp2(-min(30.0f, 80.0f * abs(1.0f - (half_depth_tex.Load(int3(_841, 0u)).x / _352))));
        for (uint _853 = 0u; _853 < 3u; )
        {
            _309[_853] += (indirect_radiance_tex.Load(int3(_841 + uint2((_853 * indirect_radiance_tex_dims.x) / 3u, 0u), 0u)) * _851);
            _853++;
            continue;
        }
        _827 = _826 + _851;
    }
    float _918;
    if (_826 < 0.001000000047497451305389404296875f)
    {
        float _874;
        _874 = _826;
        float _875;
        for (uint _877 = 3u; _877 < 6u; _874 = _875, _877++)
        {
            uint2 _889 = (_317 / uint2(2u, 2u)) + uint2(spatial_resolve_offsets_array[(_824 * 16u) + _877].xy);
            float _899 = exp2(-min(30.0f, 80.0f * abs(1.0f - (half_depth_tex.Load(int3(_889, 0u)).x / _352))));
            for (uint _901 = 0u; _901 < 3u; )
            {
                _309[_901] += (indirect_radiance_tex.Load(int3(_889 + uint2((_901 * indirect_radiance_tex_dims.x) / 3u, 0u), 0u)) * _899);
                _901++;
                continue;
            }
            _875 = _874 + _899;
        }
        _918 = _874;
    }
    else
    {
        _918 = _826;
    }
    float _920 = 1.0f / max(9.9999997473787516355514526367188e-05f, _918);
    _309[0] *= _920;
    _309[1] *= _920;
    _309[2] *= _920;
    float3 _937 = float3(_309[0].x, _309[1].x, _309[2].x);
    float3 _941 = _309[0].yzw / max(9.9999996826552253889678874634872e-21f, _309[0].x).xxx;
    float3 _945 = _309[1].yzw / max(9.9999996826552253889678874634872e-21f, _309[1].x).xxx;
    float3 _949 = _309[2].yzw / max(9.9999996826552253889678874634872e-21f, _309[2].x).xxx;
    float3 _955 = max(0.0f.xxx, 1.0f.xxx - (color_library.indirect_light_tint.xyz * 0.920000016689300537109375f));
    float3 _963 = max(0.0f.xxx, lerp((_955 / dot(_955, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f)).xxx) * dot(_937, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f)), _937, 1.16999995708465576171875f.xxx) * 1.2000000476837158203125f);
    float _974 = 0.004999999888241291046142578125f / (((-0.5f) / (_433.xyz / _433.w.xxx).z) * view_constants[0u].view_to_clip[1].y);
    float2 _979[4] = { _332.xw, _332.yz, _332.zy, _332.wx };
    float2 _310[4] = _979;
    float _988;
    _988 = 0.0f;
    float _989;
    float _982;
    float _985;
    float _987;
    float _981 = 0.0f;
    float _984 = 0.0f;
    float _986 = 0.0f;
    for (uint _990 = 0u; _990 < 2u; _981 = _982, _984 = _985, _986 = _987, _988 = _989, _990++)
    {
        float _999 = 1.0f - (2.0f * _310[_990].x);
        float _1003 = sqrt(max(0.0f, 1.0f - (_999 * _999)));
        float _1005 = 6.283185482025146484375f * _310[_990].y;
        float3 _1013 = _432.xyz;
        float4 _1025 = mul(view_constants[0u].view_to_sample, mul(view_constants[0u].world_to_view, float4(_441 + (normalize(float3(cos(_1005) * _1003, sin(_1005) * _1003, _999) + _386) * _974), 1.0f)));
        float3 _1029 = _1025.xyz / _1025.w.xxx;
        float4 _1040 = _432 + (float4(_1029 - _1013, 0.0f) * float(int(sign(_1029.z))));
        float _1041 = _1040.w;
        float3 _1050 = (_1040 / (float((_1041 >= 0.0f) ? 1 : (-1)) * max(1.0000000133514319600180897396058e-10f, abs(_1041))).xxxx).xyz;
        float3 _1051 = _1050 - _1013;
        bool3 _1052 = bool3(_1051.x < 0.0f.xxx.x, _1051.y < 0.0f.xxx.y, _1051.z < 0.0f.xxx.z);
        float3 _1055 = (float3(_1052.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, _1052.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, _1052.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - _1013) / _1051;
        float3 _1061 = _1013 + (_1051 * max(0.0f, max(_1055.x, _1055.y)));
        float3 _1062 = _1050 - _1061;
        bool3 _1063 = bool3(_1062.x >= 0.0f.xxx.x, _1062.y >= 0.0f.xxx.y, _1062.z >= 0.0f.xxx.z);
        float3 _1066 = (float3(_1063.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, _1063.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, _1063.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - _1061) / _1062;
        float3 _1074 = _1061 + (_1062 * min(1.0f, min(min(_1066.x, _1066.y), _1066.z)));
        bool _1207;
        float _1208;
        do
        {
            float _1095 = 0.300000011920928955078125f * view_constants[0u].clip_to_view[3].z;
            uint _1096 = uint(int(max(2u, min(2u, uint(int(floor(length((((_1074.xy * float2(0.5f, -0.5f)) + 0.5f.xx) - ((_1061.xy * float2(0.5f, -0.5f)) + 0.5f.xx)) * _417))))))));
            bool _1195;
            do
            {
                float3 _1101 = _1074 - _1061;
                float _1109;
                if (_1074.z > 9.5367431640625e-07f)
                {
                    _1109 = float(_1096);
                }
                else
                {
                    _1109 = max(1.0f, float(_1096) - 1.0f);
                }
                float _1110 = 1.0f / _1109;
                float _1188;
                float _1189;
                float _1190;
                bool _1191;
                if (_1096 > 0u)
                {
                    float3 _1117 = _1061 + (_1101 * lerp(0.0f, 1.0f, pow(_1110, 1.0f)));
                    float2 _1120 = (_1117.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                    float _1122 = 1.0f / _1117.z;
                    float _1127 = 1.0f / depth_tex.SampleLevel(sampler_llc, _1120, 0.0f).x;
                    float _1131 = 1.0f / depth_tex.SampleLevel(sampler_nnc, _1120, 0.0f).x;
                    float _1135 = (max(_1127, _1131) * 1.00000202655792236328125f) - _1122;
                    bool _1138 = (_1135 < 0.0f) ? true : false;
                    float _1184;
                    float _1185;
                    float _1186;
                    bool _1187;
                    if (_1138)
                    {
                        _1184 = _1135;
                        _1185 = _1122 - min(_1127, _1131);
                        _1186 = _1095;
                        _1187 = _1138;
                    }
                    else
                    {
                        bool _1147;
                        float _1180;
                        float _1181;
                        float _1182;
                        bool _1183;
                        uint _1143 = 1u;
                        bool _1146 = _1138;
                        for (;;)
                        {
                            if (_1143 < _1096)
                            {
                                float3 _1158 = _1061 + (_1101 * lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(_1143) + 1.0f) * _1110, 1.0f))));
                                float2 _1161 = (_1158.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                                float _1163 = 1.0f / _1158.z;
                                float _1167 = 1.0f / depth_tex.SampleLevel(sampler_llc, _1161, 0.0f).x;
                                float _1171 = 1.0f / depth_tex.SampleLevel(sampler_nnc, _1161, 0.0f).x;
                                float _1175 = (max(_1167, _1171) * 1.00000202655792236328125f) - _1163;
                                _1147 = (_1175 < 0.0f) ? true : false;
                                if (_1147)
                                {
                                    _1180 = _1175;
                                    _1181 = _1163 - min(_1167, _1171);
                                    _1182 = _1095;
                                    _1183 = _1147;
                                    break;
                                }
                                _1143++;
                                _1146 = _1147;
                                continue;
                            }
                            else
                            {
                                _1180 = 0.0f;
                                _1181 = 0.0f;
                                _1182 = 0.0f;
                                _1183 = _1146;
                                break;
                            }
                        }
                        _1184 = _1180;
                        _1185 = _1181;
                        _1186 = _1182;
                        _1187 = _1183;
                    }
                    _1188 = _1184;
                    _1189 = _1185;
                    _1190 = _1186;
                    _1191 = _1187;
                }
                else
                {
                    _1188 = 0.0f;
                    _1189 = 0.0f;
                    _1190 = 0.0f;
                    _1191 = false;
                }
                if (_1191)
                {
                    _982 = _1188;
                    _985 = _1190;
                    _987 = _1189;
                    _1195 = true;
                    break;
                }
                else
                {
                    _982 = _981;
                    _985 = _984;
                    _987 = _986;
                    _1195 = false;
                    break;
                }
                break; 
            } while(false);
            bool _1203;
            if (_1195)
            {
                bool _1202;
                if (_987 < _985)
                {
                    _1202 = _982 < _985;
                }
                else
                {
                    _1202 = false;
                }
                _1203 = _1202;
            }
            else
            {
                _1203 = false;
            }
            if (_1203)
            {
                _1207 = true;
                _1208 = _987 / _985;
                break;
            }
            _1207 = false;
            _1208 = 0.0f;
            break;
        } while(false);
        _989 = _988 + (_1207 ? _1208 : 1.0f);
    }
    float _1211 = lerp(_988 * 0.5f, 1.0f, 0.4000000059604644775390625f);
    bool _1212 = !_410;
    float3 _1297;
    if (_1212)
    {
        float3 _1222 = (-float3(_941.y, _945.y, _949.y)) * _963;
        float3 _1227 = float3(_941.z, _945.z, _949.z) * _963;
        float3 _1233 = (-float3(_941.x, _945.x, _949.x)) * _963;
        float3 _1240 = normalize(float3(-dot(_1233, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f)), -dot(_1222, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f)), dot(_1227, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f))));
        float3 _1266 = float3(abs(dot(float3(-_1233.x, -_1222.x, _1227.x), _1240)), abs(dot(float3(-_1233.y, -_1222.y, _1227.y), _1240)), abs(dot(float3(-_1233.z, -_1222.z, _1227.z), _1240))) / _963;
        float _1272 = dot(_1240, _386);
        _1297 = max(0.0f.xxx, ((((0.886227548122406005859375f.xxx * _963) + ((1.0f.xxx * (_386.y * (-1.02332770824432373046875f))) * _1222)) + ((1.0f.xxx * (_386.z * 1.02332770824432373046875f)) * _1227)) + ((1.0f.xxx * (_386.x * (-1.02332770824432373046875f))) * _1233)) + (((_963 * ((_1266 * 0.07999999821186065673828125f) + ((_1266 * 0.60000002384185791015625f) * _1266))) * 0.25f) * (0.3153915703296661376953125f * (((3.0f * _1272) * _1272) - 1.0f))));
    }
    else
    {
        _1297 = _963 * 1.7999999523162841796875f;
    }
    float _1303;
    if (_395)
    {
        _1303 = lerp(_448, 1.0f, 0.300000011920928955078125f);
    }
    else
    {
        _1303 = 1.0f;
    }
    float4 _1312 = brdf_fg_lut.SampleLevel(sampler_lnc, (float2(dot(_386, _437), _682) * 0.984375f.xx) + 0.0078125f.xx, 0.0f);
    float _1313 = _1312.x;
    float _1315 = _1312.y;
    float3 _1317 = (0.039999999105930328369140625f.xxx * _1313) + _1315.xxx;
    float _1318 = _1313 + _1315;
    float3 _1323 = lerp(_1317 / _1318.xxx, 1.0f.xxx, 0.4000000059604644775390625f.xxx) * (1.0f - _1318);
    float3 _1327 = _1317 * (1.0f.xxx + (_1323 / (1.0f.xxx - _1323)));
    float3 _1424;
    if (_1212)
    {
        float _1334 = lerp(1.0f, _368, 0.5f);
        float _1335 = _1334 * _1334;
        float _1337 = 1.0f - _1335;
        float3 _1343 = normalize(lerp(_386, reflect(_436, _386), (_1337 * (sqrt(_1337) + _1335)).xxx));
        float3 _1349 = (-float3(_941.y, _945.y, _949.y)) * _963;
        float3 _1354 = float3(_941.z, _945.z, _949.z) * _963;
        float3 _1360 = (-float3(_941.x, _945.x, _949.x)) * _963;
        float3 _1367 = normalize(float3(-dot(_1360, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f)), -dot(_1349, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f)), dot(_1354, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f))));
        float3 _1393 = float3(abs(dot(float3(-_1360.x, -_1349.x, _1354.x), _1367)), abs(dot(float3(-_1360.y, -_1349.y, _1354.y), _1367)), abs(dot(float3(-_1360.z, -_1349.z, _1354.z), _1367))) / _963;
        float _1399 = dot(_1367, _1343);
        _1424 = max(0.0f.xxx, ((((0.886227548122406005859375f.xxx * _963) + ((1.0f.xxx * (_1343.y * (-1.02332770824432373046875f))) * _1349)) + ((1.0f.xxx * (_1343.z * 1.02332770824432373046875f)) * _1354)) + ((1.0f.xxx * (_1343.x * (-1.02332770824432373046875f))) * _1360)) + (((_963 * ((_1393 * 0.07999999821186065673828125f) + ((_1393 * 0.60000002384185791015625f) * _1393))) * 0.25f) * (0.3153915703296661376953125f * (((3.0f * _1399) * _1399) - 1.0f))));
    }
    else
    {
        _1424 = _1297;
    }
    float _1433;
    if (_392)
    {
        float _1430 = _1211 * _1211;
        _1433 = (_1430 * _1430) * _1211;
    }
    else
    {
        _1433 = 1.0f;
    }
    float _1438;
    if (_395)
    {
        _1438 = lerp(_448, 1.0f, 0.300000011920928955078125f);
    }
    else
    {
        _1438 = 1.0f;
    }
    float3 _1442 = _819 + (((((1.0f.xxx - _1327) * min(_392 ? _1211 : 1.0f, _1303)) * _666) * _1297) + (((_1424 * 1.0f) * _1327) * min(_1433, _1438)));
    float3 _2913;
    if ((lantern_count > 0u) ? true : false)
    {
        uint3 _1453 = uint3(clamp(floor((_441 * float3(0.24615384638309478759765625f, 0.123076923191547393798828125f, 0.24615384638309478759765625f)) + float3(16.0f, 2.0f, 16.0f)), 0.0f.xxx, float3(31.0f, 15.0f, 31.0f)));
        uint _1460 = (32u * ((32u * _1453.y) + _1453.z)) + _1453.x;
        bool _1465 = WaveActiveAllTrue((light_indirection_grid.Load<uint>(_1460 * 4 + 0) >> 20u) <= 2u);
        uint _1466 = _1465 ? 2u : 6u;
        float3 _2912;
        if (6u == _1466)
        {
            uint _311[8];
            for (uint _2017 = 0u; _2017 < 8u; )
            {
                _311[_2017] = 4294967295u;
                _2017++;
                continue;
            }
            uint2 _2024 = uint2(_316 & int2(1, 1));
            uint _2029 = (_2024.x + (_2024.y * 2u)) & 3u;
            uint _2031;
            _2031 = 0u;
            uint _2032;
            uint _2034 = 0u;
            for (;;)
            {
                bool _2042;
                if (_2034 < 8u)
                {
                    _2042 = _2031 < 8u;
                }
                else
                {
                    _2042 = false;
                }
                if (_2042)
                {
                    uint4 _2054 = rdi_reservoir_tex.Load(int3(uint2((_316 / int2(2, 2)) + spatial_resolve_offsets_array[((_2029 * 16u) + _2034) + 256u].xy), 0u));
                    uint2 _2055 = _2054.xy;
                    uint _2056 = _2054.x;
                    bool _2062;
                    if (any(bool2(_2055.x != uint2(0u, 0u).x, _2055.y != uint2(0u, 0u).y)))
                    {
                        _2062 = _2056 < lantern_count;
                    }
                    else
                    {
                        _2062 = false;
                    }
                    bool _2068;
                    if (_2062)
                    {
                        _2068 = _311[0u] != _2056;
                    }
                    else
                    {
                        _2068 = false;
                    }
                    bool _2074;
                    if (_2068)
                    {
                        _2074 = _311[1u] != _2056;
                    }
                    else
                    {
                        _2074 = false;
                    }
                    bool _2080;
                    if (_2074)
                    {
                        _2080 = _311[2u] != _2056;
                    }
                    else
                    {
                        _2080 = false;
                    }
                    bool _2086;
                    if (_2080)
                    {
                        _2086 = _311[3u] != _2056;
                    }
                    else
                    {
                        _2086 = false;
                    }
                    bool _2092;
                    if (_2086)
                    {
                        _2092 = _311[4u] != _2056;
                    }
                    else
                    {
                        _2092 = false;
                    }
                    bool _2098;
                    if (_2092)
                    {
                        _2098 = _311[5u] != _2056;
                    }
                    else
                    {
                        _2098 = false;
                    }
                    bool _2104;
                    if (_2098)
                    {
                        _2104 = _311[6u] != _2056;
                    }
                    else
                    {
                        _2104 = false;
                    }
                    bool _2110;
                    if (_2104)
                    {
                        _2110 = _311[7u] != _2056;
                    }
                    else
                    {
                        _2110 = false;
                    }
                    if (_2110)
                    {
                        _311[_2031] = _2056;
                        _2032 = _2031 + 1u;
                    }
                    else
                    {
                        _2032 = _2031;
                    }
                    _2031 = _2032;
                    _2034++;
                    continue;
                }
                else
                {
                    break;
                }
            }
            uint _2116;
            _2116 = _2031;
            uint _2117;
            for (uint _2119 = 0u; _2119 < 8u; _2116 = _2117, _2119++)
            {
                uint _2126 = QuadReadAcrossDiagonal(_311[_2119]);
                bool _2131;
                if (_2126 < lantern_count)
                {
                    _2131 = _2116 < 8u;
                }
                else
                {
                    _2131 = false;
                }
                if (_2131)
                {
                    bool _2142;
                    if (_311[0u] != _2126)
                    {
                        _2142 = _311[1u] != _2126;
                    }
                    else
                    {
                        _2142 = false;
                    }
                    bool _2148;
                    if (_2142)
                    {
                        _2148 = _311[2u] != _2126;
                    }
                    else
                    {
                        _2148 = false;
                    }
                    bool _2154;
                    if (_2148)
                    {
                        _2154 = _311[3u] != _2126;
                    }
                    else
                    {
                        _2154 = false;
                    }
                    bool _2160;
                    if (_2154)
                    {
                        _2160 = _311[4u] != _2126;
                    }
                    else
                    {
                        _2160 = false;
                    }
                    bool _2166;
                    if (_2160)
                    {
                        _2166 = _311[5u] != _2126;
                    }
                    else
                    {
                        _2166 = false;
                    }
                    bool _2172;
                    if (_2166)
                    {
                        _2172 = _311[6u] != _2126;
                    }
                    else
                    {
                        _2172 = false;
                    }
                    bool _2178;
                    if (_2172)
                    {
                        _2178 = _311[7u] != _2126;
                    }
                    else
                    {
                        _2178 = false;
                    }
                    uint _2183;
                    if (_2178)
                    {
                        _311[_2116] = _2126;
                        _2183 = _2116 + 1u;
                    }
                    else
                    {
                        _2183 = _2116;
                    }
                    _2117 = _2183;
                }
                else
                {
                    _2117 = _2116;
                }
            }
            Reservoir1sppStreamState _312[2];
            Reservoir1spp _313[2];
            for (uint _2185 = 0u; _2185 < 2u; )
            {
                _312[_2185] = _292;
                _313[_2185] = _293;
                _2185++;
                continue;
            }
            for (uint _2193 = 0u; _2193 < _2116; _2193++)
            {
                float2 _2206 = spvUnpackHalf2x16(lantern_buf.Load<uint>(_311[_2193] * 16 + 12) & 65535u);
                float _2207 = _2206.x;
                uint _2209 = lantern_buf.Load<uint>(_311[_2193] * 16 + 12) >> 16u;
                float3 _2210 = lantern_buf.Load<float3>(_311[_2193] * 16 + 0) - _441;
                float _2212 = dot(_2210, _2210);
                float3 _2219 = _2210 + float3(0.0f, 0.1500000059604644775390625f, 0.0f);
                float3 _2264;
                do
                {
                    if (_2209 == 0u)
                    {
                        _2264 = float3(0.7919800281524658203125f, 0.63358402252197265625f, 0.07919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.7919800281524658203125f).xxx;
                        break;
                    }
                    else
                    {
                        if (_2209 == 1u)
                        {
                            _2264 = float3(0.0f, 0.1583960056304931640625f, 0.7919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.215240001678466796875f).xxx;
                            break;
                        }
                        else
                        {
                            if (_2209 == 2u)
                            {
                                _2264 = float3(0.1583960056304931640625f, 0.7919800281524658203125f, 0.1583960056304931640625f) / max(1.0000000133514319600180897396058e-10f, 0.772159993648529052734375f).xxx;
                                break;
                            }
                            else
                            {
                                if (_2209 == 3u)
                                {
                                    _2264 = float3(0.23759402334690093994140625f, 0.1583960056304931640625f, 0.7919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.279020011425018310546875f).xxx;
                                    break;
                                }
                                else
                                {
                                    _2264 = float3(0.7919800281524658203125f, 0.23759402334690093994140625f, 0.07919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.4343800246715545654296875f).xxx;
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
                float _2268 = dot(((_2264 * 4.0f) * _2207) * ((max(0.0f, dot(_386, _2210 / length(_2210).xxx)) * ((1.0f / (0.3359999954700469970703125f + _2212)) * smoothstep(36.0f * _2207, 16.0f * _2207, _2212))) * smoothstep(-0.5f, 0.20000000298023223876953125f, (_2219 / length(_2219).xxx).y)), float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f));
                float _2269 = float(_2116);
                float _2273 = frac(_820 + (float(_2193) / _2269));
                if (_313[0].w_sum <= _313[1].w_sum)
                {
                    do
                    {
                        _312[0].M_sum += 1.0f;
                        float _2312 = _2268 * _2269;
                        bool _2327;
                        do
                        {
                            _313[0].w_sum += _2312;
                            _313[0].M += 1.0f;
                            if ((_2312 / _313[0].w_sum) >= _2273)
                            {
                                _313[0].payload = _311[_2193];
                                _2327 = true;
                                break;
                            }
                            else
                            {
                                _2327 = false;
                                break;
                            }
                            break; 
                        } while(false);
                        if (_2327)
                        {
                            _312[0].p_q_sel = _2268;
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
                        _312[1].M_sum += 1.0f;
                        float _2287 = _2268 * _2269;
                        bool _2302;
                        do
                        {
                            _313[1].w_sum += _2287;
                            _313[1].M += 1.0f;
                            if ((_2287 / _313[1].w_sum) >= _2273)
                            {
                                _313[1].payload = _311[_2193];
                                _2302 = true;
                                break;
                            }
                            else
                            {
                                _2302 = false;
                                break;
                            }
                            break; 
                        } while(false);
                        if (_2302)
                        {
                            _312[1].p_q_sel = _2268;
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
            for (uint _2333 = 1u; _2333 < 2u; )
            {
                _312[0].M_sum += _312[_2333].M_sum;
                _2333++;
                continue;
            }
            for (uint _2344 = 1u; _2344 < 2u; )
            {
                _312[_2344].M_sum = _312[0].M_sum;
                _2344++;
                continue;
            }
            float3 _2360;
            _2360 = _1442;
            float _2354;
            float _2357;
            float _2359;
            float3 _2361;
            uint _2365;
            uint _2367;
            bool _2369;
            bool _2371;
            float _2353 = 0.0f;
            float _2356 = 0.0f;
            float _2358 = 0.0f;
            uint _2362 = 0u;
            uint _2364 = 0u;
            uint _2366 = 0u;
            bool _2368 = false;
            bool _2370 = false;
            for (; _2362 < 2u; _2353 = _2354, _2356 = _2357, _2358 = _2359, _2360 = _2361, _2362++, _2364 = _2365, _2366 = _2367, _2368 = _2369, _2370 = _2371)
            {
                _313[_2362].M = _312[_2362].M_sum;
                _313[_2362].W = _313[_2362].w_sum / max(9.9999999392252902907785028219223e-09f, _313[_2362].M * _312[_2362].p_q_sel);
                if (_313[_2362].W > 0.0f)
                {
                    float2 _2398 = spvUnpackHalf2x16(lantern_buf.Load<uint>(_313[_2362].payload * 16 + 12) & 65535u);
                    float _2399 = _2398.x;
                    uint _2401 = lantern_buf.Load<uint>(_313[_2362].payload * 16 + 12) >> 16u;
                    float3 _2402 = lantern_buf.Load<float3>(_313[_2362].payload * 16 + 0) - _441;
                    float _2403 = length(_2402);
                    float _2404 = dot(_2402, _2402);
                    float3 _2411 = _2402 + float3(0.0f, 0.1500000059604644775390625f, 0.0f);
                    float3 _2417 = _2403.xxx;
                    float3 _2456;
                    do
                    {
                        if (_2401 == 0u)
                        {
                            _2456 = float3(0.7919800281524658203125f, 0.63358402252197265625f, 0.07919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.7919800281524658203125f).xxx;
                            break;
                        }
                        else
                        {
                            if (_2401 == 1u)
                            {
                                _2456 = float3(0.0f, 0.1583960056304931640625f, 0.7919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.215240001678466796875f).xxx;
                                break;
                            }
                            else
                            {
                                if (_2401 == 2u)
                                {
                                    _2456 = float3(0.1583960056304931640625f, 0.7919800281524658203125f, 0.1583960056304931640625f) / max(1.0000000133514319600180897396058e-10f, 0.772159993648529052734375f).xxx;
                                    break;
                                }
                                else
                                {
                                    if (_2401 == 3u)
                                    {
                                        _2456 = float3(0.23759402334690093994140625f, 0.1583960056304931640625f, 0.7919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.279020011425018310546875f).xxx;
                                        break;
                                    }
                                    else
                                    {
                                        _2456 = float3(0.7919800281524658203125f, 0.23759402334690093994140625f, 0.07919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.4343800246715545654296875f).xxx;
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
                    float3 _2459 = ((_2456 * 4.0f) * _2399) * ((max(0.0f, dot(_386, _2402 / _2417)) * ((1.0f / (0.3359999954700469970703125f + _2404)) * smoothstep(36.0f * _2399, 16.0f * _2399, _2404))) * smoothstep(-0.5f, 0.20000000298023223876953125f, (_2411 / length(_2411).xxx).y));
                    float _2903;
                    float _2904;
                    float _2905;
                    uint _2906;
                    uint _2907;
                    bool _2908;
                    bool _2909;
                    float3 _2910;
                    if ((_313[_2362].W * max(max(_2459.x, _2459.y), _2459.z)) > 0.0f)
                    {
                        float _2718;
                        float _2719;
                        float _2720;
                        float3 _2721;
                        if (_2403 > 0.5f)
                        {
                            float3 _2479 = (_441 + ((_2402 * (_2403 - 0.5f)) / _2417)) + (_386 * (0.0199999995529651641845703125f * _332.y));
                            float3 _2480 = _432.xyz;
                            float4 _2490 = mul(view_constants[0u].view_to_sample, mul(view_constants[0u].world_to_view, float4(_2479, 1.0f)));
                            float3 _2494 = _2490.xyz / _2490.w.xxx;
                            float4 _2505 = _432 + (float4(_2494 - _2480, 0.0f) * float(int(sign(_2494.z))));
                            float _2506 = _2505.w;
                            float3 _2515 = (_2505 / (float((_2506 >= 0.0f) ? 1 : (-1)) * max(1.0000000133514319600180897396058e-10f, abs(_2506))).xxxx).xyz;
                            float3 _2516 = _2515 - _2480;
                            bool3 _2517 = bool3(_2516.x < 0.0f.xxx.x, _2516.y < 0.0f.xxx.y, _2516.z < 0.0f.xxx.z);
                            float3 _2520 = (float3(_2517.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, _2517.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, _2517.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - _2480) / _2516;
                            float3 _2526 = _2480 + (_2516 * max(0.0f, max(_2520.x, _2520.y)));
                            float3 _2527 = _2515 - _2526;
                            bool3 _2528 = bool3(_2527.x >= 0.0f.xxx.x, _2527.y >= 0.0f.xxx.y, _2527.z >= 0.0f.xxx.z);
                            float3 _2531 = (float3(_2528.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, _2528.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, _2528.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - _2526) / _2527;
                            float3 _2539 = _2526 + (_2527 * min(1.0f, min(min(_2531.x, _2531.y), _2531.z)));
                            float _2670;
                            float _2671;
                            float _2672;
                            bool _2685;
                            float _2686;
                            float _2687;
                            do
                            {
                                float _2560 = 0.800000011920928955078125f * view_constants[0u].clip_to_view[3].z;
                                uint _2561 = uint(int(max(2u, min(4u, uint(int(floor(length((((_2539.xy * float2(0.5f, -0.5f)) + 0.5f.xx) - ((_2526.xy * float2(0.5f, -0.5f)) + 0.5f.xx)) * _417))))))));
                                float _2665;
                                float _2669;
                                bool _2673;
                                do
                                {
                                    float3 _2566 = _2539 - _2526;
                                    float _2574;
                                    if (_2539.z > 9.5367431640625e-07f)
                                    {
                                        _2574 = float(_2561);
                                    }
                                    else
                                    {
                                        _2574 = max(1.0f, float(_2561) - 1.0f);
                                    }
                                    float _2575 = 1.0f / _2574;
                                    float _2660;
                                    float _2661;
                                    float _2662;
                                    float _2663;
                                    bool _2664;
                                    if (_2561 > 0u)
                                    {
                                        float _2581 = lerp(0.0f, 1.0f, pow(_820 * _2575, 1.0f));
                                        float3 _2583 = _2526 + (_2566 * _2581);
                                        float2 _2586 = (_2583.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                                        float _2588 = 1.0f / _2583.z;
                                        float _2593 = 1.0f / depth_tex.SampleLevel(sampler_llc, _2586, 0.0f).x;
                                        float _2597 = 1.0f / depth_tex.SampleLevel(sampler_nnc, _2586, 0.0f).x;
                                        float _2601 = (max(_2593, _2597) * 1.00000202655792236328125f) - _2588;
                                        float _2602 = _2588 - min(_2593, _2597);
                                        bool _2605 = (_2601 < 0.0f) ? (_2602 < _2560) : false;
                                        float _2654;
                                        float _2655;
                                        float _2656;
                                        float _2657;
                                        bool _2658;
                                        float _2659;
                                        if (_2605)
                                        {
                                            _2654 = _2601;
                                            _2655 = _2602;
                                            _2656 = _2560;
                                            _2657 = _2581;
                                            _2658 = _2605;
                                            _2659 = 0.0f;
                                        }
                                        else
                                        {
                                            float _2615;
                                            _2615 = _2581;
                                            bool _2614;
                                            float _2616;
                                            float _2649;
                                            float _2650;
                                            float _2651;
                                            float _2652;
                                            bool _2653;
                                            uint _2610 = 1u;
                                            bool _2613 = _2605;
                                            for (;;)
                                            {
                                                if (_2610 < _2561)
                                                {
                                                    _2616 = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(_2610) + _820) * _2575, 1.0f)));
                                                    float3 _2626 = _2526 + (_2566 * _2616);
                                                    float2 _2629 = (_2626.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                                                    float _2631 = 1.0f / _2626.z;
                                                    float _2635 = 1.0f / depth_tex.SampleLevel(sampler_llc, _2629, 0.0f).x;
                                                    float _2639 = 1.0f / depth_tex.SampleLevel(sampler_nnc, _2629, 0.0f).x;
                                                    float _2643 = (max(_2635, _2639) * 1.00000202655792236328125f) - _2631;
                                                    float _2644 = _2631 - min(_2635, _2639);
                                                    _2614 = (_2643 < 0.0f) ? (_2644 < _2560) : false;
                                                    if (_2614)
                                                    {
                                                        _2649 = _2643;
                                                        _2650 = _2644;
                                                        _2651 = _2560;
                                                        _2652 = _2616;
                                                        _2653 = _2614;
                                                        break;
                                                    }
                                                    _2610++;
                                                    _2613 = _2614;
                                                    _2615 = _2616;
                                                    continue;
                                                }
                                                else
                                                {
                                                    _2649 = 0.0f;
                                                    _2650 = 0.0f;
                                                    _2651 = 0.0f;
                                                    _2652 = 1.0f;
                                                    _2653 = _2613;
                                                    break;
                                                }
                                            }
                                            _2654 = _2649;
                                            _2655 = _2650;
                                            _2656 = _2651;
                                            _2657 = _2652;
                                            _2658 = _2653;
                                            _2659 = _2615;
                                        }
                                        _2660 = _2654;
                                        _2661 = _2655;
                                        _2662 = _2656;
                                        _2663 = _2657;
                                        _2664 = _2658;
                                        _2665 = _2659;
                                    }
                                    else
                                    {
                                        _2660 = 0.0f;
                                        _2661 = 0.0f;
                                        _2662 = 0.0f;
                                        _2663 = 1.0f;
                                        _2664 = false;
                                        _2665 = 0.0f;
                                    }
                                    if (_2664)
                                    {
                                        _2669 = _2663;
                                        _2670 = _2660;
                                        _2671 = _2662;
                                        _2672 = _2661;
                                        _2673 = true;
                                        break;
                                    }
                                    else
                                    {
                                        _2669 = _2665;
                                        _2670 = _2353;
                                        _2671 = _2356;
                                        _2672 = _2358;
                                        _2673 = false;
                                        break;
                                    }
                                    break; 
                                } while(false);
                                bool _2681;
                                if (_2673)
                                {
                                    bool _2680;
                                    if (_2672 < _2671)
                                    {
                                        _2680 = _2670 < _2671;
                                    }
                                    else
                                    {
                                        _2680 = false;
                                    }
                                    _2681 = _2680;
                                }
                                else
                                {
                                    _2681 = false;
                                }
                                if (_2681)
                                {
                                    _2685 = true;
                                    _2686 = _2669;
                                    _2687 = _2672 / view_constants[0u].clip_to_view[3].z;
                                    break;
                                }
                                _2685 = false;
                                _2686 = _2665;
                                _2687 = 0.0f;
                                break;
                            } while(false);
                            float3 _2717;
                            if (_2685)
                            {
                                float4 _2697 = mul(view_constants[0u].view_to_world, mul(view_constants[0u].sample_to_view, float4(lerp(_2526, _2539, _2686.xxx), 1.0f)));
                                float3 _2704 = ((_2697.xyz / _2697.w.xxx) + (_437 * _2687)) - lantern_buf.Load<float3>(_313[_2362].payload * 16 + 0);
                                bool _2714;
                                if (!(abs(_2704.y) > 0.5f))
                                {
                                    float2 _2711 = _2704.xz;
                                    _2714 = dot(_2711, _2711) > 0.136900007724761962890625f;
                                }
                                else
                                {
                                    _2714 = true;
                                }
                                bool3 _2715 = _2714.xxx;
                                _2717 = float3(_2715.x ? 0.0f.xxx.x : _2459.x, _2715.y ? 0.0f.xxx.y : _2459.y, _2715.z ? 0.0f.xxx.z : _2459.z);
                            }
                            else
                            {
                                _2717 = _2459;
                            }
                            _2718 = _2670;
                            _2719 = _2671;
                            _2720 = _2672;
                            _2721 = _2717;
                        }
                        else
                        {
                            _2718 = _2353;
                            _2719 = _2356;
                            _2720 = _2358;
                            _2721 = _2459;
                        }
                        float2 _2731 = float2((float(game_time_sec) + game_time_frac) * 4.0f, float(_313[_2362].payload * 10u));
                        int2 _2733 = int2(floor(_2731));
                        float2 _2734 = frac(_2731);
                        uint2 _2735 = uint2(_2733);
                        uint _2737 = _2735.y;
                        uint _2739 = _2737 + (_2737 << 10u);
                        uint _2741 = _2739 ^ (_2739 >> 6u);
                        uint _2743 = _2741 + (_2741 << 3u);
                        uint _2745 = _2743 ^ (_2743 >> 11u);
                        uint _2751 = (((_2735.x * 1664525u) + (_2745 + (_2745 << 15u))) + 1013904223u) * 1664525u;
                        uint _2753 = _2751 ^ (_2751 >> 11u);
                        uint _2756 = _2753 ^ ((_2753 << 7u) & 2636928640u);
                        uint _2759 = _2756 ^ ((_2756 << 15u) & 4022730752u);
                        bool _2762 = _2370 ? _2370 : true;
                        uint _2763 = _2370 ? _2366 : 8388607u;
                        bool _2764 = _2368 ? _2368 : true;
                        uint _2765 = _2368 ? _2364 : 1065353216u;
                        uint2 _2771 = uint2(_2733 + int2(1, 0));
                        uint _2773 = _2771.y;
                        uint _2775 = _2773 + (_2773 << 10u);
                        uint _2777 = _2775 ^ (_2775 >> 6u);
                        uint _2779 = _2777 + (_2777 << 3u);
                        uint _2781 = _2779 ^ (_2779 >> 11u);
                        uint _2787 = (((_2771.x * 1664525u) + (_2781 + (_2781 << 15u))) + 1013904223u) * 1664525u;
                        uint _2789 = _2787 ^ (_2787 >> 11u);
                        uint _2792 = _2789 ^ ((_2789 << 7u) & 2636928640u);
                        uint _2795 = _2792 ^ ((_2792 << 15u) & 4022730752u);
                        bool _2798 = _2762 ? _2762 : true;
                        uint _2799 = _2762 ? _2763 : 8388607u;
                        bool _2800 = _2764 ? _2764 : true;
                        uint _2801 = _2764 ? _2765 : 1065353216u;
                        uint2 _2807 = uint2(_2733 + int2(0, 1));
                        uint _2809 = _2807.y;
                        uint _2811 = _2809 + (_2809 << 10u);
                        uint _2813 = _2811 ^ (_2811 >> 6u);
                        uint _2815 = _2813 + (_2813 << 3u);
                        uint _2817 = _2815 ^ (_2815 >> 11u);
                        uint _2823 = (((_2807.x * 1664525u) + (_2817 + (_2817 << 15u))) + 1013904223u) * 1664525u;
                        uint _2825 = _2823 ^ (_2823 >> 11u);
                        uint _2828 = _2825 ^ ((_2825 << 7u) & 2636928640u);
                        uint _2831 = _2828 ^ ((_2828 << 15u) & 4022730752u);
                        bool _2834 = _2798 ? _2798 : true;
                        uint _2835 = _2798 ? _2799 : 8388607u;
                        bool _2836 = _2800 ? _2800 : true;
                        uint _2837 = _2800 ? _2801 : 1065353216u;
                        uint2 _2843 = uint2(_2733 + int2(1, 1));
                        uint _2845 = _2843.y;
                        uint _2847 = _2845 + (_2845 << 10u);
                        uint _2849 = _2847 ^ (_2847 >> 6u);
                        uint _2851 = _2849 + (_2849 << 3u);
                        uint _2853 = _2851 ^ (_2851 >> 11u);
                        uint _2859 = (((_2843.x * 1664525u) + (_2853 + (_2853 << 15u))) + 1013904223u) * 1664525u;
                        uint _2861 = _2859 ^ (_2859 >> 11u);
                        uint _2864 = _2861 ^ ((_2861 << 7u) & 2636928640u);
                        uint _2867 = _2864 ^ ((_2864 << 15u) & 4022730752u);
                        uint _2871 = _2834 ? _2835 : 8388607u;
                        uint _2873 = _2836 ? _2837 : 1065353216u;
                        float2 _2881 = (_2734 * _2734) * (3.0f.xx - (_2734 * 2.0f));
                        float _2882 = _2881.x;
                        float _2883 = 1.0f - _2882;
                        float _2885 = _2881.y;
                        _2903 = _2718;
                        _2904 = _2719;
                        _2905 = _2720;
                        _2906 = _2873;
                        _2907 = _2871;
                        _2908 = _2836 ? _2836 : true;
                        _2909 = _2834 ? _2834 : true;
                        _2910 = (((_2721 * lerp(0.5f, 1.0f, (((1.0f - _2885) * (((asfloat(((_2759 ^ (_2759 >> 18u)) & _2763) | _2765) - 1.0f) * _2883) + ((asfloat(((_2795 ^ (_2795 >> 18u)) & _2799) | _2801) - 1.0f) * _2882))) + (((asfloat(((_2831 ^ (_2831 >> 18u)) & _2835) | _2837) - 1.0f) * _2883) * _2885)) + (((asfloat(((_2867 ^ (_2867 >> 18u)) & _2871) | _2873) - 1.0f) * _2882) * _2885))) * _313[_2362].W) * _666) * ev_mult;
                    }
                    else
                    {
                        _2903 = _2353;
                        _2904 = _2356;
                        _2905 = _2358;
                        _2906 = _2364;
                        _2907 = _2366;
                        _2908 = _2368;
                        _2909 = _2370;
                        _2910 = 0.0f.xxx;
                    }
                    _2354 = _2903;
                    _2357 = _2904;
                    _2359 = _2905;
                    _2361 = _2360 + _2910;
                    _2365 = _2906;
                    _2367 = _2907;
                    _2369 = _2908;
                    _2371 = _2909;
                }
                else
                {
                    _2354 = _2353;
                    _2357 = _2356;
                    _2359 = _2358;
                    _2361 = _2360;
                    _2365 = _2364;
                    _2367 = _2366;
                    _2369 = _2368;
                    _2371 = _2370;
                }
            }
            _2912 = _2360;
        }
        else
        {
            float3 _2015;
            if (2u == _1466)
            {
                uint _1476 = light_indirection_grid.Load<uint>(_1460 * 4 + 0) & 1048575u;
                uint _1478 = min(64u, (light_indirection_grid.Load<uint>(_1460 * 4 + 0) >> 20u));
                float3 _1480;
                _1480 = _1442;
                float3 _1481;
                float _1484;
                float _1486;
                float _1488;
                uint _1492;
                uint _1494;
                bool _1496;
                bool _1498;
                float _1483 = 0.0f;
                float _1485 = 0.0f;
                float _1487 = 0.0f;
                uint _1489 = 0u;
                uint _1491 = 0u;
                uint _1493 = 0u;
                bool _1495 = false;
                bool _1497 = false;
                for (; _1489 < _1478; _1480 = _1481, _1483 = _1484, _1485 = _1486, _1487 = _1488, _1489++, _1491 = _1492, _1493 = _1494, _1495 = _1496, _1497 = _1498)
                {
                    uint _1502 = _1476 + _1489;
                    float2 _1510 = spvUnpackHalf2x16(lantern_buf.Load<uint>(grid_light_indices.Load<uint>(_1502 * 4 + 0) * 16 + 12) & 65535u);
                    float _1511 = _1510.x;
                    uint _1513 = lantern_buf.Load<uint>(grid_light_indices.Load<uint>(_1502 * 4 + 0) * 16 + 12) >> 16u;
                    float3 _1514 = lantern_buf.Load<float3>(grid_light_indices.Load<uint>(_1502 * 4 + 0) * 16 + 0) - _441;
                    float _1515 = length(_1514);
                    float _1516 = dot(_1514, _1514);
                    float3 _1523 = _1514 + float3(0.0f, 0.1500000059604644775390625f, 0.0f);
                    float3 _1529 = _1515.xxx;
                    float3 _1568;
                    do
                    {
                        if (_1513 == 0u)
                        {
                            _1568 = float3(0.7919800281524658203125f, 0.63358402252197265625f, 0.07919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.7919800281524658203125f).xxx;
                            break;
                        }
                        else
                        {
                            if (_1513 == 1u)
                            {
                                _1568 = float3(0.0f, 0.1583960056304931640625f, 0.7919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.215240001678466796875f).xxx;
                                break;
                            }
                            else
                            {
                                if (_1513 == 2u)
                                {
                                    _1568 = float3(0.1583960056304931640625f, 0.7919800281524658203125f, 0.1583960056304931640625f) / max(1.0000000133514319600180897396058e-10f, 0.772159993648529052734375f).xxx;
                                    break;
                                }
                                else
                                {
                                    if (_1513 == 3u)
                                    {
                                        _1568 = float3(0.23759402334690093994140625f, 0.1583960056304931640625f, 0.7919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.279020011425018310546875f).xxx;
                                        break;
                                    }
                                    else
                                    {
                                        _1568 = float3(0.7919800281524658203125f, 0.23759402334690093994140625f, 0.07919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.4343800246715545654296875f).xxx;
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
                    float3 _1571 = ((_1568 * 4.0f) * _1511) * ((max(0.0f, dot(_386, _1514 / _1529)) * ((1.0f / (0.3359999954700469970703125f + _1516)) * smoothstep(36.0f * _1511, 16.0f * _1511, _1516))) * smoothstep(-0.5f, 0.20000000298023223876953125f, (_1523 / length(_1523).xxx).y));
                    float3 _2014;
                    if (max(max(_1571.x, _1571.y), _1571.z) > 0.0f)
                    {
                        float _1829;
                        float _1830;
                        float _1831;
                        float3 _1832;
                        if (_1515 > 0.5f)
                        {
                            float3 _1590 = (_441 + ((_1514 * (_1515 - 0.5f)) / _1529)) + (_386 * (0.0199999995529651641845703125f * _332.y));
                            float3 _1591 = _432.xyz;
                            float4 _1601 = mul(view_constants[0u].view_to_sample, mul(view_constants[0u].world_to_view, float4(_1590, 1.0f)));
                            float3 _1605 = _1601.xyz / _1601.w.xxx;
                            float4 _1616 = _432 + (float4(_1605 - _1591, 0.0f) * float(int(sign(_1605.z))));
                            float _1617 = _1616.w;
                            float3 _1626 = (_1616 / (float((_1617 >= 0.0f) ? 1 : (-1)) * max(1.0000000133514319600180897396058e-10f, abs(_1617))).xxxx).xyz;
                            float3 _1627 = _1626 - _1591;
                            bool3 _1628 = bool3(_1627.x < 0.0f.xxx.x, _1627.y < 0.0f.xxx.y, _1627.z < 0.0f.xxx.z);
                            float3 _1631 = (float3(_1628.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, _1628.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, _1628.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - _1591) / _1627;
                            float3 _1637 = _1591 + (_1627 * max(0.0f, max(_1631.x, _1631.y)));
                            float3 _1638 = _1626 - _1637;
                            bool3 _1639 = bool3(_1638.x >= 0.0f.xxx.x, _1638.y >= 0.0f.xxx.y, _1638.z >= 0.0f.xxx.z);
                            float3 _1642 = (float3(_1639.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, _1639.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, _1639.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - _1637) / _1638;
                            float3 _1650 = _1637 + (_1638 * min(1.0f, min(min(_1642.x, _1642.y), _1642.z)));
                            float _1781;
                            float _1782;
                            float _1783;
                            bool _1796;
                            float _1797;
                            float _1798;
                            do
                            {
                                float _1671 = 0.800000011920928955078125f * view_constants[0u].clip_to_view[3].z;
                                uint _1672 = uint(int(max(2u, min(4u, uint(int(floor(length((((_1650.xy * float2(0.5f, -0.5f)) + 0.5f.xx) - ((_1637.xy * float2(0.5f, -0.5f)) + 0.5f.xx)) * _417))))))));
                                float _1776;
                                float _1780;
                                bool _1784;
                                do
                                {
                                    float3 _1677 = _1650 - _1637;
                                    float _1685;
                                    if (_1650.z > 9.5367431640625e-07f)
                                    {
                                        _1685 = float(_1672);
                                    }
                                    else
                                    {
                                        _1685 = max(1.0f, float(_1672) - 1.0f);
                                    }
                                    float _1686 = 1.0f / _1685;
                                    float _1771;
                                    float _1772;
                                    float _1773;
                                    float _1774;
                                    bool _1775;
                                    if (_1672 > 0u)
                                    {
                                        float _1692 = lerp(0.0f, 1.0f, pow(_820 * _1686, 1.0f));
                                        float3 _1694 = _1637 + (_1677 * _1692);
                                        float2 _1697 = (_1694.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                                        float _1699 = 1.0f / _1694.z;
                                        float _1704 = 1.0f / depth_tex.SampleLevel(sampler_llc, _1697, 0.0f).x;
                                        float _1708 = 1.0f / depth_tex.SampleLevel(sampler_nnc, _1697, 0.0f).x;
                                        float _1712 = (max(_1704, _1708) * 1.00000202655792236328125f) - _1699;
                                        float _1713 = _1699 - min(_1704, _1708);
                                        bool _1716 = (_1712 < 0.0f) ? (_1713 < _1671) : false;
                                        float _1765;
                                        float _1766;
                                        float _1767;
                                        float _1768;
                                        bool _1769;
                                        float _1770;
                                        if (_1716)
                                        {
                                            _1765 = _1712;
                                            _1766 = _1713;
                                            _1767 = _1671;
                                            _1768 = _1692;
                                            _1769 = _1716;
                                            _1770 = 0.0f;
                                        }
                                        else
                                        {
                                            float _1726;
                                            _1726 = _1692;
                                            bool _1725;
                                            float _1727;
                                            float _1760;
                                            float _1761;
                                            float _1762;
                                            float _1763;
                                            bool _1764;
                                            uint _1721 = 1u;
                                            bool _1724 = _1716;
                                            for (;;)
                                            {
                                                if (_1721 < _1672)
                                                {
                                                    _1727 = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(_1721) + _820) * _1686, 1.0f)));
                                                    float3 _1737 = _1637 + (_1677 * _1727);
                                                    float2 _1740 = (_1737.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                                                    float _1742 = 1.0f / _1737.z;
                                                    float _1746 = 1.0f / depth_tex.SampleLevel(sampler_llc, _1740, 0.0f).x;
                                                    float _1750 = 1.0f / depth_tex.SampleLevel(sampler_nnc, _1740, 0.0f).x;
                                                    float _1754 = (max(_1746, _1750) * 1.00000202655792236328125f) - _1742;
                                                    float _1755 = _1742 - min(_1746, _1750);
                                                    _1725 = (_1754 < 0.0f) ? (_1755 < _1671) : false;
                                                    if (_1725)
                                                    {
                                                        _1760 = _1754;
                                                        _1761 = _1755;
                                                        _1762 = _1671;
                                                        _1763 = _1727;
                                                        _1764 = _1725;
                                                        break;
                                                    }
                                                    _1721++;
                                                    _1724 = _1725;
                                                    _1726 = _1727;
                                                    continue;
                                                }
                                                else
                                                {
                                                    _1760 = 0.0f;
                                                    _1761 = 0.0f;
                                                    _1762 = 0.0f;
                                                    _1763 = 1.0f;
                                                    _1764 = _1724;
                                                    break;
                                                }
                                            }
                                            _1765 = _1760;
                                            _1766 = _1761;
                                            _1767 = _1762;
                                            _1768 = _1763;
                                            _1769 = _1764;
                                            _1770 = _1726;
                                        }
                                        _1771 = _1765;
                                        _1772 = _1766;
                                        _1773 = _1767;
                                        _1774 = _1768;
                                        _1775 = _1769;
                                        _1776 = _1770;
                                    }
                                    else
                                    {
                                        _1771 = 0.0f;
                                        _1772 = 0.0f;
                                        _1773 = 0.0f;
                                        _1774 = 1.0f;
                                        _1775 = false;
                                        _1776 = 0.0f;
                                    }
                                    if (_1775)
                                    {
                                        _1780 = _1774;
                                        _1781 = _1771;
                                        _1782 = _1773;
                                        _1783 = _1772;
                                        _1784 = true;
                                        break;
                                    }
                                    else
                                    {
                                        _1780 = _1776;
                                        _1781 = _1483;
                                        _1782 = _1485;
                                        _1783 = _1487;
                                        _1784 = false;
                                        break;
                                    }
                                    break; 
                                } while(false);
                                bool _1792;
                                if (_1784)
                                {
                                    bool _1791;
                                    if (_1783 < _1782)
                                    {
                                        _1791 = _1781 < _1782;
                                    }
                                    else
                                    {
                                        _1791 = false;
                                    }
                                    _1792 = _1791;
                                }
                                else
                                {
                                    _1792 = false;
                                }
                                if (_1792)
                                {
                                    _1796 = true;
                                    _1797 = _1780;
                                    _1798 = _1783 / view_constants[0u].clip_to_view[3].z;
                                    break;
                                }
                                _1796 = false;
                                _1797 = _1776;
                                _1798 = 0.0f;
                                break;
                            } while(false);
                            float3 _1828;
                            if (_1796)
                            {
                                float4 _1808 = mul(view_constants[0u].view_to_world, mul(view_constants[0u].sample_to_view, float4(lerp(_1637, _1650, _1797.xxx), 1.0f)));
                                float3 _1815 = ((_1808.xyz / _1808.w.xxx) + (_437 * _1798)) - lantern_buf.Load<float3>(grid_light_indices.Load<uint>(_1502 * 4 + 0) * 16 + 0);
                                bool _1825;
                                if (!(abs(_1815.y) > 0.5f))
                                {
                                    float2 _1822 = _1815.xz;
                                    _1825 = dot(_1822, _1822) > 0.136900007724761962890625f;
                                }
                                else
                                {
                                    _1825 = true;
                                }
                                bool3 _1826 = _1825.xxx;
                                _1828 = float3(_1826.x ? 0.0f.xxx.x : _1571.x, _1826.y ? 0.0f.xxx.y : _1571.y, _1826.z ? 0.0f.xxx.z : _1571.z);
                            }
                            else
                            {
                                _1828 = _1571;
                            }
                            _1829 = _1781;
                            _1830 = _1782;
                            _1831 = _1783;
                            _1832 = _1828;
                        }
                        else
                        {
                            _1829 = _1483;
                            _1830 = _1485;
                            _1831 = _1487;
                            _1832 = _1571;
                        }
                        float2 _1842 = float2((float(game_time_sec) + game_time_frac) * 4.0f, float(grid_light_indices.Load<uint>(_1502 * 4 + 0) * 10u));
                        int2 _1844 = int2(floor(_1842));
                        float2 _1845 = frac(_1842);
                        uint2 _1846 = uint2(_1844);
                        uint _1848 = _1846.y;
                        uint _1850 = _1848 + (_1848 << 10u);
                        uint _1852 = _1850 ^ (_1850 >> 6u);
                        uint _1854 = _1852 + (_1852 << 3u);
                        uint _1856 = _1854 ^ (_1854 >> 11u);
                        uint _1862 = (((_1846.x * 1664525u) + (_1856 + (_1856 << 15u))) + 1013904223u) * 1664525u;
                        uint _1864 = _1862 ^ (_1862 >> 11u);
                        uint _1867 = _1864 ^ ((_1864 << 7u) & 2636928640u);
                        uint _1870 = _1867 ^ ((_1867 << 15u) & 4022730752u);
                        bool _1873 = _1497 ? _1497 : true;
                        uint _1874 = _1497 ? _1493 : 8388607u;
                        bool _1875 = _1495 ? _1495 : true;
                        uint _1876 = _1495 ? _1491 : 1065353216u;
                        uint2 _1882 = uint2(_1844 + int2(1, 0));
                        uint _1884 = _1882.y;
                        uint _1886 = _1884 + (_1884 << 10u);
                        uint _1888 = _1886 ^ (_1886 >> 6u);
                        uint _1890 = _1888 + (_1888 << 3u);
                        uint _1892 = _1890 ^ (_1890 >> 11u);
                        uint _1898 = (((_1882.x * 1664525u) + (_1892 + (_1892 << 15u))) + 1013904223u) * 1664525u;
                        uint _1900 = _1898 ^ (_1898 >> 11u);
                        uint _1903 = _1900 ^ ((_1900 << 7u) & 2636928640u);
                        uint _1906 = _1903 ^ ((_1903 << 15u) & 4022730752u);
                        bool _1909 = _1873 ? _1873 : true;
                        uint _1910 = _1873 ? _1874 : 8388607u;
                        bool _1911 = _1875 ? _1875 : true;
                        uint _1912 = _1875 ? _1876 : 1065353216u;
                        uint2 _1918 = uint2(_1844 + int2(0, 1));
                        uint _1920 = _1918.y;
                        uint _1922 = _1920 + (_1920 << 10u);
                        uint _1924 = _1922 ^ (_1922 >> 6u);
                        uint _1926 = _1924 + (_1924 << 3u);
                        uint _1928 = _1926 ^ (_1926 >> 11u);
                        uint _1934 = (((_1918.x * 1664525u) + (_1928 + (_1928 << 15u))) + 1013904223u) * 1664525u;
                        uint _1936 = _1934 ^ (_1934 >> 11u);
                        uint _1939 = _1936 ^ ((_1936 << 7u) & 2636928640u);
                        uint _1942 = _1939 ^ ((_1939 << 15u) & 4022730752u);
                        bool _1945 = _1909 ? _1909 : true;
                        uint _1946 = _1909 ? _1910 : 8388607u;
                        bool _1947 = _1911 ? _1911 : true;
                        uint _1948 = _1911 ? _1912 : 1065353216u;
                        uint2 _1954 = uint2(_1844 + int2(1, 1));
                        uint _1956 = _1954.y;
                        uint _1958 = _1956 + (_1956 << 10u);
                        uint _1960 = _1958 ^ (_1958 >> 6u);
                        uint _1962 = _1960 + (_1960 << 3u);
                        uint _1964 = _1962 ^ (_1962 >> 11u);
                        uint _1970 = (((_1954.x * 1664525u) + (_1964 + (_1964 << 15u))) + 1013904223u) * 1664525u;
                        uint _1972 = _1970 ^ (_1970 >> 11u);
                        uint _1975 = _1972 ^ ((_1972 << 7u) & 2636928640u);
                        uint _1978 = _1975 ^ ((_1975 << 15u) & 4022730752u);
                        uint _1982 = _1945 ? _1946 : 8388607u;
                        uint _1984 = _1947 ? _1948 : 1065353216u;
                        float2 _1992 = (_1845 * _1845) * (3.0f.xx - (_1845 * 2.0f));
                        float _1993 = _1992.x;
                        float _1994 = 1.0f - _1993;
                        float _1996 = _1992.y;
                        _1484 = _1829;
                        _1486 = _1830;
                        _1488 = _1831;
                        _1492 = _1984;
                        _1494 = _1982;
                        _1496 = _1947 ? _1947 : true;
                        _1498 = _1945 ? _1945 : true;
                        _2014 = (((_1832 * lerp(0.5f, 1.0f, (((1.0f - _1996) * (((asfloat(((_1870 ^ (_1870 >> 18u)) & _1874) | _1876) - 1.0f) * _1994) + ((asfloat(((_1906 ^ (_1906 >> 18u)) & _1910) | _1912) - 1.0f) * _1993))) + (((asfloat(((_1942 ^ (_1942 >> 18u)) & _1946) | _1948) - 1.0f) * _1994) * _1996)) + (((asfloat(((_1978 ^ (_1978 >> 18u)) & _1982) | _1984) - 1.0f) * _1993) * _1996))) * 1.0f) * _666) * ev_mult;
                    }
                    else
                    {
                        _1484 = _1483;
                        _1486 = _1485;
                        _1488 = _1487;
                        _1492 = _1491;
                        _1494 = _1493;
                        _1496 = _1495;
                        _1498 = _1497;
                        _2014 = 0.0f.xxx;
                    }
                    _1481 = _1480 + _2014;
                }
                _2015 = _1480;
            }
            else
            {
                _2015 = _1442;
            }
            _2912 = _2015;
        }
        _2913 = _2912;
    }
    else
    {
        _2913 = _1442;
    }
    out_var_SV_TARGET = float4(_2913, 0.0f);
    return out_var_SV_TARGET;
}
#else



void ps_main(VsToPs input)
{
    int2 _140 = int2(input.position.xy);
    float4 _161 = mul(view_constants[0u].view_to_world, mul(view_constants[0u].sample_to_view, float4((((float2(_140) + 0.5f.xx) / float2(depth_tex_dims)) - 0.5f.xx) * float2(2.0f, -2.0f), depth_tex.Load(int3(uint2(_140), 0u)).x, 1.0f)));
    if ((lantern_count > 0u) ? true : false)
    {
        uint3 _176 = uint3(clamp(floor(((_161.xyz / _161.w.xxx) * float3(0.24615384638309478759765625f, 0.123076923191547393798828125f, 0.24615384638309478759765625f)) + float3(16.0f, 2.0f, 16.0f)), 0.0f.xxx, float3(31.0f, 15.0f, 31.0f)));
        if (6u == (WaveActiveAllTrue((light_indirection_grid.Load<uint>(((32u * ((32u * _176.y) + _176.z)) + _176.x) * 4 + 0) >> 20u) <= 2u) ? 2u : 6u))
        {
            uint _137[8];
            for (uint _195 = 0u; _195 < 8u; )
            {
                _137[_195] = 4294967295u;
                _195++;
                continue;
            }
            uint2 _202 = uint2(_140 & int2(1, 1));
            uint _207 = (_202.x + (_202.y * 2u)) & 3u;
            uint _209;
            _209 = 0u;
            uint _210;
            uint _212 = 0u;
            for (;;)
            {
                bool _220;
                if (_212 < 8u)
                {
                    _220 = _209 < 8u;
                }
                else
                {
                    _220 = false;
                }
                if (_220)
                {
                    uint4 _232 = rdi_reservoir_tex.Load(int3(uint2((_140 / int2(2, 2)) + spatial_resolve_offsets_array[((_207 * 16u) + _212) + 256u].xy), 0u));
                    uint2 _233 = _232.xy;
                    uint _234 = _232.x;
                    bool _240;
                    if (any(bool2(_233.x != uint2(0u, 0u).x, _233.y != uint2(0u, 0u).y)))
                    {
                        _240 = _234 < lantern_count;
                    }
                    else
                    {
                        _240 = false;
                    }
                    bool _246;
                    if (_240)
                    {
                        _246 = _137[0u] != _234;
                    }
                    else
                    {
                        _246 = false;
                    }
                    bool _252;
                    if (_246)
                    {
                        _252 = _137[1u] != _234;
                    }
                    else
                    {
                        _252 = false;
                    }
                    bool _258;
                    if (_252)
                    {
                        _258 = _137[2u] != _234;
                    }
                    else
                    {
                        _258 = false;
                    }
                    bool _264;
                    if (_258)
                    {
                        _264 = _137[3u] != _234;
                    }
                    else
                    {
                        _264 = false;
                    }
                    bool _270;
                    if (_264)
                    {
                        _270 = _137[4u] != _234;
                    }
                    else
                    {
                        _270 = false;
                    }
                    bool _276;
                    if (_270)
                    {
                        _276 = _137[5u] != _234;
                    }
                    else
                    {
                        _276 = false;
                    }
                    bool _282;
                    if (_276)
                    {
                        _282 = _137[6u] != _234;
                    }
                    else
                    {
                        _282 = false;
                    }
                    bool _288;
                    if (_282)
                    {
                        _288 = _137[7u] != _234;
                    }
                    else
                    {
                        _288 = false;
                    }
                    if (_288)
                    {
                        _137[_209] = _234;
                        _210 = _209 + 1u;
                    }
                    else
                    {
                        _210 = _209;
                    }
                    _209 = _210;
                    _212++;
                    continue;
                }
                else
                {
                    break;
                }
            }
            uint _295;
            for (uint _294 = _209, _297 = 0u; _297 < 8u; _294 = _295, _297++)
            {
                uint _304 = QuadReadAcrossDiagonal(_137[_297]);
                bool _309;
                if (_304 < lantern_count)
                {
                    _309 = _294 < 8u;
                }
                else
                {
                    _309 = false;
                }
                if (_309)
                {
                    bool _320;
                    if (_137[0u] != _304)
                    {
                        _320 = _137[1u] != _304;
                    }
                    else
                    {
                        _320 = false;
                    }
                    bool _326;
                    if (_320)
                    {
                        _326 = _137[2u] != _304;
                    }
                    else
                    {
                        _326 = false;
                    }
                    bool _332;
                    if (_326)
                    {
                        _332 = _137[3u] != _304;
                    }
                    else
                    {
                        _332 = false;
                    }
                    bool _338;
                    if (_332)
                    {
                        _338 = _137[4u] != _304;
                    }
                    else
                    {
                        _338 = false;
                    }
                    bool _344;
                    if (_338)
                    {
                        _344 = _137[5u] != _304;
                    }
                    else
                    {
                        _344 = false;
                    }
                    bool _350;
                    if (_344)
                    {
                        _350 = _137[6u] != _304;
                    }
                    else
                    {
                        _350 = false;
                    }
                    bool _356;
                    if (_350)
                    {
                        _356 = _137[7u] != _304;
                    }
                    else
                    {
                        _356 = false;
                    }
                    uint _361;
                    if (_356)
                    {
                        _137[_294] = _304;
                        _361 = _294 + 1u;
                    }
                    else
                    {
                        _361 = _294;
                    }
                    _295 = _361;
                }
                else
                {
                    _295 = _294;
                }
            }
        }
        else
        {
        }
    }
}
#endif
