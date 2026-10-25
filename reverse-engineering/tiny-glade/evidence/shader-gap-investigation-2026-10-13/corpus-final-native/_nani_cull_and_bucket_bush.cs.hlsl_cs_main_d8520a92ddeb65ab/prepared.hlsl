#define PERM_oc_pass 0
#define TGV_d8520a92ddeb65ab 1
#define TINYGLADE_NANI_NO_HZB 1
#line 1 "shaders/nani/cull_and_bucket_bush.cs.hlsl"




#if defined(TGV_8a7d2f81e72c8eeb) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)

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

struct MeshAtlasPartInfo
{
    uint first_index;
    uint index_count;
    float bounding_sphere_radius;
    uint pad0;
};

struct GpuFilteredInstanceMeta
{
    uint raw;
};

struct NaniDrawListInfo
{
    uint global_instance_offset;
};

struct Affine3Packed
{
    float data[12];
};

struct BushXform
{
    Affine3Packed transform;
};

static const GpuFilteredInstanceMeta _131 = { 0u };

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

ByteAddressBuffer atlas_part_info : register(t13, space0);
RWByteAddressBuffer __alias_filtered_instance_buffer : register(u14, space0);
RWByteAddressBuffer __alias_filtered_instance_allocator_buffer : register(u15, space0);
RWByteAddressBuffer __alias_filtered_instance_meta_buffer : register(u16, space0);
RWByteAddressBuffer __alias_output_histogram : register(u17, space0);
ByteAddressBuffer prev_visibility_bitmask : register(t18, space0);
RWByteAddressBuffer __alias_next_visibility_bitmask : register(u19, space0);
ByteAddressBuffer draw_list_info : register(t20, space0);
cbuffer $Globals : register(b22, space0)
{
    uint2 hzb_tex_inner_dims : packoffset(c0);
};

ByteAddressBuffer instance_buffer : register(t23, space0);
ByteAddressBuffer dispatch_args_buffer : register(t24, space0);
struct PushConstants
{
    uint world_subset;
    uint draw_list_index_base;
};
[[vk::push_constant]]
PushConstants push_constants;

Texture2D<float4> hzb_tex : register(t21, space0);

static uint3 gl_GlobalInvocationID;
struct SPIRV_Cross_Input
{
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
};

void comp_main()
{
    do
    {
        if (gl_GlobalInvocationID.x >= dispatch_args_buffer.Load<uint>(12))
        {
            break;
        }
        BushXform _146;
        [unroll]
        for (int _1ident = 0; _1ident < 12; _1ident++)
        {
            _146.transform.data[_1ident] = instance_buffer.Load<float>(_1ident * 4 + gl_GlobalInvocationID.x * 48 + 0);
        }
        float3x4 _166 = transpose(float4x3(float3(_146.transform.data[0], _146.transform.data[1], _146.transform.data[2]), float3(_146.transform.data[3], _146.transform.data[4], _146.transform.data[5]), float3(_146.transform.data[6], _146.transform.data[7], _146.transform.data[8]), float3(_146.transform.data[9], _146.transform.data[10], _146.transform.data[11])));
        float3 _169 = _166[0].xyz;
        float3 _172 = _166[1].xyz;
        float3 _175 = _166[2].xyz;
        float _184 = sqrt(max(max(dot(_169, _169), dot(_172, _172)), dot(_175, _175))) * atlas_part_info.Load<float>(push_constants.world_subset * 1024 + 8);
        float4 _191 = mul(frame_constants_view_constants[0u].world_to_view, float4(mul(_166, float4(0.0f, 0.0f, 0.0f, 1.0f)), 1.0f));
        float _192 = _191.z;
        uint _198 = clamp(uint(clamp((32.0f - _192) * 0.00347222224809229373931884765625f, 0.0f, 1.0f) * 1024.0f), 0u, 1023u);
        float4 _207 = float4(-frame_constants_view_constants[0u].view_to_clip[0].x, frame_constants_view_constants[0u].view_to_clip[0].y, 0.0f, frame_constants_view_constants[0u].view_to_clip[0].w);
        float4 _211 = float4(frame_constants_view_constants[0u].view_to_clip[1].x, frame_constants_view_constants[0u].view_to_clip[1].y, 0.0f, frame_constants_view_constants[0u].view_to_clip[1].w);
        float4 _215 = float4(frame_constants_view_constants[0u].view_to_clip[3].x, frame_constants_view_constants[0u].view_to_clip[3].y, -1.0f, frame_constants_view_constants[0u].view_to_clip[3].w);
        float4 _135[4];
        _135[0] = _215 + _207;
        _135[1] = _215 - _207;
        _135[2] = _215 + _211;
        _135[3] = _215 - _211;
        for (int _226 = 0; _226 < 4; )
        {
            _135[_226] /= length(_135[_226].xyz).xxxx;
            _226++;
            continue;
        }
        float4 _134[4] = _135;
        float3 _241 = _191.xyz;
        float _242 = -_192;
        bool _249;
        if ((_242 + _184) > frame_constants_view_constants[0u].view_to_clip[2].w)
        {
            _249 = (_242 - _184) < 3.4028234663852885981170418348452e+38f;
        }
        else
        {
            _249 = false;
        }
        bool _251;
        _251 = _249;
        bool _252;
        for (int _254 = 0; _254 < 4; _251 = _252, _254++)
        {
            if (_251)
            {
                _252 = dot(float4(_191.xy, _192, 1.0f), _134[_254]) > (-_184);
            }
            else
            {
                _252 = false;
            }
        }
        uint _270 = draw_list_info.Load<uint>(push_constants.draw_list_index_base * 4 + 0);
        uint _271 = gl_GlobalInvocationID.x + _270;
        _241.z = _242;
        float _274 = frame_constants_view_constants[0u].view_to_clip[0].x * 0.5f;
        float _275 = frame_constants_view_constants[0u].view_to_clip[1].y * 0.5f;
        float3 _276 = _241 * _184;
        float _279 = (_242 * _242) - (_184 * _184);
        float _280 = _191.x;
        float _283 = sqrt((_280 * _280) + _279);
        float _284 = _283 * _280;
        float _285 = _276.z;
        float _287 = _283 * _242;
        float _288 = _276.x;
        float _294 = _191.y;
        float _297 = sqrt((_294 * _294) + _279);
        float _298 = _297 * _294;
        float _300 = _297 * _242;
        float _301 = _276.y;
        float4 _318 = float4(hzb_tex_inner_dims.xyxy);
        float4 _321 = clamp((float4(((_284 - _285) / (_287 + _288)) * _274, (-((_298 + _285) / (_300 - _301))) * _275, ((_284 + _285) / (_287 - _288)) * _274, (-((_298 - _285) / (_300 + _301))) * _275) + 0.5f.xxxx) * _318, 0.0f.xxxx, _318 - 1.0f.xxxx);
        uint _332 = uint(max(0.0f, ceil(log2(max(_321.z - _321.x, _321.w - _321.y)))));
        bool _369;
        if (_251 ? true : false)
        {
            uint2 _339 = _332.xx & uint2(31u, 31u);
            uint2 _340 = uint2(_321.xy) >> _339;
            uint2 _343 = uint2(_321.zw) >> _339;
            _369 = (_192 + _184) > (1.0f / (min(min(hzb_tex.Load(int3(_340, _332)).x, hzb_tex.Load(int3(uint2(_340.x, _343.y), _332)).x), min(hzb_tex.Load(int3(uint2(_343.x, _340.y), _332)).x, hzb_tex.Load(int3(_343, _332)).x)) * (-frame_constants_view_constants[0u].clip_to_view[3].z)));
        }
        else
        {
            _369 = _251;
        }
        if (_369)
        {
            uint _376 = 1u << ((_271 % 32u) & 31u);
            uint _377;
            __alias_next_visibility_bitmask.InterlockedOr((_271 / 32u) * 4 + 0, _376, _377);
            if (!((prev_visibility_bitmask.Load<uint>((_271 / 32u) * 4 + 0) & _376) != 0u))
            {
                uint _386;
                __alias_filtered_instance_allocator_buffer.InterlockedAdd(push_constants.draw_list_index_base * 4 + 0, 1u, _386);
                __alias_filtered_instance_buffer.Store<uint>(_386 * 4 + 0, (gl_GlobalInvocationID.x * 1024u) + _198);
                __alias_filtered_instance_meta_buffer.Store<uint>(_386 * 4 + 0, _131.raw);
                uint _392;
                __alias_output_histogram.InterlockedAdd(((push_constants.draw_list_index_base * 1024u) + _198) * 4 + 0, 1u, _392);
            }
        }
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

#if defined(TGV_d8520a92ddeb65ab) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)

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

struct MeshAtlasPartInfo
{
    uint first_index;
    uint index_count;
    float bounding_sphere_radius;
    uint pad0;
};

struct GpuFilteredInstanceMeta
{
    uint raw;
};

struct NaniDrawListInfo
{
    uint global_instance_offset;
};

struct Affine3Packed
{
    float data[12];
};

struct BushXform
{
    Affine3Packed transform;
};

static const GpuFilteredInstanceMeta _130 = { 0u };

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

ByteAddressBuffer atlas_part_info : register(t13, space0);
RWByteAddressBuffer __alias_filtered_instance_buffer : register(u14, space0);
RWByteAddressBuffer __alias_filtered_instance_allocator_buffer : register(u15, space0);
RWByteAddressBuffer __alias_filtered_instance_meta_buffer : register(u16, space0);
RWByteAddressBuffer __alias_output_histogram : register(u17, space0);
ByteAddressBuffer prev_visibility_bitmask : register(t18, space0);
RWByteAddressBuffer __alias_next_visibility_bitmask : register(u19, space0);
ByteAddressBuffer draw_list_info : register(t20, space0);
ByteAddressBuffer instance_buffer : register(t23, space0);
ByteAddressBuffer dispatch_args_buffer : register(t24, space0);
struct PushConstants
{
    uint world_subset;
    uint draw_list_index_base;
};
[[vk::push_constant]]
PushConstants push_constants;


static uint3 gl_GlobalInvocationID;
struct SPIRV_Cross_Input
{
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
};

void comp_main()
{
    do
    {
        if (gl_GlobalInvocationID.x >= dispatch_args_buffer.Load<uint>(12))
        {
            break;
        }
        BushXform _146;
        [unroll]
        for (int _1ident = 0; _1ident < 12; _1ident++)
        {
            _146.transform.data[_1ident] = instance_buffer.Load<float>(_1ident * 4 + gl_GlobalInvocationID.x * 48 + 0);
        }
        float3x4 _166 = transpose(float4x3(float3(_146.transform.data[0], _146.transform.data[1], _146.transform.data[2]), float3(_146.transform.data[3], _146.transform.data[4], _146.transform.data[5]), float3(_146.transform.data[6], _146.transform.data[7], _146.transform.data[8]), float3(_146.transform.data[9], _146.transform.data[10], _146.transform.data[11])));
        float3 _169 = _166[0].xyz;
        float3 _172 = _166[1].xyz;
        float3 _175 = _166[2].xyz;
        float _183 = atlas_part_info.Load<float>(push_constants.world_subset * 1024 + 8);
        float _184 = sqrt(max(max(dot(_169, _169), dot(_172, _172)), dot(_175, _175))) * _183;
        float4 _190 = float4(mul(_166, float4(0.0f, 0.0f, 0.0f, 1.0f)), 1.0f);
        float4 _191 = mul(frame_constants_view_constants[0u].world_to_view, _190);
        float _192 = _191.z;
        float _193 = -_192;
        uint _213 = clamp(uint(clamp((32.0f - _192) * 0.00347222224809229373931884765625f, 0.0f, 1.0f) * 1024.0f), 0u, 1023u);
        uint _214 = gl_GlobalInvocationID.x * 1024u;
        float4 _220 = float4(-frame_constants_view_constants[0u].view_to_clip[0].x, frame_constants_view_constants[0u].view_to_clip[0].y, 0.0f, frame_constants_view_constants[0u].view_to_clip[0].w);
        float4 _223 = float4(frame_constants_view_constants[0u].view_to_clip[1].x, frame_constants_view_constants[0u].view_to_clip[1].y, 0.0f, frame_constants_view_constants[0u].view_to_clip[1].w);
        float4 _227 = float4(frame_constants_view_constants[0u].view_to_clip[3].x, frame_constants_view_constants[0u].view_to_clip[3].y, -1.0f, frame_constants_view_constants[0u].view_to_clip[3].w);
        float4 _135[4];
        _135[0] = _227 + _220;
        _135[1] = _227 - _220;
        _135[2] = _227 + _223;
        _135[3] = _227 - _223;
        for (int _238 = 0; _238 < 4; )
        {
            _135[_238] /= length(_135[_238].xyz).xxxx;
            _238++;
            continue;
        }
        float4 _134[4] = _135;
        bool _259;
        if ((_193 + _184) > frame_constants_view_constants[0u].view_to_clip[2].w)
        {
            _259 = (_193 - _184) < 3.4028234663852885981170418348452e+38f;
        }
        else
        {
            _259 = false;
        }
        bool _261;
        _261 = _259;
        bool _262;
        for (int _264 = 0; _264 < 4; _261 = _262, _264++)
        {
            if (_261)
            {
                _262 = dot(float4(_191.xy, _192, 1.0f), _134[_264]) > (-_184);
            }
            else
            {
                _262 = false;
            }
        }
        uint _281 = gl_GlobalInvocationID.x + draw_list_info.Load<uint>(push_constants.draw_list_index_base * 4 + 0);
        bool _292;
        if (_261)
        {
            _292 = (prev_visibility_bitmask.Load<uint>((_281 / 32u) * 4 + 0) & (1u << ((_281 % 32u) & 31u))) != 0u;
        }
        else
        {
            _292 = false;
        }
        if (_292)
        {
            uint _300;
            __alias_next_visibility_bitmask.InterlockedOr((_281 / 32u) * 4 + 0, 1u << ((_281 % 32u) & 31u), _300);
            uint _302;
            __alias_filtered_instance_allocator_buffer.InterlockedAdd(push_constants.draw_list_index_base * 4 + 0, 1u, _302);
            __alias_filtered_instance_buffer.Store<uint>(_302 * 4 + 0, _214 + _213);
            __alias_filtered_instance_meta_buffer.Store<uint>(_302 * 4 + 0, _130.raw);
            uint _308;
            __alias_output_histogram.InterlockedAdd(((push_constants.draw_list_index_base * 1024u) + _213) * 4 + 0, 1u, _308);
        }
        if ((((_184 / max(9.9999997473787516355514526367188e-06f, _193)) * ((0.5f * frame_constants_view_constants[0u].view_to_clip[1].y) * float(clamp(frame_constants_inner_rendering_resolution.y, 1080u, 2160u)))) < 10.0f) ? false : true)
        {
            float4 _316 = mul(frame_constants_view_constants[2u].view_to_clip, mul(frame_constants_view_constants[2u].world_to_view, _190));
            uint _324 = clamp(uint((_316.xyz / _316.w.xxx).z * 1024.0f), 0u, 1023u);
            uint _325 = _214 + _324;
            float _334 = _184 * (2.0f * frame_constants_view_constants[2u].view_to_sample[0].x);
            float2 _335 = abs(mul(frame_constants_view_constants[2u].world_to_sample, _190).xy);
            float2 _337 = (-_334).xx;
            float2 _341 = (1.0f + _334).xx;
            uint _345 = (any(bool2(_335.x >= _337.x, _335.y >= _337.y)) ? all(bool2(_335.x <= _341.x, _335.y <= _341.y)) : false) ? 0u : 3u;
            float2 _347 = (1.0f - _334).xx;
            float2 _351 = (2.0f + _334).xx;
            bool _354 = any(bool2(_335.x >= _347.x, _335.y >= _347.y)) ? all(bool2(_335.x <= _351.x, _335.y <= _351.y)) : false;
            uint _358;
            if (_354)
            {
                _358 = min(_345, 1u);
            }
            else
            {
                _358 = _345;
            }
            uint _359 = uint(_354);
            float2 _361 = (2.0f - _334).xx;
            float2 _365 = (7.0f + _334).xx;
            uint _373;
            uint _374;
            if (any(bool2(_335.x >= _361.x, _335.y >= _361.y)) ? all(bool2(_335.x <= _365.x, _335.y <= _365.y)) : false)
            {
                _373 = max(_359, 2u);
                _374 = min(_358, 2u);
            }
            else
            {
                _373 = _359;
                _374 = _358;
            }
            for (uint _376 = _374; _376 <= _373; )
            {
                uint _384;
                __alias_filtered_instance_allocator_buffer.InterlockedAdd(((push_constants.draw_list_index_base + 1u) + _376) * 4 + 0, 1u, _384);
                __alias_filtered_instance_buffer.Store<uint>(_384 * 4 + 0, _325);
                __alias_filtered_instance_meta_buffer.Store<uint>(_384 * 4 + 0, _130.raw);
                uint _390;
                __alias_output_histogram.InterlockedAdd(((((push_constants.draw_list_index_base + 1u) + _376) * 1024u) + _324) * 4 + 0, 1u, _390);
                _376++;
                continue;
            }
        }
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
