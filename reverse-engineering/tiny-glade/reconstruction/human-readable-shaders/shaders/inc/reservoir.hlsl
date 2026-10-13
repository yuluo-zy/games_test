// ======================================================================
// inc/reservoir.hlsl —— ReSTIR 水库采样公共头文件
// ======================================================================
// 用途:为延迟光照(_deferred_*_light_sun_sky)、RDI 时域(_rdi_temporal)与 GI
//       消费者提供 ReSTIR 水库(1spp)数据结构与更新/合并/结算数学。
//
// 核心公式(与内联 opcode 一致):
//   更新  w_sum += w, M += 1;当 w / w_sum >= u 时以概率替换 payload;
//   结算  W = w_sum / (M * p_q_sel)。
// ======================================================================
// ==RECONSTRUCTED== shaders/inc/reservoir.hlsl | evidence: raw-per-bin OpMemberName Reservoir1spp{w_sum,payload,M,W} 16B + Reservoir1sppStreamState{p_q_sel,M_sum} 8B; spirv-cross (_deferred_*_light_sun_sky/_rdi_temporal/_gi_*) ReSTIR update/resolve math (w_sum+=w, M+=1, replace payload when w/w_sum>=u, W=w_sum/(M*p_q_sel)) | status: O (function names Inference: inlined)

#ifndef TINYGLADE_INC_RESERVOIR_HLSL
#define TINYGLADE_INC_RESERVOIR_HLSL

// 1spp ReSTIR 水库(16 字节,与 rdi_reservoir_tex 的 uint4 布局一致)。
struct Reservoir1spp {
    float w_sum;
    uint payload;
    float M;
    float W;
};

// 合并候选水库时使用的逐流累加状态。
struct Reservoir1sppStreamState {
    float p_q_sel; // 当前选中样本的目标概率密度
    float M_sum;   // 各流合并后的样本数累加
};

// 将水库及其流状态清零。
void reservoir_init(out Reservoir1spp r, out Reservoir1sppStreamState s)
{
    r.w_sum = 0.0;
    r.payload = 0xFFFFFFFFu;
    r.M = 0.0;
    r.W = 0.0;
    s.p_q_sel = 0.0;
    s.M_sum = 0.0;
}

// 加权水库样本更新(与 deferred/reservoir 消费者中的内联 opcode 等价):
// 累加 w 与 M,并以概率 w / w_sum 替换已存储样本。
bool reservoir_update(inout Reservoir1spp r, inout Reservoir1sppStreamState s,
                      float w, float target_pdf, uint payload, float u)
{
    s.M_sum += 1.0;
    r.M += 1.0;
    r.w_sum += w;
    if (w / r.w_sum >= u) {
        r.payload = payload;
        s.p_q_sel = target_pdf;
        return true;
    }
    return false;
}

// 将第二个流并入第一个流(M_sum 累加)。
void reservoir_combine_streams(inout Reservoir1sppStreamState dst, Reservoir1sppStreamState src)
{
    dst.M_sum += src.M_sum;
}

// 流合并后结算水库:M 取合并的样本数,W 变为无偏权重 w_sum / (M * p_q_sel)。
void reservoir_resolve(inout Reservoir1spp r, Reservoir1sppStreamState s)
{
    r.M = s.M_sum;
    r.W = r.w_sum / max(1e-8, r.M * s.p_q_sel);
}

// 将水库打包为 uint4 纹素(w_sum, payload, M, W)。
uint4 pack_reservoir1spp(Reservoir1spp r)
{
    return uint4(asuint(r.w_sum), r.payload, asuint(r.M), asuint(r.W));
}

// 从 uint4 纹素解包水库。
Reservoir1spp unpack_reservoir1spp(uint4 v)
{
    Reservoir1spp r;
    r.w_sum = asfloat(v.x);
    r.payload = v.y;
    r.M = asfloat(v.z);
    r.W = asfloat(v.w);
    return r;
}

#endif // TINYGLADE_INC_RESERVOIR_HLSL
