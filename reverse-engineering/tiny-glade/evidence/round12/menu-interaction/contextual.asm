
D:\game\reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141e23d20:     	pushq	%r15
141e23d22:     	pushq	%r14
141e23d24:     	pushq	%r12
141e23d26:     	pushq	%rsi
141e23d27:     	pushq	%rdi
141e23d28:     	pushq	%rbx
141e23d29:     	subq	$0xc8, %rsp
141e23d30:     	movaps	%xmm7, 0xb0(%rsp)
141e23d38:     	movaps	%xmm6, 0xa0(%rsp)
141e23d40:     	movaps	%xmm2, %xmm7
141e23d43:     	movaps	%xmm1, %xmm6
141e23d46:     	movq	0x130(%rsp), %rbx
141e23d4e:     	movq	0x128(%rsp), %r14
141e23d56:     	movq	0x120(%rsp), %r15
141e23d5e:     	movzbl	0x140(%rsp), %eax
141e23d66:     	movzbl	0x138(%rsp), %edx
141e23d6e:     	movq	%r9, 0x30(%rsp)
141e23d73:     	movb	%dl, 0x2e(%rsp)
141e23d77:     	movb	%al, 0x2f(%rsp)
141e23d7b:     	movl	0x1c(%rcx), %eax
141e23d7e:     	movq	0x10(%rcx), %rdx
141e23d82:     	movl	%eax, (%rdx)
141e23d84:     	movq	(%rcx), %rsi
141e23d87:     	leaq	0x1164666(%rip), %rax   # 0x142f883f4
141e23d8e:     	movq	%rax, 0x78(%rsp)
141e23d93:     	movq	$0x6, 0x80(%rsp)
141e23d9f:     	leaq	0x78(%rsp), %rcx
141e23da4:     	callq	0x1405bdda0 <.text+0x5bcda0>
141e23da9:     	movabsq	$-0x4e96acdfffb04bbd, %rdi # imm = 0xB1695320004FB443
141e23db3:     	xorq	%rax, %rdi
141e23db6:     	movq	%rsi, %rcx
141e23db9:     	movq	%rdi, %rdx
141e23dbc:     	callq	0x140683ec0 <.text+0x682ec0>
141e23dc1:     	movb	$0x0, 0x5d(%rax)
141e23dc5:     	testb	$0x1, %dl
141e23dc8:     	jne	0x141e23e2a <.text+0x1e22e2a>
141e23dca:     	unpcklps	%xmm7, %xmm6            # xmm6 = xmm6[0],xmm7[0],xmm6[1],xmm7[1]
141e23dcd:     	addps	0x116412c(%rip), %xmm6  # 0x142f87f00
141e23dd4:     	movq	0x10(%rsi), %r12
141e23dd8:     	cmpq	(%rsi), %r12
141e23ddb:     	jne	0x141e23dec <.text+0x1e22dec>
141e23ddd:     	leaq	0xd8b1e4(%rip), %rdx    # 0x142baefc8
141e23de4:     	movq	%rsi, %rcx
141e23de7:     	callq	0x1406319d0 <.text+0x6309d0>
141e23dec:     	movq	0x8(%rsi), %rax
141e23df0:     	imulq	$0x38, %r12, %rcx
141e23df4:     	unpcklpd	0xc649c4(%rip), %xmm6   # xmm6 = xmm6[0],mem[0]
                                                # 0x142a887c0
141e23dfc:     	movl	$0x4, (%rax,%rcx)
141e23e03:     	movups	%xmm6, 0x4(%rax,%rcx)
141e23e08:     	movl	$0x1, 0x14(%rax,%rcx)
141e23e10:     	movsd	0x11640d8(%rip), %xmm0  # 0x142f87ef0
141e23e18:     	movsd	%xmm0, 0x18(%rax,%rcx)
141e23e1e:     	movq	%rdi, 0x30(%rax,%rcx)
141e23e23:     	incq	%r12
141e23e26:     	movq	%r12, 0x10(%rsi)
141e23e2a:     	movq	%rsi, 0x38(%rsp)
141e23e2f:     	movq	%rdi, 0x40(%rsp)
141e23e34:     	movq	$0x0, 0x48(%rsp)
141e23e3d:     	movq	$0x0, 0x20(%rsp)
141e23e46:     	leaq	0x11643f7(%rip), %r8    # 0x142f88244
141e23e4d:     	leaq	0x78(%rsp), %rsi
141e23e52:     	leaq	0x38(%rsp), %rdx
141e23e57:     	movl	$0x18, %r9d
141e23e5d:     	movq	%rsi, %rcx
141e23e60:     	callq	0x140f9e970 <.text+0xf9d970>
141e23e65:     	leaq	0x38(%rbx), %rax
141e23e69:     	leaq	0x49(%rbx), %rcx
141e23e6d:     	addq	$0x10, %rbx
141e23e71:     	leaq	0x2e(%rsp), %rdx
141e23e76:     	movq	%rdx, 0x38(%rsp)
141e23e7b:     	leaq	0x2f(%rsp), %rdx
141e23e80:     	movq	%rdx, 0x40(%rsp)
141e23e85:     	leaq	0x30(%rsp), %rdx
141e23e8a:     	movq	%rdx, 0x48(%rsp)
141e23e8f:     	movq	%r15, 0x50(%rsp)
141e23e94:     	movq	%r14, 0x58(%rsp)
141e23e99:     	movq	%rax, 0x60(%rsp)
141e23e9e:     	movq	%rcx, 0x68(%rsp)
141e23ea3:     	movq	%rbx, 0x70(%rsp)
141e23ea8:     	leaq	0x11643ad(%rip), %rdx   # 0x142f8825c
141e23eaf:     	leaq	0x38(%rsp), %r9
141e23eb4:     	movl	$0x4, %r8d
141e23eba:     	movq	%rsi, %rcx
141e23ebd:     	callq	0x140fc26c0 <.text+0xfc16c0>
141e23ec2:     	movaps	0xa0(%rsp), %xmm6
141e23eca:     	movaps	0xb0(%rsp), %xmm7
141e23ed2:     	addq	$0xc8, %rsp
141e23ed9:     	popq	%rbx
141e23eda:     	popq	%rdi
141e23edb:     	popq	%rsi
141e23edc:     	popq	%r12
141e23ede:     	popq	%r14
141e23ee0:     	popq	%r15
141e23ee2:     	retq
