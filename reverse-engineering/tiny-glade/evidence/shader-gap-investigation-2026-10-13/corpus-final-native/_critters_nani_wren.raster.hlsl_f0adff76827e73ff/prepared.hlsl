#define SHADER_VARIANT_DEPTH_ONLY 1
#define TGV_f0adff76827e73ff 1
#line 1 "shaders/critters/nani_wren.raster.hlsl"


#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))


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
#line 6 "shaders/critters/nani_wren.raster.hlsl"

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
#line 7 "shaders/critters/nani_wren.raster.hlsl"

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
#line 8 "shaders/critters/nani_wren.raster.hlsl"

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
#line 9 "shaders/critters/nani_wren.raster.hlsl"

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
#line 10 "shaders/critters/nani_wren.raster.hlsl"







struct InstanceData
{
    Affine3Packed xform;      
    Affine3Packed prev_xform; 
    float flap_t;             
    float wing_fold_t;        
    int id;                   
    uint pad0;                
};

StructuredBuffer<uint> instance_indirection_buffer : register(t14);
StructuredBuffer<InstanceData> nani_instance_data : register(t15);
Texture2D<float4> bird_texture : register(t16);

uint2 bird_texture_dims;
int bool_render_shadows;

struct VsIn
{
    [[vk::location(0)]] float3 Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 Vertex_Color: Vertex_Color;
    [[vk::location(2)]] float3 Vertex_Normal: Vertex_Normal;
    [[vk::location(3)]] float2 Vertex_UV: Vertex_UV;
    [[vk::location(4)]] float3 rest_pos: rest_pos;
    [[vk::location(5)]] float3 rest_n: rest_n;
    [[vk::location(7)]] float wing_t: wing_t;
    uint instance_id : SV_InstanceID;
};

struct VsOut
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float2 C3 : TEXCOORD3;
    float3 C4 : TEXCOORD4;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;

    uint inst = instance_indirection_buffer[vin.instance_id];
    Affine3Packed xform = nani_instance_data[inst].xform;
    Affine3Packed prev_xform = nani_instance_data[inst].prev_xform;
    float flap_t = nani_instance_data[inst].flap_t;
    float wing_fold_t = nani_instance_data[inst].wing_fold_t;
    int id = nani_instance_data[inst].id;

    float3 _155 = vin.Vertex_Position * 0.4000000059604644775390625f;
    _155.y = _155.y + ((vin.wing_t * 0.4000000059604644775390625f) * sin((flap_t * 6.283185482025146484375f) - (vin.wing_t * 0.31415927410125732421875f)));
    float3 _166 = wing_fold_t.xxx;

    float3x4 _174 = transpose(float4x3(
        float3(xform.data[0], xform.data[1], xform.data[2]),
        float3(xform.data[3], xform.data[4], xform.data[5]),
        float3(xform.data[6], xform.data[7], xform.data[8]),
        float3(xform.data[9], xform.data[10], xform.data[11])));
    float4 _178 = float4(lerp(_155, vin.rest_pos * 0.4000000059604644775390625f, _166), 1.0f);
    float3 _179 = mul(_174, _178);

    float2 _180 = vin.Vertex_UV * 0.5f;
    float _181 = _180.y;
    float2 _183 = _180;
    _183.y = _181 + 0.5f;
    int _184 = id - 8 * (id / 8);
    float2 _189;
    if (_184 == 3)
    {
        float2 _188 = _183;
        _188.y = _181;
        _189 = _188;
    }
    else
    {
        _189 = _183;
    }
    float2 _196;
    if (_184 == 4)
    {
        float2 _195 = _189;
        _195.x = _189.x - 0.5f;
        _196 = _195;
    }
    else
    {
        _196 = _189;
    }
    float2 _201;
    if (_184 == 5)
    {
        _201 = _196 - 0.5f.xx;
    }
    else
    {
        _201 = _196;
    }

    o.position = mul(view_constants[current_view_index].world_to_sample, float4(_179, 1.0f));
    o.C0 = vin.Vertex_Color;
    o.C1 = mul(_174, float4(lerp(vin.Vertex_Normal, vin.rest_n, _166), 0.0f));
    o.C2 = _179;
    o.C3 = _201;
    o.C4 = mul(transpose(float4x3(
        float3(prev_xform.data[0], prev_xform.data[1], prev_xform.data[2]),
        float3(prev_xform.data[3], prev_xform.data[4], prev_xform.data[5]),
        float3(prev_xform.data[6], prev_xform.data[7], prev_xform.data[8]),
        float3(prev_xform.data[9], prev_xform.data[10], prev_xform.data[11]))), _178);
    return o;
}

struct PsIn
{
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float2 C3 : TEXCOORD3;
    float3 C4 : TEXCOORD4;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
    float3 velocity : SV_Target1;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;
    float3 packed_normal = (normalize(pin.C1) * 0.5f) + 0.5f;
    float3 _111 = sqrt(clamp(bird_texture.SampleBias(sampler_mipmap, pin.C3, global_mip_bias).xyz, 0.0f, 1.0f));
    uint albedo_word = (uint((clamp(_111.x, 0.0f, 1.0f) * 255.0f) + 0.5f)
                      + (uint((clamp(_111.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)
                      + (uint((clamp(_111.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u));
    uint2 gbuf = uint2(((uint((clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f) + 0.5f)
                      + (uint((clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u))
                      + (uint((clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 3422552064u, albedo_word);
    gbuf.y = albedo_word | 117440512u;
    o.gbuffer = gbuf;
    o.velocity = pin.C2 - pin.C4;
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

struct Affine3Packed
{
    float data[12];
};

struct InstanceData
{
    Affine3Packed xform;
    Affine3Packed prev_xform;
    float flap_t;
    float wing_fold_t;
    int id;
    uint pad0;
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
    uint current_view_index : packoffset(c0);
    uint2 bird_texture_dims : packoffset(c0.y);
    int bool_render_shadows : packoffset(c0.w);
};

ByteAddressBuffer instance_indirection_buffer : register(t14, space0);
ByteAddressBuffer nani_instance_data : register(t15, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Color;
static float3 in_var_Vertex_Normal;
static float2 in_var_Vertex_UV;
static float3 in_var_rest_pos;
static float3 in_var_rest_n;
static float in_var_wing_t;
static float3 out_var_C0;
static float3 out_var_C1;
static float3 out_var_C2;
static float2 out_var_C3;
static float3 out_var_C4;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 in_var_Vertex_Color: Vertex_Color;
    [[vk::location(2)]] float3 in_var_Vertex_Normal: Vertex_Normal;
    [[vk::location(3)]] float2 in_var_Vertex_UV: Vertex_UV;
    [[vk::location(4)]] float3 in_var_rest_pos: rest_pos;
    [[vk::location(5)]] float3 in_var_rest_n: rest_n;
    [[vk::location(7)]] float in_var_wing_t: wing_t;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float2 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float3 out_var_C4 : TEXCOORD4;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    Affine3Packed _120;
    [unroll]
    for (int _2ident = 0; _2ident < 12; _2ident++)
    {
        _120.data[_2ident] = nani_instance_data.Load<float>(_2ident * 4 + instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 0);
    }
    Affine3Packed _122;
    [unroll]
    for (int _3ident = 0; _3ident < 12; _3ident++)
    {
        _122.data[_3ident] = nani_instance_data.Load<float>(_3ident * 4 + instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 48);
    }
    float3 _155 = in_var_Vertex_Position * 0.4000000059604644775390625f;
    _155.y = _155.y + ((in_var_wing_t * 0.4000000059604644775390625f) * sin((nani_instance_data.Load<float>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 96) * 6.283185482025146484375f) - (in_var_wing_t * 0.31415927410125732421875f)));
    float3 _166 = nani_instance_data.Load<float>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 100).xxx;
    float3x4 _174 = transpose(float4x3(float3(_120.data[0], _120.data[1], _120.data[2]), float3(_120.data[3], _120.data[4], _120.data[5]), float3(_120.data[6], _120.data[7], _120.data[8]), float3(_120.data[9], _120.data[10], _120.data[11])));
    float4 _178 = float4(lerp(_155, in_var_rest_pos * 0.4000000059604644775390625f, _166), 1.0f);
    float3 _179 = mul(_174, _178);
    float2 _180 = in_var_Vertex_UV * 0.5f;
    float _181 = _180.y;
    float2 _183 = _180;
    _183.y = _181 + 0.5f;
    int _184 = nani_instance_data.Load<int>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 104) - 8 * (nani_instance_data.Load<int>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 104) / 8);
    float2 _189;
    if (_184 == 3)
    {
        float2 _188 = _183;
        _188.y = _181;
        _189 = _188;
    }
    else
    {
        _189 = _183;
    }
    float2 _196;
    if (_184 == 4)
    {
        float2 _195 = _189;
        _195.x = _189.x - 0.5f;
        _196 = _195;
    }
    else
    {
        _196 = _189;
    }
    float2 _201;
    if (_184 == 5)
    {
        _201 = _196 - 0.5f.xx;
    }
    else
    {
        _201 = _196;
    }
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(_179, 1.0f));
    out_var_C0 = in_var_Vertex_Color;
    out_var_C1 = mul(_174, float4(lerp(in_var_Vertex_Normal, in_var_rest_n, _166), 0.0f));
    out_var_C2 = _179;
    out_var_C3 = _201;
    out_var_C4 = mul(transpose(float4x3(float3(_122.data[0], _122.data[1], _122.data[2]), float3(_122.data[3], _122.data[4], _122.data[5]), float3(_122.data[6], _122.data[7], _122.data[8]), float3(_122.data[9], _122.data[10], _122.data[11]))), _178);
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Color = stage_input.in_var_Vertex_Color;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_Vertex_UV = stage_input.in_var_Vertex_UV;
    in_var_rest_pos = stage_input.in_var_rest_pos;
    in_var_rest_n = stage_input.in_var_rest_n;
    in_var_wing_t = stage_input.in_var_wing_t;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    stage_output.out_var_C4 = out_var_C4;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)

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

SamplerState sampler_mipmap : register(s10, space0);
Texture2D<float4> bird_texture : register(t16, space0);

static float3 in_var_C1;
static float3 in_var_C2;
static float2 in_var_C3;
static float3 in_var_C4;
static uint2 out_var_SV_TARGET0;
static float3 out_var_SV_TARGET1;

struct SPIRV_Cross_Input
{
    [[vk::location(1)]] float3 in_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 in_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float2 in_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float3 in_var_C4 : TEXCOORD4;
};

struct SPIRV_Cross_Output
{
    uint2 out_var_SV_TARGET0 : SV_Target0;
    float3 out_var_SV_TARGET1 : SV_Target1;
};

void frag_main()
{
    float3 _89 = (normalize(in_var_C1) * 0.5f) + 0.5f.xxx;
    float3 _111 = sqrt(clamp(bird_texture.SampleBias(sampler_mipmap, in_var_C3, frame_constants_global_mip_bias).xyz, 0.0f.xxx, 1.0f.xxx));
    uint _130 = (uint((clamp(_111.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_111.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_111.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u);
    uint2 _131 = uint2(((uint((clamp(_89.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_89.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_89.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 3422552064u, _130);
    _131.y = _130 | 117440512u;
    out_var_SV_TARGET0 = _131;
    out_var_SV_TARGET1 = in_var_C2 - in_var_C4;
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C1 = stage_input.in_var_C1;
    in_var_C2 = stage_input.in_var_C2;
    in_var_C3 = stage_input.in_var_C3;
    in_var_C4 = stage_input.in_var_C4;
    frag_main();
    SPIRV_Cross_Output stage_output;
    stage_output.out_var_SV_TARGET0 = out_var_SV_TARGET0;
    stage_output.out_var_SV_TARGET1 = out_var_SV_TARGET1;
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

struct Affine3Packed
{
    float data[12];
};

struct InstanceData
{
    Affine3Packed xform;
    Affine3Packed prev_xform;
    float flap_t;
    float wing_fold_t;
    int id;
    uint pad0;
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
    uint current_view_index : packoffset(c0);
    uint2 bird_texture_dims : packoffset(c0.y);
    int bool_render_shadows : packoffset(c0.w);
};

ByteAddressBuffer instance_indirection_buffer : register(t14, space0);
ByteAddressBuffer nani_instance_data : register(t15, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Color;
static float3 in_var_Vertex_Normal;
static float2 in_var_Vertex_UV;
static float3 in_var_rest_pos;
static float3 in_var_rest_n;
static float in_var_wing_t;
static float3 out_var_C0;
static float3 out_var_C1;
static float3 out_var_C2;
static float2 out_var_C3;
static float3 out_var_C4;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 in_var_Vertex_Color: Vertex_Color;
    [[vk::location(2)]] float3 in_var_Vertex_Normal: Vertex_Normal;
    [[vk::location(3)]] float2 in_var_Vertex_UV: Vertex_UV;
    [[vk::location(4)]] float3 in_var_rest_pos: rest_pos;
    [[vk::location(5)]] float3 in_var_rest_n: rest_n;
    [[vk::location(7)]] float in_var_wing_t: wing_t;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float2 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float3 out_var_C4 : TEXCOORD4;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    Affine3Packed _120;
    [unroll]
    for (int _2ident = 0; _2ident < 12; _2ident++)
    {
        _120.data[_2ident] = nani_instance_data.Load<float>(_2ident * 4 + instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 0);
    }
    Affine3Packed _122;
    [unroll]
    for (int _3ident = 0; _3ident < 12; _3ident++)
    {
        _122.data[_3ident] = nani_instance_data.Load<float>(_3ident * 4 + instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 48);
    }
    float3 _155 = in_var_Vertex_Position * 0.4000000059604644775390625f;
    _155.y = _155.y + ((in_var_wing_t * 0.4000000059604644775390625f) * sin((nani_instance_data.Load<float>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 96) * 6.283185482025146484375f) - (in_var_wing_t * 0.31415927410125732421875f)));
    float3 _166 = nani_instance_data.Load<float>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 100).xxx;
    float3x4 _174 = transpose(float4x3(float3(_120.data[0], _120.data[1], _120.data[2]), float3(_120.data[3], _120.data[4], _120.data[5]), float3(_120.data[6], _120.data[7], _120.data[8]), float3(_120.data[9], _120.data[10], _120.data[11])));
    float4 _178 = float4(lerp(_155, in_var_rest_pos * 0.4000000059604644775390625f, _166), 1.0f);
    float3 _179 = mul(_174, _178);
    float2 _180 = in_var_Vertex_UV * 0.5f;
    float _181 = _180.y;
    float2 _183 = _180;
    _183.y = _181 + 0.5f;
    int _184 = nani_instance_data.Load<int>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 104) - 8 * (nani_instance_data.Load<int>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 104) / 8);
    float2 _189;
    if (_184 == 3)
    {
        float2 _188 = _183;
        _188.y = _181;
        _189 = _188;
    }
    else
    {
        _189 = _183;
    }
    float2 _196;
    if (_184 == 4)
    {
        float2 _195 = _189;
        _195.x = _189.x - 0.5f;
        _196 = _195;
    }
    else
    {
        _196 = _189;
    }
    float2 _201;
    if (_184 == 5)
    {
        _201 = _196 - 0.5f.xx;
    }
    else
    {
        _201 = _196;
    }
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(_179, 1.0f));
    out_var_C0 = in_var_Vertex_Color;
    out_var_C1 = mul(_174, float4(lerp(in_var_Vertex_Normal, in_var_rest_n, _166), 0.0f));
    out_var_C2 = _179;
    out_var_C3 = _201;
    out_var_C4 = mul(transpose(float4x3(float3(_122.data[0], _122.data[1], _122.data[2]), float3(_122.data[3], _122.data[4], _122.data[5]), float3(_122.data[6], _122.data[7], _122.data[8]), float3(_122.data[9], _122.data[10], _122.data[11]))), _178);
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Color = stage_input.in_var_Vertex_Color;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_Vertex_UV = stage_input.in_var_Vertex_UV;
    in_var_rest_pos = stage_input.in_var_rest_pos;
    in_var_rest_n = stage_input.in_var_rest_n;
    in_var_wing_t = stage_input.in_var_wing_t;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    stage_output.out_var_C4 = out_var_C4;
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
