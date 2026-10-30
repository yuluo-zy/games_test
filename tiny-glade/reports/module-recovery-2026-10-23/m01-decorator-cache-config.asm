
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1400c6200: 55                          	pushq	%rbp
1400c6201: 56                          	pushq	%rsi
1400c6202: 57                          	pushq	%rdi
1400c6203: 48 81 ec 00 03 00 00        	subq	$0x300, %rsp            # imm = 0x300
1400c620a: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1400c6212: 48 c7 85 78 02 00 00 fe ff ff ff    	movq	$-0x2, 0x278(%rbp)
1400c621d: 48 89 ce                    	movq	%rcx, %rsi
1400c6220: 0f 57 c0                    	xorps	%xmm0, %xmm0
1400c6223: 0f 11 45 c8                 	movups	%xmm0, -0x38(%rbp)
1400c6227: 0f 11 45 e0                 	movups	%xmm0, -0x20(%rbp)
1400c622b: 0f 11 45 f8                 	movups	%xmm0, -0x8(%rbp)
1400c622f: 0f 11 45 10                 	movups	%xmm0, 0x10(%rbp)
1400c6233: 0f 11 45 28                 	movups	%xmm0, 0x28(%rbp)
1400c6237: c7 45 38 00 00 00 00        	movl	$0x0, 0x38(%rbp)
1400c623e: 0f 11 45 60                 	movups	%xmm0, 0x60(%rbp)
1400c6242: 0f 11 45 78                 	movups	%xmm0, 0x78(%rbp)
1400c6246: 0f 11 85 90 00 00 00        	movups	%xmm0, 0x90(%rbp)
1400c624d: 0f 11 85 a8 00 00 00        	movups	%xmm0, 0xa8(%rbp)
1400c6254: 0f 11 85 c0 00 00 00        	movups	%xmm0, 0xc0(%rbp)
1400c625b: c7 85 d0 00 00 00 00 00 00 00       	movl	$0x0, 0xd0(%rbp)
1400c6265: 48 c7 45 a8 00 00 00 00     	movq	$0x0, -0x58(%rbp)
1400c626d: 48 c7 45 b0 08 00 00 00     	movq	$0x8, -0x50(%rbp)
1400c6275: 48 c7 45 b8 00 00 00 00     	movq	$0x0, -0x48(%rbp)
1400c627d: 48 c7 45 c0 10 00 00 00     	movq	$0x10, -0x40(%rbp)
1400c6285: 48 c7 45 d8 10 00 00 00     	movq	$0x10, -0x28(%rbp)
1400c628d: 48 c7 45 f0 10 00 00 00     	movq	$0x10, -0x10(%rbp)
1400c6295: 48 c7 45 08 10 00 00 00     	movq	$0x10, 0x8(%rbp)
1400c629d: 48 c7 45 20 10 00 00 00     	movq	$0x10, 0x20(%rbp)
1400c62a5: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1400c62af: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
1400c62b3: 48 8d 0d 90 fd 86 02        	leaq	0x286fd90(%rip), %rcx   # 0x14293604a
1400c62ba: 48 89 4d 48                 	movq	%rcx, 0x48(%rbp)
1400c62be: 48 c7 45 50 48 00 00 00     	movq	$0x48, 0x50(%rbp)
1400c62c6: 48 c7 45 58 10 00 00 00     	movq	$0x10, 0x58(%rbp)
1400c62ce: 48 c7 45 70 10 00 00 00     	movq	$0x10, 0x70(%rbp)
1400c62d6: 48 c7 85 88 00 00 00 10 00 00 00    	movq	$0x10, 0x88(%rbp)
1400c62e1: 48 c7 85 a0 00 00 00 10 00 00 00    	movq	$0x10, 0xa0(%rbp)
1400c62ec: 48 c7 85 b8 00 00 00 10 00 00 00    	movq	$0x10, 0xb8(%rbp)
1400c62f7: c7 85 d8 00 00 00 00 00 00 00       	movl	$0x0, 0xd8(%rbp)
1400c6301: 66 c7 85 dc 00 00 00 01 00  	movw	$0x1, 0xdc(%rbp)
1400c630a: 48 89 85 d8 01 00 00        	movq	%rax, 0x1d8(%rbp)
1400c6311: c7 85 70 02 00 00 00 00 00 00       	movl	$0x0, 0x270(%rbp)
1400c631b: 0f b6 05 e7 e8 56 03        	movzbl	0x356e8e7(%rip), %eax   # 0x143634c09
1400c6322: b9 d0 02 00 00              	movl	$0x2d0, %ecx            # imm = 0x2D0
1400c6327: ba 08 00 00 00              	movl	$0x8, %edx
1400c632c: e8 df d8 54 00              	callq	0x140613c10 <.text+0x612c10>
1400c6331: 48 85 c0                    	testq	%rax, %rax
1400c6334: 74 35                       	je	0x1400c636b <.text+0xc536b>
1400c6336: 48 89 c7                    	movq	%rax, %rdi
1400c6339: 48 8d 55 a8                 	leaq	-0x58(%rbp), %rdx
1400c633d: 41 b8 d0 02 00 00           	movl	$0x2d0, %r8d            # imm = 0x2D0
1400c6343: 48 89 c1                    	movq	%rax, %rcx
1400c6346: e8 b5 d8 85 02              	callq	0x142923c00 <.text+0x2922c00>
1400c634b: 4c 8d 05 ee fa 86 02        	leaq	0x286faee(%rip), %r8    # 0x142935e40
1400c6352: 48 89 f1                    	movq	%rsi, %rcx
1400c6355: 48 89 fa                    	movq	%rdi, %rdx
1400c6358: e8 b3 e7 c1 00              	callq	0x140ce4b10 <.text+0xce3b10>
1400c635d: 48 89 f0                    	movq	%rsi, %rax
1400c6360: 48 81 c4 00 03 00 00        	addq	$0x300, %rsp            # imm = 0x300
1400c6367: 5f                          	popq	%rdi
1400c6368: 5e                          	popq	%rsi
1400c6369: 5d                          	popq	%rbp
1400c636a: c3                          	retq
1400c636b: b9 08 00 00 00              	movl	$0x8, %ecx
1400c6370: ba d0 02 00 00              	movl	$0x2d0, %edx            # imm = 0x2D0
1400c6375: e8 86 2c 81 02              	callq	0x1428d9000 <.text+0x28d8000>
