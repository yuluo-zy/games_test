1407e2510: 4157 push r15
1407e2512: 4156 push r14
1407e2514: 4155 push r13
1407e2516: 4154 push r12
1407e2518: 56 push rsi
1407e2519: 57 push rdi
1407e251a: 55 push rbp
1407e251b: 53 push rbx
1407e251c: 4881ecb8030000 sub rsp, 0x3b8
1407e2523: 440f29bc24a0030000 movaps xmmword ptr [rsp + 0x3a0], xmm15
1407e252c: 440f29b42490030000 movaps xmmword ptr [rsp + 0x390], xmm14
1407e2535: 440f29ac2480030000 movaps xmmword ptr [rsp + 0x380], xmm13
1407e253e: 66440f7fa42470030000 movdqa xmmword ptr [rsp + 0x370], xmm12
1407e2548: 440f299c2460030000 movaps xmmword ptr [rsp + 0x360], xmm11
1407e2551: 440f29942450030000 movaps xmmword ptr [rsp + 0x350], xmm10
1407e255a: 440f298c2440030000 movaps xmmword ptr [rsp + 0x340], xmm9
1407e2563: 440f29842430030000 movaps xmmword ptr [rsp + 0x330], xmm8
1407e256c: 0f29bc2420030000 movaps xmmword ptr [rsp + 0x320], xmm7
1407e2574: 0f29b42410030000 movaps xmmword ptr [rsp + 0x310], xmm6
1407e257c: 44884c2444 mov byte ptr [rsp + 0x44], r9b
1407e2581: 4889942480000000 mov qword ptr [rsp + 0x80], rdx
1407e2589: 48894c2478 mov qword ptr [rsp + 0x78], rcx
1407e258e: 488bb42468040000 mov rsi, qword ptr [rsp + 0x468]
1407e2596: 488bbc2460040000 mov rdi, qword ptr [rsp + 0x460]
1407e259e: f3440f10842458040000 movss xmm8, dword ptr [rsp + 0x458]
1407e25a8: 488b842430040000 mov rax, qword ptr [rsp + 0x430]
1407e25b0: 488b8c2428040000 mov rcx, qword ptr [rsp + 0x428]
1407e25b8: f3440f104918 movss xmm9, dword ptr [rcx + 0x18]
1407e25be: f20f104910 movsd xmm1, qword ptr [rcx + 0x10]
1407e25c3: f20f105810 movsd xmm3, qword ptr [rax + 0x10]
1407e25c8: f3440f105018 movss xmm10, dword ptr [rax + 0x18]
1407e25ce: 410f28c1 movaps xmm0, xmm9
1407e25d2: f3410f5cc2 subss xmm0, xmm10
1407e25d7: 0f298c2420010000 movaps xmmword ptr [rsp + 0x120], xmm1
1407e25df: 0f299c2410010000 movaps xmmword ptr [rsp + 0x110], xmm3
1407e25e7: 0f5ccb subps xmm1, xmm3
1407e25ea: 0f59c9 mulps xmm1, xmm1
1407e25ed: f30f16d9 movshdup xmm3, xmm1
1407e25f1: f30f58d9 addss xmm3, xmm1
1407e25f5: f30f59c0 mulss xmm0, xmm0
1407e25f9: f30f58c3 addss xmm0, xmm3
1407e25fd: 450f57db xorps xmm11, xmm11
1407e2601: f3440f51d8 sqrtss xmm11, xmm0
1407e2606: 410f28c3 movaps xmm0, xmm11
1407e260a: f30f5ec2 divss xmm0, xmm2
1407e260e: e84d181402 call 0x142923e60
1407e2613: f3480f2cc0 cvttss2si rax, xmm0
1407e2618: 4889c1 mov rcx, rax
1407e261b: 48c1f93f sar rcx, 0x3f
1407e261f: 0f28c8 movaps xmm1, xmm0
1407e2622: f30f5c0db27a1702 subss xmm1, dword ptr [rip + 0x2177ab2]
1407e262a: f3480f2cd1 cvttss2si rdx, xmm1
1407e262f: 4821ca and rdx, rcx
1407e2632: 4809c2 or rdx, rax
1407e2635: 4531ed xor r13d, r13d
1407e2638: 450f57ed xorps xmm13, xmm13
1407e263c: 410f2ec5 ucomiss xmm0, xmm13
1407e2640: 490f42d5 cmovb rdx, r13
1407e2644: 0f2e05957a1702 ucomiss xmm0, dword ptr [rip + 0x2177a95]
1407e264b: 48c7c0ffffffff mov rax, 0xffffffffffffffff
1407e2652: 480f46c2 cmovbe rax, rdx
1407e2656: 4883f803 cmp rax, 3
1407e265a: 41be02000000 mov r14d, 2
1407e2660: 4c0f43f0 cmovae r14, rax
1407e2664: f3450f5ec3 divss xmm8, xmm11
1407e2669: 0f57c9 xorps xmm1, xmm1
1407e266c: f3410f5fc8 maxss xmm1, xmm8
1407e2671: f30f1005d7321402 movss xmm0, dword ptr [rip + 0x21432d7]
1407e2679: f30f5dc1 minss xmm0, xmm1
1407e267d: 4d85f6 test r14, r14
1407e2680: 780a js 0x1407e268c
1407e2682: 0f57c9 xorps xmm1, xmm1
1407e2685: f3490f2ace cvtsi2ss xmm1, r14
1407e268a: eb1b jmp 0x1407e26a7
1407e268c: 4c89f0 mov rax, r14
1407e268f: 48d1e8 shr rax, 1
1407e2692: 4489f1 mov ecx, r14d
1407e2695: 83e101 and ecx, 1
1407e2698: 4809c1 or rcx, rax
1407e269b: 0f57c9 xorps xmm1, xmm1
1407e269e: f3480f2ac9 cvtsi2ss xmm1, rcx
1407e26a3: f30f58c9 addss xmm1, xmm1
1407e26a7: 0fb607 movzx eax, byte ptr [rdi]
1407e26aa: 88442443 mov byte ptr [rsp + 0x43], al
1407e26ae: 8b4704 mov eax, dword ptr [rdi + 4]
1407e26b1: 89442454 mov dword ptr [rsp + 0x54], eax
1407e26b5: 8b4708 mov eax, dword ptr [rdi + 8]
1407e26b8: 89442450 mov dword ptr [rsp + 0x50], eax
1407e26bc: 8b470c mov eax, dword ptr [rdi + 0xc]
1407e26bf: 8944244c mov dword ptr [rsp + 0x4c], eax
1407e26c3: 8b4710 mov eax, dword ptr [rdi + 0x10]
1407e26c6: 89442448 mov dword ptr [rsp + 0x48], eax
1407e26ca: 488b06 mov rax, qword ptr [rsi]
1407e26cd: 4889442470 mov qword ptr [rsp + 0x70], rax
1407e26d2: 488b4610 mov rax, qword ptr [rsi + 0x10]
1407e26d6: 4883c048 add rax, 0x48
1407e26da: 4889442468 mov qword ptr [rsp + 0x68], rax
1407e26df: 488d9c2484010000 lea rbx, [rsp + 0x184]
1407e26e7: f3410f12d2 movsldup xmm2, xmm10
1407e26ec: 0f29942400010000 movaps xmmword ptr [rsp + 0x100], xmm2
1407e26f4: f30f12c9 movsldup xmm1, xmm1
1407e26f8: 0f298c24f0000000 movaps xmmword ptr [rsp + 0xf0], xmm1
1407e2700: f30f12c0 movsldup xmm0, xmm0
1407e2704: 0f298424e0000000 movaps xmmword ptr [rsp + 0xe0], xmm0
1407e270c: f3410f12c1 movsldup xmm0, xmm9
1407e2711: 0f298424d0000000 movaps xmmword ptr [rsp + 0xd0], xmm0
1407e2719: f3410f12c3 movsldup xmm0, xmm11
1407e271e: 0f298424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm0
1407e2726: f3440f103dd5311402 movss xmm15, dword ptr [rip + 0x21431d5]
1407e272f: 66450fefe4 pxor xmm12, xmm12
1407e2734: 31ff xor edi, edi
1407e2736: e9d0020000 jmp 0x1407e2a0b
1407e273b: 0f1f440000 nop dword ptr [rax + rax]
1407e2740: 40f6c701 test dil, 1
1407e2744: 488b442458 mov rax, qword ptr [rsp + 0x58]
1407e2749: 488b4c245f mov rcx, qword ptr [rsp + 0x5f]
1407e274e: 448b7c244c mov r15d, dword ptr [rsp + 0x4c]
1407e2753: 440f447c2454 cmove r15d, dword ptr [rsp + 0x54]
1407e2759: 48898c2457010000 mov qword ptr [rsp + 0x157], rcx
1407e2761: 8b742448 mov esi, dword ptr [rsp + 0x48]
1407e2765: 0f44742450 cmove esi, dword ptr [rsp + 0x50]
1407e276a: 4889842450010000 mov qword ptr [rsp + 0x150], rax
1407e2772: 488b842450010000 mov rax, qword ptr [rsp + 0x150]
1407e277a: 488b8c2457010000 mov rcx, qword ptr [rsp + 0x157]
1407e2782: 4889442458 mov qword ptr [rsp + 0x58], rax
1407e2787: 48894c245f mov qword ptr [rsp + 0x5f], rcx
1407e278c: b90a000000 mov ecx, 0xa
1407e2791: b801000000 mov eax, 1
1407e2796: 488d15db642a02 lea rdx, [rip + 0x22a64db]
1407e279d: 48c784245001000000000000 mov qword ptr [rsp + 0x150], 0
1407e27a9: 48c784246001000000000000 mov qword ptr [rsp + 0x160], 0
1407e27b5: 48c784247001000000000000 mov qword ptr [rsp + 0x170], 0
1407e27c1: c784248001000000000000 mov dword ptr [rsp + 0x180], 0
1407e27cc: 448b8424ac000000 mov r8d, dword ptr [rsp + 0xac]
1407e27d4: 44894308 mov dword ptr [rbx + 8], r8d
1407e27d8: 4c8b8424a4000000 mov r8, qword ptr [rsp + 0xa4]
1407e27e0: 4c8903 mov qword ptr [rbx], r8
1407e27e3: c784249001000000000000 mov dword ptr [rsp + 0x190], 0
1407e27ee: 0f10a424b4020000 movups xmm4, xmmword ptr [rsp + 0x2b4]
1407e27f6: 0f10ac24c4020000 movups xmm5, xmmword ptr [rsp + 0x2c4]
1407e27fe: 0f10b424d4020000 movups xmm6, xmmword ptr [rsp + 0x2d4]
1407e2806: 0f10bc24e0020000 movups xmm7, xmmword ptr [rsp + 0x2e0]
1407e280e: 0f117b3c movups xmmword ptr [rbx + 0x3c], xmm7
1407e2812: 0f117330 movups xmmword ptr [rbx + 0x30], xmm6
1407e2816: 0f116b20 movups xmmword ptr [rbx + 0x20], xmm5
1407e281a: 0f116310 movups xmmword ptr [rbx + 0x10], xmm4
1407e281e: c78424d001000000000000 mov dword ptr [rsp + 0x1d0], 0
1407e2829: 448b8424a0000000 mov r8d, dword ptr [rsp + 0xa0]
1407e2831: 44894358 mov dword ptr [rbx + 0x58], r8d
1407e2835: 4c8b842498000000 mov r8, qword ptr [rsp + 0x98]
1407e283d: 4c894350 mov qword ptr [rbx + 0x50], r8
1407e2841: c78424e001000000000000 mov dword ptr [rsp + 0x1e0], 0
1407e284c: 448b842494000000 mov r8d, dword ptr [rsp + 0x94]
1407e2854: 44894368 mov dword ptr [rbx + 0x68], r8d
1407e2858: 4c8b84248c000000 mov r8, qword ptr [rsp + 0x8c]
1407e2860: 4c894360 mov qword ptr [rbx + 0x60], r8
1407e2864: 4c8b442478 mov r8, qword ptr [rsp + 0x78]
1407e2869: 4c898424f0010000 mov qword ptr [rsp + 0x1f0], r8
1407e2871: 4c8b842480000000 mov r8, qword ptr [rsp + 0x80]
1407e2879: 4c898424f8010000 mov qword ptr [rsp + 0x1f8], r8
1407e2881: 4889942400020000 mov qword ptr [rsp + 0x200], rdx
1407e2889: 48898c2408020000 mov qword ptr [rsp + 0x208], rcx
1407e2891: 0f299c2410020000 movaps xmmword ptr [rsp + 0x210], xmm3
1407e2899: 0f13842420020000 movlps qword ptr [rsp + 0x220], xmm0
1407e28a1: f30f118c2428020000 movss dword ptr [rsp + 0x228], xmm1
1407e28aa: f30f10842420040000 movss xmm0, dword ptr [rsp + 0x420]
1407e28b3: f30f1184242c020000 movss dword ptr [rsp + 0x22c], xmm0
1407e28bc: f30f11842430020000 movss dword ptr [rsp + 0x230], xmm0
1407e28c5: f30f11942434020000 movss dword ptr [rsp + 0x234], xmm2
1407e28ce: 89842440020000 mov dword ptr [rsp + 0x240], eax
1407e28d5: 4489bc2444020000 mov dword ptr [rsp + 0x244], r15d
1407e28dd: 89842448020000 mov dword ptr [rsp + 0x248], eax
1407e28e4: 89b4244c020000 mov dword ptr [rsp + 0x24c], esi
1407e28eb: 8b84244c010000 mov eax, dword ptr [rsp + 0x14c]
1407e28f2: 8983dc000000 mov dword ptr [rbx + 0xdc], eax
1407e28f8: 0f1084243c010000 movups xmm0, xmmword ptr [rsp + 0x13c]
1407e2900: 0f1183cc000000 movups xmmword ptr [rbx + 0xcc], xmm0
1407e2907: c684246402000002 mov byte ptr [rsp + 0x264], 2
1407e290f: 0fb6442447 movzx eax, byte ptr [rsp + 0x47]
1407e2914: 8883e3000000 mov byte ptr [rbx + 0xe3], al
1407e291a: 0fb7442445 movzx eax, word ptr [rsp + 0x45]
1407e291f: 668983e1000000 mov word ptr [rbx + 0xe1], ax
1407e2926: c784246802000000000000 mov dword ptr [rsp + 0x268], 0
1407e2931: 0f108424f0020000 movups xmm0, xmmword ptr [rsp + 0x2f0]
1407e2939: 0f108c2400030000 movups xmm1, xmmword ptr [rsp + 0x300]
1407e2941: 0f118bf8000000 movups xmmword ptr [rbx + 0xf8], xmm1
1407e2948: 0f1183e8000000 movups xmmword ptr [rbx + 0xe8], xmm0
1407e294f: c684248c02000002 mov byte ptr [rsp + 0x28c], 2
1407e2957: 488b442458 mov rax, qword ptr [rsp + 0x58]
1407e295c: 488b4c245f mov rcx, qword ptr [rsp + 0x5f]
1407e2961: 48898b10010000 mov qword ptr [rbx + 0x110], rcx
1407e2968: 48898309010000 mov qword ptr [rbx + 0x109], rax
1407e296f: c684249c02000002 mov byte ptr [rsp + 0x29c], 2
1407e2977: 66c784249e0200000000 mov word ptr [rsp + 0x29e], 0
1407e2981: c68424a002000009 mov byte ptr [rsp + 0x2a0], 9
1407e2989: 488b8424b1000000 mov rax, qword ptr [rsp + 0xb1]
1407e2991: 488b8c24b8000000 mov rcx, qword ptr [rsp + 0xb8]
1407e2999: 48898b24010000 mov qword ptr [rbx + 0x124], rcx
1407e29a0: 4889831d010000 mov qword ptr [rbx + 0x11d], rax
1407e29a7: 488b442468 mov rax, qword ptr [rsp + 0x68]
1407e29ac: 4889442430 mov qword ptr [rsp + 0x30], rax
1407e29b1: 488b442470 mov rax, qword ptr [rsp + 0x70]
1407e29b6: 4889442428 mov qword ptr [rsp + 0x28], rax
1407e29bb: 488b842450040000 mov rax, qword ptr [rsp + 0x450]
1407e29c3: 4889442420 mov qword ptr [rsp + 0x20], rax
1407e29c8: 488d8c2450010000 lea rcx, [rsp + 0x150]
1407e29d0: 8b942438040000 mov edx, dword ptr [rsp + 0x438]
1407e29d7: 4c8b842440040000 mov r8, qword ptr [rsp + 0x440]
1407e29df: 4c8b8c2448040000 mov r9, qword ptr [rsp + 0x448]
1407e29e7: e88488ac00 call 0x1412ab270
1407e29ec: 4c39f5 cmp rbp, r14
1407e29ef: 0f97c0 seta al
1407e29f2: 4108c4 or r12b, al
1407e29f5: 66480f6ec5 movq xmm0, rbp
1407e29fa: 66440f6ce0 punpcklqdq xmm12, xmm0
1407e29ff: 48ffc7 inc rdi
1407e2a02: 4584e4 test r12b, r12b
1407e2a05: 0f85c5040000 jne 0x1407e2ed0
1407e2a0b: 66410f70c4ee pshufd xmm0, xmm12, 0xee
1407e2a11: 66480f7ec5 movq rbp, xmm0
1407e2a16: 4c39f5 cmp rbp, r14
1407e2a19: 7325 jae 0x1407e2a40
1407e2a1b: 488d4501 lea rax, [rbp + 1]
1407e2a1f: 41f6c501 test r13b, 1
1407e2a23: 746b je 0x1407e2a90
1407e2a25: 4531e4 xor r12d, r12d
1407e2a28: 4889c5 mov rbp, rax
1407e2a2b: 664c0f7ee0 movq rax, xmm12
1407e2a30: 4885c0 test rax, rax
1407e2a33: 7922 jns 0x1407e2a57
1407e2a35: e983000000 jmp 0x1407e2abd
1407e2a3a: 660f1f440000 nop word ptr [rax + rax]
1407e2a40: 41b401 mov r12b, 1
1407e2a43: 41f6c501 test r13b, 1
1407e2a47: 0f8483040000 je 0x1407e2ed0
1407e2a4d: 664c0f7ee0 movq rax, xmm12
1407e2a52: 4885c0 test rax, rax
1407e2a55: 7866 js 0x1407e2abd
1407e2a57: 0f57c0 xorps xmm0, xmm0
1407e2a5a: f3480f2ac0 cvtsi2ss xmm0, rax
1407e2a5f: 66450f70e4ee pshufd xmm12, xmm12, 0xee
1407e2a65: 664c0f7ee0 movq rax, xmm12
1407e2a6a: 4885c0 test rax, rax
1407e2a6d: 7976 jns 0x1407e2ae5
1407e2a6f: 4889c1 mov rcx, rax
1407e2a72: 48d1e9 shr rcx, 1
1407e2a75: 83e001 and eax, 1
1407e2a78: 4809c8 or rax, rcx
1407e2a7b: 0f57c9 xorps xmm1, xmm1
1407e2a7e: f3480f2ac8 cvtsi2ss xmm1, rax
1407e2a83: f30f58c9 addss xmm1, xmm1
1407e2a87: eb64 jmp 0x1407e2aed
1407e2a89: 0f1f8000000000 nop dword ptr [rax]
1407e2a90: 4883c502 add rbp, 2
1407e2a94: 4c39f0 cmp rax, r14
1407e2a97: 410f93c4 setae r12b
1407e2a9b: 480f43e8 cmovae rbp, rax
1407e2a9f: 66480f6ec8 movq xmm1, rax
1407e2aa4: 660f6cc1 punpcklqdq xmm0, xmm1
1407e2aa8: 41bd01000000 mov r13d, 1
1407e2aae: 66440f6fe0 movdqa xmm12, xmm0
1407e2ab3: 664c0f7ee0 movq rax, xmm12
1407e2ab8: 4885c0 test rax, rax
1407e2abb: 799a jns 0x1407e2a57
1407e2abd: 4889c1 mov rcx, rax
1407e2ac0: 48d1e9 shr rcx, 1
1407e2ac3: 83e001 and eax, 1
1407e2ac6: 4809c8 or rax, rcx
1407e2ac9: 0f57c0 xorps xmm0, xmm0
1407e2acc: f3480f2ac0 cvtsi2ss xmm0, rax
1407e2ad1: f30f58c0 addss xmm0, xmm0
1407e2ad5: 66450f70e4ee pshufd xmm12, xmm12, 0xee
1407e2adb: 664c0f7ee0 movq rax, xmm12
1407e2ae0: 4885c0 test rax, rax
1407e2ae3: 788a js 0x1407e2a6f
1407e2ae5: 0f57c9 xorps xmm1, xmm1
1407e2ae8: f3480f2ac8 cvtsi2ss xmm1, rax
1407e2aed: 0f14c1 unpcklps xmm0, xmm1
1407e2af0: 0f5e8424f0000000 divps xmm0, xmmword ptr [rsp + 0xf0]
1407e2af8: 0f282db1bb1402 movaps xmm5, xmmword ptr [rip + 0x214bbb1]
1407e2aff: 0f28e5 movaps xmm4, xmm5
1407e2b02: 0f5ce0 subps xmm4, xmm0
1407e2b05: 0f59a424e0000000 mulps xmm4, xmmword ptr [rsp + 0xe0]
1407e2b0d: 0f58e0 addps xmm4, xmm0
1407e2b10: 0f28d5 movaps xmm2, xmm5
1407e2b13: 0f5cd4 subps xmm2, xmm4
1407e2b16: f30f12c4 movsldup xmm0, xmm4
1407e2b1a: 0f288c2400010000 movaps xmm1, xmmword ptr [rsp + 0x100]
1407e2b22: 0f59cc mulps xmm1, xmm4
1407e2b25: f3440f16f4 movshdup xmm14, xmm4
1407e2b2a: 0f58e4 addps xmm4, xmm4
1407e2b2d: 0f58259cbb1402 addps xmm4, xmmword ptr [rip + 0x214bb9c]
1407e2b34: 0f59e4 mulps xmm4, xmm4
1407e2b37: 0f5cec subps xmm5, xmm4
1407e2b3a: 0f592d4f5d2a02 mulps xmm5, xmmword ptr [rip + 0x22a5d4f]
1407e2b41: 0f59ac24c0000000 mulps xmm5, xmmword ptr [rsp + 0xc0]
1407e2b49: f30f12e2 movsldup xmm4, xmm2
1407e2b4d: 0f289c2420010000 movaps xmm3, xmmword ptr [rsp + 0x120]
1407e2b55: 0f59e3 mulps xmm4, xmm3
1407e2b58: 0f28b42410010000 movaps xmm6, xmmword ptr [rsp + 0x110]
1407e2b60: 0f59c6 mulps xmm0, xmm6
1407e2b63: 0f58c4 addps xmm0, xmm4
1407e2b66: 440f28c5 movaps xmm8, xmm5
1407e2b6a: 440f59052e2e1402 mulps xmm8, xmmword ptr [rip + 0x2142e2e]
1407e2b72: 410f28e0 movaps xmm4, xmm8
1407e2b76: 0f14e5 unpcklps xmm4, xmm5
1407e2b79: 0f5cc4 subps xmm0, xmm4
1407e2b7c: 0f28a424d0000000 movaps xmm4, xmmword ptr [rsp + 0xd0]
1407e2b84: 0f59e2 mulps xmm4, xmm2
1407e2b87: 0f58cc addps xmm1, xmm4
1407e2b8a: 410f5cc8 subps xmm1, xmm8
1407e2b8e: f30f16e1 movshdup xmm4, xmm1
1407e2b92: f30f5ce1 subss xmm4, xmm1
1407e2b96: f30f16d2 movshdup xmm2, xmm2
1407e2b9a: 0f59d3 mulps xmm2, xmm3
1407e2b9d: 440f59f6 mulps xmm14, xmm6
1407e2ba1: 440f58f2 addps xmm14, xmm2
1407e2ba5: 410fc6e811 shufps xmm5, xmm8, 0x11
1407e2baa: 410fc6e8e2 shufps xmm5, xmm8, 0xe2
1407e2baf: 440f5cf5 subps xmm14, xmm5
1407e2bb3: 440f5cf0 subps xmm14, xmm0
1407e2bb7: 410f28d6 movaps xmm2, xmm14
1407e2bbb: 410f59d6 mulps xmm2, xmm14
1407e2bbf: f30f16ea movshdup xmm5, xmm2
1407e2bc3: f30f58ea addss xmm5, xmm2
1407e2bc7: 0f28d4 movaps xmm2, xmm4
1407e2bca: f30f59d4 mulss xmm2, xmm4
1407e2bce: f30f58d5 addss xmm2, xmm5
1407e2bd2: f30f51d2 sqrtss xmm2, xmm2
1407e2bd6: 450f28c7 movaps xmm8, xmm15
1407e2bda: f3440f5ec2 divss xmm8, xmm2
1407e2bdf: f3450f16d6 movshdup xmm10, xmm14
1407e2be4: f3450f59f0 mulss xmm14, xmm8
1407e2be9: f3450f59d0 mulss xmm10, xmm8
1407e2bee: f3440f59c4 mulss xmm8, xmm4
1407e2bf3: 410f28e2 movaps xmm4, xmm10
1407e2bf7: f30f101d991a1502 movss xmm3, dword ptr [rip + 0x2151a99]
1407e2bff: f30f59e3 mulss xmm4, xmm3
1407e2c03: 410f28f0 movaps xmm6, xmm8
1407e2c07: f30f58f4 addss xmm6, xmm4
1407e2c0b: 410f28e8 movaps xmm5, xmm8
1407e2c0f: f30f59eb mulss xmm5, xmm3
1407e2c13: 410f28fe movaps xmm7, xmm14
1407e2c17: f3410f59fd mulss xmm7, xmm13
1407e2c1c: f30f58fd addss xmm7, xmm5
1407e2c20: 450f28de movaps xmm11, xmm14
1407e2c24: 440f571dc42c1402 xorps xmm11, xmmword ptr [rip + 0x2142cc4]
1407e2c2c: f3440f5cdc subss xmm11, xmm4
1407e2c31: 0f28e6 movaps xmm4, xmm6
1407e2c34: f30f59e6 mulss xmm4, xmm6
1407e2c38: 0f28ef movaps xmm5, xmm7
1407e2c3b: f30f59ef mulss xmm5, xmm7
1407e2c3f: f30f58ec addss xmm5, xmm4
1407e2c43: 410f28e3 movaps xmm4, xmm11
1407e2c47: f3410f59e3 mulss xmm4, xmm11
1407e2c4c: f30f58e5 addss xmm4, xmm5
1407e2c50: f30f51e4 sqrtss xmm4, xmm4
1407e2c54: 410f28ef movaps xmm5, xmm15
1407e2c58: f30f5eec divss xmm5, xmm4
1407e2c5c: f30f59f5 mulss xmm6, xmm5
1407e2c60: f30f59fd mulss xmm7, xmm5
1407e2c64: f3410f59eb mulss xmm5, xmm11
1407e2c69: 410f28e2 movaps xmm4, xmm10
1407e2c6d: f30f59e5 mulss xmm4, xmm5
1407e2c71: 450f28d8 movaps xmm11, xmm8
1407e2c75: f3440f59df mulss xmm11, xmm7
1407e2c7a: f3410f5ce3 subss xmm4, xmm11
1407e2c7f: 450f28c8 movaps xmm9, xmm8
1407e2c83: f3440f59ce mulss xmm9, xmm6
1407e2c88: 450f28de movaps xmm11, xmm14
1407e2c8c: f3440f59dd mulss xmm11, xmm5
1407e2c91: f3450f5ccb subss xmm9, xmm11
1407e2c96: 410f28de movaps xmm3, xmm14
1407e2c9a: f30f59df mulss xmm3, xmm7
1407e2c9e: 450f28da movaps xmm11, xmm10
1407e2ca2: f3440f59de mulss xmm11, xmm6
1407e2ca7: f3410f5cdb subss xmm3, xmm11
1407e2cac: 440f28dc movaps xmm11, xmm4
1407e2cb0: f3440f59dc mulss xmm11, xmm4
1407e2cb5: 450f28e9 movaps xmm13, xmm9
1407e2cb9: f3450f59e9 mulss xmm13, xmm9
1407e2cbe: f3450f58eb addss xmm13, xmm11
1407e2cc3: 440f28db movaps xmm11, xmm3
1407e2cc7: f3440f59db mulss xmm11, xmm3
1407e2ccc: f3450f58dd addss xmm11, xmm13
1407e2cd1: 450f57ed xorps xmm13, xmm13
1407e2cd5: f3450f51eb sqrtss xmm13, xmm11
1407e2cda: 450f28df movaps xmm11, xmm15
1407e2cde: f3450f5edd divss xmm11, xmm13
1407e2ce3: 450f57ed xorps xmm13, xmm13
1407e2ce7: f3410f59e3 mulss xmm4, xmm11
1407e2cec: f3450f59cb mulss xmm9, xmm11
1407e2cf1: f3440f59db mulss xmm11, xmm3
1407e2cf6: 450f2ee8 ucomiss xmm13, xmm8
1407e2cfa: 0f8390000000 jae 0x1407e2d90
1407e2d00: f3410f58f1 addss xmm6, xmm9
1407e2d05: f3450f58c7 addss xmm8, xmm15
1407e2d0a: 440f2eee ucomiss xmm13, xmm6
1407e2d0e: 0f83cc000000 jae 0x1407e2de0
1407e2d14: 450f14de unpcklps xmm11, xmm14
1407e2d18: 450f28cb movaps xmm9, xmm11
1407e2d1c: 440f16cf movlhps xmm9, xmm7
1407e2d20: 440f16c7 movlhps xmm8, xmm7
1407e2d24: 450fc6d824 shufps xmm11, xmm8, 0x24
1407e2d29: 440f14d5 unpcklps xmm10, xmm5
1407e2d2d: 410f28da movaps xmm3, xmm10
1407e2d31: 0f16dc movlhps xmm3, xmm4
1407e2d34: 440f5ccb subps xmm9, xmm3
1407e2d38: 0f16f4 movlhps xmm6, xmm4
1407e2d3b: 440fc6d624 shufps xmm10, xmm6, 0x24
1407e2d40: 450f58d3 addps xmm10, xmm11
1407e2d44: 410f28da movaps xmm3, xmm10
1407e2d48: 410fc6d923 shufps xmm3, xmm9, 0x23
1407e2d4d: 440fc6cb24 shufps xmm9, xmm3, 0x24
1407e2d52: 450fc6d2ff shufps xmm10, xmm10, 0xff
1407e2d57: 0f57e4 xorps xmm4, xmm4
1407e2d5a: f3410f51e2 sqrtss xmm4, xmm10
1407e2d5f: f30f101d092b1402 movss xmm3, dword ptr [rip + 0x2142b09]
1407e2d67: f30f5edc divss xmm3, xmm4
1407e2d6b: 0fc6db00 shufps xmm3, xmm3, 0
1407e2d6f: 410f59d9 mulps xmm3, xmm9
1407e2d73: 807c244400 cmp byte ptr [rsp + 0x44], 0
1407e2d78: 0f84ad000000 je 0x1407e2e2b
1407e2d7e: e9b1000000 jmp 0x1407e2e34
1407e2d83: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1407e2d90: f3440f5cce subss xmm9, xmm6
1407e2d95: 410f28f7 movaps xmm6, xmm15
1407e2d99: f3410f5cf0 subss xmm6, xmm8
1407e2d9e: 450f2ee9 ucomiss xmm13, xmm9
1407e2da2: 0f83c8000000 jae 0x1407e2e70
1407e2da8: 450f14de unpcklps xmm11, xmm14
1407e2dac: 0f14fe unpcklps xmm7, xmm6
1407e2daf: 410f16fb movlhps xmm7, xmm11
1407e2db3: 440f14d5 unpcklps xmm10, xmm5
1407e2db7: 410f14e1 unpcklps xmm4, xmm9
1407e2dbb: 410f16e2 movlhps xmm4, xmm10
1407e2dbf: 0f28ef movaps xmm5, xmm7
1407e2dc2: 0f58ec addps xmm5, xmm4
1407e2dc5: 0f5cfc subps xmm7, xmm4
1407e2dc8: 0fc6fd23 shufps xmm7, xmm5, 0x23
1407e2dcc: f30f16dd movshdup xmm3, xmm5
1407e2dd0: 0fc6ef24 shufps xmm5, xmm7, 0x24
1407e2dd4: eb34 jmp 0x1407e2e0a
1407e2dd6: 662e0f1f840000000000 nop word ptr cs:[rax + rax]
1407e2de0: 410f14ea unpcklps xmm5, xmm10
1407e2de4: 410f16f8 movlhps xmm7, xmm8
1407e2de8: 0f28dd movaps xmm3, xmm5
1407e2deb: 0fc6df24 shufps xmm3, xmm7, 0x24
1407e2def: 450f14f3 unpcklps xmm14, xmm11
1407e2df3: 0f16e6 movlhps xmm4, xmm6
1407e2df6: 410f58ee addps xmm5, xmm14
1407e2dfa: 440fc6f424 shufps xmm14, xmm4, 0x24
1407e2dff: 410f5cde subps xmm3, xmm14
1407e2e03: 0fc6ebe4 shufps xmm5, xmm3, 0xe4
1407e2e07: 0f12db movhlps xmm3, xmm3
1407e2e0a: 0f57e4 xorps xmm4, xmm4
1407e2e0d: f30f51e3 sqrtss xmm4, xmm3
1407e2e11: f30f101d572a1402 movss xmm3, dword ptr [rip + 0x2142a57]
1407e2e19: f30f5edc divss xmm3, xmm4
1407e2e1d: 0fc6db00 shufps xmm3, xmm3, 0
1407e2e21: 0f59dd mulps xmm3, xmm5
1407e2e24: 807c244400 cmp byte ptr [rsp + 0x44], 0
1407e2e29: 7509 jne 0x1407e2e34
1407e2e2b: f30f10942420040000 movss xmm2, dword ptr [rsp + 0x420]
1407e2e34: 488d44245b lea rax, [rsp + 0x5b]
1407e2e39: c7400800000000 mov dword ptr [rax + 8], 0
1407e2e40: 48c70000000000 mov qword ptr [rax], 0
1407e2e47: f644244301 test byte ptr [rsp + 0x43], 1
1407e2e4c: 0f85eef8ffff jne 0x1407e2740
1407e2e52: b917000000 mov ecx, 0x17
1407e2e57: 31c0 xor eax, eax
1407e2e59: 488d15a4e34302 lea rdx, [rip + 0x243e3a4]
1407e2e60: e938f9ffff jmp 0x1407e279d
1407e2e65: 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
1407e2e70: 0f14f7 unpcklps xmm6, xmm7
1407e2e73: 0f28de movaps xmm3, xmm6
1407e2e76: 410f16de movlhps xmm3, xmm14
1407e2e7a: 450f16de movlhps xmm11, xmm14
1407e2e7e: 410fc6f324 shufps xmm6, xmm11, 0x24
1407e2e83: 440f14cc unpcklps xmm9, xmm4
1407e2e87: 410f28e1 movaps xmm4, xmm9
1407e2e8b: 0f16e5 movlhps xmm4, xmm5
1407e2e8e: 0f58e3 addps xmm4, xmm3
1407e2e91: 440f16d5 movlhps xmm10, xmm5
1407e2e95: 450fc6ca24 shufps xmm9, xmm10, 0x24
1407e2e9a: 410f5cf1 subps xmm6, xmm9
1407e2e9e: 0f57ed xorps xmm5, xmm5
1407e2ea1: f30f51ee sqrtss xmm5, xmm6
1407e2ea5: 0fc6f49c shufps xmm6, xmm4, 0x9c
1407e2ea9: 0fc6f678 shufps xmm6, xmm6, 0x78
1407e2ead: f30f101dbb291402 movss xmm3, dword ptr [rip + 0x21429bb]
1407e2eb5: f30f5edd divss xmm3, xmm5
1407e2eb9: 0fc6db00 shufps xmm3, xmm3, 0
1407e2ebd: 0f59de mulps xmm3, xmm6
1407e2ec0: 807c244400 cmp byte ptr [rsp + 0x44], 0
1407e2ec5: 0f8460ffffff je 0x1407e2e2b
1407e2ecb: e964ffffff jmp 0x1407e2e34
1407e2ed0: 0f28b42410030000 movaps xmm6, xmmword ptr [rsp + 0x310]
1407e2ed8: 0f28bc2420030000 movaps xmm7, xmmword ptr [rsp + 0x320]
1407e2ee0: 440f28842430030000 movaps xmm8, xmmword ptr [rsp + 0x330]
1407e2ee9: 440f288c2440030000 movaps xmm9, xmmword ptr [rsp + 0x340]
1407e2ef2: 440f28942450030000 movaps xmm10, xmmword ptr [rsp + 0x350]
1407e2efb: 440f289c2460030000 movaps xmm11, xmmword ptr [rsp + 0x360]
1407e2f04: 440f28a42470030000 movaps xmm12, xmmword ptr [rsp + 0x370]
1407e2f0d: 440f28ac2480030000 movaps xmm13, xmmword ptr [rsp + 0x380]
1407e2f16: 440f28b42490030000 movaps xmm14, xmmword ptr [rsp + 0x390]
1407e2f1f: 440f28bc24a0030000 movaps xmm15, xmmword ptr [rsp + 0x3a0]
1407e2f28: 4881c4b8030000 add rsp, 0x3b8
1407e2f2f: 5b pop rbx
1407e2f30: 5d pop rbp
1407e2f31: 5f pop rdi
1407e2f32: 5e pop rsi
1407e2f33: 415c pop r12
1407e2f35: 415d pop r13
1407e2f37: 415e pop r14
1407e2f39: 415f pop r15
1407e2f3b: c3 ret 
