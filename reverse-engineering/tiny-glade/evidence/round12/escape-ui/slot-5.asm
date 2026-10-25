
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140503c00:     	pushq	%r15
140503c02:     	pushq	%r14
140503c04:     	pushq	%r13
140503c06:     	pushq	%r12
140503c08:     	pushq	%rsi
140503c09:     	pushq	%rdi
140503c0a:     	pushq	%rbp
140503c0b:     	pushq	%rbx
140503c0c:     	subq	$0xf8, %rsp
140503c13:     	movq	%r9, %rsi
140503c16:     	movq	%rcx, %rbx
140503c19:     	movq	%rdx, 0x68(%rsp)
140503c1e:     	movq	%r8, 0x70(%rsp)
140503c23:     	leaq	0x10(%rcx), %rdi
140503c27:     	movq	%rdi, %rcx
140503c2a:     	callq	0x1406868b0 <.text+0x6858b0>
140503c2f:     	movq	%rax, %r12
140503c32:     	leaq	0x68(%rsp), %r15
140503c37:     	movq	%r15, %rcx
140503c3a:     	callq	0x140683c10 <.text+0x682c10>
140503c3f:     	movq	%rax, 0x48(%rsp)
140503c44:     	leaq	0x48(%rsp), %r14
140503c49:     	movq	%r14, %rcx
140503c4c:     	callq	0x1400885d0 <.text+0x875d0>
140503c51:     	movq	%rax, 0x78(%rsp)
140503c56:     	movabsq	$0x41223169ff28813b, %r13 # imm = 0x41223169FF28813B
140503c60:     	movq	%r13, 0x80(%rsp)
140503c68:     	movabsq	$-0x58648d975d569727, %rbp # imm = 0xA79B7268A2A968D9
140503c72:     	movq	%rbp, 0x88(%rsp)
140503c7a:     	leaq	0x78(%rsp), %rcx
140503c7f:     	callq	0x1405bdf20 <.text+0x5bcf20>
140503c84:     	movq	%r12, 0x78(%rsp)
140503c89:     	movq	%rax, 0x80(%rsp)
140503c91:     	leaq	0x78(%rsp), %rcx
140503c96:     	callq	0x1405bd910 <.text+0x5bc910>
140503c9b:     	movq	%rax, %r12
140503c9e:     	movq	%rdi, %rcx
140503ca1:     	movq	%rax, %rdx
140503ca4:     	callq	0x140686450 <.text+0x685450>
140503ca9:     	testb	%al, %al
140503cab:     	je	0x140503cc5 <.text+0x502cc5>
140503cad:     	leaq	0x48(%rsp), %rbx
140503cb2:     	movq	%rbx, %rcx
140503cb5:     	movq	%rdi, %rdx
140503cb8:     	movq	%r12, %r8
140503cbb:     	callq	0x140686530 <.text+0x685530>
140503cc0:     	jmp	0x140503e1a <.text+0x502e1a>
140503cc5:     	movq	0x31286dc(%rip), %rax   # 0x14362c3a8
140503ccc:     	movq	(%rax), %rax
140503ccf:     	testq	%rax, %rax
140503cd2:     	je	0x140503db9 <.text+0x502db9>
140503cd8:     	movq	%rbx, 0x48(%rsp)
140503cdd:     	leaq	0x2fbbc(%rip), %rax     # 0x1405338a0 <.text+0x5328a0>
140503ce4:     	movq	%rax, 0x50(%rsp)
140503ce9:     	movq	%r15, 0x58(%rsp)
140503cee:     	movq	%rax, 0x60(%rsp)
140503cf3:     	leaq	0x24ea05e(%rip), %rcx   # 0x1429edd58
140503cfa:     	callq	0x14067dd80 <.text+0x67cd80>
140503cff:     	movups	(%rax), %xmm0
140503d02:     	movl	0x10(%rax), %eax
140503d05:     	movq	$0x1, 0xa8(%rsp)
140503d11:     	leaq	0x24ea010(%rip), %rcx   # 0x1429edd28
140503d18:     	movq	%rcx, 0xb0(%rsp)
140503d20:     	movq	$0x14, 0xb8(%rsp)
140503d2c:     	leaq	0x24e9f65(%rip), %rdx   # 0x1429edc98
140503d33:     	movq	%rdx, 0xc8(%rsp)
140503d3b:     	movq	$0x2, 0xd0(%rsp)
140503d47:     	movq	%r14, 0xd8(%rsp)
140503d4f:     	movq	$0x2, 0xe0(%rsp)
140503d5b:     	movq	$0x0, 0xe8(%rsp)
140503d67:     	movq	$0x0, 0x78(%rsp)
140503d70:     	movq	%rcx, 0x80(%rsp)
140503d78:     	movq	$0x14, 0x88(%rsp)
140503d84:     	movq	$0x0, 0x90(%rsp)
140503d90:     	movups	%xmm0, 0x98(%rsp)
140503d98:     	movl	$0x1, 0xc0(%rsp)
140503da3:     	movl	%eax, 0xc4(%rsp)
140503daa:     	leaq	0x47(%rsp), %rcx
140503daf:     	leaq	0x78(%rsp), %rdx
140503db4:     	callq	0x140d4a330 <.text+0xd49330>
140503db9:     	movups	0x68(%rsp), %xmm0
140503dbe:     	movq	0x18(%rbx), %rbx
140503dc2:     	movups	%xmm0, 0x48(%rsp)
140503dc7:     	leaq	0x48(%rsp), %rcx
140503dcc:     	callq	0x140683c10 <.text+0x682c10>
140503dd1:     	movq	%rax, 0x78(%rsp)
140503dd6:     	movq	%r13, 0x80(%rsp)
140503dde:     	movq	%rbp, 0x88(%rsp)
140503de6:     	leaq	0x78(%rsp), %rcx
140503deb:     	callq	0x1405bdf20 <.text+0x5bcf20>
140503df0:     	movq	%rbx, 0x78(%rsp)
140503df5:     	movq	%rax, 0x80(%rsp)
140503dfd:     	leaq	0x78(%rsp), %rcx
140503e02:     	callq	0x1405bd910 <.text+0x5bc910>
140503e07:     	leaq	0x48(%rsp), %rbx
140503e0c:     	movq	%rbx, %rcx
140503e0f:     	movq	%rdi, %rdx
140503e12:     	movq	%rax, %r8
140503e15:     	callq	0x1406861b0 <.text+0x6851b0>
140503e1a:     	movq	%rbx, %rcx
140503e1d:     	callq	0x140683ce0 <.text+0x682ce0>
140503e22:     	movl	%eax, %ecx
140503e24:     	xorb	$0x1, %cl
140503e27:     	movzbl	%cl, %ecx
140503e2a:     	leaq	0xf(,%rcx,2), %rcx
140503e32:     	leaq	0x24ea049(%rip), %rdx   # 0x1429ede82
140503e39:     	leaq	0x24ea031(%rip), %r8    # 0x1429ede71
140503e40:     	testb	%al, %al
140503e42:     	cmovneq	%rdx, %r8
140503e46:     	movq	%r8, 0x20(%rsp)
140503e4b:     	movq	%rcx, 0x28(%rsp)
140503e50:     	movl	$0x42a80000, 0x30(%rsp) # imm = 0x42A80000
140503e58:     	leaq	0x24ea032(%rip), %r8    # 0x1429ede91
140503e5f:     	leaq	0x78(%rsp), %rdi
140503e64:     	movl	$0xe, %r9d
140503e6a:     	movq	%rdi, %rcx
140503e6d:     	movq	%rbx, %rdx
140503e70:     	callq	0x14032e0b0 <.text+0x32d0b0>
140503e75:     	movq	%rdi, %rcx
140503e78:     	callq	0x140681d50 <.text+0x680d50>
140503e7d:     	movq	%rax, %rcx
140503e80:     	callq	0x140683c80 <.text+0x682c80>
140503e85:     	testb	%al, %al
140503e87:     	je	0x140503e91 <.text+0x502e91>
140503e89:     	movq	%rsi, %rcx
140503e8c:     	callq	0x1403d0af0 <.text+0x3cfaf0>
140503e91:     	nop
140503e92:     	addq	$0xf8, %rsp
140503e99:     	popq	%rbx
140503e9a:     	popq	%rbp
140503e9b:     	popq	%rdi
140503e9c:     	popq	%rsi
140503e9d:     	popq	%r12
140503e9f:     	popq	%r13
140503ea1:     	popq	%r14
140503ea3:     	popq	%r15
140503ea5:     	retq
