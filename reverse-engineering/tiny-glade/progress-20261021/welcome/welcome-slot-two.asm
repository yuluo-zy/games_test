0x140fc9540: push rbp
0x140fc9541: push r14
0x140fc9543: push rsi
0x140fc9544: push rdi
0x140fc9545: push rbx
0x140fc9546: sub rsp, 0xc0
0x140fc954d: lea rbp, [rsp + 0x80]
0x140fc9555: movaps xmmword ptr [rbp + 0x30], xmm6
0x140fc9559: mov qword ptr [rbp + 0x28], 0xfffffffffffffffe
0x140fc9561: mov rdi, rdx
0x140fc9564: mov rsi, rcx
0x140fc9567: lea rcx, [rip + 0x1bea93f]
0x140fc956e: mov edx, 0xe
0x140fc9573: call 0x140b9d480
0x140fc9578: mov qword ptr [rbp - 0x30], rax
0x140fc957c: mov qword ptr [rbp - 0x28], rdx
0x140fc9580: lea rax, [rbp - 0x30]
0x140fc9584: mov qword ptr [rbp - 0x58], rax
0x140fc9588: lea rax, [rip - 0xebf6ff]
0x140fc958f: mov qword ptr [rbp - 0x50], rax
0x140fc9593: lea rax, [rip + 0x1bea926]
0x140fc959a: mov qword ptr [rbp - 0x10], rax
0x140fc959e: mov qword ptr [rbp - 8], 2
0x140fc95a6: lea rax, [rbp - 0x58]
0x140fc95aa: mov qword ptr [rbp], rax
0x140fc95ae: mov qword ptr [rbp + 8], 1
0x140fc95b6: mov qword ptr [rbp + 0x10], 0
0x140fc95be: lea rcx, [rbp - 0x48]
0x140fc95c2: lea rdx, [rbp - 0x10]
0x140fc95c6: call 0x140d79f90
0x140fc95cb: mov r8, qword ptr [rbp - 0x40]
0x140fc95cf: mov r9, qword ptr [rbp - 0x38]
0x140fc95d3: lea rcx, [rbp - 0x10]
0x140fc95d7: mov rdx, rdi
0x140fc95da: mov qword ptr [rbp + 0x20], r8
0x140fc95de: call 0x14068bfc0
0x140fc95e3: mov rax, qword ptr [rsi]
0x140fc95e6: movss xmm6, dword ptr [rax]
0x140fc95ea: lea rcx, [rbp - 0x10]
0x140fc95ee: mov dl, 1
0x140fc95f0: call 0x140f9dba0
0x140fc95f5: mulss xmm6, dword ptr [rip + 0x195c2d3]
0x140fc95fd: mov rcx, rax
0x140fc9600: movaps xmm1, xmm6
0x140fc9603: call 0x140fac150
0x140fc9608: mov rax, qword ptr [rbp]
0x140fc960c: mov qword ptr [rbp - 0x20], rax
0x140fc9610: movdqu xmm0, xmmword ptr [rbp - 0x10]
0x140fc9615: movdqa xmmword ptr [rbp - 0x30], xmm0
0x140fc961a: lea rcx, [rbp - 0x10]
0x140fc961e: lea rdx, [rbp - 0x30]
0x140fc9622: mov r8b, 0x13
0x140fc9625: call 0x14067e6d0
0x140fc962a: mov rdi, qword ptr [rbp - 0x10]
0x140fc962e: cmp qword ptr [rdi + 0x1b8], 0
0x140fc9636: je 0x140fc9794
0x140fc963c: lea rdx, [rbp - 8]
0x140fc9640: lea rcx, [rdi + 0x1c0]
0x140fc9647: call 0x1405b0d70
0x140fc964c: mov rcx, qword ptr [rdi + 0x1a0]
0x140fc9653: mov rdx, qword ptr [rdi + 0x1a8]
0x140fc965a: mov r8, rdx
0x140fc965d: and r8, rax
0x140fc9660: shr rax, 0x39
0x140fc9664: movd xmm0, eax
0x140fc9668: punpcklbw xmm0, xmm0
0x140fc966c: pshuflw xmm0, xmm0, 0
0x140fc9671: pshufd xmm0, xmm0, 0
0x140fc9676: lea rax, [rcx - 0xc0]
0x140fc967d: mov r9, qword ptr [rbp - 8]
0x140fc9681: xor r10d, r10d
0x140fc9684: pcmpeqd xmm1, xmm1
0x140fc9688: movdqu xmm2, xmmword ptr [rcx + r8]
0x140fc968e: movdqa xmm3, xmm2
0x140fc9692: pcmpeqb xmm3, xmm0
0x140fc9696: pmovmskb r11d, xmm3
0x140fc969b: test r11d, r11d
0x140fc969e: je 0x140fc96d0
0x140fc96a0: tzcnt edi, r11d
0x140fc96a5: add rdi, r8
0x140fc96a8: and rdi, rdx
0x140fc96ab: neg rdi
0x140fc96ae: lea rdi, [rdi + rdi*2]
0x140fc96b2: shl rdi, 6
0x140fc96b6: cmp r9, qword ptr [rax + rdi]
0x140fc96ba: je 0x140fc96f2
0x140fc96bc: lea edi, [r11 - 1]
0x140fc96c0: and di, r11w
0x140fc96c4: mov r11d, edi
0x140fc96c7: jne 0x140fc96a0
0x140fc96c9: nop dword ptr [rax]
0x140fc96d0: pcmpeqb xmm2, xmm1
0x140fc96d4: pmovmskb r11d, xmm2
0x140fc96d9: test r11d, r11d
0x140fc96dc: jne 0x140fc9794
0x140fc96e2: add r8, r10
0x140fc96e5: add r8, 0x10
0x140fc96e9: add r10, 0x10
0x140fc96ed: and r8, rdx
0x140fc96f0: jmp 0x140fc9688
0x140fc96f2: movzx ebx, byte ptr [rcx + rdi - 2]
0x140fc96f7: mov rdx, qword ptr [rbp - 0x48]
0x140fc96fb: test rdx, rdx
0x140fc96fe: je 0x140fc970f
0x140fc9700: mov r8d, 1
0x140fc9706: mov rcx, qword ptr [rbp + 0x20]
0x140fc970a: call 0x140613c20
0x140fc970f: test bl, 0x30
0x140fc9712: je 0x140fc9782
0x140fc9714: mov rax, qword ptr [rsi + 8]
0x140fc9718: mov ecx, dword ptr [rax + 0x1c]
0x140fc971b: mov rdx, qword ptr [rax + 0x10]
0x140fc971f: mov dword ptr [rdx], ecx
0x140fc9721: mov rax, qword ptr [rax]
0x140fc9724: mov byte ptr [rax + 0x206], 1
0x140fc972b: mov rax, qword ptr [rsi + 0x10]
0x140fc972f: mov ecx, dword ptr [rax + 0x1c]
0x140fc9732: mov rdx, qword ptr [rax + 0x10]
0x140fc9736: mov dword ptr [rdx], ecx
0x140fc9738: mov rdi, qword ptr [rax]
0x140fc973b: mov rbx, qword ptr [rdi + 0x30]
0x140fc973f: mov r14, qword ptr [rdi + 0x40]
0x140fc9743: cmp rbx, qword ptr [rdi + 0x20]
0x140fc9747: jne 0x140fc9759
0x140fc9749: lea rcx, [rdi + 0x20]
0x140fc974d: lea rdx, [rip + 0x1bebb1c]
0x140fc9754: call 0x140562870
0x140fc9759: mov rax, qword ptr [rdi + 0x28]
0x140fc975d: mov qword ptr [rax + rbx*8], r14
0x140fc9761: inc rbx
0x140fc9764: mov qword ptr [rdi + 0x30], rbx
0x140fc9768: inc qword ptr [rdi + 0x40]
0x140fc976c: mov rax, qword ptr [rsi + 0x18]
0x140fc9770: mov ecx, dword ptr [rax + 0x1c]
0x140fc9773: mov rdx, qword ptr [rax + 0x10]
0x140fc9777: mov dword ptr [rdx], ecx
0x140fc9779: mov rax, qword ptr [rax]
0x140fc977c: mov dword ptr [rax], 2
0x140fc9782: movaps xmm6, xmmword ptr [rbp + 0x30]
0x140fc9786: add rsp, 0xc0
0x140fc978d: pop rbx
0x140fc978e: pop rdi
0x140fc978f: pop rsi
0x140fc9790: pop r14
0x140fc9792: pop rbp
0x140fc9793: ret 
0x140fc9794: lea rcx, [rip + 0x1a4da6d]
0x140fc979b: lea r8, [rip + 0x1a4e836]
0x140fc97a2: mov edx, 0x16
0x140fc97a7: call 0x1428d9330
0x140fc97ac: ud2 
0x140fc97ae: nop 
0x140fc97b0: mov qword ptr [rsp + 0x10], rdx
0x140fc97b5: push rbp
0x140fc97b6: push r14
0x140fc97b8: push rsi
0x140fc97b9: push rdi
0x140fc97ba: push rbx
0x140fc97bb: sub rsp, 0x30
0x140fc97bf: lea rbp, [rdx + 0x80]
0x140fc97c6: movaps xmmword ptr [rsp + 0x20], xmm6
0x140fc97cb: mov rdx, qword ptr [rbp - 0x48]
0x140fc97cf: test rdx, rdx
0x140fc97d2: je 0x140fc97e3
0x140fc97d4: mov r8d, 1
0x140fc97da: mov rcx, qword ptr [rbp + 0x20]
0x140fc97de: call 0x140613c20
0x140fc97e3: movaps xmm6, xmmword ptr [rsp + 0x20]
0x140fc97e8: add rsp, 0x30
0x140fc97ec: pop rbx
0x140fc97ed: pop rdi
0x140fc97ee: pop rsi
0x140fc97ef: pop r14
0x140fc97f1: pop rbp
0x140fc97f2: ret 
