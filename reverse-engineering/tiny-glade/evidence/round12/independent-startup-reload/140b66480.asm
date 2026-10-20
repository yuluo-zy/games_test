
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140b66480: 55                          	pushq	%rbp
140b66481: 56                          	pushq	%rsi
140b66482: 53                          	pushq	%rbx
140b66483: 48 81 ec 40 01 00 00        	subq	$0x140, %rsp            # imm = 0x140
140b6648a: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140b66492: 48 c7 85 b8 00 00 00 fe ff ff ff    	movq	$-0x2, 0xb8(%rbp)
140b6649d: 4c 89 85 a8 00 00 00        	movq	%r8, 0xa8(%rbp)
140b664a4: 88 d3                       	movb	%dl, %bl
140b664a6: 48 89 ce                    	movq	%rcx, %rsi
140b664a9: 48 83 c1 38                 	addq	$0x38, %rcx
140b664ad: c6 85 b7 00 00 00 01        	movb	$0x1, 0xb7(%rbp)
140b664b4: 48 ba 84 a7 2b 96 51 d2 1b 8e       	movabsq	$-0x71e42dae69d4587c, %rdx # imm = 0x8E1BD251962BA784
140b664be: 49 b8 9b 52 e6 c0 74 23 78 6a       	movabsq	$0x6a782374c0e6529b, %r8 # imm = 0x6A782374C0E6529B
140b664c8: e8 23 72 57 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
140b664cd: 48 85 c0                    	testq	%rax, %rax
140b664d0: 0f 84 20 01 00 00           	je	0x140b665f6 <.text+0xb655f6>
140b664d6: 48 39 96 18 02 00 00        	cmpq	%rdx, 0x218(%rsi)
140b664dd: 0f 86 13 01 00 00           	jbe	0x140b665f6 <.text+0xb655f6>
140b664e3: 48 8b 86 10 02 00 00        	movq	0x210(%rsi), %rax
140b664ea: 48 8b 0c d0                 	movq	(%rax,%rdx,8), %rcx
140b664ee: 48 85 c9                    	testq	%rcx, %rcx
140b664f1: 0f 84 ff 00 00 00           	je	0x140b665f6 <.text+0xb655f6>
140b664f7: 48 8b 86 e0 01 00 00        	movq	0x1e0(%rsi), %rax
140b664fe: 48 f7 d1                    	notq	%rcx
140b66501: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
140b66505: 48 c1 e1 05                 	shlq	$0x5, %rcx
140b66509: 48 83 7c 08 20 00           	cmpq	$0x0, 0x20(%rax,%rcx)
140b6650f: 0f 84 e1 00 00 00           	je	0x140b665f6 <.text+0xb655f6>
140b66515: 48 01 c8                    	addq	%rcx, %rax
140b66518: 48 8b 48 10                 	movq	0x10(%rax), %rcx
140b6651c: 8b 96 b0 05 00 00           	movl	0x5b0(%rsi), %edx
140b66522: 89 50 5c                    	movl	%edx, 0x5c(%rax)
140b66525: 48 8b 85 a8 00 00 00        	movq	0xa8(%rbp), %rax
140b6652c: 0f 10 40 60                 	movups	0x60(%rax), %xmm0
140b66530: 0f 29 45 70                 	movaps	%xmm0, 0x70(%rbp)
140b66534: 0f 10 40 50                 	movups	0x50(%rax), %xmm0
140b66538: 0f 29 45 60                 	movaps	%xmm0, 0x60(%rbp)
140b6653c: 0f 10 40 40                 	movups	0x40(%rax), %xmm0
140b66540: 0f 29 45 50                 	movaps	%xmm0, 0x50(%rbp)
140b66544: 0f 10 00                    	movups	(%rax), %xmm0
140b66547: 0f 10 48 10                 	movups	0x10(%rax), %xmm1
140b6654b: 0f 10 50 20                 	movups	0x20(%rax), %xmm2
140b6654f: 0f 10 58 30                 	movups	0x30(%rax), %xmm3
140b66553: 0f 29 5d 40                 	movaps	%xmm3, 0x40(%rbp)
140b66557: 0f 29 55 30                 	movaps	%xmm2, 0x30(%rbp)
140b6655b: 0f 29 4d 20                 	movaps	%xmm1, 0x20(%rbp)
140b6655f: 0f 29 45 10                 	movaps	%xmm0, 0x10(%rbp)
140b66563: c6 85 b6 00 00 00 01        	movb	$0x1, 0xb6(%rbp)
140b6656a: 89 da                       	movl	%ebx, %edx
140b6656c: e8 8f b2 b3 00              	callq	0x1416a1800 <.text+0x16a0800>
140b66571: 0f 28 45 70                 	movaps	0x70(%rbp), %xmm0
140b66575: 0f 29 45 00                 	movaps	%xmm0, (%rbp)
140b66579: 0f 28 45 60                 	movaps	0x60(%rbp), %xmm0
140b6657d: 0f 29 45 f0                 	movaps	%xmm0, -0x10(%rbp)
140b66581: 0f 28 45 50                 	movaps	0x50(%rbp), %xmm0
140b66585: 0f 29 45 e0                 	movaps	%xmm0, -0x20(%rbp)
140b66589: 0f 28 45 10                 	movaps	0x10(%rbp), %xmm0
140b6658d: 0f 28 4d 20                 	movaps	0x20(%rbp), %xmm1
140b66591: 0f 28 55 30                 	movaps	0x30(%rbp), %xmm2
140b66595: 0f 28 5d 40                 	movaps	0x40(%rbp), %xmm3
140b66599: 0f 29 5d d0                 	movaps	%xmm3, -0x30(%rbp)
140b6659d: 0f 29 55 c0                 	movaps	%xmm2, -0x40(%rbp)
140b665a1: 0f 29 4d b0                 	movaps	%xmm1, -0x50(%rbp)
140b665a5: 0f 29 45 a0                 	movaps	%xmm0, -0x60(%rbp)
140b665a9: c6 85 b6 00 00 00 00        	movb	$0x0, 0xb6(%rbp)
140b665b0: 48 8d 8d 88 00 00 00        	leaq	0x88(%rbp), %rcx
140b665b7: 4c 8d 45 a0                 	leaq	-0x60(%rbp), %r8
140b665bb: 48 89 c2                    	movq	%rax, %rdx
140b665be: 45 31 c9                    	xorl	%r9d, %r9d
140b665c1: e8 6a 87 b3 00              	callq	0x14169ed30 <.text+0x169dd30>
140b665c6: 48 8b 95 88 00 00 00        	movq	0x88(%rbp), %rdx
140b665cd: 48 85 d2                    	testq	%rdx, %rdx
140b665d0: 74 16                       	je	0x140b665e8 <.text+0xb655e8>
140b665d2: 48 8b 8d 90 00 00 00        	movq	0x90(%rbp), %rcx
140b665d9: 48 c1 e2 04                 	shlq	$0x4, %rdx
140b665dd: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140b665e3: e8 38 d6 aa ff              	callq	0x140613c20 <.text+0x612c20>
140b665e8: 48 89 f0                    	movq	%rsi, %rax
140b665eb: 48 81 c4 40 01 00 00        	addq	$0x140, %rsp            # imm = 0x140
140b665f2: 5b                          	popq	%rbx
140b665f3: 5e                          	popq	%rsi
140b665f4: 5d                          	popq	%rbp
140b665f5: c3                          	retq
140b665f6: 48 8d 05 c3 c6 f8 01        	leaq	0x1f8c6c3(%rip), %rax   # 0x142af2cc0
140b665fd: 48 89 45 10                 	movq	%rax, 0x10(%rbp)
140b66601: 48 c7 45 18 27 00 00 00     	movq	$0x27, 0x18(%rbp)
140b66609: 48 8d 45 10                 	leaq	0x10(%rbp), %rax
140b6660d: 48 89 85 88 00 00 00        	movq	%rax, 0x88(%rbp)
140b66614: 48 8d 05 75 38 5a ff        	leaq	-0xa5c78b(%rip), %rax   # 0x140109e90 <.text+0x108e90>
140b6661b: 48 89 85 90 00 00 00        	movq	%rax, 0x90(%rbp)
140b66622: 48 8d 05 c7 c7 f8 01        	leaq	0x1f8c7c7(%rip), %rax   # 0x142af2df0
140b66629: 48 89 45 a0                 	movq	%rax, -0x60(%rbp)
140b6662d: 48 c7 45 a8 02 00 00 00     	movq	$0x2, -0x58(%rbp)
140b66635: 48 c7 45 c0 00 00 00 00     	movq	$0x0, -0x40(%rbp)
140b6663d: 48 8d 85 88 00 00 00        	leaq	0x88(%rbp), %rax
140b66644: 48 89 45 b0                 	movq	%rax, -0x50(%rbp)
140b66648: 48 c7 45 b8 01 00 00 00     	movq	$0x1, -0x48(%rbp)
140b66650: c6 85 b7 00 00 00 01        	movb	$0x1, 0xb7(%rbp)
140b66657: 48 8d 15 4a c6 f8 01        	leaq	0x1f8c64a(%rip), %rdx   # 0x142af2ca8
140b6665e: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
140b66662: e8 29 2d d7 01              	callq	0x1428d9390 <.text+0x28d8390>
140b66667: 0f 0b                       	ud2
140b66669: 0f 1f 80 00 00 00 00        	nopl	(%rax)
140b66670: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140b66675: 55                          	pushq	%rbp
140b66676: 56                          	pushq	%rsi
140b66677: 53                          	pushq	%rbx
140b66678: 48 83 ec 20                 	subq	$0x20, %rsp
140b6667c: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140b66683: 80 bd b6 00 00 00 00        	cmpb	$0x0, 0xb6(%rbp)
140b6668a: 74 10                       	je	0x140b6669c <.text+0xb6569c>
140b6668c: c6 85 b7 00 00 00 00        	movb	$0x0, 0xb7(%rbp)
140b66693: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
140b66697: e8 e4 31 4c ff              	callq	0x140029880 <.text+0x28880>
140b6669c: c6 85 b7 00 00 00 00        	movb	$0x0, 0xb7(%rbp)
140b666a3: 48 83 c4 20                 	addq	$0x20, %rsp
140b666a7: 5b                          	popq	%rbx
140b666a8: 5e                          	popq	%rsi
140b666a9: 5d                          	popq	%rbp
140b666aa: c3                          	retq
140b666ab: 0f 1f 44 00 00              	nopl	(%rax,%rax)
140b666b0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140b666b5: 55                          	pushq	%rbp
140b666b6: 56                          	pushq	%rsi
140b666b7: 53                          	pushq	%rbx
140b666b8: 48 83 ec 20                 	subq	$0x20, %rsp
140b666bc: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140b666c3: 80 bd b7 00 00 00 00        	cmpb	$0x0, 0xb7(%rbp)
140b666ca: 75 08                       	jne	0x140b666d4 <.text+0xb656d4>
140b666cc: 48 83 c4 20                 	addq	$0x20, %rsp
140b666d0: 5b                          	popq	%rbx
140b666d1: 5e                          	popq	%rsi
140b666d2: 5d                          	popq	%rbp
140b666d3: c3                          	retq
140b666d4: 48 8b 8d a8 00 00 00        	movq	0xa8(%rbp), %rcx
140b666db: e8 a0 31 4c ff              	callq	0x140029880 <.text+0x28880>
140b666e0: eb ea                       	jmp	0x140b666cc <.text+0xb656cc>
