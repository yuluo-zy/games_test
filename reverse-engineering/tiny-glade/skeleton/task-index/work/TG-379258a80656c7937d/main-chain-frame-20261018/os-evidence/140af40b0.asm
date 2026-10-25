0x140af40b0 push rbp
0x140af40b1 push r15
0x140af40b3 push r14
0x140af40b5 push r13
0x140af40b7 push r12
0x140af40b9 push rsi
0x140af40ba push rdi
0x140af40bb push rbx
0x140af40bc mov eax, 0x31e8
0x140af40c1 call 0x14289aa40
0x140af40c6 sub rsp, rax
0x140af40c9 lea rbp, [rsp + 0x80]
0x140af40d1 movaps xmmword ptr [rbp + 0x3150], xmm15
0x140af40d9 movaps xmmword ptr [rbp + 0x3140], xmm14
0x140af40e1 movaps xmmword ptr [rbp + 0x3130], xmm13
0x140af40e9 movaps xmmword ptr [rbp + 0x3120], xmm12
0x140af40f1 movaps xmmword ptr [rbp + 0x3110], xmm11
0x140af40f9 movaps xmmword ptr [rbp + 0x3100], xmm10
0x140af4101 movdqa xmmword ptr [rbp + 0x30f0], xmm9
0x140af410a movaps xmmword ptr [rbp + 0x30e0], xmm8
0x140af4112 movdqa xmmword ptr [rbp + 0x30d0], xmm7
0x140af411a movaps xmmword ptr [rbp + 0x30c0], xmm6
0x140af4121 mov qword ptr [rbp + 0x30b8], 0xfffffffffffffffe
0x140af412c mov qword ptr [rbp + 0x3068], r9
0x140af4133 mov qword ptr [rbp + 0x3018], r8
0x140af413a mov r12, rdx
0x140af413d mov r14, rcx
0x140af4140 mov r15, qword ptr [rcx + 0x20]
0x140af4144 mov rax, qword ptr [rcx + 0x40]
0x140af4148 mov qword ptr [rbp + 0x3048], rax
0x140af414f mov rax, qword ptr [rcx + 0x60]
0x140af4153 mov qword ptr [rbp + 0x3090], rax
0x140af415a mov rax, qword ptr [rcx + 0x80]
0x140af4161 mov qword ptr [rbp + 0x3070], rax
0x140af4168 mov rax, qword ptr [rcx + 0xa0]
0x140af416f mov qword ptr [rbp + 0x3098], rax
0x140af4176 mov rdi, qword ptr [rcx + 0xc0]
0x140af417d mov rbx, qword ptr [rcx + 0xe0]
0x140af4184 mov rax, qword ptr [rcx + 0x100]
0x140af418b mov qword ptr [rbp + 0x3078], rax
0x140af4192 movzx esi, byte ptr [r15 + 0xc0]
0x140af419a shl esi, 6
0x140af419d movzx r13d, byte ptr [r15 + rsi + 0x28]
0x140af41a3 mov rax, qword ptr [rcx + 0x120]
0x140af41aa mov qword ptr [rbp + 0x30a8], rax
0x140af41b1 mov rax, qword ptr [rcx + 0x140]
0x140af41b8 mov qword ptr [rbp + 0x3080], rax
0x140af41bf mov rax, qword ptr [rcx + 0x160]
0x140af41c6 mov qword ptr [rbp + 0x3050], rax
0x140af41cd mov rax, qword ptr [rcx + 0x180]
0x140af41d4 mov qword ptr [rbp + 0x3030], rax
0x140af41db mov rax, qword ptr [rcx + 0x1a0]
0x140af41e2 mov qword ptr [rbp + 0x3020], rax
0x140af41e9 test r13b, r13b
0x140af41ec je 0x140af425d
0x140af41ee movzx eax, byte ptr [rip + 0x2b40a14] ; 0x143634c09 5a1903301400006a140000745a190370 f32=4.769354911005053e-10
0x140af41f5 mov ecx, 7
0x140af41fa mov edx, 1
0x140af41ff call 0x140613c10
0x140af4204 test rax, rax
0x140af4207 je 0x140af4f37
0x140af420d mov ecx, dword ptr [rip + 0x1ff29df] ; 0x142ae6bf2 61756c746173736574732f6c7574732f f32=7.4936693461279935e+31
0x140af4213 mov dword ptr [rax + 3], ecx
0x140af4216 mov ecx, dword ptr [rip + 0x1ff29d3] ; 0x142ae6bef 64656661756c746173736574732f6c75 f32=2.6562856884054367e+20
0x140af421c mov dword ptr [rax], ecx
0x140af421e mov byte ptr [rbp + 0x3010], 0
0x140af4225 mov qword ptr [rbp + 0x2fe8], 7
0x140af4230 mov qword ptr [rbp + 0x2ff0], rax
0x140af4237 mov qword ptr [rbp + 0x2ff8], 7
0x140af4242 movaps xmm8, xmmword ptr [rip + 0x1fa7be6] ; 0x142a9be30 0000803f000000000000803f0000803f f32=1.0
0x140af424a movups xmmword ptr [rbp + 0x3000], xmm8
0x140af4252 cmp dword ptr [rbx], 2
0x140af4255 mov r8, qword ptr [r12]
0x140af4259 jae 0x140af428c
0x140af425b jmp 0x140af42a5
0x140af425d add rsi, r15
0x140af4260 lea rcx, [rbp + 0x2fe8]
0x140af4267 mov rdx, rsi
0x140af426a call 0x140d7bd90
0x140af426f movups xmm8, xmmword ptr [rsi + 0x18]
0x140af4274 mov byte ptr [rbp + 0x3010], r13b
0x140af427b movups xmmword ptr [rbp + 0x3000], xmm8
0x140af4283 cmp dword ptr [rbx], 2
0x140af4286 mov r8, qword ptr [r12]
0x140af428a jb 0x140af42a5
0x140af428c mov eax, dword ptr [rbx + 4]
0x140af428f mov ecx, dword ptr [r12 + 0x1c]
0x140af4294 mov rdx, qword ptr [r12 + 0x10]
0x140af4299 mov dword ptr [rdx], ecx
0x140af429b and eax, 3
0x140af429e mov dword ptr [r8 + 0x108], eax
0x140af42a5 cmp byte ptr [rip + 0x2b37df4], 0 ; 0x14362c0a0 0100000000000000a0c0624301000000 f32=1.401298464324817e-45
0x140af42ac mov qword ptr [rbp + 0x3088], r8
0x140af42b3 je 0x140af42ea
0x140af42b5 mov rcx, qword ptr [rdi + 0x10]
0x140af42b9 test rcx, rcx
0x140af42bc je 0x140af4f4d
0x140af42c2 mov r13d, dword ptr [r8 + 0x108]
0x140af42c9 mov rdx, r13
0x140af42cc cmp r13, rcx
0x140af42cf jb 0x140af42d8
0x140af42d1 mov eax, r13d
0x140af42d4 xor edx, edx
0x140af42d6 div ecx
0x140af42d8 mov rax, qword ptr [rdi + 8]
0x140af42dc movsd xmm6, qword ptr [rax + rdx*8]
0x140af42e1 mov rbx, qword ptr [rbp + 0x3088]
0x140af42e8 jmp 0x140af42f7
0x140af42ea mov r13d, dword ptr [r8 + 0x108]
0x140af42f1 xorps xmm6, xmm6
0x140af42f4 mov rbx, r8
0x140af42f7 lea rsi, [rbp + 0x1be0]
0x140af42fe mov r8d, 0x100
0x140af4304 mov rcx, rsi
0x140af4307 mov rdi, qword ptr [rbp + 0x3090]
0x140af430e mov rdx, rdi
0x140af4311 call 0x142923c00
0x140af4316 test r13d, r13d
0x140af4319 mov rdx, rbx
0x140af431c cmove rdx, rdi
0x140af4320 lea rcx, [rbp + 0x1ce0]
0x140af4327 mov r8d, 0x100
0x140af432d call 0x142923c00
0x140af4332 mov rax, qword ptr [rbp + 0x3098]
0x140af4339 mov rax, qword ptr [rax + 0x10]
0x140af433d mov dword ptr [rbp + 0x1de0], eax
0x140af4343 shr rax, 0x20
0x140af4347 mov dword ptr [rbp + 0x1de4], eax
0x140af434d pxor xmm0, xmm0
0x140af4351 movdqu xmmword ptr [rbp + 0x1de8], xmm0
0x140af4359 lea rcx, [rbp + 0x920]
0x140af4360 mov rdx, rsi
0x140af4363 call 0x140b18f60
0x140af4368 mov r8d, 0x100
0x140af436e mov rcx, rsi
0x140af4371 mov rdx, qword ptr [rbp + 0x3090]
0x140af4378 call 0x142923c00
0x140af437d mov edi, dword ptr [r12 + 0x1c]
0x140af4382 mov rbx, qword ptr [r12 + 0x10]
0x140af4387 mov dword ptr [rbx], edi
0x140af4389 mov r8d, 0x100
0x140af438f mov r12, qword ptr [rbp + 0x3088]
0x140af4396 mov rcx, r12
0x140af4399 mov rdx, rsi
0x140af439c call 0x142923c00
0x140af43a1 movd xmm7, dword ptr [r12 + 0x100]
0x140af43ab movd xmm9, dword ptr [r12 + 0x104]
0x140af43b5 mov dword ptr [rbx], edi
0x140af43b7 movlps qword ptr [r12 + 0x100], xmm6
0x140af43c0 lea rcx, [rbp + 0x600]
0x140af43c7 lea rsi, [rbp + 0x920]
0x140af43ce mov r8d, 0x320
0x140af43d4 mov rdx, rsi
0x140af43d7 call 0x142923c00
0x140af43dc mov rax, qword ptr [rbp + 0x3098]
0x140af43e3 mov rax, qword ptr [rax + 0x10]
0x140af43e7 mov rcx, rax
0x140af43ea shr rcx, 0x20
0x140af43ee movshdup xmm2, xmm6
0x140af43f2 mov dword ptr [rsp + 0x30], ecx
0x140af43f6 mov dword ptr [rsp + 0x28], eax
0x140af43fa movd dword ptr [rsp + 0x20], xmm9
0x140af4401 mov rcx, rsi
0x140af4404 movaps xmm1, xmm6
0x140af4407 movdqa xmm3, xmm7
0x140af440b call 0x140b18af0
0x140af4410 movzx eax, byte ptr [r15 + 0xc0]
0x140af4418 shl eax, 6
0x140af441b movzx edx, byte ptr [r15 + rax + 0x3c]
0x140af4421 movzx eax, byte ptr [r15 + rax + 0x3d]
0x140af4427 mov byte ptr [rbp + 0x30a4], al
0x140af442d mov rax, qword ptr [rbp + 0x3068]
0x140af4434 mov rsi, qword ptr [rax]
0x140af4437 mov r8, qword ptr [rax + 8]
0x140af443b mov r11, qword ptr [rsi]
0x140af443e xor r9d, r9d
0x140af4441 mov rcx, r11
0x140af4444 sub rcx, qword ptr [r8 + 0x18]
0x140af4448 cmovb rcx, r9
0x140af444c sub r11, qword ptr [r8 + 0x38]
0x140af4450 cmovb r11, r9
0x140af4454 lea rax, [rcx + rcx*2]
0x140af4458 shl rax, 3
0x140af445c add rax, qword ptr [r8 + 8]
0x140af4460 mov r10, qword ptr [r8 + 0x10]
0x140af4464 sub r10, rcx
0x140af4467 cmovb r10, r9
0x140af446b mov rcx, qword ptr [rbp + 0x30a8]
0x140af4472 movss xmm11, dword ptr [rcx + 0xa0]
0x140af447b mov edi, 8
0x140af4480 cmovb rax, rdi
0x140af4484 lea rcx, [r11 + r11*2]
0x140af4488 shl rcx, 3
0x140af448c add rcx, qword ptr [r8 + 0x28]
0x140af4490 mov rbx, qword ptr [r8 + 0x30]
0x140af4494 sub rbx, r11
0x140af4497 movzx edx, dl
0x140af449a mov qword ptr [rbp + 0x3028], rdx
0x140af44a1 mov rdx, qword ptr [r14]
0x140af44a4 movss xmm10, dword ptr [rdx + 0x58]
0x140af44aa mov qword ptr [rbp + 0x30a8], rdx
0x140af44b1 movsd xmm6, qword ptr [rdx + 0x5c]
0x140af44b6 mov rdx, qword ptr [rbp + 0x3080]
0x140af44bd mov edx, dword ptr [rdx]
0x140af44bf mov dword ptr [rbp + 0x3064], edx
0x140af44c5 mov rdx, qword ptr [rbp + 0x3090]
0x140af44cc movzx edx, byte ptr [rdx + 0x4d0]
0x140af44d3 mov byte ptr [rbp + 0x30b7], dl
0x140af44d9 cmovb rbx, r9
0x140af44dd mov rdx, qword ptr [rbp + 0x3098]
0x140af44e4 mov r9d, dword ptr [rdx + 8]
0x140af44e8 mov dword ptr [rbp + 0x3068], r9d
0x140af44ef mov r9, qword ptr [rbp + 0x3078]
0x140af44f6 movq xmm0, qword ptr [r9]
0x140af44fb movss xmm14, dword ptr [r9 + 8]
0x140af4501 mov edx, dword ptr [rdx + 0x10]
0x140af4504 mov dword ptr [rbp + 0x3060], edx
0x140af450a cmovb rcx, rdi
0x140af450e mov rdx, qword ptr [r8 + 0x40]
0x140af4512 sub rdx, rbx
0x140af4515 sub rdx, r10
0x140af4518 mov qword ptr [rsi], rdx
0x140af451b lea rdx, [r10 + r10*2]
0x140af451f lea r12, [rax + rdx*8]
0x140af4523 lea rdx, [rbx + rbx*2]
0x140af4527 lea rdx, [rcx + rdx*8]
0x140af452b mov qword ptr [rbp + 0x3078], rdx
0x140af4532 mov rdx, qword ptr [rbp + 0x3050]
0x140af4539 add rdx, 0x20
0x140af453d mov qword ptr [rbp + 0x3080], rdx
0x140af4544 movq xmm7, qword ptr [rip + 0x1fc2104] ; 0x142ab6650 cdcccc3d90c2f53c0000000000000000 f32=0.10000000149011612
0x140af454c pcmpeqd xmm9, xmm9
0x140af4551 jmp 0x140af45f0
0x140af4556 nop word ptr cs:[rax + rax]
0x140af4560 movdqu xmm1, xmmword ptr [rcx + r8]
0x140af4566 movdqa xmm2, xmm1
0x140af456a pcmpeqb xmm2, xmm0
0x140af456e pmovmskb r10d, xmm2
0x140af4573 test r10d, r10d
0x140af4576 je 0x140af45b0
0x140af4578 tzcnt r11d, r10d
0x140af457d add r11, r8
0x140af4580 and r11, rdx
0x140af4583 neg r11
0x140af4586 lea r11, [r11 + r11*2]
0x140af458a cmp dword ptr [rax + r11*4], r13d
0x140af458e jne 0x140af4597
0x140af4590 cmp dword ptr [rax + r11*4 + 4], edi
0x140af4595 je 0x140af45d3
0x140af4597 lea r11d, [r10 - 1]
0x140af459b and r11w, r10w
0x140af459f mov r10d, r11d
0x140af45a2 jne 0x140af4578
0x140af45a4 nop word ptr cs:[rax + rax]
0x140af45b0 pcmpeqb xmm1, xmm9
0x140af45b5 pmovmskb r10d, xmm1
0x140af45ba test r10d, r10d
0x140af45bd jne 0x140af46c0
0x140af45c3 add r8, r9
0x140af45c6 add r8, 0x10
0x140af45ca add r9, 0x10
0x140af45ce and r8, rdx
0x140af45d1 jmp 0x140af4560
0x140af45d3 mov r13d, dword ptr [rcx + r11*4 - 4]
0x140af45d8 mov ebx, 1
0x140af45dd jmp 0x140af46c0
0x140af45e2 nop word ptr cs:[rax + rax]
0x140af45f0 test rax, rax
0x140af45f3 je 0x140af4620
0x140af45f5 cmp rax, r12
0x140af45f8 je 0x140af4620
0x140af45fa lea r14, [rax + 0x18]
0x140af45fe mov r15, rcx
0x140af4601 inc qword ptr [rsi]
0x140af4604 xor ebx, ebx
0x140af4606 cmp byte ptr [rax + 0x14], 3
0x140af460a je 0x140af464e
0x140af460c mov rcx, r15
0x140af460f mov rax, r14
0x140af4612 test ebx, ebx
0x140af4614 je 0x140af45f0
0x140af4616 jmp 0x140af46d6
0x140af461b nop dword ptr [rax + rax]
0x140af4620 xor r13d, r13d
0x140af4623 test rcx, rcx
0x140af4626 je 0x140af46d6
0x140af462c cmp rcx, qword ptr [rbp + 0x3078]
0x140af4633 je 0x140af46d6
0x140af4639 lea r15, [rcx + 0x18]
0x140af463d xor r14d, r14d
0x140af4640 mov rax, rcx
0x140af4643 inc qword ptr [rsi]
0x140af4646 xor ebx, ebx
0x140af4648 cmp byte ptr [rax + 0x14], 3
0x140af464c jne 0x140af460c
0x140af464e mov r13d, dword ptr [rax + 8]
0x140af4652 mov edi, dword ptr [rax + 0xc]
0x140af4655 mov dword ptr [rbp + 0x1be0], r13d
0x140af465c mov dword ptr [rbp + 0x1be4], edi
0x140af4662 xor ebx, ebx
0x140af4664 mov rax, qword ptr [rbp + 0x3050]
0x140af466b cmp qword ptr [rax + 0x18], 0
0x140af4670 je 0x140af46c0
0x140af4672 mov rcx, qword ptr [rbp + 0x3080]
0x140af4679 lea rdx, [rbp + 0x1be0]
0x140af4680 call 0x1405b1be0
0x140af4685 mov rdx, qword ptr [rbp + 0x3050]
0x140af468c mov rcx, qword ptr [rdx]
0x140af468f mov rdx, qword ptr [rdx + 8]
0x140af4693 mov r8, rdx
0x140af4696 and r8, rax
0x140af4699 shr rax, 0x39
0x140af469d movd xmm0, eax
0x140af46a1 punpcklbw xmm0, xmm0
0x140af46a5 pshuflw xmm0, xmm0, 0
0x140af46aa pshufd xmm0, xmm0, 0
0x140af46af lea rax, [rcx - 0xc]
0x140af46b3 xor r9d, r9d
0x140af46b6 jmp 0x140af4560
0x140af46bb nop dword ptr [rax + rax]
0x140af46c0 xorps xmm14, xmm14
0x140af46c4 movdqa xmm0, xmm7
0x140af46c8 mov rcx, r15
0x140af46cb mov rax, r14
0x140af46ce test ebx, ebx
0x140af46d0 je 0x140af45f0
0x140af46d6 movdqa xmmword ptr [rbp + 0x3050], xmm0
0x140af46de lea rcx, [rbp + 0x2e0]
0x140af46e5 lea rdx, [rbp + 0x920]
0x140af46ec mov r8d, 0x320
0x140af46f2 call 0x142923c00
0x140af46f7 movss xmm0, dword ptr [rip + 0x1fef98d] ; 0x142ae408c c10f7c3c00000000c10f7c3c00000000 f32=0.015384615398943424
0x140af46ff movaps xmmword ptr [rbp + 0x2b80], xmm0
0x140af4706 movq xmm0, qword ptr [rip + 0x1fef982] ; 0x142ae4090 00000000c10f7c3c0000000000000000 f32=0.0
0x140af470e movdqa xmmword ptr [rbp + 0x2b90], xmm0
0x140af4716 movdqa xmm7, xmmword ptr [rip + 0x1fef982] ; 0x142ae40a0 0000000000000000000080ba00000000 f32=0.0
0x140af471e movdqa xmmword ptr [rbp + 0x2ba0], xmm7
0x140af4726 movdqa xmm9, xmmword ptr [rip + 0x1fef981] ; 0x142ae40b0 00000080000000800000003f0000803f f32=-0.0
0x140af472f movdqa xmmword ptr [rbp + 0x2bb0], xmm9
0x140af4738 lea rcx, [rbp + 0xc80]
0x140af473f lea rdx, [rbp + 0x2b80]
0x140af4746 call 0x1412b7d00
0x140af474b mov rdx, qword ptr [rbp + 0x30a8]
0x140af4752 movdqa xmm0, xmmword ptr [rdx]
0x140af4756 movaps xmm1, xmmword ptr [rdx + 0x10]
0x140af475a movaps xmm2, xmmword ptr [rdx + 0x20]
0x140af475e movdqa xmm3, xmmword ptr [rdx + 0x30]
0x140af4763 movdqa xmmword ptr [rbp + 0xcf0], xmm3
0x140af476b movaps xmmword ptr [rbp + 0xce0], xmm2
0x140af4772 movaps xmmword ptr [rbp + 0xcd0], xmm1
0x140af4779 movdqa xmmword ptr [rbp + 0xcc0], xmm0
0x140af4781 lea rcx, [rbp + 0xd00]
0x140af4788 call 0x1412b7d00
0x140af478d movdqa xmm0, xmmword ptr [rbp + 0x2b80]
0x140af4795 movaps xmm1, xmmword ptr [rbp + 0x2b90]
0x140af479c movaps xmm2, xmmword ptr [rbp + 0x2ba0]
0x140af47a3 movdqa xmm3, xmmword ptr [rbp + 0x2bb0]
0x140af47ab movdqa xmmword ptr [rbp + 0xc70], xmm3
0x140af47b3 movaps xmmword ptr [rbp + 0xc60], xmm2
0x140af47ba movaps xmmword ptr [rbp + 0xc50], xmm1
0x140af47c1 movdqa xmmword ptr [rbp + 0xc40], xmm0
0x140af47c9 lea rsi, [rbp + 0x1be0]
0x140af47d0 lea r15, [rbp + 0xc40]
0x140af47d7 mov r8d, 0x100
0x140af47dd mov rcx, rsi
0x140af47e0 mov rdx, r15
0x140af47e3 call 0x142923c00
0x140af47e8 movabs rdi, 0x4b0000004b0
0x140af47f2 mov qword ptr [rbp + 0x1de0], rdi
0x140af47f9 lea rcx, [rbp + 0x1ce0]
0x140af4800 mov r8d, 0x100
0x140af4806 mov rdx, r15
0x140af4809 call 0x142923c00
0x140af480e pxor xmm0, xmm0
0x140af4812 movdqu xmmword ptr [rbp + 0x1de8], xmm0
0x140af481a lea rcx, [rbp - 0x40]
0x140af481e mov rdx, rsi
0x140af4821 call 0x140b18f60
0x140af4826 movss xmm0, dword ptr [rip + 0x1fef892] ; 0x142ae40c0 c10ffc3b000000000000000000000000 f32=0.007692307699471712
0x140af482e movaps xmmword ptr [rbp + 0x2ea0], xmm0
0x140af4835 movq xmm0, qword ptr [rip + 0x1fef893] ; 0x142ae40d0 00000000c10ffc3b0000000000000000 f32=0.0
0x140af483d movdqa xmmword ptr [rbp + 0x2eb0], xmm0
0x140af4845 movdqa xmmword ptr [rbp + 0x2ec0], xmm7
0x140af484d movdqa xmmword ptr [rbp + 0x2ed0], xmm9
0x140af4856 lea rcx, [rbp + 0xc80]
0x140af485d lea rdx, [rbp + 0x2ea0]
0x140af4864 call 0x1412b7d00
0x140af4869 mov rdx, qword ptr [rbp + 0x30a8]
0x140af4870 movdqa xmm0, xmmword ptr [rdx]
0x140af4874 movaps xmm1, xmmword ptr [rdx + 0x10]
0x140af4878 movaps xmm2, xmmword ptr [rdx + 0x20]
0x140af487c movdqa xmm3, xmmword ptr [rdx + 0x30]
0x140af4881 movdqa xmmword ptr [rbp + 0xcf0], xmm3
0x140af4889 movaps xmmword ptr [rbp + 0xce0], xmm2
0x140af4890 movaps xmmword ptr [rbp + 0xcd0], xmm1
0x140af4897 movdqa xmmword ptr [rbp + 0xcc0], xmm0
0x140af489f lea rcx, [rbp + 0xd00]
0x140af48a6 call 0x1412b7d00
0x140af48ab movdqa xmm0, xmmword ptr [rbp + 0x2ea0]
0x140af48b3 movaps xmm1, xmmword ptr [rbp + 0x2eb0]
0x140af48ba movaps xmm2, xmmword ptr [rbp + 0x2ec0]
0x140af48c1 movdqa xmm3, xmmword ptr [rbp + 0x2ed0]
0x140af48c9 movdqa xmmword ptr [rbp + 0xc70], xmm3
0x140af48d1 movaps xmmword ptr [rbp + 0xc60], xmm2
0x140af48d8 movaps xmmword ptr [rbp + 0xc50], xmm1
0x140af48df movdqa xmmword ptr [rbp + 0xc40], xmm0
0x140af48e7 mov r8d, 0x100
0x140af48ed mov rcx, rsi
0x140af48f0 mov rdx, r15
0x140af48f3 call 0x142923c00
0x140af48f8 mov qword ptr [rbp + 0x1de0], rdi
0x140af48ff lea rcx, [rbp + 0x1ce0]
0x140af4906 mov r8d, 0x100
0x140af490c mov rdx, r15
0x140af490f call 0x142923c00
0x140af4914 pxor xmm0, xmm0
0x140af4918 movdqu xmmword ptr [rbp + 0x1de8], xmm0
0x140af4920 lea rcx, [rbp + 0x2b80]
0x140af4927 mov rdx, rsi
0x140af492a call 0x140b18f60
0x140af492f movss xmm0, dword ptr [rip + 0x1fef7a9] ; 0x142ae40e0 0109103b000000000000000000000000 f32=0.002197802299633622
0x140af4937 movaps xmmword ptr [rbp + 0x2fa0], xmm0
0x140af493e movq xmm0, qword ptr [rip + 0x1fef7aa] ; 0x142ae40f0 000000000109103b0000000000000000 f32=0.0
0x140af4946 movdqa xmmword ptr [rbp + 0x2fb0], xmm0
0x140af494e movdqa xmmword ptr [rbp + 0x2fc0], xmm7
0x140af4956 movdqa xmmword ptr [rbp + 0x2fd0], xmm9
0x140af495f lea rcx, [rbp + 0x2ee0]
0x140af4966 lea rdx, [rbp + 0x2fa0]
0x140af496d call 0x1412b7d00
0x140af4972 mov rdx, qword ptr [rbp + 0x30a8]
0x140af4979 movdqa xmm0, xmmword ptr [rdx]
0x140af497d movaps xmm1, xmmword ptr [rdx + 0x10]
0x140af4981 movaps xmm2, xmmword ptr [rdx + 0x20]
0x140af4985 movdqa xmm3, xmmword ptr [rdx + 0x30]
0x140af498a movdqa xmmword ptr [rbp + 0x2f50], xmm3
0x140af4992 movaps xmmword ptr [rbp + 0x2f40], xmm2
0x140af4999 movaps xmmword ptr [rbp + 0x2f30], xmm1
0x140af49a0 movdqa xmmword ptr [rbp + 0x2f20], xmm0
0x140af49a8 lea rcx, [rbp + 0x2f60]
0x140af49af call 0x1412b7d00
0x140af49b4 movdqa xmm0, xmmword ptr [rbp + 0x2fa0]
0x140af49bc movaps xmm1, xmmword ptr [rbp + 0x2fb0]
0x140af49c3 movaps xmm2, xmmword ptr [rbp + 0x2fc0]
0x140af49ca movdqa xmm3, xmmword ptr [rbp + 0x2fd0]
0x140af49d2 movdqa xmmword ptr [rbp + 0x2ed0], xmm3
0x140af49da movaps xmmword ptr [rbp + 0x2ec0], xmm2
0x140af49e1 movaps xmmword ptr [rbp + 0x2eb0], xmm1
0x140af49e8 movdqa xmmword ptr [rbp + 0x2ea0], xmm0
0x140af49f0 lea r14, [rbp + 0x2ea0]
0x140af49f7 mov r8d, 0x100
0x140af49fd mov rcx, rsi
0x140af4a00 mov rdx, r14
0x140af4a03 call 0x142923c00
0x140af4a08 mov qword ptr [rbp + 0x1de0], rdi
0x140af4a0f lea rcx, [rbp + 0x1ce0]
0x140af4a16 mov r8d, 0x100
0x140af4a1c mov rdx, r14
0x140af4a1f call 0x142923c00
0x140af4a24 pxor xmm0, xmm0
0x140af4a28 movdqu xmmword ptr [rbp + 0x1de8], xmm0
0x140af4a30 lea rcx, [rbp + 0xc40]
0x140af4a37 mov rdx, rsi
0x140af4a3a call 0x140b18f60
0x140af4a3f lea rbx, [rbp + 0x1be0]
0x140af4a46 lea rdx, [rbp + 0x2e0]
0x140af4a4d mov r8d, 0x320
0x140af4a53 mov rcx, rbx
0x140af4a56 call 0x142923c00
0x140af4a5b lea rcx, [rbp + 0x1f00]
0x140af4a62 lea rdx, [rbp + 0x600]
0x140af4a69 mov r8d, 0x320
0x140af4a6f call 0x142923c00
0x140af4a74 lea rcx, [rbp + 0x2220]
0x140af4a7b lea rdx, [rbp - 0x40]
0x140af4a7f mov r8d, 0x320
0x140af4a85 call 0x142923c00
0x140af4a8a lea rcx, [rbp + 0x2540]
0x140af4a91 lea rdx, [rbp + 0x2b80]
0x140af4a98 mov r8d, 0x320
0x140af4a9e call 0x142923c00
0x140af4aa3 lea rcx, [rbp + 0x2860]
0x140af4aaa lea rdx, [rbp + 0xc40]
0x140af4ab1 mov r8d, 0x320
0x140af4ab7 call 0x142923c00
0x140af4abc mov rax, qword ptr [rbp + 0x3088]
0x140af4ac3 mov eax, dword ptr [rax + 0x108]
0x140af4ac9 mov dword ptr [rbp + 0x30a8], eax
0x140af4acf mov rsi, qword ptr [rbp + 0x3048]
0x140af4ad6 movsd xmm7, qword ptr [rsi + 0x1c]
0x140af4adb movsd xmm12, qword ptr [rsi + 0x34]
0x140af4ae1 movups xmm15, xmmword ptr [rsi + 0x54]
0x140af4ae6 mov rax, qword ptr [rbp + 0x3070]
0x140af4aed movss xmm0, dword ptr [rax + 0x28]
0x140af4af2 movss dword ptr [rbp + 0x3088], xmm0
0x140af4afa mov rcx, qword ptr [rbp + 0x3098]
0x140af4b01 mov rcx, qword ptr [rcx + 0x10]
0x140af4b05 mov qword ptr [rbp + 0x3098], rcx
0x140af4b0c movsd xmm13, qword ptr [rax + 0x10]
0x140af4b12 mov rax, qword ptr [rbp + 0x3030]
0x140af4b19 movzx eax, byte ptr [rax + 0x58]
0x140af4b1d mov rcx, qword ptr [rbp + 0x3090]
0x140af4b24 movzx ecx, byte ptr [rcx + 0x4d0]
0x140af4b2b mov dword ptr [rbp + 0x3080], ecx
0x140af4b31 lea rcx, [rip + 0x1ff0488] ; 0x142ae4fc0 00000000030000000100000002000000 f32=0.0
0x140af4b38 mov eax, dword ptr [rcx + rax*4]
0x140af4b3b mov dword ptr [rbp + 0x3070], eax
0x140af4b41 mov rdi, qword ptr [rbp + 0x3028]
0x140af4b48 lea eax, [rdi + 2]
0x140af4b4b cmp byte ptr [rbp + 0x30a4], 0
0x140af4b52 mov r15d, edi
0x140af4b55 cmovne r15d, eax
0x140af4b59 movss xmm0, dword ptr [rsi + 0x78]
0x140af4b5e movss dword ptr [rbp + 0x3090], xmm0
0x140af4b66 cmp byte ptr [rbp + 0x30b7], 3
0x140af4b6d cmove r15d, edi
0x140af4b71 cmp dword ptr [rbp + 0x3064], 0x13
0x140af4b78 cmovne r15d, eax
0x140af4b7c movshdup xmm9, xmm8
0x140af4b81 movaps xmm0, xmm9
0x140af4b85 call 0x142923eb0
0x140af4b8a movss dword ptr [rbp + 0x30a4], xmm0
0x140af4b92 shufps xmm15, xmm15, 0x39
0x140af4b97 movaps xmmword ptr [rbp + 0x3030], xmm15
0x140af4b9f movss xmm15, dword ptr [rsi + 0x30]
0x140af4ba5 shufps xmm15, xmm12, 0x4c
0x140af4baa shufps xmm15, xmm15, 0x78
0x140af4baf test dil, dil
0x140af4bb2 movss dword ptr [rbp + 0x3078], xmm14
0x140af4bbb jne 0x140af4bc2
0x140af4bbd xorps xmm0, xmm0
0x140af4bc0 jmp 0x140af4bca
0x140af4bc2 movss xmm0, dword ptr [rip + 0x1e6551a] ; 0x14295a0e4 000080bf000000bf2f72757374632f30 f32=-1.0
0x140af4bca addss xmm9, xmm0
0x140af4bcf addss xmm11, xmm9
0x140af4bd4 movaps xmm0, xmm11
0x140af4bd8 call 0x142923eb0
0x140af4bdd mov r14, qword ptr [rbp + 0x3048]
0x140af4be4 movsd xmm5, qword ptr [r14 + 0x28]
0x140af4bea movaps xmm1, xmmword ptr [rip + 0x1ef8fef] ; 0x1429edbe0 0000f0410000f0410000000000000000 f32=30.0
0x140af4bf1 mulps xmm5, xmm1
0x140af4bf4 movsldup xmm4, xmm0
0x140af4bf8 mulps xmm5, xmm4
0x140af4bfb movss xmm2, dword ptr [rip + 0x1ed8431] ; 0x1429cd034 0000f0416f1283ba000000006f12833a f32=30.0
0x140af4c03 movss xmm9, dword ptr [r14 + 0x24]
0x140af4c09 mulss xmm9, xmm2
0x140af4c0e mulss xmm9, xmm0
0x140af4c13 movaps xmm3, xmmword ptr [rip + 0x1e494a6] ; 0x14293e0c0 0000000000000000000000000000803f f32=0.0
0x140af4c1a shufps xmm9, xmm3, 0xe4
0x140af4c1f shufps xmm9, xmm5, 0x4c
0x140af4c24 shufps xmm9, xmm9, 0x78
0x140af4c29 movss xmm8, dword ptr [r14 + 0x18]
0x140af4c2f shufps xmm8, xmm7, 0x4c
0x140af4c34 shufps xmm8, xmm8, 0x78
0x140af4c39 movsd xmm5, qword ptr [r14 + 4]
0x140af4c3f movsd xmm11, qword ptr [r14 + 0x10]
0x140af4c45 mulps xmm11, xmm1
0x140af4c49 mulps xmm11, xmm4
0x140af4c4d movss xmm7, dword ptr [r14 + 0xc]
0x140af4c53 mulss xmm7, xmm2
0x140af4c57 mulss xmm7, xmm0
0x140af4c5b shufps xmm7, xmm3, 0xe4
0x140af4c5f shufps xmm7, xmm11, 0x4c
0x140af4c64 shufps xmm7, xmm7, 0x78
0x140af4c68 mulss xmm10, xmm0
0x140af4c6d movss xmm11, dword ptr [r14 + 0x48]
0x140af4c73 mulss xmm11, xmm10
0x140af4c78 mulss xmm11, dword ptr [rip + 0x1fef47f] ; 0x142ae4100 7b142e405c8f02410000000000000000 f32=2.7200000286102295
0x140af4c81 shufps xmm11, xmm3, 0xe4
0x140af4c86 movss xmm14, dword ptr [r14 + 0x3c]
0x140af4c8c mulss xmm14, xmm10
0x140af4c91 mulss xmm14, dword ptr [rip + 0x1fef46a] ; 0x142ae4104 5c8f024100000000000000007b142e40 f32=8.15999984741211
0x140af4c9a mulps xmm6, xmm4
0x140af4c9d movsd xmm0, qword ptr [r14 + 0x4c]
0x140af4ca3 mulps xmm0, xmm6
0x140af4ca6 mulps xmm0, xmmword ptr [rip + 0x1fef463] ; 0x142ae4110 7b142e407b142e400000000000000000 f32=2.7200000286102295
0x140af4cad shufps xmm14, xmm3, 0xe4
0x140af4cb2 shufps xmm11, xmm0, 0x4c
0x140af4cb7 shufps xmm11, xmm11, 0x78
0x140af4cbc movsd xmm0, qword ptr [r14 + 0x40]
0x140af4cc2 mulps xmm0, xmm6
0x140af4cc5 mulps xmm0, xmmword ptr [rip + 0x1fef454] ; 0x142ae4120 5c8f02415c8f02410000000000000000 f32=8.15999984741211
0x140af4ccc shufps xmm14, xmm0, 0x4c
0x140af4cd1 shufps xmm14, xmm14, 0x78
0x140af4cd6 mulps xmm6, xmm5
0x140af4cd9 mulss xmm10, dword ptr [r14]
0x140af4cde mulps xmm6, xmm1
0x140af4ce1 mulss xmm10, xmm2
0x140af4ce6 shufps xmm10, xmm3, 0xe4
0x140af4ceb shufps xmm10, xmm6, 0x4c
0x140af4cf0 mov eax, dword ptr [rbp + 0x3060]
0x140af4cf6 xorps xmm0, xmm0
0x140af4cf9 cvtsi2ss xmm0, rax
0x140af4cfe mov eax, dword ptr [rbp + 0x3068]
0x140af4d04 xorps xmm1, xmm1
0x140af4d07 cvtsi2ss xmm1, rax
0x140af4d0c shufps xmm10, xmm10, 0x78
0x140af4d11 maxss xmm1, dword ptr [rip + 0x1e30beb] ; 0x142925904 0000803f9a99193f00000000feffffff f32=1.0
0x140af4d19 divss xmm0, xmm1
0x140af4d1d call 0x142923f50
0x140af4d22 movaps xmm6, xmm0
0x140af4d25 movss xmm0, dword ptr [r14 + 0x64]
0x140af4d2b mov rax, qword ptr [rbp + 0x3020]
0x140af4d32 movss xmm12, dword ptr [rax + 8]
0x140af4d38 unpcklps xmm12, xmm0
0x140af4d3c movhps xmm12, qword ptr [r14 + 0x68]
0x140af4d41 lea rsi, [rbp + 0xc40]
0x140af4d48 mov r8d, 0xfa0
0x140af4d4e mov rcx, rsi
0x140af4d51 mov rdx, rbx
0x140af4d54 call 0x142923c00
0x140af4d59 mov rax, qword ptr [rbp + 0x3018]
0x140af4d60 mov edi, dword ptr [rax + 0x1c]
0x140af4d63 mov rbx, qword ptr [rax]
0x140af4d66 mov r12, qword ptr [rax + 0x10]
0x140af4d6a movsd xmm1, qword ptr [rip + 0x1e3f876] ; 0x1429345e8 000000000000f03f000000000000f0ff f32=0.0
0x140af4d72 movapd xmm0, xmm13
0x140af4d77 call 0x142923f00
0x140af4d7c xorpd xmm1, xmm1
0x140af4d80 maxsd xmm1, xmm13
0x140af4d85 movsd xmm2, qword ptr [rip + 0x1f24bb3] ; 0x142a19940 0000e0ffffffef4100000000000060c0 f32=nan
0x140af4d8d minsd xmm2, xmm1
0x140af4d91 cvttsd2si rax, xmm2
0x140af4d96 movsd xmm1, qword ptr [r14 + 0x70]
0x140af4d9c mov dword ptr [r12], edi
0x140af4da0 movaps xmmword ptr [rbx], xmm10
0x140af4da4 movaps xmmword ptr [rbx + 0x10], xmm14
0x140af4da9 movaps xmmword ptr [rbx + 0x20], xmm11
0x140af4dae movaps xmm2, xmmword ptr [rbp + 0x3030]
0x140af4db5 movaps xmmword ptr [rbx + 0x30], xmm2
0x140af4db9 movaps xmmword ptr [rbx + 0x40], xmm7
0x140af4dbd movaps xmmword ptr [rbx + 0x50], xmm8
0x140af4dc2 movaps xmmword ptr [rbx + 0x60], xmm9
0x140af4dc7 movaps xmmword ptr [rbx + 0x70], xmm15
0x140af4dcc mov ecx, dword ptr [rbp + 0x30a8]
0x140af4dd2 mov dword ptr [rbx + 0x80], ecx
0x140af4dd8 movss xmm2, dword ptr [rbp + 0x3088]
0x140af4de0 movss dword ptr [rbx + 0x84], xmm2
0x140af4de8 movss xmm2, dword ptr [rbp + 0x30a4]
0x140af4df0 movss dword ptr [rbx + 0x88], xmm2
0x140af4df8 cvtsd2ss xmm0, xmm0
0x140af4dfc mov ecx, dword ptr [rbp + 0x3070]
0x140af4e02 mov dword ptr [rbx + 0x8c], ecx
0x140af4e08 mov rcx, qword ptr [rbp + 0x3098]
0x140af4e0f mov qword ptr [rbx + 0x90], rcx
0x140af4e16 mov dword ptr [rbx + 0x98], eax
0x140af4e1c movss dword ptr [rbx + 0x9c], xmm0
0x140af4e24 movaps xmm0, xmmword ptr [rbp + 0x3050]
0x140af4e2b movlps qword ptr [rbx + 0xa0], xmm0
0x140af4e32 movss xmm0, dword ptr [rbp + 0x3078]
0x140af4e3a movss dword ptr [rbx + 0xa8], xmm0
0x140af4e42 movups xmmword ptr [rbx + 0xac], xmm12
0x140af4e4a mov eax, dword ptr [rbp + 0x3080]
0x140af4e50 mov dword ptr [rbx + 0xbc], eax
0x140af4e56 movsd qword ptr [rbx + 0xc0], xmm1
0x140af4e5e movss xmm0, dword ptr [rbp + 0x3090]
0x140af4e66 movss dword ptr [rbx + 0xc8], xmm0
0x140af4e6e mov dword ptr [rbx + 0xcc], r15d
0x140af4e75 movss dword ptr [rbx + 0xd0], xmm6
0x140af4e7d mov dword ptr [rbx + 0xd4], r13d
0x140af4e84 movabs rax, 0x200000001
0x140af4e8e mov qword ptr [rbx + 0xd8], rax
0x140af4e95 xorps xmm0, xmm0
0x140af4e98 movaps xmmword ptr [rbx + 0xe0], xmm0
0x140af4e9f add rbx, 0xf0
0x140af4ea6 mov r8d, 0xfa0
0x140af4eac mov rcx, rbx
0x140af4eaf mov rdx, rsi
0x140af4eb2 call 0x142923c00
0x140af4eb7 mov rdx, qword ptr [rbp + 0x2fe8]
0x140af4ebe test rdx, rdx
0x140af4ec1 je 0x140af4ed5
0x140af4ec3 mov rcx, qword ptr [rbp + 0x2ff0]
0x140af4eca mov r8d, 1
0x140af4ed0 call 0x140613c20
0x140af4ed5 movaps xmm6, xmmword ptr [rbp + 0x30c0]
0x140af4edc movaps xmm7, xmmword ptr [rbp + 0x30d0]
0x140af4ee3 movaps xmm8, xmmword ptr [rbp + 0x30e0]
0x140af4eeb movaps xmm9, xmmword ptr [rbp + 0x30f0]
0x140af4ef3 movaps xmm10, xmmword ptr [rbp + 0x3100]
0x140af4efb movaps xmm11, xmmword ptr [rbp + 0x3110]
0x140af4f03 movaps xmm12, xmmword ptr [rbp + 0x3120]
0x140af4f0b movaps xmm13, xmmword ptr [rbp + 0x3130]
0x140af4f13 movaps xmm14, xmmword ptr [rbp + 0x3140]
0x140af4f1b movaps xmm15, xmmword ptr [rbp + 0x3150]
0x140af4f23 add rsp, 0x31e8
0x140af4f2a pop rbx
0x140af4f2b pop rdi
0x140af4f2c pop rsi
0x140af4f2d pop r12
0x140af4f2f pop r13
0x140af4f31 pop r14
0x140af4f33 pop r15
0x140af4f35 pop rbp
0x140af4f36 ret 
0x140af4f37 lea r8, [rip + 0x1ff18a2] ; 0x142ae67e0 9067ae42010000004a00000000000000 f32=87.2022705078125
0x140af4f3e mov ecx, 1
0x140af4f43 mov edx, 7
0x140af4f48 call 0x1428d8fe3
0x140af4f4d lea rcx, [rip + 0x1ff0054] ; 0x142ae4fa8 624fae42010000004200000000000000 f32=87.15504455566406
0x140af4f54 call 0x1428d9a20
0x140af4f59 ud2 
0x140af4f5b nop dword ptr [rax + rax]
0x140af4f60 mov qword ptr [rsp + 0x10], rdx
0x140af4f65 push rbp
0x140af4f66 push r15
0x140af4f68 push r14
0x140af4f6a push r13
0x140af4f6c push r12
0x140af4f6e push rsi
0x140af4f6f push rdi
0x140af4f70 push rbx
0x140af4f71 sub rsp, 0xd8
0x140af4f78 lea rbp, [rdx + 0x80]
0x140af4f7f movaps xmmword ptr [rsp + 0x30], xmm15
0x140af4f85 movaps xmmword ptr [rsp + 0x40], xmm14
0x140af4f8b movaps xmmword ptr [rsp + 0x50], xmm13
0x140af4f91 movaps xmmword ptr [rsp + 0x60], xmm12
0x140af4f97 movaps xmmword ptr [rsp + 0x70], xmm11
0x140af4f9d movaps xmmword ptr [rsp + 0x80], xmm10
0x140af4fa6 movdqa xmmword ptr [rsp + 0x90], xmm9
0x140af4fb0 movaps xmmword ptr [rsp + 0xa0], xmm8
0x140af4fb9 movdqa xmmword ptr [rsp + 0xb0], xmm7
0x140af4fc2 movaps xmmword ptr [rsp + 0xc0], xmm6
0x140af4fca mov rdx, qword ptr [rbp + 0x2fe8]
0x140af4fd1 test rdx, rdx
0x140af4fd4 je 0x140af4fe8
0x140af4fd6 mov rcx, qword ptr [rbp + 0x2ff0]
0x140af4fdd mov r8d, 1
0x140af4fe3 call 0x140613c20
0x140af4fe8 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x140af4ff0 movaps xmm7, xmmword ptr [rsp + 0xb0]
0x140af4ff8 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x140af5001 movaps xmm9, xmmword ptr [rsp + 0x90]
0x140af500a movaps xmm10, xmmword ptr [rsp + 0x80]
0x140af5013 movaps xmm11, xmmword ptr [rsp + 0x70]
0x140af5019 movaps xmm12, xmmword ptr [rsp + 0x60]
0x140af501f movaps xmm13, xmmword ptr [rsp + 0x50]
0x140af5025 movaps xmm14, xmmword ptr [rsp + 0x40]
0x140af502b movaps xmm15, xmmword ptr [rsp + 0x30]
0x140af5031 add rsp, 0xd8
0x140af5038 pop rbx
0x140af5039 pop rdi
0x140af503a pop rsi
0x140af503b pop r12
0x140af503d pop r13
0x140af503f pop r14
0x140af5041 pop r15
0x140af5043 pop rbp
0x140af5044 ret 