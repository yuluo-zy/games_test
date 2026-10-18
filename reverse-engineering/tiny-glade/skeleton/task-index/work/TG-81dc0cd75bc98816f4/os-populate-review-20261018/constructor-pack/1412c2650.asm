1412c2650: push rbp
1412c2651: push rsi
1412c2652: sub rsp, 0xd8
1412c2659: lea rbp, [rsp + 0x80]
1412c2661: mov qword ptr [rbp + 0x50], 0xfffffffffffffffe
1412c2669: mov rsi, rcx
1412c266c: lea rcx, [rbp - 0x58]
1412c2670: xor r8d, r8d
1412c2673: call 0x14161a590
1412c2678: mov qword ptr [rbp - 0x60], 0
1412c2680: movzx eax, byte ptr [rip + 0x2372582]
1412c2687: mov ecx, 0x30
1412c268c: mov edx, 8
1412c2691: call 0x140613c10
1412c2696: test rax, rax
1412c2699: je 0x1412c274f
1412c269f: movups xmm0, xmmword ptr [rbp - 0x60]
1412c26a3: movups xmm1, xmmword ptr [rbp - 0x50]
1412c26a7: movups xmm2, xmmword ptr [rbp - 0x40]
1412c26ab: movups xmmword ptr [rax + 0x20], xmm2
1412c26af: movups xmmword ptr [rax + 0x10], xmm1
1412c26b3: movups xmmword ptr [rax], xmm0
1412c26b6: lea rcx, [rip + 0x17db351]
1412c26bd: mov qword ptr [rbp], rcx
1412c26c1: mov qword ptr [rbp + 8], 0x41
1412c26c9: mov qword ptr [rbp + 0x10], rax
1412c26cd: lea rax, [rip + 0x196a76c]
1412c26d4: mov qword ptr [rbp + 0x18], rax
1412c26d8: mov qword ptr [rbp + 0x30], 0
1412c26e0: mov byte ptr [rbp + 0x46], 5
1412c26e4: mov qword ptr [rbp + 0x20], rcx
1412c26e8: mov qword ptr [rbp + 0x28], 0x41
1412c26f0: movzx eax, byte ptr [rip + 0x2372512]
1412c26f7: mov ecx, 0x40
1412c26fc: mov edx, 8
1412c2701: call 0x140613c10
1412c2706: test rax, rax
1412c2709: je 0x1412c2760
1412c270b: movups xmm0, xmmword ptr [rbp + 0x10]
1412c270f: movups xmm1, xmmword ptr [rbp + 0x20]
1412c2713: movups xmm2, xmmword ptr [rbp + 0x30]
1412c2717: movups xmm3, xmmword ptr [rbp + 0x40]
1412c271b: movups xmmword ptr [rax + 0x30], xmm3
1412c271f: movups xmmword ptr [rax + 0x20], xmm2
1412c2723: movups xmmword ptr [rax + 0x10], xmm1
1412c2727: movups xmmword ptr [rax], xmm0
1412c272a: movabs rdx, 0x83773598f5ecfead
1412c2734: mov rcx, rsi
1412c2737: mov r8, rax
1412c273a: call 0x140969bf0
1412c273f: cmp rax, 1
1412c2743: je 0x1412c2771
1412c2745: add rsp, 0xd8
1412c274c: pop rsi
1412c274d: pop rbp
1412c274e: ret 
1412c274f: mov ecx, 8
1412c2754: mov edx, 0x30
1412c2759: call 0x1428d9000
1412c275e: jmp 0x1412c276f
1412c2760: mov ecx, 8
1412c2765: mov edx, 0x40
1412c276a: call 0x1428d9000
1412c276f: ud2 
1412c2771: mov rax, rdx
1412c2774: add rdx, 0x10
1412c2778: mov rsi, rdx
1412c277b: cmp qword ptr [rax + 0x18], 0x41
1412c2780: jne 0x1412c27fe
1412c2782: mov rdx, qword ptr [rsi]
1412c2785: lea rcx, [rip + 0x17db282]
1412c278c: mov r8d, 0x41
1412c2792: call 0x142923bf0
1412c2797: test eax, eax
1412c2799: jne 0x1412c27fe
1412c279b: lea rax, [rip + 0x196b356]
1412c27a2: mov qword ptr [rbp - 0x30], rax
1412c27a6: lea rax, [rip - 0xd8ef0d]
1412c27ad: mov qword ptr [rbp - 0x28], rax
1412c27b1: mov rax, rbp
1412c27b4: mov qword ptr [rbp - 0x20], rax
1412c27b8: lea rax, [rip - 0x11b892f]
1412c27bf: mov qword ptr [rbp - 0x18], rax
1412c27c3: lea rax, [rip + 0x196ae5e]
1412c27ca: mov qword ptr [rbp + 0x10], rax
1412c27ce: mov qword ptr [rbp + 0x18], 3
1412c27d6: mov qword ptr [rbp + 0x30], 0
1412c27de: lea rax, [rbp - 0x30]
1412c27e2: mov qword ptr [rbp + 0x20], rax
1412c27e6: mov qword ptr [rbp + 0x28], 2
1412c27ee: lea rdx, [rip + 0x196ae63]
1412c27f5: lea rcx, [rbp + 0x10]
1412c27f9: call 0x1428d9390
1412c27fe: lea rax, [rip + 0x196b2f3]
1412c2805: mov qword ptr [rbp + 0x10], rax
1412c2809: lea rax, [rip - 0xd8ef70]
1412c2810: mov qword ptr [rbp + 0x18], rax
1412c2814: mov qword ptr [rbp + 0x20], rsi
1412c2818: lea rax, [rip - 0x11b898f]
1412c281f: mov qword ptr [rbp + 0x28], rax
1412c2823: mov rcx, rbp
1412c2826: mov qword ptr [rbp + 0x30], rcx
1412c282a: mov qword ptr [rbp + 0x38], rax
1412c282e: lea rax, [rip + 0x196ad33]
1412c2835: mov qword ptr [rbp - 0x30], rax
1412c2839: mov qword ptr [rbp - 0x28], 3
1412c2841: mov qword ptr [rbp - 0x10], 0
1412c2849: lea rax, [rbp + 0x10]
1412c284d: mov qword ptr [rbp - 0x20], rax
1412c2851: mov qword ptr [rbp - 0x18], 3
1412c2859: lea rdx, [rip + 0x196ad70]
1412c2860: lea rcx, [rbp - 0x30]
1412c2864: call 0x1428d9390
1412c2869: int3 
1412c286a: nop word ptr [rax + rax]
1412c2870: mov qword ptr [rsp + 0x10], rdx
1412c2875: push rbp
1412c2876: push rsi
1412c2877: sub rsp, 0x28
1412c287b: lea rbp, [rdx + 0x80]
1412c2882: lea rcx, [rbp - 0x58]
1412c2886: call 0x140be24b0
1412c288b: nop 
1412c288c: add rsp, 0x28
1412c2890: pop rsi
1412c2891: pop rbp
1412c2892: ret 
1412c2893: nop word ptr cs:[rax + rax]
1412c28a0: mov qword ptr [rsp + 0x10], rdx
1412c28a5: push rbp
1412c28a6: push rsi
1412c28a7: sub rsp, 0x28
1412c28ab: lea rbp, [rdx + 0x80]
1412c28b2: lea rcx, [rbp + 0x10]
1412c28b6: call 0x1400c04b0
1412c28bb: nop 
1412c28bc: add rsp, 0x28
1412c28c0: pop rsi
1412c28c1: pop rbp
1412c28c2: ret 
