#define PERM_oc_pass 0
#define TGV_d8520a92ddeb65ab 1
#define TINYGLADE_NANI_NO_HZB 1
#line 1 "shaders/nani/cull_and_bucket_plaster.cs.hlsl"




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

struct GpuFilteredInstanceMeta
{
    uint raw;
};

struct NaniDrawListInfo
{
    uint global_instance_offset;
};

struct PlasterInstanceData
{
    float4 bounding_sphere;
    uint wall_id;
    float wall_length;
    uint blend_uv_seam;
    uint quad_count;
    uint2 verts_gpu_address;
    int row_stride;
    float peel_strength;
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
struct PushConstants
{
    uint2 instance_buffer_addr;
    uint world_subset;
    uint draw_list_index_base;
    uint input_instance_count;
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
        if (gl_GlobalInvocationID.x >= push_constants.input_instance_count)
        {
            break;
        }
        uint2 _148 = push_constants.instance_buffer_addr;
        uint2 _146;
        _146.x = _148.x + (16u * (gl_GlobalInvocationID.x));
        _146.y = _148.y + uint(int(_146.x < _148.x));
        uint4 _178 = vk::RawBufferLoad<uint4>(((uint64_t)_146.y << 32) | _146.x, 16);
        uint4 _147 = _178;
        float4 _183 = instance_buffer.Load<float4>(gl_GlobalInvocationID.x * 48 + 0);
        float4 _129 = _183;
        float4 _192 = mul(frame_constants_view_constants[0u].world_to_view, float4(_183.xyz, 1.0f));
        uint _199 = clamp(uint(clamp((32.0f - _192.z) * 0.00347222224809229373931884765625f, 0.0f, 1.0f) * 1024.0f), 0u, 1023u);
        float4x4 _138 = frame_constants_view_constants[0u].view_to_clip;
        float2 _136 = 1.0f.xx;
        float2 _137 = 0.0f.xx;
        float4 _216 = float4((-_138[0u].x) * _136.x, _138[0u].y, _137.x, _138[0u].w);
        float4 _228 = float4(_138[1u].x, _138[1u].y * _136.y, _137.y, _138[1u].w);
        float4 _235 = float4(_138[3u].x, _138[3u].y, -1.0f, _138[3u].w);
        float4 _132[4];
        _132[0] = _235 + _216;
        _132[1] = _235 - _216;
        _132[2] = _235 + _228;
        _132[3] = _235 - _228;
        for (int _247 = 0; _247 < 4; )
        {
            _132[_247] /= length(_132[_247].xyz).xxxx;
            _247++;
            continue;
        }
        float4 _131[4] = _132;
        float3 _262 = _192.xyz;
        float3 _139 = _262;
        float3 _140 = _262;
        bool _274;
        if (((-_140.z) + _129.w) > _138[2].w)
        {
            _274 = ((-_140.z) - _129.w) < 3.4028234663852885981170418348452e+38f;
        }
        else
        {
            _274 = false;
        }
        bool _276;
        _276 = _274;
        bool _277;
        for (int _279 = 0; _279 < 4; _276 = _277, _279++)
        {
            if (_276)
            {
                _277 = dot(float4(_192.xyz, 1.0f), _131[_279]) > (-_129.w);
            }
            else
            {
                _277 = false;
            }
        }
        uint _295 = draw_list_info.Load<uint>(push_constants.draw_list_index_base * 4 + 0);
        uint _296 = gl_GlobalInvocationID.x + _295;
        float3 _142 = _262;
        _142.z = -_142.z;
        float4x4 _134 = frame_constants_view_constants[0u].view_to_clip;
        float _305 = _134[0u].x * 0.5f;
        float4x4 _133 = frame_constants_view_constants[0u].view_to_clip;
        float _308 = _133[1u].y * 0.5f;
        float3 _135 = _142 * _129.w;
        float _315 = (_142.z * _142.z) - (_129.w * _129.w);
        float _321 = sqrt((_142.x * _142.x) + _315);
        float _347 = sqrt((_142.y * _142.y) + _315);
        float4 _378 = float4(hzb_tex_inner_dims.xyxy);
        float4 _381 = clamp((float4((((_321 * _142.x) - _135.z) / ((_321 * _142.z) + _135.x)) * _305, (-(((_347 * _142.y) + _135.z) / ((_347 * _142.z) - _135.y))) * _308, (((_321 * _142.x) + _135.z) / ((_321 * _142.z) - _135.x)) * _305, (-(((_347 * _142.y) - _135.z) / ((_347 * _142.z) + _135.y))) * _308) + 0.5f.xxxx) * _378, 0.0f.xxxx, _378 - 1.0f.xxxx);
        float4 _141 = _381;
        float2 _143 = float2(_141.z - _141.x, _141.w - _141.y);
        uint _401 = uint(max(0.0f, ceil(log2(max(_143.x, _143.y)))));
        bool _449;
        if (_276 ? true : false)
        {
            uint2 _408 = _401.xx & uint2(31u, 31u);
            uint2 _144 = uint2(_381.xy) >> _408;
            uint2 _145 = uint2(_381.zw) >> _408;
            float4x4 _130 = frame_constants_view_constants[0u].clip_to_view;
            _449 = (_139.z + _129.w) > (1.0f / (min(min(hzb_tex.Load(int3(uint2(_144.x, _144.y), _401)).x, hzb_tex.Load(int3(uint2(_144.x, _145.y), _401)).x), min(hzb_tex.Load(int3(uint2(_145.x, _144.y), _401)).x, hzb_tex.Load(int3(uint2(_145.x, _145.y), _401)).x)) * (-_130[3].z)));
        }
        else
        {
            _449 = _276;
        }
        if (_449)
        {
            uint _456 = 1u << ((_296 % 32u) & 31u);
            uint _457;
            __alias_next_visibility_bitmask.InterlockedOr((_296 / 32u) * 4 + 0, _456, _457);
            if (!((prev_visibility_bitmask.Load<uint>((_296 / 32u) * 4 + 0) & _456) != 0u))
            {
                uint _466;
                __alias_filtered_instance_allocator_buffer.InterlockedAdd(push_constants.draw_list_index_base * 4 + 0, 1u, _466);
                __alias_filtered_instance_buffer.Store<uint>(_466 * 4 + 0, (gl_GlobalInvocationID.x * 1024u) + _199);
                GpuFilteredInstanceMeta _469 = { _147.w >> 16u };
                __alias_filtered_instance_meta_buffer.Store<uint>(_466 * 4 + 0, _469.raw);
                uint _473;
                __alias_output_histogram.InterlockedAdd(((push_constants.draw_list_index_base * 1024u) + _199) * 4 + 0, 1u, _473);
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

struct GpuFilteredInstanceMeta
{
    uint raw;
};

struct NaniDrawListInfo
{
    uint global_instance_offset;
};

struct PlasterInstanceData
{
    float4 bounding_sphere;
    uint wall_id;
    float wall_length;
    uint blend_uv_seam;
    uint quad_count;
    uint2 verts_gpu_address;
    int row_stride;
    float peel_strength;
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

RWByteAddressBuffer __alias_filtered_instance_buffer : register(u14, space0);
RWByteAddressBuffer __alias_filtered_instance_allocator_buffer : register(u15, space0);
RWByteAddressBuffer __alias_filtered_instance_meta_buffer : register(u16, space0);
RWByteAddressBuffer __alias_output_histogram : register(u17, space0);
ByteAddressBuffer prev_visibility_bitmask : register(t18, space0);
RWByteAddressBuffer __alias_next_visibility_bitmask : register(u19, space0);
ByteAddressBuffer draw_list_info : register(t20, space0);
ByteAddressBuffer instance_buffer : register(t23, space0);
struct PushConstants
{
    uint2 instance_buffer_addr;
    uint world_subset;
    uint draw_list_index_base;
    uint input_instance_count;
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
        if (gl_GlobalInvocationID.x >= push_constants.input_instance_count)
        {
            break;
        }
        uint2 _125 = push_constants.instance_buffer_addr;
        uint2 _123;
        _123.x = _125.x + (16u * (gl_GlobalInvocationID.x));
        _123.y = _125.y + uint(int(_123.x < _125.x));
        uint4 _155 = vk::RawBufferLoad<uint4>(((uint64_t)_123.y << 32) | _123.x, 16);
        uint4 _124 = _155;
        float4 _160 = instance_buffer.Load<float4>(gl_GlobalInvocationID.x * 48 + 0);
        float4 _116 = _160;
        float4 _169 = mul(frame_constants_view_constants[0u].world_to_view, float4(_160.xyz, 1.0f));
        uint _176 = clamp(uint(clamp((32.0f - _169.z) * 0.00347222224809229373931884765625f, 0.0f, 1.0f) * 1024.0f), 0u, 1023u);
        float4x4 _121 = frame_constants_view_constants[0u].view_to_clip;
        float2 _119 = 1.0f.xx;
        float2 _120 = 0.0f.xx;
        float4 _193 = float4((-_121[0u].x) * _119.x, _121[0u].y, _120.x, _121[0u].w);
        float4 _205 = float4(_121[1u].x, _121[1u].y * _119.y, _120.y, _121[1u].w);
        float4 _212 = float4(_121[3u].x, _121[3u].y, -1.0f, _121[3u].w);
        float4 _118[4];
        _118[0] = _212 + _193;
        _118[1] = _212 - _193;
        _118[2] = _212 + _205;
        _118[3] = _212 - _205;
        for (int _224 = 0; _224 < 4; )
        {
            _118[_224] /= length(_118[_224].xyz).xxxx;
            _224++;
            continue;
        }
        float4 _117[4] = _118;
        float3 _122 = _169.xyz;
        bool _251;
        if (((-_122.z) + _116.w) > _121[2].w)
        {
            _251 = ((-_122.z) - _116.w) < 3.4028234663852885981170418348452e+38f;
        }
        else
        {
            _251 = false;
        }
        bool _253;
        _253 = _251;
        bool _254;
        for (int _256 = 0; _256 < 4; _253 = _254, _256++)
        {
            if (_253)
            {
                _254 = dot(float4(_169.xyz, 1.0f), _117[_256]) > (-_116.w);
            }
            else
            {
                _254 = false;
            }
        }
        uint _273 = gl_GlobalInvocationID.x + draw_list_info.Load<uint>(push_constants.draw_list_index_base * 4 + 0);
        bool _284;
        if (_253)
        {
            _284 = (prev_visibility_bitmask.Load<uint>((_273 / 32u) * 4 + 0) & (1u << ((_273 % 32u) & 31u))) != 0u;
        }
        else
        {
            _284 = false;
        }
        if (_284)
        {
            uint _292;
            __alias_next_visibility_bitmask.InterlockedOr((_273 / 32u) * 4 + 0, 1u << ((_273 % 32u) & 31u), _292);
            uint _294;
            __alias_filtered_instance_allocator_buffer.InterlockedAdd(push_constants.draw_list_index_base * 4 + 0, 1u, _294);
            __alias_filtered_instance_buffer.Store<uint>(_294 * 4 + 0, (gl_GlobalInvocationID.x * 1024u) + _176);
            GpuFilteredInstanceMeta _297 = { _124.w >> 16u };
            __alias_filtered_instance_meta_buffer.Store<uint>(_294 * 4 + 0, _297.raw);
            uint _301;
            __alias_output_histogram.InterlockedAdd(((push_constants.draw_list_index_base * 1024u) + _176) * 4 + 0, 1u, _301);
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
