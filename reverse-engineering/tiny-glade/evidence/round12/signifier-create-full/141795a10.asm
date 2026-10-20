
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141795a10:     	pushq	%rsi
141795a11:     	pushq	%rdi
141795a12:     	subq	$0x28, %rsp
141795a16:     	movq	0x20(%rdx), %r8
141795a1a:     	movq	0x28(%rdx), %r10
141795a1e:     	cmpb	$0x0, 0x68(%rdx)
141795a22:     	je	0x141795a6c <.text+0x1794a6c>
141795a24:     	cmpq	%r10, %r8
141795a27:     	je	0x141795ab0 <.text+0x1794ab0>
141795a2d:     	movq	0x8(%rdx), %rax
141795a31:     	movq	0x8(%rax), %rsi
141795a35:     	movq	0x10(%rax), %rax
141795a39:     	subq	%r8, %r10
141795a3c:     	shrq	$0x2, %r10
141795a40:     	xorl	%r11d, %r11d
141795a43:     	xorl	%edi, %edi
141795a45:     	nopw	%cs:(%rax,%rax)
141795a50:     	movl	(%r8,%rdi,4), %r9d
141795a54:     	cmpq	%r9, %rax
141795a57:     	jbe	0x141795ad4 <.text+0x1794ad4>
141795a59:     	leaq	(%r9,%r9,8), %r9
141795a5d:     	addq	0x10(%rsi,%r9,8), %r11
141795a62:     	incq	%rdi
141795a65:     	cmpq	%rdi, %r10
141795a68:     	jne	0x141795a50 <.text+0x1794a50>
141795a6a:     	jmp	0x141795ab3 <.text+0x1794ab3>
141795a6c:     	cmpq	%r10, %r8
141795a6f:     	je	0x141795ab0 <.text+0x1794ab0>
141795a71:     	movq	0x10(%rdx), %rax
141795a75:     	movq	0x8(%rax), %rsi
141795a79:     	movq	0x10(%rax), %rax
141795a7d:     	subq	%r8, %r10
141795a80:     	shrq	$0x2, %r10
141795a84:     	xorl	%r11d, %r11d
141795a87:     	xorl	%edi, %edi
141795a89:     	nopl	(%rax)
141795a90:     	movl	(%r8,%rdi,4), %r9d
141795a94:     	cmpq	%r9, %rax
141795a97:     	jbe	0x141795ae6 <.text+0x1794ae6>
141795a99:     	leaq	(%r9,%r9,4), %r9
141795a9d:     	shlq	$0x5, %r9
141795aa1:     	addq	0x58(%rsi,%r9), %r11
141795aa6:     	incq	%rdi
141795aa9:     	cmpq	%rdi, %r10
141795aac:     	jne	0x141795a90 <.text+0x1794a90>
141795aae:     	jmp	0x141795ab3 <.text+0x1794ab3>
141795ab0:     	xorl	%r11d, %r11d
141795ab3:     	addq	0x58(%rdx), %r11
141795ab7:     	subq	0x60(%rdx), %r11
141795abb:     	movq	%r11, (%rcx)
141795abe:     	movq	$0x1, 0x8(%rcx)
141795ac6:     	movq	%r11, 0x10(%rcx)
141795aca:     	movq	%rcx, %rax
141795acd:     	addq	$0x28, %rsp
141795ad1:     	popq	%rdi
141795ad2:     	popq	%rsi
141795ad3:     	retq
141795ad4:     	leaq	0x154392d(%rip), %r8    # 0x142cd9408
141795adb:     	movq	%r9, %rcx
141795ade:     	movq	%rax, %rdx
141795ae1:     	callq	0x1428d9518 <.text+0x28d8518>
141795ae6:     	leaq	0x1543933(%rip), %r8    # 0x142cd9420
141795aed:     	movq	%r9, %rcx
141795af0:     	movq	%rax, %rdx
141795af3:     	callq	0x1428d9518 <.text+0x28d8518>
141795af8:     	int3
