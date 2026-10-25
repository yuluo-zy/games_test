
D:\game\reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141e23c00:     	pushq	%rbp
141e23c01:     	pushq	%rsi
141e23c02:     	pushq	%rdi
141e23c03:     	subq	$0x70, %rsp
141e23c07:     	leaq	0x70(%rsp), %rbp
141e23c0c:     	movq	$-0x2, -0x8(%rbp)
141e23c14:     	movq	%rcx, %rsi
141e23c17:     	leaq	-0x48(%rbp), %rdi
141e23c1b:     	movq	%rdi, %rcx
141e23c1e:     	callq	0x140b46000 <.text+0xb45000>
141e23c23:     	movq	%rdi, %rcx
141e23c26:     	callq	0x140d76ed0 <.text+0xd75ed0>
141e23c2b:     	movq	%rdx, %r8
141e23c2e:     	movq	$0x9, 0x20(%rsp)
141e23c37:     	leaq	0x11647ad(%rip), %r9    # 0x142f883eb
141e23c3e:     	leaq	-0x28(%rbp), %rcx
141e23c42:     	movq	%rax, %rdx
141e23c45:     	callq	0x140d62810 <.text+0xd61810>
141e23c4a:     	leaq	-0x28(%rbp), %rcx
141e23c4e:     	callq	0x140d76ed0 <.text+0xd75ed0>
141e23c53:     	movq	%rdx, %r8
141e23c56:     	movq	%rsi, %rcx
141e23c59:     	movq	%rax, %rdx
141e23c5c:     	callq	0x140b46480 <.text+0xb45480>
141e23c61:     	movq	-0x28(%rbp), %rdx
141e23c65:     	testq	%rdx, %rdx
141e23c68:     	je	0x141e23c79 <.text+0x1e22c79>
141e23c6a:     	movq	-0x20(%rbp), %rcx
141e23c6e:     	movl	$0x1, %r8d
141e23c74:     	callq	0x140613c20 <.text+0x612c20>
141e23c79:     	movq	-0x48(%rbp), %rdx
141e23c7d:     	testq	%rdx, %rdx
141e23c80:     	je	0x141e23c91 <.text+0x1e22c91>
141e23c82:     	movq	-0x40(%rbp), %rcx
141e23c86:     	movl	$0x1, %r8d
141e23c8c:     	callq	0x140613c20 <.text+0x612c20>
141e23c91:     	movq	%rsi, %rax
141e23c94:     	addq	$0x70, %rsp
141e23c98:     	popq	%rdi
141e23c99:     	popq	%rsi
141e23c9a:     	popq	%rbp
141e23c9b:     	retq
141e23c9c:     	nopl	(%rax)
141e23ca0:     	movq	%rdx, 0x10(%rsp)
141e23ca5:     	pushq	%rbp
141e23ca6:     	pushq	%rsi
141e23ca7:     	pushq	%rdi
141e23ca8:     	subq	$0x30, %rsp
141e23cac:     	leaq	0x70(%rdx), %rbp
141e23cb0:     	movq	-0x48(%rbp), %rdx
141e23cb4:     	testq	%rdx, %rdx
141e23cb7:     	je	0x141e23cc8 <.text+0x1e22cc8>
141e23cb9:     	movq	-0x40(%rbp), %rcx
141e23cbd:     	movl	$0x1, %r8d
141e23cc3:     	callq	0x140613c20 <.text+0x612c20>
141e23cc8:     	nop
141e23cc9:     	addq	$0x30, %rsp
141e23ccd:     	popq	%rdi
141e23cce:     	popq	%rsi
141e23ccf:     	popq	%rbp
141e23cd0:     	retq
141e23cd1:     	nopw	%cs:(%rax,%rax)
141e23ce0:     	movq	%rdx, 0x10(%rsp)
141e23ce5:     	pushq	%rbp
141e23ce6:     	pushq	%rsi
141e23ce7:     	pushq	%rdi
141e23ce8:     	subq	$0x30, %rsp
141e23cec:     	leaq	0x70(%rdx), %rbp
141e23cf0:     	movq	-0x28(%rbp), %rdx
141e23cf4:     	testq	%rdx, %rdx
141e23cf7:     	je	0x141e23d08 <.text+0x1e22d08>
141e23cf9:     	movq	-0x20(%rbp), %rcx
141e23cfd:     	movl	$0x1, %r8d
141e23d03:     	callq	0x140613c20 <.text+0x612c20>
141e23d08:     	nop
141e23d09:     	addq	$0x30, %rsp
141e23d0d:     	popq	%rdi
141e23d0e:     	popq	%rsi
141e23d0f:     	popq	%rbp
141e23d10:     	retq
