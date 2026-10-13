#define TGV_0 1
#line 1 "shaders/ssgi/temporal_filter.cs.hlsl"


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
#line 18 "shaders/ssgi/temporal_filter.cs.hlsl"

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
#line 19 "shaders/ssgi/temporal_filter.cs.hlsl"

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
#line 20 "shaders/ssgi/temporal_filter.cs.hlsl"

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
#line 21 "shaders/ssgi/temporal_filter.cs.hlsl"

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
#line 22 "shaders/ssgi/temporal_filter.cs.hlsl"




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
#line 26 "shaders/ssgi/temporal_filter.cs.hlsl"


cbuffer $Globals : register(b13, space0)
{
    uint history_valid;
    uint2 input_tex_dims;
    uint2 history_tex_dims;
    uint2 reprojection_tex_dims;
    uint2 final_output_tex_dims;
    uint2 history_output_tex_dims;
};

Texture2D<float4> input_tex : register(t14, space0);
Texture2D<float4> history_tex : register(t15, space0);
Texture2D<float4> reprojection_tex : register(t16, space0);
RWTexture2D<unorm float> final_output_tex : register(u17, space0);
RWTexture2D<unorm float> history_output_tex : register(u18, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 dispatchThreadID : SV_DispatchThreadID)
{
    float4 result;

    bool history_ok = history_valid != 0u;
    if (history_ok && !(frame_index == 0u))
    {
        float4 current = input_tex.Load(uint3(dispatchThreadID.xy, 0u));
        float2 history_uv = pixel_to_uv(int2(dispatchThreadID.xy), float2(input_tex_dims)) + reprojection_tex.Load(uint3(dispatchThreadID.xy, 0u)).xy;

        float4 history;
        if (history_ok && !(frame_index == 0u))
        {
            history = history_tex.SampleLevel(sampler_lnc, history_uv, 0.0f);
        }
        else
        {
            history = current;
        }

        
        float4 c00 = input_tex.Load(uint3(dispatchThreadID.xy + uint2(4294967294u, 4294967294u), 0u));
        float4 c10 = input_tex.Load(uint3(dispatchThreadID.xy + uint2(0u, 4294967294u), 0u));
        float4 c20 = input_tex.Load(uint3(dispatchThreadID.xy + uint2(2u, 4294967294u), 0u));
        float4 c01 = input_tex.Load(uint3(dispatchThreadID.xy + uint2(4294967294u, 0u), 0u));
        float4 c11 = input_tex.Load(uint3(dispatchThreadID.xy, 0u));
        float4 c21 = input_tex.Load(uint3(dispatchThreadID.xy + uint2(2u, 0u), 0u));
        float4 c02 = input_tex.Load(uint3(dispatchThreadID.xy + uint2(4294967294u, 2u), 0u));
        float4 c12 = input_tex.Load(uint3(dispatchThreadID.xy + uint2(0u, 2u), 0u));
        float4 c22 = input_tex.Load(uint3(dispatchThreadID.xy + uint2(2u, 2u), 0u));

        float4 sum = c00 * 0.223130166530609130859375f;
        sum += c10 * 0.4723665416240692138671875f;
        sum += c20 * 0.223130166530609130859375f;
        sum += c01 * 0.4723665416240692138671875f;
        sum += c11 * 1.0f;
        sum += c21 * 0.4723665416240692138671875f;
        sum += c02 * 0.223130166530609130859375f;
        sum += c12 * 0.4723665416240692138671875f;
        sum += c22 * 0.223130166530609130859375f;

        float4 sumsq = (c00 * c00) * 0.223130166530609130859375f;
        sumsq += (c10 * c10) * 0.4723665416240692138671875f;
        sumsq += (c20 * c20) * 0.223130166530609130859375f;
        sumsq += (c01 * c01) * 0.4723665416240692138671875f;
        sumsq += (c11 * c11) * 1.0f;
        sumsq += (c21 * c21) * 0.4723665416240692138671875f;
        sumsq += (c02 * c02) * 0.223130166530609130859375f;
        sumsq += (c12 * c12) * 0.4723665416240692138671875f;
        sumsq += (c22 * c22) * 0.223130166530609130859375f;

        float4 blur = sum * 0.264411270618438720703125f.xxxx;
        float4 variance = (sumsq * 0.264411270618438720703125f.xxxx) - (blur * blur);
        float4 stddev = sqrt(max(0.0f.xxxx, variance)) * 0.5f * 5.0f;
        float4 blended = lerp(current, blur, 0.25f.xxxx);

        result = lerp(clamp(history, blended - stddev, blended + stddev), current, 0.125f.xxxx);
    }
    else
    {
        result = input_tex.Load(uint3(dispatchThreadID.xy, 0u));
    }

    history_output_tex[dispatchThreadID.xy] = result.x;
    final_output_tex[dispatchThreadID.xy] = result.x;
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

cbuffer $Globals : register(b13, space0)
{
    uint _Globals_history_valid : packoffset(c0);
    uint2 _Globals_input_tex_dims : packoffset(c0.y);
    uint2 _Globals_history_tex_dims : packoffset(c1);
    uint2 _Globals_reprojection_tex_dims : packoffset(c1.z);
    uint2 _Globals_final_output_tex_dims : packoffset(c2);
    uint2 _Globals_history_output_tex_dims : packoffset(c2.z);
};

SamplerState sampler_lnc : register(s0, space0);
Texture2D<float4> input_tex : register(t14, space0);
Texture2D<float4> history_tex : register(t15, space0);
Texture2D<float4> reprojection_tex : register(t16, space0);
RWTexture2D<unorm float> final_output_tex : register(u17, space0);
RWTexture2D<unorm float> history_output_tex : register(u18, space0);

static uint3 gl_GlobalInvocationID;
struct SPIRV_Cross_Input
{
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
};

void comp_main()
{
    bool _87 = _Globals_history_valid != 0u;
    bool _94;
    if (_87)
    {
        _94 = !(0u == frame_constants_frame_index);
    }
    else
    {
        _94 = false;
    }
    float4 _201;
    if (_94)
    {
        float4 _106 = input_tex.Load(int3(gl_GlobalInvocationID.xy, 0u));
        bool _115;
        if (_87)
        {
            _115 = !(0u == frame_constants_frame_index);
        }
        else
        {
            _115 = false;
        }
        float4 _125;
        if (_115)
        {
            _125 = history_tex.SampleLevel(sampler_lnc, ((float2(int2(gl_GlobalInvocationID.xy)) + 0.5f.xx) / float2(_Globals_input_tex_dims)) + reprojection_tex.Load(int3(gl_GlobalInvocationID.xy, 0u)).xy, 0.0f);
        }
        else
        {
            _125 = _106;
        }
        float4 _127 = input_tex.Load(int3(gl_GlobalInvocationID.xy + uint2(4294967294u, 4294967294u), 0u));
        float4 _132 = input_tex.Load(int3(gl_GlobalInvocationID.xy + uint2(0u, 4294967294u), 0u));
        float4 _139 = input_tex.Load(int3(gl_GlobalInvocationID.xy + uint2(2u, 4294967294u), 0u));
        float4 _146 = input_tex.Load(int3(gl_GlobalInvocationID.xy + uint2(4294967294u, 0u), 0u));
        float4 _152 = input_tex.Load(int3(gl_GlobalInvocationID.xy, 0u));
        float4 _159 = input_tex.Load(int3(gl_GlobalInvocationID.xy + uint2(2u, 0u), 0u));
        float4 _166 = input_tex.Load(int3(gl_GlobalInvocationID.xy + uint2(4294967294u, 2u), 0u));
        float4 _173 = input_tex.Load(int3(gl_GlobalInvocationID.xy + uint2(0u, 2u), 0u));
        float4 _180 = input_tex.Load(int3(gl_GlobalInvocationID.xy + uint2(2u, 2u), 0u));
        float4 _186 = (((((((((_127 * 0.223130166530609130859375f) + (_132 * 0.4723665416240692138671875f)) + (_139 * 0.223130166530609130859375f)) + (_146 * 0.4723665416240692138671875f)) + (_152 * 1.0f)) + (_159 * 0.4723665416240692138671875f)) + (_166 * 0.223130166530609130859375f)) + (_173 * 0.4723665416240692138671875f)) + (_180 * 0.223130166530609130859375f)) * 0.264411270618438720703125f.xxxx;
        float4 _192 = lerp(_106, _186, 0.25f.xxxx);
        float4 _194 = (sqrt(max(0.0f.xxxx, (((((((((((_127 * _127) * 0.223130166530609130859375f) + ((_132 * _132) * 0.4723665416240692138671875f)) + ((_139 * _139) * 0.223130166530609130859375f)) + ((_146 * _146) * 0.4723665416240692138671875f)) + ((_152 * _152) * 1.0f)) + ((_159 * _159) * 0.4723665416240692138671875f)) + ((_166 * _166) * 0.223130166530609130859375f)) + ((_173 * _173) * 0.4723665416240692138671875f)) + ((_180 * _180) * 0.223130166530609130859375f)) * 0.264411270618438720703125f.xxxx) - (_186 * _186))) * 0.5f) * 5.0f;
        _201 = lerp(clamp(_125, _192 - _194, _192 + _194), _106, 0.125f.xxxx);
    }
    else
    {
        _201 = input_tex.Load(int3(gl_GlobalInvocationID.xy, 0u));
    }
    float4 _203 = _201.x.xxxx;
    history_output_tex[gl_GlobalInvocationID.xy] = _203.x;
    final_output_tex[gl_GlobalInvocationID.xy] = _203.x;
}

[numthreads(8, 8, 1)]
void cs_main(SPIRV_Cross_Input stage_input)
{
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    comp_main();
}
#pragma dxc diagnostic pop
#endif
