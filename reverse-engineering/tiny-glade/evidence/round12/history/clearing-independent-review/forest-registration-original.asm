
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140650c50: 55                          	pushq	%rbp
140650c51: 56                          	pushq	%rsi
140650c52: 48 81 ec 18 01 00 00        	subq	$0x118, %rsp            # imm = 0x118
140650c59: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140650c61: 48 c7 85 90 00 00 00 fe ff ff ff    	movq	$-0x2, 0x90(%rbp)
140650c6c: 48 89 d6                    	movq	%rdx, %rsi
140650c6f: 48 8d 4d 18                 	leaq	0x18(%rbp), %rcx
140650c73: e8 58 5c 85 00              	callq	0x140ea68d0 <.text+0xea58d0>
140650c78: c6 85 8f 00 00 00 24        	movb	$0x24, 0x8f(%rbp)
140650c7f: 48 8b 0d ea b6 fd 02        	movq	0x2fdb6ea(%rip), %rcx   # 0x14362c370
140650c86: 4c 8d 05 cb ef 53 02        	leaq	0x253efcb(%rip), %r8    # 0x142b8fc58
140650c8d: 48 8d 95 8f 00 00 00        	leaq	0x8f(%rbp), %rdx
140650c94: e8 07 68 85 00              	callq	0x140ea74a0 <.text+0xea64a0>
140650c99: 49 89 d0                    	movq	%rdx, %r8
140650c9c: 48 8d 4d 18                 	leaq	0x18(%rbp), %rcx
140650ca0: 48 89 c2                    	movq	%rax, %rdx
140650ca3: e8 58 6f 85 00              	callq	0x140ea7c00 <.text+0xea6c00>
140650ca8: 0f 10 45 78                 	movups	0x78(%rbp), %xmm0
140650cac: 0f 29 45 00                 	movaps	%xmm0, (%rbp)
140650cb0: 0f 10 45 68                 	movups	0x68(%rbp), %xmm0
140650cb4: 0f 29 45 f0                 	movaps	%xmm0, -0x10(%rbp)
140650cb8: 0f 10 45 58                 	movups	0x58(%rbp), %xmm0
140650cbc: 0f 29 45 e0                 	movaps	%xmm0, -0x20(%rbp)
140650cc0: 0f 10 45 18                 	movups	0x18(%rbp), %xmm0
140650cc4: 0f 10 4d 28                 	movups	0x28(%rbp), %xmm1
140650cc8: 0f 10 55 38                 	movups	0x38(%rbp), %xmm2
140650ccc: 0f 10 5d 48                 	movups	0x48(%rbp), %xmm3
140650cd0: 0f 29 5d d0                 	movaps	%xmm3, -0x30(%rbp)
140650cd4: 0f 29 55 c0                 	movaps	%xmm2, -0x40(%rbp)
140650cd8: 0f 29 4d b0                 	movaps	%xmm1, -0x50(%rbp)
140650cdc: 0f 29 45 a0                 	movaps	%xmm0, -0x60(%rbp)
140650ce0: 48 8d 55 a0                 	leaq	-0x60(%rbp), %rdx
140650ce4: 48 89 f1                    	movq	%rsi, %rcx
140650ce7: e8 a4 6f 85 00              	callq	0x140ea7c90 <.text+0xea6c90>
140650cec: 48 c7 45 18 00 00 00 00     	movq	$0x0, 0x18(%rbp)
140650cf4: 48 c7 45 20 08 00 00 00     	movq	$0x8, 0x20(%rbp)
140650cfc: 48 c7 45 28 00 00 00 00     	movq	$0x0, 0x28(%rbp)
140650d04: c6 45 30 00                 	movb	$0x0, 0x30(%rbp)
140650d08: 48 8d 55 18                 	leaq	0x18(%rbp), %rdx
140650d0c: 48 89 f1                    	movq	%rsi, %rcx
140650d0f: e8 dc 72 85 00              	callq	0x140ea7ff0 <.text+0xea6ff0>
140650d14: 90                          	nop
140650d15: 48 81 c4 18 01 00 00        	addq	$0x118, %rsp            # imm = 0x118
140650d1c: 5e                          	popq	%rsi
140650d1d: 5d                          	popq	%rbp
140650d1e: c3                          	retq
140650d1f: 90                          	nop
140650d20: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140650d25: 55                          	pushq	%rbp
140650d26: 56                          	pushq	%rsi
140650d27: 48 83 ec 28                 	subq	$0x28, %rsp
140650d2b: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140650d32: 48 8d 4d 18                 	leaq	0x18(%rbp), %rcx
140650d36: e8 45 8b 9d ff              	callq	0x140029880 <.text+0x28880>
140650d3b: 90                          	nop
140650d3c: 48 83 c4 28                 	addq	$0x28, %rsp
140650d40: 5e                          	popq	%rsi
140650d41: 5d                          	popq	%rbp
140650d42: c3                          	retq
