#define SHADER_VARIANT_DEPTH_ONLY 0
#define TGV_b903e43ffb3da915 1
#line 1 "shaders/critters/butterfly.raster.hlsl"


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
#line 6 "shaders/critters/butterfly.raster.hlsl"

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
#line 7 "shaders/critters/butterfly.raster.hlsl"

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
#line 8 "shaders/critters/butterfly.raster.hlsl"

Texture2D<float4> terrain_heightmap : register(t17);
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
#line 10 "shaders/critters/butterfly.raster.hlsl"

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
#line 11 "shaders/critters/butterfly.raster.hlsl"

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
#line 12 "shaders/critters/butterfly.raster.hlsl"






struct Butterfly
{
    float3 position;
    float anim_t;
    float3 forward;
    uint pad;
};

StructuredBuffer<Butterfly> butterfly_buffer : register(t19);
Texture2DArray<float4> shadow_map : register(t14);
Texture2D<float4> butterfly_skins : register(t16);

int bool_render_shadows;
uint2 butterfly_skins_dims;
uint2 terrain_heightmap_dims;
uint2 visual_terrain_heightmap_dims;





float2 butterfly_shadow(float3 world_pos, float4 shadow_dir)
{
    float4 _237 = mul(view_constants[2].world_to_sample, float4(world_pos, 1.0f));
    if (_237.z > 1.0f)
    {
        return 0.0f.xx;
    }
    float2 _242 = _237.xy;
    float2 _243 = abs(_242);
    bool _245 = any(bool2(_243.x > 0.9833333492279052734375f, _243.y > 0.9833333492279052734375f));
    float2 _249;
    float _250;
    float _252;
    if (_245)
    {
        _249 = _242 * 0.5f;
        _250 = 5.0f;
        _252 = 0.5f;
    }
    else
    {
        _249 = _242;
        _250 = 10.0f;
        _252 = 1.0f;
    }
    float2 _253 = abs(_249);
    float2 _256 = 1.0f.xx - (0.000833333353511989116668701171875f.xx * (2.0f * _250));
    bool _258 = any(bool2(_253.x > _256.x, _253.y > _256.y));
    float _264;
    float2 _265;
    float _266;
    if (_258)
    {
        _264 = _250 * 0.2857142984867095947265625f;
        _265 = _249 * 0.2857142984867095947265625f;
        _266 = _252 * 0.2857142984867095947265625f;
    }
    else
    {
        _264 = _250;
        _265 = _249;
        _266 = _252;
    }
    float _269 = float(_258 ? 2u : uint(_245));
    uint _271 = 2u + uint(_269);
    float _275 = shadow_dir.y;
    float4 _287 = float4(_265.x, _265.y, _237.z, _237.w)
        + mul(view_constants[_271].world_to_sample,
              float4((float3(0.0f, 1.0f, 0.0f) * (0.066666670143604278564453125f * clamp(1.0f - _275, 0.0f, 1.0f))) / _266.xxx, 0.0f));
    float2 _290 = (_287.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
    float3 _291 = float3(_290.x, _290.y, _287.z);
    float _296 = max((0.300000011920928955078125f / _266) * (1.0f - abs(_275)), 0.02500000037252902984619140625f);
    float _302 = float((frame_index * 5u) % 64u) * 1.61803400516510009765625f;
    float _308 = frac(52.98291778564453125f * frac(float(frame_index % 64u) * 0.40764915943145751953125f));
    float4 _310 = mul(view_constants[_271].world_to_sample, float4(0.0f, 1.0f, 0.0f, 0.0f));
    float _312 = 0.0f;
    int _315 = 0;
    for (int _317 = 0; _317 < 5; _317++)
    {
        float _322 = float(_317);
        float _324 = (_322 + _302) * 2.3999631404876708984375f;
        float3 _337 = _291 + float3(((float2(cos(_324), sin(_324)) * (_322 + _308)) * 0.20000000298023223876953125f.xx) * _264 * 0.000833333353511989116668701171875f.xx, _296 * (-0.0009765625f));
        float4 _344 = shadow_map.SampleLevel(sampler_shadow_nearest, float3(_337.xy, _269), 0.0f);
        float _345 = _344.x;
        if (_337.z > _345)
        {
            _312 = _312 + _345;
            _315 = _315 + 1;
        }
    }
    float _353 = _312 / float(_315);
    if (_315 == 0)
    {
        return 0.0f.xx;
    }
    float _367 = max(((_287.z - _353) * ((17.0666675567626953125f * _264) * pow(10.0f, smoothstep(0.0f, 0.25f, fog_tint_thickness.w)))) / _353, 0.0f);
    float _374 = (_367 / pow(pow(_367 / (_264 - 1.0f), 5.0f) + 1.0f, 0.20000000298023223876953125f)) + 1.0f;
    float _376 = _310.z;
    float2 _386 = (((_310.xy * float(int(sign(_376)))) / max(0.1500000059604644775390625f, abs(_376)).xx) * 0.0009765625f.xx) * 2.0f;
    float2 _388 = 0.0f.xx;
    for (int _391 = 0; _391 < 10; _391++)
    {
        float _395 = float(_391);
        float _397 = (_395 + _302) * 2.3999631404876708984375f;
        float2 _404 = (float2(cos(_397), sin(_397)) * (_395 + _308)) * 0.100000001490116119384765625f.xx * _374;
        float3 _410 = _291 + float3(_404 * 0.000833333353511989116668701171875f.xx, _296 * (-0.0009765625f));
        _388 = _388 + float2(1.0f - shadow_map.SampleCmpLevelZero(sampler_shadow_linear, float3(_410.xy, _269), _410.z + min(0.0f, dot(_386, _404))), 0.0f);
    }
    return _388 * 0.100000001490116119384765625f;
}

struct VsIn
{
    [[vk::location(0)]] float3 Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float2 Vertex_UV: Vertex_UV;
    [[vk::location(2)]] float is_wing: is_wing;
    uint instance_id : SV_InstanceID;
};

struct VsOut
{
    float3 C0 : TEXCOORD0;
    float2 C1 : TEXCOORD1;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;

    float2 _156 = vin.Vertex_UV;
    float3 _168 = vin.Vertex_Position * 0.4000000059604644775390625f;
    float3 _206;
    if (vin.is_wing > 0.0f)
    {
        float _176 = abs(0.5f - butterfly_buffer[vin.instance_id].anim_t);
        float _190 = lerp(-0.60000002384185791015625f, 1.39999997615814208984375f,
            (cos((butterfly_buffer[vin.instance_id].anim_t + ((_168.z + 0.300000011920928955078125f) * (-0.25f)) * smoothstep(0.0f, 0.5f, _176)) + 0.5f) * 6.283185482025146484375f) * 0.5f + 0.5f)
            * float(int(sign(_168.x)));
        float _191 = cos(_190);
        float _192 = sin(_190);
        float2 _201 = _168.xy;
        float2 _204 = lerp(_201, mul(transpose(float2x2(float2(_191, _192), float2(-_192, _191))), _201), lerp(vin.is_wing, 1.0f, 1.0f - (_176 * 2.0f)).xx);
        _206 = float3(_204.x, _204.y, _168.z);
    }
    else
    {
        _206 = _168;
    }
    float3 _219 = (mul(float3x3(cross(butterfly_buffer[vin.instance_id].forward, float3(0.0f, 1.0f, 0.0f)), float3(0.0f, 1.0f, 0.0f), butterfly_buffer[vin.instance_id].forward), _206) + butterfly_buffer[vin.instance_id].position) + float3(0.0f, lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, butterfly_buffer[vin.instance_id].position.xz * 0.0076923076994717121124267578125f.xx + 0.5f.xx, 0.0f).x), 0.0f);

    float _427;
    if (bool_render_shadows == 1)
    {
        float4 _230 = mul(view_constants[2].view_to_world, float4(0.0f, 0.0f, 1.0f, 0.0f));
        float2 _425 = butterfly_shadow(_219, _230);
        _427 = _425.x;
    }
    else
    {
        _427 = 0.0f;
    }

    _156.y = 1.0f - _156.y;
    float2 _434 = _156 * 0.5f;
    float _435 = _434.y;
    float2 _437 = _434;
    _437.y = _435 + 0.5f;
    uint _438 = vin.instance_id % 8u;
    float2 _443;
    if (_438 == 3u)
    {
        float2 _442 = _437;
        _442.y = _435;
        _443 = _442;
    }
    else
    {
        _443 = _437;
    }
    float2 _450;
    if (_438 == 4u)
    {
        float2 _449 = _443;
        _449.x = _443.x - 0.5f;
        _450 = _449;
    }
    else
    {
        _450 = _443;
    }
    float2 _455;
    if (_438 == 5u)
    {
        _455 = _450 - 0.5f.xx;
    }
    else
    {
        _455 = _450;
    }

    o.position = mul(view_constants[current_view_index].world_to_sample, float4(_219, 1.0f));
    o.C0 = ((sun_color.xyz * (1.0f - _427)) * 0.3183098733425140380859375f.xxx) + sky_high_color.xyz;
    o.C1 = _455;
    return o;
}

struct PsIn
{
    float3 C0 : TEXCOORD0;
    float2 C1 : TEXCOORD1;
};

struct PsOut
{
    float4 color : SV_Target0;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;
    float3 col = (butterfly_skins.Sample(sampler_llr, pin.C1).xyz * pin.C0) * 0.800000011920928955078125f;
    col = ((render_overrides & 1u) != 0u) ? 0.5f.xxx : col;
    o.color = float4(col, 1.0f);
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

struct Butterfly
{
    float3 position;
    float anim_t;
    float3 forward;
    uint pad;
};

static float _148;

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
    int _Globals_bool_render_shadows : packoffset(c0.y);
    uint2 _Globals_butterfly_skins_dims : packoffset(c0.z);
    uint2 _Globals_terrain_heightmap_dims : packoffset(c1);
    uint2 _Globals_visual_terrain_heightmap_dims : packoffset(c1.z);
};

ByteAddressBuffer butterfly_buffer : register(t19, space0);
SamplerState sampler_default : register(s9, space0);
SamplerState sampler_shadow_nearest : register(s11, space0);
SamplerComparisonState sampler_shadow_linear : register(s12, space0);
Texture2DArray<float4> shadow_map : register(t14, space0);
Texture2D<float4> terrain_heightmap : register(t17, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float2 in_var_Vertex_UV;
static float in_var_is_wing;
static float3 out_var_C0;
static float2 out_var_C1;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float2 in_var_Vertex_UV: Vertex_UV;
    [[vk::location(2)]] float in_var_is_wing: is_wing;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float2 out_var_C1 : TEXCOORD1;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    float2 _156 = in_var_Vertex_UV;
    float3 _168 = in_var_Vertex_Position * 0.4000000059604644775390625f;
    float3 _206;
    if (in_var_is_wing > 0.0f)
    {
        float _176 = abs(0.5f - butterfly_buffer.Load<float>(uint(gl_InstanceIndex) * 32 + 12));
        float _190 = lerp(-0.60000002384185791015625f, 1.39999997615814208984375f, (cos(((butterfly_buffer.Load<float>(uint(gl_InstanceIndex) * 32 + 12) + (((_168.z + 0.300000011920928955078125f) * (-0.25f)) * smoothstep(0.0f, 0.5f, _176))) + 0.5f) * 6.283185482025146484375f) * 0.5f) + 0.5f) * float(int(sign(_168.x)));
        float _191 = cos(_190);
        float _192 = sin(_190);
        float2 _201 = _168.xy;
        float2 _204 = lerp(_201, mul(transpose(float2x2(float2(_191, _192), float2(-_192, _191))), _201), lerp(in_var_is_wing, 1.0f, 1.0f - (_176 * 2.0f)).xx);
        _206 = float3(_204.x, _204.y, _168.z);
    }
    else
    {
        _206 = _168;
    }
    float3 _219 = (mul(float3x3(cross(butterfly_buffer.Load<float3>(uint(gl_InstanceIndex) * 32 + 16), float3(0.0f, 1.0f, 0.0f)), float3(0.0f, 1.0f, 0.0f), butterfly_buffer.Load<float3>(uint(gl_InstanceIndex) * 32 + 16)), _206) + butterfly_buffer.Load<float3>(uint(gl_InstanceIndex) * 32 + 0)) + float3(0.0f, lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (butterfly_buffer.Load<float3>(uint(gl_InstanceIndex) * 32 + 0).xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x), 0.0f);
    float _427;
    if (_Globals_bool_render_shadows == 1)
    {
        float4 _230 = mul(frame_constants_view_constants[2u].view_to_world, float4(0.0f, 0.0f, 1.0f, 0.0f));
        float2 _425;
        do
        {
            float4 _237 = mul(frame_constants_view_constants[2u].world_to_sample, float4(_219, 1.0f));
            if (_237.z > 1.0f)
            {
                _425 = 0.0f.xx;
                break;
            }
            float2 _242 = _237.xy;
            float2 _243 = abs(_242);
            bool _245 = any(bool2(_243.x > 0.9833333492279052734375f.xx.x, _243.y > 0.9833333492279052734375f.xx.y));
            float2 _249;
            if (_245)
            {
                _249 = _242 * 0.5f;
            }
            else
            {
                _249 = _242;
            }
            float _250 = _245 ? 5.0f : 10.0f;
            float _252 = _245 ? 0.5f : 1.0f;
            float2 _253 = abs(_249);
            float2 _256 = 1.0f.xx - (0.000833333353511989116668701171875f.xx * (2.0f * _250));
            bool _258 = any(bool2(_253.x > _256.x, _253.y > _256.y));
            float _264;
            float2 _265;
            float _266;
            if (_258)
            {
                _264 = _250 * 0.2857142984867095947265625f;
                _265 = _249 * 0.2857142984867095947265625f;
                _266 = _252 * 0.2857142984867095947265625f;
            }
            else
            {
                _264 = _250;
                _265 = _249;
                _266 = _252;
            }
            float _269 = float(_258 ? 2u : uint(_245));
            uint _271 = 2u + uint(_269);
            float _275 = _230.y;
            float4 _287 = float4(_265.x, _265.y, _237.z, _237.w) + mul(frame_constants_view_constants[_271].world_to_sample, float4((float3(0.0f, 1.0f, 0.0f) * (0.066666670143604278564453125f * clamp(1.0f - _275, 0.0f, 1.0f))) / _266.xxx, 0.0f));
            float2 _290 = (_287.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
            float3 _291 = float3(_290.x, _290.y, _287.z);
            float _296 = max((0.300000011920928955078125f / _266) * (1.0f - abs(_275)), 0.02500000037252902984619140625f);
            float _302 = float((frame_constants_frame_index * 5u) % 64u) * 1.61803400516510009765625f;
            float _308 = frac(52.98291778564453125f * frac(float(frame_constants_frame_index % 64u) * 0.40764915943145751953125f));
            float4 _310 = mul(frame_constants_view_constants[_271].world_to_sample, float4(0.0f, 1.0f, 0.0f, 0.0f));
            float _312;
            int _315;
            _312 = 0.0f;
            _315 = 0;
            float _313;
            int _316;
            for (int _317 = 0; _317 < 5; _312 = _313, _315 = _316, _317++)
            {
                float _322 = float(_317);
                float _324 = (_322 + _302) * 2.3999631404876708984375f;
                float3 _337 = _291 + float3((((float2(cos(_324), sin(_324)) * (_322 + _308)) * 0.20000000298023223876953125f.xx) * _264) * 0.000833333353511989116668701171875f.xx, _296 * (-0.0009765625f));
                float4 _344 = shadow_map.SampleLevel(sampler_shadow_nearest, float3(_337.xy, _269), 0.0f);
                float _345 = _344.x;
                if (_337.z > _345)
                {
                    _313 = _312 + _345;
                    _316 = _315 + 1;
                }
                else
                {
                    _313 = _312;
                    _316 = _315;
                }
            }
            float _353 = _312 / float(_315);
            if (_315 == 0)
            {
                _425 = 0.0f.xx;
                break;
            }
            float _367 = max(((_287.z - _353) * ((17.0666675567626953125f * _264) * pow(10.0f, smoothstep(0.0f, 0.25f, frame_constants_fog_tint_thickness.w)))) / _353, 0.0f);
            float _374 = (_367 / pow(pow(_367 / (_264 - 1.0f), 5.0f) + 1.0f, 0.20000000298023223876953125f)) + 1.0f;
            float _376 = _310.z;
            float2 _386 = (((_310.xy * float(int(sign(_376)))) / max(0.1500000059604644775390625f, abs(_376)).xx) * 0.0009765625f.xx) * 2.0f;
            float2 _388;
            _388 = 0.0f.xx;
            for (int _391 = 0; _391 < 10; )
            {
                float _395 = float(_391);
                float _397 = (_395 + _302) * 2.3999631404876708984375f;
                float2 _404 = ((float2(cos(_397), sin(_397)) * (_395 + _308)) * 0.100000001490116119384765625f.xx) * _374;
                float3 _410 = _291 + float3(_404 * 0.000833333353511989116668701171875f.xx, _296 * (-0.0009765625f));
                _388 += float2(1.0f - shadow_map.SampleCmpLevelZero(sampler_shadow_linear, float3(_410.xy, _269), _410.z + min(0.0f, dot(_386, _404))), _148);
                _391++;
                continue;
            }
            _425 = _388 * 0.100000001490116119384765625f;
            break;
        } while(false);
        _427 = _425.x;
    }
    else
    {
        _427 = 0.0f;
    }
    _156.y = 1.0f - _156.y;
    float2 _434 = _156 * 0.5f;
    float _435 = _434.y;
    float2 _437 = _434;
    _437.y = _435 + 0.5f;
    uint _438 = uint(gl_InstanceIndex) % 8u;
    float2 _443;
    if (_438 == 3u)
    {
        float2 _442 = _437;
        _442.y = _435;
        _443 = _442;
    }
    else
    {
        _443 = _437;
    }
    float2 _450;
    if (_438 == 4u)
    {
        float2 _449 = _443;
        _449.x = _443.x - 0.5f;
        _450 = _449;
    }
    else
    {
        _450 = _443;
    }
    float2 _455;
    if (_438 == 5u)
    {
        _455 = _450 - 0.5f.xx;
    }
    else
    {
        _455 = _450;
    }
    gl_Position = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, float4(_219, 1.0f));
    out_var_C0 = ((frame_constants_sun_color.xyz * (1.0f - _427)) * 0.3183098733425140380859375f.xxx) + frame_constants_sky_high_color.xyz;
    out_var_C1 = _455;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_UV = stage_input.in_var_Vertex_UV;
    in_var_is_wing = stage_input.in_var_is_wing;
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

struct Butterfly
{
    float3 position;
    float anim_t;
    float3 forward;
    uint pad;
};

static float _148;

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
    int _Globals_bool_render_shadows : packoffset(c0.y);
    uint2 _Globals_butterfly_skins_dims : packoffset(c0.z);
    uint2 _Globals_terrain_heightmap_dims : packoffset(c1);
    uint2 _Globals_visual_terrain_heightmap_dims : packoffset(c1.z);
};

ByteAddressBuffer butterfly_buffer : register(t19, space0);
SamplerState sampler_default : register(s9, space0);
SamplerState sampler_shadow_nearest : register(s11, space0);
SamplerComparisonState sampler_shadow_linear : register(s12, space0);
Texture2DArray<float4> shadow_map : register(t14, space0);
Texture2D<float4> terrain_heightmap : register(t17, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float2 in_var_Vertex_UV;
static float in_var_is_wing;
static float3 out_var_C0;
static float2 out_var_C1;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float2 in_var_Vertex_UV: Vertex_UV;
    [[vk::location(2)]] float in_var_is_wing: is_wing;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float2 out_var_C1 : TEXCOORD1;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    float2 _156 = in_var_Vertex_UV;
    float3 _168 = in_var_Vertex_Position * 0.4000000059604644775390625f;
    float3 _206;
    if (in_var_is_wing > 0.0f)
    {
        float _176 = abs(0.5f - butterfly_buffer.Load<float>(uint(gl_InstanceIndex) * 32 + 12));
        float _190 = lerp(-0.60000002384185791015625f, 1.39999997615814208984375f, (cos(((butterfly_buffer.Load<float>(uint(gl_InstanceIndex) * 32 + 12) + (((_168.z + 0.300000011920928955078125f) * (-0.25f)) * smoothstep(0.0f, 0.5f, _176))) + 0.5f) * 6.283185482025146484375f) * 0.5f) + 0.5f) * float(int(sign(_168.x)));
        float _191 = cos(_190);
        float _192 = sin(_190);
        float2 _201 = _168.xy;
        float2 _204 = lerp(_201, mul(transpose(float2x2(float2(_191, _192), float2(-_192, _191))), _201), lerp(in_var_is_wing, 1.0f, 1.0f - (_176 * 2.0f)).xx);
        _206 = float3(_204.x, _204.y, _168.z);
    }
    else
    {
        _206 = _168;
    }
    float3 _219 = (mul(float3x3(cross(butterfly_buffer.Load<float3>(uint(gl_InstanceIndex) * 32 + 16), float3(0.0f, 1.0f, 0.0f)), float3(0.0f, 1.0f, 0.0f), butterfly_buffer.Load<float3>(uint(gl_InstanceIndex) * 32 + 16)), _206) + butterfly_buffer.Load<float3>(uint(gl_InstanceIndex) * 32 + 0)) + float3(0.0f, lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (butterfly_buffer.Load<float3>(uint(gl_InstanceIndex) * 32 + 0).xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x), 0.0f);
    float _427;
    if (_Globals_bool_render_shadows == 1)
    {
        float4 _230 = mul(frame_constants_view_constants[2u].view_to_world, float4(0.0f, 0.0f, 1.0f, 0.0f));
        float2 _425;
        do
        {
            float4 _237 = mul(frame_constants_view_constants[2u].world_to_sample, float4(_219, 1.0f));
            if (_237.z > 1.0f)
            {
                _425 = 0.0f.xx;
                break;
            }
            float2 _242 = _237.xy;
            float2 _243 = abs(_242);
            bool _245 = any(bool2(_243.x > 0.9833333492279052734375f.xx.x, _243.y > 0.9833333492279052734375f.xx.y));
            float2 _249;
            if (_245)
            {
                _249 = _242 * 0.5f;
            }
            else
            {
                _249 = _242;
            }
            float _250 = _245 ? 5.0f : 10.0f;
            float _252 = _245 ? 0.5f : 1.0f;
            float2 _253 = abs(_249);
            float2 _256 = 1.0f.xx - (0.000833333353511989116668701171875f.xx * (2.0f * _250));
            bool _258 = any(bool2(_253.x > _256.x, _253.y > _256.y));
            float _264;
            float2 _265;
            float _266;
            if (_258)
            {
                _264 = _250 * 0.2857142984867095947265625f;
                _265 = _249 * 0.2857142984867095947265625f;
                _266 = _252 * 0.2857142984867095947265625f;
            }
            else
            {
                _264 = _250;
                _265 = _249;
                _266 = _252;
            }
            float _269 = float(_258 ? 2u : uint(_245));
            uint _271 = 2u + uint(_269);
            float _275 = _230.y;
            float4 _287 = float4(_265.x, _265.y, _237.z, _237.w) + mul(frame_constants_view_constants[_271].world_to_sample, float4((float3(0.0f, 1.0f, 0.0f) * (0.066666670143604278564453125f * clamp(1.0f - _275, 0.0f, 1.0f))) / _266.xxx, 0.0f));
            float2 _290 = (_287.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
            float3 _291 = float3(_290.x, _290.y, _287.z);
            float _296 = max((0.300000011920928955078125f / _266) * (1.0f - abs(_275)), 0.02500000037252902984619140625f);
            float _302 = float((frame_constants_frame_index * 5u) % 64u) * 1.61803400516510009765625f;
            float _308 = frac(52.98291778564453125f * frac(float(frame_constants_frame_index % 64u) * 0.40764915943145751953125f));
            float4 _310 = mul(frame_constants_view_constants[_271].world_to_sample, float4(0.0f, 1.0f, 0.0f, 0.0f));
            float _312;
            int _315;
            _312 = 0.0f;
            _315 = 0;
            float _313;
            int _316;
            for (int _317 = 0; _317 < 5; _312 = _313, _315 = _316, _317++)
            {
                float _322 = float(_317);
                float _324 = (_322 + _302) * 2.3999631404876708984375f;
                float3 _337 = _291 + float3((((float2(cos(_324), sin(_324)) * (_322 + _308)) * 0.20000000298023223876953125f.xx) * _264) * 0.000833333353511989116668701171875f.xx, _296 * (-0.0009765625f));
                float4 _344 = shadow_map.SampleLevel(sampler_shadow_nearest, float3(_337.xy, _269), 0.0f);
                float _345 = _344.x;
                if (_337.z > _345)
                {
                    _313 = _312 + _345;
                    _316 = _315 + 1;
                }
                else
                {
                    _313 = _312;
                    _316 = _315;
                }
            }
            float _353 = _312 / float(_315);
            if (_315 == 0)
            {
                _425 = 0.0f.xx;
                break;
            }
            float _367 = max(((_287.z - _353) * ((17.0666675567626953125f * _264) * pow(10.0f, smoothstep(0.0f, 0.25f, frame_constants_fog_tint_thickness.w)))) / _353, 0.0f);
            float _374 = (_367 / pow(pow(_367 / (_264 - 1.0f), 5.0f) + 1.0f, 0.20000000298023223876953125f)) + 1.0f;
            float _376 = _310.z;
            float2 _386 = (((_310.xy * float(int(sign(_376)))) / max(0.1500000059604644775390625f, abs(_376)).xx) * 0.0009765625f.xx) * 2.0f;
            float2 _388;
            _388 = 0.0f.xx;
            for (int _391 = 0; _391 < 10; )
            {
                float _395 = float(_391);
                float _397 = (_395 + _302) * 2.3999631404876708984375f;
                float2 _404 = ((float2(cos(_397), sin(_397)) * (_395 + _308)) * 0.100000001490116119384765625f.xx) * _374;
                float3 _410 = _291 + float3(_404 * 0.000833333353511989116668701171875f.xx, _296 * (-0.0009765625f));
                _388 += float2(1.0f - shadow_map.SampleCmpLevelZero(sampler_shadow_linear, float3(_410.xy, _269), _410.z + min(0.0f, dot(_386, _404))), _148);
                _391++;
                continue;
            }
            _425 = _388 * 0.100000001490116119384765625f;
            break;
        } while(false);
        _427 = _425.x;
    }
    else
    {
        _427 = 0.0f;
    }
    _156.y = 1.0f - _156.y;
    float2 _434 = _156 * 0.5f;
    float _435 = _434.y;
    float2 _437 = _434;
    _437.y = _435 + 0.5f;
    uint _438 = uint(gl_InstanceIndex) % 8u;
    float2 _443;
    if (_438 == 3u)
    {
        float2 _442 = _437;
        _442.y = _435;
        _443 = _442;
    }
    else
    {
        _443 = _437;
    }
    float2 _450;
    if (_438 == 4u)
    {
        float2 _449 = _443;
        _449.x = _443.x - 0.5f;
        _450 = _449;
    }
    else
    {
        _450 = _443;
    }
    float2 _455;
    if (_438 == 5u)
    {
        _455 = _450 - 0.5f.xx;
    }
    else
    {
        _455 = _450;
    }
    gl_Position = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, float4(_219, 1.0f));
    out_var_C0 = ((frame_constants_sun_color.xyz * (1.0f - _427)) * 0.3183098733425140380859375f.xxx) + frame_constants_sky_high_color.xyz;
    out_var_C1 = _455;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_UV = stage_input.in_var_Vertex_UV;
    in_var_is_wing = stage_input.in_var_is_wing;
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
