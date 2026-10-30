
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140818df0: 55                          	pushq	%rbp
140818df1: 41 57                       	pushq	%r15
140818df3: 41 56                       	pushq	%r14
140818df5: 41 55                       	pushq	%r13
140818df7: 41 54                       	pushq	%r12
140818df9: 56                          	pushq	%rsi
140818dfa: 57                          	pushq	%rdi
140818dfb: 53                          	pushq	%rbx
140818dfc: 48 81 ec 78 01 00 00        	subq	$0x178, %rsp            # imm = 0x178
140818e03: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140818e0b: 44 0f 29 ad e0 00 00 00     	movaps	%xmm13, 0xe0(%rbp)
140818e13: 44 0f 29 a5 d0 00 00 00     	movaps	%xmm12, 0xd0(%rbp)
140818e1b: 66 44 0f 7f 9d c0 00 00 00  	movdqa	%xmm11, 0xc0(%rbp)
140818e24: 44 0f 29 95 b0 00 00 00     	movaps	%xmm10, 0xb0(%rbp)
140818e2c: 44 0f 29 8d a0 00 00 00     	movaps	%xmm9, 0xa0(%rbp)
140818e34: 44 0f 29 85 90 00 00 00     	movaps	%xmm8, 0x90(%rbp)
140818e3c: 0f 29 bd 80 00 00 00        	movaps	%xmm7, 0x80(%rbp)
140818e43: 66 0f 7f 75 70              	movdqa	%xmm6, 0x70(%rbp)
140818e48: 48 c7 45 68 fe ff ff ff     	movq	$-0x2, 0x68(%rbp)
140818e50: 4c 89 4d c0                 	movq	%r9, -0x40(%rbp)
140818e54: 4d 89 c6                    	movq	%r8, %r14
140818e57: 48 89 d6                    	movq	%rdx, %rsi
140818e5a: 48 89 cf                    	movq	%rcx, %rdi
140818e5d: e8 9e db 3f 00              	callq	0x140c16a00 <.text+0xc15a00>
140818e62: 88 45 67                    	movb	%al, 0x67(%rbp)
140818e65: 84 c0                       	testb	%al, %al
140818e67: 0f 84 06 02 00 00           	je	0x140819073 <.text+0x818073>
140818e6d: 41 bf 4c 00 00 00           	movl	$0x4c, %r15d
140818e73: b9 4c 00 00 00              	movl	$0x4c, %ecx
140818e78: e8 43 fb ff ff              	callq	0x1408189c0 <.text+0x8179c0>
140818e7d: 4c 8d 25 ac 71 27 02        	leaq	0x22771ac(%rip), %r12   # 0x142a90030
140818e84: 48 83 f8 01                 	cmpq	$0x1, %rax
140818e88: 0f 85 a1 00 00 00           	jne	0x140818f2f <.text+0x817f2f>
140818e8e: 49 89 d1                    	movq	%rdx, %r9
140818e91: 48 85 d2                    	testq	%rdx, %rdx
140818e94: 74 36                       	je	0x140818ecc <.text+0x817ecc>
140818e96: 49 83 f9 4c                 	cmpq	$0x4c, %r9
140818e9a: 73 2e                       	jae	0x140818eca <.text+0x817eca>
140818e9c: 48 8d 05 8d 71 27 02        	leaq	0x227718d(%rip), %rax   # 0x142a90030
140818ea3: 41 80 3c 01 bf              	cmpb	$-0x41, (%r9,%rax)
140818ea8: 7f 22                       	jg	0x140818ecc <.text+0x817ecc>
140818eaa: 48 8d 05 47 72 27 02        	leaq	0x2277247(%rip), %rax   # 0x142a900f8
140818eb1: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140818eb6: 48 8d 0d 73 71 27 02        	leaq	0x2277173(%rip), %rcx   # 0x142a90030
140818ebd: ba 4c 00 00 00              	movl	$0x4c, %edx
140818ec2: 45 31 c0                    	xorl	%r8d, %r8d
140818ec5: e8 76 0a 0c 02              	callq	0x1428d9940 <.text+0x28d8940>
140818eca: 75 de                       	jne	0x140818eaa <.text+0x817eaa>
140818ecc: 4c 89 c9                    	movq	%r9, %rcx
140818ecf: e8 ec fa ff ff              	callq	0x1408189c0 <.text+0x8179c0>
140818ed4: 48 83 f8 01                 	cmpq	$0x1, %rax
140818ed8: 75 55                       	jne	0x140818f2f <.text+0x817f2f>
140818eda: 49 89 d0                    	movq	%rdx, %r8
140818edd: 49 83 c0 02                 	addq	$0x2, %r8
140818ee1: 74 39                       	je	0x140818f1c <.text+0x817f1c>
140818ee3: 49 83 f8 4c                 	cmpq	$0x4c, %r8
140818ee7: 73 31                       	jae	0x140818f1a <.text+0x817f1a>
140818ee9: 48 8d 05 40 71 27 02        	leaq	0x2277140(%rip), %rax   # 0x142a90030
140818ef0: 41 80 3c 00 bf              	cmpb	$-0x41, (%r8,%rax)
140818ef5: 7f 25                       	jg	0x140818f1c <.text+0x817f1c>
140818ef7: 48 8d 05 12 72 27 02        	leaq	0x2277212(%rip), %rax   # 0x142a90110
140818efe: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140818f03: 48 8d 0d 26 71 27 02        	leaq	0x2277126(%rip), %rcx   # 0x142a90030
140818f0a: ba 4c 00 00 00              	movl	$0x4c, %edx
140818f0f: 41 b9 4c 00 00 00           	movl	$0x4c, %r9d
140818f15: e8 26 0a 0c 02              	callq	0x1428d9940 <.text+0x28d8940>
140818f1a: 75 db                       	jne	0x140818ef7 <.text+0x817ef7>
140818f1c: 41 bf 4a 00 00 00           	movl	$0x4a, %r15d
140818f22: 49 29 d7                    	subq	%rdx, %r15
140818f25: 4c 8d 25 04 71 27 02        	leaq	0x2277104(%rip), %r12   # 0x142a90030
140818f2c: 4d 01 c4                    	addq	%r8, %r12
140818f2f: 4c 8d 05 46 72 27 02        	leaq	0x2277246(%rip), %r8    # 0x142a9017c
140818f36: 4c 8d 2d 03 72 27 02        	leaq	0x2277203(%rip), %r13   # 0x142a90140
140818f3d: 0f 1f 00                    	nopl	(%rax)
140818f40: 41 0f be 40 ff              	movsbl	-0x1(%r8), %eax
140818f45: 85 c0                       	testl	%eax, %eax
140818f47: 78 17                       	js	0x140818f60 <.text+0x817f60>
140818f49: 49 ff c8                    	decq	%r8
140818f4c: 83 f8 5c                    	cmpl	$0x5c, %eax
140818f4f: 75 63                       	jne	0x140818fb4 <.text+0x817fb4>
140818f51: eb 72                       	jmp	0x140818fc5 <.text+0x817fc5>
140818f53: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
140818f60: 41 0f b6 48 fe              	movzbl	-0x2(%r8), %ecx
140818f65: 80 f9 c0                    	cmpb	$-0x40, %cl
140818f68: 7d 23                       	jge	0x140818f8d <.text+0x817f8d>
140818f6a: 41 0f b6 50 fd              	movzbl	-0x3(%r8), %edx
140818f6f: 80 fa c0                    	cmpb	$-0x40, %dl
140818f72: 7d 22                       	jge	0x140818f96 <.text+0x817f96>
140818f74: 45 0f b6 48 fc              	movzbl	-0x4(%r8), %r9d
140818f79: 49 83 c0 fc                 	addq	$-0x4, %r8
140818f7d: 41 83 e1 07                 	andl	$0x7, %r9d
140818f81: 41 c1 e1 06                 	shll	$0x6, %r9d
140818f85: 83 e2 3f                    	andl	$0x3f, %edx
140818f88: 44 09 ca                    	orl	%r9d, %edx
140818f8b: eb 10                       	jmp	0x140818f9d <.text+0x817f9d>
140818f8d: 49 83 c0 fe                 	addq	$-0x2, %r8
140818f91: 83 e1 1f                    	andl	$0x1f, %ecx
140818f94: eb 0f                       	jmp	0x140818fa5 <.text+0x817fa5>
140818f96: 49 83 c0 fd                 	addq	$-0x3, %r8
140818f9a: 83 e2 0f                    	andl	$0xf, %edx
140818f9d: c1 e2 06                    	shll	$0x6, %edx
140818fa0: 83 e1 3f                    	andl	$0x3f, %ecx
140818fa3: 09 d1                       	orl	%edx, %ecx
140818fa5: c1 e1 06                    	shll	$0x6, %ecx
140818fa8: 24 3f                       	andb	$0x3f, %al
140818faa: 0f b6 c0                    	movzbl	%al, %eax
140818fad: 09 c8                       	orl	%ecx, %eax
140818faf: 83 f8 5c                    	cmpl	$0x5c, %eax
140818fb2: 74 11                       	je	0x140818fc5 <.text+0x817fc5>
140818fb4: 83 f8 2f                    	cmpl	$0x2f, %eax
140818fb7: 74 0c                       	je	0x140818fc5 <.text+0x817fc5>
140818fb9: 4d 39 e8                    	cmpq	%r13, %r8
140818fbc: 75 82                       	jne	0x140818f40 <.text+0x817f40>
140818fbe: bb 3c 00 00 00              	movl	$0x3c, %ebx
140818fc3: eb 4c                       	jmp	0x140819011 <.text+0x818011>
140818fc5: 4c 8d 2d 74 71 27 02        	leaq	0x2277174(%rip), %r13   # 0x142a90140
140818fcc: 4d 29 e8                    	subq	%r13, %r8
140818fcf: 49 ff c0                    	incq	%r8
140818fd2: 74 32                       	je	0x140819006 <.text+0x818006>
140818fd4: 49 83 f8 3c                 	cmpq	$0x3c, %r8
140818fd8: 73 2a                       	jae	0x140819004 <.text+0x818004>
140818fda: 43 80 3c 28 bf              	cmpb	$-0x41, (%r8,%r13)
140818fdf: 7f 25                       	jg	0x140819006 <.text+0x818006>
140818fe1: 48 8d 05 f0 70 27 02        	leaq	0x22770f0(%rip), %rax   # 0x142a900d8
140818fe8: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140818fed: 48 8d 0d 4c 71 27 02        	leaq	0x227714c(%rip), %rcx   # 0x142a90140
140818ff4: ba 3c 00 00 00              	movl	$0x3c, %edx
140818ff9: 41 b9 3c 00 00 00           	movl	$0x3c, %r9d
140818fff: e8 3c 09 0c 02              	callq	0x1428d9940 <.text+0x28d8940>
140819004: 75 db                       	jne	0x140818fe1 <.text+0x817fe1>
140819006: bb 3c 00 00 00              	movl	$0x3c, %ebx
14081900b: 4c 29 c3                    	subq	%r8, %rbx
14081900e: 4d 01 c5                    	addq	%r8, %r13
140819011: e8 1a f1 3f 00              	callq	0x140c18130 <.text+0xc17130>
140819016: 48 89 c1                    	movq	%rax, %rcx
140819019: 48 8b 00                    	movq	(%rax), %rax
14081901c: 48 83 f8 01                 	cmpq	$0x1, %rax
140819020: 0f 85 ef 08 00 00           	jne	0x140819915 <.text+0x818915>
140819026: 48 83 c1 08                 	addq	$0x8, %rcx
14081902a: 48 83 39 00                 	cmpq	$0x0, (%rcx)
14081902e: 0f 85 1f 09 00 00           	jne	0x140819953 <.text+0x818953>
140819034: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
14081903b: 48 89 4d 58                 	movq	%rcx, 0x58(%rbp)
14081903f: 48 83 c1 08                 	addq	$0x8, %rcx
140819043: 48 89 5c 24 20              	movq	%rbx, 0x20(%rsp)
140819048: 48 c7 44 24 30 00 00 00 00  	movq	$0x0, 0x30(%rsp)
140819051: 48 c7 44 24 28 01 00 00 00  	movq	$0x1, 0x28(%rsp)
14081905a: 4c 89 e2                    	movq	%r12, %rdx
14081905d: 4d 89 f8                    	movq	%r15, %r8
140819060: 4d 89 e9                    	movq	%r13, %r9
140819063: e8 d8 dc 3f 00              	callq	0x140c16d40 <.text+0xc15d40>
140819068: 48 89 45 d0                 	movq	%rax, -0x30(%rbp)
14081906c: 48 8b 45 58                 	movq	0x58(%rbp), %rax
140819070: 48 ff 00                    	incq	(%rax)
140819073: 49 8b 06                    	movq	(%r14), %rax
140819076: f3 0f 10 40 08              	movss	0x8(%rax), %xmm0
14081907b: f3 0f 10 0d 09 38 26 02     	movss	0x2263809(%rip), %xmm1  # 0x142a7c88c
140819083: f3 0f 10 15 b9 7c 26 02     	movss	0x2267cb9(%rip), %xmm2  # 0x142a80d44
14081908b: 0f c2 d0 02                 	cmpleps	%xmm0, %xmm2
14081908f: 0f c2 c1 01                 	cmpltps	%xmm1, %xmm0
140819093: 0f 54 c2                    	andps	%xmm2, %xmm0
140819096: f3 0f 11 45 28              	movss	%xmm0, 0x28(%rbp)
14081909b: 48 c7 45 f8 00 00 00 00     	movq	$0x0, -0x8(%rbp)
1408190a3: 48 c7 45 00 08 00 00 00     	movq	$0x8, (%rbp)
1408190ab: 48 c7 45 08 00 00 00 00     	movq	$0x0, 0x8(%rbp)
1408190b3: 48 8b 0e                    	movq	(%rsi), %rcx
1408190b6: 4c 8b 5e 08                 	movq	0x8(%rsi), %r11
1408190ba: 8b 46 14                    	movl	0x14(%rsi), %eax
1408190bd: 89 45 dc                    	movl	%eax, -0x24(%rbp)
1408190c0: 4c 8b a1 f8 00 00 00        	movq	0xf8(%rcx), %r12
1408190c7: 48 8b 81 00 01 00 00        	movq	0x100(%rcx), %rax
1408190ce: 4d 8d 3c 84                 	leaq	(%r12,%rax,4), %r15
1408190d2: 48 89 4d 38                 	movq	%rcx, 0x38(%rbp)
1408190d6: 0f b6 81 08 01 00 00        	movzbl	0x108(%rcx), %eax
1408190dd: 88 45 f0                    	movb	%al, -0x10(%rbp)
1408190e0: 8b 47 1c                    	movl	0x1c(%rdi), %eax
1408190e3: 89 45 4c                    	movl	%eax, 0x4c(%rbp)
1408190e6: 48 8b 07                    	movq	(%rdi), %rax
1408190e9: 48 8b 4f 10                 	movq	0x10(%rdi), %rcx
1408190ed: 48 89 4d e0                 	movq	%rcx, -0x20(%rbp)
1408190f1: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
1408190f5: 48 83 c0 20                 	addq	$0x20, %rax
1408190f9: 48 89 45 e8                 	movq	%rax, -0x18(%rbp)
1408190fd: b8 08 00 00 00              	movl	$0x8, %eax
140819102: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
140819106: 48 c7 45 50 00 00 00 00     	movq	$0x0, 0x50(%rbp)
14081910e: 66 0f 76 f6                 	pcmpeqd	%xmm6, %xmm6
140819112: 66 0f 6e 3d ea c7 10 02     	movd	0x210c7ea(%rip), %xmm7  # 0x142925904
14081911a: 31 ff                       	xorl	%edi, %edi
14081911c: 31 db                       	xorl	%ebx, %ebx
14081911e: 4c 89 5d 20                 	movq	%r11, 0x20(%rbp)
140819122: 80 7d f0 00                 	cmpb	$0x0, -0x10(%rbp)
140819126: 0f 84 84 00 00 00           	je	0x1408191b0 <.text+0x8181b0>
14081912c: 48 39 fb                    	cmpq	%rdi, %rbx
14081912f: 0f 85 10 01 00 00           	jne	0x140819245 <.text+0x818245>
140819135: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
140819140: 4d 39 fc                    	cmpq	%r15, %r12
140819143: 0f 84 64 03 00 00           	je	0x1408194ad <.text+0x8184ad>
140819149: 41 8b 0c 24                 	movl	(%r12), %ecx
14081914d: 49 83 c4 04                 	addq	$0x4, %r12
140819151: 49 8b 83 a8 01 00 00        	movq	0x1a8(%r11), %rax
140819158: 48 8d 0c c9                 	leaq	(%rcx,%rcx,8), %rcx
14081915c: 48 8b 7c c8 10              	movq	0x10(%rax,%rcx,8), %rdi
140819161: 48 85 ff                    	testq	%rdi, %rdi
140819164: 74 da                       	je	0x140819140 <.text+0x818140>
140819166: 48 8d 04 c8                 	leaq	(%rax,%rcx,8), %rax
14081916a: 48 8b 4d 38                 	movq	0x38(%rbp), %rcx
14081916e: 48 8b 89 10 01 00 00        	movq	0x110(%rcx), %rcx
140819175: 48 39 48 40                 	cmpq	%rcx, 0x40(%rax)
140819179: 0f 86 bc 00 00 00           	jbe	0x14081923b <.text+0x81823b>
14081917f: 48 8b 50 38                 	movq	0x38(%rax), %rdx
140819183: 48 8b 0c ca                 	movq	(%rdx,%rcx,8), %rcx
140819187: 48 85 c9                    	testq	%rcx, %rcx
14081918a: 0f 84 ab 00 00 00           	je	0x14081923b <.text+0x81823b>
140819190: 48 8b 40 18                 	movq	0x18(%rax), %rax
140819194: 48 f7 d1                    	notq	%rcx
140819197: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
14081919b: 48 c1 e1 04                 	shlq	$0x4, %rcx
14081919f: 48 8b 44 08 10              	movq	0x10(%rax,%rcx), %rax
1408191a4: 48 89 45 50                 	movq	%rax, 0x50(%rbp)
1408191a8: e9 96 00 00 00              	jmp	0x140819243 <.text+0x818243>
1408191ad: 0f 1f 00                    	nopl	(%rax)
1408191b0: 48 39 fb                    	cmpq	%rdi, %rbx
1408191b3: 0f 85 a6 00 00 00           	jne	0x14081925f <.text+0x81825f>
1408191b9: 0f 1f 80 00 00 00 00        	nopl	(%rax)
1408191c0: 4d 39 fc                    	cmpq	%r15, %r12
1408191c3: 0f 84 e4 02 00 00           	je	0x1408194ad <.text+0x8184ad>
1408191c9: 41 8b 0c 24                 	movl	(%r12), %ecx
1408191cd: 49 83 c4 04                 	addq	$0x4, %r12
1408191d1: 49 8b 83 00 01 00 00        	movq	0x100(%r11), %rax
1408191d8: 48 8d 0c 89                 	leaq	(%rcx,%rcx,4), %rcx
1408191dc: 48 c1 e1 05                 	shlq	$0x5, %rcx
1408191e0: 48 8b 7c 08 58              	movq	0x58(%rax,%rcx), %rdi
1408191e5: 48 85 ff                    	testq	%rdi, %rdi
1408191e8: 74 d6                       	je	0x1408191c0 <.text+0x8181c0>
1408191ea: 48 01 c8                    	addq	%rcx, %rax
1408191ed: 8b 88 94 00 00 00           	movl	0x94(%rax), %ecx
1408191f3: 49 8b 93 a8 01 00 00        	movq	0x1a8(%r11), %rdx
1408191fa: 4c 8d 04 c9                 	leaq	(%rcx,%rcx,8), %r8
1408191fe: 48 8b 4d 38                 	movq	0x38(%rbp), %rcx
140819202: 48 8b 89 10 01 00 00        	movq	0x110(%rcx), %rcx
140819209: 4a 39 4c c2 40              	cmpq	%rcx, 0x40(%rdx,%r8,8)
14081920e: 76 3d                       	jbe	0x14081924d <.text+0x81824d>
140819210: 4a 8d 14 c2                 	leaq	(%rdx,%r8,8), %rdx
140819214: 4c 8b 42 38                 	movq	0x38(%rdx), %r8
140819218: 49 8b 0c c8                 	movq	(%r8,%rcx,8), %rcx
14081921c: 48 85 c9                    	testq	%rcx, %rcx
14081921f: 74 2c                       	je	0x14081924d <.text+0x81824d>
140819221: 48 8b 52 18                 	movq	0x18(%rdx), %rdx
140819225: 48 f7 d1                    	notq	%rcx
140819228: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
14081922c: 48 c1 e1 04                 	shlq	$0x4, %rcx
140819230: 48 8b 4c 0a 10              	movq	0x10(%rdx,%rcx), %rcx
140819235: 48 89 4d 50                 	movq	%rcx, 0x50(%rbp)
140819239: eb 1a                       	jmp	0x140819255 <.text+0x818255>
14081923b: 48 c7 45 50 00 00 00 00     	movq	$0x0, 0x50(%rbp)
140819243: 31 db                       	xorl	%ebx, %ebx
140819245: 48 89 5d 30                 	movq	%rbx, 0x30(%rbp)
140819249: 89 db                       	movl	%ebx, %ebx
14081924b: eb 25                       	jmp	0x140819272 <.text+0x818272>
14081924d: 48 c7 45 50 00 00 00 00     	movq	$0x0, 0x50(%rbp)
140819255: 48 8b 40 50                 	movq	0x50(%rax), %rax
140819259: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
14081925d: 31 db                       	xorl	%ebx, %ebx
14081925f: 48 89 5d 30                 	movq	%rbx, 0x30(%rbp)
140819263: 48 89 d8                    	movq	%rbx, %rax
140819266: 48 c1 e0 04                 	shlq	$0x4, %rax
14081926a: 48 8b 4d 40                 	movq	0x40(%rbp), %rcx
14081926e: 8b 5c 01 08                 	movl	0x8(%rcx,%rax), %ebx
140819272: 48 c1 e3 04                 	shlq	$0x4, %rbx
140819276: 48 03 5d 50                 	addq	0x50(%rbp), %rbx
14081927a: 4c 8b 2b                    	movq	(%rbx), %r13
14081927d: 48 8b 75 08                 	movq	0x8(%rbp), %rsi
140819281: 48 3b 75 f8                 	cmpq	-0x8(%rbp), %rsi
140819285: 75 10                       	jne	0x140819297 <.text+0x818297>
140819287: 48 8d 4d f8                 	leaq	-0x8(%rbp), %rcx
14081928b: 48 8d 15 06 6f 27 02        	leaq	0x2276f06(%rip), %rdx   # 0x142a90198
140819292: e8 59 8b 36 00              	callq	0x140b81df0 <.text+0xb80df0>
140819297: 48 ff 45 30                 	incq	0x30(%rbp)
14081929b: 48 8b 45 00                 	movq	(%rbp), %rax
14081929f: 4c 89 2c f0                 	movq	%r13, (%rax,%rsi,8)
1408192a3: 48 ff c6                    	incq	%rsi
1408192a6: 48 89 75 08                 	movq	%rsi, 0x8(%rbp)
1408192aa: 8b 45 4c                    	movl	0x4c(%rbp), %eax
1408192ad: 48 8b 4d e0                 	movq	-0x20(%rbp), %rcx
1408192b1: 89 01                       	movl	%eax, (%rcx)
1408192b3: 48 8b 33                    	movq	(%rbx), %rsi
1408192b6: 48 89 75 10                 	movq	%rsi, 0x10(%rbp)
1408192ba: 48 8b 4d e8                 	movq	-0x18(%rbp), %rcx
1408192be: 48 8d 55 10                 	leaq	0x10(%rbp), %rdx
1408192c2: e8 a9 7a d9 ff              	callq	0x1405b0d70 <.text+0x5afd70>
1408192c7: 49 89 c5                    	movq	%rax, %r13
1408192ca: 48 8b 45 58                 	movq	0x58(%rbp), %rax
1408192ce: 48 8b 08                    	movq	(%rax), %rcx
1408192d1: 48 8b 50 08                 	movq	0x8(%rax), %rdx
1408192d5: 48 89 55 18                 	movq	%rdx, 0x18(%rbp)
1408192d9: 4c 21 ea                    	andq	%r13, %rdx
1408192dc: 4d 89 e8                    	movq	%r13, %r8
1408192df: 49 c1 e8 39                 	shrq	$0x39, %r8
1408192e3: 66 41 0f 6e c0              	movd	%r8d, %xmm0
1408192e8: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1408192ec: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1408192f1: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1408192f6: 4c 8d 41 f0                 	leaq	-0x10(%rcx), %r8
1408192fa: 45 31 f6                    	xorl	%r14d, %r14d
1408192fd: f3 0f 6f 0c 11              	movdqu	(%rcx,%rdx), %xmm1
140819302: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
140819306: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
14081930a: 66 44 0f d7 d2              	pmovmskb	%xmm2, %r10d
14081930f: 45 85 d2                    	testl	%r10d, %r10d
140819312: 74 3c                       	je	0x140819350 <.text+0x818350>
140819314: 4c 89 f8                    	movq	%r15, %rax
140819317: f3 45 0f bc da              	tzcntl	%r10d, %r11d
14081931c: 49 01 d3                    	addq	%rdx, %r11
14081931f: 4c 23 5d 18                 	andq	0x18(%rbp), %r11
140819323: 4d 89 d9                    	movq	%r11, %r9
140819326: 49 c1 e1 04                 	shlq	$0x4, %r9
14081932a: 4d 89 c7                    	movq	%r8, %r15
14081932d: 4d 29 c8                    	subq	%r9, %r8
140819330: 49 39 30                    	cmpq	%rsi, (%r8)
140819333: 74 4b                       	je	0x140819380 <.text+0x818380>
140819335: 45 8d 42 ff                 	leal	-0x1(%r10), %r8d
140819339: 66 45 21 d0                 	andw	%r10w, %r8w
14081933d: 45 89 c2                    	movl	%r8d, %r10d
140819340: 4d 89 f8                    	movq	%r15, %r8
140819343: 49 89 c7                    	movq	%rax, %r15
140819346: 75 cc                       	jne	0x140819314 <.text+0x818314>
140819348: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
140819350: 4d 89 c2                    	movq	%r8, %r10
140819353: 66 0f 74 ce                 	pcmpeqb	%xmm6, %xmm1
140819357: 66 44 0f d7 c1              	pmovmskb	%xmm1, %r8d
14081935c: 45 85 c0                    	testl	%r8d, %r8d
14081935f: 75 3f                       	jne	0x1408193a0 <.text+0x8183a0>
140819361: 4c 01 f2                    	addq	%r14, %rdx
140819364: 48 83 c2 10                 	addq	$0x10, %rdx
140819368: 49 83 c6 10                 	addq	$0x10, %r14
14081936c: 48 23 55 18                 	andq	0x18(%rbp), %rdx
140819370: 4d 89 d0                    	movq	%r10, %r8
140819373: eb 88                       	jmp	0x1408192fd <.text+0x8182fd>
140819375: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
140819380: 49 f7 db                    	negq	%r11
140819383: 49 c1 e3 04                 	shlq	$0x4, %r11
140819387: 4c 01 d9                    	addq	%r11, %rcx
14081938a: 45 31 f6                    	xorl	%r14d, %r14d
14081938d: 49 89 cd                    	movq	%rcx, %r13
140819390: 48 8b 75 58                 	movq	0x58(%rbp), %rsi
140819394: 49 89 c7                    	movq	%rax, %r15
140819397: eb 19                       	jmp	0x1408193b2 <.text+0x8183b2>
140819399: 0f 1f 80 00 00 00 00        	nopl	(%rax)
1408193a0: 48 8b 4d 58                 	movq	0x58(%rbp), %rcx
1408193a4: 48 83 79 10 00              	cmpq	$0x0, 0x10(%rcx)
1408193a9: 0f 84 e5 00 00 00           	je	0x140819494 <.text+0x818494>
1408193af: 49 89 ce                    	movq	%rcx, %r14
1408193b2: 66 0f ef c0                 	pxor	%xmm0, %xmm0
1408193b6: f6 45 28 01                 	testb	$0x1, 0x28(%rbp)
1408193ba: 74 14                       	je	0x1408193d0 <.text+0x8183d0>
1408193bc: 4c 8b 5d 20                 	movq	0x20(%rbp), %r11
1408193c0: 48 8b 5d 30                 	movq	0x30(%rbp), %rbx
1408193c4: eb 1c                       	jmp	0x1408193e2 <.text+0x8183e2>
1408193c6: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
1408193d0: 80 7b 0d 01                 	cmpb	$0x1, 0xd(%rbx)
1408193d4: 4c 8b 5d 20                 	movq	0x20(%rbp), %r11
1408193d8: 48 8b 5d 30                 	movq	0x30(%rbp), %rbx
1408193dc: 75 04                       	jne	0x1408193e2 <.text+0x8183e2>
1408193de: 66 0f 6f c7                 	movdqa	%xmm7, %xmm0
1408193e2: 4d 85 f6                    	testq	%r14, %r14
1408193e5: 0f 84 37 fd ff ff           	je	0x140819122 <.text+0x818122>
1408193eb: 49 8b 0e                    	movq	(%r14), %rcx
1408193ee: 49 8b 56 08                 	movq	0x8(%r14), %rdx
1408193f2: 49 89 d0                    	movq	%rdx, %r8
1408193f5: 4d 21 e8                    	andq	%r13, %r8
1408193f8: f3 42 0f 6f 0c 01           	movdqu	(%rcx,%r8), %xmm1
1408193fe: 66 44 0f d7 c9              	pmovmskb	%xmm1, %r9d
140819403: 45 85 c9                    	testl	%r9d, %r9d
140819406: 74 55                       	je	0x14081945d <.text+0x81845d>
140819408: f3 45 0f bc c9              	tzcntl	%r9d, %r9d
14081940d: 4d 01 c1                    	addq	%r8, %r9
140819410: 49 21 d1                    	andq	%rdx, %r9
140819413: 46 0f b6 04 09              	movzbl	(%rcx,%r9), %r8d
140819418: 45 84 c0                    	testb	%r8b, %r8b
14081941b: 79 62                       	jns	0x14081947f <.text+0x81847f>
14081941d: 49 c1 ed 39                 	shrq	$0x39, %r13
140819421: 4d 8d 51 f0                 	leaq	-0x10(%r9), %r10
140819425: 49 21 d2                    	andq	%rdx, %r10
140819428: 46 88 2c 09                 	movb	%r13b, (%rcx,%r9)
14081942c: 46 88 6c 11 10              	movb	%r13b, 0x10(%rcx,%r10)
140819431: 49 c1 e1 04                 	shlq	$0x4, %r9
140819435: 41 80 e0 01                 	andb	$0x1, %r8b
140819439: 41 0f b6 d0                 	movzbl	%r8b, %edx
14081943d: 49 29 56 10                 	subq	%rdx, 0x10(%r14)
140819441: 48 89 ca                    	movq	%rcx, %rdx
140819444: 4c 29 ca                    	subq	%r9, %rdx
140819447: 49 f7 d9                    	negq	%r9
14081944a: 4a 89 74 09 f0              	movq	%rsi, -0x10(%rcx,%r9)
14081944f: 66 0f 7e 42 f8              	movd	%xmm0, -0x8(%rdx)
140819454: 49 ff 46 18                 	incq	0x18(%r14)
140819458: e9 c5 fc ff ff              	jmp	0x140819122 <.text+0x818122>
14081945d: 41 ba 10 00 00 00           	movl	$0x10, %r10d
140819463: 4d 01 d0                    	addq	%r10, %r8
140819466: 49 21 d0                    	andq	%rdx, %r8
140819469: f3 42 0f 6f 0c 01           	movdqu	(%rcx,%r8), %xmm1
14081946f: 66 44 0f d7 c9              	pmovmskb	%xmm1, %r9d
140819474: 49 83 c2 10                 	addq	$0x10, %r10
140819478: 45 85 c9                    	testl	%r9d, %r9d
14081947b: 75 8b                       	jne	0x140819408 <.text+0x818408>
14081947d: eb e4                       	jmp	0x140819463 <.text+0x818463>
14081947f: 66 0f 6f 09                 	movdqa	(%rcx), %xmm1
140819483: 66 44 0f d7 c1              	pmovmskb	%xmm1, %r8d
140819488: f3 45 0f bc c8              	tzcntl	%r8d, %r9d
14081948d: 46 0f b6 04 09              	movzbl	(%rcx,%r9), %r8d
140819492: eb 89                       	jmp	0x14081941d <.text+0x81841d>
140819494: ba 01 00 00 00              	movl	$0x1, %edx
140819499: 4c 8b 45 e8                 	movq	-0x18(%rbp), %r8
14081949d: 41 b1 01                    	movb	$0x1, %r9b
1408194a0: 49 89 ce                    	movq	%rcx, %r14
1408194a3: e8 a8 0d 09 02              	callq	0x1428aa250 <.text+0x28a9250>
1408194a8: e9 05 ff ff ff              	jmp	0x1408193b2 <.text+0x8183b2>
1408194ad: 8b 45 4c                    	movl	0x4c(%rbp), %eax
1408194b0: 48 8b 4d e0                 	movq	-0x20(%rbp), %rcx
1408194b4: 89 01                       	movl	%eax, (%rcx)
1408194b6: 48 8d 55 f8                 	leaq	-0x8(%rbp), %rdx
1408194ba: 48 8b 4d 58                 	movq	0x58(%rbp), %rcx
1408194be: e8 1d e9 a4 00              	callq	0x141267de0 <.text+0x1266de0>
1408194c3: 48 8b 45 38                 	movq	0x38(%rbp), %rax
1408194c7: 48 8b b8 f8 00 00 00        	movq	0xf8(%rax), %rdi
1408194ce: 48 8b 80 00 01 00 00        	movq	0x100(%rax), %rax
1408194d5: f3 0f 10 3d 07 0c 14 02     	movss	0x2140c07(%rip), %xmm7  # 0x14295a0e4
1408194dd: f3 44 0f 10 05 1e c4 10 02  	movss	0x210c41e(%rip), %xmm8  # 0x142925904
1408194e6: 44 0f 28 cf                 	movaps	%xmm7, %xmm9
1408194ea: f6 45 28 01                 	testb	$0x1, 0x28(%rbp)
1408194ee: 75 04                       	jne	0x1408194f4 <.text+0x8184f4>
1408194f0: 45 0f 28 c8                 	movaps	%xmm8, %xmm9
1408194f4: 4c 8d 3c 87                 	leaq	(%rdi,%rax,4), %r15
1408194f8: 48 8b 45 38                 	movq	0x38(%rbp), %rax
1408194fc: 0f b6 88 08 01 00 00        	movzbl	0x108(%rax), %ecx
140819503: 48 8b 45 c0                 	movq	-0x40(%rbp), %rax
140819507: 48 8b 00                    	movq	(%rax), %rax
14081950a: 48 89 45 18                 	movq	%rax, 0x18(%rbp)
14081950e: b8 08 00 00 00              	movl	$0x8, %eax
140819513: 48 89 45 28                 	movq	%rax, 0x28(%rbp)
140819517: 45 31 c0                    	xorl	%r8d, %r8d
14081951a: 48 8d 75 10                 	leaq	0x10(%rbp), %rsi
14081951e: f3 44 0f 10 15 6d b1 11 02  	movss	0x211b16d(%rip), %xmm10 # 0x142934694
140819527: 66 45 0f 76 db              	pcmpeqd	%xmm11, %xmm11
14081952c: f3 44 0f 10 25 43 3b 1b 02  	movss	0x21b3b43(%rip), %xmm12 # 0x1429cd078
140819535: 45 31 ed                    	xorl	%r13d, %r13d
140819538: 31 db                       	xorl	%ebx, %ebx
14081953a: 88 4d 50                    	movb	%cl, 0x50(%rbp)
14081953d: 84 c9                       	testb	%cl, %cl
14081953f: 0f 85 cb 00 00 00           	jne	0x140819610 <.text+0x818610>
140819545: 4c 39 eb                    	cmpq	%r13, %rbx
140819548: 0f 85 aa 00 00 00           	jne	0x1408195f8 <.text+0x8185f8>
14081954e: 4c 39 ff                    	cmpq	%r15, %rdi
140819551: 0f 84 fa 02 00 00           	je	0x140819851 <.text+0x818851>
140819557: 48 8b 45 20                 	movq	0x20(%rbp), %rax
14081955b: 48 8b 80 00 01 00 00        	movq	0x100(%rax), %rax
140819562: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
140819570: 8b 0f                       	movl	(%rdi), %ecx
140819572: 48 8d 0c 89                 	leaq	(%rcx,%rcx,4), %rcx
140819576: 48 c1 e1 05                 	shlq	$0x5, %rcx
14081957a: 4c 8b 6c 08 58              	movq	0x58(%rax,%rcx), %r13
14081957f: 4d 85 ed                    	testq	%r13, %r13
140819582: 75 1c                       	jne	0x1408195a0 <.text+0x8185a0>
140819584: 48 83 c7 04                 	addq	$0x4, %rdi
140819588: 4c 39 ff                    	cmpq	%r15, %rdi
14081958b: 75 e3                       	jne	0x140819570 <.text+0x818570>
14081958d: e9 bf 02 00 00              	jmp	0x140819851 <.text+0x818851>
140819592: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
1408195a0: 48 01 c8                    	addq	%rcx, %rax
1408195a3: 8b 88 94 00 00 00           	movl	0x94(%rax), %ecx
1408195a9: 48 8b 55 20                 	movq	0x20(%rbp), %rdx
1408195ad: 48 8b 92 a8 01 00 00        	movq	0x1a8(%rdx), %rdx
1408195b4: 48 8d 0c c9                 	leaq	(%rcx,%rcx,8), %rcx
1408195b8: 4c 8b 45 38                 	movq	0x38(%rbp), %r8
1408195bc: 4d 8b 80 10 01 00 00        	movq	0x110(%r8), %r8
1408195c3: 4c 8b 4c ca 18              	movq	0x18(%rdx,%rcx,8), %r9
1408195c8: 48 8b 4c ca 38              	movq	0x38(%rdx,%rcx,8), %rcx
1408195cd: 4a 8b 0c c1                 	movq	(%rcx,%r8,8), %rcx
1408195d1: 48 f7 d1                    	notq	%rcx
1408195d4: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
1408195d8: 48 c1 e1 04                 	shlq	$0x4, %rcx
1408195dc: 4d 8b 44 09 10              	movq	0x10(%r9,%rcx), %r8
1408195e1: 49 8b 4c 09 28              	movq	0x28(%r9,%rcx), %rcx
1408195e6: 48 89 4d 40                 	movq	%rcx, 0x40(%rbp)
1408195ea: 48 8b 40 50                 	movq	0x50(%rax), %rax
1408195ee: 48 89 45 28                 	movq	%rax, 0x28(%rbp)
1408195f2: 48 83 c7 04                 	addq	$0x4, %rdi
1408195f6: 31 db                       	xorl	%ebx, %ebx
1408195f8: 48 89 d8                    	movq	%rbx, %rax
1408195fb: 48 c1 e0 04                 	shlq	$0x4, %rax
1408195ff: 48 8b 4d 28                 	movq	0x28(%rbp), %rcx
140819603: 44 8b 74 01 08              	movl	0x8(%rcx,%rax), %r14d
140819608: e9 80 00 00 00              	jmp	0x14081968d <.text+0x81868d>
14081960d: 0f 1f 00                    	nopl	(%rax)
140819610: 4c 39 eb                    	cmpq	%r13, %rbx
140819613: 75 75                       	jne	0x14081968a <.text+0x81868a>
140819615: 4c 39 ff                    	cmpq	%r15, %rdi
140819618: 0f 84 33 02 00 00           	je	0x140819851 <.text+0x818851>
14081961e: 48 8b 45 20                 	movq	0x20(%rbp), %rax
140819622: 48 8b 80 a8 01 00 00        	movq	0x1a8(%rax), %rax
140819629: 0f 1f 80 00 00 00 00        	nopl	(%rax)
140819630: 8b 0f                       	movl	(%rdi), %ecx
140819632: 48 8d 0c c9                 	leaq	(%rcx,%rcx,8), %rcx
140819636: 4c 8b 6c c8 10              	movq	0x10(%rax,%rcx,8), %r13
14081963b: 4d 85 ed                    	testq	%r13, %r13
14081963e: 75 10                       	jne	0x140819650 <.text+0x818650>
140819640: 48 83 c7 04                 	addq	$0x4, %rdi
140819644: 4c 39 ff                    	cmpq	%r15, %rdi
140819647: 75 e7                       	jne	0x140819630 <.text+0x818630>
140819649: e9 03 02 00 00              	jmp	0x140819851 <.text+0x818851>
14081964e: 66 90                       	nop
140819650: 48 8d 04 c8                 	leaq	(%rax,%rcx,8), %rax
140819654: 48 8b 4d 38                 	movq	0x38(%rbp), %rcx
140819658: 48 8b 89 10 01 00 00        	movq	0x110(%rcx), %rcx
14081965f: 48 8b 50 18                 	movq	0x18(%rax), %rdx
140819663: 48 8b 40 38                 	movq	0x38(%rax), %rax
140819667: 48 8b 04 c8                 	movq	(%rax,%rcx,8), %rax
14081966b: 48 f7 d0                    	notq	%rax
14081966e: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
140819672: 48 c1 e0 04                 	shlq	$0x4, %rax
140819676: 4c 8b 44 02 10              	movq	0x10(%rdx,%rax), %r8
14081967b: 48 8b 44 02 28              	movq	0x28(%rdx,%rax), %rax
140819680: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
140819684: 48 83 c7 04                 	addq	$0x4, %rdi
140819688: 31 db                       	xorl	%ebx, %ebx
14081968a: 41 89 de                    	movl	%ebx, %r14d
14081968d: 48 8b 45 40                 	movq	0x40(%rbp), %rax
140819691: 4e 8d 24 b0                 	leaq	(%rax,%r14,4), %r12
140819695: 49 c1 e6 04                 	shlq	$0x4, %r14
140819699: 4d 01 c6                    	addq	%r8, %r14
14081969c: 4d 85 f6                    	testq	%r14, %r14
14081969f: 0f 84 ac 01 00 00           	je	0x140819851 <.text+0x818851>
1408196a5: 4c 89 65 f0                 	movq	%r12, -0x10(%rbp)
1408196a9: 4c 89 45 30                 	movq	%r8, 0x30(%rbp)
1408196ad: 41 8b 0e                    	movl	(%r14), %ecx
1408196b0: e8 8b cb 47 00              	callq	0x140c96240 <.text+0xc95240>
1408196b5: 48 89 45 10                 	movq	%rax, 0x10(%rbp)
1408196b9: 48 89 f1                    	movq	%rsi, %rcx
1408196bc: e8 bf 5c 49 00              	callq	0x140caf380 <.text+0xcae380>
1408196c1: 0f 28 f0                    	movaps	%xmm0, %xmm6
1408196c4: f3 41 0f 59 f2              	mulss	%xmm10, %xmm6
1408196c9: 41 80 7e 0d 00              	cmpb	$0x0, 0xd(%r14)
1408196ce: 74 20                       	je	0x1408196f0 <.text+0x8186f0>
1408196d0: 41 0f 28 c8                 	movaps	%xmm8, %xmm1
1408196d4: f3 0f 5c c8                 	subss	%xmm0, %xmm1
1408196d8: f3 0f 58 c9                 	addss	%xmm1, %xmm1
1408196dc: f3 0f 58 f1                 	addss	%xmm1, %xmm6
1408196e0: 45 0f 28 e9                 	movaps	%xmm9, %xmm13
1408196e4: eb 2a                       	jmp	0x140819710 <.text+0x818710>
1408196e6: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
1408196f0: 48 89 f1                    	movq	%rsi, %rcx
1408196f3: e8 88 5c 49 00              	callq	0x140caf380 <.text+0xcae380>
1408196f8: 0f 28 f0                    	movaps	%xmm0, %xmm6
1408196fb: 41 0f 28 c0                 	movaps	%xmm8, %xmm0
1408196ff: f3 0f 5c c6                 	subss	%xmm6, %xmm0
140819703: f3 41 0f 59 f4              	mulss	%xmm12, %xmm6
140819708: f3 0f 58 f0                 	addss	%xmm0, %xmm6
14081970c: 44 0f 28 ef                 	movaps	%xmm7, %xmm13
140819710: 8b 45 4c                    	movl	0x4c(%rbp), %eax
140819713: 48 8b 4d e0                 	movq	-0x20(%rbp), %rcx
140819717: 89 01                       	movl	%eax, (%rcx)
140819719: 48 8b 75 58                 	movq	0x58(%rbp), %rsi
14081971d: 48 83 7e 18 00              	cmpq	$0x0, 0x18(%rsi)
140819722: 0f 84 df 01 00 00           	je	0x140819907 <.text+0x818907>
140819728: 48 ff c3                    	incq	%rbx
14081972b: 48 8b 4d e8                 	movq	-0x18(%rbp), %rcx
14081972f: 4c 89 f2                    	movq	%r14, %rdx
140819732: e8 39 76 d9 ff              	callq	0x1405b0d70 <.text+0x5afd70>
140819737: 48 8b 0e                    	movq	(%rsi), %rcx
14081973a: 48 8b 76 08                 	movq	0x8(%rsi), %rsi
14081973e: 49 89 f0                    	movq	%rsi, %r8
140819741: 49 21 c0                    	andq	%rax, %r8
140819744: 48 c1 e8 39                 	shrq	$0x39, %rax
140819748: 66 0f 6e c0                 	movd	%eax, %xmm0
14081974c: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140819750: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140819755: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
14081975a: 4c 8d 49 f0                 	leaq	-0x10(%rcx), %r9
14081975e: 45 31 e4                    	xorl	%r12d, %r12d
140819761: f3 42 0f 6f 0c 01           	movdqu	(%rcx,%r8), %xmm1
140819767: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
14081976b: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
14081976f: 66 44 0f d7 d2              	pmovmskb	%xmm2, %r10d
140819774: 45 85 d2                    	testl	%r10d, %r10d
140819777: 74 47                       	je	0x1408197c0 <.text+0x8187c0>
140819779: 49 8b 16                    	movq	(%r14), %rdx
14081977c: 4c 89 7d c8                 	movq	%r15, -0x38(%rbp)
140819780: f3 45 0f bc da              	tzcntl	%r10d, %r11d
140819785: 4d 01 c3                    	addq	%r8, %r11
140819788: 49 21 f3                    	andq	%rsi, %r11
14081978b: 49 89 f7                    	movq	%rsi, %r15
14081978e: 4c 89 de                    	movq	%r11, %rsi
140819791: 48 c1 e6 04                 	shlq	$0x4, %rsi
140819795: 4c 89 c8                    	movq	%r9, %rax
140819798: 48 29 f0                    	subq	%rsi, %rax
14081979b: 48 3b 10                    	cmpq	(%rax), %rdx
14081979e: 74 50                       	je	0x1408197f0 <.text+0x8187f0>
1408197a0: 41 8d 42 ff                 	leal	-0x1(%r10), %eax
1408197a4: 66 44 21 d0                 	andw	%r10w, %ax
1408197a8: 41 89 c2                    	movl	%eax, %r10d
1408197ab: 4c 89 fe                    	movq	%r15, %rsi
1408197ae: 4c 8b 7d c8                 	movq	-0x38(%rbp), %r15
1408197b2: 75 cc                       	jne	0x140819780 <.text+0x818780>
1408197b4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
1408197c0: 66 41 0f 74 cb              	pcmpeqb	%xmm11, %xmm1
1408197c5: 66 0f d7 c1                 	pmovmskb	%xmm1, %eax
1408197c9: 85 c0                       	testl	%eax, %eax
1408197cb: 0f 85 36 01 00 00           	jne	0x140819907 <.text+0x818907>
1408197d1: 4d 01 e0                    	addq	%r12, %r8
1408197d4: 49 83 c0 10                 	addq	$0x10, %r8
1408197d8: 49 83 c4 10                 	addq	$0x10, %r12
1408197dc: 49 21 f0                    	andq	%rsi, %r8
1408197df: eb 80                       	jmp	0x140819761 <.text+0x818761>
1408197e1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
1408197f0: 49 f7 db                    	negq	%r11
1408197f3: 48 8b 45 18                 	movq	0x18(%rbp), %rax
1408197f7: f3 44 0f 59 68 28           	mulss	0x28(%rax), %xmm13
1408197fd: 49 c1 e3 04                 	shlq	$0x4, %r11
140819801: f3 44 0f 5e ee              	divss	%xmm6, %xmm13
140819806: f3 46 0f 58 6c 19 f8        	addss	-0x8(%rcx,%r11), %xmm13
14081980d: 66 0f ef c0                 	pxor	%xmm0, %xmm0
140819811: f3 41 0f 5f c5              	maxss	%xmm13, %xmm0
140819816: 41 0f 28 c8                 	movaps	%xmm8, %xmm1
14081981a: f3 0f 5d c8                 	minss	%xmm0, %xmm1
14081981e: f3 42 0f 11 4c 19 f8        	movss	%xmm1, -0x8(%rcx,%r11)
140819825: 8b 45 dc                    	movl	-0x24(%rbp), %eax
140819828: 48 8b 4d f0                 	movq	-0x10(%rbp), %rcx
14081982c: 89 01                       	movl	%eax, (%rcx)
14081982e: f3 41 0f 11 4e 08           	movss	%xmm1, 0x8(%r14)
140819834: 0f b6 4d 50                 	movzbl	0x50(%rbp), %ecx
140819838: 48 8d 75 10                 	leaq	0x10(%rbp), %rsi
14081983c: 4c 8b 45 30                 	movq	0x30(%rbp), %r8
140819840: 4c 8b 7d c8                 	movq	-0x38(%rbp), %r15
140819844: 84 c9                       	testb	%cl, %cl
140819846: 0f 85 c4 fd ff ff           	jne	0x140819610 <.text+0x818610>
14081984c: e9 f4 fc ff ff              	jmp	0x140819545 <.text+0x818545>
140819851: 48 8b 55 f8                 	movq	-0x8(%rbp), %rdx
140819855: 48 85 d2                    	testq	%rdx, %rdx
140819858: 74 13                       	je	0x14081986d <.text+0x81886d>
14081985a: 48 8b 4d 00                 	movq	(%rbp), %rcx
14081985e: 48 c1 e2 03                 	shlq	$0x3, %rdx
140819862: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140819868: e8 b3 a3 df ff              	callq	0x140613c20 <.text+0x612c20>
14081986d: 80 7d 67 00                 	cmpb	$0x0, 0x67(%rbp)
140819871: 48 8b 75 d0                 	movq	-0x30(%rbp), %rsi
140819875: 74 41                       	je	0x1408198b8 <.text+0x8188b8>
140819877: e8 b4 e8 3f 00              	callq	0x140c18130 <.text+0xc17130>
14081987c: 48 89 c1                    	movq	%rax, %rcx
14081987f: 48 8b 00                    	movq	(%rax), %rax
140819882: 48 83 f8 01                 	cmpq	$0x1, %rax
140819886: 0f 85 a3 00 00 00           	jne	0x14081992f <.text+0x81892f>
14081988c: 48 83 c1 08                 	addq	$0x8, %rcx
140819890: 48 83 39 00                 	cmpq	$0x0, (%rcx)
140819894: 0f 85 b9 00 00 00           	jne	0x140819953 <.text+0x818953>
14081989a: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
1408198a1: 48 89 4d 58                 	movq	%rcx, 0x58(%rbp)
1408198a5: 48 83 c1 08                 	addq	$0x8, %rcx
1408198a9: 48 89 f2                    	movq	%rsi, %rdx
1408198ac: e8 3f d8 3f 00              	callq	0x140c170f0 <.text+0xc160f0>
1408198b1: 48 8b 45 58                 	movq	0x58(%rbp), %rax
1408198b5: 48 ff 00                    	incq	(%rax)
1408198b8: 0f 28 75 70                 	movaps	0x70(%rbp), %xmm6
1408198bc: 0f 28 bd 80 00 00 00        	movaps	0x80(%rbp), %xmm7
1408198c3: 44 0f 28 85 90 00 00 00     	movaps	0x90(%rbp), %xmm8
1408198cb: 44 0f 28 8d a0 00 00 00     	movaps	0xa0(%rbp), %xmm9
1408198d3: 44 0f 28 95 b0 00 00 00     	movaps	0xb0(%rbp), %xmm10
1408198db: 44 0f 28 9d c0 00 00 00     	movaps	0xc0(%rbp), %xmm11
1408198e3: 44 0f 28 a5 d0 00 00 00     	movaps	0xd0(%rbp), %xmm12
1408198eb: 44 0f 28 ad e0 00 00 00     	movaps	0xe0(%rbp), %xmm13
1408198f3: 48 81 c4 78 01 00 00        	addq	$0x178, %rsp            # imm = 0x178
1408198fa: 5b                          	popq	%rbx
1408198fb: 5f                          	popq	%rdi
1408198fc: 5e                          	popq	%rsi
1408198fd: 41 5c                       	popq	%r12
1408198ff: 41 5d                       	popq	%r13
140819901: 41 5e                       	popq	%r14
140819903: 41 5f                       	popq	%r15
140819905: 5d                          	popq	%rbp
140819906: c3                          	retq
140819907: 48 8d 0d 72 68 27 02        	leaq	0x2276872(%rip), %rcx   # 0x142a90180
14081990e: e8 fd f9 0b 02              	callq	0x1428d9310 <.text+0x28d8310>
140819913: 0f 0b                       	ud2
140819915: 48 85 c0                    	testq	%rax, %rax
140819918: 75 2d                       	jne	0x140819947 <.text+0x818947>
14081991a: 31 d2                       	xorl	%edx, %edx
14081991c: e8 5f 8c 0d 02              	callq	0x1428f2580 <.text+0x28f1580>
140819921: 48 89 c1                    	movq	%rax, %rcx
140819924: 48 85 c0                    	testq	%rax, %rax
140819927: 0f 85 fd f6 ff ff           	jne	0x14081902a <.text+0x81802a>
14081992d: eb 18                       	jmp	0x140819947 <.text+0x818947>
14081992f: 48 85 c0                    	testq	%rax, %rax
140819932: 75 13                       	jne	0x140819947 <.text+0x818947>
140819934: 31 d2                       	xorl	%edx, %edx
140819936: e8 45 8c 0d 02              	callq	0x1428f2580 <.text+0x28f1580>
14081993b: 48 89 c1                    	movq	%rax, %rcx
14081993e: 48 85 c0                    	testq	%rax, %rax
140819941: 0f 85 49 ff ff ff           	jne	0x140819890 <.text+0x818890>
140819947: 48 8d 0d 7a 7a 40 02        	leaq	0x2407a7a(%rip), %rcx   # 0x142c213c8
14081994e: e8 7d e7 0b 02              	callq	0x1428d80d0 <.text+0x28d70d0>
140819953: 48 8d 0d de 7a 40 02        	leaq	0x2407ade(%rip), %rcx   # 0x142c21438
14081995a: e8 f1 f8 0b 02              	callq	0x1428d9250 <.text+0x28d8250>
14081995f: cc                          	int3
140819960: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140819965: 55                          	pushq	%rbp
140819966: 41 57                       	pushq	%r15
140819968: 41 56                       	pushq	%r14
14081996a: 41 55                       	pushq	%r13
14081996c: 41 54                       	pushq	%r12
14081996e: 56                          	pushq	%rsi
14081996f: 57                          	pushq	%rdi
140819970: 53                          	pushq	%rbx
140819971: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
140819978: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14081997f: 44 0f 29 6c 24 30           	movaps	%xmm13, 0x30(%rsp)
140819985: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
14081998b: 66 44 0f 7f 5c 24 50        	movdqa	%xmm11, 0x50(%rsp)
140819992: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
140819998: 44 0f 29 4c 24 70           	movaps	%xmm9, 0x70(%rsp)
14081999e: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
1408199a7: 0f 29 bc 24 90 00 00 00     	movaps	%xmm7, 0x90(%rsp)
1408199af: 0f 29 b4 24 a0 00 00 00     	movaps	%xmm6, 0xa0(%rsp)
1408199b7: 48 8b 45 58                 	movq	0x58(%rbp), %rax
1408199bb: 48 ff 00                    	incq	(%rax)
1408199be: 0f 28 b4 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm6
1408199c6: 0f 28 bc 24 90 00 00 00     	movaps	0x90(%rsp), %xmm7
1408199ce: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
1408199d7: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
1408199dd: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
1408199e3: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
1408199e9: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
1408199ef: 44 0f 28 6c 24 30           	movaps	0x30(%rsp), %xmm13
1408199f5: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
1408199fc: 5b                          	popq	%rbx
1408199fd: 5f                          	popq	%rdi
1408199fe: 5e                          	popq	%rsi
1408199ff: 41 5c                       	popq	%r12
140819a01: 41 5d                       	popq	%r13
140819a03: 41 5e                       	popq	%r14
140819a05: 41 5f                       	popq	%r15
140819a07: 5d                          	popq	%rbp
140819a08: c3                          	retq
140819a09: 0f 1f 80 00 00 00 00        	nopl	(%rax)
140819a10: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140819a15: 55                          	pushq	%rbp
140819a16: 41 57                       	pushq	%r15
140819a18: 41 56                       	pushq	%r14
140819a1a: 41 55                       	pushq	%r13
140819a1c: 41 54                       	pushq	%r12
140819a1e: 56                          	pushq	%rsi
140819a1f: 57                          	pushq	%rdi
140819a20: 53                          	pushq	%rbx
140819a21: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
140819a28: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140819a2f: 44 0f 29 6c 24 30           	movaps	%xmm13, 0x30(%rsp)
140819a35: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
140819a3b: 66 44 0f 7f 5c 24 50        	movdqa	%xmm11, 0x50(%rsp)
140819a42: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
140819a48: 44 0f 29 4c 24 70           	movaps	%xmm9, 0x70(%rsp)
140819a4e: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
140819a57: 0f 29 bc 24 90 00 00 00     	movaps	%xmm7, 0x90(%rsp)
140819a5f: 66 0f 7f b4 24 a0 00 00 00  	movdqa	%xmm6, 0xa0(%rsp)
140819a68: 48 8b 45 58                 	movq	0x58(%rbp), %rax
140819a6c: 48 ff 00                    	incq	(%rax)
140819a6f: 0f 28 b4 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm6
140819a77: 0f 28 bc 24 90 00 00 00     	movaps	0x90(%rsp), %xmm7
140819a7f: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
140819a88: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
140819a8e: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
140819a94: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
140819a9a: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
140819aa0: 44 0f 28 6c 24 30           	movaps	0x30(%rsp), %xmm13
140819aa6: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
140819aad: 5b                          	popq	%rbx
140819aae: 5f                          	popq	%rdi
140819aaf: 5e                          	popq	%rsi
140819ab0: 41 5c                       	popq	%r12
140819ab2: 41 5d                       	popq	%r13
140819ab4: 41 5e                       	popq	%r14
140819ab6: 41 5f                       	popq	%r15
140819ab8: 5d                          	popq	%rbp
140819ab9: c3                          	retq
140819aba: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140819ac0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140819ac5: 55                          	pushq	%rbp
140819ac6: 41 57                       	pushq	%r15
140819ac8: 41 56                       	pushq	%r14
140819aca: 41 55                       	pushq	%r13
140819acc: 41 54                       	pushq	%r12
140819ace: 56                          	pushq	%rsi
140819acf: 57                          	pushq	%rdi
140819ad0: 53                          	pushq	%rbx
140819ad1: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
140819ad8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140819adf: 44 0f 29 6c 24 30           	movaps	%xmm13, 0x30(%rsp)
140819ae5: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
140819aeb: 66 44 0f 7f 5c 24 50        	movdqa	%xmm11, 0x50(%rsp)
140819af2: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
140819af8: 44 0f 29 4c 24 70           	movaps	%xmm9, 0x70(%rsp)
140819afe: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
140819b07: 66 0f 7f bc 24 90 00 00 00  	movdqa	%xmm7, 0x90(%rsp)
140819b10: 66 0f 7f b4 24 a0 00 00 00  	movdqa	%xmm6, 0xa0(%rsp)
140819b19: 48 8b 55 f8                 	movq	-0x8(%rbp), %rdx
140819b1d: 48 85 d2                    	testq	%rdx, %rdx
140819b20: 74 13                       	je	0x140819b35 <.text+0x818b35>
140819b22: 48 8b 4d 00                 	movq	(%rbp), %rcx
140819b26: 48 c1 e2 03                 	shlq	$0x3, %rdx
140819b2a: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140819b30: e8 eb a0 df ff              	callq	0x140613c20 <.text+0x612c20>
140819b35: 80 7d 67 00                 	cmpb	$0x0, 0x67(%rbp)
140819b39: 48 8b 7d d0                 	movq	-0x30(%rbp), %rdi
140819b3d: 74 35                       	je	0x140819b74 <.text+0x818b74>
140819b3f: e8 ec e5 3f 00              	callq	0x140c18130 <.text+0xc17130>
140819b44: 48 89 c6                    	movq	%rax, %rsi
140819b47: 48 8b 00                    	movq	(%rax), %rax
140819b4a: 48 83 f8 01                 	cmpq	$0x1, %rax
140819b4e: 75 6f                       	jne	0x140819bbf <.text+0x818bbf>
140819b50: 48 83 c6 08                 	addq	$0x8, %rsi
140819b54: 48 83 3e 00                 	cmpq	$0x0, (%rsi)
140819b58: 0f 85 88 00 00 00           	jne	0x140819be6 <.text+0x818be6>
140819b5e: 48 c7 06 ff ff ff ff        	movq	$-0x1, (%rsi)
140819b65: 48 8d 4e 08                 	leaq	0x8(%rsi), %rcx
140819b69: 48 89 fa                    	movq	%rdi, %rdx
140819b6c: e8 7f d5 3f 00              	callq	0x140c170f0 <.text+0xc160f0>
140819b71: 48 ff 06                    	incq	(%rsi)
140819b74: 0f 28 b4 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm6
140819b7c: 0f 28 bc 24 90 00 00 00     	movaps	0x90(%rsp), %xmm7
140819b84: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
140819b8d: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
140819b93: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
140819b99: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
140819b9f: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
140819ba5: 44 0f 28 6c 24 30           	movaps	0x30(%rsp), %xmm13
140819bab: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
140819bb2: 5b                          	popq	%rbx
140819bb3: 5f                          	popq	%rdi
140819bb4: 5e                          	popq	%rsi
140819bb5: 41 5c                       	popq	%r12
140819bb7: 41 5d                       	popq	%r13
140819bb9: 41 5e                       	popq	%r14
140819bbb: 41 5f                       	popq	%r15
140819bbd: 5d                          	popq	%rbp
140819bbe: c3                          	retq
140819bbf: 48 85 c0                    	testq	%rax, %rax
140819bc2: 75 16                       	jne	0x140819bda <.text+0x818bda>
140819bc4: 48 89 f1                    	movq	%rsi, %rcx
140819bc7: 31 d2                       	xorl	%edx, %edx
140819bc9: e8 b2 89 0d 02              	callq	0x1428f2580 <.text+0x28f1580>
140819bce: 48 89 c6                    	movq	%rax, %rsi
140819bd1: 48 85 c0                    	testq	%rax, %rax
140819bd4: 0f 85 7a ff ff ff           	jne	0x140819b54 <.text+0x818b54>
140819bda: 48 8d 0d e7 77 40 02        	leaq	0x24077e7(%rip), %rcx   # 0x142c213c8
140819be1: e8 ea e4 0b 02              	callq	0x1428d80d0 <.text+0x28d70d0>
140819be6: 48 8d 0d 4b 78 40 02        	leaq	0x240784b(%rip), %rcx   # 0x142c21438
140819bed: e8 5e f6 0b 02              	callq	0x1428d9250 <.text+0x28d8250>
140819bf2: cc                          	int3
