
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140c2c770: 55                          	pushq	%rbp
140c2c771: 56                          	pushq	%rsi
140c2c772: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
140c2c779: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140c2c781: 48 c7 45 40 fe ff ff ff     	movq	$-0x2, 0x40(%rbp)
140c2c789: 48 89 ce                    	movq	%rcx, %rsi
140c2c78c: 0f 10 05 3d 1a 0e 02        	movups	0x20e1a3d(%rip), %xmm0  # 0x142d0e1d0
140c2c793: 0f 29 45 30                 	movaps	%xmm0, 0x30(%rbp)
140c2c797: 0f 10 05 22 1a 0e 02        	movups	0x20e1a22(%rip), %xmm0  # 0x142d0e1c0
140c2c79e: 0f 29 45 20                 	movaps	%xmm0, 0x20(%rbp)
140c2c7a2: 48 c7 45 a0 ff ff ff ff     	movq	$-0x1, -0x60(%rbp)
140c2c7aa: 48 c7 45 a8 00 00 00 00     	movq	$0x0, -0x58(%rbp)
140c2c7b2: 48 c7 45 b0 01 00 00 00     	movq	$0x1, -0x50(%rbp)
140c2c7ba: 48 c7 45 b8 00 00 00 00     	movq	$0x0, -0x48(%rbp)
140c2c7c2: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
140c2c7cc: 48 89 45 c0                 	movq	%rax, -0x40(%rbp)
140c2c7d0: c7 45 d8 00 00 00 00        	movl	$0x0, -0x28(%rbp)
140c2c7d7: 48 c7 45 e0 00 00 00 00     	movq	$0x0, -0x20(%rbp)
140c2c7df: c7 45 e8 00 00 00 00        	movl	$0x0, -0x18(%rbp)
140c2c7e6: c7 45 f4 00 00 00 00        	movl	$0x0, -0xc(%rbp)
140c2c7ed: c7 45 08 00 00 00 00        	movl	$0x0, 0x8(%rbp)
140c2c7f4: c7 45 1c 02 02 02 02        	movl	$0x2020202, 0x1c(%rbp)  # imm = 0x2020202
140c2c7fb: 48 8d 4d 20                 	leaq	0x20(%rbp), %rcx
140c2c7ff: 48 8d 55 a0                 	leaq	-0x60(%rbp), %rdx
140c2c803: e8 a8 e2 01 00              	callq	0x140c4aab0 <.text+0xc49ab0>
140c2c808: 0f 28 45 20                 	movaps	0x20(%rbp), %xmm0
140c2c80c: 0f 28 4d 30                 	movaps	0x30(%rbp), %xmm1
140c2c810: 0f 11 8e 80 00 00 00        	movups	%xmm1, 0x80(%rsi)
140c2c817: 0f 11 46 70                 	movups	%xmm0, 0x70(%rsi)
140c2c81b: 48 c7 46 68 ff ff ff ff     	movq	$-0x1, 0x68(%rsi)
140c2c823: c7 86 90 00 00 00 00 00 00 00       	movl	$0x0, 0x90(%rsi)
140c2c82d: 48 c7 06 00 00 00 00        	movq	$0x0, (%rsi)
140c2c834: 48 c7 46 10 00 00 00 00     	movq	$0x0, 0x10(%rsi)
140c2c83c: 48 c7 86 a4 00 00 00 89 88 88 3c    	movq	$0x3c888889, 0xa4(%rsi) # imm = 0x3C888889
140c2c847: 48 c7 46 20 00 00 00 00     	movq	$0x0, 0x20(%rsi)
140c2c84f: 66 c7 86 ac 00 00 00 00 01  	movw	$0x100, 0xac(%rsi)      # imm = 0x100
140c2c858: 48 c7 46 28 08 00 00 00     	movq	$0x8, 0x28(%rsi)
140c2c860: 0f 57 c0                    	xorps	%xmm0, %xmm0
140c2c863: 0f 11 46 30                 	movups	%xmm0, 0x30(%rsi)
140c2c867: 48 c7 46 40 08 00 00 00     	movq	$0x8, 0x40(%rsi)
140c2c86f: 0f 11 46 48                 	movups	%xmm0, 0x48(%rsi)
