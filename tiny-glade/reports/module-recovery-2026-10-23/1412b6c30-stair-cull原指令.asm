
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1412b6c30: 55                          	pushq	%rbp
1412b6c31: 41 57                       	pushq	%r15
1412b6c33: 41 56                       	pushq	%r14
1412b6c35: 41 55                       	pushq	%r13
1412b6c37: 41 54                       	pushq	%r12
1412b6c39: 56                          	pushq	%rsi
1412b6c3a: 57                          	pushq	%rdi
1412b6c3b: 53                          	pushq	%rbx
1412b6c3c: 48 81 ec c8 01 00 00        	subq	$0x1c8, %rsp            # imm = 0x1C8
1412b6c43: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1412b6c4b: 66 0f 7f bd 30 01 00 00     	movdqa	%xmm7, 0x130(%rbp)
1412b6c53: 66 0f 7f b5 20 01 00 00     	movdqa	%xmm6, 0x120(%rbp)
1412b6c5b: 48 c7 85 18 01 00 00 fe ff ff ff    	movq	$-0x2, 0x118(%rbp)
1412b6c66: 4c 89 8d e0 00 00 00        	movq	%r9, 0xe0(%rbp)
1412b6c6d: 4c 89 c3                    	movq	%r8, %rbx
1412b6c70: 49 89 d6                    	movq	%rdx, %r14
1412b6c73: 48 89 cf                    	movq	%rcx, %rdi
1412b6c76: e8 c5 6a a9 ff              	callq	0x140d4d740 <.text+0xd4c740>
1412b6c7b: 48 89 c6                    	movq	%rax, %rsi
1412b6c7e: f6 00 01                    	testb	$0x1, (%rax)
1412b6c81: 4c 89 b5 88 00 00 00        	movq	%r14, 0x88(%rbp)
1412b6c88: 74 11                       	je	0x1412b6c9b <.text+0x12b5c9b>
1412b6c8a: 48 89 f1                    	movq	%rsi, %rcx
1412b6c8d: 48 83 c1 08                 	addq	$0x8, %rcx
1412b6c91: 48 8b 46 08                 	movq	0x8(%rsi), %rax
1412b6c95: 48 8b 56 10                 	movq	0x10(%rsi), %rdx
1412b6c99: eb 1b                       	jmp	0x1412b6cb6 <.text+0x12b5cb6>
1412b6c9b: e8 60 e1 aa ff              	callq	0x140d64e00 <.text+0xd63e00>
1412b6ca0: 48 89 f1                    	movq	%rsi, %rcx
1412b6ca3: 48 83 c1 08                 	addq	$0x8, %rcx
1412b6ca7: 48 c7 06 01 00 00 00        	movq	$0x1, (%rsi)
1412b6cae: 48 89 46 08                 	movq	%rax, 0x8(%rsi)
1412b6cb2: 48 89 56 10                 	movq	%rdx, 0x10(%rsi)
1412b6cb6: 4c 8d 40 01                 	leaq	0x1(%rax), %r8
1412b6cba: 4c 89 01                    	movq	%r8, (%rcx)
1412b6cbd: 0f 10 05 6c 51 97 01        	movups	0x197516c(%rip), %xmm0  # 0x142c2be30
1412b6cc4: 0f 29 45 40                 	movaps	%xmm0, 0x40(%rbp)
1412b6cc8: f3 0f 6f 05 50 51 97 01     	movdqu	0x1975150(%rip), %xmm0  # 0x142c2be20
1412b6cd0: 66 0f 7f 45 30              	movdqa	%xmm0, 0x30(%rbp)
1412b6cd5: 48 89 45 50                 	movq	%rax, 0x50(%rbp)
1412b6cd9: 48 89 55 58                 	movq	%rdx, 0x58(%rbp)
1412b6cdd: 48 89 9d 80 00 00 00        	movq	%rbx, 0x80(%rbp)
1412b6ce4: 48 8b 13                    	movq	(%rbx), %rdx
1412b6ce7: c6 85 17 01 00 00 01        	movb	$0x1, 0x117(%rbp)
1412b6cee: 48 8d 4d c0                 	leaq	-0x40(%rbp), %rcx
1412b6cf2: 48 89 55 68                 	movq	%rdx, 0x68(%rbp)
1412b6cf6: e8 c5 75 76 ff              	callq	0x140a1e2c0 <.text+0xa1d2c0>
1412b6cfb: 48 8b 85 c8 01 00 00        	movq	0x1c8(%rbp), %rax
1412b6d02: 48 8b 4d c0                 	movq	-0x40(%rbp), %rcx
1412b6d06: 48 89 8d b0 00 00 00        	movq	%rcx, 0xb0(%rbp)
1412b6d0d: 48 8b 4d c8                 	movq	-0x38(%rbp), %rcx
1412b6d11: 48 89 8d f0 00 00 00        	movq	%rcx, 0xf0(%rbp)
1412b6d18: 4c 8b 65 d0                 	movq	-0x30(%rbp), %r12
1412b6d1c: 4c 8b 6d d8                 	movq	-0x28(%rbp), %r13
1412b6d20: 0f b7 4d e8                 	movzwl	-0x18(%rbp), %ecx
1412b6d24: 48 89 8d 00 01 00 00        	movq	%rcx, 0x100(%rbp)
1412b6d2b: 48 8b 4d f0                 	movq	-0x10(%rbp), %rcx
1412b6d2f: 48 89 8d 08 01 00 00        	movq	%rcx, 0x108(%rbp)
1412b6d36: 4c 8b 7d f8                 	movq	-0x8(%rbp), %r15
1412b6d3a: 48 8b 5d 00                 	movq	(%rbp), %rbx
1412b6d3e: 48 8b 0f                    	movq	(%rdi), %rcx
1412b6d41: 48 89 8d b8 00 00 00        	movq	%rcx, 0xb8(%rbp)
1412b6d48: 48 8b 00                    	movq	(%rax), %rax
1412b6d4b: 48 89 85 c0 00 00 00        	movq	%rax, 0xc0(%rbp)
1412b6d52: 66 0f 6e 35 3e d9 67 01     	movd	0x167d93e(%rip), %xmm6  # 0x142934698
1412b6d5a: f3 0f 7e 3d 9e 50 97 01     	movq	0x197509e(%rip), %xmm7  # 0x142c2be00
1412b6d62: 0f b7 7d 10                 	movzwl	0x10(%rbp), %edi
1412b6d66: 48 8b 45 18                 	movq	0x18(%rbp), %rax
1412b6d6a: 48 89 85 f8 00 00 00        	movq	%rax, 0xf8(%rbp)
1412b6d71: 48 8b 85 e0 00 00 00        	movq	0xe0(%rbp), %rax
1412b6d78: 48 8b 00                    	movq	(%rax), %rax
1412b6d7b: 48 89 85 a0 00 00 00        	movq	%rax, 0xa0(%rbp)
1412b6d82: eb 54                       	jmp	0x1412b6dd8 <.text+0x12b5dd8>
1412b6d84: 66 0f 6f c6                 	movdqa	%xmm6, %xmm0
1412b6d88: f3 0f 7e 8d d4 00 00 00     	movq	0xd4(%rbp), %xmm1
1412b6d90: 66 0f 6e d0                 	movd	%eax, %xmm2
1412b6d94: 66 0f 70 d2 50              	pshufd	$0x50, %xmm2, %xmm2     # xmm2 = xmm2[0,0,1,1]
1412b6d99: 66 0f 72 f2 1f              	pslld	$0x1f, %xmm2
1412b6d9e: 66 0f 72 e2 1f              	psrad	$0x1f, %xmm2
1412b6da3: 66 0f db ca                 	pand	%xmm2, %xmm1
1412b6da7: 66 0f df d7                 	pandn	%xmm7, %xmm2
1412b6dab: 66 0f eb d1                 	por	%xmm1, %xmm2
1412b6daf: 66 0f d6 55 70              	movq	%xmm2, 0x70(%rbp)
1412b6db4: 66 0f 7e 45 78              	movd	%xmm0, 0x78(%rbp)
1412b6db9: c6 85 17 01 00 00 01        	movb	$0x1, 0x117(%rbp)
1412b6dc0: 48 8d 8d d0 00 00 00        	leaq	0xd0(%rbp), %rcx
1412b6dc7: 48 8d 55 30                 	leaq	0x30(%rbp), %rdx
1412b6dcb: 4c 8d 45 c0                 	leaq	-0x40(%rbp), %r8
1412b6dcf: 4c 8d 4d 70                 	leaq	0x70(%rbp), %r9
1412b6dd3: e8 38 e9 fa ff              	callq	0x141265710 <.text+0x1264710>
1412b6dd8: f6 85 b0 00 00 00 01        	testb	$0x1, 0xb0(%rbp)
1412b6ddf: 74 3f                       	je	0x1412b6e20 <.text+0x12b5e20>
1412b6de1: 4d 85 e4                    	testq	%r12, %r12
1412b6de4: 74 15                       	je	0x1412b6dfb <.text+0x12b5dfb>
1412b6de6: 48 83 bd 08 01 00 00 00     	cmpq	$0x0, 0x108(%rbp)
1412b6dee: 75 3f                       	jne	0x1412b6e2f <.text+0x12b5e2f>
1412b6df0: 48 c7 85 08 01 00 00 00 00 00 00    	movq	$0x0, 0x108(%rbp)
1412b6dfb: 48 8b 85 f0 00 00 00        	movq	0xf0(%rbp), %rax
1412b6e02: 48 85 c0                    	testq	%rax, %rax
1412b6e05: 48 8b 95 f8 00 00 00        	movq	0xf8(%rbp), %rdx
1412b6e0c: 0f 85 cf 00 00 00           	jne	0x1412b6ee1 <.text+0x12b5ee1>
1412b6e12: 31 c0                       	xorl	%eax, %eax
1412b6e14: 4d 85 ff                    	testq	%r15, %r15
1412b6e17: 75 65                       	jne	0x1412b6e7e <.text+0x12b5e7e>
1412b6e19: e9 21 02 00 00              	jmp	0x1412b703f <.text+0x12b603f>
1412b6e1e: 66 90                       	nop
1412b6e20: 4d 85 e4                    	testq	%r12, %r12
1412b6e23: 74 42                       	je	0x1412b6e67 <.text+0x12b5e67>
1412b6e25: 48 83 bd 08 01 00 00 00     	cmpq	$0x0, 0x108(%rbp)
1412b6e2d: 74 2d                       	je	0x1412b6e5c <.text+0x12b5e5c>
1412b6e2f: 48 8b 95 00 01 00 00        	movq	0x100(%rbp), %rdx
1412b6e36: 66 85 d2                    	testw	%dx, %dx
1412b6e39: 0f 84 e1 00 00 00           	je	0x1412b6f20 <.text+0x12b5f20>
1412b6e3f: 8d 4a ff                    	leal	-0x1(%rdx), %ecx
1412b6e42: 21 d1                       	andl	%edx, %ecx
1412b6e44: 48 ff 8d 08 01 00 00        	decq	0x108(%rbp)
1412b6e4b: 89 d0                       	movl	%edx, %eax
1412b6e4d: 4c 89 e2                    	movq	%r12, %rdx
1412b6e50: 48 89 8d 00 01 00 00        	movq	%rcx, 0x100(%rbp)
1412b6e57: e9 04 01 00 00              	jmp	0x1412b6f60 <.text+0x12b5f60>
1412b6e5c: 48 c7 85 08 01 00 00 00 00 00 00    	movq	$0x0, 0x108(%rbp)
1412b6e67: 48 8b 95 f8 00 00 00        	movq	0xf8(%rbp), %rdx
1412b6e6e: 48 8b 85 f0 00 00 00        	movq	0xf0(%rbp), %rax
1412b6e75: 4d 85 ff                    	testq	%r15, %r15
1412b6e78: 0f 84 c1 01 00 00           	je	0x1412b703f <.text+0x12b603f>
1412b6e7e: 48 85 d2                    	testq	%rdx, %rdx
1412b6e81: 0f 84 b8 01 00 00           	je	0x1412b703f <.text+0x12b603f>
1412b6e87: 66 85 ff                    	testw	%di, %di
1412b6e8a: 48 89 85 f0 00 00 00        	movq	%rax, 0xf0(%rbp)
1412b6e91: 74 0d                       	je	0x1412b6ea0 <.text+0x12b5ea0>
1412b6e93: 8d 4f ff                    	leal	-0x1(%rdi), %ecx
1412b6e96: 21 f9                       	andl	%edi, %ecx
1412b6e98: eb 2e                       	jmp	0x1412b6ec8 <.text+0x12b5ec8>
1412b6e9a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
1412b6ea0: 66 0f 6f 03                 	movdqa	(%rbx), %xmm0
1412b6ea4: 66 0f d7 f8                 	pmovmskb	%xmm0, %edi
1412b6ea8: 49 81 c7 00 fb ff ff        	addq	$-0x500, %r15           # imm = 0xFB00
1412b6eaf: 48 83 c3 10                 	addq	$0x10, %rbx
1412b6eb3: 81 ff ff ff 00 00           	cmpl	$0xffff, %edi           # imm = 0xFFFF
1412b6eb9: 74 e5                       	je	0x1412b6ea0 <.text+0x12b5ea0>
1412b6ebb: b8 fe ff ff ff              	movl	$0xfffffffe, %eax       # imm = 0xFFFFFFFE
1412b6ec0: 29 f8                       	subl	%edi, %eax
1412b6ec2: f7 d7                       	notl	%edi
1412b6ec4: 89 f9                       	movl	%edi, %ecx
1412b6ec6: 21 c1                       	andl	%eax, %ecx
1412b6ec8: 48 ff ca                    	decq	%rdx
1412b6ecb: 48 89 95 f8 00 00 00        	movq	%rdx, 0xf8(%rbp)
1412b6ed2: 89 f8                       	movl	%edi, %eax
1412b6ed4: 4c 89 fa                    	movq	%r15, %rdx
1412b6ed7: 89 cf                       	movl	%ecx, %edi
1412b6ed9: 45 31 e4                    	xorl	%r12d, %r12d
1412b6edc: e9 7f 00 00 00              	jmp	0x1412b6f60 <.text+0x12b5f60>
1412b6ee1: 4c 8b 60 30                 	movq	0x30(%rax), %r12
1412b6ee5: 48 8b 48 48                 	movq	0x48(%rax), %rcx
1412b6ee9: 66 41 0f 6f 04 24           	movdqa	(%r12), %xmm0
1412b6eef: 66 0f d7 d0                 	pmovmskb	%xmm0, %edx
1412b6ef3: f7 d2                       	notl	%edx
1412b6ef5: 4d 8d 6c 24 10              	leaq	0x10(%r12), %r13
1412b6efa: 48 89 8d 08 01 00 00        	movq	%rcx, 0x108(%rbp)
1412b6f01: 48 85 c9                    	testq	%rcx, %rcx
1412b6f04: 0f 84 11 01 00 00           	je	0x1412b701b <.text+0x12b601b>
1412b6f0a: 48 c7 85 f0 00 00 00 00 00 00 00    	movq	$0x0, 0xf0(%rbp)
1412b6f15: 66 85 d2                    	testw	%dx, %dx
1412b6f18: 0f 85 21 ff ff ff           	jne	0x1412b6e3f <.text+0x12b5e3f>
1412b6f1e: 66 90                       	nop
1412b6f20: 66 41 0f 6f 45 00           	movdqa	(%r13), %xmm0
1412b6f26: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
1412b6f2a: 49 81 c4 00 fb ff ff        	addq	$-0x500, %r12           # imm = 0xFB00
1412b6f31: 49 83 c5 10                 	addq	$0x10, %r13
1412b6f35: 3d ff ff 00 00              	cmpl	$0xffff, %eax           # imm = 0xFFFF
1412b6f3a: 74 e4                       	je	0x1412b6f20 <.text+0x12b5f20>
1412b6f3c: b9 fe ff ff ff              	movl	$0xfffffffe, %ecx       # imm = 0xFFFFFFFE
1412b6f41: 29 c1                       	subl	%eax, %ecx
1412b6f43: f7 d0                       	notl	%eax
1412b6f45: 89 c2                       	movl	%eax, %edx
1412b6f47: 21 ca                       	andl	%ecx, %edx
1412b6f49: 48 89 95 00 01 00 00        	movq	%rdx, 0x100(%rbp)
1412b6f50: 48 ff 8d 08 01 00 00        	decq	0x108(%rbp)
1412b6f57: 4c 89 e2                    	movq	%r12, %rdx
1412b6f5a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
1412b6f60: f3 0f bc c0                 	tzcntl	%eax, %eax
1412b6f64: 48 f7 d8                    	negq	%rax
1412b6f67: 48 8d 34 80                 	leaq	(%rax,%rax,4), %rsi
1412b6f6b: 48 c1 e6 04                 	shlq	$0x4, %rsi
1412b6f6f: 49 89 d6                    	movq	%rdx, %r14
1412b6f72: 8b 54 32 b0                 	movl	-0x50(%rdx,%rsi), %edx
1412b6f76: c6 85 17 01 00 00 01        	movb	$0x1, 0x117(%rbp)
1412b6f7d: 48 8b 8d a0 00 00 00        	movq	0xa0(%rbp), %rcx
1412b6f84: 89 95 a8 00 00 00           	movl	%edx, 0xa8(%rbp)
1412b6f8a: e8 01 4e 87 ff              	callq	0x140b2bd90 <.text+0xb2ad90>
1412b6f8f: 84 c0                       	testb	%al, %al
1412b6f91: 0f 84 41 fe ff ff           	je	0x1412b6dd8 <.text+0x12b5dd8>
1412b6f97: 49 8d 0c 36                 	leaq	(%r14,%rsi), %rcx
1412b6f9b: 48 83 c1 b8                 	addq	$-0x48, %rcx
1412b6f9f: c6 85 17 01 00 00 01        	movb	$0x1, 0x117(%rbp)
1412b6fa6: 48 8d 15 33 4f 97 01        	leaq	0x1974f33(%rip), %rdx   # 0x142c2bee0
1412b6fad: e8 be c4 57 ff              	callq	0x140833470 <.text+0x832470>
1412b6fb2: 8b 10                       	movl	(%rax), %edx
1412b6fb4: 44 8b 40 04                 	movl	0x4(%rax), %r8d
1412b6fb8: c6 85 17 01 00 00 01        	movb	$0x1, 0x117(%rbp)
1412b6fbf: 48 8b 8d b8 00 00 00        	movq	0xb8(%rbp), %rcx
1412b6fc6: e8 b5 9e 64 ff              	callq	0x140900e80 <.text+0x8ffe80>
1412b6fcb: 84 c0                       	testb	%al, %al
1412b6fcd: 0f 85 05 fe ff ff           	jne	0x1412b6dd8 <.text+0x12b5dd8>
1412b6fd3: 48 c7 45 c0 02 00 00 00     	movq	$0x2, -0x40(%rbp)
1412b6fdb: 44 8b 85 a8 00 00 00        	movl	0xa8(%rbp), %r8d
1412b6fe2: 44 89 45 d0                 	movl	%r8d, -0x30(%rbp)
1412b6fe6: c6 85 17 01 00 00 01        	movb	$0x1, 0x117(%rbp)
1412b6fed: 48 8d 8d d0 00 00 00        	leaq	0xd0(%rbp), %rcx
1412b6ff4: 48 8b 95 c0 00 00 00        	movq	0xc0(%rbp), %rdx
1412b6ffb: e8 80 3b 76 ff              	callq	0x140a1ab80 <.text+0xa19b80>
1412b7000: 8b 85 d0 00 00 00           	movl	0xd0(%rbp), %eax
1412b7006: 85 c0                       	testl	%eax, %eax
1412b7008: 0f 84 76 fd ff ff           	je	0x1412b6d84 <.text+0x12b5d84>
1412b700e: 66 0f 6e 85 dc 00 00 00     	movd	0xdc(%rbp), %xmm0
1412b7016: e9 6d fd ff ff              	jmp	0x1412b6d88 <.text+0x12b5d88>
1412b701b: 48 89 95 00 01 00 00        	movq	%rdx, 0x100(%rbp)
1412b7022: 48 c7 85 08 01 00 00 00 00 00 00    	movq	$0x0, 0x108(%rbp)
1412b702d: 31 c0                       	xorl	%eax, %eax
1412b702f: 48 8b 95 f8 00 00 00        	movq	0xf8(%rbp), %rdx
1412b7036: 4d 85 ff                    	testq	%r15, %r15
1412b7039: 0f 85 3f fe ff ff           	jne	0x1412b6e7e <.text+0x12b5e7e>
1412b703f: 48 8b 7d 30                 	movq	0x30(%rbp), %rdi
1412b7043: 4c 8b 45 38                 	movq	0x38(%rbp), %r8
1412b7047: 66 0f 6f 07                 	movdqa	(%rdi), %xmm0
1412b704b: 4d 85 c0                    	testq	%r8, %r8
1412b704e: 4c 89 85 90 00 00 00        	movq	%r8, 0x90(%rbp)
1412b7055: 74 3e                       	je	0x1412b7095 <.text+0x12b6095>
1412b7057: 49 8d 40 01                 	leaq	0x1(%r8), %rax
1412b705b: b9 28 00 00 00              	movl	$0x28, %ecx
1412b7060: 48 f7 e1                    	mulq	%rcx
1412b7063: 48 83 c0 0f                 	addq	$0xf, %rax
1412b7067: 48 83 e0 f0                 	andq	$-0x10, %rax
1412b706b: 49 8d 0c 00                 	leaq	(%r8,%rax), %rcx
1412b706f: 48 83 c1 11                 	addq	$0x11, %rcx
1412b7073: 48 89 8d e8 00 00 00        	movq	%rcx, 0xe8(%rbp)
1412b707a: 48 89 f9                    	movq	%rdi, %rcx
1412b707d: 48 29 c1                    	subq	%rax, %rcx
1412b7080: 48 89 8d 98 00 00 00        	movq	%rcx, 0x98(%rbp)
1412b7087: b8 10 00 00 00              	movl	$0x10, %eax
1412b708c: 48 89 85 c8 00 00 00        	movq	%rax, 0xc8(%rbp)
1412b7093: eb 0b                       	jmp	0x1412b70a0 <.text+0x12b60a0>
1412b7095: 48 c7 85 c8 00 00 00 00 00 00 00    	movq	$0x0, 0xc8(%rbp)
1412b70a0: 48 8b 8d 80 00 00 00        	movq	0x80(%rbp), %rcx
1412b70a7: 48 8b 95 88 00 00 00        	movq	0x88(%rbp), %rdx
1412b70ae: 4c 8b 85 b0 01 00 00        	movq	0x1b0(%rbp), %r8
1412b70b5: 4c 8b 8d b8 01 00 00        	movq	0x1b8(%rbp), %r9
1412b70bc: 4c 8b 6d 48                 	movq	0x48(%rbp), %r13
1412b70c0: 4c 8d 77 10                 	leaq	0x10(%rdi), %r14
1412b70c4: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
1412b70c8: f7 d0                       	notl	%eax
1412b70ca: 45 8b 51 1c                 	movl	0x1c(%r9), %r10d
1412b70ce: 44 89 95 08 01 00 00        	movl	%r10d, 0x108(%rbp)
1412b70d5: 49 8b 19                    	movq	(%r9), %rbx
1412b70d8: 4d 8b 49 10                 	movq	0x10(%r9), %r9
1412b70dc: 4c 89 8d a8 00 00 00        	movq	%r9, 0xa8(%rbp)
1412b70e3: 48 8b 12                    	movq	(%rdx), %rdx
1412b70e6: 48 89 95 f8 00 00 00        	movq	%rdx, 0xf8(%rbp)
1412b70ed: 8b 51 1c                    	movl	0x1c(%rcx), %edx
1412b70f0: 89 95 b0 00 00 00           	movl	%edx, 0xb0(%rbp)
1412b70f6: 48 8b 49 10                 	movq	0x10(%rcx), %rcx
1412b70fa: 48 89 8d 00 01 00 00        	movq	%rcx, 0x100(%rbp)
1412b7101: 48 8b 8d e0 00 00 00        	movq	0xe0(%rbp), %rcx
1412b7108: 8b 51 1c                    	movl	0x1c(%rcx), %edx
1412b710b: 89 95 b8 00 00 00           	movl	%edx, 0xb8(%rbp)
1412b7111: 48 8b 49 10                 	movq	0x10(%rcx), %rcx
1412b7115: 48 89 8d c0 00 00 00        	movq	%rcx, 0xc0(%rbp)
1412b711c: 49 8b 08                    	movq	(%r8), %rcx
1412b711f: 48 89 8d e0 00 00 00        	movq	%rcx, 0xe0(%rbp)
1412b7126: 48 8d 4b 20                 	leaq	0x20(%rbx), %rcx
1412b712a: 48 89 8d f0 00 00 00        	movq	%rcx, 0xf0(%rbp)
1412b7131: 4c 8d a5 d0 00 00 00        	leaq	0xd0(%rbp), %r12
1412b7138: 49 83 ed 01                 	subq	$0x1, %r13
1412b713c: 0f 82 7c 01 00 00           	jb	0x1412b72be <.text+0x12b62be>
1412b7142: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
1412b7150: 66 85 c0                    	testw	%ax, %ax
1412b7153: 74 0b                       	je	0x1412b7160 <.text+0x12b6160>
1412b7155: 44 8d 78 ff                 	leal	-0x1(%rax), %r15d
1412b7159: 41 21 c7                    	andl	%eax, %r15d
1412b715c: eb 2c                       	jmp	0x1412b718a <.text+0x12b618a>
1412b715e: 66 90                       	nop
1412b7160: 66 41 0f 6f 06              	movdqa	(%r14), %xmm0
1412b7165: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
1412b7169: 48 81 c7 80 fd ff ff        	addq	$-0x280, %rdi           # imm = 0xFD80
1412b7170: 49 83 c6 10                 	addq	$0x10, %r14
1412b7174: 3d ff ff 00 00              	cmpl	$0xffff, %eax           # imm = 0xFFFF
1412b7179: 74 e5                       	je	0x1412b7160 <.text+0x12b6160>
1412b717b: b9 fe ff ff ff              	movl	$0xfffffffe, %ecx       # imm = 0xFFFFFFFE
1412b7180: 29 c1                       	subl	%eax, %ecx
1412b7182: f7 d0                       	notl	%eax
1412b7184: 41 89 c7                    	movl	%eax, %r15d
1412b7187: 41 21 cf                    	andl	%ecx, %r15d
1412b718a: f3 0f bc c0                 	tzcntl	%eax, %eax
1412b718e: 48 f7 d8                    	negq	%rax
1412b7191: 48 8d 04 80                 	leaq	(%rax,%rax,4), %rax
1412b7195: f3 0f 6f 74 c7 d8           	movdqu	-0x28(%rdi,%rax,8), %xmm6
1412b719b: 48 8b 4c c7 d8              	movq	-0x28(%rdi,%rax,8), %rcx
1412b71a0: 8b 74 c7 e8                 	movl	-0x18(%rdi,%rax,8), %esi
1412b71a4: f3 0f 6f 44 c7 ec           	movdqu	-0x14(%rdi,%rax,8), %xmm0
1412b71aa: 66 0f 7f 45 70              	movdqa	%xmm0, 0x70(%rbp)
1412b71af: 48 83 f9 03                 	cmpq	$0x3, %rcx
1412b71b3: 0f 84 05 01 00 00           	je	0x1412b72be <.text+0x12b62be>
1412b71b9: 48 8b 8d a0 00 00 00        	movq	0xa0(%rbp), %rcx
1412b71c0: 89 f2                       	movl	%esi, %edx
1412b71c2: e8 c9 4b 87 ff              	callq	0x140b2bd90 <.text+0xb2ad90>
1412b71c7: 84 c0                       	testb	%al, %al
1412b71c9: 0f 84 3f 01 00 00           	je	0x1412b730e <.text+0x12b630e>
1412b71cf: 8b 85 08 01 00 00           	movl	0x108(%rbp), %eax
1412b71d5: 48 8b 8d a8 00 00 00        	movq	0xa8(%rbp), %rcx
1412b71dc: 89 01                       	movl	%eax, (%rcx)
1412b71de: 89 b5 d0 00 00 00           	movl	%esi, 0xd0(%rbp)
1412b71e4: 48 8b 8d f0 00 00 00        	movq	0xf0(%rbp), %rcx
1412b71eb: 4c 89 e2                    	movq	%r12, %rdx
1412b71ee: e8 dd 9c 2f ff              	callq	0x1405b0ed0 <.text+0x5afed0>
1412b71f3: 48 89 d9                    	movq	%rbx, %rcx
1412b71f6: 48 89 c2                    	movq	%rax, %rdx
1412b71f9: 4d 89 e0                    	movq	%r12, %r8
1412b71fc: e8 1f d6 fa ff              	callq	0x141264820 <.text+0x1263820>
1412b7201: 48 89 55 c0                 	movq	%rdx, -0x40(%rbp)
1412b7205: 48 85 d2                    	testq	%rdx, %rdx
1412b7208: 74 0f                       	je	0x1412b7219 <.text+0x12b6219>
1412b720a: f0                          	lock
1412b720b: 48 ff 0a                    	decq	(%rdx)
1412b720e: 75 09                       	jne	0x1412b7219 <.text+0x12b6219>
1412b7210: 48 8d 4d c0                 	leaq	-0x40(%rbp), %rcx
1412b7214: e8 87 3f 57 ff              	callq	0x14082b1a0 <.text+0x82a1a0>
1412b7219: f3 0f 7f 75 c8              	movdqu	%xmm6, -0x38(%rbp)
1412b721e: 89 75 d8                    	movl	%esi, -0x28(%rbp)
1412b7221: 48 c7 45 c0 04 00 00 00     	movq	$0x4, -0x40(%rbp)
1412b7229: 48 8b 8d f8 00 00 00        	movq	0xf8(%rbp), %rcx
1412b7230: e8 3b f7 7c ff              	callq	0x140a86970 <.text+0xa85970>
1412b7235: 34 01                       	xorb	$0x1, %al
1412b7237: 48 8d 55 74                 	leaq	0x74(%rbp), %rdx
1412b723b: 8b 4a 08                    	movl	0x8(%rdx), %ecx
1412b723e: 89 8d d8 00 00 00           	movl	%ecx, 0xd8(%rbp)
1412b7244: 48 8b 0a                    	movq	(%rdx), %rcx
1412b7247: 48 89 8d d0 00 00 00        	movq	%rcx, 0xd0(%rbp)
1412b724e: 88 85 dc 00 00 00           	movb	%al, 0xdc(%rbp)
1412b7254: 8b 85 b0 00 00 00           	movl	0xb0(%rbp), %eax
1412b725a: 48 8b 8d 00 01 00 00        	movq	0x100(%rbp), %rcx
1412b7261: 89 01                       	movl	%eax, (%rcx)
1412b7263: 8b 85 b8 00 00 00           	movl	0xb8(%rbp), %eax
1412b7269: 48 8b 8d c0 00 00 00        	movq	0xc0(%rbp), %rcx
1412b7270: 89 01                       	movl	%eax, (%rcx)
1412b7272: 48 8b 85 e0 00 00 00        	movq	0xe0(%rbp), %rax
1412b7279: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
1412b727e: 48 8b 85 c0 01 00 00        	movq	0x1c0(%rbp), %rax
1412b7285: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
1412b728a: 48 8b 85 a0 00 00 00        	movq	0xa0(%rbp), %rax
1412b7291: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1412b7296: 48 c7 44 24 28 00 00 00 00  	movq	$0x0, 0x28(%rsp)
1412b729f: 48 8d 4d c0                 	leaq	-0x40(%rbp), %rcx
1412b72a3: 31 d2                       	xorl	%edx, %edx
1412b72a5: 4d 89 e0                    	movq	%r12, %r8
1412b72a8: 4c 8b 4d 68                 	movq	0x68(%rbp), %r9
1412b72ac: e8 7f fb fb ff              	callq	0x141276e30 <.text+0x1275e30>
1412b72b1: 44 89 f8                    	movl	%r15d, %eax
1412b72b4: 49 83 ed 01                 	subq	$0x1, %r13
1412b72b8: 0f 83 92 fe ff ff           	jae	0x1412b7150 <.text+0x12b6150>
1412b72be: 48 83 bd 90 00 00 00 00     	cmpq	$0x0, 0x90(%rbp)
1412b72c6: 74 24                       	je	0x1412b72ec <.text+0x12b62ec>
1412b72c8: 48 83 bd e8 00 00 00 00     	cmpq	$0x0, 0xe8(%rbp)
1412b72d0: 74 1a                       	je	0x1412b72ec <.text+0x12b62ec>
1412b72d2: 48 8b 8d 98 00 00 00        	movq	0x98(%rbp), %rcx
1412b72d9: 48 8b 95 e8 00 00 00        	movq	0xe8(%rbp), %rdx
1412b72e0: 4c 8b 85 c8 00 00 00        	movq	0xc8(%rbp), %r8
1412b72e7: e8 34 c9 35 ff              	callq	0x140613c20 <.text+0x612c20>
1412b72ec: 0f 28 b5 20 01 00 00        	movaps	0x120(%rbp), %xmm6
1412b72f3: 0f 28 bd 30 01 00 00        	movaps	0x130(%rbp), %xmm7
1412b72fa: 48 81 c4 c8 01 00 00        	addq	$0x1c8, %rsp            # imm = 0x1C8
1412b7301: 5b                          	popq	%rbx
1412b7302: 5f                          	popq	%rdi
1412b7303: 5e                          	popq	%rsi
1412b7304: 41 5c                       	popq	%r12
1412b7306: 41 5d                       	popq	%r13
1412b7308: 41 5e                       	popq	%r14
1412b730a: 41 5f                       	popq	%r15
1412b730c: 5d                          	popq	%rbp
1412b730d: c3                          	retq
1412b730e: 48 8d 0d 2b 4b 97 01        	leaq	0x1974b2b(%rip), %rcx   # 0x142c2be40
1412b7315: 4c 8d 05 ac 4b 97 01        	leaq	0x1974bac(%rip), %r8    # 0x142c2bec8
1412b731c: ba 52 00 00 00              	movl	$0x52, %edx
1412b7321: e8 0a 21 62 01              	callq	0x1428d9430 <.text+0x28d8430>
1412b7326: 0f 0b                       	ud2
