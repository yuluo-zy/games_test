1412fc510: push rbp
1412fc511: push r15
1412fc513: push r14
1412fc515: push r13
1412fc517: push r12
1412fc519: push rsi
1412fc51a: push rdi
1412fc51b: push rbx
1412fc51c: sub rsp, 0x98
1412fc523: lea rbp, [rsp + 0x80]
1412fc52b: movdqa xmmword ptr [rbp], xmm8
1412fc531: movaps xmmword ptr [rbp - 0x10], xmm7
1412fc535: movaps xmmword ptr [rbp - 0x20], xmm6
1412fc539: mov qword ptr [rbp - 0x28], 0xfffffffffffffffe
1412fc541: mov rsi, rcx
1412fc544: mov r14, qword ptr [rdx + 0x20]
1412fc548: test r14, r14
1412fc54b: je 0x1412fc649
1412fc551: mov r15, qword ptr [rdx]
1412fc554: mov r12, qword ptr [rdx + 8]
1412fc558: movzx eax, word ptr [rdx + 0x18]
1412fc55c: movss xmm6, dword ptr [rip + 0x163480c]
1412fc564: jmp 0x1412fc57c
1412fc566: nop word ptr cs:[rax + rax]
1412fc570: mov eax, r13d
1412fc573: test r14, r14
1412fc576: je 0x1412fc649
1412fc57c: test ax, ax
1412fc57f: je 0x1412fc5a0
1412fc581: lea r13d, [rax - 1]
1412fc585: and r13d, eax
1412fc588: mov word ptr [rdx + 0x18], r13w
1412fc58d: dec r14
1412fc590: mov qword ptr [rdx + 0x20], r14
1412fc594: test r15, r15
1412fc597: jne 0x1412fc5de
1412fc599: jmp 0x1412fc649
1412fc59e: nop 
1412fc5a0: movdqa xmm0, xmmword ptr [r12]
1412fc5a6: pmovmskb eax, xmm0
1412fc5aa: add r15, -0x2a80
1412fc5b1: add r12, 0x10
1412fc5b5: cmp eax, 0xffff
1412fc5ba: je 0x1412fc5a0
1412fc5bc: mov qword ptr [rdx + 8], r12
1412fc5c0: mov qword ptr [rdx], r15
1412fc5c3: mov ecx, 0xfffffffe
1412fc5c8: sub ecx, eax
1412fc5ca: not eax
1412fc5cc: mov r13d, eax
1412fc5cf: and r13d, ecx
1412fc5d2: mov word ptr [rdx + 0x18], r13w
1412fc5d7: dec r14
1412fc5da: mov qword ptr [rdx + 0x20], r14
1412fc5de: tzcnt eax, eax
1412fc5e2: neg rax
1412fc5e5: imul rax, rax, 0x2a8
1412fc5ec: movss xmm0, dword ptr [r15 + rax - 0x20]
1412fc5f3: ucomiss xmm0, xmm6
1412fc5f6: ja 0x1412fc570
1412fc5fc: add rax, r15
1412fc5ff: mov ecx, dword ptr [rax - 0x48]
1412fc602: cmp ecx, 1
1412fc605: ja 0x1412fc570
1412fc60b: movd xmm8, dword ptr [rax - 0x44]
1412fc611: movd xmm1, dword ptr [rax - 0x40]
1412fc616: movss xmm0, dword ptr [rax - 0x3c]
1412fc61b: test ecx, ecx
1412fc61d: mov qword ptr [rbp - 0x30], r8
1412fc621: je 0x1412fc684
1412fc623: movsd xmm7, qword ptr [rax - 0x34]
1412fc628: mov eax, dword ptr [rax - 0x38]
1412fc62b: mov dword ptr [rbp - 0x38], eax
1412fc62e: movd edi, xmm8
1412fc633: movd eax, xmm1
1412fc637: shl rax, 0x20
1412fc63b: or rdi, rax
1412fc63e: mov ebx, 1
1412fc643: movaps xmm8, xmm0
1412fc647: jmp 0x1412fc691
1412fc649: mov qword ptr [rsi], 0
1412fc650: mov qword ptr [rsi + 8], 4
1412fc658: mov qword ptr [rsi + 0x10], 0
1412fc660: mov rax, rsi
1412fc663: movaps xmm6, xmmword ptr [rbp - 0x20]
1412fc667: movaps xmm7, xmmword ptr [rbp - 0x10]
1412fc66b: movaps xmm8, xmmword ptr [rbp]
1412fc670: add rsp, 0x98
1412fc677: pop rbx
1412fc678: pop rdi
1412fc679: pop rsi
1412fc67a: pop r12
1412fc67c: pop r13
1412fc67e: pop r14
1412fc680: pop r15
1412fc682: pop rbp
1412fc683: ret 
1412fc684: movd dword ptr [rbp - 0x38], xmm1
1412fc689: movsldup xmm7, xmm0
1412fc68d: xor edi, edi
1412fc68f: xor ebx, ebx
1412fc691: movzx eax, byte ptr [rip + 0x2338571]
1412fc698: mov ecx, 0x80
1412fc69d: mov edx, 4
1412fc6a2: call 0x140613c10
1412fc6a7: test rax, rax
1412fc6aa: je 0x1412fc872
1412fc6b0: movss dword ptr [rax], xmm8
1412fc6b5: mov ecx, dword ptr [rbp - 0x38]
1412fc6b8: mov dword ptr [rax + 4], ecx
1412fc6bb: movlps qword ptr [rax + 8], xmm7
1412fc6bf: mov qword ptr [rax + 0x10], rdi
1412fc6c3: mov dword ptr [rax + 0x18], ebx
1412fc6c6: mov dword ptr [rax + 0x1c], 0
1412fc6cd: mov qword ptr [rbp - 0x50], 4
1412fc6d5: mov qword ptr [rbp - 0x48], rax
1412fc6d9: mov qword ptr [rbp - 0x40], 1
1412fc6e1: test r14, r14
1412fc6e4: je 0x1412fc856
1412fc6ea: mov edi, 1
1412fc6ef: lea rcx, [rbp - 0x50]
1412fc6f3: mov r8d, r13d
1412fc6f6: jmp 0x1412fc70c
1412fc6f8: nop dword ptr [rax + rax]
1412fc700: mov r8d, r13d
1412fc703: test r14, r14
1412fc706: je 0x1412fc856
1412fc70c: test r8w, r8w
1412fc710: je 0x1412fc720
1412fc712: lea r13d, [r8 - 1]
1412fc716: and r13d, r8d
1412fc719: jmp 0x1412fc750
1412fc71b: nop dword ptr [rax + rax]
1412fc720: movdqa xmm0, xmmword ptr [r12]
1412fc726: pmovmskb r8d, xmm0
1412fc72b: add r15, -0x2a80
1412fc732: add r12, 0x10
1412fc736: cmp r8d, 0xffff
1412fc73d: je 0x1412fc720
1412fc73f: mov edx, 0xfffffffe
1412fc744: sub edx, r8d
1412fc747: not r8d
1412fc74a: mov r13d, r8d
1412fc74d: and r13d, edx
1412fc750: dec r14
1412fc753: tzcnt edx, r8d
1412fc758: neg rdx
1412fc75b: imul r8, rdx, 0x2a8
1412fc762: movss xmm0, dword ptr [r15 + r8 - 0x20]
1412fc769: ucomiss xmm0, xmm6
1412fc76c: ja 0x1412fc700
1412fc76e: add r8, r15
1412fc771: mov edx, dword ptr [r8 - 0x48]
1412fc775: cmp edx, 1
1412fc778: ja 0x1412fc700
1412fc77a: movd xmm8, dword ptr [r8 - 0x44]
1412fc780: movd xmm1, dword ptr [r8 - 0x40]
1412fc786: movss xmm0, dword ptr [r8 - 0x3c]
1412fc78c: movd r9d, xmm1
1412fc791: test edx, edx
1412fc793: je 0x1412fc7cc
1412fc795: movsd xmm7, qword ptr [r8 - 0x34]
1412fc79b: mov edx, dword ptr [r8 - 0x38]
1412fc79f: movd r8d, xmm8
1412fc7a4: shl r9, 0x20
1412fc7a8: or r8, r9
1412fc7ab: mov qword ptr [rbp - 0x30], r8
1412fc7af: mov r8d, 1
1412fc7b5: mov qword ptr [rbp - 0x58], r8
1412fc7b9: mov r9d, edx
1412fc7bc: movaps xmm8, xmm0
1412fc7c0: mov qword ptr [rbp - 0x38], r9
1412fc7c4: cmp rdi, qword ptr [rbp - 0x50]
1412fc7c8: je 0x1412fc7ea
1412fc7ca: jmp 0x1412fc811
1412fc7cc: movsldup xmm7, xmm0
1412fc7d0: mov qword ptr [rbp - 0x58], 0
1412fc7d8: mov qword ptr [rbp - 0x30], 0
1412fc7e0: mov qword ptr [rbp - 0x38], r9
1412fc7e4: cmp rdi, qword ptr [rbp - 0x50]
1412fc7e8: jne 0x1412fc811
1412fc7ea: mov qword ptr [rsp + 0x20], 0x20
1412fc7f3: mov r8d, 1
1412fc7f9: mov r9d, 4
1412fc7ff: mov rbx, rcx
1412fc802: mov rdx, rdi
1412fc805: call 0x1428a9a60 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
1412fc80a: mov rax, qword ptr [rbp - 0x48]
1412fc80e: mov rcx, rbx
1412fc811: mov rdx, rdi
1412fc814: shl rdx, 5
1412fc818: movd dword ptr [rax + rdx], xmm8
1412fc81e: mov r8, qword ptr [rbp - 0x38]
1412fc822: mov dword ptr [rax + rdx + 4], r8d
1412fc827: movlps qword ptr [rax + rdx + 8], xmm7
1412fc82c: mov r8, qword ptr [rbp - 0x30]
1412fc830: mov qword ptr [rax + rdx + 0x10], r8
1412fc835: mov r8, qword ptr [rbp - 0x58]
1412fc839: mov dword ptr [rax + rdx + 0x18], r8d
1412fc83e: mov dword ptr [rax + rdx + 0x1c], 0
1412fc846: inc rdi
1412fc849: mov qword ptr [rbp - 0x40], rdi
1412fc84d: test r14, r14
1412fc850: jne 0x1412fc6f3
1412fc856: mov rax, qword ptr [rbp - 0x40]
1412fc85a: mov qword ptr [rsi + 0x10], rax
1412fc85e: mov rax, qword ptr [rbp - 0x50]
1412fc862: mov qword ptr [rsi], rax
1412fc865: mov rax, qword ptr [rbp - 0x48]
1412fc869: mov qword ptr [rsi + 8], rax
1412fc86d: jmp 0x1412fc660
1412fc872: mov ecx, 4
1412fc877: mov edx, 0x80
1412fc87c: mov r8, qword ptr [rbp - 0x30]
1412fc880: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
1412fc885: int3 
1412fc886: nop word ptr cs:[rax + rax]
1412fc890: mov qword ptr [rsp + 0x10], rdx
1412fc895: push rbp
1412fc896: push r15
1412fc898: push r14
1412fc89a: push r13
1412fc89c: push r12
1412fc89e: push rsi
1412fc89f: push rdi
1412fc8a0: push rbx
1412fc8a1: sub rsp, 0x58
1412fc8a5: lea rbp, [rdx + 0x80]
1412fc8ac: movdqa xmmword ptr [rsp + 0x20], xmm8
1412fc8b3: movaps xmmword ptr [rsp + 0x30], xmm7
1412fc8b8: movaps xmmword ptr [rsp + 0x40], xmm6
1412fc8bd: mov rdx, qword ptr [rbp - 0x50]
1412fc8c1: test rdx, rdx
1412fc8c4: je 0x1412fc8d9
1412fc8c6: mov rcx, qword ptr [rbp - 0x48]
1412fc8ca: shl rdx, 5
1412fc8ce: mov r8d, 4
1412fc8d4: call 0x140613c20
1412fc8d9: movaps xmm6, xmmword ptr [rsp + 0x40]
1412fc8de: movaps xmm7, xmmword ptr [rsp + 0x30]
1412fc8e3: movaps xmm8, xmmword ptr [rsp + 0x20]
1412fc8e9: add rsp, 0x58
1412fc8ed: pop rbx
1412fc8ee: pop rdi
1412fc8ef: pop rsi
1412fc8f0: pop r12
1412fc8f2: pop r13
1412fc8f4: pop r14
1412fc8f6: pop r15
1412fc8f8: pop rbp
1412fc8f9: ret 
