142268201: mov rax, qword ptr [rbp + 0x10]
142268205: mov r15, qword ptr [rax]
142268208: mov rax, qword ptr [rax + 8]
14226820c: mov r8, qword ptr [r15]
14226820f: xor edx, edx
142268211: mov r9, r8
142268214: sub r9, qword ptr [rax + 0x18]
142268218: cmovb r9, rdx
14226821c: sub r8, qword ptr [rax + 0x38]
142268220: cmovb r8, rdx
142268224: mov rcx, r9
142268227: shl rcx, 4
14226822b: add rcx, qword ptr [rax + 8]
14226822f: mov r14, qword ptr [rax + 0x10]
142268233: sub r14, r9
142268236: cmovb r14, rdx
14226823a: mov r9d, 8
142268240: cmovb rcx, r9
142268244: mov rdi, r8
142268247: shl rdi, 4
14226824b: add rdi, qword ptr [rax + 0x28]
14226824f: mov r10, qword ptr [rax + 0x30]
142268253: sub r10, r8
142268256: cmovb r10, rdx
14226825a: cmovb rdi, r9
14226825e: mov rax, qword ptr [rax + 0x40]
142268262: sub rax, r10
142268265: sub rax, r14
142268268: mov qword ptr [r15], rax
14226826b: shl r14, 4
14226826f: add r14, rcx
142268272: shl r10, 4
142268276: add r10, rdi
142268279: mov qword ptr [rbp + 0x110], r10
142268280: movss xmm6, dword ptr [rip + 0x6bd5e8]
142268288: movss xmm7, dword ptr [rip + 0x8145fc]
142268290: mov qword ptr [rbp + 0x100], r15
142268297: jmp 0x1422682e0
142268299: nop dword ptr [rax]
1422682a0: mov rdx, qword ptr [rbp + 0x108]
1422682a7: mov rax, qword ptr [rdx + 0x28]
1422682ab: mov rcx, rdi
1422682ae: shl rcx, 5
1422682b2: mov qword ptr [rax + rcx], rbx
1422682b6: mov byte ptr [rax + rcx + 8], 2
1422682bb: movss dword ptr [rax + rcx + 0xc], xmm8
1422682c2: movlps qword ptr [rax + rcx + 0x10], xmm9
1422682c8: movss dword ptr [rax + rcx + 0x18], xmm10
1422682cf: inc rdi
1422682d2: mov qword ptr [rdx + 0x30], rdi
1422682d6: inc qword ptr [rdx + 0x40]
1422682da: mov rcx, r13
1422682dd: mov rdi, rsi
1422682e0: test rcx, rcx
1422682e3: je 0x142268300
1422682e5: cmp rcx, r14
1422682e8: je 0x142268300
1422682ea: lea r13, [rcx + 0x10]
1422682ee: mov rsi, rdi
1422682f1: jmp 0x142268323
1422682f3: nop word ptr cs:[rax + rax]
142268300: test rdi, rdi
142268303: je 0x1422684fe
142268309: cmp rdi, qword ptr [rbp + 0x110]
142268310: je 0x1422684fe
142268316: lea rsi, [rdi + 0x10]
14226831a: xor r13d, r13d
14226831d: mov rcx, rdi
142268320: mov rdi, rsi
142268323: inc qword ptr [r15]
142268326: mov eax, dword ptr [rbp + 0x11c]
14226832c: mov rdx, qword ptr [rbp + 0x50]
142268330: mov dword ptr [rdx], eax
142268332: mov rdx, qword ptr [rcx + 8]
142268336: mov rcx, qword ptr [rbp + 0x48]
14226833a: call 0x140a385a0
14226833f: mov rbx, rax
142268342: shr rbx, 0x20
142268346: mov rcx, r13
142268349: je 0x1422682e0
14226834b: mov r12, rax
14226834e: mov rax, qword ptr [rbp + 0x250]
142268355: mov rcx, qword ptr [rax]
142268358: mov rdx, r12
14226835b: call 0x140ceafa0
142268360: mov rcx, r13
142268363: mov rdi, rsi
142268366: test al, al
142268368: je 0x1422682e0
14226836e: mov r15, r14
142268371: mov rax, qword ptr [rbp + 0x250]
142268378: mov r14, qword ptr [rax + 8]
14226837c: test r14, r14
14226837f: jne 0x142268385
142268381: mov r14, qword ptr [rax + 0x10]
142268385: mov rax, qword ptr [r14]
142268388: mov rdi, qword ptr [r14 + 0x10]
14226838c: sub rax, rdi
14226838f: cmp rax, 0x1f
142268393: lea rcx, [rip + 0x375f16]
14226839a: jbe 0x1422684d2
1422683a0: mov rax, qword ptr [r14 + 8]
1422683a4: lea rdx, [rip + 0x3763e5]
1422683ab: mov qword ptr [rax + rdi], rdx
1422683af: mov qword ptr [rax + rdi + 8], rcx
1422683b4: mov qword ptr [rax + rdi + 0x10], r12
1422683b9: mov byte ptr [rax + rdi + 0x18], 0
1422683be: add rdi, 0x20
1422683c2: mov qword ptr [r14 + 0x10], rdi
1422683c6: mov ecx, r12d
1422683c9: mov r11, qword ptr [rbp + 0x70]
1422683cd: cmp rcx, qword ptr [r11 + 0x10]
1422683d1: jae 0x142268560
1422683d7: mov rax, qword ptr [r11 + 8]
1422683db: mov rbx, r12
1422683de: shr rbx, 0x20
1422683e2: lea rdx, [rcx + rcx*4]
1422683e6: cmp dword ptr [rax + rdx*4], ebx
1422683e9: mov r10, qword ptr [rbp + 0x30]
1422683ed: jne 0x142268560
1422683f3: mov r14, r15
1422683f6: lea rdx, [rax + rdx*4]
1422683fa: mov eax, dword ptr [rdx + 4]
1422683fd: mov r8d, 0xffffffff
142268403: cmp rax, r8
142268406: mov r15, qword ptr [rbp + 0x100]
14226840d: je 0x142268560
142268413: xor ecx, ecx
142268415: cmp qword ptr [r10 + 0x38], rax
142268419: jbe 0x14226856f
14226841f: mov r8, qword ptr [r10 + 0x28]
142268423: mov r9d, eax
142268426: shr r9d, 6
14226842a: mov r8, qword ptr [r8 + r9*8]
14226842e: bt r8, rax
142268432: jae 0x14226856f
142268438: mov eax, dword ptr [rdx + 0xc]
14226843b: mov ecx, dword ptr [rdx + 0x10]
14226843e: mov rdx, qword ptr [r11 + 0x1a8]
142268445: lea rax, [rax + rax*8]
142268449: mov r8, qword ptr [r10 + 0x110]
142268450: mov r9, qword ptr [rdx + rax*8 + 0x18]
142268455: mov rax, qword ptr [rdx + rax*8 + 0x38]
14226845a: mov rax, qword ptr [rax + r8*8]
14226845e: not rax
142268461: lea rax, [rax + rax*2]
142268465: shl rax, 4
142268469: mov rdi, qword ptr [r9 + rax + 0x10]
14226846e: lea rbx, [rcx + rcx*2]
142268472: shl rbx, 4
142268476: movss xmm2, dword ptr [rdi + rbx + 0xc]
14226847c: movaps xmm0, xmm6
14226847f: movaps xmm1, xmm7
142268482: call 0x140c904a0
142268487: movaps xmm8, xmm0
14226848b: movsd xmm9, qword ptr [rdi + rbx]
142268491: movss xmm10, dword ptr [rdi + rbx + 8]
142268498: mov eax, dword ptr [rbp + 0x120]
14226849e: mov rcx, qword ptr [rbp + 0x60]
1422684a2: mov dword ptr [rcx], eax
1422684a4: mov rax, qword ptr [rbp + 0x108]
1422684ab: mov rdi, qword ptr [rax + 0x30]
1422684af: mov rbx, qword ptr [rax + 0x40]
1422684b3: cmp rdi, qword ptr [rax + 0x20]
1422684b7: jne 0x1422682a0
1422684bd: mov rcx, qword ptr [rbp + 0x68]
1422684c1: lea rdx, [rip + 0xdcc848]
1422684c8: call 0x140562c30
1422684cd: jmp 0x1422682a0
1422684d2: mov qword ptr [rsp + 0x20], 1
1422684db: mov r8d, 0x20
1422684e1: mov r9d, 1
1422684e7: mov rcx, r14
1422684ea: mov rdx, rdi
1422684ed: call 0x1429105f0
1422684f2: lea rcx, [rip + 0x375db7]
1422684f9: jmp 0x1422683a0
1422684fe: movaps xmm6, xmmword ptr [rbp + 0x130]
142268505: movaps xmm7, xmmword ptr [rbp + 0x140]
14226850c: movaps xmm8, xmmword ptr [rbp + 0x150]
142268514: movaps xmm9, xmmword ptr [rbp + 0x160]
14226851c: movaps xmm10, xmmword ptr [rbp + 0x170]
142268524: movaps xmm11, xmmword ptr [rbp + 0x180]
14226852c: movaps xmm12, xmmword ptr [rbp + 0x190]
142268534: movaps xmm13, xmmword ptr [rbp + 0x1a0]
14226853c: movaps xmm14, xmmword ptr [rbp + 0x1b0]
142268544: movaps xmm15, xmmword ptr [rbp + 0x1c0]
14226854c: add rsp, 0x258
142268553: pop rbx
142268554: pop rdi
142268555: pop rsi
142268556: pop r12
142268558: pop r13
14226855a: pop r14
14226855c: pop r15
14226855e: pop rbp
14226855f: ret 
142268560: shl rbx, 0x20
142268564: or rbx, rcx
142268567: mov ecx, 1
14226856c: mov r12, rbx
14226856f: mov dword ptr [rbp + 0x78], ecx
142268572: mov dword ptr [rbp + 0x7c], eax
142268575: mov qword ptr [rbp + 0x80], r12
14226857c: lea rax, [rip + 0xdcc98d]
142268583: mov qword ptr [rsp + 0x20], rax
142268588: lea rcx, [rip + 0xdcc821]
