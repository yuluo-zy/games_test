
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1400c5cc0: 55                          	pushq	%rbp
1400c5cc1: 56                          	pushq	%rsi
1400c5cc2: 57                          	pushq	%rdi
1400c5cc3: 53                          	pushq	%rbx
1400c5cc4: 48 81 ec 58 03 00 00        	subq	$0x358, %rsp            # imm = 0x358
1400c5ccb: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1400c5cd3: 48 c7 85 d0 02 00 00 fe ff ff ff    	movq	$-0x2, 0x2d0(%rbp)
1400c5cde: 48 89 ce                    	movq	%rcx, %rsi
1400c5ce1: 0f b6 05 21 ef 56 03        	movzbl	0x356ef21(%rip), %eax   # 0x143634c09
1400c5ce8: b9 30 02 00 00              	movl	$0x230, %ecx            # imm = 0x230
1400c5ced: ba 08 00 00 00              	movl	$0x8, %edx
1400c5cf2: e8 19 df 54 00              	callq	0x140613c10 <.text+0x612c10>
1400c5cf7: 48 85 c0                    	testq	%rax, %rax
1400c5cfa: 0f 84 a7 03 00 00           	je	0x1400c60a7 <.text+0xc50a7>
1400c5d00: 48 89 85 c8 02 00 00        	movq	%rax, 0x2c8(%rbp)
1400c5d07: 0f 57 c0                    	xorps	%xmm0, %xmm0
1400c5d0a: 0f 11 85 a8 01 00 00        	movups	%xmm0, 0x1a8(%rbp)
1400c5d11: 0f 11 85 c0 01 00 00        	movups	%xmm0, 0x1c0(%rbp)
1400c5d18: 0f 11 85 d8 01 00 00        	movups	%xmm0, 0x1d8(%rbp)
1400c5d1f: 0f 11 85 f0 01 00 00        	movups	%xmm0, 0x1f0(%rbp)
1400c5d26: 0f 11 85 08 02 00 00        	movups	%xmm0, 0x208(%rbp)
1400c5d2d: c7 85 18 02 00 00 00 00 00 00       	movl	$0x0, 0x218(%rbp)
1400c5d37: 0f 11 85 40 02 00 00        	movups	%xmm0, 0x240(%rbp)
1400c5d3e: 0f 11 85 58 02 00 00        	movups	%xmm0, 0x258(%rbp)
1400c5d45: 0f 11 85 70 02 00 00        	movups	%xmm0, 0x270(%rbp)
1400c5d4c: 0f 11 85 88 02 00 00        	movups	%xmm0, 0x288(%rbp)
1400c5d53: 0f 11 85 a0 02 00 00        	movups	%xmm0, 0x2a0(%rbp)
1400c5d5a: c7 85 b0 02 00 00 00 00 00 00       	movl	$0x0, 0x2b0(%rbp)
1400c5d64: 48 c7 85 60 01 00 00 00 00 00 00    	movq	$0x0, 0x160(%rbp)
1400c5d6f: 48 c7 85 88 01 00 00 00 00 00 00    	movq	$0x0, 0x188(%rbp)
1400c5d7a: 48 c7 85 90 01 00 00 08 00 00 00    	movq	$0x8, 0x190(%rbp)
1400c5d85: 48 c7 85 98 01 00 00 00 00 00 00    	movq	$0x0, 0x198(%rbp)
1400c5d90: 48 c7 85 a0 01 00 00 10 00 00 00    	movq	$0x10, 0x1a0(%rbp)
1400c5d9b: 48 c7 85 b8 01 00 00 10 00 00 00    	movq	$0x10, 0x1b8(%rbp)
1400c5da6: 48 c7 85 d0 01 00 00 10 00 00 00    	movq	$0x10, 0x1d0(%rbp)
1400c5db1: 48 c7 85 e8 01 00 00 10 00 00 00    	movq	$0x10, 0x1e8(%rbp)
1400c5dbc: 48 c7 85 00 02 00 00 10 00 00 00    	movq	$0x10, 0x200(%rbp)
1400c5dc7: 48 bb 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rbx # imm = 0x8000000000000000
1400c5dd1: 48 89 9d 20 02 00 00        	movq	%rbx, 0x220(%rbp)
1400c5dd8: 48 8d 05 b3 02 87 02        	leaq	0x28702b3(%rip), %rax   # 0x142936092
1400c5ddf: 48 89 85 28 02 00 00        	movq	%rax, 0x228(%rbp)
1400c5de6: 48 c7 85 30 02 00 00 42 00 00 00    	movq	$0x42, 0x230(%rbp)
1400c5df1: 48 c7 85 38 02 00 00 10 00 00 00    	movq	$0x10, 0x238(%rbp)
1400c5dfc: 48 c7 85 50 02 00 00 10 00 00 00    	movq	$0x10, 0x250(%rbp)
1400c5e07: 48 c7 85 68 02 00 00 10 00 00 00    	movq	$0x10, 0x268(%rbp)
1400c5e12: 48 c7 85 80 02 00 00 10 00 00 00    	movq	$0x10, 0x280(%rbp)
1400c5e1d: 48 c7 85 98 02 00 00 10 00 00 00    	movq	$0x10, 0x298(%rbp)
1400c5e28: c7 85 b8 02 00 00 00 00 00 00       	movl	$0x0, 0x2b8(%rbp)
1400c5e32: 66 c7 85 bc 02 00 00 01 00  	movw	$0x1, 0x2bc(%rbp)
1400c5e3b: c7 85 c0 02 00 00 00 00 00 00       	movl	$0x0, 0x2c0(%rbp)
1400c5e45: 0f b6 05 bd ed 56 03        	movzbl	0x356edbd(%rip), %eax   # 0x143634c09
1400c5e4c: b9 68 01 00 00              	movl	$0x168, %ecx            # imm = 0x168
1400c5e51: ba 08 00 00 00              	movl	$0x8, %edx
1400c5e56: e8 b5 dd 54 00              	callq	0x140613c10 <.text+0x612c10>
1400c5e5b: 48 85 c0                    	testq	%rax, %rax
1400c5e5e: 0f 84 52 02 00 00           	je	0x1400c60b6 <.text+0xc50b6>
1400c5e64: 48 89 c7                    	movq	%rax, %rdi
1400c5e67: 48 8d 95 60 01 00 00        	leaq	0x160(%rbp), %rdx
1400c5e6e: 41 b8 68 01 00 00           	movl	$0x168, %r8d            # imm = 0x168
1400c5e74: 48 89 c1                    	movq	%rax, %rcx
1400c5e77: e8 84 dd 85 02              	callq	0x142923c00 <.text+0x2922c00>
1400c5e7c: 4c 8d 05 75 00 87 02        	leaq	0x2870075(%rip), %r8    # 0x142935ef8
1400c5e83: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
1400c5e8a: 48 89 fa                    	movq	%rdi, %rdx
1400c5e8d: e8 7e ec c1 00              	callq	0x140ce4b10 <.text+0xce3b10>
1400c5e92: 48 8d 8d 80 00 00 00        	leaq	0x80(%rbp), %rcx
1400c5e99: e8 72 2f f8 ff              	callq	0x140048e10 <.text+0x47e10>
1400c5e9e: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
1400c5ea2: e8 29 33 0a 00              	callq	0x1401691d0 <.text+0x1681d0>
1400c5ea7: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
1400c5eab: e8 c0 12 50 00              	callq	0x1405c7170 <.text+0x5c6170>
1400c5eb0: 48 8d 8d 60 01 00 00        	leaq	0x160(%rbp), %rcx
1400c5eb7: e8 34 0f 50 00              	callq	0x1405c6df0 <.text+0x5c5df0>
1400c5ebc: 0f 10 85 50 01 00 00        	movups	0x150(%rbp), %xmm0
1400c5ec3: 48 8b 85 c8 02 00 00        	movq	0x2c8(%rbp), %rax
1400c5eca: 0f 11 40 60                 	movups	%xmm0, 0x60(%rax)
1400c5ece: 0f 10 85 40 01 00 00        	movups	0x140(%rbp), %xmm0
1400c5ed5: 0f 11 40 50                 	movups	%xmm0, 0x50(%rax)
1400c5ed9: 0f 10 85 30 01 00 00        	movups	0x130(%rbp), %xmm0
1400c5ee0: 0f 11 40 40                 	movups	%xmm0, 0x40(%rax)
1400c5ee4: 0f 10 85 f0 00 00 00        	movups	0xf0(%rbp), %xmm0
1400c5eeb: 0f 10 8d 00 01 00 00        	movups	0x100(%rbp), %xmm1
1400c5ef2: 0f 10 95 10 01 00 00        	movups	0x110(%rbp), %xmm2
1400c5ef9: 0f 10 9d 20 01 00 00        	movups	0x120(%rbp), %xmm3
1400c5f00: 0f 11 58 30                 	movups	%xmm3, 0x30(%rax)
1400c5f04: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
1400c5f08: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
1400c5f0c: 0f 11 00                    	movups	%xmm0, (%rax)
1400c5f0f: 0f 10 85 80 00 00 00        	movups	0x80(%rbp), %xmm0
1400c5f16: 0f 10 8d 90 00 00 00        	movups	0x90(%rbp), %xmm1
1400c5f1d: 0f 10 95 a0 00 00 00        	movups	0xa0(%rbp), %xmm2
1400c5f24: 0f 10 9d b0 00 00 00        	movups	0xb0(%rbp), %xmm3
1400c5f2b: 0f 11 40 70                 	movups	%xmm0, 0x70(%rax)
1400c5f2f: 0f 11 88 80 00 00 00        	movups	%xmm1, 0x80(%rax)
1400c5f36: 0f 11 90 90 00 00 00        	movups	%xmm2, 0x90(%rax)
1400c5f3d: 0f 11 98 a0 00 00 00        	movups	%xmm3, 0xa0(%rax)
1400c5f44: 0f 10 85 c0 00 00 00        	movups	0xc0(%rbp), %xmm0
1400c5f4b: 0f 11 80 b0 00 00 00        	movups	%xmm0, 0xb0(%rax)
1400c5f52: 0f 10 85 d0 00 00 00        	movups	0xd0(%rbp), %xmm0
1400c5f59: 0f 11 80 c0 00 00 00        	movups	%xmm0, 0xc0(%rax)
1400c5f60: 0f 10 85 e0 00 00 00        	movups	0xe0(%rbp), %xmm0
1400c5f67: 0f 11 80 d0 00 00 00        	movups	%xmm0, 0xd0(%rax)
1400c5f6e: 0f 10 45 70                 	movups	0x70(%rbp), %xmm0
1400c5f72: 0f 11 80 40 01 00 00        	movups	%xmm0, 0x140(%rax)
1400c5f79: 0f 10 45 60                 	movups	0x60(%rbp), %xmm0
1400c5f7d: 0f 11 80 30 01 00 00        	movups	%xmm0, 0x130(%rax)
1400c5f84: 0f 10 45 50                 	movups	0x50(%rbp), %xmm0
1400c5f88: 0f 11 80 20 01 00 00        	movups	%xmm0, 0x120(%rax)
1400c5f8f: 0f 10 45 10                 	movups	0x10(%rbp), %xmm0
1400c5f93: 0f 10 4d 20                 	movups	0x20(%rbp), %xmm1
1400c5f97: 0f 10 55 30                 	movups	0x30(%rbp), %xmm2
1400c5f9b: 0f 10 5d 40                 	movups	0x40(%rbp), %xmm3
1400c5f9f: 0f 11 98 10 01 00 00        	movups	%xmm3, 0x110(%rax)
1400c5fa6: 0f 11 90 00 01 00 00        	movups	%xmm2, 0x100(%rax)
1400c5fad: 0f 11 88 f0 00 00 00        	movups	%xmm1, 0xf0(%rax)
1400c5fb4: 0f 11 80 e0 00 00 00        	movups	%xmm0, 0xe0(%rax)
1400c5fbb: 0f 10 45 a0                 	movups	-0x60(%rbp), %xmm0
1400c5fbf: 0f 10 4d b0                 	movups	-0x50(%rbp), %xmm1
1400c5fc3: 0f 10 55 c0                 	movups	-0x40(%rbp), %xmm2
1400c5fc7: 0f 10 5d d0                 	movups	-0x30(%rbp), %xmm3
1400c5fcb: 0f 11 80 50 01 00 00        	movups	%xmm0, 0x150(%rax)
1400c5fd2: 0f 11 88 60 01 00 00        	movups	%xmm1, 0x160(%rax)
1400c5fd9: 0f 11 90 70 01 00 00        	movups	%xmm2, 0x170(%rax)
1400c5fe0: 0f 11 98 80 01 00 00        	movups	%xmm3, 0x180(%rax)
1400c5fe7: 0f 10 45 e0                 	movups	-0x20(%rbp), %xmm0
1400c5feb: 0f 11 80 90 01 00 00        	movups	%xmm0, 0x190(%rax)
1400c5ff2: 0f 10 45 f0                 	movups	-0x10(%rbp), %xmm0
1400c5ff6: 0f 11 80 a0 01 00 00        	movups	%xmm0, 0x1a0(%rax)
1400c5ffd: 0f 10 45 00                 	movups	(%rbp), %xmm0
1400c6001: 0f 11 80 b0 01 00 00        	movups	%xmm0, 0x1b0(%rax)
1400c6008: 0f 10 85 60 01 00 00        	movups	0x160(%rbp), %xmm0
1400c600f: 0f 10 8d 70 01 00 00        	movups	0x170(%rbp), %xmm1
1400c6016: 0f 10 95 80 01 00 00        	movups	0x180(%rbp), %xmm2
1400c601d: 0f 10 9d 90 01 00 00        	movups	0x190(%rbp), %xmm3
1400c6024: 0f 11 80 c0 01 00 00        	movups	%xmm0, 0x1c0(%rax)
1400c602b: 0f 11 88 d0 01 00 00        	movups	%xmm1, 0x1d0(%rax)
1400c6032: 0f 11 90 e0 01 00 00        	movups	%xmm2, 0x1e0(%rax)
1400c6039: 0f 11 98 f0 01 00 00        	movups	%xmm3, 0x1f0(%rax)
1400c6040: 0f 10 85 a0 01 00 00        	movups	0x1a0(%rbp), %xmm0
1400c6047: 0f 11 80 00 02 00 00        	movups	%xmm0, 0x200(%rax)
1400c604e: 0f 10 85 c0 01 00 00        	movups	0x1c0(%rbp), %xmm0
1400c6055: 0f 11 80 20 02 00 00        	movups	%xmm0, 0x220(%rax)
1400c605c: 0f 10 85 b0 01 00 00        	movups	0x1b0(%rbp), %xmm0
1400c6063: 0f 11 80 10 02 00 00        	movups	%xmm0, 0x210(%rax)
1400c606a: 48 c7 46 08 05 00 00 00     	movq	$0x5, 0x8(%rsi)
1400c6072: 48 89 46 10                 	movq	%rax, 0x10(%rsi)
1400c6076: 48 c7 46 18 05 00 00 00     	movq	$0x5, 0x18(%rsi)
1400c607e: 48 c7 46 20 00 00 00 00     	movq	$0x0, 0x20(%rsi)
1400c6086: 48 c7 46 28 08 00 00 00     	movq	$0x8, 0x28(%rsi)
1400c608e: 0f 57 c0                    	xorps	%xmm0, %xmm0
1400c6091: 0f 11 46 30                 	movups	%xmm0, 0x30(%rsi)
1400c6095: 48 89 1e                    	movq	%rbx, (%rsi)
1400c6098: 48 89 f0                    	movq	%rsi, %rax
1400c609b: 48 81 c4 58 03 00 00        	addq	$0x358, %rsp            # imm = 0x358
1400c60a2: 5b                          	popq	%rbx
1400c60a3: 5f                          	popq	%rdi
1400c60a4: 5e                          	popq	%rsi
1400c60a5: 5d                          	popq	%rbp
1400c60a6: c3                          	retq
1400c60a7: b9 08 00 00 00              	movl	$0x8, %ecx
1400c60ac: ba 30 02 00 00              	movl	$0x230, %edx            # imm = 0x230
1400c60b1: e8 4a 2f 81 02              	callq	0x1428d9000 <.text+0x28d8000>
1400c60b6: b9 08 00 00 00              	movl	$0x8, %ecx
1400c60bb: ba 68 01 00 00              	movl	$0x168, %edx            # imm = 0x168
1400c60c0: e8 3b 2f 81 02              	callq	0x1428d9000 <.text+0x28d8000>
