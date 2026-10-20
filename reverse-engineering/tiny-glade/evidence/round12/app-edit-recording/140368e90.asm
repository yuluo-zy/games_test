
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140368e90:     	pushq	%rbp
140368e91:     	pushq	%rsi
140368e92:     	pushq	%rdi
140368e93:     	subq	$0x1a0, %rsp            # imm = 0x1A0
140368e9a:     	leaq	0x80(%rsp), %rbp
140368ea2:     	movq	$-0x2, 0x118(%rbp)
140368ead:     	movq	%rcx, %rsi
140368eb0:     	xorps	%xmm0, %xmm0
140368eb3:     	movups	%xmm0, -0x8(%rbp)
140368eb7:     	movups	%xmm0, 0x10(%rbp)
140368ebb:     	movups	%xmm0, 0x28(%rbp)
140368ebf:     	movups	%xmm0, 0x40(%rbp)
140368ec3:     	movups	%xmm0, 0x58(%rbp)
140368ec7:     	movl	$0x0, 0x68(%rbp)
140368ece:     	movups	%xmm0, 0x90(%rbp)
140368ed5:     	movups	%xmm0, 0xa8(%rbp)
140368edc:     	movups	%xmm0, 0xc0(%rbp)
140368ee3:     	movups	%xmm0, 0xd8(%rbp)
140368eea:     	movups	%xmm0, 0xf0(%rbp)
140368ef1:     	movl	$0x0, 0x100(%rbp)
140368efb:     	movq	$0x0, -0x58(%rbp)
140368f03:     	movq	$0x0, -0x28(%rbp)
140368f0b:     	movq	$0x8, -0x20(%rbp)
140368f13:     	movq	$0x0, -0x18(%rbp)
140368f1b:     	movq	$0x10, -0x10(%rbp)
140368f23:     	movq	$0x10, 0x8(%rbp)
140368f2b:     	movq	$0x10, 0x20(%rbp)
140368f33:     	movq	$0x10, 0x38(%rbp)
140368f3b:     	movq	$0x10, 0x50(%rbp)
140368f43:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
140368f4d:     	movq	%rax, 0x70(%rbp)
140368f51:     	leaq	0x265d9cc(%rip), %rax   # 0x1429c6924
140368f58:     	movq	%rax, 0x78(%rbp)
140368f5c:     	movq	$0x33, 0x80(%rbp)
140368f67:     	movq	$0x10, 0x88(%rbp)
140368f72:     	movq	$0x10, 0xa0(%rbp)
140368f7d:     	movq	$0x10, 0xb8(%rbp)
140368f88:     	movq	$0x10, 0xd0(%rbp)
140368f93:     	movq	$0x10, 0xe8(%rbp)
140368f9e:     	movl	$0x0, 0x108(%rbp)
140368fa8:     	movw	$0x1, 0x10c(%rbp)
140368fb1:     	movl	$0x0, 0x110(%rbp)
140368fbb:     	movzbl	0x32cbc47(%rip), %eax   # 0x143634c09
140368fc2:     	movl	$0x170, %ecx            # imm = 0x170
140368fc7:     	movl	$0x8, %edx
140368fcc:     	callq	0x140613c10 <.text+0x612c10>
140368fd1:     	testq	%rax, %rax
140368fd4:     	je	0x14036900b <.text+0x36800b>
140368fd6:     	movq	%rax, %rdi
140368fd9:     	leaq	-0x58(%rbp), %rdx
140368fdd:     	movl	$0x170, %r8d            # imm = 0x170
140368fe3:     	movq	%rax, %rcx
140368fe6:     	callq	0x142923c00 <.text+0x2922c00>
140368feb:     	leaq	0x265d62e(%rip), %r8    # 0x1429c6620
140368ff2:     	movq	%rsi, %rcx
140368ff5:     	movq	%rdi, %rdx
140368ff8:     	callq	0x140ce4b10 <.text+0xce3b10>
140368ffd:     	movq	%rsi, %rax
140369000:     	addq	$0x1a0, %rsp            # imm = 0x1A0
140369007:     	popq	%rdi
140369008:     	popq	%rsi
140369009:     	popq	%rbp
14036900a:     	retq
14036900b:     	movl	$0x8, %ecx
140369010:     	movl	$0x170, %edx            # imm = 0x170
140369015:     	callq	0x1428d9000 <.text+0x28d8000>
14036901a:     	ud2
