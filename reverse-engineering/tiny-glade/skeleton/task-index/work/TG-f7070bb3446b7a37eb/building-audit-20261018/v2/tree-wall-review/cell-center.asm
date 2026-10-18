140aa5c70: 56                           push rsi
140aa5c71: 57                           push rdi
140aa5c72: 4883ec78                     sub rsp, 0x78
140aa5c76: 440f29542460                 movaps xmmword ptr [rsp + 0x60], xmm10
140aa5c7c: 440f294c2450                 movaps xmmword ptr [rsp + 0x50], xmm9
140aa5c82: 440f29442440                 movaps xmmword ptr [rsp + 0x40], xmm8
140aa5c88: 0f297c2430                   movaps xmmword ptr [rsp + 0x30], xmm7
140aa5c8d: 0f29742420                   movaps xmmword ptr [rsp + 0x20], xmm6
140aa5c92: 4889ce                       mov rsi, rcx
140aa5c95: 8b4948                       mov ecx, dword ptr [rcx + 0x48]
140aa5c98: 85c9                         test ecx, ecx
140aa5c9a: 0f8410010000                 je 0x140aa5db0
140aa5ca0: 89c8                         mov eax, ecx
140aa5ca2: f7d0                         not eax
140aa5ca4: 448d8200000080               lea r8d, [rdx - 0x80000000]
140aa5cab: 4109c0                       or r8d, eax
140aa5cae: 0f8408010000                 je 0x140aa5dbc
140aa5cb4: f30f2ac2                     cvtsi2ss xmm0, edx
140aa5cb8: f30f2ac9                     cvtsi2ss xmm1, ecx
140aa5cbc: 89d0                         mov eax, edx
140aa5cbe: 99                           cdq 
140aa5cbf: f7f9                         idiv ecx
140aa5cc1: 89d7                         mov edi, edx
140aa5cc3: f30f5ec1                     divss xmm0, xmm1
140aa5cc7: e814e2e701                   call 0x142923ee0
140aa5ccc: f30f2cc0                     cvttss2si eax, xmm0
140aa5cd0: 0f2e05e5ecef01               ucomiss xmm0, dword ptr [rip + 0x1efece5]
140aa5cd7: b9ffffff7f                   mov ecx, 0x7fffffff
140aa5cdc: 0f46c8                       cmovbe ecx, eax
140aa5cdf: 31c0                         xor eax, eax
140aa5ce1: 0f2ec0                       ucomiss xmm0, xmm0
140aa5ce4: 0f4bc1                       cmovnp eax, ecx
140aa5ce7: 0f57c0                       xorps xmm0, xmm0
140aa5cea: f30f2ac7                     cvtsi2ss xmm0, edi
140aa5cee: 488b4e48                     mov rcx, qword ptr [rsi + 0x48]
140aa5cf2: f30f2ad9                     cvtsi2ss xmm3, ecx
140aa5cf6: f30f2ae0                     cvtsi2ss xmm4, eax
140aa5cfa: 48c1e920                     shr rcx, 0x20
140aa5cfe: f30f2ae9                     cvtsi2ss xmm5, ecx
140aa5d02: f30f107650                   movss xmm6, dword ptr [rsi + 0x50]
140aa5d07: f30f104e54                   movss xmm1, dword ptr [rsi + 0x54]
140aa5d0c: f30f10155cfbe701             movss xmm2, dword ptr [rip + 0x1e7fb5c]
140aa5d14: 0f28fe                       movaps xmm7, xmm6
140aa5d17: f30f59fa                     mulss xmm7, xmm2
140aa5d1b: 440f28c1                     movaps xmm8, xmm1
140aa5d1f: f3440f59c2                   mulss xmm8, xmm2
140aa5d24: 0f14f0                       unpcklps xmm6, xmm0
140aa5d27: f30f12c3                     movsldup xmm0, xmm3
140aa5d2b: 0f5ef0                       divps xmm6, xmm0
140aa5d2e: f30f16c6                     movshdup xmm0, xmm6
140aa5d32: f30f101dcafbe701             movss xmm3, dword ptr [rip + 0x1e7fbca]
140aa5d3a: 440f28cb                     movaps xmm9, xmm3
140aa5d3e: f3440f5cc8                   subss xmm9, xmm0
140aa5d43: f3440f59cf                   mulss xmm9, xmm7
140aa5d48: 440f28d2                     movaps xmm10, xmm2
140aa5d4c: 440f14d7                     unpcklps xmm10, xmm7
140aa5d50: 440f59d6                     mulps xmm10, xmm6
140aa5d54: f3410f16c2                   movshdup xmm0, xmm10
140aa5d59: f3410f5cc1                   subss xmm0, xmm9
140aa5d5e: 0f14cc                       unpcklps xmm1, xmm4
140aa5d61: f30f12e5                     movsldup xmm4, xmm5
140aa5d65: 0f5ecc                       divps xmm1, xmm4
140aa5d68: f30f16e1                     movshdup xmm4, xmm1
140aa5d6c: f30f5cdc                     subss xmm3, xmm4
140aa5d70: f3410f59d8                   mulss xmm3, xmm8
140aa5d75: 410f14d0                     unpcklps xmm2, xmm8
140aa5d79: 0f59d1                       mulps xmm2, xmm1
140aa5d7c: f30f16ca                     movshdup xmm1, xmm2
140aa5d80: f30f5ccb                     subss xmm1, xmm3
140aa5d84: f3410f58c2                   addss xmm0, xmm10
140aa5d89: f30f58ca                     addss xmm1, xmm2
140aa5d8d: 0f28742420                   movaps xmm6, xmmword ptr [rsp + 0x20]
140aa5d92: 0f287c2430                   movaps xmm7, xmmword ptr [rsp + 0x30]
140aa5d97: 440f28442440                 movaps xmm8, xmmword ptr [rsp + 0x40]
140aa5d9d: 440f284c2450                 movaps xmm9, xmmword ptr [rsp + 0x50]
140aa5da3: 440f28542460                 movaps xmm10, xmmword ptr [rsp + 0x60]
140aa5da9: 4883c478                     add rsp, 0x78
140aa5dad: 5f                           pop rdi
140aa5dae: 5e                           pop rsi
140aa5daf: c3                           ret 
140aa5db0: 488d0de1700302               lea rcx, [rip + 0x20370e1]
140aa5db7: e8643ce301                   call 0x1428d9a20
140aa5dbc: 488d0dd5700302               lea rcx, [rip + 0x20370d5]
140aa5dc3: e8d83be301                   call 0x1428d99a0
140aa5dc8: cc                           int3 
