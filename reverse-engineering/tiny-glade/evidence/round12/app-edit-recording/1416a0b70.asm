
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1416a0b70:     	pushq	%rbp
1416a0b71:     	pushq	%r15
1416a0b73:     	pushq	%r14
1416a0b75:     	pushq	%r13
1416a0b77:     	pushq	%r12
1416a0b79:     	pushq	%rsi
1416a0b7a:     	pushq	%rdi
1416a0b7b:     	pushq	%rbx
1416a0b7c:     	subq	$0x7d8, %rsp            # imm = 0x7D8
1416a0b83:     	leaq	0x80(%rsp), %rbp
1416a0b8b:     	movdqa	%xmm8, 0x740(%rbp)
1416a0b94:     	movdqa	%xmm7, 0x730(%rbp)
1416a0b9c:     	movaps	%xmm6, 0x720(%rbp)
1416a0ba3:     	movq	$-0x2, 0x718(%rbp)
1416a0bae:     	movq	%rcx, %rsi
1416a0bb1:     	movq	0x1f8b7c0(%rip), %rcx   # 0x14362c378
1416a0bb8:     	leaq	0x15eb7b9(%rip), %r8    # 0x142c8c378
1416a0bbf:     	leaq	0x717(%rbp), %rdx
1416a0bc6:     	callq	0x140b22000 <.text+0xb21000>
1416a0bcb:     	movq	%rax, %rdi
1416a0bce:     	movq	%rdx, %rbx
1416a0bd1:     	movq	%rax, %rcx
1416a0bd4:     	callq	*0x28(%rdx)
1416a0bd7:     	movq	%rax, %rcx
1416a0bda:     	callq	*0x18(%rdx)
1416a0bdd:     	movabsq	$-0x623f01a31097210f, %rax # imm = 0x9DC0FE5CEF68DEF1
1416a0be7:     	xorq	%rdx, %rax
1416a0bea:     	movabsq	$0x452821e638d01377, %rcx # imm = 0x452821E638D01377
1416a0bf4:     	xorq	%rdi, %rcx
1416a0bf7:     	mulq	%rcx
1416a0bfa:     	xorq	%rax, %rdx
1416a0bfd:     	movq	%rdx, %rax
1416a0c00:     	shrq	$0x39, %rax
1416a0c04:     	movq	(%rsi), %rcx
1416a0c07:     	movq	%rsi, 0x708(%rbp)
1416a0c0e:     	movq	0x8(%rsi), %r12
1416a0c12:     	movq	%r12, %r13
1416a0c15:     	movq	%rdx, 0x6f8(%rbp)
1416a0c1c:     	andq	%rdx, %r13
1416a0c1f:     	movd	%eax, %xmm0
1416a0c23:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1416a0c27:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1416a0c2c:     	pshufd	$0x0, %xmm0, %xmm6      # xmm6 = xmm0[0,0,0,0]
1416a0c31:     	leaq	-0x340(%rcx), %rsi
1416a0c38:     	xorl	%eax, %eax
1416a0c3a:     	pcmpeqd	%xmm7, %xmm7
1416a0c3e:     	movq	%rcx, 0x700(%rbp)
1416a0c45:     	movq	%rax, 0x6f0(%rbp)
1416a0c4c:     	movdqu	(%rcx,%r13), %xmm8
1416a0c52:     	movdqa	%xmm8, %xmm0
1416a0c57:     	pcmpeqb	%xmm6, %xmm0
1416a0c5b:     	pmovmskb	%xmm0, %r14d
1416a0c60:     	testl	%r14d, %r14d
1416a0c63:     	je	0x1416a0caa <.text+0x169fcaa>
1416a0c65:     	nopw	%cs:(%rax,%rax)
1416a0c70:     	tzcntl	%r14d, %eax
1416a0c75:     	addq	%r13, %rax
1416a0c78:     	andq	%r12, %rax
1416a0c7b:     	negq	%rax
1416a0c7e:     	imulq	$0x340, %rax, %r15      # imm = 0x340
1416a0c85:     	movq	(%rsi,%r15), %r8
1416a0c89:     	movq	0x8(%rsi,%r15), %r9
1416a0c8e:     	movq	%rdi, %rcx
1416a0c91:     	movq	%rbx, %rdx
1416a0c94:     	callq	0x140d0b760 <.text+0xd0a760>
1416a0c99:     	testb	%al, %al
1416a0c9b:     	jne	0x1416a0cdd <.text+0x169fcdd>
1416a0c9d:     	leal	-0x1(%r14), %eax
1416a0ca1:     	andw	%r14w, %ax
1416a0ca5:     	movl	%eax, %r14d
1416a0ca8:     	jne	0x1416a0c70 <.text+0x169fc70>
1416a0caa:     	pcmpeqb	%xmm7, %xmm8
1416a0caf:     	pmovmskb	%xmm8, %eax
1416a0cb4:     	testl	%eax, %eax
1416a0cb6:     	movq	0x6f0(%rbp), %rax
1416a0cbd:     	jne	0x1416a0d65 <.text+0x169fd65>
1416a0cc3:     	addq	%rax, %r13
1416a0cc6:     	addq	$0x10, %r13
1416a0cca:     	addq	$0x10, %rax
1416a0cce:     	andq	%r12, %r13
1416a0cd1:     	movq	0x700(%rbp), %rcx
1416a0cd8:     	jmp	0x1416a0c45 <.text+0x169fc45>
1416a0cdd:     	movq	0x700(%rbp), %rbx
1416a0ce4:     	addq	%r15, %rbx
1416a0ce7:     	movq	0x708(%rbp), %rsi
1416a0cee:     	addq	$-0x330, %rbx           # imm = 0xFCD0
1416a0cf5:     	leaq	-0x58(%rbp), %rdi
1416a0cf9:     	movq	%rdi, %rcx
1416a0cfc:     	callq	0x14098ec10 <.text+0x98dc10>
1416a0d01:     	leaq	0x2e8(%rbp), %rcx
1416a0d08:     	movq	%rbx, %rdx
1416a0d0b:     	movq	%rdi, %r8
1416a0d0e:     	xorl	%r9d, %r9d
1416a0d11:     	callq	0x14169ed30 <.text+0x169dd30>
1416a0d16:     	movq	0x2e8(%rbp), %rdx
1416a0d1d:     	testq	%rdx, %rdx
1416a0d20:     	je	0x1416a0d38 <.text+0x169fd38>
1416a0d22:     	movq	0x2f0(%rbp), %rcx
1416a0d29:     	shlq	$0x4, %rdx
1416a0d2d:     	movl	$0x8, %r8d
1416a0d33:     	callq	0x140613c20 <.text+0x612c20>
1416a0d38:     	movq	%rsi, %rax
1416a0d3b:     	movaps	0x720(%rbp), %xmm6
1416a0d42:     	movaps	0x730(%rbp), %xmm7
1416a0d49:     	movaps	0x740(%rbp), %xmm8
1416a0d51:     	addq	$0x7d8, %rsp            # imm = 0x7D8
1416a0d58:     	popq	%rbx
1416a0d59:     	popq	%rdi
1416a0d5a:     	popq	%rsi
1416a0d5b:     	popq	%r12
1416a0d5d:     	popq	%r13
1416a0d5f:     	popq	%r14
1416a0d61:     	popq	%r15
1416a0d63:     	popq	%rbp
1416a0d64:     	retq
1416a0d65:     	movq	0x1f8b60c(%rip), %rcx   # 0x14362c378
1416a0d6c:     	leaq	0x15eb605(%rip), %r8    # 0x142c8c378
1416a0d73:     	leaq	0x717(%rbp), %rdx
1416a0d7a:     	callq	0x140b22000 <.text+0xb21000>
1416a0d7f:     	movq	%rax, %rsi
1416a0d82:     	movq	%rdx, %r12
1416a0d85:     	leaq	-0x58(%rbp), %rcx
1416a0d89:     	callq	0x140d31540 <.text+0xd30540>
1416a0d8e:     	movq	$0x0, 0x618(%rbp)
1416a0d99:     	movq	$0x8, 0x620(%rbp)
1416a0da4:     	pxor	%xmm0, %xmm0
1416a0da8:     	movdqu	%xmm0, 0x628(%rbp)
1416a0db0:     	movq	$0x8, 0x638(%rbp)
1416a0dbb:     	movdqu	%xmm0, 0x640(%rbp)
1416a0dc3:     	movq	$0x8, 0x650(%rbp)
1416a0dce:     	movdqu	%xmm0, 0x658(%rbp)
1416a0dd6:     	movq	$0x8, 0x668(%rbp)
1416a0de1:     	movdqu	%xmm0, 0x670(%rbp)
1416a0de9:     	movq	$0x8, 0x680(%rbp)
1416a0df4:     	movdqu	%xmm0, 0x688(%rbp)
1416a0dfc:     	movq	$0x8, 0x698(%rbp)
1416a0e07:     	movdqu	%xmm0, 0x6a0(%rbp)
1416a0e0f:     	movq	$0x8, 0x6b0(%rbp)
1416a0e1a:     	movdqu	%xmm0, 0x6b8(%rbp)
1416a0e22:     	movq	$0x8, 0x6c8(%rbp)
1416a0e2d:     	movdqu	%xmm0, 0x6d0(%rbp)
1416a0e35:     	movq	$0x8, 0x6e0(%rbp)
1416a0e40:     	movq	$0x0, 0x6e8(%rbp)
1416a0e4b:     	movb	$0x2, %cl
1416a0e4d:     	callq	0x140d2f660 <.text+0xd2e660>
1416a0e52:     	movq	%rax, %r13
1416a0e55:     	movq	%rdx, %r15
1416a0e58:     	movq	%rsi, 0x5f0(%rbp)
1416a0e5f:     	movq	%r12, 0x5f8(%rbp)
1416a0e66:     	leaq	0x2e8(%rbp), %r12
1416a0e6d:     	leaq	-0x58(%rbp), %rdx
1416a0e71:     	movl	$0x230, %r8d            # imm = 0x230
1416a0e77:     	movq	%r12, %rcx
1416a0e7a:     	callq	0x142923c00 <.text+0x2922c00>
1416a0e7f:     	leaq	0x518(%rbp), %rcx
1416a0e86:     	leaq	0x618(%rbp), %rdx
1416a0e8d:     	movl	$0xd8, %r8d
1416a0e93:     	callq	0x142923c00 <.text+0x2922c00>
1416a0e98:     	movq	%r13, 0x600(%rbp)
1416a0e9f:     	movq	%r15, 0x608(%rbp)
1416a0ea6:     	movb	$0x0, 0x610(%rbp)
1416a0ead:     	movabsq	$0x100010101, %rdx      # imm = 0x100010101
1416a0eb7:     	movq	%r12, %rcx
1416a0eba:     	callq	0x140d2fa40 <.text+0xd2ea40>
1416a0ebf:     	leaq	-0x48(%rbp), %rcx
1416a0ec3:     	leaq	0x2e8(%rbp), %rdx
1416a0eca:     	movl	$0x330, %r8d            # imm = 0x330
1416a0ed0:     	callq	0x142923c00 <.text+0x2922c00>
1416a0ed5:     	movq	%rdi, -0x58(%rbp)
1416a0ed9:     	movq	%rbx, -0x50(%rbp)
1416a0edd:     	movq	0x708(%rbp), %rsi
1416a0ee4:     	leaq	0x20(%rsi), %r9
1416a0ee8:     	leaq	-0x58(%rbp), %r8
1416a0eec:     	movq	%rsi, %rcx
1416a0eef:     	movq	0x6f8(%rbp), %rdx
1416a0ef6:     	callq	0x1416a2040 <.text+0x16a1040>
1416a0efb:     	movq	%rax, %rbx
1416a0efe:     	jmp	0x1416a0cee <.text+0x169fcee>
