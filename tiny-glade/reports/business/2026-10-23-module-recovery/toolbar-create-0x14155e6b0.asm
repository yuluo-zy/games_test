
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14155e6b0: 55                          	pushq	%rbp
14155e6b1: 41 57                       	pushq	%r15
14155e6b3: 41 56                       	pushq	%r14
14155e6b5: 41 55                       	pushq	%r13
14155e6b7: 41 54                       	pushq	%r12
14155e6b9: 56                          	pushq	%rsi
14155e6ba: 57                          	pushq	%rdi
14155e6bb: 53                          	pushq	%rbx
14155e6bc: 48 81 ec 88 01 00 00        	subq	$0x188, %rsp            # imm = 0x188
14155e6c3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
14155e6cb: 0f 29 b5 f0 00 00 00        	movaps	%xmm6, 0xf0(%rbp)
14155e6d2: 48 c7 85 e8 00 00 00 fe ff ff ff    	movq	$-0x2, 0xe8(%rbp)
14155e6dd: 4d 89 ce                    	movq	%r9, %r14
14155e6e0: 4d 89 c7                    	movq	%r8, %r15
14155e6e3: 48 89 d7                    	movq	%rdx, %rdi
14155e6e6: 48 89 ce                    	movq	%rcx, %rsi
14155e6e9: 0f b6 05 19 65 0d 02        	movzbl	0x20d6519(%rip), %eax   # 0x143634c09
14155e6f0: b9 08 00 00 00              	movl	$0x8, %ecx
14155e6f5: ba 08 00 00 00              	movl	$0x8, %edx
14155e6fa: e8 11 55 0b ff              	callq	0x140613c10 <.text+0x612c10>
14155e6ff: 48 85 c0                    	testq	%rax, %rax
14155e702: 0f 84 72 03 00 00           	je	0x14155ea7a <.text+0x155da7a>
14155e708: 48 89 c3                    	movq	%rax, %rbx
14155e70b: 0f b6 05 f7 64 0d 02        	movzbl	0x20d64f7(%rip), %eax   # 0x143634c09
14155e712: b9 18 00 00 00              	movl	$0x18, %ecx
14155e717: ba 04 00 00 00              	movl	$0x4, %edx
14155e71c: e8 ef 54 0b ff              	callq	0x140613c10 <.text+0x612c10>
14155e721: 48 85 c0                    	testq	%rax, %rax
14155e724: 48 89 9d e0 00 00 00        	movq	%rbx, 0xe0(%rbp)
14155e72b: 0f 84 58 03 00 00           	je	0x14155ea89 <.text+0x155da89>
14155e731: 4c 89 bd d8 00 00 00        	movq	%r15, 0xd8(%rbp)
14155e738: 4c 89 b5 d0 00 00 00        	movq	%r14, 0xd0(%rbp)
14155e73f: 48 8b 4f 10                 	movq	0x10(%rdi), %rcx
14155e743: 48 89 48 10                 	movq	%rcx, 0x10(%rax)
14155e747: 0f 10 07                    	movups	(%rdi), %xmm0
14155e74a: 0f 11 00                    	movups	%xmm0, (%rax)
14155e74d: 48 89 45 b8                 	movq	%rax, -0x48(%rbp)
14155e751: 48 8d 05 18 fc 70 01        	leaq	0x170fc18(%rip), %rax   # 0x142c6e370
14155e758: 48 89 45 c0                 	movq	%rax, -0x40(%rbp)
14155e75c: 48 b8 04 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffc, %rax # imm = 0x8000000000000004
14155e766: 48 89 45 b0                 	movq	%rax, -0x50(%rbp)
14155e76a: 0f b6 05 98 64 0d 02        	movzbl	0x20d6498(%rip), %eax   # 0x143634c09
14155e771: b9 13 00 00 00              	movl	$0x13, %ecx
14155e776: ba 01 00 00 00              	movl	$0x1, %edx
14155e77b: e8 90 54 0b ff              	callq	0x140613c10 <.text+0x612c10>
14155e780: 48 85 c0                    	testq	%rax, %rax
14155e783: 0f 84 11 03 00 00           	je	0x14155ea9a <.text+0x155da9a>
14155e789: 49 89 c7                    	movq	%rax, %r15
14155e78c: 8b 05 09 ad 53 01           	movl	0x153ad09(%rip), %eax   # 0x142a9949b
14155e792: 41 89 47 0f                 	movl	%eax, 0xf(%r15)
14155e796: 0f 10 05 ef ac 53 01        	movups	0x153acef(%rip), %xmm0  # 0x142a9948c
14155e79d: 41 0f 11 07                 	movups	%xmm0, (%r15)
14155e7a1: 0f 10 45 b0                 	movups	-0x50(%rbp), %xmm0
14155e7a5: 0f 10 4d c0                 	movups	-0x40(%rbp), %xmm1
14155e7a9: 0f 10 55 d0                 	movups	-0x30(%rbp), %xmm2
14155e7ad: 0f 29 45 50                 	movaps	%xmm0, 0x50(%rbp)
14155e7b1: 0f 29 4d 60                 	movaps	%xmm1, 0x60(%rbp)
14155e7b5: 0f 29 55 70                 	movaps	%xmm2, 0x70(%rbp)
14155e7b9: 48 c7 85 bc 00 00 00 00 00 00 00    	movq	$0x0, 0xbc(%rbp)
14155e7c4: 48 c7 85 c1 00 00 00 00 00 00 00    	movq	$0x0, 0xc1(%rbp)
14155e7cf: 0f 28 05 ea f8 3d 01        	movaps	0x13df8ea(%rip), %xmm0  # 0x14293e0c0
14155e7d6: 0f 29 45 40                 	movaps	%xmm0, 0x40(%rbp)
14155e7da: 66 c7 85 80 00 00 00 00 01  	movw	$0x100, 0x80(%rbp)      # imm = 0x100
14155e7e3: 48 b8 00 00 80 3f 01 00 00 00       	movabsq	$0x13f800000, %rax # imm = 0x13F800000
14155e7ed: 48 89 85 84 00 00 00        	movq	%rax, 0x84(%rbp)
14155e7f4: c7 85 90 00 00 00 00 00 00 00       	movl	$0x0, 0x90(%rbp)
14155e7fe: c7 85 98 00 00 00 00 00 00 00       	movl	$0x0, 0x98(%rbp)
14155e808: 48 c7 85 a0 00 00 00 13 00 00 00    	movq	$0x13, 0xa0(%rbp)
14155e813: 4c 89 bd a8 00 00 00        	movq	%r15, 0xa8(%rbp)
14155e81a: 48 c7 85 b0 00 00 00 13 00 00 00    	movq	$0x13, 0xb0(%rbp)
14155e825: c7 85 b8 00 00 00 00 00 00 00       	movl	$0x0, 0xb8(%rbp)
14155e82f: 66 c7 85 c9 00 00 00 00 00  	movw	$0x0, 0xc9(%rbp)
14155e838: 44 0f b6 77 18              	movzbl	0x18(%rdi), %r14d
14155e83d: 44 89 f0                    	movl	%r14d, %eax
14155e840: 83 f0 01                    	xorl	$0x1, %eax
14155e843: 4c 8d 24 c5 0b 00 00 00     	leaq	0xb(,%rax,8), %r12
14155e84b: 0f b6 05 b7 63 0d 02        	movzbl	0x20d63b7(%rip), %eax   # 0x143634c09
14155e852: ba 01 00 00 00              	movl	$0x1, %edx
14155e857: 4c 89 e1                    	movq	%r12, %rcx
14155e85a: e8 b1 53 0b ff              	callq	0x140613c10 <.text+0x612c10>
14155e85f: 48 85 c0                    	testq	%rax, %rax
14155e862: 0f 84 4a 02 00 00           	je	0x14155eab2 <.text+0x155dab2>
14155e868: 49 89 c5                    	movq	%rax, %r13
14155e86b: 48 8b 9d 70 01 00 00        	movq	0x170(%rbp), %rbx
14155e872: 48 8d 05 3a fb 70 01        	leaq	0x170fb3a(%rip), %rax   # 0x142c6e3b3
14155e879: 48 8d 15 20 fb 70 01        	leaq	0x170fb20(%rip), %rdx   # 0x142c6e3a0
14155e880: 45 84 f6                    	testb	%r14b, %r14b
14155e883: 48 0f 45 d0                 	cmovneq	%rax, %rdx
14155e887: 4c 89 e9                    	movq	%r13, %rcx
14155e88a: 4d 89 e0                    	movq	%r12, %r8
14155e88d: e8 6e 53 3c 01              	callq	0x142923c00 <.text+0x2922c00>
14155e892: ba 13 00 00 00              	movl	$0x13, %edx
14155e897: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14155e89d: 4c 89 f9                    	movq	%r15, %rcx
14155e8a0: e8 7b 53 0b ff              	callq	0x140613c20 <.text+0x612c20>
14155e8a5: 4c 89 a5 a0 00 00 00        	movq	%r12, 0xa0(%rbp)
14155e8ac: 4c 89 ad a8 00 00 00        	movq	%r13, 0xa8(%rbp)
14155e8b3: 4c 89 a5 b0 00 00 00        	movq	%r12, 0xb0(%rbp)
14155e8ba: 4c 8d 7d b0                 	leaq	-0x50(%rbp), %r15
14155e8be: 48 8d 55 40                 	leaq	0x40(%rbp), %rdx
14155e8c2: 41 b8 90 00 00 00           	movl	$0x90, %r8d
14155e8c8: 4c 89 f9                    	movq	%r15, %rcx
14155e8cb: e8 30 53 3c 01              	callq	0x142923c00 <.text+0x2922c00>
14155e8d0: 48 89 5c 24 20              	movq	%rbx, 0x20(%rsp)
14155e8d5: 48 8d 4d 40                 	leaq	0x40(%rbp), %rcx
14155e8d9: 4c 89 fa                    	movq	%r15, %rdx
14155e8dc: 4c 8b 85 d8 00 00 00        	movq	0xd8(%rbp), %r8
14155e8e3: 4c 8b 8d d0 00 00 00        	movq	0xd0(%rbp), %r9
14155e8ea: e8 61 f4 33 ff              	callq	0x14089dd50 <.text+0x89cd50>
14155e8ef: 48 8b 5d 48                 	movq	0x48(%rbp), %rbx
14155e8f3: 48 85 db                    	testq	%rbx, %rbx
14155e8f6: 75 04                       	jne	0x14155e8fc <.text+0x155d8fc>
14155e8f8: 48 8b 5d 50                 	movq	0x50(%rbp), %rbx
14155e8fc: 4c 8b 7d 60                 	movq	0x60(%rbp), %r15
14155e900: 48 8b 03                    	movq	(%rbx), %rax
14155e903: 4c 8b 73 10                 	movq	0x10(%rbx), %r14
14155e907: 4c 29 f0                    	subq	%r14, %rax
14155e90a: 48 83 f8 27                 	cmpq	$0x27, %rax
14155e90e: 0f 86 fa 00 00 00           	jbe	0x14155ea0e <.text+0x155da0e>
14155e914: 48 8b 43 08                 	movq	0x8(%rbx), %rax
14155e918: 48 8d 0d d1 df d7 ff        	leaq	-0x28202f(%rip), %rcx   # 0x1412dc8f0 <.text+0x12db8f0>
14155e91f: 4a 89 0c 30                 	movq	%rcx, (%rax,%r14)
14155e923: 4e 89 7c 30 08              	movq	%r15, 0x8(%rax,%r14)
14155e928: 48 b9 01 00 00 00 00 00 40 40       	movabsq	$0x4040000000000001, %rcx # imm = 0x4040000000000001
14155e932: 4a 89 4c 30 10              	movq	%rcx, 0x10(%rax,%r14)
14155e937: 42 c6 44 30 18 00           	movb	$0x0, 0x18(%rax,%r14)
14155e93d: 4c 8d 25 ec 63 00 00        	leaq	0x63ec(%rip), %r12      # 0x141564d30 <.text+0x1563d30>
14155e944: 4e 89 64 30 20              	movq	%r12, 0x20(%rax,%r14)
14155e949: 49 83 c6 28                 	addq	$0x28, %r14
14155e94d: 4c 89 73 10                 	movq	%r14, 0x10(%rbx)
14155e951: 48 89 f9                    	movq	%rdi, %rcx
14155e954: e8 d7 ab 73 ff              	callq	0x140c99530 <.text+0xc98530>
14155e959: 0f 28 f0                    	movaps	%xmm0, %xmm6
14155e95c: 48 8b 03                    	movq	(%rbx), %rax
14155e95f: 48 8b 7b 10                 	movq	0x10(%rbx), %rdi
14155e963: 48 29 f8                    	subq	%rdi, %rax
14155e966: 48 83 f8 1f                 	cmpq	$0x1f, %rax
14155e96a: 0f 86 c3 00 00 00           	jbe	0x14155ea33 <.text+0x155da33>
14155e970: 48 8b 43 08                 	movq	0x8(%rbx), %rax
14155e974: 48 8d 0d e5 a2 d7 ff        	leaq	-0x285d1b(%rip), %rcx   # 0x1412d8c60 <.text+0x12d7c60>
14155e97b: 48 89 0c 38                 	movq	%rcx, (%rax,%rdi)
14155e97f: 4c 89 64 38 08              	movq	%r12, 0x8(%rax,%rdi)
14155e984: 4c 89 7c 38 10              	movq	%r15, 0x10(%rax,%rdi)
14155e989: f3 0f 11 74 38 18           	movss	%xmm6, 0x18(%rax,%rdi)
14155e98f: c6 44 38 1c 00              	movb	$0x0, 0x1c(%rax,%rdi)
14155e994: 48 8d 57 20                 	leaq	0x20(%rdi), %rdx
14155e998: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
14155e99c: 48 8b 03                    	movq	(%rbx), %rax
14155e99f: 48 29 d0                    	subq	%rdx, %rax
14155e9a2: 48 83 f8 1f                 	cmpq	$0x1f, %rax
14155e9a6: 0f 86 ac 00 00 00           	jbe	0x14155ea58 <.text+0x155da58>
14155e9ac: 48 8b 43 08                 	movq	0x8(%rbx), %rax
14155e9b0: 48 8d 0d 69 b8 d7 ff        	leaq	-0x284797(%rip), %rcx   # 0x1412da220 <.text+0x12d9220>
14155e9b7: 48 89 4c 38 20              	movq	%rcx, 0x20(%rax,%rdi)
14155e9bc: 4c 89 64 38 28              	movq	%r12, 0x28(%rax,%rdi)
14155e9c1: 4c 89 7c 38 30              	movq	%r15, 0x30(%rax,%rdi)
14155e9c6: c6 44 38 38 00              	movb	$0x0, 0x38(%rax,%rdi)
14155e9cb: 48 83 c7 40                 	addq	$0x40, %rdi
14155e9cf: 48 89 7b 10                 	movq	%rdi, 0x10(%rbx)
14155e9d3: 48 8b 85 e0 00 00 00        	movq	0xe0(%rbp), %rax
14155e9da: 4c 89 38                    	movq	%r15, (%rax)
14155e9dd: 48 c7 06 01 00 00 00        	movq	$0x1, (%rsi)
14155e9e4: 48 89 46 08                 	movq	%rax, 0x8(%rsi)
14155e9e8: 48 c7 46 10 01 00 00 00     	movq	$0x1, 0x10(%rsi)
14155e9f0: 48 89 f0                    	movq	%rsi, %rax
14155e9f3: 0f 28 b5 f0 00 00 00        	movaps	0xf0(%rbp), %xmm6
14155e9fa: 48 81 c4 88 01 00 00        	addq	$0x188, %rsp            # imm = 0x188
14155ea01: 5b                          	popq	%rbx
14155ea02: 5f                          	popq	%rdi
14155ea03: 5e                          	popq	%rsi
14155ea04: 41 5c                       	popq	%r12
14155ea06: 41 5d                       	popq	%r13
14155ea08: 41 5e                       	popq	%r14
14155ea0a: 41 5f                       	popq	%r15
14155ea0c: 5d                          	popq	%rbp
14155ea0d: c3                          	retq
14155ea0e: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
14155ea17: 41 b8 28 00 00 00           	movl	$0x28, %r8d
14155ea1d: 41 b9 01 00 00 00           	movl	$0x1, %r9d
14155ea23: 48 89 d9                    	movq	%rbx, %rcx
14155ea26: 4c 89 f2                    	movq	%r14, %rdx
14155ea29: e8 32 b0 34 01              	callq	0x1428a9a60 <.text+0x28a8a60>
14155ea2e: e9 e1 fe ff ff              	jmp	0x14155e914 <.text+0x155d914>
14155ea33: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
14155ea3c: 41 b8 20 00 00 00           	movl	$0x20, %r8d
14155ea42: 41 b9 01 00 00 00           	movl	$0x1, %r9d
14155ea48: 48 89 d9                    	movq	%rbx, %rcx
14155ea4b: 48 89 fa                    	movq	%rdi, %rdx
14155ea4e: e8 0d b0 34 01              	callq	0x1428a9a60 <.text+0x28a8a60>
14155ea53: e9 18 ff ff ff              	jmp	0x14155e970 <.text+0x155d970>
14155ea58: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
14155ea61: 41 b8 20 00 00 00           	movl	$0x20, %r8d
14155ea67: 41 b9 01 00 00 00           	movl	$0x1, %r9d
14155ea6d: 48 89 d9                    	movq	%rbx, %rcx
14155ea70: e8 eb af 34 01              	callq	0x1428a9a60 <.text+0x28a8a60>
14155ea75: e9 32 ff ff ff              	jmp	0x14155e9ac <.text+0x155d9ac>
14155ea7a: b9 08 00 00 00              	movl	$0x8, %ecx
14155ea7f: ba 08 00 00 00              	movl	$0x8, %edx
14155ea84: e8 77 a5 37 01              	callq	0x1428d9000 <.text+0x28d8000>
14155ea89: b9 04 00 00 00              	movl	$0x4, %ecx
14155ea8e: ba 18 00 00 00              	movl	$0x18, %edx
14155ea93: e8 68 a5 37 01              	callq	0x1428d9000 <.text+0x28d8000>
14155ea98: eb 2c                       	jmp	0x14155eac6 <.text+0x155dac6>
14155ea9a: 4c 8d 05 7f a4 53 01        	leaq	0x153a47f(%rip), %r8    # 0x142a98f20
14155eaa1: b9 01 00 00 00              	movl	$0x1, %ecx
14155eaa6: ba 13 00 00 00              	movl	$0x13, %edx
14155eaab: e8 33 a5 37 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
14155eab0: eb 14                       	jmp	0x14155eac6 <.text+0x155dac6>
14155eab2: 4c 8d 05 67 a4 53 01        	leaq	0x153a467(%rip), %r8    # 0x142a98f20
14155eab9: b9 01 00 00 00              	movl	$0x1, %ecx
14155eabe: 4c 89 e2                    	movq	%r12, %rdx
14155eac1: e8 1d a5 37 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
14155eac6: 0f 0b                       	ud2
14155eac8: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
14155ead0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14155ead5: 55                          	pushq	%rbp
14155ead6: 41 57                       	pushq	%r15
14155ead8: 41 56                       	pushq	%r14
14155eada: 41 55                       	pushq	%r13
14155eadc: 41 54                       	pushq	%r12
14155eade: 56                          	pushq	%rsi
14155eadf: 57                          	pushq	%rdi
14155eae0: 53                          	pushq	%rbx
14155eae1: 48 83 ec 38                 	subq	$0x38, %rsp
14155eae5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14155eaec: 0f 29 74 24 20              	movaps	%xmm6, 0x20(%rsp)
14155eaf1: ba 08 00 00 00              	movl	$0x8, %edx
14155eaf6: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14155eafc: 48 8b 8d e0 00 00 00        	movq	0xe0(%rbp), %rcx
14155eb03: e8 18 51 0b ff              	callq	0x140613c20 <.text+0x612c20>
14155eb08: 0f 28 74 24 20              	movaps	0x20(%rsp), %xmm6
14155eb0d: 48 83 c4 38                 	addq	$0x38, %rsp
14155eb11: 5b                          	popq	%rbx
14155eb12: 5f                          	popq	%rdi
14155eb13: 5e                          	popq	%rsi
14155eb14: 41 5c                       	popq	%r12
14155eb16: 41 5d                       	popq	%r13
14155eb18: 41 5e                       	popq	%r14
14155eb1a: 41 5f                       	popq	%r15
14155eb1c: 5d                          	popq	%rbp
14155eb1d: c3                          	retq
14155eb1e: 66 90                       	nop
14155eb20: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14155eb25: 55                          	pushq	%rbp
14155eb26: 41 57                       	pushq	%r15
14155eb28: 41 56                       	pushq	%r14
14155eb2a: 41 55                       	pushq	%r13
14155eb2c: 41 54                       	pushq	%r12
14155eb2e: 56                          	pushq	%rsi
14155eb2f: 57                          	pushq	%rdi
14155eb30: 53                          	pushq	%rbx
14155eb31: 48 83 ec 38                 	subq	$0x38, %rsp
14155eb35: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14155eb3c: 0f 29 74 24 20              	movaps	%xmm6, 0x20(%rsp)
14155eb41: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
14155eb45: e8 56 c2 33 ff              	callq	0x14089ada0 <.text+0x899da0>
14155eb4a: 0f 28 74 24 20              	movaps	0x20(%rsp), %xmm6
14155eb4f: 48 83 c4 38                 	addq	$0x38, %rsp
14155eb53: 5b                          	popq	%rbx
14155eb54: 5f                          	popq	%rdi
14155eb55: 5e                          	popq	%rsi
14155eb56: 41 5c                       	popq	%r12
14155eb58: 41 5d                       	popq	%r13
14155eb5a: 41 5e                       	popq	%r14
14155eb5c: 41 5f                       	popq	%r15
14155eb5e: 5d                          	popq	%rbp
14155eb5f: c3                          	retq
14155eb60: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14155eb65: 55                          	pushq	%rbp
14155eb66: 41 57                       	pushq	%r15
14155eb68: 41 56                       	pushq	%r14
14155eb6a: 41 55                       	pushq	%r13
14155eb6c: 41 54                       	pushq	%r12
14155eb6e: 56                          	pushq	%rsi
14155eb6f: 57                          	pushq	%rdi
14155eb70: 53                          	pushq	%rbx
14155eb71: 48 83 ec 38                 	subq	$0x38, %rsp
14155eb75: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14155eb7c: 0f 29 74 24 20              	movaps	%xmm6, 0x20(%rsp)
14155eb81: 48 8d 4d 40                 	leaq	0x40(%rbp), %rcx
14155eb85: e8 c6 4d 33 ff              	callq	0x140893950 <.text+0x892950>
14155eb8a: 0f 28 74 24 20              	movaps	0x20(%rsp), %xmm6
14155eb8f: 48 83 c4 38                 	addq	$0x38, %rsp
14155eb93: 5b                          	popq	%rbx
14155eb94: 5f                          	popq	%rdi
14155eb95: 5e                          	popq	%rsi
14155eb96: 41 5c                       	popq	%r12
14155eb98: 41 5d                       	popq	%r13
14155eb9a: 41 5e                       	popq	%r14
14155eb9c: 41 5f                       	popq	%r15
14155eb9e: 5d                          	popq	%rbp
14155eb9f: c3                          	retq
