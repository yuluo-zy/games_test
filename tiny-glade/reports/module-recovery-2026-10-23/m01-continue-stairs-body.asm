
/Users/yuluo/Desktop/game/B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14079c3a0: 55                          	pushq	%rbp
14079c3a1: 41 57                       	pushq	%r15
14079c3a3: 41 56                       	pushq	%r14
14079c3a5: 41 55                       	pushq	%r13
14079c3a7: 41 54                       	pushq	%r12
14079c3a9: 56                          	pushq	%rsi
14079c3aa: 57                          	pushq	%rdi
14079c3ab: 53                          	pushq	%rbx
14079c3ac: 48 83 ec 58                 	subq	$0x58, %rsp
14079c3b0: 48 8d 6c 24 50              	leaq	0x50(%rsp), %rbp
14079c3b5: 48 c7 45 00 fe ff ff ff     	movq	$-0x2, (%rbp)
14079c3bd: 48 8b 01                    	movq	(%rcx), %rax
14079c3c0: 8b 08                       	movl	(%rax), %ecx
14079c3c2: 44 8d 51 fa                 	leal	-0x6(%rcx), %r10d
14079c3c6: 49 89 cb                    	movq	%rcx, %r11
14079c3c9: 49 83 c3 fb                 	addq	$-0x5, %r11
14079c3cd: 45 31 c9                    	xorl	%r9d, %r9d
14079c3d0: 41 83 fa 0f                 	cmpl	$0xf, %r10d
14079c3d4: 4d 0f 42 cb                 	cmovbq	%r11, %r9
14079c3d8: 4d 85 c9                    	testq	%r9, %r9
14079c3db: 74 15                       	je	0x14079c3f2 <.text+0x79b3f2>
14079c3dd: 49 83 f9 06                 	cmpq	$0x6, %r9
14079c3e1: 0f 85 64 01 00 00           	jne	0x14079c54b <.text+0x79b54b>
14079c3e7: 80 78 04 3f                 	cmpb	$0x3f, 0x4(%rax)
14079c3eb: 74 0e                       	je	0x14079c3fb <.text+0x79b3fb>
14079c3ed: e9 59 01 00 00              	jmp	0x14079c54b <.text+0x79b54b>
14079c3f2: 83 f9 04                    	cmpl	$0x4, %ecx
14079c3f5: 0f 85 50 01 00 00           	jne	0x14079c54b <.text+0x79b54b>
14079c3fb: 8b 42 1c                    	movl	0x1c(%rdx), %eax
14079c3fe: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
14079c402: 89 01                       	movl	%eax, (%rcx)
14079c404: 48 8b 32                    	movq	(%rdx), %rsi
14079c407: 49 8b 08                    	movq	(%r8), %rcx
14079c40a: e8 71 19 ee ff              	callq	0x14067dd80 <.text+0x67cd80>
14079c40f: 48 89 c7                    	movq	%rax, %rdi
14079c412: 48 8b 00                    	movq	(%rax), %rax
14079c415: 89 c1                       	movl	%eax, %ecx
14079c417: 83 e1 08                    	andl	$0x8, %ecx
14079c41a: c1 e9 03                    	shrl	$0x3, %ecx
14079c41d: 48 83 f8 f0                 	cmpq	$-0x10, %rax
14079c421: 0f 93 c2                    	setae	%dl
14079c424: 08 ca                       	orb	%cl, %dl
14079c426: 74 12                       	je	0x14079c43a <.text+0x79b43a>
14079c428: 48 89 f9                    	movq	%rdi, %rcx
14079c42b: 31 d2                       	xorl	%edx, %edx
14079c42d: 41 b9 00 ca 9a 3b           	movl	$0x3b9aca00, %r9d       # imm = 0x3B9ACA00
14079c433: e8 78 00 13 02              	callq	0x1428cc4b0 <.text+0x28cb4b0>
14079c438: eb 0b                       	jmp	0x14079c445 <.text+0x79b445>
14079c43a: 48 8d 48 10                 	leaq	0x10(%rax), %rcx
14079c43e: f0                          	lock
14079c43f: 48 0f b1 0f                 	cmpxchgq	%rcx, (%rdi)
14079c443: 75 e3                       	jne	0x14079c428 <.text+0x79b428>
14079c445: 48 89 f9                    	movq	%rdi, %rcx
14079c448: e8 23 58 4e 00              	callq	0x140c81c70 <.text+0xc80c70>
14079c44d: 48 89 f9                    	movq	%rdi, %rcx
14079c450: 48 83 c9 01                 	orq	$0x1, %rcx
14079c454: 48 89 4d f0                 	movq	%rcx, -0x10(%rbp)
14079c458: e8 13 58 4e 00              	callq	0x140c81c70 <.text+0xc80c70>
14079c45d: 48 89 7d f8                 	movq	%rdi, -0x8(%rbp)
14079c461: 48 8b 47 18                 	movq	0x18(%rdi), %rax
14079c465: 48 85 c0                    	testq	%rax, %rax
14079c468: 0f 84 b2 00 00 00           	je	0x14079c520 <.text+0x79b520>
14079c46e: 48 8b 4d f8                 	movq	-0x8(%rbp), %rcx
14079c472: 4c 8b 69 10                 	movq	0x10(%rcx), %r13
14079c476: 48 89 c1                    	movq	%rax, %rcx
14079c479: 48 c1 e1 07                 	shlq	$0x7, %rcx
14079c47d: 48 8d 3c c1                 	leaq	(%rcx,%rax,8), %rdi
14079c481: 31 db                       	xorl	%ebx, %ebx
14079c483: eb 1b                       	jmp	0x14079c4a0 <.text+0x79b4a0>
14079c485: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
14079c490: 48 81 c3 88 00 00 00        	addq	$0x88, %rbx
14079c497: 48 39 df                    	cmpq	%rbx, %rdi
14079c49a: 0f 84 80 00 00 00           	je	0x14079c520 <.text+0x79b520>
14079c4a0: 41 83 7c 1d 00 03           	cmpl	$0x3, (%r13,%rbx)
14079c4a6: 75 e8                       	jne	0x14079c490 <.text+0x79b490>
14079c4a8: 41 80 bc 1d 81 00 00 00 3f  	cmpb	$0x3f, 0x81(%r13,%rbx)
14079c4b1: 75 dd                       	jne	0x14079c490 <.text+0x79b490>
14079c4b3: 4d 8b 74 1d 08              	movq	0x8(%r13,%rbx), %r14
14079c4b8: 4d 8b 7c 1d 10              	movq	0x10(%r13,%rbx), %r15
14079c4bd: 45 8b 64 1d 40              	movl	0x40(%r13,%rbx), %r12d
14079c4c2: 48 89 f1                    	movq	%rsi, %rcx
14079c4c5: 44 89 e2                    	movl	%r12d, %edx
14079c4c8: e8 63 38 16 00              	callq	0x1408ffd30 <.text+0x8fed30>
14079c4cd: 48 8d 4d d8                 	leaq	-0x28(%rbp), %rcx
14079c4d1: 48 89 f2                    	movq	%rsi, %rdx
14079c4d4: e8 b7 23 16 00              	callq	0x1408fe890 <.text+0x8fd890>
14079c4d9: 83 7d d8 03                 	cmpl	$0x3, -0x28(%rbp)
14079c4dd: 74 13                       	je	0x14079c4f2 <.text+0x79b4f2>
14079c4df: 8b 55 e8                    	movl	-0x18(%rbp), %edx
14079c4e2: 44 39 e2                    	cmpl	%r12d, %edx
14079c4e5: 74 0b                       	je	0x14079c4f2 <.text+0x79b4f2>
14079c4e7: 48 89 f1                    	movq	%rsi, %rcx
14079c4ea: 45 89 e0                    	movl	%r12d, %r8d
14079c4ed: e8 7e 38 16 00              	callq	0x1408ffd70 <.text+0x8fed70>
14079c4f2: 41 80 bc 1d 80 00 00 00 00  	cmpb	$0x0, 0x80(%r13,%rbx)
14079c4fb: 75 93                       	jne	0x14079c490 <.text+0x79b490>
14079c4fd: 48 8d 4d d8                 	leaq	-0x28(%rbp), %rcx
14079c501: 4c 89 f2                    	movq	%r14, %rdx
14079c504: 4d 89 f8                    	movq	%r15, %r8
14079c507: 45 89 e1                    	movl	%r12d, %r9d
14079c50a: e8 31 6d 09 00              	callq	0x140833240 <.text+0x832240>
14079c50f: 48 89 f1                    	movq	%rsi, %rcx
14079c512: 48 8d 55 d8                 	leaq	-0x28(%rbp), %rdx
14079c516: e8 95 22 16 00              	callq	0x1408fe7b0 <.text+0x8fd7b0>
14079c51b: e9 70 ff ff ff              	jmp	0x14079c490 <.text+0x79b490>
14079c520: 48 8b 75 f8                 	movq	-0x8(%rbp), %rsi
14079c524: 48 89 f1                    	movq	%rsi, %rcx
14079c527: e8 e4 58 4e 00              	callq	0x140c81e10 <.text+0xc80e10>
14079c52c: 48 8b 4d f0                 	movq	-0x10(%rbp), %rcx
14079c530: e8 db 58 4e 00              	callq	0x140c81e10 <.text+0xc80e10>
14079c535: 48 c7 c0 f0 ff ff ff        	movq	$-0x10, %rax
14079c53c: f0                          	lock
14079c53d: 48 0f c1 06                 	xaddq	%rax, (%rsi)
14079c541: 48 83 e0 f2                 	andq	$-0xe, %rax
14079c545: 48 83 f8 12                 	cmpq	$0x12, %rax
14079c549: 74 11                       	je	0x14079c55c <.text+0x79b55c>
14079c54b: 48 83 c4 58                 	addq	$0x58, %rsp
14079c54f: 5b                          	popq	%rbx
14079c550: 5f                          	popq	%rdi
14079c551: 5e                          	popq	%rsi
14079c552: 41 5c                       	popq	%r12
14079c554: 41 5d                       	popq	%r13
14079c556: 41 5e                       	popq	%r14
14079c558: 41 5f                       	popq	%r15
14079c55a: 5d                          	popq	%rbp
14079c55b: c3                          	retq
14079c55c: 48 89 f1                    	movq	%rsi, %rcx
14079c55f: 48 83 c4 58                 	addq	$0x58, %rsp
14079c563: 5b                          	popq	%rbx
14079c564: 5f                          	popq	%rdi
14079c565: 5e                          	popq	%rsi
14079c566: 41 5c                       	popq	%r12
14079c568: 41 5d                       	popq	%r13
14079c56a: 41 5e                       	popq	%r14
14079c56c: 41 5f                       	popq	%r15
14079c56e: 5d                          	popq	%rbp
14079c56f: e9 dc 09 13 02              	jmp	0x1428ccf50 <.text+0x28cbf50>
