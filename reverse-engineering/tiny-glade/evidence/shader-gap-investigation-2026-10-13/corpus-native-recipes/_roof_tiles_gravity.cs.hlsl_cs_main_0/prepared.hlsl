#define TGV_0 1
#line 1 "shaders/roof_tiles_gravity.cs.hlsl"


#if !((defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)))










#line 1 "shaders/roof/roof_tile.inc.hlsl"













#ifndef TINYGLADE_ROOF_ROOF_TILE_HLSL
#define TINYGLADE_ROOF_ROOF_TILE_HLSL



struct RoofShingle
{
    float3 base_position;
    float current_height;
    uint2 clipping_plane_;
    uint2 scale_t;
    uint2 rotation_quat_;
    uint roof_id;
    uint velocity_rotation;
    float animation_t;
    int seed;
    int is_circle;
    int is_main_roof_body;
};


struct RoofPushConstants
{
    uint first_instance;
    uint instance_count;
    float wall_height_delta;
    float roof_pivot_y;
    float animation_t;
};


uint pack_half2x16(float2 value)
{
    uint2 packed = f32tof16(value);
    return packed.x | (packed.y << 16);
}


float2 unpack_half2x16(uint value)
{
    return f16tof32(uint2(value & 0xFFFFu, value >> 16));
}


float3x4 quat_to_matrix(float4 q, float3 translation)
{
    float x = q.x;
    float y = q.y;
    float z = q.z;
    float w = q.w;
    float x2 = x + x;
    float y2 = y + y;
    float z2 = z + z;
    float xx = x * x2;
    float xy = x * y2;
    float xz = x * z2;
    float yy = y * y2;
    float yz = y * z2;
    float zz = z * z2;
    float wx = w * x2;
    float wy = w * y2;
    float wz = w * z2;
    return float3x4(
        float4(1.0f - (yy + zz), xy + wz, xz - wy, translation.x),
        float4(xy - wz, 1.0f - (xx + zz), yz + wx, translation.y),
        float4(xz + wy, yz - wx, 1.0f - (xx + yy), translation.z));
}

#endif 
#line 14 "shaders/roof_tiles_gravity.cs.hlsl"

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
#line 15 "shaders/roof_tiles_gravity.cs.hlsl"

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
#line 16 "shaders/roof_tiles_gravity.cs.hlsl"

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
#line 17 "shaders/roof_tiles_gravity.cs.hlsl"

#line 1 "shaders/inc/consts.hlsl"






#ifndef TINYGLADE_INC_CONSTS_HLSL
#define TINYGLADE_INC_CONSTS_HLSL


static const float RENDER_SCALE = 1.0f;

#endif 
#line 18 "shaders/roof_tiles_gravity.cs.hlsl"

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
#line 19 "shaders/roof_tiles_gravity.cs.hlsl"

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
#line 20 "shaders/roof_tiles_gravity.cs.hlsl"

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
#line 21 "shaders/roof_tiles_gravity.cs.hlsl"

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
#line 22 "shaders/roof_tiles_gravity.cs.hlsl"

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
#line 23 "shaders/roof_tiles_gravity.cs.hlsl"

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
#line 24 "shaders/roof_tiles_gravity.cs.hlsl"



struct FrameConstants
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

StructuredBuffer<FrameConstants> frame_constants_storage_buffer : register(t1, space1);

RWStructuredBuffer<RoofShingle> __alias_instanced_data : register(u13);




[[vk::push_constant]]
ConstantBuffer<RoofPushConstants> push_constants;

[numthreads(64, 1, 1)]
void cs_main(uint3 gl_GlobalInvocationID : SV_DispatchThreadID)
{
    if (gl_GlobalInvocationID.x >= push_constants.instance_count)
    {
        return;
    }

    uint index = push_constants.first_instance + gl_GlobalInvocationID.x;
    RoofShingle s = __alias_instanced_data[index];

    bool wall_clip;
    if (push_constants.wall_height_delta < 0.0f)
    {
        wall_clip = (push_constants.wall_height_delta / frame_constants_storage_buffer[0].delta_time_seconds) > -120.0f;
    }
    else
    {
        wall_clip = false;
    }

    bool below_roof;
    if (wall_clip)
    {
        below_roof = s.current_height <= (push_constants.roof_pivot_y - push_constants.wall_height_delta);
    }
    else
    {
        below_roof = false;
    }

    float t = floor(unpack_half2x16((s.scale_t.y >> 16u) & 0xFFFFu).x * 50.0f) * 0.02f;
    uint old_velocity = s.velocity_rotation & 0xFFFFu;
    float velocity = unpack_half2x16(old_velocity).x
                   + (-98.1f * lerp(1.0f, 0.4f, t)) * frame_constants_storage_buffer[0].delta_time_seconds;
    float new_height = (below_roof ? push_constants.roof_pivot_y : s.current_height)
                     + velocity * frame_constants_storage_buffer[0].delta_time_seconds;

    float clamped_velocity;
    float height;
    if (new_height <= push_constants.roof_pivot_y)
    {
        clamped_velocity = min(15.0f, max((push_constants.wall_height_delta / frame_constants_storage_buffer[0].delta_time_seconds) * 0.166f, velocity));
        height = max(new_height, push_constants.roof_pivot_y);
    }
    else
    {
        clamped_velocity = velocity;
        height = new_height;
    }

    float eased_height;
    if (t > 0.9f)
    {
        eased_height = push_constants.roof_pivot_y;
    }
    else
    {
        float settle = 1.0f - ((t * t) * (t * t));
        float delay = 1.0f - exp(-frame_constants_storage_buffer[0].delta_time_seconds
                                * (5.0f * lerp(0.0f, 2.0f, settle))
                                * ((rand01(hash(uint(s.seed))) * t > 0.3f) ? 1.0f : 1000.0f));
        eased_height = lerp(height, push_constants.roof_pivot_y, delay);
    }

    float delta = abs(push_constants.roof_pivot_y - eased_height);
    float rot;
    if (clamped_velocity > 0.0f)
    {
        rot = (1.0f - exp(-((delta * delta) * delta))) * -1.57f;
    }
    else
    {
        rot = 0.0f;
    }

    uint new_velocity_rotation = ((old_velocity | (pack_half2x16(float2(rot, 0.0f)) << 16u)) & 0xFFF00000u)
                               | pack_half2x16(float2(clamped_velocity, 0.0f));

    RoofShingle out_s;
    out_s.base_position = s.base_position;
    out_s.current_height = eased_height;
    out_s.clipping_plane_ = s.clipping_plane_;
    out_s.scale_t = s.scale_t;
    out_s.rotation_quat_ = s.rotation_quat_;
    out_s.roof_id = s.roof_id;
    out_s.velocity_rotation = new_velocity_rotation;
    out_s.animation_t = push_constants.animation_t;
    out_s.seed = s.seed;
    out_s.is_circle = s.is_circle;
    out_s.is_main_roof_body = s.is_main_roof_body;
    __alias_instanced_data[index] = out_s;
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

struct FrameConstants
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

struct RoofShingle
{
    float3 base_position;
    float current_height;
    uint2 clipping_plane_;
    uint2 scale_t;
    uint2 rotation_quat_;
    uint roof_id;
    uint velocity_rotation;
    float animation_t;
    int seed;
    int is_circle;
    int is_main_roof_body;
};

ByteAddressBuffer frame_constants_storage_buffer : register(t1, space1);
RWByteAddressBuffer __alias_instanced_data : register(u13, space0);
struct PushConstants
{
    uint first_instance;
    uint instance_count;
    float wall_height_delta;
    float roof_pivot_y;
    float animation_t;
};
[[vk::push_constant]]
PushConstants push_constants;


static uint3 gl_GlobalInvocationID;
struct SPIRV_Cross_Input
{
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
};

uint spvPackHalf2x16(float2 value)
{
    uint2 Packed = f32tof16(value);
    return Packed.x | (Packed.y << 16);
}

float2 spvUnpackHalf2x16(uint value)
{
    return f16tof32(uint2(value & 0xffff, value >> 16));
}

void comp_main()
{
    do
    {
        if (gl_GlobalInvocationID.x >= push_constants.instance_count)
        {
            break;
        }
        uint _97 = push_constants.first_instance + gl_GlobalInvocationID.x;
        RoofShingle _105;
        _105.base_position = __alias_instanced_data.Load<float3>(_97 * 64 + 0);
        _105.current_height = __alias_instanced_data.Load<float>(_97 * 64 + 12);
        _105.clipping_plane_ = __alias_instanced_data.Load<uint2>(_97 * 64 + 16);
        _105.scale_t = __alias_instanced_data.Load<uint2>(_97 * 64 + 24);
        _105.rotation_quat_ = __alias_instanced_data.Load<uint2>(_97 * 64 + 32);
        _105.roof_id = __alias_instanced_data.Load<uint>(_97 * 64 + 40);
        _105.velocity_rotation = __alias_instanced_data.Load<uint>(_97 * 64 + 44);
        _105.animation_t = __alias_instanced_data.Load<float>(_97 * 64 + 48);
        _105.seed = __alias_instanced_data.Load<int>(_97 * 64 + 52);
        _105.is_circle = __alias_instanced_data.Load<int>(_97 * 64 + 56);
        _105.is_main_roof_body = __alias_instanced_data.Load<int>(_97 * 64 + 60);
        bool _121;
        if (push_constants.wall_height_delta < 0.0f)
        {
            _121 = (push_constants.wall_height_delta / frame_constants_storage_buffer.Load<float>(132)) > (-120.0f);
        }
        else
        {
            _121 = false;
        }
        bool _126;
        if (_121)
        {
            _126 = _105.current_height <= (push_constants.roof_pivot_y - push_constants.wall_height_delta);
        }
        else
        {
            _126 = false;
        }
        float _135 = floor(spvUnpackHalf2x16((_105.scale_t.y >> 16u) & 65535u).x * 50.0f) * 0.0199999995529651641845703125f;
        uint _138 = _105.velocity_rotation & 65535u;
        float _142 = spvUnpackHalf2x16(_138).x + (((-98.09999847412109375f) * lerp(1.0f, 0.4000000059604644775390625f, _135)) * frame_constants_storage_buffer.Load<float>(132));
        float _144 = (_126 ? push_constants.roof_pivot_y : _105.current_height) + (_142 * frame_constants_storage_buffer.Load<float>(132));
        float _153;
        float _154;
        if (_144 <= push_constants.roof_pivot_y)
        {
            _153 = min(15.0f, max((push_constants.wall_height_delta / frame_constants_storage_buffer.Load<float>(132)) * 0.16599999368190765380859375f, _142));
            _154 = max(_144, push_constants.roof_pivot_y);
        }
        else
        {
            _153 = _142;
            _154 = _144;
        }
        uint _155 = uint(_105.seed);
        uint _157 = _155 + (_155 << 10u);
        uint _159 = _157 ^ (_157 >> 6u);
        uint _161 = _159 + (_159 << 3u);
        uint _163 = _161 ^ (_161 >> 11u);
        float _189;
        if (_135 > 0.89999997615814208984375f)
        {
            _189 = push_constants.roof_pivot_y;
        }
        else
        {
            _189 = lerp(_154, push_constants.roof_pivot_y, 1.0f - exp((-frame_constants_storage_buffer.Load<float>(132)) * ((5.0f * lerp(0.0f, 2.0f, 1.0f - (((_135 * _135) * _135) * _135))) * ((((asfloat(((_163 + (_163 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _135) > 0.300000011920928955078125f) ? 1.0f : 1000.0f))));
        }
        float _191 = abs(push_constants.roof_pivot_y - _189);
        float _201;
        if (_153 > 0.0f)
        {
            _201 = (1.0f - exp(((-_191) * _191) * _191)) * (-1.57000005245208740234375f);
        }
        else
        {
            _201 = 0.0f;
        }
        RoofShingle _212 = { _105.base_position, _189, _105.clipping_plane_, _105.scale_t, _105.rotation_quat_, _105.roof_id, ((_138 | (spvPackHalf2x16(float2(_201, 0.0f)) << 16u)) & 4294901760u) | spvPackHalf2x16(float2(_153, 0.0f)), push_constants.animation_t, _105.seed, _105.is_circle, _105.is_main_roof_body };
        __alias_instanced_data.Store<float3>(_97 * 64 + 0, _212.base_position);
        __alias_instanced_data.Store<float>(_97 * 64 + 12, _212.current_height);
        __alias_instanced_data.Store<uint2>(_97 * 64 + 16, _212.clipping_plane_);
        __alias_instanced_data.Store<uint2>(_97 * 64 + 24, _212.scale_t);
        __alias_instanced_data.Store<uint2>(_97 * 64 + 32, _212.rotation_quat_);
        __alias_instanced_data.Store<uint>(_97 * 64 + 40, _212.roof_id);
        __alias_instanced_data.Store<uint>(_97 * 64 + 44, _212.velocity_rotation);
        __alias_instanced_data.Store<float>(_97 * 64 + 48, _212.animation_t);
        __alias_instanced_data.Store<int>(_97 * 64 + 52, _212.seed);
        __alias_instanced_data.Store<int>(_97 * 64 + 56, _212.is_circle);
        __alias_instanced_data.Store<int>(_97 * 64 + 60, _212.is_main_roof_body);
        break;
    } while(false);
}

[numthreads(64, 1, 1)]
void cs_main(SPIRV_Cross_Input stage_input)
{
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    comp_main();
}
#pragma dxc diagnostic pop
#endif
