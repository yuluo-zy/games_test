#define PERM_oc_pass 1
#define TGV_8a7d2f81e72c8eeb 1
#line 1 "shaders/nani/cull_and_bucket_curved_halftimber.cs.hlsl"




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

struct CurvedHalftimberInstanceData
{
    float3 position;
    uint color_id;
    float radius0;
    float radius1;
    float angle0;
    float angle1;
    int is_straight;
    int is_xz;
    float plank_depth;
    int caps;
    int mesh_atlas_part_idx;
    int pad0;
    int pad1;
    int pad2;
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
    uint2 _Globals_hzb_tex_inner_dims : packoffset(c0);
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
        float3 _155 = instance_buffer.Load<float3>(gl_GlobalInvocationID.x * 64 + 0);
        int _163 = instance_buffer.Load<int>(gl_GlobalInvocationID.x * 64 + 48);
        float _166 = (instance_buffer.Load<int>(gl_GlobalInvocationID.x * 64 + 32) > 0) ? instance_buffer.Load<float>(gl_GlobalInvocationID.x * 64 + 28) : instance_buffer.Load<float>(gl_GlobalInvocationID.x * 64 + 20);
        float4 _173 = mul(frame_constants_view_constants[0u].world_to_view, float4(_155, 1.0f));
        uint _180 = clamp(uint(clamp((32.0f - _173.z) * 0.00347222224809229373931884765625f, 0.0f, 1.0f) * 1024.0f), 0u, 1023u);
        float4x4 _137 = frame_constants_view_constants[0u].view_to_clip;
        float2 _135 = 1.0f.xx;
        float2 _136 = 0.0f.xx;
        float4 _197 = float4((-_137[0u].x) * _135.x, _137[0u].y, _136.x, _137[0u].w);
        float4 _209 = float4(_137[1u].x, _137[1u].y * _135.y, _136.y, _137[1u].w);
        float4 _216 = float4(_137[3u].x, _137[3u].y, -1.0f, _137[3u].w);
        float4 _131[4];
        _131[0] = _216 + _197;
        _131[1] = _216 - _197;
        _131[2] = _216 + _209;
        _131[3] = _216 - _209;
        for (int _228 = 0; _228 < 4; )
        {
            _131[_228] /= length(_131[_228].xyz).xxxx;
            _228++;
            continue;
        }
        float4 _130[4] = _131;
        float3 _243 = _173.xyz;
        float3 _138 = _243;
        float3 _139 = _243;
        bool _255;
        if (((-_139.z) + _166) > _137[2].w)
        {
            _255 = ((-_139.z) - _166) < 3.4028234663852885981170418348452e+38f;
        }
        else
        {
            _255 = false;
        }
        bool _257;
        _257 = _255;
        bool _258;
        for (int _260 = 0; _260 < 4; _257 = _258, _260++)
        {
            if (_257)
            {
                _258 = dot(float4(_173.xyz, 1.0f), _130[_260]) > (-_166);
            }
            else
            {
                _258 = false;
            }
        }
        uint _276 = draw_list_info.Load<uint>(push_constants.draw_list_index_base * 4 + 0);
        uint _277 = gl_GlobalInvocationID.x + _276;
        float3 _141 = _243;
        _141.z = -_141.z;
        float4x4 _133 = frame_constants_view_constants[0u].view_to_clip;
        float _286 = _133[0u].x * 0.5f;
        float4x4 _132 = frame_constants_view_constants[0u].view_to_clip;
        float _289 = _132[1u].y * 0.5f;
        float3 _134 = _141 * _166;
        float _296 = (_141.z * _141.z) - (_166 * _166);
        float _302 = sqrt((_141.x * _141.x) + _296);
        float _328 = sqrt((_141.y * _141.y) + _296);
        float4 _359 = float4(_Globals_hzb_tex_inner_dims.xyxy);
        float4 _362 = clamp((float4((((_302 * _141.x) - _134.z) / ((_302 * _141.z) + _134.x)) * _286, (-(((_328 * _141.y) + _134.z) / ((_328 * _141.z) - _134.y))) * _289, (((_302 * _141.x) + _134.z) / ((_302 * _141.z) - _134.x)) * _286, (-(((_328 * _141.y) - _134.z) / ((_328 * _141.z) + _134.y))) * _289) + 0.5f.xxxx) * _359, 0.0f.xxxx, _359 - 1.0f.xxxx);
        float4 _140 = _362;
        float2 _142 = float2(_140.z - _140.x, _140.w - _140.y);
        uint _382 = uint(max(0.0f, ceil(log2(max(_142.x, _142.y)))));
        bool _430;
        if (_257 ? true : false)
        {
            uint2 _389 = _382.xx & uint2(31u, 31u);
            uint2 _143 = uint2(_362.xy) >> _389;
            uint2 _144 = uint2(_362.zw) >> _389;
            float4x4 _129 = frame_constants_view_constants[0u].clip_to_view;
            _430 = (_138.z + _166) > (1.0f / (min(min(hzb_tex.Load(int3(uint2(_143.x, _143.y), _382)).x, hzb_tex.Load(int3(uint2(_143.x, _144.y), _382)).x), min(hzb_tex.Load(int3(uint2(_144.x, _143.y), _382)).x, hzb_tex.Load(int3(uint2(_144.x, _144.y), _382)).x)) * (-_129[3].z)));
        }
        else
        {
            _430 = _257;
        }
        if (_430)
        {
            uint _437 = 1u << ((_277 % 32u) & 31u);
            uint _438;
            __alias_next_visibility_bitmask.InterlockedOr((_277 / 32u) * 4 + 0, _437, _438);
            if (!((prev_visibility_bitmask.Load<uint>((_277 / 32u) * 4 + 0) & _437) != 0u))
            {
                uint _447;
                __alias_filtered_instance_allocator_buffer.InterlockedAdd(push_constants.draw_list_index_base * 4 + 0, 1u, _447);
                __alias_filtered_instance_buffer.Store<uint>(_447 * 4 + 0, (gl_GlobalInvocationID.x * 1024u) + _180);
                GpuFilteredInstanceMeta _450 = { uint(_163) };
                __alias_filtered_instance_meta_buffer.Store<uint>(_447 * 4 + 0, _450.raw);
                uint _454;
                __alias_output_histogram.InterlockedAdd(((push_constants.draw_list_index_base * 1024u) + _180) * 4 + 0, 1u, _454);
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

struct CurvedHalftimberInstanceData
{
    float3 position;
    uint color_id;
    float radius0;
    float radius1;
    float angle0;
    float angle1;
    int is_straight;
    int is_xz;
    float plank_depth;
    int caps;
    int mesh_atlas_part_idx;
    int pad0;
    int pad1;
    int pad2;
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
        float3 _142 = instance_buffer.Load<float3>(gl_GlobalInvocationID.x * 64 + 0);
        float _144 = instance_buffer.Load<float>(gl_GlobalInvocationID.x * 64 + 20);
        float _146 = instance_buffer.Load<float>(gl_GlobalInvocationID.x * 64 + 28);
        int _148 = instance_buffer.Load<int>(gl_GlobalInvocationID.x * 64 + 32);
        int _150 = instance_buffer.Load<int>(gl_GlobalInvocationID.x * 64 + 48);
        uint _151 = uint(_150);
        float _153 = (_148 > 0) ? _146 : _144;
        float4 _159 = float4(_142, 1.0f);
        float4 _160 = mul(frame_constants_view_constants[0u].world_to_view, _159);
        float _161 = _160.z;
        float4x4 _125 = frame_constants_view_constants[0u].view_to_clip;
        uint _183 = clamp(uint(clamp((32.0f - _161) * 0.00347222224809229373931884765625f, 0.0f, 1.0f) * 1024.0f), 0u, 1023u);
        uint _184 = gl_GlobalInvocationID.x * 1024u;
        float4x4 _130 = frame_constants_view_constants[0u].view_to_clip;
        float2 _128 = 1.0f.xx;
        float2 _129 = 0.0f.xx;
        float4 _198 = float4((-_130[0u].x) * _128.x, _130[0u].y, _129.x, _130[0u].w);
        float4 _210 = float4(_130[1u].x, _130[1u].y * _128.y, _129.y, _130[1u].w);
        float4 _217 = float4(_130[3u].x, _130[3u].y, -1.0f, _130[3u].w);
        float4 _126[4];
        _126[0] = _217 + _198;
        _126[1] = _217 - _198;
        _126[2] = _217 + _210;
        _126[3] = _217 - _210;
        for (int _229 = 0; _229 < 4; )
        {
            _126[_229] /= length(_126[_229].xyz).xxxx;
            _229++;
            continue;
        }
        float4 _124[4] = _126;
        float3 _131 = _160.xyz;
        bool _256;
        if (((-_131.z) + _153) > _130[2].w)
        {
            _256 = ((-_131.z) - _153) < 3.4028234663852885981170418348452e+38f;
        }
        else
        {
            _256 = false;
        }
        bool _258;
        _258 = _256;
        bool _259;
        for (int _261 = 0; _261 < 4; _258 = _259, _261++)
        {
            if (_258)
            {
                _259 = dot(float4(_160.xy, _161, 1.0f), _124[_261]) > (-_153);
            }
            else
            {
                _259 = false;
            }
        }
        uint _278 = gl_GlobalInvocationID.x + draw_list_info.Load<uint>(push_constants.draw_list_index_base * 4 + 0);
        bool _289;
        if (_258)
        {
            _289 = (prev_visibility_bitmask.Load<uint>((_278 / 32u) * 4 + 0) & (1u << ((_278 % 32u) & 31u))) != 0u;
        }
        else
        {
            _289 = false;
        }
        if (_289)
        {
            uint _297;
            __alias_next_visibility_bitmask.InterlockedOr((_278 / 32u) * 4 + 0, 1u << ((_278 % 32u) & 31u), _297);
            uint _299;
            __alias_filtered_instance_allocator_buffer.InterlockedAdd(push_constants.draw_list_index_base * 4 + 0, 1u, _299);
            __alias_filtered_instance_buffer.Store<uint>(_299 * 4 + 0, _184 + _183);
            GpuFilteredInstanceMeta _302 = { _151 };
            __alias_filtered_instance_meta_buffer.Store<uint>(_299 * 4 + 0, _302.raw);
            uint _306;
            __alias_output_histogram.InterlockedAdd(((push_constants.draw_list_index_base * 1024u) + _183) * 4 + 0, 1u, _306);
        }
        if ((((_153 / max(9.9999997473787516355514526367188e-06f, -_161)) * ((0.5f * _125[1u].y) * float(clamp(frame_constants_inner_rendering_resolution.y, 1080u, 2160u)))) < 10.0f) ? false : true)
        {
            float4 _314 = mul(frame_constants_view_constants[2u].view_to_clip, mul(frame_constants_view_constants[2u].world_to_view, _159));
            float4 _127 = _314;
            uint _323 = clamp(uint((_314.xyz / _127.w.xxx).z * 1024.0f), 0u, 1023u);
            uint _324 = _184 + _323;
            float _333 = _153 * (2.0f * frame_constants_view_constants[2u].view_to_sample[0].x);
            float2 _334 = abs(mul(frame_constants_view_constants[2u].world_to_sample, _159).xy);
            float2 _336 = (-_333).xx;
            float2 _340 = (1.0f + _333).xx;
            uint _344 = (any(bool2(_334.x >= _336.x, _334.y >= _336.y)) ? all(bool2(_334.x <= _340.x, _334.y <= _340.y)) : false) ? 0u : 3u;
            float2 _346 = (1.0f - _333).xx;
            float2 _350 = (2.0f + _333).xx;
            bool _353 = any(bool2(_334.x >= _346.x, _334.y >= _346.y)) ? all(bool2(_334.x <= _350.x, _334.y <= _350.y)) : false;
            uint _357;
            if (_353)
            {
                _357 = min(_344, 1u);
            }
            else
            {
                _357 = _344;
            }
            uint _358 = uint(_353);
            float2 _360 = (2.0f - _333).xx;
            float2 _364 = (7.0f + _333).xx;
            uint _372;
            uint _373;
            if (any(bool2(_334.x >= _360.x, _334.y >= _360.y)) ? all(bool2(_334.x <= _364.x, _334.y <= _364.y)) : false)
            {
                _372 = max(_358, 2u);
                _373 = min(_357, 2u);
            }
            else
            {
                _372 = _358;
                _373 = _357;
            }
            for (uint _375 = _373; _375 <= _372; )
            {
                uint _383;
                __alias_filtered_instance_allocator_buffer.InterlockedAdd(((push_constants.draw_list_index_base + 1u) + _375) * 4 + 0, 1u, _383);
                __alias_filtered_instance_buffer.Store<uint>(_383 * 4 + 0, _324);
                GpuFilteredInstanceMeta _386 = { _151 };
                __alias_filtered_instance_meta_buffer.Store<uint>(_383 * 4 + 0, _386.raw);
                uint _390;
                __alias_output_histogram.InterlockedAdd(((((push_constants.draw_list_index_base + 1u) + _375) * 1024u) + _323) * 4 + 0, 1u, _390);
                _375++;
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
