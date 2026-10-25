
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1403cf9b0:     	pushq	%r15
1403cf9b2:     	pushq	%r14
1403cf9b4:     	pushq	%r13
1403cf9b6:     	pushq	%r12
1403cf9b8:     	pushq	%rsi
1403cf9b9:     	pushq	%rdi
1403cf9ba:     	pushq	%rbp
1403cf9bb:     	pushq	%rbx
1403cf9bc:     	subq	$0x148, %rsp            # imm = 0x148
1403cf9c3:     	movq	%r9, %rsi
1403cf9c6:     	movq	%rcx, %r14
1403cf9c9:     	movq	0x1f0(%rsp), %r9
1403cf9d1:     	movq	0x1b0(%rsp), %rax
1403cf9d9:     	movq	(%rdx), %rcx
1403cf9dc:     	movq	%rcx, 0xe8(%rsp)
1403cf9e4:     	movq	0x20(%rdx), %rbp
1403cf9e8:     	movq	0x40(%rdx), %rbx
1403cf9ec:     	movq	0x60(%rdx), %r12
1403cf9f0:     	movups	(%r8), %xmm0
1403cf9f4:     	movups	0x10(%r8), %xmm1
1403cf9f9:     	movaps	%xmm1, 0x130(%rsp)
1403cfa01:     	movaps	%xmm0, 0x120(%rsp)
1403cfa09:     	movq	0x20(%r8), %rcx
1403cfa0d:     	movq	%rcx, 0xd0(%rsp)
1403cfa15:     	movq	0x20(%r9), %rcx
1403cfa19:     	movq	%rcx, 0xe0(%rsp)
1403cfa21:     	movq	0x30(%r9), %rcx
1403cfa25:     	movq	%rcx, 0xd8(%rsp)
1403cfa2d:     	movl	0x3c(%r9), %ecx
1403cfa31:     	movl	%ecx, 0xc8(%rsp)
1403cfa38:     	movb	$0x0, 0x32(%rsp)
1403cfa3d:     	movb	$0x0, 0x33(%rsp)
1403cfa42:     	movl	0x1c(%rax), %ecx
1403cfa45:     	movq	0x10(%rax), %rdx
1403cfa49:     	movl	%ecx, 0x34(%rsp)
1403cfa4d:     	movq	%rdx, 0x58(%rsp)
1403cfa52:     	movl	%ecx, (%rdx)
1403cfa54:     	movq	(%rax), %rdi
1403cfa57:     	leaq	0x26020eb(%rip), %rax   # 0x1429d1b49
1403cfa5e:     	movq	%rax, 0x68(%rsp)
1403cfa63:     	movq	$0xa, 0x70(%rsp)
1403cfa6c:     	leaq	0x68(%rsp), %rcx
1403cfa71:     	callq	0x140683c10 <.text+0x682c10>
1403cfa76:     	movq	%rax, %r15
1403cfa79:     	movabsq	$-0x4e96acdfffb04bbd, %rax # imm = 0xB1695320004FB443
1403cfa83:     	xorq	%rax, %r15
1403cfa86:     	movq	%rdi, %rcx
1403cfa89:     	movq	%r15, %rdx
1403cfa8c:     	callq	0x140683ec0 <.text+0x682ec0>
1403cfa91:     	movb	$0x0, 0x5d(%rax)
1403cfa95:     	testb	$0x1, %dl
1403cfa98:     	jne	0x1403cfacd <.text+0x3ceacd>
1403cfa9a:     	movq	0x10(%rdi), %r13
1403cfa9e:     	cmpq	(%rdi), %r13
1403cfaa1:     	jne	0x1403cfab2 <.text+0x3ceab2>
1403cfaa3:     	leaq	0x25ef796(%rip), %rdx   # 0x1429bf240
1403cfaaa:     	movq	%rdi, %rcx
1403cfaad:     	callq	0x1406319d0 <.text+0x6309d0>
1403cfab2:     	movq	0x8(%rdi), %rax
1403cfab6:     	imulq	$0x38, %r13, %rcx
1403cfaba:     	movl	$0x2, (%rax,%rcx)
1403cfac1:     	movq	%r15, 0x30(%rax,%rcx)
1403cfac6:     	incq	%r13
1403cfac9:     	movq	%r13, 0x10(%rdi)
1403cfacd:     	movq	%rdi, 0x38(%rsp)
1403cfad2:     	movq	%r15, 0x40(%rsp)
1403cfad7:     	movq	$0x0, 0x48(%rsp)
1403cfae0:     	leaq	0x260206c(%rip), %r8    # 0x1429d1b53
1403cfae7:     	leaq	0x68(%rsp), %rcx
1403cfaec:     	leaq	0x38(%rsp), %rdx
1403cfaf1:     	movl	$0x9, %r9d
1403cfaf7:     	callq	0x14068bee0 <.text+0x68aee0>
1403cfafc:     	leaq	0x2602059(%rip), %rax   # 0x1429d1b5c
1403cfb03:     	movq	%rax, 0xf0(%rsp)
1403cfb0b:     	movq	$0x6, 0xf8(%rsp)
1403cfb17:     	movq	$0x1, 0x100(%rsp)
1403cfb23:     	movq	$0x0, 0x108(%rsp)
1403cfb2f:     	movq	$0x1, 0x110(%rsp)
1403cfb3b:     	movq	$0x0, 0x118(%rsp)
1403cfb47:     	movl	$0x41a00000, 0xcc(%rsp) # imm = 0x41A00000
1403cfb52:     	cmpb	$0x0, 0x1(%r12)
1403cfb58:     	jne	0x1403cfbaa <.text+0x3cebaa>
1403cfb5a:     	leaq	0xf0(%rsp), %rax
1403cfb62:     	movq	%rax, 0x38(%rsp)
1403cfb67:     	leaq	0x100(%rsp), %rax
1403cfb6f:     	movq	%rax, 0x40(%rsp)
1403cfb74:     	leaq	0xcc(%rsp), %rax
1403cfb7c:     	movq	%rax, 0x48(%rsp)
1403cfb81:     	leaq	0x110(%rsp), %rax
1403cfb89:     	movq	%rax, 0x50(%rsp)
1403cfb8e:     	leaq	0x2601fcd(%rip), %rdx   # 0x1429d1b62
1403cfb95:     	leaq	0x68(%rsp), %rcx
1403cfb9a:     	leaq	0x38(%rsp), %r9
1403cfb9f:     	movl	$0x8, %r8d
1403cfba5:     	callq	0x140503240 <.text+0x502240>
1403cfbaa:     	movq	0x1b8(%rsp), %r15
1403cfbb2:     	cmpb	$0x1, (%rbp)
1403cfbb6:     	jne	0x1403cfc56 <.text+0x3cec56>
1403cfbbc:     	movq	(%r14), %r14
1403cfbbf:     	movl	0xd8(%r14), %ecx
1403cfbc6:     	movss	0xdc(%r14), %xmm1
1403cfbcf:     	callq	0x14093fa90 <.text+0x93ea90>
1403cfbd4:     	testb	%al, %al
1403cfbd6:     	je	0x1403cfc78 <.text+0x3cec78>
1403cfbdc:     	movq	0x1f8(%rsp), %rax
1403cfbe4:     	movq	(%rax), %rcx
1403cfbe7:     	callq	0x14092e6b0 <.text+0x92d6b0>
1403cfbec:     	testb	%al, %al
1403cfbee:     	je	0x1403cfc5d <.text+0x3cec5d>
1403cfbf0:     	movq	%rsi, 0x60(%rsp)
1403cfbf5:     	callq	0x140af5b20 <.text+0xaf4b20>
1403cfbfa:     	movl	%eax, %ebp
1403cfbfc:     	movl	0x1c(%r15), %eax
1403cfc00:     	movq	0x10(%r15), %rcx
1403cfc04:     	movl	%eax, (%rcx)
1403cfc06:     	movq	(%r15), %r15
1403cfc09:     	movq	0x30(%r15), %r13
1403cfc0d:     	movq	0x40(%r15), %rsi
1403cfc11:     	cmpq	0x20(%r15), %r13
1403cfc15:     	jne	0x1403cfc27 <.text+0x3cec27>
1403cfc17:     	leaq	0x20(%r15), %rcx
1403cfc1b:     	leaq	0x258d8ae(%rip), %rdx   # 0x14295d4d0
1403cfc22:     	callq	0x140562630 <.text+0x561630>
1403cfc27:     	movq	0x28(%r15), %rax
1403cfc2b:     	movq	%r13, %rcx
1403cfc2e:     	shlq	$0x4, %rcx
1403cfc32:     	movq	%rsi, (%rax,%rcx)
1403cfc36:     	movl	%ebp, 0x8(%rax,%rcx)
1403cfc3a:     	incq	%r13
1403cfc3d:     	movq	%r13, 0x30(%r15)
1403cfc41:     	incq	0x40(%r15)
1403cfc45:     	xorl	%ebp, %ebp
1403cfc47:     	movq	0x60(%rsp), %rsi
1403cfc4c:     	movq	0x1b8(%rsp), %r15
1403cfc54:     	jmp	0x1403cfc7a <.text+0x3cec7a>
1403cfc56:     	xorl	%ebp, %ebp
1403cfc58:     	jmp	0x1403cfd11 <.text+0x3ced11>
1403cfc5d:     	movq	%rbx, %rcx
1403cfc60:     	callq	0x14093fbe0 <.text+0x93ebe0>
1403cfc65:     	testb	%al, %al
1403cfc67:     	je	0x1403cfc73 <.text+0x3cec73>
1403cfc69:     	movb	$0x1, 0x32(%rsp)
1403cfc6e:     	movb	$0x1, %bpl
1403cfc71:     	jmp	0x1403cfc7a <.text+0x3cec7a>
1403cfc73:     	movb	$0x1, 0x33(%rsp)
1403cfc78:     	xorl	%ebp, %ebp
1403cfc7a:     	movl	0xf8(%r14), %ecx
1403cfc81:     	movss	0xfc(%r14), %xmm1
1403cfc8a:     	callq	0x14093fa90 <.text+0x93ea90>
1403cfc8f:     	testb	%al, %al
1403cfc91:     	je	0x1403cfd11 <.text+0x3ced11>
1403cfc93:     	cmpl	$0x8, (%rbx)
1403cfc96:     	je	0x1403cfd11 <.text+0x3ced11>
1403cfc98:     	movq	%rsi, %r13
1403cfc9b:     	movq	0x1c8(%rsp), %rdx
1403cfca3:     	movl	0x1c(%rdx), %eax
1403cfca6:     	movq	0x10(%rdx), %rcx
1403cfcaa:     	movl	%eax, (%rcx)
1403cfcac:     	movq	(%rdx), %r14
1403cfcaf:     	movq	0x30(%r14), %r15
1403cfcb3:     	movq	0x40(%r14), %rsi
1403cfcb7:     	cmpq	0x20(%r14), %r15
1403cfcbb:     	jne	0x1403cfccd <.text+0x3ceccd>
1403cfcbd:     	leaq	0x20(%r14), %rcx
1403cfcc1:     	leaq	0x2623460(%rip), %rdx   # 0x1429f3128
1403cfcc8:     	callq	0x140562870 <.text+0x561870>
1403cfccd:     	movq	0x28(%r14), %rax
1403cfcd1:     	movq	%rsi, (%rax,%r15,8)
1403cfcd5:     	incq	%r15
1403cfcd8:     	movq	%r15, 0x30(%r14)
1403cfcdc:     	incq	0x40(%r14)
1403cfce0:     	movq	%r13, %rsi
1403cfce3:     	movl	0x1c(%r13), %eax
1403cfce7:     	movq	0x10(%r13), %rcx
1403cfceb:     	movl	%eax, (%rcx)
1403cfced:     	movq	(%r13), %rcx
1403cfcf1:     	movb	$0x1, 0x70(%rsp)
1403cfcf6:     	movq	$0x8, 0x68(%rsp)
1403cfcff:     	leaq	0x68(%rsp), %rdx
1403cfd04:     	callq	0x14093fad0 <.text+0x93ead0>
1403cfd09:     	movq	0x1b8(%rsp), %r15
1403cfd11:     	movq	%rbx, %rcx
1403cfd14:     	callq	0x14093fbe0 <.text+0x93ebe0>
1403cfd19:     	testb	%al, %al
1403cfd1b:     	je	0x1403cfd29 <.text+0x3ced29>
1403cfd1d:     	cmpb	$0x0, 0x1(%r12)
1403cfd23:     	je	0x1403cfe3a <.text+0x3cee3a>
1403cfd29:     	cmpl	$0x3, (%rbx)
1403cfd2c:     	je	0x1403cfd50 <.text+0x3ced50>
1403cfd2e:     	movq	0x1d0(%rsp), %r13
1403cfd36:     	testb	$0x1, %bpl
1403cfd3a:     	jne	0x1403cff4d <.text+0x3cef4d>
1403cfd40:     	cmpb	$0x0, 0x33(%rsp)
1403cfd45:     	jne	0x1403d01b1 <.text+0x3cf1b1>
1403cfd4b:     	jmp	0x1403d0222 <.text+0x3cf222>
1403cfd50:     	movl	0x34(%rsp), %eax
1403cfd54:     	movq	0x58(%rsp), %rcx
1403cfd59:     	movl	%eax, (%rcx)
1403cfd5b:     	leaq	0x2601e08(%rip), %rax   # 0x1429d1b6a
1403cfd62:     	movq	%rax, 0x68(%rsp)
1403cfd67:     	movq	$0x4, 0x70(%rsp)
1403cfd70:     	leaq	0x68(%rsp), %rcx
1403cfd75:     	callq	0x140683c10 <.text+0x682c10>
1403cfd7a:     	movq	%rax, %r14
1403cfd7d:     	movabsq	$-0x4e96acdfffb04bbd, %rax # imm = 0xB1695320004FB443
1403cfd87:     	xorq	%rax, %r14
1403cfd8a:     	movq	%rdi, %rcx
1403cfd8d:     	movq	%r14, %rdx
1403cfd90:     	callq	0x140683ec0 <.text+0x682ec0>
1403cfd95:     	movb	$0x0, 0x5d(%rax)
1403cfd99:     	testb	$0x1, %dl
1403cfd9c:     	jne	0x1403cfdd7 <.text+0x3cedd7>
1403cfd9e:     	movq	%r15, %r12
1403cfda1:     	movq	0x10(%rdi), %r15
1403cfda5:     	cmpq	(%rdi), %r15
1403cfda8:     	jne	0x1403cfdb9 <.text+0x3cedb9>
1403cfdaa:     	leaq	0x25ef48f(%rip), %rdx   # 0x1429bf240
1403cfdb1:     	movq	%rdi, %rcx
1403cfdb4:     	callq	0x1406319d0 <.text+0x6309d0>
1403cfdb9:     	movq	0x8(%rdi), %rax
1403cfdbd:     	imulq	$0x38, %r15, %rcx
1403cfdc1:     	movl	$0x2, (%rax,%rcx)
1403cfdc8:     	movq	%r14, 0x30(%rax,%rcx)
1403cfdcd:     	incq	%r15
1403cfdd0:     	movq	%r15, 0x10(%rdi)
1403cfdd4:     	movq	%r12, %r15
1403cfdd7:     	movq	%rdi, 0x38(%rsp)
1403cfddc:     	movq	%r14, 0x40(%rsp)
1403cfde1:     	movq	$0x0, 0x48(%rsp)
1403cfdea:     	leaq	0x2601d62(%rip), %r8    # 0x1429d1b53
1403cfdf1:     	leaq	0x68(%rsp), %r14
1403cfdf6:     	leaq	0x38(%rsp), %rdx
1403cfdfb:     	movl	$0x9, %r9d
1403cfe01:     	movq	%r14, %rcx
1403cfe04:     	callq	0x14068bee0 <.text+0x68aee0>
1403cfe09:     	leaq	0x2601d7b(%rip), %rdx   # 0x1429d1b8b
1403cfe10:     	leaq	0x33(%rsp), %r9
1403cfe15:     	movl	$0x6, %r8d
1403cfe1b:     	movq	%r14, %rcx
1403cfe1e:     	callq	0x140503eb0 <.text+0x502eb0>
1403cfe23:     	movq	0x1d0(%rsp), %r13
1403cfe2b:     	testb	$0x1, %bpl
1403cfe2f:     	je	0x1403cfd40 <.text+0x3ced40>
1403cfe35:     	jmp	0x1403cff4d <.text+0x3cef4d>
1403cfe3a:     	movl	0x34(%rsp), %eax
1403cfe3e:     	movq	0x58(%rsp), %rcx
1403cfe43:     	movl	%eax, (%rcx)
1403cfe45:     	leaq	0x2601d1e(%rip), %rax   # 0x1429d1b6a
1403cfe4c:     	movq	%rax, 0x68(%rsp)
1403cfe51:     	movq	$0x4, 0x70(%rsp)
1403cfe5a:     	leaq	0x68(%rsp), %rcx
1403cfe5f:     	callq	0x140683c10 <.text+0x682c10>
1403cfe64:     	movq	%rax, %r14
1403cfe67:     	movabsq	$-0x4e96acdfffb04bbd, %rax # imm = 0xB1695320004FB443
1403cfe71:     	xorq	%rax, %r14
1403cfe74:     	movq	%rdi, %rcx
1403cfe77:     	movq	%r14, %rdx
1403cfe7a:     	callq	0x140683ec0 <.text+0x682ec0>
1403cfe7f:     	movb	$0x0, 0x5d(%rax)
1403cfe83:     	testb	$0x1, %dl
1403cfe86:     	jne	0x1403cfec1 <.text+0x3ceec1>
1403cfe88:     	movq	%r15, %r12
1403cfe8b:     	movq	0x10(%rdi), %r15
1403cfe8f:     	cmpq	(%rdi), %r15
1403cfe92:     	jne	0x1403cfea3 <.text+0x3ceea3>
1403cfe94:     	leaq	0x25ef3a5(%rip), %rdx   # 0x1429bf240
1403cfe9b:     	movq	%rdi, %rcx
1403cfe9e:     	callq	0x1406319d0 <.text+0x6309d0>
1403cfea3:     	movq	0x8(%rdi), %rax
1403cfea7:     	imulq	$0x38, %r15, %rcx
1403cfeab:     	movl	$0x2, (%rax,%rcx)
1403cfeb2:     	movq	%r14, 0x30(%rax,%rcx)
1403cfeb7:     	incq	%r15
1403cfeba:     	movq	%r15, 0x10(%rdi)
1403cfebe:     	movq	%r12, %r15
1403cfec1:     	movq	%rdi, 0x38(%rsp)
1403cfec6:     	movq	%r14, 0x40(%rsp)
1403cfecb:     	movq	$0x0, 0x48(%rsp)
1403cfed4:     	leaq	0x2601c93(%rip), %r8    # 0x1429d1b6e
1403cfedb:     	leaq	0x68(%rsp), %r14
1403cfee0:     	leaq	0x38(%rsp), %rdx
1403cfee5:     	movl	$0x16, %r9d
1403cfeeb:     	movq	%r14, %rcx
1403cfeee:     	callq	0x14068bee0 <.text+0x68aee0>
1403cfef3:     	leaq	0x120(%rsp), %rax
1403cfefb:     	movq	%rax, 0x38(%rsp)
1403cff00:     	movq	0x1c8(%rsp), %rax
1403cff08:     	movq	%rax, 0x40(%rsp)
1403cff0d:     	movq	%rsi, 0x48(%rsp)
1403cff12:     	leaq	0x32(%rsp), %rax
1403cff17:     	movq	%rax, 0x50(%rsp)
1403cff1c:     	leaq	0x2601c61(%rip), %rdx   # 0x1429d1b84
1403cff23:     	leaq	0x38(%rsp), %r9
1403cff28:     	movl	$0x7, %r8d
1403cff2e:     	movq	%r14, %rcx
1403cff31:     	callq	0x1405005e0 <.text+0x4ff5e0>
1403cff36:     	movzbl	0x32(%rsp), %ebp
1403cff3b:     	movq	0x1d0(%rsp), %r13
1403cff43:     	testb	$0x1, %bpl
1403cff47:     	je	0x1403cfd40 <.text+0x3ced40>
1403cff4d:     	movq	%rbx, 0x60(%rsp)
1403cff52:     	movq	0x1c0(%rsp), %r12
1403cff5a:     	movl	0x1c(%rsi), %eax
1403cff5d:     	movq	0x10(%rsi), %rcx
1403cff61:     	movl	%eax, (%rcx)
1403cff63:     	movq	(%rsi), %rcx
1403cff66:     	movq	$0x3, 0x68(%rsp)
1403cff6f:     	leaq	0x68(%rsp), %rdx
1403cff74:     	callq	0x14093fad0 <.text+0x93ead0>
1403cff79:     	movq	0xe8(%rsp), %rax
1403cff81:     	movl	(%rax), %eax
1403cff83:     	cmpl	$0x13, %eax
1403cff86:     	je	0x1403d007f <.text+0x3cf07f>
1403cff8c:     	movq	%r13, %rbx
1403cff8f:     	cmpl	$0xd, %eax
1403cff92:     	jne	0x1403d0018 <.text+0x3cf018>
1403cff98:     	movq	%rsi, %r14
1403cff9b:     	movl	0x1c(%r15), %eax
1403cff9f:     	movq	0x10(%r15), %rcx
1403cffa3:     	movl	%eax, (%rcx)
1403cffa5:     	movq	(%r15), %r15
1403cffa8:     	movq	0x30(%r15), %r13
1403cffac:     	movq	0x40(%r15), %rsi
1403cffb0:     	cmpq	0x20(%r15), %r13
1403cffb4:     	jne	0x1403cffc6 <.text+0x3cefc6>
1403cffb6:     	leaq	0x20(%r15), %rcx
1403cffba:     	leaq	0x258d50f(%rip), %rdx   # 0x14295d4d0
1403cffc1:     	callq	0x140562630 <.text+0x561630>
1403cffc6:     	movq	0x28(%r15), %rax
1403cffca:     	movq	%r13, %rcx
1403cffcd:     	shlq	$0x4, %rcx
1403cffd1:     	movq	%rsi, (%rax,%rcx)
1403cffd5:     	movl	$0x3000300, 0x8(%rax,%rcx) # imm = 0x3000300
1403cffdd:     	incq	%r13
1403cffe0:     	movq	%r13, 0x30(%r15)
1403cffe4:     	incq	0x40(%r15)
1403cffe8:     	leaq	0x68(%rsp), %rcx
1403cffed:     	callq	0x140a86b30 <.text+0xa85b30>
1403cfff2:     	movq	0xd8(%rsp), %rax
1403cfffa:     	movl	0xc8(%rsp), %ecx
1403d0001:     	movl	%ecx, (%rax)
1403d0003:     	movups	0x68(%rsp), %xmm0
1403d0008:     	movq	0xe0(%rsp), %rax
1403d0010:     	movups	%xmm0, (%rax)
1403d0013:     	movq	%r14, %rsi
1403d0016:     	jmp	0x1403d007c <.text+0x3cf07c>
1403d0018:     	leaq	0x68(%rsp), %rcx
1403d001d:     	callq	0x140a86b30 <.text+0xa85b30>
1403d0022:     	movq	0x1f0(%rsp), %rdx
1403d002a:     	movl	0x1c(%rdx), %eax
1403d002d:     	movq	0x10(%rdx), %rcx
1403d0031:     	movl	%eax, (%rcx)
1403d0033:     	movq	(%rdx), %r15
1403d0036:     	movq	0x30(%r15), %r13
1403d003a:     	movq	0x40(%r15), %rbp
1403d003e:     	cmpq	0x20(%r15), %r13
1403d0042:     	jne	0x1403d0054 <.text+0x3cf054>
1403d0044:     	leaq	0x20(%r15), %rcx
1403d0048:     	leaq	0x258d481(%rip), %rdx   # 0x14295d4d0
1403d004f:     	callq	0x140562db0 <.text+0x561db0>
1403d0054:     	movq	0x28(%r15), %rax
1403d0058:     	leaq	(,%r13,2), %rcx
1403d0060:     	addq	%r13, %rcx
1403d0063:     	movups	0x68(%rsp), %xmm0
1403d0068:     	movups	%xmm0, (%rax,%rcx,8)
1403d006c:     	movq	%rbp, 0x10(%rax,%rcx,8)
1403d0071:     	incq	%r13
1403d0074:     	movq	%r13, 0x30(%r15)
1403d0078:     	incq	0x40(%r15)
1403d007c:     	movq	%rbx, %r13
1403d007f:     	movq	%rsi, %r14
1403d0082:     	movl	0x1c(%r12), %eax
1403d0087:     	movq	0x10(%r12), %rcx
1403d008c:     	movl	%eax, (%rcx)
1403d008e:     	movq	(%r12), %r15
1403d0092:     	movq	0x30(%r15), %r12
1403d0096:     	movq	0x40(%r15), %rsi
1403d009a:     	cmpq	0x20(%r15), %r12
1403d009e:     	jne	0x1403d00b0 <.text+0x3cf0b0>
1403d00a0:     	leaq	0x20(%r15), %rcx
1403d00a4:     	leaq	0x258d425(%rip), %rdx   # 0x14295d4d0
1403d00ab:     	callq	0x140562870 <.text+0x561870>
1403d00b0:     	movq	0x28(%r15), %rax
1403d00b4:     	movq	%rsi, (%rax,%r12,8)
1403d00b8:     	incq	%r12
1403d00bb:     	movq	%r12, 0x30(%r15)
1403d00bf:     	incq	0x40(%r15)
1403d00c3:     	movl	0x1c(%r13), %eax
1403d00c7:     	movq	0x10(%r13), %rcx
1403d00cb:     	movl	%eax, (%rcx)
1403d00cd:     	movq	%r13, %rbx
1403d00d0:     	movq	(%r13), %r13
1403d00d4:     	movq	0x30(%r13), %r15
1403d00d8:     	movq	0x40(%r13), %rbp
1403d00dc:     	cmpq	0x20(%r13), %r15
1403d00e0:     	jne	0x1403d00f2 <.text+0x3cf0f2>
1403d00e2:     	leaq	0x20(%r13), %rcx
1403d00e6:     	leaq	0x258d3e3(%rip), %rdx   # 0x14295d4d0
1403d00ed:     	callq	0x140562630 <.text+0x561630>
1403d00f2:     	movq	0x1e0(%rsp), %r12
1403d00fa:     	movq	0x28(%r13), %rax
1403d00fe:     	movq	%r15, %rcx
1403d0101:     	shlq	$0x4, %rcx
1403d0105:     	movq	%rbp, (%rax,%rcx)
1403d0109:     	movb	$0xa, 0x8(%rax,%rcx)
1403d010e:     	incq	%r15
1403d0111:     	movq	%r15, 0x30(%r13)
1403d0115:     	incq	0x40(%r13)
1403d0119:     	movq	0x1c8(%rsp), %rdx
1403d0121:     	movl	0x1c(%rdx), %eax
1403d0124:     	movq	0x10(%rdx), %rcx
1403d0128:     	movl	%eax, (%rcx)
1403d012a:     	movq	(%rdx), %r15
1403d012d:     	movq	0x30(%r15), %r13
1403d0131:     	movq	0x40(%r15), %rsi
1403d0135:     	cmpq	0x20(%r15), %r13
1403d0139:     	jne	0x1403d014b <.text+0x3cf14b>
1403d013b:     	leaq	0x20(%r15), %rcx
1403d013f:     	leaq	0x2622fe2(%rip), %rdx   # 0x1429f3128
1403d0146:     	callq	0x140562870 <.text+0x561870>
1403d014b:     	movq	0x28(%r15), %rax
1403d014f:     	movq	%rsi, (%rax,%r13,8)
1403d0153:     	incq	%r13
1403d0156:     	movq	%r13, 0x30(%r15)
1403d015a:     	incq	0x40(%r15)
1403d015e:     	movl	0x1c(%r12), %eax
1403d0163:     	movq	0x10(%r12), %rcx
1403d0168:     	movl	%eax, (%rcx)
1403d016a:     	movq	(%r12), %r15
1403d016e:     	movq	0x30(%r15), %r12
1403d0172:     	movq	0x40(%r15), %rsi
1403d0176:     	cmpq	0x20(%r15), %r12
1403d017a:     	jne	0x1403d018c <.text+0x3cf18c>
1403d017c:     	leaq	0x20(%r15), %rcx
1403d0180:     	leaq	0x258d349(%rip), %rdx   # 0x14295d4d0
1403d0187:     	callq	0x140562870 <.text+0x561870>
1403d018c:     	movq	0x28(%r15), %rax
1403d0190:     	movq	%rsi, (%rax,%r12,8)
1403d0194:     	incq	%r12
1403d0197:     	movq	%r12, 0x30(%r15)
1403d019b:     	incq	0x40(%r15)
1403d019f:     	movq	%r14, %rsi
1403d01a2:     	movq	%rbx, %r13
1403d01a5:     	movq	0x60(%rsp), %rbx
1403d01aa:     	cmpb	$0x0, 0x33(%rsp)
1403d01af:     	je	0x1403d0222 <.text+0x3cf222>
1403d01b1:     	movq	%rsi, %r14
1403d01b4:     	movl	0x1c(%r13), %eax
1403d01b8:     	movq	0x10(%r13), %rcx
1403d01bc:     	movl	%eax, (%rcx)
1403d01be:     	movq	(%r13), %r15
1403d01c2:     	movq	0x30(%r15), %r12
1403d01c6:     	movq	0x40(%r15), %rsi
1403d01ca:     	cmpq	0x20(%r15), %r12
1403d01ce:     	jne	0x1403d01e0 <.text+0x3cf1e0>
1403d01d0:     	leaq	0x20(%r15), %rcx
1403d01d4:     	leaq	0x258d2f5(%rip), %rdx   # 0x14295d4d0
1403d01db:     	callq	0x140562630 <.text+0x561630>
1403d01e0:     	movq	0x28(%r15), %rax
1403d01e4:     	movq	%r12, %rcx
1403d01e7:     	shlq	$0x4, %rcx
1403d01eb:     	movq	%rsi, (%rax,%rcx)
1403d01ef:     	movb	$0xb, 0x8(%rax,%rcx)
1403d01f4:     	incq	%r12
1403d01f7:     	movq	%r12, 0x30(%r15)
1403d01fb:     	incq	0x40(%r15)
1403d01ff:     	movq	%r14, %rsi
1403d0202:     	movl	0x1c(%r14), %eax
1403d0206:     	movq	0x10(%r14), %rcx
1403d020a:     	movl	%eax, (%rcx)
1403d020c:     	movq	(%r14), %rcx
1403d020f:     	movq	$0x2, 0x68(%rsp)
1403d0218:     	leaq	0x68(%rsp), %rdx
1403d021d:     	callq	0x14093fad0 <.text+0x93ead0>
1403d0222:     	movq	0x200(%rsp), %r12
1403d022a:     	movq	0x1d8(%rsp), %r15
1403d0232:     	movq	(%rbx), %rax
1403d0235:     	cmpq	$0x3, %rax
1403d0239:     	je	0x1403d03ac <.text+0x3cf3ac>
1403d023f:     	cmpl	$0x4, %eax
1403d0242:     	jne	0x1403d07cd <.text+0x3cf7cd>
1403d0248:     	movq	0x1e8(%rsp), %r14
1403d0250:     	movq	0x325be41(%rip), %rax   # 0x14362c098
1403d0257:     	cmpb	$0x0, (%rax)
1403d025a:     	je	0x1403d04d6 <.text+0x3cf4d6>
1403d0260:     	movl	0x34(%rsp), %eax
1403d0264:     	movq	0x58(%rsp), %rcx
1403d0269:     	movl	%eax, (%rcx)
1403d026b:     	leaq	0x260195e(%rip), %rax   # 0x1429d1bd0
1403d0272:     	movq	%rax, 0x68(%rsp)
1403d0277:     	movq	$0xa, 0x70(%rsp)
1403d0280:     	leaq	0x68(%rsp), %rcx
1403d0285:     	callq	0x140683c10 <.text+0x682c10>
1403d028a:     	movq	%rax, %rbx
1403d028d:     	movabsq	$-0x4e96acdfffb04bbd, %rax # imm = 0xB1695320004FB443
1403d0297:     	xorq	%rax, %rbx
1403d029a:     	movq	%rdi, %rcx
1403d029d:     	movq	%rbx, %rdx
1403d02a0:     	callq	0x140683ec0 <.text+0x682ec0>
1403d02a5:     	movb	$0x0, 0x5d(%rax)
1403d02a9:     	movb	$0x1, 0xb7(%rax)
1403d02b0:     	testb	$0x1, %dl
1403d02b3:     	jne	0x1403d02e8 <.text+0x3cf2e8>
1403d02b5:     	movq	0x10(%rdi), %r13
1403d02b9:     	cmpq	(%rdi), %r13
1403d02bc:     	jne	0x1403d02cd <.text+0x3cf2cd>
1403d02be:     	leaq	0x25eef7b(%rip), %rdx   # 0x1429bf240
1403d02c5:     	movq	%rdi, %rcx
1403d02c8:     	callq	0x1406319d0 <.text+0x6309d0>
1403d02cd:     	movq	0x8(%rdi), %rax
1403d02d1:     	imulq	$0x38, %r13, %rcx
1403d02d5:     	movl	$0x1, (%rax,%rcx)
1403d02dc:     	movq	%rbx, 0x30(%rax,%rcx)
1403d02e1:     	incq	%r13
1403d02e4:     	movq	%r13, 0x10(%rdi)
1403d02e8:     	movq	%rdi, 0x38(%rsp)
1403d02ed:     	movq	%rbx, 0x40(%rsp)
1403d02f2:     	movq	$0x0, 0x48(%rsp)
1403d02fb:     	leaq	0x26018fc(%rip), %r8    # 0x1429d1bfe
1403d0302:     	leaq	0x68(%rsp), %rdi
1403d0307:     	leaq	0x38(%rsp), %rdx
1403d030c:     	movl	$0xe, %r9d
1403d0312:     	movq	%rdi, %rcx
1403d0315:     	callq	0x14068bee0 <.text+0x68aee0>
1403d031a:     	leaq	0x26018c1(%rip), %rdx   # 0x1429d1be2
1403d0321:     	movl	$0x4, %r8d
1403d0327:     	movq	%rdi, %rcx
1403d032a:     	callq	0x1404ffeb0 <.text+0x4feeb0>
1403d032f:     	movq	0xd0(%rsp), %rax
1403d0337:     	movss	0x1f0(%rax), %xmm0
1403d033f:     	ucomiss	0x2601582(%rip), %xmm0  # 0x1429d18c8
1403d0346:     	ja	0x1403d0353 <.text+0x3cf353>
1403d0348:     	movq	(%r12), %rax
1403d034c:     	cmpq	$0x2, 0x10(%rax)
1403d0351:     	jbe	0x1403d036d <.text+0x3cf36d>
1403d0353:     	leaq	0x260185a(%rip), %rdx   # 0x1429d1bb4
1403d035a:     	leaq	0x68(%rsp), %rcx
1403d035f:     	movl	$0x7, %r8d
1403d0365:     	movq	%r15, %r9
1403d0368:     	callq	0x140503950 <.text+0x502950>
1403d036d:     	leaq	0x2601847(%rip), %rdx   # 0x1429d1bbb
1403d0374:     	leaq	0x68(%rsp), %rdi
1403d0379:     	movl	$0x8, %r8d
1403d037f:     	movq	%rdi, %rcx
1403d0382:     	movq	%r15, %r9
1403d0385:     	callq	0x140500a20 <.text+0x4ffa20>
1403d038a:     	movq	%r14, 0x20(%rsp)
1403d038f:     	leaq	0x26017ee(%rip), %rdx   # 0x1429d1b84
1403d0396:     	movl	$0x7, %r8d
1403d039c:     	movq	%rdi, %rcx
1403d039f:     	movq	%rsi, %r9
1403d03a2:     	callq	0x1405028a0 <.text+0x5018a0>
1403d03a7:     	jmp	0x1403d07cd <.text+0x3cf7cd>
1403d03ac:     	movq	0x325bce5(%rip), %rax   # 0x14362c098
1403d03b3:     	cmpb	$0x0, (%rax)
1403d03b6:     	je	0x1403d05c7 <.text+0x3cf5c7>
1403d03bc:     	movl	0x34(%rsp), %eax
1403d03c0:     	movq	0x58(%rsp), %rcx
1403d03c5:     	movl	%eax, (%rcx)
1403d03c7:     	leaq	0x26017ed(%rip), %rax   # 0x1429d1bbb
1403d03ce:     	movq	%rax, 0x68(%rsp)
1403d03d3:     	movq	$0x8, 0x70(%rsp)
1403d03dc:     	leaq	0x68(%rsp), %rcx
1403d03e1:     	callq	0x140683c10 <.text+0x682c10>
1403d03e6:     	movq	%rax, %rbx
1403d03e9:     	movabsq	$-0x4e96acdfffb04bbd, %rax # imm = 0xB1695320004FB443
1403d03f3:     	xorq	%rax, %rbx
1403d03f6:     	movq	%rdi, %rcx
1403d03f9:     	movq	%rbx, %rdx
1403d03fc:     	callq	0x140683ec0 <.text+0x682ec0>
1403d0401:     	movb	$0x0, 0x5d(%rax)
1403d0405:     	movb	$0x1, 0xb7(%rax)
1403d040c:     	testb	$0x1, %dl
1403d040f:     	jne	0x1403d044a <.text+0x3cf44a>
1403d0411:     	movq	%r13, %r14
1403d0414:     	movq	0x10(%rdi), %r13
1403d0418:     	cmpq	(%rdi), %r13
1403d041b:     	jne	0x1403d042c <.text+0x3cf42c>
1403d041d:     	leaq	0x25eee1c(%rip), %rdx   # 0x1429bf240
1403d0424:     	movq	%rdi, %rcx
1403d0427:     	callq	0x1406319d0 <.text+0x6309d0>
1403d042c:     	movq	0x8(%rdi), %rax
1403d0430:     	imulq	$0x38, %r13, %rcx
1403d0434:     	movl	$0x2, (%rax,%rcx)
1403d043b:     	movq	%rbx, 0x30(%rax,%rcx)
1403d0440:     	incq	%r13
1403d0443:     	movq	%r13, 0x10(%rdi)
1403d0447:     	movq	%r14, %r13
1403d044a:     	movq	%rdi, 0x38(%rsp)
1403d044f:     	movq	%rbx, 0x40(%rsp)
1403d0454:     	movq	$0x0, 0x48(%rsp)
1403d045d:     	leaq	0x260175f(%rip), %r8    # 0x1429d1bc3
1403d0464:     	leaq	0x68(%rsp), %rcx
1403d0469:     	leaq	0x38(%rsp), %rdx
1403d046e:     	movl	$0xd, %r9d
1403d0474:     	callq	0x14068bee0 <.text+0x68aee0>
1403d0479:     	movq	0xd0(%rsp), %rax
1403d0481:     	movss	0x1f0(%rax), %xmm0
1403d0489:     	ucomiss	0x2601438(%rip), %xmm0  # 0x1429d18c8
1403d0490:     	ja	0x1403d049d <.text+0x3cf49d>
1403d0492:     	movq	(%r12), %rax
1403d0496:     	cmpq	$0x2, 0x10(%rax)
1403d049b:     	jbe	0x1403d04b7 <.text+0x3cf4b7>
1403d049d:     	leaq	0x2601710(%rip), %rdx   # 0x1429d1bb4
1403d04a4:     	leaq	0x68(%rsp), %rcx
1403d04a9:     	movl	$0x7, %r8d
1403d04af:     	movq	%r15, %r9
1403d04b2:     	callq	0x140503460 <.text+0x502460>
1403d04b7:     	leaq	0x26016fd(%rip), %rdx   # 0x1429d1bbb
1403d04be:     	leaq	0x68(%rsp), %rcx
1403d04c3:     	movl	$0x8, %r8d
1403d04c9:     	movq	%r15, %r9
1403d04cc:     	callq	0x140501640 <.text+0x500640>
1403d04d1:     	jmp	0x1403d06b1 <.text+0x3cf6b1>
1403d04d6:     	movl	0x34(%rsp), %eax
1403d04da:     	movq	0x58(%rsp), %rcx
1403d04df:     	movl	%eax, (%rcx)
1403d04e1:     	leaq	0x26016e8(%rip), %rax   # 0x1429d1bd0
1403d04e8:     	movq	%rax, 0x68(%rsp)
1403d04ed:     	movq	$0xa, 0x70(%rsp)
1403d04f6:     	leaq	0x68(%rsp), %rcx
1403d04fb:     	callq	0x140683c10 <.text+0x682c10>
1403d0500:     	movq	%rax, %rbx
1403d0503:     	movabsq	$-0x4e96acdfffb04bbd, %rax # imm = 0xB1695320004FB443
1403d050d:     	xorq	%rax, %rbx
1403d0510:     	movq	%rdi, %rcx
1403d0513:     	movq	%rbx, %rdx
1403d0516:     	callq	0x140683ec0 <.text+0x682ec0>
1403d051b:     	movb	$0x0, 0x5d(%rax)
1403d051f:     	movb	$0x1, 0xb7(%rax)
1403d0526:     	testb	$0x1, %dl
1403d0529:     	jne	0x1403d055e <.text+0x3cf55e>
1403d052b:     	movq	0x10(%rdi), %r15
1403d052f:     	cmpq	(%rdi), %r15
1403d0532:     	jne	0x1403d0543 <.text+0x3cf543>
1403d0534:     	leaq	0x25eed05(%rip), %rdx   # 0x1429bf240
1403d053b:     	movq	%rdi, %rcx
1403d053e:     	callq	0x1406319d0 <.text+0x6309d0>
1403d0543:     	movq	0x8(%rdi), %rax
1403d0547:     	imulq	$0x38, %r15, %rcx
1403d054b:     	movl	$0x1, (%rax,%rcx)
1403d0552:     	movq	%rbx, 0x30(%rax,%rcx)
1403d0557:     	incq	%r15
1403d055a:     	movq	%r15, 0x10(%rdi)
1403d055e:     	movq	%rdi, 0x38(%rsp)
1403d0563:     	movq	%rbx, 0x40(%rsp)
1403d0568:     	movq	$0x0, 0x48(%rsp)
1403d0571:     	leaq	0x260167d(%rip), %r8    # 0x1429d1bf5
1403d0578:     	leaq	0x68(%rsp), %rdi
1403d057d:     	leaq	0x38(%rsp), %rdx
1403d0582:     	movl	$0x9, %r9d
1403d0588:     	movq	%rdi, %rcx
1403d058b:     	callq	0x14068bee0 <.text+0x68aee0>
1403d0590:     	leaq	0x260164b(%rip), %rdx   # 0x1429d1be2
1403d0597:     	movl	$0x4, %r8d
1403d059d:     	movq	%rdi, %rcx
1403d05a0:     	callq	0x1404ff7e0 <.text+0x4fe7e0>
1403d05a5:     	movq	%r14, 0x20(%rsp)
1403d05aa:     	leaq	0x26015d3(%rip), %rdx   # 0x1429d1b84
1403d05b1:     	movl	$0x7, %r8d
1403d05b7:     	movq	%rdi, %rcx
1403d05ba:     	movq	%rsi, %r9
1403d05bd:     	callq	0x1405028a0 <.text+0x5018a0>
1403d05c2:     	jmp	0x1403d07cd <.text+0x3cf7cd>
1403d05c7:     	movl	0x34(%rsp), %eax
1403d05cb:     	movq	0x58(%rsp), %rcx
1403d05d0:     	movl	%eax, (%rcx)
1403d05d2:     	leaq	0x26015b8(%rip), %rax   # 0x1429d1b91
1403d05d9:     	movq	%rax, 0x68(%rsp)
1403d05de:     	movq	$0xd, 0x70(%rsp)
1403d05e7:     	leaq	0x68(%rsp), %rcx
1403d05ec:     	callq	0x140683c10 <.text+0x682c10>
1403d05f1:     	movq	%rax, %rbx
1403d05f4:     	movabsq	$-0x4e96acdfffb04bbd, %rax # imm = 0xB1695320004FB443
1403d05fe:     	xorq	%rax, %rbx
1403d0601:     	movq	%rdi, %rcx
1403d0604:     	movq	%rbx, %rdx
1403d0607:     	callq	0x140683ec0 <.text+0x682ec0>
1403d060c:     	movb	$0x0, 0x5d(%rax)
1403d0610:     	movb	$0x1, 0xb7(%rax)
1403d0617:     	testb	$0x1, %dl
1403d061a:     	jne	0x1403d064f <.text+0x3cf64f>
1403d061c:     	movq	0x10(%rdi), %r12
1403d0620:     	cmpq	(%rdi), %r12
1403d0623:     	jne	0x1403d0634 <.text+0x3cf634>
1403d0625:     	leaq	0x25eec14(%rip), %rdx   # 0x1429bf240
1403d062c:     	movq	%rdi, %rcx
1403d062f:     	callq	0x1406319d0 <.text+0x6309d0>
1403d0634:     	movq	0x8(%rdi), %rax
1403d0638:     	imulq	$0x38, %r12, %rcx
1403d063c:     	movl	$0x2, (%rax,%rcx)
1403d0643:     	movq	%rbx, 0x30(%rax,%rcx)
1403d0648:     	incq	%r12
1403d064b:     	movq	%r12, 0x10(%rdi)
1403d064f:     	movq	%rdi, 0x38(%rsp)
1403d0654:     	movq	%rbx, 0x40(%rsp)
1403d0659:     	movq	$0x0, 0x48(%rsp)
1403d0662:     	leaq	0x2601535(%rip), %r8    # 0x1429d1b9e
1403d0669:     	leaq	0x68(%rsp), %rbx
1403d066e:     	leaq	0x38(%rsp), %rdx
1403d0673:     	movl	$0xc, %r9d
1403d0679:     	movq	%rbx, %rcx
1403d067c:     	callq	0x14068bee0 <.text+0x68aee0>
1403d0681:     	leaq	0x2601522(%rip), %rdx   # 0x1429d1baa
1403d0688:     	movl	$0xa, %r8d
1403d068e:     	movq	%rbx, %rcx
1403d0691:     	movq	%r15, %r9
1403d0694:     	callq	0x140503c00 <.text+0x502c00>
1403d0699:     	leaq	0x2601514(%rip), %rdx   # 0x1429d1bb4
1403d06a0:     	movl	$0x7, %r8d
1403d06a6:     	movq	%rbx, %rcx
1403d06a9:     	movq	%r15, %r9
1403d06ac:     	callq	0x140501e40 <.text+0x500e40>
1403d06b1:     	movl	0x34(%rsp), %eax
1403d06b5:     	movq	0x58(%rsp), %rcx
1403d06ba:     	movl	%eax, (%rcx)
1403d06bc:     	leaq	0x260150d(%rip), %rax   # 0x1429d1bd0
1403d06c3:     	movq	%rax, 0x68(%rsp)
1403d06c8:     	movq	$0xa, 0x70(%rsp)
1403d06d1:     	leaq	0x68(%rsp), %rcx
1403d06d6:     	callq	0x140683c10 <.text+0x682c10>
1403d06db:     	movq	%rax, %rbx
1403d06de:     	movabsq	$-0x4e96acdfffb04bbd, %rax # imm = 0xB1695320004FB443
1403d06e8:     	xorq	%rax, %rbx
1403d06eb:     	movq	%rdi, %rcx
1403d06ee:     	movq	%rbx, %rdx
1403d06f1:     	callq	0x140683ec0 <.text+0x682ec0>
1403d06f6:     	movb	$0x0, 0x5d(%rax)
1403d06fa:     	movb	$0x1, 0xb7(%rax)
1403d0701:     	testb	$0x1, %dl
1403d0704:     	jne	0x1403d0739 <.text+0x3cf739>
1403d0706:     	movq	0x10(%rdi), %r15
1403d070a:     	cmpq	(%rdi), %r15
1403d070d:     	jne	0x1403d071e <.text+0x3cf71e>
1403d070f:     	leaq	0x25eeb2a(%rip), %rdx   # 0x1429bf240
1403d0716:     	movq	%rdi, %rcx
1403d0719:     	callq	0x1406319d0 <.text+0x6309d0>
1403d071e:     	movq	0x8(%rdi), %rax
1403d0722:     	imulq	$0x38, %r15, %rcx
1403d0726:     	movl	$0x1, (%rax,%rcx)
1403d072d:     	movq	%rbx, 0x30(%rax,%rcx)
1403d0732:     	incq	%r15
1403d0735:     	movq	%r15, 0x10(%rdi)
1403d0739:     	movq	%rdi, 0x38(%rsp)
1403d073e:     	movq	%rbx, 0x40(%rsp)
1403d0743:     	movq	$0x0, 0x48(%rsp)
1403d074c:     	leaq	0x2601487(%rip), %r8    # 0x1429d1bda
1403d0753:     	leaq	0x68(%rsp), %rdi
1403d0758:     	leaq	0x38(%rsp), %rdx
1403d075d:     	movl	$0x8, %r9d
1403d0763:     	movq	%rdi, %rcx
1403d0766:     	callq	0x14068bee0 <.text+0x68aee0>
1403d076b:     	leaq	0x2601470(%rip), %rdx   # 0x1429d1be2
1403d0772:     	movl	$0x4, %r8d
1403d0778:     	movq	%rdi, %rcx
1403d077b:     	callq	0x140500370 <.text+0x4ff370>
1403d0780:     	movq	%rsi, 0x20(%rsp)
1403d0785:     	leaq	0x260145a(%rip), %rdx   # 0x1429d1be6
1403d078c:     	movl	$0x8, %r8d
1403d0792:     	movq	%rdi, %rcx
1403d0795:     	movq	%r13, %r9
1403d0798:     	callq	0x1404fe790 <.text+0x4fd790>
1403d079d:     	leaq	0x26013e0(%rip), %rdx   # 0x1429d1b84
1403d07a4:     	movl	$0x7, %r8d
1403d07aa:     	movq	%rdi, %rcx
1403d07ad:     	movq	%rsi, %r9
1403d07b0:     	callq	0x140500e70 <.text+0x4ffe70>
1403d07b5:     	leaq	0x2601432(%rip), %rdx   # 0x1429d1bee
1403d07bc:     	movl	$0x7, %r8d
1403d07c2:     	movq	%rdi, %rcx
1403d07c5:     	movq	%rsi, %r9
1403d07c8:     	callq	0x140500c50 <.text+0x4ffc50>
1403d07cd:     	nop
1403d07ce:     	addq	$0x148, %rsp            # imm = 0x148
1403d07d5:     	popq	%rbx
1403d07d6:     	popq	%rbp
1403d07d7:     	popq	%rdi
1403d07d8:     	popq	%rsi
1403d07d9:     	popq	%r12
1403d07db:     	popq	%r13
1403d07dd:     	popq	%r14
1403d07df:     	popq	%r15
1403d07e1:     	retq
