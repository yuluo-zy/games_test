#define SHADER_VARIANT_DEPTH_ONLY 1
#define DEFERRED_FULL_LIGHTING 0
#line 1 "shaders/deferred/plant_light_sun_sky.raster.hlsl"






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
#line 51 "shaders/deferred/plant_light_sun_sky.raster.hlsl"

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
#line 52 "shaders/deferred/plant_light_sun_sky.raster.hlsl"

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
#line 53 "shaders/deferred/plant_light_sun_sky.raster.hlsl"

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
#line 54 "shaders/deferred/plant_light_sun_sky.raster.hlsl"

#line 1 "shaders/inc/consts.hlsl"






#ifndef TINYGLADE_INC_CONSTS_HLSL
#define TINYGLADE_INC_CONSTS_HLSL


static const float RENDER_SCALE = 1.0f;

#endif 
#line 55 "shaders/deferred/plant_light_sun_sky.raster.hlsl"

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
#line 56 "shaders/deferred/plant_light_sun_sky.raster.hlsl"

#line 1 "shaders/inc/blue_noise.hlsl"

















float4 blue_noise(Texture2D<float4> blue_noise_texture, uint2 pixel_coords, uint frame_index)
{
    uint2 offset = uint2(frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f)
        * float(frame_index)) + 0.5f.xx) * 256.0f);
    uint2 p = (pixel_coords + uint2(13u, 13u) + offset) % uint2(256u, 256u);
    float4 texel = blue_noise_texture.Load(int3(p, 0));
    return texel * 255.0f * (1.0f / 256.0f) + (1.0f / 512.0f);
}
#line 57 "shaders/deferred/plant_light_sun_sky.raster.hlsl"

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
#line 58 "shaders/deferred/plant_light_sun_sky.raster.hlsl"

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
#line 59 "shaders/deferred/plant_light_sun_sky.raster.hlsl"

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
#line 60 "shaders/deferred/plant_light_sun_sky.raster.hlsl"

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
#line 61 "shaders/deferred/plant_light_sun_sky.raster.hlsl"

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
#line 62 "shaders/deferred/plant_light_sun_sky.raster.hlsl"

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
#line 63 "shaders/deferred/plant_light_sun_sky.raster.hlsl"

#line 1 "shaders/inc/sh.hlsl"










float4 sh_basis(float3 dir)
{
    return float4(0.2820949852466583251953125f, 0.48860299587249755859375f * dir);
}

float sh_eval(float4 sh, float3 dir)
{
    return dot(sh, sh_basis(dir));
}
#line 64 "shaders/deferred/plant_light_sun_sky.raster.hlsl"

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
#line 65 "shaders/deferred/plant_light_sun_sky.raster.hlsl"

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
#line 66 "shaders/deferred/plant_light_sun_sky.raster.hlsl"

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
#line 67 "shaders/deferred/plant_light_sun_sky.raster.hlsl"

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
#line 68 "shaders/deferred/plant_light_sun_sky.raster.hlsl"

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
#line 69 "shaders/deferred/plant_light_sun_sky.raster.hlsl"

#line 1 "shaders/inc/material/eval/plant_material.inc.hlsl"


#ifndef TINYGLADE_INC_MATERIAL_EVAL_PLANT_MATERIAL_HLSL
#define TINYGLADE_INC_MATERIAL_EVAL_PLANT_MATERIAL_HLSL

#line 1 "shaders/inc/material/plant_material.inc.hlsl"


#ifndef TINYGLADE_INC_MATERIAL_PLANT_MATERIAL_HLSL
#define TINYGLADE_INC_MATERIAL_PLANT_MATERIAL_HLSL

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
#line 6 "shaders/inc/material/plant_material.inc.hlsl"



struct PlantMaterialParams {
    float3 base_color;
    float3 flower_color;
    float roughness;
    float translucency;
    uint is_flower;
};

PlantMaterialParams init_plant_material(float3 base_color, float3 flower_color, float roughness, uint is_flower)
{
    PlantMaterialParams p;
    p.base_color = base_color;
    p.flower_color = flower_color;
    p.roughness = saturate(roughness);
    p.translucency = 1.0;
    p.is_flower = is_flower;
    return p;
}

#endif 
#line 6 "shaders/inc/material/eval/plant_material.inc.hlsl"

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
#line 7 "shaders/inc/material/eval/plant_material.inc.hlsl"




MaterialEvalData eval_plant_material(PlantMaterialParams p, float3 world_normal,
                                     Texture2D depth_tex, SamplerState s, float2 texel_size,
                                     float3 ray_start_view, float3 ray_end_view)
{
    MaterialEvalData d;
    init_material_eval_data(d);
    float3 albedo = p.base_color;
    if (p.is_flower != 0u) {
        albedo = lerp(albedo, p.flower_color, 0.5);
    }
    d.albedo = albedo;
    d.normal = normalize(world_normal);
    d.roughness = p.roughness;
    d.opacity = 1.0 - raymarch_opacity(depth_tex, s, texel_size, ray_start_view, ray_end_view, p.translucency);
    d.emissive = float3(0.0, 0.0, 0.0);
    return d;
}

#endif 
#line 70 "shaders/deferred/plant_light_sun_sky.raster.hlsl"

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
#line 71 "shaders/deferred/plant_light_sun_sky.raster.hlsl"

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
#line 72 "shaders/deferred/plant_light_sun_sky.raster.hlsl"

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
#line 73 "shaders/deferred/plant_light_sun_sky.raster.hlsl"

#line 1 "shaders/inc/brdf_lut.hlsl"


#ifndef TINYGLADE_INC_BRDF_LUT_HLSL
#define TINYGLADE_INC_BRDF_LUT_HLSL



float2 sample_brdf_fg_lut(Texture2D brdf_fg_lut_tex, SamplerState s, float NdotV, float roughness)
{
    float2 uv = float2(saturate(NdotV), roughness * roughness) * 0.984375 + 0.0078125;
    return brdf_fg_lut_tex.SampleLevel(s, uv, 0.0).xy;
}

#endif 
#line 74 "shaders/deferred/plant_light_sun_sky.raster.hlsl"

#line 1 "shaders/terrain_editing/inc_highlight_active_terrain_chunk.inc.hlsl"


#ifndef TINYGLADE_TERRAIN_EDITING_INC_HIGHLIGHT_ACTIVE_TERRAIN_CHUNK_HLSL
#define TINYGLADE_TERRAIN_EDITING_INC_HIGHLIGHT_ACTIVE_TERRAIN_CHUNK_HLSL




float3 highlight_active_terrain_chunk(float3 surface_color, float3 highlight_color, float2 chunk_uv, float strength)
{
    return lerp(surface_color, highlight_color, strength);
}

#endif 
#line 75 "shaders/deferred/plant_light_sun_sky.raster.hlsl"

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
#line 76 "shaders/deferred/plant_light_sun_sky.raster.hlsl"


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
Texture2D<float4> blue_noise_texture : register(t18, space0);
Texture2D<uint4> rdi_reservoir_tex : register(t28, space0);
ByteAddressBuffer light_indirection_grid : register(t36, space0);

#if DEFERRED_FULL_LIGHTING
Texture2D<float4> half_depth_tex : register(t16, space0);
Texture2D<float4> ssgi_tex : register(t19, space0);
Texture2D<float4> screen_shadows_tex : register(t20, space0);
ByteAddressBuffer lantern_buf : register(t26, space0);
Texture2D<float4> indirect_radiance_tex : register(t27, space0);
ByteAddressBuffer grid_light_indices : register(t37, space0);
#endif





#if DEFERRED_FULL_LIGHTING


static float _249;
static uint _250;
static const float4 _255[3] = { 0.0f.xxxx, 0.0f.xxxx, 0.0f.xxxx };
static const Reservoir1sppStreamState _289 = { 0.0f, 0.0f };
static const Reservoir1spp _290 = { 0.0f, 0u, 0.0f, 0.0f };
float2 spvUnpackHalf2x16(uint value)
{
    return f16tof32(uint2(value & 0xffff, value >> 16));
}
float4 ps_main(VsToPs input) : SV_Target0
{
    float4 out_var_SV_TARGET;
    int2 _312 = int2(input.position.xy);
    uint2 _313 = uint2(_312);
    float4 _328 = ((blue_noise_texture.Load(int3((_313 + uint2(frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(frame_index)) + 0.5f.xx) * 256.0f.xx)) % uint2(256u, 256u), 0u)) * 255.0f) * 0.00390625f.xxxx) + 0.001953125f.xxxx;
    uint4 _330 = gbuffer_tex.Load(int3(_313, 0u));
    float4 _332 = depth_tex.Load(int3(_313, 0u));
    float _333 = _332.x;
    uint _334 = _330.x;
    float _349 = float((_334 >> 24u) & 255u) * 0.0039215688593685626983642578125f;
    uint _351 = _330.y;
    float _366 = float((_351 >> 24u) & 255u) * 0.0039215688593685626983642578125f;
    float3 _370 = (float4(float(_334 & 255u) * 0.0039215688593685626983642578125f, float((_334 >> 8u) & 255u) * 0.0039215688593685626983642578125f, float((_334 >> 16u) & 255u) * 0.0039215688593685626983642578125f, _349).xyz * 2.0f) - 1.0f.xxx;
    float3 _371 = float4(float(_351 & 255u) * 0.0039215688593685626983642578125f, float((_351 >> 8u) & 255u) * 0.0039215688593685626983642578125f, float((_351 >> 16u) & 255u) * 0.0039215688593685626983642578125f, _366).xyz;
    float3 _372 = _371 * _371;
    float3 _376 = mul(view_constants[2u].view_to_world, float4(0.0f, 0.0f, 1.0f, 0.0f)).xyz;
    float2 _379 = float2(depth_tex_dims);
    float2 _382 = (float2(_312) + 0.5f.xx) / _379;
    float2 _388 = (_382 - 0.5f.xx) * float2(2.0f, -2.0f);
    float _389 = _388.x;
    float4 _394 = float4(_389, _388.y, _333, 1.0f);
    float4 _396 = mul(view_constants[0u].view_to_world, mul(view_constants[0u].sample_to_view, _394));
    float3 _399 = -normalize(mul(view_constants[0u].view_to_world, mul(view_constants[0u].sample_to_view, float4(_389, _388.y, 0.0f, 1.0f))).xyz);
    float3 _403 = _396.xyz / _396.w.xxx;
    float _410 = clamp(0.100000001490116119384765625f + ssgi_tex.SampleLevel(sampler_nnc, _382, 0.0f).x, 0.0f, 1.0f);
    bool3 _418 = ((render_overrides & 1u) != 0u).xxx;
    float3 _419 = float3(_418.x ? 0.5f.xxx.x : _372.x, _418.y ? 0.5f.xxx.y : _372.y, _418.z ? 0.5f.xxx.z : _372.z);
    float _423 = 1.0f - screen_shadows_tex.Load(int3(_313, 0u)).x;
    float _426 = dot(_370, _376);
    float3 _485;
    do
    {
        bool _442;
        if (!(_426 <= 0.0f))
        {
            _442 = dot(_370, _399) <= 0.0f;
        }
        else
        {
            _442 = true;
        }
        if (_442)
        {
            _485 = 0.0f.xxx;
            break;
        }
        float3 _446 = normalize(_399 + _376);
        float _447 = dot(_370, _446);
        float _450 = max(0.0f, 1.0f - dot(_446, _376));
        float _451 = _450 * _450;
        float _456 = dot(_370, _399);
        float _476 = ((_447 * _447) * (-0.9375f)) + 1.0f;
        _485 = ((lerp(0.039999999105930328369140625f.xxx, 1.0f.xxx, ((_451 * _451) * _450).xxx) * (((2.0f * _426) * _456) / ((_426 * sqrt((((_456 * (-0.0625f)) + _456) * _456) + 0.0625f)) + (_456 * sqrt((((_426 * (-0.0625f)) + _426) * _426) + 0.0625f))))) * (0.0625f / ((3.1415927410125732421875f * _476) * _476))) / ((4.0f * _456) * _426).xxx;
        break;
    } while(false);
    float4 _306[3] = _255;
    float _499 = _328.x;
    uint _503 = uint((_499 * 3.0f) + 0.5f) & 3u;
    float _505;
    _505 = 0.0f;
    float _506;
    for (uint _508 = 0u; _508 < 3u; _505 = _506, _508++)
    {
        uint2 _520 = (_313 / uint2(2u, 2u)) + uint2(spatial_resolve_offsets_array[(_503 * 16u) + _508].xy);
        float _530 = exp2(-min(30.0f, 80.0f * abs(1.0f - (half_depth_tex.Load(int3(_520, 0u)).x / _333))));
        for (uint _532 = 0u; _532 < 3u; )
        {
            _306[_532] += (indirect_radiance_tex.Load(int3(_520 + uint2((_532 * indirect_radiance_tex_dims.x) / 3u, 0u), 0u)) * _530);
            _532++;
            continue;
        }
        _506 = _505 + _530;
    }
    float _597;
    if (_505 < 0.001000000047497451305389404296875f)
    {
        float _553;
        _553 = _505;
        float _554;
        for (uint _556 = 3u; _556 < 6u; _553 = _554, _556++)
        {
            uint2 _568 = (_313 / uint2(2u, 2u)) + uint2(spatial_resolve_offsets_array[(_503 * 16u) + _556].xy);
            float _578 = exp2(-min(30.0f, 80.0f * abs(1.0f - (half_depth_tex.Load(int3(_568, 0u)).x / _333))));
            for (uint _580 = 0u; _580 < 3u; )
            {
                _306[_580] += (indirect_radiance_tex.Load(int3(_568 + uint2((_580 * indirect_radiance_tex_dims.x) / 3u, 0u), 0u)) * _578);
                _580++;
                continue;
            }
            _554 = _553 + _578;
        }
        _597 = _553;
    }
    else
    {
        _597 = _505;
    }
    float _751;
    float _599 = 1.0f / max(9.9999997473787516355514526367188e-05f, _597);
    _306[0] *= _599;
    _306[1] *= _599;
    _306[2] *= _599;
    float3 _617 = (_306[0].yzw + _306[1].yzw) + _306[2].yzw;
    float3 _621 = _617 / max(9.9999996826552253889678874634872e-21f, length(_617)).xxx;
    float3 _625 = float3(_306[0].x, _306[1].x, _306[2].x);
    float3 _628 = _306[0].yzw / max(9.9999996826552253889678874634872e-21f, _306[0].x).xxx;
    float3 _631 = _306[1].yzw / max(9.9999996826552253889678874634872e-21f, _306[1].x).xxx;
    float3 _634 = _306[2].yzw / max(9.9999996826552253889678874634872e-21f, _306[2].x).xxx;
    float3 _640 = max(0.0f.xxx, 1.0f.xxx - (color_library.indirect_light_tint.xyz * 0.920000016689300537109375f));
    float3 _648 = max(0.0f.xxx, lerp((_640 / dot(_640, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f)).xxx) * dot(_625, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f)), _625, 1.16999995708465576171875f.xxx) * 1.2000000476837158203125f);
    float3 _654 = (-float3(_628.y, _631.y, _634.y)) * _648;
    float3 _659 = float3(_628.z, _631.z, _634.z) * _648;
    float3 _665 = (-float3(_628.x, _631.x, _634.x)) * _648;
    float3 _672 = normalize(float3(-dot(_665, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f)), -dot(_654, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f)), dot(_659, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f))));
    float3 _698 = float3(abs(dot(float3(-_665.x, -_654.x, _659.x), _672)), abs(dot(float3(-_665.y, -_654.y, _659.y), _672)), abs(dot(float3(-_665.z, -_654.z, _659.z), _672))) / _648;
    float _704 = dot(_672, _370);
    float3 _730 = max(max(0.0f.xxx, ((((0.886227548122406005859375f.xxx * _648) + ((1.0f.xxx * (_370.y * (-1.02332770824432373046875f))) * _654)) + ((1.0f.xxx * (_370.z * 1.02332770824432373046875f)) * _659)) + ((1.0f.xxx * (_370.x * (-1.02332770824432373046875f))) * _665)) + (((_648 * ((_698 * 0.07999999821186065673828125f) + ((_698 * 0.60000002384185791015625f) * _698))) * 0.25f) * (0.3153915703296661376953125f * (((3.0f * _704) * _704) - 1.0f)))), _648 * 2.0f);
    float _731 = lerp(0.5f, 1.0f, _410);
    float3 _801;
    do
    {
        _751 = dot(_370, _621);
        bool _758;
        if (!(_751 <= 0.0f))
        {
            _758 = dot(_370, _399) <= 0.0f;
        }
        else
        {
            _758 = true;
        }
        if (_758)
        {
            _801 = 0.0f.xxx;
            break;
        }
        float3 _762 = normalize(_399 + _621);
        float _763 = dot(_370, _762);
        float _766 = max(0.0f, 1.0f - dot(_762, _621));
        float _767 = _766 * _766;
        float _772 = dot(_370, _399);
        float _792 = ((_763 * _763) * (-0.9084937572479248046875f)) + 1.0f;
        _801 = ((lerp(0.039999999105930328369140625f.xxx, 1.0f.xxx, ((_767 * _767) * _766).xxx) * (((2.0f * _751) * _772) / ((_751 * sqrt((((_772 * (-0.09150625765323638916015625f)) + _772) * _772) + 0.09150625765323638916015625f)) + (_772 * sqrt((((_751 * (-0.09150625765323638916015625f)) + _751) * _751) + 0.09150625765323638916015625f))))) * (0.09150625765323638916015625f / ((3.1415927410125732421875f * _792) * _792))) / ((4.0f * _772) * _751).xxx;
        break;
    } while(false);
    float3 _808 = (0.3183098733425140380859375f.xxx * (((((_419 * 1.0f) * lerp(lerp(0.0f, 0.5f, _366), 1.0f, smoothstep(lerp(0.0f, -0.699999988079071044921875f, _366), lerp(1.0f, 0.699999988079071044921875f, _366), _426))) * sun_color.xyz) * _423) + ((((((_485 * sun_color.xyz) * _349) * (color_library.sky_base.xyz * color_library.sky_base.xyz)) * max(0.0f, _426)) * sun_color.xyz) * _423))) + ((((max(_731.xxx, ((((((_419 * 2.040400028228759765625f) - 0.3323999941349029541015625f.xxx) * _731) + ((_419 * (-4.79510021209716796875f)) + 0.6417000293731689453125f.xxx)) * _731) + ((_419 * 2.755199909210205078125f) + 0.69029998779296875f.xxx)) * _731) * 1.10000002384185791015625f) * _419) * _730) + ((((_730 * _349) * _801) * _410) * clamp(_751 * 12.0f, 0.0f, 4.0f)));
    float3 _2279;
    if ((lantern_count > 0u) ? true : false)
    {
        uint3 _819 = uint3(clamp(floor((_403 * float3(0.24615384638309478759765625f, 0.123076923191547393798828125f, 0.24615384638309478759765625f)) + float3(16.0f, 2.0f, 16.0f)), 0.0f.xxx, float3(31.0f, 15.0f, 31.0f)));
        uint _826 = (32u * ((32u * _819.y) + _819.z)) + _819.x;
        bool _831 = WaveActiveAllTrue((light_indirection_grid.Load<uint>(_826 * 4 + 0) >> 20u) <= 2u);
        uint _832 = _831 ? 2u : 6u;
        float3 _2278;
        if (6u == _832)
        {
            uint _307[8];
            for (uint _1383 = 0u; _1383 < 8u; )
            {
                _307[_1383] = 4294967295u;
                _1383++;
                continue;
            }
            uint2 _1390 = uint2(_312 & int2(1, 1));
            uint _1395 = (_1390.x + (_1390.y * 2u)) & 3u;
            uint _1397;
            _1397 = 0u;
            uint _1398;
            uint _1400 = 0u;
            for (;;)
            {
                bool _1408;
                if (_1400 < 8u)
                {
                    _1408 = _1397 < 8u;
                }
                else
                {
                    _1408 = false;
                }
                if (_1408)
                {
                    uint4 _1420 = rdi_reservoir_tex.Load(int3(uint2((_312 / int2(2, 2)) + spatial_resolve_offsets_array[((_1395 * 16u) + _1400) + 256u].xy), 0u));
                    uint2 _1421 = _1420.xy;
                    uint _1422 = _1420.x;
                    bool _1428;
                    if (any(bool2(_1421.x != uint2(0u, 0u).x, _1421.y != uint2(0u, 0u).y)))
                    {
                        _1428 = _1422 < lantern_count;
                    }
                    else
                    {
                        _1428 = false;
                    }
                    bool _1434;
                    if (_1428)
                    {
                        _1434 = _307[0u] != _1422;
                    }
                    else
                    {
                        _1434 = false;
                    }
                    bool _1440;
                    if (_1434)
                    {
                        _1440 = _307[1u] != _1422;
                    }
                    else
                    {
                        _1440 = false;
                    }
                    bool _1446;
                    if (_1440)
                    {
                        _1446 = _307[2u] != _1422;
                    }
                    else
                    {
                        _1446 = false;
                    }
                    bool _1452;
                    if (_1446)
                    {
                        _1452 = _307[3u] != _1422;
                    }
                    else
                    {
                        _1452 = false;
                    }
                    bool _1458;
                    if (_1452)
                    {
                        _1458 = _307[4u] != _1422;
                    }
                    else
                    {
                        _1458 = false;
                    }
                    bool _1464;
                    if (_1458)
                    {
                        _1464 = _307[5u] != _1422;
                    }
                    else
                    {
                        _1464 = false;
                    }
                    bool _1470;
                    if (_1464)
                    {
                        _1470 = _307[6u] != _1422;
                    }
                    else
                    {
                        _1470 = false;
                    }
                    bool _1476;
                    if (_1470)
                    {
                        _1476 = _307[7u] != _1422;
                    }
                    else
                    {
                        _1476 = false;
                    }
                    if (_1476)
                    {
                        _307[_1397] = _1422;
                        _1398 = _1397 + 1u;
                    }
                    else
                    {
                        _1398 = _1397;
                    }
                    _1397 = _1398;
                    _1400++;
                    continue;
                }
                else
                {
                    break;
                }
            }
            uint _1482;
            _1482 = _1397;
            uint _1483;
            for (uint _1485 = 0u; _1485 < 8u; _1482 = _1483, _1485++)
            {
                uint _1492 = QuadReadAcrossDiagonal(_307[_1485]);
                bool _1497;
                if (_1492 < lantern_count)
                {
                    _1497 = _1482 < 8u;
                }
                else
                {
                    _1497 = false;
                }
                if (_1497)
                {
                    bool _1508;
                    if (_307[0u] != _1492)
                    {
                        _1508 = _307[1u] != _1492;
                    }
                    else
                    {
                        _1508 = false;
                    }
                    bool _1514;
                    if (_1508)
                    {
                        _1514 = _307[2u] != _1492;
                    }
                    else
                    {
                        _1514 = false;
                    }
                    bool _1520;
                    if (_1514)
                    {
                        _1520 = _307[3u] != _1492;
                    }
                    else
                    {
                        _1520 = false;
                    }
                    bool _1526;
                    if (_1520)
                    {
                        _1526 = _307[4u] != _1492;
                    }
                    else
                    {
                        _1526 = false;
                    }
                    bool _1532;
                    if (_1526)
                    {
                        _1532 = _307[5u] != _1492;
                    }
                    else
                    {
                        _1532 = false;
                    }
                    bool _1538;
                    if (_1532)
                    {
                        _1538 = _307[6u] != _1492;
                    }
                    else
                    {
                        _1538 = false;
                    }
                    bool _1544;
                    if (_1538)
                    {
                        _1544 = _307[7u] != _1492;
                    }
                    else
                    {
                        _1544 = false;
                    }
                    uint _1549;
                    if (_1544)
                    {
                        _307[_1482] = _1492;
                        _1549 = _1482 + 1u;
                    }
                    else
                    {
                        _1549 = _1482;
                    }
                    _1483 = _1549;
                }
                else
                {
                    _1483 = _1482;
                }
            }
            Reservoir1sppStreamState _308[2];
            Reservoir1spp _309[2];
            for (uint _1551 = 0u; _1551 < 2u; )
            {
                _308[_1551] = _289;
                _309[_1551] = _290;
                _1551++;
                continue;
            }
            for (uint _1559 = 0u; _1559 < _1482; _1559++)
            {
                float2 _1572 = spvUnpackHalf2x16(lantern_buf.Load<uint>(_307[_1559] * 16 + 12) & 65535u);
                float _1573 = _1572.x;
                uint _1575 = lantern_buf.Load<uint>(_307[_1559] * 16 + 12) >> 16u;
                float3 _1576 = lantern_buf.Load<float3>(_307[_1559] * 16 + 0) - _403;
                float _1578 = dot(_1576, _1576);
                float3 _1585 = _1576 + float3(0.0f, 0.1500000059604644775390625f, 0.0f);
                float3 _1630;
                do
                {
                    if (_1575 == 0u)
                    {
                        _1630 = float3(0.7919800281524658203125f, 0.63358402252197265625f, 0.07919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.7919800281524658203125f).xxx;
                        break;
                    }
                    else
                    {
                        if (_1575 == 1u)
                        {
                            _1630 = float3(0.0f, 0.1583960056304931640625f, 0.7919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.215240001678466796875f).xxx;
                            break;
                        }
                        else
                        {
                            if (_1575 == 2u)
                            {
                                _1630 = float3(0.1583960056304931640625f, 0.7919800281524658203125f, 0.1583960056304931640625f) / max(1.0000000133514319600180897396058e-10f, 0.772159993648529052734375f).xxx;
                                break;
                            }
                            else
                            {
                                if (_1575 == 3u)
                                {
                                    _1630 = float3(0.23759402334690093994140625f, 0.1583960056304931640625f, 0.7919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.279020011425018310546875f).xxx;
                                    break;
                                }
                                else
                                {
                                    _1630 = float3(0.7919800281524658203125f, 0.23759402334690093994140625f, 0.07919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.4343800246715545654296875f).xxx;
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
                float _1634 = dot(((_1630 * 4.0f) * _1573) * ((max(0.0f, dot(_370, _1576 / length(_1576).xxx)) * ((1.0f / (0.3359999954700469970703125f + _1578)) * smoothstep(36.0f * _1573, 16.0f * _1573, _1578))) * smoothstep(-0.5f, 0.20000000298023223876953125f, (_1585 / length(_1585).xxx).y)), float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f));
                float _1635 = float(_1482);
                float _1639 = frac(_499 + (float(_1559) / _1635));
                if (_309[0].w_sum <= _309[1].w_sum)
                {
                    do
                    {
                        _308[0].M_sum += 1.0f;
                        float _1678 = _1634 * _1635;
                        bool _1693;
                        do
                        {
                            _309[0].w_sum += _1678;
                            _309[0].M += 1.0f;
                            if ((_1678 / _309[0].w_sum) >= _1639)
                            {
                                _309[0].payload = _307[_1559];
                                _1693 = true;
                                break;
                            }
                            else
                            {
                                _1693 = false;
                                break;
                            }
                            break; 
                        } while(false);
                        if (_1693)
                        {
                            _308[0].p_q_sel = _1634;
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
                        _308[1].M_sum += 1.0f;
                        float _1653 = _1634 * _1635;
                        bool _1668;
                        do
                        {
                            _309[1].w_sum += _1653;
                            _309[1].M += 1.0f;
                            if ((_1653 / _309[1].w_sum) >= _1639)
                            {
                                _309[1].payload = _307[_1559];
                                _1668 = true;
                                break;
                            }
                            else
                            {
                                _1668 = false;
                                break;
                            }
                            break; 
                        } while(false);
                        if (_1668)
                        {
                            _308[1].p_q_sel = _1634;
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
            for (uint _1699 = 1u; _1699 < 2u; )
            {
                _308[0].M_sum += _308[_1699].M_sum;
                _1699++;
                continue;
            }
            for (uint _1710 = 1u; _1710 < 2u; )
            {
                _308[_1710].M_sum = _308[0].M_sum;
                _1710++;
                continue;
            }
            float3 _1726;
            _1726 = _808;
            float _1720;
            float _1723;
            float _1725;
            float3 _1727;
            uint _1731;
            uint _1733;
            bool _1735;
            bool _1737;
            float _1719 = 0.0f;
            float _1722 = 0.0f;
            float _1724 = 0.0f;
            uint _1728 = 0u;
            uint _1730 = 0u;
            uint _1732 = 0u;
            bool _1734 = false;
            bool _1736 = false;
            for (; _1728 < 2u; _1719 = _1720, _1722 = _1723, _1724 = _1725, _1726 = _1727, _1728++, _1730 = _1731, _1732 = _1733, _1734 = _1735, _1736 = _1737)
            {
                _309[_1728].M = _308[_1728].M_sum;
                _309[_1728].W = _309[_1728].w_sum / max(9.9999999392252902907785028219223e-09f, _309[_1728].M * _308[_1728].p_q_sel);
                if (_309[_1728].W > 0.0f)
                {
                    float2 _1764 = spvUnpackHalf2x16(lantern_buf.Load<uint>(_309[_1728].payload * 16 + 12) & 65535u);
                    float _1765 = _1764.x;
                    uint _1767 = lantern_buf.Load<uint>(_309[_1728].payload * 16 + 12) >> 16u;
                    float3 _1768 = lantern_buf.Load<float3>(_309[_1728].payload * 16 + 0) - _403;
                    float _1769 = length(_1768);
                    float _1770 = dot(_1768, _1768);
                    float3 _1777 = _1768 + float3(0.0f, 0.1500000059604644775390625f, 0.0f);
                    float3 _1783 = _1769.xxx;
                    float3 _1822;
                    do
                    {
                        if (_1767 == 0u)
                        {
                            _1822 = float3(0.7919800281524658203125f, 0.63358402252197265625f, 0.07919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.7919800281524658203125f).xxx;
                            break;
                        }
                        else
                        {
                            if (_1767 == 1u)
                            {
                                _1822 = float3(0.0f, 0.1583960056304931640625f, 0.7919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.215240001678466796875f).xxx;
                                break;
                            }
                            else
                            {
                                if (_1767 == 2u)
                                {
                                    _1822 = float3(0.1583960056304931640625f, 0.7919800281524658203125f, 0.1583960056304931640625f) / max(1.0000000133514319600180897396058e-10f, 0.772159993648529052734375f).xxx;
                                    break;
                                }
                                else
                                {
                                    if (_1767 == 3u)
                                    {
                                        _1822 = float3(0.23759402334690093994140625f, 0.1583960056304931640625f, 0.7919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.279020011425018310546875f).xxx;
                                        break;
                                    }
                                    else
                                    {
                                        _1822 = float3(0.7919800281524658203125f, 0.23759402334690093994140625f, 0.07919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.4343800246715545654296875f).xxx;
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
                    float3 _1825 = ((_1822 * 4.0f) * _1765) * ((max(0.0f, dot(_370, _1768 / _1783)) * ((1.0f / (0.3359999954700469970703125f + _1770)) * smoothstep(36.0f * _1765, 16.0f * _1765, _1770))) * smoothstep(-0.5f, 0.20000000298023223876953125f, (_1777 / length(_1777).xxx).y));
                    float _2269;
                    float _2270;
                    float _2271;
                    uint _2272;
                    uint _2273;
                    bool _2274;
                    bool _2275;
                    float3 _2276;
                    if ((_309[_1728].W * max(max(_1825.x, _1825.y), _1825.z)) > 0.0f)
                    {
                        float _2084;
                        float _2085;
                        float _2086;
                        float3 _2087;
                        if (_1769 > 0.5f)
                        {
                            float3 _1845 = (_403 + ((_1768 * (_1769 - 0.5f)) / _1783)) + (_370 * (0.0199999995529651641845703125f * _328.y));
                            float3 _1846 = _394.xyz;
                            float4 _1856 = mul(view_constants[0u].view_to_sample, mul(view_constants[0u].world_to_view, float4(_1845, 1.0f)));
                            float3 _1860 = _1856.xyz / _1856.w.xxx;
                            float4 _1871 = _394 + (float4(_1860 - _1846, 0.0f) * float(int(sign(_1860.z))));
                            float _1872 = _1871.w;
                            float3 _1881 = (_1871 / (float((_1872 >= 0.0f) ? 1 : (-1)) * max(1.0000000133514319600180897396058e-10f, abs(_1872))).xxxx).xyz;
                            float3 _1882 = _1881 - _1846;
                            bool3 _1883 = bool3(_1882.x < 0.0f.xxx.x, _1882.y < 0.0f.xxx.y, _1882.z < 0.0f.xxx.z);
                            float3 _1886 = (float3(_1883.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, _1883.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, _1883.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - _1846) / _1882;
                            float3 _1892 = _1846 + (_1882 * max(0.0f, max(_1886.x, _1886.y)));
                            float3 _1893 = _1881 - _1892;
                            bool3 _1894 = bool3(_1893.x >= 0.0f.xxx.x, _1893.y >= 0.0f.xxx.y, _1893.z >= 0.0f.xxx.z);
                            float3 _1897 = (float3(_1894.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, _1894.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, _1894.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - _1892) / _1893;
                            float3 _1905 = _1892 + (_1893 * min(1.0f, min(min(_1897.x, _1897.y), _1897.z)));
                            float _2036;
                            float _2037;
                            float _2038;
                            bool _2051;
                            float _2052;
                            float _2053;
                            do
                            {
                                float _1926 = 0.800000011920928955078125f * view_constants[0u].clip_to_view[3].z;
                                uint _1927 = uint(int(max(2u, min(4u, uint(int(floor(length((((_1905.xy * float2(0.5f, -0.5f)) + 0.5f.xx) - ((_1892.xy * float2(0.5f, -0.5f)) + 0.5f.xx)) * _379))))))));
                                float _2031;
                                float _2035;
                                bool _2039;
                                do
                                {
                                    float3 _1932 = _1905 - _1892;
                                    float _1940;
                                    if (_1905.z > 9.5367431640625e-07f)
                                    {
                                        _1940 = float(_1927);
                                    }
                                    else
                                    {
                                        _1940 = max(1.0f, float(_1927) - 1.0f);
                                    }
                                    float _1941 = 1.0f / _1940;
                                    float _2026;
                                    float _2027;
                                    float _2028;
                                    float _2029;
                                    bool _2030;
                                    if (_1927 > 0u)
                                    {
                                        float _1947 = lerp(0.0f, 1.0f, pow(_499 * _1941, 1.0f));
                                        float3 _1949 = _1892 + (_1932 * _1947);
                                        float2 _1952 = (_1949.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                                        float _1954 = 1.0f / _1949.z;
                                        float _1959 = 1.0f / depth_tex.SampleLevel(sampler_llc, _1952, 0.0f).x;
                                        float _1963 = 1.0f / depth_tex.SampleLevel(sampler_nnc, _1952, 0.0f).x;
                                        float _1967 = (max(_1959, _1963) * 1.00000202655792236328125f) - _1954;
                                        float _1968 = _1954 - min(_1959, _1963);
                                        bool _1971 = (_1967 < 0.0f) ? (_1968 < _1926) : false;
                                        float _2020;
                                        float _2021;
                                        float _2022;
                                        float _2023;
                                        bool _2024;
                                        float _2025;
                                        if (_1971)
                                        {
                                            _2020 = _1967;
                                            _2021 = _1968;
                                            _2022 = _1926;
                                            _2023 = _1947;
                                            _2024 = _1971;
                                            _2025 = 0.0f;
                                        }
                                        else
                                        {
                                            float _1981;
                                            _1981 = _1947;
                                            bool _1980;
                                            float _1982;
                                            float _2015;
                                            float _2016;
                                            float _2017;
                                            float _2018;
                                            bool _2019;
                                            uint _1976 = 1u;
                                            bool _1979 = _1971;
                                            for (;;)
                                            {
                                                if (_1976 < _1927)
                                                {
                                                    _1982 = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(_1976) + _499) * _1941, 1.0f)));
                                                    float3 _1992 = _1892 + (_1932 * _1982);
                                                    float2 _1995 = (_1992.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                                                    float _1997 = 1.0f / _1992.z;
                                                    float _2001 = 1.0f / depth_tex.SampleLevel(sampler_llc, _1995, 0.0f).x;
                                                    float _2005 = 1.0f / depth_tex.SampleLevel(sampler_nnc, _1995, 0.0f).x;
                                                    float _2009 = (max(_2001, _2005) * 1.00000202655792236328125f) - _1997;
                                                    float _2010 = _1997 - min(_2001, _2005);
                                                    _1980 = (_2009 < 0.0f) ? (_2010 < _1926) : false;
                                                    if (_1980)
                                                    {
                                                        _2015 = _2009;
                                                        _2016 = _2010;
                                                        _2017 = _1926;
                                                        _2018 = _1982;
                                                        _2019 = _1980;
                                                        break;
                                                    }
                                                    _1976++;
                                                    _1979 = _1980;
                                                    _1981 = _1982;
                                                    continue;
                                                }
                                                else
                                                {
                                                    _2015 = 0.0f;
                                                    _2016 = 0.0f;
                                                    _2017 = 0.0f;
                                                    _2018 = 1.0f;
                                                    _2019 = _1979;
                                                    break;
                                                }
                                            }
                                            _2020 = _2015;
                                            _2021 = _2016;
                                            _2022 = _2017;
                                            _2023 = _2018;
                                            _2024 = _2019;
                                            _2025 = _1981;
                                        }
                                        _2026 = _2020;
                                        _2027 = _2021;
                                        _2028 = _2022;
                                        _2029 = _2023;
                                        _2030 = _2024;
                                        _2031 = _2025;
                                    }
                                    else
                                    {
                                        _2026 = 0.0f;
                                        _2027 = 0.0f;
                                        _2028 = 0.0f;
                                        _2029 = 1.0f;
                                        _2030 = false;
                                        _2031 = 0.0f;
                                    }
                                    if (_2030)
                                    {
                                        _2035 = _2029;
                                        _2036 = _2026;
                                        _2037 = _2028;
                                        _2038 = _2027;
                                        _2039 = true;
                                        break;
                                    }
                                    else
                                    {
                                        _2035 = _2031;
                                        _2036 = _1719;
                                        _2037 = _1722;
                                        _2038 = _1724;
                                        _2039 = false;
                                        break;
                                    }
                                    break; 
                                } while(false);
                                bool _2047;
                                if (_2039)
                                {
                                    bool _2046;
                                    if (_2038 < _2037)
                                    {
                                        _2046 = _2036 < _2037;
                                    }
                                    else
                                    {
                                        _2046 = false;
                                    }
                                    _2047 = _2046;
                                }
                                else
                                {
                                    _2047 = false;
                                }
                                if (_2047)
                                {
                                    _2051 = true;
                                    _2052 = _2035;
                                    _2053 = _2038 / view_constants[0u].clip_to_view[3].z;
                                    break;
                                }
                                _2051 = false;
                                _2052 = _2031;
                                _2053 = 0.0f;
                                break;
                            } while(false);
                            float3 _2083;
                            if (_2051)
                            {
                                float4 _2063 = mul(view_constants[0u].view_to_world, mul(view_constants[0u].sample_to_view, float4(lerp(_1892, _1905, _2052.xxx), 1.0f)));
                                float3 _2070 = ((_2063.xyz / _2063.w.xxx) + (_399 * _2053)) - lantern_buf.Load<float3>(_309[_1728].payload * 16 + 0);
                                bool _2080;
                                if (!(abs(_2070.y) > 0.5f))
                                {
                                    float2 _2077 = _2070.xz;
                                    _2080 = dot(_2077, _2077) > 0.136900007724761962890625f;
                                }
                                else
                                {
                                    _2080 = true;
                                }
                                bool3 _2081 = _2080.xxx;
                                _2083 = float3(_2081.x ? 0.0f.xxx.x : _1825.x, _2081.y ? 0.0f.xxx.y : _1825.y, _2081.z ? 0.0f.xxx.z : _1825.z);
                            }
                            else
                            {
                                _2083 = _1825;
                            }
                            _2084 = _2036;
                            _2085 = _2037;
                            _2086 = _2038;
                            _2087 = _2083;
                        }
                        else
                        {
                            _2084 = _1719;
                            _2085 = _1722;
                            _2086 = _1724;
                            _2087 = _1825;
                        }
                        float2 _2097 = float2((float(game_time_sec) + game_time_frac) * 4.0f, float(_309[_1728].payload * 10u));
                        int2 _2099 = int2(floor(_2097));
                        float2 _2100 = frac(_2097);
                        uint2 _2101 = uint2(_2099);
                        uint _2103 = _2101.y;
                        uint _2105 = _2103 + (_2103 << 10u);
                        uint _2107 = _2105 ^ (_2105 >> 6u);
                        uint _2109 = _2107 + (_2107 << 3u);
                        uint _2111 = _2109 ^ (_2109 >> 11u);
                        uint _2117 = (((_2101.x * 1664525u) + (_2111 + (_2111 << 15u))) + 1013904223u) * 1664525u;
                        uint _2119 = _2117 ^ (_2117 >> 11u);
                        uint _2122 = _2119 ^ ((_2119 << 7u) & 2636928640u);
                        uint _2125 = _2122 ^ ((_2122 << 15u) & 4022730752u);
                        bool _2128 = _1736 ? _1736 : true;
                        uint _2129 = _1736 ? _1732 : 8388607u;
                        bool _2130 = _1734 ? _1734 : true;
                        uint _2131 = _1734 ? _1730 : 1065353216u;
                        uint2 _2137 = uint2(_2099 + int2(1, 0));
                        uint _2139 = _2137.y;
                        uint _2141 = _2139 + (_2139 << 10u);
                        uint _2143 = _2141 ^ (_2141 >> 6u);
                        uint _2145 = _2143 + (_2143 << 3u);
                        uint _2147 = _2145 ^ (_2145 >> 11u);
                        uint _2153 = (((_2137.x * 1664525u) + (_2147 + (_2147 << 15u))) + 1013904223u) * 1664525u;
                        uint _2155 = _2153 ^ (_2153 >> 11u);
                        uint _2158 = _2155 ^ ((_2155 << 7u) & 2636928640u);
                        uint _2161 = _2158 ^ ((_2158 << 15u) & 4022730752u);
                        bool _2164 = _2128 ? _2128 : true;
                        uint _2165 = _2128 ? _2129 : 8388607u;
                        bool _2166 = _2130 ? _2130 : true;
                        uint _2167 = _2130 ? _2131 : 1065353216u;
                        uint2 _2173 = uint2(_2099 + int2(0, 1));
                        uint _2175 = _2173.y;
                        uint _2177 = _2175 + (_2175 << 10u);
                        uint _2179 = _2177 ^ (_2177 >> 6u);
                        uint _2181 = _2179 + (_2179 << 3u);
                        uint _2183 = _2181 ^ (_2181 >> 11u);
                        uint _2189 = (((_2173.x * 1664525u) + (_2183 + (_2183 << 15u))) + 1013904223u) * 1664525u;
                        uint _2191 = _2189 ^ (_2189 >> 11u);
                        uint _2194 = _2191 ^ ((_2191 << 7u) & 2636928640u);
                        uint _2197 = _2194 ^ ((_2194 << 15u) & 4022730752u);
                        bool _2200 = _2164 ? _2164 : true;
                        uint _2201 = _2164 ? _2165 : 8388607u;
                        bool _2202 = _2166 ? _2166 : true;
                        uint _2203 = _2166 ? _2167 : 1065353216u;
                        uint2 _2209 = uint2(_2099 + int2(1, 1));
                        uint _2211 = _2209.y;
                        uint _2213 = _2211 + (_2211 << 10u);
                        uint _2215 = _2213 ^ (_2213 >> 6u);
                        uint _2217 = _2215 + (_2215 << 3u);
                        uint _2219 = _2217 ^ (_2217 >> 11u);
                        uint _2225 = (((_2209.x * 1664525u) + (_2219 + (_2219 << 15u))) + 1013904223u) * 1664525u;
                        uint _2227 = _2225 ^ (_2225 >> 11u);
                        uint _2230 = _2227 ^ ((_2227 << 7u) & 2636928640u);
                        uint _2233 = _2230 ^ ((_2230 << 15u) & 4022730752u);
                        uint _2237 = _2200 ? _2201 : 8388607u;
                        uint _2239 = _2202 ? _2203 : 1065353216u;
                        float2 _2247 = (_2100 * _2100) * (3.0f.xx - (_2100 * 2.0f));
                        float _2248 = _2247.x;
                        float _2249 = 1.0f - _2248;
                        float _2251 = _2247.y;
                        _2269 = _2084;
                        _2270 = _2085;
                        _2271 = _2086;
                        _2272 = _2239;
                        _2273 = _2237;
                        _2274 = _2202 ? _2202 : true;
                        _2275 = _2200 ? _2200 : true;
                        _2276 = (((_2087 * lerp(0.5f, 1.0f, (((1.0f - _2251) * (((asfloat(((_2125 ^ (_2125 >> 18u)) & _2129) | _2131) - 1.0f) * _2249) + ((asfloat(((_2161 ^ (_2161 >> 18u)) & _2165) | _2167) - 1.0f) * _2248))) + (((asfloat(((_2197 ^ (_2197 >> 18u)) & _2201) | _2203) - 1.0f) * _2249) * _2251)) + (((asfloat(((_2233 ^ (_2233 >> 18u)) & _2237) | _2239) - 1.0f) * _2248) * _2251))) * _309[_1728].W) * _419) * ev_mult;
                    }
                    else
                    {
                        _2269 = _1719;
                        _2270 = _1722;
                        _2271 = _1724;
                        _2272 = _1730;
                        _2273 = _1732;
                        _2274 = _1734;
                        _2275 = _1736;
                        _2276 = 0.0f.xxx;
                    }
                    _1720 = _2269;
                    _1723 = _2270;
                    _1725 = _2271;
                    _1727 = _1726 + _2276;
                    _1731 = _2272;
                    _1733 = _2273;
                    _1735 = _2274;
                    _1737 = _2275;
                }
                else
                {
                    _1720 = _1719;
                    _1723 = _1722;
                    _1725 = _1724;
                    _1727 = _1726;
                    _1731 = _1730;
                    _1733 = _1732;
                    _1735 = _1734;
                    _1737 = _1736;
                }
            }
            _2278 = _1726;
        }
        else
        {
            float3 _1381;
            if (2u == _832)
            {
                uint _842 = light_indirection_grid.Load<uint>(_826 * 4 + 0) & 1048575u;
                uint _844 = min(64u, (light_indirection_grid.Load<uint>(_826 * 4 + 0) >> 20u));
                float3 _846;
                _846 = _808;
                float3 _847;
                float _850;
                float _852;
                float _854;
                uint _858;
                uint _860;
                bool _862;
                bool _864;
                float _849 = 0.0f;
                float _851 = 0.0f;
                float _853 = 0.0f;
                uint _855 = 0u;
                uint _857 = 0u;
                uint _859 = 0u;
                bool _861 = false;
                bool _863 = false;
                for (; _855 < _844; _846 = _847, _849 = _850, _851 = _852, _853 = _854, _855++, _857 = _858, _859 = _860, _861 = _862, _863 = _864)
                {
                    uint _868 = _842 + _855;
                    float2 _876 = spvUnpackHalf2x16(lantern_buf.Load<uint>(grid_light_indices.Load<uint>(_868 * 4 + 0) * 16 + 12) & 65535u);
                    float _877 = _876.x;
                    uint _879 = lantern_buf.Load<uint>(grid_light_indices.Load<uint>(_868 * 4 + 0) * 16 + 12) >> 16u;
                    float3 _880 = lantern_buf.Load<float3>(grid_light_indices.Load<uint>(_868 * 4 + 0) * 16 + 0) - _403;
                    float _881 = length(_880);
                    float _882 = dot(_880, _880);
                    float3 _889 = _880 + float3(0.0f, 0.1500000059604644775390625f, 0.0f);
                    float3 _895 = _881.xxx;
                    float3 _934;
                    do
                    {
                        if (_879 == 0u)
                        {
                            _934 = float3(0.7919800281524658203125f, 0.63358402252197265625f, 0.07919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.7919800281524658203125f).xxx;
                            break;
                        }
                        else
                        {
                            if (_879 == 1u)
                            {
                                _934 = float3(0.0f, 0.1583960056304931640625f, 0.7919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.215240001678466796875f).xxx;
                                break;
                            }
                            else
                            {
                                if (_879 == 2u)
                                {
                                    _934 = float3(0.1583960056304931640625f, 0.7919800281524658203125f, 0.1583960056304931640625f) / max(1.0000000133514319600180897396058e-10f, 0.772159993648529052734375f).xxx;
                                    break;
                                }
                                else
                                {
                                    if (_879 == 3u)
                                    {
                                        _934 = float3(0.23759402334690093994140625f, 0.1583960056304931640625f, 0.7919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.279020011425018310546875f).xxx;
                                        break;
                                    }
                                    else
                                    {
                                        _934 = float3(0.7919800281524658203125f, 0.23759402334690093994140625f, 0.07919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.4343800246715545654296875f).xxx;
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
                    float3 _937 = ((_934 * 4.0f) * _877) * ((max(0.0f, dot(_370, _880 / _895)) * ((1.0f / (0.3359999954700469970703125f + _882)) * smoothstep(36.0f * _877, 16.0f * _877, _882))) * smoothstep(-0.5f, 0.20000000298023223876953125f, (_889 / length(_889).xxx).y));
                    float3 _1380;
                    if (max(max(_937.x, _937.y), _937.z) > 0.0f)
                    {
                        float _1195;
                        float _1196;
                        float _1197;
                        float3 _1198;
                        if (_881 > 0.5f)
                        {
                            float3 _956 = (_403 + ((_880 * (_881 - 0.5f)) / _895)) + (_370 * (0.0199999995529651641845703125f * _328.y));
                            float3 _957 = _394.xyz;
                            float4 _967 = mul(view_constants[0u].view_to_sample, mul(view_constants[0u].world_to_view, float4(_956, 1.0f)));
                            float3 _971 = _967.xyz / _967.w.xxx;
                            float4 _982 = _394 + (float4(_971 - _957, 0.0f) * float(int(sign(_971.z))));
                            float _983 = _982.w;
                            float3 _992 = (_982 / (float((_983 >= 0.0f) ? 1 : (-1)) * max(1.0000000133514319600180897396058e-10f, abs(_983))).xxxx).xyz;
                            float3 _993 = _992 - _957;
                            bool3 _994 = bool3(_993.x < 0.0f.xxx.x, _993.y < 0.0f.xxx.y, _993.z < 0.0f.xxx.z);
                            float3 _997 = (float3(_994.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, _994.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, _994.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - _957) / _993;
                            float3 _1003 = _957 + (_993 * max(0.0f, max(_997.x, _997.y)));
                            float3 _1004 = _992 - _1003;
                            bool3 _1005 = bool3(_1004.x >= 0.0f.xxx.x, _1004.y >= 0.0f.xxx.y, _1004.z >= 0.0f.xxx.z);
                            float3 _1008 = (float3(_1005.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, _1005.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, _1005.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - _1003) / _1004;
                            float3 _1016 = _1003 + (_1004 * min(1.0f, min(min(_1008.x, _1008.y), _1008.z)));
                            float _1147;
                            float _1148;
                            float _1149;
                            bool _1162;
                            float _1163;
                            float _1164;
                            do
                            {
                                float _1037 = 0.800000011920928955078125f * view_constants[0u].clip_to_view[3].z;
                                uint _1038 = uint(int(max(2u, min(4u, uint(int(floor(length((((_1016.xy * float2(0.5f, -0.5f)) + 0.5f.xx) - ((_1003.xy * float2(0.5f, -0.5f)) + 0.5f.xx)) * _379))))))));
                                float _1142;
                                float _1146;
                                bool _1150;
                                do
                                {
                                    float3 _1043 = _1016 - _1003;
                                    float _1051;
                                    if (_1016.z > 9.5367431640625e-07f)
                                    {
                                        _1051 = float(_1038);
                                    }
                                    else
                                    {
                                        _1051 = max(1.0f, float(_1038) - 1.0f);
                                    }
                                    float _1052 = 1.0f / _1051;
                                    float _1137;
                                    float _1138;
                                    float _1139;
                                    float _1140;
                                    bool _1141;
                                    if (_1038 > 0u)
                                    {
                                        float _1058 = lerp(0.0f, 1.0f, pow(_499 * _1052, 1.0f));
                                        float3 _1060 = _1003 + (_1043 * _1058);
                                        float2 _1063 = (_1060.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                                        float _1065 = 1.0f / _1060.z;
                                        float _1070 = 1.0f / depth_tex.SampleLevel(sampler_llc, _1063, 0.0f).x;
                                        float _1074 = 1.0f / depth_tex.SampleLevel(sampler_nnc, _1063, 0.0f).x;
                                        float _1078 = (max(_1070, _1074) * 1.00000202655792236328125f) - _1065;
                                        float _1079 = _1065 - min(_1070, _1074);
                                        bool _1082 = (_1078 < 0.0f) ? (_1079 < _1037) : false;
                                        float _1131;
                                        float _1132;
                                        float _1133;
                                        float _1134;
                                        bool _1135;
                                        float _1136;
                                        if (_1082)
                                        {
                                            _1131 = _1078;
                                            _1132 = _1079;
                                            _1133 = _1037;
                                            _1134 = _1058;
                                            _1135 = _1082;
                                            _1136 = 0.0f;
                                        }
                                        else
                                        {
                                            float _1092;
                                            _1092 = _1058;
                                            bool _1091;
                                            float _1093;
                                            float _1126;
                                            float _1127;
                                            float _1128;
                                            float _1129;
                                            bool _1130;
                                            uint _1087 = 1u;
                                            bool _1090 = _1082;
                                            for (;;)
                                            {
                                                if (_1087 < _1038)
                                                {
                                                    _1093 = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(_1087) + _499) * _1052, 1.0f)));
                                                    float3 _1103 = _1003 + (_1043 * _1093);
                                                    float2 _1106 = (_1103.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                                                    float _1108 = 1.0f / _1103.z;
                                                    float _1112 = 1.0f / depth_tex.SampleLevel(sampler_llc, _1106, 0.0f).x;
                                                    float _1116 = 1.0f / depth_tex.SampleLevel(sampler_nnc, _1106, 0.0f).x;
                                                    float _1120 = (max(_1112, _1116) * 1.00000202655792236328125f) - _1108;
                                                    float _1121 = _1108 - min(_1112, _1116);
                                                    _1091 = (_1120 < 0.0f) ? (_1121 < _1037) : false;
                                                    if (_1091)
                                                    {
                                                        _1126 = _1120;
                                                        _1127 = _1121;
                                                        _1128 = _1037;
                                                        _1129 = _1093;
                                                        _1130 = _1091;
                                                        break;
                                                    }
                                                    _1087++;
                                                    _1090 = _1091;
                                                    _1092 = _1093;
                                                    continue;
                                                }
                                                else
                                                {
                                                    _1126 = 0.0f;
                                                    _1127 = 0.0f;
                                                    _1128 = 0.0f;
                                                    _1129 = 1.0f;
                                                    _1130 = _1090;
                                                    break;
                                                }
                                            }
                                            _1131 = _1126;
                                            _1132 = _1127;
                                            _1133 = _1128;
                                            _1134 = _1129;
                                            _1135 = _1130;
                                            _1136 = _1092;
                                        }
                                        _1137 = _1131;
                                        _1138 = _1132;
                                        _1139 = _1133;
                                        _1140 = _1134;
                                        _1141 = _1135;
                                        _1142 = _1136;
                                    }
                                    else
                                    {
                                        _1137 = 0.0f;
                                        _1138 = 0.0f;
                                        _1139 = 0.0f;
                                        _1140 = 1.0f;
                                        _1141 = false;
                                        _1142 = 0.0f;
                                    }
                                    if (_1141)
                                    {
                                        _1146 = _1140;
                                        _1147 = _1137;
                                        _1148 = _1139;
                                        _1149 = _1138;
                                        _1150 = true;
                                        break;
                                    }
                                    else
                                    {
                                        _1146 = _1142;
                                        _1147 = _849;
                                        _1148 = _851;
                                        _1149 = _853;
                                        _1150 = false;
                                        break;
                                    }
                                    break; 
                                } while(false);
                                bool _1158;
                                if (_1150)
                                {
                                    bool _1157;
                                    if (_1149 < _1148)
                                    {
                                        _1157 = _1147 < _1148;
                                    }
                                    else
                                    {
                                        _1157 = false;
                                    }
                                    _1158 = _1157;
                                }
                                else
                                {
                                    _1158 = false;
                                }
                                if (_1158)
                                {
                                    _1162 = true;
                                    _1163 = _1146;
                                    _1164 = _1149 / view_constants[0u].clip_to_view[3].z;
                                    break;
                                }
                                _1162 = false;
                                _1163 = _1142;
                                _1164 = 0.0f;
                                break;
                            } while(false);
                            float3 _1194;
                            if (_1162)
                            {
                                float4 _1174 = mul(view_constants[0u].view_to_world, mul(view_constants[0u].sample_to_view, float4(lerp(_1003, _1016, _1163.xxx), 1.0f)));
                                float3 _1181 = ((_1174.xyz / _1174.w.xxx) + (_399 * _1164)) - lantern_buf.Load<float3>(grid_light_indices.Load<uint>(_868 * 4 + 0) * 16 + 0);
                                bool _1191;
                                if (!(abs(_1181.y) > 0.5f))
                                {
                                    float2 _1188 = _1181.xz;
                                    _1191 = dot(_1188, _1188) > 0.136900007724761962890625f;
                                }
                                else
                                {
                                    _1191 = true;
                                }
                                bool3 _1192 = _1191.xxx;
                                _1194 = float3(_1192.x ? 0.0f.xxx.x : _937.x, _1192.y ? 0.0f.xxx.y : _937.y, _1192.z ? 0.0f.xxx.z : _937.z);
                            }
                            else
                            {
                                _1194 = _937;
                            }
                            _1195 = _1147;
                            _1196 = _1148;
                            _1197 = _1149;
                            _1198 = _1194;
                        }
                        else
                        {
                            _1195 = _849;
                            _1196 = _851;
                            _1197 = _853;
                            _1198 = _937;
                        }
                        float2 _1208 = float2((float(game_time_sec) + game_time_frac) * 4.0f, float(grid_light_indices.Load<uint>(_868 * 4 + 0) * 10u));
                        int2 _1210 = int2(floor(_1208));
                        float2 _1211 = frac(_1208);
                        uint2 _1212 = uint2(_1210);
                        uint _1214 = _1212.y;
                        uint _1216 = _1214 + (_1214 << 10u);
                        uint _1218 = _1216 ^ (_1216 >> 6u);
                        uint _1220 = _1218 + (_1218 << 3u);
                        uint _1222 = _1220 ^ (_1220 >> 11u);
                        uint _1228 = (((_1212.x * 1664525u) + (_1222 + (_1222 << 15u))) + 1013904223u) * 1664525u;
                        uint _1230 = _1228 ^ (_1228 >> 11u);
                        uint _1233 = _1230 ^ ((_1230 << 7u) & 2636928640u);
                        uint _1236 = _1233 ^ ((_1233 << 15u) & 4022730752u);
                        bool _1239 = _863 ? _863 : true;
                        uint _1240 = _863 ? _859 : 8388607u;
                        bool _1241 = _861 ? _861 : true;
                        uint _1242 = _861 ? _857 : 1065353216u;
                        uint2 _1248 = uint2(_1210 + int2(1, 0));
                        uint _1250 = _1248.y;
                        uint _1252 = _1250 + (_1250 << 10u);
                        uint _1254 = _1252 ^ (_1252 >> 6u);
                        uint _1256 = _1254 + (_1254 << 3u);
                        uint _1258 = _1256 ^ (_1256 >> 11u);
                        uint _1264 = (((_1248.x * 1664525u) + (_1258 + (_1258 << 15u))) + 1013904223u) * 1664525u;
                        uint _1266 = _1264 ^ (_1264 >> 11u);
                        uint _1269 = _1266 ^ ((_1266 << 7u) & 2636928640u);
                        uint _1272 = _1269 ^ ((_1269 << 15u) & 4022730752u);
                        bool _1275 = _1239 ? _1239 : true;
                        uint _1276 = _1239 ? _1240 : 8388607u;
                        bool _1277 = _1241 ? _1241 : true;
                        uint _1278 = _1241 ? _1242 : 1065353216u;
                        uint2 _1284 = uint2(_1210 + int2(0, 1));
                        uint _1286 = _1284.y;
                        uint _1288 = _1286 + (_1286 << 10u);
                        uint _1290 = _1288 ^ (_1288 >> 6u);
                        uint _1292 = _1290 + (_1290 << 3u);
                        uint _1294 = _1292 ^ (_1292 >> 11u);
                        uint _1300 = (((_1284.x * 1664525u) + (_1294 + (_1294 << 15u))) + 1013904223u) * 1664525u;
                        uint _1302 = _1300 ^ (_1300 >> 11u);
                        uint _1305 = _1302 ^ ((_1302 << 7u) & 2636928640u);
                        uint _1308 = _1305 ^ ((_1305 << 15u) & 4022730752u);
                        bool _1311 = _1275 ? _1275 : true;
                        uint _1312 = _1275 ? _1276 : 8388607u;
                        bool _1313 = _1277 ? _1277 : true;
                        uint _1314 = _1277 ? _1278 : 1065353216u;
                        uint2 _1320 = uint2(_1210 + int2(1, 1));
                        uint _1322 = _1320.y;
                        uint _1324 = _1322 + (_1322 << 10u);
                        uint _1326 = _1324 ^ (_1324 >> 6u);
                        uint _1328 = _1326 + (_1326 << 3u);
                        uint _1330 = _1328 ^ (_1328 >> 11u);
                        uint _1336 = (((_1320.x * 1664525u) + (_1330 + (_1330 << 15u))) + 1013904223u) * 1664525u;
                        uint _1338 = _1336 ^ (_1336 >> 11u);
                        uint _1341 = _1338 ^ ((_1338 << 7u) & 2636928640u);
                        uint _1344 = _1341 ^ ((_1341 << 15u) & 4022730752u);
                        uint _1348 = _1311 ? _1312 : 8388607u;
                        uint _1350 = _1313 ? _1314 : 1065353216u;
                        float2 _1358 = (_1211 * _1211) * (3.0f.xx - (_1211 * 2.0f));
                        float _1359 = _1358.x;
                        float _1360 = 1.0f - _1359;
                        float _1362 = _1358.y;
                        _850 = _1195;
                        _852 = _1196;
                        _854 = _1197;
                        _858 = _1350;
                        _860 = _1348;
                        _862 = _1313 ? _1313 : true;
                        _864 = _1311 ? _1311 : true;
                        _1380 = (((_1198 * lerp(0.5f, 1.0f, (((1.0f - _1362) * (((asfloat(((_1236 ^ (_1236 >> 18u)) & _1240) | _1242) - 1.0f) * _1360) + ((asfloat(((_1272 ^ (_1272 >> 18u)) & _1276) | _1278) - 1.0f) * _1359))) + (((asfloat(((_1308 ^ (_1308 >> 18u)) & _1312) | _1314) - 1.0f) * _1360) * _1362)) + (((asfloat(((_1344 ^ (_1344 >> 18u)) & _1348) | _1350) - 1.0f) * _1359) * _1362))) * 1.0f) * _419) * ev_mult;
                    }
                    else
                    {
                        _850 = _849;
                        _852 = _851;
                        _854 = _853;
                        _858 = _857;
                        _860 = _859;
                        _862 = _861;
                        _864 = _863;
                        _1380 = 0.0f.xxx;
                    }
                    _847 = _846 + _1380;
                }
                _1381 = _846;
            }
            else
            {
                _1381 = _808;
            }
            _2278 = _1381;
        }
        _2279 = _2278;
    }
    else
    {
        _2279 = _808;
    }
    out_var_SV_TARGET = float4(_2279, 0.0f);
    return out_var_SV_TARGET;
}
#else



void ps_main(VsToPs input)
{
    int2 _142 = int2(input.position.xy);
    float4 _163 = mul(view_constants[0u].view_to_world, mul(view_constants[0u].sample_to_view, float4((((float2(_142) + 0.5f.xx) / float2(depth_tex_dims)) - 0.5f.xx) * float2(2.0f, -2.0f), depth_tex.Load(int3(uint2(_142), 0u)).x, 1.0f)));
    if ((lantern_count > 0u) ? true : false)
    {
        uint3 _178 = uint3(clamp(floor(((_163.xyz / _163.w.xxx) * float3(0.24615384638309478759765625f, 0.123076923191547393798828125f, 0.24615384638309478759765625f)) + float3(16.0f, 2.0f, 16.0f)), 0.0f.xxx, float3(31.0f, 15.0f, 31.0f)));
        if (6u == (WaveActiveAllTrue((light_indirection_grid.Load<uint>(((32u * ((32u * _178.y) + _178.z)) + _178.x) * 4 + 0) >> 20u) <= 2u) ? 2u : 6u))
        {
            uint _139[8];
            for (uint _197 = 0u; _197 < 8u; )
            {
                _139[_197] = 4294967295u;
                _197++;
                continue;
            }
            uint2 _204 = uint2(_142 & int2(1, 1));
            uint _209 = (_204.x + (_204.y * 2u)) & 3u;
            uint _211;
            _211 = 0u;
            uint _212;
            uint _214 = 0u;
            for (;;)
            {
                bool _222;
                if (_214 < 8u)
                {
                    _222 = _211 < 8u;
                }
                else
                {
                    _222 = false;
                }
                if (_222)
                {
                    uint4 _234 = rdi_reservoir_tex.Load(int3(uint2((_142 / int2(2, 2)) + spatial_resolve_offsets_array[((_209 * 16u) + _214) + 256u].xy), 0u));
                    uint2 _235 = _234.xy;
                    uint _236 = _234.x;
                    bool _242;
                    if (any(bool2(_235.x != uint2(0u, 0u).x, _235.y != uint2(0u, 0u).y)))
                    {
                        _242 = _236 < lantern_count;
                    }
                    else
                    {
                        _242 = false;
                    }
                    bool _248;
                    if (_242)
                    {
                        _248 = _139[0u] != _236;
                    }
                    else
                    {
                        _248 = false;
                    }
                    bool _254;
                    if (_248)
                    {
                        _254 = _139[1u] != _236;
                    }
                    else
                    {
                        _254 = false;
                    }
                    bool _260;
                    if (_254)
                    {
                        _260 = _139[2u] != _236;
                    }
                    else
                    {
                        _260 = false;
                    }
                    bool _266;
                    if (_260)
                    {
                        _266 = _139[3u] != _236;
                    }
                    else
                    {
                        _266 = false;
                    }
                    bool _272;
                    if (_266)
                    {
                        _272 = _139[4u] != _236;
                    }
                    else
                    {
                        _272 = false;
                    }
                    bool _278;
                    if (_272)
                    {
                        _278 = _139[5u] != _236;
                    }
                    else
                    {
                        _278 = false;
                    }
                    bool _284;
                    if (_278)
                    {
                        _284 = _139[6u] != _236;
                    }
                    else
                    {
                        _284 = false;
                    }
                    bool _290;
                    if (_284)
                    {
                        _290 = _139[7u] != _236;
                    }
                    else
                    {
                        _290 = false;
                    }
                    if (_290)
                    {
                        _139[_211] = _236;
                        _212 = _211 + 1u;
                    }
                    else
                    {
                        _212 = _211;
                    }
                    _211 = _212;
                    _214++;
                    continue;
                }
                else
                {
                    break;
                }
            }
            uint _297;
            for (uint _296 = _211, _299 = 0u; _299 < 8u; _296 = _297, _299++)
            {
                uint _306 = QuadReadAcrossDiagonal(_139[_299]);
                bool _311;
                if (_306 < lantern_count)
                {
                    _311 = _296 < 8u;
                }
                else
                {
                    _311 = false;
                }
                if (_311)
                {
                    bool _322;
                    if (_139[0u] != _306)
                    {
                        _322 = _139[1u] != _306;
                    }
                    else
                    {
                        _322 = false;
                    }
                    bool _328;
                    if (_322)
                    {
                        _328 = _139[2u] != _306;
                    }
                    else
                    {
                        _328 = false;
                    }
                    bool _334;
                    if (_328)
                    {
                        _334 = _139[3u] != _306;
                    }
                    else
                    {
                        _334 = false;
                    }
                    bool _340;
                    if (_334)
                    {
                        _340 = _139[4u] != _306;
                    }
                    else
                    {
                        _340 = false;
                    }
                    bool _346;
                    if (_340)
                    {
                        _346 = _139[5u] != _306;
                    }
                    else
                    {
                        _346 = false;
                    }
                    bool _352;
                    if (_346)
                    {
                        _352 = _139[6u] != _306;
                    }
                    else
                    {
                        _352 = false;
                    }
                    bool _358;
                    if (_352)
                    {
                        _358 = _139[7u] != _306;
                    }
                    else
                    {
                        _358 = false;
                    }
                    uint _363;
                    if (_358)
                    {
                        _139[_296] = _306;
                        _363 = _296 + 1u;
                    }
                    else
                    {
                        _363 = _296;
                    }
                    _297 = _363;
                }
                else
                {
                    _297 = _296;
                }
            }
        }
        else
        {
        }
    }
}
#endif
