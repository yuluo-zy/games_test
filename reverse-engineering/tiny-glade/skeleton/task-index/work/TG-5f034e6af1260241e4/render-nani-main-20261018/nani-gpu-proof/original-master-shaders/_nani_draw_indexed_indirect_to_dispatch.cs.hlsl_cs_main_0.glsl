#version 450
layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

struct DrawIndexedIndirectCommand
{
    uint count;
    uint instanceCount;
    uint firstIndex;
    uint vertexOffset;
    uint firstInstance;
};

layout(set = 0, binding = 13, std430) readonly buffer type_StructuredBuffer_DrawIndexedIndirectCommand
{
    DrawIndexedIndirectCommand _m0[];
} draw_buffer;

layout(set = 0, binding = 14, std430) buffer type_RWStructuredBuffer_v4uint
{
    uvec4 _m0[];
} args_buffer;

layout(push_constant, std430) uniform type_ConstantBuffer_PushConstants
{
    uint bucket;
    uint group_size;
} push_constants;

void main()
{
    args_buffer._m0[0u] = uvec4(((draw_buffer._m0[push_constants.bucket].instanceCount + push_constants.group_size) - 1u) / push_constants.group_size, 1u, 1u, draw_buffer._m0[push_constants.bucket].instanceCount);
}

