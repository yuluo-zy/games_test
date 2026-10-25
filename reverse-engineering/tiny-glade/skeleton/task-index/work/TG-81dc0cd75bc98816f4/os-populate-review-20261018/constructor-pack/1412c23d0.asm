1412c23d0: push rbp
1412c23d1: push rsi
1412c23d2: sub rsp, 0xd8
1412c23d9: lea rbp, [rsp + 0x80]
1412c23e1: mov qword ptr [rbp + 0x50], 0xfffffffffffffffe
1412c23e9: mov rsi, rcx
1412c23ec: lea rcx, [rbp - 0x58]
1412c23f0: xor r8d, r8d
1412c23f3: call 0x14161a020
1412c23f8: mov qword ptr [rbp - 0x60], 0
1412c2400: movzx eax, byte ptr [rip + 0x2372802]
1412c2407: mov ecx, 0x30
1412c240c: mov edx, 8
1412c2411: call 0x140613c10
1412c2416: test rax, rax
1412c2419: je 0x1412c24cf
1412c241f: movups xmm0, xmmword ptr [rbp - 0x60]
1412c2423: movups xmm1, xmmword ptr [rbp - 0x50]
1412c2427: movups xmm2, xmmword ptr [rbp - 0x40]
1412c242b: movups xmmword ptr [rax + 0x20], xmm2
1412c242f: movups xmmword ptr [rax + 0x10], xmm1
1412c2433: movups xmmword ptr [rax], xmm0
1412c2436: lea rcx, [rip + 0x17dad2e]
1412c243d: mov qword ptr [rbp], rcx
1412c2441: mov qword ptr [rbp + 8], 0x49
1412c2449: mov qword ptr [rbp + 0x10], rax
1412c244d: lea rax, [rip + 0x196a9b4]
1412c2454: mov qword ptr [rbp + 0x18], rax
1412c2458: mov qword ptr [rbp + 0x30], 0
1412c2460: mov byte ptr [rbp + 0x46], 5
1412c2464: mov qword ptr [rbp + 0x20], rcx
1412c2468: mov qword ptr [rbp + 0x28], 0x49
1412c2470: movzx eax, byte ptr [rip + 0x2372792]
1412c2477: mov ecx, 0x40
1412c247c: mov edx, 8
1412c2481: call 0x140613c10
1412c2486: test rax, rax
1412c2489: je 0x1412c24e0
1412c248b: movups xmm0, xmmword ptr [rbp + 0x10]
1412c248f: movups xmm1, xmmword ptr [rbp + 0x20]
1412c2493: movups xmm2, xmmword ptr [rbp + 0x30]
1412c2497: movups xmm3, xmmword ptr [rbp + 0x40]
1412c249b: movups xmmword ptr [rax + 0x30], xmm3
1412c249f: movups xmmword ptr [rax + 0x20], xmm2
1412c24a3: movups xmmword ptr [rax + 0x10], xmm1
1412c24a7: movups xmmword ptr [rax], xmm0
1412c24aa: movabs rdx, 0x4f69c51e83e1d670
1412c24b4: mov rcx, rsi
1412c24b7: mov r8, rax
1412c24ba: call 0x140969bf0
1412c24bf: cmp rax, 1
1412c24c3: je 0x1412c24f1
1412c24c5: add rsp, 0xd8
1412c24cc: pop rsi
1412c24cd: pop rbp
1412c24ce: ret 
1412c24cf: mov ecx, 8
1412c24d4: mov edx, 0x30
1412c24d9: call 0x1428d9000
1412c24de: jmp 0x1412c24ef
1412c24e0: mov ecx, 8
1412c24e5: mov edx, 0x40
1412c24ea: call 0x1428d9000
1412c24ef: ud2 
1412c24f1: mov rax, rdx
1412c24f4: add rdx, 0x10
1412c24f8: mov rsi, rdx
1412c24fb: cmp qword ptr [rax + 0x18], 0x49
1412c2500: jne 0x1412c257e
1412c2502: mov rdx, qword ptr [rsi]
1412c2505: lea rcx, [rip + 0x17dac5f]
1412c250c: mov r8d, 0x49
1412c2512: call 0x142923bf0
1412c2517: test eax, eax
1412c2519: jne 0x1412c257e
1412c251b: lea rax, [rip + 0x196b6c6]
1412c2522: mov qword ptr [rbp - 0x30], rax
1412c2526: lea rax, [rip - 0xd8ec8d]
1412c252d: mov qword ptr [rbp - 0x28], rax
1412c2531: mov rax, rbp
1412c2534: mov qword ptr [rbp - 0x20], rax
1412c2538: lea rax, [rip - 0x11b86af]
1412c253f: mov qword ptr [rbp - 0x18], rax
1412c2543: lea rax, [rip + 0x196b0de]
1412c254a: mov qword ptr [rbp + 0x10], rax
1412c254e: mov qword ptr [rbp + 0x18], 3
1412c2556: mov qword ptr [rbp + 0x30], 0
1412c255e: lea rax, [rbp - 0x30]
1412c2562: mov qword ptr [rbp + 0x20], rax
1412c2566: mov qword ptr [rbp + 0x28], 2
1412c256e: lea rdx, [rip + 0x196b0e3]
1412c2575: lea rcx, [rbp + 0x10]
1412c2579: call 0x1428d9390
1412c257e: lea rax, [rip + 0x196b663]
1412c2585: mov qword ptr [rbp + 0x10], rax
1412c2589: lea rax, [rip - 0xd8ecf0]
1412c2590: mov qword ptr [rbp + 0x18], rax
1412c2594: mov qword ptr [rbp + 0x20], rsi
1412c2598: lea rax, [rip - 0x11b870f]
1412c259f: mov qword ptr [rbp + 0x28], rax
1412c25a3: mov rcx, rbp
1412c25a6: mov qword ptr [rbp + 0x30], rcx
1412c25aa: mov qword ptr [rbp + 0x38], rax
1412c25ae: lea rax, [rip + 0x196afb3]
1412c25b5: mov qword ptr [rbp - 0x30], rax
1412c25b9: mov qword ptr [rbp - 0x28], 3
1412c25c1: mov qword ptr [rbp - 0x10], 0
1412c25c9: lea rax, [rbp + 0x10]
1412c25cd: mov qword ptr [rbp - 0x20], rax
1412c25d1: mov qword ptr [rbp - 0x18], 3
1412c25d9: lea rdx, [rip + 0x196aff0]
1412c25e0: lea rcx, [rbp - 0x30]
1412c25e4: call 0x1428d9390
1412c25e9: int3 
1412c25ea: nop word ptr [rax + rax]
1412c25f0: mov qword ptr [rsp + 0x10], rdx
1412c25f5: push rbp
1412c25f6: push rsi
1412c25f7: sub rsp, 0x28
1412c25fb: lea rbp, [rdx + 0x80]
1412c2602: lea rcx, [rbp - 0x58]
1412c2606: call 0x140be24b0
1412c260b: nop 
1412c260c: add rsp, 0x28
1412c2610: pop rsi
1412c2611: pop rbp
1412c2612: ret 
1412c2613: nop word ptr cs:[rax + rax]
1412c2620: mov qword ptr [rsp + 0x10], rdx
1412c2625: push rbp
1412c2626: push rsi
1412c2627: sub rsp, 0x28
1412c262b: lea rbp, [rdx + 0x80]
1412c2632: lea rcx, [rbp + 0x10]
1412c2636: call 0x1400c04b0
1412c263b: nop 
1412c263c: add rsp, 0x28
1412c2640: pop rsi
1412c2641: pop rbp
1412c2642: ret 
