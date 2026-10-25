1409e86b0: push r15
1409e86b2: push r14
1409e86b4: push r13
1409e86b6: push r12
1409e86b8: push rsi
1409e86b9: push rdi
1409e86ba: push rbp
1409e86bb: push rbx
1409e86bc: sub rsp, 0x98
1409e86c3: movaps xmmword ptr [rsp + 0x80], xmm7
1409e86cb: movaps xmmword ptr [rsp + 0x70], xmm6
1409e86d0: mov r15, r9
1409e86d3: mov rdi, r8
1409e86d6: mov rbx, rdx
1409e86d9: mov rsi, rcx
1409e86dc: call 0x142923700
1409e86e1: mov r14, rax
1409e86e4: mov rax, qword ptr [r15]
1409e86e7: lock inc qword ptr [rax]
1409e86eb: jle 0x1409e8a9c
1409e86f1: mov qword ptr [rsp + 0x60], rax
1409e86f6: mov qword ptr [rsp + 0x68], r14
1409e86fb: mov r9, qword ptr [rbx + 8]
1409e86ff: lea rcx, [rsp + 0x53]
1409e8704: mov rdx, r14
1409e8707: mov r8, rdi
1409e870a: call 0x142923650
1409e870f: movsd xmm6, qword ptr [rsp + 0x57]
1409e8715: mov r9, qword ptr [rbx + 0x10]
1409e8719: lea rcx, [rsp + 0x53]
1409e871e: mov rdx, r14
1409e8721: mov r8, rdi
1409e8724: call 0x142923650
1409e8729: movhps xmm6, qword ptr [rsp + 0x57]
1409e872e: mov r9, qword ptr [rbx + 0x18]
1409e8732: lea rcx, [rsp + 0x53]
1409e8737: mov rdx, r14
1409e873a: mov r8, rdi
1409e873d: call 0x142923650
1409e8742: movsd xmm7, qword ptr [rsp + 0x57]
1409e8748: mov r8, qword ptr [rbx + 0x20]
1409e874c: mov rcx, r14
1409e874f: mov rdx, rdi
1409e8752: call 0x142923660
1409e8757: mov word ptr [rsp + 0x50], ax
1409e875c: mov r8, qword ptr [rbx + 0x28]
1409e8760: mov rcx, r14
1409e8763: mov rdx, rdi
1409e8766: call 0x142923660
1409e876b: mov word ptr [rsp + 0x4e], ax
1409e8770: mov r8, qword ptr [rbx + 0x30]
1409e8774: mov rcx, r14
1409e8777: mov rdx, rdi
1409e877a: call 0x142923660
1409e877f: mov word ptr [rsp + 0x4c], ax
1409e8784: mov r8, qword ptr [rbx + 0x38]
1409e8788: mov rcx, r14
1409e878b: mov rdx, rdi
1409e878e: call 0x142923660
1409e8793: mov word ptr [rsp + 0x4a], ax
1409e8798: mov r8, qword ptr [rbx + 0x40]
1409e879c: mov rcx, r14
1409e879f: mov rdx, rdi
1409e87a2: call 0x142923660
1409e87a7: mov word ptr [rsp + 0x48], ax
1409e87ac: mov r8, qword ptr [rbx + 0x48]
1409e87b0: mov rcx, r14
1409e87b3: mov rdx, rdi
1409e87b6: call 0x142923660
1409e87bb: mov word ptr [rsp + 0x46], ax
1409e87c0: mov r8, qword ptr [rbx + 0x50]
1409e87c4: mov rcx, r14
1409e87c7: mov rdx, rdi
1409e87ca: call 0x142923660
1409e87cf: mov word ptr [rsp + 0x44], ax
1409e87d4: mov r8, qword ptr [rbx + 0x58]
1409e87d8: mov rcx, r14
1409e87db: mov rdx, rdi
1409e87de: call 0x142923660
1409e87e3: mov word ptr [rsp + 0x42], ax
1409e87e8: mov r8, qword ptr [rbx + 0x60]
1409e87ec: mov rcx, r14
1409e87ef: mov rdx, rdi
1409e87f2: call 0x142923660
1409e87f7: mov word ptr [rsp + 0x40], ax
1409e87fc: mov r8, qword ptr [rbx + 0x68]
1409e8800: mov rcx, r14
1409e8803: mov rdx, rdi
1409e8806: call 0x142923660
1409e880b: mov word ptr [rsp + 0x3e], ax
1409e8810: mov r8, qword ptr [rbx + 0x70]
1409e8814: mov rcx, r14
1409e8817: mov rdx, rdi
1409e881a: call 0x142923660
1409e881f: mov word ptr [rsp + 0x3c], ax
1409e8824: mov r8, qword ptr [rbx + 0x78]
1409e8828: mov rcx, r14
1409e882b: mov rdx, rdi
1409e882e: call 0x142923660
1409e8833: mov word ptr [rsp + 0x3a], ax
1409e8838: mov r8, qword ptr [rbx + 0x80]
1409e883f: mov rcx, r14
1409e8842: mov rdx, rdi
1409e8845: call 0x142923660
1409e884a: mov word ptr [rsp + 0x38], ax
1409e884f: mov r8, qword ptr [rbx + 0x88]
1409e8856: mov rcx, r14
1409e8859: mov rdx, rdi
1409e885c: call 0x142923660
1409e8861: mov word ptr [rsp + 0x36], ax
1409e8866: mov r8, qword ptr [rbx + 0x90]
1409e886d: mov r12, qword ptr [rsp + 0x68]
1409e8872: mov rcx, r12
1409e8875: mov rdx, rdi
1409e8878: call 0x142923660
1409e887d: mov word ptr [rsp + 0x34], ax
1409e8882: mov r8, qword ptr [rbx + 0x98]
1409e8889: mov rcx, r12
1409e888c: mov rdx, rdi
1409e888f: call 0x142923660
1409e8894: mov word ptr [rsp + 0x32], ax
1409e8899: mov r8, qword ptr [rbx + 0xa0]
1409e88a0: mov rcx, r12
1409e88a3: mov rdx, rdi
1409e88a6: call 0x142923660
1409e88ab: mov word ptr [rsp + 0x30], ax
1409e88b0: mov r8, qword ptr [rbx + 0xa8]
1409e88b7: mov rcx, r12
1409e88ba: mov rdx, rdi
1409e88bd: call 0x142923660
1409e88c2: mov word ptr [rsp + 0x2e], ax
1409e88c7: mov r8, qword ptr [rbx + 0xb0]
1409e88ce: mov rcx, r12
1409e88d1: mov rdx, rdi
1409e88d4: call 0x142923660
1409e88d9: mov word ptr [rsp + 0x2c], ax
1409e88de: mov r8, qword ptr [rbx + 0xb8]
1409e88e5: mov rcx, r12
1409e88e8: mov rdx, rdi
1409e88eb: call 0x142923660
1409e88f0: mov word ptr [rsp + 0x2a], ax
1409e88f5: mov r8, qword ptr [rbx + 0xc0]
1409e88fc: mov rcx, r12
1409e88ff: mov rdx, rdi
1409e8902: call 0x142923660
1409e8907: mov ebp, eax
1409e8909: mov r8, qword ptr [rbx + 0xc8]
1409e8910: mov rcx, r12
1409e8913: mov rdx, rdi
1409e8916: call 0x142923660
1409e891b: mov r15d, eax
1409e891e: mov r8, qword ptr [rbx + 0xd0]
1409e8925: mov rcx, r12
1409e8928: mov rdx, rdi
1409e892b: call 0x142923660
1409e8930: mov r14d, eax
1409e8933: mov r8, qword ptr [rbx + 0xd8]
1409e893a: mov rcx, r12
1409e893d: mov rdx, rdi
1409e8940: call 0x142923660
1409e8945: mov r13d, eax
1409e8948: mov r8, qword ptr [rbx + 0xe0]
1409e894f: mov rcx, r12
1409e8952: mov rdx, rdi
1409e8955: call 0x142923660
1409e895a: movups xmmword ptr [rsi], xmm6
1409e895d: movsd qword ptr [rsi + 0x10], xmm7
1409e8962: movzx ecx, word ptr [rsp + 0x50]
1409e8967: and cl, 1
1409e896a: mov byte ptr [rsi + 0x18], cl
1409e896d: movzx ecx, word ptr [rsp + 0x4e]
1409e8972: and cl, 1
1409e8975: mov byte ptr [rsi + 0x19], cl
1409e8978: movzx ecx, word ptr [rsp + 0x4c]
1409e897d: and cl, 1
1409e8980: mov byte ptr [rsi + 0x1a], cl
1409e8983: movzx ecx, word ptr [rsp + 0x4a]
1409e8988: and cl, 1
1409e898b: mov byte ptr [rsi + 0x1b], cl
1409e898e: movzx ecx, word ptr [rsp + 0x48]
1409e8993: and cl, 1
1409e8996: mov byte ptr [rsi + 0x1c], cl
1409e8999: movzx ecx, word ptr [rsp + 0x46]
1409e899e: and cl, 1
1409e89a1: mov byte ptr [rsi + 0x1d], cl
1409e89a4: movzx ecx, word ptr [rsp + 0x44]
1409e89a9: and cl, 1
1409e89ac: mov byte ptr [rsi + 0x1e], cl
1409e89af: movzx ecx, word ptr [rsp + 0x42]
1409e89b4: and cl, 1
1409e89b7: mov byte ptr [rsi + 0x1f], cl
1409e89ba: movzx ecx, word ptr [rsp + 0x40]
1409e89bf: and cl, 1
1409e89c2: mov byte ptr [rsi + 0x20], cl
1409e89c5: movzx ecx, word ptr [rsp + 0x3e]
1409e89ca: and cl, 1
1409e89cd: mov byte ptr [rsi + 0x21], cl
1409e89d0: movzx ecx, word ptr [rsp + 0x3c]
1409e89d5: and cl, 1
1409e89d8: mov byte ptr [rsi + 0x22], cl
1409e89db: movzx ecx, word ptr [rsp + 0x3a]
1409e89e0: and cl, 1
1409e89e3: mov byte ptr [rsi + 0x23], cl
1409e89e6: movzx ecx, word ptr [rsp + 0x38]
1409e89eb: and cl, 1
1409e89ee: mov byte ptr [rsi + 0x24], cl
1409e89f1: movzx ecx, word ptr [rsp + 0x36]
1409e89f6: and cl, 1
1409e89f9: mov byte ptr [rsi + 0x25], cl
1409e89fc: movzx ecx, word ptr [rsp + 0x34]
1409e8a01: and cl, 1
1409e8a04: mov byte ptr [rsi + 0x26], cl
1409e8a07: movzx ecx, word ptr [rsp + 0x32]
1409e8a0c: and cl, 1
1409e8a0f: mov byte ptr [rsi + 0x27], cl
1409e8a12: movzx ecx, word ptr [rsp + 0x30]
1409e8a17: and cl, 1
1409e8a1a: mov byte ptr [rsi + 0x28], cl
1409e8a1d: movzx ecx, word ptr [rsp + 0x2e]
1409e8a22: and cl, 1
1409e8a25: mov byte ptr [rsi + 0x29], cl
1409e8a28: movzx ecx, word ptr [rsp + 0x2c]
1409e8a2d: and cl, 1
1409e8a30: mov byte ptr [rsi + 0x2a], cl
1409e8a33: movzx ecx, word ptr [rsp + 0x2a]
1409e8a38: and cl, 1
1409e8a3b: mov byte ptr [rsi + 0x2b], cl
1409e8a3e: and bpl, 1
1409e8a42: mov byte ptr [rsi + 0x2c], bpl
1409e8a46: and r15b, 1
1409e8a4a: mov byte ptr [rsi + 0x2d], r15b
1409e8a4e: and r14b, 1
1409e8a52: mov byte ptr [rsi + 0x2e], r14b
1409e8a56: and r13b, 1
1409e8a5a: mov byte ptr [rsi + 0x2f], r13b
1409e8a5e: and al, 1
1409e8a60: mov byte ptr [rsi + 0x30], al
1409e8a63: mov rax, qword ptr [rsp + 0x60]
1409e8a68: lock dec qword ptr [rax]
1409e8a6c: jne 0x1409e8a78
1409e8a6e: lea rcx, [rsp + 0x60]
1409e8a73: call 0x140b85350
1409e8a78: mov rax, rsi
1409e8a7b: movaps xmm6, xmmword ptr [rsp + 0x70]
1409e8a80: movaps xmm7, xmmword ptr [rsp + 0x80]
1409e8a88: add rsp, 0x98
1409e8a8f: pop rbx
1409e8a90: pop rbp
1409e8a91: pop rdi
1409e8a92: pop rsi
1409e8a93: pop r12
1409e8a95: pop r13
1409e8a97: pop r14
1409e8a99: pop r15
1409e8a9b: ret 
1409e8a9c: ud2 
