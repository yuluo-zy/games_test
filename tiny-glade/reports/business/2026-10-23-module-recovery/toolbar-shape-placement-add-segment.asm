
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140aa5dd0:     	pushq	%rbp
140aa5dd1:     	pushq	%r15
140aa5dd3:     	pushq	%r14
140aa5dd5:     	pushq	%r13
140aa5dd7:     	pushq	%r12
140aa5dd9:     	pushq	%rsi
140aa5dda:     	pushq	%rdi
140aa5ddb:     	pushq	%rbx
140aa5ddc:     	subq	$0x1e8, %rsp            # imm = 0x1E8
140aa5de3:     	leaq	0x80(%rsp), %rbp
140aa5deb:     	movdqa	%xmm13, 0x150(%rbp)
140aa5df4:     	movaps	%xmm12, 0x140(%rbp)
140aa5dfc:     	movaps	%xmm11, 0x130(%rbp)
140aa5e04:     	movaps	%xmm10, 0x120(%rbp)
140aa5e0c:     	movaps	%xmm9, 0x110(%rbp)
140aa5e14:     	movaps	%xmm8, 0x100(%rbp)
140aa5e1c:     	movaps	%xmm7, 0xf0(%rbp)
140aa5e23:     	movaps	%xmm6, 0xe0(%rbp)
140aa5e2a:     	movq	$-0x2, 0xd8(%rbp)
140aa5e35:     	movaps	%xmm2, %xmm6
140aa5e38:     	movaps	%xmm1, %xmm7
140aa5e3b:     	movq	%rcx, %rsi
140aa5e3e:     	movq	0x1f0(%rbp), %rdi
140aa5e45:     	movss	0x1e8(%rbp), %xmm8
140aa5e4e:     	movss	0x1e0(%rbp), %xmm9
140aa5e57:     	movss	0x1d8(%rbp), %xmm10
140aa5e60:     	movss	0x1d0(%rbp), %xmm11
140aa5e69:     	movd	0x1f8(%rbp), %xmm0
140aa5e71:     	movl	0x58(%rcx), %eax
140aa5e74:     	movss	%xmm1, 0x60(%rbp)
140aa5e79:     	movss	%xmm2, 0x64(%rbp)
140aa5e7e:     	movups	(%r9), %xmm1
140aa5e82:     	movaps	%xmm1, 0x50(%rbp)
140aa5e86:     	movss	%xmm11, 0x68(%rbp)
140aa5e8c:     	movss	%xmm10, 0x6c(%rbp)
140aa5e92:     	movss	%xmm9, 0x70(%rbp)
140aa5e98:     	movss	%xmm8, 0x74(%rbp)
140aa5e9e:     	movq	%rdi, 0x78(%rbp)
140aa5ea2:     	movl	%eax, 0x80(%rbp)
140aa5ea8:     	movq	%rax, 0x90(%rbp)
140aa5eaf:     	leal	0x1(%rax), %eax
140aa5eb2:     	movl	%eax, 0x58(%rcx)
140aa5eb5:     	movq	$0x0, 0xa8(%rbp)
140aa5ec0:     	movq	$0x8, 0xb0(%rbp)
140aa5ecb:     	movq	$0x0, 0xb8(%rbp)
140aa5ed6:     	movd	(%r9), %xmm1
140aa5edb:     	movd	0x4(%r9), %xmm2
140aa5ee1:     	movss	0x8(%r9), %xmm3
140aa5ee7:     	movss	0xc(%r9), %xmm4
140aa5eed:     	movb	$0x1, 0xd7(%rbp)
140aa5ef4:     	leaq	0xa8(%rbp), %rax
140aa5efb:     	movq	%rax, 0x30(%rsp)
140aa5f00:     	movd	%xmm0, 0x28(%rsp)
140aa5f06:     	movss	%xmm4, 0x20(%rsp)
140aa5f0c:     	callq	0x1409ef870 <.text+0x9ee870>
140aa5f11:     	movq	0xa8(%rbp), %rax
140aa5f18:     	movq	%rax, 0xc0(%rbp)
140aa5f1f:     	movq	0xb0(%rbp), %r15
140aa5f26:     	movq	0xb8(%rbp), %rax
140aa5f2d:     	leaq	(%r15,%rax,8), %r14
140aa5f31:     	leaq	0x18(%rsi), %rax
140aa5f35:     	movq	%rax, 0xa0(%rbp)
140aa5f3c:     	leaq	0x38(%rsi), %rax
140aa5f40:     	movq	%rax, 0x98(%rbp)
140aa5f47:     	movaps	0x50(%rbp), %xmm12
140aa5f4c:     	pcmpeqd	%xmm13, %xmm13
140aa5f51:     	movq	%r15, 0xc8(%rbp)
140aa5f58:     	jmp	0x140aa5fdf <.text+0xaa4fdf>
140aa5f5d:     	nopl	(%rax)
140aa5f60:     	movdqu	(%rcx,%r8), %xmm1
140aa5f66:     	movdqa	%xmm1, %xmm2
140aa5f6a:     	pcmpeqb	%xmm0, %xmm2
140aa5f6e:     	pmovmskb	%xmm2, %r10d
140aa5f73:     	testl	%r10d, %r10d
140aa5f76:     	je	0x140aa5fa0 <.text+0xaa4fa0>
140aa5f78:     	tzcntl	%r10d, %r11d
140aa5f7d:     	addq	%r8, %r11
140aa5f80:     	andq	%rdx, %r11
140aa5f83:     	negq	%r11
140aa5f86:     	imulq	$0x38, %r11, %r11
140aa5f8a:     	cmpq	%rdi, (%rax,%r11)
140aa5f8e:     	je	0x140aa5fd0 <.text+0xaa4fd0>
140aa5f90:     	leal	-0x1(%r10), %r11d
140aa5f94:     	andw	%r10w, %r11w
140aa5f98:     	movl	%r11d, %r10d
140aa5f9b:     	jne	0x140aa5f78 <.text+0xaa4f78>
140aa5f9d:     	nopl	(%rax)
140aa5fa0:     	pcmpeqb	%xmm13, %xmm1
140aa5fa5:     	pmovmskb	%xmm1, %r10d
140aa5faa:     	testl	%r10d, %r10d
140aa5fad:     	jne	0x140aa6120 <.text+0xaa5120>
140aa5fb3:     	addq	%r9, %r8
140aa5fb6:     	addq	$0x10, %r8
140aa5fba:     	addq	$0x10, %r9
140aa5fbe:     	andq	%rdx, %r8
