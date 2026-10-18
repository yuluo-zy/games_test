#version 450
#extension GL_EXT_nonuniform_qualifier : require
#extension GL_EXT_samplerless_texture_functions : require
#extension GL_EXT_spirv_intrinsics : require
layout(local_size_x = 8, local_size_y = 8, local_size_z = 1) in;

struct DofTileOffsets
{
    uint offsets[8];
};

layout(set = 0, binding = 16, std140) uniform type_spatial_resolve_offsets
{
    ivec4 spatial_resolve_offsets_array[512];
} spatial_resolve_offsets;

layout(set = 0, binding = 18, std430) readonly buffer type_StructuredBuffer_DofTileOffsets
{
    DofTileOffsets _m0[];
} tile_offsets_buffer;

layout(set = 0, binding = 19, std430) readonly buffer type_StructuredBuffer_v2uint
{
    uvec2 _m0[];
} tile_coords_buffer;

layout(set = 0, binding = 15) uniform texture2D halfres_tex;
layout(set = 0, binding = 17, rgba16f) uniform writeonly image2D img_output;

spirv_instruction(set = "GLSL.std.450", id = 80) float spvNMax(float, float);
spirv_instruction(set = "GLSL.std.450", id = 80) vec2 spvNMax(vec2, vec2);
spirv_instruction(set = "GLSL.std.450", id = 80) vec3 spvNMax(vec3, vec3);
spirv_instruction(set = "GLSL.std.450", id = 80) vec4 spvNMax(vec4, vec4);

void main()
{
    uvec2 _85 = (gl_GlobalInvocationID.xy & uvec2(7u)) + (uvec2(8u) * tile_coords_buffer._m0[tile_offsets_buffer._m0[0u].offsets[3] + gl_WorkGroupID.x]);
    uvec2 _86 = _85 & uvec2(1u);
    uint _90 = _86.x + (_86.y * 2u);
    vec4 _92;
    _92 = vec4(0.0);
    for (uint _95 = 0u; _95 < 8u; )
    {
        _92 += texelFetch(halfres_tex, ivec2((_85 / uvec2(2u)) + uvec2(spatial_resolve_offsets.spatial_resolve_offsets_array[(_90 * 16u) + _95].xy)), int(0u));
        _95++;
        continue;
    }
    vec4 _109 = _92 * vec4(0.125);
    imageStore(img_output, ivec2(_85), _109 / vec4(spvNMax(1.0000000133514319600180897396058e-10, _109.w)));
}

