140dce380: push rbp
140dce381: push r15
140dce383: push r14
140dce385: push r13
140dce387: push r12
140dce389: push rsi
140dce38a: push rdi
140dce38b: push rbx
140dce38c: sub rsp, 0xf8
140dce393: lea rbp, [rsp + 0x80]
140dce39b: mov qword ptr [rbp + 0x70], 0xfffffffffffffffe
140dce3a3: mov rbx, r8
140dce3a6: mov rdi, rdx
140dce3a9: mov rsi, rcx
140dce3ac: lea rcx, [rbp - 0x20]
140dce3b0: call 0x140dcdb00
140dce3b5: test byte ptr [rbp - 0x20], 1
140dce3b9: je 0x140dce3f7
140dce3bb: mov rax, qword ptr [rdi + 0x20]
140dce3bf: mov r8, qword ptr [rdi + 0x28]
140dce3c3: cmp byte ptr [rdi + 0x70], 0
140dce3c7: je 0x140dce413
140dce3c9: cmp rax, r8
140dce3cc: je 0x140dce445
140dce3ce: mov rcx, qword ptr [rdi + 8]
140dce3d2: mov rdx, qword ptr [rcx + 0x10]
140dce3d6: sub r8, rax
140dce3d9: shr r8, 2
140dce3dd: xor r9d, r9d
140dce3e0: mov ecx, dword ptr [rax + r9*4]
140dce3e4: cmp rdx, rcx
140dce3e7: jbe 0x140dce60c
140dce3ed: inc r9
140dce3f0: cmp r8, r9
140dce3f3: jne 0x140dce3e0
140dce3f5: jmp 0x140dce445
140dce3f7: mov qword ptr [rsi], 0
140dce3fe: mov qword ptr [rsi + 8], 0x10
140dce406: mov qword ptr [rsi + 0x10], 0
140dce40e: jmp 0x140dce5d9
140dce413: cmp rax, r8
140dce416: je 0x140dce445
140dce418: mov rcx, qword ptr [rdi + 0x10]
140dce41c: mov rdx, qword ptr [rcx + 0x10]
140dce420: sub r8, rax
140dce423: shr r8, 2
140dce427: xor r9d, r9d
140dce42a: nop word ptr [rax + rax]
140dce430: mov ecx, dword ptr [rax + r9*4]
140dce434: cmp rdx, rcx
140dce437: jbe 0x140dce618
140dce43d: inc r9
140dce440: cmp r8, r9
140dce443: jne 0x140dce430
140dce445: movzx eax, byte ptr [rip + 0x28667bd]
140dce44c: mov ecx, 0x80
140dce451: mov edx, 0x10
140dce456: call 0x140613c10
140dce45b: test rax, rax
140dce45e: je 0x140dce624
140dce464: mov r12, rax
140dce467: movaps xmm0, xmmword ptr [rbp - 0x10]
140dce46b: movaps xmm1, xmmword ptr [rbp]
140dce46f: movaps xmmword ptr [rax + 0x10], xmm1
140dce473: movaps xmmword ptr [rax], xmm0
140dce476: mov qword ptr [rbp + 0x58], 4
140dce47e: mov qword ptr [rbp + 0x60], rax
140dce482: mov qword ptr [rbp + 0x68], 1
140dce48a: mov rax, qword ptr [rdi + 0x70]
140dce48e: mov qword ptr [rbp + 0x50], rax
140dce492: movups xmm0, xmmword ptr [rdi + 0x60]
140dce496: movaps xmmword ptr [rbp + 0x40], xmm0
140dce49a: movups xmm0, xmmword ptr [rdi + 0x50]
140dce49e: movaps xmmword ptr [rbp + 0x30], xmm0
140dce4a2: movups xmm0, xmmword ptr [rdi + 0x40]
140dce4a6: movaps xmmword ptr [rbp + 0x20], xmm0
140dce4aa: movups xmm0, xmmword ptr [rdi]
140dce4ad: movups xmm1, xmmword ptr [rdi + 0x10]
140dce4b1: movups xmm2, xmmword ptr [rdi + 0x20]
140dce4b5: movups xmm3, xmmword ptr [rdi + 0x30]
140dce4b9: movaps xmmword ptr [rbp + 0x10], xmm3
140dce4bd: movaps xmmword ptr [rbp], xmm2
140dce4c1: movaps xmmword ptr [rbp - 0x10], xmm1
140dce4c5: movaps xmmword ptr [rbp - 0x20], xmm0
140dce4c9: mov edi, 1
140dce4ce: lea r13, [rbp - 0x40]
140dce4d2: lea rbx, [rbp - 0x50]
140dce4d6: lea r14, [rbp - 0x20]
140dce4da: lea r15, [rbp + 0x58]
140dce4de: jmp 0x140dce503
140dce4e0: mov rax, rdi
140dce4e3: shl rax, 5
140dce4e7: movaps xmm0, xmmword ptr [r13]
140dce4ec: movaps xmm1, xmmword ptr [r13 + 0x10]
140dce4f1: movaps xmmword ptr [r12 + rax + 0x10], xmm1
140dce4f7: movaps xmmword ptr [r12 + rax], xmm0
140dce4fc: inc rdi
140dce4ff: mov qword ptr [rbp + 0x68], rdi
140dce503: mov rcx, rbx
140dce506: mov rdx, r14
140dce509: call 0x140dcdb00
140dce50e: test byte ptr [rbp - 0x50], 1
140dce512: je 0x140dce5ca
140dce518: cmp rdi, qword ptr [rbp + 0x58]
140dce51c: jne 0x140dce4e0
140dce51e: mov rax, qword ptr [rbp]
140dce522: mov r8, qword ptr [rbp + 8]
140dce526: cmp byte ptr [rbp + 0x50], 0
140dce52a: je 0x140dce570
140dce52c: cmp rax, r8
140dce52f: je 0x140dce5a1
140dce531: mov rcx, qword ptr [rbp - 0x18]
140dce535: mov rdx, qword ptr [rcx + 0x10]
140dce539: sub r8, rax
140dce53c: shr r8, 2
140dce540: xor r9d, r9d
140dce543: nop word ptr cs:[rax + rax]
140dce550: mov ecx, dword ptr [rax + r9*4]
140dce554: cmp rdx, rcx
140dce557: jbe 0x140dce5f0
140dce55d: inc r9
140dce560: cmp r8, r9
140dce563: jne 0x140dce550
140dce565: jmp 0x140dce5a1
140dce567: nop word ptr [rax + rax]
140dce570: cmp rax, r8
140dce573: je 0x140dce5a1
140dce575: mov rcx, qword ptr [rbp - 0x10]
140dce579: mov rdx, qword ptr [rcx + 0x10]
140dce57d: sub r8, rax
140dce580: shr r8, 2
140dce584: xor r9d, r9d
140dce587: nop word ptr [rax + rax]
140dce590: mov ecx, dword ptr [rax + r9*4]
140dce594: cmp rdx, rcx
140dce597: jbe 0x140dce5fe
140dce599: inc r9
140dce59c: cmp r8, r9
140dce59f: jne 0x140dce590
140dce5a1: mov qword ptr [rsp + 0x20], 0x20
140dce5aa: mov r8d, 1
140dce5b0: mov r9d, 0x10
140dce5b6: mov rcx, r15
140dce5b9: mov rdx, rdi
140dce5bc: call 0x1428dacd0
140dce5c1: mov r12, qword ptr [rbp + 0x60]
140dce5c5: jmp 0x140dce4e0
140dce5ca: mov rax, qword ptr [rbp + 0x68]
140dce5ce: mov qword ptr [rsi + 0x10], rax
140dce5d2: movups xmm0, xmmword ptr [rbp + 0x58]
140dce5d6: movups xmmword ptr [rsi], xmm0
140dce5d9: mov rax, rsi
140dce5dc: add rsp, 0xf8
140dce5e3: pop rbx
140dce5e4: pop rdi
140dce5e5: pop rsi
140dce5e6: pop r12
140dce5e8: pop r13
140dce5ea: pop r14
140dce5ec: pop r15
140dce5ee: pop rbp
140dce5ef: ret 
140dce5f0: lea r8, [rip + 0x20dbbc9]
140dce5f7: call 0x1428d9518
140dce5fc: jmp 0x140dce60a
140dce5fe: lea r8, [rip + 0x20dbba3]
140dce605: call 0x1428d9518
140dce60a: ud2 
140dce60c: lea r8, [rip + 0x20dbbad]
140dce613: call 0x1428d9518
140dce618: lea r8, [rip + 0x20dbb89]
140dce61f: call 0x1428d9518
140dce624: mov ecx, 0x10
140dce629: mov edx, 0x80
140dce62e: mov r8, rbx
140dce631: call 0x1428d8fe3
140dce636: int3 
140dce637: nop word ptr [rax + rax]
140dce640: mov qword ptr [rsp + 0x10], rdx
140dce645: push rbp
140dce646: push r15
140dce648: push r14
140dce64a: push r13
140dce64c: push r12
140dce64e: push rsi
140dce64f: push rdi
140dce650: push rbx
140dce651: sub rsp, 0x28
140dce655: lea rbp, [rdx + 0x80]
140dce65c: mov rdx, qword ptr [rbp + 0x58]
140dce660: test rdx, rdx
140dce663: je 0x140dce678
140dce665: mov rcx, qword ptr [rbp + 0x60]
140dce669: shl rdx, 5
140dce66d: mov r8d, 0x10
140dce673: call 0x140613c20
140dce678: nop 
140dce679: add rsp, 0x28
140dce67d: pop rbx
140dce67e: pop rdi
140dce67f: pop rsi
140dce680: pop r12
140dce682: pop r13
140dce684: pop r14
140dce686: pop r15
140dce688: pop rbp
140dce689: ret 
