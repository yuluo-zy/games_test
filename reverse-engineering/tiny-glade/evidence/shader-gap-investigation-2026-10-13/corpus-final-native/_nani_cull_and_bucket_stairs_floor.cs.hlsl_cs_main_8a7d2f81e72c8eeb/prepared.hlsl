#define PERM_oc_pass 1
#define TGV_8a7d2f81e72c8eeb 1
#line 1 "shaders/nani/cull_and_bucket_stairs_floor.cs.hlsl"




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

struct NaniTrimeshChunk
{
    uint2 gpu_address;
    uint vertex_index_count;
    uint vertex_byte_offset;
    float4 bounding_sphere;
};

struct StairsFloorInstanceData
{
    NaniTrimeshChunk mesh;
    uint color_id;
    uint pad0;
    uint pad1;
    uint pad2;
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
        uint2 _150 = push_constants.instance_buffer_addr;
        uint2 _148;
        _148.x = _150.x + (16u * (gl_GlobalInvocationID.x));
        _148.y = _150.y + uint(int(_148.x < _150.x));
        uint4 _180 = vk::RawBufferLoad<uint4>(((uint64_t)_148.y << 32) | _148.x, 16);
        uint4 _149 = _180;
        float4 _185 = instance_buffer.Load<float4>(gl_GlobalInvocationID.x * 48 + 16);
        float4 _131 = _185;
        float4 _194 = mul(frame_constants_view_constants[0u].world_to_view, float4(_185.xyz, 1.0f));
        uint _201 = clamp(uint(clamp((32.0f - _194.z) * 0.00347222224809229373931884765625f, 0.0f, 1.0f) * 1024.0f), 0u, 1023u);
        float4x4 _140 = frame_constants_view_constants[0u].view_to_clip;
        float2 _138 = 1.0f.xx;
        float2 _139 = 0.0f.xx;
        float4 _218 = float4((-_140[0u].x) * _138.x, _140[0u].y, _139.x, _140[0u].w);
        float4 _230 = float4(_140[1u].x, _140[1u].y * _138.y, _139.y, _140[1u].w);
        float4 _237 = float4(_140[3u].x, _140[3u].y, -1.0f, _140[3u].w);
        float4 _134[4];
        _134[0] = _237 + _218;
        _134[1] = _237 - _218;
        _134[2] = _237 + _230;
        _134[3] = _237 - _230;
        for (int _249 = 0; _249 < 4; )
        {
            _134[_249] /= length(_134[_249].xyz).xxxx;
            _249++;
            continue;
        }
        float4 _133[4] = _134;
        float3 _264 = _194.xyz;
        float3 _141 = _264;
        float3 _142 = _264;
        bool _276;
        if (((-_142.z) + _131.w) > _140[2].w)
        {
            _276 = ((-_142.z) - _131.w) < 3.4028234663852885981170418348452e+38f;
        }
        else
        {
            _276 = false;
        }
        bool _278;
        _278 = _276;
        bool _279;
        for (int _281 = 0; _281 < 4; _278 = _279, _281++)
        {
            if (_278)
            {
                _279 = dot(float4(_194.xyz, 1.0f), _133[_281]) > (-_131.w);
            }
            else
            {
                _279 = false;
            }
        }
        uint _297 = draw_list_info.Load<uint>(push_constants.draw_list_index_base * 4 + 0);
        uint _298 = gl_GlobalInvocationID.x + _297;
        float3 _144 = _264;
        _144.z = -_144.z;
        float4x4 _136 = frame_constants_view_constants[0u].view_to_clip;
        float _307 = _136[0u].x * 0.5f;
        float4x4 _135 = frame_constants_view_constants[0u].view_to_clip;
        float _310 = _135[1u].y * 0.5f;
        float3 _137 = _144 * _131.w;
        float _317 = (_144.z * _144.z) - (_131.w * _131.w);
        float _323 = sqrt((_144.x * _144.x) + _317);
        float _349 = sqrt((_144.y * _144.y) + _317);
        float4 _380 = float4(hzb_tex_inner_dims.xyxy);
        float4 _383 = clamp((float4((((_323 * _144.x) - _137.z) / ((_323 * _144.z) + _137.x)) * _307, (-(((_349 * _144.y) + _137.z) / ((_349 * _144.z) - _137.y))) * _310, (((_323 * _144.x) + _137.z) / ((_323 * _144.z) - _137.x)) * _307, (-(((_349 * _144.y) - _137.z) / ((_349 * _144.z) + _137.y))) * _310) + 0.5f.xxxx) * _380, 0.0f.xxxx, _380 - 1.0f.xxxx);
        float4 _143 = _383;
        float2 _145 = float2(_143.z - _143.x, _143.w - _143.y);
        uint _403 = uint(max(0.0f, ceil(log2(max(_145.x, _145.y)))));
        bool _451;
        if (_278 ? true : false)
        {
            uint2 _410 = _403.xx & uint2(31u, 31u);
            uint2 _146 = uint2(_383.xy) >> _410;
            uint2 _147 = uint2(_383.zw) >> _410;
            float4x4 _132 = frame_constants_view_constants[0u].clip_to_view;
            _451 = (_141.z + _131.w) > (1.0f / (min(min(hzb_tex.Load(int3(uint2(_146.x, _146.y), _403)).x, hzb_tex.Load(int3(uint2(_146.x, _147.y), _403)).x), min(hzb_tex.Load(int3(uint2(_147.x, _146.y), _403)).x, hzb_tex.Load(int3(uint2(_147.x, _147.y), _403)).x)) * (-_132[3].z)));
        }
        else
        {
            _451 = _278;
        }
        if (_451)
        {
            uint _458 = 1u << ((_298 % 32u) & 31u);
            uint _459;
            __alias_next_visibility_bitmask.InterlockedOr((_298 / 32u) * 4 + 0, _458, _459);
            if (!((prev_visibility_bitmask.Load<uint>((_298 / 32u) * 4 + 0) & _458) != 0u))
            {
                uint _468;
                __alias_filtered_instance_allocator_buffer.InterlockedAdd(push_constants.draw_list_index_base * 4 + 0, 1u, _468);
                __alias_filtered_instance_buffer.Store<uint>(_468 * 4 + 0, (gl_GlobalInvocationID.x * 1024u) + _201);
                GpuFilteredInstanceMeta _471 = { _149.w >> 16u };
                __alias_filtered_instance_meta_buffer.Store<uint>(_468 * 4 + 0, _471.raw);
                uint _475;
                __alias_output_histogram.InterlockedAdd(((push_constants.draw_list_index_base * 1024u) + _201) * 4 + 0, 1u, _475);
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

struct NaniTrimeshChunk
{
    uint2 gpu_address;
    uint vertex_index_count;
    uint vertex_byte_offset;
    float4 bounding_sphere;
};

struct StairsFloorInstanceData
{
    NaniTrimeshChunk mesh;
    uint color_id;
    uint pad0;
    uint pad1;
    uint pad2;
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
        uint2 _141 = push_constants.instance_buffer_addr;
        uint2 _139;
        _139.x = _141.x + (16u * (gl_GlobalInvocationID.x));
        _139.y = _141.y + uint(int(_139.x < _141.x));
        uint4 _171 = vk::RawBufferLoad<uint4>(((uint64_t)_139.y << 32) | _139.x, 16);
        uint4 _140 = _171;
        uint _174 = _140.w >> 16u;
        float4 _176 = instance_buffer.Load<float4>(gl_GlobalInvocationID.x * 48 + 16);
        float4 _130 = _176;
        float4 _184 = float4(_176.xyz, 1.0f);
        float4 _185 = mul(frame_constants_view_constants[0u].world_to_view, _184);
        float _186 = _185.z;
        float4x4 _132 = frame_constants_view_constants[0u].view_to_clip;
        uint _208 = clamp(uint(clamp((32.0f - _186) * 0.00347222224809229373931884765625f, 0.0f, 1.0f) * 1024.0f), 0u, 1023u);
        uint _209 = gl_GlobalInvocationID.x * 1024u;
        float4x4 _137 = frame_constants_view_constants[0u].view_to_clip;
        float2 _135 = 1.0f.xx;
        float2 _136 = 0.0f.xx;
        float4 _223 = float4((-_137[0u].x) * _135.x, _137[0u].y, _136.x, _137[0u].w);
        float4 _235 = float4(_137[1u].x, _137[1u].y * _135.y, _136.y, _137[1u].w);
        float4 _242 = float4(_137[3u].x, _137[3u].y, -1.0f, _137[3u].w);
        float4 _133[4];
        _133[0] = _242 + _223;
        _133[1] = _242 - _223;
        _133[2] = _242 + _235;
        _133[3] = _242 - _235;
        for (int _254 = 0; _254 < 4; )
        {
            _133[_254] /= length(_133[_254].xyz).xxxx;
            _254++;
            continue;
        }
        float4 _131[4] = _133;
        float3 _138 = _185.xyz;
        bool _281;
        if (((-_138.z) + _130.w) > _137[2].w)
        {
            _281 = ((-_138.z) - _130.w) < 3.4028234663852885981170418348452e+38f;
        }
        else
        {
            _281 = false;
        }
        bool _283;
        _283 = _281;
        bool _284;
        for (int _286 = 0; _286 < 4; _283 = _284, _286++)
        {
            if (_283)
            {
                _284 = dot(float4(_185.xy, _186, 1.0f), _131[_286]) > (-_130.w);
            }
            else
            {
                _284 = false;
            }
        }
        uint _303 = gl_GlobalInvocationID.x + draw_list_info.Load<uint>(push_constants.draw_list_index_base * 4 + 0);
        bool _314;
        if (_283)
        {
            _314 = (prev_visibility_bitmask.Load<uint>((_303 / 32u) * 4 + 0) & (1u << ((_303 % 32u) & 31u))) != 0u;
        }
        else
        {
            _314 = false;
        }
        if (_314)
        {
            uint _322;
            __alias_next_visibility_bitmask.InterlockedOr((_303 / 32u) * 4 + 0, 1u << ((_303 % 32u) & 31u), _322);
            uint _324;
            __alias_filtered_instance_allocator_buffer.InterlockedAdd(push_constants.draw_list_index_base * 4 + 0, 1u, _324);
            __alias_filtered_instance_buffer.Store<uint>(_324 * 4 + 0, _209 + _208);
            GpuFilteredInstanceMeta _327 = { _174 };
            __alias_filtered_instance_meta_buffer.Store<uint>(_324 * 4 + 0, _327.raw);
            uint _331;
            __alias_output_histogram.InterlockedAdd(((push_constants.draw_list_index_base * 1024u) + _208) * 4 + 0, 1u, _331);
        }
        if ((((_130.w / max(9.9999997473787516355514526367188e-06f, -_186)) * ((0.5f * _132[1u].y) * float(clamp(frame_constants_inner_rendering_resolution.y, 1080u, 2160u)))) < 10.0f) ? false : true)
        {
            float4 _339 = mul(frame_constants_view_constants[2u].view_to_clip, mul(frame_constants_view_constants[2u].world_to_view, _184));
            float4 _134 = _339;
            uint _348 = clamp(uint((_339.xyz / _134.w.xxx).z * 1024.0f), 0u, 1023u);
            uint _349 = _209 + _348;
            float _358 = _130.w * (2.0f * frame_constants_view_constants[2u].view_to_sample[0].x);
            float2 _359 = abs(mul(frame_constants_view_constants[2u].world_to_sample, _184).xy);
            float2 _361 = (-_358).xx;
            float2 _365 = (1.0f + _358).xx;
            uint _369 = (any(bool2(_359.x >= _361.x, _359.y >= _361.y)) ? all(bool2(_359.x <= _365.x, _359.y <= _365.y)) : false) ? 0u : 3u;
            float2 _371 = (1.0f - _358).xx;
            float2 _375 = (2.0f + _358).xx;
            bool _378 = any(bool2(_359.x >= _371.x, _359.y >= _371.y)) ? all(bool2(_359.x <= _375.x, _359.y <= _375.y)) : false;
            uint _382;
            if (_378)
            {
                _382 = min(_369, 1u);
            }
            else
            {
                _382 = _369;
            }
            uint _383 = uint(_378);
            float2 _385 = (2.0f - _358).xx;
            float2 _389 = (7.0f + _358).xx;
            uint _397;
            uint _398;
            if (any(bool2(_359.x >= _385.x, _359.y >= _385.y)) ? all(bool2(_359.x <= _389.x, _359.y <= _389.y)) : false)
            {
                _397 = max(_383, 2u);
                _398 = min(_382, 2u);
            }
            else
            {
                _397 = _383;
                _398 = _382;
            }
            for (uint _400 = _398; _400 <= _397; )
            {
                uint _408;
                __alias_filtered_instance_allocator_buffer.InterlockedAdd(((push_constants.draw_list_index_base + 1u) + _400) * 4 + 0, 1u, _408);
                __alias_filtered_instance_buffer.Store<uint>(_408 * 4 + 0, _349);
                GpuFilteredInstanceMeta _411 = { _174 };
                __alias_filtered_instance_meta_buffer.Store<uint>(_408 * 4 + 0, _411.raw);
                uint _415;
                __alias_output_histogram.InterlockedAdd(((((push_constants.draw_list_index_base + 1u) + _400) * 1024u) + _348) * 4 + 0, 1u, _415);
                _400++;
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
