// Derived from original SPV, not original HLSL.
#version 460
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

struct BushXform
{
    Affine3Packed transform;
};

layout(set = 0, binding = 13, std430) readonly buffer type_StructuredBuffer_DrawIndexedIndirectCommand
{
    DrawIndexedIndirectCommand _m0[];
} draw_commands;

layout(set = 0, binding = 14, std140) uniform type_Globals
{
    int draw_indirect_buffer_offset;
} _Globals;

layout(set = 0, binding = 15, std430) readonly buffer type_StructuredBuffer_BushXform
{
    BushXform _m0[];
} bush_buffer;

layout(set = 0, binding = 16, std430) buffer type_RWStructuredBuffer_v2float
{
    vec2 _m0[];
} _alias_bush_cpu;

void main()
{
    uint _67 = draw_commands._m0[uint(_Globals.draw_indirect_buffer_offset)].instanceCount;
    int _68 = int(_67);
    if (gl_GlobalInvocationID.x == 0u)
    {
        _alias_bush_cpu._m0[gl_GlobalInvocationID.x] = vec2(intBitsToFloat(_68));
    }
    else
    {
        if (gl_GlobalInvocationID.x < uint(_68 + 1))
        {
            uint _81 = gl_GlobalInvocationID.x - 1u;
            _alias_bush_cpu._m0[gl_GlobalInvocationID.x] = (vec4(0.0, 0.0, 0.0, 1.0) * transpose(mat4x3(vec3(bush_buffer._m0[_81].transform.data[0], bush_buffer._m0[_81].transform.data[1], bush_buffer._m0[_81].transform.data[2]), vec3(bush_buffer._m0[_81].transform.data[3], bush_buffer._m0[_81].transform.data[4], bush_buffer._m0[_81].transform.data[5]), vec3(bush_buffer._m0[_81].transform.data[6], bush_buffer._m0[_81].transform.data[7], bush_buffer._m0[_81].transform.data[8]), vec3(bush_buffer._m0[_81].transform.data[9], bush_buffer._m0[_81].transform.data[10], bush_buffer._m0[_81].transform.data[11])))).xz;
        }
    }
}

