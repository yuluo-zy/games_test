
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14025fbd0:     	pushq	%rbp
14025fbd1:     	pushq	%rsi
14025fbd2:     	pushq	%rdi
14025fbd3:     	subq	$0x1a0, %rsp            # imm = 0x1A0
14025fbda:     	leaq	0x80(%rsp), %rbp
14025fbe2:     	movq	$-0x2, 0x118(%rbp)
14025fbed:     	movq	%rcx, %rsi
14025fbf0:     	xorps	%xmm0, %xmm0
14025fbf3:     	movups	%xmm0, -0x8(%rbp)
14025fbf7:     	movups	%xmm0, 0x10(%rbp)
14025fbfb:     	movups	%xmm0, 0x28(%rbp)
14025fbff:     	movups	%xmm0, 0x40(%rbp)
14025fc03:     	movups	%xmm0, 0x58(%rbp)
14025fc07:     	movl	$0x0, 0x68(%rbp)
14025fc0e:     	movups	%xmm0, 0x90(%rbp)
14025fc15:     	movups	%xmm0, 0xa8(%rbp)
14025fc1c:     	movups	%xmm0, 0xc0(%rbp)
14025fc23:     	movups	%xmm0, 0xd8(%rbp)
14025fc2a:     	movups	%xmm0, 0xf0(%rbp)
14025fc31:     	movl	$0x0, 0x100(%rbp)
14025fc3b:     	movq	$0x0, -0x60(%rbp)
14025fc43:     	movq	$0x0, -0x28(%rbp)
14025fc4b:     	movq	$0x8, -0x20(%rbp)
14025fc53:     	movq	$0x0, -0x18(%rbp)
14025fc5b:     	movq	$0x10, -0x10(%rbp)
14025fc63:     	movq	$0x10, 0x8(%rbp)
14025fc6b:     	movq	$0x10, 0x20(%rbp)
14025fc73:     	movq	$0x10, 0x38(%rbp)
14025fc7b:     	movq	$0x10, 0x50(%rbp)
14025fc83:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14025fc8d:     	movq	%rax, 0x70(%rbp)
14025fc91:     	leaq	0x2746425(%rip), %rax   # 0x1429a60bd
14025fc98:     	movq	%rax, 0x78(%rbp)
14025fc9c:     	movq	$0x3b, 0x80(%rbp)
14025fca7:     	movq	$0x10, 0x88(%rbp)
14025fcb2:     	movq	$0x10, 0xa0(%rbp)
14025fcbd:     	movq	$0x10, 0xb8(%rbp)
14025fcc8:     	movq	$0x10, 0xd0(%rbp)
14025fcd3:     	movq	$0x10, 0xe8(%rbp)
14025fcde:     	movl	$0x0, 0x108(%rbp)
14025fce8:     	movw	$0x1, 0x10c(%rbp)
14025fcf1:     	movl	$0x0, 0x110(%rbp)
14025fcfb:     	movzbl	0x33d4f07(%rip), %eax   # 0x143634c09
14025fd02:     	movl	$0x178, %ecx            # imm = 0x178
14025fd07:     	movl	$0x8, %edx
14025fd0c:     	callq	0x140613c10 <.text+0x612c10>
14025fd11:     	testq	%rax, %rax
14025fd14:     	je	0x14025fd4b <.text+0x25ed4b>
14025fd16:     	movq	%rax, %rdi
14025fd19:     	leaq	-0x60(%rbp), %rdx
14025fd1d:     	movl	$0x178, %r8d            # imm = 0x178
14025fd23:     	movq	%rax, %rcx
14025fd26:     	callq	0x142923c00 <.text+0x2922c00>
14025fd2b:     	leaq	0x27461e6(%rip), %r8    # 0x1429a5f18
14025fd32:     	movq	%rsi, %rcx
14025fd35:     	movq	%rdi, %rdx
14025fd38:     	callq	0x140ce4b10 <.text+0xce3b10>
14025fd3d:     	movq	%rsi, %rax
14025fd40:     	addq	$0x1a0, %rsp            # imm = 0x1A0
14025fd47:     	popq	%rdi
14025fd48:     	popq	%rsi
14025fd49:     	popq	%rbp
14025fd4a:     	retq
14025fd4b:     	movl	$0x8, %ecx
14025fd50:     	movl	$0x178, %edx            # imm = 0x178
14025fd55:     	callq	0x1428d9000 <.text+0x28d8000>
14025fd5a:     	ud2
