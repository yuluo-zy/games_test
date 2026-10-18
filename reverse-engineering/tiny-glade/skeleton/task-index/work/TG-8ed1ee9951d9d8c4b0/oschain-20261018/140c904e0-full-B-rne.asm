0000000140c904e0: 4883ec48                         sub rsp, 0x48
0000000140c904e4: 0f297c2430                       movaps xmmword ptr [rsp + 0x30], xmm7
0000000140c904e9: 0f29742420                       movaps xmmword ptr [rsp + 0x20], xmm6
0000000140c904ee: 0f28f1                           movaps xmm6, xmm1
0000000140c904f1: 0f28f8                           movaps xmm7, xmm0
0000000140c904f4: 0f5715f553c901                   xorps xmm2, xmmword ptr [rip + 0x1c953f5]
0000000140c904fb: f30f59d3                         mulss xmm2, xmm3
0000000140c904ff: 0f28c2                           movaps xmm0, xmm2
0000000140c90502: e8b939c901                       call 0x142923ec0 ; 
0000000140c90507: f30f100df553c901                 movss xmm1, dword ptr [rip + 0x1c953f5]
0000000140c9050f: f30f5cc8                         subss xmm1, xmm0
0000000140c90513: f30f5cf7                         subss xmm6, xmm7
0000000140c90517: f30f59f1                         mulss xmm6, xmm1
0000000140c9051b: f30f58f7                         addss xmm6, xmm7
0000000140c9051f: 0f28c6                           movaps xmm0, xmm6
0000000140c90522: 0f28742420                       movaps xmm6, xmmword ptr [rsp + 0x20]
0000000140c90527: 0f287c2430                       movaps xmm7, xmmword ptr [rsp + 0x30]
0000000140c9052c: 4883c448                         add rsp, 0x48
0000000140c90530: c3                               ret 
