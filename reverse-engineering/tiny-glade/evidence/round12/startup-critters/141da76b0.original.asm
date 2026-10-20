
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141da76b0: 55                          	pushq	%rbp
141da76b1: 56                          	pushq	%rsi
141da76b2: 57                          	pushq	%rdi
141da76b3: 48 81 ec c0 03 00 00        	subq	$0x3c0, %rsp            # imm = 0x3C0
141da76ba: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
141da76c2: 48 c7 85 38 03 00 00 fe ff ff ff    	movq	$-0x2, 0x338(%rbp)
141da76cd: 48 89 ce                    	movq	%rcx, %rsi
141da76d0: 0f 57 c0                    	xorps	%xmm0, %xmm0
141da76d3: 0f 11 45 c0                 	movups	%xmm0, -0x40(%rbp)
141da76d7: 0f 11 45 d8                 	movups	%xmm0, -0x28(%rbp)
141da76db: 0f 11 45 f0                 	movups	%xmm0, -0x10(%rbp)
141da76df: 0f 11 45 08                 	movups	%xmm0, 0x8(%rbp)
141da76e3: 0f 11 45 20                 	movups	%xmm0, 0x20(%rbp)
141da76e7: c7 45 30 00 00 00 00        	movl	$0x0, 0x30(%rbp)
141da76ee: 0f 11 45 58                 	movups	%xmm0, 0x58(%rbp)
141da76f2: 0f 11 45 70                 	movups	%xmm0, 0x70(%rbp)
141da76f6: 0f 11 85 88 00 00 00        	movups	%xmm0, 0x88(%rbp)
141da76fd: 0f 11 85 a0 00 00 00        	movups	%xmm0, 0xa0(%rbp)
141da7704: 0f 11 85 b8 00 00 00        	movups	%xmm0, 0xb8(%rbp)
141da770b: c7 85 c8 00 00 00 00 00 00 00       	movl	$0x0, 0xc8(%rbp)
141da7715: 48 c7 45 a0 00 00 00 00     	movq	$0x0, -0x60(%rbp)
141da771d: 48 c7 45 a8 08 00 00 00     	movq	$0x8, -0x58(%rbp)
141da7725: 48 c7 45 b0 00 00 00 00     	movq	$0x0, -0x50(%rbp)
141da772d: 48 c7 45 b8 10 00 00 00     	movq	$0x10, -0x48(%rbp)
141da7735: 48 c7 45 d0 10 00 00 00     	movq	$0x10, -0x30(%rbp)
141da773d: 48 c7 45 e8 10 00 00 00     	movq	$0x10, -0x18(%rbp)
141da7745: 48 c7 45 00 10 00 00 00     	movq	$0x10, (%rbp)
141da774d: 48 c7 45 18 10 00 00 00     	movq	$0x10, 0x18(%rbp)
141da7755: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
141da775f: 48 89 45 38                 	movq	%rax, 0x38(%rbp)
141da7763: 48 8d 0d 2b 5c 1c 01        	leaq	0x11c5c2b(%rip), %rcx   # 0x142f6d395
141da776a: 48 89 4d 40                 	movq	%rcx, 0x40(%rbp)
141da776e: 48 c7 45 48 5d 00 00 00     	movq	$0x5d, 0x48(%rbp)
141da7776: 48 c7 45 50 10 00 00 00     	movq	$0x10, 0x50(%rbp)
141da777e: 48 c7 45 68 10 00 00 00     	movq	$0x10, 0x68(%rbp)
141da7786: 48 c7 85 80 00 00 00 10 00 00 00    	movq	$0x10, 0x80(%rbp)
141da7791: 48 c7 85 98 00 00 00 10 00 00 00    	movq	$0x10, 0x98(%rbp)
141da779c: 48 c7 85 b0 00 00 00 10 00 00 00    	movq	$0x10, 0xb0(%rbp)
141da77a7: c7 85 d0 00 00 00 00 00 00 00       	movl	$0x0, 0xd0(%rbp)
141da77b1: 66 c7 85 d4 00 00 00 01 00  	movw	$0x1, 0xd4(%rbp)
141da77ba: 48 89 85 c8 01 00 00        	movq	%rax, 0x1c8(%rbp)
141da77c1: c7 85 30 03 00 00 00 00 00 00       	movl	$0x0, 0x330(%rbp)
141da77cb: 0f b6 05 37 d4 88 01        	movzbl	0x188d437(%rip), %eax   # 0x143634c09
141da77d2: b9 98 03 00 00              	movl	$0x398, %ecx            # imm = 0x398
141da77d7: ba 08 00 00 00              	movl	$0x8, %edx
141da77dc: e8 2f c4 86 fe              	callq	0x140613c10 <.text+0x612c10>
141da77e1: 48 85 c0                    	testq	%rax, %rax
141da77e4: 74 35                       	je	0x141da781b <.text+0x1da681b>
141da77e6: 48 89 c7                    	movq	%rax, %rdi
141da77e9: 48 8d 55 a0                 	leaq	-0x60(%rbp), %rdx
141da77ed: 41 b8 98 03 00 00           	movl	$0x398, %r8d            # imm = 0x398
141da77f3: 48 89 c1                    	movq	%rax, %rcx
141da77f6: e8 05 c4 b7 00              	callq	0x142923c00 <.text+0x2922c00>
141da77fb: 4c 8d 05 be 5a 1c 01        	leaq	0x11c5abe(%rip), %r8    # 0x142f6d2c0
141da7802: 48 89 f1                    	movq	%rsi, %rcx
141da7805: 48 89 fa                    	movq	%rdi, %rdx
141da7808: e8 03 d3 f3 fe              	callq	0x140ce4b10 <.text+0xce3b10>
141da780d: 48 89 f0                    	movq	%rsi, %rax
141da7810: 48 81 c4 c0 03 00 00        	addq	$0x3c0, %rsp            # imm = 0x3C0
141da7817: 5f                          	popq	%rdi
141da7818: 5e                          	popq	%rsi
141da7819: 5d                          	popq	%rbp
141da781a: c3                          	retq
141da781b: b9 08 00 00 00              	movl	$0x8, %ecx
141da7820: ba 98 03 00 00              	movl	$0x398, %edx            # imm = 0x398
141da7825: e8 d6 17 b3 00              	callq	0x1428d9000 <.text+0x28d8000>
141da782a: 0f 0b                       	ud2
141da782c: 0f 1f 40 00                 	nopl	(%rax)
141da7830: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141da7835: 55                          	pushq	%rbp
141da7836: 56                          	pushq	%rsi
141da7837: 57                          	pushq	%rdi
141da7838: 48 83 ec 20                 	subq	$0x20, %rsp
141da783c: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141da7843: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
141da7847: e8 64 be fd ff              	callq	0x141d836b0 <.text+0x1d826b0>
141da784c: 90                          	nop
141da784d: 48 83 c4 20                 	addq	$0x20, %rsp
141da7851: 5f                          	popq	%rdi
141da7852: 5e                          	popq	%rsi
141da7853: 5d                          	popq	%rbp
141da7854: c3                          	retq
