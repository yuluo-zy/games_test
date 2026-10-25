#define SHADER_VARIANT_DEPTH_ONLY 0
#line 1 "shaders/sparkles.raster.hlsl"










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
#line 11 "shaders/sparkles.raster.hlsl"

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
#line 12 "shaders/sparkles.raster.hlsl"

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
#line 13 "shaders/sparkles.raster.hlsl"

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
#line 14 "shaders/sparkles.raster.hlsl"







struct SparkleInstanceData
{
    float3 position;
    float scale;
    float time_offset;
    float speed_mult;
    int color_id;
    float pad1;
};

StructuredBuffer<SparkleInstanceData> instanced_data : register(t14);



float sparkle_noise(float2 v)
{
    const float F2 = 0.366025403784439f;
    const float G2 = 0.211324865405187f;
    float2 i = floor(v + dot(v, F2.xx).xx);
    float2 x0 = (v - i) + dot(i, G2.xx).xx;
    float x0x = x0.x;
    float x0y = x0.y;
    bool2 ge = (x0x > x0y).xx;
    float2 i1 = select(ge, float2(1.0f, 0.0f), float2(0.0f, 1.0f));

    float4 x1x2 = x0.xyxy + float4(G2, G2, -0.577350269189626f, -0.577350269189626f);
    float2 x1 = x1x2.xy - i1;
    float4 x1x2b = float4(x1.x, x1.y, x1x2.z, x1x2.w);
    float2 x2 = x1x2.zw;

    float2 im = fmod(i, 289.0f);
    float3 p0 = im.y.xxx + float3(0.0f, i1.y, 1.0f);
    float3 p1 = (fmod((p0 * 34.0f + 1.0f) * p0, 289.0f) + im.x.xxx) + float3(0.0f, i1.x, 1.0f);

    float3 m = max(0.5f - float3(dot(x0, x0), dot(x1, x1), dot(x2, x2)), 0.0f);
    m = m * m;
    m = m * m;

    float3 n2 = frac(fmod((p1 * 34.0f + 1.0f) * p1, 289.0f) * (1.0f / 41.0f)) * 2.0f;
    float3 n = n2 - 1.0f;
    float3 s = abs(n) - 0.5f;
    float3 t = n - floor(n2 - 0.5f);

    float3 grad;
    grad.x = t.x * x0x + s.x * x0y;
    float2 gradyz = (t.yz * x1x2b.xz) + (s.yz * x1x2b.yw);
    return dot(m * (1.79284291400159f - ((t * t + s * s) * 0.85373472095314f)), float3(grad.x, gradyz)) * 13.0f;
}

struct VsIn
{
    float3 Vertex_Position : TEXCOORD0;
    float3 Vertex_Color : TEXCOORD1;
    float3 alt_pos : TEXCOORD2;
    uint id : SV_InstanceID;
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

    
    float speed_mult = instanced_data[vin.id].speed_mult;
    float time_offset = instanced_data[vin.id].time_offset;
    float speed = (1.0f + speed_mult) * 0.7f;
    float phase = frac(((fmod(float(game_time_sec) * speed, TWO_PI) + (game_time_frac * speed)) * INV_TWO_PI) + time_offset);
    bool3 big = (time_offset * speed_mult > 0.4f).xxx;
    float pulse = smoothstep(0.0f, 0.6f, phase) * (1.0f - smoothstep(0.6f, 1.0f, phase));
    float3 base = select(big, vin.alt_pos, vin.Vertex_Position)
        * ((1.0f - pow(1.0f - pulse, 3.0f)) * (0.18f + instanced_data[vin.id].scale * 0.1f))
        + instanced_data[vin.id].position + float3(0.0f, phase * 1.4f - 0.5f, 0.0f);
    float3 displaced = base + (sparkle_noise(((base.xz + base.xy) + base.yz) * 0.6f)).xxx;

    
    float prev_speed_mult = instanced_data[vin.id].speed_mult;
    float prev_time_offset = instanced_data[vin.id].time_offset;
    float prev_speed = (1.0f + prev_speed_mult) * 0.7f;
    float prev_phase = frac((((fmod(float(game_time_sec) * prev_speed, TWO_PI) + (game_time_frac * prev_speed))
        + (-delta_time_seconds * prev_speed)) * INV_TWO_PI) + prev_time_offset);
    bool3 prev_big = (prev_time_offset * prev_speed_mult > 0.4f).xxx;
    float prev_pulse = smoothstep(0.0f, 0.6f, prev_phase) * (1.0f - smoothstep(0.6f, 1.0f, prev_phase));
    float3 prev_base = select(prev_big, vin.alt_pos, vin.Vertex_Position)
        * ((1.0f - pow(1.0f - prev_pulse, 3.0f)) * (0.18f + instanced_data[vin.id].scale * 0.1f))
        + instanced_data[vin.id].position + float3(0.0f, prev_phase * 1.4f - 0.5f, 0.0f);
    float3 prev_displaced = prev_base + (sparkle_noise(((prev_base.xz + prev_base.xy) + prev_base.yz) * 0.6f)).xxx;

    
    int color_id = instanced_data[vin.id].color_id;
    float3 color;
    if (color_id == 0)
    {
        color = vin.Vertex_Color;
    }
    else if (color_id == 1)
    {
        color = float3(0.0f, 0.75f, 10.0f);
    }
    else if (color_id == 2)
    {
        color = float3(2.0f, 1.0f, 8.0f);
    }
    else if (color_id == 3)
    {
        color = float3(8.0f, 3.0f, 0.0f);
    }
    else
    {
        color = select(color_id == 4, float3(2.0f, 9.0f, 2.0f), float3(0.0f, 0.0f, 0.0f));
    }

    o.position = mul(view_constants[current_view_index].world_to_sample, float4(displaced, 1.0f));
    o.C0 = color;
    o.C1 = displaced - prev_displaced;
    return o;
}

struct PsIn
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
};

struct PsOut
{
    float4 color : SV_Target0;
    float3 delta : SV_Target1;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;
    o.color = float4(pin.C0, 1.0f);
    o.delta = pin.C1;
    return o;
}
