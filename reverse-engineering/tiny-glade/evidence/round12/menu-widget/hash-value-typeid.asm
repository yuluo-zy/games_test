
D:\game\reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1405bdf20:     	pushq	%rsi
1405bdf21:     	pushq	%rdi
1405bdf22:     	subq	$0x78, %rsp
1405bdf26:     	movabsq	$0x736f6d6570736575, %rax # imm = 0x736F6D6570736575
1405bdf30:     	movq	%rax, 0x30(%rsp)
1405bdf35:     	movabsq	$0x6c7967656e657261, %rax # imm = 0x6C7967656E657261
1405bdf3f:     	movq	%rax, 0x38(%rsp)
1405bdf44:     	movabsq	$0x646f72616e646f6d, %rax # imm = 0x646F72616E646F6D
1405bdf4e:     	movq	%rax, 0x40(%rsp)
1405bdf53:     	movabsq	$0x7465646279746573, %rax # imm = 0x7465646279746573
1405bdf5d:     	movq	%rax, 0x48(%rsp)
1405bdf62:     	xorps	%xmm0, %xmm0
1405bdf65:     	movups	%xmm0, 0x50(%rsp)
1405bdf6a:     	movups	%xmm0, 0x60(%rsp)
1405bdf6f:     	movq	$0x0, 0x70(%rsp)
1405bdf78:     	movq	(%rcx), %rax
1405bdf7b:     	movq	0x10(%rcx), %rdi
1405bdf7f:     	movq	%rax, 0x28(%rsp)
1405bdf84:     	leaq	0x30(%rsp), %rsi
1405bdf89:     	leaq	0x28(%rsp), %rdx
1405bdf8e:     	movl	$0x8, %r8d
1405bdf94:     	movq	%rsi, %rcx
1405bdf97:     	callq	0x140106160 <.text+0x105160>
1405bdf9c:     	movq	%rdi, 0x28(%rsp)
1405bdfa1:     	leaq	0x28(%rsp), %rdx
1405bdfa6:     	movl	$0x8, %r8d
1405bdfac:     	movq	%rsi, %rcx
1405bdfaf:     	callq	0x140106160 <.text+0x105160>
1405bdfb4:     	movq	0x60(%rsp), %r9
1405bdfb9:     	shlq	$0x38, %r9
1405bdfbd:     	orq	0x68(%rsp), %r9
1405bdfc2:     	movq	0x40(%rsp), %rdx
1405bdfc7:     	movq	0x48(%rsp), %rcx
1405bdfcc:     	xorq	%r9, %rcx
1405bdfcf:     	movq	0x30(%rsp), %rax
1405bdfd4:     	addq	%rdx, %rax
1405bdfd7:     	movq	0x38(%rsp), %r8
1405bdfdc:     	rolq	$0xd, %rdx
1405bdfe0:     	addq	%rcx, %r8
1405bdfe3:     	xorq	%rax, %rdx
1405bdfe6:     	rolq	$0x10, %rcx
1405bdfea:     	xorq	%r8, %rcx
1405bdfed:     	rolq	$0x20, %rax
1405bdff1:     	addq	%rdx, %r8
1405bdff4:     	addq	%rcx, %rax
1405bdff7:     	rolq	$0x11, %rdx
1405bdffb:     	xorq	%r8, %rdx
1405bdffe:     	rolq	$0x15, %rcx
1405be002:     	rolq	$0x20, %r8
1405be006:     	xorq	%rax, %rcx
1405be009:     	xorq	%r9, %rax
1405be00c:     	xorq	$0xff, %r8
1405be013:     	addq	%rdx, %rax
1405be016:     	addq	%rcx, %r8
1405be019:     	rolq	$0xd, %rdx
1405be01d:     	xorq	%rax, %rdx
1405be020:     	rolq	$0x10, %rcx
1405be024:     	rolq	$0x20, %rax
1405be028:     	xorq	%r8, %rcx
1405be02b:     	addq	%rdx, %r8
1405be02e:     	rolq	$0x11, %rdx
1405be032:     	addq	%rcx, %rax
1405be035:     	xorq	%r8, %rdx
1405be038:     	rolq	$0x15, %rcx
1405be03c:     	xorq	%rax, %rcx
1405be03f:     	rolq	$0x20, %r8
1405be043:     	addq	%rdx, %rax
1405be046:     	addq	%rcx, %r8
1405be049:     	rolq	$0xd, %rdx
1405be04d:     	xorq	%rax, %rdx
1405be050:     	rolq	$0x10, %rcx
1405be054:     	rolq	$0x20, %rax
1405be058:     	xorq	%r8, %rcx
1405be05b:     	addq	%rdx, %r8
1405be05e:     	rolq	$0x11, %rdx
1405be062:     	addq	%rcx, %rax
1405be065:     	xorq	%r8, %rdx
1405be068:     	rolq	$0x15, %rcx
1405be06c:     	xorq	%rax, %rcx
1405be06f:     	rolq	$0x20, %r8
1405be073:     	addq	%rdx, %rax
1405be076:     	addq	%rcx, %r8
1405be079:     	rolq	$0xd, %rdx
1405be07d:     	xorq	%rax, %rdx
1405be080:     	rolq	$0x10, %rcx
1405be084:     	xorq	%r8, %rcx
1405be087:     	addq	%rdx, %r8
1405be08a:     	rolq	$0x11, %rdx
1405be08e:     	rolq	$0x15, %rcx
1405be092:     	movq	%r8, %rax
1405be095:     	rolq	$0x20, %rax
1405be099:     	xorq	%rdx, %rax
1405be09c:     	xorq	%rcx, %rax
1405be09f:     	xorq	%r8, %rax
1405be0a2:     	addq	$0x78, %rsp
1405be0a6:     	popq	%rdi
1405be0a7:     	popq	%rsi
1405be0a8:     	retq
