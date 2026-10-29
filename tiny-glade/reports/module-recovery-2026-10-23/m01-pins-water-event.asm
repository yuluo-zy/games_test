
/Users/yuluo/Desktop/game/B版本内容/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141ba9510: 01 00                       	addl	%eax, (%rax)
141ba9512: 00 48 8b                    	addb	%cl, -0x75(%rax)
141ba9515: 9d                          	popfq
141ba9516: 48 01 00                    	addq	%rax, (%rax)
141ba9519: 00 48 8d                    	addb	%cl, -0x73(%rax)
141ba951c: 4d 28 48 89                 	subb	%r9b, -0x77(%r8)
141ba9520: fa                          	cli
141ba9521: e8 2a 91 e6 fe              	callq	0x140a12650 <.text+0xa11650>
141ba9526: f3 0f 10 4d 28              	movss	0x28(%rbp), %xmm1
141ba952b: f3 0f 10 55 30              	movss	0x30(%rbp), %xmm2
141ba9530: 48 8d 4d 2c                 	leaq	0x2c(%rbp), %rcx
141ba9534: 0f 28 de                    	movaps	%xmm6, %xmm3
141ba9537: e8 04 e0 0e ff              	callq	0x140c97540 <.text+0xc96540>
141ba953c: 48 8d 4d b8                 	leaq	-0x48(%rbp), %rcx
141ba9540: 48 89 fa                    	movq	%rdi, %rdx
141ba9543: e8 08 91 e6 fe              	callq	0x140a12650 <.text+0xa11650>
141ba9548: f3 0f 10 45 bc              	movss	-0x44(%rbp), %xmm0
141ba954d: f3 0f 58 05 7b c3 d7 00     	addss	0xd7c37b(%rip), %xmm0   # 0x1429258d0
141ba9555: 48 c7 45 48 00 00 00 00     	movq	$0x0, 0x48(%rbp)
141ba955d: c7 45 3c 00 00 00 00        	movl	$0x0, 0x3c(%rbp)
141ba9564: f3 0f 11 45 40              	movss	%xmm0, 0x40(%rbp)
141ba9569: 66 c7 45 28 00 01           	movw	$0x100, 0x28(%rbp)      # imm = 0x100
141ba956f: 4c 8d 05 0a 27 34 01        	leaq	0x134270a(%rip), %r8    # 0x142eebc80
141ba9576: 48 8d 55 28                 	leaq	0x28(%rbp), %rdx
141ba957a: 48 89 d9                    	movq	%rbx, %rcx
141ba957d: e8 de 74 6b 00              	callq	0x142260a60 <.text+0x225fa60>
141ba9582: c7 45 d0 02 00 00 00        	movl	$0x2, -0x30(%rbp)
141ba9589: eb 07                       	jmp	0x141ba9592 <.text+0x1ba8592>
141ba958b: c7 45 d0 03 00 00 00        	movl	$0x3, -0x30(%rbp)
141ba9592: 4c 8b b5 10 01 00 00        	movq	0x110(%rbp), %r14
