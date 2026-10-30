
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14099ea30: 55                          	pushq	%rbp
14099ea31: 41 56                       	pushq	%r14
14099ea33: 56                          	pushq	%rsi
14099ea34: 57                          	pushq	%rdi
14099ea35: 53                          	pushq	%rbx
14099ea36: 48 81 ec 10 01 00 00        	subq	$0x110, %rsp            # imm = 0x110
14099ea3d: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
14099ea45: 0f 29 b5 80 00 00 00        	movaps	%xmm6, 0x80(%rbp)
14099ea4c: 48 c7 45 78 fe ff ff ff     	movq	$-0x2, 0x78(%rbp)
14099ea54: 4c 89 cb                    	movq	%r9, %rbx
14099ea57: 4d 89 c6                    	movq	%r8, %r14
14099ea5a: 48 89 d7                    	movq	%rdx, %rdi
14099ea5d: 48 89 ce                    	movq	%rcx, %rsi
14099ea60: 0f b6 05 a2 61 c9 02        	movzbl	0x2c961a2(%rip), %eax   # 0x143634c09
14099ea67: b9 08 00 00 00              	movl	$0x8, %ecx
14099ea6c: ba 08 00 00 00              	movl	$0x8, %edx
14099ea71: e8 9a 51 c7 ff              	callq	0x140613c10 <.text+0x612c10>
14099ea76: 48 85 c0                    	testq	%rax, %rax
14099ea79: 0f 84 7f 01 00 00           	je	0x14099ebfe <.text+0x99dbfe>
14099ea7f: 48 89 45 70                 	movq	%rax, 0x70(%rbp)
14099ea83: 48 b8 01 00 00 00 00 00 00 80       	movabsq	$-0x7fffffffffffffff, %rax # imm = 0x8000000000000001
14099ea8d: 48 89 45 b0                 	movq	%rax, -0x50(%rbp)
14099ea91: 0f b6 05 71 61 c9 02        	movzbl	0x2c96171(%rip), %eax   # 0x143634c09
14099ea98: b9 13 00 00 00              	movl	$0x13, %ecx
14099ea9d: ba 01 00 00 00              	movl	$0x1, %edx
14099eaa2: e8 69 51 c7 ff              	callq	0x140613c10 <.text+0x612c10>
14099eaa7: 48 85 c0                    	testq	%rax, %rax
14099eaaa: 0f 84 5d 01 00 00           	je	0x14099ec0d <.text+0x99dc0d>
14099eab0: 48 8b 8d e0 00 00 00        	movq	0xe0(%rbp), %rcx
14099eab7: 8b 15 de a9 0f 02           	movl	0x20fa9de(%rip), %edx   # 0x142a9949b
14099eabd: 89 50 0f                    	movl	%edx, 0xf(%rax)
14099eac0: 0f 10 05 c5 a9 0f 02        	movups	0x20fa9c5(%rip), %xmm0  # 0x142a9948c
14099eac7: 0f 11 00                    	movups	%xmm0, (%rax)
14099eaca: 0f 10 45 b0                 	movups	-0x50(%rbp), %xmm0
14099eace: 0f 10 4d c0                 	movups	-0x40(%rbp), %xmm1
14099ead2: 0f 10 55 d0                 	movups	-0x30(%rbp), %xmm2
14099ead6: 0f 29 45 f0                 	movaps	%xmm0, -0x10(%rbp)
14099eada: 0f 29 4d 00                 	movaps	%xmm1, (%rbp)
14099eade: 0f 29 55 10                 	movaps	%xmm2, 0x10(%rbp)
14099eae2: 48 c7 45 5c 00 00 00 00     	movq	$0x0, 0x5c(%rbp)
14099eaea: 48 c7 45 61 00 00 00 00     	movq	$0x0, 0x61(%rbp)
14099eaf2: 0f 28 05 c7 f5 f9 01        	movaps	0x1f9f5c7(%rip), %xmm0  # 0x14293e0c0
14099eaf9: 0f 29 45 e0                 	movaps	%xmm0, -0x20(%rbp)
14099eafd: 66 c7 45 20 00 01           	movw	$0x100, 0x20(%rbp)      # imm = 0x100
14099eb03: 48 ba 00 00 80 3f 01 00 00 00       	movabsq	$0x13f800000, %rdx # imm = 0x13F800000
14099eb0d: 48 89 55 24                 	movq	%rdx, 0x24(%rbp)
14099eb11: c7 45 30 00 00 00 00        	movl	$0x0, 0x30(%rbp)
14099eb18: c7 45 38 00 00 00 00        	movl	$0x0, 0x38(%rbp)
14099eb1f: 48 c7 45 40 13 00 00 00     	movq	$0x13, 0x40(%rbp)
14099eb27: 48 89 45 48                 	movq	%rax, 0x48(%rbp)
14099eb2b: 48 c7 45 50 13 00 00 00     	movq	$0x13, 0x50(%rbp)
14099eb33: c7 45 58 00 00 00 00        	movl	$0x0, 0x58(%rbp)
14099eb3a: 66 c7 45 69 00 00           	movw	$0x0, 0x69(%rbp)
14099eb40: 48 89 4c 24 20              	movq	%rcx, 0x20(%rsp)
14099eb45: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
14099eb49: 48 8d 55 e0                 	leaq	-0x20(%rbp), %rdx
14099eb4d: 4d 89 f0                    	movq	%r14, %r8
14099eb50: 49 89 d9                    	movq	%rbx, %r9
14099eb53: e8 f8 f1 ef ff              	callq	0x14089dd50 <.text+0x89cd50>
14099eb58: f3 0f 10 77 0c              	movss	0xc(%rdi), %xmm6
14099eb5d: 0f b6 05 a5 60 c9 02        	movzbl	0x2c960a5(%rip), %eax   # 0x143634c09
14099eb64: b9 04 00 00 00              	movl	$0x4, %ecx
14099eb69: ba 01 00 00 00              	movl	$0x1, %edx
14099eb6e: e8 9d 50 c7 ff              	callq	0x140613c10 <.text+0x612c10>
14099eb73: 48 85 c0                    	testq	%rax, %rax
14099eb76: 0f 84 a9 00 00 00           	je	0x14099ec25 <.text+0x99dc25>
14099eb7c: c7 00 6c 69 6e 6b           	movl	$0x6b6e696c, (%rax)     # imm = 0x6B6E696C
14099eb82: f3 0f 11 75 e0              	movss	%xmm6, -0x20(%rbp)
14099eb87: f3 0f 11 75 e4              	movss	%xmm6, -0x1c(%rbp)
14099eb8c: 48 8b 0f                    	movq	(%rdi), %rcx
14099eb8f: 48 89 4d e8                 	movq	%rcx, -0x18(%rbp)
14099eb93: 8b 4f 08                    	movl	0x8(%rdi), %ecx
14099eb96: 89 4d f0                    	movl	%ecx, -0x10(%rbp)
14099eb99: 48 c7 45 f8 04 00 00 00     	movq	$0x4, -0x8(%rbp)
14099eba1: 48 89 45 00                 	movq	%rax, (%rbp)
14099eba5: 48 c7 45 08 04 00 00 00     	movq	$0x4, 0x8(%rbp)
14099ebad: c7 45 10 00 00 00 00        	movl	$0x0, 0x10(%rbp)
14099ebb4: 4c 8d 05 dd d0 11 02        	leaq	0x211d0dd(%rip), %r8    # 0x142abbc98
14099ebbb: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
14099ebbf: 48 8d 55 e0                 	leaq	-0x20(%rbp), %rdx
14099ebc3: e8 58 fa 93 00              	callq	0x1412de620 <.text+0x12dd620>
14099ebc8: 48 8b 45 d0                 	movq	-0x30(%rbp), %rax
14099ebcc: 48 8b 4d 70                 	movq	0x70(%rbp), %rcx
14099ebd0: 48 89 01                    	movq	%rax, (%rcx)
14099ebd3: 48 c7 06 01 00 00 00        	movq	$0x1, (%rsi)
14099ebda: 48 89 4e 08                 	movq	%rcx, 0x8(%rsi)
14099ebde: 48 c7 46 10 01 00 00 00     	movq	$0x1, 0x10(%rsi)
14099ebe6: 48 89 f0                    	movq	%rsi, %rax
14099ebe9: 0f 28 b5 80 00 00 00        	movaps	0x80(%rbp), %xmm6
14099ebf0: 48 81 c4 10 01 00 00        	addq	$0x110, %rsp            # imm = 0x110
14099ebf7: 5b                          	popq	%rbx
14099ebf8: 5f                          	popq	%rdi
14099ebf9: 5e                          	popq	%rsi
14099ebfa: 41 5e                       	popq	%r14
14099ebfc: 5d                          	popq	%rbp
14099ebfd: c3                          	retq
14099ebfe: b9 08 00 00 00              	movl	$0x8, %ecx
14099ec03: ba 08 00 00 00              	movl	$0x8, %edx
14099ec08: e8 f3 a3 f3 01              	callq	0x1428d9000 <.text+0x28d8000>
14099ec0d: 4c 8d 05 0c a3 0f 02        	leaq	0x20fa30c(%rip), %r8    # 0x142a98f20
14099ec14: b9 01 00 00 00              	movl	$0x1, %ecx
14099ec19: ba 13 00 00 00              	movl	$0x13, %edx
14099ec1e: e8 c0 a3 f3 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
14099ec23: eb 16                       	jmp	0x14099ec3b <.text+0x99dc3b>
14099ec25: 4c 8d 05 a4 00 2f 02        	leaq	0x22f00a4(%rip), %r8    # 0x142c8ecd0
14099ec2c: b9 01 00 00 00              	movl	$0x1, %ecx
14099ec31: ba 04 00 00 00              	movl	$0x4, %edx
14099ec36: e8 a8 a3 f3 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
14099ec3b: 0f 0b                       	ud2
14099ec3d: 0f 1f 00                    	nopl	(%rax)
14099ec40: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14099ec45: 55                          	pushq	%rbp
14099ec46: 41 56                       	pushq	%r14
14099ec48: 56                          	pushq	%rsi
14099ec49: 57                          	pushq	%rdi
14099ec4a: 53                          	pushq	%rbx
14099ec4b: 48 83 ec 40                 	subq	$0x40, %rsp
14099ec4f: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14099ec56: 0f 29 74 24 20              	movaps	%xmm6, 0x20(%rsp)
14099ec5b: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
14099ec5f: e8 3c c1 ef ff              	callq	0x14089ada0 <.text+0x899da0>
14099ec64: 0f 28 74 24 20              	movaps	0x20(%rsp), %xmm6
14099ec69: 48 83 c4 40                 	addq	$0x40, %rsp
14099ec6d: 5b                          	popq	%rbx
14099ec6e: 5f                          	popq	%rdi
14099ec6f: 5e                          	popq	%rsi
14099ec70: 41 5e                       	popq	%r14
14099ec72: 5d                          	popq	%rbp
14099ec73: c3                          	retq
14099ec74: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
14099ec80: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14099ec85: 55                          	pushq	%rbp
14099ec86: 41 56                       	pushq	%r14
14099ec88: 56                          	pushq	%rsi
14099ec89: 57                          	pushq	%rdi
14099ec8a: 53                          	pushq	%rbx
14099ec8b: 48 83 ec 40                 	subq	$0x40, %rsp
14099ec8f: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14099ec96: 0f 29 74 24 20              	movaps	%xmm6, 0x20(%rsp)
14099ec9b: ba 08 00 00 00              	movl	$0x8, %edx
14099eca0: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14099eca6: 48 8b 4d 70                 	movq	0x70(%rbp), %rcx
14099ecaa: e8 71 4f c7 ff              	callq	0x140613c20 <.text+0x612c20>
14099ecaf: 0f 28 74 24 20              	movaps	0x20(%rsp), %xmm6
14099ecb4: 48 83 c4 40                 	addq	$0x40, %rsp
14099ecb8: 5b                          	popq	%rbx
14099ecb9: 5f                          	popq	%rdi
14099ecba: 5e                          	popq	%rsi
14099ecbb: 41 5e                       	popq	%r14
14099ecbd: 5d                          	popq	%rbp
14099ecbe: c3                          	retq
14099ecbf: cc                          	int3
14099ecc0: 41 56                       	pushq	%r14
14099ecc2: 56                          	pushq	%rsi
14099ecc3: 57                          	pushq	%rdi
14099ecc4: 53                          	pushq	%rbx
14099ecc5: 48 83 ec 68                 	subq	$0x68, %rsp
14099ecc9: 48 89 d7                    	movq	%rdx, %rdi
14099eccc: 48 89 ce                    	movq	%rcx, %rsi
14099eccf: 83 39 01                    	cmpl	$0x1, (%rcx)
14099ecd2: 75 60                       	jne	0x14099ed34 <.text+0x99dd34>
14099ecd4: 48 8b 87 a0 05 00 00        	movq	0x5a0(%rdi), %rax
14099ecdb: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
14099ece0: 48 39 46 18                 	cmpq	%rax, 0x18(%rsi)
14099ece4: 0f 84 81 00 00 00           	je	0x14099ed6b <.text+0x99dd6b>
14099ecea: 48 8d 05 27 d7 11 02        	leaq	0x211d727(%rip), %rax   # 0x142abc418
14099ecf1: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
14099ecf6: 48 c7 44 24 40 01 00 00 00  	movq	$0x1, 0x40(%rsp)
14099ecff: 48 c7 44 24 48 08 00 00 00  	movq	$0x8, 0x48(%rsp)
14099ed08: 0f 57 c0                    	xorps	%xmm0, %xmm0
14099ed0b: 0f 11 44 24 50              	movups	%xmm0, 0x50(%rsp)
14099ed10: 48 83 c6 18                 	addq	$0x18, %rsi
14099ed14: 48 8d 05 7d d7 11 02        	leaq	0x211d77d(%rip), %rax   # 0x142abc498
14099ed1b: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14099ed20: 4c 8d 44 24 30              	leaq	0x30(%rsp), %r8
14099ed25: 4c 8d 4c 24 38              	leaq	0x38(%rsp), %r9
14099ed2a: 31 c9                       	xorl	%ecx, %ecx
14099ed2c: 48 89 f2                    	movq	%rsi, %rdx
14099ed2f: e8 ad f3 f2 01              	callq	0x1428ce0e1 <.text+0x28cd0e1>
14099ed34: 48 8d 5e 20                 	leaq	0x20(%rsi), %rbx
14099ed38: 48 89 f9                    	movq	%rdi, %rcx
14099ed3b: 48 89 da                    	movq	%rbx, %rdx
14099ed3e: e8 fd 56 9a 00              	callq	0x141344440 <.text+0x1343440>
14099ed43: 49 89 c6                    	movq	%rax, %r14
14099ed46: 48 89 f9                    	movq	%rdi, %rcx
14099ed49: 48 89 da                    	movq	%rbx, %rdx
14099ed4c: e8 bf 68 9c 00              	callq	0x141365610 <.text+0x1364610>
