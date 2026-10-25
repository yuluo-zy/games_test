
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140f7cf20: 55                          	pushq	%rbp
140f7cf21: 56                          	pushq	%rsi
140f7cf22: 57                          	pushq	%rdi
140f7cf23: 53                          	pushq	%rbx
140f7cf24: 48 81 ec 08 04 00 00        	subq	$0x408, %rsp            # imm = 0x408
140f7cf2b: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140f7cf33: 48 c7 85 80 03 00 00 fe ff ff ff    	movq	$-0x2, 0x380(%rbp)
140f7cf3e: 48 89 ce                    	movq	%rcx, %rsi
140f7cf41: 0f b6 05 c1 7c 6b 02        	movzbl	0x26b7cc1(%rip), %eax   # 0x143634c09
140f7cf48: b9 c0 01 00 00              	movl	$0x1c0, %ecx            # imm = 0x1C0
140f7cf4d: ba 08 00 00 00              	movl	$0x8, %edx
140f7cf52: e8 b9 6c 69 ff              	callq	0x140613c10 <.text+0x612c10>
140f7cf57: 48 85 c0                    	testq	%rax, %rax
140f7cf5a: 0f 84 0b 03 00 00           	je	0x140f7d26b <.text+0xf7c26b>
140f7cf60: 48 89 85 78 03 00 00        	movq	%rax, 0x378(%rbp)
140f7cf67: 0f 57 c0                    	xorps	%xmm0, %xmm0
140f7cf6a: 0f 11 45 c8                 	movups	%xmm0, -0x38(%rbp)
140f7cf6e: 0f 11 45 e0                 	movups	%xmm0, -0x20(%rbp)
140f7cf72: 0f 11 45 f8                 	movups	%xmm0, -0x8(%rbp)
140f7cf76: 0f 11 45 10                 	movups	%xmm0, 0x10(%rbp)
140f7cf7a: 0f 11 45 28                 	movups	%xmm0, 0x28(%rbp)
140f7cf7e: c7 45 38 00 00 00 00        	movl	$0x0, 0x38(%rbp)
140f7cf85: 0f 11 45 60                 	movups	%xmm0, 0x60(%rbp)
140f7cf89: 0f 11 45 78                 	movups	%xmm0, 0x78(%rbp)
140f7cf8d: 0f 11 85 90 00 00 00        	movups	%xmm0, 0x90(%rbp)
140f7cf94: 0f 11 85 a8 00 00 00        	movups	%xmm0, 0xa8(%rbp)
140f7cf9b: 0f 11 85 c0 00 00 00        	movups	%xmm0, 0xc0(%rbp)
140f7cfa2: c7 85 d0 00 00 00 00 00 00 00       	movl	$0x0, 0xd0(%rbp)
140f7cfac: 48 c7 45 a8 00 00 00 00     	movq	$0x0, -0x58(%rbp)
140f7cfb4: 48 c7 45 b0 08 00 00 00     	movq	$0x8, -0x50(%rbp)
140f7cfbc: 48 c7 45 b8 00 00 00 00     	movq	$0x0, -0x48(%rbp)
140f7cfc4: 48 c7 45 c0 10 00 00 00     	movq	$0x10, -0x40(%rbp)
140f7cfcc: 48 c7 45 d8 10 00 00 00     	movq	$0x10, -0x28(%rbp)
140f7cfd4: 48 c7 45 f0 10 00 00 00     	movq	$0x10, -0x10(%rbp)
140f7cfdc: 48 c7 45 08 10 00 00 00     	movq	$0x10, 0x8(%rbp)
140f7cfe4: 48 c7 45 20 10 00 00 00     	movq	$0x10, 0x20(%rbp)
140f7cfec: 48 bb 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rbx # imm = 0x8000000000000000
140f7cff6: 48 89 5d 40                 	movq	%rbx, 0x40(%rbp)
140f7cffa: 48 8d 05 c1 f1 c2 01        	leaq	0x1c2f1c1(%rip), %rax   # 0x142bac1c2
140f7d001: 48 89 45 48                 	movq	%rax, 0x48(%rbp)
140f7d005: 48 c7 45 50 42 00 00 00     	movq	$0x42, 0x50(%rbp)
140f7d00d: 48 c7 45 58 10 00 00 00     	movq	$0x10, 0x58(%rbp)
140f7d015: 48 c7 45 70 10 00 00 00     	movq	$0x10, 0x70(%rbp)
140f7d01d: 48 c7 85 88 00 00 00 10 00 00 00    	movq	$0x10, 0x88(%rbp)
140f7d028: 48 c7 85 a0 00 00 00 10 00 00 00    	movq	$0x10, 0xa0(%rbp)
140f7d033: 48 c7 85 b8 00 00 00 10 00 00 00    	movq	$0x10, 0xb8(%rbp)
140f7d03e: c7 85 d8 00 00 00 00 00 00 00       	movl	$0x0, 0xd8(%rbp)
140f7d048: 66 c7 85 dc 00 00 00 01 00  	movw	$0x1, 0xdc(%rbp)
140f7d051: 48 89 9d 20 01 00 00        	movq	%rbx, 0x120(%rbp)
140f7d058: c7 85 20 02 00 00 00 00 00 00       	movl	$0x0, 0x220(%rbp)
140f7d062: 0f b6 05 a0 7b 6b 02        	movzbl	0x26b7ba0(%rip), %eax   # 0x143634c09
140f7d069: b9 80 02 00 00              	movl	$0x280, %ecx            # imm = 0x280
140f7d06e: ba 08 00 00 00              	movl	$0x8, %edx
140f7d073: e8 98 6b 69 ff              	callq	0x140613c10 <.text+0x612c10>
140f7d078: 48 85 c0                    	testq	%rax, %rax
140f7d07b: 0f 84 f9 01 00 00           	je	0x140f7d27a <.text+0xf7c27a>
140f7d081: 48 89 c7                    	movq	%rax, %rdi
140f7d084: 48 8d 55 a8                 	leaq	-0x58(%rbp), %rdx
140f7d088: 41 b8 80 02 00 00           	movl	$0x280, %r8d            # imm = 0x280
140f7d08e: 48 89 c1                    	movq	%rax, %rcx
140f7d091: e8 6a 6b 9a 01              	callq	0x142923c00 <.text+0x2922c00>
140f7d096: 4c 8d 05 eb ee c2 01        	leaq	0x1c2eeeb(%rip), %r8    # 0x142babf88
140f7d09d: 48 8d 8d 08 03 00 00        	leaq	0x308(%rbp), %rcx
140f7d0a4: 48 89 fa                    	movq	%rdi, %rdx
140f7d0a7: e8 64 7a d6 ff              	callq	0x140ce4b10 <.text+0xce3b10>
140f7d0ac: 48 8d 8d 98 02 00 00        	leaq	0x298(%rbp), %rcx
140f7d0b3: e8 e8 46 e1 00              	callq	0x141d917a0 <.text+0x1d907a0>
140f7d0b8: 48 8d 8d 28 02 00 00        	leaq	0x228(%rbp), %rcx
140f7d0bf: e8 1c f3 e0 00              	callq	0x141d8c3e0 <.text+0x1d8b3e0>
140f7d0c4: 48 8d 4d a8                 	leaq	-0x58(%rbp), %rcx
140f7d0c8: e8 93 27 e1 00              	callq	0x141d8f860 <.text+0x1d8e860>
140f7d0cd: 0f 10 85 68 03 00 00        	movups	0x368(%rbp), %xmm0
140f7d0d4: 48 8b 85 78 03 00 00        	movq	0x378(%rbp), %rax
140f7d0db: 0f 11 40 60                 	movups	%xmm0, 0x60(%rax)
140f7d0df: 0f 10 85 58 03 00 00        	movups	0x358(%rbp), %xmm0
140f7d0e6: 0f 11 40 50                 	movups	%xmm0, 0x50(%rax)
140f7d0ea: 0f 10 85 48 03 00 00        	movups	0x348(%rbp), %xmm0
140f7d0f1: 0f 11 40 40                 	movups	%xmm0, 0x40(%rax)
140f7d0f5: 0f 10 85 08 03 00 00        	movups	0x308(%rbp), %xmm0
140f7d0fc: 0f 10 8d 18 03 00 00        	movups	0x318(%rbp), %xmm1
140f7d103: 0f 10 95 28 03 00 00        	movups	0x328(%rbp), %xmm2
140f7d10a: 0f 10 9d 38 03 00 00        	movups	0x338(%rbp), %xmm3
140f7d111: 0f 11 58 30                 	movups	%xmm3, 0x30(%rax)
140f7d115: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
140f7d119: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
140f7d11d: 0f 11 00                    	movups	%xmm0, (%rax)
140f7d120: 0f 10 85 98 02 00 00        	movups	0x298(%rbp), %xmm0
140f7d127: 0f 10 8d a8 02 00 00        	movups	0x2a8(%rbp), %xmm1
140f7d12e: 0f 10 95 b8 02 00 00        	movups	0x2b8(%rbp), %xmm2
140f7d135: 0f 10 9d c8 02 00 00        	movups	0x2c8(%rbp), %xmm3
140f7d13c: 0f 11 40 70                 	movups	%xmm0, 0x70(%rax)
140f7d140: 0f 11 88 80 00 00 00        	movups	%xmm1, 0x80(%rax)
140f7d147: 0f 11 90 90 00 00 00        	movups	%xmm2, 0x90(%rax)
140f7d14e: 0f 11 98 a0 00 00 00        	movups	%xmm3, 0xa0(%rax)
140f7d155: 0f 10 85 d8 02 00 00        	movups	0x2d8(%rbp), %xmm0
140f7d15c: 0f 11 80 b0 00 00 00        	movups	%xmm0, 0xb0(%rax)
140f7d163: 0f 10 85 e8 02 00 00        	movups	0x2e8(%rbp), %xmm0
140f7d16a: 0f 11 80 c0 00 00 00        	movups	%xmm0, 0xc0(%rax)
140f7d171: 0f 10 85 f8 02 00 00        	movups	0x2f8(%rbp), %xmm0
140f7d178: 0f 11 80 d0 00 00 00        	movups	%xmm0, 0xd0(%rax)
140f7d17f: 0f 10 85 88 02 00 00        	movups	0x288(%rbp), %xmm0
140f7d186: 0f 11 80 40 01 00 00        	movups	%xmm0, 0x140(%rax)
140f7d18d: 0f 10 85 78 02 00 00        	movups	0x278(%rbp), %xmm0
140f7d194: 0f 11 80 30 01 00 00        	movups	%xmm0, 0x130(%rax)
140f7d19b: 0f 10 85 68 02 00 00        	movups	0x268(%rbp), %xmm0
140f7d1a2: 0f 11 80 20 01 00 00        	movups	%xmm0, 0x120(%rax)
140f7d1a9: 0f 10 85 28 02 00 00        	movups	0x228(%rbp), %xmm0
140f7d1b0: 0f 10 8d 38 02 00 00        	movups	0x238(%rbp), %xmm1
140f7d1b7: 0f 10 95 48 02 00 00        	movups	0x248(%rbp), %xmm2
140f7d1be: 0f 10 9d 58 02 00 00        	movups	0x258(%rbp), %xmm3
140f7d1c5: 0f 11 98 10 01 00 00        	movups	%xmm3, 0x110(%rax)
140f7d1cc: 0f 11 90 00 01 00 00        	movups	%xmm2, 0x100(%rax)
140f7d1d3: 0f 11 88 f0 00 00 00        	movups	%xmm1, 0xf0(%rax)
140f7d1da: 0f 11 80 e0 00 00 00        	movups	%xmm0, 0xe0(%rax)
140f7d1e1: 0f 10 45 a8                 	movups	-0x58(%rbp), %xmm0
140f7d1e5: 0f 10 4d b8                 	movups	-0x48(%rbp), %xmm1
140f7d1e9: 0f 10 55 c8                 	movups	-0x38(%rbp), %xmm2
140f7d1ed: 0f 10 5d d8                 	movups	-0x28(%rbp), %xmm3
140f7d1f1: 0f 11 80 50 01 00 00        	movups	%xmm0, 0x150(%rax)
140f7d1f8: 0f 11 88 60 01 00 00        	movups	%xmm1, 0x160(%rax)
140f7d1ff: 0f 11 90 70 01 00 00        	movups	%xmm2, 0x170(%rax)
140f7d206: 0f 11 98 80 01 00 00        	movups	%xmm3, 0x180(%rax)
140f7d20d: 0f 10 45 e8                 	movups	-0x18(%rbp), %xmm0
140f7d211: 0f 11 80 90 01 00 00        	movups	%xmm0, 0x190(%rax)
140f7d218: 0f 10 45 f8                 	movups	-0x8(%rbp), %xmm0
140f7d21c: 0f 11 80 a0 01 00 00        	movups	%xmm0, 0x1a0(%rax)
140f7d223: 0f 10 45 08                 	movups	0x8(%rbp), %xmm0
140f7d227: 0f 11 80 b0 01 00 00        	movups	%xmm0, 0x1b0(%rax)
140f7d22e: 48 c7 46 08 04 00 00 00     	movq	$0x4, 0x8(%rsi)
140f7d236: 48 89 46 10                 	movq	%rax, 0x10(%rsi)
140f7d23a: 48 c7 46 18 04 00 00 00     	movq	$0x4, 0x18(%rsi)
140f7d242: 48 c7 46 20 00 00 00 00     	movq	$0x0, 0x20(%rsi)
140f7d24a: 48 c7 46 28 08 00 00 00     	movq	$0x8, 0x28(%rsi)
140f7d252: 0f 57 c0                    	xorps	%xmm0, %xmm0
140f7d255: 0f 11 46 30                 	movups	%xmm0, 0x30(%rsi)
140f7d259: 48 89 1e                    	movq	%rbx, (%rsi)
140f7d25c: 48 89 f0                    	movq	%rsi, %rax
140f7d25f: 48 81 c4 08 04 00 00        	addq	$0x408, %rsp            # imm = 0x408
140f7d266: 5b                          	popq	%rbx
140f7d267: 5f                          	popq	%rdi
140f7d268: 5e                          	popq	%rsi
140f7d269: 5d                          	popq	%rbp
140f7d26a: c3                          	retq
140f7d26b: b9 08 00 00 00              	movl	$0x8, %ecx
140f7d270: ba c0 01 00 00              	movl	$0x1c0, %edx            # imm = 0x1C0
140f7d275: e8 86 bd 95 01              	callq	0x1428d9000 <.text+0x28d8000>
140f7d27a: b9 08 00 00 00              	movl	$0x8, %ecx
140f7d27f: ba 80 02 00 00              	movl	$0x280, %edx            # imm = 0x280
140f7d284: e8 77 bd 95 01              	callq	0x1428d9000 <.text+0x28d8000>
140f7d289: 0f 0b                       	ud2
140f7d28b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
140f7d290: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140f7d295: 55                          	pushq	%rbp
140f7d296: 56                          	pushq	%rsi
140f7d297: 57                          	pushq	%rdi
140f7d298: 53                          	pushq	%rbx
140f7d299: 48 83 ec 28                 	subq	$0x28, %rsp
140f7d29d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140f7d2a4: 48 8d 4d a8                 	leaq	-0x58(%rbp), %rcx
140f7d2a8: e8 33 9f 8f ff              	callq	0x1408771e0 <.text+0x8761e0>
140f7d2ad: 90                          	nop
140f7d2ae: 48 83 c4 28                 	addq	$0x28, %rsp
140f7d2b2: 5b                          	popq	%rbx
140f7d2b3: 5f                          	popq	%rdi
140f7d2b4: 5e                          	popq	%rsi
140f7d2b5: 5d                          	popq	%rbp
140f7d2b6: c3                          	retq
140f7d2b7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
140f7d2c0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140f7d2c5: 55                          	pushq	%rbp
140f7d2c6: 56                          	pushq	%rsi
140f7d2c7: 57                          	pushq	%rdi
140f7d2c8: 53                          	pushq	%rbx
140f7d2c9: 48 83 ec 28                 	subq	$0x28, %rsp
140f7d2cd: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140f7d2d4: ba c0 01 00 00              	movl	$0x1c0, %edx            # imm = 0x1C0
140f7d2d9: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140f7d2df: 48 8b 8d 78 03 00 00        	movq	0x378(%rbp), %rcx
140f7d2e6: e8 35 69 69 ff              	callq	0x140613c20 <.text+0x612c20>
140f7d2eb: 90                          	nop
140f7d2ec: 48 83 c4 28                 	addq	$0x28, %rsp
140f7d2f0: 5b                          	popq	%rbx
140f7d2f1: 5f                          	popq	%rdi
140f7d2f2: 5e                          	popq	%rsi
140f7d2f3: 5d                          	popq	%rbp
140f7d2f4: c3                          	retq
140f7d2f5: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
140f7d300: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140f7d305: 55                          	pushq	%rbp
140f7d306: 56                          	pushq	%rsi
140f7d307: 57                          	pushq	%rdi
140f7d308: 53                          	pushq	%rbx
140f7d309: 48 83 ec 28                 	subq	$0x28, %rsp
140f7d30d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140f7d314: 48 8d 8d 08 03 00 00        	leaq	0x308(%rbp), %rcx
140f7d31b: e8 60 c5 0a ff              	callq	0x140029880 <.text+0x28880>
140f7d320: 90                          	nop
140f7d321: 48 83 c4 28                 	addq	$0x28, %rsp
140f7d325: 5b                          	popq	%rbx
140f7d326: 5f                          	popq	%rdi
140f7d327: 5e                          	popq	%rsi
140f7d328: 5d                          	popq	%rbp
140f7d329: c3                          	retq
140f7d32a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140f7d330: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140f7d335: 55                          	pushq	%rbp
140f7d336: 56                          	pushq	%rsi
140f7d337: 57                          	pushq	%rdi
140f7d338: 53                          	pushq	%rbx
140f7d339: 48 83 ec 28                 	subq	$0x28, %rsp
140f7d33d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140f7d344: 48 8d 8d 98 02 00 00        	leaq	0x298(%rbp), %rcx
140f7d34b: e8 30 c5 0a ff              	callq	0x140029880 <.text+0x28880>
140f7d350: 90                          	nop
140f7d351: 48 83 c4 28                 	addq	$0x28, %rsp
140f7d355: 5b                          	popq	%rbx
140f7d356: 5f                          	popq	%rdi
140f7d357: 5e                          	popq	%rsi
140f7d358: 5d                          	popq	%rbp
140f7d359: c3                          	retq
140f7d35a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140f7d360: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140f7d365: 55                          	pushq	%rbp
140f7d366: 56                          	pushq	%rsi
140f7d367: 57                          	pushq	%rdi
140f7d368: 53                          	pushq	%rbx
140f7d369: 48 83 ec 28                 	subq	$0x28, %rsp
140f7d36d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140f7d374: 48 8d 8d 28 02 00 00        	leaq	0x228(%rbp), %rcx
140f7d37b: e8 00 c5 0a ff              	callq	0x140029880 <.text+0x28880>
140f7d380: 90                          	nop
140f7d381: 48 83 c4 28                 	addq	$0x28, %rsp
140f7d385: 5b                          	popq	%rbx
140f7d386: 5f                          	popq	%rdi
140f7d387: 5e                          	popq	%rsi
140f7d388: 5d                          	popq	%rbp
140f7d389: c3                          	retq
