
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1402cf8a0:     	pushq	%rbp
1402cf8a1:     	pushq	%rsi
1402cf8a2:     	pushq	%rdi
1402cf8a3:     	subq	$0x1a0, %rsp            # imm = 0x1A0
1402cf8aa:     	leaq	0x80(%rsp), %rbp
1402cf8b2:     	movq	$-0x2, 0x118(%rbp)
1402cf8bd:     	movq	%rcx, %rsi
1402cf8c0:     	xorps	%xmm0, %xmm0
1402cf8c3:     	movups	%xmm0, -0x8(%rbp)
1402cf8c7:     	movups	%xmm0, 0x10(%rbp)
1402cf8cb:     	movups	%xmm0, 0x28(%rbp)
1402cf8cf:     	movups	%xmm0, 0x40(%rbp)
1402cf8d3:     	movups	%xmm0, 0x58(%rbp)
1402cf8d7:     	movl	$0x0, 0x68(%rbp)
1402cf8de:     	movups	%xmm0, 0x90(%rbp)
1402cf8e5:     	movups	%xmm0, 0xa8(%rbp)
1402cf8ec:     	movups	%xmm0, 0xc0(%rbp)
1402cf8f3:     	movups	%xmm0, 0xd8(%rbp)
1402cf8fa:     	movups	%xmm0, 0xf0(%rbp)
1402cf901:     	movl	$0x0, 0x100(%rbp)
1402cf90b:     	movq	$0x0, -0x58(%rbp)
1402cf913:     	movq	$0x0, -0x28(%rbp)
1402cf91b:     	movq	$0x8, -0x20(%rbp)
1402cf923:     	movq	$0x0, -0x18(%rbp)
1402cf92b:     	movq	$0x10, -0x10(%rbp)
1402cf933:     	movq	$0x10, 0x8(%rbp)
1402cf93b:     	movq	$0x10, 0x20(%rbp)
1402cf943:     	movq	$0x10, 0x38(%rbp)
1402cf94b:     	movq	$0x10, 0x50(%rbp)
1402cf953:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1402cf95d:     	movq	%rax, 0x70(%rbp)
1402cf961:     	leaq	0x26e2075(%rip), %rax   # 0x1429b19dd
1402cf968:     	movq	%rax, 0x78(%rbp)
1402cf96c:     	movq	$0x43, 0x80(%rbp)
1402cf977:     	movq	$0x10, 0x88(%rbp)
1402cf982:     	movq	$0x10, 0xa0(%rbp)
1402cf98d:     	movq	$0x10, 0xb8(%rbp)
1402cf998:     	movq	$0x10, 0xd0(%rbp)
1402cf9a3:     	movq	$0x10, 0xe8(%rbp)
1402cf9ae:     	movl	$0x0, 0x108(%rbp)
1402cf9b8:     	movw	$0x1, 0x10c(%rbp)
1402cf9c1:     	movl	$0x0, 0x110(%rbp)
1402cf9cb:     	movzbl	0x3365237(%rip), %eax   # 0x143634c09
1402cf9d2:     	movl	$0x170, %ecx            # imm = 0x170
1402cf9d7:     	movl	$0x8, %edx
1402cf9dc:     	callq	0x140613c10 <.text+0x612c10>
1402cf9e1:     	testq	%rax, %rax
1402cf9e4:     	je	0x1402cfa1b <.text+0x2cea1b>
1402cf9e6:     	movq	%rax, %rdi
1402cf9e9:     	leaq	-0x58(%rbp), %rdx
1402cf9ed:     	movl	$0x170, %r8d            # imm = 0x170
1402cf9f3:     	movq	%rax, %rcx
1402cf9f6:     	callq	0x142923c00 <.text+0x2922c00>
1402cf9fb:     	leaq	0x26e1e4e(%rip), %r8    # 0x1429b1850
1402cfa02:     	movq	%rsi, %rcx
1402cfa05:     	movq	%rdi, %rdx
1402cfa08:     	callq	0x140ce4b10 <.text+0xce3b10>
1402cfa0d:     	movq	%rsi, %rax
1402cfa10:     	addq	$0x1a0, %rsp            # imm = 0x1A0
1402cfa17:     	popq	%rdi
1402cfa18:     	popq	%rsi
1402cfa19:     	popq	%rbp
1402cfa1a:     	retq
1402cfa1b:     	movl	$0x8, %ecx
1402cfa20:     	movl	$0x170, %edx            # imm = 0x170
1402cfa25:     	callq	0x1428d9000 <.text+0x28d8000>
1402cfa2a:     	ud2
