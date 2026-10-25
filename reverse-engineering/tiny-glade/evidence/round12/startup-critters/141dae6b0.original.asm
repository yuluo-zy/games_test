
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141dae6b0: 55                          	pushq	%rbp
141dae6b1: 56                          	pushq	%rsi
141dae6b2: 57                          	pushq	%rdi
141dae6b3: 48 81 ec b0 02 00 00        	subq	$0x2b0, %rsp            # imm = 0x2B0
141dae6ba: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
141dae6c2: 48 c7 85 28 02 00 00 fe ff ff ff    	movq	$-0x2, 0x228(%rbp)
141dae6cd: 48 89 ce                    	movq	%rcx, %rsi
141dae6d0: 0f 57 c0                    	xorps	%xmm0, %xmm0
141dae6d3: 0f 11 45 c8                 	movups	%xmm0, -0x38(%rbp)
141dae6d7: 0f 11 45 e0                 	movups	%xmm0, -0x20(%rbp)
141dae6db: 0f 11 45 f8                 	movups	%xmm0, -0x8(%rbp)
141dae6df: 0f 11 45 10                 	movups	%xmm0, 0x10(%rbp)
141dae6e3: 0f 11 45 28                 	movups	%xmm0, 0x28(%rbp)
141dae6e7: c7 45 38 00 00 00 00        	movl	$0x0, 0x38(%rbp)
141dae6ee: 0f 11 45 60                 	movups	%xmm0, 0x60(%rbp)
141dae6f2: 0f 11 45 78                 	movups	%xmm0, 0x78(%rbp)
141dae6f6: 0f 11 85 90 00 00 00        	movups	%xmm0, 0x90(%rbp)
141dae6fd: 0f 11 85 a8 00 00 00        	movups	%xmm0, 0xa8(%rbp)
141dae704: 0f 11 85 c0 00 00 00        	movups	%xmm0, 0xc0(%rbp)
141dae70b: c7 85 d0 00 00 00 00 00 00 00       	movl	$0x0, 0xd0(%rbp)
141dae715: 48 c7 45 a8 00 00 00 00     	movq	$0x0, -0x58(%rbp)
141dae71d: 48 c7 45 b0 08 00 00 00     	movq	$0x8, -0x50(%rbp)
141dae725: 48 c7 45 b8 00 00 00 00     	movq	$0x0, -0x48(%rbp)
141dae72d: 48 c7 45 c0 10 00 00 00     	movq	$0x10, -0x40(%rbp)
141dae735: 48 c7 45 d8 10 00 00 00     	movq	$0x10, -0x28(%rbp)
141dae73d: 48 c7 45 f0 10 00 00 00     	movq	$0x10, -0x10(%rbp)
141dae745: 48 c7 45 08 10 00 00 00     	movq	$0x10, 0x8(%rbp)
141dae74d: 48 c7 45 20 10 00 00 00     	movq	$0x10, 0x20(%rbp)
141dae755: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
141dae75f: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
141dae763: 48 8d 0d 8e 0f 1c 01        	leaq	0x11c0f8e(%rip), %rcx   # 0x142f6f6f8
141dae76a: 48 89 4d 48                 	movq	%rcx, 0x48(%rbp)
141dae76e: 48 c7 45 50 4c 00 00 00     	movq	$0x4c, 0x50(%rbp)
141dae776: 48 c7 45 58 10 00 00 00     	movq	$0x10, 0x58(%rbp)
141dae77e: 48 c7 45 70 10 00 00 00     	movq	$0x10, 0x70(%rbp)
141dae786: 48 c7 85 88 00 00 00 10 00 00 00    	movq	$0x10, 0x88(%rbp)
141dae791: 48 c7 85 a0 00 00 00 10 00 00 00    	movq	$0x10, 0xa0(%rbp)
141dae79c: 48 c7 85 b8 00 00 00 10 00 00 00    	movq	$0x10, 0xb8(%rbp)
141dae7a7: c7 85 d8 00 00 00 00 00 00 00       	movl	$0x0, 0xd8(%rbp)
141dae7b1: 66 c7 85 dc 00 00 00 01 00  	movw	$0x1, 0xdc(%rbp)
141dae7ba: 48 89 85 d8 01 00 00        	movq	%rax, 0x1d8(%rbp)
141dae7c1: c7 85 20 02 00 00 00 00 00 00       	movl	$0x0, 0x220(%rbp)
141dae7cb: 0f b6 05 37 64 88 01        	movzbl	0x1886437(%rip), %eax   # 0x143634c09
141dae7d2: b9 80 02 00 00              	movl	$0x280, %ecx            # imm = 0x280
141dae7d7: ba 08 00 00 00              	movl	$0x8, %edx
141dae7dc: e8 2f 54 86 fe              	callq	0x140613c10 <.text+0x612c10>
141dae7e1: 48 85 c0                    	testq	%rax, %rax
141dae7e4: 74 35                       	je	0x141dae81b <.text+0x1dad81b>
141dae7e6: 48 89 c7                    	movq	%rax, %rdi
141dae7e9: 48 8d 55 a8                 	leaq	-0x58(%rbp), %rdx
141dae7ed: 41 b8 80 02 00 00           	movl	$0x280, %r8d            # imm = 0x280
141dae7f3: 48 89 c1                    	movq	%rax, %rcx
141dae7f6: e8 05 54 b7 00              	callq	0x142923c00 <.text+0x2922c00>
141dae7fb: 4c 8d 05 3e 0e 1c 01        	leaq	0x11c0e3e(%rip), %r8    # 0x142f6f640
141dae802: 48 89 f1                    	movq	%rsi, %rcx
141dae805: 48 89 fa                    	movq	%rdi, %rdx
141dae808: e8 03 63 f3 fe              	callq	0x140ce4b10 <.text+0xce3b10>
141dae80d: 48 89 f0                    	movq	%rsi, %rax
141dae810: 48 81 c4 b0 02 00 00        	addq	$0x2b0, %rsp            # imm = 0x2B0
141dae817: 5f                          	popq	%rdi
141dae818: 5e                          	popq	%rsi
141dae819: 5d                          	popq	%rbp
141dae81a: c3                          	retq
141dae81b: b9 08 00 00 00              	movl	$0x8, %ecx
141dae820: ba 80 02 00 00              	movl	$0x280, %edx            # imm = 0x280
141dae825: e8 d6 a7 b2 00              	callq	0x1428d9000 <.text+0x28d8000>
141dae82a: 0f 0b                       	ud2
141dae82c: 0f 1f 40 00                 	nopl	(%rax)
141dae830: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141dae835: 55                          	pushq	%rbp
141dae836: 56                          	pushq	%rsi
141dae837: 57                          	pushq	%rdi
141dae838: 48 83 ec 20                 	subq	$0x20, %rsp
141dae83c: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141dae843: 48 8d 4d a8                 	leaq	-0x58(%rbp), %rcx
141dae847: e8 64 0b 82 fe              	callq	0x1405cf3b0 <.text+0x5ce3b0>
141dae84c: 90                          	nop
141dae84d: 48 83 c4 20                 	addq	$0x20, %rsp
141dae851: 5f                          	popq	%rdi
141dae852: 5e                          	popq	%rsi
141dae853: 5d                          	popq	%rbp
141dae854: c3                          	retq
