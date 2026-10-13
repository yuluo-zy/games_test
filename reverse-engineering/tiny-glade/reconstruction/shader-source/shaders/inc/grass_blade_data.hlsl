// ==RECONSTRUCTED== shaders/inc/grass_blade_data.hlsl | evidence: raw-per-bin + spirv-cross (_generate_grass/_grass.raster/_grass_lod1.raster) GrassBladeDataPacked{position,color,packed0,packed1,packed2,pad} 32B; byte packing decoded from vertex reads (offset 12 rgb8, 16 bend/scale bytes, 20/24 half2 pairs) | status: O/I (unpack function names Inference)

#ifndef TINYGLADE_INC_GRASS_BLADE_DATA_HLSL
#define TINYGLADE_INC_GRASS_BLADE_DATA_HLSL

// A grass blade instance, 32 bytes. Members keep the embedded OpMemberName
// names verbatim; the pack/unpack helpers below own the byte layout.
struct GrassBladeDataPacked {
    float3 position; // world position (3 floats)
    uint color;      // rgb8 (bytes r|g<<8|b<<16), byte3 unused
    uint packed0;    // bytes: [bend_y?][scale?][bend_y2?][dir_x?]
    uint packed1;    // half2(width_a, 0) | half2(width_b, 0) << 16
    uint packed2;    // half2(2.0, 0) | half2(bend_xz, 0) << 16
    uint pad;
};

// Decode the rgb8 color field into 0..1 rgb.
float3 grass_blade_color(GrassBladeDataPacked b)
{
    return float3(float(b.color & 255u) * (1.0 / 255.0),
                  float((b.color >> 8u) & 255u) * (1.0 / 255.0),
                  float((b.color >> 16u) & 255u) * (1.0 / 255.0));
}

// Two widths (bottom/top) packed as two half2s.
float2 grass_blade_widths(GrassBladeDataPacked b)
{
    return float2(f16tof32(b.packed1 & 0xffffu), f16tof32((b.packed1 >> 16u) & 0xffffu));
}

// Bending along the local xz plane, packed in the high half2 of packed2.
float grass_blade_bend_xz(GrassBladeDataPacked b)
{
    return f16tof32((b.packed2 >> 16u) & 0xffffu);
}

// Direction hint in the xz plane, byte3 of packed0 remapped to [-1, 1].
float grass_blade_dir_x(GrassBladeDataPacked b)
{
    return float((b.packed0 >> 24u) & 255u) * (2.0 / 255.0) - 1.0;
}

#endif // TINYGLADE_INC_GRASS_BLADE_DATA_HLSL
