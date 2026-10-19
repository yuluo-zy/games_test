140c97660: 4883ec58 sub rsp, 0x58
140c97664: 440f29442440 movaps xmmword ptr [rsp + 0x40], xmm8
140c9766a: 0f297c2430 movaps xmmword ptr [rsp + 0x30], xmm7
140c9766f: 0f29742420 movaps xmmword ptr [rsp + 0x20], xmm6
140c97674: f3440f100587e2c801 movss xmm8, dword ptr [rip + 0x1c8e287]
140c9767d: f3440f5cc1 subss xmm8, xmm1
140c97682: f30f1035eacfc901 movss xmm6, dword ptr [rip + 0x1c9cfea]
140c9768a: f3440f59c6 mulss xmm8, xmm6
140c9768f: f30f10410c movss xmm0, dword ptr [rcx + 0xc]
140c97694: 0f28ce movaps xmm1, xmm6
140c97697: e874c8c801 call 0x142923f10 ; unknown
140c9769c: f30f58f0 addss xmm6, xmm0
140c976a0: 0f57c9 xorps xmm1, xmm1
140c976a3: 0f28f8 movaps xmm7, xmm0
140c976a6: f30fc2f901 cmpltss xmm7, xmm1
140c976ab: 0f54f7 andps xmm6, xmm7
140c976ae: 0f55f8 andnps xmm7, xmm0
140c976b1: 0f56fe orps xmm7, xmm6
140c976b4: f3410f58f8 addss xmm7, xmm8
140c976b9: 0f28c7 movaps xmm0, xmm7
140c976bc: e8bfc7c801 call 0x142923e80 ; unknown
140c976c1: 0f28f0 movaps xmm6, xmm0
140c976c4: 0f28c7 movaps xmm0, xmm7
140c976c7: e804c9c801 call 0x142923fd0 ; unknown
140c976cc: 0f57051de2c801 xorps xmm0, xmmword ptr [rip + 0x1c8e21d]
140c976d3: 0f28ce movaps xmm1, xmm6
140c976d6: 0f28742420 movaps xmm6, xmmword ptr [rsp + 0x20]
140c976db: 0f287c2430 movaps xmm7, xmmword ptr [rsp + 0x30]
140c976e0: 440f28442440 movaps xmm8, xmmword ptr [rsp + 0x40]
140c976e6: 4883c458 add rsp, 0x58
140c976ea: c3 ret 
