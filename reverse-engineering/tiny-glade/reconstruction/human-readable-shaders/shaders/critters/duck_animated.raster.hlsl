// 鸭子动画着色器(顶点动画 + G-Buffer 打包)。
// 逐顶点鸭子动画:翅膀扇动、降落/起飞姿态混合、抚摸姿态混合、嘴部上下摆动;
// 像素阶段把顶点色反照率与打包法线写入 G-Buffer。
// 绑定:$Globals b13,frame_constants set1/b0(无纹理)。入口 vs_main / ps_main。
// 行为取自 _critters_duck_animated bins(spirv-cross 转译,语义化重命名,指令不变)。

#include "inc/common.hlsl"
#include "inc/tracked_buffer.hlsl"
#include "inc/current_view.hlsl"
#include "inc/math_const.hlsl"
#include "inc/utils.hlsl"

// INC 问题规避:原版 samplers.hlsl 声明 s0..s12(13 个采样器);
// inc/samplers.hlsl 只带 s0..s4,s9..s12,因此缺失的 s5..s8 在此声明,
// 以还原原版 $Globals 绑定(set0/b13)。

float4x4 model_to_world;
float4x4 prev_model_to_world;
float anim_t;
float landing_t;
uint is_landing;
uint is_taking_off;
float pet_t;
int pet_t_is_playing_forward;
int bool_render_shadows;

struct VsIn
{
    [[vk::location(0)]] float3 Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 Vertex_Color: Vertex_Color;
    [[vk::location(2)]] float3 Vertex_Normal: Vertex_Normal;
    [[vk::location(3)]] float3 rest_pos: rest_pos;
    [[vk::location(4)]] float3 rest_n: rest_n;
    [[vk::location(5)]] int is_wing: is_wing;
    [[vk::location(6)]] float wing_t: wing_t;
    [[vk::location(7)]] int is_mouth_bottom: is_mouth_bottom;
    [[vk::location(8)]] float3 pet_pos: pet_pos;
    [[vk::location(9)]] float3 pet_n: pet_n;
};

struct VsOut
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float3 C3 : TEXCOORD3;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;

    // 翅膀扇动:顶点 y 叠加 wing_t 驱动的正弦摆动。
    float3 pos = vin.Vertex_Position;
    pos.y = pos.y + ((vin.wing_t * 0.5f) * cos((frac(anim_t) * 6.283185482025146484375f) - (vin.wing_t * 1.57079637050628662109375f)));

    // 降落/起飞姿态混合:非翅膀顶点与翅膀顶点用不同的相位曲线。
    float3 pose_normal;
    float3 pose_pos;
    if (vin.is_wing == 0)
    {
        float land_pose;
        if (is_landing != 0u)
        {
            // 降落:ease-out 曲线从 1 落到 0。
            float takeoff_phase = clamp(1.0f - landing_t, 0.0f, 1.0f);
            land_pose = 1.0f - ((takeoff_phase * takeoff_phase) - (takeoff_phase * sin(takeoff_phase * 3.1400001049041748046875f)));
        }
        else
        {
            land_pose = (is_taking_off != 0u) ? 0.0f : landing_t;
        }
        float3 land_pose_v = land_pose.xxx;
        pose_normal = lerp(vin.Vertex_Normal, vin.rest_n, land_pose_v);
        pose_pos = lerp(pos, vin.rest_pos, land_pose_v);
    }
    else
    {
        float wing_pose;
        if (is_landing != 0u)
        {
            // 翅膀在降落时用更陡的相位曲线。
            float wing_land_phase = clamp(smoothstep(0.0f, 0.800000011920928955078125f, landing_t), 0.0f, 1.0f);
            wing_pose = ((wing_land_phase * wing_land_phase) * wing_land_phase) - (wing_land_phase * sin(wing_land_phase * 3.1400001049041748046875f));
        }
        else
        {
            float wing_phase = clamp(landing_t, 0.0f, 1.0f);
            wing_pose = ((wing_phase * wing_phase) * wing_phase) - (wing_phase * sin(wing_phase * 3.1400001049041748046875f));
        }
        float3 wing_pose_v = wing_pose.xxx;
        pose_normal = lerp(vin.Vertex_Normal, vin.rest_n, wing_pose_v);
        pose_pos = lerp(pos, vin.rest_pos, wing_pose_v);
    }

    // 抚摸姿态混合:正放/倒放使用不同相位。
    float pet_pose;
    if (pet_t_is_playing_forward == 1)
    {
        float pet_reverse = clamp(1.0f - pet_t, 0.0f, 1.0f);
        pet_pose = 1.0f - ((pet_reverse * pet_reverse) - (pet_reverse * sin(pet_reverse * 3.1400001049041748046875f)));
    }
    else
    {
        float pet_phase = clamp(pet_t, 0.0f, 1.0f);
        pet_pose = ((pet_phase * pet_phase) * pet_phase) - (pet_phase * sin(pet_phase * 3.1400001049041748046875f));
    }
    float3 pet_pose_v = pet_pose.xxx;
    float3 animated_pos = lerp(pose_pos, vin.pet_pos, pet_pose_v);
    // 嘴部下摆:嘴部底端顶点再向下偏移。
    if (vin.is_mouth_bottom == 1)
    {
        float3 mouth_pos = animated_pos;
        mouth_pos.y = animated_pos.y + (min(pet_pose, 0.0f) * 0.100000001490116119384765625f);
        animated_pos = mouth_pos;
    }

    float4 scaled_pos = float4(animated_pos * 0.60000002384185791015625f, 1.0f);
    float4 model_pos = mul(model_to_world, scaled_pos);
    o.position = mul(view_constants[current_view_index].world_to_sample, model_pos);
    o.C0 = vin.Vertex_Color;
    o.C1 = mul(model_to_world, float4(lerp(pose_normal, vin.pet_n, pet_pose_v), 0.0f)).xyz;
    o.C2 = model_pos.xyz;
    o.C3 = mul(prev_model_to_world, scaled_pos).xyz;
    return o;
}

struct PsIn
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float3 C3 : TEXCOORD3;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
    float3 velocity : SV_Target1;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;
    // 法线打包到 [0,1] 空间。
    float3 packed_normal = (normalize(pin.C1) * 0.5f) + 0.5f;
    // 顶点色开方(线性化反照率)后打包为 RGB8。
    float3 albedo_sqrt = sqrt(clamp(pin.C0, 0.0f, 1.0f));
    uint albedo_word = (uint((clamp(albedo_sqrt.x, 0.0f, 1.0f) * 255.0f) + 0.5f)
                      + (uint((clamp(albedo_sqrt.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)
                      + (uint((clamp(albedo_sqrt.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u));
    uint2 gbuf = uint2(((uint((clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f) + 0.5f)
                      + (uint((clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u))
                      + (uint((clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 3422552064u, albedo_word);
    gbuf.y = albedo_word | 117440512u;
    o.gbuffer = gbuf;
    o.velocity = pin.C2 - pin.C3;
    return o;
}
