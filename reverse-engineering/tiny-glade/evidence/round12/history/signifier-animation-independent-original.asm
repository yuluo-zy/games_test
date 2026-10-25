
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a69340: 55                          	pushq	%rbp
140a69341: 41 57                       	pushq	%r15
140a69343: 41 56                       	pushq	%r14
140a69345: 41 55                       	pushq	%r13
140a69347: 41 54                       	pushq	%r12
140a69349: 56                          	pushq	%rsi
140a6934a: 57                          	pushq	%rdi
140a6934b: 53                          	pushq	%rbx
140a6934c: 48 81 ec 18 01 00 00        	subq	$0x118, %rsp            # imm = 0x118
140a69353: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140a6935b: 44 0f 29 85 80 00 00 00     	movaps	%xmm8, 0x80(%rbp)
140a69363: 0f 29 7d 70                 	movaps	%xmm7, 0x70(%rbp)
140a69367: 0f 29 75 60                 	movaps	%xmm6, 0x60(%rbp)
140a6936b: 48 c7 45 58 fe ff ff ff     	movq	$-0x2, 0x58(%rbp)
140a69373: 48 89 d6                    	movq	%rdx, %rsi
140a69376: 48 89 cf                    	movq	%rcx, %rdi
140a69379: e8 82 d6 1a 00              	callq	0x140c16a00 <.text+0xc15a00>
140a6937e: 84 c0                       	testb	%al, %al
140a69380: 74 5f                       	je	0x140a693e1 <.text+0xa683e1>
140a69382: 4c 8d 35 97 b7 06 02        	leaq	0x206b797(%rip), %r14   # 0x142ad4b20
140a69389: bb 47 00 00 00              	movl	$0x47, %ebx
140a6938e: ba 47 00 00 00              	movl	$0x47, %edx
140a69393: 4c 89 f1                    	movq	%r14, %rcx
140a69396: e8 e5 df ff ff              	callq	0x140a67380 <.text+0xa66380>
140a6939b: 48 83 f8 01                 	cmpq	$0x1, %rax
140a6939f: 0f 85 b1 00 00 00           	jne	0x140a69456 <.text+0xa68456>
140a693a5: 49 89 d1                    	movq	%rdx, %r9
140a693a8: 48 85 d2                    	testq	%rdx, %rdx
140a693ab: 74 3d                       	je	0x140a693ea <.text+0xa683ea>
140a693ad: 49 83 f9 47                 	cmpq	$0x47, %r9
140a693b1: 73 35                       	jae	0x140a693e8 <.text+0xa683e8>
140a693b3: 48 8d 05 66 b7 06 02        	leaq	0x206b766(%rip), %rax   # 0x142ad4b20
140a693ba: 41 80 3c 01 bf              	cmpb	$-0x41, (%r9,%rax)
140a693bf: 7f 29                       	jg	0x140a693ea <.text+0xa683ea>
140a693c1: 48 8d 05 b0 b8 06 02        	leaq	0x206b8b0(%rip), %rax   # 0x142ad4c78
140a693c8: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140a693cd: 48 8d 0d 4c b7 06 02        	leaq	0x206b74c(%rip), %rcx   # 0x142ad4b20
140a693d4: ba 47 00 00 00              	movl	$0x47, %edx
140a693d9: 45 31 c0                    	xorl	%r8d, %r8d
140a693dc: e8 5f 05 e7 01              	callq	0x1428d9940 <.text+0x28d8940>
140a693e1: 31 c0                       	xorl	%eax, %eax
140a693e3: e9 ce 01 00 00              	jmp	0x140a695b6 <.text+0xa685b6>
140a693e8: 75 d7                       	jne	0x140a693c1 <.text+0xa683c1>
140a693ea: 4c 8d 35 2f b7 06 02        	leaq	0x206b72f(%rip), %r14   # 0x142ad4b20
140a693f1: 4c 89 f1                    	movq	%r14, %rcx
140a693f4: 4c 89 ca                    	movq	%r9, %rdx
140a693f7: e8 84 df ff ff              	callq	0x140a67380 <.text+0xa66380>
140a693fc: 48 83 f8 01                 	cmpq	$0x1, %rax
140a69400: 75 54                       	jne	0x140a69456 <.text+0xa68456>
140a69402: 49 89 d0                    	movq	%rdx, %r8
140a69405: 49 83 c0 02                 	addq	$0x2, %r8
140a69409: 74 39                       	je	0x140a69444 <.text+0xa68444>
140a6940b: 49 83 f8 47                 	cmpq	$0x47, %r8
140a6940f: 73 31                       	jae	0x140a69442 <.text+0xa68442>
140a69411: 48 8d 05 08 b7 06 02        	leaq	0x206b708(%rip), %rax   # 0x142ad4b20
140a69418: 41 80 3c 00 bf              	cmpb	$-0x41, (%r8,%rax)
140a6941d: 7f 25                       	jg	0x140a69444 <.text+0xa68444>
140a6941f: 48 8d 05 6a b8 06 02        	leaq	0x206b86a(%rip), %rax   # 0x142ad4c90
140a69426: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140a6942b: 48 8d 0d ee b6 06 02        	leaq	0x206b6ee(%rip), %rcx   # 0x142ad4b20
140a69432: ba 47 00 00 00              	movl	$0x47, %edx
140a69437: 41 b9 47 00 00 00           	movl	$0x47, %r9d
140a6943d: e8 fe 04 e7 01              	callq	0x1428d9940 <.text+0x28d8940>
140a69442: 75 db                       	jne	0x140a6941f <.text+0xa6841f>
140a69444: bb 45 00 00 00              	movl	$0x45, %ebx
140a69449: 48 29 d3                    	subq	%rdx, %rbx
140a6944c: 4c 8d 35 cd b6 06 02        	leaq	0x206b6cd(%rip), %r14   # 0x142ad4b20
140a69453: 4d 01 c6                    	addq	%r8, %r14
140a69456: 48 8d 05 81 bd 06 02        	leaq	0x206bd81(%rip), %rax   # 0x142ad51de
140a6945d: 4c 8d 3d 40 bd 06 02        	leaq	0x206bd40(%rip), %r15   # 0x142ad51a4
140a69464: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
140a69470: 0f be 48 ff                 	movsbl	-0x1(%rax), %ecx
140a69474: 85 c9                       	testl	%ecx, %ecx
140a69476: 78 18                       	js	0x140a69490 <.text+0xa68490>
140a69478: 48 ff c8                    	decq	%rax
140a6947b: 83 f9 5c                    	cmpl	$0x5c, %ecx
140a6947e: 75 69                       	jne	0x140a694e9 <.text+0xa684e9>
140a69480: eb 7d                       	jmp	0x140a694ff <.text+0xa684ff>
140a69482: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
140a69490: 0f b6 50 fe                 	movzbl	-0x2(%rax), %edx
140a69494: 80 fa c0                    	cmpb	$-0x40, %dl
140a69497: 7d 25                       	jge	0x140a694be <.text+0xa684be>
140a69499: 44 0f b6 40 fd              	movzbl	-0x3(%rax), %r8d
140a6949e: 41 80 f8 c0                 	cmpb	$-0x40, %r8b
140a694a2: 7d 23                       	jge	0x140a694c7 <.text+0xa684c7>
140a694a4: 44 0f b6 48 fc              	movzbl	-0x4(%rax), %r9d
140a694a9: 48 83 c0 fc                 	addq	$-0x4, %rax
140a694ad: 41 83 e1 07                 	andl	$0x7, %r9d
140a694b1: 41 c1 e1 06                 	shll	$0x6, %r9d
140a694b5: 41 83 e0 3f                 	andl	$0x3f, %r8d
140a694b9: 45 09 c8                    	orl	%r9d, %r8d
140a694bc: eb 11                       	jmp	0x140a694cf <.text+0xa684cf>
140a694be: 48 83 c0 fe                 	addq	$-0x2, %rax
140a694c2: 83 e2 1f                    	andl	$0x1f, %edx
140a694c5: eb 12                       	jmp	0x140a694d9 <.text+0xa684d9>
140a694c7: 48 83 c0 fd                 	addq	$-0x3, %rax
140a694cb: 41 83 e0 0f                 	andl	$0xf, %r8d
140a694cf: 41 c1 e0 06                 	shll	$0x6, %r8d
140a694d3: 83 e2 3f                    	andl	$0x3f, %edx
140a694d6: 44 09 c2                    	orl	%r8d, %edx
140a694d9: c1 e2 06                    	shll	$0x6, %edx
140a694dc: 80 e1 3f                    	andb	$0x3f, %cl
140a694df: 0f b6 c9                    	movzbl	%cl, %ecx
140a694e2: 09 d1                       	orl	%edx, %ecx
140a694e4: 83 f9 5c                    	cmpl	$0x5c, %ecx
140a694e7: 74 16                       	je	0x140a694ff <.text+0xa684ff>
140a694e9: 83 f9 2f                    	cmpl	$0x2f, %ecx
140a694ec: 74 11                       	je	0x140a694ff <.text+0xa684ff>
140a694ee: 4c 39 f8                    	cmpq	%r15, %rax
140a694f1: 0f 85 79 ff ff ff           	jne	0x140a69470 <.text+0xa68470>
140a694f7: 41 bc 3a 00 00 00           	movl	$0x3a, %r12d
140a694fd: eb 50                       	jmp	0x140a6954f <.text+0xa6854f>
140a694ff: 4c 8d 3d 9e bc 06 02        	leaq	0x206bc9e(%rip), %r15   # 0x142ad51a4
140a69506: 4c 29 f8                    	subq	%r15, %rax
140a69509: 49 89 c0                    	movq	%rax, %r8
140a6950c: 49 ff c0                    	incq	%r8
140a6950f: 74 32                       	je	0x140a69543 <.text+0xa68543>
140a69511: 49 83 f8 3a                 	cmpq	$0x3a, %r8
140a69515: 73 2a                       	jae	0x140a69541 <.text+0xa68541>
140a69517: 43 80 3c 38 bf              	cmpb	$-0x41, (%r8,%r15)
140a6951c: 7f 25                       	jg	0x140a69543 <.text+0xa68543>
140a6951e: 48 8d 05 33 b7 06 02        	leaq	0x206b733(%rip), %rax   # 0x142ad4c58
140a69525: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140a6952a: 48 8d 0d 73 bc 06 02        	leaq	0x206bc73(%rip), %rcx   # 0x142ad51a4
140a69531: ba 3a 00 00 00              	movl	$0x3a, %edx
140a69536: 41 b9 3a 00 00 00           	movl	$0x3a, %r9d
140a6953c: e8 ff 03 e7 01              	callq	0x1428d9940 <.text+0x28d8940>
140a69541: 75 db                       	jne	0x140a6951e <.text+0xa6851e>
140a69543: 41 bc 39 00 00 00           	movl	$0x39, %r12d
140a69549: 49 29 c4                    	subq	%rax, %r12
140a6954c: 4d 01 c7                    	addq	%r8, %r15
140a6954f: e8 dc eb 1a 00              	callq	0x140c18130 <.text+0xc17130>
140a69554: 48 89 c1                    	movq	%rax, %rcx
140a69557: 48 8b 00                    	movq	(%rax), %rax
140a6955a: 48 83 f8 01                 	cmpq	$0x1, %rax
140a6955e: 0f 85 56 05 00 00           	jne	0x140a69aba <.text+0xa68aba>
140a69564: 48 83 c1 08                 	addq	$0x8, %rcx
140a69568: 48 83 39 00                 	cmpq	$0x0, (%rcx)
140a6956c: 0f 85 6c 05 00 00           	jne	0x140a69ade <.text+0xa68ade>
140a69572: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
140a69579: 48 89 4d 40                 	movq	%rcx, 0x40(%rbp)
140a6957d: 48 83 c1 08                 	addq	$0x8, %rcx
140a69581: 4c 89 64 24 20              	movq	%r12, 0x20(%rsp)
140a69586: 48 c7 44 24 30 00 00 00 00  	movq	$0x0, 0x30(%rsp)
140a6958f: 48 c7 44 24 28 01 00 00 00  	movq	$0x1, 0x28(%rsp)
140a69598: 4c 89 f2                    	movq	%r14, %rdx
140a6959b: 49 89 d8                    	movq	%rbx, %r8
140a6959e: 4d 89 f9                    	movq	%r15, %r9
140a695a1: e8 9a d7 1a 00              	callq	0x140c16d40 <.text+0xc15d40>
140a695a6: 48 8b 4d 40                 	movq	0x40(%rbp), %rcx
140a695aa: 48 ff 01                    	incq	(%rcx)
140a695ad: 48 89 45 e8                 	movq	%rax, -0x18(%rbp)
140a695b1: b8 01 00 00 00              	movl	$0x1, %eax
140a695b6: 48 89 45 e0                 	movq	%rax, -0x20(%rbp)
140a695ba: 48 8b 0f                    	movq	(%rdi), %rcx
140a695bd: 4c 8b 77 08                 	movq	0x8(%rdi), %r14
140a695c1: 8b 47 14                    	movl	0x14(%rdi), %eax
140a695c4: 89 45 3c                    	movl	%eax, 0x3c(%rbp)
140a695c7: 4c 8b b9 f8 00 00 00        	movq	0xf8(%rcx), %r15
140a695ce: 48 8b 81 00 01 00 00        	movq	0x100(%rcx), %rax
140a695d5: 4d 8d 24 87                 	leaq	(%r15,%rax,4), %r12
140a695d9: 48 89 4d 28                 	movq	%rcx, 0x28(%rbp)
140a695dd: 0f b6 81 08 01 00 00        	movzbl	0x108(%rcx), %eax
140a695e4: 88 45 4f                    	movb	%al, 0x4f(%rbp)
140a695e7: 48 8b 06                    	movq	(%rsi), %rax
140a695ea: 48 89 45 f0                 	movq	%rax, -0x10(%rbp)
140a695ee: b8 08 00 00 00              	movl	$0x8, %eax
140a695f3: 48 89 45 f8                 	movq	%rax, -0x8(%rbp)
140a695f7: 48 c7 45 10 00 00 00 00     	movq	$0x0, 0x10(%rbp)
140a695ff: 0f 57 f6                    	xorps	%xmm6, %xmm6
140a69602: f3 0f 10 3d fa c2 eb 01     	movss	0x1ebc2fa(%rip), %xmm7  # 0x142925904
140a6960a: 44 0f 28 05 de c2 eb 01     	movaps	0x1ebc2de(%rip), %xmm8  # 0x1429258f0
140a69612: c7 45 50 00 00 00 00        	movl	$0x0, 0x50(%rbp)
140a69619: 48 c7 45 30 00 00 00 00     	movq	$0x0, 0x30(%rbp)
140a69621: 31 ff                       	xorl	%edi, %edi
140a69623: 45 31 ed                    	xorl	%r13d, %r13d
140a69626: 48 c7 45 20 00 00 00 00     	movq	$0x0, 0x20(%rbp)
140a6962e: c7 45 54 00 00 00 00        	movl	$0x0, 0x54(%rbp)
140a69635: 80 7d 4f 00                 	cmpb	$0x0, 0x4f(%rbp)
140a69639: 0f 84 91 00 00 00           	je	0x140a696d0 <.text+0xa686d0>
140a6963f: 4c 39 ef                    	cmpq	%r13, %rdi
140a69642: 0f 85 58 01 00 00           	jne	0x140a697a0 <.text+0xa687a0>
140a69648: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
140a69650: 4d 39 e7                    	cmpq	%r12, %r15
140a69653: 0f 84 e6 03 00 00           	je	0x140a69a3f <.text+0xa68a3f>
140a69659: 41 8b 0f                    	movl	(%r15), %ecx
140a6965c: 49 83 c7 04                 	addq	$0x4, %r15
140a69660: 49 8b 86 a8 01 00 00        	movq	0x1a8(%r14), %rax
140a69667: 48 8d 0c c9                 	leaq	(%rcx,%rcx,8), %rcx
140a6966b: 4c 8b 6c c8 10              	movq	0x10(%rax,%rcx,8), %r13
140a69670: 4d 85 ed                    	testq	%r13, %r13
140a69673: 74 db                       	je	0x140a69650 <.text+0xa68650>
140a69675: 48 8d 0c c8                 	leaq	(%rax,%rcx,8), %rcx
140a69679: 48 8b 51 40                 	movq	0x40(%rcx), %rdx
140a6967d: 48 8b 41 18                 	movq	0x18(%rcx), %rax
140a69681: 48 8b 49 38                 	movq	0x38(%rcx), %rcx
140a69685: 4c 8b 45 28                 	movq	0x28(%rbp), %r8
140a69689: 4d 8b 80 18 01 00 00        	movq	0x118(%r8), %r8
140a69690: 4c 39 c2                    	cmpq	%r8, %rdx
140a69693: 0f 86 1f 01 00 00           	jbe	0x140a697b8 <.text+0xa687b8>
140a69699: 4e 8b 04 c1                 	movq	(%rcx,%r8,8), %r8
140a6969d: 4d 85 c0                    	testq	%r8, %r8
140a696a0: 0f 84 12 01 00 00           	je	0x140a697b8 <.text+0xa687b8>
140a696a6: 49 f7 d0                    	notq	%r8
140a696a9: 4f 8d 04 40                 	leaq	(%r8,%r8,2), %r8
140a696ad: 49 c1 e0 04                 	shlq	$0x4, %r8
140a696b1: 4e 8b 44 00 10              	movq	0x10(%rax,%r8), %r8
140a696b6: 4c 89 45 30                 	movq	%r8, 0x30(%rbp)
140a696ba: 41 b0 01                    	movb	$0x1, %r8b
140a696bd: 44 89 45 50                 	movl	%r8d, 0x50(%rbp)
140a696c1: e9 f9 00 00 00              	jmp	0x140a697bf <.text+0xa687bf>
140a696c6: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
140a696d0: 4c 39 ef                    	cmpq	%r13, %rdi
140a696d3: 0f 85 d7 00 00 00           	jne	0x140a697b0 <.text+0xa687b0>
140a696d9: 0f 1f 80 00 00 00 00        	nopl	(%rax)
140a696e0: 4d 39 e7                    	cmpq	%r12, %r15
140a696e3: 0f 84 56 03 00 00           	je	0x140a69a3f <.text+0xa68a3f>
140a696e9: 41 8b 0f                    	movl	(%r15), %ecx
140a696ec: 49 83 c7 04                 	addq	$0x4, %r15
140a696f0: 49 8b 86 00 01 00 00        	movq	0x100(%r14), %rax
140a696f7: 48 8d 0c 89                 	leaq	(%rcx,%rcx,4), %rcx
140a696fb: 48 c1 e1 05                 	shlq	$0x5, %rcx
140a696ff: 4c 8b 6c 08 58              	movq	0x58(%rax,%rcx), %r13
140a69704: 4d 85 ed                    	testq	%r13, %r13
140a69707: 74 d7                       	je	0x140a696e0 <.text+0xa686e0>
140a69709: 48 01 c8                    	addq	%rcx, %rax
140a6970c: 8b 88 94 00 00 00           	movl	0x94(%rax), %ecx
140a69712: 4d 8b 8e a8 01 00 00        	movq	0x1a8(%r14), %r9
140a69719: 4c 8d 1c c9                 	leaq	(%rcx,%rcx,8), %r11
140a6971d: 4b 8b 4c d9 18              	movq	0x18(%r9,%r11,8), %rcx
140a69722: 4b 8b 54 d9 38              	movq	0x38(%r9,%r11,8), %rdx
140a69727: 4c 8b 45 28                 	movq	0x28(%rbp), %r8
140a6972b: 4d 8b 90 18 01 00 00        	movq	0x118(%r8), %r10
140a69732: 4c 8b 80 88 00 00 00        	movq	0x88(%rax), %r8
140a69739: 4d 39 d0                    	cmpq	%r10, %r8
140a6973c: 0f 86 e4 00 00 00           	jbe	0x140a69826 <.text+0xa68826>
140a69742: 48 8b b0 80 00 00 00        	movq	0x80(%rax), %rsi
140a69749: 4a 83 3c d6 00              	cmpq	$0x0, (%rsi,%r10,8)
140a6974e: 0f 84 d2 00 00 00           	je	0x140a69826 <.text+0xa68826>
140a69754: 40 b6 01                    	movb	$0x1, %sil
140a69757: 89 75 50                    	movl	%esi, 0x50(%rbp)
140a6975a: 4f 39 54 d9 40              	cmpq	%r10, 0x40(%r9,%r11,8)
140a6975f: 0f 86 45 01 00 00           	jbe	0x140a698aa <.text+0xa688aa>
140a69765: 4e 8b 0c d2                 	movq	(%rdx,%r10,8), %r9
140a69769: 4d 85 c9                    	testq	%r9, %r9
140a6976c: 0f 84 38 01 00 00           	je	0x140a698aa <.text+0xa688aa>
140a69772: 49 f7 d1                    	notq	%r9
140a69775: 4f 8d 0c 49                 	leaq	(%r9,%r9,2), %r9
140a69779: 49 c1 e1 04                 	shlq	$0x4, %r9
140a6977d: 4e 8b 4c 09 10              	movq	0x10(%rcx,%r9), %r9
140a69782: 4c 89 4d 30                 	movq	%r9, 0x30(%rbp)
140a69786: 4c 8b 4d 28                 	movq	0x28(%rbp), %r9
140a6978a: 4d 8b 89 20 01 00 00        	movq	0x120(%r9), %r9
140a69791: 4d 39 c8                    	cmpq	%r9, %r8
140a69794: 0f 87 28 01 00 00           	ja	0x140a698c2 <.text+0xa688c2>
140a6979a: e9 5b 01 00 00              	jmp	0x140a698fa <.text+0xa688fa>
140a6979f: 90                          	nop
140a697a0: 48 89 f8                    	movq	%rdi, %rax
140a697a3: e9 a7 00 00 00              	jmp	0x140a6984f <.text+0xa6884f>
140a697a8: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
140a697b0: 48 89 f8                    	movq	%rdi, %rax
140a697b3: e9 7f 01 00 00              	jmp	0x140a69937 <.text+0xa68937>
140a697b8: c7 45 50 00 00 00 00        	movl	$0x0, 0x50(%rbp)
140a697bf: 4c 8b 4d 28                 	movq	0x28(%rbp), %r9
140a697c3: 4d 8b 81 10 01 00 00        	movq	0x110(%r9), %r8
140a697ca: 4e 8b 04 c1                 	movq	(%rcx,%r8,8), %r8
140a697ce: 49 f7 d0                    	notq	%r8
140a697d1: 4f 8d 04 40                 	leaq	(%r8,%r8,2), %r8
140a697d5: 49 c1 e0 04                 	shlq	$0x4, %r8
140a697d9: 4e 8b 54 00 10              	movq	0x10(%rax,%r8), %r10
140a697de: 4c 89 55 10                 	movq	%r10, 0x10(%rbp)
140a697e2: 4e 8b 44 00 28              	movq	0x28(%rax,%r8), %r8
140a697e7: 4c 89 45 00                 	movq	%r8, (%rbp)
140a697eb: 4d 8b 81 20 01 00 00        	movq	0x120(%r9), %r8
140a697f2: 4c 39 c2                    	cmpq	%r8, %rdx
140a697f5: 76 4f                       	jbe	0x140a69846 <.text+0xa68846>
140a697f7: 4a 8b 0c c1                 	movq	(%rcx,%r8,8), %rcx
140a697fb: 48 85 c9                    	testq	%rcx, %rcx
140a697fe: 74 46                       	je	0x140a69846 <.text+0xa68846>
140a69800: 48 f7 d1                    	notq	%rcx
140a69803: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
140a69807: 48 c1 e1 04                 	shlq	$0x4, %rcx
140a6980b: 48 8b 54 08 10              	movq	0x10(%rax,%rcx), %rdx
140a69810: 48 89 55 20                 	movq	%rdx, 0x20(%rbp)
140a69814: 48 8b 44 08 28              	movq	0x28(%rax,%rcx), %rax
140a69819: 48 89 45 08                 	movq	%rax, 0x8(%rbp)
140a6981d: b0 01                       	movb	$0x1, %al
140a6981f: 89 45 54                    	movl	%eax, 0x54(%rbp)
140a69822: 31 c0                       	xorl	%eax, %eax
140a69824: eb 29                       	jmp	0x140a6984f <.text+0xa6884f>
140a69826: c7 45 50 00 00 00 00        	movl	$0x0, 0x50(%rbp)
140a6982d: 4c 8b 4d 28                 	movq	0x28(%rbp), %r9
140a69831: 4d 8b 89 20 01 00 00        	movq	0x120(%r9), %r9
140a69838: 4d 39 c8                    	cmpq	%r9, %r8
140a6983b: 0f 87 81 00 00 00           	ja	0x140a698c2 <.text+0xa688c2>
140a69841: e9 b4 00 00 00              	jmp	0x140a698fa <.text+0xa688fa>
140a69846: 31 c0                       	xorl	%eax, %eax
140a69848: c7 45 54 00 00 00 00        	movl	$0x0, 0x54(%rbp)
140a6984f: 48 89 c7                    	movq	%rax, %rdi
140a69852: 89 c1                       	movl	%eax, %ecx
140a69854: 48 8d 04 c9                 	leaq	(%rcx,%rcx,8), %rax
140a69858: 48 8d 34 40                 	leaq	(%rax,%rax,2), %rsi
140a6985c: 48 8b 45 10                 	movq	0x10(%rbp), %rax
140a69860: 48 01 c8                    	addq	%rcx, %rax
140a69863: 48 01 c6                    	addq	%rax, %rsi
140a69866: 48 8b 45 00                 	movq	(%rbp), %rax
140a6986a: 48 8d 04 88                 	leaq	(%rax,%rcx,4), %rax
140a6986e: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
140a69872: f6 45 50 01                 	testb	$0x1, 0x50(%rbp)
140a69876: b8 00 00 00 00              	movl	$0x0, %eax
140a6987b: 48 0f 45 45 30              	cmovneq	0x30(%rbp), %rax
140a69880: f6 45 54 01                 	testb	$0x1, 0x54(%rbp)
140a69884: 74 19                       	je	0x140a6989f <.text+0xa6889f>
140a69886: 48 8b 55 20                 	movq	0x20(%rbp), %rdx
140a6988a: 48 8d 1c ca                 	leaq	(%rdx,%rcx,8), %rbx
140a6988e: 48 8b 55 08                 	movq	0x8(%rbp), %rdx
140a69892: 48 8d 0c 8a                 	leaq	(%rdx,%rcx,4), %rcx
140a69896: 48 89 4d d0                 	movq	%rcx, -0x30(%rbp)
140a6989a: e9 f5 00 00 00              	jmp	0x140a69994 <.text+0xa68994>
140a6989f: 31 db                       	xorl	%ebx, %ebx
140a698a1: 48 8b 4d d0                 	movq	-0x30(%rbp), %rcx
140a698a5: e9 ea 00 00 00              	jmp	0x140a69994 <.text+0xa68994>
140a698aa: 48 c7 45 30 00 00 00 00     	movq	$0x0, 0x30(%rbp)
140a698b2: 4c 8b 4d 28                 	movq	0x28(%rbp), %r9
140a698b6: 4d 8b 89 20 01 00 00        	movq	0x120(%r9), %r9
140a698bd: 4d 39 c8                    	cmpq	%r9, %r8
140a698c0: 76 38                       	jbe	0x140a698fa <.text+0xa688fa>
140a698c2: 4c 8b 80 80 00 00 00        	movq	0x80(%rax), %r8
140a698c9: 4b 83 3c c8 00              	cmpq	$0x0, (%r8,%r9,8)
140a698ce: 74 2a                       	je	0x140a698fa <.text+0xa688fa>
140a698d0: 4e 8b 04 ca                 	movq	(%rdx,%r9,8), %r8
140a698d4: 49 f7 d0                    	notq	%r8
140a698d7: 4f 8d 04 40                 	leaq	(%r8,%r8,2), %r8
140a698db: 49 c1 e0 04                 	shlq	$0x4, %r8
140a698df: 4e 8b 4c 01 10              	movq	0x10(%rcx,%r8), %r9
140a698e4: 4c 89 4d 20                 	movq	%r9, 0x20(%rbp)
140a698e8: 4e 8b 44 01 28              	movq	0x28(%rcx,%r8), %r8
140a698ed: 4c 89 45 08                 	movq	%r8, 0x8(%rbp)
140a698f1: 41 b0 01                    	movb	$0x1, %r8b
140a698f4: 44 89 45 54                 	movl	%r8d, 0x54(%rbp)
140a698f8: eb 07                       	jmp	0x140a69901 <.text+0xa68901>
140a698fa: c7 45 54 00 00 00 00        	movl	$0x0, 0x54(%rbp)
140a69901: 4c 8b 45 28                 	movq	0x28(%rbp), %r8
140a69905: 4d 8b 80 10 01 00 00        	movq	0x110(%r8), %r8
140a6990c: 4a 8b 14 c2                 	movq	(%rdx,%r8,8), %rdx
140a69910: 48 f7 d2                    	notq	%rdx
140a69913: 48 8d 14 52                 	leaq	(%rdx,%rdx,2), %rdx
140a69917: 48 c1 e2 04                 	shlq	$0x4, %rdx
140a6991b: 4c 8b 44 11 10              	movq	0x10(%rcx,%rdx), %r8
140a69920: 4c 89 45 10                 	movq	%r8, 0x10(%rbp)
140a69924: 48 8b 4c 11 28              	movq	0x28(%rcx,%rdx), %rcx
140a69929: 48 89 4d 00                 	movq	%rcx, (%rbp)
140a6992d: 48 8b 40 50                 	movq	0x50(%rax), %rax
140a69931: 48 89 45 f8                 	movq	%rax, -0x8(%rbp)
140a69935: 31 c0                       	xorl	%eax, %eax
140a69937: 48 89 c7                    	movq	%rax, %rdi
140a6993a: 48 c1 e0 04                 	shlq	$0x4, %rax
140a6993e: 48 8b 4d f8                 	movq	-0x8(%rbp), %rcx
140a69942: 8b 4c 01 08                 	movl	0x8(%rcx,%rax), %ecx
140a69946: 48 8d 04 c9                 	leaq	(%rcx,%rcx,8), %rax
140a6994a: 48 8d 34 40                 	leaq	(%rax,%rax,2), %rsi
140a6994e: 48 8b 45 10                 	movq	0x10(%rbp), %rax
140a69952: 48 01 c8                    	addq	%rcx, %rax
140a69955: 48 01 c6                    	addq	%rax, %rsi
140a69958: 48 8b 45 00                 	movq	(%rbp), %rax
140a6995c: 48 8d 04 88                 	leaq	(%rax,%rcx,4), %rax
140a69960: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
140a69964: f6 45 50 01                 	testb	$0x1, 0x50(%rbp)
140a69968: b8 00 00 00 00              	movl	$0x0, %eax
140a6996d: 48 0f 45 45 30              	cmovneq	0x30(%rbp), %rax
140a69972: f6 45 54 01                 	testb	$0x1, 0x54(%rbp)
140a69976: 74 16                       	je	0x140a6998e <.text+0xa6898e>
140a69978: 48 8b 55 20                 	movq	0x20(%rbp), %rdx
140a6997c: 48 8d 1c ca                 	leaq	(%rdx,%rcx,8), %rbx
140a69980: 48 8b 55 08                 	movq	0x8(%rbp), %rdx
140a69984: 48 8d 0c 8a                 	leaq	(%rdx,%rcx,4), %rcx
140a69988: 48 89 4d d8                 	movq	%rcx, -0x28(%rbp)
140a6998c: eb 06                       	jmp	0x140a69994 <.text+0xa68994>
140a6998e: 31 db                       	xorl	%ebx, %ebx
140a69990: 48 8b 4d d8                 	movq	-0x28(%rbp), %rcx
140a69994: 48 89 4d 18                 	movq	%rcx, 0x18(%rbp)
140a69998: f3 0f 10 46 10              	movss	0x10(%rsi), %xmm0
140a6999d: 0f 2e c6                    	ucomiss	%xmm6, %xmm0
140a699a0: 75 1e                       	jne	0x140a699c0 <.text+0xa689c0>
140a699a2: 7a 1c                       	jp	0x140a699c0 <.text+0xa689c0>
140a699a4: 0f 28 cf                    	movaps	%xmm7, %xmm1
140a699a7: 48 85 c0                    	testq	%rax, %rax
140a699aa: 74 03                       	je	0x140a699af <.text+0xa689af>
140a699ac: 0f 57 c9                    	xorps	%xmm1, %xmm1
140a699af: 8b 45 3c                    	movl	0x3c(%rbp), %eax
140a699b2: 48 8b 4d 40                 	movq	0x40(%rbp), %rcx
140a699b6: 89 01                       	movl	%eax, (%rcx)
140a699b8: eb 3f                       	jmp	0x140a699f9 <.text+0xa689f9>
140a699ba: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140a699c0: 48 8b 4d f0                 	movq	-0x10(%rbp), %rcx
140a699c4: f3 0f 10 49 28              	movss	0x28(%rcx), %xmm1
140a699c9: 48 85 c0                    	testq	%rax, %rax
140a699cc: 74 04                       	je	0x140a699d2 <.text+0xa689d2>
140a699ce: 41 0f 57 c8                 	xorps	%xmm8, %xmm1
140a699d2: 0f 28 d7                    	movaps	%xmm7, %xmm2
140a699d5: f3 0f 5e d0                 	divss	%xmm0, %xmm2
140a699d9: f3 0f 59 d1                 	mulss	%xmm1, %xmm2
140a699dd: 8b 45 3c                    	movl	0x3c(%rbp), %eax
140a699e0: 48 8b 4d 40                 	movq	0x40(%rbp), %rcx
140a699e4: 89 01                       	movl	%eax, (%rcx)
140a699e6: f3 0f 58 56 14              	addss	0x14(%rsi), %xmm2
140a699eb: 0f 57 c0                    	xorps	%xmm0, %xmm0
140a699ee: f3 0f 5f c2                 	maxss	%xmm2, %xmm0
140a699f2: 0f 28 cf                    	movaps	%xmm7, %xmm1
140a699f5: f3 0f 5d c8                 	minss	%xmm0, %xmm1
140a699f9: f3 0f 11 4e 14              	movss	%xmm1, 0x14(%rsi)
140a699fe: 48 89 f1                    	movq	%rsi, %rcx
140a69a01: e8 ea c4 22 00              	callq	0x140c95ef0 <.text+0xc94ef0>
140a69a06: 48 ff c7                    	incq	%rdi
140a69a09: 8b 45 3c                    	movl	0x3c(%rbp), %eax
140a69a0c: 48 8b 4d 40                 	movq	0x40(%rbp), %rcx
140a69a10: 89 01                       	movl	%eax, (%rcx)
140a69a12: f3 0f 11 46 18              	movss	%xmm0, 0x18(%rsi)
140a69a17: 48 85 db                    	testq	%rbx, %rbx
140a69a1a: 0f 84 15 fc ff ff           	je	0x140a69635 <.text+0xa68635>
140a69a20: 48 8b 45 f0                 	movq	-0x10(%rbp), %rax
140a69a24: f3 0f 10 40 28              	movss	0x28(%rax), %xmm0
140a69a29: 8b 45 3c                    	movl	0x3c(%rbp), %eax
140a69a2c: 48 8b 4d 18                 	movq	0x18(%rbp), %rcx
140a69a30: 89 01                       	movl	%eax, (%rcx)
140a69a32: f3 0f 58 03                 	addss	(%rbx), %xmm0
140a69a36: f3 0f 11 03                 	movss	%xmm0, (%rbx)
140a69a3a: e9 f6 fb ff ff              	jmp	0x140a69635 <.text+0xa68635>
140a69a3f: 48 83 7d e0 00              	cmpq	$0x0, -0x20(%rbp)
140a69a44: 74 3a                       	je	0x140a69a80 <.text+0xa68a80>
140a69a46: e8 e5 e6 1a 00              	callq	0x140c18130 <.text+0xc17130>
140a69a4b: 48 89 c1                    	movq	%rax, %rcx
140a69a4e: 48 8b 00                    	movq	(%rax), %rax
140a69a51: 48 83 f8 01                 	cmpq	$0x1, %rax
140a69a55: 75 4d                       	jne	0x140a69aa4 <.text+0xa68aa4>
140a69a57: 48 83 c1 08                 	addq	$0x8, %rcx
140a69a5b: 48 83 39 00                 	cmpq	$0x0, (%rcx)
140a69a5f: 75 7d                       	jne	0x140a69ade <.text+0xa68ade>
140a69a61: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
140a69a68: 48 89 4d 40                 	movq	%rcx, 0x40(%rbp)
140a69a6c: 48 83 c1 08                 	addq	$0x8, %rcx
140a69a70: 48 8b 55 e8                 	movq	-0x18(%rbp), %rdx
140a69a74: e8 77 d6 1a 00              	callq	0x140c170f0 <.text+0xc160f0>
140a69a79: 48 8b 45 40                 	movq	0x40(%rbp), %rax
140a69a7d: 48 ff 00                    	incq	(%rax)
140a69a80: 0f 28 75 60                 	movaps	0x60(%rbp), %xmm6
140a69a84: 0f 28 7d 70                 	movaps	0x70(%rbp), %xmm7
140a69a88: 44 0f 28 85 80 00 00 00     	movaps	0x80(%rbp), %xmm8
140a69a90: 48 81 c4 18 01 00 00        	addq	$0x118, %rsp            # imm = 0x118
140a69a97: 5b                          	popq	%rbx
140a69a98: 5f                          	popq	%rdi
140a69a99: 5e                          	popq	%rsi
140a69a9a: 41 5c                       	popq	%r12
140a69a9c: 41 5d                       	popq	%r13
140a69a9e: 41 5e                       	popq	%r14
140a69aa0: 41 5f                       	popq	%r15
140a69aa2: 5d                          	popq	%rbp
140a69aa3: c3                          	retq
140a69aa4: 48 85 c0                    	testq	%rax, %rax
140a69aa7: 75 29                       	jne	0x140a69ad2 <.text+0xa68ad2>
140a69aa9: 31 d2                       	xorl	%edx, %edx
140a69aab: e8 20 ed e8 01              	callq	0x1428f87d0 <.text+0x28f77d0>
140a69ab0: 48 89 c1                    	movq	%rax, %rcx
140a69ab3: 48 85 c0                    	testq	%rax, %rax
140a69ab6: 75 a3                       	jne	0x140a69a5b <.text+0xa68a5b>
140a69ab8: eb 18                       	jmp	0x140a69ad2 <.text+0xa68ad2>
140a69aba: 48 85 c0                    	testq	%rax, %rax
140a69abd: 75 13                       	jne	0x140a69ad2 <.text+0xa68ad2>
140a69abf: 31 d2                       	xorl	%edx, %edx
140a69ac1: e8 0a ed e8 01              	callq	0x1428f87d0 <.text+0x28f77d0>
140a69ac6: 48 89 c1                    	movq	%rax, %rcx
140a69ac9: 48 85 c0                    	testq	%rax, %rax
140a69acc: 0f 85 96 fa ff ff           	jne	0x140a69568 <.text+0xa68568>
140a69ad2: 48 8d 0d c7 0f 1d 02        	leaq	0x21d0fc7(%rip), %rcx   # 0x142c3aaa0
140a69ad9: e8 f2 e5 e6 01              	callq	0x1428d80d0 <.text+0x28d70d0>
140a69ade: 48 8d 0d 8b 10 1d 02        	leaq	0x21d108b(%rip), %rcx   # 0x142c3ab70
140a69ae5: e8 66 f7 e6 01              	callq	0x1428d9250 <.text+0x28d8250>
140a69aea: cc                          	int3
140a69aeb: 0f 1f 44 00 00              	nopl	(%rax,%rax)
140a69af0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a69af5: 55                          	pushq	%rbp
140a69af6: 41 57                       	pushq	%r15
140a69af8: 41 56                       	pushq	%r14
140a69afa: 41 55                       	pushq	%r13
140a69afc: 41 54                       	pushq	%r12
140a69afe: 56                          	pushq	%rsi
140a69aff: 57                          	pushq	%rdi
140a69b00: 53                          	pushq	%rbx
140a69b01: 48 83 ec 68                 	subq	$0x68, %rsp
140a69b05: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a69b0c: 44 0f 29 44 24 30           	movaps	%xmm8, 0x30(%rsp)
140a69b12: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
140a69b17: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
140a69b1c: 48 8b 45 40                 	movq	0x40(%rbp), %rax
140a69b20: 48 ff 00                    	incq	(%rax)
140a69b23: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
140a69b28: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
140a69b2d: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
140a69b33: 48 83 c4 68                 	addq	$0x68, %rsp
140a69b37: 5b                          	popq	%rbx
140a69b38: 5f                          	popq	%rdi
140a69b39: 5e                          	popq	%rsi
140a69b3a: 41 5c                       	popq	%r12
140a69b3c: 41 5d                       	popq	%r13
140a69b3e: 41 5e                       	popq	%r14
140a69b40: 41 5f                       	popq	%r15
140a69b42: 5d                          	popq	%rbp
140a69b43: c3                          	retq
140a69b44: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
140a69b50: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a69b55: 55                          	pushq	%rbp
140a69b56: 41 57                       	pushq	%r15
140a69b58: 41 56                       	pushq	%r14
140a69b5a: 41 55                       	pushq	%r13
140a69b5c: 41 54                       	pushq	%r12
140a69b5e: 56                          	pushq	%rsi
140a69b5f: 57                          	pushq	%rdi
140a69b60: 53                          	pushq	%rbx
140a69b61: 48 83 ec 68                 	subq	$0x68, %rsp
140a69b65: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a69b6c: 44 0f 29 44 24 30           	movaps	%xmm8, 0x30(%rsp)
140a69b72: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
140a69b77: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
140a69b7c: 48 8b 45 40                 	movq	0x40(%rbp), %rax
140a69b80: 48 ff 00                    	incq	(%rax)
140a69b83: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
140a69b88: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
140a69b8d: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
140a69b93: 48 83 c4 68                 	addq	$0x68, %rsp
140a69b97: 5b                          	popq	%rbx
140a69b98: 5f                          	popq	%rdi
140a69b99: 5e                          	popq	%rsi
140a69b9a: 41 5c                       	popq	%r12
140a69b9c: 41 5d                       	popq	%r13
140a69b9e: 41 5e                       	popq	%r14
140a69ba0: 41 5f                       	popq	%r15
140a69ba2: 5d                          	popq	%rbp
140a69ba3: c3                          	retq
140a69ba4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
140a69bb0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a69bb5: 55                          	pushq	%rbp
140a69bb6: 41 57                       	pushq	%r15
140a69bb8: 41 56                       	pushq	%r14
140a69bba: 41 55                       	pushq	%r13
140a69bbc: 41 54                       	pushq	%r12
140a69bbe: 56                          	pushq	%rsi
140a69bbf: 57                          	pushq	%rdi
140a69bc0: 53                          	pushq	%rbx
140a69bc1: 48 83 ec 68                 	subq	$0x68, %rsp
140a69bc5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a69bcc: 44 0f 29 44 24 30           	movaps	%xmm8, 0x30(%rsp)
140a69bd2: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
140a69bd7: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
140a69bdc: 48 83 7d e0 00              	cmpq	$0x0, -0x20(%rbp)
140a69be1: 74 20                       	je	0x140a69c03 <.text+0xa68c03>
140a69be3: 48 8d 45 e8                 	leaq	-0x18(%rbp), %rax
140a69be7: 48 89 45 c8                 	movq	%rax, -0x38(%rbp)
140a69beb: 48 8d 45 c8                 	leaq	-0x38(%rbp), %rax
140a69bef: 48 89 45 c0                 	movq	%rax, -0x40(%rbp)
140a69bf3: 48 8d 0d fe af 06 02        	leaq	0x206affe(%rip), %rcx   # 0x142ad4bf8
140a69bfa: 48 8d 55 c0                 	leaq	-0x40(%rbp), %rdx
140a69bfe: e8 cd 56 8c 00              	callq	0x14132f2d0 <.text+0x132e2d0>
140a69c03: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
140a69c08: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
140a69c0d: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
140a69c13: 48 83 c4 68                 	addq	$0x68, %rsp
140a69c17: 5b                          	popq	%rbx
140a69c18: 5f                          	popq	%rdi
140a69c19: 5e                          	popq	%rsi
140a69c1a: 41 5c                       	popq	%r12
140a69c1c: 41 5d                       	popq	%r13
140a69c1e: 41 5e                       	popq	%r14
140a69c20: 41 5f                       	popq	%r15
140a69c22: 5d                          	popq	%rbp
140a69c23: c3                          	retq

D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a69c30: 55                          	pushq	%rbp
140a69c31: 41 57                       	pushq	%r15
140a69c33: 41 56                       	pushq	%r14
140a69c35: 41 55                       	pushq	%r13
140a69c37: 41 54                       	pushq	%r12
140a69c39: 56                          	pushq	%rsi
140a69c3a: 57                          	pushq	%rdi
140a69c3b: 53                          	pushq	%rbx
140a69c3c: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
140a69c43: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140a69c4b: 48 c7 45 30 fe ff ff ff     	movq	$-0x2, 0x30(%rbp)
140a69c53: 48 89 d6                    	movq	%rdx, %rsi
140a69c56: 48 89 cf                    	movq	%rcx, %rdi
140a69c59: e8 a2 cd 1a 00              	callq	0x140c16a00 <.text+0xc15a00>
140a69c5e: 84 c0                       	testb	%al, %al
140a69c60: 74 5f                       	je	0x140a69cc1 <.text+0xa68cc1>
140a69c62: 4c 8d 35 43 af 06 02        	leaq	0x206af43(%rip), %r14   # 0x142ad4bac
140a69c69: bb 45 00 00 00              	movl	$0x45, %ebx
140a69c6e: ba 45 00 00 00              	movl	$0x45, %edx
140a69c73: 4c 89 f1                    	movq	%r14, %rcx
140a69c76: e8 05 d7 ff ff              	callq	0x140a67380 <.text+0xa66380>
140a69c7b: 48 83 f8 01                 	cmpq	$0x1, %rax
140a69c7f: 0f 85 b1 00 00 00           	jne	0x140a69d36 <.text+0xa68d36>
140a69c85: 49 89 d1                    	movq	%rdx, %r9
140a69c88: 48 85 d2                    	testq	%rdx, %rdx
140a69c8b: 74 3d                       	je	0x140a69cca <.text+0xa68cca>
140a69c8d: 49 83 f9 45                 	cmpq	$0x45, %r9
140a69c91: 73 35                       	jae	0x140a69cc8 <.text+0xa68cc8>
140a69c93: 48 8d 05 12 af 06 02        	leaq	0x206af12(%rip), %rax   # 0x142ad4bac
140a69c9a: 41 80 3c 01 bf              	cmpb	$-0x41, (%r9,%rax)
140a69c9f: 7f 29                       	jg	0x140a69cca <.text+0xa68cca>
140a69ca1: 48 8d 05 d0 af 06 02        	leaq	0x206afd0(%rip), %rax   # 0x142ad4c78
140a69ca8: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140a69cad: 48 8d 0d f8 ae 06 02        	leaq	0x206aef8(%rip), %rcx   # 0x142ad4bac
140a69cb4: ba 45 00 00 00              	movl	$0x45, %edx
140a69cb9: 45 31 c0                    	xorl	%r8d, %r8d
140a69cbc: e8 7f fc e6 01              	callq	0x1428d9940 <.text+0x28d8940>
140a69cc1: 31 c0                       	xorl	%eax, %eax
140a69cc3: e9 ce 01 00 00              	jmp	0x140a69e96 <.text+0xa68e96>
140a69cc8: 75 d7                       	jne	0x140a69ca1 <.text+0xa68ca1>
140a69cca: 4c 8d 35 db ae 06 02        	leaq	0x206aedb(%rip), %r14   # 0x142ad4bac
140a69cd1: 4c 89 f1                    	movq	%r14, %rcx
140a69cd4: 4c 89 ca                    	movq	%r9, %rdx
140a69cd7: e8 a4 d6 ff ff              	callq	0x140a67380 <.text+0xa66380>
140a69cdc: 48 83 f8 01                 	cmpq	$0x1, %rax
140a69ce0: 75 54                       	jne	0x140a69d36 <.text+0xa68d36>
140a69ce2: 49 89 d0                    	movq	%rdx, %r8
140a69ce5: 49 83 c0 02                 	addq	$0x2, %r8
140a69ce9: 74 39                       	je	0x140a69d24 <.text+0xa68d24>
140a69ceb: 49 83 f8 45                 	cmpq	$0x45, %r8
140a69cef: 73 31                       	jae	0x140a69d22 <.text+0xa68d22>
140a69cf1: 48 8d 05 b4 ae 06 02        	leaq	0x206aeb4(%rip), %rax   # 0x142ad4bac
140a69cf8: 41 80 3c 00 bf              	cmpb	$-0x41, (%r8,%rax)
140a69cfd: 7f 25                       	jg	0x140a69d24 <.text+0xa68d24>
140a69cff: 48 8d 05 8a af 06 02        	leaq	0x206af8a(%rip), %rax   # 0x142ad4c90
140a69d06: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140a69d0b: 48 8d 0d 9a ae 06 02        	leaq	0x206ae9a(%rip), %rcx   # 0x142ad4bac
140a69d12: ba 45 00 00 00              	movl	$0x45, %edx
140a69d17: 41 b9 45 00 00 00           	movl	$0x45, %r9d
140a69d1d: e8 1e fc e6 01              	callq	0x1428d9940 <.text+0x28d8940>
140a69d22: 75 db                       	jne	0x140a69cff <.text+0xa68cff>
140a69d24: bb 43 00 00 00              	movl	$0x43, %ebx
140a69d29: 48 29 d3                    	subq	%rdx, %rbx
140a69d2c: 4c 8d 35 79 ae 06 02        	leaq	0x206ae79(%rip), %r14   # 0x142ad4bac
140a69d33: 4d 01 c6                    	addq	%r8, %r14
140a69d36: 48 8d 05 a1 b4 06 02        	leaq	0x206b4a1(%rip), %rax   # 0x142ad51de
140a69d3d: 4c 8d 3d 60 b4 06 02        	leaq	0x206b460(%rip), %r15   # 0x142ad51a4
140a69d44: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
140a69d50: 0f be 48 ff                 	movsbl	-0x1(%rax), %ecx
140a69d54: 85 c9                       	testl	%ecx, %ecx
140a69d56: 78 18                       	js	0x140a69d70 <.text+0xa68d70>
140a69d58: 48 ff c8                    	decq	%rax
140a69d5b: 83 f9 5c                    	cmpl	$0x5c, %ecx
140a69d5e: 75 69                       	jne	0x140a69dc9 <.text+0xa68dc9>
140a69d60: eb 7d                       	jmp	0x140a69ddf <.text+0xa68ddf>
140a69d62: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
140a69d70: 0f b6 50 fe                 	movzbl	-0x2(%rax), %edx
140a69d74: 80 fa c0                    	cmpb	$-0x40, %dl
140a69d77: 7d 25                       	jge	0x140a69d9e <.text+0xa68d9e>
140a69d79: 44 0f b6 40 fd              	movzbl	-0x3(%rax), %r8d
140a69d7e: 41 80 f8 c0                 	cmpb	$-0x40, %r8b
140a69d82: 7d 23                       	jge	0x140a69da7 <.text+0xa68da7>
140a69d84: 44 0f b6 48 fc              	movzbl	-0x4(%rax), %r9d
140a69d89: 48 83 c0 fc                 	addq	$-0x4, %rax
140a69d8d: 41 83 e1 07                 	andl	$0x7, %r9d
140a69d91: 41 c1 e1 06                 	shll	$0x6, %r9d
140a69d95: 41 83 e0 3f                 	andl	$0x3f, %r8d
140a69d99: 45 09 c8                    	orl	%r9d, %r8d
140a69d9c: eb 11                       	jmp	0x140a69daf <.text+0xa68daf>
140a69d9e: 48 83 c0 fe                 	addq	$-0x2, %rax
140a69da2: 83 e2 1f                    	andl	$0x1f, %edx
140a69da5: eb 12                       	jmp	0x140a69db9 <.text+0xa68db9>
140a69da7: 48 83 c0 fd                 	addq	$-0x3, %rax
140a69dab: 41 83 e0 0f                 	andl	$0xf, %r8d
140a69daf: 41 c1 e0 06                 	shll	$0x6, %r8d
140a69db3: 83 e2 3f                    	andl	$0x3f, %edx
140a69db6: 44 09 c2                    	orl	%r8d, %edx
140a69db9: c1 e2 06                    	shll	$0x6, %edx
140a69dbc: 80 e1 3f                    	andb	$0x3f, %cl
140a69dbf: 0f b6 c9                    	movzbl	%cl, %ecx
140a69dc2: 09 d1                       	orl	%edx, %ecx
140a69dc4: 83 f9 5c                    	cmpl	$0x5c, %ecx
140a69dc7: 74 16                       	je	0x140a69ddf <.text+0xa68ddf>
140a69dc9: 83 f9 2f                    	cmpl	$0x2f, %ecx
140a69dcc: 74 11                       	je	0x140a69ddf <.text+0xa68ddf>
140a69dce: 4c 39 f8                    	cmpq	%r15, %rax
140a69dd1: 0f 85 79 ff ff ff           	jne	0x140a69d50 <.text+0xa68d50>
140a69dd7: 41 bc 3a 00 00 00           	movl	$0x3a, %r12d
140a69ddd: eb 50                       	jmp	0x140a69e2f <.text+0xa68e2f>
140a69ddf: 4c 8d 3d be b3 06 02        	leaq	0x206b3be(%rip), %r15   # 0x142ad51a4
140a69de6: 4c 29 f8                    	subq	%r15, %rax
140a69de9: 49 89 c0                    	movq	%rax, %r8
140a69dec: 49 ff c0                    	incq	%r8
140a69def: 74 32                       	je	0x140a69e23 <.text+0xa68e23>
140a69df1: 49 83 f8 3a                 	cmpq	$0x3a, %r8
140a69df5: 73 2a                       	jae	0x140a69e21 <.text+0xa68e21>
140a69df7: 43 80 3c 38 bf              	cmpb	$-0x41, (%r8,%r15)
140a69dfc: 7f 25                       	jg	0x140a69e23 <.text+0xa68e23>
140a69dfe: 48 8d 05 53 ae 06 02        	leaq	0x206ae53(%rip), %rax   # 0x142ad4c58
140a69e05: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140a69e0a: 48 8d 0d 93 b3 06 02        	leaq	0x206b393(%rip), %rcx   # 0x142ad51a4
140a69e11: ba 3a 00 00 00              	movl	$0x3a, %edx
140a69e16: 41 b9 3a 00 00 00           	movl	$0x3a, %r9d
140a69e1c: e8 1f fb e6 01              	callq	0x1428d9940 <.text+0x28d8940>
140a69e21: 75 db                       	jne	0x140a69dfe <.text+0xa68dfe>
140a69e23: 41 bc 39 00 00 00           	movl	$0x39, %r12d
140a69e29: 49 29 c4                    	subq	%rax, %r12
140a69e2c: 4d 01 c7                    	addq	%r8, %r15
140a69e2f: e8 fc e2 1a 00              	callq	0x140c18130 <.text+0xc17130>
140a69e34: 48 89 c1                    	movq	%rax, %rcx
140a69e37: 48 8b 00                    	movq	(%rax), %rax
140a69e3a: 48 83 f8 01                 	cmpq	$0x1, %rax
140a69e3e: 0f 85 15 05 00 00           	jne	0x140a6a359 <.text+0xa69359>
140a69e44: 48 83 c1 08                 	addq	$0x8, %rcx
140a69e48: 48 83 39 00                 	cmpq	$0x0, (%rcx)
140a69e4c: 0f 85 2b 05 00 00           	jne	0x140a6a37d <.text+0xa6937d>
140a69e52: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
140a69e59: 48 89 4d 20                 	movq	%rcx, 0x20(%rbp)
140a69e5d: 48 83 c1 08                 	addq	$0x8, %rcx
140a69e61: 4c 89 64 24 20              	movq	%r12, 0x20(%rsp)
140a69e66: 48 c7 44 24 30 00 00 00 00  	movq	$0x0, 0x30(%rsp)
140a69e6f: 48 c7 44 24 28 01 00 00 00  	movq	$0x1, 0x28(%rsp)
140a69e78: 4c 89 f2                    	movq	%r14, %rdx
140a69e7b: 49 89 d8                    	movq	%rbx, %r8
140a69e7e: 4d 89 f9                    	movq	%r15, %r9
140a69e81: e8 ba ce 1a 00              	callq	0x140c16d40 <.text+0xc15d40>
140a69e86: 48 8b 4d 20                 	movq	0x20(%rbp), %rcx
140a69e8a: 48 ff 01                    	incq	(%rcx)
140a69e8d: 48 89 45 f0                 	movq	%rax, -0x10(%rbp)
140a69e91: b8 01 00 00 00              	movl	$0x1, %eax
140a69e96: 48 89 45 e8                 	movq	%rax, -0x18(%rbp)
140a69e9a: 48 8b 07                    	movq	(%rdi), %rax
140a69e9d: 4c 8b 47 08                 	movq	0x8(%rdi), %r8
140a69ea1: 8b 4f 14                    	movl	0x14(%rdi), %ecx
140a69ea4: 89 4d 1c                    	movl	%ecx, 0x1c(%rbp)
140a69ea7: 4c 8b 90 f8 00 00 00        	movq	0xf8(%rax), %r10
140a69eae: 48 8b 88 00 01 00 00        	movq	0x100(%rax), %rcx
140a69eb5: 4d 8d 1c 8a                 	leaq	(%r10,%rcx,4), %r11
140a69eb9: 0f b6 88 08 01 00 00        	movzbl	0x108(%rax), %ecx
140a69ec0: 88 4d 20                    	movb	%cl, 0x20(%rbp)
140a69ec3: 48 8b 0e                    	movq	(%rsi), %rcx
140a69ec6: 48 89 4d d0                 	movq	%rcx, -0x30(%rbp)
140a69eca: 41 be 08 00 00 00           	movl	$0x8, %r14d
140a69ed0: 31 f6                       	xorl	%esi, %esi
140a69ed2: 31 db                       	xorl	%ebx, %ebx
140a69ed4: 45 31 c9                    	xorl	%r9d, %r9d
140a69ed7: 45 31 ff                    	xorl	%r15d, %r15d
140a69eda: 45 31 e4                    	xorl	%r12d, %r12d
140a69edd: 48 c7 45 10 00 00 00 00     	movq	$0x0, 0x10(%rbp)
140a69ee5: c7 45 2c 00 00 00 00        	movl	$0x0, 0x2c(%rbp)
140a69eec: 80 7d 20 00                 	cmpb	$0x0, 0x20(%rbp)
140a69ef0: 0f 84 8a 00 00 00           	je	0x140a69f80 <.text+0xa68f80>
140a69ef6: 4d 39 e7                    	cmpq	%r12, %r15
140a69ef9: 0f 85 a0 02 00 00           	jne	0x140a6a19f <.text+0xa6919f>
140a69eff: 90                          	nop
140a69f00: 4d 39 da                    	cmpq	%r11, %r10
140a69f03: 0f 84 e5 03 00 00           	je	0x140a6a2ee <.text+0xa692ee>
140a69f09: 41 8b 12                    	movl	(%r10), %edx
140a69f0c: 49 83 c2 04                 	addq	$0x4, %r10
140a69f10: 49 8b 88 a8 01 00 00        	movq	0x1a8(%r8), %rcx
140a69f17: 48 8d 14 d2                 	leaq	(%rdx,%rdx,8), %rdx
140a69f1b: 4c 8b 64 d1 10              	movq	0x10(%rcx,%rdx,8), %r12
140a69f20: 4d 85 e4                    	testq	%r12, %r12
140a69f23: 74 db                       	je	0x140a69f00 <.text+0xa68f00>
140a69f25: 4c 89 cf                    	movq	%r9, %rdi
140a69f28: 48 8d 14 d1                 	leaq	(%rcx,%rdx,8), %rdx
140a69f2c: 48 8b b0 10 01 00 00        	movq	0x110(%rax), %rsi
140a69f33: 48 8b 4a 38                 	movq	0x38(%rdx), %rcx
140a69f37: 4c 8b 4a 40                 	movq	0x40(%rdx), %r9
140a69f3b: 49 39 f1                    	cmpq	%rsi, %r9
140a69f3e: 0f 86 d8 00 00 00           	jbe	0x140a6a01c <.text+0xa6901c>
140a69f44: 48 8b 34 f1                 	movq	(%rcx,%rsi,8), %rsi
140a69f48: 48 8b 52 18                 	movq	0x18(%rdx), %rdx
140a69f4c: 48 85 f6                    	testq	%rsi, %rsi
140a69f4f: 0f 84 bb 01 00 00           	je	0x140a6a110 <.text+0xa69110>
140a69f55: 48 f7 d6                    	notq	%rsi
140a69f58: 48 8d 34 76                 	leaq	(%rsi,%rsi,2), %rsi
140a69f5c: 48 c1 e6 04                 	shlq	$0x4, %rsi
140a69f60: 48 8b 5c 32 10              	movq	0x10(%rdx,%rsi), %rbx
140a69f65: 48 8b b0 20 01 00 00        	movq	0x120(%rax), %rsi
140a69f6c: 49 39 f1                    	cmpq	%rsi, %r9
140a69f6f: 0f 87 a9 01 00 00           	ja	0x140a6a11e <.text+0xa6911e>
140a69f75: e9 d5 01 00 00              	jmp	0x140a6a14f <.text+0xa6914f>
140a69f7a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140a69f80: 4d 39 e7                    	cmpq	%r12, %r15
140a69f83: 0f 85 76 01 00 00           	jne	0x140a6a0ff <.text+0xa690ff>
140a69f89: 0f 1f 80 00 00 00 00        	nopl	(%rax)
140a69f90: 4d 39 da                    	cmpq	%r11, %r10
140a69f93: 0f 84 55 03 00 00           	je	0x140a6a2ee <.text+0xa692ee>
140a69f99: 41 8b 12                    	movl	(%r10), %edx
140a69f9c: 49 83 c2 04                 	addq	$0x4, %r10
140a69fa0: 49 8b 88 00 01 00 00        	movq	0x100(%r8), %rcx
140a69fa7: 48 8d 14 92                 	leaq	(%rdx,%rdx,4), %rdx
140a69fab: 48 c1 e2 05                 	shlq	$0x5, %rdx
140a69faf: 4c 8b 64 11 58              	movq	0x58(%rcx,%rdx), %r12
140a69fb4: 4d 85 e4                    	testq	%r12, %r12
140a69fb7: 74 d7                       	je	0x140a69f90 <.text+0xa68f90>
140a69fb9: 4d 89 cf                    	movq	%r9, %r15
140a69fbc: 48 01 d1                    	addq	%rdx, %rcx
140a69fbf: 8b 91 94 00 00 00           	movl	0x94(%rcx), %edx
140a69fc5: 49 8b b8 a8 01 00 00        	movq	0x1a8(%r8), %rdi
140a69fcc: 48 8d 1c d2                 	leaq	(%rdx,%rdx,8), %rbx
140a69fd0: 4c 8d 0c df                 	leaq	(%rdi,%rbx,8), %r9
140a69fd4: 48 8b b0 10 01 00 00        	movq	0x110(%rax), %rsi
140a69fdb: 48 8b 54 df 38              	movq	0x38(%rdi,%rbx,8), %rdx
140a69fe0: 48 8b 5c df 40              	movq	0x40(%rdi,%rbx,8), %rbx
140a69fe5: 48 39 f3                    	cmpq	%rsi, %rbx
140a69fe8: 76 4d                       	jbe	0x140a6a037 <.text+0xa69037>
140a69fea: 48 8b 3c f2                 	movq	(%rdx,%rsi,8), %rdi
140a69fee: 49 8b 71 18                 	movq	0x18(%r9), %rsi
140a69ff2: 48 85 ff                    	testq	%rdi, %rdi
140a69ff5: 74 44                       	je	0x140a6a03b <.text+0xa6903b>
140a69ff7: 48 f7 d7                    	notq	%rdi
140a69ffa: 4c 8d 0c 7f                 	leaq	(%rdi,%rdi,2), %r9
140a69ffe: 49 c1 e1 04                 	shlq	$0x4, %r9
140a6a002: 4e 8b 6c 0e 10              	movq	0x10(%rsi,%r9), %r13
140a6a007: 4c 8b b0 20 01 00 00        	movq	0x120(%rax), %r14
140a6a00e: 4c 8b 89 88 00 00 00        	movq	0x88(%rcx), %r9
140a6a015: 4d 39 f1                    	cmpq	%r14, %r9
140a6a018: 77 37                       	ja	0x140a6a051 <.text+0xa69051>
140a6a01a: eb 72                       	jmp	0x140a6a08e <.text+0xa6908e>
140a6a01c: 48 8b 52 18                 	movq	0x18(%rdx), %rdx
140a6a020: 31 db                       	xorl	%ebx, %ebx
140a6a022: 48 8b b0 20 01 00 00        	movq	0x120(%rax), %rsi
140a6a029: 49 39 f1                    	cmpq	%rsi, %r9
140a6a02c: 0f 86 1d 01 00 00           	jbe	0x140a6a14f <.text+0xa6914f>
140a6a032: e9 e7 00 00 00              	jmp	0x140a6a11e <.text+0xa6911e>
140a6a037: 49 8b 71 18                 	movq	0x18(%r9), %rsi
140a6a03b: 45 31 ed                    	xorl	%r13d, %r13d
140a6a03e: 4c 8b b0 20 01 00 00        	movq	0x120(%rax), %r14
140a6a045: 4c 8b 89 88 00 00 00        	movq	0x88(%rcx), %r9
140a6a04c: 4d 39 f1                    	cmpq	%r14, %r9
140a6a04f: 76 3d                       	jbe	0x140a6a08e <.text+0xa6908e>
140a6a051: 48 8b b9 80 00 00 00        	movq	0x80(%rcx), %rdi
140a6a058: 4a 83 3c f7 00              	cmpq	$0x0, (%rdi,%r14,8)
140a6a05d: 74 2f                       	je	0x140a6a08e <.text+0xa6908e>
140a6a05f: 40 b7 01                    	movb	$0x1, %dil
140a6a062: 4c 39 f3                    	cmpq	%r14, %rbx
140a6a065: 76 4b                       	jbe	0x140a6a0b2 <.text+0xa690b2>
140a6a067: 4a 8b 1c f2                 	movq	(%rdx,%r14,8), %rbx
140a6a06b: 48 85 db                    	testq	%rbx, %rbx
140a6a06e: 74 42                       	je	0x140a6a0b2 <.text+0xa690b2>
140a6a070: 48 f7 d3                    	notq	%rbx
140a6a073: 48 8d 1c 5b                 	leaq	(%rbx,%rbx,2), %rbx
140a6a077: 48 c1 e3 04                 	shlq	$0x4, %rbx
140a6a07b: 4c 8b 7c 1e 10              	movq	0x10(%rsi,%rbx), %r15
140a6a080: 48 8b 98 28 01 00 00        	movq	0x128(%rax), %rbx
140a6a087: 49 39 d9                    	cmpq	%rbx, %r9
140a6a08a: 77 10                       	ja	0x140a6a09c <.text+0xa6909c>
140a6a08c: eb 33                       	jmp	0x140a6a0c1 <.text+0xa690c1>
140a6a08e: 31 ff                       	xorl	%edi, %edi
140a6a090: 48 8b 98 28 01 00 00        	movq	0x128(%rax), %rbx
140a6a097: 49 39 d9                    	cmpq	%rbx, %r9
140a6a09a: 76 25                       	jbe	0x140a6a0c1 <.text+0xa690c1>
140a6a09c: 4c 8b 89 80 00 00 00        	movq	0x80(%rcx), %r9
140a6a0a3: 49 83 3c d9 00              	cmpq	$0x0, (%r9,%rbx,8)
140a6a0a8: 41 0f 95 c1                 	setne	%r9b
140a6a0ac: 44 89 4d 2c                 	movl	%r9d, 0x2c(%rbp)
140a6a0b0: eb 16                       	jmp	0x140a6a0c8 <.text+0xa690c8>
140a6a0b2: 45 31 ff                    	xorl	%r15d, %r15d
140a6a0b5: 48 8b 98 28 01 00 00        	movq	0x128(%rax), %rbx
140a6a0bc: 49 39 d9                    	cmpq	%rbx, %r9
140a6a0bf: 77 db                       	ja	0x140a6a09c <.text+0xa6909c>
140a6a0c1: c7 45 2c 00 00 00 00        	movl	$0x0, 0x2c(%rbp)
140a6a0c8: 4d 89 f9                    	movq	%r15, %r9
140a6a0cb: 48 8b 98 18 01 00 00        	movq	0x118(%rax), %rbx
140a6a0d2: 48 8b 14 da                 	movq	(%rdx,%rbx,8), %rdx
140a6a0d6: 48 f7 d2                    	notq	%rdx
140a6a0d9: 48 8d 14 52                 	leaq	(%rdx,%rdx,2), %rdx
140a6a0dd: 48 c1 e2 04                 	shlq	$0x4, %rdx
140a6a0e1: 48 8b 5c 16 10              	movq	0x10(%rsi,%rdx), %rbx
140a6a0e6: 48 89 5d 10                 	movq	%rbx, 0x10(%rbp)
140a6a0ea: 48 8b 54 16 28              	movq	0x28(%rsi,%rdx), %rdx
140a6a0ef: 48 89 55 08                 	movq	%rdx, 0x8(%rbp)
140a6a0f3: 4c 8b 71 50                 	movq	0x50(%rcx), %r14
140a6a0f7: 45 31 ff                    	xorl	%r15d, %r15d
140a6a0fa: 4c 89 ee                    	movq	%r13, %rsi
140a6a0fd: 89 fb                       	movl	%edi, %ebx
140a6a0ff: 4c 89 f9                    	movq	%r15, %rcx
140a6a102: 48 c1 e1 04                 	shlq	$0x4, %rcx
140a6a106: 41 8b 4c 0e 08              	movl	0x8(%r14,%rcx), %ecx
140a6a10b: e9 92 00 00 00              	jmp	0x140a6a1a2 <.text+0xa691a2>
140a6a110: 31 db                       	xorl	%ebx, %ebx
140a6a112: 48 8b b0 20 01 00 00        	movq	0x120(%rax), %rsi
140a6a119: 49 39 f1                    	cmpq	%rsi, %r9
140a6a11c: 76 31                       	jbe	0x140a6a14f <.text+0xa6914f>
140a6a11e: 48 8b 34 f1                 	movq	(%rcx,%rsi,8), %rsi
140a6a122: 48 85 f6                    	testq	%rsi, %rsi
140a6a125: 74 28                       	je	0x140a6a14f <.text+0xa6914f>
140a6a127: 48 f7 d6                    	notq	%rsi
140a6a12a: 48 8d 34 76                 	leaq	(%rsi,%rsi,2), %rsi
140a6a12e: 48 c1 e6 04                 	shlq	$0x4, %rsi
140a6a132: 48 8b 7c 32 10              	movq	0x10(%rdx,%rsi), %rdi
140a6a137: 41 b5 01                    	movb	$0x1, %r13b
140a6a13a: 48 8b b0 28 01 00 00        	movq	0x128(%rax), %rsi
140a6a141: 49 39 f1                    	cmpq	%rsi, %r9
140a6a144: 77 18                       	ja	0x140a6a15e <.text+0xa6915e>
140a6a146: c7 45 2c 00 00 00 00        	movl	$0x0, 0x2c(%rbp)
140a6a14d: eb 1c                       	jmp	0x140a6a16b <.text+0xa6916b>
140a6a14f: 45 31 ed                    	xorl	%r13d, %r13d
140a6a152: 48 8b b0 28 01 00 00        	movq	0x128(%rax), %rsi
140a6a159: 49 39 f1                    	cmpq	%rsi, %r9
140a6a15c: 76 e8                       	jbe	0x140a6a146 <.text+0xa69146>
140a6a15e: 48 83 3c f1 00              	cmpq	$0x0, (%rcx,%rsi,8)
140a6a163: 41 0f 95 c1                 	setne	%r9b
140a6a167: 44 89 4d 2c                 	movl	%r9d, 0x2c(%rbp)
140a6a16b: 49 89 f9                    	movq	%rdi, %r9
140a6a16e: 48 8b b0 18 01 00 00        	movq	0x118(%rax), %rsi
140a6a175: 48 8b 0c f1                 	movq	(%rcx,%rsi,8), %rcx
140a6a179: 48 f7 d1                    	notq	%rcx
140a6a17c: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
140a6a180: 48 c1 e1 04                 	shlq	$0x4, %rcx
140a6a184: 48 8b 74 0a 10              	movq	0x10(%rdx,%rcx), %rsi
140a6a189: 48 89 75 10                 	movq	%rsi, 0x10(%rbp)
140a6a18d: 48 8b 4c 0a 28              	movq	0x28(%rdx,%rcx), %rcx
140a6a192: 48 89 4d 08                 	movq	%rcx, 0x8(%rbp)
140a6a196: 45 31 ff                    	xorl	%r15d, %r15d
140a6a199: 48 89 de                    	movq	%rbx, %rsi
140a6a19c: 44 89 eb                    	movl	%r13d, %ebx
140a6a19f: 44 89 f9                    	movl	%r15d, %ecx
140a6a1a2: 48 8d 14 c9                 	leaq	(%rcx,%rcx,8), %rdx
140a6a1a6: 4c 8d 2c 52                 	leaq	(%rdx,%rdx,2), %r13
140a6a1aa: 49 01 f5                    	addq	%rsi, %r13
140a6a1ad: 49 01 cd                    	addq	%rcx, %r13
140a6a1b0: 48 8d 14 49                 	leaq	(%rcx,%rcx,2), %rdx
140a6a1b4: 48 c1 e2 04                 	shlq	$0x4, %rdx
140a6a1b8: 48 03 55 10                 	addq	0x10(%rbp), %rdx
140a6a1bc: 48 8b 7d 08                 	movq	0x8(%rbp), %rdi
140a6a1c0: 48 8d 3c 8f                 	leaq	(%rdi,%rcx,4), %rdi
140a6a1c4: 48 89 7d 00                 	movq	%rdi, (%rbp)
140a6a1c8: 48 c1 e1 04                 	shlq	$0x4, %rcx
140a6a1cc: 4c 01 c9                    	addq	%r9, %rcx
140a6a1cf: f6 c3 01                    	testb	$0x1, %bl
140a6a1d2: bf 00 00 00 00              	movl	$0x0, %edi
140a6a1d7: 48 0f 44 cf                 	cmoveq	%rdi, %rcx
140a6a1db: 41 80 7d 08 00              	cmpb	$0x0, 0x8(%r13)
140a6a1e0: 75 37                       	jne	0x140a6a219 <.text+0xa69219>
140a6a1e2: f3 41 0f 10 4d 0c           	movss	0xc(%r13), %xmm1
140a6a1e8: f3 41 0f 10 45 18           	movss	0x18(%r13), %xmm0
140a6a1ee: f3 0f 59 c1                 	mulss	%xmm1, %xmm0
140a6a1f2: 41 80 7d 09 00              	cmpb	$0x0, 0x9(%r13)
140a6a1f7: 0f 28 d0                    	movaps	%xmm0, %xmm2
140a6a1fa: 75 03                       	jne	0x140a6a1ff <.text+0xa691ff>
140a6a1fc: 0f 28 d1                    	movaps	%xmm1, %xmm2
140a6a1ff: 44 8b 6d 1c                 	movl	0x1c(%rbp), %r13d
140a6a203: 48 8b 7d 00                 	movq	(%rbp), %rdi
140a6a207: 44 89 2f                    	movl	%r13d, (%rdi)
140a6a20a: f3 0f 11 52 1c              	movss	%xmm2, 0x1c(%rdx)
140a6a20f: f3 0f 11 42 20              	movss	%xmm0, 0x20(%rdx)
140a6a214: f3 0f 11 52 24              	movss	%xmm2, 0x24(%rdx)
140a6a219: 49 ff c7                    	incq	%r15
140a6a21c: 48 85 c9                    	testq	%rcx, %rcx
140a6a21f: 48 8b 7d d0                 	movq	-0x30(%rbp), %rdi
140a6a223: 0f 84 c3 fc ff ff           	je	0x140a69eec <.text+0xa68eec>
140a6a229: f6 45 2c 01                 	testb	$0x1, 0x2c(%rbp)
140a6a22d: 0f 84 b9 fc ff ff           	je	0x140a69eec <.text+0xa68eec>
140a6a233: 80 7f 18 00                 	cmpb	$0x0, 0x18(%rdi)
140a6a237: 0f 84 af fc ff ff           	je	0x140a69eec <.text+0xa68eec>
140a6a23d: 4c 8b 6f 38                 	movq	0x38(%rdi), %r13
140a6a241: 4c 3b 69 08                 	cmpq	0x8(%rcx), %r13
140a6a245: 0f 85 a1 fc ff ff           	jne	0x140a69eec <.text+0xa68eec>
140a6a24b: 4c 8b 6f 20                 	movq	0x20(%rdi), %r13
140a6a24f: 41 0f b6 8d 98 00 00 00     	movzbl	0x98(%r13), %ecx
140a6a257: 89 5d fc                    	movl	%ebx, -0x4(%rbp)
140a6a25a: 4c 89 f3                    	movq	%r14, %rbx
140a6a25d: 49 89 c6                    	movq	%rax, %r14
140a6a260: 48 89 f0                    	movq	%rsi, %rax
140a6a263: 4c 89 ce                    	movq	%r9, %rsi
140a6a266: 4c 8d 0d 3f b3 06 02        	leaq	0x206b33f(%rip), %r9    # 0x142ad55ac
140a6a26d: 49 63 0c 89                 	movslq	(%r9,%rcx,4), %rcx
140a6a271: 4c 01 c9                    	addq	%r9, %rcx
140a6a274: 49 89 f1                    	movq	%rsi, %r9
140a6a277: 48 89 c6                    	movq	%rax, %rsi
140a6a27a: 4c 89 f0                    	movq	%r14, %rax
140a6a27d: 49 89 de                    	movq	%rbx, %r14
140a6a280: 8b 5d fc                    	movl	-0x4(%rbp), %ebx
140a6a283: ff e1                       	jmpq	*%rcx
140a6a285: b9 08 00 00 00              	movl	$0x8, %ecx
140a6a28a: eb 05                       	jmp	0x140a6a291 <.text+0xa69291>
140a6a28c: b9 04 00 00 00              	movl	$0x4, %ecx
140a6a291: 49 81 c5 98 00 00 00        	addq	$0x98, %r13
140a6a298: 49 01 cd                    	addq	%rcx, %r13
140a6a29b: eb 26                       	jmp	0x140a6a2c3 <.text+0xa692c3>
140a6a29d: 48 8b 4f 40                 	movq	0x40(%rdi), %rcx
140a6a2a1: 49 8b bd b0 00 00 00        	movq	0xb0(%r13), %rdi
140a6a2a8: 48 39 cf                    	cmpq	%rcx, %rdi
140a6a2ab: 0f 86 d8 00 00 00           	jbe	0x140a6a389 <.text+0xa69389>
140a6a2b1: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
140a6a2b5: 48 c1 e1 02                 	shlq	$0x2, %rcx
140a6a2b9: 49 03 8d a8 00 00 00        	addq	0xa8(%r13), %rcx
140a6a2c0: 49 89 cd                    	movq	%rcx, %r13
140a6a2c3: 41 8b 4d 08                 	movl	0x8(%r13), %ecx
140a6a2c7: 89 4d e0                    	movl	%ecx, -0x20(%rbp)
140a6a2ca: 49 8b 4d 00                 	movq	(%r13), %rcx
140a6a2ce: 48 89 4d d8                 	movq	%rcx, -0x28(%rbp)
140a6a2d2: 8b 4d 1c                    	movl	0x1c(%rbp), %ecx
140a6a2d5: 48 8b 7d 00                 	movq	(%rbp), %rdi
140a6a2d9: 89 0f                       	movl	%ecx, (%rdi)
140a6a2db: 8b 4d e0                    	movl	-0x20(%rbp), %ecx
140a6a2de: 89 4a 18                    	movl	%ecx, 0x18(%rdx)
140a6a2e1: 48 8b 4d d8                 	movq	-0x28(%rbp), %rcx
140a6a2e5: 48 89 4a 10                 	movq	%rcx, 0x10(%rdx)
140a6a2e9: e9 fe fb ff ff              	jmp	0x140a69eec <.text+0xa68eec>
140a6a2ee: 48 83 7d e8 00              	cmpq	$0x0, -0x18(%rbp)
140a6a2f3: 74 3a                       	je	0x140a6a32f <.text+0xa6932f>
140a6a2f5: e8 36 de 1a 00              	callq	0x140c18130 <.text+0xc17130>
140a6a2fa: 48 89 c1                    	movq	%rax, %rcx
140a6a2fd: 48 8b 00                    	movq	(%rax), %rax
140a6a300: 48 83 f8 01                 	cmpq	$0x1, %rax
140a6a304: 75 3d                       	jne	0x140a6a343 <.text+0xa69343>
140a6a306: 48 83 c1 08                 	addq	$0x8, %rcx
140a6a30a: 48 83 39 00                 	cmpq	$0x0, (%rcx)
140a6a30e: 75 6d                       	jne	0x140a6a37d <.text+0xa6937d>
140a6a310: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
140a6a317: 48 89 4d 20                 	movq	%rcx, 0x20(%rbp)
140a6a31b: 48 83 c1 08                 	addq	$0x8, %rcx
140a6a31f: 48 8b 55 f0                 	movq	-0x10(%rbp), %rdx
140a6a323: e8 c8 cd 1a 00              	callq	0x140c170f0 <.text+0xc160f0>
140a6a328: 48 8b 45 20                 	movq	0x20(%rbp), %rax
140a6a32c: 48 ff 00                    	incq	(%rax)
140a6a32f: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
140a6a336: 5b                          	popq	%rbx
140a6a337: 5f                          	popq	%rdi
140a6a338: 5e                          	popq	%rsi
140a6a339: 41 5c                       	popq	%r12
140a6a33b: 41 5d                       	popq	%r13
140a6a33d: 41 5e                       	popq	%r14
140a6a33f: 41 5f                       	popq	%r15
140a6a341: 5d                          	popq	%rbp
140a6a342: c3                          	retq
140a6a343: 48 85 c0                    	testq	%rax, %rax
140a6a346: 75 29                       	jne	0x140a6a371 <.text+0xa69371>
140a6a348: 31 d2                       	xorl	%edx, %edx
140a6a34a: e8 81 e4 e8 01              	callq	0x1428f87d0 <.text+0x28f77d0>
140a6a34f: 48 89 c1                    	movq	%rax, %rcx
140a6a352: 48 85 c0                    	testq	%rax, %rax
140a6a355: 75 b3                       	jne	0x140a6a30a <.text+0xa6930a>
140a6a357: eb 18                       	jmp	0x140a6a371 <.text+0xa69371>
140a6a359: 48 85 c0                    	testq	%rax, %rax
140a6a35c: 75 13                       	jne	0x140a6a371 <.text+0xa69371>
140a6a35e: 31 d2                       	xorl	%edx, %edx
140a6a360: e8 6b e4 e8 01              	callq	0x1428f87d0 <.text+0x28f77d0>
140a6a365: 48 89 c1                    	movq	%rax, %rcx
140a6a368: 48 85 c0                    	testq	%rax, %rax
140a6a36b: 0f 85 d7 fa ff ff           	jne	0x140a69e48 <.text+0xa68e48>
140a6a371: 48 8d 0d 28 07 1d 02        	leaq	0x21d0728(%rip), %rcx   # 0x142c3aaa0
140a6a378: e8 53 dd e6 01              	callq	0x1428d80d0 <.text+0x28d70d0>
140a6a37d: 48 8d 0d ec 07 1d 02        	leaq	0x21d07ec(%rip), %rcx   # 0x142c3ab70
140a6a384: e8 c7 ee e6 01              	callq	0x1428d9250 <.text+0x28d8250>
140a6a389: 4c 8d 05 98 35 05 02        	leaq	0x2053598(%rip), %r8    # 0x142abd928
140a6a390: 48 89 fa                    	movq	%rdi, %rdx
140a6a393: e8 80 f1 e6 01              	callq	0x1428d9518 <.text+0x28d8518>
140a6a398: 0f 0b                       	ud2
140a6a39a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140a6a3a0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a6a3a5: 55                          	pushq	%rbp
140a6a3a6: 41 57                       	pushq	%r15
140a6a3a8: 41 56                       	pushq	%r14
140a6a3aa: 41 55                       	pushq	%r13
140a6a3ac: 41 54                       	pushq	%r12
140a6a3ae: 56                          	pushq	%rsi
140a6a3af: 57                          	pushq	%rdi
140a6a3b0: 53                          	pushq	%rbx
140a6a3b1: 48 83 ec 38                 	subq	$0x38, %rsp
140a6a3b5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a6a3bc: 48 83 7d e8 00              	cmpq	$0x0, -0x18(%rbp)
140a6a3c1: 74 20                       	je	0x140a6a3e3 <.text+0xa693e3>
140a6a3c3: 48 8d 45 f0                 	leaq	-0x10(%rbp), %rax
140a6a3c7: 48 89 45 c8                 	movq	%rax, -0x38(%rbp)
140a6a3cb: 48 8d 45 c8                 	leaq	-0x38(%rbp), %rax
140a6a3cf: 48 89 45 c0                 	movq	%rax, -0x40(%rbp)
140a6a3d3: 48 8d 0d 1e a8 06 02        	leaq	0x206a81e(%rip), %rcx   # 0x142ad4bf8
140a6a3da: 48 8d 55 c0                 	leaq	-0x40(%rbp), %rdx
140a6a3de: e8 ed 4e 8c 00              	callq	0x14132f2d0 <.text+0x132e2d0>
140a6a3e3: 90                          	nop
140a6a3e4: 48 83 c4 38                 	addq	$0x38, %rsp
140a6a3e8: 5b                          	popq	%rbx
140a6a3e9: 5f                          	popq	%rdi
140a6a3ea: 5e                          	popq	%rsi
140a6a3eb: 41 5c                       	popq	%r12
140a6a3ed: 41 5d                       	popq	%r13
140a6a3ef: 41 5e                       	popq	%r14
140a6a3f1: 41 5f                       	popq	%r15
140a6a3f3: 5d                          	popq	%rbp
140a6a3f4: c3                          	retq
140a6a3f5: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
140a6a400: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a6a405: 55                          	pushq	%rbp
140a6a406: 41 57                       	pushq	%r15
140a6a408: 41 56                       	pushq	%r14
140a6a40a: 41 55                       	pushq	%r13
140a6a40c: 41 54                       	pushq	%r12
140a6a40e: 56                          	pushq	%rsi
140a6a40f: 57                          	pushq	%rdi
140a6a410: 53                          	pushq	%rbx
140a6a411: 48 83 ec 38                 	subq	$0x38, %rsp
140a6a415: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a6a41c: 48 8b 45 20                 	movq	0x20(%rbp), %rax
140a6a420: 48 ff 00                    	incq	(%rax)
140a6a423: 48 83 c4 38                 	addq	$0x38, %rsp
140a6a427: 5b                          	popq	%rbx
140a6a428: 5f                          	popq	%rdi
140a6a429: 5e                          	popq	%rsi
140a6a42a: 41 5c                       	popq	%r12
140a6a42c: 41 5d                       	popq	%r13
140a6a42e: 41 5e                       	popq	%r14
140a6a430: 41 5f                       	popq	%r15
140a6a432: 5d                          	popq	%rbp
140a6a433: c3                          	retq
140a6a434: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
140a6a440: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a6a445: 55                          	pushq	%rbp
140a6a446: 41 57                       	pushq	%r15
140a6a448: 41 56                       	pushq	%r14
140a6a44a: 41 55                       	pushq	%r13
140a6a44c: 41 54                       	pushq	%r12
140a6a44e: 56                          	pushq	%rsi
140a6a44f: 57                          	pushq	%rdi
140a6a450: 53                          	pushq	%rbx
140a6a451: 48 83 ec 38                 	subq	$0x38, %rsp
140a6a455: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a6a45c: 48 8b 45 20                 	movq	0x20(%rbp), %rax
140a6a460: 48 ff 00                    	incq	(%rax)
140a6a463: 48 83 c4 38                 	addq	$0x38, %rsp
140a6a467: 5b                          	popq	%rbx
140a6a468: 5f                          	popq	%rdi
140a6a469: 5e                          	popq	%rsi
140a6a46a: 41 5c                       	popq	%r12
140a6a46c: 41 5d                       	popq	%r13
140a6a46e: 41 5e                       	popq	%r14
140a6a470: 41 5f                       	popq	%r15
140a6a472: 5d                          	popq	%rbp
140a6a473: c3                          	retq

D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140929160: 55                          	pushq	%rbp
140929161: 41 57                       	pushq	%r15
140929163: 41 56                       	pushq	%r14
140929165: 41 55                       	pushq	%r13
140929167: 41 54                       	pushq	%r12
140929169: 56                          	pushq	%rsi
14092916a: 57                          	pushq	%rdi
14092916b: 53                          	pushq	%rbx
14092916c: 48 81 ec 18 01 00 00        	subq	$0x118, %rsp            # imm = 0x118
140929173: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
14092917b: 66 0f 7f b5 80 00 00 00     	movdqa	%xmm6, 0x80(%rbp)
140929183: 48 c7 45 78 fe ff ff ff     	movq	$-0x2, 0x78(%rbp)
14092918b: 4c 89 c3                    	movq	%r8, %rbx
14092918e: 49 89 d6                    	movq	%rdx, %r14
140929191: 48 89 cf                    	movq	%rcx, %rdi
140929194: e8 67 d8 2e 00              	callq	0x140c16a00 <.text+0xc15a00>
140929199: 84 c0                       	testb	%al, %al
14092919b: 88 45 77                    	movb	%al, 0x77(%rbp)
14092919e: 74 5f                       	je	0x1409291ff <.text+0x9281ff>
1409291a0: 4c 8d 3d 61 54 18 02        	leaq	0x2185461(%rip), %r15   # 0x142aae608
1409291a7: be 5a 00 00 00              	movl	$0x5a, %esi
1409291ac: ba 5a 00 00 00              	movl	$0x5a, %edx
1409291b1: 4c 89 f9                    	movq	%r15, %rcx
1409291b4: e8 67 a7 ff ff              	callq	0x140923920 <.text+0x922920>
1409291b9: 48 83 f8 01                 	cmpq	$0x1, %rax
1409291bd: 0f 85 b1 00 00 00           	jne	0x140929274 <.text+0x928274>
1409291c3: 49 89 d1                    	movq	%rdx, %r9
1409291c6: 48 85 d2                    	testq	%rdx, %rdx
1409291c9: 74 3d                       	je	0x140929208 <.text+0x928208>
1409291cb: 49 83 f9 5a                 	cmpq	$0x5a, %r9
1409291cf: 73 35                       	jae	0x140929206 <.text+0x928206>
1409291d1: 48 8d 05 30 54 18 02        	leaq	0x2185430(%rip), %rax   # 0x142aae608
1409291d8: 41 80 3c 01 bf              	cmpb	$-0x41, (%r9,%rax)
1409291dd: 7f 29                       	jg	0x140929208 <.text+0x928208>
1409291df: 48 8d 05 52 55 18 02        	leaq	0x2185552(%rip), %rax   # 0x142aae738
1409291e6: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1409291eb: 48 8d 0d 16 54 18 02        	leaq	0x2185416(%rip), %rcx   # 0x142aae608
1409291f2: ba 5a 00 00 00              	movl	$0x5a, %edx
1409291f7: 45 31 c0                    	xorl	%r8d, %r8d
1409291fa: e8 41 07 fb 01              	callq	0x1428d9940 <.text+0x28d8940>
1409291ff: 31 c0                       	xorl	%eax, %eax
140929201: e9 d4 01 00 00              	jmp	0x1409293da <.text+0x9283da>
140929206: 75 d7                       	jne	0x1409291df <.text+0x9281df>
140929208: 4c 8d 3d f9 53 18 02        	leaq	0x21853f9(%rip), %r15   # 0x142aae608
14092920f: 4c 89 f9                    	movq	%r15, %rcx
140929212: 4c 89 ca                    	movq	%r9, %rdx
140929215: e8 06 a7 ff ff              	callq	0x140923920 <.text+0x922920>
14092921a: 48 83 f8 01                 	cmpq	$0x1, %rax
14092921e: 75 54                       	jne	0x140929274 <.text+0x928274>
140929220: 49 89 d0                    	movq	%rdx, %r8
140929223: 49 83 c0 02                 	addq	$0x2, %r8
140929227: 74 39                       	je	0x140929262 <.text+0x928262>
140929229: 49 83 f8 5a                 	cmpq	$0x5a, %r8
14092922d: 73 31                       	jae	0x140929260 <.text+0x928260>
14092922f: 48 8d 05 d2 53 18 02        	leaq	0x21853d2(%rip), %rax   # 0x142aae608
140929236: 41 80 3c 00 bf              	cmpb	$-0x41, (%r8,%rax)
14092923b: 7f 25                       	jg	0x140929262 <.text+0x928262>
14092923d: 48 8d 05 0c 55 18 02        	leaq	0x218550c(%rip), %rax   # 0x142aae750
140929244: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140929249: 48 8d 0d b8 53 18 02        	leaq	0x21853b8(%rip), %rcx   # 0x142aae608
140929250: ba 5a 00 00 00              	movl	$0x5a, %edx
140929255: 41 b9 5a 00 00 00           	movl	$0x5a, %r9d
14092925b: e8 e0 06 fb 01              	callq	0x1428d9940 <.text+0x28d8940>
140929260: 75 db                       	jne	0x14092923d <.text+0x92823d>
140929262: be 58 00 00 00              	movl	$0x58, %esi
140929267: 48 29 d6                    	subq	%rdx, %rsi
14092926a: 4c 8d 3d 97 53 18 02        	leaq	0x2185397(%rip), %r15   # 0x142aae608
140929271: 4d 01 c7                    	addq	%r8, %r15
140929274: 48 8d 05 0b 63 18 02        	leaq	0x218630b(%rip), %rax   # 0x142aaf586
14092927b: 4c 8d 25 ba 62 18 02        	leaq	0x21862ba(%rip), %r12   # 0x142aaf53c
140929282: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
140929290: 0f be 48 ff                 	movsbl	-0x1(%rax), %ecx
140929294: 85 c9                       	testl	%ecx, %ecx
140929296: 78 18                       	js	0x1409292b0 <.text+0x9282b0>
140929298: 48 ff c8                    	decq	%rax
14092929b: 83 f9 5c                    	cmpl	$0x5c, %ecx
14092929e: 75 69                       	jne	0x140929309 <.text+0x928309>
1409292a0: eb 7d                       	jmp	0x14092931f <.text+0x92831f>
1409292a2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
1409292b0: 0f b6 50 fe                 	movzbl	-0x2(%rax), %edx
1409292b4: 80 fa c0                    	cmpb	$-0x40, %dl
1409292b7: 7d 25                       	jge	0x1409292de <.text+0x9282de>
1409292b9: 44 0f b6 40 fd              	movzbl	-0x3(%rax), %r8d
1409292be: 41 80 f8 c0                 	cmpb	$-0x40, %r8b
1409292c2: 7d 23                       	jge	0x1409292e7 <.text+0x9282e7>
1409292c4: 44 0f b6 48 fc              	movzbl	-0x4(%rax), %r9d
1409292c9: 48 83 c0 fc                 	addq	$-0x4, %rax
1409292cd: 41 83 e1 07                 	andl	$0x7, %r9d
1409292d1: 41 c1 e1 06                 	shll	$0x6, %r9d
1409292d5: 41 83 e0 3f                 	andl	$0x3f, %r8d
1409292d9: 45 09 c8                    	orl	%r9d, %r8d
1409292dc: eb 11                       	jmp	0x1409292ef <.text+0x9282ef>
1409292de: 48 83 c0 fe                 	addq	$-0x2, %rax
1409292e2: 83 e2 1f                    	andl	$0x1f, %edx
1409292e5: eb 12                       	jmp	0x1409292f9 <.text+0x9282f9>
1409292e7: 48 83 c0 fd                 	addq	$-0x3, %rax
1409292eb: 41 83 e0 0f                 	andl	$0xf, %r8d
1409292ef: 41 c1 e0 06                 	shll	$0x6, %r8d
1409292f3: 83 e2 3f                    	andl	$0x3f, %edx
1409292f6: 44 09 c2                    	orl	%r8d, %edx
1409292f9: c1 e2 06                    	shll	$0x6, %edx
1409292fc: 80 e1 3f                    	andb	$0x3f, %cl
1409292ff: 0f b6 c9                    	movzbl	%cl, %ecx
140929302: 09 d1                       	orl	%edx, %ecx
140929304: 83 f9 5c                    	cmpl	$0x5c, %ecx
140929307: 74 16                       	je	0x14092931f <.text+0x92831f>
140929309: 83 f9 2f                    	cmpl	$0x2f, %ecx
14092930c: 74 11                       	je	0x14092931f <.text+0x92831f>
14092930e: 4c 39 e0                    	cmpq	%r12, %rax
140929311: 0f 85 79 ff ff ff           	jne	0x140929290 <.text+0x928290>
140929317: 41 bd 4a 00 00 00           	movl	$0x4a, %r13d
14092931d: eb 50                       	jmp	0x14092936f <.text+0x92836f>
14092931f: 4c 8d 25 16 62 18 02        	leaq	0x2186216(%rip), %r12   # 0x142aaf53c
140929326: 4c 29 e0                    	subq	%r12, %rax
140929329: 49 89 c0                    	movq	%rax, %r8
14092932c: 49 ff c0                    	incq	%r8
14092932f: 74 32                       	je	0x140929363 <.text+0x928363>
140929331: 49 83 f8 4a                 	cmpq	$0x4a, %r8
140929335: 73 2a                       	jae	0x140929361 <.text+0x928361>
140929337: 43 80 3c 20 bf              	cmpb	$-0x41, (%r8,%r12)
14092933c: 7f 25                       	jg	0x140929363 <.text+0x928363>
14092933e: 48 8d 05 d3 53 18 02        	leaq	0x21853d3(%rip), %rax   # 0x142aae718
140929345: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14092934a: 48 8d 0d eb 61 18 02        	leaq	0x21861eb(%rip), %rcx   # 0x142aaf53c
140929351: ba 4a 00 00 00              	movl	$0x4a, %edx
140929356: 41 b9 4a 00 00 00           	movl	$0x4a, %r9d
14092935c: e8 df 05 fb 01              	callq	0x1428d9940 <.text+0x28d8940>
140929361: 75 db                       	jne	0x14092933e <.text+0x92833e>
140929363: 41 bd 49 00 00 00           	movl	$0x49, %r13d
140929369: 49 29 c5                    	subq	%rax, %r13
14092936c: 4d 01 c4                    	addq	%r8, %r12
14092936f: e8 bc ed 2e 00              	callq	0x140c18130 <.text+0xc17130>
140929374: 48 89 c1                    	movq	%rax, %rcx
140929377: 48 8b 00                    	movq	(%rax), %rax
14092937a: 48 83 f8 01                 	cmpq	$0x1, %rax
14092937e: 0f 85 87 05 00 00           	jne	0x14092990b <.text+0x92890b>
140929384: 48 83 c1 08                 	addq	$0x8, %rcx
140929388: 48 83 39 00                 	cmpq	$0x0, (%rcx)
14092938c: 0f 85 b3 05 00 00           	jne	0x140929945 <.text+0x928945>
140929392: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
140929399: 48 89 4d 60                 	movq	%rcx, 0x60(%rbp)
14092939d: 48 83 c1 08                 	addq	$0x8, %rcx
1409293a1: 4c 89 6c 24 20              	movq	%r13, 0x20(%rsp)
1409293a6: 48 c7 44 24 30 00 00 00 00  	movq	$0x0, 0x30(%rsp)
1409293af: 48 c7 44 24 28 01 00 00 00  	movq	$0x1, 0x28(%rsp)
1409293b8: 4c 89 fa                    	movq	%r15, %rdx
1409293bb: 49 89 f0                    	movq	%rsi, %r8
1409293be: 4d 89 e1                    	movq	%r12, %r9
1409293c1: e8 7a d9 2e 00              	callq	0x140c16d40 <.text+0xc15d40>
1409293c6: 48 8b 4d 60                 	movq	0x60(%rbp), %rcx
1409293ca: 48 ff 01                    	incq	(%rcx)
1409293cd: 48 89 45 30                 	movq	%rax, 0x30(%rbp)
1409293d1: 48 89 45 d8                 	movq	%rax, -0x28(%rbp)
1409293d5: b8 01 00 00 00              	movl	$0x1, %eax
1409293da: 48 89 45 d0                 	movq	%rax, -0x30(%rbp)
1409293de: 31 c9                       	xorl	%ecx, %ecx
1409293e0: e8 3b 07 7b ff              	callq	0x1400d9b20 <.text+0xd8b20>
1409293e5: 48 85 c0                    	testq	%rax, %rax
1409293e8: 0f 84 63 05 00 00           	je	0x140929951 <.text+0x928951>
1409293ee: 0f 10 00                    	movups	(%rax), %xmm0
1409293f1: 48 ff 00                    	incq	(%rax)
1409293f4: 0f 29 45 00                 	movaps	%xmm0, (%rbp)
1409293f8: 0f 10 05 11 2b 34 02        	movups	0x2342b11(%rip), %xmm0  # 0x142c6bf10
1409293ff: 0f 29 45 f0                 	movaps	%xmm0, -0x10(%rbp)
140929403: f3 0f 6f 05 f5 2a 34 02     	movdqu	0x2342af5(%rip), %xmm0  # 0x142c6bf00
14092940b: 66 0f 7f 45 e0              	movdqa	%xmm0, -0x20(%rbp)
140929410: 48 8b 37                    	movq	(%rdi), %rsi
140929413: 80 7e 18 00                 	cmpb	$0x0, 0x18(%rsi)
140929417: 74 59                       	je	0x140929472 <.text+0x928472>
140929419: 48 8b 46 20                 	movq	0x20(%rsi), %rax
14092941d: 4c 8b 60 18                 	movq	0x18(%rax), %r12
140929421: 48 8b 40 20                 	movq	0x20(%rax), %rax
140929425: 48 8d 04 80                 	leaq	(%rax,%rax,4), %rax
140929429: 4d 8d 2c c4                 	leaq	(%r12,%rax,8), %r13
14092942d: 48 8d 7d b8                 	leaq	-0x48(%rbp), %rdi
140929431: 4c 8d 7d e0                 	leaq	-0x20(%rbp), %r15
140929435: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
140929440: 4d 39 ec                    	cmpq	%r13, %r12
140929443: 74 2d                       	je	0x140929472 <.text+0x928472>
140929445: 48 8b 46 38                 	movq	0x38(%rsi), %rax
140929449: f3 41 0f 6f 44 24 08        	movdqu	0x8(%r12), %xmm0
140929450: 48 89 45 b8                 	movq	%rax, -0x48(%rbp)
140929454: f3 0f 7f 45 c0              	movdqu	%xmm0, -0x40(%rbp)
140929459: 48 89 f9                    	movq	%rdi, %rcx
14092945c: e8 af 12 b3 00              	callq	0x14145a710 <.text+0x1459710>
140929461: 49 83 c4 28                 	addq	$0x28, %r12
140929465: 4c 89 f9                    	movq	%r15, %rcx
140929468: 48 89 c2                    	movq	%rax, %rdx
14092946b: e8 80 0e 9a ff              	callq	0x1402ca2f0 <.text+0x2c92f0>
140929470: eb ce                       	jmp	0x140929440 <.text+0x928440>
140929472: 49 8b 0e                    	movq	(%r14), %rcx
140929475: 4c 8b a1 f8 00 00 00        	movq	0xf8(%rcx), %r12
14092947c: 48 8b 81 00 01 00 00        	movq	0x100(%rcx), %rax
140929483: 48 8b 53 08                 	movq	0x8(%rbx), %rdx
140929487: 48 85 d2                    	testq	%rdx, %rdx
14092948a: 75 04                       	jne	0x140929490 <.text+0x928490>
14092948c: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140929490: 48 89 55 28                 	movq	%rdx, 0x28(%rbp)
140929494: 4d 8b 6e 08                 	movq	0x8(%r14), %r13
140929498: 49 8d 34 84                 	leaq	(%r12,%rax,4), %rsi
14092949c: 48 89 4d 50                 	movq	%rcx, 0x50(%rbp)
1409294a0: 0f b6 81 08 01 00 00        	movzbl	0x108(%rcx), %eax
1409294a7: 88 45 60                    	movb	%al, 0x60(%rbp)
1409294aa: b8 08 00 00 00              	movl	$0x8, %eax
1409294af: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
1409294b3: 66 0f 76 f6                 	pcmpeqd	%xmm6, %xmm6
1409294b7: 48 89 45 48                 	movq	%rax, 0x48(%rbp)
1409294bb: 31 db                       	xorl	%ebx, %ebx
1409294bd: 45 31 ff                    	xorl	%r15d, %r15d
1409294c0: 48 c7 45 68 00 00 00 00     	movq	$0x0, 0x68(%rbp)
1409294c8: 48 c7 45 58 00 00 00 00     	movq	$0x0, 0x58(%rbp)
1409294d0: c7 45 70 00 00 00 00        	movl	$0x0, 0x70(%rbp)
1409294d7: e9 84 00 00 00              	jmp	0x140929560 <.text+0x928560>
1409294dc: 0f 1f 40 00                 	nopl	(%rax)
1409294e0: f3 42 0f 6f 0c 02           	movdqu	(%rdx,%r8), %xmm1
1409294e6: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
1409294ea: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
1409294ee: 66 44 0f d7 ca              	pmovmskb	%xmm2, %r9d
1409294f3: 45 85 c9                    	testl	%r9d, %r9d
1409294f6: 74 38                       	je	0x140929530 <.text+0x928530>
1409294f8: 4d 8b 16                    	movq	(%r14), %r10
1409294fb: 48 89 c8                    	movq	%rcx, %rax
1409294fe: f3 45 0f bc d9              	tzcntl	%r9d, %r11d
140929503: 4d 01 c3                    	addq	%r8, %r11
140929506: 4c 23 5d 38                 	andq	0x38(%rbp), %r11
14092950a: 49 c1 e3 03                 	shlq	$0x3, %r11
14092950e: 48 89 d1                    	movq	%rdx, %rcx
140929511: 4c 29 d9                    	subq	%r11, %rcx
140929514: 4c 3b 51 f8                 	cmpq	-0x8(%rcx), %r10
140929518: 74 46                       	je	0x140929560 <.text+0x928560>
14092951a: 41 8d 49 ff                 	leal	-0x1(%r9), %ecx
14092951e: 66 44 21 c9                 	andw	%r9w, %cx
140929522: 41 89 c9                    	movl	%ecx, %r9d
140929525: 48 89 c1                    	movq	%rax, %rcx
140929528: 75 d1                       	jne	0x1409294fb <.text+0x9284fb>
14092952a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140929530: 66 0f 74 ce                 	pcmpeqb	%xmm6, %xmm1
140929534: 66 0f d7 c1                 	pmovmskb	%xmm1, %eax
140929538: 85 c0                       	testl	%eax, %eax
14092953a: 0f 85 c0 02 00 00           	jne	0x140929800 <.text+0x928800>
140929540: 49 01 c8                    	addq	%rcx, %r8
140929543: 49 83 c0 10                 	addq	$0x10, %r8
140929547: 48 83 c1 10                 	addq	$0x10, %rcx
14092954b: 4c 23 45 38                 	andq	0x38(%rbp), %r8
14092954f: eb 8f                       	jmp	0x1409294e0 <.text+0x9284e0>
140929551: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
140929560: 80 7d 60 00                 	cmpb	$0x0, 0x60(%rbp)
140929564: 0f 84 96 00 00 00           	je	0x140929600 <.text+0x928600>
14092956a: 49 39 df                    	cmpq	%rbx, %r15
14092956d: 0f 85 dd 01 00 00           	jne	0x140929750 <.text+0x928750>
140929573: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
140929580: 49 39 f4                    	cmpq	%rsi, %r12
140929583: 0f 84 f7 02 00 00           	je	0x140929880 <.text+0x928880>
140929589: 41 8b 0c 24                 	movl	(%r12), %ecx
14092958d: 49 83 c4 04                 	addq	$0x4, %r12
140929591: 49 8b 85 a8 01 00 00        	movq	0x1a8(%r13), %rax
140929598: 48 8d 0c c9                 	leaq	(%rcx,%rcx,8), %rcx
14092959c: 48 8b 5c c8 10              	movq	0x10(%rax,%rcx,8), %rbx
1409295a1: 48 85 db                    	testq	%rbx, %rbx
1409295a4: 74 da                       	je	0x140929580 <.text+0x928580>
1409295a6: 48 8d 04 c8                 	leaq	(%rax,%rcx,8), %rax
1409295aa: 4c 8b 4d 50                 	movq	0x50(%rbp), %r9
1409295ae: 49 8b 91 10 01 00 00        	movq	0x110(%r9), %rdx
1409295b5: 48 8b 48 40                 	movq	0x40(%rax), %rcx
1409295b9: 48 39 d1                    	cmpq	%rdx, %rcx
1409295bc: 0f 86 df 00 00 00           	jbe	0x1409296a1 <.text+0x9286a1>
1409295c2: 4c 8b 40 38                 	movq	0x38(%rax), %r8
1409295c6: 49 8b 14 d0                 	movq	(%r8,%rdx,8), %rdx
1409295ca: 48 85 d2                    	testq	%rdx, %rdx
1409295cd: 0f 84 ce 00 00 00           	je	0x1409296a1 <.text+0x9286a1>
1409295d3: 4c 8b 40 18                 	movq	0x18(%rax), %r8
1409295d7: 48 f7 d2                    	notq	%rdx
1409295da: 48 8d 14 52                 	leaq	(%rdx,%rdx,2), %rdx
1409295de: 48 c1 e2 04                 	shlq	$0x4, %rdx
1409295e2: 49 8b 54 10 10              	movq	0x10(%r8,%rdx), %rdx
1409295e7: 48 89 55 68                 	movq	%rdx, 0x68(%rbp)
1409295eb: 49 8b 91 18 01 00 00        	movq	0x118(%r9), %rdx
1409295f2: 48 39 d1                    	cmpq	%rdx, %rcx
1409295f5: 0f 87 be 00 00 00           	ja	0x1409296b9 <.text+0x9286b9>
1409295fb: e9 3e 01 00 00              	jmp	0x14092973e <.text+0x92873e>
140929600: 49 39 df                    	cmpq	%rbx, %r15
140929603: 0f 85 70 01 00 00           	jne	0x140929779 <.text+0x928779>
140929609: 0f 1f 80 00 00 00 00        	nopl	(%rax)
140929610: 49 39 f4                    	cmpq	%rsi, %r12
140929613: 0f 84 67 02 00 00           	je	0x140929880 <.text+0x928880>
140929619: 41 8b 0c 24                 	movl	(%r12), %ecx
14092961d: 49 83 c4 04                 	addq	$0x4, %r12
140929621: 49 8b 85 00 01 00 00        	movq	0x100(%r13), %rax
140929628: 48 8d 0c 89                 	leaq	(%rcx,%rcx,4), %rcx
14092962c: 48 c1 e1 05                 	shlq	$0x5, %rcx
140929630: 48 8b 5c 08 58              	movq	0x58(%rax,%rcx), %rbx
140929635: 48 85 db                    	testq	%rbx, %rbx
140929638: 74 d6                       	je	0x140929610 <.text+0x928610>
14092963a: 48 01 c8                    	addq	%rcx, %rax
14092963d: 8b 88 94 00 00 00           	movl	0x94(%rax), %ecx
140929643: 49 8b 95 a8 01 00 00        	movq	0x1a8(%r13), %rdx
14092964a: 4c 8d 0c c9                 	leaq	(%rcx,%rcx,8), %r9
14092964e: 4a 8d 0c ca                 	leaq	(%rdx,%r9,8), %rcx
140929652: 4c 8b 55 50                 	movq	0x50(%rbp), %r10
140929656: 4d 8b 82 10 01 00 00        	movq	0x110(%r10), %r8
14092965d: 4a 8b 54 ca 40              	movq	0x40(%rdx,%r9,8), %rdx
140929662: 4c 39 c2                    	cmpq	%r8, %rdx
140929665: 76 7e                       	jbe	0x1409296e5 <.text+0x9286e5>
140929667: 4c 8b 49 38                 	movq	0x38(%rcx), %r9
14092966b: 4f 8b 04 c1                 	movq	(%r9,%r8,8), %r8
14092966f: 4d 85 c0                    	testq	%r8, %r8
140929672: 74 71                       	je	0x1409296e5 <.text+0x9286e5>
140929674: 4c 8b 49 18                 	movq	0x18(%rcx), %r9
140929678: 49 f7 d0                    	notq	%r8
14092967b: 4f 8d 04 40                 	leaq	(%r8,%r8,2), %r8
14092967f: 49 c1 e0 04                 	shlq	$0x4, %r8
140929683: 4f 8b 44 01 10              	movq	0x10(%r9,%r8), %r8
140929688: 4c 89 45 68                 	movq	%r8, 0x68(%rbp)
14092968c: 4d 8b 82 18 01 00 00        	movq	0x118(%r10), %r8
140929693: 4c 39 80 88 00 00 00        	cmpq	%r8, 0x88(%rax)
14092969a: 77 61                       	ja	0x1409296fd <.text+0x9286fd>
14092969c: e9 bc 00 00 00              	jmp	0x14092975d <.text+0x92875d>
1409296a1: 48 c7 45 68 00 00 00 00     	movq	$0x0, 0x68(%rbp)
1409296a9: 49 8b 91 18 01 00 00        	movq	0x118(%r9), %rdx
1409296b0: 48 39 d1                    	cmpq	%rdx, %rcx
1409296b3: 0f 86 85 00 00 00           	jbe	0x14092973e <.text+0x92873e>
1409296b9: 48 8b 48 38                 	movq	0x38(%rax), %rcx
1409296bd: 48 8b 0c d1                 	movq	(%rcx,%rdx,8), %rcx
1409296c1: 48 85 c9                    	testq	%rcx, %rcx
1409296c4: 74 78                       	je	0x14092973e <.text+0x92873e>
1409296c6: 48 8b 50 18                 	movq	0x18(%rax), %rdx
1409296ca: 48 f7 d1                    	notq	%rcx
1409296cd: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
1409296d1: 48 c1 e1 04                 	shlq	$0x4, %rcx
1409296d5: 48 8b 4c 0a 10              	movq	0x10(%rdx,%rcx), %rcx
1409296da: 48 89 4d 58                 	movq	%rcx, 0x58(%rbp)
1409296de: b1 01                       	movb	$0x1, %cl
1409296e0: 89 4d 70                    	movl	%ecx, 0x70(%rbp)
1409296e3: eb 60                       	jmp	0x140929745 <.text+0x928745>
1409296e5: 48 c7 45 68 00 00 00 00     	movq	$0x0, 0x68(%rbp)
1409296ed: 4d 8b 82 18 01 00 00        	movq	0x118(%r10), %r8
1409296f4: 4c 39 80 88 00 00 00        	cmpq	%r8, 0x88(%rax)
1409296fb: 76 60                       	jbe	0x14092975d <.text+0x92875d>
1409296fd: 4c 8b 88 80 00 00 00        	movq	0x80(%rax), %r9
140929704: 4b 83 3c c1 00              	cmpq	$0x0, (%r9,%r8,8)
140929709: 74 52                       	je	0x14092975d <.text+0x92875d>
14092970b: 41 b1 01                    	movb	$0x1, %r9b
14092970e: 44 89 4d 70                 	movl	%r9d, 0x70(%rbp)
140929712: 4c 39 c2                    	cmpq	%r8, %rdx
140929715: 76 4f                       	jbe	0x140929766 <.text+0x928766>
140929717: 48 8b 51 38                 	movq	0x38(%rcx), %rdx
14092971b: 4a 8b 14 c2                 	movq	(%rdx,%r8,8), %rdx
14092971f: 48 85 d2                    	testq	%rdx, %rdx
140929722: 74 42                       	je	0x140929766 <.text+0x928766>
140929724: 48 8b 49 18                 	movq	0x18(%rcx), %rcx
140929728: 48 f7 d2                    	notq	%rdx
14092972b: 48 8d 14 52                 	leaq	(%rdx,%rdx,2), %rdx
14092972f: 48 c1 e2 04                 	shlq	$0x4, %rdx
140929733: 48 8b 4c 11 10              	movq	0x10(%rcx,%rdx), %rcx
140929738: 48 89 4d 58                 	movq	%rcx, 0x58(%rbp)
14092973c: eb 30                       	jmp	0x14092976e <.text+0x92876e>
14092973e: c7 45 70 00 00 00 00        	movl	$0x0, 0x70(%rbp)
140929745: 48 8b 40 08                 	movq	0x8(%rax), %rax
140929749: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
14092974d: 45 31 ff                    	xorl	%r15d, %r15d
140929750: 48 8b 45 40                 	movq	0x40(%rbp), %rax
140929754: 4a 8d 04 f8                 	leaq	(%rax,%r15,8), %rax
140929758: 45 89 fe                    	movl	%r15d, %r14d
14092975b: eb 30                       	jmp	0x14092978d <.text+0x92878d>
14092975d: c7 45 70 00 00 00 00        	movl	$0x0, 0x70(%rbp)
140929764: eb 08                       	jmp	0x14092976e <.text+0x92876e>
140929766: 48 c7 45 58 00 00 00 00     	movq	$0x0, 0x58(%rbp)
14092976e: 48 8b 40 50                 	movq	0x50(%rax), %rax
140929772: 48 89 45 48                 	movq	%rax, 0x48(%rbp)
140929776: 45 31 ff                    	xorl	%r15d, %r15d
140929779: 4c 89 f9                    	movq	%r15, %rcx
14092977c: 48 c1 e1 04                 	shlq	$0x4, %rcx
140929780: 48 8b 55 48                 	movq	0x48(%rbp), %rdx
140929784: 48 8d 04 0a                 	leaq	(%rdx,%rcx), %rax
140929788: 44 8b 74 0a 08              	movl	0x8(%rdx,%rcx), %r14d
14092978d: 48 8b 4d 58                 	movq	0x58(%rbp), %rcx
140929791: 4a 8d 3c f1                 	leaq	(%rcx,%r14,8), %rdi
140929795: 49 c1 e6 04                 	shlq	$0x4, %r14
140929799: 4c 03 75 68                 	addq	0x68(%rbp), %r14
14092979d: f6 45 70 01                 	testb	$0x1, 0x70(%rbp)
1409297a1: b9 00 00 00 00              	movl	$0x0, %ecx
1409297a6: 48 0f 44 f9                 	cmoveq	%rcx, %rdi
1409297aa: 48 8b 00                    	movq	(%rax), %rax
1409297ad: 48 89 45 20                 	movq	%rax, 0x20(%rbp)
1409297b1: 49 ff c7                    	incq	%r15
1409297b4: 48 83 7d f8 00              	cmpq	$0x0, -0x8(%rbp)
1409297b9: 74 45                       	je	0x140929800 <.text+0x928800>
1409297bb: 48 8d 4d 00                 	leaq	(%rbp), %rcx
1409297bf: 4c 89 f2                    	movq	%r14, %rdx
1409297c2: e8 a9 75 c8 ff              	callq	0x1405b0d70 <.text+0x5afd70>
1409297c7: 48 8b 55 e0                 	movq	-0x20(%rbp), %rdx
1409297cb: 4c 8b 45 e8                 	movq	-0x18(%rbp), %r8
1409297cf: 4c 89 45 38                 	movq	%r8, 0x38(%rbp)
1409297d3: 49 21 c0                    	andq	%rax, %r8
1409297d6: 48 c1 e8 39                 	shrq	$0x39, %rax
1409297da: 66 0f 6e c0                 	movd	%eax, %xmm0
1409297de: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1409297e2: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1409297e7: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1409297ec: 31 c9                       	xorl	%ecx, %ecx
1409297ee: e9 ed fc ff ff              	jmp	0x1409294e0 <.text+0x9284e0>
1409297f3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
140929800: 48 85 ff                    	testq	%rdi, %rdi
140929803: 74 0e                       	je	0x140929813 <.text+0x928813>
140929805: f3 0f 10 47 04              	movss	0x4(%rdi), %xmm0
14092980a: 0f 2e 07                    	ucomiss	(%rdi), %xmm0
14092980d: 0f 87 4d fd ff ff           	ja	0x140929560 <.text+0x928560>
140929813: 48 8b 7d 28                 	movq	0x28(%rbp), %rdi
140929817: 48 8b 07                    	movq	(%rdi), %rax
14092981a: 4c 8b 77 10                 	movq	0x10(%rdi), %r14
14092981e: 4c 29 f0                    	subq	%r14, %rax
140929821: 48 83 f8 1f                 	cmpq	$0x1f, %rax
140929825: 76 37                       	jbe	0x14092985e <.text+0x92885e>
140929827: 48 8b 47 08                 	movq	0x8(%rdi), %rax
14092982b: 48 8d 0d fe 34 9b 00        	leaq	0x9b34fe(%rip), %rcx    # 0x1412dcd30 <.text+0x12dbd30>
140929832: 4a 89 0c 30                 	movq	%rcx, (%rax,%r14)
140929836: 48 8d 0d f3 b4 c3 00        	leaq	0xc3b4f3(%rip), %rcx    # 0x141564d30 <.text+0x1563d30>
14092983d: 4a 89 4c 30 08              	movq	%rcx, 0x8(%rax,%r14)
140929842: 48 8b 4d 20                 	movq	0x20(%rbp), %rcx
140929846: 4a 89 4c 30 10              	movq	%rcx, 0x10(%rax,%r14)
14092984b: 42 c6 44 30 18 00           	movb	$0x0, 0x18(%rax,%r14)
140929851: 49 83 c6 20                 	addq	$0x20, %r14
140929855: 4c 89 77 10                 	movq	%r14, 0x10(%rdi)
140929859: e9 02 fd ff ff              	jmp	0x140929560 <.text+0x928560>
14092985e: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140929867: 41 b8 20 00 00 00           	movl	$0x20, %r8d
14092986d: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140929873: 48 89 f9                    	movq	%rdi, %rcx
140929876: 4c 89 f2                    	movq	%r14, %rdx
140929879: e8 e2 01 f8 01              	callq	0x1428a9a60 <.text+0x28a8a60>
14092987e: eb a7                       	jmp	0x140929827 <.text+0x928827>
140929880: 48 8b 55 e8                 	movq	-0x18(%rbp), %rdx
140929884: 48 85 d2                    	testq	%rdx, %rdx
140929887: 74 27                       	je	0x1409298b0 <.text+0x9288b0>
140929889: 48 8d 04 d5 17 00 00 00     	leaq	0x17(,%rdx,8), %rax
140929891: 48 83 e0 f0                 	andq	$-0x10, %rax
140929895: 48 01 c2                    	addq	%rax, %rdx
140929898: 48 83 c2 11                 	addq	$0x11, %rdx
14092989c: 74 12                       	je	0x1409298b0 <.text+0x9288b0>
14092989e: 48 8b 4d e0                 	movq	-0x20(%rbp), %rcx
1409298a2: 48 29 c1                    	subq	%rax, %rcx
1409298a5: 41 b8 10 00 00 00           	movl	$0x10, %r8d
1409298ab: e8 70 a3 ce ff              	callq	0x140613c20 <.text+0x612c20>
1409298b0: 80 7d 77 00                 	cmpb	$0x0, 0x77(%rbp)
1409298b4: 74 3a                       	je	0x1409298f0 <.text+0x9288f0>
1409298b6: e8 75 e8 2e 00              	callq	0x140c18130 <.text+0xc17130>
1409298bb: 48 89 c1                    	movq	%rax, %rcx
1409298be: 48 8b 00                    	movq	(%rax), %rax
1409298c1: 48 83 f8 01                 	cmpq	$0x1, %rax
1409298c5: 75 5e                       	jne	0x140929925 <.text+0x928925>
1409298c7: 48 83 c1 08                 	addq	$0x8, %rcx
1409298cb: 48 83 39 00                 	cmpq	$0x0, (%rcx)
1409298cf: 75 74                       	jne	0x140929945 <.text+0x928945>
1409298d1: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
1409298d8: 48 89 4d 60                 	movq	%rcx, 0x60(%rbp)
1409298dc: 48 83 c1 08                 	addq	$0x8, %rcx
1409298e0: 48 8b 55 30                 	movq	0x30(%rbp), %rdx
1409298e4: e8 07 d8 2e 00              	callq	0x140c170f0 <.text+0xc160f0>
1409298e9: 48 8b 45 60                 	movq	0x60(%rbp), %rax
1409298ed: 48 ff 00                    	incq	(%rax)
1409298f0: 0f 28 b5 80 00 00 00        	movaps	0x80(%rbp), %xmm6
1409298f7: 48 81 c4 18 01 00 00        	addq	$0x118, %rsp            # imm = 0x118
1409298fe: 5b                          	popq	%rbx
1409298ff: 5f                          	popq	%rdi
140929900: 5e                          	popq	%rsi
140929901: 41 5c                       	popq	%r12
140929903: 41 5d                       	popq	%r13
140929905: 41 5e                       	popq	%r14
140929907: 41 5f                       	popq	%r15
140929909: 5d                          	popq	%rbp
14092990a: c3                          	retq
14092990b: 48 85 c0                    	testq	%rax, %rax
14092990e: 75 29                       	jne	0x140929939 <.text+0x928939>
140929910: 31 d2                       	xorl	%edx, %edx
140929912: e8 b9 ee fc 01              	callq	0x1428f87d0 <.text+0x28f77d0>
140929917: 48 89 c1                    	movq	%rax, %rcx
14092991a: 48 85 c0                    	testq	%rax, %rax
14092991d: 0f 85 65 fa ff ff           	jne	0x140929388 <.text+0x928388>
140929923: eb 14                       	jmp	0x140929939 <.text+0x928939>
140929925: 48 85 c0                    	testq	%rax, %rax
140929928: 75 0f                       	jne	0x140929939 <.text+0x928939>
14092992a: 31 d2                       	xorl	%edx, %edx
14092992c: e8 9f ee fc 01              	callq	0x1428f87d0 <.text+0x28f77d0>
140929931: 48 89 c1                    	movq	%rax, %rcx
140929934: 48 85 c0                    	testq	%rax, %rax
140929937: 75 92                       	jne	0x1409298cb <.text+0x9288cb>
140929939: 48 8d 0d 60 11 31 02        	leaq	0x2311160(%rip), %rcx   # 0x142c3aaa0
140929940: e8 8b e7 fa 01              	callq	0x1428d80d0 <.text+0x28d70d0>
140929945: 48 8d 0d 24 12 31 02        	leaq	0x2311224(%rip), %rcx   # 0x142c3ab70
14092994c: e8 ff f8 fa 01              	callq	0x1428d9250 <.text+0x28d8250>
140929951: 48 8d 0d 48 11 31 02        	leaq	0x2311148(%rip), %rcx   # 0x142c3aaa0
140929958: e8 73 e7 fa 01              	callq	0x1428d80d0 <.text+0x28d70d0>
14092995d: 0f 0b                       	ud2
14092995f: 90                          	nop
140929960: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140929965: 55                          	pushq	%rbp
140929966: 41 57                       	pushq	%r15
140929968: 41 56                       	pushq	%r14
14092996a: 41 55                       	pushq	%r13
14092996c: 41 54                       	pushq	%r12
14092996e: 56                          	pushq	%rsi
14092996f: 57                          	pushq	%rdi
140929970: 53                          	pushq	%rbx
140929971: 48 83 ec 48                 	subq	$0x48, %rsp
140929975: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14092997c: 66 0f 7f 74 24 30           	movdqa	%xmm6, 0x30(%rsp)
140929982: 48 8b 45 60                 	movq	0x60(%rbp), %rax
140929986: 48 ff 00                    	incq	(%rax)
140929989: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
14092998e: 48 83 c4 48                 	addq	$0x48, %rsp
140929992: 5b                          	popq	%rbx
140929993: 5f                          	popq	%rdi
140929994: 5e                          	popq	%rsi
140929995: 41 5c                       	popq	%r12
140929997: 41 5d                       	popq	%r13
140929999: 41 5e                       	popq	%r14
14092999b: 41 5f                       	popq	%r15
14092999d: 5d                          	popq	%rbp
14092999e: c3                          	retq
14092999f: 90                          	nop
1409299a0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409299a5: 55                          	pushq	%rbp
1409299a6: 41 57                       	pushq	%r15
1409299a8: 41 56                       	pushq	%r14
1409299aa: 41 55                       	pushq	%r13
1409299ac: 41 54                       	pushq	%r12
1409299ae: 56                          	pushq	%rsi
1409299af: 57                          	pushq	%rdi
1409299b0: 53                          	pushq	%rbx
1409299b1: 48 83 ec 48                 	subq	$0x48, %rsp
1409299b5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409299bc: 66 0f 7f 74 24 30           	movdqa	%xmm6, 0x30(%rsp)
1409299c2: 48 8b 45 60                 	movq	0x60(%rbp), %rax
1409299c6: 48 ff 00                    	incq	(%rax)
1409299c9: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
1409299ce: 48 83 c4 48                 	addq	$0x48, %rsp
1409299d2: 5b                          	popq	%rbx
1409299d3: 5f                          	popq	%rdi
1409299d4: 5e                          	popq	%rsi
1409299d5: 41 5c                       	popq	%r12
1409299d7: 41 5d                       	popq	%r13
1409299d9: 41 5e                       	popq	%r14
1409299db: 41 5f                       	popq	%r15
1409299dd: 5d                          	popq	%rbp
1409299de: c3                          	retq
1409299df: 90                          	nop
1409299e0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409299e5: 55                          	pushq	%rbp
1409299e6: 41 57                       	pushq	%r15
1409299e8: 41 56                       	pushq	%r14
1409299ea: 41 55                       	pushq	%r13
1409299ec: 41 54                       	pushq	%r12
1409299ee: 56                          	pushq	%rsi
1409299ef: 57                          	pushq	%rdi
1409299f0: 53                          	pushq	%rbx
1409299f1: 48 83 ec 48                 	subq	$0x48, %rsp
1409299f5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409299fc: 66 0f 7f 74 24 30           	movdqa	%xmm6, 0x30(%rsp)
140929a02: 48 8d 4d e0                 	leaq	-0x20(%rbp), %rcx
140929a06: e8 45 05 82 ff              	callq	0x140149f50 <.text+0x148f50>
140929a0b: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
140929a10: 48 83 c4 48                 	addq	$0x48, %rsp
140929a14: 5b                          	popq	%rbx
140929a15: 5f                          	popq	%rdi
140929a16: 5e                          	popq	%rsi
140929a17: 41 5c                       	popq	%r12
140929a19: 41 5d                       	popq	%r13
140929a1b: 41 5e                       	popq	%r14
140929a1d: 41 5f                       	popq	%r15
140929a1f: 5d                          	popq	%rbp
140929a20: c3                          	retq
140929a21: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
140929a30: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140929a35: 55                          	pushq	%rbp
140929a36: 41 57                       	pushq	%r15
140929a38: 41 56                       	pushq	%r14
140929a3a: 41 55                       	pushq	%r13
140929a3c: 41 54                       	pushq	%r12
140929a3e: 56                          	pushq	%rsi
140929a3f: 57                          	pushq	%rdi
140929a40: 53                          	pushq	%rbx
140929a41: 48 83 ec 48                 	subq	$0x48, %rsp
140929a45: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140929a4c: 66 0f 7f 74 24 30           	movdqa	%xmm6, 0x30(%rsp)
140929a52: 80 7d 77 00                 	cmpb	$0x0, 0x77(%rbp)
140929a56: 74 20                       	je	0x140929a78 <.text+0x928a78>
140929a58: 48 8d 45 d8                 	leaq	-0x28(%rbp), %rax
140929a5c: 48 89 45 18                 	movq	%rax, 0x18(%rbp)
140929a60: 48 8d 45 18                 	leaq	0x18(%rbp), %rax
140929a64: 48 89 45 10                 	movq	%rax, 0x10(%rbp)
140929a68: 48 8d 0d 49 4c 18 02        	leaq	0x2184c49(%rip), %rcx   # 0x142aae6b8
140929a6f: 48 8d 55 10                 	leaq	0x10(%rbp), %rdx
140929a73: e8 58 58 a0 00              	callq	0x14132f2d0 <.text+0x132e2d0>
140929a78: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
140929a7d: 48 83 c4 48                 	addq	$0x48, %rsp
140929a81: 5b                          	popq	%rbx
140929a82: 5f                          	popq	%rdi
140929a83: 5e                          	popq	%rsi
140929a84: 41 5c                       	popq	%r12
140929a86: 41 5d                       	popq	%r13
140929a88: 41 5e                       	popq	%r14
140929a8a: 41 5f                       	popq	%r15
140929a8c: 5d                          	popq	%rbp
140929a8d: c3                          	retq

D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140929a90: 55                          	pushq	%rbp
140929a91: 41 57                       	pushq	%r15
140929a93: 41 56                       	pushq	%r14
140929a95: 41 55                       	pushq	%r13
140929a97: 41 54                       	pushq	%r12
140929a99: 56                          	pushq	%rsi
140929a9a: 57                          	pushq	%rdi
140929a9b: 53                          	pushq	%rbx
140929a9c: 48 81 ec f8 00 00 00        	subq	$0xf8, %rsp
140929aa3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140929aab: 0f 29 75 60                 	movaps	%xmm6, 0x60(%rbp)
140929aaf: 48 c7 45 58 fe ff ff ff     	movq	$-0x2, 0x58(%rbp)
140929ab7: 48 89 d7                    	movq	%rdx, %rdi
140929aba: 48 89 cb                    	movq	%rcx, %rbx
140929abd: e8 3e cf 2e 00              	callq	0x140c16a00 <.text+0xc15a00>
140929ac2: 84 c0                       	testb	%al, %al
140929ac4: 88 45 57                    	movb	%al, 0x57(%rbp)
140929ac7: 74 5f                       	je	0x140929b28 <.text+0x928b28>
140929ac9: 4c 8d 35 95 4b 18 02        	leaq	0x2184b95(%rip), %r14   # 0x142aae665
140929ad0: be 50 00 00 00              	movl	$0x50, %esi
140929ad5: ba 50 00 00 00              	movl	$0x50, %edx
140929ada: 4c 89 f1                    	movq	%r14, %rcx
140929add: e8 3e 9e ff ff              	callq	0x140923920 <.text+0x922920>
140929ae2: 48 83 f8 01                 	cmpq	$0x1, %rax
140929ae6: 0f 85 b1 00 00 00           	jne	0x140929b9d <.text+0x928b9d>
140929aec: 49 89 d1                    	movq	%rdx, %r9
140929aef: 48 85 d2                    	testq	%rdx, %rdx
140929af2: 74 3d                       	je	0x140929b31 <.text+0x928b31>
140929af4: 49 83 f9 50                 	cmpq	$0x50, %r9
140929af8: 73 35                       	jae	0x140929b2f <.text+0x928b2f>
140929afa: 48 8d 05 64 4b 18 02        	leaq	0x2184b64(%rip), %rax   # 0x142aae665
140929b01: 41 80 3c 01 bf              	cmpb	$-0x41, (%r9,%rax)
140929b06: 7f 29                       	jg	0x140929b31 <.text+0x928b31>
140929b08: 48 8d 05 29 4c 18 02        	leaq	0x2184c29(%rip), %rax   # 0x142aae738
140929b0f: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140929b14: 48 8d 0d 4a 4b 18 02        	leaq	0x2184b4a(%rip), %rcx   # 0x142aae665
140929b1b: ba 50 00 00 00              	movl	$0x50, %edx
140929b20: 45 31 c0                    	xorl	%r8d, %r8d
140929b23: e8 18 fe fa 01              	callq	0x1428d9940 <.text+0x28d8940>
140929b28: 31 c0                       	xorl	%eax, %eax
140929b2a: e9 cb 01 00 00              	jmp	0x140929cfa <.text+0x928cfa>
140929b2f: 75 d7                       	jne	0x140929b08 <.text+0x928b08>
140929b31: 4c 8d 35 2d 4b 18 02        	leaq	0x2184b2d(%rip), %r14   # 0x142aae665
140929b38: 4c 89 f1                    	movq	%r14, %rcx
140929b3b: 4c 89 ca                    	movq	%r9, %rdx
140929b3e: e8 dd 9d ff ff              	callq	0x140923920 <.text+0x922920>
140929b43: 48 83 f8 01                 	cmpq	$0x1, %rax
140929b47: 75 54                       	jne	0x140929b9d <.text+0x928b9d>
140929b49: 49 89 d0                    	movq	%rdx, %r8
140929b4c: 49 83 c0 02                 	addq	$0x2, %r8
140929b50: 74 39                       	je	0x140929b8b <.text+0x928b8b>
140929b52: 49 83 f8 50                 	cmpq	$0x50, %r8
140929b56: 73 31                       	jae	0x140929b89 <.text+0x928b89>
140929b58: 48 8d 05 06 4b 18 02        	leaq	0x2184b06(%rip), %rax   # 0x142aae665
140929b5f: 41 80 3c 00 bf              	cmpb	$-0x41, (%r8,%rax)
140929b64: 7f 25                       	jg	0x140929b8b <.text+0x928b8b>
140929b66: 48 8d 05 e3 4b 18 02        	leaq	0x2184be3(%rip), %rax   # 0x142aae750
140929b6d: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140929b72: 48 8d 0d ec 4a 18 02        	leaq	0x2184aec(%rip), %rcx   # 0x142aae665
140929b79: ba 50 00 00 00              	movl	$0x50, %edx
140929b7e: 41 b9 50 00 00 00           	movl	$0x50, %r9d
140929b84: e8 b7 fd fa 01              	callq	0x1428d9940 <.text+0x28d8940>
140929b89: 75 db                       	jne	0x140929b66 <.text+0x928b66>
140929b8b: be 4e 00 00 00              	movl	$0x4e, %esi
140929b90: 48 29 d6                    	subq	%rdx, %rsi
140929b93: 4c 8d 35 cb 4a 18 02        	leaq	0x2184acb(%rip), %r14   # 0x142aae665
140929b9a: 4d 01 c6                    	addq	%r8, %r14
140929b9d: 48 8d 05 e2 59 18 02        	leaq	0x21859e2(%rip), %rax   # 0x142aaf586
140929ba4: 4c 8d 3d 91 59 18 02        	leaq	0x2185991(%rip), %r15   # 0x142aaf53c
140929bab: 0f 1f 44 00 00              	nopl	(%rax,%rax)
140929bb0: 0f be 48 ff                 	movsbl	-0x1(%rax), %ecx
140929bb4: 85 c9                       	testl	%ecx, %ecx
140929bb6: 78 18                       	js	0x140929bd0 <.text+0x928bd0>
140929bb8: 48 ff c8                    	decq	%rax
140929bbb: 83 f9 5c                    	cmpl	$0x5c, %ecx
140929bbe: 75 69                       	jne	0x140929c29 <.text+0x928c29>
140929bc0: eb 7d                       	jmp	0x140929c3f <.text+0x928c3f>
140929bc2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
140929bd0: 0f b6 50 fe                 	movzbl	-0x2(%rax), %edx
140929bd4: 80 fa c0                    	cmpb	$-0x40, %dl
140929bd7: 7d 25                       	jge	0x140929bfe <.text+0x928bfe>
140929bd9: 44 0f b6 40 fd              	movzbl	-0x3(%rax), %r8d
140929bde: 41 80 f8 c0                 	cmpb	$-0x40, %r8b
140929be2: 7d 23                       	jge	0x140929c07 <.text+0x928c07>
140929be4: 44 0f b6 48 fc              	movzbl	-0x4(%rax), %r9d
140929be9: 48 83 c0 fc                 	addq	$-0x4, %rax
140929bed: 41 83 e1 07                 	andl	$0x7, %r9d
140929bf1: 41 c1 e1 06                 	shll	$0x6, %r9d
140929bf5: 41 83 e0 3f                 	andl	$0x3f, %r8d
140929bf9: 45 09 c8                    	orl	%r9d, %r8d
140929bfc: eb 11                       	jmp	0x140929c0f <.text+0x928c0f>
140929bfe: 48 83 c0 fe                 	addq	$-0x2, %rax
140929c02: 83 e2 1f                    	andl	$0x1f, %edx
140929c05: eb 12                       	jmp	0x140929c19 <.text+0x928c19>
140929c07: 48 83 c0 fd                 	addq	$-0x3, %rax
140929c0b: 41 83 e0 0f                 	andl	$0xf, %r8d
140929c0f: 41 c1 e0 06                 	shll	$0x6, %r8d
140929c13: 83 e2 3f                    	andl	$0x3f, %edx
140929c16: 44 09 c2                    	orl	%r8d, %edx
140929c19: c1 e2 06                    	shll	$0x6, %edx
140929c1c: 80 e1 3f                    	andb	$0x3f, %cl
140929c1f: 0f b6 c9                    	movzbl	%cl, %ecx
140929c22: 09 d1                       	orl	%edx, %ecx
140929c24: 83 f9 5c                    	cmpl	$0x5c, %ecx
140929c27: 74 16                       	je	0x140929c3f <.text+0x928c3f>
140929c29: 83 f9 2f                    	cmpl	$0x2f, %ecx
140929c2c: 74 11                       	je	0x140929c3f <.text+0x928c3f>
140929c2e: 4c 39 f8                    	cmpq	%r15, %rax
140929c31: 0f 85 79 ff ff ff           	jne	0x140929bb0 <.text+0x928bb0>
140929c37: 41 bc 4a 00 00 00           	movl	$0x4a, %r12d
140929c3d: eb 50                       	jmp	0x140929c8f <.text+0x928c8f>
140929c3f: 4c 8d 3d f6 58 18 02        	leaq	0x21858f6(%rip), %r15   # 0x142aaf53c
140929c46: 4c 29 f8                    	subq	%r15, %rax
140929c49: 49 89 c0                    	movq	%rax, %r8
140929c4c: 49 ff c0                    	incq	%r8
140929c4f: 74 32                       	je	0x140929c83 <.text+0x928c83>
140929c51: 49 83 f8 4a                 	cmpq	$0x4a, %r8
140929c55: 73 2a                       	jae	0x140929c81 <.text+0x928c81>
140929c57: 43 80 3c 38 bf              	cmpb	$-0x41, (%r8,%r15)
140929c5c: 7f 25                       	jg	0x140929c83 <.text+0x928c83>
140929c5e: 48 8d 05 b3 4a 18 02        	leaq	0x2184ab3(%rip), %rax   # 0x142aae718
140929c65: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140929c6a: 48 8d 0d cb 58 18 02        	leaq	0x21858cb(%rip), %rcx   # 0x142aaf53c
140929c71: ba 4a 00 00 00              	movl	$0x4a, %edx
140929c76: 41 b9 4a 00 00 00           	movl	$0x4a, %r9d
140929c7c: e8 bf fc fa 01              	callq	0x1428d9940 <.text+0x28d8940>
140929c81: 75 db                       	jne	0x140929c5e <.text+0x928c5e>
140929c83: 41 bc 49 00 00 00           	movl	$0x49, %r12d
140929c89: 49 29 c4                    	subq	%rax, %r12
140929c8c: 4d 01 c7                    	addq	%r8, %r15
140929c8f: e8 9c e4 2e 00              	callq	0x140c18130 <.text+0xc17130>
140929c94: 48 89 c1                    	movq	%rax, %rcx
140929c97: 48 8b 00                    	movq	(%rax), %rax
140929c9a: 48 83 f8 01                 	cmpq	$0x1, %rax
140929c9e: 0f 85 27 03 00 00           	jne	0x140929fcb <.text+0x928fcb>
140929ca4: 48 83 c1 08                 	addq	$0x8, %rcx
140929ca8: 48 83 39 00                 	cmpq	$0x0, (%rcx)
140929cac: 0f 85 53 03 00 00           	jne	0x14092a005 <.text+0x929005>
140929cb2: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
140929cb9: 48 89 4d 48                 	movq	%rcx, 0x48(%rbp)
140929cbd: 48 83 c1 08                 	addq	$0x8, %rcx
140929cc1: 4c 89 64 24 20              	movq	%r12, 0x20(%rsp)
140929cc6: 48 c7 44 24 30 00 00 00 00  	movq	$0x0, 0x30(%rsp)
140929ccf: 48 c7 44 24 28 01 00 00 00  	movq	$0x1, 0x28(%rsp)
140929cd8: 4c 89 f2                    	movq	%r14, %rdx
140929cdb: 49 89 f0                    	movq	%rsi, %r8
140929cde: 4d 89 f9                    	movq	%r15, %r9
140929ce1: e8 5a d0 2e 00              	callq	0x140c16d40 <.text+0xc15d40>
140929ce6: 48 8b 4d 48                 	movq	0x48(%rbp), %rcx
140929cea: 48 ff 01                    	incq	(%rcx)
140929ced: 48 89 45 20                 	movq	%rax, 0x20(%rbp)
140929cf1: 48 89 45 f0                 	movq	%rax, -0x10(%rbp)
140929cf5: b8 01 00 00 00              	movl	$0x1, %eax
140929cfa: 48 89 45 e8                 	movq	%rax, -0x18(%rbp)
140929cfe: 48 8b 0b                    	movq	(%rbx), %rcx
140929d01: 4c 8b 7b 08                 	movq	0x8(%rbx), %r15
140929d05: 4c 8b a1 f8 00 00 00        	movq	0xf8(%rcx), %r12
140929d0c: 48 8b 81 00 01 00 00        	movq	0x100(%rcx), %rax
140929d13: 4d 8d 2c 84                 	leaq	(%r12,%rax,4), %r13
140929d17: 48 89 4d 38                 	movq	%rcx, 0x38(%rbp)
140929d1b: 0f b6 b1 08 01 00 00        	movzbl	0x108(%rcx), %esi
140929d22: 48 8b 07                    	movq	(%rdi), %rax
140929d25: 48 89 45 48                 	movq	%rax, 0x48(%rbp)
140929d29: 48 8b 47 08                 	movq	0x8(%rdi), %rax
140929d2d: 48 89 45 08                 	movq	%rax, 0x8(%rbp)
140929d31: 48 8b 47 18                 	movq	0x18(%rdi), %rax
140929d35: 48 89 45 10                 	movq	%rax, 0x10(%rbp)
140929d39: 48 8b 47 10                 	movq	0x10(%rdi), %rax
140929d3d: 48 89 45 18                 	movq	%rax, 0x18(%rbp)
140929d41: b8 08 00 00 00              	movl	$0x8, %eax
140929d46: 48 89 45 30                 	movq	%rax, 0x30(%rbp)
140929d4a: 31 db                       	xorl	%ebx, %ebx
140929d4c: 0f 57 f6                    	xorps	%xmm6, %xmm6
140929d4f: 48 89 45 28                 	movq	%rax, 0x28(%rbp)
140929d53: 45 31 f6                    	xorl	%r14d, %r14d
140929d56: 48 c7 45 40 00 00 00 00     	movq	$0x0, 0x40(%rbp)
140929d5e: 31 ff                       	xorl	%edi, %edi
140929d60: eb 45                       	jmp	0x140929da7 <.text+0x928da7>
140929d62: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
140929d70: 49 ff c6                    	incq	%r14
140929d73: 48 89 45 e0                 	movq	%rax, -0x20(%rbp)
140929d77: 48 8b 45 48                 	movq	0x48(%rbp), %rax
140929d7b: 48 89 45 c0                 	movq	%rax, -0x40(%rbp)
140929d7f: 48 8b 45 08                 	movq	0x8(%rbp), %rax
140929d83: 48 89 45 c8                 	movq	%rax, -0x38(%rbp)
140929d87: 48 8b 45 18                 	movq	0x18(%rbp), %rax
140929d8b: 48 89 45 d0                 	movq	%rax, -0x30(%rbp)
140929d8f: 48 8b 45 10                 	movq	0x10(%rbp), %rax
140929d93: 48 89 45 d8                 	movq	%rax, -0x28(%rbp)
140929d97: 48 8d 4d c0                 	leaq	-0x40(%rbp), %rcx
140929d9b: 48 8d 15 e6 57 18 02        	leaq	0x21857e6(%rip), %rdx   # 0x142aaf588
140929da2: e8 c9 a9 3e 00              	callq	0x140d14770 <.text+0xd13770>
140929da7: 40 84 f6                    	testb	%sil, %sil
140929daa: 0f 84 80 00 00 00           	je	0x140929e30 <.text+0x928e30>
140929db0: 49 39 de                    	cmpq	%rbx, %r14
140929db3: 0f 85 3a 01 00 00           	jne	0x140929ef3 <.text+0x928ef3>
140929db9: 0f 1f 80 00 00 00 00        	nopl	(%rax)
140929dc0: 4d 39 ec                    	cmpq	%r13, %r12
140929dc3: 0f 84 aa 01 00 00           	je	0x140929f73 <.text+0x928f73>
140929dc9: 41 8b 0c 24                 	movl	(%r12), %ecx
140929dcd: 49 83 c4 04                 	addq	$0x4, %r12
140929dd1: 49 8b 87 a8 01 00 00        	movq	0x1a8(%r15), %rax
140929dd8: 48 8d 0c c9                 	leaq	(%rcx,%rcx,8), %rcx
140929ddc: 48 8b 5c c8 10              	movq	0x10(%rax,%rcx,8), %rbx
140929de1: 48 85 db                    	testq	%rbx, %rbx
140929de4: 74 da                       	je	0x140929dc0 <.text+0x928dc0>
140929de6: 48 8d 04 c8                 	leaq	(%rax,%rcx,8), %rax
140929dea: 48 8b 4d 38                 	movq	0x38(%rbp), %rcx
140929dee: 48 8b 89 18 01 00 00        	movq	0x118(%rcx), %rcx
140929df5: 48 39 48 40                 	cmpq	%rcx, 0x40(%rax)
140929df9: 0f 86 e7 00 00 00           	jbe	0x140929ee6 <.text+0x928ee6>
140929dff: 48 8b 50 38                 	movq	0x38(%rax), %rdx
140929e03: 48 8b 0c ca                 	movq	(%rdx,%rcx,8), %rcx
140929e07: 48 85 c9                    	testq	%rcx, %rcx
140929e0a: 0f 84 d6 00 00 00           	je	0x140929ee6 <.text+0x928ee6>
140929e10: 48 8b 50 18                 	movq	0x18(%rax), %rdx
140929e14: 48 f7 d1                    	notq	%rcx
140929e17: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
140929e1b: 48 c1 e1 04                 	shlq	$0x4, %rcx
140929e1f: 48 8b 4c 0a 10              	movq	0x10(%rdx,%rcx), %rcx
140929e24: 48 89 4d 40                 	movq	%rcx, 0x40(%rbp)
140929e28: 40 b7 01                    	movb	$0x1, %dil
140929e2b: e9 b8 00 00 00              	jmp	0x140929ee8 <.text+0x928ee8>
140929e30: 49 39 de                    	cmpq	%rbx, %r14
140929e33: 0f 85 a7 00 00 00           	jne	0x140929ee0 <.text+0x928ee0>
140929e39: 0f 1f 80 00 00 00 00        	nopl	(%rax)
140929e40: 4d 39 ec                    	cmpq	%r13, %r12
140929e43: 0f 84 2a 01 00 00           	je	0x140929f73 <.text+0x928f73>
140929e49: 41 8b 0c 24                 	movl	(%r12), %ecx
140929e4d: 49 83 c4 04                 	addq	$0x4, %r12
140929e51: 49 8b 87 00 01 00 00        	movq	0x100(%r15), %rax
140929e58: 48 8d 0c 89                 	leaq	(%rcx,%rcx,4), %rcx
140929e5c: 48 c1 e1 05                 	shlq	$0x5, %rcx
140929e60: 48 8b 5c 08 58              	movq	0x58(%rax,%rcx), %rbx
140929e65: 48 85 db                    	testq	%rbx, %rbx
140929e68: 74 d6                       	je	0x140929e40 <.text+0x928e40>
140929e6a: 48 01 c8                    	addq	%rcx, %rax
140929e6d: 48 8b 4d 38                 	movq	0x38(%rbp), %rcx
140929e71: 48 8b 89 18 01 00 00        	movq	0x118(%rcx), %rcx
140929e78: 48 39 88 88 00 00 00        	cmpq	%rcx, 0x88(%rax)
140929e7f: 0f 86 85 00 00 00           	jbe	0x140929f0a <.text+0x928f0a>
140929e85: 48 8b 90 80 00 00 00        	movq	0x80(%rax), %rdx
140929e8c: 48 83 3c ca 00              	cmpq	$0x0, (%rdx,%rcx,8)
140929e91: 74 77                       	je	0x140929f0a <.text+0x928f0a>
140929e93: 44 8b 80 94 00 00 00        	movl	0x94(%rax), %r8d
140929e9a: 49 8b 97 a8 01 00 00        	movq	0x1a8(%r15), %rdx
140929ea1: 4f 8d 04 c0                 	leaq	(%r8,%r8,8), %r8
140929ea5: 40 b7 01                    	movb	$0x1, %dil
140929ea8: 4a 39 4c c2 40              	cmpq	%rcx, 0x40(%rdx,%r8,8)
140929ead: 76 5f                       	jbe	0x140929f0e <.text+0x928f0e>
140929eaf: 4e 8b 4c c2 38              	movq	0x38(%rdx,%r8,8), %r9
140929eb4: 49 8b 0c c9                 	movq	(%r9,%rcx,8), %rcx
140929eb8: 48 85 c9                    	testq	%rcx, %rcx
140929ebb: 74 51                       	je	0x140929f0e <.text+0x928f0e>
140929ebd: 4a 8d 14 c2                 	leaq	(%rdx,%r8,8), %rdx
140929ec1: 48 8b 52 18                 	movq	0x18(%rdx), %rdx
140929ec5: 48 f7 d1                    	notq	%rcx
140929ec8: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
140929ecc: 48 c1 e1 04                 	shlq	$0x4, %rcx
140929ed0: 48 8b 4c 0a 10              	movq	0x10(%rdx,%rcx), %rcx
140929ed5: 48 89 4d 40                 	movq	%rcx, 0x40(%rbp)
140929ed9: eb 3b                       	jmp	0x140929f16 <.text+0x928f16>
140929edb: 0f 1f 44 00 00              	nopl	(%rax,%rax)
140929ee0: 48 8b 45 28                 	movq	0x28(%rbp), %rax
140929ee4: eb 37                       	jmp	0x140929f1d <.text+0x928f1d>
140929ee6: 31 ff                       	xorl	%edi, %edi
140929ee8: 48 8b 40 08                 	movq	0x8(%rax), %rax
140929eec: 48 89 45 30                 	movq	%rax, 0x30(%rbp)
140929ef0: 45 31 f6                    	xorl	%r14d, %r14d
140929ef3: 48 8b 45 30                 	movq	0x30(%rbp), %rax
140929ef7: 4a 8b 04 f0                 	movq	(%rax,%r14,8), %rax
140929efb: 40 f6 c7 01                 	testb	$0x1, %dil
140929eff: 0f 84 6b fe ff ff           	je	0x140929d70 <.text+0x928d70>
140929f05: 44 89 f1                    	movl	%r14d, %ecx
140929f08: eb 35                       	jmp	0x140929f3f <.text+0x928f3f>
140929f0a: 31 ff                       	xorl	%edi, %edi
140929f0c: eb 08                       	jmp	0x140929f16 <.text+0x928f16>
140929f0e: 48 c7 45 40 00 00 00 00     	movq	$0x0, 0x40(%rbp)
140929f16: 48 8b 40 50                 	movq	0x50(%rax), %rax
140929f1a: 45 31 f6                    	xorl	%r14d, %r14d
140929f1d: 4c 89 f1                    	movq	%r14, %rcx
140929f20: 48 c1 e1 04                 	shlq	$0x4, %rcx
140929f24: 48 89 c2                    	movq	%rax, %rdx
140929f27: 48 8b 04 08                 	movq	(%rax,%rcx), %rax
140929f2b: 40 f6 c7 01                 	testb	$0x1, %dil
140929f2f: 48 89 55 28                 	movq	%rdx, 0x28(%rbp)
140929f33: 0f 84 37 fe ff ff           	je	0x140929d70 <.text+0x928d70>
140929f39: 48 01 d1                    	addq	%rdx, %rcx
140929f3c: 8b 49 08                    	movl	0x8(%rcx), %ecx
140929f3f: 48 8d 14 c9                 	leaq	(%rcx,%rcx,8), %rdx
140929f43: 48 8d 14 52                 	leaq	(%rdx,%rdx,2), %rdx
140929f47: 48 03 4d 40                 	addq	0x40(%rbp), %rcx
140929f4b: 48 01 d1                    	addq	%rdx, %rcx
140929f4e: 49 ff c6                    	incq	%r14
140929f51: 48 85 c9                    	testq	%rcx, %rcx
140929f54: 0f 84 19 fe ff ff           	je	0x140929d73 <.text+0x928d73>
140929f5a: f3 0f 10 41 14              	movss	0x14(%rcx), %xmm0
140929f5f: 0f 2e c6                    	ucomiss	%xmm6, %xmm0
140929f62: 0f 85 3f fe ff ff           	jne	0x140929da7 <.text+0x928da7>
140929f68: 0f 8b 05 fe ff ff           	jnp	0x140929d73 <.text+0x928d73>
140929f6e: e9 34 fe ff ff              	jmp	0x140929da7 <.text+0x928da7>
140929f73: 80 7d 57 00                 	cmpb	$0x0, 0x57(%rbp)
140929f77: 74 3a                       	je	0x140929fb3 <.text+0x928fb3>
140929f79: e8 b2 e1 2e 00              	callq	0x140c18130 <.text+0xc17130>
140929f7e: 48 89 c1                    	movq	%rax, %rcx
140929f81: 48 8b 00                    	movq	(%rax), %rax
140929f84: 48 83 f8 01                 	cmpq	$0x1, %rax
140929f88: 75 5b                       	jne	0x140929fe5 <.text+0x928fe5>
140929f8a: 48 83 c1 08                 	addq	$0x8, %rcx
140929f8e: 48 83 39 00                 	cmpq	$0x0, (%rcx)
140929f92: 75 71                       	jne	0x14092a005 <.text+0x929005>
140929f94: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
140929f9b: 48 89 4d 48                 	movq	%rcx, 0x48(%rbp)
140929f9f: 48 83 c1 08                 	addq	$0x8, %rcx
140929fa3: 48 8b 55 20                 	movq	0x20(%rbp), %rdx
140929fa7: e8 44 d1 2e 00              	callq	0x140c170f0 <.text+0xc160f0>
140929fac: 48 8b 45 48                 	movq	0x48(%rbp), %rax
140929fb0: 48 ff 00                    	incq	(%rax)
140929fb3: 0f 28 75 60                 	movaps	0x60(%rbp), %xmm6
140929fb7: 48 81 c4 f8 00 00 00        	addq	$0xf8, %rsp
140929fbe: 5b                          	popq	%rbx
140929fbf: 5f                          	popq	%rdi
140929fc0: 5e                          	popq	%rsi
140929fc1: 41 5c                       	popq	%r12
140929fc3: 41 5d                       	popq	%r13
140929fc5: 41 5e                       	popq	%r14
140929fc7: 41 5f                       	popq	%r15
140929fc9: 5d                          	popq	%rbp
140929fca: c3                          	retq
140929fcb: 48 85 c0                    	testq	%rax, %rax
140929fce: 75 29                       	jne	0x140929ff9 <.text+0x928ff9>
140929fd0: 31 d2                       	xorl	%edx, %edx
140929fd2: e8 f9 e7 fc 01              	callq	0x1428f87d0 <.text+0x28f77d0>
140929fd7: 48 89 c1                    	movq	%rax, %rcx
140929fda: 48 85 c0                    	testq	%rax, %rax
140929fdd: 0f 85 c5 fc ff ff           	jne	0x140929ca8 <.text+0x928ca8>
140929fe3: eb 14                       	jmp	0x140929ff9 <.text+0x928ff9>
140929fe5: 48 85 c0                    	testq	%rax, %rax
140929fe8: 75 0f                       	jne	0x140929ff9 <.text+0x928ff9>
140929fea: 31 d2                       	xorl	%edx, %edx
140929fec: e8 df e7 fc 01              	callq	0x1428f87d0 <.text+0x28f77d0>
140929ff1: 48 89 c1                    	movq	%rax, %rcx
140929ff4: 48 85 c0                    	testq	%rax, %rax
140929ff7: 75 95                       	jne	0x140929f8e <.text+0x928f8e>
140929ff9: 48 8d 0d a0 0a 31 02        	leaq	0x2310aa0(%rip), %rcx   # 0x142c3aaa0
14092a000: e8 cb e0 fa 01              	callq	0x1428d80d0 <.text+0x28d70d0>
14092a005: 48 8d 0d 64 0b 31 02        	leaq	0x2310b64(%rip), %rcx   # 0x142c3ab70
14092a00c: e8 3f f2 fa 01              	callq	0x1428d9250 <.text+0x28d8250>
14092a011: cc                          	int3
14092a012: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
14092a020: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14092a025: 55                          	pushq	%rbp
14092a026: 41 57                       	pushq	%r15
14092a028: 41 56                       	pushq	%r14
14092a02a: 41 55                       	pushq	%r13
14092a02c: 41 54                       	pushq	%r12
14092a02e: 56                          	pushq	%rsi
14092a02f: 57                          	pushq	%rdi
14092a030: 53                          	pushq	%rbx
14092a031: 48 83 ec 48                 	subq	$0x48, %rsp
14092a035: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14092a03c: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
14092a041: 48 8b 45 48                 	movq	0x48(%rbp), %rax
14092a045: 48 ff 00                    	incq	(%rax)
14092a048: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
14092a04d: 48 83 c4 48                 	addq	$0x48, %rsp
14092a051: 5b                          	popq	%rbx
14092a052: 5f                          	popq	%rdi
14092a053: 5e                          	popq	%rsi
14092a054: 41 5c                       	popq	%r12
14092a056: 41 5d                       	popq	%r13
14092a058: 41 5e                       	popq	%r14
14092a05a: 41 5f                       	popq	%r15
14092a05c: 5d                          	popq	%rbp
14092a05d: c3                          	retq
14092a05e: 66 90                       	nop
14092a060: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14092a065: 55                          	pushq	%rbp
14092a066: 41 57                       	pushq	%r15
14092a068: 41 56                       	pushq	%r14
14092a06a: 41 55                       	pushq	%r13
14092a06c: 41 54                       	pushq	%r12
14092a06e: 56                          	pushq	%rsi
14092a06f: 57                          	pushq	%rdi
14092a070: 53                          	pushq	%rbx
14092a071: 48 83 ec 48                 	subq	$0x48, %rsp
14092a075: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14092a07c: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
14092a081: 48 8b 45 48                 	movq	0x48(%rbp), %rax
14092a085: 48 ff 00                    	incq	(%rax)
14092a088: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
14092a08d: 48 83 c4 48                 	addq	$0x48, %rsp
14092a091: 5b                          	popq	%rbx
14092a092: 5f                          	popq	%rdi
14092a093: 5e                          	popq	%rsi
14092a094: 41 5c                       	popq	%r12
14092a096: 41 5d                       	popq	%r13
14092a098: 41 5e                       	popq	%r14
14092a09a: 41 5f                       	popq	%r15
14092a09c: 5d                          	popq	%rbp
14092a09d: c3                          	retq
14092a09e: 66 90                       	nop
14092a0a0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14092a0a5: 55                          	pushq	%rbp
14092a0a6: 41 57                       	pushq	%r15
14092a0a8: 41 56                       	pushq	%r14
14092a0aa: 41 55                       	pushq	%r13
14092a0ac: 41 54                       	pushq	%r12
14092a0ae: 56                          	pushq	%rsi
14092a0af: 57                          	pushq	%rdi
14092a0b0: 53                          	pushq	%rbx
14092a0b1: 48 83 ec 48                 	subq	$0x48, %rsp
14092a0b5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14092a0bc: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
14092a0c1: 80 7d 57 00                 	cmpb	$0x0, 0x57(%rbp)
14092a0c5: 74 1f                       	je	0x14092a0e6 <.text+0x9290e6>
14092a0c7: 48 8d 45 f0                 	leaq	-0x10(%rbp), %rax
14092a0cb: 48 89 45 00                 	movq	%rax, (%rbp)
14092a0cf: 48 89 e8                    	movq	%rbp, %rax
14092a0d2: 48 89 45 f8                 	movq	%rax, -0x8(%rbp)
14092a0d6: 48 8d 0d db 45 18 02        	leaq	0x21845db(%rip), %rcx   # 0x142aae6b8
14092a0dd: 48 8d 55 f8                 	leaq	-0x8(%rbp), %rdx
14092a0e1: e8 ea 51 a0 00              	callq	0x14132f2d0 <.text+0x132e2d0>
14092a0e6: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
14092a0eb: 48 83 c4 48                 	addq	$0x48, %rsp
14092a0ef: 5b                          	popq	%rbx
14092a0f0: 5f                          	popq	%rdi
14092a0f1: 5e                          	popq	%rsi
14092a0f2: 41 5c                       	popq	%r12
14092a0f4: 41 5d                       	popq	%r13
14092a0f6: 41 5e                       	popq	%r14
14092a0f8: 41 5f                       	popq	%r15
14092a0fa: 5d                          	popq	%rbp
14092a0fb: c3                          	retq
