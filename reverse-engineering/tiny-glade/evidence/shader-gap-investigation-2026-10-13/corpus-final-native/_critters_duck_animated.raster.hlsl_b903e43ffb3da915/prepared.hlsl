#define SHADER_VARIANT_DEPTH_ONLY 0
#line 1 "shaders/critters/duck_animated.raster.hlsl"


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
#line 3 "shaders/critters/duck_animated.raster.hlsl"

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
#line 4 "shaders/critters/duck_animated.raster.hlsl"

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
#line 5 "shaders/critters/duck_animated.raster.hlsl"

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
#line 6 "shaders/critters/duck_animated.raster.hlsl"

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
#line 7 "shaders/critters/duck_animated.raster.hlsl"






float4x4 model_to_world;
float4x4 prev_model_to_world;
float anim_t;
float landing_t;
uint is_landing;
uint is_taking_off;
float pet_t;
int pet_t_is_playing_forward;
int bool_render_shadows;

struct VsIn
{
    [[vk::location(0)]] float3 Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 Vertex_Color: Vertex_Color;
    [[vk::location(2)]] float3 Vertex_Normal: Vertex_Normal;
    [[vk::location(3)]] float3 rest_pos: rest_pos;
    [[vk::location(4)]] float3 rest_n: rest_n;
    [[vk::location(5)]] int is_wing: is_wing;
    [[vk::location(6)]] float wing_t: wing_t;
    [[vk::location(7)]] int is_mouth_bottom: is_mouth_bottom;
    [[vk::location(8)]] float3 pet_pos: pet_pos;
    [[vk::location(9)]] float3 pet_n: pet_n;
};

struct VsOut
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float3 C3 : TEXCOORD3;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;

    float3 _95 = vin.Vertex_Position;
    _95.y = _95.y + ((vin.wing_t * 0.5f) * cos((frac(anim_t) * 6.283185482025146484375f) - (vin.wing_t * 1.57079637050628662109375f)));

    float3 _172;
    float3 _173;
    if (vin.is_wing == 0)
    {
        float _141;
        if (is_landing != 0u)
        {
            float _130 = clamp(1.0f - landing_t, 0.0f, 1.0f);
            _141 = 1.0f - ((_130 * _130) - (_130 * sin(_130 * 3.1400001049041748046875f)));
        }
        else
        {
            _141 = (is_taking_off != 0u) ? 0.0f : landing_t;
        }
        float3 _142 = _141.xxx;
        _172 = lerp(vin.Vertex_Normal, vin.rest_n, _142);
        _173 = lerp(_95, vin.rest_pos, _142);
    }
    else
    {
        float _168;
        if (is_landing != 0u)
        {
            float _154 = clamp(smoothstep(0.0f, 0.800000011920928955078125f, landing_t), 0.0f, 1.0f);
            _168 = ((_154 * _154) * _154) - (_154 * sin(_154 * 3.1400001049041748046875f));
        }
        else
        {
            float _161 = clamp(landing_t, 0.0f, 1.0f);
            _168 = ((_161 * _161) * _161) - (_161 * sin(_161 * 3.1400001049041748046875f));
        }
        float3 _169 = _168.xxx;
        _172 = lerp(vin.Vertex_Normal, vin.rest_n, _169);
        _173 = lerp(_95, vin.rest_pos, _169);
    }

    float _199;
    if (pet_t_is_playing_forward == 1)
    {
        float _183 = clamp(1.0f - pet_t, 0.0f, 1.0f);
        _199 = 1.0f - ((_183 * _183) - (_183 * sin(_183 * 3.1400001049041748046875f)));
    }
    else
    {
        float _192 = clamp(pet_t, 0.0f, 1.0f);
        _199 = ((_192 * _192) * _192) - (_192 * sin(_192 * 3.1400001049041748046875f));
    }
    float3 _200 = _199.xxx;
    float3 _201 = lerp(_173, vin.pet_pos, _200);
    if (vin.is_mouth_bottom == 1)
    {
        float3 _210 = _201;
        _210.y = _201.y + (min(_199, 0.0f) * 0.100000001490116119384765625f);
        _201 = _210;
    }

    float4 _218 = float4(_201 * 0.60000002384185791015625f, 1.0f);
    float4 _219 = mul(model_to_world, _218);
    o.position = mul(view_constants[current_view_index].world_to_sample, _219);
    o.C0 = vin.Vertex_Color;
    o.C1 = mul(model_to_world, float4(lerp(_172, vin.pet_n, _200), 0.0f)).xyz;
    o.C2 = _219.xyz;
    o.C3 = mul(prev_model_to_world, _218).xyz;
    return o;
}

struct PsIn
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float3 C3 : TEXCOORD3;
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
    float3 _83 = sqrt(clamp(pin.C0, 0.0f, 1.0f));
    uint albedo_word = (uint((clamp(_83.x, 0.0f, 1.0f) * 255.0f) + 0.5f)
                      + (uint((clamp(_83.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)
                      + (uint((clamp(_83.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u));
    uint2 gbuf = uint2(((uint((clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f) + 0.5f)
                      + (uint((clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u))
                      + (uint((clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 3422552064u, albedo_word);
    gbuf.y = albedo_word | 117440512u;
    o.gbuffer = gbuf;
    o.velocity = pin.C2 - pin.C3;
    return o;
}
