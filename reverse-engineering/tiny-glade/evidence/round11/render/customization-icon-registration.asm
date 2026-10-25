
reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1422b58f0: 55                          	pushq	%rbp
1422b58f1: 41 57                       	pushq	%r15
1422b58f3: 41 56                       	pushq	%r14
1422b58f5: 41 54                       	pushq	%r12
1422b58f7: 56                          	pushq	%rsi
1422b58f8: 57                          	pushq	%rdi
1422b58f9: 53                          	pushq	%rbx
1422b58fa: 48 81 ec e0 02 00 00        	subq	$0x2e0, %rsp            # imm = 0x2E0
1422b5901: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1422b5909: 48 c7 85 58 02 00 00 fe ff ff ff    	movq	$-0x2, 0x258(%rbp)
1422b5914: 48 89 ce                    	movq	%rcx, %rsi
1422b5917: 48 c7 85 30 02 00 00 00 00 00 00    	movq	$0x0, 0x230(%rbp)
1422b5922: 48 c7 85 38 02 00 00 08 00 00 00    	movq	$0x8, 0x238(%rbp)
1422b592d: 48 c7 85 40 02 00 00 00 00 00 00    	movq	$0x0, 0x240(%rbp)
1422b5938: 48 8b 39                    	movq	(%rcx), %rdi
1422b593b: 48 83 7f 30 00              	cmpq	$0x0, 0x30(%rdi)
1422b5940: 49 bf 58 4b 91 81 c8 2b 5d 48       	movabsq	$0x485d2bc881914b58, %r15 # imm = 0x485D2BC881914B58
1422b594a: 49 bc df cc be f2 e6 ef c1 48       	movabsq	$0x48c1efe6f2beccdf, %r12 # imm = 0x48C1EFE6F2BECCDF
1422b5954: 0f 84 90 00 00 00           	je	0x1422b59ea <.text+0x22b49ea>
1422b595a: 48 8b 47 18                 	movq	0x18(%rdi), %rax
1422b595e: 48 8b 4f 20                 	movq	0x20(%rdi), %rcx
1422b5962: 48 89 ca                    	movq	%rcx, %rdx
1422b5965: 4c 21 fa                    	andq	%r15, %rdx
1422b5968: 45 31 c0                    	xorl	%r8d, %r8d
1422b596b: 66 0f 6f 05 6d bd 6a 00     	movdqa	0x6abd6d(%rip), %xmm0   # 0x1429616e0
1422b5973: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
1422b5977: f3 0f 6f 14 10              	movdqu	(%rax,%rdx), %xmm2
1422b597c: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
1422b5980: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
1422b5984: 66 44 0f d7 cb              	pmovmskb	%xmm3, %r9d
1422b5989: 45 85 c9                    	testl	%r9d, %r9d
1422b598c: 75 2f                       	jne	0x1422b59bd <.text+0x22b49bd>
1422b598e: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
1422b5992: 66 44 0f d7 ca              	pmovmskb	%xmm2, %r9d
1422b5997: 45 85 c9                    	testl	%r9d, %r9d
1422b599a: 75 4e                       	jne	0x1422b59ea <.text+0x22b49ea>
1422b599c: 4c 01 c2                    	addq	%r8, %rdx
1422b599f: 48 83 c2 10                 	addq	$0x10, %rdx
1422b59a3: 49 83 c0 10                 	addq	$0x10, %r8
1422b59a7: 48 21 ca                    	andq	%rcx, %rdx
1422b59aa: eb cb                       	jmp	0x1422b5977 <.text+0x22b4977>
1422b59ac: 0f 1f 40 00                 	nopl	(%rax)
1422b59b0: 45 8d 51 ff                 	leal	-0x1(%r9), %r10d
1422b59b4: 66 45 21 ca                 	andw	%r9w, %r10w
1422b59b8: 45 89 d1                    	movl	%r10d, %r9d
1422b59bb: 74 d1                       	je	0x1422b598e <.text+0x22b498e>
1422b59bd: f3 45 0f bc d1              	tzcntl	%r9d, %r10d
1422b59c2: 49 01 d2                    	addq	%rdx, %r10
1422b59c5: 49 21 ca                    	andq	%rcx, %r10
1422b59c8: 49 f7 da                    	negq	%r10
1422b59cb: 4f 8d 14 52                 	leaq	(%r10,%r10,2), %r10
1422b59cf: 4e 39 64 d0 e8              	cmpq	%r12, -0x18(%rax,%r10,8)
1422b59d4: 75 da                       	jne	0x1422b59b0 <.text+0x22b49b0>
1422b59d6: 4e 39 7c d0 f0              	cmpq	%r15, -0x10(%rax,%r10,8)
1422b59db: 75 d3                       	jne	0x1422b59b0 <.text+0x22b49b0>
1422b59dd: 4a 8d 04 d0                 	leaq	(%rax,%r10,8), %rax
1422b59e1: 48 8b 58 f8                 	movq	-0x8(%rax), %rbx
1422b59e5: e9 b8 03 00 00              	jmp	0x1422b5da2 <.text+0x22b4da2>
1422b59ea: 0f b6 47 60                 	movzbl	0x60(%rdi), %eax
1422b59ee: 48 8b 87 80 00 00 00        	movq	0x80(%rdi), %rax
1422b59f5: 48 8b 97 88 00 00 00        	movq	0x88(%rdi), %rdx
1422b59fc: 49 89 d0                    	movq	%rdx, %r8
1422b59ff: 4d 21 f8                    	andq	%r15, %r8
1422b5a02: 45 31 c9                    	xorl	%r9d, %r9d
1422b5a05: 66 0f 6f 05 d3 bc 6a 00     	movdqa	0x6abcd3(%rip), %xmm0   # 0x1429616e0
1422b5a0d: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
1422b5a11: f3 42 0f 6f 14 00           	movdqu	(%rax,%r8), %xmm2
1422b5a17: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
1422b5a1b: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
1422b5a1f: 66 44 0f d7 d3              	pmovmskb	%xmm3, %r10d
1422b5a24: 45 85 d2                    	testl	%r10d, %r10d
1422b5a27: 75 34                       	jne	0x1422b5a5d <.text+0x22b4a5d>
1422b5a29: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
1422b5a2d: 66 0f d7 ca                 	pmovmskb	%xmm2, %ecx
1422b5a31: 85 c9                       	testl	%ecx, %ecx
1422b5a33: 0f 85 ef 00 00 00           	jne	0x1422b5b28 <.text+0x22b4b28>
1422b5a39: 4d 01 c8                    	addq	%r9, %r8
1422b5a3c: 49 83 c0 10                 	addq	$0x10, %r8
1422b5a40: 49 83 c1 10                 	addq	$0x10, %r9
1422b5a44: 49 21 d0                    	andq	%rdx, %r8
1422b5a47: eb c8                       	jmp	0x1422b5a11 <.text+0x22b4a11>
1422b5a49: 0f 1f 80 00 00 00 00        	nopl	(%rax)
1422b5a50: 41 8d 4a ff                 	leal	-0x1(%r10), %ecx
1422b5a54: 66 44 21 d1                 	andw	%r10w, %cx
1422b5a58: 41 89 ca                    	movl	%ecx, %r10d
1422b5a5b: 74 cc                       	je	0x1422b5a29 <.text+0x22b4a29>
1422b5a5d: f3 45 0f bc da              	tzcntl	%r10d, %r11d
1422b5a62: 4d 01 c3                    	addq	%r8, %r11
1422b5a65: 49 21 d3                    	andq	%rdx, %r11
1422b5a68: 4c 89 d9                    	movq	%r11, %rcx
1422b5a6b: 48 f7 d9                    	negq	%rcx
1422b5a6e: 4a 8d 1c dd 00 00 00 00     	leaq	(,%r11,8), %rbx
1422b5a76: 48 c1 e1 07                 	shlq	$0x7, %rcx
1422b5a7a: 48 29 d9                    	subq	%rbx, %rcx
1422b5a7d: 4c 39 a4 08 78 ff ff ff     	cmpq	%r12, -0x88(%rax,%rcx)
1422b5a85: 75 c9                       	jne	0x1422b5a50 <.text+0x22b4a50>
1422b5a87: 4c 39 7c 08 80              	cmpq	%r15, -0x80(%rax,%rcx)
1422b5a8c: 75 c2                       	jne	0x1422b5a50 <.text+0x22b4a50>
1422b5a8e: 4d 89 d8                    	movq	%r11, %r8
1422b5a91: 49 c1 e0 07                 	shlq	$0x7, %r8
1422b5a95: 4f 8d 0c d8                 	leaq	(%r8,%r11,8), %r9
1422b5a99: 49 c1 f9 03                 	sarq	$0x3, %r9
1422b5a9d: 49 b8 f1 f0 f0 f0 f0 f0 f0 f0       	movabsq	$-0xf0f0f0f0f0f0f0f, %r8 # imm = 0xF0F0F0F0F0F0F0F1
1422b5aa7: 4d 0f af c1                 	imulq	%r9, %r8
1422b5aab: 4d 8d 48 f0                 	leaq	-0x10(%r8), %r9
1422b5aaf: 49 21 d1                    	andq	%rdx, %r9
1422b5ab2: f3 42 0f 6f 04 08           	movdqu	(%rax,%r9), %xmm0
1422b5ab8: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
1422b5abc: 66 0f 74 c1                 	pcmpeqb	%xmm1, %xmm0
1422b5ac0: 66 44 0f d7 d0              	pmovmskb	%xmm0, %r10d
1422b5ac5: f3 42 0f 6f 04 00           	movdqu	(%rax,%r8), %xmm0
1422b5acb: 66 0f 74 c1                 	pcmpeqb	%xmm1, %xmm0
1422b5acf: 66 0f d7 d0                 	pmovmskb	%xmm0, %edx
1422b5ad3: 45 85 d2                    	testl	%r10d, %r10d
1422b5ad6: 74 0b                       	je	0x1422b5ae3 <.text+0x22b4ae3>
1422b5ad8: 66 45 0f bd d2              	bsrw	%r10w, %r10w
1422b5add: 41 83 f2 0f                 	xorl	$0xf, %r10d
1422b5ae1: eb 05                       	jmp	0x1422b5ae8 <.text+0x22b4ae8>
1422b5ae3: 66 41 ba 10 00              	movw	$0x10, %r10w
1422b5ae8: 48 01 c1                    	addq	%rax, %rcx
1422b5aeb: 81 ca 00 00 01 00           	orl	$0x10000, %edx          # imm = 0x10000
1422b5af1: f3 0f bc d2                 	tzcntl	%edx, %edx
1422b5af5: 41 01 d2                    	addl	%edx, %r10d
1422b5af8: b2 80                       	movb	$-0x80, %dl
1422b5afa: 66 41 83 fa 0f              	cmpw	$0xf, %r10w
1422b5aff: 77 09                       	ja	0x1422b5b0a <.text+0x22b4b0a>
1422b5b01: 48 ff 87 90 00 00 00        	incq	0x90(%rdi)
1422b5b08: b2 ff                       	movb	$-0x1, %dl
1422b5b0a: 42 88 14 00                 	movb	%dl, (%rax,%r8)
1422b5b0e: 42 88 54 08 10              	movb	%dl, 0x10(%rax,%r9)
1422b5b13: 48 ff 8f 98 00 00 00        	decq	0x98(%rdi)
1422b5b1a: 48 8b 41 88                 	movq	-0x78(%rcx), %rax
1422b5b1e: 48 83 f8 02                 	cmpq	$0x2, %rax
1422b5b22: 0f 85 0c 02 00 00           	jne	0x1422b5d34 <.text+0x22b4d34>
1422b5b28: 48 8b 46 08                 	movq	0x8(%rsi), %rax
1422b5b2c: 48 8b 18                    	movq	(%rax), %rbx
1422b5b2f: 4c 8d 73 01                 	leaq	0x1(%rbx), %r14
1422b5b33: 4c 89 30                    	movq	%r14, (%rax)
1422b5b36: e8 35 c8 d4 fd              	callq	0x140002370 <.text+0x1370>
1422b5b3b: 48 b9 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rcx # imm = 0x8000000000000000
1422b5b45: 48 89 8d 08 02 00 00        	movq	%rcx, 0x208(%rbp)
1422b5b4c: 48 8d 0d 9f 23 c6 00        	leaq	0xc6239f(%rip), %rcx    # 0x142f17ef2
1422b5b53: 48 89 8d 10 02 00 00        	movq	%rcx, 0x210(%rbp)
1422b5b5a: 48 c7 85 18 02 00 00 36 00 00 00    	movq	$0x36, 0x218(%rbp)
1422b5b65: 66 c7 85 28 02 00 00 00 01  	movw	$0x100, 0x228(%rbp)     # imm = 0x100
1422b5b6e: 48 c7 85 d0 01 00 00 01 00 00 00    	movq	$0x1, 0x1d0(%rbp)
1422b5b79: 4c 89 a5 d8 01 00 00        	movq	%r12, 0x1d8(%rbp)
1422b5b80: 4c 89 bd e0 01 00 00        	movq	%r15, 0x1e0(%rbp)
1422b5b87: 48 c7 85 f8 01 00 00 04 00 00 00    	movq	$0x4, 0x1f8(%rbp)
1422b5b92: 48 c7 85 00 02 00 00 24 00 00 00    	movq	$0x24, 0x200(%rbp)
1422b5b9d: 48 c7 85 20 02 00 00 00 00 00 00    	movq	$0x0, 0x220(%rbp)
1422b5ba8: c6 85 2a 02 00 00 01        	movb	$0x1, 0x22a(%rbp)
1422b5baf: 48 89 85 e8 01 00 00        	movq	%rax, 0x1e8(%rbp)
1422b5bb6: 48 89 95 f0 01 00 00        	movq	%rdx, 0x1f0(%rbp)
1422b5bbd: 48 8d 8d 80 00 00 00        	leaq	0x80(%rbp), %rcx
1422b5bc4: 4c 8d 85 d0 01 00 00        	leaq	0x1d0(%rbp), %r8
1422b5bcb: 48 89 da                    	movq	%rbx, %rdx
1422b5bce: e8 9d 9c a8 fe              	callq	0x140d3f870 <.text+0xd3e870>
1422b5bd3: 4c 39 77 10                 	cmpq	%r14, 0x10(%rdi)
1422b5bd7: 73 19                       	jae	0x1422b5bf2 <.text+0x22b4bf2>
1422b5bd9: c6 85 57 02 00 00 01        	movb	$0x1, 0x257(%rbp)
1422b5be0: 4c 8d 05 61 78 d8 00        	leaq	0xd87861(%rip), %r8     # 0x14303d448
1422b5be7: 48 89 f9                    	movq	%rdi, %rcx
1422b5bea: 4c 89 f2                    	movq	%r14, %rdx
1422b5bed: e8 5e 46 9a ff              	callq	0x141c5a250 <.text+0x1c59250>
1422b5bf2: 4c 8b 7f 08                 	movq	0x8(%rdi), %r15
1422b5bf6: 4c 69 f3 d0 00 00 00        	imulq	$0xd0, %rbx, %r14
1422b5bfd: 4b 8d 04 37                 	leaq	(%r15,%r14), %rax
1422b5c01: 48 89 85 48 02 00 00        	movq	%rax, 0x248(%rbp)
1422b5c08: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
1422b5c0c: 48 8d 95 80 00 00 00        	leaq	0x80(%rbp), %rdx
1422b5c13: 41 b8 d0 00 00 00           	movl	$0xd0, %r8d
1422b5c19: e8 e2 df 66 00              	callq	0x142923c00 <.text+0x2922c00>
1422b5c1e: 43 83 3c 37 02              	cmpl	$0x2, (%r15,%r14)
1422b5c23: 74 0c                       	je	0x1422b5c31 <.text+0x22b4c31>
1422b5c25: 48 8b 8d 48 02 00 00        	movq	0x248(%rbp), %rcx
1422b5c2c: e8 1f af 33 fe              	callq	0x1405f0b50 <.text+0x5efb50>
1422b5c31: 48 8d 55 b0                 	leaq	-0x50(%rbp), %rdx
1422b5c35: 41 b8 d0 00 00 00           	movl	$0xd0, %r8d
1422b5c3b: 48 8b 8d 48 02 00 00        	movq	0x248(%rbp), %rcx
1422b5c42: e8 b9 df 66 00              	callq	0x142923c00 <.text+0x2922c00>
1422b5c47: 48 83 c7 18                 	addq	$0x18, %rdi
1422b5c4b: 48 ba df cc be f2 e6 ef c1 48       	movabsq	$0x48c1efe6f2beccdf, %rdx # imm = 0x48C1EFE6F2BECCDF
1422b5c55: 49 b8 58 4b 91 81 c8 2b 5d 48       	movabsq	$0x485d2bc881914b58, %r8 # imm = 0x485D2BC881914B58
1422b5c5f: 48 89 f9                    	movq	%rdi, %rcx
1422b5c62: 49 89 d9                    	movq	%rbx, %r9
1422b5c65: e8 b6 cf dc fd              	callq	0x140082c20 <.text+0x81c20>
1422b5c6a: 0f 10 05 7f 72 d8 00        	movups	0xd8727f(%rip), %xmm0   # 0x14303cef0
1422b5c71: 0f 29 85 90 00 00 00        	movaps	%xmm0, 0x90(%rbp)
1422b5c78: f3 0f 6f 05 60 72 d8 00     	movdqu	0xd87260(%rip), %xmm0   # 0x14303cee0
1422b5c80: 66 0f 7f 85 80 00 00 00     	movdqa	%xmm0, 0x80(%rbp)
1422b5c88: c6 85 56 02 00 00 01        	movb	$0x1, 0x256(%rbp)
1422b5c8f: 48 8d 85 30 02 00 00        	leaq	0x230(%rbp), %rax
1422b5c96: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1422b5c9b: 4c 8d 85 80 00 00 00        	leaq	0x80(%rbp), %r8
1422b5ca2: 48 89 d9                    	movq	%rbx, %rcx
1422b5ca5: 48 89 f2                    	movq	%rsi, %rdx
1422b5ca8: 45 31 c9                    	xorl	%r9d, %r9d
1422b5cab: e8 70 0e 85 fe              	callq	0x140b06b20 <.text+0xb05b20>
1422b5cb0: 48 8b 06                    	movq	(%rsi), %rax
1422b5cb3: 48 8b 40 08                 	movq	0x8(%rax), %rax
1422b5cb7: 0f 28 85 80 00 00 00        	movaps	0x80(%rbp), %xmm0
1422b5cbe: 0f 28 8d 90 00 00 00        	movaps	0x90(%rbp), %xmm1
1422b5cc5: 0f 29 4d c0                 	movaps	%xmm1, -0x40(%rbp)
1422b5cc9: 0f 29 45 b0                 	movaps	%xmm0, -0x50(%rbp)
1422b5ccd: 4a 8d 4c 30 60              	leaq	0x60(%rax,%r14), %rcx
1422b5cd2: 4a 8d 94 30 80 00 00 00     	leaq	0x80(%rax,%r14), %rdx
1422b5cda: 41 b8 20 00 00 00           	movl	$0x20, %r8d
1422b5ce0: 41 b9 10 00 00 00           	movl	$0x10, %r9d
1422b5ce6: 48 89 8d 48 02 00 00        	movq	%rcx, 0x248(%rbp)
1422b5ced: e8 4e b6 d9 fd              	callq	0x140051340 <.text+0x50340>
1422b5cf2: 66 0f 6f 45 b0              	movdqa	-0x50(%rbp), %xmm0
1422b5cf7: 66 0f 6f 4d c0              	movdqa	-0x40(%rbp), %xmm1
1422b5cfc: 48 8b 85 48 02 00 00        	movq	0x248(%rbp), %rax
1422b5d03: f3 0f 7f 48 10              	movdqu	%xmm1, 0x10(%rax)
1422b5d08: f3 0f 7f 00                 	movdqu	%xmm0, (%rax)
1422b5d0c: 48 8b 95 30 02 00 00        	movq	0x230(%rbp), %rdx
1422b5d13: 48 85 d2                    	testq	%rdx, %rdx
1422b5d16: 0f 84 86 00 00 00           	je	0x1422b5da2 <.text+0x22b4da2>
1422b5d1c: 48 8b 8d 38 02 00 00        	movq	0x238(%rbp), %rcx
1422b5d23: 48 c1 e2 03                 	shlq	$0x3, %rdx
1422b5d27: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1422b5d2d: e8 ee de 35 fe              	callq	0x140613c20 <.text+0x612c20>
1422b5d32: eb 6e                       	jmp	0x1422b5da2 <.text+0x22b4da2>
1422b5d34: 0f 10 41 f0                 	movups	-0x10(%rcx), %xmm0
1422b5d38: 0f 11 85 c0 01 00 00        	movups	%xmm0, 0x1c0(%rbp)
1422b5d3f: 0f 10 41 e0                 	movups	-0x20(%rcx), %xmm0
1422b5d43: 0f 11 85 b0 01 00 00        	movups	%xmm0, 0x1b0(%rbp)
1422b5d4a: 0f 10 41 d0                 	movups	-0x30(%rcx), %xmm0
1422b5d4e: 0f 11 85 a0 01 00 00        	movups	%xmm0, 0x1a0(%rbp)
1422b5d55: f3 0f 6f 41 90              	movdqu	-0x70(%rcx), %xmm0
1422b5d5a: f3 0f 6f 49 a0              	movdqu	-0x60(%rcx), %xmm1
1422b5d5f: f3 0f 6f 51 b0              	movdqu	-0x50(%rcx), %xmm2
1422b5d64: f3 0f 6f 59 c0              	movdqu	-0x40(%rcx), %xmm3
1422b5d69: f3 0f 7f 9d 90 01 00 00     	movdqu	%xmm3, 0x190(%rbp)
1422b5d71: f3 0f 7f 95 80 01 00 00     	movdqu	%xmm2, 0x180(%rbp)
1422b5d79: f3 0f 7f 8d 70 01 00 00     	movdqu	%xmm1, 0x170(%rbp)
1422b5d81: f3 0f 7f 85 60 01 00 00     	movdqu	%xmm0, 0x160(%rbp)
1422b5d89: 48 89 85 58 01 00 00        	movq	%rax, 0x158(%rbp)
1422b5d90: 48 8d 8d 58 01 00 00        	leaq	0x158(%rbp), %rcx
1422b5d97: 48 89 f2                    	movq	%rsi, %rdx
1422b5d9a: e8 a1 9c a8 fe              	callq	0x140d3fa40 <.text+0xd3ea40>
1422b5d9f: 48 89 c3                    	movq	%rax, %rbx
1422b5da2: 48 89 d8                    	movq	%rbx, %rax
1422b5da5: 48 81 c4 e0 02 00 00        	addq	$0x2e0, %rsp            # imm = 0x2E0
1422b5dac: 5b                          	popq	%rbx
1422b5dad: 5f                          	popq	%rdi
1422b5dae: 5e                          	popq	%rsi
1422b5daf: 41 5c                       	popq	%r12
1422b5db1: 41 5e                       	popq	%r14
1422b5db3: 41 5f                       	popq	%r15
1422b5db5: 5d                          	popq	%rbp
1422b5db6: c3                          	retq
1422b5db7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1422b5dc0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1422b5dc5: 55                          	pushq	%rbp
1422b5dc6: 41 57                       	pushq	%r15
1422b5dc8: 41 56                       	pushq	%r14
1422b5dca: 41 54                       	pushq	%r12
1422b5dcc: 56                          	pushq	%rsi
1422b5dcd: 57                          	pushq	%rdi
1422b5dce: 53                          	pushq	%rbx
1422b5dcf: 48 83 ec 30                 	subq	$0x30, %rsp
1422b5dd3: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1422b5dda: 48 8d 55 b0                 	leaq	-0x50(%rbp), %rdx
1422b5dde: 41 b8 d0 00 00 00           	movl	$0xd0, %r8d
1422b5de4: 48 8b 8d 48 02 00 00        	movq	0x248(%rbp), %rcx
1422b5deb: e8 10 de 66 00              	callq	0x142923c00 <.text+0x2922c00>
1422b5df0: c6 85 57 02 00 00 00        	movb	$0x0, 0x257(%rbp)
1422b5df7: 48 83 c4 30                 	addq	$0x30, %rsp
1422b5dfb: 5b                          	popq	%rbx
1422b5dfc: 5f                          	popq	%rdi
1422b5dfd: 5e                          	popq	%rsi
1422b5dfe: 41 5c                       	popq	%r12
1422b5e00: 41 5e                       	popq	%r14
1422b5e02: 41 5f                       	popq	%r15
1422b5e04: 5d                          	popq	%rbp
1422b5e05: c3                          	retq
1422b5e06: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
1422b5e10: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1422b5e15: 55                          	pushq	%rbp
1422b5e16: 41 57                       	pushq	%r15
1422b5e18: 41 56                       	pushq	%r14
1422b5e1a: 41 54                       	pushq	%r12
1422b5e1c: 56                          	pushq	%rsi
1422b5e1d: 57                          	pushq	%rdi
1422b5e1e: 53                          	pushq	%rbx
1422b5e1f: 48 83 ec 30                 	subq	$0x30, %rsp
1422b5e23: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1422b5e2a: 80 bd 57 02 00 00 00        	cmpb	$0x0, 0x257(%rbp)
1422b5e31: 74 0c                       	je	0x1422b5e3f <.text+0x22b4e3f>
1422b5e33: 48 8d 8d 80 00 00 00        	leaq	0x80(%rbp), %rcx
1422b5e3a: e8 11 ad 33 fe              	callq	0x1405f0b50 <.text+0x5efb50>
1422b5e3f: 90                          	nop
1422b5e40: 48 83 c4 30                 	addq	$0x30, %rsp
1422b5e44: 5b                          	popq	%rbx
1422b5e45: 5f                          	popq	%rdi
1422b5e46: 5e                          	popq	%rsi
1422b5e47: 41 5c                       	popq	%r12
1422b5e49: 41 5e                       	popq	%r14
1422b5e4b: 41 5f                       	popq	%r15
1422b5e4d: 5d                          	popq	%rbp
1422b5e4e: c3                          	retq
1422b5e4f: 90                          	nop
1422b5e50: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1422b5e55: 55                          	pushq	%rbp
1422b5e56: 41 57                       	pushq	%r15
1422b5e58: 41 56                       	pushq	%r14
1422b5e5a: 41 54                       	pushq	%r12
1422b5e5c: 56                          	pushq	%rsi
1422b5e5d: 57                          	pushq	%rdi
1422b5e5e: 53                          	pushq	%rbx
1422b5e5f: 48 83 ec 30                 	subq	$0x30, %rsp
1422b5e63: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1422b5e6a: 66 0f 6f 45 b0              	movdqa	-0x50(%rbp), %xmm0
1422b5e6f: 66 0f 6f 4d c0              	movdqa	-0x40(%rbp), %xmm1
1422b5e74: 48 8b 85 48 02 00 00        	movq	0x248(%rbp), %rax
1422b5e7b: f3 0f 7f 48 10              	movdqu	%xmm1, 0x10(%rax)
1422b5e80: f3 0f 7f 00                 	movdqu	%xmm0, (%rax)
1422b5e84: c6 85 56 02 00 00 00        	movb	$0x0, 0x256(%rbp)
1422b5e8b: 48 83 c4 30                 	addq	$0x30, %rsp
1422b5e8f: 5b                          	popq	%rbx
1422b5e90: 5f                          	popq	%rdi
1422b5e91: 5e                          	popq	%rsi
1422b5e92: 41 5c                       	popq	%r12
1422b5e94: 41 5e                       	popq	%r14
1422b5e96: 41 5f                       	popq	%r15
1422b5e98: 5d                          	popq	%rbp
1422b5e99: c3                          	retq
1422b5e9a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
1422b5ea0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1422b5ea5: 55                          	pushq	%rbp
1422b5ea6: 41 57                       	pushq	%r15
1422b5ea8: 41 56                       	pushq	%r14
1422b5eaa: 41 54                       	pushq	%r12
1422b5eac: 56                          	pushq	%rsi
1422b5ead: 57                          	pushq	%rdi
1422b5eae: 53                          	pushq	%rbx
1422b5eaf: 48 83 ec 30                 	subq	$0x30, %rsp
1422b5eb3: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1422b5eba: 80 bd 56 02 00 00 00        	cmpb	$0x0, 0x256(%rbp)
1422b5ec1: 74 1f                       	je	0x1422b5ee2 <.text+0x22b4ee2>
1422b5ec3: 48 8d 95 a0 00 00 00        	leaq	0xa0(%rbp), %rdx
1422b5eca: 48 8d 8d 80 00 00 00        	leaq	0x80(%rbp), %rcx
1422b5ed1: 41 b8 20 00 00 00           	movl	$0x20, %r8d
1422b5ed7: 41 b9 10 00 00 00           	movl	$0x10, %r9d
1422b5edd: e8 5e b4 d9 fd              	callq	0x140051340 <.text+0x50340>
1422b5ee2: 90                          	nop
1422b5ee3: 48 83 c4 30                 	addq	$0x30, %rsp
1422b5ee7: 5b                          	popq	%rbx
1422b5ee8: 5f                          	popq	%rdi
1422b5ee9: 5e                          	popq	%rsi
1422b5eea: 41 5c                       	popq	%r12
1422b5eec: 41 5e                       	popq	%r14
1422b5eee: 41 5f                       	popq	%r15
1422b5ef0: 5d                          	popq	%rbp
1422b5ef1: c3                          	retq
1422b5ef2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
1422b5f00: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1422b5f05: 55                          	pushq	%rbp
1422b5f06: 41 57                       	pushq	%r15
1422b5f08: 41 56                       	pushq	%r14
1422b5f0a: 41 54                       	pushq	%r12
1422b5f0c: 56                          	pushq	%rsi
1422b5f0d: 57                          	pushq	%rdi
1422b5f0e: 53                          	pushq	%rbx
1422b5f0f: 48 83 ec 30                 	subq	$0x30, %rsp
1422b5f13: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1422b5f1a: 48 8b 95 30 02 00 00        	movq	0x230(%rbp), %rdx
1422b5f21: 48 85 d2                    	testq	%rdx, %rdx
1422b5f24: 74 16                       	je	0x1422b5f3c <.text+0x22b4f3c>
1422b5f26: 48 8b 8d 38 02 00 00        	movq	0x238(%rbp), %rcx
1422b5f2d: 48 c1 e2 03                 	shlq	$0x3, %rdx
1422b5f31: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1422b5f37: e8 e4 dc 35 fe              	callq	0x140613c20 <.text+0x612c20>
1422b5f3c: 90                          	nop
1422b5f3d: 48 83 c4 30                 	addq	$0x30, %rsp
1422b5f41: 5b                          	popq	%rbx
1422b5f42: 5f                          	popq	%rdi
1422b5f43: 5e                          	popq	%rsi
1422b5f44: 41 5c                       	popq	%r12
1422b5f46: 41 5e                       	popq	%r14
1422b5f48: 41 5f                       	popq	%r15
1422b5f4a: 5d                          	popq	%rbp
1422b5f4b: c3                          	retq
