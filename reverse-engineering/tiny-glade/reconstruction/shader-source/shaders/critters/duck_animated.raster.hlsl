// ==RECONSTRUCTED== shaders/critters/duck_animated.raster.hlsl | target: _critters_duck_animated.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main | evidence: spirv-cross + SPIR-V access/type inspection (per-vertex duck animation: wing flap, landing/take-off pose blend, pet blend, mouth-bob; gbuffer pack of vertex color) | bindings: $Globals b13, frame_constants set1/b0 (no textures) | status: R (reconstruction)

#include "inc/common.hlsl"
#include "inc/tracked_buffer.hlsl"
#include "inc/current_view.hlsl"
#include "inc/math_const.hlsl"
#include "inc/utils.hlsl"

// INC-issue workaround: original samplers.hlsl declared s0..s12 (13 samplers);
// inc/samplers.hlsl carries only s0..s4,s9..s12, so the missing s5..s8 are
// declared here to reproduce the original $Globals binding (set0/b13).

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
    float3 Vertex_Position : TEXCOORD0;
    float3 Vertex_Color : TEXCOORD1;
    float3 Vertex_Normal : TEXCOORD2;
    float3 rest_pos : TEXCOORD3;
    float3 rest_n : TEXCOORD4;
    int is_wing : TEXCOORD5;
    float wing_t : TEXCOORD6;
    int is_mouth_bottom : TEXCOORD7;
    float3 pet_pos : TEXCOORD8;
    float3 pet_n : TEXCOORD9;
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

    float3 _95 = vin.Vertex_Position;
    _95.y = _95.y + ((vin.wing_t * 0.5f) * cos((frac(anim_t) * 6.283185482025146484375f) - (vin.wing_t * 1.57079637050628662109375f)));

    float3 _172;
    float3 _173;
    if (vin.is_wing == 0)
    {
        float _141;
        if (is_landing != 0u)
        {
            float _130 = clamp(1.0f - landing_t, 0.0f, 1.0f);
            _141 = 1.0f - ((_130 * _130) - (_130 * sin(_130 * 3.1400001049041748046875f)));
        }
        else
        {
            _141 = (is_taking_off != 0u) ? 0.0f : landing_t;
        }
        float3 _142 = _141.xxx;
        _172 = lerp(vin.Vertex_Normal, vin.rest_n, _142);
        _173 = lerp(_95, vin.rest_pos, _142);
    }
    else
    {
        float _168;
        if (is_landing != 0u)
        {
            float _154 = clamp(smoothstep(0.0f, 0.800000011920928955078125f, landing_t), 0.0f, 1.0f);
            _168 = ((_154 * _154) * _154) - (_154 * sin(_154 * 3.1400001049041748046875f));
        }
        else
        {
            float _161 = clamp(landing_t, 0.0f, 1.0f);
            _168 = ((_161 * _161) * _161) - (_161 * sin(_161 * 3.1400001049041748046875f));
        }
        float3 _169 = _168.xxx;
        _172 = lerp(vin.Vertex_Normal, vin.rest_n, _169);
        _173 = lerp(_95, vin.rest_pos, _169);
    }

    float _199;
    if (pet_t_is_playing_forward == 1)
    {
        float _183 = clamp(1.0f - pet_t, 0.0f, 1.0f);
        _199 = 1.0f - ((_183 * _183) - (_183 * sin(_183 * 3.1400001049041748046875f)));
    }
    else
    {
        float _192 = clamp(pet_t, 0.0f, 1.0f);
        _199 = ((_192 * _192) * _192) - (_192 * sin(_192 * 3.1400001049041748046875f));
    }
    float3 _200 = _199.xxx;
    float3 _201 = lerp(_173, vin.pet_pos, _200);
    if (vin.is_mouth_bottom == 1)
    {
        float3 _210 = _201;
        _210.y = _201.y + (min(_199, 0.0f) * 0.100000001490116119384765625f);
        _201 = _210;
    }

    float4 _218 = float4(_201 * 0.60000002384185791015625f, 1.0f);
    float4 _219 = mul(model_to_world, _218);
    o.position = mul(view_constants[current_view_index].world_to_sample, _219);
    o.C0 = vin.Vertex_Color;
    o.C1 = mul(model_to_world, float4(lerp(_172, vin.pet_n, _200), 0.0f)).xyz;
    o.C2 = _219.xyz;
    o.C3 = mul(prev_model_to_world, _218).xyz;
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
    float3 packed_normal = (normalize(pin.C1) * 0.5f) + 0.5f;
    float3 _83 = sqrt(clamp(pin.C0, 0.0f, 1.0f));
    uint albedo_word = (uint((clamp(_83.x, 0.0f, 1.0f) * 255.0f) + 0.5f)
                      + (uint((clamp(_83.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)
                      + (uint((clamp(_83.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u));
    uint2 gbuf = uint2(((uint((clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f) + 0.5f)
                      + (uint((clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u))
                      + (uint((clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 3422552064u, albedo_word);
    gbuf.y = albedo_word | 117440512u;
    o.gbuffer = gbuf;
    o.velocity = pin.C2 - pin.C3;
    return o;
}
