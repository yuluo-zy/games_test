// Trivial compute shader for toolchain self-test (identical content, two paths).
cbuffer Constants : register(b0, space1) {
    float4 params;
};
Texture2D<float4> in_tex : register(t0, space0);
RWTexture2D<float4> out_tex : register(u0, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 tid : SV_DispatchThreadID) {
    float4 c = in_tex[tid.xy];
    out_tex[tid.xy] = c * params;
}
