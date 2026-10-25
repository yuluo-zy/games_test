
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14033a260:     	pushq	%rbp
14033a261:     	pushq	%rsi
14033a262:     	pushq	%rdi
14033a263:     	subq	$0x1c0, %rsp            # imm = 0x1C0
14033a26a:     	leaq	0x80(%rsp), %rbp
14033a272:     	movq	$-0x2, 0x138(%rbp)
14033a27d:     	movq	%rcx, %rsi
14033a280:     	xorps	%xmm0, %xmm0
14033a283:     	movups	%xmm0, 0x18(%rbp)
14033a287:     	movups	%xmm0, 0x30(%rbp)
14033a28b:     	movups	%xmm0, 0x48(%rbp)
14033a28f:     	movups	%xmm0, 0x60(%rbp)
14033a293:     	movups	%xmm0, 0x78(%rbp)
14033a297:     	movl	$0x0, 0x88(%rbp)
14033a2a1:     	movups	%xmm0, 0xb0(%rbp)
14033a2a8:     	movups	%xmm0, 0xc8(%rbp)
14033a2af:     	movups	%xmm0, 0xe0(%rbp)
14033a2b6:     	movups	%xmm0, 0xf8(%rbp)
14033a2bd:     	movups	%xmm0, 0x110(%rbp)
14033a2c4:     	movl	$0x0, 0x120(%rbp)
14033a2ce:     	movq	$0x0, -0x60(%rbp)
14033a2d6:     	movq	$0x0, -0x8(%rbp)
14033a2de:     	movq	$0x8, (%rbp)
14033a2e6:     	movq	$0x0, 0x8(%rbp)
14033a2ee:     	movq	$0x10, 0x10(%rbp)
14033a2f6:     	movq	$0x10, 0x28(%rbp)
14033a2fe:     	movq	$0x10, 0x40(%rbp)
14033a306:     	movq	$0x10, 0x58(%rbp)
14033a30e:     	movq	$0x10, 0x70(%rbp)
14033a316:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14033a320:     	movq	%rax, 0x90(%rbp)
14033a327:     	leaq	0x26866c2(%rip), %rax   # 0x1429c09f0
14033a32e:     	movq	%rax, 0x98(%rbp)
14033a335:     	movq	$0x35, 0xa0(%rbp)
14033a340:     	movq	$0x10, 0xa8(%rbp)
14033a34b:     	movq	$0x10, 0xc0(%rbp)
14033a356:     	movq	$0x10, 0xd8(%rbp)
14033a361:     	movq	$0x10, 0xf0(%rbp)
14033a36c:     	movq	$0x10, 0x108(%rbp)
14033a377:     	movl	$0x0, 0x128(%rbp)
14033a381:     	movw	$0x1, 0x12c(%rbp)
14033a38a:     	movl	$0x0, 0x130(%rbp)
14033a394:     	movzbl	0x32fa86e(%rip), %eax   # 0x143634c09
14033a39b:     	movl	$0x198, %ecx            # imm = 0x198
14033a3a0:     	movl	$0x8, %edx
14033a3a5:     	callq	0x140613c10 <.text+0x612c10>
14033a3aa:     	testq	%rax, %rax
14033a3ad:     	je	0x14033a3e4 <.text+0x3393e4>
14033a3af:     	movq	%rax, %rdi
14033a3b2:     	leaq	-0x60(%rbp), %rdx
14033a3b6:     	movl	$0x198, %r8d            # imm = 0x198
14033a3bc:     	movq	%rax, %rcx
14033a3bf:     	callq	0x142923c00 <.text+0x2922c00>
14033a3c4:     	leaq	0x268616d(%rip), %r8    # 0x1429c0538
14033a3cb:     	movq	%rsi, %rcx
14033a3ce:     	movq	%rdi, %rdx
14033a3d1:     	callq	0x140ce4b10 <.text+0xce3b10>
14033a3d6:     	movq	%rsi, %rax
14033a3d9:     	addq	$0x1c0, %rsp            # imm = 0x1C0
14033a3e0:     	popq	%rdi
14033a3e1:     	popq	%rsi
14033a3e2:     	popq	%rbp
14033a3e3:     	retq
14033a3e4:     	movl	$0x8, %ecx
14033a3e9:     	movl	$0x198, %edx            # imm = 0x198
14033a3ee:     	callq	0x1428d9000 <.text+0x28d8000>
14033a3f3:     	ud2
