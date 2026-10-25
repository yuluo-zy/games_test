
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1408c9a30: 55                          	pushq	%rbp
1408c9a31: 41 57                       	pushq	%r15
1408c9a33: 41 56                       	pushq	%r14
1408c9a35: 41 55                       	pushq	%r13
1408c9a37: 41 54                       	pushq	%r12
1408c9a39: 56                          	pushq	%rsi
1408c9a3a: 57                          	pushq	%rdi
1408c9a3b: 53                          	pushq	%rbx
1408c9a3c: 48 81 ec 48 09 00 00        	subq	$0x948, %rsp            # imm = 0x948
1408c9a43: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1408c9a4b: 48 c7 85 c0 08 00 00 fe ff ff ff    	movq	$-0x2, 0x8c0(%rbp)
1408c9a56: 4c 89 c3                    	movq	%r8, %rbx
1408c9a59: 48 89 d6                    	movq	%rdx, %rsi
1408c9a5c: 48 89 cf                    	movq	%rcx, %rdi
1408c9a5f: 4c 8d 72 38                 	leaq	0x38(%rdx), %r14
1408c9a63: 48 ba 7a 9c b7 7f 61 ae 8d d3       	movabsq	$-0x2c72519e80486386, %rdx # imm = 0xD38DAE617FB79C7A
1408c9a6d: 49 b8 ae 1e ad 5d 5c df aa 22       	movabsq	$0x22aadf5c5dad1eae, %r8 # imm = 0x22AADF5C5DAD1EAE
1408c9a77: 4c 89 f1                    	movq	%r14, %rcx
1408c9a7a: e8 71 3c 81 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
1408c9a7f: 48 85 c0                    	testq	%rax, %rax
1408c9a82: 0f 84 4c 05 00 00           	je	0x1408c9fd4 <.text+0x8c8fd4>
1408c9a88: 48 39 96 18 02 00 00        	cmpq	%rdx, 0x218(%rsi)
1408c9a8f: 0f 86 3f 05 00 00           	jbe	0x1408c9fd4 <.text+0x8c8fd4>
1408c9a95: 48 8b 86 10 02 00 00        	movq	0x210(%rsi), %rax
1408c9a9c: 48 8b 0c d0                 	movq	(%rax,%rdx,8), %rcx
1408c9aa0: 48 85 c9                    	testq	%rcx, %rcx
1408c9aa3: 0f 84 2b 05 00 00           	je	0x1408c9fd4 <.text+0x8c8fd4>
1408c9aa9: 48 8b 86 e0 01 00 00        	movq	0x1e0(%rsi), %rax
1408c9ab0: 48 f7 d1                    	notq	%rcx
1408c9ab3: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
1408c9ab7: 48 c1 e1 05                 	shlq	$0x5, %rcx
1408c9abb: 48 83 7c 08 20 00           	cmpq	$0x0, 0x20(%rax,%rcx)
1408c9ac1: 0f 84 0d 05 00 00           	je	0x1408c9fd4 <.text+0x8c8fd4>
1408c9ac7: 48 01 c8                    	addq	%rcx, %rax
1408c9aca: 48 8b 40 10                 	movq	0x10(%rax), %rax
1408c9ace: 48 8b 08                    	movq	(%rax), %rcx
1408c9ad1: 48 83 f9 02                 	cmpq	$0x2, %rcx
1408c9ad5: 0f 84 b0 04 00 00           	je	0x1408c9f8b <.text+0x8c8f8b>
1408c9adb: 83 f9 0a                    	cmpl	$0xa, %ecx
1408c9ade: 75 0a                       	jne	0x1408c9aea <.text+0x8c8aea>
1408c9ae0: 80 78 08 01                 	cmpb	$0x1, 0x8(%rax)
1408c9ae4: 0f 85 a1 04 00 00           	jne	0x1408c9f8b <.text+0x8c8f8b>
1408c9aea: 4c 8d 41 fe                 	leaq	-0x2(%rcx), %r8
1408c9aee: 49 83 f8 09                 	cmpq	$0x9, %r8
1408c9af2: ba 04 00 00 00              	movl	$0x4, %edx
1408c9af7: 49 0f 42 d0                 	cmovbq	%r8, %rdx
1408c9afb: 41 b8 af 00 00 00           	movl	$0xaf, %r8d
1408c9b01: 49 0f a3 d0                 	btq	%rdx, %r8
1408c9b05: 0f 83 74 04 00 00           	jae	0x1408c9f7f <.text+0x8c8f7f>
1408c9b0b: 83 f9 07                    	cmpl	$0x7, %ecx
1408c9b0e: 75 0a                       	jne	0x1408c9b1a <.text+0x8c8b1a>
1408c9b10: 80 78 08 01                 	cmpb	$0x1, 0x8(%rax)
1408c9b14: 0f 84 71 04 00 00           	je	0x1408c9f8b <.text+0x8c8f8b>
1408c9b1a: 48 89 bd a8 08 00 00        	movq	%rdi, 0x8a8(%rbp)
1408c9b21: 48 ba ca 7b 28 c8 dc e4 32 fb       	movabsq	$-0x4cd1b2337d78436, %rdx # imm = 0xFB32E4DCC8287BCA
1408c9b2b: 49 b8 c2 7d 59 98 81 3a 38 80       	movabsq	$-0x7fc7c57e67a6823e, %r8 # imm = 0x80383A8198597DC2
1408c9b35: 4c 89 f1                    	movq	%r14, %rcx
1408c9b38: e8 b3 3b 81 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
1408c9b3d: 48 85 c0                    	testq	%rax, %rax
1408c9b40: 0f 84 9a 04 00 00           	je	0x1408c9fe0 <.text+0x8c8fe0>
1408c9b46: 48 39 96 18 02 00 00        	cmpq	%rdx, 0x218(%rsi)
1408c9b4d: 0f 86 8d 04 00 00           	jbe	0x1408c9fe0 <.text+0x8c8fe0>
1408c9b53: 48 8b 86 10 02 00 00        	movq	0x210(%rsi), %rax
1408c9b5a: 48 8b 04 d0                 	movq	(%rax,%rdx,8), %rax
1408c9b5e: 48 85 c0                    	testq	%rax, %rax
1408c9b61: 0f 84 79 04 00 00           	je	0x1408c9fe0 <.text+0x8c8fe0>
1408c9b67: 4c 8b ae e0 01 00 00        	movq	0x1e0(%rsi), %r13
1408c9b6e: 48 f7 d0                    	notq	%rax
1408c9b71: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
1408c9b75: 48 c1 e0 05                 	shlq	$0x5, %rax
1408c9b79: 49 83 7c 05 20 00           	cmpq	$0x0, 0x20(%r13,%rax)
1408c9b7f: 0f 84 5b 04 00 00           	je	0x1408c9fe0 <.text+0x8c8fe0>
1408c9b85: 49 01 c5                    	addq	%rax, %r13
1408c9b88: 4d 8b 65 10                 	movq	0x10(%r13), %r12
1408c9b8c: 8b 86 b0 05 00 00           	movl	0x5b0(%rsi), %eax
1408c9b92: 89 85 b0 08 00 00           	movl	%eax, 0x8b0(%rbp)
1408c9b98: 49 8b 3c 24                 	movq	(%r12), %rdi
1408c9b9c: 4c 8d 7f 10                 	leaq	0x10(%rdi), %r15
1408c9ba0: b1 01                       	movb	$0x1, %cl
1408c9ba2: 31 c0                       	xorl	%eax, %eax
1408c9ba4: f0                          	lock
1408c9ba5: 0f b0 4f 10                 	cmpxchgb	%cl, 0x10(%rdi)
1408c9ba9: 0f 85 fc 03 00 00           	jne	0x1408c9fab <.text+0x8c8fab>
1408c9baf: 4c 89 f9                    	movq	%r15, %rcx
1408c9bb2: e8 b9 80 3b 00              	callq	0x140c81c70 <.text+0xc80c70>
1408c9bb7: 48 8b 47 48                 	movq	0x48(%rdi), %rax
1408c9bbb: 48 89 45 00                 	movq	%rax, (%rbp)
1408c9bbf: 0f 10 47 18                 	movups	0x18(%rdi), %xmm0
1408c9bc3: 0f 10 4f 28                 	movups	0x28(%rdi), %xmm1
1408c9bc7: 0f 10 57 38                 	movups	0x38(%rdi), %xmm2
1408c9bcb: 0f 29 55 f0                 	movaps	%xmm2, -0x10(%rbp)
1408c9bcf: 0f 29 4d e0                 	movaps	%xmm1, -0x20(%rbp)
1408c9bd3: 0f 29 45 d0                 	movaps	%xmm0, -0x30(%rbp)
1408c9bd7: c6 47 4e 07                 	movb	$0x7, 0x4e(%rdi)
1408c9bdb: c6 85 be 08 00 00 01        	movb	$0x1, 0x8be(%rbp)
1408c9be2: 4c 89 f9                    	movq	%r15, %rcx
1408c9be5: e8 26 82 3b 00              	callq	0x140c81e10 <.text+0xc80e10>
1408c9bea: 31 c9                       	xorl	%ecx, %ecx
1408c9bec: b0 01                       	movb	$0x1, %al
1408c9bee: f0                          	lock
1408c9bef: 41 0f b0 0f                 	cmpxchgb	%cl, (%r15)
1408c9bf3: 0f 85 c5 03 00 00           	jne	0x1408c9fbe <.text+0x8c8fbe>
1408c9bf9: 80 7d 06 07                 	cmpb	$0x7, 0x6(%rbp)
1408c9bfd: 0f 84 94 00 00 00           	je	0x1408c9c97 <.text+0x8c8c97>
1408c9c03: 48 8b 45 00                 	movq	(%rbp), %rax
1408c9c07: 48 89 85 f0 03 00 00        	movq	%rax, 0x3f0(%rbp)
1408c9c0e: 0f 28 45 d0                 	movaps	-0x30(%rbp), %xmm0
1408c9c12: 0f 28 4d e0                 	movaps	-0x20(%rbp), %xmm1
1408c9c16: 0f 28 55 f0                 	movaps	-0x10(%rbp), %xmm2
1408c9c1a: 0f 29 95 e0 03 00 00        	movaps	%xmm2, 0x3e0(%rbp)
1408c9c21: 0f 29 8d d0 03 00 00        	movaps	%xmm1, 0x3d0(%rbp)
1408c9c28: 0f 29 85 c0 03 00 00        	movaps	%xmm0, 0x3c0(%rbp)
1408c9c2f: 8b 85 b0 08 00 00           	movl	0x8b0(%rbp), %eax
1408c9c35: 41 89 45 5c                 	movl	%eax, 0x5c(%r13)
1408c9c39: 49 8d 44 24 08              	leaq	0x8(%r12), %rax
1408c9c3e: 48 89 85 b0 08 00 00        	movq	%rax, 0x8b0(%rbp)
1408c9c45: 41 80 7c 24 3e 07           	cmpb	$0x7, 0x3e(%r12)
1408c9c4b: 74 18                       	je	0x1408c9c65 <.text+0x8c8c65>
1408c9c4d: 48 8b 8d b0 08 00 00        	movq	0x8b0(%rbp), %rcx
1408c9c54: e8 27 1f 77 ff              	callq	0x14003bb80 <.text+0x3ab80>
1408c9c59: 48 8b 8d b0 08 00 00        	movq	0x8b0(%rbp), %rcx
1408c9c60: e8 4b 88 31 00              	callq	0x140be24b0 <.text+0xbe14b0>
1408c9c65: 48 8b 85 f0 03 00 00        	movq	0x3f0(%rbp), %rax
1408c9c6c: 48 8b 8d b0 08 00 00        	movq	0x8b0(%rbp), %rcx
1408c9c73: 48 89 41 30                 	movq	%rax, 0x30(%rcx)
1408c9c77: 0f 28 85 c0 03 00 00        	movaps	0x3c0(%rbp), %xmm0
1408c9c7e: 0f 28 8d d0 03 00 00        	movaps	0x3d0(%rbp), %xmm1
1408c9c85: 0f 28 95 e0 03 00 00        	movaps	0x3e0(%rbp), %xmm2
1408c9c8c: 0f 11 51 20                 	movups	%xmm2, 0x20(%rcx)
1408c9c90: 0f 11 49 10                 	movups	%xmm1, 0x10(%rcx)
1408c9c94: 0f 11 01                    	movups	%xmm0, (%rcx)
1408c9c97: 41 80 7c 24 3e 07           	cmpb	$0x7, 0x3e(%r12)
1408c9c9d: 48 8b bd a8 08 00 00        	movq	0x8a8(%rbp), %rdi
1408c9ca4: 0f 84 e1 02 00 00           	je	0x1408c9f8b <.text+0x8c8f8b>
1408c9caa: 48 ba b3 61 b4 27 07 ee 97 85       	movabsq	$-0x7a6811f8d84b9e4d, %rdx # imm = 0x8597EE0727B461B3
1408c9cb4: 49 b8 5c f1 15 ae c5 7e 29 8b       	movabsq	$-0x74d6813a51ea0ea4, %r8 # imm = 0x8B297EC5AE15F15C
1408c9cbe: 4c 89 f1                    	movq	%r14, %rcx
1408c9cc1: e8 2a 3a 81 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
1408c9cc6: 48 85 c0                    	testq	%rax, %rax
1408c9cc9: 0f 84 1d 03 00 00           	je	0x1408c9fec <.text+0x8c8fec>
1408c9ccf: 48 39 96 18 02 00 00        	cmpq	%rdx, 0x218(%rsi)
1408c9cd6: 0f 86 10 03 00 00           	jbe	0x1408c9fec <.text+0x8c8fec>
1408c9cdc: 48 8b 86 10 02 00 00        	movq	0x210(%rsi), %rax
1408c9ce3: 48 8b 0c d0                 	movq	(%rax,%rdx,8), %rcx
1408c9ce7: 48 85 c9                    	testq	%rcx, %rcx
1408c9cea: 0f 84 fc 02 00 00           	je	0x1408c9fec <.text+0x8c8fec>
1408c9cf0: 48 8b 86 e0 01 00 00        	movq	0x1e0(%rsi), %rax
1408c9cf7: 48 f7 d1                    	notq	%rcx
1408c9cfa: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
1408c9cfe: 48 c1 e1 05                 	shlq	$0x5, %rcx
1408c9d02: 48 83 7c 08 20 00           	cmpq	$0x0, 0x20(%rax,%rcx)
1408c9d08: 0f 84 de 02 00 00           	je	0x1408c9fec <.text+0x8c8fec>
1408c9d0e: 48 01 c8                    	addq	%rcx, %rax
1408c9d11: 48 8b 48 10                 	movq	0x10(%rax), %rcx
1408c9d15: 48 8d 15 c4 94 1d 02        	leaq	0x21d94c4(%rip), %rdx   # 0x142aa31e0
1408c9d1c: 41 b8 18 00 00 00           	movl	$0x18, %r8d
1408c9d22: e8 79 dd 06 00              	callq	0x140937aa0 <.text+0x936aa0>
1408c9d27: a8 01                       	testb	$0x1, %al
1408c9d29: 0f 84 c9 02 00 00           	je	0x1408c9ff8 <.text+0x8c8ff8>
1408c9d2f: 49 89 d6                    	movq	%rdx, %r14
1408c9d32: 48 8d 4b 04                 	leaq	0x4(%rbx), %rcx
1408c9d36: e8 55 ba 30 00              	callq	0x140bd5790 <.text+0xbd4790>
1408c9d3b: c7 85 c0 03 00 00 02 00 00 00       	movl	$0x2, 0x3c0(%rbp)
1408c9d45: 48 89 9d c8 03 00 00        	movq	%rbx, 0x3c8(%rbp)
1408c9d4c: c7 85 a8 07 00 00 02 00 00 00       	movl	$0x2, 0x7a8(%rbp)
1408c9d56: 48 89 7c 24 40              	movq	%rdi, 0x40(%rsp)
1408c9d5b: 48 89 74 24 38              	movq	%rsi, 0x38(%rsp)
1408c9d60: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
1408c9d65: 48 8d 85 a8 07 00 00        	leaq	0x7a8(%rbp), %rax
1408c9d6c: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
1408c9d71: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1408c9d7a: 48 8d 15 8f 94 1d 02        	leaq	0x21d948f(%rip), %rdx   # 0x142aa3210
1408c9d81: 48 8d 4d d0                 	leaq	-0x30(%rbp), %rcx
1408c9d85: 4c 8d 8d c0 03 00 00        	leaq	0x3c0(%rbp), %r9
1408c9d8c: 41 b8 07 00 00 00           	movl	$0x7, %r8d
1408c9d92: e8 d9 db f9 ff              	callq	0x140867970 <.text+0x866970>
1408c9d97: c6 85 bf 08 00 00 01        	movb	$0x1, 0x8bf(%rbp)
1408c9d9e: e8 ad c9 31 00              	callq	0x140be6750 <.text+0xbe5750>
1408c9da3: 89 c6                       	movl	%eax, %esi
1408c9da5: c6 85 bf 08 00 00 01        	movb	$0x1, 0x8bf(%rbp)
1408c9dac: 31 c9                       	xorl	%ecx, %ecx
1408c9dae: e8 6d fd 80 ff              	callq	0x1400d9b20 <.text+0xd8b20>
1408c9db3: 48 85 c0                    	testq	%rax, %rax
1408c9db6: 0f 84 48 02 00 00           	je	0x1408ca004 <.text+0x8c9004>
1408c9dbc: 89 b5 68 04 00 00           	movl	%esi, 0x468(%rbp)
1408c9dc2: 66 c7 85 70 04 00 00 00 00  	movw	$0x0, 0x470(%rbp)
1408c9dcb: c7 85 6c 04 00 00 00 00 80 3f       	movl	$0x3f800000, 0x46c(%rbp) # imm = 0x3F800000
1408c9dd5: 48 c7 85 c0 03 00 00 00 00 00 00    	movq	$0x0, 0x3c0(%rbp)
1408c9de0: 4c 89 b5 60 04 00 00        	movq	%r14, 0x460(%rbp)
1408c9de7: 48 c7 85 d0 03 00 00 00 00 00 00    	movq	$0x0, 0x3d0(%rbp)
1408c9df2: 48 c7 85 d8 03 00 00 04 00 00 00    	movq	$0x4, 0x3d8(%rbp)
1408c9dfd: 48 c7 85 e0 03 00 00 00 00 00 00    	movq	$0x0, 0x3e0(%rbp)
1408c9e08: 0f 10 00                    	movups	(%rax), %xmm0
1408c9e0b: 48 ff 00                    	incq	(%rax)
1408c9e0e: 0f 10 0d 1b c9 1c 02        	movups	0x21cc91b(%rip), %xmm1  # 0x142a96730
1408c9e15: 0f 11 8d 40 04 00 00        	movups	%xmm1, 0x440(%rbp)
1408c9e1c: 0f 10 0d fd c8 1c 02        	movups	0x21cc8fd(%rip), %xmm1  # 0x142a96720
1408c9e23: 0f 11 8d 30 04 00 00        	movups	%xmm1, 0x430(%rbp)
1408c9e2a: 0f 11 85 50 04 00 00        	movups	%xmm0, 0x450(%rbp)
1408c9e31: 48 b8 03 00 00 00 01 00 00 00       	movabsq	$0x100000003, %rax # imm = 0x100000003
1408c9e3b: 48 89 85 00 04 00 00        	movq	%rax, 0x400(%rbp)
1408c9e42: 48 c7 85 e8 03 00 00 00 00 00 00    	movq	$0x0, 0x3e8(%rbp)
1408c9e4d: 48 c7 85 f0 03 00 00 01 00 00 00    	movq	$0x1, 0x3f0(%rbp)
1408c9e58: 48 c7 85 f8 03 00 00 00 00 00 00    	movq	$0x0, 0x3f8(%rbp)
1408c9e63: c6 85 bf 08 00 00 01        	movb	$0x1, 0x8bf(%rbp)
1408c9e6a: 48 8d 8d a8 07 00 00        	leaq	0x7a8(%rbp), %rcx
1408c9e71: 48 8d b5 c0 03 00 00        	leaq	0x3c0(%rbp), %rsi
1408c9e78: 4c 8d 45 d0                 	leaq	-0x30(%rbp), %r8
1408c9e7c: 48 89 f2                    	movq	%rsi, %rdx
1408c9e7f: 45 31 c9                    	xorl	%r9d, %r9d
1408c9e82: e8 59 bd 03 00              	callq	0x140905be0 <.text+0x904be0>
1408c9e87: 49 83 c4 08                 	addq	$0x8, %r12
1408c9e8b: 4c 89 a5 a0 08 00 00        	movq	%r12, 0x8a0(%rbp)
1408c9e92: c7 85 c0 03 00 00 02 00 00 00       	movl	$0x2, 0x3c0(%rbp)
1408c9e9c: 48 89 74 24 20              	movq	%rsi, 0x20(%rsp)
1408c9ea1: 48 8d 8d a0 08 00 00        	leaq	0x8a0(%rbp), %rcx
1408c9ea8: 48 8d 95 a8 07 00 00        	leaq	0x7a8(%rbp), %rdx
1408c9eaf: 49 b8 dd b3 40 cc 97 56 ba 11       	movabsq	$0x11ba5697cc40b3dd, %r8 # imm = 0x11BA5697CC40B3DD
1408c9eb9: 41 b1 1d                    	movb	$0x1d, %r9b
1408c9ebc: e8 ff 37 03 00              	callq	0x1408fd6c0 <.text+0x8fc6c0>
1408c9ec1: 48 8b b5 a8 07 00 00        	movq	0x7a8(%rbp), %rsi
1408c9ec8: 48 8d 95 b0 07 00 00        	leaq	0x7b0(%rbp), %rdx
1408c9ecf: 48 8d 8d c8 03 00 00        	leaq	0x3c8(%rbp), %rcx
1408c9ed6: 41 b8 a0 00 00 00           	movl	$0xa0, %r8d
1408c9edc: e8 1f 9d 05 02              	callq	0x142923c00 <.text+0x2922c00>
1408c9ee1: 0f b6 85 50 08 00 00        	movzbl	0x850(%rbp), %eax
1408c9ee8: 0f 10 85 51 08 00 00        	movups	0x851(%rbp), %xmm0
1408c9eef: 0f 11 85 69 04 00 00        	movups	%xmm0, 0x469(%rbp)
1408c9ef6: 0f 10 85 61 08 00 00        	movups	0x861(%rbp), %xmm0
1408c9efd: 0f 11 85 79 04 00 00        	movups	%xmm0, 0x479(%rbp)
1408c9f04: 0f 10 85 71 08 00 00        	movups	0x871(%rbp), %xmm0
1408c9f0b: 0f 11 85 89 04 00 00        	movups	%xmm0, 0x489(%rbp)
1408c9f12: 0f 10 85 81 08 00 00        	movups	0x881(%rbp), %xmm0
1408c9f19: 0f 11 85 99 04 00 00        	movups	%xmm0, 0x499(%rbp)
1408c9f20: 0f 10 85 90 08 00 00        	movups	0x890(%rbp), %xmm0
1408c9f27: 0f 11 85 a8 04 00 00        	movups	%xmm0, 0x4a8(%rbp)
1408c9f2e: 24 7f                       	andb	$0x7f, %al
1408c9f30: 48 89 b5 c0 03 00 00        	movq	%rsi, 0x3c0(%rbp)
1408c9f37: 88 85 68 04 00 00           	movb	%al, 0x468(%rbp)
1408c9f3d: c6 85 bf 08 00 00 01        	movb	$0x1, 0x8bf(%rbp)
1408c9f44: 48 8d 8d c0 03 00 00        	leaq	0x3c0(%rbp), %rcx
1408c9f4b: ba 03 00 00 00              	movl	$0x3, %edx
1408c9f50: e8 3b cb 03 00              	callq	0x140906a90 <.text+0x905a90>
1408c9f55: 48 8d b5 c0 03 00 00        	leaq	0x3c0(%rbp), %rsi
1408c9f5c: 48 8d 55 d0                 	leaq	-0x30(%rbp), %rdx
1408c9f60: 41 b8 e8 03 00 00           	movl	$0x3e8, %r8d            # imm = 0x3E8
1408c9f66: 48 89 f1                    	movq	%rsi, %rcx
1408c9f69: e8 92 9c 05 02              	callq	0x142923c00 <.text+0x2922c00>
1408c9f6e: c6 85 bf 08 00 00 00        	movb	$0x0, 0x8bf(%rbp)
1408c9f75: 48 89 f1                    	movq	%rsi, %rcx
1408c9f78: e8 f3 e3 f9 ff              	callq	0x140868370 <.text+0x867370>
1408c9f7d: eb 0c                       	jmp	0x1408c9f8b <.text+0x8c8f8b>
1408c9f7f: 41 b8 50 00 00 00           	movl	$0x50, %r8d
1408c9f85: 49 0f a3 d0                 	btq	%rdx, %r8
1408c9f89: 73 14                       	jae	0x1408c9f9f <.text+0x8c8f9f>
1408c9f8b: 48 81 c4 48 09 00 00        	addq	$0x948, %rsp            # imm = 0x948
1408c9f92: 5b                          	popq	%rbx
1408c9f93: 5f                          	popq	%rdi
1408c9f94: 5e                          	popq	%rsi
1408c9f95: 41 5c                       	popq	%r12
1408c9f97: 41 5d                       	popq	%r13
1408c9f99: 41 5e                       	popq	%r14
1408c9f9b: 41 5f                       	popq	%r15
1408c9f9d: 5d                          	popq	%rbp
1408c9f9e: c3                          	retq
1408c9f9f: 80 78 08 00                 	cmpb	$0x0, 0x8(%rax)
1408c9fa3: 0f 84 62 fb ff ff           	je	0x1408c9b0b <.text+0x8c8b0b>
1408c9fa9: eb e0                       	jmp	0x1408c9f8b <.text+0x8c8f8b>
1408c9fab: 4c 89 f9                    	movq	%r15, %rcx
1408c9fae: 41 b8 00 ca 9a 3b           	movl	$0x3b9aca00, %r8d       # imm = 0x3B9ACA00
1408c9fb4: e8 57 32 00 02              	callq	0x1428cd210 <.text+0x28cc210>
1408c9fb9: e9 f1 fb ff ff              	jmp	0x1408c9baf <.text+0x8c8baf>
1408c9fbe: c6 85 be 08 00 00 01        	movb	$0x1, 0x8be(%rbp)
1408c9fc5: 4c 89 f9                    	movq	%r15, %rcx
1408c9fc8: 31 d2                       	xorl	%edx, %edx
1408c9fca: e8 c1 3b 00 02              	callq	0x1428cdb90 <.text+0x28ccb90>
1408c9fcf: e9 25 fc ff ff              	jmp	0x1408c9bf9 <.text+0x8c8bf9>
1408c9fd4: 48 8d 0d bd 91 1d 02        	leaq	0x21d91bd(%rip), %rcx   # 0x142aa3198
1408c9fdb: e8 30 f3 00 02              	callq	0x1428d9310 <.text+0x28d8310>
1408c9fe0: 48 8d 0d c9 91 1d 02        	leaq	0x21d91c9(%rip), %rcx   # 0x142aa31b0
1408c9fe7: e8 24 f3 00 02              	callq	0x1428d9310 <.text+0x28d8310>
1408c9fec: 48 8d 0d d5 91 1d 02        	leaq	0x21d91d5(%rip), %rcx   # 0x142aa31c8
1408c9ff3: e8 18 f3 00 02              	callq	0x1428d9310 <.text+0x28d8310>
1408c9ff8: 48 8d 0d f9 91 1d 02        	leaq	0x21d91f9(%rip), %rcx   # 0x142aa31f8
1408c9fff: e8 0c f3 00 02              	callq	0x1428d9310 <.text+0x28d8310>
1408ca004: c6 85 bf 08 00 00 01        	movb	$0x1, 0x8bf(%rbp)
1408ca00b: 48 8d 0d 8e 0a 37 02        	leaq	0x2370a8e(%rip), %rcx   # 0x142c3aaa0
1408ca012: e8 b9 e0 00 02              	callq	0x1428d80d0 <.text+0x28d70d0>
1408ca017: 0f 0b                       	ud2
1408ca019: 0f 1f 80 00 00 00 00        	nopl	(%rax)
1408ca020: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1408ca025: 55                          	pushq	%rbp
1408ca026: 41 57                       	pushq	%r15
1408ca028: 41 56                       	pushq	%r14
1408ca02a: 41 55                       	pushq	%r13
1408ca02c: 41 54                       	pushq	%r12
1408ca02e: 56                          	pushq	%rsi
1408ca02f: 57                          	pushq	%rdi
1408ca030: 53                          	pushq	%rbx
1408ca031: 48 83 ec 48                 	subq	$0x48, %rsp
1408ca035: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1408ca03c: 80 bd bf 08 00 00 00        	cmpb	$0x0, 0x8bf(%rbp)
1408ca043: 75 11                       	jne	0x1408ca056 <.text+0x8c9056>
1408ca045: 48 83 c4 48                 	addq	$0x48, %rsp
1408ca049: 5b                          	popq	%rbx
1408ca04a: 5f                          	popq	%rdi
1408ca04b: 5e                          	popq	%rsi
1408ca04c: 41 5c                       	popq	%r12
1408ca04e: 41 5d                       	popq	%r13
1408ca050: 41 5e                       	popq	%r14
1408ca052: 41 5f                       	popq	%r15
1408ca054: 5d                          	popq	%rbp
1408ca055: c3                          	retq
1408ca056: 48 8d 4d d0                 	leaq	-0x30(%rbp), %rcx
1408ca05a: e8 61 83 fc ff              	callq	0x1408923c0 <.text+0x8913c0>
1408ca05f: eb e4                       	jmp	0x1408ca045 <.text+0x8c9045>
1408ca061: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
1408ca070: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1408ca075: 55                          	pushq	%rbp
1408ca076: 41 57                       	pushq	%r15
1408ca078: 41 56                       	pushq	%r14
1408ca07a: 41 55                       	pushq	%r13
1408ca07c: 41 54                       	pushq	%r12
1408ca07e: 56                          	pushq	%rsi
1408ca07f: 57                          	pushq	%rdi
1408ca080: 53                          	pushq	%rbx
1408ca081: 48 83 ec 48                 	subq	$0x48, %rsp
1408ca085: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1408ca08c: 80 7d 06 07                 	cmpb	$0x7, 0x6(%rbp)
1408ca090: 74 09                       	je	0x1408ca09b <.text+0x8c909b>
1408ca092: f6 85 be 08 00 00 01        	testb	$0x1, 0x8be(%rbp)
1408ca099: 75 11                       	jne	0x1408ca0ac <.text+0x8c90ac>
1408ca09b: 48 83 c4 48                 	addq	$0x48, %rsp
1408ca09f: 5b                          	popq	%rbx
1408ca0a0: 5f                          	popq	%rdi
1408ca0a1: 5e                          	popq	%rsi
1408ca0a2: 41 5c                       	popq	%r12
1408ca0a4: 41 5d                       	popq	%r13
1408ca0a6: 41 5e                       	popq	%r14
1408ca0a8: 41 5f                       	popq	%r15
1408ca0aa: 5d                          	popq	%rbp
1408ca0ab: c3                          	retq
1408ca0ac: 48 8d 4d d0                 	leaq	-0x30(%rbp), %rcx
1408ca0b0: e8 2b 45 d2 ff              	callq	0x1405ee5e0 <.text+0x5ed5e0>
1408ca0b5: eb e4                       	jmp	0x1408ca09b <.text+0x8c909b>
1408ca0b7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1408ca0c0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1408ca0c5: 55                          	pushq	%rbp
1408ca0c6: 41 57                       	pushq	%r15
1408ca0c8: 41 56                       	pushq	%r14
1408ca0ca: 41 55                       	pushq	%r13
1408ca0cc: 41 54                       	pushq	%r12
1408ca0ce: 56                          	pushq	%rsi
1408ca0cf: 57                          	pushq	%rdi
1408ca0d0: 53                          	pushq	%rbx
1408ca0d1: 48 83 ec 48                 	subq	$0x48, %rsp
1408ca0d5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1408ca0dc: 48 8b 8d b0 08 00 00        	movq	0x8b0(%rbp), %rcx
1408ca0e3: e8 c8 83 31 00              	callq	0x140be24b0 <.text+0xbe14b0>
1408ca0e8: 90                          	nop
1408ca0e9: 48 83 c4 48                 	addq	$0x48, %rsp
1408ca0ed: 5b                          	popq	%rbx
1408ca0ee: 5f                          	popq	%rdi
1408ca0ef: 5e                          	popq	%rsi
1408ca0f0: 41 5c                       	popq	%r12
1408ca0f2: 41 5d                       	popq	%r13
1408ca0f4: 41 5e                       	popq	%r14
1408ca0f6: 41 5f                       	popq	%r15
1408ca0f8: 5d                          	popq	%rbp
1408ca0f9: c3                          	retq
1408ca0fa: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
1408ca100: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1408ca105: 55                          	pushq	%rbp
1408ca106: 41 57                       	pushq	%r15
1408ca108: 41 56                       	pushq	%r14
1408ca10a: 41 55                       	pushq	%r13
1408ca10c: 41 54                       	pushq	%r12
1408ca10e: 56                          	pushq	%rsi
1408ca10f: 57                          	pushq	%rdi
1408ca110: 53                          	pushq	%rbx
1408ca111: 48 83 ec 48                 	subq	$0x48, %rsp
1408ca115: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1408ca11c: 48 8b 85 f0 03 00 00        	movq	0x3f0(%rbp), %rax
1408ca123: 48 8b 8d b0 08 00 00        	movq	0x8b0(%rbp), %rcx
1408ca12a: 48 89 41 30                 	movq	%rax, 0x30(%rcx)
1408ca12e: 0f 28 85 c0 03 00 00        	movaps	0x3c0(%rbp), %xmm0
1408ca135: 0f 28 8d d0 03 00 00        	movaps	0x3d0(%rbp), %xmm1
1408ca13c: 0f 28 95 e0 03 00 00        	movaps	0x3e0(%rbp), %xmm2
1408ca143: 0f 11 51 20                 	movups	%xmm2, 0x20(%rcx)
1408ca147: 0f 11 49 10                 	movups	%xmm1, 0x10(%rcx)
1408ca14b: 0f 11 01                    	movups	%xmm0, (%rcx)
1408ca14e: c6 85 be 08 00 00 00        	movb	$0x0, 0x8be(%rbp)
1408ca155: 48 83 c4 48                 	addq	$0x48, %rsp
1408ca159: 5b                          	popq	%rbx
1408ca15a: 5f                          	popq	%rdi
1408ca15b: 5e                          	popq	%rsi
1408ca15c: 41 5c                       	popq	%r12
1408ca15e: 41 5d                       	popq	%r13
1408ca160: 41 5e                       	popq	%r14
1408ca162: 41 5f                       	popq	%r15
1408ca164: 5d                          	popq	%rbp
1408ca165: c3                          	retq
1408ca166: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
1408ca170: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1408ca175: 55                          	pushq	%rbp
1408ca176: 41 57                       	pushq	%r15
1408ca178: 41 56                       	pushq	%r14
1408ca17a: 41 55                       	pushq	%r13
1408ca17c: 41 54                       	pushq	%r12
1408ca17e: 56                          	pushq	%rsi
1408ca17f: 57                          	pushq	%rdi
1408ca180: 53                          	pushq	%rbx
1408ca181: 48 83 ec 48                 	subq	$0x48, %rsp
1408ca185: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1408ca18c: c6 85 bf 08 00 00 01        	movb	$0x1, 0x8bf(%rbp)
1408ca193: 48 8d 8d a8 07 00 00        	leaq	0x7a8(%rbp), %rcx
1408ca19a: e8 21 94 fc ff              	callq	0x1408935c0 <.text+0x8925c0>
1408ca19f: c6 85 bf 08 00 00 01        	movb	$0x1, 0x8bf(%rbp)
1408ca1a6: 48 83 c4 48                 	addq	$0x48, %rsp
1408ca1aa: 5b                          	popq	%rbx
1408ca1ab: 5f                          	popq	%rdi
1408ca1ac: 5e                          	popq	%rsi
1408ca1ad: 41 5c                       	popq	%r12
1408ca1af: 41 5d                       	popq	%r13
1408ca1b1: 41 5e                       	popq	%r14
1408ca1b3: 41 5f                       	popq	%r15
1408ca1b5: 5d                          	popq	%rbp
1408ca1b6: c3                          	retq
