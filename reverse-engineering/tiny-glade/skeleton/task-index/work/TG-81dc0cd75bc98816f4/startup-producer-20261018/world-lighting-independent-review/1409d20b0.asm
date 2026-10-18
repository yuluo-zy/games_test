1409d20b0: 55 push rbp
1409d20b1: 4157 push r15
1409d20b3: 4156 push r14
1409d20b5: 4155 push r13
1409d20b7: 4154 push r12
1409d20b9: 56 push rsi
1409d20ba: 57 push rdi
1409d20bb: 53 push rbx
1409d20bc: 4881ec080e0000 sub rsp, 0xe08
1409d20c3: 488dac2480000000 lea rbp, [rsp + 0x80]
1409d20cb: 0f29bd700d0000 movaps xmmword ptr [rbp + 0xd70], xmm7
1409d20d2: 0f29b5600d0000 movaps xmmword ptr [rbp + 0xd60], xmm6
1409d20d9: 48c785580d0000feffffff mov qword ptr [rbp + 0xd58], 0xfffffffffffffffe
1409d20e4: 4c898d400d0000 mov qword ptr [rbp + 0xd40], r9
1409d20eb: 4c8985200d0000 mov qword ptr [rbp + 0xd20], r8
1409d20f2: 4889d3 mov rbx, rdx
1409d20f5: 4889cf mov rdi, rcx
1409d20f8: 488d7238 lea rsi, [rdx + 0x38]
1409d20fc: 48ba82a3afb957c5ca78 movabs rdx, 0x78cac557b9afa382
1409d2106: 49b855a0837a1d809941 movabs r8, 0x4199801d7a83a055
1409d2110: 4889f1 mov rcx, rsi
1409d2113: e8d8b570ff call 0x1400dd6f0
1409d2118: 4885c0 test rax, rax
1409d211b: 0f84690e0000 je 0x1409d2f8a
1409d2121: 48399318020000 cmp qword ptr [rbx + 0x218], rdx
1409d2128: 0f865c0e0000 jbe 0x1409d2f8a
1409d212e: 488b8310020000 mov rax, qword ptr [rbx + 0x210]
1409d2135: 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
1409d2139: 4885c9 test rcx, rcx
1409d213c: 0f84480e0000 je 0x1409d2f8a
1409d2142: 488b83e0010000 mov rax, qword ptr [rbx + 0x1e0]
1409d2149: 48f7d1 not rcx
1409d214c: 488d0c49 lea rcx, [rcx + rcx*2]
1409d2150: 48c1e105 shl rcx, 5
1409d2154: 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
1409d215a: 0f842a0e0000 je 0x1409d2f8a
1409d2160: 4801c8 add rax, rcx
1409d2163: 488b4010 mov rax, qword ptr [rax + 0x10]
1409d2167: 488985300d0000 mov qword ptr [rbp + 0xd30], rax
1409d216e: 48ba29f7805b1962c34b movabs rdx, 0x4bc362195b80f729
1409d2178: 49b8fbcecb80ce982798 movabs r8, 0x982798ce80cbcefb
1409d2182: 4889f1 mov rcx, rsi
1409d2185: e866b570ff call 0x1400dd6f0
1409d218a: 4885c0 test rax, rax
1409d218d: 0f84030e0000 je 0x1409d2f96
1409d2193: 48399318020000 cmp qword ptr [rbx + 0x218], rdx
1409d219a: 0f86f60d0000 jbe 0x1409d2f96
1409d21a0: 488b8310020000 mov rax, qword ptr [rbx + 0x210]
1409d21a7: 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
1409d21ab: 4885c9 test rcx, rcx
1409d21ae: 0f84e20d0000 je 0x1409d2f96
1409d21b4: 488b83e0010000 mov rax, qword ptr [rbx + 0x1e0]
1409d21bb: 48f7d1 not rcx
1409d21be: 488d0c49 lea rcx, [rcx + rcx*2]
1409d21c2: 48c1e105 shl rcx, 5
1409d21c6: 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
1409d21cc: 0f84c40d0000 je 0x1409d2f96
1409d21d2: 4801c8 add rax, rcx
1409d21d5: 488b4010 mov rax, qword ptr [rax + 0x10]
1409d21d9: 488b08 mov rcx, qword ptr [rax]
1409d21dc: 4885c9 test rcx, rcx
1409d21df: 7807 js 0x1409d21e8
1409d21e1: f3480f2af9 cvtsi2ss xmm7, rcx
1409d21e6: eb15 jmp 0x1409d21fd
1409d21e8: 4889ca mov rdx, rcx
1409d21eb: 48d1ea shr rdx, 1
1409d21ee: 83e101 and ecx, 1
1409d21f1: 4809d1 or rcx, rdx
1409d21f4: f3480f2af9 cvtsi2ss xmm7, rcx
1409d21f9: f30f58ff addss xmm7, xmm7
1409d21fd: f30f2a7008 cvtsi2ss xmm6, dword ptr [rax + 8]
1409d2202: 488b85200d0000 mov rax, qword ptr [rbp + 0xd20]
1409d2209: 4c8d7024 lea r14, [rax + 0x24]
1409d220d: 488d4828 lea rcx, [rax + 0x28]
1409d2211: e87a352000 call 0x140bd5790
1409d2216: c7859806000000000000 mov dword ptr [rbp + 0x698], 0
1409d2220: 488b8d400d0000 mov rcx, qword ptr [rbp + 0xd40]
1409d2227: 48898da0060000 mov qword ptr [rbp + 0x6a0], rcx
1409d222e: 48c785780b000001000000 mov qword ptr [rbp + 0xb78], 1
1409d2239: 4c89b5f80c0000 mov qword ptr [rbp + 0xcf8], r14
1409d2240: 4c89b5800b0000 mov qword ptr [rbp + 0xb80], r14
1409d2247: 48c785880b000000000000 mov qword ptr [rbp + 0xb88], 0
1409d2252: 48897c2440 mov qword ptr [rsp + 0x40], rdi
1409d2257: 48899d280d0000 mov qword ptr [rbp + 0xd28], rbx
1409d225e: 48895c2438 mov qword ptr [rsp + 0x38], rbx
1409d2263: 4889442430 mov qword ptr [rsp + 0x30], rax
1409d2268: 4c8db5780b0000 lea r14, [rbp + 0xb78]
1409d226f: 4c89742428 mov qword ptr [rsp + 0x28], r14
1409d2274: 48c744242001000000 mov qword ptr [rsp + 0x20], 1
1409d227d: 488d150cf60e02 lea rdx, [rip + 0x20ef60c]
1409d2284: 488d4dc8 lea rcx, [rbp - 0x38]
1409d2288: 4c8dbd98060000 lea r15, [rbp + 0x698]
1409d228f: 41b817000000 mov r8d, 0x17
1409d2295: 4d89f9 mov r9, r15
1409d2298: e8d356e9ff call 0x140867970
1409d229d: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d22a4: 48ba5593676e78cc2682 movabs rdx, 0x8226cc786e679355
1409d22ae: 49b8ac41a8556adacf65 movabs r8, 0x65cfda6a55a841ac
1409d22b8: 4889f1 mov rcx, rsi
1409d22bb: e830b470ff call 0x1400dd6f0
1409d22c0: 4885c0 test rax, rax
1409d22c3: 4c8b85280d0000 mov r8, qword ptr [rbp + 0xd28]
1409d22ca: 0f84d20c0000 je 0x1409d2fa2
1409d22d0: 49399018020000 cmp qword ptr [r8 + 0x218], rdx
1409d22d7: 0f86c50c0000 jbe 0x1409d2fa2
1409d22dd: 498b8010020000 mov rax, qword ptr [r8 + 0x210]
1409d22e4: 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
1409d22e8: 4885c9 test rcx, rcx
1409d22eb: 0f84b10c0000 je 0x1409d2fa2
1409d22f1: 498b80e0010000 mov rax, qword ptr [r8 + 0x1e0]
1409d22f8: 48f7d1 not rcx
1409d22fb: 488d0c49 lea rcx, [rcx + rcx*2]
1409d22ff: 48c1e105 shl rcx, 5
1409d2303: 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
1409d2309: 0f84930c0000 je 0x1409d2fa2
1409d230f: 4801c8 add rax, rcx
1409d2312: 488b4810 mov rcx, qword ptr [rax + 0x10]
1409d2316: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d231d: 488d150cbf2502 lea rdx, [rip + 0x225bf0c]
1409d2324: e8f773f9ff call 0x140969720
1409d2329: 4889c6 mov rsi, rax
1409d232c: 4885c0 test rax, rax
1409d232f: 0f84820c0000 je 0x1409d2fb7
1409d2335: 488d05c2af0c02 lea rax, [rip + 0x20cafc2]
1409d233c: 488985780b0000 mov qword ptr [rbp + 0xb78], rax
1409d2343: 48c785800b000035000000 mov qword ptr [rbp + 0xb80], 0x35
1409d234e: 4889f7 mov rdi, rsi
1409d2351: 4883c710 add rdi, 0x10
1409d2355: 48837e1835 cmp qword ptr [rsi + 0x18], 0x35
1409d235a: 0f85910b0000 jne 0x1409d2ef1
1409d2360: 488b17 mov rdx, qword ptr [rdi]
1409d2363: 488d0d94af0c02 lea rcx, [rip + 0x20caf94]
1409d236a: 41b835000000 mov r8d, 0x35
1409d2370: e87b18f501 call 0x142923bf0
1409d2375: 85c0 test eax, eax
1409d2377: 0f85740b0000 jne 0x1409d2ef1
1409d237d: 488b0e mov rcx, qword ptr [rsi]
1409d2380: 488b4608 mov rax, qword ptr [rsi + 8]
1409d2384: 488b4018 mov rax, qword ptr [rax + 0x18]
1409d2388: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d238f: ffd0 call rax
1409d2391: 4885c0 test rax, rax
1409d2394: 0f841d0c0000 je 0x1409d2fb7
1409d239a: 488b95300e0000 mov rdx, qword ptr [rbp + 0xe30]
1409d23a1: 4c8b85100e0000 mov r8, qword ptr [rbp + 0xe10]
1409d23a8: 0fb68df80d0000 movzx ecx, byte ptr [rbp + 0xdf8]
1409d23af: f30f5e358952fd01 divss xmm6, dword ptr [rip + 0x1fd5289]
1409d23b7: f30f58f7 addss xmm6, xmm7
1409d23bb: 488b00 mov rax, qword ptr [rax]
1409d23be: 488985180d0000 mov qword ptr [rbp + 0xd18], rax
1409d23c5: 48c785c80c000005000000 mov qword ptr [rbp + 0xcc8], 5
1409d23d0: c785d00c000001000000 mov dword ptr [rbp + 0xcd0], 1
1409d23da: c685d40c000000 mov byte ptr [rbp + 0xcd4], 0
1409d23e1: c785d80c000003000000 mov dword ptr [rbp + 0xcd8], 3
1409d23eb: c685dc0c000000 mov byte ptr [rbp + 0xcdc], 0
1409d23f2: c785e00c000000000000 mov dword ptr [rbp + 0xce0], 0
1409d23fc: c685e40c000000 mov byte ptr [rbp + 0xce4], 0
1409d2403: c785e80c000000000000 mov dword ptr [rbp + 0xce8], 0
1409d240d: c685ec0c000001 mov byte ptr [rbp + 0xcec], 1
1409d2414: c785f00c000002000000 mov dword ptr [rbp + 0xcf0], 2
1409d241e: c685f40c000000 mov byte ptr [rbp + 0xcf4], 0
1409d2425: 0fb6c1 movzx eax, cl
1409d2428: 89853c0d0000 mov dword ptr [rbp + 0xd3c], eax
1409d242e: 488385400d000004 add qword ptr [rbp + 0xd40], 4
1409d2436: 498d4024 lea rax, [r8 + 0x24]
1409d243a: 488985000d0000 mov qword ptr [rbp + 0xd00], rax
1409d2441: 498d4048 lea rax, [r8 + 0x48]
1409d2445: 488985080d0000 mov qword ptr [rbp + 0xd08], rax
1409d244c: 488d4218 lea rax, [rdx + 0x18]
1409d2450: 488985100d0000 mov qword ptr [rbp + 0xd10], rax
1409d2457: 0f57ff xorps xmm7, xmm7
1409d245a: 488db5800a0000 lea rsi, [rbp + 0xa80]
1409d2461: 31c0 xor eax, eax
1409d2463: 488b95300d0000 mov rdx, qword ptr [rbp + 0xd30]
1409d246a: 4883f805 cmp rax, 5
1409d246e: 0f84330a0000 je 0x1409d2ea7
1409d2474: 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1409d2480: 4c8d6001 lea r12, [rax + 1]
1409d2484: 4c89a5c00c0000 mov qword ptr [rbp + 0xcc0], r12
1409d248b: 448bacc5d00c0000 mov r13d, dword ptr [rbp + rax*8 + 0xcd0]
1409d2493: 4983fd05 cmp r13, 5
1409d2497: 0f838f0a0000 jae 0x1409d2f2c
1409d249d: 4489e9 mov ecx, r13d
1409d24a0: c1e104 shl ecx, 4
1409d24a3: f6040a01 test byte ptr [rdx + rcx], 1
1409d24a7: 0f849e0a0000 je 0x1409d2f4b
1409d24ad: 488b7c0a08 mov rdi, qword ptr [rdx + rcx + 8]
1409d24b2: 80bcc5d40c000000 cmp byte ptr [rbp + rax*8 + 0xcd4], 0
1409d24ba: 0f84a0000000 je 0x1409d2560
1409d24c0: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d24c7: e884422100 call 0x140be6750
1409d24cc: 89c3 mov ebx, eax
1409d24ce: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d24d5: 31c9 xor ecx, ecx
1409d24d7: e8447670ff call 0x1400d9b20
1409d24dc: 4885c0 test rax, rax
1409d24df: 0f84900a0000 je 0x1409d2f75
1409d24e5: 0f1000 movups xmm0, xmmword ptr [rax]
1409d24e8: 48ff00 inc qword ptr [rax]
1409d24eb: 0f100d3e420c02 movups xmm1, xmmword ptr [rip + 0x20c423e]
1409d24f2: 488d85c0060000 lea rax, [rbp + 0x6c0]
1409d24f9: 0f114810 movups xmmword ptr [rax + 0x10], xmm1
1409d24fd: 0f100d1c420c02 movups xmm1, xmmword ptr [rip + 0x20c421c]
1409d2504: 0f1108 movups xmmword ptr [rax], xmm1
1409d2507: 4180cd80 or r13b, 0x80
1409d250b: 0f108d98060000 movups xmm1, xmmword ptr [rbp + 0x698]
1409d2512: 0f1095a8060000 movups xmm2, xmmword ptr [rbp + 0x6a8]
1409d2519: 0f109db8060000 movups xmm3, xmmword ptr [rbp + 0x6b8]
1409d2520: 0f10a5c8060000 movups xmm4, xmmword ptr [rbp + 0x6c8]
1409d2527: 0f298d700c0000 movaps xmmword ptr [rbp + 0xc70], xmm1
1409d252e: 0f2995800c0000 movaps xmmword ptr [rbp + 0xc80], xmm2
1409d2535: 0f299d900c0000 movaps xmmword ptr [rbp + 0xc90], xmm3
1409d253c: 0f29a5a00c0000 movaps xmmword ptr [rbp + 0xca0], xmm4
1409d2543: 488b85d8060000 mov rax, qword ptr [rbp + 0x6d8]
1409d254a: 488985b00c0000 mov qword ptr [rbp + 0xcb0], rax
1409d2551: 89d8 mov eax, ebx
1409d2553: c1e818 shr eax, 0x18
1409d2556: 249f and al, 0x9f
1409d2558: 0c20 or al, 0x20
1409d255a: e993000000 jmp 0x1409d25f2
1409d255f: 90 nop 
1409d2560: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d2567: e8e4412100 call 0x140be6750
1409d256c: 89c3 mov ebx, eax
1409d256e: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d2575: 31c9 xor ecx, ecx
1409d2577: e8a47570ff call 0x1400d9b20
1409d257c: 4885c0 test rax, rax
1409d257f: 0f84db090000 je 0x1409d2f60
1409d2585: 0f1000 movups xmm0, xmmword ptr [rax]
1409d2588: 48ff00 inc qword ptr [rax]
1409d258b: 0f100d9e410c02 movups xmm1, xmmword ptr [rip + 0x20c419e]
1409d2592: 488d85c0060000 lea rax, [rbp + 0x6c0]
1409d2599: 0f114810 movups xmmword ptr [rax + 0x10], xmm1
1409d259d: 0f100d7c410c02 movups xmm1, xmmword ptr [rip + 0x20c417c]
1409d25a4: 0f1108 movups xmmword ptr [rax], xmm1
1409d25a7: 0f108d98060000 movups xmm1, xmmword ptr [rbp + 0x698]
1409d25ae: 0f1095a8060000 movups xmm2, xmmword ptr [rbp + 0x6a8]
1409d25b5: 0f109db8060000 movups xmm3, xmmword ptr [rbp + 0x6b8]
1409d25bc: 0f10a5c8060000 movups xmm4, xmmword ptr [rbp + 0x6c8]
1409d25c3: 0f298d700c0000 movaps xmmword ptr [rbp + 0xc70], xmm1
1409d25ca: 0f2995800c0000 movaps xmmword ptr [rbp + 0xc80], xmm2
1409d25d1: 0f299d900c0000 movaps xmmword ptr [rbp + 0xc90], xmm3
1409d25d8: 0f29a5a00c0000 movaps xmmword ptr [rbp + 0xca0], xmm4
1409d25df: 488b85d8060000 mov rax, qword ptr [rbp + 0x6d8]
1409d25e6: 488985b00c0000 mov qword ptr [rbp + 0xcb0], rax
1409d25ed: 89d8 mov eax, ebx
1409d25ef: c1e818 shr eax, 0x18
1409d25f2: 89d9 mov ecx, ebx
1409d25f4: 80e1bf and cl, 0xbf
1409d25f7: c1eb08 shr ebx, 8
1409d25fa: 48c785780b000000000000 mov qword ptr [rbp + 0xb78], 0
1409d2605: 48c785880b000000000000 mov qword ptr [rbp + 0xb88], 0
1409d2610: 48c785900b000004000000 mov qword ptr [rbp + 0xb90], 4
1409d261b: 4c8d85980b0000 lea r8, [rbp + 0xb98]
1409d2622: 410f1138 movups xmmword ptr [r8], xmm7
1409d2626: 48c785a80b000001000000 mov qword ptr [rbp + 0xba8], 1
1409d2631: 48c785b00b000000000000 mov qword ptr [rbp + 0xbb0], 0
1409d263c: 48ba0300000001000000 movabs rdx, 0x100000003
1409d2646: 488995b80b0000 mov qword ptr [rbp + 0xbb8], rdx
1409d264d: 488b95b00c0000 mov rdx, qword ptr [rbp + 0xcb0]
1409d2654: 49895068 mov qword ptr [r8 + 0x68], rdx
1409d2658: 0f288d700c0000 movaps xmm1, xmmword ptr [rbp + 0xc70]
1409d265f: 0f2895800c0000 movaps xmm2, xmmword ptr [rbp + 0xc80]
1409d2666: 0f289d900c0000 movaps xmm3, xmmword ptr [rbp + 0xc90]
1409d266d: 0f28a5a00c0000 movaps xmm4, xmmword ptr [rbp + 0xca0]
1409d2674: 410f116058 movups xmmword ptr [r8 + 0x58], xmm4
1409d2679: 410f115848 movups xmmword ptr [r8 + 0x48], xmm3
1409d267e: 410f115038 movups xmmword ptr [r8 + 0x38], xmm2
1409d2683: 410f114828 movups xmmword ptr [r8 + 0x28], xmm1
1409d2688: 0f1185080c0000 movups xmmword ptr [rbp + 0xc08], xmm0
1409d268f: 4889bd180c0000 mov qword ptr [rbp + 0xc18], rdi
1409d2696: 888d200c0000 mov byte ptr [rbp + 0xc20], cl
1409d269c: 66899d210c0000 mov word ptr [rbp + 0xc21], bx
1409d26a3: 8885230c0000 mov byte ptr [rbp + 0xc23], al
1409d26a9: c785240c00000000803f mov dword ptr [rbp + 0xc24], 0x3f800000
1409d26b3: c685280c000000 mov byte ptr [rbp + 0xc28], 0
1409d26ba: 4488ad290c0000 mov byte ptr [rbp + 0xc29], r13b
1409d26c1: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d26c8: 4c89f9 mov rcx, r15
1409d26cb: 4c89f2 mov rdx, r14
1409d26ce: 4c8d45c8 lea r8, [rbp - 0x38]
1409d26d2: 4531c9 xor r9d, r9d
1409d26d5: e80635f3ff call 0x140905be0
1409d26da: 80bd7607000008 cmp byte ptr [rbp + 0x776], 8
1409d26e1: 742c je 0x1409d270f
1409d26e3: 488b9580070000 mov rdx, qword ptr [rbp + 0x780]
1409d26ea: b8e8000000 mov eax, 0xe8
1409d26ef: 4801c2 add rdx, rax
1409d26f2: 488d8d50070000 lea rcx, [rbp + 0x750]
1409d26f9: 49b827a0523494838857 movabs r8, 0x578883943452a027
1409d2703: 448b8d3c0d0000 mov r9d, dword ptr [rbp + 0xd3c]
1409d270a: e8618ef9ff call 0x14096b570
1409d270f: 41b8f8000000 mov r8d, 0xf8
1409d2715: 4c89f1 mov rcx, r14
1409d2718: 4c89fa mov rdx, r15
1409d271b: e8e014f501 call 0x142923c00
1409d2720: 488b85200d0000 mov rax, qword ptr [rbp + 0xd20]
1409d2727: 488985480d0000 mov qword ptr [rbp + 0xd48], rax
1409d272e: c7859806000002000000 mov dword ptr [rbp + 0x698], 2
1409d2738: 4c897c2420 mov qword ptr [rsp + 0x20], r15
1409d273d: 488dbd480d0000 lea rdi, [rbp + 0xd48]
1409d2744: 4889f9 mov rcx, rdi
1409d2747: 4c89f2 mov rdx, r14
1409d274a: 49b8360bea2d0d4aaa41 movabs r8, 0x41aa4a0d2dea0b36
1409d2754: 41b11d mov r9b, 0x1d
1409d2757: e8a4c0f1ff call 0x1408ee800
1409d275c: 41b8f8000000 mov r8d, 0xf8
1409d2762: 4c89f9 mov rcx, r15
1409d2765: 4c89f2 mov rdx, r14
1409d2768: e89314f501 call 0x142923c00
1409d276d: 488b85f80c0000 mov rax, qword ptr [rbp + 0xcf8]
1409d2774: 488985480d0000 mov qword ptr [rbp + 0xd48], rax
1409d277b: c785780b000002000000 mov dword ptr [rbp + 0xb78], 2
1409d2785: 4c89742420 mov qword ptr [rsp + 0x20], r14
1409d278a: 4889f9 mov rcx, rdi
1409d278d: 4c89fa mov rdx, r15
1409d2790: 49b8fcfb49a09d576148 movabs r8, 0x4861579da049fbfc
1409d279a: 41b11d mov r9b, 0x1d
1409d279d: e85ec0f1ff call 0x1408ee800
1409d27a2: 41b8f8000000 mov r8d, 0xf8
1409d27a8: 4c89f1 mov rcx, r14
1409d27ab: 4c89fa mov rdx, r15
1409d27ae: e84d14f501 call 0x142923c00
1409d27b3: 488b85080e0000 mov rax, qword ptr [rbp + 0xe08]
1409d27ba: 488985480d0000 mov qword ptr [rbp + 0xd48], rax
1409d27c1: c7859806000002000000 mov dword ptr [rbp + 0x698], 2
1409d27cb: 4c897c2420 mov qword ptr [rsp + 0x20], r15
1409d27d0: 4889f9 mov rcx, rdi
1409d27d3: 4c89f2 mov rdx, r14
1409d27d6: 49b8308202283a3c332a movabs r8, 0x2a333c3a28028230
1409d27e0: 41b11d mov r9b, 0x1d
1409d27e3: e818c0f1ff call 0x1408ee800
1409d27e8: 41b8f8000000 mov r8d, 0xf8
1409d27ee: 4c89f9 mov rcx, r15
1409d27f1: 4c89f2 mov rdx, r14
1409d27f4: e80714f501 call 0x142923c00
1409d27f9: 488b85f00d0000 mov rax, qword ptr [rbp + 0xdf0]
1409d2800: 488985480d0000 mov qword ptr [rbp + 0xd48], rax
1409d2807: c785780b000002000000 mov dword ptr [rbp + 0xb78], 2
1409d2811: 4c89742420 mov qword ptr [rsp + 0x20], r14
1409d2816: 4889f9 mov rcx, rdi
1409d2819: 4c89fa mov rdx, r15
1409d281c: 49b8d0096c8285511704 movabs r8, 0x4175185826c09d0
1409d2826: 41b11d mov r9b, 0x1d
1409d2829: e8d2bff1ff call 0x1408ee800
1409d282e: 41b8f8000000 mov r8d, 0xf8
1409d2834: 4c89f1 mov rcx, r14
1409d2837: 4c89fa mov rdx, r15
1409d283a: e8c113f501 call 0x142923c00
1409d283f: 48b86518a091ab399f57 movabs rax, 0x579f39ab91a01865
1409d2849: 488985480d0000 mov qword ptr [rbp + 0xd48], rax
1409d2850: c7859806000002000000 mov dword ptr [rbp + 0x698], 2
1409d285a: 4c897c2420 mov qword ptr [rsp + 0x20], r15
1409d285f: 4889f9 mov rcx, rdi
1409d2862: 4c89f2 mov rdx, r14
1409d2865: 49b88c97666903f55b81 movabs r8, 0x815bf5036966978c
1409d286f: 41b11d mov r9b, 0x1d
1409d2872: e8995df3ff call 0x140908610
1409d2877: 41b8f8000000 mov r8d, 0xf8
1409d287d: 4c89f9 mov rcx, r15
1409d2880: 4c89f2 mov rdx, r14
1409d2883: e87813f501 call 0x142923c00
1409d2888: 488b85000e0000 mov rax, qword ptr [rbp + 0xe00]
1409d288f: 488985480d0000 mov qword ptr [rbp + 0xd48], rax
1409d2896: c785780b000002000000 mov dword ptr [rbp + 0xb78], 2
1409d28a0: 4c89742420 mov qword ptr [rsp + 0x20], r14
1409d28a5: 4889f9 mov rcx, rdi
1409d28a8: 4c89fa mov rdx, r15
1409d28ab: 49b8f99a3964bbe203e0 movabs r8, 0xe003e2bb64399af9
1409d28b5: 41b11d mov r9b, 0x1d
1409d28b8: e843bff1ff call 0x1408ee800
1409d28bd: 41b8f8000000 mov r8d, 0xf8
1409d28c3: 4c89f1 mov rcx, r14
1409d28c6: 4c89fa mov rdx, r15
1409d28c9: e83213f501 call 0x142923c00
1409d28ce: c685480d000008 mov byte ptr [rbp + 0xd48], 8
1409d28d5: c7859806000002000000 mov dword ptr [rbp + 0x698], 2
1409d28df: 4c897c2420 mov qword ptr [rsp + 0x20], r15
1409d28e4: 4889f9 mov rcx, rdi
1409d28e7: 4c89f2 mov rdx, r14
1409d28ea: 49b83fca8dac13011677 movabs r8, 0x77160113ac8dca3f
1409d28f4: 41b11d mov r9b, 0x1d
1409d28f7: e85463f3ff call 0x140908c50
1409d28fc: 41b8f8000000 mov r8d, 0xf8
1409d2902: 4c89f9 mov rcx, r15
1409d2905: 4c89f2 mov rdx, r14
1409d2908: e8f312f501 call 0x142923c00
1409d290d: c685480d00000c mov byte ptr [rbp + 0xd48], 0xc
1409d2914: c785780b000002000000 mov dword ptr [rbp + 0xb78], 2
1409d291e: 4c89742420 mov qword ptr [rsp + 0x20], r14
1409d2923: 4889f9 mov rcx, rdi
1409d2926: 4c89fa mov rdx, r15
1409d2929: 49b8f4b060705c0a878f movabs r8, 0x8f870a5c7060b0f4
1409d2933: 41b11d mov r9b, 0x1d
1409d2936: e81563f3ff call 0x140908c50
1409d293b: 41b8f8000000 mov r8d, 0xf8
1409d2941: 4889f1 mov rcx, rsi
1409d2944: 4c89fa mov rdx, r15
1409d2947: e8b412f501 call 0x142923c00
1409d294c: 80bd5e0b000008 cmp byte ptr [rbp + 0xb5e], 8
1409d2953: 7428 je 0x1409d297d
1409d2955: 488b95680b0000 mov rdx, qword ptr [rbp + 0xb68]
1409d295c: b8e8000000 mov eax, 0xe8
1409d2961: 4801c2 add rdx, rax
1409d2964: 488d8d380b0000 lea rcx, [rbp + 0xb38]
1409d296b: 49b84ca5591f44988a41 movabs r8, 0x418a98441f59a54c
1409d2975: 41b101 mov r9b, 1
1409d2978: e8d38ef9ff call 0x14096b850
1409d297d: 41b8f8000000 mov r8d, 0xf8
1409d2983: 4c89f9 mov rcx, r15
1409d2986: 4889f2 mov rdx, rsi
1409d2989: e87212f501 call 0x142923c00
1409d298e: c685480d000011 mov byte ptr [rbp + 0xd48], 0x11
1409d2995: c785780b000002000000 mov dword ptr [rbp + 0xb78], 2
1409d299f: 4c89742420 mov qword ptr [rsp + 0x20], r14
1409d29a4: 4889f9 mov rcx, rdi
1409d29a7: 4c89fa mov rdx, r15
1409d29aa: 49b8a41a84c94d1f6511 movabs r8, 0x11651f4dc9841aa4
1409d29b4: 41b11d mov r9b, 0x1d
1409d29b7: e89462f3ff call 0x140908c50
1409d29bc: 41b8f8000000 mov r8d, 0xf8
1409d29c2: 4c89f1 mov rcx, r14
1409d29c5: 4c89fa mov rdx, r15
1409d29c8: e83312f501 call 0x142923c00
1409d29cd: 488b85200e0000 mov rax, qword ptr [rbp + 0xe20]
1409d29d4: 488985800a0000 mov qword ptr [rbp + 0xa80], rax
1409d29db: c7859806000002000000 mov dword ptr [rbp + 0x698], 2
1409d29e5: 4c897c2420 mov qword ptr [rsp + 0x20], r15
1409d29ea: 4889f1 mov rcx, rsi
1409d29ed: 4c89f2 mov rdx, r14
1409d29f0: 49b8256dd6567e434199 movabs r8, 0x9941437e56d66d25
1409d29fa: 41b11d mov r9b, 0x1d
1409d29fd: e8febdf1ff call 0x1408ee800
1409d2a02: 41b8f8000000 mov r8d, 0xf8
1409d2a08: 4c89f9 mov rcx, r15
1409d2a0b: 4c89f2 mov rdx, r14
1409d2a0e: e8ed11f501 call 0x142923c00
1409d2a13: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d2a1a: 488d8da8040000 lea rcx, [rbp + 0x4a8]
1409d2a21: 4c89fa mov rdx, r15
1409d2a24: 49b884939f2eef85541e movabs r8, 0x1e5485ef2e9f9384
1409d2a2e: 0f28de movaps xmm3, xmm6
1409d2a31: e8ea6fc500 call 0x141629a20
1409d2a36: 80bd8605000008 cmp byte ptr [rbp + 0x586], 8
1409d2a3d: 7428 je 0x1409d2a67
1409d2a3f: 488b9590050000 mov rdx, qword ptr [rbp + 0x590]
1409d2a46: b8e8000000 mov eax, 0xe8
1409d2a4b: 4801c2 add rdx, rax
1409d2a4e: 488d8d60050000 lea rcx, [rbp + 0x560]
1409d2a55: 49b8f55186b03f67abc5 movabs r8, 0xc5ab673fb08651f5
1409d2a5f: 4531c9 xor r9d, r9d
1409d2a62: e8098bf9ff call 0x14096b570
1409d2a67: 41b8f8000000 mov r8d, 0xf8
1409d2a6d: 4c89f9 mov rcx, r15
1409d2a70: 488d95a8040000 lea rdx, [rbp + 0x4a8]
1409d2a77: e88411f501 call 0x142923c00
1409d2a7c: c685560d000001 mov byte ptr [rbp + 0xd56], 1
1409d2a83: 488b8d400d0000 mov rcx, qword ptr [rbp + 0xd40]
1409d2a8a: e8012d2000 call 0x140bd5790
1409d2a8f: c685560d000000 mov byte ptr [rbp + 0xd56], 0
1409d2a96: 488d8db0030000 lea rcx, [rbp + 0x3b0]
1409d2a9d: 4c89fa mov rdx, r15
1409d2aa0: 49b87149ea8e31b91b53 movabs r8, 0x531bb9318eea4971
1409d2aaa: 4989c1 mov r9, rax
1409d2aad: e86e72c500 call 0x141629d20
1409d2ab2: 80bd8e04000008 cmp byte ptr [rbp + 0x48e], 8
1409d2ab9: 742c je 0x1409d2ae7
1409d2abb: 488b9598040000 mov rdx, qword ptr [rbp + 0x498]
1409d2ac2: b8e8000000 mov eax, 0xe8
1409d2ac7: 4801c2 add rdx, rax
1409d2aca: 488d8d68040000 lea rcx, [rbp + 0x468]
1409d2ad1: 49b89ff68f409e32889b movabs r8, 0x9b88329e408ff69f
1409d2adb: 4c8b8d180d0000 mov r9, qword ptr [rbp + 0xd18]
1409d2ae2: e8898af9ff call 0x14096b570
1409d2ae7: 41b8f8000000 mov r8d, 0xf8
1409d2aed: 4c89f9 mov rcx, r15
1409d2af0: 488d95b0030000 lea rdx, [rbp + 0x3b0]
1409d2af7: e80411f501 call 0x142923c00
1409d2afc: 488b85180e0000 mov rax, qword ptr [rbp + 0xe18]
1409d2b03: 488985800a0000 mov qword ptr [rbp + 0xa80], rax
1409d2b0a: c785780b000002000000 mov dword ptr [rbp + 0xb78], 2
1409d2b14: 4c89742420 mov qword ptr [rsp + 0x20], r14
1409d2b19: 4889f1 mov rcx, rsi
1409d2b1c: 4c89fa mov rdx, r15
1409d2b1f: 49b8d7745702af6c3eba movabs r8, 0xba3e6caf025774d7
1409d2b29: 41b11d mov r9b, 0x1d
1409d2b2c: e8cfbcf1ff call 0x1408ee800
1409d2b31: 41b8f8000000 mov r8d, 0xf8
1409d2b37: 4c89f1 mov rcx, r14
1409d2b3a: 4c89fa mov rdx, r15
1409d2b3d: e8be10f501 call 0x142923c00
1409d2b42: 488b85100e0000 mov rax, qword ptr [rbp + 0xe10]
1409d2b49: 488985800a0000 mov qword ptr [rbp + 0xa80], rax
1409d2b50: c7859806000002000000 mov dword ptr [rbp + 0x698], 2
1409d2b5a: 4c897c2420 mov qword ptr [rsp + 0x20], r15
1409d2b5f: 4889f1 mov rcx, rsi
1409d2b62: 4c89f2 mov rdx, r14
1409d2b65: 49b8aa9d74979fa58dfe movabs r8, 0xfe8da59f97749daa
1409d2b6f: 41b11d mov r9b, 0x1d
1409d2b72: e889bcf1ff call 0x1408ee800
1409d2b77: 41b8f8000000 mov r8d, 0xf8
1409d2b7d: 4c89f9 mov rcx, r15
1409d2b80: 4c89f2 mov rdx, r14
1409d2b83: e87810f501 call 0x142923c00
1409d2b88: 488b85000d0000 mov rax, qword ptr [rbp + 0xd00]
1409d2b8f: 488985800a0000 mov qword ptr [rbp + 0xa80], rax
1409d2b96: c785780b000002000000 mov dword ptr [rbp + 0xb78], 2
1409d2ba0: 4c89742420 mov qword ptr [rsp + 0x20], r14
1409d2ba5: 4889f1 mov rcx, rsi
1409d2ba8: 4c89fa mov rdx, r15
1409d2bab: 49b8f19da22b99dd360d movabs r8, 0xd36dd992ba29df1
1409d2bb5: 41b11d mov r9b, 0x1d
1409d2bb8: e843bcf1ff call 0x1408ee800
1409d2bbd: 41b8f8000000 mov r8d, 0xf8
1409d2bc3: 4c89f1 mov rcx, r14
1409d2bc6: 4c89fa mov rdx, r15
1409d2bc9: e83210f501 call 0x142923c00
1409d2bce: 488b85080d0000 mov rax, qword ptr [rbp + 0xd08]
1409d2bd5: 488985800a0000 mov qword ptr [rbp + 0xa80], rax
1409d2bdc: c7859806000002000000 mov dword ptr [rbp + 0x698], 2
1409d2be6: 4c897c2420 mov qword ptr [rsp + 0x20], r15
1409d2beb: 4889f1 mov rcx, rsi
1409d2bee: 4c89f2 mov rdx, r14
1409d2bf1: 49b8f04df136d28b1b4b movabs r8, 0x4b1b8bd236f14df0
1409d2bfb: 41b11d mov r9b, 0x1d
1409d2bfe: e8fdbbf1ff call 0x1408ee800
1409d2c03: 41b8f8000000 mov r8d, 0xf8
1409d2c09: 4c89f9 mov rcx, r15
1409d2c0c: 4c89f2 mov rdx, r14
1409d2c0f: e8ec0ff501 call 0x142923c00
1409d2c14: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d2c1b: 4c89f1 mov rcx, r14
1409d2c1e: 4c89fa mov rdx, r15
1409d2c21: 49b8c315a90b0cd85abc movabs r8, 0xbc5ad80c0ba915c3
1409d2c2b: e860dec500 call 0x141630a90
1409d2c30: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d2c37: 4c89f9 mov rcx, r15
1409d2c3a: 4c89f2 mov rdx, r14
1409d2c3d: 49b8ad2a9920eb336b0c movabs r8, 0xc6b33eb20992aad
1409d2c47: e8447bc500 call 0x14162a790
1409d2c4c: 488b85280e0000 mov rax, qword ptr [rbp + 0xe28]
1409d2c53: 488985800a0000 mov qword ptr [rbp + 0xa80], rax
1409d2c5a: c785780b000002000000 mov dword ptr [rbp + 0xb78], 2
1409d2c64: 4c89742420 mov qword ptr [rsp + 0x20], r14
1409d2c69: 4889f1 mov rcx, rsi
1409d2c6c: 4c89fa mov rdx, r15
1409d2c6f: 49b8fe6f11267034675a movabs r8, 0x5a67347026116ffe
1409d2c79: 41b11d mov r9b, 0x1d
1409d2c7c: e87fbbf1ff call 0x1408ee800
1409d2c81: 41b8f8000000 mov r8d, 0xf8
1409d2c87: 4c89f1 mov rcx, r14
1409d2c8a: 4c89fa mov rdx, r15
1409d2c8d: e86e0ff501 call 0x142923c00
1409d2c92: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d2c99: 4c89f9 mov rcx, r15
1409d2c9c: 4c89f2 mov rdx, r14
1409d2c9f: 49b8e1c8879028acd02b movabs r8, 0x2bd0ac289087c8e1
1409d2ca9: 4c8b8d300e0000 mov r9, qword ptr [rbp + 0xe30]
1409d2cb0: e89b34f3ff call 0x140906150
1409d2cb5: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d2cbc: 4c89f1 mov rcx, r14
1409d2cbf: 4c89fa mov rdx, r15
1409d2cc2: 49b8a006548dce019f12 movabs r8, 0x129f01ce8d5406a0
1409d2ccc: 4c8b8d100d0000 mov r9, qword ptr [rbp + 0xd10]
1409d2cd3: e87834f3ff call 0x140906150
1409d2cd8: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d2cdf: 4c89f9 mov rcx, r15
1409d2ce2: 4c89f2 mov rdx, r14
1409d2ce5: e8065ac500 call 0x1416286f0
1409d2cea: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d2cf1: 4c89f1 mov rcx, r14
1409d2cf4: 4c89fa mov rdx, r15
1409d2cf7: 4c8b85280d0000 mov r8, qword ptr [rbp + 0xd28]
1409d2cfe: e8cd5cc500 call 0x1416289d0
1409d2d03: c685800a000009 mov byte ptr [rbp + 0xa80], 9
1409d2d0a: c7859806000002000000 mov dword ptr [rbp + 0x698], 2
1409d2d14: 4c897c2420 mov qword ptr [rsp + 0x20], r15
1409d2d19: 4889f1 mov rcx, rsi
1409d2d1c: 4c89f2 mov rdx, r14
1409d2d1f: 49b8f2c570045e8bb061 movabs r8, 0x61b08b5e0470c5f2
1409d2d29: 41b11d mov r9b, 0x1d
1409d2d2c: e81f5ff3ff call 0x140908c50
1409d2d31: 41b8f8000000 mov r8d, 0xf8
1409d2d37: 4c89f9 mov rcx, r15
1409d2d3a: 4c89f2 mov rdx, r14
1409d2d3d: e8be0ef501 call 0x142923c00
1409d2d42: c685a80400000a mov byte ptr [rbp + 0x4a8], 0xa
1409d2d49: c785800a000002000000 mov dword ptr [rbp + 0xa80], 2
1409d2d53: 4889742420 mov qword ptr [rsp + 0x20], rsi
1409d2d58: 488d8da8040000 lea rcx, [rbp + 0x4a8]
1409d2d5f: 4c89fa mov rdx, r15
1409d2d62: 49b857f6e8da4c1e7d29 movabs r8, 0x297d1e4cdae8f657
1409d2d6c: 41b11d mov r9b, 0x1d
1409d2d6f: e8dc5ef3ff call 0x140908c50
1409d2d74: 41b8f8000000 mov r8d, 0xf8
1409d2d7a: 488dbda0050000 lea rdi, [rbp + 0x5a0]
1409d2d81: 4889f9 mov rcx, rdi
1409d2d84: 4c89fa mov rdx, r15
1409d2d87: e8740ef501 call 0x142923c00
1409d2d8c: c685780b000000 mov byte ptr [rbp + 0xb78], 0
1409d2d93: c7859806000002000000 mov dword ptr [rbp + 0x698], 2
1409d2d9d: 4c897c2420 mov qword ptr [rsp + 0x20], r15
1409d2da2: 4c89f1 mov rcx, r14
1409d2da5: 4889fa mov rdx, rdi
1409d2da8: 49b84541256a8a1a9db7 movabs r8, 0xb79d1a8a6a254145
1409d2db2: 41b11d mov r9b, 0x1d
1409d2db5: e8965ef3ff call 0x140908c50
1409d2dba: 488bbda0050000 mov rdi, qword ptr [rbp + 0x5a0]
1409d2dc1: 41b8a0000000 mov r8d, 0xa0
1409d2dc7: 488d8da0060000 lea rcx, [rbp + 0x6a0]
1409d2dce: 488d9da8050000 lea rbx, [rbp + 0x5a8]
1409d2dd5: 4889da mov rdx, rbx
1409d2dd8: e8230ef501 call 0x142923c00
1409d2ddd: 0fb68548060000 movzx eax, byte ptr [rbp + 0x648]
1409d2de4: 0fb68d49060000 movzx ecx, byte ptr [rbp + 0x649]
1409d2deb: 0fb6954a060000 movzx edx, byte ptr [rbp + 0x64a]
1409d2df2: 440fb6854b060000 movzx r8d, byte ptr [rbp + 0x64b]
1409d2dfa: 0f1083e0000000 movups xmm0, xmmword ptr [rbx + 0xe0]
1409d2e01: 4c8d8dc0060000 lea r9, [rbp + 0x6c0]
1409d2e08: 410f1181c0000000 movups xmmword ptr [r9 + 0xc0], xmm0
1409d2e10: 0f1083a4000000 movups xmm0, xmmword ptr [rbx + 0xa4]
1409d2e17: 0f108bb4000000 movups xmm1, xmmword ptr [rbx + 0xb4]
1409d2e1e: 0f1093c4000000 movups xmm2, xmmword ptr [rbx + 0xc4]
1409d2e25: 0f109bd4000000 movups xmm3, xmmword ptr [rbx + 0xd4]
1409d2e2c: 410f1199b4000000 movups xmmword ptr [r9 + 0xb4], xmm3
1409d2e34: 410f1191a4000000 movups xmmword ptr [r9 + 0xa4], xmm2
1409d2e3c: 410f118994000000 movups xmmword ptr [r9 + 0x94], xmm1
1409d2e44: 410f118184000000 movups xmmword ptr [r9 + 0x84], xmm0
1409d2e4c: 247f and al, 0x7f
1409d2e4e: 80c901 or cl, 1
1409d2e51: 80e203 and dl, 3
1409d2e54: 80cae8 or dl, 0xe8
1409d2e57: 4180e0e0 and r8b, 0xe0
1409d2e5b: 4180c810 or r8b, 0x10
1409d2e5f: 4889bd98060000 mov qword ptr [rbp + 0x698], rdi
1409d2e66: 888540070000 mov byte ptr [rbp + 0x740], al
1409d2e6c: 888d41070000 mov byte ptr [rbp + 0x741], cl
1409d2e72: 889542070000 mov byte ptr [rbp + 0x742], dl
1409d2e78: 44888543070000 mov byte ptr [rbp + 0x743], r8b
1409d2e7f: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d2e86: ba03000000 mov edx, 3
1409d2e8b: 4c89f9 mov rcx, r15
1409d2e8e: e8fd3bf3ff call 0x140906a90
1409d2e93: 4c89e0 mov rax, r12
1409d2e96: 488b95300d0000 mov rdx, qword ptr [rbp + 0xd30]
1409d2e9d: 4883f805 cmp rax, 5
1409d2ea1: 0f85d9f5ffff jne 0x1409d2480
1409d2ea7: 488db598060000 lea rsi, [rbp + 0x698]
1409d2eae: 488d55c8 lea rdx, [rbp - 0x38]
1409d2eb2: 41b8e8030000 mov r8d, 0x3e8
1409d2eb8: 4889f1 mov rcx, rsi
1409d2ebb: e8400df501 call 0x142923c00
1409d2ec0: c685570d000000 mov byte ptr [rbp + 0xd57], 0
1409d2ec7: 4889f1 mov rcx, rsi
1409d2eca: e8a154e9ff call 0x140868370
1409d2ecf: 0f28b5600d0000 movaps xmm6, xmmword ptr [rbp + 0xd60]
1409d2ed6: 0f28bd700d0000 movaps xmm7, xmmword ptr [rbp + 0xd70]
1409d2edd: 4881c4080e0000 add rsp, 0xe08
1409d2ee4: 5b pop rbx
1409d2ee5: 5f pop rdi
1409d2ee6: 5e pop rsi
1409d2ee7: 415c pop r12
1409d2ee9: 415d pop r13
1409d2eeb: 415e pop r14
1409d2eed: 415f pop r15
1409d2eef: 5d pop rbp
1409d2ef0: c3 ret 
1409d2ef1: 48c7859806000000000000 mov qword ptr [rbp + 0x698], 0
1409d2efc: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d2f03: 488d0576b22502 lea rax, [rip + 0x225b276]
1409d2f0a: 4889442420 mov qword ptr [rsp + 0x20], rax
1409d2f0f: 488d95780b0000 lea rdx, [rbp + 0xb78]
1409d2f16: 4c8d8d98060000 lea r9, [rbp + 0x698]
1409d2f1d: 31c9 xor ecx, ecx
1409d2f1f: 4989f8 mov r8, rdi
1409d2f22: e81a7cef01 call 0x1428cab41
1409d2f27: e99e000000 jmp 0x1409d2fca
1409d2f2c: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d2f33: 4c8d059ee90e02 lea r8, [rip + 0x20ee99e]
1409d2f3a: b905000000 mov ecx, 5
1409d2f3f: ba05000000 mov edx, 5
1409d2f44: e8cf65f001 call 0x1428d9518
1409d2f49: eb7f jmp 0x1409d2fca
1409d2f4b: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d2f52: 488d0d97e90e02 lea rcx, [rip + 0x20ee997]
1409d2f59: e8b263f001 call 0x1428d9310
1409d2f5e: eb6a jmp 0x1409d2fca
1409d2f60: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d2f67: 488d0d327b2602 lea rcx, [rip + 0x2267b32]
1409d2f6e: e85d51f001 call 0x1428d80d0
1409d2f73: eb55 jmp 0x1409d2fca
1409d2f75: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d2f7c: 488d0d1d7b2602 lea rcx, [rip + 0x2267b1d]
1409d2f83: e84851f001 call 0x1428d80d0
1409d2f88: eb40 jmp 0x1409d2fca
1409d2f8a: 488d0dcfe80e02 lea rcx, [rip + 0x20ee8cf]
1409d2f91: e87a63f001 call 0x1428d9310
1409d2f96: 488d0ddbe80e02 lea rcx, [rip + 0x20ee8db]
1409d2f9d: e86e63f001 call 0x1428d9310
1409d2fa2: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d2fa9: 488d0df8e80e02 lea rcx, [rip + 0x20ee8f8]
1409d2fb0: e85b63f001 call 0x1428d9310
1409d2fb5: eb13 jmp 0x1409d2fca
1409d2fb7: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d2fbe: 488d0dfbe80e02 lea rcx, [rip + 0x20ee8fb]
1409d2fc5: e84663f001 call 0x1428d9310
1409d2fca: 0f0b ud2 
1409d2fcc: 0f1f4000 nop dword ptr [rax]
1409d2fd0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409d2fd5: 55 push rbp
1409d2fd6: 4157 push r15
1409d2fd8: 4156 push r14
1409d2fda: 4155 push r13
1409d2fdc: 4154 push r12
1409d2fde: 56 push rsi
1409d2fdf: 57 push rdi
1409d2fe0: 53 push rbx
1409d2fe1: 4883ec68 sub rsp, 0x68
1409d2fe5: 488daa80000000 lea rbp, [rdx + 0x80]
1409d2fec: 0f297c2440 movaps xmmword ptr [rsp + 0x40], xmm7
1409d2ff1: 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
1409d2ff6: 80bd560d000000 cmp byte ptr [rbp + 0xd56], 0
1409d2ffd: 742e je 0x1409d302d
1409d2fff: 488d8d98060000 lea rcx, [rbp + 0x698]
1409d3006: e8b505ecff call 0x1408935c0
1409d300b: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d3012: 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
1409d3017: 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
1409d301c: 4883c468 add rsp, 0x68
1409d3020: 5b pop rbx
1409d3021: 5f pop rdi
1409d3022: 5e pop rsi
1409d3023: 415c pop r12
1409d3025: 415d pop r13
1409d3027: 415e pop r14
1409d3029: 415f pop r15
1409d302b: 5d pop rbp
1409d302c: c3 ret 
1409d302d: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d3034: 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
1409d3039: 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
1409d303e: 4883c468 add rsp, 0x68
1409d3042: 5b pop rbx
1409d3043: 5f pop rdi
1409d3044: 5e pop rsi
1409d3045: 415c pop r12
1409d3047: 415d pop r13
1409d3049: 415e pop r14
1409d304b: 415f pop r15
1409d304d: 5d pop rbp
1409d304e: c3 ret 
1409d304f: 90 nop 
1409d3050: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409d3055: 55 push rbp
1409d3056: 4157 push r15
1409d3058: 4156 push r14
1409d305a: 4155 push r13
1409d305c: 4154 push r12
1409d305e: 56 push rsi
1409d305f: 57 push rdi
1409d3060: 53 push rbx
1409d3061: 4883ec68 sub rsp, 0x68
1409d3065: 488daa80000000 lea rbp, [rdx + 0x80]
1409d306c: 0f297c2440 movaps xmmword ptr [rsp + 0x40], xmm7
1409d3071: 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
1409d3076: 80bd570d000000 cmp byte ptr [rbp + 0xd57], 0
1409d307d: 751b jne 0x1409d309a
1409d307f: 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
1409d3084: 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
1409d3089: 4883c468 add rsp, 0x68
1409d308d: 5b pop rbx
1409d308e: 5f pop rdi
1409d308f: 5e pop rsi
1409d3090: 415c pop r12
1409d3092: 415d pop r13
1409d3094: 415e pop r14
1409d3096: 415f pop r15
1409d3098: 5d pop rbp
1409d3099: c3 ret 
1409d309a: 488d4dc8 lea rcx, [rbp - 0x38]
1409d309e: e81df3ebff call 0x1408923c0
1409d30a3: ebda jmp 0x1409d307f
1409d30a5: 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
1409d30b0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409d30b5: 55 push rbp
1409d30b6: 4157 push r15
1409d30b8: 4156 push r14
1409d30ba: 4155 push r13
1409d30bc: 4154 push r12
1409d30be: 56 push rsi
1409d30bf: 57 push rdi
1409d30c0: 53 push rbx
1409d30c1: 4883ec68 sub rsp, 0x68
1409d30c5: 488daa80000000 lea rbp, [rdx + 0x80]
1409d30cc: 0f297c2440 movaps xmmword ptr [rsp + 0x40], xmm7
1409d30d1: 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
1409d30d6: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d30dd: 488d8d98060000 lea rcx, [rbp + 0x698]
1409d30e4: e8d704ecff call 0x1408935c0
1409d30e9: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d30f0: 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
1409d30f5: 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
1409d30fa: 4883c468 add rsp, 0x68
1409d30fe: 5b pop rbx
1409d30ff: 5f pop rdi
1409d3100: 5e pop rsi
1409d3101: 415c pop r12
1409d3103: 415d pop r13
1409d3105: 415e pop r14
1409d3107: 415f pop r15
1409d3109: 5d pop rbp
1409d310a: c3 ret 
1409d310b: 0f1f440000 nop dword ptr [rax + rax]
1409d3110: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409d3115: 55 push rbp
1409d3116: 4157 push r15
1409d3118: 4156 push r14
1409d311a: 4155 push r13
1409d311c: 4154 push r12
1409d311e: 56 push rsi
1409d311f: 57 push rdi
1409d3120: 53 push rbx
1409d3121: 4883ec68 sub rsp, 0x68
1409d3125: 488daa80000000 lea rbp, [rdx + 0x80]
1409d312c: 0f297c2440 movaps xmmword ptr [rsp + 0x40], xmm7
1409d3131: 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
1409d3136: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d313d: 488d8d780b0000 lea rcx, [rbp + 0xb78]
1409d3144: e87704ecff call 0x1408935c0
1409d3149: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d3150: 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
1409d3155: 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
1409d315a: 4883c468 add rsp, 0x68
1409d315e: 5b pop rbx
1409d315f: 5f pop rdi
1409d3160: 5e pop rsi
1409d3161: 415c pop r12
1409d3163: 415d pop r13
1409d3165: 415e pop r14
1409d3167: 415f pop r15
1409d3169: 5d pop rbp
1409d316a: c3 ret 
1409d316b: 0f1f440000 nop dword ptr [rax + rax]
1409d3170: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409d3175: 55 push rbp
1409d3176: 4157 push r15
1409d3178: 4156 push r14
1409d317a: 4155 push r13
1409d317c: 4154 push r12
1409d317e: 56 push rsi
1409d317f: 57 push rdi
1409d3180: 53 push rbx
1409d3181: 4883ec68 sub rsp, 0x68
1409d3185: 488daa80000000 lea rbp, [rdx + 0x80]
1409d318c: 0f297c2440 movaps xmmword ptr [rsp + 0x40], xmm7
1409d3191: 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
1409d3196: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d319d: 488d8d98060000 lea rcx, [rbp + 0x698]
1409d31a4: e81704ecff call 0x1408935c0
1409d31a9: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d31b0: 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
1409d31b5: 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
1409d31ba: 4883c468 add rsp, 0x68
1409d31be: 5b pop rbx
1409d31bf: 5f pop rdi
1409d31c0: 5e pop rsi
1409d31c1: 415c pop r12
1409d31c3: 415d pop r13
1409d31c5: 415e pop r14
1409d31c7: 415f pop r15
1409d31c9: 5d pop rbp
1409d31ca: c3 ret 
1409d31cb: 0f1f440000 nop dword ptr [rax + rax]
1409d31d0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409d31d5: 55 push rbp
1409d31d6: 4157 push r15
1409d31d8: 4156 push r14
1409d31da: 4155 push r13
1409d31dc: 4154 push r12
1409d31de: 56 push rsi
1409d31df: 57 push rdi
1409d31e0: 53 push rbx
1409d31e1: 4883ec68 sub rsp, 0x68
1409d31e5: 488daa80000000 lea rbp, [rdx + 0x80]
1409d31ec: 0f297c2440 movaps xmmword ptr [rsp + 0x40], xmm7
1409d31f1: 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
1409d31f6: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d31fd: 488d8d780b0000 lea rcx, [rbp + 0xb78]
1409d3204: e8b703ecff call 0x1408935c0
1409d3209: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d3210: 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
1409d3215: 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
1409d321a: 4883c468 add rsp, 0x68
1409d321e: 5b pop rbx
1409d321f: 5f pop rdi
1409d3220: 5e pop rsi
1409d3221: 415c pop r12
1409d3223: 415d pop r13
1409d3225: 415e pop r14
1409d3227: 415f pop r15
1409d3229: 5d pop rbp
1409d322a: c3 ret 
1409d322b: 0f1f440000 nop dword ptr [rax + rax]
1409d3230: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409d3235: 55 push rbp
1409d3236: 4157 push r15
1409d3238: 4156 push r14
1409d323a: 4155 push r13
1409d323c: 4154 push r12
1409d323e: 56 push rsi
1409d323f: 57 push rdi
1409d3240: 53 push rbx
1409d3241: 4883ec68 sub rsp, 0x68
1409d3245: 488daa80000000 lea rbp, [rdx + 0x80]
1409d324c: 0f297c2440 movaps xmmword ptr [rsp + 0x40], xmm7
1409d3251: 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
1409d3256: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d325d: 488d8d98060000 lea rcx, [rbp + 0x698]
1409d3264: e85703ecff call 0x1408935c0
1409d3269: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d3270: 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
1409d3275: 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
1409d327a: 4883c468 add rsp, 0x68
1409d327e: 5b pop rbx
1409d327f: 5f pop rdi
1409d3280: 5e pop rsi
1409d3281: 415c pop r12
1409d3283: 415d pop r13
1409d3285: 415e pop r14
1409d3287: 415f pop r15
1409d3289: 5d pop rbp
1409d328a: c3 ret 
1409d328b: 0f1f440000 nop dword ptr [rax + rax]
1409d3290: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409d3295: 55 push rbp
1409d3296: 4157 push r15
1409d3298: 4156 push r14
1409d329a: 4155 push r13
1409d329c: 4154 push r12
1409d329e: 56 push rsi
1409d329f: 57 push rdi
1409d32a0: 53 push rbx
1409d32a1: 4883ec68 sub rsp, 0x68
1409d32a5: 488daa80000000 lea rbp, [rdx + 0x80]
1409d32ac: 0f297c2440 movaps xmmword ptr [rsp + 0x40], xmm7
1409d32b1: 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
1409d32b6: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d32bd: 488d8d780b0000 lea rcx, [rbp + 0xb78]
1409d32c4: e8f702ecff call 0x1408935c0
1409d32c9: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d32d0: 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
1409d32d5: 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
1409d32da: 4883c468 add rsp, 0x68
1409d32de: 5b pop rbx
1409d32df: 5f pop rdi
1409d32e0: 5e pop rsi
1409d32e1: 415c pop r12
1409d32e3: 415d pop r13
1409d32e5: 415e pop r14
1409d32e7: 415f pop r15
1409d32e9: 5d pop rbp
1409d32ea: c3 ret 
1409d32eb: 0f1f440000 nop dword ptr [rax + rax]
1409d32f0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409d32f5: 55 push rbp
1409d32f6: 4157 push r15
1409d32f8: 4156 push r14
1409d32fa: 4155 push r13
1409d32fc: 4154 push r12
1409d32fe: 56 push rsi
1409d32ff: 57 push rdi
1409d3300: 53 push rbx
1409d3301: 4883ec68 sub rsp, 0x68
1409d3305: 488daa80000000 lea rbp, [rdx + 0x80]
1409d330c: 0f297c2440 movaps xmmword ptr [rsp + 0x40], xmm7
1409d3311: 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
1409d3316: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d331d: 488d8d98060000 lea rcx, [rbp + 0x698]
1409d3324: e89702ecff call 0x1408935c0
1409d3329: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d3330: 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
1409d3335: 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
1409d333a: 4883c468 add rsp, 0x68
1409d333e: 5b pop rbx
1409d333f: 5f pop rdi
1409d3340: 5e pop rsi
1409d3341: 415c pop r12
1409d3343: 415d pop r13
1409d3345: 415e pop r14
1409d3347: 415f pop r15
1409d3349: 5d pop rbp
1409d334a: c3 ret 
1409d334b: 0f1f440000 nop dword ptr [rax + rax]
1409d3350: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409d3355: 55 push rbp
1409d3356: 4157 push r15
1409d3358: 4156 push r14
1409d335a: 4155 push r13
1409d335c: 4154 push r12
1409d335e: 56 push rsi
1409d335f: 57 push rdi
1409d3360: 53 push rbx
1409d3361: 4883ec68 sub rsp, 0x68
1409d3365: 488daa80000000 lea rbp, [rdx + 0x80]
1409d336c: 0f297c2440 movaps xmmword ptr [rsp + 0x40], xmm7
1409d3371: 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
1409d3376: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d337d: 488d8d780b0000 lea rcx, [rbp + 0xb78]
1409d3384: e83702ecff call 0x1408935c0
1409d3389: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d3390: 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
1409d3395: 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
1409d339a: 4883c468 add rsp, 0x68
1409d339e: 5b pop rbx
1409d339f: 5f pop rdi
1409d33a0: 5e pop rsi
1409d33a1: 415c pop r12
1409d33a3: 415d pop r13
1409d33a5: 415e pop r14
1409d33a7: 415f pop r15
1409d33a9: 5d pop rbp
1409d33aa: c3 ret 
1409d33ab: 0f1f440000 nop dword ptr [rax + rax]
1409d33b0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409d33b5: 55 push rbp
1409d33b6: 4157 push r15
1409d33b8: 4156 push r14
1409d33ba: 4155 push r13
1409d33bc: 4154 push r12
1409d33be: 56 push rsi
1409d33bf: 57 push rdi
1409d33c0: 53 push rbx
1409d33c1: 4883ec68 sub rsp, 0x68
1409d33c5: 488daa80000000 lea rbp, [rdx + 0x80]
1409d33cc: 0f297c2440 movaps xmmword ptr [rsp + 0x40], xmm7
1409d33d1: 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
1409d33d6: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d33dd: 488d8d98060000 lea rcx, [rbp + 0x698]
1409d33e4: e8d701ecff call 0x1408935c0
1409d33e9: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d33f0: 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
1409d33f5: 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
1409d33fa: 4883c468 add rsp, 0x68
1409d33fe: 5b pop rbx
1409d33ff: 5f pop rdi
1409d3400: 5e pop rsi
1409d3401: 415c pop r12
1409d3403: 415d pop r13
1409d3405: 415e pop r14
1409d3407: 415f pop r15
1409d3409: 5d pop rbp
1409d340a: c3 ret 
1409d340b: 0f1f440000 nop dword ptr [rax + rax]
1409d3410: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409d3415: 55 push rbp
1409d3416: 4157 push r15
1409d3418: 4156 push r14
1409d341a: 4155 push r13
1409d341c: 4154 push r12
1409d341e: 56 push rsi
1409d341f: 57 push rdi
1409d3420: 53 push rbx
1409d3421: 4883ec68 sub rsp, 0x68
1409d3425: 488daa80000000 lea rbp, [rdx + 0x80]
1409d342c: 0f297c2440 movaps xmmword ptr [rsp + 0x40], xmm7
1409d3431: 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
1409d3436: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d343d: 488d8d800a0000 lea rcx, [rbp + 0xa80]
1409d3444: e87701ecff call 0x1408935c0
1409d3449: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d3450: 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
1409d3455: 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
1409d345a: 4883c468 add rsp, 0x68
1409d345e: 5b pop rbx
1409d345f: 5f pop rdi
1409d3460: 5e pop rsi
1409d3461: 415c pop r12
1409d3463: 415d pop r13
1409d3465: 415e pop r14
1409d3467: 415f pop r15
1409d3469: 5d pop rbp
1409d346a: c3 ret 
1409d346b: 0f1f440000 nop dword ptr [rax + rax]
1409d3470: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409d3475: 55 push rbp
1409d3476: 4157 push r15
1409d3478: 4156 push r14
1409d347a: 4155 push r13
1409d347c: 4154 push r12
1409d347e: 56 push rsi
1409d347f: 57 push rdi
1409d3480: 53 push rbx
1409d3481: 4883ec68 sub rsp, 0x68
1409d3485: 488daa80000000 lea rbp, [rdx + 0x80]
1409d348c: 0f297c2440 movaps xmmword ptr [rsp + 0x40], xmm7
1409d3491: 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
1409d3496: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d349d: 488d8d98060000 lea rcx, [rbp + 0x698]
1409d34a4: e81701ecff call 0x1408935c0
1409d34a9: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d34b0: 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
1409d34b5: 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
1409d34ba: 4883c468 add rsp, 0x68
1409d34be: 5b pop rbx
1409d34bf: 5f pop rdi
1409d34c0: 5e pop rsi
1409d34c1: 415c pop r12
1409d34c3: 415d pop r13
1409d34c5: 415e pop r14
1409d34c7: 415f pop r15
1409d34c9: 5d pop rbp
1409d34ca: c3 ret 
1409d34cb: 0f1f440000 nop dword ptr [rax + rax]
1409d34d0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409d34d5: 55 push rbp
1409d34d6: 4157 push r15
1409d34d8: 4156 push r14
1409d34da: 4155 push r13
1409d34dc: 4154 push r12
1409d34de: 56 push rsi
1409d34df: 57 push rdi
1409d34e0: 53 push rbx
1409d34e1: 4883ec68 sub rsp, 0x68
1409d34e5: 488daa80000000 lea rbp, [rdx + 0x80]
1409d34ec: 0f297c2440 movaps xmmword ptr [rsp + 0x40], xmm7
1409d34f1: 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
1409d34f6: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d34fd: 488d8d780b0000 lea rcx, [rbp + 0xb78]
1409d3504: e8b700ecff call 0x1408935c0
1409d3509: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d3510: 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
1409d3515: 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
1409d351a: 4883c468 add rsp, 0x68
1409d351e: 5b pop rbx
1409d351f: 5f pop rdi
1409d3520: 5e pop rsi
1409d3521: 415c pop r12
1409d3523: 415d pop r13
1409d3525: 415e pop r14
1409d3527: 415f pop r15
1409d3529: 5d pop rbp
1409d352a: c3 ret 
1409d352b: 0f1f440000 nop dword ptr [rax + rax]
1409d3530: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409d3535: 55 push rbp
1409d3536: 4157 push r15
1409d3538: 4156 push r14
1409d353a: 4155 push r13
1409d353c: 4154 push r12
1409d353e: 56 push rsi
1409d353f: 57 push rdi
1409d3540: 53 push rbx
1409d3541: 4883ec68 sub rsp, 0x68
1409d3545: 488daa80000000 lea rbp, [rdx + 0x80]
1409d354c: 0f297c2440 movaps xmmword ptr [rsp + 0x40], xmm7
1409d3551: 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
1409d3556: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d355d: 488d8da8040000 lea rcx, [rbp + 0x4a8]
1409d3564: e85700ecff call 0x1408935c0
1409d3569: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d3570: 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
1409d3575: 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
1409d357a: 4883c468 add rsp, 0x68
1409d357e: 5b pop rbx
1409d357f: 5f pop rdi
1409d3580: 5e pop rsi
1409d3581: 415c pop r12
1409d3583: 415d pop r13
1409d3585: 415e pop r14
1409d3587: 415f pop r15
1409d3589: 5d pop rbp
1409d358a: c3 ret 
1409d358b: 0f1f440000 nop dword ptr [rax + rax]
1409d3590: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409d3595: 55 push rbp
1409d3596: 4157 push r15
1409d3598: 4156 push r14
1409d359a: 4155 push r13
1409d359c: 4154 push r12
1409d359e: 56 push rsi
1409d359f: 57 push rdi
1409d35a0: 53 push rbx
1409d35a1: 4883ec68 sub rsp, 0x68
1409d35a5: 488daa80000000 lea rbp, [rdx + 0x80]
1409d35ac: 0f297c2440 movaps xmmword ptr [rsp + 0x40], xmm7
1409d35b1: 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
1409d35b6: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d35bd: 488d8db0030000 lea rcx, [rbp + 0x3b0]
1409d35c4: e8f7ffebff call 0x1408935c0
1409d35c9: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d35d0: 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
1409d35d5: 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
1409d35da: 4883c468 add rsp, 0x68
1409d35de: 5b pop rbx
1409d35df: 5f pop rdi
1409d35e0: 5e pop rsi
1409d35e1: 415c pop r12
1409d35e3: 415d pop r13
1409d35e5: 415e pop r14
1409d35e7: 415f pop r15
1409d35e9: 5d pop rbp
1409d35ea: c3 ret 
1409d35eb: 0f1f440000 nop dword ptr [rax + rax]
1409d35f0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409d35f5: 55 push rbp
1409d35f6: 4157 push r15
1409d35f8: 4156 push r14
1409d35fa: 4155 push r13
1409d35fc: 4154 push r12
1409d35fe: 56 push rsi
1409d35ff: 57 push rdi
1409d3600: 53 push rbx
1409d3601: 4883ec68 sub rsp, 0x68
1409d3605: 488daa80000000 lea rbp, [rdx + 0x80]
1409d360c: 0f297c2440 movaps xmmword ptr [rsp + 0x40], xmm7
1409d3611: 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
1409d3616: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d361d: 488d8d98060000 lea rcx, [rbp + 0x698]
1409d3624: e897ffebff call 0x1408935c0
1409d3629: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d3630: 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
1409d3635: 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
1409d363a: 4883c468 add rsp, 0x68
1409d363e: 5b pop rbx
1409d363f: 5f pop rdi
1409d3640: 5e pop rsi
1409d3641: 415c pop r12
1409d3643: 415d pop r13
1409d3645: 415e pop r14
1409d3647: 415f pop r15
1409d3649: 5d pop rbp
1409d364a: c3 ret 
1409d364b: 0f1f440000 nop dword ptr [rax + rax]
1409d3650: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409d3655: 55 push rbp
1409d3656: 4157 push r15
1409d3658: 4156 push r14
1409d365a: 4155 push r13
1409d365c: 4154 push r12
1409d365e: 56 push rsi
1409d365f: 57 push rdi
1409d3660: 53 push rbx
1409d3661: 4883ec68 sub rsp, 0x68
1409d3665: 488daa80000000 lea rbp, [rdx + 0x80]
1409d366c: 0f297c2440 movaps xmmword ptr [rsp + 0x40], xmm7
1409d3671: 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
1409d3676: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d367d: 488d8d780b0000 lea rcx, [rbp + 0xb78]
1409d3684: e837ffebff call 0x1408935c0
1409d3689: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d3690: 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
1409d3695: 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
1409d369a: 4883c468 add rsp, 0x68
1409d369e: 5b pop rbx
1409d369f: 5f pop rdi
1409d36a0: 5e pop rsi
1409d36a1: 415c pop r12
1409d36a3: 415d pop r13
1409d36a5: 415e pop r14
1409d36a7: 415f pop r15
1409d36a9: 5d pop rbp
1409d36aa: c3 ret 
1409d36ab: 0f1f440000 nop dword ptr [rax + rax]
1409d36b0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409d36b5: 55 push rbp
1409d36b6: 4157 push r15
1409d36b8: 4156 push r14
1409d36ba: 4155 push r13
1409d36bc: 4154 push r12
1409d36be: 56 push rsi
1409d36bf: 57 push rdi
1409d36c0: 53 push rbx
1409d36c1: 4883ec68 sub rsp, 0x68
1409d36c5: 488daa80000000 lea rbp, [rdx + 0x80]
1409d36cc: 0f297c2440 movaps xmmword ptr [rsp + 0x40], xmm7
1409d36d1: 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
1409d36d6: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d36dd: 488d8d98060000 lea rcx, [rbp + 0x698]
1409d36e4: e8d7feebff call 0x1408935c0
1409d36e9: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d36f0: 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
1409d36f5: 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
1409d36fa: 4883c468 add rsp, 0x68
1409d36fe: 5b pop rbx
1409d36ff: 5f pop rdi
1409d3700: 5e pop rsi
1409d3701: 415c pop r12
1409d3703: 415d pop r13
1409d3705: 415e pop r14
1409d3707: 415f pop r15
1409d3709: 5d pop rbp
1409d370a: c3 ret 
1409d370b: 0f1f440000 nop dword ptr [rax + rax]
1409d3710: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409d3715: 55 push rbp
1409d3716: 4157 push r15
1409d3718: 4156 push r14
1409d371a: 4155 push r13
1409d371c: 4154 push r12
1409d371e: 56 push rsi
1409d371f: 57 push rdi
1409d3720: 53 push rbx
1409d3721: 4883ec68 sub rsp, 0x68
1409d3725: 488daa80000000 lea rbp, [rdx + 0x80]
1409d372c: 0f297c2440 movaps xmmword ptr [rsp + 0x40], xmm7
1409d3731: 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
1409d3736: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d373d: 488d8d780b0000 lea rcx, [rbp + 0xb78]
1409d3744: e877feebff call 0x1408935c0
1409d3749: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d3750: 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
1409d3755: 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
1409d375a: 4883c468 add rsp, 0x68
1409d375e: 5b pop rbx
1409d375f: 5f pop rdi
1409d3760: 5e pop rsi
1409d3761: 415c pop r12
1409d3763: 415d pop r13
1409d3765: 415e pop r14
1409d3767: 415f pop r15
1409d3769: 5d pop rbp
1409d376a: c3 ret 
1409d376b: 0f1f440000 nop dword ptr [rax + rax]
1409d3770: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409d3775: 55 push rbp
1409d3776: 4157 push r15
1409d3778: 4156 push r14
1409d377a: 4155 push r13
1409d377c: 4154 push r12
1409d377e: 56 push rsi
1409d377f: 57 push rdi
1409d3780: 53 push rbx
1409d3781: 4883ec68 sub rsp, 0x68
1409d3785: 488daa80000000 lea rbp, [rdx + 0x80]
1409d378c: 0f297c2440 movaps xmmword ptr [rsp + 0x40], xmm7
1409d3791: 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
1409d3796: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d379d: 488d8d98060000 lea rcx, [rbp + 0x698]
1409d37a4: e817feebff call 0x1408935c0
1409d37a9: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d37b0: 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
1409d37b5: 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
1409d37ba: 4883c468 add rsp, 0x68
1409d37be: 5b pop rbx
1409d37bf: 5f pop rdi
1409d37c0: 5e pop rsi
1409d37c1: 415c pop r12
1409d37c3: 415d pop r13
1409d37c5: 415e pop r14
1409d37c7: 415f pop r15
1409d37c9: 5d pop rbp
1409d37ca: c3 ret 
1409d37cb: 0f1f440000 nop dword ptr [rax + rax]
1409d37d0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409d37d5: 55 push rbp
1409d37d6: 4157 push r15
1409d37d8: 4156 push r14
1409d37da: 4155 push r13
1409d37dc: 4154 push r12
1409d37de: 56 push rsi
1409d37df: 57 push rdi
1409d37e0: 53 push rbx
1409d37e1: 4883ec68 sub rsp, 0x68
1409d37e5: 488daa80000000 lea rbp, [rdx + 0x80]
1409d37ec: 0f297c2440 movaps xmmword ptr [rsp + 0x40], xmm7
1409d37f1: 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
1409d37f6: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d37fd: 488d8d780b0000 lea rcx, [rbp + 0xb78]
1409d3804: e8b7fdebff call 0x1408935c0
1409d3809: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d3810: 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
1409d3815: 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
1409d381a: 4883c468 add rsp, 0x68
1409d381e: 5b pop rbx
1409d381f: 5f pop rdi
1409d3820: 5e pop rsi
1409d3821: 415c pop r12
1409d3823: 415d pop r13
1409d3825: 415e pop r14
1409d3827: 415f pop r15
1409d3829: 5d pop rbp
1409d382a: c3 ret 
1409d382b: 0f1f440000 nop dword ptr [rax + rax]
1409d3830: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409d3835: 55 push rbp
1409d3836: 4157 push r15
1409d3838: 4156 push r14
1409d383a: 4155 push r13
1409d383c: 4154 push r12
1409d383e: 56 push rsi
1409d383f: 57 push rdi
1409d3840: 53 push rbx
1409d3841: 4883ec68 sub rsp, 0x68
1409d3845: 488daa80000000 lea rbp, [rdx + 0x80]
1409d384c: 0f297c2440 movaps xmmword ptr [rsp + 0x40], xmm7
1409d3851: 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
1409d3856: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d385d: 488d8d98060000 lea rcx, [rbp + 0x698]
1409d3864: e857fdebff call 0x1408935c0
1409d3869: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d3870: 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
1409d3875: 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
1409d387a: 4883c468 add rsp, 0x68
1409d387e: 5b pop rbx
1409d387f: 5f pop rdi
1409d3880: 5e pop rsi
1409d3881: 415c pop r12
1409d3883: 415d pop r13
1409d3885: 415e pop r14
1409d3887: 415f pop r15
1409d3889: 5d pop rbp
1409d388a: c3 ret 
1409d388b: 0f1f440000 nop dword ptr [rax + rax]
1409d3890: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409d3895: 55 push rbp
1409d3896: 4157 push r15
1409d3898: 4156 push r14
1409d389a: 4155 push r13
1409d389c: 4154 push r12
1409d389e: 56 push rsi
1409d389f: 57 push rdi
1409d38a0: 53 push rbx
1409d38a1: 4883ec68 sub rsp, 0x68
1409d38a5: 488daa80000000 lea rbp, [rdx + 0x80]
1409d38ac: 0f297c2440 movaps xmmword ptr [rsp + 0x40], xmm7
1409d38b1: 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
1409d38b6: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d38bd: 488d8da0050000 lea rcx, [rbp + 0x5a0]
1409d38c4: e8f7fcebff call 0x1408935c0
1409d38c9: c685570d000001 mov byte ptr [rbp + 0xd57], 1
1409d38d0: 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
1409d38d5: 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
1409d38da: 4883c468 add rsp, 0x68
1409d38de: 5b pop rbx
1409d38df: 5f pop rdi
1409d38e0: 5e pop rsi
1409d38e1: 415c pop r12
1409d38e3: 415d pop r13
1409d38e5: 415e pop r14
1409d38e7: 415f pop r15
1409d38e9: 5d pop rbp
1409d38ea: c3 ret 
