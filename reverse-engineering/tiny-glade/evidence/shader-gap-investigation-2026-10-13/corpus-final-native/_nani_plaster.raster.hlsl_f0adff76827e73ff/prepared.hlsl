#define SHADER_VARIANT_DEPTH_ONLY 1
#define TGV_f0adff76827e73ff 1
#line 1 "shaders/nani_plaster.raster.hlsl"




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

cbuffer $Globals : register(b13, space0)
{
    uint current_view_index : packoffset(c0);
    int highlight_bricks_to_delete : packoffset(c0.y);
    float3 brush_position : packoffset(c1);
    float eraser_mult : packoffset(c1.w);
    uint highlight_mode : packoffset(c2);
    uint highlighted_wall_id : packoffset(c2.y);
    uint2 normal_map_dims : packoffset(c2.z);
    uint2 mortar_dims : packoffset(c3);
};

ByteAddressBuffer instance_indirection_buffer : register(t18, space0);
ByteAddressBuffer nani_instance_data : register(t19, space0);

static float4 gl_Position;
static int gl_VertexIndex;
static int gl_InstanceIndex;
static uint out_var_C0;
static float3 out_var_C1;
static float3 out_var_C2;
static float2 out_var_C3;
static float3 out_var_C4;
static float3 out_var_C5;
static float out_var_C6;
static float out_var_C7;
static float out_var_C8;
static float3x3 out_var_C9;

struct SPIRV_Cross_Input
{
    uint gl_VertexIndex : SV_VertexID;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] nointerpolation uint out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float2 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float3 out_var_C4 : TEXCOORD4;
    [[vk::location(5)]] float3 out_var_C5 : TEXCOORD5;
    [[vk::location(6)]] float out_var_C6 : TEXCOORD6;
    [[vk::location(7)]] float out_var_C7 : TEXCOORD7;
    [[vk::location(8)]] float out_var_C8 : TEXCOORD8;
    [[vk::location(9)]] float3x3 out_var_C9 : TEXCOORD9;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    float4 _958;
    uint _959;
    float3 _960;
    float3 _961;
    float2 _962;
    float3 _963;
    float3 _964;
    float _965;
    float _966;
    float _967;
    float3x3 _968;
    do
    {
        uint _193 = uint(gl_VertexIndex) & 65535u;
        if (_193 > nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 48 + 28))
        {
            _958 = asfloat(0x7fbfffffu          ).xxxx;
            _959 = 0u;
            _960 = asfloat(0x7fbfffffu          ).xxx;
            _961 = asfloat(0x7fbfffffu          ).xxx;
            _962 = asfloat(0x7fbfffffu          ).xx;
            _963 = asfloat(0x7fbfffffu          ).xxx;
            _964 = asfloat(0x7fbfffffu          ).xxx;
            _965 = asfloat(0x7fbfffffu          );
            _966 = asfloat(0x7fbfffffu          );
            _967 = asfloat(0x7fbfffffu          );
            _968 = float3x3(asfloat(0x7fbfffffu          ).xxx, asfloat(0x7fbfffffu          ).xxx, asfloat(0x7fbfffffu          ).xxx);
            break;
        }
        uint2 _174 = nani_instance_data.Load<uint2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 48 + 32);
        uint _201 = 48u * (_193 + ((uint(gl_VertexIndex) >> 16u) * uint(nani_instance_data.Load<int>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 48 + 40))));
        int _202 = int(_201);
        uint2 _172;
        if (_202 >= 0)
        {
            _172.x = _174.x + uint(_202);
            _172.y = _174.y + uint(int(_172.x < _174.x));
        }
        else
        {
            _172.x = _174.x + uint(_202);
            _172.y = _174.y + uint((_172.x > _174.x) ? (-1) : 0);
        }
        uint4 _237 = vk::RawBufferLoad<uint4>(((uint64_t)_172.y << 32) | _172.x, 16);
        uint4 _173 = _237;
        uint2 _176 = nani_instance_data.Load<uint2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 48 + 32);
        int _239 = int(16u + _201);
        uint2 _171;
        if (_239 >= 0)
        {
            _171.x = _176.x + uint(_239);
            _171.y = _176.y + uint(int(_171.x < _176.x));
        }
        else
        {
            _171.x = _176.x + uint(_239);
            _171.y = _176.y + uint((_171.x > _176.x) ? (-1) : 0);
        }
        uint4 _274 = vk::RawBufferLoad<uint4>(((uint64_t)_171.y << 32) | _171.x, 16);
        uint4 _175 = _274;
        uint2 _177 = nani_instance_data.Load<uint2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 48 + 32);
        int _276 = int(32u + _201);
        uint2 _170;
        if (_276 >= 0)
        {
            _170.x = _177.x + uint(_276);
            _170.y = _177.y + uint(int(_170.x < _177.x));
        }
        else
        {
            _170.x = _177.x + uint(_276);
            _170.y = _177.y + uint((_170.x > _177.x) ? (-1) : 0);
        }
        uint2 _311 = vk::RawBufferLoad<uint2>(((uint64_t)_170.y << 32) | _170.x, 8);
        float3 _318 = asfloat(_274.xyz);
        float4 _326 = float4(asfloat(_237.xyz), 1.0f);
        float4 _178 = _326;
        float3 _328 = _326.xyz * 0.4000000059604644775390625f;
        int3 _330 = int3(floor(_328));
        float3 _331 = frac(_328);
        uint3 _332 = uint3(_330);
        uint3 _167 = _332;
        uint2 _166 = _332.yz;
        uint _341 = _166.y + (_166.y << 10u);
        uint _343 = _341 ^ (_341 >> 6u);
        uint _345 = _343 + (_343 << 3u);
        uint _347 = _345 ^ (_345 >> 11u);
        uint _353 = (((_166.x * 1664525u) + (_347 + (_347 << 15u))) + 1013904223u) * 1664525u;
        uint _355 = _353 ^ (_353 >> 11u);
        uint _358 = _355 ^ ((_355 << 7u) & 2636928640u);
        uint _361 = _358 ^ ((_358 << 15u) & 4022730752u);
        uint _367 = (((_167.x * 1664525u) + (_361 ^ (_361 >> 18u))) + 1013904223u) * 1664525u;
        uint _369 = _367 ^ (_367 >> 11u);
        uint _372 = _369 ^ ((_369 << 7u) & 2636928640u);
        uint _375 = _372 ^ ((_372 << 15u) & 4022730752u);
        uint _377 = _375 ^ (_375 >> 18u);
        uint _379 = _377 + (_377 << 10u);
        uint _381 = _379 ^ (_379 >> 6u);
        uint _383 = _381 + (_381 << 3u);
        uint _385 = _383 ^ (_383 >> 11u);
        float2 _165;
        _165.x = asfloat((_377 & 8388607u) | 1065353216u) - 1.0f;
        _165.y = asfloat(((_385 + (_385 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
        uint3 _399 = uint3(_330 + int3(1, 0, 0));
        _167 = _399;
        _166 = _399.yz;
        uint _405 = _166.y + (_166.y << 10u);
        uint _407 = _405 ^ (_405 >> 6u);
        uint _409 = _407 + (_407 << 3u);
        uint _411 = _409 ^ (_409 >> 11u);
        uint _417 = (((_166.x * 1664525u) + (_411 + (_411 << 15u))) + 1013904223u) * 1664525u;
        uint _419 = _417 ^ (_417 >> 11u);
        uint _422 = _419 ^ ((_419 << 7u) & 2636928640u);
        uint _425 = _422 ^ ((_422 << 15u) & 4022730752u);
        uint _431 = (((_167.x * 1664525u) + (_425 ^ (_425 >> 18u))) + 1013904223u) * 1664525u;
        uint _433 = _431 ^ (_431 >> 11u);
        uint _436 = _433 ^ ((_433 << 7u) & 2636928640u);
        uint _439 = _436 ^ ((_436 << 15u) & 4022730752u);
        uint _441 = _439 ^ (_439 >> 18u);
        uint _443 = _441 + (_441 << 10u);
        uint _445 = _443 ^ (_443 >> 6u);
        uint _447 = _445 + (_445 << 3u);
        uint _449 = _447 ^ (_447 >> 11u);
        float2 _164;
        _164.x = asfloat((_441 & 8388607u) | 1065353216u) - 1.0f;
        _164.y = asfloat(((_449 + (_449 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
        uint3 _463 = uint3(_330 + int3(0, 1, 0));
        _167 = _463;
        _166 = _463.yz;
        uint _469 = _166.y + (_166.y << 10u);
        uint _471 = _469 ^ (_469 >> 6u);
        uint _473 = _471 + (_471 << 3u);
        uint _475 = _473 ^ (_473 >> 11u);
        uint _481 = (((_166.x * 1664525u) + (_475 + (_475 << 15u))) + 1013904223u) * 1664525u;
        uint _483 = _481 ^ (_481 >> 11u);
        uint _486 = _483 ^ ((_483 << 7u) & 2636928640u);
        uint _489 = _486 ^ ((_486 << 15u) & 4022730752u);
        uint _495 = (((_167.x * 1664525u) + (_489 ^ (_489 >> 18u))) + 1013904223u) * 1664525u;
        uint _497 = _495 ^ (_495 >> 11u);
        uint _500 = _497 ^ ((_497 << 7u) & 2636928640u);
        uint _503 = _500 ^ ((_500 << 15u) & 4022730752u);
        uint _505 = _503 ^ (_503 >> 18u);
        uint _507 = _505 + (_505 << 10u);
        uint _509 = _507 ^ (_507 >> 6u);
        uint _511 = _509 + (_509 << 3u);
        uint _513 = _511 ^ (_511 >> 11u);
        float2 _163;
        _163.x = asfloat((_505 & 8388607u) | 1065353216u) - 1.0f;
        _163.y = asfloat(((_513 + (_513 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
        uint3 _527 = uint3(_330 + int3(1, 1, 0));
        _167 = _527;
        _166 = _527.yz;
        uint _533 = _166.y + (_166.y << 10u);
        uint _535 = _533 ^ (_533 >> 6u);
        uint _537 = _535 + (_535 << 3u);
        uint _539 = _537 ^ (_537 >> 11u);
        uint _545 = (((_166.x * 1664525u) + (_539 + (_539 << 15u))) + 1013904223u) * 1664525u;
        uint _547 = _545 ^ (_545 >> 11u);
        uint _550 = _547 ^ ((_547 << 7u) & 2636928640u);
        uint _553 = _550 ^ ((_550 << 15u) & 4022730752u);
        uint _559 = (((_167.x * 1664525u) + (_553 ^ (_553 >> 18u))) + 1013904223u) * 1664525u;
        uint _561 = _559 ^ (_559 >> 11u);
        uint _564 = _561 ^ ((_561 << 7u) & 2636928640u);
        uint _567 = _564 ^ ((_564 << 15u) & 4022730752u);
        uint _569 = _567 ^ (_567 >> 18u);
        uint _571 = _569 + (_569 << 10u);
        uint _573 = _571 ^ (_571 >> 6u);
        uint _575 = _573 + (_573 << 3u);
        uint _577 = _575 ^ (_575 >> 11u);
        float2 _162;
        _162.x = asfloat((_569 & 8388607u) | 1065353216u) - 1.0f;
        _162.y = asfloat(((_577 + (_577 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
        uint3 _591 = uint3(_330 + int3(0, 0, 1));
        _167 = _591;
        _166 = _591.yz;
        uint _597 = _166.y + (_166.y << 10u);
        uint _599 = _597 ^ (_597 >> 6u);
        uint _601 = _599 + (_599 << 3u);
        uint _603 = _601 ^ (_601 >> 11u);
        uint _609 = (((_166.x * 1664525u) + (_603 + (_603 << 15u))) + 1013904223u) * 1664525u;
        uint _611 = _609 ^ (_609 >> 11u);
        uint _614 = _611 ^ ((_611 << 7u) & 2636928640u);
        uint _617 = _614 ^ ((_614 << 15u) & 4022730752u);
        uint _623 = (((_167.x * 1664525u) + (_617 ^ (_617 >> 18u))) + 1013904223u) * 1664525u;
        uint _625 = _623 ^ (_623 >> 11u);
        uint _628 = _625 ^ ((_625 << 7u) & 2636928640u);
        uint _631 = _628 ^ ((_628 << 15u) & 4022730752u);
        uint _633 = _631 ^ (_631 >> 18u);
        uint _635 = _633 + (_633 << 10u);
        uint _637 = _635 ^ (_635 >> 6u);
        uint _639 = _637 + (_637 << 3u);
        uint _641 = _639 ^ (_639 >> 11u);
        float2 _161;
        _161.x = asfloat((_633 & 8388607u) | 1065353216u) - 1.0f;
        _161.y = asfloat(((_641 + (_641 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
        uint3 _655 = uint3(_330 + int3(1, 0, 1));
        _167 = _655;
        _166 = _655.yz;
        uint _661 = _166.y + (_166.y << 10u);
        uint _663 = _661 ^ (_661 >> 6u);
        uint _665 = _663 + (_663 << 3u);
        uint _667 = _665 ^ (_665 >> 11u);
        uint _673 = (((_166.x * 1664525u) + (_667 + (_667 << 15u))) + 1013904223u) * 1664525u;
        uint _675 = _673 ^ (_673 >> 11u);
        uint _678 = _675 ^ ((_675 << 7u) & 2636928640u);
        uint _681 = _678 ^ ((_678 << 15u) & 4022730752u);
        uint _687 = (((_167.x * 1664525u) + (_681 ^ (_681 >> 18u))) + 1013904223u) * 1664525u;
        uint _689 = _687 ^ (_687 >> 11u);
        uint _692 = _689 ^ ((_689 << 7u) & 2636928640u);
        uint _695 = _692 ^ ((_692 << 15u) & 4022730752u);
        uint _697 = _695 ^ (_695 >> 18u);
        uint _699 = _697 + (_697 << 10u);
        uint _701 = _699 ^ (_699 >> 6u);
        uint _703 = _701 + (_701 << 3u);
        uint _705 = _703 ^ (_703 >> 11u);
        float2 _160;
        _160.x = asfloat((_697 & 8388607u) | 1065353216u) - 1.0f;
        _160.y = asfloat(((_705 + (_705 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
        uint3 _719 = uint3(_330 + int3(0, 1, 1));
        _167 = _719;
        _166 = _719.yz;
        uint _725 = _166.y + (_166.y << 10u);
        uint _727 = _725 ^ (_725 >> 6u);
        uint _729 = _727 + (_727 << 3u);
        uint _731 = _729 ^ (_729 >> 11u);
        uint _737 = (((_166.x * 1664525u) + (_731 + (_731 << 15u))) + 1013904223u) * 1664525u;
        uint _739 = _737 ^ (_737 >> 11u);
        uint _742 = _739 ^ ((_739 << 7u) & 2636928640u);
        uint _745 = _742 ^ ((_742 << 15u) & 4022730752u);
        uint _751 = (((_167.x * 1664525u) + (_745 ^ (_745 >> 18u))) + 1013904223u) * 1664525u;
        uint _753 = _751 ^ (_751 >> 11u);
        uint _756 = _753 ^ ((_753 << 7u) & 2636928640u);
        uint _759 = _756 ^ ((_756 << 15u) & 4022730752u);
        uint _761 = _759 ^ (_759 >> 18u);
        uint _763 = _761 + (_761 << 10u);
        uint _765 = _763 ^ (_763 >> 6u);
        uint _767 = _765 + (_765 << 3u);
        uint _769 = _767 ^ (_767 >> 11u);
        float2 _159;
        _159.x = asfloat((_761 & 8388607u) | 1065353216u) - 1.0f;
        _159.y = asfloat(((_769 + (_769 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
        uint3 _783 = uint3(_330 + int3(1, 1, 1));
        _167 = _783;
        _166 = _783.yz;
        uint _789 = _166.y + (_166.y << 10u);
        uint _791 = _789 ^ (_789 >> 6u);
        uint _793 = _791 + (_791 << 3u);
        uint _795 = _793 ^ (_793 >> 11u);
        uint _801 = (((_166.x * 1664525u) + (_795 + (_795 << 15u))) + 1013904223u) * 1664525u;
        uint _803 = _801 ^ (_801 >> 11u);
        uint _806 = _803 ^ ((_803 << 7u) & 2636928640u);
        uint _809 = _806 ^ ((_806 << 15u) & 4022730752u);
        uint _815 = (((_167.x * 1664525u) + (_809 ^ (_809 >> 18u))) + 1013904223u) * 1664525u;
        uint _817 = _815 ^ (_815 >> 11u);
        uint _820 = _817 ^ ((_817 << 7u) & 2636928640u);
        uint _823 = _820 ^ ((_820 << 15u) & 4022730752u);
        uint _825 = _823 ^ (_823 >> 18u);
        uint _827 = _825 + (_825 << 10u);
        uint _829 = _827 ^ (_827 >> 6u);
        uint _831 = _829 + (_829 << 3u);
        uint _833 = _831 ^ (_831 >> 11u);
        float2 _158;
        _158.x = asfloat((_825 & 8388607u) | 1065353216u) - 1.0f;
        _158.y = asfloat(((_833 + (_833 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
        float3 _168 = (_331 * _331) * (3.0f.xxx - (_331 * 2.0f));
        float2 _927 = (((((((((_165 * (1.0f - _168.x)) * (1.0f - _168.y)) * (1.0f - _168.z)) + (((_164 * _168.x) * (1.0f - _168.y)) * (1.0f - _168.z))) + (((_163 * (1.0f - _168.x)) * _168.y) * (1.0f - _168.z))) + (((_162 * _168.x) * _168.y) * (1.0f - _168.z))) + (((_161 * (1.0f - _168.x)) * (1.0f - _168.y)) * _168.z)) + (((_160 * _168.x) * (1.0f - _168.y)) * _168.z)) + (((_159 * (1.0f - _168.x)) * _168.y) * _168.z)) + (((_158 * _168.x) * _168.y) * _168.z);
        float2 _169 = clamp((_927 - 0.5f.xx) * 2.5f, (-0.5f).xx, 0.5f.xx);
        float4 _937 = _178;
        float3 _939 = _937.xyz + (float3(_169.x, 0.0f, _169.y) * 0.0599999986588954925537109375f);
        float4 _940 = float4(_939.x, _939.y, _939.z, _937.w);
        _178 = _940;
        float3 _947 = cross(_318, float3(0.0f, 1.0f, 0.0f));
        _958 = mul(frame_constants_view_constants[current_view_index].world_to_sample, _940);
        _959 = instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0);
        _960 = _318;
        _961 = _939.xyz;
        _962 = asfloat(_311);
        _963 = _947;
        _964 = float3(0.0f, 1.0f, 0.0f);
        _965 = asfloat(_173.w);
        _966 = asfloat(_175.w) + (1.0f - smoothstep(0.0f, 1.0f, _178.y));
        _967 = nani_instance_data.Load<float>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 48 + 44);
        _968 = transpose(float3x3(normalize(_947), normalize(float3(0.0f, 1.0f, 0.0f)), normalize(_318)));
        break;
    } while(false);
    gl_Position = _958;
    out_var_C0 = _959;
    out_var_C1 = _960;
    out_var_C2 = _961;
    out_var_C3 = _962;
    out_var_C4 = _963;
    out_var_C5 = _964;
    out_var_C6 = _965;
    out_var_C7 = _966;
    out_var_C8 = _967;
    out_var_C9 = _968;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_VertexIndex = int(stage_input.gl_VertexIndex);
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    stage_output.out_var_C4 = out_var_C4;
    stage_output.out_var_C5 = out_var_C5;
    stage_output.out_var_C6 = out_var_C6;
    stage_output.out_var_C7 = out_var_C7;
    stage_output.out_var_C8 = out_var_C8;
    stage_output.out_var_C9 = out_var_C9;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)

#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
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

static uint _173;

cbuffer $Globals : register(b13, space0)
{
    uint current_view_index : packoffset(c0);
    int highlight_bricks_to_delete : packoffset(c0.y);
    float3 brush_position : packoffset(c1);
    float eraser_mult : packoffset(c1.w);
    uint highlight_mode : packoffset(c2);
    uint highlighted_wall_id : packoffset(c2.y);
    uint2 normal_map_dims : packoffset(c2.z);
    uint2 mortar_dims : packoffset(c3);
};

ByteAddressBuffer wall_color_ids : register(t15, space0);
ByteAddressBuffer nani_instance_data : register(t19, space0);
SamplerState sampler_llr : register(s1, space0);
Texture2DArray<float4> plaster_colors : register(t17, space0);
Texture2D<float4> normal_map : register(t20, space0);
Texture2D<float4> mortar : register(t21, space0);

static uint in_var_C0;
static float3 in_var_C1;
static float3 in_var_C2;
static float2 in_var_C3;
static float3 in_var_C4;
static float3 in_var_C5;
static float in_var_C6;
static float in_var_C7;
static float in_var_C8;
static float3x3 in_var_C9;
static uint2 out_var_SV_TARGET0;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] nointerpolation uint in_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 in_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 in_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float2 in_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float3 in_var_C4 : TEXCOORD4;
    [[vk::location(5)]] float3 in_var_C5 : TEXCOORD5;
    [[vk::location(6)]] float in_var_C6 : TEXCOORD6;
    [[vk::location(7)]] float in_var_C7 : TEXCOORD7;
    [[vk::location(8)]] float in_var_C8 : TEXCOORD8;
    [[vk::location(9)]] float3x3 in_var_C9 : TEXCOORD9;
};

struct SPIRV_Cross_Output
{
    uint2 out_var_SV_TARGET0 : SV_Target0;
};

void frag_main()
{
    uint _223;
    if (nani_instance_data.Load<uint>(in_var_C0 * 48 + 16) == highlighted_wall_id)
    {
        _223 = highlight_mode;
    }
    else
    {
        _223 = 0u;
    }
    float _224 = clamp(in_var_C6, 0.0f, 1.0f);
    uint _231 = uint(nani_instance_data.Load<float>(in_var_C0 * 48 + 20) * 1.75f);
    uint _233 = _231 + (_231 << 10u);
    uint _235 = _233 ^ (_233 >> 6u);
    uint _237 = _235 + (_235 << 3u);
    uint _239 = _237 ^ (_237 >> 11u);
    float _249 = (int(nani_instance_data.Load<uint>(in_var_C0 * 48 + 24) != 0u) == 0) ? 0.0f : smoothstep(1.0f - (1.0f / nani_instance_data.Load<float>(in_var_C0 * 48 + 20)), 1.0f, in_var_C3.x / nani_instance_data.Load<float>(in_var_C0 * 48 + 20));
    if (_224 < 0.100000001490116119384765625f)
    {
        discard;
    }
    float2 _253 = lerp(0.0f, 10.0f, asfloat(((_239 + (_239 << 15u)) & 8388607u) | 1065353216u) - 1.0f).xx;
    float _254 = lerp(0.0f, 0.5f, in_var_C8);
    float2 _265;
    uint _267;
    uint _269;
    bool _271;
    bool _273;
    float _275;
    _265 = ((in_var_C3 * 1.0f) + 23.3999996185302734375f.xx) + _253;
    _267 = _173;
    _269 = _173;
    _271 = false;
    _273 = false;
    _275 = 0.0f;
    float _262 = 0.5f;
    int _277 = 0;
    for (; _277 < 3; )
    {
        int2 _282 = int2(floor(_265));
        float2 _283 = frac(_265);
        uint2 _284 = uint2(_282);
        uint _286 = _284.y;
        uint _288 = _286 + (_286 << 10u);
        uint _290 = _288 ^ (_288 >> 6u);
        uint _292 = _290 + (_290 << 3u);
        uint _294 = _292 ^ (_292 >> 11u);
        bool _297 = _273 ? _273 : true;
        uint _298 = _273 ? _269 : 1664525u;
        bool _299 = _271 ? _271 : true;
        uint _300 = _271 ? _267 : 1013904223u;
        uint _304 = (((_284.x * _298) + (_294 + (_294 << 15u))) + _300) * _298;
        uint _306 = _304 ^ (_304 >> 11u);
        uint _309 = _306 ^ ((_306 << 7u) & 2636928640u);
        uint _312 = _309 ^ ((_309 << 15u) & 4022730752u);
        uint2 _320 = uint2(_282 + int2(1, 0));
        uint _322 = _320.y;
        uint _324 = _322 + (_322 << 10u);
        uint _326 = _324 ^ (_324 >> 6u);
        uint _328 = _326 + (_326 << 3u);
        uint _330 = _328 ^ (_328 >> 11u);
        bool _333 = _297 ? _297 : true;
        uint _334 = _297 ? _298 : 1664525u;
        bool _335 = _299 ? _299 : true;
        uint _336 = _299 ? _300 : 1013904223u;
        uint _340 = (((_320.x * _334) + (_330 + (_330 << 15u))) + _336) * _334;
        uint _342 = _340 ^ (_340 >> 11u);
        uint _345 = _342 ^ ((_342 << 7u) & 2636928640u);
        uint _348 = _345 ^ ((_345 << 15u) & 4022730752u);
        uint2 _356 = uint2(_282 + int2(0, 1));
        uint _358 = _356.y;
        uint _360 = _358 + (_358 << 10u);
        uint _362 = _360 ^ (_360 >> 6u);
        uint _364 = _362 + (_362 << 3u);
        uint _366 = _364 ^ (_364 >> 11u);
        bool _369 = _333 ? _333 : true;
        uint _370 = _333 ? _334 : 1664525u;
        bool _371 = _335 ? _335 : true;
        uint _372 = _335 ? _336 : 1013904223u;
        uint _376 = (((_356.x * _370) + (_366 + (_366 << 15u))) + _372) * _370;
        uint _378 = _376 ^ (_376 >> 11u);
        uint _381 = _378 ^ ((_378 << 7u) & 2636928640u);
        uint _384 = _381 ^ ((_381 << 15u) & 4022730752u);
        uint2 _392 = uint2(_282 + int2(1, 1));
        uint _394 = _392.y;
        uint _396 = _394 + (_394 << 10u);
        uint _398 = _396 ^ (_396 >> 6u);
        uint _400 = _398 + (_398 << 3u);
        uint _402 = _400 ^ (_400 >> 11u);
        uint _270 = _369 ? _370 : 1664525u;
        uint _268 = _371 ? _372 : 1013904223u;
        uint _408 = (((_392.x * _270) + (_402 + (_402 << 15u))) + _268) * _270;
        uint _410 = _408 ^ (_408 >> 11u);
        uint _413 = _410 ^ ((_410 << 7u) & 2636928640u);
        uint _416 = _413 ^ ((_413 << 15u) & 4022730752u);
        float2 _426 = (_283 * _283) * (3.0f.xx - (_283 * 2.0f));
        float _427 = _426.x;
        float _428 = 1.0f - _427;
        float _430 = _426.y;
        float _443 = _262 * ((-1.0f) + (2.0f * ((((1.0f - _430) * (((asfloat(((_312 ^ (_312 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _428) + ((asfloat(((_348 ^ (_348 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _427))) + (((asfloat(((_384 ^ (_384 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _428) * _430)) + (((asfloat(((_416 ^ (_416 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _427) * _430))));
        _262 *= 0.550000011920928955078125f;
        _265 = mul(float2x2(float2(0.800000011920928955078125f, -0.60000002384185791015625f), float2(0.60000002384185791015625f, 0.800000011920928955078125f)), _265 * 1.89999997615814208984375f);
        _267 = _268;
        _269 = _270;
        _271 = _371 ? _371 : true;
        _273 = _369 ? _369 : true;
        _275 += _443;
        _277++;
        continue;
    }
    float2 _454;
    uint _456;
    uint _458;
    bool _460;
    bool _462;
    float _464;
    _454 = ((in_var_C3 * 0.5f) + 0.5f.xx) + _253;
    _456 = _267;
    _458 = _269;
    _460 = _271;
    _462 = _273;
    _464 = 0.0f;
    float _451 = 0.5f;
    int _466 = 0;
    for (; _466 < 5; )
    {
        int2 _471 = int2(floor(_454));
        float2 _472 = frac(_454);
        uint2 _473 = uint2(_471);
        uint _475 = _473.y;
        uint _477 = _475 + (_475 << 10u);
        uint _479 = _477 ^ (_477 >> 6u);
        uint _481 = _479 + (_479 << 3u);
        uint _483 = _481 ^ (_481 >> 11u);
        bool _486 = _462 ? _462 : true;
        uint _487 = _462 ? _458 : 1664525u;
        bool _488 = _460 ? _460 : true;
        uint _489 = _460 ? _456 : 1013904223u;
        uint _493 = (((_473.x * _487) + (_483 + (_483 << 15u))) + _489) * _487;
        uint _495 = _493 ^ (_493 >> 11u);
        uint _498 = _495 ^ ((_495 << 7u) & 2636928640u);
        uint _501 = _498 ^ ((_498 << 15u) & 4022730752u);
        uint2 _509 = uint2(_471 + int2(1, 0));
        uint _511 = _509.y;
        uint _513 = _511 + (_511 << 10u);
        uint _515 = _513 ^ (_513 >> 6u);
        uint _517 = _515 + (_515 << 3u);
        uint _519 = _517 ^ (_517 >> 11u);
        bool _522 = _486 ? _486 : true;
        uint _523 = _486 ? _487 : 1664525u;
        bool _524 = _488 ? _488 : true;
        uint _525 = _488 ? _489 : 1013904223u;
        uint _529 = (((_509.x * _523) + (_519 + (_519 << 15u))) + _525) * _523;
        uint _531 = _529 ^ (_529 >> 11u);
        uint _534 = _531 ^ ((_531 << 7u) & 2636928640u);
        uint _537 = _534 ^ ((_534 << 15u) & 4022730752u);
        uint2 _545 = uint2(_471 + int2(0, 1));
        uint _547 = _545.y;
        uint _549 = _547 + (_547 << 10u);
        uint _551 = _549 ^ (_549 >> 6u);
        uint _553 = _551 + (_551 << 3u);
        uint _555 = _553 ^ (_553 >> 11u);
        bool _558 = _522 ? _522 : true;
        uint _559 = _522 ? _523 : 1664525u;
        bool _560 = _524 ? _524 : true;
        uint _561 = _524 ? _525 : 1013904223u;
        uint _565 = (((_545.x * _559) + (_555 + (_555 << 15u))) + _561) * _559;
        uint _567 = _565 ^ (_565 >> 11u);
        uint _570 = _567 ^ ((_567 << 7u) & 2636928640u);
        uint _573 = _570 ^ ((_570 << 15u) & 4022730752u);
        uint2 _581 = uint2(_471 + int2(1, 1));
        uint _583 = _581.y;
        uint _585 = _583 + (_583 << 10u);
        uint _587 = _585 ^ (_585 >> 6u);
        uint _589 = _587 + (_587 << 3u);
        uint _591 = _589 ^ (_589 >> 11u);
        uint _459 = _558 ? _559 : 1664525u;
        uint _457 = _560 ? _561 : 1013904223u;
        uint _597 = (((_581.x * _459) + (_591 + (_591 << 15u))) + _457) * _459;
        uint _599 = _597 ^ (_597 >> 11u);
        uint _602 = _599 ^ ((_599 << 7u) & 2636928640u);
        uint _605 = _602 ^ ((_602 << 15u) & 4022730752u);
        float2 _615 = (_472 * _472) * (3.0f.xx - (_472 * 2.0f));
        float _616 = _615.x;
        float _617 = 1.0f - _616;
        float _619 = _615.y;
        float _632 = _451 * ((-1.0f) + (2.0f * ((((1.0f - _619) * (((asfloat(((_501 ^ (_501 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _617) + ((asfloat(((_537 ^ (_537 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _616))) + (((asfloat(((_573 ^ (_573 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _617) * _619)) + (((asfloat(((_605 ^ (_605 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _616) * _619))));
        _451 *= 0.550000011920928955078125f;
        _454 = mul(float2x2(float2(0.800000011920928955078125f, -0.60000002384185791015625f), float2(0.60000002384185791015625f, 0.800000011920928955078125f)), _454 * 1.89999997615814208984375f);
        _456 = _457;
        _458 = _459;
        _460 = _560 ? _560 : true;
        _462 = _558 ? _558 : true;
        _464 += _632;
        _466++;
        continue;
    }
    float _636 = smoothstep(0.0f, 1.0f, 1.0f - _224);
    float _637 = _636 * (lerp(0.300000011920928955078125f, 1.0f, _275) * in_var_C7);
    float _638 = (_254 + _464) + _637;
    bool _639 = _249 > 0.0f;
    uint _835;
    uint _836;
    bool _837;
    bool _838;
    float _839;
    if (_639)
    {
        float2 _652;
        uint _654;
        uint _656;
        bool _658;
        bool _660;
        float _662;
        _652 = ((float2(nani_instance_data.Load<float>(in_var_C0 * 48 + 20) - in_var_C3.x, in_var_C3.y) * 0.5f) + 0.5f.xx) + _253;
        _654 = _456;
        _656 = _458;
        _658 = _460;
        _660 = _462;
        _662 = 0.0f;
        float _649 = 0.5f;
        int _664 = 0;
        for (; _664 < 5; )
        {
            int2 _669 = int2(floor(_652));
            float2 _670 = frac(_652);
            uint2 _671 = uint2(_669);
            uint _673 = _671.y;
            uint _675 = _673 + (_673 << 10u);
            uint _677 = _675 ^ (_675 >> 6u);
            uint _679 = _677 + (_677 << 3u);
            uint _681 = _679 ^ (_679 >> 11u);
            bool _684 = _660 ? _660 : true;
            uint _685 = _660 ? _656 : 1664525u;
            bool _686 = _658 ? _658 : true;
            uint _687 = _658 ? _654 : 1013904223u;
            uint _691 = (((_671.x * _685) + (_681 + (_681 << 15u))) + _687) * _685;
            uint _693 = _691 ^ (_691 >> 11u);
            uint _696 = _693 ^ ((_693 << 7u) & 2636928640u);
            uint _699 = _696 ^ ((_696 << 15u) & 4022730752u);
            uint2 _707 = uint2(_669 + int2(1, 0));
            uint _709 = _707.y;
            uint _711 = _709 + (_709 << 10u);
            uint _713 = _711 ^ (_711 >> 6u);
            uint _715 = _713 + (_713 << 3u);
            uint _717 = _715 ^ (_715 >> 11u);
            bool _720 = _684 ? _684 : true;
            uint _721 = _684 ? _685 : 1664525u;
            bool _722 = _686 ? _686 : true;
            uint _723 = _686 ? _687 : 1013904223u;
            uint _727 = (((_707.x * _721) + (_717 + (_717 << 15u))) + _723) * _721;
            uint _729 = _727 ^ (_727 >> 11u);
            uint _732 = _729 ^ ((_729 << 7u) & 2636928640u);
            uint _735 = _732 ^ ((_732 << 15u) & 4022730752u);
            uint2 _743 = uint2(_669 + int2(0, 1));
            uint _745 = _743.y;
            uint _747 = _745 + (_745 << 10u);
            uint _749 = _747 ^ (_747 >> 6u);
            uint _751 = _749 + (_749 << 3u);
            uint _753 = _751 ^ (_751 >> 11u);
            bool _756 = _720 ? _720 : true;
            uint _757 = _720 ? _721 : 1664525u;
            bool _758 = _722 ? _722 : true;
            uint _759 = _722 ? _723 : 1013904223u;
            uint _763 = (((_743.x * _757) + (_753 + (_753 << 15u))) + _759) * _757;
            uint _765 = _763 ^ (_763 >> 11u);
            uint _768 = _765 ^ ((_765 << 7u) & 2636928640u);
            uint _771 = _768 ^ ((_768 << 15u) & 4022730752u);
            uint2 _779 = uint2(_669 + int2(1, 1));
            uint _781 = _779.y;
            uint _783 = _781 + (_781 << 10u);
            uint _785 = _783 ^ (_783 >> 6u);
            uint _787 = _785 + (_785 << 3u);
            uint _789 = _787 ^ (_787 >> 11u);
            uint _657 = _756 ? _757 : 1664525u;
            uint _655 = _758 ? _759 : 1013904223u;
            uint _795 = (((_779.x * _657) + (_789 + (_789 << 15u))) + _655) * _657;
            uint _797 = _795 ^ (_795 >> 11u);
            uint _800 = _797 ^ ((_797 << 7u) & 2636928640u);
            uint _803 = _800 ^ ((_800 << 15u) & 4022730752u);
            float2 _813 = (_670 * _670) * (3.0f.xx - (_670 * 2.0f));
            float _814 = _813.x;
            float _815 = 1.0f - _814;
            float _817 = _813.y;
            float _830 = _649 * ((-1.0f) + (2.0f * ((((1.0f - _817) * (((asfloat(((_699 ^ (_699 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _815) + ((asfloat(((_735 ^ (_735 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _814))) + (((asfloat(((_771 ^ (_771 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _815) * _817)) + (((asfloat(((_803 ^ (_803 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _814) * _817))));
            _649 *= 0.550000011920928955078125f;
            _652 = mul(float2x2(float2(0.800000011920928955078125f, -0.60000002384185791015625f), float2(0.60000002384185791015625f, 0.800000011920928955078125f)), _652 * 1.89999997615814208984375f);
            _654 = _655;
            _656 = _657;
            _658 = _758 ? _758 : true;
            _660 = _756 ? _756 : true;
            _662 += _830;
            _664++;
            continue;
        }
        _835 = _654;
        _836 = _656;
        _837 = _658;
        _838 = _660;
        _839 = lerp(_638, (_254 + _662) + _637, _249);
    }
    else
    {
        _835 = _456;
        _836 = _458;
        _837 = _460;
        _838 = _462;
        _839 = _638;
    }
    float _841 = smoothstep(0.0f, 4.0f, in_var_C3.y);
    float _844 = min((_841 * 1.5f) - 0.20000000298023223876953125f, 0.89999997615814208984375f);
    float _845 = _844 + 0.100000001490116119384765625f;
    float _846 = _839 * 2.0f;
    float _847 = smoothstep(_844, _845, _846);
    float _851 = clamp(smoothstep(_844 - 0.100000001490116119384765625f, _845, _846) - _847, 0.0f, 1.0f);
    if ((_839 * lerp(2.0f, 2.7999999523162841796875f, clamp(in_var_C8 * 2.0f, 0.0f, 1.0f))) > (_841 * 2.0f))
    {
        discard;
    }
    float2 _857 = in_var_C3 * 0.100000001490116119384765625f;
    int _860 = int(wall_color_ids.Load<uint>(nani_instance_data.Load<uint>(in_var_C0 * 48 + 16) * 4 + 0));
    float _865 = float(_860 - 4);
    float4 _870 = plaster_colors.Sample(sampler_llr, float3(_857 + _253, _865));
    float3 _871 = _870.xyz;
    float3 _888;
    if (_639)
    {
        _888 = lerp(_871, plaster_colors.Sample(sampler_llr, float3(float2((nani_instance_data.Load<float>(in_var_C0 * 48 + 20) * 0.100000001490116119384765625f) - _857.x, _857.y) + _253, _865)).xyz, _249.xxx);
    }
    else
    {
        _888 = _871;
    }
    float3 _926;
    if (_860 == 4)
    {
        _926 = float3(0.4766300618648529052734375f, 0.23876892030239105224609375f, 0.17930369079113006591796875f);
    }
    else
    {
        float3 _925;
        if (_860 == 5)
        {
            _925 = float3(0.36000001430511474609375f, 0.3288960158824920654296875f, 0.165600001811981201171875f);
        }
        else
        {
            float3 _924;
            if (_860 == 6)
            {
                _924 = float3(0.2800000011920928955078125f, 0.29254400730133056640625f, 0.2512000501155853271484375f);
            }
            else
            {
                float3 _923;
                if (_860 == 7)
                {
                    _923 = float3(0.2100000083446502685546875f, 0.1674120128154754638671875f, 0.1281000077724456787109375f);
                }
                else
                {
                    float3 _922;
                    if (_860 == 8)
                    {
                        _922 = float3(0.4000000059604644775390625f, 0.3656800091266632080078125f, 0.24400003254413604736328125f);
                    }
                    else
                    {
                        float3 _921;
                        if (_860 == 9)
                        {
                            _921 = float3(0.2700000107288360595703125f, 0.3499999940395355224609375f, 0.2700000107288360595703125f);
                        }
                        else
                        {
                            bool _918;
                            if (!(_860 == 10))
                            {
                                _918 = _860 == 12;
                            }
                            else
                            {
                                _918 = true;
                            }
                            bool3 _919 = _918.xxx;
                            _921 = float3(_919.x ? float3(0.4000000059604644775390625f, 0.3656800091266632080078125f, 0.24400003254413604736328125f).x : 0.0f.xxx.x, _919.y ? float3(0.4000000059604644775390625f, 0.3656800091266632080078125f, 0.24400003254413604736328125f).y : 0.0f.xxx.y, _919.z ? float3(0.4000000059604644775390625f, 0.3656800091266632080078125f, 0.24400003254413604736328125f).z : 0.0f.xxx.z);
                        }
                        _922 = _921;
                    }
                    _923 = _922;
                }
                _924 = _923;
            }
            _925 = _924;
        }
        _926 = _925;
    }
    float4 _931 = mortar.Sample(sampler_llr, _857 * 2.0f);
    bool _934 = _838 ? _838 : true;
    bool _936 = _837 ? _837 : true;
    bool _938 = _934 ? _934 : true;
    bool _940 = _936 ? _936 : true;
    bool _942 = _938 ? _938 : true;
    bool _944 = _940 ? _940 : true;
    float3 _950 = _847.xxx;
    float3 _951 = lerp(_888, (_926 * 1.10000002384185791015625f) * _931.xyz, _950);
    float4 _954 = normal_map.Sample(sampler_llr, _857);
    float3 _961 = normalize(lerp(normalize(mul(in_var_C9, (_954.xyz * 2.0f) - 1.0f.xxx)), in_var_C1, _950));
    float3 _2157;
    if (_851 > 0.0f)
    {
        float2 _965 = in_var_C3 + float2(0.00999999977648258209228515625f, 0.0f);
        float2 _973;
        uint _975;
        uint _977;
        bool _979;
        bool _981;
        float _983;
        _973 = ((_965 * 1.0f) + 23.3999996185302734375f.xx) + _253;
        _975 = _944 ? (_940 ? (_936 ? (_837 ? _835 : 1013904223u) : 1013904223u) : 1013904223u) : 1013904223u;
        _977 = _942 ? (_938 ? (_934 ? (_838 ? _836 : 1664525u) : 1664525u) : 1664525u) : 1664525u;
        _979 = _944 ? _944 : true;
        _981 = _942 ? _942 : true;
        _983 = 0.0f;
        float _970 = 0.5f;
        int _985 = 0;
        for (; _985 < 3; )
        {
            int2 _990 = int2(floor(_973));
            float2 _991 = frac(_973);
            uint2 _992 = uint2(_990);
            uint _994 = _992.y;
            uint _996 = _994 + (_994 << 10u);
            uint _998 = _996 ^ (_996 >> 6u);
            uint _1000 = _998 + (_998 << 3u);
            uint _1002 = _1000 ^ (_1000 >> 11u);
            bool _1005 = _981 ? _981 : true;
            uint _1006 = _981 ? _977 : 1664525u;
            bool _1007 = _979 ? _979 : true;
            uint _1008 = _979 ? _975 : 1013904223u;
            uint _1012 = (((_992.x * _1006) + (_1002 + (_1002 << 15u))) + _1008) * _1006;
            uint _1014 = _1012 ^ (_1012 >> 11u);
            uint _1017 = _1014 ^ ((_1014 << 7u) & 2636928640u);
            uint _1020 = _1017 ^ ((_1017 << 15u) & 4022730752u);
            uint2 _1028 = uint2(_990 + int2(1, 0));
            uint _1030 = _1028.y;
            uint _1032 = _1030 + (_1030 << 10u);
            uint _1034 = _1032 ^ (_1032 >> 6u);
            uint _1036 = _1034 + (_1034 << 3u);
            uint _1038 = _1036 ^ (_1036 >> 11u);
            bool _1041 = _1005 ? _1005 : true;
            uint _1042 = _1005 ? _1006 : 1664525u;
            bool _1043 = _1007 ? _1007 : true;
            uint _1044 = _1007 ? _1008 : 1013904223u;
            uint _1048 = (((_1028.x * _1042) + (_1038 + (_1038 << 15u))) + _1044) * _1042;
            uint _1050 = _1048 ^ (_1048 >> 11u);
            uint _1053 = _1050 ^ ((_1050 << 7u) & 2636928640u);
            uint _1056 = _1053 ^ ((_1053 << 15u) & 4022730752u);
            uint2 _1064 = uint2(_990 + int2(0, 1));
            uint _1066 = _1064.y;
            uint _1068 = _1066 + (_1066 << 10u);
            uint _1070 = _1068 ^ (_1068 >> 6u);
            uint _1072 = _1070 + (_1070 << 3u);
            uint _1074 = _1072 ^ (_1072 >> 11u);
            bool _1077 = _1041 ? _1041 : true;
            uint _1078 = _1041 ? _1042 : 1664525u;
            bool _1079 = _1043 ? _1043 : true;
            uint _1080 = _1043 ? _1044 : 1013904223u;
            uint _1084 = (((_1064.x * _1078) + (_1074 + (_1074 << 15u))) + _1080) * _1078;
            uint _1086 = _1084 ^ (_1084 >> 11u);
            uint _1089 = _1086 ^ ((_1086 << 7u) & 2636928640u);
            uint _1092 = _1089 ^ ((_1089 << 15u) & 4022730752u);
            uint2 _1100 = uint2(_990 + int2(1, 1));
            uint _1102 = _1100.y;
            uint _1104 = _1102 + (_1102 << 10u);
            uint _1106 = _1104 ^ (_1104 >> 6u);
            uint _1108 = _1106 + (_1106 << 3u);
            uint _1110 = _1108 ^ (_1108 >> 11u);
            uint _978 = _1077 ? _1078 : 1664525u;
            uint _976 = _1079 ? _1080 : 1013904223u;
            uint _1116 = (((_1100.x * _978) + (_1110 + (_1110 << 15u))) + _976) * _978;
            uint _1118 = _1116 ^ (_1116 >> 11u);
            uint _1121 = _1118 ^ ((_1118 << 7u) & 2636928640u);
            uint _1124 = _1121 ^ ((_1121 << 15u) & 4022730752u);
            float2 _1134 = (_991 * _991) * (3.0f.xx - (_991 * 2.0f));
            float _1135 = _1134.x;
            float _1136 = 1.0f - _1135;
            float _1138 = _1134.y;
            float _1151 = _970 * ((-1.0f) + (2.0f * ((((1.0f - _1138) * (((asfloat(((_1020 ^ (_1020 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _1136) + ((asfloat(((_1056 ^ (_1056 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _1135))) + (((asfloat(((_1092 ^ (_1092 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _1136) * _1138)) + (((asfloat(((_1124 ^ (_1124 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _1135) * _1138))));
            _970 *= 0.550000011920928955078125f;
            _973 = mul(float2x2(float2(0.800000011920928955078125f, -0.60000002384185791015625f), float2(0.60000002384185791015625f, 0.800000011920928955078125f)), _973 * 1.89999997615814208984375f);
            _975 = _976;
            _977 = _978;
            _979 = _1079 ? _1079 : true;
            _981 = _1077 ? _1077 : true;
            _983 += _1151;
            _985++;
            continue;
        }
        float2 _1162;
        uint _1164;
        uint _1166;
        bool _1168;
        bool _1170;
        float _1172;
        _1162 = ((_965 * 0.5f) + 0.5f.xx) + _253;
        _1164 = _975;
        _1166 = _977;
        _1168 = _979;
        _1170 = _981;
        _1172 = 0.0f;
        float _1159 = 0.5f;
        int _1174 = 0;
        for (; _1174 < 5; )
        {
            int2 _1179 = int2(floor(_1162));
            float2 _1180 = frac(_1162);
            uint2 _1181 = uint2(_1179);
            uint _1183 = _1181.y;
            uint _1185 = _1183 + (_1183 << 10u);
            uint _1187 = _1185 ^ (_1185 >> 6u);
            uint _1189 = _1187 + (_1187 << 3u);
            uint _1191 = _1189 ^ (_1189 >> 11u);
            bool _1194 = _1170 ? _1170 : true;
            uint _1195 = _1170 ? _1166 : 1664525u;
            bool _1196 = _1168 ? _1168 : true;
            uint _1197 = _1168 ? _1164 : 1013904223u;
            uint _1201 = (((_1181.x * _1195) + (_1191 + (_1191 << 15u))) + _1197) * _1195;
            uint _1203 = _1201 ^ (_1201 >> 11u);
            uint _1206 = _1203 ^ ((_1203 << 7u) & 2636928640u);
            uint _1209 = _1206 ^ ((_1206 << 15u) & 4022730752u);
            uint2 _1217 = uint2(_1179 + int2(1, 0));
            uint _1219 = _1217.y;
            uint _1221 = _1219 + (_1219 << 10u);
            uint _1223 = _1221 ^ (_1221 >> 6u);
            uint _1225 = _1223 + (_1223 << 3u);
            uint _1227 = _1225 ^ (_1225 >> 11u);
            bool _1230 = _1194 ? _1194 : true;
            uint _1231 = _1194 ? _1195 : 1664525u;
            bool _1232 = _1196 ? _1196 : true;
            uint _1233 = _1196 ? _1197 : 1013904223u;
            uint _1237 = (((_1217.x * _1231) + (_1227 + (_1227 << 15u))) + _1233) * _1231;
            uint _1239 = _1237 ^ (_1237 >> 11u);
            uint _1242 = _1239 ^ ((_1239 << 7u) & 2636928640u);
            uint _1245 = _1242 ^ ((_1242 << 15u) & 4022730752u);
            uint2 _1253 = uint2(_1179 + int2(0, 1));
            uint _1255 = _1253.y;
            uint _1257 = _1255 + (_1255 << 10u);
            uint _1259 = _1257 ^ (_1257 >> 6u);
            uint _1261 = _1259 + (_1259 << 3u);
            uint _1263 = _1261 ^ (_1261 >> 11u);
            bool _1266 = _1230 ? _1230 : true;
            uint _1267 = _1230 ? _1231 : 1664525u;
            bool _1268 = _1232 ? _1232 : true;
            uint _1269 = _1232 ? _1233 : 1013904223u;
            uint _1273 = (((_1253.x * _1267) + (_1263 + (_1263 << 15u))) + _1269) * _1267;
            uint _1275 = _1273 ^ (_1273 >> 11u);
            uint _1278 = _1275 ^ ((_1275 << 7u) & 2636928640u);
            uint _1281 = _1278 ^ ((_1278 << 15u) & 4022730752u);
            uint2 _1289 = uint2(_1179 + int2(1, 1));
            uint _1291 = _1289.y;
            uint _1293 = _1291 + (_1291 << 10u);
            uint _1295 = _1293 ^ (_1293 >> 6u);
            uint _1297 = _1295 + (_1295 << 3u);
            uint _1299 = _1297 ^ (_1297 >> 11u);
            uint _1167 = _1266 ? _1267 : 1664525u;
            uint _1165 = _1268 ? _1269 : 1013904223u;
            uint _1305 = (((_1289.x * _1167) + (_1299 + (_1299 << 15u))) + _1165) * _1167;
            uint _1307 = _1305 ^ (_1305 >> 11u);
            uint _1310 = _1307 ^ ((_1307 << 7u) & 2636928640u);
            uint _1313 = _1310 ^ ((_1310 << 15u) & 4022730752u);
            float2 _1323 = (_1180 * _1180) * (3.0f.xx - (_1180 * 2.0f));
            float _1324 = _1323.x;
            float _1325 = 1.0f - _1324;
            float _1327 = _1323.y;
            float _1340 = _1159 * ((-1.0f) + (2.0f * ((((1.0f - _1327) * (((asfloat(((_1209 ^ (_1209 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _1325) + ((asfloat(((_1245 ^ (_1245 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _1324))) + (((asfloat(((_1281 ^ (_1281 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _1325) * _1327)) + (((asfloat(((_1313 ^ (_1313 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _1324) * _1327))));
            _1159 *= 0.550000011920928955078125f;
            _1162 = mul(float2x2(float2(0.800000011920928955078125f, -0.60000002384185791015625f), float2(0.60000002384185791015625f, 0.800000011920928955078125f)), _1162 * 1.89999997615814208984375f);
            _1164 = _1165;
            _1166 = _1167;
            _1168 = _1268 ? _1268 : true;
            _1170 = _1266 ? _1266 : true;
            _1172 += _1340;
            _1174++;
            continue;
        }
        float _1343 = _636 * (lerp(0.300000011920928955078125f, 1.0f, _983) * in_var_C7);
        float _1344 = (_254 + _1172) + _1343;
        uint _1541;
        uint _1542;
        bool _1543;
        bool _1544;
        float _1545;
        if (_639)
        {
            float2 _1358;
            uint _1360;
            uint _1362;
            bool _1364;
            bool _1366;
            float _1368;
            _1358 = ((float2(nani_instance_data.Load<float>(in_var_C0 * 48 + 20) - _965.x, _965.y) * 0.5f) + 0.5f.xx) + _253;
            _1360 = _1164;
            _1362 = _1166;
            _1364 = _1168;
            _1366 = _1170;
            _1368 = 0.0f;
            float _1355 = 0.5f;
            int _1370 = 0;
            for (; _1370 < 5; )
            {
                int2 _1375 = int2(floor(_1358));
                float2 _1376 = frac(_1358);
                uint2 _1377 = uint2(_1375);
                uint _1379 = _1377.y;
                uint _1381 = _1379 + (_1379 << 10u);
                uint _1383 = _1381 ^ (_1381 >> 6u);
                uint _1385 = _1383 + (_1383 << 3u);
                uint _1387 = _1385 ^ (_1385 >> 11u);
                bool _1390 = _1366 ? _1366 : true;
                uint _1391 = _1366 ? _1362 : 1664525u;
                bool _1392 = _1364 ? _1364 : true;
                uint _1393 = _1364 ? _1360 : 1013904223u;
                uint _1397 = (((_1377.x * _1391) + (_1387 + (_1387 << 15u))) + _1393) * _1391;
                uint _1399 = _1397 ^ (_1397 >> 11u);
                uint _1402 = _1399 ^ ((_1399 << 7u) & 2636928640u);
                uint _1405 = _1402 ^ ((_1402 << 15u) & 4022730752u);
                uint2 _1413 = uint2(_1375 + int2(1, 0));
                uint _1415 = _1413.y;
                uint _1417 = _1415 + (_1415 << 10u);
                uint _1419 = _1417 ^ (_1417 >> 6u);
                uint _1421 = _1419 + (_1419 << 3u);
                uint _1423 = _1421 ^ (_1421 >> 11u);
                bool _1426 = _1390 ? _1390 : true;
                uint _1427 = _1390 ? _1391 : 1664525u;
                bool _1428 = _1392 ? _1392 : true;
                uint _1429 = _1392 ? _1393 : 1013904223u;
                uint _1433 = (((_1413.x * _1427) + (_1423 + (_1423 << 15u))) + _1429) * _1427;
                uint _1435 = _1433 ^ (_1433 >> 11u);
                uint _1438 = _1435 ^ ((_1435 << 7u) & 2636928640u);
                uint _1441 = _1438 ^ ((_1438 << 15u) & 4022730752u);
                uint2 _1449 = uint2(_1375 + int2(0, 1));
                uint _1451 = _1449.y;
                uint _1453 = _1451 + (_1451 << 10u);
                uint _1455 = _1453 ^ (_1453 >> 6u);
                uint _1457 = _1455 + (_1455 << 3u);
                uint _1459 = _1457 ^ (_1457 >> 11u);
                bool _1462 = _1426 ? _1426 : true;
                uint _1463 = _1426 ? _1427 : 1664525u;
                bool _1464 = _1428 ? _1428 : true;
                uint _1465 = _1428 ? _1429 : 1013904223u;
                uint _1469 = (((_1449.x * _1463) + (_1459 + (_1459 << 15u))) + _1465) * _1463;
                uint _1471 = _1469 ^ (_1469 >> 11u);
                uint _1474 = _1471 ^ ((_1471 << 7u) & 2636928640u);
                uint _1477 = _1474 ^ ((_1474 << 15u) & 4022730752u);
                uint2 _1485 = uint2(_1375 + int2(1, 1));
                uint _1487 = _1485.y;
                uint _1489 = _1487 + (_1487 << 10u);
                uint _1491 = _1489 ^ (_1489 >> 6u);
                uint _1493 = _1491 + (_1491 << 3u);
                uint _1495 = _1493 ^ (_1493 >> 11u);
                uint _1363 = _1462 ? _1463 : 1664525u;
                uint _1361 = _1464 ? _1465 : 1013904223u;
                uint _1501 = (((_1485.x * _1363) + (_1495 + (_1495 << 15u))) + _1361) * _1363;
                uint _1503 = _1501 ^ (_1501 >> 11u);
                uint _1506 = _1503 ^ ((_1503 << 7u) & 2636928640u);
                uint _1509 = _1506 ^ ((_1506 << 15u) & 4022730752u);
                float2 _1519 = (_1376 * _1376) * (3.0f.xx - (_1376 * 2.0f));
                float _1520 = _1519.x;
                float _1521 = 1.0f - _1520;
                float _1523 = _1519.y;
                float _1536 = _1355 * ((-1.0f) + (2.0f * ((((1.0f - _1523) * (((asfloat(((_1405 ^ (_1405 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _1521) + ((asfloat(((_1441 ^ (_1441 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _1520))) + (((asfloat(((_1477 ^ (_1477 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _1521) * _1523)) + (((asfloat(((_1509 ^ (_1509 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _1520) * _1523))));
                _1355 *= 0.550000011920928955078125f;
                _1358 = mul(float2x2(float2(0.800000011920928955078125f, -0.60000002384185791015625f), float2(0.60000002384185791015625f, 0.800000011920928955078125f)), _1358 * 1.89999997615814208984375f);
                _1360 = _1361;
                _1362 = _1363;
                _1364 = _1464 ? _1464 : true;
                _1366 = _1462 ? _1462 : true;
                _1368 += _1536;
                _1370++;
                continue;
            }
            _1541 = _1360;
            _1542 = _1362;
            _1543 = _1364;
            _1544 = _1366;
            _1545 = lerp(_1344, (_254 + _1368) + _1343, _249);
        }
        else
        {
            _1541 = _1164;
            _1542 = _1166;
            _1543 = _1168;
            _1544 = _1170;
            _1545 = _1344;
        }
        float _1550 = min((smoothstep(0.0f, 4.0f, _965.y) * 1.5f) - 0.20000000298023223876953125f, 0.89999997615814208984375f);
        float _1551 = _1550 + 0.100000001490116119384765625f;
        float _1552 = _1545 * 2.0f;
        float2 _1559 = in_var_C3 + float2(0.0f, 0.00999999977648258209228515625f);
        float2 _1567;
        uint _1569;
        uint _1571;
        bool _1573;
        bool _1575;
        float _1577;
        _1567 = ((_1559 * 1.0f) + 23.3999996185302734375f.xx) + _253;
        _1569 = _1541;
        _1571 = _1542;
        _1573 = _1543;
        _1575 = _1544;
        _1577 = 0.0f;
        float _1564 = 0.5f;
        int _1579 = 0;
        for (; _1579 < 3; )
        {
            int2 _1584 = int2(floor(_1567));
            float2 _1585 = frac(_1567);
            uint2 _1586 = uint2(_1584);
            uint _1588 = _1586.y;
            uint _1590 = _1588 + (_1588 << 10u);
            uint _1592 = _1590 ^ (_1590 >> 6u);
            uint _1594 = _1592 + (_1592 << 3u);
            uint _1596 = _1594 ^ (_1594 >> 11u);
            bool _1599 = _1575 ? _1575 : true;
            uint _1600 = _1575 ? _1571 : 1664525u;
            bool _1601 = _1573 ? _1573 : true;
            uint _1602 = _1573 ? _1569 : 1013904223u;
            uint _1606 = (((_1586.x * _1600) + (_1596 + (_1596 << 15u))) + _1602) * _1600;
            uint _1608 = _1606 ^ (_1606 >> 11u);
            uint _1611 = _1608 ^ ((_1608 << 7u) & 2636928640u);
            uint _1614 = _1611 ^ ((_1611 << 15u) & 4022730752u);
            uint2 _1622 = uint2(_1584 + int2(1, 0));
            uint _1624 = _1622.y;
            uint _1626 = _1624 + (_1624 << 10u);
            uint _1628 = _1626 ^ (_1626 >> 6u);
            uint _1630 = _1628 + (_1628 << 3u);
            uint _1632 = _1630 ^ (_1630 >> 11u);
            bool _1635 = _1599 ? _1599 : true;
            uint _1636 = _1599 ? _1600 : 1664525u;
            bool _1637 = _1601 ? _1601 : true;
            uint _1638 = _1601 ? _1602 : 1013904223u;
            uint _1642 = (((_1622.x * _1636) + (_1632 + (_1632 << 15u))) + _1638) * _1636;
            uint _1644 = _1642 ^ (_1642 >> 11u);
            uint _1647 = _1644 ^ ((_1644 << 7u) & 2636928640u);
            uint _1650 = _1647 ^ ((_1647 << 15u) & 4022730752u);
            uint2 _1658 = uint2(_1584 + int2(0, 1));
            uint _1660 = _1658.y;
            uint _1662 = _1660 + (_1660 << 10u);
            uint _1664 = _1662 ^ (_1662 >> 6u);
            uint _1666 = _1664 + (_1664 << 3u);
            uint _1668 = _1666 ^ (_1666 >> 11u);
            bool _1671 = _1635 ? _1635 : true;
            uint _1672 = _1635 ? _1636 : 1664525u;
            bool _1673 = _1637 ? _1637 : true;
            uint _1674 = _1637 ? _1638 : 1013904223u;
            uint _1678 = (((_1658.x * _1672) + (_1668 + (_1668 << 15u))) + _1674) * _1672;
            uint _1680 = _1678 ^ (_1678 >> 11u);
            uint _1683 = _1680 ^ ((_1680 << 7u) & 2636928640u);
            uint _1686 = _1683 ^ ((_1683 << 15u) & 4022730752u);
            uint2 _1694 = uint2(_1584 + int2(1, 1));
            uint _1696 = _1694.y;
            uint _1698 = _1696 + (_1696 << 10u);
            uint _1700 = _1698 ^ (_1698 >> 6u);
            uint _1702 = _1700 + (_1700 << 3u);
            uint _1704 = _1702 ^ (_1702 >> 11u);
            uint _1572 = _1671 ? _1672 : 1664525u;
            uint _1570 = _1673 ? _1674 : 1013904223u;
            uint _1710 = (((_1694.x * _1572) + (_1704 + (_1704 << 15u))) + _1570) * _1572;
            uint _1712 = _1710 ^ (_1710 >> 11u);
            uint _1715 = _1712 ^ ((_1712 << 7u) & 2636928640u);
            uint _1718 = _1715 ^ ((_1715 << 15u) & 4022730752u);
            float2 _1728 = (_1585 * _1585) * (3.0f.xx - (_1585 * 2.0f));
            float _1729 = _1728.x;
            float _1730 = 1.0f - _1729;
            float _1732 = _1728.y;
            float _1745 = _1564 * ((-1.0f) + (2.0f * ((((1.0f - _1732) * (((asfloat(((_1614 ^ (_1614 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _1730) + ((asfloat(((_1650 ^ (_1650 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _1729))) + (((asfloat(((_1686 ^ (_1686 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _1730) * _1732)) + (((asfloat(((_1718 ^ (_1718 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _1729) * _1732))));
            _1564 *= 0.550000011920928955078125f;
            _1567 = mul(float2x2(float2(0.800000011920928955078125f, -0.60000002384185791015625f), float2(0.60000002384185791015625f, 0.800000011920928955078125f)), _1567 * 1.89999997615814208984375f);
            _1569 = _1570;
            _1571 = _1572;
            _1573 = _1673 ? _1673 : true;
            _1575 = _1671 ? _1671 : true;
            _1577 += _1745;
            _1579++;
            continue;
        }
        float2 _1756;
        uint _1758;
        uint _1760;
        bool _1762;
        bool _1764;
        float _1766;
        _1756 = ((_1559 * 0.5f) + 0.5f.xx) + _253;
        _1758 = _1569;
        _1760 = _1571;
        _1762 = _1573;
        _1764 = _1575;
        _1766 = 0.0f;
        float _1753 = 0.5f;
        int _1768 = 0;
        for (; _1768 < 5; )
        {
            int2 _1773 = int2(floor(_1756));
            float2 _1774 = frac(_1756);
            uint2 _1775 = uint2(_1773);
            uint _1777 = _1775.y;
            uint _1779 = _1777 + (_1777 << 10u);
            uint _1781 = _1779 ^ (_1779 >> 6u);
            uint _1783 = _1781 + (_1781 << 3u);
            uint _1785 = _1783 ^ (_1783 >> 11u);
            bool _1788 = _1764 ? _1764 : true;
            uint _1789 = _1764 ? _1760 : 1664525u;
            bool _1790 = _1762 ? _1762 : true;
            uint _1791 = _1762 ? _1758 : 1013904223u;
            uint _1795 = (((_1775.x * _1789) + (_1785 + (_1785 << 15u))) + _1791) * _1789;
            uint _1797 = _1795 ^ (_1795 >> 11u);
            uint _1800 = _1797 ^ ((_1797 << 7u) & 2636928640u);
            uint _1803 = _1800 ^ ((_1800 << 15u) & 4022730752u);
            uint2 _1811 = uint2(_1773 + int2(1, 0));
            uint _1813 = _1811.y;
            uint _1815 = _1813 + (_1813 << 10u);
            uint _1817 = _1815 ^ (_1815 >> 6u);
            uint _1819 = _1817 + (_1817 << 3u);
            uint _1821 = _1819 ^ (_1819 >> 11u);
            bool _1824 = _1788 ? _1788 : true;
            uint _1825 = _1788 ? _1789 : 1664525u;
            bool _1826 = _1790 ? _1790 : true;
            uint _1827 = _1790 ? _1791 : 1013904223u;
            uint _1831 = (((_1811.x * _1825) + (_1821 + (_1821 << 15u))) + _1827) * _1825;
            uint _1833 = _1831 ^ (_1831 >> 11u);
            uint _1836 = _1833 ^ ((_1833 << 7u) & 2636928640u);
            uint _1839 = _1836 ^ ((_1836 << 15u) & 4022730752u);
            uint2 _1847 = uint2(_1773 + int2(0, 1));
            uint _1849 = _1847.y;
            uint _1851 = _1849 + (_1849 << 10u);
            uint _1853 = _1851 ^ (_1851 >> 6u);
            uint _1855 = _1853 + (_1853 << 3u);
            uint _1857 = _1855 ^ (_1855 >> 11u);
            bool _1860 = _1824 ? _1824 : true;
            uint _1861 = _1824 ? _1825 : 1664525u;
            bool _1862 = _1826 ? _1826 : true;
            uint _1863 = _1826 ? _1827 : 1013904223u;
            uint _1867 = (((_1847.x * _1861) + (_1857 + (_1857 << 15u))) + _1863) * _1861;
            uint _1869 = _1867 ^ (_1867 >> 11u);
            uint _1872 = _1869 ^ ((_1869 << 7u) & 2636928640u);
            uint _1875 = _1872 ^ ((_1872 << 15u) & 4022730752u);
            uint2 _1883 = uint2(_1773 + int2(1, 1));
            uint _1885 = _1883.y;
            uint _1887 = _1885 + (_1885 << 10u);
            uint _1889 = _1887 ^ (_1887 >> 6u);
            uint _1891 = _1889 + (_1889 << 3u);
            uint _1893 = _1891 ^ (_1891 >> 11u);
            uint _1761 = _1860 ? _1861 : 1664525u;
            uint _1759 = _1862 ? _1863 : 1013904223u;
            uint _1899 = (((_1883.x * _1761) + (_1893 + (_1893 << 15u))) + _1759) * _1761;
            uint _1901 = _1899 ^ (_1899 >> 11u);
            uint _1904 = _1901 ^ ((_1901 << 7u) & 2636928640u);
            uint _1907 = _1904 ^ ((_1904 << 15u) & 4022730752u);
            float2 _1917 = (_1774 * _1774) * (3.0f.xx - (_1774 * 2.0f));
            float _1918 = _1917.x;
            float _1919 = 1.0f - _1918;
            float _1921 = _1917.y;
            float _1934 = _1753 * ((-1.0f) + (2.0f * ((((1.0f - _1921) * (((asfloat(((_1803 ^ (_1803 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _1919) + ((asfloat(((_1839 ^ (_1839 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _1918))) + (((asfloat(((_1875 ^ (_1875 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _1919) * _1921)) + (((asfloat(((_1907 ^ (_1907 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _1918) * _1921))));
            _1753 *= 0.550000011920928955078125f;
            _1756 = mul(float2x2(float2(0.800000011920928955078125f, -0.60000002384185791015625f), float2(0.60000002384185791015625f, 0.800000011920928955078125f)), _1756 * 1.89999997615814208984375f);
            _1758 = _1759;
            _1760 = _1761;
            _1762 = _1862 ? _1862 : true;
            _1764 = _1860 ? _1860 : true;
            _1766 += _1934;
            _1768++;
            continue;
        }
        float _1937 = _636 * (lerp(0.300000011920928955078125f, 1.0f, _1577) * in_var_C7);
        float _1938 = (_254 + _1766) + _1937;
        float _2135;
        if (_639)
        {
            float2 _1952;
            float _1954;
            _1952 = ((float2(nani_instance_data.Load<float>(in_var_C0 * 48 + 20) - _1559.x, _1559.y) * 0.5f) + 0.5f.xx) + _253;
            _1954 = 0.0f;
            float _1949 = 0.5f;
            int _1956 = 0;
            uint _1958 = _1758;
            uint _1960 = _1760;
            bool _1962 = _1762;
            bool _1964 = _1764;
            for (; _1956 < 5; )
            {
                int2 _1969 = int2(floor(_1952));
                float2 _1970 = frac(_1952);
                uint2 _1971 = uint2(_1969);
                uint _1973 = _1971.y;
                uint _1975 = _1973 + (_1973 << 10u);
                uint _1977 = _1975 ^ (_1975 >> 6u);
                uint _1979 = _1977 + (_1977 << 3u);
                uint _1981 = _1979 ^ (_1979 >> 11u);
                bool _1984 = _1964 ? _1964 : true;
                uint _1985 = _1964 ? _1960 : 1664525u;
                bool _1986 = _1962 ? _1962 : true;
                uint _1987 = _1962 ? _1958 : 1013904223u;
                uint _1991 = (((_1971.x * _1985) + (_1981 + (_1981 << 15u))) + _1987) * _1985;
                uint _1993 = _1991 ^ (_1991 >> 11u);
                uint _1996 = _1993 ^ ((_1993 << 7u) & 2636928640u);
                uint _1999 = _1996 ^ ((_1996 << 15u) & 4022730752u);
                uint2 _2007 = uint2(_1969 + int2(1, 0));
                uint _2009 = _2007.y;
                uint _2011 = _2009 + (_2009 << 10u);
                uint _2013 = _2011 ^ (_2011 >> 6u);
                uint _2015 = _2013 + (_2013 << 3u);
                uint _2017 = _2015 ^ (_2015 >> 11u);
                bool _2020 = _1984 ? _1984 : true;
                uint _2021 = _1984 ? _1985 : 1664525u;
                bool _2022 = _1986 ? _1986 : true;
                uint _2023 = _1986 ? _1987 : 1013904223u;
                uint _2027 = (((_2007.x * _2021) + (_2017 + (_2017 << 15u))) + _2023) * _2021;
                uint _2029 = _2027 ^ (_2027 >> 11u);
                uint _2032 = _2029 ^ ((_2029 << 7u) & 2636928640u);
                uint _2035 = _2032 ^ ((_2032 << 15u) & 4022730752u);
                uint2 _2043 = uint2(_1969 + int2(0, 1));
                uint _2045 = _2043.y;
                uint _2047 = _2045 + (_2045 << 10u);
                uint _2049 = _2047 ^ (_2047 >> 6u);
                uint _2051 = _2049 + (_2049 << 3u);
                uint _2053 = _2051 ^ (_2051 >> 11u);
                bool _2056 = _2020 ? _2020 : true;
                uint _2057 = _2020 ? _2021 : 1664525u;
                bool _2058 = _2022 ? _2022 : true;
                uint _2059 = _2022 ? _2023 : 1013904223u;
                uint _2063 = (((_2043.x * _2057) + (_2053 + (_2053 << 15u))) + _2059) * _2057;
                uint _2065 = _2063 ^ (_2063 >> 11u);
                uint _2068 = _2065 ^ ((_2065 << 7u) & 2636928640u);
                uint _2071 = _2068 ^ ((_2068 << 15u) & 4022730752u);
                uint2 _2079 = uint2(_1969 + int2(1, 1));
                uint _2081 = _2079.y;
                uint _2083 = _2081 + (_2081 << 10u);
                uint _2085 = _2083 ^ (_2083 >> 6u);
                uint _2087 = _2085 + (_2085 << 3u);
                uint _2089 = _2087 ^ (_2087 >> 11u);
                uint _1961 = _2056 ? _2057 : 1664525u;
                uint _1959 = _2058 ? _2059 : 1013904223u;
                uint _2095 = (((_2079.x * _1961) + (_2089 + (_2089 << 15u))) + _1959) * _1961;
                uint _2097 = _2095 ^ (_2095 >> 11u);
                uint _2100 = _2097 ^ ((_2097 << 7u) & 2636928640u);
                uint _2103 = _2100 ^ ((_2100 << 15u) & 4022730752u);
                float2 _2113 = (_1970 * _1970) * (3.0f.xx - (_1970 * 2.0f));
                float _2114 = _2113.x;
                float _2115 = 1.0f - _2114;
                float _2117 = _2113.y;
                float _2130 = _1949 * ((-1.0f) + (2.0f * ((((1.0f - _2117) * (((asfloat(((_1999 ^ (_1999 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _2115) + ((asfloat(((_2035 ^ (_2035 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _2114))) + (((asfloat(((_2071 ^ (_2071 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _2115) * _2117)) + (((asfloat(((_2103 ^ (_2103 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _2114) * _2117))));
                _1949 *= 0.550000011920928955078125f;
                _1952 = mul(float2x2(float2(0.800000011920928955078125f, -0.60000002384185791015625f), float2(0.60000002384185791015625f, 0.800000011920928955078125f)), _1952 * 1.89999997615814208984375f);
                _1954 += _2130;
                _1956++;
                _1958 = _1959;
                _1960 = _1961;
                _1962 = _2058 ? _2058 : true;
                _1964 = _2056 ? _2056 : true;
                continue;
            }
            _2135 = lerp(_1938, (_254 + _1954) + _1937, _249);
        }
        else
        {
            _2135 = _1938;
        }
        float _2140 = min((smoothstep(0.0f, 4.0f, _1559.y) * 1.5f) - 0.20000000298023223876953125f, 0.89999997615814208984375f);
        float _2141 = _2140 + 0.100000001490116119384765625f;
        float _2142 = _2135 * 2.0f;
        _2157 = lerp(_961, normalize((in_var_C4 * (clamp(smoothstep(_1550 - 0.100000001490116119384765625f, _1551, _1552) - smoothstep(_1550, _1551, _1552), 0.0f, 1.0f) - _851)) + (in_var_C5 * (-(clamp(smoothstep(_2140 - 0.100000001490116119384765625f, _2141, _2142) - smoothstep(_2140, _2141, _2142), 0.0f, 1.0f) - _851)))), (_847 * 0.300000011920928955078125f).xxx);
    }
    else
    {
        _2157 = _961;
    }
    float3 _2169;
    bool _2170;
    if (_223 == 1u)
    {
        _2169 = lerp(_951, float3(0.699999988079071044921875f, 0.2100000083446502685546875f, 0.0f), 0.5f.xxx);
        _2170 = true;
    }
    else
    {
        bool _2162 = _223 == 2u;
        float3 _2166;
        if (_2162)
        {
            _2166 = lerp(_951, 0.5f.xxx, 0.100000001490116119384765625f.xxx);
        }
        else
        {
            _2166 = _951;
        }
        _2169 = _2166;
        _2170 = _2162 ? true : false;
    }
    float3 _2191;
    bool _2192;
    if (highlight_bricks_to_delete == 1)
    {
        bool _2185 = distance(brush_position.xz, in_var_C2.xz) < ((1.88900005817413330078125f * eraser_mult) - 0.2700000107288360595703125f);
        float3 _2189;
        if (_2185)
        {
            _2189 = lerp(_951, float3(0.699999988079071044921875f, 0.2100000083446502685546875f, 0.0f), 0.800000011920928955078125f.xxx);
        }
        else
        {
            _2189 = _2169;
        }
        _2191 = _2189;
        _2192 = _2185 ? true : _2170;
    }
    else
    {
        _2191 = _2169;
        _2192 = _2170;
    }
    float3 _2194 = (_2157 * 0.5f) + 0.5f.xxx;
    float3 _2216 = sqrt(clamp(_2191, 0.0f.xxx, 1.0f.xxx));
    uint _2235 = (uint((clamp(_2216.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_2216.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_2216.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u);
    uint2 _2236 = uint2(((uint((clamp(_2194.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_2194.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_2194.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 3422552064u, _2235);
    _2236.y = _2235 | (((((7u | (uint(_2192) << 3u)) | 0u) | 0u) | 0u) << 24u);
    out_var_SV_TARGET0 = _2236;
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C0 = stage_input.in_var_C0;
    in_var_C1 = stage_input.in_var_C1;
    in_var_C2 = stage_input.in_var_C2;
    in_var_C3 = stage_input.in_var_C3;
    in_var_C4 = stage_input.in_var_C4;
    in_var_C5 = stage_input.in_var_C5;
    in_var_C6 = stage_input.in_var_C6;
    in_var_C7 = stage_input.in_var_C7;
    in_var_C8 = stage_input.in_var_C8;
    in_var_C9 = stage_input.in_var_C9;
    frag_main();
    SPIRV_Cross_Output stage_output;
    stage_output.out_var_SV_TARGET0 = out_var_SV_TARGET0;
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

cbuffer $Globals : register(b13, space0)
{
    uint current_view_index : packoffset(c0);
    int highlight_bricks_to_delete : packoffset(c0.y);
    float3 brush_position : packoffset(c1);
    float eraser_mult : packoffset(c1.w);
    uint highlight_mode : packoffset(c2);
    uint highlighted_wall_id : packoffset(c2.y);
    uint2 normal_map_dims : packoffset(c2.z);
    uint2 mortar_dims : packoffset(c3);
};

ByteAddressBuffer instance_indirection_buffer : register(t18, space0);
ByteAddressBuffer nani_instance_data : register(t19, space0);

static float4 gl_Position;
static int gl_VertexIndex;
static int gl_InstanceIndex;
static uint out_var_C0;
static float3 out_var_C1;
static float3 out_var_C2;
static float2 out_var_C3;
static float3 out_var_C4;
static float3 out_var_C5;
static float out_var_C6;
static float out_var_C7;
static float out_var_C8;
static float3x3 out_var_C9;

struct SPIRV_Cross_Input
{
    uint gl_VertexIndex : SV_VertexID;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] nointerpolation uint out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float2 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float3 out_var_C4 : TEXCOORD4;
    [[vk::location(5)]] float3 out_var_C5 : TEXCOORD5;
    [[vk::location(6)]] float out_var_C6 : TEXCOORD6;
    [[vk::location(7)]] float out_var_C7 : TEXCOORD7;
    [[vk::location(8)]] float out_var_C8 : TEXCOORD8;
    [[vk::location(9)]] float3x3 out_var_C9 : TEXCOORD9;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    float4 _958;
    uint _959;
    float3 _960;
    float3 _961;
    float2 _962;
    float3 _963;
    float3 _964;
    float _965;
    float _966;
    float _967;
    float3x3 _968;
    do
    {
        uint _193 = uint(gl_VertexIndex) & 65535u;
        if (_193 > nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 48 + 28))
        {
            _958 = asfloat(0x7fbfffffu          ).xxxx;
            _959 = 0u;
            _960 = asfloat(0x7fbfffffu          ).xxx;
            _961 = asfloat(0x7fbfffffu          ).xxx;
            _962 = asfloat(0x7fbfffffu          ).xx;
            _963 = asfloat(0x7fbfffffu          ).xxx;
            _964 = asfloat(0x7fbfffffu          ).xxx;
            _965 = asfloat(0x7fbfffffu          );
            _966 = asfloat(0x7fbfffffu          );
            _967 = asfloat(0x7fbfffffu          );
            _968 = float3x3(asfloat(0x7fbfffffu          ).xxx, asfloat(0x7fbfffffu          ).xxx, asfloat(0x7fbfffffu          ).xxx);
            break;
        }
        uint2 _174 = nani_instance_data.Load<uint2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 48 + 32);
        uint _201 = 48u * (_193 + ((uint(gl_VertexIndex) >> 16u) * uint(nani_instance_data.Load<int>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 48 + 40))));
        int _202 = int(_201);
        uint2 _172;
        if (_202 >= 0)
        {
            _172.x = _174.x + uint(_202);
            _172.y = _174.y + uint(int(_172.x < _174.x));
        }
        else
        {
            _172.x = _174.x + uint(_202);
            _172.y = _174.y + uint((_172.x > _174.x) ? (-1) : 0);
        }
        uint4 _237 = vk::RawBufferLoad<uint4>(((uint64_t)_172.y << 32) | _172.x, 16);
        uint4 _173 = _237;
        uint2 _176 = nani_instance_data.Load<uint2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 48 + 32);
        int _239 = int(16u + _201);
        uint2 _171;
        if (_239 >= 0)
        {
            _171.x = _176.x + uint(_239);
            _171.y = _176.y + uint(int(_171.x < _176.x));
        }
        else
        {
            _171.x = _176.x + uint(_239);
            _171.y = _176.y + uint((_171.x > _176.x) ? (-1) : 0);
        }
        uint4 _274 = vk::RawBufferLoad<uint4>(((uint64_t)_171.y << 32) | _171.x, 16);
        uint4 _175 = _274;
        uint2 _177 = nani_instance_data.Load<uint2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 48 + 32);
        int _276 = int(32u + _201);
        uint2 _170;
        if (_276 >= 0)
        {
            _170.x = _177.x + uint(_276);
            _170.y = _177.y + uint(int(_170.x < _177.x));
        }
        else
        {
            _170.x = _177.x + uint(_276);
            _170.y = _177.y + uint((_170.x > _177.x) ? (-1) : 0);
        }
        uint2 _311 = vk::RawBufferLoad<uint2>(((uint64_t)_170.y << 32) | _170.x, 8);
        float3 _318 = asfloat(_274.xyz);
        float4 _326 = float4(asfloat(_237.xyz), 1.0f);
        float4 _178 = _326;
        float3 _328 = _326.xyz * 0.4000000059604644775390625f;
        int3 _330 = int3(floor(_328));
        float3 _331 = frac(_328);
        uint3 _332 = uint3(_330);
        uint3 _167 = _332;
        uint2 _166 = _332.yz;
        uint _341 = _166.y + (_166.y << 10u);
        uint _343 = _341 ^ (_341 >> 6u);
        uint _345 = _343 + (_343 << 3u);
        uint _347 = _345 ^ (_345 >> 11u);
        uint _353 = (((_166.x * 1664525u) + (_347 + (_347 << 15u))) + 1013904223u) * 1664525u;
        uint _355 = _353 ^ (_353 >> 11u);
        uint _358 = _355 ^ ((_355 << 7u) & 2636928640u);
        uint _361 = _358 ^ ((_358 << 15u) & 4022730752u);
        uint _367 = (((_167.x * 1664525u) + (_361 ^ (_361 >> 18u))) + 1013904223u) * 1664525u;
        uint _369 = _367 ^ (_367 >> 11u);
        uint _372 = _369 ^ ((_369 << 7u) & 2636928640u);
        uint _375 = _372 ^ ((_372 << 15u) & 4022730752u);
        uint _377 = _375 ^ (_375 >> 18u);
        uint _379 = _377 + (_377 << 10u);
        uint _381 = _379 ^ (_379 >> 6u);
        uint _383 = _381 + (_381 << 3u);
        uint _385 = _383 ^ (_383 >> 11u);
        float2 _165;
        _165.x = asfloat((_377 & 8388607u) | 1065353216u) - 1.0f;
        _165.y = asfloat(((_385 + (_385 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
        uint3 _399 = uint3(_330 + int3(1, 0, 0));
        _167 = _399;
        _166 = _399.yz;
        uint _405 = _166.y + (_166.y << 10u);
        uint _407 = _405 ^ (_405 >> 6u);
        uint _409 = _407 + (_407 << 3u);
        uint _411 = _409 ^ (_409 >> 11u);
        uint _417 = (((_166.x * 1664525u) + (_411 + (_411 << 15u))) + 1013904223u) * 1664525u;
        uint _419 = _417 ^ (_417 >> 11u);
        uint _422 = _419 ^ ((_419 << 7u) & 2636928640u);
        uint _425 = _422 ^ ((_422 << 15u) & 4022730752u);
        uint _431 = (((_167.x * 1664525u) + (_425 ^ (_425 >> 18u))) + 1013904223u) * 1664525u;
        uint _433 = _431 ^ (_431 >> 11u);
        uint _436 = _433 ^ ((_433 << 7u) & 2636928640u);
        uint _439 = _436 ^ ((_436 << 15u) & 4022730752u);
        uint _441 = _439 ^ (_439 >> 18u);
        uint _443 = _441 + (_441 << 10u);
        uint _445 = _443 ^ (_443 >> 6u);
        uint _447 = _445 + (_445 << 3u);
        uint _449 = _447 ^ (_447 >> 11u);
        float2 _164;
        _164.x = asfloat((_441 & 8388607u) | 1065353216u) - 1.0f;
        _164.y = asfloat(((_449 + (_449 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
        uint3 _463 = uint3(_330 + int3(0, 1, 0));
        _167 = _463;
        _166 = _463.yz;
        uint _469 = _166.y + (_166.y << 10u);
        uint _471 = _469 ^ (_469 >> 6u);
        uint _473 = _471 + (_471 << 3u);
        uint _475 = _473 ^ (_473 >> 11u);
        uint _481 = (((_166.x * 1664525u) + (_475 + (_475 << 15u))) + 1013904223u) * 1664525u;
        uint _483 = _481 ^ (_481 >> 11u);
        uint _486 = _483 ^ ((_483 << 7u) & 2636928640u);
        uint _489 = _486 ^ ((_486 << 15u) & 4022730752u);
        uint _495 = (((_167.x * 1664525u) + (_489 ^ (_489 >> 18u))) + 1013904223u) * 1664525u;
        uint _497 = _495 ^ (_495 >> 11u);
        uint _500 = _497 ^ ((_497 << 7u) & 2636928640u);
        uint _503 = _500 ^ ((_500 << 15u) & 4022730752u);
        uint _505 = _503 ^ (_503 >> 18u);
        uint _507 = _505 + (_505 << 10u);
        uint _509 = _507 ^ (_507 >> 6u);
        uint _511 = _509 + (_509 << 3u);
        uint _513 = _511 ^ (_511 >> 11u);
        float2 _163;
        _163.x = asfloat((_505 & 8388607u) | 1065353216u) - 1.0f;
        _163.y = asfloat(((_513 + (_513 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
        uint3 _527 = uint3(_330 + int3(1, 1, 0));
        _167 = _527;
        _166 = _527.yz;
        uint _533 = _166.y + (_166.y << 10u);
        uint _535 = _533 ^ (_533 >> 6u);
        uint _537 = _535 + (_535 << 3u);
        uint _539 = _537 ^ (_537 >> 11u);
        uint _545 = (((_166.x * 1664525u) + (_539 + (_539 << 15u))) + 1013904223u) * 1664525u;
        uint _547 = _545 ^ (_545 >> 11u);
        uint _550 = _547 ^ ((_547 << 7u) & 2636928640u);
        uint _553 = _550 ^ ((_550 << 15u) & 4022730752u);
        uint _559 = (((_167.x * 1664525u) + (_553 ^ (_553 >> 18u))) + 1013904223u) * 1664525u;
        uint _561 = _559 ^ (_559 >> 11u);
        uint _564 = _561 ^ ((_561 << 7u) & 2636928640u);
        uint _567 = _564 ^ ((_564 << 15u) & 4022730752u);
        uint _569 = _567 ^ (_567 >> 18u);
        uint _571 = _569 + (_569 << 10u);
        uint _573 = _571 ^ (_571 >> 6u);
        uint _575 = _573 + (_573 << 3u);
        uint _577 = _575 ^ (_575 >> 11u);
        float2 _162;
        _162.x = asfloat((_569 & 8388607u) | 1065353216u) - 1.0f;
        _162.y = asfloat(((_577 + (_577 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
        uint3 _591 = uint3(_330 + int3(0, 0, 1));
        _167 = _591;
        _166 = _591.yz;
        uint _597 = _166.y + (_166.y << 10u);
        uint _599 = _597 ^ (_597 >> 6u);
        uint _601 = _599 + (_599 << 3u);
        uint _603 = _601 ^ (_601 >> 11u);
        uint _609 = (((_166.x * 1664525u) + (_603 + (_603 << 15u))) + 1013904223u) * 1664525u;
        uint _611 = _609 ^ (_609 >> 11u);
        uint _614 = _611 ^ ((_611 << 7u) & 2636928640u);
        uint _617 = _614 ^ ((_614 << 15u) & 4022730752u);
        uint _623 = (((_167.x * 1664525u) + (_617 ^ (_617 >> 18u))) + 1013904223u) * 1664525u;
        uint _625 = _623 ^ (_623 >> 11u);
        uint _628 = _625 ^ ((_625 << 7u) & 2636928640u);
        uint _631 = _628 ^ ((_628 << 15u) & 4022730752u);
        uint _633 = _631 ^ (_631 >> 18u);
        uint _635 = _633 + (_633 << 10u);
        uint _637 = _635 ^ (_635 >> 6u);
        uint _639 = _637 + (_637 << 3u);
        uint _641 = _639 ^ (_639 >> 11u);
        float2 _161;
        _161.x = asfloat((_633 & 8388607u) | 1065353216u) - 1.0f;
        _161.y = asfloat(((_641 + (_641 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
        uint3 _655 = uint3(_330 + int3(1, 0, 1));
        _167 = _655;
        _166 = _655.yz;
        uint _661 = _166.y + (_166.y << 10u);
        uint _663 = _661 ^ (_661 >> 6u);
        uint _665 = _663 + (_663 << 3u);
        uint _667 = _665 ^ (_665 >> 11u);
        uint _673 = (((_166.x * 1664525u) + (_667 + (_667 << 15u))) + 1013904223u) * 1664525u;
        uint _675 = _673 ^ (_673 >> 11u);
        uint _678 = _675 ^ ((_675 << 7u) & 2636928640u);
        uint _681 = _678 ^ ((_678 << 15u) & 4022730752u);
        uint _687 = (((_167.x * 1664525u) + (_681 ^ (_681 >> 18u))) + 1013904223u) * 1664525u;
        uint _689 = _687 ^ (_687 >> 11u);
        uint _692 = _689 ^ ((_689 << 7u) & 2636928640u);
        uint _695 = _692 ^ ((_692 << 15u) & 4022730752u);
        uint _697 = _695 ^ (_695 >> 18u);
        uint _699 = _697 + (_697 << 10u);
        uint _701 = _699 ^ (_699 >> 6u);
        uint _703 = _701 + (_701 << 3u);
        uint _705 = _703 ^ (_703 >> 11u);
        float2 _160;
        _160.x = asfloat((_697 & 8388607u) | 1065353216u) - 1.0f;
        _160.y = asfloat(((_705 + (_705 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
        uint3 _719 = uint3(_330 + int3(0, 1, 1));
        _167 = _719;
        _166 = _719.yz;
        uint _725 = _166.y + (_166.y << 10u);
        uint _727 = _725 ^ (_725 >> 6u);
        uint _729 = _727 + (_727 << 3u);
        uint _731 = _729 ^ (_729 >> 11u);
        uint _737 = (((_166.x * 1664525u) + (_731 + (_731 << 15u))) + 1013904223u) * 1664525u;
        uint _739 = _737 ^ (_737 >> 11u);
        uint _742 = _739 ^ ((_739 << 7u) & 2636928640u);
        uint _745 = _742 ^ ((_742 << 15u) & 4022730752u);
        uint _751 = (((_167.x * 1664525u) + (_745 ^ (_745 >> 18u))) + 1013904223u) * 1664525u;
        uint _753 = _751 ^ (_751 >> 11u);
        uint _756 = _753 ^ ((_753 << 7u) & 2636928640u);
        uint _759 = _756 ^ ((_756 << 15u) & 4022730752u);
        uint _761 = _759 ^ (_759 >> 18u);
        uint _763 = _761 + (_761 << 10u);
        uint _765 = _763 ^ (_763 >> 6u);
        uint _767 = _765 + (_765 << 3u);
        uint _769 = _767 ^ (_767 >> 11u);
        float2 _159;
        _159.x = asfloat((_761 & 8388607u) | 1065353216u) - 1.0f;
        _159.y = asfloat(((_769 + (_769 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
        uint3 _783 = uint3(_330 + int3(1, 1, 1));
        _167 = _783;
        _166 = _783.yz;
        uint _789 = _166.y + (_166.y << 10u);
        uint _791 = _789 ^ (_789 >> 6u);
        uint _793 = _791 + (_791 << 3u);
        uint _795 = _793 ^ (_793 >> 11u);
        uint _801 = (((_166.x * 1664525u) + (_795 + (_795 << 15u))) + 1013904223u) * 1664525u;
        uint _803 = _801 ^ (_801 >> 11u);
        uint _806 = _803 ^ ((_803 << 7u) & 2636928640u);
        uint _809 = _806 ^ ((_806 << 15u) & 4022730752u);
        uint _815 = (((_167.x * 1664525u) + (_809 ^ (_809 >> 18u))) + 1013904223u) * 1664525u;
        uint _817 = _815 ^ (_815 >> 11u);
        uint _820 = _817 ^ ((_817 << 7u) & 2636928640u);
        uint _823 = _820 ^ ((_820 << 15u) & 4022730752u);
        uint _825 = _823 ^ (_823 >> 18u);
        uint _827 = _825 + (_825 << 10u);
        uint _829 = _827 ^ (_827 >> 6u);
        uint _831 = _829 + (_829 << 3u);
        uint _833 = _831 ^ (_831 >> 11u);
        float2 _158;
        _158.x = asfloat((_825 & 8388607u) | 1065353216u) - 1.0f;
        _158.y = asfloat(((_833 + (_833 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
        float3 _168 = (_331 * _331) * (3.0f.xxx - (_331 * 2.0f));
        float2 _927 = (((((((((_165 * (1.0f - _168.x)) * (1.0f - _168.y)) * (1.0f - _168.z)) + (((_164 * _168.x) * (1.0f - _168.y)) * (1.0f - _168.z))) + (((_163 * (1.0f - _168.x)) * _168.y) * (1.0f - _168.z))) + (((_162 * _168.x) * _168.y) * (1.0f - _168.z))) + (((_161 * (1.0f - _168.x)) * (1.0f - _168.y)) * _168.z)) + (((_160 * _168.x) * (1.0f - _168.y)) * _168.z)) + (((_159 * (1.0f - _168.x)) * _168.y) * _168.z)) + (((_158 * _168.x) * _168.y) * _168.z);
        float2 _169 = clamp((_927 - 0.5f.xx) * 2.5f, (-0.5f).xx, 0.5f.xx);
        float4 _937 = _178;
        float3 _939 = _937.xyz + (float3(_169.x, 0.0f, _169.y) * 0.0599999986588954925537109375f);
        float4 _940 = float4(_939.x, _939.y, _939.z, _937.w);
        _178 = _940;
        float3 _947 = cross(_318, float3(0.0f, 1.0f, 0.0f));
        _958 = mul(frame_constants_view_constants[current_view_index].world_to_sample, _940);
        _959 = instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0);
        _960 = _318;
        _961 = _939.xyz;
        _962 = asfloat(_311);
        _963 = _947;
        _964 = float3(0.0f, 1.0f, 0.0f);
        _965 = asfloat(_173.w);
        _966 = asfloat(_175.w) + (1.0f - smoothstep(0.0f, 1.0f, _178.y));
        _967 = nani_instance_data.Load<float>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 48 + 44);
        _968 = transpose(float3x3(normalize(_947), normalize(float3(0.0f, 1.0f, 0.0f)), normalize(_318)));
        break;
    } while(false);
    gl_Position = _958;
    out_var_C0 = _959;
    out_var_C1 = _960;
    out_var_C2 = _961;
    out_var_C3 = _962;
    out_var_C4 = _963;
    out_var_C5 = _964;
    out_var_C6 = _965;
    out_var_C7 = _966;
    out_var_C8 = _967;
    out_var_C9 = _968;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_VertexIndex = int(stage_input.gl_VertexIndex);
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    stage_output.out_var_C4 = out_var_C4;
    stage_output.out_var_C5 = out_var_C5;
    stage_output.out_var_C6 = out_var_C6;
    stage_output.out_var_C7 = out_var_C7;
    stage_output.out_var_C8 = out_var_C8;
    stage_output.out_var_C9 = out_var_C9;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)

#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
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

static uint _103;

ByteAddressBuffer nani_instance_data : register(t19, space0);

static uint in_var_C0;
static float2 in_var_C3;
static float in_var_C6;
static float in_var_C7;
static float in_var_C8;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] nointerpolation uint in_var_C0 : TEXCOORD0;
    [[vk::location(3)]] float2 in_var_C3 : TEXCOORD3;
    [[vk::location(6)]] float in_var_C6 : TEXCOORD6;
    [[vk::location(7)]] float in_var_C7 : TEXCOORD7;
    [[vk::location(8)]] float in_var_C8 : TEXCOORD8;
};

void frag_main()
{
    float _123 = clamp(in_var_C6, 0.0f, 1.0f);
    uint _130 = uint(nani_instance_data.Load<float>(in_var_C0 * 48 + 20) * 1.75f);
    uint _132 = _130 + (_130 << 10u);
    uint _134 = _132 ^ (_132 >> 6u);
    uint _136 = _134 + (_134 << 3u);
    uint _138 = _136 ^ (_136 >> 11u);
    float _148 = (int(nani_instance_data.Load<uint>(in_var_C0 * 48 + 24) != 0u) == 0) ? 0.0f : smoothstep(1.0f - (1.0f / nani_instance_data.Load<float>(in_var_C0 * 48 + 20)), 1.0f, in_var_C3.x / nani_instance_data.Load<float>(in_var_C0 * 48 + 20));
    if (_123 < 0.100000001490116119384765625f)
    {
        discard;
    }
    float2 _152 = lerp(0.0f, 10.0f, asfloat(((_138 + (_138 << 15u)) & 8388607u) | 1065353216u) - 1.0f).xx;
    float _153 = lerp(0.0f, 0.5f, in_var_C8);
    float2 _164;
    uint _166;
    uint _168;
    bool _170;
    bool _172;
    float _174;
    _164 = ((in_var_C3 * 1.0f) + 23.3999996185302734375f.xx) + _152;
    _166 = _103;
    _168 = _103;
    _170 = false;
    _172 = false;
    _174 = 0.0f;
    float _161 = 0.5f;
    int _176 = 0;
    for (; _176 < 3; )
    {
        int2 _181 = int2(floor(_164));
        float2 _182 = frac(_164);
        uint2 _183 = uint2(_181);
        uint _185 = _183.y;
        uint _187 = _185 + (_185 << 10u);
        uint _189 = _187 ^ (_187 >> 6u);
        uint _191 = _189 + (_189 << 3u);
        uint _193 = _191 ^ (_191 >> 11u);
        bool _196 = _172 ? _172 : true;
        uint _197 = _172 ? _168 : 1664525u;
        bool _198 = _170 ? _170 : true;
        uint _199 = _170 ? _166 : 1013904223u;
        uint _203 = (((_183.x * _197) + (_193 + (_193 << 15u))) + _199) * _197;
        uint _205 = _203 ^ (_203 >> 11u);
        uint _208 = _205 ^ ((_205 << 7u) & 2636928640u);
        uint _211 = _208 ^ ((_208 << 15u) & 4022730752u);
        uint2 _219 = uint2(_181 + int2(1, 0));
        uint _221 = _219.y;
        uint _223 = _221 + (_221 << 10u);
        uint _225 = _223 ^ (_223 >> 6u);
        uint _227 = _225 + (_225 << 3u);
        uint _229 = _227 ^ (_227 >> 11u);
        bool _232 = _196 ? _196 : true;
        uint _233 = _196 ? _197 : 1664525u;
        bool _234 = _198 ? _198 : true;
        uint _235 = _198 ? _199 : 1013904223u;
        uint _239 = (((_219.x * _233) + (_229 + (_229 << 15u))) + _235) * _233;
        uint _241 = _239 ^ (_239 >> 11u);
        uint _244 = _241 ^ ((_241 << 7u) & 2636928640u);
        uint _247 = _244 ^ ((_244 << 15u) & 4022730752u);
        uint2 _255 = uint2(_181 + int2(0, 1));
        uint _257 = _255.y;
        uint _259 = _257 + (_257 << 10u);
        uint _261 = _259 ^ (_259 >> 6u);
        uint _263 = _261 + (_261 << 3u);
        uint _265 = _263 ^ (_263 >> 11u);
        bool _268 = _232 ? _232 : true;
        uint _269 = _232 ? _233 : 1664525u;
        bool _270 = _234 ? _234 : true;
        uint _271 = _234 ? _235 : 1013904223u;
        uint _275 = (((_255.x * _269) + (_265 + (_265 << 15u))) + _271) * _269;
        uint _277 = _275 ^ (_275 >> 11u);
        uint _280 = _277 ^ ((_277 << 7u) & 2636928640u);
        uint _283 = _280 ^ ((_280 << 15u) & 4022730752u);
        uint2 _291 = uint2(_181 + int2(1, 1));
        uint _293 = _291.y;
        uint _295 = _293 + (_293 << 10u);
        uint _297 = _295 ^ (_295 >> 6u);
        uint _299 = _297 + (_297 << 3u);
        uint _301 = _299 ^ (_299 >> 11u);
        uint _169 = _268 ? _269 : 1664525u;
        uint _167 = _270 ? _271 : 1013904223u;
        uint _307 = (((_291.x * _169) + (_301 + (_301 << 15u))) + _167) * _169;
        uint _309 = _307 ^ (_307 >> 11u);
        uint _312 = _309 ^ ((_309 << 7u) & 2636928640u);
        uint _315 = _312 ^ ((_312 << 15u) & 4022730752u);
        float2 _325 = (_182 * _182) * (3.0f.xx - (_182 * 2.0f));
        float _326 = _325.x;
        float _327 = 1.0f - _326;
        float _329 = _325.y;
        float _342 = _161 * ((-1.0f) + (2.0f * ((((1.0f - _329) * (((asfloat(((_211 ^ (_211 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _327) + ((asfloat(((_247 ^ (_247 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _326))) + (((asfloat(((_283 ^ (_283 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _327) * _329)) + (((asfloat(((_315 ^ (_315 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _326) * _329))));
        _161 *= 0.550000011920928955078125f;
        _164 = mul(float2x2(float2(0.800000011920928955078125f, -0.60000002384185791015625f), float2(0.60000002384185791015625f, 0.800000011920928955078125f)), _164 * 1.89999997615814208984375f);
        _166 = _167;
        _168 = _169;
        _170 = _270 ? _270 : true;
        _172 = _268 ? _268 : true;
        _174 += _342;
        _176++;
        continue;
    }
    float2 _353;
    uint _355;
    uint _357;
    bool _359;
    bool _361;
    float _363;
    _353 = ((in_var_C3 * 0.5f) + 0.5f.xx) + _152;
    _355 = _166;
    _357 = _168;
    _359 = _170;
    _361 = _172;
    _363 = 0.0f;
    float _350 = 0.5f;
    int _365 = 0;
    for (; _365 < 5; )
    {
        int2 _370 = int2(floor(_353));
        float2 _371 = frac(_353);
        uint2 _372 = uint2(_370);
        uint _374 = _372.y;
        uint _376 = _374 + (_374 << 10u);
        uint _378 = _376 ^ (_376 >> 6u);
        uint _380 = _378 + (_378 << 3u);
        uint _382 = _380 ^ (_380 >> 11u);
        bool _385 = _361 ? _361 : true;
        uint _386 = _361 ? _357 : 1664525u;
        bool _387 = _359 ? _359 : true;
        uint _388 = _359 ? _355 : 1013904223u;
        uint _392 = (((_372.x * _386) + (_382 + (_382 << 15u))) + _388) * _386;
        uint _394 = _392 ^ (_392 >> 11u);
        uint _397 = _394 ^ ((_394 << 7u) & 2636928640u);
        uint _400 = _397 ^ ((_397 << 15u) & 4022730752u);
        uint2 _408 = uint2(_370 + int2(1, 0));
        uint _410 = _408.y;
        uint _412 = _410 + (_410 << 10u);
        uint _414 = _412 ^ (_412 >> 6u);
        uint _416 = _414 + (_414 << 3u);
        uint _418 = _416 ^ (_416 >> 11u);
        bool _421 = _385 ? _385 : true;
        uint _422 = _385 ? _386 : 1664525u;
        bool _423 = _387 ? _387 : true;
        uint _424 = _387 ? _388 : 1013904223u;
        uint _428 = (((_408.x * _422) + (_418 + (_418 << 15u))) + _424) * _422;
        uint _430 = _428 ^ (_428 >> 11u);
        uint _433 = _430 ^ ((_430 << 7u) & 2636928640u);
        uint _436 = _433 ^ ((_433 << 15u) & 4022730752u);
        uint2 _444 = uint2(_370 + int2(0, 1));
        uint _446 = _444.y;
        uint _448 = _446 + (_446 << 10u);
        uint _450 = _448 ^ (_448 >> 6u);
        uint _452 = _450 + (_450 << 3u);
        uint _454 = _452 ^ (_452 >> 11u);
        bool _457 = _421 ? _421 : true;
        uint _458 = _421 ? _422 : 1664525u;
        bool _459 = _423 ? _423 : true;
        uint _460 = _423 ? _424 : 1013904223u;
        uint _464 = (((_444.x * _458) + (_454 + (_454 << 15u))) + _460) * _458;
        uint _466 = _464 ^ (_464 >> 11u);
        uint _469 = _466 ^ ((_466 << 7u) & 2636928640u);
        uint _472 = _469 ^ ((_469 << 15u) & 4022730752u);
        uint2 _480 = uint2(_370 + int2(1, 1));
        uint _482 = _480.y;
        uint _484 = _482 + (_482 << 10u);
        uint _486 = _484 ^ (_484 >> 6u);
        uint _488 = _486 + (_486 << 3u);
        uint _490 = _488 ^ (_488 >> 11u);
        uint _358 = _457 ? _458 : 1664525u;
        uint _356 = _459 ? _460 : 1013904223u;
        uint _496 = (((_480.x * _358) + (_490 + (_490 << 15u))) + _356) * _358;
        uint _498 = _496 ^ (_496 >> 11u);
        uint _501 = _498 ^ ((_498 << 7u) & 2636928640u);
        uint _504 = _501 ^ ((_501 << 15u) & 4022730752u);
        float2 _514 = (_371 * _371) * (3.0f.xx - (_371 * 2.0f));
        float _515 = _514.x;
        float _516 = 1.0f - _515;
        float _518 = _514.y;
        float _531 = _350 * ((-1.0f) + (2.0f * ((((1.0f - _518) * (((asfloat(((_400 ^ (_400 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _516) + ((asfloat(((_436 ^ (_436 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _515))) + (((asfloat(((_472 ^ (_472 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _516) * _518)) + (((asfloat(((_504 ^ (_504 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _515) * _518))));
        _350 *= 0.550000011920928955078125f;
        _353 = mul(float2x2(float2(0.800000011920928955078125f, -0.60000002384185791015625f), float2(0.60000002384185791015625f, 0.800000011920928955078125f)), _353 * 1.89999997615814208984375f);
        _355 = _356;
        _357 = _358;
        _359 = _459 ? _459 : true;
        _361 = _457 ? _457 : true;
        _363 += _531;
        _365++;
        continue;
    }
    float _536 = smoothstep(0.0f, 1.0f, 1.0f - _123) * (lerp(0.300000011920928955078125f, 1.0f, _174) * in_var_C7);
    float _537 = (_153 + _363) + _536;
    float _734;
    if (_148 > 0.0f)
    {
        float2 _551;
        float _557;
        _551 = ((float2(nani_instance_data.Load<float>(in_var_C0 * 48 + 20) - in_var_C3.x, in_var_C3.y) * 0.5f) + 0.5f.xx) + _152;
        _557 = 0.0f;
        float _548 = 0.5f;
        bool _553 = _359;
        bool _555 = _361;
        int _559 = 0;
        uint _561 = _355;
        uint _563 = _357;
        for (; _559 < 5; )
        {
            int2 _568 = int2(floor(_551));
            float2 _569 = frac(_551);
            uint2 _570 = uint2(_568);
            uint _572 = _570.y;
            uint _574 = _572 + (_572 << 10u);
            uint _576 = _574 ^ (_574 >> 6u);
            uint _578 = _576 + (_576 << 3u);
            uint _580 = _578 ^ (_578 >> 11u);
            bool _583 = _555 ? _555 : true;
            uint _584 = _555 ? _563 : 1664525u;
            bool _585 = _553 ? _553 : true;
            uint _586 = _553 ? _561 : 1013904223u;
            uint _590 = (((_570.x * _584) + (_580 + (_580 << 15u))) + _586) * _584;
            uint _592 = _590 ^ (_590 >> 11u);
            uint _595 = _592 ^ ((_592 << 7u) & 2636928640u);
            uint _598 = _595 ^ ((_595 << 15u) & 4022730752u);
            uint2 _606 = uint2(_568 + int2(1, 0));
            uint _608 = _606.y;
            uint _610 = _608 + (_608 << 10u);
            uint _612 = _610 ^ (_610 >> 6u);
            uint _614 = _612 + (_612 << 3u);
            uint _616 = _614 ^ (_614 >> 11u);
            bool _619 = _583 ? _583 : true;
            uint _620 = _583 ? _584 : 1664525u;
            bool _621 = _585 ? _585 : true;
            uint _622 = _585 ? _586 : 1013904223u;
            uint _626 = (((_606.x * _620) + (_616 + (_616 << 15u))) + _622) * _620;
            uint _628 = _626 ^ (_626 >> 11u);
            uint _631 = _628 ^ ((_628 << 7u) & 2636928640u);
            uint _634 = _631 ^ ((_631 << 15u) & 4022730752u);
            uint2 _642 = uint2(_568 + int2(0, 1));
            uint _644 = _642.y;
            uint _646 = _644 + (_644 << 10u);
            uint _648 = _646 ^ (_646 >> 6u);
            uint _650 = _648 + (_648 << 3u);
            uint _652 = _650 ^ (_650 >> 11u);
            bool _655 = _619 ? _619 : true;
            uint _656 = _619 ? _620 : 1664525u;
            bool _657 = _621 ? _621 : true;
            uint _658 = _621 ? _622 : 1013904223u;
            uint _662 = (((_642.x * _656) + (_652 + (_652 << 15u))) + _658) * _656;
            uint _664 = _662 ^ (_662 >> 11u);
            uint _667 = _664 ^ ((_664 << 7u) & 2636928640u);
            uint _670 = _667 ^ ((_667 << 15u) & 4022730752u);
            uint2 _678 = uint2(_568 + int2(1, 1));
            uint _680 = _678.y;
            uint _682 = _680 + (_680 << 10u);
            uint _684 = _682 ^ (_682 >> 6u);
            uint _686 = _684 + (_684 << 3u);
            uint _688 = _686 ^ (_686 >> 11u);
            uint _564 = _655 ? _656 : 1664525u;
            uint _562 = _657 ? _658 : 1013904223u;
            uint _694 = (((_678.x * _564) + (_688 + (_688 << 15u))) + _562) * _564;
            uint _696 = _694 ^ (_694 >> 11u);
            uint _699 = _696 ^ ((_696 << 7u) & 2636928640u);
            uint _702 = _699 ^ ((_699 << 15u) & 4022730752u);
            float2 _712 = (_569 * _569) * (3.0f.xx - (_569 * 2.0f));
            float _713 = _712.x;
            float _714 = 1.0f - _713;
            float _716 = _712.y;
            float _729 = _548 * ((-1.0f) + (2.0f * ((((1.0f - _716) * (((asfloat(((_598 ^ (_598 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _714) + ((asfloat(((_634 ^ (_634 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _713))) + (((asfloat(((_670 ^ (_670 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _714) * _716)) + (((asfloat(((_702 ^ (_702 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _713) * _716))));
            _548 *= 0.550000011920928955078125f;
            _551 = mul(float2x2(float2(0.800000011920928955078125f, -0.60000002384185791015625f), float2(0.60000002384185791015625f, 0.800000011920928955078125f)), _551 * 1.89999997615814208984375f);
            _553 = _657 ? _657 : true;
            _555 = _655 ? _655 : true;
            _557 += _729;
            _559++;
            _561 = _562;
            _563 = _564;
            continue;
        }
        _734 = lerp(_537, (_153 + _557) + _536, _148);
    }
    else
    {
        _734 = _537;
    }
    if ((_734 * lerp(2.0f, 2.7999999523162841796875f, clamp(in_var_C8 * 2.0f, 0.0f, 1.0f))) > (smoothstep(0.0f, 4.0f, in_var_C3.y) * 2.0f))
    {
        discard;
    }
}

void ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C0 = stage_input.in_var_C0;
    in_var_C3 = stage_input.in_var_C3;
    in_var_C6 = stage_input.in_var_C6;
    in_var_C7 = stage_input.in_var_C7;
    in_var_C8 = stage_input.in_var_C8;
    frag_main();
}
#pragma dxc diagnostic pop
#endif
