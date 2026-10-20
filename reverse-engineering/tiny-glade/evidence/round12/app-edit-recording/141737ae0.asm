
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141737ae0:     	pushq	%rbp
141737ae1:     	pushq	%rsi
141737ae2:     	pushq	%rdi
141737ae3:     	pushq	%rbx
141737ae4:     	subq	$0x2a8, %rsp            # imm = 0x2A8
141737aeb:     	leaq	0x80(%rsp), %rbp
141737af3:     	movq	$-0x2, 0x220(%rbp)
141737afe:     	movq	%rcx, %rsi
141737b01:     	movups	0x60(%rdx), %xmm0
141737b05:     	movaps	%xmm0, 0x70(%rbp)
141737b09:     	movups	0x50(%rdx), %xmm0
141737b0d:     	movaps	%xmm0, 0x60(%rbp)
141737b11:     	movups	0x40(%rdx), %xmm0
141737b15:     	movaps	%xmm0, 0x50(%rbp)
141737b19:     	movups	(%rdx), %xmm0
141737b1c:     	movups	0x10(%rdx), %xmm1
141737b20:     	movups	0x20(%rdx), %xmm2
141737b24:     	movups	0x30(%rdx), %xmm3
141737b28:     	movaps	%xmm3, 0x40(%rbp)
141737b2c:     	movaps	%xmm2, 0x30(%rbp)
141737b30:     	movaps	%xmm1, 0x20(%rbp)
141737b34:     	movaps	%xmm0, 0x10(%rbp)
141737b38:     	movzbl	0x1efd0ca(%rip), %eax   # 0x143634c09
141737b3f:     	movl	$0x150, %ecx            # imm = 0x150
141737b44:     	movl	$0x8, %edx
141737b49:     	callq	0x140613c10 <.text+0x612c10>
141737b4e:     	testq	%rax, %rax
141737b51:     	je	0x141737e87 <.text+0x1736e87>
141737b57:     	movq	%rax, 0x218(%rbp)
141737b5e:     	xorps	%xmm0, %xmm0
141737b61:     	movups	%xmm0, 0xf8(%rbp)
141737b68:     	movups	%xmm0, 0x110(%rbp)
141737b6f:     	movups	%xmm0, 0x128(%rbp)
141737b76:     	movups	%xmm0, 0x140(%rbp)
141737b7d:     	movups	%xmm0, 0x158(%rbp)
141737b84:     	movl	$0x0, 0x168(%rbp)
141737b8e:     	movups	%xmm0, 0x190(%rbp)
141737b95:     	movups	%xmm0, 0x1a8(%rbp)
141737b9c:     	movups	%xmm0, 0x1c0(%rbp)
141737ba3:     	movups	%xmm0, 0x1d8(%rbp)
141737baa:     	movups	%xmm0, 0x1f0(%rbp)
141737bb1:     	movl	$0x0, 0x200(%rbp)
141737bbb:     	movq	$0x0, 0x80(%rbp)
141737bc6:     	movq	$0x0, 0xd8(%rbp)
141737bd1:     	movq	$0x8, 0xe0(%rbp)
141737bdc:     	movq	$0x0, 0xe8(%rbp)
141737be7:     	movq	$0x10, 0xf0(%rbp)
141737bf2:     	movq	$0x10, 0x108(%rbp)
141737bfd:     	movq	$0x10, 0x120(%rbp)
141737c08:     	movq	$0x10, 0x138(%rbp)
141737c13:     	movq	$0x10, 0x150(%rbp)
141737c1e:     	movabsq	$-0x8000000000000000, %rbx # imm = 0x8000000000000000
141737c28:     	movq	%rbx, 0x170(%rbp)
141737c2f:     	leaq	0x1595897(%rip), %rax   # 0x142ccd4cd
141737c36:     	movq	%rax, 0x178(%rbp)
141737c3d:     	movq	$0x44, 0x180(%rbp)
141737c48:     	movq	$0x10, 0x188(%rbp)
141737c53:     	movq	$0x10, 0x1a0(%rbp)
141737c5e:     	movq	$0x10, 0x1b8(%rbp)
141737c69:     	movq	$0x10, 0x1d0(%rbp)
141737c74:     	movq	$0x10, 0x1e8(%rbp)
141737c7f:     	movl	$0x0, 0x208(%rbp)
141737c89:     	movw	$0x1, 0x20c(%rbp)
141737c92:     	movl	$0x0, 0x210(%rbp)
141737c9c:     	movzbl	0x1efcf66(%rip), %eax   # 0x143634c09
141737ca3:     	movl	$0x198, %ecx            # imm = 0x198
141737ca8:     	movl	$0x8, %edx
141737cad:     	callq	0x140613c10 <.text+0x612c10>
141737cb2:     	testq	%rax, %rax
141737cb5:     	je	0x141737e98 <.text+0x1736e98>
141737cbb:     	movq	%rax, %rdi
141737cbe:     	leaq	0x80(%rbp), %rdx
141737cc5:     	movl	$0x198, %r8d            # imm = 0x198
141737ccb:     	movq	%rax, %rcx
141737cce:     	callq	0x142923c00 <.text+0x2922c00>
141737cd3:     	leaq	0x159571e(%rip), %r8    # 0x142ccd3f8
141737cda:     	leaq	-0x60(%rbp), %rcx
141737cde:     	movq	%rdi, %rdx
141737ce1:     	callq	0x140ce4b10 <.text+0xce3b10>
141737ce6:     	leaq	0x80(%rbp), %rcx
141737ced:     	callq	0x141740460 <.text+0x173f460>
141737cf2:     	movaps	0x70(%rbp), %xmm0
141737cf6:     	movq	0x218(%rbp), %rax
141737cfd:     	movups	%xmm0, 0x140(%rax)
141737d04:     	movaps	0x60(%rbp), %xmm0
141737d08:     	movups	%xmm0, 0x130(%rax)
141737d0f:     	movaps	0x50(%rbp), %xmm0
141737d13:     	movups	%xmm0, 0x120(%rax)
141737d1a:     	movaps	0x10(%rbp), %xmm0
141737d1e:     	movaps	0x20(%rbp), %xmm1
141737d22:     	movaps	0x30(%rbp), %xmm2
141737d26:     	movaps	0x40(%rbp), %xmm3
141737d2a:     	movups	%xmm3, 0x110(%rax)
141737d31:     	movups	%xmm2, 0x100(%rax)
141737d38:     	movups	%xmm1, 0xf0(%rax)
141737d3f:     	movups	%xmm0, 0xe0(%rax)
141737d46:     	movups	-0x60(%rbp), %xmm0
141737d4a:     	movups	-0x50(%rbp), %xmm1
141737d4e:     	movups	-0x40(%rbp), %xmm2
141737d52:     	movups	-0x30(%rbp), %xmm3
141737d56:     	movups	%xmm0, (%rax)
141737d59:     	movups	%xmm1, 0x10(%rax)
141737d5d:     	movups	%xmm2, 0x20(%rax)
141737d61:     	movups	%xmm3, 0x30(%rax)
141737d65:     	movups	-0x20(%rbp), %xmm0
141737d69:     	movups	%xmm0, 0x40(%rax)
141737d6d:     	movups	-0x10(%rbp), %xmm0
141737d71:     	movups	%xmm0, 0x50(%rax)
141737d75:     	movups	(%rbp), %xmm0
141737d79:     	movups	%xmm0, 0x60(%rax)
141737d7d:     	movups	0x80(%rbp), %xmm0
141737d84:     	movups	0x90(%rbp), %xmm1
141737d8b:     	movups	0xa0(%rbp), %xmm2
141737d92:     	movups	0xb0(%rbp), %xmm3
141737d99:     	movups	%xmm0, 0x70(%rax)
141737d9d:     	movups	%xmm1, 0x80(%rax)
141737da4:     	movups	%xmm2, 0x90(%rax)
141737dab:     	movups	%xmm3, 0xa0(%rax)
141737db2:     	movups	0xc0(%rbp), %xmm0
141737db9:     	movups	%xmm0, 0xb0(%rax)
141737dc0:     	movups	0xd0(%rbp), %xmm0
141737dc7:     	movups	%xmm0, 0xc0(%rax)
141737dce:     	movups	0xe0(%rbp), %xmm0
141737dd5:     	movups	%xmm0, 0xd0(%rax)
141737ddc:     	movq	$0x3, 0x88(%rbp)
141737de7:     	movq	%rax, 0x90(%rbp)
141737dee:     	movq	$0x3, 0x98(%rbp)
141737df9:     	movq	$0x0, 0xa0(%rbp)
141737e04:     	movq	$0x8, 0xa8(%rbp)
141737e0f:     	xorps	%xmm0, %xmm0
141737e12:     	movups	%xmm0, 0xb0(%rbp)
141737e19:     	movq	%rbx, 0x80(%rbp)
141737e20:     	leaq	0xb8(%rbp), %rcx
141737e27:     	callq	0x1416a00a0 <.text+0x169f0a0>
141737e2c:     	movups	0xe0(%rbp), %xmm0
141737e33:     	movups	%xmm0, 0x60(%rsi)
141737e37:     	movups	0xd0(%rbp), %xmm0
141737e3e:     	movups	%xmm0, 0x50(%rsi)
141737e42:     	movups	0xc0(%rbp), %xmm0
141737e49:     	movups	%xmm0, 0x40(%rsi)
141737e4d:     	movups	0x80(%rbp), %xmm0
141737e54:     	movups	0x90(%rbp), %xmm1
141737e5b:     	movups	0xa0(%rbp), %xmm2
141737e62:     	movups	0xb0(%rbp), %xmm3
141737e69:     	movups	%xmm3, 0x30(%rsi)
141737e6d:     	movups	%xmm2, 0x20(%rsi)
141737e71:     	movups	%xmm1, 0x10(%rsi)
141737e75:     	movups	%xmm0, (%rsi)
141737e78:     	movq	%rsi, %rax
141737e7b:     	addq	$0x2a8, %rsp            # imm = 0x2A8
141737e82:     	popq	%rbx
141737e83:     	popq	%rdi
141737e84:     	popq	%rsi
141737e85:     	popq	%rbp
141737e86:     	retq
141737e87:     	movl	$0x8, %ecx
141737e8c:     	movl	$0x150, %edx            # imm = 0x150
141737e91:     	callq	0x1428d9000 <.text+0x28d8000>
141737e96:     	jmp	0x141737ea7 <.text+0x1736ea7>
141737e98:     	movl	$0x8, %ecx
141737e9d:     	movl	$0x198, %edx            # imm = 0x198
141737ea2:     	callq	0x1428d9000 <.text+0x28d8000>
141737ea7:     	ud2
