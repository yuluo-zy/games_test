
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141da88b0: 55                          	pushq	%rbp
141da88b1: 56                          	pushq	%rsi
141da88b2: 57                          	pushq	%rdi
141da88b3: 48 81 ec 00 03 00 00        	subq	$0x300, %rsp            # imm = 0x300
141da88ba: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
141da88c2: 48 c7 85 78 02 00 00 fe ff ff ff    	movq	$-0x2, 0x278(%rbp)
141da88cd: 48 89 ce                    	movq	%rcx, %rsi
141da88d0: 0f 57 c0                    	xorps	%xmm0, %xmm0
141da88d3: 0f 11 45 c8                 	movups	%xmm0, -0x38(%rbp)
141da88d7: 0f 11 45 e0                 	movups	%xmm0, -0x20(%rbp)
141da88db: 0f 11 45 f8                 	movups	%xmm0, -0x8(%rbp)
141da88df: 0f 11 45 10                 	movups	%xmm0, 0x10(%rbp)
141da88e3: 0f 11 45 28                 	movups	%xmm0, 0x28(%rbp)
141da88e7: c7 45 38 00 00 00 00        	movl	$0x0, 0x38(%rbp)
141da88ee: 0f 11 45 60                 	movups	%xmm0, 0x60(%rbp)
141da88f2: 0f 11 45 78                 	movups	%xmm0, 0x78(%rbp)
141da88f6: 0f 11 85 90 00 00 00        	movups	%xmm0, 0x90(%rbp)
141da88fd: 0f 11 85 a8 00 00 00        	movups	%xmm0, 0xa8(%rbp)
141da8904: 0f 11 85 c0 00 00 00        	movups	%xmm0, 0xc0(%rbp)
141da890b: c7 85 d0 00 00 00 00 00 00 00       	movl	$0x0, 0xd0(%rbp)
141da8915: 48 c7 45 a8 00 00 00 00     	movq	$0x0, -0x58(%rbp)
141da891d: 48 c7 45 b0 08 00 00 00     	movq	$0x8, -0x50(%rbp)
141da8925: 48 c7 45 b8 00 00 00 00     	movq	$0x0, -0x48(%rbp)
141da892d: 48 c7 45 c0 10 00 00 00     	movq	$0x10, -0x40(%rbp)
141da8935: 48 c7 45 d8 10 00 00 00     	movq	$0x10, -0x28(%rbp)
141da893d: 48 c7 45 f0 10 00 00 00     	movq	$0x10, -0x10(%rbp)
141da8945: 48 c7 45 08 10 00 00 00     	movq	$0x10, 0x8(%rbp)
141da894d: 48 c7 45 20 10 00 00 00     	movq	$0x10, 0x20(%rbp)
141da8955: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
141da895f: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
141da8963: 48 8d 0d 4b 50 1c 01        	leaq	0x11c504b(%rip), %rcx   # 0x142f6d9b5
141da896a: 48 89 4d 48                 	movq	%rcx, 0x48(%rbp)
141da896e: 48 c7 45 50 34 00 00 00     	movq	$0x34, 0x50(%rbp)
141da8976: 48 c7 45 58 10 00 00 00     	movq	$0x10, 0x58(%rbp)
141da897e: 48 c7 45 70 10 00 00 00     	movq	$0x10, 0x70(%rbp)
141da8986: 48 c7 85 88 00 00 00 10 00 00 00    	movq	$0x10, 0x88(%rbp)
141da8991: 48 c7 85 a0 00 00 00 10 00 00 00    	movq	$0x10, 0xa0(%rbp)
141da899c: 48 c7 85 b8 00 00 00 10 00 00 00    	movq	$0x10, 0xb8(%rbp)
141da89a7: c7 85 d8 00 00 00 00 00 00 00       	movl	$0x0, 0xd8(%rbp)
141da89b1: 66 c7 85 dc 00 00 00 01 00  	movw	$0x1, 0xdc(%rbp)
141da89ba: 48 89 85 d8 01 00 00        	movq	%rax, 0x1d8(%rbp)
141da89c1: c7 85 70 02 00 00 00 00 00 00       	movl	$0x0, 0x270(%rbp)
141da89cb: 0f b6 05 37 c2 88 01        	movzbl	0x188c237(%rip), %eax   # 0x143634c09
141da89d2: b9 d0 02 00 00              	movl	$0x2d0, %ecx            # imm = 0x2D0
141da89d7: ba 08 00 00 00              	movl	$0x8, %edx
141da89dc: e8 2f b2 86 fe              	callq	0x140613c10 <.text+0x612c10>
141da89e1: 48 85 c0                    	testq	%rax, %rax
141da89e4: 74 35                       	je	0x141da8a1b <.text+0x1da7a1b>
141da89e6: 48 89 c7                    	movq	%rax, %rdi
141da89e9: 48 8d 55 a8                 	leaq	-0x58(%rbp), %rdx
141da89ed: 41 b8 d0 02 00 00           	movl	$0x2d0, %r8d            # imm = 0x2D0
141da89f3: 48 89 c1                    	movq	%rax, %rcx
141da89f6: e8 05 b2 b7 00              	callq	0x142923c00 <.text+0x2922c00>
141da89fb: 4c 8d 05 de 4e 1c 01        	leaq	0x11c4ede(%rip), %r8    # 0x142f6d8e0
141da8a02: 48 89 f1                    	movq	%rsi, %rcx
141da8a05: 48 89 fa                    	movq	%rdi, %rdx
141da8a08: e8 03 c1 f3 fe              	callq	0x140ce4b10 <.text+0xce3b10>
141da8a0d: 48 89 f0                    	movq	%rsi, %rax
141da8a10: 48 81 c4 00 03 00 00        	addq	$0x300, %rsp            # imm = 0x300
141da8a17: 5f                          	popq	%rdi
141da8a18: 5e                          	popq	%rsi
141da8a19: 5d                          	popq	%rbp
141da8a1a: c3                          	retq
141da8a1b: b9 08 00 00 00              	movl	$0x8, %ecx
141da8a20: ba d0 02 00 00              	movl	$0x2d0, %edx            # imm = 0x2D0
141da8a25: e8 d6 05 b3 00              	callq	0x1428d9000 <.text+0x28d8000>
141da8a2a: 0f 0b                       	ud2
141da8a2c: 0f 1f 40 00                 	nopl	(%rax)
141da8a30: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141da8a35: 55                          	pushq	%rbp
141da8a36: 56                          	pushq	%rsi
141da8a37: 57                          	pushq	%rdi
141da8a38: 48 83 ec 20                 	subq	$0x20, %rsp
141da8a3c: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141da8a43: 48 8d 4d a8                 	leaq	-0x58(%rbp), %rcx
141da8a47: e8 64 69 82 fe              	callq	0x1405cf3b0 <.text+0x5ce3b0>
141da8a4c: 90                          	nop
141da8a4d: 48 83 c4 20                 	addq	$0x20, %rsp
141da8a51: 5f                          	popq	%rdi
141da8a52: 5e                          	popq	%rsi
141da8a53: 5d                          	popq	%rbp
141da8a54: c3                          	retq
