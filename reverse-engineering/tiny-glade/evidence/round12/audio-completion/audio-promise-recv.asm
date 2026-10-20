
D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a57100: 55                          	pushq	%rbp
140a57101: 48 83 ec 70                 	subq	$0x70, %rsp
140a57105: 48 8d 6c 24 70              	leaq	0x70(%rsp), %rbp
140a5710a: 48 c7 45 f8 fe ff ff ff     	movq	$-0x2, -0x8(%rbp)
140a57112: 0f 10 01                    	movups	(%rcx), %xmm0
140a57115: 0f 10 49 10                 	movups	0x10(%rcx), %xmm1
140a57119: 0f 29 4d d0                 	movaps	%xmm1, -0x30(%rbp)
140a5711d: 0f 29 45 c0                 	movaps	%xmm0, -0x40(%rbp)
140a57121: 48 c7 01 00 00 00 00        	movq	$0x0, (%rcx)
140a57128: 48 8b 45 c0                 	movq	-0x40(%rbp), %rax
140a5712c: 48 83 f8 01                 	cmpq	$0x1, %rax
140a57130: 75 31                       	jne	0x140a57163 <.text+0xa56163>
140a57132: 48 8b 41 10                 	movq	0x10(%rcx), %rax
140a57136: 48 89 ca                    	movq	%rcx, %rdx
140a57139: 48 8b 49 18                 	movq	0x18(%rcx), %rcx
140a5713d: 80 7a 09 00                 	cmpb	$0x0, 0x9(%rdx)
140a57141: 75 61                       	jne	0x140a571a4 <.text+0xa561a4>
140a57143: 48 89 45 e0                 	movq	%rax, -0x20(%rbp)
140a57147: 48 89 4d e8                 	movq	%rcx, -0x18(%rbp)
140a5714b: 48 85 c0                    	testq	%rax, %rax
140a5714e: 74 28                       	je	0x140a57178 <.text+0xa56178>
140a57150: 48 83 f8 01                 	cmpq	$0x1, %rax
140a57154: 75 2f                       	jne	0x140a57185 <.text+0xa56185>
140a57156: 41 b8 00 ca 9a 3b           	movl	$0x3b9aca00, %r8d       # imm = 0x3B9ACA00
140a5715c: e8 3f d7 c1 00              	callq	0x1416748a0 <.text+0x16738a0>
140a57161: eb 2d                       	jmp	0x140a57190 <.text+0xa56190>
140a57163: 48 85 c0                    	testq	%rax, %rax
140a57166: 74 09                       	je	0x140a57171 <.text+0xa56171>
140a57168: 48 8d 4d c8                 	leaq	-0x38(%rbp), %rcx
140a5716c: e8 7f 8d e2 ff              	callq	0x14087fef0 <.text+0x87eef0>
140a57171: 90                          	nop
140a57172: 48 83 c4 70                 	addq	$0x70, %rsp
140a57176: 5d                          	popq	%rbp
140a57177: c3                          	retq
140a57178: 41 b8 00 ca 9a 3b           	movl	$0x3b9aca00, %r8d       # imm = 0x3B9ACA00
140a5717e: e8 ed 2b c2 00              	callq	0x141679d70 <.text+0x1678d70>
140a57183: eb 0b                       	jmp	0x140a57190 <.text+0xa56190>
140a57185: 41 b8 00 ca 9a 3b           	movl	$0x3b9aca00, %r8d       # imm = 0x3B9ACA00
140a5718b: e8 20 60 c7 00              	callq	0x1416cd1b0 <.text+0x16cc1b0>
140a57190: 3c 02                       	cmpb	$0x2, %al
140a57192: 75 42                       	jne	0x140a571d6 <.text+0xa561d6>
140a57194: 48 8d 4d e0                 	leaq	-0x20(%rbp), %rcx
140a57198: e8 73 23 e2 ff              	callq	0x140879510 <.text+0x878510>
140a5719d: 90                          	nop
140a5719e: 48 83 c4 70                 	addq	$0x70, %rsp
140a571a2: 5d                          	popq	%rbp
140a571a3: c3                          	retq
140a571a4: 48 89 45 e0                 	movq	%rax, -0x20(%rbp)
140a571a8: 48 89 4d e8                 	movq	%rcx, -0x18(%rbp)
140a571ac: 48 8d 05 dd b9 07 02        	leaq	0x207b9dd(%rip), %rax   # 0x142ad2b90
140a571b3: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140a571b8: 48 8d 0d 41 94 07 02        	leaq	0x2079441(%rip), %rcx   # 0x142ad0600
140a571bf: 4c 8d 0d 8a 94 07 02        	leaq	0x207948a(%rip), %r9    # 0x142ad0650
140a571c6: 4c 8d 45 e0                 	leaq	-0x20(%rbp), %r8
140a571ca: ba 2b 00 00 00              	movl	$0x2b, %edx
140a571cf: e8 8c 25 e8 01              	callq	0x1428d9760 <.text+0x28d8760>
140a571d4: eb 28                       	jmp	0x140a571fe <.text+0xa561fe>
140a571d6: 48 8d 05 cb b9 07 02        	leaq	0x207b9cb(%rip), %rax   # 0x142ad2ba8
140a571dd: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140a571e2: 48 8d 0d 17 94 07 02        	leaq	0x2079417(%rip), %rcx   # 0x142ad0600
140a571e9: 4c 8d 0d 40 94 07 02        	leaq	0x2079440(%rip), %r9    # 0x142ad0630
140a571f0: 4c 8d 45 f7                 	leaq	-0x9(%rbp), %r8
140a571f4: ba 2b 00 00 00              	movl	$0x2b, %edx
140a571f9: e8 62 25 e8 01              	callq	0x1428d9760 <.text+0x28d8760>
140a571fe: 0f 0b                       	ud2
140a57200: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a57205: 55                          	pushq	%rbp
140a57206: 48 83 ec 30                 	subq	$0x30, %rsp
140a5720a: 48 8d 6a 70                 	leaq	0x70(%rdx), %rbp
140a5720e: 48 8d 4d e0                 	leaq	-0x20(%rbp), %rcx
140a57212: e8 f9 22 e2 ff              	callq	0x140879510 <.text+0x878510>
140a57217: 90                          	nop
140a57218: 48 83 c4 30                 	addq	$0x30, %rsp
140a5721c: 5d                          	popq	%rbp
140a5721d: c3                          	retq
140a5721e: 66 90                       	nop
140a57220: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a57225: 55                          	pushq	%rbp
140a57226: 48 83 ec 30                 	subq	$0x30, %rsp
140a5722a: 48 8d 6a 70                 	leaq	0x70(%rdx), %rbp
140a5722e: 48 8d 4d e0                 	leaq	-0x20(%rbp), %rcx
140a57232: e8 d9 22 e2 ff              	callq	0x140879510 <.text+0x878510>
140a57237: 90                          	nop
140a57238: 48 83 c4 30                 	addq	$0x30, %rsp
140a5723c: 5d                          	popq	%rbp
140a5723d: c3                          	retq
140a5723e: cc                          	int3
140a5723f: cc                          	int3
