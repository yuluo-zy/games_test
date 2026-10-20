
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141dabf40: 55                          	pushq	%rbp
141dabf41: 56                          	pushq	%rsi
141dabf42: 57                          	pushq	%rdi
141dabf43: 48 81 ec f0 02 00 00        	subq	$0x2f0, %rsp            # imm = 0x2F0
141dabf4a: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
141dabf52: 48 c7 85 68 02 00 00 fe ff ff ff    	movq	$-0x2, 0x268(%rbp)
141dabf5d: 48 89 ce                    	movq	%rcx, %rsi
141dabf60: 0f 57 c0                    	xorps	%xmm0, %xmm0
141dabf63: 0f 11 45 c0                 	movups	%xmm0, -0x40(%rbp)
141dabf67: 0f 11 45 d8                 	movups	%xmm0, -0x28(%rbp)
141dabf6b: 0f 11 45 f0                 	movups	%xmm0, -0x10(%rbp)
141dabf6f: 0f 11 45 08                 	movups	%xmm0, 0x8(%rbp)
141dabf73: 0f 11 45 20                 	movups	%xmm0, 0x20(%rbp)
141dabf77: c7 45 30 00 00 00 00        	movl	$0x0, 0x30(%rbp)
141dabf7e: 0f 11 45 58                 	movups	%xmm0, 0x58(%rbp)
141dabf82: 0f 11 45 70                 	movups	%xmm0, 0x70(%rbp)
141dabf86: 0f 11 85 88 00 00 00        	movups	%xmm0, 0x88(%rbp)
141dabf8d: 0f 11 85 a0 00 00 00        	movups	%xmm0, 0xa0(%rbp)
141dabf94: 0f 11 85 b8 00 00 00        	movups	%xmm0, 0xb8(%rbp)
141dabf9b: c7 85 c8 00 00 00 00 00 00 00       	movl	$0x0, 0xc8(%rbp)
141dabfa5: 48 c7 45 a0 00 00 00 00     	movq	$0x0, -0x60(%rbp)
141dabfad: 48 c7 45 a8 08 00 00 00     	movq	$0x8, -0x58(%rbp)
141dabfb5: 48 c7 45 b0 00 00 00 00     	movq	$0x0, -0x50(%rbp)
141dabfbd: 48 c7 45 b8 10 00 00 00     	movq	$0x10, -0x48(%rbp)
141dabfc5: 48 c7 45 d0 10 00 00 00     	movq	$0x10, -0x30(%rbp)
141dabfcd: 48 c7 45 e8 10 00 00 00     	movq	$0x10, -0x18(%rbp)
141dabfd5: 48 c7 45 00 10 00 00 00     	movq	$0x10, (%rbp)
141dabfdd: 48 c7 45 18 10 00 00 00     	movq	$0x10, 0x18(%rbp)
141dabfe5: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
141dabfef: 48 89 45 38                 	movq	%rax, 0x38(%rbp)
141dabff3: 48 8d 0d 17 27 1c 01        	leaq	0x11c2717(%rip), %rcx   # 0x142f6e711
141dabffa: 48 89 4d 40                 	movq	%rcx, 0x40(%rbp)
141dabffe: 48 c7 45 48 26 00 00 00     	movq	$0x26, 0x48(%rbp)
141dac006: 48 c7 45 50 10 00 00 00     	movq	$0x10, 0x50(%rbp)
141dac00e: 48 c7 45 68 10 00 00 00     	movq	$0x10, 0x68(%rbp)
141dac016: 48 c7 85 80 00 00 00 10 00 00 00    	movq	$0x10, 0x80(%rbp)
141dac021: 48 c7 85 98 00 00 00 10 00 00 00    	movq	$0x10, 0x98(%rbp)
141dac02c: 48 c7 85 b0 00 00 00 10 00 00 00    	movq	$0x10, 0xb0(%rbp)
141dac037: c7 85 d0 00 00 00 00 00 00 00       	movl	$0x0, 0xd0(%rbp)
141dac041: 66 c7 85 d4 00 00 00 01 00  	movw	$0x1, 0xd4(%rbp)
141dac04a: 48 89 85 18 01 00 00        	movq	%rax, 0x118(%rbp)
141dac051: c7 85 60 02 00 00 00 00 00 00       	movl	$0x0, 0x260(%rbp)
141dac05b: 0f b6 05 a7 8b 88 01        	movzbl	0x1888ba7(%rip), %eax   # 0x143634c09
141dac062: b9 c8 02 00 00              	movl	$0x2c8, %ecx            # imm = 0x2C8
141dac067: ba 08 00 00 00              	movl	$0x8, %edx
141dac06c: e8 9f 7b 86 fe              	callq	0x140613c10 <.text+0x612c10>
141dac071: 48 85 c0                    	testq	%rax, %rax
141dac074: 74 35                       	je	0x141dac0ab <.text+0x1dab0ab>
141dac076: 48 89 c7                    	movq	%rax, %rdi
141dac079: 48 8d 55 a0                 	leaq	-0x60(%rbp), %rdx
141dac07d: 41 b8 c8 02 00 00           	movl	$0x2c8, %r8d            # imm = 0x2C8
141dac083: 48 89 c1                    	movq	%rax, %rcx
141dac086: e8 75 7b b7 00              	callq	0x142923c00 <.text+0x2922c00>
141dac08b: 4c 8d 05 46 25 1c 01        	leaq	0x11c2546(%rip), %r8    # 0x142f6e5d8
141dac092: 48 89 f1                    	movq	%rsi, %rcx
141dac095: 48 89 fa                    	movq	%rdi, %rdx
141dac098: e8 73 8a f3 fe              	callq	0x140ce4b10 <.text+0xce3b10>
141dac09d: 48 89 f0                    	movq	%rsi, %rax
141dac0a0: 48 81 c4 f0 02 00 00        	addq	$0x2f0, %rsp            # imm = 0x2F0
141dac0a7: 5f                          	popq	%rdi
141dac0a8: 5e                          	popq	%rsi
141dac0a9: 5d                          	popq	%rbp
141dac0aa: c3                          	retq
141dac0ab: b9 08 00 00 00              	movl	$0x8, %ecx
141dac0b0: ba c8 02 00 00              	movl	$0x2c8, %edx            # imm = 0x2C8
141dac0b5: e8 46 cf b2 00              	callq	0x1428d9000 <.text+0x28d8000>
141dac0ba: 0f 0b                       	ud2
141dac0bc: 0f 1f 40 00                 	nopl	(%rax)
141dac0c0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141dac0c5: 55                          	pushq	%rbp
141dac0c6: 56                          	pushq	%rsi
141dac0c7: 57                          	pushq	%rdi
141dac0c8: 48 83 ec 20                 	subq	$0x20, %rsp
141dac0cc: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141dac0d3: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
141dac0d7: e8 04 b1 ac fe              	callq	0x1408771e0 <.text+0x8761e0>
141dac0dc: 90                          	nop
141dac0dd: 48 83 c4 20                 	addq	$0x20, %rsp
141dac0e1: 5f                          	popq	%rdi
141dac0e2: 5e                          	popq	%rsi
141dac0e3: 5d                          	popq	%rbp
141dac0e4: c3                          	retq
