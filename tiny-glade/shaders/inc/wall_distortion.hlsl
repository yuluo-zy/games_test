// 墙体材质 UV 微扰动工具:给墙/灰泥/砖块表面的烘焙材质添加低频程序化扰动,
// 使同一材质在一面墙上看起来非均匀。函数名与函数体为推断(原版内联)。

#ifndef TINYGLADE_INC_WALL_DISTORTION_HLSL
#define TINYGLADE_INC_WALL_DISTORTION_HLSL

// 应用于墙/灰泥/砖块 UV 的低频程序化扰动。
float2 wall_distortion_uv(float2 uv, float2 world_pos, float time)
{
    float2 wave = float2(sin(world_pos.x * 0.125f + time * 0.5f),
                         cos(world_pos.y * 0.125f + time * 0.5f));
    return uv + wave * 0.0025f;
}

#endif // TINYGLADE_INC_WALL_DISTORTION_HLSL
