
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141dadcc0: 55                          	pushq	%rbp
141dadcc1: 56                          	pushq	%rsi
141dadcc2: 57                          	pushq	%rdi
141dadcc3: 48 81 ec 10 04 00 00        	subq	$0x410, %rsp            # imm = 0x410
141dadcca: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
141dadcd2: 48 c7 85 88 03 00 00 fe ff ff ff    	movq	$-0x2, 0x388(%rbp)
141dadcdd: 48 89 ce                    	movq	%rcx, %rsi
141dadce0: 0f 57 c0                    	xorps	%xmm0, %xmm0
141dadce3: 0f 11 45 c0                 	movups	%xmm0, -0x40(%rbp)
141dadce7: 0f 11 45 d8                 	movups	%xmm0, -0x28(%rbp)
141dadceb: 0f 11 45 f0                 	movups	%xmm0, -0x10(%rbp)
141dadcef: 0f 11 45 08                 	movups	%xmm0, 0x8(%rbp)
141dadcf3: 0f 11 45 20                 	movups	%xmm0, 0x20(%rbp)
141dadcf7: c7 45 30 00 00 00 00        	movl	$0x0, 0x30(%rbp)
141dadcfe: 0f 11 45 58                 	movups	%xmm0, 0x58(%rbp)
141dadd02: 0f 11 45 70                 	movups	%xmm0, 0x70(%rbp)
141dadd06: 0f 11 85 88 00 00 00        	movups	%xmm0, 0x88(%rbp)
141dadd0d: 0f 11 85 a0 00 00 00        	movups	%xmm0, 0xa0(%rbp)
141dadd14: 0f 11 85 b8 00 00 00        	movups	%xmm0, 0xb8(%rbp)
141dadd1b: c7 85 c8 00 00 00 00 00 00 00       	movl	$0x0, 0xc8(%rbp)
141dadd25: 48 c7 45 a0 00 00 00 00     	movq	$0x0, -0x60(%rbp)
141dadd2d: 48 c7 45 a8 08 00 00 00     	movq	$0x8, -0x58(%rbp)
141dadd35: 48 c7 45 b0 00 00 00 00     	movq	$0x0, -0x50(%rbp)
141dadd3d: 48 c7 45 b8 10 00 00 00     	movq	$0x10, -0x48(%rbp)
141dadd45: 48 c7 45 d0 10 00 00 00     	movq	$0x10, -0x30(%rbp)
141dadd4d: 48 c7 45 e8 10 00 00 00     	movq	$0x10, -0x18(%rbp)
141dadd55: 48 c7 45 00 10 00 00 00     	movq	$0x10, (%rbp)
141dadd5d: 48 c7 45 18 10 00 00 00     	movq	$0x10, 0x18(%rbp)
141dadd65: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
141dadd6f: 48 89 45 38                 	movq	%rax, 0x38(%rbp)
141dadd73: 48 8d 0d 5e 15 1c 01        	leaq	0x11c155e(%rip), %rcx   # 0x142f6f2d8
141dadd7a: 48 89 4d 40                 	movq	%rcx, 0x40(%rbp)
141dadd7e: 48 c7 45 48 35 00 00 00     	movq	$0x35, 0x48(%rbp)
141dadd86: 48 c7 45 50 10 00 00 00     	movq	$0x10, 0x50(%rbp)
141dadd8e: 48 c7 45 68 10 00 00 00     	movq	$0x10, 0x68(%rbp)
141dadd96: 48 c7 85 80 00 00 00 10 00 00 00    	movq	$0x10, 0x80(%rbp)
141dadda1: 48 c7 85 98 00 00 00 10 00 00 00    	movq	$0x10, 0x98(%rbp)
141daddac: 48 c7 85 b0 00 00 00 10 00 00 00    	movq	$0x10, 0xb0(%rbp)
141daddb7: c7 85 d0 00 00 00 00 00 00 00       	movl	$0x0, 0xd0(%rbp)
141daddc1: 66 c7 85 d4 00 00 00 01 00  	movw	$0x1, 0xd4(%rbp)
141daddca: 48 89 85 d8 00 00 00        	movq	%rax, 0xd8(%rbp)
141daddd1: c7 85 80 03 00 00 00 00 00 00       	movl	$0x0, 0x380(%rbp)
141dadddb: 0f b6 05 27 6e 88 01        	movzbl	0x1886e27(%rip), %eax   # 0x143634c09
141dadde2: b9 e8 03 00 00              	movl	$0x3e8, %ecx            # imm = 0x3E8
141dadde7: ba 08 00 00 00              	movl	$0x8, %edx
141daddec: e8 1f 5e 86 fe              	callq	0x140613c10 <.text+0x612c10>
141daddf1: 48 85 c0                    	testq	%rax, %rax
141daddf4: 74 35                       	je	0x141dade2b <.text+0x1dace2b>
141daddf6: 48 89 c7                    	movq	%rax, %rdi
141daddf9: 48 8d 55 a0                 	leaq	-0x60(%rbp), %rdx
141daddfd: 41 b8 e8 03 00 00           	movl	$0x3e8, %r8d            # imm = 0x3E8
141dade03: 48 89 c1                    	movq	%rax, %rcx
141dade06: e8 f5 5d b7 00              	callq	0x142923c00 <.text+0x2922c00>
141dade0b: 4c 8d 05 0e 14 1c 01        	leaq	0x11c140e(%rip), %r8    # 0x142f6f220
141dade12: 48 89 f1                    	movq	%rsi, %rcx
141dade15: 48 89 fa                    	movq	%rdi, %rdx
141dade18: e8 f3 6c f3 fe              	callq	0x140ce4b10 <.text+0xce3b10>
141dade1d: 48 89 f0                    	movq	%rsi, %rax
141dade20: 48 81 c4 10 04 00 00        	addq	$0x410, %rsp            # imm = 0x410
141dade27: 5f                          	popq	%rdi
141dade28: 5e                          	popq	%rsi
141dade29: 5d                          	popq	%rbp
141dade2a: c3                          	retq
141dade2b: b9 08 00 00 00              	movl	$0x8, %ecx
141dade30: ba e8 03 00 00              	movl	$0x3e8, %edx            # imm = 0x3E8
141dade35: e8 c6 b1 b2 00              	callq	0x1428d9000 <.text+0x28d8000>
141dade3a: 0f 0b                       	ud2
141dade3c: 0f 1f 40 00                 	nopl	(%rax)
141dade40: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141dade45: 55                          	pushq	%rbp
141dade46: 56                          	pushq	%rsi
141dade47: 57                          	pushq	%rdi
141dade48: 48 83 ec 20                 	subq	$0x20, %rsp
141dade4c: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141dade53: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
141dade57: e8 54 4e 83 fe              	callq	0x1405e2cb0 <.text+0x5e1cb0>
141dade5c: 90                          	nop
141dade5d: 48 83 c4 20                 	addq	$0x20, %rsp
141dade61: 5f                          	popq	%rdi
141dade62: 5e                          	popq	%rsi
141dade63: 5d                          	popq	%rbp
141dade64: c3                          	retq
