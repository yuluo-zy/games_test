// ==RECONSTRUCTED== shaders/inc/reservoir.hlsl | evidence: raw-per-bin OpMemberName Reservoir1spp{w_sum,payload,M,W} 16B + Reservoir1sppStreamState{p_q_sel,M_sum} 8B; spirv-cross (_deferred_*_light_sun_sky/_rdi_temporal/_gi_*) ReSTIR update/resolve math (w_sum+=w, M+=1, replace payload when w/w_sum>=u, W=w_sum/(M*p_q_sel)) | status: O (function names Inference: inlined)

#ifndef TINYGLADE_INC_RESERVOIR_HLSL
#define TINYGLADE_INC_RESERVOIR_HLSL

// 1-sample-per-pixel ReSTIR reservoir (16 bytes, matches rdi_reservoir_tex uint4).
struct Reservoir1spp {
    float w_sum;
    uint payload;
    float M;
    float W;
};

// Per-stream accumulation used while merging candidate reservoirs.
struct Reservoir1sppStreamState {
    float p_q_sel; // target PDF of the currently selected sample
    float M_sum;   // summed sample count over merged streams
};

// Zero-initialize a reservoir and its stream state.
void reservoir_init(out Reservoir1spp r, out Reservoir1sppStreamState s)
{
    r.w_sum = 0.0;
    r.payload = 0xFFFFFFFFu;
    r.M = 0.0;
    r.W = 0.0;
    s.p_q_sel = 0.0;
    s.M_sum = 0.0;
}

// Weighted-reservoir sample update (equivalent to the inlined opcodes observed
// in the deferred/reservoir consumers): accumulate w and M, and replace the
// stored sample with probability w / w_sum.
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

// Merge a second stream into the first (M_sum accumulates).
void reservoir_combine_streams(inout Reservoir1sppStreamState dst, Reservoir1sppStreamState src)
{
    dst.M_sum += src.M_sum;
}

// Finish a reservoir after stream combination: M takes the summed sample count
// and W becomes the unbiased weight w_sum / (M * p_q_sel).
void reservoir_resolve(inout Reservoir1spp r, Reservoir1sppStreamState s)
{
    r.M = s.M_sum;
    r.W = r.w_sum / max(1e-8, r.M * s.p_q_sel);
}

// Pack a reservoir into a uint4 texel (w_sum, payload, M, W).
uint4 pack_reservoir1spp(Reservoir1spp r)
{
    return uint4(asuint(r.w_sum), r.payload, asuint(r.M), asuint(r.W));
}

// Unpack a reservoir from a uint4 texel.
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
