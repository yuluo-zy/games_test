
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140531f10:     	pushq	%rbp
140531f11:     	pushq	%rsi
140531f12:     	pushq	%rdi
140531f13:     	subq	$0x200, %rsp            # imm = 0x200
140531f1a:     	leaq	0x80(%rsp), %rbp
140531f22:     	movq	$-0x2, 0x178(%rbp)
140531f2d:     	movq	%rcx, %rsi
140531f30:     	xorps	%xmm0, %xmm0
140531f33:     	movups	%xmm0, 0x58(%rbp)
140531f37:     	movups	%xmm0, 0x70(%rbp)
140531f3b:     	movups	%xmm0, 0x88(%rbp)
140531f42:     	movups	%xmm0, 0xa0(%rbp)
140531f49:     	movups	%xmm0, 0xb8(%rbp)
140531f50:     	movl	$0x0, 0xc8(%rbp)
140531f5a:     	movups	%xmm0, 0xf0(%rbp)
140531f61:     	movups	%xmm0, 0x108(%rbp)
140531f68:     	movups	%xmm0, 0x120(%rbp)
140531f6f:     	movups	%xmm0, 0x138(%rbp)
140531f76:     	movups	%xmm0, 0x150(%rbp)
140531f7d:     	movl	$0x0, 0x160(%rbp)
140531f87:     	movq	$0x0, -0x60(%rbp)
140531f8f:     	movq	$0x0, 0x38(%rbp)
140531f97:     	movq	$0x8, 0x40(%rbp)
140531f9f:     	movq	$0x0, 0x48(%rbp)
140531fa7:     	movq	$0x10, 0x50(%rbp)
140531faf:     	movq	$0x10, 0x68(%rbp)
140531fb7:     	movq	$0x10, 0x80(%rbp)
140531fc2:     	movq	$0x10, 0x98(%rbp)
140531fcd:     	movq	$0x10, 0xb0(%rbp)
140531fd8:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
140531fe2:     	movq	%rax, 0xd0(%rbp)
140531fe9:     	leaq	0x24c1dad(%rip), %rax   # 0x1429f3d9d
140531ff0:     	movq	%rax, 0xd8(%rbp)
140531ff7:     	movq	$0x5d, 0xe0(%rbp)
140532002:     	movq	$0x10, 0xe8(%rbp)
14053200d:     	movq	$0x10, 0x100(%rbp)
140532018:     	movq	$0x10, 0x118(%rbp)
140532023:     	movq	$0x10, 0x130(%rbp)
14053202e:     	movq	$0x10, 0x148(%rbp)
140532039:     	movl	$0x0, 0x168(%rbp)
140532043:     	movw	$0x1, 0x16c(%rbp)
14053204c:     	movl	$0x0, 0x170(%rbp)
140532056:     	movzbl	0x3102bac(%rip), %eax   # 0x143634c09
14053205d:     	movl	$0x1d8, %ecx            # imm = 0x1D8
140532062:     	movl	$0x8, %edx
140532067:     	callq	0x140613c10 <.text+0x612c10>
14053206c:     	testq	%rax, %rax
14053206f:     	je	0x1405320a6 <.text+0x5310a6>
140532071:     	movq	%rax, %rdi
140532074:     	leaq	-0x60(%rbp), %rdx
140532078:     	movl	$0x1d8, %r8d            # imm = 0x1D8
14053207e:     	movq	%rax, %rcx
140532081:     	callq	0x142923c00 <.text+0x2922c00>
140532086:     	leaq	0x24c1c3b(%rip), %r8    # 0x1429f3cc8
14053208d:     	movq	%rsi, %rcx
140532090:     	movq	%rdi, %rdx
140532093:     	callq	0x140ce4b10 <.text+0xce3b10>
140532098:     	movq	%rsi, %rax
14053209b:     	addq	$0x200, %rsp            # imm = 0x200
1405320a2:     	popq	%rdi
1405320a3:     	popq	%rsi
1405320a4:     	popq	%rbp
1405320a5:     	retq
1405320a6:     	movl	$0x8, %ecx
1405320ab:     	movl	$0x1d8, %edx            # imm = 0x1D8
1405320b0:     	callq	0x1428d9000 <.text+0x28d8000>
1405320b5:     	ud2
