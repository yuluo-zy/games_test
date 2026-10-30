
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1403d68d0: 55                          	pushq	%rbp
1403d68d1: 56                          	pushq	%rsi
1403d68d2: 57                          	pushq	%rdi
1403d68d3: 48 81 ec 00 03 00 00        	subq	$0x300, %rsp            # imm = 0x300
1403d68da: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1403d68e2: 48 c7 85 78 02 00 00 fe ff ff ff    	movq	$-0x2, 0x278(%rbp)
1403d68ed: 48 89 ce                    	movq	%rcx, %rsi
1403d68f0: 0f 57 c0                    	xorps	%xmm0, %xmm0
1403d68f3: 0f 11 45 c8                 	movups	%xmm0, -0x38(%rbp)
1403d68f7: 0f 11 45 e0                 	movups	%xmm0, -0x20(%rbp)
1403d68fb: 0f 11 45 f8                 	movups	%xmm0, -0x8(%rbp)
1403d68ff: 0f 11 45 10                 	movups	%xmm0, 0x10(%rbp)
1403d6903: 0f 11 45 28                 	movups	%xmm0, 0x28(%rbp)
1403d6907: c7 45 38 00 00 00 00        	movl	$0x0, 0x38(%rbp)
1403d690e: 0f 11 45 60                 	movups	%xmm0, 0x60(%rbp)
1403d6912: 0f 11 45 78                 	movups	%xmm0, 0x78(%rbp)
1403d6916: 0f 11 85 90 00 00 00        	movups	%xmm0, 0x90(%rbp)
1403d691d: 0f 11 85 a8 00 00 00        	movups	%xmm0, 0xa8(%rbp)
1403d6924: 0f 11 85 c0 00 00 00        	movups	%xmm0, 0xc0(%rbp)
1403d692b: c7 85 d0 00 00 00 00 00 00 00       	movl	$0x0, 0xd0(%rbp)
1403d6935: 48 c7 45 a8 00 00 00 00     	movq	$0x0, -0x58(%rbp)
1403d693d: 48 c7 45 b0 08 00 00 00     	movq	$0x8, -0x50(%rbp)
1403d6945: 48 c7 45 b8 00 00 00 00     	movq	$0x0, -0x48(%rbp)
1403d694d: 48 c7 45 c0 10 00 00 00     	movq	$0x10, -0x40(%rbp)
1403d6955: 48 c7 45 d8 10 00 00 00     	movq	$0x10, -0x28(%rbp)
1403d695d: 48 c7 45 f0 10 00 00 00     	movq	$0x10, -0x10(%rbp)
1403d6965: 48 c7 45 08 10 00 00 00     	movq	$0x10, 0x8(%rbp)
1403d696d: 48 c7 45 20 10 00 00 00     	movq	$0x10, 0x20(%rbp)
1403d6975: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1403d697f: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
1403d6983: 48 8d 0d 33 c4 5f 02        	leaq	0x25fc433(%rip), %rcx   # 0x1429d2dbd
1403d698a: 48 89 4d 48                 	movq	%rcx, 0x48(%rbp)
1403d698e: 48 c7 45 50 30 00 00 00     	movq	$0x30, 0x50(%rbp)
1403d6996: 48 c7 45 58 10 00 00 00     	movq	$0x10, 0x58(%rbp)
1403d699e: 48 c7 45 70 10 00 00 00     	movq	$0x10, 0x70(%rbp)
1403d69a6: 48 c7 85 88 00 00 00 10 00 00 00    	movq	$0x10, 0x88(%rbp)
1403d69b1: 48 c7 85 a0 00 00 00 10 00 00 00    	movq	$0x10, 0xa0(%rbp)
1403d69bc: 48 c7 85 b8 00 00 00 10 00 00 00    	movq	$0x10, 0xb8(%rbp)
1403d69c7: c7 85 d8 00 00 00 00 00 00 00       	movl	$0x0, 0xd8(%rbp)
1403d69d1: 66 c7 85 dc 00 00 00 01 00  	movw	$0x1, 0xdc(%rbp)
1403d69da: 48 89 85 d8 01 00 00        	movq	%rax, 0x1d8(%rbp)
1403d69e1: c7 85 70 02 00 00 00 00 00 00       	movl	$0x0, 0x270(%rbp)
1403d69eb: 0f b6 05 17 e2 25 03        	movzbl	0x325e217(%rip), %eax   # 0x143634c09
1403d69f2: b9 d0 02 00 00              	movl	$0x2d0, %ecx            # imm = 0x2D0
1403d69f7: ba 08 00 00 00              	movl	$0x8, %edx
1403d69fc: e8 0f d2 23 00              	callq	0x140613c10 <.text+0x612c10>
1403d6a01: 48 85 c0                    	testq	%rax, %rax
1403d6a04: 74 35                       	je	0x1403d6a3b <.text+0x3d5a3b>
1403d6a06: 48 89 c7                    	movq	%rax, %rdi
1403d6a09: 48 8d 55 a8                 	leaq	-0x58(%rbp), %rdx
1403d6a0d: 41 b8 d0 02 00 00           	movl	$0x2d0, %r8d            # imm = 0x2D0
1403d6a13: 48 89 c1                    	movq	%rax, %rcx
1403d6a16: e8 e5 d1 54 02              	callq	0x142923c00 <.text+0x2922c00>
1403d6a1b: 4c 8d 05 e6 bf 5f 02        	leaq	0x25fbfe6(%rip), %r8    # 0x1429d2a08
1403d6a22: 48 89 f1                    	movq	%rsi, %rcx
1403d6a25: 48 89 fa                    	movq	%rdi, %rdx
1403d6a28: e8 e3 e0 90 00              	callq	0x140ce4b10 <.text+0xce3b10>
1403d6a2d: 48 89 f0                    	movq	%rsi, %rax
1403d6a30: 48 81 c4 00 03 00 00        	addq	$0x300, %rsp            # imm = 0x300
1403d6a37: 5f                          	popq	%rdi
1403d6a38: 5e                          	popq	%rsi
1403d6a39: 5d                          	popq	%rbp
1403d6a3a: c3                          	retq
1403d6a3b: b9 08 00 00 00              	movl	$0x8, %ecx
1403d6a40: ba d0 02 00 00              	movl	$0x2d0, %edx            # imm = 0x2D0
1403d6a45: e8 b6 25 50 02              	callq	0x1428d9000 <.text+0x28d8000>
