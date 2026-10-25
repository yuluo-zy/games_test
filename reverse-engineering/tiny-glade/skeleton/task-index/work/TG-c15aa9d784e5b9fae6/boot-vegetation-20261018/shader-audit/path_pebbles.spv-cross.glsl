// DERIVED by SPIRV-Cross from original packaged SPIR-V; NOT original HLSL/OpSource.
#version 460
#extension GL_EXT_nonuniform_qualifier : require
#extension GL_EXT_scalar_block_layout : require
layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

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

vec3 _103;

layout(set = 0, binding = 13, std430) buffer type_RWStructuredBuffer_DrawIndexedIndirectCommand
{
    DrawIndexedIndirectCommand _m0[];
} _alias_draw_commands;

layout(set = 0, binding = 14, scalar) uniform type_Globals
{
    layout(offset = 0) int draw_indirect_buffer_offset;
    layout(offset = 4) uvec2 terrain_heightmap_dims;
    layout(offset = 16) uvec2 visual_terrain_heightmap_dims;
    layout(offset = 24) uvec2 path_mask_dims;
    layout(offset = 32) uvec2 rocky_terrain_dims;
    layout(offset = 40) uvec2 water_mask_dims;
    layout(offset = 48) uint pebble_buffer_size;
    layout(offset = 52) int scatter_grid_dim;
} _Globals;

layout(set = 0, binding = 20, std430) buffer type_RWStructuredBuffer_Affine3Packed
{
    Affine3Packed _m0[];
} _alias_pebble_placement_buffer;

layout(set = 0, binding = 4) uniform sampler sampler_llb;
layout(set = 0, binding = 9) uniform sampler sampler_default;
layout(set = 0, binding = 15) uniform texture2D terrain_heightmap;
layout(set = 0, binding = 17) uniform texture2D path_mask;
layout(set = 0, binding = 18) uniform texture2D rocky_terrain;
layout(set = 0, binding = 19) uniform texture2D water_mask;

void main()
{
    do
    {
        int _112 = int(gl_GlobalInvocationID.x);
        int _113 = _112 - (_Globals.scatter_grid_dim * _Globals.scatter_grid_dim) * (_112 / (_Globals.scatter_grid_dim * _Globals.scatter_grid_dim));
        float _117 = float(_Globals.scatter_grid_dim);
        vec2 _123 = (vec2(float(_113 - _Globals.scatter_grid_dim * (_113 / _Globals.scatter_grid_dim)) / _117, float(_113 / _Globals.scatter_grid_dim) / _117) - vec2(0.5)) * vec2(104.0);
        uint _126 = uint(_112);
        uint _128 = _126 + (_126 << 10u);
        uint _130 = _128 ^ (_128 >> 6u);
        uint _132 = _130 + (_130 << 3u);
        uint _134 = _132 ^ (_132 >> 11u);
        uint _141 = _126 ^ 188053769u;
        uint _143 = _141 + (_141 << 10u);
        uint _145 = _143 ^ (_143 >> 6u);
        uint _147 = _145 + (_145 << 3u);
        uint _149 = _147 ^ (_147 >> 11u);
        vec2 _157 = vec2(uintBitsToFloat(((_134 + (_134 << 15u)) & 8388607u) | 1065353216u) - 1.0, uintBitsToFloat(((_149 + (_149 << 15u)) & 8388607u) | 1065353216u) - 1.0) * 6.30000019073486328125;
        float _159 = _123.x + _157.x;
        vec3 _160;
        _160.x = _159;
        float _162 = _123.y + _157.y;
        _160.z = _162;
        float _172 = mix(-2.5, 24.0, textureLod(sampler2D(terrain_heightmap, sampler_default), (_160.xz * vec2(0.0076923076994717121124267578125)) + vec2(0.5), 0.0).x);
        vec2 _178 = (vec3(_159, _172, _162).xz * vec2(0.0076923076994717121124267578125)) + vec2(0.5);
        if (textureLod(sampler2D(path_mask, sampler_llb), _178, 0.0).x < 0.89999997615814208984375)
        {
            break;
        }
        float _187;
        bool _189;
        float _213;
        do
        {
            _187 = _178.x;
            _189 = !(_187 < 0.0);
            bool _193;
            if (_189)
            {
                _193 = _187 > 1.0;
            }
            else
            {
                _193 = true;
            }
            bool _199;
            if (!_193)
            {
                _199 = _178.y < 0.0;
            }
            else
            {
                _199 = true;
            }
            bool _205;
            if (!_199)
            {
                _205 = _178.y > 1.0;
            }
            else
            {
                _205 = true;
            }
            if (_205)
            {
                _213 = 0.0;
                break;
            }
            else
            {
                _213 = textureLod(sampler2D(rocky_terrain, sampler_default), _178, 0.0).x;
                break;
            }
            break; // unreachable workaround
        } while(false);
        float _239;
        do
        {
            bool _219;
            if (_189)
            {
                _219 = _187 > 1.0;
            }
            else
            {
                _219 = true;
            }
            bool _225;
            if (!_219)
            {
                _225 = _178.y < 0.0;
            }
            else
            {
                _225 = true;
            }
            bool _231;
            if (!_225)
            {
                _231 = _178.y > 1.0;
            }
            else
            {
                _231 = true;
            }
            if (_231)
            {
                _239 = 0.0;
                break;
            }
            else
            {
                _239 = textureLod(sampler2D(water_mask, sampler_default), _178, 0.0).y;
                break;
            }
            break; // unreachable workaround
        } while(false);
        bool _245;
        if (!(_213 > 0.0))
        {
            _245 = _239 > 0.5;
        }
        else
        {
            _245 = true;
        }
        if (_245)
        {
            break;
        }
        int _252 = (int(_159) ^ (345 + int(_162))) ^ 2378;
        uint _253 = uint(_252);
        uint _255 = _253 + (_253 << 10u);
        uint _257 = _255 ^ (_255 >> 6u);
        uint _259 = _257 + (_257 << 3u);
        uint _261 = _259 ^ (_259 >> 11u);
        float _266 = uintBitsToFloat(((_261 + (_261 << 15u)) & 8388607u) | 1065353216u);
        float _269 = mix(0.4000000059604644775390625, 1.2000000476837158203125, _266 - 1.0) * 1.2000000476837158203125;
        float _271 = (1.0 - _266) * 6.280000209808349609375;
        vec2 _275 = normalize(vec2(cos(_271), sin(_271)));
        float _276 = _275.x;
        float _277 = _275.y;
        uint _280 = uint(_252 - 235);
        uint _282 = _280 + (_280 << 10u);
        uint _284 = _282 ^ (_282 >> 6u);
        uint _286 = _284 + (_284 << 3u);
        uint _288 = _286 ^ (_286 >> 11u);
        float _295 = (1.0 - uintBitsToFloat(((_288 + (_288 << 15u)) & 8388607u) | 1065353216u)) * 6.280000209808349609375;
        vec2 _299 = normalize(vec2(cos(_295), sin(_295)));
        uint _301 = uint(_252 + 2345);
        uint _303 = _301 + (_301 << 10u);
        uint _305 = _303 ^ (_303 >> 6u);
        uint _307 = _305 + (_305 << 3u);
        uint _309 = _307 ^ (_307 >> 11u);
        float _316 = (uintBitsToFloat(((_309 + (_309 << 15u)) & 8388607u) | 1065353216u) - 1.0) * 0.5;
        vec3 _322 = normalize(vec3(_299.x * _316, 1.0, _299.y * _316));
        uint _328 = atomicAdd(_alias_draw_commands._m0[uint(_Globals.draw_indirect_buffer_offset)].instanceCount, 1u);
        mat4 _351 = (mat4(vec4(_269, 0.0, 0.0, 0.0), vec4(0.0, _269, 0.0, 0.0), vec4(0.0, 0.0, _269, 0.0), vec4(0.0, 0.0, 0.0, 1.0)) * transpose(mat4(vec4(cross(vec3(_276, 0.0, _277), _322), 0.0), vec4(_322, 0.0), vec4(_276, 0.0, _277, 0.0), vec4(0.0, 0.0, 0.0, 1.0)))) * mat4(vec4(1.0, 0.0, 0.0, _159), vec4(0.0, 1.0, 0.0, _172), vec4(0.0, 0.0, 1.0, _162), vec4(0.0, 0.0, 0.0, 1.0));
        _alias_pebble_placement_buffer._m0[uint(int(_328))] = Affine3Packed(float[](_351[0].x, _351[1].x, _351[2].x, _351[0].y, _351[1].y, _351[2].y, _351[0].z, _351[1].z, _351[2].z, _351[0].w, _351[1].w, _351[2].w));
        break;
    } while(false);
}

