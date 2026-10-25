// ============================================================
// inc/grass_blade_data.hlsl —— 草叶实例数据解码
// 用途:把 32 字节的 GrassBladeDataPacked 打包草叶数据解出颜色/宽度/弯曲等。
//   消费方:_generate_grass、_grass.raster、_grass_lod1.raster。
// 入口:无(纯头文件)
// 关键布局:32B;颜色 rgb8@12,弯曲/缩放字节@16,packed1/2 为 half2 对
// ============================================================
// ==RECONSTRUCTED== shaders/inc/grass_blade_data.hlsl | evidence: raw-per-bin + spirv-cross (_generate_grass/_grass.raster/_grass_lod1.raster) GrassBladeDataPacked{position,color,packed0,packed1,packed2,pad} 32B; byte packing decoded from vertex reads (offset 12 rgb8, 16 bend/scale bytes, 20/24 half2 pairs) | status: O/I (unpack function names Inference)

#ifndef TINYGLADE_INC_GRASS_BLADE_DATA_HLSL
#define TINYGLADE_INC_GRASS_BLADE_DATA_HLSL

// 一片草叶实例,32 字节。成员沿用嵌入的 OpMemberName 原名字;
// 下面的 pack/unpack 辅助函数负责字节布局。
struct GrassBladeDataPacked {
    float3 position; // 世界坐标(3 float)
    uint color;      // rgb8(字节 r|g<<8|b<<16),第 4 字节未用
    uint packed0;    // 字节:[bend_y?][scale?][bend_y2?][dir_x?]
    uint packed1;    // half2(width_a, 0) | half2(width_b, 0) << 16
    uint packed2;    // half2(2.0, 0) | half2(bend_xz, 0) << 16
    uint pad;
};

// 把 rgb8 颜色字段解码为 0..1 的 rgb。
float3 grass_blade_color(GrassBladeDataPacked b)
{
    return float3(float(b.color & 255u) * (1.0 / 255.0),
                  float((b.color >> 8u) & 255u) * (1.0 / 255.0),
                  float((b.color >> 16u) & 255u) * (1.0 / 255.0));
}

// 两个宽度(底部/顶部),打包在两个 half2 里。
float2 grass_blade_widths(GrassBladeDataPacked b)
{
    return float2(f16tof32(b.packed1 & 0xffffu), f16tof32((b.packed1 >> 16u) & 0xffffu));
}

// 局部 xz 平面上的弯曲,打包在 packed2 的高 half2 里。
float grass_blade_bend_xz(GrassBladeDataPacked b)
{
    return f16tof32((b.packed2 >> 16u) & 0xffffu);
}

// xz 平面的方向提示,packed0 的字节 3 重映射到 [-1, 1]。
float grass_blade_dir_x(GrassBladeDataPacked b)
{
    return float((b.packed0 >> 24u) & 255u) * (2.0 / 255.0) - 1.0;
}

#endif // TINYGLADE_INC_GRASS_BLADE_DATA_HLSL
