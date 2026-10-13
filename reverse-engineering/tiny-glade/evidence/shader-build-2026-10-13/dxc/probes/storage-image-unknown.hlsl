[[vk::image_format("unknown")]] RWTexture2D<float4> data_tex:register(u0); [numthreads(1,1,1)] void cs_main(uint3 tid:SV_DispatchThreadID){data_tex[tid.xy]=data_tex[tid.xy]+1;}
