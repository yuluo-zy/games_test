
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14089a380: 55                          	pushq	%rbp
14089a381: 41 57                       	pushq	%r15
14089a383: 41 56                       	pushq	%r14
14089a385: 41 54                       	pushq	%r12
14089a387: 56                          	pushq	%rsi
14089a388: 57                          	pushq	%rdi
14089a389: 53                          	pushq	%rbx
14089a38a: 48 83 ec 70                 	subq	$0x70, %rsp
14089a38e: 48 8d 6c 24 70              	leaq	0x70(%rsp), %rbp
14089a393: 0f 29 75 f0                 	movaps	%xmm6, -0x10(%rbp)
14089a397: 48 c7 45 e8 fe ff ff ff     	movq	$-0x2, -0x18(%rbp)
14089a39f: 48 89 ce                    	movq	%rcx, %rsi
14089a3a2: 48 8b 49 10                 	movq	0x10(%rcx), %rcx
14089a3a6: 48 83 f9 ff                 	cmpq	$-0x1, %rcx
14089a3aa: 0f 84 28 01 00 00           	je	0x14089a4d8 <.text+0x8994d8>
14089a3b0: 48 8b 01                    	movq	(%rcx), %rax
14089a3b3: 48 89 4d e0                 	movq	%rcx, -0x20(%rbp)
14089a3b7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
14089a3c0: 48 85 c0                    	testq	%rax, %rax
14089a3c3: 0f 84 f8 00 00 00           	je	0x14089a4c1 <.text+0x8994c1>
14089a3c9: 0f 88 1c 01 00 00           	js	0x14089a4eb <.text+0x8994eb>
14089a3cf: 48 8d 50 01                 	leaq	0x1(%rax), %rdx
14089a3d3: f0                          	lock
14089a3d4: 48 0f b1 11                 	cmpxchgq	%rdx, (%rcx)
14089a3d8: 75 e6                       	jne	0x14089a3c0 <.text+0x8993c0>
14089a3da: 48 89 4d d8                 	movq	%rcx, -0x28(%rbp)
14089a3de: 48 8d 79 10                 	leaq	0x10(%rcx), %rdi
14089a3e2: b2 01                       	movb	$0x1, %dl
14089a3e4: 31 c0                       	xorl	%eax, %eax
14089a3e6: f0                          	lock
14089a3e7: 0f b0 51 10                 	cmpxchgb	%dl, 0x10(%rcx)
14089a3eb: 0f 85 0f 01 00 00           	jne	0x14089a500 <.text+0x899500>
14089a3f1: 48 89 f9                    	movq	%rdi, %rcx
14089a3f4: e8 77 78 3e 00              	callq	0x140c81c70 <.text+0xc80c70>
14089a3f9: 48 89 7d d0                 	movq	%rdi, -0x30(%rbp)
14089a3fd: 48 8b 46 08                 	movq	0x8(%rsi), %rax
14089a401: 48 85 c0                    	testq	%rax, %rax
14089a404: 4c 8b 45 e0                 	movq	-0x20(%rbp), %r8
14089a408: 0f 84 86 00 00 00           	je	0x14089a494 <.text+0x899494>
14089a40e: 48 8b 1e                    	movq	(%rsi), %rbx
14089a411: 49 8d 70 18                 	leaq	0x18(%r8), %rsi
14089a415: 48 c1 e0 03                 	shlq	$0x3, %rax
14089a419: 4c 8d 34 40                 	leaq	(%rax,%rax,2), %r14
14089a41d: 45 31 ff                    	xorl	%r15d, %r15d
14089a420: 0f 57 f6                    	xorps	%xmm6, %xmm6
14089a423: 48 8d 3d 66 e6 1f 02        	leaq	0x21fe666(%rip), %rdi   # 0x142a98a90
14089a42a: eb 31                       	jmp	0x14089a45d <.text+0x89945d>
14089a42c: 0f 1f 40 00                 	nopl	(%rax)
14089a430: 4c 8b 45 e0                 	movq	-0x20(%rbp), %r8
14089a434: 49 8b 40 20                 	movq	0x20(%r8), %rax
14089a438: 4b 8d 0c 64                 	leaq	(%r12,%r12,2), %rcx
14089a43c: 48 8b 55 c0                 	movq	-0x40(%rbp), %rdx
14089a440: 48 89 54 c8 10              	movq	%rdx, 0x10(%rax,%rcx,8)
14089a445: 0f 28 45 b0                 	movaps	-0x50(%rbp), %xmm0
14089a449: 0f 11 04 c8                 	movups	%xmm0, (%rax,%rcx,8)
14089a44d: 49 ff c4                    	incq	%r12
14089a450: 4d 89 60 28                 	movq	%r12, 0x28(%r8)
14089a454: 49 83 c7 18                 	addq	$0x18, %r15
14089a458: 4d 39 fe                    	cmpq	%r15, %r14
14089a45b: 74 37                       	je	0x14089a494 <.text+0x899494>
14089a45d: 4a 8b 44 3b 10              	movq	0x10(%rbx,%r15), %rax
14089a462: 48 89 45 c0                 	movq	%rax, -0x40(%rbp)
14089a466: 42 0f 10 04 3b              	movups	(%rbx,%r15), %xmm0
14089a46b: 0f 29 45 b0                 	movaps	%xmm0, -0x50(%rbp)
14089a46f: 42 0f 11 34 3b              	movups	%xmm6, (%rbx,%r15)
14089a474: 4a c7 44 3b 10 00 00 00 00  	movq	$0x0, 0x10(%rbx,%r15)
14089a47d: 4d 8b 60 28                 	movq	0x28(%r8), %r12
14089a481: 4d 3b 60 18                 	cmpq	0x18(%r8), %r12
14089a485: 75 a9                       	jne	0x14089a430 <.text+0x899430>
14089a487: 48 89 f1                    	movq	%rsi, %rcx
14089a48a: 48 89 fa                    	movq	%rdi, %rdx
14089a48d: e8 1e 89 cc ff              	callq	0x140562db0 <.text+0x561db0>
14089a492: eb 9c                       	jmp	0x14089a430 <.text+0x899430>
14089a494: 48 8b 75 d0                 	movq	-0x30(%rbp), %rsi
14089a498: 48 89 f1                    	movq	%rsi, %rcx
14089a49b: e8 70 79 3e 00              	callq	0x140c81e10 <.text+0xc80e10>
14089a4a0: 31 c9                       	xorl	%ecx, %ecx
14089a4a2: b0 01                       	movb	$0x1, %al
14089a4a4: f0                          	lock
14089a4a5: 0f b0 0e                    	cmpxchgb	%cl, (%rsi)
14089a4a8: 75 69                       	jne	0x14089a513 <.text+0x899513>
14089a4aa: 48 8b 45 d8                 	movq	-0x28(%rbp), %rax
14089a4ae: f0                          	lock
14089a4af: 48 ff 08                    	decq	(%rax)
14089a4b2: 75 09                       	jne	0x14089a4bd <.text+0x8994bd>
14089a4b4: 48 8d 4d d8                 	leaq	-0x28(%rbp), %rcx
14089a4b8: e8 03 21 2c 00              	callq	0x140b5c5c0 <.text+0xb5b5c0>
14089a4bd: 48 8b 4d e0                 	movq	-0x20(%rbp), %rcx
14089a4c1: f0                          	lock
14089a4c2: 48 ff 49 08                 	decq	0x8(%rcx)
14089a4c6: 75 10                       	jne	0x14089a4d8 <.text+0x8994d8>
14089a4c8: ba 30 00 00 00              	movl	$0x30, %edx
14089a4cd: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14089a4d3: e8 48 97 d7 ff              	callq	0x140613c20 <.text+0x612c20>
14089a4d8: 0f 28 75 f0                 	movaps	-0x10(%rbp), %xmm6
14089a4dc: 48 83 c4 70                 	addq	$0x70, %rsp
14089a4e0: 5b                          	popq	%rbx
14089a4e1: 5f                          	popq	%rdi
14089a4e2: 5e                          	popq	%rsi
14089a4e3: 41 5c                       	popq	%r12
14089a4e5: 41 5e                       	popq	%r14
14089a4e7: 41 5f                       	popq	%r15
14089a4e9: 5d                          	popq	%rbp
14089a4ea: c3                          	retq
14089a4eb: 48 8d 0d 4e 6e 25 02        	leaq	0x2256e4e(%rip), %rcx   # 0x142af1340
14089a4f2: 48 8d 15 cf 6e 25 02        	leaq	0x2256ecf(%rip), %rdx   # 0x142af13c8
14089a4f9: e8 02 f5 00 02              	callq	0x1428a9a00 <.text+0x28a8a00>
14089a4fe: 0f 0b                       	ud2
14089a500: 48 89 f9                    	movq	%rdi, %rcx
14089a503: 41 b8 00 ca 9a 3b           	movl	$0x3b9aca00, %r8d       # imm = 0x3B9ACA00
14089a509: e8 02 2d 03 02              	callq	0x1428cd210 <.text+0x28cc210>
14089a50e: e9 de fe ff ff              	jmp	0x14089a3f1 <.text+0x8993f1>
14089a513: 48 89 f1                    	movq	%rsi, %rcx
14089a516: 31 d2                       	xorl	%edx, %edx
14089a518: e8 73 36 03 02              	callq	0x1428cdb90 <.text+0x28ccb90>
14089a51d: eb 8b                       	jmp	0x14089a4aa <.text+0x8994aa>
14089a51f: 90                          	nop
14089a520: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14089a525: 55                          	pushq	%rbp
14089a526: 41 57                       	pushq	%r15
14089a528: 41 56                       	pushq	%r14
14089a52a: 41 54                       	pushq	%r12
14089a52c: 56                          	pushq	%rsi
14089a52d: 57                          	pushq	%rdi
14089a52e: 53                          	pushq	%rbx
14089a52f: 48 83 ec 30                 	subq	$0x30, %rsp
14089a533: 48 8d 6a 70                 	leaq	0x70(%rdx), %rbp
14089a537: 0f 29 74 24 20              	movaps	%xmm6, 0x20(%rsp)
14089a53c: 48 8b 4d e0                 	movq	-0x20(%rbp), %rcx
14089a540: f0                          	lock
14089a541: 48 ff 49 08                 	decq	0x8(%rcx)
14089a545: 75 10                       	jne	0x14089a557 <.text+0x899557>
14089a547: ba 30 00 00 00              	movl	$0x30, %edx
14089a54c: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14089a552: e8 c9 96 d7 ff              	callq	0x140613c20 <.text+0x612c20>
14089a557: 0f 28 74 24 20              	movaps	0x20(%rsp), %xmm6
14089a55c: 48 83 c4 30                 	addq	$0x30, %rsp
14089a560: 5b                          	popq	%rbx
14089a561: 5f                          	popq	%rdi
14089a562: 5e                          	popq	%rsi
14089a563: 41 5c                       	popq	%r12
14089a565: 41 5e                       	popq	%r14
14089a567: 41 5f                       	popq	%r15
14089a569: 5d                          	popq	%rbp
14089a56a: c3                          	retq
14089a56b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
