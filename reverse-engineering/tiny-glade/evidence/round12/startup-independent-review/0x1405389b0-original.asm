
D:\game\reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1405389b0: 55                          	pushq	%rbp
1405389b1: 56                          	pushq	%rsi
1405389b2: 57                          	pushq	%rdi
1405389b3: 53                          	pushq	%rbx
1405389b4: 48 81 ec 98 05 00 00        	subq	$0x598, %rsp            # imm = 0x598
1405389bb: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1405389c3: 48 c7 85 10 05 00 00 fe ff ff ff    	movq	$-0x2, 0x510(%rbp)
1405389ce: 48 89 ce                    	movq	%rcx, %rsi
1405389d1: 0f b6 05 31 c2 0f 03        	movzbl	0x30fc231(%rip), %eax   # 0x143634c09
1405389d8: b9 30 02 00 00              	movl	$0x230, %ecx            # imm = 0x230
1405389dd: ba 08 00 00 00              	movl	$0x8, %edx
1405389e2: e8 29 b2 0d 00              	callq	0x140613c10 <.text+0x612c10>
1405389e7: 48 85 c0                    	testq	%rax, %rax
1405389ea: 0f 84 ba 03 00 00           	je	0x140538daa <.text+0x537daa>
1405389f0: 48 89 85 08 05 00 00        	movq	%rax, 0x508(%rbp)
1405389f7: 0f 57 c0                    	xorps	%xmm0, %xmm0
1405389fa: 0f 11 45 c0                 	movups	%xmm0, -0x40(%rbp)
1405389fe: 0f 11 45 d8                 	movups	%xmm0, -0x28(%rbp)
140538a02: 0f 11 45 f0                 	movups	%xmm0, -0x10(%rbp)
140538a06: 0f 11 45 08                 	movups	%xmm0, 0x8(%rbp)
140538a0a: 0f 11 45 20                 	movups	%xmm0, 0x20(%rbp)
140538a0e: c7 45 30 00 00 00 00        	movl	$0x0, 0x30(%rbp)
140538a15: 0f 11 45 58                 	movups	%xmm0, 0x58(%rbp)
140538a19: 0f 11 45 70                 	movups	%xmm0, 0x70(%rbp)
140538a1d: 0f 11 85 88 00 00 00        	movups	%xmm0, 0x88(%rbp)
140538a24: 0f 11 85 a0 00 00 00        	movups	%xmm0, 0xa0(%rbp)
140538a2b: 0f 11 85 b8 00 00 00        	movups	%xmm0, 0xb8(%rbp)
140538a32: c7 85 c8 00 00 00 00 00 00 00       	movl	$0x0, 0xc8(%rbp)
140538a3c: 48 c7 45 a0 00 00 00 00     	movq	$0x0, -0x60(%rbp)
140538a44: 48 c7 45 a8 08 00 00 00     	movq	$0x8, -0x58(%rbp)
140538a4c: 48 c7 45 b0 00 00 00 00     	movq	$0x0, -0x50(%rbp)
140538a54: 48 c7 45 b8 10 00 00 00     	movq	$0x10, -0x48(%rbp)
140538a5c: 48 c7 45 d0 10 00 00 00     	movq	$0x10, -0x30(%rbp)
140538a64: 48 c7 45 e8 10 00 00 00     	movq	$0x10, -0x18(%rbp)
140538a6c: 48 c7 45 00 10 00 00 00     	movq	$0x10, (%rbp)
140538a74: 48 c7 45 18 10 00 00 00     	movq	$0x10, 0x18(%rbp)
140538a7c: 48 bb 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rbx # imm = 0x8000000000000000
140538a86: 48 89 5d 38                 	movq	%rbx, 0x38(%rbp)
140538a8a: 48 8d 05 8f db 4b 02        	leaq	0x24bdb8f(%rip), %rax   # 0x1429f6620
140538a91: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
140538a95: 48 c7 45 48 4d 00 00 00     	movq	$0x4d, 0x48(%rbp)
140538a9d: 48 c7 45 50 10 00 00 00     	movq	$0x10, 0x50(%rbp)
140538aa5: 48 c7 45 68 10 00 00 00     	movq	$0x10, 0x68(%rbp)
140538aad: 48 c7 85 80 00 00 00 10 00 00 00    	movq	$0x10, 0x80(%rbp)
140538ab8: 48 c7 85 98 00 00 00 10 00 00 00    	movq	$0x10, 0x98(%rbp)
140538ac3: 48 c7 85 b0 00 00 00 10 00 00 00    	movq	$0x10, 0xb0(%rbp)
140538ace: c7 85 d0 00 00 00 00 00 00 00       	movl	$0x0, 0xd0(%rbp)
140538ad8: 66 c7 85 d4 00 00 00 01 00  	movw	$0x1, 0xd4(%rbp)
140538ae1: 48 89 9d c8 01 00 00        	movq	%rbx, 0x1c8(%rbp)
140538ae8: c7 85 40 03 00 00 00 00 00 00       	movl	$0x0, 0x340(%rbp)
140538af2: 0f b6 05 10 c1 0f 03        	movzbl	0x30fc110(%rip), %eax   # 0x143634c09
140538af9: b9 a8 03 00 00              	movl	$0x3a8, %ecx            # imm = 0x3A8
140538afe: ba 08 00 00 00              	movl	$0x8, %edx
140538b03: e8 08 b1 0d 00              	callq	0x140613c10 <.text+0x612c10>
140538b08: 48 85 c0                    	testq	%rax, %rax
140538b0b: 0f 84 a8 02 00 00           	je	0x140538db9 <.text+0x537db9>
140538b11: 48 89 c7                    	movq	%rax, %rdi
140538b14: 48 8d 55 a0                 	leaq	-0x60(%rbp), %rdx
140538b18: 41 b8 a8 03 00 00           	movl	$0x3a8, %r8d            # imm = 0x3A8
140538b1e: 48 89 c1                    	movq	%rax, %rcx
140538b21: e8 da b0 3e 02              	callq	0x142923c00 <.text+0x2922c00>
140538b26: 4c 8d 05 eb d7 4b 02        	leaq	0x24bd7eb(%rip), %r8    # 0x1429f6318
140538b2d: 48 8d 8d 98 04 00 00        	leaq	0x498(%rbp), %rcx
140538b34: 48 89 fa                    	movq	%rdi, %rdx
140538b37: e8 d4 bf 7a 00              	callq	0x140ce4b10 <.text+0xce3b10>
140538b3c: 48 8d 8d 28 04 00 00        	leaq	0x428(%rbp), %rcx
140538b43: e8 18 e5 fa ff              	callq	0x1404e7060 <.text+0x4e6060>
140538b48: 48 8d 8d b8 03 00 00        	leaq	0x3b8(%rbp), %rcx
140538b4f: e8 8c e4 c3 ff              	callq	0x140176fe0 <.text+0x175fe0>
140538b54: 48 8d 8d 48 03 00 00        	leaq	0x348(%rbp), %rcx
140538b5b: e8 c0 ab d5 ff              	callq	0x140293720 <.text+0x292720>
140538b60: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
140538b64: e8 f7 6e d9 ff              	callq	0x1402cfa60 <.text+0x2cea60>
140538b69: 0f 10 85 f8 04 00 00        	movups	0x4f8(%rbp), %xmm0
140538b70: 48 8b 85 08 05 00 00        	movq	0x508(%rbp), %rax
140538b77: 0f 11 40 60                 	movups	%xmm0, 0x60(%rax)
140538b7b: 0f 10 85 e8 04 00 00        	movups	0x4e8(%rbp), %xmm0
140538b82: 0f 11 40 50                 	movups	%xmm0, 0x50(%rax)
140538b86: 0f 10 85 d8 04 00 00        	movups	0x4d8(%rbp), %xmm0
140538b8d: 0f 11 40 40                 	movups	%xmm0, 0x40(%rax)
140538b91: 0f 10 85 98 04 00 00        	movups	0x498(%rbp), %xmm0
140538b98: 0f 10 8d a8 04 00 00        	movups	0x4a8(%rbp), %xmm1
140538b9f: 0f 10 95 b8 04 00 00        	movups	0x4b8(%rbp), %xmm2
140538ba6: 0f 10 9d c8 04 00 00        	movups	0x4c8(%rbp), %xmm3
140538bad: 0f 11 58 30                 	movups	%xmm3, 0x30(%rax)
140538bb1: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
140538bb5: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
140538bb9: 0f 11 00                    	movups	%xmm0, (%rax)
140538bbc: 0f 10 85 28 04 00 00        	movups	0x428(%rbp), %xmm0
140538bc3: 0f 10 8d 38 04 00 00        	movups	0x438(%rbp), %xmm1
140538bca: 0f 10 95 48 04 00 00        	movups	0x448(%rbp), %xmm2
140538bd1: 0f 10 9d 58 04 00 00        	movups	0x458(%rbp), %xmm3
140538bd8: 0f 11 40 70                 	movups	%xmm0, 0x70(%rax)
140538bdc: 0f 11 88 80 00 00 00        	movups	%xmm1, 0x80(%rax)
140538be3: 0f 11 90 90 00 00 00        	movups	%xmm2, 0x90(%rax)
140538bea: 0f 11 98 a0 00 00 00        	movups	%xmm3, 0xa0(%rax)
140538bf1: 0f 10 85 68 04 00 00        	movups	0x468(%rbp), %xmm0
140538bf8: 0f 11 80 b0 00 00 00        	movups	%xmm0, 0xb0(%rax)
140538bff: 0f 10 85 78 04 00 00        	movups	0x478(%rbp), %xmm0
140538c06: 0f 11 80 c0 00 00 00        	movups	%xmm0, 0xc0(%rax)
140538c0d: 0f 10 85 88 04 00 00        	movups	0x488(%rbp), %xmm0
140538c14: 0f 11 80 d0 00 00 00        	movups	%xmm0, 0xd0(%rax)
140538c1b: 0f 10 85 18 04 00 00        	movups	0x418(%rbp), %xmm0
140538c22: 0f 11 80 40 01 00 00        	movups	%xmm0, 0x140(%rax)
140538c29: 0f 10 85 08 04 00 00        	movups	0x408(%rbp), %xmm0
140538c30: 0f 11 80 30 01 00 00        	movups	%xmm0, 0x130(%rax)
140538c37: 0f 10 85 f8 03 00 00        	movups	0x3f8(%rbp), %xmm0
140538c3e: 0f 11 80 20 01 00 00        	movups	%xmm0, 0x120(%rax)
140538c45: 0f 10 85 b8 03 00 00        	movups	0x3b8(%rbp), %xmm0
140538c4c: 0f 10 8d c8 03 00 00        	movups	0x3c8(%rbp), %xmm1
140538c53: 0f 10 95 d8 03 00 00        	movups	0x3d8(%rbp), %xmm2
140538c5a: 0f 10 9d e8 03 00 00        	movups	0x3e8(%rbp), %xmm3
140538c61: 0f 11 98 10 01 00 00        	movups	%xmm3, 0x110(%rax)
140538c68: 0f 11 90 00 01 00 00        	movups	%xmm2, 0x100(%rax)
140538c6f: 0f 11 88 f0 00 00 00        	movups	%xmm1, 0xf0(%rax)
140538c76: 0f 11 80 e0 00 00 00        	movups	%xmm0, 0xe0(%rax)
140538c7d: 0f 10 85 48 03 00 00        	movups	0x348(%rbp), %xmm0
140538c84: 0f 10 8d 58 03 00 00        	movups	0x358(%rbp), %xmm1
140538c8b: 0f 10 95 68 03 00 00        	movups	0x368(%rbp), %xmm2
140538c92: 0f 10 9d 78 03 00 00        	movups	0x378(%rbp), %xmm3
140538c99: 0f 11 80 50 01 00 00        	movups	%xmm0, 0x150(%rax)
140538ca0: 0f 11 88 60 01 00 00        	movups	%xmm1, 0x160(%rax)
140538ca7: 0f 11 90 70 01 00 00        	movups	%xmm2, 0x170(%rax)
140538cae: 0f 11 98 80 01 00 00        	movups	%xmm3, 0x180(%rax)
140538cb5: 0f 10 85 88 03 00 00        	movups	0x388(%rbp), %xmm0
140538cbc: 0f 11 80 90 01 00 00        	movups	%xmm0, 0x190(%rax)
140538cc3: 0f 10 85 98 03 00 00        	movups	0x398(%rbp), %xmm0
140538cca: 0f 11 80 a0 01 00 00        	movups	%xmm0, 0x1a0(%rax)
140538cd1: 0f 10 85 a8 03 00 00        	movups	0x3a8(%rbp), %xmm0
140538cd8: 0f 11 80 b0 01 00 00        	movups	%xmm0, 0x1b0(%rax)
140538cdf: 0f 10 45 a0                 	movups	-0x60(%rbp), %xmm0
140538ce3: 0f 10 4d b0                 	movups	-0x50(%rbp), %xmm1
140538ce7: 0f 10 55 c0                 	movups	-0x40(%rbp), %xmm2
140538ceb: 0f 10 5d d0                 	movups	-0x30(%rbp), %xmm3
140538cef: 0f 11 80 c0 01 00 00        	movups	%xmm0, 0x1c0(%rax)
140538cf6: 0f 11 88 d0 01 00 00        	movups	%xmm1, 0x1d0(%rax)
140538cfd: 0f 11 90 e0 01 00 00        	movups	%xmm2, 0x1e0(%rax)
140538d04: 0f 11 98 f0 01 00 00        	movups	%xmm3, 0x1f0(%rax)
140538d0b: 0f 10 45 e0                 	movups	-0x20(%rbp), %xmm0
140538d0f: 0f 11 80 00 02 00 00        	movups	%xmm0, 0x200(%rax)
140538d16: 0f 10 45 00                 	movups	(%rbp), %xmm0
140538d1a: 0f 11 80 20 02 00 00        	movups	%xmm0, 0x220(%rax)
140538d21: 0f 10 45 f0                 	movups	-0x10(%rbp), %xmm0
140538d25: 0f 11 80 10 02 00 00        	movups	%xmm0, 0x210(%rax)
140538d2c: 48 c7 45 a8 05 00 00 00     	movq	$0x5, -0x58(%rbp)
140538d34: 48 89 45 b0                 	movq	%rax, -0x50(%rbp)
140538d38: 48 c7 45 b8 05 00 00 00     	movq	$0x5, -0x48(%rbp)
140538d40: 48 c7 45 c0 00 00 00 00     	movq	$0x0, -0x40(%rbp)
140538d48: 48 c7 45 c8 08 00 00 00     	movq	$0x8, -0x38(%rbp)
140538d50: 0f 57 c0                    	xorps	%xmm0, %xmm0
140538d53: 0f 11 45 d0                 	movups	%xmm0, -0x30(%rbp)
140538d57: 48 89 5d a0                 	movq	%rbx, -0x60(%rbp)
140538d5b: 48 8d 4d d8                 	leaq	-0x28(%rbp), %rcx
140538d5f: e8 6c 78 c3 ff              	callq	0x1401705d0 <.text+0x16f5d0>
140538d64: 0f 10 45 00                 	movups	(%rbp), %xmm0
140538d68: 0f 11 46 60                 	movups	%xmm0, 0x60(%rsi)
140538d6c: 0f 10 45 f0                 	movups	-0x10(%rbp), %xmm0
140538d70: 0f 11 46 50                 	movups	%xmm0, 0x50(%rsi)
140538d74: 0f 10 45 e0                 	movups	-0x20(%rbp), %xmm0
140538d78: 0f 11 46 40                 	movups	%xmm0, 0x40(%rsi)
140538d7c: 0f 10 45 a0                 	movups	-0x60(%rbp), %xmm0
140538d80: 0f 10 4d b0                 	movups	-0x50(%rbp), %xmm1
140538d84: 0f 10 55 c0                 	movups	-0x40(%rbp), %xmm2
140538d88: 0f 10 5d d0                 	movups	-0x30(%rbp), %xmm3
140538d8c: 0f 11 5e 30                 	movups	%xmm3, 0x30(%rsi)
140538d90: 0f 11 56 20                 	movups	%xmm2, 0x20(%rsi)
140538d94: 0f 11 4e 10                 	movups	%xmm1, 0x10(%rsi)
140538d98: 0f 11 06                    	movups	%xmm0, (%rsi)
140538d9b: 48 89 f0                    	movq	%rsi, %rax
140538d9e: 48 81 c4 98 05 00 00        	addq	$0x598, %rsp            # imm = 0x598
140538da5: 5b                          	popq	%rbx
140538da6: 5f                          	popq	%rdi
140538da7: 5e                          	popq	%rsi
140538da8: 5d                          	popq	%rbp
140538da9: c3                          	retq
140538daa: b9 08 00 00 00              	movl	$0x8, %ecx
140538daf: ba 30 02 00 00              	movl	$0x230, %edx            # imm = 0x230
140538db4: e8 47 02 3a 02              	callq	0x1428d9000 <.text+0x28d8000>
140538db9: b9 08 00 00 00              	movl	$0x8, %ecx
140538dbe: ba a8 03 00 00              	movl	$0x3a8, %edx            # imm = 0x3A8
140538dc3: e8 38 02 3a 02              	callq	0x1428d9000 <.text+0x28d8000>
140538dc8: 0f 0b                       	ud2
140538dca: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140538dd0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140538dd5: 55                          	pushq	%rbp
140538dd6: 56                          	pushq	%rsi
140538dd7: 57                          	pushq	%rdi
140538dd8: 53                          	pushq	%rbx
140538dd9: 48 83 ec 28                 	subq	$0x28, %rsp
140538ddd: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140538de4: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
140538de8: e8 c3 88 09 00              	callq	0x1405d16b0 <.text+0x5d06b0>
140538ded: 90                          	nop
140538dee: 48 83 c4 28                 	addq	$0x28, %rsp
140538df2: 5b                          	popq	%rbx
140538df3: 5f                          	popq	%rdi
140538df4: 5e                          	popq	%rsi
140538df5: 5d                          	popq	%rbp
140538df6: c3                          	retq
140538df7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
140538e00: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140538e05: 55                          	pushq	%rbp
140538e06: 56                          	pushq	%rsi
140538e07: 57                          	pushq	%rdi
140538e08: 53                          	pushq	%rbx
140538e09: 48 83 ec 28                 	subq	$0x28, %rsp
140538e0d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140538e14: ba 30 02 00 00              	movl	$0x230, %edx            # imm = 0x230
140538e19: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140538e1f: 48 8b 8d 08 05 00 00        	movq	0x508(%rbp), %rcx
140538e26: e8 f5 ad 0d 00              	callq	0x140613c20 <.text+0x612c20>
140538e2b: 90                          	nop
140538e2c: 48 83 c4 28                 	addq	$0x28, %rsp
140538e30: 5b                          	popq	%rbx
140538e31: 5f                          	popq	%rdi
140538e32: 5e                          	popq	%rsi
140538e33: 5d                          	popq	%rbp
140538e34: c3                          	retq
140538e35: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
140538e40: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140538e45: 55                          	pushq	%rbp
140538e46: 56                          	pushq	%rsi
140538e47: 57                          	pushq	%rdi
140538e48: 53                          	pushq	%rbx
140538e49: 48 83 ec 28                 	subq	$0x28, %rsp
140538e4d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140538e54: 48 8d 8d 98 04 00 00        	leaq	0x498(%rbp), %rcx
140538e5b: e8 20 0a af ff              	callq	0x140029880 <.text+0x28880>
140538e60: 90                          	nop
140538e61: 48 83 c4 28                 	addq	$0x28, %rsp
140538e65: 5b                          	popq	%rbx
140538e66: 5f                          	popq	%rdi
140538e67: 5e                          	popq	%rsi
140538e68: 5d                          	popq	%rbp
140538e69: c3                          	retq
140538e6a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140538e70: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140538e75: 55                          	pushq	%rbp
140538e76: 56                          	pushq	%rsi
140538e77: 57                          	pushq	%rdi
140538e78: 53                          	pushq	%rbx
140538e79: 48 83 ec 28                 	subq	$0x28, %rsp
140538e7d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140538e84: 48 8d 8d 28 04 00 00        	leaq	0x428(%rbp), %rcx
140538e8b: e8 f0 09 af ff              	callq	0x140029880 <.text+0x28880>
140538e90: 90                          	nop
140538e91: 48 83 c4 28                 	addq	$0x28, %rsp
140538e95: 5b                          	popq	%rbx
140538e96: 5f                          	popq	%rdi
140538e97: 5e                          	popq	%rsi
140538e98: 5d                          	popq	%rbp
140538e99: c3                          	retq
140538e9a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140538ea0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140538ea5: 55                          	pushq	%rbp
140538ea6: 56                          	pushq	%rsi
140538ea7: 57                          	pushq	%rdi
140538ea8: 53                          	pushq	%rbx
140538ea9: 48 83 ec 28                 	subq	$0x28, %rsp
140538ead: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140538eb4: 48 8d 8d b8 03 00 00        	leaq	0x3b8(%rbp), %rcx
140538ebb: e8 c0 09 af ff              	callq	0x140029880 <.text+0x28880>
140538ec0: 90                          	nop
140538ec1: 48 83 c4 28                 	addq	$0x28, %rsp
140538ec5: 5b                          	popq	%rbx
140538ec6: 5f                          	popq	%rdi
140538ec7: 5e                          	popq	%rsi
140538ec8: 5d                          	popq	%rbp
140538ec9: c3                          	retq
140538eca: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140538ed0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140538ed5: 55                          	pushq	%rbp
140538ed6: 56                          	pushq	%rsi
140538ed7: 57                          	pushq	%rdi
140538ed8: 53                          	pushq	%rbx
140538ed9: 48 83 ec 28                 	subq	$0x28, %rsp
140538edd: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140538ee4: 48 8d 8d 48 03 00 00        	leaq	0x348(%rbp), %rcx
140538eeb: e8 90 09 af ff              	callq	0x140029880 <.text+0x28880>
140538ef0: 90                          	nop
140538ef1: 48 83 c4 28                 	addq	$0x28, %rsp
140538ef5: 5b                          	popq	%rbx
140538ef6: 5f                          	popq	%rdi
140538ef7: 5e                          	popq	%rsi
140538ef8: 5d                          	popq	%rbp
140538ef9: c3                          	retq
140538efa: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140538f00: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140538f05: 55                          	pushq	%rbp
140538f06: 56                          	pushq	%rsi
140538f07: 57                          	pushq	%rdi
140538f08: 53                          	pushq	%rbx
140538f09: 48 83 ec 28                 	subq	$0x28, %rsp
140538f0d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140538f14: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
140538f18: e8 63 09 af ff              	callq	0x140029880 <.text+0x28880>
140538f1d: 90                          	nop
140538f1e: 48 83 c4 28                 	addq	$0x28, %rsp
140538f22: 5b                          	popq	%rbx
140538f23: 5f                          	popq	%rdi
140538f24: 5e                          	popq	%rsi
140538f25: 5d                          	popq	%rbp
140538f26: c3                          	retq
