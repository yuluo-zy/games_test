
/Users/yuluo/Desktop/game/B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a57300: 55                          	pushq	%rbp
140a57301: 56                          	pushq	%rsi
140a57302: 57                          	pushq	%rdi
140a57303: 48 81 ec b0 00 00 00        	subq	$0xb0, %rsp
140a5730a: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140a57312: 48 c7 45 28 fe ff ff ff     	movq	$-0x2, 0x28(%rbp)
140a5731a: f6 01 01                    	testb	$0x1, (%rcx)
140a5731d: 74 43                       	je	0x140a57362 <.text+0xa56362>
140a5731f: 48 89 ce                    	movq	%rcx, %rsi
140a57322: 48 83 c6 08                 	addq	$0x8, %rsi
140a57326: 48 8d 4d f0                 	leaq	-0x10(%rbp), %rcx
140a5732a: 48 89 f2                    	movq	%rsi, %rdx
140a5732d: e8 fe 03 9e 00              	callq	0x141437730 <.text+0x1436730>
140a57332: 48 bf 04 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffc, %rdi # imm = 0x8000000000000004
140a5733c: 48 39 7d f0                 	cmpq	%rdi, -0x10(%rbp)
140a57340: 75 2b                       	jne	0x140a5736d <.text+0xa5636d>
140a57342: 0f b6 45 f8                 	movzbl	-0x8(%rbp), %eax
140a57346: b9 0b 00 00 00              	movl	$0xb, %ecx
140a5734b: 0f a3 c1                    	btl	%eax, %ecx
140a5734e: 72 12                       	jb	0x140a57362 <.text+0xa56362>
140a57350: 48 8d 4d f0                 	leaq	-0x10(%rbp), %rcx
140a57354: 48 89 f2                    	movq	%rsi, %rdx
140a57357: e8 74 ff 9d 00              	callq	0x1414372d0 <.text+0x14362d0>
140a5735c: 48 39 7d f0                 	cmpq	%rdi, -0x10(%rbp)
140a57360: 75 55                       	jne	0x140a573b7 <.text+0xa563b7>
140a57362: 48 81 c4 b0 00 00 00        	addq	$0xb0, %rsp
140a57369: 5f                          	popq	%rdi
140a5736a: 5e                          	popq	%rsi
140a5736b: 5d                          	popq	%rbp
140a5736c: c3                          	retq
140a5736d: 48 8b 45 20                 	movq	0x20(%rbp), %rax
140a57371: 48 89 45 e0                 	movq	%rax, -0x20(%rbp)
140a57375: 0f 10 45 f0                 	movups	-0x10(%rbp), %xmm0
140a57379: 0f 10 4d 00                 	movups	(%rbp), %xmm1
140a5737d: 0f 10 55 10                 	movups	0x10(%rbp), %xmm2
140a57381: 0f 29 55 d0                 	movaps	%xmm2, -0x30(%rbp)
140a57385: 0f 29 4d c0                 	movaps	%xmm1, -0x40(%rbp)
140a57389: 0f 29 45 b0                 	movaps	%xmm0, -0x50(%rbp)
140a5738d: 48 8d 05 44 b8 07 02        	leaq	0x207b844(%rip), %rax   # 0x142ad2bd8
140a57394: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140a57399: 48 8d 0d 60 92 07 02        	leaq	0x2079260(%rip), %rcx   # 0x142ad0600
140a573a0: 4c 8d 0d 19 92 07 02        	leaq	0x2079219(%rip), %r9    # 0x142ad05c0
140a573a7: 4c 8d 45 b0                 	leaq	-0x50(%rbp), %r8
140a573ab: ba 2b 00 00 00              	movl	$0x2b, %edx
140a573b0: e8 ab 23 e8 01              	callq	0x1428d9760 <.text+0x28d8760>
140a573b5: eb 48                       	jmp	0x140a573ff <.text+0xa563ff>
140a573b7: 48 8b 45 20                 	movq	0x20(%rbp), %rax
140a573bb: 48 89 45 e0                 	movq	%rax, -0x20(%rbp)
140a573bf: 0f 10 45 f0                 	movups	-0x10(%rbp), %xmm0
140a573c3: 0f 10 4d 00                 	movups	(%rbp), %xmm1
140a573c7: 0f 10 55 10                 	movups	0x10(%rbp), %xmm2
140a573cb: 0f 29 55 d0                 	movaps	%xmm2, -0x30(%rbp)
140a573cf: 0f 29 4d c0                 	movaps	%xmm1, -0x40(%rbp)
140a573d3: 0f 29 45 b0                 	movaps	%xmm0, -0x50(%rbp)
140a573d7: 48 8d 05 12 b8 07 02        	leaq	0x207b812(%rip), %rax   # 0x142ad2bf0
140a573de: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140a573e3: 48 8d 0d 16 92 07 02        	leaq	0x2079216(%rip), %rcx   # 0x142ad0600
140a573ea: 4c 8d 0d cf 91 07 02        	leaq	0x20791cf(%rip), %r9    # 0x142ad05c0
140a573f1: 4c 8d 45 b0                 	leaq	-0x50(%rbp), %r8
140a573f5: ba 2b 00 00 00              	movl	$0x2b, %edx
140a573fa: e8 61 23 e8 01              	callq	0x1428d9760 <.text+0x28d8760>
140a573ff: 0f 0b                       	ud2
