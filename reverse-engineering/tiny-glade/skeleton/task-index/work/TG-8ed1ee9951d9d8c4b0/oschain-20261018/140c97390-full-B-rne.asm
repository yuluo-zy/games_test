0000000140c97390: 56                               push rsi
0000000140c97391: 57                               push rdi
0000000140c97392: 4883ec58                         sub rsp, 0x58
0000000140c97396: 440f29442440                     movaps xmmword ptr [rsp + 0x40], xmm8
0000000140c9739c: 0f297c2430                       movaps xmmword ptr [rsp + 0x30], xmm7
0000000140c973a1: 0f29742420                       movaps xmmword ptr [rsp + 0x20], xmm6
0000000140c973a6: 4889d7                           mov rdi, rdx
0000000140c973a9: 4889ce                           mov rsi, rcx
0000000140c973ac: 66440f6e4204                     movd xmm8, dword ptr [rdx + 4]
0000000140c973b2: f30f107208                       movss xmm6, dword ptr [rdx + 8]
0000000140c973b7: f30f5935f140e101                 mulss xmm6, dword ptr [rip + 0x1e140f1]
0000000140c973bf: f30f5935a9e4c801                 mulss xmm6, dword ptr [rip + 0x1c8e4a9]
0000000140c973c7: 0f28c6                           movaps xmm0, xmm6
0000000140c973ca: e8b1cac801                       call 0x142923e80 ; 
0000000140c973cf: 0f28f8                           movaps xmm7, xmm0
0000000140c973d2: 0f28c6                           movaps xmm0, xmm6
0000000140c973d5: e8f6cbc801                       call 0x142923fd0 ; 
0000000140c973da: f30f5ef8                         divss xmm7, xmm0
0000000140c973de: 0f28c7                           movaps xmm0, xmm7
0000000140c973e1: f30f5e07                         divss xmm0, dword ptr [rdi]
0000000140c973e5: 0f57c9                           xorps xmm1, xmm1
0000000140c973e8: 0f57d2                           xorps xmm2, xmm2
0000000140c973eb: f30f10d0                         movss xmm2, xmm0
0000000140c973ef: f30f7edf                         movq xmm3, xmm7
0000000140c973f3: 0fc6d9e2                         shufps xmm3, xmm1, 0xe2
0000000140c973f7: f3410f7ee0                       movq xmm4, xmm8
0000000140c973fc: 0f57ed                           xorps xmm5, xmm5
0000000140c973ff: 0fc6ec84                         shufps xmm5, xmm4, 0x84
0000000140c97403: f30f1025f9e4c801                 movss xmm4, dword ptr [rip + 0x1c8e4f9]
0000000140c9740b: 0f28f4                           movaps xmm6, xmm4
0000000140c9740e: f30f5ef0                         divss xmm6, xmm0
0000000140c97412: 0f57c0                           xorps xmm0, xmm0
0000000140c97415: f30f10c6                         movss xmm0, xmm6
0000000140c97419: 0f28f4                           movaps xmm6, xmm4
0000000140c9741c: f30f5ef7                         divss xmm6, xmm7
0000000140c97420: f30f7ef6                         movq xmm6, xmm6
0000000140c97424: 0fc6f1e2                         shufps xmm6, xmm1, 0xe2
0000000140c97428: f3410f5ee0                       divss xmm4, xmm8
0000000140c9742d: f30f7ee4                         movq xmm4, xmm4
0000000140c97431: 0fc6cc24                         shufps xmm1, xmm4, 0x24
0000000140c97435: 0f2916                           movaps xmmword ptr [rsi], xmm2
0000000140c97438: 0f295e10                         movaps xmmword ptr [rsi + 0x10], xmm3
0000000140c9743c: 0f28154d6fe901                   movaps xmm2, xmmword ptr [rip + 0x1e96f4d]
0000000140c97443: 0f295620                         movaps xmmword ptr [rsi + 0x20], xmm2
0000000140c97447: 0f296e30                         movaps xmmword ptr [rsi + 0x30], xmm5
0000000140c9744b: 0f294640                         movaps xmmword ptr [rsi + 0x40], xmm0
0000000140c9744f: 0f297650                         movaps xmmword ptr [rsi + 0x50], xmm6
0000000140c97453: 0f294e60                         movaps xmmword ptr [rsi + 0x60], xmm1
0000000140c97457: 0f2805f299d701                   movaps xmm0, xmmword ptr [rip + 0x1d799f2]
0000000140c9745e: 0f294670                         movaps xmmword ptr [rsi + 0x70], xmm0
0000000140c97462: 4889f0                           mov rax, rsi
0000000140c97465: 0f28742420                       movaps xmm6, xmmword ptr [rsp + 0x20]
0000000140c9746a: 0f287c2430                       movaps xmm7, xmmword ptr [rsp + 0x30]
0000000140c9746f: 440f28442440                     movaps xmm8, xmmword ptr [rsp + 0x40]
0000000140c97475: 4883c458                         add rsp, 0x58
0000000140c97479: 5f                               pop rdi
0000000140c9747a: 5e                               pop rsi
0000000140c9747b: c3                               ret 
