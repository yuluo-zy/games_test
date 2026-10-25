
D:\game\reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140fb9a80:     	pushq	%r15
140fb9a82:     	pushq	%r14
140fb9a84:     	pushq	%r13
140fb9a86:     	pushq	%r12
140fb9a88:     	pushq	%rsi
140fb9a89:     	pushq	%rdi
140fb9a8a:     	pushq	%rbx
140fb9a8b:     	subq	$0x100, %rsp            # imm = 0x100
140fb9a92:     	movq	%rcx, %rsi
140fb9a95:     	movq	%rdx, 0xf0(%rsp)
140fb9a9d:     	movq	%r8, 0xf8(%rsp)
140fb9aa5:     	movq	0x18(%rcx), %r12
140fb9aa9:     	leaq	0xf0(%rsp), %rdi
140fb9ab1:     	movq	%rdi, %rcx
140fb9ab4:     	callq	0x1405bdda0 <.text+0x5bcda0>
140fb9ab9:     	movq	%rax, 0x40(%rsp)
140fb9abe:     	movabsq	$0x41223169ff28813b, %r14 # imm = 0x41223169FF28813B
140fb9ac8:     	movq	%r14, 0x48(%rsp)
140fb9acd:     	movabsq	$-0x58648d975d569727, %r15 # imm = 0xA79B7268A2A968D9
140fb9ad7:     	movq	%r15, 0x50(%rsp)
140fb9adc:     	leaq	0x40(%rsp), %rcx
140fb9ae1:     	callq	0x1405bdf20 <.text+0x5bcf20>
140fb9ae6:     	movq	%r12, 0x40(%rsp)
140fb9aeb:     	movq	%rax, 0x48(%rsp)
140fb9af0:     	leaq	0x40(%rsp), %rcx
140fb9af5:     	callq	0x1405bd910 <.text+0x5bc910>
140fb9afa:     	movq	%rax, 0x40(%rsp)
140fb9aff:     	movq	0x10(%rsi), %r13
140fb9b03:     	cmpq	$0x0, 0x1b8(%r13)
140fb9b0b:     	je	0x140fb9bce <.text+0xfb8bce>
140fb9b11:     	movq	%rax, %rbx
140fb9b14:     	leaq	0x1c0(%r13), %rcx
140fb9b1b:     	leaq	0x40(%rsp), %rdx
140fb9b20:     	callq	0x1405b0d70 <.text+0x5afd70>
140fb9b25:     	movq	0x1a0(%r13), %rcx
140fb9b2c:     	movq	0x1a8(%r13), %rdx
140fb9b33:     	movq	%rdx, %r8
140fb9b36:     	andq	%rax, %r8
140fb9b39:     	shrq	$0x39, %rax
140fb9b3d:     	movd	%eax, %xmm0
140fb9b41:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140fb9b45:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140fb9b4a:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140fb9b4f:     	leaq	-0xc0(%rcx), %rax
140fb9b56:     	xorl	%r9d, %r9d
140fb9b59:     	pcmpeqd	%xmm1, %xmm1
140fb9b5d:     	movdqu	(%rcx,%r8), %xmm2
140fb9b63:     	movdqa	%xmm2, %xmm3
140fb9b67:     	pcmpeqb	%xmm0, %xmm3
140fb9b6b:     	pmovmskb	%xmm3, %r10d
140fb9b70:     	testl	%r10d, %r10d
140fb9b73:     	je	0x140fb9bb0 <.text+0xfb8bb0>
140fb9b75:     	tzcntl	%r10d, %r11d
140fb9b7a:     	addq	%r8, %r11
140fb9b7d:     	andq	%rdx, %r11
140fb9b80:     	negq	%r11
140fb9b83:     	leaq	(%r11,%r11,2), %r11
140fb9b87:     	shlq	$0x6, %r11
140fb9b8b:     	cmpq	%rbx, (%rax,%r11)
140fb9b8f:     	je	0x140fb9f02 <.text+0xfb8f02>
140fb9b95:     	leal	-0x1(%r10), %r11d
140fb9b99:     	andw	%r10w, %r11w
140fb9b9d:     	movl	%r11d, %r10d
140fb9ba0:     	jne	0x140fb9b75 <.text+0xfb8b75>
140fb9ba2:     	nopw	%cs:(%rax,%rax)
140fb9bb0:     	pcmpeqb	%xmm1, %xmm2
140fb9bb4:     	pmovmskb	%xmm2, %r10d
140fb9bb9:     	testl	%r10d, %r10d
140fb9bbc:     	jne	0x140fb9bce <.text+0xfb8bce>
140fb9bbe:     	addq	%r9, %r8
140fb9bc1:     	addq	$0x10, %r8
140fb9bc5:     	addq	$0x10, %r9
140fb9bc9:     	andq	%rdx, %r8
140fb9bcc:     	jmp	0x140fb9b5d <.text+0xfb8b5d>
140fb9bce:     	leaq	0x10(%rsi), %rbx
140fb9bd2:     	movq	0x26727cf(%rip), %rax   # 0x14362c3a8
140fb9bd9:     	movq	(%rax), %rax
140fb9bdc:     	testq	%rax, %rax
140fb9bdf:     	je	0x140fb9cc2 <.text+0xfb8cc2>
140fb9be5:     	movq	%rsi, 0x20(%rsp)
140fb9bea:     	leaq	-0xa86351(%rip), %rax   # 0x1405338a0 <.text+0x5328a0>
140fb9bf1:     	movq	%rax, 0x28(%rsp)
140fb9bf6:     	movq	%rdi, 0x30(%rsp)
140fb9bfb:     	movq	%rax, 0x38(%rsp)
140fb9c00:     	leaq	0x1bfa289(%rip), %rcx   # 0x142bb3e90
140fb9c07:     	callq	0x14067dd80 <.text+0x67cd80>
140fb9c0c:     	movdqu	(%rax), %xmm0
140fb9c10:     	movl	0x10(%rax), %eax
140fb9c13:     	movq	$0x1, 0x70(%rsp)
140fb9c1c:     	leaq	0x1bfa1ed(%rip), %rcx   # 0x142bb3e10
140fb9c23:     	movq	%rcx, 0x78(%rsp)
140fb9c28:     	movq	$0x14, 0x80(%rsp)
140fb9c34:     	leaq	0x1bfa205(%rip), %rdx   # 0x142bb3e40
140fb9c3b:     	movq	%rdx, 0x90(%rsp)
140fb9c43:     	movq	$0x2, 0x98(%rsp)
140fb9c4f:     	leaq	0x20(%rsp), %rdx
140fb9c54:     	movq	%rdx, 0xa0(%rsp)
140fb9c5c:     	movq	$0x2, 0xa8(%rsp)
140fb9c68:     	movq	$0x0, 0xb0(%rsp)
140fb9c74:     	movq	$0x0, 0x40(%rsp)
140fb9c7d:     	movq	%rcx, 0x48(%rsp)
140fb9c82:     	movq	$0x14, 0x50(%rsp)
140fb9c8b:     	movq	$0x0, 0x58(%rsp)
140fb9c94:     	movdqu	%xmm0, 0x60(%rsp)
140fb9c9a:     	movl	$0x1, 0x88(%rsp)
140fb9ca5:     	movl	%eax, 0x8c(%rsp)
140fb9cac:     	leaq	0xd8(%rsp), %rcx
140fb9cb4:     	leaq	0x40(%rsp), %rdx
140fb9cb9:     	callq	0x140d4a330 <.text+0xd49330>
140fb9cbe:     	movq	0x18(%rsi), %r12
140fb9cc2:     	movups	0xf0(%rsp), %xmm0
140fb9cca:     	movups	%xmm0, 0x20(%rsp)
140fb9ccf:     	leaq	0x20(%rsp), %rcx
140fb9cd4:     	callq	0x1405bdda0 <.text+0x5bcda0>
140fb9cd9:     	movq	%rax, 0x40(%rsp)
140fb9cde:     	movq	%r14, 0x48(%rsp)
140fb9ce3:     	movq	%r15, 0x50(%rsp)
140fb9ce8:     	leaq	0x40(%rsp), %rcx
140fb9ced:     	callq	0x1405bdf20 <.text+0x5bcf20>
140fb9cf2:     	movq	%r12, 0x40(%rsp)
140fb9cf7:     	movq	%rax, 0x48(%rsp)
140fb9cfc:     	leaq	0x40(%rsp), %rcx
140fb9d01:     	callq	0x1405bd910 <.text+0x5bc910>
140fb9d06:     	leaq	0xd8(%rsp), %rdi
140fb9d0e:     	movq	%rdi, %rcx
140fb9d11:     	movq	%rbx, %rdx
140fb9d14:     	movq	%rax, %r8
140fb9d17:     	callq	0x1406861b0 <.text+0x6851b0>
140fb9d1c:     	leaq	0x1bfa1c9(%rip), %rsi   # 0x142bb3eec
140fb9d23:     	movq	%rsi, 0xc8(%rsp)
140fb9d2b:     	movq	$0x3, 0xd0(%rsp)
140fb9d37:     	leaq	0xc8(%rsp), %rcx
140fb9d3f:     	callq	0x1405bdda0 <.text+0x5bcda0>
140fb9d44:     	movq	%rax, 0x40(%rsp)
140fb9d49:     	movabsq	$-0x239fc26ae46c7f4e, %rax # imm = 0xDC603D951B9380B2
140fb9d53:     	movq	%rax, 0x48(%rsp)
140fb9d58:     	movabsq	$-0x3196240f511ec25e, %rax # imm = 0xCE69DBF0AEE13DA2
140fb9d62:     	movq	%rax, 0x50(%rsp)
140fb9d67:     	leaq	0x40(%rsp), %rcx
140fb9d6c:     	callq	0x1405bdf20 <.text+0x5bcf20>
140fb9d71:     	movq	0xe0(%rsp), %rcx
140fb9d79:     	movq	%rcx, 0x40(%rsp)
140fb9d7e:     	movq	%rax, 0x48(%rsp)
140fb9d83:     	leaq	0x40(%rsp), %rcx
140fb9d88:     	callq	0x1405bd910 <.text+0x5bc910>
140fb9d8d:     	leaq	0x40(%rsp), %rbx
140fb9d92:     	movq	%rbx, %rcx
140fb9d95:     	movq	%rdi, %rdx
140fb9d98:     	movq	%rax, %r8
140fb9d9b:     	callq	0x1406861b0 <.text+0x6851b0>
140fb9da0:     	movq	%rbx, %rcx
140fb9da3:     	xorl	%edx, %edx
140fb9da5:     	callq	0x140f9d930 <.text+0xf9c930>
140fb9daa:     	movl	$0x3, %r8d
140fb9db0:     	movq	%rax, %rcx
140fb9db3:     	movq	%rsi, %rdx
140fb9db6:     	callq	0x140fac4c0 <.text+0xfab4c0>
140fb9dbb:     	movq	0x50(%rsp), %rax
140fb9dc0:     	movq	%rax, 0x30(%rsp)
140fb9dc5:     	movups	0x40(%rsp), %xmm0
140fb9dca:     	movaps	%xmm0, 0x20(%rsp)
140fb9dcf:     	leaq	0x20(%rsp), %rcx
140fb9dd4:     	movb	$0x1, %dl
140fb9dd6:     	callq	0x140f9d930 <.text+0xf9c930>
140fb9ddb:     	movd	0x1a59a65(%rip), %xmm1  # 0x142a13848
140fb9de3:     	movq	%rax, %rcx
140fb9de6:     	callq	0x140fac150 <.text+0xfab150>
140fb9deb:     	movq	0x30(%rsp), %rax
140fb9df0:     	movq	%rax, 0x50(%rsp)
140fb9df5:     	movaps	0x20(%rsp), %xmm0
140fb9dfa:     	movaps	%xmm0, 0x40(%rsp)
140fb9dff:     	leaq	0x40(%rsp), %rcx
140fb9e04:     	movb	$0x1, %dl
140fb9e06:     	callq	0x140f9d930 <.text+0xf9c930>
140fb9e0b:     	movq	%rax, %rcx
140fb9e0e:     	movw	$0x320, %dx             # imm = 0x320
140fb9e12:     	callq	0x140b90ea0 <.text+0xb8fea0>
140fb9e17:     	movdqa	0x40(%rsp), %xmm0
140fb9e1d:     	movdqa	%xmm0, 0x20(%rsp)
140fb9e23:     	movq	0x50(%rsp), %rax
140fb9e28:     	movq	%rax, 0x30(%rsp)
140fb9e2d:     	movq	0x20(%rsp), %rsi
140fb9e32:     	cmpq	$0x0, 0x1b8(%rsi)
140fb9e3a:     	je	0x140fba102 <.text+0xfb9102>
140fb9e40:     	leaq	0x28(%rsp), %rdx
140fb9e45:     	leaq	0x1c0(%rsi), %rcx
140fb9e4c:     	callq	0x1405b0d70 <.text+0x5afd70>
140fb9e51:     	movq	0x1a0(%rsi), %rcx
140fb9e58:     	movq	0x1a8(%rsi), %rdx
140fb9e5f:     	movq	%rdx, %r8
140fb9e62:     	andq	%rax, %r8
140fb9e65:     	shrq	$0x39, %rax
140fb9e69:     	movd	%eax, %xmm0
140fb9e6d:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140fb9e71:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140fb9e76:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140fb9e7b:     	leaq	-0xc0(%rcx), %rax
140fb9e82:     	xorl	%r9d, %r9d
140fb9e85:     	movq	0x28(%rsp), %r10
140fb9e8a:     	pcmpeqd	%xmm1, %xmm1
140fb9e8e:     	movdqu	(%rcx,%r8), %xmm2
140fb9e94:     	movdqa	%xmm2, %xmm3
140fb9e98:     	pcmpeqb	%xmm0, %xmm3
140fb9e9c:     	pmovmskb	%xmm3, %r11d
140fb9ea1:     	testl	%r11d, %r11d
140fb9ea4:     	je	0x140fb9ee0 <.text+0xfb8ee0>
140fb9ea6:     	tzcntl	%r11d, %esi
140fb9eab:     	addq	%r8, %rsi
140fb9eae:     	andq	%rdx, %rsi
140fb9eb1:     	negq	%rsi
140fb9eb4:     	leaq	(%rsi,%rsi,2), %rsi
140fb9eb8:     	shlq	$0x6, %rsi
140fb9ebc:     	cmpq	(%rax,%rsi), %r10
140fb9ec0:     	je	0x140fba0ea <.text+0xfb90ea>
140fb9ec6:     	leal	-0x1(%r11), %esi
140fb9eca:     	andw	%r11w, %si
140fb9ece:     	movl	%esi, %r11d
140fb9ed1:     	jne	0x140fb9ea6 <.text+0xfb8ea6>
140fb9ed3:     	nopw	%cs:(%rax,%rax)
140fb9ee0:     	pcmpeqb	%xmm1, %xmm2
140fb9ee4:     	pmovmskb	%xmm2, %r11d
140fb9ee9:     	testl	%r11d, %r11d
140fb9eec:     	jne	0x140fba102 <.text+0xfb9102>
140fb9ef2:     	addq	%r9, %r8
140fb9ef5:     	addq	$0x10, %r8
140fb9ef9:     	addq	$0x10, %r9
140fb9efd:     	andq	%rdx, %r8
140fb9f00:     	jmp	0x140fb9e8e <.text+0xfb8e8e>
140fb9f02:     	movq	%r13, 0xd8(%rsp)
140fb9f0a:     	movq	%rbx, 0xe0(%rsp)
140fb9f12:     	movq	$0x0, 0xe8(%rsp)
140fb9f1e:     	leaq	0x1bf9fc7(%rip), %rsi   # 0x142bb3eec
140fb9f25:     	movq	%rsi, 0xc8(%rsp)
140fb9f2d:     	movq	$0x3, 0xd0(%rsp)
140fb9f39:     	leaq	0xc8(%rsp), %rcx
140fb9f41:     	callq	0x1405bdda0 <.text+0x5bcda0>
140fb9f46:     	movq	%rax, 0x40(%rsp)
140fb9f4b:     	movabsq	$-0x239fc26ae46c7f4e, %rax # imm = 0xDC603D951B9380B2
140fb9f55:     	movq	%rax, 0x48(%rsp)
140fb9f5a:     	movabsq	$-0x3196240f511ec25e, %rax # imm = 0xCE69DBF0AEE13DA2
140fb9f64:     	movq	%rax, 0x50(%rsp)
140fb9f69:     	leaq	0x40(%rsp), %rcx
140fb9f6e:     	callq	0x1405bdf20 <.text+0x5bcf20>
140fb9f73:     	movq	%rbx, 0x40(%rsp)
140fb9f78:     	movq	%rax, 0x48(%rsp)
140fb9f7d:     	leaq	0x40(%rsp), %rcx
140fb9f82:     	callq	0x1405bd910 <.text+0x5bc910>
140fb9f87:     	leaq	0x40(%rsp), %rdi
140fb9f8c:     	leaq	0xd8(%rsp), %rdx
140fb9f94:     	movq	%rdi, %rcx
140fb9f97:     	movq	%rax, %r8
140fb9f9a:     	callq	0x1406861b0 <.text+0x6851b0>
140fb9f9f:     	movq	%rdi, %rcx
140fb9fa2:     	xorl	%edx, %edx
140fb9fa4:     	callq	0x140f9d930 <.text+0xf9c930>
140fb9fa9:     	movl	$0x3, %r8d
140fb9faf:     	movq	%rax, %rcx
140fb9fb2:     	movq	%rsi, %rdx
140fb9fb5:     	callq	0x140fac4c0 <.text+0xfab4c0>
140fb9fba:     	movq	0x50(%rsp), %rax
140fb9fbf:     	movq	%rax, 0x30(%rsp)
140fb9fc4:     	movups	0x40(%rsp), %xmm0
140fb9fc9:     	movaps	%xmm0, 0x20(%rsp)
140fb9fce:     	leaq	0x20(%rsp), %rcx
140fb9fd3:     	movb	$0x1, %dl
140fb9fd5:     	callq	0x140f9d930 <.text+0xf9c930>
140fb9fda:     	movd	0x1a59866(%rip), %xmm1  # 0x142a13848
140fb9fe2:     	movq	%rax, %rcx
140fb9fe5:     	callq	0x140fac150 <.text+0xfab150>
140fb9fea:     	movq	0x30(%rsp), %rax
140fb9fef:     	movq	%rax, 0x50(%rsp)
140fb9ff4:     	movaps	0x20(%rsp), %xmm0
140fb9ff9:     	movaps	%xmm0, 0x40(%rsp)
140fb9ffe:     	leaq	0x40(%rsp), %rcx
140fba003:     	movb	$0x1, %dl
140fba005:     	callq	0x140f9d930 <.text+0xf9c930>
140fba00a:     	movq	%rax, %rcx
140fba00d:     	movw	$0x320, %dx             # imm = 0x320
140fba011:     	callq	0x140b90ea0 <.text+0xb8fea0>
140fba016:     	movaps	0x40(%rsp), %xmm0
140fba01b:     	movaps	%xmm0, 0x20(%rsp)
140fba020:     	movq	0x50(%rsp), %rax
140fba025:     	movq	%rax, 0x30(%rsp)
140fba02a:     	movq	0x20(%rsp), %rsi
140fba02f:     	cmpq	$0x0, 0x1b8(%rsi)
140fba037:     	je	0x140fba102 <.text+0xfb9102>
140fba03d:     	leaq	0x28(%rsp), %rdx
140fba042:     	leaq	0x1c0(%rsi), %rcx
140fba049:     	callq	0x1405b0d70 <.text+0x5afd70>
140fba04e:     	movq	0x1a0(%rsi), %rcx
140fba055:     	movq	0x1a8(%rsi), %rdx
140fba05c:     	movq	%rdx, %r8
140fba05f:     	andq	%rax, %r8
140fba062:     	shrq	$0x39, %rax
140fba066:     	movd	%eax, %xmm0
140fba06a:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140fba06e:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140fba073:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140fba078:     	leaq	-0xc0(%rcx), %rax
140fba07f:     	xorl	%r9d, %r9d
140fba082:     	movq	0x28(%rsp), %r10
140fba087:     	pcmpeqd	%xmm1, %xmm1
140fba08b:     	movdqu	(%rcx,%r8), %xmm2
140fba091:     	movdqa	%xmm2, %xmm3
140fba095:     	pcmpeqb	%xmm0, %xmm3
140fba099:     	pmovmskb	%xmm3, %r11d
140fba09e:     	testl	%r11d, %r11d
140fba0a1:     	je	0x140fba0cc <.text+0xfb90cc>
140fba0a3:     	tzcntl	%r11d, %esi
140fba0a8:     	addq	%r8, %rsi
140fba0ab:     	andq	%rdx, %rsi
140fba0ae:     	negq	%rsi
140fba0b1:     	leaq	(%rsi,%rsi,2), %rsi
140fba0b5:     	shlq	$0x6, %rsi
140fba0b9:     	cmpq	(%rax,%rsi), %r10
140fba0bd:     	je	0x140fba0ea <.text+0xfb90ea>
140fba0bf:     	leal	-0x1(%r11), %esi
140fba0c3:     	andw	%r11w, %si
140fba0c7:     	movl	%esi, %r11d
140fba0ca:     	jne	0x140fba0a3 <.text+0xfb90a3>
140fba0cc:     	pcmpeqb	%xmm1, %xmm2
140fba0d0:     	pmovmskb	%xmm2, %r11d
140fba0d5:     	testl	%r11d, %r11d
140fba0d8:     	jne	0x140fba102 <.text+0xfb9102>
140fba0da:     	addq	%r9, %r8
140fba0dd:     	addq	$0x10, %r8
140fba0e1:     	addq	$0x10, %r9
140fba0e5:     	andq	%rdx, %r8
140fba0e8:     	jmp	0x140fba08b <.text+0xfb908b>
140fba0ea:     	orb	$0x2, -0x1(%rcx,%rsi)
140fba0ef:     	addq	$0x100, %rsp            # imm = 0x100
140fba0f6:     	popq	%rbx
140fba0f7:     	popq	%rdi
140fba0f8:     	popq	%rsi
140fba0f9:     	popq	%r12
140fba0fb:     	popq	%r13
140fba0fd:     	popq	%r14
140fba0ff:     	popq	%r15
140fba101:     	retq
140fba102:     	leaq	0x1a5dee7(%rip), %rcx   # 0x142a17ff0
140fba109:     	callq	0x1428d9310 <.text+0x28d8310>
140fba10e:     	int3
