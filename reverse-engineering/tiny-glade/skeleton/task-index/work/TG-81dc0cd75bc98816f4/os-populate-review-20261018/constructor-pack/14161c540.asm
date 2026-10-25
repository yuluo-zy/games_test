14161c540: push rbp
14161c541: push r15
14161c543: push r14
14161c545: push r12
14161c547: push rsi
14161c548: push rdi
14161c549: push rbx
14161c54a: sub rsp, 0x130
14161c551: lea rbp, [rsp + 0x80]
14161c559: mov qword ptr [rbp + 0xa8], 0xfffffffffffffffe
14161c564: mov rdi, rdx
14161c567: mov rsi, rcx
14161c56a: test r8, r8
14161c56d: je 0x14161c5a9
14161c56f: mov rbx, r9
14161c572: mov rax, r9
14161c575: and rax, 3
14161c579: mov qword ptr [rbp - 0x40], rax
14161c57d: jne 0x14161c7ce
14161c583: mov r15, r8
14161c586: mov rax, r8
14161c589: and rax, 3
14161c58d: mov qword ptr [rbp - 0x40], rax
14161c591: jne 0x14161c7df
14161c597: cmp rbx, 0x80004
14161c59e: jae 0x14161c87a
14161c5a4: xor r12d, r12d
14161c5a7: jmp 0x14161c623
14161c5a9: movzx eax, byte ptr [rip + 0x2018659]
14161c5b0: mov ecx, 0x80000
14161c5b5: mov edx, 4
14161c5ba: call 0x140613c10
14161c5bf: test rax, rax
14161c5c2: je 0x14161c8e4
14161c5c8: mov r15, rax
14161c5cb: mov eax, 0x1c
14161c5d0: xorps xmm0, xmm0
14161c5d3: nop word ptr cs:[rax + rax]
14161c5e0: movups xmmword ptr [r15 + rax*4 - 0x70], xmm0
14161c5e6: movups xmmword ptr [r15 + rax*4 - 0x60], xmm0
14161c5ec: movups xmmword ptr [r15 + rax*4 - 0x50], xmm0
14161c5f2: movups xmmword ptr [r15 + rax*4 - 0x40], xmm0
14161c5f8: movups xmmword ptr [r15 + rax*4 - 0x30], xmm0
14161c5fe: movups xmmword ptr [r15 + rax*4 - 0x20], xmm0
14161c604: movups xmmword ptr [r15 + rax*4 - 0x10], xmm0
14161c60a: movups xmmword ptr [r15 + rax*4], xmm0
14161c60f: add rax, 0x20
14161c613: cmp rax, 0x2001c
14161c619: jne 0x14161c5e0
14161c61b: mov r12b, 1
14161c61e: mov ebx, 0x80000
14161c623: mov qword ptr [rbp + 0x98], r15
14161c62a: mov byte ptr [rbp + 0xa7], r12b
14161c631: lea rcx, [rbp + 0x10]
14161c635: mov edx, 0x80000
14161c63a: mov r8d, 0x22
14161c640: mov r9b, 4
14161c643: call 0x140bd4de0
14161c648: mov qword ptr [rbp + 0x98], r15
14161c64f: mov byte ptr [rbp + 0xa7], r12b
14161c656: lea r14, [rbp + 0x60]
14161c65a: lea rdx, [rbp + 0x10]
14161c65e: mov r8d, 4
14161c664: mov rcx, r14
14161c667: call 0x140bd4f20
14161c66c: mov qword ptr [rbp + 0x98], r15
14161c673: mov byte ptr [rbp + 0xa7], r12b
14161c67a: mov qword ptr [rsp + 0x30], rbx
14161c67f: mov qword ptr [rsp + 0x28], r15
14161c684: mov qword ptr [rsp + 0x20], r14
14161c689: lea r8, [rip + 0x16634d2]
14161c690: lea rcx, [rbp - 0x40]
14161c694: mov r9d, 0x13
14161c69a: mov rdx, rdi
14161c69d: call 0x1416a2dc0
14161c6a2: cmp dword ptr [rbp - 0x40], 0xb
14161c6a6: jne 0x14161c809
14161c6ac: mov rax, qword ptr [rbp - 0x18]
14161c6b0: mov qword ptr [rbp + 0x80], rax
14161c6b7: movups xmm0, xmmword ptr [rbp - 0x38]
14161c6bb: movups xmm1, xmmword ptr [rbp - 0x28]
14161c6bf: movaps xmmword ptr [rbp + 0x70], xmm1
14161c6c3: movaps xmmword ptr [rbp + 0x60], xmm0
14161c6c7: test r12b, r12b
14161c6ca: je 0x14161c6df
14161c6cc: mov edx, 0x80000
14161c6d1: mov r8d, 4
14161c6d7: mov rcx, r15
14161c6da: call 0x140613c20
14161c6df: lea rcx, [rbp + 0x60]
14161c6e3: mov rdx, rdi
14161c6e6: call 0x140bd4cd0
14161c6eb: test rax, rax
14161c6ee: je 0x14161c869
14161c6f4: cmp rdx, qword ptr [rbp + 0x78]
14161c6f8: jb 0x14161c753
14161c6fa: mov rcx, rdx
14161c6fd: and rcx, 3
14161c701: mov qword ptr [rbp - 0x40], rcx
14161c705: jne 0x14161c770
14161c707: mov rcx, rax
14161c70a: and rcx, 3
14161c70e: mov qword ptr [rbp - 0x40], rcx
14161c712: jne 0x14161c79f
14161c718: mov rcx, qword ptr [rbp + 0x80]
14161c71f: mov qword ptr [rsi + 0x30], rcx
14161c723: movaps xmm0, xmmword ptr [rbp + 0x60]
14161c727: movaps xmm1, xmmword ptr [rbp + 0x70]
14161c72b: movups xmmword ptr [rsi + 0x20], xmm1
14161c72f: movups xmmword ptr [rsi + 0x10], xmm0
14161c733: shr rdx, 2
14161c737: mov qword ptr [rsi], rax
14161c73a: mov qword ptr [rsi + 8], rdx
14161c73e: mov rax, rsi
14161c741: add rsp, 0x130
14161c748: pop rbx
14161c749: pop rdi
14161c74a: pop rsi
14161c74b: pop r12
14161c74d: pop r14
14161c74f: pop r15
14161c751: pop rbp
14161c752: ret 
14161c753: lea rcx, [rip + 0x1663a96]
14161c75a: lea r8, [rip + 0x1663ac7]
14161c761: mov edx, 0x37
14161c766: call 0x1428d9430
14161c76b: jmp 0x14161c901
14161c770: mov qword ptr [rbp + 0x10], 0
14161c778: lea rax, [rip + 0x1663ac9]
14161c77f: mov qword ptr [rsp + 0x20], rax
14161c784: lea r8, [rip + 0x1663ab5]
14161c78b: lea rdx, [rbp - 0x40]
14161c78f: lea r9, [rbp + 0x10]
14161c793: xor ecx, ecx
14161c795: call 0x1428d9596
14161c79a: jmp 0x14161c901
14161c79f: mov qword ptr [rbp + 0x10], 0
14161c7a7: lea rax, [rip + 0x1663ab2]
14161c7ae: mov qword ptr [rsp + 0x20], rax
14161c7b3: lea r8, [rip + 0x1663a86]
14161c7ba: lea rdx, [rbp - 0x40]
14161c7be: lea r9, [rbp + 0x10]
14161c7c2: xor ecx, ecx
14161c7c4: call 0x1428d9596
14161c7c9: jmp 0x14161c901
14161c7ce: mov qword ptr [rbp + 0x10], 0
14161c7d6: lea rax, [rip + 0x16ad76b]
14161c7dd: jmp 0x14161c7ee
14161c7df: mov qword ptr [rbp + 0x10], 0
14161c7e7: lea rax, [rip + 0x16ad772]
14161c7ee: mov qword ptr [rsp + 0x20], rax
14161c7f3: lea r8, [rip + 0x16ad746]
14161c7fa: lea rdx, [rbp - 0x40]
14161c7fe: lea r9, [rbp + 0x10]
14161c802: xor ecx, ecx
14161c804: call 0x1428d9596
14161c809: mov dword ptr [rbp + 0x94], r12d
14161c810: mov qword ptr [rbp - 0x48], r15
14161c814: movups xmm0, xmmword ptr [rbp]
14161c818: movaps xmmword ptr [rbp + 0x50], xmm0
14161c81c: movups xmm0, xmmword ptr [rbp - 0x40]
14161c820: movups xmm1, xmmword ptr [rbp - 0x30]
14161c824: movups xmm2, xmmword ptr [rbp - 0x20]
14161c828: movups xmm3, xmmword ptr [rbp - 0x10]
14161c82c: movaps xmmword ptr [rbp + 0x40], xmm3
14161c830: movaps xmmword ptr [rbp + 0x30], xmm2
14161c834: movaps xmmword ptr [rbp + 0x20], xmm1
14161c838: movaps xmmword ptr [rbp + 0x10], xmm0
14161c83c: lea rax, [rip + 0x1663965]
14161c843: mov qword ptr [rsp + 0x20], rax
14161c848: lea rcx, [rip + 0x1662299]
14161c84f: lea r9, [rip + 0x1662272]
14161c856: lea r8, [rbp + 0x10]
14161c85a: mov edx, 0x2b
14161c85f: call 0x1428d9760
14161c864: jmp 0x14161c901
14161c869: lea rcx, [rip + 0x1663968]
14161c870: call 0x1428d9310
14161c875: jmp 0x14161c901
14161c87a: lea rax, [rip + 0x1481cfb]
14161c881: mov qword ptr [rbp - 0x40], rax
14161c885: mov qword ptr [rbp - 0x38], 0x40
14161c88d: lea rax, [rbp - 0x40]
14161c891: mov qword ptr [rbp + 0x60], rax
14161c895: lea rax, [rip - 0x1512a0c]
14161c89c: mov qword ptr [rbp + 0x68], rax
14161c8a0: lea rax, [rip + 0x1662f21]
14161c8a7: mov qword ptr [rbp + 0x10], rax
14161c8ab: mov qword ptr [rbp + 0x18], 1
14161c8b3: mov qword ptr [rbp + 0x30], 0
14161c8bb: lea rax, [rbp + 0x60]
14161c8bf: mov qword ptr [rbp + 0x20], rax
14161c8c3: mov qword ptr [rbp + 0x28], 1
14161c8cb: mov byte ptr [rbp + 0xa7], 0
14161c8d2: lea rdx, [rip + 0x1662eff]
14161c8d9: lea rcx, [rbp + 0x10]
14161c8dd: call 0x1428d9390
14161c8e2: jmp 0x14161c901
14161c8e4: mov byte ptr [rbp + 0xa7], 0
14161c8eb: lea r8, [rip + 0x16621be]
14161c8f2: mov ecx, 4
14161c8f7: mov edx, 0x80000
14161c8fc: call 0x1428d8fe3
14161c901: ud2 
14161c903: nop word ptr cs:[rax + rax]
14161c910: mov qword ptr [rsp + 0x10], rdx
14161c915: push rbp
14161c916: push r15
14161c918: push r14
14161c91a: push r12
14161c91c: push rsi
14161c91d: push rdi
14161c91e: push rbx
14161c91f: sub rsp, 0x40
14161c923: lea rbp, [rdx + 0x80]
14161c92a: mov rsi, qword ptr [rbp - 0x48]
14161c92e: mov qword ptr [rbp + 0x98], rsi
14161c935: mov edi, dword ptr [rbp + 0x94]
14161c93b: mov byte ptr [rbp + 0xa7], dil
14161c942: lea rcx, [rbp + 0x10]
14161c946: call 0x1407cea70
14161c94b: mov qword ptr [rbp + 0x98], rsi
14161c952: mov byte ptr [rbp + 0xa7], dil
14161c959: add rsp, 0x40
14161c95d: pop rbx
14161c95e: pop rdi
14161c95f: pop rsi
14161c960: pop r12
14161c962: pop r14
14161c964: pop r15
14161c966: pop rbp
14161c967: ret 
14161c968: nop dword ptr [rax + rax]
14161c970: mov qword ptr [rsp + 0x10], rdx
14161c975: push rbp
14161c976: push r15
14161c978: push r14
14161c97a: push r12
14161c97c: push rsi
14161c97d: push rdi
14161c97e: push rbx
14161c97f: sub rsp, 0x40
14161c983: lea rbp, [rdx + 0x80]
14161c98a: test byte ptr [rbp + 0xa7], 1
14161c991: je 0x14161c9aa
14161c993: mov rcx, qword ptr [rbp + 0x98]
14161c99a: mov edx, 0x80000
14161c99f: mov r8d, 4
14161c9a5: call 0x140613c20
14161c9aa: nop 
14161c9ab: add rsp, 0x40
14161c9af: pop rbx
14161c9b0: pop rdi
14161c9b1: pop rsi
14161c9b2: pop r12
14161c9b4: pop r14
14161c9b6: pop r15
14161c9b8: pop rbp
14161c9b9: ret 
14161c9ba: nop word ptr [rax + rax]
14161c9c0: mov qword ptr [rsp + 0x10], rdx
14161c9c5: push rbp
14161c9c6: push r15
14161c9c8: push r14
14161c9ca: push r12
14161c9cc: push rsi
14161c9cd: push rdi
14161c9ce: push rbx
14161c9cf: sub rsp, 0x40
14161c9d3: lea rbp, [rdx + 0x80]
14161c9da: lea rcx, [rbp + 0x60]
14161c9de: call 0x140be24b0
14161c9e3: nop 
14161c9e4: add rsp, 0x40
14161c9e8: pop rbx
14161c9e9: pop rdi
14161c9ea: pop rsi
14161c9eb: pop r12
14161c9ed: pop r14
14161c9ef: pop r15
14161c9f1: pop rbp
14161c9f2: ret 
