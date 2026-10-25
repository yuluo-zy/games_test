
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141c3de00:     	pushq	%rbp
141c3de01:     	pushq	%rsi
141c3de02:     	pushq	%rdi
141c3de03:     	subq	$0x100, %rsp            # imm = 0x100
141c3de0a:     	leaq	0x80(%rsp), %rbp
141c3de12:     	movq	$-0x2, 0x78(%rbp)
141c3de1a:     	movq	%rdx, %rdi
141c3de1d:     	movq	%rcx, %rsi
141c3de20:     	movb	$0x1, %cl
141c3de22:     	callq	0x1409e8d90 <.text+0x9e7d90>
141c3de27:     	movw	%ax, 0x44(%rbp)
141c3de2b:     	movl	$0xc9, 0x40(%rbp)
141c3de32:     	movzwl	0x1c4(%rdi), %r8d
141c3de3a:     	cmpl	$0x1, %r8d
141c3de3e:     	je	0x141c3dee5 <.text+0x1c3cee5>
141c3de44:     	cmpl	$0x2, %r8d
141c3de48:     	jne	0x141c3dfa9 <.text+0x1c3cfa9>
141c3de4e:     	movups	0x40(%rbp), %xmm0
141c3de52:     	movups	0x50(%rbp), %xmm1
141c3de56:     	movups	0x60(%rbp), %xmm2
141c3de5a:     	movaps	%xmm2, 0x20(%rbp)
141c3de5e:     	movaps	%xmm1, 0x10(%rbp)
141c3de62:     	movaps	%xmm0, (%rbp)
141c3de66:     	movzbl	0x19f6d9c(%rip), %eax   # 0x143634c09
141c3de6d:     	movl	$0x20, %ecx
141c3de72:     	movl	$0x8, %edx
141c3de77:     	callq	0x140613c10 <.text+0x612c10>
141c3de7c:     	testq	%rax, %rax
141c3de7f:     	movq	%rax, 0x38(%rbp)
141c3de83:     	je	0x141c3dffa <.text+0x1c3cffa>
141c3de89:     	leaq	0x12d0f78(%rip), %rcx   # 0x142f0ee08
141c3de90:     	movq	%rcx, (%rax)
141c3de93:     	movq	$0xa, 0x8(%rax)
141c3de9b:     	leaq	0x12d0f78(%rip), %rcx   # 0x142f0ee1a
141c3dea2:     	movq	%rcx, 0x10(%rax)
141c3dea6:     	movq	$0x8, 0x18(%rax)
141c3deae:     	movq	$0x2, -0x28(%rbp)
141c3deb6:     	movq	%rax, -0x20(%rbp)
141c3deba:     	movq	$0x2, -0x18(%rbp)
141c3dec2:     	movl	$0x3f333333, -0x10(%rbp) # imm = 0x3F333333
141c3dec9:     	movl	$0x42c80000, -0x8(%rbp) # imm = 0x42C80000
141c3ded0:     	leaq	-0x58(%rbp), %rcx
141c3ded4:     	movq	%rbp, %rdx
141c3ded7:     	leaq	-0x28(%rbp), %r8
141c3dedb:     	callq	0x141c4e8b0 <.text+0x1c4d8b0>
141c3dee0:     	jmp	0x141c3df8f <.text+0x1c3cf8f>
141c3dee5:     	movl	0x40(%rbp), %eax
141c3dee8:     	movzwl	0x44(%rbp), %ecx
141c3deec:     	movq	0x46(%rbp), %rdx
141c3def0:     	movzwl	0x4e(%rbp), %r8d
141c3def5:     	movaps	0x50(%rbp), %xmm0
141c3def9:     	movaps	0x60(%rbp), %xmm1
141c3defd:     	movaps	%xmm1, 0x20(%rbp)
141c3df01:     	movaps	%xmm0, 0x10(%rbp)
141c3df05:     	movl	%eax, (%rbp)
141c3df08:     	movw	%cx, 0x4(%rbp)
141c3df0c:     	movq	%rdx, 0x6(%rbp)
141c3df10:     	movw	%r8w, 0xe(%rbp)
141c3df15:     	movzbl	0x19f6ced(%rip), %eax   # 0x143634c09
141c3df1c:     	movl	$0x20, %ecx
141c3df21:     	movl	$0x8, %edx
141c3df26:     	callq	0x140613c10 <.text+0x612c10>
141c3df2b:     	testq	%rax, %rax
141c3df2e:     	movq	%rax, 0x38(%rbp)
141c3df32:     	je	0x141c3e00b <.text+0x1c3d00b>
141c3df38:     	leaq	0x12d0ec9(%rip), %rcx   # 0x142f0ee08
141c3df3f:     	movq	%rcx, (%rax)
141c3df42:     	movq	$0xa, 0x8(%rax)
141c3df4a:     	leaq	0x12d0ec1(%rip), %rcx   # 0x142f0ee12
141c3df51:     	movq	%rcx, 0x10(%rax)
141c3df55:     	movq	$0x8, 0x18(%rax)
141c3df5d:     	movq	$0x2, -0x28(%rbp)
141c3df65:     	movq	%rax, -0x20(%rbp)
141c3df69:     	movq	$0x2, -0x18(%rbp)
141c3df71:     	movl	$0x3f333333, -0x10(%rbp) # imm = 0x3F333333
141c3df78:     	movl	$0x42c80000, -0x8(%rbp) # imm = 0x42C80000
141c3df7f:     	leaq	-0x58(%rbp), %rcx
141c3df83:     	movq	%rbp, %rdx
141c3df86:     	leaq	-0x28(%rbp), %r8
141c3df8a:     	callq	0x141c4e8b0 <.text+0x1c4d8b0>
141c3df8f:     	movups	-0x58(%rbp), %xmm0
141c3df93:     	movups	-0x48(%rbp), %xmm1
141c3df97:     	movups	-0x38(%rbp), %xmm2
141c3df9b:     	movaps	%xmm0, 0x40(%rbp)
141c3df9f:     	movaps	%xmm1, 0x50(%rbp)
141c3dfa3:     	movaps	%xmm2, 0x60(%rbp)
141c3dfa7:     	jmp	0x141c3dfd5 <.text+0x1c3cfd5>
141c3dfa9:     	movaps	0x40(%rbp), %xmm0
141c3dfad:     	movaps	0x50(%rbp), %xmm1
141c3dfb1:     	movaps	0x60(%rbp), %xmm2
141c3dfb5:     	movaps	%xmm2, 0x20(%rbp)
141c3dfb9:     	movaps	%xmm1, 0x10(%rbp)
141c3dfbd:     	movaps	%xmm0, (%rbp)
141c3dfc1:     	movzwl	0x1c6(%rdi), %r9d
141c3dfc9:     	leaq	0x40(%rbp), %rcx
141c3dfcd:     	movq	%rbp, %rdx
141c3dfd0:     	callq	0x141c4e6d0 <.text+0x1c4d6d0>
141c3dfd5:     	movaps	0x40(%rbp), %xmm0
141c3dfd9:     	movaps	0x50(%rbp), %xmm1
141c3dfdd:     	movaps	0x60(%rbp), %xmm2
141c3dfe1:     	movups	%xmm2, 0x20(%rsi)
141c3dfe5:     	movups	%xmm1, 0x10(%rsi)
141c3dfe9:     	movups	%xmm0, (%rsi)
141c3dfec:     	movq	%rsi, %rax
141c3dfef:     	addq	$0x100, %rsp            # imm = 0x100
141c3dff6:     	popq	%rdi
141c3dff7:     	popq	%rsi
141c3dff8:     	popq	%rbp
141c3dff9:     	retq
141c3dffa:     	movl	$0x8, %ecx
141c3dfff:     	movl	$0x20, %edx
141c3e004:     	callq	0x1428d9000 <.text+0x28d8000>
141c3e009:     	jmp	0x141c3e01a <.text+0x1c3d01a>
141c3e00b:     	movl	$0x8, %ecx
141c3e010:     	movl	$0x20, %edx
141c3e015:     	callq	0x1428d9000 <.text+0x28d8000>
141c3e01a:     	ud2
141c3e01c:     	nopl	(%rax)
141c3e020:     	movq	%rdx, 0x10(%rsp)
141c3e025:     	pushq	%rbp
141c3e026:     	pushq	%rsi
141c3e027:     	pushq	%rdi
141c3e028:     	subq	$0x20, %rsp
141c3e02c:     	leaq	0x80(%rdx), %rbp
141c3e033:     	cmpq	$0x0, 0x38(%rbp)
141c3e038:     	je	0x141c3e042 <.text+0x1c3d042>
141c3e03a:     	addq	$0x20, %rsp
141c3e03e:     	popq	%rdi
141c3e03f:     	popq	%rsi
141c3e040:     	popq	%rbp
141c3e041:     	retq
141c3e042:     	movq	%rbp, %rcx
141c3e045:     	callq	0x141c3d6d0 <.text+0x1c3c6d0>
141c3e04a:     	jmp	0x141c3e03a <.text+0x1c3d03a>
141c3e04c:     	nopl	(%rax)
141c3e050:     	movq	%rdx, 0x10(%rsp)
141c3e055:     	pushq	%rbp
141c3e056:     	pushq	%rsi
141c3e057:     	pushq	%rdi
141c3e058:     	subq	$0x20, %rsp
141c3e05c:     	leaq	0x80(%rdx), %rbp
141c3e063:     	cmpq	$0x0, 0x38(%rbp)
141c3e068:     	je	0x141c3e072 <.text+0x1c3d072>
141c3e06a:     	addq	$0x20, %rsp
141c3e06e:     	popq	%rdi
141c3e06f:     	popq	%rsi
