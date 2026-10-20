
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141dac8f0: 55                          	pushq	%rbp
141dac8f1: 56                          	pushq	%rsi
141dac8f2: 57                          	pushq	%rdi
141dac8f3: 48 81 ec c0 02 00 00        	subq	$0x2c0, %rsp            # imm = 0x2C0
141dac8fa: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
141dac902: 48 c7 85 38 02 00 00 fe ff ff ff    	movq	$-0x2, 0x238(%rbp)
141dac90d: 48 89 ce                    	movq	%rcx, %rsi
141dac910: 0f 57 c0                    	xorps	%xmm0, %xmm0
141dac913: 0f 11 45 c8                 	movups	%xmm0, -0x38(%rbp)
141dac917: 0f 11 45 e0                 	movups	%xmm0, -0x20(%rbp)
141dac91b: 0f 11 45 f8                 	movups	%xmm0, -0x8(%rbp)
141dac91f: 0f 11 45 10                 	movups	%xmm0, 0x10(%rbp)
141dac923: 0f 11 45 28                 	movups	%xmm0, 0x28(%rbp)
141dac927: c7 45 38 00 00 00 00        	movl	$0x0, 0x38(%rbp)
141dac92e: 0f 11 45 60                 	movups	%xmm0, 0x60(%rbp)
141dac932: 0f 11 45 78                 	movups	%xmm0, 0x78(%rbp)
141dac936: 0f 11 85 90 00 00 00        	movups	%xmm0, 0x90(%rbp)
141dac93d: 0f 11 85 a8 00 00 00        	movups	%xmm0, 0xa8(%rbp)
141dac944: 0f 11 85 c0 00 00 00        	movups	%xmm0, 0xc0(%rbp)
141dac94b: c7 85 d0 00 00 00 00 00 00 00       	movl	$0x0, 0xd0(%rbp)
141dac955: 48 c7 45 a8 00 00 00 00     	movq	$0x0, -0x58(%rbp)
141dac95d: 48 c7 45 b0 08 00 00 00     	movq	$0x8, -0x50(%rbp)
141dac965: 48 c7 45 b8 00 00 00 00     	movq	$0x0, -0x48(%rbp)
141dac96d: 48 c7 45 c0 10 00 00 00     	movq	$0x10, -0x40(%rbp)
141dac975: 48 c7 45 d8 10 00 00 00     	movq	$0x10, -0x28(%rbp)
141dac97d: 48 c7 45 f0 10 00 00 00     	movq	$0x10, -0x10(%rbp)
141dac985: 48 c7 45 08 10 00 00 00     	movq	$0x10, 0x8(%rbp)
141dac98d: 48 c7 45 20 10 00 00 00     	movq	$0x10, 0x20(%rbp)
141dac995: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
141dac99f: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
141dac9a3: 48 8d 0d 96 21 1c 01        	leaq	0x11c2196(%rip), %rcx   # 0x142f6eb40
141dac9aa: 48 89 4d 48                 	movq	%rcx, 0x48(%rbp)
141dac9ae: 48 c7 45 50 3c 00 00 00     	movq	$0x3c, 0x50(%rbp)
141dac9b6: 48 c7 45 58 10 00 00 00     	movq	$0x10, 0x58(%rbp)
141dac9be: 48 c7 45 70 10 00 00 00     	movq	$0x10, 0x70(%rbp)
141dac9c6: 48 c7 85 88 00 00 00 10 00 00 00    	movq	$0x10, 0x88(%rbp)
141dac9d1: 48 c7 85 a0 00 00 00 10 00 00 00    	movq	$0x10, 0xa0(%rbp)
141dac9dc: 48 c7 85 b8 00 00 00 10 00 00 00    	movq	$0x10, 0xb8(%rbp)
141dac9e7: c7 85 d8 00 00 00 00 00 00 00       	movl	$0x0, 0xd8(%rbp)
141dac9f1: 66 c7 85 dc 00 00 00 01 00  	movw	$0x1, 0xdc(%rbp)
141dac9fa: 48 89 85 d8 01 00 00        	movq	%rax, 0x1d8(%rbp)
141daca01: c7 85 30 02 00 00 00 00 00 00       	movl	$0x0, 0x230(%rbp)
141daca0b: 0f b6 05 f7 81 88 01        	movzbl	0x18881f7(%rip), %eax   # 0x143634c09
141daca12: b9 90 02 00 00              	movl	$0x290, %ecx            # imm = 0x290
141daca17: ba 08 00 00 00              	movl	$0x8, %edx
141daca1c: e8 ef 71 86 fe              	callq	0x140613c10 <.text+0x612c10>
141daca21: 48 85 c0                    	testq	%rax, %rax
141daca24: 74 35                       	je	0x141daca5b <.text+0x1daba5b>
141daca26: 48 89 c7                    	movq	%rax, %rdi
141daca29: 48 8d 55 a8                 	leaq	-0x58(%rbp), %rdx
141daca2d: 41 b8 90 02 00 00           	movl	$0x290, %r8d            # imm = 0x290
141daca33: 48 89 c1                    	movq	%rax, %rcx
141daca36: e8 c5 71 b7 00              	callq	0x142923c00 <.text+0x2922c00>
141daca3b: 4c 8d 05 46 20 1c 01        	leaq	0x11c2046(%rip), %r8    # 0x142f6ea88
141daca42: 48 89 f1                    	movq	%rsi, %rcx
141daca45: 48 89 fa                    	movq	%rdi, %rdx
141daca48: e8 c3 80 f3 fe              	callq	0x140ce4b10 <.text+0xce3b10>
141daca4d: 48 89 f0                    	movq	%rsi, %rax
141daca50: 48 81 c4 c0 02 00 00        	addq	$0x2c0, %rsp            # imm = 0x2C0
141daca57: 5f                          	popq	%rdi
141daca58: 5e                          	popq	%rsi
141daca59: 5d                          	popq	%rbp
141daca5a: c3                          	retq
141daca5b: b9 08 00 00 00              	movl	$0x8, %ecx
141daca60: ba 90 02 00 00              	movl	$0x290, %edx            # imm = 0x290
141daca65: e8 96 c5 b2 00              	callq	0x1428d9000 <.text+0x28d8000>
141daca6a: 0f 0b                       	ud2
141daca6c: 0f 1f 40 00                 	nopl	(%rax)
141daca70: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141daca75: 55                          	pushq	%rbp
141daca76: 56                          	pushq	%rsi
141daca77: 57                          	pushq	%rdi
141daca78: 48 83 ec 20                 	subq	$0x20, %rsp
141daca7c: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141daca83: 48 8d 4d a8                 	leaq	-0x58(%rbp), %rcx
141daca87: e8 24 29 82 fe              	callq	0x1405cf3b0 <.text+0x5ce3b0>
141daca8c: 90                          	nop
141daca8d: 48 83 c4 20                 	addq	$0x20, %rsp
141daca91: 5f                          	popq	%rdi
141daca92: 5e                          	popq	%rsi
141daca93: 5d                          	popq	%rbp
141daca94: c3                          	retq
