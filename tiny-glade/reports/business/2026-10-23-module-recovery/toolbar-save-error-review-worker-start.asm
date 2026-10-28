
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141720680: 55                          	pushq	%rbp
141720681: 41 57                       	pushq	%r15
141720683: 41 56                       	pushq	%r14
141720685: 41 55                       	pushq	%r13
141720687: 41 54                       	pushq	%r12
141720689: 56                          	pushq	%rsi
14172068a: 57                          	pushq	%rdi
14172068b: 53                          	pushq	%rbx
14172068c: 48 81 ec 28 07 00 00        	subq	$0x728, %rsp            # imm = 0x728
141720693: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
14172069b: 0f 29 b5 90 06 00 00        	movaps	%xmm6, 0x690(%rbp)
1417206a2: 48 c7 85 88 06 00 00 fe ff ff ff    	movq	$-0x2, 0x688(%rbp)
1417206ad: 48 89 8d 10 06 00 00        	movq	%rcx, 0x610(%rbp)
1417206b4: 48 89 95 18 06 00 00        	movq	%rdx, 0x618(%rbp)
1417206bb: 4c 89 85 40 05 00 00        	movq	%r8, 0x540(%rbp)
1417206c2: 4c 89 8d 48 05 00 00        	movq	%r9, 0x548(%rbp)
1417206c9: 48 c7 45 a8 03 00 00 00     	movq	$0x3, -0x58(%rbp)
1417206d1: 31 c9                       	xorl	%ecx, %ecx
1417206d3: e8 48 94 9b fe              	callq	0x1400d9b20 <.text+0xd8b20>
1417206d8: 48 85 c0                    	testq	%rax, %rax
1417206db: 0f 84 04 0e 00 00           	je	0x1417214e5 <.text+0x17204e5>
1417206e1: 0f 10 00                    	movups	(%rax), %xmm0
1417206e4: 48 ff 00                    	incq	(%rax)
1417206e7: 0f 29 85 20 05 00 00        	movaps	%xmm0, 0x520(%rbp)
1417206ee: 0f 10 05 0b b8 54 01        	movups	0x154b80b(%rip), %xmm0  # 0x142c6bf00
1417206f5: 0f 29 85 00 05 00 00        	movaps	%xmm0, 0x500(%rbp)
1417206fc: 0f 10 05 0d b8 54 01        	movups	0x154b80d(%rip), %xmm0  # 0x142c6bf10
141720703: 0f 29 85 10 05 00 00        	movaps	%xmm0, 0x510(%rbp)
14172070a: e8 d1 2e 64 ff              	callq	0x140d635e0 <.text+0xd625e0>
14172070f: 48 89 85 20 06 00 00        	movq	%rax, 0x620(%rbp)
141720716: 89 95 54 06 00 00           	movl	%edx, 0x654(%rbp)
14172071c: 48 c7 85 30 06 00 00 00 00 00 00    	movq	$0x0, 0x630(%rbp)
141720727: 48 c7 85 38 06 00 00 08 00 00 00    	movq	$0x8, 0x638(%rbp)
141720732: 48 c7 85 40 06 00 00 00 00 00 00    	movq	$0x0, 0x640(%rbp)
14172073d: 48 8d 7d b8                 	leaq	-0x48(%rbp), %rdi
141720741: 41 bf ff 00 00 00           	movl	$0xff, %r15d
141720747: 4c 8d ad 60 06 00 00        	leaq	0x660(%rbp), %r13
14172074e: 0f 57 f6                    	xorps	%xmm6, %xmm6
141720751: 4c 8d 35 7c ac 5a 01        	leaq	0x15aac7c(%rip), %r14   # 0x142ccb3d4
141720758: eb 67                       	jmp	0x1417207c1 <.text+0x171f7c1>
14172075a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
141720760: 0f 28 85 90 05 00 00        	movaps	0x590(%rbp), %xmm0
141720767: 0f 28 8d a0 05 00 00        	movaps	0x5a0(%rbp), %xmm1
14172076e: 0f 28 95 b0 05 00 00        	movaps	0x5b0(%rbp), %xmm2
141720775: 0f 28 9d c0 05 00 00        	movaps	0x5c0(%rbp), %xmm3
14172077c: 0f 29 9d 80 02 00 00        	movaps	%xmm3, 0x280(%rbp)
141720783: 0f 29 95 70 02 00 00        	movaps	%xmm2, 0x270(%rbp)
14172078a: 0f 29 8d 60 02 00 00        	movaps	%xmm1, 0x260(%rbp)
141720791: 0f 29 85 50 02 00 00        	movaps	%xmm0, 0x250(%rbp)
141720798: 4c 8b 8d 10 07 00 00        	movq	0x710(%rbp), %r9
14172079f: 49 83 c1 10                 	addq	$0x10, %r9
1417207a3: c6 85 86 06 00 00 00        	movb	$0x0, 0x686(%rbp)
1417207aa: 48 8d 8d 50 02 00 00        	leaq	0x250(%rbp), %rcx
1417207b1: 48 8d 55 a8                 	leaq	-0x58(%rbp), %rdx
1417207b5: 4c 8d 85 00 05 00 00        	leaq	0x500(%rbp), %r8
1417207bc: e8 df 12 00 00              	callq	0x141721aa0 <.text+0x1720aa0>
