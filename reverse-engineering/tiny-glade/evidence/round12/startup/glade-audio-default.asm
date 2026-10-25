
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1408bf2c0: 55                          	pushq	%rbp
1408bf2c1: 56                          	pushq	%rsi
1408bf2c2: 57                          	pushq	%rdi
1408bf2c3: 48 83 ec 50                 	subq	$0x50, %rsp
1408bf2c7: 48 8d 6c 24 50              	leaq	0x50(%rsp), %rbp
1408bf2cc: 48 c7 45 f8 fe ff ff ff     	movq	$-0x2, -0x8(%rbp)
1408bf2d4: 48 89 ce                    	movq	%rcx, %rsi
1408bf2d7: 0f b6 05 2b 59 d7 02        	movzbl	0x2d7592b(%rip), %eax   # 0x143634c09
1408bf2de: b9 18 00 00 00              	movl	$0x18, %ecx
1408bf2e3: ba 08 00 00 00              	movl	$0x8, %edx
1408bf2e8: e8 23 49 d5 ff              	callq	0x140613c10 <.text+0x612c10>
1408bf2ed: 48 85 c0                    	testq	%rax, %rax
1408bf2f0: 0f 84 c6 00 00 00           	je	0x1408bf3bc <.text+0x8be3bc>
1408bf2f6: 48 89 c7                    	movq	%rax, %rdi
1408bf2f9: 0f b6 05 09 59 d7 02        	movzbl	0x2d75909(%rip), %eax   # 0x143634c09
1408bf300: b9 19 00 00 00              	movl	$0x19, %ecx
1408bf305: ba 01 00 00 00              	movl	$0x1, %edx
1408bf30a: e8 01 49 d5 ff              	callq	0x140613c10 <.text+0x612c10>
1408bf30f: 48 85 c0                    	testq	%rax, %rax
1408bf312: 0f 84 b3 00 00 00           	je	0x1408bf3cb <.text+0x8be3cb>
1408bf318: 0f 10 05 02 1a 1e 02        	movups	0x21e1a02(%rip), %xmm0  # 0x142aa0d21
1408bf31f: 0f 11 40 09                 	movups	%xmm0, 0x9(%rax)
1408bf323: 0f 10 05 ee 19 1e 02        	movups	0x21e19ee(%rip), %xmm0  # 0x142aa0d18
1408bf32a: 0f 11 00                    	movups	%xmm0, (%rax)
1408bf32d: 48 c7 07 19 00 00 00        	movq	$0x19, (%rdi)
1408bf334: 48 89 47 08                 	movq	%rax, 0x8(%rdi)
1408bf338: 48 c7 47 10 19 00 00 00     	movq	$0x19, 0x10(%rdi)
1408bf340: 48 c7 45 d8 01 00 00 00     	movq	$0x1, -0x28(%rbp)
1408bf348: 48 89 7d e0                 	movq	%rdi, -0x20(%rbp)
1408bf34c: 48 c7 45 e8 01 00 00 00     	movq	$0x1, -0x18(%rbp)
1408bf354: 0f b6 05 ae 58 d7 02        	movzbl	0x2d758ae(%rip), %eax   # 0x143634c09
1408bf35b: b9 13 00 00 00              	movl	$0x13, %ecx
1408bf360: ba 01 00 00 00              	movl	$0x1, %edx
1408bf365: e8 a6 48 d5 ff              	callq	0x140613c10 <.text+0x612c10>
1408bf36a: 48 85 c0                    	testq	%rax, %rax
1408bf36d: 74 78                       	je	0x1408bf3e7 <.text+0x8be3e7>
1408bf36f: 0f 10 05 bb 19 1e 02        	movups	0x21e19bb(%rip), %xmm0  # 0x142aa0d31
1408bf376: 0f 11 00                    	movups	%xmm0, (%rax)
1408bf379: c7 40 0f 65 6e 63 65        	movl	$0x65636e65, 0xf(%rax)  # imm = 0x65636E65
1408bf380: 0f 10 45 d8                 	movups	-0x28(%rbp), %xmm0
1408bf384: 0f 11 06                    	movups	%xmm0, (%rsi)
1408bf387: 48 8b 4d e8                 	movq	-0x18(%rbp), %rcx
1408bf38b: 48 89 4e 10                 	movq	%rcx, 0x10(%rsi)
1408bf38f: 48 c7 46 18 13 00 00 00     	movq	$0x13, 0x18(%rsi)
1408bf397: 48 89 46 20                 	movq	%rax, 0x20(%rsi)
1408bf39b: 48 c7 46 28 13 00 00 00     	movq	$0x13, 0x28(%rsi)
1408bf3a3: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1408bf3ad: 48 89 46 30                 	movq	%rax, 0x30(%rsi)
1408bf3b1: 48 89 f0                    	movq	%rsi, %rax
1408bf3b4: 48 83 c4 50                 	addq	$0x50, %rsp
1408bf3b8: 5f                          	popq	%rdi
1408bf3b9: 5e                          	popq	%rsi
1408bf3ba: 5d                          	popq	%rbp
1408bf3bb: c3                          	retq
1408bf3bc: b9 08 00 00 00              	movl	$0x8, %ecx
1408bf3c1: ba 18 00 00 00              	movl	$0x18, %edx
1408bf3c6: e8 35 9c 01 02              	callq	0x1428d9000 <.text+0x28d8000>
1408bf3cb: 48 89 7d f0                 	movq	%rdi, -0x10(%rbp)
1408bf3cf: 4c 8d 05 4a 11 1e 02        	leaq	0x21e114a(%rip), %r8    # 0x142aa0520
1408bf3d6: b9 01 00 00 00              	movl	$0x1, %ecx
1408bf3db: ba 19 00 00 00              	movl	$0x19, %edx
1408bf3e0: e8 fe 9b 01 02              	callq	0x1428d8fe3 <.text+0x28d7fe3>
1408bf3e5: eb 16                       	jmp	0x1408bf3fd <.text+0x8be3fd>
1408bf3e7: 4c 8d 05 32 11 1e 02        	leaq	0x21e1132(%rip), %r8    # 0x142aa0520
1408bf3ee: b9 01 00 00 00              	movl	$0x1, %ecx
1408bf3f3: ba 13 00 00 00              	movl	$0x13, %edx
1408bf3f8: e8 e6 9b 01 02              	callq	0x1428d8fe3 <.text+0x28d7fe3>
1408bf3fd: 0f 0b                       	ud2
1408bf3ff: 90                          	nop
1408bf400: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1408bf405: 55                          	pushq	%rbp
1408bf406: 56                          	pushq	%rsi
1408bf407: 57                          	pushq	%rdi
1408bf408: 48 83 ec 20                 	subq	$0x20, %rsp
1408bf40c: 48 8d 6a 50                 	leaq	0x50(%rdx), %rbp
1408bf410: ba 18 00 00 00              	movl	$0x18, %edx
1408bf415: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1408bf41b: 48 8b 4d f0                 	movq	-0x10(%rbp), %rcx
1408bf41f: e8 fc 47 d5 ff              	callq	0x140613c20 <.text+0x612c20>
1408bf424: 90                          	nop
1408bf425: 48 83 c4 20                 	addq	$0x20, %rsp
1408bf429: 5f                          	popq	%rdi
1408bf42a: 5e                          	popq	%rsi
1408bf42b: 5d                          	popq	%rbp
1408bf42c: c3                          	retq
1408bf42d: 0f 1f 00                    	nopl	(%rax)
1408bf430: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1408bf435: 55                          	pushq	%rbp
1408bf436: 56                          	pushq	%rsi
1408bf437: 57                          	pushq	%rdi
1408bf438: 48 83 ec 20                 	subq	$0x20, %rsp
1408bf43c: 48 8d 6a 50                 	leaq	0x50(%rdx), %rbp
1408bf440: 48 8d 4d d8                 	leaq	-0x28(%rbp), %rcx
1408bf444: e8 27 9b d3 ff              	callq	0x1405f8f70 <.text+0x5f7f70>
1408bf449: 90                          	nop
1408bf44a: 48 83 c4 20                 	addq	$0x20, %rsp
1408bf44e: 5f                          	popq	%rdi
1408bf44f: 5e                          	popq	%rsi
1408bf450: 5d                          	popq	%rbp
1408bf451: c3                          	retq
