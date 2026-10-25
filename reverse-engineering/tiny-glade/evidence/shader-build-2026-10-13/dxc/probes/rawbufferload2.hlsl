RWStructuredBuffer<uint> out_buffer : register(u0);
[numthreads(1,1,1)] void cs_main(uint3 tid:SV_DispatchThreadID){out_buffer[tid.x]=vk::RawBufferLoad2<uint>(uint2(0,0));}
