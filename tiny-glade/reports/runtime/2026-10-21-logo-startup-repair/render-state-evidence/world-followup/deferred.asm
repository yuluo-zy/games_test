0x1409d20b0: push rbp
0x1409d20b1: push r15
0x1409d20b3: push r14
0x1409d20b5: push r13
0x1409d20b7: push r12
0x1409d20b9: push rsi
0x1409d20ba: push rdi
0x1409d20bb: push rbx
0x1409d20bc: sub rsp, 0xe08
0x1409d20c3: lea rbp, [rsp + 0x80]
0x1409d20cb: movaps xmmword ptr [rbp + 0xd70], xmm7
0x1409d20d2: movaps xmmword ptr [rbp + 0xd60], xmm6
0x1409d20d9: mov qword ptr [rbp + 0xd58], 0xfffffffffffffffe
0x1409d20e4: mov qword ptr [rbp + 0xd40], r9
0x1409d20eb: mov qword ptr [rbp + 0xd20], r8
0x1409d20f2: mov rbx, rdx
0x1409d20f5: mov rdi, rcx
0x1409d20f8: lea rsi, [rdx + 0x38]
0x1409d20fc: movabs rdx, 0x78cac557b9afa382
0x1409d2106: movabs r8, 0x4199801d7a83a055
0x1409d2110: mov rcx, rsi
0x1409d2113: call 0x1400dd6f0
0x1409d2118: test rax, rax
0x1409d211b: je 0x1409d2f8a
0x1409d2121: cmp qword ptr [rbx + 0x218], rdx
0x1409d2128: jbe 0x1409d2f8a
0x1409d212e: mov rax, qword ptr [rbx + 0x210]
0x1409d2135: mov rcx, qword ptr [rax + rdx*8]
0x1409d2139: test rcx, rcx
0x1409d213c: je 0x1409d2f8a
0x1409d2142: mov rax, qword ptr [rbx + 0x1e0]
0x1409d2149: not rcx
0x1409d214c: lea rcx, [rcx + rcx*2]
0x1409d2150: shl rcx, 5
0x1409d2154: cmp qword ptr [rax + rcx + 0x20], 0
0x1409d215a: je 0x1409d2f8a
0x1409d2160: add rax, rcx
0x1409d2163: mov rax, qword ptr [rax + 0x10]
0x1409d2167: mov qword ptr [rbp + 0xd30], rax
0x1409d216e: movabs rdx, 0x4bc362195b80f729
0x1409d2178: movabs r8, 0x982798ce80cbcefb
0x1409d2182: mov rcx, rsi
0x1409d2185: call 0x1400dd6f0
0x1409d218a: test rax, rax
0x1409d218d: je 0x1409d2f96
0x1409d2193: cmp qword ptr [rbx + 0x218], rdx
0x1409d219a: jbe 0x1409d2f96
0x1409d21a0: mov rax, qword ptr [rbx + 0x210]
0x1409d21a7: mov rcx, qword ptr [rax + rdx*8]
0x1409d21ab: test rcx, rcx
0x1409d21ae: je 0x1409d2f96
0x1409d21b4: mov rax, qword ptr [rbx + 0x1e0]
0x1409d21bb: not rcx
0x1409d21be: lea rcx, [rcx + rcx*2]
0x1409d21c2: shl rcx, 5
0x1409d21c6: cmp qword ptr [rax + rcx + 0x20], 0
0x1409d21cc: je 0x1409d2f96
0x1409d21d2: add rax, rcx
0x1409d21d5: mov rax, qword ptr [rax + 0x10]
0x1409d21d9: mov rcx, qword ptr [rax]
0x1409d21dc: test rcx, rcx
0x1409d21df: js 0x1409d21e8
0x1409d21e1: cvtsi2ss xmm7, rcx
0x1409d21e6: jmp 0x1409d21fd
0x1409d21e8: mov rdx, rcx
0x1409d21eb: shr rdx, 1
0x1409d21ee: and ecx, 1
0x1409d21f1: or rcx, rdx
0x1409d21f4: cvtsi2ss xmm7, rcx
0x1409d21f9: addss xmm7, xmm7
0x1409d21fd: cvtsi2ss xmm6, dword ptr [rax + 8]
0x1409d2202: mov rax, qword ptr [rbp + 0xd20]
0x1409d2209: lea r14, [rax + 0x24]
0x1409d220d: lea rcx, [rax + 0x28]
0x1409d2211: call 0x140bd5790
0x1409d2216: mov dword ptr [rbp + 0x698], 0
0x1409d2220: mov rcx, qword ptr [rbp + 0xd40]
0x1409d2227: mov qword ptr [rbp + 0x6a0], rcx
0x1409d222e: mov qword ptr [rbp + 0xb78], 1
0x1409d2239: mov qword ptr [rbp + 0xcf8], r14
0x1409d2240: mov qword ptr [rbp + 0xb80], r14
0x1409d2247: mov qword ptr [rbp + 0xb88], 0
0x1409d2252: mov qword ptr [rsp + 0x40], rdi
0x1409d2257: mov qword ptr [rbp + 0xd28], rbx
0x1409d225e: mov qword ptr [rsp + 0x38], rbx
0x1409d2263: mov qword ptr [rsp + 0x30], rax
0x1409d2268: lea r14, [rbp + 0xb78]
0x1409d226f: mov qword ptr [rsp + 0x28], r14
0x1409d2274: mov qword ptr [rsp + 0x20], 1
0x1409d227d: lea rdx, [rip + 0x20ef60c]
0x1409d2284: lea rcx, [rbp - 0x38]
0x1409d2288: lea r15, [rbp + 0x698]
0x1409d228f: mov r8d, 0x17
0x1409d2295: mov r9, r15
0x1409d2298: call 0x140867970
0x1409d229d: mov byte ptr [rbp + 0xd57], 1
0x1409d22a4: movabs rdx, 0x8226cc786e679355
0x1409d22ae: movabs r8, 0x65cfda6a55a841ac
0x1409d22b8: mov rcx, rsi
0x1409d22bb: call 0x1400dd6f0
0x1409d22c0: test rax, rax
0x1409d22c3: mov r8, qword ptr [rbp + 0xd28]
0x1409d22ca: je 0x1409d2fa2
0x1409d22d0: cmp qword ptr [r8 + 0x218], rdx
0x1409d22d7: jbe 0x1409d2fa2
0x1409d22dd: mov rax, qword ptr [r8 + 0x210]
0x1409d22e4: mov rcx, qword ptr [rax + rdx*8]
0x1409d22e8: test rcx, rcx
0x1409d22eb: je 0x1409d2fa2
0x1409d22f1: mov rax, qword ptr [r8 + 0x1e0]
0x1409d22f8: not rcx
0x1409d22fb: lea rcx, [rcx + rcx*2]
0x1409d22ff: shl rcx, 5
0x1409d2303: cmp qword ptr [rax + rcx + 0x20], 0
0x1409d2309: je 0x1409d2fa2
0x1409d230f: add rax, rcx
0x1409d2312: mov rcx, qword ptr [rax + 0x10]
0x1409d2316: mov byte ptr [rbp + 0xd57], 1
0x1409d231d: lea rdx, [rip + 0x225bf0c]
0x1409d2324: call 0x140969720
0x1409d2329: mov rsi, rax
0x1409d232c: test rax, rax
0x1409d232f: je 0x1409d2fb7
0x1409d2335: lea rax, [rip + 0x20cafc2]
0x1409d233c: mov qword ptr [rbp + 0xb78], rax
0x1409d2343: mov qword ptr [rbp + 0xb80], 0x35
0x1409d234e: mov rdi, rsi
0x1409d2351: add rdi, 0x10
0x1409d2355: cmp qword ptr [rsi + 0x18], 0x35
0x1409d235a: jne 0x1409d2ef1
0x1409d2360: mov rdx, qword ptr [rdi]
0x1409d2363: lea rcx, [rip + 0x20caf94]
0x1409d236a: mov r8d, 0x35
0x1409d2370: call 0x142923bf0
0x1409d2375: test eax, eax
0x1409d2377: jne 0x1409d2ef1
0x1409d237d: mov rcx, qword ptr [rsi]
0x1409d2380: mov rax, qword ptr [rsi + 8]
0x1409d2384: mov rax, qword ptr [rax + 0x18]
0x1409d2388: mov byte ptr [rbp + 0xd57], 1
0x1409d238f: call rax
0x1409d2391: test rax, rax
0x1409d2394: je 0x1409d2fb7
0x1409d239a: mov rdx, qword ptr [rbp + 0xe30]
0x1409d23a1: mov r8, qword ptr [rbp + 0xe10]
0x1409d23a8: movzx ecx, byte ptr [rbp + 0xdf8]
0x1409d23af: divss xmm6, dword ptr [rip + 0x1fd5289]
0x1409d23b7: addss xmm6, xmm7
0x1409d23bb: mov rax, qword ptr [rax]
0x1409d23be: mov qword ptr [rbp + 0xd18], rax
0x1409d23c5: mov qword ptr [rbp + 0xcc8], 5
0x1409d23d0: mov dword ptr [rbp + 0xcd0], 1
0x1409d23da: mov byte ptr [rbp + 0xcd4], 0
0x1409d23e1: mov dword ptr [rbp + 0xcd8], 3
0x1409d23eb: mov byte ptr [rbp + 0xcdc], 0
0x1409d23f2: mov dword ptr [rbp + 0xce0], 0
0x1409d23fc: mov byte ptr [rbp + 0xce4], 0
0x1409d2403: mov dword ptr [rbp + 0xce8], 0
0x1409d240d: mov byte ptr [rbp + 0xcec], 1
0x1409d2414: mov dword ptr [rbp + 0xcf0], 2
0x1409d241e: mov byte ptr [rbp + 0xcf4], 0
0x1409d2425: movzx eax, cl
0x1409d2428: mov dword ptr [rbp + 0xd3c], eax
0x1409d242e: add qword ptr [rbp + 0xd40], 4
0x1409d2436: lea rax, [r8 + 0x24]
0x1409d243a: mov qword ptr [rbp + 0xd00], rax
0x1409d2441: lea rax, [r8 + 0x48]
0x1409d2445: mov qword ptr [rbp + 0xd08], rax
0x1409d244c: lea rax, [rdx + 0x18]
0x1409d2450: mov qword ptr [rbp + 0xd10], rax
0x1409d2457: xorps xmm7, xmm7
0x1409d245a: lea rsi, [rbp + 0xa80]
0x1409d2461: xor eax, eax
0x1409d2463: mov rdx, qword ptr [rbp + 0xd30]
0x1409d246a: cmp rax, 5
0x1409d246e: je 0x1409d2ea7
0x1409d2474: nop word ptr cs:[rax + rax]
0x1409d2480: lea r12, [rax + 1]
0x1409d2484: mov qword ptr [rbp + 0xcc0], r12
0x1409d248b: mov r13d, dword ptr [rbp + rax*8 + 0xcd0]
0x1409d2493: cmp r13, 5
0x1409d2497: jae 0x1409d2f2c
0x1409d249d: mov ecx, r13d
0x1409d24a0: shl ecx, 4
0x1409d24a3: test byte ptr [rdx + rcx], 1
0x1409d24a7: je 0x1409d2f4b
0x1409d24ad: mov rdi, qword ptr [rdx + rcx + 8]
0x1409d24b2: cmp byte ptr [rbp + rax*8 + 0xcd4], 0
0x1409d24ba: je 0x1409d2560
0x1409d24c0: mov byte ptr [rbp + 0xd57], 1
0x1409d24c7: call 0x140be6750
0x1409d24cc: mov ebx, eax
0x1409d24ce: mov byte ptr [rbp + 0xd57], 1
0x1409d24d5: xor ecx, ecx
0x1409d24d7: call 0x1400d9b20
0x1409d24dc: test rax, rax
0x1409d24df: je 0x1409d2f75
0x1409d24e5: movups xmm0, xmmword ptr [rax]
0x1409d24e8: inc qword ptr [rax]
0x1409d24eb: movups xmm1, xmmword ptr [rip + 0x20c423e]
0x1409d24f2: lea rax, [rbp + 0x6c0]
0x1409d24f9: movups xmmword ptr [rax + 0x10], xmm1
0x1409d24fd: movups xmm1, xmmword ptr [rip + 0x20c421c]
0x1409d2504: movups xmmword ptr [rax], xmm1
0x1409d2507: or r13b, 0x80
0x1409d250b: movups xmm1, xmmword ptr [rbp + 0x698]
0x1409d2512: movups xmm2, xmmword ptr [rbp + 0x6a8]
0x1409d2519: movups xmm3, xmmword ptr [rbp + 0x6b8]
0x1409d2520: movups xmm4, xmmword ptr [rbp + 0x6c8]
0x1409d2527: movaps xmmword ptr [rbp + 0xc70], xmm1
0x1409d252e: movaps xmmword ptr [rbp + 0xc80], xmm2
0x1409d2535: movaps xmmword ptr [rbp + 0xc90], xmm3
0x1409d253c: movaps xmmword ptr [rbp + 0xca0], xmm4
0x1409d2543: mov rax, qword ptr [rbp + 0x6d8]
0x1409d254a: mov qword ptr [rbp + 0xcb0], rax
0x1409d2551: mov eax, ebx
0x1409d2553: shr eax, 0x18
0x1409d2556: and al, 0x9f
0x1409d2558: or al, 0x20
0x1409d255a: jmp 0x1409d25f2
0x1409d255f: nop 
0x1409d2560: mov byte ptr [rbp + 0xd57], 1
0x1409d2567: call 0x140be6750
0x1409d256c: mov ebx, eax
0x1409d256e: mov byte ptr [rbp + 0xd57], 1
0x1409d2575: xor ecx, ecx
0x1409d2577: call 0x1400d9b20
0x1409d257c: test rax, rax
0x1409d257f: je 0x1409d2f60
0x1409d2585: movups xmm0, xmmword ptr [rax]
0x1409d2588: inc qword ptr [rax]
0x1409d258b: movups xmm1, xmmword ptr [rip + 0x20c419e]
0x1409d2592: lea rax, [rbp + 0x6c0]
0x1409d2599: movups xmmword ptr [rax + 0x10], xmm1
0x1409d259d: movups xmm1, xmmword ptr [rip + 0x20c417c]
0x1409d25a4: movups xmmword ptr [rax], xmm1
0x1409d25a7: movups xmm1, xmmword ptr [rbp + 0x698]
0x1409d25ae: movups xmm2, xmmword ptr [rbp + 0x6a8]
0x1409d25b5: movups xmm3, xmmword ptr [rbp + 0x6b8]
0x1409d25bc: movups xmm4, xmmword ptr [rbp + 0x6c8]
0x1409d25c3: movaps xmmword ptr [rbp + 0xc70], xmm1
0x1409d25ca: movaps xmmword ptr [rbp + 0xc80], xmm2
0x1409d25d1: movaps xmmword ptr [rbp + 0xc90], xmm3
0x1409d25d8: movaps xmmword ptr [rbp + 0xca0], xmm4
0x1409d25df: mov rax, qword ptr [rbp + 0x6d8]
0x1409d25e6: mov qword ptr [rbp + 0xcb0], rax
0x1409d25ed: mov eax, ebx
0x1409d25ef: shr eax, 0x18
0x1409d25f2: mov ecx, ebx
0x1409d25f4: and cl, 0xbf
0x1409d25f7: shr ebx, 8
0x1409d25fa: mov qword ptr [rbp + 0xb78], 0
0x1409d2605: mov qword ptr [rbp + 0xb88], 0
0x1409d2610: mov qword ptr [rbp + 0xb90], 4
0x1409d261b: lea r8, [rbp + 0xb98]
0x1409d2622: movups xmmword ptr [r8], xmm7
0x1409d2626: mov qword ptr [rbp + 0xba8], 1
0x1409d2631: mov qword ptr [rbp + 0xbb0], 0
0x1409d263c: movabs rdx, 0x100000003
0x1409d2646: mov qword ptr [rbp + 0xbb8], rdx
0x1409d264d: mov rdx, qword ptr [rbp + 0xcb0]
0x1409d2654: mov qword ptr [r8 + 0x68], rdx
0x1409d2658: movaps xmm1, xmmword ptr [rbp + 0xc70]
0x1409d265f: movaps xmm2, xmmword ptr [rbp + 0xc80]
0x1409d2666: movaps xmm3, xmmword ptr [rbp + 0xc90]
0x1409d266d: movaps xmm4, xmmword ptr [rbp + 0xca0]
0x1409d2674: movups xmmword ptr [r8 + 0x58], xmm4
0x1409d2679: movups xmmword ptr [r8 + 0x48], xmm3
0x1409d267e: movups xmmword ptr [r8 + 0x38], xmm2
0x1409d2683: movups xmmword ptr [r8 + 0x28], xmm1
0x1409d2688: movups xmmword ptr [rbp + 0xc08], xmm0
0x1409d268f: mov qword ptr [rbp + 0xc18], rdi
0x1409d2696: mov byte ptr [rbp + 0xc20], cl
0x1409d269c: mov word ptr [rbp + 0xc21], bx
0x1409d26a3: mov byte ptr [rbp + 0xc23], al
0x1409d26a9: mov dword ptr [rbp + 0xc24], 0x3f800000
0x1409d26b3: mov byte ptr [rbp + 0xc28], 0
0x1409d26ba: mov byte ptr [rbp + 0xc29], r13b
0x1409d26c1: mov byte ptr [rbp + 0xd57], 1
0x1409d26c8: mov rcx, r15
0x1409d26cb: mov rdx, r14
0x1409d26ce: lea r8, [rbp - 0x38]
0x1409d26d2: xor r9d, r9d
0x1409d26d5: call 0x140905be0
0x1409d26da: cmp byte ptr [rbp + 0x776], 8
0x1409d26e1: je 0x1409d270f
0x1409d26e3: mov rdx, qword ptr [rbp + 0x780]
0x1409d26ea: mov eax, 0xe8
0x1409d26ef: add rdx, rax
0x1409d26f2: lea rcx, [rbp + 0x750]
0x1409d26f9: movabs r8, 0x578883943452a027
0x1409d2703: mov r9d, dword ptr [rbp + 0xd3c]
0x1409d270a: call 0x14096b570
0x1409d270f: mov r8d, 0xf8
0x1409d2715: mov rcx, r14
0x1409d2718: mov rdx, r15
0x1409d271b: call 0x142923c00
0x1409d2720: mov rax, qword ptr [rbp + 0xd20]
0x1409d2727: mov qword ptr [rbp + 0xd48], rax
0x1409d272e: mov dword ptr [rbp + 0x698], 2
0x1409d2738: mov qword ptr [rsp + 0x20], r15
0x1409d273d: lea rdi, [rbp + 0xd48]
0x1409d2744: mov rcx, rdi
0x1409d2747: mov rdx, r14
0x1409d274a: movabs r8, 0x41aa4a0d2dea0b36
0x1409d2754: mov r9b, 0x1d
0x1409d2757: call 0x1408ee800
0x1409d275c: mov r8d, 0xf8
0x1409d2762: mov rcx, r15
0x1409d2765: mov rdx, r14
0x1409d2768: call 0x142923c00
0x1409d276d: mov rax, qword ptr [rbp + 0xcf8]
0x1409d2774: mov qword ptr [rbp + 0xd48], rax
0x1409d277b: mov dword ptr [rbp + 0xb78], 2
0x1409d2785: mov qword ptr [rsp + 0x20], r14
0x1409d278a: mov rcx, rdi
0x1409d278d: mov rdx, r15
0x1409d2790: movabs r8, 0x4861579da049fbfc
0x1409d279a: mov r9b, 0x1d
0x1409d279d: call 0x1408ee800
0x1409d27a2: mov r8d, 0xf8
0x1409d27a8: mov rcx, r14
0x1409d27ab: mov rdx, r15
0x1409d27ae: call 0x142923c00
0x1409d27b3: mov rax, qword ptr [rbp + 0xe08]
0x1409d27ba: mov qword ptr [rbp + 0xd48], rax
0x1409d27c1: mov dword ptr [rbp + 0x698], 2
0x1409d27cb: mov qword ptr [rsp + 0x20], r15
0x1409d27d0: mov rcx, rdi
0x1409d27d3: mov rdx, r14
0x1409d27d6: movabs r8, 0x2a333c3a28028230
0x1409d27e0: mov r9b, 0x1d
0x1409d27e3: call 0x1408ee800
0x1409d27e8: mov r8d, 0xf8
0x1409d27ee: mov rcx, r15
0x1409d27f1: mov rdx, r14
0x1409d27f4: call 0x142923c00
0x1409d27f9: mov rax, qword ptr [rbp + 0xdf0]
0x1409d2800: mov qword ptr [rbp + 0xd48], rax
0x1409d2807: mov dword ptr [rbp + 0xb78], 2
0x1409d2811: mov qword ptr [rsp + 0x20], r14
0x1409d2816: mov rcx, rdi
0x1409d2819: mov rdx, r15
0x1409d281c: movabs r8, 0x4175185826c09d0
0x1409d2826: mov r9b, 0x1d
0x1409d2829: call 0x1408ee800
0x1409d282e: mov r8d, 0xf8
0x1409d2834: mov rcx, r14
0x1409d2837: mov rdx, r15
0x1409d283a: call 0x142923c00
0x1409d283f: movabs rax, 0x579f39ab91a01865
0x1409d2849: mov qword ptr [rbp + 0xd48], rax
0x1409d2850: mov dword ptr [rbp + 0x698], 2
0x1409d285a: mov qword ptr [rsp + 0x20], r15
0x1409d285f: mov rcx, rdi
0x1409d2862: mov rdx, r14
0x1409d2865: movabs r8, 0x815bf5036966978c
0x1409d286f: mov r9b, 0x1d
0x1409d2872: call 0x140908610
0x1409d2877: mov r8d, 0xf8
0x1409d287d: mov rcx, r15
0x1409d2880: mov rdx, r14
0x1409d2883: call 0x142923c00
0x1409d2888: mov rax, qword ptr [rbp + 0xe00]
0x1409d288f: mov qword ptr [rbp + 0xd48], rax
0x1409d2896: mov dword ptr [rbp + 0xb78], 2
0x1409d28a0: mov qword ptr [rsp + 0x20], r14
0x1409d28a5: mov rcx, rdi
0x1409d28a8: mov rdx, r15
0x1409d28ab: movabs r8, 0xe003e2bb64399af9
0x1409d28b5: mov r9b, 0x1d
0x1409d28b8: call 0x1408ee800
0x1409d28bd: mov r8d, 0xf8
0x1409d28c3: mov rcx, r14
0x1409d28c6: mov rdx, r15
0x1409d28c9: call 0x142923c00
0x1409d28ce: mov byte ptr [rbp + 0xd48], 8
0x1409d28d5: mov dword ptr [rbp + 0x698], 2
0x1409d28df: mov qword ptr [rsp + 0x20], r15
0x1409d28e4: mov rcx, rdi
0x1409d28e7: mov rdx, r14
0x1409d28ea: movabs r8, 0x77160113ac8dca3f
0x1409d28f4: mov r9b, 0x1d
0x1409d28f7: call 0x140908c50
0x1409d28fc: mov r8d, 0xf8
0x1409d2902: mov rcx, r15
0x1409d2905: mov rdx, r14
0x1409d2908: call 0x142923c00
0x1409d290d: mov byte ptr [rbp + 0xd48], 0xc
0x1409d2914: mov dword ptr [rbp + 0xb78], 2
0x1409d291e: mov qword ptr [rsp + 0x20], r14
0x1409d2923: mov rcx, rdi
0x1409d2926: mov rdx, r15
0x1409d2929: movabs r8, 0x8f870a5c7060b0f4
0x1409d2933: mov r9b, 0x1d
0x1409d2936: call 0x140908c50
0x1409d293b: mov r8d, 0xf8
0x1409d2941: mov rcx, rsi
0x1409d2944: mov rdx, r15
0x1409d2947: call 0x142923c00
0x1409d294c: cmp byte ptr [rbp + 0xb5e], 8
0x1409d2953: je 0x1409d297d
0x1409d2955: mov rdx, qword ptr [rbp + 0xb68]
0x1409d295c: mov eax, 0xe8
0x1409d2961: add rdx, rax
0x1409d2964: lea rcx, [rbp + 0xb38]
0x1409d296b: movabs r8, 0x418a98441f59a54c
0x1409d2975: mov r9b, 1
0x1409d2978: call 0x14096b850
0x1409d297d: mov r8d, 0xf8
0x1409d2983: mov rcx, r15
0x1409d2986: mov rdx, rsi
0x1409d2989: call 0x142923c00
0x1409d298e: mov byte ptr [rbp + 0xd48], 0x11
0x1409d2995: mov dword ptr [rbp + 0xb78], 2
0x1409d299f: mov qword ptr [rsp + 0x20], r14
0x1409d29a4: mov rcx, rdi
0x1409d29a7: mov rdx, r15
0x1409d29aa: movabs r8, 0x11651f4dc9841aa4
0x1409d29b4: mov r9b, 0x1d
0x1409d29b7: call 0x140908c50
0x1409d29bc: mov r8d, 0xf8
0x1409d29c2: mov rcx, r14
0x1409d29c5: mov rdx, r15
0x1409d29c8: call 0x142923c00
0x1409d29cd: mov rax, qword ptr [rbp + 0xe20]
0x1409d29d4: mov qword ptr [rbp + 0xa80], rax
0x1409d29db: mov dword ptr [rbp + 0x698], 2
0x1409d29e5: mov qword ptr [rsp + 0x20], r15
0x1409d29ea: mov rcx, rsi
0x1409d29ed: mov rdx, r14
0x1409d29f0: movabs r8, 0x9941437e56d66d25
0x1409d29fa: mov r9b, 0x1d
0x1409d29fd: call 0x1408ee800
0x1409d2a02: mov r8d, 0xf8
0x1409d2a08: mov rcx, r15
0x1409d2a0b: mov rdx, r14
0x1409d2a0e: call 0x142923c00
0x1409d2a13: mov byte ptr [rbp + 0xd57], 1
0x1409d2a1a: lea rcx, [rbp + 0x4a8]
0x1409d2a21: mov rdx, r15
0x1409d2a24: movabs r8, 0x1e5485ef2e9f9384
0x1409d2a2e: movaps xmm3, xmm6
0x1409d2a31: call 0x141629a20
0x1409d2a36: cmp byte ptr [rbp + 0x586], 8
0x1409d2a3d: je 0x1409d2a67
0x1409d2a3f: mov rdx, qword ptr [rbp + 0x590]
0x1409d2a46: mov eax, 0xe8
0x1409d2a4b: add rdx, rax
0x1409d2a4e: lea rcx, [rbp + 0x560]
0x1409d2a55: movabs r8, 0xc5ab673fb08651f5
0x1409d2a5f: xor r9d, r9d
0x1409d2a62: call 0x14096b570
0x1409d2a67: mov r8d, 0xf8
0x1409d2a6d: mov rcx, r15
0x1409d2a70: lea rdx, [rbp + 0x4a8]
0x1409d2a77: call 0x142923c00
0x1409d2a7c: mov byte ptr [rbp + 0xd56], 1
0x1409d2a83: mov rcx, qword ptr [rbp + 0xd40]
0x1409d2a8a: call 0x140bd5790
0x1409d2a8f: mov byte ptr [rbp + 0xd56], 0
0x1409d2a96: lea rcx, [rbp + 0x3b0]
0x1409d2a9d: mov rdx, r15
0x1409d2aa0: movabs r8, 0x531bb9318eea4971
0x1409d2aaa: mov r9, rax
0x1409d2aad: call 0x141629d20
0x1409d2ab2: cmp byte ptr [rbp + 0x48e], 8
0x1409d2ab9: je 0x1409d2ae7
0x1409d2abb: mov rdx, qword ptr [rbp + 0x498]
0x1409d2ac2: mov eax, 0xe8
0x1409d2ac7: add rdx, rax
0x1409d2aca: lea rcx, [rbp + 0x468]
0x1409d2ad1: movabs r8, 0x9b88329e408ff69f
0x1409d2adb: mov r9, qword ptr [rbp + 0xd18]
0x1409d2ae2: call 0x14096b570
0x1409d2ae7: mov r8d, 0xf8
0x1409d2aed: mov rcx, r15
0x1409d2af0: lea rdx, [rbp + 0x3b0]
0x1409d2af7: call 0x142923c00
0x1409d2afc: mov rax, qword ptr [rbp + 0xe18]
0x1409d2b03: mov qword ptr [rbp + 0xa80], rax
0x1409d2b0a: mov dword ptr [rbp + 0xb78], 2
0x1409d2b14: mov qword ptr [rsp + 0x20], r14
0x1409d2b19: mov rcx, rsi
0x1409d2b1c: mov rdx, r15
0x1409d2b1f: movabs r8, 0xba3e6caf025774d7
0x1409d2b29: mov r9b, 0x1d
0x1409d2b2c: call 0x1408ee800
0x1409d2b31: mov r8d, 0xf8
0x1409d2b37: mov rcx, r14
0x1409d2b3a: mov rdx, r15
0x1409d2b3d: call 0x142923c00
0x1409d2b42: mov rax, qword ptr [rbp + 0xe10]
0x1409d2b49: mov qword ptr [rbp + 0xa80], rax
0x1409d2b50: mov dword ptr [rbp + 0x698], 2
0x1409d2b5a: mov qword ptr [rsp + 0x20], r15
0x1409d2b5f: mov rcx, rsi
0x1409d2b62: mov rdx, r14
0x1409d2b65: movabs r8, 0xfe8da59f97749daa
0x1409d2b6f: mov r9b, 0x1d
0x1409d2b72: call 0x1408ee800
0x1409d2b77: mov r8d, 0xf8
0x1409d2b7d: mov rcx, r15
0x1409d2b80: mov rdx, r14
0x1409d2b83: call 0x142923c00
0x1409d2b88: mov rax, qword ptr [rbp + 0xd00]
0x1409d2b8f: mov qword ptr [rbp + 0xa80], rax
0x1409d2b96: mov dword ptr [rbp + 0xb78], 2
0x1409d2ba0: mov qword ptr [rsp + 0x20], r14
0x1409d2ba5: mov rcx, rsi
0x1409d2ba8: mov rdx, r15
0x1409d2bab: movabs r8, 0xd36dd992ba29df1
0x1409d2bb5: mov r9b, 0x1d
0x1409d2bb8: call 0x1408ee800
0x1409d2bbd: mov r8d, 0xf8
0x1409d2bc3: mov rcx, r14
0x1409d2bc6: mov rdx, r15
0x1409d2bc9: call 0x142923c00
0x1409d2bce: mov rax, qword ptr [rbp + 0xd08]
0x1409d2bd5: mov qword ptr [rbp + 0xa80], rax
0x1409d2bdc: mov dword ptr [rbp + 0x698], 2
0x1409d2be6: mov qword ptr [rsp + 0x20], r15
0x1409d2beb: mov rcx, rsi
0x1409d2bee: mov rdx, r14
0x1409d2bf1: movabs r8, 0x4b1b8bd236f14df0
0x1409d2bfb: mov r9b, 0x1d
0x1409d2bfe: call 0x1408ee800
0x1409d2c03: mov r8d, 0xf8
0x1409d2c09: mov rcx, r15
0x1409d2c0c: mov rdx, r14
0x1409d2c0f: call 0x142923c00
0x1409d2c14: mov byte ptr [rbp + 0xd57], 1
0x1409d2c1b: mov rcx, r14
0x1409d2c1e: mov rdx, r15
0x1409d2c21: movabs r8, 0xbc5ad80c0ba915c3
0x1409d2c2b: call 0x141630a90
0x1409d2c30: mov byte ptr [rbp + 0xd57], 1
0x1409d2c37: mov rcx, r15
0x1409d2c3a: mov rdx, r14
0x1409d2c3d: movabs r8, 0xc6b33eb20992aad
0x1409d2c47: call 0x14162a790
0x1409d2c4c: mov rax, qword ptr [rbp + 0xe28]
0x1409d2c53: mov qword ptr [rbp + 0xa80], rax
0x1409d2c5a: mov dword ptr [rbp + 0xb78], 2
0x1409d2c64: mov qword ptr [rsp + 0x20], r14
0x1409d2c69: mov rcx, rsi
0x1409d2c6c: mov rdx, r15
0x1409d2c6f: movabs r8, 0x5a67347026116ffe
0x1409d2c79: mov r9b, 0x1d
0x1409d2c7c: call 0x1408ee800
0x1409d2c81: mov r8d, 0xf8
0x1409d2c87: mov rcx, r14
0x1409d2c8a: mov rdx, r15
0x1409d2c8d: call 0x142923c00
0x1409d2c92: mov byte ptr [rbp + 0xd57], 1
0x1409d2c99: mov rcx, r15
0x1409d2c9c: mov rdx, r14
0x1409d2c9f: movabs r8, 0x2bd0ac289087c8e1
0x1409d2ca9: mov r9, qword ptr [rbp + 0xe30]
0x1409d2cb0: call 0x140906150
0x1409d2cb5: mov byte ptr [rbp + 0xd57], 1
0x1409d2cbc: mov rcx, r14
0x1409d2cbf: mov rdx, r15
0x1409d2cc2: movabs r8, 0x129f01ce8d5406a0
0x1409d2ccc: mov r9, qword ptr [rbp + 0xd10]
0x1409d2cd3: call 0x140906150
0x1409d2cd8: mov byte ptr [rbp + 0xd57], 1
0x1409d2cdf: mov rcx, r15
0x1409d2ce2: mov rdx, r14
0x1409d2ce5: call 0x1416286f0
0x1409d2cea: mov byte ptr [rbp + 0xd57], 1
0x1409d2cf1: mov rcx, r14
0x1409d2cf4: mov rdx, r15
0x1409d2cf7: mov r8, qword ptr [rbp + 0xd28]
0x1409d2cfe: call 0x1416289d0
0x1409d2d03: mov byte ptr [rbp + 0xa80], 9
0x1409d2d0a: mov dword ptr [rbp + 0x698], 2
0x1409d2d14: mov qword ptr [rsp + 0x20], r15
0x1409d2d19: mov rcx, rsi
0x1409d2d1c: mov rdx, r14
0x1409d2d1f: movabs r8, 0x61b08b5e0470c5f2
0x1409d2d29: mov r9b, 0x1d
0x1409d2d2c: call 0x140908c50
0x1409d2d31: mov r8d, 0xf8
0x1409d2d37: mov rcx, r15
0x1409d2d3a: mov rdx, r14
0x1409d2d3d: call 0x142923c00
0x1409d2d42: mov byte ptr [rbp + 0x4a8], 0xa
0x1409d2d49: mov dword ptr [rbp + 0xa80], 2
0x1409d2d53: mov qword ptr [rsp + 0x20], rsi
0x1409d2d58: lea rcx, [rbp + 0x4a8]
0x1409d2d5f: mov rdx, r15
0x1409d2d62: movabs r8, 0x297d1e4cdae8f657
0x1409d2d6c: mov r9b, 0x1d
0x1409d2d6f: call 0x140908c50
0x1409d2d74: mov r8d, 0xf8
0x1409d2d7a: lea rdi, [rbp + 0x5a0]
0x1409d2d81: mov rcx, rdi
0x1409d2d84: mov rdx, r15
0x1409d2d87: call 0x142923c00
0x1409d2d8c: mov byte ptr [rbp + 0xb78], 0
0x1409d2d93: mov dword ptr [rbp + 0x698], 2
0x1409d2d9d: mov qword ptr [rsp + 0x20], r15
0x1409d2da2: mov rcx, r14
0x1409d2da5: mov rdx, rdi
0x1409d2da8: movabs r8, 0xb79d1a8a6a254145
0x1409d2db2: mov r9b, 0x1d
0x1409d2db5: call 0x140908c50
0x1409d2dba: mov rdi, qword ptr [rbp + 0x5a0]
0x1409d2dc1: mov r8d, 0xa0
0x1409d2dc7: lea rcx, [rbp + 0x6a0]
0x1409d2dce: lea rbx, [rbp + 0x5a8]
0x1409d2dd5: mov rdx, rbx
0x1409d2dd8: call 0x142923c00
0x1409d2ddd: movzx eax, byte ptr [rbp + 0x648]
0x1409d2de4: movzx ecx, byte ptr [rbp + 0x649]
0x1409d2deb: movzx edx, byte ptr [rbp + 0x64a]
0x1409d2df2: movzx r8d, byte ptr [rbp + 0x64b]
0x1409d2dfa: movups xmm0, xmmword ptr [rbx + 0xe0]
0x1409d2e01: lea r9, [rbp + 0x6c0]
0x1409d2e08: movups xmmword ptr [r9 + 0xc0], xmm0
0x1409d2e10: movups xmm0, xmmword ptr [rbx + 0xa4]
0x1409d2e17: movups xmm1, xmmword ptr [rbx + 0xb4]
0x1409d2e1e: movups xmm2, xmmword ptr [rbx + 0xc4]
0x1409d2e25: movups xmm3, xmmword ptr [rbx + 0xd4]
0x1409d2e2c: movups xmmword ptr [r9 + 0xb4], xmm3
0x1409d2e34: movups xmmword ptr [r9 + 0xa4], xmm2
0x1409d2e3c: movups xmmword ptr [r9 + 0x94], xmm1
0x1409d2e44: movups xmmword ptr [r9 + 0x84], xmm0
0x1409d2e4c: and al, 0x7f
0x1409d2e4e: or cl, 1
0x1409d2e51: and dl, 3
0x1409d2e54: or dl, 0xe8
0x1409d2e57: and r8b, 0xe0
0x1409d2e5b: or r8b, 0x10
0x1409d2e5f: mov qword ptr [rbp + 0x698], rdi
0x1409d2e66: mov byte ptr [rbp + 0x740], al
0x1409d2e6c: mov byte ptr [rbp + 0x741], cl
0x1409d2e72: mov byte ptr [rbp + 0x742], dl
0x1409d2e78: mov byte ptr [rbp + 0x743], r8b
0x1409d2e7f: mov byte ptr [rbp + 0xd57], 1
0x1409d2e86: mov edx, 3
0x1409d2e8b: mov rcx, r15
0x1409d2e8e: call 0x140906a90
0x1409d2e93: mov rax, r12
0x1409d2e96: mov rdx, qword ptr [rbp + 0xd30]
0x1409d2e9d: cmp rax, 5
0x1409d2ea1: jne 0x1409d2480
0x1409d2ea7: lea rsi, [rbp + 0x698]
0x1409d2eae: lea rdx, [rbp - 0x38]
0x1409d2eb2: mov r8d, 0x3e8
0x1409d2eb8: mov rcx, rsi
0x1409d2ebb: call 0x142923c00
0x1409d2ec0: mov byte ptr [rbp + 0xd57], 0
0x1409d2ec7: mov rcx, rsi
0x1409d2eca: call 0x140868370
0x1409d2ecf: movaps xmm6, xmmword ptr [rbp + 0xd60]
0x1409d2ed6: movaps xmm7, xmmword ptr [rbp + 0xd70]
0x1409d2edd: add rsp, 0xe08
0x1409d2ee4: pop rbx
0x1409d2ee5: pop rdi
0x1409d2ee6: pop rsi
0x1409d2ee7: pop r12
0x1409d2ee9: pop r13
0x1409d2eeb: pop r14
0x1409d2eed: pop r15
0x1409d2eef: pop rbp
0x1409d2ef0: ret 
0x1409d2ef1: mov qword ptr [rbp + 0x698], 0
0x1409d2efc: mov byte ptr [rbp + 0xd57], 1
0x1409d2f03: lea rax, [rip + 0x225b276]
0x1409d2f0a: mov qword ptr [rsp + 0x20], rax
0x1409d2f0f: lea rdx, [rbp + 0xb78]
0x1409d2f16: lea r9, [rbp + 0x698]
0x1409d2f1d: xor ecx, ecx
0x1409d2f1f: mov r8, rdi
0x1409d2f22: call 0x1428cab41
0x1409d2f27: jmp 0x1409d2fca
0x1409d2f2c: mov byte ptr [rbp + 0xd57], 1
0x1409d2f33: lea r8, [rip + 0x20ee99e]
0x1409d2f3a: mov ecx, 5
0x1409d2f3f: mov edx, 5
0x1409d2f44: call 0x1428d9518
0x1409d2f49: jmp 0x1409d2fca
0x1409d2f4b: mov byte ptr [rbp + 0xd57], 1
0x1409d2f52: lea rcx, [rip + 0x20ee997]
0x1409d2f59: call 0x1428d9310
0x1409d2f5e: jmp 0x1409d2fca
0x1409d2f60: mov byte ptr [rbp + 0xd57], 1
0x1409d2f67: lea rcx, [rip + 0x2267b32]
0x1409d2f6e: call 0x1428d80d0
0x1409d2f73: jmp 0x1409d2fca
0x1409d2f75: mov byte ptr [rbp + 0xd57], 1
0x1409d2f7c: lea rcx, [rip + 0x2267b1d]
0x1409d2f83: call 0x1428d80d0
0x1409d2f88: jmp 0x1409d2fca
0x1409d2f8a: lea rcx, [rip + 0x20ee8cf]
0x1409d2f91: call 0x1428d9310
0x1409d2f96: lea rcx, [rip + 0x20ee8db]
0x1409d2f9d: call 0x1428d9310
0x1409d2fa2: mov byte ptr [rbp + 0xd57], 1
0x1409d2fa9: lea rcx, [rip + 0x20ee8f8]
0x1409d2fb0: call 0x1428d9310
0x1409d2fb5: jmp 0x1409d2fca
0x1409d2fb7: mov byte ptr [rbp + 0xd57], 1
0x1409d2fbe: lea rcx, [rip + 0x20ee8fb]
0x1409d2fc5: call 0x1428d9310
0x1409d2fca: ud2 
0x1409d2fcc: nop dword ptr [rax]
0x1409d2fd0: mov qword ptr [rsp + 0x10], rdx
0x1409d2fd5: push rbp
0x1409d2fd6: push r15
0x1409d2fd8: push r14
0x1409d2fda: push r13
0x1409d2fdc: push r12
0x1409d2fde: push rsi
0x1409d2fdf: push rdi
0x1409d2fe0: push rbx
0x1409d2fe1: sub rsp, 0x68
0x1409d2fe5: lea rbp, [rdx + 0x80]
0x1409d2fec: movaps xmmword ptr [rsp + 0x40], xmm7
0x1409d2ff1: movaps xmmword ptr [rsp + 0x50], xmm6
0x1409d2ff6: cmp byte ptr [rbp + 0xd56], 0
0x1409d2ffd: je 0x1409d302d
0x1409d2fff: lea rcx, [rbp + 0x698]
0x1409d3006: call 0x1408935c0
0x1409d300b: mov byte ptr [rbp + 0xd57], 1
0x1409d3012: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1409d3017: movaps xmm7, xmmword ptr [rsp + 0x40]
0x1409d301c: add rsp, 0x68
0x1409d3020: pop rbx
0x1409d3021: pop rdi
0x1409d3022: pop rsi
0x1409d3023: pop r12
0x1409d3025: pop r13
0x1409d3027: pop r14
0x1409d3029: pop r15
0x1409d302b: pop rbp
0x1409d302c: ret 
0x1409d302d: mov byte ptr [rbp + 0xd57], 1
0x1409d3034: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1409d3039: movaps xmm7, xmmword ptr [rsp + 0x40]
0x1409d303e: add rsp, 0x68
0x1409d3042: pop rbx
0x1409d3043: pop rdi
0x1409d3044: pop rsi
0x1409d3045: pop r12
0x1409d3047: pop r13
0x1409d3049: pop r14
0x1409d304b: pop r15
0x1409d304d: pop rbp
0x1409d304e: ret 
0x1409d304f: nop 
0x1409d3050: mov qword ptr [rsp + 0x10], rdx
0x1409d3055: push rbp
0x1409d3056: push r15
0x1409d3058: push r14
0x1409d305a: push r13
0x1409d305c: push r12
0x1409d305e: push rsi
0x1409d305f: push rdi
0x1409d3060: push rbx
0x1409d3061: sub rsp, 0x68
0x1409d3065: lea rbp, [rdx + 0x80]
0x1409d306c: movaps xmmword ptr [rsp + 0x40], xmm7
0x1409d3071: movaps xmmword ptr [rsp + 0x50], xmm6
0x1409d3076: cmp byte ptr [rbp + 0xd57], 0
0x1409d307d: jne 0x1409d309a
0x1409d307f: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1409d3084: movaps xmm7, xmmword ptr [rsp + 0x40]
0x1409d3089: add rsp, 0x68
0x1409d308d: pop rbx
0x1409d308e: pop rdi
0x1409d308f: pop rsi
0x1409d3090: pop r12
0x1409d3092: pop r13
0x1409d3094: pop r14
0x1409d3096: pop r15
0x1409d3098: pop rbp
0x1409d3099: ret 
0x1409d309a: lea rcx, [rbp - 0x38]
0x1409d309e: call 0x1408923c0
0x1409d30a3: jmp 0x1409d307f
0x1409d30a5: nop word ptr cs:[rax + rax]
0x1409d30b0: mov qword ptr [rsp + 0x10], rdx
0x1409d30b5: push rbp
0x1409d30b6: push r15
0x1409d30b8: push r14
0x1409d30ba: push r13
0x1409d30bc: push r12
0x1409d30be: push rsi
0x1409d30bf: push rdi
0x1409d30c0: push rbx
0x1409d30c1: sub rsp, 0x68
0x1409d30c5: lea rbp, [rdx + 0x80]
0x1409d30cc: movaps xmmword ptr [rsp + 0x40], xmm7
0x1409d30d1: movaps xmmword ptr [rsp + 0x50], xmm6
0x1409d30d6: mov byte ptr [rbp + 0xd57], 1
0x1409d30dd: lea rcx, [rbp + 0x698]
0x1409d30e4: call 0x1408935c0
0x1409d30e9: mov byte ptr [rbp + 0xd57], 1
0x1409d30f0: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1409d30f5: movaps xmm7, xmmword ptr [rsp + 0x40]
0x1409d30fa: add rsp, 0x68
0x1409d30fe: pop rbx
0x1409d30ff: pop rdi
0x1409d3100: pop rsi
0x1409d3101: pop r12
0x1409d3103: pop r13
0x1409d3105: pop r14
0x1409d3107: pop r15
0x1409d3109: pop rbp
0x1409d310a: ret 
0x1409d310b: nop dword ptr [rax + rax]
0x1409d3110: mov qword ptr [rsp + 0x10], rdx
0x1409d3115: push rbp
0x1409d3116: push r15
0x1409d3118: push r14
0x1409d311a: push r13
0x1409d311c: push r12
0x1409d311e: push rsi
0x1409d311f: push rdi
0x1409d3120: push rbx
0x1409d3121: sub rsp, 0x68
0x1409d3125: lea rbp, [rdx + 0x80]
0x1409d312c: movaps xmmword ptr [rsp + 0x40], xmm7
0x1409d3131: movaps xmmword ptr [rsp + 0x50], xmm6
0x1409d3136: mov byte ptr [rbp + 0xd57], 1
0x1409d313d: lea rcx, [rbp + 0xb78]
0x1409d3144: call 0x1408935c0
0x1409d3149: mov byte ptr [rbp + 0xd57], 1
0x1409d3150: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1409d3155: movaps xmm7, xmmword ptr [rsp + 0x40]
0x1409d315a: add rsp, 0x68
0x1409d315e: pop rbx
0x1409d315f: pop rdi
0x1409d3160: pop rsi
0x1409d3161: pop r12
0x1409d3163: pop r13
0x1409d3165: pop r14
0x1409d3167: pop r15
0x1409d3169: pop rbp
0x1409d316a: ret 
0x1409d316b: nop dword ptr [rax + rax]
0x1409d3170: mov qword ptr [rsp + 0x10], rdx
0x1409d3175: push rbp
0x1409d3176: push r15
0x1409d3178: push r14
0x1409d317a: push r13
0x1409d317c: push r12
0x1409d317e: push rsi
0x1409d317f: push rdi
0x1409d3180: push rbx
0x1409d3181: sub rsp, 0x68
0x1409d3185: lea rbp, [rdx + 0x80]
0x1409d318c: movaps xmmword ptr [rsp + 0x40], xmm7
0x1409d3191: movaps xmmword ptr [rsp + 0x50], xmm6
0x1409d3196: mov byte ptr [rbp + 0xd57], 1
0x1409d319d: lea rcx, [rbp + 0x698]
0x1409d31a4: call 0x1408935c0
0x1409d31a9: mov byte ptr [rbp + 0xd57], 1
0x1409d31b0: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1409d31b5: movaps xmm7, xmmword ptr [rsp + 0x40]
0x1409d31ba: add rsp, 0x68
0x1409d31be: pop rbx
0x1409d31bf: pop rdi
0x1409d31c0: pop rsi
0x1409d31c1: pop r12
0x1409d31c3: pop r13
0x1409d31c5: pop r14
0x1409d31c7: pop r15
0x1409d31c9: pop rbp
0x1409d31ca: ret 
0x1409d31cb: nop dword ptr [rax + rax]
0x1409d31d0: mov qword ptr [rsp + 0x10], rdx
0x1409d31d5: push rbp
0x1409d31d6: push r15
0x1409d31d8: push r14
0x1409d31da: push r13
0x1409d31dc: push r12
0x1409d31de: push rsi
0x1409d31df: push rdi
0x1409d31e0: push rbx
0x1409d31e1: sub rsp, 0x68
0x1409d31e5: lea rbp, [rdx + 0x80]
0x1409d31ec: movaps xmmword ptr [rsp + 0x40], xmm7
0x1409d31f1: movaps xmmword ptr [rsp + 0x50], xmm6
0x1409d31f6: mov byte ptr [rbp + 0xd57], 1
0x1409d31fd: lea rcx, [rbp + 0xb78]
0x1409d3204: call 0x1408935c0
0x1409d3209: mov byte ptr [rbp + 0xd57], 1
0x1409d3210: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1409d3215: movaps xmm7, xmmword ptr [rsp + 0x40]
0x1409d321a: add rsp, 0x68
0x1409d321e: pop rbx
0x1409d321f: pop rdi
0x1409d3220: pop rsi
0x1409d3221: pop r12
0x1409d3223: pop r13
0x1409d3225: pop r14
0x1409d3227: pop r15
0x1409d3229: pop rbp
0x1409d322a: ret 
0x1409d322b: nop dword ptr [rax + rax]
0x1409d3230: mov qword ptr [rsp + 0x10], rdx
0x1409d3235: push rbp
0x1409d3236: push r15
0x1409d3238: push r14
0x1409d323a: push r13
0x1409d323c: push r12
0x1409d323e: push rsi
0x1409d323f: push rdi
0x1409d3240: push rbx
0x1409d3241: sub rsp, 0x68
0x1409d3245: lea rbp, [rdx + 0x80]
0x1409d324c: movaps xmmword ptr [rsp + 0x40], xmm7
0x1409d3251: movaps xmmword ptr [rsp + 0x50], xmm6
0x1409d3256: mov byte ptr [rbp + 0xd57], 1
0x1409d325d: lea rcx, [rbp + 0x698]
0x1409d3264: call 0x1408935c0
0x1409d3269: mov byte ptr [rbp + 0xd57], 1
0x1409d3270: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1409d3275: movaps xmm7, xmmword ptr [rsp + 0x40]
0x1409d327a: add rsp, 0x68
0x1409d327e: pop rbx
0x1409d327f: pop rdi
0x1409d3280: pop rsi
0x1409d3281: pop r12
0x1409d3283: pop r13
0x1409d3285: pop r14
0x1409d3287: pop r15
0x1409d3289: pop rbp
0x1409d328a: ret 
0x1409d328b: nop dword ptr [rax + rax]
0x1409d3290: mov qword ptr [rsp + 0x10], rdx
0x1409d3295: push rbp
0x1409d3296: push r15
0x1409d3298: push r14
0x1409d329a: push r13
0x1409d329c: push r12
0x1409d329e: push rsi
0x1409d329f: push rdi
0x1409d32a0: push rbx
0x1409d32a1: sub rsp, 0x68
0x1409d32a5: lea rbp, [rdx + 0x80]
0x1409d32ac: movaps xmmword ptr [rsp + 0x40], xmm7
0x1409d32b1: movaps xmmword ptr [rsp + 0x50], xmm6
0x1409d32b6: mov byte ptr [rbp + 0xd57], 1
0x1409d32bd: lea rcx, [rbp + 0xb78]
0x1409d32c4: call 0x1408935c0
0x1409d32c9: mov byte ptr [rbp + 0xd57], 1
0x1409d32d0: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1409d32d5: movaps xmm7, xmmword ptr [rsp + 0x40]
0x1409d32da: add rsp, 0x68
0x1409d32de: pop rbx
0x1409d32df: pop rdi
0x1409d32e0: pop rsi
0x1409d32e1: pop r12
0x1409d32e3: pop r13
0x1409d32e5: pop r14
0x1409d32e7: pop r15
0x1409d32e9: pop rbp
0x1409d32ea: ret 
0x1409d32eb: nop dword ptr [rax + rax]
0x1409d32f0: mov qword ptr [rsp + 0x10], rdx
0x1409d32f5: push rbp
0x1409d32f6: push r15
0x1409d32f8: push r14
0x1409d32fa: push r13
0x1409d32fc: push r12
0x1409d32fe: push rsi
0x1409d32ff: push rdi
0x1409d3300: push rbx
0x1409d3301: sub rsp, 0x68
0x1409d3305: lea rbp, [rdx + 0x80]
0x1409d330c: movaps xmmword ptr [rsp + 0x40], xmm7
0x1409d3311: movaps xmmword ptr [rsp + 0x50], xmm6
0x1409d3316: mov byte ptr [rbp + 0xd57], 1
0x1409d331d: lea rcx, [rbp + 0x698]
0x1409d3324: call 0x1408935c0
0x1409d3329: mov byte ptr [rbp + 0xd57], 1
0x1409d3330: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1409d3335: movaps xmm7, xmmword ptr [rsp + 0x40]
0x1409d333a: add rsp, 0x68
0x1409d333e: pop rbx
0x1409d333f: pop rdi
0x1409d3340: pop rsi
0x1409d3341: pop r12
0x1409d3343: pop r13
0x1409d3345: pop r14
0x1409d3347: pop r15
0x1409d3349: pop rbp
0x1409d334a: ret 
0x1409d334b: nop dword ptr [rax + rax]
0x1409d3350: mov qword ptr [rsp + 0x10], rdx
0x1409d3355: push rbp
0x1409d3356: push r15
0x1409d3358: push r14
0x1409d335a: push r13
0x1409d335c: push r12
0x1409d335e: push rsi
0x1409d335f: push rdi
0x1409d3360: push rbx
0x1409d3361: sub rsp, 0x68
0x1409d3365: lea rbp, [rdx + 0x80]
0x1409d336c: movaps xmmword ptr [rsp + 0x40], xmm7
0x1409d3371: movaps xmmword ptr [rsp + 0x50], xmm6
0x1409d3376: mov byte ptr [rbp + 0xd57], 1
0x1409d337d: lea rcx, [rbp + 0xb78]
0x1409d3384: call 0x1408935c0
0x1409d3389: mov byte ptr [rbp + 0xd57], 1
0x1409d3390: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1409d3395: movaps xmm7, xmmword ptr [rsp + 0x40]
0x1409d339a: add rsp, 0x68
0x1409d339e: pop rbx
0x1409d339f: pop rdi
0x1409d33a0: pop rsi
0x1409d33a1: pop r12
0x1409d33a3: pop r13
0x1409d33a5: pop r14
0x1409d33a7: pop r15
0x1409d33a9: pop rbp
0x1409d33aa: ret 
0x1409d33ab: nop dword ptr [rax + rax]
0x1409d33b0: mov qword ptr [rsp + 0x10], rdx
0x1409d33b5: push rbp
0x1409d33b6: push r15
0x1409d33b8: push r14
0x1409d33ba: push r13
0x1409d33bc: push r12
0x1409d33be: push rsi
0x1409d33bf: push rdi
0x1409d33c0: push rbx
0x1409d33c1: sub rsp, 0x68
0x1409d33c5: lea rbp, [rdx + 0x80]
0x1409d33cc: movaps xmmword ptr [rsp + 0x40], xmm7
0x1409d33d1: movaps xmmword ptr [rsp + 0x50], xmm6
0x1409d33d6: mov byte ptr [rbp + 0xd57], 1
0x1409d33dd: lea rcx, [rbp + 0x698]
0x1409d33e4: call 0x1408935c0
0x1409d33e9: mov byte ptr [rbp + 0xd57], 1
0x1409d33f0: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1409d33f5: movaps xmm7, xmmword ptr [rsp + 0x40]
0x1409d33fa: add rsp, 0x68
0x1409d33fe: pop rbx
0x1409d33ff: pop rdi
0x1409d3400: pop rsi
0x1409d3401: pop r12
0x1409d3403: pop r13
0x1409d3405: pop r14
0x1409d3407: pop r15
0x1409d3409: pop rbp
0x1409d340a: ret 
0x1409d340b: nop dword ptr [rax + rax]
0x1409d3410: mov qword ptr [rsp + 0x10], rdx
0x1409d3415: push rbp
0x1409d3416: push r15
0x1409d3418: push r14
0x1409d341a: push r13
0x1409d341c: push r12
0x1409d341e: push rsi
0x1409d341f: push rdi
0x1409d3420: push rbx
0x1409d3421: sub rsp, 0x68
0x1409d3425: lea rbp, [rdx + 0x80]
0x1409d342c: movaps xmmword ptr [rsp + 0x40], xmm7
0x1409d3431: movaps xmmword ptr [rsp + 0x50], xmm6
0x1409d3436: mov byte ptr [rbp + 0xd57], 1
0x1409d343d: lea rcx, [rbp + 0xa80]
0x1409d3444: call 0x1408935c0
0x1409d3449: mov byte ptr [rbp + 0xd57], 1
0x1409d3450: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1409d3455: movaps xmm7, xmmword ptr [rsp + 0x40]
0x1409d345a: add rsp, 0x68
0x1409d345e: pop rbx
0x1409d345f: pop rdi
0x1409d3460: pop rsi
0x1409d3461: pop r12
0x1409d3463: pop r13
0x1409d3465: pop r14
0x1409d3467: pop r15
0x1409d3469: pop rbp
0x1409d346a: ret 
0x1409d346b: nop dword ptr [rax + rax]
0x1409d3470: mov qword ptr [rsp + 0x10], rdx
0x1409d3475: push rbp
0x1409d3476: push r15
0x1409d3478: push r14
0x1409d347a: push r13
0x1409d347c: push r12
0x1409d347e: push rsi
0x1409d347f: push rdi
0x1409d3480: push rbx
0x1409d3481: sub rsp, 0x68
0x1409d3485: lea rbp, [rdx + 0x80]
0x1409d348c: movaps xmmword ptr [rsp + 0x40], xmm7
0x1409d3491: movaps xmmword ptr [rsp + 0x50], xmm6
0x1409d3496: mov byte ptr [rbp + 0xd57], 1
0x1409d349d: lea rcx, [rbp + 0x698]
0x1409d34a4: call 0x1408935c0
0x1409d34a9: mov byte ptr [rbp + 0xd57], 1
0x1409d34b0: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1409d34b5: movaps xmm7, xmmword ptr [rsp + 0x40]
0x1409d34ba: add rsp, 0x68
0x1409d34be: pop rbx
0x1409d34bf: pop rdi
0x1409d34c0: pop rsi
0x1409d34c1: pop r12
0x1409d34c3: pop r13
0x1409d34c5: pop r14
0x1409d34c7: pop r15
0x1409d34c9: pop rbp
0x1409d34ca: ret 
0x1409d34cb: nop dword ptr [rax + rax]
0x1409d34d0: mov qword ptr [rsp + 0x10], rdx
0x1409d34d5: push rbp
0x1409d34d6: push r15
0x1409d34d8: push r14
0x1409d34da: push r13
0x1409d34dc: push r12
0x1409d34de: push rsi
0x1409d34df: push rdi
0x1409d34e0: push rbx
0x1409d34e1: sub rsp, 0x68
0x1409d34e5: lea rbp, [rdx + 0x80]
0x1409d34ec: movaps xmmword ptr [rsp + 0x40], xmm7
0x1409d34f1: movaps xmmword ptr [rsp + 0x50], xmm6
0x1409d34f6: mov byte ptr [rbp + 0xd57], 1
0x1409d34fd: lea rcx, [rbp + 0xb78]
0x1409d3504: call 0x1408935c0
0x1409d3509: mov byte ptr [rbp + 0xd57], 1
0x1409d3510: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1409d3515: movaps xmm7, xmmword ptr [rsp + 0x40]
0x1409d351a: add rsp, 0x68
0x1409d351e: pop rbx
0x1409d351f: pop rdi
0x1409d3520: pop rsi
0x1409d3521: pop r12
0x1409d3523: pop r13
0x1409d3525: pop r14
0x1409d3527: pop r15
0x1409d3529: pop rbp
0x1409d352a: ret 
0x1409d352b: nop dword ptr [rax + rax]
0x1409d3530: mov qword ptr [rsp + 0x10], rdx
0x1409d3535: push rbp
0x1409d3536: push r15
0x1409d3538: push r14
0x1409d353a: push r13
0x1409d353c: push r12
0x1409d353e: push rsi
0x1409d353f: push rdi
0x1409d3540: push rbx
0x1409d3541: sub rsp, 0x68
0x1409d3545: lea rbp, [rdx + 0x80]
0x1409d354c: movaps xmmword ptr [rsp + 0x40], xmm7
0x1409d3551: movaps xmmword ptr [rsp + 0x50], xmm6
0x1409d3556: mov byte ptr [rbp + 0xd57], 1
0x1409d355d: lea rcx, [rbp + 0x4a8]
0x1409d3564: call 0x1408935c0
0x1409d3569: mov byte ptr [rbp + 0xd57], 1
0x1409d3570: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1409d3575: movaps xmm7, xmmword ptr [rsp + 0x40]
0x1409d357a: add rsp, 0x68
0x1409d357e: pop rbx
0x1409d357f: pop rdi
0x1409d3580: pop rsi
0x1409d3581: pop r12
0x1409d3583: pop r13
0x1409d3585: pop r14
0x1409d3587: pop r15
0x1409d3589: pop rbp
0x1409d358a: ret 
0x1409d358b: nop dword ptr [rax + rax]
0x1409d3590: mov qword ptr [rsp + 0x10], rdx
0x1409d3595: push rbp
0x1409d3596: push r15
0x1409d3598: push r14
0x1409d359a: push r13
0x1409d359c: push r12
0x1409d359e: push rsi
0x1409d359f: push rdi
0x1409d35a0: push rbx
0x1409d35a1: sub rsp, 0x68
0x1409d35a5: lea rbp, [rdx + 0x80]
0x1409d35ac: movaps xmmword ptr [rsp + 0x40], xmm7
0x1409d35b1: movaps xmmword ptr [rsp + 0x50], xmm6
0x1409d35b6: mov byte ptr [rbp + 0xd57], 1
0x1409d35bd: lea rcx, [rbp + 0x3b0]
0x1409d35c4: call 0x1408935c0
0x1409d35c9: mov byte ptr [rbp + 0xd57], 1
0x1409d35d0: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1409d35d5: movaps xmm7, xmmword ptr [rsp + 0x40]
0x1409d35da: add rsp, 0x68
0x1409d35de: pop rbx
0x1409d35df: pop rdi
0x1409d35e0: pop rsi
0x1409d35e1: pop r12
0x1409d35e3: pop r13
0x1409d35e5: pop r14
0x1409d35e7: pop r15
0x1409d35e9: pop rbp
0x1409d35ea: ret 
0x1409d35eb: nop dword ptr [rax + rax]
0x1409d35f0: mov qword ptr [rsp + 0x10], rdx
0x1409d35f5: push rbp
0x1409d35f6: push r15
0x1409d35f8: push r14
0x1409d35fa: push r13
0x1409d35fc: push r12
0x1409d35fe: push rsi
0x1409d35ff: push rdi
0x1409d3600: push rbx
0x1409d3601: sub rsp, 0x68
0x1409d3605: lea rbp, [rdx + 0x80]
0x1409d360c: movaps xmmword ptr [rsp + 0x40], xmm7
0x1409d3611: movaps xmmword ptr [rsp + 0x50], xmm6
0x1409d3616: mov byte ptr [rbp + 0xd57], 1
0x1409d361d: lea rcx, [rbp + 0x698]
0x1409d3624: call 0x1408935c0
0x1409d3629: mov byte ptr [rbp + 0xd57], 1
0x1409d3630: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1409d3635: movaps xmm7, xmmword ptr [rsp + 0x40]
0x1409d363a: add rsp, 0x68
0x1409d363e: pop rbx
0x1409d363f: pop rdi
0x1409d3640: pop rsi
0x1409d3641: pop r12
0x1409d3643: pop r13
0x1409d3645: pop r14
0x1409d3647: pop r15
0x1409d3649: pop rbp
0x1409d364a: ret 
0x1409d364b: nop dword ptr [rax + rax]
0x1409d3650: mov qword ptr [rsp + 0x10], rdx
0x1409d3655: push rbp
0x1409d3656: push r15
0x1409d3658: push r14
0x1409d365a: push r13
0x1409d365c: push r12
0x1409d365e: push rsi
0x1409d365f: push rdi
0x1409d3660: push rbx
0x1409d3661: sub rsp, 0x68
0x1409d3665: lea rbp, [rdx + 0x80]
0x1409d366c: movaps xmmword ptr [rsp + 0x40], xmm7
0x1409d3671: movaps xmmword ptr [rsp + 0x50], xmm6
0x1409d3676: mov byte ptr [rbp + 0xd57], 1
0x1409d367d: lea rcx, [rbp + 0xb78]
0x1409d3684: call 0x1408935c0
0x1409d3689: mov byte ptr [rbp + 0xd57], 1
0x1409d3690: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1409d3695: movaps xmm7, xmmword ptr [rsp + 0x40]
0x1409d369a: add rsp, 0x68
0x1409d369e: pop rbx
0x1409d369f: pop rdi
0x1409d36a0: pop rsi
0x1409d36a1: pop r12
0x1409d36a3: pop r13
0x1409d36a5: pop r14
0x1409d36a7: pop r15
0x1409d36a9: pop rbp
0x1409d36aa: ret 
0x1409d36ab: nop dword ptr [rax + rax]
0x1409d36b0: mov qword ptr [rsp + 0x10], rdx
0x1409d36b5: push rbp
0x1409d36b6: push r15
0x1409d36b8: push r14
0x1409d36ba: push r13
0x1409d36bc: push r12
0x1409d36be: push rsi
0x1409d36bf: push rdi
0x1409d36c0: push rbx
0x1409d36c1: sub rsp, 0x68
0x1409d36c5: lea rbp, [rdx + 0x80]
0x1409d36cc: movaps xmmword ptr [rsp + 0x40], xmm7
0x1409d36d1: movaps xmmword ptr [rsp + 0x50], xmm6
0x1409d36d6: mov byte ptr [rbp + 0xd57], 1
0x1409d36dd: lea rcx, [rbp + 0x698]
0x1409d36e4: call 0x1408935c0
0x1409d36e9: mov byte ptr [rbp + 0xd57], 1
0x1409d36f0: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1409d36f5: movaps xmm7, xmmword ptr [rsp + 0x40]
0x1409d36fa: add rsp, 0x68
0x1409d36fe: pop rbx
0x1409d36ff: pop rdi
0x1409d3700: pop rsi
0x1409d3701: pop r12
0x1409d3703: pop r13
0x1409d3705: pop r14
0x1409d3707: pop r15
0x1409d3709: pop rbp
0x1409d370a: ret 
0x1409d370b: nop dword ptr [rax + rax]
0x1409d3710: mov qword ptr [rsp + 0x10], rdx
0x1409d3715: push rbp
0x1409d3716: push r15
0x1409d3718: push r14
0x1409d371a: push r13
0x1409d371c: push r12
0x1409d371e: push rsi
0x1409d371f: push rdi
0x1409d3720: push rbx
0x1409d3721: sub rsp, 0x68
0x1409d3725: lea rbp, [rdx + 0x80]
0x1409d372c: movaps xmmword ptr [rsp + 0x40], xmm7
0x1409d3731: movaps xmmword ptr [rsp + 0x50], xmm6
0x1409d3736: mov byte ptr [rbp + 0xd57], 1
0x1409d373d: lea rcx, [rbp + 0xb78]
0x1409d3744: call 0x1408935c0
0x1409d3749: mov byte ptr [rbp + 0xd57], 1
0x1409d3750: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1409d3755: movaps xmm7, xmmword ptr [rsp + 0x40]
0x1409d375a: add rsp, 0x68
0x1409d375e: pop rbx
0x1409d375f: pop rdi
0x1409d3760: pop rsi
0x1409d3761: pop r12
0x1409d3763: pop r13
0x1409d3765: pop r14
0x1409d3767: pop r15
0x1409d3769: pop rbp
0x1409d376a: ret 
0x1409d376b: nop dword ptr [rax + rax]
0x1409d3770: mov qword ptr [rsp + 0x10], rdx
0x1409d3775: push rbp
0x1409d3776: push r15
0x1409d3778: push r14
0x1409d377a: push r13
0x1409d377c: push r12
0x1409d377e: push rsi
0x1409d377f: push rdi
0x1409d3780: push rbx
0x1409d3781: sub rsp, 0x68
0x1409d3785: lea rbp, [rdx + 0x80]
0x1409d378c: movaps xmmword ptr [rsp + 0x40], xmm7
0x1409d3791: movaps xmmword ptr [rsp + 0x50], xmm6
0x1409d3796: mov byte ptr [rbp + 0xd57], 1
0x1409d379d: lea rcx, [rbp + 0x698]
0x1409d37a4: call 0x1408935c0
0x1409d37a9: mov byte ptr [rbp + 0xd57], 1
0x1409d37b0: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1409d37b5: movaps xmm7, xmmword ptr [rsp + 0x40]
0x1409d37ba: add rsp, 0x68
0x1409d37be: pop rbx
0x1409d37bf: pop rdi
0x1409d37c0: pop rsi
0x1409d37c1: pop r12
0x1409d37c3: pop r13
0x1409d37c5: pop r14
0x1409d37c7: pop r15
0x1409d37c9: pop rbp
0x1409d37ca: ret 
0x1409d37cb: nop dword ptr [rax + rax]
0x1409d37d0: mov qword ptr [rsp + 0x10], rdx
0x1409d37d5: push rbp
0x1409d37d6: push r15
0x1409d37d8: push r14
0x1409d37da: push r13
0x1409d37dc: push r12
0x1409d37de: push rsi
0x1409d37df: push rdi
0x1409d37e0: push rbx
0x1409d37e1: sub rsp, 0x68
0x1409d37e5: lea rbp, [rdx + 0x80]
0x1409d37ec: movaps xmmword ptr [rsp + 0x40], xmm7
0x1409d37f1: movaps xmmword ptr [rsp + 0x50], xmm6
0x1409d37f6: mov byte ptr [rbp + 0xd57], 1
0x1409d37fd: lea rcx, [rbp + 0xb78]
0x1409d3804: call 0x1408935c0
0x1409d3809: mov byte ptr [rbp + 0xd57], 1
0x1409d3810: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1409d3815: movaps xmm7, xmmword ptr [rsp + 0x40]
0x1409d381a: add rsp, 0x68
0x1409d381e: pop rbx
0x1409d381f: pop rdi
0x1409d3820: pop rsi
0x1409d3821: pop r12
0x1409d3823: pop r13
0x1409d3825: pop r14
0x1409d3827: pop r15
0x1409d3829: pop rbp
0x1409d382a: ret 
0x1409d382b: nop dword ptr [rax + rax]
0x1409d3830: mov qword ptr [rsp + 0x10], rdx
0x1409d3835: push rbp
0x1409d3836: push r15
0x1409d3838: push r14
0x1409d383a: push r13
0x1409d383c: push r12
0x1409d383e: push rsi
0x1409d383f: push rdi
0x1409d3840: push rbx
0x1409d3841: sub rsp, 0x68
0x1409d3845: lea rbp, [rdx + 0x80]
0x1409d384c: movaps xmmword ptr [rsp + 0x40], xmm7
0x1409d3851: movaps xmmword ptr [rsp + 0x50], xmm6
0x1409d3856: mov byte ptr [rbp + 0xd57], 1
0x1409d385d: lea rcx, [rbp + 0x698]
0x1409d3864: call 0x1408935c0
0x1409d3869: mov byte ptr [rbp + 0xd57], 1
0x1409d3870: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1409d3875: movaps xmm7, xmmword ptr [rsp + 0x40]
0x1409d387a: add rsp, 0x68
0x1409d387e: pop rbx
0x1409d387f: pop rdi
0x1409d3880: pop rsi
0x1409d3881: pop r12
0x1409d3883: pop r13
0x1409d3885: pop r14
0x1409d3887: pop r15
0x1409d3889: pop rbp
0x1409d388a: ret 
0x1409d388b: nop dword ptr [rax + rax]
0x1409d3890: mov qword ptr [rsp + 0x10], rdx
0x1409d3895: push rbp
0x1409d3896: push r15
0x1409d3898: push r14
0x1409d389a: push r13
0x1409d389c: push r12
0x1409d389e: push rsi
0x1409d389f: push rdi
0x1409d38a0: push rbx
0x1409d38a1: sub rsp, 0x68
0x1409d38a5: lea rbp, [rdx + 0x80]
0x1409d38ac: movaps xmmword ptr [rsp + 0x40], xmm7
0x1409d38b1: movaps xmmword ptr [rsp + 0x50], xmm6
0x1409d38b6: mov byte ptr [rbp + 0xd57], 1
0x1409d38bd: lea rcx, [rbp + 0x5a0]
0x1409d38c4: call 0x1408935c0
0x1409d38c9: mov byte ptr [rbp + 0xd57], 1
0x1409d38d0: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1409d38d5: movaps xmm7, xmmword ptr [rsp + 0x40]
0x1409d38da: add rsp, 0x68
0x1409d38de: pop rbx
0x1409d38df: pop rdi
0x1409d38e0: pop rsi
0x1409d38e1: pop r12
0x1409d38e3: pop r13
0x1409d38e5: pop r14
0x1409d38e7: pop r15
0x1409d38e9: pop rbp
0x1409d38ea: ret 
