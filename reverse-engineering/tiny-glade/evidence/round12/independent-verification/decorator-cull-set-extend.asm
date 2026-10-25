
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140b76260: 41 57                       	pushq	%r15
140b76262: 41 56                       	pushq	%r14
140b76264: 56                          	pushq	%rsi
140b76265: 57                          	pushq	%rdi
140b76266: 55                          	pushq	%rbp
140b76267: 53                          	pushq	%rbx
140b76268: 48 83 ec 28                 	subq	$0x28, %rsp
140b7626c: 48 89 d6                    	movq	%rdx, %rsi
140b7626f: 48 89 cf                    	movq	%rcx, %rdi
140b76272: 0f b7 41 18                 	movzwl	0x18(%rcx), %eax
140b76276: 48 8b 19                    	movq	(%rcx), %rbx
140b76279: 4c 8b 71 08                 	movq	0x8(%rcx), %r14
140b7627d: 4d 8b 38                    	movq	(%r8), %r15
140b76280: eb 40                       	jmp	0x140b762c2 <.text+0xb752c2>
140b76282: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
140b76290: f7 d0                       	notl	%eax
140b76292: 48 89 1f                    	movq	%rbx, (%rdi)
140b76295: 4c 89 77 08                 	movq	%r14, 0x8(%rdi)
140b76299: 8d 68 ff                    	leal	-0x1(%rax), %ebp
140b7629c: f3 0f bc c8                 	tzcntl	%eax, %ecx
140b762a0: 21 c5                       	andl	%eax, %ebp
140b762a2: 66 89 6f 18                 	movw	%bp, 0x18(%rdi)
140b762a6: 48 f7 d9                    	negq	%rcx
140b762a9: 48 8d 04 89                 	leaq	(%rcx,%rcx,4), %rax
140b762ad: 48 c1 e0 04                 	shlq	$0x4, %rax
140b762b1: 8b 54 03 b0                 	movl	-0x50(%rbx,%rax), %edx
140b762b5: 49 8b 0f                    	movq	(%r15), %rcx
140b762b8: e8 43 2f 75 ff              	callq	0x1402c9200 <.text+0x2c8200>
140b762bd: 48 ff ce                    	decq	%rsi
140b762c0: 89 e8                       	movl	%ebp, %eax
140b762c2: 66 85 c0                    	testw	%ax, %ax
140b762c5: 75 d2                       	jne	0x140b76299 <.text+0xb75299>
140b762c7: 48 85 f6                    	testq	%rsi, %rsi
140b762ca: 74 21                       	je	0x140b762ed <.text+0xb752ed>
140b762cc: 0f 1f 40 00                 	nopl	(%rax)
140b762d0: 66 41 0f 6f 06              	movdqa	(%r14), %xmm0
140b762d5: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
140b762d9: 48 81 c3 00 fb ff ff        	addq	$-0x500, %rbx           # imm = 0xFB00
140b762e0: 49 83 c6 10                 	addq	$0x10, %r14
140b762e4: 3d ff ff 00 00              	cmpl	$0xffff, %eax           # imm = 0xFFFF
140b762e9: 74 e5                       	je	0x140b762d0 <.text+0xb752d0>
140b762eb: eb a3                       	jmp	0x140b76290 <.text+0xb75290>
140b762ed: 48 83 c4 28                 	addq	$0x28, %rsp
140b762f1: 5b                          	popq	%rbx
140b762f2: 5d                          	popq	%rbp
140b762f3: 5f                          	popq	%rdi
140b762f4: 5e                          	popq	%rsi
140b762f5: 41 5e                       	popq	%r14
140b762f7: 41 5f                       	popq	%r15
140b762f9: c3                          	retq
