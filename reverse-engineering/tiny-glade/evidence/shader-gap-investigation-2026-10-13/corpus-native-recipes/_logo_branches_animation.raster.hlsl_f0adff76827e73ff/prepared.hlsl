#define SHADER_VARIANT_DEPTH_ONLY 1
#define TGV_f0adff76827e73ff 1
#line 1 "shaders/logo_branches_animation.raster.hlsl"


#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))




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
#line 8 "shaders/logo_branches_animation.raster.hlsl"
            
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
#line 9 "shaders/logo_branches_animation.raster.hlsl"

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
#line 10 "shaders/logo_branches_animation.raster.hlsl"

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
#line 11 "shaders/logo_branches_animation.raster.hlsl"


cbuffer $Globals : register(b13, space0)
{
    float4x4 model_to_world;
    float anim_t;
    float fade_out;
    uint2 branch_t_dims;
    uint2 branch_y_dims;
    uint2 branch_g_dims;
    uint2 branch_e_dims;
    uint2 framebuffer_dims;
};

Texture2D<float4> branch_t : register(t14, space0);
Texture2D<float4> branch_y : register(t15, space0);
Texture2D<float4> branch_g : register(t16, space0);
Texture2D<float4> branch_e : register(t17, space0);

struct VsIn
{
    [[vk::location(0)]] float3 Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 start_pos: start_pos;
    [[vk::location(2)]] float2 Vertex_UV: Vertex_UV;
    [[vk::location(3)]] int branch_idx: branch_idx;
};

struct VsOut
{
    float2 uv : TEXCOORD0;
    nointerpolation int branch_idx : TEXCOORD1;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;

    float angle;
    float len;
    if (vin.branch_idx == 0) { angle = radians(-10.0f); len = 0.2f; }
    else if (vin.branch_idx == 1) { angle = radians(-30.0f); len = 0.1f; }
    else if (vin.branch_idx == 2) { angle = radians(-30.0f); len = 0.18f; }
    else if (vin.branch_idx == 3) { angle = radians(20.0f); len = 0.22f; }
    else { angle = 0.0f; len = 0.0f; }

    float t = clamp(1.0f - (anim_t * 1.22f - len), 0.0f, 1.0f);
    float grow = 1.0f - ((t * t * t) - t * sin(t * 3.1415927f));

    float3 offset = lerp(float3(0.0f, 0.0f, 0.0f), vin.Vertex_Position - vin.start_pos, grow);
    float rot = lerp(angle, 0.0f, grow);
    float s = sin(rot);
    float c = cos(rot);
    float2x2 m = float2x2(c, s, -s, c);
    float3 world = float3(mul(m, offset.xy), offset.z) + vin.start_pos;

    float2 pos = world.xy;
    pos.x = world.x * (float(framebuffer_dims.y) / float(framebuffer_dims.x));
    o.position = float4(pos * 0.45f, 1.0f, 1.0f);
    o.uv = vin.Vertex_UV;
    o.branch_idx = vin.branch_idx;
    return o;
}

float4 ps_main(VsOut vin) : SV_Target0
{
    float4 s;
    if (vin.branch_idx == 0) s = branch_t.SampleBias(sampler_llr, vin.uv, -0.5f);
    else if (vin.branch_idx == 1) s = branch_y.SampleBias(sampler_llr, vin.uv, -0.5f);
    else if (vin.branch_idx == 2) s = branch_g.SampleBias(sampler_llr, vin.uv, -0.5f);
    else if (vin.branch_idx == 3) s = branch_e.SampleBias(sampler_llr, vin.uv, -0.5f);
    else s = float4(0.0f, 0.0f, 0.0f, 0.0f);

    float alpha = s.w * smoothstep(1.0f, 0.0f, fade_out);
    return float4(s.rgb * alpha, alpha);
}

#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)

#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
cbuffer $Globals : register(b13, space0)
{
    column_major float4x4 _Globals_model_to_world : packoffset(c0);
    float _Globals_anim_t : packoffset(c4);
    float _Globals_fade_out : packoffset(c4.y);
    uint2 _Globals_branch_t_dims : packoffset(c4.z);
    uint2 _Globals_branch_y_dims : packoffset(c5);
    uint2 _Globals_branch_g_dims : packoffset(c5.z);
    uint2 _Globals_branch_e_dims : packoffset(c6);
    uint2 _Globals_framebuffer_dims : packoffset(c6.z);
};


static float4 gl_Position;
static float3 in_var_Vertex_Position;
static float3 in_var_start_pos;
static float2 in_var_Vertex_UV;
static int in_var_branch_idx;
static float2 out_var_C0;
static int out_var_C1;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 in_var_start_pos: start_pos;
    [[vk::location(2)]] float2 in_var_Vertex_UV: Vertex_UV;
    [[vk::location(3)]] int in_var_branch_idx: branch_idx;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float2 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] nointerpolation int out_var_C1 : TEXCOORD1;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    float _91;
    float _92;
    if (in_var_branch_idx == 0)
    {
        _91 = radians(-10.0f);
        _92 = 0.20000000298023223876953125f;
    }
    else
    {
        float _89;
        float _90;
        if (in_var_branch_idx == 1)
        {
            _89 = radians(-30.0f);
            _90 = 0.100000001490116119384765625f;
        }
        else
        {
            float _87;
            float _88;
            if (in_var_branch_idx == 2)
            {
                _87 = radians(-30.0f);
                _88 = 0.180000007152557373046875f;
            }
            else
            {
                bool _81 = in_var_branch_idx == 3;
                float _85;
                if (_81)
                {
                    _85 = radians(20.0f);
                }
                else
                {
                    _85 = 0.0f;
                }
                _87 = _85;
                _88 = _81 ? 0.2199999988079071044921875f : 0.0f;
            }
            _89 = _87;
            _90 = _88;
        }
        _91 = _89;
        _92 = _90;
    }
    float _99 = clamp(1.0f - ((_Globals_anim_t * 1.2200000286102294921875f) - _92), 0.0f, 1.0f);
    float _106 = 1.0f - (((_99 * _99) * _99) - (_99 * sin(_99 * 3.1400001049041748046875f)));
    float3 _108 = lerp(0.0f.xxx, in_var_Vertex_Position - in_var_start_pos, _106.xxx);
    float _110 = lerp(_91, 0.0f, _106);
    float _111 = sin(_110);
    float _112 = cos(_110);
    float2 _117 = mul(float2x2(float2(_112, _111), float2(-_111, _112)), _108.xy);
    float3 _119 = float3(_117.x, _117.y, _108.z) + in_var_start_pos;
    float2 _120 = _119.xy;
    _120.x = _119.x * (float(_Globals_framebuffer_dims.y) / float(_Globals_framebuffer_dims.x));
    gl_Position = float4(_120 * 0.449999988079071044921875f, 1.0f, 1.0f);
    out_var_C0 = in_var_Vertex_UV;
    out_var_C1 = in_var_branch_idx;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_start_pos = stage_input.in_var_start_pos;
    in_var_Vertex_UV = stage_input.in_var_Vertex_UV;
    in_var_branch_idx = stage_input.in_var_branch_idx;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)

#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
cbuffer $Globals : register(b13, space0)
{
    column_major float4x4 _Globals_model_to_world : packoffset(c0);
    float _Globals_anim_t : packoffset(c4);
    float _Globals_fade_out : packoffset(c4.y);
    uint2 _Globals_branch_t_dims : packoffset(c4.z);
    uint2 _Globals_branch_y_dims : packoffset(c5);
    uint2 _Globals_branch_g_dims : packoffset(c5.z);
    uint2 _Globals_branch_e_dims : packoffset(c6);
    uint2 _Globals_framebuffer_dims : packoffset(c6.z);
};


static float4 gl_Position;
static float3 in_var_Vertex_Position;
static float3 in_var_start_pos;
static float2 in_var_Vertex_UV;
static int in_var_branch_idx;
static float2 out_var_C0;
static int out_var_C1;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 in_var_start_pos: start_pos;
    [[vk::location(2)]] float2 in_var_Vertex_UV: Vertex_UV;
    [[vk::location(3)]] int in_var_branch_idx: branch_idx;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float2 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] nointerpolation int out_var_C1 : TEXCOORD1;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    float _91;
    float _92;
    if (in_var_branch_idx == 0)
    {
        _91 = radians(-10.0f);
        _92 = 0.20000000298023223876953125f;
    }
    else
    {
        float _89;
        float _90;
        if (in_var_branch_idx == 1)
        {
            _89 = radians(-30.0f);
            _90 = 0.100000001490116119384765625f;
        }
        else
        {
            float _87;
            float _88;
            if (in_var_branch_idx == 2)
            {
                _87 = radians(-30.0f);
                _88 = 0.180000007152557373046875f;
            }
            else
            {
                bool _81 = in_var_branch_idx == 3;
                float _85;
                if (_81)
                {
                    _85 = radians(20.0f);
                }
                else
                {
                    _85 = 0.0f;
                }
                _87 = _85;
                _88 = _81 ? 0.2199999988079071044921875f : 0.0f;
            }
            _89 = _87;
            _90 = _88;
        }
        _91 = _89;
        _92 = _90;
    }
    float _99 = clamp(1.0f - ((_Globals_anim_t * 1.2200000286102294921875f) - _92), 0.0f, 1.0f);
    float _106 = 1.0f - (((_99 * _99) * _99) - (_99 * sin(_99 * 3.1400001049041748046875f)));
    float3 _108 = lerp(0.0f.xxx, in_var_Vertex_Position - in_var_start_pos, _106.xxx);
    float _110 = lerp(_91, 0.0f, _106);
    float _111 = sin(_110);
    float _112 = cos(_110);
    float2 _117 = mul(float2x2(float2(_112, _111), float2(-_111, _112)), _108.xy);
    float3 _119 = float3(_117.x, _117.y, _108.z) + in_var_start_pos;
    float2 _120 = _119.xy;
    _120.x = _119.x * (float(_Globals_framebuffer_dims.y) / float(_Globals_framebuffer_dims.x));
    gl_Position = float4(_120 * 0.449999988079071044921875f, 1.0f, 1.0f);
    out_var_C0 = in_var_Vertex_UV;
    out_var_C1 = in_var_branch_idx;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_start_pos = stage_input.in_var_start_pos;
    in_var_Vertex_UV = stage_input.in_var_Vertex_UV;
    in_var_branch_idx = stage_input.in_var_branch_idx;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)

#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
void frag_main()
{
}

void ps_main()
{
    frag_main();
}
#pragma dxc diagnostic pop
#endif
