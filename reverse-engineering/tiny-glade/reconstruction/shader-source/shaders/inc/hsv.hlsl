#pragma once
// ==RECONSTRUCTED== shaders/inc/hsv.hlsl | evidence: _nani_cloth.raster.hlsl_b903e43ffb3da915 m1 (ps) inlined rgb2hsv + hsv2rgb (triangle hue formula with h in [0,6]); include graph 2 bins | status: I
//
// Observation (from _nani_cloth ps inlined use-site):
//   rgb2hsv: p = lerp(c.zy,K.wz / c.yz,K.xy, step(c.z,c.y)); q = lerp(p.xyw,c.x / c.x,p.yzx, step(p.x,c.x));
//            d = q.x - min(q.w,q.y); h = abs(q.z + (q.w-q.y)/(6d+e)); s = d/(q.x+e); v = q.x   (h in [0,1])
//   hsv2rgb: p = abs(abs(h-3)-1, 2-abs(h-2), 2-abs(h-4)); clamp(p,0,1); rgb = lerp(1,p,s)*v   (h in [0,6])
// The hsv2rgb hue input is premultiplied by 6 in the consumer (observed *6 before the triangle
// formula), so this header's hsv2rgb takes h in [0,6].

float3 rgb2hsv(float3 c)
{
    float4 K = float4(0.0f, -1.0f / 3.0f, 2.0f / 3.0f, -1.0f);
    float4 p = lerp(float4(c.zy, K.wz), float4(c.yz, K.xy), step(c.z, c.y));
    float4 q = lerp(float4(p.xyw, c.x), float4(c.x, p.yzx), step(p.x, c.x));
    float d = q.x - min(q.w, q.y);
    float e = 1.0e-10f;
    return float3(abs(q.z + (q.w - q.y) / (6.0f * d + e)), d / (q.x + e), q.x);
}

float3 hsv2rgb(float3 c)
{
    float3 p = abs(float3(abs(c.x - 3.0f) - 1.0f, 2.0f - abs(c.x - 2.0f), 2.0f - abs(c.x - 4.0f)));
    p = clamp(p, 0.0f, 1.0f);
    return lerp(float3(1.0f, 1.0f, 1.0f), p, c.y) * c.z;
}
