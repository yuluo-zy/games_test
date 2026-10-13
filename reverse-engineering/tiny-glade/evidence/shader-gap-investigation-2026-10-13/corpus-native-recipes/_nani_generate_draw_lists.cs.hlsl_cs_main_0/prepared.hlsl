#line 1 "shaders/nani/generate_draw_lists.cs.hlsl"




#line 1 "shaders/nani/generate_draw_lists.cs.hlsl"

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
#line 7 "shaders/nani/generate_draw_lists.cs.hlsl"

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
#line 8 "shaders/nani/generate_draw_lists.cs.hlsl"

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
#line 9 "shaders/nani/generate_draw_lists.cs.hlsl"

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
#line 10 "shaders/nani/generate_draw_lists.cs.hlsl"

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
#line 11 "shaders/nani/generate_draw_lists.cs.hlsl"

#line 1 "shaders/nani/draw_lists.inc.hlsl"






#ifndef TINYGLADE_NANI_DRAW_LISTS_HLSL
#define TINYGLADE_NANI_DRAW_LISTS_HLSL


struct GpuFilteredInstance
{
    uint raw;
};


struct GpuFilteredInstanceMeta
{
    uint raw;
};


struct NaniDrawListInfo
{
    uint global_instance_offset;
};


struct DrawIndexedIndirectCommand
{
    uint index_count;
    uint instance_count;
    uint first_index;
    uint vertex_offset;
    uint first_instance;
};

#endif 
#line 12 "shaders/nani/generate_draw_lists.cs.hlsl"

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
#line 13 "shaders/nani/generate_draw_lists.cs.hlsl"

#line 1 "shaders/nani/draw_list_offsets.inc.hlsl"






#ifndef TINYGLADE_NANI_DRAW_LIST_OFFSETS_HLSL
#define TINYGLADE_NANI_DRAW_LIST_OFFSETS_HLSL



cbuffer instance_list_offset : register(b13, space0)
{
    uint4 instance_list_offset_array[27];
};


uint instance_list_offset_at(uint draw_list_index)
{
    uint packed_index = draw_list_index / 4u;
    uint lane = draw_list_index % 4u;
    return instance_list_offset_array[packed_index][lane];
}

#endif 
#line 14 "shaders/nani/generate_draw_lists.cs.hlsl"

#line 1 "shaders/nani/atlas_part_info.inc.hlsl"






#ifndef TINYGLADE_NANI_ATLAS_PART_INFO_HLSL
#define TINYGLADE_NANI_ATLAS_PART_INFO_HLSL


struct MeshAtlasPartInfo
{
    uint first_index;
    uint index_count;
    float bounding_sphere_radius;
    uint pad0;
};

#endif 
#line 15 "shaders/nani/generate_draw_lists.cs.hlsl"


StructuredBuffer<MeshAtlasPartInfo[64]> atlas_part_info : register(t14, space0);
StructuredBuffer<uint> sorted_instances_buffer : register(t15, space0);
StructuredBuffer<uint> thread_count_buffer : register(t16, space0);
StructuredBuffer<uint> thread_group_draw_cmd_histogram : register(t17, space0);
StructuredBuffer<uint> thread_group_draw_cmd_subtotals : register(t18, space0);
RWStructuredBuffer<DrawIndexedIndirectCommand> __alias_draw_list_buffer : register(u19, space0);
RWStructuredBuffer<uint> __alias_instance_indirection_buffer : register(u20, space0);
RWStructuredBuffer<uint> __alias_instance_indirection_allocator_buffer : register(u21, space0);

struct PushConstants
{
    uint world_subset;
    uint draw_list_index;
    uint list_draw_cmd_offset;
    uint list_merge_window_offset;
    uint list_merge_window_subtotal_offset;
};

[[vk::push_constant]]
PushConstants push_constants;

groupshared uint mesh_histogram[64];
groupshared uint group_draw_offset;

[numthreads(32, 1, 1)]
void cs_main(uint3 group_id : SV_GroupID, uint3 thread_id : SV_DispatchThreadID, uint local_index : SV_GroupIndex)
{
    for (uint i = local_index; i < 64u; i += 32u)
    {
        mesh_histogram[i] = 0u;
    }

    if (local_index == 0u)
    {
        group_draw_offset = thread_group_draw_cmd_histogram[push_constants.list_merge_window_offset + group_id.x] +
                            thread_group_draw_cmd_subtotals[push_constants.list_merge_window_subtotal_offset + (group_id.x / 1024u)];
    }

    uint count = thread_count_buffer[push_constants.draw_list_index];
    bool in_range = thread_id.x < count;
    uint instance_index = in_range ? sorted_instances_buffer[thread_id.x + instance_list_offset_at(push_constants.draw_list_index)] : 0u;
    uint mesh_id = instance_index / 64u;
    uint bucket = instance_index % 64u;

    GroupMemoryBarrierWithGroupSync();
    if (in_range)
    {
        InterlockedAdd(mesh_histogram[bucket], 1u);
    }
    GroupMemoryBarrierWithGroupSync();

    for (uint j = local_index; j < 64u; j += 32u)
    {
        uint bucket_count = mesh_histogram[j];
        if (bucket_count > 0u)
        {
            uint draw_cmd;
            InterlockedAdd(group_draw_offset, 1u, draw_cmd);
            uint cmd_index = draw_cmd + push_constants.list_draw_cmd_offset;
            uint base_instance;
            InterlockedAdd(__alias_instance_indirection_allocator_buffer[push_constants.draw_list_index], bucket_count, base_instance);
            DrawIndexedIndirectCommand cmd;
            cmd.index_count = atlas_part_info[push_constants.world_subset][j].index_count;
            cmd.instance_count = bucket_count;
            cmd.first_index = atlas_part_info[push_constants.world_subset][j].first_index;
            cmd.vertex_offset = 0u;
            cmd.first_instance = base_instance;
            __alias_draw_list_buffer[cmd_index] = cmd;
            mesh_histogram[j] = base_instance;
        }
    }
    GroupMemoryBarrierWithGroupSync();

    if (in_range)
    {
        uint slot;
        InterlockedAdd(mesh_histogram[bucket], 1u, slot);
        __alias_instance_indirection_buffer[slot] = mesh_id;
    }
}
