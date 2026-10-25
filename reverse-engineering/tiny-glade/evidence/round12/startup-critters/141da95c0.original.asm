
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141da95c0: 55                          	pushq	%rbp
141da95c1: 56                          	pushq	%rsi
141da95c2: 57                          	pushq	%rdi
141da95c3: 48 81 ec 20 04 00 00        	subq	$0x420, %rsp            # imm = 0x420
141da95ca: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
141da95d2: 48 c7 85 98 03 00 00 fe ff ff ff    	movq	$-0x2, 0x398(%rbp)
141da95dd: 48 89 ce                    	movq	%rcx, %rsi
141da95e0: 0f 57 c0                    	xorps	%xmm0, %xmm0
141da95e3: 0f 11 45 c0                 	movups	%xmm0, -0x40(%rbp)
141da95e7: 0f 11 45 d8                 	movups	%xmm0, -0x28(%rbp)
141da95eb: 0f 11 45 f0                 	movups	%xmm0, -0x10(%rbp)
141da95ef: 0f 11 45 08                 	movups	%xmm0, 0x8(%rbp)
141da95f3: 0f 11 45 20                 	movups	%xmm0, 0x20(%rbp)
141da95f7: c7 45 30 00 00 00 00        	movl	$0x0, 0x30(%rbp)
141da95fe: 0f 11 45 58                 	movups	%xmm0, 0x58(%rbp)
141da9602: 0f 11 45 70                 	movups	%xmm0, 0x70(%rbp)
141da9606: 0f 11 85 88 00 00 00        	movups	%xmm0, 0x88(%rbp)
141da960d: 0f 11 85 a0 00 00 00        	movups	%xmm0, 0xa0(%rbp)
141da9614: 0f 11 85 b8 00 00 00        	movups	%xmm0, 0xb8(%rbp)
141da961b: c7 85 c8 00 00 00 00 00 00 00       	movl	$0x0, 0xc8(%rbp)
141da9625: 48 c7 45 a0 00 00 00 00     	movq	$0x0, -0x60(%rbp)
141da962d: 48 c7 45 a8 08 00 00 00     	movq	$0x8, -0x58(%rbp)
141da9635: 48 c7 45 b0 00 00 00 00     	movq	$0x0, -0x50(%rbp)
141da963d: 48 c7 45 b8 10 00 00 00     	movq	$0x10, -0x48(%rbp)
141da9645: 48 c7 45 d0 10 00 00 00     	movq	$0x10, -0x30(%rbp)
141da964d: 48 c7 45 e8 10 00 00 00     	movq	$0x10, -0x18(%rbp)
141da9655: 48 c7 45 00 10 00 00 00     	movq	$0x10, (%rbp)
141da965d: 48 c7 45 18 10 00 00 00     	movq	$0x10, 0x18(%rbp)
141da9665: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
141da966f: 48 89 45 38                 	movq	%rax, 0x38(%rbp)
141da9673: 48 8d 0d cb 47 1c 01        	leaq	0x11c47cb(%rip), %rcx   # 0x142f6de45
141da967a: 48 89 4d 40                 	movq	%rcx, 0x40(%rbp)
141da967e: 48 c7 45 48 3b 00 00 00     	movq	$0x3b, 0x48(%rbp)
141da9686: 48 c7 45 50 10 00 00 00     	movq	$0x10, 0x50(%rbp)
141da968e: 48 c7 45 68 10 00 00 00     	movq	$0x10, 0x68(%rbp)
141da9696: 48 c7 85 80 00 00 00 10 00 00 00    	movq	$0x10, 0x80(%rbp)
141da96a1: 48 c7 85 98 00 00 00 10 00 00 00    	movq	$0x10, 0x98(%rbp)
141da96ac: 48 c7 85 b0 00 00 00 10 00 00 00    	movq	$0x10, 0xb0(%rbp)
141da96b7: c7 85 d0 00 00 00 00 00 00 00       	movl	$0x0, 0xd0(%rbp)
141da96c1: 66 c7 85 d4 00 00 00 01 00  	movw	$0x1, 0xd4(%rbp)
141da96ca: 48 89 85 d8 00 00 00        	movq	%rax, 0xd8(%rbp)
141da96d1: c7 85 90 03 00 00 00 00 00 00       	movl	$0x0, 0x390(%rbp)
141da96db: 0f b6 05 27 b5 88 01        	movzbl	0x188b527(%rip), %eax   # 0x143634c09
141da96e2: b9 f8 03 00 00              	movl	$0x3f8, %ecx            # imm = 0x3F8
141da96e7: ba 08 00 00 00              	movl	$0x8, %edx
141da96ec: e8 1f a5 86 fe              	callq	0x140613c10 <.text+0x612c10>
141da96f1: 48 85 c0                    	testq	%rax, %rax
141da96f4: 74 35                       	je	0x141da972b <.text+0x1da872b>
141da96f6: 48 89 c7                    	movq	%rax, %rdi
141da96f9: 48 8d 55 a0                 	leaq	-0x60(%rbp), %rdx
141da96fd: 41 b8 f8 03 00 00           	movl	$0x3f8, %r8d            # imm = 0x3F8
141da9703: 48 89 c1                    	movq	%rax, %rcx
141da9706: e8 f5 a4 b7 00              	callq	0x142923c00 <.text+0x2922c00>
141da970b: 4c 8d 05 5e 46 1c 01        	leaq	0x11c465e(%rip), %r8    # 0x142f6dd70
141da9712: 48 89 f1                    	movq	%rsi, %rcx
141da9715: 48 89 fa                    	movq	%rdi, %rdx
141da9718: e8 f3 b3 f3 fe              	callq	0x140ce4b10 <.text+0xce3b10>
141da971d: 48 89 f0                    	movq	%rsi, %rax
141da9720: 48 81 c4 20 04 00 00        	addq	$0x420, %rsp            # imm = 0x420
141da9727: 5f                          	popq	%rdi
141da9728: 5e                          	popq	%rsi
141da9729: 5d                          	popq	%rbp
141da972a: c3                          	retq
141da972b: b9 08 00 00 00              	movl	$0x8, %ecx
141da9730: ba f8 03 00 00              	movl	$0x3f8, %edx            # imm = 0x3F8
141da9735: e8 c6 f8 b2 00              	callq	0x1428d9000 <.text+0x28d8000>
141da973a: 0f 0b                       	ud2
141da973c: 0f 1f 40 00                 	nopl	(%rax)
141da9740: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141da9745: 55                          	pushq	%rbp
141da9746: 56                          	pushq	%rsi
141da9747: 57                          	pushq	%rdi
141da9748: 48 83 ec 20                 	subq	$0x20, %rsp
141da974c: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141da9753: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
141da9757: e8 c4 83 fd ff              	callq	0x141d81b20 <.text+0x1d80b20>
141da975c: 90                          	nop
141da975d: 48 83 c4 20                 	addq	$0x20, %rsp
141da9761: 5f                          	popq	%rdi
141da9762: 5e                          	popq	%rsi
141da9763: 5d                          	popq	%rbp
141da9764: c3                          	retq
