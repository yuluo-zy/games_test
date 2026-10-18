// DERIVED by SPIRV-Cross from original packaged SPIR-V; NOT original HLSL/OpSource.
#version 460
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

struct Affine3Packed
{
    float data[12];
};

struct Reed
{
    Affine3Packed xform;
};

layout(set = 0, binding = 13, std430) buffer type_RWStructuredBuffer_DrawIndexedIndirectCommand
{
    DrawIndexedIndirectCommand _m0[];
} _alias_draw_commands;

layout(set = 0, binding = 14, scalar) uniform type_Globals
{
    layout(offset = 0) int draw_indirect_buffer_offset;
    layout(offset = 4) uvec2 water_mask_dims;
    layout(offset = 16) uvec2 path_mask_dims;
    layout(offset = 24) uvec2 buildings_mask_non_blurred_dims;
    layout(offset = 32) ivec2 grid_dims;
} _Globals;

layout(set = 0, binding = 18, std430) buffer type_RWStructuredBuffer_Reed
{
    Reed _m0[];
} _alias_instance_buffer;

layout(set = 0, binding = 4) uniform sampler sampler_llb;
layout(set = 0, binding = 9) uniform sampler sampler_default;
layout(set = 0, binding = 15) uniform texture2D water_mask;
layout(set = 0, binding = 16) uniform texture2D path_mask;
layout(set = 0, binding = 17) uniform texture2D buildings_mask_non_blurred;

void main()
{
    vec2 _158;
    uint _137 = (gl_GlobalInvocationID.x * gl_GlobalInvocationID.y) + gl_GlobalInvocationID.x;
    uint _139 = _137 + (_137 << 10u);
    uint _141 = _139 ^ (_139 >> 6u);
    uint _143 = _141 + (_141 << 3u);
    uint _145 = _143 ^ (_143 >> 11u);
    float _151 = uintBitsToFloat(((_145 + (_145 << 15u)) & 8388607u) | 1065353216u) - 1.0;
    vec2 _154 = ((((vec2(gl_GlobalInvocationID.xy) + vec2(0.5)) / vec2(_Globals.grid_dims)) - vec2(0.5)) * vec2(130.0)) + vec2(mix(-0.4000000059604644775390625, 0.4000000059604644775390625, _151));
    float _186;
    do
    {
        _158 = (_154 * vec2(0.0076923076994717121124267578125)) + vec2(0.5);
        bool _165;
        if (!(_158.x < 0.0))
        {
            _165 = _158.x > 1.0;
        }
        else
        {
            _165 = true;
        }
        bool _171;
        if (!_165)
        {
            _171 = _158.y < 0.0;
        }
        else
        {
            _171 = true;
        }
        bool _177;
        if (!_171)
        {
            _177 = _158.y > 1.0;
        }
        else
        {
            _177 = true;
        }
        if (_177)
        {
            _186 = 0.0;
            break;
        }
        else
        {
            _186 = textureLod(sampler2D(water_mask, sampler_default), _158, 0.0).y;
            break;
        }
        break; // unreachable workaround
    } while(false);
    vec4 _190 = textureLod(sampler2D(path_mask, sampler_llb), _158, 0.0);
    float _191 = _190.x;
    float _205;
    do
    {
        float _195 = abs(_186 - 0.5);
        if (_195 > 0.300000011920928955078125)
        {
            _205 = 0.0;
            break;
        }
        float _199 = _195 * 3.3333332538604736328125;
        _205 = 1.0 - ((_199 * _199) * (3.0 - (_195 * 6.666666507720947265625)));
        break;
    } while(false);
    float _219;
    do
    {
        float _209 = abs(_191 - 0.20000000298023223876953125);
        if (_209 > 0.20000000298023223876953125)
        {
            _219 = 0.0;
            break;
        }
        float _213 = _209 * 5.0;
        _219 = 1.0 - ((_213 * _213) * (3.0 - (_209 * 10.0)));
        break;
    } while(false);
    float _224 = (_205 + (_219 * smoothstep(0.60000002384185791015625, 0.800000011920928955078125, _186))) - smoothstep(0.20000000298023223876953125, 0.25, _191);
    bool _391;
    if (_224 > 0.0)
    {
        vec2 _233 = _154 + vec2(20.0);
        ivec2 _235 = ivec2(floor(_233));
        vec2 _236 = fract(_233);
        uvec2 _237 = uvec2(_235);
        uint _239 = _237.y;
        uint _241 = _239 + (_239 << 10u);
        uint _243 = _241 ^ (_241 >> 6u);
        uint _245 = _243 + (_243 << 3u);
        uint _247 = _245 ^ (_245 >> 11u);
        uint _253 = (((_237.x * 1664525u) + (_247 + (_247 << 15u))) + 1013904223u) * 1664525u;
        uint _255 = _253 ^ (_253 >> 11u);
        uint _258 = _255 ^ ((_255 << 7u) & 2636928640u);
        uint _261 = _258 ^ ((_258 << 15u) & 4022730752u);
        uvec2 _269 = uvec2(_235 + ivec2(1, 0));
        uint _271 = _269.y;
        uint _273 = _271 + (_271 << 10u);
        uint _275 = _273 ^ (_273 >> 6u);
        uint _277 = _275 + (_275 << 3u);
        uint _279 = _277 ^ (_277 >> 11u);
        uint _285 = (((_269.x * 1664525u) + (_279 + (_279 << 15u))) + 1013904223u) * 1664525u;
        uint _287 = _285 ^ (_285 >> 11u);
        uint _290 = _287 ^ ((_287 << 7u) & 2636928640u);
        uint _293 = _290 ^ ((_290 << 15u) & 4022730752u);
        uvec2 _301 = uvec2(_235 + ivec2(0, 1));
        uint _303 = _301.y;
        uint _305 = _303 + (_303 << 10u);
        uint _307 = _305 ^ (_305 >> 6u);
        uint _309 = _307 + (_307 << 3u);
        uint _311 = _309 ^ (_309 >> 11u);
        uint _317 = (((_301.x * 1664525u) + (_311 + (_311 << 15u))) + 1013904223u) * 1664525u;
        uint _319 = _317 ^ (_317 >> 11u);
        uint _322 = _319 ^ ((_319 << 7u) & 2636928640u);
        uint _325 = _322 ^ ((_322 << 15u) & 4022730752u);
        uvec2 _333 = uvec2(_235 + ivec2(1));
        uint _335 = _333.y;
        uint _337 = _335 + (_335 << 10u);
        uint _339 = _337 ^ (_337 >> 6u);
        uint _341 = _339 + (_339 << 3u);
        uint _343 = _341 ^ (_341 >> 11u);
        uint _349 = (((_333.x * 1664525u) + (_343 + (_343 << 15u))) + 1013904223u) * 1664525u;
        uint _351 = _349 ^ (_349 >> 11u);
        uint _354 = _351 ^ ((_351 << 7u) & 2636928640u);
        uint _357 = _354 ^ ((_354 << 15u) & 4022730752u);
        vec2 _367 = (_236 * _236) * (vec2(3.0) - (_236 * 2.0));
        float _368 = _367.x;
        float _369 = 1.0 - _368;
        float _371 = _367.y;
        _391 = ((int(((((1.0 - _371) * (((uintBitsToFloat(((_261 ^ (_261 >> 18u)) & 8388607u) | 1065353216u) - 1.0) * _369) + ((uintBitsToFloat(((_293 ^ (_293 >> 18u)) & 8388607u) | 1065353216u) - 1.0) * _368))) + (((uintBitsToFloat(((_325 ^ (_325 >> 18u)) & 8388607u) | 1065353216u) - 1.0) * _369) * _371)) + (((uintBitsToFloat(((_357 ^ (_357 >> 18u)) & 8388607u) | 1065353216u) - 1.0) * _368) * _371)) > 0.60000002384185791015625) & int(_186 > 0.550000011920928955078125)) & int(textureLod(sampler2D(buildings_mask_non_blurred, sampler_default), _158, 0.0).x < 0.100000001490116119384765625)) != 0;
    }
    else
    {
        _391 = false;
    }
    if (_391)
    {
        uint _394 = gl_GlobalInvocationID.x + gl_GlobalInvocationID.y;
        uint _396 = _394 + (_394 << 10u);
        uint _398 = _396 ^ (_396 >> 6u);
        uint _400 = _398 + (_398 << 3u);
        uint _402 = _400 ^ (_400 >> 11u);
        float _409 = mix(1.0, 1.5, uintBitsToFloat(((_402 + (_402 << 15u)) & 8388607u) | 1065353216u) - 1.0);
        float _424 = 3.1400001049041748046875 * _151;
        vec3 _425 = normalize(vec3(0.0, 1.0, 0.0));
        float _426 = sin(_424);
        float _427 = cos(_424);
        float _428 = 1.0 - _427;
        float _430 = _428 * _425.x;
        float _434 = _430 * _425.y;
        float _436 = _425.z * _426;
        float _438 = _428 * _425.z;
        float _439 = _438 * _425.x;
        float _440 = _425.y * _426;
        float _443 = _428 * _425.y;
        float _446 = _443 * _425.z;
        float _447 = _425.x * _426;
        mat4 _458 = transpose(mat4(vec4((_430 * _425.x) + _427, _434 - _436, _439 + _440, 0.0), vec4(_434 + _436, (_443 * _425.y) + _427, _446 - _447, 0.0), vec4(_439 - _440, _446 + _447, (_438 * _425.z) + _427, 0.0), vec4(0.0, 0.0, 0.0, 1.0))) * ((mat4(vec4(_409, 0.0, 0.0, 0.0), vec4(0.0, _409 * mix(0.5, 1.0, _224 * _224), 0.0, 0.0), vec4(0.0, 0.0, _409, 0.0), vec4(0.0, 0.0, 0.0, 1.0)) * mat4(vec4(1.0, 0.0, 0.0, 0.0), vec4(0.0, 1.0, 0.0, 0.0), vec4(0.0, 0.0, 1.0, 0.0), vec4(0.0, 0.0, 0.0, 1.0))) * mat4(vec4(1.0, 0.0, 0.0, _154.x), vec4(0.0, 1.0, 0.0, -1.0), vec4(0.0, 0.0, 1.0, _154.y), vec4(0.0, 0.0, 0.0, 1.0)));
        uint _475 = atomicAdd(_alias_draw_commands._m0[uint(_Globals.draw_indirect_buffer_offset)].instanceCount, 1u);
        _alias_instance_buffer._m0[uint(int(_475))] = Reed(Affine3Packed(float[](_458[0].x, _458[1].x, _458[2].x, _458[0].y, _458[1].y, _458[2].y, _458[0].z, _458[1].z, _458[2].z, _458[0].w, _458[1].w, _458[2].w)));
    }
}

