
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141db5920: 55                          	pushq	%rbp
141db5921: 56                          	pushq	%rsi
141db5922: 57                          	pushq	%rdi
141db5923: 48 81 ec a0 04 00 00        	subq	$0x4a0, %rsp            # imm = 0x4A0
141db592a: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
141db5932: 48 c7 85 18 04 00 00 fe ff ff ff    	movq	$-0x2, 0x418(%rbp)
141db593d: 48 89 ce                    	movq	%rcx, %rsi
141db5940: 0f 57 c0                    	xorps	%xmm0, %xmm0
141db5943: 0f 11 45 c8                 	movups	%xmm0, -0x38(%rbp)
141db5947: 0f 11 45 e0                 	movups	%xmm0, -0x20(%rbp)
141db594b: 0f 11 45 f8                 	movups	%xmm0, -0x8(%rbp)
141db594f: 0f 11 45 10                 	movups	%xmm0, 0x10(%rbp)
141db5953: 0f 11 45 28                 	movups	%xmm0, 0x28(%rbp)
141db5957: c7 45 38 00 00 00 00        	movl	$0x0, 0x38(%rbp)
141db595e: 0f 11 45 60                 	movups	%xmm0, 0x60(%rbp)
141db5962: 0f 11 45 78                 	movups	%xmm0, 0x78(%rbp)
141db5966: 0f 11 85 90 00 00 00        	movups	%xmm0, 0x90(%rbp)
141db596d: 0f 11 85 a8 00 00 00        	movups	%xmm0, 0xa8(%rbp)
141db5974: 0f 11 85 c0 00 00 00        	movups	%xmm0, 0xc0(%rbp)
141db597b: c7 85 d0 00 00 00 00 00 00 00       	movl	$0x0, 0xd0(%rbp)
141db5985: 48 c7 45 a8 00 00 00 00     	movq	$0x0, -0x58(%rbp)
141db598d: 48 c7 45 b0 08 00 00 00     	movq	$0x8, -0x50(%rbp)
141db5995: 48 c7 45 b8 00 00 00 00     	movq	$0x0, -0x48(%rbp)
141db599d: 48 c7 45 c0 10 00 00 00     	movq	$0x10, -0x40(%rbp)
141db59a5: 48 c7 45 d8 10 00 00 00     	movq	$0x10, -0x28(%rbp)
141db59ad: 48 c7 45 f0 10 00 00 00     	movq	$0x10, -0x10(%rbp)
141db59b5: 48 c7 45 08 10 00 00 00     	movq	$0x10, 0x8(%rbp)
141db59bd: 48 c7 45 20 10 00 00 00     	movq	$0x10, 0x20(%rbp)
141db59c5: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
141db59cf: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
141db59d3: 48 8d 0d a6 b5 1b 01        	leaq	0x11bb5a6(%rip), %rcx   # 0x142f70f80
141db59da: 48 89 4d 48                 	movq	%rcx, 0x48(%rbp)
141db59de: 48 c7 45 50 1b 00 00 00     	movq	$0x1b, 0x50(%rbp)
141db59e6: 48 c7 45 58 10 00 00 00     	movq	$0x10, 0x58(%rbp)
141db59ee: 48 c7 45 70 10 00 00 00     	movq	$0x10, 0x70(%rbp)
141db59f6: 48 c7 85 88 00 00 00 10 00 00 00    	movq	$0x10, 0x88(%rbp)
141db5a01: 48 c7 85 a0 00 00 00 10 00 00 00    	movq	$0x10, 0xa0(%rbp)
141db5a0c: 48 c7 85 b8 00 00 00 10 00 00 00    	movq	$0x10, 0xb8(%rbp)
141db5a17: c7 85 d8 00 00 00 00 00 00 00       	movl	$0x0, 0xd8(%rbp)
141db5a21: 66 c7 85 dc 00 00 00 01 00  	movw	$0x1, 0xdc(%rbp)
141db5a2a: 48 89 85 e0 00 00 00        	movq	%rax, 0xe0(%rbp)
141db5a31: c7 85 10 04 00 00 00 00 00 00       	movl	$0x0, 0x410(%rbp)
141db5a3b: 0f b6 05 c7 f1 87 01        	movzbl	0x187f1c7(%rip), %eax   # 0x143634c09
141db5a42: b9 70 04 00 00              	movl	$0x470, %ecx            # imm = 0x470
141db5a47: ba 08 00 00 00              	movl	$0x8, %edx
141db5a4c: e8 bf e1 85 fe              	callq	0x140613c10 <.text+0x612c10>
141db5a51: 48 85 c0                    	testq	%rax, %rax
141db5a54: 74 35                       	je	0x141db5a8b <.text+0x1db4a8b>
141db5a56: 48 89 c7                    	movq	%rax, %rdi
141db5a59: 48 8d 55 a8                 	leaq	-0x58(%rbp), %rdx
141db5a5d: 41 b8 70 04 00 00           	movl	$0x470, %r8d            # imm = 0x470
141db5a63: 48 89 c1                    	movq	%rax, %rcx
141db5a66: e8 95 e1 b6 00              	callq	0x142923c00 <.text+0x2922c00>
141db5a6b: 4c 8d 05 16 b4 1b 01        	leaq	0x11bb416(%rip), %r8    # 0x142f70e88
141db5a72: 48 89 f1                    	movq	%rsi, %rcx
141db5a75: 48 89 fa                    	movq	%rdi, %rdx
141db5a78: e8 93 f0 f2 fe              	callq	0x140ce4b10 <.text+0xce3b10>
141db5a7d: 48 89 f0                    	movq	%rsi, %rax
141db5a80: 48 81 c4 a0 04 00 00        	addq	$0x4a0, %rsp            # imm = 0x4A0
141db5a87: 5f                          	popq	%rdi
141db5a88: 5e                          	popq	%rsi
141db5a89: 5d                          	popq	%rbp
141db5a8a: c3                          	retq
141db5a8b: b9 08 00 00 00              	movl	$0x8, %ecx
141db5a90: ba 70 04 00 00              	movl	$0x470, %edx            # imm = 0x470
141db5a95: e8 66 35 b2 00              	callq	0x1428d9000 <.text+0x28d8000>
141db5a9a: 0f 0b                       	ud2
141db5a9c: 0f 1f 40 00                 	nopl	(%rax)
141db5aa0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141db5aa5: 55                          	pushq	%rbp
141db5aa6: 56                          	pushq	%rsi
141db5aa7: 57                          	pushq	%rdi
141db5aa8: 48 83 ec 20                 	subq	$0x20, %rsp
141db5aac: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141db5ab3: 48 8d 4d a8                 	leaq	-0x58(%rbp), %rcx
141db5ab7: e8 d4 c4 fc ff              	callq	0x141d81f90 <.text+0x1d80f90>
141db5abc: 90                          	nop
141db5abd: 48 83 c4 20                 	addq	$0x20, %rsp
141db5ac1: 5f                          	popq	%rdi
141db5ac2: 5e                          	popq	%rsi
141db5ac3: 5d                          	popq	%rbp
141db5ac4: c3                          	retq
