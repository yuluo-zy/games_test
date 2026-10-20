
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141dabd90: 55                          	pushq	%rbp
141dabd91: 56                          	pushq	%rsi
141dabd92: 57                          	pushq	%rdi
141dabd93: 48 81 ec 00 03 00 00        	subq	$0x300, %rsp            # imm = 0x300
141dabd9a: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
141dabda2: 48 c7 85 78 02 00 00 fe ff ff ff    	movq	$-0x2, 0x278(%rbp)
141dabdad: 48 89 ce                    	movq	%rcx, %rsi
141dabdb0: 0f 57 c0                    	xorps	%xmm0, %xmm0
141dabdb3: 0f 11 45 c0                 	movups	%xmm0, -0x40(%rbp)
141dabdb7: 0f 11 45 d8                 	movups	%xmm0, -0x28(%rbp)
141dabdbb: 0f 11 45 f0                 	movups	%xmm0, -0x10(%rbp)
141dabdbf: 0f 11 45 08                 	movups	%xmm0, 0x8(%rbp)
141dabdc3: 0f 11 45 20                 	movups	%xmm0, 0x20(%rbp)
141dabdc7: c7 45 30 00 00 00 00        	movl	$0x0, 0x30(%rbp)
141dabdce: 0f 11 45 58                 	movups	%xmm0, 0x58(%rbp)
141dabdd2: 0f 11 45 70                 	movups	%xmm0, 0x70(%rbp)
141dabdd6: 0f 11 85 88 00 00 00        	movups	%xmm0, 0x88(%rbp)
141dabddd: 0f 11 85 a0 00 00 00        	movups	%xmm0, 0xa0(%rbp)
141dabde4: 0f 11 85 b8 00 00 00        	movups	%xmm0, 0xb8(%rbp)
141dabdeb: c7 85 c8 00 00 00 00 00 00 00       	movl	$0x0, 0xc8(%rbp)
141dabdf5: 48 c7 45 a0 00 00 00 00     	movq	$0x0, -0x60(%rbp)
141dabdfd: 48 c7 45 a8 08 00 00 00     	movq	$0x8, -0x58(%rbp)
141dabe05: 48 c7 45 b0 00 00 00 00     	movq	$0x0, -0x50(%rbp)
141dabe0d: 48 c7 45 b8 10 00 00 00     	movq	$0x10, -0x48(%rbp)
141dabe15: 48 c7 45 d0 10 00 00 00     	movq	$0x10, -0x30(%rbp)
141dabe1d: 48 c7 45 e8 10 00 00 00     	movq	$0x10, -0x18(%rbp)
141dabe25: 48 c7 45 00 10 00 00 00     	movq	$0x10, (%rbp)
141dabe2d: 48 c7 45 18 10 00 00 00     	movq	$0x10, 0x18(%rbp)
141dabe35: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
141dabe3f: 48 89 45 38                 	movq	%rax, 0x38(%rbp)
141dabe43: 48 8d 0d 95 28 1c 01        	leaq	0x11c2895(%rip), %rcx   # 0x142f6e6df
141dabe4a: 48 89 4d 40                 	movq	%rcx, 0x40(%rbp)
141dabe4e: 48 c7 45 48 32 00 00 00     	movq	$0x32, 0x48(%rbp)
141dabe56: 48 c7 45 50 10 00 00 00     	movq	$0x10, 0x50(%rbp)
141dabe5e: 48 c7 45 68 10 00 00 00     	movq	$0x10, 0x68(%rbp)
141dabe66: 48 c7 85 80 00 00 00 10 00 00 00    	movq	$0x10, 0x80(%rbp)
141dabe71: 48 c7 85 98 00 00 00 10 00 00 00    	movq	$0x10, 0x98(%rbp)
141dabe7c: 48 c7 85 b0 00 00 00 10 00 00 00    	movq	$0x10, 0xb0(%rbp)
141dabe87: c7 85 d0 00 00 00 00 00 00 00       	movl	$0x0, 0xd0(%rbp)
141dabe91: 66 c7 85 d4 00 00 00 01 00  	movw	$0x1, 0xd4(%rbp)
141dabe9a: 48 89 85 18 01 00 00        	movq	%rax, 0x118(%rbp)
141dabea1: c7 85 70 02 00 00 00 00 00 00       	movl	$0x0, 0x270(%rbp)
141dabeab: 0f b6 05 57 8d 88 01        	movzbl	0x1888d57(%rip), %eax   # 0x143634c09
141dabeb2: b9 d8 02 00 00              	movl	$0x2d8, %ecx            # imm = 0x2D8
141dabeb7: ba 08 00 00 00              	movl	$0x8, %edx
141dabebc: e8 4f 7d 86 fe              	callq	0x140613c10 <.text+0x612c10>
141dabec1: 48 85 c0                    	testq	%rax, %rax
141dabec4: 74 35                       	je	0x141dabefb <.text+0x1daaefb>
141dabec6: 48 89 c7                    	movq	%rax, %rdi
141dabec9: 48 8d 55 a0                 	leaq	-0x60(%rbp), %rdx
141dabecd: 41 b8 d8 02 00 00           	movl	$0x2d8, %r8d            # imm = 0x2D8
141dabed3: 48 89 c1                    	movq	%rax, %rcx
141dabed6: e8 25 7d b7 00              	callq	0x142923c00 <.text+0x2922c00>
141dabedb: 4c 8d 05 3e 26 1c 01        	leaq	0x11c263e(%rip), %r8    # 0x142f6e520
141dabee2: 48 89 f1                    	movq	%rsi, %rcx
141dabee5: 48 89 fa                    	movq	%rdi, %rdx
141dabee8: e8 23 8c f3 fe              	callq	0x140ce4b10 <.text+0xce3b10>
141dabeed: 48 89 f0                    	movq	%rsi, %rax
141dabef0: 48 81 c4 00 03 00 00        	addq	$0x300, %rsp            # imm = 0x300
141dabef7: 5f                          	popq	%rdi
141dabef8: 5e                          	popq	%rsi
141dabef9: 5d                          	popq	%rbp
141dabefa: c3                          	retq
141dabefb: b9 08 00 00 00              	movl	$0x8, %ecx
141dabf00: ba d8 02 00 00              	movl	$0x2d8, %edx            # imm = 0x2D8
141dabf05: e8 f6 d0 b2 00              	callq	0x1428d9000 <.text+0x28d8000>
141dabf0a: 0f 0b                       	ud2
141dabf0c: 0f 1f 40 00                 	nopl	(%rax)
141dabf10: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141dabf15: 55                          	pushq	%rbp
141dabf16: 56                          	pushq	%rsi
141dabf17: 57                          	pushq	%rdi
141dabf18: 48 83 ec 20                 	subq	$0x20, %rsp
141dabf1c: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141dabf23: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
141dabf27: e8 b4 b2 ac fe              	callq	0x1408771e0 <.text+0x8761e0>
141dabf2c: 90                          	nop
141dabf2d: 48 83 c4 20                 	addq	$0x20, %rsp
141dabf31: 5f                          	popq	%rdi
141dabf32: 5e                          	popq	%rsi
141dabf33: 5d                          	popq	%rbp
141dabf34: c3                          	retq
