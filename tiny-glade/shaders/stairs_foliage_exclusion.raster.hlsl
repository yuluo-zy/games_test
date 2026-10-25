// ==RECONSTRUCTED== shaders/stairs_foliage_exclusion.raster.hlsl | target: _stairs_foliage_exclusion.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin
// 楼梯植被剔除:VS 读取 mesh_buf 顶点条,position = (v.xz * (2/130, -2/130), 1,1),
// texcoord0 = v.xyz;PS 在世界高度超过地形高度 + 1.75 时 discard,否则输出白色。
// 绑定:mesh_buf t16 (vs);sampler_lnc s0, visual_terrain_heightmap t15 (ps)。
// 说明:该文件所有变体代码均由 batch_lift.py 从 oracle bin 提升,行为与原件一致。

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _stairs_foliage_exclusion.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
ByteAddressBuffer mesh_buf : register(t16, space0);

static float4 gl_Position;
static int gl_VertexIndex;
static float3 out_var_C0;

struct SPIRV_Cross_Input
{
    uint gl_VertexIndex : SV_VertexID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    float4 gl_Position : SV_Position;
};

// 顶点变换:把网格条带顶点的 xz 缩放到世界坐标(x 方向 2/130、z 方向 -2/130)。
void vert_main()
{
    gl_Position = float4(mesh_buf.Load<float4>(uint(gl_VertexIndex) * 16 + 0).xz * float2(0.015384615398943424224853515625f, -0.015384615398943424224853515625f), 1.0f, 1.0f);
    out_var_C0 = mesh_buf.Load<float4>(uint(gl_VertexIndex) * 16 + 0).xyz;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_VertexIndex = int(stage_input.gl_VertexIndex);
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _stairs_foliage_exclusion.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
SamplerState sampler_lnc : register(s0, space0);
Texture2D<float4> visual_terrain_heightmap : register(t15, space0);

static float3 in_var_C0;
static float4 out_var_SV_TARGET;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_C0 : TEXCOORD0;
};

struct SPIRV_Cross_Output
{
    float4 out_var_SV_TARGET : SV_Target0;
};

// 片元在世界高度超出地形高度 + 1.75 时剔除,其余输出白色(全 1)。
void frag_main()
{
    if (in_var_C0.y > (lerp(-2.5f, 24.0f, visual_terrain_heightmap.SampleLevel(sampler_lnc, (in_var_C0.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x) + 1.75f))
    {
        discard;
    }
    out_var_SV_TARGET = 1.0f.xxxx;
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C0 = stage_input.in_var_C0;
    frag_main();
    SPIRV_Cross_Output stage_output;
    stage_output.out_var_SV_TARGET = out_var_SV_TARGET;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _stairs_foliage_exclusion.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
ByteAddressBuffer mesh_buf : register(t16, space0);

static float4 gl_Position;
static int gl_VertexIndex;
static float3 out_var_C0;

struct SPIRV_Cross_Input
{
    uint gl_VertexIndex : SV_VertexID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    float4 gl_Position : SV_Position;
};

// 顶点变换:与 b903e43ffb3da915 变体一致(网格条带 xz -> 世界坐标)。
void vert_main()
{
    gl_Position = float4(mesh_buf.Load<float4>(uint(gl_VertexIndex) * 16 + 0).xz * float2(0.015384615398943424224853515625f, -0.015384615398943424224853515625f), 1.0f, 1.0f);
    out_var_C0 = mesh_buf.Load<float4>(uint(gl_VertexIndex) * 16 + 0).xyz;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_VertexIndex = int(stage_input.gl_VertexIndex);
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _stairs_foliage_exclusion.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
SamplerState sampler_lnc : register(s0, space0);
Texture2D<float4> visual_terrain_heightmap : register(t15, space0);

static float3 in_var_C0;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_C0 : TEXCOORD0;
};

// 片元在世界高度超出地形高度 + 1.75 时剔除(此变体无输出目标)。
void frag_main()
{
    if (in_var_C0.y > (lerp(-2.5f, 24.0f, visual_terrain_heightmap.SampleLevel(sampler_lnc, (in_var_C0.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x) + 1.75f))
    {
        discard;
    }
}

void ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C0 = stage_input.in_var_C0;
    frag_main();
}
#pragma dxc diagnostic pop
#endif
