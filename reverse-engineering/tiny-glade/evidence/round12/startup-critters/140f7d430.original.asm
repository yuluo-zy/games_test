
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140f7d430: 55                          	pushq	%rbp
140f7d431: 56                          	pushq	%rsi
140f7d432: 57                          	pushq	%rdi
140f7d433: 53                          	pushq	%rbx
140f7d434: 48 81 ec e8 04 00 00        	subq	$0x4e8, %rsp            # imm = 0x4E8
140f7d43b: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140f7d443: 48 c7 85 60 04 00 00 fe ff ff ff    	movq	$-0x2, 0x460(%rbp)
140f7d44e: 48 89 ce                    	movq	%rcx, %rsi
140f7d451: 0f b6 05 b1 77 6b 02        	movzbl	0x26b77b1(%rip), %eax   # 0x143634c09
140f7d458: b9 e0 00 00 00              	movl	$0xe0, %ecx
140f7d45d: ba 08 00 00 00              	movl	$0x8, %edx
140f7d462: e8 a9 67 69 ff              	callq	0x140613c10 <.text+0x612c10>
140f7d467: 48 85 c0                    	testq	%rax, %rax
140f7d46a: 0f 84 38 04 00 00           	je	0x140f7d8a8 <.text+0xf7c8a8>
140f7d470: 48 89 85 58 04 00 00        	movq	%rax, 0x458(%rbp)
140f7d477: 0f 57 c0                    	xorps	%xmm0, %xmm0
140f7d47a: 0f 11 85 a8 00 00 00        	movups	%xmm0, 0xa8(%rbp)
140f7d481: 0f 11 85 c0 00 00 00        	movups	%xmm0, 0xc0(%rbp)
140f7d488: 0f 11 85 d8 00 00 00        	movups	%xmm0, 0xd8(%rbp)
140f7d48f: 0f 11 85 f0 00 00 00        	movups	%xmm0, 0xf0(%rbp)
140f7d496: 0f 11 85 08 01 00 00        	movups	%xmm0, 0x108(%rbp)
140f7d49d: c7 85 18 01 00 00 00 00 00 00       	movl	$0x0, 0x118(%rbp)
140f7d4a7: 0f 11 85 40 01 00 00        	movups	%xmm0, 0x140(%rbp)
140f7d4ae: 0f 11 85 58 01 00 00        	movups	%xmm0, 0x158(%rbp)
140f7d4b5: 0f 11 85 70 01 00 00        	movups	%xmm0, 0x170(%rbp)
140f7d4bc: 0f 11 85 88 01 00 00        	movups	%xmm0, 0x188(%rbp)
140f7d4c3: 0f 11 85 a0 01 00 00        	movups	%xmm0, 0x1a0(%rbp)
140f7d4ca: c7 85 b0 01 00 00 00 00 00 00       	movl	$0x0, 0x1b0(%rbp)
140f7d4d4: 48 c7 85 88 00 00 00 00 00 00 00    	movq	$0x0, 0x88(%rbp)
140f7d4df: 48 c7 85 90 00 00 00 08 00 00 00    	movq	$0x8, 0x90(%rbp)
140f7d4ea: 48 c7 85 98 00 00 00 00 00 00 00    	movq	$0x0, 0x98(%rbp)
140f7d4f5: 48 c7 85 a0 00 00 00 10 00 00 00    	movq	$0x10, 0xa0(%rbp)
140f7d500: 48 c7 85 b8 00 00 00 10 00 00 00    	movq	$0x10, 0xb8(%rbp)
140f7d50b: 48 c7 85 d0 00 00 00 10 00 00 00    	movq	$0x10, 0xd0(%rbp)
140f7d516: 48 c7 85 e8 00 00 00 10 00 00 00    	movq	$0x10, 0xe8(%rbp)
140f7d521: 48 c7 85 00 01 00 00 10 00 00 00    	movq	$0x10, 0x100(%rbp)
140f7d52c: 48 bb 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rbx # imm = 0x8000000000000000
140f7d536: 48 89 9d 20 01 00 00        	movq	%rbx, 0x120(%rbp)
140f7d53d: 48 8d 05 19 ec c2 01        	leaq	0x1c2ec19(%rip), %rax   # 0x142bac15d
140f7d544: 48 89 85 28 01 00 00        	movq	%rax, 0x128(%rbp)
140f7d54b: 48 c7 85 30 01 00 00 33 00 00 00    	movq	$0x33, 0x130(%rbp)
140f7d556: 48 c7 85 38 01 00 00 10 00 00 00    	movq	$0x10, 0x138(%rbp)
140f7d561: 48 c7 85 50 01 00 00 10 00 00 00    	movq	$0x10, 0x150(%rbp)
140f7d56c: 48 c7 85 68 01 00 00 10 00 00 00    	movq	$0x10, 0x168(%rbp)
140f7d577: 48 c7 85 80 01 00 00 10 00 00 00    	movq	$0x10, 0x180(%rbp)
140f7d582: 48 c7 85 98 01 00 00 10 00 00 00    	movq	$0x10, 0x198(%rbp)
140f7d58d: c7 85 b8 01 00 00 00 00 00 00       	movl	$0x0, 0x1b8(%rbp)
140f7d597: 66 c7 85 bc 01 00 00 01 00  	movw	$0x1, 0x1bc(%rbp)
140f7d5a0: 48 89 9d 00 02 00 00        	movq	%rbx, 0x200(%rbp)
140f7d5a7: c7 85 50 04 00 00 00 00 00 00       	movl	$0x0, 0x450(%rbp)
140f7d5b1: 0f b6 05 51 76 6b 02        	movzbl	0x26b7651(%rip), %eax   # 0x143634c09
140f7d5b8: b9 d0 03 00 00              	movl	$0x3d0, %ecx            # imm = 0x3D0
140f7d5bd: ba 08 00 00 00              	movl	$0x8, %edx
140f7d5c2: e8 49 66 69 ff              	callq	0x140613c10 <.text+0x612c10>
140f7d5c7: 48 85 c0                    	testq	%rax, %rax
140f7d5ca: 0f 84 e7 02 00 00           	je	0x140f7d8b7 <.text+0xf7c8b7>
140f7d5d0: 48 89 c7                    	movq	%rax, %rdi
140f7d5d3: 48 8d 95 88 00 00 00        	leaq	0x88(%rbp), %rdx
140f7d5da: 41 b8 d0 03 00 00           	movl	$0x3d0, %r8d            # imm = 0x3D0
140f7d5e0: 48 89 c1                    	movq	%rax, %rcx
140f7d5e3: e8 18 66 9a 01              	callq	0x142923c00 <.text+0x2922c00>
140f7d5e8: 4c 8d 05 71 e7 c2 01        	leaq	0x1c2e771(%rip), %r8    # 0x142babd60
140f7d5ef: 48 8d 4d 18                 	leaq	0x18(%rbp), %rcx
140f7d5f3: 48 89 fa                    	movq	%rdi, %rdx
140f7d5f6: e8 15 75 d6 ff              	callq	0x140ce4b10 <.text+0xce3b10>
140f7d5fb: 0f 57 c0                    	xorps	%xmm0, %xmm0
140f7d5fe: 0f 11 85 a8 00 00 00        	movups	%xmm0, 0xa8(%rbp)
140f7d605: 0f 11 85 c0 00 00 00        	movups	%xmm0, 0xc0(%rbp)
140f7d60c: 0f 11 85 d8 00 00 00        	movups	%xmm0, 0xd8(%rbp)
140f7d613: 0f 11 85 f0 00 00 00        	movups	%xmm0, 0xf0(%rbp)
140f7d61a: 0f 11 85 08 01 00 00        	movups	%xmm0, 0x108(%rbp)
140f7d621: c7 85 18 01 00 00 00 00 00 00       	movl	$0x0, 0x118(%rbp)
140f7d62b: 0f 11 85 40 01 00 00        	movups	%xmm0, 0x140(%rbp)
140f7d632: 0f 11 85 58 01 00 00        	movups	%xmm0, 0x158(%rbp)
140f7d639: 0f 11 85 70 01 00 00        	movups	%xmm0, 0x170(%rbp)
140f7d640: 0f 11 85 88 01 00 00        	movups	%xmm0, 0x188(%rbp)
140f7d647: 0f 11 85 a0 01 00 00        	movups	%xmm0, 0x1a0(%rbp)
140f7d64e: c7 85 b0 01 00 00 00 00 00 00       	movl	$0x0, 0x1b0(%rbp)
140f7d658: 48 c7 85 88 00 00 00 00 00 00 00    	movq	$0x0, 0x88(%rbp)
140f7d663: 48 c7 85 90 00 00 00 08 00 00 00    	movq	$0x8, 0x90(%rbp)
140f7d66e: 48 c7 85 98 00 00 00 00 00 00 00    	movq	$0x0, 0x98(%rbp)
140f7d679: 48 c7 85 a0 00 00 00 10 00 00 00    	movq	$0x10, 0xa0(%rbp)
140f7d684: 48 c7 85 b8 00 00 00 10 00 00 00    	movq	$0x10, 0xb8(%rbp)
140f7d68f: 48 c7 85 d0 00 00 00 10 00 00 00    	movq	$0x10, 0xd0(%rbp)
140f7d69a: 48 c7 85 e8 00 00 00 10 00 00 00    	movq	$0x10, 0xe8(%rbp)
140f7d6a5: 48 c7 85 00 01 00 00 10 00 00 00    	movq	$0x10, 0x100(%rbp)
140f7d6b0: 48 89 9d 20 01 00 00        	movq	%rbx, 0x120(%rbp)
140f7d6b7: 48 8d 05 d2 ea c2 01        	leaq	0x1c2ead2(%rip), %rax   # 0x142bac190
140f7d6be: 48 89 85 28 01 00 00        	movq	%rax, 0x128(%rbp)
140f7d6c5: 48 c7 85 30 01 00 00 32 00 00 00    	movq	$0x32, 0x130(%rbp)
140f7d6d0: 48 c7 85 38 01 00 00 10 00 00 00    	movq	$0x10, 0x138(%rbp)
140f7d6db: 48 c7 85 50 01 00 00 10 00 00 00    	movq	$0x10, 0x150(%rbp)
140f7d6e6: 48 c7 85 68 01 00 00 10 00 00 00    	movq	$0x10, 0x168(%rbp)
140f7d6f1: 48 c7 85 80 01 00 00 10 00 00 00    	movq	$0x10, 0x180(%rbp)
140f7d6fc: 48 c7 85 98 01 00 00 10 00 00 00    	movq	$0x10, 0x198(%rbp)
140f7d707: c7 85 b8 01 00 00 00 00 00 00       	movl	$0x0, 0x1b8(%rbp)
140f7d711: 66 c7 85 bc 01 00 00 01 00  	movw	$0x1, 0x1bc(%rbp)
140f7d71a: 48 89 9d b8 02 00 00        	movq	%rbx, 0x2b8(%rbp)
140f7d721: c7 85 08 03 00 00 00 00 00 00       	movl	$0x0, 0x308(%rbp)
140f7d72b: 0f b6 05 d7 74 6b 02        	movzbl	0x26b74d7(%rip), %eax   # 0x143634c09
140f7d732: b9 88 02 00 00              	movl	$0x288, %ecx            # imm = 0x288
140f7d737: ba 08 00 00 00              	movl	$0x8, %edx
140f7d73c: e8 cf 64 69 ff              	callq	0x140613c10 <.text+0x612c10>
140f7d741: 48 85 c0                    	testq	%rax, %rax
140f7d744: 0f 84 7e 01 00 00           	je	0x140f7d8c8 <.text+0xf7c8c8>
140f7d74a: 48 89 c7                    	movq	%rax, %rdi
140f7d74d: 48 8d 95 88 00 00 00        	leaq	0x88(%rbp), %rdx
140f7d754: 41 b8 88 02 00 00           	movl	$0x288, %r8d            # imm = 0x288
140f7d75a: 48 89 c1                    	movq	%rax, %rcx
140f7d75d: e8 9e 64 9a 01              	callq	0x142923c00 <.text+0x2922c00>
140f7d762: 4c 8d 05 af e6 c2 01        	leaq	0x1c2e6af(%rip), %r8    # 0x142babe18
140f7d769: 48 8d 4d a8                 	leaq	-0x58(%rbp), %rcx
140f7d76d: 48 89 fa                    	movq	%rdi, %rdx
140f7d770: e8 9b 73 d6 ff              	callq	0x140ce4b10 <.text+0xce3b10>
140f7d775: 0f 10 45 78                 	movups	0x78(%rbp), %xmm0
140f7d779: 48 8b 85 58 04 00 00        	movq	0x458(%rbp), %rax
140f7d780: 0f 11 40 60                 	movups	%xmm0, 0x60(%rax)
140f7d784: 0f 10 45 68                 	movups	0x68(%rbp), %xmm0
140f7d788: 0f 11 40 50                 	movups	%xmm0, 0x50(%rax)
140f7d78c: 0f 10 45 58                 	movups	0x58(%rbp), %xmm0
140f7d790: 0f 11 40 40                 	movups	%xmm0, 0x40(%rax)
140f7d794: 0f 10 45 18                 	movups	0x18(%rbp), %xmm0
140f7d798: 0f 10 4d 28                 	movups	0x28(%rbp), %xmm1
140f7d79c: 0f 10 55 38                 	movups	0x38(%rbp), %xmm2
140f7d7a0: 0f 10 5d 48                 	movups	0x48(%rbp), %xmm3
140f7d7a4: 0f 11 58 30                 	movups	%xmm3, 0x30(%rax)
140f7d7a8: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
140f7d7ac: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
140f7d7b0: 0f 11 00                    	movups	%xmm0, (%rax)
140f7d7b3: 0f 10 45 a8                 	movups	-0x58(%rbp), %xmm0
140f7d7b7: 0f 10 4d b8                 	movups	-0x48(%rbp), %xmm1
140f7d7bb: 0f 10 55 c8                 	movups	-0x38(%rbp), %xmm2
140f7d7bf: 0f 10 5d d8                 	movups	-0x28(%rbp), %xmm3
140f7d7c3: 0f 11 40 70                 	movups	%xmm0, 0x70(%rax)
140f7d7c7: 0f 11 88 80 00 00 00        	movups	%xmm1, 0x80(%rax)
140f7d7ce: 0f 11 90 90 00 00 00        	movups	%xmm2, 0x90(%rax)
140f7d7d5: 0f 11 98 a0 00 00 00        	movups	%xmm3, 0xa0(%rax)
140f7d7dc: 0f 10 45 e8                 	movups	-0x18(%rbp), %xmm0
140f7d7e0: 0f 11 80 b0 00 00 00        	movups	%xmm0, 0xb0(%rax)
140f7d7e7: 0f 10 45 f8                 	movups	-0x8(%rbp), %xmm0
140f7d7eb: 0f 11 80 c0 00 00 00        	movups	%xmm0, 0xc0(%rax)
140f7d7f2: 0f 10 45 08                 	movups	0x8(%rbp), %xmm0
140f7d7f6: 0f 11 80 d0 00 00 00        	movups	%xmm0, 0xd0(%rax)
140f7d7fd: 48 c7 85 90 00 00 00 02 00 00 00    	movq	$0x2, 0x90(%rbp)
140f7d808: 48 89 85 98 00 00 00        	movq	%rax, 0x98(%rbp)
140f7d80f: 48 c7 85 a0 00 00 00 02 00 00 00    	movq	$0x2, 0xa0(%rbp)
140f7d81a: 48 c7 85 a8 00 00 00 00 00 00 00    	movq	$0x0, 0xa8(%rbp)
140f7d825: 48 c7 85 b0 00 00 00 08 00 00 00    	movq	$0x8, 0xb0(%rbp)
140f7d830: 0f 57 c0                    	xorps	%xmm0, %xmm0
140f7d833: 0f 11 85 b8 00 00 00        	movups	%xmm0, 0xb8(%rbp)
140f7d83a: 48 89 9d 88 00 00 00        	movq	%rbx, 0x88(%rbp)
140f7d841: 48 8d 8d c0 00 00 00        	leaq	0xc0(%rbp), %rcx
140f7d848: e8 03 45 00 00              	callq	0x140f81d50 <.text+0xf80d50>
140f7d84d: 0f 10 85 e8 00 00 00        	movups	0xe8(%rbp), %xmm0
140f7d854: 0f 11 46 60                 	movups	%xmm0, 0x60(%rsi)
140f7d858: 0f 10 85 d8 00 00 00        	movups	0xd8(%rbp), %xmm0
140f7d85f: 0f 11 46 50                 	movups	%xmm0, 0x50(%rsi)
140f7d863: 0f 10 85 c8 00 00 00        	movups	0xc8(%rbp), %xmm0
140f7d86a: 0f 11 46 40                 	movups	%xmm0, 0x40(%rsi)
140f7d86e: 0f 10 85 88 00 00 00        	movups	0x88(%rbp), %xmm0
140f7d875: 0f 10 8d 98 00 00 00        	movups	0x98(%rbp), %xmm1
140f7d87c: 0f 10 95 a8 00 00 00        	movups	0xa8(%rbp), %xmm2
140f7d883: 0f 10 9d b8 00 00 00        	movups	0xb8(%rbp), %xmm3
140f7d88a: 0f 11 5e 30                 	movups	%xmm3, 0x30(%rsi)
140f7d88e: 0f 11 56 20                 	movups	%xmm2, 0x20(%rsi)
140f7d892: 0f 11 4e 10                 	movups	%xmm1, 0x10(%rsi)
140f7d896: 0f 11 06                    	movups	%xmm0, (%rsi)
140f7d899: 48 89 f0                    	movq	%rsi, %rax
140f7d89c: 48 81 c4 e8 04 00 00        	addq	$0x4e8, %rsp            # imm = 0x4E8
140f7d8a3: 5b                          	popq	%rbx
140f7d8a4: 5f                          	popq	%rdi
140f7d8a5: 5e                          	popq	%rsi
140f7d8a6: 5d                          	popq	%rbp
140f7d8a7: c3                          	retq
140f7d8a8: b9 08 00 00 00              	movl	$0x8, %ecx
140f7d8ad: ba e0 00 00 00              	movl	$0xe0, %edx
140f7d8b2: e8 49 b7 95 01              	callq	0x1428d9000 <.text+0x28d8000>
140f7d8b7: b9 08 00 00 00              	movl	$0x8, %ecx
140f7d8bc: ba d0 03 00 00              	movl	$0x3d0, %edx            # imm = 0x3D0
140f7d8c1: e8 3a b7 95 01              	callq	0x1428d9000 <.text+0x28d8000>
140f7d8c6: eb 0f                       	jmp	0x140f7d8d7 <.text+0xf7c8d7>
140f7d8c8: b9 08 00 00 00              	movl	$0x8, %ecx
140f7d8cd: ba 88 02 00 00              	movl	$0x288, %edx            # imm = 0x288
140f7d8d2: e8 29 b7 95 01              	callq	0x1428d9000 <.text+0x28d8000>
140f7d8d7: 0f 0b                       	ud2
140f7d8d9: 0f 1f 80 00 00 00 00        	nopl	(%rax)
140f7d8e0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140f7d8e5: 55                          	pushq	%rbp
140f7d8e6: 56                          	pushq	%rsi
140f7d8e7: 57                          	pushq	%rdi
140f7d8e8: 53                          	pushq	%rbx
140f7d8e9: 48 83 ec 28                 	subq	$0x28, %rsp
140f7d8ed: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140f7d8f4: 48 8d 8d 88 00 00 00        	leaq	0x88(%rbp), %rcx
140f7d8fb: e8 50 45 e0 00              	callq	0x141d81e50 <.text+0x1d80e50>
140f7d900: 90                          	nop
140f7d901: 48 83 c4 28                 	addq	$0x28, %rsp
140f7d905: 5b                          	popq	%rbx
140f7d906: 5f                          	popq	%rdi
140f7d907: 5e                          	popq	%rsi
140f7d908: 5d                          	popq	%rbp
140f7d909: c3                          	retq
140f7d90a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140f7d910: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140f7d915: 55                          	pushq	%rbp
140f7d916: 56                          	pushq	%rsi
140f7d917: 57                          	pushq	%rdi
140f7d918: 53                          	pushq	%rbx
140f7d919: 48 83 ec 28                 	subq	$0x28, %rsp
140f7d91d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140f7d924: ba e0 00 00 00              	movl	$0xe0, %edx
140f7d929: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140f7d92f: 48 8b 8d 58 04 00 00        	movq	0x458(%rbp), %rcx
140f7d936: e8 e5 62 69 ff              	callq	0x140613c20 <.text+0x612c20>
140f7d93b: 90                          	nop
140f7d93c: 48 83 c4 28                 	addq	$0x28, %rsp
140f7d940: 5b                          	popq	%rbx
140f7d941: 5f                          	popq	%rdi
140f7d942: 5e                          	popq	%rsi
140f7d943: 5d                          	popq	%rbp
140f7d944: c3                          	retq
140f7d945: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
140f7d950: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140f7d955: 55                          	pushq	%rbp
140f7d956: 56                          	pushq	%rsi
140f7d957: 57                          	pushq	%rdi
140f7d958: 53                          	pushq	%rbx
140f7d959: 48 83 ec 28                 	subq	$0x28, %rsp
140f7d95d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140f7d964: 48 8d 8d 88 00 00 00        	leaq	0x88(%rbp), %rcx
140f7d96b: e8 40 1a 65 ff              	callq	0x1405cf3b0 <.text+0x5ce3b0>
140f7d970: 90                          	nop
140f7d971: 48 83 c4 28                 	addq	$0x28, %rsp
140f7d975: 5b                          	popq	%rbx
140f7d976: 5f                          	popq	%rdi
140f7d977: 5e                          	popq	%rsi
140f7d978: 5d                          	popq	%rbp
140f7d979: c3                          	retq
140f7d97a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140f7d980: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140f7d985: 55                          	pushq	%rbp
140f7d986: 56                          	pushq	%rsi
140f7d987: 57                          	pushq	%rdi
140f7d988: 53                          	pushq	%rbx
140f7d989: 48 83 ec 28                 	subq	$0x28, %rsp
140f7d98d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140f7d994: 48 8d 4d 18                 	leaq	0x18(%rbp), %rcx
140f7d998: e8 e3 be 0a ff              	callq	0x140029880 <.text+0x28880>
140f7d99d: 90                          	nop
140f7d99e: 48 83 c4 28                 	addq	$0x28, %rsp
140f7d9a2: 5b                          	popq	%rbx
140f7d9a3: 5f                          	popq	%rdi
140f7d9a4: 5e                          	popq	%rsi
140f7d9a5: 5d                          	popq	%rbp
140f7d9a6: c3                          	retq
140f7d9a7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
140f7d9b0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140f7d9b5: 55                          	pushq	%rbp
140f7d9b6: 56                          	pushq	%rsi
140f7d9b7: 57                          	pushq	%rdi
140f7d9b8: 53                          	pushq	%rbx
140f7d9b9: 48 83 ec 28                 	subq	$0x28, %rsp
140f7d9bd: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140f7d9c4: 48 8d 8d 88 00 00 00        	leaq	0x88(%rbp), %rcx
140f7d9cb: e8 b0 be 0a ff              	callq	0x140029880 <.text+0x28880>
140f7d9d0: 90                          	nop
140f7d9d1: 48 83 c4 28                 	addq	$0x28, %rsp
140f7d9d5: 5b                          	popq	%rbx
140f7d9d6: 5f                          	popq	%rdi
140f7d9d7: 5e                          	popq	%rsi
140f7d9d8: 5d                          	popq	%rbp
140f7d9d9: c3                          	retq
