
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1414792b0: 55                          	pushq	%rbp
1414792b1: 41 57                       	pushq	%r15
1414792b3: 41 56                       	pushq	%r14
1414792b5: 41 55                       	pushq	%r13
1414792b7: 41 54                       	pushq	%r12
1414792b9: 56                          	pushq	%rsi
1414792ba: 57                          	pushq	%rdi
1414792bb: 53                          	pushq	%rbx
1414792bc: 48 81 ec 98 00 00 00        	subq	$0x98, %rsp
1414792c3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1414792cb: 48 c7 45 10 fe ff ff ff     	movq	$-0x2, 0x10(%rbp)
1414792d3: 48 89 4d 08                 	movq	%rcx, 0x8(%rbp)
1414792d7: 4c 8b 62 18                 	movq	0x18(%rdx), %r12
1414792db: 48 8b 72 20                 	movq	0x20(%rdx), %rsi
1414792df: 49 39 f4                    	cmpq	%rsi, %r12
1414792e2: 0f 83 56 01 00 00           	jae	0x14147943e <.text+0x147843e>
1414792e8: 48 89 d3                    	movq	%rdx, %rbx
1414792eb: 48 8b 02                    	movq	(%rdx), %rax
1414792ee: 48 8b 7a 08                 	movq	0x8(%rdx), %rdi
1414792f2: 4c 8b 72 10                 	movq	0x10(%rdx), %r14
1414792f6: 4c 8b 38                    	movq	(%rax), %r15
1414792f9: 4d 85 ff                    	testq	%r15, %r15
1414792fc: 74 3d                       	je	0x14147933b <.text+0x147833b>
1414792fe: 4c 89 4d 00                 	movq	%r9, (%rbp)
141479302: 49 83 c7 10                 	addq	$0x10, %r15
141479306: 49 8d 44 24 01              	leaq	0x1(%r12), %rax
14147930b: 48 89 43 18                 	movq	%rax, 0x18(%rbx)
14147930f: 48 8d 4d c8                 	leaq	-0x38(%rbp), %rcx
141479313: 4c 89 fa                    	movq	%r15, %rdx
141479316: 49 89 f8                    	movq	%rdi, %r8
141479319: 4d 89 f1                    	movq	%r14, %r9
14147931c: e8 1f b1 5d ff              	callq	0x140a54440 <.text+0xa53440>
141479321: 48 8b 4d c8                 	movq	-0x38(%rbp), %rcx
141479325: 4c 8b 6d d0                 	movq	-0x30(%rbp), %r13
141479329: 48 83 f9 02                 	cmpq	$0x2, %rcx
14147932d: 75 7d                       	jne	0x1414793ac <.text+0x14783ac>
14147932f: 4c 89 ea                    	movq	%r13, %rdx
141479332: 48 8b 45 00                 	movq	(%rbp), %rax
141479336: e9 de 00 00 00              	jmp	0x141479419 <.text+0x1478419>
14147933b: 49 ff c4                    	incq	%r12
14147933e: 4d 85 f6                    	testq	%r14, %r14
141479341: 0f 88 6d 01 00 00           	js	0x1414794b4 <.text+0x14784b4>
141479347: 0f 84 fe 00 00 00           	je	0x14147944b <.text+0x147844b>
14147934d: 4c 89 63 18                 	movq	%r12, 0x18(%rbx)
141479351: 0f b6 05 b1 b8 1b 02        	movzbl	0x21bb8b1(%rip), %eax   # 0x143634c09
141479358: bb 01 00 00 00              	movl	$0x1, %ebx
14147935d: ba 01 00 00 00              	movl	$0x1, %edx
141479362: 4c 89 f1                    	movq	%r14, %rcx
141479365: e8 a6 a8 19 ff              	callq	0x140613c10 <.text+0x612c10>
14147936a: 48 85 c0                    	testq	%rax, %rax
14147936d: 0f 84 47 01 00 00           	je	0x1414794ba <.text+0x14784ba>
141479373: 49 89 c7                    	movq	%rax, %r15
141479376: 48 89 c1                    	movq	%rax, %rcx
141479379: 48 89 fa                    	movq	%rdi, %rdx
14147937c: 4d 89 f0                    	movq	%r14, %r8
14147937f: e8 7c a8 4a 01              	callq	0x142923c00 <.text+0x2922c00>
141479384: 4c 89 75 d8                 	movq	%r14, -0x28(%rbp)
141479388: 4c 89 7d e0                 	movq	%r15, -0x20(%rbp)
14147938c: 4c 89 75 e8                 	movq	%r14, -0x18(%rbp)
141479390: c6 45 f0 02                 	movb	$0x2, -0x10(%rbp)
141479394: 48 8d 45 d8                 	leaq	-0x28(%rbp), %rax
141479398: 0f 10 00                    	movups	(%rax), %xmm0
14147939b: 0f 10 48 10                 	movups	0x10(%rax), %xmm1
14147939f: 0f 29 45 a0                 	movaps	%xmm0, -0x60(%rbp)
1414793a3: 0f 29 4d b0                 	movaps	%xmm1, -0x50(%rbp)
1414793a7: e9 d2 00 00 00              	jmp	0x14147947e <.text+0x147847e>
1414793ac: 48 89 7d f8                 	movq	%rdi, -0x8(%rbp)
1414793b0: 4c 89 f7                    	movq	%r14, %rdi
1414793b3: 49 83 c4 02                 	addq	$0x2, %r12
1414793b7: 41 be 01 00 00 00           	movl	$0x1, %r14d
1414793bd: 49 29 f6                    	subq	%rsi, %r14
1414793c0: 48 8d 45 d8                 	leaq	-0x28(%rbp), %rax
1414793c4: 0f 10 00                    	movups	(%rax), %xmm0
1414793c7: 0f 10 48 10                 	movups	0x10(%rax), %xmm1
1414793cb: 0f 29 4d b0                 	movaps	%xmm1, -0x50(%rbp)
1414793cf: 0f 29 45 a0                 	movaps	%xmm0, -0x60(%rbp)
1414793d3: 48 83 f9 03                 	cmpq	$0x3, %rcx
1414793d7: 0f 85 a5 00 00 00           	jne	0x141479482 <.text+0x1478482>
1414793dd: 4b 8d 04 26                 	leaq	(%r14,%r12), %rax
1414793e1: 48 83 f8 02                 	cmpq	$0x2, %rax
1414793e5: 74 57                       	je	0x14147943e <.text+0x147843e>
1414793e7: 4c 89 63 18                 	movq	%r12, 0x18(%rbx)
1414793eb: 48 8d 4d c8                 	leaq	-0x38(%rbp), %rcx
1414793ef: 4c 89 fa                    	movq	%r15, %rdx
1414793f2: 4c 8b 45 f8                 	movq	-0x8(%rbp), %r8
1414793f6: 49 89 f9                    	movq	%rdi, %r9
1414793f9: e8 42 b0 5d ff              	callq	0x140a54440 <.text+0xa53440>
1414793fe: 48 8b 45 00                 	movq	(%rbp), %rax
141479402: 48 8b 4d c8                 	movq	-0x38(%rbp), %rcx
141479406: 48 8b 55 d0                 	movq	-0x30(%rbp), %rdx
14147940a: 49 ff c4                    	incq	%r12
14147940d: 4c 89 ee                    	movq	%r13, %rsi
141479410: 49 89 d5                    	movq	%rdx, %r13
141479413: 48 83 f9 02                 	cmpq	$0x2, %rcx
141479417: 75 a7                       	jne	0x1414793c0 <.text+0x14783c0>
141479419: 48 89 55 f8                 	movq	%rdx, -0x8(%rbp)
14147941d: 48 83 38 00                 	cmpq	$0x0, (%rax)
141479421: 74 09                       	je	0x14147942c <.text+0x147842c>
141479423: 48 8b 4d 00                 	movq	(%rbp), %rcx
141479427: e8 04 38 8d ff              	callq	0x140d4cc30 <.text+0xd4bc30>
14147942c: 48 8b 45 f8                 	movq	-0x8(%rbp), %rax
141479430: 48 8b 4d 00                 	movq	(%rbp), %rcx
141479434: 48 89 01                    	movq	%rax, (%rcx)
141479437: b9 02 00 00 00              	movl	$0x2, %ecx
14147943c: eb 47                       	jmp	0x141479485 <.text+0x1478485>
14147943e: 48 8b 45 08                 	movq	0x8(%rbp), %rax
141479442: 48 c7 00 03 00 00 00        	movq	$0x3, (%rax)
141479449: eb 55                       	jmp	0x1414794a0 <.text+0x14784a0>
14147944b: 48 c7 45 d8 00 00 00 00     	movq	$0x0, -0x28(%rbp)
141479453: 48 c7 45 e0 01 00 00 00     	movq	$0x1, -0x20(%rbp)
14147945b: 48 c7 45 e8 00 00 00 00     	movq	$0x0, -0x18(%rbp)
141479463: c6 45 f0 02                 	movb	$0x2, -0x10(%rbp)
141479467: 48 8d 45 d8                 	leaq	-0x28(%rbp), %rax
14147946b: 0f 10 00                    	movups	(%rax), %xmm0
14147946e: 0f 10 48 10                 	movups	0x10(%rax), %xmm1
141479472: 0f 29 45 a0                 	movaps	%xmm0, -0x60(%rbp)
141479476: 0f 29 4d b0                 	movaps	%xmm1, -0x50(%rbp)
14147947a: 4c 89 63 18                 	movq	%r12, 0x18(%rbx)
14147947e: 31 c9                       	xorl	%ecx, %ecx
141479480: eb 03                       	jmp	0x141479485 <.text+0x1478485>
141479482: 4c 89 ee                    	movq	%r13, %rsi
141479485: 48 8b 45 08                 	movq	0x8(%rbp), %rax
141479489: 0f 28 45 a0                 	movaps	-0x60(%rbp), %xmm0
14147948d: 0f 28 4d b0                 	movaps	-0x50(%rbp), %xmm1
141479491: 0f 11 48 20                 	movups	%xmm1, 0x20(%rax)
141479495: 0f 11 40 10                 	movups	%xmm0, 0x10(%rax)
141479499: 48 89 08                    	movq	%rcx, (%rax)
14147949c: 48 89 70 08                 	movq	%rsi, 0x8(%rax)
1414794a0: 48 81 c4 98 00 00 00        	addq	$0x98, %rsp
1414794a7: 5b                          	popq	%rbx
1414794a8: 5f                          	popq	%rdi
1414794a9: 5e                          	popq	%rsi
1414794aa: 41 5c                       	popq	%r12
1414794ac: 41 5d                       	popq	%r13
1414794ae: 41 5e                       	popq	%r14
1414794b0: 41 5f                       	popq	%r15
1414794b2: 5d                          	popq	%rbp
1414794b3: c3                          	retq
1414794b4: 4c 89 63 18                 	movq	%r12, 0x18(%rbx)
1414794b8: 31 db                       	xorl	%ebx, %ebx
1414794ba: 4c 8d 05 ef 72 65 01        	leaq	0x16572ef(%rip), %r8    # 0x142ad07b0
1414794c1: 48 89 d9                    	movq	%rbx, %rcx
1414794c4: 4c 89 f2                    	movq	%r14, %rdx
1414794c7: e8 17 fb 45 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
1414794cc: cc                          	int3
1414794cd: 0f 1f 00                    	nopl	(%rax)
1414794d0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1414794d5: 55                          	pushq	%rbp
1414794d6: 41 57                       	pushq	%r15
1414794d8: 41 56                       	pushq	%r14
1414794da: 41 55                       	pushq	%r13
1414794dc: 41 54                       	pushq	%r12
1414794de: 56                          	pushq	%rsi
1414794df: 57                          	pushq	%rdi
1414794e0: 53                          	pushq	%rbx
1414794e1: 48 83 ec 28                 	subq	$0x28, %rsp
1414794e5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1414794ec: 48 8b 45 f8                 	movq	-0x8(%rbp), %rax
1414794f0: 48 8b 4d 00                 	movq	(%rbp), %rcx
1414794f4: 48 89 01                    	movq	%rax, (%rcx)
1414794f7: 48 83 c4 28                 	addq	$0x28, %rsp
1414794fb: 5b                          	popq	%rbx
1414794fc: 5f                          	popq	%rdi
1414794fd: 5e                          	popq	%rsi
1414794fe: 41 5c                       	popq	%r12
141479500: 41 5d                       	popq	%r13
141479502: 41 5e                       	popq	%r14
141479504: 41 5f                       	popq	%r15
141479506: 5d                          	popq	%rbp
141479507: c3                          	retq
