
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140174db0:     	pushq	%rbp
140174db1:     	pushq	%rsi
140174db2:     	pushq	%rdi
140174db3:     	subq	$0x1a0, %rsp            # imm = 0x1A0
140174dba:     	leaq	0x80(%rsp), %rbp
140174dc2:     	movq	$-0x2, 0x118(%rbp)
140174dcd:     	movq	%rcx, %rsi
140174dd0:     	xorps	%xmm0, %xmm0
140174dd3:     	movups	%xmm0, -0x8(%rbp)
140174dd7:     	movups	%xmm0, 0x10(%rbp)
140174ddb:     	movups	%xmm0, 0x28(%rbp)
140174ddf:     	movups	%xmm0, 0x40(%rbp)
140174de3:     	movups	%xmm0, 0x58(%rbp)
140174de7:     	movl	$0x0, 0x68(%rbp)
140174dee:     	movups	%xmm0, 0x90(%rbp)
140174df5:     	movups	%xmm0, 0xa8(%rbp)
140174dfc:     	movups	%xmm0, 0xc0(%rbp)
140174e03:     	movups	%xmm0, 0xd8(%rbp)
140174e0a:     	movups	%xmm0, 0xf0(%rbp)
140174e11:     	movl	$0x0, 0x100(%rbp)
140174e1b:     	movq	$0x0, -0x60(%rbp)
140174e23:     	movq	$0x0, -0x28(%rbp)
140174e2b:     	movq	$0x8, -0x20(%rbp)
140174e33:     	movq	$0x0, -0x18(%rbp)
140174e3b:     	movq	$0x10, -0x10(%rbp)
140174e43:     	movq	$0x10, 0x8(%rbp)
140174e4b:     	movq	$0x10, 0x20(%rbp)
140174e53:     	movq	$0x10, 0x38(%rbp)
140174e5b:     	movq	$0x10, 0x50(%rbp)
140174e63:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
140174e6d:     	movq	%rax, 0x70(%rbp)
140174e71:     	leaq	0x27eafe1(%rip), %rax   # 0x14295fe59
140174e78:     	movq	%rax, 0x78(%rbp)
140174e7c:     	movq	$0x69, 0x80(%rbp)
140174e87:     	movq	$0x10, 0x88(%rbp)
140174e92:     	movq	$0x10, 0xa0(%rbp)
140174e9d:     	movq	$0x10, 0xb8(%rbp)
140174ea8:     	movq	$0x10, 0xd0(%rbp)
140174eb3:     	movq	$0x10, 0xe8(%rbp)
140174ebe:     	movl	$0x0, 0x108(%rbp)
140174ec8:     	movw	$0x1, 0x10c(%rbp)
140174ed1:     	movl	$0x0, 0x110(%rbp)
140174edb:     	movzbl	0x34bfd27(%rip), %eax   # 0x143634c09
140174ee2:     	movl	$0x178, %ecx            # imm = 0x178
140174ee7:     	movl	$0x8, %edx
140174eec:     	callq	0x140613c10 <.text+0x612c10>
140174ef1:     	testq	%rax, %rax
140174ef4:     	je	0x140174f2b <.text+0x173f2b>
140174ef6:     	movq	%rax, %rdi
140174ef9:     	leaq	-0x60(%rbp), %rdx
140174efd:     	movl	$0x178, %r8d            # imm = 0x178
140174f03:     	movq	%rax, %rcx
140174f06:     	callq	0x142923c00 <.text+0x2922c00>
140174f0b:     	leaq	0x27eace6(%rip), %r8    # 0x14295fbf8
140174f12:     	movq	%rsi, %rcx
140174f15:     	movq	%rdi, %rdx
140174f18:     	callq	0x140ce4b10 <.text+0xce3b10>
140174f1d:     	movq	%rsi, %rax
140174f20:     	addq	$0x1a0, %rsp            # imm = 0x1A0
140174f27:     	popq	%rdi
140174f28:     	popq	%rsi
140174f29:     	popq	%rbp
140174f2a:     	retq
140174f2b:     	movl	$0x8, %ecx
140174f30:     	movl	$0x178, %edx            # imm = 0x178
140174f35:     	callq	0x1428d9000 <.text+0x28d8000>
140174f3a:     	ud2
