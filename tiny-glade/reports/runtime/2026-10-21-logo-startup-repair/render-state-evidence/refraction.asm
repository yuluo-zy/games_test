0x1409bf320: push rbp
0x1409bf321: push r15
0x1409bf323: push r14
0x1409bf325: push r13
0x1409bf327: push r12
0x1409bf329: push rsi
0x1409bf32a: push rdi
0x1409bf32b: push rbx
0x1409bf32c: sub rsp, 0x9f8
0x1409bf333: lea rbp, [rsp + 0x80]
0x1409bf33b: mov qword ptr [rbp + 0x970], 0xfffffffffffffffe
0x1409bf346: mov qword ptr [rbp + 0x950], r9
0x1409bf34d: mov rsi, r8
0x1409bf350: mov r14, rdx
0x1409bf353: mov rdi, rcx
0x1409bf356: mov rbx, qword ptr [rbp + 0x9e0]
0x1409bf35d: xorps xmm0, xmm0
0x1409bf360: movups xmmword ptr [rbp + 0x3b4], xmm0
0x1409bf367: lea rcx, [rbx + 4]
0x1409bf36b: call 0x140bd5790
0x1409bf370: mov dword ptr [rbp + 0x3b0], 1
0x1409bf37a: mov qword ptr [rbp + 0x3c8], rsi
0x1409bf381: mov qword ptr [rbp + 0x798], 1
0x1409bf38c: mov qword ptr [rbp + 0x7a0], rbx
0x1409bf393: mov qword ptr [rbp + 0x7a8], 0
0x1409bf39e: mov qword ptr [rsp + 0x40], rdi
0x1409bf3a3: mov qword ptr [rsp + 0x38], r14
0x1409bf3a8: mov qword ptr [rsp + 0x30], rax
0x1409bf3ad: lea rax, [rbp + 0x798]
0x1409bf3b4: mov qword ptr [rsp + 0x28], rax
0x1409bf3b9: mov qword ptr [rsp + 0x20], 1
0x1409bf3c2: lea rdx, [rip + 0x2100812]
0x1409bf3c9: lea rcx, [rbp - 0x38]
0x1409bf3cd: lea rbx, [rbp + 0x3b0]
0x1409bf3d4: mov r8d, 0x16
0x1409bf3da: mov r9, rbx
0x1409bf3dd: call 0x140867970
0x1409bf3e2: mov qword ptr [rbp + 0x960], r14
0x1409bf3e9: lea rcx, [r14 + 0x38]
0x1409bf3ed: mov byte ptr [rbp + 0x96f], 1
0x1409bf3f4: movabs rdx, 0x4bc362195b80f729
0x1409bf3fe: movabs r8, 0x982798ce80cbcefb
0x1409bf408: call 0x1400dd6f0
0x1409bf40d: test rax, rax
0x1409bf410: mov rcx, qword ptr [rbp + 0x960]
0x1409bf417: je 0x1409bf746
0x1409bf41d: cmp qword ptr [rcx + 0x218], rdx
0x1409bf424: jbe 0x1409bf746
0x1409bf42a: mov rax, qword ptr [rcx + 0x210]
0x1409bf431: mov rax, qword ptr [rax + rdx*8]
0x1409bf435: test rax, rax
0x1409bf438: je 0x1409bf746
0x1409bf43e: mov rcx, qword ptr [rcx + 0x1e0]
0x1409bf445: not rax
0x1409bf448: lea rdx, [rax + rax*2]
0x1409bf44c: shl rdx, 5
0x1409bf450: cmp qword ptr [rcx + rdx + 0x20], 0
0x1409bf456: je 0x1409bf746
0x1409bf45c: mov rax, qword ptr [rbp + 0x9e8]
0x1409bf463: add rcx, rdx
0x1409bf466: mov rcx, qword ptr [rcx + 0x10]
0x1409bf46a: mov rdx, qword ptr [rcx]
0x1409bf46d: test rdx, rdx
0x1409bf470: js 0x1409bf47c
0x1409bf472: xorps xmm0, xmm0
0x1409bf475: cvtsi2ss xmm0, rdx
0x1409bf47a: jmp 0x1409bf494
0x1409bf47c: mov r8, rdx
0x1409bf47f: shr r8, 1
0x1409bf482: and edx, 1
0x1409bf485: or rdx, r8
0x1409bf488: xorps xmm0, xmm0
0x1409bf48b: cvtsi2ss xmm0, rdx
0x1409bf490: addss xmm0, xmm0
0x1409bf494: cvtsi2ss xmm1, dword ptr [rcx + 8]
0x1409bf499: divss xmm1, dword ptr [rip + 0x1fe819f]
0x1409bf4a1: addss xmm1, xmm0
0x1409bf4a5: movss dword ptr [rbp + 0x958], xmm1
0x1409bf4ad: mov byte ptr [rbp + 0x95c], 0
0x1409bf4b4: mov r14, qword ptr [rax + 0x158]
0x1409bf4bb: mov rax, qword ptr [rax + 0x160]
0x1409bf4c2: shl rax, 7
0x1409bf4c6: add rax, r14
0x1409bf4c9: mov qword ptr [rbp + 0x948], rax
0x1409bf4d0: lea r15, [rbp + 0x7a0]
0x1409bf4d7: lea r12, [rbp + 0x3b8]
0x1409bf4de: cmp r14, qword ptr [rbp + 0x948]
0x1409bf4e5: je 0x1409bf6f4
0x1409bf4eb: nop dword ptr [rax + rax]
0x1409bf4f0: mov r13, qword ptr [r14 + 0x50]
0x1409bf4f4: mov rsi, qword ptr [r14 + 0x58]
0x1409bf4f8: mov byte ptr [rbp + 0x96f], 1
0x1409bf4ff: call 0x140be6750
0x1409bf504: mov edi, eax
0x1409bf506: mov byte ptr [rbp + 0x96f], 1
0x1409bf50d: xor ecx, ecx
0x1409bf50f: call 0x1400d9b20
0x1409bf514: test rax, rax
0x1409bf517: je 0x1409bf731
0x1409bf51d: mov dword ptr [rbp + 0x938], edi
0x1409bf523: mov word ptr [rbp + 0x940], 0
0x1409bf52c: mov dword ptr [rbp + 0x93c], 0x3f800000
0x1409bf536: mov qword ptr [rbp + 0x890], 1
0x1409bf541: mov qword ptr [rbp + 0x898], r13
0x1409bf548: mov qword ptr [rbp + 0x930], rsi
0x1409bf54f: mov qword ptr [rbp + 0x8a0], 0
0x1409bf55a: mov qword ptr [rbp + 0x8a8], 4
0x1409bf565: mov qword ptr [rbp + 0x8b0], 0
0x1409bf570: movups xmm0, xmmword ptr [rax]
0x1409bf573: inc qword ptr [rax]
0x1409bf576: movups xmm1, xmmword ptr [rip + 0x20d71b3]
0x1409bf57d: lea rax, [rbp + 0x900]
0x1409bf584: movups xmmword ptr [rax + 0x10], xmm1
0x1409bf588: movups xmm1, xmmword ptr [rip + 0x20d7191]
0x1409bf58f: movups xmmword ptr [rax], xmm1
0x1409bf592: movups xmmword ptr [rbp + 0x920], xmm0
0x1409bf599: movabs rax, 0x100000003
0x1409bf5a3: mov qword ptr [rbp + 0x8d0], rax
0x1409bf5aa: mov qword ptr [rbp + 0x8b8], 0
0x1409bf5b5: mov qword ptr [rbp + 0x8c0], 1
0x1409bf5c0: mov qword ptr [rbp + 0x8c8], 0
0x1409bf5cb: mov byte ptr [rbp + 0x96f], 1
0x1409bf5d2: mov rcx, rbx
0x1409bf5d5: lea rdx, [rbp + 0x890]
0x1409bf5dc: lea r8, [rbp - 0x38]
0x1409bf5e0: xor r9d, r9d
0x1409bf5e3: call 0x140905be0
0x1409bf5e8: mov byte ptr [rbp + 0x96f], 1
0x1409bf5ef: lea rcx, [rbp + 0x798]
0x1409bf5f6: mov rdx, rbx
0x1409bf5f9: lea r8, [rbp + 0x958]
0x1409bf600: mov r9, r14
0x1409bf603: call 0x14090b460
0x1409bf608: mov rax, qword ptr [rbp + 0x950]
0x1409bf60f: mov qword ptr [rbp + 0x890], rax
0x1409bf616: mov dword ptr [rbp + 0x3b0], 2
0x1409bf620: mov qword ptr [rsp + 0x20], rbx
0x1409bf625: lea rcx, [rbp + 0x890]
0x1409bf62c: lea rdx, [rbp + 0x798]
0x1409bf633: movabs r8, 0xd48bea13951c1ddc
0x1409bf63d: mov r9b, 0x1d
0x1409bf640: call 0x1408ee800
0x1409bf645: mov rsi, qword ptr [rbp + 0x798]
0x1409bf64c: mov r8d, 0xa0
0x1409bf652: mov rcx, r12
0x1409bf655: mov rdx, r15
0x1409bf658: call 0x142923c00
0x1409bf65d: movzx eax, byte ptr [rbp + 0x840]
0x1409bf664: movups xmm0, xmmword ptr [r15 + 0xa1]
0x1409bf66c: movups xmm1, xmmword ptr [r15 + 0xb1]
0x1409bf674: movups xmm2, xmmword ptr [r15 + 0xc1]
0x1409bf67c: movups xmm3, xmmword ptr [r15 + 0xd1]
0x1409bf684: movups xmmword ptr [r12 + 0xa1], xmm0
0x1409bf68d: movups xmmword ptr [r12 + 0xb1], xmm1
0x1409bf696: movups xmmword ptr [r12 + 0xc1], xmm2
0x1409bf69f: movups xmmword ptr [r12 + 0xd1], xmm3
0x1409bf6a8: movups xmm0, xmmword ptr [r15 + 0xe0]
0x1409bf6b0: movups xmmword ptr [r12 + 0xe0], xmm0
0x1409bf6b9: and al, 0xbf
0x1409bf6bb: mov r9, qword ptr [r14 + 0x70]
0x1409bf6bf: mov qword ptr [rbp + 0x3b0], rsi
0x1409bf6c6: mov byte ptr [rbp + 0x458], al
0x1409bf6cc: mov byte ptr [rbp + 0x96f], 1
0x1409bf6d3: mov rcx, rbx
0x1409bf6d6: mov rdx, r14
0x1409bf6d9: mov r8, qword ptr [rbp + 0x960]
0x1409bf6e0: call r9
0x1409bf6e3: sub r14, -0x80
0x1409bf6e7: cmp r14, qword ptr [rbp + 0x948]
0x1409bf6ee: jne 0x1409bf4f0
0x1409bf6f4: lea rsi, [rbp + 0x3b0]
0x1409bf6fb: lea rdx, [rbp - 0x38]
0x1409bf6ff: mov r8d, 0x3e8
0x1409bf705: mov rcx, rsi
0x1409bf708: call 0x142923c00
0x1409bf70d: mov byte ptr [rbp + 0x96f], 0
0x1409bf714: mov rcx, rsi
0x1409bf717: call 0x140868370
0x1409bf71c: nop 
0x1409bf71d: add rsp, 0x9f8
0x1409bf724: pop rbx
0x1409bf725: pop rdi
0x1409bf726: pop rsi
0x1409bf727: pop r12
0x1409bf729: pop r13
0x1409bf72b: pop r14
0x1409bf72d: pop r15
0x1409bf72f: pop rbp
0x1409bf730: ret 
0x1409bf731: mov byte ptr [rbp + 0x96f], 1
0x1409bf738: lea rcx, [rip + 0x227b361]
0x1409bf73f: call 0x1428d80d0
0x1409bf744: jmp 0x1409bf759
0x1409bf746: mov byte ptr [rbp + 0x96f], 1
0x1409bf74d: lea rcx, [rip + 0x20e6484]
0x1409bf754: call 0x1428d9310
0x1409bf759: ud2 
0x1409bf75b: nop dword ptr [rax + rax]
0x1409bf760: mov qword ptr [rsp + 0x10], rdx
0x1409bf765: push rbp
0x1409bf766: push r15
0x1409bf768: push r14
0x1409bf76a: push r13
0x1409bf76c: push r12
0x1409bf76e: push rsi
0x1409bf76f: push rdi
0x1409bf770: push rbx
0x1409bf771: sub rsp, 0x48
0x1409bf775: lea rbp, [rdx + 0x80]
0x1409bf77c: cmp byte ptr [rbp + 0x96f], 0
0x1409bf783: jne 0x1409bf796
0x1409bf785: add rsp, 0x48
0x1409bf789: pop rbx
0x1409bf78a: pop rdi
0x1409bf78b: pop rsi
0x1409bf78c: pop r12
0x1409bf78e: pop r13
0x1409bf790: pop r14
0x1409bf792: pop r15
0x1409bf794: pop rbp
0x1409bf795: ret 
0x1409bf796: lea rcx, [rbp - 0x38]
0x1409bf79a: call 0x1408923c0
0x1409bf79f: jmp 0x1409bf785
0x1409bf7a1: nop word ptr cs:[rax + rax]
0x1409bf7b0: mov qword ptr [rsp + 0x10], rdx
0x1409bf7b5: push rbp
0x1409bf7b6: push r15
0x1409bf7b8: push r14
0x1409bf7ba: push r13
0x1409bf7bc: push r12
0x1409bf7be: push rsi
0x1409bf7bf: push rdi
0x1409bf7c0: push rbx
0x1409bf7c1: sub rsp, 0x48
0x1409bf7c5: lea rbp, [rdx + 0x80]
0x1409bf7cc: mov byte ptr [rbp + 0x96f], 1
0x1409bf7d3: lea rcx, [rbp + 0x798]
0x1409bf7da: call 0x1408935c0
0x1409bf7df: mov byte ptr [rbp + 0x96f], 1
0x1409bf7e6: add rsp, 0x48
0x1409bf7ea: pop rbx
0x1409bf7eb: pop rdi
0x1409bf7ec: pop rsi
0x1409bf7ed: pop r12
0x1409bf7ef: pop r13
0x1409bf7f1: pop r14
0x1409bf7f3: pop r15
0x1409bf7f5: pop rbp
0x1409bf7f6: ret 
