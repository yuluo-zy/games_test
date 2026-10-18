0x141e45f30 push rbp
0x141e45f31 push r15
0x141e45f33 push r14
0x141e45f35 push r13
0x141e45f37 push r12
0x141e45f39 push rsi
0x141e45f3a push rdi
0x141e45f3b push rbx
0x141e45f3c sub rsp, 0x348
0x141e45f43 lea rbp, [rsp + 0x80]
0x141e45f4b movdqa xmmword ptr [rbp + 0x2b0], xmm15
0x141e45f54 movdqa xmmword ptr [rbp + 0x2a0], xmm14
0x141e45f5d movdqa xmmword ptr [rbp + 0x290], xmm13
0x141e45f66 movdqa xmmword ptr [rbp + 0x280], xmm12
0x141e45f6f movdqa xmmword ptr [rbp + 0x270], xmm11
0x141e45f78 movdqa xmmword ptr [rbp + 0x260], xmm10
0x141e45f81 movaps xmmword ptr [rbp + 0x250], xmm9
0x141e45f89 movaps xmmword ptr [rbp + 0x240], xmm8
0x141e45f91 movdqa xmmword ptr [rbp + 0x230], xmm7
0x141e45f99 movaps xmmword ptr [rbp + 0x220], xmm6
0x141e45fa0 mov qword ptr [rbp + 0x218], 0xfffffffffffffffe
0x141e45fab mov rdi, r9
0x141e45fae mov rbx, r8
0x141e45fb1 mov r15, rdx
0x141e45fb4 mov rsi, rcx
0x141e45fb7 cmp byte ptr [rdx + 0x50], 0
0x141e45fbb jne 0x141e45fe2
0x141e45fbd lea rdx, [r15 + 0x10]
0x141e45fc1 lea rcx, [rbp + 0x1c8]
0x141e45fc8 mov r8, rbx
0x141e45fcb mov r9, rdi
0x141e45fce call 0x140bc8c90
0x141e45fd3 movzx eax, byte ptr [rbp + 0x1c8]
0x141e45fda cmp al, 0x32
0x141e45fdc jne 0x141e46f8d
0x141e45fe2 lea r14, [r15 + 0x10]
0x141e45fe6 lea rcx, [rbp + 0x1c8]
0x141e45fed mov rdx, r14
0x141e45ff0 call 0x140bca140
0x141e45ff5 cmp byte ptr [rbp + 0x1c8], 0x32
0x141e45ffc jne 0x141e46f50
0x141e46002 movzx ecx, byte ptr [r15 + 0x50]
0x141e46007 test cl, cl
0x141e46009 jne 0x141e46025
0x141e4600b mov rcx, r14
0x141e4600e mov edx, 0x28
0x141e46013 call 0x140bc41b0
0x141e46018 test al, al
0x141e4601a je 0x141e46fd7
0x141e46020 movzx ecx, byte ptr [r15 + 0x50]
0x141e46025 mov byte ptr [r15 + 0x50], 0
0x141e4602a test byte ptr [r15], 1
0x141e4602e je 0x141e46044
0x141e46030 mov rax, qword ptr [r15 + 8]
0x141e46034 test rax, rax
0x141e46037 je 0x141e46ff0
0x141e4603d dec rax
0x141e46040 mov qword ptr [r15 + 8], rax
0x141e46044 mov qword ptr [rbp + 0x68], r14
0x141e46048 mov qword ptr [rbp + 0x18], rbx
0x141e4604c mov qword ptr [rbp + 0x198], rdi
0x141e46053 mov qword ptr [rbp + 0x38], r15
0x141e46057 mov qword ptr [rbp + 0x168], r15
0x141e4605e mov word ptr [rbp + 0x170], 0x103
0x141e46067 mov byte ptr [rbp + 0x1c3], 6
0x141e4606e mov byte ptr [rbp + 0x1c2], 5
0x141e46075 mov dword ptr [rbp + 0x184], 2
0x141e4607f mov byte ptr [rbp + 0x1ae], 0xa
0x141e46086 movabs rdi, 0x8000000000000000
0x141e46090 movdqa xmm11, xmmword ptr [rip + 0xb84297]
0x141e46099 mov dword ptr [rbp + 0x18c], 2
0x141e460a3 mov dword ptr [rbp + 0x190], 2
0x141e460ad mov byte ptr [rbp + 0x217], cl
0x141e460b3 mov qword ptr [rbp + 0x1b8], rdi
0x141e460ba mov qword ptr [rbp + 0x1b0], rdi
0x141e460c1 lea rcx, [rbp + 0x1c8]
0x141e460c8 lea rdx, [rbp + 0x168]
0x141e460cf call 0x140bcd950
0x141e460d4 movzx r14d, byte ptr [rbp + 0x1c8]
0x141e460dc movzx edi, byte ptr [rbp + 0x1c9]
0x141e460e3 cmp r14b, 0x32
0x141e460e7 jne 0x141e47084
0x141e460ed test dil, 1
0x141e460f1 je 0x141e47101
0x141e460f7 movzx ecx, byte ptr [rbp + 0x170]
0x141e460fe mov rdi, qword ptr [rbp + 0x168]
0x141e46105 movzx eax, byte ptr [rdi]
0x141e46108 cmp ecx, 1
0x141e4610b mov r8, qword ptr [rbp + 0x1b8]
0x141e46112 je 0x141e462c0
0x141e46118 cmp ecx, 3
0x141e4611b jne 0x141e46460
0x141e46121 test al, 1
0x141e46123 je 0x141e46139
0x141e46125 mov rax, qword ptr [rdi + 8]
0x141e46129 test rax, rax
0x141e4612c je 0x141e4737d
0x141e46132 dec rax
0x141e46135 mov qword ptr [rdi + 8], rax
0x141e46139 mov qword ptr [rbp + 0xc8], rdi
0x141e46140 mov byte ptr [rbp + 0xd0], 0
0x141e46147 mov qword ptr [rbp + 0x1b0], r8
0x141e4614e lea rcx, [rbp + 0x1c8]
0x141e46155 lea rdx, [rbp + 0xc8]
0x141e4615c call 0x142373e70
0x141e46161 movzx r14d, byte ptr [rbp + 0x1c8]
0x141e46169 cmp r14b, 0x32
0x141e4616d movzx r11d, byte ptr [rbp + 0x195]
0x141e46175 movzx r8d, word ptr [rbp + 0x17a]
0x141e4617d mov r9d, dword ptr [rbp + 0x84]
0x141e46184 mov r10d, dword ptr [rbp + 0x88]
0x141e4618b je 0x141e46222
0x141e46191 movzx r8d, word ptr [rbp + 0x1ca]
0x141e46199 mov eax, dword ptr [rbp + 0x1cc]
0x141e4619f mov dword ptr [rbp + 0xa4], eax
0x141e461a5 mov rax, qword ptr [rbp + 0x1d0]
0x141e461ac mov qword ptr [rbp + 0x20], rax
0x141e461b0 movups xmm0, xmmword ptr [rbp + 0x1d8]
0x141e461b7 movaps xmmword ptr [rbp - 0x30], xmm0
0x141e461bb movsd xmm0, qword ptr [rbp + 0x1e8]
0x141e461c3 movaps xmmword ptr [rbp - 0x50], xmm0
0x141e461c7 mov r9d, dword ptr [rbp + 0x1f0]
0x141e461ce movss xmm0, dword ptr [rbp + 0x1f4]
0x141e461d6 movss dword ptr [rbp + 0x94], xmm0
0x141e461de mov r10d, dword ptr [rbp + 0x1f8]
0x141e461e5 movd xmm0, dword ptr [rbp + 0x1fc]
0x141e461ed movd dword ptr [rbp + 0x98], xmm0
0x141e461f5 movq xmm7, qword ptr [rbp + 0x200]
0x141e461fd movzx r11d, byte ptr [rbp + 0x202]
0x141e46205 lea rcx, [rbp + 0x1c9]
0x141e4620c mov rax, qword ptr [rcx + 0x3a]
0x141e46210 mov rcx, qword ptr [rcx + 0x3f]
0x141e46214 mov qword ptr [rbp + 0x1a0], rax
0x141e4621b mov qword ptr [rbp + 0x1a5], rcx
0x141e46222 mov r13, qword ptr [rbp + 0x168]
0x141e46229 cmp dword ptr [r13], 1
0x141e4622e jne 0x141e46246
0x141e46230 mov rax, qword ptr [r13 + 8]
0x141e46234 inc rax
0x141e46237 mov rcx, 0xffffffffffffffff
0x141e4623e cmove rax, rcx
0x141e46242 mov qword ptr [r13 + 8], rax
0x141e46246 movzx edi, byte ptr [rbp + 0x1c9]
0x141e4624d mov rax, qword ptr [rbp + 0x1a0]
0x141e46254 mov rcx, qword ptr [rbp + 0x1a5]
0x141e4625b mov qword ptr [rbp + 0x15d], rcx
0x141e46262 mov qword ptr [rbp + 0x158], rax
0x141e46269 mov byte ptr [rbp + 0x195], r11b
0x141e46270 movss xmm8, dword ptr [rbp + 0x98]
0x141e46279 mov dword ptr [rbp + 0x88], r10d
0x141e46280 movss xmm6, dword ptr [rbp + 0x94]
0x141e46288 mov dword ptr [rbp + 0x84], r9d
0x141e4628f mov rbx, qword ptr [rbp + 0x20]
0x141e46293 mov r12d, dword ptr [rbp + 0xa4]
0x141e4629a mov word ptr [rbp + 0x17a], r8w
0x141e462a2 movaps xmm9, xmmword ptr [rbp - 0x50]
0x141e462a7 movdqa xmm14, xmmword ptr [rbp - 0x30]
0x141e462ad movdqa xmm10, xmm7
0x141e462b2 jmp 0x141e466b3
0x141e462b7 nop word ptr [rax + rax]
0x141e462c0 test al, 1
0x141e462c2 je 0x141e462d8
0x141e462c4 mov rax, qword ptr [rdi + 8]
0x141e462c8 test rax, rax
0x141e462cb je 0x141e4737d
0x141e462d1 dec rax
0x141e462d4 mov qword ptr [rdi + 8], rax
0x141e462d8 mov qword ptr [rbp + 0xc8], rdi
0x141e462df mov byte ptr [rbp + 0xd0], 1
0x141e462e6 mov qword ptr [rbp + 0x1b0], r8
0x141e462ed lea rcx, [rbp + 0x1c8]
0x141e462f4 lea rdx, [rbp + 0xc8]
0x141e462fb call 0x142373e70
0x141e46300 movzx r14d, byte ptr [rbp + 0x1c8]
0x141e46308 cmp r14b, 0x32
0x141e4630c movzx r11d, byte ptr [rbp + 0x196]
0x141e46314 movzx r8d, word ptr [rbp + 0x17c]
0x141e4631c mov r9d, dword ptr [rbp + 0x8c]
0x141e46323 mov r10d, dword ptr [rbp + 0x90]
0x141e4632a je 0x141e463c0
0x141e46330 movzx r8d, word ptr [rbp + 0x1ca]
0x141e46338 mov eax, dword ptr [rbp + 0x1cc]
0x141e4633e mov dword ptr [rbp + 0xa8], eax
0x141e46344 mov rax, qword ptr [rbp + 0x1d0]
0x141e4634b mov qword ptr [rbp + 0x28], rax
0x141e4634f movdqu xmm15, xmmword ptr [rbp + 0x1d8]
0x141e46358 movsd xmm0, qword ptr [rbp + 0x1e8]
0x141e46360 movaps xmmword ptr [rbp - 0x40], xmm0
0x141e46364 mov r9d, dword ptr [rbp + 0x1f0]
0x141e4636b movss xmm0, dword ptr [rbp + 0x1f4]
0x141e46373 movss dword ptr [rbp + 0x9c], xmm0
0x141e4637b mov r10d, dword ptr [rbp + 0x1f8]
0x141e46382 movd xmm0, dword ptr [rbp + 0x1fc]
0x141e4638a movd dword ptr [rbp + 0xa0], xmm0
0x141e46392 movq xmm12, qword ptr [rbp + 0x200]
0x141e4639b movzx r11d, byte ptr [rbp + 0x202]
0x141e463a3 lea rcx, [rbp + 0x1c9]
0x141e463aa mov rax, qword ptr [rcx + 0x3a]
0x141e463ae mov rcx, qword ptr [rcx + 0x3f]
0x141e463b2 mov qword ptr [rbp + 0x1a0], rax
0x141e463b9 mov qword ptr [rbp + 0x1a5], rcx
0x141e463c0 mov r13, qword ptr [rbp + 0x168]
0x141e463c7 cmp dword ptr [r13], 1
0x141e463cc jne 0x141e463e4
0x141e463ce mov rax, qword ptr [r13 + 8]
0x141e463d2 inc rax
0x141e463d5 mov rcx, 0xffffffffffffffff
0x141e463dc cmove rax, rcx
0x141e463e0 mov qword ptr [r13 + 8], rax
0x141e463e4 movzx edi, byte ptr [rbp + 0x1c9]
0x141e463eb mov rax, qword ptr [rbp + 0x1a0]
0x141e463f2 mov rcx, qword ptr [rbp + 0x1a5]
0x141e463f9 mov qword ptr [rbp + 0x15d], rcx
0x141e46400 mov qword ptr [rbp + 0x158], rax
0x141e46407 mov byte ptr [rbp + 0x196], r11b
0x141e4640e movss xmm8, dword ptr [rbp + 0xa0]
0x141e46417 mov dword ptr [rbp + 0x90], r10d
0x141e4641e movss xmm6, dword ptr [rbp + 0x9c]
0x141e46426 mov dword ptr [rbp + 0x8c], r9d
0x141e4642d mov rbx, qword ptr [rbp + 0x28]
0x141e46431 mov r12d, dword ptr [rbp + 0xa8]
0x141e46438 mov word ptr [rbp + 0x17c], r8w
0x141e46440 movaps xmm9, xmmword ptr [rbp - 0x40]
0x141e46445 movdqa xmm14, xmm15
0x141e4644a movdqa xmm10, xmm12
0x141e4644f jmp 0x141e466b3
0x141e46454 nop word ptr cs:[rax + rax]
0x141e46460 test al, 1
0x141e46462 je 0x141e46478
0x141e46464 mov rax, qword ptr [rdi + 8]
0x141e46468 test rax, rax
0x141e4646b je 0x141e4737d
0x141e46471 dec rax
0x141e46474 mov qword ptr [rdi + 8], rax
0x141e46478 lea rdx, [rdi + 0x10]
0x141e4647c mov qword ptr [rbp + 0x1b0], r8
0x141e46483 lea rcx, [rbp + 0x1c8]
0x141e4648a call 0x140bc9b80
0x141e4648f movzx r14d, byte ptr [rbp + 0x1c8]
0x141e46497 cmp r14b, 0x32
0x141e4649b jne 0x141e46550
0x141e464a1 mov rdx, qword ptr [rbp + 0x1d0]
0x141e464a8 mov r8, qword ptr [rbp + 0x1d8]
0x141e464af mov qword ptr [rdi + 0x40], rdx
0x141e464b3 mov qword ptr [rdi + 0x48], r8
0x141e464b7 mov rax, qword ptr [rbp + 0x1b8]
0x141e464be mov qword ptr [rbp + 0x1b0], rax
0x141e464c5 lea rcx, [rbp + 0xc8]
0x141e464cc call 0x140fc9a40
0x141e464d1 movzx r14d, byte ptr [rbp + 0xc8]
0x141e464d9 cmp r14b, 0x32
0x141e464dd jne 0x141e46590
0x141e464e3 mov r14b, 0x32
0x141e464e6 movzx r8d, byte ptr [rbp + 0xc9]
0x141e464ee movzx r10d, word ptr [rbp + 0x17e]
0x141e464f6 mov r12d, dword ptr [rbp + 0xb4]
0x141e464fd mov rbx, qword ptr [rbp + 0x30]
0x141e46501 mov r9d, dword ptr [rbp + 0xb8]
0x141e46508 movss xmm6, dword ptr [rbp + 0xac]
0x141e46510 mov r11d, dword ptr [rbp + 0xbc]
0x141e46517 movss xmm8, dword ptr [rbp + 0xb0]
0x141e46520 movzx r15d, byte ptr [rbp + 0x197]
0x141e46528 movaps xmm9, xmmword ptr [rbp - 0x20]
0x141e4652d movdqa xmm14, xmmword ptr [rbp - 0x10]
0x141e46533 movdqa xmm10, xmmword ptr [rbp]
0x141e46539 mov r13, qword ptr [rbp + 0x168]
0x141e46540 cmp dword ptr [r13], 1
0x141e46545 je 0x141e4662b
0x141e4654b jmp 0x141e46641
0x141e46550 lea rdx, [rbp + 0x1c9]
0x141e46557 mov eax, dword ptr [rdx]
0x141e46559 mov ecx, dword ptr [rdx + 3]
0x141e4655c lea rdi, [rbp + 0xc9]
0x141e46563 mov dword ptr [rdi + 3], ecx
0x141e46566 mov dword ptr [rdi], eax
0x141e46568 mov rbx, qword ptr [rbp + 0x1d0]
0x141e4656f mov rax, qword ptr [rbp + 0x1d8]
0x141e46576 movups xmm0, xmmword ptr [rdx + 0x17]
0x141e4657a movups xmm1, xmmword ptr [rdx + 0x27]
0x141e4657e movups xmm2, xmmword ptr [rdx + 0x37]
0x141e46582 movups xmmword ptr [rdi + 0x37], xmm2
0x141e46586 movups xmmword ptr [rdi + 0x27], xmm1
0x141e4658a movups xmmword ptr [rdi + 0x17], xmm0
0x141e4658e jmp 0x141e465a5
0x141e46590 mov rbx, qword ptr [rbp + 0xd0]
0x141e46597 mov rax, qword ptr [rbp + 0xd8]
0x141e4659e lea rdi, [rbp + 0xc9]
0x141e465a5 movzx r8d, byte ptr [rbp + 0xc9]
0x141e465ad movzx r10d, word ptr [rbp + 0xca]
0x141e465b5 mov r12d, dword ptr [rbp + 0xcc]
0x141e465bc movsd xmm9, qword ptr [rbp + 0xe8]
0x141e465c5 mov r9d, dword ptr [rbp + 0xf0]
0x141e465cc movss xmm6, dword ptr [rbp + 0xf4]
0x141e465d4 mov r11d, dword ptr [rbp + 0xf8]
0x141e465db movss xmm8, dword ptr [rbp + 0xfc]
0x141e465e4 movq xmm10, qword ptr [rbp + 0x100]
0x141e465ed movzx r15d, byte ptr [rbp + 0x102]
0x141e465f5 mov rcx, qword ptr [rdi + 0x3a]
0x141e465f9 mov rdx, qword ptr [rdi + 0x3f]
0x141e465fd mov qword ptr [rbp + 0x1c8], rcx
0x141e46604 mov qword ptr [rbp + 0x1cd], rdx
0x141e4660b movq xmm14, rax
0x141e46610 movq xmm0, qword ptr [rbp + 0xe0]
0x141e46618 punpcklqdq xmm14, xmm0
0x141e4661d mov r13, qword ptr [rbp + 0x168]
0x141e46624 cmp dword ptr [r13], 1
0x141e46629 jne 0x141e46641
0x141e4662b mov rax, qword ptr [r13 + 8]
0x141e4662f inc rax
0x141e46632 mov rcx, 0xffffffffffffffff
0x141e46639 cmove rax, rcx
0x141e4663d mov qword ptr [r13 + 8], rax
0x141e46641 mov rax, qword ptr [rbp + 0x1c8]
0x141e46648 mov rcx, qword ptr [rbp + 0x1cd]
0x141e4664f mov qword ptr [rbp + 0x15d], rcx
0x141e46656 mov qword ptr [rbp + 0x158], rax
0x141e4665d mov word ptr [rbp + 0x17e], r10w
0x141e46665 mov dword ptr [rbp + 0xb4], r12d
0x141e4666c mov qword ptr [rbp + 0x30], rbx
0x141e46670 mov dword ptr [rbp + 0xb8], r9d
0x141e46677 movss dword ptr [rbp + 0xac], xmm6
0x141e4667f mov dword ptr [rbp + 0xbc], r11d
0x141e46686 movss dword ptr [rbp + 0xb0], xmm8
0x141e4668f mov byte ptr [rbp + 0x197], r15b
0x141e46696 movaps xmmword ptr [rbp - 0x20], xmm9
0x141e4669b movdqa xmmword ptr [rbp - 0x10], xmm14
0x141e466a1 movdqa xmmword ptr [rbp], xmm10
0x141e466a7 mov edi, r8d
0x141e466aa mov r8d, r10d
0x141e466ad mov r10d, r11d
0x141e466b0 mov r11d, r15d
0x141e466b3 cmp r14b, 0x32
0x141e466b7 mov dword ptr [rbp + 0x1c4], r9d
0x141e466be jne 0x141e473a3
0x141e466c4 cmp dil, 6
0x141e466c8 mov byte ptr [rbp + 0x188], r11b
0x141e466cf mov qword ptr [rbp + 0xc0], rbx
0x141e466d6 mov dword ptr [rbp + 0x180], r12d
0x141e466dd mov byte ptr [rbp + 0x1ad], dil
0x141e466e4 jne 0x141e46845
0x141e466ea cmp byte ptr [rbp + 0x1ae], 0xa
0x141e466f1 mov rax, qword ptr [rbp + 0x1b8]
0x141e466f8 jne 0x141e47c0f
0x141e466fe mov ebx, r10d
0x141e46701 mov r12w, r8w
0x141e46705 lea rdi, [r13 + 0x10]
0x141e46709 mov qword ptr [rbp + 0x1b0], rax
0x141e46710 lea rcx, [rbp + 0x1c8]
0x141e46717 mov rdx, rdi
0x141e4671a call 0x140bca140
0x141e4671f movzx r14d, byte ptr [rbp + 0x1c8]
0x141e46727 cmp r14b, 0x32
0x141e4672b jne 0x141e46ff8
0x141e46731 mov r14, qword ptr [rbp + 0x1b8]
0x141e46738 mov qword ptr [rbp + 0x1b0], r14
0x141e4673f mov rcx, rdi
0x141e46742 mov edx, 0x3a
0x141e46747 call 0x140bc41b0
0x141e4674c test al, al
0x141e4674e je 0x141e47279
0x141e46754 mov qword ptr [rbp + 0x1b0], r14
0x141e4675b lea rcx, [rbp + 0x1c8]
0x141e46762 mov rdx, rdi
0x141e46765 call 0x140bca140
0x141e4676a movzx r14d, byte ptr [rbp + 0x1c8]
0x141e46772 cmp r14b, 0x32
0x141e46776 jne 0x141e46ff8
0x141e4677c test byte ptr [r13], 1
0x141e46781 mov rcx, qword ptr [rbp + 0x1b8]
0x141e46788 je 0x141e4679e
0x141e4678a mov rax, qword ptr [r13 + 8]
0x141e4678e test rax, rax
0x141e46791 je 0x141e47430
0x141e46797 dec rax
0x141e4679a mov qword ptr [r13 + 8], rax
0x141e4679e mov qword ptr [rbp + 0x1b0], rcx
0x141e467a5 mov r9d, 0xa
0x141e467ab lea rcx, [rbp + 0x1c8]
0x141e467b2 mov rdx, r13
0x141e467b5 lea r8, [rip + 0xd6e03c]
0x141e467bc call 0x141e43f30
0x141e467c1 movzx r14d, byte ptr [rbp + 0x1c8]
0x141e467c9 movzx eax, byte ptr [rbp + 0x1c9]
0x141e467d0 mov byte ptr [rbp + 0x1ae], al
0x141e467d6 cmp r14b, 0x32
0x141e467da jne 0x141e47281
0x141e467e0 test byte ptr [r13], 1
0x141e467e5 je 0x141e467fd
0x141e467e7 mov rax, qword ptr [r13 + 8]
0x141e467eb inc rax
0x141e467ee mov rcx, 0xffffffffffffffff
0x141e467f5 cmove rax, rcx
0x141e467f9 mov qword ptr [r13 + 8], rax
0x141e467fd mov rax, qword ptr [rbp + 0x1b8]
0x141e46804 mov qword ptr [rbp + 0x1b0], rax
0x141e4680b lea rcx, [rbp + 0x1c8]
0x141e46812 mov rdx, rdi
0x141e46815 call 0x140bc8550
0x141e4681a movzx r14d, byte ptr [rbp + 0x1c8]
0x141e46822 movzx ecx, byte ptr [rbp + 0x1c9]
0x141e46829 cmp r14b, 0x32
0x141e4682d jne 0x141e472fe
0x141e46833 mov byte ptr [rbp + 0x171], cl
0x141e46839 mov rdi, qword ptr [rbp + 0x1b8]
0x141e46840 jmp 0x141e460ba
0x141e46845 movzx eax, dil
0x141e46849 mov eax, eax
0x141e4684b lea rcx, [rip + 0x1148ac6]
0x141e46852 movsxd rax, dword ptr [rcx + rax*4]
0x141e46856 add rax, rcx
0x141e46859 mov rdx, qword ptr [rbp + 0x1b8]
0x141e46860 jmp rax
0x141e46862 cmp dword ptr [rbp + 0x190], 2
0x141e46869 jne 0x141e47d2b
0x141e4686f mov ebx, r10d
0x141e46872 mov r12w, r8w
0x141e46876 lea rdi, [r13 + 0x10]
0x141e4687a mov qword ptr [rbp + 0x1b0], rdx
0x141e46881 lea rcx, [rbp + 0x1c8]
0x141e46888 mov rdx, rdi
0x141e4688b call 0x140bca140
0x141e46890 movzx r14d, byte ptr [rbp + 0x1c8]
0x141e46898 cmp r14b, 0x32
0x141e4689c jne 0x141e477ce
0x141e468a2 mov r14, qword ptr [rbp + 0x1b8]
0x141e468a9 mov qword ptr [rbp + 0x1b0], r14
0x141e468b0 mov rcx, rdi
0x141e468b3 mov edx, 0x3a
0x141e468b8 call 0x140bc41b0
0x141e468bd movd xmm13, dword ptr [rbp + 0x114]
0x141e468c6 test al, al
0x141e468c8 je 0x141e479e8
0x141e468ce mov qword ptr [rbp + 0x1b0], r14
0x141e468d5 lea rcx, [rbp + 0x1c8]
0x141e468dc mov rdx, rdi
0x141e468df call 0x140bca140
0x141e468e4 movzx r14d, byte ptr [rbp + 0x1c8]
0x141e468ec cmp r14b, 0x32
0x141e468f0 jne 0x141e477ce
0x141e468f6 test byte ptr [r13], 1
0x141e468fb mov rcx, qword ptr [rbp + 0x1b8]
0x141e46902 je 0x141e46918
0x141e46904 mov rax, qword ptr [r13 + 8]
0x141e46908 test rax, rax
0x141e4690b je 0x141e47c2b
0x141e46911 dec rax
0x141e46914 mov qword ptr [r13 + 8], rax
0x141e46918 mov qword ptr [rbp + 0x1b0], rcx
0x141e4691f lea rcx, [rbp + 0x1c8]
0x141e46926 mov rdx, rdi
0x141e46929 call 0x141e5cb00
0x141e4692e movzx r14d, byte ptr [rbp + 0x1c8]
0x141e46936 cmp r14b, 0x32
0x141e4693a jne 0x141e477ce
0x141e46940 test byte ptr [r13], 1
0x141e46945 je 0x141e4695d
0x141e46947 mov rax, qword ptr [r13 + 8]
0x141e4694b inc rax
0x141e4694e mov rcx, 0xffffffffffffffff
0x141e46955 cmove rax, rcx
0x141e46959 mov qword ptr [r13 + 8], rax
0x141e4695d mov eax, dword ptr [rbp + 0x1cc]
0x141e46963 mov dword ptr [rbp + 0x114], eax
0x141e46969 mov rax, qword ptr [rbp + 0x1b8]
0x141e46970 mov qword ptr [rbp + 0x1b0], rax
0x141e46977 lea rcx, [rbp + 0x1c8]
0x141e4697e mov rdx, rdi
0x141e46981 call 0x140bc8550
0x141e46986 movzx r14d, byte ptr [rbp + 0x1c8]
0x141e4698e movzx edi, byte ptr [rbp + 0x1c9]
0x141e46995 cmp r14b, 0x32
0x141e46999 jne 0x141e477d5
0x141e4699f mov byte ptr [rbp + 0x171], dil
0x141e469a6 mov dword ptr [rbp + 0x190], 1
0x141e469b0 mov rdi, qword ptr [rbp + 0x1b8]
0x141e469b7 jmp 0x141e460ba
0x141e469bc cmp dword ptr [rbp + 0x18c], 2
0x141e469c3 jne 0x141e47d71
0x141e469c9 mov ebx, r10d
0x141e469cc mov r12w, r8w
0x141e469d0 lea rdi, [r13 + 0x10]
0x141e469d4 mov qword ptr [rbp + 0x1b0], rdx
0x141e469db lea rcx, [rbp + 0x1c8]
0x141e469e2 mov rdx, rdi
0x141e469e5 call 0x140bca140
0x141e469ea movzx r14d, byte ptr [rbp + 0x1c8]
0x141e469f2 cmp r14b, 0x32
0x141e469f6 jne 0x141e476b8
0x141e469fc mov r14, qword ptr [rbp + 0x1b8]
0x141e46a03 mov qword ptr [rbp + 0x1b0], r14
0x141e46a0a mov rcx, rdi
0x141e46a0d mov edx, 0x3a
0x141e46a12 call 0x140bc41b0
0x141e46a17 test al, al
0x141e46a19 je 0x141e479f0
0x141e46a1f mov qword ptr [rbp + 0x1b0], r14
0x141e46a26 lea rcx, [rbp + 0x1c8]
0x141e46a2d mov rdx, rdi
0x141e46a30 call 0x140bca140
0x141e46a35 movzx r14d, byte ptr [rbp + 0x1c8]
0x141e46a3d cmp r14b, 0x32
0x141e46a41 jne 0x141e476b8
0x141e46a47 test byte ptr [r13], 1
0x141e46a4c mov rcx, qword ptr [rbp + 0x1b8]
0x141e46a53 je 0x141e46a69
0x141e46a55 mov rax, qword ptr [r13 + 8]
0x141e46a59 test rax, rax
0x141e46a5c je 0x141e47c77
0x141e46a62 dec rax
0x141e46a65 mov qword ptr [r13 + 8], rax
0x141e46a69 mov qword ptr [rbp + 0x1b0], rcx
0x141e46a70 lea rcx, [rbp + 0x1c8]
0x141e46a77 mov rdx, rdi
0x141e46a7a call 0x141e5b990
0x141e46a7f movzx r14d, byte ptr [rbp + 0x1c8]
0x141e46a87 cmp r14b, 0x32
0x141e46a8b jne 0x141e476b8
0x141e46a91 test byte ptr [r13], 1
0x141e46a96 je 0x141e46aae
0x141e46a98 mov rax, qword ptr [r13 + 8]
0x141e46a9c inc rax
0x141e46a9f mov rcx, 0xffffffffffffffff
0x141e46aa6 cmove rax, rcx
0x141e46aaa mov qword ptr [r13 + 8], rax
0x141e46aae movd xmm0, dword ptr [rbp + 0x1cc]
0x141e46ab6 movdqa xmmword ptr [rbp + 0x50], xmm0
0x141e46abb mov rax, qword ptr [rbp + 0x1b8]
0x141e46ac2 mov qword ptr [rbp + 0x1b0], rax
0x141e46ac9 lea rcx, [rbp + 0x1c8]
0x141e46ad0 mov rdx, rdi
0x141e46ad3 call 0x140bc8550
0x141e46ad8 movzx r14d, byte ptr [rbp + 0x1c8]
0x141e46ae0 movzx edi, byte ptr [rbp + 0x1c9]
0x141e46ae7 cmp r14b, 0x32
0x141e46aeb jne 0x141e476bf
0x141e46af1 mov byte ptr [rbp + 0x171], dil
0x141e46af8 mov dword ptr [rbp + 0x18c], 1
0x141e46b02 mov rdi, qword ptr [rbp + 0x1b8]
0x141e46b09 jmp 0x141e460ba
0x141e46b0e cmp byte ptr [rbp + 0x1c3], 6
0x141e46b15 jne 0x141e47d22
0x141e46b1b mov ebx, r10d
0x141e46b1e mov r12w, r8w
0x141e46b22 lea rdi, [r13 + 0x10]
0x141e46b26 mov qword ptr [rbp + 0x1b0], rdx
0x141e46b2d lea rcx, [rbp + 0x1c8]
0x141e46b34 mov rdx, rdi
0x141e46b37 call 0x140bca140
0x141e46b3c movzx r14d, byte ptr [rbp + 0x1c8]
0x141e46b44 cmp r14b, 0x32
0x141e46b48 jne 0x141e47854
0x141e46b4e mov r14, qword ptr [rbp + 0x1b8]
0x141e46b55 mov qword ptr [rbp + 0x1b0], r14
0x141e46b5c mov rcx, rdi
0x141e46b5f mov edx, 0x3a
0x141e46b64 call 0x140bc41b0
0x141e46b69 test al, al
0x141e46b6b je 0x141e47a00
0x141e46b71 mov qword ptr [rbp + 0x1b0], r14
0x141e46b78 lea rcx, [rbp + 0x1c8]
0x141e46b7f mov rdx, rdi
0x141e46b82 call 0x140bca140
0x141e46b87 movzx r14d, byte ptr [rbp + 0x1c8]
0x141e46b8f cmp r14b, 0x32
0x141e46b93 jne 0x141e47854
0x141e46b99 test byte ptr [r13], 1
0x141e46b9e mov rcx, qword ptr [rbp + 0x1b8]
0x141e46ba5 je 0x141e46bbb
0x141e46ba7 mov rax, qword ptr [r13 + 8]
0x141e46bab test rax, rax
0x141e46bae je 0x141e47ca7
0x141e46bb4 dec rax
0x141e46bb7 mov qword ptr [r13 + 8], rax
0x141e46bbb mov qword ptr [rbp + 0x1b0], rcx
0x141e46bc2 mov r9d, 9
0x141e46bc8 lea rcx, [rbp + 0x1c8]
0x141e46bcf mov rdx, r13
0x141e46bd2 lea r8, [rip + 0xd6dba7]
0x141e46bd9 call 0x141e43630
0x141e46bde movzx r14d, byte ptr [rbp + 0x1c8]
0x141e46be6 movzx eax, byte ptr [rbp + 0x1c9]
0x141e46bed mov byte ptr [rbp + 0x1c3], al
0x141e46bf3 cmp r14b, 0x32
0x141e46bf7 jne 0x141e47a10
0x141e46bfd test byte ptr [r13], 1
0x141e46c02 je 0x141e46c1a
0x141e46c04 mov rax, qword ptr [r13 + 8]
0x141e46c08 inc rax
0x141e46c0b mov rcx, 0xffffffffffffffff
0x141e46c12 cmove rax, rcx
0x141e46c16 mov qword ptr [r13 + 8], rax
0x141e46c1a mov rax, qword ptr [rbp + 0x1b8]
0x141e46c21 mov qword ptr [rbp + 0x1b0], rax
0x141e46c28 lea rcx, [rbp + 0x1c8]
0x141e46c2f mov rdx, rdi
0x141e46c32 call 0x140bc8550
0x141e46c37 movzx r14d, byte ptr [rbp + 0x1c8]
0x141e46c3f movzx ecx, byte ptr [rbp + 0x1c9]
0x141e46c46 cmp r14b, 0x32
0x141e46c4a je 0x141e46833
0x141e46c50 jmp 0x141e47b0a
0x141e46c55 cmp byte ptr [rbp + 0x1c2], 5
0x141e46c5c jne 0x141e47ceb
0x141e46c62 mov ebx, r10d
0x141e46c65 mov r12w, r8w
0x141e46c69 lea rdi, [r13 + 0x10]
0x141e46c6d mov qword ptr [rbp + 0x1b0], rdx
0x141e46c74 lea rcx, [rbp + 0x1c8]
0x141e46c7b mov rdx, rdi
0x141e46c7e call 0x140bca140
0x141e46c83 movzx r14d, byte ptr [rbp + 0x1c8]
0x141e46c8b cmp r14b, 0x32
0x141e46c8f jne 0x141e478d8
0x141e46c95 mov r14, qword ptr [rbp + 0x1b8]
0x141e46c9c mov qword ptr [rbp + 0x1b0], r14
0x141e46ca3 mov rcx, rdi
0x141e46ca6 mov edx, 0x3a
0x141e46cab call 0x140bc41b0
0x141e46cb0 test al, al
0x141e46cb2 je 0x141e47a08
0x141e46cb8 mov qword ptr [rbp + 0x1b0], r14
0x141e46cbf lea rcx, [rbp + 0x1c8]
0x141e46cc6 mov rdx, rdi
0x141e46cc9 call 0x140bca140
0x141e46cce movzx r14d, byte ptr [rbp + 0x1c8]
0x141e46cd6 cmp r14b, 0x32
0x141e46cda jne 0x141e478d8
0x141e46ce0 test byte ptr [r13], 1
0x141e46ce5 mov rcx, qword ptr [rbp + 0x1b8]
0x141e46cec je 0x141e46d02
0x141e46cee mov rax, qword ptr [r13 + 8]
0x141e46cf2 test rax, rax
0x141e46cf5 je 0x141e47cc9
0x141e46cfb dec rax
0x141e46cfe mov qword ptr [r13 + 8], rax
0x141e46d02 mov qword ptr [rbp + 0x1b0], rcx
0x141e46d09 mov r9d, 0xa
0x141e46d0f lea rcx, [rbp + 0x1c8]
0x141e46d16 mov rdx, r13
0x141e46d19 lea r8, [rip + 0x114c380]
0x141e46d20 call 0x141e43ab0
0x141e46d25 movzx r14d, byte ptr [rbp + 0x1c8]
0x141e46d2d movzx eax, byte ptr [rbp + 0x1c9]
0x141e46d34 mov byte ptr [rbp + 0x1c2], al
0x141e46d3a cmp r14b, 0x32
0x141e46d3e jne 0x141e47a8d
0x141e46d44 test byte ptr [r13], 1
0x141e46d49 je 0x141e46d61
0x141e46d4b mov rax, qword ptr [r13 + 8]
0x141e46d4f inc rax
0x141e46d52 mov rcx, 0xffffffffffffffff
0x141e46d59 cmove rax, rcx
0x141e46d5d mov qword ptr [r13 + 8], rax
0x141e46d61 mov rax, qword ptr [rbp + 0x1b8]
0x141e46d68 mov qword ptr [rbp + 0x1b0], rax
0x141e46d6f lea rcx, [rbp + 0x1c8]
0x141e46d76 mov rdx, rdi
0x141e46d79 call 0x140bc8550
0x141e46d7e movzx r14d, byte ptr [rbp + 0x1c8]
0x141e46d86 movzx ecx, byte ptr [rbp + 0x1c9]
0x141e46d8d cmp r14b, 0x32
0x141e46d91 je 0x141e46833
0x141e46d97 jmp 0x141e47b90
0x141e46d9c cmp dword ptr [rbp + 0x184], 2
0x141e46da3 jne 0x141e47d55
0x141e46da9 mov ebx, r10d
0x141e46dac mov r12w, r8w
0x141e46db0 lea rdi, [r13 + 0x10]
0x141e46db4 mov qword ptr [rbp + 0x1b0], rdx
0x141e46dbb lea rcx, [rbp + 0x1c8]
0x141e46dc2 mov rdx, rdi
0x141e46dc5 call 0x140bca140
0x141e46dca movzx r14d, byte ptr [rbp + 0x1c8]
0x141e46dd2 cmp r14b, 0x32
0x141e46dd6 jne 0x141e47743
0x141e46ddc mov r14, qword ptr [rbp + 0x1b8]
0x141e46de3 mov qword ptr [rbp + 0x1b0], r14
0x141e46dea mov rcx, rdi
0x141e46ded mov edx, 0x3a
0x141e46df2 call 0x140bc41b0
0x141e46df7 test al, al
0x141e46df9 je 0x141e479f8
0x141e46dff mov qword ptr [rbp + 0x1b0], r14
0x141e46e06 lea rcx, [rbp + 0x1c8]
0x141e46e0d mov rdx, rdi
0x141e46e10 call 0x140bca140
0x141e46e15 movzx r14d, byte ptr [rbp + 0x1c8]
0x141e46e1d cmp r14b, 0x32
0x141e46e21 jne 0x141e47743
0x141e46e27 test byte ptr [r13], 1
0x141e46e2c mov rcx, qword ptr [rbp + 0x1b8]
0x141e46e33 je 0x141e46e49
0x141e46e35 mov rax, qword ptr [r13 + 8]
0x141e46e39 test rax, rax
0x141e46e3c je 0x141e47c47
0x141e46e42 dec rax
0x141e46e45 mov qword ptr [r13 + 8], rax
0x141e46e49 mov qword ptr [rbp + 0x1b0], rcx
0x141e46e50 lea rcx, [rbp + 0x1c8]
0x141e46e57 mov rdx, rdi
0x141e46e5a call 0x141e5b990
0x141e46e5f movzx r14d, byte ptr [rbp + 0x1c8]
0x141e46e67 cmp r14b, 0x32
0x141e46e6b jne 0x141e47743
0x141e46e71 test byte ptr [r13], 1
0x141e46e76 je 0x141e46e8e
0x141e46e78 mov rax, qword ptr [r13 + 8]
0x141e46e7c inc rax
0x141e46e7f mov rcx, 0xffffffffffffffff
0x141e46e86 cmove rax, rcx
0x141e46e8a mov qword ptr [r13 + 8], rax
0x141e46e8e movd xmm0, dword ptr [rbp + 0x1cc]
0x141e46e96 movdqa xmmword ptr [rbp + 0x40], xmm0
0x141e46e9b mov rax, qword ptr [rbp + 0x1b8]
0x141e46ea2 mov qword ptr [rbp + 0x1b0], rax
0x141e46ea9 lea rcx, [rbp + 0x1c8]
0x141e46eb0 mov rdx, rdi
0x141e46eb3 call 0x140bc8550
0x141e46eb8 movzx r14d, byte ptr [rbp + 0x1c8]
0x141e46ec0 movzx edi, byte ptr [rbp + 0x1c9]
0x141e46ec7 cmp r14b, 0x32
0x141e46ecb jne 0x141e4774a
0x141e46ed1 mov byte ptr [rbp + 0x171], dil
0x141e46ed8 mov dword ptr [rbp + 0x184], 1
0x141e46ee2 mov rdi, qword ptr [rbp + 0x1b8]
0x141e46ee9 jmp 0x141e460ba
0x141e46eee mov rax, rdx
0x141e46ef1 neg rax
0x141e46ef4 jno 0x141e47cf7
0x141e46efa mov ebx, r10d
0x141e46efd mov r12w, r8w
0x141e46f01 movabs rax, 0x8000000000000000
0x141e46f0b mov qword ptr [rbp + 0x1b0], rax
0x141e46f12 lea rcx, [rbp + 0x1c8]
0x141e46f19 lea rdx, [rbp + 0x168]
0x141e46f20 call 0x141e3d970
0x141e46f25 movzx r14d, byte ptr [rbp + 0x1c8]
0x141e46f2d cmp r14b, 0x32
0x141e46f31 jne 0x141e4795c
0x141e46f37 mov rdi, qword ptr [rbp + 0x1d0]
0x141e46f3e movdqu xmm0, xmmword ptr [rbp + 0x1d8]
0x141e46f46 movdqa xmmword ptr [rbp + 0x70], xmm0
0x141e46f4b jmp 0x141e460b3
0x141e46f50 mov rax, qword ptr [rbp + 0x208]
0x141e46f57 mov qword ptr [rsi + 0x40], rax
0x141e46f5b movdqu xmm0, xmmword ptr [rbp + 0x1c8]
0x141e46f63 movups xmm1, xmmword ptr [rbp + 0x1d8]
0x141e46f6a movups xmm2, xmmword ptr [rbp + 0x1e8]
0x141e46f71 movups xmm3, xmmword ptr [rbp + 0x1f8]
0x141e46f78 movups xmmword ptr [rsi + 0x30], xmm3
0x141e46f7c movups xmmword ptr [rsi + 0x20], xmm2
0x141e46f80 movups xmmword ptr [rsi + 0x10], xmm1
0x141e46f84 movdqu xmmword ptr [rsi], xmm0
0x141e46f88 jmp 0x141e4763e
0x141e46f8d movzx ecx, byte ptr [rbp + 0x1c9]
0x141e46f94 mov rdx, qword ptr [rbp + 0x208]
0x141e46f9b mov qword ptr [rsi + 0x40], rdx
0x141e46f9f movups xmm0, xmmword ptr [rbp + 0x1fa]
0x141e46fa6 movups xmmword ptr [rsi + 0x32], xmm0
0x141e46faa movups xmm0, xmmword ptr [rbp + 0x1ea]
0x141e46fb1 movups xmmword ptr [rsi + 0x22], xmm0
0x141e46fb5 movups xmm0, xmmword ptr [rbp + 0x1da]
0x141e46fbc movups xmmword ptr [rsi + 0x12], xmm0
0x141e46fc0 movdqu xmm0, xmmword ptr [rbp + 0x1ca]
0x141e46fc8 movdqu xmmword ptr [rsi + 2], xmm0
0x141e46fcd mov byte ptr [rsi], al
0x141e46fcf mov byte ptr [rsi + 1], cl
0x141e46fd2 jmp 0x141e4763e
0x141e46fd7 test rdi, rdi
0x141e46fda je 0x141e4707c
0x141e46fe0 mov byte ptr [rsi], 0x17
0x141e46fe3 mov qword ptr [rsi + 8], rbx
0x141e46fe7 mov qword ptr [rsi + 0x10], rdi
0x141e46feb jmp 0x141e4763e
0x141e46ff0 mov byte ptr [rsi], 0x31
0x141e46ff3 jmp 0x141e4763e
0x141e46ff8 movzx edi, byte ptr [rbp + 0x1c9]
0x141e46fff movzx r8d, word ptr [rbp + 0x1ca]
0x141e47007 mov r12d, dword ptr [rbp + 0x1cc]
0x141e4700e mov rax, qword ptr [rbp + 0x1d0]
0x141e47015 movups xmm11, xmmword ptr [rbp + 0x1d8]
0x141e4701d movsd xmm9, qword ptr [rbp + 0x1e8]
0x141e47026 mov ecx, dword ptr [rbp + 0x1f0]
0x141e4702c mov dword ptr [rbp + 0x1c4], ecx
0x141e47032 movss xmm6, dword ptr [rbp + 0x1f4]
0x141e4703a mov r10d, dword ptr [rbp + 0x1f8]
0x141e47041 movss xmm8, dword ptr [rbp + 0x1fc]
0x141e4704a movsd xmm10, qword ptr [rbp + 0x200]
0x141e47053 movzx r11d, byte ptr [rbp + 0x202]
0x141e4705b mov rcx, qword ptr [rbp + 0x203]
0x141e47062 mov qword ptr [rbp + 0x118], rcx
0x141e47069 mov rcx, qword ptr [rbp + 0x208]
0x141e47070 mov qword ptr [rbp + 0x11d], rcx
0x141e47077 jmp 0x141e4743a
0x141e4707c mov byte ptr [rsi], 0x16
0x141e4707f jmp 0x141e4763e
0x141e47084 movzx r8d, word ptr [rbp + 0x1ca]
0x141e4708c mov r12d, dword ptr [rbp + 0x1cc]
0x141e47093 mov rbx, qword ptr [rbp + 0x1d0]
0x141e4709a movups xmm14, xmmword ptr [rbp + 0x1d8]
0x141e470a2 movsd xmm9, qword ptr [rbp + 0x1e8]
0x141e470ab mov eax, dword ptr [rbp + 0x1f0]
0x141e470b1 mov dword ptr [rbp + 0x1c4], eax
0x141e470b7 movss xmm6, dword ptr [rbp + 0x1f4]
0x141e470bf mov r10d, dword ptr [rbp + 0x1f8]
0x141e470c6 movss xmm8, dword ptr [rbp + 0x1fc]
0x141e470cf movsd xmm10, qword ptr [rbp + 0x200]
0x141e470d8 movzx r11d, byte ptr [rbp + 0x202]
0x141e470e0 mov rax, qword ptr [rbp + 0x203]
0x141e470e7 mov qword ptr [rbp + 0x158], rax
0x141e470ee mov rax, qword ptr [rbp + 0x208]
0x141e470f5 mov qword ptr [rbp + 0x15d], rax
0x141e470fc jmp 0x141e473a3
0x141e47101 mov rdx, qword ptr [rbp + 0x1b8]
0x141e47108 mov rax, rdx
0x141e4710b neg rax
0x141e4710e jno 0x141e4713a
0x141e47110 lea rax, [rip + 0x1147c19]
0x141e47117 movq xmm13, rax
0x141e4711c movaps xmm11, xmmword ptr [rip + 0xb8320c]
0x141e47124 mov r14b, 0x2c
0x141e47127 mov r9, qword ptr [rbp + 0x198]
0x141e4712e movzx ebx, byte ptr [rbp + 0x217]
0x141e47135 jmp 0x141e474e7
0x141e4713a xor eax, eax
0x141e4713c mov ecx, dword ptr [rbp + 0x190]
0x141e47142 cmp ecx, 2
0x141e47145 cmove ecx, eax
0x141e47148 mov dword ptr [rbp + 0x190], ecx
0x141e4714e movzx ecx, byte ptr [rbp + 0x1c3]
0x141e47155 cmp cl, 6
0x141e47158 movzx ecx, cl
0x141e4715b mov edi, 5
0x141e47160 cmovne edi, ecx
0x141e47163 movzx ecx, byte ptr [rbp + 0x1c2]
0x141e4716a cmp cl, 5
0x141e4716d movzx ecx, cl
0x141e47170 mov r14d, 4
0x141e47176 cmovne r14d, ecx
0x141e4717a mov ecx, dword ptr [rbp + 0x18c]
0x141e47180 cmp ecx, 2
0x141e47183 cmove ecx, eax
0x141e47186 mov dword ptr [rbp + 0x18c], ecx
0x141e4718c mov r13d, dword ptr [rbp + 0x184]
0x141e47193 cmp r13d, 2
0x141e47197 cmove r13d, eax
0x141e4719b movzx eax, byte ptr [rbp + 0x1ae]
0x141e471a2 cmp al, 0xa
0x141e471a4 movzx eax, al
0x141e471a7 mov ebx, 9
0x141e471ac cmovne ebx, eax
0x141e471af mov rdx, qword ptr [rbp + 0x38]
0x141e471b3 test byte ptr [rdx], 1
0x141e471b6 movzx r12d, byte ptr [rbp + 0x217]
0x141e471be je 0x141e471d6
0x141e471c0 mov rax, qword ptr [rdx + 8]
0x141e471c4 inc rax
0x141e471c7 mov rcx, 0xffffffffffffffff
0x141e471ce cmovne rcx, rax
0x141e471d2 mov qword ptr [rdx + 8], rcx
0x141e471d6 lea rcx, [rbp + 0x1c8]
0x141e471dd mov rdx, qword ptr [rbp + 0x68]
0x141e471e1 call 0x140bca140
0x141e471e6 movaps xmm6, xmmword ptr [rbp + 0x40]
0x141e471ea movaps xmm7, xmmword ptr [rbp + 0x50]
0x141e471ee cmp byte ptr [rbp + 0x1c8], 0x32
0x141e471f5 jne 0x141e473ce
0x141e471fb test r12b, 1
0x141e471ff jne 0x141e47217
0x141e47201 mov rcx, qword ptr [rbp + 0x68]
0x141e47205 mov edx, 0x29
0x141e4720a call 0x140bc41b0
0x141e4720f test al, al
0x141e47211 je 0x141e476a3
0x141e47217 mov rax, qword ptr [rbp + 0x1b8]
0x141e4721e mov qword ptr [rsi + 8], rax
0x141e47222 movaps xmm0, xmmword ptr [rbp + 0x70]
0x141e47226 movups xmmword ptr [rsi + 0x10], xmm0
0x141e4722a mov eax, dword ptr [rbp + 0x190]
0x141e47230 mov dword ptr [rsi + 0x20], eax
0x141e47233 mov eax, dword ptr [rbp + 0x114]
0x141e47239 mov dword ptr [rsi + 0x24], eax
0x141e4723c mov eax, dword ptr [rbp + 0x18c]
0x141e47242 mov dword ptr [rsi + 0x28], eax
0x141e47245 movss dword ptr [rsi + 0x2c], xmm7
0x141e4724a mov dword ptr [rsi + 0x30], r13d
0x141e4724e movss dword ptr [rsi + 0x34], xmm6
0x141e47253 mov byte ptr [rsi + 0x38], r14b
0x141e47257 mov byte ptr [rsi + 0x39], dil
0x141e4725b mov byte ptr [rsi + 0x3a], bl
0x141e4725e mov eax, dword ptr [rbp + 0x1a0]
0x141e47264 mov dword ptr [rsi + 0x3b], eax
0x141e47267 movzx eax, byte ptr [rbp + 0x1a4]
0x141e4726e mov byte ptr [rsi + 0x3f], al
0x141e47271 mov byte ptr [rsi], 0x32
0x141e47274 jmp 0x141e4763e
0x141e47279 mov r14b, 0x13
0x141e4727c jmp 0x141e47433
0x141e47281 movzx r8d, word ptr [rbp + 0x1ca]
0x141e47289 mov r12d, dword ptr [rbp + 0x1cc]
0x141e47290 mov rax, qword ptr [rbp + 0x1d0]
0x141e47297 movups xmm11, xmmword ptr [rbp + 0x1d8]
0x141e4729f movsd xmm9, qword ptr [rbp + 0x1e8]
0x141e472a8 mov ecx, dword ptr [rbp + 0x1f0]
0x141e472ae mov dword ptr [rbp + 0x1c4], ecx
0x141e472b4 movss xmm6, dword ptr [rbp + 0x1f4]
0x141e472bc mov r10d, dword ptr [rbp + 0x1f8]
0x141e472c3 movss xmm8, dword ptr [rbp + 0x1fc]
0x141e472cc movsd xmm10, qword ptr [rbp + 0x200]
0x141e472d5 movzx r11d, byte ptr [rbp + 0x202]
0x141e472dd mov rcx, qword ptr [rbp + 0x203]
0x141e472e4 mov qword ptr [rbp + 0x118], rcx
0x141e472eb mov rcx, qword ptr [rbp + 0x208]
0x141e472f2 mov qword ptr [rbp + 0x11d], rcx
0x141e472f9 jmp 0x141e47433
0x141e472fe movzx r8d, word ptr [rbp + 0x1ca]
0x141e47306 mov r12d, dword ptr [rbp + 0x1cc]
0x141e4730d mov rax, qword ptr [rbp + 0x1d0]
0x141e47314 movups xmm11, xmmword ptr [rbp + 0x1d8]
0x141e4731c movsd xmm9, qword ptr [rbp + 0x1e8]
0x141e47325 mov edx, dword ptr [rbp + 0x1f0]
0x141e4732b mov dword ptr [rbp + 0x1c4], edx
0x141e47331 movss xmm6, dword ptr [rbp + 0x1f4]
0x141e47339 mov r10d, dword ptr [rbp + 0x1f8]
0x141e47340 movss xmm8, dword ptr [rbp + 0x1fc]
0x141e47349 movsd xmm10, qword ptr [rbp + 0x200]
0x141e47352 movzx r11d, byte ptr [rbp + 0x202]
0x141e4735a mov rdx, qword ptr [rbp + 0x203]
0x141e47361 mov qword ptr [rbp + 0x118], rdx
0x141e47368 mov rdx, qword ptr [rbp + 0x208]
0x141e4736f mov qword ptr [rbp + 0x11d], rdx
0x141e47376 mov edi, ecx
0x141e47378 jmp 0x141e4743a
0x141e4737d mov r14b, 0x31
0x141e47380 movzx edi, byte ptr [rbp + 0x1ad]
0x141e47387 mov r8d, r12d
0x141e4738a mov r10d, ebx
0x141e4738d movzx r11d, byte ptr [rbp + 0x188]
0x141e47395 mov rbx, qword ptr [rbp + 0xc0]
0x141e4739c mov r12d, dword ptr [rbp + 0x180]
0x141e473a3 movq xmm13, rbx
0x141e473a8 mov rax, qword ptr [rbp + 0x158]
0x141e473af mov rcx, qword ptr [rbp + 0x15d]
0x141e473b6 mov qword ptr [rbp + 0x1a5], rcx
0x141e473bd mov qword ptr [rbp + 0x1a0], rax
0x141e473c4 movdqa xmm11, xmm14
0x141e473c9 jmp 0x141e4745b
0x141e473ce mov rax, qword ptr [rbp + 0x208]
0x141e473d5 mov qword ptr [rsi + 0x40], rax
0x141e473d9 movdqu xmm0, xmmword ptr [rbp + 0x1c8]
0x141e473e1 movups xmm1, xmmword ptr [rbp + 0x1d8]
0x141e473e8 movups xmm2, xmmword ptr [rbp + 0x1e8]
0x141e473ef movups xmm3, xmmword ptr [rbp + 0x1f8]
0x141e473f6 movups xmmword ptr [rsi + 0x30], xmm3
0x141e473fa movups xmmword ptr [rsi + 0x20], xmm2
0x141e473fe movups xmmword ptr [rsi + 0x10], xmm1
0x141e47402 movdqu xmmword ptr [rsi], xmm0
0x141e47406 mov rdx, qword ptr [rbp + 0x1b8]
0x141e4740d test rdx, rdx
0x141e47410 je 0x141e4763e
0x141e47416 movdqa xmm0, xmmword ptr [rbp + 0x70]
0x141e4741b movq rcx, xmm0
0x141e47420 mov r8d, 1
0x141e47426 call 0x140613c20
0x141e4742b jmp 0x141e4763e
0x141e47430 mov r14b, 0x31
0x141e47433 movzx edi, byte ptr [rbp + 0x1ae]
0x141e4743a movq xmm13, rax
0x141e4743f mov rax, qword ptr [rbp + 0x118]
0x141e47446 mov rcx, qword ptr [rbp + 0x11d]
0x141e4744d mov qword ptr [rbp + 0x1a5], rcx
0x141e47454 mov qword ptr [rbp + 0x1a0], rax
0x141e4745b mov r9, qword ptr [rbp + 0x198]
0x141e47462 movzx ebx, byte ptr [rbp + 0x217]
0x141e47469 movd ecx, xmm10
0x141e4746e mov r15d, ecx
0x141e47471 shr r15d, 8
0x141e47475 mov rdx, qword ptr [rbp + 0x1b8]
0x141e4747c mov rax, rdx
0x141e4747f neg rax
0x141e47482 jo 0x141e474e7
0x141e47484 test rdx, rdx
0x141e47487 je 0x141e474e7
0x141e47489 movdqa xmm0, xmmword ptr [rbp + 0x70]
0x141e4748e mov dword ptr [rbp + 0x188], ecx
0x141e47494 movq rcx, xmm0
0x141e47499 mov word ptr [rbp + 0x1ae], r8w
0x141e474a1 mov r8d, 1
0x141e474a7 mov r13d, r11d
0x141e474aa mov dword ptr [rbp + 0x1b8], r12d
0x141e474b1 mov rbx, rsi
0x141e474b4 mov esi, r10d
0x141e474b7 mov r12, r9
0x141e474ba call 0x140613c20
0x141e474bf mov ecx, dword ptr [rbp + 0x188]
0x141e474c5 movzx r8d, word ptr [rbp + 0x1ae]
0x141e474cd mov r9, r12
0x141e474d0 mov r10d, esi
0x141e474d3 mov rsi, rbx
0x141e474d6 movzx ebx, byte ptr [rbp + 0x217]
0x141e474dd mov r12d, dword ptr [rbp + 0x1b8]
0x141e474e4 mov r11d, r13d
0x141e474e7 mov byte ptr [rbp + 0xc8], r14b
0x141e474ee mov byte ptr [rbp + 0xc9], dil
0x141e474f5 mov word ptr [rbp + 0xca], r8w
0x141e474fd mov dword ptr [rbp + 0xcc], r12d
0x141e47504 movq qword ptr [rbp + 0xd0], xmm13
0x141e4750d movdqu xmmword ptr [rbp + 0xd8], xmm11
0x141e47516 movlps qword ptr [rbp + 0xe8], xmm9
0x141e4751e mov eax, dword ptr [rbp + 0x1c4]
0x141e47524 mov dword ptr [rbp + 0xf0], eax
0x141e4752a movss dword ptr [rbp + 0xf4], xmm6
0x141e47532 mov dword ptr [rbp + 0xf8], r10d
0x141e47539 movss dword ptr [rbp + 0xfc], xmm8
0x141e47542 mov byte ptr [rbp + 0x100], cl
0x141e47548 mov byte ptr [rbp + 0x101], r15b
0x141e4754f mov byte ptr [rbp + 0x102], r11b
0x141e47556 mov rax, qword ptr [rbp + 0x1a0]
0x141e4755d mov rcx, qword ptr [rbp + 0x1a5]
0x141e47564 mov qword ptr [rbp + 0x108], rcx
0x141e4756b xor ecx, ecx
0x141e4756d test bl, 1
0x141e47570 mov r8, qword ptr [rbp + 0x18]
0x141e47574 cmovne r8, rcx
0x141e47578 mov qword ptr [rbp + 0x103], rax
0x141e4757f test r9, r9
0x141e47582 cmove r8, r9
0x141e47586 lea rcx, [rbp + 0x1c8]
0x141e4758d lea rdx, [rbp + 0xc8]
0x141e47594 call 0x140bcd9f0
0x141e47599 movzx eax, byte ptr [rbp + 0x1c8]
0x141e475a0 mov ecx, dword ptr [rbp + 0x1cc]
0x141e475a6 mov dword ptr [rsi + 4], ecx
0x141e475a9 mov ecx, dword ptr [rbp + 0x1c9]
0x141e475af mov dword ptr [rsi + 1], ecx
0x141e475b2 mov rcx, qword ptr [rbp + 0x1d0]
0x141e475b9 mov edx, dword ptr [rbp + 0x1f0]
0x141e475bf movd xmm0, dword ptr [rbp + 0x1f4]
0x141e475c7 mov r8d, dword ptr [rbp + 0x1f8]
0x141e475ce movss xmm1, dword ptr [rbp + 0x1fc]
0x141e475d6 movzx r9d, word ptr [rbp + 0x200]
0x141e475de movzx r10d, byte ptr [rbp + 0x202]
0x141e475e6 movzx r11d, byte ptr [rbp + 0x207]
0x141e475ee mov byte ptr [rsi + 0x3f], r11b
0x141e475f2 mov r11d, dword ptr [rbp + 0x203]
0x141e475f9 mov dword ptr [rsi + 0x3b], r11d
0x141e475fd mov r11, qword ptr [rbp + 0x208]
0x141e47604 mov rdi, qword ptr [rbp + 0x1e8]
0x141e4760b movups xmm2, xmmword ptr [rbp + 0x1d8]
0x141e47612 mov byte ptr [rsi], al
0x141e47614 mov qword ptr [rsi + 8], rcx
0x141e47618 movups xmmword ptr [rsi + 0x10], xmm2
0x141e4761c mov qword ptr [rsi + 0x20], rdi
0x141e47620 mov dword ptr [rsi + 0x28], edx
0x141e47623 movd dword ptr [rsi + 0x2c], xmm0
0x141e47628 mov dword ptr [rsi + 0x30], r8d
0x141e4762c movss dword ptr [rsi + 0x34], xmm1
0x141e47631 mov word ptr [rsi + 0x38], r9w
0x141e47636 mov byte ptr [rsi + 0x3a], r10b
0x141e4763a mov qword ptr [rsi + 0x40], r11
0x141e4763e mov rax, rsi
0x141e47641 movaps xmm6, xmmword ptr [rbp + 0x220]
0x141e47648 movaps xmm7, xmmword ptr [rbp + 0x230]
0x141e4764f movaps xmm8, xmmword ptr [rbp + 0x240]
0x141e47657 movaps xmm9, xmmword ptr [rbp + 0x250]
0x141e4765f movaps xmm10, xmmword ptr [rbp + 0x260]
0x141e47667 movaps xmm11, xmmword ptr [rbp + 0x270]
0x141e4766f movaps xmm12, xmmword ptr [rbp + 0x280]
0x141e47677 movaps xmm13, xmmword ptr [rbp + 0x290]
0x141e4767f movaps xmm14, xmmword ptr [rbp + 0x2a0]
0x141e47687 movaps xmm15, xmmword ptr [rbp + 0x2b0]
0x141e4768f add rsp, 0x348
0x141e47696 pop rbx
0x141e47697 pop rdi
0x141e47698 pop rsi
0x141e47699 pop r12
0x141e4769b pop r13
0x141e4769d pop r14
0x141e4769f pop r15
0x141e476a1 pop rbp
0x141e476a2 ret 
0x141e476a3 mov byte ptr [rsi], 0x18
0x141e476a6 mov rdx, qword ptr [rbp + 0x1b8]
0x141e476ad test rdx, rdx
0x141e476b0 jne 0x141e47416
0x141e476b6 jmp 0x141e4763e
0x141e476b8 movzx edi, byte ptr [rbp + 0x1c9]
0x141e476bf movzx r8d, word ptr [rbp + 0x1ca]
0x141e476c7 mov r12d, dword ptr [rbp + 0x1cc]
0x141e476ce movd xmm0, dword ptr [rbp + 0x1d0]
0x141e476d6 mov eax, dword ptr [rbp + 0x1d4]
0x141e476dc movups xmm11, xmmword ptr [rbp + 0x1d8]
0x141e476e4 movsd xmm9, qword ptr [rbp + 0x1e8]
0x141e476ed mov ecx, dword ptr [rbp + 0x1f0]
0x141e476f3 mov dword ptr [rbp + 0x1c4], ecx
0x141e476f9 movss xmm6, dword ptr [rbp + 0x1f4]
0x141e47701 mov r10d, dword ptr [rbp + 0x1f8]
0x141e47708 movss xmm8, dword ptr [rbp + 0x1fc]
0x141e47711 movsd xmm10, qword ptr [rbp + 0x200]
0x141e4771a movzx r11d, byte ptr [rbp + 0x202]
0x141e47722 mov rcx, qword ptr [rbp + 0x203]
0x141e47729 mov qword ptr [rbp + 0x138], rcx
0x141e47730 mov rcx, qword ptr [rbp + 0x208]
0x141e47737 mov qword ptr [rbp + 0x13d], rcx
0x141e4773e jmp 0x141e47c85
0x141e47743 movzx edi, byte ptr [rbp + 0x1c9]
0x141e4774a movzx r8d, word ptr [rbp + 0x1ca]
0x141e47752 mov r12d, dword ptr [rbp + 0x1cc]
0x141e47759 movd xmm0, dword ptr [rbp + 0x1d0]
0x141e47761 mov eax, dword ptr [rbp + 0x1d4]
0x141e47767 movups xmm11, xmmword ptr [rbp + 0x1d8]
0x141e4776f movsd xmm9, qword ptr [rbp + 0x1e8]
0x141e47778 mov ecx, dword ptr [rbp + 0x1f0]
0x141e4777e mov dword ptr [rbp + 0x1c4], ecx
0x141e47784 movss xmm6, dword ptr [rbp + 0x1f4]
0x141e4778c mov r10d, dword ptr [rbp + 0x1f8]
0x141e47793 movss xmm8, dword ptr [rbp + 0x1fc]
0x141e4779c movsd xmm10, qword ptr [rbp + 0x200]
0x141e477a5 movzx r11d, byte ptr [rbp + 0x202]
0x141e477ad mov rcx, qword ptr [rbp + 0x203]
0x141e477b4 mov qword ptr [rbp + 0x128], rcx
0x141e477bb mov rcx, qword ptr [rbp + 0x208]
0x141e477c2 mov qword ptr [rbp + 0x12d], rcx
0x141e477c9 jmp 0x141e47c55
0x141e477ce movzx edi, byte ptr [rbp + 0x1c9]
0x141e477d5 movzx r8d, word ptr [rbp + 0x1ca]
0x141e477dd mov r12d, dword ptr [rbp + 0x1cc]
0x141e477e4 movsd xmm13, qword ptr [rbp + 0x1d0]
0x141e477ed movups xmm11, xmmword ptr [rbp + 0x1d8]
0x141e477f5 movsd xmm9, qword ptr [rbp + 0x1e8]
0x141e477fe mov eax, dword ptr [rbp + 0x1f0]
0x141e47804 mov dword ptr [rbp + 0x1c4], eax
0x141e4780a movss xmm6, dword ptr [rbp + 0x1f4]
0x141e47812 mov r10d, dword ptr [rbp + 0x1f8]
0x141e47819 movss xmm8, dword ptr [rbp + 0x1fc]
0x141e47822 movsd xmm10, qword ptr [rbp + 0x200]
0x141e4782b movzx r11d, byte ptr [rbp + 0x202]
0x141e47833 mov rax, qword ptr [rbp + 0x203]
0x141e4783a mov qword ptr [rbp + 0x1a0], rax
0x141e47841 mov rax, qword ptr [rbp + 0x208]
0x141e47848 mov qword ptr [rbp + 0x1a5], rax
0x141e4784f jmp 0x141e4745b
0x141e47854 movzx edi, byte ptr [rbp + 0x1c9]
0x141e4785b movzx r8d, word ptr [rbp + 0x1ca]
0x141e47863 mov r12d, dword ptr [rbp + 0x1cc]
0x141e4786a mov rax, qword ptr [rbp + 0x1d0]
0x141e47871 movups xmm11, xmmword ptr [rbp + 0x1d8]
0x141e47879 movsd xmm9, qword ptr [rbp + 0x1e8]
0x141e47882 mov ecx, dword ptr [rbp + 0x1f0]
0x141e47888 mov dword ptr [rbp + 0x1c4], ecx
0x141e4788e movss xmm6, dword ptr [rbp + 0x1f4]
0x141e47896 mov r10d, dword ptr [rbp + 0x1f8]
0x141e4789d movss xmm8, dword ptr [rbp + 0x1fc]
0x141e478a6 movsd xmm10, qword ptr [rbp + 0x200]
0x141e478af movzx r11d, byte ptr [rbp + 0x202]
0x141e478b7 mov rcx, qword ptr [rbp + 0x203]
0x141e478be mov qword ptr [rbp + 0x1a0], rcx
0x141e478c5 mov rcx, qword ptr [rbp + 0x208]
0x141e478cc mov qword ptr [rbp + 0x1a5], rcx
0x141e478d3 jmp 0x141e47b84
0x141e478d8 movzx edi, byte ptr [rbp + 0x1c9]
0x141e478df movzx r8d, word ptr [rbp + 0x1ca]
0x141e478e7 mov r12d, dword ptr [rbp + 0x1cc]
0x141e478ee mov rax, qword ptr [rbp + 0x1d0]
0x141e478f5 movups xmm11, xmmword ptr [rbp + 0x1d8]
0x141e478fd movsd xmm9, qword ptr [rbp + 0x1e8]
0x141e47906 mov ecx, dword ptr [rbp + 0x1f0]
0x141e4790c mov dword ptr [rbp + 0x1c4], ecx
0x141e47912 movss xmm6, dword ptr [rbp + 0x1f4]
0x141e4791a mov r10d, dword ptr [rbp + 0x1f8]
0x141e47921 movss xmm8, dword ptr [rbp + 0x1fc]
0x141e4792a movsd xmm10, qword ptr [rbp + 0x200]
0x141e47933 movzx r11d, byte ptr [rbp + 0x202]
0x141e4793b mov rcx, qword ptr [rbp + 0x203]
0x141e47942 mov qword ptr [rbp + 0x148], rcx
0x141e47949 mov rcx, qword ptr [rbp + 0x208]
0x141e47950 mov qword ptr [rbp + 0x14d], rcx
0x141e47957 jmp 0x141e47cd3
0x141e4795c movzx edi, byte ptr [rbp + 0x1c9]
0x141e47963 movzx r8d, word ptr [rbp + 0x1ca]
0x141e4796b mov r12d, dword ptr [rbp + 0x1cc]
0x141e47972 movups xmm11, xmmword ptr [rbp + 0x1d8]
0x141e4797a movsd xmm9, qword ptr [rbp + 0x1e8]
0x141e47983 mov eax, dword ptr [rbp + 0x1f0]
0x141e47989 mov dword ptr [rbp + 0x1c4], eax
0x141e4798f movss xmm6, dword ptr [rbp + 0x1f4]
0x141e47997 mov r10d, dword ptr [rbp + 0x1f8]
0x141e4799e movss xmm8, dword ptr [rbp + 0x1fc]
0x141e479a7 movzx ecx, byte ptr [rbp + 0x200]
0x141e479ae movzx r15d, byte ptr [rbp + 0x201]
0x141e479b6 movzx r11d, byte ptr [rbp + 0x202]
0x141e479be mov rax, qword ptr [rbp + 0x203]
0x141e479c5 mov qword ptr [rbp + 0x1a0], rax
0x141e479cc mov rax, qword ptr [rbp + 0x208]
0x141e479d3 mov qword ptr [rbp + 0x1a5], rax
0x141e479da movq xmm13, qword ptr [rbp + 0x1d0]
0x141e479e3 jmp 0x141e47127
0x141e479e8 mov r14b, 0x13
0x141e479eb jmp 0x141e47c2e
0x141e479f0 mov r14b, 0x13
0x141e479f3 jmp 0x141e47c7a
0x141e479f8 mov r14b, 0x13
0x141e479fb jmp 0x141e47c4a
0x141e47a00 mov r14b, 0x13
0x141e47a03 jmp 0x141e47caa
0x141e47a08 mov r14b, 0x13
0x141e47a0b jmp 0x141e47ccc
0x141e47a10 movzx r8d, word ptr [rbp + 0x1ca]
0x141e47a18 mov r12d, dword ptr [rbp + 0x1cc]
0x141e47a1f mov rax, qword ptr [rbp + 0x1d0]
0x141e47a26 movups xmm11, xmmword ptr [rbp + 0x1d8]
0x141e47a2e movsd xmm9, qword ptr [rbp + 0x1e8]
0x141e47a37 mov ecx, dword ptr [rbp + 0x1f0]
0x141e47a3d mov dword ptr [rbp + 0x1c4], ecx
0x141e47a43 movss xmm6, dword ptr [rbp + 0x1f4]
0x141e47a4b mov r10d, dword ptr [rbp + 0x1f8]
0x141e47a52 movss xmm8, dword ptr [rbp + 0x1fc]
0x141e47a5b movsd xmm10, qword ptr [rbp + 0x200]
0x141e47a64 movzx r11d, byte ptr [rbp + 0x202]
0x141e47a6c mov rcx, qword ptr [rbp + 0x203]
0x141e47a73 mov qword ptr [rbp + 0x1a0], rcx
0x141e47a7a mov rcx, qword ptr [rbp + 0x208]
0x141e47a81 mov qword ptr [rbp + 0x1a5], rcx
0x141e47a88 jmp 0x141e47caa
0x141e47a8d movzx r8d, word ptr [rbp + 0x1ca]
0x141e47a95 mov r12d, dword ptr [rbp + 0x1cc]
0x141e47a9c mov rax, qword ptr [rbp + 0x1d0]
0x141e47aa3 movups xmm11, xmmword ptr [rbp + 0x1d8]
0x141e47aab movsd xmm9, qword ptr [rbp + 0x1e8]
0x141e47ab4 mov ecx, dword ptr [rbp + 0x1f0]
0x141e47aba mov dword ptr [rbp + 0x1c4], ecx
0x141e47ac0 movss xmm6, dword ptr [rbp + 0x1f4]
0x141e47ac8 mov r10d, dword ptr [rbp + 0x1f8]
0x141e47acf movss xmm8, dword ptr [rbp + 0x1fc]
0x141e47ad8 movsd xmm10, qword ptr [rbp + 0x200]
0x141e47ae1 movzx r11d, byte ptr [rbp + 0x202]
0x141e47ae9 mov rcx, qword ptr [rbp + 0x203]
0x141e47af0 mov qword ptr [rbp + 0x148], rcx
0x141e47af7 mov rcx, qword ptr [rbp + 0x208]
0x141e47afe mov qword ptr [rbp + 0x14d], rcx
0x141e47b05 jmp 0x141e47ccc
0x141e47b0a movzx r8d, word ptr [rbp + 0x1ca]
0x141e47b12 mov r12d, dword ptr [rbp + 0x1cc]
0x141e47b19 mov rax, qword ptr [rbp + 0x1d0]
0x141e47b20 movups xmm11, xmmword ptr [rbp + 0x1d8]
0x141e47b28 movsd xmm9, qword ptr [rbp + 0x1e8]
0x141e47b31 mov edx, dword ptr [rbp + 0x1f0]
0x141e47b37 mov dword ptr [rbp + 0x1c4], edx
0x141e47b3d movss xmm6, dword ptr [rbp + 0x1f4]
0x141e47b45 mov r10d, dword ptr [rbp + 0x1f8]
0x141e47b4c movss xmm8, dword ptr [rbp + 0x1fc]
0x141e47b55 movsd xmm10, qword ptr [rbp + 0x200]
0x141e47b5e movzx r11d, byte ptr [rbp + 0x202]
0x141e47b66 mov rdx, qword ptr [rbp + 0x203]
0x141e47b6d mov qword ptr [rbp + 0x1a0], rdx
0x141e47b74 mov rdx, qword ptr [rbp + 0x208]
0x141e47b7b mov qword ptr [rbp + 0x1a5], rdx
0x141e47b82 mov edi, ecx
0x141e47b84 movzx ebx, byte ptr [rbp + 0x217]
0x141e47b8b jmp 0x141e47cb8
0x141e47b90 movzx r8d, word ptr [rbp + 0x1ca]
0x141e47b98 mov r12d, dword ptr [rbp + 0x1cc]
0x141e47b9f mov rax, qword ptr [rbp + 0x1d0]
0x141e47ba6 movups xmm11, xmmword ptr [rbp + 0x1d8]
0x141e47bae movsd xmm9, qword ptr [rbp + 0x1e8]
0x141e47bb7 mov edx, dword ptr [rbp + 0x1f0]
0x141e47bbd mov dword ptr [rbp + 0x1c4], edx
0x141e47bc3 movss xmm6, dword ptr [rbp + 0x1f4]
0x141e47bcb mov r10d, dword ptr [rbp + 0x1f8]
0x141e47bd2 movss xmm8, dword ptr [rbp + 0x1fc]
0x141e47bdb movsd xmm10, qword ptr [rbp + 0x200]
0x141e47be4 movzx r11d, byte ptr [rbp + 0x202]
0x141e47bec mov rdx, qword ptr [rbp + 0x203]
0x141e47bf3 mov qword ptr [rbp + 0x148], rdx
0x141e47bfa mov rdx, qword ptr [rbp + 0x208]
0x141e47c01 mov qword ptr [rbp + 0x14d], rdx
0x141e47c08 mov edi, ecx
0x141e47c0a jmp 0x141e47cd3
0x141e47c0f lea rax, [rip + 0x1147139]
0x141e47c16 movq xmm13, rax
0x141e47c1b mov r14b, 0x2d
0x141e47c1e movaps xmm11, xmmword ptr [rip + 0xb8273a]
0x141e47c26 jmp 0x141e4745b
0x141e47c2b mov r14b, 0x31
0x141e47c2e mov r9, qword ptr [rbp + 0x198]
0x141e47c35 movzx ebx, byte ptr [rbp + 0x217]
0x141e47c3c mov r12d, 1
0x141e47c42 jmp 0x141e47469
0x141e47c47 mov r14b, 0x31
0x141e47c4a movdqa xmm0, xmmword ptr [rbp + 0x40]
0x141e47c4f mov r12d, 1
0x141e47c55 movd xmm13, eax
0x141e47c5a punpcklqdq xmm13, xmm0
0x141e47c5f shufps xmm13, xmm0, 0xe2
0x141e47c64 mov rax, qword ptr [rbp + 0x128]
0x141e47c6b mov rcx, qword ptr [rbp + 0x12d]
0x141e47c72 jmp 0x141e4744d
0x141e47c77 mov r14b, 0x31
0x141e47c7a movdqa xmm0, xmmword ptr [rbp + 0x50]
0x141e47c7f mov r12d, 1
0x141e47c85 movd xmm13, eax
0x141e47c8a punpcklqdq xmm13, xmm0
0x141e47c8f shufps xmm13, xmm0, 0xe2
0x141e47c94 mov rax, qword ptr [rbp + 0x138]
0x141e47c9b mov rcx, qword ptr [rbp + 0x13d]
0x141e47ca2 jmp 0x141e4744d
0x141e47ca7 mov r14b, 0x31
0x141e47caa movzx ebx, byte ptr [rbp + 0x217]
0x141e47cb1 movzx edi, byte ptr [rbp + 0x1c3]
0x141e47cb8 movq xmm13, rax
0x141e47cbd mov r9, qword ptr [rbp + 0x198]
0x141e47cc4 jmp 0x141e47469
0x141e47cc9 mov r14b, 0x31
0x141e47ccc movzx edi, byte ptr [rbp + 0x1c2]
0x141e47cd3 movq xmm13, rax
0x141e47cd8 mov rax, qword ptr [rbp + 0x148]
0x141e47cdf mov rcx, qword ptr [rbp + 0x14d]
0x141e47ce6 jmp 0x141e4744d
0x141e47ceb lea rax, [rip + 0x114704c]
0x141e47cf2 jmp 0x141e47d78
0x141e47cf7 lea rax, [rip + 0x1147032]
0x141e47cfe movq xmm13, rax
0x141e47d03 mov r14b, 0x2d
0x141e47d06 mov r9, qword ptr [rbp + 0x198]
0x141e47d0d movzx ebx, byte ptr [rbp + 0x217]
0x141e47d14 test rdx, rdx
0x141e47d17 jne 0x141e47489
0x141e47d1d jmp 0x141e474e7
0x141e47d22 lea rax, [rip + 0x1147010]
0x141e47d29 jmp 0x141e47d32
0x141e47d2b lea rax, [rip + 0x1147002]
0x141e47d32 movq xmm13, rax
0x141e47d37 mov r14b, 0x2d
0x141e47d3a mov r9, qword ptr [rbp + 0x198]
0x141e47d41 movzx ebx, byte ptr [rbp + 0x217]
0x141e47d48 movaps xmm11, xmmword ptr [rip + 0xb82650]
0x141e47d50 jmp 0x141e47469
0x141e47d55 lea rax, [rip + 0x1146fea]
0x141e47d5c movq xmm13, rax
0x141e47d61 mov r14b, 0x2d
0x141e47d64 movaps xmm11, xmmword ptr [rip + 0xb826b4]
0x141e47d6c jmp 0x141e4745b
0x141e47d71 lea rax, [rip + 0x1146fca]
0x141e47d78 movq xmm13, rax
0x141e47d7d mov r14b, 0x2d
0x141e47d80 jmp 0x141e4745b
0x141e47d85 ud2 
0x141e47d87 nop word ptr [rax + rax]
0x141e47d90 mov qword ptr [rsp + 0x10], rdx
0x141e47d95 push rbp
0x141e47d96 push r15
0x141e47d98 push r14
0x141e47d9a push r13
0x141e47d9c push r12
0x141e47d9e push rsi
0x141e47d9f push rdi
0x141e47da0 push rbx
0x141e47da1 sub rsp, 0xd8
0x141e47da8 lea rbp, [rdx + 0x80]
0x141e47daf movdqa xmmword ptr [rsp + 0x30], xmm15
0x141e47db6 movdqa xmmword ptr [rsp + 0x40], xmm14
0x141e47dbd movdqa xmmword ptr [rsp + 0x50], xmm13
0x141e47dc4 movdqa xmmword ptr [rsp + 0x60], xmm12
0x141e47dcb movdqa xmmword ptr [rsp + 0x70], xmm11
0x141e47dd2 movdqa xmmword ptr [rsp + 0x80], xmm10
0x141e47ddc movaps xmmword ptr [rsp + 0x90], xmm9
0x141e47de5 movaps xmmword ptr [rsp + 0xa0], xmm8
0x141e47dee movaps xmmword ptr [rsp + 0xb0], xmm7
0x141e47df6 movaps xmmword ptr [rsp + 0xc0], xmm6
0x141e47dfe cmp qword ptr [rbp + 0x1b8], 0
0x141e47e06 je 0x141e47e24
0x141e47e08 movdqa xmm0, xmmword ptr [rbp + 0x70]
0x141e47e0d movq rcx, xmm0
0x141e47e12 mov r8d, 1
0x141e47e18 mov rdx, qword ptr [rbp + 0x1b8]
0x141e47e1f call 0x140613c20
0x141e47e24 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x141e47e2c movaps xmm7, xmmword ptr [rsp + 0xb0]
0x141e47e34 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x141e47e3d movaps xmm9, xmmword ptr [rsp + 0x90]
0x141e47e46 movaps xmm10, xmmword ptr [rsp + 0x80]
0x141e47e4f movaps xmm11, xmmword ptr [rsp + 0x70]
0x141e47e55 movaps xmm12, xmmword ptr [rsp + 0x60]
0x141e47e5b movaps xmm13, xmmword ptr [rsp + 0x50]
0x141e47e61 movaps xmm14, xmmword ptr [rsp + 0x40]
0x141e47e67 movaps xmm15, xmmword ptr [rsp + 0x30]
0x141e47e6d add rsp, 0xd8
0x141e47e74 pop rbx
0x141e47e75 pop rdi
0x141e47e76 pop rsi
0x141e47e77 pop r12
0x141e47e79 pop r13
0x141e47e7b pop r14
0x141e47e7d pop r15
0x141e47e7f pop rbp
0x141e47e80 ret 
0x141e47e81 nop word ptr cs:[rax + rax]
0x141e47e90 mov qword ptr [rsp + 0x10], rdx
0x141e47e95 push rbp
0x141e47e96 push r15
0x141e47e98 push r14
0x141e47e9a push r13
0x141e47e9c push r12
0x141e47e9e push rsi
0x141e47e9f push rdi
0x141e47ea0 push rbx
0x141e47ea1 sub rsp, 0xd8
0x141e47ea8 lea rbp, [rdx + 0x80]
0x141e47eaf movdqa xmmword ptr [rsp + 0x30], xmm15
0x141e47eb6 movdqa xmmword ptr [rsp + 0x40], xmm14
0x141e47ebd movdqa xmmword ptr [rsp + 0x50], xmm13
0x141e47ec4 movdqa xmmword ptr [rsp + 0x60], xmm12
0x141e47ecb movdqa xmmword ptr [rsp + 0x70], xmm11
0x141e47ed2 movdqa xmmword ptr [rsp + 0x80], xmm10
0x141e47edc movaps xmmword ptr [rsp + 0x90], xmm9
0x141e47ee5 movaps xmmword ptr [rsp + 0xa0], xmm8
0x141e47eee movdqa xmmword ptr [rsp + 0xb0], xmm7
0x141e47ef7 movaps xmmword ptr [rsp + 0xc0], xmm6
0x141e47eff mov rdx, qword ptr [rbp + 0x1b0]
0x141e47f06 mov rax, rdx
0x141e47f09 neg rax
0x141e47f0c jo 0x141e47f25
0x141e47f0e jae 0x141e47f25
0x141e47f10 movdqa xmm0, xmmword ptr [rbp + 0x70]
0x141e47f15 movq rcx, xmm0
0x141e47f1a mov r8d, 1
0x141e47f20 call 0x140613c20
0x141e47f25 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x141e47f2d movaps xmm7, xmmword ptr [rsp + 0xb0]
0x141e47f35 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x141e47f3e movaps xmm9, xmmword ptr [rsp + 0x90]
0x141e47f47 movaps xmm10, xmmword ptr [rsp + 0x80]
0x141e47f50 movaps xmm11, xmmword ptr [rsp + 0x70]
0x141e47f56 movaps xmm12, xmmword ptr [rsp + 0x60]
0x141e47f5c movaps xmm13, xmmword ptr [rsp + 0x50]
0x141e47f62 movaps xmm14, xmmword ptr [rsp + 0x40]
0x141e47f68 movaps xmm15, xmmword ptr [rsp + 0x30]
0x141e47f6e add rsp, 0xd8
0x141e47f75 pop rbx
0x141e47f76 pop rdi
0x141e47f77 pop rsi
0x141e47f78 pop r12
0x141e47f7a pop r13
0x141e47f7c pop r14
0x141e47f7e pop r15
0x141e47f80 pop rbp
0x141e47f81 ret 