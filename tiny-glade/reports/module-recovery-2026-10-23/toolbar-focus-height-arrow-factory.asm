
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a66a30: 55                          	pushq	%rbp
140a66a31: 41 57                       	pushq	%r15
140a66a33: 41 56                       	pushq	%r14
140a66a35: 56                          	pushq	%rsi
140a66a36: 57                          	pushq	%rdi
140a66a37: 53                          	pushq	%rbx
140a66a38: 48 81 ec 08 01 00 00        	subq	$0x108, %rsp            # imm = 0x108
140a66a3f: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140a66a47: 48 c7 85 80 00 00 00 fe ff ff ff    	movq	$-0x2, 0x80(%rbp)
140a66a52: 4c 89 cf                    	movq	%r9, %rdi
140a66a55: 4c 89 c3                    	movq	%r8, %rbx
140a66a58: 49 89 d6                    	movq	%rdx, %r14
140a66a5b: 48 89 ce                    	movq	%rcx, %rsi
140a66a5e: 0f b6 05 a4 e1 bc 02        	movzbl	0x2bce1a4(%rip), %eax   # 0x143634c09
140a66a65: b9 08 00 00 00              	movl	$0x8, %ecx
140a66a6a: ba 08 00 00 00              	movl	$0x8, %edx
140a66a6f: e8 9c d1 ba ff              	callq	0x140613c10 <.text+0x612c10>
140a66a74: 48 85 c0                    	testq	%rax, %rax
140a66a77: 0f 84 63 01 00 00           	je	0x140a66be0 <.text+0xa65be0>
140a66a7d: 49 89 c7                    	movq	%rax, %r15
140a66a80: 0f b6 05 82 e1 bc 02        	movzbl	0x2bce182(%rip), %eax   # 0x143634c09
140a66a87: b9 1a 00 00 00              	movl	$0x1a, %ecx
140a66a8c: ba 01 00 00 00              	movl	$0x1, %edx
140a66a91: e8 7a d1 ba ff              	callq	0x140613c10 <.text+0x612c10>
140a66a96: 48 85 c0                    	testq	%rax, %rax
140a66a99: 4c 89 7d 78                 	movq	%r15, 0x78(%rbp)
140a66a9d: 0f 84 4c 01 00 00           	je	0x140a66bef <.text+0xa65bef>
140a66aa3: 0f 10 05 04 de 06 02        	movups	0x206de04(%rip), %xmm0  # 0x142ad48ae
140a66aaa: 0f 11 40 0a                 	movups	%xmm0, 0xa(%rax)
140a66aae: 0f 10 05 ef dd 06 02        	movups	0x206ddef(%rip), %xmm0  # 0x142ad48a4
140a66ab5: 0f 11 00                    	movups	%xmm0, (%rax)
140a66ab8: 48 c7 45 50 1a 00 00 00     	movq	$0x1a, 0x50(%rbp)
140a66ac0: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
140a66ac4: 48 c7 45 60 1a 00 00 00     	movq	$0x1a, 0x60(%rbp)
140a66acc: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
140a66ad6: 48 89 45 48                 	movq	%rax, 0x48(%rbp)
140a66ada: 0f b6 05 28 e1 bc 02        	movzbl	0x2bce128(%rip), %eax   # 0x143634c09
140a66ae1: b9 13 00 00 00              	movl	$0x13, %ecx
140a66ae6: ba 01 00 00 00              	movl	$0x1, %edx
140a66aeb: e8 20 d1 ba ff              	callq	0x140613c10 <.text+0x612c10>
140a66af0: 48 85 c0                    	testq	%rax, %rax
140a66af3: 0f 84 0e 01 00 00           	je	0x140a66c07 <.text+0xa65c07>
140a66af9: 48 8b 8d e0 00 00 00        	movq	0xe0(%rbp), %rcx
140a66b00: 8b 15 95 29 03 02           	movl	0x2032995(%rip), %edx   # 0x142a9949b
140a66b06: 89 50 0f                    	movl	%edx, 0xf(%rax)
140a66b09: 0f 10 05 7c 29 03 02        	movups	0x203297c(%rip), %xmm0  # 0x142a9948c
140a66b10: 0f 11 00                    	movups	%xmm0, (%rax)
140a66b13: 0f 10 45 48                 	movups	0x48(%rbp), %xmm0
140a66b17: 0f 10 4d 58                 	movups	0x58(%rbp), %xmm1
140a66b1b: 0f 10 55 68                 	movups	0x68(%rbp), %xmm2
140a66b1f: 0f 29 45 c0                 	movaps	%xmm0, -0x40(%rbp)
140a66b23: 0f 29 4d d0                 	movaps	%xmm1, -0x30(%rbp)
140a66b27: 0f 29 55 e0                 	movaps	%xmm2, -0x20(%rbp)
140a66b2b: 41 0f 28 06                 	movaps	(%r14), %xmm0
140a66b2f: 41 8b 56 18                 	movl	0x18(%r14), %edx
140a66b33: 89 55 50                    	movl	%edx, 0x50(%rbp)
140a66b36: 49 8b 56 10                 	movq	0x10(%r14), %rdx
140a66b3a: 48 89 55 48                 	movq	%rdx, 0x48(%rbp)
140a66b3e: 0f 29 45 b0                 	movaps	%xmm0, -0x50(%rbp)
140a66b42: 66 c7 45 f0 00 01           	movw	$0x100, -0x10(%rbp)     # imm = 0x100
140a66b48: 48 ba 00 00 80 3f 01 00 00 00       	movabsq	$0x13f800000, %rdx # imm = 0x13F800000
140a66b52: 48 89 55 f4                 	movq	%rdx, -0xc(%rbp)
140a66b56: c7 45 00 00 00 00 00        	movl	$0x0, (%rbp)
140a66b5d: c7 45 08 00 00 00 00        	movl	$0x0, 0x8(%rbp)
140a66b64: 48 c7 45 10 13 00 00 00     	movq	$0x13, 0x10(%rbp)
140a66b6c: 48 89 45 18                 	movq	%rax, 0x18(%rbp)
140a66b70: 48 c7 45 20 13 00 00 00     	movq	$0x13, 0x20(%rbp)
140a66b78: c7 45 28 00 00 00 00        	movl	$0x0, 0x28(%rbp)
140a66b7f: 48 8b 45 48                 	movq	0x48(%rbp), %rax
140a66b83: 48 89 45 2c                 	movq	%rax, 0x2c(%rbp)
140a66b87: 8b 45 50                    	movl	0x50(%rbp), %eax
140a66b8a: 89 45 34                    	movl	%eax, 0x34(%rbp)
140a66b8d: 66 c7 45 38 00 00           	movw	$0x0, 0x38(%rbp)
140a66b93: c6 45 3a 00                 	movb	$0x0, 0x3a(%rbp)
140a66b97: 48 89 4c 24 20              	movq	%rcx, 0x20(%rsp)
140a66b9c: 48 8d 4d 48                 	leaq	0x48(%rbp), %rcx
140a66ba0: 48 8d 55 b0                 	leaq	-0x50(%rbp), %rdx
140a66ba4: 49 89 d8                    	movq	%rbx, %r8
140a66ba7: 49 89 f9                    	movq	%rdi, %r9
140a66baa: e8 a1 71 e3 ff              	callq	0x14089dd50 <.text+0x89cd50>
140a66baf: 48 8b 45 68                 	movq	0x68(%rbp), %rax
140a66bb3: 48 8b 4d 78                 	movq	0x78(%rbp), %rcx
140a66bb7: 48 89 01                    	movq	%rax, (%rcx)
140a66bba: 48 c7 06 01 00 00 00        	movq	$0x1, (%rsi)
140a66bc1: 48 89 4e 08                 	movq	%rcx, 0x8(%rsi)
140a66bc5: 48 c7 46 10 01 00 00 00     	movq	$0x1, 0x10(%rsi)
140a66bcd: 48 89 f0                    	movq	%rsi, %rax
140a66bd0: 48 81 c4 08 01 00 00        	addq	$0x108, %rsp            # imm = 0x108
140a66bd7: 5b                          	popq	%rbx
140a66bd8: 5f                          	popq	%rdi
140a66bd9: 5e                          	popq	%rsi
140a66bda: 41 5e                       	popq	%r14
140a66bdc: 41 5f                       	popq	%r15
140a66bde: 5d                          	popq	%rbp
140a66bdf: c3                          	retq
140a66be0: b9 08 00 00 00              	movl	$0x8, %ecx
140a66be5: ba 08 00 00 00              	movl	$0x8, %edx
140a66bea: e8 11 24 e7 01              	callq	0x1428d9000 <.text+0x28d8000>
140a66bef: 4c 8d 05 82 cf 06 02        	leaq	0x206cf82(%rip), %r8    # 0x142ad3b78
140a66bf6: b9 01 00 00 00              	movl	$0x1, %ecx
140a66bfb: ba 1a 00 00 00              	movl	$0x1a, %edx
140a66c00: e8 de 23 e7 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140a66c05: eb 16                       	jmp	0x140a66c1d <.text+0xa65c1d>
140a66c07: 4c 8d 05 12 23 03 02        	leaq	0x2032312(%rip), %r8    # 0x142a98f20
140a66c0e: b9 01 00 00 00              	movl	$0x1, %ecx
140a66c13: ba 13 00 00 00              	movl	$0x13, %edx
140a66c18: e8 c6 23 e7 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140a66c1d: 0f 0b                       	ud2
140a66c1f: 90                          	nop
140a66c20: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a66c25: 55                          	pushq	%rbp
140a66c26: 41 57                       	pushq	%r15
140a66c28: 41 56                       	pushq	%r14
140a66c2a: 56                          	pushq	%rsi
140a66c2b: 57                          	pushq	%rdi
140a66c2c: 53                          	pushq	%rbx
140a66c2d: 48 83 ec 28                 	subq	$0x28, %rsp
140a66c31: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a66c38: ba 08 00 00 00              	movl	$0x8, %edx
140a66c3d: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140a66c43: 48 8b 4d 78                 	movq	0x78(%rbp), %rcx
140a66c47: e8 d4 cf ba ff              	callq	0x140613c20 <.text+0x612c20>
140a66c4c: 90                          	nop
140a66c4d: 48 83 c4 28                 	addq	$0x28, %rsp
140a66c51: 5b                          	popq	%rbx
140a66c52: 5f                          	popq	%rdi
140a66c53: 5e                          	popq	%rsi
140a66c54: 41 5e                       	popq	%r14
140a66c56: 41 5f                       	popq	%r15
140a66c58: 5d                          	popq	%rbp
140a66c59: c3                          	retq
140a66c5a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140a66c60: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a66c65: 55                          	pushq	%rbp
140a66c66: 41 57                       	pushq	%r15
140a66c68: 41 56                       	pushq	%r14
140a66c6a: 56                          	pushq	%rsi
140a66c6b: 57                          	pushq	%rdi
140a66c6c: 53                          	pushq	%rbx
140a66c6d: 48 83 ec 28                 	subq	$0x28, %rsp
140a66c71: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a66c78: 48 8d 4d 48                 	leaq	0x48(%rbp), %rcx
140a66c7c: e8 1f 41 e3 ff              	callq	0x14089ada0 <.text+0x899da0>
140a66c81: 90                          	nop
140a66c82: 48 83 c4 28                 	addq	$0x28, %rsp
140a66c86: 5b                          	popq	%rbx
140a66c87: 5f                          	popq	%rdi
140a66c88: 5e                          	popq	%rsi
140a66c89: 41 5e                       	popq	%r14
140a66c8b: 41 5f                       	popq	%r15
140a66c8d: 5d                          	popq	%rbp
140a66c8e: c3                          	retq
