
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a1b5a0: 56                          	pushq	%rsi
140a1b5a1: 57                          	pushq	%rdi
140a1b5a2: 53                          	pushq	%rbx
140a1b5a3: 48 81 ec 90 00 00 00        	subq	$0x90, %rsp
140a1b5aa: 0f 29 b4 24 80 00 00 00     	movaps	%xmm6, 0x80(%rsp)
140a1b5b2: 48 89 ce                    	movq	%rcx, %rsi
140a1b5b5: 89 54 24 2c                 	movl	%edx, 0x2c(%rsp)
140a1b5b9: 48 8b 59 10                 	movq	0x10(%rcx), %rbx
140a1b5bd: 48 83 7b 18 00              	cmpq	$0x0, 0x18(%rbx)
140a1b5c2: 0f 84 bf 00 00 00           	je	0x140a1b687 <.text+0xa1a687>
140a1b5c8: 89 d7                       	movl	%edx, %edi
140a1b5ca: 0f 10 36                    	movups	(%rsi), %xmm6
140a1b5cd: 48 8d 4b 20                 	leaq	0x20(%rbx), %rcx
140a1b5d1: 48 8d 54 24 2c              	leaq	0x2c(%rsp), %rdx
140a1b5d6: e8 f5 58 b9 ff              	callq	0x1405b0ed0 <.text+0x5afed0>
140a1b5db: 48 8b 0b                    	movq	(%rbx), %rcx
140a1b5de: 48 8b 53 08                 	movq	0x8(%rbx), %rdx
140a1b5e2: 49 89 d0                    	movq	%rdx, %r8
140a1b5e5: 49 21 c0                    	andq	%rax, %r8
140a1b5e8: 48 c1 e8 39                 	shrq	$0x39, %rax
140a1b5ec: 66 0f 6e c0                 	movd	%eax, %xmm0
140a1b5f0: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140a1b5f4: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140a1b5f9: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140a1b5fe: 48 8d 41 e8                 	leaq	-0x18(%rcx), %rax
140a1b602: 45 31 c9                    	xorl	%r9d, %r9d
140a1b605: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
140a1b609: f3 42 0f 6f 14 01           	movdqu	(%rcx,%r8), %xmm2
140a1b60f: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
140a1b613: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
140a1b617: 66 44 0f d7 d3              	pmovmskb	%xmm3, %r10d
140a1b61c: 45 85 d2                    	testl	%r10d, %r10d
140a1b61f: 74 2f                       	je	0x140a1b650 <.text+0xa1a650>
140a1b621: f3 45 0f bc da              	tzcntl	%r10d, %r11d
140a1b626: 4d 01 c3                    	addq	%r8, %r11
140a1b629: 49 21 d3                    	andq	%rdx, %r11
140a1b62c: 49 f7 db                    	negq	%r11
140a1b62f: 4f 8d 1c 5b                 	leaq	(%r11,%r11,2), %r11
140a1b633: 42 39 3c d8                 	cmpl	%edi, (%rax,%r11,8)
140a1b637: 74 35                       	je	0x140a1b66e <.text+0xa1a66e>
140a1b639: 45 8d 5a ff                 	leal	-0x1(%r10), %r11d
140a1b63d: 66 45 21 d3                 	andw	%r10w, %r11w
140a1b641: 45 89 da                    	movl	%r11d, %r10d
140a1b644: 75 db                       	jne	0x140a1b621 <.text+0xa1a621>
140a1b646: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
140a1b650: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
140a1b654: 66 44 0f d7 d2              	pmovmskb	%xmm2, %r10d
140a1b659: 45 85 d2                    	testl	%r10d, %r10d
140a1b65c: 75 29                       	jne	0x140a1b687 <.text+0xa1a687>
140a1b65e: 4d 01 c8                    	addq	%r9, %r8
140a1b661: 49 83 c0 10                 	addq	$0x10, %r8
140a1b665: 49 83 c1 10                 	addq	$0x10, %r9
140a1b669: 49 21 d0                    	andq	%rdx, %r8
140a1b66c: eb 9b                       	jmp	0x140a1b609 <.text+0xa1a609>
140a1b66e: 42 0f 11 74 d9 f0           	movups	%xmm6, -0x10(%rcx,%r11,8)
140a1b674: 0f 28 b4 24 80 00 00 00     	movaps	0x80(%rsp), %xmm6
140a1b67c: 48 81 c4 90 00 00 00        	addq	$0x90, %rsp
140a1b683: 5b                          	popq	%rbx
140a1b684: 5f                          	popq	%rdi
140a1b685: 5e                          	popq	%rsi
140a1b686: c3                          	retq
140a1b687: 48 8d 44 24 2c              	leaq	0x2c(%rsp), %rax
140a1b68c: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
140a1b691: 48 8d 05 98 8c 00 00        	leaq	0x8c98(%rip), %rax      # 0x140a24330 <.text+0xa23330>
140a1b698: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
140a1b69d: 48 89 74 24 40              	movq	%rsi, 0x40(%rsp)
140a1b6a2: 48 8d 05 c7 8c 00 00        	leaq	0x8cc7(%rip), %rax      # 0x140a24370 <.text+0xa23370>
140a1b6a9: 48 89 44 24 48              	movq	%rax, 0x48(%rsp)
140a1b6ae: 48 8d 05 7b d9 0a 02        	leaq	0x20ad97b(%rip), %rax   # 0x142ac9030
140a1b6b5: 48 89 44 24 50              	movq	%rax, 0x50(%rsp)
140a1b6ba: 48 c7 44 24 58 02 00 00 00  	movq	$0x2, 0x58(%rsp)
140a1b6c3: 48 c7 44 24 70 00 00 00 00  	movq	$0x0, 0x70(%rsp)
140a1b6cc: 48 8d 44 24 30              	leaq	0x30(%rsp), %rax
140a1b6d1: 48 89 44 24 60              	movq	%rax, 0x60(%rsp)
140a1b6d6: 48 c7 44 24 68 02 00 00 00  	movq	$0x2, 0x68(%rsp)
140a1b6df: 48 8d 15 6a d9 0a 02        	leaq	0x20ad96a(%rip), %rdx   # 0x142ac9050
140a1b6e6: 48 8d 4c 24 50              	leaq	0x50(%rsp), %rcx
140a1b6eb: e8 a0 dc eb 01              	callq	0x1428d9390 <.text+0x28d8390>
140a1b6f0: cc                          	int3
