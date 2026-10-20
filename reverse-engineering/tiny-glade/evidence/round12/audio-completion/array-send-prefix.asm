
D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14167b9b0: 55                          	pushq	%rbp
14167b9b1: 41 57                       	pushq	%r15
14167b9b3: 41 56                       	pushq	%r14
14167b9b5: 41 55                       	pushq	%r13
14167b9b7: 41 54                       	pushq	%r12
14167b9b9: 56                          	pushq	%rsi
14167b9ba: 57                          	pushq	%rdi
14167b9bb: 53                          	pushq	%rbx
14167b9bc: 48 81 ec 88 00 00 00        	subq	$0x88, %rsp
14167b9c3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
14167b9cb: 48 c7 45 00 fe ff ff ff     	movq	$-0x2, (%rbp)
14167b9d3: 48 89 cf                    	movq	%rcx, %rdi
14167b9d6: 48 89 55 c8                 	movq	%rdx, -0x38(%rbp)
14167b9da: 44 89 45 d0                 	movl	%r8d, -0x30(%rbp)
14167b9de: 0f 57 c0                    	xorps	%xmm0, %xmm0
14167b9e1: 0f 29 45 b0                 	movaps	%xmm0, -0x50(%rbp)
14167b9e5: 0f 29 45 a0                 	movaps	%xmm0, -0x60(%rbp)
14167b9e9: 48 c7 45 c0 00 00 00 00     	movq	$0x0, -0x40(%rbp)
14167b9f1: 48 8b 89 80 00 00 00        	movq	0x80(%rcx), %rcx
14167b9f8: 48 8b 97 90 01 00 00        	movq	0x190(%rdi), %rdx
14167b9ff: 40 b6 01                    	movb	$0x1, %sil
14167ba02: 48 85 ca                    	testq	%rcx, %rdx
14167ba05: 74 16                       	je	0x14167ba1d <.text+0x167aa1d>
14167ba07: 89 f0                       	movl	%esi, %eax
14167ba09: 48 81 c4 88 00 00 00        	addq	$0x88, %rsp
14167ba10: 5b                          	popq	%rbx
14167ba11: 5f                          	popq	%rdi
14167ba12: 5e                          	popq	%rsi
14167ba13: 41 5c                       	popq	%r12
14167ba15: 41 5d                       	popq	%r13
14167ba17: 41 5e                       	popq	%r14
14167ba19: 41 5f                       	popq	%r15
14167ba1b: 5d                          	popq	%rbp
14167ba1c: c3                          	retq
14167ba1d: 4c 8d 6d a0                 	leaq	-0x60(%rbp), %r13
14167ba21: 4c 8d 75 c8                 	leaq	-0x38(%rbp), %r14
14167ba25: 48 8d 5d d8                 	leaq	-0x28(%rbp), %rbx
14167ba29: 4c 8d 65 f0                 	leaq	-0x10(%rbp), %r12
14167ba2d: 45 31 ff                    	xorl	%r15d, %r15d
14167ba30: eb 24                       	jmp	0x14167ba56 <.text+0x167aa56>
14167ba32: e8 39 9f 6d ff              	callq	0x140d55970 <.text+0xd54970>
14167ba37: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
14167ba40: 41 ff c7                    	incl	%r15d
14167ba43: 48 8b 8f 80 00 00 00        	movq	0x80(%rdi), %rcx
14167ba4a: 48 8b 97 90 01 00 00        	movq	0x190(%rdi), %rdx
14167ba51: 48 85 ca                    	testq	%rcx, %rdx
14167ba54: 75 b1                       	jne	0x14167ba07 <.text+0x167aa07>
14167ba56: 48 ff ca                    	decq	%rdx
14167ba59: 45 31 c9                    	xorl	%r9d, %r9d
14167ba5c: 4c 2b 8f 88 01 00 00        	subq	0x188(%rdi), %r9
14167ba63: 48 21 ca                    	andq	%rcx, %rdx
14167ba66: 4c 8b 87 98 01 00 00        	movq	0x198(%rdi), %r8
14167ba6d: 49 8b 04 d0                 	movq	(%r8,%rdx,8), %rax
14167ba71: 48 39 c1                    	cmpq	%rax, %rcx
14167ba74: 75 3a                       	jne	0x14167bab0 <.text+0x167aab0>
14167ba76: 48 8d 42 01                 	leaq	0x1(%rdx), %rax
14167ba7a: 48 3b 87 80 01 00 00        	cmpq	0x180(%rdi), %rax
14167ba81: 0f 83 a9 00 00 00           	jae	0x14167bb30 <.text+0x167ab30>
14167ba87: 4c 8d 49 01                 	leaq	0x1(%rcx), %r9
14167ba8b: 48 89 c8                    	movq	%rcx, %rax
14167ba8e: f0                          	lock
14167ba8f: 4c 0f b1 8f 80 00 00 00     	cmpxchgq	%r9, 0x80(%rdi)
14167ba97: 0f 85 af 00 00 00           	jne	0x14167bb4c <.text+0x167ab4c>
14167ba9d: e9 09 03 00 00              	jmp	0x14167bdab <.text+0x167adab>
14167baa2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
14167bab0: 48 03 87 88 01 00 00        	addq	0x188(%rdi), %rax
14167bab7: 48 8d 51 01                 	leaq	0x1(%rcx), %rdx
14167babb: 48 39 d0                    	cmpq	%rdx, %rax
14167babe: 0f 85 0c 01 00 00           	jne	0x14167bbd0 <.text+0x167abd0>
14167bac4: 0f ae f0                    	mfence
14167bac7: 48 8b 07                    	movq	(%rdi), %rax
14167baca: 48 03 87 88 01 00 00        	addq	0x188(%rdi), %rax
14167bad1: 48 39 c8                    	cmpq	%rcx, %rax
14167bad4: 0f 84 51 01 00 00           	je	0x14167bc2b <.text+0x167ac2b>
14167bada: 41 83 ff 06                 	cmpl	$0x6, %r15d
14167bade: b8 06 00 00 00              	movl	$0x6, %eax
14167bae3: 41 0f 42 c7                 	cmovbl	%r15d, %eax
14167bae7: 45 85 ff                    	testl	%r15d, %r15d
14167baea: 0f 84 b0 00 00 00           	je	0x14167bba0 <.text+0x167aba0>
14167baf0: 0f af c0                    	imull	%eax, %eax
14167baf3: 83 f8 08                    	cmpl	$0x8, %eax
14167baf6: 72 1d                       	jb	0x14167bb15 <.text+0x167ab15>
14167baf8: 89 c1                       	movl	%eax, %ecx
14167bafa: 83 e1 38                    	andl	$0x38, %ecx
14167bafd: 0f 1f 00                    	nopl	(%rax)
14167bb00: f3 90                       	pause
14167bb02: f3 90                       	pause
14167bb04: f3 90                       	pause
14167bb06: f3 90                       	pause
14167bb08: f3 90                       	pause
14167bb0a: f3 90                       	pause
14167bb0c: f3 90                       	pause
14167bb0e: f3 90                       	pause
14167bb10: 83 c1 f8                    	addl	$-0x8, %ecx
14167bb13: 75 eb                       	jne	0x14167bb00 <.text+0x167ab00>
14167bb15: 83 e0 05                    	andl	$0x5, %eax
14167bb18: 0f 84 22 ff ff ff           	je	0x14167ba40 <.text+0x167aa40>
14167bb1e: 66 90                       	nop
14167bb20: f3 90                       	pause
14167bb22: ff c8                       	decl	%eax
14167bb24: 75 fa                       	jne	0x14167bb20 <.text+0x167ab20>
14167bb26: e9 15 ff ff ff              	jmp	0x14167ba40 <.text+0x167aa40>
14167bb2b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
14167bb30: 49 21 c9                    	andq	%rcx, %r9
14167bb33: 4c 03 8f 88 01 00 00        	addq	0x188(%rdi), %r9
14167bb3a: 48 89 c8                    	movq	%rcx, %rax
14167bb3d: f0                          	lock
14167bb3e: 4c 0f b1 8f 80 00 00 00     	cmpxchgq	%r9, 0x80(%rdi)
14167bb46: 0f 84 5f 02 00 00           	je	0x14167bdab <.text+0x167adab>
14167bb4c: 41 83 ff 06                 	cmpl	$0x6, %r15d
14167bb50: b8 06 00 00 00              	movl	$0x6, %eax
14167bb55: 41 0f 42 c7                 	cmovbl	%r15d, %eax
14167bb59: 45 85 ff                    	testl	%r15d, %r15d
14167bb5c: 74 42                       	je	0x14167bba0 <.text+0x167aba0>
14167bb5e: 0f af c0                    	imull	%eax, %eax
14167bb61: 83 f8 08                    	cmpl	$0x8, %eax
14167bb64: 72 1f                       	jb	0x14167bb85 <.text+0x167ab85>
14167bb66: 89 c1                       	movl	%eax, %ecx
14167bb68: 83 e1 38                    	andl	$0x38, %ecx
14167bb6b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
14167bb70: f3 90                       	pause
14167bb72: f3 90                       	pause
14167bb74: f3 90                       	pause
14167bb76: f3 90                       	pause
14167bb78: f3 90                       	pause
14167bb7a: f3 90                       	pause
14167bb7c: f3 90                       	pause
14167bb7e: f3 90                       	pause
14167bb80: 83 c1 f8                    	addl	$-0x8, %ecx
14167bb83: 75 eb                       	jne	0x14167bb70 <.text+0x167ab70>
14167bb85: 83 e0 05                    	andl	$0x5, %eax
14167bb88: 0f 84 b2 fe ff ff           	je	0x14167ba40 <.text+0x167aa40>
14167bb8e: 66 90                       	nop
14167bb90: f3 90                       	pause
14167bb92: ff c8                       	decl	%eax
14167bb94: 75 fa                       	jne	0x14167bb90 <.text+0x167ab90>
14167bb96: e9 a5 fe ff ff              	jmp	0x14167ba40 <.text+0x167aa40>
14167bb9b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
14167bba0: 41 bf 01 00 00 00           	movl	$0x1, %r15d
14167bba6: 48 8b 8f 80 00 00 00        	movq	0x80(%rdi), %rcx
14167bbad: 48 8b 97 90 01 00 00        	movq	0x190(%rdi), %rdx
14167bbb4: 48 85 ca                    	testq	%rcx, %rdx
14167bbb7: 0f 84 99 fe ff ff           	je	0x14167ba56 <.text+0x167aa56>
14167bbbd: e9 45 fe ff ff              	jmp	0x14167ba07 <.text+0x167aa07>
14167bbc2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
14167bbd0: 41 83 ff 07                 	cmpl	$0x7, %r15d
14167bbd4: 0f 83 58 fe ff ff           	jae	0x14167ba32 <.text+0x167aa32>
14167bbda: 45 85 ff                    	testl	%r15d, %r15d
14167bbdd: 0f 84 5d fe ff ff           	je	0x14167ba40 <.text+0x167aa40>
14167bbe3: 44 89 f8                    	movl	%r15d, %eax
14167bbe6: 0f af c0                    	imull	%eax, %eax
14167bbe9: 8d 48 ff                    	leal	-0x1(%rax), %ecx
14167bbec: 83 f9 07                    	cmpl	$0x7, %ecx
14167bbef: 72 24                       	jb	0x14167bc15 <.text+0x167ac15>
14167bbf1: 89 c1                       	movl	%eax, %ecx
14167bbf3: 83 e1 f8                    	andl	$-0x8, %ecx
14167bbf6: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
14167bc00: f3 90                       	pause
14167bc02: f3 90                       	pause
14167bc04: f3 90                       	pause
14167bc06: f3 90                       	pause
14167bc08: f3 90                       	pause
14167bc0a: f3 90                       	pause
14167bc0c: f3 90                       	pause
14167bc0e: f3 90                       	pause
14167bc10: 83 c1 f8                    	addl	$-0x8, %ecx
14167bc13: 75 eb                       	jne	0x14167bc00 <.text+0x167ac00>
14167bc15: 83 e0 05                    	andl	$0x5, %eax
14167bc18: 0f 84 22 fe ff ff           	je	0x14167ba40 <.text+0x167aa40>
14167bc1e: 66 90                       	nop
14167bc20: f3 90                       	pause
14167bc22: ff c8                       	decl	%eax
14167bc24: 75 fa                       	jne	0x14167bc20 <.text+0x167ac20>
14167bc26: e9 15 fe ff ff              	jmp	0x14167ba40 <.text+0x167aa40>
14167bc2b: 44 8b 7d d0                 	movl	-0x30(%rbp), %r15d
14167bc2f: 41 81 ff 00 ca 9a 3b        	cmpl	$0x3b9aca00, %r15d      # imm = 0x3B9ACA00
14167bc36: 74 41                       	je	0x14167bc79 <.text+0x167ac79>
14167bc38: 4c 89 f3                    	movq	%r14, %rbx
14167bc3b: 4d 89 ee                    	movq	%r13, %r14
14167bc3e: 4d 89 e5                    	movq	%r12, %r13
14167bc41: 4c 8b 65 c8                 	movq	-0x38(%rbp), %r12
14167bc45: e8 96 79 6e ff              	callq	0x140d635e0 <.text+0xd625e0>
14167bc4a: 4c 39 e0                    	cmpq	%r12, %rax
14167bc4d: 75 17                       	jne	0x14167bc66 <.text+0x167ac66>
14167bc4f: 44 39 fa                    	cmpl	%r15d, %edx
14167bc52: 4d 89 ec                    	movq	%r13, %r12
14167bc55: 4d 89 f5                    	movq	%r14, %r13
14167bc58: 49 89 de                    	movq	%rbx, %r14
14167bc5b: 48 8d 5d d8                 	leaq	-0x28(%rbp), %rbx
14167bc5f: 72 18                       	jb	0x14167bc79 <.text+0x167ac79>
14167bc61: e9 6f 01 00 00              	jmp	0x14167bdd5 <.text+0x167add5>
14167bc66: 4d 89 ec                    	movq	%r13, %r12
14167bc69: 4d 89 f5                    	movq	%r14, %r13
14167bc6c: 49 89 de                    	movq	%rbx, %r14
14167bc6f: 48 8d 5d d8                 	leaq	-0x28(%rbp), %rbx
14167bc73: 0f 87 5c 01 00 00           	ja	0x14167bdd5 <.text+0x167add5>
14167bc79: e8 02 1b 6d ff              	callq	0x140d4d780 <.text+0xd4c780>
14167bc7e: 49 89 c7                    	movq	%rax, %r15
14167bc81: 48 8b 00                    	movq	(%rax), %rax
14167bc84: 48 83 f8 01                 	cmpq	$0x1, %rax
14167bc88: 0f 85 8d 00 00 00           	jne	0x14167bd1b <.text+0x167ad1b>
14167bc8e: 49 83 c7 08                 	addq	$0x8, %r15
14167bc92: 4d 8b 27                    	movq	(%r15), %r12
14167bc95: 49 c7 07 00 00 00 00        	movq	$0x0, (%r15)
14167bc9c: 4d 85 e4                    	testq	%r12, %r12
14167bc9f: 0f 84 9b 00 00 00           	je	0x14167bd40 <.text+0x167ad40>
14167bca5: 4c 89 65 f0                 	movq	%r12, -0x10(%rbp)
14167bca9: 49 c7 44 24 18 00 00 00 00  	movq	$0x0, 0x18(%r12)
14167bcb2: 49 c7 44 24 20 00 00 00 00  	movq	$0x0, 0x20(%r12)
14167bcbb: 4c 89 6d d8                 	movq	%r13, -0x28(%rbp)
14167bcbf: 48 89 7d e0                 	movq	%rdi, -0x20(%rbp)
14167bcc3: 4c 89 75 e8                 	movq	%r14, -0x18(%rbp)
14167bcc7: c6 45 ff 01                 	movb	$0x1, -0x1(%rbp)
14167bccb: 48 89 d9                    	movq	%rbx, %rcx
14167bcce: 4c 89 e2                    	movq	%r12, %rdx
14167bcd1: e8 aa 07 00 00              	callq	0x14167c480 <.text+0x167b480>
14167bcd6: 49 8b 07                    	movq	(%r15), %rax
14167bcd9: 48 89 45 d8                 	movq	%rax, -0x28(%rbp)
14167bcdd: 4d 89 27                    	movq	%r12, (%r15)
14167bce0: 48 85 c0                    	testq	%rax, %rax
14167bce3: 74 16                       	je	0x14167bcfb <.text+0x167acfb>
14167bce5: 48 8b 45 d8                 	movq	-0x28(%rbp), %rax
14167bce9: f0                          	lock
14167bcea: 48 ff 08                    	decq	(%rax)
14167bced: 75 0c                       	jne	0x14167bcfb <.text+0x167acfb>
14167bcef: c6 45 ff 00                 	movb	$0x0, -0x1(%rbp)
14167bcf3: 48 89 d9                    	movq	%rbx, %rcx
14167bcf6: e8 25 7f 6d ff              	callq	0x140d53c20 <.text+0xd52c20>
14167bcfb: 4c 8d 65 f0                 	leaq	-0x10(%rbp), %r12
14167bcff: 48 8b 8f 80 00 00 00        	movq	0x80(%rdi), %rcx
14167bd06: 48 8b 97 90 01 00 00        	movq	0x190(%rdi), %rdx
14167bd0d: 48 85 ca                    	testq	%rcx, %rdx
14167bd10: 0f 84 17 fd ff ff           	je	0x14167ba2d <.text+0x167aa2d>
14167bd16: e9 ec fc ff ff              	jmp	0x14167ba07 <.text+0x167aa07>
14167bd1b: 48 85 c0                    	testq	%rax, %rax
14167bd1e: 75 50                       	jne	0x14167bd70 <.text+0x167ad70>
14167bd20: 4c 89 f9                    	movq	%r15, %rcx
14167bd23: 31 d2                       	xorl	%edx, %edx
14167bd25: e8 96 c8 27 01              	callq	0x1428f85c0 <.text+0x28f75c0>
14167bd2a: 49 89 c7                    	movq	%rax, %r15
14167bd2d: 4d 8b 27                    	movq	(%r15), %r12
14167bd30: 49 c7 07 00 00 00 00        	movq	$0x0, (%r15)
14167bd37: 4d 85 e4                    	testq	%r12, %r12
14167bd3a: 0f 85 65 ff ff ff           	jne	0x14167bca5 <.text+0x167aca5>
14167bd40: e8 9b c5 25 01              	callq	0x1428d82e0 <.text+0x28d72e0>
14167bd45: 48 89 45 f0                 	movq	%rax, -0x10(%rbp)
14167bd49: 4c 89 6d d8                 	movq	%r13, -0x28(%rbp)
14167bd4d: 48 89 7d e0                 	movq	%rdi, -0x20(%rbp)
14167bd51: 4c 89 75 e8                 	movq	%r14, -0x18(%rbp)
14167bd55: 48 89 d9                    	movq	%rbx, %rcx
14167bd58: 48 89 c2                    	movq	%rax, %rdx
14167bd5b: e8 20 07 00 00              	callq	0x14167c480 <.text+0x167b480>
