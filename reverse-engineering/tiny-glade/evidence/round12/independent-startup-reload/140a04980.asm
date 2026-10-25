
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a04980: 55                          	pushq	%rbp
140a04981: 56                          	pushq	%rsi
140a04982: 57                          	pushq	%rdi
140a04983: 48 81 ec c0 02 00 00        	subq	$0x2c0, %rsp            # imm = 0x2C0
140a0498a: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140a04992: 48 c7 85 38 02 00 00 fe ff ff ff    	movq	$-0x2, 0x238(%rbp)
140a0499d: 48 89 ce                    	movq	%rcx, %rsi
140a049a0: 0f 57 c0                    	xorps	%xmm0, %xmm0
140a049a3: 0f 11 45 c0                 	movups	%xmm0, -0x40(%rbp)
140a049a7: 0f 11 45 d8                 	movups	%xmm0, -0x28(%rbp)
140a049ab: 0f 11 45 f0                 	movups	%xmm0, -0x10(%rbp)
140a049af: 0f 11 45 08                 	movups	%xmm0, 0x8(%rbp)
140a049b3: 0f 11 45 20                 	movups	%xmm0, 0x20(%rbp)
140a049b7: c7 45 30 00 00 00 00        	movl	$0x0, 0x30(%rbp)
140a049be: 0f 11 45 58                 	movups	%xmm0, 0x58(%rbp)
140a049c2: 0f 11 45 70                 	movups	%xmm0, 0x70(%rbp)
140a049c6: 0f 11 85 88 00 00 00        	movups	%xmm0, 0x88(%rbp)
140a049cd: 0f 11 85 a0 00 00 00        	movups	%xmm0, 0xa0(%rbp)
140a049d4: 0f 11 85 b8 00 00 00        	movups	%xmm0, 0xb8(%rbp)
140a049db: c7 85 c8 00 00 00 00 00 00 00       	movl	$0x0, 0xc8(%rbp)
140a049e5: 48 c7 45 a0 00 00 00 00     	movq	$0x0, -0x60(%rbp)
140a049ed: 48 c7 45 a8 08 00 00 00     	movq	$0x8, -0x58(%rbp)
140a049f5: 48 c7 45 b0 00 00 00 00     	movq	$0x0, -0x50(%rbp)
140a049fd: 48 c7 45 b8 10 00 00 00     	movq	$0x10, -0x48(%rbp)
140a04a05: 48 c7 45 d0 10 00 00 00     	movq	$0x10, -0x30(%rbp)
140a04a0d: 48 c7 45 e8 10 00 00 00     	movq	$0x10, -0x18(%rbp)
140a04a15: 48 c7 45 00 10 00 00 00     	movq	$0x10, (%rbp)
140a04a1d: 48 c7 45 18 10 00 00 00     	movq	$0x10, 0x18(%rbp)
140a04a25: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
140a04a2f: 48 89 45 38                 	movq	%rax, 0x38(%rbp)
140a04a33: 48 8d 0d 37 1a 0c 02        	leaq	0x20c1a37(%rip), %rcx   # 0x142ac6471
140a04a3a: 48 89 4d 40                 	movq	%rcx, 0x40(%rbp)
140a04a3e: 48 c7 45 48 36 00 00 00     	movq	$0x36, 0x48(%rbp)
140a04a46: 48 c7 45 50 10 00 00 00     	movq	$0x10, 0x50(%rbp)
140a04a4e: 48 c7 45 68 10 00 00 00     	movq	$0x10, 0x68(%rbp)
140a04a56: 48 c7 85 80 00 00 00 10 00 00 00    	movq	$0x10, 0x80(%rbp)
140a04a61: 48 c7 85 98 00 00 00 10 00 00 00    	movq	$0x10, 0x98(%rbp)
140a04a6c: 48 c7 85 b0 00 00 00 10 00 00 00    	movq	$0x10, 0xb0(%rbp)
140a04a77: c7 85 d0 00 00 00 00 00 00 00       	movl	$0x0, 0xd0(%rbp)
140a04a81: 66 c7 85 d4 00 00 00 01 00  	movw	$0x1, 0xd4(%rbp)
140a04a8a: 48 89 85 d0 01 00 00        	movq	%rax, 0x1d0(%rbp)
140a04a91: c7 85 30 02 00 00 00 00 00 00       	movl	$0x0, 0x230(%rbp)
140a04a9b: 0f b6 05 67 01 c3 02        	movzbl	0x2c30167(%rip), %eax   # 0x143634c09
140a04aa2: b9 98 02 00 00              	movl	$0x298, %ecx            # imm = 0x298
140a04aa7: ba 08 00 00 00              	movl	$0x8, %edx
140a04aac: e8 5f f1 c0 ff              	callq	0x140613c10 <.text+0x612c10>
140a04ab1: 48 85 c0                    	testq	%rax, %rax
140a04ab4: 74 35                       	je	0x140a04aeb <.text+0xa03aeb>
140a04ab6: 48 89 c7                    	movq	%rax, %rdi
140a04ab9: 48 8d 55 a0                 	leaq	-0x60(%rbp), %rdx
140a04abd: 41 b8 98 02 00 00           	movl	$0x298, %r8d            # imm = 0x298
140a04ac3: 48 89 c1                    	movq	%rax, %rcx
140a04ac6: e8 35 f1 f1 01              	callq	0x142923c00 <.text+0x2922c00>
140a04acb: 4c 8d 05 8e 16 0c 02        	leaq	0x20c168e(%rip), %r8    # 0x142ac6160
140a04ad2: 48 89 f1                    	movq	%rsi, %rcx
140a04ad5: 48 89 fa                    	movq	%rdi, %rdx
140a04ad8: e8 33 00 2e 00              	callq	0x140ce4b10 <.text+0xce3b10>
140a04add: 48 89 f0                    	movq	%rsi, %rax
140a04ae0: 48 81 c4 c0 02 00 00        	addq	$0x2c0, %rsp            # imm = 0x2C0
140a04ae7: 5f                          	popq	%rdi
140a04ae8: 5e                          	popq	%rsi
140a04ae9: 5d                          	popq	%rbp
140a04aea: c3                          	retq
140a04aeb: b9 08 00 00 00              	movl	$0x8, %ecx
140a04af0: ba 98 02 00 00              	movl	$0x298, %edx            # imm = 0x298
140a04af5: e8 06 45 ed 01              	callq	0x1428d9000 <.text+0x28d8000>
140a04afa: 0f 0b                       	ud2
140a04afc: 0f 1f 40 00                 	nopl	(%rax)
140a04b00: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a04b05: 55                          	pushq	%rbp
140a04b06: 56                          	pushq	%rsi
140a04b07: 57                          	pushq	%rdi
140a04b08: 48 83 ec 20                 	subq	$0x20, %rsp
140a04b0c: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a04b13: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
140a04b17: e8 d4 22 e8 ff              	callq	0x140886df0 <.text+0x885df0>
140a04b1c: 90                          	nop
140a04b1d: 48 83 c4 20                 	addq	$0x20, %rsp
140a04b21: 5f                          	popq	%rdi
140a04b22: 5e                          	popq	%rsi
140a04b23: 5d                          	popq	%rbp
140a04b24: c3                          	retq
