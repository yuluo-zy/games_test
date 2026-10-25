#version 450
#extension GL_EXT_nonuniform_qualifier : require
#extension GL_EXT_scalar_block_layout : require
#extension GL_EXT_samplerless_texture_functions : require

layout(set = 0, binding = 13, scalar) uniform type_Globals
{
    uint transform_image;
    uvec2 framebuffer_dims;
} _Globals;

layout(set = 0, binding = 14) uniform texture2D input_tex;

layout(location = 0) out vec4 out_var_SV_TARGET;

void main()
{
    ivec2 _51 = ivec2(gl_FragCoord.xy);
    ivec2 _68;
    if ((_Globals.transform_image & 1u) != 0u)
    {
        ivec2 _67 = _51;
        _67.y = int((_Globals.framebuffer_dims.y - 1u) - uint(_51.y));
        _68 = _67;
    }
    else
    {
        _68 = _51;
    }
    vec4 _70 = texelFetch(input_tex, ivec2(uvec2(_68)), int(0u));
    float _71 = _70.x;
    float _80;
    if (0.003130800090730190277099609375 >= _71)
    {
        _80 = 12.9200000762939453125 * _71;
    }
    else
    {
        _80 = (1.05499994754791259765625 * pow(_71, 0.4166666567325592041015625)) - 0.054999999701976776123046875;
    }
    float _81 = _70.y;
    float _90;
    if (0.003130800090730190277099609375 >= _81)
    {
        _90 = 12.9200000762939453125 * _81;
    }
    else
    {
        _90 = (1.05499994754791259765625 * pow(_81, 0.4166666567325592041015625)) - 0.054999999701976776123046875;
    }
    float _91 = _70.z;
    float _100;
    if (0.003130800090730190277099609375 >= _91)
    {
        _100 = 12.9200000762939453125 * _91;
    }
    else
    {
        _100 = (1.05499994754791259765625 * pow(_91, 0.4166666567325592041015625)) - 0.054999999701976776123046875;
    }
    out_var_SV_TARGET = vec4(_80, _90, _100, 1.0);
}

