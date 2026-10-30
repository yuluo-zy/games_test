
/Users/yuluo/Desktop/game/B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14089a280: 55                          	pushq	%rbp
14089a281: 56                          	pushq	%rsi
14089a282: 48 81 ec a8 00 00 00        	subq	$0xa8, %rsp
14089a289: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
14089a291: 48 c7 45 20 fe ff ff ff     	movq	$-0x2, 0x20(%rbp)
14089a299: 48 8b 41 08                 	movq	0x8(%rcx), %rax
14089a29d: 48 83 39 00                 	cmpq	$0x0, (%rcx)
14089a2a1: 48 c7 01 00 00 00 00        	movq	$0x0, (%rcx)
14089a2a8: 74 4c                       	je	0x14089a2f6 <.text+0x8992f6>
14089a2aa: 48 89 45 18                 	movq	%rax, 0x18(%rbp)
14089a2ae: 44 0f b6 41 28              	movzbl	0x28(%rcx), %r8d
14089a2b3: 48 be 04 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffc, %rsi # imm = 0x8000000000000004
14089a2bd: 41 80 f8 02                 	cmpb	$0x2, %r8b
14089a2c1: 74 20                       	je	0x14089a2e3 <.text+0x8992e3>
14089a2c3: 41 80 e0 01                 	andb	$0x1, %r8b
14089a2c7: 48 8d 4d e0                 	leaq	-0x20(%rbp), %rcx
14089a2cb: 48 8d 55 18                 	leaq	0x18(%rbp), %rdx
14089a2cf: e8 6c d1 b9 00              	callq	0x141437440 <.text+0x1436440>
14089a2d4: 48 39 75 e0                 	cmpq	%rsi, -0x20(%rbp)
14089a2d8: 74 09                       	je	0x14089a2e3 <.text+0x8992e3>
14089a2da: 48 8d 4d e0                 	leaq	-0x20(%rbp), %rcx
14089a2de: e8 4d c8 fe ff              	callq	0x140886b30 <.text+0x885b30>
14089a2e3: 48 8d 4d a8                 	leaq	-0x58(%rbp), %rcx
14089a2e7: 48 8d 55 18                 	leaq	0x18(%rbp), %rdx
14089a2eb: e8 00 d6 b9 00              	callq	0x1414378f0 <.text+0x14368f0>
14089a2f0: 48 39 75 a8                 	cmpq	%rsi, -0x58(%rbp)
14089a2f4: 75 0a                       	jne	0x14089a300 <.text+0x899300>
14089a2f6: 48 81 c4 a8 00 00 00        	addq	$0xa8, %rsp
14089a2fd: 5e                          	popq	%rsi
14089a2fe: 5d                          	popq	%rbp
14089a2ff: c3                          	retq
14089a300: 48 8b 45 d8                 	movq	-0x28(%rbp), %rax
14089a304: 48 89 45 10                 	movq	%rax, 0x10(%rbp)
14089a308: 0f 10 45 a8                 	movups	-0x58(%rbp), %xmm0
14089a30c: 0f 10 4d b8                 	movups	-0x48(%rbp), %xmm1
14089a310: 0f 10 55 c8                 	movups	-0x38(%rbp), %xmm2
14089a314: 0f 29 55 00                 	movaps	%xmm2, (%rbp)
14089a318: 0f 29 4d f0                 	movaps	%xmm1, -0x10(%rbp)
14089a31c: 0f 29 45 e0                 	movaps	%xmm0, -0x20(%rbp)
14089a320: 48 8d 05 21 e7 1f 02        	leaq	0x21fe721(%rip), %rax   # 0x142a98a48
14089a327: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14089a32c: 48 8d 0d 0d e1 1f 02        	leaq	0x21fe10d(%rip), %rcx   # 0x142a98440
14089a333: 4c 8d 0d 36 e1 1f 02        	leaq	0x21fe136(%rip), %r9    # 0x142a98470
14089a33a: 4c 8d 45 e0                 	leaq	-0x20(%rbp), %r8
14089a33e: ba 2b 00 00 00              	movl	$0x2b, %edx
14089a343: e8 18 f4 03 02              	callq	0x1428d9760 <.text+0x28d8760>
14089a348: 0f 0b                       	ud2
