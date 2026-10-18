1409dd790: push rbp
1409dd791: push r15
1409dd793: push r14
1409dd795: push rsi
1409dd796: push rdi
1409dd797: push rbx
1409dd798: sub rsp, 0x138
1409dd79f: lea rbp, [rsp + 0x80]
1409dd7a7: mov qword ptr [rbp + 0xb0], 0xfffffffffffffffe
1409dd7b2: mov rsi, qword ptr [r8]
1409dd7b5: xor eax, eax
1409dd7b7: cmp rax, qword ptr [rsi + 0x30]
1409dd7bb: jo 0x1409dd9ed
1409dd7c1: mov r10, qword ptr [rbp + 0x120]
1409dd7c8: mov rax, qword ptr [r10]
1409dd7cb: mov r10, qword ptr [r10 + 8]
1409dd7cf: mov r11, qword ptr [rax]
1409dd7d2: xor edi, edi
1409dd7d4: mov rbx, r11
1409dd7d7: sub rbx, qword ptr [r10 + 0x18]
1409dd7db: cmovb rbx, rdi
1409dd7df: sub r11, qword ptr [r10 + 0x38]
1409dd7e3: cmovb r11, rdi
1409dd7e7: mov r14, qword ptr [r10 + 0x30]
1409dd7eb: sub r14, r11
1409dd7ee: cmovb r14, rdi
1409dd7f2: setbe r11b
1409dd7f6: mov r15, qword ptr [r10 + 0x10]
1409dd7fa: sub r15, rbx
1409dd7fd: cmovb r15, rdi
1409dd801: mov r10, qword ptr [r10 + 0x40]
1409dd805: setbe bl
1409dd808: add r15, r14
1409dd80b: test bl, r11b
1409dd80e: cmove r15, rdi
1409dd812: sub r10, r15
1409dd815: mov qword ptr [rax], r10
1409dd818: mov al, 1
1409dd81a: test bl, r11b
1409dd81d: je 0x1409dd876
1409dd81f: mov rax, qword ptr [rbp + 0x128]
1409dd826: mov r10, qword ptr [rax]
1409dd829: mov rax, qword ptr [rax + 8]
1409dd82d: mov r11, qword ptr [r10]
1409dd830: xor edi, edi
1409dd832: mov rbx, r11
1409dd835: sub rbx, qword ptr [rax + 0x18]
1409dd839: cmovb rbx, rdi
1409dd83d: sub r11, qword ptr [rax + 0x38]
1409dd841: cmovb r11, rdi
1409dd845: mov r14, qword ptr [rax + 0x10]
1409dd849: mov r15, qword ptr [rax + 0x30]
1409dd84d: sub r15, r11
1409dd850: cmovb r15, rdi
1409dd854: seta r11b
1409dd858: sub r14, rbx
1409dd85b: cmovb r14, rdi
1409dd85f: mov rbx, qword ptr [rax + 0x40]
1409dd863: seta al
1409dd866: add r14, r15
1409dd869: or al, r11b
1409dd86c: cmovne r14, rdi
1409dd870: sub rbx, r14
1409dd873: mov qword ptr [r10], rbx
1409dd876: mov r10, qword ptr [rbp + 0x130]
1409dd87d: mov rcx, qword ptr [rcx]
1409dd880: cmp byte ptr [rcx + 0x18], 1
1409dd884: jbe 0x1409dd89a
1409dd886: mov r11, qword ptr [rcx + 0x20]
1409dd88a: cmp byte ptr [rcx + 0x19], 2
1409dd88e: setb cl
1409dd891: and cl, byte ptr [r11 + 0xb9]
1409dd898: or al, cl
1409dd89a: mov rcx, qword ptr [r10]
1409dd89d: mov r10, qword ptr [r10 + 8]
1409dd8a1: mov r11, qword ptr [rcx]
1409dd8a4: xor edi, edi
1409dd8a6: mov rbx, r11
1409dd8a9: sub rbx, qword ptr [r10 + 0x18]
1409dd8ad: cmovb rbx, rdi
1409dd8b1: sub r11, qword ptr [r10 + 0x38]
1409dd8b5: cmovb r11, rdi
1409dd8b9: mov r14, qword ptr [r10 + 0x30]
1409dd8bd: sub r14, r11
1409dd8c0: cmovb r14, rdi
1409dd8c4: setbe r11b
1409dd8c8: mov r15, qword ptr [r10 + 0x10]
1409dd8cc: sub r15, rbx
1409dd8cf: cmovb r15, rdi
1409dd8d3: mov r10, qword ptr [r10 + 0x40]
1409dd8d7: setbe bl
1409dd8da: add r15, r14
1409dd8dd: mov rdi, r10
1409dd8e0: sub rdi, r15
1409dd8e3: mov qword ptr [rcx], rdi
1409dd8e6: test bl, r11b
1409dd8e9: je 0x1409dd8f7
1409dd8eb: mov rdi, qword ptr [rdx]
1409dd8ee: test al, al
1409dd8f0: jne 0x1409dd8fd
1409dd8f2: jmp 0x1409dd9dd
1409dd8f7: mov qword ptr [rcx], r10
1409dd8fa: mov rdi, qword ptr [rdx]
1409dd8fd: mov r14, r8
1409dd900: mov rbx, rdx
1409dd903: mov r15, qword ptr [rbp + 0x110]
1409dd90a: mov eax, dword ptr [r9 + 0x1c]
1409dd90e: mov rcx, qword ptr [r9 + 0x10]
1409dd912: mov dword ptr [rcx], eax
1409dd914: mov rdx, qword ptr [r9]
1409dd917: lea rcx, [rbp + 0x78]
1409dd91b: mov r8, rdi
1409dd91e: xor r9d, r9d
1409dd921: call 0x140b2cb30
1409dd926: mov eax, dword ptr [r15 + 0x1c]
1409dd92a: mov rcx, qword ptr [r15 + 0x10]
1409dd92e: mov dword ptr [rcx], eax
1409dd930: mov rdx, qword ptr [r15]
1409dd933: mov byte ptr [rbp + 0xaf], 1
1409dd93a: lea rcx, [rbp - 0x60]
1409dd93e: call 0x140900010
1409dd943: mov rax, qword ptr [rbp + 0x140]
1409dd94a: mov rcx, r14
1409dd94d: mov r14d, dword ptr [r14 + 0x1c]
1409dd951: mov r15, qword ptr [rcx + 0x10]
1409dd955: mov dword ptr [r15], r14d
1409dd958: movups xmm0, xmmword ptr [rbp + 0x78]
1409dd95c: movups xmm1, xmmword ptr [rbp + 0x88]
1409dd963: movups xmm2, xmmword ptr [rbp + 0x98]
1409dd96a: movaps xmmword ptr [rbp + 0x60], xmm2
1409dd96e: movaps xmmword ptr [rbp + 0x50], xmm1
1409dd972: movaps xmmword ptr [rbp + 0x40], xmm0
1409dd976: movups xmm0, xmmword ptr [rbp - 0x20]
1409dd97a: movaps xmmword ptr [rbp + 0x30], xmm0
1409dd97e: movups xmm0, xmmword ptr [rbp - 0x60]
1409dd982: movups xmm1, xmmword ptr [rbp - 0x50]
1409dd986: movups xmm2, xmmword ptr [rbp - 0x40]
1409dd98a: movups xmm3, xmmword ptr [rbp - 0x30]
1409dd98e: movaps xmmword ptr [rbp + 0x20], xmm3
1409dd992: movaps xmmword ptr [rbp + 0x10], xmm2
1409dd996: movaps xmmword ptr [rbp], xmm1
1409dd99a: movaps xmmword ptr [rbp - 0x10], xmm0
1409dd99e: mov ecx, dword ptr [rax + 0x1c]
1409dd9a1: mov rdx, qword ptr [rax + 0x10]
1409dd9a5: mov dword ptr [rdx], ecx
1409dd9a7: mov r9, qword ptr [rax]
1409dd9aa: mov byte ptr [rbp + 0xaf], 0
1409dd9b1: lea rdx, [rbp + 0x40]
1409dd9b5: lea r8, [rbp - 0x10]
1409dd9b9: mov rcx, rsi
1409dd9bc: call 0x140a854a0
1409dd9c1: mov rdx, qword ptr [rbp + 0x118]
1409dd9c8: mov dword ptr [r15], r14d
1409dd9cb: mov byte ptr [rbp + 0xaf], 0
1409dd9d2: mov rcx, rsi
1409dd9d5: call 0x140a86730
1409dd9da: mov rdx, rbx
1409dd9dd: mov eax, dword ptr [rdx + 0x1c]
1409dd9e0: mov rcx, qword ptr [rdx + 0x10]
1409dd9e4: mov dword ptr [rcx], eax
1409dd9e6: mov byte ptr [rdi + 0x1b0], 1
1409dd9ed: add rsp, 0x138
1409dd9f4: pop rbx
1409dd9f5: pop rdi
1409dd9f6: pop rsi
1409dd9f7: pop r14
1409dd9f9: pop r15
1409dd9fb: pop rbp
1409dd9fc: ret 
1409dd9fd: nop dword ptr [rax]
1409dda00: mov qword ptr [rsp + 0x10], rdx
1409dda05: push rbp
1409dda06: push r15
1409dda08: push r14
1409dda0a: push rsi
1409dda0b: push rdi
1409dda0c: push rbx
1409dda0d: sub rsp, 0x28
1409dda11: lea rbp, [rdx + 0x80]
1409dda18: cmp byte ptr [rbp + 0xaf], 0
1409dda1f: je 0x1409dda3d
1409dda21: lea rdx, [rbp + 0x98]
