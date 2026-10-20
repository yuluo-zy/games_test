
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1405049f0:     	pushq	%r15
1405049f2:     	pushq	%r14
1405049f4:     	pushq	%rsi
1405049f5:     	pushq	%rdi
1405049f6:     	pushq	%rbx
1405049f7:     	subq	$0xb0, %rsp
1405049fe:     	movaps	%xmm6, 0xa0(%rsp)
140504a06:     	movq	%rdx, %rdi
140504a09:     	movq	%rcx, %rsi
140504a0c:     	movq	(%rcx), %rax
140504a0f:     	movq	(%rax), %rax
140504a12:     	cmpb	$0x0, 0xf1(%rax)
140504a19:     	jne	0x140504a25 <.text+0x503a25>
140504a1b:     	movss	0x2420ee1(%rip), %xmm6  # 0x142925904
140504a23:     	jmp	0x140504a2d <.text+0x503a2d>
140504a25:     	movss	0x2420e43(%rip), %xmm6  # 0x142925870
140504a2d:     	leaq	0x24e93e4(%rip), %r8    # 0x1429ede18
140504a34:     	leaq	0x40(%rsp), %rbx
140504a39:     	movl	$0xf, %r9d
140504a3f:     	movq	%rbx, %rcx
140504a42:     	movq	%rdi, %rdx
140504a45:     	callq	0x1406811c0 <.text+0x6801c0>
140504a4a:     	leaq	0x28(%rsp), %r14
140504a4f:     	movq	%r14, %rcx
140504a52:     	movq	%rbx, %rdx
140504a55:     	movaps	%xmm6, %xmm2
140504a58:     	callq	0x1406814f0 <.text+0x6804f0>
140504a5d:     	movq	%r14, %rcx
140504a60:     	callq	0x140681d50 <.text+0x680d50>
140504a65:     	movq	%rax, %rcx
140504a68:     	callq	0x140683c80 <.text+0x682c80>
140504a6d:     	testb	%al, %al
140504a6f:     	je	0x140504ada <.text+0x503ada>
140504a71:     	movq	0x8(%rsi), %rax
140504a75:     	movl	0x1c(%rax), %ecx
140504a78:     	movq	0x10(%rax), %rdx
140504a7c:     	movl	%ecx, (%rdx)
140504a7e:     	movq	(%rax), %rbx
140504a81:     	movq	0x30(%rbx), %r14
140504a85:     	movq	0x40(%rbx), %r15
140504a89:     	cmpq	0x20(%rbx), %r14
140504a8d:     	jne	0x140504a9f <.text+0x503a9f>
140504a8f:     	leaq	0x20(%rbx), %rcx
140504a93:     	leaq	0x24ee68e(%rip), %rdx   # 0x1429f3128
140504a9a:     	callq	0x140562870 <.text+0x561870>
140504a9f:     	movq	0x28(%rbx), %rax
140504aa3:     	movq	%r15, (%rax,%r14,8)
140504aa7:     	incq	%r14
140504aaa:     	movq	%r14, 0x30(%rbx)
140504aae:     	incq	0x40(%rbx)
140504ab2:     	movq	0x10(%rsi), %rax
140504ab6:     	movl	0x1c(%rax), %ecx
140504ab9:     	movq	0x10(%rax), %rdx
140504abd:     	movl	%ecx, (%rdx)
140504abf:     	movq	(%rax), %rcx
140504ac2:     	movb	$0x0, 0x48(%rsp)
140504ac7:     	movq	$0x8, 0x40(%rsp)
140504ad0:     	leaq	0x40(%rsp), %rdx
140504ad5:     	callq	0x14093fad0 <.text+0x93ead0>
140504ada:     	leaq	0x24e9346(%rip), %r8    # 0x1429ede27
140504ae1:     	leaq	0x40(%rsp), %rbx
140504ae6:     	movl	$0x12, %r9d
140504aec:     	movq	%rbx, %rcx
140504aef:     	movq	%rdi, %rdx
140504af2:     	callq	0x1406811c0 <.text+0x6801c0>
140504af7:     	leaq	0x28(%rsp), %rdi
140504afc:     	movq	%rdi, %rcx
140504aff:     	movq	%rbx, %rdx
140504b02:     	movaps	%xmm6, %xmm2
140504b05:     	callq	0x1406814f0 <.text+0x6804f0>
140504b0a:     	movq	%rdi, %rcx
140504b0d:     	callq	0x140681d50 <.text+0x680d50>
140504b12:     	movq	%rax, %rcx
140504b15:     	callq	0x140683c80 <.text+0x682c80>
140504b1a:     	testb	%al, %al
140504b1c:     	je	0x140504b25 <.text+0x503b25>
140504b1e:     	movq	0x18(%rsi), %rax
140504b22:     	movb	$0x1, (%rax)
140504b25:     	movaps	0xa0(%rsp), %xmm6
140504b2d:     	addq	$0xb0, %rsp
140504b34:     	popq	%rbx
140504b35:     	popq	%rdi
140504b36:     	popq	%rsi
140504b37:     	popq	%r14
140504b39:     	popq	%r15
140504b3b:     	retq
