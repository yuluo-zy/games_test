
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141d8fa10: 55                          	pushq	%rbp
141d8fa11: 56                          	pushq	%rsi
141d8fa12: 57                          	pushq	%rdi
141d8fa13: 48 81 ec 10 03 00 00        	subq	$0x310, %rsp            # imm = 0x310
141d8fa1a: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
141d8fa22: 48 c7 85 88 02 00 00 fe ff ff ff    	movq	$-0x2, 0x288(%rbp)
141d8fa2d: 48 89 ce                    	movq	%rcx, %rsi
141d8fa30: 0f 57 c0                    	xorps	%xmm0, %xmm0
141d8fa33: 0f 11 45 c8                 	movups	%xmm0, -0x38(%rbp)
141d8fa37: 0f 11 45 e0                 	movups	%xmm0, -0x20(%rbp)
141d8fa3b: 0f 11 45 f8                 	movups	%xmm0, -0x8(%rbp)
141d8fa3f: 0f 11 45 10                 	movups	%xmm0, 0x10(%rbp)
141d8fa43: 0f 11 45 28                 	movups	%xmm0, 0x28(%rbp)
141d8fa47: c7 45 38 00 00 00 00        	movl	$0x0, 0x38(%rbp)
141d8fa4e: 0f 11 45 60                 	movups	%xmm0, 0x60(%rbp)
141d8fa52: 0f 11 45 78                 	movups	%xmm0, 0x78(%rbp)
141d8fa56: 0f 11 85 90 00 00 00        	movups	%xmm0, 0x90(%rbp)
141d8fa5d: 0f 11 85 a8 00 00 00        	movups	%xmm0, 0xa8(%rbp)
141d8fa64: 0f 11 85 c0 00 00 00        	movups	%xmm0, 0xc0(%rbp)
141d8fa6b: c7 85 d0 00 00 00 00 00 00 00       	movl	$0x0, 0xd0(%rbp)
141d8fa75: 48 c7 45 a8 00 00 00 00     	movq	$0x0, -0x58(%rbp)
141d8fa7d: 48 c7 45 b0 08 00 00 00     	movq	$0x8, -0x50(%rbp)
141d8fa85: 48 c7 45 b8 00 00 00 00     	movq	$0x0, -0x48(%rbp)
141d8fa8d: 48 c7 45 c0 10 00 00 00     	movq	$0x10, -0x40(%rbp)
141d8fa95: 48 c7 45 d8 10 00 00 00     	movq	$0x10, -0x28(%rbp)
141d8fa9d: 48 c7 45 f0 10 00 00 00     	movq	$0x10, -0x10(%rbp)
141d8faa5: 48 c7 45 08 10 00 00 00     	movq	$0x10, 0x8(%rbp)
141d8faad: 48 c7 45 20 10 00 00 00     	movq	$0x10, 0x20(%rbp)
141d8fab5: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
141d8fabf: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
141d8fac3: 48 8d 0d cb b4 1d 01        	leaq	0x11db4cb(%rip), %rcx   # 0x142f6af95
141d8faca: 48 89 4d 48                 	movq	%rcx, 0x48(%rbp)
141d8face: 48 c7 45 50 34 00 00 00     	movq	$0x34, 0x50(%rbp)
141d8fad6: 48 c7 45 58 10 00 00 00     	movq	$0x10, 0x58(%rbp)
141d8fade: 48 c7 45 70 10 00 00 00     	movq	$0x10, 0x70(%rbp)
141d8fae6: 48 c7 85 88 00 00 00 10 00 00 00    	movq	$0x10, 0x88(%rbp)
141d8faf1: 48 c7 85 a0 00 00 00 10 00 00 00    	movq	$0x10, 0xa0(%rbp)
141d8fafc: 48 c7 85 b8 00 00 00 10 00 00 00    	movq	$0x10, 0xb8(%rbp)
141d8fb07: c7 85 d8 00 00 00 00 00 00 00       	movl	$0x0, 0xd8(%rbp)
141d8fb11: 66 c7 85 dc 00 00 00 01 00  	movw	$0x1, 0xdc(%rbp)
141d8fb1a: 48 89 85 e0 00 00 00        	movq	%rax, 0xe0(%rbp)
141d8fb21: c7 85 80 02 00 00 00 00 00 00       	movl	$0x0, 0x280(%rbp)
141d8fb2b: 0f b6 05 d7 50 8a 01        	movzbl	0x18a50d7(%rip), %eax   # 0x143634c09
141d8fb32: b9 e0 02 00 00              	movl	$0x2e0, %ecx            # imm = 0x2E0
141d8fb37: ba 08 00 00 00              	movl	$0x8, %edx
141d8fb3c: e8 cf 40 88 fe              	callq	0x140613c10 <.text+0x612c10>
141d8fb41: 48 85 c0                    	testq	%rax, %rax
141d8fb44: 74 35                       	je	0x141d8fb7b <.text+0x1d8eb7b>
141d8fb46: 48 89 c7                    	movq	%rax, %rdi
141d8fb49: 48 8d 55 a8                 	leaq	-0x58(%rbp), %rdx
141d8fb4d: 41 b8 e0 02 00 00           	movl	$0x2e0, %r8d            # imm = 0x2E0
141d8fb53: 48 89 c1                    	movq	%rax, %rcx
141d8fb56: e8 a5 40 b9 00              	callq	0x142923c00 <.text+0x2922c00>
141d8fb5b: 4c 8d 05 36 b1 1d 01        	leaq	0x11db136(%rip), %r8    # 0x142f6ac98
141d8fb62: 48 89 f1                    	movq	%rsi, %rcx
141d8fb65: 48 89 fa                    	movq	%rdi, %rdx
141d8fb68: e8 a3 4f f5 fe              	callq	0x140ce4b10 <.text+0xce3b10>
141d8fb6d: 48 89 f0                    	movq	%rsi, %rax
141d8fb70: 48 81 c4 10 03 00 00        	addq	$0x310, %rsp            # imm = 0x310
141d8fb77: 5f                          	popq	%rdi
141d8fb78: 5e                          	popq	%rsi
141d8fb79: 5d                          	popq	%rbp
141d8fb7a: c3                          	retq
141d8fb7b: b9 08 00 00 00              	movl	$0x8, %ecx
141d8fb80: ba e0 02 00 00              	movl	$0x2e0, %edx            # imm = 0x2E0
141d8fb85: e8 76 94 b4 00              	callq	0x1428d9000 <.text+0x28d8000>
141d8fb8a: 0f 0b                       	ud2
141d8fb8c: 0f 1f 40 00                 	nopl	(%rax)
141d8fb90: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141d8fb95: 55                          	pushq	%rbp
141d8fb96: 56                          	pushq	%rsi
141d8fb97: 57                          	pushq	%rdi
141d8fb98: 48 83 ec 20                 	subq	$0x20, %rsp
141d8fb9c: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141d8fba3: 48 8d 4d a8                 	leaq	-0x58(%rbp), %rcx
141d8fba7: e8 e4 75 84 fe              	callq	0x1405d7190 <.text+0x5d6190>
141d8fbac: 90                          	nop
141d8fbad: 48 83 c4 20                 	addq	$0x20, %rsp
141d8fbb1: 5f                          	popq	%rdi
141d8fbb2: 5e                          	popq	%rsi
141d8fbb3: 5d                          	popq	%rbp
141d8fbb4: c3                          	retq
