// DERIVED by SPIRV-Cross from original packaged SPIR-V; NOT original HLSL/OpSource.
#version 460
#extension GL_EXT_nonuniform_qualifier : require
#extension GL_EXT_scalar_block_layout : require
layout(local_size_x = 8, local_size_y = 8, local_size_z = 1) in;

struct DrawIndexedIndirectCommand
{
    uint count;
    uint instanceCount;
    uint firstIndex;
    uint vertexOffset;
    uint firstInstance;
};

struct ViewConstants
{
    mat4 view_to_clip;
    mat4 clip_to_view;
    mat4 view_to_sample;
    mat4 sample_to_view;
    mat4 world_to_view;
    mat4 view_to_world;
    mat4 world_to_sample;
    mat4 clip_to_prev_clip;
    mat4 prev_view_to_prev_clip;
    mat4 prev_clip_to_prev_view;
    mat4 prev_world_to_prev_view;
    mat4 prev_view_to_prev_world;
    vec2 sample_offset_pixels;
    vec2 sample_offset_clip;
    vec2 prev_sample_offset_pixels;
    vec2 prev_sample_offset_clip;
};

struct Affine3Packed
{
    float data[12];
};

struct LilypadXform
{
    Affine3Packed xform;
};

uint _137;

layout(set = 0, binding = 13, std430) buffer type_RWStructuredBuffer_DrawIndexedIndirectCommand
{
    DrawIndexedIndirectCommand _m0[];
} _alias_draw_commands;

layout(set = 0, binding = 14, scalar) uniform type_Globals
{
    layout(offset = 0) int draw_indirect_buffer_offset;
    layout(offset = 4) uvec2 water_mask_dims;
    layout(offset = 16) uvec2 path_mask_dims;
    layout(offset = 24) uvec2 garden_mask_dims;
    layout(offset = 32) uvec2 animation_dims;
    layout(offset = 40) ivec2 grid_dims;
} _Globals;

layout(set = 1, binding = 0, std140) uniform type_ConstantBuffer_FrameConstants
{
    vec4 sun_color;
    vec4 sky_low_color;
    vec4 sky_high_color;
    vec4 fog_tint_thickness;
    vec4 fake_light_color;
    vec4 fake_light_dir;
    vec4 fake_light2_color;
    vec4 fake_light2_dir;
    uint frame_index;
    float delta_time_seconds;
    float ev_mult;
    uint glade_theme_id;
    uvec2 inner_rendering_resolution;
    uint game_time_sec;
    float game_time_frac;
    vec3 editing_highlight_color;
    float time_of_day;
    vec3 dir_to_sun;
    uint camera_idx;
    vec3 dir_to_moon;
    uint render_overrides;
    float global_mip_bias;
    uint highlighted_transient_stair_edge_id;
    uint pad1;
    uint pad2;
    vec4 debug_tweakers;
    layout(row_major) ViewConstants view_constants[5];
} frame_constants;

layout(set = 0, binding = 19, std430) buffer type_RWStructuredBuffer_LilypadXform
{
    LilypadXform _m0[];
} _alias_lilypad_buffer;

layout(set = 0, binding = 20, std430) buffer _alias_lilypad_flower_buffer
{
    LilypadXform _m0[];
} _alias_lilypad_flower_buffer_1;

layout(set = 0, binding = 4) uniform sampler sampler_llb;
layout(set = 0, binding = 9) uniform sampler sampler_default;
layout(set = 0, binding = 15) uniform texture2D water_mask;
layout(set = 0, binding = 16) uniform texture2D path_mask;
layout(set = 0, binding = 17) uniform texture2D garden_mask;
layout(set = 0, binding = 18, r16f) uniform image2D animation;

void main()
{
    do
    {
        if (any(greaterThanEqual(gl_GlobalInvocationID.xy, _Globals.animation_dims)))
        {
            break;
        }
        vec2 _209;
        uint _172 = gl_GlobalInvocationID.x + (gl_GlobalInvocationID.x << 10u);
        uint _174 = _172 ^ (_172 >> 6u);
        uint _176 = _174 + (_174 << 3u);
        uint _178 = _176 ^ (_176 >> 11u);
        uint _187 = gl_GlobalInvocationID.y + (gl_GlobalInvocationID.y << 10u);
        uint _189 = _187 ^ (_187 >> 6u);
        uint _191 = _189 + (_189 << 3u);
        uint _193 = _191 ^ (_191 >> 11u);
        vec2 _205 = ((((vec2(gl_GlobalInvocationID.xy) + vec2(0.5)) / vec2(_Globals.grid_dims)) - vec2(0.5)) * vec2(130.0)) + vec2(mix(-0.25, 0.25, ((uintBitsToFloat(((_178 + (_178 << 15u)) & 8388607u) | 1065353216u) - 1.0) + (uintBitsToFloat(((_193 + (_193 << 15u)) & 8388607u) | 1065353216u) - 1.0)) * 0.5) * 2.0);
        float _237;
        do
        {
            _209 = (_205 * vec2(0.0076923076994717121124267578125)) + vec2(0.5);
            bool _216;
            if (!(_209.x < 0.0))
            {
                _216 = _209.x > 1.0;
            }
            else
            {
                _216 = true;
            }
            bool _222;
            if (!_216)
            {
                _222 = _209.y < 0.0;
            }
            else
            {
                _222 = true;
            }
            bool _228;
            if (!_222)
            {
                _228 = _209.y > 1.0;
            }
            else
            {
                _228 = true;
            }
            if (_228)
            {
                _237 = 0.0;
                break;
            }
            else
            {
                _237 = textureLod(sampler2D(water_mask, sampler_default), _209, 0.0).y;
                break;
            }
            break; // unreachable workaround
        } while(false);
        float _249 = smoothstep(0.4000000059604644775390625, 0.5, smoothstep(0.0, 0.5, textureLod(sampler2D(garden_mask, sampler_default), _209, 0.0).y));
        imageStore(animation, ivec2(gl_GlobalInvocationID.xy), vec4(mix(imageLoad(animation, ivec2(gl_GlobalInvocationID.xy)).x, float(_249 > 0.0), 1.5 * frame_constants.delta_time_seconds)));
        bool _262 = frame_constants.glade_theme_id == 2u;
        float _445;
        if (_262)
        {
            vec2 _280 = _205 * 1.83000004291534423828125;
            ivec2 _282 = ivec2(floor(_280));
            vec2 _283 = fract(_280);
            uvec2 _284 = uvec2(_282);
            uint _286 = _284.y;
            uint _288 = _286 + (_286 << 10u);
            uint _290 = _288 ^ (_288 >> 6u);
            uint _292 = _290 + (_290 << 3u);
            uint _294 = _292 ^ (_292 >> 11u);
            uint _300 = (((_284.x * 1664525u) + (_294 + (_294 << 15u))) + 1013904223u) * 1664525u;
            uint _302 = _300 ^ (_300 >> 11u);
            uint _305 = _302 ^ ((_302 << 7u) & 2636928640u);
            uint _308 = _305 ^ ((_305 << 15u) & 4022730752u);
            uvec2 _316 = uvec2(_282 + ivec2(1, 0));
            uint _318 = _316.y;
            uint _320 = _318 + (_318 << 10u);
            uint _322 = _320 ^ (_320 >> 6u);
            uint _324 = _322 + (_322 << 3u);
            uint _326 = _324 ^ (_324 >> 11u);
            uint _332 = (((_316.x * 1664525u) + (_326 + (_326 << 15u))) + 1013904223u) * 1664525u;
            uint _334 = _332 ^ (_332 >> 11u);
            uint _337 = _334 ^ ((_334 << 7u) & 2636928640u);
            uint _340 = _337 ^ ((_337 << 15u) & 4022730752u);
            uvec2 _348 = uvec2(_282 + ivec2(0, 1));
            uint _350 = _348.y;
            uint _352 = _350 + (_350 << 10u);
            uint _354 = _352 ^ (_352 >> 6u);
            uint _356 = _354 + (_354 << 3u);
            uint _358 = _356 ^ (_356 >> 11u);
            uint _364 = (((_348.x * 1664525u) + (_358 + (_358 << 15u))) + 1013904223u) * 1664525u;
            uint _366 = _364 ^ (_364 >> 11u);
            uint _369 = _366 ^ ((_366 << 7u) & 2636928640u);
            uint _372 = _369 ^ ((_369 << 15u) & 4022730752u);
            uvec2 _380 = uvec2(_282 + ivec2(1));
            uint _382 = _380.y;
            uint _384 = _382 + (_382 << 10u);
            uint _386 = _384 ^ (_384 >> 6u);
            uint _388 = _386 + (_386 << 3u);
            uint _390 = _388 ^ (_388 >> 11u);
            uint _396 = (((_380.x * 1664525u) + (_390 + (_390 << 15u))) + 1013904223u) * 1664525u;
            uint _398 = _396 ^ (_396 >> 11u);
            uint _401 = _398 ^ ((_398 << 7u) & 2636928640u);
            uint _404 = _401 ^ ((_401 << 15u) & 4022730752u);
            vec2 _414 = (_283 * _283) * (vec2(3.0) - (_283 * 2.0));
            float _415 = _414.x;
            float _416 = 1.0 - _415;
            float _418 = _414.y;
            float _444;
            do
            {
                float _434 = abs((_237 + (((((1.0 - _418) * (((uintBitsToFloat(((_308 ^ (_308 >> 18u)) & 8388607u) | 1065353216u) - 1.0) * _416) + ((uintBitsToFloat(((_340 ^ (_340 >> 18u)) & 8388607u) | 1065353216u) - 1.0) * _415))) + (((uintBitsToFloat(((_372 ^ (_372 >> 18u)) & 8388607u) | 1065353216u) - 1.0) * _416) * _418)) + (((uintBitsToFloat(((_404 ^ (_404 >> 18u)) & 8388607u) | 1065353216u) - 1.0) * _415) * _418)) * 0.300000011920928955078125)) - 0.662000000476837158203125);
                if (_434 > 0.39300000667572021484375)
                {
                    _444 = 0.0;
                    break;
                }
                float _438 = _434 * 2.5445291996002197265625;
                _444 = 1.0 - ((_438 * _438) * (3.0 - (_434 * 5.089058399200439453125)));
                break;
            } while(false);
            _445 = _444;
        }
        else
        {
            float _279;
            do
            {
                float _269 = abs(_237 - 0.449999988079071044921875);
                if (_269 > 0.300000011920928955078125)
                {
                    _279 = 0.0;
                    break;
                }
                float _273 = _269 * 3.3333332538604736328125;
                _279 = 1.0 - ((_273 * _273) * (3.0 - (_269 * 6.666666507720947265625)));
                break;
            } while(false);
            _445 = _279;
        }
        uint _446 = _262 ? 1013904223u : _137;
        uint _447 = _262 ? 1664525u : _137;
        bool _448 = _262 ? true : false;
        float _462;
        do
        {
            float _452 = abs(textureLod(sampler2D(path_mask, sampler_llb), _209, 0.0).x - 0.20000000298023223876953125);
            if (_452 > 0.20000000298023223876953125)
            {
                _462 = 0.0;
                break;
            }
            float _456 = _452 * 5.0;
            _462 = 1.0 - ((_456 * _456) * (3.0 - (_452 * 10.0)));
            break;
        } while(false);
        float _465 = _445 + (_462 * smoothstep(0.60000002384185791015625, 0.800000011920928955078125, _237));
        float _467 = _249 * smoothstep(0.4000000059604644775390625, 0.5, _237);
        uint _632;
        uint _633;
        bool _634;
        bool _635;
        if (_465 > 0.0)
        {
            ivec2 _472 = ivec2(floor(_205));
            vec2 _473 = fract(_205);
            uvec2 _474 = uvec2(_472);
            uint _476 = _474.y;
            uint _478 = _476 + (_476 << 10u);
            uint _480 = _478 ^ (_478 >> 6u);
            uint _482 = _480 + (_480 << 3u);
            uint _484 = _482 ^ (_482 >> 11u);
            bool _487 = _448 ? _448 : true;
            uint _488 = _448 ? _447 : 1664525u;
            uint _489 = _448 ? _446 : 1013904223u;
            uint _493 = (((_474.x * _488) + (_484 + (_484 << 15u))) + _489) * _488;
            uint _495 = _493 ^ (_493 >> 11u);
            uint _498 = _495 ^ ((_495 << 7u) & 2636928640u);
            uint _501 = _498 ^ ((_498 << 15u) & 4022730752u);
            uvec2 _509 = uvec2(_472 + ivec2(1, 0));
            uint _511 = _509.y;
            uint _513 = _511 + (_511 << 10u);
            uint _515 = _513 ^ (_513 >> 6u);
            uint _517 = _515 + (_515 << 3u);
            uint _519 = _517 ^ (_517 >> 11u);
            bool _522 = _487 ? _487 : true;
            uint _523 = _487 ? _488 : 1664525u;
            uint _524 = _487 ? _489 : 1013904223u;
            uint _528 = (((_509.x * _523) + (_519 + (_519 << 15u))) + _524) * _523;
            uint _530 = _528 ^ (_528 >> 11u);
            uint _533 = _530 ^ ((_530 << 7u) & 2636928640u);
            uint _536 = _533 ^ ((_533 << 15u) & 4022730752u);
            uvec2 _544 = uvec2(_472 + ivec2(0, 1));
            uint _546 = _544.y;
            uint _548 = _546 + (_546 << 10u);
            uint _550 = _548 ^ (_548 >> 6u);
            uint _552 = _550 + (_550 << 3u);
            uint _554 = _552 ^ (_552 >> 11u);
            bool _557 = _522 ? _522 : true;
            uint _558 = _522 ? _523 : 1664525u;
            uint _559 = _522 ? _524 : 1013904223u;
            uint _563 = (((_544.x * _558) + (_554 + (_554 << 15u))) + _559) * _558;
            uint _565 = _563 ^ (_563 >> 11u);
            uint _568 = _565 ^ ((_565 << 7u) & 2636928640u);
            uint _571 = _568 ^ ((_568 << 15u) & 4022730752u);
            uvec2 _579 = uvec2(_472 + ivec2(1));
            uint _581 = _579.y;
            uint _583 = _581 + (_581 << 10u);
            uint _585 = _583 ^ (_583 >> 6u);
            uint _587 = _585 + (_585 << 3u);
            uint _589 = _587 ^ (_587 >> 11u);
            uint _593 = _557 ? _558 : 1664525u;
            uint _594 = _557 ? _559 : 1013904223u;
            uint _598 = (((_579.x * _593) + (_589 + (_589 << 15u))) + _594) * _593;
            uint _600 = _598 ^ (_598 >> 11u);
            uint _603 = _600 ^ ((_600 << 7u) & 2636928640u);
            uint _606 = _603 ^ ((_603 << 15u) & 4022730752u);
            vec2 _616 = (_473 * _473) * (vec2(3.0) - (_473 * 2.0));
            float _617 = _616.x;
            float _618 = 1.0 - _617;
            float _620 = _616.y;
            _632 = _594;
            _633 = _593;
            _634 = _557 ? _557 : true;
            _635 = ((((1.0 - _620) * (((uintBitsToFloat(((_501 ^ (_501 >> 18u)) & 8388607u) | 1065353216u) - 1.0) * _618) + ((uintBitsToFloat(((_536 ^ (_536 >> 18u)) & 8388607u) | 1065353216u) - 1.0) * _617))) + (((uintBitsToFloat(((_571 ^ (_571 >> 18u)) & 8388607u) | 1065353216u) - 1.0) * _618) * _620)) + (((uintBitsToFloat(((_606 ^ (_606 >> 18u)) & 8388607u) | 1065353216u) - 1.0) * _617) * _620)) > 0.5;
        }
        else
        {
            _632 = _446;
            _633 = _447;
            _634 = _448;
            _635 = false;
        }
        bool _804;
        if (!_635)
        {
            bool _803;
            if (_467 > 0.0)
            {
                vec2 _642 = _205 * 2.0;
                ivec2 _644 = ivec2(floor(_642));
                vec2 _645 = fract(_642);
                uvec2 _646 = uvec2(_644);
                uint _648 = _646.y;
                uint _650 = _648 + (_648 << 10u);
                uint _652 = _650 ^ (_650 >> 6u);
                uint _654 = _652 + (_652 << 3u);
                uint _656 = _654 ^ (_654 >> 11u);
                bool _659 = _634 ? _634 : true;
                uint _660 = _634 ? _633 : 1664525u;
                uint _661 = _634 ? _632 : 1013904223u;
                uint _665 = (((_646.x * _660) + (_656 + (_656 << 15u))) + _661) * _660;
                uint _667 = _665 ^ (_665 >> 11u);
                uint _670 = _667 ^ ((_667 << 7u) & 2636928640u);
                uint _673 = _670 ^ ((_670 << 15u) & 4022730752u);
                uvec2 _681 = uvec2(_644 + ivec2(1, 0));
                uint _683 = _681.y;
                uint _685 = _683 + (_683 << 10u);
                uint _687 = _685 ^ (_685 >> 6u);
                uint _689 = _687 + (_687 << 3u);
                uint _691 = _689 ^ (_689 >> 11u);
                bool _694 = _659 ? _659 : true;
                uint _695 = _659 ? _660 : 1664525u;
                uint _696 = _659 ? _661 : 1013904223u;
                uint _700 = (((_681.x * _695) + (_691 + (_691 << 15u))) + _696) * _695;
                uint _702 = _700 ^ (_700 >> 11u);
                uint _705 = _702 ^ ((_702 << 7u) & 2636928640u);
                uint _708 = _705 ^ ((_705 << 15u) & 4022730752u);
                uvec2 _716 = uvec2(_644 + ivec2(0, 1));
                uint _718 = _716.y;
                uint _720 = _718 + (_718 << 10u);
                uint _722 = _720 ^ (_720 >> 6u);
                uint _724 = _722 + (_722 << 3u);
                uint _726 = _724 ^ (_724 >> 11u);
                bool _729 = _694 ? _694 : true;
                uint _730 = _694 ? _695 : 1664525u;
                uint _731 = _694 ? _696 : 1013904223u;
                uint _735 = (((_716.x * _730) + (_726 + (_726 << 15u))) + _731) * _730;
                uint _737 = _735 ^ (_735 >> 11u);
                uint _740 = _737 ^ ((_737 << 7u) & 2636928640u);
                uint _743 = _740 ^ ((_740 << 15u) & 4022730752u);
                uvec2 _751 = uvec2(_644 + ivec2(1));
                uint _753 = _751.y;
                uint _755 = _753 + (_753 << 10u);
                uint _757 = _755 ^ (_755 >> 6u);
                uint _759 = _757 + (_757 << 3u);
                uint _761 = _759 ^ (_759 >> 11u);
                uint _764 = _729 ? _730 : 1664525u;
                uint _769 = (((_751.x * _764) + (_761 + (_761 << 15u))) + (_729 ? _731 : 1013904223u)) * _764;
                uint _771 = _769 ^ (_769 >> 11u);
                uint _774 = _771 ^ ((_771 << 7u) & 2636928640u);
                uint _777 = _774 ^ ((_774 << 15u) & 4022730752u);
                vec2 _787 = (_645 * _645) * (vec2(3.0) - (_645 * 2.0));
                float _788 = _787.x;
                float _789 = 1.0 - _788;
                float _791 = _787.y;
                _803 = ((((1.0 - _791) * (((uintBitsToFloat(((_673 ^ (_673 >> 18u)) & 8388607u) | 1065353216u) - 1.0) * _789) + ((uintBitsToFloat(((_708 ^ (_708 >> 18u)) & 8388607u) | 1065353216u) - 1.0) * _788))) + (((uintBitsToFloat(((_743 ^ (_743 >> 18u)) & 8388607u) | 1065353216u) - 1.0) * _789) * _791)) + (((uintBitsToFloat(((_777 ^ (_777 >> 18u)) & 8388607u) | 1065353216u) - 1.0) * _788) * _791)) > 0.5;
            }
            else
            {
                _803 = false;
            }
            _804 = _803;
        }
        else
        {
            _804 = true;
        }
        if (_804)
        {
            int _811 = int((gl_GlobalInvocationID.x * gl_GlobalInvocationID.y) + gl_GlobalInvocationID.x);
            vec3 _813 = _205.xxy;
            _813.y = 0.0;
            vec3 _819;
            if (frame_constants.glade_theme_id == 4u)
            {
                vec3 _818 = _813;
                _818.y = -0.100000001490116119384765625;
                _819 = _818;
            }
            else
            {
                _819 = _813;
            }
            uint _820 = gl_GlobalInvocationID.x + gl_GlobalInvocationID.y;
            uint _822 = _820 + (_820 << 10u);
            uint _824 = _822 ^ (_822 >> 6u);
            uint _826 = _824 + (_824 << 3u);
            uint _828 = _826 ^ (_826 >> 11u);
            float _834 = uintBitsToFloat(((_828 + (_828 << 15u)) & 8388607u) | 1065353216u) - 1.0;
            vec3 _840 = vec3(mix(0.800000011920928955078125, _262 ? 1.2999999523162841796875 : 2.0, _834) * mix(0.5, 1.0, clamp(_465 + _467, 0.0, 1.0))) * mix(1.0, 0.75, _467);
            uint _842 = uint(_811 ^ 234);
            uint _844 = _842 + (_842 << 10u);
            uint _846 = _844 ^ (_844 >> 6u);
            uint _848 = _846 + (_846 << 3u);
            uint _850 = _848 ^ (_848 >> 11u);
            uint _858 = uint(_811 ^ 8634);
            uint _860 = _858 + (_858 << 10u);
            uint _862 = _860 ^ (_860 >> 6u);
            uint _864 = _862 + (_862 << 3u);
            uint _866 = _864 ^ (_864 >> 11u);
            vec3 _874 = normalize(vec3(uintBitsToFloat(((_850 + (_850 << 15u)) & 8388607u) | 1065353216u) - 1.0, 0.0, uintBitsToFloat(((_866 + (_866 << 15u)) & 8388607u) | 1065353216u) - 1.0));
            mat4 _899 = transpose(mat4(vec4(_874, 0.0), vec4(0.0, 1.0, 0.0, 0.0), vec4(cross(_874, vec3(0.0, 1.0, 0.0)), 0.0), vec4(0.0, 0.0, 0.0, 1.0)));
            float _902 = 3.1400001049041748046875 * _834;
            vec3 _903 = normalize(vec3(0.0, 1.0, 0.0));
            float _904 = sin(_902);
            float _905 = cos(_902);
            float _906 = 1.0 - _905;
            float _908 = _906 * _903.x;
            float _912 = _908 * _903.y;
            float _914 = _903.z * _904;
            float _916 = _906 * _903.z;
            float _917 = _916 * _903.x;
            float _918 = _903.y * _904;
            float _921 = _906 * _903.y;
            float _924 = _921 * _903.z;
            float _925 = _903.x * _904;
            mat4 _936 = transpose(mat4(vec4((_908 * _903.x) + _905, _912 - _914, _917 + _918, 0.0), vec4(_912 + _914, (_921 * _903.y) + _905, _924 - _925, 0.0), vec4(_917 - _918, _924 + _925, (_916 * _903.z) + _905, 0.0), vec4(0.0, 0.0, 0.0, 1.0))) * ((mat4(vec4(_840.x, 0.0, 0.0, 0.0), vec4(0.0, _840.y, 0.0, 0.0), vec4(0.0, 0.0, _840.z, 0.0), vec4(0.0, 0.0, 0.0, 1.0)) * _899) * mat4(vec4(1.0, 0.0, 0.0, _819.x), vec4(0.0, 1.0, 0.0, _819.y), vec4(0.0, 0.0, 1.0, _819.z), vec4(0.0, 0.0, 0.0, 1.0)));
            uint _953 = atomicAdd(_alias_draw_commands._m0[uint(_Globals.draw_indirect_buffer_offset)].instanceCount, 1u);
            _alias_lilypad_buffer._m0[uint(int(_953))] = LilypadXform(Affine3Packed(float[](_936[0].x, _936[1].x, _936[2].x, _936[0].y, _936[1].y, _936[2].y, _936[0].z, _936[1].z, _936[2].z, _936[0].w, _936[1].w, _936[2].w)));
            uint _962 = _820 + 345u;
            uint _964 = _962 + (_962 << 10u);
            uint _966 = _964 ^ (_964 >> 6u);
            uint _968 = _966 + (_966 << 3u);
            uint _970 = _968 ^ (_968 >> 11u);
            if ((int(_467 > 0.699999988079071044921875) & int((uintBitsToFloat(((_970 + (_970 << 15u)) & 8388607u) | 1065353216u) - 1.0) > 0.20000000298023223876953125)) != 0)
            {
                uint _984 = atomicAdd(_alias_draw_commands._m0[15u].instanceCount, 1u);
                uint _987 = (10u * _820) + 45u;
                uint _989 = _987 + (_987 << 10u);
                uint _991 = _989 ^ (_989 >> 6u);
                uint _993 = _991 + (_991 << 3u);
                uint _995 = _993 ^ (_993 >> 11u);
                float _1009 = clamp(1.0 - smoothstep(((uintBitsToFloat(((_995 + (_995 << 15u)) & 8388607u) | 1065353216u) - 1.0) * 0.20000000298023223876953125) - 0.20000000298023223876953125, 1.0, imageLoad(animation, ivec2(gl_GlobalInvocationID.xy)).x), 0.0, 1.0);
                float _1016 = ((_1009 * _1009) * _1009) - ((_1009 * sin(_1009 * 3.1400001049041748046875)) * 1.5);
                vec3 _1020 = (_840 * (_467 * 0.800000011920928955078125)) * (1.0 - _1016);
                vec3 _1024;
                if (_262)
                {
                    _1024 = _1020 * 2.0;
                }
                else
                {
                    _1024 = _1020;
                }
                uint _1029 = (gl_GlobalInvocationID.x + (gl_GlobalInvocationID.y * 2u)) + 345u;
                uint _1031 = _1029 + (_1029 << 10u);
                uint _1033 = _1031 ^ (_1031 >> 6u);
                uint _1035 = _1033 + (_1033 << 3u);
                uint _1037 = _1035 ^ (_1035 >> 11u);
                uint _1048 = (gl_GlobalInvocationID.x + (gl_GlobalInvocationID.y * 3u)) - 345u;
                uint _1050 = _1048 + (_1048 << 10u);
                uint _1052 = _1050 ^ (_1050 >> 6u);
                uint _1054 = _1052 + (_1052 << 3u);
                uint _1056 = _1054 ^ (_1054 >> 11u);
                uint _1067 = (gl_GlobalInvocationID.x * 10u) + (gl_GlobalInvocationID.y * 345u);
                uint _1069 = _1067 + (_1067 << 10u);
                uint _1071 = _1069 ^ (_1069 >> 6u);
                uint _1073 = _1071 + (_1071 << 3u);
                uint _1075 = _1073 ^ (_1073 >> 11u);
                mat4 _1096 = (mat4(vec4(_1024.x, 0.0, 0.0, 0.0), vec4(0.0, _1024.y, 0.0, 0.0), vec4(0.0, 0.0, _1024.z, 0.0), vec4(0.0, 0.0, 0.0, 1.0)) * _899) * mat4(vec4(1.0, 0.0, 0.0, _819.x + mix(-0.300000011920928955078125, 0.300000011920928955078125, uintBitsToFloat(((_1037 + (_1037 << 15u)) & 8388607u) | 1065353216u) - 1.0)), vec4(0.0, 1.0, 0.0, (_819.y - (_1016 * 0.300000011920928955078125)) + ((uintBitsToFloat(((_1075 + (_1075 << 15u)) & 8388607u) | 1065353216u) - 1.0) * 0.100000001490116119384765625)), vec4(0.0, 0.0, 1.0, _819.z + mix(-0.300000011920928955078125, 0.300000011920928955078125, uintBitsToFloat(((_1056 + (_1056 << 15u)) & 8388607u) | 1065353216u) - 1.0)), vec4(0.0, 0.0, 0.0, 1.0));
                _alias_lilypad_flower_buffer_1._m0[uint(int(_984))] = LilypadXform(Affine3Packed(float[](_1096[0].x, _1096[1].x, _1096[2].x, _1096[0].y, _1096[1].y, _1096[2].y, _1096[0].z, _1096[1].z, _1096[2].z, _1096[0].w, _1096[1].w, _1096[2].w)));
            }
        }
        break;
    } while(false);
}

