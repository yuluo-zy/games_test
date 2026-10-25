140aa6520: 56                           push rsi
140aa6521: 57                           push rdi
140aa6522: 53                           push rbx
140aa6523: 4883ec40                     sub rsp, 0x40
140aa6527: 0f297c2430                   movaps xmmword ptr [rsp + 0x30], xmm7
140aa652c: 0f29742420                   movaps xmmword ptr [rsp + 0x20], xmm6
140aa6531: f30f105950                   movss xmm3, dword ptr [rcx + 0x50]
140aa6536: f30f106154                   movss xmm4, dword ptr [rcx + 0x54]
140aa653b: f30f10352df3e701             movss xmm6, dword ptr [rip + 0x1e7f32d]
140aa6543: 0f28c3                       movaps xmm0, xmm3
140aa6546: f30f59c6                     mulss xmm0, xmm6
140aa654a: f30f59f4                     mulss xmm6, xmm4
140aa654e: f30f58c1                     addss xmm0, xmm1
140aa6552: 0f57c9                       xorps xmm1, xmm1
140aa6555: f30f2a4948                   cvtsi2ss xmm1, dword ptr [rcx + 0x48]
140aa655a: f30f58f2                     addss xmm6, xmm2
140aa655e: 0f57d2                       xorps xmm2, xmm2
140aa6561: f30f2a514c                   cvtsi2ss xmm2, dword ptr [rcx + 0x4c]
140aa6566: f30f5ed9                     divss xmm3, xmm1
140aa656a: f30f5ec3                     divss xmm0, xmm3
140aa656e: f30f5ee2                     divss xmm4, xmm2
140aa6572: f30f5ef4                     divss xmm6, xmm4
140aa6576: e865d9e701                   call 0x142923ee0
140aa657b: f30f2cf0                     cvttss2si esi, xmm0
140aa657f: f30f103d35e4ef01             movss xmm7, dword ptr [rip + 0x1efe435]
140aa6587: 0f2ec7                       ucomiss xmm0, xmm7
140aa658a: bfffffff7f                   mov edi, 0x7fffffff
140aa658f: 0f47f7                       cmova esi, edi
140aa6592: 31db                         xor ebx, ebx
140aa6594: 0f2ec0                       ucomiss xmm0, xmm0
140aa6597: 0f4af3                       cmovp esi, ebx
140aa659a: 0f28c6                       movaps xmm0, xmm6
140aa659d: e83ed9e701                   call 0x142923ee0
140aa65a2: f30f2cd0                     cvttss2si edx, xmm0
140aa65a6: 0f2ec7                       ucomiss xmm0, xmm7
140aa65a9: 0f47d7                       cmova edx, edi
140aa65ac: 0f2ec0                       ucomiss xmm0, xmm0
140aa65af: 0f4ad3                       cmovp edx, ebx
140aa65b2: 89f0                         mov eax, esi
140aa65b4: 0f28742420                   movaps xmm6, xmmword ptr [rsp + 0x20]
140aa65b9: 0f287c2430                   movaps xmm7, xmmword ptr [rsp + 0x30]
140aa65be: 4883c440                     add rsp, 0x40
140aa65c2: 5b                           pop rbx
140aa65c3: 5f                           pop rdi
140aa65c4: 5e                           pop rsi
140aa65c5: c3                           ret 
