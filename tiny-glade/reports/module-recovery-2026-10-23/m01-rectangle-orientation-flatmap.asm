
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1412f1b70: 56                          	pushq	%rsi
1412f1b71: 57                          	pushq	%rdi
1412f1b72: 53                          	pushq	%rbx
1412f1b73: 48 81 ec a0 00 00 00        	subq	$0xa0, %rsp
1412f1b7a: 0f 29 b4 24 90 00 00 00     	movaps	%xmm6, 0x90(%rsp)
1412f1b82: 48 89 d7                    	movq	%rdx, %rdi
1412f1b85: 48 89 ce                    	movq	%rcx, %rsi
1412f1b88: 48 8b 4a 20                 	movq	0x20(%rdx), %rcx
1412f1b8c: 48 85 c9                    	testq	%rcx, %rcx
1412f1b8f: 74 59                       	je	0x1412f1bea <.text+0x12f0bea>
1412f1b91: 48 8b 47 28                 	movq	0x28(%rdi), %rax
1412f1b95: 48 8b 57 38                 	movq	0x38(%rdi), %rdx
1412f1b99: 48 39 d0                    	cmpq	%rdx, %rax
1412f1b9c: 74 28                       	je	0x1412f1bc6 <.text+0x12f0bc6>
1412f1b9e: 48 8d 48 24                 	leaq	0x24(%rax), %rcx
1412f1ba2: 48 89 4f 28                 	movq	%rcx, 0x28(%rdi)
1412f1ba6: 8b 48 20                    	movl	0x20(%rax), %ecx
1412f1ba9: 89 4e 24                    	movl	%ecx, 0x24(%rsi)
1412f1bac: 0f 10 00                    	movups	(%rax), %xmm0
1412f1baf: 0f 10 48 10                 	movups	0x10(%rax), %xmm1
1412f1bb3: 0f 11 4e 14                 	movups	%xmm1, 0x14(%rsi)
1412f1bb7: 0f 11 46 04                 	movups	%xmm0, 0x4(%rsi)
1412f1bbb: c7 06 01 00 00 00           	movl	$0x1, (%rsi)
1412f1bc1: e9 b9 01 00 00              	jmp	0x1412f1d7f <.text+0x12f0d7f>
1412f1bc6: 48 8b 47 30                 	movq	0x30(%rdi), %rax
1412f1bca: 48 85 c0                    	testq	%rax, %rax
1412f1bcd: 74 13                       	je	0x1412f1be2 <.text+0x12f0be2>
1412f1bcf: 48 c1 e0 02                 	shlq	$0x2, %rax
1412f1bd3: 48 8d 14 c0                 	leaq	(%rax,%rax,8), %rdx
1412f1bd7: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1412f1bdd: e8 3e 20 32 ff              	callq	0x140613c20 <.text+0x612c20>
1412f1be2: 48 c7 47 20 00 00 00 00     	movq	$0x0, 0x20(%rdi)
1412f1bea: 48 8b 07                    	movq	(%rdi), %rax
1412f1bed: 48 85 c0                    	testq	%rax, %rax
1412f1bf0: 0f 84 06 01 00 00           	je	0x1412f1cfc <.text+0x12f0cfc>
1412f1bf6: 48 3b 47 08                 	cmpq	0x8(%rdi), %rax
1412f1bfa: 0f 84 fc 00 00 00           	je	0x1412f1cfc <.text+0x12f0cfc>
1412f1c00: 48 8d 48 10                 	leaq	0x10(%rax), %rcx
1412f1c04: 48 89 0f                    	movq	%rcx, (%rdi)
1412f1c07: 48 8b 4f 10                 	movq	0x10(%rdi), %rcx
1412f1c0b: 48 8b 57 18                 	movq	0x18(%rdi), %rdx
1412f1c0f: f2 0f 10 01                 	movsd	(%rcx), %xmm0
1412f1c13: f2 0f 10 30                 	movsd	(%rax), %xmm6
1412f1c17: f2 0f 10 48 08              	movsd	0x8(%rax), %xmm1
1412f1c1c: 0f 28 15 3d 34 7b 01        	movaps	0x17b343d(%rip), %xmm2  # 0x142aa5060
1412f1c23: 0f 59 d6                    	mulps	%xmm6, %xmm2
1412f1c26: 0f 58 d0                    	addps	%xmm0, %xmm2
1412f1c29: 0f 59 0d 40 34 7b 01        	mulps	0x17b3440(%rip), %xmm1  # 0x142aa5070
1412f1c30: 0f 58 ca                    	addps	%xmm2, %xmm1
1412f1c33: 0f 13 4c 24 28              	movlps	%xmm1, 0x28(%rsp)
1412f1c38: f3 0f 10 12                 	movss	(%rdx), %xmm2
1412f1c3c: f3 0f 58 15 40 3d 63 01     	addss	0x1633d40(%rip), %xmm2  # 0x142925984
1412f1c44: 48 8d 4c 24 34              	leaq	0x34(%rsp), %rcx
1412f1c49: 48 8d 54 24 28              	leaq	0x28(%rsp), %rdx
1412f1c4e: e8 8d b1 9a ff              	callq	0x140c9cde0 <.text+0xc9bde0>
1412f1c53: f3 0f 16 d6                 	movshdup	%xmm6, %xmm2            # xmm2 = xmm6[1,1,3,3]
1412f1c57: 48 8d 5c 24 40              	leaq	0x40(%rsp), %rbx
1412f1c5c: 48 89 d9                    	movq	%rbx, %rcx
1412f1c5f: 0f 28 ce                    	movaps	%xmm6, %xmm1
1412f1c62: e8 69 f3 99 ff              	callq	0x140c90fd0 <.text+0xc8ffd0>
1412f1c67: 8b 44 24 3c                 	movl	0x3c(%rsp), %eax
1412f1c6b: 89 44 24 58                 	movl	%eax, 0x58(%rsp)
1412f1c6f: 48 8b 44 24 34              	movq	0x34(%rsp), %rax
1412f1c74: 48 89 44 24 50              	movq	%rax, 0x50(%rsp)
1412f1c79: 48 8b 05 70 3f 7b 01        	movq	0x17b3f70(%rip), %rax   # 0x142aa5bf0
1412f1c80: 48 89 44 24 5c              	movq	%rax, 0x5c(%rsp)
1412f1c85: 8b 05 6d 3f 7b 01           	movl	0x17b3f6d(%rip), %eax   # 0x142aa5bf8
1412f1c8b: 89 44 24 64                 	movl	%eax, 0x64(%rsp)
1412f1c8f: 48 89 5c 24 20              	movq	%rbx, 0x20(%rsp)
1412f1c94: 48 8d 4c 24 78              	leaq	0x78(%rsp), %rcx
1412f1c99: f3 0f 10 0d 8f b3 6d 01     	movss	0x16db38f(%rip), %xmm1  # 0x1429cd030
1412f1ca1: f3 0f 10 15 c7 3b 63 01     	movss	0x1633bc7(%rip), %xmm2  # 0x142925870
1412f1ca9: f3 0f 10 1d cf 33 7b 01     	movss	0x17b33cf(%rip), %xmm3  # 0x142aa5080
1412f1cb1: e8 2a 84 7e ff              	callq	0x140ada0e0 <.text+0xad90e0>
1412f1cb6: 48 8b 44 24 78              	movq	0x78(%rsp), %rax
1412f1cbb: 48 89 c1                    	movq	%rax, %rcx
1412f1cbe: 48 f7 d9                    	negq	%rcx
1412f1cc1: 70 39                       	jo	0x1412f1cfc <.text+0x12f0cfc>
1412f1cc3: 48 8b 8c 24 80 00 00 00     	movq	0x80(%rsp), %rcx
1412f1ccb: 48 8b 94 24 88 00 00 00     	movq	0x88(%rsp), %rdx
1412f1cd3: 48 8d 14 d2                 	leaq	(%rdx,%rdx,8), %rdx
1412f1cd7: 48 8d 14 91                 	leaq	(%rcx,%rdx,4), %rdx
1412f1cdb: 48 89 4f 20                 	movq	%rcx, 0x20(%rdi)
1412f1cdf: 48 89 4f 28                 	movq	%rcx, 0x28(%rdi)
1412f1ce3: 48 89 47 30                 	movq	%rax, 0x30(%rdi)
1412f1ce7: 48 89 57 38                 	movq	%rdx, 0x38(%rdi)
1412f1ceb: 48 89 c8                    	movq	%rcx, %rax
1412f1cee: 48 39 d0                    	cmpq	%rdx, %rax
1412f1cf1: 0f 84 cf fe ff ff           	je	0x1412f1bc6 <.text+0x12f0bc6>
1412f1cf7: e9 a2 fe ff ff              	jmp	0x1412f1b9e <.text+0x12f0b9e>
1412f1cfc: 48 8b 4f 40                 	movq	0x40(%rdi), %rcx
1412f1d00: 48 85 c9                    	testq	%rcx, %rcx
1412f1d03: 74 31                       	je	0x1412f1d36 <.text+0x12f0d36>
1412f1d05: 48 8b 47 48                 	movq	0x48(%rdi), %rax
1412f1d09: 48 3b 47 58                 	cmpq	0x58(%rdi), %rax
1412f1d0d: 74 2f                       	je	0x1412f1d3e <.text+0x12f0d3e>
1412f1d0f: 48 8d 48 24                 	leaq	0x24(%rax), %rcx
1412f1d13: 48 89 4f 48                 	movq	%rcx, 0x48(%rdi)
1412f1d17: 8b 48 20                    	movl	0x20(%rax), %ecx
1412f1d1a: 89 4c 24 60                 	movl	%ecx, 0x60(%rsp)
1412f1d1e: 0f 10 00                    	movups	(%rax), %xmm0
1412f1d21: 0f 10 48 10                 	movups	0x10(%rax), %xmm1
1412f1d25: 0f 29 4c 24 50              	movaps	%xmm1, 0x50(%rsp)
1412f1d2a: 0f 29 44 24 40              	movaps	%xmm0, 0x40(%rsp)
1412f1d2f: b8 01 00 00 00              	movl	$0x1, %eax
1412f1d34: eb 2e                       	jmp	0x1412f1d64 <.text+0x12f0d64>
1412f1d36: c7 06 00 00 00 00           	movl	$0x0, (%rsi)
1412f1d3c: eb 41                       	jmp	0x1412f1d7f <.text+0x12f0d7f>
1412f1d3e: 48 8b 47 50                 	movq	0x50(%rdi), %rax
1412f1d42: 48 85 c0                    	testq	%rax, %rax
1412f1d45: 74 13                       	je	0x1412f1d5a <.text+0x12f0d5a>
1412f1d47: 48 c1 e0 02                 	shlq	$0x2, %rax
1412f1d4b: 48 8d 14 c0                 	leaq	(%rax,%rax,8), %rdx
1412f1d4f: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1412f1d55: e8 c6 1e 32 ff              	callq	0x140613c20 <.text+0x612c20>
1412f1d5a: 48 c7 47 40 00 00 00 00     	movq	$0x0, 0x40(%rdi)
1412f1d62: 31 c0                       	xorl	%eax, %eax
1412f1d64: 89 06                       	movl	%eax, (%rsi)
1412f1d66: 0f 28 44 24 40              	movaps	0x40(%rsp), %xmm0
1412f1d6b: 0f 28 4c 24 50              	movaps	0x50(%rsp), %xmm1
1412f1d70: 0f 11 46 04                 	movups	%xmm0, 0x4(%rsi)
1412f1d74: 0f 11 4e 14                 	movups	%xmm1, 0x14(%rsi)
1412f1d78: 8b 44 24 60                 	movl	0x60(%rsp), %eax
1412f1d7c: 89 46 24                    	movl	%eax, 0x24(%rsi)
1412f1d7f: 0f 28 b4 24 90 00 00 00     	movaps	0x90(%rsp), %xmm6
1412f1d87: 48 81 c4 a0 00 00 00        	addq	$0xa0, %rsp
1412f1d8e: 5b                          	popq	%rbx
1412f1d8f: 5f                          	popq	%rdi
1412f1d90: 5e                          	popq	%rsi
1412f1d91: c3                          	retq
1412f1d92: cc                          	int3
1412f1d93: cc                          	int3
1412f1d94: cc                          	int3
1412f1d95: cc                          	int3
1412f1d96: cc                          	int3
1412f1d97: cc                          	int3
1412f1d98: cc                          	int3
1412f1d99: cc                          	int3
1412f1d9a: cc                          	int3
1412f1d9b: cc                          	int3
1412f1d9c: cc                          	int3
1412f1d9d: cc                          	int3
1412f1d9e: cc                          	int3
1412f1d9f: cc                          	int3
1412f1da0: 41 57                       	pushq	%r15
1412f1da2: 41 56                       	pushq	%r14
1412f1da4: 41 55                       	pushq	%r13
1412f1da6: 41 54                       	pushq	%r12
1412f1da8: 56                          	pushq	%rsi
1412f1da9: 57                          	pushq	%rdi
1412f1daa: 55                          	pushq	%rbp
1412f1dab: 53                          	pushq	%rbx
1412f1dac: 48 83 ec 38                 	subq	$0x38, %rsp
1412f1db0: 48 89 d7                    	movq	%rdx, %rdi
1412f1db3: 48 89 ce                    	movq	%rcx, %rsi
1412f1db6: 48 8d 4c 24 20              	leaq	0x20(%rsp), %rcx
1412f1dbb: e8 e0 15 00 00              	callq	0x1412f33a0 <.text+0x12f23a0>
1412f1dc0: f6 44 24 20 01              	testb	$0x1, 0x20(%rsp)
1412f1dc5: 74 22                       	je	0x1412f1de9 <.text+0x12f0de9>
1412f1dc7: 48 8b 44 24 30              	movq	0x30(%rsp), %rax
1412f1dcc: 48 89 46 10                 	movq	%rax, 0x10(%rsi)
1412f1dd0: 0f 10 44 24 20              	movups	0x20(%rsp), %xmm0
1412f1dd5: 0f 11 06                    	movups	%xmm0, (%rsi)
1412f1dd8: 48 83 c4 38                 	addq	$0x38, %rsp
1412f1ddc: 5b                          	popq	%rbx
1412f1ddd: 5d                          	popq	%rbp
1412f1dde: 5f                          	popq	%rdi
1412f1ddf: 5e                          	popq	%rsi
1412f1de0: 41 5c                       	popq	%r12
1412f1de2: 41 5d                       	popq	%r13
1412f1de4: 41 5e                       	popq	%r14
1412f1de6: 41 5f                       	popq	%r15
1412f1de8: c3                          	retq
1412f1de9: 48 8d 5c 24 20              	leaq	0x20(%rsp), %rbx
1412f1dee: eb 3d                       	jmp	0x1412f1e2d <.text+0x12f0e2d>
1412f1df0: 48 c7 07 01 00 00 00        	movq	$0x1, (%rdi)
1412f1df7: 4c 89 6f 08                 	movq	%r13, 0x8(%rdi)
1412f1dfb: 48 89 6f 10                 	movq	%rbp, 0x10(%rdi)
1412f1dff: 4c 89 77 18                 	movq	%r14, 0x18(%rdi)
1412f1e03: 4c 89 7f 20                 	movq	%r15, 0x20(%rdi)
1412f1e07: 4c 89 67 28                 	movq	%r12, 0x28(%rdi)
1412f1e0b: 48 c7 47 30 00 00 00 00     	movq	$0x0, 0x30(%rdi)
1412f1e13: 48 c7 47 68 00 00 00 00     	movq	$0x0, 0x68(%rdi)
1412f1e1b: 48 89 d9                    	movq	%rbx, %rcx
1412f1e1e: 48 89 fa                    	movq	%rdi, %rdx
1412f1e21: e8 7a 15 00 00              	callq	0x1412f33a0 <.text+0x12f23a0>
1412f1e26: f6 44 24 20 01              	testb	$0x1, 0x20(%rsp)
1412f1e2b: 75 9a                       	jne	0x1412f1dc7 <.text+0x12f0dc7>
1412f1e2d: 44 0f b6 87 c8 01 00 00     	movzbl	0x1c8(%rdi), %r8d
1412f1e35: 41 80 f8 02                 	cmpb	$0x2, %r8b
1412f1e39: 0f 84 af 04 00 00           	je	0x1412f22ee <.text+0x12f12ee>
1412f1e3f: 48 8b 8f 50 01 00 00        	movq	0x150(%rdi), %rcx
1412f1e46: 48 8b 87 60 01 00 00        	movq	0x160(%rdi), %rax
1412f1e4d: 48 8b 97 c0 01 00 00        	movq	0x1c0(%rdi), %rdx
1412f1e54: 4c 8b 97 b8 01 00 00        	movq	0x1b8(%rdi), %r10
1412f1e5b: 4c 8b 8f 70 01 00 00        	movq	0x170(%rdi), %r9
1412f1e62: 41 f6 c0 01                 	testb	$0x1, %r8b
1412f1e66: 0f 84 a4 00 00 00           	je	0x1412f1f10 <.text+0x12f0f10>
1412f1e6c: 4c 39 d2                    	cmpq	%r10, %rdx
1412f1e6f: 0f 85 5b 01 00 00           	jne	0x1412f1fd0 <.text+0x12f0fd0>
1412f1e75: 4c 8b 87 68 01 00 00        	movq	0x168(%rdi), %r8
1412f1e7c: 0f 1f 40 00                 	nopl	(%rax)
1412f1e80: 4d 39 c8                    	cmpq	%r9, %r8
1412f1e83: 0f 84 65 04 00 00           	je	0x1412f22ee <.text+0x12f12ee>
1412f1e89: 4d 8d 70 04                 	leaq	0x4(%r8), %r14
1412f1e8d: 4c 89 b7 68 01 00 00        	movq	%r14, 0x168(%rdi)
1412f1e94: 41 8b 10                    	movl	(%r8), %edx
1412f1e97: 4c 8b 51 08                 	movq	0x8(%rcx), %r10
1412f1e9b: 4c 8d 1c d2                 	leaq	(%rdx,%rdx,8), %r11
1412f1e9f: 4b 8b 54 da 10              	movq	0x10(%r10,%r11,8), %rdx
1412f1ea4: 4d 89 f0                    	movq	%r14, %r8
1412f1ea7: 48 85 d2                    	testq	%rdx, %rdx
1412f1eaa: 74 d4                       	je	0x1412f1e80 <.text+0x12f0e80>
1412f1eac: 4f 8d 04 da                 	leaq	(%r10,%r11,8), %r8
1412f1eb0: 48 8b 88 10 01 00 00        	movq	0x110(%rax), %rcx
1412f1eb7: 4d 8b 50 40                 	movq	0x40(%r8), %r10
1412f1ebb: 49 39 ca                    	cmpq	%rcx, %r10
1412f1ebe: 0f 86 64 01 00 00           	jbe	0x1412f2028 <.text+0x12f1028>
1412f1ec4: 4d 8b 48 38                 	movq	0x38(%r8), %r9
1412f1ec8: 49 8b 0c c9                 	movq	(%r9,%rcx,8), %rcx
1412f1ecc: 48 85 c9                    	testq	%rcx, %rcx
1412f1ecf: 0f 84 53 01 00 00           	je	0x1412f2028 <.text+0x12f1028>
1412f1ed5: 4d 8b 48 18                 	movq	0x18(%r8), %r9
1412f1ed9: 48 f7 d1                    	notq	%rcx
1412f1edc: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
1412f1ee0: 48 c1 e1 04                 	shlq	$0x4, %rcx
1412f1ee4: 49 8b 4c 09 10              	movq	0x10(%r9,%rcx), %rcx
1412f1ee9: 48 89 8f 98 01 00 00        	movq	%rcx, 0x198(%rdi)
1412f1ef0: 4c 8b 88 18 01 00 00        	movq	0x118(%rax), %r9
1412f1ef7: 4d 39 ca                    	cmpq	%r9, %r10
1412f1efa: 0f 87 41 01 00 00           	ja	0x1412f2041 <.text+0x12f1041>
1412f1f00: e9 d1 01 00 00              	jmp	0x1412f20d6 <.text+0x12f10d6>
1412f1f05: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
1412f1f10: 4c 39 d2                    	cmpq	%r10, %rdx
1412f1f13: 0f 85 e7 00 00 00           	jne	0x1412f2000 <.text+0x12f1000>
1412f1f19: 4c 8b 97 58 01 00 00        	movq	0x158(%rdi), %r10
1412f1f20: 4c 8b 9f 68 01 00 00        	movq	0x168(%rdi), %r11
1412f1f27: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1412f1f30: 4d 39 cb                    	cmpq	%r9, %r11
1412f1f33: 0f 84 b5 03 00 00           	je	0x1412f22ee <.text+0x12f12ee>
1412f1f39: 4d 8d 7b 04                 	leaq	0x4(%r11), %r15
1412f1f3d: 4c 89 bf 68 01 00 00        	movq	%r15, 0x168(%rdi)
1412f1f44: 45 8b 03                    	movl	(%r11), %r8d
1412f1f47: 49 8b 52 08                 	movq	0x8(%r10), %rdx
1412f1f4b: 4f 8d 34 80                 	leaq	(%r8,%r8,4), %r14
1412f1f4f: 49 c1 e6 05                 	shlq	$0x5, %r14
1412f1f53: 4e 8b 44 32 58              	movq	0x58(%rdx,%r14), %r8
1412f1f58: 4d 89 fb                    	movq	%r15, %r11
1412f1f5b: 4d 85 c0                    	testq	%r8, %r8
1412f1f5e: 74 d0                       	je	0x1412f1f30 <.text+0x12f0f30>
1412f1f60: 4c 01 f2                    	addq	%r14, %rdx
1412f1f63: 44 8b 8a 94 00 00 00        	movl	0x94(%rdx), %r9d
1412f1f6a: 4c 8b 59 08                 	movq	0x8(%rcx), %r11
1412f1f6e: 4f 8d 0c c9                 	leaq	(%r9,%r9,8), %r9
1412f1f72: 4f 8d 14 cb                 	leaq	(%r11,%r9,8), %r10
1412f1f76: 48 8b 88 10 01 00 00        	movq	0x110(%rax), %rcx
1412f1f7d: 4f 8b 5c cb 40              	movq	0x40(%r11,%r9,8), %r11
1412f1f82: 49 39 cb                    	cmpq	%rcx, %r11
1412f1f85: 0f 86 f4 00 00 00           	jbe	0x1412f207f <.text+0x12f107f>
1412f1f8b: 4d 8b 4a 38                 	movq	0x38(%r10), %r9
1412f1f8f: 49 8b 0c c9                 	movq	(%r9,%rcx,8), %rcx
1412f1f93: 48 85 c9                    	testq	%rcx, %rcx
1412f1f96: 0f 84 e3 00 00 00           	je	0x1412f207f <.text+0x12f107f>
1412f1f9c: 4d 8b 4a 18                 	movq	0x18(%r10), %r9
1412f1fa0: 48 f7 d1                    	notq	%rcx
1412f1fa3: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
1412f1fa7: 48 c1 e1 04                 	shlq	$0x4, %rcx
1412f1fab: 49 8b 4c 09 10              	movq	0x10(%r9,%rcx), %rcx
1412f1fb0: 48 89 8f 98 01 00 00        	movq	%rcx, 0x198(%rdi)
1412f1fb7: 4c 8b 88 18 01 00 00        	movq	0x118(%rax), %r9
1412f1fbe: 4d 39 cb                    	cmpq	%r9, %r11
1412f1fc1: 0f 87 d1 00 00 00           	ja	0x1412f2098 <.text+0x12f1098>
1412f1fc7: e9 5e 01 00 00              	jmp	0x1412f212a <.text+0x12f112a>
1412f1fcc: 0f 1f 40 00                 	nopl	(%rax)
1412f1fd0: 48 8b 8f 98 01 00 00        	movq	0x198(%rdi), %rcx
1412f1fd7: 4c 8b 8f a0 01 00 00        	movq	0x1a0(%rdi), %r9
1412f1fde: 4c 8b 9f a8 01 00 00        	movq	0x1a8(%rdi), %r11
1412f1fe5: 48 8b 87 b0 01 00 00        	movq	0x1b0(%rdi), %rax
1412f1fec: e9 1f 02 00 00              	jmp	0x1412f2210 <.text+0x12f1210>
1412f1ff1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
1412f2000: 4c 8b 97 88 01 00 00        	movq	0x188(%rdi), %r10
1412f2007: 48 8b 8f 98 01 00 00        	movq	0x198(%rdi), %rcx
1412f200e: 4c 8b 8f a0 01 00 00        	movq	0x1a0(%rdi), %r9
1412f2015: 4c 8b af a8 01 00 00        	movq	0x1a8(%rdi), %r13
1412f201c: 48 8b 87 b0 01 00 00        	movq	0x1b0(%rdi), %rax
1412f2023: e9 2b 02 00 00              	jmp	0x1412f2253 <.text+0x12f1253>
1412f2028: 31 c9                       	xorl	%ecx, %ecx
1412f202a: 48 89 8f 98 01 00 00        	movq	%rcx, 0x198(%rdi)
1412f2031: 4c 8b 88 18 01 00 00        	movq	0x118(%rax), %r9
1412f2038: 4d 39 ca                    	cmpq	%r9, %r10
1412f203b: 0f 86 95 00 00 00           	jbe	0x1412f20d6 <.text+0x12f10d6>
1412f2041: 4d 8b 58 38                 	movq	0x38(%r8), %r11
1412f2045: 4f 8b 0c cb                 	movq	(%r11,%r9,8), %r9
1412f2049: 4d 85 c9                    	testq	%r9, %r9
1412f204c: 0f 84 84 00 00 00           	je	0x1412f20d6 <.text+0x12f10d6>
1412f2052: 4d 8b 58 18                 	movq	0x18(%r8), %r11
1412f2056: 49 f7 d1                    	notq	%r9
1412f2059: 4f 8d 0c 49                 	leaq	(%r9,%r9,2), %r9
1412f205d: 49 c1 e1 04                 	shlq	$0x4, %r9
1412f2061: 4f 8b 4c 0b 10              	movq	0x10(%r11,%r9), %r9
1412f2066: 4c 89 8f a0 01 00 00        	movq	%r9, 0x1a0(%rdi)
1412f206d: 4c 8b 98 20 01 00 00        	movq	0x120(%rax), %r11
1412f2074: 4d 39 da                    	cmpq	%r11, %r10
1412f2077: 0f 86 fd 00 00 00           	jbe	0x1412f217a <.text+0x12f117a>
1412f207d: eb 71                       	jmp	0x1412f20f0 <.text+0x12f10f0>
1412f207f: 31 c9                       	xorl	%ecx, %ecx
1412f2081: 48 89 8f 98 01 00 00        	movq	%rcx, 0x198(%rdi)
1412f2088: 4c 8b 88 18 01 00 00        	movq	0x118(%rax), %r9
1412f208f: 4d 39 cb                    	cmpq	%r9, %r11
1412f2092: 0f 86 92 00 00 00           	jbe	0x1412f212a <.text+0x12f112a>
1412f2098: 4d 8b 72 38                 	movq	0x38(%r10), %r14
1412f209c: 4f 8b 0c ce                 	movq	(%r14,%r9,8), %r9
1412f20a0: 4d 85 c9                    	testq	%r9, %r9
1412f20a3: 0f 84 81 00 00 00           	je	0x1412f212a <.text+0x12f112a>
1412f20a9: 4d 8b 72 18                 	movq	0x18(%r10), %r14
1412f20ad: 49 f7 d1                    	notq	%r9
1412f20b0: 4f 8d 0c 49                 	leaq	(%r9,%r9,2), %r9
1412f20b4: 49 c1 e1 04                 	shlq	$0x4, %r9
1412f20b8: 4f 8b 4c 0e 10              	movq	0x10(%r14,%r9), %r9
1412f20bd: 4c 89 8f a0 01 00 00        	movq	%r9, 0x1a0(%rdi)
1412f20c4: 4c 8b b0 20 01 00 00        	movq	0x120(%rax), %r14
1412f20cb: 4d 39 f3                    	cmpq	%r14, %r11
1412f20ce: 0f 86 df 00 00 00           	jbe	0x1412f21b3 <.text+0x12f11b3>
1412f20d4: eb 6a                       	jmp	0x1412f2140 <.text+0x12f1140>
1412f20d6: 45 31 c9                    	xorl	%r9d, %r9d
1412f20d9: 4c 89 8f a0 01 00 00        	movq	%r9, 0x1a0(%rdi)
1412f20e0: 4c 8b 98 20 01 00 00        	movq	0x120(%rax), %r11
1412f20e7: 4d 39 da                    	cmpq	%r11, %r10
1412f20ea: 0f 86 8a 00 00 00           	jbe	0x1412f217a <.text+0x12f117a>
1412f20f0: 4d 8b 70 38                 	movq	0x38(%r8), %r14
1412f20f4: 4f 8b 1c de                 	movq	(%r14,%r11,8), %r11
1412f20f8: 4d 85 db                    	testq	%r11, %r11
1412f20fb: 74 7d                       	je	0x1412f217a <.text+0x12f117a>
1412f20fd: 4d 8b 70 18                 	movq	0x18(%r8), %r14
1412f2101: 49 f7 d3                    	notq	%r11
1412f2104: 4f 8d 1c 5b                 	leaq	(%r11,%r11,2), %r11
1412f2108: 49 c1 e3 04                 	shlq	$0x4, %r11
1412f210c: 4f 8b 5c 1e 10              	movq	0x10(%r14,%r11), %r11
1412f2111: 4c 89 9f a8 01 00 00        	movq	%r11, 0x1a8(%rdi)
1412f2118: 48 8b 80 28 01 00 00        	movq	0x128(%rax), %rax
1412f211f: 49 39 c2                    	cmpq	%rax, %r10
1412f2122: 0f 86 c4 00 00 00           	jbe	0x1412f21ec <.text+0x12f11ec>
1412f2128: eb 66                       	jmp	0x1412f2190 <.text+0x12f1190>
1412f212a: 45 31 c9                    	xorl	%r9d, %r9d
1412f212d: 4c 89 8f a0 01 00 00        	movq	%r9, 0x1a0(%rdi)
1412f2134: 4c 8b b0 20 01 00 00        	movq	0x120(%rax), %r14
1412f213b: 4d 39 f3                    	cmpq	%r14, %r11
1412f213e: 76 73                       	jbe	0x1412f21b3 <.text+0x12f11b3>
1412f2140: 4d 8b 7a 38                 	movq	0x38(%r10), %r15
1412f2144: 4f 8b 34 f7                 	movq	(%r15,%r14,8), %r14
1412f2148: 4d 85 f6                    	testq	%r14, %r14
1412f214b: 74 66                       	je	0x1412f21b3 <.text+0x12f11b3>
1412f214d: 4d 8b 7a 18                 	movq	0x18(%r10), %r15
1412f2151: 49 f7 d6                    	notq	%r14
1412f2154: 4f 8d 34 76                 	leaq	(%r14,%r14,2), %r14
1412f2158: 49 c1 e6 04                 	shlq	$0x4, %r14
1412f215c: 4f 8b 6c 37 10              	movq	0x10(%r15,%r14), %r13
1412f2161: 4c 89 af a8 01 00 00        	movq	%r13, 0x1a8(%rdi)
1412f2168: 48 8b 80 28 01 00 00        	movq	0x128(%rax), %rax
1412f216f: 49 39 c3                    	cmpq	%rax, %r11
1412f2172: 0f 86 b7 00 00 00           	jbe	0x1412f222f <.text+0x12f122f>
1412f2178: eb 4f                       	jmp	0x1412f21c9 <.text+0x12f11c9>
1412f217a: 45 31 db                    	xorl	%r11d, %r11d
1412f217d: 4c 89 9f a8 01 00 00        	movq	%r11, 0x1a8(%rdi)
1412f2184: 48 8b 80 28 01 00 00        	movq	0x128(%rax), %rax
1412f218b: 49 39 c2                    	cmpq	%rax, %r10
1412f218e: 76 5c                       	jbe	0x1412f21ec <.text+0x12f11ec>
1412f2190: 4d 8b 50 38                 	movq	0x38(%r8), %r10
1412f2194: 49 8b 04 c2                 	movq	(%r10,%rax,8), %rax
1412f2198: 48 85 c0                    	testq	%rax, %rax
1412f219b: 74 4f                       	je	0x1412f21ec <.text+0x12f11ec>
1412f219d: 4d 8b 50 18                 	movq	0x18(%r8), %r10
1412f21a1: 48 f7 d0                    	notq	%rax
1412f21a4: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
1412f21a8: 48 c1 e0 04                 	shlq	$0x4, %rax
1412f21ac: 49 8b 44 02 10              	movq	0x10(%r10,%rax), %rax
1412f21b1: eb 3b                       	jmp	0x1412f21ee <.text+0x12f11ee>
1412f21b3: 45 31 ed                    	xorl	%r13d, %r13d
1412f21b6: 4c 89 af a8 01 00 00        	movq	%r13, 0x1a8(%rdi)
1412f21bd: 48 8b 80 28 01 00 00        	movq	0x128(%rax), %rax
1412f21c4: 49 39 c3                    	cmpq	%rax, %r11
1412f21c7: 76 66                       	jbe	0x1412f222f <.text+0x12f122f>
1412f21c9: 4d 8b 5a 38                 	movq	0x38(%r10), %r11
1412f21cd: 49 8b 04 c3                 	movq	(%r11,%rax,8), %rax
1412f21d1: 48 85 c0                    	testq	%rax, %rax
1412f21d4: 74 59                       	je	0x1412f222f <.text+0x12f122f>
1412f21d6: 4d 8b 52 18                 	movq	0x18(%r10), %r10
1412f21da: 48 f7 d0                    	notq	%rax
1412f21dd: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
1412f21e1: 48 c1 e0 04                 	shlq	$0x4, %rax
1412f21e5: 49 8b 44 02 10              	movq	0x10(%r10,%rax), %rax
1412f21ea: eb 45                       	jmp	0x1412f2231 <.text+0x12f1231>
1412f21ec: 31 c0                       	xorl	%eax, %eax
1412f21ee: 48 89 87 b0 01 00 00        	movq	%rax, 0x1b0(%rdi)
1412f21f5: 4d 8b 40 08                 	movq	0x8(%r8), %r8
1412f21f9: 4c 89 87 78 01 00 00        	movq	%r8, 0x178(%rdi)
