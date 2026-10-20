
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140841b00: 55                          	pushq	%rbp
140841b01: 41 57                       	pushq	%r15
140841b03: 41 56                       	pushq	%r14
140841b05: 41 55                       	pushq	%r13
140841b07: 41 54                       	pushq	%r12
140841b09: 56                          	pushq	%rsi
140841b0a: 57                          	pushq	%rdi
140841b0b: 53                          	pushq	%rbx
140841b0c: 48 81 ec 38 01 00 00        	subq	$0x138, %rsp            # imm = 0x138
140841b13: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140841b1b: 0f 29 b5 a0 00 00 00        	movaps	%xmm6, 0xa0(%rbp)
140841b22: 48 c7 85 98 00 00 00 fe ff ff ff    	movq	$-0x2, 0x98(%rbp)
140841b2d: 48 89 4d 70                 	movq	%rcx, 0x70(%rbp)
140841b31: 4c 89 45 58                 	movq	%r8, 0x58(%rbp)
140841b35: 4c 89 4d 60                 	movq	%r9, 0x60(%rbp)
140841b39: 48 89 d1                    	movq	%rdx, %rcx
140841b3c: 4c 89 c2                    	movq	%r8, %rdx
140841b3f: 4d 89 c8                    	movq	%r9, %r8
140841b42: e8 89 67 1f 00              	callq	0x140a382d0 <.text+0xa372d0>
140841b47: 48 89 c7                    	movq	%rax, %rdi
140841b4a: 48 89 d3                    	movq	%rdx, %rbx
140841b4d: 48 c7 45 78 00 00 00 00     	movq	$0x0, 0x78(%rbp)
140841b55: 48 c7 85 80 00 00 00 04 00 00 00    	movq	$0x4, 0x80(%rbp)
140841b60: 48 c7 85 88 00 00 00 00 00 00 00    	movq	$0x0, 0x88(%rbp)
140841b6b: 48 8b 05 36 a8 de 02        	movq	0x2dea836(%rip), %rax   # 0x14362c3a8
140841b72: 48 8b 00                    	movq	(%rax), %rax
140841b75: 48 83 f8 04                 	cmpq	$0x4, %rax
140841b79: 0f 86 bc 00 00 00           	jbe	0x140841c3b <.text+0x840c3b>
140841b7f: 48 89 5d 68                 	movq	%rbx, 0x68(%rbp)
140841b83: 48 8d 45 68                 	leaq	0x68(%rbp), %rax
140841b87: 48 89 45 b8                 	movq	%rax, -0x48(%rbp)
140841b8b: 48 8d 05 2e bb 54 00        	leaq	0x54bb2e(%rip), %rax    # 0x140d8d6c0 <.text+0xd8c6c0>
140841b92: 48 89 45 c0                 	movq	%rax, -0x40(%rbp)
140841b96: 48 8d 45 58                 	leaq	0x58(%rbp), %rax
140841b9a: 48 89 45 c8                 	movq	%rax, -0x38(%rbp)
140841b9e: 48 8d 05 1b 7c d1 00        	leaq	0xd17c1b(%rip), %rax    # 0x1415597c0 <.text+0x15587c0>
140841ba5: 48 89 45 d0                 	movq	%rax, -0x30(%rbp)
140841ba9: 48 8d 0d 60 27 25 02        	leaq	0x2252760(%rip), %rcx   # 0x142a94310
140841bb0: e8 cb c1 e3 ff              	callq	0x14067dd80 <.text+0x67cd80>
140841bb5: 0f 10 00                    	movups	(%rax), %xmm0
140841bb8: 8b 40 10                    	movl	0x10(%rax), %eax
140841bbb: 48 c7 45 08 05 00 00 00     	movq	$0x5, 0x8(%rbp)
140841bc3: 48 8d 0d 56 26 25 02        	leaq	0x2252656(%rip), %rcx   # 0x142a94220
140841bca: 48 89 4d 10                 	movq	%rcx, 0x10(%rbp)
140841bce: 48 c7 45 18 1d 00 00 00     	movq	$0x1d, 0x18(%rbp)
140841bd6: 48 8d 15 13 27 25 02        	leaq	0x2252713(%rip), %rdx   # 0x142a942f0
140841bdd: 48 89 55 28                 	movq	%rdx, 0x28(%rbp)
140841be1: 48 c7 45 30 02 00 00 00     	movq	$0x2, 0x30(%rbp)
140841be9: 48 8d 55 b8                 	leaq	-0x48(%rbp), %rdx
140841bed: 48 89 55 38                 	movq	%rdx, 0x38(%rbp)
140841bf1: 48 c7 45 40 02 00 00 00     	movq	$0x2, 0x40(%rbp)
140841bf9: 48 c7 45 48 00 00 00 00     	movq	$0x0, 0x48(%rbp)
140841c01: 48 c7 45 d8 00 00 00 00     	movq	$0x0, -0x28(%rbp)
140841c09: 48 89 4d e0                 	movq	%rcx, -0x20(%rbp)
140841c0d: 48 c7 45 e8 1d 00 00 00     	movq	$0x1d, -0x18(%rbp)
140841c15: 48 c7 45 f0 00 00 00 00     	movq	$0x0, -0x10(%rbp)
140841c1d: 0f 11 45 f8                 	movups	%xmm0, -0x8(%rbp)
140841c21: c7 45 20 01 00 00 00        	movl	$0x1, 0x20(%rbp)
140841c28: 89 45 24                    	movl	%eax, 0x24(%rbp)
140841c2b: 48 8d 8d 97 00 00 00        	leaq	0x97(%rbp), %rcx
140841c32: 48 8d 55 d8                 	leaq	-0x28(%rbp), %rdx
140841c36: e8 f5 86 50 00              	callq	0x140d4a330 <.text+0xd49330>
140841c3b: 48 8b 85 28 01 00 00        	movq	0x128(%rbp), %rax
140841c42: 4c 8b bd 20 01 00 00        	movq	0x120(%rbp), %r15
140841c49: 4c 8d 24 df                 	leaq	(%rdi,%rbx,8), %r12
140841c4d: f3 41 0f 10 77 0c           	movss	0xc(%r15), %xmm6
140841c53: 44 8b 68 1c                 	movl	0x1c(%rax), %r13d
140841c57: 48 8b 18                    	movq	(%rax), %rbx
140841c5a: 48 8b 70 10                 	movq	0x10(%rax), %rsi
140841c5e: 4c 39 e7                    	cmpq	%r12, %rdi
140841c61: 74 68                       	je	0x140841ccb <.text+0x840ccb>
140841c63: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
140841c70: 44 8b 37                    	movl	(%rdi), %r14d
140841c73: 0f 28 c6                    	movaps	%xmm6, %xmm0
140841c76: e8 a5 82 07 00              	callq	0x1408b9f20 <.text+0x8b8f20>
140841c7b: 44 89 2e                    	movl	%r13d, (%rsi)
140841c7e: 0f b6 57 04                 	movzbl	0x4(%rdi), %edx
140841c82: 41 8b 4f 08                 	movl	0x8(%r15), %ecx
140841c86: 89 4d e0                    	movl	%ecx, -0x20(%rbp)
140841c89: 49 8b 0f                    	movq	(%r15), %rcx
140841c8c: 48 89 4d d8                 	movq	%rcx, -0x28(%rbp)
140841c90: 66 89 45 e4                 	movw	%ax, -0x1c(%rbp)
140841c94: 66 44 89 75 e6              	movw	%r14w, -0x1a(%rbp)
140841c99: 48 8d 45 78                 	leaq	0x78(%rbp), %rax
140841c9d: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
140841ca2: 4c 89 7c 24 20              	movq	%r15, 0x20(%rsp)
140841ca7: 48 c7 44 24 28 01 00 00 00  	movq	$0x1, 0x28(%rsp)
140841cb0: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140841cb6: 48 89 d9                    	movq	%rbx, %rcx
140841cb9: 4c 8d 45 d8                 	leaq	-0x28(%rbp), %r8
140841cbd: e8 0e b4 d2 00              	callq	0x14156d0d0 <.text+0x156c0d0>
140841cc2: 48 83 c7 08                 	addq	$0x8, %rdi
140841cc6: 4c 39 e7                    	cmpq	%r12, %rdi
140841cc9: 75 a5                       	jne	0x140841c70 <.text+0x840c70>
140841ccb: 48 8b 85 88 00 00 00        	movq	0x88(%rbp), %rax
140841cd2: 48 8b 4d 70                 	movq	0x70(%rbp), %rcx
140841cd6: 48 89 41 10                 	movq	%rax, 0x10(%rcx)
140841cda: 0f 10 45 78                 	movups	0x78(%rbp), %xmm0
140841cde: 0f 11 01                    	movups	%xmm0, (%rcx)
140841ce1: 0f 28 b5 a0 00 00 00        	movaps	0xa0(%rbp), %xmm6
140841ce8: 48 81 c4 38 01 00 00        	addq	$0x138, %rsp            # imm = 0x138
140841cef: 5b                          	popq	%rbx
140841cf0: 5f                          	popq	%rdi
140841cf1: 5e                          	popq	%rsi
140841cf2: 41 5c                       	popq	%r12
140841cf4: 41 5d                       	popq	%r13
140841cf6: 41 5e                       	popq	%r14
140841cf8: 41 5f                       	popq	%r15
140841cfa: 5d                          	popq	%rbp
140841cfb: c3                          	retq
140841cfc: 0f 1f 40 00                 	nopl	(%rax)
140841d00: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140841d05: 55                          	pushq	%rbp
140841d06: 41 57                       	pushq	%r15
140841d08: 41 56                       	pushq	%r14
140841d0a: 41 55                       	pushq	%r13
140841d0c: 41 54                       	pushq	%r12
140841d0e: 56                          	pushq	%rsi
140841d0f: 57                          	pushq	%rdi
140841d10: 53                          	pushq	%rbx
140841d11: 48 83 ec 48                 	subq	$0x48, %rsp
140841d15: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140841d1c: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
140841d21: 48 8b 45 78                 	movq	0x78(%rbp), %rax
140841d25: 48 85 c0                    	testq	%rax, %rax
140841d28: 74 1a                       	je	0x140841d44 <.text+0x840d44>
140841d2a: 48 8b 8d 80 00 00 00        	movq	0x80(%rbp), %rcx
140841d31: 48 c1 e0 02                 	shlq	$0x2, %rax
140841d35: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
140841d39: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140841d3f: e8 dc 1e dd ff              	callq	0x140613c20 <.text+0x612c20>
140841d44: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
140841d49: 48 83 c4 48                 	addq	$0x48, %rsp
140841d4d: 5b                          	popq	%rbx
140841d4e: 5f                          	popq	%rdi
140841d4f: 5e                          	popq	%rsi
140841d50: 41 5c                       	popq	%r12
140841d52: 41 5d                       	popq	%r13
140841d54: 41 5e                       	popq	%r14
140841d56: 41 5f                       	popq	%r15
140841d58: 5d                          	popq	%rbp
140841d59: c3                          	retq
140841d5a: cc                          	int3
140841d5b: cc                          	int3
140841d5c: cc                          	int3
140841d5d: cc                          	int3
140841d5e: cc                          	int3
140841d5f: cc                          	int3
