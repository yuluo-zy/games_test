
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1408397a0: 48 81 ec 88 00 00 00        	subq	$0x88, %rsp
1408397a7: b8 01 00 00 00              	movl	$0x1, %eax
1408397ac: f0                          	lock
1408397ad: 0f c1 82 b0 05 00 00        	xaddl	%eax, 0x5b0(%rdx)
1408397b4: f6 01 01                    	testb	$0x1, (%rcx)
1408397b7: 0f 84 0d 01 00 00           	je	0x1408398ca <.text+0x8388ca>
1408397bd: 4c 8b 41 08                 	movq	0x8(%rcx), %r8
1408397c1: 4c 39 82 18 02 00 00        	cmpq	%r8, 0x218(%rdx)
1408397c8: 76 7b                       	jbe	0x140839845 <.text+0x838845>
1408397ca: 4c 8b 8a 10 02 00 00        	movq	0x210(%rdx), %r9
1408397d1: 4f 8b 0c c1                 	movq	(%r9,%r8,8), %r9
1408397d5: 4d 85 c9                    	testq	%r9, %r9
1408397d8: 74 6b                       	je	0x140839845 <.text+0x838845>
1408397da: 4c 8b 82 e0 01 00 00        	movq	0x1e0(%rdx), %r8
1408397e1: 49 f7 d1                    	notq	%r9
1408397e4: 4f 8d 0c 49                 	leaq	(%r9,%r9,2), %r9
1408397e8: 49 c1 e1 05                 	shlq	$0x5, %r9
1408397ec: 4b 83 7c 08 20 00           	cmpq	$0x0, 0x20(%r8,%r9)
1408397f2: 74 51                       	je	0x140839845 <.text+0x838845>
1408397f4: 4d 01 c8                    	addq	%r9, %r8
1408397f7: 4d 8b 48 10                 	movq	0x10(%r8), %r9
1408397fb: 8b 92 b0 05 00 00           	movl	0x5b0(%rdx), %edx
140839801: 80 3d 88 b5 df 02 00        	cmpb	$0x0, 0x2dfb588(%rip)   # 0x143634d90
140839808: 75 0a                       	jne	0x140839814 <.text+0x838814>
14083980a: f3 0f 10 05 66 38 19 02     	movss	0x2193866(%rip), %xmm0  # 0x1429cd078
140839812: eb 08                       	jmp	0x14083981c <.text+0x83881c>
140839814: f3 0f 10 05 e8 c0 0e 02     	movss	0x20ec0e8(%rip), %xmm0  # 0x142925904
14083981c: 41 89 40 5c                 	movl	%eax, 0x5c(%r8)
140839820: f3 41 0f 11 01              	movss	%xmm0, (%r9)
140839825: ba ff ff ff ff              	movl	$0xffffffff, %edx       # imm = 0xFFFFFFFF
14083982a: 49 89 51 04                 	movq	%rdx, 0x4(%r9)
14083982e: 66 41 c7 41 0c 00 01        	movw	$0x100, 0xc(%r9)        # imm = 0x100
140839835: 89 81 48 01 00 00           	movl	%eax, 0x148(%rcx)
14083983b: 31 c0                       	xorl	%eax, %eax
14083983d: 48 81 c4 88 00 00 00        	addq	$0x88, %rsp
140839844: c3                          	retq
140839845: 48 81 c1 b0 00 00 00        	addq	$0xb0, %rcx
14083984c: 48 8d 05 d4 96 25 02        	leaq	0x22596d4(%rip), %rax   # 0x142a92f27
140839853: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
140839858: 48 c7 44 24 30 3b 00 00 00  	movq	$0x3b, 0x30(%rsp)
140839861: 48 89 4c 24 38              	movq	%rcx, 0x38(%rsp)
140839866: 48 8d 05 63 22 80 ff        	leaq	-0x7fdd9d(%rip), %rax   # 0x14003bad0 <.text+0x3aad0>
14083986d: 48 89 44 24 40              	movq	%rax, 0x40(%rsp)
140839872: 48 8d 44 24 28              	leaq	0x28(%rsp), %rax
140839877: 48 89 44 24 48              	movq	%rax, 0x48(%rsp)
14083987c: 48 8d 05 0d 06 8d ff        	leaq	-0x72f9f3(%rip), %rax   # 0x140109e90 <.text+0x108e90>
140839883: 48 89 44 24 50              	movq	%rax, 0x50(%rsp)
140839888: 48 8d 05 01 97 25 02        	leaq	0x2259701(%rip), %rax   # 0x142a92f90
14083988f: 48 89 44 24 58              	movq	%rax, 0x58(%rsp)
140839894: 48 c7 44 24 60 02 00 00 00  	movq	$0x2, 0x60(%rsp)
14083989d: 48 c7 44 24 78 00 00 00 00  	movq	$0x0, 0x78(%rsp)
1408398a6: 48 8d 44 24 38              	leaq	0x38(%rsp), %rax
1408398ab: 48 89 44 24 68              	movq	%rax, 0x68(%rsp)
1408398b0: 48 c7 44 24 70 02 00 00 00  	movq	$0x2, 0x70(%rsp)
1408398b9: 48 8d 15 58 97 25 02        	leaq	0x2259758(%rip), %rdx   # 0x142a93018
1408398c0: 48 8d 4c 24 58              	leaq	0x58(%rsp), %rcx
1408398c5: e8 c6 fa 09 02              	callq	0x1428d9390 <.text+0x28d8390>
1408398ca: 48 8d 0d b7 98 25 02        	leaq	0x22598b7(%rip), %rcx   # 0x142a93188
1408398d1: 4c 8d 05 10 99 25 02        	leaq	0x2259910(%rip), %r8    # 0x142a931e8
1408398d8: ba 59 00 00 00              	movl	$0x59, %edx
1408398dd: e8 4e fa 09 02              	callq	0x1428d9330 <.text+0x28d8330>
1408398e2: cc                          	int3

D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140839b80: 55                          	pushq	%rbp
140839b81: 56                          	pushq	%rsi
140839b82: 57                          	pushq	%rdi
140839b83: 53                          	pushq	%rbx
140839b84: 48 81 ec 98 04 00 00        	subq	$0x498, %rsp            # imm = 0x498
140839b8b: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140839b93: 48 c7 85 10 04 00 00 fe ff ff ff    	movq	$-0x2, 0x410(%rbp)
140839b9e: 48 89 ce                    	movq	%rcx, %rsi
140839ba1: 0f b6 05 61 b0 df 02        	movzbl	0x2dfb061(%rip), %eax   # 0x143634c09
140839ba8: b9 80 03 00 00              	movl	$0x380, %ecx            # imm = 0x380
140839bad: ba 08 00 00 00              	movl	$0x8, %edx
140839bb2: e8 59 a0 dd ff              	callq	0x140613c10 <.text+0x612c10>
140839bb7: 48 85 c0                    	testq	%rax, %rax
140839bba: 0f 84 f1 04 00 00           	je	0x14083a0b1 <.text+0x8390b1>
140839bc0: 48 89 85 08 04 00 00        	movq	%rax, 0x408(%rbp)
140839bc7: 0f 57 c0                    	xorps	%xmm0, %xmm0
140839bca: 0f 11 85 e8 02 00 00        	movups	%xmm0, 0x2e8(%rbp)
140839bd1: 0f 11 85 00 03 00 00        	movups	%xmm0, 0x300(%rbp)
140839bd8: 0f 11 85 18 03 00 00        	movups	%xmm0, 0x318(%rbp)
140839bdf: 0f 11 85 30 03 00 00        	movups	%xmm0, 0x330(%rbp)
140839be6: 0f 11 85 48 03 00 00        	movups	%xmm0, 0x348(%rbp)
140839bed: c7 85 58 03 00 00 00 00 00 00       	movl	$0x0, 0x358(%rbp)
140839bf7: 0f 11 85 80 03 00 00        	movups	%xmm0, 0x380(%rbp)
140839bfe: 0f 11 85 98 03 00 00        	movups	%xmm0, 0x398(%rbp)
140839c05: 0f 11 85 b0 03 00 00        	movups	%xmm0, 0x3b0(%rbp)
140839c0c: 0f 11 85 c8 03 00 00        	movups	%xmm0, 0x3c8(%rbp)
140839c13: 0f 11 85 e0 03 00 00        	movups	%xmm0, 0x3e0(%rbp)
140839c1a: c7 85 f0 03 00 00 00 00 00 00       	movl	$0x0, 0x3f0(%rbp)
140839c24: 48 c7 85 b0 02 00 00 00 00 00 00    	movq	$0x0, 0x2b0(%rbp)
140839c2f: 48 c7 85 c8 02 00 00 00 00 00 00    	movq	$0x0, 0x2c8(%rbp)
140839c3a: 48 c7 85 d0 02 00 00 08 00 00 00    	movq	$0x8, 0x2d0(%rbp)
140839c45: 48 c7 85 d8 02 00 00 00 00 00 00    	movq	$0x0, 0x2d8(%rbp)
140839c50: 48 c7 85 e0 02 00 00 10 00 00 00    	movq	$0x10, 0x2e0(%rbp)
140839c5b: 48 c7 85 f8 02 00 00 10 00 00 00    	movq	$0x10, 0x2f8(%rbp)
140839c66: 48 c7 85 10 03 00 00 10 00 00 00    	movq	$0x10, 0x310(%rbp)
140839c71: 48 c7 85 28 03 00 00 10 00 00 00    	movq	$0x10, 0x328(%rbp)
140839c7c: 48 c7 85 40 03 00 00 10 00 00 00    	movq	$0x10, 0x340(%rbp)
140839c87: 48 bb 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rbx # imm = 0x8000000000000000
140839c91: 48 89 9d 60 03 00 00        	movq	%rbx, 0x360(%rbp)
140839c98: 48 8d 05 09 98 25 02        	leaq	0x2259809(%rip), %rax   # 0x142a934a8
140839c9f: 48 89 85 68 03 00 00        	movq	%rax, 0x368(%rbp)
140839ca6: 48 c7 85 70 03 00 00 41 00 00 00    	movq	$0x41, 0x370(%rbp)
140839cb1: 48 c7 85 78 03 00 00 10 00 00 00    	movq	$0x10, 0x378(%rbp)
140839cbc: 48 c7 85 90 03 00 00 10 00 00 00    	movq	$0x10, 0x390(%rbp)
140839cc7: 48 c7 85 a8 03 00 00 10 00 00 00    	movq	$0x10, 0x3a8(%rbp)
140839cd2: 48 c7 85 c0 03 00 00 10 00 00 00    	movq	$0x10, 0x3c0(%rbp)
140839cdd: 48 c7 85 d8 03 00 00 10 00 00 00    	movq	$0x10, 0x3d8(%rbp)
140839ce8: c7 85 f8 03 00 00 00 00 00 00       	movl	$0x0, 0x3f8(%rbp)
140839cf2: 66 c7 85 fc 03 00 00 01 00  	movw	$0x1, 0x3fc(%rbp)
140839cfb: c7 85 00 04 00 00 00 00 00 00       	movl	$0x0, 0x400(%rbp)
140839d05: 0f b6 05 fd ae df 02        	movzbl	0x2dfaefd(%rip), %eax   # 0x143634c09
140839d0c: b9 58 01 00 00              	movl	$0x158, %ecx            # imm = 0x158
140839d11: ba 08 00 00 00              	movl	$0x8, %edx
140839d16: e8 f5 9e dd ff              	callq	0x140613c10 <.text+0x612c10>
140839d1b: 48 85 c0                    	testq	%rax, %rax
140839d1e: 0f 84 9c 03 00 00           	je	0x14083a0c0 <.text+0x8390c0>
140839d24: 48 89 c7                    	movq	%rax, %rdi
140839d27: 48 8d 95 b0 02 00 00        	leaq	0x2b0(%rbp), %rdx
140839d2e: 41 b8 58 01 00 00           	movl	$0x158, %r8d            # imm = 0x158
140839d34: 48 89 c1                    	movq	%rax, %rcx
140839d37: e8 c4 9e 0e 02              	callq	0x142923c00 <.text+0x2922c00>
140839d3c: 4c 8d 05 05 95 25 02        	leaq	0x2259505(%rip), %r8    # 0x142a93248
140839d43: 48 8d 8d 40 02 00 00        	leaq	0x240(%rbp), %rcx
140839d4a: 48 89 fa                    	movq	%rdi, %rdx
140839d4d: e8 be ad 4a 00              	callq	0x140ce4b10 <.text+0xce3b10>
140839d52: 48 8d 8d d0 01 00 00        	leaq	0x1d0(%rbp), %rcx
140839d59: e8 b2 dc bb 00              	callq	0x1413f7a10 <.text+0x13f6a10>
140839d5e: 48 8d 8d 60 01 00 00        	leaq	0x160(%rbp), %rcx
140839d65: e8 66 de bb 00              	callq	0x1413f7bd0 <.text+0x13f6bd0>
140839d6a: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
140839d71: e8 0a 7f 1d 00              	callq	0x140a11c80 <.text+0xa10c80>
140839d76: 48 8d 8d 80 00 00 00        	leaq	0x80(%rbp), %rcx
140839d7d: e8 0e 84 1d 00              	callq	0x140a12190 <.text+0xa11190>
140839d82: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
140839d86: e8 35 18 bc 00              	callq	0x1413fb5c0 <.text+0x13fa5c0>
140839d8b: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
140839d8f: e8 5c 2d 1f 00              	callq	0x140a2caf0 <.text+0xa2baf0>
140839d94: 48 8d 8d b0 02 00 00        	leaq	0x2b0(%rbp), %rcx
140839d9b: e8 80 3b 30 00              	callq	0x140b3d920 <.text+0xb3c920>
140839da0: 0f 10 85 a0 02 00 00        	movups	0x2a0(%rbp), %xmm0
140839da7: 48 8b 85 08 04 00 00        	movq	0x408(%rbp), %rax
140839dae: 0f 11 40 60                 	movups	%xmm0, 0x60(%rax)
140839db2: 0f 10 85 90 02 00 00        	movups	0x290(%rbp), %xmm0
140839db9: 0f 11 40 50                 	movups	%xmm0, 0x50(%rax)
140839dbd: 0f 10 85 80 02 00 00        	movups	0x280(%rbp), %xmm0
140839dc4: 0f 11 40 40                 	movups	%xmm0, 0x40(%rax)
140839dc8: 0f 10 85 40 02 00 00        	movups	0x240(%rbp), %xmm0
140839dcf: 0f 10 8d 50 02 00 00        	movups	0x250(%rbp), %xmm1
140839dd6: 0f 10 95 60 02 00 00        	movups	0x260(%rbp), %xmm2
140839ddd: 0f 10 9d 70 02 00 00        	movups	0x270(%rbp), %xmm3
140839de4: 0f 11 58 30                 	movups	%xmm3, 0x30(%rax)
140839de8: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
140839dec: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
140839df0: 0f 11 00                    	movups	%xmm0, (%rax)
140839df3: 0f 10 85 d0 01 00 00        	movups	0x1d0(%rbp), %xmm0
140839dfa: 0f 10 8d e0 01 00 00        	movups	0x1e0(%rbp), %xmm1
140839e01: 0f 10 95 f0 01 00 00        	movups	0x1f0(%rbp), %xmm2
140839e08: 0f 10 9d 00 02 00 00        	movups	0x200(%rbp), %xmm3
140839e0f: 0f 11 40 70                 	movups	%xmm0, 0x70(%rax)
140839e13: 0f 11 88 80 00 00 00        	movups	%xmm1, 0x80(%rax)
140839e1a: 0f 11 90 90 00 00 00        	movups	%xmm2, 0x90(%rax)
140839e21: 0f 11 98 a0 00 00 00        	movups	%xmm3, 0xa0(%rax)
140839e28: 0f 10 85 10 02 00 00        	movups	0x210(%rbp), %xmm0
140839e2f: 0f 11 80 b0 00 00 00        	movups	%xmm0, 0xb0(%rax)
140839e36: 0f 10 85 20 02 00 00        	movups	0x220(%rbp), %xmm0
140839e3d: 0f 11 80 c0 00 00 00        	movups	%xmm0, 0xc0(%rax)
140839e44: 0f 10 85 30 02 00 00        	movups	0x230(%rbp), %xmm0
140839e4b: 0f 11 80 d0 00 00 00        	movups	%xmm0, 0xd0(%rax)
140839e52: 0f 10 85 c0 01 00 00        	movups	0x1c0(%rbp), %xmm0
140839e59: 0f 11 80 40 01 00 00        	movups	%xmm0, 0x140(%rax)
140839e60: 0f 10 85 b0 01 00 00        	movups	0x1b0(%rbp), %xmm0
140839e67: 0f 11 80 30 01 00 00        	movups	%xmm0, 0x130(%rax)
140839e6e: 0f 10 85 a0 01 00 00        	movups	0x1a0(%rbp), %xmm0
140839e75: 0f 11 80 20 01 00 00        	movups	%xmm0, 0x120(%rax)
140839e7c: 0f 10 85 60 01 00 00        	movups	0x160(%rbp), %xmm0
140839e83: 0f 10 8d 70 01 00 00        	movups	0x170(%rbp), %xmm1
140839e8a: 0f 10 95 80 01 00 00        	movups	0x180(%rbp), %xmm2
140839e91: 0f 10 9d 90 01 00 00        	movups	0x190(%rbp), %xmm3
140839e98: 0f 11 98 10 01 00 00        	movups	%xmm3, 0x110(%rax)
140839e9f: 0f 11 90 00 01 00 00        	movups	%xmm2, 0x100(%rax)
140839ea6: 0f 11 88 f0 00 00 00        	movups	%xmm1, 0xf0(%rax)
140839ead: 0f 11 80 e0 00 00 00        	movups	%xmm0, 0xe0(%rax)
140839eb4: 0f 10 85 f0 00 00 00        	movups	0xf0(%rbp), %xmm0
140839ebb: 0f 10 8d 00 01 00 00        	movups	0x100(%rbp), %xmm1
140839ec2: 0f 10 95 10 01 00 00        	movups	0x110(%rbp), %xmm2
140839ec9: 0f 10 9d 20 01 00 00        	movups	0x120(%rbp), %xmm3
140839ed0: 0f 11 80 50 01 00 00        	movups	%xmm0, 0x150(%rax)
140839ed7: 0f 11 88 60 01 00 00        	movups	%xmm1, 0x160(%rax)
140839ede: 0f 11 90 70 01 00 00        	movups	%xmm2, 0x170(%rax)
140839ee5: 0f 11 98 80 01 00 00        	movups	%xmm3, 0x180(%rax)
140839eec: 0f 10 85 30 01 00 00        	movups	0x130(%rbp), %xmm0
140839ef3: 0f 11 80 90 01 00 00        	movups	%xmm0, 0x190(%rax)
140839efa: 0f 10 85 40 01 00 00        	movups	0x140(%rbp), %xmm0
140839f01: 0f 11 80 a0 01 00 00        	movups	%xmm0, 0x1a0(%rax)
140839f08: 0f 10 85 50 01 00 00        	movups	0x150(%rbp), %xmm0
140839f0f: 0f 11 80 b0 01 00 00        	movups	%xmm0, 0x1b0(%rax)
140839f16: 0f 10 85 80 00 00 00        	movups	0x80(%rbp), %xmm0
140839f1d: 0f 10 8d 90 00 00 00        	movups	0x90(%rbp), %xmm1
140839f24: 0f 10 95 a0 00 00 00        	movups	0xa0(%rbp), %xmm2
140839f2b: 0f 10 9d b0 00 00 00        	movups	0xb0(%rbp), %xmm3
140839f32: 0f 11 80 c0 01 00 00        	movups	%xmm0, 0x1c0(%rax)
140839f39: 0f 11 88 d0 01 00 00        	movups	%xmm1, 0x1d0(%rax)
140839f40: 0f 11 90 e0 01 00 00        	movups	%xmm2, 0x1e0(%rax)
140839f47: 0f 11 98 f0 01 00 00        	movups	%xmm3, 0x1f0(%rax)
140839f4e: 0f 10 85 c0 00 00 00        	movups	0xc0(%rbp), %xmm0
140839f55: 0f 11 80 00 02 00 00        	movups	%xmm0, 0x200(%rax)
140839f5c: 0f 10 85 e0 00 00 00        	movups	0xe0(%rbp), %xmm0
140839f63: 0f 11 80 20 02 00 00        	movups	%xmm0, 0x220(%rax)
140839f6a: 0f 10 85 d0 00 00 00        	movups	0xd0(%rbp), %xmm0
140839f71: 0f 11 80 10 02 00 00        	movups	%xmm0, 0x210(%rax)
140839f78: 0f 10 45 70                 	movups	0x70(%rbp), %xmm0
140839f7c: 0f 11 80 90 02 00 00        	movups	%xmm0, 0x290(%rax)
140839f83: 0f 10 45 60                 	movups	0x60(%rbp), %xmm0
140839f87: 0f 11 80 80 02 00 00        	movups	%xmm0, 0x280(%rax)
140839f8e: 0f 10 45 50                 	movups	0x50(%rbp), %xmm0
140839f92: 0f 11 80 70 02 00 00        	movups	%xmm0, 0x270(%rax)
140839f99: 0f 10 45 10                 	movups	0x10(%rbp), %xmm0
140839f9d: 0f 10 4d 20                 	movups	0x20(%rbp), %xmm1
140839fa1: 0f 10 55 30                 	movups	0x30(%rbp), %xmm2
140839fa5: 0f 10 5d 40                 	movups	0x40(%rbp), %xmm3
140839fa9: 0f 11 98 60 02 00 00        	movups	%xmm3, 0x260(%rax)
140839fb0: 0f 11 90 50 02 00 00        	movups	%xmm2, 0x250(%rax)
140839fb7: 0f 11 88 40 02 00 00        	movups	%xmm1, 0x240(%rax)
140839fbe: 0f 11 80 30 02 00 00        	movups	%xmm0, 0x230(%rax)
140839fc5: 0f 10 45 a0                 	movups	-0x60(%rbp), %xmm0
140839fc9: 0f 10 4d b0                 	movups	-0x50(%rbp), %xmm1
140839fcd: 0f 10 55 c0                 	movups	-0x40(%rbp), %xmm2
140839fd1: 0f 10 5d d0                 	movups	-0x30(%rbp), %xmm3
140839fd5: 0f 11 80 a0 02 00 00        	movups	%xmm0, 0x2a0(%rax)
140839fdc: 0f 11 88 b0 02 00 00        	movups	%xmm1, 0x2b0(%rax)
140839fe3: 0f 11 90 c0 02 00 00        	movups	%xmm2, 0x2c0(%rax)
140839fea: 0f 11 98 d0 02 00 00        	movups	%xmm3, 0x2d0(%rax)
140839ff1: 0f 10 45 e0                 	movups	-0x20(%rbp), %xmm0
140839ff5: 0f 11 80 e0 02 00 00        	movups	%xmm0, 0x2e0(%rax)
140839ffc: 0f 10 45 f0                 	movups	-0x10(%rbp), %xmm0
14083a000: 0f 11 80 f0 02 00 00        	movups	%xmm0, 0x2f0(%rax)
14083a007: 0f 10 45 00                 	movups	(%rbp), %xmm0
14083a00b: 0f 11 80 00 03 00 00        	movups	%xmm0, 0x300(%rax)
14083a012: 0f 10 85 b0 02 00 00        	movups	0x2b0(%rbp), %xmm0
14083a019: 0f 10 8d c0 02 00 00        	movups	0x2c0(%rbp), %xmm1
14083a020: 0f 10 95 d0 02 00 00        	movups	0x2d0(%rbp), %xmm2
14083a027: 0f 10 9d e0 02 00 00        	movups	0x2e0(%rbp), %xmm3
14083a02e: 0f 11 80 10 03 00 00        	movups	%xmm0, 0x310(%rax)
14083a035: 0f 11 88 20 03 00 00        	movups	%xmm1, 0x320(%rax)
14083a03c: 0f 11 90 30 03 00 00        	movups	%xmm2, 0x330(%rax)
14083a043: 0f 11 98 40 03 00 00        	movups	%xmm3, 0x340(%rax)
14083a04a: 0f 10 85 f0 02 00 00        	movups	0x2f0(%rbp), %xmm0
14083a051: 0f 11 80 50 03 00 00        	movups	%xmm0, 0x350(%rax)
14083a058: 0f 10 85 10 03 00 00        	movups	0x310(%rbp), %xmm0
14083a05f: 0f 11 80 70 03 00 00        	movups	%xmm0, 0x370(%rax)
14083a066: 0f 10 85 00 03 00 00        	movups	0x300(%rbp), %xmm0
14083a06d: 0f 11 80 60 03 00 00        	movups	%xmm0, 0x360(%rax)
14083a074: 48 c7 46 08 08 00 00 00     	movq	$0x8, 0x8(%rsi)
14083a07c: 48 89 46 10                 	movq	%rax, 0x10(%rsi)
14083a080: 48 c7 46 18 08 00 00 00     	movq	$0x8, 0x18(%rsi)
14083a088: 48 c7 46 20 00 00 00 00     	movq	$0x0, 0x20(%rsi)
14083a090: 48 c7 46 28 08 00 00 00     	movq	$0x8, 0x28(%rsi)
14083a098: 0f 57 c0                    	xorps	%xmm0, %xmm0
14083a09b: 0f 11 46 30                 	movups	%xmm0, 0x30(%rsi)
14083a09f: 48 89 1e                    	movq	%rbx, (%rsi)
14083a0a2: 48 89 f0                    	movq	%rsi, %rax
14083a0a5: 48 81 c4 98 04 00 00        	addq	$0x498, %rsp            # imm = 0x498
14083a0ac: 5b                          	popq	%rbx
14083a0ad: 5f                          	popq	%rdi
14083a0ae: 5e                          	popq	%rsi
14083a0af: 5d                          	popq	%rbp
14083a0b0: c3                          	retq
14083a0b1: b9 08 00 00 00              	movl	$0x8, %ecx
14083a0b6: ba 80 03 00 00              	movl	$0x380, %edx            # imm = 0x380
14083a0bb: e8 40 ef 09 02              	callq	0x1428d9000 <.text+0x28d8000>
14083a0c0: b9 08 00 00 00              	movl	$0x8, %ecx
14083a0c5: ba 58 01 00 00              	movl	$0x158, %edx            # imm = 0x158
14083a0ca: e8 31 ef 09 02              	callq	0x1428d9000 <.text+0x28d8000>
14083a0cf: 0f 0b                       	ud2
14083a0d1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
14083a0e0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14083a0e5: 55                          	pushq	%rbp
14083a0e6: 56                          	pushq	%rsi
14083a0e7: 57                          	pushq	%rdi
14083a0e8: 53                          	pushq	%rbx
14083a0e9: 48 83 ec 28                 	subq	$0x28, %rsp
14083a0ed: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14083a0f4: 48 8d 8d b0 02 00 00        	leaq	0x2b0(%rbp), %rcx
14083a0fb: e8 00 b1 da ff              	callq	0x1405e5200 <.text+0x5e4200>
14083a100: 90                          	nop
14083a101: 48 83 c4 28                 	addq	$0x28, %rsp
14083a105: 5b                          	popq	%rbx
14083a106: 5f                          	popq	%rdi
14083a107: 5e                          	popq	%rsi
14083a108: 5d                          	popq	%rbp
14083a109: c3                          	retq
14083a10a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
14083a110: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14083a115: 55                          	pushq	%rbp
14083a116: 56                          	pushq	%rsi
14083a117: 57                          	pushq	%rdi
14083a118: 53                          	pushq	%rbx
14083a119: 48 83 ec 28                 	subq	$0x28, %rsp
14083a11d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14083a124: ba 80 03 00 00              	movl	$0x380, %edx            # imm = 0x380
14083a129: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14083a12f: 48 8b 8d 08 04 00 00        	movq	0x408(%rbp), %rcx
14083a136: e8 e5 9a dd ff              	callq	0x140613c20 <.text+0x612c20>
14083a13b: 90                          	nop
14083a13c: 48 83 c4 28                 	addq	$0x28, %rsp
14083a140: 5b                          	popq	%rbx
14083a141: 5f                          	popq	%rdi
14083a142: 5e                          	popq	%rsi
14083a143: 5d                          	popq	%rbp
14083a144: c3                          	retq
14083a145: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
14083a150: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14083a155: 55                          	pushq	%rbp
14083a156: 56                          	pushq	%rsi
14083a157: 57                          	pushq	%rdi
14083a158: 53                          	pushq	%rbx
14083a159: 48 83 ec 28                 	subq	$0x28, %rsp
14083a15d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14083a164: 48 8d 8d 40 02 00 00        	leaq	0x240(%rbp), %rcx
14083a16b: e8 10 f7 7e ff              	callq	0x140029880 <.text+0x28880>
14083a170: 90                          	nop
14083a171: 48 83 c4 28                 	addq	$0x28, %rsp
14083a175: 5b                          	popq	%rbx
14083a176: 5f                          	popq	%rdi
14083a177: 5e                          	popq	%rsi
14083a178: 5d                          	popq	%rbp
14083a179: c3                          	retq
14083a17a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
14083a180: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14083a185: 55                          	pushq	%rbp
14083a186: 56                          	pushq	%rsi
14083a187: 57                          	pushq	%rdi
14083a188: 53                          	pushq	%rbx
14083a189: 48 83 ec 28                 	subq	$0x28, %rsp
14083a18d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14083a194: 48 8d 8d d0 01 00 00        	leaq	0x1d0(%rbp), %rcx
14083a19b: e8 e0 f6 7e ff              	callq	0x140029880 <.text+0x28880>
14083a1a0: 90                          	nop
14083a1a1: 48 83 c4 28                 	addq	$0x28, %rsp
14083a1a5: 5b                          	popq	%rbx
14083a1a6: 5f                          	popq	%rdi
14083a1a7: 5e                          	popq	%rsi
14083a1a8: 5d                          	popq	%rbp
14083a1a9: c3                          	retq
14083a1aa: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
14083a1b0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14083a1b5: 55                          	pushq	%rbp
14083a1b6: 56                          	pushq	%rsi
14083a1b7: 57                          	pushq	%rdi
14083a1b8: 53                          	pushq	%rbx
14083a1b9: 48 83 ec 28                 	subq	$0x28, %rsp
14083a1bd: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14083a1c4: 48 8d 8d 60 01 00 00        	leaq	0x160(%rbp), %rcx
14083a1cb: e8 b0 f6 7e ff              	callq	0x140029880 <.text+0x28880>
14083a1d0: 90                          	nop
14083a1d1: 48 83 c4 28                 	addq	$0x28, %rsp
14083a1d5: 5b                          	popq	%rbx
14083a1d6: 5f                          	popq	%rdi
14083a1d7: 5e                          	popq	%rsi
14083a1d8: 5d                          	popq	%rbp
14083a1d9: c3                          	retq
14083a1da: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
14083a1e0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14083a1e5: 55                          	pushq	%rbp
14083a1e6: 56                          	pushq	%rsi
14083a1e7: 57                          	pushq	%rdi
14083a1e8: 53                          	pushq	%rbx
14083a1e9: 48 83 ec 28                 	subq	$0x28, %rsp
14083a1ed: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14083a1f4: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
14083a1fb: e8 80 f6 7e ff              	callq	0x140029880 <.text+0x28880>
14083a200: 90                          	nop
14083a201: 48 83 c4 28                 	addq	$0x28, %rsp
14083a205: 5b                          	popq	%rbx
14083a206: 5f                          	popq	%rdi
14083a207: 5e                          	popq	%rsi
14083a208: 5d                          	popq	%rbp
14083a209: c3                          	retq
14083a20a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
14083a210: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14083a215: 55                          	pushq	%rbp
14083a216: 56                          	pushq	%rsi
14083a217: 57                          	pushq	%rdi
14083a218: 53                          	pushq	%rbx
14083a219: 48 83 ec 28                 	subq	$0x28, %rsp
14083a21d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14083a224: 48 8d 8d 80 00 00 00        	leaq	0x80(%rbp), %rcx
14083a22b: e8 50 f6 7e ff              	callq	0x140029880 <.text+0x28880>
14083a230: 90                          	nop
14083a231: 48 83 c4 28                 	addq	$0x28, %rsp
14083a235: 5b                          	popq	%rbx
14083a236: 5f                          	popq	%rdi
14083a237: 5e                          	popq	%rsi
14083a238: 5d                          	popq	%rbp
14083a239: c3                          	retq
14083a23a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
14083a240: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14083a245: 55                          	pushq	%rbp
14083a246: 56                          	pushq	%rsi
14083a247: 57                          	pushq	%rdi
14083a248: 53                          	pushq	%rbx
14083a249: 48 83 ec 28                 	subq	$0x28, %rsp
14083a24d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14083a254: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
14083a258: e8 23 f6 7e ff              	callq	0x140029880 <.text+0x28880>
14083a25d: 90                          	nop
14083a25e: 48 83 c4 28                 	addq	$0x28, %rsp
14083a262: 5b                          	popq	%rbx
14083a263: 5f                          	popq	%rdi
14083a264: 5e                          	popq	%rsi
14083a265: 5d                          	popq	%rbp
14083a266: c3                          	retq
14083a267: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
14083a270: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14083a275: 55                          	pushq	%rbp
14083a276: 56                          	pushq	%rsi
14083a277: 57                          	pushq	%rdi
14083a278: 53                          	pushq	%rbx
14083a279: 48 83 ec 28                 	subq	$0x28, %rsp
14083a27d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14083a284: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
14083a288: e8 f3 f5 7e ff              	callq	0x140029880 <.text+0x28880>
14083a28d: 90                          	nop
14083a28e: 48 83 c4 28                 	addq	$0x28, %rsp
14083a292: 5b                          	popq	%rbx
14083a293: 5f                          	popq	%rdi
14083a294: 5e                          	popq	%rsi
14083a295: 5d                          	popq	%rbp
14083a296: c3                          	retq

D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140941110: 56                          	pushq	%rsi
140941111: 57                          	pushq	%rdi
140941112: 53                          	pushq	%rbx
140941113: 48 81 ec 00 01 00 00        	subq	$0x100, %rsp            # imm = 0x100
14094111a: 48 89 ce                    	movq	%rcx, %rsi
14094111d: 48 8d 7c 24 20              	leaq	0x20(%rsp), %rdi
140941122: 48 89 f9                    	movq	%rdi, %rcx
140941125: e8 e6 5f 15 00              	callq	0x140a97110 <.text+0xa96110>
14094112a: 48 89 f1                    	movq	%rsi, %rcx
14094112d: 48 89 fa                    	movq	%rdi, %rdx
140941130: e8 eb 64 22 00              	callq	0x140b67620 <.text+0xb66620>
140941135: 48 8d 7c 24 20              	leaq	0x20(%rsp), %rdi
14094113a: 48 89 f9                    	movq	%rdi, %rcx
14094113d: e8 de f8 df 00              	callq	0x141740a20 <.text+0x173fa20>
140941142: 48 8d 9c 24 90 00 00 00     	leaq	0x90(%rsp), %rbx
14094114a: 48 89 d9                    	movq	%rbx, %rcx
14094114d: 48 89 fa                    	movq	%rdi, %rdx
140941150: e8 1b 17 b1 00              	callq	0x141452870 <.text+0x1451870>
140941155: 48 89 f1                    	movq	%rsi, %rcx
140941158: b2 01                       	movb	$0x1, %dl
14094115a: 49 89 d8                    	movq	%rbx, %r8
14094115d: e8 ce 56 22 00              	callq	0x140b66830 <.text+0xb65830>
140941162: 48 8d 7c 24 20              	leaq	0x20(%rsp), %rdi
140941167: 48 89 f9                    	movq	%rdi, %rcx
14094116a: e8 e1 e3 1f 00              	callq	0x140b3f550 <.text+0xb3e550>
14094116f: 48 89 f1                    	movq	%rsi, %rcx
140941172: b2 01                       	movb	$0x1, %dl
140941174: 49 89 f8                    	movq	%rdi, %r8
140941177: e8 04 53 22 00              	callq	0x140b66480 <.text+0xb65480>
14094117c: 48 89 f1                    	movq	%rsi, %rcx
14094117f: e8 cc 69 22 00              	callq	0x140b67b50 <.text+0xb66b50>
140941184: 48 89 f1                    	movq	%rsi, %rcx
140941187: 48 81 c4 00 01 00 00        	addq	$0x100, %rsp            # imm = 0x100
14094118e: 5b                          	popq	%rbx
14094118f: 5f                          	popq	%rdi
140941190: 5e                          	popq	%rsi
140941191: e9 2a 4a 22 00              	jmp	0x140b65bc0 <.text+0xb64bc0>

D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141452870: 55                          	pushq	%rbp
141452871: 56                          	pushq	%rsi
141452872: 57                          	pushq	%rdi
141452873: 48 81 ec f0 01 00 00        	subq	$0x1f0, %rsp            # imm = 0x1F0
14145287a: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
141452882: 48 c7 85 68 01 00 00 fe ff ff ff    	movq	$-0x2, 0x168(%rbp)
14145288d: 48 89 d7                    	movq	%rdx, %rdi
141452890: 48 89 ce                    	movq	%rcx, %rsi
141452893: 0f 10 42 60                 	movups	0x60(%rdx), %xmm0
141452897: 0f 29 45 00                 	movaps	%xmm0, (%rbp)
14145289b: 0f 10 42 50                 	movups	0x50(%rdx), %xmm0
14145289f: 0f 29 45 f0                 	movaps	%xmm0, -0x10(%rbp)
1414528a3: 0f 10 42 40                 	movups	0x40(%rdx), %xmm0
1414528a7: 0f 29 45 e0                 	movaps	%xmm0, -0x20(%rbp)
1414528ab: 0f 10 02                    	movups	(%rdx), %xmm0
1414528ae: 0f 10 4a 10                 	movups	0x10(%rdx), %xmm1
1414528b2: 0f 10 52 20                 	movups	0x20(%rdx), %xmm2
1414528b6: 0f 10 5a 30                 	movups	0x30(%rdx), %xmm3
1414528ba: 0f 29 5d d0                 	movaps	%xmm3, -0x30(%rbp)
1414528be: 0f 29 55 c0                 	movaps	%xmm2, -0x40(%rbp)
1414528c2: 0f 29 4d b0                 	movaps	%xmm1, -0x50(%rbp)
1414528c6: 0f 29 45 a0                 	movaps	%xmm0, -0x60(%rbp)
1414528ca: 0f b6 05 38 23 1e 02        	movzbl	0x21e2338(%rip), %eax   # 0x143634c09
1414528d1: b9 50 01 00 00              	movl	$0x150, %ecx            # imm = 0x150
1414528d6: ba 08 00 00 00              	movl	$0x8, %edx
1414528db: e8 30 13 1c ff              	callq	0x140613c10 <.text+0x612c10>
1414528e0: 48 89 85 60 01 00 00        	movq	%rax, 0x160(%rbp)
1414528e7: 48 85 c0                    	testq	%rax, %rax
1414528ea: 0f 84 14 02 00 00           	je	0x141452b04 <.text+0x1451b04>
1414528f0: 0f 10 47 60                 	movups	0x60(%rdi), %xmm0
1414528f4: 0f 29 85 e0 00 00 00        	movaps	%xmm0, 0xe0(%rbp)
1414528fb: 0f 10 47 50                 	movups	0x50(%rdi), %xmm0
1414528ff: 0f 29 85 d0 00 00 00        	movaps	%xmm0, 0xd0(%rbp)
141452906: 0f 10 47 40                 	movups	0x40(%rdi), %xmm0
14145290a: 0f 29 85 c0 00 00 00        	movaps	%xmm0, 0xc0(%rbp)
141452911: 0f 10 07                    	movups	(%rdi), %xmm0
141452914: 0f 10 4f 10                 	movups	0x10(%rdi), %xmm1
141452918: 0f 10 57 20                 	movups	0x20(%rdi), %xmm2
14145291c: 0f 10 5f 30                 	movups	0x30(%rdi), %xmm3
141452920: 0f 29 9d b0 00 00 00        	movaps	%xmm3, 0xb0(%rbp)
141452927: 0f 29 95 a0 00 00 00        	movaps	%xmm2, 0xa0(%rbp)
14145292e: 0f 29 8d 90 00 00 00        	movaps	%xmm1, 0x90(%rbp)
141452935: 0f 29 85 80 00 00 00        	movaps	%xmm0, 0x80(%rbp)
14145293c: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
141452940: e8 1b 3e 2e 00              	callq	0x141736760 <.text+0x1735760>
141452945: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
14145294c: e8 2f 72 3e ff              	callq	0x140839b80 <.text+0x838b80>
141452951: 0f 28 85 e0 00 00 00        	movaps	0xe0(%rbp), %xmm0
141452958: 48 8b 85 60 01 00 00        	movq	0x160(%rbp), %rax
14145295f: 0f 11 40 60                 	movups	%xmm0, 0x60(%rax)
141452963: 0f 28 85 d0 00 00 00        	movaps	0xd0(%rbp), %xmm0
14145296a: 0f 11 40 50                 	movups	%xmm0, 0x50(%rax)
14145296e: 0f 28 85 c0 00 00 00        	movaps	0xc0(%rbp), %xmm0
141452975: 0f 11 40 40                 	movups	%xmm0, 0x40(%rax)
141452979: 0f 28 85 80 00 00 00        	movaps	0x80(%rbp), %xmm0
141452980: 0f 28 8d 90 00 00 00        	movaps	0x90(%rbp), %xmm1
141452987: 0f 28 95 a0 00 00 00        	movaps	0xa0(%rbp), %xmm2
14145298e: 0f 28 9d b0 00 00 00        	movaps	0xb0(%rbp), %xmm3
141452995: 0f 11 58 30                 	movups	%xmm3, 0x30(%rax)
141452999: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
14145299d: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
1414529a1: 0f 11 00                    	movups	%xmm0, (%rax)
1414529a4: 0f 10 45 10                 	movups	0x10(%rbp), %xmm0
1414529a8: 0f 10 4d 20                 	movups	0x20(%rbp), %xmm1
1414529ac: 0f 10 55 30                 	movups	0x30(%rbp), %xmm2
1414529b0: 0f 10 5d 40                 	movups	0x40(%rbp), %xmm3
1414529b4: 0f 11 40 70                 	movups	%xmm0, 0x70(%rax)
1414529b8: 0f 11 88 80 00 00 00        	movups	%xmm1, 0x80(%rax)
1414529bf: 0f 11 90 90 00 00 00        	movups	%xmm2, 0x90(%rax)
1414529c6: 0f 11 98 a0 00 00 00        	movups	%xmm3, 0xa0(%rax)
1414529cd: 0f 10 45 50                 	movups	0x50(%rbp), %xmm0
1414529d1: 0f 11 80 b0 00 00 00        	movups	%xmm0, 0xb0(%rax)
1414529d8: 0f 10 45 60                 	movups	0x60(%rbp), %xmm0
1414529dc: 0f 11 80 c0 00 00 00        	movups	%xmm0, 0xc0(%rax)
1414529e3: 0f 10 45 70                 	movups	0x70(%rbp), %xmm0
1414529e7: 0f 11 80 d0 00 00 00        	movups	%xmm0, 0xd0(%rax)
1414529ee: 0f 10 85 f0 00 00 00        	movups	0xf0(%rbp), %xmm0
1414529f5: 0f 10 8d 00 01 00 00        	movups	0x100(%rbp), %xmm1
1414529fc: 0f 10 95 10 01 00 00        	movups	0x110(%rbp), %xmm2
141452a03: 0f 10 9d 20 01 00 00        	movups	0x120(%rbp), %xmm3
141452a0a: 0f 11 80 e0 00 00 00        	movups	%xmm0, 0xe0(%rax)
141452a11: 0f 11 88 f0 00 00 00        	movups	%xmm1, 0xf0(%rax)
141452a18: 0f 11 90 00 01 00 00        	movups	%xmm2, 0x100(%rax)
141452a1f: 0f 11 98 10 01 00 00        	movups	%xmm3, 0x110(%rax)
141452a26: 0f 10 85 30 01 00 00        	movups	0x130(%rbp), %xmm0
141452a2d: 0f 11 80 20 01 00 00        	movups	%xmm0, 0x120(%rax)
141452a34: 0f 10 85 40 01 00 00        	movups	0x140(%rbp), %xmm0
141452a3b: 0f 11 80 30 01 00 00        	movups	%xmm0, 0x130(%rax)
141452a42: 0f 10 85 50 01 00 00        	movups	0x150(%rbp), %xmm0
141452a49: 0f 11 80 40 01 00 00        	movups	%xmm0, 0x140(%rax)
141452a50: 48 c7 85 f8 00 00 00 03 00 00 00    	movq	$0x3, 0xf8(%rbp)
141452a5b: 48 89 85 00 01 00 00        	movq	%rax, 0x100(%rbp)
141452a62: 48 c7 85 08 01 00 00 03 00 00 00    	movq	$0x3, 0x108(%rbp)
141452a6d: 48 c7 85 10 01 00 00 00 00 00 00    	movq	$0x0, 0x110(%rbp)
141452a78: 48 c7 85 18 01 00 00 08 00 00 00    	movq	$0x8, 0x118(%rbp)
141452a83: 48 8d 8d 28 01 00 00        	leaq	0x128(%rbp), %rcx
141452a8a: 0f 57 c0                    	xorps	%xmm0, %xmm0
141452a8d: 0f 11 85 20 01 00 00        	movups	%xmm0, 0x120(%rbp)
141452a94: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
141452a9e: 48 89 85 f0 00 00 00        	movq	%rax, 0xf0(%rbp)
141452aa5: e8 76 cf 8d ff              	callq	0x140d2fa20 <.text+0xd2ea20>
141452aaa: 0f 10 85 50 01 00 00        	movups	0x150(%rbp), %xmm0
141452ab1: 0f 11 46 60                 	movups	%xmm0, 0x60(%rsi)
141452ab5: 0f 10 85 40 01 00 00        	movups	0x140(%rbp), %xmm0
141452abc: 0f 11 46 50                 	movups	%xmm0, 0x50(%rsi)
141452ac0: 0f 10 85 30 01 00 00        	movups	0x130(%rbp), %xmm0
141452ac7: 0f 11 46 40                 	movups	%xmm0, 0x40(%rsi)
141452acb: 0f 10 85 f0 00 00 00        	movups	0xf0(%rbp), %xmm0
141452ad2: 0f 10 8d 00 01 00 00        	movups	0x100(%rbp), %xmm1
141452ad9: 0f 10 95 10 01 00 00        	movups	0x110(%rbp), %xmm2
141452ae0: 0f 10 9d 20 01 00 00        	movups	0x120(%rbp), %xmm3
141452ae7: 0f 11 5e 30                 	movups	%xmm3, 0x30(%rsi)
141452aeb: 0f 11 56 20                 	movups	%xmm2, 0x20(%rsi)
141452aef: 0f 11 4e 10                 	movups	%xmm1, 0x10(%rsi)
141452af3: 0f 11 06                    	movups	%xmm0, (%rsi)
141452af6: 48 89 f0                    	movq	%rsi, %rax
141452af9: 48 81 c4 f0 01 00 00        	addq	$0x1f0, %rsp            # imm = 0x1F0
141452b00: 5f                          	popq	%rdi
141452b01: 5e                          	popq	%rsi
141452b02: 5d                          	popq	%rbp
141452b03: c3                          	retq
141452b04: b9 08 00 00 00              	movl	$0x8, %ecx
141452b09: ba 50 01 00 00              	movl	$0x150, %edx            # imm = 0x150
141452b0e: e8 ed 64 48 01              	callq	0x1428d9000 <.text+0x28d8000>
141452b13: 0f 0b                       	ud2
141452b15: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
141452b20: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141452b25: 55                          	pushq	%rbp
141452b26: 56                          	pushq	%rsi
141452b27: 57                          	pushq	%rdi
141452b28: 48 83 ec 20                 	subq	$0x20, %rsp
141452b2c: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141452b33: 48 8d 8d 80 00 00 00        	leaq	0x80(%rbp), %rcx
141452b3a: e8 41 6d bd fe              	callq	0x140029880 <.text+0x28880>
141452b3f: ba 50 01 00 00              	movl	$0x150, %edx            # imm = 0x150
141452b44: 41 b8 08 00 00 00           	movl	$0x8, %r8d
141452b4a: 48 8b 8d 60 01 00 00        	movq	0x160(%rbp), %rcx
141452b51: e8 ca 10 1c ff              	callq	0x140613c20 <.text+0x612c20>
141452b56: 90                          	nop
141452b57: 48 83 c4 20                 	addq	$0x20, %rsp
141452b5b: 5f                          	popq	%rdi
141452b5c: 5e                          	popq	%rsi
141452b5d: 5d                          	popq	%rbp
141452b5e: c3                          	retq
141452b5f: 90                          	nop
141452b60: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141452b65: 55                          	pushq	%rbp
141452b66: 56                          	pushq	%rsi
141452b67: 57                          	pushq	%rdi
141452b68: 48 83 ec 20                 	subq	$0x20, %rsp
141452b6c: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141452b73: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
141452b77: e8 04 6d bd fe              	callq	0x140029880 <.text+0x28880>
141452b7c: 90                          	nop
141452b7d: 48 83 c4 20                 	addq	$0x20, %rsp
141452b81: 5f                          	popq	%rdi
141452b82: 5e                          	popq	%rsi
141452b83: 5d                          	popq	%rbp
141452b84: c3                          	retq
141452b85: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
141452b90: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141452b95: 55                          	pushq	%rbp
141452b96: 56                          	pushq	%rsi
141452b97: 57                          	pushq	%rdi
141452b98: 48 83 ec 20                 	subq	$0x20, %rsp
141452b9c: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141452ba3: 48 83 bd 60 01 00 00 00     	cmpq	$0x0, 0x160(%rbp)
141452bab: 74 08                       	je	0x141452bb5 <.text+0x1451bb5>
141452bad: 48 83 c4 20                 	addq	$0x20, %rsp
141452bb1: 5f                          	popq	%rdi
141452bb2: 5e                          	popq	%rsi
141452bb3: 5d                          	popq	%rbp
141452bb4: c3                          	retq
141452bb5: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
141452bb9: e8 c2 6c bd fe              	callq	0x140029880 <.text+0x28880>
141452bbe: eb ed                       	jmp	0x141452bad <.text+0x1451bad>
141452bc0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141452bc5: 55                          	pushq	%rbp
141452bc6: 56                          	pushq	%rsi
141452bc7: 57                          	pushq	%rdi
141452bc8: 48 83 ec 20                 	subq	$0x20, %rsp
141452bcc: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141452bd3: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
141452bda: e8 a1 6c bd fe              	callq	0x140029880 <.text+0x28880>
141452bdf: 90                          	nop
141452be0: 48 83 c4 20                 	addq	$0x20, %rsp
141452be4: 5f                          	popq	%rdi
141452be5: 5e                          	popq	%rsi
141452be6: 5d                          	popq	%rbp
141452be7: c3                          	retq

D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140b66830: 55                          	pushq	%rbp
140b66831: 56                          	pushq	%rsi
140b66832: 53                          	pushq	%rbx
140b66833: 48 81 ec d0 00 00 00        	subq	$0xd0, %rsp
140b6683a: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140b66842: 48 c7 45 48 fe ff ff ff     	movq	$-0x2, 0x48(%rbp)
140b6684a: 4c 89 45 38                 	movq	%r8, 0x38(%rbp)
140b6684e: 88 d3                       	movb	%dl, %bl
140b66850: 48 89 ce                    	movq	%rcx, %rsi
140b66853: 48 83 c1 38                 	addq	$0x38, %rcx
140b66857: c6 45 47 01                 	movb	$0x1, 0x47(%rbp)
140b6685b: 48 ba 84 a7 2b 96 51 d2 1b 8e       	movabsq	$-0x71e42dae69d4587c, %rdx # imm = 0x8E1BD251962BA784
140b66865: 49 b8 9b 52 e6 c0 74 23 78 6a       	movabsq	$0x6a782374c0e6529b, %r8 # imm = 0x6A782374C0E6529B
140b6686f: e8 7c 6e 57 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
140b66874: 48 85 c0                    	testq	%rax, %rax
140b66877: 0f 84 a3 00 00 00           	je	0x140b66920 <.text+0xb65920>
140b6687d: 48 39 96 18 02 00 00        	cmpq	%rdx, 0x218(%rsi)
140b66884: 0f 86 96 00 00 00           	jbe	0x140b66920 <.text+0xb65920>
140b6688a: 48 8b 86 10 02 00 00        	movq	0x210(%rsi), %rax
140b66891: 48 8b 0c d0                 	movq	(%rax,%rdx,8), %rcx
140b66895: 48 85 c9                    	testq	%rcx, %rcx
140b66898: 0f 84 82 00 00 00           	je	0x140b66920 <.text+0xb65920>
140b6689e: 48 8b 86 e0 01 00 00        	movq	0x1e0(%rsi), %rax
140b668a5: 48 f7 d1                    	notq	%rcx
140b668a8: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
140b668ac: 48 c1 e1 05                 	shlq	$0x5, %rcx
140b668b0: 48 83 7c 08 20 00           	cmpq	$0x0, 0x20(%rax,%rcx)
140b668b6: 74 68                       	je	0x140b66920 <.text+0xb65920>
140b668b8: 48 01 c8                    	addq	%rcx, %rax
140b668bb: 48 8b 48 10                 	movq	0x10(%rax), %rcx
140b668bf: 8b 96 b0 05 00 00           	movl	0x5b0(%rsi), %edx
140b668c5: 89 50 5c                    	movl	%edx, 0x5c(%rax)
140b668c8: 48 8b 45 38                 	movq	0x38(%rbp), %rax
140b668cc: 0f 10 40 60                 	movups	0x60(%rax), %xmm0
140b668d0: 0f 29 45 00                 	movaps	%xmm0, (%rbp)
140b668d4: 0f 10 40 50                 	movups	0x50(%rax), %xmm0
140b668d8: 0f 29 45 f0                 	movaps	%xmm0, -0x10(%rbp)
140b668dc: 0f 10 40 40                 	movups	0x40(%rax), %xmm0
140b668e0: 0f 29 45 e0                 	movaps	%xmm0, -0x20(%rbp)
140b668e4: 0f 10 00                    	movups	(%rax), %xmm0
140b668e7: 0f 10 48 10                 	movups	0x10(%rax), %xmm1
140b668eb: 0f 10 50 20                 	movups	0x20(%rax), %xmm2
140b668ef: 0f 10 58 30                 	movups	0x30(%rax), %xmm3
140b668f3: 0f 29 5d d0                 	movaps	%xmm3, -0x30(%rbp)
140b668f7: 0f 29 55 c0                 	movaps	%xmm2, -0x40(%rbp)
140b668fb: 0f 29 4d b0                 	movaps	%xmm1, -0x50(%rbp)
140b668ff: 0f 29 45 a0                 	movaps	%xmm0, -0x60(%rbp)
140b66903: c6 45 47 00                 	movb	$0x0, 0x47(%rbp)
140b66907: 4c 8d 45 a0                 	leaq	-0x60(%rbp), %r8
140b6690b: 89 da                       	movl	%ebx, %edx
140b6690d: e8 de 99 b3 00              	callq	0x1416a02f0 <.text+0x169f2f0>
140b66912: 48 89 f0                    	movq	%rsi, %rax
140b66915: 48 81 c4 d0 00 00 00        	addq	$0xd0, %rsp
140b6691c: 5b                          	popq	%rbx
140b6691d: 5e                          	popq	%rsi
140b6691e: 5d                          	popq	%rbp
140b6691f: c3                          	retq
140b66920: 48 8d 05 99 c3 f8 01        	leaq	0x1f8c399(%rip), %rax   # 0x142af2cc0
140b66927: 48 89 45 18                 	movq	%rax, 0x18(%rbp)
140b6692b: 48 c7 45 20 27 00 00 00     	movq	$0x27, 0x20(%rbp)
140b66933: 48 8d 45 18                 	leaq	0x18(%rbp), %rax
140b66937: 48 89 45 28                 	movq	%rax, 0x28(%rbp)
140b6693b: 48 8d 05 4e 35 5a ff        	leaq	-0xa5cab2(%rip), %rax   # 0x140109e90 <.text+0x108e90>
140b66942: 48 89 45 30                 	movq	%rax, 0x30(%rbp)
140b66946: 48 8d 05 a3 c4 f8 01        	leaq	0x1f8c4a3(%rip), %rax   # 0x142af2df0
140b6694d: 48 89 45 a0                 	movq	%rax, -0x60(%rbp)
140b66951: 48 c7 45 a8 02 00 00 00     	movq	$0x2, -0x58(%rbp)
140b66959: 48 c7 45 c0 00 00 00 00     	movq	$0x0, -0x40(%rbp)
140b66961: 48 8d 45 28                 	leaq	0x28(%rbp), %rax
140b66965: 48 89 45 b0                 	movq	%rax, -0x50(%rbp)
140b66969: 48 c7 45 b8 01 00 00 00     	movq	$0x1, -0x48(%rbp)
140b66971: c6 45 47 01                 	movb	$0x1, 0x47(%rbp)
140b66975: 48 8d 15 2c c3 f8 01        	leaq	0x1f8c32c(%rip), %rdx   # 0x142af2ca8
140b6697c: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
140b66980: e8 0b 2a d7 01              	callq	0x1428d9390 <.text+0x28d8390>
140b66985: 0f 0b                       	ud2
140b66987: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
140b66990: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140b66995: 55                          	pushq	%rbp
140b66996: 56                          	pushq	%rsi
140b66997: 53                          	pushq	%rbx
140b66998: 48 83 ec 20                 	subq	$0x20, %rsp
140b6699c: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140b669a3: 80 7d 47 00                 	cmpb	$0x0, 0x47(%rbp)
140b669a7: 75 08                       	jne	0x140b669b1 <.text+0xb659b1>
140b669a9: 48 83 c4 20                 	addq	$0x20, %rsp
140b669ad: 5b                          	popq	%rbx
140b669ae: 5e                          	popq	%rsi
140b669af: 5d                          	popq	%rbp
140b669b0: c3                          	retq
140b669b1: 48 8b 4d 38                 	movq	0x38(%rbp), %rcx
140b669b5: e8 c6 2e 4c ff              	callq	0x140029880 <.text+0x28880>
140b669ba: eb ed                       	jmp	0x140b669a9 <.text+0xb659a9>

D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140b66480: 55                          	pushq	%rbp
140b66481: 56                          	pushq	%rsi
140b66482: 53                          	pushq	%rbx
140b66483: 48 81 ec 40 01 00 00        	subq	$0x140, %rsp            # imm = 0x140
140b6648a: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140b66492: 48 c7 85 b8 00 00 00 fe ff ff ff    	movq	$-0x2, 0xb8(%rbp)
140b6649d: 4c 89 85 a8 00 00 00        	movq	%r8, 0xa8(%rbp)
140b664a4: 88 d3                       	movb	%dl, %bl
140b664a6: 48 89 ce                    	movq	%rcx, %rsi
140b664a9: 48 83 c1 38                 	addq	$0x38, %rcx
140b664ad: c6 85 b7 00 00 00 01        	movb	$0x1, 0xb7(%rbp)
140b664b4: 48 ba 84 a7 2b 96 51 d2 1b 8e       	movabsq	$-0x71e42dae69d4587c, %rdx # imm = 0x8E1BD251962BA784
140b664be: 49 b8 9b 52 e6 c0 74 23 78 6a       	movabsq	$0x6a782374c0e6529b, %r8 # imm = 0x6A782374C0E6529B
140b664c8: e8 23 72 57 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
140b664cd: 48 85 c0                    	testq	%rax, %rax
140b664d0: 0f 84 20 01 00 00           	je	0x140b665f6 <.text+0xb655f6>
140b664d6: 48 39 96 18 02 00 00        	cmpq	%rdx, 0x218(%rsi)
140b664dd: 0f 86 13 01 00 00           	jbe	0x140b665f6 <.text+0xb655f6>
140b664e3: 48 8b 86 10 02 00 00        	movq	0x210(%rsi), %rax
140b664ea: 48 8b 0c d0                 	movq	(%rax,%rdx,8), %rcx
140b664ee: 48 85 c9                    	testq	%rcx, %rcx
140b664f1: 0f 84 ff 00 00 00           	je	0x140b665f6 <.text+0xb655f6>
140b664f7: 48 8b 86 e0 01 00 00        	movq	0x1e0(%rsi), %rax
140b664fe: 48 f7 d1                    	notq	%rcx
140b66501: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
140b66505: 48 c1 e1 05                 	shlq	$0x5, %rcx
140b66509: 48 83 7c 08 20 00           	cmpq	$0x0, 0x20(%rax,%rcx)
140b6650f: 0f 84 e1 00 00 00           	je	0x140b665f6 <.text+0xb655f6>
140b66515: 48 01 c8                    	addq	%rcx, %rax
140b66518: 48 8b 48 10                 	movq	0x10(%rax), %rcx
140b6651c: 8b 96 b0 05 00 00           	movl	0x5b0(%rsi), %edx
140b66522: 89 50 5c                    	movl	%edx, 0x5c(%rax)
140b66525: 48 8b 85 a8 00 00 00        	movq	0xa8(%rbp), %rax
140b6652c: 0f 10 40 60                 	movups	0x60(%rax), %xmm0
140b66530: 0f 29 45 70                 	movaps	%xmm0, 0x70(%rbp)
140b66534: 0f 10 40 50                 	movups	0x50(%rax), %xmm0
140b66538: 0f 29 45 60                 	movaps	%xmm0, 0x60(%rbp)
140b6653c: 0f 10 40 40                 	movups	0x40(%rax), %xmm0
140b66540: 0f 29 45 50                 	movaps	%xmm0, 0x50(%rbp)
140b66544: 0f 10 00                    	movups	(%rax), %xmm0
140b66547: 0f 10 48 10                 	movups	0x10(%rax), %xmm1
140b6654b: 0f 10 50 20                 	movups	0x20(%rax), %xmm2
140b6654f: 0f 10 58 30                 	movups	0x30(%rax), %xmm3
140b66553: 0f 29 5d 40                 	movaps	%xmm3, 0x40(%rbp)
140b66557: 0f 29 55 30                 	movaps	%xmm2, 0x30(%rbp)
140b6655b: 0f 29 4d 20                 	movaps	%xmm1, 0x20(%rbp)
140b6655f: 0f 29 45 10                 	movaps	%xmm0, 0x10(%rbp)
140b66563: c6 85 b6 00 00 00 01        	movb	$0x1, 0xb6(%rbp)
140b6656a: 89 da                       	movl	%ebx, %edx
140b6656c: e8 8f b2 b3 00              	callq	0x1416a1800 <.text+0x16a0800>
140b66571: 0f 28 45 70                 	movaps	0x70(%rbp), %xmm0
140b66575: 0f 29 45 00                 	movaps	%xmm0, (%rbp)
140b66579: 0f 28 45 60                 	movaps	0x60(%rbp), %xmm0
140b6657d: 0f 29 45 f0                 	movaps	%xmm0, -0x10(%rbp)
140b66581: 0f 28 45 50                 	movaps	0x50(%rbp), %xmm0
140b66585: 0f 29 45 e0                 	movaps	%xmm0, -0x20(%rbp)
140b66589: 0f 28 45 10                 	movaps	0x10(%rbp), %xmm0
140b6658d: 0f 28 4d 20                 	movaps	0x20(%rbp), %xmm1
140b66591: 0f 28 55 30                 	movaps	0x30(%rbp), %xmm2
140b66595: 0f 28 5d 40                 	movaps	0x40(%rbp), %xmm3
140b66599: 0f 29 5d d0                 	movaps	%xmm3, -0x30(%rbp)
140b6659d: 0f 29 55 c0                 	movaps	%xmm2, -0x40(%rbp)
140b665a1: 0f 29 4d b0                 	movaps	%xmm1, -0x50(%rbp)
140b665a5: 0f 29 45 a0                 	movaps	%xmm0, -0x60(%rbp)
140b665a9: c6 85 b6 00 00 00 00        	movb	$0x0, 0xb6(%rbp)
140b665b0: 48 8d 8d 88 00 00 00        	leaq	0x88(%rbp), %rcx
140b665b7: 4c 8d 45 a0                 	leaq	-0x60(%rbp), %r8
140b665bb: 48 89 c2                    	movq	%rax, %rdx
140b665be: 45 31 c9                    	xorl	%r9d, %r9d
140b665c1: e8 6a 87 b3 00              	callq	0x14169ed30 <.text+0x169dd30>
140b665c6: 48 8b 95 88 00 00 00        	movq	0x88(%rbp), %rdx
140b665cd: 48 85 d2                    	testq	%rdx, %rdx
140b665d0: 74 16                       	je	0x140b665e8 <.text+0xb655e8>
140b665d2: 48 8b 8d 90 00 00 00        	movq	0x90(%rbp), %rcx
140b665d9: 48 c1 e2 04                 	shlq	$0x4, %rdx
140b665dd: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140b665e3: e8 38 d6 aa ff              	callq	0x140613c20 <.text+0x612c20>
140b665e8: 48 89 f0                    	movq	%rsi, %rax
140b665eb: 48 81 c4 40 01 00 00        	addq	$0x140, %rsp            # imm = 0x140
140b665f2: 5b                          	popq	%rbx
140b665f3: 5e                          	popq	%rsi
140b665f4: 5d                          	popq	%rbp
140b665f5: c3                          	retq
140b665f6: 48 8d 05 c3 c6 f8 01        	leaq	0x1f8c6c3(%rip), %rax   # 0x142af2cc0
140b665fd: 48 89 45 10                 	movq	%rax, 0x10(%rbp)
140b66601: 48 c7 45 18 27 00 00 00     	movq	$0x27, 0x18(%rbp)
140b66609: 48 8d 45 10                 	leaq	0x10(%rbp), %rax
140b6660d: 48 89 85 88 00 00 00        	movq	%rax, 0x88(%rbp)
140b66614: 48 8d 05 75 38 5a ff        	leaq	-0xa5c78b(%rip), %rax   # 0x140109e90 <.text+0x108e90>
140b6661b: 48 89 85 90 00 00 00        	movq	%rax, 0x90(%rbp)
140b66622: 48 8d 05 c7 c7 f8 01        	leaq	0x1f8c7c7(%rip), %rax   # 0x142af2df0
140b66629: 48 89 45 a0                 	movq	%rax, -0x60(%rbp)
140b6662d: 48 c7 45 a8 02 00 00 00     	movq	$0x2, -0x58(%rbp)
140b66635: 48 c7 45 c0 00 00 00 00     	movq	$0x0, -0x40(%rbp)
140b6663d: 48 8d 85 88 00 00 00        	leaq	0x88(%rbp), %rax
140b66644: 48 89 45 b0                 	movq	%rax, -0x50(%rbp)
140b66648: 48 c7 45 b8 01 00 00 00     	movq	$0x1, -0x48(%rbp)
140b66650: c6 85 b7 00 00 00 01        	movb	$0x1, 0xb7(%rbp)
140b66657: 48 8d 15 4a c6 f8 01        	leaq	0x1f8c64a(%rip), %rdx   # 0x142af2ca8
140b6665e: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
140b66662: e8 29 2d d7 01              	callq	0x1428d9390 <.text+0x28d8390>
140b66667: 0f 0b                       	ud2
140b66669: 0f 1f 80 00 00 00 00        	nopl	(%rax)
140b66670: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140b66675: 55                          	pushq	%rbp
140b66676: 56                          	pushq	%rsi
140b66677: 53                          	pushq	%rbx
140b66678: 48 83 ec 20                 	subq	$0x20, %rsp
140b6667c: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140b66683: 80 bd b6 00 00 00 00        	cmpb	$0x0, 0xb6(%rbp)
140b6668a: 74 10                       	je	0x140b6669c <.text+0xb6569c>
140b6668c: c6 85 b7 00 00 00 00        	movb	$0x0, 0xb7(%rbp)
140b66693: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
140b66697: e8 e4 31 4c ff              	callq	0x140029880 <.text+0x28880>
140b6669c: c6 85 b7 00 00 00 00        	movb	$0x0, 0xb7(%rbp)
140b666a3: 48 83 c4 20                 	addq	$0x20, %rsp
140b666a7: 5b                          	popq	%rbx
140b666a8: 5e                          	popq	%rsi
140b666a9: 5d                          	popq	%rbp
140b666aa: c3                          	retq
140b666ab: 0f 1f 44 00 00              	nopl	(%rax,%rax)
140b666b0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140b666b5: 55                          	pushq	%rbp
140b666b6: 56                          	pushq	%rsi
140b666b7: 53                          	pushq	%rbx
140b666b8: 48 83 ec 20                 	subq	$0x20, %rsp
140b666bc: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140b666c3: 80 bd b7 00 00 00 00        	cmpb	$0x0, 0xb7(%rbp)
140b666ca: 75 08                       	jne	0x140b666d4 <.text+0xb656d4>
140b666cc: 48 83 c4 20                 	addq	$0x20, %rsp
140b666d0: 5b                          	popq	%rbx
140b666d1: 5e                          	popq	%rsi
140b666d2: 5d                          	popq	%rbp
140b666d3: c3                          	retq
140b666d4: 48 8b 8d a8 00 00 00        	movq	0xa8(%rbp), %rcx
140b666db: e8 a0 31 4c ff              	callq	0x140029880 <.text+0x28880>
140b666e0: eb ea                       	jmp	0x140b666cc <.text+0xb656cc>

D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140b67620: 55                          	pushq	%rbp
140b67621: 56                          	pushq	%rsi
140b67622: 48 81 ec 38 01 00 00        	subq	$0x138, %rsp            # imm = 0x138
140b67629: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140b67631: 48 c7 85 b0 00 00 00 fe ff ff ff    	movq	$-0x2, 0xb0(%rbp)
140b6763c: 48 89 95 a0 00 00 00        	movq	%rdx, 0xa0(%rbp)
140b67643: 48 89 ce                    	movq	%rcx, %rsi
140b67646: 48 83 c1 38                 	addq	$0x38, %rcx
140b6764a: c6 85 af 00 00 00 01        	movb	$0x1, 0xaf(%rbp)
140b67651: 48 ba 84 a7 2b 96 51 d2 1b 8e       	movabsq	$-0x71e42dae69d4587c, %rdx # imm = 0x8E1BD251962BA784
140b6765b: 49 b8 9b 52 e6 c0 74 23 78 6a       	movabsq	$0x6a782374c0e6529b, %r8 # imm = 0x6A782374C0E6529B
140b67665: e8 86 60 57 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
140b6766a: 48 85 c0                    	testq	%rax, %rax
140b6766d: 0f 84 1d 01 00 00           	je	0x140b67790 <.text+0xb66790>
140b67673: 48 39 96 18 02 00 00        	cmpq	%rdx, 0x218(%rsi)
140b6767a: 0f 86 10 01 00 00           	jbe	0x140b67790 <.text+0xb66790>
140b67680: 48 8b 86 10 02 00 00        	movq	0x210(%rsi), %rax
140b67687: 48 8b 0c d0                 	movq	(%rax,%rdx,8), %rcx
140b6768b: 48 85 c9                    	testq	%rcx, %rcx
140b6768e: 0f 84 fc 00 00 00           	je	0x140b67790 <.text+0xb66790>
140b67694: 48 8b 86 e0 01 00 00        	movq	0x1e0(%rsi), %rax
140b6769b: 48 f7 d1                    	notq	%rcx
140b6769e: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
140b676a2: 48 c1 e1 05                 	shlq	$0x5, %rcx
140b676a6: 48 83 7c 08 20 00           	cmpq	$0x0, 0x20(%rax,%rcx)
140b676ac: 0f 84 de 00 00 00           	je	0x140b67790 <.text+0xb66790>
140b676b2: 48 01 c8                    	addq	%rcx, %rax
140b676b5: 48 8b 48 10                 	movq	0x10(%rax), %rcx
140b676b9: 8b 96 b0 05 00 00           	movl	0x5b0(%rsi), %edx
140b676bf: 89 50 5c                    	movl	%edx, 0x5c(%rax)
140b676c2: 48 8b 85 a0 00 00 00        	movq	0xa0(%rbp), %rax
140b676c9: 0f 10 40 60                 	movups	0x60(%rax), %xmm0
140b676cd: 0f 29 45 70                 	movaps	%xmm0, 0x70(%rbp)
140b676d1: 0f 10 40 50                 	movups	0x50(%rax), %xmm0
140b676d5: 0f 29 45 60                 	movaps	%xmm0, 0x60(%rbp)
140b676d9: 0f 10 40 40                 	movups	0x40(%rax), %xmm0
140b676dd: 0f 29 45 50                 	movaps	%xmm0, 0x50(%rbp)
140b676e1: 0f 10 00                    	movups	(%rax), %xmm0
140b676e4: 0f 10 48 10                 	movups	0x10(%rax), %xmm1
140b676e8: 0f 10 50 20                 	movups	0x20(%rax), %xmm2
140b676ec: 0f 10 58 30                 	movups	0x30(%rax), %xmm3
140b676f0: 0f 29 5d 40                 	movaps	%xmm3, 0x40(%rbp)
140b676f4: 0f 29 55 30                 	movaps	%xmm2, 0x30(%rbp)
140b676f8: 0f 29 4d 20                 	movaps	%xmm1, 0x20(%rbp)
140b676fc: 0f 29 45 10                 	movaps	%xmm0, 0x10(%rbp)
140b67700: c6 85 ae 00 00 00 01        	movb	$0x1, 0xae(%rbp)
140b67707: e8 e4 9c b3 00              	callq	0x1416a13f0 <.text+0x16a03f0>
140b6770c: 0f 28 45 70                 	movaps	0x70(%rbp), %xmm0
140b67710: 0f 29 45 00                 	movaps	%xmm0, (%rbp)
140b67714: 0f 28 45 60                 	movaps	0x60(%rbp), %xmm0
140b67718: 0f 29 45 f0                 	movaps	%xmm0, -0x10(%rbp)
140b6771c: 0f 28 45 50                 	movaps	0x50(%rbp), %xmm0
140b67720: 0f 29 45 e0                 	movaps	%xmm0, -0x20(%rbp)
140b67724: 0f 28 45 10                 	movaps	0x10(%rbp), %xmm0
140b67728: 0f 28 4d 20                 	movaps	0x20(%rbp), %xmm1
140b6772c: 0f 28 55 30                 	movaps	0x30(%rbp), %xmm2
140b67730: 0f 28 5d 40                 	movaps	0x40(%rbp), %xmm3
140b67734: 0f 29 5d d0                 	movaps	%xmm3, -0x30(%rbp)
140b67738: 0f 29 55 c0                 	movaps	%xmm2, -0x40(%rbp)
140b6773c: 0f 29 4d b0                 	movaps	%xmm1, -0x50(%rbp)
140b67740: 0f 29 45 a0                 	movaps	%xmm0, -0x60(%rbp)
140b67744: c6 85 ae 00 00 00 00        	movb	$0x0, 0xae(%rbp)
140b6774b: 48 8d 8d 80 00 00 00        	leaq	0x80(%rbp), %rcx
140b67752: 4c 8d 45 a0                 	leaq	-0x60(%rbp), %r8
140b67756: 48 89 c2                    	movq	%rax, %rdx
140b67759: 45 31 c9                    	xorl	%r9d, %r9d
140b6775c: e8 cf 75 b3 00              	callq	0x14169ed30 <.text+0x169dd30>
140b67761: 48 8b 95 80 00 00 00        	movq	0x80(%rbp), %rdx
140b67768: 48 85 d2                    	testq	%rdx, %rdx
140b6776b: 74 16                       	je	0x140b67783 <.text+0xb66783>
140b6776d: 48 8b 8d 88 00 00 00        	movq	0x88(%rbp), %rcx
140b67774: 48 c1 e2 04                 	shlq	$0x4, %rdx
140b67778: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140b6777e: e8 9d c4 aa ff              	callq	0x140613c20 <.text+0x612c20>
140b67783: 48 89 f0                    	movq	%rsi, %rax
140b67786: 48 81 c4 38 01 00 00        	addq	$0x138, %rsp            # imm = 0x138
140b6778d: 5e                          	popq	%rsi
140b6778e: 5d                          	popq	%rbp
140b6778f: c3                          	retq
140b67790: 48 8d 05 29 b5 f8 01        	leaq	0x1f8b529(%rip), %rax   # 0x142af2cc0
140b67797: 48 89 45 10                 	movq	%rax, 0x10(%rbp)
140b6779b: 48 c7 45 18 27 00 00 00     	movq	$0x27, 0x18(%rbp)
140b677a3: 48 8d 45 10                 	leaq	0x10(%rbp), %rax
140b677a7: 48 89 85 80 00 00 00        	movq	%rax, 0x80(%rbp)
140b677ae: 48 8d 05 db 26 5a ff        	leaq	-0xa5d925(%rip), %rax   # 0x140109e90 <.text+0x108e90>
140b677b5: 48 89 85 88 00 00 00        	movq	%rax, 0x88(%rbp)
140b677bc: 48 8d 05 2d b6 f8 01        	leaq	0x1f8b62d(%rip), %rax   # 0x142af2df0
140b677c3: 48 89 45 a0                 	movq	%rax, -0x60(%rbp)
140b677c7: 48 c7 45 a8 02 00 00 00     	movq	$0x2, -0x58(%rbp)
140b677cf: 48 c7 45 c0 00 00 00 00     	movq	$0x0, -0x40(%rbp)
140b677d7: 48 8d 85 80 00 00 00        	leaq	0x80(%rbp), %rax
140b677de: 48 89 45 b0                 	movq	%rax, -0x50(%rbp)
140b677e2: 48 c7 45 b8 01 00 00 00     	movq	$0x1, -0x48(%rbp)
140b677ea: c6 85 af 00 00 00 01        	movb	$0x1, 0xaf(%rbp)
140b677f1: 48 8d 15 b0 b4 f8 01        	leaq	0x1f8b4b0(%rip), %rdx   # 0x142af2ca8
140b677f8: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
140b677fc: e8 8f 1b d7 01              	callq	0x1428d9390 <.text+0x28d8390>
140b67801: 0f 0b                       	ud2
140b67803: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
140b67810: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140b67815: 55                          	pushq	%rbp
140b67816: 56                          	pushq	%rsi
140b67817: 48 83 ec 28                 	subq	$0x28, %rsp
140b6781b: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140b67822: 80 bd ae 00 00 00 00        	cmpb	$0x0, 0xae(%rbp)
140b67829: 74 10                       	je	0x140b6783b <.text+0xb6683b>
140b6782b: c6 85 af 00 00 00 00        	movb	$0x0, 0xaf(%rbp)
140b67832: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
140b67836: e8 45 20 4c ff              	callq	0x140029880 <.text+0x28880>
140b6783b: c6 85 af 00 00 00 00        	movb	$0x0, 0xaf(%rbp)
140b67842: 48 83 c4 28                 	addq	$0x28, %rsp
140b67846: 5e                          	popq	%rsi
140b67847: 5d                          	popq	%rbp
140b67848: c3                          	retq
140b67849: 0f 1f 80 00 00 00 00        	nopl	(%rax)
140b67850: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140b67855: 55                          	pushq	%rbp
140b67856: 56                          	pushq	%rsi
140b67857: 48 83 ec 28                 	subq	$0x28, %rsp
140b6785b: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140b67862: 80 bd af 00 00 00 00        	cmpb	$0x0, 0xaf(%rbp)
140b67869: 75 07                       	jne	0x140b67872 <.text+0xb66872>
140b6786b: 48 83 c4 28                 	addq	$0x28, %rsp
140b6786f: 5e                          	popq	%rsi
140b67870: 5d                          	popq	%rbp
140b67871: c3                          	retq
140b67872: 48 8b 8d a0 00 00 00        	movq	0xa0(%rbp), %rcx
140b67879: e8 02 20 4c ff              	callq	0x140029880 <.text+0x28880>
140b6787e: eb eb                       	jmp	0x140b6786b <.text+0xb6686b>

D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140b67b50: 56                          	pushq	%rsi
140b67b51: 57                          	pushq	%rdi
140b67b52: 53                          	pushq	%rbx
140b67b53: 48 81 ec c0 00 00 00        	subq	$0xc0, %rsp
140b67b5a: 48 89 ce                    	movq	%rcx, %rsi
140b67b5d: 48 83 c1 38                 	addq	$0x38, %rcx
140b67b61: 48 ba 84 a7 2b 96 51 d2 1b 8e       	movabsq	$-0x71e42dae69d4587c, %rdx # imm = 0x8E1BD251962BA784
140b67b6b: 49 b8 9b 52 e6 c0 74 23 78 6a       	movabsq	$0x6a782374c0e6529b, %r8 # imm = 0x6A782374C0E6529B
140b67b75: e8 76 5b 57 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
140b67b7a: 48 85 c0                    	testq	%rax, %rax
140b67b7d: 0f 84 9b 00 00 00           	je	0x140b67c1e <.text+0xb66c1e>
140b67b83: 48 39 96 18 02 00 00        	cmpq	%rdx, 0x218(%rsi)
140b67b8a: 0f 86 8e 00 00 00           	jbe	0x140b67c1e <.text+0xb66c1e>
140b67b90: 48 8b 86 10 02 00 00        	movq	0x210(%rsi), %rax
140b67b97: 48 8b 0c d0                 	movq	(%rax,%rdx,8), %rcx
140b67b9b: 48 85 c9                    	testq	%rcx, %rcx
140b67b9e: 74 7e                       	je	0x140b67c1e <.text+0xb66c1e>
140b67ba0: 48 8b 86 e0 01 00 00        	movq	0x1e0(%rsi), %rax
140b67ba7: 48 f7 d1                    	notq	%rcx
140b67baa: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
140b67bae: 48 c1 e1 05                 	shlq	$0x5, %rcx
140b67bb2: 48 83 7c 08 20 00           	cmpq	$0x0, 0x20(%rax,%rcx)
140b67bb8: 74 64                       	je	0x140b67c1e <.text+0xb66c1e>
140b67bba: 48 01 c8                    	addq	%rcx, %rax
140b67bbd: 48 8b 48 10                 	movq	0x10(%rax), %rcx
140b67bc1: 8b 96 b0 05 00 00           	movl	0x5b0(%rsi), %edx
140b67bc7: 89 50 5c                    	movl	%edx, 0x5c(%rax)
140b67bca: e8 51 a0 b3 00              	callq	0x1416a1c20 <.text+0x16a0c20>
140b67bcf: 48 89 c7                    	movq	%rax, %rdi
140b67bd2: 48 8d 5c 24 50              	leaq	0x50(%rsp), %rbx
140b67bd7: 48 89 d9                    	movq	%rbx, %rcx
140b67bda: e8 d1 3c e3 ff              	callq	0x14099b8b0 <.text+0x99a8b0>
140b67bdf: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140b67be4: 48 89 fa                    	movq	%rdi, %rdx
140b67be7: 49 89 d8                    	movq	%rbx, %r8
140b67bea: 45 31 c9                    	xorl	%r9d, %r9d
140b67bed: e8 3e 71 b3 00              	callq	0x14169ed30 <.text+0x169dd30>
140b67bf2: 48 8b 54 24 30              	movq	0x30(%rsp), %rdx
140b67bf7: 48 85 d2                    	testq	%rdx, %rdx
140b67bfa: 74 14                       	je	0x140b67c10 <.text+0xb66c10>
140b67bfc: 48 8b 4c 24 38              	movq	0x38(%rsp), %rcx
140b67c01: 48 c1 e2 04                 	shlq	$0x4, %rdx
140b67c05: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140b67c0b: e8 10 c0 aa ff              	callq	0x140613c20 <.text+0x612c20>
140b67c10: 48 89 f0                    	movq	%rsi, %rax
140b67c13: 48 81 c4 c0 00 00 00        	addq	$0xc0, %rsp
140b67c1a: 5b                          	popq	%rbx
140b67c1b: 5f                          	popq	%rdi
140b67c1c: 5e                          	popq	%rsi
140b67c1d: c3                          	retq
140b67c1e: 48 8d 05 9b b0 f8 01        	leaq	0x1f8b09b(%rip), %rax   # 0x142af2cc0
140b67c25: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
140b67c2a: 48 c7 44 24 38 27 00 00 00  	movq	$0x27, 0x38(%rsp)
140b67c33: 48 8d 44 24 30              	leaq	0x30(%rsp), %rax
140b67c38: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140b67c3d: 48 8d 05 4c 22 5a ff        	leaq	-0xa5ddb4(%rip), %rax   # 0x140109e90 <.text+0x108e90>
140b67c44: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
140b67c49: 48 8d 05 a0 b1 f8 01        	leaq	0x1f8b1a0(%rip), %rax   # 0x142af2df0
140b67c50: 48 89 44 24 50              	movq	%rax, 0x50(%rsp)
140b67c55: 48 c7 44 24 58 02 00 00 00  	movq	$0x2, 0x58(%rsp)
140b67c5e: 48 c7 44 24 70 00 00 00 00  	movq	$0x0, 0x70(%rsp)
140b67c67: 48 8d 44 24 20              	leaq	0x20(%rsp), %rax
140b67c6c: 48 89 44 24 60              	movq	%rax, 0x60(%rsp)
140b67c71: 48 c7 44 24 68 01 00 00 00  	movq	$0x1, 0x68(%rsp)
140b67c7a: 48 8d 15 27 b0 f8 01        	leaq	0x1f8b027(%rip), %rdx   # 0x142af2ca8
140b67c81: 48 8d 4c 24 50              	leaq	0x50(%rsp), %rcx
140b67c86: e8 05 17 d7 01              	callq	0x1428d9390 <.text+0x28d8390>
140b67c8b: cc                          	int3

D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140b3f550: 55                          	pushq	%rbp
140b3f551: 56                          	pushq	%rsi
140b3f552: 57                          	pushq	%rdi
140b3f553: 53                          	pushq	%rbx
140b3f554: 48 81 ec 08 02 00 00        	subq	$0x208, %rsp            # imm = 0x208
140b3f55b: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140b3f563: 48 c7 85 80 01 00 00 fe ff ff ff    	movq	$-0x2, 0x180(%rbp)
140b3f56e: 48 89 ce                    	movq	%rcx, %rsi
140b3f571: 0f b6 05 91 56 af 02        	movzbl	0x2af5691(%rip), %eax   # 0x143634c09
140b3f578: b9 e0 00 00 00              	movl	$0xe0, %ecx
140b3f57d: ba 08 00 00 00              	movl	$0x8, %edx
140b3f582: e8 89 46 ad ff              	callq	0x140613c10 <.text+0x612c10>
140b3f587: 48 85 c0                    	testq	%rax, %rax
140b3f58a: 0f 84 83 02 00 00           	je	0x140b3f813 <.text+0xb3e813>
140b3f590: 48 89 85 78 01 00 00        	movq	%rax, 0x178(%rbp)
140b3f597: 0f 57 c0                    	xorps	%xmm0, %xmm0
140b3f59a: 0f 11 45 58                 	movups	%xmm0, 0x58(%rbp)
140b3f59e: 0f 11 45 70                 	movups	%xmm0, 0x70(%rbp)
140b3f5a2: 0f 11 85 88 00 00 00        	movups	%xmm0, 0x88(%rbp)
140b3f5a9: 0f 11 85 a0 00 00 00        	movups	%xmm0, 0xa0(%rbp)
140b3f5b0: 0f 11 85 b8 00 00 00        	movups	%xmm0, 0xb8(%rbp)
140b3f5b7: c7 85 c8 00 00 00 00 00 00 00       	movl	$0x0, 0xc8(%rbp)
140b3f5c1: 0f 11 85 f0 00 00 00        	movups	%xmm0, 0xf0(%rbp)
140b3f5c8: 0f 11 85 08 01 00 00        	movups	%xmm0, 0x108(%rbp)
140b3f5cf: 0f 11 85 20 01 00 00        	movups	%xmm0, 0x120(%rbp)
140b3f5d6: 0f 11 85 38 01 00 00        	movups	%xmm0, 0x138(%rbp)
140b3f5dd: 0f 11 85 50 01 00 00        	movups	%xmm0, 0x150(%rbp)
140b3f5e4: c7 85 60 01 00 00 00 00 00 00       	movl	$0x0, 0x160(%rbp)
140b3f5ee: 48 c7 45 18 00 00 00 00     	movq	$0x0, 0x18(%rbp)
140b3f5f6: 48 c7 45 38 00 00 00 00     	movq	$0x0, 0x38(%rbp)
140b3f5fe: 48 c7 45 40 08 00 00 00     	movq	$0x8, 0x40(%rbp)
140b3f606: 48 c7 45 48 00 00 00 00     	movq	$0x0, 0x48(%rbp)
140b3f60e: 48 c7 45 50 10 00 00 00     	movq	$0x10, 0x50(%rbp)
140b3f616: 48 c7 45 68 10 00 00 00     	movq	$0x10, 0x68(%rbp)
140b3f61e: 48 c7 85 80 00 00 00 10 00 00 00    	movq	$0x10, 0x80(%rbp)
140b3f629: 48 c7 85 98 00 00 00 10 00 00 00    	movq	$0x10, 0x98(%rbp)
140b3f634: 48 c7 85 b0 00 00 00 10 00 00 00    	movq	$0x10, 0xb0(%rbp)
140b3f63f: 48 bb 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rbx # imm = 0x8000000000000000
140b3f649: 48 89 9d d0 00 00 00        	movq	%rbx, 0xd0(%rbp)
140b3f650: 48 8d 05 a6 f6 fa 01        	leaq	0x1faf6a6(%rip), %rax   # 0x142aeecfd
140b3f657: 48 89 85 d8 00 00 00        	movq	%rax, 0xd8(%rbp)
140b3f65e: 48 c7 85 e0 00 00 00 64 00 00 00    	movq	$0x64, 0xe0(%rbp)
140b3f669: 48 c7 85 e8 00 00 00 10 00 00 00    	movq	$0x10, 0xe8(%rbp)
140b3f674: 48 c7 85 00 01 00 00 10 00 00 00    	movq	$0x10, 0x100(%rbp)
140b3f67f: 48 c7 85 18 01 00 00 10 00 00 00    	movq	$0x10, 0x118(%rbp)
140b3f68a: 48 c7 85 30 01 00 00 10 00 00 00    	movq	$0x10, 0x130(%rbp)
140b3f695: 48 c7 85 48 01 00 00 10 00 00 00    	movq	$0x10, 0x148(%rbp)
140b3f6a0: c7 85 68 01 00 00 00 00 00 00       	movl	$0x0, 0x168(%rbp)
140b3f6aa: 66 c7 85 6c 01 00 00 01 00  	movw	$0x1, 0x16c(%rbp)
140b3f6b3: c7 85 70 01 00 00 00 00 00 00       	movl	$0x0, 0x170(%rbp)
140b3f6bd: 0f b6 05 45 55 af 02        	movzbl	0x2af5545(%rip), %eax   # 0x143634c09
140b3f6c4: b9 60 01 00 00              	movl	$0x160, %ecx            # imm = 0x160
140b3f6c9: ba 08 00 00 00              	movl	$0x8, %edx
140b3f6ce: e8 3d 45 ad ff              	callq	0x140613c10 <.text+0x612c10>
140b3f6d3: 48 85 c0                    	testq	%rax, %rax
140b3f6d6: 0f 84 46 01 00 00           	je	0x140b3f822 <.text+0xb3e822>
140b3f6dc: 48 89 c7                    	movq	%rax, %rdi
140b3f6df: 48 8d 55 18                 	leaq	0x18(%rbp), %rdx
140b3f6e3: 41 b8 60 01 00 00           	movl	$0x160, %r8d            # imm = 0x160
140b3f6e9: 48 89 c1                    	movq	%rax, %rcx
140b3f6ec: e8 0f 45 de 01              	callq	0x142923c00 <.text+0x2922c00>
140b3f6f1: 4c 8d 05 08 f3 fa 01        	leaq	0x1faf308(%rip), %r8    # 0x142aeea00
140b3f6f8: 48 8d 4d a8                 	leaq	-0x58(%rbp), %rcx
140b3f6fc: 48 89 fa                    	movq	%rdi, %rdx
140b3f6ff: e8 0c 54 1a 00              	callq	0x140ce4b10 <.text+0xce3b10>
140b3f704: 48 8d 4d 18                 	leaq	0x18(%rbp), %rcx
140b3f708: e8 23 22 ed ff              	callq	0x140a11930 <.text+0xa10930>
140b3f70d: 0f 10 45 08                 	movups	0x8(%rbp), %xmm0
140b3f711: 48 8b 85 78 01 00 00        	movq	0x178(%rbp), %rax
140b3f718: 0f 11 40 60                 	movups	%xmm0, 0x60(%rax)
140b3f71c: 0f 10 45 f8                 	movups	-0x8(%rbp), %xmm0
140b3f720: 0f 11 40 50                 	movups	%xmm0, 0x50(%rax)
140b3f724: 0f 10 45 e8                 	movups	-0x18(%rbp), %xmm0
140b3f728: 0f 11 40 40                 	movups	%xmm0, 0x40(%rax)
140b3f72c: 0f 10 45 a8                 	movups	-0x58(%rbp), %xmm0
140b3f730: 0f 10 4d b8                 	movups	-0x48(%rbp), %xmm1
140b3f734: 0f 10 55 c8                 	movups	-0x38(%rbp), %xmm2
140b3f738: 0f 10 5d d8                 	movups	-0x28(%rbp), %xmm3
140b3f73c: 0f 11 58 30                 	movups	%xmm3, 0x30(%rax)
140b3f740: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
140b3f744: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
140b3f748: 0f 11 00                    	movups	%xmm0, (%rax)
140b3f74b: 0f 10 45 18                 	movups	0x18(%rbp), %xmm0
140b3f74f: 0f 10 4d 28                 	movups	0x28(%rbp), %xmm1
140b3f753: 0f 10 55 38                 	movups	0x38(%rbp), %xmm2
140b3f757: 0f 10 5d 48                 	movups	0x48(%rbp), %xmm3
140b3f75b: 0f 11 40 70                 	movups	%xmm0, 0x70(%rax)
140b3f75f: 0f 11 88 80 00 00 00        	movups	%xmm1, 0x80(%rax)
140b3f766: 0f 11 90 90 00 00 00        	movups	%xmm2, 0x90(%rax)
140b3f76d: 0f 11 98 a0 00 00 00        	movups	%xmm3, 0xa0(%rax)
140b3f774: 0f 10 45 58                 	movups	0x58(%rbp), %xmm0
140b3f778: 0f 11 80 b0 00 00 00        	movups	%xmm0, 0xb0(%rax)
140b3f77f: 0f 10 45 68                 	movups	0x68(%rbp), %xmm0
140b3f783: 0f 11 80 c0 00 00 00        	movups	%xmm0, 0xc0(%rax)
140b3f78a: 0f 10 45 78                 	movups	0x78(%rbp), %xmm0
140b3f78e: 0f 11 80 d0 00 00 00        	movups	%xmm0, 0xd0(%rax)
140b3f795: 48 c7 45 20 02 00 00 00     	movq	$0x2, 0x20(%rbp)
140b3f79d: 48 89 45 28                 	movq	%rax, 0x28(%rbp)
140b3f7a1: 48 c7 45 30 02 00 00 00     	movq	$0x2, 0x30(%rbp)
140b3f7a9: 48 c7 45 38 00 00 00 00     	movq	$0x0, 0x38(%rbp)
140b3f7b1: 48 c7 45 40 08 00 00 00     	movq	$0x8, 0x40(%rbp)
140b3f7b9: 0f 57 c0                    	xorps	%xmm0, %xmm0
140b3f7bc: 0f 11 45 48                 	movups	%xmm0, 0x48(%rbp)
140b3f7c0: 48 89 5d 18                 	movq	%rbx, 0x18(%rbp)
140b3f7c4: 48 8d 4d 50                 	leaq	0x50(%rbp), %rcx
140b3f7c8: e8 53 02 1f 00              	callq	0x140d2fa20 <.text+0xd2ea20>
140b3f7cd: 0f 10 45 78                 	movups	0x78(%rbp), %xmm0
140b3f7d1: 0f 11 46 60                 	movups	%xmm0, 0x60(%rsi)
140b3f7d5: 0f 10 45 68                 	movups	0x68(%rbp), %xmm0
140b3f7d9: 0f 11 46 50                 	movups	%xmm0, 0x50(%rsi)
140b3f7dd: 0f 10 45 58                 	movups	0x58(%rbp), %xmm0
140b3f7e1: 0f 11 46 40                 	movups	%xmm0, 0x40(%rsi)
140b3f7e5: 0f 10 45 18                 	movups	0x18(%rbp), %xmm0
140b3f7e9: 0f 10 4d 28                 	movups	0x28(%rbp), %xmm1
140b3f7ed: 0f 10 55 38                 	movups	0x38(%rbp), %xmm2
140b3f7f1: 0f 10 5d 48                 	movups	0x48(%rbp), %xmm3
140b3f7f5: 0f 11 5e 30                 	movups	%xmm3, 0x30(%rsi)
140b3f7f9: 0f 11 56 20                 	movups	%xmm2, 0x20(%rsi)
140b3f7fd: 0f 11 4e 10                 	movups	%xmm1, 0x10(%rsi)
140b3f801: 0f 11 06                    	movups	%xmm0, (%rsi)
140b3f804: 48 89 f0                    	movq	%rsi, %rax
140b3f807: 48 81 c4 08 02 00 00        	addq	$0x208, %rsp            # imm = 0x208
140b3f80e: 5b                          	popq	%rbx
140b3f80f: 5f                          	popq	%rdi
140b3f810: 5e                          	popq	%rsi
140b3f811: 5d                          	popq	%rbp
140b3f812: c3                          	retq
140b3f813: b9 08 00 00 00              	movl	$0x8, %ecx
140b3f818: ba e0 00 00 00              	movl	$0xe0, %edx
140b3f81d: e8 de 97 d9 01              	callq	0x1428d9000 <.text+0x28d8000>
140b3f822: b9 08 00 00 00              	movl	$0x8, %ecx
140b3f827: ba 60 01 00 00              	movl	$0x160, %edx            # imm = 0x160
140b3f82c: e8 cf 97 d9 01              	callq	0x1428d9000 <.text+0x28d8000>
140b3f831: 0f 0b                       	ud2
140b3f833: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
140b3f840: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140b3f845: 55                          	pushq	%rbp
140b3f846: 56                          	pushq	%rsi
140b3f847: 57                          	pushq	%rdi
140b3f848: 53                          	pushq	%rbx
140b3f849: 48 83 ec 28                 	subq	$0x28, %rsp
140b3f84d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140b3f854: 48 8d 4d 18                 	leaq	0x18(%rbp), %rcx
140b3f858: e8 13 66 aa ff              	callq	0x1405e5e70 <.text+0x5e4e70>
140b3f85d: 90                          	nop
140b3f85e: 48 83 c4 28                 	addq	$0x28, %rsp
140b3f862: 5b                          	popq	%rbx
140b3f863: 5f                          	popq	%rdi
140b3f864: 5e                          	popq	%rsi
140b3f865: 5d                          	popq	%rbp
140b3f866: c3                          	retq
140b3f867: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
140b3f870: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140b3f875: 55                          	pushq	%rbp
140b3f876: 56                          	pushq	%rsi
140b3f877: 57                          	pushq	%rdi
140b3f878: 53                          	pushq	%rbx
140b3f879: 48 83 ec 28                 	subq	$0x28, %rsp
140b3f87d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140b3f884: ba e0 00 00 00              	movl	$0xe0, %edx
140b3f889: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140b3f88f: 48 8b 8d 78 01 00 00        	movq	0x178(%rbp), %rcx
140b3f896: e8 85 43 ad ff              	callq	0x140613c20 <.text+0x612c20>
140b3f89b: 90                          	nop
140b3f89c: 48 83 c4 28                 	addq	$0x28, %rsp
140b3f8a0: 5b                          	popq	%rbx
140b3f8a1: 5f                          	popq	%rdi
140b3f8a2: 5e                          	popq	%rsi
140b3f8a3: 5d                          	popq	%rbp
140b3f8a4: c3                          	retq
140b3f8a5: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
140b3f8b0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140b3f8b5: 55                          	pushq	%rbp
140b3f8b6: 56                          	pushq	%rsi
140b3f8b7: 57                          	pushq	%rdi
140b3f8b8: 53                          	pushq	%rbx
140b3f8b9: 48 83 ec 28                 	subq	$0x28, %rsp
140b3f8bd: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140b3f8c4: 48 8d 4d a8                 	leaq	-0x58(%rbp), %rcx
140b3f8c8: e8 b3 9f 4e ff              	callq	0x140029880 <.text+0x28880>
140b3f8cd: 90                          	nop
140b3f8ce: 48 83 c4 28                 	addq	$0x28, %rsp
140b3f8d2: 5b                          	popq	%rbx
140b3f8d3: 5f                          	popq	%rdi
140b3f8d4: 5e                          	popq	%rsi
140b3f8d5: 5d                          	popq	%rbp
140b3f8d6: c3                          	retq
140b3f8d7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
140b3f8e0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140b3f8e5: 55                          	pushq	%rbp
140b3f8e6: 56                          	pushq	%rsi
140b3f8e7: 57                          	pushq	%rdi
140b3f8e8: 53                          	pushq	%rbx
140b3f8e9: 48 83 ec 28                 	subq	$0x28, %rsp
140b3f8ed: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140b3f8f4: 48 8d 4d 18                 	leaq	0x18(%rbp), %rcx
140b3f8f8: e8 83 9f 4e ff              	callq	0x140029880 <.text+0x28880>
140b3f8fd: 90                          	nop
140b3f8fe: 48 83 c4 28                 	addq	$0x28, %rsp
140b3f902: 5b                          	popq	%rbx
140b3f903: 5f                          	popq	%rdi
140b3f904: 5e                          	popq	%rsi
140b3f905: 5d                          	popq	%rbp
140b3f906: c3                          	retq

D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140b3c930: 41 57                       	pushq	%r15
140b3c932: 41 56                       	pushq	%r14
140b3c934: 41 55                       	pushq	%r13
140b3c936: 41 54                       	pushq	%r12
140b3c938: 56                          	pushq	%rsi
140b3c939: 57                          	pushq	%rdi
140b3c93a: 53                          	pushq	%rbx
140b3c93b: 48 81 ec 80 00 00 00        	subq	$0x80, %rsp
140b3c942: bb 01 00 00 00              	movl	$0x1, %ebx
140b3c947: f0                          	lock
140b3c948: 0f c1 9a b0 05 00 00        	xaddl	%ebx, 0x5b0(%rdx)
140b3c94f: f6 01 01                    	testb	$0x1, (%rcx)
140b3c952: 0f 84 ad 01 00 00           	je	0x140b3cb05 <.text+0xb3bb05>
140b3c958: 48 89 ce                    	movq	%rcx, %rsi
140b3c95b: 48 8b 41 08                 	movq	0x8(%rcx), %rax
140b3c95f: 48 39 82 18 02 00 00        	cmpq	%rax, 0x218(%rdx)
140b3c966: 0f 86 f6 00 00 00           	jbe	0x140b3ca62 <.text+0xb3ba62>
140b3c96c: 48 8b 8a 10 02 00 00        	movq	0x210(%rdx), %rcx
140b3c973: 48 8b 0c c1                 	movq	(%rcx,%rax,8), %rcx
140b3c977: 48 85 c9                    	testq	%rcx, %rcx
140b3c97a: 0f 84 e2 00 00 00           	je	0x140b3ca62 <.text+0xb3ba62>
140b3c980: 48 8b 82 e0 01 00 00        	movq	0x1e0(%rdx), %rax
140b3c987: 48 f7 d1                    	notq	%rcx
140b3c98a: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
140b3c98e: 48 c1 e1 05                 	shlq	$0x5, %rcx
140b3c992: 48 83 7c 08 20 00           	cmpq	$0x0, 0x20(%rax,%rcx)
140b3c998: 0f 84 c4 00 00 00           	je	0x140b3ca62 <.text+0xb3ba62>
140b3c99e: 48 01 c8                    	addq	%rcx, %rax
140b3c9a1: 4c 8b 70 10                 	movq	0x10(%rax), %r14
140b3c9a5: 8b 8a b0 05 00 00           	movl	0x5b0(%rdx), %ecx
140b3c9ab: 48 8b 4e 10                 	movq	0x10(%rsi), %rcx
140b3c9af: 48 39 8a 18 02 00 00        	cmpq	%rcx, 0x218(%rdx)
140b3c9b6: 0f 86 c4 00 00 00           	jbe	0x140b3ca80 <.text+0xb3ba80>
140b3c9bc: 4c 8b 82 10 02 00 00        	movq	0x210(%rdx), %r8
140b3c9c3: 49 8b 0c c8                 	movq	(%r8,%rcx,8), %rcx
140b3c9c7: 48 85 c9                    	testq	%rcx, %rcx
140b3c9ca: 0f 84 b0 00 00 00           	je	0x140b3ca80 <.text+0xb3ba80>
140b3c9d0: 4c 8b ba e0 01 00 00        	movq	0x1e0(%rdx), %r15
140b3c9d7: 48 f7 d1                    	notq	%rcx
140b3c9da: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
140b3c9de: 48 c1 e1 05                 	shlq	$0x5, %rcx
140b3c9e2: 49 83 7c 0f 20 00           	cmpq	$0x0, 0x20(%r15,%rcx)
140b3c9e8: 0f 84 92 00 00 00           	je	0x140b3ca80 <.text+0xb3ba80>
140b3c9ee: 49 01 cf                    	addq	%rcx, %r15
140b3c9f1: 49 8b 7f 10                 	movq	0x10(%r15), %rdi
140b3c9f5: 8b 8a b0 05 00 00           	movl	0x5b0(%rdx), %ecx
140b3c9fb: 89 58 5c                    	movl	%ebx, 0x5c(%rax)
140b3c9fe: 4d 8b 66 30                 	movq	0x30(%r14), %r12
140b3ca02: 4d 8b 6e 40                 	movq	0x40(%r14), %r13
140b3ca06: 4d 3b 66 20                 	cmpq	0x20(%r14), %r12
140b3ca0a: 75 10                       	jne	0x140b3ca1c <.text+0xb3ba1c>
140b3ca0c: 49 8d 4e 20                 	leaq	0x20(%r14), %rcx
140b3ca10: 48 8d 15 31 d7 0f 02        	leaq	0x20fd731(%rip), %rdx   # 0x142c3a148
140b3ca17: e8 14 5c a2 ff              	callq	0x140562630 <.text+0x561630>
140b3ca1c: 49 8b 46 28                 	movq	0x28(%r14), %rax
140b3ca20: 4c 89 e1                    	movq	%r12, %rcx
140b3ca23: 48 c1 e1 04                 	shlq	$0x4, %rcx
140b3ca27: 4c 89 2c 08                 	movq	%r13, (%rax,%rcx)
140b3ca2b: c6 44 08 08 00              	movb	$0x0, 0x8(%rax,%rcx)
140b3ca30: 49 ff c4                    	incq	%r12
140b3ca33: 4d 89 66 30                 	movq	%r12, 0x30(%r14)
140b3ca37: 49 ff 46 40                 	incq	0x40(%r14)
140b3ca3b: 41 89 5f 5c                 	movl	%ebx, 0x5c(%r15)
140b3ca3f: 48 89 f9                    	movq	%rdi, %rcx
140b3ca42: e8 f9 6d f0 ff              	callq	0x140a43840 <.text+0xa42840>
140b3ca47: 89 9e 50 01 00 00           	movl	%ebx, 0x150(%rsi)
140b3ca4d: 31 c0                       	xorl	%eax, %eax
140b3ca4f: 48 81 c4 80 00 00 00        	addq	$0x80, %rsp
140b3ca56: 5b                          	popq	%rbx
140b3ca57: 5f                          	popq	%rdi
140b3ca58: 5e                          	popq	%rsi
140b3ca59: 41 5c                       	popq	%r12
140b3ca5b: 41 5d                       	popq	%r13
140b3ca5d: 41 5e                       	popq	%r14
140b3ca5f: 41 5f                       	popq	%r15
140b3ca61: c3                          	retq
140b3ca62: 48 81 c6 b8 00 00 00        	addq	$0xb8, %rsi
140b3ca69: 48 8d 05 5c 1c fb 01        	leaq	0x1fb1c5c(%rip), %rax   # 0x142aee6cc
140b3ca70: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140b3ca75: 48 c7 44 24 28 55 00 00 00  	movq	$0x55, 0x28(%rsp)
140b3ca7e: eb 1c                       	jmp	0x140b3ca9c <.text+0xb3ba9c>
140b3ca80: 48 81 c6 b8 00 00 00        	addq	$0xb8, %rsi
140b3ca87: 48 8d 05 e7 1c fb 01        	leaq	0x1fb1ce7(%rip), %rax   # 0x142aee775
140b3ca8e: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140b3ca93: 48 c7 44 24 28 35 00 00 00  	movq	$0x35, 0x28(%rsp)
140b3ca9c: 48 89 74 24 30              	movq	%rsi, 0x30(%rsp)
140b3caa1: 48 8d 05 28 f0 4f ff        	leaq	-0xb00fd8(%rip), %rax   # 0x14003bad0 <.text+0x3aad0>
140b3caa8: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
140b3caad: 48 8d 44 24 20              	leaq	0x20(%rsp), %rax
140b3cab2: 48 89 44 24 40              	movq	%rax, 0x40(%rsp)
140b3cab7: 48 8d 05 d2 d3 5c ff        	leaq	-0xa32c2e(%rip), %rax   # 0x140109e90 <.text+0x108e90>
140b3cabe: 48 89 44 24 48              	movq	%rax, 0x48(%rsp)
140b3cac3: 48 8d 05 9e 1a fb 01        	leaq	0x1fb1a9e(%rip), %rax   # 0x142aee568
140b3caca: 48 89 44 24 50              	movq	%rax, 0x50(%rsp)
140b3cacf: 48 c7 44 24 58 02 00 00 00  	movq	$0x2, 0x58(%rsp)
140b3cad8: 48 c7 44 24 70 00 00 00 00  	movq	$0x0, 0x70(%rsp)
140b3cae1: 48 8d 44 24 30              	leaq	0x30(%rsp), %rax
140b3cae6: 48 89 44 24 60              	movq	%rax, 0x60(%rsp)
140b3caeb: 48 c7 44 24 68 02 00 00 00  	movq	$0x2, 0x68(%rsp)
140b3caf4: 48 8d 15 2d 1c fb 01        	leaq	0x1fb1c2d(%rip), %rdx   # 0x142aee728
140b3cafb: 48 8d 4c 24 50              	leaq	0x50(%rsp), %rcx
140b3cb00: e8 8b c8 d9 01              	callq	0x1428d9390 <.text+0x28d8390>
140b3cb05: 48 8d 0d 7c 1d fb 01        	leaq	0x1fb1d7c(%rip), %rcx   # 0x142aee888
140b3cb0c: 4c 8d 05 d5 1d fb 01        	leaq	0x1fb1dd5(%rip), %r8    # 0x142aee8e8
140b3cb13: ba 59 00 00 00              	movl	$0x59, %edx
140b3cb18: e8 13 c8 d9 01              	callq	0x1428d9330 <.text+0x28d8330>
140b3cb1d: cc                          	int3

D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a11930: 55                          	pushq	%rbp
140a11931: 56                          	pushq	%rsi
140a11932: 57                          	pushq	%rdi
140a11933: 53                          	pushq	%rbx
140a11934: 48 81 ec 38 02 00 00        	subq	$0x238, %rsp            # imm = 0x238
140a1193b: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140a11943: 48 c7 85 b0 01 00 00 fe ff ff ff    	movq	$-0x2, 0x1b0(%rbp)
140a1194e: 48 89 ce                    	movq	%rcx, %rsi
140a11951: 0f b6 05 b1 32 c2 02        	movzbl	0x2c232b1(%rip), %eax   # 0x143634c09
140a11958: b9 e0 00 00 00              	movl	$0xe0, %ecx
140a1195d: ba 08 00 00 00              	movl	$0x8, %edx
140a11962: e8 a9 22 c0 ff              	callq	0x140613c10 <.text+0x612c10>
140a11967: 48 85 c0                    	testq	%rax, %rax
140a1196a: 0f 84 4e 02 00 00           	je	0x140a11bbe <.text+0xa10bbe>
140a11970: 48 89 85 a8 01 00 00        	movq	%rax, 0x1a8(%rbp)
140a11977: 0f 57 c0                    	xorps	%xmm0, %xmm0
140a1197a: 0f 11 85 88 00 00 00        	movups	%xmm0, 0x88(%rbp)
140a11981: 0f 11 85 a0 00 00 00        	movups	%xmm0, 0xa0(%rbp)
140a11988: 0f 11 85 b8 00 00 00        	movups	%xmm0, 0xb8(%rbp)
140a1198f: 0f 11 85 d0 00 00 00        	movups	%xmm0, 0xd0(%rbp)
140a11996: 0f 11 85 e8 00 00 00        	movups	%xmm0, 0xe8(%rbp)
140a1199d: c7 85 f8 00 00 00 00 00 00 00       	movl	$0x0, 0xf8(%rbp)
140a119a7: 0f 11 85 20 01 00 00        	movups	%xmm0, 0x120(%rbp)
140a119ae: 0f 11 85 38 01 00 00        	movups	%xmm0, 0x138(%rbp)
140a119b5: 0f 11 85 50 01 00 00        	movups	%xmm0, 0x150(%rbp)
140a119bc: 0f 11 85 68 01 00 00        	movups	%xmm0, 0x168(%rbp)
140a119c3: 0f 11 85 80 01 00 00        	movups	%xmm0, 0x180(%rbp)
140a119ca: c7 85 90 01 00 00 00 00 00 00       	movl	$0x0, 0x190(%rbp)
140a119d4: 48 c7 45 18 00 00 00 00     	movq	$0x0, 0x18(%rbp)
140a119dc: 48 c7 45 68 00 00 00 00     	movq	$0x0, 0x68(%rbp)
140a119e4: 48 c7 45 70 08 00 00 00     	movq	$0x8, 0x70(%rbp)
140a119ec: 48 c7 45 78 00 00 00 00     	movq	$0x0, 0x78(%rbp)
140a119f4: 48 c7 85 80 00 00 00 10 00 00 00    	movq	$0x10, 0x80(%rbp)
140a119ff: 48 c7 85 98 00 00 00 10 00 00 00    	movq	$0x10, 0x98(%rbp)
140a11a0a: 48 c7 85 b0 00 00 00 10 00 00 00    	movq	$0x10, 0xb0(%rbp)
140a11a15: 48 c7 85 c8 00 00 00 10 00 00 00    	movq	$0x10, 0xc8(%rbp)
140a11a20: 48 c7 85 e0 00 00 00 10 00 00 00    	movq	$0x10, 0xe0(%rbp)
140a11a2b: 48 bb 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rbx # imm = 0x8000000000000000
140a11a35: 48 89 9d 00 01 00 00        	movq	%rbx, 0x100(%rbp)
140a11a3c: 48 8d 05 7d 61 0b 02        	leaq	0x20b617d(%rip), %rax   # 0x142ac7bc0
140a11a43: 48 89 85 08 01 00 00        	movq	%rax, 0x108(%rbp)
140a11a4a: 48 c7 85 10 01 00 00 30 00 00 00    	movq	$0x30, 0x110(%rbp)
140a11a55: 48 c7 85 18 01 00 00 10 00 00 00    	movq	$0x10, 0x118(%rbp)
140a11a60: 48 c7 85 30 01 00 00 10 00 00 00    	movq	$0x10, 0x130(%rbp)
140a11a6b: 48 c7 85 48 01 00 00 10 00 00 00    	movq	$0x10, 0x148(%rbp)
140a11a76: 48 c7 85 60 01 00 00 10 00 00 00    	movq	$0x10, 0x160(%rbp)
140a11a81: 48 c7 85 78 01 00 00 10 00 00 00    	movq	$0x10, 0x178(%rbp)
140a11a8c: c7 85 98 01 00 00 00 00 00 00       	movl	$0x0, 0x198(%rbp)
140a11a96: 66 c7 85 9c 01 00 00 01 00  	movw	$0x1, 0x19c(%rbp)
140a11a9f: c7 85 a0 01 00 00 00 00 00 00       	movl	$0x0, 0x1a0(%rbp)
140a11aa9: 0f b6 05 59 31 c2 02        	movzbl	0x2c23159(%rip), %eax   # 0x143634c09
140a11ab0: b9 90 01 00 00              	movl	$0x190, %ecx            # imm = 0x190
140a11ab5: ba 08 00 00 00              	movl	$0x8, %edx
140a11aba: e8 51 21 c0 ff              	callq	0x140613c10 <.text+0x612c10>
140a11abf: 48 85 c0                    	testq	%rax, %rax
140a11ac2: 0f 84 05 01 00 00           	je	0x140a11bcd <.text+0xa10bcd>
140a11ac8: 48 89 c7                    	movq	%rax, %rdi
140a11acb: 48 8d 55 18                 	leaq	0x18(%rbp), %rdx
140a11acf: 41 b8 90 01 00 00           	movl	$0x190, %r8d            # imm = 0x190
140a11ad5: 48 89 c1                    	movq	%rax, %rcx
140a11ad8: e8 23 21 f1 01              	callq	0x142923c00 <.text+0x2922c00>
140a11add: 4c 8d 05 54 5c 0b 02        	leaq	0x20b5c54(%rip), %r8    # 0x142ac7738
140a11ae4: 48 8d 4d a8                 	leaq	-0x58(%rbp), %rcx
140a11ae8: 48 89 fa                    	movq	%rdi, %rdx
140a11aeb: e8 20 30 2d 00              	callq	0x140ce4b10 <.text+0xce3b10>
140a11af0: 48 8d 4d 18                 	leaq	0x18(%rbp), %rcx
140a11af4: e8 87 2e ff ff              	callq	0x140a04980 <.text+0xa03980>
140a11af9: 0f 10 45 08                 	movups	0x8(%rbp), %xmm0
140a11afd: 48 8b 85 a8 01 00 00        	movq	0x1a8(%rbp), %rax
140a11b04: 0f 11 40 60                 	movups	%xmm0, 0x60(%rax)
140a11b08: 0f 10 45 f8                 	movups	-0x8(%rbp), %xmm0
140a11b0c: 0f 11 40 50                 	movups	%xmm0, 0x50(%rax)
140a11b10: 0f 10 45 e8                 	movups	-0x18(%rbp), %xmm0
140a11b14: 0f 11 40 40                 	movups	%xmm0, 0x40(%rax)
140a11b18: 0f 10 45 a8                 	movups	-0x58(%rbp), %xmm0
140a11b1c: 0f 10 4d b8                 	movups	-0x48(%rbp), %xmm1
140a11b20: 0f 10 55 c8                 	movups	-0x38(%rbp), %xmm2
140a11b24: 0f 10 5d d8                 	movups	-0x28(%rbp), %xmm3
140a11b28: 0f 11 58 30                 	movups	%xmm3, 0x30(%rax)
140a11b2c: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
140a11b30: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
140a11b34: 0f 11 00                    	movups	%xmm0, (%rax)
140a11b37: 0f 10 45 18                 	movups	0x18(%rbp), %xmm0
140a11b3b: 0f 10 4d 28                 	movups	0x28(%rbp), %xmm1
140a11b3f: 0f 10 55 38                 	movups	0x38(%rbp), %xmm2
140a11b43: 0f 10 5d 48                 	movups	0x48(%rbp), %xmm3
140a11b47: 0f 11 40 70                 	movups	%xmm0, 0x70(%rax)
140a11b4b: 0f 11 88 80 00 00 00        	movups	%xmm1, 0x80(%rax)
140a11b52: 0f 11 90 90 00 00 00        	movups	%xmm2, 0x90(%rax)
140a11b59: 0f 11 98 a0 00 00 00        	movups	%xmm3, 0xa0(%rax)
140a11b60: 0f 10 45 58                 	movups	0x58(%rbp), %xmm0
140a11b64: 0f 11 80 b0 00 00 00        	movups	%xmm0, 0xb0(%rax)
140a11b6b: 0f 10 45 68                 	movups	0x68(%rbp), %xmm0
140a11b6f: 0f 11 80 c0 00 00 00        	movups	%xmm0, 0xc0(%rax)
140a11b76: 0f 10 45 78                 	movups	0x78(%rbp), %xmm0
140a11b7a: 0f 11 80 d0 00 00 00        	movups	%xmm0, 0xd0(%rax)
140a11b81: 48 c7 46 08 02 00 00 00     	movq	$0x2, 0x8(%rsi)
140a11b89: 48 89 46 10                 	movq	%rax, 0x10(%rsi)
140a11b8d: 48 c7 46 18 02 00 00 00     	movq	$0x2, 0x18(%rsi)
140a11b95: 48 c7 46 20 00 00 00 00     	movq	$0x0, 0x20(%rsi)
140a11b9d: 48 c7 46 28 08 00 00 00     	movq	$0x8, 0x28(%rsi)
140a11ba5: 0f 57 c0                    	xorps	%xmm0, %xmm0
140a11ba8: 0f 11 46 30                 	movups	%xmm0, 0x30(%rsi)
140a11bac: 48 89 1e                    	movq	%rbx, (%rsi)
140a11baf: 48 89 f0                    	movq	%rsi, %rax
140a11bb2: 48 81 c4 38 02 00 00        	addq	$0x238, %rsp            # imm = 0x238
140a11bb9: 5b                          	popq	%rbx
140a11bba: 5f                          	popq	%rdi
140a11bbb: 5e                          	popq	%rsi
140a11bbc: 5d                          	popq	%rbp
140a11bbd: c3                          	retq
140a11bbe: b9 08 00 00 00              	movl	$0x8, %ecx
140a11bc3: ba e0 00 00 00              	movl	$0xe0, %edx
140a11bc8: e8 33 74 ec 01              	callq	0x1428d9000 <.text+0x28d8000>
140a11bcd: b9 08 00 00 00              	movl	$0x8, %ecx
140a11bd2: ba 90 01 00 00              	movl	$0x190, %edx            # imm = 0x190
140a11bd7: e8 24 74 ec 01              	callq	0x1428d9000 <.text+0x28d8000>
140a11bdc: 0f 0b                       	ud2
140a11bde: 66 90                       	nop
140a11be0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a11be5: 55                          	pushq	%rbp
140a11be6: 56                          	pushq	%rsi
140a11be7: 57                          	pushq	%rdi
140a11be8: 53                          	pushq	%rbx
140a11be9: 48 83 ec 28                 	subq	$0x28, %rsp
140a11bed: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a11bf4: 48 8d 4d 18                 	leaq	0x18(%rbp), %rcx
140a11bf8: e8 43 09 bc ff              	callq	0x1405d2540 <.text+0x5d1540>
140a11bfd: 90                          	nop
140a11bfe: 48 83 c4 28                 	addq	$0x28, %rsp
140a11c02: 5b                          	popq	%rbx
140a11c03: 5f                          	popq	%rdi
140a11c04: 5e                          	popq	%rsi
140a11c05: 5d                          	popq	%rbp
140a11c06: c3                          	retq
140a11c07: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
140a11c10: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a11c15: 55                          	pushq	%rbp
140a11c16: 56                          	pushq	%rsi
140a11c17: 57                          	pushq	%rdi
140a11c18: 53                          	pushq	%rbx
140a11c19: 48 83 ec 28                 	subq	$0x28, %rsp
140a11c1d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a11c24: ba e0 00 00 00              	movl	$0xe0, %edx
140a11c29: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140a11c2f: 48 8b 8d a8 01 00 00        	movq	0x1a8(%rbp), %rcx
140a11c36: e8 e5 1f c0 ff              	callq	0x140613c20 <.text+0x612c20>
140a11c3b: 90                          	nop
140a11c3c: 48 83 c4 28                 	addq	$0x28, %rsp
140a11c40: 5b                          	popq	%rbx
140a11c41: 5f                          	popq	%rdi
140a11c42: 5e                          	popq	%rsi
140a11c43: 5d                          	popq	%rbp
140a11c44: c3                          	retq
140a11c45: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
140a11c50: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a11c55: 55                          	pushq	%rbp
140a11c56: 56                          	pushq	%rsi
140a11c57: 57                          	pushq	%rdi
140a11c58: 53                          	pushq	%rbx
140a11c59: 48 83 ec 28                 	subq	$0x28, %rsp
140a11c5d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a11c64: 48 8d 4d a8                 	leaq	-0x58(%rbp), %rcx
140a11c68: e8 13 7c 61 ff              	callq	0x140029880 <.text+0x28880>
140a11c6d: 90                          	nop
140a11c6e: 48 83 c4 28                 	addq	$0x28, %rsp
140a11c72: 5b                          	popq	%rbx
140a11c73: 5f                          	popq	%rdi
140a11c74: 5e                          	popq	%rsi
140a11c75: 5d                          	popq	%rbp
140a11c76: c3                          	retq

D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a04980: 55                          	pushq	%rbp
140a04981: 56                          	pushq	%rsi
140a04982: 57                          	pushq	%rdi
140a04983: 48 81 ec c0 02 00 00        	subq	$0x2c0, %rsp            # imm = 0x2C0
140a0498a: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140a04992: 48 c7 85 38 02 00 00 fe ff ff ff    	movq	$-0x2, 0x238(%rbp)
140a0499d: 48 89 ce                    	movq	%rcx, %rsi
140a049a0: 0f 57 c0                    	xorps	%xmm0, %xmm0
140a049a3: 0f 11 45 c0                 	movups	%xmm0, -0x40(%rbp)
140a049a7: 0f 11 45 d8                 	movups	%xmm0, -0x28(%rbp)
140a049ab: 0f 11 45 f0                 	movups	%xmm0, -0x10(%rbp)
140a049af: 0f 11 45 08                 	movups	%xmm0, 0x8(%rbp)
140a049b3: 0f 11 45 20                 	movups	%xmm0, 0x20(%rbp)
140a049b7: c7 45 30 00 00 00 00        	movl	$0x0, 0x30(%rbp)
140a049be: 0f 11 45 58                 	movups	%xmm0, 0x58(%rbp)
140a049c2: 0f 11 45 70                 	movups	%xmm0, 0x70(%rbp)
140a049c6: 0f 11 85 88 00 00 00        	movups	%xmm0, 0x88(%rbp)
140a049cd: 0f 11 85 a0 00 00 00        	movups	%xmm0, 0xa0(%rbp)
140a049d4: 0f 11 85 b8 00 00 00        	movups	%xmm0, 0xb8(%rbp)
140a049db: c7 85 c8 00 00 00 00 00 00 00       	movl	$0x0, 0xc8(%rbp)
140a049e5: 48 c7 45 a0 00 00 00 00     	movq	$0x0, -0x60(%rbp)
140a049ed: 48 c7 45 a8 08 00 00 00     	movq	$0x8, -0x58(%rbp)
140a049f5: 48 c7 45 b0 00 00 00 00     	movq	$0x0, -0x50(%rbp)
140a049fd: 48 c7 45 b8 10 00 00 00     	movq	$0x10, -0x48(%rbp)
140a04a05: 48 c7 45 d0 10 00 00 00     	movq	$0x10, -0x30(%rbp)
140a04a0d: 48 c7 45 e8 10 00 00 00     	movq	$0x10, -0x18(%rbp)
140a04a15: 48 c7 45 00 10 00 00 00     	movq	$0x10, (%rbp)
140a04a1d: 48 c7 45 18 10 00 00 00     	movq	$0x10, 0x18(%rbp)
140a04a25: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
140a04a2f: 48 89 45 38                 	movq	%rax, 0x38(%rbp)
140a04a33: 48 8d 0d 37 1a 0c 02        	leaq	0x20c1a37(%rip), %rcx   # 0x142ac6471
140a04a3a: 48 89 4d 40                 	movq	%rcx, 0x40(%rbp)
140a04a3e: 48 c7 45 48 36 00 00 00     	movq	$0x36, 0x48(%rbp)
140a04a46: 48 c7 45 50 10 00 00 00     	movq	$0x10, 0x50(%rbp)
140a04a4e: 48 c7 45 68 10 00 00 00     	movq	$0x10, 0x68(%rbp)
140a04a56: 48 c7 85 80 00 00 00 10 00 00 00    	movq	$0x10, 0x80(%rbp)
140a04a61: 48 c7 85 98 00 00 00 10 00 00 00    	movq	$0x10, 0x98(%rbp)
140a04a6c: 48 c7 85 b0 00 00 00 10 00 00 00    	movq	$0x10, 0xb0(%rbp)
140a04a77: c7 85 d0 00 00 00 00 00 00 00       	movl	$0x0, 0xd0(%rbp)
140a04a81: 66 c7 85 d4 00 00 00 01 00  	movw	$0x1, 0xd4(%rbp)
140a04a8a: 48 89 85 d0 01 00 00        	movq	%rax, 0x1d0(%rbp)
140a04a91: c7 85 30 02 00 00 00 00 00 00       	movl	$0x0, 0x230(%rbp)
140a04a9b: 0f b6 05 67 01 c3 02        	movzbl	0x2c30167(%rip), %eax   # 0x143634c09
140a04aa2: b9 98 02 00 00              	movl	$0x298, %ecx            # imm = 0x298
140a04aa7: ba 08 00 00 00              	movl	$0x8, %edx
140a04aac: e8 5f f1 c0 ff              	callq	0x140613c10 <.text+0x612c10>
140a04ab1: 48 85 c0                    	testq	%rax, %rax
140a04ab4: 74 35                       	je	0x140a04aeb <.text+0xa03aeb>
140a04ab6: 48 89 c7                    	movq	%rax, %rdi
140a04ab9: 48 8d 55 a0                 	leaq	-0x60(%rbp), %rdx
140a04abd: 41 b8 98 02 00 00           	movl	$0x298, %r8d            # imm = 0x298
140a04ac3: 48 89 c1                    	movq	%rax, %rcx
140a04ac6: e8 35 f1 f1 01              	callq	0x142923c00 <.text+0x2922c00>
140a04acb: 4c 8d 05 8e 16 0c 02        	leaq	0x20c168e(%rip), %r8    # 0x142ac6160
140a04ad2: 48 89 f1                    	movq	%rsi, %rcx
140a04ad5: 48 89 fa                    	movq	%rdi, %rdx
140a04ad8: e8 33 00 2e 00              	callq	0x140ce4b10 <.text+0xce3b10>
140a04add: 48 89 f0                    	movq	%rsi, %rax
140a04ae0: 48 81 c4 c0 02 00 00        	addq	$0x2c0, %rsp            # imm = 0x2C0
140a04ae7: 5f                          	popq	%rdi
140a04ae8: 5e                          	popq	%rsi
140a04ae9: 5d                          	popq	%rbp
140a04aea: c3                          	retq
140a04aeb: b9 08 00 00 00              	movl	$0x8, %ecx
140a04af0: ba 98 02 00 00              	movl	$0x298, %edx            # imm = 0x298
140a04af5: e8 06 45 ed 01              	callq	0x1428d9000 <.text+0x28d8000>
140a04afa: 0f 0b                       	ud2
140a04afc: 0f 1f 40 00                 	nopl	(%rax)
140a04b00: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a04b05: 55                          	pushq	%rbp
140a04b06: 56                          	pushq	%rsi
140a04b07: 57                          	pushq	%rdi
140a04b08: 48 83 ec 20                 	subq	$0x20, %rsp
140a04b0c: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a04b13: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
140a04b17: e8 d4 22 e8 ff              	callq	0x140886df0 <.text+0x885df0>
140a04b1c: 90                          	nop
140a04b1d: 48 83 c4 20                 	addq	$0x20, %rsp
140a04b21: 5f                          	popq	%rdi
140a04b22: 5e                          	popq	%rsi
140a04b23: 5d                          	popq	%rbp
140a04b24: c3                          	retq

D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a7a5b0: 55                          	pushq	%rbp
140a7a5b1: 41 57                       	pushq	%r15
140a7a5b3: 41 56                       	pushq	%r14
140a7a5b5: 41 55                       	pushq	%r13
140a7a5b7: 41 54                       	pushq	%r12
140a7a5b9: 56                          	pushq	%rsi
140a7a5ba: 57                          	pushq	%rdi
140a7a5bb: 53                          	pushq	%rbx
140a7a5bc: 48 81 ec f8 00 00 00        	subq	$0xf8, %rsp
140a7a5c3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140a7a5cb: 48 c7 45 70 fe ff ff ff     	movq	$-0x2, 0x70(%rbp)
140a7a5d3: 4c 89 ce                    	movq	%r9, %rsi
140a7a5d6: 4c 89 c7                    	movq	%r8, %rdi
140a7a5d9: 48 89 d3                    	movq	%rdx, %rbx
140a7a5dc: 4c 8b bd f8 00 00 00        	movq	0xf8(%rbp), %r15
140a7a5e3: 4c 8b a5 e8 00 00 00        	movq	0xe8(%rbp), %r12
140a7a5ea: 4c 8b ad e0 00 00 00        	movq	0xe0(%rbp), %r13
140a7a5f1: 8b 41 1c                    	movl	0x1c(%rcx), %eax
140a7a5f4: 48 8b 51 10                 	movq	0x10(%rcx), %rdx
140a7a5f8: 89 02                       	movl	%eax, (%rdx)
140a7a5fa: 4c 8b 31                    	movq	(%rcx), %r14
140a7a5fd: 4c 89 f1                    	movq	%r14, %rcx
140a7a600: e8 2b f7 0f 00              	callq	0x140b79d30 <.text+0xb78d30>
140a7a605: 49 83 c6 30                 	addq	$0x30, %r14
140a7a609: 4c 89 f1                    	movq	%r14, %rcx
140a7a60c: e8 bf f4 0f 00              	callq	0x140b79ad0 <.text+0xb78ad0>
140a7a611: 8b 43 1c                    	movl	0x1c(%rbx), %eax
140a7a614: 48 8b 4b 10                 	movq	0x10(%rbx), %rcx
140a7a618: 89 01                       	movl	%eax, (%rcx)
140a7a61a: 48 8b 03                    	movq	(%rbx), %rax
140a7a61d: 48 89 45 b0                 	movq	%rax, -0x50(%rbp)
140a7a621: 48 c7 45 b8 01 00 00 00     	movq	$0x1, -0x48(%rbp)
140a7a629: 48 8d 5d 20                 	leaq	0x20(%rbp), %rbx
140a7a62d: 4c 8d 75 b0                 	leaq	-0x50(%rbp), %r14
140a7a631: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
140a7a640: 48 89 d9                    	movq	%rbx, %rcx
140a7a643: 4c 89 f2                    	movq	%r14, %rdx
140a7a646: e8 e5 ec ad 00              	callq	0x141559330 <.text+0x1558330>
140a7a64b: 83 7d 20 00                 	cmpl	$0x0, 0x20(%rbp)
140a7a64f: 75 ef                       	jne	0x140a7a640 <.text+0xa79640>
140a7a651: 8b 47 1c                    	movl	0x1c(%rdi), %eax
140a7a654: 48 8b 4f 10                 	movq	0x10(%rdi), %rcx
140a7a658: 89 01                       	movl	%eax, (%rcx)
140a7a65a: 48 8b 3f                    	movq	(%rdi), %rdi
140a7a65d: 48 89 f9                    	movq	%rdi, %rcx
140a7a660: e8 5b ee 0f 00              	callq	0x140b794c0 <.text+0xb784c0>
140a7a665: c6 47 38 01                 	movb	$0x1, 0x38(%rdi)
140a7a669: 8b 46 1c                    	movl	0x1c(%rsi), %eax
140a7a66c: 48 8b 4e 10                 	movq	0x10(%rsi), %rcx
140a7a670: 89 01                       	movl	%eax, (%rcx)
140a7a672: 48 8b 36                    	movq	(%rsi), %rsi
140a7a675: 31 ff                       	xorl	%edi, %edi
140a7a677: 48 bb 02 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffe, %rbx # imm = 0x8000000000000002
140a7a681: eb 48                       	jmp	0x140a7a6cb <.text+0xa796cb>
140a7a683: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
140a7a690: 48 c7 44 3e 38 00 00 00 00  	movq	$0x0, 0x38(%rsi,%rdi)
140a7a699: 49 8d 46 01                 	leaq	0x1(%r14), %rax
140a7a69d: 48 89 c1                    	movq	%rax, %rcx
140a7a6a0: 48 c1 e9 03                 	shrq	$0x3, %rcx
140a7a6a4: 48 83 e0 f8                 	andq	$-0x8, %rax
140a7a6a8: 48 29 c8                    	subq	%rcx, %rax
140a7a6ab: 49 83 fe 08                 	cmpq	$0x8, %r14
140a7a6af: 49 0f 42 c6                 	cmovbq	%r14, %rax
140a7a6b3: 48 89 44 3e 30              	movq	%rax, 0x30(%rsi,%rdi)
140a7a6b8: 48 c7 84 3e a8 00 00 00 00 00 00 00 	movq	$0x0, 0xa8(%rsi,%rdi)
140a7a6c4: 48 81 c7 c0 00 00 00        	addq	$0xc0, %rdi
140a7a6cb: 48 81 ff 40 14 00 00        	cmpq	$0x1440, %rdi           # imm = 0x1440
140a7a6d2: 74 34                       	je	0x140a7a708 <.text+0xa79708>
140a7a6d4: 48 39 5c 3e 08              	cmpq	%rbx, 0x8(%rsi,%rdi)
140a7a6d9: 7c e9                       	jl	0x140a7a6c4 <.text+0xa796c4>
140a7a6db: 48 c7 44 3e 18 00 00 00 00  	movq	$0x0, 0x18(%rsi,%rdi)
140a7a6e4: 48 83 7c 3e 38 00           	cmpq	$0x0, 0x38(%rsi,%rdi)
140a7a6ea: 74 cc                       	je	0x140a7a6b8 <.text+0xa796b8>
140a7a6ec: 4c 8b 74 3e 28              	movq	0x28(%rsi,%rdi), %r14
140a7a6f1: 4d 85 f6                    	testq	%r14, %r14
140a7a6f4: 74 9a                       	je	0x140a7a690 <.text+0xa79690>
140a7a6f6: 48 8b 4c 3e 20              	movq	0x20(%rsi,%rdi), %rcx
140a7a6fb: 4d 8d 46 11                 	leaq	0x11(%r14), %r8
140a7a6ff: b2 ff                       	movb	$-0x1, %dl
140a7a701: e8 1a 95 ea 01              	callq	0x142923c20 <.text+0x2922c20>
140a7a706: eb 88                       	jmp	0x140a7a690 <.text+0xa79690>
140a7a708: 41 8b 44 24 1c              	movl	0x1c(%r12), %eax
140a7a70d: 49 8b 4c 24 10              	movq	0x10(%r12), %rcx
140a7a712: 89 01                       	movl	%eax, (%rcx)
140a7a714: 49 8b 34 24                 	movq	(%r12), %rsi
140a7a718: 48 8b 4e 08                 	movq	0x8(%rsi), %rcx
140a7a71c: 48 8b 46 10                 	movq	0x10(%rsi), %rax
140a7a720: 48 c7 46 10 00 00 00 00     	movq	$0x0, 0x10(%rsi)
140a7a728: 48 85 c0                    	testq	%rax, %rax
140a7a72b: 74 47                       	je	0x140a7a774 <.text+0xa79774>
140a7a72d: ba 01 00 00 00              	movl	$0x1, %edx
140a7a732: 48 29 c2                    	subq	%rax, %rdx
140a7a735: eb 21                       	jmp	0x140a7a758 <.text+0xa79758>
140a7a737: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
140a7a740: 48 8b 45 60                 	movq	0x60(%rbp), %rax
140a7a744: 48 83 c0 50                 	addq	$0x50, %rax
140a7a748: 48 8b 55 e8                 	movq	-0x18(%rbp), %rdx
140a7a74c: 48 ff c2                    	incq	%rdx
140a7a74f: 48 83 fa 01                 	cmpq	$0x1, %rdx
140a7a753: 48 89 c1                    	movq	%rax, %rcx
140a7a756: 74 1c                       	je	0x140a7a774 <.text+0xa79774>
140a7a758: 48 89 4d 60                 	movq	%rcx, 0x60(%rbp)
140a7a75c: 48 89 55 e8                 	movq	%rdx, -0x18(%rbp)
140a7a760: 48 8b 01                    	movq	(%rcx), %rax
140a7a763: f0                          	lock
140a7a764: 48 ff 08                    	decq	(%rax)
140a7a767: 75 d7                       	jne	0x140a7a740 <.text+0xa79740>
140a7a769: 48 8b 4d 60                 	movq	0x60(%rbp), %rcx
140a7a76d: e8 be 43 bd ff              	callq	0x14064eb30 <.text+0x64db30>
140a7a772: eb cc                       	jmp	0x140a7a740 <.text+0xa79740>
140a7a774: c6 46 18 00                 	movb	$0x0, 0x18(%rsi)
140a7a778: 49 8b 45 00                 	movq	(%r13), %rax
140a7a77c: 48 85 c0                    	testq	%rax, %rax
140a7a77f: 74 49                       	je	0x140a7a7ca <.text+0xa797ca>
140a7a781: 49 8b 4d 10                 	movq	0x10(%r13), %rcx
140a7a785: 41 8b 55 1c                 	movl	0x1c(%r13), %edx
140a7a789: 89 11                       	movl	%edx, (%rcx)
140a7a78b: 48 8b 48 10                 	movq	0x10(%rax), %rcx
140a7a78f: 48 85 c9                    	testq	%rcx, %rcx
140a7a792: 74 36                       	je	0x140a7a7ca <.text+0xa797ca>
140a7a794: 48 8b 40 08                 	movq	0x8(%rax), %rax
140a7a798: 48 c1 e1 03                 	shlq	$0x3, %rcx
140a7a79c: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
140a7a7a0: 31 d2                       	xorl	%edx, %edx
140a7a7a2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
140a7a7b0: 48 c7 44 10 08 00 00 00 00  	movq	$0x0, 0x8(%rax,%rdx)
140a7a7b9: c7 44 10 10 00 00 00 00     	movl	$0x0, 0x10(%rax,%rdx)
140a7a7c1: 48 83 c2 18                 	addq	$0x18, %rdx
140a7a7c5: 48 39 d1                    	cmpq	%rdx, %rcx
140a7a7c8: 75 e6                       	jne	0x140a7a7b0 <.text+0xa797b0>
140a7a7ca: 49 8b 1f                    	movq	(%r15), %rbx
140a7a7cd: 83 3b 02                    	cmpl	$0x2, (%rbx)
140a7a7d0: 0f 85 d4 01 00 00           	jne	0x140a7a9aa <.text+0xa799aa>
140a7a7d6: 48 8b 85 f0 00 00 00        	movq	0xf0(%rbp), %rax
140a7a7dd: 48 8b 00                    	movq	(%rax), %rax
140a7a7e0: 48 8b 10                    	movq	(%rax), %rdx
140a7a7e3: 48 85 d2                    	testq	%rdx, %rdx
140a7a7e6: 74 5a                       	je	0x140a7a842 <.text+0xa79842>
140a7a7e8: 48 83 c2 10                 	addq	$0x10, %rdx
140a7a7ec: 4c 8d 05 fd c6 05 02        	leaq	0x205c6fd(%rip), %r8    # 0x142ad6ef0
140a7a7f3: 48 8d 4d 20                 	leaq	0x20(%rbp), %rcx
140a7a7f7: 41 b9 11 00 00 00           	movl	$0x11, %r9d
140a7a7fd: e8 3e 9c fd ff              	callq	0x140a54440 <.text+0xa53440>
140a7a802: 48 83 7d 20 02              	cmpq	$0x2, 0x20(%rbp)
140a7a807: 0f 85 82 00 00 00           	jne	0x140a7a88f <.text+0xa7988f>
140a7a80d: 48 8b 45 28                 	movq	0x28(%rbp), %rax
140a7a811: 48 89 45 b0                 	movq	%rax, -0x50(%rbp)
140a7a815: 48 8d 05 ec c6 05 02        	leaq	0x205c6ec(%rip), %rax   # 0x142ad6f08
140a7a81c: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140a7a821: 48 8d 0d 78 c1 05 02        	leaq	0x205c178(%rip), %rcx   # 0x142ad69a0
140a7a828: 4c 8d 0d 51 c1 05 02        	leaq	0x205c151(%rip), %r9    # 0x142ad6980
140a7a82f: 4c 8d 45 b0                 	leaq	-0x50(%rbp), %r8
140a7a833: ba 2b 00 00 00              	movl	$0x2b, %edx
140a7a838: e8 23 ef e5 01              	callq	0x1428d9760 <.text+0x28d8760>
140a7a83d: e9 c4 01 00 00              	jmp	0x140a7aa06 <.text+0xa79a06>
140a7a842: 0f b6 05 c0 a3 bb 02        	movzbl	0x2bba3c0(%rip), %eax   # 0x143634c09
140a7a849: b9 11 00 00 00              	movl	$0x11, %ecx
140a7a84e: ba 01 00 00 00              	movl	$0x1, %edx
140a7a853: e8 b8 93 b9 ff              	callq	0x140613c10 <.text+0x612c10>
140a7a858: 48 85 c0                    	testq	%rax, %rax
140a7a85b: 0f 84 a7 01 00 00           	je	0x140a7aa08 <.text+0xa79a08>
140a7a861: 0f 10 05 88 c6 05 02        	movups	0x205c688(%rip), %xmm0  # 0x142ad6ef0
140a7a868: 0f 11 00                    	movups	%xmm0, (%rax)
140a7a86b: c6 40 10 67                 	movb	$0x67, 0x10(%rax)
140a7a86f: 48 c7 45 20 00 00 00 00     	movq	$0x0, 0x20(%rbp)
140a7a877: 48 c7 45 30 11 00 00 00     	movq	$0x11, 0x30(%rbp)
140a7a87f: 48 89 45 38                 	movq	%rax, 0x38(%rbp)
140a7a883: 48 c7 45 40 11 00 00 00     	movq	$0x11, 0x40(%rbp)
140a7a88b: c6 45 48 02                 	movb	$0x2, 0x48(%rbp)
140a7a88f: 48 8b 45 20                 	movq	0x20(%rbp), %rax
140a7a893: 48 8b 4d 28                 	movq	0x28(%rbp), %rcx
140a7a897: 48 8b 55 30                 	movq	0x30(%rbp), %rdx
140a7a89b: 4c 8b 45 38                 	movq	0x38(%rbp), %r8
140a7a89f: 4c 8b 4d 40                 	movq	0x40(%rbp), %r9
140a7a8a3: 4c 89 4d 10                 	movq	%r9, 0x10(%rbp)
140a7a8a7: 44 0f b6 4d 48              	movzbl	0x48(%rbp), %r9d
140a7a8ac: 44 88 4d 18                 	movb	%r9b, 0x18(%rbp)
140a7a8b0: 44 8b 4d 49                 	movl	0x49(%rbp), %r9d
140a7a8b4: 44 89 4d 19                 	movl	%r9d, 0x19(%rbp)
140a7a8b8: 44 0f b7 4d 4d              	movzwl	0x4d(%rbp), %r9d
140a7a8bd: 66 44 89 4d 1d              	movw	%r9w, 0x1d(%rbp)
140a7a8c2: 44 0f b6 4d 4f              	movzbl	0x4f(%rbp), %r9d
140a7a8c7: 44 88 4d 1f                 	movb	%r9b, 0x1f(%rbp)
140a7a8cb: 48 89 55 00                 	movq	%rdx, (%rbp)
140a7a8cf: 4c 89 45 08                 	movq	%r8, 0x8(%rbp)
140a7a8d3: 48 89 45 f0                 	movq	%rax, -0x10(%rbp)
140a7a8d7: 48 89 4d f8                 	movq	%rcx, -0x8(%rbp)
140a7a8db: 83 7d f0 01                 	cmpl	$0x1, -0x10(%rbp)
140a7a8df: 75 25                       	jne	0x140a7a906 <.text+0xa79906>
140a7a8e1: 48 8d 55 f8                 	leaq	-0x8(%rbp), %rdx
140a7a8e5: c6 45 6f 01                 	movb	$0x1, 0x6f(%rbp)
140a7a8e9: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
140a7a8ed: e8 de c9 9b 00              	callq	0x1414372d0 <.text+0x14362d0>
140a7a8f2: 48 b8 04 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffc, %rax # imm = 0x8000000000000004
140a7a8fc: 48 39 45 b0                 	cmpq	%rax, -0x50(%rbp)
140a7a900: 0f 85 b8 00 00 00           	jne	0x140a7a9be <.text+0xa799be>
140a7a906: 48 8b 45 f0                 	movq	-0x10(%rbp), %rax
140a7a90a: 48 8b 4d f8                 	movq	-0x8(%rbp), %rcx
140a7a90e: 48 8b 55 00                 	movq	(%rbp), %rdx
140a7a912: 4c 8b 45 08                 	movq	0x8(%rbp), %r8
140a7a916: 4c 8b 4d 10                 	movq	0x10(%rbp), %r9
140a7a91a: 4c 89 4d 40                 	movq	%r9, 0x40(%rbp)
140a7a91e: 44 8b 4d 18                 	movl	0x18(%rbp), %r9d
140a7a922: 44 89 4d 48                 	movl	%r9d, 0x48(%rbp)
140a7a926: 44 0f b7 4d 1c              	movzwl	0x1c(%rbp), %r9d
140a7a92b: 66 44 89 4d 4c              	movw	%r9w, 0x4c(%rbp)
140a7a930: 44 0f b6 4d 1e              	movzbl	0x1e(%rbp), %r9d
140a7a935: 44 88 4d 4e                 	movb	%r9b, 0x4e(%rbp)
140a7a939: 44 0f b6 4d 1f              	movzbl	0x1f(%rbp), %r9d
140a7a93e: 44 88 4d 4f                 	movb	%r9b, 0x4f(%rbp)
140a7a942: 48 89 55 30                 	movq	%rdx, 0x30(%rbp)
140a7a946: 4c 89 45 38                 	movq	%r8, 0x38(%rbp)
140a7a94a: 48 89 45 20                 	movq	%rax, 0x20(%rbp)
140a7a94e: 48 89 4d 28                 	movq	%rcx, 0x28(%rbp)
140a7a952: 41 8b 77 1c                 	movl	0x1c(%r15), %esi
140a7a956: 49 8b 7f 10                 	movq	0x10(%r15), %rdi
140a7a95a: 89 37                       	movl	%esi, (%rdi)
140a7a95c: 83 3b 02                    	cmpl	$0x2, (%rbx)
140a7a95f: 74 28                       	je	0x140a7a989 <.text+0xa79989>
140a7a961: 48 89 d9                    	movq	%rbx, %rcx
140a7a964: 48 89 5d 60                 	movq	%rbx, 0x60(%rbp)
140a7a968: e8 13 f9 e1 ff              	callq	0x14089a280 <.text+0x899280>
140a7a96d: 48 8b 5d 60                 	movq	0x60(%rbp), %rbx
140a7a971: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140a7a975: 48 85 d2                    	testq	%rdx, %rdx
140a7a978: 74 0f                       	je	0x140a7a989 <.text+0xa79989>
140a7a97a: 48 8b 4b 18                 	movq	0x18(%rbx), %rcx
140a7a97e: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140a7a984: e8 97 92 b9 ff              	callq	0x140613c20 <.text+0x612c20>
140a7a989: 0f 28 45 20                 	movaps	0x20(%rbp), %xmm0
140a7a98d: 0f 28 4d 30                 	movaps	0x30(%rbp), %xmm1
140a7a991: 0f 28 55 40                 	movaps	0x40(%rbp), %xmm2
140a7a995: 0f 11 53 20                 	movups	%xmm2, 0x20(%rbx)
140a7a999: 0f 11 4b 10                 	movups	%xmm1, 0x10(%rbx)
140a7a99d: 0f 11 03                    	movups	%xmm0, (%rbx)
140a7a9a0: 89 37                       	movl	%esi, (%rdi)
140a7a9a2: 48 c7 43 30 01 00 00 00     	movq	$0x1, 0x30(%rbx)
140a7a9aa: 48 81 c4 f8 00 00 00        	addq	$0xf8, %rsp
140a7a9b1: 5b                          	popq	%rbx
140a7a9b2: 5f                          	popq	%rdi
140a7a9b3: 5e                          	popq	%rsi
140a7a9b4: 41 5c                       	popq	%r12
140a7a9b6: 41 5d                       	popq	%r13
140a7a9b8: 41 5e                       	popq	%r14
140a7a9ba: 41 5f                       	popq	%r15
140a7a9bc: 5d                          	popq	%rbp
140a7a9bd: c3                          	retq
140a7a9be: 48 8b 45 e0                 	movq	-0x20(%rbp), %rax
140a7a9c2: 48 89 45 50                 	movq	%rax, 0x50(%rbp)
140a7a9c6: 0f 10 45 b0                 	movups	-0x50(%rbp), %xmm0
140a7a9ca: 0f 10 4d c0                 	movups	-0x40(%rbp), %xmm1
140a7a9ce: 0f 10 55 d0                 	movups	-0x30(%rbp), %xmm2
140a7a9d2: 0f 29 55 40                 	movaps	%xmm2, 0x40(%rbp)
140a7a9d6: 0f 29 4d 30                 	movaps	%xmm1, 0x30(%rbp)
140a7a9da: 0f 29 45 20                 	movaps	%xmm0, 0x20(%rbp)
140a7a9de: 48 8d 05 db 81 05 02        	leaq	0x20581db(%rip), %rax   # 0x142ad2bc0
140a7a9e5: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140a7a9ea: 48 8d 0d 0f 5c 05 02        	leaq	0x2055c0f(%rip), %rcx   # 0x142ad0600
140a7a9f1: 4c 8d 0d c8 5b 05 02        	leaq	0x2055bc8(%rip), %r9    # 0x142ad05c0
140a7a9f8: 4c 8d 45 20                 	leaq	0x20(%rbp), %r8
140a7a9fc: ba 2b 00 00 00              	movl	$0x2b, %edx
140a7aa01: e8 5a ed e5 01              	callq	0x1428d9760 <.text+0x28d8760>
140a7aa06: 0f 0b                       	ud2
140a7aa08: 4c 8d 05 a1 5d 05 02        	leaq	0x2055da1(%rip), %r8    # 0x142ad07b0
140a7aa0f: b9 01 00 00 00              	movl	$0x1, %ecx
140a7aa14: ba 11 00 00 00              	movl	$0x11, %edx
140a7aa19: e8 c5 e5 e5 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140a7aa1e: cc                          	int3
140a7aa1f: 90                          	nop
140a7aa20: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a7aa25: 55                          	pushq	%rbp
140a7aa26: 41 57                       	pushq	%r15
140a7aa28: 41 56                       	pushq	%r14
140a7aa2a: 41 55                       	pushq	%r13
140a7aa2c: 41 54                       	pushq	%r12
140a7aa2e: 56                          	pushq	%rsi
140a7aa2f: 57                          	pushq	%rdi
140a7aa30: 53                          	pushq	%rbx
140a7aa31: 48 83 ec 28                 	subq	$0x28, %rsp
140a7aa35: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a7aa3c: 48 8b 7d e8                 	movq	-0x18(%rbp), %rdi
140a7aa40: 48 85 ff                    	testq	%rdi, %rdi
140a7aa43: 48 8b 75 60                 	movq	0x60(%rbp), %rsi
140a7aa47: 74 25                       	je	0x140a7aa6e <.text+0xa79a6e>
140a7aa49: 48 83 c6 50                 	addq	$0x50, %rsi
140a7aa4d: 48 f7 df                    	negq	%rdi
140a7aa50: eb 09                       	jmp	0x140a7aa5b <.text+0xa79a5b>
140a7aa52: 48 83 c6 50                 	addq	$0x50, %rsi
140a7aa56: 48 ff cf                    	decq	%rdi
140a7aa59: 74 13                       	je	0x140a7aa6e <.text+0xa79a6e>
140a7aa5b: 48 8b 06                    	movq	(%rsi), %rax
140a7aa5e: f0                          	lock
140a7aa5f: 48 ff 08                    	decq	(%rax)
140a7aa62: 75 ee                       	jne	0x140a7aa52 <.text+0xa79a52>
140a7aa64: 48 89 f1                    	movq	%rsi, %rcx
140a7aa67: e8 c4 40 bd ff              	callq	0x14064eb30 <.text+0x64db30>
140a7aa6c: eb e4                       	jmp	0x140a7aa52 <.text+0xa79a52>
140a7aa6e: 48 83 c4 28                 	addq	$0x28, %rsp
140a7aa72: 5b                          	popq	%rbx
140a7aa73: 5f                          	popq	%rdi
140a7aa74: 5e                          	popq	%rsi
140a7aa75: 41 5c                       	popq	%r12
140a7aa77: 41 5d                       	popq	%r13
140a7aa79: 41 5e                       	popq	%r14
140a7aa7b: 41 5f                       	popq	%r15
140a7aa7d: 5d                          	popq	%rbp
140a7aa7e: c3                          	retq
140a7aa7f: 90                          	nop
140a7aa80: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a7aa85: 55                          	pushq	%rbp
140a7aa86: 41 57                       	pushq	%r15
140a7aa88: 41 56                       	pushq	%r14
140a7aa8a: 41 55                       	pushq	%r13
140a7aa8c: 41 54                       	pushq	%r12
140a7aa8e: 56                          	pushq	%rsi
140a7aa8f: 57                          	pushq	%rdi
140a7aa90: 53                          	pushq	%rbx
140a7aa91: 48 83 ec 28                 	subq	$0x28, %rsp
140a7aa95: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a7aa9c: c6 45 6f 01                 	movb	$0x1, 0x6f(%rbp)
140a7aaa0: 48 8d 4d 20                 	leaq	0x20(%rbp), %rcx
140a7aaa4: e8 07 7c fd ff              	callq	0x140a526b0 <.text+0xa516b0>
140a7aaa9: c6 45 6f 01                 	movb	$0x1, 0x6f(%rbp)
140a7aaad: 48 83 c4 28                 	addq	$0x28, %rsp
140a7aab1: 5b                          	popq	%rbx
140a7aab2: 5f                          	popq	%rdi
140a7aab3: 5e                          	popq	%rsi
140a7aab4: 41 5c                       	popq	%r12
140a7aab6: 41 5d                       	popq	%r13
140a7aab8: 41 5e                       	popq	%r14
140a7aaba: 41 5f                       	popq	%r15
140a7aabc: 5d                          	popq	%rbp
140a7aabd: c3                          	retq
140a7aabe: 66 90                       	nop
140a7aac0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a7aac5: 55                          	pushq	%rbp
140a7aac6: 41 57                       	pushq	%r15
140a7aac8: 41 56                       	pushq	%r14
140a7aaca: 41 55                       	pushq	%r13
140a7aacc: 41 54                       	pushq	%r12
140a7aace: 56                          	pushq	%rsi
140a7aacf: 57                          	pushq	%rdi
140a7aad0: 53                          	pushq	%rbx
140a7aad1: 48 83 ec 28                 	subq	$0x28, %rsp
140a7aad5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a7aadc: 48 8b 45 60                 	movq	0x60(%rbp), %rax
140a7aae0: 48 89 c6                    	movq	%rax, %rsi
140a7aae3: 48 8b 50 10                 	movq	0x10(%rax), %rdx
140a7aae7: 48 85 d2                    	testq	%rdx, %rdx
140a7aaea: 74 0f                       	je	0x140a7aafb <.text+0xa79afb>
140a7aaec: 48 8b 4e 18                 	movq	0x18(%rsi), %rcx
140a7aaf0: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140a7aaf6: e8 25 91 b9 ff              	callq	0x140613c20 <.text+0x612c20>
140a7aafb: 0f 28 45 20                 	movaps	0x20(%rbp), %xmm0
140a7aaff: 0f 28 4d 30                 	movaps	0x30(%rbp), %xmm1
140a7ab03: 0f 28 55 40                 	movaps	0x40(%rbp), %xmm2
140a7ab07: 0f 11 56 20                 	movups	%xmm2, 0x20(%rsi)
140a7ab0b: 0f 11 4e 10                 	movups	%xmm1, 0x10(%rsi)
140a7ab0f: 0f 11 06                    	movups	%xmm0, (%rsi)
140a7ab12: c6 45 6f 00                 	movb	$0x0, 0x6f(%rbp)
140a7ab16: 48 83 c4 28                 	addq	$0x28, %rsp
140a7ab1a: 5b                          	popq	%rbx
140a7ab1b: 5f                          	popq	%rdi
140a7ab1c: 5e                          	popq	%rsi
140a7ab1d: 41 5c                       	popq	%r12
140a7ab1f: 41 5d                       	popq	%r13
140a7ab21: 41 5e                       	popq	%r14
140a7ab23: 41 5f                       	popq	%r15
140a7ab25: 5d                          	popq	%rbp
140a7ab26: c3                          	retq
140a7ab27: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
140a7ab30: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a7ab35: 55                          	pushq	%rbp
140a7ab36: 41 57                       	pushq	%r15
140a7ab38: 41 56                       	pushq	%r14
140a7ab3a: 41 55                       	pushq	%r13
140a7ab3c: 41 54                       	pushq	%r12
140a7ab3e: 56                          	pushq	%rsi
140a7ab3f: 57                          	pushq	%rdi
140a7ab40: 53                          	pushq	%rbx
140a7ab41: 48 83 ec 28                 	subq	$0x28, %rsp
140a7ab45: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a7ab4c: 80 7d 6f 00                 	cmpb	$0x0, 0x6f(%rbp)
140a7ab50: 75 11                       	jne	0x140a7ab63 <.text+0xa79b63>
140a7ab52: 48 83 c4 28                 	addq	$0x28, %rsp
140a7ab56: 5b                          	popq	%rbx
140a7ab57: 5f                          	popq	%rdi
140a7ab58: 5e                          	popq	%rsi
140a7ab59: 41 5c                       	popq	%r12
140a7ab5b: 41 5d                       	popq	%r13
140a7ab5d: 41 5e                       	popq	%r14
140a7ab5f: 41 5f                       	popq	%r15
140a7ab61: 5d                          	popq	%rbp
140a7ab62: c3                          	retq
140a7ab63: 48 8d 4d f0                 	leaq	-0x10(%rbp), %rcx
140a7ab67: e8 14 fb 6c ff              	callq	0x14014a680 <.text+0x149680>
140a7ab6c: eb e4                       	jmp	0x140a7ab52 <.text+0xa79b52>
140a7ab6e: 66 90                       	nop
140a7ab70: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a7ab75: 55                          	pushq	%rbp
140a7ab76: 41 57                       	pushq	%r15
140a7ab78: 41 56                       	pushq	%r14
140a7ab7a: 41 55                       	pushq	%r13
140a7ab7c: 41 54                       	pushq	%r12
140a7ab7e: 56                          	pushq	%rsi
140a7ab7f: 57                          	pushq	%rdi
140a7ab80: 53                          	pushq	%rbx
140a7ab81: 48 83 ec 28                 	subq	$0x28, %rsp
140a7ab85: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a7ab8c: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
140a7ab90: e8 9b 20 2d 00              	callq	0x140d4cc30 <.text+0xd4bc30>
140a7ab95: 90                          	nop
140a7ab96: 48 83 c4 28                 	addq	$0x28, %rsp
140a7ab9a: 5b                          	popq	%rbx
140a7ab9b: 5f                          	popq	%rdi
140a7ab9c: 5e                          	popq	%rsi
140a7ab9d: 41 5c                       	popq	%r12
140a7ab9f: 41 5d                       	popq	%r13
140a7aba1: 41 5e                       	popq	%r14
140a7aba3: 41 5f                       	popq	%r15
140a7aba5: 5d                          	popq	%rbp
140a7aba6: c3                          	retq

D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1408b3510: 41 57                       	pushq	%r15
1408b3512: 41 56                       	pushq	%r14
1408b3514: 41 55                       	pushq	%r13
1408b3516: 41 54                       	pushq	%r12
1408b3518: 56                          	pushq	%rsi
1408b3519: 57                          	pushq	%rdi
1408b351a: 55                          	pushq	%rbp
1408b351b: 53                          	pushq	%rbx
1408b351c: 48 83 ec 58                 	subq	$0x58, %rsp
1408b3520: 48 8b 01                    	movq	(%rcx), %rax
1408b3523: 48 8b 59 08                 	movq	0x8(%rcx), %rbx
1408b3527: 4c 8b b0 f8 00 00 00        	movq	0xf8(%rax), %r14
1408b352e: 48 8b 88 00 01 00 00        	movq	0x100(%rax), %rcx
1408b3535: 4d 8d 3c 8e                 	leaq	(%r14,%rcx,4), %r15
1408b3539: 48 8b 0a                    	movq	(%rdx), %rcx
1408b353c: 48 89 4c 24 28              	movq	%rcx, 0x28(%rsp)
1408b3541: 48 8b 4a 08                 	movq	0x8(%rdx), %rcx
1408b3545: 48 89 4c 24 20              	movq	%rcx, 0x20(%rsp)
1408b354a: 48 8b 6a 18                 	movq	0x18(%rdx), %rbp
1408b354e: 48 8b 72 10                 	movq	0x10(%rdx), %rsi
1408b3552: 80 b8 08 01 00 00 00        	cmpb	$0x0, 0x108(%rax)
1408b3559: 0f 84 90 00 00 00           	je	0x1408b35ef <.text+0x8b25ef>
1408b355f: 41 bc 08 00 00 00           	movl	$0x8, %r12d
1408b3565: 31 ff                       	xorl	%edi, %edi
1408b3567: 45 31 ed                    	xorl	%r13d, %r13d
1408b356a: 49 39 fd                    	cmpq	%rdi, %r13
1408b356d: 75 31                       	jne	0x1408b35a0 <.text+0x8b25a0>
1408b356f: 90                          	nop
1408b3570: 4d 39 fe                    	cmpq	%r15, %r14
1408b3573: 0f 84 08 01 00 00           	je	0x1408b3681 <.text+0x8b2681>
1408b3579: 41 8b 0e                    	movl	(%r14), %ecx
1408b357c: 49 83 c6 04                 	addq	$0x4, %r14
1408b3580: 48 8b 83 a8 01 00 00        	movq	0x1a8(%rbx), %rax
1408b3587: 48 8d 0c c9                 	leaq	(%rcx,%rcx,8), %rcx
1408b358b: 48 8b 7c c8 10              	movq	0x10(%rax,%rcx,8), %rdi
1408b3590: 48 85 ff                    	testq	%rdi, %rdi
1408b3593: 74 db                       	je	0x1408b3570 <.text+0x8b2570>
1408b3595: 48 8d 04 c8                 	leaq	(%rax,%rcx,8), %rax
1408b3599: 4c 8b 60 08                 	movq	0x8(%rax), %r12
1408b359d: 45 31 ed                    	xorl	%r13d, %r13d
1408b35a0: 4b 8b 04 ec                 	movq	(%r12,%r13,8), %rax
1408b35a4: 48 89 c1                    	movq	%rax, %rcx
1408b35a7: 48 c1 e9 20                 	shrq	$0x20, %rcx
1408b35ab: 0f 84 d0 00 00 00           	je	0x1408b3681 <.text+0x8b2681>
1408b35b1: 49 ff c5                    	incq	%r13
1408b35b4: 48 89 44 24 50              	movq	%rax, 0x50(%rsp)
1408b35b9: 48 8b 44 24 28              	movq	0x28(%rsp), %rax
1408b35be: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
1408b35c3: 48 8b 44 24 20              	movq	0x20(%rsp), %rax
1408b35c8: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
1408b35cd: 48 89 74 24 40              	movq	%rsi, 0x40(%rsp)
1408b35d2: 48 89 6c 24 48              	movq	%rbp, 0x48(%rsp)
1408b35d7: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
1408b35dc: 48 8d 15 8d bb 1e 02        	leaq	0x21ebb8d(%rip), %rdx   # 0x142a9f170
1408b35e3: e8 88 11 46 00              	callq	0x140d14770 <.text+0xd13770>
1408b35e8: 49 39 fd                    	cmpq	%rdi, %r13
1408b35eb: 74 83                       	je	0x1408b3570 <.text+0x8b2570>
1408b35ed: eb b1                       	jmp	0x1408b35a0 <.text+0x8b25a0>
1408b35ef: 41 bc 08 00 00 00           	movl	$0x8, %r12d
1408b35f5: 31 ff                       	xorl	%edi, %edi
1408b35f7: 45 31 ed                    	xorl	%r13d, %r13d
1408b35fa: 49 39 fd                    	cmpq	%rdi, %r13
1408b35fd: 75 30                       	jne	0x1408b362f <.text+0x8b262f>
1408b35ff: 90                          	nop
1408b3600: 4d 39 fe                    	cmpq	%r15, %r14
1408b3603: 74 7c                       	je	0x1408b3681 <.text+0x8b2681>
1408b3605: 41 8b 0e                    	movl	(%r14), %ecx
1408b3608: 49 83 c6 04                 	addq	$0x4, %r14
1408b360c: 48 8b 83 00 01 00 00        	movq	0x100(%rbx), %rax
1408b3613: 48 8d 0c 89                 	leaq	(%rcx,%rcx,4), %rcx
1408b3617: 48 c1 e1 05                 	shlq	$0x5, %rcx
1408b361b: 48 8b 7c 08 58              	movq	0x58(%rax,%rcx), %rdi
1408b3620: 48 85 ff                    	testq	%rdi, %rdi
1408b3623: 74 db                       	je	0x1408b3600 <.text+0x8b2600>
1408b3625: 48 01 c8                    	addq	%rcx, %rax
1408b3628: 4c 8b 60 50                 	movq	0x50(%rax), %r12
1408b362c: 45 31 ed                    	xorl	%r13d, %r13d
1408b362f: 4c 89 e8                    	movq	%r13, %rax
1408b3632: 48 c1 e0 04                 	shlq	$0x4, %rax
1408b3636: 49 8b 04 04                 	movq	(%r12,%rax), %rax
1408b363a: 48 89 c1                    	movq	%rax, %rcx
1408b363d: 48 c1 e9 20                 	shrq	$0x20, %rcx
1408b3641: 74 3e                       	je	0x1408b3681 <.text+0x8b2681>
1408b3643: 49 ff c5                    	incq	%r13
1408b3646: 48 89 44 24 50              	movq	%rax, 0x50(%rsp)
1408b364b: 48 8b 44 24 28              	movq	0x28(%rsp), %rax
1408b3650: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
1408b3655: 48 8b 44 24 20              	movq	0x20(%rsp), %rax
1408b365a: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
1408b365f: 48 89 74 24 40              	movq	%rsi, 0x40(%rsp)
1408b3664: 48 89 6c 24 48              	movq	%rbp, 0x48(%rsp)
1408b3669: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
1408b366e: 48 8d 15 fb ba 1e 02        	leaq	0x21ebafb(%rip), %rdx   # 0x142a9f170
1408b3675: e8 f6 10 46 00              	callq	0x140d14770 <.text+0xd13770>
1408b367a: 49 39 fd                    	cmpq	%rdi, %r13
1408b367d: 74 81                       	je	0x1408b3600 <.text+0x8b2600>
1408b367f: eb ae                       	jmp	0x1408b362f <.text+0x8b262f>
1408b3681: 48 83 c4 58                 	addq	$0x58, %rsp
1408b3685: 5b                          	popq	%rbx
1408b3686: 5d                          	popq	%rbp
1408b3687: 5f                          	popq	%rdi
1408b3688: 5e                          	popq	%rsi
1408b3689: 41 5c                       	popq	%r12
1408b368b: 41 5d                       	popq	%r13
1408b368d: 41 5e                       	popq	%r14
1408b368f: 41 5f                       	popq	%r15
1408b3691: c3                          	retq

D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a43840: 55                          	pushq	%rbp
140a43841: 56                          	pushq	%rsi
140a43842: 57                          	pushq	%rdi
140a43843: 48 81 ec c0 00 00 00        	subq	$0xc0, %rsp
140a4384a: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140a43852: 48 c7 45 38 fe ff ff ff     	movq	$-0x2, 0x38(%rbp)
140a4385a: 48 89 ce                    	movq	%rcx, %rsi
140a4385d: 48 c7 45 f0 04 00 00 00     	movq	$0x4, -0x10(%rbp)
140a43865: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
140a43869: 4c 8d 45 f0                 	leaq	-0x10(%rbp), %r8
140a4386d: 48 89 f2                    	movq	%rsi, %rdx
140a43870: e8 0b 6a e9 ff              	callq	0x1408da280 <.text+0x8d9280>
140a43875: 83 7d b0 0b                 	cmpl	$0xb, -0x50(%rbp)
140a43879: 75 7b                       	jne	0x140a438f6 <.text+0xa428f6>
140a4387b: 48 c7 86 20 01 00 00 00 00 00 00    	movq	$0x0, 0x120(%rsi)
140a43886: 31 c0                       	xorl	%eax, %eax
140a43888: 48 3b 46 28                 	cmpq	0x28(%rsi), %rax
140a4388c: 70 35                       	jo	0x140a438c3 <.text+0xa428c3>
140a4388e: 48 8d 7e 28                 	leaq	0x28(%rsi), %rdi
140a43892: 48 8b 46 40                 	movq	0x40(%rsi), %rax
140a43896: 48 89 c1                    	movq	%rax, %rcx
140a43899: 48 f7 d9                    	negq	%rcx
140a4389c: 70 19                       	jo	0x140a438b7 <.text+0xa428b7>
140a4389e: 73 17                       	jae	0x140a438b7 <.text+0xa428b7>
140a438a0: 48 8b 4e 48                 	movq	0x48(%rsi), %rcx
140a438a4: 48 c1 e0 03                 	shlq	$0x3, %rax
140a438a8: 48 8d 14 80                 	leaq	(%rax,%rax,4), %rdx
140a438ac: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140a438b2: e8 69 03 bd ff              	callq	0x140613c20 <.text+0x612c20>
140a438b7: 48 89 f9                    	movq	%rdi, %rcx
140a438ba: 48 89 7d 30                 	movq	%rdi, 0x30(%rbp)
140a438be: e8 cd c9 bb ff              	callq	0x140600290 <.text+0x5ff290>
140a438c3: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
140a438cd: 48 89 46 28                 	movq	%rax, 0x28(%rsi)
140a438d1: c7 86 40 01 00 00 00 00 00 00       	movl	$0x0, 0x140(%rsi)
140a438db: f2 0f 10 05 cd ea 06 02     	movsd	0x206eacd(%rip), %xmm0  # 0x142ab23b0
140a438e3: f2 0f 11 86 44 01 00 00     	movsd	%xmm0, 0x144(%rsi)
140a438eb: 48 81 c4 c0 00 00 00        	addq	$0xc0, %rsp
140a438f2: 5f                          	popq	%rdi
140a438f3: 5e                          	popq	%rsi
140a438f4: 5d                          	popq	%rbp
140a438f5: c3                          	retq
140a438f6: 0f 10 45 b0                 	movups	-0x50(%rbp), %xmm0
140a438fa: 0f 10 4d c0                 	movups	-0x40(%rbp), %xmm1
140a438fe: 0f 10 55 d0                 	movups	-0x30(%rbp), %xmm2
140a43902: 0f 10 5d e0                 	movups	-0x20(%rbp), %xmm3
140a43906: 0f 29 5d 20                 	movaps	%xmm3, 0x20(%rbp)
140a4390a: 0f 29 55 10                 	movaps	%xmm2, 0x10(%rbp)
140a4390e: 0f 29 4d 00                 	movaps	%xmm1, (%rbp)
140a43912: 0f 29 45 f0                 	movaps	%xmm0, -0x10(%rbp)
140a43916: 48 8d 05 63 a9 08 02        	leaq	0x208a963(%rip), %rax   # 0x142ace280
140a4391d: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140a43922: 48 8d 0d c7 a3 08 02        	leaq	0x208a3c7(%rip), %rcx   # 0x142acdcf0
140a43929: 4c 8d 0d a0 a3 08 02        	leaq	0x208a3a0(%rip), %r9    # 0x142acdcd0
140a43930: 4c 8d 45 f0                 	leaq	-0x10(%rbp), %r8
140a43934: ba 2b 00 00 00              	movl	$0x2b, %edx
140a43939: e8 22 5e e9 01              	callq	0x1428d9760 <.text+0x28d8760>
140a4393e: 0f 0b                       	ud2
140a43940: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a43945: 55                          	pushq	%rbp
140a43946: 56                          	pushq	%rsi
140a43947: 57                          	pushq	%rdi
140a43948: 48 83 ec 30                 	subq	$0x30, %rsp
140a4394c: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a43953: 48 8d 4d f0                 	leaq	-0x10(%rbp), %rcx
140a43957: e8 34 ac e4 ff              	callq	0x14088e590 <.text+0x88d590>
140a4395c: 90                          	nop
140a4395d: 48 83 c4 30                 	addq	$0x30, %rsp
140a43961: 5f                          	popq	%rdi
140a43962: 5e                          	popq	%rsi
140a43963: 5d                          	popq	%rbp
140a43964: c3                          	retq
140a43965: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
140a43970: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a43975: 55                          	pushq	%rbp
140a43976: 56                          	pushq	%rsi
140a43977: 57                          	pushq	%rdi
140a43978: 48 83 ec 30                 	subq	$0x30, %rsp
140a4397c: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a43983: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
140a4398d: 48 8b 4d 30                 	movq	0x30(%rbp), %rcx
140a43991: 48 89 01                    	movq	%rax, (%rcx)
140a43994: 48 83 c4 30                 	addq	$0x30, %rsp
140a43998: 5f                          	popq	%rdi
140a43999: 5e                          	popq	%rsi
140a4399a: 5d                          	popq	%rbp
140a4399b: c3                          	retq

D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1409ebe80: 48 83 ec 28                 	subq	$0x28, %rsp
1409ebe84: 48 8b 41 10                 	movq	0x10(%rcx), %rax
1409ebe88: 48 39 d0                    	cmpq	%rdx, %rax
1409ebe8b: 76 77                       	jbe	0x1409ebf04 <.text+0x9eaf04>
1409ebe8d: 48 8b 41 08                 	movq	0x8(%rcx), %rax
1409ebe91: 48 8d 0c 52                 	leaq	(%rdx,%rdx,2), %rcx
1409ebe95: 48 8b 44 c8 08              	movq	0x8(%rax,%rcx,8), %rax
1409ebe9a: 48 89 c1                    	movq	%rax, %rcx
1409ebe9d: 48 d1 e9                    	shrq	%rcx
1409ebea0: 48 ba 55 55 55 55 55 55 55 55       	movabsq	$0x5555555555555555, %rdx # imm = 0x5555555555555555
1409ebeaa: 48 21 ca                    	andq	%rcx, %rdx
1409ebead: 48 29 d0                    	subq	%rdx, %rax
1409ebeb0: 48 b9 33 33 33 33 33 33 33 33       	movabsq	$0x3333333333333333, %rcx # imm = 0x3333333333333333
1409ebeba: 48 89 c2                    	movq	%rax, %rdx
1409ebebd: 48 21 ca                    	andq	%rcx, %rdx
1409ebec0: 48 c1 e8 02                 	shrq	$0x2, %rax
1409ebec4: 48 21 c8                    	andq	%rcx, %rax
1409ebec7: 48 01 d0                    	addq	%rdx, %rax
1409ebeca: 48 89 c1                    	movq	%rax, %rcx
1409ebecd: 48 c1 e9 04                 	shrq	$0x4, %rcx
1409ebed1: 48 01 c1                    	addq	%rax, %rcx
1409ebed4: 48 b8 0f 0f 0f 0f 0f 0f 0f 0f       	movabsq	$0xf0f0f0f0f0f0f0f, %rax # imm = 0xF0F0F0F0F0F0F0F
1409ebede: 48 21 c8                    	andq	%rcx, %rax
1409ebee1: 48 b9 01 01 01 01 01 01 01 01       	movabsq	$0x101010101010101, %rcx # imm = 0x101010101010101
1409ebeeb: 48 0f af c8                 	imulq	%rax, %rcx
1409ebeef: 48 c1 e9 38                 	shrq	$0x38, %rcx
1409ebef3: f3 0f 2a c1                 	cvtsi2ss	%ecx, %xmm0
1409ebef7: f3 0f 59 05 01 8e 0d 02     	mulss	0x20d8e01(%rip), %xmm0  # 0x142ac4d00
1409ebeff: 48 83 c4 28                 	addq	$0x28, %rsp
1409ebf03: c3                          	retq
1409ebf04: 4c 8d 05 15 93 0d 02        	leaq	0x20d9315(%rip), %r8    # 0x142ac5220
1409ebf0b: 48 89 d1                    	movq	%rdx, %rcx
1409ebf0e: 48 89 c2                    	movq	%rax, %rdx
1409ebf11: e8 02 d6 ee 01              	callq	0x1428d9518 <.text+0x28d8518>
1409ebf16: cc                          	int3

D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1409ebf20: 55                          	pushq	%rbp
1409ebf21: 41 57                       	pushq	%r15
1409ebf23: 41 56                       	pushq	%r14
1409ebf25: 41 55                       	pushq	%r13
1409ebf27: 41 54                       	pushq	%r12
1409ebf29: 56                          	pushq	%rsi
1409ebf2a: 57                          	pushq	%rdi
1409ebf2b: 53                          	pushq	%rbx
1409ebf2c: 48 81 ec 28 02 00 00        	subq	$0x228, %rsp            # imm = 0x228
1409ebf33: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1409ebf3b: 44 0f 29 9d 90 01 00 00     	movaps	%xmm11, 0x190(%rbp)
1409ebf43: 44 0f 29 95 80 01 00 00     	movaps	%xmm10, 0x180(%rbp)
1409ebf4b: 44 0f 29 8d 70 01 00 00     	movaps	%xmm9, 0x170(%rbp)
1409ebf53: 44 0f 29 85 60 01 00 00     	movaps	%xmm8, 0x160(%rbp)
1409ebf5b: 0f 29 bd 50 01 00 00        	movaps	%xmm7, 0x150(%rbp)
1409ebf62: 0f 29 b5 40 01 00 00        	movaps	%xmm6, 0x140(%rbp)
1409ebf69: 48 c7 85 38 01 00 00 fe ff ff ff    	movq	$-0x2, 0x138(%rbp)
1409ebf74: 80 3d 15 8e c4 02 00        	cmpb	$0x0, 0x2c48e15(%rip)   # 0x143634d90
1409ebf7b: 0f 85 2d 04 00 00           	jne	0x1409ec3ae <.text+0x9eb3ae>
1409ebf81: 48 89 d7                    	movq	%rdx, %rdi
1409ebf84: 48 89 ce                    	movq	%rcx, %rsi
1409ebf87: 0f b6 05 7b 8c c4 02        	movzbl	0x2c48c7b(%rip), %eax   # 0x143634c09
1409ebf8e: b9 e0 15 00 00              	movl	$0x15e0, %ecx           # imm = 0x15E0
1409ebf93: ba 08 00 00 00              	movl	$0x8, %edx
1409ebf98: e8 73 7c c2 ff              	callq	0x140613c10 <.text+0x612c10>
1409ebf9d: 48 85 c0                    	testq	%rax, %rax
1409ebfa0: 0f 84 fd 06 00 00           	je	0x1409ec6a3 <.text+0x9eb6a3>
1409ebfa6: 48 89 bd e0 00 00 00        	movq	%rdi, 0xe0(%rbp)
1409ebfad: 48 c7 85 a0 00 00 00 64 00 00 00    	movq	$0x64, 0xa0(%rbp)
1409ebfb8: 48 89 85 a8 00 00 00        	movq	%rax, 0xa8(%rbp)
1409ebfbf: 48 c7 85 b0 00 00 00 00 00 00 00    	movq	$0x0, 0xb0(%rbp)
1409ebfca: 0f b6 05 38 8c c4 02        	movzbl	0x2c48c38(%rip), %eax   # 0x143634c09
1409ebfd1: b9 60 09 00 00              	movl	$0x960, %ecx            # imm = 0x960
1409ebfd6: ba 08 00 00 00              	movl	$0x8, %edx
1409ebfdb: e8 30 7c c2 ff              	callq	0x140613c10 <.text+0x612c10>
1409ebfe0: 48 85 c0                    	testq	%rax, %rax
1409ebfe3: 0f 84 d0 06 00 00           	je	0x1409ec6b9 <.text+0x9eb6b9>
1409ebfe9: 48 c7 85 b8 00 00 00 64 00 00 00    	movq	$0x64, 0xb8(%rbp)
1409ebff4: 48 89 85 c0 00 00 00        	movq	%rax, 0xc0(%rbp)
1409ebffb: 48 c7 85 c8 00 00 00 00 00 00 00    	movq	$0x0, 0xc8(%rbp)
1409ec006: 4c 8b 2e                    	movq	(%rsi), %r13
1409ec009: 31 c0                       	xorl	%eax, %eax
1409ec00b: f3 0f 10 35 5d 98 f3 01     	movss	0x1f3985d(%rip), %xmm6  # 0x142925870
1409ec013: f3 0f 10 3d 8d 85 f4 01     	movss	0x1f4858d(%rip), %xmm7  # 0x1429345a8
1409ec01b: f3 44 0f 10 05 e0 98 f3 01  	movss	0x1f398e0(%rip), %xmm8  # 0x142925904
1409ec024: f3 44 0f 10 0d e7 77 09 02  	movss	0x20977e7(%rip), %xmm9  # 0x142a83814
1409ec02d: 48 8d 1d 64 92 0d 02        	leaq	0x20d9264(%rip), %rbx   # 0x142ac5298
1409ec034: 4c 8d b5 d0 00 00 00        	leaq	0xd0(%rbp), %r14
1409ec03b: 48 8d 35 6e bb 28 02        	leaq	0x228bb6e(%rip), %rsi   # 0x142c77bb0
1409ec042: eb 13                       	jmp	0x1409ec057 <.text+0x9eb057>
1409ec044: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
1409ec050: 48 8b 85 e8 00 00 00        	movq	0xe8(%rbp), %rax
1409ec057: 48 83 f8 0a                 	cmpq	$0xa, %rax
1409ec05b: 0f 92 85 35 01 00 00        	setb	0x135(%rbp)
1409ec062: 0f 83 cc 02 00 00           	jae	0x1409ec334 <.text+0x9eb334>
1409ec068: 45 0f 57 d2                 	xorps	%xmm10, %xmm10
1409ec06c: f3 4c 0f 2a d0              	cvtsi2ss	%rax, %xmm10
1409ec071: 48 ff c0                    	incq	%rax
1409ec074: 48 89 85 e8 00 00 00        	movq	%rax, 0xe8(%rbp)
1409ec07b: f3 44 0f 58 d6              	addss	%xmm6, %xmm10
1409ec080: f3 44 0f 5e d7              	divss	%xmm7, %xmm10
1409ec085: 41 0f 28 c0                 	movaps	%xmm8, %xmm0
1409ec089: f3 41 0f 5c c2              	subss	%xmm10, %xmm0
1409ec08e: f3 41 0f 59 c1              	mulss	%xmm9, %xmm0
1409ec093: f3 45 0f 59 d1              	mulss	%xmm9, %xmm10
1409ec098: f3 44 0f 5c d0              	subss	%xmm0, %xmm10
1409ec09d: 45 31 ff                    	xorl	%r15d, %r15d
1409ec0a0: eb 63                       	jmp	0x1409ec105 <.text+0x9eb105>
1409ec0a2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
1409ec0b0: 4c 8b bd d8 00 00 00        	movq	0xd8(%rbp), %r15
1409ec0b7: 49 ff c7                    	incq	%r15
1409ec0ba: 41 0f 28 c0                 	movaps	%xmm8, %xmm0
1409ec0be: f3 41 0f 5c c3              	subss	%xmm11, %xmm0
1409ec0c3: f3 41 0f 59 c1              	mulss	%xmm9, %xmm0
1409ec0c8: f3 45 0f 59 d9              	mulss	%xmm9, %xmm11
1409ec0cd: f3 44 0f 5c d8              	subss	%xmm0, %xmm11
1409ec0d2: 48 8b 85 c0 00 00 00        	movq	0xc0(%rbp), %rax
1409ec0d9: 48 8d 0c 7f                 	leaq	(%rdi,%rdi,2), %rcx
1409ec0dd: f3 44 0f 11 1c c8           	movss	%xmm11, (%rax,%rcx,8)
1409ec0e3: f3 44 0f 11 54 c8 04        	movss	%xmm10, 0x4(%rax,%rcx,8)
1409ec0ea: 48 c7 44 c8 08 00 00 00 00  	movq	$0x0, 0x8(%rax,%rcx,8)
1409ec0f3: c7 44 c8 10 00 00 00 00     	movl	$0x0, 0x10(%rax,%rcx,8)
1409ec0fb: 48 ff c7                    	incq	%rdi
1409ec0fe: 48 89 bd c8 00 00 00        	movq	%rdi, 0xc8(%rbp)
1409ec105: 49 83 ff 09                 	cmpq	$0x9, %r15
1409ec109: 0f 87 41 ff ff ff           	ja	0x1409ec050 <.text+0x9eb050>
1409ec10f: 45 0f 57 db                 	xorps	%xmm11, %xmm11
1409ec113: f3 4d 0f 2a df              	cvtsi2ss	%r15, %xmm11
1409ec118: f3 44 0f 58 de              	addss	%xmm6, %xmm11
1409ec11d: f3 44 0f 5e df              	divss	%xmm7, %xmm11
1409ec122: 48 c7 85 d0 00 00 00 00 00 00 00    	movq	$0x0, 0xd0(%rbp)
1409ec12d: 4c 89 ad f0 00 00 00        	movq	%r13, 0xf0(%rbp)
1409ec134: 48 89 9d f8 00 00 00        	movq	%rbx, 0xf8(%rbp)
1409ec13b: 48 c7 85 00 01 00 00 14 00 00 00    	movq	$0x14, 0x100(%rbp)
1409ec146: 48 c7 85 08 01 00 00 00 00 00 00    	movq	$0x0, 0x108(%rbp)
1409ec151: 48 c7 85 10 01 00 00 04 00 00 00    	movq	$0x4, 0x110(%rbp)
1409ec15c: 4c 89 b5 18 01 00 00        	movq	%r14, 0x118(%rbp)
1409ec163: c6 85 34 01 00 00 00        	movb	$0x0, 0x134(%rbp)
1409ec16a: 48 8d 8d 80 00 00 00        	leaq	0x80(%rbp), %rcx
1409ec171: 48 8d 95 f0 00 00 00        	leaq	0xf0(%rbp), %rdx
1409ec178: 49 89 f0                    	movq	%rsi, %r8
1409ec17b: e8 30 ee 90 00              	callq	0x1412fafb0 <.text+0x12f9fb0>
1409ec180: 4c 8b a5 d0 00 00 00        	movq	0xd0(%rbp), %r12
1409ec187: 4d 85 e4                    	testq	%r12, %r12
1409ec18a: 0f 85 60 02 00 00           	jne	0x1409ec3f0 <.text+0x9eb3f0>
1409ec190: 4c 89 bd d8 00 00 00        	movq	%r15, 0xd8(%rbp)
1409ec197: 4c 8b bd 80 00 00 00        	movq	0x80(%rbp), %r15
1409ec19e: 48 8b 8d 88 00 00 00        	movq	0x88(%rbp), %rcx
1409ec1a5: 4c 89 f8                    	movq	%r15, %rax
1409ec1a8: 48 f7 d8                    	negq	%rax
1409ec1ab: 0f 80 82 02 00 00           	jo	0x1409ec433 <.text+0x9eb433>
1409ec1b1: 48 8b bd 90 00 00 00        	movq	0x90(%rbp), %rdi
1409ec1b8: 4c 89 7d 18                 	movq	%r15, 0x18(%rbp)
1409ec1bc: 48 89 4d 20                 	movq	%rcx, 0x20(%rbp)
1409ec1c0: 48 89 7d 28                 	movq	%rdi, 0x28(%rbp)
1409ec1c4: 48 85 ff                    	testq	%rdi, %rdi
1409ec1c7: 0f 84 9d 02 00 00           	je	0x1409ec46a <.text+0x9eb46a>
1409ec1cd: 83 39 01                    	cmpl	$0x1, (%rcx)
1409ec1d0: 75 7e                       	jne	0x1409ec250 <.text+0x9eb250>
1409ec1d2: 4c 89 ee                    	movq	%r13, %rsi
1409ec1d5: 4d 89 f5                    	movq	%r14, %r13
1409ec1d8: 49 89 de                    	movq	%rbx, %r14
1409ec1db: 49 89 cc                    	movq	%rcx, %r12
1409ec1de: 48 8d 51 08                 	leaq	0x8(%rcx), %rdx
1409ec1e2: 41 b9 04 00 00 00           	movl	$0x4, %r9d
1409ec1e8: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
1409ec1ef: 4c 8d 05 2a 6d 0e 02        	leaq	0x20e6d2a(%rip), %r8    # 0x142ad2f20
1409ec1f6: e8 65 b8 a4 00              	callq	0x141437a60 <.text+0x1436a60>
1409ec1fb: 48 8b 9d f0 00 00 00        	movq	0xf0(%rbp), %rbx
1409ec202: 48 b8 04 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffc, %rax # imm = 0x8000000000000004
1409ec20c: 48 39 c3                    	cmpq	%rax, %rbx
1409ec20f: 74 0c                       	je	0x1409ec21d <.text+0x9eb21d>
1409ec211: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
1409ec218: e8 93 64 06 00              	callq	0x140a526b0 <.text+0xa516b0>
1409ec21d: 48 b8 04 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffc, %rax # imm = 0x8000000000000004
1409ec227: 48 39 c3                    	cmpq	%rax, %rbx
1409ec22a: 0f 94 c0                    	sete	%al
1409ec22d: 4c 89 f3                    	movq	%r14, %rbx
1409ec230: 4d 89 ee                    	movq	%r13, %r14
1409ec233: 49 89 f5                    	movq	%rsi, %r13
1409ec236: 48 8d 35 73 b9 28 02        	leaq	0x228b973(%rip), %rsi   # 0x142c77bb0
1409ec23d: 4c 89 e1                    	movq	%r12, %rcx
1409ec240: eb 10                       	jmp	0x1409ec252 <.text+0x9eb252>
1409ec242: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
1409ec250: 31 c0                       	xorl	%eax, %eax
1409ec252: 4c 89 bd f0 00 00 00        	movq	%r15, 0xf0(%rbp)
1409ec259: 48 89 8d f8 00 00 00        	movq	%rcx, 0xf8(%rbp)
1409ec260: 48 89 bd 00 01 00 00        	movq	%rdi, 0x100(%rbp)
1409ec267: c7 85 10 01 00 00 00 ca 9a 3b       	movl	$0x3b9aca00, 0x110(%rbp) # imm = 0x3B9ACA00
1409ec271: 48 c7 85 18 01 00 00 00 00 00 00    	movq	$0x0, 0x118(%rbp)
1409ec27c: c7 85 20 01 00 00 32 00 00 00       	movl	$0x32, 0x120(%rbp)
1409ec286: c6 85 24 01 00 00 00        	movb	$0x0, 0x124(%rbp)
1409ec28d: 88 85 25 01 00 00           	movb	%al, 0x125(%rbp)
1409ec293: c6 85 26 01 00 00 00        	movb	$0x0, 0x126(%rbp)
1409ec29a: 48 8b bd b0 00 00 00        	movq	0xb0(%rbp), %rdi
1409ec2a1: 48 3b bd a0 00 00 00        	cmpq	0xa0(%rbp), %rdi
1409ec2a8: 75 13                       	jne	0x1409ec2bd <.text+0x9eb2bd>
1409ec2aa: 48 8d 8d a0 00 00 00        	leaq	0xa0(%rbp), %rcx
1409ec2b1: 48 8d 15 10 90 0d 02        	leaq	0x20d9010(%rip), %rdx   # 0x142ac52c8
1409ec2b8: e8 13 57 c4 ff              	callq	0x1406319d0 <.text+0x6309d0>
1409ec2bd: 48 8b 85 a8 00 00 00        	movq	0xa8(%rbp), %rax
1409ec2c4: 48 6b cf 38                 	imulq	$0x38, %rdi, %rcx
1409ec2c8: 0f 10 85 f0 00 00 00        	movups	0xf0(%rbp), %xmm0
1409ec2cf: 0f 10 8d 00 01 00 00        	movups	0x100(%rbp), %xmm1
1409ec2d6: 0f 10 95 10 01 00 00        	movups	0x110(%rbp), %xmm2
1409ec2dd: 48 8b 95 20 01 00 00        	movq	0x120(%rbp), %rdx
1409ec2e4: 48 89 54 08 30              	movq	%rdx, 0x30(%rax,%rcx)
1409ec2e9: 0f 11 54 08 20              	movups	%xmm2, 0x20(%rax,%rcx)
1409ec2ee: 0f 11 4c 08 10              	movups	%xmm1, 0x10(%rax,%rcx)
1409ec2f3: 0f 11 04 08                 	movups	%xmm0, (%rax,%rcx)
1409ec2f7: 48 ff c7                    	incq	%rdi
1409ec2fa: 48 89 bd b0 00 00 00        	movq	%rdi, 0xb0(%rbp)
1409ec301: 48 8b bd c8 00 00 00        	movq	0xc8(%rbp), %rdi
1409ec308: 48 3b bd b8 00 00 00        	cmpq	0xb8(%rbp), %rdi
1409ec30f: 0f 85 9b fd ff ff           	jne	0x1409ec0b0 <.text+0x9eb0b0>
1409ec315: c6 85 37 01 00 00 01        	movb	$0x1, 0x137(%rbp)
1409ec31c: 48 8d 8d b8 00 00 00        	leaq	0xb8(%rbp), %rcx
1409ec323: 48 8d 15 b6 8f 0d 02        	leaq	0x20d8fb6(%rip), %rdx   # 0x142ac52e0
1409ec32a: e8 81 6a b7 ff              	callq	0x140562db0 <.text+0x561db0>
1409ec32f: e9 7c fd ff ff              	jmp	0x1409ec0b0 <.text+0x9eb0b0>
1409ec334: 48 8b 85 b0 00 00 00        	movq	0xb0(%rbp), %rax
1409ec33b: 48 89 85 00 01 00 00        	movq	%rax, 0x100(%rbp)
1409ec342: 0f 10 85 a0 00 00 00        	movups	0xa0(%rbp), %xmm0
1409ec349: 0f 29 85 f0 00 00 00        	movaps	%xmm0, 0xf0(%rbp)
1409ec350: c6 85 37 01 00 00 01        	movb	$0x1, 0x137(%rbp)
1409ec357: 4c 8d 05 0a 8f 0d 02        	leaq	0x20d8f0a(%rip), %r8    # 0x142ac5268
1409ec35e: 48 8d 95 f0 00 00 00        	leaq	0xf0(%rbp), %rdx
1409ec365: 48 8b 8d e0 00 00 00        	movq	0xe0(%rbp), %rcx
1409ec36c: e8 ef 36 8f 00              	callq	0x1412dfa60 <.text+0x12dea60>
1409ec371: 48 8b 85 c8 00 00 00        	movq	0xc8(%rbp), %rax
1409ec378: 48 89 85 00 01 00 00        	movq	%rax, 0x100(%rbp)
1409ec37f: 0f 10 85 b8 00 00 00        	movups	0xb8(%rbp), %xmm0
1409ec386: 0f 29 85 f0 00 00 00        	movaps	%xmm0, 0xf0(%rbp)
1409ec38d: c6 85 37 01 00 00 00        	movb	$0x0, 0x137(%rbp)
1409ec394: 4c 8d 05 e5 8e 0d 02        	leaq	0x20d8ee5(%rip), %r8    # 0x142ac5280
1409ec39b: 48 8d 95 f0 00 00 00        	leaq	0xf0(%rbp), %rdx
1409ec3a2: 48 8b 8d e0 00 00 00        	movq	0xe0(%rbp), %rcx
1409ec3a9: e8 92 29 8f 00              	callq	0x1412ded40 <.text+0x12ddd40>
1409ec3ae: 0f 28 b5 40 01 00 00        	movaps	0x140(%rbp), %xmm6
1409ec3b5: 0f 28 bd 50 01 00 00        	movaps	0x150(%rbp), %xmm7
1409ec3bc: 44 0f 28 85 60 01 00 00     	movaps	0x160(%rbp), %xmm8
1409ec3c4: 44 0f 28 8d 70 01 00 00     	movaps	0x170(%rbp), %xmm9
1409ec3cc: 44 0f 28 95 80 01 00 00     	movaps	0x180(%rbp), %xmm10
1409ec3d4: 44 0f 28 9d 90 01 00 00     	movaps	0x190(%rbp), %xmm11
1409ec3dc: 48 81 c4 28 02 00 00        	addq	$0x228, %rsp            # imm = 0x228
1409ec3e3: 5b                          	popq	%rbx
1409ec3e4: 5f                          	popq	%rdi
1409ec3e5: 5e                          	popq	%rsi
1409ec3e6: 41 5c                       	popq	%r12
1409ec3e8: 41 5d                       	popq	%r13
1409ec3ea: 41 5e                       	popq	%r14
1409ec3ec: 41 5f                       	popq	%r15
1409ec3ee: 5d                          	popq	%rbp
1409ec3ef: c3                          	retq
1409ec3f0: 48 8b 85 88 00 00 00        	movq	0x88(%rbp), %rax
1409ec3f7: 48 89 85 e8 00 00 00        	movq	%rax, 0xe8(%rbp)
1409ec3fe: 4c 8b ad 90 00 00 00        	movq	0x90(%rbp), %r13
1409ec405: 4d 85 ed                    	testq	%r13, %r13
1409ec408: 75 75                       	jne	0x1409ec47f <.text+0x9eb47f>
1409ec40a: 48 8b 85 80 00 00 00        	movq	0x80(%rbp), %rax
1409ec411: 48 85 c0                    	testq	%rax, %rax
1409ec414: 74 1a                       	je	0x1409ec430 <.text+0x9eb430>
1409ec416: 48 c1 e0 04                 	shlq	$0x4, %rax
1409ec41a: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
1409ec41e: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1409ec424: 48 8b 8d e8 00 00 00        	movq	0xe8(%rbp), %rcx
1409ec42b: e8 f0 77 c2 ff              	callq	0x140613c20 <.text+0x612c20>
1409ec430: 4c 89 e1                    	movq	%r12, %rcx
1409ec433: 48 89 8d f0 00 00 00        	movq	%rcx, 0xf0(%rbp)
1409ec43a: 48 8d 05 6f 8e 0d 02        	leaq	0x20d8e6f(%rip), %rax   # 0x142ac52b0
1409ec441: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1409ec446: 48 8d 0d db 89 0d 02        	leaq	0x20d89db(%rip), %rcx   # 0x142ac4e28
1409ec44d: 4c 8d 0d b4 89 0d 02        	leaq	0x20d89b4(%rip), %r9    # 0x142ac4e08
1409ec454: 4c 8d 85 f0 00 00 00        	leaq	0xf0(%rbp), %r8
1409ec45b: ba 2b 00 00 00              	movl	$0x2b, %edx
1409ec460: e8 fb d2 ee 01              	callq	0x1428d9760 <.text+0x28d8760>
1409ec465: e9 6c 02 00 00              	jmp	0x1409ec6d6 <.text+0x9eb6d6>
1409ec46a: 4c 8d 05 97 6a 0e 02        	leaq	0x20e6a97(%rip), %r8    # 0x142ad2f08
1409ec471: 31 c9                       	xorl	%ecx, %ecx
1409ec473: 31 d2                       	xorl	%edx, %edx
1409ec475: e8 9e d0 ee 01              	callq	0x1428d9518 <.text+0x28d8518>
1409ec47a: e9 57 02 00 00              	jmp	0x1409ec6d6 <.text+0x9eb6d6>
1409ec47f: 45 31 f6                    	xorl	%r14d, %r14d
1409ec482: 48 8d 75 a8                 	leaq	-0x58(%rbp), %rsi
1409ec486: 48 8d 7d 30                 	leaq	0x30(%rbp), %rdi
1409ec48a: 48 8d 9d f0 00 00 00        	leaq	0xf0(%rbp), %rbx
1409ec491: 4c 8d 3d 08 92 0d 02        	leaq	0x20d9208(%rip), %r15   # 0x142ac56a0
1409ec498: 31 c0                       	xorl	%eax, %eax
1409ec49a: 4c 89 ad 98 00 00 00        	movq	%r13, 0x98(%rbp)
1409ec4a1: eb 1d                       	jmp	0x1409ec4c0 <.text+0x9eb4c0>
1409ec4a3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
1409ec4b0: 48 8b 85 d8 00 00 00        	movq	0xd8(%rbp), %rax
1409ec4b7: 4c 39 e8                    	cmpq	%r13, %rax
1409ec4ba: 0f 84 4a ff ff ff           	je	0x1409ec40a <.text+0x9eb40a>
1409ec4c0: 48 8d 0c 40                 	leaq	(%rax,%rax,2), %rcx
1409ec4c4: 48 c1 e1 04                 	shlq	$0x4, %rcx
1409ec4c8: 48 8b 95 e8 00 00 00        	movq	0xe8(%rbp), %rdx
1409ec4cf: 4c 8d 04 0a                 	leaq	(%rdx,%rcx), %r8
1409ec4d3: 48 ff c0                    	incq	%rax
1409ec4d6: 48 89 85 d8 00 00 00        	movq	%rax, 0xd8(%rbp)
1409ec4dd: 48 8b 44 0a 08              	movq	0x8(%rdx,%rcx), %rax
1409ec4e2: 48 83 3c 0a 00              	cmpq	$0x0, (%rdx,%rcx)
1409ec4e7: 48 c7 04 0a 00 00 00 00     	movq	$0x0, (%rdx,%rcx)
1409ec4ef: 0f 84 34 01 00 00           	je	0x1409ec629 <.text+0x9eb629>
1409ec4f5: 48 89 45 30                 	movq	%rax, 0x30(%rbp)
1409ec4f9: 4c 89 85 e0 00 00 00        	movq	%r8, 0xe0(%rbp)
1409ec500: 45 0f b6 40 28              	movzbl	0x28(%r8), %r8d
1409ec505: 41 80 f8 02                 	cmpb	$0x2, %r8b
1409ec509: 0f 84 f1 00 00 00           	je	0x1409ec600 <.text+0x9eb600>
1409ec50f: 41 80 e0 01                 	andb	$0x1, %r8b
1409ec513: 48 89 d9                    	movq	%rbx, %rcx
1409ec516: 48 89 fa                    	movq	%rdi, %rdx
1409ec519: e8 22 af a4 00              	callq	0x141437440 <.text+0x1436440>
1409ec51e: 48 8b 95 f0 00 00 00        	movq	0xf0(%rbp), %rdx
1409ec525: 48 b8 04 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffc, %rax # imm = 0x8000000000000004
1409ec52f: 48 39 c2                    	cmpq	%rax, %rdx
1409ec532: 0f 84 c8 00 00 00           	je	0x1409ec600 <.text+0x9eb600>
1409ec538: 48 b8 01 00 00 00 00 00 00 80       	movabsq	$-0x7fffffffffffffff, %rax # imm = 0x8000000000000001
1409ec542: 48 01 d0                    	addq	%rdx, %rax
1409ec545: 48 b9 04 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffc, %rcx # imm = 0x8000000000000004
1409ec54f: 48 39 ca                    	cmpq	%rcx, %rdx
1409ec552: 49 0f 4d c6                 	cmovgeq	%r14, %rax
1409ec556: 48 83 f8 03                 	cmpq	$0x3, %rax
1409ec55a: 0f 87 a0 00 00 00           	ja	0x1409ec600 <.text+0x9eb600>
1409ec560: 49 63 04 87                 	movslq	(%r15,%rax,4), %rax
1409ec564: 4c 01 f8                    	addq	%r15, %rax
1409ec567: ff e0                       	jmpq	*%rax
1409ec569: 48 85 d2                    	testq	%rdx, %rdx
1409ec56c: 74 12                       	je	0x1409ec580 <.text+0x9eb580>
1409ec56e: 48 8b 8d f8 00 00 00        	movq	0xf8(%rbp), %rcx
1409ec575: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1409ec57b: e8 a0 76 c2 ff              	callq	0x140613c20 <.text+0x612c20>
1409ec580: 48 8b 95 08 01 00 00        	movq	0x108(%rbp), %rdx
1409ec587: 48 85 d2                    	testq	%rdx, %rdx
1409ec58a: 74 74                       	je	0x1409ec600 <.text+0x9eb600>
1409ec58c: 48 8b 8d 10 01 00 00        	movq	0x110(%rbp), %rcx
1409ec593: eb 5e                       	jmp	0x1409ec5f3 <.text+0x9eb5f3>
1409ec595: 48 8b 8d f8 00 00 00        	movq	0xf8(%rbp), %rcx
1409ec59c: 48 8b 95 00 01 00 00        	movq	0x100(%rbp), %rdx
1409ec5a3: c6 01 00                    	movb	$0x0, (%rcx)
1409ec5a6: 48 85 d2                    	testq	%rdx, %rdx
1409ec5a9: 75 48                       	jne	0x1409ec5f3 <.text+0x9eb5f3>
1409ec5ab: eb 53                       	jmp	0x1409ec600 <.text+0x9eb600>
1409ec5ad: 48 8b 95 f8 00 00 00        	movq	0xf8(%rbp), %rdx
1409ec5b4: 48 85 d2                    	testq	%rdx, %rdx
1409ec5b7: 74 47                       	je	0x1409ec600 <.text+0x9eb600>
1409ec5b9: 48 8b 8d 00 01 00 00        	movq	0x100(%rbp), %rcx
1409ec5c0: eb 31                       	jmp	0x1409ec5f3 <.text+0x9eb5f3>
1409ec5c2: 48 8b 95 f8 00 00 00        	movq	0xf8(%rbp), %rdx
1409ec5c9: 48 85 d2                    	testq	%rdx, %rdx
1409ec5cc: 74 12                       	je	0x1409ec5e0 <.text+0x9eb5e0>
1409ec5ce: 48 8b 8d 00 01 00 00        	movq	0x100(%rbp), %rcx
1409ec5d5: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1409ec5db: e8 40 76 c2 ff              	callq	0x140613c20 <.text+0x612c20>
1409ec5e0: 48 8b 95 10 01 00 00        	movq	0x110(%rbp), %rdx
1409ec5e7: 48 85 d2                    	testq	%rdx, %rdx
1409ec5ea: 74 14                       	je	0x1409ec600 <.text+0x9eb600>
1409ec5ec: 48 8b 8d 18 01 00 00        	movq	0x118(%rbp), %rcx
1409ec5f3: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1409ec5f9: e8 22 76 c2 ff              	callq	0x140613c20 <.text+0x612c20>
1409ec5fe: 66 90                       	nop
1409ec600: 48 89 f1                    	movq	%rsi, %rcx
1409ec603: 48 89 fa                    	movq	%rdi, %rdx
1409ec606: e8 e5 b2 a4 00              	callq	0x1414378f0 <.text+0x14368f0>
1409ec60b: 48 b8 04 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffc, %rax # imm = 0x8000000000000004
1409ec615: 48 39 45 a8                 	cmpq	%rax, -0x58(%rbp)
1409ec619: 75 2f                       	jne	0x1409ec64a <.text+0x9eb64a>
1409ec61b: 4c 8b ad 98 00 00 00        	movq	0x98(%rbp), %r13
1409ec622: 4c 8b 85 e0 00 00 00        	movq	0xe0(%rbp), %r8
1409ec629: 49 8b 50 10                 	movq	0x10(%r8), %rdx
1409ec62d: 48 85 d2                    	testq	%rdx, %rdx
1409ec630: 0f 84 7a fe ff ff           	je	0x1409ec4b0 <.text+0x9eb4b0>
1409ec636: 49 8b 48 18                 	movq	0x18(%r8), %rcx
1409ec63a: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1409ec640: e8 db 75 c2 ff              	callq	0x140613c20 <.text+0x612c20>
1409ec645: e9 66 fe ff ff              	jmp	0x1409ec4b0 <.text+0x9eb4b0>
1409ec64a: 48 8b 45 d8                 	movq	-0x28(%rbp), %rax
1409ec64e: 48 89 85 20 01 00 00        	movq	%rax, 0x120(%rbp)
1409ec655: 0f 10 45 a8                 	movups	-0x58(%rbp), %xmm0
1409ec659: 0f 10 4d b8                 	movups	-0x48(%rbp), %xmm1
1409ec65d: 0f 10 55 c8                 	movups	-0x38(%rbp), %xmm2
1409ec661: 0f 29 95 10 01 00 00        	movaps	%xmm2, 0x110(%rbp)
1409ec668: 0f 29 8d 00 01 00 00        	movaps	%xmm1, 0x100(%rbp)
1409ec66f: 0f 29 85 f0 00 00 00        	movaps	%xmm0, 0xf0(%rbp)
1409ec676: 48 8d 05 cb c3 0a 02        	leaq	0x20ac3cb(%rip), %rax   # 0x142a98a48
1409ec67d: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1409ec682: 48 8d 0d b7 bd 0a 02        	leaq	0x20abdb7(%rip), %rcx   # 0x142a98440
1409ec689: 4c 8d 0d e0 bd 0a 02        	leaq	0x20abde0(%rip), %r9    # 0x142a98470
1409ec690: 4c 8d 85 f0 00 00 00        	leaq	0xf0(%rbp), %r8
1409ec697: ba 2b 00 00 00              	movl	$0x2b, %edx
1409ec69c: e8 bf d0 ee 01              	callq	0x1428d9760 <.text+0x28d8760>
1409ec6a1: eb 33                       	jmp	0x1409ec6d6 <.text+0x9eb6d6>
1409ec6a3: 4c 8d 05 8e 8b 0d 02        	leaq	0x20d8b8e(%rip), %r8    # 0x142ac5238
1409ec6aa: b9 08 00 00 00              	movl	$0x8, %ecx
1409ec6af: ba e0 15 00 00              	movl	$0x15e0, %edx           # imm = 0x15E0
1409ec6b4: e8 2a c9 ee 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
1409ec6b9: c6 85 36 01 00 00 01        	movb	$0x1, 0x136(%rbp)
1409ec6c0: 4c 8d 05 89 8b 0d 02        	leaq	0x20d8b89(%rip), %r8    # 0x142ac5250
1409ec6c7: b9 08 00 00 00              	movl	$0x8, %ecx
1409ec6cc: ba 60 09 00 00              	movl	$0x960, %edx            # imm = 0x960
1409ec6d1: e8 0d c9 ee 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
1409ec6d6: 0f 0b                       	ud2
1409ec6d8: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
1409ec6e0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409ec6e5: 55                          	pushq	%rbp
1409ec6e6: 41 57                       	pushq	%r15
1409ec6e8: 41 56                       	pushq	%r14
1409ec6ea: 41 55                       	pushq	%r13
1409ec6ec: 41 54                       	pushq	%r12
1409ec6ee: 56                          	pushq	%rsi
1409ec6ef: 57                          	pushq	%rdi
1409ec6f0: 53                          	pushq	%rbx
1409ec6f1: 48 81 ec 88 00 00 00        	subq	$0x88, %rsp
1409ec6f8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409ec6ff: 44 0f 29 5c 24 20           	movaps	%xmm11, 0x20(%rsp)
1409ec705: 44 0f 29 54 24 30           	movaps	%xmm10, 0x30(%rsp)
1409ec70b: 44 0f 29 4c 24 40           	movaps	%xmm9, 0x40(%rsp)
1409ec711: 44 0f 29 44 24 50           	movaps	%xmm8, 0x50(%rsp)
1409ec717: 0f 29 7c 24 60              	movaps	%xmm7, 0x60(%rsp)
1409ec71c: 0f 29 74 24 70              	movaps	%xmm6, 0x70(%rsp)
1409ec721: 80 bd 34 01 00 00 00        	cmpb	$0x0, 0x134(%rbp)
1409ec728: 75 1d                       	jne	0x1409ec747 <.text+0x9eb747>
1409ec72a: 48 83 bd d0 00 00 00 00     	cmpq	$0x0, 0xd0(%rbp)
1409ec732: 74 13                       	je	0x1409ec747 <.text+0x9eb747>
1409ec734: c6 85 37 01 00 00 01        	movb	$0x1, 0x137(%rbp)
1409ec73b: 48 8d 8d d0 00 00 00        	leaq	0xd0(%rbp), %rcx
1409ec742: e8 e9 04 36 00              	callq	0x140d4cc30 <.text+0xd4bc30>
1409ec747: c6 85 37 01 00 00 01        	movb	$0x1, 0x137(%rbp)
1409ec74e: 0f 28 74 24 70              	movaps	0x70(%rsp), %xmm6
1409ec753: 0f 28 7c 24 60              	movaps	0x60(%rsp), %xmm7
1409ec758: 44 0f 28 44 24 50           	movaps	0x50(%rsp), %xmm8
1409ec75e: 44 0f 28 4c 24 40           	movaps	0x40(%rsp), %xmm9
1409ec764: 44 0f 28 54 24 30           	movaps	0x30(%rsp), %xmm10
1409ec76a: 44 0f 28 5c 24 20           	movaps	0x20(%rsp), %xmm11
1409ec770: 48 81 c4 88 00 00 00        	addq	$0x88, %rsp
1409ec777: 5b                          	popq	%rbx
1409ec778: 5f                          	popq	%rdi
1409ec779: 5e                          	popq	%rsi
1409ec77a: 41 5c                       	popq	%r12
1409ec77c: 41 5d                       	popq	%r13
1409ec77e: 41 5e                       	popq	%r14
1409ec780: 41 5f                       	popq	%r15
1409ec782: 5d                          	popq	%rbp
1409ec783: c3                          	retq
1409ec784: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
1409ec790: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409ec795: 55                          	pushq	%rbp
1409ec796: 41 57                       	pushq	%r15
1409ec798: 41 56                       	pushq	%r14
1409ec79a: 41 55                       	pushq	%r13
1409ec79c: 41 54                       	pushq	%r12
1409ec79e: 56                          	pushq	%rsi
1409ec79f: 57                          	pushq	%rdi
1409ec7a0: 53                          	pushq	%rbx
1409ec7a1: 48 81 ec 88 00 00 00        	subq	$0x88, %rsp
1409ec7a8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409ec7af: 44 0f 29 5c 24 20           	movaps	%xmm11, 0x20(%rsp)
1409ec7b5: 44 0f 29 54 24 30           	movaps	%xmm10, 0x30(%rsp)
1409ec7bb: 44 0f 29 4c 24 40           	movaps	%xmm9, 0x40(%rsp)
1409ec7c1: 44 0f 29 44 24 50           	movaps	%xmm8, 0x50(%rsp)
1409ec7c7: 0f 29 7c 24 60              	movaps	%xmm7, 0x60(%rsp)
1409ec7cc: 0f 29 74 24 70              	movaps	%xmm6, 0x70(%rsp)
1409ec7d1: 48 8b 85 e0 00 00 00        	movq	0xe0(%rbp), %rax
1409ec7d8: 48 8b 50 10                 	movq	0x10(%rax), %rdx
1409ec7dc: 48 85 d2                    	testq	%rdx, %rdx
1409ec7df: 74 0f                       	je	0x1409ec7f0 <.text+0x9eb7f0>
1409ec7e1: 48 8b 48 18                 	movq	0x18(%rax), %rcx
1409ec7e5: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1409ec7eb: e8 30 74 c2 ff              	callq	0x140613c20 <.text+0x612c20>
1409ec7f0: 4c 8b ad 98 00 00 00        	movq	0x98(%rbp), %r13
1409ec7f7: 48 8b 85 d8 00 00 00        	movq	0xd8(%rbp), %rax
1409ec7fe: 4c 39 e8                    	cmpq	%r13, %rax
1409ec801: 75 36                       	jne	0x1409ec839 <.text+0x9eb839>
1409ec803: 0f 28 74 24 70              	movaps	0x70(%rsp), %xmm6
1409ec808: 0f 28 7c 24 60              	movaps	0x60(%rsp), %xmm7
1409ec80d: 44 0f 28 44 24 50           	movaps	0x50(%rsp), %xmm8
1409ec813: 44 0f 28 4c 24 40           	movaps	0x40(%rsp), %xmm9
1409ec819: 44 0f 28 54 24 30           	movaps	0x30(%rsp), %xmm10
1409ec81f: 44 0f 28 5c 24 20           	movaps	0x20(%rsp), %xmm11
1409ec825: 48 81 c4 88 00 00 00        	addq	$0x88, %rsp
1409ec82c: 5b                          	popq	%rbx
1409ec82d: 5f                          	popq	%rdi
1409ec82e: 5e                          	popq	%rsi
1409ec82f: 41 5c                       	popq	%r12
1409ec831: 41 5d                       	popq	%r13
1409ec833: 41 5e                       	popq	%r14
1409ec835: 41 5f                       	popq	%r15
1409ec837: 5d                          	popq	%rbp
1409ec838: c3                          	retq
1409ec839: 48 8d 75 e0                 	leaq	-0x20(%rbp), %rsi
1409ec83d: 48 8d 7d 38                 	leaq	0x38(%rbp), %rdi
1409ec841: 48 8d 5d 40                 	leaq	0x40(%rbp), %rbx
1409ec845: 4c 8d 3d 64 8e 0d 02        	leaq	0x20d8e64(%rip), %r15   # 0x142ac56b0
1409ec84c: eb 0d                       	jmp	0x1409ec85b <.text+0x9eb85b>
1409ec84e: 66 90                       	nop
1409ec850: 4c 89 f0                    	movq	%r14, %rax
1409ec853: 48 ff c0                    	incq	%rax
1409ec856: 4c 39 e8                    	cmpq	%r13, %rax
1409ec859: 74 a8                       	je	0x1409ec803 <.text+0x9eb803>
1409ec85b: 49 89 c6                    	movq	%rax, %r14
1409ec85e: 48 8d 0c 40                 	leaq	(%rax,%rax,2), %rcx
1409ec862: 48 c1 e1 04                 	shlq	$0x4, %rcx
1409ec866: 48 8b 95 e8 00 00 00        	movq	0xe8(%rbp), %rdx
1409ec86d: 4c 8d 24 0a                 	leaq	(%rdx,%rcx), %r12
1409ec871: 48 8b 44 0a 08              	movq	0x8(%rdx,%rcx), %rax
1409ec876: 48 83 3c 0a 00              	cmpq	$0x0, (%rdx,%rcx)
1409ec87b: 48 c7 04 0a 00 00 00 00     	movq	$0x0, (%rdx,%rcx)
1409ec883: 0f 84 02 01 00 00           	je	0x1409ec98b <.text+0x9eb98b>
1409ec889: 48 89 45 38                 	movq	%rax, 0x38(%rbp)
1409ec88d: 45 0f b6 44 24 28           	movzbl	0x28(%r12), %r8d
1409ec893: 41 80 f8 02                 	cmpb	$0x2, %r8b
1409ec897: 0f 84 d3 00 00 00           	je	0x1409ec970 <.text+0x9eb970>
1409ec89d: 41 80 e0 01                 	andb	$0x1, %r8b
1409ec8a1: 48 89 d9                    	movq	%rbx, %rcx
1409ec8a4: 48 89 fa                    	movq	%rdi, %rdx
1409ec8a7: e8 94 ab a4 00              	callq	0x141437440 <.text+0x1436440>
1409ec8ac: 48 8b 55 40                 	movq	0x40(%rbp), %rdx
1409ec8b0: 48 b8 04 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffc, %rax # imm = 0x8000000000000004
1409ec8ba: 48 39 c2                    	cmpq	%rax, %rdx
1409ec8bd: 0f 84 ad 00 00 00           	je	0x1409ec970 <.text+0x9eb970>
1409ec8c3: 48 b8 01 00 00 00 00 00 00 80       	movabsq	$-0x7fffffffffffffff, %rax # imm = 0x8000000000000001
1409ec8cd: 48 01 d0                    	addq	%rdx, %rax
1409ec8d0: 48 b9 04 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffc, %rcx # imm = 0x8000000000000004
1409ec8da: 48 39 ca                    	cmpq	%rcx, %rdx
1409ec8dd: b9 00 00 00 00              	movl	$0x0, %ecx
1409ec8e2: 48 0f 4d c1                 	cmovgeq	%rcx, %rax
1409ec8e6: 48 83 f8 03                 	cmpq	$0x3, %rax
1409ec8ea: 0f 87 80 00 00 00           	ja	0x1409ec970 <.text+0x9eb970>
1409ec8f0: 49 63 04 87                 	movslq	(%r15,%rax,4), %rax
1409ec8f4: 4c 01 f8                    	addq	%r15, %rax
1409ec8f7: ff e0                       	jmpq	*%rax
1409ec8f9: 48 85 d2                    	testq	%rdx, %rdx
1409ec8fc: 74 0f                       	je	0x1409ec90d <.text+0x9eb90d>
1409ec8fe: 48 8b 4d 48                 	movq	0x48(%rbp), %rcx
1409ec902: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1409ec908: e8 13 73 c2 ff              	callq	0x140613c20 <.text+0x612c20>
1409ec90d: 48 8b 55 58                 	movq	0x58(%rbp), %rdx
1409ec911: 48 85 d2                    	testq	%rdx, %rdx
1409ec914: 74 5a                       	je	0x1409ec970 <.text+0x9eb970>
1409ec916: 48 8b 4d 60                 	movq	0x60(%rbp), %rcx
1409ec91a: eb 46                       	jmp	0x1409ec962 <.text+0x9eb962>
1409ec91c: 48 8b 4d 48                 	movq	0x48(%rbp), %rcx
1409ec920: 48 8b 55 50                 	movq	0x50(%rbp), %rdx
1409ec924: c6 01 00                    	movb	$0x0, (%rcx)
1409ec927: 48 85 d2                    	testq	%rdx, %rdx
1409ec92a: 75 36                       	jne	0x1409ec962 <.text+0x9eb962>
1409ec92c: eb 42                       	jmp	0x1409ec970 <.text+0x9eb970>
1409ec92e: 48 8b 55 48                 	movq	0x48(%rbp), %rdx
1409ec932: 48 85 d2                    	testq	%rdx, %rdx
1409ec935: 74 39                       	je	0x1409ec970 <.text+0x9eb970>
1409ec937: 48 8b 4d 50                 	movq	0x50(%rbp), %rcx
1409ec93b: eb 25                       	jmp	0x1409ec962 <.text+0x9eb962>
1409ec93d: 48 8b 55 48                 	movq	0x48(%rbp), %rdx
1409ec941: 48 85 d2                    	testq	%rdx, %rdx
1409ec944: 74 0f                       	je	0x1409ec955 <.text+0x9eb955>
1409ec946: 48 8b 4d 50                 	movq	0x50(%rbp), %rcx
1409ec94a: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1409ec950: e8 cb 72 c2 ff              	callq	0x140613c20 <.text+0x612c20>
1409ec955: 48 8b 55 60                 	movq	0x60(%rbp), %rdx
1409ec959: 48 85 d2                    	testq	%rdx, %rdx
1409ec95c: 74 12                       	je	0x1409ec970 <.text+0x9eb970>
1409ec95e: 48 8b 4d 68                 	movq	0x68(%rbp), %rcx
1409ec962: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1409ec968: e8 b3 72 c2 ff              	callq	0x140613c20 <.text+0x612c20>
1409ec96d: 0f 1f 00                    	nopl	(%rax)
1409ec970: 48 89 f1                    	movq	%rsi, %rcx
1409ec973: 48 89 fa                    	movq	%rdi, %rdx
1409ec976: e8 75 af a4 00              	callq	0x1414378f0 <.text+0x14368f0>
1409ec97b: 48 b8 04 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffc, %rax # imm = 0x8000000000000004
1409ec985: 48 39 45 e0                 	cmpq	%rax, -0x20(%rbp)
1409ec989: 75 23                       	jne	0x1409ec9ae <.text+0x9eb9ae>
1409ec98b: 49 8b 54 24 10              	movq	0x10(%r12), %rdx
1409ec990: 48 85 d2                    	testq	%rdx, %rdx
1409ec993: 0f 84 b7 fe ff ff           	je	0x1409ec850 <.text+0x9eb850>
1409ec999: 49 8b 4c 24 18              	movq	0x18(%r12), %rcx
1409ec99e: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1409ec9a4: e8 77 72 c2 ff              	callq	0x140613c20 <.text+0x612c20>
1409ec9a9: e9 a2 fe ff ff              	jmp	0x1409ec850 <.text+0x9eb850>
1409ec9ae: 48 8b 45 10                 	movq	0x10(%rbp), %rax
1409ec9b2: 48 89 45 70                 	movq	%rax, 0x70(%rbp)
1409ec9b6: 0f 10 45 e0                 	movups	-0x20(%rbp), %xmm0
1409ec9ba: 0f 10 4d f0                 	movups	-0x10(%rbp), %xmm1
1409ec9be: 0f 10 55 00                 	movups	(%rbp), %xmm2
1409ec9c2: 0f 29 55 60                 	movaps	%xmm2, 0x60(%rbp)
1409ec9c6: 0f 29 4d 50                 	movaps	%xmm1, 0x50(%rbp)
1409ec9ca: 0f 29 45 40                 	movaps	%xmm0, 0x40(%rbp)
1409ec9ce: 48 8d 05 73 c0 0a 02        	leaq	0x20ac073(%rip), %rax   # 0x142a98a48
1409ec9d5: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1409ec9da: 48 8d 0d 5f ba 0a 02        	leaq	0x20aba5f(%rip), %rcx   # 0x142a98440
1409ec9e1: 4c 8d 0d 88 ba 0a 02        	leaq	0x20aba88(%rip), %r9    # 0x142a98470
1409ec9e8: 4c 8d 45 40                 	leaq	0x40(%rbp), %r8
1409ec9ec: ba 2b 00 00 00              	movl	$0x2b, %edx
1409ec9f1: e8 6a cd ee 01              	callq	0x1428d9760 <.text+0x28d8760>
1409ec9f6: cc                          	int3
1409ec9f7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1409eca00: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409eca05: 55                          	pushq	%rbp
1409eca06: 41 57                       	pushq	%r15
1409eca08: 41 56                       	pushq	%r14
1409eca0a: 41 55                       	pushq	%r13
1409eca0c: 41 54                       	pushq	%r12
1409eca0e: 56                          	pushq	%rsi
1409eca0f: 57                          	pushq	%rdi
1409eca10: 53                          	pushq	%rbx
1409eca11: 48 81 ec 88 00 00 00        	subq	$0x88, %rsp
1409eca18: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409eca1f: 44 0f 29 5c 24 20           	movaps	%xmm11, 0x20(%rsp)
1409eca25: 44 0f 29 54 24 30           	movaps	%xmm10, 0x30(%rsp)
1409eca2b: 44 0f 29 4c 24 40           	movaps	%xmm9, 0x40(%rsp)
1409eca31: 44 0f 29 44 24 50           	movaps	%xmm8, 0x50(%rsp)
1409eca37: 0f 29 7c 24 60              	movaps	%xmm7, 0x60(%rsp)
1409eca3c: 0f 29 74 24 70              	movaps	%xmm6, 0x70(%rsp)
1409eca41: 48 8b 85 80 00 00 00        	movq	0x80(%rbp), %rax
1409eca48: 48 85 c0                    	testq	%rax, %rax
1409eca4b: 74 1a                       	je	0x1409eca67 <.text+0x9eba67>
1409eca4d: 48 c1 e0 04                 	shlq	$0x4, %rax
1409eca51: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
1409eca55: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1409eca5b: 48 8b 8d e8 00 00 00        	movq	0xe8(%rbp), %rcx
1409eca62: e8 b9 71 c2 ff              	callq	0x140613c20 <.text+0x612c20>
1409eca67: c6 85 34 01 00 00 01        	movb	$0x1, 0x134(%rbp)
1409eca6e: 0f 28 74 24 70              	movaps	0x70(%rsp), %xmm6
1409eca73: 0f 28 7c 24 60              	movaps	0x60(%rsp), %xmm7
1409eca78: 44 0f 28 44 24 50           	movaps	0x50(%rsp), %xmm8
1409eca7e: 44 0f 28 4c 24 40           	movaps	0x40(%rsp), %xmm9
1409eca84: 44 0f 28 54 24 30           	movaps	0x30(%rsp), %xmm10
1409eca8a: 44 0f 28 5c 24 20           	movaps	0x20(%rsp), %xmm11
1409eca90: 48 81 c4 88 00 00 00        	addq	$0x88, %rsp
1409eca97: 5b                          	popq	%rbx
1409eca98: 5f                          	popq	%rdi
1409eca99: 5e                          	popq	%rsi
1409eca9a: 41 5c                       	popq	%r12
1409eca9c: 41 5d                       	popq	%r13
1409eca9e: 41 5e                       	popq	%r14
1409ecaa0: 41 5f                       	popq	%r15
1409ecaa2: 5d                          	popq	%rbp
1409ecaa3: c3                          	retq
1409ecaa4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
1409ecab0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409ecab5: 55                          	pushq	%rbp
1409ecab6: 41 57                       	pushq	%r15
1409ecab8: 41 56                       	pushq	%r14
1409ecaba: 41 55                       	pushq	%r13
1409ecabc: 41 54                       	pushq	%r12
1409ecabe: 56                          	pushq	%rsi
1409ecabf: 57                          	pushq	%rdi
1409ecac0: 53                          	pushq	%rbx
1409ecac1: 48 81 ec 88 00 00 00        	subq	$0x88, %rsp
1409ecac8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409ecacf: 44 0f 29 5c 24 20           	movaps	%xmm11, 0x20(%rsp)
1409ecad5: 44 0f 29 54 24 30           	movaps	%xmm10, 0x30(%rsp)
1409ecadb: 44 0f 29 4c 24 40           	movaps	%xmm9, 0x40(%rsp)
1409ecae1: 44 0f 29 44 24 50           	movaps	%xmm8, 0x50(%rsp)
1409ecae7: 0f 29 7c 24 60              	movaps	%xmm7, 0x60(%rsp)
1409ecaec: 0f 29 74 24 70              	movaps	%xmm6, 0x70(%rsp)
1409ecaf1: c6 85 37 01 00 00 01        	movb	$0x1, 0x137(%rbp)
1409ecaf8: 48 8d 4d 18                 	leaq	0x18(%rbp), %rcx
1409ecafc: e8 3f cc ea ff              	callq	0x140899740 <.text+0x898740>
1409ecb01: c6 85 37 01 00 00 01        	movb	$0x1, 0x137(%rbp)
1409ecb08: 0f 28 74 24 70              	movaps	0x70(%rsp), %xmm6
1409ecb0d: 0f 28 7c 24 60              	movaps	0x60(%rsp), %xmm7
1409ecb12: 44 0f 28 44 24 50           	movaps	0x50(%rsp), %xmm8
1409ecb18: 44 0f 28 4c 24 40           	movaps	0x40(%rsp), %xmm9
1409ecb1e: 44 0f 28 54 24 30           	movaps	0x30(%rsp), %xmm10
1409ecb24: 44 0f 28 5c 24 20           	movaps	0x20(%rsp), %xmm11
1409ecb2a: 48 81 c4 88 00 00 00        	addq	$0x88, %rsp
1409ecb31: 5b                          	popq	%rbx
1409ecb32: 5f                          	popq	%rdi
1409ecb33: 5e                          	popq	%rsi
1409ecb34: 41 5c                       	popq	%r12
1409ecb36: 41 5d                       	popq	%r13
1409ecb38: 41 5e                       	popq	%r14
1409ecb3a: 41 5f                       	popq	%r15
1409ecb3c: 5d                          	popq	%rbp
1409ecb3d: c3                          	retq
1409ecb3e: 66 90                       	nop
1409ecb40: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409ecb45: 55                          	pushq	%rbp
1409ecb46: 41 57                       	pushq	%r15
1409ecb48: 41 56                       	pushq	%r14
1409ecb4a: 41 55                       	pushq	%r13
1409ecb4c: 41 54                       	pushq	%r12
1409ecb4e: 56                          	pushq	%rsi
1409ecb4f: 57                          	pushq	%rdi
1409ecb50: 53                          	pushq	%rbx
1409ecb51: 48 81 ec 88 00 00 00        	subq	$0x88, %rsp
1409ecb58: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409ecb5f: 44 0f 29 5c 24 20           	movaps	%xmm11, 0x20(%rsp)
1409ecb65: 44 0f 29 54 24 30           	movaps	%xmm10, 0x30(%rsp)
1409ecb6b: 44 0f 29 4c 24 40           	movaps	%xmm9, 0x40(%rsp)
1409ecb71: 44 0f 29 44 24 50           	movaps	%xmm8, 0x50(%rsp)
1409ecb77: 0f 29 7c 24 60              	movaps	%xmm7, 0x60(%rsp)
1409ecb7c: 0f 29 74 24 70              	movaps	%xmm6, 0x70(%rsp)
1409ecb81: c6 85 37 01 00 00 01        	movb	$0x1, 0x137(%rbp)
1409ecb88: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
1409ecb8f: e8 9c 00 36 00              	callq	0x140d4cc30 <.text+0xd4bc30>
1409ecb94: c6 85 37 01 00 00 01        	movb	$0x1, 0x137(%rbp)
1409ecb9b: 0f 28 74 24 70              	movaps	0x70(%rsp), %xmm6
1409ecba0: 0f 28 7c 24 60              	movaps	0x60(%rsp), %xmm7
1409ecba5: 44 0f 28 44 24 50           	movaps	0x50(%rsp), %xmm8
1409ecbab: 44 0f 28 4c 24 40           	movaps	0x40(%rsp), %xmm9
1409ecbb1: 44 0f 28 54 24 30           	movaps	0x30(%rsp), %xmm10
1409ecbb7: 44 0f 28 5c 24 20           	movaps	0x20(%rsp), %xmm11
1409ecbbd: 48 81 c4 88 00 00 00        	addq	$0x88, %rsp
1409ecbc4: 5b                          	popq	%rbx
1409ecbc5: 5f                          	popq	%rdi
1409ecbc6: 5e                          	popq	%rsi
1409ecbc7: 41 5c                       	popq	%r12
1409ecbc9: 41 5d                       	popq	%r13
1409ecbcb: 41 5e                       	popq	%r14
1409ecbcd: 41 5f                       	popq	%r15
1409ecbcf: 5d                          	popq	%rbp
1409ecbd0: c3                          	retq
1409ecbd1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
1409ecbe0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409ecbe5: 55                          	pushq	%rbp
1409ecbe6: 41 57                       	pushq	%r15
1409ecbe8: 41 56                       	pushq	%r14
1409ecbea: 41 55                       	pushq	%r13
1409ecbec: 41 54                       	pushq	%r12
1409ecbee: 56                          	pushq	%rsi
1409ecbef: 57                          	pushq	%rdi
1409ecbf0: 53                          	pushq	%rbx
1409ecbf1: 48 81 ec 88 00 00 00        	subq	$0x88, %rsp
1409ecbf8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409ecbff: 44 0f 29 5c 24 20           	movaps	%xmm11, 0x20(%rsp)
1409ecc05: 44 0f 29 54 24 30           	movaps	%xmm10, 0x30(%rsp)
1409ecc0b: 44 0f 29 4c 24 40           	movaps	%xmm9, 0x40(%rsp)
1409ecc11: 44 0f 29 44 24 50           	movaps	%xmm8, 0x50(%rsp)
1409ecc17: 0f 29 7c 24 60              	movaps	%xmm7, 0x60(%rsp)
1409ecc1c: 0f 29 74 24 70              	movaps	%xmm6, 0x70(%rsp)
1409ecc21: c6 85 37 01 00 00 01        	movb	$0x1, 0x137(%rbp)
1409ecc28: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
1409ecc2f: e8 0c cb ea ff              	callq	0x140899740 <.text+0x898740>
1409ecc34: c6 85 37 01 00 00 01        	movb	$0x1, 0x137(%rbp)
1409ecc3b: 0f 28 74 24 70              	movaps	0x70(%rsp), %xmm6
1409ecc40: 0f 28 7c 24 60              	movaps	0x60(%rsp), %xmm7
1409ecc45: 44 0f 28 44 24 50           	movaps	0x50(%rsp), %xmm8
1409ecc4b: 44 0f 28 4c 24 40           	movaps	0x40(%rsp), %xmm9
1409ecc51: 44 0f 28 54 24 30           	movaps	0x30(%rsp), %xmm10
1409ecc57: 44 0f 28 5c 24 20           	movaps	0x20(%rsp), %xmm11
1409ecc5d: 48 81 c4 88 00 00 00        	addq	$0x88, %rsp
1409ecc64: 5b                          	popq	%rbx
1409ecc65: 5f                          	popq	%rdi
1409ecc66: 5e                          	popq	%rsi
1409ecc67: 41 5c                       	popq	%r12
1409ecc69: 41 5d                       	popq	%r13
1409ecc6b: 41 5e                       	popq	%r14
1409ecc6d: 41 5f                       	popq	%r15
1409ecc6f: 5d                          	popq	%rbp
1409ecc70: c3                          	retq
1409ecc71: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
1409ecc80: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409ecc85: 55                          	pushq	%rbp
1409ecc86: 41 57                       	pushq	%r15
1409ecc88: 41 56                       	pushq	%r14
1409ecc8a: 41 55                       	pushq	%r13
1409ecc8c: 41 54                       	pushq	%r12
1409ecc8e: 56                          	pushq	%rsi
1409ecc8f: 57                          	pushq	%rdi
1409ecc90: 53                          	pushq	%rbx
1409ecc91: 48 81 ec 88 00 00 00        	subq	$0x88, %rsp
1409ecc98: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409ecc9f: 44 0f 29 5c 24 20           	movaps	%xmm11, 0x20(%rsp)
1409ecca5: 44 0f 29 54 24 30           	movaps	%xmm10, 0x30(%rsp)
1409eccab: 44 0f 29 4c 24 40           	movaps	%xmm9, 0x40(%rsp)
1409eccb1: 44 0f 29 44 24 50           	movaps	%xmm8, 0x50(%rsp)
1409eccb7: 0f 29 7c 24 60              	movaps	%xmm7, 0x60(%rsp)
1409eccbc: 0f 29 74 24 70              	movaps	%xmm6, 0x70(%rsp)
1409eccc1: 80 bd 37 01 00 00 00        	cmpb	$0x0, 0x137(%rbp)
1409eccc8: 74 69                       	je	0x1409ecd33 <.text+0x9ebd33>
1409eccca: 48 8b 85 b8 00 00 00        	movq	0xb8(%rbp), %rax
1409eccd1: 48 85 c0                    	testq	%rax, %rax
1409eccd4: 74 1a                       	je	0x1409eccf0 <.text+0x9ebcf0>
1409eccd6: 48 8b 8d c0 00 00 00        	movq	0xc0(%rbp), %rcx
1409eccdd: 48 c1 e0 03                 	shlq	$0x3, %rax
1409ecce1: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
1409ecce5: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1409ecceb: e8 30 6f c2 ff              	callq	0x140613c20 <.text+0x612c20>
1409eccf0: 0f b6 85 35 01 00 00        	movzbl	0x135(%rbp), %eax
1409eccf7: 88 85 36 01 00 00           	movb	%al, 0x136(%rbp)
1409eccfd: 0f 28 74 24 70              	movaps	0x70(%rsp), %xmm6
1409ecd02: 0f 28 7c 24 60              	movaps	0x60(%rsp), %xmm7
1409ecd07: 44 0f 28 44 24 50           	movaps	0x50(%rsp), %xmm8
1409ecd0d: 44 0f 28 4c 24 40           	movaps	0x40(%rsp), %xmm9
1409ecd13: 44 0f 28 54 24 30           	movaps	0x30(%rsp), %xmm10
1409ecd19: 44 0f 28 5c 24 20           	movaps	0x20(%rsp), %xmm11
1409ecd1f: 48 81 c4 88 00 00 00        	addq	$0x88, %rsp
1409ecd26: 5b                          	popq	%rbx
1409ecd27: 5f                          	popq	%rdi
1409ecd28: 5e                          	popq	%rsi
1409ecd29: 41 5c                       	popq	%r12
1409ecd2b: 41 5d                       	popq	%r13
1409ecd2d: 41 5e                       	popq	%r14
1409ecd2f: 41 5f                       	popq	%r15
1409ecd31: 5d                          	popq	%rbp
1409ecd32: c3                          	retq
1409ecd33: 0f b6 85 35 01 00 00        	movzbl	0x135(%rbp), %eax
1409ecd3a: 88 85 36 01 00 00           	movb	%al, 0x136(%rbp)
1409ecd40: 0f 28 74 24 70              	movaps	0x70(%rsp), %xmm6
1409ecd45: 0f 28 7c 24 60              	movaps	0x60(%rsp), %xmm7
1409ecd4a: 44 0f 28 44 24 50           	movaps	0x50(%rsp), %xmm8
1409ecd50: 44 0f 28 4c 24 40           	movaps	0x40(%rsp), %xmm9
1409ecd56: 44 0f 28 54 24 30           	movaps	0x30(%rsp), %xmm10
1409ecd5c: 44 0f 28 5c 24 20           	movaps	0x20(%rsp), %xmm11
1409ecd62: 48 81 c4 88 00 00 00        	addq	$0x88, %rsp
1409ecd69: 5b                          	popq	%rbx
1409ecd6a: 5f                          	popq	%rdi
1409ecd6b: 5e                          	popq	%rsi
1409ecd6c: 41 5c                       	popq	%r12
1409ecd6e: 41 5d                       	popq	%r13
1409ecd70: 41 5e                       	popq	%r14
1409ecd72: 41 5f                       	popq	%r15
1409ecd74: 5d                          	popq	%rbp
1409ecd75: c3                          	retq
1409ecd76: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
1409ecd80: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409ecd85: 55                          	pushq	%rbp
1409ecd86: 41 57                       	pushq	%r15
1409ecd88: 41 56                       	pushq	%r14
1409ecd8a: 41 55                       	pushq	%r13
1409ecd8c: 41 54                       	pushq	%r12
1409ecd8e: 56                          	pushq	%rsi
1409ecd8f: 57                          	pushq	%rdi
1409ecd90: 53                          	pushq	%rbx
1409ecd91: 48 81 ec 88 00 00 00        	subq	$0x88, %rsp
1409ecd98: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409ecd9f: 44 0f 29 5c 24 20           	movaps	%xmm11, 0x20(%rsp)
1409ecda5: 44 0f 29 54 24 30           	movaps	%xmm10, 0x30(%rsp)
1409ecdab: 44 0f 29 4c 24 40           	movaps	%xmm9, 0x40(%rsp)
1409ecdb1: 44 0f 29 44 24 50           	movaps	%xmm8, 0x50(%rsp)
1409ecdb7: 0f 29 7c 24 60              	movaps	%xmm7, 0x60(%rsp)
1409ecdbc: 0f 29 74 24 70              	movaps	%xmm6, 0x70(%rsp)
1409ecdc1: 80 bd 36 01 00 00 00        	cmpb	$0x0, 0x136(%rbp)
1409ecdc8: 75 36                       	jne	0x1409ece00 <.text+0x9ebe00>
1409ecdca: 0f 28 74 24 70              	movaps	0x70(%rsp), %xmm6
1409ecdcf: 0f 28 7c 24 60              	movaps	0x60(%rsp), %xmm7
1409ecdd4: 44 0f 28 44 24 50           	movaps	0x50(%rsp), %xmm8
1409ecdda: 44 0f 28 4c 24 40           	movaps	0x40(%rsp), %xmm9
1409ecde0: 44 0f 28 54 24 30           	movaps	0x30(%rsp), %xmm10
1409ecde6: 44 0f 28 5c 24 20           	movaps	0x20(%rsp), %xmm11
1409ecdec: 48 81 c4 88 00 00 00        	addq	$0x88, %rsp
1409ecdf3: 5b                          	popq	%rbx
1409ecdf4: 5f                          	popq	%rdi
1409ecdf5: 5e                          	popq	%rsi
1409ecdf6: 41 5c                       	popq	%r12
1409ecdf8: 41 5d                       	popq	%r13
1409ecdfa: 41 5e                       	popq	%r14
1409ecdfc: 41 5f                       	popq	%r15
1409ecdfe: 5d                          	popq	%rbp
1409ecdff: c3                          	retq
1409ece00: 48 8d 8d a0 00 00 00        	leaq	0xa0(%rbp), %rcx
1409ece07: e8 d4 86 ea ff              	callq	0x1408954e0 <.text+0x8944e0>
1409ece0c: eb bc                       	jmp	0x1409ecdca <.text+0x9ebdca>
1409ece0e: 66 90                       	nop
1409ece10: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409ece15: 55                          	pushq	%rbp
1409ece16: 41 57                       	pushq	%r15
1409ece18: 41 56                       	pushq	%r14
1409ece1a: 41 55                       	pushq	%r13
1409ece1c: 41 54                       	pushq	%r12
1409ece1e: 56                          	pushq	%rsi
1409ece1f: 57                          	pushq	%rdi
1409ece20: 53                          	pushq	%rbx
1409ece21: 48 81 ec 88 00 00 00        	subq	$0x88, %rsp
1409ece28: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409ece2f: 44 0f 29 5c 24 20           	movaps	%xmm11, 0x20(%rsp)
1409ece35: 44 0f 29 54 24 30           	movaps	%xmm10, 0x30(%rsp)
1409ece3b: 44 0f 29 4c 24 40           	movaps	%xmm9, 0x40(%rsp)
1409ece41: 44 0f 29 44 24 50           	movaps	%xmm8, 0x50(%rsp)
1409ece47: 0f 29 7c 24 60              	movaps	%xmm7, 0x60(%rsp)
1409ece4c: 0f 29 74 24 70              	movaps	%xmm6, 0x70(%rsp)
1409ece51: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
1409ece58: e8 d3 9c e9 ff              	callq	0x140886b30 <.text+0x885b30>
1409ece5d: 0f 28 74 24 70              	movaps	0x70(%rsp), %xmm6
1409ece62: 0f 28 7c 24 60              	movaps	0x60(%rsp), %xmm7
1409ece67: 44 0f 28 44 24 50           	movaps	0x50(%rsp), %xmm8
1409ece6d: 44 0f 28 4c 24 40           	movaps	0x40(%rsp), %xmm9
1409ece73: 44 0f 28 54 24 30           	movaps	0x30(%rsp), %xmm10
1409ece79: 44 0f 28 5c 24 20           	movaps	0x20(%rsp), %xmm11
1409ece7f: 48 81 c4 88 00 00 00        	addq	$0x88, %rsp
1409ece86: 5b                          	popq	%rbx
1409ece87: 5f                          	popq	%rdi
1409ece88: 5e                          	popq	%rsi
1409ece89: 41 5c                       	popq	%r12
1409ece8b: 41 5d                       	popq	%r13
1409ece8d: 41 5e                       	popq	%r14
1409ece8f: 41 5f                       	popq	%r15
1409ece91: 5d                          	popq	%rbp
1409ece92: c3                          	retq

D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1409ecea0: 55                          	pushq	%rbp
1409ecea1: 41 57                       	pushq	%r15
1409ecea3: 41 56                       	pushq	%r14
1409ecea5: 41 55                       	pushq	%r13
1409ecea7: 41 54                       	pushq	%r12
1409ecea9: 56                          	pushq	%rsi
1409eceaa: 57                          	pushq	%rdi
1409eceab: 53                          	pushq	%rbx
1409eceac: 48 81 ec a8 01 00 00        	subq	$0x1a8, %rsp            # imm = 0x1A8
1409eceb3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1409ecebb: 44 0f 29 bd 10 01 00 00     	movaps	%xmm15, 0x110(%rbp)
1409ecec3: 44 0f 29 b5 00 01 00 00     	movaps	%xmm14, 0x100(%rbp)
1409ececb: 44 0f 29 ad f0 00 00 00     	movaps	%xmm13, 0xf0(%rbp)
1409eced3: 44 0f 29 a5 e0 00 00 00     	movaps	%xmm12, 0xe0(%rbp)
1409ecedb: 44 0f 29 9d d0 00 00 00     	movaps	%xmm11, 0xd0(%rbp)
1409ecee3: 44 0f 29 95 c0 00 00 00     	movaps	%xmm10, 0xc0(%rbp)
1409eceeb: 44 0f 29 8d b0 00 00 00     	movaps	%xmm9, 0xb0(%rbp)
1409ecef3: 44 0f 29 85 a0 00 00 00     	movaps	%xmm8, 0xa0(%rbp)
1409ecefb: 0f 29 bd 90 00 00 00        	movaps	%xmm7, 0x90(%rbp)
1409ecf02: 0f 29 b5 80 00 00 00        	movaps	%xmm6, 0x80(%rbp)
1409ecf09: 48 c7 45 78 fe ff ff ff     	movq	$-0x2, 0x78(%rbp)
1409ecf11: 80 3d 78 7e c4 02 00        	cmpb	$0x0, 0x2c47e78(%rip)   # 0x143634d90
1409ecf18: 0f 85 c8 04 00 00           	jne	0x1409ed3e6 <.text+0x9ec3e6>
1409ecf1e: 4c 89 c3                    	movq	%r8, %rbx
1409ecf21: 48 89 d7                    	movq	%rdx, %rdi
1409ecf24: 48 89 ce                    	movq	%rcx, %rsi
1409ecf27: e8 d4 9a 22 00              	callq	0x140c16a00 <.text+0xc15a00>
1409ecf2c: 84 c0                       	testb	%al, %al
1409ecf2e: 88 45 77                    	movb	%al, 0x77(%rbp)
1409ecf31: 74 60                       	je	0x1409ecf93 <.text+0x9ebf93>
1409ecf33: 4c 8d 3d 19 7f 0d 02        	leaq	0x20d7f19(%rip), %r15   # 0x142ac4e53
1409ecf3a: 41 be 4f 00 00 00           	movl	$0x4f, %r14d
1409ecf40: ba 4f 00 00 00              	movl	$0x4f, %edx
1409ecf45: 4c 89 f9                    	movq	%r15, %rcx
1409ecf48: e8 d3 c2 ff ff              	callq	0x1409e9220 <.text+0x9e8220>
1409ecf4d: 48 83 f8 01                 	cmpq	$0x1, %rax
1409ecf51: 0f 85 b2 00 00 00           	jne	0x1409ed009 <.text+0x9ec009>
1409ecf57: 49 89 d1                    	movq	%rdx, %r9
1409ecf5a: 48 85 d2                    	testq	%rdx, %rdx
1409ecf5d: 74 3d                       	je	0x1409ecf9c <.text+0x9ebf9c>
1409ecf5f: 49 83 f9 4f                 	cmpq	$0x4f, %r9
1409ecf63: 73 35                       	jae	0x1409ecf9a <.text+0x9ebf9a>
1409ecf65: 48 8d 05 e7 7e 0d 02        	leaq	0x20d7ee7(%rip), %rax   # 0x142ac4e53
1409ecf6c: 41 80 3c 01 bf              	cmpb	$-0x41, (%r9,%rax)
1409ecf71: 7f 29                       	jg	0x1409ecf9c <.text+0x9ebf9c>
1409ecf73: 48 8d 05 46 80 0d 02        	leaq	0x20d8046(%rip), %rax   # 0x142ac4fc0
1409ecf7a: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1409ecf7f: 48 8d 0d cd 7e 0d 02        	leaq	0x20d7ecd(%rip), %rcx   # 0x142ac4e53
1409ecf86: ba 4f 00 00 00              	movl	$0x4f, %edx
1409ecf8b: 45 31 c0                    	xorl	%r8d, %r8d
1409ecf8e: e8 ad c9 ee 01              	callq	0x1428d9940 <.text+0x28d8940>
1409ecf93: 31 c0                       	xorl	%eax, %eax
1409ecf95: e9 d0 01 00 00              	jmp	0x1409ed16a <.text+0x9ec16a>
1409ecf9a: 75 d7                       	jne	0x1409ecf73 <.text+0x9ebf73>
1409ecf9c: 4c 8d 3d b0 7e 0d 02        	leaq	0x20d7eb0(%rip), %r15   # 0x142ac4e53
1409ecfa3: 4c 89 f9                    	movq	%r15, %rcx
1409ecfa6: 4c 89 ca                    	movq	%r9, %rdx
1409ecfa9: e8 72 c2 ff ff              	callq	0x1409e9220 <.text+0x9e8220>
1409ecfae: 48 83 f8 01                 	cmpq	$0x1, %rax
1409ecfb2: 75 55                       	jne	0x1409ed009 <.text+0x9ec009>
1409ecfb4: 49 89 d0                    	movq	%rdx, %r8
1409ecfb7: 49 83 c0 02                 	addq	$0x2, %r8
1409ecfbb: 74 39                       	je	0x1409ecff6 <.text+0x9ebff6>
1409ecfbd: 49 83 f8 4f                 	cmpq	$0x4f, %r8
1409ecfc1: 73 31                       	jae	0x1409ecff4 <.text+0x9ebff4>
1409ecfc3: 48 8d 05 89 7e 0d 02        	leaq	0x20d7e89(%rip), %rax   # 0x142ac4e53
1409ecfca: 41 80 3c 00 bf              	cmpb	$-0x41, (%r8,%rax)
1409ecfcf: 7f 25                       	jg	0x1409ecff6 <.text+0x9ebff6>
1409ecfd1: 48 8d 05 00 80 0d 02        	leaq	0x20d8000(%rip), %rax   # 0x142ac4fd8
1409ecfd8: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1409ecfdd: 48 8d 0d 6f 7e 0d 02        	leaq	0x20d7e6f(%rip), %rcx   # 0x142ac4e53
1409ecfe4: ba 4f 00 00 00              	movl	$0x4f, %edx
1409ecfe9: 41 b9 4f 00 00 00           	movl	$0x4f, %r9d
1409ecfef: e8 4c c9 ee 01              	callq	0x1428d9940 <.text+0x28d8940>
1409ecff4: 75 db                       	jne	0x1409ecfd1 <.text+0x9ebfd1>
1409ecff6: 41 be 4d 00 00 00           	movl	$0x4d, %r14d
1409ecffc: 49 29 d6                    	subq	%rdx, %r14
1409ecfff: 4c 8d 3d 4d 7e 0d 02        	leaq	0x20d7e4d(%rip), %r15   # 0x142ac4e53
1409ed006: 4d 01 c7                    	addq	%r8, %r15
1409ed009: 48 8d 05 0e 82 0d 02        	leaq	0x20d820e(%rip), %rax   # 0x142ac521e
1409ed010: 4c 8d 25 ca 81 0d 02        	leaq	0x20d81ca(%rip), %r12   # 0x142ac51e1
1409ed017: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1409ed020: 0f be 48 ff                 	movsbl	-0x1(%rax), %ecx
1409ed024: 85 c9                       	testl	%ecx, %ecx
1409ed026: 78 18                       	js	0x1409ed040 <.text+0x9ec040>
1409ed028: 48 ff c8                    	decq	%rax
1409ed02b: 83 f9 5c                    	cmpl	$0x5c, %ecx
1409ed02e: 75 69                       	jne	0x1409ed099 <.text+0x9ec099>
1409ed030: eb 7d                       	jmp	0x1409ed0af <.text+0x9ec0af>
1409ed032: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
1409ed040: 0f b6 50 fe                 	movzbl	-0x2(%rax), %edx
1409ed044: 80 fa c0                    	cmpb	$-0x40, %dl
1409ed047: 7d 25                       	jge	0x1409ed06e <.text+0x9ec06e>
1409ed049: 44 0f b6 40 fd              	movzbl	-0x3(%rax), %r8d
1409ed04e: 41 80 f8 c0                 	cmpb	$-0x40, %r8b
1409ed052: 7d 23                       	jge	0x1409ed077 <.text+0x9ec077>
1409ed054: 44 0f b6 48 fc              	movzbl	-0x4(%rax), %r9d
1409ed059: 48 83 c0 fc                 	addq	$-0x4, %rax
1409ed05d: 41 83 e1 07                 	andl	$0x7, %r9d
1409ed061: 41 c1 e1 06                 	shll	$0x6, %r9d
1409ed065: 41 83 e0 3f                 	andl	$0x3f, %r8d
1409ed069: 45 09 c8                    	orl	%r9d, %r8d
1409ed06c: eb 11                       	jmp	0x1409ed07f <.text+0x9ec07f>
1409ed06e: 48 83 c0 fe                 	addq	$-0x2, %rax
1409ed072: 83 e2 1f                    	andl	$0x1f, %edx
1409ed075: eb 12                       	jmp	0x1409ed089 <.text+0x9ec089>
1409ed077: 48 83 c0 fd                 	addq	$-0x3, %rax
1409ed07b: 41 83 e0 0f                 	andl	$0xf, %r8d
1409ed07f: 41 c1 e0 06                 	shll	$0x6, %r8d
1409ed083: 83 e2 3f                    	andl	$0x3f, %edx
1409ed086: 44 09 c2                    	orl	%r8d, %edx
1409ed089: c1 e2 06                    	shll	$0x6, %edx
1409ed08c: 80 e1 3f                    	andb	$0x3f, %cl
1409ed08f: 0f b6 c9                    	movzbl	%cl, %ecx
1409ed092: 09 d1                       	orl	%edx, %ecx
1409ed094: 83 f9 5c                    	cmpl	$0x5c, %ecx
1409ed097: 74 16                       	je	0x1409ed0af <.text+0x9ec0af>
1409ed099: 83 f9 2f                    	cmpl	$0x2f, %ecx
1409ed09c: 74 11                       	je	0x1409ed0af <.text+0x9ec0af>
1409ed09e: 4c 39 e0                    	cmpq	%r12, %rax
1409ed0a1: 0f 85 79 ff ff ff           	jne	0x1409ed020 <.text+0x9ec020>
1409ed0a7: 41 bd 3d 00 00 00           	movl	$0x3d, %r13d
1409ed0ad: eb 50                       	jmp	0x1409ed0ff <.text+0x9ec0ff>
1409ed0af: 4c 8d 25 2b 81 0d 02        	leaq	0x20d812b(%rip), %r12   # 0x142ac51e1
1409ed0b6: 4c 29 e0                    	subq	%r12, %rax
1409ed0b9: 49 89 c0                    	movq	%rax, %r8
1409ed0bc: 49 ff c0                    	incq	%r8
1409ed0bf: 74 32                       	je	0x1409ed0f3 <.text+0x9ec0f3>
1409ed0c1: 49 83 f8 3d                 	cmpq	$0x3d, %r8
1409ed0c5: 73 2a                       	jae	0x1409ed0f1 <.text+0x9ec0f1>
1409ed0c7: 43 80 3c 20 bf              	cmpb	$-0x41, (%r8,%r12)
1409ed0cc: 7f 25                       	jg	0x1409ed0f3 <.text+0x9ec0f3>
1409ed0ce: 48 8d 05 cb 7e 0d 02        	leaq	0x20d7ecb(%rip), %rax   # 0x142ac4fa0
1409ed0d5: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1409ed0da: 48 8d 0d 00 81 0d 02        	leaq	0x20d8100(%rip), %rcx   # 0x142ac51e1
1409ed0e1: ba 3d 00 00 00              	movl	$0x3d, %edx
1409ed0e6: 41 b9 3d 00 00 00           	movl	$0x3d, %r9d
1409ed0ec: e8 4f c8 ee 01              	callq	0x1428d9940 <.text+0x28d8940>
1409ed0f1: 75 db                       	jne	0x1409ed0ce <.text+0x9ec0ce>
1409ed0f3: 41 bd 3c 00 00 00           	movl	$0x3c, %r13d
1409ed0f9: 49 29 c5                    	subq	%rax, %r13
1409ed0fc: 4d 01 c4                    	addq	%r8, %r12
1409ed0ff: e8 2c b0 22 00              	callq	0x140c18130 <.text+0xc17130>
1409ed104: 48 89 c1                    	movq	%rax, %rcx
1409ed107: 48 8b 00                    	movq	(%rax), %rax
1409ed10a: 48 83 f8 01                 	cmpq	$0x1, %rax
1409ed10e: 0f 85 34 03 00 00           	jne	0x1409ed448 <.text+0x9ec448>
1409ed114: 48 83 c1 08                 	addq	$0x8, %rcx
1409ed118: 48 83 39 00                 	cmpq	$0x0, (%rcx)
1409ed11c: 0f 85 64 03 00 00           	jne	0x1409ed486 <.text+0x9ec486>
1409ed122: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
1409ed129: 48 89 4d 68                 	movq	%rcx, 0x68(%rbp)
1409ed12d: 48 83 c1 08                 	addq	$0x8, %rcx
1409ed131: 4c 89 6c 24 20              	movq	%r13, 0x20(%rsp)
1409ed136: 48 c7 44 24 30 00 00 00 00  	movq	$0x0, 0x30(%rsp)
1409ed13f: 48 c7 44 24 28 01 00 00 00  	movq	$0x1, 0x28(%rsp)
1409ed148: 4c 89 fa                    	movq	%r15, %rdx
1409ed14b: 4d 89 f0                    	movq	%r14, %r8
1409ed14e: 4d 89 e1                    	movq	%r12, %r9
1409ed151: e8 ea 9b 22 00              	callq	0x140c16d40 <.text+0xc15d40>
1409ed156: 48 8b 4d 68                 	movq	0x68(%rbp), %rcx
1409ed15a: 48 ff 01                    	incq	(%rcx)
1409ed15d: 48 89 45 48                 	movq	%rax, 0x48(%rbp)
1409ed161: 48 89 45 30                 	movq	%rax, 0x30(%rbp)
1409ed165: b8 01 00 00 00              	movl	$0x1, %eax
1409ed16a: 48 89 45 28                 	movq	%rax, 0x28(%rbp)
1409ed16e: 48 8b 13                    	movq	(%rbx), %rdx
1409ed171: 48 8d 4d 5c                 	leaq	0x5c(%rbp), %rcx
1409ed175: e8 c6 9b 2a 00              	callq	0x140c96d40 <.text+0xc95d40>
1409ed17a: 48 8b 07                    	movq	(%rdi), %rax
1409ed17d: 4c 8b 60 08                 	movq	0x8(%rax), %r12
1409ed181: 4c 8b 68 10                 	movq	0x10(%rax), %r13
1409ed185: 8b 46 1c                    	movl	0x1c(%rsi), %eax
1409ed188: 48 8b 4e 10                 	movq	0x10(%rsi), %rcx
1409ed18c: 89 01                       	movl	%eax, (%rcx)
1409ed18e: 48 8b 06                    	movq	(%rsi), %rax
1409ed191: 4c 8b 70 08                 	movq	0x8(%rax), %r14
1409ed195: 48 8b 40 10                 	movq	0x10(%rax), %rax
1409ed199: 4c 39 e8                    	cmpq	%r13, %rax
1409ed19c: 4c 0f 42 e8                 	cmovbq	%rax, %r13
1409ed1a0: f3 0f 10 45 5c              	movss	0x5c(%rbp), %xmm0
1409ed1a5: f3 0f 11 45 68              	movss	%xmm0, 0x68(%rbp)
1409ed1aa: f3 0f 10 7d 60              	movss	0x60(%rbp), %xmm7
1409ed1af: 31 c9                       	xorl	%ecx, %ecx
1409ed1b1: f3 44 0f 10 45 64           	movss	0x64(%rbp), %xmm8
1409ed1b7: 48 bb 33 33 33 33 33 33 33 33       	movabsq	$0x3333333333333333, %rbx # imm = 0x3333333333333333
1409ed1c1: f3 44 0f 10 0d 3a 87 f3 01  	movss	0x1f3873a(%rip), %xmm9  # 0x142925904
1409ed1ca: 48 8d 75 50                 	leaq	0x50(%rbp), %rsi
1409ed1ce: f3 44 0f 10 1d 29 7b 0d 02  	movss	0x20d7b29(%rip), %xmm11 # 0x142ac4d00
1409ed1d7: f3 44 0f 10 25 40 7b 0d 02  	movss	0x20d7b40(%rip), %xmm12 # 0x142ac4d20
1409ed1e0: eb 19                       	jmp	0x1409ed1fb <.text+0x9ec1fb>
1409ed1e2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
1409ed1f0: 4c 89 f9                    	movq	%r15, %rcx
1409ed1f3: e8 38 c2 06 00              	callq	0x140a59430 <.text+0xa58430>
1409ed1f8: 48 89 f9                    	movq	%rdi, %rcx
1409ed1fb: 4c 39 e9                    	cmpq	%r13, %rcx
1409ed1fe: 0f 83 9a 01 00 00           	jae	0x1409ed39e <.text+0x9ec39e>
1409ed204: 48 8d 79 01                 	leaq	0x1(%rcx), %rdi
1409ed208: 48 8d 04 49                 	leaq	(%rcx,%rcx,2), %rax
1409ed20c: 4c 6b f9 38                 	imulq	$0x38, %rcx, %r15
1409ed210: 4d 01 f7                    	addq	%r14, %r15
1409ed213: 49 8b 4c c4 08              	movq	0x8(%r12,%rax,8), %rcx
1409ed218: 48 85 c9                    	testq	%rcx, %rcx
1409ed21b: 74 d3                       	je	0x1409ed1f0 <.text+0x9ec1f0>
1409ed21d: 49 8d 04 c4                 	leaq	(%r12,%rax,8), %rax
1409ed221: 48 89 ca                    	movq	%rcx, %rdx
1409ed224: 48 d1 ea                    	shrq	%rdx
1409ed227: 49 b8 55 55 55 55 55 55 55 55       	movabsq	$0x5555555555555555, %r8 # imm = 0x5555555555555555
1409ed231: 4c 21 c2                    	andq	%r8, %rdx
1409ed234: 48 29 d1                    	subq	%rdx, %rcx
1409ed237: 48 89 ca                    	movq	%rcx, %rdx
1409ed23a: 48 21 da                    	andq	%rbx, %rdx
1409ed23d: 48 c1 e9 02                 	shrq	$0x2, %rcx
1409ed241: 48 21 d9                    	andq	%rbx, %rcx
1409ed244: 48 01 d1                    	addq	%rdx, %rcx
1409ed247: 48 89 ca                    	movq	%rcx, %rdx
1409ed24a: 48 c1 ea 04                 	shrq	$0x4, %rdx
1409ed24e: 48 01 ca                    	addq	%rcx, %rdx
1409ed251: 48 b9 0f 0f 0f 0f 0f 0f 0f 0f       	movabsq	$0xf0f0f0f0f0f0f0f, %rcx # imm = 0xF0F0F0F0F0F0F0F
1409ed25b: 48 21 ca                    	andq	%rcx, %rdx
1409ed25e: 48 b9 01 01 01 01 01 01 01 01       	movabsq	$0x101010101010101, %rcx # imm = 0x101010101010101
1409ed268: 48 0f af d1                 	imulq	%rcx, %rdx
1409ed26c: 48 c1 ea 38                 	shrq	$0x38, %rdx
1409ed270: 45 0f 57 ed                 	xorps	%xmm13, %xmm13
1409ed274: f3 44 0f 2a ea              	cvtsi2ss	%edx, %xmm13
1409ed279: 41 0f 28 c1                 	movaps	%xmm9, %xmm0
1409ed27d: f3 41 0f 5e c5              	divss	%xmm13, %xmm0
1409ed282: f3 0f 7e 48 10              	movq	0x10(%rax), %xmm1
1409ed287: 66 0f 61 c9                 	punpcklwd	%xmm1, %xmm1    # xmm1 = xmm1[0,0,1,1,2,2,3,3]
1409ed28b: 66 0f 72 e1 10              	psrad	$0x10, %xmm1
1409ed290: 0f 5b c9                    	cvtdq2ps	%xmm1, %xmm1
1409ed293: f3 0f 12 c0                 	movsldup	%xmm0, %xmm0            # xmm0 = xmm0[0,0,2,2]
1409ed297: 0f 59 c1                    	mulps	%xmm1, %xmm0
1409ed29a: 0f 59 05 6f 7a 0d 02        	mulps	0x20d7a6f(%rip), %xmm0  # 0x142ac4d10
1409ed2a1: f2 0f 10 08                 	movsd	(%rax), %xmm1
1409ed2a5: 0f 58 c8                    	addps	%xmm0, %xmm1
1409ed2a8: 0f 13 4d f0                 	movlps	%xmm1, -0x10(%rbp)
1409ed2ac: 48 89 f1                    	movq	%rsi, %rcx
1409ed2af: 48 8d 55 f0                 	leaq	-0x10(%rbp), %rdx
1409ed2b3: e8 08 fb 2a 00              	callq	0x140c9cdc0 <.text+0xc9bdc0>
1409ed2b8: f3 45 0f 59 eb              	mulss	%xmm11, %xmm13
1409ed2bd: 45 0f 28 f5                 	movaps	%xmm13, %xmm14
1409ed2c1: f3 45 0f 59 f4              	mulss	%xmm12, %xmm14
1409ed2c6: f3 0f 10 75 50              	movss	0x50(%rbp), %xmm6
1409ed2cb: f3 44 0f 10 55 54           	movss	0x54(%rbp), %xmm10
1409ed2d1: f3 44 0f 10 7d 58           	movss	0x58(%rbp), %xmm15
1409ed2d7: 4c 89 f9                    	movq	%r15, %rcx
1409ed2da: e8 d1 c3 06 00              	callq	0x140a596b0 <.text+0xa586b0>
1409ed2df: 49 8b 57 10                 	movq	0x10(%r15), %rdx
1409ed2e3: 49 8b 4f 28                 	movq	0x28(%r15), %rcx
1409ed2e7: 48 39 d1                    	cmpq	%rdx, %rcx
1409ed2ea: 0f 83 b0 01 00 00           	jae	0x1409ed4a0 <.text+0x9ec4a0>
1409ed2f0: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
1409ed2f4: 48 c1 e1 04                 	shlq	$0x4, %rcx
1409ed2f8: 49 03 4f 08                 	addq	0x8(%r15), %rcx
1409ed2fc: 48 89 f2                    	movq	%rsi, %rdx
1409ed2ff: e8 9c aa 06 00              	callq	0x140a57da0 <.text+0xa56da0>
1409ed304: 49 8b 57 10                 	movq	0x10(%r15), %rdx
1409ed308: 49 8b 4f 28                 	movq	0x28(%r15), %rcx
1409ed30c: 48 39 d1                    	cmpq	%rdx, %rcx
1409ed30f: 0f 83 7d 01 00 00           	jae	0x1409ed492 <.text+0x9ec492>
1409ed315: 49 8b 57 08                 	movq	0x8(%r15), %rdx
1409ed319: 48 8d 04 49                 	leaq	(%rcx,%rcx,2), %rax
1409ed31d: 48 c1 e0 04                 	shlq	$0x4, %rax
1409ed321: 83 3c 02 01                 	cmpl	$0x1, (%rdx,%rax)
1409ed325: 48 89 f9                    	movq	%rdi, %rcx
1409ed328: 0f 85 cd fe ff ff           	jne	0x1409ed1fb <.text+0x9ec1fb>
1409ed32e: f3 0f 5c 75 68              	subss	0x68(%rbp), %xmm6
1409ed333: f3 44 0f 5c d7              	subss	%xmm7, %xmm10
1409ed338: f3 45 0f 5c f8              	subss	%xmm8, %xmm15
1409ed33d: f3 0f 59 f6                 	mulss	%xmm6, %xmm6
1409ed341: f3 45 0f 59 d2              	mulss	%xmm10, %xmm10
1409ed346: f3 44 0f 58 d6              	addss	%xmm6, %xmm10
1409ed34b: f3 45 0f 59 ff              	mulss	%xmm15, %xmm15
1409ed350: f3 45 0f 58 fa              	addss	%xmm10, %xmm15
1409ed355: f3 45 0f 5e fe              	divss	%xmm14, %xmm15
1409ed35a: f3 45 0f 58 f9              	addss	%xmm9, %xmm15
1409ed35f: 41 0f 28 c1                 	movaps	%xmm9, %xmm0
1409ed363: f3 41 0f 5e c7              	divss	%xmm15, %xmm0
1409ed368: f3 44 0f 59 e8              	mulss	%xmm0, %xmm13
1409ed36d: f3 41 0f 51 d5              	sqrtss	%xmm13, %xmm2
1409ed372: 48 01 c2                    	addq	%rax, %rdx
1409ed375: 48 83 c2 08                 	addq	$0x8, %rdx
1409ed379: 48 8d 4d b8                 	leaq	-0x48(%rbp), %rcx
1409ed37d: e8 be 97 a4 00              	callq	0x141436b40 <.text+0x1435b40>
1409ed382: 48 b8 04 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffc, %rax # imm = 0x8000000000000004
1409ed38c: 48 39 45 b8                 	cmpq	%rax, -0x48(%rbp)
1409ed390: 0f 85 18 01 00 00           	jne	0x1409ed4ae <.text+0x9ec4ae>
1409ed396: 48 89 f9                    	movq	%rdi, %rcx
1409ed399: e9 5d fe ff ff              	jmp	0x1409ed1fb <.text+0x9ec1fb>
1409ed39e: 80 7d 77 00                 	cmpb	$0x0, 0x77(%rbp)
1409ed3a2: 74 42                       	je	0x1409ed3e6 <.text+0x9ec3e6>
1409ed3a4: e8 87 ad 22 00              	callq	0x140c18130 <.text+0xc17130>
1409ed3a9: 48 89 c1                    	movq	%rax, %rcx
1409ed3ac: 48 8b 00                    	movq	(%rax), %rax
1409ed3af: 48 83 f8 01                 	cmpq	$0x1, %rax
1409ed3b3: 0f 85 a9 00 00 00           	jne	0x1409ed462 <.text+0x9ec462>
1409ed3b9: 48 83 c1 08                 	addq	$0x8, %rcx
1409ed3bd: 48 83 39 00                 	cmpq	$0x0, (%rcx)
1409ed3c1: 0f 85 bf 00 00 00           	jne	0x1409ed486 <.text+0x9ec486>
1409ed3c7: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
1409ed3ce: 48 89 4d 68                 	movq	%rcx, 0x68(%rbp)
1409ed3d2: 48 83 c1 08                 	addq	$0x8, %rcx
1409ed3d6: 48 8b 55 48                 	movq	0x48(%rbp), %rdx
1409ed3da: e8 11 9d 22 00              	callq	0x140c170f0 <.text+0xc160f0>
1409ed3df: 48 8b 45 68                 	movq	0x68(%rbp), %rax
1409ed3e3: 48 ff 00                    	incq	(%rax)
1409ed3e6: 0f 28 b5 80 00 00 00        	movaps	0x80(%rbp), %xmm6
1409ed3ed: 0f 28 bd 90 00 00 00        	movaps	0x90(%rbp), %xmm7
1409ed3f4: 44 0f 28 85 a0 00 00 00     	movaps	0xa0(%rbp), %xmm8
1409ed3fc: 44 0f 28 8d b0 00 00 00     	movaps	0xb0(%rbp), %xmm9
1409ed404: 44 0f 28 95 c0 00 00 00     	movaps	0xc0(%rbp), %xmm10
1409ed40c: 44 0f 28 9d d0 00 00 00     	movaps	0xd0(%rbp), %xmm11
1409ed414: 44 0f 28 a5 e0 00 00 00     	movaps	0xe0(%rbp), %xmm12
1409ed41c: 44 0f 28 ad f0 00 00 00     	movaps	0xf0(%rbp), %xmm13
1409ed424: 44 0f 28 b5 00 01 00 00     	movaps	0x100(%rbp), %xmm14
1409ed42c: 44 0f 28 bd 10 01 00 00     	movaps	0x110(%rbp), %xmm15
1409ed434: 48 81 c4 a8 01 00 00        	addq	$0x1a8, %rsp            # imm = 0x1A8
1409ed43b: 5b                          	popq	%rbx
1409ed43c: 5f                          	popq	%rdi
1409ed43d: 5e                          	popq	%rsi
1409ed43e: 41 5c                       	popq	%r12
1409ed440: 41 5d                       	popq	%r13
1409ed442: 41 5e                       	popq	%r14
1409ed444: 41 5f                       	popq	%r15
1409ed446: 5d                          	popq	%rbp
1409ed447: c3                          	retq
1409ed448: 48 85 c0                    	testq	%rax, %rax
1409ed44b: 75 2d                       	jne	0x1409ed47a <.text+0x9ec47a>
1409ed44d: 31 d2                       	xorl	%edx, %edx
1409ed44f: e8 7c b3 f0 01              	callq	0x1428f87d0 <.text+0x28f77d0>
1409ed454: 48 89 c1                    	movq	%rax, %rcx
1409ed457: 48 85 c0                    	testq	%rax, %rax
1409ed45a: 0f 85 b8 fc ff ff           	jne	0x1409ed118 <.text+0x9ec118>
1409ed460: eb 18                       	jmp	0x1409ed47a <.text+0x9ec47a>
1409ed462: 48 85 c0                    	testq	%rax, %rax
1409ed465: 75 13                       	jne	0x1409ed47a <.text+0x9ec47a>
1409ed467: 31 d2                       	xorl	%edx, %edx
1409ed469: e8 62 b3 f0 01              	callq	0x1428f87d0 <.text+0x28f77d0>
1409ed46e: 48 89 c1                    	movq	%rax, %rcx
1409ed471: 48 85 c0                    	testq	%rax, %rax
1409ed474: 0f 85 43 ff ff ff           	jne	0x1409ed3bd <.text+0x9ec3bd>
1409ed47a: 48 8d 0d 1f d6 24 02        	leaq	0x224d61f(%rip), %rcx   # 0x142c3aaa0
1409ed481: e8 4a ac ee 01              	callq	0x1428d80d0 <.text+0x28d70d0>
1409ed486: 48 8d 0d e3 d6 24 02        	leaq	0x224d6e3(%rip), %rcx   # 0x142c3ab70
1409ed48d: e8 be bd ee 01              	callq	0x1428d9250 <.text+0x28d8250>
1409ed492: 4c 8d 05 8f 5a 0e 02        	leaq	0x20e5a8f(%rip), %r8    # 0x142ad2f28
1409ed499: e8 7a c0 ee 01              	callq	0x1428d9518 <.text+0x28d8518>
1409ed49e: eb 56                       	jmp	0x1409ed4f6 <.text+0x9ec4f6>
1409ed4a0: 4c 8d 05 81 5a 0e 02        	leaq	0x20e5a81(%rip), %r8    # 0x142ad2f28
1409ed4a7: e8 6c c0 ee 01              	callq	0x1428d9518 <.text+0x28d8518>
1409ed4ac: eb 48                       	jmp	0x1409ed4f6 <.text+0x9ec4f6>
1409ed4ae: 48 8b 45 e8                 	movq	-0x18(%rbp), %rax
1409ed4b2: 48 89 45 20                 	movq	%rax, 0x20(%rbp)
1409ed4b6: 0f 10 45 b8                 	movups	-0x48(%rbp), %xmm0
1409ed4ba: 0f 10 4d c8                 	movups	-0x38(%rbp), %xmm1
1409ed4be: 0f 10 55 d8                 	movups	-0x28(%rbp), %xmm2
1409ed4c2: 0f 29 55 10                 	movaps	%xmm2, 0x10(%rbp)
1409ed4c6: 0f 29 4d 00                 	movaps	%xmm1, (%rbp)
1409ed4ca: 0f 29 45 f0                 	movaps	%xmm0, -0x10(%rbp)
1409ed4ce: 48 8d 05 ab 57 0e 02        	leaq	0x20e57ab(%rip), %rax   # 0x142ad2c80
1409ed4d5: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1409ed4da: 48 8d 0d 1f 31 0e 02        	leaq	0x20e311f(%rip), %rcx   # 0x142ad0600
1409ed4e1: 4c 8d 0d d8 30 0e 02        	leaq	0x20e30d8(%rip), %r9    # 0x142ad05c0
1409ed4e8: 4c 8d 45 f0                 	leaq	-0x10(%rbp), %r8
1409ed4ec: ba 2b 00 00 00              	movl	$0x2b, %edx
1409ed4f1: e8 6a c2 ee 01              	callq	0x1428d9760 <.text+0x28d8760>
1409ed4f6: 0f 0b                       	ud2
1409ed4f8: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
1409ed500: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409ed505: 55                          	pushq	%rbp
1409ed506: 41 57                       	pushq	%r15
1409ed508: 41 56                       	pushq	%r14
1409ed50a: 41 55                       	pushq	%r13
1409ed50c: 41 54                       	pushq	%r12
1409ed50e: 56                          	pushq	%rsi
1409ed50f: 57                          	pushq	%rdi
1409ed510: 53                          	pushq	%rbx
1409ed511: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
1409ed518: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409ed51f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
1409ed525: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
1409ed52b: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
1409ed531: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
1409ed537: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
1409ed53d: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
1409ed546: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
1409ed54f: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
1409ed558: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
1409ed560: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
1409ed568: 48 8b 45 68                 	movq	0x68(%rbp), %rax
1409ed56c: 48 ff 00                    	incq	(%rax)
1409ed56f: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
1409ed577: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
1409ed57f: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
1409ed588: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
1409ed591: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
1409ed59a: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
1409ed5a0: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
1409ed5a6: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
1409ed5ac: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
1409ed5b2: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
1409ed5b8: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
1409ed5bf: 5b                          	popq	%rbx
1409ed5c0: 5f                          	popq	%rdi
1409ed5c1: 5e                          	popq	%rsi
1409ed5c2: 41 5c                       	popq	%r12
1409ed5c4: 41 5d                       	popq	%r13
1409ed5c6: 41 5e                       	popq	%r14
1409ed5c8: 41 5f                       	popq	%r15
1409ed5ca: 5d                          	popq	%rbp
1409ed5cb: c3                          	retq
1409ed5cc: 0f 1f 40 00                 	nopl	(%rax)
1409ed5d0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409ed5d5: 55                          	pushq	%rbp
1409ed5d6: 41 57                       	pushq	%r15
1409ed5d8: 41 56                       	pushq	%r14
1409ed5da: 41 55                       	pushq	%r13
1409ed5dc: 41 54                       	pushq	%r12
1409ed5de: 56                          	pushq	%rsi
1409ed5df: 57                          	pushq	%rdi
1409ed5e0: 53                          	pushq	%rbx
1409ed5e1: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
1409ed5e8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409ed5ef: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
1409ed5f5: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
1409ed5fb: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
1409ed601: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
1409ed607: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
1409ed60d: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
1409ed616: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
1409ed61f: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
1409ed628: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
1409ed630: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
1409ed638: 80 7d 77 00                 	cmpb	$0x0, 0x77(%rbp)
1409ed63c: 74 20                       	je	0x1409ed65e <.text+0x9ec65e>
1409ed63e: 48 8d 45 30                 	leaq	0x30(%rbp), %rax
1409ed642: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
1409ed646: 48 8d 45 40                 	leaq	0x40(%rbp), %rax
1409ed64a: 48 89 45 38                 	movq	%rax, 0x38(%rbp)
1409ed64e: 48 8d 0d eb 78 0d 02        	leaq	0x20d78eb(%rip), %rcx   # 0x142ac4f40
1409ed655: 48 8d 55 38                 	leaq	0x38(%rbp), %rdx
1409ed659: e8 72 1c 94 00              	callq	0x14132f2d0 <.text+0x132e2d0>
1409ed65e: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
1409ed666: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
1409ed66e: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
1409ed677: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
1409ed680: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
1409ed689: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
1409ed68f: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
1409ed695: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
1409ed69b: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
1409ed6a1: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
1409ed6a7: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
1409ed6ae: 5b                          	popq	%rbx
1409ed6af: 5f                          	popq	%rdi
1409ed6b0: 5e                          	popq	%rsi
1409ed6b1: 41 5c                       	popq	%r12
1409ed6b3: 41 5d                       	popq	%r13
1409ed6b5: 41 5e                       	popq	%r14
1409ed6b7: 41 5f                       	popq	%r15
1409ed6b9: 5d                          	popq	%rbp
1409ed6ba: c3                          	retq
1409ed6bb: 0f 1f 44 00 00              	nopl	(%rax,%rax)
1409ed6c0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409ed6c5: 55                          	pushq	%rbp
1409ed6c6: 41 57                       	pushq	%r15
1409ed6c8: 41 56                       	pushq	%r14
1409ed6ca: 41 55                       	pushq	%r13
1409ed6cc: 41 54                       	pushq	%r12
1409ed6ce: 56                          	pushq	%rsi
1409ed6cf: 57                          	pushq	%rdi
1409ed6d0: 53                          	pushq	%rbx
1409ed6d1: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
1409ed6d8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409ed6df: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
1409ed6e5: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
1409ed6eb: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
1409ed6f1: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
1409ed6f7: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
1409ed6fd: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
1409ed706: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
1409ed70f: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
1409ed718: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
1409ed720: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
1409ed728: 48 8b 45 68                 	movq	0x68(%rbp), %rax
1409ed72c: 48 ff 00                    	incq	(%rax)
1409ed72f: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
1409ed737: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
1409ed73f: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
1409ed748: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
1409ed751: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
1409ed75a: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
1409ed760: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
1409ed766: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
1409ed76c: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
1409ed772: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
1409ed778: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
1409ed77f: 5b                          	popq	%rbx
1409ed780: 5f                          	popq	%rdi
1409ed781: 5e                          	popq	%rsi
1409ed782: 41 5c                       	popq	%r12
1409ed784: 41 5d                       	popq	%r13
1409ed786: 41 5e                       	popq	%r14
1409ed788: 41 5f                       	popq	%r15
1409ed78a: 5d                          	popq	%rbp
1409ed78b: c3                          	retq
1409ed78c: 0f 1f 40 00                 	nopl	(%rax)
1409ed790: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409ed795: 55                          	pushq	%rbp
1409ed796: 41 57                       	pushq	%r15
1409ed798: 41 56                       	pushq	%r14
1409ed79a: 41 55                       	pushq	%r13
1409ed79c: 41 54                       	pushq	%r12
1409ed79e: 56                          	pushq	%rsi
1409ed79f: 57                          	pushq	%rdi
1409ed7a0: 53                          	pushq	%rbx
1409ed7a1: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
1409ed7a8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409ed7af: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
1409ed7b5: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
1409ed7bb: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
1409ed7c1: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
1409ed7c7: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
1409ed7cd: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
1409ed7d6: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
1409ed7df: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
1409ed7e8: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
1409ed7f0: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
1409ed7f8: 48 8d 4d f0                 	leaq	-0x10(%rbp), %rcx
1409ed7fc: e8 af 4e 06 00              	callq	0x140a526b0 <.text+0xa516b0>
1409ed801: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
1409ed809: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
1409ed811: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
1409ed81a: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
1409ed823: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
1409ed82c: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
1409ed832: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
1409ed838: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
1409ed83e: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
1409ed844: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
1409ed84a: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
1409ed851: 5b                          	popq	%rbx
1409ed852: 5f                          	popq	%rdi
1409ed853: 5e                          	popq	%rsi
1409ed854: 41 5c                       	popq	%r12
1409ed856: 41 5d                       	popq	%r13
1409ed858: 41 5e                       	popq	%r14
1409ed85a: 41 5f                       	popq	%r15
1409ed85c: 5d                          	popq	%rbp
1409ed85d: c3                          	retq

D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141baefe0: 55                          	pushq	%rbp
141baefe1: 41 57                       	pushq	%r15
141baefe3: 41 56                       	pushq	%r14
141baefe5: 41 55                       	pushq	%r13
141baefe7: 41 54                       	pushq	%r12
141baefe9: 56                          	pushq	%rsi
141baefea: 57                          	pushq	%rdi
141baefeb: 53                          	pushq	%rbx
141baefec: 48 81 ec 78 01 00 00        	subq	$0x178, %rsp            # imm = 0x178
141baeff3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
141baeffb: 44 0f 29 bd e0 00 00 00     	movaps	%xmm15, 0xe0(%rbp)
141baf003: 44 0f 29 b5 d0 00 00 00     	movaps	%xmm14, 0xd0(%rbp)
141baf00b: 44 0f 29 ad c0 00 00 00     	movaps	%xmm13, 0xc0(%rbp)
141baf013: 44 0f 29 a5 b0 00 00 00     	movaps	%xmm12, 0xb0(%rbp)
141baf01b: 44 0f 29 9d a0 00 00 00     	movaps	%xmm11, 0xa0(%rbp)
141baf023: 44 0f 29 95 90 00 00 00     	movaps	%xmm10, 0x90(%rbp)
141baf02b: 44 0f 29 8d 80 00 00 00     	movaps	%xmm9, 0x80(%rbp)
141baf033: 66 44 0f 7f 45 70           	movdqa	%xmm8, 0x70(%rbp)
141baf039: 0f 29 7d 60                 	movaps	%xmm7, 0x60(%rbp)
141baf03d: 0f 29 75 50                 	movaps	%xmm6, 0x50(%rbp)
141baf041: 48 c7 45 48 fe ff ff ff     	movq	$-0x2, 0x48(%rbp)
141baf049: 4c 89 cb                    	movq	%r9, %rbx
141baf04c: 4c 89 45 38                 	movq	%r8, 0x38(%rbp)
141baf050: 49 89 d7                    	movq	%rdx, %r15
141baf053: 49 89 ce                    	movq	%rcx, %r14
141baf056: e8 a5 79 06 ff              	callq	0x140c16a00 <.text+0xc15a00>
141baf05b: 88 45 47                    	movb	%al, 0x47(%rbp)
141baf05e: 84 c0                       	testb	%al, %al
141baf060: 74 5c                       	je	0x141baf0be <.text+0x1bae0be>
141baf062: bf 37 00 00 00              	movl	$0x37, %edi
141baf067: b9 37 00 00 00              	movl	$0x37, %ecx
141baf06c: e8 3f fb ff ff              	callq	0x141baebb0 <.text+0x1badbb0>
141baf071: 4c 8d 25 40 e9 33 01        	leaq	0x133e940(%rip), %r12   # 0x142eed9b8
141baf078: 48 83 f8 01                 	cmpq	$0x1, %rax
141baf07c: 0f 85 bd 00 00 00           	jne	0x141baf13f <.text+0x1bae13f>
141baf082: 49 89 d1                    	movq	%rdx, %r9
141baf085: 48 85 d2                    	testq	%rdx, %rdx
141baf088: 74 53                       	je	0x141baf0dd <.text+0x1bae0dd>
141baf08a: 49 83 f9 37                 	cmpq	$0x37, %r9
141baf08e: 73 4b                       	jae	0x141baf0db <.text+0x1bae0db>
141baf090: 48 8d 05 21 e9 33 01        	leaq	0x133e921(%rip), %rax   # 0x142eed9b8
141baf097: 41 80 3c 01 bf              	cmpb	$-0x41, (%r9,%rax)
141baf09c: 7f 3f                       	jg	0x141baf0dd <.text+0x1bae0dd>
141baf09e: 48 8d 05 d3 e9 33 01        	leaq	0x133e9d3(%rip), %rax   # 0x142eeda78
141baf0a5: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
141baf0aa: 48 8d 0d 07 e9 33 01        	leaq	0x133e907(%rip), %rcx   # 0x142eed9b8
141baf0b1: ba 37 00 00 00              	movl	$0x37, %edx
141baf0b6: 45 31 c0                    	xorl	%r8d, %r8d
141baf0b9: e8 82 a8 d2 00              	callq	0x1428d9940 <.text+0x28d8940>
141baf0be: 48 c7 45 08 00 00 00 00     	movq	$0x0, 0x8(%rbp)
141baf0c6: 48 8b 05 cb cf a7 01        	movq	0x1a7cfcb(%rip), %rax   # 0x14362c098
141baf0cd: 80 38 00                    	cmpb	$0x0, (%rax)
141baf0d0: 0f 84 07 02 00 00           	je	0x141baf2dd <.text+0x1bae2dd>
141baf0d6: e9 7e 04 00 00              	jmp	0x141baf559 <.text+0x1bae559>
141baf0db: 75 c1                       	jne	0x141baf09e <.text+0x1bae09e>
141baf0dd: 4c 89 c9                    	movq	%r9, %rcx
141baf0e0: e8 cb fa ff ff              	callq	0x141baebb0 <.text+0x1badbb0>
141baf0e5: 48 83 f8 01                 	cmpq	$0x1, %rax
141baf0e9: 75 54                       	jne	0x141baf13f <.text+0x1bae13f>
141baf0eb: 49 89 d0                    	movq	%rdx, %r8
141baf0ee: 49 83 c0 02                 	addq	$0x2, %r8
141baf0f2: 74 39                       	je	0x141baf12d <.text+0x1bae12d>
141baf0f4: 49 83 f8 37                 	cmpq	$0x37, %r8
141baf0f8: 73 31                       	jae	0x141baf12b <.text+0x1bae12b>
141baf0fa: 48 8d 05 b7 e8 33 01        	leaq	0x133e8b7(%rip), %rax   # 0x142eed9b8
141baf101: 41 80 3c 00 bf              	cmpb	$-0x41, (%r8,%rax)
141baf106: 7f 25                       	jg	0x141baf12d <.text+0x1bae12d>
141baf108: 48 8d 05 81 e9 33 01        	leaq	0x133e981(%rip), %rax   # 0x142eeda90
141baf10f: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
141baf114: 48 8d 0d 9d e8 33 01        	leaq	0x133e89d(%rip), %rcx   # 0x142eed9b8
141baf11b: ba 37 00 00 00              	movl	$0x37, %edx
141baf120: 41 b9 37 00 00 00           	movl	$0x37, %r9d
141baf126: e8 15 a8 d2 00              	callq	0x1428d9940 <.text+0x28d8940>
141baf12b: 75 db                       	jne	0x141baf108 <.text+0x1bae108>
141baf12d: bf 35 00 00 00              	movl	$0x35, %edi
141baf132: 48 29 d7                    	subq	%rdx, %rdi
141baf135: 4c 8d 25 7c e8 33 01        	leaq	0x133e87c(%rip), %r12   # 0x142eed9b8
141baf13c: 4d 01 c4                    	addq	%r8, %r12
141baf13f: 4c 8d 05 a6 e9 33 01        	leaq	0x133e9a6(%rip), %r8    # 0x142eedaec
141baf146: 4c 8d 2d 73 e9 33 01        	leaq	0x133e973(%rip), %r13   # 0x142eedac0
141baf14d: 0f 1f 00                    	nopl	(%rax)
141baf150: 41 0f be 40 ff              	movsbl	-0x1(%r8), %eax
141baf155: 85 c0                       	testl	%eax, %eax
141baf157: 78 17                       	js	0x141baf170 <.text+0x1bae170>
141baf159: 49 ff c8                    	decq	%r8
141baf15c: 83 f8 5c                    	cmpl	$0x5c, %eax
141baf15f: 75 63                       	jne	0x141baf1c4 <.text+0x1bae1c4>
141baf161: eb 72                       	jmp	0x141baf1d5 <.text+0x1bae1d5>
141baf163: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
141baf170: 41 0f b6 48 fe              	movzbl	-0x2(%r8), %ecx
141baf175: 80 f9 c0                    	cmpb	$-0x40, %cl
141baf178: 7d 23                       	jge	0x141baf19d <.text+0x1bae19d>
141baf17a: 41 0f b6 50 fd              	movzbl	-0x3(%r8), %edx
141baf17f: 80 fa c0                    	cmpb	$-0x40, %dl
141baf182: 7d 22                       	jge	0x141baf1a6 <.text+0x1bae1a6>
141baf184: 45 0f b6 48 fc              	movzbl	-0x4(%r8), %r9d
141baf189: 49 83 c0 fc                 	addq	$-0x4, %r8
141baf18d: 41 83 e1 07                 	andl	$0x7, %r9d
141baf191: 41 c1 e1 06                 	shll	$0x6, %r9d
141baf195: 83 e2 3f                    	andl	$0x3f, %edx
141baf198: 44 09 ca                    	orl	%r9d, %edx
141baf19b: eb 10                       	jmp	0x141baf1ad <.text+0x1bae1ad>
141baf19d: 49 83 c0 fe                 	addq	$-0x2, %r8
141baf1a1: 83 e1 1f                    	andl	$0x1f, %ecx
141baf1a4: eb 0f                       	jmp	0x141baf1b5 <.text+0x1bae1b5>
141baf1a6: 49 83 c0 fd                 	addq	$-0x3, %r8
141baf1aa: 83 e2 0f                    	andl	$0xf, %edx
141baf1ad: c1 e2 06                    	shll	$0x6, %edx
141baf1b0: 83 e1 3f                    	andl	$0x3f, %ecx
141baf1b3: 09 d1                       	orl	%edx, %ecx
141baf1b5: c1 e1 06                    	shll	$0x6, %ecx
141baf1b8: 24 3f                       	andb	$0x3f, %al
141baf1ba: 0f b6 c0                    	movzbl	%al, %eax
141baf1bd: 09 c8                       	orl	%ecx, %eax
141baf1bf: 83 f8 5c                    	cmpl	$0x5c, %eax
141baf1c2: 74 11                       	je	0x141baf1d5 <.text+0x1bae1d5>
141baf1c4: 83 f8 2f                    	cmpl	$0x2f, %eax
141baf1c7: 74 0c                       	je	0x141baf1d5 <.text+0x1bae1d5>
141baf1c9: 4d 39 e8                    	cmpq	%r13, %r8
141baf1cc: 75 82                       	jne	0x141baf150 <.text+0x1bae150>
141baf1ce: be 2c 00 00 00              	movl	$0x2c, %esi
141baf1d3: eb 4c                       	jmp	0x141baf221 <.text+0x1bae221>
141baf1d5: 4c 8d 2d e4 e8 33 01        	leaq	0x133e8e4(%rip), %r13   # 0x142eedac0
141baf1dc: 4d 29 e8                    	subq	%r13, %r8
141baf1df: 49 ff c0                    	incq	%r8
141baf1e2: 74 32                       	je	0x141baf216 <.text+0x1bae216>
141baf1e4: 49 83 f8 2c                 	cmpq	$0x2c, %r8
141baf1e8: 73 2a                       	jae	0x141baf214 <.text+0x1bae214>
141baf1ea: 43 80 3c 28 bf              	cmpb	$-0x41, (%r8,%r13)
141baf1ef: 7f 25                       	jg	0x141baf216 <.text+0x1bae216>
141baf1f1: 48 8d 05 60 e8 33 01        	leaq	0x133e860(%rip), %rax   # 0x142eeda58
141baf1f8: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
141baf1fd: 48 8d 0d bc e8 33 01        	leaq	0x133e8bc(%rip), %rcx   # 0x142eedac0
141baf204: ba 2c 00 00 00              	movl	$0x2c, %edx
141baf209: 41 b9 2c 00 00 00           	movl	$0x2c, %r9d
141baf20f: e8 2c a7 d2 00              	callq	0x1428d9940 <.text+0x28d8940>
141baf214: 75 db                       	jne	0x141baf1f1 <.text+0x1bae1f1>
141baf216: be 2c 00 00 00              	movl	$0x2c, %esi
141baf21b: 4c 29 c6                    	subq	%r8, %rsi
141baf21e: 4d 01 c5                    	addq	%r8, %r13
141baf221: e8 0a 8f 06 ff              	callq	0x140c18130 <.text+0xc17130>
141baf226: 48 89 c1                    	movq	%rax, %rcx
141baf229: 48 8b 00                    	movq	(%rax), %rax
141baf22c: 48 83 f8 01                 	cmpq	$0x1, %rax
141baf230: 0f 85 7c 03 00 00           	jne	0x141baf5b2 <.text+0x1bae5b2>
141baf236: 48 83 c1 08                 	addq	$0x8, %rcx
141baf23a: 48 83 39 00                 	cmpq	$0x0, (%rcx)
141baf23e: 0f 85 d0 03 00 00           	jne	0x141baf614 <.text+0x1bae614>
141baf244: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
141baf24b: 48 89 4d 30                 	movq	%rcx, 0x30(%rbp)
141baf24f: 48 83 c1 08                 	addq	$0x8, %rcx
141baf253: 48 89 74 24 20              	movq	%rsi, 0x20(%rsp)
141baf258: 48 c7 44 24 30 00 00 00 00  	movq	$0x0, 0x30(%rsp)
141baf261: 48 c7 44 24 28 01 00 00 00  	movq	$0x1, 0x28(%rsp)
141baf26a: 4c 89 e2                    	movq	%r12, %rdx
141baf26d: 49 89 f8                    	movq	%rdi, %r8
141baf270: 4d 89 e9                    	movq	%r13, %r9
141baf273: e8 c8 7a 06 ff              	callq	0x140c16d40 <.text+0xc15d40>
141baf278: 48 89 c6                    	movq	%rax, %rsi
141baf27b: 48 8b 45 30                 	movq	0x30(%rbp), %rax
141baf27f: 48 ff 00                    	incq	(%rax)
141baf282: 48 89 75 10                 	movq	%rsi, 0x10(%rbp)
141baf286: 48 c7 45 08 01 00 00 00     	movq	$0x1, 0x8(%rbp)
141baf28e: 48 8b 05 03 ce a7 01        	movq	0x1a7ce03(%rip), %rax   # 0x14362c098
141baf295: 80 38 01                    	cmpb	$0x1, (%rax)
141baf298: 75 3f                       	jne	0x141baf2d9 <.text+0x1bae2d9>
141baf29a: e8 91 8e 06 ff              	callq	0x140c18130 <.text+0xc17130>
141baf29f: 48 89 c1                    	movq	%rax, %rcx
141baf2a2: 48 8b 00                    	movq	(%rax), %rax
141baf2a5: 48 83 f8 01                 	cmpq	$0x1, %rax
141baf2a9: 0f 85 41 03 00 00           	jne	0x141baf5f0 <.text+0x1bae5f0>
141baf2af: 48 83 c1 08                 	addq	$0x8, %rcx
141baf2b3: 48 83 39 00                 	cmpq	$0x0, (%rcx)
141baf2b7: 0f 85 57 03 00 00           	jne	0x141baf614 <.text+0x1bae614>
141baf2bd: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
141baf2c4: 48 89 4d 38                 	movq	%rcx, 0x38(%rbp)
141baf2c8: 48 83 c1 08                 	addq	$0x8, %rcx
141baf2cc: 48 89 f2                    	movq	%rsi, %rdx
141baf2cf: e8 1c 7e 06 ff              	callq	0x140c170f0 <.text+0xc160f0>
141baf2d4: e9 79 02 00 00              	jmp	0x141baf552 <.text+0x1bae552>
141baf2d9: 48 89 75 30                 	movq	%rsi, 0x30(%rbp)
141baf2dd: 49 8b 0f                    	movq	(%r15), %rcx
141baf2e0: e8 9b ea ac fe              	callq	0x14067dd80 <.text+0x67cd80>
141baf2e5: 48 8d 48 08                 	leaq	0x8(%rax), %rcx
141baf2e9: 48 8d 50 20                 	leaq	0x20(%rax), %rdx
141baf2ed: 4c 8d 40 10                 	leaq	0x10(%rax), %r8
141baf2f1: 49 89 c1                    	movq	%rax, %r9
141baf2f4: 49 83 c1 28                 	addq	$0x28, %r9
141baf2f8: 80 b8 98 02 00 00 00        	cmpb	$0x0, 0x298(%rax)
141baf2ff: 48 0f 45 ca                 	cmovneq	%rdx, %rcx
141baf303: 4d 0f 44 c8                 	cmoveq	%r8, %r9
141baf307: 49 8b 01                    	movq	(%r9), %rax
141baf30a: 48 8b 09                    	movq	(%rcx), %rcx
141baf30d: 48 89 4d c0                 	movq	%rcx, -0x40(%rbp)
141baf311: 48 89 45 c8                 	movq	%rax, -0x38(%rbp)
141baf315: 49 8b 36                    	movq	(%r14), %rsi
141baf318: 48 8d 7e 01                 	leaq	0x1(%rsi), %rdi
141baf31c: 49 89 3e                    	movq	%rdi, (%r14)
141baf31f: 48 8b 13                    	movq	(%rbx), %rdx
141baf322: 48 8d 4d f8                 	leaq	-0x8(%rbp), %rcx
141baf326: e8 a5 2a d6 fe              	callq	0x140911dd0 <.text+0x910dd0>
141baf32b: 48 8d 04 bf                 	leaq	(%rdi,%rdi,4), %rax
141baf32f: 48 8d 0c 46                 	leaq	(%rsi,%rax,2), %rcx
141baf333: 48 ff c1                    	incq	%rcx
141baf336: 0f b7 c1                    	movzwl	%cx, %eax
141baf339: 8d 14 09                    	leal	(%rcx,%rcx), %edx
141baf33c: c1 e8 02                    	shrl	$0x2, %eax
141baf33f: 66 44 0f 6e c2              	movd	%edx, %xmm8
141baf344: 66 44 0f c4 c0 01           	pinsrw	$0x1, %eax, %xmm8
141baf34a: 66 44 0f db 05 6d e5 33 01  	pand	0x133e56d(%rip), %xmm8  # 0x142eed8c0
141baf353: 66 44 0f fd 05 74 e5 33 01  	paddw	0x133e574(%rip), %xmm8  # 0x142eed8d0
141baf35c: 48 8b 85 60 01 00 00        	movq	0x160(%rbp), %rax
141baf363: 66 41 0f 61 c0              	punpcklwd	%xmm8, %xmm0    # xmm0 = xmm0[0],xmm8[0],xmm0[1],xmm8[1],xmm0[2],xmm8[2],xmm0[3],xmm8[3]
141baf368: 66 0f 72 e0 10              	psrad	$0x10, %xmm0
141baf36d: 44 0f 5b c8                 	cvtdq2ps	%xmm0, %xmm9
141baf371: 44 0f 59 0d 67 e5 33 01     	mulps	0x133e567(%rip), %xmm9  # 0x142eed8e0
141baf379: 44 0f 59 0d 6f e5 33 01     	mulps	0x133e56f(%rip), %xmm9  # 0x142eed8f0
141baf381: 8b 50 1c                    	movl	0x1c(%rax), %edx
141baf384: 4c 8b 40 10                 	movq	0x10(%rax), %r8
141baf388: 41 89 10                    	movl	%edx, (%r8)
141baf38b: 48 8b 00                    	movq	(%rax), %rax
141baf38e: 4c 8b 78 08                 	movq	0x8(%rax), %r15
141baf392: 48 8b 40 10                 	movq	0x10(%rax), %rax
141baf396: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
141baf39a: 41 bd 01 00 00 00           	movl	$0x1, %r13d
141baf3a0: 49 d3 e5                    	shlq	%cl, %r13
141baf3a3: 49 8d 3c c7                 	leaq	(%r15,%rax,8), %rdi
141baf3a7: f2 44 0f 10 55 f8           	movsd	-0x8(%rbp), %xmm10
141baf3ad: 48 c7 c6 fe ff ff ff        	movq	$-0x2, %rsi
141baf3b4: 48 d3 c6                    	rolq	%cl, %rsi
141baf3b7: f3 44 0f 10 5d 00           	movss	(%rbp), %xmm11
141baf3bd: 48 8d 5d 18                 	leaq	0x18(%rbp), %rbx
141baf3c1: 4c 8d 75 28                 	leaq	0x28(%rbp), %r14
141baf3c5: f3 44 0f 10 25 32 e5 33 01  	movss	0x133e532(%rip), %xmm12 # 0x142eed900
141baf3ce: f3 0f 10 35 82 c2 ec 00     	movss	0xecc282(%rip), %xmm6   # 0x142a7b658
141baf3d6: f3 44 0f 10 2d 91 64 d7 00  	movss	0xd76491(%rip), %xmm13  # 0x142925870
141baf3df: f3 0f 10 3d 1d 65 d7 00     	movss	0xd7651d(%rip), %xmm7   # 0x142925904
141baf3e7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
141baf3f0: 49 39 ff                    	cmpq	%rdi, %r15
141baf3f3: 0f 84 18 01 00 00           	je	0x141baf511 <.text+0x1bae511>
141baf3f9: 4d 89 fc                    	movq	%r15, %r12
141baf3fc: f2 41 0f 10 07              	movsd	(%r15), %xmm0
141baf401: 41 0f 58 c1                 	addps	%xmm9, %xmm0
141baf405: 0f 13 45 28                 	movlps	%xmm0, 0x28(%rbp)
141baf409: 48 89 d9                    	movq	%rbx, %rcx
141baf40c: 4c 89 f2                    	movq	%r14, %rdx
141baf40f: e8 ac d9 0e ff              	callq	0x140c9cdc0 <.text+0xc9bdc0>
141baf414: 4d 8d 7c 24 18              	leaq	0x18(%r12), %r15
141baf419: f2 0f 10 45 18              	movsd	0x18(%rbp), %xmm0
141baf41e: 45 0f 28 f2                 	movaps	%xmm10, %xmm14
141baf422: 44 0f 5c f0                 	subps	%xmm0, %xmm14
141baf426: 45 0f 28 fb                 	movaps	%xmm11, %xmm15
141baf42a: f3 44 0f 5c 7d 20           	subss	0x20(%rbp), %xmm15
141baf430: 41 0f 28 c6                 	movaps	%xmm14, %xmm0
141baf434: 41 0f 59 c6                 	mulps	%xmm14, %xmm0
141baf438: f3 0f 16 c8                 	movshdup	%xmm0, %xmm1            # xmm1 = xmm0[1,1,3,3]
141baf43c: f3 0f 58 c8                 	addss	%xmm0, %xmm1
141baf440: 41 0f 28 c7                 	movaps	%xmm15, %xmm0
141baf444: f3 41 0f 59 c7              	mulss	%xmm15, %xmm0
141baf449: f3 0f 58 c1                 	addss	%xmm1, %xmm0
141baf44d: 44 0f 2e e0                 	ucomiss	%xmm0, %xmm12
141baf451: 0f 86 89 00 00 00           	jbe	0x141baf4e0 <.text+0x1bae4e0>
141baf457: f3 0f 10 4d 28              	movss	0x28(%rbp), %xmm1
141baf45c: f3 0f 10 55 2c              	movss	0x2c(%rbp), %xmm2
141baf461: c7 44 24 20 00 00 02 43     	movl	$0x43020000, 0x20(%rsp) # imm = 0x43020000
141baf469: 48 8d 4d c0                 	leaq	-0x40(%rbp), %rcx
141baf46d: 0f 28 de                    	movaps	%xmm6, %xmm3
141baf470: e8 7b 59 e6 fe              	callq	0x140a14df0 <.text+0xa13df0>
141baf475: 41 0f 2e c5                 	ucomiss	%xmm13, %xmm0
141baf479: 76 65                       	jbe	0x141baf4e0 <.text+0x1bae4e0>
141baf47b: 8b 45 20                    	movl	0x20(%rbp), %eax
141baf47e: 89 45 d8                    	movl	%eax, -0x28(%rbp)
141baf481: 48 8b 45 18                 	movq	0x18(%rbp), %rax
141baf485: 48 89 45 d0                 	movq	%rax, -0x30(%rbp)
141baf489: 44 0f 13 75 dc              	movlps	%xmm14, -0x24(%rbp)
141baf48e: f3 44 0f 11 7d e4           	movss	%xmm15, -0x1c(%rbp)
141baf494: 48 8b 4d 38                 	movq	0x38(%rbp), %rcx
141baf498: 48 8d 55 d0                 	leaq	-0x30(%rbp), %rdx
141baf49c: 0f 28 d7                    	movaps	%xmm7, %xmm2
141baf49f: 41 b9 a3 32 00 00           	movl	$0x32a3, %r9d           # imm = 0x32A3
141baf4a5: e8 76 ce d1 fe              	callq	0x1408cc320 <.text+0x8cb320>
141baf4aa: 49 8b 4c 24 08              	movq	0x8(%r12), %rcx
141baf4af: 84 c0                       	testb	%al, %al
141baf4b1: 75 32                       	jne	0x141baf4e5 <.text+0x1bae4e5>
141baf4b3: 48 89 c8                    	movq	%rcx, %rax
141baf4b6: 4c 09 e8                    	orq	%r13, %rax
141baf4b9: 49 89 44 24 08              	movq	%rax, 0x8(%r12)
141baf4be: 4c 85 e9                    	testq	%r13, %rcx
141baf4c1: 0f 85 29 ff ff ff           	jne	0x141baf3f0 <.text+0x1bae3f0>
141baf4c7: f3 41 0f 7e 44 24 10        	movq	0x10(%r12), %xmm0
141baf4ce: 66 41 0f fd c0              	paddw	%xmm8, %xmm0
141baf4d3: 66 41 0f 7e 44 24 10        	movd	%xmm0, 0x10(%r12)
141baf4da: e9 11 ff ff ff              	jmp	0x141baf3f0 <.text+0x1bae3f0>
141baf4df: 90                          	nop
141baf4e0: 49 8b 4c 24 08              	movq	0x8(%r12), %rcx
141baf4e5: 48 89 c8                    	movq	%rcx, %rax
141baf4e8: 48 21 f0                    	andq	%rsi, %rax
141baf4eb: 49 89 44 24 08              	movq	%rax, 0x8(%r12)
141baf4f0: 4c 85 e9                    	testq	%r13, %rcx
141baf4f3: 0f 84 f7 fe ff ff           	je	0x141baf3f0 <.text+0x1bae3f0>
141baf4f9: f3 41 0f 7e 44 24 10        	movq	0x10(%r12), %xmm0
141baf500: 66 41 0f f9 c0              	psubw	%xmm8, %xmm0
141baf505: 66 41 0f 7e 44 24 10        	movd	%xmm0, 0x10(%r12)
141baf50c: e9 df fe ff ff              	jmp	0x141baf3f0 <.text+0x1bae3f0>
141baf511: 80 7d 47 00                 	cmpb	$0x0, 0x47(%rbp)
141baf515: 74 42                       	je	0x141baf559 <.text+0x1bae559>
141baf517: e8 14 8c 06 ff              	callq	0x140c18130 <.text+0xc17130>
141baf51c: 48 89 c1                    	movq	%rax, %rcx
141baf51f: 48 8b 00                    	movq	(%rax), %rax
141baf522: 48 83 f8 01                 	cmpq	$0x1, %rax
141baf526: 0f 85 a0 00 00 00           	jne	0x141baf5cc <.text+0x1bae5cc>
141baf52c: 48 83 c1 08                 	addq	$0x8, %rcx
141baf530: 48 8b 55 30                 	movq	0x30(%rbp), %rdx
141baf534: 48 83 39 00                 	cmpq	$0x0, (%rcx)
141baf538: 0f 85 d6 00 00 00           	jne	0x141baf614 <.text+0x1bae614>
141baf53e: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
141baf545: 48 89 4d 38                 	movq	%rcx, 0x38(%rbp)
141baf549: 48 83 c1 08                 	addq	$0x8, %rcx
141baf54d: e8 9e 7b 06 ff              	callq	0x140c170f0 <.text+0xc160f0>
141baf552: 48 8b 45 38                 	movq	0x38(%rbp), %rax
141baf556: 48 ff 00                    	incq	(%rax)
141baf559: 0f 28 75 50                 	movaps	0x50(%rbp), %xmm6
141baf55d: 0f 28 7d 60                 	movaps	0x60(%rbp), %xmm7
141baf561: 44 0f 28 45 70              	movaps	0x70(%rbp), %xmm8
141baf566: 44 0f 28 8d 80 00 00 00     	movaps	0x80(%rbp), %xmm9
141baf56e: 44 0f 28 95 90 00 00 00     	movaps	0x90(%rbp), %xmm10
141baf576: 44 0f 28 9d a0 00 00 00     	movaps	0xa0(%rbp), %xmm11
141baf57e: 44 0f 28 a5 b0 00 00 00     	movaps	0xb0(%rbp), %xmm12
141baf586: 44 0f 28 ad c0 00 00 00     	movaps	0xc0(%rbp), %xmm13
141baf58e: 44 0f 28 b5 d0 00 00 00     	movaps	0xd0(%rbp), %xmm14
141baf596: 44 0f 28 bd e0 00 00 00     	movaps	0xe0(%rbp), %xmm15
141baf59e: 48 81 c4 78 01 00 00        	addq	$0x178, %rsp            # imm = 0x178
141baf5a5: 5b                          	popq	%rbx
141baf5a6: 5f                          	popq	%rdi
141baf5a7: 5e                          	popq	%rsi
141baf5a8: 41 5c                       	popq	%r12
141baf5aa: 41 5d                       	popq	%r13
141baf5ac: 41 5e                       	popq	%r14
141baf5ae: 41 5f                       	popq	%r15
141baf5b0: 5d                          	popq	%rbp
141baf5b1: c3                          	retq
141baf5b2: 48 85 c0                    	testq	%rax, %rax
141baf5b5: 75 51                       	jne	0x141baf608 <.text+0x1bae608>
141baf5b7: 31 d2                       	xorl	%edx, %edx
141baf5b9: e8 82 ad d6 00              	callq	0x14291a340 <.text+0x2919340>
141baf5be: 48 89 c1                    	movq	%rax, %rcx
141baf5c1: 48 85 c0                    	testq	%rax, %rax
141baf5c4: 0f 85 70 fc ff ff           	jne	0x141baf23a <.text+0x1bae23a>
141baf5ca: eb 3c                       	jmp	0x141baf608 <.text+0x1bae608>
141baf5cc: 48 85 c0                    	testq	%rax, %rax
141baf5cf: 48 8b 45 30                 	movq	0x30(%rbp), %rax
141baf5d3: 75 33                       	jne	0x141baf608 <.text+0x1bae608>
141baf5d5: 48 89 c6                    	movq	%rax, %rsi
141baf5d8: 31 d2                       	xorl	%edx, %edx
141baf5da: e8 61 ad d6 00              	callq	0x14291a340 <.text+0x2919340>
141baf5df: 48 89 f2                    	movq	%rsi, %rdx
141baf5e2: 48 89 c1                    	movq	%rax, %rcx
141baf5e5: 48 85 c0                    	testq	%rax, %rax
141baf5e8: 0f 85 46 ff ff ff           	jne	0x141baf534 <.text+0x1bae534>
141baf5ee: eb 18                       	jmp	0x141baf608 <.text+0x1bae608>
141baf5f0: 48 85 c0                    	testq	%rax, %rax
141baf5f3: 75 13                       	jne	0x141baf608 <.text+0x1bae608>
141baf5f5: 31 d2                       	xorl	%edx, %edx
141baf5f7: e8 44 ad d6 00              	callq	0x14291a340 <.text+0x2919340>
141baf5fc: 48 89 c1                    	movq	%rax, %rcx
141baf5ff: 48 85 c0                    	testq	%rax, %rax
141baf602: 0f 85 ab fc ff ff           	jne	0x141baf2b3 <.text+0x1bae2b3>
141baf608: 48 8d 0d 79 4f 48 01        	leaq	0x1484f79(%rip), %rcx   # 0x143034588
141baf60f: e8 bc 8a d2 00              	callq	0x1428d80d0 <.text+0x28d70d0>
141baf614: 48 8d 0d dd 4f 48 01        	leaq	0x1484fdd(%rip), %rcx   # 0x1430345f8
141baf61b: e8 30 9c d2 00              	callq	0x1428d9250 <.text+0x28d8250>
141baf620: cc                          	int3
141baf621: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
141baf630: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141baf635: 55                          	pushq	%rbp
141baf636: 41 57                       	pushq	%r15
141baf638: 41 56                       	pushq	%r14
141baf63a: 41 55                       	pushq	%r13
141baf63c: 41 54                       	pushq	%r12
141baf63e: 56                          	pushq	%rsi
141baf63f: 57                          	pushq	%rdi
141baf640: 53                          	pushq	%rbx
141baf641: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
141baf648: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141baf64f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
141baf655: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
141baf65b: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
141baf661: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
141baf667: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
141baf66d: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
141baf676: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
141baf67f: 66 44 0f 7f 84 24 a0 00 00 00       	movdqa	%xmm8, 0xa0(%rsp)
141baf689: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
141baf691: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
141baf699: 48 8b 45 30                 	movq	0x30(%rbp), %rax
141baf69d: 48 ff 00                    	incq	(%rax)
141baf6a0: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
141baf6a8: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
141baf6b0: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
141baf6b9: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
141baf6c2: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
141baf6cb: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
141baf6d1: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
141baf6d7: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
141baf6dd: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
141baf6e3: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
141baf6e9: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
141baf6f0: 5b                          	popq	%rbx
141baf6f1: 5f                          	popq	%rdi
141baf6f2: 5e                          	popq	%rsi
141baf6f3: 41 5c                       	popq	%r12
141baf6f5: 41 5d                       	popq	%r13
141baf6f7: 41 5e                       	popq	%r14
141baf6f9: 41 5f                       	popq	%r15
141baf6fb: 5d                          	popq	%rbp
141baf6fc: c3                          	retq
141baf6fd: 0f 1f 00                    	nopl	(%rax)
141baf700: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141baf705: 55                          	pushq	%rbp
141baf706: 41 57                       	pushq	%r15
141baf708: 41 56                       	pushq	%r14
141baf70a: 41 55                       	pushq	%r13
141baf70c: 41 54                       	pushq	%r12
141baf70e: 56                          	pushq	%rsi
141baf70f: 57                          	pushq	%rdi
141baf710: 53                          	pushq	%rbx
141baf711: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
141baf718: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141baf71f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
141baf725: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
141baf72b: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
141baf731: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
141baf737: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
141baf73d: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
141baf746: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
141baf74f: 66 44 0f 7f 84 24 a0 00 00 00       	movdqa	%xmm8, 0xa0(%rsp)
141baf759: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
141baf761: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
141baf769: 48 8b 45 38                 	movq	0x38(%rbp), %rax
141baf76d: 48 ff 00                    	incq	(%rax)
141baf770: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
141baf778: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
141baf780: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
141baf789: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
141baf792: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
141baf79b: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
141baf7a1: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
141baf7a7: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
141baf7ad: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
141baf7b3: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
141baf7b9: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
141baf7c0: 5b                          	popq	%rbx
141baf7c1: 5f                          	popq	%rdi
141baf7c2: 5e                          	popq	%rsi
141baf7c3: 41 5c                       	popq	%r12
141baf7c5: 41 5d                       	popq	%r13
141baf7c7: 41 5e                       	popq	%r14
141baf7c9: 41 5f                       	popq	%r15
141baf7cb: 5d                          	popq	%rbp
141baf7cc: c3                          	retq
141baf7cd: 0f 1f 00                    	nopl	(%rax)
141baf7d0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141baf7d5: 55                          	pushq	%rbp
141baf7d6: 41 57                       	pushq	%r15
141baf7d8: 41 56                       	pushq	%r14
141baf7da: 41 55                       	pushq	%r13
141baf7dc: 41 54                       	pushq	%r12
141baf7de: 56                          	pushq	%rsi
141baf7df: 57                          	pushq	%rdi
141baf7e0: 53                          	pushq	%rbx
141baf7e1: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
141baf7e8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141baf7ef: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
141baf7f5: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
141baf7fb: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
141baf801: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
141baf807: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
141baf80d: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
141baf816: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
141baf81f: 66 44 0f 7f 84 24 a0 00 00 00       	movdqa	%xmm8, 0xa0(%rsp)
141baf829: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
141baf831: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
141baf839: 80 7d 47 00                 	cmpb	$0x0, 0x47(%rbp)
141baf83d: 74 20                       	je	0x141baf85f <.text+0x1bae85f>
141baf83f: 48 8d 45 10                 	leaq	0x10(%rbp), %rax
141baf843: 48 89 45 f0                 	movq	%rax, -0x10(%rbp)
141baf847: 48 8d 45 f0                 	leaq	-0x10(%rbp), %rax
141baf84b: 48 89 45 e8                 	movq	%rax, -0x18(%rbp)
141baf84f: 48 8d 0d a2 e1 33 01        	leaq	0x133e1a2(%rip), %rcx   # 0x142eed9f8
141baf856: 48 8d 55 e8                 	leaq	-0x18(%rbp), %rdx
141baf85a: e8 01 3a 6b 00              	callq	0x142263260 <.text+0x2262260>
141baf85f: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
141baf867: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
141baf86f: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
141baf878: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
141baf881: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
141baf88a: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
141baf890: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
141baf896: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
141baf89c: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
141baf8a2: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
141baf8a8: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
141baf8af: 5b                          	popq	%rbx
141baf8b0: 5f                          	popq	%rdi
141baf8b1: 5e                          	popq	%rsi
141baf8b2: 41 5c                       	popq	%r12
141baf8b4: 41 5d                       	popq	%r13
141baf8b6: 41 5e                       	popq	%r14
141baf8b8: 41 5f                       	popq	%r15
141baf8ba: 5d                          	popq	%rbp
141baf8bb: c3                          	retq
141baf8bc: 0f 1f 40 00                 	nopl	(%rax)
141baf8c0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141baf8c5: 55                          	pushq	%rbp
141baf8c6: 41 57                       	pushq	%r15
141baf8c8: 41 56                       	pushq	%r14
141baf8ca: 41 55                       	pushq	%r13
141baf8cc: 41 54                       	pushq	%r12
141baf8ce: 56                          	pushq	%rsi
141baf8cf: 57                          	pushq	%rdi
141baf8d0: 53                          	pushq	%rbx
141baf8d1: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
141baf8d8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141baf8df: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
141baf8e5: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
141baf8eb: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
141baf8f1: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
141baf8f7: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
141baf8fd: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
141baf906: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
141baf90f: 66 44 0f 7f 84 24 a0 00 00 00       	movdqa	%xmm8, 0xa0(%rsp)
141baf919: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
141baf921: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
141baf929: 48 8b 45 38                 	movq	0x38(%rbp), %rax
141baf92d: 48 ff 00                    	incq	(%rax)
141baf930: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
141baf938: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
141baf940: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
141baf949: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
141baf952: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
141baf95b: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
141baf961: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
141baf967: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
141baf96d: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
141baf973: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
141baf979: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
141baf980: 5b                          	popq	%rbx
141baf981: 5f                          	popq	%rdi
141baf982: 5e                          	popq	%rsi
141baf983: 41 5c                       	popq	%r12
141baf985: 41 5d                       	popq	%r13
141baf987: 41 5e                       	popq	%r14
141baf989: 41 5f                       	popq	%r15
141baf98b: 5d                          	popq	%rbp
141baf98c: c3                          	retq

D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140b1ae70: 55                          	pushq	%rbp
140b1ae71: 56                          	pushq	%rsi
140b1ae72: 48 81 ec e8 00 00 00        	subq	$0xe8, %rsp
140b1ae79: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140b1ae81: 48 c7 45 60 fe ff ff ff     	movq	$-0x2, 0x60(%rbp)
140b1ae89: 48 89 d6                    	movq	%rdx, %rsi
140b1ae8c: 48 8b 01                    	movq	(%rcx), %rax
140b1ae8f: 48 8b 10                    	movq	(%rax), %rdx
140b1ae92: 48 85 d2                    	testq	%rdx, %rdx
140b1ae95: 74 5a                       	je	0x140b1aef1 <.text+0xb19ef1>
140b1ae97: 48 83 c2 10                 	addq	$0x10, %rdx
140b1ae9b: 4c 8d 05 06 e6 fc 01        	leaq	0x1fce606(%rip), %r8    # 0x142ae94a8
140b1aea2: 48 8d 4d 20                 	leaq	0x20(%rbp), %rcx
140b1aea6: 41 b9 11 00 00 00           	movl	$0x11, %r9d
140b1aeac: e8 8f 95 f3 ff              	callq	0x140a54440 <.text+0xa53440>
140b1aeb1: 48 83 7d 20 02              	cmpq	$0x2, 0x20(%rbp)
140b1aeb6: 0f 85 82 00 00 00           	jne	0x140b1af3e <.text+0xb19f3e>
140b1aebc: 48 8b 45 28                 	movq	0x28(%rbp), %rax
140b1aec0: 48 89 45 a8                 	movq	%rax, -0x58(%rbp)
140b1aec4: 48 8d 05 2d e6 fc 01        	leaq	0x1fce62d(%rip), %rax   # 0x142ae94f8
140b1aecb: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140b1aed0: 48 8d 0d b1 e1 fc 01        	leaq	0x1fce1b1(%rip), %rcx   # 0x142ae9088
140b1aed7: 4c 8d 0d 8a e1 fc 01        	leaq	0x1fce18a(%rip), %r9    # 0x142ae9068
140b1aede: 4c 8d 45 a8                 	leaq	-0x58(%rbp), %r8
140b1aee2: ba 2b 00 00 00              	movl	$0x2b, %edx
140b1aee7: e8 74 e8 db 01              	callq	0x1428d9760 <.text+0x28d8760>
140b1aeec: e9 b2 01 00 00              	jmp	0x140b1b0a3 <.text+0xb1a0a3>
140b1aef1: 0f b6 05 11 9d b1 02        	movzbl	0x2b19d11(%rip), %eax   # 0x143634c09
140b1aef8: b9 11 00 00 00              	movl	$0x11, %ecx
140b1aefd: ba 01 00 00 00              	movl	$0x1, %edx
140b1af02: e8 09 8d af ff              	callq	0x140613c10 <.text+0x612c10>
140b1af07: 48 85 c0                    	testq	%rax, %rax
140b1af0a: 0f 84 95 01 00 00           	je	0x140b1b0a5 <.text+0xb1a0a5>
140b1af10: 0f 10 05 91 e5 fc 01        	movups	0x1fce591(%rip), %xmm0  # 0x142ae94a8
140b1af17: 0f 11 00                    	movups	%xmm0, (%rax)
140b1af1a: c6 40 10 67                 	movb	$0x67, 0x10(%rax)
140b1af1e: 48 c7 45 20 00 00 00 00     	movq	$0x0, 0x20(%rbp)
140b1af26: 48 c7 45 30 11 00 00 00     	movq	$0x11, 0x30(%rbp)
140b1af2e: 48 89 45 38                 	movq	%rax, 0x38(%rbp)
140b1af32: 48 c7 45 40 11 00 00 00     	movq	$0x11, 0x40(%rbp)
140b1af3a: c6 45 48 02                 	movb	$0x2, 0x48(%rbp)
140b1af3e: 48 8b 45 20                 	movq	0x20(%rbp), %rax
140b1af42: 48 8b 4d 28                 	movq	0x28(%rbp), %rcx
140b1af46: 48 8b 55 30                 	movq	0x30(%rbp), %rdx
140b1af4a: 4c 8b 45 38                 	movq	0x38(%rbp), %r8
140b1af4e: 4c 8b 4d 40                 	movq	0x40(%rbp), %r9
140b1af52: 4c 89 4d 00                 	movq	%r9, (%rbp)
140b1af56: 44 0f b6 4d 48              	movzbl	0x48(%rbp), %r9d
140b1af5b: 44 88 4d 08                 	movb	%r9b, 0x8(%rbp)
140b1af5f: 44 8b 4d 49                 	movl	0x49(%rbp), %r9d
140b1af63: 44 89 4d 09                 	movl	%r9d, 0x9(%rbp)
140b1af67: 44 0f b7 4d 4d              	movzwl	0x4d(%rbp), %r9d
140b1af6c: 66 44 89 4d 0d              	movw	%r9w, 0xd(%rbp)
140b1af71: 44 0f b6 4d 4f              	movzbl	0x4f(%rbp), %r9d
140b1af76: 44 88 4d 0f                 	movb	%r9b, 0xf(%rbp)
140b1af7a: 48 89 55 f0                 	movq	%rdx, -0x10(%rbp)
140b1af7e: 4c 89 45 f8                 	movq	%r8, -0x8(%rbp)
140b1af82: 48 89 45 e0                 	movq	%rax, -0x20(%rbp)
140b1af86: 48 89 4d e8                 	movq	%rcx, -0x18(%rbp)
140b1af8a: 83 7d e0 01                 	cmpl	$0x1, -0x20(%rbp)
140b1af8e: 75 25                       	jne	0x140b1afb5 <.text+0xb19fb5>
140b1af90: 48 8d 55 e8                 	leaq	-0x18(%rbp), %rdx
140b1af94: c6 45 5f 01                 	movb	$0x1, 0x5f(%rbp)
140b1af98: 48 8d 4d a8                 	leaq	-0x58(%rbp), %rcx
140b1af9c: e8 2f c3 91 00              	callq	0x1414372d0 <.text+0x14362d0>
140b1afa1: 48 b8 04 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffc, %rax # imm = 0x8000000000000004
140b1afab: 48 39 45 a8                 	cmpq	%rax, -0x58(%rbp)
140b1afaf: 0f 85 a6 00 00 00           	jne	0x140b1b05b <.text+0xb1a05b>
140b1afb5: 48 8b 45 e0                 	movq	-0x20(%rbp), %rax
140b1afb9: 48 8b 4d e8                 	movq	-0x18(%rbp), %rcx
140b1afbd: 48 8b 55 f0                 	movq	-0x10(%rbp), %rdx
140b1afc1: 4c 8b 45 f8                 	movq	-0x8(%rbp), %r8
140b1afc5: 4c 8b 4d 00                 	movq	(%rbp), %r9
140b1afc9: 4c 89 4d 40                 	movq	%r9, 0x40(%rbp)
140b1afcd: 44 8b 4d 08                 	movl	0x8(%rbp), %r9d
140b1afd1: 44 89 4d 48                 	movl	%r9d, 0x48(%rbp)
140b1afd5: 44 0f b7 4d 0c              	movzwl	0xc(%rbp), %r9d
140b1afda: 66 44 89 4d 4c              	movw	%r9w, 0x4c(%rbp)
140b1afdf: 44 0f b6 4d 0e              	movzbl	0xe(%rbp), %r9d
140b1afe4: 44 88 4d 4e                 	movb	%r9b, 0x4e(%rbp)
140b1afe8: 44 0f b6 4d 0f              	movzbl	0xf(%rbp), %r9d
140b1afed: 44 88 4d 4f                 	movb	%r9b, 0x4f(%rbp)
140b1aff1: 48 89 55 30                 	movq	%rdx, 0x30(%rbp)
140b1aff5: 4c 89 45 38                 	movq	%r8, 0x38(%rbp)
140b1aff9: 48 89 45 20                 	movq	%rax, 0x20(%rbp)
140b1affd: 48 89 4d 28                 	movq	%rcx, 0x28(%rbp)
140b1b001: 8b 46 1c                    	movl	0x1c(%rsi), %eax
140b1b004: 48 8b 4e 10                 	movq	0x10(%rsi), %rcx
140b1b008: 89 01                       	movl	%eax, (%rcx)
140b1b00a: 48 8b 36                    	movq	(%rsi), %rsi
140b1b00d: 83 3e 02                    	cmpl	$0x2, (%rsi)
140b1b010: 74 28                       	je	0x140b1b03a <.text+0xb1a03a>
140b1b012: 48 89 75 18                 	movq	%rsi, 0x18(%rbp)
140b1b016: 48 89 f1                    	movq	%rsi, %rcx
140b1b019: e8 62 f2 d7 ff              	callq	0x14089a280 <.text+0x899280>
140b1b01e: 48 8b 75 18                 	movq	0x18(%rbp), %rsi
140b1b022: 48 8b 56 10                 	movq	0x10(%rsi), %rdx
140b1b026: 48 85 d2                    	testq	%rdx, %rdx
140b1b029: 74 0f                       	je	0x140b1b03a <.text+0xb1a03a>
140b1b02b: 48 8b 4e 18                 	movq	0x18(%rsi), %rcx
140b1b02f: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140b1b035: e8 e6 8b af ff              	callq	0x140613c20 <.text+0x612c20>
140b1b03a: 0f 28 45 20                 	movaps	0x20(%rbp), %xmm0
140b1b03e: 0f 28 4d 30                 	movaps	0x30(%rbp), %xmm1
140b1b042: 0f 28 55 40                 	movaps	0x40(%rbp), %xmm2
140b1b046: 0f 11 56 20                 	movups	%xmm2, 0x20(%rsi)
140b1b04a: 0f 11 4e 10                 	movups	%xmm1, 0x10(%rsi)
140b1b04e: 0f 11 06                    	movups	%xmm0, (%rsi)
140b1b051: 48 81 c4 e8 00 00 00        	addq	$0xe8, %rsp
140b1b058: 5e                          	popq	%rsi
140b1b059: 5d                          	popq	%rbp
140b1b05a: c3                          	retq
140b1b05b: 48 8b 45 d8                 	movq	-0x28(%rbp), %rax
140b1b05f: 48 89 45 50                 	movq	%rax, 0x50(%rbp)
140b1b063: 0f 10 45 a8                 	movups	-0x58(%rbp), %xmm0
140b1b067: 0f 10 4d b8                 	movups	-0x48(%rbp), %xmm1
140b1b06b: 0f 10 55 c8                 	movups	-0x38(%rbp), %xmm2
140b1b06f: 0f 29 55 40                 	movaps	%xmm2, 0x40(%rbp)
140b1b073: 0f 29 4d 30                 	movaps	%xmm1, 0x30(%rbp)
140b1b077: 0f 29 45 20                 	movaps	%xmm0, 0x20(%rbp)
140b1b07b: 48 8d 05 3e 7b fb 01        	leaq	0x1fb7b3e(%rip), %rax   # 0x142ad2bc0
140b1b082: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140b1b087: 48 8d 0d 72 55 fb 01        	leaq	0x1fb5572(%rip), %rcx   # 0x142ad0600
140b1b08e: 4c 8d 0d 2b 55 fb 01        	leaq	0x1fb552b(%rip), %r9    # 0x142ad05c0
140b1b095: 4c 8d 45 20                 	leaq	0x20(%rbp), %r8
140b1b099: ba 2b 00 00 00              	movl	$0x2b, %edx
140b1b09e: e8 bd e6 db 01              	callq	0x1428d9760 <.text+0x28d8760>
140b1b0a3: 0f 0b                       	ud2
140b1b0a5: 4c 8d 05 04 57 fb 01        	leaq	0x1fb5704(%rip), %r8    # 0x142ad07b0
140b1b0ac: b9 01 00 00 00              	movl	$0x1, %ecx
140b1b0b1: ba 11 00 00 00              	movl	$0x11, %edx
140b1b0b6: e8 28 df db 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140b1b0bb: cc                          	int3
140b1b0bc: 0f 1f 40 00                 	nopl	(%rax)
140b1b0c0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140b1b0c5: 55                          	pushq	%rbp
140b1b0c6: 56                          	pushq	%rsi
140b1b0c7: 48 83 ec 28                 	subq	$0x28, %rsp
140b1b0cb: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140b1b0d2: c6 45 5f 01                 	movb	$0x1, 0x5f(%rbp)
140b1b0d6: 48 8d 4d 20                 	leaq	0x20(%rbp), %rcx
140b1b0da: e8 d1 75 f3 ff              	callq	0x140a526b0 <.text+0xa516b0>
140b1b0df: c6 45 5f 01                 	movb	$0x1, 0x5f(%rbp)
140b1b0e3: 48 83 c4 28                 	addq	$0x28, %rsp
140b1b0e7: 5e                          	popq	%rsi
140b1b0e8: 5d                          	popq	%rbp
140b1b0e9: c3                          	retq
140b1b0ea: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140b1b0f0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140b1b0f5: 55                          	pushq	%rbp
140b1b0f6: 56                          	pushq	%rsi
140b1b0f7: 48 83 ec 28                 	subq	$0x28, %rsp
140b1b0fb: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140b1b102: 48 8b 45 18                 	movq	0x18(%rbp), %rax
140b1b106: 48 8b 50 10                 	movq	0x10(%rax), %rdx
140b1b10a: 48 85 d2                    	testq	%rdx, %rdx
140b1b10d: 74 13                       	je	0x140b1b122 <.text+0xb1a122>
140b1b10f: 48 8b 45 18                 	movq	0x18(%rbp), %rax
140b1b113: 48 8b 48 18                 	movq	0x18(%rax), %rcx
140b1b117: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140b1b11d: e8 fe 8a af ff              	callq	0x140613c20 <.text+0x612c20>
140b1b122: 0f 28 45 20                 	movaps	0x20(%rbp), %xmm0
140b1b126: 0f 28 4d 30                 	movaps	0x30(%rbp), %xmm1
140b1b12a: 0f 28 55 40                 	movaps	0x40(%rbp), %xmm2
140b1b12e: 48 8b 45 18                 	movq	0x18(%rbp), %rax
140b1b132: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
140b1b136: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
140b1b13a: 0f 11 00                    	movups	%xmm0, (%rax)
140b1b13d: c6 45 5f 00                 	movb	$0x0, 0x5f(%rbp)
140b1b141: 48 83 c4 28                 	addq	$0x28, %rsp
140b1b145: 5e                          	popq	%rsi
140b1b146: 5d                          	popq	%rbp
140b1b147: c3                          	retq
140b1b148: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
140b1b150: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140b1b155: 55                          	pushq	%rbp
140b1b156: 56                          	pushq	%rsi
140b1b157: 48 83 ec 28                 	subq	$0x28, %rsp
140b1b15b: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140b1b162: 80 7d 5f 00                 	cmpb	$0x0, 0x5f(%rbp)
140b1b166: 75 07                       	jne	0x140b1b16f <.text+0xb1a16f>
140b1b168: 48 83 c4 28                 	addq	$0x28, %rsp
140b1b16c: 5e                          	popq	%rsi
140b1b16d: 5d                          	popq	%rbp
140b1b16e: c3                          	retq
140b1b16f: 48 8d 4d e0                 	leaq	-0x20(%rbp), %rcx
140b1b173: e8 08 f5 62 ff              	callq	0x14014a680 <.text+0x149680>
140b1b178: eb ee                       	jmp	0x140b1b168 <.text+0xb1a168>
140b1b17a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140b1b180: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140b1b185: 55                          	pushq	%rbp
140b1b186: 56                          	pushq	%rsi
140b1b187: 48 83 ec 28                 	subq	$0x28, %rsp
140b1b18b: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140b1b192: 48 8d 4d a8                 	leaq	-0x58(%rbp), %rcx
140b1b196: e8 95 1a 23 00              	callq	0x140d4cc30 <.text+0xd4bc30>
140b1b19b: 90                          	nop
140b1b19c: 48 83 c4 28                 	addq	$0x28, %rsp
140b1b1a0: 5e                          	popq	%rsi
140b1b1a1: 5d                          	popq	%rbp
140b1b1a2: c3                          	retq

D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140b1b1b0: 55                          	pushq	%rbp
140b1b1b1: 41 56                       	pushq	%r14
140b1b1b3: 56                          	pushq	%rsi
140b1b1b4: 57                          	pushq	%rdi
140b1b1b5: 53                          	pushq	%rbx
140b1b1b6: 48 81 ec 10 01 00 00        	subq	$0x110, %rsp            # imm = 0x110
140b1b1bd: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140b1b1c5: 48 c7 85 88 00 00 00 fe ff ff ff    	movq	$-0x2, 0x88(%rbp)
140b1b1d0: 4c 89 cf                    	movq	%r9, %rdi
140b1b1d3: 48 89 d6                    	movq	%rdx, %rsi
140b1b1d6: 49 8b 00                    	movq	(%r8), %rax
140b1b1d9: 49 8b 50 08                 	movq	0x8(%r8), %rdx
140b1b1dd: 4c 8b 00                    	movq	(%rax), %r8
140b1b1e0: 45 31 c9                    	xorl	%r9d, %r9d
140b1b1e3: 4d 89 c2                    	movq	%r8, %r10
140b1b1e6: 4c 2b 52 18                 	subq	0x18(%rdx), %r10
140b1b1ea: 4d 0f 42 d1                 	cmovbq	%r9, %r10
140b1b1ee: 4c 2b 42 38                 	subq	0x38(%rdx), %r8
140b1b1f2: 4d 0f 42 c1                 	cmovbq	%r9, %r8
140b1b1f6: 4c 8b 5a 30                 	movq	0x30(%rdx), %r11
140b1b1fa: 4d 29 c3                    	subq	%r8, %r11
140b1b1fd: 4d 0f 42 d9                 	cmovbq	%r9, %r11
140b1b201: 41 0f 96 c0                 	setbe	%r8b
140b1b205: 48 8b 5a 10                 	movq	0x10(%rdx), %rbx
140b1b209: 4c 29 d3                    	subq	%r10, %rbx
140b1b20c: 49 0f 42 d9                 	cmovbq	%r9, %rbx
140b1b210: 48 8b 52 40                 	movq	0x40(%rdx), %rdx
140b1b214: 41 0f 96 c1                 	setbe	%r9b
140b1b218: 4c 01 db                    	addq	%r11, %rbx
140b1b21b: 49 89 d2                    	movq	%rdx, %r10
140b1b21e: 49 29 da                    	subq	%rbx, %r10
140b1b221: 4c 89 10                    	movq	%r10, (%rax)
140b1b224: 45 84 c1                    	testb	%r8b, %r9b
140b1b227: 0f 85 49 01 00 00           	jne	0x140b1b376 <.text+0xb1a376>
140b1b22d: 48 89 10                    	movq	%rdx, (%rax)
140b1b230: 48 8b 01                    	movq	(%rcx), %rax
140b1b233: 48 8b 10                    	movq	(%rax), %rdx
140b1b236: 48 85 d2                    	testq	%rdx, %rdx
140b1b239: 74 5a                       	je	0x140b1b295 <.text+0xb1a295>
140b1b23b: 48 83 c2 10                 	addq	$0x10, %rdx
140b1b23f: 4c 8d 05 ca e2 fc 01        	leaq	0x1fce2ca(%rip), %r8    # 0x142ae9510
140b1b246: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
140b1b24a: 41 b9 11 00 00 00           	movl	$0x11, %r9d
140b1b250: e8 eb 91 f3 ff              	callq	0x140a54440 <.text+0xa53440>
140b1b255: 48 83 7d 10 02              	cmpq	$0x2, 0x10(%rbp)
140b1b25a: 0f 85 82 00 00 00           	jne	0x140b1b2e2 <.text+0xb1a2e2>
140b1b260: 48 8b 45 18                 	movq	0x18(%rbp), %rax
140b1b264: 48 89 45 d8                 	movq	%rax, -0x28(%rbp)
140b1b268: 48 8d 05 b9 e2 fc 01        	leaq	0x1fce2b9(%rip), %rax   # 0x142ae9528
140b1b26f: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140b1b274: 48 8d 0d 0d de fc 01        	leaq	0x1fcde0d(%rip), %rcx   # 0x142ae9088
140b1b27b: 4c 8d 0d e6 dd fc 01        	leaq	0x1fcdde6(%rip), %r9    # 0x142ae9068
140b1b282: 4c 8d 45 d8                 	leaq	-0x28(%rbp), %r8
140b1b286: ba 2b 00 00 00              	movl	$0x2b, %edx
140b1b28b: e8 d0 e4 db 01              	callq	0x1428d9760 <.text+0x28d8760>
140b1b290: e9 af 03 00 00              	jmp	0x140b1b644 <.text+0xb1a644>
140b1b295: 0f b6 05 6d 99 b1 02        	movzbl	0x2b1996d(%rip), %eax   # 0x143634c09
140b1b29c: b9 11 00 00 00              	movl	$0x11, %ecx
140b1b2a1: ba 01 00 00 00              	movl	$0x1, %edx
140b1b2a6: e8 65 89 af ff              	callq	0x140613c10 <.text+0x612c10>
140b1b2ab: 48 85 c0                    	testq	%rax, %rax
140b1b2ae: 0f 84 e8 02 00 00           	je	0x140b1b59c <.text+0xb1a59c>
140b1b2b4: 0f 10 05 55 e2 fc 01        	movups	0x1fce255(%rip), %xmm0  # 0x142ae9510
140b1b2bb: 0f 11 00                    	movups	%xmm0, (%rax)
140b1b2be: c6 40 10 67                 	movb	$0x67, 0x10(%rax)
140b1b2c2: 48 c7 45 10 00 00 00 00     	movq	$0x0, 0x10(%rbp)
140b1b2ca: 48 c7 45 20 11 00 00 00     	movq	$0x11, 0x20(%rbp)
140b1b2d2: 48 89 45 28                 	movq	%rax, 0x28(%rbp)
140b1b2d6: 48 c7 45 30 11 00 00 00     	movq	$0x11, 0x30(%rbp)
140b1b2de: c6 45 38 02                 	movb	$0x2, 0x38(%rbp)
140b1b2e2: 48 8b 45 10                 	movq	0x10(%rbp), %rax
140b1b2e6: 48 8b 4d 18                 	movq	0x18(%rbp), %rcx
140b1b2ea: 48 8b 55 20                 	movq	0x20(%rbp), %rdx
140b1b2ee: 4c 8b 45 28                 	movq	0x28(%rbp), %r8
140b1b2f2: 4c 8b 4d 30                 	movq	0x30(%rbp), %r9
140b1b2f6: 4c 89 4d 70                 	movq	%r9, 0x70(%rbp)
140b1b2fa: 44 0f b6 4d 38              	movzbl	0x38(%rbp), %r9d
140b1b2ff: 44 88 4d 78                 	movb	%r9b, 0x78(%rbp)
140b1b303: 44 8b 4d 39                 	movl	0x39(%rbp), %r9d
140b1b307: 44 89 4d 79                 	movl	%r9d, 0x79(%rbp)
140b1b30b: 44 0f b7 4d 3d              	movzwl	0x3d(%rbp), %r9d
140b1b310: 66 44 89 4d 7d              	movw	%r9w, 0x7d(%rbp)
140b1b315: 44 0f b6 4d 3f              	movzbl	0x3f(%rbp), %r9d
140b1b31a: 44 88 4d 7f                 	movb	%r9b, 0x7f(%rbp)
140b1b31e: 48 89 55 60                 	movq	%rdx, 0x60(%rbp)
140b1b322: 4c 89 45 68                 	movq	%r8, 0x68(%rbp)
140b1b326: 48 89 45 50                 	movq	%rax, 0x50(%rbp)
140b1b32a: 48 89 4d 58                 	movq	%rcx, 0x58(%rbp)
140b1b32e: 83 7d 50 01                 	cmpl	$0x1, 0x50(%rbp)
140b1b332: 75 21                       	jne	0x140b1b355 <.text+0xb1a355>
140b1b334: 48 8d 55 58                 	leaq	0x58(%rbp), %rdx
140b1b338: 48 8d 4d d8                 	leaq	-0x28(%rbp), %rcx
140b1b33c: e8 8f bf 91 00              	callq	0x1414372d0 <.text+0x14362d0>
140b1b341: 48 b8 04 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffc, %rax # imm = 0x8000000000000004
140b1b34b: 48 39 45 d8                 	cmpq	%rax, -0x28(%rbp)
140b1b34f: 0f 85 fa 01 00 00           	jne	0x140b1b54f <.text+0xb1a54f>
140b1b355: 48 8d 4d 50                 	leaq	0x50(%rbp), %rcx
140b1b359: e8 22 ef d7 ff              	callq	0x14089a280 <.text+0x899280>
140b1b35e: 48 8b 55 60                 	movq	0x60(%rbp), %rdx
140b1b362: 48 85 d2                    	testq	%rdx, %rdx
140b1b365: 74 0f                       	je	0x140b1b376 <.text+0xb1a376>
140b1b367: 48 8b 4d 68                 	movq	0x68(%rbp), %rcx
140b1b36b: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140b1b371: e8 aa 88 af ff              	callq	0x140613c20 <.text+0x612c20>
140b1b376: 48 8b 07                    	movq	(%rdi), %rax
140b1b379: 48 8b 4f 08                 	movq	0x8(%rdi), %rcx
140b1b37d: 48 8b 10                    	movq	(%rax), %rdx
140b1b380: 45 31 c0                    	xorl	%r8d, %r8d
140b1b383: 49 89 d1                    	movq	%rdx, %r9
140b1b386: 4c 2b 49 18                 	subq	0x18(%rcx), %r9
140b1b38a: 4d 0f 42 c8                 	cmovbq	%r8, %r9
140b1b38e: 48 2b 51 38                 	subq	0x38(%rcx), %rdx
140b1b392: 49 0f 42 d0                 	cmovbq	%r8, %rdx
140b1b396: 4c 8b 51 30                 	movq	0x30(%rcx), %r10
140b1b39a: 49 29 d2                    	subq	%rdx, %r10
140b1b39d: 4d 0f 42 d0                 	cmovbq	%r8, %r10
140b1b3a1: 0f 97 c2                    	seta	%dl
140b1b3a4: 4c 8b 59 10                 	movq	0x10(%rcx), %r11
140b1b3a8: 4d 29 cb                    	subq	%r9, %r11
140b1b3ab: 4d 0f 42 d8                 	cmovbq	%r8, %r11
140b1b3af: 48 8b 49 40                 	movq	0x40(%rcx), %rcx
140b1b3b3: 41 0f 97 c0                 	seta	%r8b
140b1b3b7: 4d 01 d3                    	addq	%r10, %r11
140b1b3ba: 49 89 c9                    	movq	%rcx, %r9
140b1b3bd: 4d 29 d9                    	subq	%r11, %r9
140b1b3c0: 4c 89 08                    	movq	%r9, (%rax)
140b1b3c3: 41 08 d0                    	orb	%dl, %r8b
140b1b3c6: 75 0f                       	jne	0x140b1b3d7 <.text+0xb1a3d7>
140b1b3c8: 8b 7e 1c                    	movl	0x1c(%rsi), %edi
140b1b3cb: 48 8b 1e                    	movq	(%rsi), %rbx
140b1b3ce: 4c 8b 76 10                 	movq	0x10(%rsi), %r14
140b1b3d2: e9 9d 00 00 00              	jmp	0x140b1b474 <.text+0xb1a474>
140b1b3d7: 48 89 08                    	movq	%rcx, (%rax)
140b1b3da: 8b 7e 1c                    	movl	0x1c(%rsi), %edi
140b1b3dd: 4c 8b 76 10                 	movq	0x10(%rsi), %r14
140b1b3e1: 41 89 3e                    	movl	%edi, (%r14)
140b1b3e4: 48 8b 1e                    	movq	(%rsi), %rbx
140b1b3e7: 48 8b 03                    	movq	(%rbx), %rax
140b1b3ea: 48 8b 4b 28                 	movq	0x28(%rbx), %rcx
140b1b3ee: 48 89 4d d0                 	movq	%rcx, -0x30(%rbp)
140b1b3f2: 0f 10 43 18                 	movups	0x18(%rbx), %xmm0
140b1b3f6: 0f 29 45 c0                 	movaps	%xmm0, -0x40(%rbp)
140b1b3fa: 0f 10 43 08                 	movups	0x8(%rbx), %xmm0
140b1b3fe: 0f 29 45 b0                 	movaps	%xmm0, -0x50(%rbp)
140b1b402: 48 c7 03 02 00 00 00        	movq	$0x2, (%rbx)
140b1b409: 48 83 f8 02                 	cmpq	$0x2, %rax
140b1b40d: 74 65                       	je	0x140b1b474 <.text+0xb1a474>
140b1b40f: 48 89 45 50                 	movq	%rax, 0x50(%rbp)
140b1b413: 0f 28 45 b0                 	movaps	-0x50(%rbp), %xmm0
140b1b417: 0f 28 4d c0                 	movaps	-0x40(%rbp), %xmm1
140b1b41b: 0f 11 45 58                 	movups	%xmm0, 0x58(%rbp)
140b1b41f: 0f 11 4d 68                 	movups	%xmm1, 0x68(%rbp)
140b1b423: 48 8b 4d d0                 	movq	-0x30(%rbp), %rcx
140b1b427: 48 89 4d 78                 	movq	%rcx, 0x78(%rbp)
140b1b42b: a8 01                       	testb	$0x1, %al
140b1b42d: 74 24                       	je	0x140b1b453 <.text+0xb1a453>
140b1b42f: 48 8d 55 58                 	leaq	0x58(%rbp), %rdx
140b1b433: 48 8d 4d d8                 	leaq	-0x28(%rbp), %rcx
140b1b437: 45 31 c0                    	xorl	%r8d, %r8d
140b1b43a: e8 01 c0 91 00              	callq	0x141437440 <.text+0x1436440>
140b1b43f: 48 b8 04 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffc, %rax # imm = 0x8000000000000004
140b1b449: 48 39 45 d8                 	cmpq	%rax, -0x28(%rbp)
140b1b44d: 0f 85 5f 01 00 00           	jne	0x140b1b5b2 <.text+0xb1a5b2>
140b1b453: 48 8d 4d 50                 	leaq	0x50(%rbp), %rcx
140b1b457: e8 24 ee d7 ff              	callq	0x14089a280 <.text+0x899280>
140b1b45c: 48 8b 55 60                 	movq	0x60(%rbp), %rdx
140b1b460: 48 85 d2                    	testq	%rdx, %rdx
140b1b463: 74 0f                       	je	0x140b1b474 <.text+0xb1a474>
140b1b465: 48 8b 4d 68                 	movq	0x68(%rbp), %rcx
140b1b469: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140b1b46f: e8 ac 87 af ff              	callq	0x140613c20 <.text+0x612c20>
140b1b474: 41 89 3e                    	movl	%edi, (%r14)
140b1b477: f6 43 30 01                 	testb	$0x1, 0x30(%rbx)
140b1b47b: 0f 84 c0 00 00 00           	je	0x140b1b541 <.text+0xb1a541>
140b1b481: 48 8b 85 e0 00 00 00        	movq	0xe0(%rbp), %rax
140b1b488: 48 8b 00                    	movq	(%rax), %rax
140b1b48b: f3 0f 10 40 28              	movss	0x28(%rax), %xmm0
140b1b490: f3 0f 58 43 34              	addss	0x34(%rbx), %xmm0
140b1b495: f3 0f 11 43 34              	movss	%xmm0, 0x34(%rbx)
140b1b49a: 0f 2e 05 63 a4 e0 01        	ucomiss	0x1e0a463(%rip), %xmm0  # 0x142925904
140b1b4a1: 0f 86 9a 00 00 00           	jbe	0x140b1b541 <.text+0xb1a541>
140b1b4a7: 41 89 3e                    	movl	%edi, (%r14)
140b1b4aa: 48 8b 03                    	movq	(%rbx), %rax
140b1b4ad: 48 8b 4b 28                 	movq	0x28(%rbx), %rcx
140b1b4b1: 48 89 4d d0                 	movq	%rcx, -0x30(%rbp)
140b1b4b5: 0f 10 43 18                 	movups	0x18(%rbx), %xmm0
140b1b4b9: 0f 29 45 c0                 	movaps	%xmm0, -0x40(%rbp)
140b1b4bd: 0f 10 43 08                 	movups	0x8(%rbx), %xmm0
140b1b4c1: 0f 29 45 b0                 	movaps	%xmm0, -0x50(%rbp)
140b1b4c5: 48 c7 03 02 00 00 00        	movq	$0x2, (%rbx)
140b1b4cc: 48 83 f8 02                 	cmpq	$0x2, %rax
140b1b4d0: 74 65                       	je	0x140b1b537 <.text+0xb1a537>
140b1b4d2: 48 89 45 50                 	movq	%rax, 0x50(%rbp)
140b1b4d6: 0f 28 45 b0                 	movaps	-0x50(%rbp), %xmm0
140b1b4da: 0f 28 4d c0                 	movaps	-0x40(%rbp), %xmm1
140b1b4de: 0f 11 45 58                 	movups	%xmm0, 0x58(%rbp)
140b1b4e2: 0f 11 4d 68                 	movups	%xmm1, 0x68(%rbp)
140b1b4e6: 48 8b 4d d0                 	movq	-0x30(%rbp), %rcx
140b1b4ea: 48 89 4d 78                 	movq	%rcx, 0x78(%rbp)
140b1b4ee: a8 01                       	testb	$0x1, %al
140b1b4f0: 74 24                       	je	0x140b1b516 <.text+0xb1a516>
140b1b4f2: 48 8d 55 58                 	leaq	0x58(%rbp), %rdx
140b1b4f6: 48 8d 4d d8                 	leaq	-0x28(%rbp), %rcx
140b1b4fa: 45 31 c0                    	xorl	%r8d, %r8d
140b1b4fd: e8 3e bf 91 00              	callq	0x141437440 <.text+0x1436440>
140b1b502: 48 b8 04 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffc, %rax # imm = 0x8000000000000004
140b1b50c: 48 39 45 d8                 	cmpq	%rax, -0x28(%rbp)
140b1b510: 0f 85 e6 00 00 00           	jne	0x140b1b5fc <.text+0xb1a5fc>
140b1b516: 48 8d 4d 50                 	leaq	0x50(%rbp), %rcx
140b1b51a: e8 61 ed d7 ff              	callq	0x14089a280 <.text+0x899280>
140b1b51f: 48 8b 55 60                 	movq	0x60(%rbp), %rdx
140b1b523: 48 85 d2                    	testq	%rdx, %rdx
140b1b526: 74 0f                       	je	0x140b1b537 <.text+0xb1a537>
140b1b528: 48 8b 4d 68                 	movq	0x68(%rbp), %rcx
140b1b52c: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140b1b532: e8 e9 86 af ff              	callq	0x140613c20 <.text+0x612c20>
140b1b537: 41 89 3e                    	movl	%edi, (%r14)
140b1b53a: c7 43 30 00 00 00 00        	movl	$0x0, 0x30(%rbx)
140b1b541: 48 81 c4 10 01 00 00        	addq	$0x110, %rsp            # imm = 0x110
140b1b548: 5b                          	popq	%rbx
140b1b549: 5f                          	popq	%rdi
140b1b54a: 5e                          	popq	%rsi
140b1b54b: 41 5e                       	popq	%r14
140b1b54d: 5d                          	popq	%rbp
140b1b54e: c3                          	retq
140b1b54f: 48 8b 45 08                 	movq	0x8(%rbp), %rax
140b1b553: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
140b1b557: 0f 10 45 d8                 	movups	-0x28(%rbp), %xmm0
140b1b55b: 0f 10 4d e8                 	movups	-0x18(%rbp), %xmm1
140b1b55f: 0f 10 55 f8                 	movups	-0x8(%rbp), %xmm2
140b1b563: 0f 29 55 30                 	movaps	%xmm2, 0x30(%rbp)
140b1b567: 0f 29 4d 20                 	movaps	%xmm1, 0x20(%rbp)
140b1b56b: 0f 29 45 10                 	movaps	%xmm0, 0x10(%rbp)
140b1b56f: 48 8d 05 4a 76 fb 01        	leaq	0x1fb764a(%rip), %rax   # 0x142ad2bc0
140b1b576: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140b1b57b: 48 8d 0d 7e 50 fb 01        	leaq	0x1fb507e(%rip), %rcx   # 0x142ad0600
140b1b582: 4c 8d 0d 37 50 fb 01        	leaq	0x1fb5037(%rip), %r9    # 0x142ad05c0
140b1b589: 4c 8d 45 10                 	leaq	0x10(%rbp), %r8
140b1b58d: ba 2b 00 00 00              	movl	$0x2b, %edx
140b1b592: e8 c9 e1 db 01              	callq	0x1428d9760 <.text+0x28d8760>
140b1b597: e9 a8 00 00 00              	jmp	0x140b1b644 <.text+0xb1a644>
140b1b59c: 4c 8d 05 0d 52 fb 01        	leaq	0x1fb520d(%rip), %r8    # 0x142ad07b0
140b1b5a3: b9 01 00 00 00              	movl	$0x1, %ecx
140b1b5a8: ba 11 00 00 00              	movl	$0x11, %edx
140b1b5ad: e8 31 da db 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140b1b5b2: 48 8b 45 08                 	movq	0x8(%rbp), %rax
140b1b5b6: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
140b1b5ba: 0f 10 45 d8                 	movups	-0x28(%rbp), %xmm0
140b1b5be: 0f 10 4d e8                 	movups	-0x18(%rbp), %xmm1
140b1b5c2: 0f 10 55 f8                 	movups	-0x8(%rbp), %xmm2
140b1b5c6: 0f 29 55 30                 	movaps	%xmm2, 0x30(%rbp)
140b1b5ca: 0f 29 4d 20                 	movaps	%xmm1, 0x20(%rbp)
140b1b5ce: 0f 29 45 10                 	movaps	%xmm0, 0x10(%rbp)
140b1b5d2: 48 8d 05 2f 76 fb 01        	leaq	0x1fb762f(%rip), %rax   # 0x142ad2c08
140b1b5d9: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140b1b5de: 48 8d 0d 1b 50 fb 01        	leaq	0x1fb501b(%rip), %rcx   # 0x142ad0600
140b1b5e5: 4c 8d 0d d4 4f fb 01        	leaq	0x1fb4fd4(%rip), %r9    # 0x142ad05c0
140b1b5ec: 4c 8d 45 10                 	leaq	0x10(%rbp), %r8
140b1b5f0: ba 2b 00 00 00              	movl	$0x2b, %edx
140b1b5f5: e8 66 e1 db 01              	callq	0x1428d9760 <.text+0x28d8760>
140b1b5fa: eb 48                       	jmp	0x140b1b644 <.text+0xb1a644>
140b1b5fc: 48 8b 45 08                 	movq	0x8(%rbp), %rax
140b1b600: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
140b1b604: 0f 10 45 d8                 	movups	-0x28(%rbp), %xmm0
140b1b608: 0f 10 4d e8                 	movups	-0x18(%rbp), %xmm1
140b1b60c: 0f 10 55 f8                 	movups	-0x8(%rbp), %xmm2
140b1b610: 0f 29 55 30                 	movaps	%xmm2, 0x30(%rbp)
140b1b614: 0f 29 4d 20                 	movaps	%xmm1, 0x20(%rbp)
140b1b618: 0f 29 45 10                 	movaps	%xmm0, 0x10(%rbp)
140b1b61c: 48 8d 05 e5 75 fb 01        	leaq	0x1fb75e5(%rip), %rax   # 0x142ad2c08
140b1b623: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140b1b628: 48 8d 0d d1 4f fb 01        	leaq	0x1fb4fd1(%rip), %rcx   # 0x142ad0600
140b1b62f: 4c 8d 0d 8a 4f fb 01        	leaq	0x1fb4f8a(%rip), %r9    # 0x142ad05c0
140b1b636: 4c 8d 45 10                 	leaq	0x10(%rbp), %r8
140b1b63a: ba 2b 00 00 00              	movl	$0x2b, %edx
140b1b63f: e8 1c e1 db 01              	callq	0x1428d9760 <.text+0x28d8760>
140b1b644: 0f 0b                       	ud2
140b1b646: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
140b1b650: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140b1b655: 55                          	pushq	%rbp
140b1b656: 41 56                       	pushq	%r14
140b1b658: 56                          	pushq	%rsi
140b1b659: 57                          	pushq	%rdi
140b1b65a: 53                          	pushq	%rbx
140b1b65b: 48 83 ec 30                 	subq	$0x30, %rsp
140b1b65f: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140b1b666: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
140b1b66a: e8 41 70 f3 ff              	callq	0x140a526b0 <.text+0xa516b0>
140b1b66f: 90                          	nop
140b1b670: 48 83 c4 30                 	addq	$0x30, %rsp
140b1b674: 5b                          	popq	%rbx
140b1b675: 5f                          	popq	%rdi
140b1b676: 5e                          	popq	%rsi
140b1b677: 41 5e                       	popq	%r14
140b1b679: 5d                          	popq	%rbp
140b1b67a: c3                          	retq
140b1b67b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
140b1b680: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140b1b685: 55                          	pushq	%rbp
140b1b686: 41 56                       	pushq	%r14
140b1b688: 56                          	pushq	%rsi
140b1b689: 57                          	pushq	%rdi
140b1b68a: 53                          	pushq	%rbx
140b1b68b: 48 83 ec 30                 	subq	$0x30, %rsp
140b1b68f: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140b1b696: 48 8d 4d 50                 	leaq	0x50(%rbp), %rcx
140b1b69a: e8 e1 ef 62 ff              	callq	0x14014a680 <.text+0x149680>
140b1b69f: 90                          	nop
140b1b6a0: 48 83 c4 30                 	addq	$0x30, %rsp
140b1b6a4: 5b                          	popq	%rbx
140b1b6a5: 5f                          	popq	%rdi
140b1b6a6: 5e                          	popq	%rsi
140b1b6a7: 41 5e                       	popq	%r14
140b1b6a9: 5d                          	popq	%rbp
140b1b6aa: c3                          	retq
140b1b6ab: 0f 1f 44 00 00              	nopl	(%rax,%rax)
140b1b6b0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140b1b6b5: 55                          	pushq	%rbp
140b1b6b6: 41 56                       	pushq	%r14
140b1b6b8: 56                          	pushq	%rsi
140b1b6b9: 57                          	pushq	%rdi
140b1b6ba: 53                          	pushq	%rbx
140b1b6bb: 48 83 ec 30                 	subq	$0x30, %rsp
140b1b6bf: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140b1b6c6: 48 8b 55 60                 	movq	0x60(%rbp), %rdx
140b1b6ca: 48 85 d2                    	testq	%rdx, %rdx
140b1b6cd: 74 0f                       	je	0x140b1b6de <.text+0xb1a6de>
140b1b6cf: 48 8b 4d 68                 	movq	0x68(%rbp), %rcx
140b1b6d3: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140b1b6d9: e8 42 85 af ff              	callq	0x140613c20 <.text+0x612c20>
140b1b6de: 90                          	nop
140b1b6df: 48 83 c4 30                 	addq	$0x30, %rsp
140b1b6e3: 5b                          	popq	%rbx
140b1b6e4: 5f                          	popq	%rdi
140b1b6e5: 5e                          	popq	%rsi
140b1b6e6: 41 5e                       	popq	%r14
140b1b6e8: 5d                          	popq	%rbp
140b1b6e9: c3                          	retq
140b1b6ea: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140b1b6f0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140b1b6f5: 55                          	pushq	%rbp
140b1b6f6: 41 56                       	pushq	%r14
140b1b6f8: 56                          	pushq	%rsi
140b1b6f9: 57                          	pushq	%rdi
140b1b6fa: 53                          	pushq	%rbx
140b1b6fb: 48 83 ec 30                 	subq	$0x30, %rsp
140b1b6ff: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140b1b706: 48 8d 4d 50                 	leaq	0x50(%rbp), %rcx
140b1b70a: e8 71 ef 62 ff              	callq	0x14014a680 <.text+0x149680>
140b1b70f: 90                          	nop
140b1b710: 48 83 c4 30                 	addq	$0x30, %rsp
140b1b714: 5b                          	popq	%rbx
140b1b715: 5f                          	popq	%rdi
140b1b716: 5e                          	popq	%rsi
140b1b717: 41 5e                       	popq	%r14
140b1b719: 5d                          	popq	%rbp
140b1b71a: c3                          	retq
140b1b71b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
140b1b720: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140b1b725: 55                          	pushq	%rbp
140b1b726: 41 56                       	pushq	%r14
140b1b728: 56                          	pushq	%rsi
140b1b729: 57                          	pushq	%rdi
140b1b72a: 53                          	pushq	%rbx
140b1b72b: 48 83 ec 30                 	subq	$0x30, %rsp
140b1b72f: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140b1b736: 48 8b 55 60                 	movq	0x60(%rbp), %rdx
140b1b73a: 48 85 d2                    	testq	%rdx, %rdx
140b1b73d: 74 0f                       	je	0x140b1b74e <.text+0xb1a74e>
140b1b73f: 48 8b 4d 68                 	movq	0x68(%rbp), %rcx
140b1b743: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140b1b749: e8 d2 84 af ff              	callq	0x140613c20 <.text+0x612c20>
140b1b74e: 90                          	nop
140b1b74f: 48 83 c4 30                 	addq	$0x30, %rsp
140b1b753: 5b                          	popq	%rbx
140b1b754: 5f                          	popq	%rdi
140b1b755: 5e                          	popq	%rsi
140b1b756: 41 5e                       	popq	%r14
140b1b758: 5d                          	popq	%rbp
140b1b759: c3                          	retq
140b1b75a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140b1b760: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140b1b765: 55                          	pushq	%rbp
140b1b766: 41 56                       	pushq	%r14
140b1b768: 56                          	pushq	%rsi
140b1b769: 57                          	pushq	%rdi
140b1b76a: 53                          	pushq	%rbx
140b1b76b: 48 83 ec 30                 	subq	$0x30, %rsp
140b1b76f: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140b1b776: 48 8d 4d 50                 	leaq	0x50(%rbp), %rcx
140b1b77a: e8 01 ef 62 ff              	callq	0x14014a680 <.text+0x149680>
140b1b77f: 90                          	nop
140b1b780: 48 83 c4 30                 	addq	$0x30, %rsp
140b1b784: 5b                          	popq	%rbx
140b1b785: 5f                          	popq	%rdi
140b1b786: 5e                          	popq	%rsi
140b1b787: 41 5e                       	popq	%r14
140b1b789: 5d                          	popq	%rbp
140b1b78a: c3                          	retq
140b1b78b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
140b1b790: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140b1b795: 55                          	pushq	%rbp
140b1b796: 41 56                       	pushq	%r14
140b1b798: 56                          	pushq	%rsi
140b1b799: 57                          	pushq	%rdi
140b1b79a: 53                          	pushq	%rbx
140b1b79b: 48 83 ec 30                 	subq	$0x30, %rsp
140b1b79f: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140b1b7a6: 48 8b 55 60                 	movq	0x60(%rbp), %rdx
140b1b7aa: 48 85 d2                    	testq	%rdx, %rdx
140b1b7ad: 74 0f                       	je	0x140b1b7be <.text+0xb1a7be>
140b1b7af: 48 8b 4d 68                 	movq	0x68(%rbp), %rcx
140b1b7b3: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140b1b7b9: e8 62 84 af ff              	callq	0x140613c20 <.text+0x612c20>
140b1b7be: 90                          	nop
140b1b7bf: 48 83 c4 30                 	addq	$0x30, %rsp
140b1b7c3: 5b                          	popq	%rbx
140b1b7c4: 5f                          	popq	%rdi
140b1b7c5: 5e                          	popq	%rsi
140b1b7c6: 41 5e                       	popq	%r14
140b1b7c8: 5d                          	popq	%rbp
140b1b7c9: c3                          	retq
140b1b7ca: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140b1b7d0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140b1b7d5: 55                          	pushq	%rbp
140b1b7d6: 41 56                       	pushq	%r14
140b1b7d8: 56                          	pushq	%rsi
140b1b7d9: 57                          	pushq	%rdi
140b1b7da: 53                          	pushq	%rbx
140b1b7db: 48 83 ec 30                 	subq	$0x30, %rsp
140b1b7df: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140b1b7e6: 48 8d 4d d8                 	leaq	-0x28(%rbp), %rcx
140b1b7ea: e8 41 14 23 00              	callq	0x140d4cc30 <.text+0xd4bc30>
140b1b7ef: 90                          	nop
140b1b7f0: 48 83 c4 30                 	addq	$0x30, %rsp
140b1b7f4: 5b                          	popq	%rbx
140b1b7f5: 5f                          	popq	%rdi
140b1b7f6: 5e                          	popq	%rsi
140b1b7f7: 41 5e                       	popq	%r14
140b1b7f9: 5d                          	popq	%rbp
140b1b7fa: c3                          	retq
140b1b7fb: 0f 1f 44 00 00              	nopl	(%rax,%rax)
140b1b800: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140b1b805: 55                          	pushq	%rbp
140b1b806: 41 56                       	pushq	%r14
140b1b808: 56                          	pushq	%rsi
140b1b809: 57                          	pushq	%rdi
140b1b80a: 53                          	pushq	%rbx
140b1b80b: 48 83 ec 30                 	subq	$0x30, %rsp
140b1b80f: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140b1b816: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
140b1b81a: e8 91 6e f3 ff              	callq	0x140a526b0 <.text+0xa516b0>
140b1b81f: 90                          	nop
140b1b820: 48 83 c4 30                 	addq	$0x30, %rsp
140b1b824: 5b                          	popq	%rbx
140b1b825: 5f                          	popq	%rdi
140b1b826: 5e                          	popq	%rsi
140b1b827: 41 5e                       	popq	%r14
140b1b829: 5d                          	popq	%rbp
140b1b82a: c3                          	retq
140b1b82b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
140b1b830: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140b1b835: 55                          	pushq	%rbp
140b1b836: 41 56                       	pushq	%r14
140b1b838: 56                          	pushq	%rsi
140b1b839: 57                          	pushq	%rdi
140b1b83a: 53                          	pushq	%rbx
140b1b83b: 48 83 ec 30                 	subq	$0x30, %rsp
140b1b83f: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140b1b846: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
140b1b84a: e8 61 6e f3 ff              	callq	0x140a526b0 <.text+0xa516b0>
140b1b84f: 90                          	nop
140b1b850: 48 83 c4 30                 	addq	$0x30, %rsp
140b1b854: 5b                          	popq	%rbx
140b1b855: 5f                          	popq	%rdi
140b1b856: 5e                          	popq	%rsi
140b1b857: 41 5e                       	popq	%r14
140b1b859: 5d                          	popq	%rbp
140b1b85a: c3                          	retq

D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140b61d30: 41 57                       	pushq	%r15
140b61d32: 41 56                       	pushq	%r14
140b61d34: 41 54                       	pushq	%r12
140b61d36: 56                          	pushq	%rsi
140b61d37: 57                          	pushq	%rdi
140b61d38: 53                          	pushq	%rbx
140b61d39: 48 81 ec 08 01 00 00        	subq	$0x108, %rsp            # imm = 0x108
140b61d40: bf 01 00 00 00              	movl	$0x1, %edi
140b61d45: f0                          	lock
140b61d46: 0f c1 ba b0 05 00 00        	xaddl	%edi, 0x5b0(%rdx)
140b61d4d: f6 01 01                    	testb	$0x1, (%rcx)
140b61d50: 0f 84 ea 03 00 00           	je	0x140b62140 <.text+0xb61140>
140b61d56: 48 89 ce                    	movq	%rcx, %rsi
140b61d59: 48 8b 49 28                 	movq	0x28(%rcx), %rcx
140b61d5d: 48 8b 82 18 02 00 00        	movq	0x218(%rdx), %rax
140b61d64: 48 39 c8                    	cmpq	%rcx, %rax
140b61d67: 0f 86 6c 02 00 00           	jbe	0x140b61fd9 <.text+0xb60fd9>
140b61d6d: 4c 8b 82 10 02 00 00        	movq	0x210(%rdx), %r8
140b61d74: 49 8b 0c c8                 	movq	(%r8,%rcx,8), %rcx
140b61d78: 48 85 c9                    	testq	%rcx, %rcx
140b61d7b: 0f 84 58 02 00 00           	je	0x140b61fd9 <.text+0xb60fd9>
140b61d81: 4c 8b 8a e0 01 00 00        	movq	0x1e0(%rdx), %r9
140b61d88: 48 f7 d1                    	notq	%rcx
140b61d8b: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
140b61d8f: 48 c1 e1 05                 	shlq	$0x5, %rcx
140b61d93: 49 83 7c 09 20 00           	cmpq	$0x0, 0x20(%r9,%rcx)
140b61d99: 0f 84 3a 02 00 00           	je	0x140b61fd9 <.text+0xb60fd9>
140b61d9f: 4c 8b 56 30                 	movq	0x30(%rsi), %r10
140b61da3: 4c 39 d0                    	cmpq	%r10, %rax
140b61da6: 0f 86 4e 02 00 00           	jbe	0x140b61ffa <.text+0xb60ffa>
140b61dac: 4b 8b 04 d0                 	movq	(%r8,%r10,8), %rax
140b61db0: 48 85 c0                    	testq	%rax, %rax
140b61db3: 0f 84 41 02 00 00           	je	0x140b61ffa <.text+0xb60ffa>
140b61db9: 48 f7 d0                    	notq	%rax
140b61dbc: 4c 8d 04 40                 	leaq	(%rax,%rax,2), %r8
140b61dc0: 49 c1 e0 05                 	shlq	$0x5, %r8
140b61dc4: 4b 83 7c 01 20 00           	cmpq	$0x0, 0x20(%r9,%r8)
140b61dca: 0f 84 2a 02 00 00           	je	0x140b61ffa <.text+0xb60ffa>
140b61dd0: 4c 01 c9                    	addq	%r9, %rcx
140b61dd3: 4c 8b 51 10                 	movq	0x10(%rcx), %r10
140b61dd7: 8b 86 78 01 00 00           	movl	0x178(%rsi), %eax
140b61ddd: 4d 01 c8                    	addq	%r9, %r8
140b61de0: 4d 8b 48 10                 	movq	0x10(%r8), %r9
140b61de4: 44 8b 9a b0 05 00 00        	movl	0x5b0(%rdx), %r11d
140b61deb: 4c 8b 5e 10                 	movq	0x10(%rsi), %r11
140b61def: 48 8b 9a 18 02 00 00        	movq	0x218(%rdx), %rbx
140b61df6: 4c 39 db                    	cmpq	%r11, %rbx
140b61df9: 0f 86 80 02 00 00           	jbe	0x140b6207f <.text+0xb6107f>
140b61dff: 4c 8b ba 10 02 00 00        	movq	0x210(%rdx), %r15
140b61e06: 4f 8b 1c df                 	movq	(%r15,%r11,8), %r11
140b61e0a: 4d 85 db                    	testq	%r11, %r11
140b61e0d: 0f 84 6c 02 00 00           	je	0x140b6207f <.text+0xb6107f>
140b61e13: 4c 8b b2 e0 01 00 00        	movq	0x1e0(%rdx), %r14
140b61e1a: 49 f7 d3                    	notq	%r11
140b61e1d: 4b 8d 14 5b                 	leaq	(%r11,%r11,2), %rdx
140b61e21: 48 c1 e2 05                 	shlq	$0x5, %rdx
140b61e25: 49 83 7c 16 20 00           	cmpq	$0x0, 0x20(%r14,%rdx)
140b61e2b: 0f 84 4e 02 00 00           	je	0x140b6207f <.text+0xb6107f>
140b61e31: 4c 8b 5e 20                 	movq	0x20(%rsi), %r11
140b61e35: 4c 39 db                    	cmpq	%r11, %rbx
140b61e38: 0f 86 5f 02 00 00           	jbe	0x140b6209d <.text+0xb6109d>
140b61e3e: 4f 8b 1c df                 	movq	(%r15,%r11,8), %r11
140b61e42: 4d 85 db                    	testq	%r11, %r11
140b61e45: 0f 84 52 02 00 00           	je	0x140b6209d <.text+0xb6109d>
140b61e4b: 49 f7 d3                    	notq	%r11
140b61e4e: 4f 8d 1c 5b                 	leaq	(%r11,%r11,2), %r11
140b61e52: 49 c1 e3 05                 	shlq	$0x5, %r11
140b61e56: 4b 83 7c 1e 20 00           	cmpq	$0x0, 0x20(%r14,%r11)
140b61e5c: 0f 84 3b 02 00 00           	je	0x140b6209d <.text+0xb6109d>
140b61e62: 4c 8b 66 38                 	movq	0x38(%rsi), %r12
140b61e66: 4c 39 e3                    	cmpq	%r12, %rbx
140b61e69: 0f 86 4c 02 00 00           	jbe	0x140b620bb <.text+0xb610bb>
140b61e6f: 4b 8b 1c e7                 	movq	(%r15,%r12,8), %rbx
140b61e73: 48 85 db                    	testq	%rbx, %rbx
140b61e76: 0f 84 3f 02 00 00           	je	0x140b620bb <.text+0xb610bb>
140b61e7c: 48 f7 d3                    	notq	%rbx
140b61e7f: 48 8d 1c 5b                 	leaq	(%rbx,%rbx,2), %rbx
140b61e83: 48 c1 e3 05                 	shlq	$0x5, %rbx
140b61e87: 49 83 7c 1e 20 00           	cmpq	$0x0, 0x20(%r14,%rbx)
140b61e8d: 0f 84 28 02 00 00           	je	0x140b620bb <.text+0xb610bb>
140b61e93: 4c 01 f2                    	addq	%r14, %rdx
140b61e96: 4c 8b 62 10                 	movq	0x10(%rdx), %r12
140b61e9a: 4d 01 f3                    	addq	%r14, %r11
140b61e9d: 4d 8b 7b 10                 	movq	0x10(%r11), %r15
140b61ea1: 4c 01 f3                    	addq	%r14, %rbx
140b61ea4: 4c 8b 73 10                 	movq	0x10(%rbx), %r14
140b61ea8: 4c 89 54 24 58              	movq	%r10, 0x58(%rsp)
140b61ead: 4c 8d 51 5c                 	leaq	0x5c(%rcx), %r10
140b61eb1: 48 83 c1 58                 	addq	$0x58, %rcx
140b61eb5: 48 89 4c 24 60              	movq	%rcx, 0x60(%rsp)
140b61eba: 4c 89 54 24 68              	movq	%r10, 0x68(%rsp)
140b61ebf: 89 44 24 70                 	movl	%eax, 0x70(%rsp)
140b61ec3: 89 7c 24 74                 	movl	%edi, 0x74(%rsp)
140b61ec7: 4c 89 4c 24 78              	movq	%r9, 0x78(%rsp)
140b61ecc: 49 8d 48 5c                 	leaq	0x5c(%r8), %rcx
140b61ed0: 49 83 c0 58                 	addq	$0x58, %r8
140b61ed4: 4c 89 84 24 80 00 00 00     	movq	%r8, 0x80(%rsp)
140b61edc: 48 89 8c 24 88 00 00 00     	movq	%rcx, 0x88(%rsp)
140b61ee4: 48 8d 4e 08                 	leaq	0x8(%rsi), %rcx
140b61ee8: 89 84 24 90 00 00 00        	movl	%eax, 0x90(%rsp)
140b61eef: 89 bc 24 94 00 00 00        	movl	%edi, 0x94(%rsp)
140b61ef6: 48 89 8c 24 98 00 00 00     	movq	%rcx, 0x98(%rsp)
140b61efe: 4c 89 a4 24 a0 00 00 00     	movq	%r12, 0xa0(%rsp)
140b61f06: 48 8d 4a 5c                 	leaq	0x5c(%rdx), %rcx
140b61f0a: 48 83 c2 58                 	addq	$0x58, %rdx
140b61f0e: 48 89 94 24 a8 00 00 00     	movq	%rdx, 0xa8(%rsp)
140b61f16: 48 89 8c 24 b0 00 00 00     	movq	%rcx, 0xb0(%rsp)
140b61f1e: 48 8d 4e 18                 	leaq	0x18(%rsi), %rcx
140b61f22: 89 84 24 b8 00 00 00        	movl	%eax, 0xb8(%rsp)
140b61f29: 89 bc 24 bc 00 00 00        	movl	%edi, 0xbc(%rsp)
140b61f30: 48 89 8c 24 c0 00 00 00     	movq	%rcx, 0xc0(%rsp)
140b61f38: 4c 89 bc 24 c8 00 00 00     	movq	%r15, 0xc8(%rsp)
140b61f40: 49 8d 4b 5c                 	leaq	0x5c(%r11), %rcx
140b61f44: 49 83 c3 58                 	addq	$0x58, %r11
140b61f48: 4c 89 9c 24 d0 00 00 00     	movq	%r11, 0xd0(%rsp)
140b61f50: 48 89 8c 24 d8 00 00 00     	movq	%rcx, 0xd8(%rsp)
140b61f58: 48 8d 4b 58                 	leaq	0x58(%rbx), %rcx
140b61f5c: 48 83 c3 5c                 	addq	$0x5c, %rbx
140b61f60: 89 84 24 e0 00 00 00        	movl	%eax, 0xe0(%rsp)
140b61f67: 89 bc 24 e4 00 00 00        	movl	%edi, 0xe4(%rsp)
140b61f6e: 4c 89 b4 24 e8 00 00 00     	movq	%r14, 0xe8(%rsp)
140b61f76: 48 8d 54 24 78              	leaq	0x78(%rsp), %rdx
140b61f7b: 48 89 8c 24 f0 00 00 00     	movq	%rcx, 0xf0(%rsp)
140b61f83: 4c 8d 84 24 98 00 00 00     	leaq	0x98(%rsp), %r8
140b61f8b: 48 89 9c 24 f8 00 00 00     	movq	%rbx, 0xf8(%rsp)
140b61f93: 4c 8d 8c 24 c0 00 00 00     	leaq	0xc0(%rsp), %r9
140b61f9b: 89 84 24 00 01 00 00        	movl	%eax, 0x100(%rsp)
140b61fa2: 48 8d 84 24 e8 00 00 00     	leaq	0xe8(%rsp), %rax
140b61faa: 89 bc 24 04 01 00 00        	movl	%edi, 0x104(%rsp)
140b61fb1: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140b61fb6: 48 8d 4c 24 58              	leaq	0x58(%rsp), %rcx
140b61fbb: e8 f0 91 fb ff              	callq	0x140b1b1b0 <.text+0xb1a1b0>
140b61fc0: 89 be 78 01 00 00           	movl	%edi, 0x178(%rsi)
140b61fc6: 31 c0                       	xorl	%eax, %eax
140b61fc8: 48 81 c4 08 01 00 00        	addq	$0x108, %rsp            # imm = 0x108
140b61fcf: 5b                          	popq	%rbx
140b61fd0: 5f                          	popq	%rdi
140b61fd1: 5e                          	popq	%rsi
140b61fd2: 41 5c                       	popq	%r12
140b61fd4: 41 5e                       	popq	%r14
140b61fd6: 41 5f                       	popq	%r15
140b61fd8: c3                          	retq
140b61fd9: 48 81 c6 e0 00 00 00        	addq	$0xe0, %rsi
140b61fe0: 48 8d 05 3d 02 f9 01        	leaq	0x1f9023d(%rip), %rax   # 0x142af2224
140b61fe7: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
140b61fec: 48 c7 44 24 30 33 00 00 00  	movq	$0x33, 0x30(%rsp)
140b61ff5: e9 dd 00 00 00              	jmp	0x140b620d7 <.text+0xb610d7>
140b61ffa: 48 81 c6 e0 00 00 00        	addq	$0xe0, %rsi
140b62001: 48 8d 05 da 03 f9 01        	leaq	0x1f903da(%rip), %rax   # 0x142af23e2
140b62008: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
140b6200d: 48 c7 44 24 30 47 00 00 00  	movq	$0x47, 0x30(%rsp)
140b62016: 48 89 74 24 38              	movq	%rsi, 0x38(%rsp)
140b6201b: 48 8d 05 ae 9a 4d ff        	leaq	-0xb26552(%rip), %rax   # 0x14003bad0 <.text+0x3aad0>
140b62022: 48 89 44 24 40              	movq	%rax, 0x40(%rsp)
140b62027: 48 8d 44 24 28              	leaq	0x28(%rsp), %rax
140b6202c: 48 89 44 24 48              	movq	%rax, 0x48(%rsp)
140b62031: 48 8d 05 58 7e 5a ff        	leaq	-0xa581a8(%rip), %rax   # 0x140109e90 <.text+0x108e90>
140b62038: 48 89 44 24 50              	movq	%rax, 0x50(%rsp)
140b6203d: 48 8d 05 14 ff f8 01        	leaq	0x1f8ff14(%rip), %rax   # 0x142af1f58
140b62044: 48 89 44 24 58              	movq	%rax, 0x58(%rsp)
140b62049: 48 c7 44 24 60 02 00 00 00  	movq	$0x2, 0x60(%rsp)
140b62052: 48 c7 44 24 78 00 00 00 00  	movq	$0x0, 0x78(%rsp)
140b6205b: 48 8d 44 24 38              	leaq	0x38(%rsp), %rax
140b62060: 48 89 44 24 68              	movq	%rax, 0x68(%rsp)
140b62065: 48 c7 44 24 70 02 00 00 00  	movq	$0x2, 0x70(%rsp)
140b6206e: 48 8d 15 fb 02 f9 01        	leaq	0x1f902fb(%rip), %rdx   # 0x142af2370
140b62075: 48 8d 4c 24 58              	leaq	0x58(%rsp), %rcx
140b6207a: e8 11 73 d7 01              	callq	0x1428d9390 <.text+0x28d8390>
140b6207f: 48 81 c6 e0 00 00 00        	addq	$0xe0, %rsi
140b62086: 48 8d 05 7e 00 f9 01        	leaq	0x1f9007e(%rip), %rax   # 0x142af210b
140b6208d: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
140b62092: 48 c7 44 24 30 62 00 00 00  	movq	$0x62, 0x30(%rsp)
140b6209b: eb 3a                       	jmp	0x140b620d7 <.text+0xb610d7>
140b6209d: 48 81 c6 e0 00 00 00        	addq	$0xe0, %rsi
140b620a4: 48 8d 05 b8 ff f8 01        	leaq	0x1f8ffb8(%rip), %rax   # 0x142af2063
140b620ab: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
140b620b0: 48 c7 44 24 30 63 00 00 00  	movq	$0x63, 0x30(%rsp)
140b620b9: eb 1c                       	jmp	0x140b620d7 <.text+0xb610d7>
140b620bb: 48 81 c6 e0 00 00 00        	addq	$0xe0, %rsi
140b620c2: 48 8d 05 2f ff f8 01        	leaq	0x1f8ff2f(%rip), %rax   # 0x142af1ff8
140b620c9: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
140b620ce: 48 c7 44 24 30 27 00 00 00  	movq	$0x27, 0x30(%rsp)
140b620d7: 48 89 74 24 38              	movq	%rsi, 0x38(%rsp)
140b620dc: 48 8d 05 ed 99 4d ff        	leaq	-0xb26613(%rip), %rax   # 0x14003bad0 <.text+0x3aad0>
140b620e3: 48 89 44 24 40              	movq	%rax, 0x40(%rsp)
140b620e8: 48 8d 44 24 28              	leaq	0x28(%rsp), %rax
140b620ed: 48 89 44 24 48              	movq	%rax, 0x48(%rsp)
140b620f2: 48 8d 05 97 7d 5a ff        	leaq	-0xa58269(%rip), %rax   # 0x140109e90 <.text+0x108e90>
140b620f9: 48 89 44 24 50              	movq	%rax, 0x50(%rsp)
140b620fe: 48 8d 05 53 fe f8 01        	leaq	0x1f8fe53(%rip), %rax   # 0x142af1f58
140b62105: 48 89 44 24 58              	movq	%rax, 0x58(%rsp)
140b6210a: 48 c7 44 24 60 02 00 00 00  	movq	$0x2, 0x60(%rsp)
140b62113: 48 c7 44 24 78 00 00 00 00  	movq	$0x0, 0x78(%rsp)
140b6211c: 48 8d 44 24 38              	leaq	0x38(%rsp), %rax
140b62121: 48 89 44 24 68              	movq	%rax, 0x68(%rsp)
140b62126: 48 c7 44 24 70 02 00 00 00  	movq	$0x2, 0x70(%rsp)
140b6212f: 48 8d 15 aa fe f8 01        	leaq	0x1f8feaa(%rip), %rdx   # 0x142af1fe0
140b62136: 48 8d 4c 24 58              	leaq	0x58(%rsp), %rcx
140b6213b: e8 50 72 d7 01              	callq	0x1428d9390 <.text+0x28d8390>
140b62140: 48 8d 0d c1 03 f9 01        	leaq	0x1f903c1(%rip), %rcx   # 0x142af2508
140b62147: 4c 8d 05 1a 04 f9 01        	leaq	0x1f9041a(%rip), %r8    # 0x142af2568
140b6214e: ba 59 00 00 00              	movl	$0x59, %edx
140b62153: e8 d8 71 d7 01              	callq	0x1428d9330 <.text+0x28d8330>
140b62158: cc                          	int3

D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141bf16c0: 55                          	pushq	%rbp
141bf16c1: 41 57                       	pushq	%r15
141bf16c3: 41 56                       	pushq	%r14
141bf16c5: 41 55                       	pushq	%r13
141bf16c7: 41 54                       	pushq	%r12
141bf16c9: 56                          	pushq	%rsi
141bf16ca: 57                          	pushq	%rdi
141bf16cb: 53                          	pushq	%rbx
141bf16cc: 48 81 ec 88 02 00 00        	subq	$0x288, %rsp            # imm = 0x288
141bf16d3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
141bf16db: 44 0f 29 bd f0 01 00 00     	movaps	%xmm15, 0x1f0(%rbp)
141bf16e3: 44 0f 29 b5 e0 01 00 00     	movaps	%xmm14, 0x1e0(%rbp)
141bf16eb: 44 0f 29 ad d0 01 00 00     	movaps	%xmm13, 0x1d0(%rbp)
141bf16f3: 44 0f 29 a5 c0 01 00 00     	movaps	%xmm12, 0x1c0(%rbp)
141bf16fb: 44 0f 29 9d b0 01 00 00     	movaps	%xmm11, 0x1b0(%rbp)
141bf1703: 44 0f 29 95 a0 01 00 00     	movaps	%xmm10, 0x1a0(%rbp)
141bf170b: 44 0f 29 8d 90 01 00 00     	movaps	%xmm9, 0x190(%rbp)
141bf1713: 44 0f 29 85 80 01 00 00     	movaps	%xmm8, 0x180(%rbp)
141bf171b: 0f 29 bd 70 01 00 00        	movaps	%xmm7, 0x170(%rbp)
141bf1722: 0f 29 b5 60 01 00 00        	movaps	%xmm6, 0x160(%rbp)
141bf1729: 48 c7 85 58 01 00 00 fe ff ff ff    	movq	$-0x2, 0x158(%rbp)
141bf1734: 4c 89 8d 30 01 00 00        	movq	%r9, 0x130(%rbp)
141bf173b: 4c 89 85 40 01 00 00        	movq	%r8, 0x140(%rbp)
141bf1742: 48 89 d7                    	movq	%rdx, %rdi
141bf1745: e8 b6 52 02 ff              	callq	0x140c16a00 <.text+0xc15a00>
141bf174a: 88 85 57 01 00 00           	movb	%al, 0x157(%rbp)
141bf1750: 84 c0                       	testb	%al, %al
141bf1752: 74 5c                       	je	0x141bf17b0 <.text+0x1bf07b0>
141bf1754: bb 30 00 00 00              	movl	$0x30, %ebx
141bf1759: b9 30 00 00 00              	movl	$0x30, %ecx
141bf175e: e8 2d fb ff ff              	callq	0x141bf1290 <.text+0x1bf0290>
141bf1763: 4c 8d 35 fe be 30 01        	leaq	0x130befe(%rip), %r14   # 0x142efd668
141bf176a: 48 83 f8 01                 	cmpq	$0x1, %rax
141bf176e: 0f 85 a7 00 00 00           	jne	0x141bf181b <.text+0x1bf081b>
141bf1774: 49 89 d1                    	movq	%rdx, %r9
141bf1777: 48 85 d2                    	testq	%rdx, %rdx
141bf177a: 74 3d                       	je	0x141bf17b9 <.text+0x1bf07b9>
141bf177c: 49 83 f9 30                 	cmpq	$0x30, %r9
141bf1780: 73 35                       	jae	0x141bf17b7 <.text+0x1bf07b7>
141bf1782: 48 8d 05 df be 30 01        	leaq	0x130bedf(%rip), %rax   # 0x142efd668
141bf1789: 41 80 3c 01 bf              	cmpb	$-0x41, (%r9,%rax)
141bf178e: 7f 29                       	jg	0x141bf17b9 <.text+0x1bf07b9>
141bf1790: 48 8d 05 89 bf 30 01        	leaq	0x130bf89(%rip), %rax   # 0x142efd720
141bf1797: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
141bf179c: 48 8d 0d c5 be 30 01        	leaq	0x130bec5(%rip), %rcx   # 0x142efd668
141bf17a3: ba 30 00 00 00              	movl	$0x30, %edx
141bf17a8: 45 31 c0                    	xorl	%r8d, %r8d
141bf17ab: e8 90 81 ce 00              	callq	0x1428d9940 <.text+0x28d8940>
141bf17b0: 31 c0                       	xorl	%eax, %eax
141bf17b2: e9 c1 01 00 00              	jmp	0x141bf1978 <.text+0x1bf0978>
141bf17b7: 75 d7                       	jne	0x141bf1790 <.text+0x1bf0790>
141bf17b9: 4c 89 c9                    	movq	%r9, %rcx
141bf17bc: e8 cf fa ff ff              	callq	0x141bf1290 <.text+0x1bf0290>
141bf17c1: 48 83 f8 01                 	cmpq	$0x1, %rax
141bf17c5: 75 54                       	jne	0x141bf181b <.text+0x1bf081b>
141bf17c7: 49 89 d0                    	movq	%rdx, %r8
141bf17ca: 49 83 c0 02                 	addq	$0x2, %r8
141bf17ce: 74 39                       	je	0x141bf1809 <.text+0x1bf0809>
141bf17d0: 49 83 f8 30                 	cmpq	$0x30, %r8
141bf17d4: 73 31                       	jae	0x141bf1807 <.text+0x1bf0807>
141bf17d6: 48 8d 05 8b be 30 01        	leaq	0x130be8b(%rip), %rax   # 0x142efd668
141bf17dd: 41 80 3c 00 bf              	cmpb	$-0x41, (%r8,%rax)
141bf17e2: 7f 25                       	jg	0x141bf1809 <.text+0x1bf0809>
141bf17e4: 48 8d 05 4d bf 30 01        	leaq	0x130bf4d(%rip), %rax   # 0x142efd738
141bf17eb: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
141bf17f0: 48 8d 0d 71 be 30 01        	leaq	0x130be71(%rip), %rcx   # 0x142efd668
141bf17f7: ba 30 00 00 00              	movl	$0x30, %edx
141bf17fc: 41 b9 30 00 00 00           	movl	$0x30, %r9d
141bf1802: e8 39 81 ce 00              	callq	0x1428d9940 <.text+0x28d8940>
141bf1807: 75 db                       	jne	0x141bf17e4 <.text+0x1bf07e4>
141bf1809: bb 2e 00 00 00              	movl	$0x2e, %ebx
141bf180e: 48 29 d3                    	subq	%rdx, %rbx
141bf1811: 4c 8d 35 50 be 30 01        	leaq	0x130be50(%rip), %r14   # 0x142efd668
141bf1818: 4d 01 c6                    	addq	%r8, %r14
141bf181b: 4c 8d 05 76 bf 30 01        	leaq	0x130bf76(%rip), %r8    # 0x142efd798
141bf1822: 4c 8d 3d 3f bf 30 01        	leaq	0x130bf3f(%rip), %r15   # 0x142efd768
141bf1829: 0f 1f 80 00 00 00 00        	nopl	(%rax)
141bf1830: 41 0f be 40 ff              	movsbl	-0x1(%r8), %eax
141bf1835: 85 c0                       	testl	%eax, %eax
141bf1837: 78 17                       	js	0x141bf1850 <.text+0x1bf0850>
141bf1839: 49 ff c8                    	decq	%r8
141bf183c: 83 f8 5c                    	cmpl	$0x5c, %eax
141bf183f: 75 63                       	jne	0x141bf18a4 <.text+0x1bf08a4>
141bf1841: eb 72                       	jmp	0x141bf18b5 <.text+0x1bf08b5>
141bf1843: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
141bf1850: 41 0f b6 48 fe              	movzbl	-0x2(%r8), %ecx
141bf1855: 80 f9 c0                    	cmpb	$-0x40, %cl
141bf1858: 7d 23                       	jge	0x141bf187d <.text+0x1bf087d>
141bf185a: 41 0f b6 50 fd              	movzbl	-0x3(%r8), %edx
141bf185f: 80 fa c0                    	cmpb	$-0x40, %dl
141bf1862: 7d 22                       	jge	0x141bf1886 <.text+0x1bf0886>
141bf1864: 45 0f b6 48 fc              	movzbl	-0x4(%r8), %r9d
141bf1869: 49 83 c0 fc                 	addq	$-0x4, %r8
141bf186d: 41 83 e1 07                 	andl	$0x7, %r9d
141bf1871: 41 c1 e1 06                 	shll	$0x6, %r9d
141bf1875: 83 e2 3f                    	andl	$0x3f, %edx
141bf1878: 44 09 ca                    	orl	%r9d, %edx
141bf187b: eb 10                       	jmp	0x141bf188d <.text+0x1bf088d>
141bf187d: 49 83 c0 fe                 	addq	$-0x2, %r8
141bf1881: 83 e1 1f                    	andl	$0x1f, %ecx
141bf1884: eb 0f                       	jmp	0x141bf1895 <.text+0x1bf0895>
141bf1886: 49 83 c0 fd                 	addq	$-0x3, %r8
141bf188a: 83 e2 0f                    	andl	$0xf, %edx
141bf188d: c1 e2 06                    	shll	$0x6, %edx
141bf1890: 83 e1 3f                    	andl	$0x3f, %ecx
141bf1893: 09 d1                       	orl	%edx, %ecx
141bf1895: c1 e1 06                    	shll	$0x6, %ecx
141bf1898: 24 3f                       	andb	$0x3f, %al
141bf189a: 0f b6 c0                    	movzbl	%al, %eax
141bf189d: 09 c8                       	orl	%ecx, %eax
141bf189f: 83 f8 5c                    	cmpl	$0x5c, %eax
141bf18a2: 74 11                       	je	0x141bf18b5 <.text+0x1bf08b5>
141bf18a4: 83 f8 2f                    	cmpl	$0x2f, %eax
141bf18a7: 74 0c                       	je	0x141bf18b5 <.text+0x1bf08b5>
141bf18a9: 4d 39 f8                    	cmpq	%r15, %r8
141bf18ac: 75 82                       	jne	0x141bf1830 <.text+0x1bf0830>
141bf18ae: be 30 00 00 00              	movl	$0x30, %esi
141bf18b3: eb 4c                       	jmp	0x141bf1901 <.text+0x1bf0901>
141bf18b5: 4c 8d 3d ac be 30 01        	leaq	0x130beac(%rip), %r15   # 0x142efd768
141bf18bc: 4d 29 f8                    	subq	%r15, %r8
141bf18bf: 49 ff c0                    	incq	%r8
141bf18c2: 74 32                       	je	0x141bf18f6 <.text+0x1bf08f6>
141bf18c4: 49 83 f8 30                 	cmpq	$0x30, %r8
141bf18c8: 73 2a                       	jae	0x141bf18f4 <.text+0x1bf08f4>
141bf18ca: 43 80 3c 38 bf              	cmpb	$-0x41, (%r8,%r15)
141bf18cf: 7f 25                       	jg	0x141bf18f6 <.text+0x1bf08f6>
141bf18d1: 48 8d 05 28 be 30 01        	leaq	0x130be28(%rip), %rax   # 0x142efd700
141bf18d8: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
141bf18dd: 48 8d 0d 84 be 30 01        	leaq	0x130be84(%rip), %rcx   # 0x142efd768
141bf18e4: ba 30 00 00 00              	movl	$0x30, %edx
141bf18e9: 41 b9 30 00 00 00           	movl	$0x30, %r9d
141bf18ef: e8 4c 80 ce 00              	callq	0x1428d9940 <.text+0x28d8940>
141bf18f4: 75 db                       	jne	0x141bf18d1 <.text+0x1bf08d1>
141bf18f6: be 30 00 00 00              	movl	$0x30, %esi
141bf18fb: 4c 29 c6                    	subq	%r8, %rsi
141bf18fe: 4d 01 c7                    	addq	%r8, %r15
141bf1901: e8 2a 68 02 ff              	callq	0x140c18130 <.text+0xc17130>
141bf1906: 48 89 c1                    	movq	%rax, %rcx
141bf1909: 48 8b 00                    	movq	(%rax), %rax
141bf190c: 48 83 f8 01                 	cmpq	$0x1, %rax
141bf1910: 0f 85 2e 0d 00 00           	jne	0x141bf2644 <.text+0x1bf1644>
141bf1916: 48 83 c1 08                 	addq	$0x8, %rcx
141bf191a: 48 83 39 00                 	cmpq	$0x0, (%rcx)
141bf191e: 0f 85 8f 0d 00 00           	jne	0x141bf26b3 <.text+0x1bf16b3>
141bf1924: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
141bf192b: 48 89 8d 48 01 00 00        	movq	%rcx, 0x148(%rbp)
141bf1932: 48 83 c1 08                 	addq	$0x8, %rcx
141bf1936: 48 89 74 24 20              	movq	%rsi, 0x20(%rsp)
141bf193b: 48 c7 44 24 30 00 00 00 00  	movq	$0x0, 0x30(%rsp)
141bf1944: 48 c7 44 24 28 01 00 00 00  	movq	$0x1, 0x28(%rsp)
141bf194d: 4c 89 f2                    	movq	%r14, %rdx
141bf1950: 49 89 d8                    	movq	%rbx, %r8
141bf1953: 4d 89 f9                    	movq	%r15, %r9
141bf1956: e8 e5 53 02 ff              	callq	0x140c16d40 <.text+0xc15d40>
141bf195b: 48 8b 8d 48 01 00 00        	movq	0x148(%rbp), %rcx
141bf1962: 48 ff 01                    	incq	(%rcx)
141bf1965: 48 89 85 a0 00 00 00        	movq	%rax, 0xa0(%rbp)
141bf196c: 48 89 85 80 00 00 00        	movq	%rax, 0x80(%rbp)
141bf1973: b8 01 00 00 00              	movl	$0x1, %eax
141bf1978: 4c 8b bd b8 02 00 00        	movq	0x2b8(%rbp), %r15
141bf197f: 48 8b 8d a8 02 00 00        	movq	0x2a8(%rbp), %rcx
141bf1986: 48 89 45 78                 	movq	%rax, 0x78(%rbp)
141bf198a: 49 c7 47 10 00 00 00 00     	movq	$0x0, 0x10(%r15)
141bf1992: 48 8b 09                    	movq	(%rcx), %rcx
141bf1995: e8 e6 c3 a8 fe              	callq	0x14067dd80 <.text+0x67cd80>
141bf199a: 48 8b 95 a0 02 00 00        	movq	0x2a0(%rbp), %rdx
141bf19a1: 48 8d 48 08                 	leaq	0x8(%rax), %rcx
141bf19a5: 4c 8d 40 20                 	leaq	0x20(%rax), %r8
141bf19a9: 4c 8d 48 10                 	leaq	0x10(%rax), %r9
141bf19ad: 49 89 c2                    	movq	%rax, %r10
141bf19b0: 49 83 c2 28                 	addq	$0x28, %r10
141bf19b4: 80 b8 98 02 00 00 00        	cmpb	$0x0, 0x298(%rax)
141bf19bb: 49 0f 45 c8                 	cmovneq	%r8, %rcx
141bf19bf: 4d 0f 44 d1                 	cmoveq	%r9, %r10
141bf19c3: 49 8b 02                    	movq	(%r10), %rax
141bf19c6: 48 8b 09                    	movq	(%rcx), %rcx
141bf19c9: 48 89 4d 40                 	movq	%rcx, 0x40(%rbp)
141bf19cd: 48 89 45 48                 	movq	%rax, 0x48(%rbp)
141bf19d1: 48 8d 4d b8                 	leaq	-0x48(%rbp), %rcx
141bf19d5: e8 46 32 ef fe              	callq	0x140ae4c20 <.text+0xae3c20>
141bf19da: 48 8b 85 b0 02 00 00        	movq	0x2b0(%rbp), %rax
141bf19e1: 48 8b 08                    	movq	(%rax), %rcx
141bf19e4: e8 97 c3 a8 fe              	callq	0x14067dd80 <.text+0x67cd80>
141bf19e9: 48 8d 48 08                 	leaq	0x8(%rax), %rcx
141bf19ed: 48 8d 50 20                 	leaq	0x20(%rax), %rdx
141bf19f1: 49 89 c0                    	movq	%rax, %r8
141bf19f4: 49 83 c0 28                 	addq	$0x28, %r8
141bf19f8: 80 b8 98 02 00 00 00        	cmpb	$0x0, 0x298(%rax)
141bf19ff: 48 0f 45 ca                 	cmovneq	%rdx, %rcx
141bf1a03: 48 8d 40 10                 	leaq	0x10(%rax), %rax
141bf1a07: 4c 0f 44 c0                 	cmoveq	%rax, %r8
141bf1a0b: 49 8b 00                    	movq	(%r8), %rax
141bf1a0e: 48 8b 09                    	movq	(%rcx), %rcx
141bf1a11: 48 89 4d 30                 	movq	%rcx, 0x30(%rbp)
141bf1a15: 48 89 45 38                 	movq	%rax, 0x38(%rbp)
141bf1a19: 48 8b 0f                    	movq	(%rdi), %rcx
141bf1a1c: 48 8d 15 bd 58 44 01        	leaq	0x14458bd(%rip), %rdx   # 0x1430372e0
141bf1a23: e8 a8 68 68 00              	callq	0x1422782d0 <.text+0x22772d0>
141bf1a28: 48 89 c7                    	movq	%rax, %rdi
141bf1a2b: 48 85 c0                    	testq	%rax, %rax
141bf1a2e: 0f 84 0d 0d 00 00           	je	0x141bf2741 <.text+0x1bf1741>
141bf1a34: 48 8d 05 dd 59 44 01        	leaq	0x14459dd(%rip), %rax   # 0x143037418
141bf1a3b: 48 89 45 08                 	movq	%rax, 0x8(%rbp)
141bf1a3f: 48 c7 45 10 3f 00 00 00     	movq	$0x3f, 0x10(%rbp)
141bf1a47: 48 89 fb                    	movq	%rdi, %rbx
141bf1a4a: 48 83 c3 10                 	addq	$0x10, %rbx
141bf1a4e: 48 83 7f 18 3f              	cmpq	$0x3f, 0x18(%rdi)
141bf1a53: 0f 85 29 0c 00 00           	jne	0x141bf2682 <.text+0x1bf1682>
141bf1a59: 48 8b 13                    	movq	(%rbx), %rdx
141bf1a5c: 48 8d 0d b5 59 44 01        	leaq	0x14459b5(%rip), %rcx   # 0x143037418
141bf1a63: 41 b8 3f 00 00 00           	movl	$0x3f, %r8d
141bf1a69: e8 82 21 d3 00              	callq	0x142923bf0 <.text+0x2922bf0>
141bf1a6e: 85 c0                       	testl	%eax, %eax
141bf1a70: 0f 85 0c 0c 00 00           	jne	0x141bf2682 <.text+0x1bf1682>
141bf1a76: 48 8b 0f                    	movq	(%rdi), %rcx
141bf1a79: 48 8b 47 08                 	movq	0x8(%rdi), %rax
141bf1a7d: ff 50 18                    	callq	*0x18(%rax)
141bf1a80: 48 85 c0                    	testq	%rax, %rax
141bf1a83: 0f 84 b8 0c 00 00           	je	0x141bf2741 <.text+0x1bf1741>
141bf1a89: 48 8b 95 80 02 00 00        	movq	0x280(%rbp), %rdx
141bf1a90: 48 8b 0a                    	movq	(%rdx), %rcx
141bf1a93: 4c 8b 42 08                 	movq	0x8(%rdx), %r8
141bf1a97: 4c 8b 09                    	movq	(%rcx), %r9
141bf1a9a: 31 d2                       	xorl	%edx, %edx
141bf1a9c: 4c 89 ce                    	movq	%r9, %rsi
141bf1a9f: 49 2b 70 18                 	subq	0x18(%r8), %rsi
141bf1aa3: 48 0f 42 f2                 	cmovbq	%rdx, %rsi
141bf1aa7: 4d 2b 48 38                 	subq	0x38(%r8), %r9
141bf1aab: 4c 0f 42 ca                 	cmovbq	%rdx, %r9
141bf1aaf: 48 8d 3c b6                 	leaq	(%rsi,%rsi,4), %rdi
141bf1ab3: 48 c1 e7 03                 	shlq	$0x3, %rdi
141bf1ab7: 49 03 78 08                 	addq	0x8(%r8), %rdi
141bf1abb: 4f 8d 1c 89                 	leaq	(%r9,%r9,4), %r11
141bf1abf: 49 c1 e3 03                 	shlq	$0x3, %r11
141bf1ac3: 4d 03 58 28                 	addq	0x28(%r8), %r11
141bf1ac7: 4d 8b 50 10                 	movq	0x10(%r8), %r10
141bf1acb: 49 29 f2                    	subq	%rsi, %r10
141bf1ace: 4c 0f 42 d2                 	cmovbq	%rdx, %r10
141bf1ad2: bb 08 00 00 00              	movl	$0x8, %ebx
141bf1ad7: 48 0f 42 fb                 	cmovbq	%rbx, %rdi
141bf1adb: 4b 8d 34 92                 	leaq	(%r10,%r10,4), %rsi
141bf1adf: 41 0f 96 c6                 	setbe	%r14b
141bf1ae3: 48 8d 7c f7 d8              	leaq	-0x28(%rdi,%rsi,8), %rdi
141bf1ae8: 49 8b 70 30                 	movq	0x30(%r8), %rsi
141bf1aec: 48 0f 46 fa                 	cmovbeq	%rdx, %rdi
141bf1af0: 4c 29 ce                    	subq	%r9, %rsi
141bf1af3: 4c 8b 8d 88 02 00 00        	movq	0x288(%rbp), %r9
141bf1afa: 48 0f 42 f2                 	cmovbq	%rdx, %rsi
141bf1afe: 4c 0f 42 db                 	cmovbq	%rbx, %r11
141bf1b02: 48 8d 1c b6                 	leaq	(%rsi,%rsi,4), %rbx
141bf1b06: 4d 8d 5c db d8              	leaq	-0x28(%r11,%rbx,8), %r11
141bf1b0b: 4c 0f 46 df                 	cmovbeq	%rdi, %r11
141bf1b0f: 0f 96 c3                    	setbe	%bl
141bf1b12: 4c 01 d6                    	addq	%r10, %rsi
141bf1b15: 4c 8b 10                    	movq	(%rax), %r10
141bf1b18: 4c 89 95 38 01 00 00        	movq	%r10, 0x138(%rbp)
141bf1b1f: 48 8b 40 08                 	movq	0x8(%rax), %rax
141bf1b23: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
141bf1b2a: 49 8b 40 40                 	movq	0x40(%r8), %rax
141bf1b2e: 44 20 f3                    	andb	%r14b, %bl
141bf1b31: 88 9d 08 01 00 00           	movb	%bl, 0x108(%rbp)
141bf1b37: 48 0f 44 f2                 	cmoveq	%rdx, %rsi
141bf1b3b: 48 29 f0                    	subq	%rsi, %rax
141bf1b3e: 48 89 01                    	movq	%rax, (%rcx)
141bf1b41: 48 8b 85 40 01 00 00        	movq	0x140(%rbp), %rax
141bf1b48: 48 8b 00                    	movq	(%rax), %rax
141bf1b4b: 48 8b 58 08                 	movq	0x8(%rax), %rbx
141bf1b4f: 48 89 85 48 01 00 00        	movq	%rax, 0x148(%rbp)
141bf1b56: 48 8b 40 10                 	movq	0x10(%rax), %rax
141bf1b5a: 48 8d 04 80                 	leaq	(%rax,%rax,4), %rax
141bf1b5e: 48 c1 e0 04                 	shlq	$0x4, %rax
141bf1b62: 48 01 d8                    	addq	%rbx, %rax
141bf1b65: 48 89 85 00 01 00 00        	movq	%rax, 0x100(%rbp)
141bf1b6c: 49 8d 43 04                 	leaq	0x4(%r11), %rax
141bf1b70: 48 89 85 20 01 00 00        	movq	%rax, 0x120(%rbp)
141bf1b77: 49 8b 01                    	movq	(%r9), %rax
141bf1b7a: 48 89 85 18 01 00 00        	movq	%rax, 0x118(%rbp)
141bf1b81: 44 0f 28 05 f7 cd d3 00     	movaps	0xd3cdf7(%rip), %xmm8   # 0x14292e980
141bf1b89: f3 0f 10 35 83 1c e9 00     	movss	0xe91c83(%rip), %xmm6   # 0x142a83814
141bf1b91: f3 0f 10 3d bf 9a e8 00     	movss	0xe89abf(%rip), %xmm7   # 0x142a7b658
141bf1b99: eb 27                       	jmp	0x141bf1bc2 <.text+0x1bf0bc2>
141bf1b9b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
141bf1ba0: 48 83 c3 50                 	addq	$0x50, %rbx
141bf1ba4: 49 8b 47 08                 	movq	0x8(%r15), %rax
141bf1ba8: 4c 89 e1                    	movq	%r12, %rcx
141bf1bab: 48 c1 e1 04                 	shlq	$0x4, %rcx
141bf1baf: 4c 89 34 08                 	movq	%r14, (%rax,%rcx)
141bf1bb3: 40 88 74 08 08              	movb	%sil, 0x8(%rax,%rcx)
141bf1bb8: 49 ff c4                    	incq	%r12
141bf1bbb: 4d 89 67 10                 	movq	%r12, 0x10(%r15)
141bf1bbf: 49 89 fb                    	movq	%rdi, %r11
141bf1bc2: 48 3b 9d 00 01 00 00        	cmpq	0x100(%rbp), %rbx
141bf1bc9: 0f 84 72 02 00 00           	je	0x141bf1e41 <.text+0x1bf0e41>
141bf1bcf: 4c 89 df                    	movq	%r11, %rdi
141bf1bd2: 48 8d 53 10                 	leaq	0x10(%rbx), %rdx
141bf1bd6: 48 8d 4d 54                 	leaq	0x54(%rbp), %rcx
141bf1bda: e8 c1 35 ee fe              	callq	0x140ad51a0 <.text+0xad41a0>
141bf1bdf: f3 44 0f 10 4d 54           	movss	0x54(%rbp), %xmm9
141bf1be5: 41 0f 28 c1                 	movaps	%xmm9, %xmm0
141bf1be9: 41 0f 54 c0                 	andps	%xmm8, %xmm0
141bf1bed: 0f 2e c6                    	ucomiss	%xmm6, %xmm0
141bf1bf0: 77 13                       	ja	0x141bf1c05 <.text+0x1bf0c05>
141bf1bf2: f3 44 0f 10 55 5c           	movss	0x5c(%rbp), %xmm10
141bf1bf8: 41 0f 28 c2                 	movaps	%xmm10, %xmm0
141bf1bfc: 41 0f 54 c0                 	andps	%xmm8, %xmm0
141bf1c00: 0f 2e c6                    	ucomiss	%xmm6, %xmm0
141bf1c03: 76 2b                       	jbe	0x141bf1c30 <.text+0x1bf0c30>
141bf1c05: c6 85 a8 00 00 00 01        	movb	$0x1, 0xa8(%rbp)
141bf1c0c: 40 b6 01                    	movb	$0x1, %sil
141bf1c0f: 4c 8b 73 40                 	movq	0x40(%rbx), %r14
141bf1c13: 4d 8b 67 10                 	movq	0x10(%r15), %r12
141bf1c17: 4d 3b 27                    	cmpq	(%r15), %r12
141bf1c1a: 75 84                       	jne	0x141bf1ba0 <.text+0x1bf0ba0>
141bf1c1c: 4c 89 f9                    	movq	%r15, %rcx
141bf1c1f: 48 8d 15 4a bc 30 01        	leaq	0x130bc4a(%rip), %rdx   # 0x142efd870
141bf1c26: e8 05 0a 97 fe              	callq	0x140562630 <.text+0x561630>
141bf1c2b: e9 70 ff ff ff              	jmp	0x141bf1ba0 <.text+0x1bf0ba0>
141bf1c30: c7 44 24 20 00 00 02 43     	movl	$0x43020000, 0x20(%rsp) # imm = 0x43020000
141bf1c38: 48 8d 4d b8                 	leaq	-0x48(%rbp), %rcx
141bf1c3c: 41 0f 28 c9                 	movaps	%xmm9, %xmm1
141bf1c40: 41 0f 28 d2                 	movaps	%xmm10, %xmm2
141bf1c44: 0f 28 df                    	movaps	%xmm7, %xmm3
141bf1c47: e8 a4 31 e2 fe              	callq	0x140a14df0 <.text+0xa13df0>
141bf1c4c: 44 0f 28 d8                 	movaps	%xmm0, %xmm11
141bf1c50: c7 44 24 20 00 00 02 43     	movl	$0x43020000, 0x20(%rsp) # imm = 0x43020000
141bf1c58: 48 8d 4d c8                 	leaq	-0x38(%rbp), %rcx
141bf1c5c: 41 0f 28 c9                 	movaps	%xmm9, %xmm1
141bf1c60: 41 0f 28 d2                 	movaps	%xmm10, %xmm2
141bf1c64: 0f 28 df                    	movaps	%xmm7, %xmm3
141bf1c67: e8 84 31 e2 fe              	callq	0x140a14df0 <.text+0xa13df0>
141bf1c6c: 44 0f 28 e0                 	movaps	%xmm0, %xmm12
141bf1c70: c7 44 24 20 00 00 02 43     	movl	$0x43020000, 0x20(%rsp) # imm = 0x43020000
141bf1c78: 48 8d 4d d8                 	leaq	-0x28(%rbp), %rcx
141bf1c7c: 41 0f 28 c9                 	movaps	%xmm9, %xmm1
141bf1c80: 41 0f 28 d2                 	movaps	%xmm10, %xmm2
141bf1c84: 0f 28 df                    	movaps	%xmm7, %xmm3
141bf1c87: e8 64 31 e2 fe              	callq	0x140a14df0 <.text+0xa13df0>
141bf1c8c: 44 0f 28 e8                 	movaps	%xmm0, %xmm13
141bf1c90: c7 44 24 20 00 00 02 43     	movl	$0x43020000, 0x20(%rsp) # imm = 0x43020000
141bf1c98: 48 8d 4d e8                 	leaq	-0x18(%rbp), %rcx
141bf1c9c: 41 0f 28 c9                 	movaps	%xmm9, %xmm1
141bf1ca0: 41 0f 28 d2                 	movaps	%xmm10, %xmm2
141bf1ca4: 0f 28 df                    	movaps	%xmm7, %xmm3
141bf1ca7: e8 44 31 e2 fe              	callq	0x140a14df0 <.text+0xa13df0>
141bf1cac: 44 0f 28 f0                 	movaps	%xmm0, %xmm14
141bf1cb0: c7 44 24 20 00 00 02 43     	movl	$0x43020000, 0x20(%rsp) # imm = 0x43020000
141bf1cb8: 48 8d 4d 40                 	leaq	0x40(%rbp), %rcx
141bf1cbc: 41 0f 28 c9                 	movaps	%xmm9, %xmm1
141bf1cc0: 41 0f 28 d2                 	movaps	%xmm10, %xmm2
141bf1cc4: 0f 28 df                    	movaps	%xmm7, %xmm3
141bf1cc7: e8 24 31 e2 fe              	callq	0x140a14df0 <.text+0xa13df0>
141bf1ccc: 44 0f 28 f8                 	movaps	%xmm0, %xmm15
141bf1cd0: c7 44 24 20 00 00 02 43     	movl	$0x43020000, 0x20(%rsp) # imm = 0x43020000
141bf1cd8: 48 8d 4d 30                 	leaq	0x30(%rbp), %rcx
141bf1cdc: 41 0f 28 c9                 	movaps	%xmm9, %xmm1
141bf1ce0: 41 0f 28 d2                 	movaps	%xmm10, %xmm2
141bf1ce4: 0f 28 df                    	movaps	%xmm7, %xmm3
141bf1ce7: e8 04 31 e2 fe              	callq	0x140a14df0 <.text+0xa13df0>
141bf1cec: 0f 57 c9                    	xorps	%xmm1, %xmm1
141bf1cef: 44 0f 2e f9                 	ucomiss	%xmm1, %xmm15
141bf1cf3: 0f 87 b4 00 00 00           	ja	0x141bf1dad <.text+0x1bf0dad>
141bf1cf9: 0f 2e c1                    	ucomiss	%xmm1, %xmm0
141bf1cfc: 0f 87 ab 00 00 00           	ja	0x141bf1dad <.text+0x1bf0dad>
141bf1d02: 48 8b 8d 38 01 00 00        	movq	0x138(%rbp), %rcx
141bf1d09: 48 8b 95 10 01 00 00        	movq	0x110(%rbp), %rdx
141bf1d10: 41 0f 28 d1                 	movaps	%xmm9, %xmm2
141bf1d14: 41 0f 28 da                 	movaps	%xmm10, %xmm3
141bf1d18: e8 e3 8a df fe              	callq	0x1409ea800 <.text+0x9e9800>
141bf1d1d: f3 45 0f 58 dc              	addss	%xmm12, %xmm11
141bf1d22: f3 45 0f 58 dd              	addss	%xmm13, %xmm11
141bf1d27: f3 45 0f 58 de              	addss	%xmm14, %xmm11
141bf1d2c: 0f 57 c0                    	xorps	%xmm0, %xmm0
141bf1d2f: f3 41 0f 5f c3              	maxss	%xmm11, %xmm0
141bf1d34: f3 0f 10 0d c8 3b d3 00     	movss	0xd33bc8(%rip), %xmm1   # 0x142925904
141bf1d3c: f3 0f 5d c8                 	minss	%xmm0, %xmm1
141bf1d40: 0f 57 c0                    	xorps	%xmm0, %xmm0
141bf1d43: 0f 2e c8                    	ucomiss	%xmm0, %xmm1
141bf1d46: 40 0f 97 c6                 	seta	%sil
141bf1d4a: 40 08 c6                    	orb	%al, %sil
141bf1d4d: 0f b6 85 08 01 00 00        	movzbl	0x108(%rbp), %eax
141bf1d54: 40 08 f0                    	orb	%sil, %al
141bf1d57: 40 80 f6 01                 	xorb	$0x1, %sil
141bf1d5b: 40 88 b5 a8 00 00 00        	movb	%sil, 0xa8(%rbp)
141bf1d62: 3c 01                       	cmpb	$0x1, %al
141bf1d64: 0f 84 92 00 00 00           	je	0x141bf1dfc <.text+0x1bf0dfc>
141bf1d6a: f6 07 01                    	testb	$0x1, (%rdi)
141bf1d6d: 74 4c                       	je	0x141bf1dbb <.text+0x1bf0dbb>
141bf1d6f: 48 8b 8d 20 01 00 00        	movq	0x120(%rbp), %rcx
141bf1d76: 41 0f 28 c9                 	movaps	%xmm9, %xmm1
141bf1d7a: 41 0f 28 d2                 	movaps	%xmm10, %xmm2
141bf1d7e: e8 9d 90 0a ff              	callq	0x140c9ae20 <.text+0xc99e20>
141bf1d83: 0f 14 c1                    	unpcklps	%xmm1, %xmm0            # xmm0 = xmm0[0],xmm1[0],xmm0[1],xmm1[1]
141bf1d86: 41 0f 28 c9                 	movaps	%xmm9, %xmm1
141bf1d8a: 41 0f 14 ca                 	unpcklps	%xmm10, %xmm1           # xmm1 = xmm1[0],xmm10[0],xmm1[1],xmm10[1]
141bf1d8e: 0f 5c c1                    	subps	%xmm1, %xmm0
141bf1d91: 0f 59 c0                    	mulps	%xmm0, %xmm0
141bf1d94: f3 0f 16 c8                 	movshdup	%xmm0, %xmm1            # xmm1 = xmm0[1,1,3,3]
141bf1d98: f3 0f 58 c8                 	addss	%xmm0, %xmm1
141bf1d9c: 0f 57 c0                    	xorps	%xmm0, %xmm0
141bf1d9f: f3 0f 51 c1                 	sqrtss	%xmm1, %xmm0
141bf1da3: f3 0f 10 0d 9d af e8 00     	movss	0xe8af9d(%rip), %xmm1   # 0x142a7cd48
141bf1dab: eb 3d                       	jmp	0x141bf1dea <.text+0x1bf0dea>
141bf1dad: c6 85 a8 00 00 00 00        	movb	$0x0, 0xa8(%rbp)
141bf1db4: 31 f6                       	xorl	%esi, %esi
141bf1db6: e9 54 fe ff ff              	jmp	0x141bf1c0f <.text+0x1bf0c0f>
141bf1dbb: f2 0f 10 47 04              	movsd	0x4(%rdi), %xmm0
141bf1dc0: 41 0f 28 c9                 	movaps	%xmm9, %xmm1
141bf1dc4: 41 0f 14 ca                 	unpcklps	%xmm10, %xmm1           # xmm1 = xmm1[0],xmm10[0],xmm1[1],xmm10[1]
141bf1dc8: 0f 5c c1                    	subps	%xmm1, %xmm0
141bf1dcb: 0f 59 c0                    	mulps	%xmm0, %xmm0
141bf1dce: f3 0f 16 c8                 	movshdup	%xmm0, %xmm1            # xmm1 = xmm0[1,1,3,3]
141bf1dd2: f3 0f 58 c8                 	addss	%xmm0, %xmm1
141bf1dd6: 0f 57 c0                    	xorps	%xmm0, %xmm0
141bf1dd9: f3 0f 51 c1                 	sqrtss	%xmm1, %xmm0
141bf1ddd: f3 0f 10 4f 0c              	movss	0xc(%rdi), %xmm1
141bf1de2: f3 0f 58 0d 5e af e8 00     	addss	0xe8af5e(%rip), %xmm1   # 0x142a7cd48
141bf1dea: 0f 2e c8                    	ucomiss	%xmm0, %xmm1
141bf1ded: 40 0f 97 c6                 	seta	%sil
141bf1df1: 40 80 f6 01                 	xorb	$0x1, %sil
141bf1df5: 40 88 b5 a8 00 00 00        	movb	%sil, 0xa8(%rbp)
141bf1dfc: 40 84 f6                    	testb	%sil, %sil
141bf1dff: 0f 84 0a fe ff ff           	je	0x141bf1c0f <.text+0x1bf0c0f>
141bf1e05: 48 8d 4d f8                 	leaq	-0x8(%rbp), %rcx
141bf1e09: 41 0f 28 c9                 	movaps	%xmm9, %xmm1
141bf1e0d: 41 0f 28 d2                 	movaps	%xmm10, %xmm2
141bf1e11: f3 0f 10 1d 2f af e8 00     	movss	0xe8af2f(%rip), %xmm3   # 0x142a7cd48
141bf1e19: e8 22 57 0a ff              	callq	0x140c97540 <.text+0xc96540>
141bf1e1e: 48 8b 8d 18 01 00 00        	movq	0x118(%rbp), %rcx
141bf1e25: 48 8d 55 f8                 	leaq	-0x8(%rbp), %rdx
141bf1e29: 4c 8d 85 a8 00 00 00        	leaq	0xa8(%rbp), %r8
141bf1e30: e8 9b 67 68 00              	callq	0x1422785d0 <.text+0x22775d0>
141bf1e35: 0f b6 b5 a8 00 00 00        	movzbl	0xa8(%rbp), %esi
141bf1e3c: e9 ce fd ff ff              	jmp	0x141bf1c0f <.text+0x1bf0c0f>
141bf1e41: 48 8b 85 90 02 00 00        	movq	0x290(%rbp), %rax
141bf1e48: 48 8b 00                    	movq	(%rax), %rax
141bf1e4b: 8b 48 2c                    	movl	0x2c(%rax), %ecx
141bf1e4e: e8 ed 43 0a ff              	callq	0x140c96240 <.text+0xc95240>
141bf1e53: 48 89 45 70                 	movq	%rax, 0x70(%rbp)
141bf1e57: 4d 8b 47 08                 	movq	0x8(%r15), %r8
141bf1e5b: 49 8b 47 10                 	movq	0x10(%r15), %rax
141bf1e5f: 48 c1 e0 04                 	shlq	$0x4, %rax
141bf1e63: 4c 01 c0                    	addq	%r8, %rax
141bf1e66: 48 89 85 00 01 00 00        	movq	%rax, 0x100(%rbp)
141bf1e6d: 48 8b 8d 78 02 00 00        	movq	0x278(%rbp), %rcx
141bf1e74: 48 8b 01                    	movq	(%rcx), %rax
141bf1e77: 48 89 85 28 01 00 00        	movq	%rax, 0x128(%rbp)
141bf1e7e: 48 8b 41 08                 	movq	0x8(%rcx), %rax
141bf1e82: 48 89 85 38 01 00 00        	movq	%rax, 0x138(%rbp)
141bf1e89: 48 8b 85 98 02 00 00        	movq	0x298(%rbp), %rax
141bf1e90: 48 8b 00                    	movq	(%rax), %rax
141bf1e93: 48 89 85 98 00 00 00        	movq	%rax, 0x98(%rbp)
141bf1e9a: 48 8b 85 40 01 00 00        	movq	0x140(%rbp), %rax
141bf1ea1: 8b 48 1c                    	movl	0x1c(%rax), %ecx
141bf1ea4: 89 8d 18 01 00 00           	movl	%ecx, 0x118(%rbp)
141bf1eaa: 48 8b 40 10                 	movq	0x10(%rax), %rax
141bf1eae: 48 89 85 20 01 00 00        	movq	%rax, 0x120(%rbp)
141bf1eb5: 48 8b 8d 70 02 00 00        	movq	0x270(%rbp), %rcx
141bf1ebc: 8b 41 1c                    	movl	0x1c(%rcx), %eax
141bf1ebf: 89 85 fc 00 00 00           	movl	%eax, 0xfc(%rbp)
141bf1ec5: 48 8b 01                    	movq	(%rcx), %rax
141bf1ec8: 48 8b 49 10                 	movq	0x10(%rcx), %rcx
141bf1ecc: 48 89 8d 88 00 00 00        	movq	%rcx, 0x88(%rbp)
141bf1ed3: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
141bf1eda: 48 83 c0 20                 	addq	$0x20, %rax
141bf1ede: 48 89 85 90 00 00 00        	movq	%rax, 0x90(%rbp)
141bf1ee5: 31 db                       	xorl	%ebx, %ebx
141bf1ee7: f3 44 0f 10 15 e8 39 d3 00  	movss	0xd339e8(%rip), %xmm10  # 0x1429258d8
141bf1ef0: f3 44 0f 10 1d 97 bc df 00  	movss	0xdfbc97(%rip), %xmm11  # 0x1429edb90
141bf1ef9: f3 44 0f 10 25 4e 2e ed 00  	movss	0xed2e4e(%rip), %xmm12  # 0x142ac4d50
141bf1f02: f2 44 0f 10 2d a5 3e ea 00  	movsd	0xea3ea5(%rip), %xmm13  # 0x142a95db0
141bf1f0b: 4c 8b a5 48 01 00 00        	movq	0x148(%rbp), %r12
141bf1f12: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
141bf1f20: 4c 3b 85 00 01 00 00        	cmpq	0x100(%rbp), %r8
141bf1f27: 0f 84 4c 06 00 00           	je	0x141bf2579 <.text+0x1bf1579>
141bf1f2d: 4c 89 c6                    	movq	%r8, %rsi
141bf1f30: 41 80 7c 24 18 01           	cmpb	$0x1, 0x18(%r12)
141bf1f36: 75 38                       	jne	0x141bf1f70 <.text+0x1bf0f70>
141bf1f38: 48 8d 4d 70                 	leaq	0x70(%rbp), %rcx
141bf1f3c: e8 3f d4 0b ff              	callq	0x140caf380 <.text+0xcae380>
141bf1f41: 0f 28 f8                    	movaps	%xmm0, %xmm7
141bf1f44: f3 0f 10 05 b8 39 d3 00     	movss	0xd339b8(%rip), %xmm0   # 0x142925904
141bf1f4c: f3 0f 5c c7                 	subss	%xmm7, %xmm0
141bf1f50: f3 41 0f 59 c2              	mulss	%xmm10, %xmm0
141bf1f55: f3 41 0f 59 fb              	mulss	%xmm11, %xmm7
141bf1f5a: f3 0f 58 f8                 	addss	%xmm0, %xmm7
141bf1f5e: 45 0f 57 c0                 	xorps	%xmm8, %xmm8
141bf1f62: 45 0f 57 f6                 	xorps	%xmm14, %xmm14
141bf1f66: 4c 8b a5 48 01 00 00        	movq	0x148(%rbp), %r12
141bf1f6d: eb 2f                       	jmp	0x141bf1f9e <.text+0x1bf0f9e>
141bf1f6f: 90                          	nop
141bf1f70: 49 8b 54 24 10              	movq	0x10(%r12), %rdx
141bf1f75: 48 39 d3                    	cmpq	%rdx, %rbx
141bf1f78: 0f 83 41 07 00 00           	jae	0x141bf26bf <.text+0x1bf16bf>
141bf1f7e: 49 8b 44 24 08              	movq	0x8(%r12), %rax
141bf1f83: 48 8d 0c 9b                 	leaq	(%rbx,%rbx,4), %rcx
141bf1f87: 48 c1 e1 04                 	shlq	$0x4, %rcx
141bf1f8b: f3 44 0f 10 74 08 1c        	movss	0x1c(%rax,%rcx), %xmm14
141bf1f92: 0f 57 ff                    	xorps	%xmm7, %xmm7
141bf1f95: f3 44 0f 10 05 66 39 d3 00  	movss	0xd33966(%rip), %xmm8   # 0x142925904
141bf1f9e: 4c 8d 46 10                 	leaq	0x10(%rsi), %r8
141bf1fa2: 4c 8d 73 01                 	leaq	0x1(%rbx), %r14
141bf1fa6: 8b 46 04                    	movl	0x4(%rsi), %eax
141bf1fa9: 80 7e 08 00                 	cmpb	$0x0, 0x8(%rsi)
141bf1fad: 0f 84 fd 00 00 00           	je	0x141bf20b0 <.text+0x1bf10b0>
141bf1fb3: 85 c0                       	testl	%eax, %eax
141bf1fb5: 0f 84 e6 01 00 00           	je	0x141bf21a1 <.text+0x1bf11a1>
141bf1fbb: 48 8b 36                    	movq	(%rsi), %rsi
141bf1fbe: 48 89 f0                    	movq	%rsi, %rax
141bf1fc1: 48 c1 e8 20                 	shrq	$0x20, %rax
141bf1fc5: 0f 84 05 07 00 00           	je	0x141bf26d0 <.text+0x1bf16d0>
141bf1fcb: 89 f1                       	movl	%esi, %ecx
141bf1fcd: 4c 89 f3                    	movq	%r14, %rbx
141bf1fd0: 48 8b 95 38 01 00 00        	movq	0x138(%rbp), %rdx
141bf1fd7: 48 3b 4a 10                 	cmpq	0x10(%rdx), %rcx
141bf1fdb: 0f 83 3f ff ff ff           	jae	0x141bf1f20 <.text+0x1bf0f20>
141bf1fe1: 48 8b 95 38 01 00 00        	movq	0x138(%rbp), %rdx
141bf1fe8: 48 8b 52 08                 	movq	0x8(%rdx), %rdx
141bf1fec: 48 8d 0c 89                 	leaq	(%rcx,%rcx,4), %rcx
141bf1ff0: 4c 89 f3                    	movq	%r14, %rbx
141bf1ff3: 39 04 8a                    	cmpl	%eax, (%rdx,%rcx,4)
141bf1ff6: 0f 85 24 ff ff ff           	jne	0x141bf1f20 <.text+0x1bf0f20>
141bf1ffc: 48 8d 04 8a                 	leaq	(%rdx,%rcx,4), %rax
141bf2000: 8b 40 04                    	movl	0x4(%rax), %eax
141bf2003: 4c 89 f3                    	movq	%r14, %rbx
141bf2006: b9 ff ff ff ff              	movl	$0xffffffff, %ecx       # imm = 0xFFFFFFFF
141bf200b: 48 39 c8                    	cmpq	%rcx, %rax
141bf200e: 0f 84 0c ff ff ff           	je	0x141bf1f20 <.text+0x1bf0f20>
141bf2014: 4c 89 f3                    	movq	%r14, %rbx
141bf2017: 48 8b 8d 28 01 00 00        	movq	0x128(%rbp), %rcx
141bf201e: 48 39 41 38                 	cmpq	%rax, 0x38(%rcx)
141bf2022: 0f 86 f8 fe ff ff           	jbe	0x141bf1f20 <.text+0x1bf0f20>
141bf2028: 48 8b 8d 28 01 00 00        	movq	0x128(%rbp), %rcx
141bf202f: 48 8b 49 28                 	movq	0x28(%rcx), %rcx
141bf2033: 89 c2                       	movl	%eax, %edx
141bf2035: c1 ea 06                    	shrl	$0x6, %edx
141bf2038: 48 8b 0c d1                 	movq	(%rcx,%rdx,8), %rcx
141bf203c: 48 0f a3 c1                 	btq	%rax, %rcx
141bf2040: 4c 89 f3                    	movq	%r14, %rbx
141bf2043: 0f 83 d7 fe ff ff           	jae	0x141bf1f20 <.text+0x1bf0f20>
141bf2049: 4d 89 c7                    	movq	%r8, %r15
141bf204c: 48 8b 85 30 01 00 00        	movq	0x130(%rbp), %rax
141bf2053: 48 8b 78 08                 	movq	0x8(%rax), %rdi
141bf2057: 48 85 ff                    	testq	%rdi, %rdi
141bf205a: 75 0b                       	jne	0x141bf2067 <.text+0x1bf1067>
141bf205c: 48 8b 85 30 01 00 00        	movq	0x130(%rbp), %rax
141bf2063: 48 8b 78 10                 	movq	0x10(%rax), %rdi
141bf2067: 48 8b 07                    	movq	(%rdi), %rax
141bf206a: 48 8b 5f 10                 	movq	0x10(%rdi), %rbx
141bf206e: 48 29 d8                    	subq	%rbx, %rax
141bf2071: 48 83 f8 17                 	cmpq	$0x17, %rax
141bf2075: 0f 86 d9 04 00 00           	jbe	0x141bf2554 <.text+0x1bf1554>
141bf207b: 48 8b 47 08                 	movq	0x8(%rdi), %rax
141bf207f: 48 8d 0d 9a 68 68 00        	leaq	0x68689a(%rip), %rcx    # 0x142278920 <.text+0x2277920>
141bf2086: 48 89 0c 18                 	movq	%rcx, (%rax,%rbx)
141bf208a: 48 8d 0d 5f 6c 68 00        	leaq	0x686c5f(%rip), %rcx    # 0x142278cf0 <.text+0x2277cf0>
141bf2091: 48 89 4c 18 08              	movq	%rcx, 0x8(%rax,%rbx)
141bf2096: 48 89 74 18 10              	movq	%rsi, 0x10(%rax,%rbx)
141bf209b: 48 83 c3 18                 	addq	$0x18, %rbx
141bf209f: e9 e7 00 00 00              	jmp	0x141bf218b <.text+0x1bf118b>
141bf20a4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
141bf20b0: 4c 89 f3                    	movq	%r14, %rbx
141bf20b3: 85 c0                       	testl	%eax, %eax
141bf20b5: 0f 84 65 fe ff ff           	je	0x141bf1f20 <.text+0x1bf0f20>
141bf20bb: 48 8b 36                    	movq	(%rsi), %rsi
141bf20be: 48 89 f0                    	movq	%rsi, %rax
141bf20c1: 48 c1 e8 20                 	shrq	$0x20, %rax
141bf20c5: 0f 84 13 06 00 00           	je	0x141bf26de <.text+0x1bf16de>
141bf20cb: 89 f1                       	movl	%esi, %ecx
141bf20cd: 48 8b 95 38 01 00 00        	movq	0x138(%rbp), %rdx
141bf20d4: 48 3b 4a 10                 	cmpq	0x10(%rdx), %rcx
141bf20d8: 73 56                       	jae	0x141bf2130 <.text+0x1bf1130>
141bf20da: 48 8b 95 38 01 00 00        	movq	0x138(%rbp), %rdx
141bf20e1: 48 8b 52 08                 	movq	0x8(%rdx), %rdx
141bf20e5: 48 8d 0c 89                 	leaq	(%rcx,%rcx,4), %rcx
141bf20e9: 39 04 8a                    	cmpl	%eax, (%rdx,%rcx,4)
141bf20ec: 75 42                       	jne	0x141bf2130 <.text+0x1bf1130>
141bf20ee: 48 8d 04 8a                 	leaq	(%rdx,%rcx,4), %rax
141bf20f2: 8b 40 04                    	movl	0x4(%rax), %eax
141bf20f5: b9 ff ff ff ff              	movl	$0xffffffff, %ecx       # imm = 0xFFFFFFFF
141bf20fa: 48 39 c8                    	cmpq	%rcx, %rax
141bf20fd: 74 31                       	je	0x141bf2130 <.text+0x1bf1130>
141bf20ff: 48 8b 8d 28 01 00 00        	movq	0x128(%rbp), %rcx
141bf2106: 48 39 41 38                 	cmpq	%rax, 0x38(%rcx)
141bf210a: 76 24                       	jbe	0x141bf2130 <.text+0x1bf1130>
141bf210c: 48 8b 8d 28 01 00 00        	movq	0x128(%rbp), %rcx
141bf2113: 48 8b 49 28                 	movq	0x28(%rcx), %rcx
141bf2117: 89 c2                       	movl	%eax, %edx
141bf2119: c1 ea 06                    	shrl	$0x6, %edx
141bf211c: 48 8b 0c d1                 	movq	(%rcx,%rdx,8), %rcx
141bf2120: 48 0f a3 c1                 	btq	%rax, %rcx
141bf2124: 4c 89 f3                    	movq	%r14, %rbx
141bf2127: 0f 82 f3 fd ff ff           	jb	0x141bf1f20 <.text+0x1bf0f20>
141bf212d: 0f 1f 00                    	nopl	(%rax)
141bf2130: 4d 89 c7                    	movq	%r8, %r15
141bf2133: 48 8b 85 30 01 00 00        	movq	0x130(%rbp), %rax
141bf213a: 48 8b 78 08                 	movq	0x8(%rax), %rdi
141bf213e: 48 85 ff                    	testq	%rdi, %rdi
141bf2141: 75 0b                       	jne	0x141bf214e <.text+0x1bf114e>
141bf2143: 48 8b 85 30 01 00 00        	movq	0x130(%rbp), %rax
141bf214a: 48 8b 78 10                 	movq	0x10(%rax), %rdi
141bf214e: 48 8b 07                    	movq	(%rdi), %rax
141bf2151: 48 8b 5f 10                 	movq	0x10(%rdi), %rbx
141bf2155: 48 29 d8                    	subq	%rbx, %rax
141bf2158: 48 83 f8 1f                 	cmpq	$0x1f, %rax
141bf215c: 0f 86 83 03 00 00           	jbe	0x141bf24e5 <.text+0x1bf14e5>
141bf2162: 48 8b 47 08                 	movq	0x8(%rdi), %rax
141bf2166: 48 8d 0d a3 69 68 00        	leaq	0x6869a3(%rip), %rcx    # 0x142278b10 <.text+0x2277b10>
141bf216d: 48 89 0c 18                 	movq	%rcx, (%rax,%rbx)
141bf2171: 48 8d 0d 88 6d 68 00        	leaq	0x686d88(%rip), %rcx    # 0x142278f00 <.text+0x2277f00>
141bf2178: 48 89 4c 18 08              	movq	%rcx, 0x8(%rax,%rbx)
141bf217d: 48 89 74 18 10              	movq	%rsi, 0x10(%rax,%rbx)
141bf2182: c6 44 18 18 00              	movb	$0x0, 0x18(%rax,%rbx)
141bf2187: 48 83 c3 20                 	addq	$0x20, %rbx
141bf218b: 48 89 5f 10                 	movq	%rbx, 0x10(%rdi)
141bf218f: 4c 89 f3                    	movq	%r14, %rbx
141bf2192: 4c 8b a5 48 01 00 00        	movq	0x148(%rbp), %r12
141bf2199: 4d 89 f8                    	movq	%r15, %r8
141bf219c: e9 7f fd ff ff              	jmp	0x141bf1f20 <.text+0x1bf0f20>
141bf21a1: 4c 89 85 08 01 00 00        	movq	%r8, 0x108(%rbp)
141bf21a8: 48 8d 4d 08                 	leaq	0x8(%rbp), %rcx
141bf21ac: 48 8b 95 30 01 00 00        	movq	0x130(%rbp), %rdx
141bf21b3: e8 e8 24 12 ff              	callq	0x140d146a0 <.text+0xd136a0>
141bf21b8: 48 8b 85 48 01 00 00        	movq	0x148(%rbp), %rax
141bf21bf: 48 8b 50 10                 	movq	0x10(%rax), %rdx
141bf21c3: 48 39 d3                    	cmpq	%rdx, %rbx
141bf21c6: 0f 83 31 05 00 00           	jae	0x141bf26fd <.text+0x1bf16fd>
141bf21cc: 48 8b 7d 10                 	movq	0x10(%rbp), %rdi
141bf21d0: 48 85 ff                    	testq	%rdi, %rdi
141bf21d3: 75 04                       	jne	0x141bf21d9 <.text+0x1bf11d9>
141bf21d5: 48 8b 7d 18                 	movq	0x18(%rbp), %rdi
141bf21d9: 48 8b 40 08                 	movq	0x8(%rax), %rax
141bf21dd: 48 8d 34 9b                 	leaq	(%rbx,%rbx,4), %rsi
141bf21e1: 48 c1 e6 04                 	shlq	$0x4, %rsi
141bf21e5: f3 44 0f 10 4c 30 1c        	movss	0x1c(%rax,%rsi), %xmm9
141bf21ec: 4c 8b 65 28                 	movq	0x28(%rbp), %r12
141bf21f0: 48 8b 07                    	movq	(%rdi), %rax
141bf21f3: 4c 8b 7f 10                 	movq	0x10(%rdi), %r15
141bf21f7: 4c 29 f8                    	subq	%r15, %rax
141bf21fa: 48 83 f8 37                 	cmpq	$0x37, %rax
141bf21fe: 0f 86 06 03 00 00           	jbe	0x141bf250a <.text+0x1bf150a>
141bf2204: 48 8b 47 08                 	movq	0x8(%rdi), %rax
141bf2208: 48 8d 0d f1 67 68 00        	leaq	0x6867f1(%rip), %rcx    # 0x142278a00 <.text+0x2277a00>
141bf220f: 4a 89 0c 38                 	movq	%rcx, (%rax,%r15)
141bf2213: 48 8d 0d e6 6c 68 00        	leaq	0x686ce6(%rip), %rcx    # 0x142278f00 <.text+0x2277f00>
141bf221a: 4a 89 4c 38 08              	movq	%rcx, 0x8(%rax,%r15)
141bf221f: 4d 89 e5                    	movq	%r12, %r13
141bf2222: 4e 89 64 38 10              	movq	%r12, 0x10(%rax,%r15)
141bf2227: 4a 89 5c 38 18              	movq	%rbx, 0x18(%rax,%r15)
141bf222c: f3 46 0f 11 4c 38 20        	movss	%xmm9, 0x20(%rax,%r15)
141bf2233: f3 46 0f 11 44 38 24        	movss	%xmm8, 0x24(%rax,%r15)
141bf223a: f3 42 0f 11 7c 38 28        	movss	%xmm7, 0x28(%rax,%r15)
141bf2241: 42 c6 44 38 30 00           	movb	$0x0, 0x30(%rax,%r15)
141bf2247: 49 83 c7 38                 	addq	$0x38, %r15
141bf224b: 4c 89 7f 10                 	movq	%r15, 0x10(%rdi)
141bf224f: 4c 8b a5 48 01 00 00        	movq	0x148(%rbp), %r12
141bf2256: 49 8b 54 24 10              	movq	0x10(%r12), %rdx
141bf225b: 48 39 d3                    	cmpq	%rdx, %rbx
141bf225e: 0f 83 aa 04 00 00           	jae	0x141bf270e <.text+0x1bf170e>
141bf2264: 49 8b 44 24 08              	movq	0x8(%r12), %rax
141bf2269: f3 0f 10 7c 30 10           	movss	0x10(%rax,%rsi), %xmm7
141bf226f: f3 44 0f 10 4c 30 14        	movss	0x14(%rax,%rsi), %xmm9
141bf2276: f3 44 0f 10 44 30 18        	movss	0x18(%rax,%rsi), %xmm8
141bf227d: 41 0f 2e fc                 	ucomiss	%xmm12, %xmm7
141bf2281: 0f 82 ab 00 00 00           	jb	0x141bf2332 <.text+0x1bf1332>
141bf2287: 0f 2e f7                    	ucomiss	%xmm7, %xmm6
141bf228a: 0f 86 a2 00 00 00           	jbe	0x141bf2332 <.text+0x1bf1332>
141bf2290: 45 0f 2e c4                 	ucomiss	%xmm12, %xmm8
141bf2294: 0f 82 98 00 00 00           	jb	0x141bf2332 <.text+0x1bf1332>
141bf229a: 41 0f 2e f0                 	ucomiss	%xmm8, %xmm6
141bf229e: 0f 86 8e 00 00 00           	jbe	0x141bf2332 <.text+0x1bf1332>
141bf22a4: 48 8b 8d 98 00 00 00        	movq	0x98(%rbp), %rcx
141bf22ab: 0f 28 cf                    	movaps	%xmm7, %xmm1
141bf22ae: 41 0f 28 d0                 	movaps	%xmm8, %xmm2
141bf22b2: e8 f9 4c 65 ff              	callq	0x141246fb0 <.text+0x1245fb0>
141bf22b7: 44 0f 28 c8                 	movaps	%xmm0, %xmm9
141bf22bb: 4d 89 ec                    	movq	%r13, %r12
141bf22be: f3 45 0f 12 fe              	movsldup	%xmm14, %xmm15          # xmm15 = xmm14[0,0,2,2]
141bf22c3: 45 0f 59 fd                 	mulps	%xmm13, %xmm15
141bf22c7: 48 8b 07                    	movq	(%rdi), %rax
141bf22ca: 4c 8b 7f 10                 	movq	0x10(%rdi), %r15
141bf22ce: 4c 29 f8                    	subq	%r15, %rax
141bf22d1: 48 83 f8 27                 	cmpq	$0x27, %rax
141bf22d5: 0f 86 54 02 00 00           	jbe	0x141bf252f <.text+0x1bf152f>
141bf22db: 0f 28 c7                    	movaps	%xmm7, %xmm0
141bf22de: 41 0f 14 c1                 	unpcklps	%xmm9, %xmm0            # xmm0 = xmm0[0],xmm9[0],xmm0[1],xmm9[1]
141bf22e2: 41 0f 58 c7                 	addps	%xmm15, %xmm0
141bf22e6: f3 45 0f 58 f8              	addss	%xmm8, %xmm15
141bf22eb: 48 8b 47 08                 	movq	0x8(%rdi), %rax
141bf22ef: 48 8d 0d 0a 69 68 00        	leaq	0x68690a(%rip), %rcx    # 0x142278c00 <.text+0x2277c00>
141bf22f6: 4a 89 0c 38                 	movq	%rcx, (%rax,%r15)
141bf22fa: 48 8d 0d ff 6b 68 00        	leaq	0x686bff(%rip), %rcx    # 0x142278f00 <.text+0x2277f00>
141bf2301: 4a 89 4c 38 08              	movq	%rcx, 0x8(%rax,%r15)
141bf2306: 4e 89 64 38 10              	movq	%r12, 0x10(%rax,%r15)
141bf230b: 42 0f 13 44 38 18           	movlps	%xmm0, 0x18(%rax,%r15)
141bf2311: f3 46 0f 11 7c 38 20        	movss	%xmm15, 0x20(%rax,%r15)
141bf2318: 42 c6 44 38 24 00           	movb	$0x0, 0x24(%rax,%r15)
141bf231e: 49 83 c7 28                 	addq	$0x28, %r15
141bf2322: 4c 89 7f 10                 	movq	%r15, 0x10(%rdi)
141bf2326: 4c 8b a5 48 01 00 00        	movq	0x148(%rbp), %r12
141bf232d: 49 8b 54 24 10              	movq	0x10(%r12), %rdx
141bf2332: 48 39 d3                    	cmpq	%rdx, %rbx
141bf2335: 0f 83 e4 03 00 00           	jae	0x141bf271f <.text+0x1bf171f>
141bf233b: 49 8b 44 24 08              	movq	0x8(%r12), %rax
141bf2340: 48 8b 0c 30                 	movq	(%rax,%rsi), %rcx
141bf2344: 48 8b 7c 30 08              	movq	0x8(%rax,%rsi), %rdi
141bf2349: f0                          	lock
141bf234a: 48 ff 01                    	incq	(%rcx)
141bf234d: 0f 8e fa 03 00 00           	jle	0x141bf274d <.text+0x1bf174d>
141bf2353: 48 89 8d 40 01 00 00        	movq	%rcx, 0x140(%rbp)
141bf235a: 48 89 8d a8 00 00 00        	movq	%rcx, 0xa8(%rbp)
141bf2361: 48 89 bd b0 00 00 00        	movq	%rdi, 0xb0(%rbp)
141bf2368: f3 0f 10 05 94 35 d3 00     	movss	0xd33594(%rip), %xmm0   # 0x142925904
141bf2370: 0f 28 c8                    	movaps	%xmm0, %xmm1
141bf2373: e8 78 ea 09 ff              	callq	0x140c90df0 <.text+0xc8fdf0>
141bf2378: 49 8b 54 24 10              	movq	0x10(%r12), %rdx
141bf237d: 48 39 d3                    	cmpq	%rdx, %rbx
141bf2380: 0f 83 aa 03 00 00           	jae	0x141bf2730 <.text+0x1bf1730>
141bf2386: 49 8b 4c 24 08              	movq	0x8(%r12), %rcx
141bf238b: 8b 54 31 48                 	movl	0x48(%rcx,%rsi), %edx
141bf238f: f3 0f 10 44 31 20           	movss	0x20(%rcx,%rsi), %xmm0
141bf2395: 44 8b 44 31 24              	movl	0x24(%rcx,%rsi), %r8d
141bf239a: 44 8b 4c 31 2c              	movl	0x2c(%rcx,%rsi), %r9d
141bf239f: 4c 8b 54 31 34              	movq	0x34(%rcx,%rsi), %r10
141bf23a4: 8b 4c 31 3c                 	movl	0x3c(%rcx,%rsi), %ecx
141bf23a8: 44 8b 9d fc 00 00 00        	movl	0xfc(%rbp), %r11d
141bf23af: 4c 8b bd 88 00 00 00        	movq	0x88(%rbp), %r15
141bf23b6: 45 89 1f                    	movl	%r11d, (%r15)
141bf23b9: 4c 8b bd 10 01 00 00        	movq	0x110(%rbp), %r15
141bf23c0: 4d 8b 5f 40                 	movq	0x40(%r15), %r11
141bf23c4: 4c 89 9d f0 00 00 00        	movq	%r11, 0xf0(%rbp)
141bf23cb: 4c 8b 9d 40 01 00 00        	movq	0x140(%rbp), %r11
141bf23d2: 4c 89 9d a8 00 00 00        	movq	%r11, 0xa8(%rbp)
141bf23d9: 48 89 bd b0 00 00 00        	movq	%rdi, 0xb0(%rbp)
141bf23e0: 4c 89 ad b8 00 00 00        	movq	%r13, 0xb8(%rbp)
141bf23e7: f3 0f 11 bd c0 00 00 00     	movss	%xmm7, 0xc0(%rbp)
141bf23ef: f3 44 0f 11 8d c4 00 00 00  	movss	%xmm9, 0xc4(%rbp)
141bf23f8: f3 44 0f 11 85 c8 00 00 00  	movss	%xmm8, 0xc8(%rbp)
141bf2401: f3 44 0f 11 b5 cc 00 00 00  	movss	%xmm14, 0xcc(%rbp)
141bf240a: f3 0f 11 85 d0 00 00 00     	movss	%xmm0, 0xd0(%rbp)
141bf2412: 44 89 85 d4 00 00 00        	movl	%r8d, 0xd4(%rbp)
141bf2419: 89 95 d8 00 00 00           	movl	%edx, 0xd8(%rbp)
141bf241f: 44 89 8d dc 00 00 00        	movl	%r9d, 0xdc(%rbp)
141bf2426: 89 85 e0 00 00 00           	movl	%eax, 0xe0(%rbp)
141bf242c: 4c 89 95 e4 00 00 00        	movq	%r10, 0xe4(%rbp)
141bf2433: 89 8d ec 00 00 00           	movl	%ecx, 0xec(%rbp)
141bf2439: 49 8b 7f 30                 	movq	0x30(%r15), %rdi
141bf243d: 49 3b 7f 20                 	cmpq	0x20(%r15), %rdi
141bf2441: 75 13                       	jne	0x141bf2456 <.text+0x1bf1456>
141bf2443: 48 8b 8d 90 00 00 00        	movq	0x90(%rbp), %rcx
141bf244a: 48 8d 15 c7 58 44 01        	leaq	0x14458c7(%rip), %rdx   # 0x143037d18
141bf2451: e8 ea 10 97 fe              	callq	0x140563540 <.text+0x562540>
141bf2456: 48 8b 95 10 01 00 00        	movq	0x110(%rbp), %rdx
141bf245d: 48 8b 42 28                 	movq	0x28(%rdx), %rax
141bf2461: 48 8d 0c bf                 	leaq	(%rdi,%rdi,4), %rcx
141bf2465: 48 c1 e1 04                 	shlq	$0x4, %rcx
141bf2469: 0f 10 85 a8 00 00 00        	movups	0xa8(%rbp), %xmm0
141bf2470: 0f 10 8d b8 00 00 00        	movups	0xb8(%rbp), %xmm1
141bf2477: 0f 10 95 c8 00 00 00        	movups	0xc8(%rbp), %xmm2
141bf247e: 0f 10 9d d8 00 00 00        	movups	0xd8(%rbp), %xmm3
141bf2485: 0f 10 a5 e8 00 00 00        	movups	0xe8(%rbp), %xmm4
141bf248c: 0f 11 64 08 40              	movups	%xmm4, 0x40(%rax,%rcx)
141bf2491: 0f 11 5c 08 30              	movups	%xmm3, 0x30(%rax,%rcx)
141bf2496: 0f 11 54 08 20              	movups	%xmm2, 0x20(%rax,%rcx)
141bf249b: 0f 11 4c 08 10              	movups	%xmm1, 0x10(%rax,%rcx)
141bf24a0: 0f 11 04 08                 	movups	%xmm0, (%rax,%rcx)
141bf24a4: 48 ff c7                    	incq	%rdi
141bf24a7: 48 89 7a 30                 	movq	%rdi, 0x30(%rdx)
141bf24ab: 48 ff 42 40                 	incq	0x40(%rdx)
141bf24af: 8b 85 18 01 00 00           	movl	0x118(%rbp), %eax
141bf24b5: 48 8b 8d 20 01 00 00        	movq	0x120(%rbp), %rcx
141bf24bc: 89 01                       	movl	%eax, (%rcx)
141bf24be: 49 8b 54 24 10              	movq	0x10(%r12), %rdx
141bf24c3: 48 39 d3                    	cmpq	%rdx, %rbx
141bf24c6: 0f 83 20 02 00 00           	jae	0x141bf26ec <.text+0x1bf16ec>
141bf24cc: 49 8b 44 24 08              	movq	0x8(%r12), %rax
141bf24d1: 4c 89 6c 30 40              	movq	%r13, 0x40(%rax,%rsi)
141bf24d6: 4c 89 f3                    	movq	%r14, %rbx
141bf24d9: 4c 8b 85 08 01 00 00        	movq	0x108(%rbp), %r8
141bf24e0: e9 3b fa ff ff              	jmp	0x141bf1f20 <.text+0x1bf0f20>
141bf24e5: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
141bf24ee: 41 b8 20 00 00 00           	movl	$0x20, %r8d
141bf24f4: 41 b9 01 00 00 00           	movl	$0x1, %r9d
141bf24fa: 48 89 f9                    	movq	%rdi, %rcx
141bf24fd: 48 89 da                    	movq	%rbx, %rdx
141bf2500: e8 7b 81 d2 00              	callq	0x14291a680 <.text+0x2919680>
141bf2505: e9 58 fc ff ff              	jmp	0x141bf2162 <.text+0x1bf1162>
141bf250a: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
141bf2513: 41 b8 38 00 00 00           	movl	$0x38, %r8d
141bf2519: 41 b9 01 00 00 00           	movl	$0x1, %r9d
141bf251f: 48 89 f9                    	movq	%rdi, %rcx
141bf2522: 4c 89 fa                    	movq	%r15, %rdx
141bf2525: e8 56 81 d2 00              	callq	0x14291a680 <.text+0x2919680>
141bf252a: e9 d5 fc ff ff              	jmp	0x141bf2204 <.text+0x1bf1204>
141bf252f: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
141bf2538: 41 b8 28 00 00 00           	movl	$0x28, %r8d
141bf253e: 41 b9 01 00 00 00           	movl	$0x1, %r9d
141bf2544: 48 89 f9                    	movq	%rdi, %rcx
141bf2547: 4c 89 fa                    	movq	%r15, %rdx
141bf254a: e8 31 81 d2 00              	callq	0x14291a680 <.text+0x2919680>
141bf254f: e9 87 fd ff ff              	jmp	0x141bf22db <.text+0x1bf12db>
141bf2554: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
141bf255d: 41 b8 18 00 00 00           	movl	$0x18, %r8d
141bf2563: 41 b9 01 00 00 00           	movl	$0x1, %r9d
141bf2569: 48 89 f9                    	movq	%rdi, %rcx
141bf256c: 48 89 da                    	movq	%rbx, %rdx
141bf256f: e8 0c 81 d2 00              	callq	0x14291a680 <.text+0x2919680>
141bf2574: e9 02 fb ff ff              	jmp	0x141bf207b <.text+0x1bf107b>
141bf2579: 8b 85 18 01 00 00           	movl	0x118(%rbp), %eax
141bf257f: 48 8b 8d 20 01 00 00        	movq	0x120(%rbp), %rcx
141bf2586: 89 01                       	movl	%eax, (%rcx)
141bf2588: 41 c6 44 24 18 01           	movb	$0x1, 0x18(%r12)
141bf258e: 80 bd 57 01 00 00 00        	cmpb	$0x0, 0x157(%rbp)
141bf2595: 74 4b                       	je	0x141bf25e2 <.text+0x1bf15e2>
141bf2597: e8 94 5b 02 ff              	callq	0x140c18130 <.text+0xc17130>
141bf259c: 48 89 c1                    	movq	%rax, %rcx
141bf259f: 48 8b 00                    	movq	(%rax), %rax
141bf25a2: 48 83 f8 01                 	cmpq	$0x1, %rax
141bf25a6: 0f 85 b2 00 00 00           	jne	0x141bf265e <.text+0x1bf165e>
141bf25ac: 48 83 c1 08                 	addq	$0x8, %rcx
141bf25b0: 48 83 39 00                 	cmpq	$0x0, (%rcx)
141bf25b4: 0f 85 f9 00 00 00           	jne	0x141bf26b3 <.text+0x1bf16b3>
141bf25ba: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
141bf25c1: 48 89 8d 48 01 00 00        	movq	%rcx, 0x148(%rbp)
141bf25c8: 48 83 c1 08                 	addq	$0x8, %rcx
141bf25cc: 48 8b 95 a0 00 00 00        	movq	0xa0(%rbp), %rdx
141bf25d3: e8 18 4b 02 ff              	callq	0x140c170f0 <.text+0xc160f0>
141bf25d8: 48 8b 85 48 01 00 00        	movq	0x148(%rbp), %rax
141bf25df: 48 ff 00                    	incq	(%rax)
141bf25e2: 0f 28 b5 60 01 00 00        	movaps	0x160(%rbp), %xmm6
141bf25e9: 0f 28 bd 70 01 00 00        	movaps	0x170(%rbp), %xmm7
141bf25f0: 44 0f 28 85 80 01 00 00     	movaps	0x180(%rbp), %xmm8
141bf25f8: 44 0f 28 8d 90 01 00 00     	movaps	0x190(%rbp), %xmm9
141bf2600: 44 0f 28 95 a0 01 00 00     	movaps	0x1a0(%rbp), %xmm10
141bf2608: 44 0f 28 9d b0 01 00 00     	movaps	0x1b0(%rbp), %xmm11
141bf2610: 44 0f 28 a5 c0 01 00 00     	movaps	0x1c0(%rbp), %xmm12
141bf2618: 44 0f 28 ad d0 01 00 00     	movaps	0x1d0(%rbp), %xmm13
141bf2620: 44 0f 28 b5 e0 01 00 00     	movaps	0x1e0(%rbp), %xmm14
141bf2628: 44 0f 28 bd f0 01 00 00     	movaps	0x1f0(%rbp), %xmm15
141bf2630: 48 81 c4 88 02 00 00        	addq	$0x288, %rsp            # imm = 0x288
141bf2637: 5b                          	popq	%rbx
141bf2638: 5f                          	popq	%rdi
141bf2639: 5e                          	popq	%rsi
141bf263a: 41 5c                       	popq	%r12
141bf263c: 41 5d                       	popq	%r13
141bf263e: 41 5e                       	popq	%r14
141bf2640: 41 5f                       	popq	%r15
141bf2642: 5d                          	popq	%rbp
141bf2643: c3                          	retq
141bf2644: 48 85 c0                    	testq	%rax, %rax
141bf2647: 75 2d                       	jne	0x141bf2676 <.text+0x1bf1676>
141bf2649: 31 d2                       	xorl	%edx, %edx
141bf264b: e8 10 7f d2 00              	callq	0x14291a560 <.text+0x2919560>
141bf2650: 48 89 c1                    	movq	%rax, %rcx
141bf2653: 48 85 c0                    	testq	%rax, %rax
141bf2656: 0f 85 be f2 ff ff           	jne	0x141bf191a <.text+0x1bf091a>
141bf265c: eb 18                       	jmp	0x141bf2676 <.text+0x1bf1676>
141bf265e: 48 85 c0                    	testq	%rax, %rax
141bf2661: 75 13                       	jne	0x141bf2676 <.text+0x1bf1676>
141bf2663: 31 d2                       	xorl	%edx, %edx
141bf2665: e8 f6 7e d2 00              	callq	0x14291a560 <.text+0x2919560>
141bf266a: 48 89 c1                    	movq	%rax, %rcx
141bf266d: 48 85 c0                    	testq	%rax, %rax
141bf2670: 0f 85 3a ff ff ff           	jne	0x141bf25b0 <.text+0x1bf15b0>
141bf2676: 48 8d 0d 03 57 44 01        	leaq	0x1445703(%rip), %rcx   # 0x143037d80
141bf267d: e8 4e 5a ce 00              	callq	0x1428d80d0 <.text+0x28d70d0>
141bf2682: 48 c7 85 a8 00 00 00 00 00 00 00    	movq	$0x0, 0xa8(%rbp)
141bf268d: 48 8d 05 bc 4c 44 01        	leaq	0x1444cbc(%rip), %rax   # 0x143037350
141bf2694: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
141bf2699: 48 8d 55 08                 	leaq	0x8(%rbp), %rdx
141bf269d: 4c 8d 8d a8 00 00 00        	leaq	0xa8(%rbp), %r9
141bf26a4: 31 c9                       	xorl	%ecx, %ecx
141bf26a6: 49 89 d8                    	movq	%rbx, %r8
141bf26a9: e8 93 84 cd 00              	callq	0x1428cab41 <.text+0x28c9b41>
141bf26ae: e9 9a 00 00 00              	jmp	0x141bf274d <.text+0x1bf174d>
141bf26b3: 48 8d 0d 36 57 44 01        	leaq	0x1445736(%rip), %rcx   # 0x143037df0
141bf26ba: e8 91 6b ce 00              	callq	0x1428d9250 <.text+0x28d8250>
141bf26bf: 4c 8d 05 ea b0 30 01        	leaq	0x130b0ea(%rip), %r8    # 0x142efd7b0
141bf26c6: 48 89 d9                    	movq	%rbx, %rcx
141bf26c9: e8 4a 6e ce 00              	callq	0x1428d9518 <.text+0x28d8518>
141bf26ce: eb 7d                       	jmp	0x141bf274d <.text+0x1bf174d>
141bf26d0: 48 8d 0d f1 b0 30 01        	leaq	0x130b0f1(%rip), %rcx   # 0x142efd7c8
141bf26d7: e8 34 6c ce 00              	callq	0x1428d9310 <.text+0x28d8310>
141bf26dc: eb 6f                       	jmp	0x141bf274d <.text+0x1bf174d>
141bf26de: 48 8d 0d fb b0 30 01        	leaq	0x130b0fb(%rip), %rcx   # 0x142efd7e0
141bf26e5: e8 26 6c ce 00              	callq	0x1428d9310 <.text+0x28d8310>
141bf26ea: eb 61                       	jmp	0x141bf274d <.text+0x1bf174d>
141bf26ec: 4c 8d 05 65 b1 30 01        	leaq	0x130b165(%rip), %r8    # 0x142efd858
141bf26f3: 48 89 d9                    	movq	%rbx, %rcx
141bf26f6: e8 1d 6e ce 00              	callq	0x1428d9518 <.text+0x28d8518>
141bf26fb: eb 50                       	jmp	0x141bf274d <.text+0x1bf174d>
141bf26fd: 4c 8d 05 f4 b0 30 01        	leaq	0x130b0f4(%rip), %r8    # 0x142efd7f8
141bf2704: 48 89 d9                    	movq	%rbx, %rcx
141bf2707: e8 0c 6e ce 00              	callq	0x1428d9518 <.text+0x28d8518>
141bf270c: eb 3f                       	jmp	0x141bf274d <.text+0x1bf174d>
141bf270e: 4c 8d 05 fb b0 30 01        	leaq	0x130b0fb(%rip), %r8    # 0x142efd810
141bf2715: 48 89 d9                    	movq	%rbx, %rcx
141bf2718: e8 fb 6d ce 00              	callq	0x1428d9518 <.text+0x28d8518>
141bf271d: eb 2e                       	jmp	0x141bf274d <.text+0x1bf174d>
141bf271f: 4c 8d 05 02 b1 30 01        	leaq	0x130b102(%rip), %r8    # 0x142efd828
141bf2726: 48 89 d9                    	movq	%rbx, %rcx
141bf2729: e8 ea 6d ce 00              	callq	0x1428d9518 <.text+0x28d8518>
141bf272e: eb 1d                       	jmp	0x141bf274d <.text+0x1bf174d>
141bf2730: 4c 8d 05 09 b1 30 01        	leaq	0x130b109(%rip), %r8    # 0x142efd840
141bf2737: 48 89 d9                    	movq	%rbx, %rcx
141bf273a: e8 d9 6d ce 00              	callq	0x1428d9518 <.text+0x28d8518>
141bf273f: eb 0c                       	jmp	0x141bf274d <.text+0x1bf174d>
141bf2741: 48 8d 0d 50 b0 30 01        	leaq	0x130b050(%rip), %rcx   # 0x142efd798
141bf2748: e8 c3 6b ce 00              	callq	0x1428d9310 <.text+0x28d8310>
141bf274d: 0f 0b                       	ud2
141bf274f: 90                          	nop
141bf2750: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141bf2755: 55                          	pushq	%rbp
141bf2756: 41 57                       	pushq	%r15
141bf2758: 41 56                       	pushq	%r14
141bf275a: 41 55                       	pushq	%r13
141bf275c: 41 54                       	pushq	%r12
141bf275e: 56                          	pushq	%rsi
141bf275f: 57                          	pushq	%rdi
141bf2760: 53                          	pushq	%rbx
141bf2761: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
141bf2768: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141bf276f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
141bf2775: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
141bf277b: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
141bf2781: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
141bf2787: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
141bf278d: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
141bf2796: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
141bf279f: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
141bf27a8: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
141bf27b0: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
141bf27b8: 48 8b 85 48 01 00 00        	movq	0x148(%rbp), %rax
141bf27bf: 48 ff 00                    	incq	(%rax)
141bf27c2: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
141bf27ca: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
141bf27d2: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
141bf27db: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
141bf27e4: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
141bf27ed: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
141bf27f3: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
141bf27f9: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
141bf27ff: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
141bf2805: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
141bf280b: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
141bf2812: 5b                          	popq	%rbx
141bf2813: 5f                          	popq	%rdi
141bf2814: 5e                          	popq	%rsi
141bf2815: 41 5c                       	popq	%r12
141bf2817: 41 5d                       	popq	%r13
141bf2819: 41 5e                       	popq	%r14
141bf281b: 41 5f                       	popq	%r15
141bf281d: 5d                          	popq	%rbp
141bf281e: c3                          	retq
141bf281f: 90                          	nop
141bf2820: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141bf2825: 55                          	pushq	%rbp
141bf2826: 41 57                       	pushq	%r15
141bf2828: 41 56                       	pushq	%r14
141bf282a: 41 55                       	pushq	%r13
141bf282c: 41 54                       	pushq	%r12
141bf282e: 56                          	pushq	%rsi
141bf282f: 57                          	pushq	%rdi
141bf2830: 53                          	pushq	%rbx
141bf2831: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
141bf2838: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141bf283f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
141bf2845: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
141bf284b: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
141bf2851: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
141bf2857: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
141bf285d: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
141bf2866: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
141bf286f: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
141bf2878: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
141bf2880: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
141bf2888: 48 83 7d 78 00              	cmpq	$0x0, 0x78(%rbp)
141bf288d: 74 23                       	je	0x141bf28b2 <.text+0x1bf18b2>
141bf288f: 48 8d 85 80 00 00 00        	leaq	0x80(%rbp), %rax
141bf2896: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
141bf289a: 48 8d 45 68                 	leaq	0x68(%rbp), %rax
141bf289e: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
141bf28a2: 48 8d 0d f7 ad 30 01        	leaq	0x130adf7(%rip), %rcx   # 0x142efd6a0
141bf28a9: 48 8d 55 60                 	leaq	0x60(%rbp), %rdx
141bf28ad: e8 9e 67 68 00              	callq	0x142279050 <.text+0x2278050>
141bf28b2: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
141bf28ba: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
141bf28c2: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
141bf28cb: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
141bf28d4: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
141bf28dd: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
141bf28e3: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
141bf28e9: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
141bf28ef: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
141bf28f5: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
141bf28fb: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
141bf2902: 5b                          	popq	%rbx
141bf2903: 5f                          	popq	%rdi
141bf2904: 5e                          	popq	%rsi
141bf2905: 41 5c                       	popq	%r12
141bf2907: 41 5d                       	popq	%r13
141bf2909: 41 5e                       	popq	%r14
141bf290b: 41 5f                       	popq	%r15
141bf290d: 5d                          	popq	%rbp
141bf290e: c3                          	retq
141bf290f: 90                          	nop
141bf2910: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141bf2915: 55                          	pushq	%rbp
141bf2916: 41 57                       	pushq	%r15
141bf2918: 41 56                       	pushq	%r14
141bf291a: 41 55                       	pushq	%r13
141bf291c: 41 54                       	pushq	%r12
141bf291e: 56                          	pushq	%rsi
141bf291f: 57                          	pushq	%rdi
141bf2920: 53                          	pushq	%rbx
141bf2921: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
141bf2928: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141bf292f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
141bf2935: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
141bf293b: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
141bf2941: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
141bf2947: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
141bf294d: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
141bf2956: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
141bf295f: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
141bf2968: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
141bf2970: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
141bf2978: 48 8b 85 48 01 00 00        	movq	0x148(%rbp), %rax
141bf297f: 48 ff 00                    	incq	(%rax)
141bf2982: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
141bf298a: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
141bf2992: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
141bf299b: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
141bf29a4: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
141bf29ad: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
141bf29b3: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
141bf29b9: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
141bf29bf: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
141bf29c5: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
141bf29cb: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
141bf29d2: 5b                          	popq	%rbx
141bf29d3: 5f                          	popq	%rdi
141bf29d4: 5e                          	popq	%rsi
141bf29d5: 41 5c                       	popq	%r12
141bf29d7: 41 5d                       	popq	%r13
141bf29d9: 41 5e                       	popq	%r14
141bf29db: 41 5f                       	popq	%r15
141bf29dd: 5d                          	popq	%rbp
141bf29de: c3                          	retq
141bf29df: 90                          	nop
141bf29e0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141bf29e5: 55                          	pushq	%rbp
141bf29e6: 41 57                       	pushq	%r15
141bf29e8: 41 56                       	pushq	%r14
141bf29ea: 41 55                       	pushq	%r13
141bf29ec: 41 54                       	pushq	%r12
141bf29ee: 56                          	pushq	%rsi
141bf29ef: 57                          	pushq	%rdi
141bf29f0: 53                          	pushq	%rbx
141bf29f1: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
141bf29f8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141bf29ff: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
141bf2a05: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
141bf2a0b: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
141bf2a11: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
141bf2a17: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
141bf2a1d: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
141bf2a26: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
141bf2a2f: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
141bf2a38: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
141bf2a40: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
141bf2a48: 48 8b 85 40 01 00 00        	movq	0x140(%rbp), %rax
141bf2a4f: f0                          	lock
141bf2a50: 48 ff 08                    	decq	(%rax)
141bf2a53: 75 0c                       	jne	0x141bf2a61 <.text+0x1bf1a61>
141bf2a55: 48 8d 8d a8 00 00 00        	leaq	0xa8(%rbp), %rcx
141bf2a5c: e8 cf c0 a5 fe              	callq	0x14064eb30 <.text+0x64db30>
141bf2a61: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
141bf2a69: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
141bf2a71: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
141bf2a7a: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
141bf2a83: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
141bf2a8c: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
141bf2a92: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
141bf2a98: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
141bf2a9e: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
141bf2aa4: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
141bf2aaa: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
141bf2ab1: 5b                          	popq	%rbx
141bf2ab2: 5f                          	popq	%rdi
141bf2ab3: 5e                          	popq	%rsi
141bf2ab4: 41 5c                       	popq	%r12
141bf2ab6: 41 5d                       	popq	%r13
141bf2ab8: 41 5e                       	popq	%r14
141bf2aba: 41 5f                       	popq	%r15
141bf2abc: 5d                          	popq	%rbp
141bf2abd: c3                          	retq
141bf2abe: 66 90                       	nop
141bf2ac0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141bf2ac5: 55                          	pushq	%rbp
141bf2ac6: 41 57                       	pushq	%r15
141bf2ac8: 41 56                       	pushq	%r14
141bf2aca: 41 55                       	pushq	%r13
141bf2acc: 41 54                       	pushq	%r12
141bf2ace: 56                          	pushq	%rsi
141bf2acf: 57                          	pushq	%rdi
141bf2ad0: 53                          	pushq	%rbx
141bf2ad1: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
141bf2ad8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141bf2adf: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
141bf2ae5: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
141bf2aeb: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
141bf2af1: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
141bf2af7: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
141bf2afd: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
141bf2b06: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
141bf2b0f: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
141bf2b18: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
141bf2b20: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
141bf2b28: 48 8b 85 40 01 00 00        	movq	0x140(%rbp), %rax
141bf2b2f: f0                          	lock
141bf2b30: 48 ff 08                    	decq	(%rax)
141bf2b33: 75 0c                       	jne	0x141bf2b41 <.text+0x1bf1b41>
141bf2b35: 48 8d 8d a8 00 00 00        	leaq	0xa8(%rbp), %rcx
141bf2b3c: e8 ef bf a5 fe              	callq	0x14064eb30 <.text+0x64db30>
141bf2b41: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
141bf2b49: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
141bf2b51: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
141bf2b5a: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
141bf2b63: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
141bf2b6c: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
141bf2b72: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
141bf2b78: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
141bf2b7e: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
141bf2b84: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
141bf2b8a: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
141bf2b91: 5b                          	popq	%rbx
141bf2b92: 5f                          	popq	%rdi
141bf2b93: 5e                          	popq	%rsi
141bf2b94: 41 5c                       	popq	%r12
141bf2b96: 41 5d                       	popq	%r13
141bf2b98: 41 5e                       	popq	%r14
141bf2b9a: 41 5f                       	popq	%r15
141bf2b9c: 5d                          	popq	%rbp
141bf2b9d: c3                          	retq
