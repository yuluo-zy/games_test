
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140335b30:     	pushq	%rbp
140335b31:     	pushq	%rsi
140335b32:     	pushq	%rdi
140335b33:     	pushq	%rbx
140335b34:     	subq	$0xa68, %rsp            # imm = 0xA68
140335b3b:     	leaq	0x80(%rsp), %rbp
140335b43:     	movq	$-0x2, 0x9e0(%rbp)
140335b4e:     	movq	%rcx, %rsi
140335b51:     	movzbl	0x32ff0b1(%rip), %eax   # 0x143634c09
140335b58:     	movl	$0x700, %ecx            # imm = 0x700
140335b5d:     	movl	$0x8, %edx
140335b62:     	callq	0x140613c10 <.text+0x612c10>
140335b67:     	testq	%rax, %rax
140335b6a:     	je	0x1403363fa <.text+0x3353fa>
140335b70:     	movq	%rax, 0x9d8(%rbp)
140335b77:     	xorps	%xmm0, %xmm0
140335b7a:     	movups	%xmm0, -0x40(%rbp)
140335b7e:     	movups	%xmm0, -0x28(%rbp)
140335b82:     	movups	%xmm0, -0x10(%rbp)
140335b86:     	movups	%xmm0, 0x8(%rbp)
140335b8a:     	movups	%xmm0, 0x20(%rbp)
140335b8e:     	movl	$0x0, 0x30(%rbp)
140335b95:     	movups	%xmm0, 0x58(%rbp)
140335b99:     	movups	%xmm0, 0x70(%rbp)
140335b9d:     	movups	%xmm0, 0x88(%rbp)
140335ba4:     	movups	%xmm0, 0xa0(%rbp)
140335bab:     	movups	%xmm0, 0xb8(%rbp)
140335bb2:     	movl	$0x0, 0xc8(%rbp)
140335bbc:     	movq	$0x0, -0x60(%rbp)
140335bc4:     	movq	$0x8, -0x58(%rbp)
140335bcc:     	movq	$0x0, -0x50(%rbp)
140335bd4:     	movq	$0x10, -0x48(%rbp)
140335bdc:     	movq	$0x10, -0x30(%rbp)
140335be4:     	movq	$0x10, -0x18(%rbp)
140335bec:     	movq	$0x10, (%rbp)
140335bf4:     	movq	$0x10, 0x18(%rbp)
140335bfc:     	movabsq	$-0x8000000000000000, %rbx # imm = 0x8000000000000000
140335c06:     	movq	%rbx, 0x38(%rbp)
140335c0a:     	leaq	0x268a19f(%rip), %rax   # 0x1429bfdb0
140335c11:     	movq	%rax, 0x40(%rbp)
140335c15:     	movq	$0x6f, 0x48(%rbp)
140335c1d:     	movq	$0x10, 0x50(%rbp)
140335c25:     	movq	$0x10, 0x68(%rbp)
140335c2d:     	movq	$0x10, 0x80(%rbp)
140335c38:     	movq	$0x10, 0x98(%rbp)
140335c43:     	movq	$0x10, 0xb0(%rbp)
140335c4e:     	movl	$0x0, 0xd0(%rbp)
140335c58:     	movw	$0x1, 0xd4(%rbp)
140335c61:     	movq	%rbx, 0x1c8(%rbp)
140335c68:     	movl	$0x0, 0x340(%rbp)
140335c72:     	movzbl	0x32fef90(%rip), %eax   # 0x143634c09
140335c79:     	movl	$0x3a8, %ecx            # imm = 0x3A8
140335c7e:     	movl	$0x8, %edx
140335c83:     	callq	0x140613c10 <.text+0x612c10>
140335c88:     	testq	%rax, %rax
140335c8b:     	je	0x140336409 <.text+0x335409>
140335c91:     	movq	%rax, %rdi
140335c94:     	leaq	-0x60(%rbp), %rdx
140335c98:     	movl	$0x3a8, %r8d            # imm = 0x3A8
140335c9e:     	movq	%rax, %rcx
140335ca1:     	callq	0x142923c00 <.text+0x2922c00>
140335ca6:     	leaq	0x2689ecb(%rip), %r8    # 0x1429bfb78
140335cad:     	leaq	0x8f8(%rbp), %rcx
140335cb4:     	movq	%rdi, %rdx
140335cb7:     	callq	0x140ce4b10 <.text+0xce3b10>
140335cbc:     	leaq	0x888(%rbp), %rcx
140335cc3:     	callq	0x1400cbb50 <.text+0xcab50>
140335cc8:     	leaq	0x818(%rbp), %rcx
140335ccf:     	callq	0x14015ccb0 <.text+0x15bcb0>
140335cd4:     	leaq	0x7a8(%rbp), %rcx
140335cdb:     	callq	0x140112b10 <.text+0x111b10>
140335ce0:     	leaq	0x738(%rbp), %rcx
140335ce7:     	callq	0x1405b0b70 <.text+0x5afb70>
140335cec:     	leaq	0x6c8(%rbp), %rcx
140335cf3:     	callq	0x14025fbd0 <.text+0x25ebd0>
140335cf8:     	leaq	0x658(%rbp), %rcx
140335cff:     	callq	0x140531f10 <.text+0x530f10>
140335d04:     	leaq	0x5e8(%rbp), %rcx
140335d0b:     	callq	0x140295030 <.text+0x294030>
140335d10:     	leaq	0x578(%rbp), %rcx
140335d17:     	callq	0x1402dfe10 <.text+0x2dee10>
140335d1c:     	leaq	0x508(%rbp), %rcx
140335d23:     	callq	0x140252490 <.text+0x251490>
140335d28:     	leaq	0x498(%rbp), %rcx
140335d2f:     	callq	0x1405c6fb0 <.text+0x5c5fb0>
140335d34:     	leaq	0x428(%rbp), %rcx
140335d3b:     	callq	0x140369050 <.text+0x368050>
140335d40:     	leaq	0x3b8(%rbp), %rcx
140335d47:     	callq	0x140368e90 <.text+0x367e90>
140335d4c:     	leaq	0x348(%rbp), %rcx
140335d53:     	callq	0x1404e6e90 <.text+0x4e5e90>
140335d58:     	leaq	0x968(%rbp), %rcx
140335d5f:     	callq	0x140174db0 <.text+0x173db0>
140335d64:     	leaq	-0x60(%rbp), %rcx
140335d68:     	callq	0x140538010 <.text+0x537010>
140335d6d:     	movups	0x958(%rbp), %xmm0
140335d74:     	movq	0x9d8(%rbp), %rax
140335d7b:     	movups	%xmm0, 0x60(%rax)
140335d7f:     	movups	0x948(%rbp), %xmm0
140335d86:     	movups	%xmm0, 0x50(%rax)
140335d8a:     	movups	0x938(%rbp), %xmm0
140335d91:     	movups	%xmm0, 0x40(%rax)
140335d95:     	movups	0x8f8(%rbp), %xmm0
140335d9c:     	movups	0x908(%rbp), %xmm1
140335da3:     	movups	0x918(%rbp), %xmm2
140335daa:     	movups	0x928(%rbp), %xmm3
140335db1:     	movups	%xmm3, 0x30(%rax)
140335db5:     	movups	%xmm2, 0x20(%rax)
140335db9:     	movups	%xmm1, 0x10(%rax)
140335dbd:     	movups	%xmm0, (%rax)
140335dc0:     	movups	0x888(%rbp), %xmm0
140335dc7:     	movups	0x898(%rbp), %xmm1
140335dce:     	movups	0x8a8(%rbp), %xmm2
140335dd5:     	movups	0x8b8(%rbp), %xmm3
140335ddc:     	movups	%xmm0, 0x70(%rax)
140335de0:     	movups	%xmm1, 0x80(%rax)
140335de7:     	movups	%xmm2, 0x90(%rax)
140335dee:     	movups	%xmm3, 0xa0(%rax)
140335df5:     	movups	0x8c8(%rbp), %xmm0
140335dfc:     	movups	%xmm0, 0xb0(%rax)
140335e03:     	movups	0x8d8(%rbp), %xmm0
140335e0a:     	movups	%xmm0, 0xc0(%rax)
140335e11:     	movups	0x8e8(%rbp), %xmm0
140335e18:     	movups	%xmm0, 0xd0(%rax)
140335e1f:     	movups	0x878(%rbp), %xmm0
