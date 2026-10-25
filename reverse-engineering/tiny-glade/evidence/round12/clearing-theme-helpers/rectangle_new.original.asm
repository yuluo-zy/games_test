
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140c98980:     	subq	$0x28, %rsp
140c98984:     	movd	%xmm1, %eax
140c98988:     	movq	0x68(%rsp), %r8
140c9898d:     	andl	$0x7fffffff, %eax       # imm = 0x7FFFFFFF
140c98992:     	cmpl	$0x7f800000, %eax       # imm = 0x7F800000
140c98997:     	jge	0x140c98a0e <.text+0xc97a0e>
140c98999:     	movd	%xmm2, %eax
140c9899d:     	andl	$0x7fffffff, %eax       # imm = 0x7FFFFFFF
140c989a2:     	cmpl	$0x7f800000, %eax       # imm = 0x7F800000
140c989a7:     	jge	0x140c98a1f <.text+0xc97a1f>
140c989a9:     	movss	0x50(%rsp), %xmm0
140c989af:     	movaps	%xmm3, %xmm4
140c989b2:     	mulss	%xmm3, %xmm4
140c989b6:     	movaps	%xmm0, %xmm5
140c989b9:     	mulss	%xmm0, %xmm5
140c989bd:     	addss	%xmm4, %xmm5
140c989c1:     	addss	0x1cc171b(%rip), %xmm5  # 0x14295a0e4
140c989c9:     	andps	0x1c95fb0(%rip), %xmm5  # 0x14292e980
140c989d0:     	movss	0x1e95b60(%rip), %xmm4  # 0x142b2e538
140c989d8:     	ucomiss	%xmm5, %xmm4
140c989db:     	jb	0x140c98a30 <.text+0xc97a30>
140c989dd:     	movss	0x60(%rsp), %xmm4
140c989e3:     	movss	0x58(%rsp), %xmm5
140c989e9:     	movss	%xmm3, (%rcx)
140c989ed:     	movss	%xmm0, 0x4(%rcx)
140c989f2:     	movd	%xmm1, 0x10(%rcx)
140c989f7:     	movd	%xmm2, 0x14(%rcx)
140c989fc:     	movss	%xmm5, 0x8(%rcx)
140c98a01:     	movss	%xmm4, 0xc(%rcx)
140c98a06:     	movq	%rcx, %rax
140c98a09:     	addq	$0x28, %rsp
140c98a0d:     	retq
140c98a0e:     	leaq	0x1e95be3(%rip), %rcx   # 0x142b2e5f8
140c98a15:     	movl	$0x23, %edx
140c98a1a:     	callq	0x1428d9430 <.text+0x28d8430>
140c98a1f:     	leaq	0x1e95bf5(%rip), %rcx   # 0x142b2e61b
140c98a26:     	movl	$0x24, %edx
140c98a2b:     	callq	0x1428d9430 <.text+0x28d8430>
140c98a30:     	leaq	0x1e95c08(%rip), %rcx   # 0x142b2e63f
140c98a37:     	movl	$0x2d, %edx
140c98a3c:     	callq	0x1428d9430 <.text+0x28d8430>
140c98a41:     	int3
