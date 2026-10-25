
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140f87860: 41 56                       	pushq	%r14
140f87862: 56                          	pushq	%rsi
140f87863: 57                          	pushq	%rdi
140f87864: 53                          	pushq	%rbx
140f87865: 48 83 ec 38                 	subq	$0x38, %rsp
140f87869: 44 89 c3                    	movl	%r8d, %ebx
140f8786c: 48 89 d7                    	movq	%rdx, %rdi
140f8786f: 48 89 ce                    	movq	%rcx, %rsi
140f87872: 0f 10 02                    	movups	(%rdx), %xmm0
140f87875: 0f 29 44 24 20              	movaps	%xmm0, 0x20(%rsp)
140f8787a: 44 88 44 24 30              	movb	%r8b, 0x30(%rsp)
140f8787f: 48 8d 4c 24 20              	leaq	0x20(%rsp), %rcx
140f87884: e8 87 40 4d 00              	callq	0x14145b910 <.text+0x145a910>
140f87889: 48 01 46 38                 	addq	%rax, 0x38(%rsi)
140f8788d: 48 31 46 40                 	xorq	%rax, 0x40(%rsi)
140f87891: 4c 8b 76 10                 	movq	0x10(%rsi), %r14
140f87895: 4c 3b 36                    	cmpq	(%rsi), %r14
140f87898: 75 0f                       	jne	0x140f878a9 <.text+0xf868a9>
140f8789a: 48 8d 15 47 5c c2 01        	leaq	0x1c25c47(%rip), %rdx   # 0x142bad4e8
140f878a1: 48 89 f1                    	movq	%rsi, %rcx
140f878a4: e8 57 bd 5d ff              	callq	0x140563600 <.text+0x562600>
140f878a9: 48 8b 46 08                 	movq	0x8(%rsi), %rax
140f878ad: 4b 8d 0c b6                 	leaq	(%r14,%r14,4), %rcx
140f878b1: 0f 10 07                    	movups	(%rdi), %xmm0
140f878b4: 0f 11 04 88                 	movups	%xmm0, (%rax,%rcx,4)
140f878b8: 88 5c 88 10                 	movb	%bl, 0x10(%rax,%rcx,4)
140f878bc: 49 ff c6                    	incq	%r14
140f878bf: 4c 89 76 10                 	movq	%r14, 0x10(%rsi)
140f878c3: 48 83 c4 38                 	addq	$0x38, %rsp
140f878c7: 5b                          	popq	%rbx
140f878c8: 5f                          	popq	%rdi
140f878c9: 5e                          	popq	%rsi
140f878ca: 41 5e                       	popq	%r14
140f878cc: c3                          	retq
140f878cd: cc                          	int3
140f878ce: cc                          	int3
140f878cf: cc                          	int3
140f878d0: 55                          	pushq	%rbp
140f878d1: 56                          	pushq	%rsi
140f878d2: 57                          	pushq	%rdi
140f878d3: 48 81 ec 00 03 00 00        	subq	$0x300, %rsp            # imm = 0x300
140f878da: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140f878e2: 48 c7 85 78 02 00 00 fe ff ff ff    	movq	$-0x2, 0x278(%rbp)
140f878ed: 48 89 d6                    	movq	%rdx, %rsi
140f878f0: 48 8b 39                    	movq	(%rcx), %rdi
140f878f3: 48 c7 85 50 02 00 00 03 00 00 00    	movq	$0x3, 0x250(%rbp)
140f878fe: c7 85 58 02 00 00 32 00 00 00       	movl	$0x32, 0x258(%rbp)
140f87908: 48 8d 05 f1 5b c2 01        	leaq	0x1c25bf1(%rip), %rax   # 0x142bad500
140f8790f: 48 89 85 40 02 00 00        	movq	%rax, 0x240(%rbp)
140f87916: 48 c7 85 48 02 00 00 1b 00 00 00    	movq	$0x1b, 0x248(%rbp)
140f87921: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
140f87928: 4c 8d 85 40 02 00 00        	leaq	0x240(%rbp), %r8
140f8792f: 48 89 fa                    	movq	%rdi, %rdx
140f87932: e8 e9 18 ad ff              	callq	0x140a59220 <.text+0xa58220>
140f87937: 31 c0                       	xorl	%eax, %eax
140f87939: 48 3b 85 f0 00 00 00        	cmpq	0xf0(%rbp), %rax
140f87940: 0f 80 db 03 00 00           	jo	0x140f87d21 <.text+0xf86d21>
140f87946: 48 8b 85 20 01 00 00        	movq	0x120(%rbp), %rax
140f8794d: 48 89 85 e0 00 00 00        	movq	%rax, 0xe0(%rbp)
140f87954: 0f 10 85 f0 00 00 00        	movups	0xf0(%rbp), %xmm0
140f8795b: 0f 10 8d 00 01 00 00        	movups	0x100(%rbp), %xmm1
140f87962: 0f 10 95 10 01 00 00        	movups	0x110(%rbp), %xmm2
140f87969: 0f 29 95 d0 00 00 00        	movaps	%xmm2, 0xd0(%rbp)
140f87970: 0f 29 8d c0 00 00 00        	movaps	%xmm1, 0xc0(%rbp)
140f87977: 0f 29 85 b0 00 00 00        	movaps	%xmm0, 0xb0(%rbp)
140f8797e: 48 c7 85 50 02 00 00 03 00 00 00    	movq	$0x3, 0x250(%rbp)
140f87989: c7 85 58 02 00 00 32 00 00 00       	movl	$0x32, 0x258(%rbp)
140f87993: 48 8d 05 de 5b c2 01        	leaq	0x1c25bde(%rip), %rax   # 0x142bad578
140f8799a: 48 89 85 40 02 00 00        	movq	%rax, 0x240(%rbp)
140f879a1: 48 c7 85 48 02 00 00 23 00 00 00    	movq	$0x23, 0x248(%rbp)
140f879ac: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
140f879b3: 4c 8d 85 40 02 00 00        	leaq	0x240(%rbp), %r8
140f879ba: 48 89 fa                    	movq	%rdi, %rdx
140f879bd: e8 5e 18 ad ff              	callq	0x140a59220 <.text+0xa58220>
140f879c2: 31 c0                       	xorl	%eax, %eax
140f879c4: 48 3b 85 f0 00 00 00        	cmpq	0xf0(%rbp), %rax
140f879cb: 0f 80 8e 03 00 00           	jo	0x140f87d5f <.text+0xf86d5f>
140f879d1: 48 8b 85 20 01 00 00        	movq	0x120(%rbp), %rax
140f879d8: 48 89 85 a0 00 00 00        	movq	%rax, 0xa0(%rbp)
140f879df: 0f 10 85 f0 00 00 00        	movups	0xf0(%rbp), %xmm0
140f879e6: 0f 10 8d 00 01 00 00        	movups	0x100(%rbp), %xmm1
140f879ed: 0f 10 95 10 01 00 00        	movups	0x110(%rbp), %xmm2
140f879f4: 0f 29 95 90 00 00 00        	movaps	%xmm2, 0x90(%rbp)
140f879fb: 0f 29 8d 80 00 00 00        	movaps	%xmm1, 0x80(%rbp)
140f87a02: 0f 29 45 70                 	movaps	%xmm0, 0x70(%rbp)
140f87a06: 48 c7 85 50 02 00 00 03 00 00 00    	movq	$0x3, 0x250(%rbp)
140f87a11: c7 85 58 02 00 00 32 00 00 00       	movl	$0x32, 0x258(%rbp)
140f87a1b: 48 8d 05 96 5b c2 01        	leaq	0x1c25b96(%rip), %rax   # 0x142bad5b8
