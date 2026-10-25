140a82060: push r15
140a82062: push r14
140a82064: push r13
140a82066: push r12
140a82068: push rsi
140a82069: push rdi
140a8206a: push rbp
140a8206b: push rbx
140a8206c: sub rsp, 0x48
140a82070: movaps xmmword ptr [rsp + 0x30], xmm6
140a82075: mov rax, qword ptr [rcx]
140a82078: mov rbx, qword ptr [rax]
140a8207b: cmp byte ptr [rbx], 2
140a8207e: jne 0x140a8230b
140a82084: mov rsi, rcx
140a82087: mov rax, qword ptr [rcx + 8]
140a8208b: mov r15, qword ptr [rax]
140a8208e: mov r14, qword ptr [rbx + 0x10]
140a82092: mov qword ptr [rsp + 0x28], r14
140a82097: cmp qword ptr [r15 + 0x18], 0
140a8209c: je 0x140a821bc
140a820a2: mov rdi, rdx
140a820a5: lea rcx, [r15 + 0x20]
140a820a9: lea rdx, [rsp + 0x28]
140a820ae: call 0x1405b0d70
140a820b3: mov rcx, qword ptr [r15]
140a820b6: mov rdx, qword ptr [r15 + 8]
140a820ba: mov r8, rdx
140a820bd: and r8, rax
140a820c0: shr rax, 0x39
140a820c4: movd xmm0, eax
140a820c8: punpcklbw xmm0, xmm0
140a820cc: pshuflw xmm0, xmm0, 0
140a820d1: pshufd xmm0, xmm0, 0
140a820d6: lea rax, [rcx - 0x78]
140a820da: xor r9d, r9d
140a820dd: pcmpeqd xmm1, xmm1
140a820e1: movdqu xmm2, xmmword ptr [rcx + r8]
140a820e7: movdqa xmm3, xmm2
140a820eb: pcmpeqb xmm3, xmm0
140a820ef: pmovmskb r10d, xmm3
140a820f4: test r10d, r10d
140a820f7: je 0x140a8211e
140a820f9: tzcnt r11d, r10d
140a820fe: add r11, r8
140a82101: and r11, rdx
140a82104: neg r11
140a82107: imul r11, r11, 0x78
140a8210b: cmp qword ptr [rax + r11], r14
140a8210f: je 0x140a82140
140a82111: lea r11d, [r10 - 1]
140a82115: and r11w, r10w
140a82119: mov r10d, r11d
140a8211c: jne 0x140a820f9
140a8211e: pcmpeqb xmm2, xmm1
140a82122: pmovmskb r10d, xmm2
140a82127: test r10d, r10d
140a8212a: jne 0x140a821bc
140a82130: add r8, r9
140a82133: add r8, 0x10
140a82137: add r9, 0x10
140a8213b: and r8, rdx
140a8213e: jmp 0x140a820e1
140a82140: movzx eax, byte ptr [rdi + 0x9c]
140a82147: cmp al, byte ptr [rcx + r11 - 0xb]
140a8214c: jne 0x140a8230b
140a82152: mov r14, qword ptr [rdi + 0x80]
140a82159: mov r15, qword ptr [rbx + 0x10]
140a8215d: cmp r14, r15
140a82160: jne 0x140a821c8
140a82162: mov rdi, qword ptr [rsi + 0x20]
140a82166: mov r15, qword ptr [rdi + 0x10]
140a8216a: cmp r15, qword ptr [rdi]
140a8216d: jne 0x140a8217e
140a8216f: lea rdx, [rip + 0x205554a]
140a82176: mov rcx, rdi
140a82179: call 0x140562870
140a8217e: mov rax, qword ptr [rdi + 8]
140a82182: mov qword ptr [rax + r15*8], r14
140a82186: inc r15
140a82189: mov qword ptr [rdi + 0x10], r15
140a8218d: mov rax, qword ptr [rsi + 0x18]
140a82191: mov rsi, qword ptr [rbx + 0x10]
140a82195: movzx ebx, byte ptr [rbx + 0x18]
140a82199: mov ecx, dword ptr [rax + 0x1c]
140a8219c: mov rdx, qword ptr [rax + 0x10]
140a821a0: mov dword ptr [rdx], ecx
140a821a2: mov rdi, qword ptr [rax]
140a821a5: mov r14, qword ptr [rdi + 0x30]
140a821a9: mov r15, qword ptr [rdi + 0x40]
140a821ad: cmp r14, qword ptr [rdi + 0x20]
140a821b1: je 0x140a822db
140a821b7: jmp 0x140a822eb
140a821bc: lea rcx, [rip + 0x20554e5]
140a821c3: call 0x1428d9310
140a821c8: mov rax, qword ptr [rsi + 0x10]
140a821cc: movzx r8d, byte ptr [rdi + 0x9e]
140a821d4: mov r10d, dword ptr [rdi + 0x98]
140a821db: movzx r13d, byte ptr [rbx + 0x18]
140a821e0: movss xmm6, dword ptr [rdi + 0x94]
140a821e8: mov ecx, dword ptr [rax + 0x1c]
140a821eb: mov rdx, qword ptr [rax + 0x10]
140a821ef: mov dword ptr [rdx], ecx
140a821f1: mov rdi, qword ptr [rax]
140a821f4: mov rbp, qword ptr [rdi + 0x30]
140a821f8: mov r12, qword ptr [rdi + 0x40]
140a821fc: cmp rbp, qword ptr [rdi + 0x20]
140a82200: jne 0x140a82227
140a82202: lea rcx, [rdi + 0x20]
140a82206: lea rdx, [rip + 0x21b7f3b]
140a8220d: mov byte ptr [rsp + 0x23], r8b
140a82212: mov dword ptr [rsp + 0x24], r10d
140a82217: call 0x140562ab0
140a8221c: mov r10d, dword ptr [rsp + 0x24]
140a82221: movzx r8d, byte ptr [rsp + 0x23]
140a82227: mov rax, qword ptr [rdi + 0x28]
140a8222b: lea rcx, [rbp*4]
140a82233: add rcx, rbp
140a82236: mov qword ptr [rax + rcx*8], r12
140a8223a: mov qword ptr [rax + rcx*8 + 8], r14
140a8223f: mov qword ptr [rax + rcx*8 + 0x10], r15
140a82244: movss dword ptr [rax + rcx*8 + 0x18], xmm6
140a8224a: mov dword ptr [rax + rcx*8 + 0x1c], r10d
140a8224f: mov byte ptr [rax + rcx*8 + 0x20], r8b
140a82254: mov byte ptr [rax + rcx*8 + 0x21], r13b
140a82259: inc rbp
140a8225c: mov qword ptr [rdi + 0x30], rbp
140a82260: inc qword ptr [rdi + 0x40]
140a82264: mov rdi, qword ptr [rsi + 0x18]
140a82268: mov eax, dword ptr [rdi + 0x1c]
140a8226b: mov rcx, qword ptr [rdi + 0x10]
140a8226f: mov dword ptr [rcx], eax
140a82271: mov rsi, qword ptr [rdi]
140a82274: mov r15, qword ptr [rsi + 0x30]
140a82278: mov r12, qword ptr [rsi + 0x40]
140a8227c: cmp r15, qword ptr [rsi + 0x20]
140a82280: jne 0x140a82298
140a82282: lea rcx, [rsi + 0x20]
140a82286: lea rdx, [rip + 0x21b7ebb]
140a8228d: mov ebp, r8d
140a82290: call 0x140562db0
140a82295: mov r8d, ebp
140a82298: mov rax, qword ptr [rsi + 0x28]
140a8229c: lea rcx, [r15 + r15*2]
140a822a0: mov qword ptr [rax + rcx*8], r12
140a822a4: mov qword ptr [rax + rcx*8 + 8], r14
140a822a9: mov byte ptr [rax + rcx*8 + 0x10], r8b
140a822ae: inc r15
140a822b1: mov qword ptr [rsi + 0x30], r15
140a822b5: inc qword ptr [rsi + 0x40]
140a822b9: mov rsi, qword ptr [rbx + 0x10]
140a822bd: movzx ebx, byte ptr [rbx + 0x18]
140a822c1: mov eax, dword ptr [rdi + 0x1c]
140a822c4: mov rcx, qword ptr [rdi + 0x10]
140a822c8: mov dword ptr [rcx], eax
140a822ca: mov rdi, qword ptr [rdi]
140a822cd: mov r14, qword ptr [rdi + 0x30]
140a822d1: mov r15, qword ptr [rdi + 0x40]
140a822d5: cmp r14, qword ptr [rdi + 0x20]
140a822d9: jne 0x140a822eb
140a822db: lea rcx, [rdi + 0x20]
140a822df: lea rdx, [rip + 0x21b7e62]
140a822e6: call 0x140562db0
140a822eb: mov rax, qword ptr [rdi + 0x28]
140a822ef: lea rcx, [r14 + r14*2]
140a822f3: mov qword ptr [rax + rcx*8], r15
140a822f7: mov qword ptr [rax + rcx*8 + 8], rsi
140a822fc: mov byte ptr [rax + rcx*8 + 0x10], bl
140a82300: inc r14
140a82303: mov qword ptr [rdi + 0x30], r14
140a82307: inc qword ptr [rdi + 0x40]
140a8230b: movaps xmm6, xmmword ptr [rsp + 0x30]
140a82310: add rsp, 0x48
140a82314: pop rbx
140a82315: pop rbp
140a82316: pop rdi
140a82317: pop rsi
140a82318: pop r12
140a8231a: pop r13
140a8231c: pop r14
140a8231e: pop r15
140a82320: ret 
140a82321: int3 
140a82322: int3 
140a82323: int3 
140a82324: int3 
140a82325: int3 
140a82326: int3 
140a82327: int3 
140a82328: int3 
140a82329: int3 
140a8232a: int3 
140a8232b: int3 
140a8232c: int3 
140a8232d: int3 
140a8232e: int3 
140a8232f: int3 
140a82330: sub rsp, 0x38
140a82334: mov rax, rdx
140a82337: cmp dword ptr [rcx], 0xb
140a8233a: jne 0x140a82355
140a8233c: lea rdx, [rip + 0x2055395]
140a82343: mov r8d, 0xb
140a82349: mov rcx, rax
140a8234c: add rsp, 0x38
140a82350: jmp 0x140d86b90
140a82355: mov qword ptr [rsp + 0x30], rcx
140a8235a: lea rcx, [rip + 0x2055387]
140a82361: mov qword ptr [rsp + 0x20], rcx
140a82366: lea rdx, [rip + 0x205539b]
140a8236d: lea r9, [rsp + 0x30]
140a82372: mov r8d, 0xc
140a82378: mov rcx, rax
140a8237b: call 0x140d872d0
140a82380: nop 
140a82381: add rsp, 0x38
140a82385: ret 
140a82386: int3 
140a82387: int3 
140a82388: int3 
140a82389: int3 
140a8238a: int3 
140a8238b: int3 
140a8238c: int3 
140a8238d: int3 
140a8238e: int3 
140a8238f: int3 
