
D:\game\reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140fc6be0:     	pushq	%r14
140fc6be2:     	pushq	%rsi
140fc6be3:     	pushq	%rdi
140fc6be4:     	pushq	%rbp
140fc6be5:     	pushq	%rbx
140fc6be6:     	subq	$0x90, %rsp
140fc6bed:     	movdqa	%xmm6, 0x80(%rsp)
140fc6bf6:     	movq	(%rdx), %rbx
140fc6bf9:     	cmpq	$0x0, 0x1b8(%rbx)
140fc6c01:     	je	0x140fc6e09 <.text+0xfc5e09>
140fc6c07:     	movq	%rdx, %rdi
140fc6c0a:     	movq	%rcx, %rsi
140fc6c0d:     	addq	$0x8, %rdx
140fc6c11:     	leaq	0x1c0(%rbx), %rcx
140fc6c18:     	callq	0x1405b0d70 <.text+0x5afd70>
140fc6c1d:     	movq	0x1a0(%rbx), %rcx
140fc6c24:     	movq	0x1a8(%rbx), %rdx
140fc6c2b:     	movq	%rdx, %r8
140fc6c2e:     	andq	%rax, %r8
140fc6c31:     	shrq	$0x39, %rax
140fc6c35:     	movd	%eax, %xmm0
140fc6c39:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140fc6c3d:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140fc6c42:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140fc6c47:     	leaq	-0xc0(%rcx), %rax
140fc6c4e:     	movq	0x8(%rdi), %r9
140fc6c52:     	xorl	%r10d, %r10d
140fc6c55:     	pcmpeqd	%xmm1, %xmm1
140fc6c59:     	movdqu	(%rcx,%r8), %xmm2
140fc6c5f:     	movdqa	%xmm2, %xmm3
140fc6c63:     	pcmpeqb	%xmm0, %xmm3
140fc6c67:     	pmovmskb	%xmm3, %r11d
140fc6c6c:     	testl	%r11d, %r11d
140fc6c6f:     	je	0x140fc6ca0 <.text+0xfc5ca0>
140fc6c71:     	tzcntl	%r11d, %ebx
140fc6c76:     	addq	%r8, %rbx
140fc6c79:     	andq	%rdx, %rbx
140fc6c7c:     	negq	%rbx
140fc6c7f:     	leaq	(%rbx,%rbx,2), %rbx
140fc6c83:     	shlq	$0x6, %rbx
140fc6c87:     	cmpq	(%rax,%rbx), %r9
140fc6c8b:     	je	0x140fc6cc2 <.text+0xfc5cc2>
140fc6c8d:     	leal	-0x1(%r11), %ebx
140fc6c91:     	andw	%r11w, %bx
140fc6c95:     	movl	%ebx, %r11d
140fc6c98:     	jne	0x140fc6c71 <.text+0xfc5c71>
140fc6c9a:     	nopw	(%rax,%rax)
140fc6ca0:     	pcmpeqb	%xmm1, %xmm2
140fc6ca4:     	pmovmskb	%xmm2, %r11d
140fc6ca9:     	testl	%r11d, %r11d
140fc6cac:     	jne	0x140fc6e09 <.text+0xfc5e09>
140fc6cb2:     	addq	%r10, %r8
140fc6cb5:     	addq	$0x10, %r8
140fc6cb9:     	addq	$0x10, %r10
140fc6cbd:     	andq	%rdx, %r8
140fc6cc0:     	jmp	0x140fc6c59 <.text+0xfc5c59>
140fc6cc2:     	xorl	%eax, %eax
140fc6cc4:     	testb	$0x1, -0x2(%rcx,%rbx)
140fc6cc9:     	sete	%al
140fc6ccc:     	leaq	0x1bec4f5(%rip), %rcx   # 0x142bb31c8
140fc6cd3:     	movd	(%rcx,%rax,4), %xmm6
140fc6cd8:     	movq	(%rsi), %rax
140fc6cdb:     	cmpb	$0x0, (%rax)
140fc6cde:     	je	0x140fc6e15 <.text+0xfc5e15>
140fc6ce4:     	leaq	0x1bed6c2(%rip), %rax   # 0x142bb43ad
140fc6ceb:     	movq	%rax, 0x20(%rsp)
140fc6cf0:     	movq	$0xb, 0x28(%rsp)
140fc6cf9:     	leaq	0x1bed6a0(%rip), %r8    # 0x142bb43a0
140fc6d00:     	leaq	0x50(%rsp), %rbx
140fc6d05:     	movl	$0xd, %r9d
140fc6d0b:     	movq	%rbx, %rcx
140fc6d0e:     	movq	%rdi, %rdx
140fc6d11:     	callq	0x140fa0130 <.text+0xf9f130>
140fc6d16:     	movq	%rbx, %rcx
140fc6d19:     	movb	$0x1, %dl
140fc6d1b:     	callq	0x140f9de80 <.text+0xf9ce80>
140fc6d20:     	movl	$0x3f19999a, 0x40(%rax) # imm = 0x3F19999A
140fc6d27:     	movdqu	0x50(%rsp), %xmm0
140fc6d2d:     	movdqa	%xmm0, 0x30(%rsp)
140fc6d33:     	movq	0x60(%rsp), %rax
140fc6d38:     	movq	%rax, 0x40(%rsp)
140fc6d3d:     	movq	0x30(%rsp), %rbx
140fc6d42:     	cmpq	$0x0, 0x1b8(%rbx)
140fc6d4a:     	je	0x140fc72c9 <.text+0xfc62c9>
140fc6d50:     	movq	0x8(%rsi), %r14
140fc6d54:     	leaq	0x38(%rsp), %rdx
140fc6d59:     	leaq	0x1c0(%rbx), %rcx
140fc6d60:     	callq	0x1405b0d70 <.text+0x5afd70>
140fc6d65:     	movq	0x1a0(%rbx), %rcx
140fc6d6c:     	movq	0x1a8(%rbx), %rdx
140fc6d73:     	movq	%rdx, %r8
140fc6d76:     	andq	%rax, %r8
140fc6d79:     	shrq	$0x39, %rax
140fc6d7d:     	movd	%eax, %xmm0
140fc6d81:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140fc6d85:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140fc6d8a:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140fc6d8f:     	leaq	-0xc0(%rcx), %rax
140fc6d96:     	xorl	%r9d, %r9d
140fc6d99:     	movq	0x38(%rsp), %r10
140fc6d9e:     	pcmpeqd	%xmm1, %xmm1
140fc6da2:     	movdqu	(%rcx,%r8), %xmm2
140fc6da8:     	movdqa	%xmm2, %xmm3
140fc6dac:     	pcmpeqb	%xmm0, %xmm3
140fc6db0:     	pmovmskb	%xmm3, %r11d
140fc6db5:     	testl	%r11d, %r11d
140fc6db8:     	je	0x140fc6de7 <.text+0xfc5de7>
140fc6dba:     	tzcntl	%r11d, %ebx
140fc6dbf:     	addq	%r8, %rbx
140fc6dc2:     	andq	%rdx, %rbx
140fc6dc5:     	negq	%rbx
140fc6dc8:     	leaq	(%rbx,%rbx,2), %rbx
140fc6dcc:     	shlq	$0x6, %rbx
140fc6dd0:     	cmpq	(%rax,%rbx), %r10
140fc6dd4:     	je	0x140fc704f <.text+0xfc604f>
140fc6dda:     	leal	-0x1(%r11), %ebx
140fc6dde:     	andw	%r11w, %bx
140fc6de2:     	movl	%ebx, %r11d
140fc6de5:     	jne	0x140fc6dba <.text+0xfc5dba>
140fc6de7:     	pcmpeqb	%xmm1, %xmm2
140fc6deb:     	pmovmskb	%xmm2, %r11d
140fc6df0:     	testl	%r11d, %r11d
140fc6df3:     	jne	0x140fc72c9 <.text+0xfc62c9>
140fc6df9:     	addq	%r9, %r8
140fc6dfc:     	addq	$0x10, %r8
140fc6e00:     	addq	$0x10, %r9
140fc6e04:     	andq	%rdx, %r8
140fc6e07:     	jmp	0x140fc6da2 <.text+0xfc5da2>
140fc6e09:     	leaq	0x1a50e10(%rip), %rcx   # 0x142a17c20
140fc6e10:     	callq	0x1428d9310 <.text+0x28d8310>
140fc6e15:     	movq	0x8(%rsi), %r14
140fc6e19:     	xorl	%ebp, %ebp
140fc6e1b:     	leaq	0x1bed5ba(%rip), %rax   # 0x142bb43dc
140fc6e22:     	movq	%rax, 0x20(%rsp)
140fc6e27:     	movq	$0x11, 0x28(%rsp)
140fc6e30:     	leaq	0x1bed599(%rip), %r8    # 0x142bb43d0
140fc6e37:     	leaq	0x50(%rsp), %rbx
140fc6e3c:     	movl	$0xc, %r9d
140fc6e42:     	movq	%rbx, %rcx
140fc6e45:     	movq	%rdi, %rdx
140fc6e48:     	callq	0x140fa0130 <.text+0xf9f130>
140fc6e4d:     	movq	%rbx, %rcx
140fc6e50:     	movb	$0x1, %dl
140fc6e52:     	callq	0x140f9de80 <.text+0xf9ce80>
140fc6e57:     	movl	$0x3f19999a, 0x40(%rax) # imm = 0x3F19999A
140fc6e5e:     	movdqu	0x50(%rsp), %xmm0
140fc6e64:     	movdqa	%xmm0, 0x30(%rsp)
140fc6e6a:     	movq	0x60(%rsp), %rax
140fc6e6f:     	movq	%rax, 0x40(%rsp)
140fc6e74:     	movq	0x30(%rsp), %rbx
140fc6e79:     	cmpq	$0x0, 0x1b8(%rbx)
140fc6e81:     	je	0x140fc72c9 <.text+0xfc62c9>
140fc6e87:     	leaq	0x38(%rsp), %rdx
140fc6e8c:     	leaq	0x1c0(%rbx), %rcx
140fc6e93:     	callq	0x1405b0d70 <.text+0x5afd70>
140fc6e98:     	movq	0x1a0(%rbx), %rcx
140fc6e9f:     	movq	0x1a8(%rbx), %rdx
140fc6ea6:     	movq	%rdx, %r8
140fc6ea9:     	andq	%rax, %r8
140fc6eac:     	shrq	$0x39, %rax
140fc6eb0:     	movd	%eax, %xmm0
140fc6eb4:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140fc6eb8:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140fc6ebd:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140fc6ec2:     	leaq	-0xc0(%rcx), %rax
140fc6ec9:     	xorl	%r9d, %r9d
140fc6ecc:     	movq	0x38(%rsp), %r10
140fc6ed1:     	pcmpeqd	%xmm1, %xmm1
140fc6ed5:     	movdqu	(%rcx,%r8), %xmm2
140fc6edb:     	movdqa	%xmm2, %xmm3
140fc6edf:     	pcmpeqb	%xmm0, %xmm3
140fc6ee3:     	pmovmskb	%xmm3, %r11d
140fc6ee8:     	testl	%r11d, %r11d
140fc6eeb:     	je	0x140fc6f16 <.text+0xfc5f16>
140fc6eed:     	tzcntl	%r11d, %ebx
140fc6ef2:     	addq	%r8, %rbx
140fc6ef5:     	andq	%rdx, %rbx
140fc6ef8:     	negq	%rbx
140fc6efb:     	leaq	(%rbx,%rbx,2), %rbx
140fc6eff:     	shlq	$0x6, %rbx
140fc6f03:     	cmpq	(%rax,%rbx), %r10
140fc6f07:     	je	0x140fc6f38 <.text+0xfc5f38>
140fc6f09:     	leal	-0x1(%r11), %ebx
140fc6f0d:     	andw	%r11w, %bx
140fc6f11:     	movl	%ebx, %r11d
140fc6f14:     	jne	0x140fc6eed <.text+0xfc5eed>
140fc6f16:     	pcmpeqb	%xmm1, %xmm2
140fc6f1a:     	pmovmskb	%xmm2, %r11d
140fc6f1f:     	testl	%r11d, %r11d
140fc6f22:     	jne	0x140fc72c9 <.text+0xfc62c9>
140fc6f28:     	addq	%r9, %r8
140fc6f2b:     	addq	$0x10, %r8
140fc6f2f:     	addq	$0x10, %r9
140fc6f33:     	andq	%rdx, %r8
140fc6f36:     	jmp	0x140fc6ed5 <.text+0xfc5ed5>
140fc6f38:     	movzbl	-0x1(%rcx,%rbx), %eax
140fc6f3d:     	movl	%eax, %edx
140fc6f3f:     	orb	$0x10, %dl
140fc6f42:     	movb	%dl, -0x1(%rcx,%rbx)
140fc6f46:     	movzbl	(%r14), %edx
140fc6f4a:     	xorb	$0x1, %dl
140fc6f4d:     	orb	%dl, %bpl
140fc6f50:     	jne	0x140fc6f5a <.text+0xfc5f5a>
140fc6f52:     	addq	%rbx, %rcx
140fc6f55:     	orb	$0x18, %al
140fc6f57:     	movb	%al, -0x1(%rcx)
140fc6f5a:     	movq	0x40(%rsp), %rax
140fc6f5f:     	movq	%rax, 0x60(%rsp)
140fc6f64:     	movdqa	0x30(%rsp), %xmm0
140fc6f6a:     	movdqa	%xmm0, 0x50(%rsp)
140fc6f70:     	leaq	0x68(%rsp), %rcx
140fc6f75:     	leaq	0x50(%rsp), %rdx
140fc6f7a:     	movdqa	%xmm6, %xmm2
140fc6f7e:     	movdqa	%xmm6, %xmm3
140fc6f82:     	callq	0x140681530 <.text+0x680530>
140fc6f87:     	movq	0x68(%rsp), %rbx
140fc6f8c:     	cmpq	$0x0, 0x1b8(%rbx)
140fc6f94:     	je	0x140fc7418 <.text+0xfc6418>
140fc6f9a:     	leaq	0x70(%rsp), %rdx
140fc6f9f:     	leaq	0x1c0(%rbx), %rcx
140fc6fa6:     	callq	0x1405b0d70 <.text+0x5afd70>
140fc6fab:     	movq	0x1a0(%rbx), %rcx
140fc6fb2:     	movq	0x1a8(%rbx), %rdx
140fc6fb9:     	movq	%rdx, %r8
140fc6fbc:     	andq	%rax, %r8
140fc6fbf:     	shrq	$0x39, %rax
140fc6fc3:     	movd	%eax, %xmm0
140fc6fc7:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140fc6fcb:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140fc6fd0:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140fc6fd5:     	leaq	-0xc0(%rcx), %rax
140fc6fdc:     	xorl	%r9d, %r9d
140fc6fdf:     	movq	0x70(%rsp), %r10
140fc6fe4:     	pcmpeqd	%xmm1, %xmm1
140fc6fe8:     	movdqu	(%rcx,%r8), %xmm2
140fc6fee:     	movdqa	%xmm2, %xmm3
140fc6ff2:     	pcmpeqb	%xmm0, %xmm3
140fc6ff6:     	pmovmskb	%xmm3, %r11d
140fc6ffb:     	testl	%r11d, %r11d
140fc6ffe:     	je	0x140fc702d <.text+0xfc602d>
140fc7000:     	tzcntl	%r11d, %ebx
140fc7005:     	addq	%r8, %rbx
140fc7008:     	andq	%rdx, %rbx
140fc700b:     	negq	%rbx
140fc700e:     	leaq	(%rbx,%rbx,2), %rbx
140fc7012:     	shlq	$0x6, %rbx
140fc7016:     	cmpq	(%rax,%rbx), %r10
140fc701a:     	je	0x140fc7164 <.text+0xfc6164>
140fc7020:     	leal	-0x1(%r11), %ebx
140fc7024:     	andw	%r11w, %bx
140fc7028:     	movl	%ebx, %r11d
140fc702b:     	jne	0x140fc7000 <.text+0xfc6000>
140fc702d:     	pcmpeqb	%xmm1, %xmm2
140fc7031:     	pmovmskb	%xmm2, %r11d
140fc7036:     	testl	%r11d, %r11d
140fc7039:     	jne	0x140fc7418 <.text+0xfc6418>
140fc703f:     	addq	%r9, %r8
140fc7042:     	addq	$0x10, %r8
140fc7046:     	addq	$0x10, %r9
140fc704a:     	andq	%rdx, %r8
140fc704d:     	jmp	0x140fc6fe8 <.text+0xfc5fe8>
140fc704f:     	movzbl	-0x1(%rcx,%rbx), %eax
140fc7054:     	movl	%eax, %edx
140fc7056:     	orb	$0x10, %dl
140fc7059:     	movb	%dl, -0x1(%rcx,%rbx)
140fc705d:     	movzbl	(%r14), %ebp
140fc7061:     	cmpb	$0x1, %bpl
140fc7065:     	jne	0x140fc706f <.text+0xfc606f>
140fc7067:     	addq	%rbx, %rcx
140fc706a:     	orb	$0x18, %al
140fc706c:     	movb	%al, -0x1(%rcx)
140fc706f:     	movq	0x40(%rsp), %rax
140fc7074:     	movq	%rax, 0x60(%rsp)
140fc7079:     	movdqa	0x30(%rsp), %xmm0
140fc707f:     	movdqa	%xmm0, 0x50(%rsp)
140fc7085:     	leaq	0x30(%rsp), %rcx
140fc708a:     	leaq	0x50(%rsp), %rdx
140fc708f:     	movdqa	%xmm6, %xmm2
140fc7093:     	movdqa	%xmm6, %xmm3
140fc7097:     	callq	0x140681530 <.text+0x680530>
140fc709c:     	movq	0x30(%rsp), %rbx
140fc70a1:     	cmpq	$0x0, 0x1b8(%rbx)
140fc70a9:     	je	0x140fc7418 <.text+0xfc6418>
140fc70af:     	leaq	0x38(%rsp), %rdx
140fc70b4:     	leaq	0x1c0(%rbx), %rcx
140fc70bb:     	callq	0x1405b0d70 <.text+0x5afd70>
140fc70c0:     	movq	0x1a0(%rbx), %rcx
140fc70c7:     	movq	0x1a8(%rbx), %rdx
140fc70ce:     	movq	%rdx, %r8
140fc70d1:     	andq	%rax, %r8
140fc70d4:     	shrq	$0x39, %rax
140fc70d8:     	movd	%eax, %xmm0
140fc70dc:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140fc70e0:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140fc70e5:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140fc70ea:     	leaq	-0xc0(%rcx), %rax
140fc70f1:     	xorl	%r9d, %r9d
140fc70f4:     	movq	0x38(%rsp), %r10
140fc70f9:     	pcmpeqd	%xmm1, %xmm1
140fc70fd:     	movdqu	(%rcx,%r8), %xmm2
140fc7103:     	movdqa	%xmm2, %xmm3
140fc7107:     	pcmpeqb	%xmm0, %xmm3
140fc710b:     	pmovmskb	%xmm3, %r11d
140fc7110:     	testl	%r11d, %r11d
140fc7113:     	je	0x140fc7142 <.text+0xfc6142>
140fc7115:     	tzcntl	%r11d, %ebx
140fc711a:     	addq	%r8, %rbx
140fc711d:     	andq	%rdx, %rbx
140fc7120:     	negq	%rbx
140fc7123:     	leaq	(%rbx,%rbx,2), %rbx
140fc7127:     	shlq	$0x6, %rbx
140fc712b:     	cmpq	(%rax,%rbx), %r10
140fc712f:     	je	0x140fc72d5 <.text+0xfc62d5>
140fc7135:     	leal	-0x1(%r11), %ebx
140fc7139:     	andw	%r11w, %bx
140fc713d:     	movl	%ebx, %r11d
140fc7140:     	jne	0x140fc7115 <.text+0xfc6115>
140fc7142:     	pcmpeqb	%xmm1, %xmm2
140fc7146:     	pmovmskb	%xmm2, %r11d
140fc714b:     	testl	%r11d, %r11d
140fc714e:     	jne	0x140fc7418 <.text+0xfc6418>
140fc7154:     	addq	%r9, %r8
140fc7157:     	addq	$0x10, %r8
140fc715b:     	addq	$0x10, %r9
140fc715f:     	andq	%rdx, %r8
140fc7162:     	jmp	0x140fc70fd <.text+0xfc60fd>
140fc7164:     	testb	$0x30, -0x2(%rcx,%rbx)
140fc7169:     	je	0x140fc71ac <.text+0xfc61ac>
140fc716b:     	movq	0x10(%rsi), %rax
140fc716f:     	movq	0x20(%rsi), %rdx
140fc7173:     	movq	(%rax), %rcx
140fc7176:     	decq	%rcx
140fc7179:     	cmpq	%rdx, %rcx
140fc717c:     	jae	0x140fc7496 <.text+0xfc6496>
140fc7182:     	movq	0x18(%rsi), %rax
140fc7186:     	leaq	(%rcx,%rcx,8), %rdx
140fc718a:     	leaq	(%rdx,%rdx,2), %rdx
140fc718e:     	addq	%rcx, %rdx
140fc7191:     	movups	(%rax,%rdx), %xmm0
140fc7195:     	movaps	%xmm0, 0x50(%rsp)
140fc719a:     	movq	0x30(%rsi), %rax
140fc719e:     	movb	$0x1, (%rax)
140fc71a1:     	movdqa	0x50(%rsp), %xmm0
140fc71a7:     	movdqu	%xmm0, 0x1(%rax)
140fc71ac:     	leaq	0x1bed261(%rip), %rax   # 0x142bb4414
140fc71b3:     	movq	%rax, 0x20(%rsp)
140fc71b8:     	movq	$0x12, 0x28(%rsp)
140fc71c1:     	leaq	0x1bed240(%rip), %r8    # 0x142bb4408
140fc71c8:     	leaq	0x50(%rsp), %rbx
140fc71cd:     	movl	$0xc, %r9d
140fc71d3:     	movq	%rbx, %rcx
140fc71d6:     	movq	%rdi, %rdx
140fc71d9:     	callq	0x140fa0130 <.text+0xf9f130>
140fc71de:     	movq	%rbx, %rcx
140fc71e1:     	movb	$0x1, %dl
140fc71e3:     	callq	0x140f9de80 <.text+0xf9ce80>
140fc71e8:     	movl	$0x3f19999a, 0x40(%rax) # imm = 0x3F19999A
140fc71ef:     	movdqu	0x50(%rsp), %xmm0
140fc71f5:     	movdqa	%xmm0, 0x30(%rsp)
140fc71fb:     	movq	0x60(%rsp), %rax
140fc7200:     	movq	%rax, 0x40(%rsp)
140fc7205:     	movq	0x30(%rsp), %rdi
140fc720a:     	cmpq	$0x0, 0x1b8(%rdi)
140fc7212:     	je	0x140fc72c9 <.text+0xfc62c9>
140fc7218:     	leaq	0x38(%rsp), %rdx
140fc721d:     	leaq	0x1c0(%rdi), %rcx
140fc7224:     	callq	0x1405b0d70 <.text+0x5afd70>
140fc7229:     	movq	0x1a0(%rdi), %rcx
140fc7230:     	movq	0x1a8(%rdi), %rdx
140fc7237:     	movq	%rdx, %r8
140fc723a:     	andq	%rax, %r8
140fc723d:     	shrq	$0x39, %rax
140fc7241:     	movd	%eax, %xmm0
140fc7245:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140fc7249:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140fc724e:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140fc7253:     	leaq	-0xc0(%rcx), %rax
140fc725a:     	xorl	%r9d, %r9d
140fc725d:     	movq	0x38(%rsp), %r10
140fc7262:     	pcmpeqd	%xmm1, %xmm1
140fc7266:     	movdqu	(%rcx,%r8), %xmm2
140fc726c:     	movdqa	%xmm2, %xmm3
140fc7270:     	pcmpeqb	%xmm0, %xmm3
140fc7274:     	pmovmskb	%xmm3, %r11d
140fc7279:     	testl	%r11d, %r11d
140fc727c:     	je	0x140fc72ab <.text+0xfc62ab>
140fc727e:     	tzcntl	%r11d, %edi
140fc7283:     	addq	%r8, %rdi
140fc7286:     	andq	%rdx, %rdi
140fc7289:     	negq	%rdi
140fc728c:     	leaq	(%rdi,%rdi,2), %rdi
140fc7290:     	shlq	$0x6, %rdi
140fc7294:     	cmpq	(%rax,%rdi), %r10
140fc7298:     	je	0x140fc7326 <.text+0xfc6326>
140fc729e:     	leal	-0x1(%r11), %edi
140fc72a2:     	andw	%r11w, %di
140fc72a6:     	movl	%edi, %r11d
140fc72a9:     	jne	0x140fc727e <.text+0xfc627e>
140fc72ab:     	pcmpeqb	%xmm1, %xmm2
140fc72af:     	pmovmskb	%xmm2, %r11d
140fc72b4:     	testl	%r11d, %r11d
140fc72b7:     	jne	0x140fc72c9 <.text+0xfc62c9>
140fc72b9:     	addq	%r9, %r8
140fc72bc:     	addq	$0x10, %r8
140fc72c0:     	addq	$0x10, %r9
140fc72c4:     	andq	%rdx, %r8
140fc72c7:     	jmp	0x140fc7266 <.text+0xfc6266>
140fc72c9:     	leaq	0x1a50d20(%rip), %rcx   # 0x142a17ff0
140fc72d0:     	callq	0x1428d9310 <.text+0x28d8310>
140fc72d5:     	testb	$0x30, -0x2(%rcx,%rbx)
140fc72da:     	je	0x140fc6e1b <.text+0xfc5e1b>
140fc72e0:     	movq	0x10(%rsi), %rax
140fc72e4:     	movq	0x20(%rsi), %rdx
140fc72e8:     	movq	(%rax), %rcx
140fc72eb:     	decq	%rcx
140fc72ee:     	cmpq	%rdx, %rcx
140fc72f1:     	jae	0x140fc74a2 <.text+0xfc64a2>
140fc72f7:     	movq	0x18(%rsi), %rax
140fc72fb:     	leaq	(%rcx,%rcx,8), %rdx
140fc72ff:     	leaq	(%rdx,%rdx,2), %rdx
140fc7303:     	addq	%rcx, %rdx
140fc7306:     	movups	(%rax,%rdx), %xmm0
140fc730a:     	movaps	%xmm0, 0x50(%rsp)
140fc730f:     	movq	0x28(%rsi), %rax
140fc7313:     	movb	$0x1, (%rax)
140fc7316:     	movdqa	0x50(%rsp), %xmm0
140fc731c:     	movdqu	%xmm0, 0x1(%rax)
140fc7321:     	jmp	0x140fc6e1b <.text+0xfc5e1b>
140fc7326:     	orb	$0x10, -0x1(%rcx,%rdi)
140fc732b:     	movq	0x40(%rsp), %rax
140fc7330:     	movq	%rax, 0x60(%rsp)
140fc7335:     	movdqa	0x30(%rsp), %xmm0
140fc733b:     	movdqa	%xmm0, 0x50(%rsp)
140fc7341:     	leaq	0x30(%rsp), %rcx
140fc7346:     	leaq	0x50(%rsp), %rdx
140fc734b:     	movdqa	%xmm6, %xmm2
140fc734f:     	movdqa	%xmm6, %xmm3
140fc7353:     	callq	0x140681530 <.text+0x680530>
140fc7358:     	movq	0x30(%rsp), %rdi
140fc735d:     	cmpq	$0x0, 0x1b8(%rdi)
140fc7365:     	je	0x140fc7418 <.text+0xfc6418>
140fc736b:     	leaq	0x38(%rsp), %rdx
140fc7370:     	leaq	0x1c0(%rdi), %rcx
140fc7377:     	callq	0x1405b0d70 <.text+0x5afd70>
140fc737c:     	movq	0x1a0(%rdi), %rcx
140fc7383:     	movq	0x1a8(%rdi), %rdx
140fc738a:     	movq	%rdx, %r8
140fc738d:     	andq	%rax, %r8
140fc7390:     	shrq	$0x39, %rax
140fc7394:     	movd	%eax, %xmm0
140fc7398:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140fc739c:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140fc73a1:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140fc73a6:     	leaq	-0xc0(%rcx), %rax
140fc73ad:     	xorl	%r9d, %r9d
140fc73b0:     	movq	0x38(%rsp), %r10
140fc73b5:     	pcmpeqd	%xmm1, %xmm1
140fc73b9:     	movdqu	(%rcx,%r8), %xmm2
140fc73bf:     	movdqa	%xmm2, %xmm3
140fc73c3:     	pcmpeqb	%xmm0, %xmm3
140fc73c7:     	pmovmskb	%xmm3, %r11d
140fc73cc:     	testl	%r11d, %r11d
140fc73cf:     	je	0x140fc73fa <.text+0xfc63fa>
140fc73d1:     	tzcntl	%r11d, %edi
140fc73d6:     	addq	%r8, %rdi
140fc73d9:     	andq	%rdx, %rdi
140fc73dc:     	negq	%rdi
140fc73df:     	leaq	(%rdi,%rdi,2), %rdi
140fc73e3:     	shlq	$0x6, %rdi
140fc73e7:     	cmpq	(%rax,%rdi), %r10
140fc73eb:     	je	0x140fc7430 <.text+0xfc6430>
140fc73ed:     	leal	-0x1(%r11), %edi
140fc73f1:     	andw	%r11w, %di
140fc73f5:     	movl	%edi, %r11d
140fc73f8:     	jne	0x140fc73d1 <.text+0xfc63d1>
140fc73fa:     	pcmpeqb	%xmm1, %xmm2
140fc73fe:     	pmovmskb	%xmm2, %r11d
140fc7403:     	testl	%r11d, %r11d
140fc7406:     	jne	0x140fc7418 <.text+0xfc6418>
140fc7408:     	addq	%r9, %r8
140fc740b:     	addq	$0x10, %r8
140fc740f:     	addq	$0x10, %r9
140fc7413:     	andq	%rdx, %r8
140fc7416:     	jmp	0x140fc73b9 <.text+0xfc63b9>
140fc7418:     	leaq	0x1a4fde9(%rip), %rcx   # 0x142a17208
140fc741f:     	leaq	0x1a50bb2(%rip), %r8    # 0x142a17fd8
140fc7426:     	movl	$0x16, %edx
140fc742b:     	callq	0x1428d9330 <.text+0x28d8330>
140fc7430:     	testb	$0x30, -0x2(%rcx,%rdi)
140fc7435:     	je	0x140fc7480 <.text+0xfc6480>
140fc7437:     	movq	0x10(%rsi), %rax
140fc743b:     	movq	0x20(%rsi), %rdx
140fc743f:     	movq	(%rax), %rcx
140fc7442:     	decq	%rcx
140fc7445:     	cmpq	%rdx, %rcx
140fc7448:     	jae	0x140fc74ae <.text+0xfc64ae>
140fc744a:     	movq	0x18(%rsi), %rax
140fc744e:     	leaq	(%rcx,%rcx,8), %rdx
140fc7452:     	leaq	(%rdx,%rdx,2), %rdx
140fc7456:     	addq	%rcx, %rdx
140fc7459:     	movups	(%rax,%rdx), %xmm0
140fc745d:     	movaps	%xmm0, 0x50(%rsp)
140fc7462:     	movq	0x38(%rsi), %rax
140fc7466:     	movq	$0x1, (%rax)
140fc746d:     	movdqa	0x50(%rsp), %xmm0
140fc7473:     	movdqu	%xmm0, 0x8(%rax)
140fc7478:     	movq	$0x0, 0x18(%rax)
140fc7480:     	movaps	0x80(%rsp), %xmm6
140fc7488:     	addq	$0x90, %rsp
140fc748f:     	popq	%rbx
140fc7490:     	popq	%rbp
140fc7491:     	popq	%rdi
140fc7492:     	popq	%rsi
140fc7493:     	popq	%r14
140fc7495:     	retq
140fc7496:     	leaq	0x1becf53(%rip), %r8    # 0x142bb43f0
140fc749d:     	callq	0x1428d9518 <.text+0x28d8518>
140fc74a2:     	leaq	0x1becf0f(%rip), %r8    # 0x142bb43b8
140fc74a9:     	callq	0x1428d9518 <.text+0x28d8518>
140fc74ae:     	leaq	0x1becf73(%rip), %r8    # 0x142bb4428
140fc74b5:     	callq	0x1428d9518 <.text+0x28d8518>
140fc74ba:     	int3
