14161d570: push rbp
14161d571: push r15
14161d573: push r14
14161d575: push r12
14161d577: push rsi
14161d578: push rdi
14161d579: push rbx
14161d57a: sub rsp, 0x130
14161d581: lea rbp, [rsp + 0x80]
14161d589: mov qword ptr [rbp + 0xa8], 0xfffffffffffffffe
14161d594: mov rdi, rdx
14161d597: mov rsi, rcx
14161d59a: test r8, r8
14161d59d: je 0x14161d5d9
14161d59f: mov rbx, r9
14161d5a2: mov rax, r9
14161d5a5: and rax, 3
14161d5a9: mov qword ptr [rbp - 0x40], rax
14161d5ad: jne 0x14161d7fe
14161d5b3: mov r15, r8
14161d5b6: mov rax, r8
14161d5b9: and rax, 3
14161d5bd: mov qword ptr [rbp - 0x40], rax
14161d5c1: jne 0x14161d80f
14161d5c7: cmp rbx, 0x8004
14161d5ce: jae 0x14161d8aa
14161d5d4: xor r12d, r12d
14161d5d7: jmp 0x14161d653
14161d5d9: movzx eax, byte ptr [rip + 0x2017629]
14161d5e0: mov ecx, 0x8000
14161d5e5: mov edx, 4
14161d5ea: call 0x140613c10
14161d5ef: test rax, rax
14161d5f2: je 0x14161d914
14161d5f8: mov r15, rax
14161d5fb: mov eax, 0x1c
14161d600: xorps xmm0, xmm0
14161d603: nop word ptr cs:[rax + rax]
14161d610: movups xmmword ptr [r15 + rax*4 - 0x70], xmm0
14161d616: movups xmmword ptr [r15 + rax*4 - 0x60], xmm0
14161d61c: movups xmmword ptr [r15 + rax*4 - 0x50], xmm0
14161d622: movups xmmword ptr [r15 + rax*4 - 0x40], xmm0
14161d628: movups xmmword ptr [r15 + rax*4 - 0x30], xmm0
14161d62e: movups xmmword ptr [r15 + rax*4 - 0x20], xmm0
14161d634: movups xmmword ptr [r15 + rax*4 - 0x10], xmm0
14161d63a: movups xmmword ptr [r15 + rax*4], xmm0
14161d63f: add rax, 0x20
14161d643: cmp rax, 0x201c
14161d649: jne 0x14161d610
14161d64b: mov r12b, 1
14161d64e: mov ebx, 0x8000
14161d653: mov qword ptr [rbp + 0x98], r15
14161d65a: mov byte ptr [rbp + 0xa7], r12b
14161d661: lea rcx, [rbp + 0x10]
14161d665: mov edx, 0x8000
14161d66a: mov r8d, 0x22
14161d670: mov r9b, 4
14161d673: call 0x140bd4de0
14161d678: mov qword ptr [rbp + 0x98], r15
14161d67f: mov byte ptr [rbp + 0xa7], r12b
14161d686: lea r14, [rbp + 0x60]
14161d68a: lea rdx, [rbp + 0x10]
14161d68e: mov r8d, 4
14161d694: mov rcx, r14
14161d697: call 0x140bd4f20
14161d69c: mov qword ptr [rbp + 0x98], r15
14161d6a3: mov byte ptr [rbp + 0xa7], r12b
14161d6aa: mov qword ptr [rsp + 0x30], rbx
14161d6af: mov qword ptr [rsp + 0x28], r15
14161d6b4: mov qword ptr [rsp + 0x20], r14
14161d6b9: lea r8, [rip + 0x16621ee]
14161d6c0: lea rcx, [rbp - 0x40]
14161d6c4: mov r9d, 9
14161d6ca: mov rdx, rdi
14161d6cd: call 0x1416a2dc0
14161d6d2: cmp dword ptr [rbp - 0x40], 0xb
14161d6d6: jne 0x14161d839
14161d6dc: mov rax, qword ptr [rbp - 0x18]
14161d6e0: mov qword ptr [rbp + 0x80], rax
14161d6e7: movups xmm0, xmmword ptr [rbp - 0x38]
14161d6eb: movups xmm1, xmmword ptr [rbp - 0x28]
14161d6ef: movaps xmmword ptr [rbp + 0x70], xmm1
14161d6f3: movaps xmmword ptr [rbp + 0x60], xmm0
14161d6f7: test r12b, r12b
14161d6fa: je 0x14161d70f
14161d6fc: mov edx, 0x8000
14161d701: mov r8d, 4
14161d707: mov rcx, r15
14161d70a: call 0x140613c20
14161d70f: lea rcx, [rbp + 0x60]
14161d713: mov rdx, rdi
14161d716: call 0x140bd4cd0
14161d71b: test rax, rax
14161d71e: je 0x14161d899
14161d724: cmp rdx, qword ptr [rbp + 0x78]
14161d728: jb 0x14161d783
14161d72a: mov rcx, rdx
14161d72d: and rcx, 3
14161d731: mov qword ptr [rbp - 0x40], rcx
14161d735: jne 0x14161d7a0
14161d737: mov rcx, rax
14161d73a: and rcx, 3
14161d73e: mov qword ptr [rbp - 0x40], rcx
14161d742: jne 0x14161d7cf
14161d748: mov rcx, qword ptr [rbp + 0x80]
14161d74f: mov qword ptr [rsi + 0x30], rcx
14161d753: movaps xmm0, xmmword ptr [rbp + 0x60]
14161d757: movaps xmm1, xmmword ptr [rbp + 0x70]
14161d75b: movups xmmword ptr [rsi + 0x20], xmm1
14161d75f: movups xmmword ptr [rsi + 0x10], xmm0
14161d763: shr rdx, 2
14161d767: mov qword ptr [rsi], rax
14161d76a: mov qword ptr [rsi + 8], rdx
14161d76e: mov rax, rsi
14161d771: add rsp, 0x130
14161d778: pop rbx
14161d779: pop rdi
14161d77a: pop rsi
14161d77b: pop r12
14161d77d: pop r14
14161d77f: pop r15
14161d781: pop rbp
14161d782: ret 
14161d783: lea rcx, [rip + 0x1662a66]
14161d78a: lea r8, [rip + 0x1662a97]
14161d791: mov edx, 0x37
14161d796: call 0x1428d9430
14161d79b: jmp 0x14161d931
14161d7a0: mov qword ptr [rbp + 0x10], 0
14161d7a8: lea rax, [rip + 0x1662a99]
14161d7af: mov qword ptr [rsp + 0x20], rax
14161d7b4: lea r8, [rip + 0x1662a85]
14161d7bb: lea rdx, [rbp - 0x40]
14161d7bf: lea r9, [rbp + 0x10]
14161d7c3: xor ecx, ecx
14161d7c5: call 0x1428d9596
14161d7ca: jmp 0x14161d931
14161d7cf: mov qword ptr [rbp + 0x10], 0
14161d7d7: lea rax, [rip + 0x1662a82]
14161d7de: mov qword ptr [rsp + 0x20], rax
14161d7e3: lea r8, [rip + 0x1662a56]
14161d7ea: lea rdx, [rbp - 0x40]
14161d7ee: lea r9, [rbp + 0x10]
14161d7f2: xor ecx, ecx
14161d7f4: call 0x1428d9596
14161d7f9: jmp 0x14161d931
14161d7fe: mov qword ptr [rbp + 0x10], 0
14161d806: lea rax, [rip + 0x16ac73b]
14161d80d: jmp 0x14161d81e
14161d80f: mov qword ptr [rbp + 0x10], 0
14161d817: lea rax, [rip + 0x16ac742]
14161d81e: mov qword ptr [rsp + 0x20], rax
14161d823: lea r8, [rip + 0x16ac716]
14161d82a: lea rdx, [rbp - 0x40]
14161d82e: lea r9, [rbp + 0x10]
14161d832: xor ecx, ecx
14161d834: call 0x1428d9596
14161d839: mov dword ptr [rbp + 0x94], r12d
14161d840: mov qword ptr [rbp - 0x48], r15
14161d844: movups xmm0, xmmword ptr [rbp]
14161d848: movaps xmmword ptr [rbp + 0x50], xmm0
14161d84c: movups xmm0, xmmword ptr [rbp - 0x40]
14161d850: movups xmm1, xmmword ptr [rbp - 0x30]
14161d854: movups xmm2, xmmword ptr [rbp - 0x20]
14161d858: movups xmm3, xmmword ptr [rbp - 0x10]
14161d85c: movaps xmmword ptr [rbp + 0x40], xmm3
14161d860: movaps xmmword ptr [rbp + 0x30], xmm2
14161d864: movaps xmmword ptr [rbp + 0x20], xmm1
14161d868: movaps xmmword ptr [rbp + 0x10], xmm0
14161d86c: lea rax, [rip + 0x1662935]
14161d873: mov qword ptr [rsp + 0x20], rax
14161d878: lea rcx, [rip + 0x1661269]
14161d87f: lea r9, [rip + 0x1661242]
14161d886: lea r8, [rbp + 0x10]
14161d88a: mov edx, 0x2b
14161d88f: call 0x1428d9760
14161d894: jmp 0x14161d931
14161d899: lea rcx, [rip + 0x1662938]
14161d8a0: call 0x1428d9310
14161d8a5: jmp 0x14161d931
14161d8aa: lea rax, [rip + 0x14808f8]
14161d8b1: mov qword ptr [rbp - 0x40], rax
14161d8b5: mov qword ptr [rbp - 0x38], 0x32
14161d8bd: lea rax, [rbp - 0x40]
14161d8c1: mov qword ptr [rbp + 0x60], rax
14161d8c5: lea rax, [rip - 0x1513a3c]
14161d8cc: mov qword ptr [rbp + 0x68], rax
14161d8d0: lea rax, [rip + 0x1661ef1]
14161d8d7: mov qword ptr [rbp + 0x10], rax
14161d8db: mov qword ptr [rbp + 0x18], 1
14161d8e3: mov qword ptr [rbp + 0x30], 0
14161d8eb: lea rax, [rbp + 0x60]
14161d8ef: mov qword ptr [rbp + 0x20], rax
14161d8f3: mov qword ptr [rbp + 0x28], 1
14161d8fb: mov byte ptr [rbp + 0xa7], 0
14161d902: lea rdx, [rip + 0x1661ecf]
14161d909: lea rcx, [rbp + 0x10]
14161d90d: call 0x1428d9390
14161d912: jmp 0x14161d931
14161d914: mov byte ptr [rbp + 0xa7], 0
14161d91b: lea r8, [rip + 0x166118e]
14161d922: mov ecx, 4
14161d927: mov edx, 0x8000
14161d92c: call 0x1428d8fe3
14161d931: ud2 
14161d933: nop word ptr cs:[rax + rax]
14161d940: mov qword ptr [rsp + 0x10], rdx
14161d945: push rbp
14161d946: push r15
14161d948: push r14
14161d94a: push r12
14161d94c: push rsi
14161d94d: push rdi
14161d94e: push rbx
14161d94f: sub rsp, 0x40
14161d953: lea rbp, [rdx + 0x80]
14161d95a: mov rsi, qword ptr [rbp - 0x48]
14161d95e: mov qword ptr [rbp + 0x98], rsi
14161d965: mov edi, dword ptr [rbp + 0x94]
14161d96b: mov byte ptr [rbp + 0xa7], dil
14161d972: lea rcx, [rbp + 0x10]
14161d976: call 0x1407cea70
14161d97b: mov qword ptr [rbp + 0x98], rsi
14161d982: mov byte ptr [rbp + 0xa7], dil
14161d989: add rsp, 0x40
14161d98d: pop rbx
14161d98e: pop rdi
14161d98f: pop rsi
14161d990: pop r12
14161d992: pop r14
14161d994: pop r15
14161d996: pop rbp
14161d997: ret 
14161d998: nop dword ptr [rax + rax]
14161d9a0: mov qword ptr [rsp + 0x10], rdx
14161d9a5: push rbp
14161d9a6: push r15
14161d9a8: push r14
14161d9aa: push r12
14161d9ac: push rsi
14161d9ad: push rdi
14161d9ae: push rbx
14161d9af: sub rsp, 0x40
14161d9b3: lea rbp, [rdx + 0x80]
14161d9ba: test byte ptr [rbp + 0xa7], 1
14161d9c1: je 0x14161d9da
14161d9c3: mov rcx, qword ptr [rbp + 0x98]
14161d9ca: mov edx, 0x8000
14161d9cf: mov r8d, 4
14161d9d5: call 0x140613c20
14161d9da: nop 
14161d9db: add rsp, 0x40
14161d9df: pop rbx
14161d9e0: pop rdi
14161d9e1: pop rsi
14161d9e2: pop r12
14161d9e4: pop r14
14161d9e6: pop r15
14161d9e8: pop rbp
14161d9e9: ret 
14161d9ea: nop word ptr [rax + rax]
14161d9f0: mov qword ptr [rsp + 0x10], rdx
14161d9f5: push rbp
14161d9f6: push r15
14161d9f8: push r14
14161d9fa: push r12
14161d9fc: push rsi
14161d9fd: push rdi
14161d9fe: push rbx
14161d9ff: sub rsp, 0x40
14161da03: lea rbp, [rdx + 0x80]
14161da0a: lea rcx, [rbp + 0x60]
14161da0e: call 0x140be24b0
14161da13: nop 
14161da14: add rsp, 0x40
14161da18: pop rbx
14161da19: pop rdi
14161da1a: pop rsi
14161da1b: pop r12
14161da1d: pop r14
14161da1f: pop r15
14161da21: pop rbp
14161da22: ret 
