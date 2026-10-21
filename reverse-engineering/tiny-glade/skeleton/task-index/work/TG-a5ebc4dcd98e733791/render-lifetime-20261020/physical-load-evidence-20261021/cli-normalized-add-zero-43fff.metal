#pragma clang diagnostic ignored "-Wmissing-prototypes"
#pragma clang diagnostic ignored "-Wincompatible-pointer-types-discards-qualifiers"

#include <metal_stdlib>
#include <simd/simd.h>

using namespace metal;

template<typename T>
struct spvDescriptor
{
    T value;
};

template<typename T>
struct spvDescriptorArray
{
    spvDescriptorArray(const device spvDescriptor<T>* ptr_) : ptr(&ptr_->value) {}
    spvDescriptorArray(const device void *ptr_) : spvDescriptorArray(static_cast<const device spvDescriptor<T>*>(ptr_)) {}
    const device T& operator [] (size_t i) const { return ptr[i]; }
    const device T* ptr;
};

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

struct type_ConstantBuffer_FrameConstants
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
    packed_float3 editing_highlight_color;
    float time_of_day;
    packed_float3 dir_to_sun;
    uint camera_idx;
    packed_float3 dir_to_moon;
    uint render_overrides;
    float global_mip_bias;
    uint highlighted_transient_stair_edge_id;
    uint pad1;
    uint pad2;
    float4 debug_tweakers;
    ViewConstants view_constants[5];
};

struct type_Globals
{
    uint current_view_index;
    int highlight_bricks_to_delete;
    char _m2_pad[8];
    packed_float3 brush_position;
    float eraser_mult;
    uint highlight_mode;
    uint highlighted_wall_id;
    uint2 normal_map_dims;
    uint2 mortar_dims;
};

struct type_StructuredBuffer_uint
{
    uint _m0[1];
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

struct type_StructuredBuffer_PlasterInstanceData
{
    PlasterInstanceData _m0[1];
};

struct _971
{
    uint _m0[4];
};

struct _975
{
    uint _m0;
};

struct spvDescriptorSetBuffer0
{
    spvDescriptor<const device type_ConstantBuffer_FrameConstants *> frame_constants [[id(0)]][1] /* unsized array hack */;
    // Overlapping binding: spvDescriptor<const device type_Globals *> _Globals [[id(0)]][1] /* unsized array hack */;
    // Overlapping binding: spvDescriptor<const device type_StructuredBuffer_uint *> instance_indirection_buffer [[id(0)]][1] /* unsized array hack */;
    // Overlapping binding: spvDescriptor<const device type_StructuredBuffer_PlasterInstanceData *> nani_instance_data [[id(0)]][1] /* unsized array hack */;
    // Overlapping binding: spvDescriptor<const device _971 *> m_974 [[id(0)]][1] /* unsized array hack */;
};

struct vs_main_out
{
    uint out_var_C0 [[user(locn0)]];
    float3 out_var_C1 [[user(locn1)]];
    float3 out_var_C2 [[user(locn2)]];
    float2 out_var_C3 [[user(locn3)]];
    float3 out_var_C4 [[user(locn4)]];
    float3 out_var_C5 [[user(locn5)]];
    float out_var_C6 [[user(locn6)]];
    float out_var_C7 [[user(locn7)]];
    float out_var_C8 [[user(locn8)]];
    float3 out_var_C9_0 [[user(locn9)]];
    float3 out_var_C9_1 [[user(locn10)]];
    float3 out_var_C9_2 [[user(locn11)]];
    float4 gl_Position [[position]];
};

vertex vs_main_out vs_main(const device spvDescriptorSetBuffer0& spvDescriptorSet0 [[buffer(0)]], constant _975& _977 [[buffer(1)]], uint gl_VertexIndex [[vertex_id]], uint gl_InstanceIndex [[instance_id]])
{
    spvDescriptorArray<const device type_ConstantBuffer_FrameConstants*> frame_constants {spvDescriptorSet0.frame_constants};
    spvDescriptorArray<const device type_Globals*> _Globals {reinterpret_cast<spvDescriptor<const device type_Globals *> const device *>(&spvDescriptorSet0.frame_constants)};
    spvDescriptorArray<const device type_StructuredBuffer_uint*> instance_indirection_buffer {reinterpret_cast<spvDescriptor<const device type_StructuredBuffer_uint *> const device *>(&spvDescriptorSet0.frame_constants)};
    spvDescriptorArray<const device type_StructuredBuffer_PlasterInstanceData*> nani_instance_data {reinterpret_cast<spvDescriptor<const device type_StructuredBuffer_PlasterInstanceData *> const device *>(&spvDescriptorSet0.frame_constants)};
    spvDescriptorArray<const device _971*> _974 {reinterpret_cast<spvDescriptor<const device _971 *> const device *>(&spvDescriptorSet0.frame_constants)};

    vs_main_out out = {};
    float3x3 out_var_C9 = {};
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
        uint _193 = gl_VertexIndex & 65535u;
        if (_193 > nani_instance_data[_974[_977._m0]->_m0[2u]]->_m0[instance_indirection_buffer[_974[_977._m0]->_m0[1u]]->_m0[gl_InstanceIndex]].quad_count)
        {
            _958 = float4(as_type<float>(0x7fbfffffu /* nan */));
            _959 = 0u;
            _960 = float3(as_type<float>(0x7fbfffffu /* nan */));
            _961 = float3(as_type<float>(0x7fbfffffu /* nan */));
            _962 = float2(as_type<float>(0x7fbfffffu /* nan */));
            _963 = float3(as_type<float>(0x7fbfffffu /* nan */));
            _964 = float3(as_type<float>(0x7fbfffffu /* nan */));
            _965 = as_type<float>(0x7fbfffffu /* nan */);
            _966 = as_type<float>(0x7fbfffffu /* nan */);
            _967 = as_type<float>(0x7fbfffffu /* nan */);
            _968 = float3x3(float3(as_type<float>(0x7fbfffffu /* nan */)), float3(as_type<float>(0x7fbfffffu /* nan */)), float3(as_type<float>(0x7fbfffffu /* nan */)));
            break;
        }
        uint2 _174 = nani_instance_data[_974[_977._m0]->_m0[2u]]->_m0[instance_indirection_buffer[_974[_977._m0]->_m0[1u]]->_m0[gl_InstanceIndex]].verts_gpu_address;
        uint _201 = 48u * (_193 + ((gl_VertexIndex >> 16u) * uint(nani_instance_data[_974[_977._m0]->_m0[2u]]->_m0[instance_indirection_buffer[_974[_977._m0]->_m0[1u]]->_m0[gl_InstanceIndex]].row_stride)));
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
        const device uint4* _236 = reinterpret_cast<const device uint4*>(as_type<ulong>(_172));
        uint4 _173 = *_236;
        uint2 _176 = nani_instance_data[_974[_977._m0]->_m0[2u]]->_m0[instance_indirection_buffer[_974[_977._m0]->_m0[1u]]->_m0[gl_InstanceIndex]].verts_gpu_address;
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
        const device uint4* _273 = reinterpret_cast<const device uint4*>(as_type<ulong>(_171));
        uint4 _175 = *_273;
        uint2 _177 = nani_instance_data[_974[_977._m0]->_m0[2u]]->_m0[instance_indirection_buffer[_974[_977._m0]->_m0[1u]]->_m0[gl_InstanceIndex]].verts_gpu_address;
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
        uint2 _311 = *(reinterpret_cast<const device uint2*>(as_type<ulong>(_170 + uint2(0u))));
        float3 _318 = as_type<float3>((*_273).xyz);
        float4 _326 = float4(as_type<float3>((*_236).xyz), 1.0);
        float4 _178 = _326;
        float3 _328 = _326.xyz * 0.4000000059604644775390625;
        int3 _330 = int3(floor(_328));
        float3 _331 = fract(_328);
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
        _165.x = as_type<float>((_377 & 8388607u) | 1065353216u) - 1.0;
        _165.y = as_type<float>(((_385 + (_385 << 15u)) & 8388607u) | 1065353216u) - 1.0;
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
        _164.x = as_type<float>((_441 & 8388607u) | 1065353216u) - 1.0;
        _164.y = as_type<float>(((_449 + (_449 << 15u)) & 8388607u) | 1065353216u) - 1.0;
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
        _163.x = as_type<float>((_505 & 8388607u) | 1065353216u) - 1.0;
        _163.y = as_type<float>(((_513 + (_513 << 15u)) & 8388607u) | 1065353216u) - 1.0;
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
        _162.x = as_type<float>((_569 & 8388607u) | 1065353216u) - 1.0;
        _162.y = as_type<float>(((_577 + (_577 << 15u)) & 8388607u) | 1065353216u) - 1.0;
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
        _161.x = as_type<float>((_633 & 8388607u) | 1065353216u) - 1.0;
        _161.y = as_type<float>(((_641 + (_641 << 15u)) & 8388607u) | 1065353216u) - 1.0;
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
        _160.x = as_type<float>((_697 & 8388607u) | 1065353216u) - 1.0;
        _160.y = as_type<float>(((_705 + (_705 << 15u)) & 8388607u) | 1065353216u) - 1.0;
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
        _159.x = as_type<float>((_761 & 8388607u) | 1065353216u) - 1.0;
        _159.y = as_type<float>(((_769 + (_769 << 15u)) & 8388607u) | 1065353216u) - 1.0;
        uint3 _783 = uint3(_330 + int3(1));
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
        _158.x = as_type<float>((_825 & 8388607u) | 1065353216u) - 1.0;
        _158.y = as_type<float>(((_833 + (_833 << 15u)) & 8388607u) | 1065353216u) - 1.0;
        float3 _168 = (_331 * _331) * (float3(3.0) - (_331 * 2.0));
        float2 _927 = (((((((((_165 * (1.0 - _168.x)) * (1.0 - _168.y)) * (1.0 - _168.z)) + (((_164 * _168.x) * (1.0 - _168.y)) * (1.0 - _168.z))) + (((_163 * (1.0 - _168.x)) * _168.y) * (1.0 - _168.z))) + (((_162 * _168.x) * _168.y) * (1.0 - _168.z))) + (((_161 * (1.0 - _168.x)) * (1.0 - _168.y)) * _168.z)) + (((_160 * _168.x) * (1.0 - _168.y)) * _168.z)) + (((_159 * (1.0 - _168.x)) * _168.y) * _168.z)) + (((_158 * _168.x) * _168.y) * _168.z);
        float2 _169 = fast::clamp((_927 - float2(0.5)) * 2.5, float2(-0.5), float2(0.5));
        float4 _937 = _178;
        float3 _939 = _937.xyz + (float3(_169.x, 0.0, _169.y) * 0.0599999986588954925537109375);
        float4 _940 = float4(_939.x, _939.y, _939.z, _937.w);
        _178 = _940;
        float3 _947 = cross(_318, float3(0.0, 1.0, 0.0));
        _958 = frame_constants[_974[_977._m0]->_m0[3u]]->view_constants[_Globals[_974[_977._m0]->_m0[0u]]->current_view_index].world_to_sample * _940;
        _959 = instance_indirection_buffer[_974[_977._m0]->_m0[1u]]->_m0[gl_InstanceIndex];
        _960 = _318;
        _961 = _939.xyz;
        _962 = as_type<float2>(_311);
        _963 = _947;
        _964 = float3(0.0, 1.0, 0.0);
        _965 = as_type<float>(_173.w);
        _966 = as_type<float>(_175.w) + (1.0 - smoothstep(0.0, 1.0, _178.y));
        _967 = nani_instance_data[_974[_977._m0]->_m0[2u]]->_m0[instance_indirection_buffer[_974[_977._m0]->_m0[1u]]->_m0[gl_InstanceIndex]].peel_strength;
        _968 = transpose(float3x3(fast::normalize(_947), fast::normalize(float3(0.0, 1.0, 0.0)), fast::normalize(_318)));
        break;
    } while(false);
    out.gl_Position = _958;
    out.out_var_C0 = _959;
    out.out_var_C1 = _960;
    out.out_var_C2 = _961;
    out.out_var_C3 = _962;
    out.out_var_C4 = _963;
    out.out_var_C5 = _964;
    out.out_var_C6 = _965;
    out.out_var_C7 = _966;
    out.out_var_C8 = _967;
    out_var_C9 = _968;
    out.out_var_C9_0 = out_var_C9[0];
    out.out_var_C9_1 = out_var_C9[1];
    out.out_var_C9_2 = out_var_C9[2];
    return out;
}

