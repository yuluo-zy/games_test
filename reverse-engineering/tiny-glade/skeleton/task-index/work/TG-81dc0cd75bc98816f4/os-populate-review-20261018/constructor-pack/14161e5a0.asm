14161e5a0: push rbp
14161e5a1: push r15
14161e5a3: push r14
14161e5a5: push r12
14161e5a7: push rsi
14161e5a8: push rdi
14161e5a9: push rbx
14161e5aa: sub rsp, 0x110
14161e5b1: lea rbp, [rsp + 0x80]
14161e5b9: mov qword ptr [rbp + 0x88], 0xfffffffffffffffe
14161e5c4: mov rbx, r8
14161e5c7: mov rdi, rdx
14161e5ca: mov rsi, rcx
14161e5cd: test r8, r8
14161e5d0: je 0x14161e665
14161e5d6: mov r14, r9
14161e5d9: mov rax, r9
14161e5dc: and rax, 0x1f
14161e5e0: mov qword ptr [rbp - 0x40], rax
14161e5e4: jne 0x14161e7d3
14161e5ea: mov rax, rbx
14161e5ed: and rax, 3
14161e5f1: mov qword ptr [rbp - 0x40], rax
14161e5f5: jne 0x14161e7e4
14161e5fb: cmp r14, 0x4e220
14161e602: jb 0x14161e668
14161e604: lea rax, [rip + 0x147e936]
14161e60b: mov qword ptr [rbp - 0x40], rax
14161e60f: mov qword ptr [rbp - 0x38], 0x34
14161e617: lea rax, [rbp - 0x40]
14161e61b: mov qword ptr [rbp + 0x60], rax
14161e61f: lea rax, [rip - 0x1514796]
14161e626: mov qword ptr [rbp + 0x68], rax
14161e62a: lea rax, [rip + 0x1661197]
14161e631: mov qword ptr [rbp + 0x10], rax
14161e635: mov qword ptr [rbp + 0x18], 1
14161e63d: mov qword ptr [rbp + 0x30], 0
14161e645: lea rax, [rbp + 0x60]
14161e649: mov qword ptr [rbp + 0x20], rax
14161e64d: mov qword ptr [rbp + 0x28], 1
14161e655: lea rdx, [rip + 0x166117c]
14161e65c: lea rcx, [rbp + 0x10]
14161e660: call 0x1428d9390
14161e665: xor r14d, r14d
14161e668: lea r15, [rbp + 0x10]
14161e66c: mov edx, 0x4e200
14161e671: mov rcx, r15
14161e674: mov r8d, 0x22
14161e67a: mov r9b, 2
14161e67d: call 0x140bd4de0
14161e682: lea r12, [rbp + 0x60]
14161e686: mov r8d, 4
14161e68c: mov rcx, r12
14161e68f: mov rdx, r15
14161e692: call 0x140bd4f20
14161e697: mov qword ptr [rsp + 0x30], r14
14161e69c: mov qword ptr [rsp + 0x28], rbx
14161e6a1: mov qword ptr [rsp + 0x20], r12
14161e6a6: lea r8, [rip + 0x16611d9]
14161e6ad: lea rcx, [rbp - 0x40]
14161e6b1: mov r9d, 0xb
14161e6b7: mov rdx, rdi
14161e6ba: call 0x1416a2dc0
14161e6bf: cmp dword ptr [rbp - 0x40], 0xb
14161e6c3: jne 0x14161e80e
14161e6c9: mov rax, qword ptr [rbp - 0x18]
14161e6cd: mov qword ptr [rbp + 0x80], rax
14161e6d4: movups xmm0, xmmword ptr [rbp - 0x38]
14161e6d8: movups xmm1, xmmword ptr [rbp - 0x28]
14161e6dc: movaps xmmword ptr [rbp + 0x70], xmm1
14161e6e0: movaps xmmword ptr [rbp + 0x60], xmm0
14161e6e4: lea rcx, [rbp + 0x60]
14161e6e8: mov rdx, rdi
14161e6eb: call 0x140bd4cd0
14161e6f0: test rax, rax
14161e6f3: je 0x14161e860
14161e6f9: cmp rdx, qword ptr [rbp + 0x78]
14161e6fd: jb 0x14161e758
14161e6ff: mov rcx, rdx
14161e702: and rcx, 0x1f
14161e706: mov qword ptr [rbp - 0x40], rcx
14161e70a: jne 0x14161e775
14161e70c: mov rcx, rax
14161e70f: and rcx, 3
14161e713: mov qword ptr [rbp - 0x40], rcx
14161e717: jne 0x14161e7a4
14161e71d: mov rcx, qword ptr [rbp + 0x80]
14161e724: mov qword ptr [rsi + 0x30], rcx
14161e728: movaps xmm0, xmmword ptr [rbp + 0x60]
14161e72c: movaps xmm1, xmmword ptr [rbp + 0x70]
14161e730: movups xmmword ptr [rsi + 0x20], xmm1
14161e734: movups xmmword ptr [rsi + 0x10], xmm0
14161e738: shr rdx, 5
14161e73c: mov qword ptr [rsi], rax
14161e73f: mov qword ptr [rsi + 8], rdx
14161e743: mov rax, rsi
14161e746: add rsp, 0x110
14161e74d: pop rbx
14161e74e: pop rdi
14161e74f: pop rsi
14161e750: pop r12
14161e752: pop r14
14161e754: pop r15
14161e756: pop rbp
14161e757: ret 
14161e758: lea rcx, [rip + 0x1661a91]
14161e75f: lea r8, [rip + 0x1661ac2]
14161e766: mov edx, 0x37
14161e76b: call 0x1428d9430
14161e770: jmp 0x14161e86c
14161e775: mov qword ptr [rbp + 0x10], 0
14161e77d: lea rax, [rip + 0x1661ac4]
14161e784: mov qword ptr [rsp + 0x20], rax
14161e789: lea r8, [rip + 0x1661ab0]
14161e790: lea rdx, [rbp - 0x40]
14161e794: lea r9, [rbp + 0x10]
14161e798: xor ecx, ecx
14161e79a: call 0x1428d9596
14161e79f: jmp 0x14161e86c
14161e7a4: mov qword ptr [rbp + 0x10], 0
14161e7ac: lea rax, [rip + 0x1661aad]
14161e7b3: mov qword ptr [rsp + 0x20], rax
14161e7b8: lea r8, [rip + 0x1661a81]
14161e7bf: lea rdx, [rbp - 0x40]
14161e7c3: lea r9, [rbp + 0x10]
14161e7c7: xor ecx, ecx
14161e7c9: call 0x1428d9596
14161e7ce: jmp 0x14161e86c
14161e7d3: mov qword ptr [rbp + 0x10], 0
14161e7db: lea rax, [rip + 0x16ab766]
14161e7e2: jmp 0x14161e7f3
14161e7e4: mov qword ptr [rbp + 0x10], 0
14161e7ec: lea rax, [rip + 0x16ab76d]
14161e7f3: mov qword ptr [rsp + 0x20], rax
14161e7f8: lea r8, [rip + 0x16ab741]
14161e7ff: lea rdx, [rbp - 0x40]
14161e803: lea r9, [rbp + 0x10]
14161e807: xor ecx, ecx
14161e809: call 0x1428d9596
14161e80e: movups xmm0, xmmword ptr [rbp]
14161e812: movaps xmmword ptr [rbp + 0x50], xmm0
14161e816: movups xmm0, xmmword ptr [rbp - 0x40]
14161e81a: movups xmm1, xmmword ptr [rbp - 0x30]
14161e81e: movups xmm2, xmmword ptr [rbp - 0x20]
14161e822: movups xmm3, xmmword ptr [rbp - 0x10]
14161e826: movaps xmmword ptr [rbp + 0x40], xmm3
14161e82a: movaps xmmword ptr [rbp + 0x30], xmm2
14161e82e: movaps xmmword ptr [rbp + 0x20], xmm1
14161e832: movaps xmmword ptr [rbp + 0x10], xmm0
14161e836: lea rax, [rip + 0x166196b]
14161e83d: mov qword ptr [rsp + 0x20], rax
14161e842: lea rcx, [rip + 0x166029f]
14161e849: lea r9, [rip + 0x1660278]
14161e850: lea r8, [rbp + 0x10]
14161e854: mov edx, 0x2b
14161e859: call 0x1428d9760
14161e85e: jmp 0x14161e86c
14161e860: lea rcx, [rip + 0x1661971]
14161e867: call 0x1428d9310
14161e86c: ud2 
14161e86e: nop 
14161e870: mov qword ptr [rsp + 0x10], rdx
14161e875: push rbp
14161e876: push r15
14161e878: push r14
14161e87a: push r12
14161e87c: push rsi
14161e87d: push rdi
14161e87e: push rbx
14161e87f: sub rsp, 0x40
14161e883: lea rbp, [rdx + 0x80]
14161e88a: lea rcx, [rbp + 0x60]
14161e88e: call 0x140be24b0
14161e893: nop 
14161e894: add rsp, 0x40
14161e898: pop rbx
14161e899: pop rdi
14161e89a: pop rsi
14161e89b: pop r12
14161e89d: pop r14
14161e89f: pop r15
14161e8a1: pop rbp
14161e8a2: ret 
14161e8a3: nop word ptr cs:[rax + rax]
14161e8b0: mov qword ptr [rsp + 0x10], rdx
14161e8b5: push rbp
14161e8b6: push r15
14161e8b8: push r14
14161e8ba: push r12
14161e8bc: push rsi
14161e8bd: push rdi
14161e8be: push rbx
14161e8bf: sub rsp, 0x40
14161e8c3: lea rbp, [rdx + 0x80]
14161e8ca: lea rcx, [rbp + 0x10]
14161e8ce: call 0x1407cea70
14161e8d3: nop 
14161e8d4: add rsp, 0x40
14161e8d8: pop rbx
14161e8d9: pop rdi
14161e8da: pop rsi
14161e8db: pop r12
14161e8dd: pop r14
14161e8df: pop r15
14161e8e1: pop rbp
14161e8e2: ret 
