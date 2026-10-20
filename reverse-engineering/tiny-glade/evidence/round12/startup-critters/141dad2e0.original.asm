
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141dad2e0: 55                          	pushq	%rbp
141dad2e1: 56                          	pushq	%rsi
141dad2e2: 57                          	pushq	%rdi
141dad2e3: 48 81 ec 10 04 00 00        	subq	$0x410, %rsp            # imm = 0x410
141dad2ea: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
141dad2f2: 48 c7 85 88 03 00 00 fe ff ff ff    	movq	$-0x2, 0x388(%rbp)
141dad2fd: 48 89 ce                    	movq	%rcx, %rsi
141dad300: 0f 57 c0                    	xorps	%xmm0, %xmm0
141dad303: 0f 11 45 c0                 	movups	%xmm0, -0x40(%rbp)
141dad307: 0f 11 45 d8                 	movups	%xmm0, -0x28(%rbp)
141dad30b: 0f 11 45 f0                 	movups	%xmm0, -0x10(%rbp)
141dad30f: 0f 11 45 08                 	movups	%xmm0, 0x8(%rbp)
141dad313: 0f 11 45 20                 	movups	%xmm0, 0x20(%rbp)
141dad317: c7 45 30 00 00 00 00        	movl	$0x0, 0x30(%rbp)
141dad31e: 0f 11 45 58                 	movups	%xmm0, 0x58(%rbp)
141dad322: 0f 11 45 70                 	movups	%xmm0, 0x70(%rbp)
141dad326: 0f 11 85 88 00 00 00        	movups	%xmm0, 0x88(%rbp)
141dad32d: 0f 11 85 a0 00 00 00        	movups	%xmm0, 0xa0(%rbp)
141dad334: 0f 11 85 b8 00 00 00        	movups	%xmm0, 0xb8(%rbp)
141dad33b: c7 85 c8 00 00 00 00 00 00 00       	movl	$0x0, 0xc8(%rbp)
141dad345: 48 c7 45 a0 00 00 00 00     	movq	$0x0, -0x60(%rbp)
141dad34d: 48 c7 45 a8 08 00 00 00     	movq	$0x8, -0x58(%rbp)
141dad355: 48 c7 45 b0 00 00 00 00     	movq	$0x0, -0x50(%rbp)
141dad35d: 48 c7 45 b8 10 00 00 00     	movq	$0x10, -0x48(%rbp)
141dad365: 48 c7 45 d0 10 00 00 00     	movq	$0x10, -0x30(%rbp)
141dad36d: 48 c7 45 e8 10 00 00 00     	movq	$0x10, -0x18(%rbp)
141dad375: 48 c7 45 00 10 00 00 00     	movq	$0x10, (%rbp)
141dad37d: 48 c7 45 18 10 00 00 00     	movq	$0x10, 0x18(%rbp)
141dad385: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
141dad38f: 48 89 45 38                 	movq	%rax, 0x38(%rbp)
141dad393: 48 8d 0d 76 1b 1c 01        	leaq	0x11c1b76(%rip), %rcx   # 0x142f6ef10
141dad39a: 48 89 4d 40                 	movq	%rcx, 0x40(%rbp)
141dad39e: 48 c7 45 48 37 00 00 00     	movq	$0x37, 0x48(%rbp)
141dad3a6: 48 c7 45 50 10 00 00 00     	movq	$0x10, 0x50(%rbp)
141dad3ae: 48 c7 45 68 10 00 00 00     	movq	$0x10, 0x68(%rbp)
141dad3b6: 48 c7 85 80 00 00 00 10 00 00 00    	movq	$0x10, 0x80(%rbp)
141dad3c1: 48 c7 85 98 00 00 00 10 00 00 00    	movq	$0x10, 0x98(%rbp)
141dad3cc: 48 c7 85 b0 00 00 00 10 00 00 00    	movq	$0x10, 0xb0(%rbp)
141dad3d7: c7 85 d0 00 00 00 00 00 00 00       	movl	$0x0, 0xd0(%rbp)
141dad3e1: 66 c7 85 d4 00 00 00 01 00  	movw	$0x1, 0xd4(%rbp)
141dad3ea: 48 89 85 d8 00 00 00        	movq	%rax, 0xd8(%rbp)
141dad3f1: c7 85 80 03 00 00 00 00 00 00       	movl	$0x0, 0x380(%rbp)
141dad3fb: 0f b6 05 07 78 88 01        	movzbl	0x1887807(%rip), %eax   # 0x143634c09
141dad402: b9 e8 03 00 00              	movl	$0x3e8, %ecx            # imm = 0x3E8
141dad407: ba 08 00 00 00              	movl	$0x8, %edx
141dad40c: e8 ff 67 86 fe              	callq	0x140613c10 <.text+0x612c10>
141dad411: 48 85 c0                    	testq	%rax, %rax
141dad414: 74 35                       	je	0x141dad44b <.text+0x1dac44b>
141dad416: 48 89 c7                    	movq	%rax, %rdi
141dad419: 48 8d 55 a0                 	leaq	-0x60(%rbp), %rdx
141dad41d: 41 b8 e8 03 00 00           	movl	$0x3e8, %r8d            # imm = 0x3E8
141dad423: 48 89 c1                    	movq	%rax, %rcx
141dad426: e8 d5 67 b7 00              	callq	0x142923c00 <.text+0x2922c00>
141dad42b: 4c 8d 05 26 1a 1c 01        	leaq	0x11c1a26(%rip), %r8    # 0x142f6ee58
141dad432: 48 89 f1                    	movq	%rsi, %rcx
141dad435: 48 89 fa                    	movq	%rdi, %rdx
141dad438: e8 d3 76 f3 fe              	callq	0x140ce4b10 <.text+0xce3b10>
141dad43d: 48 89 f0                    	movq	%rsi, %rax
141dad440: 48 81 c4 10 04 00 00        	addq	$0x410, %rsp            # imm = 0x410
141dad447: 5f                          	popq	%rdi
141dad448: 5e                          	popq	%rsi
141dad449: 5d                          	popq	%rbp
141dad44a: c3                          	retq
141dad44b: b9 08 00 00 00              	movl	$0x8, %ecx
141dad450: ba e8 03 00 00              	movl	$0x3e8, %edx            # imm = 0x3E8
141dad455: e8 a6 bb b2 00              	callq	0x1428d9000 <.text+0x28d8000>
141dad45a: 0f 0b                       	ud2
141dad45c: 0f 1f 40 00                 	nopl	(%rax)
141dad460: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141dad465: 55                          	pushq	%rbp
141dad466: 56                          	pushq	%rsi
141dad467: 57                          	pushq	%rdi
141dad468: 48 83 ec 20                 	subq	$0x20, %rsp
141dad46c: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141dad473: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
141dad477: e8 34 58 83 fe              	callq	0x1405e2cb0 <.text+0x5e1cb0>
141dad47c: 90                          	nop
141dad47d: 48 83 c4 20                 	addq	$0x20, %rsp
141dad481: 5f                          	popq	%rdi
141dad482: 5e                          	popq	%rsi
141dad483: 5d                          	popq	%rbp
141dad484: c3                          	retq
