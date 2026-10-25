0x140c904e0 sub      rsp, 0x48
0x140c904e4 movaps   xmmword ptr [rsp + 0x30], xmm7
0x140c904e9 movaps   xmmword ptr [rsp + 0x20], xmm6
0x140c904ee movaps   xmm6, xmm1
0x140c904f1 movaps   xmm7, xmm0
0x140c904f4 xorps    xmm2, xmmword ptr [rip + 0x1c953f5]
0x140c904fb mulss    xmm2, xmm3
0x140c904ff movaps   xmm0, xmm2
0x140c90502 call     0x142923ec0
0x140c90507 movss    xmm1, dword ptr [rip + 0x1c953f5]
0x140c9050f subss    xmm1, xmm0
0x140c90513 subss    xmm6, xmm7
0x140c90517 mulss    xmm6, xmm1
0x140c9051b addss    xmm6, xmm7
0x140c9051f movaps   xmm0, xmm6
0x140c90522 movaps   xmm6, xmmword ptr [rsp + 0x20]
0x140c90527 movaps   xmm7, xmmword ptr [rsp + 0x30]
0x140c9052c add      rsp, 0x48
0x140c90530 ret      
