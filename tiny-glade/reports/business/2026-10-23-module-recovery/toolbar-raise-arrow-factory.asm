
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1409553f0: 55                          	pushq	%rbp
1409553f1: 41 57                       	pushq	%r15
1409553f3: 41 56                       	pushq	%r14
1409553f5: 56                          	pushq	%rsi
1409553f6: 57                          	pushq	%rdi
1409553f7: 53                          	pushq	%rbx
1409553f8: 48 81 ec 08 01 00 00        	subq	$0x108, %rsp            # imm = 0x108
1409553ff: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140955407: 48 c7 85 80 00 00 00 fe ff ff ff    	movq	$-0x2, 0x80(%rbp)
140955412: 4c 89 cf                    	movq	%r9, %rdi
140955415: 4c 89 c3                    	movq	%r8, %rbx
140955418: 49 89 d6                    	movq	%rdx, %r14
14095541b: 48 89 ce                    	movq	%rcx, %rsi
14095541e: 0f b6 05 e4 f7 cd 02        	movzbl	0x2cdf7e4(%rip), %eax   # 0x143634c09
140955425: b9 08 00 00 00              	movl	$0x8, %ecx
14095542a: ba 08 00 00 00              	movl	$0x8, %edx
14095542f: e8 dc e7 cb ff              	callq	0x140613c10 <.text+0x612c10>
140955434: 48 85 c0                    	testq	%rax, %rax
140955437: 0f 84 58 01 00 00           	je	0x140955595 <.text+0x954595>
14095543d: 49 89 c7                    	movq	%rax, %r15
140955440: 0f b6 05 c2 f7 cd 02        	movzbl	0x2cdf7c2(%rip), %eax   # 0x143634c09
140955447: b9 10 00 00 00              	movl	$0x10, %ecx
14095544c: ba 01 00 00 00              	movl	$0x1, %edx
140955451: e8 ba e7 cb ff              	callq	0x140613c10 <.text+0x612c10>
140955456: 48 85 c0                    	testq	%rax, %rax
140955459: 4c 89 7d 78                 	movq	%r15, 0x78(%rbp)
14095545d: 0f 84 41 01 00 00           	je	0x1409555a4 <.text+0x9545a4>
140955463: 0f 10 05 66 f2 15 02        	movups	0x215f266(%rip), %xmm0  # 0x142ab46d0
14095546a: 0f 11 00                    	movups	%xmm0, (%rax)
14095546d: 48 c7 45 50 10 00 00 00     	movq	$0x10, 0x50(%rbp)
140955475: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
140955479: 48 c7 45 60 10 00 00 00     	movq	$0x10, 0x60(%rbp)
140955481: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14095548b: 48 89 45 48                 	movq	%rax, 0x48(%rbp)
14095548f: 0f b6 05 73 f7 cd 02        	movzbl	0x2cdf773(%rip), %eax   # 0x143634c09
140955496: b9 13 00 00 00              	movl	$0x13, %ecx
14095549b: ba 01 00 00 00              	movl	$0x1, %edx
1409554a0: e8 6b e7 cb ff              	callq	0x140613c10 <.text+0x612c10>
1409554a5: 48 85 c0                    	testq	%rax, %rax
1409554a8: 0f 84 0e 01 00 00           	je	0x1409555bc <.text+0x9545bc>
1409554ae: 48 8b 8d e0 00 00 00        	movq	0xe0(%rbp), %rcx
1409554b5: 8b 15 e0 3f 14 02           	movl	0x2143fe0(%rip), %edx   # 0x142a9949b
1409554bb: 89 50 0f                    	movl	%edx, 0xf(%rax)
1409554be: 0f 10 05 c7 3f 14 02        	movups	0x2143fc7(%rip), %xmm0  # 0x142a9948c
1409554c5: 0f 11 00                    	movups	%xmm0, (%rax)
1409554c8: 0f 10 45 48                 	movups	0x48(%rbp), %xmm0
1409554cc: 0f 10 4d 58                 	movups	0x58(%rbp), %xmm1
1409554d0: 0f 10 55 68                 	movups	0x68(%rbp), %xmm2
1409554d4: 0f 29 45 c0                 	movaps	%xmm0, -0x40(%rbp)
1409554d8: 0f 29 4d d0                 	movaps	%xmm1, -0x30(%rbp)
1409554dc: 0f 29 55 e0                 	movaps	%xmm2, -0x20(%rbp)
1409554e0: 41 0f 28 06                 	movaps	(%r14), %xmm0
1409554e4: 41 8b 56 18                 	movl	0x18(%r14), %edx
1409554e8: 89 55 50                    	movl	%edx, 0x50(%rbp)
1409554eb: 49 8b 56 10                 	movq	0x10(%r14), %rdx
1409554ef: 48 89 55 48                 	movq	%rdx, 0x48(%rbp)
1409554f3: 0f 29 45 b0                 	movaps	%xmm0, -0x50(%rbp)
1409554f7: 66 c7 45 f0 00 01           	movw	$0x100, -0x10(%rbp)     # imm = 0x100
1409554fd: 48 ba 00 00 80 3f 01 00 00 00       	movabsq	$0x13f800000, %rdx # imm = 0x13F800000
140955507: 48 89 55 f4                 	movq	%rdx, -0xc(%rbp)
14095550b: c7 45 00 00 00 00 00        	movl	$0x0, (%rbp)
140955512: c7 45 08 00 00 00 00        	movl	$0x0, 0x8(%rbp)
140955519: 48 c7 45 10 13 00 00 00     	movq	$0x13, 0x10(%rbp)
140955521: 48 89 45 18                 	movq	%rax, 0x18(%rbp)
140955525: 48 c7 45 20 13 00 00 00     	movq	$0x13, 0x20(%rbp)
14095552d: c7 45 28 00 00 00 00        	movl	$0x0, 0x28(%rbp)
140955534: 48 8b 45 48                 	movq	0x48(%rbp), %rax
140955538: 48 89 45 2c                 	movq	%rax, 0x2c(%rbp)
14095553c: 8b 45 50                    	movl	0x50(%rbp), %eax
14095553f: 89 45 34                    	movl	%eax, 0x34(%rbp)
140955542: 66 c7 45 38 00 00           	movw	$0x0, 0x38(%rbp)
140955548: c6 45 3a 00                 	movb	$0x0, 0x3a(%rbp)
14095554c: 48 89 4c 24 20              	movq	%rcx, 0x20(%rsp)
140955551: 48 8d 4d 48                 	leaq	0x48(%rbp), %rcx
140955555: 48 8d 55 b0                 	leaq	-0x50(%rbp), %rdx
140955559: 49 89 d8                    	movq	%rbx, %r8
14095555c: 49 89 f9                    	movq	%rdi, %r9
14095555f: e8 ec 87 f4 ff              	callq	0x14089dd50 <.text+0x89cd50>
140955564: 48 8b 45 68                 	movq	0x68(%rbp), %rax
140955568: 48 8b 4d 78                 	movq	0x78(%rbp), %rcx
14095556c: 48 89 01                    	movq	%rax, (%rcx)
14095556f: 48 c7 06 01 00 00 00        	movq	$0x1, (%rsi)
140955576: 48 89 4e 08                 	movq	%rcx, 0x8(%rsi)
14095557a: 48 c7 46 10 01 00 00 00     	movq	$0x1, 0x10(%rsi)
140955582: 48 89 f0                    	movq	%rsi, %rax
140955585: 48 81 c4 08 01 00 00        	addq	$0x108, %rsp            # imm = 0x108
14095558c: 5b                          	popq	%rbx
14095558d: 5f                          	popq	%rdi
14095558e: 5e                          	popq	%rsi
14095558f: 41 5e                       	popq	%r14
140955591: 41 5f                       	popq	%r15
140955593: 5d                          	popq	%rbp
140955594: c3                          	retq
140955595: b9 08 00 00 00              	movl	$0x8, %ecx
14095559a: ba 08 00 00 00              	movl	$0x8, %edx
14095559f: e8 5c 3a f8 01              	callq	0x1428d9000 <.text+0x28d8000>
1409555a4: 4c 8d 05 1d e8 15 02        	leaq	0x215e81d(%rip), %r8    # 0x142ab3dc8
1409555ab: b9 01 00 00 00              	movl	$0x1, %ecx
1409555b0: ba 10 00 00 00              	movl	$0x10, %edx
1409555b5: e8 29 3a f8 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
1409555ba: eb 16                       	jmp	0x1409555d2 <.text+0x9545d2>
1409555bc: 4c 8d 05 5d 39 14 02        	leaq	0x214395d(%rip), %r8    # 0x142a98f20
1409555c3: b9 01 00 00 00              	movl	$0x1, %ecx
1409555c8: ba 13 00 00 00              	movl	$0x13, %edx
1409555cd: e8 11 3a f8 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
