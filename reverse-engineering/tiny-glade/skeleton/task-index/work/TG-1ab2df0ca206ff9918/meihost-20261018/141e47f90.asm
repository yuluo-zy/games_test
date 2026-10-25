0x141e47f90 push rbp
0x141e47f91 push r15
0x141e47f93 push r14
0x141e47f95 push r13
0x141e47f97 push r12
0x141e47f99 push rsi
0x141e47f9a push rdi
0x141e47f9b push rbx
0x141e47f9c sub rsp, 0x388
0x141e47fa3 lea rbp, [rsp + 0x80]
0x141e47fab movaps xmmword ptr [rbp + 0x2f0], xmm15
0x141e47fb3 movdqa xmmword ptr [rbp + 0x2e0], xmm14
0x141e47fbc movaps xmmword ptr [rbp + 0x2d0], xmm13
0x141e47fc4 movaps xmmword ptr [rbp + 0x2c0], xmm12
0x141e47fcc movdqa xmmword ptr [rbp + 0x2b0], xmm11
0x141e47fd5 movaps xmmword ptr [rbp + 0x2a0], xmm10
0x141e47fdd movdqa xmmword ptr [rbp + 0x290], xmm9
0x141e47fe6 movaps xmmword ptr [rbp + 0x280], xmm8
0x141e47fee movaps xmmword ptr [rbp + 0x270], xmm7
0x141e47ff5 movdqa xmmword ptr [rbp + 0x260], xmm6
0x141e47ffd mov qword ptr [rbp + 0x258], 0xfffffffffffffffe
0x141e48008 mov rdi, r9
0x141e4800b mov rbx, r8
0x141e4800e mov r15, rdx
0x141e48011 mov rsi, rcx
0x141e48014 cmp byte ptr [rdx + 0x50], 0
0x141e48018 jne 0x141e4803f
0x141e4801a lea rdx, [r15 + 0x10]
0x141e4801e lea rcx, [rbp + 0x1d8]
0x141e48025 mov r8, rbx
0x141e48028 mov r9, rdi
0x141e4802b call 0x140bc8c90
0x141e48030 movzx eax, byte ptr [rbp + 0x1d8]
0x141e48037 cmp al, 0x32
0x141e48039 jne 0x141e48dee
0x141e4803f lea r14, [r15 + 0x10]
0x141e48043 lea rcx, [rbp + 0x1d8]
0x141e4804a mov rdx, r14
0x141e4804d call 0x140bca140
0x141e48052 cmp byte ptr [rbp + 0x1d8], 0x32
0x141e48059 jne 0x141e48dae
0x141e4805f movzx ecx, byte ptr [r15 + 0x50]
0x141e48064 test cl, cl
0x141e48066 jne 0x141e48082
0x141e48068 mov rcx, r14
0x141e4806b mov edx, 0x28
0x141e48070 call 0x140bc41b0
0x141e48075 test al, al
0x141e48077 je 0x141e48e39
0x141e4807d movzx ecx, byte ptr [r15 + 0x50]
0x141e48082 mov byte ptr [r15 + 0x50], 0
0x141e48087 test byte ptr [r15], 1
0x141e4808b je 0x141e480a1
0x141e4808d mov rax, qword ptr [r15 + 8]
0x141e48091 test rax, rax
0x141e48094 je 0x141e48e53
0x141e4809a dec rax
0x141e4809d mov qword ptr [r15 + 8], rax
0x141e480a1 mov qword ptr [rbp + 0xd0], rsi
0x141e480a8 mov qword ptr [rbp + 0xc0], r14
0x141e480af mov qword ptr [rbp + 0x90], rbx
0x141e480b6 mov qword ptr [rbp + 0x88], rdi
0x141e480bd mov byte ptr [rbp + 0x227], cl
0x141e480c3 mov qword ptr [rbp + 0xd8], r15
0x141e480ca mov qword ptr [rbp + 0x180], r15
0x141e480d1 mov word ptr [rbp + 0x188], 0x103
0x141e480da movabs rbx, 0x8000000000000000
0x141e480e4 mov qword ptr [rbp + 0x168], rbx
0x141e480eb movabs rcx, 0x8000000000000001
0x141e480f5 lea r13, [rbp + 0x1d8]
0x141e480fc movdqa xmm6, xmmword ptr [rip + 0xb8222c]
0x141e48104 lea rax, [rip + 0x1146c4a]
0x141e4810b mov qword ptr [rbp + 0x1b0], rax
0x141e48112 movabs r15, 0x7ffffffffffffffe
0x141e4811c mov rdx, rbx
0x141e4811f mov dword ptr [rbp + 0x1cc], eax
0x141e48125 mov qword ptr [rbp + 0x240], rcx
0x141e4812c mov qword ptr [rbp + 0x1c0], rdx
0x141e48133 jmp 0x141e48165
0x141e48135 nop word ptr cs:[rax + rax]
0x141e48140 mov qword ptr [rbp + 0x1a8], rsi
0x141e48147 mov qword ptr [rbp + 0x168], rsi
0x141e4814e movdqu xmmword ptr [rbp + 0x170], xmm9
0x141e48157 mov rdx, qword ptr [rbp + 0x1c0]
0x141e4815e mov rcx, qword ptr [rbp + 0x240]
0x141e48165 mov rbx, rcx
0x141e48168 mov qword ptr [rbp + 0x238], rcx
0x141e4816f mov rdi, rdx
0x141e48172 mov qword ptr [rbp + 0x230], rdx
0x141e48179 mov byte ptr [rbp + 0x257], 1
0x141e48180 mov byte ptr [rbp + 0x256], 1
0x141e48187 mov byte ptr [rbp + 0x255], 1
0x141e4818e mov rcx, r13
0x141e48191 lea rdx, [rbp + 0x180]
0x141e48198 call 0x140bcd950
0x141e4819d movzx r12d, byte ptr [rbp + 0x1d8]
0x141e481a5 movzx esi, byte ptr [rbp + 0x1d9]
0x141e481ac cmp r12b, 0x32
0x141e481b0 jne 0x141e48f90
0x141e481b6 test sil, 1
0x141e481ba je 0x141e48e5c
0x141e481c0 mov r9, rbx
0x141e481c3 movzx ecx, byte ptr [rbp + 0x188]
0x141e481ca mov r14, qword ptr [rbp + 0x180]
0x141e481d1 movzx eax, byte ptr [r14]
0x141e481d5 cmp ecx, 1
0x141e481d8 je 0x141e48320
0x141e481de cmp ecx, 3
0x141e481e1 lea rsi, [rbp + 0x1d8]
0x141e481e8 jne 0x141e48450
0x141e481ee test al, 1
0x141e481f0 je 0x141e48206
0x141e481f2 mov rax, qword ptr [r14 + 8]
0x141e481f6 test rax, rax
0x141e481f9 je 0x141e49172
0x141e481ff dec rax
0x141e48202 mov qword ptr [r14 + 8], rax
0x141e48206 mov qword ptr [rbp + 0x110], r14
0x141e4820d mov byte ptr [rbp + 0x118], 0
0x141e48214 mov qword ptr [rbp + 0x238], r9
0x141e4821b mov qword ptr [rbp + 0x230], rdi
0x141e48222 mov byte ptr [rbp + 0x257], 1
0x141e48229 mov byte ptr [rbp + 0x256], 1
0x141e48230 mov byte ptr [rbp + 0x255], 1
0x141e48237 mov rcx, rsi
0x141e4823a lea rdx, [rbp + 0x110]
0x141e48241 call 0x142373a70
0x141e48246 movzx r12d, byte ptr [rbp + 0x1d8]
0x141e4824e cmp r12b, 0x32
0x141e48252 je 0x141e482aa
0x141e48254 lea rcx, [rbp + 0x1da]
0x141e4825b movzx eax, word ptr [rcx + 4]
0x141e4825f mov word ptr [rbp + 0x22c], ax
0x141e48266 mov eax, dword ptr [rcx]
0x141e48268 mov dword ptr [rbp + 0x228], eax
0x141e4826e mov rax, qword ptr [rbp + 0x1e0]
0x141e48275 mov qword ptr [rbp + 0xb0], rax
0x141e4827c movups xmm0, xmmword ptr [rbp + 0x1e8]
0x141e48283 movaps xmmword ptr [rbp - 0x50], xmm0
0x141e48287 movdqu xmm0, xmmword ptr [rbp + 0x1f8]
0x141e4828f movdqa xmmword ptr [rbp - 0x30], xmm0
0x141e48294 movups xmm15, xmmword ptr [rbp + 0x208]
0x141e4829c mov rax, qword ptr [rbp + 0x218]
0x141e482a3 mov qword ptr [rbp + 0xa8], rax
0x141e482aa mov r13, qword ptr [rbp + 0x180]
0x141e482b1 cmp dword ptr [r13], 1
0x141e482b6 mov r9, rbx
0x141e482b9 jne 0x141e482d1
0x141e482bb mov rax, qword ptr [r13 + 8]
0x141e482bf inc rax
0x141e482c2 mov rcx, 0xffffffffffffffff
0x141e482c9 cmove rax, rcx
0x141e482cd mov qword ptr [r13 + 8], rax
0x141e482d1 movzx r11d, byte ptr [rbp + 0x1d9]
0x141e482d9 movzx eax, word ptr [rbp + 0x22c]
0x141e482e0 mov word ptr [rbp + 0x1bc], ax
0x141e482e7 mov eax, dword ptr [rbp + 0x228]
0x141e482ed mov dword ptr [rbp + 0x1b8], eax
0x141e482f3 mov rax, qword ptr [rbp + 0xa8]
0x141e482fa mov r10, qword ptr [rbp + 0xb0]
0x141e48301 movdqa xmm11, xmmword ptr [rbp - 0x50]
0x141e48307 movaps xmm7, xmmword ptr [rbp - 0x30]
0x141e4830b movaps xmm8, xmm15
0x141e4830f jmp 0x141e485e5
0x141e48314 nop word ptr cs:[rax + rax]
0x141e48320 test al, 1
0x141e48322 lea rsi, [rbp + 0x1d8]
0x141e48329 je 0x141e4833f
0x141e4832b mov rax, qword ptr [r14 + 8]
0x141e4832f test rax, rax
0x141e48332 je 0x141e49172
0x141e48338 dec rax
0x141e4833b mov qword ptr [r14 + 8], rax
0x141e4833f mov qword ptr [rbp + 0x110], r14
0x141e48346 mov byte ptr [rbp + 0x118], 1
0x141e4834d mov qword ptr [rbp + 0x238], r9
0x141e48354 mov qword ptr [rbp + 0x230], rdi
0x141e4835b mov byte ptr [rbp + 0x257], 1
0x141e48362 mov byte ptr [rbp + 0x256], 1
0x141e48369 mov byte ptr [rbp + 0x255], 1
0x141e48370 mov rcx, rsi
0x141e48373 lea rdx, [rbp + 0x110]
0x141e4837a call 0x142373a70
0x141e4837f movzx r12d, byte ptr [rbp + 0x1d8]
0x141e48387 cmp r12b, 0x32
0x141e4838b je 0x141e483e3
0x141e4838d lea rcx, [rbp + 0x1da]
0x141e48394 movzx eax, word ptr [rcx + 4]
0x141e48398 mov word ptr [rbp + 0x22c], ax
0x141e4839f mov eax, dword ptr [rcx]
0x141e483a1 mov dword ptr [rbp + 0x228], eax
0x141e483a7 mov rax, qword ptr [rbp + 0x1e0]
0x141e483ae mov qword ptr [rbp + 0xb8], rax
0x141e483b5 movups xmm0, xmmword ptr [rbp + 0x1e8]
0x141e483bc movaps xmmword ptr [rbp - 0x40], xmm0
0x141e483c0 movdqu xmm0, xmmword ptr [rbp + 0x1f8]
0x141e483c8 movdqa xmmword ptr [rbp - 0x20], xmm0
0x141e483cd movups xmm10, xmmword ptr [rbp + 0x208]
0x141e483d5 mov rax, qword ptr [rbp + 0x218]
0x141e483dc mov qword ptr [rbp + 0xa0], rax
0x141e483e3 mov r13, qword ptr [rbp + 0x180]
0x141e483ea cmp dword ptr [r13], 1
0x141e483ef mov r9, rbx
0x141e483f2 jne 0x141e4840a
0x141e483f4 mov rax, qword ptr [r13 + 8]
0x141e483f8 inc rax
0x141e483fb mov rcx, 0xffffffffffffffff
0x141e48402 cmove rax, rcx
0x141e48406 mov qword ptr [r13 + 8], rax
0x141e4840a movzx r11d, byte ptr [rbp + 0x1d9]
0x141e48412 movzx eax, word ptr [rbp + 0x22c]
0x141e48419 mov word ptr [rbp + 0x1bc], ax
0x141e48420 mov eax, dword ptr [rbp + 0x228]
0x141e48426 mov dword ptr [rbp + 0x1b8], eax
0x141e4842c mov rax, qword ptr [rbp + 0xa0]
0x141e48433 mov r10, qword ptr [rbp + 0xb8]
0x141e4843a movdqa xmm11, xmmword ptr [rbp - 0x40]
0x141e48440 movaps xmm7, xmmword ptr [rbp - 0x20]
0x141e48444 movaps xmm8, xmm10
0x141e48448 jmp 0x141e485e5
0x141e4844d nop dword ptr [rax]
0x141e48450 test al, 1
0x141e48452 je 0x141e48468
0x141e48454 mov rax, qword ptr [r14 + 8]
0x141e48458 test rax, rax
0x141e4845b je 0x141e49172
0x141e48461 dec rax
0x141e48464 mov qword ptr [r14 + 8], rax
0x141e48468 lea rdx, [r14 + 0x10]
0x141e4846c mov qword ptr [rbp + 0x238], r9
0x141e48473 mov qword ptr [rbp + 0x230], rdi
0x141e4847a mov byte ptr [rbp + 0x257], 1
0x141e48481 mov byte ptr [rbp + 0x256], 1
0x141e48488 mov byte ptr [rbp + 0x255], 1
0x141e4848f mov rcx, rsi
0x141e48492 call 0x140bc9b80
0x141e48497 movzx r12d, byte ptr [rbp + 0x1d8]
0x141e4849f cmp r12b, 0x32
0x141e484a3 jne 0x141e484ef
0x141e484a5 mov rdx, qword ptr [rbp + 0x1e0]
0x141e484ac mov r8, qword ptr [rbp + 0x1e8]
0x141e484b3 mov qword ptr [r14 + 0x40], rdx
0x141e484b7 mov qword ptr [r14 + 0x48], r8
0x141e484bb cmp r8, 4
0x141e484bf mov r9, rbx
0x141e484c2 jne 0x141e48867
0x141e484c8 mov eax, dword ptr [rdx]
0x141e484ca mov r12b, 0x32
0x141e484cd cmp eax, dword ptr [rip + 0xd6c4a2]
0x141e484d3 je 0x141e4888f
0x141e484d9 mov eax, dword ptr [rdx]
0x141e484db mov r11b, 2
0x141e484de cmp eax, dword ptr [rip + 0xd6c49b]
0x141e484e4 je 0x141e48892
0x141e484ea jmp 0x141e488bd
0x141e484ef lea rdx, [rbp + 0x1da]
0x141e484f6 mov eax, dword ptr [rdx - 1]
0x141e484f9 mov ecx, dword ptr [rdx + 2]
0x141e484fc lea r8, [rbp + 0x111]
0x141e48503 mov dword ptr [r8 + 3], ecx
0x141e48507 mov dword ptr [r8], eax
0x141e4850a mov r10, qword ptr [rbp + 0x1e0]
0x141e48511 mov rax, qword ptr [rbp + 0x1e8]
0x141e48518 movups xmm0, xmmword ptr [rdx + 0x16]
0x141e4851c movdqu xmm1, xmmword ptr [rdx + 0x26]
0x141e48521 movups xmm2, xmmword ptr [rdx + 0x36]
0x141e48525 movups xmmword ptr [r8 + 0x37], xmm2
0x141e4852a movdqu xmmword ptr [r8 + 0x27], xmm1
0x141e48530 movups xmmword ptr [r8 + 0x17], xmm0
0x141e48535 mov qword ptr [rbp + 0x118], r10
0x141e4853c mov r9, rbx
0x141e4853f movzx r11d, byte ptr [rbp + 0x111]
0x141e48547 mov ecx, dword ptr [r8 + 1]
0x141e4854b mov dword ptr [rbp + 0x1d8], ecx
0x141e48551 movzx ecx, word ptr [r8 + 5]
0x141e48556 mov word ptr [rbp + 0x1dc], cx
0x141e4855d movups xmm7, xmmword ptr [rbp + 0x130]
0x141e48564 movups xmm8, xmmword ptr [rbp + 0x140]
0x141e4856c mov rcx, qword ptr [rbp + 0x150]
0x141e48573 mov qword ptr [rbp + 0x98], rcx
0x141e4857a movq xmm11, rax
0x141e4857f movq xmm0, qword ptr [rbp + 0x128]
0x141e48587 punpcklqdq xmm11, xmm0
0x141e4858c mov r13, qword ptr [rbp + 0x180]
0x141e48593 cmp dword ptr [r13], 1
0x141e48598 jne 0x141e485b0
0x141e4859a mov rax, qword ptr [r13 + 8]
0x141e4859e inc rax
0x141e485a1 mov rcx, 0xffffffffffffffff
0x141e485a8 cmove rax, rcx
0x141e485ac mov qword ptr [r13 + 8], rax
0x141e485b0 movzx eax, word ptr [rbp + 0x1dc]
0x141e485b7 mov word ptr [rbp + 0x1bc], ax
0x141e485be mov eax, dword ptr [rbp + 0x1d8]
0x141e485c4 mov dword ptr [rbp + 0x1b8], eax
0x141e485ca mov rax, qword ptr [rbp + 0x98]
0x141e485d1 mov qword ptr [rbp + 0x158], r10
0x141e485d8 movdqa xmm14, xmm11
0x141e485dd movaps xmm12, xmm7
0x141e485e1 movaps xmm13, xmm8
0x141e485e5 cmp r12b, 0x32
0x141e485e9 mov qword ptr [rbp + 0x1d0], rax
0x141e485f0 jne 0x141e48fde
0x141e485f6 test r11b, r11b
0x141e485f9 mov qword ptr [rbp + 0x190], r10
0x141e48600 mov dword ptr [rbp + 0x1cc], r11d
0x141e48607 jne 0x141e4895d
0x141e4860d xor ecx, ecx
0x141e4860f cmp rcx, qword ptr [rbp + 0x168]
0x141e48616 jno 0x141e497a3
0x141e4861c lea r14, [r13 + 0x10]
0x141e48620 mov qword ptr [rbp + 0x238], r9
0x141e48627 mov qword ptr [rbp + 0x230], rdi
0x141e4862e mov byte ptr [rbp + 0x257], 1
0x141e48635 mov byte ptr [rbp + 0x256], 1
0x141e4863c mov byte ptr [rbp + 0x255], 1
0x141e48643 mov rcx, rsi
0x141e48646 mov rdx, r14
0x141e48649 call 0x140bca140
0x141e4864e movzx r12d, byte ptr [rbp + 0x1d8]
0x141e48656 cmp r12b, 0x32
0x141e4865a jne 0x141e48f35
0x141e48660 mov qword ptr [rbp + 0x238], rbx
0x141e48667 mov qword ptr [rbp + 0x230], rdi
0x141e4866e mov byte ptr [rbp + 0x257], 1
0x141e48675 mov byte ptr [rbp + 0x256], 1
0x141e4867c mov byte ptr [rbp + 0x255], 1
0x141e48683 mov rcx, r14
0x141e48686 mov edx, 0x3a
0x141e4868b call 0x140bc41b0
0x141e48690 test al, al
0x141e48692 je 0x141e4907c
0x141e48698 mov qword ptr [rbp + 0x238], rbx
0x141e4869f mov qword ptr [rbp + 0x230], rdi
0x141e486a6 mov byte ptr [rbp + 0x257], 1
0x141e486ad mov byte ptr [rbp + 0x256], 1
0x141e486b4 mov byte ptr [rbp + 0x255], 1
0x141e486bb mov rcx, rsi
0x141e486be mov rdx, r14
0x141e486c1 call 0x140bca140
0x141e486c6 movzx r12d, byte ptr [rbp + 0x1d8]
0x141e486ce cmp r12b, 0x32
0x141e486d2 jne 0x141e48f35
0x141e486d8 test byte ptr [r13], 1
0x141e486dd mov rcx, rbx
0x141e486e0 mov rdx, rdi
0x141e486e3 je 0x141e486f9
0x141e486e5 mov rax, qword ptr [r13 + 8]
0x141e486e9 test rax, rax
0x141e486ec je 0x141e4925b
0x141e486f2 dec rax
0x141e486f5 mov qword ptr [r13 + 8], rax
0x141e486f9 mov qword ptr [rbp + 0x238], rcx
0x141e48700 mov qword ptr [rbp + 0x230], rdx
0x141e48707 mov byte ptr [rbp + 0x257], 1
0x141e4870e mov byte ptr [rbp + 0x256], 1
0x141e48715 mov byte ptr [rbp + 0x255], 1
0x141e4871c mov rcx, rsi
0x141e4871f mov rdx, r13
0x141e48722 call 0x141e42650
0x141e48727 movzx r12d, byte ptr [rbp + 0x1d8]
0x141e4872f cmp r12b, 0x32
0x141e48733 jne 0x141e49084
0x141e48739 mov rax, qword ptr [rbp + 0x1e0]
0x141e48740 movdqu xmm0, xmmword ptr [rbp + 0x1e8]
0x141e48748 test byte ptr [r13], 1
0x141e4874d je 0x141e48765
0x141e4874f mov rcx, qword ptr [r13 + 8]
0x141e48753 inc rcx
0x141e48756 mov rdx, 0xffffffffffffffff
0x141e4875d cmove rcx, rdx
0x141e48761 mov qword ptr [r13 + 8], rcx
0x141e48765 mov qword ptr [rbp + 0x110], rax
0x141e4876c movdqu xmmword ptr [rbp + 0x118], xmm0
0x141e48774 mov rcx, rsi
0x141e48777 mov rdx, r14
0x141e4877a call 0x140bc8550
0x141e4877f movzx r12d, byte ptr [rbp + 0x1d8]
0x141e48787 movzx esi, byte ptr [rbp + 0x1d9]
0x141e4878e cmp r12b, 0x32
0x141e48792 jne 0x141e490dd
0x141e48798 lea r13, [rbp + 0x1d8]
0x141e4879f mov byte ptr [rbp + 0x189], sil
0x141e487a6 mov rsi, qword ptr [rbp + 0x110]
0x141e487ad movdqu xmm9, xmmword ptr [rbp + 0x118]
0x141e487b6 mov r14, qword ptr [rbp + 0x168]
0x141e487bd mov rax, r14
0x141e487c0 neg rax
0x141e487c3 movabs r12, 0x8000000000000002
0x141e487cd jo 0x141e48140
0x141e487d3 mov rax, qword ptr [rbp + 0x170]
0x141e487da mov qword ptr [rbp + 0x1a8], rax
0x141e487e1 mov rdi, qword ptr [rbp + 0x178]
0x141e487e8 test rdi, rdi
0x141e487eb je 0x141e48840
0x141e487ed mov rax, qword ptr [rbp + 0x1a8]
0x141e487f4 lea rbx, [rax + 8]
0x141e487f8 jmp 0x141e48809
0x141e487fa nop word ptr [rax + rax]
0x141e48800 add rbx, 0x20
0x141e48804 dec rdi
0x141e48807 je 0x141e48840
0x141e48809 mov rdx, qword ptr [rbx - 8]
0x141e4880d lea rcx, [rdx + r15]
0x141e48811 cmp rcx, 3
0x141e48815 setb al
0x141e48818 cmp rcx, 1
0x141e4881c setne cl
0x141e4881f cmp rdx, r12
0x141e48822 jl 0x141e48800
0x141e48824 and al, cl
0x141e48826 jne 0x141e48800
0x141e48828 test rdx, rdx
0x141e4882b je 0x141e48800
0x141e4882d mov rcx, qword ptr [rbx]
0x141e48830 mov r8d, 1
0x141e48836 call 0x140613c20
0x141e4883b jmp 0x141e48800
0x141e4883d nop dword ptr [rax]
0x141e48840 test r14, r14
0x141e48843 je 0x141e48140
0x141e48849 shl r14, 5
0x141e4884d mov r8d, 8
0x141e48853 mov rcx, qword ptr [rbp + 0x1a8]
0x141e4885a mov rdx, r14
0x141e4885d call 0x140613c20
0x141e48862 jmp 0x141e48140
0x141e48867 cmp r8, 6
0x141e4886b jne 0x141e488bd
0x141e4886d mov eax, dword ptr [rdx]
0x141e4886f xor eax, dword ptr [rip + 0xd6c104]
0x141e48875 movzx ecx, word ptr [rdx + 4]
0x141e48879 xor cx, word ptr [rip + 0xd6c0fd]
0x141e48880 movzx ecx, cx
0x141e48883 or ecx, eax
0x141e48885 jne 0x141e488bd
0x141e48887 mov r11b, 1
0x141e4888a mov r12b, 0x32
0x141e4888d jmp 0x141e48892
0x141e4888f xor r11d, r11d
0x141e48892 mov r10, qword ptr [rbp + 0x158]
0x141e48899 movdqa xmm11, xmm14
0x141e4889e movaps xmm7, xmm12
0x141e488a2 movaps xmm8, xmm13
0x141e488a6 mov r13, qword ptr [rbp + 0x180]
0x141e488ad cmp dword ptr [r13], 1
0x141e488b2 jne 0x141e485b0
0x141e488b8 jmp 0x141e4859a
0x141e488bd mov qword ptr [rbp + 0x238], r9
0x141e488c4 mov qword ptr [rbp + 0x230], rdi
0x141e488cb mov byte ptr [rbp + 0x257], 1
0x141e488d2 mov byte ptr [rbp + 0x256], 1
0x141e488d9 mov byte ptr [rbp + 0x255], 1
0x141e488e0 mov qword ptr [rsp + 0x20], 3
0x141e488e9 lea rcx, [rbp + 0x110]
0x141e488f0 lea r9, [rip + 0xd6c091]
0x141e488f7 call 0x1428ca6f0
0x141e488fc movzx r12d, byte ptr [rbp + 0x110]
0x141e48904 cmp r12b, 0x32
0x141e48908 jne 0x141e48943
0x141e4890a mov r12b, 0x32
0x141e4890d movzx r11d, byte ptr [rbp + 0x111]
0x141e48915 mov r10, qword ptr [rbp + 0x158]
0x141e4891c movdqa xmm11, xmm14
0x141e48921 movaps xmm7, xmm12
0x141e48925 movaps xmm8, xmm13
0x141e48929 mov r9, rbx
0x141e4892c mov r13, qword ptr [rbp + 0x180]
0x141e48933 cmp dword ptr [r13], 1
0x141e48938 je 0x141e4859a
0x141e4893e jmp 0x141e485b0
0x141e48943 mov r10, qword ptr [rbp + 0x118]
0x141e4894a mov rax, qword ptr [rbp + 0x120]
0x141e48951 lea r8, [rbp + 0x111]
0x141e48958 jmp 0x141e4853c
0x141e4895d movzx eax, r11b
0x141e48961 cmp eax, 1
0x141e48964 jne 0x141e48b48
0x141e4896a mov rax, rdi
0x141e4896d neg rax
0x141e48970 jno 0x141e49a1a
0x141e48976 lea r14, [r13 + 0x10]
0x141e4897a mov qword ptr [rbp + 0x238], r9
0x141e48981 movabs rax, 0x8000000000000000
0x141e4898b mov qword ptr [rbp + 0x230], rax
0x141e48992 mov byte ptr [rbp + 0x257], 1
0x141e48999 mov byte ptr [rbp + 0x256], 1
0x141e489a0 mov byte ptr [rbp + 0x255], 1
0x141e489a7 mov rcx, rsi
0x141e489aa mov rdx, r14
0x141e489ad call 0x140bca140
0x141e489b2 movzx r12d, byte ptr [rbp + 0x1d8]
0x141e489ba cmp r12b, 0x32
0x141e489be jne 0x141e49566
0x141e489c4 mov qword ptr [rbp + 0x238], rbx
0x141e489cb movabs rax, 0x8000000000000000
0x141e489d5 mov qword ptr [rbp + 0x230], rax
0x141e489dc mov byte ptr [rbp + 0x257], 1
0x141e489e3 mov byte ptr [rbp + 0x256], 1
0x141e489ea mov byte ptr [rbp + 0x255], 1
0x141e489f1 mov rcx, r14
0x141e489f4 mov edx, 0x3a
0x141e489f9 call 0x140bc41b0
0x141e489fe test al, al
0x141e48a00 je 0x141e49608
0x141e48a06 mov qword ptr [rbp + 0x238], rbx
0x141e48a0d movabs rax, 0x8000000000000000
0x141e48a17 mov qword ptr [rbp + 0x230], rax
0x141e48a1e mov byte ptr [rbp + 0x257], 1
0x141e48a25 mov byte ptr [rbp + 0x256], 1
0x141e48a2c mov byte ptr [rbp + 0x255], 1
0x141e48a33 mov rcx, rsi
0x141e48a36 mov rdx, r14
0x141e48a39 call 0x140bca140
0x141e48a3e movzx r12d, byte ptr [rbp + 0x1d8]
0x141e48a46 cmp r12b, 0x32
0x141e48a4a jne 0x141e49566
0x141e48a50 test byte ptr [r13], 1
0x141e48a55 mov r9, rbx
0x141e48a58 je 0x141e48a6e
0x141e48a5a mov rax, qword ptr [r13 + 8]
0x141e48a5e test rax, rax
0x141e48a61 je 0x141e497b7
0x141e48a67 dec rax
0x141e48a6a mov qword ptr [r13 + 8], rax
0x141e48a6e mov qword ptr [rbp + 0x238], r9
0x141e48a75 movabs rax, 0x8000000000000000
0x141e48a7f mov qword ptr [rbp + 0x230], rax
0x141e48a86 mov byte ptr [rbp + 0x257], 1
0x141e48a8d mov byte ptr [rbp + 0x256], 1
0x141e48a94 mov byte ptr [rbp + 0x255], 1
0x141e48a9b mov rcx, rsi
0x141e48a9e mov rdx, r13
0x141e48aa1 call 0x141e42f90
0x141e48aa6 movzx r12d, byte ptr [rbp + 0x1d8]
0x141e48aae cmp r12b, 0x32
0x141e48ab2 jne 0x141e49566
0x141e48ab8 mov rax, qword ptr [rbp + 0x1e0]
0x141e48abf mov qword ptr [rbp + 0x1a0], rax
0x141e48ac6 movdqu xmm0, xmmword ptr [rbp + 0x1e8]
0x141e48ace movdqa xmmword ptr [rbp + 0xe0], xmm0
0x141e48ad6 test byte ptr [r13], 1
0x141e48adb je 0x141e48af3
0x141e48add mov rax, qword ptr [r13 + 8]
0x141e48ae1 inc rax
0x141e48ae4 mov rcx, 0xffffffffffffffff
0x141e48aeb cmove rax, rcx
0x141e48aef mov qword ptr [r13 + 8], rax
0x141e48af3 mov rcx, rsi
0x141e48af6 mov rdx, r14
0x141e48af9 call 0x140bc8550
0x141e48afe movzx r12d, byte ptr [rbp + 0x1d8]
0x141e48b06 movzx esi, byte ptr [rbp + 0x1d9]
0x141e48b0d cmp r12b, 0x32
0x141e48b11 jne 0x141e496b2
0x141e48b17 mov byte ptr [rbp + 0x189], sil
0x141e48b1e mov rdx, qword ptr [rbp + 0x1a0]
0x141e48b25 movdqa xmm0, xmmword ptr [rbp + 0xe0]
0x141e48b2d movdqa xmmword ptr [rbp + 0xf0], xmm0
0x141e48b35 mov rcx, qword ptr [rbp + 0x240]
0x141e48b3c lea r13, [rbp + 0x1d8]
0x141e48b43 jmp 0x141e4812c
0x141e48b48 cmp eax, 2
0x141e48b4b jne 0x141e48e5c
0x141e48b51 movabs rax, 0x8000000000000001
0x141e48b5b cmp r9, rax
0x141e48b5e jne 0x141e49a01
0x141e48b64 lea r14, [r13 + 0x10]
0x141e48b68 mov qword ptr [rbp + 0x238], rax
0x141e48b6f mov qword ptr [rbp + 0x230], rdi
0x141e48b76 mov byte ptr [rbp + 0x257], 1
0x141e48b7d mov byte ptr [rbp + 0x256], 1
0x141e48b84 mov byte ptr [rbp + 0x255], 1
0x141e48b8b mov rcx, rsi
0x141e48b8e mov rdx, r14
0x141e48b91 call 0x140bca140
0x141e48b96 movzx r12d, byte ptr [rbp + 0x1d8]
0x141e48b9e cmp r12b, 0x32
0x141e48ba2 jne 0x141e495b4
0x141e48ba8 movabs rax, 0x8000000000000001
0x141e48bb2 mov qword ptr [rbp + 0x238], rax
0x141e48bb9 mov qword ptr [rbp + 0x230], rdi
0x141e48bc0 mov byte ptr [rbp + 0x257], 1
0x141e48bc7 mov byte ptr [rbp + 0x256], 1
0x141e48bce mov byte ptr [rbp + 0x255], 1
0x141e48bd5 mov rcx, r14
0x141e48bd8 mov edx, 0x3a
0x141e48bdd call 0x140bc41b0
0x141e48be2 test al, al
0x141e48be4 je 0x141e4964c
0x141e48bea movabs rax, 0x8000000000000001
0x141e48bf4 mov qword ptr [rbp + 0x238], rax
0x141e48bfb mov qword ptr [rbp + 0x230], rdi
0x141e48c02 mov byte ptr [rbp + 0x257], 1
0x141e48c09 mov byte ptr [rbp + 0x256], 1
0x141e48c10 mov byte ptr [rbp + 0x255], 1
0x141e48c17 mov rcx, rsi
0x141e48c1a mov rdx, r14
0x141e48c1d call 0x140bca140
0x141e48c22 movzx r12d, byte ptr [rbp + 0x1d8]
0x141e48c2a cmp r12b, 0x32
0x141e48c2e jne 0x141e495b4
0x141e48c34 test byte ptr [r13], 1
0x141e48c39 mov rdx, rdi
0x141e48c3c je 0x141e48c52
0x141e48c3e mov rax, qword ptr [r13 + 8]
0x141e48c42 test rax, rax
0x141e48c45 je 0x141e497ab
0x141e48c4b dec rax
0x141e48c4e mov qword ptr [r13 + 8], rax
0x141e48c52 movabs rax, 0x8000000000000001
0x141e48c5c mov qword ptr [rbp + 0x238], rax
0x141e48c63 mov qword ptr [rbp + 0x230], rdx
0x141e48c6a mov byte ptr [rbp + 0x257], 1
0x141e48c71 mov byte ptr [rbp + 0x256], 1
0x141e48c78 mov byte ptr [rbp + 0x255], 1
0x141e48c7f mov rcx, rsi
0x141e48c82 mov rdx, r14
0x141e48c85 call 0x140bcac50
0x141e48c8a movzx r12d, byte ptr [rbp + 0x1d8]
0x141e48c92 cmp r12b, 0x32
0x141e48c96 jne 0x141e4965e
0x141e48c9c mov rcx, qword ptr [rbp + 0x1e0]
0x141e48ca3 mov r12, qword ptr [rbp + 0x1e8]
0x141e48caa mov rbx, qword ptr [rbp + 0x1f0]
0x141e48cb1 mov rax, rcx
0x141e48cb4 neg rax
0x141e48cb7 jno 0x141e48d25
0x141e48cb9 mov qword ptr [rbp + 0x240], rbx
0x141e48cc0 test rbx, rbx
0x141e48cc3 js 0x141e49a37
0x141e48cc9 mov qword ptr [rbp + 0x198], r14
0x141e48cd0 je 0x141e48cfd
0x141e48cd2 movzx eax, byte ptr [rip + 0x17ebf30]
0x141e48cd9 mov ebx, 1
0x141e48cde mov edx, 1
0x141e48ce3 mov rcx, qword ptr [rbp + 0x240]
0x141e48cea call 0x140613c10
0x141e48cef test rax, rax
0x141e48cf2 je 0x141e49a39
0x141e48cf8 mov r14, rax
0x141e48cfb jmp 0x141e48d03
0x141e48cfd mov r14d, 1
0x141e48d03 mov rcx, r14
0x141e48d06 mov rdx, r12
0x141e48d09 mov rbx, qword ptr [rbp + 0x240]
0x141e48d10 mov r8, rbx
0x141e48d13 call 0x142923c00
0x141e48d18 mov r12, r14
0x141e48d1b mov rcx, rbx
0x141e48d1e mov r14, qword ptr [rbp + 0x198]
0x141e48d25 mov qword ptr [rbp + 0x240], rcx
0x141e48d2c test byte ptr [r13], 1
0x141e48d31 je 0x141e48d49
0x141e48d33 mov rax, qword ptr [r13 + 8]
0x141e48d37 inc rax
0x141e48d3a mov rcx, 0xffffffffffffffff
0x141e48d41 cmove rax, rcx
0x141e48d45 mov qword ptr [r13 + 8], rax
0x141e48d49 mov qword ptr [rbp + 0x198], r12
0x141e48d50 mov rcx, rsi
0x141e48d53 mov rdx, r14
0x141e48d56 call 0x140bc8550
0x141e48d5b movzx r12d, byte ptr [rbp + 0x1d8]
0x141e48d63 movzx esi, byte ptr [rbp + 0x1d9]
0x141e48d6a cmp r12b, 0x32
0x141e48d6e jne 0x141e4972e
0x141e48d74 mov byte ptr [rbp + 0x189], sil
0x141e48d7b movq xmm0, rbx
0x141e48d80 movq xmm1, qword ptr [rbp + 0x198]
0x141e48d88 punpcklqdq xmm1, xmm0
0x141e48d8c movdqa xmmword ptr [rbp + 0x100], xmm1
0x141e48d94 mov rdx, qword ptr [rbp + 0x1c0]
0x141e48d9b mov rcx, qword ptr [rbp + 0x240]
0x141e48da2 lea r13, [rbp + 0x1d8]
0x141e48da9 jmp 0x141e48125
0x141e48dae mov rax, qword ptr [rbp + 0x218]
0x141e48db5 mov qword ptr [rsi + 0x48], rax
0x141e48db9 movdqu xmm0, xmmword ptr [rbp + 0x1d8]
0x141e48dc1 movdqu xmm1, xmmword ptr [rbp + 0x1e8]
0x141e48dc9 movups xmm2, xmmword ptr [rbp + 0x1f8]
0x141e48dd0 movups xmm3, xmmword ptr [rbp + 0x208]
0x141e48dd7 movups xmmword ptr [rsi + 0x38], xmm3
0x141e48ddb movups xmmword ptr [rsi + 0x28], xmm2
0x141e48ddf movdqu xmmword ptr [rsi + 0x18], xmm1
0x141e48de4 movdqu xmmword ptr [rsi + 8], xmm0
0x141e48de9 jmp 0x141e49995
0x141e48dee movzx ecx, byte ptr [rbp + 0x1d9]
0x141e48df5 mov rdx, qword ptr [rbp + 0x218]
0x141e48dfc mov qword ptr [rsi + 0x48], rdx
0x141e48e00 movups xmm0, xmmword ptr [rbp + 0x20a]
0x141e48e07 movups xmmword ptr [rsi + 0x3a], xmm0
0x141e48e0b movups xmm0, xmmword ptr [rbp + 0x1fa]
0x141e48e12 movups xmmword ptr [rsi + 0x2a], xmm0
0x141e48e16 movups xmm0, xmmword ptr [rbp + 0x1ea]
0x141e48e1d movups xmmword ptr [rsi + 0x1a], xmm0
0x141e48e21 movdqu xmm0, xmmword ptr [rbp + 0x1da]
0x141e48e29 movdqu xmmword ptr [rsi + 0xa], xmm0
0x141e48e2e mov byte ptr [rsi + 8], al
0x141e48e31 mov byte ptr [rsi + 9], cl
0x141e48e34 jmp 0x141e49995
0x141e48e39 test rdi, rdi
0x141e48e3c je 0x141e48f87
0x141e48e42 mov byte ptr [rsi + 8], 0x17
0x141e48e46 mov qword ptr [rsi + 0x10], rbx
0x141e48e4a mov qword ptr [rsi + 0x18], rdi
0x141e48e4e jmp 0x141e49995
0x141e48e53 mov byte ptr [rsi + 8], 0x31
0x141e48e57 jmp 0x141e49995
0x141e48e5c mov qword ptr [rbp + 0x1d8], 0
0x141e48e67 mov qword ptr [rbp + 0x1e0], 8
0x141e48e72 pxor xmm0, xmm0
0x141e48e76 movdqu xmmword ptr [rbp + 0x1e8], xmm0
0x141e48e7e mov qword ptr [rbp + 0x1f8], 4
0x141e48e89 mov qword ptr [rbp + 0x200], 0
0x141e48e94 movabs r13, 0x8000000000000000
0x141e48e9e mov qword ptr [rbp + 0x208], r13
0x141e48ea5 mov rsi, qword ptr [rbp + 0x168]
0x141e48eac mov rax, rdi
0x141e48eaf neg rax
0x141e48eb2 seto dl
0x141e48eb5 mov rax, rsi
0x141e48eb8 neg rax
0x141e48ebb jno 0x141e48fe6
0x141e48ec1 neg rdi
0x141e48ec4 movdqa xmm6, xmmword ptr [rbp + 0x100]
0x141e48ecc mov r15, qword ptr [rbp + 0xd8]
0x141e48ed3 movabs rax, 0x8000000000000001
0x141e48edd jno 0x141e491af
0x141e48ee3 cmp qword ptr [rbp + 0x240], rax
0x141e48eea jne 0x141e492e9
0x141e48ef0 movdqu xmm6, xmmword ptr [rbp + 0x210]
0x141e48ef8 lea rcx, [rbp + 0x1da]
0x141e48eff mov eax, dword ptr [rcx]
0x141e48f01 mov dword ptr [rbp + 0x24c], eax
0x141e48f07 movzx eax, word ptr [rcx + 4]
0x141e48f0b mov word ptr [rbp + 0x250], ax
0x141e48f12 movss xmm7, dword ptr [rip + 0xca3bea]
0x141e48f1a mov ebx, 8
0x141e48f1f mov qword ptr [rbp + 0x1c0], 0
0x141e48f2a xor r14d, r14d
0x141e48f2d xor r12d, r12d
0x141e48f30 jmp 0x141e49421
0x141e48f35 movzx esi, byte ptr [rbp + 0x1d9]
0x141e48f3c mov eax, dword ptr [rbp + 0x1da]
0x141e48f42 mov dword ptr [rbp + 0x228], eax
0x141e48f48 movzx eax, word ptr [rbp + 0x1de]
0x141e48f4f mov word ptr [rbp + 0x22c], ax
0x141e48f56 mov rcx, qword ptr [rbp + 0x1e0]
0x141e48f5d movups xmm9, xmmword ptr [rbp + 0x1e8]
0x141e48f65 movups xmm7, xmmword ptr [rbp + 0x1f8]
0x141e48f6c movups xmm8, xmmword ptr [rbp + 0x208]
0x141e48f74 mov rax, qword ptr [rbp + 0x218]
0x141e48f7b mov qword ptr [rbp + 0x1d0], rax
0x141e48f82 jmp 0x141e49265
0x141e48f87 mov byte ptr [rsi + 8], 0x16
0x141e48f8b jmp 0x141e49995
0x141e48f90 movzx eax, word ptr [rbp + 0x1de]
0x141e48f97 mov word ptr [rbp + 0x1bc], ax
0x141e48f9e mov eax, dword ptr [rbp + 0x1da]
0x141e48fa4 mov dword ptr [rbp + 0x1b8], eax
0x141e48faa mov r10, qword ptr [rbp + 0x1e0]
0x141e48fb1 movups xmm11, xmmword ptr [rbp + 0x1e8]
0x141e48fb9 movups xmm7, xmmword ptr [rbp + 0x1f8]
0x141e48fc0 movups xmm8, xmmword ptr [rbp + 0x208]
0x141e48fc8 mov rax, qword ptr [rbp + 0x218]
0x141e48fcf mov qword ptr [rbp + 0x1d0], rax
0x141e48fd6 mov r9, rbx
0x141e48fd9 jmp 0x141e49184
0x141e48fde mov esi, r11d
0x141e48fe1 jmp 0x141e49184
0x141e48fe6 mov r12d, esi
0x141e48fe9 shr r12d, 8
0x141e48fed neg rdi
0x141e48ff0 lea rax, [rbp + 0x16a]
0x141e48ff7 mov rbx, qword ptr [rbp + 0x170]
0x141e48ffe mov r14, qword ptr [rbp + 0x178]
0x141e49005 mov r15, qword ptr [rbp + 0xd8]
0x141e4900c mov byte ptr [rbp + 0x1d0], dl
0x141e49012 movabs rcx, 0x8000000000000001
0x141e4901c jno 0x141e491eb
0x141e49022 cmp qword ptr [rbp + 0x240], rcx
0x141e49029 jne 0x141e49326
0x141e4902f movdqu xmm6, xmmword ptr [rbp + 0x210]
0x141e49037 mov ecx, dword ptr [rax]
0x141e49039 mov dword ptr [rbp + 0x24c], ecx
0x141e4903f movzx eax, word ptr [rax + 4]
0x141e49043 mov word ptr [rbp + 0x250], ax
0x141e4904a mov byte ptr [rbp + 0x254], 1
0x141e49051 mov byte ptr [rbp + 0x253], 0
0x141e49058 lea rcx, [rbp + 0x1d8]
0x141e4905f call 0x1405f11f0
0x141e49064 movss xmm7, dword ptr [rip + 0xca3a98]
0x141e4906c mov qword ptr [rbp + 0x1c0], 0
0x141e49077 jmp 0x141e49421
0x141e4907c mov r12b, 0x13
0x141e4907f jmp 0x141e4925e
0x141e49084 movzx esi, byte ptr [rbp + 0x1d9]
0x141e4908b mov eax, dword ptr [rbp + 0x1da]
0x141e49091 mov dword ptr [rbp + 0x228], eax
0x141e49097 movzx eax, word ptr [rbp + 0x1de]
0x141e4909e mov word ptr [rbp + 0x22c], ax
0x141e490a5 mov rax, qword ptr [rbp + 0x1e0]
0x141e490ac mov qword ptr [rbp + 0x1a8], rax
0x141e490b3 movups xmm9, xmmword ptr [rbp + 0x1e8]
0x141e490bb movups xmm7, xmmword ptr [rbp + 0x1f8]
0x141e490c2 movups xmm8, xmmword ptr [rbp + 0x208]
0x141e490ca mov rax, qword ptr [rbp + 0x218]
0x141e490d1 mov qword ptr [rbp + 0x1d0], rax
0x141e490d8 jmp 0x141e4925e
0x141e490dd movzx eax, word ptr [rbp + 0x1de]
0x141e490e4 mov word ptr [rbp + 0x22c], ax
0x141e490eb mov eax, dword ptr [rbp + 0x1da]
0x141e490f1 mov dword ptr [rbp + 0x228], eax
0x141e490f7 mov rax, qword ptr [rbp + 0x1e0]
0x141e490fe mov qword ptr [rbp + 0x1a8], rax
0x141e49105 movups xmm9, xmmword ptr [rbp + 0x1e8]
0x141e4910d movups xmm7, xmmword ptr [rbp + 0x1f8]
0x141e49114 movups xmm8, xmmword ptr [rbp + 0x208]
0x141e4911c mov rax, qword ptr [rbp + 0x218]
0x141e49123 mov qword ptr [rbp + 0x1d0], rax
0x141e4912a mov rax, qword ptr [rbp + 0x240]
0x141e49131 mov rbx, rax
0x141e49134 mov qword ptr [rbp + 0x238], rax
0x141e4913b mov rax, qword ptr [rbp + 0x1c0]
0x141e49142 mov rdi, rax
0x141e49145 mov qword ptr [rbp + 0x230], rax
0x141e4914c mov byte ptr [rbp + 0x257], 1
0x141e49153 mov byte ptr [rbp + 0x256], 1
0x141e4915a mov byte ptr [rbp + 0x255], 1
0x141e49161 lea rcx, [rbp + 0x110]
0x141e49168 call 0x1405f11f0
0x141e4916d jmp 0x141e4925e
0x141e49172 mov r12b, 0x31
0x141e49175 mov eax, dword ptr [rbp + 0x1cc]
0x141e4917b mov esi, eax
0x141e4917d mov r10, qword ptr [rbp + 0x190]
0x141e49184 movzx eax, word ptr [rbp + 0x1bc]
0x141e4918b mov word ptr [rbp + 0x250], ax
0x141e49192 mov eax, dword ptr [rbp + 0x1b8]
0x141e49198 mov dword ptr [rbp + 0x24c], eax
0x141e4919e mov qword ptr [rbp + 0x1b0], r10
0x141e491a5 movdqa xmm6, xmm11
0x141e491aa jmp 0x141e4928e
0x141e491af cmp qword ptr [rbp + 0x240], rax
0x141e491b6 jne 0x141e4936b
0x141e491bc movdqu xmm6, xmmword ptr [rbp + 0x210]
0x141e491c4 lea rcx, [rbp + 0x1da]
0x141e491cb mov eax, dword ptr [rcx]
0x141e491cd mov dword ptr [rbp + 0x24c], eax
0x141e491d3 movzx eax, word ptr [rcx + 4]
0x141e491d7 mov word ptr [rbp + 0x250], ax
0x141e491de mov ebx, 8
0x141e491e3 xor r14d, r14d
0x141e491e6 xor r12d, r12d
0x141e491e9 jmp 0x141e4924f
0x141e491eb cmp qword ptr [rbp + 0x240], rcx
0x141e491f2 jne 0x141e49399
0x141e491f8 movdqu xmm6, xmmword ptr [rbp + 0x210]
0x141e49200 mov ecx, dword ptr [rax]
0x141e49202 mov dword ptr [rbp + 0x24c], ecx
0x141e49208 movzx eax, word ptr [rax + 4]
0x141e4920c mov word ptr [rbp + 0x250], ax
0x141e49213 mov byte ptr [rbp + 0x254], 1
0x141e4921a mov byte ptr [rbp + 0x253], 0
0x141e49221 lea rcx, [rbp + 0x1d8]
0x141e49228 call 0x1405f11f0
0x141e4922d mov rdx, qword ptr [rbp + 0x1f0]
0x141e49234 test rdx, rdx
0x141e49237 je 0x141e4924f
0x141e49239 mov rcx, qword ptr [rbp + 0x1f8]
0x141e49240 shl rdx, 5
0x141e49244 mov r8d, 4
0x141e4924a call 0x140613c20
0x141e4924f movaps xmm7, xmmword ptr [rbp + 0xf0]
0x141e49256 jmp 0x141e49421
0x141e4925b mov r12b, 0x31
0x141e4925e mov rcx, qword ptr [rbp + 0x1a8]
0x141e49265 mov r9, rbx
0x141e49268 movzx eax, word ptr [rbp + 0x22c]
0x141e4926f mov word ptr [rbp + 0x250], ax
0x141e49276 mov eax, dword ptr [rbp + 0x228]
0x141e4927c mov dword ptr [rbp + 0x24c], eax
0x141e49282 mov qword ptr [rbp + 0x1b0], rcx
0x141e49289 movdqa xmm6, xmm9
0x141e4928e movabs rax, 0x8000000000000002
0x141e49298 cmp r9, rax
0x141e4929b jl 0x141e492bd
0x141e4929d test r9, r9
0x141e492a0 je 0x141e492bd
0x141e492a2 movdqa xmm0, xmmword ptr [rbp + 0x100]
0x141e492aa movq rcx, xmm0
0x141e492af mov r8d, 1
0x141e492b5 mov rdx, r9
0x141e492b8 call 0x140613c20
0x141e492bd mov rdx, rdi
0x141e492c0 lea rax, [rdx*2]
0x141e492c8 test rax, rax
0x141e492cb jne 0x141e49805
0x141e492d1 mov r14, qword ptr [rbp + 0x168]
0x141e492d8 mov rax, r14
0x141e492db neg rax
0x141e492de jno 0x141e49834
0x141e492e4 jmp 0x141e498b1
0x141e492e9 lea rcx, [rbp + 0x1da]
0x141e492f0 movzx eax, word ptr [rcx + 4]
0x141e492f4 mov word ptr [rbp + 0x250], ax
0x141e492fb mov eax, dword ptr [rcx]
0x141e492fd mov dword ptr [rbp + 0x24c], eax
0x141e49303 movss xmm7, dword ptr [rip + 0xca37f9]
0x141e4930b mov ebx, 8
0x141e49310 mov qword ptr [rbp + 0x1c0], 0
0x141e4931b xor r14d, r14d
0x141e4931e xor r12d, r12d
0x141e49321 jmp 0x141e493f7
0x141e49326 movzx ecx, word ptr [rax + 4]
0x141e4932a mov word ptr [rbp + 0x250], cx
0x141e49331 mov eax, dword ptr [rax]
0x141e49333 mov dword ptr [rbp + 0x24c], eax
0x141e49339 mov byte ptr [rbp + 0x254], 0
0x141e49340 mov byte ptr [rbp + 0x253], 1
0x141e49347 lea rcx, [rbp + 0x1d8]
0x141e4934e call 0x1405f11f0
0x141e49353 movss xmm7, dword ptr [rip + 0xca37a9]
0x141e4935b mov qword ptr [rbp + 0x1c0], 0
0x141e49366 jmp 0x141e493ef
0x141e4936b lea rcx, [rbp + 0x1da]
0x141e49372 movzx eax, word ptr [rcx + 4]
0x141e49376 mov word ptr [rbp + 0x250], ax
0x141e4937d mov eax, dword ptr [rcx]
0x141e4937f mov dword ptr [rbp + 0x24c], eax
0x141e49385 mov ebx, 8
0x141e4938a xor r14d, r14d
0x141e4938d xor r12d, r12d
0x141e49390 movaps xmm7, xmmword ptr [rbp + 0xf0]
0x141e49397 jmp 0x141e493f7
0x141e49399 movzx ecx, word ptr [rax + 4]
0x141e4939d mov word ptr [rbp + 0x250], cx
0x141e493a4 mov eax, dword ptr [rax]
0x141e493a6 mov dword ptr [rbp + 0x24c], eax
0x141e493ac mov byte ptr [rbp + 0x254], 0
0x141e493b3 mov byte ptr [rbp + 0x253], 1
0x141e493ba lea rcx, [rbp + 0x1d8]
0x141e493c1 call 0x1405f11f0
0x141e493c6 mov rdx, qword ptr [rbp + 0x1f0]
0x141e493cd test rdx, rdx
0x141e493d0 je 0x141e493e8
0x141e493d2 mov rcx, qword ptr [rbp + 0x1f8]
0x141e493d9 shl rdx, 5
0x141e493dd mov r8d, 4
0x141e493e3 call 0x140613c20
0x141e493e8 movaps xmm7, xmmword ptr [rbp + 0xf0]
0x141e493ef movdqa xmm6, xmmword ptr [rbp + 0x100]
0x141e493f7 mov rdx, qword ptr [rbp + 0x208]
0x141e493fe mov rax, rdx
0x141e49401 neg rax
0x141e49404 jo 0x141e4941a
0x141e49406 jae 0x141e4941a
0x141e49408 mov rcx, qword ptr [rbp + 0x210]
0x141e4940f mov r8d, 1
0x141e49415 call 0x140613c20
0x141e4941a mov r13, qword ptr [rbp + 0x240]
0x141e49421 movzx eax, word ptr [rbp + 0x250]
0x141e49428 mov ecx, dword ptr [rbp + 0x24c]
0x141e4942e mov dword ptr [rbp + 0x160], ecx
0x141e49434 mov word ptr [rbp + 0x164], ax
0x141e4943b test byte ptr [r15], 1
0x141e4943f je 0x141e49457
0x141e49441 mov rax, qword ptr [r15 + 8]
0x141e49445 inc rax
0x141e49448 mov rcx, 0xffffffffffffffff
0x141e4944f cmovne rcx, rax
0x141e49453 mov qword ptr [r15 + 8], rcx
0x141e49457 mov byte ptr [rbp + 0x40], sil
0x141e4945b mov byte ptr [rbp + 0x41], r12b
0x141e4945f mov eax, dword ptr [rbp + 0x160]
0x141e49465 mov dword ptr [rbp + 0x42], eax
0x141e49468 movzx eax, word ptr [rbp + 0x164]
0x141e4946f mov word ptr [rbp + 0x46], ax
0x141e49473 mov qword ptr [rbp + 0x48], rbx
0x141e49477 mov qword ptr [rbp + 0x50], r14
0x141e4947b mov rax, qword ptr [rbp + 0x1c0]
0x141e49482 mov qword ptr [rbp + 0x58], rax
0x141e49486 movups xmmword ptr [rbp + 0x60], xmm7
0x141e4948a mov qword ptr [rbp + 0x70], r13
0x141e4948e movdqu xmmword ptr [rbp + 0x78], xmm6
0x141e49493 lea rcx, [rbp + 0x1d8]
0x141e4949a mov rdi, qword ptr [rbp + 0xc0]
0x141e494a1 mov rdx, rdi
0x141e494a4 call 0x140bca140
0x141e494a9 mov rsi, qword ptr [rbp + 0xd0]
0x141e494b0 cmp byte ptr [rbp + 0x1d8], 0x32
0x141e494b7 movzx eax, byte ptr [rbp + 0x227]
0x141e494be jne 0x141e49510
0x141e494c0 test al, 1
0x141e494c2 jne 0x141e494d9
0x141e494c4 mov rcx, rdi
0x141e494c7 mov edx, 0x29
0x141e494cc call 0x140bc41b0
0x141e494d1 test al, al
0x141e494d3 je 0x141e49560
0x141e494d9 mov rax, qword ptr [rbp + 0x80]
0x141e494e0 mov qword ptr [rsi + 0x48], rax
0x141e494e4 movups xmm0, xmmword ptr [rbp + 0x40]
0x141e494e8 movups xmm1, xmmword ptr [rbp + 0x50]
0x141e494ec movups xmm2, xmmword ptr [rbp + 0x60]
0x141e494f0 movups xmm3, xmmword ptr [rbp + 0x70]
0x141e494f4 movups xmmword ptr [rsi + 0x38], xmm3
0x141e494f8 movups xmmword ptr [rsi + 0x28], xmm2
0x141e494fc movups xmmword ptr [rsi + 0x18], xmm1
0x141e49500 movups xmmword ptr [rsi + 8], xmm0
0x141e49504 mov qword ptr [rsi], 0
0x141e4950b jmp 0x141e4999c
0x141e49510 mov rax, qword ptr [rbp + 0x218]
0x141e49517 mov qword ptr [rsi + 0x48], rax
0x141e4951b movdqu xmm0, xmmword ptr [rbp + 0x1d8]
0x141e49523 movdqu xmm1, xmmword ptr [rbp + 0x1e8]
0x141e4952b movups xmm2, xmmword ptr [rbp + 0x1f8]
0x141e49532 movups xmm3, xmmword ptr [rbp + 0x208]
0x141e49539 movups xmmword ptr [rsi + 0x38], xmm3
0x141e4953d movups xmmword ptr [rsi + 0x28], xmm2
0x141e49541 movdqu xmmword ptr [rsi + 0x18], xmm1
0x141e49546 movdqu xmmword ptr [rsi + 8], xmm0
0x141e4954b mov qword ptr [rsi], 1
0x141e49552 lea rcx, [rbp + 0x40]
0x141e49556 call 0x140fa6cf0
0x141e4955b jmp 0x141e4999c
0x141e49560 mov byte ptr [rsi + 8], 0x18
0x141e49564 jmp 0x141e4954b
0x141e49566 movzx esi, byte ptr [rbp + 0x1d9]
0x141e4956d mov eax, dword ptr [rbp + 0x1da]
0x141e49573 mov dword ptr [rbp + 0x110], eax
0x141e49579 movzx eax, word ptr [rbp + 0x1de]
0x141e49580 mov word ptr [rbp + 0x114], ax
0x141e49587 mov rcx, qword ptr [rbp + 0x1e0]
0x141e4958e movups xmm6, xmmword ptr [rbp + 0x1e8]
0x141e49595 movups xmm7, xmmword ptr [rbp + 0x1f8]
0x141e4959c movups xmm8, xmmword ptr [rbp + 0x208]
0x141e495a4 mov rax, qword ptr [rbp + 0x218]
0x141e495ab mov qword ptr [rbp + 0x1d0], rax
0x141e495b2 jmp 0x141e49619
0x141e495b4 movzx esi, byte ptr [rbp + 0x1d9]
0x141e495bb mov eax, dword ptr [rbp + 0x1da]
0x141e495c1 mov dword ptr [rbp + 0x110], eax
0x141e495c7 movzx eax, word ptr [rbp + 0x1de]
0x141e495ce mov word ptr [rbp + 0x114], ax
0x141e495d5 mov rbx, qword ptr [rbp + 0x1e0]
0x141e495dc movups xmm6, xmmword ptr [rbp + 0x1e8]
0x141e495e3 movups xmm7, xmmword ptr [rbp + 0x1f8]
0x141e495ea movups xmm8, xmmword ptr [rbp + 0x208]
0x141e495f2 mov rax, qword ptr [rbp + 0x218]
0x141e495f9 mov qword ptr [rbp + 0x1d0], rax
0x141e49600 mov rdx, rdi
0x141e49603 jmp 0x141e497d3
0x141e49608 mov r12b, 0x13
0x141e4960b mov rcx, qword ptr [rbp + 0x1a0]
0x141e49612 movaps xmm6, xmmword ptr [rbp + 0xe0]
0x141e49619 mov r9, rbx
0x141e4961c movzx eax, word ptr [rbp + 0x114]
0x141e49623 mov word ptr [rbp + 0x250], ax
0x141e4962a mov eax, dword ptr [rbp + 0x110]
0x141e49630 mov dword ptr [rbp + 0x24c], eax
0x141e49636 movabs rdi, 0x8000000000000000
0x141e49640 mov qword ptr [rbp + 0x1b0], rcx
0x141e49647 jmp 0x141e4928e
0x141e4964c mov r12b, 0x13
0x141e4964f movaps xmm6, xmmword ptr [rbp + 0x100]
0x141e49656 mov rdx, rdi
0x141e49659 jmp 0x141e497d3
0x141e4965e movzx esi, byte ptr [rbp + 0x1d9]
0x141e49665 mov eax, dword ptr [rbp + 0x1da]
0x141e4966b mov dword ptr [rbp + 0x110], eax
0x141e49671 movzx eax, word ptr [rbp + 0x1de]
0x141e49678 mov word ptr [rbp + 0x114], ax
0x141e4967f mov rbx, qword ptr [rbp + 0x1e0]
0x141e49686 movups xmm7, xmmword ptr [rbp + 0x1f8]
0x141e4968d movups xmm8, xmmword ptr [rbp + 0x208]
0x141e49695 mov rax, qword ptr [rbp + 0x218]
0x141e4969c mov qword ptr [rbp + 0x1d0], rax
0x141e496a3 movups xmm6, xmmword ptr [rbp + 0x1e8]
0x141e496aa mov rdx, rdi
0x141e496ad jmp 0x141e497d3
0x141e496b2 movzx eax, word ptr [rbp + 0x1de]
0x141e496b9 mov word ptr [rbp + 0x114], ax
0x141e496c0 mov eax, dword ptr [rbp + 0x1da]
0x141e496c6 mov dword ptr [rbp + 0x110], eax
0x141e496cc mov rdi, qword ptr [rbp + 0x1e0]
0x141e496d3 movups xmm6, xmmword ptr [rbp + 0x1e8]
0x141e496da movups xmm7, xmmword ptr [rbp + 0x1f8]
0x141e496e1 movups xmm8, xmmword ptr [rbp + 0x208]
0x141e496e9 mov rax, qword ptr [rbp + 0x218]
0x141e496f0 mov qword ptr [rbp + 0x1d0], rax
0x141e496f7 mov rdx, qword ptr [rbp + 0x1a0]
0x141e496fe test rdx, rdx
0x141e49701 je 0x141e4971f
0x141e49703 movdqa xmm0, xmmword ptr [rbp + 0xe0]
0x141e4970b movq rcx, xmm0
0x141e49710 shl rdx, 5
0x141e49714 mov r8d, 4
0x141e4971a call 0x140613c20
0x141e4971f mov rcx, rdi
0x141e49722 mov r9, qword ptr [rbp + 0x240]
0x141e49729 jmp 0x141e4961c
0x141e4972e movzx eax, word ptr [rbp + 0x1de]
0x141e49735 mov word ptr [rbp + 0x114], ax
0x141e4973c mov eax, dword ptr [rbp + 0x1da]
0x141e49742 mov dword ptr [rbp + 0x110], eax
0x141e49748 mov rdi, qword ptr [rbp + 0x1e0]
0x141e4974f movups xmm6, xmmword ptr [rbp + 0x1e8]
0x141e49756 movups xmm7, xmmword ptr [rbp + 0x1f8]
0x141e4975d movups xmm8, xmmword ptr [rbp + 0x208]
0x141e49765 mov rax, qword ptr [rbp + 0x218]
0x141e4976c mov qword ptr [rbp + 0x1d0], rax
0x141e49773 mov rdx, qword ptr [rbp + 0x240]
0x141e4977a test rdx, rdx
0x141e4977d mov rax, qword ptr [rbp + 0x1c0]
0x141e49784 je 0x141e497cd
0x141e49786 mov r8d, 1
0x141e4978c mov rcx, qword ptr [rbp + 0x198]
0x141e49793 mov rbx, rax
0x141e49796 call 0x140613c20
0x141e4979b mov rdx, rbx
0x141e4979e mov rbx, rdi
0x141e497a1 jmp 0x141e497d3
0x141e497a3 mov r12b, 0x2d
0x141e497a6 jmp 0x141e4928e
0x141e497ab mov r12b, 0x31
0x141e497ae movaps xmm6, xmmword ptr [rbp + 0x100]
0x141e497b5 jmp 0x141e497d3
0x141e497b7 mov r12b, 0x31
0x141e497ba mov rcx, qword ptr [rbp + 0x1a0]
0x141e497c1 movaps xmm6, xmmword ptr [rbp + 0xe0]
0x141e497c8 jmp 0x141e4961c
0x141e497cd mov rbx, rdi
0x141e497d0 mov rdx, rax
0x141e497d3 movzx eax, word ptr [rbp + 0x114]
0x141e497da mov word ptr [rbp + 0x250], ax
0x141e497e1 mov eax, dword ptr [rbp + 0x110]
0x141e497e7 mov dword ptr [rbp + 0x24c], eax
0x141e497ed mov qword ptr [rbp + 0x1b0], rbx
0x141e497f4 lea rax, [rdx*2]
0x141e497fc test rax, rax
0x141e497ff je 0x141e492d1
0x141e49805 movdqa xmm0, xmmword ptr [rbp + 0xf0]
0x141e4980d movq rcx, xmm0
0x141e49812 shl rdx, 5
0x141e49816 mov r8d, 4
0x141e4981c call 0x140613c20
0x141e49821 mov r14, qword ptr [rbp + 0x168]
0x141e49828 mov rax, r14
0x141e4982b neg rax
0x141e4982e jo 0x141e498b1
0x141e49834 mov r13, qword ptr [rbp + 0x170]
0x141e4983b mov rdi, qword ptr [rbp + 0x178]
0x141e49842 test rdi, rdi
0x141e49845 je 0x141e49897
0x141e49847 lea rbx, [r13 + 8]
0x141e4984b jmp 0x141e49859
0x141e4984d nop dword ptr [rax]
0x141e49850 add rbx, 0x20
0x141e49854 dec rdi
0x141e49857 je 0x141e49897
0x141e49859 mov rdx, qword ptr [rbx - 8]
0x141e4985d lea rcx, [rdx + r15]
0x141e49861 cmp rcx, 3
0x141e49865 setb al
0x141e49868 cmp rcx, 1
0x141e4986c setne cl
0x141e4986f movabs r8, 0x8000000000000002
0x141e49879 cmp rdx, r8
0x141e4987c jl 0x141e49850
0x141e4987e and al, cl
0x141e49880 jne 0x141e49850
0x141e49882 test rdx, rdx
0x141e49885 je 0x141e49850
0x141e49887 mov rcx, qword ptr [rbx]
0x141e4988a mov r8d, 1
0x141e49890 call 0x140613c20
0x141e49895 jmp 0x141e49850
0x141e49897 test r14, r14
0x141e4989a je 0x141e498b1
0x141e4989c shl r14, 5
0x141e498a0 mov r8d, 8
0x141e498a6 mov rcx, r13
0x141e498a9 mov rdx, r14
0x141e498ac call 0x140613c20
0x141e498b1 mov byte ptr [rbp - 0x10], r12b
0x141e498b5 mov byte ptr [rbp - 0xf], sil
0x141e498b9 mov eax, dword ptr [rbp + 0x24c]
0x141e498bf mov dword ptr [rbp - 0xe], eax
0x141e498c2 movzx eax, word ptr [rbp + 0x250]
0x141e498c9 mov word ptr [rbp - 0xa], ax
0x141e498cd mov rax, qword ptr [rbp + 0x1b0]
0x141e498d4 mov qword ptr [rbp - 8], rax
0x141e498d8 movdqu xmmword ptr [rbp], xmm6
0x141e498dd movups xmmword ptr [rbp + 0x10], xmm7
0x141e498e1 movups xmmword ptr [rbp + 0x20], xmm8
0x141e498e6 xor eax, eax
0x141e498e8 test byte ptr [rbp + 0x227], 1
0x141e498ef mov r8, qword ptr [rbp + 0x90]
0x141e498f6 cmovne r8, rax
0x141e498fa mov rax, qword ptr [rbp + 0x1d0]
0x141e49901 mov qword ptr [rbp + 0x30], rax
0x141e49905 mov r9, qword ptr [rbp + 0x88]
0x141e4990c test r9, r9
0x141e4990f cmove r8, r9
0x141e49913 lea rcx, [rbp + 0x1d8]
0x141e4991a lea rdx, [rbp - 0x10]
0x141e4991e call 0x140bcd9f0
0x141e49923 movzx eax, word ptr [rbp + 0x1d8]
0x141e4992a movzx ecx, word ptr [rbp + 0x1de]
0x141e49931 mov word ptr [rbp + 0x3c], cx
0x141e49935 mov edx, dword ptr [rbp + 0x1da]
0x141e4993b mov r8, qword ptr [rbp + 0x1e0]
0x141e49942 mov r9, qword ptr [rbp + 0x218]
0x141e49949 movdqu xmm0, xmmword ptr [rbp + 0x1e8]
0x141e49951 movdqu xmm1, xmmword ptr [rbp + 0x1f8]
0x141e49959 movups xmm2, xmmword ptr [rbp + 0x208]
0x141e49960 mov dword ptr [rbp + 0xc8], edx
0x141e49966 mov word ptr [rbp + 0xcc], cx
0x141e4996d mov rsi, qword ptr [rbp + 0xd0]
0x141e49974 mov word ptr [rsi + 0xe], cx
0x141e49978 mov dword ptr [rsi + 0xa], edx
0x141e4997b movdqu xmmword ptr [rsi + 0x28], xmm1
0x141e49980 movups xmmword ptr [rsi + 0x38], xmm2
0x141e49984 mov qword ptr [rsi + 0x48], r9
0x141e49988 mov word ptr [rsi + 8], ax
0x141e4998c mov qword ptr [rsi + 0x10], r8
0x141e49990 movdqu xmmword ptr [rsi + 0x18], xmm0
0x141e49995 mov qword ptr [rsi], 1
0x141e4999c mov rax, rsi
0x141e4999f movaps xmm6, xmmword ptr [rbp + 0x260]
0x141e499a6 movaps xmm7, xmmword ptr [rbp + 0x270]
0x141e499ad movaps xmm8, xmmword ptr [rbp + 0x280]
0x141e499b5 movaps xmm9, xmmword ptr [rbp + 0x290]
0x141e499bd movaps xmm10, xmmword ptr [rbp + 0x2a0]
0x141e499c5 movaps xmm11, xmmword ptr [rbp + 0x2b0]
0x141e499cd movaps xmm12, xmmword ptr [rbp + 0x2c0]
0x141e499d5 movaps xmm13, xmmword ptr [rbp + 0x2d0]
0x141e499dd movaps xmm14, xmmword ptr [rbp + 0x2e0]
0x141e499e5 movaps xmm15, xmmword ptr [rbp + 0x2f0]
0x141e499ed add rsp, 0x388
0x141e499f4 pop rbx
0x141e499f5 pop rdi
0x141e499f6 pop rsi
0x141e499f7 pop r12
0x141e499f9 pop r13
0x141e499fb pop r14
0x141e499fd pop r15
0x141e499ff pop rbp
0x141e49a00 ret 
0x141e49a01 mov r12b, 0x2d
0x141e49a04 lea rax, [rip + 0x1145354]
0x141e49a0b mov qword ptr [rbp + 0x1b0], rax
0x141e49a12 mov r9, rbx
0x141e49a15 jmp 0x141e4928e
0x141e49a1a mov r12b, 0x2d
0x141e49a1d lea rax, [rip + 0x1145335]
0x141e49a24 mov qword ptr [rbp + 0x1b0], rax
0x141e49a2b movaps xmm6, xmmword ptr [rip + 0xb8092e]
0x141e49a32 jmp 0x141e4928e
0x141e49a37 xor ebx, ebx
0x141e49a39 movabs rax, 0x8000000000000001
0x141e49a43 mov qword ptr [rbp + 0x238], rax
0x141e49a4a mov qword ptr [rbp + 0x230], rdi
0x141e49a51 mov byte ptr [rbp + 0x257], 1
0x141e49a58 mov byte ptr [rbp + 0x256], 1
0x141e49a5f mov byte ptr [rbp + 0x255], 1
0x141e49a66 lea r8, [rip + 0x1208dcb]
0x141e49a6d mov rcx, rbx
0x141e49a70 mov rdx, qword ptr [rbp + 0x240]
0x141e49a77 call 0x1428d8fe3
0x141e49a7c ud2 
0x141e49a7e nop 
0x141e49a80 mov qword ptr [rsp + 0x10], rdx
0x141e49a85 push rbp
0x141e49a86 push r15
0x141e49a88 push r14
0x141e49a8a push r13
0x141e49a8c push r12
0x141e49a8e push rsi
0x141e49a8f push rdi
0x141e49a90 push rbx
0x141e49a91 sub rsp, 0xc8
0x141e49a98 lea rbp, [rdx + 0x80]
0x141e49a9f movaps xmmword ptr [rsp + 0x20], xmm15
0x141e49aa5 movdqa xmmword ptr [rsp + 0x30], xmm14
0x141e49aac movaps xmmword ptr [rsp + 0x40], xmm13
0x141e49ab2 movaps xmmword ptr [rsp + 0x50], xmm12
0x141e49ab8 movdqa xmmword ptr [rsp + 0x60], xmm11
0x141e49abf movaps xmmword ptr [rsp + 0x70], xmm10
0x141e49ac5 movaps xmmword ptr [rsp + 0x80], xmm9
0x141e49ace movaps xmmword ptr [rsp + 0x90], xmm8
0x141e49ad7 movaps xmmword ptr [rsp + 0xa0], xmm7
0x141e49adf movdqa xmmword ptr [rsp + 0xb0], xmm6
0x141e49ae8 cmp qword ptr [rbp + 0x240], 0
0x141e49af0 je 0x141e49b0b
0x141e49af2 mov r8d, 1
0x141e49af8 mov rcx, qword ptr [rbp + 0x198]
0x141e49aff mov rdx, qword ptr [rbp + 0x240]
0x141e49b06 call 0x140613c20
0x141e49b0b movabs rax, 0x8000000000000001
0x141e49b15 mov qword ptr [rbp + 0x238], rax
0x141e49b1c mov rax, qword ptr [rbp + 0x1c0]
0x141e49b23 mov qword ptr [rbp + 0x230], rax
0x141e49b2a mov byte ptr [rbp + 0x257], 1
0x141e49b31 mov byte ptr [rbp + 0x256], 1
0x141e49b38 mov byte ptr [rbp + 0x255], 1
0x141e49b3f movdqa xmm6, xmmword ptr [rsp + 0xb0]
0x141e49b48 movaps xmm7, xmmword ptr [rsp + 0xa0]
0x141e49b50 movaps xmm8, xmmword ptr [rsp + 0x90]
0x141e49b59 movaps xmm9, xmmword ptr [rsp + 0x80]
0x141e49b62 movaps xmm10, xmmword ptr [rsp + 0x70]
0x141e49b68 movdqa xmm11, xmmword ptr [rsp + 0x60]
0x141e49b6f movaps xmm12, xmmword ptr [rsp + 0x50]
0x141e49b75 movaps xmm13, xmmword ptr [rsp + 0x40]
0x141e49b7b movdqa xmm14, xmmword ptr [rsp + 0x30]
0x141e49b82 movaps xmm15, xmmword ptr [rsp + 0x20]
0x141e49b88 add rsp, 0xc8
0x141e49b8f pop rbx
0x141e49b90 pop rdi
0x141e49b91 pop rsi
0x141e49b92 pop r12
0x141e49b94 pop r13
0x141e49b96 pop r14
0x141e49b98 pop r15
0x141e49b9a pop rbp
0x141e49b9b ret 
0x141e49b9c nop dword ptr [rax]
0x141e49ba0 mov qword ptr [rsp + 0x10], rdx
0x141e49ba5 push rbp
0x141e49ba6 push r15
0x141e49ba8 push r14
0x141e49baa push r13
0x141e49bac push r12
0x141e49bae push rsi
0x141e49baf push rdi
0x141e49bb0 push rbx
0x141e49bb1 sub rsp, 0xc8
0x141e49bb8 lea rbp, [rdx + 0x80]
0x141e49bbf movaps xmmword ptr [rsp + 0x20], xmm15
0x141e49bc5 movdqa xmmword ptr [rsp + 0x30], xmm14
0x141e49bcc movaps xmmword ptr [rsp + 0x40], xmm13
0x141e49bd2 movaps xmmword ptr [rsp + 0x50], xmm12
0x141e49bd8 movdqa xmmword ptr [rsp + 0x60], xmm11
0x141e49bdf movaps xmmword ptr [rsp + 0x70], xmm10
0x141e49be5 movaps xmmword ptr [rsp + 0x80], xmm9
0x141e49bee movaps xmmword ptr [rsp + 0x90], xmm8
0x141e49bf7 movaps xmmword ptr [rsp + 0xa0], xmm7
0x141e49bff movdqa xmmword ptr [rsp + 0xb0], xmm6
0x141e49c08 cmp qword ptr [rbp + 0x1a0], 0
0x141e49c10 je 0x141e49c35
0x141e49c12 movdqa xmm0, xmmword ptr [rbp + 0xe0]
0x141e49c1a movq rcx, xmm0
0x141e49c1f mov rdx, qword ptr [rbp + 0x1a0]
0x141e49c26 shl rdx, 5
0x141e49c2a mov r8d, 4
0x141e49c30 call 0x140613c20
0x141e49c35 mov rax, qword ptr [rbp + 0x240]
0x141e49c3c mov qword ptr [rbp + 0x238], rax
0x141e49c43 movabs rax, 0x8000000000000000
0x141e49c4d mov qword ptr [rbp + 0x230], rax
0x141e49c54 mov byte ptr [rbp + 0x257], 1
0x141e49c5b mov byte ptr [rbp + 0x256], 1
0x141e49c62 mov byte ptr [rbp + 0x255], 1
0x141e49c69 movdqa xmm6, xmmword ptr [rsp + 0xb0]
0x141e49c72 movaps xmm7, xmmword ptr [rsp + 0xa0]
0x141e49c7a movaps xmm8, xmmword ptr [rsp + 0x90]
0x141e49c83 movaps xmm9, xmmword ptr [rsp + 0x80]
0x141e49c8c movaps xmm10, xmmword ptr [rsp + 0x70]
0x141e49c92 movdqa xmm11, xmmword ptr [rsp + 0x60]
0x141e49c99 movaps xmm12, xmmword ptr [rsp + 0x50]
0x141e49c9f movaps xmm13, xmmword ptr [rsp + 0x40]
0x141e49ca5 movdqa xmm14, xmmword ptr [rsp + 0x30]
0x141e49cac movaps xmm15, xmmword ptr [rsp + 0x20]
0x141e49cb2 add rsp, 0xc8
0x141e49cb9 pop rbx
0x141e49cba pop rdi
0x141e49cbb pop rsi
0x141e49cbc pop r12
0x141e49cbe pop r13
0x141e49cc0 pop r14
0x141e49cc2 pop r15
0x141e49cc4 pop rbp
0x141e49cc5 ret 
0x141e49cc6 nop word ptr cs:[rax + rax]
0x141e49cd0 mov qword ptr [rsp + 0x10], rdx
0x141e49cd5 push rbp
0x141e49cd6 push r15
0x141e49cd8 push r14
0x141e49cda push r13
0x141e49cdc push r12
0x141e49cde push rsi
0x141e49cdf push rdi
0x141e49ce0 push rbx
0x141e49ce1 sub rsp, 0xc8
0x141e49ce8 lea rbp, [rdx + 0x80]
0x141e49cef movaps xmmword ptr [rsp + 0x20], xmm15
0x141e49cf5 movdqa xmmword ptr [rsp + 0x30], xmm14
0x141e49cfc movaps xmmword ptr [rsp + 0x40], xmm13
0x141e49d02 movaps xmmword ptr [rsp + 0x50], xmm12
0x141e49d08 movdqa xmmword ptr [rsp + 0x60], xmm11
0x141e49d0f movaps xmmword ptr [rsp + 0x70], xmm10
0x141e49d15 movaps xmmword ptr [rsp + 0x80], xmm9
0x141e49d1e movaps xmmword ptr [rsp + 0x90], xmm8
0x141e49d27 movaps xmmword ptr [rsp + 0xa0], xmm7
0x141e49d2f movdqa xmmword ptr [rsp + 0xb0], xmm6
0x141e49d38 movzx eax, byte ptr [rbp + 0x253]
0x141e49d3f mov byte ptr [rbp + 0x190], al
0x141e49d45 movzx eax, byte ptr [rbp + 0x254]
0x141e49d4c mov byte ptr [rbp + 0x1cc], al
0x141e49d52 mov rax, qword ptr [rbp + 0x1c0]
0x141e49d59 neg rax
0x141e49d5c jno 0x141e49db8
0x141e49d5e movaps xmm6, xmmword ptr [rsp + 0xb0]
0x141e49d66 movaps xmm7, xmmword ptr [rsp + 0xa0]
0x141e49d6e movaps xmm8, xmmword ptr [rsp + 0x90]
0x141e49d77 movaps xmm9, xmmword ptr [rsp + 0x80]
0x141e49d80 movaps xmm10, xmmword ptr [rsp + 0x70]
0x141e49d86 movaps xmm11, xmmword ptr [rsp + 0x60]
0x141e49d8c movaps xmm12, xmmword ptr [rsp + 0x50]
0x141e49d92 movaps xmm13, xmmword ptr [rsp + 0x40]
0x141e49d98 movaps xmm14, xmmword ptr [rsp + 0x30]
0x141e49d9e movaps xmm15, xmmword ptr [rsp + 0x20]
0x141e49da4 add rsp, 0xc8
0x141e49dab pop rbx
0x141e49dac pop rdi
0x141e49dad pop rsi
0x141e49dae pop r12
0x141e49db0 pop r13
0x141e49db2 pop r14
0x141e49db4 pop r15
0x141e49db6 pop rbp
0x141e49db7 ret 
0x141e49db8 mov rdx, qword ptr [rbp + 0x1f0]
0x141e49dbf test rdx, rdx
0x141e49dc2 je 0x141e49dda
0x141e49dc4 mov rcx, qword ptr [rbp + 0x1f8]
0x141e49dcb shl rdx, 5
0x141e49dcf mov r8d, 4
0x141e49dd5 call 0x140613c20
0x141e49dda movaps xmm6, xmmword ptr [rsp + 0xb0]
0x141e49de2 movaps xmm7, xmmword ptr [rsp + 0xa0]
0x141e49dea movaps xmm8, xmmword ptr [rsp + 0x90]
0x141e49df3 movaps xmm9, xmmword ptr [rsp + 0x80]
0x141e49dfc movaps xmm10, xmmword ptr [rsp + 0x70]
0x141e49e02 movaps xmm11, xmmword ptr [rsp + 0x60]
0x141e49e08 movaps xmm12, xmmword ptr [rsp + 0x50]
0x141e49e0e movaps xmm13, xmmword ptr [rsp + 0x40]
0x141e49e14 movaps xmm14, xmmword ptr [rsp + 0x30]
0x141e49e1a movaps xmm15, xmmword ptr [rsp + 0x20]
0x141e49e20 add rsp, 0xc8
0x141e49e27 pop rbx
0x141e49e28 pop rdi
0x141e49e29 pop rsi
0x141e49e2a pop r12
0x141e49e2c pop r13
0x141e49e2e pop r14
0x141e49e30 pop r15
0x141e49e32 pop rbp
0x141e49e33 ret 
0x141e49e34 nop word ptr cs:[rax + rax]
0x141e49e40 mov qword ptr [rsp + 0x10], rdx
0x141e49e45 push rbp
0x141e49e46 push r15
0x141e49e48 push r14
0x141e49e4a push r13
0x141e49e4c push r12
0x141e49e4e push rsi
0x141e49e4f push rdi
0x141e49e50 push rbx
0x141e49e51 sub rsp, 0xc8
0x141e49e58 lea rbp, [rdx + 0x80]
0x141e49e5f movaps xmmword ptr [rsp + 0x20], xmm15
0x141e49e65 movaps xmmword ptr [rsp + 0x30], xmm14
0x141e49e6b movaps xmmword ptr [rsp + 0x40], xmm13
0x141e49e71 movaps xmmword ptr [rsp + 0x50], xmm12
0x141e49e77 movaps xmmword ptr [rsp + 0x60], xmm11
0x141e49e7d movaps xmmword ptr [rsp + 0x70], xmm10
0x141e49e83 movaps xmmword ptr [rsp + 0x80], xmm9
0x141e49e8c movaps xmmword ptr [rsp + 0x90], xmm8
0x141e49e95 movaps xmmword ptr [rsp + 0xa0], xmm7
0x141e49e9d movaps xmmword ptr [rsp + 0xb0], xmm6
0x141e49ea5 test byte ptr [rbp + 0x190], 1
0x141e49eac je 0x141e49f74
0x141e49eb2 mov rdx, qword ptr [rbp + 0x208]
0x141e49eb9 lea rax, [rdx*2]
0x141e49ec1 test rax, rax
0x141e49ec4 je 0x141e49ed8
0x141e49ec6 mov rcx, qword ptr [rbp + 0x210]
0x141e49ecd mov r8d, 1
0x141e49ed3 call 0x140613c20
0x141e49ed8 mov rax, qword ptr [rbp + 0x240]
0x141e49edf mov qword ptr [rbp + 0x238], rax
0x141e49ee6 mov rax, qword ptr [rbp + 0x1c0]
0x141e49eed mov qword ptr [rbp + 0x230], rax
0x141e49ef4 movzx eax, byte ptr [rbp + 0x1cc]
0x141e49efb and al, 1
0x141e49efd mov byte ptr [rbp + 0x257], al
0x141e49f03 movzx eax, byte ptr [rbp + 0x1d0]
0x141e49f0a mov byte ptr [rbp + 0x256], al
0x141e49f10 mov byte ptr [rbp + 0x255], 0
0x141e49f17 movdqa xmm6, xmmword ptr [rsp + 0xb0]
0x141e49f20 movaps xmm7, xmmword ptr [rsp + 0xa0]
0x141e49f28 movaps xmm8, xmmword ptr [rsp + 0x90]
0x141e49f31 movaps xmm9, xmmword ptr [rsp + 0x80]
0x141e49f3a movaps xmm10, xmmword ptr [rsp + 0x70]
0x141e49f40 movdqa xmm11, xmmword ptr [rsp + 0x60]
0x141e49f47 movaps xmm12, xmmword ptr [rsp + 0x50]
0x141e49f4d movaps xmm13, xmmword ptr [rsp + 0x40]
0x141e49f53 movdqa xmm14, xmmword ptr [rsp + 0x30]
0x141e49f5a movaps xmm15, xmmword ptr [rsp + 0x20]
0x141e49f60 add rsp, 0xc8
0x141e49f67 pop rbx
0x141e49f68 pop rdi
0x141e49f69 pop rsi
0x141e49f6a pop r12
0x141e49f6c pop r13
0x141e49f6e pop r14
0x141e49f70 pop r15
0x141e49f72 pop rbp
0x141e49f73 ret 
0x141e49f74 mov rax, qword ptr [rbp + 0x240]
0x141e49f7b mov qword ptr [rbp + 0x238], rax
0x141e49f82 mov rax, qword ptr [rbp + 0x1c0]
0x141e49f89 mov qword ptr [rbp + 0x230], rax
0x141e49f90 movzx eax, byte ptr [rbp + 0x1cc]
0x141e49f97 and al, 1
0x141e49f99 mov byte ptr [rbp + 0x257], al
0x141e49f9f movzx eax, byte ptr [rbp + 0x1d0]
0x141e49fa6 mov byte ptr [rbp + 0x256], al
0x141e49fac mov byte ptr [rbp + 0x255], 0
0x141e49fb3 movdqa xmm6, xmmword ptr [rsp + 0xb0]
0x141e49fbc movaps xmm7, xmmword ptr [rsp + 0xa0]
0x141e49fc4 movaps xmm8, xmmword ptr [rsp + 0x90]
0x141e49fcd movaps xmm9, xmmword ptr [rsp + 0x80]
0x141e49fd6 movaps xmm10, xmmword ptr [rsp + 0x70]
0x141e49fdc movdqa xmm11, xmmword ptr [rsp + 0x60]
0x141e49fe3 movaps xmm12, xmmword ptr [rsp + 0x50]
0x141e49fe9 movaps xmm13, xmmword ptr [rsp + 0x40]
0x141e49fef movdqa xmm14, xmmword ptr [rsp + 0x30]
0x141e49ff6 movaps xmm15, xmmword ptr [rsp + 0x20]
0x141e49ffc add rsp, 0xc8
0x141e4a003 pop rbx
0x141e4a004 pop rdi
0x141e4a005 pop rsi
0x141e4a006 pop r12
0x141e4a008 pop r13
0x141e4a00a pop r14
0x141e4a00c pop r15
0x141e4a00e pop rbp
0x141e4a00f ret 
0x141e4a010 mov qword ptr [rsp + 0x10], rdx
0x141e4a015 push rbp
0x141e4a016 push r15
0x141e4a018 push r14
0x141e4a01a push r13
0x141e4a01c push r12
0x141e4a01e push rsi
0x141e4a01f push rdi
0x141e4a020 push rbx
0x141e4a021 sub rsp, 0xc8
0x141e4a028 lea rbp, [rdx + 0x80]
0x141e4a02f movaps xmmword ptr [rsp + 0x20], xmm15
0x141e4a035 movdqa xmmword ptr [rsp + 0x30], xmm14
0x141e4a03c movaps xmmword ptr [rsp + 0x40], xmm13
0x141e4a042 movaps xmmword ptr [rsp + 0x50], xmm12
0x141e4a048 movdqa xmmword ptr [rsp + 0x60], xmm11
0x141e4a04f movaps xmmword ptr [rsp + 0x70], xmm10
0x141e4a055 movaps xmmword ptr [rsp + 0x80], xmm9
0x141e4a05e movaps xmmword ptr [rsp + 0x90], xmm8
0x141e4a067 movaps xmmword ptr [rsp + 0xa0], xmm7
0x141e4a06f movdqa xmmword ptr [rsp + 0xb0], xmm6
0x141e4a078 lea rcx, [rbp + 0x40]
0x141e4a07c call 0x140fa6cf0
0x141e4a081 movaps xmm6, xmmword ptr [rsp + 0xb0]
0x141e4a089 movaps xmm7, xmmword ptr [rsp + 0xa0]
0x141e4a091 movaps xmm8, xmmword ptr [rsp + 0x90]
0x141e4a09a movaps xmm9, xmmword ptr [rsp + 0x80]
0x141e4a0a3 movaps xmm10, xmmword ptr [rsp + 0x70]
0x141e4a0a9 movaps xmm11, xmmword ptr [rsp + 0x60]
0x141e4a0af movaps xmm12, xmmword ptr [rsp + 0x50]
0x141e4a0b5 movaps xmm13, xmmword ptr [rsp + 0x40]
0x141e4a0bb movaps xmm14, xmmword ptr [rsp + 0x30]
0x141e4a0c1 movaps xmm15, xmmword ptr [rsp + 0x20]
0x141e4a0c7 add rsp, 0xc8
0x141e4a0ce pop rbx
0x141e4a0cf pop rdi
0x141e4a0d0 pop rsi
0x141e4a0d1 pop r12
0x141e4a0d3 pop r13
0x141e4a0d5 pop r14
0x141e4a0d7 pop r15
0x141e4a0d9 pop rbp
0x141e4a0da ret 
0x141e4a0db nop dword ptr [rax + rax]
0x141e4a0e0 mov qword ptr [rsp + 0x10], rdx
0x141e4a0e5 push rbp
0x141e4a0e6 push r15
0x141e4a0e8 push r14
0x141e4a0ea push r13
0x141e4a0ec push r12
0x141e4a0ee push rsi
0x141e4a0ef push rdi
0x141e4a0f0 push rbx
0x141e4a0f1 sub rsp, 0xc8
0x141e4a0f8 lea rbp, [rdx + 0x80]
0x141e4a0ff movaps xmmword ptr [rsp + 0x20], xmm15
0x141e4a105 movdqa xmmword ptr [rsp + 0x30], xmm14
0x141e4a10c movaps xmmword ptr [rsp + 0x40], xmm13
0x141e4a112 movaps xmmword ptr [rsp + 0x50], xmm12
0x141e4a118 movdqa xmmword ptr [rsp + 0x60], xmm11
0x141e4a11f movaps xmmword ptr [rsp + 0x70], xmm10
0x141e4a125 movaps xmmword ptr [rsp + 0x80], xmm9
0x141e4a12e movaps xmmword ptr [rsp + 0x90], xmm8
0x141e4a137 movaps xmmword ptr [rsp + 0xa0], xmm7
0x141e4a13f movdqa xmmword ptr [rsp + 0xb0], xmm6
0x141e4a148 mov rdi, qword ptr [rbp + 0x240]
0x141e4a14f mov qword ptr [rbp + 0x238], rdi
0x141e4a156 mov rsi, qword ptr [rbp + 0x1c0]
0x141e4a15d mov qword ptr [rbp + 0x230], rsi
0x141e4a164 mov byte ptr [rbp + 0x257], 1
0x141e4a16b mov byte ptr [rbp + 0x256], 1
0x141e4a172 mov byte ptr [rbp + 0x255], 1
0x141e4a179 lea rcx, [rbp + 0x110]
0x141e4a180 call 0x1405f11f0
0x141e4a185 mov qword ptr [rbp + 0x238], rdi
0x141e4a18c mov qword ptr [rbp + 0x230], rsi
0x141e4a193 mov byte ptr [rbp + 0x257], 1
0x141e4a19a mov byte ptr [rbp + 0x256], 1
0x141e4a1a1 mov byte ptr [rbp + 0x255], 1
0x141e4a1a8 movdqa xmm6, xmmword ptr [rsp + 0xb0]
0x141e4a1b1 movaps xmm7, xmmword ptr [rsp + 0xa0]
0x141e4a1b9 movaps xmm8, xmmword ptr [rsp + 0x90]
0x141e4a1c2 movaps xmm9, xmmword ptr [rsp + 0x80]
0x141e4a1cb movaps xmm10, xmmword ptr [rsp + 0x70]
0x141e4a1d1 movdqa xmm11, xmmword ptr [rsp + 0x60]
0x141e4a1d8 movaps xmm12, xmmword ptr [rsp + 0x50]
0x141e4a1de movaps xmm13, xmmword ptr [rsp + 0x40]
0x141e4a1e4 movdqa xmm14, xmmword ptr [rsp + 0x30]
0x141e4a1eb movaps xmm15, xmmword ptr [rsp + 0x20]
0x141e4a1f1 add rsp, 0xc8
0x141e4a1f8 pop rbx
0x141e4a1f9 pop rdi
0x141e4a1fa pop rsi
0x141e4a1fb pop r12
0x141e4a1fd pop r13
0x141e4a1ff pop r14
0x141e4a201 pop r15
0x141e4a203 pop rbp
0x141e4a204 ret 
0x141e4a205 nop word ptr cs:[rax + rax]
0x141e4a210 mov qword ptr [rsp + 0x10], rdx
0x141e4a215 push rbp
0x141e4a216 push r15
0x141e4a218 push r14
0x141e4a21a push r13
0x141e4a21c push r12
0x141e4a21e push rsi
0x141e4a21f push rdi
0x141e4a220 push rbx
0x141e4a221 sub rsp, 0xc8
0x141e4a228 lea rbp, [rdx + 0x80]
0x141e4a22f movaps xmmword ptr [rsp + 0x20], xmm15
0x141e4a235 movdqa xmmword ptr [rsp + 0x30], xmm14
0x141e4a23c movaps xmmword ptr [rsp + 0x40], xmm13
0x141e4a242 movaps xmmword ptr [rsp + 0x50], xmm12
0x141e4a248 movdqa xmmword ptr [rsp + 0x60], xmm11
0x141e4a24f movaps xmmword ptr [rsp + 0x70], xmm10
0x141e4a255 movaps xmmword ptr [rsp + 0x80], xmm9
0x141e4a25e movaps xmmword ptr [rsp + 0x90], xmm8
0x141e4a267 movaps xmmword ptr [rsp + 0xa0], xmm7
0x141e4a26f movdqa xmmword ptr [rsp + 0xb0], xmm6
0x141e4a278 movzx eax, byte ptr [rbp + 0x255]
0x141e4a27f mov byte ptr [rbp + 0x1cc], al
0x141e4a285 movzx eax, byte ptr [rbp + 0x256]
0x141e4a28c mov byte ptr [rbp + 0x240], al
0x141e4a292 mov rax, qword ptr [rbp + 0x230]
0x141e4a299 mov qword ptr [rbp + 0x1d0], rax
0x141e4a2a0 mov rdx, qword ptr [rbp + 0x238]
0x141e4a2a7 movabs rax, 0x8000000000000001
0x141e4a2b1 cmp rdx, rax
0x141e4a2b4 jne 0x141e4a310
0x141e4a2b6 movaps xmm6, xmmword ptr [rsp + 0xb0]
0x141e4a2be movaps xmm7, xmmword ptr [rsp + 0xa0]
0x141e4a2c6 movaps xmm8, xmmword ptr [rsp + 0x90]
0x141e4a2cf movaps xmm9, xmmword ptr [rsp + 0x80]
0x141e4a2d8 movaps xmm10, xmmword ptr [rsp + 0x70]
0x141e4a2de movaps xmm11, xmmword ptr [rsp + 0x60]
0x141e4a2e4 movaps xmm12, xmmword ptr [rsp + 0x50]
0x141e4a2ea movaps xmm13, xmmword ptr [rsp + 0x40]
0x141e4a2f0 movaps xmm14, xmmword ptr [rsp + 0x30]
0x141e4a2f6 movaps xmm15, xmmword ptr [rsp + 0x20]
0x141e4a2fc add rsp, 0xc8
0x141e4a303 pop rbx
0x141e4a304 pop rdi
0x141e4a305 pop rsi
0x141e4a306 pop r12
0x141e4a308 pop r13
0x141e4a30a pop r14
0x141e4a30c pop r15
0x141e4a30e pop rbp
0x141e4a30f ret 
0x141e4a310 test byte ptr [rbp + 0x257], 1
0x141e4a317 je 0x141e4a398
0x141e4a319 lea rax, [rdx*2]
0x141e4a321 test rax, rax
0x141e4a324 je 0x141e4a33e
0x141e4a326 movdqa xmm0, xmmword ptr [rbp + 0x100]
0x141e4a32e movq rcx, xmm0
0x141e4a333 mov r8d, 1
0x141e4a339 call 0x140613c20
0x141e4a33e movaps xmm6, xmmword ptr [rsp + 0xb0]
0x141e4a346 movaps xmm7, xmmword ptr [rsp + 0xa0]
0x141e4a34e movaps xmm8, xmmword ptr [rsp + 0x90]
0x141e4a357 movaps xmm9, xmmword ptr [rsp + 0x80]
0x141e4a360 movaps xmm10, xmmword ptr [rsp + 0x70]
0x141e4a366 movaps xmm11, xmmword ptr [rsp + 0x60]
0x141e4a36c movaps xmm12, xmmword ptr [rsp + 0x50]
0x141e4a372 movaps xmm13, xmmword ptr [rsp + 0x40]
0x141e4a378 movaps xmm14, xmmword ptr [rsp + 0x30]
0x141e4a37e movaps xmm15, xmmword ptr [rsp + 0x20]
0x141e4a384 add rsp, 0xc8
0x141e4a38b pop rbx
0x141e4a38c pop rdi
0x141e4a38d pop rsi
0x141e4a38e pop r12
0x141e4a390 pop r13
0x141e4a392 pop r14
0x141e4a394 pop r15
0x141e4a396 pop rbp
0x141e4a397 ret 
0x141e4a398 movaps xmm6, xmmword ptr [rsp + 0xb0]
0x141e4a3a0 movaps xmm7, xmmword ptr [rsp + 0xa0]
0x141e4a3a8 movaps xmm8, xmmword ptr [rsp + 0x90]
0x141e4a3b1 movaps xmm9, xmmword ptr [rsp + 0x80]
0x141e4a3ba movaps xmm10, xmmword ptr [rsp + 0x70]
0x141e4a3c0 movaps xmm11, xmmword ptr [rsp + 0x60]
0x141e4a3c6 movaps xmm12, xmmword ptr [rsp + 0x50]
0x141e4a3cc movaps xmm13, xmmword ptr [rsp + 0x40]
0x141e4a3d2 movaps xmm14, xmmword ptr [rsp + 0x30]
0x141e4a3d8 movaps xmm15, xmmword ptr [rsp + 0x20]
0x141e4a3de add rsp, 0xc8
0x141e4a3e5 pop rbx
0x141e4a3e6 pop rdi
0x141e4a3e7 pop rsi
0x141e4a3e8 pop r12
0x141e4a3ea pop r13
0x141e4a3ec pop r14
0x141e4a3ee pop r15
0x141e4a3f0 pop rbp
0x141e4a3f1 ret 
0x141e4a3f2 nop word ptr cs:[rax + rax]
0x141e4a400 mov qword ptr [rsp + 0x10], rdx
0x141e4a405 push rbp
0x141e4a406 push r15
0x141e4a408 push r14
0x141e4a40a push r13
0x141e4a40c push r12
0x141e4a40e push rsi
0x141e4a40f push rdi
0x141e4a410 push rbx
0x141e4a411 sub rsp, 0xc8
0x141e4a418 lea rbp, [rdx + 0x80]
0x141e4a41f movaps xmmword ptr [rsp + 0x20], xmm15
0x141e4a425 movaps xmmword ptr [rsp + 0x30], xmm14
0x141e4a42b movaps xmmword ptr [rsp + 0x40], xmm13
0x141e4a431 movaps xmmword ptr [rsp + 0x50], xmm12
0x141e4a437 movaps xmmword ptr [rsp + 0x60], xmm11
0x141e4a43d movaps xmmword ptr [rsp + 0x70], xmm10
0x141e4a443 movaps xmmword ptr [rsp + 0x80], xmm9
0x141e4a44c movaps xmmword ptr [rsp + 0x90], xmm8
0x141e4a455 movaps xmmword ptr [rsp + 0xa0], xmm7
0x141e4a45d movaps xmmword ptr [rsp + 0xb0], xmm6
0x141e4a465 mov rax, qword ptr [rbp + 0x1d0]
0x141e4a46c neg rax
0x141e4a46f jno 0x141e4a4cb
0x141e4a471 movaps xmm6, xmmword ptr [rsp + 0xb0]
0x141e4a479 movaps xmm7, xmmword ptr [rsp + 0xa0]
0x141e4a481 movaps xmm8, xmmword ptr [rsp + 0x90]
0x141e4a48a movaps xmm9, xmmword ptr [rsp + 0x80]
0x141e4a493 movaps xmm10, xmmword ptr [rsp + 0x70]
0x141e4a499 movaps xmm11, xmmword ptr [rsp + 0x60]
0x141e4a49f movaps xmm12, xmmword ptr [rsp + 0x50]
0x141e4a4a5 movaps xmm13, xmmword ptr [rsp + 0x40]
0x141e4a4ab movaps xmm14, xmmword ptr [rsp + 0x30]
0x141e4a4b1 movaps xmm15, xmmword ptr [rsp + 0x20]
0x141e4a4b7 add rsp, 0xc8
0x141e4a4be pop rbx
0x141e4a4bf pop rdi
0x141e4a4c0 pop rsi
0x141e4a4c1 pop r12
0x141e4a4c3 pop r13
0x141e4a4c5 pop r14
0x141e4a4c7 pop r15
0x141e4a4c9 pop rbp
0x141e4a4ca ret 
0x141e4a4cb test byte ptr [rbp + 0x240], 1
0x141e4a4d2 je 0x141e4a55f
0x141e4a4d8 cmp qword ptr [rbp + 0x1d0], 0
0x141e4a4e0 je 0x141e4a505
0x141e4a4e2 movdqa xmm0, xmmword ptr [rbp + 0xf0]
0x141e4a4ea movq rcx, xmm0
0x141e4a4ef mov rdx, qword ptr [rbp + 0x1d0]
0x141e4a4f6 shl rdx, 5
0x141e4a4fa mov r8d, 4
0x141e4a500 call 0x140613c20
0x141e4a505 movaps xmm6, xmmword ptr [rsp + 0xb0]
0x141e4a50d movaps xmm7, xmmword ptr [rsp + 0xa0]
0x141e4a515 movaps xmm8, xmmword ptr [rsp + 0x90]
0x141e4a51e movaps xmm9, xmmword ptr [rsp + 0x80]
0x141e4a527 movaps xmm10, xmmword ptr [rsp + 0x70]
0x141e4a52d movaps xmm11, xmmword ptr [rsp + 0x60]
0x141e4a533 movaps xmm12, xmmword ptr [rsp + 0x50]
0x141e4a539 movaps xmm13, xmmword ptr [rsp + 0x40]
0x141e4a53f movaps xmm14, xmmword ptr [rsp + 0x30]
0x141e4a545 movaps xmm15, xmmword ptr [rsp + 0x20]
0x141e4a54b add rsp, 0xc8
0x141e4a552 pop rbx
0x141e4a553 pop rdi
0x141e4a554 pop rsi
0x141e4a555 pop r12
0x141e4a557 pop r13
0x141e4a559 pop r14
0x141e4a55b pop r15
0x141e4a55d pop rbp
0x141e4a55e ret 
0x141e4a55f movaps xmm6, xmmword ptr [rsp + 0xb0]
0x141e4a567 movaps xmm7, xmmword ptr [rsp + 0xa0]
0x141e4a56f movaps xmm8, xmmword ptr [rsp + 0x90]
0x141e4a578 movaps xmm9, xmmword ptr [rsp + 0x80]
0x141e4a581 movaps xmm10, xmmword ptr [rsp + 0x70]
0x141e4a587 movaps xmm11, xmmword ptr [rsp + 0x60]
0x141e4a58d movaps xmm12, xmmword ptr [rsp + 0x50]
0x141e4a593 movaps xmm13, xmmword ptr [rsp + 0x40]
0x141e4a599 movaps xmm14, xmmword ptr [rsp + 0x30]
0x141e4a59f movaps xmm15, xmmword ptr [rsp + 0x20]
0x141e4a5a5 add rsp, 0xc8
0x141e4a5ac pop rbx
0x141e4a5ad pop rdi
0x141e4a5ae pop rsi
0x141e4a5af pop r12
0x141e4a5b1 pop r13
0x141e4a5b3 pop r14
0x141e4a5b5 pop r15
0x141e4a5b7 pop rbp
0x141e4a5b8 ret 
0x141e4a5b9 nop dword ptr [rax]
0x141e4a5c0 mov qword ptr [rsp + 0x10], rdx
0x141e4a5c5 push rbp
0x141e4a5c6 push r15
0x141e4a5c8 push r14
0x141e4a5ca push r13
0x141e4a5cc push r12
0x141e4a5ce push rsi
0x141e4a5cf push rdi
0x141e4a5d0 push rbx
0x141e4a5d1 sub rsp, 0xc8
0x141e4a5d8 lea rbp, [rdx + 0x80]
0x141e4a5df movaps xmmword ptr [rsp + 0x20], xmm15
0x141e4a5e5 movaps xmmword ptr [rsp + 0x30], xmm14
0x141e4a5eb movaps xmmword ptr [rsp + 0x40], xmm13
0x141e4a5f1 movaps xmmword ptr [rsp + 0x50], xmm12
0x141e4a5f7 movaps xmmword ptr [rsp + 0x60], xmm11
0x141e4a5fd movaps xmmword ptr [rsp + 0x70], xmm10
0x141e4a603 movaps xmmword ptr [rsp + 0x80], xmm9
0x141e4a60c movaps xmmword ptr [rsp + 0x90], xmm8
0x141e4a615 movaps xmmword ptr [rsp + 0xa0], xmm7
0x141e4a61d movaps xmmword ptr [rsp + 0xb0], xmm6
0x141e4a625 xor eax, eax
0x141e4a627 cmp rax, qword ptr [rbp + 0x168]
0x141e4a62e jo 0x141e4a639
0x141e4a630 test byte ptr [rbp + 0x1cc], 1
0x141e4a637 jne 0x141e4a693
0x141e4a639 movaps xmm6, xmmword ptr [rsp + 0xb0]
0x141e4a641 movaps xmm7, xmmword ptr [rsp + 0xa0]
0x141e4a649 movaps xmm8, xmmword ptr [rsp + 0x90]
0x141e4a652 movaps xmm9, xmmword ptr [rsp + 0x80]
0x141e4a65b movaps xmm10, xmmword ptr [rsp + 0x70]
0x141e4a661 movaps xmm11, xmmword ptr [rsp + 0x60]
0x141e4a667 movaps xmm12, xmmword ptr [rsp + 0x50]
0x141e4a66d movaps xmm13, xmmword ptr [rsp + 0x40]
0x141e4a673 movaps xmm14, xmmword ptr [rsp + 0x30]
0x141e4a679 movaps xmm15, xmmword ptr [rsp + 0x20]
0x141e4a67f add rsp, 0xc8
0x141e4a686 pop rbx
0x141e4a687 pop rdi
0x141e4a688 pop rsi
0x141e4a689 pop r12
0x141e4a68b pop r13
0x141e4a68d pop r14
0x141e4a68f pop r15
0x141e4a691 pop rbp
0x141e4a692 ret 
0x141e4a693 lea rcx, [rbp + 0x168]
0x141e4a69a call 0x1405f11f0
0x141e4a69f jmp 0x141e4a639