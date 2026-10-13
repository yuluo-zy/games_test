#define PERM_oc_pass 0
#define TGV_d8520a92ddeb65ab 1
#define TINYGLADE_NANI_NO_HZB 1
#line 1 "shaders/nani/cull_and_bucket_flowers.cs.hlsl"




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

struct FlowerXform
{
    Affine3Packed transform;
    float3 color;
    uint flower_type;
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
    uint2 _Globals_hzb_tex_inner_dims : packoffset(c0);
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
        Affine3Packed _146;
        [unroll]
        for (int _1ident = 0; _1ident < 12; _1ident++)
        {
            _146.data[_1ident] = instance_buffer.Load<float>(_1ident * 4 + gl_GlobalInvocationID.x * 64 + 0);
        }
        uint _148 = instance_buffer.Load<uint>(gl_GlobalInvocationID.x * 64 + 60);
        float3x4 _167 = transpose(float4x3(float3(_146.data[0], _146.data[1], _146.data[2]), float3(_146.data[3], _146.data[4], _146.data[5]), float3(_146.data[6], _146.data[7], _146.data[8]), float3(_146.data[9], _146.data[10], _146.data[11])));
        float3 _170 = _167[0].xyz;
        float3 _173 = _167[1].xyz;
        float3 _176 = _167[2].xyz;
        float _185 = sqrt(max(max(dot(_170, _170), dot(_173, _173)), dot(_176, _176))) * atlas_part_info.Load<float>(push_constants.world_subset * 1024 + _148 * 16 + 8);
        float4 _192 = mul(frame_constants_view_constants[0u].world_to_view, float4(mul(_167, float4(0.0f, 0.0f, 0.0f, 1.0f)), 1.0f));
        float _193 = _192.z;
        uint _199 = clamp(uint(clamp((32.0f - _193) * 0.00347222224809229373931884765625f, 0.0f, 1.0f) * 1024.0f), 0u, 1023u);
        float4 _208 = float4(-frame_constants_view_constants[0u].view_to_clip[0].x, frame_constants_view_constants[0u].view_to_clip[0].y, 0.0f, frame_constants_view_constants[0u].view_to_clip[0].w);
        float4 _212 = float4(frame_constants_view_constants[0u].view_to_clip[1].x, frame_constants_view_constants[0u].view_to_clip[1].y, 0.0f, frame_constants_view_constants[0u].view_to_clip[1].w);
        float4 _216 = float4(frame_constants_view_constants[0u].view_to_clip[3].x, frame_constants_view_constants[0u].view_to_clip[3].y, -1.0f, frame_constants_view_constants[0u].view_to_clip[3].w);
        float4 _135[4];
        _135[0] = _216 + _208;
        _135[1] = _216 - _208;
        _135[2] = _216 + _212;
        _135[3] = _216 - _212;
        for (int _227 = 0; _227 < 4; )
        {
            _135[_227] /= length(_135[_227].xyz).xxxx;
            _227++;
            continue;
        }
        float4 _134[4] = _135;
        float3 _242 = _192.xyz;
        float _243 = -_193;
        bool _250;
        if ((_243 + _185) > frame_constants_view_constants[0u].view_to_clip[2].w)
        {
            _250 = (_243 - _185) < 3.4028234663852885981170418348452e+38f;
        }
        else
        {
            _250 = false;
        }
        bool _252;
        _252 = _250;
        bool _253;
        for (int _255 = 0; _255 < 4; _252 = _253, _255++)
        {
            if (_252)
            {
                _253 = dot(float4(_192.xy, _193, 1.0f), _134[_255]) > (-_185);
            }
            else
            {
                _253 = false;
            }
        }
        uint _271 = draw_list_info.Load<uint>(push_constants.draw_list_index_base * 4 + 0);
        uint _272 = gl_GlobalInvocationID.x + _271;
        _242.z = _243;
        float _275 = frame_constants_view_constants[0u].view_to_clip[0].x * 0.5f;
        float _276 = frame_constants_view_constants[0u].view_to_clip[1].y * 0.5f;
        float3 _277 = _242 * _185;
        float _280 = (_243 * _243) - (_185 * _185);
        float _281 = _192.x;
        float _284 = sqrt((_281 * _281) + _280);
        float _285 = _284 * _281;
        float _286 = _277.z;
        float _288 = _284 * _243;
        float _289 = _277.x;
        float _295 = _192.y;
        float _298 = sqrt((_295 * _295) + _280);
        float _299 = _298 * _295;
        float _301 = _298 * _243;
        float _302 = _277.y;
        float4 _319 = float4(_Globals_hzb_tex_inner_dims.xyxy);
        float4 _322 = clamp((float4(((_285 - _286) / (_288 + _289)) * _275, (-((_299 + _286) / (_301 - _302))) * _276, ((_285 + _286) / (_288 - _289)) * _275, (-((_299 - _286) / (_301 + _302))) * _276) + 0.5f.xxxx) * _319, 0.0f.xxxx, _319 - 1.0f.xxxx);
        uint _333 = uint(max(0.0f, ceil(log2(max(_322.z - _322.x, _322.w - _322.y)))));
        bool _370;
        if (_252 ? true : false)
        {
            uint2 _340 = _333.xx & uint2(31u, 31u);
            uint2 _341 = uint2(_322.xy) >> _340;
            uint2 _344 = uint2(_322.zw) >> _340;
            _370 = (_193 + _185) > (1.0f / (min(min(hzb_tex.Load(int3(_341, _333)).x, hzb_tex.Load(int3(uint2(_341.x, _344.y), _333)).x), min(hzb_tex.Load(int3(uint2(_344.x, _341.y), _333)).x, hzb_tex.Load(int3(_344, _333)).x)) * (-frame_constants_view_constants[0u].clip_to_view[3].z)));
        }
        else
        {
            _370 = _252;
        }
        if (_370)
        {
            uint _377 = 1u << ((_272 % 32u) & 31u);
            uint _378;
            __alias_next_visibility_bitmask.InterlockedOr((_272 / 32u) * 4 + 0, _377, _378);
            if (!((prev_visibility_bitmask.Load<uint>((_272 / 32u) * 4 + 0) & _377) != 0u))
            {
                uint _387;
                __alias_filtered_instance_allocator_buffer.InterlockedAdd(push_constants.draw_list_index_base * 4 + 0, 1u, _387);
                __alias_filtered_instance_buffer.Store<uint>(_387 * 4 + 0, (gl_GlobalInvocationID.x * 1024u) + _199);
                GpuFilteredInstanceMeta _390 = { _148 };
                __alias_filtered_instance_meta_buffer.Store<uint>(_387 * 4 + 0, _390.raw);
                uint _394;
                __alias_output_histogram.InterlockedAdd(((push_constants.draw_list_index_base * 1024u) + _199) * 4 + 0, 1u, _394);
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

struct FlowerXform
{
    Affine3Packed transform;
    float3 color;
    uint flower_type;
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
        Affine3Packed _145;
        [unroll]
        for (int _1ident = 0; _1ident < 12; _1ident++)
        {
            _145.data[_1ident] = instance_buffer.Load<float>(_1ident * 4 + gl_GlobalInvocationID.x * 64 + 0);
        }
        uint _147 = instance_buffer.Load<uint>(gl_GlobalInvocationID.x * 64 + 60);
        float3x4 _166 = transpose(float4x3(float3(_145.data[0], _145.data[1], _145.data[2]), float3(_145.data[3], _145.data[4], _145.data[5]), float3(_145.data[6], _145.data[7], _145.data[8]), float3(_145.data[9], _145.data[10], _145.data[11])));
        float3 _169 = _166[0].xyz;
        float3 _172 = _166[1].xyz;
        float3 _175 = _166[2].xyz;
        float _183 = atlas_part_info.Load<float>(push_constants.world_subset * 1024 + _147 * 16 + 8);
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
        float4 _134[4];
        _134[0] = _227 + _220;
        _134[1] = _227 - _220;
        _134[2] = _227 + _223;
        _134[3] = _227 - _223;
        for (int _238 = 0; _238 < 4; )
        {
            _134[_238] /= length(_134[_238].xyz).xxxx;
            _238++;
            continue;
        }
        float4 _133[4] = _134;
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
                _262 = dot(float4(_191.xy, _192, 1.0f), _133[_264]) > (-_184);
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
            GpuFilteredInstanceMeta _305 = { _147 };
            __alias_filtered_instance_meta_buffer.Store<uint>(_302 * 4 + 0, _305.raw);
            uint _309;
            __alias_output_histogram.InterlockedAdd(((push_constants.draw_list_index_base * 1024u) + _213) * 4 + 0, 1u, _309);
        }
        if ((((_184 / max(9.9999997473787516355514526367188e-06f, _193)) * ((0.5f * frame_constants_view_constants[0u].view_to_clip[1].y) * float(clamp(frame_constants_inner_rendering_resolution.y, 1080u, 2160u)))) < 10.0f) ? false : true)
        {
            float4 _317 = mul(frame_constants_view_constants[2u].view_to_clip, mul(frame_constants_view_constants[2u].world_to_view, _190));
            uint _325 = clamp(uint((_317.xyz / _317.w.xxx).z * 1024.0f), 0u, 1023u);
            uint _326 = _214 + _325;
            float _335 = _184 * (2.0f * frame_constants_view_constants[2u].view_to_sample[0].x);
            float2 _336 = abs(mul(frame_constants_view_constants[2u].world_to_sample, _190).xy);
            float2 _338 = (-_335).xx;
            float2 _342 = (1.0f + _335).xx;
            uint _346 = (any(bool2(_336.x >= _338.x, _336.y >= _338.y)) ? all(bool2(_336.x <= _342.x, _336.y <= _342.y)) : false) ? 0u : 3u;
            float2 _348 = (1.0f - _335).xx;
            float2 _352 = (2.0f + _335).xx;
            bool _355 = any(bool2(_336.x >= _348.x, _336.y >= _348.y)) ? all(bool2(_336.x <= _352.x, _336.y <= _352.y)) : false;
            uint _359;
            if (_355)
            {
                _359 = min(_346, 1u);
            }
            else
            {
                _359 = _346;
            }
            uint _360 = uint(_355);
            float2 _362 = (2.0f - _335).xx;
            float2 _366 = (7.0f + _335).xx;
            uint _374;
            uint _375;
            if (any(bool2(_336.x >= _362.x, _336.y >= _362.y)) ? all(bool2(_336.x <= _366.x, _336.y <= _366.y)) : false)
            {
                _374 = max(_360, 2u);
                _375 = min(_359, 2u);
            }
            else
            {
                _374 = _360;
                _375 = _359;
            }
            for (uint _377 = _375; _377 <= _374; )
            {
                uint _385;
                __alias_filtered_instance_allocator_buffer.InterlockedAdd(((push_constants.draw_list_index_base + 1u) + _377) * 4 + 0, 1u, _385);
                __alias_filtered_instance_buffer.Store<uint>(_385 * 4 + 0, _326);
                GpuFilteredInstanceMeta _388 = { _147 };
                __alias_filtered_instance_meta_buffer.Store<uint>(_385 * 4 + 0, _388.raw);
                uint _392;
                __alias_output_histogram.InterlockedAdd(((((push_constants.draw_list_index_base + 1u) + _377) * 1024u) + _325) * 4 + 0, 1u, _392);
                _377++;
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
