
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1415539d0:     	pushq	%rbp
1415539d1:     	pushq	%r15
1415539d3:     	pushq	%r14
1415539d5:     	pushq	%r13
1415539d7:     	pushq	%r12
1415539d9:     	pushq	%rsi
1415539da:     	pushq	%rdi
1415539db:     	pushq	%rbx
1415539dc:     	subq	$0x2e8, %rsp            # imm = 0x2E8
1415539e3:     	leaq	0x80(%rsp), %rbp
1415539eb:     	movaps	%xmm15, 0x250(%rbp)
1415539f3:     	movaps	%xmm14, 0x240(%rbp)
1415539fb:     	movaps	%xmm13, 0x230(%rbp)
141553a03:     	movaps	%xmm12, 0x220(%rbp)
141553a0b:     	movaps	%xmm11, 0x210(%rbp)
141553a13:     	movaps	%xmm10, 0x200(%rbp)
141553a1b:     	movaps	%xmm9, 0x1f0(%rbp)
141553a23:     	movaps	%xmm8, 0x1e0(%rbp)
141553a2b:     	movaps	%xmm7, 0x1d0(%rbp)
141553a32:     	movaps	%xmm6, 0x1c0(%rbp)
141553a39:     	movq	$-0x2, 0x1b8(%rbp)
141553a44:     	movq	%r8, 0x8(%rbp)
141553a48:     	movq	%rdx, 0x10(%rbp)
141553a4c:     	movq	%rcx, 0x18(%rbp)
141553a50:     	movq	$0x0, 0x198(%rbp)
141553a5b:     	movq	$0x4, 0x1a0(%rbp)
141553a66:     	movq	$0x0, 0x1a8(%rbp)
141553a71:     	leaq	0x1718eae(%rip), %rax   # 0x142c6c926
141553a78:     	movq	%rax, 0xc8(%rbp)
141553a7f:     	movq	$0x10, 0xd0(%rbp)
141553a8a:     	movb	$0x0, 0xd8(%rbp)
141553a91:     	movsd	0x16cd3f7(%rip), %xmm0  # 0x142c20e90
141553a99:     	movsd	%xmm0, 0xdc(%rbp)
141553aa1:     	leaq	0x1718e8e(%rip), %rax   # 0x142c6c936
141553aa8:     	movq	%rax, 0xe8(%rbp)
141553aaf:     	movq	$0x1e, 0xf0(%rbp)
141553aba:     	movb	$0x0, 0xf8(%rbp)
141553ac1:     	movsd	%xmm0, 0xfc(%rbp)
141553ac9:     	leaq	0x1718e84(%rip), %rax   # 0x142c6c954
141553ad0:     	movq	%rax, 0x108(%rbp)
141553ad7:     	movq	$0x11, 0x110(%rbp)
141553ae2:     	movb	$0x2, 0x118(%rbp)
141553ae9:     	movsd	0x171861f(%rip), %xmm0  # 0x142c6c110
141553af1:     	movsd	%xmm0, 0x11c(%rbp)
141553af9:     	leaq	0x1718e65(%rip), %rax   # 0x142c6c965
141553b00:     	movq	%rax, 0x128(%rbp)
141553b07:     	movq	$0x24, 0x130(%rbp)
141553b12:     	movb	$0x2, 0x138(%rbp)
141553b19:     	movsd	%xmm0, 0x13c(%rbp)
141553b21:     	leaq	0x1718e61(%rip), %rax   # 0x142c6c989
141553b28:     	movq	%rax, 0x148(%rbp)
141553b2f:     	movq	$0xf, 0x150(%rbp)
141553b3a:     	movb	$0x1, 0x158(%rbp)
141553b41:     	movsd	0x17185d7(%rip), %xmm0  # 0x142c6c120
141553b49:     	movsd	%xmm0, 0x15c(%rbp)
141553b51:     	leaq	0x1718e40(%rip), %rax   # 0x142c6c998
141553b58:     	movq	%rax, 0x168(%rbp)
141553b5f:     	movq	$0x22, 0x170(%rbp)
141553b6a:     	movb	$0x1, 0x178(%rbp)
141553b71:     	movsd	%xmm0, 0x17c(%rbp)
141553b79:     	movq	$0x6, 0x190(%rbp)
141553b84:     	movzbl	0x58(%r9), %eax
141553b89:     	movq	%rax, (%rbp)
141553b8d:     	xorl	%eax, %eax
141553b8f:     	leaq	0x20(%rbp), %r14
141553b93:     	xorps	%xmm9, %xmm9
141553b97:     	movss	0x13d1d64(%rip), %xmm8  # 0x142925904
141553ba0:     	xorps	%xmm11, %xmm11
141553ba4:     	leaq	0x1718efd(%rip), %rdi   # 0x142c6caa8
141553bab:     	leaq	-0x38(%rbp), %rsi
141553baf:     	jmp	0x141553bc3 <.text+0x1552bc3>
141553bb1:     	nopw	%cs:(%rax,%rax)
141553bc0:     	movq	%rbx, %rax
141553bc3:     	cmpq	$0x6, %rax
141553bc7:     	je	0x141553eb8 <.text+0x1552eb8>
141553bcd:     	leaq	0x1(%rax), %rbx
141553bd1:     	movq	%rbx, 0x188(%rbp)
141553bd8:     	shlq	$0x5, %rax
141553bdc:     	movzbl	0xd8(%rbp,%rax), %r12d
141553be5:     	cmpb	$0x3, %r12b
141553be9:     	je	0x141553eb8 <.text+0x1552eb8>
141553bef:     	movq	0xc8(%rbp,%rax), %r15
141553bf7:     	movq	0xd0(%rbp,%rax), %r13
141553bff:     	movss	0xdc(%rbp,%rax), %xmm12
141553c09:     	movss	0xe0(%rbp,%rax), %xmm13
141553c13:     	movq	(%rbp), %rax
141553c17:     	leaq	0x1718faa(%rip), %rcx   # 0x142c6cbc8
141553c1e:     	movq	(%rcx,%rax,8), %r9
141553c22:     	leaq	0x1718fc7(%rip), %rcx   # 0x142c6cbf0
141553c29:     	movq	(%rcx,%rax,8), %rax
141553c2d:     	movq	%rax, 0x20(%rsp)
141553c32:     	movl	$0xc, %r8d
141553c38:     	movq	%r14, %rcx
141553c3b:     	leaq	0x159b8d8(%rip), %rdx   # 0x142aef51a
141553c42:     	callq	0x140d62810 <.text+0xd61810>
141553c47:     	movq	%r14, %rcx
141553c4a:     	callq	0x140d76ed0 <.text+0xd75ed0>
141553c4f:     	movq	%rdx, %r8
141553c52:     	movq	%r13, 0x20(%rsp)
141553c57:     	leaq	0xa0(%rbp), %rcx
141553c5e:     	movq	%rax, %rdx
141553c61:     	movq	%r15, %r9
141553c64:     	callq	0x140d62810 <.text+0xd61810>
141553c69:     	leaq	-0x20(%rbp), %rcx
141553c6d:     	movq	0x10(%rbp), %rdx
141553c71:     	movq	0x8(%rbp), %r8
141553c75:     	leaq	0xa0(%rbp), %r9
141553c7c:     	callq	0x1409365d0 <.text+0x9355d0>
141553c81:     	movq	-0x18(%rbp), %rax
141553c85:     	movq	%rax, 0x1b0(%rbp)
141553c8c:     	movq	-0x10(%rbp), %rax
141553c90:     	testq	%rax, %rax
141553c93:     	je	0x141553e70 <.text+0x1552e70>
141553c99:     	leaq	(%rax,%rax,2), %r15
141553c9d:     	shlq	$0x4, %r15
141553ca1:     	movq	0x1b0(%rbp), %r13
141553ca8:     	addq	%r13, %r15
141553cab:     	jmp	0x141553cef <.text+0x1552cef>
141553cad:     	nopl	(%rax)
141553cb0:     	movq	0x1a0(%rbp), %rax
141553cb7:     	leaq	(%r14,%r14,8), %rcx
141553cbb:     	leaq	(%rcx,%rcx,2), %rcx
141553cbf:     	addq	%r14, %rcx
141553cc2:     	movq	-0x28(%rbp), %rdx
141553cc6:     	movq	%rdx, 0x10(%rax,%rcx)
141553ccb:     	movups	-0x38(%rbp), %xmm0
141553ccf:     	movups	%xmm0, (%rax,%rcx)
141553cd3:     	movb	%r12b, 0x18(%rax,%rcx)
141553cd8:     	incq	%r14
141553cdb:     	movq	%r14, 0x1a8(%rbp)
141553ce2:     	addq	$0x30, %r13
141553ce6:     	cmpq	%r15, %r13
141553ce9:     	je	0x141553e70 <.text+0x1552e70>
141553cef:     	movss	(%r13), %xmm14
141553cf5:     	movss	0x8(%r13), %xmm15
141553cfb:     	movss	0x10(%r13), %xmm7
141553d01:     	mulss	0x13d1b67(%rip), %xmm7  # 0x142925870
141553d09:     	movaps	%xmm7, %xmm0
141553d0c:     	callq	0x142923fd0 <.text+0x2922fd0>
141553d11:     	movaps	%xmm0, %xmm6
141553d14:     	movaps	%xmm7, %xmm0
141553d17:     	callq	0x142923e80 <.text+0x2922e80>
141553d1c:     	movaps	%xmm6, %xmm2
141553d1f:     	mulss	%xmm9, %xmm2
141553d24:     	movss	0xc(%r13), %xmm1
141553d2a:     	movss	%xmm1, 0xa0(%rbp)
141553d32:     	movss	%xmm1, 0xa4(%rbp)
141553d3a:     	movss	%xmm1, 0xa8(%rbp)
141553d42:     	movups	0xa4(%rbp), %xmm3
141553d49:     	movaps	%xmm6, %xmm5
141553d4c:     	unpcklps	%xmm2, %xmm5            # xmm5 = xmm5[0],xmm2[0],xmm5[1],xmm2[1]
141553d4f:     	movaps	%xmm5, %xmm4
141553d52:     	addps	%xmm5, %xmm4
141553d55:     	mulss	%xmm4, %xmm2
141553d59:     	mulps	%xmm4, %xmm5
141553d5c:     	movshdup	%xmm4, %xmm7            # xmm7 = xmm4[1,1,3,3]
141553d60:     	mulss	%xmm7, %xmm6
141553d64:     	mulss	%xmm0, %xmm7
141553d68:     	mulss	%xmm0, %xmm4
141553d6c:     	movshdup	%xmm5, %xmm0            # xmm0 = xmm5[1,1,3,3]
141553d70:     	addss	%xmm0, %xmm5
141553d74:     	movaps	%xmm8, %xmm10
141553d78:     	subss	%xmm5, %xmm10
141553d7d:     	movaps	%xmm0, %xmm5
141553d80:     	subss	%xmm4, %xmm5
141553d84:     	subss	%xmm7, %xmm2
141553d88:     	addss	%xmm6, %xmm7
141553d8c:     	movlhps	%xmm7, %xmm2            # xmm2 = xmm2[0],xmm7[0]
141553d8f:     	addss	%xmm0, %xmm4
141553d93:     	movlhps	%xmm10, %xmm4           # xmm4 = xmm4[0],xmm10[0]
141553d97:     	movlhps	%xmm5, %xmm10           # xmm10 = xmm10[0],xmm5[0]
141553d9b:     	movddup	0xa0(%rbp), %xmm0       # xmm0 = mem[0,0]
141553da3:     	mulps	%xmm10, %xmm0
141553da7:     	movddup	0xa4(%rbp), %xmm5       # xmm5 = mem[0,0]
141553daf:     	mulps	%xmm2, %xmm5
141553db2:     	movshdup	%xmm3, %xmm2            # xmm2 = xmm3[1,1,3,3]
141553db6:     	movaps	%xmm3, %xmm6
141553db9:     	shufps	$0x55, %xmm3, %xmm6     # xmm6 = xmm6[1,1],xmm3[1,1]
141553dbd:     	mulps	%xmm4, %xmm6
141553dc0:     	movaps	%xmm14, %xmm3
141553dc4:     	movlhps	%xmm15, %xmm3           # xmm3 = xmm3[0],xmm15[0]
141553dc8:     	mulps	%xmm11, %xmm5
141553dcc:     	addps	%xmm0, %xmm5
141553dcf:     	mulps	%xmm11, %xmm6
141553dd3:     	addps	%xmm5, %xmm6
141553dd6:     	mulps	%xmm11, %xmm3
141553dda:     	addps	%xmm6, %xmm3
141553ddd:     	movaps	%xmm3, %xmm0
141553de0:     	unpckhpd	%xmm3, %xmm0            # xmm0 = xmm0[1],xmm3[1]
141553de4:     	movaps	%xmm3, %xmm4
141553de7:     	mulss	%xmm3, %xmm4
141553deb:     	movaps	%xmm0, %xmm5
141553dee:     	mulss	%xmm0, %xmm5
141553df2:     	addss	%xmm4, %xmm5
141553df6:     	xorps	%xmm4, %xmm4
141553df9:     	sqrtss	%xmm5, %xmm4
141553dfd:     	movaps	%xmm8, %xmm5
141553e01:     	divss	%xmm4, %xmm5
141553e05:     	mulss	%xmm5, %xmm3
141553e09:     	mulss	%xmm0, %xmm5
141553e0d:     	mulss	%xmm13, %xmm2
141553e12:     	mulss	%xmm12, %xmm1
141553e17:     	movq	%rdi, 0x38(%rsp)
141553e1c:     	movss	%xmm15, 0x30(%rsp)
141553e23:     	movss	%xmm14, 0x28(%rsp)
141553e2a:     	movss	%xmm5, 0x20(%rsp)
141553e30:     	movq	%rsi, %rcx
141553e33:     	callq	0x140c98980 <.text+0xc97980>
141553e38:     	movq	0x1a8(%rbp), %r14
141553e3f:     	cmpq	0x198(%rbp), %r14
141553e46:     	jne	0x141553cb0 <.text+0x1552cb0>
141553e4c:     	leaq	0x198(%rbp), %rcx
141553e53:     	leaq	0x1718c66(%rip), %rdx   # 0x142c6cac0
141553e5a:     	callq	0x14078cb80 <.text+0x78bb80>
141553e5f:     	jmp	0x141553cb0 <.text+0x1552cb0>
141553e64:     	nopw	%cs:(%rax,%rax)
141553e70:     	movq	-0x20(%rbp), %rax
141553e74:     	testq	%rax, %rax
141553e77:     	je	0x141553e93 <.text+0x1552e93>
141553e79:     	shlq	$0x4, %rax
141553e7d:     	leaq	(%rax,%rax,2), %rdx
141553e81:     	movl	$0x4, %r8d
141553e87:     	movq	0x1b0(%rbp), %rcx
141553e8e:     	callq	0x140613c20 <.text+0x612c20>
141553e93:     	movq	0x20(%rbp), %rdx
141553e97:     	testq	%rdx, %rdx
141553e9a:     	leaq	0x20(%rbp), %r14
141553e9e:     	je	0x141553bc0 <.text+0x1552bc0>
141553ea4:     	movq	0x28(%rbp), %rcx
141553ea8:     	movl	$0x1, %r8d
141553eae:     	callq	0x140613c20 <.text+0x612c20>
141553eb3:     	jmp	0x141553bc0 <.text+0x1552bc0>
141553eb8:     	leaq	0x1718afb(%rip), %rdx   # 0x142c6c9ba
141553ebf:     	leaq	0xc8(%rbp), %rcx
141553ec6:     	movl	$0x19, %r8d
141553ecc:     	callq	0x140b0eeb0 <.text+0xb0deb0>
141553ed1:     	xorl	%eax, %eax
141553ed3:     	cmpq	0xc8(%rbp), %rax
141553eda:     	jo	0x1415542ba <.text+0x15532ba>
141553ee0:     	movq	0x138(%rbp), %rax
141553ee7:     	movq	%rax, 0x90(%rbp)
141553eee:     	movups	0x128(%rbp), %xmm0
141553ef5:     	movaps	%xmm0, 0x80(%rbp)
141553efc:     	movups	0x118(%rbp), %xmm0
141553f03:     	movaps	%xmm0, 0x70(%rbp)
141553f07:     	movups	0x108(%rbp), %xmm0
141553f0e:     	movaps	%xmm0, 0x60(%rbp)
141553f12:     	movups	0xc8(%rbp), %xmm0
141553f19:     	movups	0xd8(%rbp), %xmm1
141553f20:     	movups	0xe8(%rbp), %xmm2
141553f27:     	movups	0xf8(%rbp), %xmm3
141553f2e:     	movaps	%xmm3, 0x50(%rbp)
141553f32:     	movaps	%xmm2, 0x40(%rbp)
141553f36:     	movaps	%xmm1, 0x30(%rbp)
141553f3a:     	movaps	%xmm0, 0x20(%rbp)
141553f3e:     	movq	0x2e0(%rbp), %rax
141553f45:     	movq	%rax, 0x20(%rsp)
141553f4a:     	leaq	0x1718acf(%rip), %rdx   # 0x142c6ca20
141553f51:     	leaq	0x20(%rbp), %rcx
141553f55:     	movl	$0x6, %r8d
141553f5b:     	movq	0x2d8(%rbp), %r9
141553f62:     	callq	0x140b1aca0 <.text+0xb19ca0>
141553f67:     	movq	%rax, %rsi
141553f6a:     	leaq	0x1718982(%rip), %rdx   # 0x142c6c8f3
141553f71:     	movl	$0xd, %r8d
141553f77:     	movq	0x2e8(%rbp), %rcx
141553f7e:     	callq	0x140937aa0 <.text+0x936aa0>
141553f83:     	testb	$0x1, %al
141553f85:     	je	0x1415542ef <.text+0x15532ef>
141553f8b:     	movq	%rdx, %rdi
141553f8e:     	leaq	0xc8(%rbp), %rcx
141553f95:     	movq	0x2d0(%rbp), %rdx
141553f9c:     	callq	0x140d146a0 <.text+0xd136a0>
141553fa1:     	movq	0xd0(%rbp), %rbx
141553fa8:     	testq	%rbx, %rbx
141553fab:     	jne	0x141553fb4 <.text+0x1552fb4>
141553fad:     	movq	0xd8(%rbp), %rbx
141553fb4:     	movq	0xe8(%rbp), %r12
141553fbb:     	movq	(%rbx), %rax
141553fbe:     	movq	0x10(%rbx), %r14
141553fc2:     	subq	%r14, %rax
141553fc5:     	cmpq	$0x97, %rax
141553fcb:     	jbe	0x141554270 <.text+0x1553270>
141553fd1:     	movq	0x8(%rbx), %rax
141553fd5:     	leaq	-0x27abdc(%rip), %rcx   # 0x1412d9400 <.text+0x12d8400>
141553fdc:     	movq	%rcx, (%rax,%r14)
141553fe0:     	movaps	0x13ea0d9(%rip), %xmm6  # 0x14293e0c0
141553fe7:     	movups	%xmm6, 0x8(%rax,%r14)
141553fed:     	movq	$0x0, 0x18(%rax,%r14)
141553ff6:     	movl	$0x0, 0x20(%rax,%r14)
141553fff:     	movabsq	$0x3f8000003f800000, %r15 # imm = 0x3F8000003F800000
141554009:     	movq	%r15, 0x24(%rax,%r14)
14155400e:     	movl	$0x3f800000, 0x2c(%rax,%r14) # imm = 0x3F800000
141554017:     	movq	%rsi, 0x38(%rax,%r14)
14155401c:     	movq	%rdi, 0x40(%rax,%r14)
141554021:     	movabsq	$0x23ae4b5c6ce0d14d, %rcx # imm = 0x23AE4B5C6CE0D14D
14155402b:     	movq	%rcx, 0x48(%rax,%r14)
141554030:     	movabsq	$0x691e5a80737217f2, %rcx # imm = 0x691E5A80737217F2
14155403a:     	movq	%rcx, 0x50(%rax,%r14)
14155403f:     	movabsq	$-0x40c57d0cad3ebff, %rcx # imm = 0xFBF3A82F352C1401
141554049:     	movq	%rcx, 0x58(%rax,%r14)
14155404e:     	movb	$0x0, 0x68(%rax,%r14)
141554054:     	movq	%r12, 0x78(%rax,%r14)
141554059:     	leaq	0x10cd0(%rip), %r12     # 0x141564d30 <.text+0x1563d30>
141554060:     	movq	%r12, 0x88(%rax,%r14)
141554068:     	addq	$0x98, %r14
14155406f:     	movq	%r14, 0x10(%rbx)
141554073:     	leaq	0x17189c6(%rip), %rdx   # 0x142c6ca40
14155407a:     	leaq	0xc8(%rbp), %rcx
141554081:     	movl	$0x21, %r8d
141554087:     	callq	0x140b0eeb0 <.text+0xb0deb0>
14155408c:     	xorl	%eax, %eax
14155408e:     	cmpq	0xc8(%rbp), %rax
141554095:     	jo	0x1415542fd <.text+0x15532fd>
14155409b:     	movq	0x138(%rbp), %rax
1415540a2:     	movq	%rax, 0x90(%rbp)
1415540a9:     	movups	0x128(%rbp), %xmm0
1415540b0:     	movaps	%xmm0, 0x80(%rbp)
1415540b7:     	movups	0x118(%rbp), %xmm0
1415540be:     	movaps	%xmm0, 0x70(%rbp)
1415540c2:     	movups	0x108(%rbp), %xmm0
1415540c9:     	movaps	%xmm0, 0x60(%rbp)
1415540cd:     	movups	0xc8(%rbp), %xmm0
1415540d4:     	movups	0xd8(%rbp), %xmm1
1415540db:     	movups	0xe8(%rbp), %xmm2
1415540e2:     	movups	0xf8(%rbp), %xmm3
1415540e9:     	movaps	%xmm3, 0x50(%rbp)
1415540ed:     	movaps	%xmm2, 0x40(%rbp)
1415540f1:     	movaps	%xmm1, 0x30(%rbp)
1415540f5:     	movaps	%xmm0, 0x20(%rbp)
1415540f9:     	movq	0x2e0(%rbp), %rax
141554100:     	movq	%rax, 0x20(%rsp)
141554105:     	leaq	0x1718974(%rip), %rdx   # 0x142c6ca80
14155410c:     	leaq	0x20(%rbp), %rcx
141554110:     	movl	$0xe, %r8d
141554116:     	movq	0x2d8(%rbp), %r9
14155411d:     	callq	0x140b1aca0 <.text+0xb19ca0>
141554122:     	movq	%rax, %rsi
141554125:     	leaq	0x17187c7(%rip), %rdx   # 0x142c6c8f3
14155412c:     	movl	$0xd, %r8d
141554132:     	movq	0x2e8(%rbp), %rcx
141554139:     	callq	0x140937aa0 <.text+0x936aa0>
14155413e:     	testb	$0x1, %al
141554140:     	je	0x141554332 <.text+0x1553332>
141554146:     	movq	%rdx, %rdi
141554149:     	leaq	0xc8(%rbp), %rcx
141554150:     	movq	0x2d0(%rbp), %rdx
141554157:     	callq	0x140d146a0 <.text+0xd136a0>
14155415c:     	movq	0xd0(%rbp), %rbx
141554163:     	testq	%rbx, %rbx
141554166:     	jne	0x14155416f <.text+0x155316f>
141554168:     	movq	0xd8(%rbp), %rbx
14155416f:     	movq	0xe8(%rbp), %r13
141554176:     	movq	(%rbx), %rax
141554179:     	movq	0x10(%rbx), %r14
14155417d:     	subq	%r14, %rax
141554180:     	cmpq	$0x87, %rax
141554186:     	jbe	0x141554295 <.text+0x1553295>
14155418c:     	movq	0x8(%rbx), %rax
141554190:     	leaq	-0x27af27(%rip), %rcx   # 0x1412d9270 <.text+0x12d8270>
141554197:     	movq	%rcx, (%rax,%r14)
14155419b:     	movups	%xmm6, 0x8(%rax,%r14)
1415541a1:     	movq	$0x0, 0x18(%rax,%r14)
1415541aa:     	movl	$0x0, 0x20(%rax,%r14)
1415541b3:     	movq	%r15, 0x24(%rax,%r14)
1415541b8:     	movl	$0x3f800000, 0x2c(%rax,%r14) # imm = 0x3F800000
1415541c1:     	movq	%rsi, 0x38(%rax,%r14)
1415541c6:     	movq	%rdi, 0x40(%rax,%r14)
1415541cb:     	movabsq	$0x2bb813728bcb5dd4, %rcx # imm = 0x2BB813728BCB5DD4
1415541d5:     	movq	%rcx, 0x48(%rax,%r14)
1415541da:     	movb	$0x0, 0x58(%rax,%r14)
1415541e0:     	movq	%r13, 0x68(%rax,%r14)
1415541e5:     	movq	%r12, 0x78(%rax,%r14)
1415541ea:     	addq	$0x88, %r14
1415541f1:     	movq	%r14, 0x10(%rbx)
1415541f5:     	movq	0x1a8(%rbp), %rcx
1415541fc:     	movq	0x18(%rbp), %rax
141554200:     	movq	%rcx, 0x10(%rax)
141554204:     	movups	0x198(%rbp), %xmm0
14155420b:     	movups	%xmm0, (%rax)
14155420e:     	movaps	0x1c0(%rbp), %xmm6
141554215:     	movaps	0x1d0(%rbp), %xmm7
14155421c:     	movaps	0x1e0(%rbp), %xmm8
141554224:     	movaps	0x1f0(%rbp), %xmm9
14155422c:     	movaps	0x200(%rbp), %xmm10
141554234:     	movaps	0x210(%rbp), %xmm11
14155423c:     	movaps	0x220(%rbp), %xmm12
141554244:     	movaps	0x230(%rbp), %xmm13
14155424c:     	movaps	0x240(%rbp), %xmm14
141554254:     	movaps	0x250(%rbp), %xmm15
14155425c:     	addq	$0x2e8, %rsp            # imm = 0x2E8
141554263:     	popq	%rbx
141554264:     	popq	%rdi
141554265:     	popq	%rsi
141554266:     	popq	%r12
141554268:     	popq	%r13
14155426a:     	popq	%r14
14155426c:     	popq	%r15
14155426e:     	popq	%rbp
14155426f:     	retq
141554270:     	movq	$0x1, 0x20(%rsp)
141554279:     	movl	$0x98, %r8d
14155427f:     	movl	$0x1, %r9d
141554285:     	movq	%rbx, %rcx
141554288:     	movq	%r14, %rdx
14155428b:     	callq	0x1428a9a60 <.text+0x28a8a60>
141554290:     	jmp	0x141553fd1 <.text+0x1552fd1>
141554295:     	movq	$0x1, 0x20(%rsp)
14155429e:     	movl	$0x88, %r8d
1415542a4:     	movl	$0x1, %r9d
1415542aa:     	movq	%rbx, %rcx
1415542ad:     	movq	%r14, %rdx
1415542b0:     	callq	0x1428a9a60 <.text+0x28a8a60>
1415542b5:     	jmp	0x14155418c <.text+0x155318c>
1415542ba:     	movq	0xd0(%rbp), %rax
1415542c1:     	movq	%rax, 0x20(%rbp)
1415542c5:     	leaq	0x171873c(%rip), %rax   # 0x142c6ca08
1415542cc:     	movq	%rax, 0x20(%rsp)
1415542d1:     	leaq	0x1717ee8(%rip), %rcx   # 0x142c6c1c0
1415542d8:     	leaq	0x1717ec1(%rip), %r9    # 0x142c6c1a0
1415542df:     	leaq	0x20(%rbp), %r8
1415542e3:     	movl	$0x2b, %edx
1415542e8:     	callq	0x1428d9760 <.text+0x28d8760>
1415542ed:     	jmp	0x14155433e <.text+0x155333e>
1415542ef:     	leaq	0x1718732(%rip), %rcx   # 0x142c6ca28
1415542f6:     	callq	0x1428d9310 <.text+0x28d8310>
1415542fb:     	jmp	0x14155433e <.text+0x155333e>
1415542fd:     	movq	0xd0(%rbp), %rax
141554304:     	movq	%rax, 0x20(%rbp)
141554308:     	leaq	0x1718759(%rip), %rax   # 0x142c6ca68
14155430f:     	movq	%rax, 0x20(%rsp)
141554314:     	leaq	0x1717ea5(%rip), %rcx   # 0x142c6c1c0
14155431b:     	leaq	0x1717e7e(%rip), %r9    # 0x142c6c1a0
141554322:     	leaq	0x20(%rbp), %r8
141554326:     	movl	$0x2b, %edx
14155432b:     	callq	0x1428d9760 <.text+0x28d8760>
141554330:     	jmp	0x14155433e <.text+0x155333e>
141554332:     	leaq	0x1718757(%rip), %rcx   # 0x142c6ca90
141554339:     	callq	0x1428d9310 <.text+0x28d8310>
14155433e:     	ud2
141554340:     	movq	%rdx, 0x10(%rsp)
141554345:     	pushq	%rbp
141554346:     	pushq	%r15
141554348:     	pushq	%r14
14155434a:     	pushq	%r13
14155434c:     	pushq	%r12
14155434e:     	pushq	%rsi
14155434f:     	pushq	%rdi
141554350:     	pushq	%rbx
141554351:     	subq	$0xe8, %rsp
141554358:     	leaq	0x80(%rdx), %rbp
14155435f:     	movaps	%xmm15, 0x40(%rsp)
141554365:     	movaps	%xmm14, 0x50(%rsp)
14155436b:     	movaps	%xmm13, 0x60(%rsp)
141554371:     	movaps	%xmm12, 0x70(%rsp)
141554377:     	movaps	%xmm11, 0x80(%rsp)
141554380:     	movaps	%xmm10, 0x90(%rsp)
141554389:     	movaps	%xmm9, 0xa0(%rsp)
141554392:     	movaps	%xmm8, 0xb0(%rsp)
14155439b:     	movaps	%xmm7, 0xc0(%rsp)
1415543a3:     	movaps	%xmm6, 0xd0(%rsp)
1415543ab:     	movq	0x198(%rbp), %rax
1415543b2:     	testq	%rax, %rax
1415543b5:     	je	0x1415543d4 <.text+0x15533d4>
1415543b7:     	movq	0x1a0(%rbp), %rcx
1415543be:     	leaq	(%rax,%rax,8), %rdx
1415543c2:     	leaq	(%rdx,%rdx,2), %rdx
1415543c6:     	addq	%rax, %rdx
1415543c9:     	movl	$0x4, %r8d
1415543cf:     	callq	0x140613c20 <.text+0x612c20>
1415543d4:     	movaps	0xd0(%rsp), %xmm6
1415543dc:     	movaps	0xc0(%rsp), %xmm7
1415543e4:     	movaps	0xb0(%rsp), %xmm8
1415543ed:     	movaps	0xa0(%rsp), %xmm9
1415543f6:     	movaps	0x90(%rsp), %xmm10
1415543ff:     	movaps	0x80(%rsp), %xmm11
141554408:     	movaps	0x70(%rsp), %xmm12
14155440e:     	movaps	0x60(%rsp), %xmm13
141554414:     	movaps	0x50(%rsp), %xmm14
14155441a:     	movaps	0x40(%rsp), %xmm15
141554420:     	addq	$0xe8, %rsp
141554427:     	popq	%rbx
141554428:     	popq	%rdi
141554429:     	popq	%rsi
14155442a:     	popq	%r12
14155442c:     	popq	%r13
14155442e:     	popq	%r14
141554430:     	popq	%r15
141554432:     	popq	%rbp
141554433:     	retq
141554434:     	nopw	%cs:(%rax,%rax)
141554440:     	movq	%rdx, 0x10(%rsp)
141554445:     	pushq	%rbp
141554446:     	pushq	%r15
141554448:     	pushq	%r14
14155444a:     	pushq	%r13
14155444c:     	pushq	%r12
14155444e:     	pushq	%rsi
14155444f:     	pushq	%rdi
141554450:     	pushq	%rbx
141554451:     	subq	$0xe8, %rsp
141554458:     	leaq	0x80(%rdx), %rbp
14155445f:     	movaps	%xmm15, 0x40(%rsp)
141554465:     	movaps	%xmm14, 0x50(%rsp)
14155446b:     	movaps	%xmm13, 0x60(%rsp)
141554471:     	movaps	%xmm12, 0x70(%rsp)
141554477:     	movaps	%xmm11, 0x80(%rsp)
141554480:     	movaps	%xmm10, 0x90(%rsp)
141554489:     	movaps	%xmm9, 0xa0(%rsp)
141554492:     	movaps	%xmm8, 0xb0(%rsp)
14155449b:     	movaps	%xmm7, 0xc0(%rsp)
1415544a3:     	movaps	%xmm6, 0xd0(%rsp)
1415544ab:     	movq	0x20(%rbp), %rdx
1415544af:     	testq	%rdx, %rdx
1415544b2:     	je	0x1415544c3 <.text+0x15534c3>
1415544b4:     	movq	0x28(%rbp), %rcx
1415544b8:     	movl	$0x1, %r8d
1415544be:     	callq	0x140613c20 <.text+0x612c20>
1415544c3:     	movaps	0xd0(%rsp), %xmm6
1415544cb:     	movaps	0xc0(%rsp), %xmm7
1415544d3:     	movaps	0xb0(%rsp), %xmm8
1415544dc:     	movaps	0xa0(%rsp), %xmm9
1415544e5:     	movaps	0x90(%rsp), %xmm10
1415544ee:     	movaps	0x80(%rsp), %xmm11
1415544f7:     	movaps	0x70(%rsp), %xmm12
1415544fd:     	movaps	0x60(%rsp), %xmm13
141554503:     	movaps	0x50(%rsp), %xmm14
141554509:     	movaps	0x40(%rsp), %xmm15
14155450f:     	addq	$0xe8, %rsp
141554516:     	popq	%rbx
141554517:     	popq	%rdi
141554518:     	popq	%rsi
141554519:     	popq	%r12
14155451b:     	popq	%r13
14155451d:     	popq	%r14
14155451f:     	popq	%r15
141554521:     	popq	%rbp
141554522:     	retq
141554523:     	nopw	%cs:(%rax,%rax)
141554530:     	movq	%rdx, 0x10(%rsp)
141554535:     	pushq	%rbp
141554536:     	pushq	%r15
141554538:     	pushq	%r14
14155453a:     	pushq	%r13
14155453c:     	pushq	%r12
14155453e:     	pushq	%rsi
14155453f:     	pushq	%rdi
141554540:     	pushq	%rbx
141554541:     	subq	$0xe8, %rsp
141554548:     	leaq	0x80(%rdx), %rbp
14155454f:     	movaps	%xmm15, 0x40(%rsp)
141554555:     	movaps	%xmm14, 0x50(%rsp)
14155455b:     	movaps	%xmm13, 0x60(%rsp)
141554561:     	movaps	%xmm12, 0x70(%rsp)
141554567:     	movaps	%xmm11, 0x80(%rsp)
141554570:     	movaps	%xmm10, 0x90(%rsp)
141554579:     	movaps	%xmm9, 0xa0(%rsp)
141554582:     	movaps	%xmm8, 0xb0(%rsp)
14155458b:     	movaps	%xmm7, 0xc0(%rsp)
141554593:     	movaps	%xmm6, 0xd0(%rsp)
14155459b:     	movq	-0x20(%rbp), %rax
14155459f:     	testq	%rax, %rax
1415545a2:     	je	0x1415545be <.text+0x15535be>
1415545a4:     	shlq	$0x4, %rax
1415545a8:     	leaq	(%rax,%rax,2), %rdx
1415545ac:     	movl	$0x4, %r8d
1415545b2:     	movq	0x1b0(%rbp), %rcx
1415545b9:     	callq	0x140613c20 <.text+0x612c20>
1415545be:     	movaps	0xd0(%rsp), %xmm6
1415545c6:     	movaps	0xc0(%rsp), %xmm7
1415545ce:     	movaps	0xb0(%rsp), %xmm8
1415545d7:     	movaps	0xa0(%rsp), %xmm9
1415545e0:     	movaps	0x90(%rsp), %xmm10
1415545e9:     	movaps	0x80(%rsp), %xmm11
1415545f2:     	movaps	0x70(%rsp), %xmm12
1415545f8:     	movaps	0x60(%rsp), %xmm13
1415545fe:     	movaps	0x50(%rsp), %xmm14
141554604:     	movaps	0x40(%rsp), %xmm15
14155460a:     	addq	$0xe8, %rsp
141554611:     	popq	%rbx
141554612:     	popq	%rdi
141554613:     	popq	%rsi
141554614:     	popq	%r12
141554616:     	popq	%r13
141554618:     	popq	%r14
14155461a:     	popq	%r15
14155461c:     	popq	%rbp
14155461d:     	retq
14155461e:     	nop
141554620:     	movq	%rdx, 0x10(%rsp)
141554625:     	pushq	%rbp
141554626:     	pushq	%r15
141554628:     	pushq	%r14
14155462a:     	pushq	%r13
14155462c:     	pushq	%r12
14155462e:     	pushq	%rsi
14155462f:     	pushq	%rdi
141554630:     	pushq	%rbx
141554631:     	subq	$0xe8, %rsp
141554638:     	leaq	0x80(%rdx), %rbp
14155463f:     	movaps	%xmm15, 0x40(%rsp)
141554645:     	movaps	%xmm14, 0x50(%rsp)
14155464b:     	movaps	%xmm13, 0x60(%rsp)
141554651:     	movaps	%xmm12, 0x70(%rsp)
141554657:     	movaps	%xmm11, 0x80(%rsp)
141554660:     	movaps	%xmm10, 0x90(%rsp)
141554669:     	movaps	%xmm9, 0xa0(%rsp)
141554672:     	movaps	%xmm8, 0xb0(%rsp)
14155467b:     	movaps	%xmm7, 0xc0(%rsp)
141554683:     	movaps	%xmm6, 0xd0(%rsp)
14155468b:     	leaq	0x20(%rbp), %rcx
14155468f:     	callq	0x140d4cc30 <.text+0xd4bc30>
141554694:     	movaps	0xd0(%rsp), %xmm6
14155469c:     	movaps	0xc0(%rsp), %xmm7
1415546a4:     	movaps	0xb0(%rsp), %xmm8
1415546ad:     	movaps	0xa0(%rsp), %xmm9
1415546b6:     	movaps	0x90(%rsp), %xmm10
1415546bf:     	movaps	0x80(%rsp), %xmm11
1415546c8:     	movaps	0x70(%rsp), %xmm12
1415546ce:     	movaps	0x60(%rsp), %xmm13
1415546d4:     	movaps	0x50(%rsp), %xmm14
1415546da:     	movaps	0x40(%rsp), %xmm15
1415546e0:     	addq	$0xe8, %rsp
1415546e7:     	popq	%rbx
1415546e8:     	popq	%rdi
1415546e9:     	popq	%rsi
1415546ea:     	popq	%r12
1415546ec:     	popq	%r13
1415546ee:     	popq	%r14
1415546f0:     	popq	%r15
1415546f2:     	popq	%rbp
1415546f3:     	retq
1415546f4:     	nopw	%cs:(%rax,%rax)
141554700:     	movq	%rdx, 0x10(%rsp)
141554705:     	pushq	%rbp
141554706:     	pushq	%r15
141554708:     	pushq	%r14
14155470a:     	pushq	%r13
14155470c:     	pushq	%r12
14155470e:     	pushq	%rsi
14155470f:     	pushq	%rdi
141554710:     	pushq	%rbx
141554711:     	subq	$0xe8, %rsp
141554718:     	leaq	0x80(%rdx), %rbp
14155471f:     	movaps	%xmm15, 0x40(%rsp)
141554725:     	movaps	%xmm14, 0x50(%rsp)
14155472b:     	movaps	%xmm13, 0x60(%rsp)
141554731:     	movaps	%xmm12, 0x70(%rsp)
141554737:     	movaps	%xmm11, 0x80(%rsp)
141554740:     	movaps	%xmm10, 0x90(%rsp)
141554749:     	movaps	%xmm9, 0xa0(%rsp)
141554752:     	movaps	%xmm8, 0xb0(%rsp)
14155475b:     	movaps	%xmm7, 0xc0(%rsp)
141554763:     	movaps	%xmm6, 0xd0(%rsp)
14155476b:     	leaq	0x20(%rbp), %rcx
14155476f:     	callq	0x140d4cc30 <.text+0xd4bc30>
141554774:     	movaps	0xd0(%rsp), %xmm6
14155477c:     	movaps	0xc0(%rsp), %xmm7
141554784:     	movaps	0xb0(%rsp), %xmm8
14155478d:     	movaps	0xa0(%rsp), %xmm9
141554796:     	movaps	0x90(%rsp), %xmm10
14155479f:     	movaps	0x80(%rsp), %xmm11
1415547a8:     	movaps	0x70(%rsp), %xmm12
1415547ae:     	movaps	0x60(%rsp), %xmm13
1415547b4:     	movaps	0x50(%rsp), %xmm14
1415547ba:     	movaps	0x40(%rsp), %xmm15
1415547c0:     	addq	$0xe8, %rsp
1415547c7:     	popq	%rbx
1415547c8:     	popq	%rdi
1415547c9:     	popq	%rsi
1415547ca:     	popq	%r12
1415547cc:     	popq	%r13
1415547ce:     	popq	%r14
1415547d0:     	popq	%r15
1415547d2:     	popq	%rbp
1415547d3:     	retq
