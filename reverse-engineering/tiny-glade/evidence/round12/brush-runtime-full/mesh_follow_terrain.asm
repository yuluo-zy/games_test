
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a68430: 55                          	pushq	%rbp
140a68431: 41 57                       	pushq	%r15
140a68433: 41 56                       	pushq	%r14
140a68435: 41 55                       	pushq	%r13
140a68437: 41 54                       	pushq	%r12
140a68439: 56                          	pushq	%rsi
140a6843a: 57                          	pushq	%rdi
140a6843b: 53                          	pushq	%rbx
140a6843c: 48 81 ec 08 01 00 00        	subq	$0x108, %rsp            # imm = 0x108
140a68443: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140a6844b: 48 c7 85 80 00 00 00 fe ff ff ff    	movq	$-0x2, 0x80(%rbp)
140a68456: 4c 89 45 38                 	movq	%r8, 0x38(%rbp)
140a6845a: 48 89 d6                    	movq	%rdx, %rsi
140a6845d: 49 89 ce                    	movq	%rcx, %r14
140a68460: e8 9b e5 1a 00              	callq	0x140c16a00 <.text+0xc15a00>
140a68465: 84 c0                       	testb	%al, %al
140a68467: 88 45 7f                    	movb	%al, 0x7f(%rbp)
140a6846a: 74 5f                       	je	0x140a684cb <.text+0xa674cb>
140a6846c: 4c 8d 3d f7 c6 06 02        	leaq	0x206c6f7(%rip), %r15   # 0x142ad4b6a
140a68473: bf 3f 00 00 00              	movl	$0x3f, %edi
140a68478: ba 3f 00 00 00              	movl	$0x3f, %edx
140a6847d: 4c 89 f9                    	movq	%r15, %rcx
140a68480: e8 fb ee ff ff              	callq	0x140a67380 <.text+0xa66380>
140a68485: 48 83 f8 01                 	cmpq	$0x1, %rax
140a68489: 0f 85 b1 00 00 00           	jne	0x140a68540 <.text+0xa67540>
140a6848f: 49 89 d1                    	movq	%rdx, %r9
140a68492: 48 85 d2                    	testq	%rdx, %rdx
140a68495: 74 3d                       	je	0x140a684d4 <.text+0xa674d4>
140a68497: 49 83 f9 3f                 	cmpq	$0x3f, %r9
140a6849b: 73 35                       	jae	0x140a684d2 <.text+0xa674d2>
140a6849d: 48 8d 05 c6 c6 06 02        	leaq	0x206c6c6(%rip), %rax   # 0x142ad4b6a
140a684a4: 41 80 3c 01 bf              	cmpb	$-0x41, (%r9,%rax)
140a684a9: 7f 29                       	jg	0x140a684d4 <.text+0xa674d4>
140a684ab: 48 8d 05 c6 c7 06 02        	leaq	0x206c7c6(%rip), %rax   # 0x142ad4c78
140a684b2: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140a684b7: 48 8d 0d ac c6 06 02        	leaq	0x206c6ac(%rip), %rcx   # 0x142ad4b6a
140a684be: ba 3f 00 00 00              	movl	$0x3f, %edx
140a684c3: 45 31 c0                    	xorl	%r8d, %r8d
140a684c6: e8 75 14 e7 01              	callq	0x1428d9940 <.text+0x28d8940>
140a684cb: 31 c0                       	xorl	%eax, %eax
140a684cd: e9 c6 01 00 00              	jmp	0x140a68698 <.text+0xa67698>
140a684d2: 75 d7                       	jne	0x140a684ab <.text+0xa674ab>
140a684d4: 4c 8d 3d 8f c6 06 02        	leaq	0x206c68f(%rip), %r15   # 0x142ad4b6a
140a684db: 4c 89 f9                    	movq	%r15, %rcx
140a684de: 4c 89 ca                    	movq	%r9, %rdx
140a684e1: e8 9a ee ff ff              	callq	0x140a67380 <.text+0xa66380>
140a684e6: 48 83 f8 01                 	cmpq	$0x1, %rax
140a684ea: 75 54                       	jne	0x140a68540 <.text+0xa67540>
140a684ec: 49 89 d0                    	movq	%rdx, %r8
140a684ef: 49 83 c0 02                 	addq	$0x2, %r8
140a684f3: 74 39                       	je	0x140a6852e <.text+0xa6752e>
140a684f5: 49 83 f8 3f                 	cmpq	$0x3f, %r8
140a684f9: 73 31                       	jae	0x140a6852c <.text+0xa6752c>
140a684fb: 48 8d 05 68 c6 06 02        	leaq	0x206c668(%rip), %rax   # 0x142ad4b6a
140a68502: 41 80 3c 00 bf              	cmpb	$-0x41, (%r8,%rax)
140a68507: 7f 25                       	jg	0x140a6852e <.text+0xa6752e>
140a68509: 48 8d 05 80 c7 06 02        	leaq	0x206c780(%rip), %rax   # 0x142ad4c90
140a68510: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140a68515: 48 8d 0d 4e c6 06 02        	leaq	0x206c64e(%rip), %rcx   # 0x142ad4b6a
140a6851c: ba 3f 00 00 00              	movl	$0x3f, %edx
140a68521: 41 b9 3f 00 00 00           	movl	$0x3f, %r9d
140a68527: e8 14 14 e7 01              	callq	0x1428d9940 <.text+0x28d8940>
140a6852c: 75 db                       	jne	0x140a68509 <.text+0xa67509>
140a6852e: bf 3d 00 00 00              	movl	$0x3d, %edi
140a68533: 48 29 d7                    	subq	%rdx, %rdi
140a68536: 4c 8d 3d 2d c6 06 02        	leaq	0x206c62d(%rip), %r15   # 0x142ad4b6a
140a6853d: 4d 01 c7                    	addq	%r8, %r15
140a68540: 48 8d 05 91 cb 06 02        	leaq	0x206cb91(%rip), %rax   # 0x142ad50d8
140a68547: 4c 8d 25 54 cb 06 02        	leaq	0x206cb54(%rip), %r12   # 0x142ad50a2
140a6854e: 66 90                       	nop
140a68550: 0f be 48 ff                 	movsbl	-0x1(%rax), %ecx
140a68554: 85 c9                       	testl	%ecx, %ecx
140a68556: 78 18                       	js	0x140a68570 <.text+0xa67570>
140a68558: 48 ff c8                    	decq	%rax
140a6855b: 83 f9 5c                    	cmpl	$0x5c, %ecx
140a6855e: 75 69                       	jne	0x140a685c9 <.text+0xa675c9>
140a68560: eb 7c                       	jmp	0x140a685de <.text+0xa675de>
140a68562: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
140a68570: 0f b6 50 fe                 	movzbl	-0x2(%rax), %edx
140a68574: 80 fa c0                    	cmpb	$-0x40, %dl
140a68577: 7d 25                       	jge	0x140a6859e <.text+0xa6759e>
140a68579: 44 0f b6 40 fd              	movzbl	-0x3(%rax), %r8d
140a6857e: 41 80 f8 c0                 	cmpb	$-0x40, %r8b
140a68582: 7d 23                       	jge	0x140a685a7 <.text+0xa675a7>
140a68584: 44 0f b6 48 fc              	movzbl	-0x4(%rax), %r9d
140a68589: 48 83 c0 fc                 	addq	$-0x4, %rax
140a6858d: 41 83 e1 07                 	andl	$0x7, %r9d
140a68591: 41 c1 e1 06                 	shll	$0x6, %r9d
140a68595: 41 83 e0 3f                 	andl	$0x3f, %r8d
140a68599: 45 09 c8                    	orl	%r9d, %r8d
140a6859c: eb 11                       	jmp	0x140a685af <.text+0xa675af>
140a6859e: 48 83 c0 fe                 	addq	$-0x2, %rax
140a685a2: 83 e2 1f                    	andl	$0x1f, %edx
140a685a5: eb 12                       	jmp	0x140a685b9 <.text+0xa675b9>
140a685a7: 48 83 c0 fd                 	addq	$-0x3, %rax
140a685ab: 41 83 e0 0f                 	andl	$0xf, %r8d
140a685af: 41 c1 e0 06                 	shll	$0x6, %r8d
140a685b3: 83 e2 3f                    	andl	$0x3f, %edx
140a685b6: 44 09 c2                    	orl	%r8d, %edx
140a685b9: c1 e2 06                    	shll	$0x6, %edx
140a685bc: 80 e1 3f                    	andb	$0x3f, %cl
140a685bf: 0f b6 c9                    	movzbl	%cl, %ecx
140a685c2: 09 d1                       	orl	%edx, %ecx
140a685c4: 83 f9 5c                    	cmpl	$0x5c, %ecx
140a685c7: 74 15                       	je	0x140a685de <.text+0xa675de>
140a685c9: 83 f9 2f                    	cmpl	$0x2f, %ecx
140a685cc: 74 10                       	je	0x140a685de <.text+0xa675de>
140a685ce: 4c 39 e0                    	cmpq	%r12, %rax
140a685d1: 0f 85 79 ff ff ff           	jne	0x140a68550 <.text+0xa67550>
140a685d7: bb 36 00 00 00              	movl	$0x36, %ebx
140a685dc: eb 4f                       	jmp	0x140a6862d <.text+0xa6762d>
140a685de: 4c 8d 25 bd ca 06 02        	leaq	0x206cabd(%rip), %r12   # 0x142ad50a2
140a685e5: 4c 29 e0                    	subq	%r12, %rax
140a685e8: 49 89 c0                    	movq	%rax, %r8
140a685eb: 49 ff c0                    	incq	%r8
140a685ee: 74 32                       	je	0x140a68622 <.text+0xa67622>
140a685f0: 49 83 f8 36                 	cmpq	$0x36, %r8
140a685f4: 73 2a                       	jae	0x140a68620 <.text+0xa67620>
140a685f6: 43 80 3c 20 bf              	cmpb	$-0x41, (%r8,%r12)
140a685fb: 7f 25                       	jg	0x140a68622 <.text+0xa67622>
140a685fd: 48 8d 05 54 c6 06 02        	leaq	0x206c654(%rip), %rax   # 0x142ad4c58
140a68604: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140a68609: 48 8d 0d 92 ca 06 02        	leaq	0x206ca92(%rip), %rcx   # 0x142ad50a2
140a68610: ba 36 00 00 00              	movl	$0x36, %edx
140a68615: 41 b9 36 00 00 00           	movl	$0x36, %r9d
140a6861b: e8 20 13 e7 01              	callq	0x1428d9940 <.text+0x28d8940>
140a68620: 75 db                       	jne	0x140a685fd <.text+0xa675fd>
140a68622: bb 35 00 00 00              	movl	$0x35, %ebx
140a68627: 48 29 c3                    	subq	%rax, %rbx
140a6862a: 4d 01 c4                    	addq	%r8, %r12
140a6862d: e8 fe fa 1a 00              	callq	0x140c18130 <.text+0xc17130>
140a68632: 48 89 c1                    	movq	%rax, %rcx
140a68635: 48 8b 00                    	movq	(%rax), %rax
140a68638: 48 83 f8 01                 	cmpq	$0x1, %rax
140a6863c: 0f 85 b9 02 00 00           	jne	0x140a688fb <.text+0xa678fb>
140a68642: 48 83 c1 08                 	addq	$0x8, %rcx
140a68646: 48 83 39 00                 	cmpq	$0x0, (%rcx)
140a6864a: 0f 85 ef 02 00 00           	jne	0x140a6893f <.text+0xa6793f>
140a68650: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
140a68657: 48 89 4d 70                 	movq	%rcx, 0x70(%rbp)
140a6865b: 48 83 c1 08                 	addq	$0x8, %rcx
140a6865f: 48 89 5c 24 20              	movq	%rbx, 0x20(%rsp)
140a68664: 48 c7 44 24 30 00 00 00 00  	movq	$0x0, 0x30(%rsp)
140a6866d: 48 c7 44 24 28 01 00 00 00  	movq	$0x1, 0x28(%rsp)
140a68676: 4c 89 fa                    	movq	%r15, %rdx
140a68679: 49 89 f8                    	movq	%rdi, %r8
140a6867c: 4d 89 e1                    	movq	%r12, %r9
140a6867f: e8 bc e6 1a 00              	callq	0x140c16d40 <.text+0xc15d40>
140a68684: 48 8b 4d 70                 	movq	0x70(%rbp), %rcx
140a68688: 48 ff 01                    	incq	(%rcx)
140a6868b: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
140a6868f: 48 89 45 20                 	movq	%rax, 0x20(%rbp)
140a68693: b8 01 00 00 00              	movl	$0x1, %eax
140a68698: 48 89 45 18                 	movq	%rax, 0x18(%rbp)
140a6869c: 49 8b 0e                    	movq	(%r14), %rcx
140a6869f: 4d 8b 6e 08                 	movq	0x8(%r14), %r13
140a686a3: 48 8b b9 f8 00 00 00        	movq	0xf8(%rcx), %rdi
140a686aa: 48 8b 81 00 01 00 00        	movq	0x100(%rcx), %rax
140a686b1: 4c 8d 24 87                 	leaq	(%rdi,%rax,4), %r12
140a686b5: 48 89 4d 58                 	movq	%rcx, 0x58(%rbp)
140a686b9: 0f b6 81 08 01 00 00        	movzbl	0x108(%rcx), %eax
140a686c0: 88 45 70                    	movb	%al, 0x70(%rbp)
140a686c3: 8b 46 1c                    	movl	0x1c(%rsi), %eax
140a686c6: 89 45 6c                    	movl	%eax, 0x6c(%rbp)
140a686c9: 48 8b 06                    	movq	(%rsi), %rax
140a686cc: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
140a686d0: 48 8b 46 10                 	movq	0x10(%rsi), %rax
140a686d4: 48 89 45 48                 	movq	%rax, 0x48(%rbp)
140a686d8: b8 08 00 00 00              	movl	$0x8, %eax
140a686dd: 48 89 45 50                 	movq	%rax, 0x50(%rbp)
140a686e1: 45 31 ff                    	xorl	%r15d, %r15d
140a686e4: 45 31 f6                    	xorl	%r14d, %r14d
140a686e7: 31 db                       	xorl	%ebx, %ebx
140a686e9: 80 7d 70 00                 	cmpb	$0x0, 0x70(%rbp)
140a686ed: 0f 84 7d 00 00 00           	je	0x140a68770 <.text+0xa67770>
140a686f3: 4c 39 f3                    	cmpq	%r14, %rbx
140a686f6: 0f 85 04 01 00 00           	jne	0x140a68800 <.text+0xa67800>
140a686fc: 0f 1f 40 00                 	nopl	(%rax)
140a68700: 4c 39 e7                    	cmpq	%r12, %rdi
140a68703: 0f 84 9e 01 00 00           	je	0x140a688a7 <.text+0xa678a7>
140a68709: 8b 0f                       	movl	(%rdi), %ecx
140a6870b: 48 83 c7 04                 	addq	$0x4, %rdi
140a6870f: 49 8b 85 a8 01 00 00        	movq	0x1a8(%r13), %rax
140a68716: 48 8d 0c c9                 	leaq	(%rcx,%rcx,8), %rcx
140a6871a: 4c 8b 74 c8 10              	movq	0x10(%rax,%rcx,8), %r14
140a6871f: 4d 85 f6                    	testq	%r14, %r14
140a68722: 74 dc                       	je	0x140a68700 <.text+0xa67700>
140a68724: 48 8d 04 c8                 	leaq	(%rax,%rcx,8), %rax
140a68728: 48 8b 4d 58                 	movq	0x58(%rbp), %rcx
140a6872c: 48 8b 89 10 01 00 00        	movq	0x110(%rcx), %rcx
140a68733: 48 39 48 40                 	cmpq	%rcx, 0x40(%rax)
140a68737: 0f 86 c7 00 00 00           	jbe	0x140a68804 <.text+0xa67804>
140a6873d: 48 8b 50 38                 	movq	0x38(%rax), %rdx
140a68741: 48 8b 0c ca                 	movq	(%rdx,%rcx,8), %rcx
140a68745: 48 85 c9                    	testq	%rcx, %rcx
140a68748: 0f 84 b6 00 00 00           	je	0x140a68804 <.text+0xa67804>
140a6874e: 48 8b 40 18                 	movq	0x18(%rax), %rax
140a68752: 48 f7 d1                    	notq	%rcx
140a68755: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
140a68759: 48 c1 e1 04                 	shlq	$0x4, %rcx
140a6875d: 4c 8b 7c 08 10              	movq	0x10(%rax,%rcx), %r15
140a68762: 31 db                       	xorl	%ebx, %ebx
140a68764: 89 d8                       	movl	%ebx, %eax
140a68766: e9 be 00 00 00              	jmp	0x140a68829 <.text+0xa67829>
140a6876b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
140a68770: 4c 39 f3                    	cmpq	%r14, %rbx
140a68773: 0f 85 a1 00 00 00           	jne	0x140a6881a <.text+0xa6781a>
140a68779: 0f 1f 80 00 00 00 00        	nopl	(%rax)
140a68780: 4c 39 e7                    	cmpq	%r12, %rdi
140a68783: 0f 84 1e 01 00 00           	je	0x140a688a7 <.text+0xa678a7>
140a68789: 8b 0f                       	movl	(%rdi), %ecx
140a6878b: 48 83 c7 04                 	addq	$0x4, %rdi
140a6878f: 49 8b 85 00 01 00 00        	movq	0x100(%r13), %rax
140a68796: 48 8d 0c 89                 	leaq	(%rcx,%rcx,4), %rcx
140a6879a: 48 c1 e1 05                 	shlq	$0x5, %rcx
140a6879e: 4c 8b 74 08 58              	movq	0x58(%rax,%rcx), %r14
140a687a3: 4d 85 f6                    	testq	%r14, %r14
140a687a6: 74 d8                       	je	0x140a68780 <.text+0xa67780>
140a687a8: 48 01 c8                    	addq	%rcx, %rax
140a687ab: 8b 88 94 00 00 00           	movl	0x94(%rax), %ecx
140a687b1: 49 8b 95 a8 01 00 00        	movq	0x1a8(%r13), %rdx
140a687b8: 4c 8d 04 c9                 	leaq	(%rcx,%rcx,8), %r8
140a687bc: 48 8b 4d 58                 	movq	0x58(%rbp), %rcx
140a687c0: 48 8b 89 10 01 00 00        	movq	0x110(%rcx), %rcx
140a687c7: 4a 39 4c c2 40              	cmpq	%rcx, 0x40(%rdx,%r8,8)
140a687cc: 76 3f                       	jbe	0x140a6880d <.text+0xa6780d>
140a687ce: 4a 8d 14 c2                 	leaq	(%rdx,%r8,8), %rdx
140a687d2: 4c 8b 42 38                 	movq	0x38(%rdx), %r8
140a687d6: 49 8b 0c c8                 	movq	(%r8,%rcx,8), %rcx
140a687da: 48 85 c9                    	testq	%rcx, %rcx
140a687dd: 74 2e                       	je	0x140a6880d <.text+0xa6780d>
140a687df: 48 8b 52 18                 	movq	0x18(%rdx), %rdx
140a687e3: 48 f7 d1                    	notq	%rcx
140a687e6: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
140a687ea: 48 c1 e1 04                 	shlq	$0x4, %rcx
140a687ee: 4c 8b 7c 0a 10              	movq	0x10(%rdx,%rcx), %r15
140a687f3: eb 1b                       	jmp	0x140a68810 <.text+0xa67810>
140a687f5: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
140a68800: 89 d8                       	movl	%ebx, %eax
140a68802: eb 25                       	jmp	0x140a68829 <.text+0xa67829>
140a68804: 45 31 ff                    	xorl	%r15d, %r15d
140a68807: 31 db                       	xorl	%ebx, %ebx
140a68809: 89 d8                       	movl	%ebx, %eax
140a6880b: eb 1c                       	jmp	0x140a68829 <.text+0xa67829>
140a6880d: 45 31 ff                    	xorl	%r15d, %r15d
140a68810: 48 8b 40 50                 	movq	0x50(%rax), %rax
140a68814: 48 89 45 50                 	movq	%rax, 0x50(%rbp)
140a68818: 31 db                       	xorl	%ebx, %ebx
140a6881a: 48 89 d8                    	movq	%rbx, %rax
140a6881d: 48 c1 e0 04                 	shlq	$0x4, %rax
140a68821: 48 8b 4d 50                 	movq	0x50(%rbp), %rcx
140a68825: 8b 44 01 08                 	movl	0x8(%rcx,%rax), %eax
140a68829: 49 8d 04 c7                 	leaq	(%r15,%rax,8), %rax
140a6882d: 8b 4d 6c                    	movl	0x6c(%rbp), %ecx
140a68830: 48 8b 55 48                 	movq	0x48(%rbp), %rdx
140a68834: 89 0a                       	movl	%ecx, (%rdx)
140a68836: 48 8b 10                    	movq	(%rax), %rdx
140a68839: 48 8b 4d 40                 	movq	0x40(%rbp), %rcx
140a6883d: e8 be 4a ec ff              	callq	0x14092d300 <.text+0x92c300>
140a68842: 48 89 c6                    	movq	%rax, %rsi
140a68845: 48 85 c0                    	testq	%rax, %rax
140a68848: 0f 84 fd 00 00 00           	je	0x140a6894b <.text+0xa6794b>
140a6884e: 48 8d 4d d0                 	leaq	-0x30(%rbp), %rcx
140a68852: 48 89 f2                    	movq	%rsi, %rdx
140a68855: e8 d6 f2 f9 ff              	callq	0x140a07b30 <.text+0xa06b30>
140a6885a: 0f 10 45 d0                 	movups	-0x30(%rbp), %xmm0
140a6885e: 0f 10 4d e0                 	movups	-0x20(%rbp), %xmm1
140a68862: 0f 29 4d 00                 	movaps	%xmm1, (%rbp)
140a68866: 0f 29 45 f0                 	movaps	%xmm0, -0x10(%rbp)
140a6886a: 48 8b 45 38                 	movq	0x38(%rbp), %rax
140a6886e: 48 89 45 10                 	movq	%rax, 0x10(%rbp)
140a68872: 48 8d 4d b8                 	leaq	-0x48(%rbp), %rcx
140a68876: 48 8d 55 f0                 	leaq	-0x10(%rbp), %rdx
140a6887a: 4c 8d 05 17 c2 06 02        	leaq	0x206c217(%rip), %r8    # 0x142ad4a98
140a68881: e8 ca 4c 89 00              	callq	0x1412fd550 <.text+0x12fc550>
140a68886: 48 ff c3                    	incq	%rbx
140a68889: 41 b8 0f 00 00 00           	movl	$0xf, %r8d
140a6888f: 48 89 f1                    	movq	%rsi, %rcx
140a68892: 48 8d 15 87 c8 06 02        	leaq	0x206c887(%rip), %rdx   # 0x142ad5120
140a68899: 4c 8d 4d b8                 	leaq	-0x48(%rbp), %r9
140a6889d: e8 ee c3 fc ff              	callq	0x140a34c90 <.text+0xa33c90>
140a688a2: e9 42 fe ff ff              	jmp	0x140a686e9 <.text+0xa676e9>
140a688a7: 80 7d 7f 00                 	cmpb	$0x0, 0x7f(%rbp)
140a688ab: 74 3a                       	je	0x140a688e7 <.text+0xa678e7>
140a688ad: e8 7e f8 1a 00              	callq	0x140c18130 <.text+0xc17130>
140a688b2: 48 89 c1                    	movq	%rax, %rcx
140a688b5: 48 8b 00                    	movq	(%rax), %rax
140a688b8: 48 83 f8 01                 	cmpq	$0x1, %rax
140a688bc: 75 57                       	jne	0x140a68915 <.text+0xa67915>
140a688be: 48 83 c1 08                 	addq	$0x8, %rcx
140a688c2: 48 8b 55 60                 	movq	0x60(%rbp), %rdx
140a688c6: 48 83 39 00                 	cmpq	$0x0, (%rcx)
140a688ca: 75 73                       	jne	0x140a6893f <.text+0xa6793f>
140a688cc: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
140a688d3: 48 89 4d 70                 	movq	%rcx, 0x70(%rbp)
140a688d7: 48 83 c1 08                 	addq	$0x8, %rcx
140a688db: e8 10 e8 1a 00              	callq	0x140c170f0 <.text+0xc160f0>
140a688e0: 48 8b 45 70                 	movq	0x70(%rbp), %rax
140a688e4: 48 ff 00                    	incq	(%rax)
140a688e7: 48 81 c4 08 01 00 00        	addq	$0x108, %rsp            # imm = 0x108
140a688ee: 5b                          	popq	%rbx
140a688ef: 5f                          	popq	%rdi
140a688f0: 5e                          	popq	%rsi
140a688f1: 41 5c                       	popq	%r12
140a688f3: 41 5d                       	popq	%r13
140a688f5: 41 5e                       	popq	%r14
140a688f7: 41 5f                       	popq	%r15
140a688f9: 5d                          	popq	%rbp
140a688fa: c3                          	retq
140a688fb: 48 85 c0                    	testq	%rax, %rax
140a688fe: 75 33                       	jne	0x140a68933 <.text+0xa67933>
140a68900: 31 d2                       	xorl	%edx, %edx
140a68902: e8 c9 fe e8 01              	callq	0x1428f87d0 <.text+0x28f77d0>
140a68907: 48 89 c1                    	movq	%rax, %rcx
140a6890a: 48 85 c0                    	testq	%rax, %rax
140a6890d: 0f 85 33 fd ff ff           	jne	0x140a68646 <.text+0xa67646>
140a68913: eb 1e                       	jmp	0x140a68933 <.text+0xa67933>
140a68915: 48 85 c0                    	testq	%rax, %rax
140a68918: 48 8b 45 60                 	movq	0x60(%rbp), %rax
140a6891c: 75 15                       	jne	0x140a68933 <.text+0xa67933>
140a6891e: 48 89 c6                    	movq	%rax, %rsi
140a68921: 31 d2                       	xorl	%edx, %edx
140a68923: e8 a8 fe e8 01              	callq	0x1428f87d0 <.text+0x28f77d0>
140a68928: 48 89 f2                    	movq	%rsi, %rdx
140a6892b: 48 89 c1                    	movq	%rax, %rcx
140a6892e: 48 85 c0                    	testq	%rax, %rax
140a68931: 75 93                       	jne	0x140a688c6 <.text+0xa678c6>
140a68933: 48 8d 0d 66 21 1d 02        	leaq	0x21d2166(%rip), %rcx   # 0x142c3aaa0
140a6893a: e8 91 f7 e6 01              	callq	0x1428d80d0 <.text+0x28d70d0>
140a6893f: 48 8d 0d 2a 22 1d 02        	leaq	0x21d222a(%rip), %rcx   # 0x142c3ab70
140a68946: e8 05 09 e7 01              	callq	0x1428d9250 <.text+0x28d8250>
140a6894b: 48 8d 0d 86 c7 06 02        	leaq	0x206c786(%rip), %rcx   # 0x142ad50d8
140a68952: 4c 8d 05 af c7 06 02        	leaq	0x206c7af(%rip), %r8    # 0x142ad5108
140a68959: ba 2c 00 00 00              	movl	$0x2c, %edx
140a6895e: e8 cd 09 e7 01              	callq	0x1428d9330 <.text+0x28d8330>
140a68963: 0f 0b                       	ud2
140a68965: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
140a68970: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a68975: 55                          	pushq	%rbp
140a68976: 41 57                       	pushq	%r15
140a68978: 41 56                       	pushq	%r14
140a6897a: 41 55                       	pushq	%r13
140a6897c: 41 54                       	pushq	%r12
140a6897e: 56                          	pushq	%rsi
140a6897f: 57                          	pushq	%rdi
140a68980: 53                          	pushq	%rbx
140a68981: 48 83 ec 38                 	subq	$0x38, %rsp
140a68985: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a6898c: 48 8b 45 70                 	movq	0x70(%rbp), %rax
140a68990: 48 ff 00                    	incq	(%rax)
140a68993: 48 83 c4 38                 	addq	$0x38, %rsp
140a68997: 5b                          	popq	%rbx
140a68998: 5f                          	popq	%rdi
140a68999: 5e                          	popq	%rsi
140a6899a: 41 5c                       	popq	%r12
140a6899c: 41 5d                       	popq	%r13
140a6899e: 41 5e                       	popq	%r14
140a689a0: 41 5f                       	popq	%r15
140a689a2: 5d                          	popq	%rbp
140a689a3: c3                          	retq
140a689a4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
140a689b0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a689b5: 55                          	pushq	%rbp
140a689b6: 41 57                       	pushq	%r15
140a689b8: 41 56                       	pushq	%r14
140a689ba: 41 55                       	pushq	%r13
140a689bc: 41 54                       	pushq	%r12
140a689be: 56                          	pushq	%rsi
140a689bf: 57                          	pushq	%rdi
140a689c0: 53                          	pushq	%rbx
140a689c1: 48 83 ec 38                 	subq	$0x38, %rsp
140a689c5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a689cc: 48 8b 45 70                 	movq	0x70(%rbp), %rax
140a689d0: 48 ff 00                    	incq	(%rax)
140a689d3: 48 83 c4 38                 	addq	$0x38, %rsp
140a689d7: 5b                          	popq	%rbx
140a689d8: 5f                          	popq	%rdi
140a689d9: 5e                          	popq	%rsi
140a689da: 41 5c                       	popq	%r12
140a689dc: 41 5d                       	popq	%r13
140a689de: 41 5e                       	popq	%r14
140a689e0: 41 5f                       	popq	%r15
140a689e2: 5d                          	popq	%rbp
140a689e3: c3                          	retq
140a689e4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
140a689f0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a689f5: 55                          	pushq	%rbp
140a689f6: 41 57                       	pushq	%r15
140a689f8: 41 56                       	pushq	%r14
140a689fa: 41 55                       	pushq	%r13
140a689fc: 41 54                       	pushq	%r12
140a689fe: 56                          	pushq	%rsi
140a689ff: 57                          	pushq	%rdi
140a68a00: 53                          	pushq	%rbx
140a68a01: 48 83 ec 38                 	subq	$0x38, %rsp
140a68a05: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a68a0c: 80 7d 7f 00                 	cmpb	$0x0, 0x7f(%rbp)
140a68a10: 74 20                       	je	0x140a68a32 <.text+0xa67a32>
140a68a12: 48 8d 45 20                 	leaq	0x20(%rbp), %rax
140a68a16: 48 89 45 30                 	movq	%rax, 0x30(%rbp)
140a68a1a: 48 8d 45 30                 	leaq	0x30(%rbp), %rax
140a68a1e: 48 89 45 28                 	movq	%rax, 0x28(%rbp)
140a68a22: 48 8d 0d cf c1 06 02        	leaq	0x206c1cf(%rip), %rcx   # 0x142ad4bf8
140a68a29: 48 8d 55 28                 	leaq	0x28(%rbp), %rdx
140a68a2d: e8 9e 68 8c 00              	callq	0x14132f2d0 <.text+0x132e2d0>
140a68a32: 90                          	nop
140a68a33: 48 83 c4 38                 	addq	$0x38, %rsp
140a68a37: 5b                          	popq	%rbx
140a68a38: 5f                          	popq	%rdi
140a68a39: 5e                          	popq	%rsi
140a68a3a: 41 5c                       	popq	%r12
140a68a3c: 41 5d                       	popq	%r13
140a68a3e: 41 5e                       	popq	%r14
140a68a40: 41 5f                       	popq	%r15
140a68a42: 5d                          	popq	%rbp
140a68a43: c3                          	retq
