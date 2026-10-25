
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141dabbe0: 55                          	pushq	%rbp
141dabbe1: 56                          	pushq	%rsi
141dabbe2: 57                          	pushq	%rdi
141dabbe3: 48 81 ec a0 02 00 00        	subq	$0x2a0, %rsp            # imm = 0x2A0
141dabbea: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
141dabbf2: 48 c7 85 18 02 00 00 fe ff ff ff    	movq	$-0x2, 0x218(%rbp)
141dabbfd: 48 89 ce                    	movq	%rcx, %rsi
141dabc00: 0f 57 c0                    	xorps	%xmm0, %xmm0
141dabc03: 0f 11 45 c8                 	movups	%xmm0, -0x38(%rbp)
141dabc07: 0f 11 45 e0                 	movups	%xmm0, -0x20(%rbp)
141dabc0b: 0f 11 45 f8                 	movups	%xmm0, -0x8(%rbp)
141dabc0f: 0f 11 45 10                 	movups	%xmm0, 0x10(%rbp)
141dabc13: 0f 11 45 28                 	movups	%xmm0, 0x28(%rbp)
141dabc17: c7 45 38 00 00 00 00        	movl	$0x0, 0x38(%rbp)
141dabc1e: 0f 11 45 60                 	movups	%xmm0, 0x60(%rbp)
141dabc22: 0f 11 45 78                 	movups	%xmm0, 0x78(%rbp)
141dabc26: 0f 11 85 90 00 00 00        	movups	%xmm0, 0x90(%rbp)
141dabc2d: 0f 11 85 a8 00 00 00        	movups	%xmm0, 0xa8(%rbp)
141dabc34: 0f 11 85 c0 00 00 00        	movups	%xmm0, 0xc0(%rbp)
141dabc3b: c7 85 d0 00 00 00 00 00 00 00       	movl	$0x0, 0xd0(%rbp)
141dabc45: 48 c7 45 a8 00 00 00 00     	movq	$0x0, -0x58(%rbp)
141dabc4d: 48 c7 45 b0 08 00 00 00     	movq	$0x8, -0x50(%rbp)
141dabc55: 48 c7 45 b8 00 00 00 00     	movq	$0x0, -0x48(%rbp)
141dabc5d: 48 c7 45 c0 10 00 00 00     	movq	$0x10, -0x40(%rbp)
141dabc65: 48 c7 45 d8 10 00 00 00     	movq	$0x10, -0x28(%rbp)
141dabc6d: 48 c7 45 f0 10 00 00 00     	movq	$0x10, -0x10(%rbp)
141dabc75: 48 c7 45 08 10 00 00 00     	movq	$0x10, 0x8(%rbp)
141dabc7d: 48 c7 45 20 10 00 00 00     	movq	$0x10, 0x20(%rbp)
141dabc85: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
141dabc8f: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
141dabc93: 48 8d 0d 13 2a 1c 01        	leaq	0x11c2a13(%rip), %rcx   # 0x142f6e6ad
141dabc9a: 48 89 4d 48                 	movq	%rcx, 0x48(%rbp)
141dabc9e: 48 c7 45 50 32 00 00 00     	movq	$0x32, 0x50(%rbp)
141dabca6: 48 c7 45 58 10 00 00 00     	movq	$0x10, 0x58(%rbp)
141dabcae: 48 c7 45 70 10 00 00 00     	movq	$0x10, 0x70(%rbp)
141dabcb6: 48 c7 85 88 00 00 00 10 00 00 00    	movq	$0x10, 0x88(%rbp)
141dabcc1: 48 c7 85 a0 00 00 00 10 00 00 00    	movq	$0x10, 0xa0(%rbp)
141dabccc: 48 c7 85 b8 00 00 00 10 00 00 00    	movq	$0x10, 0xb8(%rbp)
141dabcd7: c7 85 d8 00 00 00 00 00 00 00       	movl	$0x0, 0xd8(%rbp)
141dabce1: 66 c7 85 dc 00 00 00 01 00  	movw	$0x1, 0xdc(%rbp)
141dabcea: 48 89 85 d8 01 00 00        	movq	%rax, 0x1d8(%rbp)
141dabcf1: c7 85 10 02 00 00 00 00 00 00       	movl	$0x0, 0x210(%rbp)
141dabcfb: 0f b6 05 07 8f 88 01        	movzbl	0x1888f07(%rip), %eax   # 0x143634c09
141dabd02: b9 70 02 00 00              	movl	$0x270, %ecx            # imm = 0x270
141dabd07: ba 08 00 00 00              	movl	$0x8, %edx
141dabd0c: e8 ff 7e 86 fe              	callq	0x140613c10 <.text+0x612c10>
141dabd11: 48 85 c0                    	testq	%rax, %rax
141dabd14: 74 35                       	je	0x141dabd4b <.text+0x1daad4b>
141dabd16: 48 89 c7                    	movq	%rax, %rdi
141dabd19: 48 8d 55 a8                 	leaq	-0x58(%rbp), %rdx
141dabd1d: 41 b8 70 02 00 00           	movl	$0x270, %r8d            # imm = 0x270
141dabd23: 48 89 c1                    	movq	%rax, %rcx
141dabd26: e8 d5 7e b7 00              	callq	0x142923c00 <.text+0x2922c00>
141dabd2b: 4c 8d 05 36 27 1c 01        	leaq	0x11c2736(%rip), %r8    # 0x142f6e468
141dabd32: 48 89 f1                    	movq	%rsi, %rcx
141dabd35: 48 89 fa                    	movq	%rdi, %rdx
141dabd38: e8 d3 8d f3 fe              	callq	0x140ce4b10 <.text+0xce3b10>
141dabd3d: 48 89 f0                    	movq	%rsi, %rax
141dabd40: 48 81 c4 a0 02 00 00        	addq	$0x2a0, %rsp            # imm = 0x2A0
141dabd47: 5f                          	popq	%rdi
141dabd48: 5e                          	popq	%rsi
141dabd49: 5d                          	popq	%rbp
141dabd4a: c3                          	retq
141dabd4b: b9 08 00 00 00              	movl	$0x8, %ecx
141dabd50: ba 70 02 00 00              	movl	$0x270, %edx            # imm = 0x270
141dabd55: e8 a6 d2 b2 00              	callq	0x1428d9000 <.text+0x28d8000>
141dabd5a: 0f 0b                       	ud2
141dabd5c: 0f 1f 40 00                 	nopl	(%rax)
141dabd60: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141dabd65: 55                          	pushq	%rbp
141dabd66: 56                          	pushq	%rsi
141dabd67: 57                          	pushq	%rdi
141dabd68: 48 83 ec 20                 	subq	$0x20, %rsp
141dabd6c: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141dabd73: 48 8d 4d a8                 	leaq	-0x58(%rbp), %rcx
141dabd77: e8 34 36 82 fe              	callq	0x1405cf3b0 <.text+0x5ce3b0>
141dabd7c: 90                          	nop
141dabd7d: 48 83 c4 20                 	addq	$0x20, %rsp
141dabd81: 5f                          	popq	%rdi
141dabd82: 5e                          	popq	%rsi
141dabd83: 5d                          	popq	%rbp
141dabd84: c3                          	retq
