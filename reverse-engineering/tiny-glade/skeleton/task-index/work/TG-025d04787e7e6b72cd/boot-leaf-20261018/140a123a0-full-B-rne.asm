140a123a0: 56 push rsi
140a123a1: 57 push rdi
140a123a2: 4881eca8000000 sub rsp, 0xa8
140a123a9: 440f299c2490000000 movaps xmmword ptr [rsp + 0x90], xmm11
140a123b2: 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
140a123bb: 440f294c2470 movaps xmmword ptr [rsp + 0x70], xmm9
140a123c1: 440f29442460 movaps xmmword ptr [rsp + 0x60], xmm8
140a123c7: 0f297c2450 movaps xmmword ptr [rsp + 0x50], xmm7
140a123cc: 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
140a123d1: 4889c8 mov rax, rcx
140a123d4: 488b0a mov rcx, qword ptr [rdx]
140a123d7: 488b09 mov rcx, qword ptr [rcx]
140a123da: f2440f1001 movsd xmm8, qword ptr [rcx]
140a123df: f3440f104908 movss xmm9, dword ptr [rcx + 8]
140a123e5: 410f28c0 movaps xmm0, xmm8
140a123e9: 410f59c0 mulps xmm0, xmm8
140a123ed: f30f16c8 movshdup xmm1, xmm0
140a123f1: f30f58c8 addss xmm1, xmm0
140a123f5: 410f28c1 movaps xmm0, xmm9
140a123f9: f3410f59c1 mulss xmm0, xmm9
140a123fe: f30f58c1 addss xmm0, xmm1
140a12402: f30f51c0 sqrtss xmm0, xmm0
140a12406: f30f100d1222f201 movss xmm1, dword ptr [rip + 0x1f22212]
140a1240e: 0f2ec8 ucomiss xmm1, xmm0
140a12411: 0f865a010000 jbe 0x140a12571
140a12417: f3440f10510c movss xmm10, dword ptr [rcx + 0xc]
140a1241d: 440f2e150bacfb01 ucomiss xmm10, dword ptr [rip + 0x1fbac0b] ; @1429cd030 '333?'
140a12425: 0f8646010000 jbe 0x140a12571
140a1242b: f30f107910 movss xmm7, dword ptr [rcx + 0x10]
140a12430: f30f593d3834f101 mulss xmm7, dword ptr [rip + 0x1f13438]
140a12438: 0f28c7 movaps xmm0, xmm7
140a1243b: 4889c7 mov rdi, rax
140a1243e: 4c89c6 mov rsi, r8
140a12441: e88a1bf101 call 0x142923fd0 ; 
140a12446: 0f28f0 movaps xmm6, xmm0
140a12449: 0f28c7 movaps xmm0, xmm7
140a1244c: e82f1af101 call 0x142923e80 ; 
140a12451: 4889f8 mov rax, rdi
140a12454: 0f57db xorps xmm3, xmm3
140a12457: f30f59de mulss xmm3, xmm6
140a1245b: 440f13442420 movlps qword ptr [rsp + 0x20], xmm8
140a12461: f3440f114c2428 movss dword ptr [rsp + 0x28], xmm9
140a12468: f3440f1154242c movss dword ptr [rsp + 0x2c], xmm10
140a1246f: f3440f11542430 movss dword ptr [rsp + 0x30], xmm10
140a12476: f3440f11542434 movss dword ptr [rsp + 0x34], xmm10
140a1247d: 0f1054242c movups xmm2, xmmword ptr [rsp + 0x2c]
140a12482: 0f284c2430 movaps xmm1, xmmword ptr [rsp + 0x30]
140a12487: 440f28c6 movaps xmm8, xmm6
140a1248b: 440f14c3 unpcklps xmm8, xmm3
140a1248f: 410f28e0 movaps xmm4, xmm8
140a12493: 410f58e0 addps xmm4, xmm8
140a12497: f30f59dc mulss xmm3, xmm4
140a1249b: 440f59c4 mulps xmm8, xmm4
140a1249f: f30f16ec movshdup xmm5, xmm4
140a124a3: f30f59f5 mulss xmm6, xmm5
140a124a7: f30f59e8 mulss xmm5, xmm0
140a124ab: f30f59e0 mulss xmm4, xmm0
140a124af: f3410f16c0 movshdup xmm0, xmm8
140a124b4: f3440f58c0 addss xmm8, xmm0
140a124b9: f30f103d4334f101 movss xmm7, dword ptr [rip + 0x1f13443]
140a124c1: 440f28cf movaps xmm9, xmm7
140a124c5: f3450f5cc8 subss xmm9, xmm8
140a124ca: 440f28c3 movaps xmm8, xmm3
140a124ce: f3440f58c5 addss xmm8, xmm5
140a124d3: 440f28d0 movaps xmm10, xmm0
140a124d7: f3440f5cd4 subss xmm10, xmm4
140a124dc: f30f58e0 addss xmm4, xmm0
140a124e0: 440f28de movaps xmm11, xmm6
140a124e4: f30f5cf5 subss xmm6, xmm5
140a124e8: 0f14e6 unpcklps xmm4, xmm6
140a124eb: 410f16e1 movlhps xmm4, xmm9
140a124ef: 450f14c8 unpcklps xmm9, xmm8
140a124f3: 450f16ca movlhps xmm9, xmm10
140a124f7: f30f5cdd subss xmm3, xmm5
140a124fb: f30f58c0 addss xmm0, xmm0
140a124ff: f30f5cf8 subss xmm7, xmm0
140a12503: 0f14df unpcklps xmm3, xmm7
140a12506: f3440f58dd addss xmm11, xmm5
140a1250b: 410f16db movlhps xmm3, xmm11
140a1250f: 0fc6d200 shufps xmm2, xmm2, 0
140a12513: 410f59d1 mulps xmm2, xmm9
140a12517: 0f28c1 movaps xmm0, xmm1
140a1251a: 0fc6c100 shufps xmm0, xmm1, 0
140a1251e: 0f59c3 mulps xmm0, xmm3
140a12521: 0fc6c955 shufps xmm1, xmm1, 0x55
140a12525: 0f59cc mulps xmm1, xmm4
140a12528: f30f101e movss xmm3, dword ptr [rsi]
140a1252c: f30f106604 movss xmm4, dword ptr [rsi + 4]
140a12531: f30f106e08 movss xmm5, dword ptr [rsi + 8]
140a12536: 0fc6db00 shufps xmm3, xmm3, 0
140a1253a: 0f59da mulps xmm3, xmm2
140a1253d: 0fc6e400 shufps xmm4, xmm4, 0
140a12541: 0f59e0 mulps xmm4, xmm0
140a12544: 0f58e3 addps xmm4, xmm3
140a12547: 0fc6ed00 shufps xmm5, xmm5, 0
140a1254b: 0f59e9 mulps xmm5, xmm1
140a1254e: 0f58ec addps xmm5, xmm4
140a12551: 0f586c2420 addps xmm5, xmmword ptr [rsp + 0x20]
140a12556: 0f136f04 movlps qword ptr [rdi + 4], xmm5
140a1255a: 0f12ed movhlps xmm5, xmm5
140a1255d: f30f116f0c movss dword ptr [rdi + 0xc], xmm5
140a12562: c7471000000000 mov dword ptr [rdi + 0x10], 0
140a12569: c70701000000 mov dword ptr [rdi], 1
140a1256f: eb06 jmp 0x140a12577
140a12571: c70000000000 mov dword ptr [rax], 0
140a12577: 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
140a1257c: 0f287c2450 movaps xmm7, xmmword ptr [rsp + 0x50]
140a12581: 440f28442460 movaps xmm8, xmmword ptr [rsp + 0x60]
140a12587: 440f284c2470 movaps xmm9, xmmword ptr [rsp + 0x70]
140a1258d: 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
140a12596: 440f289c2490000000 movaps xmm11, xmmword ptr [rsp + 0x90]
140a1259f: 4881c4a8000000 add rsp, 0xa8
140a125a6: 5f pop rdi
140a125a7: 5e pop rsi
140a125a8: c3 ret 
