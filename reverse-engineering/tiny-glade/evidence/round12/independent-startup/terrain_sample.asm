1409e9640: 4883ec78 sub rsp, 0x78
1409e9644: 440f29542460 movaps xmmword ptr [rsp + 0x60], xmm10
1409e964a: 440f294c2450 movaps xmmword ptr [rsp + 0x50], xmm9
1409e9650: 440f29442440 movaps xmmword ptr [rsp + 0x40], xmm8
1409e9656: 0f297c2430 movaps xmmword ptr [rsp + 0x30], xmm7
1409e965b: 0f29742420 movaps xmmword ptr [rsp + 0x20], xmm6
1409e9660: 488b4108 mov rax, qword ptr [rcx + 8]
1409e9664: f30f106150 movss xmm4, dword ptr [rcx + 0x50]
1409e9669: f30f105954 movss xmm3, dword ptr [rcx + 0x54]
1409e966e: 488b0d932cc402 mov rcx, qword ptr [rip + 0x2c42c93]
1409e9675: 0f14d1 unpcklps xmm2, xmm1
1409e9678: 0f5e15f1a5fb01 divps xmm2, xmmword ptr [rip + 0x1fba5f1]
1409e967f: 0f58155ac2f301 addps xmm2, xmmword ptr [rip + 0x1f3c25a]
1409e9686: 803900 cmp byte ptr [rcx], 0
1409e9689: 0f846e010000 je 0x1409e97fd
1409e968f: f30f16f2 movshdup xmm6, xmm2
1409e9693: f30f102d69c2f301 movss xmm5, dword ptr [rip + 0x1f3c269]
1409e969b: f30f5df5 minss xmm6, xmm5
1409e969f: f30f5dd5 minss xmm2, xmm5
1409e96a3: f30f100505a6fb01 movss xmm0, dword ptr [rip + 0x1fba605]
1409e96ab: f30f59f0 mulss xmm6, xmm0
1409e96af: f30f59d0 mulss xmm2, xmm0
1409e96b3: f30f10052d0af701 movss xmm0, dword ptr [rip + 0x1f70a2d]
1409e96bb: f30f58f0 addss xmm6, xmm0
1409e96bf: f30f58d0 addss xmm2, xmm0
1409e96c3: 0f57c0 xorps xmm0, xmm0
1409e96c6: f30f5ff0 maxss xmm6, xmm0
1409e96ca: f3480f2cce cvttss2si rcx, xmm6
1409e96cf: 4531c0 xor r8d, r8d
1409e96d2: 0f2ef0 ucomiss xmm6, xmm0
1409e96d5: 410f42c8 cmovb ecx, r8d
1409e96d9: f30f5fd0 maxss xmm2, xmm0
1409e96dd: f30f100deb70fb01 movss xmm1, dword ptr [rip + 0x1fb70eb]
1409e96e5: 0f2ef1 ucomiss xmm6, xmm1
1409e96e8: 41b9ffffffff mov r9d, 0xffffffff
1409e96ee: 410f47c9 cmova ecx, r9d
1409e96f2: f3480f2cd2 cvttss2si rdx, xmm2
1409e96f7: 0f2ed0 ucomiss xmm2, xmm0
1409e96fa: 410f42d0 cmovb edx, r8d
1409e96fe: 0f2ed1 ucomiss xmm2, xmm1
1409e9701: 410f47d1 cmova edx, r9d
1409e9705: 0f57c0 xorps xmm0, xmm0
1409e9708: f3480f2ac1 cvtsi2ss xmm0, rcx
1409e970d: 0f57c9 xorps xmm1, xmm1
1409e9710: f3480f2aca cvtsi2ss xmm1, rdx
1409e9715: f30f5cf0 subss xmm6, xmm0
1409e9719: 81f9ff010000 cmp ecx, 0x1ff
1409e971f: 41baff010000 mov r10d, 0x1ff
1409e9725: 41b8ff010000 mov r8d, 0x1ff
1409e972b: 440f42c1 cmovb r8d, ecx
1409e972f: ffc1 inc ecx
1409e9731: 81f9ff010000 cmp ecx, 0x1ff
1409e9737: 410f43ca cmovae ecx, r10d
1409e973b: 81faff010000 cmp edx, 0x1ff
1409e9741: 41b9ff010000 mov r9d, 0x1ff
1409e9747: 440f42ca cmovb r9d, edx
1409e974b: ffc2 inc edx
1409e974d: 81faff010000 cmp edx, 0x1ff
1409e9753: 410f43d2 cmovae edx, r10d
1409e9757: 41c1e109 shl r9d, 9
1409e975b: 4589ca mov r10d, r9d
1409e975e: 4509c2 or r10d, r8d
1409e9761: 4501d2 add r10d, r10d
1409e9764: 460fb71410 movzx r10d, word ptr [rax + r10]
1409e9769: 0f57c0 xorps xmm0, xmm0
1409e976c: f3410f2ac2 cvtsi2ss xmm0, r10d
1409e9771: f30f5cd1 subss xmm2, xmm1
1409e9775: f3440f100dce350902 movss xmm9, dword ptr [rip + 0x20935ce]
1409e977e: f3410f5ec1 divss xmm0, xmm9
1409e9783: 0f28cd movaps xmm1, xmm5
1409e9786: f30f5cc8 subss xmm1, xmm0
1409e978a: f30f59cc mulss xmm1, xmm4
1409e978e: 4109c9 or r9d, ecx
1409e9791: 4501c9 add r9d, r9d
1409e9794: 460fb70c08 movzx r9d, word ptr [rax + r9]
1409e9799: f3410f2af9 cvtsi2ss xmm7, r9d
1409e979e: f30f59c3 mulss xmm0, xmm3
1409e97a2: f30f58c1 addss xmm0, xmm1
1409e97a6: f3410f5ef9 divss xmm7, xmm9
1409e97ab: 0f28cd movaps xmm1, xmm5
1409e97ae: f30f5ccf subss xmm1, xmm7
1409e97b2: f30f59cc mulss xmm1, xmm4
1409e97b6: f30f59fb mulss xmm7, xmm3
1409e97ba: f30f58f9 addss xmm7, xmm1
1409e97be: c1e209 shl edx, 9
1409e97c1: 4109d0 or r8d, edx
1409e97c4: 4501c0 add r8d, r8d
1409e97c7: 460fb70400 movzx r8d, word ptr [rax + r8]
1409e97cc: 0f57c9 xorps xmm1, xmm1
1409e97cf: f3410f2ac8 cvtsi2ss xmm1, r8d
1409e97d4: f3410f5ec9 divss xmm1, xmm9
1409e97d9: 440f28c5 movaps xmm8, xmm5
1409e97dd: f3440f5cc1 subss xmm8, xmm1
1409e97e2: f3440f59c4 mulss xmm8, xmm4
1409e97e7: f30f59cb mulss xmm1, xmm3
1409e97eb: f3410f58c8 addss xmm1, xmm8
1409e97f0: 09ca or edx, ecx
1409e97f2: 01d2 add edx, edx
1409e97f4: 0fb70410 movzx eax, word ptr [rax + rdx]
1409e97f8: e9ad010000 jmp 0x1409e99aa
1409e97fd: 0f57c0 xorps xmm0, xmm0
1409e9800: 0f5fc2 maxps xmm0, xmm2
1409e9803: 0fc2d207 cmpordps xmm2, xmm2
1409e9807: 0f54d0 andps xmm2, xmm0
1409e980a: 0f28059f4ef401 movaps xmm0, xmmword ptr [rip + 0x1f44e9f]
1409e9811: 0f28c8 movaps xmm1, xmm0
1409e9814: 0f5dca minps xmm1, xmm2
1409e9817: 0fc2d203 cmpunordps xmm2, xmm2
1409e981b: 0f54c2 andps xmm0, xmm2
1409e981e: 0f55d1 andnps xmm2, xmm1
1409e9821: 0f56d0 orps xmm2, xmm0
1409e9824: 0f591555a4fb01 mulps xmm2, xmmword ptr [rip + 0x1fba455]
1409e982b: 0f28ea movaps xmm5, xmm2
1409e982e: 0fc6eaff shufps xmm5, xmm2, 0xff
1409e9832: f34c0f2cc5 cvttss2si r8, xmm5
1409e9837: 31c9 xor ecx, ecx
1409e9839: 0f57c0 xorps xmm0, xmm0
1409e983c: 0f2ee8 ucomiss xmm5, xmm0
1409e983f: 440f42c1 cmovb r8d, ecx
1409e9843: f30f100d856ffb01 movss xmm1, dword ptr [rip + 0x1fb6f85]
1409e984b: 0f2ee9 ucomiss xmm5, xmm1
1409e984e: baffffffff mov edx, 0xffffffff
1409e9853: 440f47c2 cmova r8d, edx
1409e9857: 66410f6ee8 movd xmm5, r8d
1409e985c: 0f28f2 movaps xmm6, xmm2
1409e985f: 660f15f2 unpckhpd xmm6, xmm2
1409e9863: f34c0f2cc6 cvttss2si r8, xmm6
1409e9868: 0f2ef0 ucomiss xmm6, xmm0
1409e986b: 440f42c1 cmovb r8d, ecx
1409e986f: 0f2ef1 ucomiss xmm6, xmm1
1409e9872: 440f47c2 cmova r8d, edx
1409e9876: 66410f6ef8 movd xmm7, r8d
1409e987b: f34c0f2cc2 cvttss2si r8, xmm2
1409e9880: 0f2ed0 ucomiss xmm2, xmm0
1409e9883: 440f42c1 cmovb r8d, ecx
1409e9887: 0f2ed1 ucomiss xmm2, xmm1
1409e988a: 440f47c2 cmova r8d, edx
1409e988e: f30f16f2 movshdup xmm6, xmm2
1409e9892: f34c0f2cce cvttss2si r9, xmm6
1409e9897: 0f2ef0 ucomiss xmm6, xmm0
1409e989a: 440f42c9 cmovb r9d, ecx
1409e989e: 0f2ef1 ucomiss xmm6, xmm1
1409e98a1: 660f62fd punpckldq xmm7, xmm5
1409e98a5: 440f47ca cmova r9d, edx
1409e98a9: 66410f6ec0 movd xmm0, r8d
1409e98ae: 66410f6ec9 movd xmm1, r9d
1409e98b3: 660f62c1 punpckldq xmm0, xmm1
1409e98b7: 660f6cc7 punpcklqdq xmm0, xmm7
1409e98bb: 660f6f0d2dc0f301 movdqa xmm1, xmmword ptr [rip + 0x1f3c02d]
1409e98c3: 660fefc8 pxor xmm1, xmm0
1409e98c7: 660f660dc1a3fb01 pcmpgtd xmm1, xmmword ptr [rip + 0x1fba3c1]
1409e98cf: 660f6fe9 movdqa xmm5, xmm1
1409e98d3: 660fdb0dc5a3fb01 pand xmm1, xmmword ptr [rip + 0x1fba3c5]
1409e98db: 660fdfe8 pandn xmm5, xmm0
1409e98df: 660febcd por xmm1, xmm5
1409e98e3: 660f70e955 pshufd xmm5, xmm1, 0x55
1409e98e8: 660f7ee9 movd ecx, xmm5
1409e98ec: 660f7eca movd edx, xmm1
1409e98f0: c1e209 shl edx, 9
1409e98f3: 4c8d040a lea r8, [rdx + rcx]
1409e98f7: 49ffc0 inc r8
1409e98fa: 4c8d0c0a lea r9, [rdx + rcx]
1409e98fe: 4981c100020000 add r9, 0x200
1409e9905: 4c8d140a lea r10, [rdx + rcx]
1409e9909: 4981c201020000 add r10, 0x201
1409e9910: 4809ca or rdx, rcx
1409e9913: 0fb70c50 movzx ecx, word ptr [rax + rdx*2]
1409e9917: 0f57c0 xorps xmm0, xmm0
1409e991a: f30f2ac1 cvtsi2ss xmm0, ecx
1409e991e: 0f5bed cvtdq2ps xmm5, xmm5
1409e9921: 0f5bc9 cvtdq2ps xmm1, xmm1
1409e9924: f30f5cf5 subss xmm6, xmm5
1409e9928: f30f5cd1 subss xmm2, xmm1
1409e992c: f3440f100d17340902 movss xmm9, dword ptr [rip + 0x2093417]
1409e9935: f3410f5ec1 divss xmm0, xmm9
1409e993a: f30f102dc2bff301 movss xmm5, dword ptr [rip + 0x1f3bfc2]
1409e9942: 0f28cd movaps xmm1, xmm5
1409e9945: f30f5cc8 subss xmm1, xmm0
1409e9949: f30f59cc mulss xmm1, xmm4
1409e994d: f30f59c3 mulss xmm0, xmm3
1409e9951: f30f58c1 addss xmm0, xmm1
1409e9955: 420fb70c40 movzx ecx, word ptr [rax + r8*2]
1409e995a: 0f57ff xorps xmm7, xmm7
1409e995d: f30f2af9 cvtsi2ss xmm7, ecx
1409e9961: f3410f5ef9 divss xmm7, xmm9
1409e9966: 440f28c5 movaps xmm8, xmm5
1409e996a: f3440f5cc7 subss xmm8, xmm7
1409e996f: f3440f59c4 mulss xmm8, xmm4
1409e9974: f30f59fb mulss xmm7, xmm3
1409e9978: 420fb70c48 movzx ecx, word ptr [rax + r9*2]
1409e997d: 0f57c9 xorps xmm1, xmm1
1409e9980: f30f2ac9 cvtsi2ss xmm1, ecx
1409e9984: f3410f58f8 addss xmm7, xmm8
1409e9989: f3410f5ec9 divss xmm1, xmm9
1409e998e: 440f28c5 movaps xmm8, xmm5
1409e9992: f3440f5cc1 subss xmm8, xmm1
1409e9997: f3440f59c4 mulss xmm8, xmm4
1409e999c: f30f59cb mulss xmm1, xmm3
1409e99a0: f3410f58c8 addss xmm1, xmm8
1409e99a5: 420fb70450 movzx eax, word ptr [rax + r10*2]
1409e99aa: 450f57c0 xorps xmm8, xmm8
1409e99ae: f3440f2ac0 cvtsi2ss xmm8, eax
1409e99b3: f3450f5ec1 divss xmm8, xmm9
1409e99b8: f3410f5ce8 subss xmm5, xmm8
1409e99bd: f30f59ec mulss xmm5, xmm4
1409e99c1: f3440f59c3 mulss xmm8, xmm3
1409e99c6: f3440f58c5 addss xmm8, xmm5
1409e99cb: 440f28ca movaps xmm9, xmm2
1409e99cf: e8fc6a2a00 call 0x140c904d0
1409e99d4: 440f28d0 movaps xmm10, xmm0
1409e99d8: 0f28c7 movaps xmm0, xmm7
1409e99db: 410f28c8 movaps xmm1, xmm8
1409e99df: 410f28d1 movaps xmm2, xmm9
1409e99e3: e8e86a2a00 call 0x140c904d0
1409e99e8: 0f28c8 movaps xmm1, xmm0
1409e99eb: 410f28c2 movaps xmm0, xmm10
1409e99ef: 0f28d6 movaps xmm2, xmm6
1409e99f2: 0f28742420 movaps xmm6, xmmword ptr [rsp + 0x20]
1409e99f7: 0f287c2430 movaps xmm7, xmmword ptr [rsp + 0x30]
1409e99fc: 440f28442440 movaps xmm8, xmmword ptr [rsp + 0x40]
1409e9a02: 440f284c2450 movaps xmm9, xmmword ptr [rsp + 0x50]
1409e9a08: 440f28542460 movaps xmm10, xmmword ptr [rsp + 0x60]
1409e9a0e: 4883c478 add rsp, 0x78
1409e9a12: e9b96a2a00 jmp 0x140c904d0
