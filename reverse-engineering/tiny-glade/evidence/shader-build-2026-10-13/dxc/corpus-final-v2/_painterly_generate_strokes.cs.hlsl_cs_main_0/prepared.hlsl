#line 1 "shaders/painterly/generate_strokes.cs.hlsl"






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
#line 7 "shaders/painterly/generate_strokes.cs.hlsl"
            
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
#line 8 "shaders/painterly/generate_strokes.cs.hlsl"

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
#line 9 "shaders/painterly/generate_strokes.cs.hlsl"

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
#line 10 "shaders/painterly/generate_strokes.cs.hlsl"

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
#line 11 "shaders/painterly/generate_strokes.cs.hlsl"

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
#line 12 "shaders/painterly/generate_strokes.cs.hlsl"

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
#line 13 "shaders/painterly/generate_strokes.cs.hlsl"

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
#line 14 "shaders/painterly/generate_strokes.cs.hlsl"

#line 1 "shaders/painterly/stroke.inc.hlsl"







#ifndef TINYGLADE_PAINTERLY_STROKE_INC_HLSL
#define TINYGLADE_PAINTERLY_STROKE_INC_HLSL

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
#line 11 "shaders/painterly/stroke.inc.hlsl"


struct PainterlyStrokePacked
{
    uint pos;
    uint color;
    uint size_rotation;
    float appear_t;
};


uint pack_half2(float2 v)
{
    uint2 p = f32tof16(v);
    return p.x | (p.y << 16u);
}


float unpack_half_x(uint v)
{
    return f16tof32(v & 0xffffu);
}


float unpack_half_y(uint v)
{
    return f16tof32(v >> 16u);
}


float2 unpack_stroke_pos(uint pos)
{
    return float2(unpack_half_x(pos & 0xffffu), unpack_half_x(pos >> 16u));
}


float2 unpack_stroke_size_rotation(uint size_rotation)
{
    return float2(unpack_half_x(size_rotation & 0xffffu), unpack_half_x(size_rotation >> 16u));
}

#endif 
#line 15 "shaders/painterly/generate_strokes.cs.hlsl"


cbuffer $Globals : register(b13, space0)
{
    uint2 target_tex_dims;
    uint2 depth_target_tex_dims;
};

struct PushConstants
{
    uint strokes_captured;
    uint strokes_this_frame;
    float current_t;
};

[[vk::push_constant]]
PushConstants push_constants;

Texture2D<float4> target_tex : register(t14, space0);
Texture2D<float4> depth_target_tex : register(t15, space0);
RWByteAddressBuffer __alias_strokes_buf : register(u16, space0);

[numthreads(32, 1, 1)]
void cs_main(uint3 dispatch_thread_id : SV_DispatchThreadID)
{
    if (dispatch_thread_id.x >= push_constants.strokes_this_frame)
    {
        return;
    }

    uint stroke_index = push_constants.strokes_captured + dispatch_thread_id.x;
    float n = float(stroke_index);

    
    float2 rnd = frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * n) + 0.5f.xx);

    float aspect = float(target_tex_dims.x) / float(target_tex_dims.y);
    float2 aspect_vec = float2(aspect, 1.0f);
    float base_size = (0.0500000007450580596923828125f * sqrt(aspect)) *
                      lerp(1.2999999523162841796875f, 0.60000002384185791015625f, pow(clamp(n * 0.0009765625f, 0.0f, 1.0f), 1.0f));

    
    
    float2 pos;
    if (rnd.x > 0.300000011920928955078125f)
    {
        float2 jitter_seed = frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) *
                                   float(1024u + ((stroke_index * 839u) % 1024u))) + 0.5f.xx);
        float2 candidate = frac(rnd + (((2.0f * base_size).xx / aspect_vec) * (jitter_seed - 0.5f.xx)));
        bool2 same_side = (depth_target_tex.SampleLevel(sampler_lnc, candidate, 0.0f).x >
                           depth_target_tex.SampleLevel(sampler_lnc, rnd, 0.0f).x) == (stroke_index < 341u).xx;
        pos = float2(same_side.x ? rnd.x : candidate.x, same_side.y ? rnd.y : candidate.y);
    }
    else
    {
        pos = rnd;
    }

    float4 target = target_tex.SampleLevel(sampler_lnc, pos, 0.0f);
    float2 centered = pos - 0.5f.xx;
    float2 doubled = centered * 2.0f.xx;

    float size = base_size * exp2(min(1.5f, (lerp(0.0f, 3.0f, clamp(dot(doubled, doubled), 0.0f, 1.0f)) +
                lerp(0.800000011920928955078125f, 0.0f, smoothstep(0.0f, 0.5f, pos.x))) +
                lerp(0.0f, 1.0f, clamp(abs((target.w * 2.0f) - 1.0f), 0.0f, 1.0f))));

    
    float2 pos_uv = (((centered * float2(2.0f, -2.0f)) * aspect_vec) * float2(0.5f, -0.5f)) + 0.5f.xx;

    float3 stroke_color = linear_to_srgb(target.xyz);

    
    float angle = float(hash(stroke_index + 1u) % 360u) * 0.0174532942473888397216796875f;
    float best_angle = angle;
    float best_error = 10000000000.0f;
    for (uint k = 0u; k < 8u; k++)
    {
        float a = angle + (float(k) * 0.3926990926265716552734375f);
        float2x2 rot = float2x2(float2(cos(a), sin(a)), float2(-sin(a), cos(a)));
        float half_size = size * 0.5f;
        float2 off_x = mul(rot, float2(half_size, 0.0f)) / aspect_vec;
        float2 off_y = mul(rot, float2(0.0f, half_size)) / aspect_vec;

        float err = 0.0f;
        for (uint v = 0u; v < 2u; v++)
        {
            for (uint u = 0u; u < 2u; u++)
            {
                float2 sample_pos = pos + (off_x * ((float(u) * 2.0f) - 1.0f)) +
                                    (off_y * (((float(v) * 2.0f) - 1.0f) * 0.20000000298023223876953125f));
                float3 diff = target.xyz - target_tex.SampleLevel(sampler_lnc, sample_pos, 0.0f).xyz;
                err += dot(diff, diff);
            }
        }
        err *= 0.25f;
        if (err < best_error)
        {
            best_error = err;
            best_angle = a;
        }
    }

    
    if (pos.x > 0.300000011920928955078125f)
    {
        if (best_error <= 0.20000000298023223876953125f)
        {
            size *= lerp(1.0f, 0.25f, smoothstep(0.0f, 0.20000000298023223876953125f, best_error));
        }
        else
        {
            size = 0.0f;
        }
    }

    PainterlyStrokePacked stroke;
    stroke.pos = pack_half2(pos_uv);
    stroke.color = pack_rgba8(float4(stroke_color, 1.0f));
    stroke.size_rotation = pack_half2(float2(size, best_angle));
    stroke.appear_t = push_constants.current_t;

    uint byte_offset = stroke_index * 16u;
    __alias_strokes_buf.Store<uint>(byte_offset + 0u, stroke.pos);
    __alias_strokes_buf.Store<uint>(byte_offset + 4u, stroke.color);
    __alias_strokes_buf.Store<uint>(byte_offset + 8u, stroke.size_rotation);
    __alias_strokes_buf.Store<float>(byte_offset + 12u, stroke.appear_t);
}
