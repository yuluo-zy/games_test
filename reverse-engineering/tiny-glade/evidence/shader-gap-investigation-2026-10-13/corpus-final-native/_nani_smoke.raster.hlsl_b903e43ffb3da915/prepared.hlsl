#define SHADER_VARIANT_DEPTH_ONLY 0
#define TGV_b903e43ffb3da915 1
#line 1 "shaders/nani_smoke.raster.hlsl"




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

struct Affine3Packed
{
    float data[12];
};

struct SmokeInstanceData
{
    Affine3Packed xform;
    uint smoke_type;
    int seed;
    float thickness;
    uint pad0;
};

static float _124;
static float3 _125;

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
};

ByteAddressBuffer instance_indirection_buffer : register(t14, space0);
ByteAddressBuffer nani_instance_data : register(t15, space0);

static float4 gl_Position;
static int gl_VertexIndex;
static int gl_InstanceIndex;
static float3 out_var_C0;
static float3 out_var_C1;
static float out_var_C2;

struct SPIRV_Cross_Input
{
    uint gl_VertexIndex : SV_VertexID;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float out_var_C2 : TEXCOORD2;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    float4 _1524;
    float3 _1525;
    float3 _1526;
    float _1527;
    do
    {
        Affine3Packed _167;
        [unroll]
        for (int _2ident = 0; _2ident < 12; _2ident++)
        {
            _167.data[_2ident] = nani_instance_data.Load<float>(_2ident * 4 + instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 0);
        }
        float2 _192 = float2(float(uint(gl_VertexIndex) % 13u), float(uint(gl_VertexIndex) / 13u)) * float2(0.083333335816860198974609375f, 0.0199999995529651641845703125f);
        float _194 = _192.x * (-6.283185482025146484375f);
        float _195 = cos(_194);
        float _197 = sin(_194);
        if (nani_instance_data.Load<float>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 56) < 0.100000001490116119384765625f)
        {
            _1524 = asfloat(0x7fbfffffu          ).xxxx;
            _1525 = _125;
            _1526 = _125;
            _1527 = _124;
            break;
        }
        float _209 = float(frame_constants_game_time_sec) + frame_constants_game_time_frac;
        float3 _210 = float3(_195, _192.y, _197) * float3(0.0599999986588954925537109375f, 15.0f, 0.0599999986588954925537109375f);
        float _216 = _210.y + ((frac(_209 * 4.333333492279052734375f) - 1.0f) * 0.2999999821186065673828125f);
        float3 _217 = _210;
        _217.y = _216;
        float3x4 _223 = transpose(float4x3(float3(_167.data[0], _167.data[1], _167.data[2]), float3(_167.data[3], _167.data[4], _167.data[5]), float3(_167.data[6], _167.data[7], _167.data[8]), float3(_167.data[9], _167.data[10], _167.data[11])));
        float3 _224 = mul(_223, float4(0.0f, 0.0f, 0.0f, 1.0f));
        float3 _229 = ((_224 * 2.0f) + (_217 * 2.0f)) + (float3(0.0f, -2.0f, 0.0f) * _209);
        int3 _231 = int3(floor(_229));
        float3 _232 = frac(_229);
        uint3 _233 = uint3(_231);
        uint _236 = _233.z;
        uint _238 = _236 + (_236 << 10u);
        uint _240 = _238 ^ (_238 >> 6u);
        uint _242 = _240 + (_240 << 3u);
        uint _244 = _242 ^ (_242 >> 11u);
        uint _250 = (((_233.y * 1664525u) + (_244 + (_244 << 15u))) + 1013904223u) * 1664525u;
        uint _252 = _250 ^ (_250 >> 11u);
        uint _255 = _252 ^ ((_252 << 7u) & 2636928640u);
        uint _258 = _255 ^ ((_255 << 15u) & 4022730752u);
        uint _264 = (((_233.x * 1664525u) + (_258 ^ (_258 >> 18u))) + 1013904223u) * 1664525u;
        uint _266 = _264 ^ (_264 >> 11u);
        uint _269 = _266 ^ ((_266 << 7u) & 2636928640u);
        uint _272 = _269 ^ ((_269 << 15u) & 4022730752u);
        uint3 _280 = uint3(_231 + int3(1, 0, 0));
        uint _283 = _280.z;
        uint _285 = _283 + (_283 << 10u);
        uint _287 = _285 ^ (_285 >> 6u);
        uint _289 = _287 + (_287 << 3u);
        uint _291 = _289 ^ (_289 >> 11u);
        uint _297 = (((_280.y * 1664525u) + (_291 + (_291 << 15u))) + 1013904223u) * 1664525u;
        uint _299 = _297 ^ (_297 >> 11u);
        uint _302 = _299 ^ ((_299 << 7u) & 2636928640u);
        uint _305 = _302 ^ ((_302 << 15u) & 4022730752u);
        uint _311 = (((_280.x * 1664525u) + (_305 ^ (_305 >> 18u))) + 1013904223u) * 1664525u;
        uint _313 = _311 ^ (_311 >> 11u);
        uint _316 = _313 ^ ((_313 << 7u) & 2636928640u);
        uint _319 = _316 ^ ((_316 << 15u) & 4022730752u);
        uint3 _327 = uint3(_231 + int3(0, 1, 0));
        uint _330 = _327.z;
        uint _332 = _330 + (_330 << 10u);
        uint _334 = _332 ^ (_332 >> 6u);
        uint _336 = _334 + (_334 << 3u);
        uint _338 = _336 ^ (_336 >> 11u);
        uint _344 = (((_327.y * 1664525u) + (_338 + (_338 << 15u))) + 1013904223u) * 1664525u;
        uint _346 = _344 ^ (_344 >> 11u);
        uint _349 = _346 ^ ((_346 << 7u) & 2636928640u);
        uint _352 = _349 ^ ((_349 << 15u) & 4022730752u);
        uint _358 = (((_327.x * 1664525u) + (_352 ^ (_352 >> 18u))) + 1013904223u) * 1664525u;
        uint _360 = _358 ^ (_358 >> 11u);
        uint _363 = _360 ^ ((_360 << 7u) & 2636928640u);
        uint _366 = _363 ^ ((_363 << 15u) & 4022730752u);
        uint3 _374 = uint3(_231 + int3(1, 1, 0));
        uint _377 = _374.z;
        uint _379 = _377 + (_377 << 10u);
        uint _381 = _379 ^ (_379 >> 6u);
        uint _383 = _381 + (_381 << 3u);
        uint _385 = _383 ^ (_383 >> 11u);
        uint _391 = (((_374.y * 1664525u) + (_385 + (_385 << 15u))) + 1013904223u) * 1664525u;
        uint _393 = _391 ^ (_391 >> 11u);
        uint _396 = _393 ^ ((_393 << 7u) & 2636928640u);
        uint _399 = _396 ^ ((_396 << 15u) & 4022730752u);
        uint _405 = (((_374.x * 1664525u) + (_399 ^ (_399 >> 18u))) + 1013904223u) * 1664525u;
        uint _407 = _405 ^ (_405 >> 11u);
        uint _410 = _407 ^ ((_407 << 7u) & 2636928640u);
        uint _413 = _410 ^ ((_410 << 15u) & 4022730752u);
        uint3 _421 = uint3(_231 + int3(0, 0, 1));
        uint _424 = _421.z;
        uint _426 = _424 + (_424 << 10u);
        uint _428 = _426 ^ (_426 >> 6u);
        uint _430 = _428 + (_428 << 3u);
        uint _432 = _430 ^ (_430 >> 11u);
        uint _438 = (((_421.y * 1664525u) + (_432 + (_432 << 15u))) + 1013904223u) * 1664525u;
        uint _440 = _438 ^ (_438 >> 11u);
        uint _443 = _440 ^ ((_440 << 7u) & 2636928640u);
        uint _446 = _443 ^ ((_443 << 15u) & 4022730752u);
        uint _452 = (((_421.x * 1664525u) + (_446 ^ (_446 >> 18u))) + 1013904223u) * 1664525u;
        uint _454 = _452 ^ (_452 >> 11u);
        uint _457 = _454 ^ ((_454 << 7u) & 2636928640u);
        uint _460 = _457 ^ ((_457 << 15u) & 4022730752u);
        uint3 _468 = uint3(_231 + int3(1, 0, 1));
        uint _471 = _468.z;
        uint _473 = _471 + (_471 << 10u);
        uint _475 = _473 ^ (_473 >> 6u);
        uint _477 = _475 + (_475 << 3u);
        uint _479 = _477 ^ (_477 >> 11u);
        uint _485 = (((_468.y * 1664525u) + (_479 + (_479 << 15u))) + 1013904223u) * 1664525u;
        uint _487 = _485 ^ (_485 >> 11u);
        uint _490 = _487 ^ ((_487 << 7u) & 2636928640u);
        uint _493 = _490 ^ ((_490 << 15u) & 4022730752u);
        uint _499 = (((_468.x * 1664525u) + (_493 ^ (_493 >> 18u))) + 1013904223u) * 1664525u;
        uint _501 = _499 ^ (_499 >> 11u);
        uint _504 = _501 ^ ((_501 << 7u) & 2636928640u);
        uint _507 = _504 ^ ((_504 << 15u) & 4022730752u);
        uint3 _515 = uint3(_231 + int3(0, 1, 1));
        uint _518 = _515.z;
        uint _520 = _518 + (_518 << 10u);
        uint _522 = _520 ^ (_520 >> 6u);
        uint _524 = _522 + (_522 << 3u);
        uint _526 = _524 ^ (_524 >> 11u);
        uint _532 = (((_515.y * 1664525u) + (_526 + (_526 << 15u))) + 1013904223u) * 1664525u;
        uint _534 = _532 ^ (_532 >> 11u);
        uint _537 = _534 ^ ((_534 << 7u) & 2636928640u);
        uint _540 = _537 ^ ((_537 << 15u) & 4022730752u);
        uint _546 = (((_515.x * 1664525u) + (_540 ^ (_540 >> 18u))) + 1013904223u) * 1664525u;
        uint _548 = _546 ^ (_546 >> 11u);
        uint _551 = _548 ^ ((_548 << 7u) & 2636928640u);
        uint _554 = _551 ^ ((_551 << 15u) & 4022730752u);
        uint3 _562 = uint3(_231 + int3(1, 1, 1));
        uint _565 = _562.z;
        uint _567 = _565 + (_565 << 10u);
        uint _569 = _567 ^ (_567 >> 6u);
        uint _571 = _569 + (_569 << 3u);
        uint _573 = _571 ^ (_571 >> 11u);
        uint _579 = (((_562.y * 1664525u) + (_573 + (_573 << 15u))) + 1013904223u) * 1664525u;
        uint _581 = _579 ^ (_579 >> 11u);
        uint _584 = _581 ^ ((_581 << 7u) & 2636928640u);
        uint _587 = _584 ^ ((_584 << 15u) & 4022730752u);
        uint _593 = (((_562.x * 1664525u) + (_587 ^ (_587 >> 18u))) + 1013904223u) * 1664525u;
        uint _595 = _593 ^ (_593 >> 11u);
        uint _598 = _595 ^ ((_595 << 7u) & 2636928640u);
        uint _601 = _598 ^ ((_598 << 15u) & 4022730752u);
        float3 _611 = (_232 * _232) * (3.0f.xxx - (_232 * 2.0f));
        float _612 = _611.x;
        float _613 = 1.0f - _612;
        float _615 = _611.y;
        float _616 = 1.0f - _615;
        float _617 = _611.z;
        float _640 = ((((1.0f - _617) * (((_616 * (((asfloat(((_272 ^ (_272 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _613) + ((asfloat(((_319 ^ (_319 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _612))) + (((asfloat(((_366 ^ (_366 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _613) * _615)) + (((asfloat(((_413 ^ (_413 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _612) * _615))) + ((((asfloat(((_460 ^ (_460 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _613) * _616) * _617)) + ((((asfloat(((_507 ^ (_507 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _612) * _616) * _617)) + ((((asfloat(((_554 ^ (_554 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _613) * _615) * _617);
        float _645 = _216 * 0.066666670143604278564453125f;
        float2 _658 = _217.xz * ((lerp(0.5f, 5.0f, (_640 + ((((asfloat(((_601 ^ (_601 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _612) * _615) * _617)) * clamp(1.0f - _645, 0.0f, 1.0f)) * (1.0f - smoothstep(0.0f, 1.0f, _216 + lerp(0.0f, -10.0f, nani_instance_data.Load<float>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 56))))) * (nani_instance_data.Load<float>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 56) * nani_instance_data.Load<float>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 56)));
        float3 _668;
        if (frame_constants_glade_theme_id == 4u)
        {
            float2 _666 = _658.xy * 2.0f;
            _668 = float3(_666.x, _217.y, _666.y);
        }
        else
        {
            _668 = float3(_658.x, _217.y, _658.y);
        }
        float3 _678;
        if (nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 48) == 1u)
        {
            float3 _677 = _668;
            _677.z = _668.z + (smoothstep(0.20000000298023223876953125f, 0.89999997615814208984375f, _668.y) * 0.800000011920928955078125f);
            _678 = _677;
        }
        else
        {
            _678 = _668;
        }
        float3 _683 = mul(_223, float4(_678, 1.0f));
        float _685 = _683.y - 0.1500000059604644775390625f;
        float3 _686 = _683;
        _686.y = _685;
        float2 _687 = _224.xz;
        int2 _689 = int2(floor(_687));
        float2 _690 = frac(_687);
        uint2 _691 = uint2(_689);
        uint _693 = _691.y;
        uint _695 = _693 + (_693 << 10u);
        uint _697 = _695 ^ (_695 >> 6u);
        uint _699 = _697 + (_697 << 3u);
        uint _701 = _699 ^ (_699 >> 11u);
        uint _707 = (((_691.x * 1664525u) + (_701 + (_701 << 15u))) + 1013904223u) * 1664525u;
        uint _709 = _707 ^ (_707 >> 11u);
        uint _712 = _709 ^ ((_709 << 7u) & 2636928640u);
        uint _715 = _712 ^ ((_712 << 15u) & 4022730752u);
        uint2 _723 = uint2(_689 + int2(1, 0));
        uint _725 = _723.y;
        uint _727 = _725 + (_725 << 10u);
        uint _729 = _727 ^ (_727 >> 6u);
        uint _731 = _729 + (_729 << 3u);
        uint _733 = _731 ^ (_731 >> 11u);
        uint _739 = (((_723.x * 1664525u) + (_733 + (_733 << 15u))) + 1013904223u) * 1664525u;
        uint _741 = _739 ^ (_739 >> 11u);
        uint _744 = _741 ^ ((_741 << 7u) & 2636928640u);
        uint _747 = _744 ^ ((_744 << 15u) & 4022730752u);
        uint2 _755 = uint2(_689 + int2(0, 1));
        uint _757 = _755.y;
        uint _759 = _757 + (_757 << 10u);
        uint _761 = _759 ^ (_759 >> 6u);
        uint _763 = _761 + (_761 << 3u);
        uint _765 = _763 ^ (_763 >> 11u);
        uint _771 = (((_755.x * 1664525u) + (_765 + (_765 << 15u))) + 1013904223u) * 1664525u;
        uint _773 = _771 ^ (_771 >> 11u);
        uint _776 = _773 ^ ((_773 << 7u) & 2636928640u);
        uint _779 = _776 ^ ((_776 << 15u) & 4022730752u);
        uint2 _787 = uint2(_689 + int2(1, 1));
        uint _789 = _787.y;
        uint _791 = _789 + (_789 << 10u);
        uint _793 = _791 ^ (_791 >> 6u);
        uint _795 = _793 + (_793 << 3u);
        uint _797 = _795 ^ (_795 >> 11u);
        uint _803 = (((_787.x * 1664525u) + (_797 + (_797 << 15u))) + 1013904223u) * 1664525u;
        uint _805 = _803 ^ (_803 >> 11u);
        uint _808 = _805 ^ ((_805 << 7u) & 2636928640u);
        uint _811 = _808 ^ ((_808 << 15u) & 4022730752u);
        float2 _821 = (_690 * _690) * (3.0f.xx - (_690 * 2.0f));
        float _822 = _821.x;
        float _823 = 1.0f - _822;
        float _825 = _821.y;
        float _836 = _209 + ((((1.0f - _825) * (((asfloat(((_715 ^ (_715 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _823) + ((asfloat(((_747 ^ (_747 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _822))) + (((asfloat(((_779 ^ (_779 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _823) * _825)) + (((asfloat(((_811 ^ (_811 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _822) * _825));
        float _837 = smoothstep(0.100000001490116119384765625f, 0.20000000298023223876953125f, _645);
        uint _838 = uint(nani_instance_data.Load<int>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 52));
        uint _840 = _838 + (_838 << 10u);
        uint _842 = _840 ^ (_840 >> 6u);
        uint _844 = _842 + (_842 << 3u);
        uint _846 = _844 ^ (_844 >> 11u);
        float _852 = asfloat(((_846 + (_846 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
        float _857 = (((_836 * (-0.800000011920928955078125f)) + (_678.y * 0.800000011920928955078125f)) + _852) * 0.4000000059604644775390625f;
        int _859 = int(floor(_857));
        float _860 = frac(_857);
        uint _861 = uint(_859);
        uint _863 = _861 + (_861 << 10u);
        uint _865 = _863 ^ (_863 >> 6u);
        uint _867 = _865 + (_865 << 3u);
        uint _869 = _867 ^ (_867 >> 11u);
        uint _875 = (((_861 * 1664525u) + (_869 + (_869 << 15u))) + 1013904223u) * 1664525u;
        uint _877 = _875 ^ (_875 >> 11u);
        uint _880 = _877 ^ ((_877 << 7u) & 2636928640u);
        uint _883 = _880 ^ ((_880 << 15u) & 4022730752u);
        uint _885 = _883 ^ (_883 >> 18u);
        uint _887 = _885 + (_885 << 10u);
        uint _889 = _887 ^ (_887 >> 6u);
        uint _891 = _889 + (_889 << 3u);
        uint _893 = _891 ^ (_891 >> 11u);
        uint _906 = uint(_859 + 1);
        uint _908 = _906 + (_906 << 10u);
        uint _910 = _908 ^ (_908 >> 6u);
        uint _912 = _910 + (_910 << 3u);
        uint _914 = _912 ^ (_912 >> 11u);
        uint _920 = (((_906 * 1664525u) + (_914 + (_914 << 15u))) + 1013904223u) * 1664525u;
        uint _922 = _920 ^ (_920 >> 11u);
        uint _925 = _922 ^ ((_922 << 7u) & 2636928640u);
        uint _928 = _925 ^ ((_925 << 15u) & 4022730752u);
        uint _930 = _928 ^ (_928 >> 18u);
        uint _932 = _930 + (_930 << 10u);
        uint _934 = _932 ^ (_932 >> 6u);
        uint _936 = _934 + (_934 << 3u);
        uint _938 = _936 ^ (_936 >> 11u);
        float _953 = (_860 * _860) * (3.0f - (2.0f * _860));
        float _962 = (_836 * (-0.3200000226497650146484375f)) + _852;
        int _964 = int(floor(_962));
        float _965 = frac(_962);
        uint _966 = uint(_964);
        uint _968 = _966 + (_966 << 10u);
        uint _970 = _968 ^ (_968 >> 6u);
        uint _972 = _970 + (_970 << 3u);
        uint _974 = _972 ^ (_972 >> 11u);
        uint _980 = (((_966 * 1664525u) + (_974 + (_974 << 15u))) + 1013904223u) * 1664525u;
        uint _982 = _980 ^ (_980 >> 11u);
        uint _985 = _982 ^ ((_982 << 7u) & 2636928640u);
        uint _988 = _985 ^ ((_985 << 15u) & 4022730752u);
        uint _990 = _988 ^ (_988 >> 18u);
        uint _992 = _990 + (_990 << 10u);
        uint _994 = _992 ^ (_992 >> 6u);
        uint _996 = _994 + (_994 << 3u);
        uint _998 = _996 ^ (_996 >> 11u);
        uint _1011 = uint(_964 + 1);
        uint _1013 = _1011 + (_1011 << 10u);
        uint _1015 = _1013 ^ (_1013 >> 6u);
        uint _1017 = _1015 + (_1015 << 3u);
        uint _1019 = _1017 ^ (_1017 >> 11u);
        uint _1025 = (((_1011 * 1664525u) + (_1019 + (_1019 << 15u))) + 1013904223u) * 1664525u;
        uint _1027 = _1025 ^ (_1025 >> 11u);
        uint _1030 = _1027 ^ ((_1027 << 7u) & 2636928640u);
        uint _1033 = _1030 ^ ((_1030 << 15u) & 4022730752u);
        uint _1035 = _1033 ^ (_1033 >> 18u);
        uint _1037 = _1035 + (_1035 << 10u);
        uint _1039 = _1037 ^ (_1037 >> 6u);
        uint _1041 = _1039 + (_1039 << 3u);
        uint _1043 = _1041 ^ (_1041 >> 11u);
        float _1058 = (_965 * _965) * (3.0f - (2.0f * _965));
        float _1066 = smoothstep(0.300000011920928955078125f, 0.0f, _645);
        float2 _1069 = ((((((float2(asfloat((_885 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_893 + (_893 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * (1.0f - _953)) + (float2(asfloat((_930 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_938 + (_938 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _953)) * 2.0f) - 1.0f.xx) * 2.0f) - ((((((float2(asfloat((_990 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_998 + (_998 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * (1.0f - _1058)) + (float2(asfloat((_1035 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_1043 + (_1043 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _1058)) * 2.0f) - 1.0f.xx) * 2.0f) * _1066)) * 0.300000011920928955078125f;
        float _1077 = (((_836 * (-2.1296999454498291015625f)) + (_678.y * 2.1296999454498291015625f)) + _852) * 0.4000000059604644775390625f;
        int _1079 = int(floor(_1077));
        float _1080 = frac(_1077);
        uint _1081 = uint(_1079);
        uint _1083 = _1081 + (_1081 << 10u);
        uint _1085 = _1083 ^ (_1083 >> 6u);
        uint _1087 = _1085 + (_1085 << 3u);
        uint _1089 = _1087 ^ (_1087 >> 11u);
        uint _1095 = (((_1081 * 1664525u) + (_1089 + (_1089 << 15u))) + 1013904223u) * 1664525u;
        uint _1097 = _1095 ^ (_1095 >> 11u);
        uint _1100 = _1097 ^ ((_1097 << 7u) & 2636928640u);
        uint _1103 = _1100 ^ ((_1100 << 15u) & 4022730752u);
        uint _1105 = _1103 ^ (_1103 >> 18u);
        uint _1107 = _1105 + (_1105 << 10u);
        uint _1109 = _1107 ^ (_1107 >> 6u);
        uint _1111 = _1109 + (_1109 << 3u);
        uint _1113 = _1111 ^ (_1111 >> 11u);
        uint _1126 = uint(_1079 + 1);
        uint _1128 = _1126 + (_1126 << 10u);
        uint _1130 = _1128 ^ (_1128 >> 6u);
        uint _1132 = _1130 + (_1130 << 3u);
        uint _1134 = _1132 ^ (_1132 >> 11u);
        uint _1140 = (((_1126 * 1664525u) + (_1134 + (_1134 << 15u))) + 1013904223u) * 1664525u;
        uint _1142 = _1140 ^ (_1140 >> 11u);
        uint _1145 = _1142 ^ ((_1142 << 7u) & 2636928640u);
        uint _1148 = _1145 ^ ((_1145 << 15u) & 4022730752u);
        uint _1150 = _1148 ^ (_1148 >> 18u);
        uint _1152 = _1150 + (_1150 << 10u);
        uint _1154 = _1152 ^ (_1152 >> 6u);
        uint _1156 = _1154 + (_1154 << 3u);
        uint _1158 = _1156 ^ (_1156 >> 11u);
        float _1173 = (_1080 * _1080) * (3.0f - (2.0f * _1080));
        float _1182 = (_836 * (-0.851880013942718505859375f)) + _852;
        int _1184 = int(floor(_1182));
        float _1185 = frac(_1182);
        uint _1186 = uint(_1184);
        uint _1188 = _1186 + (_1186 << 10u);
        uint _1190 = _1188 ^ (_1188 >> 6u);
        uint _1192 = _1190 + (_1190 << 3u);
        uint _1194 = _1192 ^ (_1192 >> 11u);
        uint _1200 = (((_1186 * 1664525u) + (_1194 + (_1194 << 15u))) + 1013904223u) * 1664525u;
        uint _1202 = _1200 ^ (_1200 >> 11u);
        uint _1205 = _1202 ^ ((_1202 << 7u) & 2636928640u);
        uint _1208 = _1205 ^ ((_1205 << 15u) & 4022730752u);
        uint _1210 = _1208 ^ (_1208 >> 18u);
        uint _1212 = _1210 + (_1210 << 10u);
        uint _1214 = _1212 ^ (_1212 >> 6u);
        uint _1216 = _1214 + (_1214 << 3u);
        uint _1218 = _1216 ^ (_1216 >> 11u);
        uint _1231 = uint(_1184 + 1);
        uint _1233 = _1231 + (_1231 << 10u);
        uint _1235 = _1233 ^ (_1233 >> 6u);
        uint _1237 = _1235 + (_1235 << 3u);
        uint _1239 = _1237 ^ (_1237 >> 11u);
        uint _1245 = (((_1231 * 1664525u) + (_1239 + (_1239 << 15u))) + 1013904223u) * 1664525u;
        uint _1247 = _1245 ^ (_1245 >> 11u);
        uint _1250 = _1247 ^ ((_1247 << 7u) & 2636928640u);
        uint _1253 = _1250 ^ ((_1250 << 15u) & 4022730752u);
        uint _1255 = _1253 ^ (_1253 >> 18u);
        uint _1257 = _1255 + (_1255 << 10u);
        uint _1259 = _1257 ^ (_1257 >> 6u);
        uint _1261 = _1259 + (_1259 << 3u);
        uint _1263 = _1261 ^ (_1261 >> 11u);
        float _1278 = (_1185 * _1185) * (3.0f - (2.0f * _1185));
        float2 _1288 = ((((((float2(asfloat((_1105 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_1113 + (_1113 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * (1.0f - _1173)) + (float2(asfloat((_1150 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_1158 + (_1158 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _1173)) * 2.0f) - 1.0f.xx) * 2.0f) - ((((((float2(asfloat((_1210 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_1218 + (_1218 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * (1.0f - _1278)) + (float2(asfloat((_1255 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_1263 + (_1263 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _1278)) * 2.0f) - 1.0f.xx) * 2.0f) * _1066)) * 0.20000000298023223876953125f;
        float _1296 = (((_836 * (-3.3970000743865966796875f)) + (_678.y * 3.3970000743865966796875f)) + _852) * 0.4000000059604644775390625f;
        int _1298 = int(floor(_1296));
        float _1299 = frac(_1296);
        uint _1300 = uint(_1298);
        uint _1302 = _1300 + (_1300 << 10u);
        uint _1304 = _1302 ^ (_1302 >> 6u);
        uint _1306 = _1304 + (_1304 << 3u);
        uint _1308 = _1306 ^ (_1306 >> 11u);
        uint _1314 = (((_1300 * 1664525u) + (_1308 + (_1308 << 15u))) + 1013904223u) * 1664525u;
        uint _1316 = _1314 ^ (_1314 >> 11u);
        uint _1319 = _1316 ^ ((_1316 << 7u) & 2636928640u);
        uint _1322 = _1319 ^ ((_1319 << 15u) & 4022730752u);
        uint _1324 = _1322 ^ (_1322 >> 18u);
        uint _1326 = _1324 + (_1324 << 10u);
        uint _1328 = _1326 ^ (_1326 >> 6u);
        uint _1330 = _1328 + (_1328 << 3u);
        uint _1332 = _1330 ^ (_1330 >> 11u);
        uint _1345 = uint(_1298 + 1);
        uint _1347 = _1345 + (_1345 << 10u);
        uint _1349 = _1347 ^ (_1347 >> 6u);
        uint _1351 = _1349 + (_1349 << 3u);
        uint _1353 = _1351 ^ (_1351 >> 11u);
        uint _1359 = (((_1345 * 1664525u) + (_1353 + (_1353 << 15u))) + 1013904223u) * 1664525u;
        uint _1361 = _1359 ^ (_1359 >> 11u);
        uint _1364 = _1361 ^ ((_1361 << 7u) & 2636928640u);
        uint _1367 = _1364 ^ ((_1364 << 15u) & 4022730752u);
        uint _1369 = _1367 ^ (_1367 >> 18u);
        uint _1371 = _1369 + (_1369 << 10u);
        uint _1373 = _1371 ^ (_1371 >> 6u);
        uint _1375 = _1373 + (_1373 << 3u);
        uint _1377 = _1375 ^ (_1375 >> 11u);
        float _1392 = (_1299 * _1299) * (3.0f - (2.0f * _1299));
        float _1401 = (_836 * (-1.35880005359649658203125f)) + _852;
        int _1403 = int(floor(_1401));
        float _1404 = frac(_1401);
        uint _1405 = uint(_1403);
        uint _1407 = _1405 + (_1405 << 10u);
        uint _1409 = _1407 ^ (_1407 >> 6u);
        uint _1411 = _1409 + (_1409 << 3u);
        uint _1413 = _1411 ^ (_1411 >> 11u);
        uint _1419 = (((_1405 * 1664525u) + (_1413 + (_1413 << 15u))) + 1013904223u) * 1664525u;
        uint _1421 = _1419 ^ (_1419 >> 11u);
        uint _1424 = _1421 ^ ((_1421 << 7u) & 2636928640u);
        uint _1427 = _1424 ^ ((_1424 << 15u) & 4022730752u);
        uint _1429 = _1427 ^ (_1427 >> 18u);
        uint _1431 = _1429 + (_1429 << 10u);
        uint _1433 = _1431 ^ (_1431 >> 6u);
        uint _1435 = _1433 + (_1433 << 3u);
        uint _1437 = _1435 ^ (_1435 >> 11u);
        uint _1450 = uint(_1403 + 1);
        uint _1452 = _1450 + (_1450 << 10u);
        uint _1454 = _1452 ^ (_1452 >> 6u);
        uint _1456 = _1454 + (_1454 << 3u);
        uint _1458 = _1456 ^ (_1456 >> 11u);
        uint _1464 = (((_1450 * 1664525u) + (_1458 + (_1458 << 15u))) + 1013904223u) * 1664525u;
        uint _1466 = _1464 ^ (_1464 >> 11u);
        uint _1469 = _1466 ^ ((_1466 << 7u) & 2636928640u);
        uint _1472 = _1469 ^ ((_1469 << 15u) & 4022730752u);
        uint _1474 = _1472 ^ (_1472 >> 18u);
        uint _1476 = _1474 + (_1474 << 10u);
        uint _1478 = _1476 ^ (_1476 >> 6u);
        uint _1480 = _1478 + (_1478 << 3u);
        uint _1482 = _1480 ^ (_1480 >> 11u);
        float _1497 = (_1404 * _1404) * (3.0f - (2.0f * _1404));
        float2 _1507 = ((((((float2(asfloat((_1324 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_1332 + (_1332 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * (1.0f - _1392)) + (float2(asfloat((_1369 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_1377 + (_1377 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _1392)) * 2.0f) - 1.0f.xx) * 2.0f) - ((((((float2(asfloat((_1429 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_1437 + (_1437 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * (1.0f - _1497)) + (float2(asfloat((_1474 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_1482 + (_1482 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _1497)) * 2.0f) - 1.0f.xx) * 2.0f) * _1066)) * 0.0500000007450580596923828125f;
        float2 _1509 = ((_686.xz + (_1069 * _837)).xy + (_1288 * _837)).xy + _1507;
        _1524 = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(_1509.x, _685, _1509.y, 1.0f));
        _1525 = mul(_223, float4(normalize(float3(_195, 0.0f, _197)), 0.0f));
        _1526 = float3(_1509.x, _686.y, _1509.y);
        _1527 = _645;
        break;
    } while(false);
    gl_Position = _1524;
    out_var_C0 = _1525;
    out_var_C1 = _1526;
    out_var_C2 = _1527;
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
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)

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


static float3 in_var_C0;
static float3 in_var_C1;
static float in_var_C2;
static uint2 out_var_SV_TARGET0;
static float3 out_var_SV_TARGET1;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 in_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float in_var_C2 : TEXCOORD2;
};

struct SPIRV_Cross_Output
{
    uint2 out_var_SV_TARGET0 : SV_Target0;
    float3 out_var_SV_TARGET1 : SV_Target1;
};

void frag_main()
{
    bool _115;
    if (!(in_var_C2 < 0.0f))
    {
        _115 = in_var_C2 > 1.0f;
    }
    else
    {
        _115 = true;
    }
    if (_115)
    {
        discard;
    }
    float3 _127 = (in_var_C1 * 2.0f) + (float3(0.0f, -4.0f, 0.0f) * (float(frame_constants_game_time_sec) + frame_constants_game_time_frac));
    int3 _129 = int3(floor(_127));
    float3 _130 = frac(_127);
    uint3 _131 = uint3(_129);
    uint _134 = _131.z;
    uint _136 = _134 + (_134 << 10u);
    uint _138 = _136 ^ (_136 >> 6u);
    uint _140 = _138 + (_138 << 3u);
    uint _142 = _140 ^ (_140 >> 11u);
    uint _148 = (((_131.y * 1664525u) + (_142 + (_142 << 15u))) + 1013904223u) * 1664525u;
    uint _150 = _148 ^ (_148 >> 11u);
    uint _153 = _150 ^ ((_150 << 7u) & 2636928640u);
    uint _156 = _153 ^ ((_153 << 15u) & 4022730752u);
    uint _162 = (((_131.x * 1664525u) + (_156 ^ (_156 >> 18u))) + 1013904223u) * 1664525u;
    uint _164 = _162 ^ (_162 >> 11u);
    uint _167 = _164 ^ ((_164 << 7u) & 2636928640u);
    uint _170 = _167 ^ ((_167 << 15u) & 4022730752u);
    uint3 _178 = uint3(_129 + int3(1, 0, 0));
    uint _181 = _178.z;
    uint _183 = _181 + (_181 << 10u);
    uint _185 = _183 ^ (_183 >> 6u);
    uint _187 = _185 + (_185 << 3u);
    uint _189 = _187 ^ (_187 >> 11u);
    uint _195 = (((_178.y * 1664525u) + (_189 + (_189 << 15u))) + 1013904223u) * 1664525u;
    uint _197 = _195 ^ (_195 >> 11u);
    uint _200 = _197 ^ ((_197 << 7u) & 2636928640u);
    uint _203 = _200 ^ ((_200 << 15u) & 4022730752u);
    uint _209 = (((_178.x * 1664525u) + (_203 ^ (_203 >> 18u))) + 1013904223u) * 1664525u;
    uint _211 = _209 ^ (_209 >> 11u);
    uint _214 = _211 ^ ((_211 << 7u) & 2636928640u);
    uint _217 = _214 ^ ((_214 << 15u) & 4022730752u);
    uint3 _225 = uint3(_129 + int3(0, 1, 0));
    uint _228 = _225.z;
    uint _230 = _228 + (_228 << 10u);
    uint _232 = _230 ^ (_230 >> 6u);
    uint _234 = _232 + (_232 << 3u);
    uint _236 = _234 ^ (_234 >> 11u);
    uint _242 = (((_225.y * 1664525u) + (_236 + (_236 << 15u))) + 1013904223u) * 1664525u;
    uint _244 = _242 ^ (_242 >> 11u);
    uint _247 = _244 ^ ((_244 << 7u) & 2636928640u);
    uint _250 = _247 ^ ((_247 << 15u) & 4022730752u);
    uint _256 = (((_225.x * 1664525u) + (_250 ^ (_250 >> 18u))) + 1013904223u) * 1664525u;
    uint _258 = _256 ^ (_256 >> 11u);
    uint _261 = _258 ^ ((_258 << 7u) & 2636928640u);
    uint _264 = _261 ^ ((_261 << 15u) & 4022730752u);
    uint3 _272 = uint3(_129 + int3(1, 1, 0));
    uint _275 = _272.z;
    uint _277 = _275 + (_275 << 10u);
    uint _279 = _277 ^ (_277 >> 6u);
    uint _281 = _279 + (_279 << 3u);
    uint _283 = _281 ^ (_281 >> 11u);
    uint _289 = (((_272.y * 1664525u) + (_283 + (_283 << 15u))) + 1013904223u) * 1664525u;
    uint _291 = _289 ^ (_289 >> 11u);
    uint _294 = _291 ^ ((_291 << 7u) & 2636928640u);
    uint _297 = _294 ^ ((_294 << 15u) & 4022730752u);
    uint _303 = (((_272.x * 1664525u) + (_297 ^ (_297 >> 18u))) + 1013904223u) * 1664525u;
    uint _305 = _303 ^ (_303 >> 11u);
    uint _308 = _305 ^ ((_305 << 7u) & 2636928640u);
    uint _311 = _308 ^ ((_308 << 15u) & 4022730752u);
    uint3 _319 = uint3(_129 + int3(0, 0, 1));
    uint _322 = _319.z;
    uint _324 = _322 + (_322 << 10u);
    uint _326 = _324 ^ (_324 >> 6u);
    uint _328 = _326 + (_326 << 3u);
    uint _330 = _328 ^ (_328 >> 11u);
    uint _336 = (((_319.y * 1664525u) + (_330 + (_330 << 15u))) + 1013904223u) * 1664525u;
    uint _338 = _336 ^ (_336 >> 11u);
    uint _341 = _338 ^ ((_338 << 7u) & 2636928640u);
    uint _344 = _341 ^ ((_341 << 15u) & 4022730752u);
    uint _350 = (((_319.x * 1664525u) + (_344 ^ (_344 >> 18u))) + 1013904223u) * 1664525u;
    uint _352 = _350 ^ (_350 >> 11u);
    uint _355 = _352 ^ ((_352 << 7u) & 2636928640u);
    uint _358 = _355 ^ ((_355 << 15u) & 4022730752u);
    uint3 _366 = uint3(_129 + int3(1, 0, 1));
    uint _369 = _366.z;
    uint _371 = _369 + (_369 << 10u);
    uint _373 = _371 ^ (_371 >> 6u);
    uint _375 = _373 + (_373 << 3u);
    uint _377 = _375 ^ (_375 >> 11u);
    uint _383 = (((_366.y * 1664525u) + (_377 + (_377 << 15u))) + 1013904223u) * 1664525u;
    uint _385 = _383 ^ (_383 >> 11u);
    uint _388 = _385 ^ ((_385 << 7u) & 2636928640u);
    uint _391 = _388 ^ ((_388 << 15u) & 4022730752u);
    uint _397 = (((_366.x * 1664525u) + (_391 ^ (_391 >> 18u))) + 1013904223u) * 1664525u;
    uint _399 = _397 ^ (_397 >> 11u);
    uint _402 = _399 ^ ((_399 << 7u) & 2636928640u);
    uint _405 = _402 ^ ((_402 << 15u) & 4022730752u);
    uint3 _413 = uint3(_129 + int3(0, 1, 1));
    uint _416 = _413.z;
    uint _418 = _416 + (_416 << 10u);
    uint _420 = _418 ^ (_418 >> 6u);
    uint _422 = _420 + (_420 << 3u);
    uint _424 = _422 ^ (_422 >> 11u);
    uint _430 = (((_413.y * 1664525u) + (_424 + (_424 << 15u))) + 1013904223u) * 1664525u;
    uint _432 = _430 ^ (_430 >> 11u);
    uint _435 = _432 ^ ((_432 << 7u) & 2636928640u);
    uint _438 = _435 ^ ((_435 << 15u) & 4022730752u);
    uint _444 = (((_413.x * 1664525u) + (_438 ^ (_438 >> 18u))) + 1013904223u) * 1664525u;
    uint _446 = _444 ^ (_444 >> 11u);
    uint _449 = _446 ^ ((_446 << 7u) & 2636928640u);
    uint _452 = _449 ^ ((_449 << 15u) & 4022730752u);
    uint3 _460 = uint3(_129 + int3(1, 1, 1));
    uint _463 = _460.z;
    uint _465 = _463 + (_463 << 10u);
    uint _467 = _465 ^ (_465 >> 6u);
    uint _469 = _467 + (_467 << 3u);
    uint _471 = _469 ^ (_469 >> 11u);
    uint _477 = (((_460.y * 1664525u) + (_471 + (_471 << 15u))) + 1013904223u) * 1664525u;
    uint _479 = _477 ^ (_477 >> 11u);
    uint _482 = _479 ^ ((_479 << 7u) & 2636928640u);
    uint _485 = _482 ^ ((_482 << 15u) & 4022730752u);
    uint _491 = (((_460.x * 1664525u) + (_485 ^ (_485 >> 18u))) + 1013904223u) * 1664525u;
    uint _493 = _491 ^ (_491 >> 11u);
    uint _496 = _493 ^ ((_493 << 7u) & 2636928640u);
    uint _499 = _496 ^ ((_496 << 15u) & 4022730752u);
    float3 _509 = (_130 * _130) * (3.0f.xxx - (_130 * 2.0f));
    float _510 = _509.x;
    float _511 = 1.0f - _510;
    float _513 = _509.y;
    float _514 = 1.0f - _513;
    float _515 = _509.z;
    float _538 = ((((1.0f - _515) * (((_514 * (((asfloat(((_170 ^ (_170 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _511) + ((asfloat(((_217 ^ (_217 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _510))) + (((asfloat(((_264 ^ (_264 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _511) * _513)) + (((asfloat(((_311 ^ (_311 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _510) * _513))) + ((((asfloat(((_358 ^ (_358 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _511) * _514) * _515)) + ((((asfloat(((_405 ^ (_405 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _510) * _514) * _515)) + ((((asfloat(((_452 ^ (_452 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _511) * _513) * _515);
    if ((_538 + ((((asfloat(((_499 ^ (_499 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _510) * _513) * _515)) > smoothstep(0.800000011920928955078125f, 0.100000001490116119384765625f, in_var_C2))
    {
        discard;
    }
    float3 _548 = (normalize(in_var_C0) * 0.5f) + 0.5f.xxx;
    uint2 _569 = uint2(((uint((clamp(_548.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_548.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_548.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 4278190080u, 9211020u);
    _569.y = 1183616140u;
    out_var_SV_TARGET0 = _569;
    out_var_SV_TARGET1 = float3(0.0f, (1.2999999523162841796875f * frame_constants_delta_time_seconds) * smoothstep(0.0f, 0.0199999995529651641845703125f, in_var_C2), 0.0f);
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C0 = stage_input.in_var_C0;
    in_var_C1 = stage_input.in_var_C1;
    in_var_C2 = stage_input.in_var_C2;
    frag_main();
    SPIRV_Cross_Output stage_output;
    stage_output.out_var_SV_TARGET0 = out_var_SV_TARGET0;
    stage_output.out_var_SV_TARGET1 = out_var_SV_TARGET1;
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

struct Affine3Packed
{
    float data[12];
};

struct SmokeInstanceData
{
    Affine3Packed xform;
    uint smoke_type;
    int seed;
    float thickness;
    uint pad0;
};

static float _124;
static float3 _125;

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
};

ByteAddressBuffer instance_indirection_buffer : register(t14, space0);
ByteAddressBuffer nani_instance_data : register(t15, space0);

static float4 gl_Position;
static int gl_VertexIndex;
static int gl_InstanceIndex;
static float3 out_var_C0;
static float3 out_var_C1;
static float out_var_C2;

struct SPIRV_Cross_Input
{
    uint gl_VertexIndex : SV_VertexID;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float out_var_C2 : TEXCOORD2;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    float4 _1524;
    float3 _1525;
    float3 _1526;
    float _1527;
    do
    {
        Affine3Packed _167;
        [unroll]
        for (int _2ident = 0; _2ident < 12; _2ident++)
        {
            _167.data[_2ident] = nani_instance_data.Load<float>(_2ident * 4 + instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 0);
        }
        float2 _192 = float2(float(uint(gl_VertexIndex) % 13u), float(uint(gl_VertexIndex) / 13u)) * float2(0.083333335816860198974609375f, 0.0199999995529651641845703125f);
        float _194 = _192.x * (-6.283185482025146484375f);
        float _195 = cos(_194);
        float _197 = sin(_194);
        if (nani_instance_data.Load<float>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 56) < 0.100000001490116119384765625f)
        {
            _1524 = asfloat(0x7fbfffffu          ).xxxx;
            _1525 = _125;
            _1526 = _125;
            _1527 = _124;
            break;
        }
        float _209 = float(frame_constants_game_time_sec) + frame_constants_game_time_frac;
        float3 _210 = float3(_195, _192.y, _197) * float3(0.0599999986588954925537109375f, 15.0f, 0.0599999986588954925537109375f);
        float _216 = _210.y + ((frac(_209 * 4.333333492279052734375f) - 1.0f) * 0.2999999821186065673828125f);
        float3 _217 = _210;
        _217.y = _216;
        float3x4 _223 = transpose(float4x3(float3(_167.data[0], _167.data[1], _167.data[2]), float3(_167.data[3], _167.data[4], _167.data[5]), float3(_167.data[6], _167.data[7], _167.data[8]), float3(_167.data[9], _167.data[10], _167.data[11])));
        float3 _224 = mul(_223, float4(0.0f, 0.0f, 0.0f, 1.0f));
        float3 _229 = ((_224 * 2.0f) + (_217 * 2.0f)) + (float3(0.0f, -2.0f, 0.0f) * _209);
        int3 _231 = int3(floor(_229));
        float3 _232 = frac(_229);
        uint3 _233 = uint3(_231);
        uint _236 = _233.z;
        uint _238 = _236 + (_236 << 10u);
        uint _240 = _238 ^ (_238 >> 6u);
        uint _242 = _240 + (_240 << 3u);
        uint _244 = _242 ^ (_242 >> 11u);
        uint _250 = (((_233.y * 1664525u) + (_244 + (_244 << 15u))) + 1013904223u) * 1664525u;
        uint _252 = _250 ^ (_250 >> 11u);
        uint _255 = _252 ^ ((_252 << 7u) & 2636928640u);
        uint _258 = _255 ^ ((_255 << 15u) & 4022730752u);
        uint _264 = (((_233.x * 1664525u) + (_258 ^ (_258 >> 18u))) + 1013904223u) * 1664525u;
        uint _266 = _264 ^ (_264 >> 11u);
        uint _269 = _266 ^ ((_266 << 7u) & 2636928640u);
        uint _272 = _269 ^ ((_269 << 15u) & 4022730752u);
        uint3 _280 = uint3(_231 + int3(1, 0, 0));
        uint _283 = _280.z;
        uint _285 = _283 + (_283 << 10u);
        uint _287 = _285 ^ (_285 >> 6u);
        uint _289 = _287 + (_287 << 3u);
        uint _291 = _289 ^ (_289 >> 11u);
        uint _297 = (((_280.y * 1664525u) + (_291 + (_291 << 15u))) + 1013904223u) * 1664525u;
        uint _299 = _297 ^ (_297 >> 11u);
        uint _302 = _299 ^ ((_299 << 7u) & 2636928640u);
        uint _305 = _302 ^ ((_302 << 15u) & 4022730752u);
        uint _311 = (((_280.x * 1664525u) + (_305 ^ (_305 >> 18u))) + 1013904223u) * 1664525u;
        uint _313 = _311 ^ (_311 >> 11u);
        uint _316 = _313 ^ ((_313 << 7u) & 2636928640u);
        uint _319 = _316 ^ ((_316 << 15u) & 4022730752u);
        uint3 _327 = uint3(_231 + int3(0, 1, 0));
        uint _330 = _327.z;
        uint _332 = _330 + (_330 << 10u);
        uint _334 = _332 ^ (_332 >> 6u);
        uint _336 = _334 + (_334 << 3u);
        uint _338 = _336 ^ (_336 >> 11u);
        uint _344 = (((_327.y * 1664525u) + (_338 + (_338 << 15u))) + 1013904223u) * 1664525u;
        uint _346 = _344 ^ (_344 >> 11u);
        uint _349 = _346 ^ ((_346 << 7u) & 2636928640u);
        uint _352 = _349 ^ ((_349 << 15u) & 4022730752u);
        uint _358 = (((_327.x * 1664525u) + (_352 ^ (_352 >> 18u))) + 1013904223u) * 1664525u;
        uint _360 = _358 ^ (_358 >> 11u);
        uint _363 = _360 ^ ((_360 << 7u) & 2636928640u);
        uint _366 = _363 ^ ((_363 << 15u) & 4022730752u);
        uint3 _374 = uint3(_231 + int3(1, 1, 0));
        uint _377 = _374.z;
        uint _379 = _377 + (_377 << 10u);
        uint _381 = _379 ^ (_379 >> 6u);
        uint _383 = _381 + (_381 << 3u);
        uint _385 = _383 ^ (_383 >> 11u);
        uint _391 = (((_374.y * 1664525u) + (_385 + (_385 << 15u))) + 1013904223u) * 1664525u;
        uint _393 = _391 ^ (_391 >> 11u);
        uint _396 = _393 ^ ((_393 << 7u) & 2636928640u);
        uint _399 = _396 ^ ((_396 << 15u) & 4022730752u);
        uint _405 = (((_374.x * 1664525u) + (_399 ^ (_399 >> 18u))) + 1013904223u) * 1664525u;
        uint _407 = _405 ^ (_405 >> 11u);
        uint _410 = _407 ^ ((_407 << 7u) & 2636928640u);
        uint _413 = _410 ^ ((_410 << 15u) & 4022730752u);
        uint3 _421 = uint3(_231 + int3(0, 0, 1));
        uint _424 = _421.z;
        uint _426 = _424 + (_424 << 10u);
        uint _428 = _426 ^ (_426 >> 6u);
        uint _430 = _428 + (_428 << 3u);
        uint _432 = _430 ^ (_430 >> 11u);
        uint _438 = (((_421.y * 1664525u) + (_432 + (_432 << 15u))) + 1013904223u) * 1664525u;
        uint _440 = _438 ^ (_438 >> 11u);
        uint _443 = _440 ^ ((_440 << 7u) & 2636928640u);
        uint _446 = _443 ^ ((_443 << 15u) & 4022730752u);
        uint _452 = (((_421.x * 1664525u) + (_446 ^ (_446 >> 18u))) + 1013904223u) * 1664525u;
        uint _454 = _452 ^ (_452 >> 11u);
        uint _457 = _454 ^ ((_454 << 7u) & 2636928640u);
        uint _460 = _457 ^ ((_457 << 15u) & 4022730752u);
        uint3 _468 = uint3(_231 + int3(1, 0, 1));
        uint _471 = _468.z;
        uint _473 = _471 + (_471 << 10u);
        uint _475 = _473 ^ (_473 >> 6u);
        uint _477 = _475 + (_475 << 3u);
        uint _479 = _477 ^ (_477 >> 11u);
        uint _485 = (((_468.y * 1664525u) + (_479 + (_479 << 15u))) + 1013904223u) * 1664525u;
        uint _487 = _485 ^ (_485 >> 11u);
        uint _490 = _487 ^ ((_487 << 7u) & 2636928640u);
        uint _493 = _490 ^ ((_490 << 15u) & 4022730752u);
        uint _499 = (((_468.x * 1664525u) + (_493 ^ (_493 >> 18u))) + 1013904223u) * 1664525u;
        uint _501 = _499 ^ (_499 >> 11u);
        uint _504 = _501 ^ ((_501 << 7u) & 2636928640u);
        uint _507 = _504 ^ ((_504 << 15u) & 4022730752u);
        uint3 _515 = uint3(_231 + int3(0, 1, 1));
        uint _518 = _515.z;
        uint _520 = _518 + (_518 << 10u);
        uint _522 = _520 ^ (_520 >> 6u);
        uint _524 = _522 + (_522 << 3u);
        uint _526 = _524 ^ (_524 >> 11u);
        uint _532 = (((_515.y * 1664525u) + (_526 + (_526 << 15u))) + 1013904223u) * 1664525u;
        uint _534 = _532 ^ (_532 >> 11u);
        uint _537 = _534 ^ ((_534 << 7u) & 2636928640u);
        uint _540 = _537 ^ ((_537 << 15u) & 4022730752u);
        uint _546 = (((_515.x * 1664525u) + (_540 ^ (_540 >> 18u))) + 1013904223u) * 1664525u;
        uint _548 = _546 ^ (_546 >> 11u);
        uint _551 = _548 ^ ((_548 << 7u) & 2636928640u);
        uint _554 = _551 ^ ((_551 << 15u) & 4022730752u);
        uint3 _562 = uint3(_231 + int3(1, 1, 1));
        uint _565 = _562.z;
        uint _567 = _565 + (_565 << 10u);
        uint _569 = _567 ^ (_567 >> 6u);
        uint _571 = _569 + (_569 << 3u);
        uint _573 = _571 ^ (_571 >> 11u);
        uint _579 = (((_562.y * 1664525u) + (_573 + (_573 << 15u))) + 1013904223u) * 1664525u;
        uint _581 = _579 ^ (_579 >> 11u);
        uint _584 = _581 ^ ((_581 << 7u) & 2636928640u);
        uint _587 = _584 ^ ((_584 << 15u) & 4022730752u);
        uint _593 = (((_562.x * 1664525u) + (_587 ^ (_587 >> 18u))) + 1013904223u) * 1664525u;
        uint _595 = _593 ^ (_593 >> 11u);
        uint _598 = _595 ^ ((_595 << 7u) & 2636928640u);
        uint _601 = _598 ^ ((_598 << 15u) & 4022730752u);
        float3 _611 = (_232 * _232) * (3.0f.xxx - (_232 * 2.0f));
        float _612 = _611.x;
        float _613 = 1.0f - _612;
        float _615 = _611.y;
        float _616 = 1.0f - _615;
        float _617 = _611.z;
        float _640 = ((((1.0f - _617) * (((_616 * (((asfloat(((_272 ^ (_272 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _613) + ((asfloat(((_319 ^ (_319 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _612))) + (((asfloat(((_366 ^ (_366 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _613) * _615)) + (((asfloat(((_413 ^ (_413 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _612) * _615))) + ((((asfloat(((_460 ^ (_460 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _613) * _616) * _617)) + ((((asfloat(((_507 ^ (_507 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _612) * _616) * _617)) + ((((asfloat(((_554 ^ (_554 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _613) * _615) * _617);
        float _645 = _216 * 0.066666670143604278564453125f;
        float2 _658 = _217.xz * ((lerp(0.5f, 5.0f, (_640 + ((((asfloat(((_601 ^ (_601 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _612) * _615) * _617)) * clamp(1.0f - _645, 0.0f, 1.0f)) * (1.0f - smoothstep(0.0f, 1.0f, _216 + lerp(0.0f, -10.0f, nani_instance_data.Load<float>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 56))))) * (nani_instance_data.Load<float>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 56) * nani_instance_data.Load<float>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 56)));
        float3 _668;
        if (frame_constants_glade_theme_id == 4u)
        {
            float2 _666 = _658.xy * 2.0f;
            _668 = float3(_666.x, _217.y, _666.y);
        }
        else
        {
            _668 = float3(_658.x, _217.y, _658.y);
        }
        float3 _678;
        if (nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 48) == 1u)
        {
            float3 _677 = _668;
            _677.z = _668.z + (smoothstep(0.20000000298023223876953125f, 0.89999997615814208984375f, _668.y) * 0.800000011920928955078125f);
            _678 = _677;
        }
        else
        {
            _678 = _668;
        }
        float3 _683 = mul(_223, float4(_678, 1.0f));
        float _685 = _683.y - 0.1500000059604644775390625f;
        float3 _686 = _683;
        _686.y = _685;
        float2 _687 = _224.xz;
        int2 _689 = int2(floor(_687));
        float2 _690 = frac(_687);
        uint2 _691 = uint2(_689);
        uint _693 = _691.y;
        uint _695 = _693 + (_693 << 10u);
        uint _697 = _695 ^ (_695 >> 6u);
        uint _699 = _697 + (_697 << 3u);
        uint _701 = _699 ^ (_699 >> 11u);
        uint _707 = (((_691.x * 1664525u) + (_701 + (_701 << 15u))) + 1013904223u) * 1664525u;
        uint _709 = _707 ^ (_707 >> 11u);
        uint _712 = _709 ^ ((_709 << 7u) & 2636928640u);
        uint _715 = _712 ^ ((_712 << 15u) & 4022730752u);
        uint2 _723 = uint2(_689 + int2(1, 0));
        uint _725 = _723.y;
        uint _727 = _725 + (_725 << 10u);
        uint _729 = _727 ^ (_727 >> 6u);
        uint _731 = _729 + (_729 << 3u);
        uint _733 = _731 ^ (_731 >> 11u);
        uint _739 = (((_723.x * 1664525u) + (_733 + (_733 << 15u))) + 1013904223u) * 1664525u;
        uint _741 = _739 ^ (_739 >> 11u);
        uint _744 = _741 ^ ((_741 << 7u) & 2636928640u);
        uint _747 = _744 ^ ((_744 << 15u) & 4022730752u);
        uint2 _755 = uint2(_689 + int2(0, 1));
        uint _757 = _755.y;
        uint _759 = _757 + (_757 << 10u);
        uint _761 = _759 ^ (_759 >> 6u);
        uint _763 = _761 + (_761 << 3u);
        uint _765 = _763 ^ (_763 >> 11u);
        uint _771 = (((_755.x * 1664525u) + (_765 + (_765 << 15u))) + 1013904223u) * 1664525u;
        uint _773 = _771 ^ (_771 >> 11u);
        uint _776 = _773 ^ ((_773 << 7u) & 2636928640u);
        uint _779 = _776 ^ ((_776 << 15u) & 4022730752u);
        uint2 _787 = uint2(_689 + int2(1, 1));
        uint _789 = _787.y;
        uint _791 = _789 + (_789 << 10u);
        uint _793 = _791 ^ (_791 >> 6u);
        uint _795 = _793 + (_793 << 3u);
        uint _797 = _795 ^ (_795 >> 11u);
        uint _803 = (((_787.x * 1664525u) + (_797 + (_797 << 15u))) + 1013904223u) * 1664525u;
        uint _805 = _803 ^ (_803 >> 11u);
        uint _808 = _805 ^ ((_805 << 7u) & 2636928640u);
        uint _811 = _808 ^ ((_808 << 15u) & 4022730752u);
        float2 _821 = (_690 * _690) * (3.0f.xx - (_690 * 2.0f));
        float _822 = _821.x;
        float _823 = 1.0f - _822;
        float _825 = _821.y;
        float _836 = _209 + ((((1.0f - _825) * (((asfloat(((_715 ^ (_715 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _823) + ((asfloat(((_747 ^ (_747 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _822))) + (((asfloat(((_779 ^ (_779 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _823) * _825)) + (((asfloat(((_811 ^ (_811 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _822) * _825));
        float _837 = smoothstep(0.100000001490116119384765625f, 0.20000000298023223876953125f, _645);
        uint _838 = uint(nani_instance_data.Load<int>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 52));
        uint _840 = _838 + (_838 << 10u);
        uint _842 = _840 ^ (_840 >> 6u);
        uint _844 = _842 + (_842 << 3u);
        uint _846 = _844 ^ (_844 >> 11u);
        float _852 = asfloat(((_846 + (_846 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
        float _857 = (((_836 * (-0.800000011920928955078125f)) + (_678.y * 0.800000011920928955078125f)) + _852) * 0.4000000059604644775390625f;
        int _859 = int(floor(_857));
        float _860 = frac(_857);
        uint _861 = uint(_859);
        uint _863 = _861 + (_861 << 10u);
        uint _865 = _863 ^ (_863 >> 6u);
        uint _867 = _865 + (_865 << 3u);
        uint _869 = _867 ^ (_867 >> 11u);
        uint _875 = (((_861 * 1664525u) + (_869 + (_869 << 15u))) + 1013904223u) * 1664525u;
        uint _877 = _875 ^ (_875 >> 11u);
        uint _880 = _877 ^ ((_877 << 7u) & 2636928640u);
        uint _883 = _880 ^ ((_880 << 15u) & 4022730752u);
        uint _885 = _883 ^ (_883 >> 18u);
        uint _887 = _885 + (_885 << 10u);
        uint _889 = _887 ^ (_887 >> 6u);
        uint _891 = _889 + (_889 << 3u);
        uint _893 = _891 ^ (_891 >> 11u);
        uint _906 = uint(_859 + 1);
        uint _908 = _906 + (_906 << 10u);
        uint _910 = _908 ^ (_908 >> 6u);
        uint _912 = _910 + (_910 << 3u);
        uint _914 = _912 ^ (_912 >> 11u);
        uint _920 = (((_906 * 1664525u) + (_914 + (_914 << 15u))) + 1013904223u) * 1664525u;
        uint _922 = _920 ^ (_920 >> 11u);
        uint _925 = _922 ^ ((_922 << 7u) & 2636928640u);
        uint _928 = _925 ^ ((_925 << 15u) & 4022730752u);
        uint _930 = _928 ^ (_928 >> 18u);
        uint _932 = _930 + (_930 << 10u);
        uint _934 = _932 ^ (_932 >> 6u);
        uint _936 = _934 + (_934 << 3u);
        uint _938 = _936 ^ (_936 >> 11u);
        float _953 = (_860 * _860) * (3.0f - (2.0f * _860));
        float _962 = (_836 * (-0.3200000226497650146484375f)) + _852;
        int _964 = int(floor(_962));
        float _965 = frac(_962);
        uint _966 = uint(_964);
        uint _968 = _966 + (_966 << 10u);
        uint _970 = _968 ^ (_968 >> 6u);
        uint _972 = _970 + (_970 << 3u);
        uint _974 = _972 ^ (_972 >> 11u);
        uint _980 = (((_966 * 1664525u) + (_974 + (_974 << 15u))) + 1013904223u) * 1664525u;
        uint _982 = _980 ^ (_980 >> 11u);
        uint _985 = _982 ^ ((_982 << 7u) & 2636928640u);
        uint _988 = _985 ^ ((_985 << 15u) & 4022730752u);
        uint _990 = _988 ^ (_988 >> 18u);
        uint _992 = _990 + (_990 << 10u);
        uint _994 = _992 ^ (_992 >> 6u);
        uint _996 = _994 + (_994 << 3u);
        uint _998 = _996 ^ (_996 >> 11u);
        uint _1011 = uint(_964 + 1);
        uint _1013 = _1011 + (_1011 << 10u);
        uint _1015 = _1013 ^ (_1013 >> 6u);
        uint _1017 = _1015 + (_1015 << 3u);
        uint _1019 = _1017 ^ (_1017 >> 11u);
        uint _1025 = (((_1011 * 1664525u) + (_1019 + (_1019 << 15u))) + 1013904223u) * 1664525u;
        uint _1027 = _1025 ^ (_1025 >> 11u);
        uint _1030 = _1027 ^ ((_1027 << 7u) & 2636928640u);
        uint _1033 = _1030 ^ ((_1030 << 15u) & 4022730752u);
        uint _1035 = _1033 ^ (_1033 >> 18u);
        uint _1037 = _1035 + (_1035 << 10u);
        uint _1039 = _1037 ^ (_1037 >> 6u);
        uint _1041 = _1039 + (_1039 << 3u);
        uint _1043 = _1041 ^ (_1041 >> 11u);
        float _1058 = (_965 * _965) * (3.0f - (2.0f * _965));
        float _1066 = smoothstep(0.300000011920928955078125f, 0.0f, _645);
        float2 _1069 = ((((((float2(asfloat((_885 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_893 + (_893 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * (1.0f - _953)) + (float2(asfloat((_930 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_938 + (_938 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _953)) * 2.0f) - 1.0f.xx) * 2.0f) - ((((((float2(asfloat((_990 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_998 + (_998 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * (1.0f - _1058)) + (float2(asfloat((_1035 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_1043 + (_1043 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _1058)) * 2.0f) - 1.0f.xx) * 2.0f) * _1066)) * 0.300000011920928955078125f;
        float _1077 = (((_836 * (-2.1296999454498291015625f)) + (_678.y * 2.1296999454498291015625f)) + _852) * 0.4000000059604644775390625f;
        int _1079 = int(floor(_1077));
        float _1080 = frac(_1077);
        uint _1081 = uint(_1079);
        uint _1083 = _1081 + (_1081 << 10u);
        uint _1085 = _1083 ^ (_1083 >> 6u);
        uint _1087 = _1085 + (_1085 << 3u);
        uint _1089 = _1087 ^ (_1087 >> 11u);
        uint _1095 = (((_1081 * 1664525u) + (_1089 + (_1089 << 15u))) + 1013904223u) * 1664525u;
        uint _1097 = _1095 ^ (_1095 >> 11u);
        uint _1100 = _1097 ^ ((_1097 << 7u) & 2636928640u);
        uint _1103 = _1100 ^ ((_1100 << 15u) & 4022730752u);
        uint _1105 = _1103 ^ (_1103 >> 18u);
        uint _1107 = _1105 + (_1105 << 10u);
        uint _1109 = _1107 ^ (_1107 >> 6u);
        uint _1111 = _1109 + (_1109 << 3u);
        uint _1113 = _1111 ^ (_1111 >> 11u);
        uint _1126 = uint(_1079 + 1);
        uint _1128 = _1126 + (_1126 << 10u);
        uint _1130 = _1128 ^ (_1128 >> 6u);
        uint _1132 = _1130 + (_1130 << 3u);
        uint _1134 = _1132 ^ (_1132 >> 11u);
        uint _1140 = (((_1126 * 1664525u) + (_1134 + (_1134 << 15u))) + 1013904223u) * 1664525u;
        uint _1142 = _1140 ^ (_1140 >> 11u);
        uint _1145 = _1142 ^ ((_1142 << 7u) & 2636928640u);
        uint _1148 = _1145 ^ ((_1145 << 15u) & 4022730752u);
        uint _1150 = _1148 ^ (_1148 >> 18u);
        uint _1152 = _1150 + (_1150 << 10u);
        uint _1154 = _1152 ^ (_1152 >> 6u);
        uint _1156 = _1154 + (_1154 << 3u);
        uint _1158 = _1156 ^ (_1156 >> 11u);
        float _1173 = (_1080 * _1080) * (3.0f - (2.0f * _1080));
        float _1182 = (_836 * (-0.851880013942718505859375f)) + _852;
        int _1184 = int(floor(_1182));
        float _1185 = frac(_1182);
        uint _1186 = uint(_1184);
        uint _1188 = _1186 + (_1186 << 10u);
        uint _1190 = _1188 ^ (_1188 >> 6u);
        uint _1192 = _1190 + (_1190 << 3u);
        uint _1194 = _1192 ^ (_1192 >> 11u);
        uint _1200 = (((_1186 * 1664525u) + (_1194 + (_1194 << 15u))) + 1013904223u) * 1664525u;
        uint _1202 = _1200 ^ (_1200 >> 11u);
        uint _1205 = _1202 ^ ((_1202 << 7u) & 2636928640u);
        uint _1208 = _1205 ^ ((_1205 << 15u) & 4022730752u);
        uint _1210 = _1208 ^ (_1208 >> 18u);
        uint _1212 = _1210 + (_1210 << 10u);
        uint _1214 = _1212 ^ (_1212 >> 6u);
        uint _1216 = _1214 + (_1214 << 3u);
        uint _1218 = _1216 ^ (_1216 >> 11u);
        uint _1231 = uint(_1184 + 1);
        uint _1233 = _1231 + (_1231 << 10u);
        uint _1235 = _1233 ^ (_1233 >> 6u);
        uint _1237 = _1235 + (_1235 << 3u);
        uint _1239 = _1237 ^ (_1237 >> 11u);
        uint _1245 = (((_1231 * 1664525u) + (_1239 + (_1239 << 15u))) + 1013904223u) * 1664525u;
        uint _1247 = _1245 ^ (_1245 >> 11u);
        uint _1250 = _1247 ^ ((_1247 << 7u) & 2636928640u);
        uint _1253 = _1250 ^ ((_1250 << 15u) & 4022730752u);
        uint _1255 = _1253 ^ (_1253 >> 18u);
        uint _1257 = _1255 + (_1255 << 10u);
        uint _1259 = _1257 ^ (_1257 >> 6u);
        uint _1261 = _1259 + (_1259 << 3u);
        uint _1263 = _1261 ^ (_1261 >> 11u);
        float _1278 = (_1185 * _1185) * (3.0f - (2.0f * _1185));
        float2 _1288 = ((((((float2(asfloat((_1105 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_1113 + (_1113 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * (1.0f - _1173)) + (float2(asfloat((_1150 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_1158 + (_1158 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _1173)) * 2.0f) - 1.0f.xx) * 2.0f) - ((((((float2(asfloat((_1210 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_1218 + (_1218 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * (1.0f - _1278)) + (float2(asfloat((_1255 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_1263 + (_1263 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _1278)) * 2.0f) - 1.0f.xx) * 2.0f) * _1066)) * 0.20000000298023223876953125f;
        float _1296 = (((_836 * (-3.3970000743865966796875f)) + (_678.y * 3.3970000743865966796875f)) + _852) * 0.4000000059604644775390625f;
        int _1298 = int(floor(_1296));
        float _1299 = frac(_1296);
        uint _1300 = uint(_1298);
        uint _1302 = _1300 + (_1300 << 10u);
        uint _1304 = _1302 ^ (_1302 >> 6u);
        uint _1306 = _1304 + (_1304 << 3u);
        uint _1308 = _1306 ^ (_1306 >> 11u);
        uint _1314 = (((_1300 * 1664525u) + (_1308 + (_1308 << 15u))) + 1013904223u) * 1664525u;
        uint _1316 = _1314 ^ (_1314 >> 11u);
        uint _1319 = _1316 ^ ((_1316 << 7u) & 2636928640u);
        uint _1322 = _1319 ^ ((_1319 << 15u) & 4022730752u);
        uint _1324 = _1322 ^ (_1322 >> 18u);
        uint _1326 = _1324 + (_1324 << 10u);
        uint _1328 = _1326 ^ (_1326 >> 6u);
        uint _1330 = _1328 + (_1328 << 3u);
        uint _1332 = _1330 ^ (_1330 >> 11u);
        uint _1345 = uint(_1298 + 1);
        uint _1347 = _1345 + (_1345 << 10u);
        uint _1349 = _1347 ^ (_1347 >> 6u);
        uint _1351 = _1349 + (_1349 << 3u);
        uint _1353 = _1351 ^ (_1351 >> 11u);
        uint _1359 = (((_1345 * 1664525u) + (_1353 + (_1353 << 15u))) + 1013904223u) * 1664525u;
        uint _1361 = _1359 ^ (_1359 >> 11u);
        uint _1364 = _1361 ^ ((_1361 << 7u) & 2636928640u);
        uint _1367 = _1364 ^ ((_1364 << 15u) & 4022730752u);
        uint _1369 = _1367 ^ (_1367 >> 18u);
        uint _1371 = _1369 + (_1369 << 10u);
        uint _1373 = _1371 ^ (_1371 >> 6u);
        uint _1375 = _1373 + (_1373 << 3u);
        uint _1377 = _1375 ^ (_1375 >> 11u);
        float _1392 = (_1299 * _1299) * (3.0f - (2.0f * _1299));
        float _1401 = (_836 * (-1.35880005359649658203125f)) + _852;
        int _1403 = int(floor(_1401));
        float _1404 = frac(_1401);
        uint _1405 = uint(_1403);
        uint _1407 = _1405 + (_1405 << 10u);
        uint _1409 = _1407 ^ (_1407 >> 6u);
        uint _1411 = _1409 + (_1409 << 3u);
        uint _1413 = _1411 ^ (_1411 >> 11u);
        uint _1419 = (((_1405 * 1664525u) + (_1413 + (_1413 << 15u))) + 1013904223u) * 1664525u;
        uint _1421 = _1419 ^ (_1419 >> 11u);
        uint _1424 = _1421 ^ ((_1421 << 7u) & 2636928640u);
        uint _1427 = _1424 ^ ((_1424 << 15u) & 4022730752u);
        uint _1429 = _1427 ^ (_1427 >> 18u);
        uint _1431 = _1429 + (_1429 << 10u);
        uint _1433 = _1431 ^ (_1431 >> 6u);
        uint _1435 = _1433 + (_1433 << 3u);
        uint _1437 = _1435 ^ (_1435 >> 11u);
        uint _1450 = uint(_1403 + 1);
        uint _1452 = _1450 + (_1450 << 10u);
        uint _1454 = _1452 ^ (_1452 >> 6u);
        uint _1456 = _1454 + (_1454 << 3u);
        uint _1458 = _1456 ^ (_1456 >> 11u);
        uint _1464 = (((_1450 * 1664525u) + (_1458 + (_1458 << 15u))) + 1013904223u) * 1664525u;
        uint _1466 = _1464 ^ (_1464 >> 11u);
        uint _1469 = _1466 ^ ((_1466 << 7u) & 2636928640u);
        uint _1472 = _1469 ^ ((_1469 << 15u) & 4022730752u);
        uint _1474 = _1472 ^ (_1472 >> 18u);
        uint _1476 = _1474 + (_1474 << 10u);
        uint _1478 = _1476 ^ (_1476 >> 6u);
        uint _1480 = _1478 + (_1478 << 3u);
        uint _1482 = _1480 ^ (_1480 >> 11u);
        float _1497 = (_1404 * _1404) * (3.0f - (2.0f * _1404));
        float2 _1507 = ((((((float2(asfloat((_1324 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_1332 + (_1332 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * (1.0f - _1392)) + (float2(asfloat((_1369 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_1377 + (_1377 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _1392)) * 2.0f) - 1.0f.xx) * 2.0f) - ((((((float2(asfloat((_1429 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_1437 + (_1437 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * (1.0f - _1497)) + (float2(asfloat((_1474 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_1482 + (_1482 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _1497)) * 2.0f) - 1.0f.xx) * 2.0f) * _1066)) * 0.0500000007450580596923828125f;
        float2 _1509 = ((_686.xz + (_1069 * _837)).xy + (_1288 * _837)).xy + _1507;
        _1524 = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(_1509.x, _685, _1509.y, 1.0f));
        _1525 = mul(_223, float4(normalize(float3(_195, 0.0f, _197)), 0.0f));
        _1526 = float3(_1509.x, _686.y, _1509.y);
        _1527 = _645;
        break;
    } while(false);
    gl_Position = _1524;
    out_var_C0 = _1525;
    out_var_C1 = _1526;
    out_var_C2 = _1527;
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
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)

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


static float3 in_var_C1;
static float in_var_C2;

struct SPIRV_Cross_Input
{
    [[vk::location(1)]] float3 in_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float in_var_C2 : TEXCOORD2;
};

void frag_main()
{
    bool _98;
    if (!(in_var_C2 < 0.0f))
    {
        _98 = in_var_C2 > 1.0f;
    }
    else
    {
        _98 = true;
    }
    if (_98)
    {
        discard;
    }
    float3 _110 = (in_var_C1 * 2.0f) + (float3(0.0f, -4.0f, 0.0f) * (float(frame_constants_game_time_sec) + frame_constants_game_time_frac));
    int3 _112 = int3(floor(_110));
    float3 _113 = frac(_110);
    uint3 _114 = uint3(_112);
    uint _117 = _114.z;
    uint _119 = _117 + (_117 << 10u);
    uint _121 = _119 ^ (_119 >> 6u);
    uint _123 = _121 + (_121 << 3u);
    uint _125 = _123 ^ (_123 >> 11u);
    uint _131 = (((_114.y * 1664525u) + (_125 + (_125 << 15u))) + 1013904223u) * 1664525u;
    uint _133 = _131 ^ (_131 >> 11u);
    uint _136 = _133 ^ ((_133 << 7u) & 2636928640u);
    uint _139 = _136 ^ ((_136 << 15u) & 4022730752u);
    uint _145 = (((_114.x * 1664525u) + (_139 ^ (_139 >> 18u))) + 1013904223u) * 1664525u;
    uint _147 = _145 ^ (_145 >> 11u);
    uint _150 = _147 ^ ((_147 << 7u) & 2636928640u);
    uint _153 = _150 ^ ((_150 << 15u) & 4022730752u);
    uint3 _161 = uint3(_112 + int3(1, 0, 0));
    uint _164 = _161.z;
    uint _166 = _164 + (_164 << 10u);
    uint _168 = _166 ^ (_166 >> 6u);
    uint _170 = _168 + (_168 << 3u);
    uint _172 = _170 ^ (_170 >> 11u);
    uint _178 = (((_161.y * 1664525u) + (_172 + (_172 << 15u))) + 1013904223u) * 1664525u;
    uint _180 = _178 ^ (_178 >> 11u);
    uint _183 = _180 ^ ((_180 << 7u) & 2636928640u);
    uint _186 = _183 ^ ((_183 << 15u) & 4022730752u);
    uint _192 = (((_161.x * 1664525u) + (_186 ^ (_186 >> 18u))) + 1013904223u) * 1664525u;
    uint _194 = _192 ^ (_192 >> 11u);
    uint _197 = _194 ^ ((_194 << 7u) & 2636928640u);
    uint _200 = _197 ^ ((_197 << 15u) & 4022730752u);
    uint3 _208 = uint3(_112 + int3(0, 1, 0));
    uint _211 = _208.z;
    uint _213 = _211 + (_211 << 10u);
    uint _215 = _213 ^ (_213 >> 6u);
    uint _217 = _215 + (_215 << 3u);
    uint _219 = _217 ^ (_217 >> 11u);
    uint _225 = (((_208.y * 1664525u) + (_219 + (_219 << 15u))) + 1013904223u) * 1664525u;
    uint _227 = _225 ^ (_225 >> 11u);
    uint _230 = _227 ^ ((_227 << 7u) & 2636928640u);
    uint _233 = _230 ^ ((_230 << 15u) & 4022730752u);
    uint _239 = (((_208.x * 1664525u) + (_233 ^ (_233 >> 18u))) + 1013904223u) * 1664525u;
    uint _241 = _239 ^ (_239 >> 11u);
    uint _244 = _241 ^ ((_241 << 7u) & 2636928640u);
    uint _247 = _244 ^ ((_244 << 15u) & 4022730752u);
    uint3 _255 = uint3(_112 + int3(1, 1, 0));
    uint _258 = _255.z;
    uint _260 = _258 + (_258 << 10u);
    uint _262 = _260 ^ (_260 >> 6u);
    uint _264 = _262 + (_262 << 3u);
    uint _266 = _264 ^ (_264 >> 11u);
    uint _272 = (((_255.y * 1664525u) + (_266 + (_266 << 15u))) + 1013904223u) * 1664525u;
    uint _274 = _272 ^ (_272 >> 11u);
    uint _277 = _274 ^ ((_274 << 7u) & 2636928640u);
    uint _280 = _277 ^ ((_277 << 15u) & 4022730752u);
    uint _286 = (((_255.x * 1664525u) + (_280 ^ (_280 >> 18u))) + 1013904223u) * 1664525u;
    uint _288 = _286 ^ (_286 >> 11u);
    uint _291 = _288 ^ ((_288 << 7u) & 2636928640u);
    uint _294 = _291 ^ ((_291 << 15u) & 4022730752u);
    uint3 _302 = uint3(_112 + int3(0, 0, 1));
    uint _305 = _302.z;
    uint _307 = _305 + (_305 << 10u);
    uint _309 = _307 ^ (_307 >> 6u);
    uint _311 = _309 + (_309 << 3u);
    uint _313 = _311 ^ (_311 >> 11u);
    uint _319 = (((_302.y * 1664525u) + (_313 + (_313 << 15u))) + 1013904223u) * 1664525u;
    uint _321 = _319 ^ (_319 >> 11u);
    uint _324 = _321 ^ ((_321 << 7u) & 2636928640u);
    uint _327 = _324 ^ ((_324 << 15u) & 4022730752u);
    uint _333 = (((_302.x * 1664525u) + (_327 ^ (_327 >> 18u))) + 1013904223u) * 1664525u;
    uint _335 = _333 ^ (_333 >> 11u);
    uint _338 = _335 ^ ((_335 << 7u) & 2636928640u);
    uint _341 = _338 ^ ((_338 << 15u) & 4022730752u);
    uint3 _349 = uint3(_112 + int3(1, 0, 1));
    uint _352 = _349.z;
    uint _354 = _352 + (_352 << 10u);
    uint _356 = _354 ^ (_354 >> 6u);
    uint _358 = _356 + (_356 << 3u);
    uint _360 = _358 ^ (_358 >> 11u);
    uint _366 = (((_349.y * 1664525u) + (_360 + (_360 << 15u))) + 1013904223u) * 1664525u;
    uint _368 = _366 ^ (_366 >> 11u);
    uint _371 = _368 ^ ((_368 << 7u) & 2636928640u);
    uint _374 = _371 ^ ((_371 << 15u) & 4022730752u);
    uint _380 = (((_349.x * 1664525u) + (_374 ^ (_374 >> 18u))) + 1013904223u) * 1664525u;
    uint _382 = _380 ^ (_380 >> 11u);
    uint _385 = _382 ^ ((_382 << 7u) & 2636928640u);
    uint _388 = _385 ^ ((_385 << 15u) & 4022730752u);
    uint3 _396 = uint3(_112 + int3(0, 1, 1));
    uint _399 = _396.z;
    uint _401 = _399 + (_399 << 10u);
    uint _403 = _401 ^ (_401 >> 6u);
    uint _405 = _403 + (_403 << 3u);
    uint _407 = _405 ^ (_405 >> 11u);
    uint _413 = (((_396.y * 1664525u) + (_407 + (_407 << 15u))) + 1013904223u) * 1664525u;
    uint _415 = _413 ^ (_413 >> 11u);
    uint _418 = _415 ^ ((_415 << 7u) & 2636928640u);
    uint _421 = _418 ^ ((_418 << 15u) & 4022730752u);
    uint _427 = (((_396.x * 1664525u) + (_421 ^ (_421 >> 18u))) + 1013904223u) * 1664525u;
    uint _429 = _427 ^ (_427 >> 11u);
    uint _432 = _429 ^ ((_429 << 7u) & 2636928640u);
    uint _435 = _432 ^ ((_432 << 15u) & 4022730752u);
    uint3 _443 = uint3(_112 + int3(1, 1, 1));
    uint _446 = _443.z;
    uint _448 = _446 + (_446 << 10u);
    uint _450 = _448 ^ (_448 >> 6u);
    uint _452 = _450 + (_450 << 3u);
    uint _454 = _452 ^ (_452 >> 11u);
    uint _460 = (((_443.y * 1664525u) + (_454 + (_454 << 15u))) + 1013904223u) * 1664525u;
    uint _462 = _460 ^ (_460 >> 11u);
    uint _465 = _462 ^ ((_462 << 7u) & 2636928640u);
    uint _468 = _465 ^ ((_465 << 15u) & 4022730752u);
    uint _474 = (((_443.x * 1664525u) + (_468 ^ (_468 >> 18u))) + 1013904223u) * 1664525u;
    uint _476 = _474 ^ (_474 >> 11u);
    uint _479 = _476 ^ ((_476 << 7u) & 2636928640u);
    uint _482 = _479 ^ ((_479 << 15u) & 4022730752u);
    float3 _492 = (_113 * _113) * (3.0f.xxx - (_113 * 2.0f));
    float _493 = _492.x;
    float _494 = 1.0f - _493;
    float _496 = _492.y;
    float _497 = 1.0f - _496;
    float _498 = _492.z;
    float _521 = ((((1.0f - _498) * (((_497 * (((asfloat(((_153 ^ (_153 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _494) + ((asfloat(((_200 ^ (_200 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _493))) + (((asfloat(((_247 ^ (_247 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _494) * _496)) + (((asfloat(((_294 ^ (_294 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _493) * _496))) + ((((asfloat(((_341 ^ (_341 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _494) * _497) * _498)) + ((((asfloat(((_388 ^ (_388 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _493) * _497) * _498)) + ((((asfloat(((_435 ^ (_435 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _494) * _496) * _498);
    if ((_521 + ((((asfloat(((_482 ^ (_482 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _493) * _496) * _498)) > smoothstep(0.800000011920928955078125f, 0.100000001490116119384765625f, in_var_C2))
    {
        discard;
    }
}

void ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C1 = stage_input.in_var_C1;
    in_var_C2 = stage_input.in_var_C2;
    frag_main();
}
#pragma dxc diagnostic pop
#endif
