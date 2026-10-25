#define SHADER_VARIANT_DEPTH_ONLY 1
#line 1 "shaders/critters/firefly_glow.raster.hlsl"


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
#line 3 "shaders/critters/firefly_glow.raster.hlsl"

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
#line 4 "shaders/critters/firefly_glow.raster.hlsl"

#line 1 "shaders/inc/current_view.hlsl"





#ifndef TINYGLADE_INC_CURRENT_VIEW_HLSL
#define TINYGLADE_INC_CURRENT_VIEW_HLSL

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
#line 9 "shaders/inc/current_view.hlsl"


uint current_view_index;


ViewConstants GetCurrentViewConstants()
{
    return view_constants[current_view_index];
}

#endif 
#line 5 "shaders/critters/firefly_glow.raster.hlsl"

Texture2D<float4> terrain_heightmap : register(t16);
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
#line 7 "shaders/critters/firefly_glow.raster.hlsl"

#line 1 "shaders/inc/coc.hlsl"







#ifndef TINYGLADE_INC_COC_HLSL
#define TINYGLADE_INC_COC_HLSL


struct DofFocus {
    float focus0;
    float focus1;
    float lock_bias0;
    float lock_bias1;
    float3 lock_pos_ws;
    uint is_locked;
    float3 interp_lock_pos_ws;
    uint use_tilt_shift;
    float interp_lock_bias0;
    float interp_lock_bias1;
    float focus_scale;
    uint pad2;
};


struct DofFocusReadback {
    float3 lock_pos_ws;
    uint lock_reponse; 
};



float compute_coc(float depth, float focus_dist, float focus_scale)
{
    float coc = (focus_dist - depth) / max(depth, 1e-6);
    return clamp(coc * focus_scale, -1.0, 1.0);
}

#endif 
#line 8 "shaders/critters/firefly_glow.raster.hlsl"

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
#line 9 "shaders/critters/firefly_glow.raster.hlsl"






struct Firefly
{
    float3 position;
    uint pad;
};

StructuredBuffer<Firefly> firefly_buffer : register(t18);
StructuredBuffer<float> firefly_visibility_buffer : register(t19);
StructuredBuffer<DofFocus> autofocus_buf : register(t20);

uint2 terrain_heightmap_dims;
uint2 visual_terrain_heightmap_dims;
float firefly_visibility;
uint2 framebuffer_dims;

struct VsIn
{
    float3 Vertex_Position : TEXCOORD0;
    uint instance_id : SV_InstanceID;
};

struct VsOut
{
    float2 C0 : TEXCOORD0;
    float C1 : TEXCOORD1;
    float C2 : TEXCOORD2;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;
    if (firefly_visibility_buffer[vin.instance_id] == 0.0f)
    {
        o.position = asfloat(0x7fbfffff).xxxx;
        o.C0 = asfloat(0x7fbfffff).xx;
        o.C1 = asfloat(0x7fbfffff);
        o.C2 = asfloat(0x7fbfffff);
        return o;
    }

    float3 firefly_pos = firefly_buffer[vin.instance_id].position;
    float3 world_pos = firefly_pos + float3(0.0f,
        lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, firefly_pos.xz * 0.0076923076994717121124267578125f.xx + 0.5f.xx, 0.0f).x),
        0.0f);
    float4 view_pos = mul(view_constants[current_view_index].world_to_view, float4(world_pos, 1.0f));

    float inv_focus0 = 1.0f / autofocus_buf[camera_idx].focus0;
    float inv_focus1 = 1.0f / autofocus_buf[camera_idx].focus1;
    float near_plane = max(0.001000000047497451305389404296875f, max(inv_focus0, inv_focus1)) + 9.9999997473787516355514526367188e-06f;
    float far_plane = max(0.001000000047497451305389404296875f, min(inv_focus0, inv_focus1)) - 9.9999997473787516355514526367188e-06f;
    bool use_tilt_shift = autofocus_buf[camera_idx].use_tilt_shift != 0u;

    float3 proj_pos = world_pos;
    proj_pos.y = world_pos.y * float(use_tilt_shift ? 0 : 1);
    float4 cam_pos4 = mul(view_constants[0].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 cam_forward = normalize(mul(view_constants[0].view_to_world, float4(0.0f, 0.0f, -1.0f, 0.0f)).xyz);
    float depth = 1.0f / max(1.0000000133514319600180897396058e-10f, dot(proj_pos - (cam_pos4.xyz / cam_pos4.w.xxx), cam_forward));

    float range = near_plane - far_plane;
    float blur = min(0.00999999977648258209228515625f, 0.0500000007450580596923828125f * range);
    float coc;
    if (depth > near_plane)
    {
        coc = (depth - near_plane) + blur;
    }
    else if (depth < far_plane)
    {
        coc = (depth - far_plane) - blur;
    }
    else
    {
        coc = (-blur) + (((2.0f * blur) * (depth - far_plane)) / range);
    }
    if (use_tilt_shift)
    {
        coc = coc / max(0.00999999977648258209228515625f, length(cam_forward.xz));
    }

    float coc_px = ((((((-coc) * autofocus_buf[camera_idx].focus_scale) * view_constants[0].view_to_clip[1].y) * 0.5f) * view_constants[0].view_to_clip[1].y) / float(framebuffer_dims.y)) * (-view_pos.z);
    float scale = 0.800000011920928955078125f + coc_px;
    float k = coc_px / scale;

    o.position = mul(view_constants[current_view_index].view_to_sample, view_pos + float4(vin.Vertex_Position.xzy * scale, 0.0f));
    o.C0 = vin.Vertex_Position.xz * 2.0f;
    o.C1 = clamp(((firefly_visibility_buffer[vin.instance_id] - 0.20000000298023223876953125f) * 0.753982365131378173828125f) / ((scale * scale) * (3.1415927410125732421875f * (0.300000011920928955078125f + (0.699999988079071044921875f * (k * (0.579999983310699462890625f + (0.4199999868869781494140625f * k))))))), 0.0f, 1.0f);
    o.C2 = k;
    return o;
}

struct PsIn
{
    float2 C0 : TEXCOORD0;
    float C1 : TEXCOORD1;
    float C2 : TEXCOORD2;
};

struct PsOut
{
    float4 color : SV_Target0;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;
    float4 col = float4(0.039999999105930328369140625f, 0.039999999105930328369140625f, 0.00200000009499490261077880859375f, 0.0f) * ev_mult;
    col *= pin.C1;
    col *= smoothstep(1.0f, pin.C2, length(pin.C0));
    col *= firefly_visibility;
    o.color = col;
    return o;
}
