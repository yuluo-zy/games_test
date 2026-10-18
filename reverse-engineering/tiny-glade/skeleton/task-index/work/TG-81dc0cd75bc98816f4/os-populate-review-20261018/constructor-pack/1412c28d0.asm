1412c28d0: push rbp
1412c28d1: push rsi
1412c28d2: sub rsp, 0xd8
1412c28d9: lea rbp, [rsp + 0x80]
1412c28e1: mov qword ptr [rbp + 0x50], 0xfffffffffffffffe
1412c28e9: mov rsi, rcx
1412c28ec: lea rcx, [rbp - 0x58]
1412c28f0: xor r8d, r8d
1412c28f3: call 0x14161a3c0
1412c28f8: mov qword ptr [rbp - 0x60], 0
1412c2900: movzx eax, byte ptr [rip + 0x2372302]
1412c2907: mov ecx, 0x30
1412c290c: mov edx, 8
1412c2911: call 0x140613c10
1412c2916: test rax, rax
1412c2919: je 0x1412c29cf
1412c291f: movups xmm0, xmmword ptr [rbp - 0x60]
1412c2923: movups xmm1, xmmword ptr [rbp - 0x50]
1412c2927: movups xmm2, xmmword ptr [rbp - 0x40]
1412c292b: movups xmmword ptr [rax + 0x20], xmm2
1412c292f: movups xmmword ptr [rax + 0x10], xmm1
1412c2933: movups xmmword ptr [rax], xmm0
1412c2936: lea rcx, [rip + 0x17dbd67]
1412c293d: mov qword ptr [rbp], rcx
1412c2941: mov qword ptr [rbp + 8], 0x44
1412c2949: mov qword ptr [rbp + 0x10], rax
1412c294d: lea rax, [rip + 0x196a524]
1412c2954: mov qword ptr [rbp + 0x18], rax
1412c2958: mov qword ptr [rbp + 0x30], 0
1412c2960: mov byte ptr [rbp + 0x46], 5
1412c2964: mov qword ptr [rbp + 0x20], rcx
1412c2968: mov qword ptr [rbp + 0x28], 0x44
1412c2970: movzx eax, byte ptr [rip + 0x2372292]
1412c2977: mov ecx, 0x40
1412c297c: mov edx, 8
1412c2981: call 0x140613c10
1412c2986: test rax, rax
1412c2989: je 0x1412c29e0
1412c298b: movups xmm0, xmmword ptr [rbp + 0x10]
1412c298f: movups xmm1, xmmword ptr [rbp + 0x20]
1412c2993: movups xmm2, xmmword ptr [rbp + 0x30]
1412c2997: movups xmm3, xmmword ptr [rbp + 0x40]
1412c299b: movups xmmword ptr [rax + 0x30], xmm3
1412c299f: movups xmmword ptr [rax + 0x20], xmm2
1412c29a3: movups xmmword ptr [rax + 0x10], xmm1
1412c29a7: movups xmmword ptr [rax], xmm0
1412c29aa: movabs rdx, 0x2002d2fbdff3d80f
1412c29b4: mov rcx, rsi
1412c29b7: mov r8, rax
1412c29ba: call 0x140969bf0
1412c29bf: cmp rax, 1
1412c29c3: je 0x1412c29f1
1412c29c5: add rsp, 0xd8
1412c29cc: pop rsi
1412c29cd: pop rbp
1412c29ce: ret 
1412c29cf: mov ecx, 8
1412c29d4: mov edx, 0x30
1412c29d9: call 0x1428d9000
1412c29de: jmp 0x1412c29ef
1412c29e0: mov ecx, 8
1412c29e5: mov edx, 0x40
1412c29ea: call 0x1428d9000
1412c29ef: ud2 
1412c29f1: mov rax, rdx
1412c29f4: add rdx, 0x10
1412c29f8: mov rsi, rdx
1412c29fb: cmp qword ptr [rax + 0x18], 0x44
1412c2a00: jne 0x1412c2a7e
1412c2a02: mov rdx, qword ptr [rsi]
1412c2a05: lea rcx, [rip + 0x17dbc98]
1412c2a0c: mov r8d, 0x44
1412c2a12: call 0x142923bf0
1412c2a17: test eax, eax
1412c2a19: jne 0x1412c2a7e
1412c2a1b: lea rax, [rip + 0x196b316]
1412c2a22: mov qword ptr [rbp - 0x30], rax
1412c2a26: lea rax, [rip - 0xd8f18d]
1412c2a2d: mov qword ptr [rbp - 0x28], rax
1412c2a31: mov rax, rbp
1412c2a34: mov qword ptr [rbp - 0x20], rax
1412c2a38: lea rax, [rip - 0x11b8baf]
1412c2a3f: mov qword ptr [rbp - 0x18], rax
1412c2a43: lea rax, [rip + 0x196abde]
1412c2a4a: mov qword ptr [rbp + 0x10], rax
1412c2a4e: mov qword ptr [rbp + 0x18], 3
1412c2a56: mov qword ptr [rbp + 0x30], 0
1412c2a5e: lea rax, [rbp - 0x30]
1412c2a62: mov qword ptr [rbp + 0x20], rax
1412c2a66: mov qword ptr [rbp + 0x28], 2
1412c2a6e: lea rdx, [rip + 0x196abe3]
1412c2a75: lea rcx, [rbp + 0x10]
1412c2a79: call 0x1428d9390
1412c2a7e: lea rax, [rip + 0x196b2b3]
1412c2a85: mov qword ptr [rbp + 0x10], rax
1412c2a89: lea rax, [rip - 0xd8f1f0]
1412c2a90: mov qword ptr [rbp + 0x18], rax
1412c2a94: mov qword ptr [rbp + 0x20], rsi
1412c2a98: lea rax, [rip - 0x11b8c0f]
1412c2a9f: mov qword ptr [rbp + 0x28], rax
1412c2aa3: mov rcx, rbp
1412c2aa6: mov qword ptr [rbp + 0x30], rcx
1412c2aaa: mov qword ptr [rbp + 0x38], rax
1412c2aae: lea rax, [rip + 0x196aab3]
1412c2ab5: mov qword ptr [rbp - 0x30], rax
1412c2ab9: mov qword ptr [rbp - 0x28], 3
1412c2ac1: mov qword ptr [rbp - 0x10], 0
1412c2ac9: lea rax, [rbp + 0x10]
1412c2acd: mov qword ptr [rbp - 0x20], rax
1412c2ad1: mov qword ptr [rbp - 0x18], 3
1412c2ad9: lea rdx, [rip + 0x196aaf0]
1412c2ae0: lea rcx, [rbp - 0x30]
1412c2ae4: call 0x1428d9390
1412c2ae9: int3 
1412c2aea: nop word ptr [rax + rax]
1412c2af0: mov qword ptr [rsp + 0x10], rdx
1412c2af5: push rbp
1412c2af6: push rsi
1412c2af7: sub rsp, 0x28
1412c2afb: lea rbp, [rdx + 0x80]
1412c2b02: lea rcx, [rbp - 0x58]
1412c2b06: call 0x140be24b0
1412c2b0b: nop 
1412c2b0c: add rsp, 0x28
1412c2b10: pop rsi
1412c2b11: pop rbp
1412c2b12: ret 
1412c2b13: nop word ptr cs:[rax + rax]
1412c2b20: mov qword ptr [rsp + 0x10], rdx
1412c2b25: push rbp
1412c2b26: push rsi
1412c2b27: sub rsp, 0x28
1412c2b2b: lea rbp, [rdx + 0x80]
1412c2b32: lea rcx, [rbp + 0x10]
1412c2b36: call 0x1400c04b0
1412c2b3b: nop 
1412c2b3c: add rsp, 0x28
1412c2b40: pop rsi
1412c2b41: pop rbp
1412c2b42: ret 
