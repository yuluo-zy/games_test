#define PERM_oc_pass 1
#define TGV_8a7d2f81e72c8eeb 1
#line 1 "shaders/nani/cull_and_bucket_smoke.cs.hlsl"




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
        uint2 _145 = push_constants.instance_buffer_addr;
        uint2 _143;
        _143.x = _145.x + (16u * (gl_GlobalInvocationID.x));
        _143.y = _145.y + uint(int(_143.x < _145.x));
        uint4 _175 = vk::RawBufferLoad<uint4>(((uint64_t)_143.y << 32) | _143.x, 16);
        uint4 _144 = _175;
        float3 _126 = asfloat(_175.xyz);
        _126.y += 7.5f;
        float4 _191 = mul(frame_constants_view_constants[0u].world_to_view, float4(_126, 1.0f));
        uint _198 = clamp(uint(clamp((32.0f - _191.z) * 0.00347222224809229373931884765625f, 0.0f, 1.0f) * 1024.0f), 0u, 1023u);
        float4x4 _135 = frame_constants_view_constants[0u].view_to_clip;
        float2 _133 = 1.0f.xx;
        float2 _134 = 0.0f.xx;
        float4 _215 = float4((-_135[0u].x) * _133.x, _135[0u].y, _134.x, _135[0u].w);
        float4 _227 = float4(_135[1u].x, _135[1u].y * _133.y, _134.y, _135[1u].w);
        float4 _234 = float4(_135[3u].x, _135[3u].y, -1.0f, _135[3u].w);
        float4 _129[4];
        _129[0] = _234 + _215;
        _129[1] = _234 - _215;
        _129[2] = _234 + _227;
        _129[3] = _234 - _227;
        for (int _246 = 0; _246 < 4; )
        {
            _129[_246] /= length(_129[_246].xyz).xxxx;
            _246++;
            continue;
        }
        float4 _128[4] = _129;
        float3 _261 = _191.xyz;
        float3 _136 = _261;
        float3 _137 = _261;
        bool _271;
        if ((7.5f - _137.z) > _135[2].w)
        {
            _271 = ((-7.5f) - _137.z) < 3.4028234663852885981170418348452e+38f;
        }
        else
        {
            _271 = false;
        }
        bool _273;
        _273 = _271;
        bool _274;
        for (int _276 = 0; _276 < 4; _273 = _274, _276++)
        {
            if (_273)
            {
                _274 = dot(float4(_191.xyz, 1.0f), _128[_276]) > (-7.5f);
            }
            else
            {
                _274 = false;
            }
        }
        uint _291 = draw_list_info.Load<uint>(push_constants.draw_list_index_base * 4 + 0);
        uint _292 = gl_GlobalInvocationID.x + _291;
        float3 _139 = _261;
        _139.z = -_139.z;
        float4x4 _131 = frame_constants_view_constants[0u].view_to_clip;
        float _301 = _131[0u].x * 0.5f;
        float4x4 _130 = frame_constants_view_constants[0u].view_to_clip;
        float _304 = _130[1u].y * 0.5f;
        float3 _132 = _139 * 7.5f;
        float _310 = (_139.z * _139.z) - 56.25f;
        float _316 = sqrt((_139.x * _139.x) + _310);
        float _342 = sqrt((_139.y * _139.y) + _310);
        float4 _373 = float4(hzb_tex_inner_dims.xyxy);
        float4 _376 = clamp((float4((((_316 * _139.x) - _132.z) / ((_316 * _139.z) + _132.x)) * _301, (-(((_342 * _139.y) + _132.z) / ((_342 * _139.z) - _132.y))) * _304, (((_316 * _139.x) + _132.z) / ((_316 * _139.z) - _132.x)) * _301, (-(((_342 * _139.y) - _132.z) / ((_342 * _139.z) + _132.y))) * _304) + 0.5f.xxxx) * _373, 0.0f.xxxx, _373 - 1.0f.xxxx);
        float4 _138 = _376;
        float2 _140 = float2(_138.z - _138.x, _138.w - _138.y);
        uint _396 = uint(max(0.0f, ceil(log2(max(_140.x, _140.y)))));
        bool _444;
        if (_273 ? true : false)
        {
            uint2 _403 = _396.xx & uint2(31u, 31u);
            uint2 _141 = uint2(_376.xy) >> _403;
            uint2 _142 = uint2(_376.zw) >> _403;
            float4x4 _127 = frame_constants_view_constants[0u].clip_to_view;
            _444 = (_136.z + 7.5f) > (1.0f / (min(min(hzb_tex.Load(int3(uint2(_141.x, _141.y), _396)).x, hzb_tex.Load(int3(uint2(_141.x, _142.y), _396)).x), min(hzb_tex.Load(int3(uint2(_142.x, _141.y), _396)).x, hzb_tex.Load(int3(uint2(_142.x, _142.y), _396)).x)) * (-_127[3].z)));
        }
        else
        {
            _444 = _273;
        }
        if (_444)
        {
            uint _451 = 1u << ((_292 % 32u) & 31u);
            uint _452;
            __alias_next_visibility_bitmask.InterlockedOr((_292 / 32u) * 4 + 0, _451, _452);
            if (!((prev_visibility_bitmask.Load<uint>((_292 / 32u) * 4 + 0) & _451) != 0u))
            {
                uint _461;
                __alias_filtered_instance_allocator_buffer.InterlockedAdd(push_constants.draw_list_index_base * 4 + 0, 1u, _461);
                __alias_filtered_instance_buffer.Store<uint>(_461 * 4 + 0, (gl_GlobalInvocationID.x * 1024u) + _198);
                GpuFilteredInstanceMeta _464 = { _144.w >> 16u };
                __alias_filtered_instance_meta_buffer.Store<uint>(_461 * 4 + 0, _464.raw);
                uint _468;
                __alias_output_histogram.InterlockedAdd(((push_constants.draw_list_index_base * 1024u) + _198) * 4 + 0, 1u, _468);
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
        uint2 _137 = push_constants.instance_buffer_addr;
        uint2 _135;
        _135.x = _137.x + (16u * (gl_GlobalInvocationID.x));
        _135.y = _137.y + uint(int(_135.x < _137.x));
        uint4 _167 = vk::RawBufferLoad<uint4>(((uint64_t)_135.y << 32) | _135.x, 16);
        uint4 _136 = _167;
        uint _172 = _136.w >> 16u;
        float3 _126 = asfloat(_167.xyz);
        _126.y += 7.5f;
        float4 _182 = float4(_126, 1.0f);
        float4 _183 = mul(frame_constants_view_constants[0u].world_to_view, _182);
        float _184 = _183.z;
        float4x4 _128 = frame_constants_view_constants[0u].view_to_clip;
        uint _206 = clamp(uint(clamp((32.0f - _184) * 0.00347222224809229373931884765625f, 0.0f, 1.0f) * 1024.0f), 0u, 1023u);
        uint _207 = gl_GlobalInvocationID.x * 1024u;
        float4x4 _133 = frame_constants_view_constants[0u].view_to_clip;
        float2 _131 = 1.0f.xx;
        float2 _132 = 0.0f.xx;
        float4 _221 = float4((-_133[0u].x) * _131.x, _133[0u].y, _132.x, _133[0u].w);
        float4 _233 = float4(_133[1u].x, _133[1u].y * _131.y, _132.y, _133[1u].w);
        float4 _240 = float4(_133[3u].x, _133[3u].y, -1.0f, _133[3u].w);
        float4 _129[4];
        _129[0] = _240 + _221;
        _129[1] = _240 - _221;
        _129[2] = _240 + _233;
        _129[3] = _240 - _233;
        for (int _252 = 0; _252 < 4; )
        {
            _129[_252] /= length(_129[_252].xyz).xxxx;
            _252++;
            continue;
        }
        float4 _127[4] = _129;
        float3 _134 = _183.xyz;
        bool _277;
        if ((7.5f - _134.z) > _133[2].w)
        {
            _277 = ((-7.5f) - _134.z) < 3.4028234663852885981170418348452e+38f;
        }
        else
        {
            _277 = false;
        }
        bool _279;
        _279 = _277;
        bool _280;
        for (int _282 = 0; _282 < 4; _279 = _280, _282++)
        {
            if (_279)
            {
                _280 = dot(float4(_183.xy, _184, 1.0f), _127[_282]) > (-7.5f);
            }
            else
            {
                _280 = false;
            }
        }
        uint _298 = gl_GlobalInvocationID.x + draw_list_info.Load<uint>(push_constants.draw_list_index_base * 4 + 0);
        bool _309;
        if (_279)
        {
            _309 = (prev_visibility_bitmask.Load<uint>((_298 / 32u) * 4 + 0) & (1u << ((_298 % 32u) & 31u))) != 0u;
        }
        else
        {
            _309 = false;
        }
        if (_309)
        {
            uint _317;
            __alias_next_visibility_bitmask.InterlockedOr((_298 / 32u) * 4 + 0, 1u << ((_298 % 32u) & 31u), _317);
            uint _319;
            __alias_filtered_instance_allocator_buffer.InterlockedAdd(push_constants.draw_list_index_base * 4 + 0, 1u, _319);
            __alias_filtered_instance_buffer.Store<uint>(_319 * 4 + 0, _207 + _206);
            GpuFilteredInstanceMeta _322 = { _172 };
            __alias_filtered_instance_meta_buffer.Store<uint>(_319 * 4 + 0, _322.raw);
            uint _326;
            __alias_output_histogram.InterlockedAdd(((push_constants.draw_list_index_base * 1024u) + _206) * 4 + 0, 1u, _326);
        }
        if ((((7.5f / max(9.9999997473787516355514526367188e-06f, -_184)) * ((0.5f * _128[1u].y) * float(clamp(frame_constants_inner_rendering_resolution.y, 1080u, 2160u)))) < 10.0f) ? false : true)
        {
            float4 _334 = mul(frame_constants_view_constants[2u].view_to_clip, mul(frame_constants_view_constants[2u].world_to_view, _182));
            float4 _130 = _334;
            uint _343 = clamp(uint((_334.xyz / _130.w.xxx).z * 1024.0f), 0u, 1023u);
            uint _344 = _207 + _343;
            float _352 = frame_constants_view_constants[2u].view_to_sample[0].x * 15.0f;
            float2 _353 = abs(mul(frame_constants_view_constants[2u].world_to_sample, _182).xy);
            float2 _355 = (frame_constants_view_constants[2u].view_to_sample[0].x * (-15.0f)).xx;
            float2 _359 = (1.0f + _352).xx;
            uint _363 = (any(bool2(_353.x >= _355.x, _353.y >= _355.y)) ? all(bool2(_353.x <= _359.x, _353.y <= _359.y)) : false) ? 0u : 3u;
            float2 _365 = (1.0f - _352).xx;
            float2 _369 = (2.0f + _352).xx;
            bool _372 = any(bool2(_353.x >= _365.x, _353.y >= _365.y)) ? all(bool2(_353.x <= _369.x, _353.y <= _369.y)) : false;
            uint _376;
            if (_372)
            {
                _376 = min(_363, 1u);
            }
            else
            {
                _376 = _363;
            }
            uint _377 = uint(_372);
            float2 _379 = (2.0f - _352).xx;
            float2 _383 = (7.0f + _352).xx;
            uint _391;
            uint _392;
            if (any(bool2(_353.x >= _379.x, _353.y >= _379.y)) ? all(bool2(_353.x <= _383.x, _353.y <= _383.y)) : false)
            {
                _391 = max(_377, 2u);
                _392 = min(_376, 2u);
            }
            else
            {
                _391 = _377;
                _392 = _376;
            }
            for (uint _394 = _392; _394 <= _391; )
            {
                uint _402;
                __alias_filtered_instance_allocator_buffer.InterlockedAdd(((push_constants.draw_list_index_base + 1u) + _394) * 4 + 0, 1u, _402);
                __alias_filtered_instance_buffer.Store<uint>(_402 * 4 + 0, _344);
                GpuFilteredInstanceMeta _405 = { _172 };
                __alias_filtered_instance_meta_buffer.Store<uint>(_402 * 4 + 0, _405.raw);
                uint _409;
                __alias_output_histogram.InterlockedAdd(((((push_constants.draw_list_index_base + 1u) + _394) * 1024u) + _343) * 4 + 0, 1u, _409);
                _394++;
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
