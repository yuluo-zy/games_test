
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1416ee410: 55                          	pushq	%rbp
1416ee411: 41 57                       	pushq	%r15
1416ee413: 41 56                       	pushq	%r14
1416ee415: 41 55                       	pushq	%r13
1416ee417: 41 54                       	pushq	%r12
1416ee419: 56                          	pushq	%rsi
1416ee41a: 57                          	pushq	%rdi
1416ee41b: 53                          	pushq	%rbx
1416ee41c: 48 81 ec 88 03 00 00        	subq	$0x388, %rsp            # imm = 0x388
1416ee423: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1416ee42b: 44 0f 29 9d f0 02 00 00     	movaps	%xmm11, 0x2f0(%rbp)
1416ee433: 44 0f 29 95 e0 02 00 00     	movaps	%xmm10, 0x2e0(%rbp)
1416ee43b: 44 0f 29 8d d0 02 00 00     	movaps	%xmm9, 0x2d0(%rbp)
1416ee443: 44 0f 29 85 c0 02 00 00     	movaps	%xmm8, 0x2c0(%rbp)
1416ee44b: 0f 29 bd b0 02 00 00        	movaps	%xmm7, 0x2b0(%rbp)
1416ee452: 0f 29 b5 a0 02 00 00        	movaps	%xmm6, 0x2a0(%rbp)
1416ee459: 48 c7 85 98 02 00 00 fe ff ff ff    	movq	$-0x2, 0x298(%rbp)
1416ee464: 49 89 d5                    	movq	%rdx, %r13
1416ee467: 48 89 cf                    	movq	%rcx, %rdi
1416ee46a: 48 8d 4d e0                 	leaq	-0x20(%rbp), %rcx
1416ee46e: f3 0f 10 15 ca 6c 3b 01     	movss	0x13b6cca(%rip), %xmm2  # 0x142aa5140
1416ee476: 0f 57 db                    	xorps	%xmm3, %xmm3
1416ee479: e8 a2 c9 54 ff              	callq	0x140c3ae20 <.text+0xc39e20>
1416ee47e: 48 8b 37                    	movq	(%rdi), %rsi
1416ee481: 0f b6 05 81 67 f4 01        	movzbl	0x1f46781(%rip), %eax   # 0x143634c09
1416ee488: b9 06 00 00 00              	movl	$0x6, %ecx
1416ee48d: ba 01 00 00 00              	movl	$0x1, %edx
1416ee492: e8 79 57 f2 fe              	callq	0x140613c10 <.text+0x612c10>
1416ee497: 48 85 c0                    	testq	%rax, %rax
1416ee49a: 0f 84 c8 08 00 00           	je	0x1416eed68 <.text+0x16edd68>
1416ee4a0: 66 c7 40 04 7a 65           	movw	$0x657a, 0x4(%rax)      # imm = 0x657A
1416ee4a6: c7 00 46 72 65 65           	movl	$0x65657246, (%rax)     # imm = 0x65657246
1416ee4ac: c6 45 40 00                 	movb	$0x0, 0x40(%rbp)
1416ee4b0: c7 45 45 00 00 00 00        	movl	$0x0, 0x45(%rbp)
1416ee4b7: c7 45 48 00 00 00 00        	movl	$0x0, 0x48(%rbp)
1416ee4be: 48 89 75 50                 	movq	%rsi, 0x50(%rbp)
1416ee4c2: 48 c7 45 e0 03 00 00 00     	movq	$0x3, -0x20(%rbp)
1416ee4ca: 48 c7 45 f8 06 00 00 00     	movq	$0x6, -0x8(%rbp)
1416ee4d2: 48 c7 45 10 06 00 00 00     	movq	$0x6, 0x10(%rbp)
1416ee4da: 48 89 45 18                 	movq	%rax, 0x18(%rbp)
1416ee4de: 48 c7 45 20 06 00 00 00     	movq	$0x6, 0x20(%rbp)
1416ee4e6: c7 45 28 00 00 00 00        	movl	$0x0, 0x28(%rbp)
1416ee4ed: c7 45 30 00 00 00 00        	movl	$0x0, 0x30(%rbp)
1416ee4f4: 48 c7 45 38 00 00 00 00     	movq	$0x0, 0x38(%rbp)
1416ee4fc: c6 45 58 00                 	movb	$0x0, 0x58(%rbp)
1416ee500: 48 8d 8d e0 01 00 00        	leaq	0x1e0(%rbp), %rcx
1416ee507: 48 8d 55 e0                 	leaq	-0x20(%rbp), %rdx
1416ee50b: 4d 89 e8                    	movq	%r13, %r8
1416ee50e: e8 1d a4 55 ff              	callq	0x140c48930 <.text+0xc47930>
1416ee513: 48 8b 85 f0 01 00 00        	movq	0x1f0(%rbp), %rax
1416ee51a: f0                          	lock
1416ee51b: 48 ff 08                    	decq	(%rax)
1416ee51e: 75 0c                       	jne	0x1416ee52c <.text+0x16ed52c>
1416ee520: 48 8d 8d f0 01 00 00        	leaq	0x1f0(%rbp), %rcx
1416ee527: e8 44 2c 55 ff              	callq	0x140c41170 <.text+0xc40170>
1416ee52c: 0f 10 77 28                 	movups	0x28(%rdi), %xmm6
1416ee530: 48 8b 77 38                 	movq	0x38(%rdi), %rsi
1416ee534: 0f b6 05 ce 66 f4 01        	movzbl	0x1f466ce(%rip), %eax   # 0x143634c09
1416ee53b: b9 18 00 00 00              	movl	$0x18, %ecx
1416ee540: ba 08 00 00 00              	movl	$0x8, %edx
1416ee545: e8 c6 56 f2 fe              	callq	0x140613c10 <.text+0x612c10>
1416ee54a: 48 85 c0                    	testq	%rax, %rax
1416ee54d: 0f 84 2b 08 00 00           	je	0x1416eed7e <.text+0x16edd7e>
1416ee553: 0f 11 30                    	movups	%xmm6, (%rax)
1416ee556: 48 89 70 10                 	movq	%rsi, 0x10(%rax)
1416ee55a: 48 8d 0d ff 1a 5a 01        	leaq	0x15a1aff(%rip), %rcx   # 0x142c90060
1416ee561: 48 89 4c 24 20              	movq	%rcx, 0x20(%rsp)
1416ee566: 48 8d 4d e0                 	leaq	-0x20(%rbp), %rcx
1416ee56a: 4c 89 ea                    	movq	%r13, %rdx
1416ee56d: 45 31 c0                    	xorl	%r8d, %r8d
1416ee570: 49 89 c1                    	movq	%rax, %r9
1416ee573: e8 e8 f6 3e ff              	callq	0x140addc60 <.text+0xadcc60>
1416ee578: 48 8b 45 f0                 	movq	-0x10(%rbp), %rax
1416ee57c: f0                          	lock
1416ee57d: 48 ff 08                    	decq	(%rax)
1416ee580: 75 09                       	jne	0x1416ee58b <.text+0x16ed58b>
1416ee582: 48 8d 4d f0                 	leaq	-0x10(%rbp), %rcx
1416ee586: e8 e5 2b 55 ff              	callq	0x140c41170 <.text+0xc40170>
1416ee58b: 4c 8b 77 08                 	movq	0x8(%rdi), %r14
1416ee58f: 0f b6 05 73 66 f4 01        	movzbl	0x1f46673(%rip), %eax   # 0x143634c09
1416ee596: b9 08 00 00 00              	movl	$0x8, %ecx
1416ee59b: ba 08 00 00 00              	movl	$0x8, %edx
1416ee5a0: e8 6b 56 f2 fe              	callq	0x140613c10 <.text+0x612c10>
1416ee5a5: 48 85 c0                    	testq	%rax, %rax
1416ee5a8: 0f 84 df 07 00 00           	je	0x1416eed8d <.text+0x16edd8d>
1416ee5ae: 4c 89 30                    	movq	%r14, (%rax)
1416ee5b1: 48 8d 0d 88 1a 5a 01        	leaq	0x15a1a88(%rip), %rcx   # 0x142c90040
1416ee5b8: 48 89 4c 24 20              	movq	%rcx, 0x20(%rsp)
1416ee5bd: 48 8d 4d e0                 	leaq	-0x20(%rbp), %rcx
1416ee5c1: 4c 89 ea                    	movq	%r13, %rdx
1416ee5c4: 45 31 c0                    	xorl	%r8d, %r8d
1416ee5c7: 49 89 c1                    	movq	%rax, %r9
1416ee5ca: e8 91 f6 3e ff              	callq	0x140addc60 <.text+0xadcc60>
1416ee5cf: 48 8b 45 f0                 	movq	-0x10(%rbp), %rax
1416ee5d3: f0                          	lock
1416ee5d4: 48 ff 08                    	decq	(%rax)
1416ee5d7: 75 09                       	jne	0x1416ee5e2 <.text+0x16ed5e2>
1416ee5d9: 48 8d 4d f0                 	leaq	-0x10(%rbp), %rcx
1416ee5dd: e8 8e 2b 55 ff              	callq	0x140c41170 <.text+0xc40170>
1416ee5e2: 48 8d 15 70 1e 5a 01        	leaq	0x15a1e70(%rip), %rdx   # 0x142c90459
1416ee5e9: 48 8d 5d e0                 	leaq	-0x20(%rbp), %rbx
1416ee5ed: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1416ee5f3: 48 89 d9                    	movq	%rbx, %rcx
1416ee5f6: e8 75 dd 2e ff              	callq	0x1409dc370 <.text+0x9db370>
1416ee5fb: 48 8d 8d e0 01 00 00        	leaq	0x1e0(%rbp), %rcx
1416ee602: 48 89 da                    	movq	%rbx, %rdx
1416ee605: 4d 89 e8                    	movq	%r13, %r8
1416ee608: e8 13 87 55 ff              	callq	0x140c46d20 <.text+0xc45d20>
1416ee60d: 80 bd 34 02 00 00 00        	cmpb	$0x0, 0x234(%rbp)
1416ee614: 74 22                       	je	0x1416ee638 <.text+0x16ed638>
1416ee616: 48 8b 85 f0 01 00 00        	movq	0x1f0(%rbp), %rax
1416ee61d: f0                          	lock
1416ee61e: 48 ff 08                    	decq	(%rax)
1416ee621: 75 0c                       	jne	0x1416ee62f <.text+0x16ed62f>
1416ee623: 48 8d 8d f0 01 00 00        	leaq	0x1f0(%rbp), %rcx
1416ee62a: e8 41 2b 55 ff              	callq	0x140c41170 <.text+0xc40170>
1416ee62f: 48 8b 47 10                 	movq	0x10(%rdi), %rax
1416ee633: c6 00 01                    	movb	$0x1, (%rax)
1416ee636: eb 3a                       	jmp	0x1416ee672 <.text+0x16ed672>
1416ee638: c6 45 e0 00                 	movb	$0x0, -0x20(%rbp)
1416ee63c: 80 bd 33 02 00 00 00        	cmpb	$0x0, 0x233(%rbp)
1416ee643: 74 14                       	je	0x1416ee659 <.text+0x16ed659>
1416ee645: 48 8d 8d f0 01 00 00        	leaq	0x1f0(%rbp), %rcx
1416ee64c: 48 8d 55 e0                 	leaq	-0x20(%rbp), %rdx
1416ee650: e8 1b 42 95 fe              	callq	0x140042870 <.text+0x41870>
1416ee655: 84 c0                       	testb	%al, %al
1416ee657: 75 bd                       	jne	0x1416ee616 <.text+0x16ed616>
1416ee659: 48 8b 85 f0 01 00 00        	movq	0x1f0(%rbp), %rax
1416ee660: f0                          	lock
1416ee661: 48 ff 08                    	decq	(%rax)
1416ee664: 75 0c                       	jne	0x1416ee672 <.text+0x16ed672>
1416ee666: 48 8d 8d f0 01 00 00        	leaq	0x1f0(%rbp), %rcx
1416ee66d: e8 fe 2a 55 ff              	callq	0x140c41170 <.text+0xc40170>
1416ee672: 48 8d 15 e8 1d 5a 01        	leaq	0x15a1de8(%rip), %rdx   # 0x142c90461
1416ee679: 41 b8 0c 00 00 00           	movl	$0xc, %r8d
1416ee67f: 48 89 d9                    	movq	%rbx, %rcx
1416ee682: e8 e9 dc 2e ff              	callq	0x1409dc370 <.text+0x9db370>
1416ee687: 48 8d 8d e0 01 00 00        	leaq	0x1e0(%rbp), %rcx
1416ee68e: 48 89 da                    	movq	%rbx, %rdx
1416ee691: 4d 89 e8                    	movq	%r13, %r8
1416ee694: e8 87 86 55 ff              	callq	0x140c46d20 <.text+0xc45d20>
1416ee699: 80 bd 34 02 00 00 00        	cmpb	$0x0, 0x234(%rbp)
1416ee6a0: 4c 89 b5 d0 01 00 00        	movq	%r14, 0x1d0(%rbp)
1416ee6a7: 74 22                       	je	0x1416ee6cb <.text+0x16ed6cb>
1416ee6a9: 48 8b 85 f0 01 00 00        	movq	0x1f0(%rbp), %rax
1416ee6b0: f0                          	lock
1416ee6b1: 48 ff 08                    	decq	(%rax)
1416ee6b4: 75 0c                       	jne	0x1416ee6c2 <.text+0x16ed6c2>
1416ee6b6: 48 8d 8d f0 01 00 00        	leaq	0x1f0(%rbp), %rcx
1416ee6bd: e8 ae 2a 55 ff              	callq	0x140c41170 <.text+0xc40170>
1416ee6c2: 48 8b 47 10                 	movq	0x10(%rdi), %rax
1416ee6c6: c6 00 02                    	movb	$0x2, (%rax)
1416ee6c9: eb 3a                       	jmp	0x1416ee705 <.text+0x16ed705>
1416ee6cb: c6 45 e0 00                 	movb	$0x0, -0x20(%rbp)
1416ee6cf: 80 bd 33 02 00 00 00        	cmpb	$0x0, 0x233(%rbp)
1416ee6d6: 74 14                       	je	0x1416ee6ec <.text+0x16ed6ec>
1416ee6d8: 48 8d 8d f0 01 00 00        	leaq	0x1f0(%rbp), %rcx
1416ee6df: 48 8d 55 e0                 	leaq	-0x20(%rbp), %rdx
1416ee6e3: e8 88 41 95 fe              	callq	0x140042870 <.text+0x41870>
1416ee6e8: 84 c0                       	testb	%al, %al
1416ee6ea: 75 bd                       	jne	0x1416ee6a9 <.text+0x16ed6a9>
1416ee6ec: 48 8b 85 f0 01 00 00        	movq	0x1f0(%rbp), %rax
1416ee6f3: f0                          	lock
1416ee6f4: 48 ff 08                    	decq	(%rax)
1416ee6f7: 75 0c                       	jne	0x1416ee705 <.text+0x16ed705>
1416ee6f9: 48 8d 8d f0 01 00 00        	leaq	0x1f0(%rbp), %rcx
1416ee700: e8 6b 2a 55 ff              	callq	0x140c41170 <.text+0xc40170>
1416ee705: 48 8b 0e                    	movq	(%rsi), %rcx
1416ee708: 4c 8b 66 08                 	movq	0x8(%rsi), %r12
1416ee70c: 4c 8b b1 f8 00 00 00        	movq	0xf8(%rcx), %r14
1416ee713: 48 8b 81 00 01 00 00        	movq	0x100(%rcx), %rax
1416ee71a: 49 8d 34 86                 	leaq	(%r14,%rax,4), %rsi
1416ee71e: 48 89 8d 60 02 00 00        	movq	%rcx, 0x260(%rbp)
1416ee725: 44 0f b6 99 08 01 00 00     	movzbl	0x108(%rcx), %r11d
1416ee72d: 48 8b 47 40                 	movq	0x40(%rdi), %rax
1416ee731: 48 89 85 48 02 00 00        	movq	%rax, 0x248(%rbp)
1416ee738: 48 8b 47 18                 	movq	0x18(%rdi), %rax
1416ee73c: 48 89 85 d8 01 00 00        	movq	%rax, 0x1d8(%rbp)
1416ee743: 48 8b 47 20                 	movq	0x20(%rdi), %rax
1416ee747: 48 89 85 58 02 00 00        	movq	%rax, 0x258(%rbp)
1416ee74e: b8 08 00 00 00              	movl	$0x8, %eax
1416ee753: 48 89 85 40 02 00 00        	movq	%rax, 0x240(%rbp)
1416ee75a: 48 c7 85 70 02 00 00 00 00 00 00    	movq	$0x0, 0x270(%rbp)
1416ee765: f2 0f 10 35 43 5e 24 01     	movsd	0x1245e43(%rip), %xmm6  # 0x1429345b0
1416ee76d: 0f 57 ff                    	xorps	%xmm7, %xmm7
1416ee770: f2 44 0f 10 05 67 71 23 01  	movsd	0x1237167(%rip), %xmm8  # 0x1429258e0
1416ee779: 48 c7 85 78 02 00 00 00 00 00 00    	movq	$0x0, 0x278(%rbp)
1416ee784: 45 31 c0                    	xorl	%r8d, %r8d
1416ee787: 31 ff                       	xorl	%edi, %edi
1416ee789: 45 31 c9                    	xorl	%r9d, %r9d
1416ee78c: 4c 89 ad 50 02 00 00        	movq	%r13, 0x250(%rbp)
1416ee793: 44 88 9d 97 02 00 00        	movb	%r11b, 0x297(%rbp)
1416ee79a: 45 84 db                    	testb	%r11b, %r11b
1416ee79d: 0f 84 bd 00 00 00           	je	0x1416ee860 <.text+0x16ed860>
1416ee7a3: 49 39 f9                    	cmpq	%rdi, %r9
1416ee7a6: 0f 85 9f 00 00 00           	jne	0x1416ee84b <.text+0x16ed84b>
1416ee7ac: 0f 1f 40 00                 	nopl	(%rax)
1416ee7b0: 49 39 f6                    	cmpq	%rsi, %r14
1416ee7b3: 0f 84 6d 05 00 00           	je	0x1416eed26 <.text+0x16edd26>
1416ee7b9: 41 8b 0e                    	movl	(%r14), %ecx
1416ee7bc: 49 83 c6 04                 	addq	$0x4, %r14
1416ee7c0: 49 8b 84 24 a8 01 00 00     	movq	0x1a8(%r12), %rax
1416ee7c8: 48 8d 0c c9                 	leaq	(%rcx,%rcx,8), %rcx
1416ee7cc: 48 8b 7c c8 10              	movq	0x10(%rax,%rcx,8), %rdi
1416ee7d1: 48 85 ff                    	testq	%rdi, %rdi
1416ee7d4: 74 da                       	je	0x1416ee7b0 <.text+0x16ed7b0>
1416ee7d6: 48 8d 04 c8                 	leaq	(%rax,%rcx,8), %rax
1416ee7da: 48 8b 48 18                 	movq	0x18(%rax), %rcx
1416ee7de: 48 8b 40 38                 	movq	0x38(%rax), %rax
1416ee7e2: 4c 8b 8d 60 02 00 00        	movq	0x260(%rbp), %r9
1416ee7e9: 49 8b 91 10 01 00 00        	movq	0x110(%r9), %rdx
1416ee7f0: 4d 8b 81 18 01 00 00        	movq	0x118(%r9), %r8
1416ee7f7: 48 8b 14 d0                 	movq	(%rax,%rdx,8), %rdx
1416ee7fb: 48 f7 d2                    	notq	%rdx
1416ee7fe: 48 8d 14 52                 	leaq	(%rdx,%rdx,2), %rdx
1416ee802: 48 c1 e2 04                 	shlq	$0x4, %rdx
1416ee806: 48 8b 54 11 10              	movq	0x10(%rcx,%rdx), %rdx
1416ee80b: 48 89 95 78 02 00 00        	movq	%rdx, 0x278(%rbp)
1416ee812: 4a 8b 14 c0                 	movq	(%rax,%r8,8), %rdx
1416ee816: 48 f7 d2                    	notq	%rdx
1416ee819: 48 8d 14 52                 	leaq	(%rdx,%rdx,2), %rdx
1416ee81d: 48 c1 e2 04                 	shlq	$0x4, %rdx
1416ee821: 48 8b 54 11 10              	movq	0x10(%rcx,%rdx), %rdx
1416ee826: 48 89 95 70 02 00 00        	movq	%rdx, 0x270(%rbp)
1416ee82d: 49 8b 91 20 01 00 00        	movq	0x120(%r9), %rdx
1416ee834: 48 8b 04 d0                 	movq	(%rax,%rdx,8), %rax
1416ee838: 48 f7 d0                    	notq	%rax
1416ee83b: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
1416ee83f: 48 c1 e0 04                 	shlq	$0x4, %rax
1416ee843: 4c 8b 44 01 10              	movq	0x10(%rcx,%rax), %r8
1416ee848: 45 31 c9                    	xorl	%r9d, %r9d
1416ee84b: 45 89 cf                    	movl	%r9d, %r15d
1416ee84e: e9 ed 00 00 00              	jmp	0x1416ee940 <.text+0x16ed940>
1416ee853: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
1416ee860: 49 39 f9                    	cmpq	%rdi, %r9
1416ee863: 0f 85 c4 00 00 00           	jne	0x1416ee92d <.text+0x16ed92d>
1416ee869: 0f 1f 80 00 00 00 00        	nopl	(%rax)
1416ee870: 49 39 f6                    	cmpq	%rsi, %r14
1416ee873: 0f 84 ad 04 00 00           	je	0x1416eed26 <.text+0x16edd26>
1416ee879: 41 8b 0e                    	movl	(%r14), %ecx
1416ee87c: 49 83 c6 04                 	addq	$0x4, %r14
1416ee880: 49 8b 84 24 00 01 00 00     	movq	0x100(%r12), %rax
1416ee888: 48 8d 0c 89                 	leaq	(%rcx,%rcx,4), %rcx
1416ee88c: 48 c1 e1 05                 	shlq	$0x5, %rcx
1416ee890: 48 8b 7c 08 58              	movq	0x58(%rax,%rcx), %rdi
1416ee895: 48 85 ff                    	testq	%rdi, %rdi
1416ee898: 74 d6                       	je	0x1416ee870 <.text+0x16ed870>
1416ee89a: 48 01 c8                    	addq	%rcx, %rax
1416ee89d: 8b 88 94 00 00 00           	movl	0x94(%rax), %ecx
1416ee8a3: 49 8b 94 24 a8 01 00 00     	movq	0x1a8(%r12), %rdx
1416ee8ab: 48 8d 0c c9                 	leaq	(%rcx,%rcx,8), %rcx
1416ee8af: 4c 8b 44 ca 18              	movq	0x18(%rdx,%rcx,8), %r8
1416ee8b4: 48 8b 4c ca 38              	movq	0x38(%rdx,%rcx,8), %rcx
1416ee8b9: 4c 8b 95 60 02 00 00        	movq	0x260(%rbp), %r10
1416ee8c0: 49 8b 92 10 01 00 00        	movq	0x110(%r10), %rdx
1416ee8c7: 4d 8b 8a 18 01 00 00        	movq	0x118(%r10), %r9
1416ee8ce: 48 8b 14 d1                 	movq	(%rcx,%rdx,8), %rdx
1416ee8d2: 48 f7 d2                    	notq	%rdx
1416ee8d5: 48 8d 14 52                 	leaq	(%rdx,%rdx,2), %rdx
1416ee8d9: 48 c1 e2 04                 	shlq	$0x4, %rdx
1416ee8dd: 49 8b 54 10 10              	movq	0x10(%r8,%rdx), %rdx
1416ee8e2: 48 89 95 78 02 00 00        	movq	%rdx, 0x278(%rbp)
1416ee8e9: 4a 8b 14 c9                 	movq	(%rcx,%r9,8), %rdx
1416ee8ed: 48 f7 d2                    	notq	%rdx
1416ee8f0: 48 8d 14 52                 	leaq	(%rdx,%rdx,2), %rdx
1416ee8f4: 48 c1 e2 04                 	shlq	$0x4, %rdx
1416ee8f8: 49 8b 54 10 10              	movq	0x10(%r8,%rdx), %rdx
1416ee8fd: 48 89 95 70 02 00 00        	movq	%rdx, 0x270(%rbp)
1416ee904: 49 8b 92 20 01 00 00        	movq	0x120(%r10), %rdx
1416ee90b: 48 8b 0c d1                 	movq	(%rcx,%rdx,8), %rcx
1416ee90f: 48 f7 d1                    	notq	%rcx
1416ee912: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
1416ee916: 48 c1 e1 04                 	shlq	$0x4, %rcx
1416ee91a: 4d 8b 44 08 10              	movq	0x10(%r8,%rcx), %r8
1416ee91f: 48 8b 40 50                 	movq	0x50(%rax), %rax
1416ee923: 48 89 85 40 02 00 00        	movq	%rax, 0x240(%rbp)
1416ee92a: 45 31 c9                    	xorl	%r9d, %r9d
1416ee92d: 4c 89 c8                    	movq	%r9, %rax
1416ee930: 48 c1 e0 04                 	shlq	$0x4, %rax
1416ee934: 48 8b 8d 40 02 00 00        	movq	0x240(%rbp), %rcx
1416ee93b: 44 8b 7c 01 08              	movl	0x8(%rcx,%rax), %r15d
1416ee940: 4b 8d 1c 7f                 	leaq	(%r15,%r15,2), %rbx
1416ee944: 48 c1 e3 04                 	shlq	$0x4, %rbx
1416ee948: 48 03 9d 78 02 00 00        	addq	0x278(%rbp), %rbx
1416ee94f: 49 6b c7 68                 	imulq	$0x68, %r15, %rax
1416ee953: 48 03 85 70 02 00 00        	addq	0x270(%rbp), %rax
1416ee95a: 49 c1 e7 04                 	shlq	$0x4, %r15
1416ee95e: 4d 01 c7                    	addq	%r8, %r15
1416ee961: 49 ff c1                    	incq	%r9
1416ee964: 48 8b 48 20                 	movq	0x20(%rax), %rcx
1416ee968: 48 8b 95 d0 01 00 00        	movq	0x1d0(%rbp), %rdx
1416ee96f: 48 3b 0a                    	cmpq	(%rdx), %rcx
1416ee972: 0f 85 22 fe ff ff           	jne	0x1416ee79a <.text+0x16ed79a>
1416ee978: 4c 89 8d 88 02 00 00        	movq	%r9, 0x288(%rbp)
1416ee97f: 4c 89 85 80 02 00 00        	movq	%r8, 0x280(%rbp)
1416ee986: 48 83 c0 30                 	addq	$0x30, %rax
1416ee98a: 48 89 85 c0 01 00 00        	movq	%rax, 0x1c0(%rbp)
1416ee991: 48 8d 05 68 3e 00 00        	leaq	0x3e68(%rip), %rax      # 0x1416f2800 <.text+0x16f1800>
1416ee998: 48 89 85 c8 01 00 00        	movq	%rax, 0x1c8(%rbp)
1416ee99f: 48 c7 45 a8 02 00 00 00     	movq	$0x2, -0x58(%rbp)
1416ee9a7: 48 c7 45 b8 02 00 00 00     	movq	$0x2, -0x48(%rbp)
1416ee9af: 48 c7 45 c8 00 00 00 00     	movq	$0x0, -0x38(%rbp)
1416ee9b7: 48 b8 20 00 00 00 04 00 00 00       	movabsq	$0x400000020, %rax # imm = 0x400000020
1416ee9c1: 48 89 45 d0                 	movq	%rax, -0x30(%rbp)
1416ee9c5: c6 45 d8 03                 	movb	$0x3, -0x28(%rbp)
1416ee9c9: 48 8d 05 a0 1a 5a 01        	leaq	0x15a1aa0(%rip), %rax   # 0x142c90470
1416ee9d0: 48 89 85 90 01 00 00        	movq	%rax, 0x190(%rbp)
1416ee9d7: 48 c7 85 98 01 00 00 01 00 00 00    	movq	$0x1, 0x198(%rbp)
1416ee9e2: 48 8d 85 c0 01 00 00        	leaq	0x1c0(%rbp), %rax
1416ee9e9: 48 89 85 a0 01 00 00        	movq	%rax, 0x1a0(%rbp)
1416ee9f0: 48 c7 85 a8 01 00 00 01 00 00 00    	movq	$0x1, 0x1a8(%rbp)
1416ee9fb: 48 8d 45 a8                 	leaq	-0x58(%rbp), %rax
1416ee9ff: 48 89 85 b0 01 00 00        	movq	%rax, 0x1b0(%rbp)
1416eea06: 48 c7 85 b8 01 00 00 01 00 00 00    	movq	$0x1, 0x1b8(%rbp)
1416eea11: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
1416eea15: 48 8d 95 90 01 00 00        	leaq	0x190(%rbp), %rdx
1416eea1c: e8 6f b5 68 ff              	callq	0x140d79f90 <.text+0xd78f90>
1416eea21: c6 45 40 00                 	movb	$0x0, 0x40(%rbp)
1416eea25: 48 8d 45 45                 	leaq	0x45(%rbp), %rax
1416eea29: c7 40 03 00 00 00 00        	movl	$0x0, 0x3(%rax)
1416eea30: c7 00 00 00 00 00           	movl	$0x0, (%rax)
1416eea36: 48 c7 45 e0 03 00 00 00     	movq	$0x3, -0x20(%rbp)
1416eea3e: 48 c7 45 f8 06 00 00 00     	movq	$0x6, -0x8(%rbp)
1416eea46: c7 45 28 00 00 00 00        	movl	$0x0, 0x28(%rbp)
1416eea4d: c7 45 30 00 00 00 00        	movl	$0x0, 0x30(%rbp)
1416eea54: 48 c7 45 38 00 00 00 00     	movq	$0x0, 0x38(%rbp)
1416eea5c: 66 c7 45 50 00 02           	movw	$0x200, 0x50(%rbp)      # imm = 0x200
1416eea62: c6 45 52 02                 	movb	$0x2, 0x52(%rbp)
1416eea66: c6 45 55 02                 	movb	$0x2, 0x55(%rbp)
1416eea6a: 48 8d 8d e0 01 00 00        	leaq	0x1e0(%rbp), %rcx
1416eea71: 48 8d 55 e0                 	leaq	-0x20(%rbp), %rdx
1416eea75: 4d 89 e8                    	movq	%r13, %r8
1416eea78: e8 c3 a2 53 ff              	callq	0x140c28d40 <.text+0xc27d40>
1416eea7d: 48 8b 85 f0 01 00 00        	movq	0x1f0(%rbp), %rax
1416eea84: f0                          	lock
1416eea85: 48 ff 08                    	decq	(%rax)
1416eea88: 75 0c                       	jne	0x1416eea96 <.text+0x16eda96>
1416eea8a: 48 8d 8d f0 01 00 00        	leaq	0x1f0(%rbp), %rcx
1416eea91: e8 da 26 55 ff              	callq	0x140c41170 <.text+0xc40170>
1416eea96: 48 8b 85 d8 01 00 00        	movq	0x1d8(%rbp), %rax
1416eea9d: 4c 8b 10                    	movq	(%rax), %r10
1416eeaa0: 48 8b 8d 48 02 00 00        	movq	0x248(%rbp), %rcx
1416eeaa7: 4c 8b 69 08                 	movq	0x8(%rcx), %r13
1416eeaab: 4d 85 ed                    	testq	%r13, %r13
1416eeaae: 75 04                       	jne	0x1416eeab4 <.text+0x16edab4>
1416eeab0: 4c 8b 69 10                 	movq	0x10(%rcx), %r13
1416eeab4: 0f 28 03                    	movaps	(%rbx), %xmm0
1416eeab7: 0f 28 4b 10                 	movaps	0x10(%rbx), %xmm1
1416eeabb: 0f 28 53 20                 	movaps	0x20(%rbx), %xmm2
1416eeabf: 0f 29 55 00                 	movaps	%xmm2, (%rbp)
1416eeac3: 0f 29 4d f0                 	movaps	%xmm1, -0x10(%rbp)
1416eeac7: 0f 29 45 e0                 	movaps	%xmm0, -0x20(%rbp)
1416eeacb: 49 8b 45 00                 	movq	(%r13), %rax
1416eeacf: 49 8b 5d 10                 	movq	0x10(%r13), %rbx
1416eead3: 48 29 d8                    	subq	%rbx, %rax
1416eead6: 48 83 f8 67                 	cmpq	$0x67, %rax
1416eeada: 44 0f b6 9d 97 02 00 00     	movzbl	0x297(%rbp), %r11d
1416eeae2: 4c 8b 85 80 02 00 00        	movq	0x280(%rbp), %r8
1416eeae9: 4c 8b 8d 88 02 00 00        	movq	0x288(%rbp), %r9
1416eeaf0: 0f 86 80 01 00 00           	jbe	0x1416eec76 <.text+0x16edc76>
1416eeaf6: 49 8b 45 08                 	movq	0x8(%r13), %rax
1416eeafa: 48 8d 15 bf cd be ff        	leaq	-0x413241(%rip), %rdx   # 0x1412db8c0 <.text+0x12da8c0>
1416eeb01: 48 89 14 18                 	movq	%rdx, (%rax,%rbx)
1416eeb05: 0f 28 45 e0                 	movaps	-0x20(%rbp), %xmm0
1416eeb09: 0f 28 4d f0                 	movaps	-0x10(%rbp), %xmm1
1416eeb0d: 0f 28 55 00                 	movaps	(%rbp), %xmm2
1416eeb11: 0f 11 44 18 08              	movups	%xmm0, 0x8(%rax,%rbx)
1416eeb16: 0f 11 4c 18 18              	movups	%xmm1, 0x18(%rax,%rbx)
1416eeb1b: 0f 11 54 18 28              	movups	%xmm2, 0x28(%rax,%rbx)
1416eeb20: c6 44 18 38 00              	movb	$0x0, 0x38(%rax,%rbx)
1416eeb25: 4c 89 54 18 48              	movq	%r10, 0x48(%rax,%rbx)
1416eeb2a: 48 8d 15 ff 61 e7 ff        	leaq	-0x189e01(%rip), %rdx   # 0x141564d30 <.text+0x1563d30>
1416eeb31: 48 89 54 18 58              	movq	%rdx, 0x58(%rax,%rbx)
1416eeb36: 48 83 c3 68                 	addq	$0x68, %rbx
1416eeb3a: 49 89 5d 10                 	movq	%rbx, 0x10(%r13)
1416eeb3e: 41 83 3f 01                 	cmpl	$0x1, (%r15)
1416eeb42: 0f 85 ce 00 00 00           	jne	0x1416eec16 <.text+0x16edc16>
1416eeb48: f2 45 0f 10 4f 04           	movsd	0x4(%r15), %xmm9
1416eeb4e: 4c 8b 69 08                 	movq	0x8(%rcx), %r13
1416eeb52: 44 0f 58 ce                 	addps	%xmm6, %xmm9
1416eeb56: 4d 85 ed                    	testq	%r13, %r13
1416eeb59: 75 04                       	jne	0x1416eeb5f <.text+0x16edb5f>
1416eeb5b: 4c 8b 69 10                 	movq	0x10(%rcx), %r13
1416eeb5f: 48 8b 85 58 02 00 00        	movq	0x258(%rbp), %rax
1416eeb66: 48 8b 00                    	movq	(%rax), %rax
1416eeb69: 48 89 85 68 02 00 00        	movq	%rax, 0x268(%rbp)
1416eeb70: f3 45 0f 10 57 0c           	movss	0xc(%r15), %xmm10
1416eeb76: f3 44 0f 58 d7              	addss	%xmm7, %xmm10
1416eeb7b: 48 8d 4d e0                 	leaq	-0x20(%rbp), %rcx
1416eeb7f: e8 fc e5 5a ff              	callq	0x140c9d180 <.text+0xc9c180>
1416eeb84: 44 0f 28 5d e0              	movaps	-0x20(%rbp), %xmm11
1416eeb89: 49 8b 45 00                 	movq	(%r13), %rax
1416eeb8d: 49 8b 5d 10                 	movq	0x10(%r13), %rbx
1416eeb91: 48 29 d8                    	subq	%rbx, %rax
1416eeb94: 48 83 f8 67                 	cmpq	$0x67, %rax
1416eeb98: 0f 86 28 01 00 00           	jbe	0x1416eecc6 <.text+0x16edcc6>
1416eeb9e: 49 8b 45 08                 	movq	0x8(%r13), %rax
1416eeba2: 48 8d 0d 17 cd be ff        	leaq	-0x4132e9(%rip), %rcx   # 0x1412db8c0 <.text+0x12da8c0>
1416eeba9: 48 89 0c 18                 	movq	%rcx, (%rax,%rbx)
1416eebad: 44 0f 11 5c 18 08           	movups	%xmm11, 0x8(%rax,%rbx)
1416eebb3: 44 0f 13 4c 18 18           	movlps	%xmm9, 0x18(%rax,%rbx)
1416eebb9: f3 44 0f 11 54 18 20        	movss	%xmm10, 0x20(%rax,%rbx)
1416eebc0: f2 44 0f 11 44 18 24        	movsd	%xmm8, 0x24(%rax,%rbx)
1416eebc7: c7 44 18 2c 00 00 00 3f     	movl	$0x3f000000, 0x2c(%rax,%rbx) # imm = 0x3F000000
1416eebcf: c6 44 18 38 00              	movb	$0x0, 0x38(%rax,%rbx)
1416eebd4: 48 8b 8d 68 02 00 00        	movq	0x268(%rbp), %rcx
1416eebdb: 48 89 4c 18 48              	movq	%rcx, 0x48(%rax,%rbx)
1416eebe0: 48 8d 0d 49 61 e7 ff        	leaq	-0x189eb7(%rip), %rcx   # 0x141564d30 <.text+0x1563d30>
1416eebe7: 48 89 4c 18 58              	movq	%rcx, 0x58(%rax,%rbx)
1416eebec: 48 83 c3 68                 	addq	$0x68, %rbx
1416eebf0: 49 89 5d 10                 	movq	%rbx, 0x10(%r13)
1416eebf4: 4c 8b ad 50 02 00 00        	movq	0x250(%rbp), %r13
1416eebfb: 44 0f b6 9d 97 02 00 00     	movzbl	0x297(%rbp), %r11d
1416eec03: 4c 8b 85 80 02 00 00        	movq	0x280(%rbp), %r8
1416eec0a: 4c 8b 8d 88 02 00 00        	movq	0x288(%rbp), %r9
1416eec11: e9 84 fb ff ff              	jmp	0x1416ee79a <.text+0x16ed79a>
1416eec16: 4c 8b 69 08                 	movq	0x8(%rcx), %r13
1416eec1a: 4d 85 ed                    	testq	%r13, %r13
1416eec1d: 75 04                       	jne	0x1416eec23 <.text+0x16edc23>
1416eec1f: 4c 8b 69 10                 	movq	0x10(%rcx), %r13
1416eec23: 48 8b 85 58 02 00 00        	movq	0x258(%rbp), %rax
1416eec2a: 4c 8b 38                    	movq	(%rax), %r15
1416eec2d: 49 8b 45 00                 	movq	(%r13), %rax
1416eec31: 49 8b 5d 10                 	movq	0x10(%r13), %rbx
1416eec35: 48 29 d8                    	subq	%rbx, %rax
1416eec38: 48 83 f8 17                 	cmpq	$0x17, %rax
1416eec3c: 0f 86 a9 00 00 00           	jbe	0x1416eeceb <.text+0x16edceb>
1416eec42: 49 8b 45 08                 	movq	0x8(%r13), %rax
1416eec46: 48 8d 0d 23 e2 be ff        	leaq	-0x411ddd(%rip), %rcx   # 0x1412dce70 <.text+0x12dbe70>
1416eec4d: 48 89 0c 18                 	movq	%rcx, (%rax,%rbx)
1416eec51: 48 8d 0d 78 ef be ff        	leaq	-0x411088(%rip), %rcx   # 0x1412ddbd0 <.text+0x12dcbd0>
1416eec58: 48 89 4c 18 08              	movq	%rcx, 0x8(%rax,%rbx)
1416eec5d: 4c 89 7c 18 10              	movq	%r15, 0x10(%rax,%rbx)
1416eec62: 48 83 c3 18                 	addq	$0x18, %rbx
1416eec66: 49 89 5d 10                 	movq	%rbx, 0x10(%r13)
1416eec6a: 4c 8b ad 50 02 00 00        	movq	0x250(%rbp), %r13
1416eec71: e9 24 fb ff ff              	jmp	0x1416ee79a <.text+0x16ed79a>
1416eec76: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1416eec7f: 41 b8 68 00 00 00           	movl	$0x68, %r8d
1416eec85: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1416eec8b: 4c 89 e9                    	movq	%r13, %rcx
1416eec8e: 48 89 da                    	movq	%rbx, %rdx
1416eec91: 4c 89 95 68 02 00 00        	movq	%r10, 0x268(%rbp)
1416eec98: e8 c3 ad 1b 01              	callq	0x1428a9a60 <.text+0x28a8a60>
1416eec9d: 4c 8b 95 68 02 00 00        	movq	0x268(%rbp), %r10
1416eeca4: 4c 8b 8d 88 02 00 00        	movq	0x288(%rbp), %r9
1416eecab: 4c 8b 85 80 02 00 00        	movq	0x280(%rbp), %r8
1416eecb2: 48 8b 8d 48 02 00 00        	movq	0x248(%rbp), %rcx
1416eecb9: 44 0f b6 9d 97 02 00 00     	movzbl	0x297(%rbp), %r11d
1416eecc1: e9 30 fe ff ff              	jmp	0x1416eeaf6 <.text+0x16edaf6>
1416eecc6: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1416eeccf: 41 b8 68 00 00 00           	movl	$0x68, %r8d
1416eecd5: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1416eecdb: 4c 89 e9                    	movq	%r13, %rcx
1416eecde: 48 89 da                    	movq	%rbx, %rdx
1416eece1: e8 7a ad 1b 01              	callq	0x1428a9a60 <.text+0x28a8a60>
1416eece6: e9 b3 fe ff ff              	jmp	0x1416eeb9e <.text+0x16edb9e>
1416eeceb: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1416eecf4: 41 b8 18 00 00 00           	movl	$0x18, %r8d
1416eecfa: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1416eed00: 4c 89 e9                    	movq	%r13, %rcx
1416eed03: 48 89 da                    	movq	%rbx, %rdx
1416eed06: e8 55 ad 1b 01              	callq	0x1428a9a60 <.text+0x28a8a60>
1416eed0b: 4c 8b 8d 88 02 00 00        	movq	0x288(%rbp), %r9
1416eed12: 4c 8b 85 80 02 00 00        	movq	0x280(%rbp), %r8
1416eed19: 44 0f b6 9d 97 02 00 00     	movzbl	0x297(%rbp), %r11d
1416eed21: e9 1c ff ff ff              	jmp	0x1416eec42 <.text+0x16edc42>
1416eed26: 0f 28 b5 a0 02 00 00        	movaps	0x2a0(%rbp), %xmm6
1416eed2d: 0f 28 bd b0 02 00 00        	movaps	0x2b0(%rbp), %xmm7
1416eed34: 44 0f 28 85 c0 02 00 00     	movaps	0x2c0(%rbp), %xmm8
1416eed3c: 44 0f 28 8d d0 02 00 00     	movaps	0x2d0(%rbp), %xmm9
1416eed44: 44 0f 28 95 e0 02 00 00     	movaps	0x2e0(%rbp), %xmm10
1416eed4c: 44 0f 28 9d f0 02 00 00     	movaps	0x2f0(%rbp), %xmm11
1416eed54: 48 81 c4 88 03 00 00        	addq	$0x388, %rsp            # imm = 0x388
1416eed5b: 5b                          	popq	%rbx
1416eed5c: 5f                          	popq	%rdi
1416eed5d: 5e                          	popq	%rsi
1416eed5e: 41 5c                       	popq	%r12
1416eed60: 41 5d                       	popq	%r13
1416eed62: 41 5e                       	popq	%r14
1416eed64: 41 5f                       	popq	%r15
1416eed66: 5d                          	popq	%rbp
1416eed67: c3                          	retq
1416eed68: 4c 8d 05 81 13 5a 01        	leaq	0x15a1381(%rip), %r8    # 0x142c900f0
1416eed6f: b9 01 00 00 00              	movl	$0x1, %ecx
1416eed74: ba 06 00 00 00              	movl	$0x6, %edx
1416eed79: e8 65 a2 1e 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
1416eed7e: b9 08 00 00 00              	movl	$0x8, %ecx
1416eed83: ba 18 00 00 00              	movl	$0x18, %edx
1416eed88: e8 73 a2 1e 01              	callq	0x1428d9000 <.text+0x28d8000>
1416eed8d: b9 08 00 00 00              	movl	$0x8, %ecx
1416eed92: ba 08 00 00 00              	movl	$0x8, %edx
1416eed97: e8 64 a2 1e 01              	callq	0x1428d9000 <.text+0x28d8000>
1416eed9c: cc                          	int3
1416eed9d: 0f 1f 00                    	nopl	(%rax)
1416eeda0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1416eeda5: 55                          	pushq	%rbp
1416eeda6: 41 57                       	pushq	%r15
1416eeda8: 41 56                       	pushq	%r14
1416eedaa: 41 55                       	pushq	%r13
1416eedac: 41 54                       	pushq	%r12
1416eedae: 56                          	pushq	%rsi
1416eedaf: 57                          	pushq	%rdi
1416eedb0: 53                          	pushq	%rbx
1416eedb1: 48 81 ec 88 00 00 00        	subq	$0x88, %rsp
1416eedb8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1416eedbf: 44 0f 29 5c 24 20           	movaps	%xmm11, 0x20(%rsp)
1416eedc5: 44 0f 29 54 24 30           	movaps	%xmm10, 0x30(%rsp)
1416eedcb: 44 0f 29 4c 24 40           	movaps	%xmm9, 0x40(%rsp)
1416eedd1: 44 0f 29 44 24 50           	movaps	%xmm8, 0x50(%rsp)
1416eedd7: 0f 29 7c 24 60              	movaps	%xmm7, 0x60(%rsp)
1416eeddc: 0f 29 74 24 70              	movaps	%xmm6, 0x70(%rsp)
1416eede1: 48 8d 8d f0 01 00 00        	leaq	0x1f0(%rbp), %rcx
1416eede8: 48 8b 85 f0 01 00 00        	movq	0x1f0(%rbp), %rax
1416eedef: f0                          	lock
1416eedf0: 48 ff 08                    	decq	(%rax)
1416eedf3: 75 05                       	jne	0x1416eedfa <.text+0x16eddfa>
1416eedf5: e8 76 23 55 ff              	callq	0x140c41170 <.text+0xc40170>
1416eedfa: 0f 28 74 24 70              	movaps	0x70(%rsp), %xmm6
1416eedff: 0f 28 7c 24 60              	movaps	0x60(%rsp), %xmm7
1416eee04: 44 0f 28 44 24 50           	movaps	0x50(%rsp), %xmm8
1416eee0a: 44 0f 28 4c 24 40           	movaps	0x40(%rsp), %xmm9
1416eee10: 44 0f 28 54 24 30           	movaps	0x30(%rsp), %xmm10
1416eee16: 44 0f 28 5c 24 20           	movaps	0x20(%rsp), %xmm11
1416eee1c: 48 81 c4 88 00 00 00        	addq	$0x88, %rsp
1416eee23: 5b                          	popq	%rbx
1416eee24: 5f                          	popq	%rdi
1416eee25: 5e                          	popq	%rsi
1416eee26: 41 5c                       	popq	%r12
1416eee28: 41 5d                       	popq	%r13
1416eee2a: 41 5e                       	popq	%r14
1416eee2c: 41 5f                       	popq	%r15
1416eee2e: 5d                          	popq	%rbp
1416eee2f: c3                          	retq
1416eee30: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1416eee35: 55                          	pushq	%rbp
1416eee36: 41 57                       	pushq	%r15
1416eee38: 41 56                       	pushq	%r14
1416eee3a: 41 55                       	pushq	%r13
1416eee3c: 41 54                       	pushq	%r12
1416eee3e: 56                          	pushq	%rsi
1416eee3f: 57                          	pushq	%rdi
1416eee40: 53                          	pushq	%rbx
1416eee41: 48 81 ec 88 00 00 00        	subq	$0x88, %rsp
1416eee48: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1416eee4f: 44 0f 29 5c 24 20           	movaps	%xmm11, 0x20(%rsp)
1416eee55: 44 0f 29 54 24 30           	movaps	%xmm10, 0x30(%rsp)
1416eee5b: 44 0f 29 4c 24 40           	movaps	%xmm9, 0x40(%rsp)
1416eee61: 44 0f 29 44 24 50           	movaps	%xmm8, 0x50(%rsp)
1416eee67: 0f 29 7c 24 60              	movaps	%xmm7, 0x60(%rsp)
1416eee6c: 0f 29 74 24 70              	movaps	%xmm6, 0x70(%rsp)
1416eee71: 48 8d 8d f0 01 00 00        	leaq	0x1f0(%rbp), %rcx
1416eee78: 48 8b 85 f0 01 00 00        	movq	0x1f0(%rbp), %rax
1416eee7f: f0                          	lock
1416eee80: 48 ff 08                    	decq	(%rax)
1416eee83: 75 05                       	jne	0x1416eee8a <.text+0x16ede8a>
1416eee85: e8 e6 22 55 ff              	callq	0x140c41170 <.text+0xc40170>
1416eee8a: 0f 28 74 24 70              	movaps	0x70(%rsp), %xmm6
1416eee8f: 0f 28 7c 24 60              	movaps	0x60(%rsp), %xmm7
1416eee94: 44 0f 28 44 24 50           	movaps	0x50(%rsp), %xmm8
1416eee9a: 44 0f 28 4c 24 40           	movaps	0x40(%rsp), %xmm9
1416eeea0: 44 0f 28 54 24 30           	movaps	0x30(%rsp), %xmm10
1416eeea6: 44 0f 28 5c 24 20           	movaps	0x20(%rsp), %xmm11
1416eeeac: 48 81 c4 88 00 00 00        	addq	$0x88, %rsp
1416eeeb3: 5b                          	popq	%rbx
1416eeeb4: 5f                          	popq	%rdi
1416eeeb5: 5e                          	popq	%rsi
1416eeeb6: 41 5c                       	popq	%r12
1416eeeb8: 41 5d                       	popq	%r13
1416eeeba: 41 5e                       	popq	%r14
1416eeebc: 41 5f                       	popq	%r15
1416eeebe: 5d                          	popq	%rbp
1416eeebf: c3                          	retq
