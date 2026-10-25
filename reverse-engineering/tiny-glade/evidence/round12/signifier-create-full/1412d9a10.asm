
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1412d9a10:     	pushq	%rsi
1412d9a11:     	pushq	%rdi
1412d9a12:     	subq	$0x78, %rsp
1412d9a16:     	addq	$0x28, (%r8)
1412d9a1a:     	testq	%rdx, %rdx
1412d9a1d:     	je	0x1412d9a9a <.text+0x12d8a9a>
1412d9a1f:     	movq	%rdx, %rsi
1412d9a22:     	movq	(%rcx), %rdi
1412d9a25:     	movq	0x8(%rcx), %rdx
1412d9a29:     	movq	0x20(%rcx), %rax
1412d9a2d:     	movq	%rax, 0x70(%rsp)
1412d9a32:     	movups	0x10(%rcx), %xmm0
1412d9a36:     	movaps	%xmm0, 0x60(%rsp)
1412d9a3b:     	leaq	0x20(%rsp), %rcx
1412d9a40:     	movq	%rsi, %r8
1412d9a43:     	callq	0x140ceb900 <.text+0xcea900>
1412d9a48:     	movq	0x20(%rsp), %rdx
1412d9a4d:     	movq	0x28(%rsp), %rax
1412d9a52:     	movq	0x30(%rsp), %rcx
1412d9a57:     	testq	%rdx, %rdx
1412d9a5a:     	je	0x1412d9aa1 <.text+0x12d8aa1>
1412d9a5c:     	movq	0x38(%rsp), %r8
1412d9a61:     	movq	%rdx, 0x40(%rsp)
1412d9a66:     	movq	%rax, 0x48(%rsp)
1412d9a6b:     	movq	%rcx, 0x50(%rsp)
1412d9a70:     	movq	%r8, 0x58(%rsp)
1412d9a75:     	movq	0x70(%rsp), %rax
1412d9a7a:     	movq	%rax, 0x30(%rsp)
1412d9a7f:     	movaps	0x60(%rsp), %xmm0
1412d9a84:     	movaps	%xmm0, 0x20(%rsp)
1412d9a89:     	leaq	0x20(%rsp), %rcx
1412d9a8e:     	leaq	0x40(%rsp), %rdx
1412d9a93:     	callq	0x1420272e0 <.text+0x20262e0>
1412d9a98:     	jmp	0x1412d9aa7 <.text+0x12d8aa7>
1412d9a9a:     	addq	$0x78, %rsp
1412d9a9e:     	popq	%rdi
1412d9a9f:     	popq	%rsi
1412d9aa0:     	retq
1412d9aa1:     	cmpq	$0x2, %rax
1412d9aa5:     	jne	0x1412d9ac5 <.text+0x12d8ac5>
1412d9aa7:     	movq	%rsi, %rcx
1412d9aaa:     	callq	0x140cf81b0 <.text+0xcf71b0>
1412d9aaf:     	movq	%rsi, %rcx
1412d9ab2:     	callq	0x140cf8250 <.text+0xcf7250>
1412d9ab7:     	movq	%rsi, %rcx
1412d9aba:     	addq	$0x78, %rsp
1412d9abe:     	popq	%rdi
1412d9abf:     	popq	%rsi
1412d9ac0:     	jmp	0x140cf8200 <.text+0xcf7200>
1412d9ac5:     	movq	%rax, 0x20(%rsp)
1412d9aca:     	movq	%rcx, 0x28(%rsp)
1412d9acf:     	leaq	0x20(%rsp), %rcx
1412d9ad4:     	callq	0x1428a81c0 <.text+0x28a71c0>
1412d9ad9:     	movabsq	$-0x8000000000000000, %rcx # imm = 0x8000000000000000
1412d9ae3:     	movq	%rcx, 0x48(%rsp)
1412d9ae8:     	leaq	0x19973ec(%rip), %rcx   # 0x142c70edb
1412d9aef:     	movq	%rcx, 0x50(%rsp)
1412d9af4:     	movq	$0x112, 0x58(%rsp)      # imm = 0x112
1412d9afd:     	movl	$0x2, 0x40(%rsp)
1412d9b05:     	leaq	0x40(%rsp), %rdx
1412d9b0a:     	movq	%rax, %rcx
1412d9b0d:     	callq	*%rdi
1412d9b0f:     	jmp	0x1412d9aa7 <.text+0x12d8aa7>
