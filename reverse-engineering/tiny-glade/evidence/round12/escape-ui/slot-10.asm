
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140500c50:     	pushq	%r15
140500c52:     	pushq	%r14
140500c54:     	pushq	%r13
140500c56:     	pushq	%r12
140500c58:     	pushq	%rsi
140500c59:     	pushq	%rdi
140500c5a:     	pushq	%rbp
140500c5b:     	pushq	%rbx
140500c5c:     	subq	$0xd8, %rsp
140500c63:     	movq	%r9, %rsi
140500c66:     	movq	%rcx, %rbx
140500c69:     	movq	%rdx, 0x28(%rsp)
140500c6e:     	movq	%r8, 0x30(%rsp)
140500c73:     	leaq	0x10(%rcx), %rdi
140500c77:     	movq	%rdi, %rcx
140500c7a:     	callq	0x1406868b0 <.text+0x6858b0>
140500c7f:     	movq	%rax, %r12
140500c82:     	leaq	0x28(%rsp), %r15
140500c87:     	movq	%r15, %rcx
140500c8a:     	callq	0x140683c10 <.text+0x682c10>
140500c8f:     	movq	%rax, 0x38(%rsp)
140500c94:     	leaq	0x38(%rsp), %r14
140500c99:     	movq	%r14, %rcx
140500c9c:     	callq	0x1400885d0 <.text+0x875d0>
140500ca1:     	movq	%rax, 0x58(%rsp)
140500ca6:     	movabsq	$0x41223169ff28813b, %r13 # imm = 0x41223169FF28813B
140500cb0:     	movq	%r13, 0x60(%rsp)
140500cb5:     	movabsq	$-0x58648d975d569727, %rbp # imm = 0xA79B7268A2A968D9
140500cbf:     	movq	%rbp, 0x68(%rsp)
140500cc4:     	leaq	0x58(%rsp), %rcx
140500cc9:     	callq	0x1405bdf20 <.text+0x5bcf20>
140500cce:     	movq	%r12, 0x58(%rsp)
140500cd3:     	movq	%rax, 0x60(%rsp)
140500cd8:     	leaq	0x58(%rsp), %rcx
140500cdd:     	callq	0x1405bd910 <.text+0x5bc910>
140500ce2:     	movq	%rax, %r12
140500ce5:     	movq	%rdi, %rcx
140500ce8:     	movq	%rax, %rdx
140500ceb:     	callq	0x140686450 <.text+0x685450>
140500cf0:     	testb	%al, %al
140500cf2:     	je	0x140500d0c <.text+0x4ffd0c>
140500cf4:     	leaq	0x58(%rsp), %rbx
140500cf9:     	movq	%rbx, %rcx
140500cfc:     	movq	%rdi, %rdx
140500cff:     	movq	%r12, %r8
140500d02:     	callq	0x140686530 <.text+0x685530>
140500d07:     	jmp	0x140500e4c <.text+0x4ffe4c>
140500d0c:     	movq	0x312b695(%rip), %rax   # 0x14362c3a8
140500d13:     	movq	(%rax), %rax
140500d16:     	testq	%rax, %rax
140500d19:     	je	0x140500df4 <.text+0x4ffdf4>
140500d1f:     	movq	%rbx, 0x38(%rsp)
140500d24:     	leaq	0x32b75(%rip), %rax     # 0x1405338a0 <.text+0x5328a0>
140500d2b:     	movq	%rax, 0x40(%rsp)
140500d30:     	movq	%r15, 0x48(%rsp)
140500d35:     	movq	%rax, 0x50(%rsp)
140500d3a:     	leaq	0x24ed017(%rip), %rcx   # 0x1429edd58
140500d41:     	callq	0x14067dd80 <.text+0x67cd80>
140500d46:     	movups	(%rax), %xmm0
140500d49:     	movl	0x10(%rax), %eax
140500d4c:     	movq	$0x1, 0x88(%rsp)
140500d58:     	leaq	0x24ecfc9(%rip), %rcx   # 0x1429edd28
140500d5f:     	movq	%rcx, 0x90(%rsp)
140500d67:     	movq	$0x14, 0x98(%rsp)
140500d73:     	leaq	0x24ecf1e(%rip), %rdx   # 0x1429edc98
140500d7a:     	movq	%rdx, 0xa8(%rsp)
140500d82:     	movq	$0x2, 0xb0(%rsp)
140500d8e:     	movq	%r14, 0xb8(%rsp)
140500d96:     	movq	$0x2, 0xc0(%rsp)
140500da2:     	movq	$0x0, 0xc8(%rsp)
140500dae:     	movq	$0x0, 0x58(%rsp)
140500db7:     	movq	%rcx, 0x60(%rsp)
140500dbc:     	movq	$0x14, 0x68(%rsp)
140500dc5:     	movq	$0x0, 0x70(%rsp)
140500dce:     	movups	%xmm0, 0x78(%rsp)
140500dd3:     	movl	$0x1, 0xa0(%rsp)
140500dde:     	movl	%eax, 0xa4(%rsp)
140500de5:     	leaq	0x27(%rsp), %rcx
140500dea:     	leaq	0x58(%rsp), %rdx
140500def:     	callq	0x140d4a330 <.text+0xd49330>
140500df4:     	movups	0x28(%rsp), %xmm0
140500df9:     	movq	0x18(%rbx), %rbx
140500dfd:     	movups	%xmm0, 0x38(%rsp)
140500e02:     	leaq	0x38(%rsp), %rcx
140500e07:     	callq	0x140683c10 <.text+0x682c10>
140500e0c:     	movq	%rax, 0x58(%rsp)
140500e11:     	movq	%r13, 0x60(%rsp)
140500e16:     	movq	%rbp, 0x68(%rsp)
140500e1b:     	leaq	0x58(%rsp), %rcx
140500e20:     	callq	0x1405bdf20 <.text+0x5bcf20>
140500e25:     	movq	%rbx, 0x58(%rsp)
140500e2a:     	movq	%rax, 0x60(%rsp)
140500e2f:     	leaq	0x58(%rsp), %rcx
140500e34:     	callq	0x1405bd910 <.text+0x5bc910>
140500e39:     	leaq	0x58(%rsp), %rbx
140500e3e:     	movq	%rbx, %rcx
140500e41:     	movq	%rdi, %rdx
140500e44:     	movq	%rax, %r8
140500e47:     	callq	0x1406861b0 <.text+0x6851b0>
140500e4c:     	movq	%rsi, %rcx
140500e4f:     	movq	%rbx, %rdx
140500e52:     	callq	0x140505080 <.text+0x504080>
140500e57:     	nop
140500e58:     	addq	$0xd8, %rsp
140500e5f:     	popq	%rbx
140500e60:     	popq	%rbp
140500e61:     	popq	%rdi
140500e62:     	popq	%rsi
140500e63:     	popq	%r12
140500e65:     	popq	%r13
140500e67:     	popq	%r14
140500e69:     	popq	%r15
140500e6b:     	retq
