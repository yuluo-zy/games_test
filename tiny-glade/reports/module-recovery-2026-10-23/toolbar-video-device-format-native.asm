
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14048d5c0: 55                          	pushq	%rbp
14048d5c1: 41 57                       	pushq	%r15
14048d5c3: 41 56                       	pushq	%r14
14048d5c5: 41 55                       	pushq	%r13
14048d5c7: 41 54                       	pushq	%r12
14048d5c9: 56                          	pushq	%rsi
14048d5ca: 57                          	pushq	%rdi
14048d5cb: 53                          	pushq	%rbx
14048d5cc: 48 81 ec a8 00 00 00        	subq	$0xa8, %rsp
14048d5d3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
14048d5db: 48 c7 45 20 fe ff ff ff     	movq	$-0x2, 0x20(%rbp)
14048d5e3: 49 8b 00                    	movq	(%r8), %rax
14048d5e6: 48 89 45 18                 	movq	%rax, 0x18(%rbp)
14048d5ea: 4d 8b 48 08                 	movq	0x8(%r8), %r9
14048d5ee: 48 39 d1                    	cmpq	%rdx, %rcx
14048d5f1: 0f 84 23 01 00 00           	je	0x14048d71a <.text+0x48c71a>
14048d5f7: 48 89 ce                    	movq	%rcx, %rsi
14048d5fa: 4d 8b 68 10                 	movq	0x10(%r8), %r13
14048d5fe: 48 29 ca                    	subq	%rcx, %rdx
14048d601: 48 c1 ea 05                 	shrq	$0x5, %rdx
14048d605: 48 bf cd cc cc cc cc cc cc cc       	movabsq	$-0x3333333333333333, %rdi # imm = 0xCCCCCCCCCCCCCCCD
14048d60f: 48 0f af fa                 	imulq	%rdx, %rdi
14048d613: 31 db                       	xorl	%ebx, %ebx
14048d615: eb 4a                       	jmp	0x14048d661 <.text+0x48c661>
14048d617: 41 bc 01 00 00 00           	movl	$0x1, %r12d
14048d61d: 4c 89 e1                    	movq	%r12, %rcx
14048d620: 4c 89 fa                    	movq	%r15, %rdx
14048d623: 4d 89 f0                    	movq	%r14, %r8
14048d626: e8 d5 65 49 02              	callq	0x142923c00 <.text+0x2922c00>
14048d62b: 4c 89 75 f8                 	movq	%r14, -0x8(%rbp)
14048d62f: 4c 89 65 00                 	movq	%r12, (%rbp)
14048d633: 4c 89 75 08                 	movq	%r14, 0x8(%rbp)
14048d637: 4c 8b 4d 10                 	movq	0x10(%rbp), %r9
14048d63b: 4b 8d 04 49                 	leaq	(%r9,%r9,2), %rax
14048d63f: 48 8b 4d 08                 	movq	0x8(%rbp), %rcx
14048d643: 49 89 4c c5 10              	movq	%rcx, 0x10(%r13,%rax,8)
14048d648: 0f 10 45 f8                 	movups	-0x8(%rbp), %xmm0
14048d64c: 41 0f 11 44 c5 00           	movups	%xmm0, (%r13,%rax,8)
14048d652: 49 ff c1                    	incq	%r9
14048d655: 48 ff c3                    	incq	%rbx
14048d658: 48 39 fb                    	cmpq	%rdi, %rbx
14048d65b: 0f 84 b9 00 00 00           	je	0x14048d71a <.text+0x48c71a>
14048d661: 4c 89 4d 10                 	movq	%r9, 0x10(%rbp)
14048d665: 48 8d 04 9b                 	leaq	(%rbx,%rbx,4), %rax
14048d669: 48 c1 e0 05                 	shlq	$0x5, %rax
14048d66d: 4c 8b 7c 06 18              	movq	0x18(%rsi,%rax), %r15
14048d672: 4c 8b 74 06 20              	movq	0x20(%rsi,%rax), %r14
14048d677: 49 83 fe 19                 	cmpq	$0x19, %r14
14048d67b: 76 73                       	jbe	0x14048d6f0 <.text+0x48c6f0>
14048d67d: 41 80 7f 19 bf              	cmpb	$-0x41, 0x19(%r15)
14048d682: 0f 8e ad 00 00 00           	jle	0x14048d735 <.text+0x48c735>
14048d688: ba 19 00 00 00              	movl	$0x19, %edx
14048d68d: 4c 89 f9                    	movq	%r15, %rcx
14048d690: e8 9b c5 c6 ff              	callq	0x1400f9c30 <.text+0xf8c30>
14048d695: 48 89 45 d8                 	movq	%rax, -0x28(%rbp)
14048d699: 48 89 55 e0                 	movq	%rdx, -0x20(%rbp)
14048d69d: 48 8d 45 d8                 	leaq	-0x28(%rbp), %rax
14048d6a1: 48 89 45 e8                 	movq	%rax, -0x18(%rbp)
14048d6a5: 48 8d 05 e4 c7 c7 ff        	leaq	-0x38381c(%rip), %rax   # 0x140109e90 <.text+0x108e90>
14048d6ac: 48 89 45 f0                 	movq	%rax, -0x10(%rbp)
14048d6b0: 48 8d 05 e1 0c 55 02        	leaq	0x2550ce1(%rip), %rax   # 0x1429de398
14048d6b7: 48 89 45 a8                 	movq	%rax, -0x58(%rbp)
14048d6bb: 48 c7 45 b0 02 00 00 00     	movq	$0x2, -0x50(%rbp)
14048d6c3: 48 8d 45 e8                 	leaq	-0x18(%rbp), %rax
14048d6c7: 48 89 45 b8                 	movq	%rax, -0x48(%rbp)
14048d6cb: 48 c7 45 c0 01 00 00 00     	movq	$0x1, -0x40(%rbp)
14048d6d3: 48 c7 45 c8 00 00 00 00     	movq	$0x0, -0x38(%rbp)
14048d6db: 48 8d 4d f8                 	leaq	-0x8(%rbp), %rcx
14048d6df: 48 8d 55 a8                 	leaq	-0x58(%rbp), %rdx
14048d6e3: e8 a8 c8 8e 00              	callq	0x140d79f90 <.text+0xd78f90>
14048d6e8: e9 4a ff ff ff              	jmp	0x14048d637 <.text+0x48c637>
14048d6ed: 0f 1f 00                    	nopl	(%rax)
14048d6f0: 4d 85 f6                    	testq	%r14, %r14
14048d6f3: 0f 84 1e ff ff ff           	je	0x14048d617 <.text+0x48c617>
14048d6f9: 0f b6 05 09 75 1a 03        	movzbl	0x31a7509(%rip), %eax   # 0x143634c09
14048d700: ba 01 00 00 00              	movl	$0x1, %edx
14048d705: 4c 89 f1                    	movq	%r14, %rcx
14048d708: e8 03 65 18 00              	callq	0x140613c10 <.text+0x612c10>
14048d70d: 48 85 c0                    	testq	%rax, %rax
14048d710: 74 45                       	je	0x14048d757 <.text+0x48c757>
14048d712: 49 89 c4                    	movq	%rax, %r12
14048d715: e9 03 ff ff ff              	jmp	0x14048d61d <.text+0x48c61d>
14048d71a: 48 8b 45 18                 	movq	0x18(%rbp), %rax
14048d71e: 4c 89 08                    	movq	%r9, (%rax)
14048d721: 48 81 c4 a8 00 00 00        	addq	$0xa8, %rsp
14048d728: 5b                          	popq	%rbx
14048d729: 5f                          	popq	%rdi
14048d72a: 5e                          	popq	%rsi
14048d72b: 41 5c                       	popq	%r12
14048d72d: 41 5d                       	popq	%r13
14048d72f: 41 5e                       	popq	%r14
14048d731: 41 5f                       	popq	%r15
14048d733: 5d                          	popq	%rbp
14048d734: c3                          	retq
14048d735: 48 8d 05 3c 0c 55 02        	leaq	0x2550c3c(%rip), %rax   # 0x1429de378
14048d73c: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14048d741: 41 b9 19 00 00 00           	movl	$0x19, %r9d
14048d747: 4c 89 f9                    	movq	%r15, %rcx
14048d74a: 4c 89 f2                    	movq	%r14, %rdx
14048d74d: 45 31 c0                    	xorl	%r8d, %r8d
14048d750: e8 eb c1 44 02              	callq	0x1428d9940 <.text+0x28d8940>
14048d755: eb 14                       	jmp	0x14048d76b <.text+0x48c76b>
14048d757: 4c 8d 05 b2 09 55 02        	leaq	0x25509b2(%rip), %r8    # 0x1429de110
14048d75e: b9 01 00 00 00              	movl	$0x1, %ecx
14048d763: 4c 89 f2                    	movq	%r14, %rdx
