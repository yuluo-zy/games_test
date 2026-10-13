#define SHADER_VARIANT_DEPTH_ONLY 0
#define TGV_b903e43ffb3da915 1
#line 1 "shaders/rain_particles.raster.hlsl"


#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)))







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
#line 11 "shaders/rain_particles.raster.hlsl"
          
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
#line 12 "shaders/rain_particles.raster.hlsl"

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
#line 13 "shaders/rain_particles.raster.hlsl"

#line 1 "shaders/inc/consts.hlsl"






#ifndef TINYGLADE_INC_CONSTS_HLSL
#define TINYGLADE_INC_CONSTS_HLSL


static const float RENDER_SCALE = 1.0f;

#endif 
#line 14 "shaders/rain_particles.raster.hlsl"

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
#line 15 "shaders/rain_particles.raster.hlsl"

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
#line 16 "shaders/rain_particles.raster.hlsl"

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
#line 17 "shaders/rain_particles.raster.hlsl"
    
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
#line 18 "shaders/rain_particles.raster.hlsl"






float4x4 model_to_world;
float rain_strength;

struct VsIn
{
    [[vk::location(0)]] float3 Vertex_Position: Vertex_Position;
    [[vk::location(1)]] int id: id;
};

struct VsOut
{
    float3 C0 : TEXCOORD0;
    float C1 : TEXCOORD1;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;

    float fall = lerp(0.6f, 1.2f, rand01(hash(uint(vin.id ^ 5678))));
    float3 offset = float3(
        lerp(0.0f, 60.0f, rand01(hash(uint(vin.id ^ 45845)))),
        lerp(0.0f, 1.0f, rand01(hash(uint(vin.id ^ 35934)))),
        lerp(0.0f, 60.0f, rand01(hash(uint(vin.id ^ 23475)))));

    offset.y = frac(offset.y - ((float(game_time_sec) + game_time_frac) * 0.5f) * fall) * 50.0f;

    float3 world = float3(0.0f, 1.0f, 0.0f) * vin.Vertex_Position.y
                 + normalize(mul(view_constants[current_view_index].view_to_world, float4(1.0f, 0.0f, 0.0f, 0.0f)).xyz)
                   * vin.Vertex_Position.x * (0.04f * rain_strength)
                 + offset + float3(-30.0f, 0.0f, -30.0f);

    o.position = mul(view_constants[current_view_index].world_to_sample, float4(world, 1.0f));
    o.C0 = float3(1.0f, 1.0f, 1.0f);
    o.C1 = lerp(0.3f, 0.0f, vin.Vertex_Position.y + 0.5f) * pow(rain_strength, 0.5f);
    return o;
}

struct PsIn
{
    float3 C0 : TEXCOORD0;
    float C1 : TEXCOORD1;
};

struct PsOut
{
    float4 color : SV_Target0;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;
    o.color = float4(pin.C0 * pin.C1, pin.C1);
    return o;
}

#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)

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
    uint _Globals_current_view_index : packoffset(c0);
    column_major float4x4 _Globals_model_to_world : packoffset(c1);
    float _Globals_rain_strength : packoffset(c5);
};


static float4 gl_Position;
static float3 in_var_Vertex_Position;
static int in_var_id;
static float3 out_var_C0;
static float out_var_C1;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] int in_var_id: id;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float out_var_C1 : TEXCOORD1;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    uint _84 = uint(in_var_id ^ 5678);
    uint _86 = _84 + (_84 << 10u);
    uint _88 = _86 ^ (_86 >> 6u);
    uint _90 = _88 + (_88 << 3u);
    uint _92 = _90 ^ (_90 >> 11u);
    uint _104 = uint(in_var_id ^ 23475);
    uint _106 = _104 + (_104 << 10u);
    uint _108 = _106 ^ (_106 >> 6u);
    uint _110 = _108 + (_108 << 3u);
    uint _112 = _110 ^ (_110 >> 11u);
    uint _121 = uint(in_var_id ^ 35934);
    uint _123 = _121 + (_121 << 10u);
    uint _125 = _123 ^ (_123 >> 6u);
    uint _127 = _125 + (_125 << 3u);
    uint _129 = _127 ^ (_127 >> 11u);
    float _136 = lerp(0.0f, 1.0f, asfloat(((_129 + (_129 << 15u)) & 8388607u) | 1065353216u) - 1.0f);
    uint _138 = uint(in_var_id ^ 45845);
    uint _140 = _138 + (_138 << 10u);
    uint _142 = _140 ^ (_140 >> 6u);
    uint _144 = _142 + (_142 << 3u);
    uint _146 = _144 ^ (_144 >> 11u);
    float3 _171 = float3(lerp(0.0f, 60.0f, asfloat(((_112 + (_112 << 15u)) & 8388607u) | 1065353216u) - 1.0f), _136, lerp(0.0f, 60.0f, asfloat(((_146 + (_146 << 15u)) & 8388607u) | 1065353216u) - 1.0f));
    _171.y = frac(_136 - (((float(frame_constants_game_time_sec) + frame_constants_game_time_frac) * 0.5f) * lerp(0.60000002384185791015625f, 1.2000000476837158203125f, asfloat(((_92 + (_92 << 15u)) & 8388607u) | 1065353216u) - 1.0f))) * 50.0f;
    gl_Position = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, float4((((float3(0.0f, 1.0f, 0.0f) * in_var_Vertex_Position.y) * 1.0f) + ((normalize(mul(frame_constants_view_constants[_Globals_current_view_index].view_to_world, float4(1.0f, 0.0f, 0.0f, 0.0f)).xyz) * in_var_Vertex_Position.x) * (0.039999999105930328369140625f * _Globals_rain_strength))) + (_171 + float3(-30.0f, 0.0f, -30.0f)), 1.0f));
    out_var_C0 = 1.0f.xxx;
    out_var_C1 = lerp(0.300000011920928955078125f, 0.0f, in_var_Vertex_Position.y + 0.5f) * pow(_Globals_rain_strength, 0.5f);
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_id = stage_input.in_var_id;
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
    uint _Globals_current_view_index : packoffset(c0);
    column_major float4x4 _Globals_model_to_world : packoffset(c1);
    float _Globals_rain_strength : packoffset(c5);
};


static float4 gl_Position;
static float3 in_var_Vertex_Position;
static int in_var_id;
static float3 out_var_C0;
static float out_var_C1;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] int in_var_id: id;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float out_var_C1 : TEXCOORD1;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    uint _84 = uint(in_var_id ^ 5678);
    uint _86 = _84 + (_84 << 10u);
    uint _88 = _86 ^ (_86 >> 6u);
    uint _90 = _88 + (_88 << 3u);
    uint _92 = _90 ^ (_90 >> 11u);
    uint _104 = uint(in_var_id ^ 23475);
    uint _106 = _104 + (_104 << 10u);
    uint _108 = _106 ^ (_106 >> 6u);
    uint _110 = _108 + (_108 << 3u);
    uint _112 = _110 ^ (_110 >> 11u);
    uint _121 = uint(in_var_id ^ 35934);
    uint _123 = _121 + (_121 << 10u);
    uint _125 = _123 ^ (_123 >> 6u);
    uint _127 = _125 + (_125 << 3u);
    uint _129 = _127 ^ (_127 >> 11u);
    float _136 = lerp(0.0f, 1.0f, asfloat(((_129 + (_129 << 15u)) & 8388607u) | 1065353216u) - 1.0f);
    uint _138 = uint(in_var_id ^ 45845);
    uint _140 = _138 + (_138 << 10u);
    uint _142 = _140 ^ (_140 >> 6u);
    uint _144 = _142 + (_142 << 3u);
    uint _146 = _144 ^ (_144 >> 11u);
    float3 _171 = float3(lerp(0.0f, 60.0f, asfloat(((_112 + (_112 << 15u)) & 8388607u) | 1065353216u) - 1.0f), _136, lerp(0.0f, 60.0f, asfloat(((_146 + (_146 << 15u)) & 8388607u) | 1065353216u) - 1.0f));
    _171.y = frac(_136 - (((float(frame_constants_game_time_sec) + frame_constants_game_time_frac) * 0.5f) * lerp(0.60000002384185791015625f, 1.2000000476837158203125f, asfloat(((_92 + (_92 << 15u)) & 8388607u) | 1065353216u) - 1.0f))) * 50.0f;
    gl_Position = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, float4((((float3(0.0f, 1.0f, 0.0f) * in_var_Vertex_Position.y) * 1.0f) + ((normalize(mul(frame_constants_view_constants[_Globals_current_view_index].view_to_world, float4(1.0f, 0.0f, 0.0f, 0.0f)).xyz) * in_var_Vertex_Position.x) * (0.039999999105930328369140625f * _Globals_rain_strength))) + (_171 + float3(-30.0f, 0.0f, -30.0f)), 1.0f));
    out_var_C0 = 1.0f.xxx;
    out_var_C1 = lerp(0.300000011920928955078125f, 0.0f, in_var_Vertex_Position.y + 0.5f) * pow(_Globals_rain_strength, 0.5f);
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_id = stage_input.in_var_id;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif
