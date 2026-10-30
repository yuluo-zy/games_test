
../B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1407de8b0:     	pushq	%r15
1407de8b2:     	pushq	%r14
1407de8b4:     	pushq	%r12
1407de8b6:     	pushq	%rsi
1407de8b7:     	pushq	%rdi
1407de8b8:     	pushq	%rbx
1407de8b9:     	subq	$0x1d8, %rsp            # imm = 0x1D8
1407de8c0:     	movq	0x240(%rsp), %r10
1407de8c8:     	movq	$0x2, 0x28(%rsp)
1407de8d1:     	movq	$0x3, 0x30(%rsp)
1407de8da:     	movq	$0x6, 0x38(%rsp)
1407de8e3:     	movq	$0x3, 0x40(%rsp)
1407de8ec:     	movq	$0x7, 0x48(%rsp)
1407de8f5:     	movq	$0x6, 0x50(%rsp)
1407de8fe:     	movq	$0x0, 0x58(%rsp)
1407de907:     	movq	$0x4, 0x60(%rsp)
1407de910:     	movq	$0x1, 0x68(%rsp)
1407de919:     	movq	$0x1, 0x70(%rsp)
1407de922:     	movq	$0x4, 0x78(%rsp)
1407de92b:     	movq	$0x5, 0x80(%rsp)
1407de937:     	movq	$0x2, 0x88(%rsp)
1407de943:     	movq	$0x3, 0x90(%rsp)
1407de94f:     	movq	$0x6, 0x98(%rsp)
1407de95b:     	movq	$0x3, 0xa0(%rsp)
1407de967:     	movq	$0x7, 0xa8(%rsp)
1407de973:     	movq	$0x6, 0xb0(%rsp)
1407de97f:     	movq	$0x0, 0xb8(%rsp)
1407de98b:     	movq	$0x2, 0xc0(%rsp)
1407de997:     	movq	$0x4, 0xc8(%rsp)
1407de9a3:     	movq	$0x2, 0xd0(%rsp)
1407de9af:     	movq	$0x6, 0xd8(%rsp)
1407de9bb:     	movq	$0x4, 0xe0(%rsp)
1407de9c7:     	movq	$0x1, 0xe8(%rsp)
1407de9d3:     	movq	$0x5, 0xf0(%rsp)
1407de9df:     	movq	$0x3, 0xf8(%rsp)
1407de9eb:     	movq	$0x3, 0x100(%rsp)
1407de9f7:     	movq	$0x5, 0x108(%rsp)
1407dea03:     	movq	$0x7, 0x110(%rsp)
1407dea0f:     	testq	%r10, %r10
1407dea12:     	je	0x1407deb3e <.text+0x7ddb3e>
1407dea18:     	movq	0x258(%rsp), %r11
1407dea20:     	movq	0x250(%rsp), %rsi
1407dea28:     	movq	0x248(%rsp), %rax
1407dea30:     	movq	0x238(%rsp), %rdi
1407dea38:     	movq	0x230(%rsp), %rbx
1407dea40:     	cmpq	%rdx, %r9
1407dea43:     	movq	%rdx, %r14
1407dea46:     	cmovbq	%r9, %r14
1407dea4a:     	cmpq	%r14, %rdi
1407dea4d:     	movq	%r14, %r15
1407dea50:     	cmovbq	%rdi, %r15
1407dea54:     	leaq	(%rbx,%rdi,4), %rdi
1407dea58:     	leaq	(%r8,%r9,8), %r9
1407dea5c:     	leaq	(%rdx,%rdx,2), %r12
1407dea60:     	leaq	(%rcx,%r12,4), %r12
1407dea64:     	decq	%r10
1407dea67:     	movq	%rsi, 0x118(%rsp)
1407dea6f:     	leaq	0x58(%rsp), %rsi
1407dea74:     	movq	%rsi, 0x120(%rsp)
1407dea7c:     	movq	%r11, 0x128(%rsp)
1407dea84:     	leaq	0x28(%rsp), %r11
1407dea89:     	movq	%r11, 0x130(%rsp)
1407dea91:     	movq	$0x0, 0x138(%rsp)
1407dea9d:     	movq	%rcx, 0x168(%rsp)
1407deaa5:     	movq	%r12, 0x170(%rsp)
1407deaad:     	movq	%r8, 0x178(%rsp)
1407deab5:     	movq	%r9, 0x180(%rsp)
1407deabd:     	movq	$0x0, 0x188(%rsp)
1407deac9:     	movq	%r14, 0x190(%rsp)
1407dead1:     	movq	%rdx, 0x198(%rsp)
1407dead9:     	movq	%rbx, 0x1a0(%rsp)
1407deae1:     	movq	%rdi, 0x1a8(%rsp)
1407deae9:     	movq	$0x0, 0x1b0(%rsp)
1407deaf5:     	movq	%r15, 0x1b8(%rsp)
1407deafd:     	movq	%r14, 0x1c0(%rsp)
1407deb05:     	movq	%r10, 0x1c8(%rsp)
1407deb0d:     	movb	$0x1, 0x1d0(%rsp)
1407deb15:     	leaq	0x22a9c24(%rip), %r8    # 0x142a88740
1407deb1c:     	leaq	0x118(%rsp), %rdx
1407deb24:     	movq	%rax, %rcx
1407deb27:     	callq	0x14125f8e0 <.text+0x125e8e0>
1407deb2c:     	nop
1407deb2d:     	addq	$0x1d8, %rsp            # imm = 0x1D8
1407deb34:     	popq	%rbx
1407deb35:     	popq	%rdi
1407deb36:     	popq	%rsi
1407deb37:     	popq	%r12
1407deb39:     	popq	%r14
1407deb3b:     	popq	%r15
1407deb3d:     	retq
1407deb3e:     	leaq	0x22a8e8b(%rip), %rcx   # 0x142a879d0
1407deb45:     	leaq	0x22a8efc(%rip), %r8    # 0x142a87a48
1407deb4c:     	movl	$0x1b, %edx
1407deb51:     	callq	0x1428d9430 <.text+0x28d8430>
